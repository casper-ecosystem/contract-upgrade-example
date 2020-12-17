#[cfg(test)]
mod tests {
    use casperlabs_engine_test_support::{Code, Error, SessionBuilder, TestContextBuilder, Value, TestContext, Hash};
    use casperlabs_types::{account::AccountHash, U512, RuntimeArgs, runtime_args, CLTyped, contracts::{ContractVersion, EntryPoints, NamedKeys, Contract}, bytesrepr::FromBytes};

    const MY_ACCOUNT: AccountHash = AccountHash::new([7u8; 32]);

    const METHOD_DEPOSIT: &str = "deposit";
    const METHOD_UPGRADE: &str = "upgrade";

    const INCOMMING_PURSE: &str = "incomming_purse";
    const ACCESS_TOKEN: &str = "access_token";
    const CONTRACT_NAME: &str = "deposit_box";
    const CONTRACT_HASH: &str = "deposit_box_hash";
    const CONTRACT_VERSION: &str = "contract_version";
    const TEXT_KEY: &str = "text";
    const TEXT_VALUE_V1: &str = "value_one";
    const TEXT_VALUE_V2: &str = "value_two";

    #[test]
    fn should_store_hello_world() {
        let mut context = TestContextBuilder::new()
            .with_account(MY_ACCOUNT, U512::from(128_000_000))
            .build();

        let session_code = Code::from("contract.wasm");
        let session_args = runtime_args! {};
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(MY_ACCOUNT)
            .with_authorization_keys(&[MY_ACCOUNT])
            .build();
        
        context.run(session);
        assert_eq!(get_contract_version_v1(&context), 1);

        call_deposit_v1(&mut context);
        assert_eq!(get_text(&context), TEXT_VALUE_V1);

        call_upgrade_v1(&mut context);

        // assert_eq!(get_contract_version_v2(&context), 2);
    }

    fn call_deposit_v1(context: &mut TestContext) {
        let contract_hash = get_contract_hash(&context);
        let code = Code::Hash(contract_hash, METHOD_DEPOSIT.to_string());
        let args = runtime_args!{};
        let session = SessionBuilder::new(code, args)
            .with_address(MY_ACCOUNT)
            .with_authorization_keys(&[MY_ACCOUNT])
            .build();
        context.run(session);
    }

    fn call_upgrade_v1(context: &mut TestContext) {
        let contract_hash = get_contract_hash(&context);
        let code = Code::Hash(contract_hash, METHOD_UPGRADE.to_string());
        let args = runtime_args!{};
        let session = SessionBuilder::new(code, args)
            .with_address(MY_ACCOUNT)
            .with_authorization_keys(&[MY_ACCOUNT])
            .build();
        context.run(session);
    }

    fn get_contract_version_v1(context: &TestContext) -> u32 {
        query_account(context, CONTRACT_VERSION).unwrap()
    }

    fn get_contract_version_v2(context: &TestContext) -> u32 {
        query_contract(context, CONTRACT_VERSION).unwrap()
    }

    fn get_contract_hash(context: &TestContext) -> Hash {
        query_account(context, CONTRACT_HASH).unwrap()
    }

    fn get_text(context: &TestContext) -> String {
        query_contract(context, TEXT_KEY).unwrap()
    }

    fn query_account<T: CLTyped + FromBytes>(context: &TestContext, key: &str) -> Option<T> {
        query(context, &[key])
    }

    fn query_contract<T: CLTyped + FromBytes>(context: &TestContext, key: &str) -> Option<T> {
        query(context, &[CONTRACT_NAME, key])
    }

    fn query<T: CLTyped + FromBytes>(context: &TestContext, path: &[&str]) -> Option<T> {
        match context
            .query(MY_ACCOUNT, path)
        {
            Err(err) => {
                println!("{:?}", err);
                None
            },
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not expected type.", path[0]));
                Some(value)
            }
        }
    }

}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
