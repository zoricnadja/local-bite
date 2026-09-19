FROM rust AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY services services
COPY libs ./libs
COPY .sqlx .sqlx
ENV SQLX_OFFLINE=true
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked --mount=type=cache,target=/app/target,sharing=locked cargo build --locked --release --workspace && mkdir -p /out && cp target/release/auth target/release/raw-materials target/release/productions target/release/products target/release/orders target/release/read-models /out/
FROM debian:trixie-slim
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /out/read-models .
CMD ["./read-models"]
