# Use the official Rust image as a builder
FROM rust:latest AS builder

# Create a new empty shell project
WORKDIR /usr/src/pokemon_arena
COPY Cargo.toml Cargo.lock ./

# Copy your source code
COPY src ./src

# Build for release
RUN cargo build --release

# The final, smaller image
FROM debian:bookworm-slim

# Install OpenSSL 3 runtime library
RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/pokemon_arena/target/release/pokemon_arena .

# Copy the assets required by the application
COPY src/pokemon ./src/pokemon
COPY users ./users

# Set the command to run the application
CMD ["./pokemon_arena"]
