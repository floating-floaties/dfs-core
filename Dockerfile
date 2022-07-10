FROM rust:latest

COPY cargo.toml .
COPY cargo.lock .

RUN cargo build --release

COPY . .

RUN cargo build --release
RUN cargo test --release

CMD ["cargo", "run", "--release"]
