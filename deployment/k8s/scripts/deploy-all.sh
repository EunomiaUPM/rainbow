#!/bin/bash
# ============================================================================
# Rainbow Dev - Deploy All Namespaces
# ============================================================================
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
K8S_DIR="$(dirname "$SCRIPT_DIR")"

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

# ============================================================================
# Prerequisites Check
# ============================================================================
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    if ! command -v kubectl &> /dev/null; then
        log_error "kubectl not found. Please install kubectl first."
        exit 1
    fi
    
    if ! kubectl cluster-info &> /dev/null; then
        log_error "Cannot connect to Kubernetes cluster. Please check your kubeconfig."
        exit 1
    fi
    
    log_success "Prerequisites check passed"
}

# ============================================================================
# Main Deployment Flow
# ============================================================================
main() {
    echo ""
    echo "=============================================="
    echo "  Rainbow Dev - Full Stack Deployment"
    echo "=============================================="
    echo ""
    
    check_prerequisites
    
    log_info "Deploying all namespaces..."
    echo ""
    
    # 1. Deploy Infra first (Heimdall needs to be available)
    log_info "=== Phase 1: Infra Namespace ==="
    "$K8S_DIR/infra/scripts/deploy.sh"
    
    echo ""
    
    # 2. Deploy Provider
    log_info "=== Phase 2: Provider Namespace ==="
    "$K8S_DIR/provider/scripts/deploy.sh"
    
    echo ""
    
    # 3. Deploy Consumer
    log_info "=== Phase 3: Consumer Namespace ==="
    "$K8S_DIR/consumer/scripts/deploy.sh"
    
    echo ""
    echo "=============================================="
    log_success "All namespaces deployed successfully!"
    echo "=============================================="
    echo ""
    echo "Service Access Summary:"
    echo ""
    echo "  Infra (rainbow-infra):"
    echo "    - PostgreSQL:  localhost:31450"
    echo "    - Vault:       http://localhost:31203"
    echo "    - Heimdall:    http://localhost:31500"
    echo ""
    echo "  Provider (rainbow-provider):"
    echo "    - PostgreSQL:  localhost:31400"
    echo "    - Redis:       localhost:31379"
    echo "    - Vault:       http://localhost:31201"
    echo "    - Keycloak:    http://localhost:32000"
    echo ""
    echo "  Consumer (rainbow-consumer):"
    echo "    - PostgreSQL:  localhost:31300"
    echo "    - Redis:       localhost:31380"
    echo "    - Vault:       http://localhost:31202"
    echo "    - Keycloak:    http://localhost:32001"
    echo ""
    echo "=============================================="
    echo "  Next Steps: Telepresence"
    echo "=============================================="
    echo ""
    echo "  1. Connect to cluster:"
    echo "     telepresence connect"
    echo ""
    echo "  2. Intercept a service for local dev:"
    echo "     telepresence intercept <service-name> -n <namespace> --port <local>:<remote>"
    echo ""
}

main "$@"
