#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]

use casper_types::ContractPackageHash;
use core::convert::TryInto;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    runtime_args, ApiError, CLType, CLTyped, RuntimeArgs, URef,
};

#[no_mangle]
pub extern "C" fn call() {
    //1. call package hash set_text
    let text: String = runtime::call_versioned_contract(
        get_key("messenger_package_hash"),
        None,
        "get_message",
        casper_types::runtime_args! {},
    );
    //2. assert previous to 'v1'
    if "v1" != text {
        runtime::revert(ApiError::User(1));
    }
    //3. call upgrade on the contract with passing the upgrader as the argument
    // runtime::revert(ApiError::User(67));
    let _ = runtime::call_versioned_contract::<()>(
        get_key("dao_contract_hash"),
        None,
        "upgrade",
        casper_types::runtime_args! {"installer_package_hash" => get_key::<ContractPackageHash>("installer_package_hash")},
    );
    runtime::revert(ApiError::User(70));
    //4. call package hash get_message on the upgraded contract
    let text_2: String = runtime::call_versioned_contract(
        get_key("messenger_package_hash"),
        None,
        "get_message",
        casper_types::runtime_args! {},
    );
    //5. assert previous to 'v2'
    if "v2" != text_2 {
        runtime::revert(ApiError::User(2));
    }
}

fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}
