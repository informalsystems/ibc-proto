update:
	git submodule init
	git submodule update

build:
	cargo run --bin ibc-prost-compiler
	cargo build --lib

clean:
	cargo clean

publish:
	cd ibc_proto && cargo publish --dry-run --allow-dirty
	@echo "*** Run: 'cd ibc_proto && cargo publish' to really publish."

.PHONY: update clean build publish
