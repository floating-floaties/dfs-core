ARG RUST_VERSION=latest

FROM rust:$RUST_VERSION as build_api

RUN USER=root cargo new --bin /app
WORKDIR /app

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
WORKDIR /app
RUN date > build-date.txt
COPY --from=build_api /app/target/release/dfs ./dfs
CMD ["./dfs"]
