#![no_main]

use casper_contract::{
    contract_api::{
        runtime::{self, get_caller, revert},
        storage::{self, dictionary_put, disable_contract_version, new_dictionary},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    ApiError, CLType, ContractPackageHash, Parameter,
};

pub const POST: &str = "post";
pub const POSTS: &str = "posts";
pub const DATE: &str = "date";
pub const AUTH: &str = "auth";
pub const POST_BOARD_CONTRACT_HASH_: &str = "post_board_contract_hash_";

// Revised entrypoint to make a post. Now has an authorization step, and a new argument `date`.
// With this version posts will be stored under the date they were made.
// From now on posts will be stored in a dictionary instead of just simply in context.
#[no_mangle]
pub extern "C" fn post() {
    if get_caller()
        != runtime::get_key(AUTH)
            .unwrap_or_revert()
            .into_account()
            .unwrap_or_revert()
    {
        revert(ApiError::User(0));
    }
    let dictionary_uref = match runtime::get_key(POSTS) {
        Some(uref_key) => uref_key.into_uref().unwrap_or_revert(),
        None => new_dictionary(POSTS).unwrap_or_revert(),
    };
    dictionary_put(
        dictionary_uref,
        &runtime::get_named_arg::<String>(DATE),
        runtime::get_named_arg::<String>(POST),
    );
}

#[no_mangle]
pub extern "C" fn call() {
    // Entrypoint set for the new contract version.
    // Does not need to match the set of the earlier versions,
    // but entrypoints not in this set will not be available from this version,
    // just as older versions do not hold the new entrypoints.
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        POST,
        vec![
            Parameter::new(POST, CLType::String),
            Parameter::new(DATE, CLType::String),
        ],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    // Read package hash from storage as it is needed when adding a new version.
    let post_board_package_hash: ContractPackageHash = runtime::get_key("post_board_package_hash")
        .unwrap_or_revert()
        .into_hash()
        .unwrap_or_revert()
        .into();

    let posts_dict = new_dictionary(POSTS).unwrap_or_revert();
    runtime::put_key("posts_uref_key", storage::new_uref(posts_dict).into());

    // On adding a new contract version new named keys can be added to the contracts context.
    let mut named_keys = NamedKeys::default();
    named_keys.insert(AUTH.to_string(), get_caller().into());
    named_keys.insert(POSTS.to_string(), posts_dict.into());

    // Adding a new version is done to the contract package so its hash is necessary to be known.
    // New entrypoints can be added and/or old ones can be overwritten.
    // Additional named keys are added and old ones are overwritten.
    let (contract_hash, _version) =
        storage::add_contract_version(post_board_package_hash, entry_points, named_keys);

    // Store the hash of the new version so it is known.
    runtime::put_key("post_board_contract_hash_2", contract_hash.into());

    // Disable older unneeded contract versions to seal any potential security issues
    // or bugs that might stem from having multiple incompatible versions.
    disable_contract_version(
        post_board_package_hash,
        runtime::get_key("post_board_contract_hash_1")
            .unwrap_or_revert()
            .into_hash()
            .unwrap_or_revert()
            .into(),
    )
    .unwrap_or_revert();
}
