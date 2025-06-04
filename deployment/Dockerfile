# BUILD STAGE
FROM rust:1.83-bullseye AS builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y \
        pkg-config \
        libssl-dev \
        libpq-dev

# Files
WORKDIR /app
COPY ./. .



# Build the application
RUN cd ./rainbow-core && cargo build --release

# IMAGE STAGE
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y \
        libssl1.1 \
        libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Files
WORKDIR /app
COPY --from=builder /app/target/release/rainbow .

# Envs
ENV LD_LIBRARY_PATH="/usr/lib"
EXPOSE 1234

# Entrypoint
ENTRYPOINT ["./rainbow"]

