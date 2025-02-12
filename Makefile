.PHONY: build run test clean fmt fmt-check clippy taplo taplo-check deny-check pre-release

build:
	cargo build --all

release:
	cargo build --release

run:
	cargo run

test:
	cargo test --features eth
	cargo test --features optimism

clean:
	cargo clean

fmt:
	cargo fmt

fmt-check:
	cargo fmt --all --check

clippy:
	cargo clippy --all --features eth -- -D warnings
	cargo clippy --all --features optimism -- -D warnings

taplo:
	taplo format

taplo-check:
	taplo format --check

deny-check:
	cargo deny --all-features check

.PHONY: pre-release
pre-release:
	make fmt
	make clippy
	make test
	make taplo-check
	make deny-check