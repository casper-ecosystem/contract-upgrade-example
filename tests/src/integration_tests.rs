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
            let public_key: PublicKey = SecretKey::ed25519([1u8; 32]).into();
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
            let base_code = Code::from("test.wasm");
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
    }

    #[test]
    fn test_simple_upgrade() {
        let mut upgrade_test = ContractUpgrader::setup();
        upgrade_test.deploy_contract("installer.wasm");
        upgrade_test.assert_msg("v1");

        upgrade_test.deploy_contract("upgrader.wasm");
        upgrade_test.assert_msg("v2");
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
