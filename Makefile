.PHONY: build
build: 
	cargo build --release

.PHONY: clean
clean: 
	cargo clean
	rm dump*

.PHONY: check
check: 
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings
