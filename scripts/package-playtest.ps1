[CmdletBinding()]
param([Parameter(Mandatory)][ValidatePattern('^[a-zA-Z0-9][a-zA-Z0-9._-]+$')][string]$Version)
$ErrorActionPreference='Stop'
$root=Split-Path $PSScriptRoot
Push-Location $root
try {
    $commit=git rev-parse HEAD
    if($LASTEXITCODE){throw 'Package from a committed Git checkout.'}
    $dirty=git status --porcelain --untracked-files=normal
    if($dirty){throw 'Commit or remove untracked changes before packaging.'}
    Push-Location viewer
    try {
        cargo build --release --locked
        if($LASTEXITCODE){throw 'Release build failed.'}
    } finally {Pop-Location}
    $name="nv-rs-$Version-windows-x64"
    $stage=Join-Path $root "dist/$name"
    $zip="$stage.zip"
    if((Test-Path -LiteralPath $stage) -or (Test-Path -LiteralPath $zip)){throw 'This package version already exists. Use a new version.'}
    $notice=Join-Path $root 'distribution/THIRD-PARTY-NOTICES.txt'
    if(!(Test-Path -LiteralPath $notice)){throw 'Dependency notices must be assembled before packaging.'}
    $licenseLock=(Get-Content -LiteralPath (Join-Path $root 'distribution/DEPENDENCY-LOCK.sha256') -Raw).Trim()
    if($licenseLock -ne (Get-FileHash -LiteralPath (Join-Path $root 'viewer/Cargo.lock')).Hash.ToLowerInvariant()) {
        throw 'Dependency lockfile changed. Refresh license notices and their lockfile hash before packaging.'
    }
    New-Item -ItemType Directory -Path (Join-Path $stage 'app') -Force | Out-Null
    Copy-Item -LiteralPath (Join-Path $root 'viewer/target/release/nv-viewer.exe') -Destination (Join-Path $stage 'app/nv-viewer.exe')
    foreach($file in @('Play.cmd','Play.ps1','README.txt','THIRD-PARTY-NOTICES.txt')) {
        Copy-Item -LiteralPath (Join-Path $root "distribution/$file") -Destination $stage
    }
    Copy-Item -LiteralPath (Join-Path $root 'distribution/licenses') -Destination $stage -Recurse
    foreach($file in @('LICENSE','LEGAL.md')) { Copy-Item -LiteralPath (Join-Path $root $file) -Destination $stage }
    $sha=(Get-FileHash -LiteralPath (Join-Path $stage 'app/nv-viewer.exe') -Algorithm SHA256).Hash
    @("nv-rs experimental playtest $Version","Source commit: $commit","Executable SHA256: $sha","Platform: Windows x64","Source: https://github.com/slaterain/nv-rs/tree/$commit") | Set-Content -LiteralPath (Join-Path $stage 'BUILD.txt') -Encoding utf8
    # Explicit input list above: no game data, saves, reports, PDBs or research files.
    Compress-Archive -LiteralPath $stage -DestinationPath $zip -CompressionLevel Optimal
    $zipHash=(Get-FileHash -LiteralPath $zip -Algorithm SHA256).Hash.ToLowerInvariant()
    "$zipHash  $name.zip" | Set-Content -LiteralPath "$zip.sha256" -Encoding ascii
    [pscustomobject]@{Zip=$zip;Checksum="$zip.sha256";Commit=$commit;ExecutableSHA256=$sha}
} finally {Pop-Location}
