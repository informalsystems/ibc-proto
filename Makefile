SDK_BRANCH=master
#Cosmos SDK master at 2020-JUL-11 1:41PM, commit ID 152ac249315a50f428dce490a8103d93177fadd7

# Clone and fetch the Cosmos-SDK repo for the IBC proto files.
cosmos-sdk:
	@git clone --depth 1 https://github.com/cosmos/cosmos-sdk --branch "$(SDK_BRANCH)"

build: cosmos-sdk
	cargo clean
	cargo run --bin ibc-prost-compiler
	cargo build --lib

clean:
	rm -rf cosmos-sdk target

publish:
	cd ibc_proto && cargo publish --dry-run --allow-dirty
	@echo "*** Run: 'cd ibc_proto && cargo publish' to really publish."

.PHONY: clean build
