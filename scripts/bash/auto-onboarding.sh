#!/bin/bash

# Definir URLs
AuthorityUrl="https://authority.example.com"
ConsumerUrl="https://consumer.example.com"
ProviderUrl="https://provider.example.com"

# 1. A Onboarding
echo "==> A Onboarding Copy 2"
curl -s -X POST "$AuthorityUrl/api/v1/wallet/onboard"

# 2. C Onboarding
echo "==> C Onboarding Copy 2"
curl -s -X POST "$ConsumerUrl/api/v1/wallet/onboard"

# 3. P Onboarding
echo "==> P Onboarding Copy 2"
curl -s -X POST "$ProviderUrl/api/v1/wallet/onboard"

# 4. Obtener DIDs
echo "==> A Did Copy"
AuthorityDid=$(curl -s -X GET "$AuthorityUrl/api/v1/did.json" | jq -r '.id')
echo "AuthorityDid=$AuthorityDid"

echo "==> C Did Copy"
ConsumerDid=$(curl -s -X GET "$ConsumerUrl/api/v1/did.json" | jq -r '.id')
echo "ConsumerDid=$ConsumerDid"

echo "==> P Did Copy"
ProviderDid=$(curl -s -X GET "$ProviderUrl/api/v1/did.json" | jq -r '.id')
echo "ProviderDid=$ProviderDid"

# 5. C Beg 4 Credential
echo "==> C Beg 4 Credential Copy"
curl -s -X POST "$ConsumerUrl/api/v1/authority/beg" \
  -H "Content-Type: application/json" \
  -d "{
        \"url\": \"http://127.0.0.1:1500/api/v1/request/credential\",
        \"id\": \"$AuthorityDid\",
        \"slug\": \"authority\",
        \"vc_type\": \"DataspaceParticipantCredential\"
      }"

# 6. A All Requests
echo "==> A All Requests Copy"
PetitionId=$(curl -s -X GET "$AuthorityUrl/api/v1/request/all" | jq -r '.[-1].id')
echo "PetitionId=$PetitionId"

# 7. A Accept Request
echo "==> A Accept Request Copy"
curl -s -X POST "$AuthorityUrl/api/v1/request/$PetitionId" \
  -H "Content-Type: application/json" \
  -d '{"approve": true}'

# 8. C All Authorities
echo "==> C All Authorities Copy 2"
OIDC4VCI_URI=$(curl -s -X GET "$ConsumerUrl/api/v1/authority/request/all" | jq -r '.[-1].vc_uri')
echo "OIDC4VCI_URI=$OIDC4VCI_URI"

# 9. C Manage OIDC4VCI
echo "==> C Manage OIDC4VCI Copy"
curl -s -X POST "$ConsumerUrl/api/v1/process/oidc4vci" \
  -H "Content-Type: application/json" \
  -d "{\"uri\": \"$OIDC4VCI_URI\"}"

# 10. C Grant Request
echo "==> C Grant Request Copy 2"
OIDC4VP_URI=$(curl -s -X POST "$ConsumerUrl/api/v1/request/onboard/provider" \
  -H "Content-Type: application/json" \
  -d "{
        \"url\": \"http://127.0.0.1:1200/api/v1/access\",
        \"id\": \"$ProviderDid\",
        \"slug\": \"provider\",
        \"actions\": \"talk\"
      }")
echo "OIDC4VP_URI=$OIDC4VP_URI"

# 11. C Manage OIDC4VP
echo "==> C Manage OIDC4VP"
curl -s -X POST "$ConsumerUrl/api/v1/process/oidc4vp" \
  -H "Content-Type: application/json" \
  -d "{\"uri\": \"$OIDC4VP_URI\"}"
