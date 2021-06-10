prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cd messanger && cargo build --release --target wasm32-unknown-unknown

test-simple-upgrade:
	mkdir -p tests/wasm
	cp messanger/target/wasm32-unknown-unknown/release/*.wasm tests/wasm/
	cd tests && cargo test -- --nocapture

test: build-contract test-simple-upgrade

clippy:
	cd messanger && cargo clippy --all-targets --all -- -D warnings -A renamed_and_removed_lints
	cd tests && cargo clippy

check-lint: clippy
	cd messanger && cargo fmt --all -- --check

lint: clippy
	cd messanger && cargo fmt --all

format:
	cd messanger && cargo fmt 
	cd tests && cargo fmt

clean:
	cd messanger && cargo clean
	cd tests && cargo clean
	rm tests/wasm/*.wasm