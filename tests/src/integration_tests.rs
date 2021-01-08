#[cfg(test)]
// This is a test file, demostrating how to upgrade our first contract
mod tests {
    use casperlabs_engine_test_support::{Code, SessionBuilder, TestContextBuilder, TestContext};

    use casperlabs_types::{account::AccountHash, ContractHash, U512, RuntimeArgs, runtime_args, CLTyped, bytesrepr::FromBytes};

// lets start up by creating an account with keys
    const MY_ACCOUNT: AccountHash = AccountHash::new([7u8; 32]);

    const CONTRACT_NAME: &str = "text_contract";
    const TEXT_KEY: &str = "text";
    const TEXT_VALUE_V1: &str = "value_one";
    const TEXT_VALUE_V2: &str = "value_two";

    #[test]
    fn upgrade_contract_text() {
	// In order to run the contracts, we will create a context and attach our contracts to it
        let mut context = TestContextBuilder::new()
            .with_account(MY_ACCOUNT, U512::from(128_000_000)) // create it with our account, so we can use our keys all the way
            .build();
	// lets use the wasm file our contract code has generated
        let session_code = Code::from("contract.wasm");
        let session_args = runtime_args! {};
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(MY_ACCOUNT) //create the session with our account
            .with_authorization_keys(&[MY_ACCOUNT]) 
            .build();
        context.run(session); //run the first smartcontract "contract"

	// lets now call our installer contract, so we can start the upgrade
        let session_code = Code::from("installer.wasm");
        let session_args = runtime_args! {};
        let session = SessionBuilder::new(session_code, session_args)
        .with_address(MY_ACCOUNT)
        .with_authorization_keys(&[MY_ACCOUNT])
        .build();
        context.run(session);// run the installer contract with the same context
	// lets execute the set_text function in our first contract
        call_contract_package_set_text(&mut context);
	// lets verify that it says version 1
	assert_eq!(get_text(&context), TEXT_VALUE_V1);
        println!("Contract text value is now: {:?}", get_text(&context));
	//call and activate the upgrade function
        call_contract_package_upgrade(&mut context);
	// the upgrade is done! Lets tell it to change our text with our call-set-text contract
        call_contract_package_set_text(&mut context);
	// text is now changed and our contract is upgraded!
        println!("Contract text value is now: {:?}", get_text(&context));
	// lets verify that the upgrade has worked and is returning the new text
	assert_eq!(get_text(&context), TEXT_VALUE_V2);
    }


	/// use the call-set-text to call the set text function in the contract we define 
	fn call_contract_package_set_text(context: &mut TestContext) {
        let contract_package_hash = get_contract_package(&context); // grab the contract package hash that we want the call-set-text smart contract to use
        let code = Code::from("call-set-text.wasm");
        let args = runtime_args! {
            "contract_package" => contract_package_hash, // we will pass the contract package as an argument so that call-set-text can read it
        };
	// lets create a session and execute the contract!
        let session = SessionBuilder::new(code, args)
            .with_address(MY_ACCOUNT)
            .with_authorization_keys(&[MY_ACCOUNT])
            .build();
        context.run(session);
    }

	/// lets call the upgrade function and overwrite the old functions with new once
	fn call_contract_package_upgrade(context: &mut TestContext) {
        let contract_package = get_contract_package(&context);
        let installer_package = get_installer_package(&context);
//lets execute our call-upgrade contract so we can upgrade the original contract with our new functions
        let code = Code::from("call-upgrade.wasm");
        let args = runtime_args! {
// pass along the contract hashes of our installer and contract package
            "contract_package" => contract_package,
            "installer_package" => installer_package,
        };
        let session = SessionBuilder::new(code, args)
            .with_address(MY_ACCOUNT)
            .with_authorization_keys(&[MY_ACCOUNT])
            .build();
        context.run(session);
    }


/// Query a contract for the contract_package key
    fn get_contract_package(context: &TestContext) -> ContractHash {
        query_account(context, "contract_package").unwrap()
    }

/// Query a contract for the installer_package key
    fn get_installer_package(context: &TestContext) -> ContractHash {
        query_account(context, "installer_package").unwrap()
    }

/// Query a contract for the stored text key
    fn get_text(context: &TestContext) -> String {
        query_contract(context, TEXT_KEY).unwrap()
    }

/// Query an account
    fn query_account<T: CLTyped + FromBytes>(context: &TestContext, key: &str) -> Option<T> {
        query(context, &[key])
    }

/// Query a contract for a key with a context and the contracts package hash
    fn query_contract<T: CLTyped + FromBytes>(context: &TestContext, key: &str) -> Option<T> {
        query(context, &[CONTRACT_NAME, key])
    }

    fn query<T: CLTyped + FromBytes>(context: &TestContext, path: &[&str]) -> Option<T> {
        match context
            .query(MY_ACCOUNT, path)
        {
            Err(err) => {
                println!("{:?}", err);
                None
            },
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not expected type.", path[0]));
                Some(value)
            }
        }
    }

}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
