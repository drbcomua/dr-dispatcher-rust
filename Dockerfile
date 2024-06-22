# Build stage
FROM rust:1.67 as builder

WORKDIR /app

# Copy the entire project
COPY . .

# Build the project in release mode
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Install necessary dependencies for the Rust binary
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/dispatcher-rust /usr/local/bin/dispatcher-rust

# Expose the port your application runs on
EXPOSE 8080

# Run the binary
CMD ["dispatcher-rust"]
