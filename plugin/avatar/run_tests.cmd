@echo off
rem One-command puppet driver tests (plugin/avatar). Standalone build in build\nvav of this checkout (Ninja, Release, 2 jobs), then
rem   1. group table check (python) : plugin\avatar\avatar_groups.h equals tools/animcheck/animgroups_table.py
rem   2. avatar_tests               : the suite, then the whole suite once per mutation twin (each twin MUST fail)
rem   3. avatar_live_tests          : the driver on top of the real synccore/live InterpBuffer (when live.cpp exists)
rem   4. proof script selftest      : tools/avatar/proof_puppet.py --selftest (fake bridge, never touches a game)
rem Takes the repo-wide compile lock out\locks\build.lock (atomic mkdir) only while compiling.
rem Prints "avatar: ALL OK" when everything passes.
setlocal enabledelayedexpansion
set "HERE=%~dp0"
set "ROOT=%HERE%..\.."
rem per checkout: one %TEMP% tree shared by two worktrees fails CMake's source check
set "BLD=%ROOT%\build\nvav"
set "LOCKS=%ROOT%\out\locks"
set "LOCK=%LOCKS%\build.lock"
set "CXX="
set "CC="
set "CMAKE_TOOLCHAIN_FILE="
set "CMAKE_GENERATOR="
set "CMAKE_BUILD_PARALLEL_LEVEL=2"
set "PATH=%PATH%;C:\Program Files (x86)\Microsoft Visual Studio\Installer"
call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat" >nul || exit /b 1
set "PATH=C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja;%PATH%"
if not exist "%LOCKS%" mkdir "%LOCKS%"

cmake -S "%HERE%." -B "%BLD%" -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=cl >nul || exit /b 1
:lock
mkdir "%LOCK%" 2>nul
if errorlevel 1 (
  ping -n 16 127.0.0.1 >nul
  goto lock
)
cmake --build "%BLD%"
set BRC=%ERRORLEVEL%
rmdir "%LOCK%" 2>nul
if not "%BRC%"=="0" exit /b %BRC%

python -I "%ROOT%\tools\avatar\check_groups.py" || exit /b 1
python -I "%ROOT%\tools\avatar\check_groups.py" --mutant >nul
if "%ERRORLEVEL%"=="0" (
  echo avatar: the group checker mutant SURVIVED
  exit /b 1
)

"%BLD%\avatar_tests\avatar_tests.exe" || exit /b 1

if exist "%BLD%\avatar_tests\avatar_live_tests.exe" (
  "%BLD%\avatar_tests\avatar_live_tests.exe" || exit /b 1
) else (
  echo avatar: live integration test not built, synccore/live/live.cpp missing
  exit /b 1
)

python -I "%ROOT%\tools\avatar\proof_puppet.py" --selftest || exit /b 1
echo avatar: ALL OK
exit /b 0
