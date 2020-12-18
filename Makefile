test:
	cd contract && cargo build --release
	cd installer && cargo build --release
	cp contract/target/wasm32-unknown-unknown/release/contract.wasm tests/wasm
	cp installer/target/wasm32-unknown-unknown/release/installer.wasm tests/wasm
	cd tests && cargo test
