#!/bin/sh

set -e -o xtrace
pwd

export HOST=127.0.0.1
export PORT=3000
export ENV=dev
export SAVE_LOGS=false
cargo watch -x 'test && cargo run --bin dfs'
