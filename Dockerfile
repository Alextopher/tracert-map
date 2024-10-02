# Builder
FROM rust:latest AS builder
RUN apt update && apt upgrade -y && \
    apt install -y musl-tools musl-dev libssl-dev pkg-config

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src
RUN echo "fn main() {panic!(\"if you see this, the build is broken\")}" > src/main.rs

RUN cargo build --release --target x86_64-unknown-linux-musl

COPY src ./src
COPY pages ./pages

RUN rm target/x86_64-unknown-linux-musl/release/deps/tracert_map*
RUN cargo build --release --target x86_64-unknown-linux-musl

# FINAL
FROM alpine:latest

WORKDIR /app

RUN apk add --no-cache bash

COPY static ./static
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/tracert-map .

CMD ["./tracert-map", "-c", "config.toml", "-p", "80"]