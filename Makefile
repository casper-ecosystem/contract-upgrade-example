prepare:
	rustup target add wasm32-unknown-unknown

test:
	cd contract && cargo +nightly build --release --target wasm32-unknown-unknown

	cp contract/target/wasm32-unknown-unknown/release/messenger.wasm tests/wasm
	cp contract/target/wasm32-unknown-unknown/release/installer.wasm tests/wasm
	cp contract/target/wasm32-unknown-unknown/release/test.wasm tests/wasm

	cd tests && cargo +nightly test -- --nocapture

format:
	cd contract && cargo fmt 
	cd tests && cargo fmt

clean:
	cd contract && cargo clean 
	cd tests && cargo clean
	rm tests/wasm/messenger.wasm
	rm tests/wasm/installer.wasm
	rm tests/wasm/test.wasm