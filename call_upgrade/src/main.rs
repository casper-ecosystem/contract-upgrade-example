#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]

// This contract executes the contract package based on a contract package hash that we give it then calls theupgrade_to function in that contract with a contract hash to our installer contract, upgrading our functions in the first contract

use casperlabs_contract::contract_api::runtime;
use casperlabs_types::{runtime_args, RuntimeArgs, 
    ContractPackageHash
};


#[no_mangle]
pub extern "C" fn call() {
    let contract_package: ContractPackageHash = runtime::get_named_arg("contract_package");
    let installer_package: ContractPackageHash = runtime::get_named_arg("installer_package");
    runtime::call_versioned_contract(contract_package, None, "upgrade_to", runtime_args! {
        "installer_package" => installer_package
    })
}
