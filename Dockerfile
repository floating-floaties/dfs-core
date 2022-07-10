FROM rust:latest

RUN cargo new --bin /app

WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo build --release

COPY ./src ./src
COPY ./tests ./tests
COPY ./examples ./examples

RUN cargo build --release
RUN cargo test --release

CMD ["cargo", "run", "--release"]
