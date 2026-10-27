@echo off
rem live-core: build only the live targets (Win32/x86, Release, 2 jobs) into build\live-core and run its tests + twins.
rem Takes the repo-wide compile lock (out\locks\build.lock, atomic mkdir) only while compiling.
rem Prints ALL OK when every test passes.  Usage: synccore\live\run_tests.bat
setlocal
call "%~dp0..\..\tools\build\vsroot.cmd"
set "PATH=C:\Program Files (x86)\Microsoft Visual Studio\Installer;%PATH%"
set CC=
set CXX=
set CMAKE_TOOLCHAIN_FILE=
set ANDROID_NDK=
set CMAKE_BUILD_PARALLEL_LEVEL=2
call "%VSROOT%\VC\Auxiliary\Build\vcvars32.bat" >nul
set "PATH=%VSROOT%\Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja;%PATH%"
set "SRC=%~dp0.."
set "OUT=%~dp0..\..\build\live-core"
set "LOCKS=%~dp0..\..\out\locks"
set "LOCK=%LOCKS%\build.lock"
if not exist "%LOCKS%" mkdir "%LOCKS%"
cmake -S "%SRC%" -B "%OUT%" -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_C_COMPILER=cl -DCMAKE_CXX_COMPILER=cl || exit /b 1
:lock
mkdir "%LOCK%" 2>nul
if errorlevel 1 (
  ping -n 16 127.0.0.1 >nul
  goto lock
)
cmake --build "%OUT%" --target live_tests
set RC=%ERRORLEVEL%
rmdir "%LOCK%" 2>nul
if not "%RC%"=="0" exit /b %RC%
ctest --test-dir "%OUT%" -R "^(live_|twin_live_)" --output-on-failure || exit /b 1
echo ALL OK
exit /b 0
