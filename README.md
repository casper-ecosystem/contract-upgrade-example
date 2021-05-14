# Simplest contract upgrading example

## Purpose

This example demonstrates the easiest and simplest way to upgrade or overwrite a contract.
However this method, as is, is not recommended because it is not secure.
The originally deployed contract stores its access token into the account's storage.
With this the contract can be overwritten without first passing said access token and
checking for privileges. We recommend you do not store access tokens in places that can be accessed
by others with ease, and that you write further access right checks. The framework does not allow you
to retrieve access tokens from wasm execution context.

## Scenario

The way the testing of the contract upgrade play out is as follows.
- Deployment of `messenger` contract into the contract tester context.
This contract contains a `get_message` entrypoint that returns the text "v1".
(`messenger` is found in `/simple_upgrade/src/installer.rs` compiled to `installer.wasm`)

- Executing the `test` contract on the tester context with the argument called `expected` having value "v1".
`test` calls `get_message` on the `messenger` contract, then asserts the return value to the `expected` argument. 
(`test` found in `/simple_upgrade/src/test.rs` compiled to `test.wasm`)

- Deployment of `upgrader` contract into the contract tester context.
`upgrader` overwrites the `get_message` entrypoint to return "v2", and saves it under the hash
of the original `messenger`contract. From this point forward if you call `get_message` on the `messenger` contract
you should recieve "v2" as the result.
(`upgrader` is found in `/simple_upgrade/src/upgrader.rs` compiled to `upgrader.wasm`)

- Call `test` again with "v2" as the `expected` argument to check if the upgrade went as expected.

## Make commands
### prepare
Adds wasm to the cargo compilation targets.

### build-contract
Builds the contracts using the nightly toolchain with wasm as the compilation target.

### test-simple-upgrade
Copies the `.wasm` files into `/tests/wasm` folder, where the test framework is set to look for them.

### test
Executes the `build-contract` and `test-simple-upgrade` commands.

### clippy
Executes the clippy linter on the contract and test codes.

### format
Applies formatting to the codes.

### clean
Artifact removal command. (`.wasm` files, `target` folders)