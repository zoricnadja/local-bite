FROM rust AS builder

WORKDIR /app

COPY . .

ENV SQLX_OFFLINE=true
RUN cargo build --release -p auth

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/auth .

EXPOSE 8080

CMD ["./auth"]