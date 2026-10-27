@echo off
rem plugin/actors: one-command tests. Builds (Win32/x86, Release, 2 jobs) into build\actors, then
rem   the real suite must report "0 failed"; every mutation twin (NVA_MUT=n) must FAIL it (failed checks or a crash).
rem Takes the repo-wide compile lock (out\locks\build.lock, atomic mkdir) only while compiling.  Prints ALL OK when everything holds.
rem Usage: plugin\actors\run_tests.cmd
setlocal enabledelayedexpansion
call "%~dp0..\..\tools\build\vsroot.cmd"
set "PATH=C:\Program Files (x86)\Microsoft Visual Studio\Installer;%PATH%"
set CC=
set CXX=
set CMAKE_TOOLCHAIN_FILE=
set CMAKE_GENERATOR=
set ANDROID_NDK=
set CMAKE_BUILD_PARALLEL_LEVEL=2
call "%VSROOT%\VC\Auxiliary\Build\vcvars32.bat" >nul || exit /b 1
set "PATH=%VSROOT%\Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja;%PATH%"
set "HERE=%~dp0"
set "OUT=%~dp0..\..\build\actors"
set "LOCKS=%~dp0..\..\out\locks"
set "LOCK=%LOCKS%\build.lock"
if not exist "%LOCKS%" mkdir "%LOCKS%"
cmake -S "%HERE%." -B "%OUT%" -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_C_COMPILER=cl -DCMAKE_CXX_COMPILER=cl >nul || exit /b 1
:lock
mkdir "%LOCK%" 2>nul
if errorlevel 1 (
  ping -n 16 127.0.0.1 >nul
  goto lock
)
cmake --build "%OUT%"
set RC=%ERRORLEVEL%
rmdir "%LOCK%" 2>nul
if not "%RC%"=="0" exit /b %RC%

"%OUT%\actors_tests\nvcoop_actors_tests.exe" > "%OUT%\real.txt"
set REAL_RC=%ERRORLEVEL%
type "%OUT%\real.txt"
findstr /C:", 0 failed" "%OUT%\real.txt" >nul
set REAL_OK=%ERRORLEVEL%

set MUT_BAD=0
set MUT_N=0
for %%F in ("%OUT%\actors_tests\nvcoop_actors_tests_mut*.exe") do (
  set /a MUT_N+=1
  "%%F" > "%OUT%\%%~nF.txt"
  set RC2=!ERRORLEVEL!
  if "!RC2!"=="0" ( echo SURVIVED: %%~nF & set /a MUT_BAD+=1 ) else ( echo   killed: %%~nF )
)
echo real rc=%REAL_RC% zero-failed-line=%REAL_OK%  mutants=%MUT_N% survivors=%MUT_BAD%
if not "%REAL_RC%"=="0" exit /b 1
if not "%REAL_OK%"=="0" exit /b 1
if not "%MUT_BAD%"=="0" exit /b 1
if "%MUT_N%"=="0" exit /b 1
echo plugin/actors: real tests pass, all %MUT_N% mutants killed - ALL OK
endlocal
exit /b 0
