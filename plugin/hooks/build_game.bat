@echo off
rem Compile-checks the x86 game glue (engine_hooks.cpp, apply_remote.cpp) against the xNVSE headers.
setlocal
call "%~dp0..\..\tools\build\vsroot.cmd"
set HERE=%~dp0
set B=%TEMP%\nvh_build_x86
set CXX=
set CC=
set CMAKE_TOOLCHAIN_FILE=
set CMAKE_GENERATOR=
set PATH=%PATH%;C:\Program Files (x86)\Microsoft Visual Studio\Installer
call "%VSROOT%\VC\Auxiliary\Build\vcvars32.bat" >nul || exit /b 1
cmake -S "%HERE%." -B "%B%" -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=cl >nul || exit /b 1
cmake --build "%B%" || exit /b 1
ctest --test-dir "%B%" --output-on-failure
exit /b %ERRORLEVEL%
