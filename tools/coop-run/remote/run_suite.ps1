# Runs ON the test PC, inside the logged-in desktop session (start it with task.ps1).
# Runs the scenarios N times each through runner.py and writes everything under
# C:\fnv\coop-run\results\<Tag>. The marker file C:\fnv\coop-run\done-<Tag>.txt appears when it ends.
# The settings are read from C:\fnv\coop-run\run-<Tag>.json, uploaded before the start:
#   {"scenarios": "a,b", "n": 3, "python": "C:\\Program Files\\Python311\\python.exe"}
param([Parameter(Mandatory = $true)][string]$Tag)
Set-Location C:\fnv
$ErrorActionPreference = 'Stop'
$base = 'C:\fnv\coop-run'
$res = Join-Path $base ('results\' + $Tag)
$done = Join-Path $base ('done-' + $Tag + '.txt')
try {
    if ($Tag -notmatch '^[A-Za-z0-9_-]+$') { throw "bad tag" }
    $p = Get-Content -LiteralPath (Join-Path $base ('run-' + $Tag + '.json')) -Raw | ConvertFrom-Json
    $Scenarios = [string]$p.scenarios
    $N = [int]$p.n
    $Python = [string]$p.python
    $ver = (Get-Content -LiteralPath (Join-Path $base 'current.txt') -Raw).Trim()
    if (-not $ver.StartsWith('C:\fnv\coop-run\v')) { throw "current.txt points outside coop-run" }
    New-Item -ItemType Directory -Force -Path $res | Out-Null
    $files = @()
    foreach ($s in $Scenarios.Split(',')) {
        if ($s -notmatch '^[A-Za-z0-9_-]+$') { throw "bad scenario name $s" }
        $files += (Join-Path $ver ('scenarios\' + $s + '.json'))
    }
    $cfg = Join-Path $base 'coop-run.remote.json'
    $console = Join-Path $res 'console.txt'
    & $Python -I (Join-Path $ver 'runner.py') suite @files --config $cfg --out $res --n $N *> $console
    $code = $LASTEXITCODE
    $last = (Get-Content -LiteralPath $console -Tail 1)
    Set-Content -LiteralPath $done -Value ("exit=" + $code + " " + $last)
} catch {
    Set-Content -LiteralPath $done -Value ("exit=2 error: " + $_.Exception.Message)
}
