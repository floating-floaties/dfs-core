ARG NODE_VERSION=16
ARG RUST_VERSION=latest

FROM node:$NODE_VERSION as build_ui

WORKDIR /uiapp
RUN npm install --location=global expo-cli sharp-cli

COPY ./ui/yarn.lock .
COPY ./ui/package.json .

RUN yarn install --ignore-enigines

COPY ./ui .

RUN expo build:web

# 
FROM rust:$RUST_VERSION as build_api

RUN USER=root cargo new --bin /dfs
WORKDIR /dfs

COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo build --release
RUN rm -rf ./src

COPY ./src ./src
COPY ./tests ./tests
COPY ./examples ./examples

RUN rm ./target/release/deps/dfs*
RUN cargo build --target="dfs" --release

# 
FROM rust:$RUST_VERSION

COPY --from=build_api /dfs/target/release/deps/dfs* ./
COPY --from=build_ui /uiapp/web-build/. ./static/.

RUN sed -i -e 's/\/static\/js/\/static\/static\/js/g' ./static/index.html 
RUN sed -i -e 's/\.manifest.json/\\static\\\.manifest.json/g' ./static/index.html 
RUN sed -i -e 's/\\pwa/\\static\\pwa/g' ./static/index.html 
RUN sed -i -e 's/\\pwa/\\static\\\\pwa/g' ./static/manifest.json
RUN sed -i -e 's/\.\/fonts\//\.\/static\/fonts\//g' ./static/static/js/*.js

CMD ["./dfs"]
