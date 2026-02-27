.PHONY: build
build: 
	cargo build --release

.PHONY: clean
clean: clean-dumps 
	cargo clean

.PHONY: check
check: 
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: clean-dumps
clean-dumps:
	rm *.mlir *.mathir
