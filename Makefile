# SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
#
# SPDX-License-Identifier: AGPL-3.0-or-later
define test_core
	cargo test --no-fail-fast
endef

default: ## Build library in debug mode
	cargo build

check: ## Check for syntax errors on all workspaces
	cargo check --workspace --tests --all-features

clean: ## Delete build artifacts
	@cargo clean

doc: ## Generate documentation
	cargo doc --no-deps --workspace --all-features

env: ## Setup development environtment
	cargo fetch

lint: ## Lint codebase
	cargo fmt -v --all -- --emit files
	cargo clippy --workspace --tests --all-features

release: ## Build library with release optimizations
	cargo build --release

test: ## Run all available tests
	$(call test_core)

test.cov.html: ## Generate code coverage report in HTML format
	cargo tarpaulin -t 1200 --out Html

test.cov.xml: ## Generate code coverage report in XML format
	cargo tarpaulin -t 1200 --out Xml

test.core: ## Run all core tests
	$(call test_core)

help: ## Prints help for targets with comments
	@cat $(MAKEFILE_LIST) | grep -E '^[a-zA-Z_-].+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
