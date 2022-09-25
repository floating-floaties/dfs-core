#!/bin/bash

# Update sources
git stash
git switch development
git pull origin development

# Update Rust
rustup update

# Build new changes
cargo build --release
date > build-date.txt

# Quit screen if any
if screen -list | grep -q "dfs-core"; then
    screen -S dfs-core -X quit
fi

# Start server on screen
screen -dmS dfs-core
screen -S dfs-core -X stuff 'cargo run --release\n'
