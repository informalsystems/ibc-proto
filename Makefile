SDK_BRANCH=master

# Clone and fetch the Cosmos-SDK repo for the IBC proto files.
cosmos-sdk:
	@git clone --depth 1 https://github.com/cosmos/cosmos-sdk --branch "$(SDK_BRANCH)"

build: cosmos-sdk
	cargo clean
	cargo build --lib

clean:
	rm -rf cosmos-sdk target

.PHONY: clean build