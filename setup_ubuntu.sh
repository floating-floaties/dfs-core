#!/bin/bash

set -e -o xtrace

sudo apt-get update
sudo apt-get install vim screen htop -y

docker pull rust:latest
docker-compose up -d
