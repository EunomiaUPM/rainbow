#!/bin/bash
# Brings up the mini stacks (published images) and waits until each agent answers.
# Usage: scripts/run/run-mini.sh [provider|consumer|all] [--down]
set -e

DEPLOYMENT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../deployment" && pwd)"

TARGET="all"
DOWN=false
for arg in "$@"; do
    case "$arg" in
        provider|consumer|all) TARGET="$arg" ;;
        --down|-d) DOWN=true ;;
        *) echo "Uso: $0 [provider|consumer|all] [--down]" >&2; exit 2 ;;
    esac
done

port_of() {
    case "$1" in
        provider) echo 1200 ;;
        consumer) echo 1100 ;;
    esac
}

targets() {
    if [ "$TARGET" = "all" ]; then echo "provider consumer"; else echo "$TARGET"; fi
}

compose_file() {
    echo "$DEPLOYMENT_DIR/docker-compose.mini.$1.yaml"
}

# Any answer below 500 counts as alive; gives up after $2 seconds (180 by default).
wait_http() {
    local url="$1" timeout="${2:-180}" deadline code
    deadline=$(( $(date +%s) + timeout ))
    while [ "$(date +%s)" -lt "$deadline" ]; do
        code=$(curl -s -o /dev/null -m 2 -w "%{http_code}" "$url" || true)
        if [ "$code" != "000" ] && [ "$code" -lt 500 ]; then
            return 0
        fi
        sleep 2
    done
    return 1
}

start_connector() {
    local name="$1" port
    echo -e "\033[0;36mLevantando $name (mini)...\033[0m"
    if ! docker compose -f "$(compose_file "$name")" up -d; then
        echo -e "\033[0;31mFallo al levantar $name. Logs del setup: docker logs $name-setup\033[0m"
        exit 1
    fi

    port=$(port_of "$name")
    echo -e "\033[0;36mEsperando a que $name responda en el puerto $port...\033[0m"
    if ! wait_http "http://localhost:$port/.well-known/dspace-version"; then
        echo -e "\033[0;31m$name no responde. Logs: docker logs $name\033[0m"
        exit 1
    fi
    echo -e "\033[0;32m$name listo: http://localhost:$port/admin/\033[0m"
}

stop_connector() {
    local name="$1"
    echo -e "\033[0;36mBajando $name...\033[0m"
    if ! docker compose -f "$(compose_file "$name")" down; then
        echo -e "\033[0;31mFallo al bajar $name\033[0m"
        exit 1
    fi
}

for name in $(targets); do
    if [ "$DOWN" = true ]; then stop_connector "$name"; else start_connector "$name"; fi
done
