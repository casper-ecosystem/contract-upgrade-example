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
use casperlabs_types::{CLType,runtime_args, U512, RuntimeArgs, CLTyped,
    bytesrepr::{FromBytes, ToBytes}, URef,
    contracts::{EntryPoints, EntryPoint, NamedKeys, EntryPointAccess, EntryPointType},
    ContractPackageHash, ContractVersion
};

const METHOD_SET_TEXT: &str = "set_text";
const METHOD_UPGRADE: &str = "upgrade_to";

const ACCESS_TOKEN: &str = "access_token";
const CONTRACT_PACKAGE: &str = "contract_package";
const CONTRACT_NAME: &str = "text_contract";
const CONTRACT_HASH: &str = "text_contract_hash";
const CONTRACT_VERSION: &str = "contract_version";

const TEXT_KEY: &str = "text";
const TEXT_VALUE_V1: &str = "value_one";

#[no_mangle]
pub extern "C" fn call() {
    let contract_package: ContractPackageHash = runtime::get_named_arg("contract_package");
    runtime::call_versioned_contract(contract_package, None, "set_text", runtime_args! {})
}
