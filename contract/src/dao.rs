#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]

use casper_types::ApiError;
use casper_types::ContractPackageHash;
use casper_types::Key;
use casper_types::URef;
use core::convert::TryInto;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    CLType, CLTyped, CLValue, RuntimeArgs,
};


#[no_mangle]
pub extern "C" fn upgrade() {
    let dao_contract_hash: ContractPackageHash = get_key("dao_contract_hash");
    let messenger_package_hash: ContractPackageHash = get_key("messenger_package_hash");
    runtime::call_versioned_contract(
        runtime::get_named_arg::<ContractPackageHash>("installer_package_hash"),
        None,
        "install",
        casper_types::runtime_args! {
            "dao_contract_hash" => dao_contract_hash,
            "messenger_package_hash" => messenger_package_hash
        },
    )
}

#[no_mangle]
pub extern "C" fn get_messenger_access() {
    runtime::revert(ApiError::User(69));
    let access_token: URef = runtime::get_key("MESSENGER_ACCESS_TOKEN")
        .unwrap()
        .try_into()
        .unwrap();
    runtime::ret(CLValue::from_t(access_token).unwrap_or_revert());
}

pub fn deploy_dao(messenger_package_hash: ContractPackageHash, access_token: URef) {
    let (dao_contract_hash, _dao_access_token) = storage::create_contract_package_at_hash();
    let mut dao_entry_points = EntryPoints::new();

    dao_entry_points.add_entry_point(EntryPoint::new(
        "upgrade",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public, //change to ?
        EntryPointType::Contract,
    ));

    dao_entry_points.add_entry_point(EntryPoint::new(
        "get_messenger_access",
        vec![],
        CLType::URef,
        EntryPointAccess::Public, //change to ?
        EntryPointType::Contract,
    ));

    let mut dao_named_keys = NamedKeys::new();
    dao_named_keys.insert(
        "DAO_CONTRACT_HASH".to_string(),
        storage::new_uref(dao_contract_hash).into(),
    );
    dao_named_keys.insert(
        "MESSENGER_ACCESS_TOKEN".to_string(),
        access_token.into(),
    );
    dao_named_keys.insert(
        "MESSENGER_PACKAGE_HASH".to_string(),
        casper_types::Key::URef(storage::new_uref(messenger_package_hash)),
    );
    let (_stored_contract_hash, _) =
        storage::add_contract_version(dao_contract_hash, dao_entry_points, dao_named_keys);

    // runtime::put_key(
    //     "dao_hash",
    //     casper_types::Key::URef(storage::new_uref(dao_contract_hash)),
    // );    
    set_key("dao_contract_hash", dao_contract_hash);
}

fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}

fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}
