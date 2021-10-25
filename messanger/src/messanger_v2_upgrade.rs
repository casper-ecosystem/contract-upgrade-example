#![no_main]

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    CLType, CLValue, ContractPackageHash,
};

/// Upgraded version of the version getter fuction, returning "second" proving,
/// that the contract has gained and upgraded version.
#[no_mangle]
pub extern "C" fn get_message() {
    runtime::put_key("message", storage::new_uref("second").into());
    runtime::ret(CLValue::from_t("second".to_string()).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn call() {
    // Add entrypoint that will overwrite the one in the original contract
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "get_message",
        vec![],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    // Get the package hash of the originally deployed contract.
    let messanger_package_hash: ContractPackageHash = runtime::get_key("messenger_package_hash")
        .unwrap_or_revert()
        .into_hash()
        .unwrap()
        .into();

    let mut named_keys = NamedKeys::new();
    // When upgrading a contract, the system will not overwrite already existing keys,
    named_keys.insert(
        "version".to_string(),
        storage::new_uref("version_updated").into(),
    );
    // but you can create new keys.
    named_keys.insert(
        "version2".to_string(),
        storage::new_uref("version_updated").into(),
    );

    // Overwrite the original contract with the new entry points. This works
    // because the original code stored the required access token into the accounts storage.
    let (hash, _) = storage::add_contract_version(messanger_package_hash, entry_points, named_keys);
    runtime::put_key("hash", hash.into());
}
