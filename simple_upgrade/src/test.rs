#![no_main]

use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{runtime_args, ApiError, ContractPackageHash, RuntimeArgs};

#[no_mangle]
pub extern "C" fn call() {
    // Get assertion target
    let expected: String = runtime::get_named_arg("expected");

    let messanger_package_hash: ContractPackageHash = runtime::get_key("messenger_package_hash")
        .unwrap_or_revert()
        .into_hash()
        .unwrap_or_revert()
        .into();

    // Retrieve message from the current contract version
    let message: String = runtime::call_versioned_contract(
        messanger_package_hash,
        None,
        "get_message",
        runtime_args! {},
    );

    // Assert the result of the call with what we expect
    if message != expected {
        runtime::revert(ApiError::User(1));
    }
}
