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
    CLType, CLValue, ContractPackageHash,
};

#[no_mangle]
pub extern "C" fn get_message() {
    runtime::ret(CLValue::from_t("v2".to_string()).unwrap_or_revert());
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
    let messanger_package_hash: ContractPackageHash = runtime::get_key("messenger_package_hash")
        .unwrap_or_revert()
        .into_hash()
        .unwrap()
        .into();
    let _ = storage::add_contract_version(messanger_package_hash, entry_points, Default::default());
}
