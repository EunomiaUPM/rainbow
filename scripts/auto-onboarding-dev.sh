#!/bin/bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib/auth.sh
source "$SCRIPT_DIR/lib/auth.sh"

# ----------------------------
# URLs
# ----------------------------
AUTHORITY_URL="${AUTHORITY_URL:-http://127.0.0.1:1500}"
CONSUMER_URL="${CONSUMER_URL:-http://127.0.0.1:1100}"
PROVIDER_URL="${PROVIDER_URL:-http://127.0.0.1:1200}"

INTERNAL_AUTHORITY_URL="${INTERNAL_AUTHORITY_URL:-$AUTHORITY_URL}"
INTERNAL_CONSUMER_URL="${INTERNAL_CONSUMER_URL:-$CONSUMER_URL}"
INTERNAL_PROVIDER_URL="${INTERNAL_PROVIDER_URL:-$PROVIDER_URL}"

# ----------------------------
# Logging (stderr)
# ----------------------------
log_step()    { echo -e "\n\033[36m$1\033[0m" >&2; }
log_success() { echo -e "\033[32m$1\033[0m" >&2; }
log_error()   { echo -e "\033[31m$1\033[0m" >&2; exit 1; }
log_info()    { echo -e "\033[33m$1\033[0m" >&2; }

# ----------------------------
# CURL
# ----------------------------
CURL_TIMEOUT="${CURL_TIMEOUT:-60}"

# Human-readable reason for the curl exit codes seen when a service is down or
# is not speaking HTTP at all (e.g. a Postgres port).
curl_reason() {
    case "$1" in
        6)  echo "no se resuelve el host" ;;
        7)  echo "conexión rechazada, no hay nada escuchando" ;;
        28) echo "timeout tras ${CURL_TIMEOUT}s" ;;
        35) echo "fallo de TLS" ;;
        52) echo "respuesta vacía: el puerto responde pero no habla HTTP" ;;
        56) echo "conexión cortada por el otro extremo" ;;
        *)  echo "error de curl" ;;
    esac
}

curl_raw() {
    local method=${1:-GET}
    local url=$2
    local body=${3:-}
    eunomia_auth "$url"
    if [ -n "$body" ]; then
        curl -s --max-time "$CURL_TIMEOUT" -X "$method" "$url" ${AUTH_ARGS[@]+"${AUTH_ARGS[@]}"} \
            -H "Content-Type: application/json" -d "$body"
    else
        curl -s --max-time "$CURL_TIMEOUT" -X "$method" "$url" ${AUTH_ARGS[@]+"${AUTH_ARGS[@]}"} \
            -H "Content-Type: application/json"
    fi
}

# Como curl_raw pero aborta ante cualquier respuesta no-2xx. Úsalo para todo lo
# que muta estado: curl_raw se traga los errores y el script reportaría éxito
# aunque un paso hubiese fallado.
curl_checked() {
    local method=${1:-GET}
    local url=$2
    local body=${3:-}
    local out code rc
    out=$(mktemp)
    eunomia_auth "$url"
    # `code=$(curl ...)` alone would abort the script under `set -e` before the
    # HTTP check runs, losing the reason. Capture curl's exit status instead.
    if [ -n "$body" ]; then
        code=$(curl -s -o "$out" -w '%{http_code}' --max-time "$CURL_TIMEOUT" \
            -X "$method" "$url" ${AUTH_ARGS[@]+"${AUTH_ARGS[@]}"} -H "Content-Type: application/json" -d "$body") \
            && rc=0 || rc=$?
    else
        code=$(curl -s -o "$out" -w '%{http_code}' --max-time "$CURL_TIMEOUT" \
            -X "$method" "$url" ${AUTH_ARGS[@]+"${AUTH_ARGS[@]}"} -H "Content-Type: application/json") \
            && rc=0 || rc=$?
    fi
    if [ "$rc" -ne 0 ]; then
        rm -f "$out"
        log_error "$method $url -> $(curl_reason "$rc") (curl exit $rc)"
    fi
    if [[ ! "$code" =~ ^2 ]]; then
        local snippet
        snippet=$(head -c 400 "$out")
        rm -f "$out"
        log_error "$method $url -> HTTP $code
$snippet"
    fi
    cat "$out"
    rm -f "$out"
}

# Fails with the full list of unreachable services instead of dying on the
# first one, so a broken dev environment is diagnosed in a single run.
preflight() {
    local failed=0 name url rc
    for entry in "authority|$AUTHORITY_URL" "consumer|$CONSUMER_URL" "provider|$PROVIDER_URL"; do
        name=${entry%%|*}
        url=${entry#*|}
        curl -s -o /dev/null --max-time 10 "$url/api/v1/wallet/is-linked" && rc=0 || rc=$?
        if [ "$rc" -ne 0 ]; then
            log_info "  $name ($url) -> $(curl_reason "$rc") (curl exit $rc)"
            failed=1
        else
            log_success "  $name ($url) -> OK"
        fi
    done
    if [ "$failed" -ne 0 ]; then
        log_error "Hay servicios inalcanzables. Arráncalos o apunta a los suyos con
AUTHORITY_URL / CONSUMER_URL / PROVIDER_URL antes de relanzar el onboarding."
    fi
}

# ----------------------------
# HEADER
# ----------------------------
echo -e "\n======================================"
echo "      AUTO ONBOARDING (DEV)"
echo "======================================"

VC_TYPE="DataSpaceParticipant_jwt_vc_json"

# ----------------------------
# STEP 0 - Reachability
# ----------------------------
log_step "STEP 0 - Comprobando servicios"
preflight

# ----------------------------
# STEP 1 - Link wallets
# ----------------------------
log_step "STEP 1 - Linking wallets"
curl_checked POST "$AUTHORITY_URL/api/v1/wallet/link" >/dev/null
curl_checked POST "$CONSUMER_URL/api/v1/wallet/link" >/dev/null
curl_checked POST "$PROVIDER_URL/api/v1/wallet/link" >/dev/null
log_success "Wallets linked"

# ----------------------------
# STEP 2 - DIDs
# ----------------------------
log_step "STEP 2 - Retrieving DIDs"
AUTH_DID=$(curl_raw GET "$AUTHORITY_URL/.well-known/did.json" | jq -r '.id')
CONSUMER_DID=$(curl_raw GET "$CONSUMER_URL/.well-known/did.json" | jq -r '.id')
PROVIDER_DID=$(curl_raw GET "$PROVIDER_URL/.well-known/did.json" | jq -r '.id')
log_success "Authority DID: $AUTH_DID"
log_success "Consumer DID:  $CONSUMER_DID"
log_success "Provider DID:  $PROVIDER_DID"

# ----------------------------
# Helpers para onboarding
# ----------------------------
request_credential() {
    local agent_name="$1"
    local agent_url="$2"

    log_step "STEP 3 ($agent_name) - Requesting credential from authority"
    if curl_raw GET "$agent_url/api/v1/vc-request/all" \
        | jq -e --arg id "$AUTH_DID" --arg vc "$VC_TYPE" \
            'any(.items[]?; .participant_id == $id
                       and .status == "Finalized"
                       and (.vc_type_config // [] | index($vc)))' >/dev/null; then
        log_info "$agent_name ya tiene la credencial, saltando"
    else
        local beg_body
        beg_body=$(jq -n \
            --arg url "$INTERNAL_AUTHORITY_URL/api/v1/gate/access" \
            --arg id "$AUTH_DID" \
            --arg nick "authority" \
            --arg vc_type "$VC_TYPE" \
            --arg method "cert" \
            '{url:$url, id:$id, nick:$nick, vc_type:$vc_type, method:$method, auto:true}')
        curl_checked POST "$agent_url/api/v1/vc-request/beg" "$beg_body" >/dev/null

        if ! curl_raw GET "$agent_url/api/v1/vc-request/all" \
            | jq -e --arg id "$AUTH_DID" --arg vc "$VC_TYPE" \
                'any(.items[]?; .participant_id == $id and .status == "Finalized" and (.vc_type_config // [] | index($vc)))' >/dev/null; then
            log_error "La solicitud de credencial de $agent_name a la autoridad no se finalizó correctamente"
        fi
        log_success "$agent_name: Credential request finished"
    fi
}

connect_peer() {
    local source_name="$1"
    local source_url="$2"
    local target_name="$3"
    local target_gate_url="$4"
    local target_did="$5"

    log_step "STEP 4 ($source_name -> $target_name) - Authenticating with peer"
    if curl_raw GET "$source_url/api/v1/peer-connection/request/all" \
        | jq -e --arg id "$target_did" \
            'any(.items[]?; .participant_id == $id and .status == "Approved" and (.final_token // "") != "")' >/dev/null; then
        log_info "$source_name ya está autenticado con $target_name, saltando"
    else
        local connect_body
        connect_body=$(jq -n \
            --arg id "$target_did" \
            --arg nick "$target_name" \
            --arg url "$target_gate_url" \
            '{id:$id, nick:$nick, url:$url, actions:["talk"], visibility:"Public", auto:true}')
        curl_checked POST "$source_url/api/v1/peer-connection/connect" "$connect_body" >/dev/null

        log_info "Esperando a que $source_name complete la verificación y emisión del token con $target_name..."
        local done=0
        local status=""
        for _ in $(seq 1 30); do
            status=$(curl_raw GET "$source_url/api/v1/peer-connection/request/all" \
                | jq -r --arg id "$target_did" \
                    '[.items[]? | select(.participant_id == $id)] | last | .status // empty')
            if [ "$status" = "Approved" ]; then
                done=1
                break
            elif [ "$status" = "Rejected" ]; then
                log_error "La conexión de $source_name con $target_name fue rechazada"
            fi
            sleep 1
        done
        if [ "$done" -ne 1 ]; then
            log_error "Timeout esperando autenticación de $source_name con $target_name (estado actual: ${status:-desconocido})"
        fi
        log_success "$source_name autenticado con éxito en $target_name"
    fi
}

# ----------------------------
# STEP 3 - Solicitud de credenciales a la autoridad
# ----------------------------
request_credential "Consumer" "$CONSUMER_URL"
request_credential "Provider" "$PROVIDER_URL"

# ----------------------------
# STEP 4 - Autenticación mutua entre Consumer y Provider
# ----------------------------
connect_peer "Consumer" "$CONSUMER_URL" "provider" "$INTERNAL_PROVIDER_URL/api/v1/gate/access" "$PROVIDER_DID"
connect_peer "Provider" "$PROVIDER_URL" "consumer" "$INTERNAL_CONSUMER_URL/api/v1/gate/access" "$CONSUMER_DID"

echo -e "\n======================================"
echo "   ONBOARDING FINISHED SUCCESSFULLY"
echo "======================================"
