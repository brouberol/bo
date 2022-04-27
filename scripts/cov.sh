#!/usr/bin/env bash

export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="bo-%p-%m.profraw"
rm -rf *.profraw target/debug/coverage
cargo build
cargo test
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage
open ./target/debug/coverage/index.html
rm -rf *.profraw
