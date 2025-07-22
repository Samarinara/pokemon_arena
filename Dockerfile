# Step 1: Build with musl for Alpine compatibility
FROM rust:latest AS builder

# Install musl tools and OpenSSL for musl
RUN rustup target add x86_64-unknown-linux-musl

RUN apk add --no-cache musl-dev openssl-dev

WORKDIR /usr/src/pokemon_arena
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build statically for musl
RUN cargo build --release --target x86_64-unknown-linux-musl

# Step 2: Use minimal Alpine image for runtime
FROM alpine:latest

# Add runtime OpenSSL (Alpine provides libssl3)
RUN apk add --no-cache libssl3

# Copy the statically built binary
COPY --from=builder /usr/src/pokemon_arena/target/x86_64-unknown-linux-musl/release/pokemon_arena .

# Copy required assets
COPY src/pokemon ./src/pokemon
COPY users ./users

# Run it
CMD ["./pokemon_arena"]
