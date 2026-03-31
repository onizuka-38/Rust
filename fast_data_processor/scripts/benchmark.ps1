param(
  [string]$Input = "data/trades_1m.ndjson",
  [int]$BatchSize = 50000
)

Write-Host "[1/3] Generate sample dataset if missing..."
if (!(Test-Path $Input)) {
  cargo run -- bench-input --output $Input --lines 1000000 --symbols 6
}

Write-Host "[2/3] Run serial mode..."
$serial = cargo run --quiet -- crypto-stats --input $Input --mode serial --batch-size $BatchSize --top 10
$serialLine = $serial | Select-String -Pattern "elapsed_ms=" | Select-Object -Last 1
$serialMs = [int](($serialLine -split "elapsed_ms=")[1])

Write-Host "[3/3] Run parallel mode..."
$parallel = cargo run --quiet -- crypto-stats --input $Input --mode parallel --batch-size $BatchSize --top 10
$parallelLine = $parallel | Select-String -Pattern "elapsed_ms=" | Select-Object -Last 1
$parallelMs = [int](($parallelLine -split "elapsed_ms=")[1])

$speedup = [math]::Round($serialMs / [double]$parallelMs, 2)

Write-Host ""
Write-Host "Benchmark Result"
Write-Host "Input      : $Input"
Write-Host "Serial ms  : $serialMs"
Write-Host "Parallel ms: $parallelMs"
Write-Host "Speedup    : ${speedup}x"
