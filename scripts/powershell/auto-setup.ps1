# auto-setup.ps1
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

Write-Host "===> Starting Rainbow environment setup..."

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
Write-Host "Restarting databases with Docker Compose..."
docker-compose -f $DockerComposePath down -v
docker-compose -f $DockerComposePath up -d

Write-Host "Waiting for databases to be ready..."
Start-Sleep -Seconds 5

# ----------------------------
# Authority setup
# ----------------------------
Write-Host "Running setup for Authority..."
cargo run --manifest-path (Join-Path $BaseDir "rainbow-authority\Cargo.toml") setup `
    --env-file (Join-Path $BaseDir "static\envs\.env.authority")

# ----------------------------
# Consumer setup
# ----------------------------
if ($Module -eq "core") {
    Write-Host "Running setup for Consumer (all modules)..."
    cargo run --manifest-path (Join-Path $BaseDir "rainbow-core\Cargo.toml") consumer setup `
        --env-file (Join-Path $BaseDir "static\envs\.env.consumer.core")
} else {
    Write-Host "Running setup for Consumer module: $Module..."
    cargo run --manifest-path (Join-Path $BaseDir "rainbow-$Module\Cargo.toml") consumer setup `
        --env-file (Join-Path $BaseDir "static\envs\.env.consumer.core")
}

# ----------------------------
# Provider setup
# ----------------------------
if ($Module -eq "core") {
    Write-Host "Running setup for Provider (all modules)..."
    cargo run --manifest-path (Join-Path $BaseDir "rainbow-core\Cargo.toml") provider setup `
        --env-file (Join-Path $BaseDir "static\envs\.env.provider.core")
} else {
    Write-Host "Running setup for Provider module: $Module..."
    cargo run --manifest-path (Join-Path $BaseDir "rainbow-$Module\Cargo.toml") provider setup `
        --env-file (Join-Path $BaseDir "static\envs\.env.provider.core")
}

Write-Host ""
Write-Host "===> Setup completed successfully for module '$Module'"
