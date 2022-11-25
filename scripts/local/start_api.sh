#!/bin/sh

export HOST=127.0.0.1
export PORT=3000
export ENV=dev
export SAVE_LOGS=false

kill -9 "$(lsof -t -i tcp:"$PORT")"

cargo watch -x 'run --release --bin dfs'
