@echo off
rem Build (x86, Ninja) and run the game-free loader tests + mutation twins. Prints ALL OK on success.
rem Twins 21..34 (loader_host.cpp, list at its top) are switched at RUN time (FNVCOOP_MUT), the test exe is compiled once.
rem   21 nocopy 22 nopurge 23 noveto 24 norollback 25 noconfig 26 configonce 27 noprune 28 noattrs 29 novolatile
rem   30 coredirconfig 31 retryall 32 noretry 33 staleid
rem Usage: plugin\loader\run_tests.cmd [clean]
setlocal
call "%~dp0..\..\tools\build\vsroot.cmd"
set "CMAKE_TOOLCHAIN_FILE="
set "ANDROID_NDK_HOME="
set "CXX="
set "CC="
set "CMAKE_BUILD_PARALLEL_LEVEL=2"
set "HERE=%~dp0"
for %%I in ("%~dp0..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
set "PATH=C:\Program Files (x86)\Microsoft Visual Studio\Installer;%PATH%"
call "%VSROOT%\VC\Auxiliary\Build\vcvars32.bat" >nul || exit /b 1
set "PATH=%VSROOT%\Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja;%PATH%"
set "BLD=%REPO%\build\T3a"
if /i "%1"=="clean" rmdir /s /q "%BLD%" 2>nul
cmake -S "%HERE%." -B "%BLD%" -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=cl >nul || (echo CONFIGURE FAILED & exit /b 1)
cmake --build "%BLD%" || (echo BUILD FAILED & exit /b 1)
set "OUT=%REPO%\out\T3a"
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "T3a loader" --twins 21-34 --jobs 1 --real-regex ", 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%OUT%\nvloader_test.exe" "%OUT%" || (echo TESTS FAILED & exit /b 1)
echo ALL OK
endlocal
exit /b 0
