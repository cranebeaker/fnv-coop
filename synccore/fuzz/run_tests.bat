@echo off
rem One command: build the co-op fuzzer (Win32/x86 MSVC, Ninja, 2 build jobs) into ..\..\build\T2b-fuzz and run its tests.
rem Prints ALL OK when every test and every mutation twin passes.
setlocal
call "%~dp0..\..\tools\build\vsroot.cmd"
set "PATH=C:\Program Files (x86)\Microsoft Visual Studio\Installer;%PATH%"
set CC=
set CXX=
set CMAKE_TOOLCHAIN_FILE=
set ANDROID_NDK=
set CMAKE_BUILD_PARALLEL_LEVEL=2
call "%VSROOT%\VC\Auxiliary\Build\vcvars32.bat" >nul
set "OUT=%~dp0..\..\build\T2b-fuzz"
set "PATH=%VSROOT%\Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja;%PATH%"
if "%FUZZ_FLIGHT%"=="" set FUZZ_FLIGHT=ON
set EMFLAG=ON
if "%FUZZ_EM%"=="0" set EMFLAG=OFF
if "%FUZZ_EM%"=="0" set "OUT=%~dp0..\..\build\T2b-fuzz-mini"
cmake -S "%~dp0." -B "%OUT%" -DFUZZ_EM=%EMFLAG% -DFUZZ_FLIGHT=%FUZZ_FLIGHT% -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_C_COMPILER=cl -DCMAKE_CXX_COMPILER=cl || exit /b 1
if not exist "%~dp0..\..\out\locks" mkdir "%~dp0..\..\out\locks"
rem repo-wide compile lock (atomic mkdir), held only while compiling; ping instead of timeout (timeout.exe breaks under redirected stdin)
:lock
mkdir "%~dp0..\..\out\locks\build.lock" 2>nul
if errorlevel 1 (
  ping -n 16 127.0.0.1 >nul
  goto lock
)
cmake --build "%OUT%"
set BRC=%ERRORLEVEL%
rmdir "%~dp0..\..\out\locks\build.lock" 2>nul
if not "%BRC%"=="0" exit /b %BRC%
"%OUT%\fnvcoop_fuzz_tests.exe" %*
exit /b %ERRORLEVEL%
