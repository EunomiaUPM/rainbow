# prepare-docker-scripts.ps1
Write-Host "Preparing Docker scripts for Windows/Linux compatibility..." -ForegroundColor Cyan

$scriptDir = $PSScriptRoot
$vaultDir = Join-Path $scriptDir "vault"

# List of scripts to convert
$scripts = @(
    "vault-setup.sh",
    "vault-init.sh",
    "secrets-gen.sh"
)

# Ensure vault directory exists
if (-not (Test-Path $vaultDir)) {
    Write-Error "Vault directory not found at $vaultDir"
    exit 1
}

foreach ($script in $scripts) {
    $sourcePath = Join-Path $vaultDir $script
    $destName = $script.Replace(".sh", ".linux.sh")
    $destPath = Join-Path $vaultDir $destName

    if (Test-Path $sourcePath) {
        Write-Host "Converting $script to $destName (CRLF -> LF)..."
        
        $content = [System.IO.File]::ReadAllText($sourcePath)
        $contentLF = $content -replace "`r`n", "`n"
        
        $utf8NoBom = New-Object System.Text.UTF8Encoding $false
        [System.IO.File]::WriteAllText($destPath, $contentLF, $utf8NoBom)
    }
    else {
        Write-Warning "Source script not found: $sourcePath"
    }
}

Write-Host "Script preparation complete." -ForegroundColor Green
