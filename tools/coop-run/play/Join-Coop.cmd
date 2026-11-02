@echo off
rem Join a co-op game through the launcher. You get YOUR OWN character: you make it with the same
rem menus as the single-player start (name, Vit-o-Matic, tags, traits) while the host plays on, in
rem the host's place if the host is still in the opening, else in a local room first; then you
rem arrive at the host's place. The host's world is never changed by you.
rem
rem Copy this file next to nv-launcher.exe (the co-op package folder) and edit the lines below.
rem   NVL_CODE      the join code the host sent (NVRS-...). Empty: you are asked.
rem   NVL_DATA      the game's Data folder (empty: use the launcher's saved setting)
rem   NVL_NAME      your player name (empty: the saved setting)
rem   NVL_CAMPAIGN  the campaign id the host showed (optional): keeps your saves with that
rem                 campaign, and a saved character of yours there is used instead of the menus

set "NVL_CODE="
set "NVL_DATA="
set "NVL_NAME="
set "NVL_CAMPAIGN="

if "%NVL_CODE%"=="" set /p NVL_CODE=Join code (NVRS-...):
set "ARGS=--start join --join %NVL_CODE%"
if not "%NVL_DATA%"=="" set ARGS=%ARGS% --data "%NVL_DATA%"
if not "%NVL_NAME%"=="" set ARGS=%ARGS% --name "%NVL_NAME%"
if not "%NVL_CAMPAIGN%"=="" set ARGS=%ARGS% --campaign %NVL_CAMPAIGN%

"%~dp0nv-launcher.exe" %ARGS%
set "NVL_EXIT=%ERRORLEVEL%"
if not "%NVL_EXIT%"=="0" (
  echo.
  echo The game ended with exit code %NVL_EXIT%. Run "nv-launcher.exe --diagnostics" to collect the logs.
)
pause
exit /b %NVL_EXIT%
