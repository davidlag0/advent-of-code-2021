# Advent of Code 2021 (Rust)

[![codecov](https://codecov.io/gh/davidlag0/advent-of-code-2021/branch/main/graph/badge.svg?token=YBGR2fclvo)](https://codecov.io/gh/davidlag0/advent-of-code-2021)

This year (2021), I'm learning Rust! As I go through Advent of Code, I will attempt the challenges on my own and then improve it by looking at other people's code.

## Usage

### Solve puzzles
```sh
$ cargo run <path to folder with input files>
```

### Run tests in current environment
```sh
$ cargo test -- --nocapture
```

## Development

### Prepare Environment
```sh
$ make init
```

### Code coverage (Reference: https://github.com/mozilla/grcov)
```sh
$ make coverage
```

### Code Coverage (Method #1)
* `cargo clean`
* `RUSTFLAGS="-Z instrument-coverage" LLVM_PROFILE_FILE="aoc_rust_2021-%m.profraw" cargo test --tests`
* `cargo profdata -- merge -sparse aoc_rust_2021-*.profraw -o aoc_rust_2021.profdata`
* `cargo cov -- report \
    --use-color --instr-profile=aoc_rust_2021.profdata \
    --object target/debug/deps/<binary name>`

### Code Coverage (Method #2)
Setup: https://github.com/mozilla/grcov

* `cargo test`
* `grcov . -s . --binary-path ./target/debug/ -t lcov --branch --ignore-not-existing -o ./target/debug/lcov.info`
* `genhtml -o ./target/debug/coverage/ --show-details --highlight --ignore-errors source --legend ./target/debug/lcov.info`
