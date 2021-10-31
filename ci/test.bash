#!/usr/bin/bash

# fail fast
#
set -e

# print each command before it's executed
#
set -x

cargo          test --all-features
cargo +nightly test --all-features
