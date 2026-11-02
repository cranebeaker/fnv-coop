# Runs ON the test PC (over ssh). Prints "DONE exit=N ..." once the run's marker file exists,
# otherwise "PENDING". Read-only.
param([Parameter(Mandatory = $true)][string]$Tag)
Set-Location C:\fnv
if ($Tag -notmatch '^[A-Za-z0-9_-]+$') { "BAD TAG"; exit 2 }
$done = 'C:\fnv\coop-run\done-' + $Tag + '.txt'
if (Test-Path -LiteralPath $done) {
    "DONE " + (Get-Content -LiteralPath $done -Raw).Trim()
} else {
    "PENDING"
}
