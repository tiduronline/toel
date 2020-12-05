.PHONY: build test

run:
	cargo run

build:
	cargo build --release

test:
	cargo test