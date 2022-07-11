#!/bin.sh

set -e -o xtrace

cargo install cargo-watch
npm install --location=global expo-cli sharp-cli yarn

cargo build

cd ui

yarn install

cd ..
