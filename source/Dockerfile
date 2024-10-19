# syntax=docker/dockerfile:1
FROM rust:1.81.0-alpine AS chef

WORKDIR /usr/src/project

RUN set -eux
RUN apk add --no-cache musl-dev pkgconfig openssl-dev openssl-libs-static
RUN cargo install cargo-chef
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown
RUN rm -rf $CARGO_HOME/registry

FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /usr/src/project/recipe.json .
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

WORKDIR /usr/src/project/yew
RUN trunk build --release

FROM alpine:3.20

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/project/target/release/frand-home-actix .
COPY --from=builder /usr/src/project/target/dist ./target/dist