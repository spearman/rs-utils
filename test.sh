#!/usr/bin/env bash

set -e
set -x

cargo clippy --all-targets --all-features
cargo test --all-targets --all-features

exit 0
