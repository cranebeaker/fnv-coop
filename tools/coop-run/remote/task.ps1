# Runs ON the test PC (over ssh). Starts an uploaded script in the logged-in user's desktop
# session through a one-shot scheduled task (a game window needs a desktop; an ssh session
# has none), then removes the task it created. The script keeps running after the task entry
# is gone. No mouse or keyboard input is simulated anywhere.
param(
    [Parameter(Mandatory = $true)][string]$Script,
    [string]$Tag = '',
    [int]$Wait = 5
)
Set-Location C:\fnv
$ErrorActionPreference = 'Stop'

if ($Script -notmatch '^C:\\fnv\\[A-Za-z0-9_.\\-]+\.ps1$' -or $Script.Contains('..')) { throw "script must be a .ps1 under C:\fnv" }
if (-not (Test-Path -LiteralPath $Script)) { throw "script not found: $Script" }
if ($Tag -and $Tag -notmatch '^[A-Za-z0-9_-]+$') { throw "bad tag" }

# the user who is logged in on the console; the task must run as that user, interactively
$user = (Get-CimInstance Win32_ComputerSystem).UserName
if (-not $user) { throw "nobody is logged in on the test PC desktop; log in first" }

$tn = 'nvrs_coop_' + [IO.Path]::GetFileNameWithoutExtension($Script) + '_' + (Get-Date -Format 'HHmmss')
$tr = 'powershell -NoProfile -ExecutionPolicy Bypass -File ' + $Script
if ($Tag) { $tr = $tr + ' -Tag ' + $Tag }
schtasks /Create /TN $tn /TR $tr /SC ONCE /ST 23:59 /RU $user /IT /F | Out-Null
if ($LASTEXITCODE -ne 0) { throw "schtasks create failed" }
schtasks /Run /TN $tn | Out-Null
if ($LASTEXITCODE -ne 0) { throw "schtasks run failed" }
Start-Sleep -Seconds $Wait
# remove only the task this script made (name prefix checked above by construction)
schtasks /Delete /TN $tn /F | Out-Null
"launched $Script tag=$Tag as $user"
