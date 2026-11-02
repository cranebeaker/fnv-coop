# Controller side. Zips tools\coop-run (no local config, no out\) and uploads it to the test PC
# as a new versioned folder under C:\fnv\coop-run. Returns nothing; prints the version name.
param([string]$Remote = '', [string]$Version = '')
. "$PSScriptRoot\lib.ps1"
$ErrorActionPreference = 'Stop'
if (-not $Remote) { $Remote = Get-RemoteName }
if (-not $Version) { $Version = Get-Date -Format 'yyyyMMdd_HHmmss' }

$src = Split-Path -Parent $PSScriptRoot
$stage = Join-Path $src ('out\push\' + $Version)
New-Item -ItemType Directory -Force -Path $stage | Out-Null
foreach ($item in @('*.py', 'README.md', 'config.example.json', 'scenarios', 'remote', 'controller')) {
    Copy-Item -Path (Join-Path $src $item) -Destination $stage -Recurse -Force
}
$zip = Join-Path $src ('out\push\' + $Version + '.zip')
Compress-Archive -Path (Join-Path $stage '*') -DestinationPath $zip -Force

# bootstrap: the installer itself is uploaded as a file, then run with -File
Send-RemoteFile $Remote (Join-Path $src 'remote\install.ps1') 'C:\fnv\coop-run-install.ps1'
Send-RemoteFile $Remote $zip ('C:\fnv\coop-run-' + $Version + '.zip')
$out = Invoke-RemoteFile $Remote 'C:\fnv\coop-run-install.ps1' @('-Zip', ('C:\fnv\coop-run-' + $Version + '.zip'), '-Version', $Version)
$out
if ($script:RemoteExit -ne 0) { throw "remote install failed" }
# the remote helper scripts now live inside the versioned folder; copy them to a stable place
foreach ($f in @('task.ps1', 'preflight.ps1', 'run_suite.ps1', 'run_list.ps1', 'status.ps1')) {
    Send-RemoteFile $Remote (Join-Path $src ('remote\' + $f)) ('C:\fnv\coop-run\' + $f)
}
$Version
