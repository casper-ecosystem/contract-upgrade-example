#[cfg(test)]
mod utils;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use casper_engine_test_support::{InMemoryWasmTestBuilder, DEFAULT_RUN_GENESIS_REQUEST};
    use casper_types::bytesrepr::FromBytes;

    use crate::utils::{deploy, fund_account, query, DeploySource};
    use casper_types::{
        account::AccountHash, runtime_args, CLTyped, Key, PublicKey, RuntimeArgs, SecretKey, URef,
    };

    pub struct Contract {
        pub builder: InMemoryWasmTestBuilder,
        pub alice_account: AccountHash,
        pub bob_account: AccountHash,
    }

    impl Contract {
        pub fn deploy() -> Self {
            // We create 2 accounts. "alice" will be the one who installs the contract.
            let alice_public_key: PublicKey =
                PublicKey::from(&SecretKey::ed25519_from_bytes([1u8; 32]).unwrap());
            let bob_public_key: PublicKey =
                PublicKey::from(&SecretKey::ed25519_from_bytes([2u8; 32]).unwrap());
            // Get addresses for participating accounts.
            let alice_account = AccountHash::from(&alice_public_key);
            let bob_account = AccountHash::from(&bob_public_key);

            // Set up the test framework and fund accounts
            let mut builder = InMemoryWasmTestBuilder::default();
            builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();
            builder
                .exec(fund_account(&alice_account))
                .expect_success()
                .commit();
            builder
                .exec(fund_account(&bob_account))
                .expect_success()
                .commit();

            // install contract
            let code = PathBuf::from("contract.wasm");
            deploy(
                &mut builder,
                &alice_account,
                &DeploySource::Code(code),
                runtime_args! {},
                None,
            );

            Self {
                builder,
                alice_account,
                bob_account,
            }
        }

        /// Function that handles the creation and execution of deploys.
        fn call(&mut self, caller: AccountHash, contract_name: &str, entry_point: &str, args: RuntimeArgs) {
            deploy(
                &mut self.builder,
                &caller,
                &DeploySource::ByContractName {
                    name: contract_name.to_string(),
                    entry_point: entry_point.to_string(),
                },
                args,
                None,
            );
        }

        pub fn query_dictionary_value<T: CLTyped + FromBytes>(
            &self,
            base: Key,
            dict_name: &str,
            key: &str,
        ) -> T {
            crate::utils::query_dictionary_item(
                &self.builder,
                base,
                Some(dict_name.to_string()),
                key,
            )
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t()
            .expect("Wrong type in query result.")
        }
    }

    #[test]
    fn test_base() {
        let mut context = Contract::deploy();
        context.call(
            context.alice_account,
            "post_board_contract_hash_1",
            "post",
            runtime_args! {"post" => "post"},
        );
        let post: String = query(
            &context.builder,
            Key::Account(context.alice_account),
            &["post_board_contract_hash_1".to_string(), "post".to_string()],
        );
        assert_eq!(post, "post");
    }

    #[test]
    fn test_upgrade() {
        let mut context = Contract::deploy();

        let code = PathBuf::from("upgrade.wasm");
        deploy(
            &mut context.builder,
            &context.alice_account,
            &DeploySource::Code(code),
            runtime_args! {},
            None,
        );

        context.call(
            context.alice_account,
            "post_board_contract_hash_1",
            "post",
            runtime_args! {"post" => "post"},
        );

        let post: String = query(
            &context.builder,
            Key::Account(context.alice_account),
            &["post_board_contract_hash_1".to_string(), "post".to_string()],
        );
        assert_eq!(post, "post");

        context.call(
            context.alice_account,
            "post_board_contract_hash_2",
            "post",
            runtime_args! {"date"=>"today", "post" => "post"},
        );

        let dict_uref: URef = query(
            &context.builder,
            Key::Account(context.alice_account),
            &["posts_uref_key".to_string()],
        );

        let post = context.query_dictionary_value::<String>(Key::URef(dict_uref), "posts", "today");

        assert_eq!(post, "post");
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
