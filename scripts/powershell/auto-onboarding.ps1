# auto-onboarding.ps1
param(
    [string]$AuthorityUrl = "http://127.0.0.1:1500",
    [string]$ConsumerUrl  = "http://127.0.0.1:1100",
    [string]$ProviderUrl  = "http://127.0.0.1:1200"
)

function Invoke-CurlJson {
    param(
        [string]$Method = "GET",
        [string]$Url,
        [object]$Body = $null,
        [bool]$ParseJson = $true  # parsea a JSON solo si se espera JSON
    )

    try {
        $Params = @{
            Method      = $Method
            Uri         = $Url
            ContentType = "application/json"
            ErrorAction = 'Stop'
        }

        if ($Body) { $Params.Body = $Body | ConvertTo-Json -Compress }

        $Response = Invoke-WebRequest @Params

        if ($Response.StatusCode -ge 200 -and $Response.StatusCode -lt 300) {
            Write-Host "SUCCESS: $Method $Url returned $($Response.StatusCode)" -ForegroundColor Green
        } else {
            Write-Host "ERROR: $Method $Url returned $($Response.StatusCode)" -ForegroundColor Red
            exit 1
        }

        if ($ParseJson -and $Response.Content) {
            return $Response.Content | ConvertFrom-Json
        } else {
            return $Response.Content
        }

    } catch {
        Write-Host "ERROR: Request to $Url failed" -ForegroundColor Red
        Write-Host $_.Exception.Message -ForegroundColor Red
        Write-Host "The script wont continue executing" -ForegroundColor Red
        Write-Host ""
        exit 1
    }
}

Write-Host "Starting auto-onboarding script..."

# ----------------------------
# Onboarding Authority / Consumer / Provider (no se parsea JSON)
# ----------------------------
Invoke-CurlJson -Method "POST" -Url "$AuthorityUrl/api/v1/wallet/onboard" -ParseJson:$false
Invoke-CurlJson -Method "POST" -Url "$ConsumerUrl/api/v1/wallet/onboard" -ParseJson:$false
Invoke-CurlJson -Method "POST" -Url "$ProviderUrl/api/v1/wallet/onboard" -ParseJson:$false

# ----------------------------
# Getting DIDs
# ----------------------------
$AUTH_DID     = (Invoke-CurlJson -Url "$AuthorityUrl/api/v1/did.json").id
Write-Host "Authority DID: $AUTH_DID"
$CONSUMER_DID = (Invoke-CurlJson -Url "$ConsumerUrl/api/v1/did.json").id
Write-Host "Consumer DID: $CONSUMER_DID"
$PROVIDER_DID = (Invoke-CurlJson -Url "$ProviderUrl/api/v1/did.json").id
Write-Host "Provider DID: $PROVIDER_DID"

# ----------------------------
# Consumer begins request for credential
# ----------------------------
$C_BEG_BODY = @{
    url = "$AuthorityUrl/api/v1/request/credential"
    id  = $AUTH_DID
    slug = "authority"
    vc_type = "DataspaceParticipantCredential"
}
$C_BEG_RESPONSE = Invoke-CurlJson -Method "POST" -Url "$ConsumerUrl/api/v1/authority/beg" -Body $C_BEG_BODY -ParseJson:$false
Write-Host "Consumer request completed."

# ----------------------------
# Get all requests from Authority
# ----------------------------
$ALL_REQUESTS = Invoke-CurlJson -Url "$AuthorityUrl/api/v1/request/all"
$PETITION_ID = $ALL_REQUESTS[-1].id
Write-Host "Petition ID: $PETITION_ID"

# ----------------------------
# Authority approves request
# ----------------------------
$APPROVE_BODY = @{ approve = $true }
Invoke-CurlJson -Method "POST" -Url "$AuthorityUrl/api/v1/request/$PETITION_ID" -Body $APPROVE_BODY -ParseJson:$false
Write-Host "Request approved."

# ----------------------------
# Get all authority requests for Consumer
# ----------------------------
$ALL_AUTHORITY = Invoke-CurlJson -Url "$ConsumerUrl/api/v1/authority/request/all"
$OIDC4VCI_URI = $ALL_AUTHORITY[-1].vc_uri
Write-Host "OIDC4VCI_URI: $OIDC4VCI_URI"

# ----------------------------
# Consumer processes OIDC4VCI
# ----------------------------
Invoke-CurlJson -Method "POST" -Url "$ConsumerUrl/api/v1/process/oidc4vci" -Body @{ uri = $OIDC4VCI_URI } -ParseJson:$false
Write-Host "OIDC4VCI processed."

# ----------------------------
# Consumer requests grant from Provider
# ----------------------------
$OIDC4VP_BODY = @{
    url = "$ProviderUrl/api/v1/access"
    id  = $PROVIDER_DID
    slug = "provider"
    actions = "talk"
}
$OIDC4VP_URI = Invoke-CurlJson -Method "POST" -Url "$ConsumerUrl/api/v1/request/onboard/provider" -Body $OIDC4VP_BODY -ParseJson:$false
Write-Host "OIDC4VP_URI: $OIDC4VP_URI"

# ----------------------------
# Consumer processes OIDC4VP
# ----------------------------
Write-Host "Consumer processes OIDC4VP..."
Invoke-CurlJson -Method "POST" -Url "$ConsumerUrl/api/v1/process/oidc4vp" -Body @{ uri = $OIDC4VP_URI } -ReturnJson:$false
Write-Host "OIDC4VP processed." -ForegroundColor Green

Write-Host "Onboarding script finished successfully!" -ForegroundColor Green
Write-Host ""