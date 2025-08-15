#!/usr/bin/env bash

set -e
set -x

cargo run --example example --features=env-logger-format

exit 0
