#!/bin/sh

export HOST=0.0.0.0
export PORT=80
export ENV=dev
export SAVE_LOGS=false

#for i in $(lsof -t -i tcp:"$PORT"); do kill -9 "$i"; done
#for i in $(lsof -t -i tcp:"$PORT"); do kill -9 "$i"; done

authbind --deep cargo watch -x 'run --release --bin dfs'
