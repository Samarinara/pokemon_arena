# Step 1: Builder using a specific Alpine-based Rust image
FROM rust:1.88.0-alpine AS builder

# Install necessary build dependencies for musl target
RUN apk add --no-cache alpine-sdk openssl-dev cmake

# Add the musl target for Rust compilation
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/pokemon_arena

# Copy manifest files to leverage Docker cache for dependencies
COPY Cargo.toml Cargo.lock ./

# Build dependencies first to leverage Docker cache
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl

# Copy the rest of the source code
COPY src ./src

# Build the application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Step 2: Final image based on a specific Alpine version
FROM alpine:3.19

# Install runtime dependencies for OpenSSL
RUN apk add --no-cache libssl3

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/pokemon_arena/target/x86_64-unknown-linux-musl/release/pokemon_arena .

# Copy required assets
COPY src/pokemon ./src/pokemon
COPY users ./users

# Ensure the binary is executable
RUN chmod +x ./pokemon_arena

# Expose the correct port
EXPOSE 2222

# Set the entrypoint for the container
CMD ["./pokemon_arena"]
