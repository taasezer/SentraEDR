$ErrorActionPreference = "Stop"

$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")

function Invoke-Gate {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Name,

        [Parameter(Mandatory = $true)]
        [string]$Program,

        [Parameter(Mandatory = $true)]
        [string[]]$Arguments
    )

    Write-Output "==> $Name"
    & $Program @Arguments

    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
}

Push-Location $repoRoot
try {
    Invoke-Gate "format" "cargo" @("fmt", "--all", "--", "--check")
    Invoke-Gate "clippy" "cargo" @("clippy", "--workspace", "--all-targets", "--", "-D", "warnings")
    Invoke-Gate "workspace-tests" "cargo" @("test", "--workspace")
    Invoke-Gate "architecture-validation" "powershell" @("-ExecutionPolicy", "Bypass", "-File", "tools\validate-architecture.ps1")
    Invoke-Gate "agent-dry-run" "cargo" @("run", "-p", "sentra-agent")
} finally {
    Pop-Location
}
