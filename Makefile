SHELL := /bin/bash
.POSIX:
.PHONY: help init clean
.DEFAULT_GOAL := help

COVERAGE_RUST_VARIABLES = RUSTC_BOOTSTRAP=1 CARGO_INCREMENTAL=0 RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort" RUSTDOCFLAGS="-Cpanic=abort"
INSTRUMENT_COVERAGE_RUST_VARIABLES = RUSTC_BOOTSTRAP=1 RUSTFLAGS="-Zinstrument-coverage" LLVM_PROFILE_FILE="aoc_rust_2021-%p-%m.profraw"

help: ## Show this help
	@egrep -h '\s##\s' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

init: ## Initialize the development environment (setup Git hooks)
	@chmod u+x .githooks/pre-commit
	@chmod u+x .githooks/commit-msg
	@git config --local core.hooksPath .githooks/
	@cargo install cocogitto
	@echo "Environment is ready!"

clean: ## Clean development environment (remove profiling files and such)
	@rm -f aoc_rust_2021-*.profraw
	@rm -f aoc_rust_2021.profdata
	@rm -f coverage.html
	@cargo clean

coverage: clean ## Code coverage
	$(COVERAGE_RUST_VARIABLES) cargo build
	$(COVERAGE_RUST_VARIABLES) cargo test
	grcov . -s . -t html --branch --ignore-not-existing -o ./target/debug/coverage/

instrument-coverage: clean ## Rust "instrument-coverage" nightly feature
	$(INSTRUMENT_COVERAGE_RUST_VARIABLES) cargo build
	$(INSTRUMENT_COVERAGE_RUST_VARIABLES) cargo test
	grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
