#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]
// This contract will call another contract and execute the "set_text" function in the external contract

use casper_contract::contract_api::runtime;
use casper_types::{runtime_args, ContractPackageHash, RuntimeArgs};

const ARG_CONTRACT_PACKAGE: &str = "contract_package";
const EXTERNAL_METHOD: &str = "set_text";

#[no_mangle]
pub extern "C" fn call() {
    // We ask for the contract package hash of the external contract that we want to call
    let contract_package: ContractPackageHash =
        runtime::get_named_arg(ARG_CONTRACT_PACKAGE);
    // We call the external contract and execute the set_text function
    runtime::call_versioned_contract(
        contract_package,
        None,
        EXTERNAL_METHOD,
        runtime_args! {},
    )
}
