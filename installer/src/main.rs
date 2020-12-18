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
use casperlabs_types::{CLType, CLTyped, Key, CLValue,
    bytesrepr::{FromBytes, ToBytes},
    contracts::{EntryPoints, EntryPoint, NamedKeys, Parameter, EntryPointAccess, EntryPointType}
};

const METHOD_GET_TEXT: &str = "get_text";
const METHOD_DEPOSIT_V2: &str = "deposit_v2";
const METHOD_INSTALL: &str = "install";
const INCOMMING_PURSE: &str = "incomming_purse";
const CONTRACT_PACKAGE: &str = "contract_package";
const ACCESS_TOKEN: &str = "access_token";
const CONTRACT_NAME: &str = "deposit_box";
const CONTRACT_HASH: &str = "deposit_box_hash";

const CONTRACT_VERSION: &str = "contract_version";
const TEXT_KEY: &str = "text";
const TEXT_VALUE_V1: &str = "value_one";
const TEXT_VALUE_V2: &str = "value_two";


#[no_mangle]
pub extern "C" fn get_text() {
    runtime::ret(CLValue::from_t("I'm v1").unwrap());
}

#[no_mangle]
pub extern "C" fn install() {
    // 1. Create endpoints.
    let entry_points = {
        let mut entry_points = EntryPoints::new();
        let deposit = EntryPoint::new(
            METHOD_GET_TEXT,
            vec![Parameter::new(INCOMMING_PURSE, CLType::URef)],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );
        entry_points.add_entry_point(deposit);
        entry_points
    };

    // 2. Use package_hash from args to install v2.
    

}


#[no_mangle]
pub extern "C" fn call() {
    let (contract_package, access_token) = storage::create_contract_package_at_hash();
    
    let entry_points = {
        let mut entry_points = EntryPoints::new();
        let deposit = EntryPoint::new(
            METHOD_GET_TEXT,
            vec![Parameter::new(INCOMMING_PURSE, CLType::URef)],
            CLType::Unit,
            EntryPointAccess::Groups(vec![]),
            EntryPointType::Contract,
        );
        entry_points.add_entry_point(deposit);

        let upgrade = EntryPoint::new(
            METHOD_INSTALL,
            vec![],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );
        entry_points.add_entry_point(upgrade);

        entry_points
    };
    
    // this should overwrite the previous contract obj with the new contract obj at the same uref
    let mut named_keys = NamedKeys::new();
    named_keys.insert(ACCESS_TOKEN.to_string(), access_token.into());
    named_keys.insert(CONTRACT_PACKAGE.to_string(), contract_package.into());
    let (new_contract_hash, new_contract_version) =
        storage::add_contract_version(contract_package, entry_points, named_keys);
 
    runtime::put_key(CONTRACT_NAME, new_contract_hash.into());
    set_key(CONTRACT_HASH, new_contract_hash);
    set_key(CONTRACT_VERSION, new_contract_version);
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