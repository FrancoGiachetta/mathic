.PHONY: build
build: 
	cargo build --release

.PHONY: clean
clean: 
	cargo clean

.PHONY: clippy
clippy: 
	cargo clippy --all-targets --all-features -- -D warnings
