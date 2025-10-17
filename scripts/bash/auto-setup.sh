#!/bin/bash
set -euo pipefail

# ----------------------------
# Base directory (two levels up from scripts/bash)
# ----------------------------
BASE_DIR="$(cd "$(dirname "$0")" && pwd)/../.."

# ----------------------------
# Global configuration
# ----------------------------
MODULE=${1:-core}  # Default to 'core' if no module is passed
DOCKER_COMPOSE_PATH="$BASE_DIR/deployment/docker-compose.core.yaml"

echo "=== Starting Rainbow environment setup..."

# ----------------------------
# Valid modules
# ----------------------------
VALID_MODULES=("core" "catalog" "contracts" "transfer" "auth")

# Check if module is valid
# shellcheck disable=SC2076
if [[ ! " ${VALID_MODULES[*]} " =~ " ${MODULE} " ]]; then
    echo "ERROR: Invalid module '$MODULE'. Valid options: ${VALID_MODULES[*]}"
    exit 1
fi

# ----------------------------
# Start databases
# ----------------------------
echo "Restarting databases with Docker Compose..."
docker-compose -f "$DOCKER_COMPOSE_PATH" down -v
docker-compose -f "$DOCKER_COMPOSE_PATH" up -d
echo "Waiting for databases to be ready..."
sleep 5

# ----------------------------
# Authority setup
# ----------------------------
echo "Running setup for Authority..."
cargo run --manifest-path "$BASE_DIR/rainbow-authority/Cargo.toml" setup \
  --env-file "$BASE_DIR/static/envs/.env.authority"

# ----------------------------
# Consumer setup
# ----------------------------
if [[ "$MODULE" == "core" ]]; then
  echo "Running setup for Consumer (all modules)..."
  cargo run --manifest-path "$BASE_DIR/rainbow-core/Cargo.toml" consumer setup \
    --env-file "$BASE_DIR/static/envs/.env.consumer.core"
else
  echo "Running setup for Consumer module: $MODULE..."
  cargo run --manifest-path "$BASE_DIR/rainbow-$MODULE/Cargo.toml" consumer setup \
    --env-file "$BASE_DIR/static/envs/.env.consumer.core"
fi

# ----------------------------
# Provider setup
# ----------------------------
if [[ "$MODULE" == "core" ]]; then
  echo "Running setup for Provider (all modules)..."
  cargo run --manifest-path "$BASE_DIR/rainbow-core/Cargo.toml" provider setup \
    --env-file "$BASE_DIR/static/envs/.env.provider.core"
else
  echo "Running setup for Provider module: $MODULE..."
  cargo run --manifest-path "$BASE_DIR/rainbow-$MODULE/Cargo.toml" provider setup \
    --env-file "$BASE_DIR/static/envs/.env.provider.core"
fi

echo ""
echo "Setup completed successfully for module '$MODULE'"
