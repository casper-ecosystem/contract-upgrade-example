#![no_main]

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    CLType, CLValue,
};

/// Original getter function that returns to the caller that this contract is "first"
#[no_mangle]
pub extern "C" fn get_message() {
    runtime::ret(CLValue::from_t(String::from("first")).unwrap_or_revert());
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

    // Introduce the contract itself to the account, and save it's package hash and access token
    // to the account's storage as "messenger_package_hash" and "messanger_access_token" respectively.
    let _ = storage::new_contract(
        entry_points,
        None,
        Some("messenger_package_hash".to_string()),
        Some("messanger_access_token".to_string()),
    );
}
