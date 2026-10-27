@echo off
rem One command: build synccore (Win32/x86 MSVC, Ninja) and run every test.
rem   x86 (default, matches the game plugin): uses the SHARED build tree build\all (root CMakeLists.txt, tools\build\build.py: MSVC env,
rem   build lock out\locks\build.lock, sccache if installed), builds "synccore/all" and runs ctest in build\all\synccore.
rem   x64: legacy standalone tree build\L4-x64 (not shared).
rem Usage: synccore\run_tests.bat [x86|x64]
setlocal
call "%~dp0..\tools\build\vsroot.cmd"
set ARCH=%1
if "%ARCH%"=="" set ARCH=x86
for %%I in ("%~dp0..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
if "%ARCH%"=="x64" goto legacy64
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target synccore/all --label "L4 synccore" || exit /b 1
ctest --test-dir "%REPO%\build\all\synccore" -j 2 --output-on-failure
exit /b %ERRORLEVEL%

:legacy64
set "PATH=C:\Program Files (x86)\Microsoft Visual Studio\Installer;%PATH%"
set CC=
set CXX=
set CMAKE_TOOLCHAIN_FILE=
set ANDROID_NDK=
call "%VSROOT%\VC\Auxiliary\Build\vcvars64.bat" >nul
set "OUT=%REPO%\build\L4-x64"
set "PATH=%VSROOT%\Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja;%PATH%"
cmake -S "%~dp0." -B "%OUT%" -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_C_COMPILER=cl -DCMAKE_CXX_COMPILER=cl || exit /b 1
cmake --build "%OUT%" || exit /b 1
ctest --test-dir "%OUT%" --output-on-failure
exit /b %ERRORLEVEL%
