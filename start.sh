#!/bin/bash

# Update sources
git stash
git switch development
git pull origin development

# Update Rust
#rustup update

# Build new changes
# cargo build --release
date > build-date.txt

# Quit screen if any
if screen -list | grep -q "dfs-core"; then
    screen -S dfs-core -X quit
fi

export LIBTORCH="$HOME/libtorch/"
export LD_LIBRARY_PATH="$LIBTORCH/lib:$LD_LIBRARY_PATH"

for i in $(lsof -t -i tcp:"8080"); do kill -9 "$i"; done
for i in $(lsof -t -i tcp:"80"); do kill -9 "$i"; done

sudo iptables -t nat -A PREROUTING -p tcp --dport 80 -j REDIRECT --to-port 8080

# Start server on screen
screen -dmS dfs-core
screen -S dfs-core -X stuff "export LIBTORCH=\"$LIBTORCH\" \
                             && export LD_LIBRARY_PATH=\"$LD_LIBRARY_PATH\" \
                             && export CONFIG_CONCORD_URL=\"$CONFIG_CONCORD_URL\" \
                             && export CONFIG_CONCORD_API_KEY=\"$CONFIG_CONCORD_API_KEY\" \
                             && export CONFIG_CONCORD_APP_NAME=\"$CONFIG_CONCORD_APP_NAME\" \
                             && export CONFIG_CONCORD_EMAIL=\"$CONFIG_CONCORD_EMAIL\" \
                             && export HOST=\"0.0.0.0\" \
                             && export PORT=\"8080\" \
                             && export ENV=\"production\" \
                             && export SAVE_LOGS=\"true\" \
                             && ./bin/x86_64-unknown-linux-gnu/dfs\n"
