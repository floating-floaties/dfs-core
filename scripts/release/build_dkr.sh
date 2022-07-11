#!/bin/sh

set -e -o xtrace

export PORT=80

docker build -t dfs .
docker run --rm -it -p 80:80 dfs
