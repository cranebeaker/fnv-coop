@echo off
setlocal
if not exist "%~dp0nv-launcher.exe" (
  echo Extract the entire ZIP before launching. nv-launcher.exe was not found.
  pause
  exit /b 1
)
"%~dp0nv-launcher.exe" %*
if errorlevel 1 pause
endlocal
