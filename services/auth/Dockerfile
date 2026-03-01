# Build stage — musl target for fully static binary (no glibc dependency)
FROM rust:1.75-slim AS builder

RUN apt-get update && apt-get install -y \
    musl-tools pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY libs/ libs/
COPY services/auth/ services/auth/

RUN for svc in raw-materials production products orders; do \
    mkdir -p services/$svc/src && \
    printf '[package]\nname="%s"\nversion="0.1.0"\nedition="2021"\n[dependencies]' $svc \
        > services/$svc/Cargo.toml && \
    echo 'fn main(){}' > services/$svc/src/main.rs; \
done

RUN cargo build --release --target x86_64-unknown-linux-musl --bin auth

# Runtime stage — scratch has nothing, binary is fully self-contained
FROM alpine:3.19
RUN apk add --no-cache ca-certificates tzdata
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/auth ./auth
COPY services/auth/migrations/ ./migrations/
CMD ["./auth"]
