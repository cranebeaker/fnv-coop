@echo off
rem Host a co-op game through the launcher: a NEW GAME by default (the whole normal start:
rem Doc Mitchell, the name box, the Vit-o-Matic, tags, traits). Friends join any time.
rem
rem Copy this file next to nv-launcher.exe (the co-op package folder) and edit the lines below.
rem   NVL_DATA      the game's Data folder (empty: use the launcher's saved setting)
rem   NVL_NAME      your player name (empty: the saved setting)
rem   NVL_ADDRESS   the address put in the join code: your LAN IP for friends on your network,
rem                 your public IP for the internet (empty: the saved setting, else the LAN IP)
rem   NVL_CONTINUE  a campaign id to CONTINUE from its newest save (empty: a new game)
rem
rem The join code is printed in this window; send it to your friends. They run Join-Coop.cmd.
rem This window shows the game's co-op lines and ends with the game's exit code. The whole log
rem is in %APPDATA%\nv-rs-coop\logs.

set "NVL_DATA="
set "NVL_NAME="
set "NVL_ADDRESS="
set "NVL_CONTINUE="

set "ARGS=--start host"
if not "%NVL_DATA%"=="" set ARGS=%ARGS% --data "%NVL_DATA%"
if not "%NVL_NAME%"=="" set ARGS=%ARGS% --name "%NVL_NAME%"
if not "%NVL_ADDRESS%"=="" set ARGS=%ARGS% --address "%NVL_ADDRESS%"
if not "%NVL_CONTINUE%"=="" set ARGS=%ARGS% --continue %NVL_CONTINUE%

"%~dp0nv-launcher.exe" %ARGS%
set "NVL_EXIT=%ERRORLEVEL%"
if not "%NVL_EXIT%"=="0" (
  echo.
  echo The game ended with exit code %NVL_EXIT%. Run "nv-launcher.exe --diagnostics" to collect the logs.
)
pause
exit /b %NVL_EXIT%
