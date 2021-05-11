#[cfg(test)]
mod tests {
    pub use casper_engine_test_support::{Code, SessionBuilder, TestContext, TestContextBuilder};
    pub use casper_types::{
        account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractHash, PublicKey,
        RuntimeArgs, SecretKey, U512,
    };

    pub struct ContractUpgrader {
        context: TestContext,
        account_addr: AccountHash,
    }

    impl ContractUpgrader {
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

        pub fn deploy_messenger_and_dao_contract(&mut self) {
            let base_code = Code::from("messenger.wasm");
            let base_args = runtime_args! {};
            let base_session = SessionBuilder::new(base_code, base_args)
                .with_address(self.account_addr)
                .with_authorization_keys(&[self.account_addr])
                .build();
            self.context.run(base_session);
            println!("deployed messenger version");
        }

        pub fn deploy_upgrade_installer_contract(&mut self) {
            let upgrader_code = Code::from("installer.wasm");
            let upgrader_args = runtime_args! {};
            let upgrader_session = SessionBuilder::new(upgrader_code, upgrader_args)
                .with_address(self.account_addr)
                .with_authorization_keys(&[self.account_addr])
                .build();
            self.context.run(upgrader_session);
            println!("deployed installer package")
        }

        pub fn deploy_upgrader_test_contract(&mut self) {
            let upgrader_test_code = Code::from("test.wasm");
            let upgrader_test_args = runtime_args! {};
            let upgrader_test_session = SessionBuilder::new(upgrader_test_code, upgrader_test_args)
                .with_address(self.account_addr)
                .with_authorization_keys(&[self.account_addr])
                .build();
            self.context.run(upgrader_test_session);
            println!("deployed test package")
        }
    }

    #[test]
    fn upgrade_contract_text() {
        let mut upgrade_test = ContractUpgrader::setup();
        // Deploy messenger and dao contracts
        upgrade_test.deploy_messenger_and_dao_contract();
        // Deploy upgrade installer package
        upgrade_test.deploy_upgrade_installer_contract();
        // Deploy and run test scenario
        upgrade_test.deploy_upgrader_test_contract();
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
