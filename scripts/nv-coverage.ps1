<#
.SYNOPSIS
CI-style single-player coverage report: `nv-coverage report` on the full game data.

.DESCRIPTION
Builds `nv-coverage` (no Bevy; the viewer is not built) and runs it against a
Data folder. With the full Data (meshes, textures, animations in the BSAs) the
load sweep runs the viewer's own loader on every interior cell and every
exterior grid square and reports missing and unreadable assets; with only the
ESMs it runs in records mode. Outputs go to -Out: nv-coverage.json (machine
readable) and nv-coverage.md (summary). The exit code is 0 when the report was
written, 2 when -FailBelow is given and the share of runnable main-line quest
stages is under it.

.EXAMPLE
powershell -File scripts\nv-coverage.ps1 -Data "D:\Steam\steamapps\common\Fallout New Vegas\Data"
powershell -File scripts\nv-coverage.ps1 -Data $env:FNV_ESM_DIR -Sweep off -FailBelow 95
powershell -File scripts\nv-coverage.ps1 -Data $env:FNV_ESM_DIR -Stride 4   # every 4th exterior square, quick
#>
param(
    [Parameter(Mandatory = $true)][string]$Data,
    [string]$Out = "nv-coverage-out",
    [ValidateSet("auto", "records", "full", "off")][string]$Sweep = "auto",
    [int]$Stride = 1,
    [double]$FailBelow = 0,
    [string]$TargetDir = ""
)
$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
Set-Location $root
if ($TargetDir) { $env:CARGO_TARGET_DIR = $TargetDir }
cargo build --release -p nv-coverage
if ($LASTEXITCODE -ne 0) { exit 1 }
$target = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $root "target" }
$exe = Join-Path $target "release\nv-coverage.exe"
$env:NV_RS_REPO = $root
& $exe report --data $Data --out $Out --sweep $Sweep --stride $Stride
if ($LASTEXITCODE -ne 0) { exit 1 }
$json = Get-Content (Join-Path $Out "nv-coverage.json") -Raw | ConvertFrom-Json
$main = $json.quest_walk.'main (base game)'
Write-Host ("main-line stages runnable: {0} of {1} ({2}%)" -f $main.stages_runnable, $main.stages, $main.percent_runnable)
if ($FailBelow -gt 0 -and $main.percent_runnable -lt $FailBelow) { exit 2 }
