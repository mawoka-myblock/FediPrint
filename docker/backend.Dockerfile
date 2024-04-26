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

COPY ./ .
RUN cargo build --release

FROM debian:stable-slim

WORKDIR /app

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

USER fediprint:fediprint

COPY --from=builder /app/target/release/fedi_print fedi_print

ENTRYPOINT ["/app/fedi_print"]