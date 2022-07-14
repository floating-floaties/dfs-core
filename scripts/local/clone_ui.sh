#!/bin/sh

set -e -o xtrace

sudo npm install --location=global expo-cli sharp-cli yarn

PREV_LOC=$(pwd)
LOC=./ui

rm -rf $LOC

git clone git@github.com:floating-floaties/dfs-ui.git $LOC

cd $LOC

yarn install

cd $PREV_LOC
