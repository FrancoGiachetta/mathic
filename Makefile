.PHONY: build
build: 
	cargo build --release

.PHONY: check
check: 
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: clean
clean: clean-dumps 
	cargo clean
	rm -rf Dialeclects/builds Dialects/.cache

.PHONY: clean-dumps
clean-dumps:
	rm -f *.mlir *.mathir

.PHONY: fmt
fmt:
	cargo fmt
	$(MAKE) -C Dialects fmt

.PHONY: test
test:
	cargo nextest run test


