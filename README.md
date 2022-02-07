# Advent of Code 2021 (Rust)

[![codecov](https://codecov.io/gh/davidlag0/advent-of-code-2021/branch/main/graph/badge.svg?token=YBGR2fclvo)](https://codecov.io/gh/davidlag0/advent-of-code-2021)
![Rust CI](https://github.com/davidlag0/advent-of-code-2021/actions/workflows/rust.yml/badge.svg)

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
