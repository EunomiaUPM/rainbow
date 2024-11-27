FROM rust:bullseye AS builder

RUN apt-get update && \
    apt-get install -y \
        pkg-config \
        libssl-dev

WORKDIR /app
COPY ./. .
RUN cargo build --release

FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y \
        libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

# Files
WORKDIR /app
COPY --from=builder /app/target/release/push-receiving-server .

# Envs
EXPOSE 1237

# Entrypoint
CMD ["./push-receiving-server"]