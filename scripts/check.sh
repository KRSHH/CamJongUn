#!/usr/bin/env sh
set -eu

cd "$(dirname "$0")/.."
cargo build
cargo test
cargo run -p camjongunctl -- doctor
