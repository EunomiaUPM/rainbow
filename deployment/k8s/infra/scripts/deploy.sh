#!/bin/bash
# ============================================================================
# Infra Namespace Deployment Script
# ============================================================================
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INFRA_DIR="$(dirname "$SCRIPT_DIR")"
NAMESPACE="rainbow-infra"

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
# Wait Functions
# ============================================================================
wait_for_deployment() {
    local deployment=$1
    local timeout=${2:-180}
    
    log_info "Waiting for deployment '$deployment' to be ready..."
    kubectl rollout status deployment/"$deployment" \
        -n "$NAMESPACE" \
        --timeout="${timeout}s"
}

# ============================================================================
# Deploy Functions
# ============================================================================
deploy_namespace() {
    log_info "Creating namespace..."
    kubectl apply -f "$INFRA_DIR/namespace.yaml"
}

deploy_configmaps() {
    log_info "Deploying ConfigMaps..."
    if [ -d "$INFRA_DIR/configmaps" ] && [ "$(ls -A "$INFRA_DIR/configmaps" 2>/dev/null)" ]; then
        kubectl apply -f "$INFRA_DIR/configmaps/"
    else
        log_warn "No configmaps found, skipping..."
    fi
}

deploy_infrastructure() {
    log_info "Deploying Infra infrastructure..."
    
    # Deploy DB and Vault first
    kubectl apply -f "$INFRA_DIR/postgres.yaml"
    kubectl apply -f "$INFRA_DIR/vault.yaml"
    
    wait_for_deployment "authority-postgres" 120
    wait_for_deployment "authority-vault" 120
}

deploy_heimdall() {
    log_info "Deploying Heimdall..."
    kubectl apply -f "$INFRA_DIR/heimdall.yaml"
    
    wait_for_deployment "heimdall" 180
}

run_jobs() {
    log_info "Running Infra jobs..."
    
    if [ -d "$INFRA_DIR/jobs" ] && [ "$(ls -A "$INFRA_DIR/jobs" 2>/dev/null)" ]; then
        kubectl delete jobs --all -n "$NAMESPACE" 2>/dev/null || true
        kubectl apply -f "$INFRA_DIR/jobs/"
        
        log_info "Waiting for jobs to complete..."
        for job in $(kubectl get jobs -n "$NAMESPACE" -o jsonpath='{.items[*].metadata.name}'); do
            kubectl wait --for=condition=complete job/"$job" \
                -n "$NAMESPACE" \
                --timeout=180s || log_warn "Job $job may still be running"
        done
    else
        log_warn "No jobs found, skipping..."
    fi
}

# ============================================================================
# Main Deployment Flow
# ============================================================================
main() {
    echo ""
    echo "=============================================="
    echo "  Infra Namespace Deployment"
    echo "=============================================="
    echo ""
    
    check_prerequisites
    
    log_info "Starting deployment..."
    
    deploy_namespace
    deploy_configmaps
    deploy_infrastructure
    run_jobs
    deploy_heimdall
    
    echo ""
    log_success "Infra deployment complete!"
    echo ""
    echo "=============================================="
    echo "  Infra Service Access (NodePorts)"
    echo "=============================================="
    echo ""
    echo "  - PostgreSQL:  localhost:31450"
    echo "  - Vault:       http://localhost:31203"
    echo "  - Heimdall:    http://localhost:31500"
    echo ""
}

main "$@"
