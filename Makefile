test:
	cd contract && cargo build --release
	cp contract/target/wasm32-unknown-unknown/release/contract.wasm tests/wasm
	cd tests && cargo test -- --nocapture

