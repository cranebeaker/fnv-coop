[CmdletBinding()]
param(
    [Parameter(Mandatory)][ValidatePattern('^[a-zA-Z0-9][a-zA-Z0-9._-]+$')][string]$Version,
    # Skip the cargo builds and use the existing release binaries (the checks still run).
    [switch]$NoBuild
)
# Builds release nv-viewer and nv-launcher and produces the co-op zip.
# Refuses to produce the zip if the staged files or file names contain the build machine's user
# name or profile folder name (read from the environment at run time, never written here).
$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot
Push-Location $root
try {
    $commit = git rev-parse HEAD
    if ($LASTEXITCODE) { throw 'Package from a committed Git checkout.' }
    $dirty = git status --porcelain --untracked-files=normal
    if ($dirty) { throw 'Commit or remove untracked changes before packaging.' }

    # Identifiers that must not appear anywhere in the package.
    $forbidden = @()
    foreach ($v in @($env:USERNAME, [Environment]::UserName, (Split-Path -Leaf $env:USERPROFILE))) {
        if ($v -and $v.Length -ge 3 -and ($forbidden -notcontains $v.ToLowerInvariant())) { $forbidden += $v.ToLowerInvariant() }
    }
    if (-not $forbidden) { throw 'Could not read the machine user name from the environment; cannot run the identifier check.' }

    if (-not $NoBuild) {
        # Keep build machine paths out of the binaries (panic messages, PDB paths).
        $remap = @()
        $remap += "--remap-path-prefix=$($env:USERPROFILE)=C:\build"
        $remap += "--remap-path-prefix=$root=C:\src\nv-rs"
        $cargoHome = if ($env:CARGO_HOME) { $env:CARGO_HOME } else { Join-Path $env:USERPROFILE '.cargo' }
        $remap += "--remap-path-prefix=$cargoHome=C:\cargo"
        $env:CARGO_ENCODED_RUSTFLAGS = ($remap -join [char]0x1f)
        Push-Location viewer
        try {
            cargo build --release --locked
            if ($LASTEXITCODE) { throw 'Viewer release build failed.' }
        } finally { Pop-Location }
        cargo build --release --locked -p nv-launcher
        if ($LASTEXITCODE) { throw 'Launcher release build failed.' }
        Remove-Item Env:CARGO_ENCODED_RUSTFLAGS -ErrorAction SilentlyContinue
    }
    $viewerExe = Join-Path $root 'viewer/target/release/nv-viewer.exe'
    $launcherExe = Join-Path $root 'target/release/nv-launcher.exe'
    foreach ($f in @($viewerExe, $launcherExe)) { if (!(Test-Path -LiteralPath $f)) { throw "Missing build output: $f" } }

    $name = "nv-rs-coop-$Version-windows-x64"
    $stage = Join-Path $root "dist/$name"
    $zip = "$stage.zip"
    if ((Test-Path -LiteralPath $stage) -or (Test-Path -LiteralPath $zip)) { throw 'This package version already exists. Use a new version.' }

    $notice = Join-Path $root 'distribution/THIRD-PARTY-NOTICES.txt'
    if (!(Test-Path -LiteralPath $notice)) { throw 'Dependency notices must be assembled before packaging.' }
    $licenseLock = (Get-Content -LiteralPath (Join-Path $root 'distribution/DEPENDENCY-LOCK.sha256') -Raw).Trim()
    if ($licenseLock -ne (Get-FileHash -LiteralPath (Join-Path $root 'viewer/Cargo.lock')).Hash.ToLowerInvariant()) {
        throw 'Dependency lockfile changed. Refresh license notices and their lockfile hash before packaging.'
    }

    New-Item -ItemType Directory -Path (Join-Path $stage 'app') -Force | Out-Null
    Copy-Item -LiteralPath $viewerExe -Destination (Join-Path $stage 'app/nv-viewer.exe')
    Copy-Item -LiteralPath $launcherExe -Destination (Join-Path $stage 'nv-launcher.exe')
    foreach ($file in @('Play-Coop.cmd', 'README-COOP.txt')) {
        Copy-Item -LiteralPath (Join-Path $root "distribution/coop/$file") -Destination $stage
    }
    Copy-Item -LiteralPath $notice -Destination $stage
    Copy-Item -LiteralPath (Join-Path $root 'distribution/licenses') -Destination $stage -Recurse
    foreach ($file in @('LICENSE', 'LEGAL.md')) { Copy-Item -LiteralPath (Join-Path $root $file) -Destination $stage }

    # The optional Steam DLL: shipped unmodified only when the pinned file is present
    # (third_party/steam, gitignored); otherwise the player supplies it or does without.
    $pinned = 'eb17909a76668cf9ae0b92a618a34a50f6c73d3a6787cb4dd8ce36a8b10bfb75'
    $dll = Join-Path $root 'third_party/steam/steam_api64.dll'
    $dllStatus = 'It is user-supplied: this package does not include it. To use Steam features, place your own steam_api64.dll next to nv-launcher.exe.'
    if (Test-Path -LiteralPath $dll) {
        $h = (Get-FileHash -LiteralPath $dll -Algorithm SHA256).Hash.ToLowerInvariant()
        if ($h -eq $pinned) {
            Copy-Item -LiteralPath $dll -Destination $stage
            $dllStatus = "It is shipped here unmodified (Steamworks.NET Standalone 2025.164.1, Windows-x64, SHA-256 $pinned)."
        } else {
            Write-Warning 'third_party/steam/steam_api64.dll does not match the pinned hash; not shipping it.'
        }
    }
    $tp = (Get-Content -LiteralPath (Join-Path $root 'distribution/coop/THIRD-PARTY.txt') -Raw).Replace('STEAM_DLL_STATUS_PLACEHOLDER', $dllStatus)
    Set-Content -LiteralPath (Join-Path $stage 'THIRD-PARTY.txt') -Value $tp -Encoding utf8

    $sha = @{}
    foreach ($f in @('nv-launcher.exe', 'app/nv-viewer.exe')) { $sha[$f] = (Get-FileHash -LiteralPath (Join-Path $stage $f) -Algorithm SHA256).Hash }
    @("nv-rs co-op package $Version", "Source commit: $commit", "nv-launcher.exe SHA256: $($sha['nv-launcher.exe'])", "nv-viewer.exe SHA256: $($sha['app/nv-viewer.exe'])", 'Platform: Windows x64') |
        Set-Content -LiteralPath (Join-Path $stage 'BUILD.txt') -Encoding utf8

    # Identifier check: file names, then contents (ASCII and UTF-16 forms, case-insensitive).
    $latin = [Text.Encoding]::GetEncoding(28591)
    $hits = @()
    foreach ($item in Get-ChildItem -LiteralPath $stage -Recurse -Force) {
        $rel = $item.FullName.Substring($stage.Length)
        foreach ($id in $forbidden) {
            if ($rel.ToLowerInvariant().Contains($id)) { $hits += "file name: $rel" }
        }
        if ($item.PSIsContainer) { continue }
        $bytes = [IO.File]::ReadAllBytes($item.FullName)
        $text = $latin.GetString($bytes).ToLowerInvariant()
        foreach ($id in $forbidden) {
            $wide = [string]::Join([string][char]0, [char[]]$id.ToCharArray())
            if ($text.Contains($id)) { $hits += "contents (ASCII): $rel" }
            if ($text.Contains($wide)) { $hits += "contents (UTF-16): $rel" }
        }
    }
    if ($hits.Count) {
        $hits | Sort-Object -Unique | ForEach-Object { Write-Host "FORBIDDEN IDENTIFIER FOUND  $_" -ForegroundColor Red }
        Remove-Item -LiteralPath $stage -Recurse -Force
        throw 'A forbidden identifier was found in the package. No zip was produced. Fix the source of the leak and package again.'
    }

    Compress-Archive -LiteralPath $stage -DestinationPath $zip -CompressionLevel Optimal
    # Entry names in the finished zip.
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $arc = [IO.Compression.ZipFile]::OpenRead($zip)
    try {
        foreach ($e in $arc.Entries) {
            foreach ($id in $forbidden) {
                if ($e.FullName.ToLowerInvariant().Contains($id)) { $arc.Dispose(); Remove-Item -LiteralPath $zip -Force; throw 'A forbidden identifier is in a zip entry name. Zip removed.' }
            }
        }
    } finally { $arc.Dispose() }
    $zipHash = (Get-FileHash -LiteralPath $zip -Algorithm SHA256).Hash.ToLowerInvariant()
    "$zipHash  $name.zip" | Set-Content -LiteralPath "$zip.sha256" -Encoding ascii
    [pscustomobject]@{ Zip = $zip; Checksum = "$zip.sha256"; ZipSHA256 = $zipHash; Commit = $commit }
} finally { Pop-Location }
