#!/bin/bash
# OAuth helpers shared by the scripts. Source it, then add "${AUTH_ARGS[@]}" to curl calls
# after `eunomia_auth <base_url>`, or use `eunomia_curl`.
#
# Env: EUNOMIA_TOKEN (optional access token issued by the identity provider, e.g. Keycloak).
#      EUNOMIA_KEYCLOAK=1: the agents are behind Keycloak (mini with -Auth keycloak). The calls
#        go through the edge by name (http://provider.localhost, ...) and carry a token of the
#        realm's service account, asked with client credentials; the realm is the first label
#        of the target's host name (provider.localhost -> provider).
#        KEYCLOAK_URL (default http://keycloak.localhost), EUNOMIA_CLIENT_ID (default cli),
#        EUNOMIA_CLIENT_SECRET (default <realm>-cli-secret, the example secret of the mini).
#        Without it, the mode is detected: static when the provider answers on its own port
#        (http://127.0.0.1:1200), Keycloak when it answers by name through the edge
#        (http://provider.localhost). EUNOMIA_KEYCLOAK=0 forces static.
#      EUNOMIA_PROVIDER_URL / EUNOMIA_CONSUMER_URL: where the agents are (defaults per mode).
#
# The agents no longer issue tokens nor take a tenant: the user comes from the token. Without
# EUNOMIA_TOKEN nor Keycloak the calls go without one, which an agent in `static` mode serves as
# its configured user (by default the root, /admin).

# Whether an agent answers at <base url> (any HTTP status counts).
eunomia_answers() {
    local code
    code=$(curl -s -o /dev/null -m 2 -w "%{http_code}" "$1/.well-known/dspace-version" || true)
    [ -n "$code" ] && [ "$code" != "000" ]
}

if [ -z "${EUNOMIA_KEYCLOAK:-}" ]; then
    if eunomia_answers "${EUNOMIA_PROVIDER_URL:-http://127.0.0.1:1200}"; then
        EUNOMIA_KEYCLOAK=0
    elif eunomia_answers "http://provider.localhost"; then
        EUNOMIA_KEYCLOAK=1
    else
        EUNOMIA_KEYCLOAK=0
        echo "Warning: no provider answers on http://127.0.0.1:1200 nor on http://provider.localhost; is the deployment up?" >&2
    fi
fi
export EUNOMIA_KEYCLOAK

if [ "${EUNOMIA_KEYCLOAK:-}" = "1" ]; then
    EUNOMIA_PROVIDER_URL="${EUNOMIA_PROVIDER_URL:-http://provider.localhost}"
    EUNOMIA_CONSUMER_URL="${EUNOMIA_CONSUMER_URL:-http://consumer.localhost}"
else
    EUNOMIA_PROVIDER_URL="${EUNOMIA_PROVIDER_URL:-http://127.0.0.1:1200}"
    EUNOMIA_CONSUMER_URL="${EUNOMIA_CONSUMER_URL:-http://127.0.0.1:1100}"
fi
# Service-account tokens are cached per realm in a directory shared with subshells.
export EUNOMIA_TOKEN_DIR="${EUNOMIA_TOKEN_DIR:-$(mktemp -d)}"

if [ -z "${EUNOMIA_MODE_SHOWN:-}" ]; then
    if [ "$EUNOMIA_KEYCLOAK" = "1" ]; then
        echo "Mode: Keycloak (provider $EUNOMIA_PROVIDER_URL, service-account token)" >&2
    else
        echo "Mode: static (provider $EUNOMIA_PROVIDER_URL, no token)" >&2
    fi
    export EUNOMIA_MODE_SHOWN=1
fi

# Former login of every agent, with tokens cached per agent:
# ADMIN_EMAIL="${ADMIN_EMAIL:-admin@admin.local}"
# ADMIN_PASSWORD="${ADMIN_PASSWORD:-admin}"
# export EUNOMIA_TOKEN_DIR="${EUNOMIA_TOKEN_DIR:-$(mktemp -d)}"

# Prints the scheme://host:port part of a URL.
eunomia_origin() {
    printf '%s' "$1" | sed -E 's#^(https?://[^/]+).*#\1#'
}

# Prints the access token to send to the agent serving <url> (empty: none): EUNOMIA_TOKEN if
# set; with EUNOMIA_KEYCLOAK=1, the service account's token of the target's realm (asked on
# first use); otherwise none.
eunomia_token() {
    if [ -n "${EUNOMIA_TOKEN:-}" ]; then
        printf '%s' "$EUNOMIA_TOKEN"
        return 0
    fi
    [ "${EUNOMIA_KEYCLOAK:-}" = "1" ] || return 0

    local host realm client cache token
    host=$(printf '%s' "$1" | sed -E 's#^https?://([^/:]+).*#\1#')
    realm="${host%%.*}"
    client="${EUNOMIA_CLIENT_ID:-cli}"
    cache="$EUNOMIA_TOKEN_DIR/$realm"
    if [ ! -s "$cache" ]; then
        token=$(curl -s -X POST "${KEYCLOAK_URL:-http://keycloak.localhost}/realms/$realm/protocol/openid-connect/token" \
            -d grant_type=client_credentials \
            -d "client_id=$client" \
            -d "client_secret=${EUNOMIA_CLIENT_SECRET:-$realm-cli-secret}" \
            | jq -r '.access_token // empty')
        if [ -z "$token" ]; then
            echo "Keycloak sign-in of client $client in realm $realm failed" >&2
            return 1
        fi
        printf '%s' "$token" > "$cache"
    fi
    cat "$cache"
}

# Former version, logging into the agent's own /oauth/token (gone with the built-in OAuth module):
# # Prints an access token for the agent serving <url>, logging in on first use.
# eunomia_token() {
#     local origin cache token
#     origin=$(eunomia_origin "$1")
#     cache="$EUNOMIA_TOKEN_DIR/$(printf '%s' "$origin" | tr -c 'A-Za-z0-9' '_')"
#     if [ ! -s "$cache" ]; then
#         token=$(curl -s -X POST "$origin/oauth/token" -H "Content-Type: application/json" \
#             -d "{\"grant_type\":\"password\",\"client_id\":\"eunomia-admin-gui\",\"username\":\"$ADMIN_EMAIL\",\"password\":\"$ADMIN_PASSWORD\"}" \
#             | jq -r '.access_token // empty')
#         if [ -z "$token" ]; then
#             echo "Login as $ADMIN_EMAIL on $origin failed" >&2
#             return 1
#         fi
#         printf '%s' "$token" > "$cache"
#     fi
#     cat "$cache"
# }

# Sets AUTH_ARGS to the curl header arguments for the agent serving <url>.
# Without a token the request goes without the header.
eunomia_auth() {
    local token
    token=$(eunomia_token "$1") || token=""
    AUTH_ARGS=()
    if [ -n "$token" ]; then
        AUTH_ARGS+=(-H "Authorization: Bearer $token")
    fi
    # Tenants are gone: the token's user decides what the call sees.
    # if [ -n "${TENANT:-}" ]; then
    #     AUTH_ARGS+=(-H "x-tenant-id: $TENANT")
    # fi
}

# curl with the agent's credentials: eunomia_curl <method> <url> [json body] [extra curl args...]
eunomia_curl() {
    local method="$1" url="$2" body="${3:-}"
    shift 3 2>/dev/null || shift $#
    eunomia_auth "$url"
    if [ -n "$body" ]; then
        curl -s -X "$method" "${AUTH_ARGS[@]}" -H "Content-Type: application/json" -d "$body" "$@" "$url"
    else
        curl -s -X "$method" "${AUTH_ARGS[@]}" "$@" "$url"
    fi
}
