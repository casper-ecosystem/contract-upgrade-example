prepare:
	rustup default nightly-2021-09-17-x86_64-unknown-linux-gnu
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build -p messanger --release --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/messanger_v1_install.wasm
	wasm-strip target/wasm32-unknown-unknown/release/messanger_v2_upgrade.wasm
	

test-simple-upgrade:
	mkdir -p tests/wasm
	cp target/wasm32-unknown-unknown/release/*.wasm tests/wasm/
	cargo test -p tests -- --nocapture

test: build-contract test-simple-upgrade

clippy:
	cargo clippy -p messanger --all-targets --all -- -D warnings -A renamed_and_removed_lints
	cargo clippy -p tests

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

format:
	cargo fmt 

clean:
	cargo clean
	cargo clean
	rm tests/wasm/*.wasm