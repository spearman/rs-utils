#!/usr/bin/env bash

RUSTDOCFLAGS="--cfg docsrs" cargo doc --all-features --no-deps

exit 0
