.PHONY: build test clean all run docs

all: clean build test docs

build:
	@echo "👷🏽‍♀️building rust project..."
	cargo build

test:
	@echo "👷🏽‍♀️running tests..."
	RUST_LOG=debug cargo test

clean:
	@echo "👷🏽‍♀️cleaning up..."
	cargo clean

run:
	@echo "👷🏽‍♀️running rust project..."
	RUST_LOG=info cargo run

docs:
	@echo "👷🏽‍♀️building project documentation..."
	cargo doc --no-deps
	open target/doc/riot_rso_sample_app/index.html

debug:
	@echo "👷🏽‍♀️debugging rust project..."
	RUST_BACKTRACE=1 RUST_LOG=debug cargo run
