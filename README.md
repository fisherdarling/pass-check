# pass-check

A tool for collecting optimization statistics.

## Usage

To run it on the project's bitcode:

`RUSTFLAGS='--emit llvm-bc' cargo run --release -- target/debug/deps`