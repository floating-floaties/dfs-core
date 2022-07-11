FROM node:16 as ui

WORKDIR /uiapp
RUN npm install --location=global expo-cli sharp-cli

COPY ./ui/yarn.lock .
COPY ./ui/package.json .

RUN yarn install --ignore-enigines

COPY ./ui .

RUN expo build:web

FROM rust:latest as api

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

COPY --from=ui /uiapp/web-build/. ./static/.

RUN sed -i -e 's/\/static\/js/\/static\/static\/js/g' ./static/index.html 
RUN sed -i -e 's/\.manifest.json/\\static\\\.manifest.json/g' ./static/index.html 
RUN sed -i -e 's/\\pwa/\\static\\pwa/g' ./static/index.html 
RUN sed -i -e 's/\\pwa/\\static\\\\pwa/g' ./static/manifest.json
RUN sed -i -e 's/\.\/fonts\//\.\/static\/fonts\//g' ./static/static/js/*.js

CMD ["cargo", "run", "--release"]
