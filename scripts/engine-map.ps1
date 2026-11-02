# Rebuilds research/engine-map/engine_map.tsv, research/engine-map/frame.tsv and
# docs/LEDGER.md from the private inputs (research/engine-map/README.md). Run
# from the repository root:
#
#   .\scripts\engine-map.ps1                 # reuse the named project copy
#   .\scripts\engine-map.ps1 -FreshProject   # copy the source project again first
#
# Private inputs and outputs (never committed):
#   $NvRe\ghidra_mcp            analyzed PC project (copied, never written)
#   $NvRe\ghidra-phase0         the private copy this script writes to
#   $XboxDir\Fallout_Release_MemDebug.pdb / .exe   Xbox 360 prototype (ADR-0002)
#   $NvRe\work\phase0           intermediate files
# Only the ledger step (scripts/ledger) is needed to refresh docs/LEDGER.md
# after Rust changes; it reads the committed map.
param(
    [switch]$FreshProject,
    [string]$NvRe = (Join-Path $env:USERPROFILE 'nv-re'),
    [string]$SourceProject = (Join-Path (Join-Path $env:USERPROFILE 'nv-re') 'ghidra_mcp'),
    [string]$XboxDir = (Join-Path $env:USERPROFILE 'OneDrive\Desktop\FalloutNV')
)
$ErrorActionPreference = 'Stop'
$Repo = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$Headless = Join-Path $NvRe 'tools\ghidra_12.1.4_PUBLIC\support\analyzeHeadless.bat'
$env:JAVA_HOME = Join-Path $NvRe 'tools\jdk-21.0.12.1+1'
$env:GHIDRA_HEADLESS_MAXMEM = '16G'
$Project = Join-Path $NvRe 'ghidra-phase0'
$Work = Join-Path $NvRe 'work\phase0'
$Scripts = Join-Path $Repo 'research\ghidra'
$Pdb = Join-Path $XboxDir 'Fallout_Release_MemDebug.pdb'
$XboxExe = Join-Path $XboxDir 'Fallout_Release_MemDebug.exe'
New-Item -ItemType Directory -Force $Work | Out-Null

# analyzeHeadless splits arguments at '=' when called directly from
# PowerShell, and exits with 0 even when a script fails; so every call goes
# through a one-off .cmd file with quoted arguments, and the log is checked
# (research/ghidra/README.md, "Commands for the Windows machine").
function Invoke-Ghidra {
    param([string]$Log, [switch]$Write, [string]$Script, [string[]]$ScriptArgs)
    $q = { param($s) '"' + $s + '"' }
    $line = "call $(& $q $Headless) $(& $q $Project) FalloutNV -process FalloutNV_unpacked.exe -noanalysis"
    if (-not $Write) { $line += ' -readOnly' }
    $line += " -scriptPath $(& $q $Scripts) -postScript $(& $q $Script)"
    foreach ($a in $ScriptArgs) { $line += ' ' + (& $q $a) }
    $cmd = Join-Path $Work 'run.cmd'
    Set-Content -Encoding ascii $cmd "@echo off`r`n$line"
    cmd /c "`"$cmd`"" *> $Log
    $code = $LASTEXITCODE
    Remove-Item $cmd
    if ($code -ne 0 -or
        (Select-String -Path $Log -Quiet -Pattern 'REPORT SCRIPT ERROR|Abort due to Headless analyzer error|Open failed|Error during analysis') -or
        -not (Select-String -Path $Log -Quiet -Pattern '^INFO  SCRIPT: ')) {
        Get-Content $Log -Tail 30
        throw "analyzeHeadless failed for $Script (exit $code); see $Log"
    }
}

function Invoke-Checked {
    param([string]$What, [scriptblock]$Block)
    & $Block
    if ($LASTEXITCODE -ne 0) { throw "$What failed (exit $LASTEXITCODE)" }
}

if ($FreshProject -or -not (Test-Path $Project)) {
    Write-Host "Copying $SourceProject to $Project"
    Remove-Item $Project -Recurse -Force -ErrorAction SilentlyContinue
    Copy-Item $SourceProject $Project -Recurse
    Get-ChildItem $Project -Recurse -Force -Include '*.lock', '*.lock~' | Remove-Item -Force

    # Functions at vtable slot targets (and their callees) that analysis missed.
    Invoke-Ghidra "$Work\map0.log" -Script NvEngineMap.java -ScriptArgs @("out=$Work\pc0")
    $missing = foreach ($row in (Get-Content "$Work\pc0\vtables.tsv" | Select-Object -Skip 1)) {
        $f = $row -split "`t"
        $slots = $f[4] -split ','
        for ($i = 0; $i -lt $slots.Count; $i++) { if ($f[5][$i] -eq '0') { $slots[$i] } }
    }
    $missing | Sort-Object -Unique | Set-Content -Encoding ascii "$Work\vt_missing.txt"
    Invoke-Ghidra "$Work\create.log" -Write -Script NvCreateFunctions.java `
        -ScriptArgs @("addrs=$Work\vt_missing.txt", "report=$Work\create.csv")
}

Invoke-Ghidra "$Work\map.log" -Script NvEngineMap.java -ScriptArgs @("out=$Work\pc")

$Cargo = Join-Path $Repo 'research\engine-map\Cargo.toml'
Invoke-Checked 'xbox' { cargo run --release -q --manifest-path $Cargo --bin xbox -- $Pdb $XboxExe "$Work\xb" }
Invoke-Checked 'match' { cargo run --release -q --manifest-path $Cargo --bin match -- "$Work\pc" "$Work\xb" "$Work\match" }
Invoke-Checked 'map' { cargo run --release -q --manifest-path $Cargo --bin map -- "$Work\pc" "$Work\xb" "$Work\match" (Join-Path $Repo 'research\engine-map\engine_map.tsv') }
# The frame tree (Main::OnIdle to depth 3); the root's alignment for review stays private.
Invoke-Checked 'frame' { cargo run --release -q --manifest-path $Cargo --bin frame -- "$Work\pc" "$Work\xb" (Join-Path $Repo 'research\engine-map\engine_map.tsv') (Join-Path $Repo 'research\engine-map\frame.tsv') "$Work\frame-root.tsv" }

# Names into the private copy: dry run first (one rejected row aborts it).
Invoke-Ghidra "$Work\names-dry.log" -Script NvImportNameMap.java `
    -ScriptArgs @("csv=$Work\match\names.csv", 'dry=1', "report=$Work\names-dry.csv")
Invoke-Ghidra "$Work\names.log" -Write -Script NvImportNameMap.java `
    -ScriptArgs @("csv=$Work\match\names.csv", "report=$Work\names-applied.csv")

Invoke-Checked 'ledger' { cargo run --release -q --manifest-path (Join-Path $Repo 'scripts\ledger\Cargo.toml') -- --tsv "$Work\ledger.tsv" }
Get-Content "$Work\match\report.txt", "$Work\match\map-report.txt"
