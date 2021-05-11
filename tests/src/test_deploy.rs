use casper_engine_test_support::{Code, Hash, SessionBuilder, TestContext, TestContextBuilder};
use casper_types::{
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

    pub fn deploy_base_contract(&mut self) {
        let base_code = Code::from("messenger.wasm");
        let base_args = runtime_args! {};
        let base_session = SessionBuilder::new(base_code, base_args)
            .with_address(self.account_addr)
            .with_authorization_keys(&[self.account_addr])
            .build();
        self.context.run(base_session);
        println!("deployed messenger version");
    }

    pub fn deploy_upgrader_contract(&mut self) {
        let upgrader_code = Code::from("installer.wasm");
        let upgrader_args = runtime_args! {};
        let upgrader_session = SessionBuilder::new(upgrader_code, upgrader_args)
            .with_address(self.account_addr)
            .with_authorization_keys(&[self.account_addr])
            .build();
        self.context.run(upgrader_session);
        println!("deployed installer package")
    }

    pub fn deploy_dao_contract(&mut self) {
        let dao_code = Code::from("dao.wasm");
        let dao_args = runtime_args! {};
        let dao_session = SessionBuilder::new(dao_code, dao_args)
            .with_address(self.account_addr)
            .with_authorization_keys(&[self.account_addr])
            .build();
        self.context.run(dao_session);
        println!("deployed dao package")
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

    pub fn set_text(&mut self) {
        let session_code = Code::Hash(
            self.get_contract_package("messenger_package"),
            "get_text".to_string(),
        );
        let session_args = runtime_args! {};
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(self.account_addr)
            .with_authorization_keys(&[self.account_addr])
            .build();
        self.context.run(session);
        println!("set text to {}", self.get_text());
    }

    pub fn upgrade(&mut self) {
        let session_code =
            Code::NamedKey("messenger_contract".to_string(), "upgrade_me".to_string());
        let session_args =
            runtime_args! {"upgrader"=>self.get_contract_package("upgrader_package")};
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(self.account_addr)
            .with_authorization_keys(&[self.account_addr])
            .build();
        self.context.run(session);
        println!("upgraded contract");
    }

    pub fn get_text(&self) -> String {
        self.query(&[
            "messenger_contract".to_string(),
            "contract_version".to_string(),
        ])
        .unwrap()
    }

    pub fn get_contract_package(&self, contract_name: &str) -> Hash {
        self.query(&[contract_name.to_string()]).unwrap()
    }

    pub fn query<T: CLTyped + FromBytes>(&self, path: &[String]) -> Option<T> {
        match self.context.query(self.account_addr, path) {
            Err(err) => {
                println!("{:?}", err);
                None
            }
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not expected type.", path[0]));
                Some(value)
            }
        }
    }
}
