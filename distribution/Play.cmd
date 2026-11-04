@echo off
setlocal DisableDelayedExpansion
rem Interactive launcher needs no PowerShell execution-policy change.
set "nvrs_data=%NV_RS_DATA%"
if not defined nvrs_data set /p "nvrs_data=Fallout New Vegas Data folder: "
if not defined nvrs_data exit /b 1
set "nvrs_data=%nvrs_data:"=%"
if not exist "%nvrs_data%\FalloutNV.esm" (
  echo FalloutNV.esm was not found in that folder.
  pause
  exit /b 1
)
if not exist "%~dp0app\nv-viewer.exe" (
  echo Extract the entire playtest ZIP before launching.
  pause
  exit /b 1
)
rem The game draws in its own window through Direct3D 9 and plays its sound through DirectSound.
set "NV_D3D9=real"
set "NV_SOUND=out"
type "%~dp0BUILD.txt"
"%~dp0app\nv-viewer.exe" "%nvrs_data%"
if errorlevel 1 pause
