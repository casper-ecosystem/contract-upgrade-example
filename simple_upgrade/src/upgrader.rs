#![no_main]

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    CLType, CLValue, ContractPackageHash,
};

/// Upgraded version of the version getter fuction, returning "v2" proving that the contract has gained
/// and upgraded version
#[no_mangle]
pub extern "C" fn get_message() {
    runtime::ret(CLValue::from_t("v2".to_string()).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();
    // Add entrypoint that will overwrite the one in the original contract
    entry_points.add_entry_point(EntryPoint::new(
        "get_message",
        vec![],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    // Get the package hash of the originally deployed contract
    let messanger_package_hash: ContractPackageHash = runtime::get_key("messenger_package_hash")
        .unwrap_or_revert()
        .into_hash()
        .unwrap()
        .into();
    // Overwrite the original contract with the new entry points. This works because the original code stored
    // the required access token into the accounts storage.
    let _ = storage::add_contract_version(messanger_package_hash, entry_points, Default::default());
}
