#![cfg_attr(not(target_arch = "wasm32"), crate_type = "target arch should be wasm32")]
#![no_main]

// This contract executes the contract package based on a contract package hash that we give it then calls theupgrade_to function in that contract with a contract hash to our installer contract, upgrading our functions in the first contract

use casper_contract::contract_api::runtime;
use casper_types::{runtime_args, ContractPackageHash, RuntimeArgs};

const ARG_CONTRACT_PACKAGE: &str = "contract_package";
const ARG_INSTALLER_PACKAGE: &str = "installer_package";
const EXTERNAL_METHOD: &str = "upgrade_to";

#[no_mangle]
pub extern "C" fn call() {
    let contract_package: ContractPackageHash = runtime::get_named_arg(ARG_CONTRACT_PACKAGE);
    let installer_package: ContractPackageHash = runtime::get_named_arg(ARG_INSTALLER_PACKAGE);
    runtime::call_versioned_contract(
        contract_package,
        None,
        EXTERNAL_METHOD,
        runtime_args! {
            "installer_package" => installer_package
        },
    )
}
