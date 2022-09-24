# Builder step
FROM rust:1.64.0-slim-bullseye AS builder
WORKDIR /build
RUN apt-get install --no-install-recommends -y pkg-config libssl-dev openssl
COPY action .
RUN cargo build

# Run step
FROM debian:bullseye-slim AS runner
COPY --from=builder /build/target/debug/action /usr/local/bin/action
ENTRYPOINT /usr/local/bin/action
