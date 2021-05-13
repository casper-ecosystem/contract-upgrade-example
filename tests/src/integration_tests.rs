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
    }

    // #[test]
    fn upgrade_messenger_with_dao() {
        let mut upgrade_test = ContractUpgrader::setup();
        // Deploy messenger and dao contracts
        upgrade_test.deploy_contract("messenger.wasm");
        // Deploy upgrade installer package
        upgrade_test.deploy_contract("installer.wasm");
        // Deploy and run test scenario
        upgrade_test.deploy_contract("test.wasm");
    }

    #[test]
    fn insecure_upgrade() {
        let mut upgrade_test = ContractUpgrader::setup();
        // Deploy messenger and dao contracts
        upgrade_test.deploy_contract("nonsec_messenger.wasm");
        // Deploy upgrade installer package
        upgrade_test.deploy_contract("nonsec_installer.wasm");
        // Deploy and run test scenario
        upgrade_test.deploy_contract("nonsec_test.wasm");
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
