prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cd simple_upgrade && cargo +nightly build --release --target wasm32-unknown-unknown

test-simple-upgrade:
	cp simple_upgrade/target/wasm32-unknown-unknown/release/installer.wasm tests/wasm
	cp simple_upgrade/target/wasm32-unknown-unknown/release/upgrader.wasm tests/wasm
	cp simple_upgrade/target/wasm32-unknown-unknown/release/test.wasm tests/wasm

	cd tests && cargo +nightly test -- --nocapture

test: build-contract test-simple-upgrade

clippy:
	cd simple_upgrade && cargo +nightly clippy --target wasm32-unknown-unknown
	cd tests && cargo +nightly clippy

format:
	cd simple_upgrade && cargo fmt 
	cd tests && cargo fmt

clean:
	cd simple_upgrade && cargo clean
	cd tests && cargo clean
	rm tests/wasm/*.wasm