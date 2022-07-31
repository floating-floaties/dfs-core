ARG RUST_VERSION=latest

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
RUN cargo build --release

FROM rust:$RUST_VERSION
COPY --from=build_api /dfs/target/release/dfs ./dfs
CMD ["./dfs"]
