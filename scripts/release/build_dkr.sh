#!/bin/sh

set -e -o xtrace

export PORT=80

docker build -t dfs .
docker run -e PORT=$PORT --rm -it -p $PORT:$PORT dfs
