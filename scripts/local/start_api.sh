#!/bin/sh

set -e -o xtrace
pwd

export HOST=127.0.0.1
export PORT=3000
export ENV=dev
cargo watch -x 'run --bin dfs'
