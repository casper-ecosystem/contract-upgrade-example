extern crate casper_engine_test_support;
extern crate casper_types;

mod test_deploy;

#[cfg(test)]
mod tests {
    use super::test_deploy::ContractUpgrader;
    pub use casper_engine_test_support::{Code, SessionBuilder, TestContext, TestContextBuilder};
    pub use casper_types::{
        account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractHash, PublicKey,
        RuntimeArgs, SecretKey, U512,
    };

    #[test]
    fn upgrade_contract_text() {
        let mut upgrade_test = ContractUpgrader::setup();
        upgrade_test.deploy_base_contract();
        upgrade_test.deploy_upgrader_contract();
        upgrade_test.deploy_upgrader_test_contract();
        // upgrade_test.set_text();
        // assert_eq!(upgrade_test.get_text(), "v1");
        // upgrade_test.upgrade();
        // upgrade_test.set_text();
        // assert_eq!(upgrade_test.get_text(), "v2");
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
