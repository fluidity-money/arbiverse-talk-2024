
CARGO_BUILD_STYLUS := \
	cargo build \
		--release \
		--target wasm32-unknown-unknown \
		--features

RELEASE_OUT := target/wasm32-unknown-unknown/release/lendingstablecoin.wasm

build: contract-factory.wasm contract-lending.wasm

contract-factory.wasm: $(shell find src -name '*.rs')
	@rm -f contract-factory.wasm
	@${CARGO_BUILD_STYLUS} contract-factory
	@mv ${RELEASE_OUT} contract-factory.wasm

contract-lending.wasm: $(shell find src -name '*.rs')
	@rm -f contract-lending.wasm
	@${CARGO_BUILD_STYLUS} contract-lending
	@mv ${RELEASE_OUT} contract-lending.wasm
