#!/usr/bin/bash

# fail fast
#
set -e

# print each command before it's executed
#
set -x

wasm-pack test --headless --firefox --release
wasm-pack test --headless --firefox -- --all-features
