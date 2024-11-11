
include ../build.mk

build: fizzbuzzer.wasm

fizzbuzzer.wasm: $(shell find src -name '*.rs')
	@${CARGO_BUILD_STYLUS}
