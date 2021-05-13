#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]

use core::convert::TryInto;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::ToBytes,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    runtime_args, CLType, CLTyped, CLValue, ContractPackageHash, RuntimeArgs, URef,
};

#[no_mangle]
pub extern "C" fn get_message() {
    runtime::ret(CLValue::from_t("v2".to_string()).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn install() {
    let dao_contract_hash = runtime::get_named_arg::<ContractPackageHash>("dao_contract_hash");
    let messenger_hash = runtime::get_named_arg::<ContractPackageHash>("messenger_package_hash");
    let _messenger_access_token: URef = runtime::call_versioned_contract(
        dao_contract_hash,
        None,
        "get_messenger_access",
        runtime_args! {},
    );

    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "get_message",
        vec![],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let (_stored_contract_hash, _) =
        storage::add_contract_version(messenger_hash, entry_points, Default::default());
}

#[no_mangle]
pub extern "C" fn call() {
    let (installer_contract_hash, _access_token) = storage::create_contract_package_at_hash();
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "get_message",
        vec![],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "install",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let mut named_keys = NamedKeys::new();
    named_keys.insert(
        "INSTALLER_CONTRACT_HASH".to_string(),
        storage::new_uref(installer_contract_hash).into(),
    );

    let (_stored_contract_hash, _) =
        storage::add_contract_version(installer_contract_hash, entry_points, Default::default());
    set_key("installer_package_hash", installer_contract_hash);
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
