prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cd simple_upgrade && cargo build --release --target wasm32-unknown-unknown

test-simple-upgrade:
	mkdir -p tests/wasm
	cp simple_upgrade/target/wasm32-unknown-unknown/release/installer.wasm tests/wasm/
	cp simple_upgrade/target/wasm32-unknown-unknown/release/upgrader.wasm tests/wasm/
	cp simple_upgrade/target/wasm32-unknown-unknown/release/test.wasm tests/wasm/

	cd tests && cargo test -- --nocapture

test: build-contract test-simple-upgrade

clippy:
	cd simple_upgrade && cargo clippy --all-targets --all -- -D warnings -A renamed_and_removed_lints
	cd tests && cargo clippy

check-lint: clippy
	cd simple_upgrade && cargo fmt --all -- --check

lint: clippy
	cd simple_upgrade && cargo fmt --all

format:
	cd simple_upgrade && cargo fmt 
	cd tests && cargo fmt

clean:
	cd simple_upgrade && cargo clean
	cd tests && cargo clean
	rm tests/wasm/*.wasm