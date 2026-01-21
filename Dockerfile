FROM rust:1.89-bookworm AS builder

RUN apt-get update && \
    apt-get install -y musl-tools pkg-config libssl-dev clang

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl
