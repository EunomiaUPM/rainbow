param(
    [Parameter(Position = 0)]
    [string]$Module = "core",

    [Parameter(Position = 1)]
    [switch]$Watch
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

    $psCommand = "cd '$WorkingDir'; $Command"

    Start-Process powershell -ArgumentList "-NoExit", "-Command", $psCommand -WindowStyle Normal
}

# ----------------------------
# Helper to choose cargo command
# ----------------------------
function Get-CargoCommand {
    param([string]$RunArgs)
    if ($Watch) {
        return "cargo watch -x 'run $RunArgs'"
    } else {
        return "cargo run $RunArgs"
    }
}



# ----------------------------
# Authority
# ----------------------------
#$authorityCmd = Get-CargoCommand "--manifest-path Cargo.toml start --env-file ../static/envs/.env.authority"
#Start-ServiceWindow -Title "Authority" `
#    -WorkingDir (Join-Path $BaseDir "rainbow-authority") `
#    -Command $authorityCmd

# ----------------------------
# Consumer
# ----------------------------
if ($Module -eq "core") {
    $consumerPath = Join-Path $BaseDir "rainbow-core"
} else {
    $consumerPath = Join-Path $BaseDir "rainbow-$Module"
}
$consumerCmd = Get-CargoCommand "--manifest-path Cargo.toml consumer start --env-file ../static/envs/core.consumer.yaml"
Start-ServiceWindow -Title "Consumer" -WorkingDir $consumerPath -Command $consumerCmd

# ----------------------------
# Provider
# ----------------------------
$providerCmd = Get-CargoCommand "--manifest-path Cargo.toml provider start --env-file ../static/envs/core.provider.yaml"
Start-ServiceWindow -Title "Provider" -WorkingDir $consumerPath -Command $providerCmd

Write-Host ""
Write-Host "===> Rainbow services started successfully for module '$Module' (watch mode: $Watch)"
