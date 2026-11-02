# Controller side. Runs scenarios on the second test PC the safe way and fetches the results:
#   1. push the harness (versioned folder under C:\fnv\coop-run, never overwritten)
#   2. upload a generated remote config and run settings (viewer exe, Data folder) as files
#   3. preflight (read-only): viewer and Data exist, RAM, desktop user, no foreign viewer running
#   4. start run_suite.ps1 inside the logged-in desktop session via a scheduled task
#   5. poll for the done marker, then copy the results folder back to out\remote-<tag>
# The test PC is reached with ssh/scp; this script never deletes anything there and never
# stops a process there (the runner on that PC stops only the viewers it started).
param(
    [Parameter(Mandatory = $true)][string]$Scenarios,
    [string]$Remote = '',
    [int]$N = 3,
    [string]$Viewer = 'C:\fnv\nv-rs\target\release\nv-viewer.exe',
    [string]$Data = 'C:\fnv\Fallout New Vegas\Data',
    [string]$Python = 'C:\Program Files\Python311\python.exe',
    [int]$MaxMinutes = 60,
    [int]$PollSeconds = 15,
    [switch]$SkipPush
)
. "$PSScriptRoot\lib.ps1"
$ErrorActionPreference = 'Stop'
if (-not $Remote) { $Remote = Get-RemoteName }
$src = Split-Path -Parent $PSScriptRoot
if ($Scenarios -notmatch '^[A-Za-z0-9_,-]+$') { throw "Scenarios is a comma list of scenario names (no spaces)" }
Assert-RemotePath $Viewer
if (-not $Data.StartsWith('C:\fnv\') -or $Data.Contains('..')) { throw "Data must be under C:\fnv" }

# 1. push
if (-not $SkipPush) {
    $pushOut = & "$PSScriptRoot\push.ps1" -Remote $Remote
    "pushed version: " + ($pushOut | Select-Object -Last 1)
}
$tag = Get-Date -Format 'yyyyMMdd_HHmmss'

# 2. remote config and run settings, generated here, uploaded as files
$outDir = Join-Path $src 'out'
New-Item -ItemType Directory -Force -Path $outDir | Out-Null
$cfgLocal = Join-Path $outDir ('remote-config-' + $tag + '.json')
$cfg = [ordered]@{
    viewer = $Viewer; data = $Data; out_dir = 'C:\fnv\coop-run\results'; base_port = 27960
    launch_timeout_s = 180; grace_s = 25; background = $true; screen_size = '640,360'
}
($cfg | ConvertTo-Json) | Set-Content -LiteralPath $cfgLocal -Encoding UTF8
Send-RemoteFile $Remote $cfgLocal 'C:\fnv\coop-run\coop-run.remote.json'
$runLocal = Join-Path $outDir ('run-' + $tag + '.json')
([ordered]@{ scenarios = $Scenarios; n = $N; python = $Python } | ConvertTo-Json) | Set-Content -LiteralPath $runLocal -Encoding UTF8
Send-RemoteFile $Remote $runLocal ('C:\fnv\coop-run\run-' + $tag + '.json')

# 3. preflight
$pf = Invoke-RemoteFile $Remote 'C:\fnv\coop-run\preflight.ps1' @('-Config', 'C:\fnv\coop-run\coop-run.remote.json')
$pf
if ($script:RemoteExit -ne 0) { throw "preflight failed, nothing was started" }

# 4. start in the desktop session
$o = Invoke-RemoteFile $Remote 'C:\fnv\coop-run\task.ps1' @('-Script', 'C:\fnv\coop-run\run_suite.ps1', '-Tag', $tag, '-Wait', '5')
$o
if ($script:RemoteExit -ne 0) { throw "could not start the run" }

# 5. wait, fetch
$deadline = (Get-Date).AddMinutes($MaxMinutes)
$state = 'PENDING'
while ((Get-Date) -lt $deadline) {
    Start-Sleep -Seconds $PollSeconds
    $s = Invoke-RemoteFile $Remote 'C:\fnv\coop-run\status.ps1' @('-Tag', $tag)
    $state = [string]($s | Select-Object -Last 1)
    if ($state -like 'DONE*') { break }
}
$dest = Join-Path $outDir ('remote-' + $tag)
New-Item -ItemType Directory -Force -Path $dest | Out-Null
if ($state -notlike 'DONE*') {
    "TIMED OUT waiting for the run; it may still be going on the test PC (tag $tag). Fetching what exists."
}
Receive-RemoteDir $Remote ('C:\fnv\coop-run\results\' + $tag) $dest
"results copied to $dest (each run folder has verdict.json)"
$state
if ($state -like 'DONE exit=0*') { exit 0 } else { exit 1 }
