$ErrorActionPreference = "Stop"

$projectRoot = $PSScriptRoot
$env:CARGO_TARGET_DIR = Join-Path $projectRoot "target"

Push-Location $projectRoot
try {
    & cargo build --locked
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
} finally {
    Pop-Location
}
