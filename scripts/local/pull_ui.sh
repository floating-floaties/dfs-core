#!/bin/sh

set -e -o xtrace

rm -rf ui/

git clone git@github.com:floating-floaties/dfs-ui.git ./ui
