@echo off
setlocal
title FNV Co-Op
set "NVDIR=%~dp0"
echo.
echo  FNV Co-Op: play the story of Fallout: New Vegas together.
echo  One player hosts. The others join with the host's join code.
echo  If this is the first start on this PC, run Play.exe once alone first.
echo.
set "MODE="
set /p "MODE=Type H to host or J to join, then press Enter: "
set "NV_COOP_NAME=player"
set /p "NV_COOP_NAME=Your name (letters and digits): "
if /i "%MODE%"=="H" goto host
if /i "%MODE%"=="J" goto join
echo Not H or J. Start again.
pause
exit /b 1

:host
set "NV_COOP=host"
echo.
echo  Friends in your home network: press Enter at the next question.
echo  Friends over the internet: type your public address, and forward UDP port 27961 to this PC.
echo  Friends in a private network app (Tailscale, ZeroTier, Radmin VPN): type your address there.
set "NV_COOP_ADDRESS="
set /p "NV_COOP_ADDRESS=Address for the join code (or Enter): "
start "" "%NVDIR%Play.exe"
echo.
echo  The game starts. Waiting for the join code...
powershell -NoProfile -ExecutionPolicy Bypass -File "%NVDIR%app\tools\join-code.ps1"
echo.
pause
exit /b 0

:join
echo.
set "CODE="
set /p "CODE=Paste the host's join code (NVRS-...): "
if "%CODE%"=="" (
  echo No code. Start again.
  pause
  exit /b 1
)
set "NV_COOP=join:%CODE%"
start "" "%NVDIR%Play.exe"
echo.
echo  The game starts. Press New Game at the main menu. When you are in the world, you are moved
echo  to the host.
echo.
pause
exit /b 0
