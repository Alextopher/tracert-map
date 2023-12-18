# Plan using cargo-chef
FROM rust:1.74 as planner
WORKDIR /app
RUN cargo install cargo-chef 
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

# Cache dependencies
FROM rust:1.74 as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build
FROM rust:1.74 as builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
RUN cargo build --release

# Runtime
FROM rust:1.74 as runtime
WORKDIR /app

COPY --from=builder /app/target/release/tracert-map .
COPY config.toml .
COPY static ./static

CMD ["./tracert-map", "-c", "config.toml", "-p", "80"]