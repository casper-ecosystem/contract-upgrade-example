#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    CLType, CLValue, ContractPackageHash, RuntimeArgs,
};

#[no_mangle]
pub extern "C" fn get_message() {
    runtime::ret(CLValue::from_t("v1".to_string()).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn upgrade() {
    runtime::call_versioned_contract(
        runtime::get_named_arg::<ContractPackageHash>("installer_package_hash"),
        None,
        "install",
        casper_types::runtime_args! {},
    )
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "get_message",
        vec![],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "upgrade",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let (contract_hash, _version) = storage::new_contract(
        entry_points,
        None,
        Some("messenger_package_hash".to_string()),
        Some("access_token".to_string()),
    );

    runtime::put_key(
        "messenger_hash",
        casper_types::Key::URef(storage::new_uref(contract_hash)),
    );
}