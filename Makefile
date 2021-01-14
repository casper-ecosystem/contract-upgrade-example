test:
	cd contract && cargo build --release
	cd installer && cargo build --release
	cd call_set_text && cargo build --release
	cd call_upgrade && cargo build --release

	cp contract/target/wasm32-unknown-unknown/release/contract.wasm tests/wasm
	cp installer/target/wasm32-unknown-unknown/release/installer.wasm tests/wasm
	cp call_set_text/target/wasm32-unknown-unknown/release/call-set-text.wasm tests/wasm
	cp call_upgrade/target/wasm32-unknown-unknown/release/call-upgrade.wasm tests/wasm
	
	cd tests && cargo test -- --nocapture

format:
	cd contract && cargo fmt 
	cd installer && cargo fmt
	cd call_set_text && cargo fmt
	cd call_upgrade && cargo fmt
	cd tests && cargo fmt

clean:
	cd contract && cargo clean 
	cd installer && cargo clean
	cd call_set_text && cargo clean
	cd call_upgrade && cargo clean
	cd tests && cargo clean
	rm tests/wasm/call-set-text.wasm
	rm tests/wasm/call-upgrade.wasm
	rm tests/wasm/installer.wasm
	rm tests/wasm/contract.wasm