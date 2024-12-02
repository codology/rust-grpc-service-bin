# Build Rust project
FROM rust:1.70 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Create runtime image
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/grpc-service .
CMD ["./grpc-service"]
