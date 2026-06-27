$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$cargoFiles = Get-ChildItem -Path (Join-Path $repoRoot "crates") -Filter Cargo.toml -Recurse

$forbidden = @(
    @{ Crate = "shared-models"; Pattern = 'path\s*=\s*"\.\./(shared-ipc|sentra-agent)"'; Message = "shared-models must not depend on other Sentra crates" },
    @{ Crate = "shared-ipc"; Pattern = 'path\s*=\s*"\.\./sentra-agent"'; Message = "shared-ipc must not depend on sentra-agent" },
    @{ Crate = "shared-ipc"; Pattern = 'path\s*=\s*"\.\./engine-'; Message = "shared-ipc must not depend on engine crates" },
    @{ Crate = "engine-etw"; Pattern = 'path\s*=\s*"\.\./(sentra-agent|sentra-ui)"'; Message = "engine-etw must not depend on agent or UI crates" },
    @{ Crate = "engine-etw"; Pattern = 'path\s*=\s*"\.\./engine-'; Message = "engine-etw must not depend on peer engine crates" },
    @{ Crate = "engine-process"; Pattern = 'path\s*=\s*"\.\./(sentra-agent|sentra-ui)"'; Message = "engine-process must not depend on agent or UI crates" },
    @{ Crate = "engine-process"; Pattern = 'path\s*=\s*"\.\./engine-'; Message = "engine-process must not depend on peer engine crates" },
    @{ Crate = "engine-persistence"; Pattern = 'path\s*=\s*"\.\./(sentra-agent|sentra-ui)"'; Message = "engine-persistence must not depend on agent or UI crates" },
    @{ Crate = "engine-persistence"; Pattern = 'path\s*=\s*"\.\./engine-'; Message = "engine-persistence must not depend on peer engine crates" },
    @{ Crate = "engine-network"; Pattern = 'path\s*=\s*"\.\./(sentra-agent|sentra-ui)"'; Message = "engine-network must not depend on agent or UI crates" },
    @{ Crate = "engine-network"; Pattern = 'path\s*=\s*"\.\./engine-'; Message = "engine-network must not depend on peer engine crates" },
    @{ Crate = "engine-detection"; Pattern = 'path\s*=\s*"\.\./(sentra-agent|sentra-ui)"'; Message = "engine-detection must not depend on agent or UI crates" },
    @{ Crate = "engine-detection"; Pattern = 'path\s*=\s*"\.\./engine-'; Message = "engine-detection must not depend on peer engine crates" },
    @{ Crate = "sentra-agent"; Pattern = 'path\s*=\s*"\.\./sentra-ui"'; Message = "sentra-agent must not depend on UI crates" }
)

$violations = New-Object System.Collections.Generic.List[string]

foreach ($cargoFile in $cargoFiles) {
    $content = Get-Content -Raw -LiteralPath $cargoFile.FullName
    $crateName = Select-String -InputObject $content -Pattern 'name\s*=\s*"([^"]+)"' | Select-Object -First 1
    if ($null -eq $crateName) {
        $violations.Add("Could not find crate name in $($cargoFile.FullName)")
        continue
    }

    $name = $crateName.Matches[0].Groups[1].Value
    foreach ($rule in $forbidden) {
        if ($name -eq $rule.Crate -and $content -match $rule.Pattern) {
            $violations.Add("$name violates boundary: $($rule.Message)")
        }
    }
}

if ($violations.Count -gt 0) {
    $violations | ForEach-Object { Write-Error $_ }
    exit 1
}

Write-Output "Architecture dependency validation passed."
