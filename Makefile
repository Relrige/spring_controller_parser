build:
	cargo build

test:
	cargo test

run:
	cargo run -- parse examples/SampleController.java

help:
	cargo run -- help

credits:
	cargo run -- credits

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

clean:
	cargo clean

pre-commit: format clippy test
