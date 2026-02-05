#!/bin/bash

# ==========================================
# CONFIGURATION
# ==========================================
AGENT_URL="http://127.0.0.1:1200"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
NC='\033[0m'

# ==========================================
# HELPER FUNCTIONS
# ==========================================
invoke_curl() {
    local method="$1"
    local url="$2"
    local body="$3"
    local description="$4"

    echo -e "${GREEN}--- $description ---${NC}" >&2
    echo "Request: $method $url" >&2

    if [[ -n "$body" ]]; then
        response=$(curl -s -X "$method" -H "Content-Type: application/json" -d "$body" "$url")
    else
        response=$(curl -s -X "$method" "$url")
    fi

    echo "Response (preview): ${response:0:100}..." >&2
    echo "---------------------------------------------------" >&2
    echo "$response"
}

# ==========================================
# TRANSFER AGENT FUNCTIONS
# ==========================================

# 1. Create Transfer Process
# Args: consumer_did, agreement_id, initial_state
create_transfer_process() {
    local consumer_did="$1"
    local agreement_id="$2"
    local state="$3"

    # "transferDirection": "OUTGOING" as we are acting as Provider sending data
    local body=$(jq -n \
        --arg peer "$consumer_did" \
        --arg agId "$agreement_id" \
        --arg st "$state" \
        '{
        "state": $st,
        "associatedAgentPeer": $peer,
        "protocol": "dataspace-protocol-http",
        "transferDirection": "OUTGOING",
        "agreementId": $agId,
        "role": "PROVIDER",
        "identifiers": {
            "localId": ("urn:uuid:" + (now|tostring)),
            "remoteId": ("urn:uuid:" + (now|tostring) + "-remote")
        }
    }')
    invoke_curl "POST" "$AGENT_URL/api/v1/transfer-agent/transfer-processes" "$body" "Creating Transfer Process ($state)" | jq -r '.id // empty'
}

# 2. Update Transfer Process State (Simulates internal state changes)
update_transfer_state() {
    local process_id="$1"
    local new_state="$2"

    local body=$(jq -n --arg st "$new_state" '{ "state": $st }')
    invoke_curl "PUT" "$AGENT_URL/api/v1/transfer-agent/transfer-processes/$process_id" "$body" "Updating Transfer State -> $new_state" >/dev/null
}

# 3. Create Transfer Message
# Args: process_id, type, from_state, to_state
create_transfer_message() {
    local process_id="$1"
    local type="$2"
    local from="$3"
    local to="$4"

    # Determine direction based on message type (DSP Logic)
    # REQUEST comes IN, START/COMPLETION goes OUT
    local direction="OUTGOING"
    if [[ "$type" == "TRANSFER_REQUEST" ]]; then
        direction="INCOMING"
    fi

    local body=$(jq -n \
        --arg pid "$process_id" \
        --arg type "$type" \
        --arg dir "$direction" \
        --arg from "$from" \
        --arg to "$to" \
        '{
        "transferAgentProcessId": $pid,
        "direction": $dir,
        "protocol": "dataspace-protocol-http",
        "messageType": $type,
        "stateTransitionFrom": $from,
        "stateTransitionTo": $to,
        "payload": {
            "dataAddress": { "properties": { "endpoint": "http://dummy/data" } }
        }
    }')
    invoke_curl "POST" "$AGENT_URL/api/v1/transfer-agent/transfer-messages" "$body" "Msg: $type ($from -> $to)" | jq -r '.id // empty'
}

# ==========================================
# SIMULATION SCENARIOS
# ==========================================



# SCENARIO A: HAPPY PATH (Completed Successfully)
# REQUESTED -> STARTED -> COMPLETED
simulate_successful_transfer() {
    local consumer="$1"
    local agreement="$2"
    local label="$3"

    echo -e "${YELLOW}>>> Starting Transfer Scenario: SUCCESSFUL ($label)${NC}" >&2

    # 1. REQUESTED (Consumer asks for data)
    local PID=$(create_transfer_process "$consumer" "$agreement" "REQUESTED")
    create_transfer_message "$PID" "TRANSFER_REQUEST" "INITIAL" "REQUESTED" >/dev/null

    # 2. STARTED (Provider starts sending data)
    update_transfer_state "$PID" "STARTED"
    create_transfer_message "$PID" "TRANSFER_START" "REQUESTED" "STARTED" >/dev/null

    # 3. COMPLETED (Data transfer finished)
    update_transfer_state "$PID" "COMPLETED"
    create_transfer_message "$PID" "TRANSFER_COMPLETION" "STARTED" "COMPLETED" >/dev/null

    echo -e "${GREEN}>>> Finished Transfer: SUCCESSFUL (Process: $PID)${NC}\n" >&2
}

# SCENARIO B: TERMINATED (Failed/Cancelled)
# REQUESTED -> STARTED -> TERMINATED
simulate_terminated_transfer() {
    local consumer="$1"
    local agreement="$2"
    local label="$3"

    echo -e "${YELLOW}>>> Starting Transfer Scenario: TERMINATED ($label)${NC}" >&2

    # 1. REQUESTED
    local PID=$(create_transfer_process "$consumer" "$agreement" "REQUESTED")
    create_transfer_message "$PID" "TRANSFER_REQUEST" "INITIAL" "REQUESTED" >/dev/null

    # 2. STARTED
    update_transfer_state "$PID" "STARTED"
    create_transfer_message "$PID" "TRANSFER_START" "REQUESTED" "STARTED" >/dev/null

    # 3. TERMINATED (Something went wrong)
    update_transfer_state "$PID" "TERMINATED"
    create_transfer_message "$PID" "TRANSFER_TERMINATION" "STARTED" "TERMINATED" >/dev/null

    echo -e "${RED}>>> Finished Transfer: TERMINATED (Process: $PID)${NC}\n" >&2
}

# SCENARIO C: SUSPENDED (Paused)
simulate_suspended_transfer() {
    local consumer="$1"
    local agreement="$2"
    local label="$3"

    echo -e "${YELLOW}>>> Starting Transfer Scenario: SUSPENDED ($label)${NC}" >&2

    local PID=$(create_transfer_process "$consumer" "$agreement" "STARTED")
    create_transfer_message "$PID" "TRANSFER_START" "REQUESTED" "STARTED" >/dev/null

    update_transfer_state "$PID" "SUSPENDED"
    # DSP message type for suspension might be generic or TERMINATION with reason,
    # but strictly speaking simple suspension is often internal state.
    # We leave it without message or invent a status message.

    echo -e "${GREEN}>>> Finished Transfer: SUSPENDED (Process: $PID)${NC}\n" >&2
}

# ==========================================
# MAIN EXECUTION
# ==========================================

main() {
    echo -e "${GREEN}=== GENERATING MOCK TRANSFER DATA ===${NC}" >&2

    # --- MOCK DATA ---
    # En un flujo real, usarías el Agreement ID retornado por el script de Negociación.
    # Aquí usamos uno falso si no pasas uno como argumento.
    CONSUMER_A="did:web:consumer:company-a"
    AGREEMENT_ID=${1:-"urn:agreement:mock-agreement-001"}

    echo "Using Agreement ID: $AGREEMENT_ID" >&2

    # --- EXECUTE SCENARIOS ---

    # 1. Transferencia Completada (Happy Path)
    simulate_successful_transfer "$CONSUMER_A" "$AGREEMENT_ID" "Daily Batch Download"

    # 2. Transferencia Fallida (Error de Red simulado)
    simulate_terminated_transfer "$CONSUMER_A" "$AGREEMENT_ID" "Real-time Stream (Failed)"

    # 3. Transferencia Suspendida
    simulate_suspended_transfer "$CONSUMER_A" "$AGREEMENT_ID" "Large File Upload (Paused)"

    echo -e "${GREEN}=== DONE. GENERATED TRANSFER PROCESSES ===${NC}" >&2
}

# Pass Agreement ID as argument if available
main "$@"