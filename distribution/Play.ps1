[CmdletBinding()]
param(
    [string]$DataPath = $env:NV_RS_DATA,
    [switch]$ValidateOnly
)
$ErrorActionPreference = 'Stop'
if (!$DataPath) { $DataPath = Read-Host 'Enter your Fallout New Vegas Data folder (containing FalloutNV.esm)' }
$DataPath = $DataPath.Trim().Trim('"')
if (!(Test-Path -LiteralPath (Join-Path $DataPath 'FalloutNV.esm') -PathType Leaf)) {
    throw 'That folder does not contain FalloutNV.esm. Supply -DataPath or set NV_RS_DATA.'
}
$DataPath = (Resolve-Path -LiteralPath $DataPath).Path
$executable = Join-Path $PSScriptRoot 'app/nv-viewer.exe'
if (!(Test-Path -LiteralPath $executable -PathType Leaf)) { throw 'app/nv-viewer.exe is missing. Extract the entire playtest ZIP.' }
# The game draws in its own window through Direct3D 9 and plays its sound through DirectSound.
$env:NV_D3D9 = 'real'
$env:NV_SOUND = 'out'
if ($ValidateOnly) {
    [pscustomobject]@{ Executable=$executable; Arguments=@($DataPath); NV_D3D9=$env:NV_D3D9; NV_SOUND=$env:NV_SOUND }
    return
}
if (Test-Path -LiteralPath (Join-Path $PSScriptRoot 'BUILD.txt')) { Get-Content -LiteralPath (Join-Path $PSScriptRoot 'BUILD.txt') }
& $executable $DataPath
exit $LASTEXITCODE
