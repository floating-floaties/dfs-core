FROM rust:latest as build_api

RUN USER=root cargo new --bin /app
WORKDIR /app

RUN apt-get update
RUN apt-get install \
    apt-utils curl unzip libssl-dev git gcc \
    build-essential clang libclang-dev \
    libgomp1 manpages-dev clang cmake \
    gobjc gfortran gnat pkg-config \
    libssl-dev librust-openssl-dev \
    librust-openssl-sys-dev -y

RUN curl https://download.pytorch.org/libtorch/cpu/libtorch-shared-with-deps-1.13.0%2Bcpu.zip -o libtorch.zip
RUN unzip libtorch.zip
RUN rm libtorch.zip

ENV LIBTORCH=/app/libtorch/
ENV LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH

COPY Cargo.toml .
COPY Cargo.lock .
COPY dfs-ml .

RUN cargo build --release
RUN rm -rf ./src

COPY . .
RUN rm ./target/release/deps/dfs*
RUN cargo build --release

FROM tensorflow:latest
WORKDIR /app

ENV LIBTORCH=/app/libtorch/
ENV LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH

RUN date > build-date.txt

COPY --from=build_api /app/libtorch ./libtorch
COPY --from=build_api /app/target/release/dfs ./dfs
CMD ["./dfs"]
