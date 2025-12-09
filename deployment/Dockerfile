# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.88.0
ARG APP_NAME=rainbow_transfer

# --- builder ---
# Usa una imagen base Rust optimizada para compilación.
FROM rust:${RUST_VERSION}-slim-bookworm AS builder
ARG APP_NAME

WORKDIR /app

# dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev curl protobuf-compiler && \
    rm -rf /var/lib/apt/lists/*


# app building
# https://docs.docker.com/guides/rust/develop/
RUN --mount=type=bind,source=./../Cargo.toml,target=/app/Cargo.toml \
    --mount=type=bind,source=./../Cargo.lock,target=/app/Cargo.lock \
    --mount=type=bind,source=./../rainbow-core,target=/app/rainbow-core \
    --mount=type=bind,source=./../rainbow-catalog,target=/app/rainbow-catalog \
    --mount=type=bind,source=./../rainbow-contracts,target=/app/rainbow-contracts \
    --mount=type=bind,source=./../rainbow-dataplane,target=/app/rainbow-dataplane \
    --mount=type=bind,source=./../rainbow-transfer,target=/app/rainbow-transfer \
    --mount=type=bind,source=./../rainbow-transfer-agent,target=/app/rainbow-transfer-agent \
    --mount=type=bind,source=./../rainbow-common,target=/app/rainbow-common \
    --mount=type=bind,source=./../rainbow-auth,target=/app/rainbow-auth \
    --mount=type=bind,source=./../rainbow-db,target=/app/rainbow-db \
    --mount=type=bind,source=./../rainbow-events,target=/app/rainbow-events \
    --mount=type=bind,source=./../rainbow-datahub-catalog,target=/app/rainbow-datahub-catalog \
    --mount=type=bind,source=./../rainbow-fe-gateway,target=/app/rainbow-fe-gateway \
    --mount=type=bind,source=./../rainbow-business-gateway,target=/app/rainbow-business-gateway \
    --mount=type=bind,source=./../rainbow-authority,target=/app/rainbow-authority \
    --mount=type=bind,source=./../static,target=/app/static \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    /bin/sh -c "\
    set -e; \
    \
    \
    \
    cargo build --locked --release -p ${APP_NAME}; \
    \
    cp ./target/release/${APP_NAME} /bin/${APP_NAME}; \
"

# --- image ---
FROM debian:bookworm-slim AS final
ARG APP_NAME
ENV APP_NAME=${APP_NAME}


# update on image
RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# not privileged user
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

# binary copy
COPY --from=builder /bin/${APP_NAME} /usr/local/bin/

# user permission
RUN chmod +x /usr/local/bin/${APP_NAME}
USER appuser

# entrypoint
ENTRYPOINT ["/usr/local/bin/${APP_NAME}"]