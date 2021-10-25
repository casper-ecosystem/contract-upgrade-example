#![no_main]

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    CLType, CLValue,
};

/// This `get_message` function will never be updated, and as such will always return "locked"
#[no_mangle]
pub extern "C" fn get_message() {
    runtime::put_key("message", storage::new_uref("locked").into());
    runtime::ret(CLValue::from_t(String::from("locked")).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn call() {
    // Introduce a singular, public "get_message" entry point.
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "get_message",
        vec![],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    // Write a new key that states that this is the locked version
    let mut named_keys = NamedKeys::new();
    named_keys.insert(
        "version".to_string(),
        storage::new_uref("locked_version").into(),
    );

    // Creating a contract with `new_locked_contract` permanently denies the possibility of upgrading a contract.
    let (hash, _) = storage::new_locked_contract(
        entry_points,
        Some(named_keys),
        Some("messenger_package_hash".to_string()),
        Some("messanger_access_token".to_string()),
    );
    runtime::put_key("hash", hash.into());
}
