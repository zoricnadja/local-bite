FROM rust AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY services services
COPY libs ./libs
COPY .sqlx .sqlx

ENV SQLX_OFFLINE=true
RUN cargo build --release -p raw-materials

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/raw-materials .

EXPOSE 8080

CMD ["./raw-materials"]
