#!/bin/bash
# ============================================================================
# Rainbow Dev - Teardown All Namespaces
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
# Main Teardown Flow
# ============================================================================
main() {
    echo ""
    echo "=============================================="
    echo "  Rainbow Dev - Full Stack Teardown"
    echo "=============================================="
    echo ""
    
    echo "This will teardown ALL namespaces:"
    echo "  - rainbow-provider"
    echo "  - rainbow-consumer"
    echo "  - rainbow-infra"
    echo ""
    
    read -p "Continue with full teardown? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log_warn "Teardown cancelled."
        exit 0
    fi
    
    # Ask about PVCs once for all namespaces
    read -p "Delete ALL persistent volumes (all data will be lost)? (y/N) " -n 1 -r
    echo
    DELETE_PVCS="$REPLY"
    
    echo ""
    
    # Teardown in reverse order
    log_info "=== Phase 1: Consumer Namespace ==="
    if [[ $DELETE_PVCS =~ ^[Yy]$ ]]; then
        echo "y" | "$K8S_DIR/consumer/scripts/teardown.sh" <<< "y"
    else
        echo "y" | "$K8S_DIR/consumer/scripts/teardown.sh" <<< "n"
    fi
    
    echo ""
    
    log_info "=== Phase 2: Provider Namespace ==="
    if [[ $DELETE_PVCS =~ ^[Yy]$ ]]; then
        echo "y" | "$K8S_DIR/provider/scripts/teardown.sh" <<< "y"
    else
        echo "y" | "$K8S_DIR/provider/scripts/teardown.sh" <<< "n"
    fi
    
    echo ""
    
    log_info "=== Phase 3: Infra Namespace ==="
    if [[ $DELETE_PVCS =~ ^[Yy]$ ]]; then
        echo "y" | "$K8S_DIR/infra/scripts/teardown.sh" <<< "y"
    else
        echo "y" | "$K8S_DIR/infra/scripts/teardown.sh" <<< "n"
    fi
    
    echo ""
    echo "=============================================="
    log_success "All namespaces torn down!"
    echo "=============================================="
    echo ""
}

main "$@"
