FROM rust:latest as BUILD

WORKDIR /app

COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=BUILD /app/target/release/coverage_scope /coverage_scope
COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
