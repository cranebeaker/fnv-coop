# Runs ON the test PC, inside the logged-in desktop session (start it with task.ps1).
# Like run_suite.ps1, but the scenario names come from a FILE, one per line, so a long list never
# has to travel on a command line (a scheduled-task command line is cut at 261 characters).
#   C:\fnv\coop-run\list-<Tag>.txt   scenario names, one per line ('#' starts a comment, blank lines ignored)
#   C:\fnv\coop-run\run-<Tag>.json   {"n": 3, "python": "C:\\Program Files\\Python311\\python.exe"}
#                                    (a "scenarios" key there is ignored here)
# Results go to C:\fnv\coop-run\results\<Tag>; the marker C:\fnv\coop-run\done-<Tag>.txt appears when it ends.
param([Parameter(Mandatory = $true)][string]$Tag)
Set-Location C:\fnv
$ErrorActionPreference = 'Stop'
$base = 'C:\fnv\coop-run'
$res = Join-Path $base ('results\' + $Tag)
$done = Join-Path $base ('done-' + $Tag + '.txt')
try {
    if ($Tag -notmatch '^[A-Za-z0-9_-]+$') { throw "bad tag" }
    $p = Get-Content -LiteralPath (Join-Path $base ('run-' + $Tag + '.json')) -Raw | ConvertFrom-Json
    $N = [int]$p.n
    $Python = [string]$p.python
    $ver = (Get-Content -LiteralPath (Join-Path $base 'current.txt') -Raw).Trim()
    if (-not $ver.StartsWith('C:\fnv\coop-run\v')) { throw "current.txt points outside coop-run" }
    New-Item -ItemType Directory -Force -Path $res | Out-Null
    $files = @()
    foreach ($raw in (Get-Content -LiteralPath (Join-Path $base ('list-' + $Tag + '.txt')))) {
        $s = $raw.Trim()
        if ($s -eq '' -or $s.StartsWith('#')) { continue }
        if ($s -notmatch '^[A-Za-z0-9_-]+$') { throw "bad scenario name $s" }
        $f = Join-Path $ver ('scenarios\' + $s + '.json')
        if (-not (Test-Path -LiteralPath $f)) { throw "No such scenario file: $f" }
        $files += $f
    }
    if ($files.Count -eq 0) { throw "the list file names no scenarios" }
    $cfg = Join-Path $base 'coop-run.remote.json'
    $console = Join-Path $res 'console.txt'
    & $Python -I (Join-Path $ver 'runner.py') suite @files --config $cfg --out $res --n $N *> $console
    $code = $LASTEXITCODE
    $last = (Get-Content -LiteralPath $console -Tail 1)
    Set-Content -LiteralPath $done -Value ("exit=" + $code + " " + $last)
} catch {
    Set-Content -LiteralPath $done -Value ("exit=2 error: " + $_.Exception.Message)
}
