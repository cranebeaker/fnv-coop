@echo off
rem One-command T6a bot-driver tests:
rem   real tests must report " 0 failed"; every mutation twin (FNVCOOP_MUT=1..64, switched at RUN time) must report ">0 failed".
rem Build: the SHARED build tree (build\all, tools\build\build.py) when that script exists. Without it (main has no tools\build\build.py),
rem a standalone build of plugin\bots in %TEMP%\nvbots_build (MSVC x64 + Ninja).
rem BOTS_EXE: set NVCOOP_REF_EXE to a reference FalloutNV.exe image to compare the pinned prologues with it.
setlocal
call "%~dp0..\..\tools\build\vsroot.cmd"
for %%I in ("%~dp0..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
if not defined BOTS_EXE if defined NVCOOP_REF_EXE set "BOTS_EXE=%NVCOOP_REF_EXE%"
if exist "%REPO%\tools\build\build.py" (
  "%PYTHON%" -I "%REPO%\tools\build\build.py" --target nvbots_tests --label "T6a bots" || exit /b 1
  set "TESTEXE=%REPO%\build\all\bots_tests\nvbots_tests.exe"
) else (
  call :standalone || exit /b 1
  set "TESTEXE=%TEMP%\nvbots_build\bots_tests\nvbots_tests.exe"
)
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "T6a bots" --twins 1-67--real-regex " 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%TESTEXE%" || exit /b 1
exit /b 0

:standalone
setlocal
set "BLD=%TEMP%\nvbots_build"
set CXX=
set CC=
set CMAKE_TOOLCHAIN_FILE=
set CMAKE_GENERATOR=
set CMAKE_ANDROID_NDK=
if not defined CMAKE_BUILD_PARALLEL_LEVEL set CMAKE_BUILD_PARALLEL_LEVEL=2
set "PATH=%PATH%;C:\Program Files (x86)\Microsoft Visual Studio\Installer"
call "%VSROOT%\VC\Auxiliary\Build\vcvars64.bat" >nul || exit /b 1
cmake -S "%~dp0." -B "%BLD%" -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=cl >nul || exit /b 1
cmake --build "%BLD%" || exit /b 1
exit /b 0
