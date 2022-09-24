FROM rust:1.64.0-slim-bullseye AS builder
WORKDIR /build
COPY action .
RUN cargo build
FROM debian:bullseye-slim AS runner
COPY --from=builder /build/target/debug/action /usr/local/bin/action
ENTRYPOINT /usr/local/bin/action
