#!/bin/bash
# ============================================================================
# Provider Namespace Deployment Script
# ============================================================================
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROVIDER_DIR="$(dirname "$SCRIPT_DIR")"
NAMESPACE="rainbow-provider"

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
    kubectl apply -f "$PROVIDER_DIR/namespace.yaml"
}

deploy_configmaps() {
    log_info "Deploying ConfigMaps..."
    if [ -d "$PROVIDER_DIR/configmaps" ] && [ "$(ls -A "$PROVIDER_DIR/configmaps" 2>/dev/null)" ]; then
        kubectl apply -f "$PROVIDER_DIR/configmaps/"
    else
        log_warn "No configmaps found, skipping..."
    fi
}

deploy_infrastructure() {
    log_info "Deploying Provider infrastructure..."
    
    # Deploy core services
    kubectl apply -f "$PROVIDER_DIR/postgres.yaml"
    kubectl apply -f "$PROVIDER_DIR/redis.yaml"
    kubectl apply -f "$PROVIDER_DIR/vault.yaml"
    
    wait_for_deployment "provider-postgres" 120
    wait_for_deployment "provider-redis" 60
    wait_for_deployment "provider-vault" 120
}

deploy_keycloak() {
    log_info "Deploying Provider Keycloak..."
    kubectl apply -f "$PROVIDER_DIR/keycloak.yaml"
    
    wait_for_deployment "provider-keycloak" 180
}

run_jobs() {
    log_info "Running Provider jobs..."
    
    if [ -d "$PROVIDER_DIR/jobs" ] && [ "$(ls -A "$PROVIDER_DIR/jobs" 2>/dev/null)" ]; then
        # Delete previous jobs if exist
        kubectl delete jobs --all -n "$NAMESPACE" 2>/dev/null || true
        
        # Apply jobs
        kubectl apply -f "$PROVIDER_DIR/jobs/"
        
        # Wait for job completion
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
    echo "  Provider Namespace Deployment"
    echo "=============================================="
    echo ""
    
    check_prerequisites
    
    log_info "Starting deployment..."
    
    deploy_namespace
    deploy_configmaps
    deploy_infrastructure
    deploy_keycloak
    run_jobs
    
    echo ""
    log_success "Provider deployment complete!"
    echo ""
    echo "=============================================="
    echo "  Provider Service Access (NodePorts)"
    echo "=============================================="
    echo ""
    echo "  - PostgreSQL:  localhost:31400"
    echo "  - Redis:       localhost:31379"
    echo "  - Vault:       http://localhost:31201"
    echo "  - Keycloak:    http://localhost:32000"
    echo ""
}

main "$@"
