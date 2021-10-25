# Simplest contract upgrading example

## Purpose

This example demonstrates the easiest and simplest way to upgrade a contract.
The ownership of the contract is granted to the contract deployer.
With this, the contract can be overwritten. We recommend you do not store access
tokens in places that can be accessed by others with ease, and that you write
further access right checks.

## Scenario

The way the testing of the contract upgrade play out is as follows.
- Deployment of `messenger` contract into the contract tester context.
This contract contains a `get_message` entrypoint that returns the text "first".
(`messenger` is found in `/messanger/src/messanger_v1_install.rs` compiled
to `messanger_v1_install.wasm`)

- Executing the `assert_message` contract on the tester context with the argument
called `expected` having value "first". `assert_message` calls `get_message` on
the `messenger` contract, then asserts the return value to the `expected` argument. 
(`assert_message` found in `/messanger/src/assert_message.rs` compiled
to `assert_message.wasm`)

- Upgrade of `messanger` contract into the next version.
`messanger_v2_upgrade` overwrites the `get_message` entrypoint to return "second".
From this point forward if you call `get_message` on the `messenger` contract
you should recieve "second" as the result.
(`messanger_v2_upgrade` is found in `/simple_upgrade/src/messanger_v2_upgrade.rs`
compiled to `messanger_v2_upgrade.wasm`)

- Call `assert_message` again with "second" as the `expected` argument to check
if the upgrade went as expected.

### Named Keys

When upgrading a contract you cannot overwrite existing named keys, but you can create new ones.
This does not mean that you cannot overwrite the named keys at all, you simply cannot do it
by passing them to the `add_contract_version` function as parameter, and even if you do it
it will not take any effect.

### Locked Contracts

There is also a demonstration of the contract locking feature.
Locking a contract is permanent, and as such you will never be able to upgrade it.
This contract can be found at `/simple_upgrade/src/messanger_locked_install.rs`

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

## Rust version
This contract compiles and runs when using `rustc 1.57.0-nightly (e4828d5b7 2021-09-16)`

## Casper contract sdk version
casper-types = "1.4.1"
casper-contract = "1.4.1"
casper-engine-test-support = "1.4.1"

