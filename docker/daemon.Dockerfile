FROM rust:latest AS chef
WORKDIR /app
RUN cargo install cargo-chef

FROM chef AS planner
WORKDIR /app
COPY Cargo.lock ./
COPY Cargo-docker.toml Cargo.toml
COPY daemon daemon
COPY core core
COPY cli cli
COPY gui gui
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
COPY Cargo.lock ./
COPY Cargo-docker.toml Cargo.toml
COPY daemon daemon
COPY core core
COPY cli cli
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release -p griffon_daemon

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/griffon_daemon /usr/local/bin/griffon-daemon

CMD ["griffon-daemon"]
