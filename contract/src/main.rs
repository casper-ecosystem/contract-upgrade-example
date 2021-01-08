#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]
// This contract creates a basic contract with a upgrade method

use core::convert::TryInto;

use casperlabs_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casperlabs_types::{CLType,runtime_args, RuntimeArgs, CLTyped, CLValue,
    bytesrepr::{FromBytes, ToBytes}, URef,
    contracts::{EntryPoints, EntryPoint, NamedKeys, EntryPointAccess, EntryPointType},
    ContractPackageHash
};

const METHOD_SET_TEXT: &str = "set_text";
const METHOD_UPGRADE: &str = "upgrade_to";
const METHOD_GET_ACCESS_TOKEN: &str = "get_access_token";

const ACCESS_TOKEN: &str = "access_token";
const CONTRACT_PACKAGE: &str = "contract_package";
const CONTRACT_NAME: &str = "text_contract";
const CONTRACT_HASH: &str = "text_contract_hash";

const TEXT_KEY: &str = "text";
const TEXT_VALUE_V1: &str = "value_one";


#[no_mangle]
pub extern "C" fn set_text() {
    set_key(TEXT_KEY, TEXT_VALUE_V1)
}

#[no_mangle]
pub extern "C" fn upgrade_to() {
    let installer_package: ContractPackageHash = runtime::get_named_arg("installer_package");
    let contract_package: ContractPackageHash = get_key(CONTRACT_PACKAGE);
    
    runtime::call_versioned_contract(installer_package, None, "install", runtime_args! {
        "contract_package" => contract_package,
    })
}

#[no_mangle]
pub extern "C" fn get_access_token() {
    let access_token: URef = runtime::get_key(ACCESS_TOKEN).unwrap_or_revert().into_uref().unwrap_or_revert();
    runtime::ret(CLValue::from_t(access_token).unwrap());
}

#[no_mangle]
pub extern "C" fn call() {
    let (contract_package, access_token) = storage::create_contract_package_at_hash();
    
    let entry_points = {
        let mut entry_points = EntryPoints::new();
        let deposit = EntryPoint::new(
            METHOD_SET_TEXT,
            vec![],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );
        entry_points.add_entry_point(deposit);

        let upgrade = EntryPoint::new(
            METHOD_UPGRADE,
            vec![],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );
        entry_points.add_entry_point(upgrade);

        let get_access_token = EntryPoint::new(
            METHOD_GET_ACCESS_TOKEN,
            vec![],
            CLType::URef,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );
        entry_points.add_entry_point(get_access_token);

        entry_points
    };
    
    // this should overwrite the previous contract obj with the new contract obj at the same uref
    let mut named_keys = NamedKeys::new();
    named_keys.insert(ACCESS_TOKEN.to_string(), access_token.into());
    named_keys.insert(CONTRACT_PACKAGE.to_string(), storage::new_uref(contract_package).into());
    let (new_contract_hash, _) =
        storage::add_contract_version(contract_package, entry_points, named_keys);
    
    
    runtime::put_key(CONTRACT_NAME, new_contract_hash.into());
    set_key(CONTRACT_PACKAGE, contract_package); // stores contract package hash under account's named key
    set_key(CONTRACT_HASH, new_contract_hash);
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
