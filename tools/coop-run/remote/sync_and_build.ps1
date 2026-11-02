# Runs ON the test PC (over ssh, always with -File). Applies a git bundle to the checkout under
# C:\fnv, builds the viewer with the fast profile and prints compile errors in a compact form.
#
#   powershell -NoProfile -File C:\fnv\coop-run\sync_and_build.ps1 -Bundle C:\fnv\drop\x.bundle -Ref worktree-coop
#
# What it never does: delete anything (no Remove-Item anywhere), write outside C:\fnv, discard
# local edits (the checkout is a plain `git checkout --detach`, which stops if they would be
# overwritten), or touch a running viewer.
#
# Exit code 0 build ok, 1 build failed or sync failed, 2 bad arguments. The last line is
# `RESULT: PASS build ...` or `RESULT: FAIL ...` like the rest of coop-run.
param(
    [Parameter(Mandatory = $true)][string]$Bundle,
    [string]$Ref = 'HEAD',
    [string]$Repo = 'C:\fnv\nv-rs',
    [string]$TargetDir = 'C:\fnv\target-live',
    [string]$BuildProfile = 'play-quick',
    [switch]$Lld,
    [switch]$SyncOnly,
    [int]$MaxErrors = 15
)
Set-Location C:\fnv
$ErrorActionPreference = 'Stop'

function Test-UnderFnv([string]$p) { return ($p -match '^C:\\fnv\\[A-Za-z0-9_.\\-]+$') -and (-not $p.Contains('..')) }
foreach ($p in @($Bundle, $Repo, $TargetDir)) {
    if (-not (Test-UnderFnv $p)) { "RESULT: FAIL args: $p must be a path under C:\fnv"; exit 2 }
}
if ($Ref -notmatch '^[A-Za-z0-9_./-]+$') { "RESULT: FAIL args: bad ref"; exit 2 }
if ($BuildProfile -notmatch '^[A-Za-z0-9_-]+$') { "RESULT: FAIL args: bad profile"; exit 2 }
if (-not (Test-Path -LiteralPath $Bundle)) { "RESULT: FAIL sync: bundle not found: $Bundle"; exit 1 }
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"

$clock = [System.Diagnostics.Stopwatch]::StartNew()
# ---- sync ----
if (-not (Test-Path -LiteralPath (Join-Path $Repo '.git'))) {
    if (Test-Path -LiteralPath $Repo) { "RESULT: FAIL sync: $Repo exists but is not a git checkout; pick another -Repo"; exit 1 }
    git clone --quiet --no-checkout $Bundle $Repo 2>&1 | ForEach-Object { "$_" }
    if ($LASTEXITCODE -ne 0) { "RESULT: FAIL sync: git clone failed"; exit 1 }
}
git -C $Repo fetch --quiet $Bundle "+refs/heads/*:refs/remotes/bundle/*" 2>&1 | ForEach-Object { "$_" }
if ($LASTEXITCODE -ne 0) { "RESULT: FAIL sync: git fetch from the bundle failed (an incremental bundle needs its base commits here)"; exit 1 }
$sha = (git -C $Repo rev-parse --verify --quiet "refs/remotes/bundle/$Ref^{commit}") 2>$null
if (-not $sha) { $sha = (git -C $Repo rev-parse --verify --quiet "$Ref^{commit}") 2>$null }
if (-not $sha) { "RESULT: FAIL sync: ref $Ref is not in the bundle"; exit 1 }
$dirty = git -C $Repo status --porcelain --untracked-files=no
if ($dirty) { "RESULT: FAIL sync: $Repo has local edits; commit or stash them there first:"; $dirty | Select-Object -First 10; exit 1 }
git -C $Repo checkout --quiet --detach $sha 2>&1 | ForEach-Object { "$_" }
if ($LASTEXITCODE -ne 0) { "RESULT: FAIL sync: checkout of $sha failed"; exit 1 }
"synced $($sha.Substring(0,10)) in $([math]::Round($clock.Elapsed.TotalSeconds,1)) s"
if ($SyncOnly) { "RESULT: PASS sync $($sha.Substring(0,10))"; exit 0 }

# ---- build ----
New-Item -ItemType Directory -Force -Path $TargetDir | Out-Null
$env:CARGO_TARGET_DIR = $TargetDir
$env:CARGO_INCREMENTAL = '1'
$env:CARGO_TERM_COLOR = 'never'
if ($Lld) { $env:CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER = 'rust-lld.exe' }
$log = Join-Path $TargetDir 'last-build.log'
$buildClock = [System.Diagnostics.Stopwatch]::StartNew()
Push-Location (Join-Path $Repo 'viewer')
try {
    $ErrorActionPreference = 'Continue'
    $lines = & cargo build --profile $BuildProfile --message-format=short 2>&1 | ForEach-Object { "$_" }
    $code = $LASTEXITCODE
} finally { Pop-Location }
$secs = [math]::Round($buildClock.Elapsed.TotalSeconds, 1)
Set-Content -LiteralPath $log -Value $lines
$errs = @($lines | Where-Object { $_ -match ': error(\[E\d+\])?:' -or $_ -match '^error(\[E\d+\])?:' } |
    Where-Object { $_ -notmatch '^error: could not compile' -and $_ -notmatch '^error: aborting' })
$warns = @($lines | Where-Object { $_ -match ': warning' }).Count
if ($code -ne 0) {
    "BUILD FAILED after $secs s: $($errs.Count) error(s), $warns warning(s); full log $log"
    if ($errs.Count -eq 0) { $lines | Select-Object -Last 8 }
    $errs | Select-Object -First $MaxErrors | ForEach-Object { "  $_" }
    if ($errs.Count -gt $MaxErrors) { "  ... and $($errs.Count - $MaxErrors) more" }
    "RESULT: FAIL build $BuildProfile $secs s ($($errs.Count) errors)"
    exit 1
}
$exe = Join-Path $TargetDir "$BuildProfile\nv-viewer.exe"
"RESULT: PASS build $BuildProfile $secs s, $warns warning(s), $exe"
exit 0
