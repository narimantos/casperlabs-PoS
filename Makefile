build-contract:
	caspiler contract/src/erc20.sol > contract/src/main.rs
	cd contract && cargo fmt
	cd contract && cargo build --release

copy-wasm:
	cp contract/target/wasm32-unknown-unknown/release/contract.wasm tests/wasm/

test-only:
	cd tests && cargo test --lib 

test: build-contract copy-wasm test-only