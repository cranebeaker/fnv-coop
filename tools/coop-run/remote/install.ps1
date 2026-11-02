# Runs ON the test PC (over ssh). Unpacks an uploaded tools\coop-run zip into a new versioned
# folder under C:\fnv\coop-run and points current.txt at it. Old versions are left in place;
# nothing is ever deleted.
param([Parameter(Mandatory = $true)][string]$Zip, [Parameter(Mandatory = $true)][string]$Version)
Set-Location C:\fnv
$ErrorActionPreference = 'Stop'

if ($Version -notmatch '^[A-Za-z0-9_-]+$') { throw "bad version name" }
if ($Zip -notmatch '^C:\\fnv\\[A-Za-z0-9_.\\-]+\.zip$' -or $Zip.Contains('..')) { throw "zip must be under C:\fnv" }
if (-not (Test-Path -LiteralPath $Zip)) { throw "zip not found: $Zip" }

$base = 'C:\fnv\coop-run'
$dest = Join-Path $base ('v' + $Version)
if (Test-Path -LiteralPath $dest) { throw "version folder already exists: $dest" }
New-Item -ItemType Directory -Path $dest | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $base 'results') | Out-Null
Expand-Archive -LiteralPath $Zip -DestinationPath $dest
Set-Content -LiteralPath (Join-Path $base 'current.txt') -Value $dest
"installed $dest"
