.PHONY: build release install clean test fmt lint check

# Default target
all: build

# Build debug version
build:
	cargo build

# Build release version
release:
	cargo build --release

# Install locally (requires cargo install)
install:
	cargo install --path .

# Run tests
test:
	cargo test

# Run clippy lints
lint:
	cargo clippy -- -D warnings

# Format code
fmt:
	cargo fmt

# Check code without building
check:
	cargo check

# Clean build artifacts
clean:
	cargo clean
	rm -rf target/

# Run the CLI (for development)
run:
	cargo run --

# Build and run with arguments (usage: make run-args ARGS="pick -d easy")
run-args:
	cargo run -- $(ARGS)

# Create distribution package
dist: release
	mkdir -p dist
	cp target/release/leetcode-cli dist/
	cp README.md dist/
	tar -czvf leetcode-cli.tar.gz dist/

# Setup development environment
dev-setup:
	rustup component add rustfmt
	rustup component add clippy

# Full CI check
ci: fmt lint test release
	@echo "All CI checks passed!"
