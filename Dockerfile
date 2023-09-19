# Stage 1: Building
FROM rust:latest AS builder

WORKDIR /usr/src/chrona 
COPY . .

RUN cargo build --release

# Stage 2: Setup runtime environment
FROM debian:bookworm-slim

# Install libssl
RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/chrona

# Copy binary from builder stage
COPY --from=builder /usr/src/chrona/target/release/chrona ./chrona

CMD ["./chrona"]