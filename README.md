# Simplest contract upgrading example

## Purpose

Demonstrate the general way of upgrading a contract and the necessary steps.

## Scenario

A company makes a campaign to post motivational quotes on the chain each day.
For this purpose they develop a smart contract (found in the `contract` folder).
After a few days they notice that not just their company posts in the contract,
and that they cannot easily read the earlier posts as they are overwritten with each post.
The company develops and deploys and upgrader (found in `contract_upgrade`) which adds a new contract version that:
- adds a security check so only the companys account can make posts.
- changes the storage so that the posts are stored in a dictionary, paired with their date.
- disables the first contract version, so the issues with the earlier version will not be exploitable in the future. 

## Important Notes

To add a new contract version or to disable one, the access token URef for the contract package is necessary to be in the context where these functions are called. The access token URef of a contract package is only created once with the creation of the contract package and cannot be recreated, replaced, or substituted later. For this reason storing the access token URef for a contract package is a mandatory step if you want to make changes to the package later.

When adding a new version of a contract you can add new entrypoint, or overwrite older ones, in addition remove certain ones by not adding the entrypoint to the set of entrypoints you add to the new contract version. Because of this if you want to keep entrypoint in a new version but you do not with to make changes to it, you still need to add it.

Named keys can be added or overwritten when adding a new version of a contract. Named keys are per contract package so every version has access to them, they are not lost. Because of this be mindful of data scheme incompatibilities between your contract versions, and disable ones that may cause breaks or bugs in your datasets.

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
This contract compiles and runs when using `rustc 1.63.0-nightly (c52b9c10b 2022-05-16)`

## Casper contract sdk version
casper-types = "1.5.0"
casper-contract = "1.4.4"
casper-execution-engine = "2.0.0"
casper-engine-test-support = "2.2.0"
