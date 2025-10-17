# auto-start.ps1
param(
    [string]$Module = "core"
)

# ----------------------------
# Find the rainbow base directory
# ----------------------------
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

$BaseDir = $ScriptDir
while (-not (Test-Path (Join-Path $BaseDir "deployment")) -and $BaseDir -ne (Split-Path $BaseDir -Parent)) {
    $BaseDir = Split-Path $BaseDir -Parent
}

if (-not (Test-Path (Join-Path $BaseDir "deployment"))) {
    Write-Host "ERROR: Could not find 'rainbow' root directory containing 'deployment'."
    exit 1
}

$DockerComposePath = Join-Path $BaseDir "deployment\docker-compose.core.yaml"

Write-Host "===> Starting Rainbow environment..."

# ----------------------------
# Valid modules
# ----------------------------
$ValidModules = @("core", "catalog", "contracts", "transfer", "auth")

if (-not ($ValidModules -contains $Module)) {
    Write-Host "ERROR: Invalid module '$Module'. Valid options: $($ValidModules -join ', ')"
    exit 1
}

# ----------------------------
# Start databases
# ----------------------------
Write-Host "Starting databases with Docker Compose..."
docker-compose -f $DockerComposePath up -d

Write-Host "Waiting for databases to be ready..."
Start-Sleep -Seconds 5

# ----------------------------
# Helper function to start a new PowerShell window
# ----------------------------
function Start-ServiceWindow {
    param (
        [string]$Title,
        [string]$WorkingDir,
        [string]$Command
    )

    Write-Host "Starting $Title..."
    Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd `"$WorkingDir`"; $Command" -WindowStyle Normal
}

# ----------------------------
# Authority
# ----------------------------
Start-ServiceWindow -Title "Authority" `
    -WorkingDir (Join-Path $BaseDir "rainbow-authority") `
    -Command "cargo run --manifest-path Cargo.toml start --env-file ../static/envs/.env.authority"

# ----------------------------
# Consumer
# ----------------------------
if ($Module -eq "core") {
    Start-ServiceWindow -Title "Consumer" `
        -WorkingDir (Join-Path $BaseDir "rainbow-core") `
        -Command "cargo run --manifest-path Cargo.toml consumer start --env-file ../static/envs/.env.consumer.core"
} else {
    Start-ServiceWindow -Title "Consumer" `
        -WorkingDir (Join-Path $BaseDir "rainbow-$Module") `
        -Command "cargo run --manifest-path Cargo.toml consumer start --env-file ../static/envs/.env.consumer.core"
}

# ----------------------------
# Provider
# ----------------------------
if ($Module -eq "core") {
    Start-ServiceWindow -Title "Provider" `
        -WorkingDir (Join-Path $BaseDir "rainbow-core") `
        -Command "cargo run --manifest-path Cargo.toml provider start --env-file ../static/envs/.env.provider.core"
} else {
    Start-ServiceWindow -Title "Provider" `
        -WorkingDir (Join-Path $BaseDir "rainbow-$Module") `
        -Command "cargo run --manifest-path Cargo.toml provider start --env-file ../static/envs/.env.provider.core"
}

Write-Host ""
Write-Host "===> Rainbow services started successfully for module '$Module'"
