#!/bin/bash
# ============================================================================
# Consumer Namespace Teardown Script
# ============================================================================
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONSUMER_DIR="$(dirname "$SCRIPT_DIR")"
NAMESPACE="rainbow-consumer"

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
# Teardown Functions
# ============================================================================
delete_jobs() {
    log_info "Deleting jobs..."
    if [ -d "$CONSUMER_DIR/jobs" ] && [ "$(ls -A "$CONSUMER_DIR/jobs" 2>/dev/null)" ]; then
        kubectl delete -f "$CONSUMER_DIR/jobs/" -n "$NAMESPACE" 2>/dev/null || true
    fi
}

delete_deployments() {
    log_info "Deleting deployments..."
    kubectl delete -f "$CONSUMER_DIR/keycloak.yaml" -n "$NAMESPACE" 2>/dev/null || true
    kubectl delete -f "$CONSUMER_DIR/vault.yaml" -n "$NAMESPACE" 2>/dev/null || true
    kubectl delete -f "$CONSUMER_DIR/redis.yaml" -n "$NAMESPACE" 2>/dev/null || true
    kubectl delete -f "$CONSUMER_DIR/postgres.yaml" -n "$NAMESPACE" 2>/dev/null || true
}

delete_configmaps() {
    log_info "Deleting configmaps..."
    if [ -d "$CONSUMER_DIR/configmaps" ] && [ "$(ls -A "$CONSUMER_DIR/configmaps" 2>/dev/null)" ]; then
        kubectl delete -f "$CONSUMER_DIR/configmaps/" -n "$NAMESPACE" 2>/dev/null || true
    fi
}

delete_pvcs() {
    log_info "Deleting PVCs..."
    kubectl delete pvc --all -n "$NAMESPACE" 2>/dev/null || true
}

delete_namespace() {
    log_info "Deleting namespace..."
    kubectl delete -f "$CONSUMER_DIR/namespace.yaml" 2>/dev/null || true
}

# ============================================================================
# Main Teardown Flow
# ============================================================================
main() {
    echo ""
    echo "=============================================="
    echo "  Consumer Namespace Teardown"
    echo "=============================================="
    echo ""
    
    read -p "This will delete ALL resources in namespace '$NAMESPACE'. Continue? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log_warn "Teardown cancelled."
        exit 0
    fi
    
    log_info "Starting teardown..."
    
    delete_jobs
    delete_deployments
    delete_configmaps
    
    read -p "Delete persistent volumes (data will be lost)? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        delete_pvcs
    else
        log_warn "PVCs preserved."
    fi
    
    delete_namespace
    
    echo ""
    log_success "Consumer teardown complete!"
    echo ""
}

main "$@"
