#[cfg(test)]
mod tests {
    pub use casper_engine_test_support::{Code, SessionBuilder, TestContext, TestContextBuilder};
    pub use casper_types::{
        account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractHash, PublicKey,
        RuntimeArgs, SecretKey, U512,
    };

    /// Struct to hold test relevant data, such as context and account_hash
    pub struct ContractUpgrader {
        context: TestContext,
        account_addr: AccountHash,
    }

    impl ContractUpgrader {
        /// Test context constructor
        pub fn setup() -> Self {
            let secret_key = SecretKey::ed25519_from_bytes([1u8; 32]).unwrap();
            let public_key = PublicKey::from(&secret_key);
            let account_addr = AccountHash::from(&public_key);
            let context = TestContextBuilder::new()
                .with_public_key(public_key, U512::from("128000000000"))
                .build();
            Self {
                context,
                account_addr,
            }
        }

        /// Introduce a new contract to the test, that we try to open from the file ~/tests/wasm/$pack
        pub fn deploy_contract(&mut self, pack: &str) {
            let base_code = Code::from(pack);
            let base_args = runtime_args! {};
            let base_session = SessionBuilder::new(base_code, base_args)
                .with_address(self.account_addr)
                .with_authorization_keys(&[self.account_addr])
                .build();
            self.context.run(base_session);
            println!("deployed {}", pack);
        }

        /// Execute the code of ~/tests/wasm/test.wasm with the argument named "expected"
        pub fn assert_msg(&mut self, msg: &str) {
            let base_code = Code::from("assert_message.wasm");
            let base_args = runtime_args! {
                "expected" => msg
            };
            let base_session = SessionBuilder::new(base_code, base_args)
                .with_address(self.account_addr)
                .with_authorization_keys(&[self.account_addr])
                .build();
            self.context.run(base_session);
            println!("asserted {}", msg);
        }

        fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
            match self
                .context
                .query(self.account_addr, &["hash".to_string(), name.to_string()])
            {
                Err(_) => None,
                Ok(maybe_value) => {
                    let value = maybe_value
                        .into_t()
                        .unwrap_or_else(|_| panic!("{} is not expected type.", name));
                    Some(value)
                }
            }
        }
    }

    #[test]
    fn test_simple_upgrade() {
        // Setup test context
        let mut upgrade_test = ContractUpgrader::setup();
        // Introduce the original contract to the test system.
        upgrade_test.deploy_contract("messanger_v1_install.wasm");
        // Check for version 1 of the contract in the system.
        upgrade_test.assert_msg("first");

        // When we called the `get_message` we also stored the message "first" to the key "message".
        assert_eq!(
            "first",
            upgrade_test.query_contract::<String>("message").unwrap()
        );

        // When we deployed the contract we stored "version_original" string to the "version" key.
        assert_eq!(
            "version_original",
            upgrade_test.query_contract::<String>("version").unwrap()
        );

        // Deploy upgrader that overwrites the original contract.
        upgrade_test.deploy_contract("messanger_v2_upgrade.wasm");
        // Check whether the contract has been changed to version 2.
        upgrade_test.assert_msg("second");

        // The upgraded contract overwrites the "message" keys value to "second".
        assert_eq!(
            "second",
            upgrade_test.query_contract::<String>("message").unwrap()
        );

        // During upgrade we supplied the "version" key with a new "version_updated" value,
        // but this does not take effect as the key already existed,
        // and so the value remained "version_original".
        assert_eq!(
            "version_original",
            upgrade_test.query_contract::<String>("version").unwrap()
        );

        // On the otherhand we created a new key "version2" and this was indeed stored.
        assert_eq!(
            "version_updated",
            upgrade_test.query_contract::<String>("version2").unwrap()
        );
    }

    #[test]
    #[should_panic = "LockedContract"]
    fn test_locked_upgrade() {
        // Setup test context
        let mut upgrade_test = ContractUpgrader::setup();
        // Introduce the locked contract to the test system.
        upgrade_test.deploy_contract("messanger_locked_install.wasm");
        // See if we succesfully deployed the locked contract.
        upgrade_test.assert_msg("locked");

        // When we called the `get_message` we also stored the message "locked" to the key "message".
        assert_eq!(
            "locked",
            upgrade_test.query_contract::<String>("message").unwrap()
        );

        // When we deployed the contract we stored "locked_version" string to the "version" key.
        assert_eq!(
            "locked_version",
            upgrade_test.query_contract::<String>("version").unwrap()
        );

        // Here we try to upgrade the locked contract, this will inherently fail.
        upgrade_test.deploy_contract("messanger_v2_upgrade.wasm");
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
