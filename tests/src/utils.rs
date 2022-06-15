use std::path::PathBuf;

use casper_engine_test_support::{
    DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder, ARG_AMOUNT,
    DEFAULT_ACCOUNT_ADDR, DEFAULT_PAYMENT,
};
use casper_execution_engine::core::engine_state::ExecuteRequest;
use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, system::mint, CLTyped, Key,
    RuntimeArgs, StoredValue, U512,
};
use rand::Rng;

pub fn query<T: FromBytes + CLTyped>(
    builder: &InMemoryWasmTestBuilder,
    base: Key,
    path: &[String],
) -> T {
    builder
        .query(None, base, path)
        .expect("should be stored value.")
        .as_cl_value()
        .expect("should be cl value.")
        .clone()
        .into_t()
        .expect("Wrong type in query result.")
}

pub fn fund_account(account: &AccountHash) -> ExecuteRequest {
    let mut rng = rand::thread_rng();
    let deploy_item = DeployItemBuilder::new()
        .with_address(*DEFAULT_ACCOUNT_ADDR)
        .with_authorization_keys(&[*DEFAULT_ACCOUNT_ADDR])
        .with_empty_payment_bytes(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
        .with_transfer_args(runtime_args! {
            mint::ARG_AMOUNT => U512::from(50_000_000_000_000_u64),
            mint::ARG_TARGET => *account,
            mint::ARG_ID => <Option::<u64>>::None
        })
        .with_deploy_hash(rng.gen())
        .build();

    ExecuteRequestBuilder::from_deploy_item(deploy_item).build()
}

pub enum DeploySource {
    Code(PathBuf),
    ByContractName { name: String, entry_point: String },
}

pub fn deploy(
    builder: &mut InMemoryWasmTestBuilder,
    deployer: &AccountHash,
    source: &DeploySource,
    args: RuntimeArgs,
    block_time: Option<u64>,
) {
    let mut rng = rand::thread_rng();
    let mut deploy_builder = DeployItemBuilder::new()
        .with_empty_payment_bytes(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
        .with_address(*deployer)
        .with_authorization_keys(&[*deployer])
        .with_deploy_hash(rng.gen());

    deploy_builder = match source {
        DeploySource::Code(path) => deploy_builder.with_session_code(path, args),
        DeploySource::ByContractName { name, entry_point } => {
            deploy_builder.with_stored_session_named_key(name, entry_point, args)
        }
    };

    let mut execute_request_builder =
        ExecuteRequestBuilder::from_deploy_item(deploy_builder.build());
    if let Some(ustamp) = block_time {
        execute_request_builder = execute_request_builder.with_block_time(ustamp)
    }
    builder
        .exec(execute_request_builder.build())
        .expect_success()
        .commit();
}

pub fn query_dictionary_item(
    builder: &InMemoryWasmTestBuilder,
    key: Key,
    dictionary_name: Option<String>,
    dictionary_item_key: &str,
) -> Result<StoredValue, String> {
    let empty_path = vec![];
    let dictionary_key_bytes = dictionary_item_key.as_bytes();
    let address = match key {
        Key::Account(_) | Key::Hash(_) => {
            if let Some(name) = dictionary_name {
                let stored_value = builder.query(None, key, &[])?;

                let named_keys = match &stored_value {
                    StoredValue::Account(account) => account.named_keys(),
                    StoredValue::Contract(contract) => contract.named_keys(),
                    _ => {
                        return Err(
                            "Provided base key is nether an account or a contract".to_string()
                        )
                    }
                };

                let dictionary_uref = named_keys
                    .get(&name)
                    .and_then(Key::as_uref)
                    .ok_or_else(|| "No dictionary uref was found in named keys".to_string())?;

                Key::dictionary(*dictionary_uref, dictionary_key_bytes)
            } else {
                return Err("No dictionary name was provided".to_string());
            }
        }
        Key::URef(uref) => Key::dictionary(uref, dictionary_key_bytes),
        Key::Dictionary(address) => Key::Dictionary(address),
        _ => return Err("Unsupported key type for a query to a dictionary item".to_string()),
    };
    builder.query(None, address, &empty_path)
}
