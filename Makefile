prepare:
	rustup target add wasm32-unknown-unknown

test_sec:
	cd secure_contract && cargo +nightly build --release --target wasm32-unknown-unknown

	cp secure_contract/target/wasm32-unknown-unknown/release/messenger.wasm tests/wasm
	cp secure_contract/target/wasm32-unknown-unknown/release/installer.wasm tests/wasm
	cp secure_contract/target/wasm32-unknown-unknown/release/test.wasm tests/wasm


test_nonsec:
	cd not_secure_contract && cargo +nightly build --release --target wasm32-unknown-unknown

	cp not_secure_contract/target/wasm32-unknown-unknown/release/nonsec_messenger.wasm tests/wasm
	cp not_secure_contract/target/wasm32-unknown-unknown/release/nonsec_installer.wasm tests/wasm
	cp not_secure_contract/target/wasm32-unknown-unknown/release/nonsec_test.wasm tests/wasm

	cd tests && cargo +nightly test -- --nocapture

test: test_sec test_nonsec

clippy:
	cd not_secure_contract && cargo +nightly clippy --target wasm32-unknown-unknown
	cd secure_contract && cargo +nightly clippy --target wasm32-unknown-unknown
	cd tests && cargo +nightly clippy

format:
	cd not_secure_contract && cargo fmt 
	cd secure_contract && cargo fmt 
	cd tests && cargo fmt

clean:
	cd not_secure_contract && cargo clean
	cd secure_contract && cargo clean 
	cd tests && cargo clean

	rm tests/wasm/messenger.wasm
	rm tests/wasm/installer.wasm
	rm tests/wasm/test.wasm

	rm tests/wasm/nonsec_messenger.wasm
	rm tests/wasm/nonsec_installer.wasm
	rm tests/wasm/nonsec_test.wasm