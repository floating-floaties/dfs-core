#!/bin/sh

set -e -o xtrace

cargo watch -x 'run --bin dfs'
