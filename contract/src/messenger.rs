#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]

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
pub extern "C" fn get_text() {
    runtime::ret(CLValue::from_t("v1".to_string()).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn get_access() {
    let access_token: URef = runtime::get_key("ACCESS_TOKEN")
        .unwrap()
        .try_into()
        .unwrap();
    runtime::ret(CLValue::from_t(access_token).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn upgrade_me() {
    let this_contract_hash: ContractPackageHash = get_key("messanger_package");
    runtime::call_versioned_contract(
        runtime::get_named_arg::<ContractPackageHash>("upgrader"),
        None,
        "install_upgrade",
        casper_types::runtime_args! {"to_upgrade"=> this_contract_hash},
    )
}

#[no_mangle]
pub extern "C" fn call() {
    let (contract_package_hash, access_token) = storage::create_contract_package_at_hash();
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "get_text",
        vec![],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_access",
        vec![],
        CLType::URef,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "upgrade_me",
        vec![],
        CLType::URef,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let mut named_keys = NamedKeys::new();
    named_keys.insert(
        "messenger_package".to_string(),
        storage::new_uref(contract_package_hash).into(),
    );
    named_keys.insert("ACCESS_TOKEN".to_string(), access_token.into());
    let (_stored_contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);
    set_key("contract_package_hash", contract_package_hash);
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
