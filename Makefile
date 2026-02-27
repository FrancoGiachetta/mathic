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

.PHONY: test
test:
	cargo nextest run

.PHONY: clean-dumps
clean-dumps:
	rm *.mlir *.mathir
