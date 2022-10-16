#!/bin/bash

# Update sources
git stash
git switch development
git pull origin development

# Update Rust
rustup update

# Build new changes
# cargo build --release
date > build-date.txt

# Quit screen if any
if screen -list | grep -q "dfs-core"; then
    screen -S dfs-core -X quit
fi

export HOST=127.0.0.1
export PORT=3000
export ENV=dev
export SAVE_LOGS=false

# Start server on screen
screen -dmS dfs-core
screen -S dfs-core -X stuff "$HOME/dfs-core/bin/x86_64-unknown-linux-gnu/dfs\n"
