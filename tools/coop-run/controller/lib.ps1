# Controller-side helpers (run on the PC you work at, they talk to the test PC over ssh/scp).
# Dot-source: . "$PSScriptRoot\lib.ps1"
#
# HARD SAFETY RULES, enforced here and checked by test_safety.py:
#   - remote writes only under C:\fnv, and every remote command starts with: cd /d C:\fnv
#   - remote scripts are uploaded as files and run with -File; never a hand-escaped one-liner
#   - nothing here deletes remotely, kills processes, or sends mouse or keyboard input
#   - arguments passed to remote scripts are restricted to a small safe character set

Set-StrictMode -Version Latest

function Get-RemoteName {
    if ($env:COOPRUN_REMOTE) { return $env:COOPRUN_REMOTE }
    return 'dino'
}

function Assert-RemotePath([string]$Path) {
    if ($Path -notmatch '^C:\\fnv\\[A-Za-z0-9_.\\-]+$' -or $Path.Contains('..')) {
        throw "refusing remote path outside C:\fnv: $Path"
    }
}

function Assert-SafeArg([string]$Arg) {
    if ($Arg -notmatch '^[A-Za-z0-9_.,:\\/=-]+$') {
        throw "refusing remote argument: $Arg"
    }
}

# Runs an uploaded .ps1 on the remote PC (ssh session, no desktop). Returns the output lines.
# The exit code of the remote script is left in $script:RemoteExit.
function Invoke-RemoteFile([string]$Remote, [string]$File, [string[]]$Arguments = @()) {
    Assert-RemotePath $File
    if ($File -notmatch '\.ps1$') { throw "remote file must be a .ps1: $File" }
    foreach ($a in $Arguments) { Assert-SafeArg $a }
    $cmd = 'cd /d C:\fnv && powershell -NoProfile -ExecutionPolicy Bypass -File ' + $File
    if ($Arguments.Count -gt 0) { $cmd = $cmd + ' ' + ($Arguments -join ' ') }
    $out = & ssh $Remote $cmd
    $script:RemoteExit = $LASTEXITCODE
    return $out
}

function Send-RemoteFile([string]$Remote, [string]$Local, [string]$RemotePath) {
    Assert-RemotePath $RemotePath
    if (-not (Test-Path -LiteralPath $Local)) { throw "local file missing: $Local" }
    & scp $Local ($Remote + ':' + ($RemotePath -replace '\\', '/'))
    if ($LASTEXITCODE -ne 0) { throw "scp failed for $Local" }
}

# Copies a remote folder (under C:\fnv) into an existing local folder. Copy only.
function Receive-RemoteDir([string]$Remote, [string]$RemotePath, [string]$LocalDir) {
    Assert-RemotePath $RemotePath
    & scp -r ($Remote + ':' + ($RemotePath -replace '\\', '/')) $LocalDir
    if ($LASTEXITCODE -ne 0) { throw "scp -r failed for $RemotePath" }
}
