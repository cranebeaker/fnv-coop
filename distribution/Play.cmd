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
echo 1. Explore Doc Mitchell's house
echo 2. Start the experimental opening
echo 3. Explore Goodsprings
choice /c 123 /n /m "Choose 1, 2 or 3: "
set "nvrs_choice=%errorlevel%"
if not exist "%~dp0userdata" mkdir "%~dp0userdata"
cd /d "%~dp0userdata"
type "%~dp0BUILD.txt"
if "%nvrs_choice%"=="3" "%~dp0app\nv-viewer.exe" "%nvrs_data%" Goodsprings --official
if "%nvrs_choice%"=="2" "%~dp0app\nv-viewer.exe" "%nvrs_data%" --new-game --official
if "%nvrs_choice%"=="1" "%~dp0app\nv-viewer.exe" "%nvrs_data%" GSDocMitchellHouse --official
if errorlevel 1 pause
