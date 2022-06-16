#![no_main]

use casper_contract::contract_api::{
    runtime::{self, put_key},
    storage::{self, new_uref},
};
use casper_types::{
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    CLType, Parameter,
};

pub const POST: &str = "post";

// EntryPoint to make a post. Stores the post argument under the name `post`.
#[no_mangle]
pub extern "C" fn post() {
    let post = runtime::get_named_arg::<String>(POST);
    put_key(POST, new_uref(post).into());
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        POST,
        vec![Parameter::new(POST, CLType::String)],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    // Create the new contract and store its package hash and access token. This is an important step,
    // as only the context holding the access token can make changes to the contract package (add/disable contract version).
    let (contract_hash, _version) = storage::new_contract(
        entry_points,
        None,
        Some("post_board_package_hash".to_string()),
        Some("post_board_access_token".to_string()),
    );
    // Store the hash of this initial contract version.
    runtime::put_key("post_board_contract_hash_1", contract_hash.into());
}
