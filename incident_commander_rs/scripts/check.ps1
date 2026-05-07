param(
  [switch]$WithDocker
)

$ErrorActionPreference = "Stop"

Write-Host "[1/4] cargo fmt --check"
cargo fmt --check

Write-Host "[2/4] cargo check"
cargo check

Write-Host "[3/4] cargo test"
cargo test

Write-Host "[4/4] validate sample JSON"
Get-Content -Raw examples/sample_incident.json | ConvertFrom-Json | Out-Null
Get-Content -Raw examples/empty_invalid.json | ConvertFrom-Json | Out-Null

if ($WithDocker) {
  Write-Host "[docker] build image"
  docker build -t incident-commander-rs .
}

Write-Host "ok"
