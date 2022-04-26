#!/usr/bin/env bash

rm -rf *.profraw target/debug/coverage
RUSTFLAGS="-Cinstrument-coverage" cargo build
LLVM_PROFILE_FILE="bo-%p-%m.profraw" cargo test
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage
open ./target/debug/coverage/index.html
rm -rf *.profraw
