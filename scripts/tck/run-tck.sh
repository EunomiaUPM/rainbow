#!/usr/bin/env bash
# =============================================================================
# run-tck.sh — Prepares the environment and launches the DSP TCK
#              (eclipse-dataspacetck/dsp-tck release/1.0.0-RC6)
#
# Two independent choices, each with or without Docker:
#   --agents=docker|local   docker: start heimdall + mini provider/consumer with compose
#                           local:  use agents already running on this machine (cargo, IDE)
#   --tck=docker|jar        docker: run the TCK image
#                           jar:    run the TCK runtime jar with java (TCK_JAR=<path>)
# Both default to docker when the docker CLI is available, otherwise local and jar.
#
# The agents must run with DSP_TCK_MAPPING pointing at tck/tck_mapping.json: in docker mode
# the script adds it through a compose override; in local mode export it before starting them.
#
# Other flags: --seed-only, --tck-only, --no-consumer-tests.
# =============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
DEPLOYMENT_DIR="$ROOT_DIR/deployment"
HEIMDALL_DIR="${HEIMDALL_DIR:-$(cd "$ROOT_DIR/../heimdall" 2>/dev/null && pwd || echo "")}"
TCK_DIR="$ROOT_DIR/tck"
TCK_PROPS="$TCK_DIR/tck.properties"
TCK_MAPPING="$TCK_DIR/tck_mapping.json"

PROVIDER_URL="${PROVIDER_URL:-http://127.0.0.1:1200}"
CONSUMER_URL="${CONSUMER_URL:-http://127.0.0.1:1100}"
AUTHORITY_URL="${AUTHORITY_URL:-http://127.0.0.1:1500}"

# URLs visible from inside Docker containers
DOCKER_PROVIDER_URL="${DOCKER_PROVIDER_URL:-http://host.docker.internal:1200}"
DOCKER_CONSUMER_URL="${DOCKER_CONSUMER_URL:-http://host.docker.internal:1100}"
DOCKER_HOST_NAME="${DOCKER_HOST_NAME:-host.docker.internal}"

TCK_IMAGE="${TCK_IMAGE:-eclipsedataspacetck/dsp-tck-runtime:1.0.0-RC6}"
# 8888 is often taken by Jupyter on loopback, which then answers the agents' callbacks.
TCK_PORT="${TCK_PORT:-18888}"
# Runtime jar for --tck=jar, and the arguments it takes.
TCK_JAR="${TCK_JAR:-}"
TCK_JAVA_OPTS="${TCK_JAVA_OPTS:-}"

# Agent image for --agents=docker; the published one predates TCK mode.
AGENT_IMAGE="${AGENT_IMAGE:-}"
# Where the agents read the mapping, on this machine and inside their containers.
TCK_MAPPING_IN_CONTAINER="/app/tck/tck_mapping.json"
TCK_OVERRIDE_DIR="${TCK_OVERRIDE_DIR:-${TMPDIR:-/tmp}/ds-protocol-tck}"

if command -v docker >/dev/null 2>&1; then
  AGENTS_MODE="${AGENTS_MODE:-docker}"
  TCK_RUNNER="${TCK_RUNNER:-docker}"
else
  AGENTS_MODE="${AGENTS_MODE:-local}"
  TCK_RUNNER="${TCK_RUNNER:-jar}"
fi

SEED_ONLY=false
TCK_ONLY=false
NO_CONSUMER_TESTS=false

for arg in "$@"; do
  case $arg in
    --seed-only)         SEED_ONLY=true ;;
    --tck-only)          TCK_ONLY=true ;;
    --no-consumer-tests) NO_CONSUMER_TESTS=true ;;
    --agents=*)          AGENTS_MODE="${arg#*=}" ;;
    --tck=*)             TCK_RUNNER="${arg#*=}" ;;
    -h|--help)           sed -n '2,16p' "$0"; exit 0 ;;
    *)                   echo "Unknown option: $arg (see --help)" >&2; exit 2 ;;
  esac
done

case "$AGENTS_MODE" in docker|local) ;; *) echo "--agents must be docker or local" >&2; exit 2 ;; esac
case "$TCK_RUNNER" in docker|jar) ;; *) echo "--tck must be docker or jar" >&2; exit 2 ;; esac

# ── helpers ───────────────────────────────────────────────────────────────────
log()  { echo -e "\n\033[36m▶ $*\033[0m" >&2; }
ok()   { echo -e "\033[32m  ✓ $*\033[0m" >&2; }
warn() { echo -e "\033[33m  ! $*\033[0m" >&2; }
die()  { echo -e "\033[31m  ✗ $*\033[0m" >&2; exit 1; }

# shellcheck source=../lib/auth.sh
source "$SCRIPT_DIR/../lib/auth.sh"

curl_j() {
  local method=$1 url=$2 body=${3:-}
  eunomia_auth "$url"
  if [[ -n "$body" ]]; then
    curl -sf -X "$method" ${AUTH_ARGS[@]+"${AUTH_ARGS[@]}"} -H "Content-Type: application/json" -d "$body" "$url"
  else
    curl -sf -X "$method" ${AUTH_ARGS[@]+"${AUTH_ARGS[@]}"} -H "Content-Type: application/json" "$url"
  fi
}

wait_for_health() {
  local name=$1 url=$2
  log "Waiting for $name to be healthy at $url"
  local i=0
  until curl -sf "$url/.well-known/dspace-version" >/dev/null 2>&1; do
    ((i++))
    [[ $i -ge 90 ]] && die "$name did not respond after 3 minutes"
    sleep 2
  done
  ok "$name ready"
}

wait_for_authority() {
  local name=$1 url=$2
  log "Waiting for $name to be healthy at $url"
  local i=0
  until curl -sf "$url/.well-known/openid-credential-issuer" >/dev/null 2>&1; do
    ((i++))
    [[ $i -ge 90 ]] && die "$name did not respond after 3 minutes"
    sleep 2
  done
  ok "$name ready"
}

# ── Where each side reaches the other ────────────────────────────────────────
# The TCK calls the agents: from a container through the Docker host, from a jar directly.
tck_to_provider() { [[ "$TCK_RUNNER" == "docker" ]] && echo "$DOCKER_PROVIDER_URL" || echo "$PROVIDER_URL"; }
tck_to_consumer() { [[ "$TCK_RUNNER" == "docker" ]] && echo "$DOCKER_CONSUMER_URL" || echo "$CONSUMER_URL"; }
# The agents call the TCK back: from their containers through the Docker host, locally on loopback.
tck_callback() {
  [[ "$AGENTS_MODE" == "docker" ]] && echo "http://${DOCKER_HOST_NAME}:${TCK_PORT}" \
    || echo "http://127.0.0.1:${TCK_PORT}"
}

require() {
  command -v "$1" >/dev/null 2>&1 || die "$1 is required for $2"
}

# ── PHASE 1: agents ──────────────────────────────────────────────────────────
# Compose override giving <service> the TCK mapping (and AGENT_IMAGE when set).
write_override() {
  local service=$1 file="$TCK_OVERRIDE_DIR/docker-compose.tck.$1.yaml"
  mkdir -p "$TCK_OVERRIDE_DIR"
  {
    echo "services:"
    if [[ -n "$AGENT_IMAGE" ]]; then
      echo "  ${service}-setup:"
      echo "    image: \"${AGENT_IMAGE}\""
    fi
    echo "  ${service}:"
    [[ -n "$AGENT_IMAGE" ]] && echo "    image: \"${AGENT_IMAGE}\""
    echo "    environment:"
    echo "      DSP_TCK_MAPPING: \"${TCK_MAPPING_IN_CONTAINER}\""
    echo "    volumes:"
    echo "      - \"${TCK_DIR}:$(dirname "$TCK_MAPPING_IN_CONTAINER")\""
  } > "$file"
  echo "$file"
}

start_stacks() {
  require docker "--agents=docker"
  log "Starting Docker stacks (heimdall + provider + consumer) in TCK mode"

  if [[ -z "$HEIMDALL_DIR" || ! -f "$HEIMDALL_DIR/docker-compose.yml" ]]; then
    die "Heimdall directory not found. Set HEIMDALL_DIR or place heimdall repo next to ds-protocol."
  fi
  [[ -n "$AGENT_IMAGE" ]] || warn "AGENT_IMAGE not set: the compose image must already include TCK mode"

  docker compose -f "$HEIMDALL_DIR/docker-compose.yml" up -d
  docker compose -f "$DEPLOYMENT_DIR/docker-compose.mini.provider.yaml" \
    -f "$(write_override provider)" up -d
  docker compose -f "$DEPLOYMENT_DIR/docker-compose.mini.consumer.yaml" \
    -f "$(write_override consumer)" up -d
  ok "Containers started"
}

local_agents_hint() {
  log "Using agents already running locally (--agents=local)"
  warn "Start provider and consumer in TCK mode:"
  warn "  task deployment:dev:provider:tck   and   task deployment:dev:consumer:tck"
  warn "  (or export DSP_TCK_MAPPING=$TCK_MAPPING before starting them yourself)"
}

# Both agents mount the TCK trigger only in TCK mode, so a 404 there means the mapping is unset.
check_tck_mode() {
  local name url code
  for name in Provider Consumer; do
    [[ "$name" == "Provider" ]] && url=$PROVIDER_URL || url=$CONSUMER_URL
    code=$(curl -s -o /dev/null -w '%{http_code}' -X POST -H "Content-Type: application/json" \
      -d '{}' "$url/dsp/current/transfers/tck/transfers/requests" || true)
    if [[ "$code" == "404" || "$code" == "000" ]]; then
      die "$name at $url is not in TCK mode (got $code): start it with task deployment:dev:$(echo "$name" | tr '[:upper:]' '[:lower:]'):tck"
    fi
  done
  ok "Provider and consumer are in TCK mode"
}

# ── PHASE 2: onboarding ───────────────────────────────────────────────────────
# The agents address each other through the Docker host in docker mode, directly otherwise.
run_onboarding() {
  log "Executing full onboarding ($AGENTS_MODE agents)"
  if [[ "$AGENTS_MODE" == "docker" ]]; then
    AUTHORITY_URL="$AUTHORITY_URL" \
    CONSUMER_URL="$CONSUMER_URL"   \
    PROVIDER_URL="$PROVIDER_URL"   \
    DOCKER_AUTHORITY_URL="${DOCKER_AUTHORITY_URL:-http://host.docker.internal:1500}" \
    DOCKER_CONSUMER_URL="$DOCKER_CONSUMER_URL" \
    DOCKER_PROVIDER_URL="$DOCKER_PROVIDER_URL" \
      bash "$SCRIPT_DIR/../onboarding/auto-onboarding-mini.sh"
  else
    AUTHORITY_URL="$AUTHORITY_URL" \
    CONSUMER_URL="$CONSUMER_URL"   \
    PROVIDER_URL="$PROVIDER_URL"   \
      bash "$SCRIPT_DIR/../onboarding/auto-onboarding-dev.sh"
  fi
  ok "Onboarding completed"
}

# ── PHASE 3: the token the TCK presents ──────────────────────────────────────
# The TCK always calls itself TCK_PARTICIPANT and holds no GNAP grant: in TCK mode
# (DSP_TCK_MAPPING) both agents take that id as its token, on their DSP routes and towards it.
TCK_TOKEN="TCK_PARTICIPANT"

# ── PHASE 4 & 5: Seeding ──────────────────────────────────────────────────────
run_seeding() {
  log "Seeding TCK data using populate_tck.sh"
  PROVIDER_URL="$PROVIDER_URL" \
  CONSUMER_URL="$CONSUMER_URL" \
  OUTPUT_MAPPING="$TCK_MAPPING" \
    zsh "$SCRIPT_DIR/populate_tck.sh"
}

# ── PHASE 6: generate tck.properties ──────────────────────────────────────────
generate_properties() {
  local token=$1
  log "Generating $TCK_PROPS"

  if [[ ! -f "$TCK_MAPPING" ]]; then
    die "Mapping file $TCK_MAPPING not found. Seeding must have failed."
  fi

  local mapping=$(cat "$TCK_MAPPING")
  
  lookup_ds()    { echo "$mapping" | jq -r --arg k "$1" '.datasets[$k] // "MISSING"'; }
  lookup_offer() { echo "$mapping" | jq -r --arg k "$1" '.offers[$k] // "MISSING"'; }
  lookup_agr_p() { echo "$mapping" | jq -r --arg k "$1" '.provider_agreements[$k] // "MISSING"'; }
  lookup_agr_c() { echo "$mapping" | jq -r --arg k "$1" '.consumer_agreements[$k] // "MISSING"'; }

  local agent_id provider_url consumer_url
  agent_id=$(curl_j GET "$PROVIDER_URL/api/v1/mates/myself" | jq -r '.participant_id // "urn:provider"')
  provider_url=$(tck_to_provider)
  consumer_url=$(tck_to_consumer)

  cat > "$TCK_PROPS" <<EOF
# ============================================================
# DSP TCK — eclipse-dataspacetck/dsp-tck release/1.0.0-RC6
# Automatically generated by scripts/tck/run-tck.sh
# agents: ${AGENTS_MODE}, tck: ${TCK_RUNNER}
# ============================================================

dataspacetck.debug=true
dataspacetck.dsp.local.connector=false

dataspacetck.host=0.0.0.0
dataspacetck.port=${TCK_PORT}
dataspacetck.callback.address=$(tck_callback)

dataspacetck.dsp.connector.agent.id=${agent_id}
dataspacetck.dsp.connector.http.url=${provider_url}/dsp/current
dataspacetck.dsp.connector.http.base.url=${provider_url}
dataspacetck.dsp.connector.http.headers.authorization=Bearer ${token}

dataspacetck.dsp.default.wait=15000

dataspacetck.dsp.connector.negotiation.initiate.url=${consumer_url}/dsp/current/negotiations/tck/negotiations/requests
dataspacetck.dsp.connector.transfer.initiate.url=${consumer_url}/dsp/current/transfers/tck/transfers/requests

# ── Catalog ───────────────────────────────────────────────────────────────────
CAT_01_01_DATASETID=$(lookup_ds CAT0101)
CAT_01_02_DATASETID=$(lookup_ds CAT0102)
# CAT:01-03 asks for a dataset that must not exist, so it gets a fresh id, never a seeded one.
CAT_01_03_DATASETID=urn:dataset:$(uuidgen | tr 'A-Z' 'a-z')

# ── Negotiation Provider (CN) ─────────────────────────────────────────────────
CN_01_01_DATASETID=$(lookup_ds CN0101)
CN_01_01_OFFERID=$(lookup_offer CN0101)
CN_01_02_DATASETID=$(lookup_ds CN0102)
CN_01_02_OFFERID=$(lookup_offer CN0102)
CN_01_03_DATASETID=$(lookup_ds CN0103)
CN_01_03_OFFERID=$(lookup_offer CN0103)
CN_01_04_DATASETID=$(lookup_ds CN0104)
CN_01_04_OFFERID=$(lookup_offer CN0104)
CN_02_01_DATASETID=$(lookup_ds CN0201)
CN_02_01_OFFERID=$(lookup_offer CN0201)
CN_02_02_DATASETID=$(lookup_ds CN0202)
CN_02_02_OFFERID=$(lookup_offer CN0202)
CN_02_03_DATASETID=$(lookup_ds CN0203)
CN_02_03_OFFERID=$(lookup_offer CN0203)
CN_02_04_DATASETID=$(lookup_ds CN0204)
CN_02_04_OFFERID=$(lookup_offer CN0204)
CN_02_05_DATASETID=$(lookup_ds CN0205)
CN_02_05_OFFERID=$(lookup_offer CN0205)
CN_02_06_DATASETID=$(lookup_ds CN0206)
CN_02_06_OFFERID=$(lookup_offer CN0206)
CN_02_07_DATASETID=$(lookup_ds CN0207)
CN_02_07_OFFERID=$(lookup_offer CN0207)
CN_03_01_DATASETID=$(lookup_ds CN0301)
CN_03_01_OFFERID=$(lookup_offer CN0301)
CN_03_02_DATASETID=$(lookup_ds CN0302)
CN_03_02_OFFERID=$(lookup_offer CN0302)
CN_03_03_DATASETID=$(lookup_ds CN0303)
CN_03_03_OFFERID=$(lookup_offer CN0303)
CN_03_04_DATASETID=$(lookup_ds CN0304)
CN_03_04_OFFERID=$(lookup_offer CN0304)

# ── Negotiation Consumer (CN_C) ───────────────────────────────────────────────
CN_C_01_01_DATASETID=$(lookup_ds CNC0101)
CN_C_01_01_OFFERID=$(lookup_offer CNC0101)
CN_C_01_02_DATASETID=$(lookup_ds CNC0102)
CN_C_01_02_OFFERID=$(lookup_offer CNC0102)
CN_C_01_03_DATASETID=$(lookup_ds CNC0103)
CN_C_01_03_OFFERID=$(lookup_offer CNC0103)
CN_C_01_04_DATASETID=$(lookup_ds CNC0104)
CN_C_01_04_OFFERID=$(lookup_offer CNC0104)
CN_C_02_01_DATASETID=$(lookup_ds CNC0201)
CN_C_02_01_OFFERID=$(lookup_offer CNC0201)
CN_C_02_02_DATASETID=$(lookup_ds CNC0202)
CN_C_02_02_OFFERID=$(lookup_offer CNC0202)
CN_C_02_03_DATASETID=$(lookup_ds CNC0203)
CN_C_02_03_OFFERID=$(lookup_offer CNC0203)
CN_C_02_04_DATASETID=$(lookup_ds CNC0204)
CN_C_02_04_OFFERID=$(lookup_offer CNC0204)
CN_C_02_05_DATASETID=$(lookup_ds CNC0205)
CN_C_02_05_OFFERID=$(lookup_offer CNC0205)
CN_C_02_06_DATASETID=$(lookup_ds CNC0206)
CN_C_02_06_OFFERID=$(lookup_offer CNC0206)
CN_C_03_01_DATASETID=$(lookup_ds CNC0301)
CN_C_03_01_OFFERID=$(lookup_offer CNC0301)
CN_C_03_02_DATASETID=$(lookup_ds CNC0302)
CN_C_03_02_OFFERID=$(lookup_offer CNC0302)
CN_C_03_03_DATASETID=$(lookup_ds CNC0303)
CN_C_03_03_OFFERID=$(lookup_offer CNC0303)
CN_C_03_04_DATASETID=$(lookup_ds CNC0304)
CN_C_03_04_OFFERID=$(lookup_offer CNC0304)
CN_C_03_05_DATASETID=$(lookup_ds CNC0305)
CN_C_03_05_OFFERID=$(lookup_offer CNC0305)
CN_C_03_06_DATASETID=$(lookup_ds CNC0306)
CN_C_03_06_OFFERID=$(lookup_offer CNC0306)

# ── Transfer Provider (TP) ────────────────────────────────────────────────────
TP_01_01_AGREEMENTID=$(lookup_agr_p TP0101)
TP_01_01_FORMAT=HttpData-PULL
TP_01_02_AGREEMENTID=$(lookup_agr_p TP0102)
TP_01_02_FORMAT=HttpData-PULL
TP_01_03_AGREEMENTID=$(lookup_agr_p TP0103)
TP_01_03_FORMAT=HttpData-PULL
TP_01_04_AGREEMENTID=$(lookup_agr_p TP0104)
TP_01_04_FORMAT=HttpData-PULL
TP_01_05_AGREEMENTID=$(lookup_agr_p TP0105)
TP_01_05_FORMAT=HttpData-PULL
TP_02_01_AGREEMENTID=$(lookup_agr_p TP0201)
TP_02_01_FORMAT=HttpData-PULL
TP_02_02_AGREEMENTID=$(lookup_agr_p TP0202)
TP_02_02_FORMAT=HttpData-PULL
TP_02_03_AGREEMENTID=$(lookup_agr_p TP0203)
TP_02_03_FORMAT=HttpData-PULL
TP_02_04_AGREEMENTID=$(lookup_agr_p TP0204)
TP_02_04_FORMAT=HttpData-PULL
TP_02_05_AGREEMENTID=$(lookup_agr_p TP0205)
TP_02_05_FORMAT=HttpData-PULL
TP_03_01_AGREEMENTID=$(lookup_agr_p TP0301)
TP_03_01_FORMAT=HttpData-PULL
TP_03_02_AGREEMENTID=$(lookup_agr_p TP0302)
TP_03_02_FORMAT=HttpData-PULL
TP_03_03_AGREEMENTID=$(lookup_agr_p TP0303)
TP_03_03_FORMAT=HttpData-PULL
TP_03_04_AGREEMENTID=$(lookup_agr_p TP0304)
TP_03_04_FORMAT=HttpData-PULL
TP_03_05_AGREEMENTID=$(lookup_agr_p TP0305)
TP_03_05_FORMAT=HttpData-PULL
TP_03_06_AGREEMENTID=$(lookup_agr_p TP0306)
TP_03_06_FORMAT=HttpData-PULL

# ── Transfer Consumer (TP_C) ──────────────────────────────────────────────────
TP_C_01_01_AGREEMENTID=$(lookup_agr_c TPC0101)
TP_C_01_01_FORMAT=HttpData-PULL
TP_C_01_02_AGREEMENTID=$(lookup_agr_c TPC0102)
TP_C_01_02_FORMAT=HttpData-PULL
TP_C_01_03_AGREEMENTID=$(lookup_agr_c TPC0103)
TP_C_01_03_FORMAT=HttpData-PULL
TP_C_01_04_AGREEMENTID=$(lookup_agr_c TPC0104)
TP_C_01_04_FORMAT=HttpData-PULL
TP_C_01_05_AGREEMENTID=$(lookup_agr_c TPC0105)
TP_C_01_05_FORMAT=HttpData-PULL
TP_C_02_01_AGREEMENTID=$(lookup_agr_c TPC0201)
TP_C_02_01_FORMAT=HttpData-PULL
TP_C_02_02_AGREEMENTID=$(lookup_agr_c TPC0202)
TP_C_02_02_FORMAT=HttpData-PULL
TP_C_02_03_AGREEMENTID=$(lookup_agr_c TPC0203)
TP_C_02_03_FORMAT=HttpData-PULL
TP_C_02_04_AGREEMENTID=$(lookup_agr_c TPC0204)
TP_C_02_04_FORMAT=HttpData-PULL
TP_C_02_05_AGREEMENTID=$(lookup_agr_c TPC0205)
TP_C_02_05_FORMAT=HttpData-PULL
TP_C_03_01_AGREEMENTID=$(lookup_agr_c TPC0301)
TP_C_03_01_FORMAT=HttpData-PULL
TP_C_03_02_AGREEMENTID=$(lookup_agr_c TPC0302)
TP_C_03_02_FORMAT=HttpData-PULL
TP_C_03_03_AGREEMENTID=$(lookup_agr_c TPC0303)
TP_C_03_03_FORMAT=HttpData-PULL
TP_C_03_04_AGREEMENTID=$(lookup_agr_c TPC0304)
TP_C_03_04_FORMAT=HttpData-PULL
TP_C_03_05_AGREEMENTID=$(lookup_agr_c TPC0305)
TP_C_03_05_FORMAT=HttpData-PULL
TP_C_03_06_AGREEMENTID=$(lookup_agr_c TPC0306)
TP_C_03_06_FORMAT=HttpData-PULL
EOF

  ok "tck.properties generated at $TCK_PROPS"
}

# ── PHASE 7: launch TCK ───────────────────────────────────────────────────────
# Something else on the callback port would take the agents' calls meant for the TCK.
check_tck_port() {
  command -v lsof >/dev/null 2>&1 || return 0
  local holders
  # lsof exits 1 when nothing listens, which pipefail would turn into a silent exit.
  holders=$(lsof -nP -iTCP:"$TCK_PORT" -sTCP:LISTEN 2>/dev/null \
    | awk 'NR>1 {print $1" (pid "$2")"}' | sort -u || true)
  [[ -z "$holders" ]] || die "Port $TCK_PORT is taken by: $(echo $holders) — free it or set TCK_PORT"
}

run_tck() {
  [[ -f "$TCK_PROPS" ]] || die "$TCK_PROPS not found: run without --tck-only first"
  # A TCK container left from an earlier run keeps the port and the name.
  if command -v docker >/dev/null 2>&1; then
    docker rm -f dsp-tck >/dev/null 2>&1 || true
  fi
  check_tck_port
  if [[ "$TCK_RUNNER" == "docker" ]]; then
    require docker "--tck=docker"
    log "Launching TCK Docker: $TCK_IMAGE"
    docker pull "$TCK_IMAGE" >&2
    docker run --rm --name dsp-tck \
      --add-host "host.docker.internal:host-gateway" \
      -p "${TCK_PORT}:${TCK_PORT}" \
      --mount "type=bind,source=${TCK_PROPS},target=/etc/tck/config.properties" \
      "$TCK_IMAGE"
  else
    require java "--tck=jar"
    [[ -n "$TCK_JAR" && -f "$TCK_JAR" ]] \
      || die "--tck=jar needs TCK_JAR pointing at the dsp-tck runtime jar (release 1.0.0-RC6)"
    log "Launching TCK jar: $TCK_JAR"
    # shellcheck disable=SC2086
    java $TCK_JAVA_OPTS -jar "$TCK_JAR" -config "$TCK_PROPS"
  fi
}

# ── Main Orchestration ────────────────────────────────────────────────────────
main() {
  echo ""
  echo "╔══════════════════════════════════════════════════════╗"
  echo "║        DSP TCK runner — eclipse-dataspacetck         ║"
  echo "╚══════════════════════════════════════════════════════╝"
  echo "  agents: $AGENTS_MODE · tck: $TCK_RUNNER"

  if [[ "$TCK_ONLY" == "true" ]]; then
    log "--tck-only mode: skipping setup"
    run_tck
    exit 0
  fi

  if [[ "$SEED_ONLY" != "true" ]]; then
    if [[ "$AGENTS_MODE" == "docker" ]]; then
      start_stacks
    else
      local_agents_hint
    fi
    wait_for_authority "Authority" "$AUTHORITY_URL"
    wait_for_health "Provider" "$PROVIDER_URL"
    wait_for_health "Consumer" "$CONSUMER_URL"
    check_tck_mode
    run_onboarding
  fi

  run_seeding

  generate_properties "$TCK_TOKEN"

  if [[ "$SEED_ONLY" == "true" ]]; then
    ok "Seeding completed. tck.properties ready."
    exit 0
  fi

  run_tck
}

main
