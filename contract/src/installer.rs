#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]

use casper_types::ContractPackageHash;
use core::convert::TryInto;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    runtime_args, CLType, CLTyped, CLValue, RuntimeArgs, URef,
};

#[no_mangle]
pub extern "C" fn get_text() {
    runtime::ret(CLValue::from_t("v2".to_string()).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn install_upgrade() {
    let to_upgrade = runtime::get_named_arg::<ContractPackageHash>("to_upgrade");
    let _access_token: URef =
        runtime::call_versioned_contract(to_upgrade, None, "get_access", runtime_args! {});

    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "get_text",
        vec![],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let (_stored_contract_hash, _) =
        storage::add_contract_version(to_upgrade.into(), entry_points, Default::default());
}

#[no_mangle]
pub extern "C" fn call() {
    let (contract_hash, access_token) = storage::create_contract_package_at_hash();
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "get_text",
        vec![],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "install_upgrade",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let mut named_keys = NamedKeys::new();
    named_keys.insert(
        "CONTRACT_HASH".to_string(),
        storage::new_uref(contract_hash).into(),
    );

    let (stored_contract_hash, _) =
        storage::add_contract_version(contract_hash.into(), entry_points, Default::default());
    set_key("installer_hash", contract_hash);
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
