#!/bin/bash

sudo apt-get update

# Install generic tools
sudo apt-get install screen vim htop -y

# Install build tools
sudo apt install build-essential manpages-dev -y
sudo apt install gobjc gfortran gnat -y
sudo apt install pkg-config libssl-dev librust-openssl-dev librust-openssl-sys-dev -y

sudo apt-get install \
    apt-utils curl unzip libssl-dev git gcc \
    build-essential clang libclang-dev \
    libgomp1 manpages-dev clang cmake \
    gobjc gfortran gnat pkg-config \
    libssl-dev librust-openssl-dev \
    librust-openssl-sys-dev -y

curl https://download.pytorch.org/libtorch/cpu/libtorch-shared-with-deps-1.13.0%2Bcpu.zip -o libtorch.zip
unzip libtorch.zip
rm libtorch.zip

CURR_DIR="$(pwd)"
export LIBTORCH="$CURR_DIR/libtorch/"
export LD_LIBRARY_PATH="$LIBTORCH/lib:$LD_LIBRARY_PATH"

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup default nightly
rustup update

iptables -t nat -A PREROUTING -p tcp --dport 80 -j REDIRECT --to-port 8080

/bin/bash start.sh