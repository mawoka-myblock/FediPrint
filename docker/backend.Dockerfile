FROM rust:latest AS planner
WORKDIR /app
# We only pay the installation cost once,
# it will be cached from the second build onwards
# To ensure a reproducible build consider pinning
# the cargo-chef version with `--version X.X.X`
RUN cargo install cargo-chef
COPY fediprint/ .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust:latest AS cacher
WORKDIR app
RUN cargo install cargo-chef && \
    apt update; \
    apt install -y \
        build-essential clang mold
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:latest AS builder
WORKDIR /app

ENV USER=fediprint
ENV UID=1001
RUN adduser \
    --disabled-password \
    --gecos "" \
    #--home "/" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

RUN apt update; \
    apt install -y \
        build-essential clang mold

ENV SQLX_OFFLINE true
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo



COPY fediprint/ .
COPY migrations migrations
COPY fediprint/.sqlx .sqlx
COPY .git .git
RUN sed -i -e 's/\.\.\/\.\.\/migrations/\.\.\/migrations/g' app/src/main.rs
RUN cargo build --release --bin app

FROM debian:stable-slim

WORKDIR /app
RUN apt update && apt install -y openssl && rm -rf /var/lib/apt/lists/*

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

USER fediprint:fediprint

COPY --from=builder /app/target/release/app app

EXPOSE 8000

CMD ["/app/app"]
