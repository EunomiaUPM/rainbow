#!/bin/bash
set -euo pipefail

# ----------------------------
# Base directory (one level up from scripts)
# ----------------------------
BASE_DIR="$(cd "$(dirname "$0")" && pwd)/../.."

# ----------------------------
# Global configuration
# ----------------------------
MODULE=${1:-core}  # default to 'core' if no module is passed
DOCKER_COMPOSE_PATH="$BASE_DIR/deployment/docker-compose.core.yaml"

echo "=== Starting Rainbow environment..."

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
echo "Starting databases with Docker Compose..."
docker-compose -f "$DOCKER_COMPOSE_PATH" up -d
sleep 5  # wait for DBs to be ready

# ----------------------------
# Helper function to open a new cmd window
# ----------------------------
start_cmd() {
    local title="$1"
    local command="$2"
    echo "Starting $title..."
    cmd.exe /C start "Rainbow $title" powershell -NoExit -Command "cd '$BASE_DIR'; $command"
}

# ----------------------------
# Start Authority
# ----------------------------
mintty -t "Rainbow Authority" /bin/bash -c "cd '$BASE_DIR/rainbow-authority' && cargo run --manifest-path Cargo.toml start --env-file ../static/envs/.env.authority; exec bash" &

# ----------------------------
# Start Consumer
# ----------------------------
if [[ "$MODULE" == "core" ]]; then
    mintty -t "Rainbow Consumer" /bin/bash -c "cd '$BASE_DIR/rainbow-core' && cargo run --manifest-path Cargo.toml consumer start --env-file ../static/envs/.env.consumer.core; exec bash" &
else
    mintty -t "Rainbow Consumer" /bin/bash -c "cd '$BASE_DIR/rainbow-$MODULE' && cargo run --manifest-path Cargo.toml consumer start --env-file ../static/envs/.env.consumer.core; exec bash" &
fi

# ----------------------------
# Start Provider
# ----------------------------
if [[ "$MODULE" == "core" ]]; then
    mintty -t "Rainbow Provider" /bin/bash -c "cd '$BASE_DIR/rainbow-core' && cargo run --manifest-path Cargo.toml provider start --env-file ../static/envs/.env.provider.core; exec bash" &
else
    mintty -t "Rainbow Provider" /bin/bash -c "cd '$BASE_DIR/rainbow-$MODULE' && cargo run --manifest-path Cargo.toml provider start --env-file ../static/envs/.env.provider.core; exec bash" &
fi

echo ""
echo "Rainbow services started successfully for module '$MODULE'"
