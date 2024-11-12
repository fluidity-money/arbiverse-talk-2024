
include ../build.mk

build: hello_world.wasm

hello_world.wasm: $(shell find src -name '*.rs')
	@${CARGO_BUILD_STYLUS}
