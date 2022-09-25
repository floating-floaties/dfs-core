#!/bin/bash

sudo apt-get update

# Install generic tools
sudo apt-get install screen vim htop -y

# Install build tools
sudo apt install build-essential manpages-dev -y
sudo apt install gobjc gfortran gnat -y

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup default nightly
rustup update

/bin/bash start.sh