$ErrorActionPreference = "Stop"

Write-Host "=========================================" -ForegroundColor Cyan
Write-Host "   SentraEDR Release Builder v1.0" -ForegroundColor Cyan
Write-Host "=========================================" -ForegroundColor Cyan
Write-Host ""

Write-Host "[1/3] Building the project in Release mode (Optimized for performance)..." -ForegroundColor Yellow
cargo build --release -p sentra-agent

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit $LASTEXITCODE
}

Write-Host "[2/3] Preparing the output directory..." -ForegroundColor Yellow
$OutputDir = "Release"
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir | Out-Null
}

$SourceExe = "target\release\sentra-agent.exe"
$TargetExe = "$OutputDir\SentraEDR.exe"

Write-Host "[3/3] Copying the executable..." -ForegroundColor Yellow
Copy-Item -Path $SourceExe -Destination $TargetExe -Force

Write-Host ""
Write-Host "=========================================" -ForegroundColor Green
Write-Host " BUILD SUCCESSFUL!" -ForegroundColor Green
Write-Host " Your SentraEDR executable is ready at:" -ForegroundColor White
Write-Host " -> $(Resolve-Path $TargetExe)" -ForegroundColor Magenta
Write-Host "=========================================" -ForegroundColor Green
Write-Host ""
Write-Host "Note: Since SentraEDR uses Kernel ETW providers, make sure to run" -ForegroundColor DarkGray
Write-Host "SentraEDR.exe as Administrator." -ForegroundColor DarkGray
