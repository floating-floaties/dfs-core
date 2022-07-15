#!/bin/sh

set -e -o xtrace
pwd

export HOST=127.0.0.1
export PORT=8080
export ENV=development
cargo watch -x 'run --bin dfs'
