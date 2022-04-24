.PHONY = test check lint run help
.DEFAULT_GOAL = help

test:  ## Run the unit tests
	RUST_BACKTRACE=1 cargo test

lint:  ## Check for clippy warnings
	cargo clippy --all-targets --all-features -- -D warnings

target/debug/bo:
	cargo build

target/release/bo:
	cargo build --release

run:  target/debug/bo ## Run bo in debug mode
	target/debug/bo

run-release:  target/release/bo  ## Run bo in release mode
	target/release/bo

check:  ## Check local package and dependencies
	cargo check

release:  ## Build a release binary
	cargo build --release

debug:  ## Build bo in debug mode
	cargo build

ci:  lint check test  ## Run all checks run by the CI

help:  ## Display help
	@grep -E '^[%a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?##"}; {printf "\033[36m%-22s\033[0m %s\n", $$1, $$2}'
