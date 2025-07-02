# BUILDER
FROM rust:1.88-slim AS builder

WORKDIR /usr/src/hncli/

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y pkg-config libssl-dev

COPY . ./
RUN cargo build --release

ENTRYPOINT ["./target/release/hncli"]
