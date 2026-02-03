#!/bin/bash
# ============================================================================
# Telepresence Intercept Helper for Rainbow Dev
# ============================================================================
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Available namespaces
NAMESPACES=("rainbow-provider" "rainbow-consumer" "rainbow-infra")

# ============================================================================
# Help
# ============================================================================
show_help() {
    echo ""
    echo "Usage: $0 <command> [options]"
    echo ""
    echo "Commands:"
    echo "  connect              Connect telepresence to the cluster"
    echo "  status               Show telepresence status"
    echo "  list                 List available services in all namespaces"
    echo "  intercept <service>  Intercept a service"
    echo "  leave                Stop all intercepts and disconnect"
    echo "  env <namespace>      Print env vars to connect to a namespace's services"
    echo ""
    echo "Intercept Options:"
    echo "  -n, --namespace NS   Namespace (provider, consumer, infra)"
    echo "  -p, --port PORT      Local port to forward (default: 8080)"
    echo "  -e, --env FILE       File to write environment variables"
    echo ""
    echo "Examples:"
    echo "  $0 connect"
    echo "  $0 list"
    echo "  $0 intercept heimdall -n infra -p 1500"
    echo "  $0 env provider"
    echo "  $0 leave"
    echo ""
}

# ============================================================================
# Commands
# ============================================================================
cmd_connect() {
    log_info "Connecting to cluster..."
    
    telepresence connect
    
    log_success "Connected! Services are now accessible via Kubernetes DNS."
    echo ""
    echo "=============================================="
    echo "  Available Service URLs"
    echo "=============================================="
    echo ""
    echo "Provider (rainbow-provider):"
    echo "  - postgres:  provider-postgres.rainbow-provider:5432"
    echo "  - redis:     provider-redis.rainbow-provider:6379"
    echo "  - vault:     http://provider-vault.rainbow-provider:8200"
    echo "  - keycloak:  http://provider-keycloak.rainbow-provider:8080"
    echo ""
    echo "Consumer (rainbow-consumer):"
    echo "  - postgres:  consumer-postgres.rainbow-consumer:5432"
    echo "  - redis:     consumer-redis.rainbow-consumer:6379"
    echo "  - vault:     http://consumer-vault.rainbow-consumer:8200"
    echo "  - keycloak:  http://consumer-keycloak.rainbow-consumer:8080"
    echo ""
    echo "Infra (rainbow-infra):"
    echo "  - postgres:  authority-postgres.rainbow-infra:5432"
    echo "  - vault:     http://authority-vault.rainbow-infra:8200"
    echo "  - heimdall:  http://heimdall.rainbow-infra:1500"
    echo ""
}

cmd_status() {
    telepresence status
}

cmd_list() {
    log_info "Available services in all namespaces:"
    echo ""
    
    for ns in "${NAMESPACES[@]}"; do
        echo "=== $ns ==="
        kubectl get svc -n "$ns" -o wide 2>/dev/null || echo "  (namespace not found)"
        echo ""
    done
    
    log_info "To intercept a service, run:"
    echo "  $0 intercept <service-name> -n <namespace> -p <local-port>"
    echo ""
}

get_full_namespace() {
    local short=$1
    case $short in
        provider|rainbow-provider)
            echo "rainbow-provider"
            ;;
        consumer|rainbow-consumer)
            echo "rainbow-consumer"
            ;;
        infra|rainbow-infra)
            echo "rainbow-infra"
            ;;
        *)
            echo "$short"
            ;;
    esac
}

cmd_intercept() {
    local service=$1
    shift
    
    if [ -z "$service" ]; then
        log_error "Service name required"
        show_help
        exit 1
    fi
    
    # Parse options
    local namespace="rainbow-provider"
    local port="8080"
    local env_file=""
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            -n|--namespace)
                namespace=$(get_full_namespace "$2")
                shift 2
                ;;
            -p|--port)
                port="$2"
                shift 2
                ;;
            -e|--env)
                env_file="$2"
                shift 2
                ;;
            *)
                log_error "Unknown option: $1"
                exit 1
                ;;
        esac
    done
    
    log_info "Intercepting service '$service' in namespace '$namespace' on port $port..."
    
    local cmd="telepresence intercept $service -n $namespace --port $port"
    
    if [ -n "$env_file" ]; then
        cmd="$cmd --env-file $env_file"
        log_info "Environment variables will be saved to: $env_file"
    fi
    
    eval "$cmd"
    
    log_success "Intercept active!"
    echo ""
    echo "Your local application on port $port now receives traffic from $service."
    echo ""
    echo "To stop this intercept:"
    echo "  telepresence leave $service-$namespace"
    echo ""
}

cmd_env() {
    local role=$1
    
    if [ -z "$role" ]; then
        log_error "Namespace role required (provider, consumer, or infra)"
        exit 1
    fi
    
    local namespace=$(get_full_namespace "$role")
    
    echo "# Environment variables for $namespace"
    echo "# Copy these to your .env or export them"
    echo ""
    
    case $role in
        provider|rainbow-provider)
            cat <<EOF
# Database
DATABASE_HOST=provider-postgres.rainbow-provider
DATABASE_PORT=5432
DATABASE_USER=ds_provider
DATABASE_PASSWORD=ds_provider
DATABASE_NAME=ds_provider

# Redis
REDIS_HOST=provider-redis.rainbow-provider
REDIS_PORT=6379
REDIS_PASSWORD=ds_core_provider_redis

# Vault
VAULT_ADDR=http://provider-vault.rainbow-provider:8200

# Keycloak
KEYCLOAK_URL=http://provider-keycloak.rainbow-provider:8080

# Heimdall (from infra)
HEIMDALL_URL=http://heimdall.rainbow-infra:1500
EOF
            ;;
        consumer|rainbow-consumer)
            cat <<EOF
# Database
DATABASE_HOST=consumer-postgres.rainbow-consumer
DATABASE_PORT=5432
DATABASE_USER=ds_consumer
DATABASE_PASSWORD=ds_consumer
DATABASE_NAME=ds_consumer

# Redis
REDIS_HOST=consumer-redis.rainbow-consumer
REDIS_PORT=6379
REDIS_PASSWORD=ds_core_consumer_redis

# Vault
VAULT_ADDR=http://consumer-vault.rainbow-consumer:8200

# Keycloak
KEYCLOAK_URL=http://consumer-keycloak.rainbow-consumer:8080

# Heimdall (from infra)
HEIMDALL_URL=http://heimdall.rainbow-infra:1500
EOF
            ;;
        infra|rainbow-infra)
            cat <<EOF
# Database
DATABASE_HOST=authority-postgres.rainbow-infra
DATABASE_PORT=5432
DATABASE_USER=ds_authority
DATABASE_PASSWORD=ds_authority
DATABASE_NAME=ds_authority

# Vault
VAULT_ADDR=http://authority-vault.rainbow-infra:8200
EOF
            ;;
        *)
            log_error "Unknown namespace: $role. Use provider, consumer, or infra."
            exit 1
            ;;
    esac
    echo ""
}

cmd_leave() {
    log_info "Stopping all intercepts and disconnecting..."
    
    # Leave all intercepts
    telepresence leave --all 2>/dev/null || true
    
    # Quit telepresence
    telepresence quit
    
    log_success "Disconnected from cluster"
}

# ============================================================================
# Main
# ============================================================================
main() {
    # Check if telepresence is installed
    if ! command -v telepresence &> /dev/null; then
        log_error "Telepresence not found. Install it with:"
        echo ""
        echo "  brew install telepresence"
        echo ""
        echo "  Or from: https://www.telepresence.io/docs/install"
        echo ""
        exit 1
    fi
    
    local command=${1:-help}
    shift 2>/dev/null || true
    
    case $command in
        connect)
            cmd_connect
            ;;
        status)
            cmd_status
            ;;
        list)
            cmd_list
            ;;
        intercept)
            cmd_intercept "$@"
            ;;
        env)
            cmd_env "$@"
            ;;
        leave)
            cmd_leave
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            log_error "Unknown command: $command"
            show_help
            exit 1
            ;;
    esac
}

main "$@"
