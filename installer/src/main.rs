#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]

use core::convert::TryInto;

use casperlabs_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casperlabs_types::{CLType, URef, CLTyped, ContractPackageHash, RuntimeArgs, runtime_args,
    bytesrepr::{ToBytes},
    contracts::{EntryPoints, EntryPoint, Parameter, EntryPointAccess, EntryPointType}
};

const METHOD_SET_TEXT: &str = "set_text";
const METHOD_INSTALL: &str = "install";
const CONTRACT_PACKAGE: &str = "installer_package";
const CONTRACT_NAME: &str = "installer_contract";
const CONTRACT_HASH: &str = "installer_contract_hash";
const TEXT_KEY: &str = "text";
const TEXT_VALUE_V2: &str = "value_two";


#[no_mangle]
pub extern "C" fn set_text() {
    set_key(TEXT_KEY, TEXT_VALUE_V2);
}

#[no_mangle]
pub extern "C" fn install() {
    let contract_package: ContractPackageHash = runtime::get_named_arg("contract_package");
    let _access_token: URef = runtime::call_versioned_contract(contract_package, None, "get_access_token", runtime_args!{});

    // // 1. Create endpoints.
    let entry_points = {
        let mut entry_points = EntryPoints::new();
        let gettext = EntryPoint::new(
            METHOD_SET_TEXT,
            vec![],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );
        entry_points.add_entry_point(gettext);
        entry_points
    };

    // 2. Use package_hash from args to install v2.
    let (_, _) =
        storage::add_contract_version(contract_package.into(), entry_points, Default::default());
}


#[no_mangle]
pub extern "C" fn call() {
    let (contract_package, _access_token) = storage::create_contract_package_at_hash();
    
    let entry_points = {
        let mut entry_points = EntryPoints::new();
        let deposit = EntryPoint::new(
            METHOD_SET_TEXT,
            vec![],
            CLType::Unit,
            EntryPointAccess::Groups(vec![]),
            EntryPointType::Contract,
        );
        entry_points.add_entry_point(deposit);

        let upgrade = EntryPoint::new(
            METHOD_INSTALL,
            vec![
                Parameter::new("package_hash", ContractPackageHash::cl_type()),
                Parameter::new("access_token", CLType::URef)
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );
        entry_points.add_entry_point(upgrade);

        entry_points
    };

    // this should overwrite the previous contract obj with the new contract obj at the same uref
    let (new_contract_hash, _) =
        storage::add_contract_version(contract_package, entry_points, Default::default());
 
    runtime::put_key(CONTRACT_NAME, new_contract_hash.into());
    set_key(CONTRACT_HASH, new_contract_hash);
    set_key(CONTRACT_PACKAGE, contract_package); // stores contract package hash under account's named key

}


fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}
