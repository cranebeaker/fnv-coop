@echo off
rem Build the game-free coop tests + fake peers (x86, like the game) into out\L10. Usage: plugin\coop\build_coop.cmd [clean]
setlocal
call "%~dp0..\..\tools\build\vsroot.cmd"
set "CMAKE_TOOLCHAIN_FILE="
set "ANDROID_NDK_HOME="
set "CXX="
set "CC="
set "REPO=%~dp0..\.."
set "PATH=C:\Program Files (x86)\Microsoft Visual Studio\Installer;%PATH%"
call "%VSROOT%\VC\Auxiliary\Build\vcvars32.bat" >nul || exit /b 1
set "PATH=%VSROOT%\Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja;%PATH%"
if /i "%1"=="clean" rmdir /s /q "%REPO%\build\L10" 2>nul
cmake -S "%REPO%\plugin\coop" -B "%REPO%\build\L10" -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=cl || exit /b 1
cmake --build "%REPO%\build\L10" || exit /b 1
endlocal
