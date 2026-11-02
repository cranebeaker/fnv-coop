# Runs the DLC data pass (docs/DEAD_MONEY.md) against your own game files and
# writes every result to one text file for review. Nothing is changed in the
# game folder, and nothing is uploaded.
#
# Usage, from the repository folder:
#   powershell -ExecutionPolicy Bypass -File scripts\dlc-data-pass.ps1 `
#       -Game "D:\SteamLibrary\steamapps\common\Fallout New Vegas"
#
# The output (dlc-data-pass.txt by default) holds editor IDs, counts and
# script function names. Never commit it: share only the counts, editor IDs
# and conclusions drawn from it.

param(
    [Parameter(Mandatory = $true)][string]$Game,
    [string[]]$Plugins = @("DeadMoney.esm", "HonestHearts.esm", "OldWorldBlues.esm", "LonesomeRoad.esm"),
    [string]$Out = "dlc-data-pass.txt"
)

# Continue: in Windows PowerShell 5.1, "Stop" would abort on any line nvinspect
# writes to stderr (warnings), which this script records instead.
$ErrorActionPreference = "Continue"
$data = Join-Path $Game "Data"
if (-not (Test-Path (Join-Path $data "FalloutNV.esm"))) {
    throw "No FalloutNV.esm in ${data}: pass the folder that contains Data."
}

cargo build --release -p nvinspect
if ($LASTEXITCODE -ne 0) { throw "nvinspect did not build." }
$nvinspect = Join-Path (Get-Location) "target\release\nvinspect.exe"

function Section([string]$title, [string[]]$arguments) {
    "`n===== $title =====" | Out-File $Out -Append -Encoding utf8
    & $nvinspect @arguments 2>&1 | Out-File $Out -Append -Encoding utf8
}

"DLC data pass, $(Get-Date -Format s)" | Out-File $Out -Encoding utf8
Section "load order" @($data, "info")
foreach ($p in $Plugins) {
    $file = Join-Path $data $p
    if (-not (Test-Path $file)) {
        "`n===== ${p}: not installed =====" | Out-File $Out -Append -Encoding utf8
        continue
    }
    foreach ($command in @(@("types"), @("list", "QUST"), @("cells"), @("worlds"), @("functions"), @("scripts"))) {
        Section "$p $($command -join ' ')" (@($file) + $command)
    }
}
Write-Host "Done: $Out"
