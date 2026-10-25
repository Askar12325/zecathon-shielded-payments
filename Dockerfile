FROM rust:1.80-slim as builder

WORKDIR /app

# Install build dependencies for C crates (secp256k1, etc.)
RUN apt-get update && apt-get install -y build-essential pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock* ./
COPY src ./src
COPY tests ./tests

RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zecpay /app/zecpay
COPY frontend ./frontend

ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=3000
EXPOSE 3000

CMD ["/app/zecpay"]
