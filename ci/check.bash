#!/usr/bin/bash

# fail fast
#
set -e

# print each command before it's executed
#
set -x

cargo check --tests --examples --no-default-features
cargo check --tests --examples
cargo check --tests --examples --all-features
