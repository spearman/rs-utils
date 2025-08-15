#!/usr/bin/env bash

set -e
set -x

cargo clippy --all-features
cargo run --example example --features=env-logger-format

exit 0
