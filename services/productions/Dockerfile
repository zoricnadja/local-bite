FROM rust AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY services services
COPY libs ./libs
COPY .sqlx .sqlx

ENV SQLX_OFFLINE=true
RUN cargo build --release -p productions

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/productions .

EXPOSE 8080

# runtime
CMD ["./productions"]