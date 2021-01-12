#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]
// This contract will call another contract and execute the "set_text" function in the external contract

use casperlabs_contract::contract_api::runtime;
use casperlabs_types::{runtime_args, ContractPackageHash, RuntimeArgs};

#[no_mangle]
pub extern "C" fn call() {
    // We ask for the contract package hash of the external contract that we want to call
    let contract_package: ContractPackageHash =
        runtime::get_named_arg("contract_package");
    // We call the external contract and execute the set_text function
    runtime::call_versioned_contract(
        contract_package,
        None,
        "set_text",
        runtime_args! {},
    )
}
