.PHONY = test check lint run help
.DEFAULT_GOAL = help

test:  ## Run the unit tests
	cargo test

lint:  ## Check for clippy warnings
	cargo clippy --all-targets --all-features -- -D warnings

run:  ## Run bo
	cargo run

check:  ## Check local package and dependencies
	cargo check

release:  ## Build a release binary
	cargo build --release

help:  ## Display help
	@grep -E '^[%a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?##"}; {printf "\033[36m%-22s\033[0m %s\n", $$1, $$2}'
