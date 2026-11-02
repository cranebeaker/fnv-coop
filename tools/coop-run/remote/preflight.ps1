# Runs ON the test PC (over ssh). Read-only checks before a run. Exit 1 when something is wrong.
# It reports other nv-viewer processes but never touches them: a viewer we did not start
# might be somebody's live session.
param(
    [Parameter(Mandatory = $true)][string]$Config,
    [double]$MinFreeGB = 4,
    [switch]$AllowRunning
)
Set-Location C:\fnv
$ErrorActionPreference = 'Stop'
if ($Config -notmatch '^C:\\fnv\\[A-Za-z0-9_.\\-]+\.json$' -or $Config.Contains('..')) { throw "config must be a .json under C:\fnv" }

$bad = 0
if (-not (Test-Path -LiteralPath $Config)) { "MISSING config $Config"; exit 1 }
$cfg = Get-Content -LiteralPath $Config -Raw | ConvertFrom-Json
$viewer = $cfg.viewer
if ($viewer -is [array]) { $viewer = $viewer[0] }
if (-not (Test-Path -LiteralPath $viewer)) { "MISSING viewer exe $viewer"; $bad = 1 } else { "ok viewer $viewer" }
if (-not (Test-Path -LiteralPath $cfg.data)) { "MISSING data folder $($cfg.data)"; $bad = 1 } else { "ok data $($cfg.data)" }

$os = Get-CimInstance Win32_OperatingSystem
$freeGB = [math]::Round($os.FreePhysicalMemory / 1MB, 1)
if ($freeGB -lt $MinFreeGB) { "LOW RAM free ${freeGB} GB, need $MinFreeGB"; $bad = 1 } else { "ok free RAM ${freeGB} GB" }

$user = (Get-CimInstance Win32_ComputerSystem).UserName
if (-not $user) { "NO desktop user logged in"; $bad = 1 } else { "ok desktop user present" }

$running = @(Get-Process -Name 'nv-viewer' -ErrorAction SilentlyContinue)
if ($running.Count -gt 0) {
    "FOUND $($running.Count) nv-viewer process(es) already running (ids: $(($running | ForEach-Object { $_.Id }) -join ','))"
    if (-not $AllowRunning) { "refusing to start; close them yourself or pass -AllowRunning"; $bad = 1 }
} else { "ok no other nv-viewer running" }

exit $bad
