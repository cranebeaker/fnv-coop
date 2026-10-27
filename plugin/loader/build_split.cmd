@echo off
rem Build the SPLIT plugin (loader nvcoop.dll + core nvcoop_mod.dll) into out\L1split using its own build dir.
rem Usage: plugin\loader\build_split.cmd [clean]      (the canonical monolith build is tools\runner\build_l1.cmd)
setlocal
call "%~dp0..\..\tools\build\vsroot.cmd"
set "CMAKE_TOOLCHAIN_FILE="
set "ANDROID_NDK_HOME="
set "CXX="
set "CC="
if "%CMAKE_BUILD_PARALLEL_LEVEL%"=="" set "CMAKE_BUILD_PARALLEL_LEVEL=2"
set "REPO=%~dp0..\.."
set "PATH=C:\Program Files (x86)\Microsoft Visual Studio\Installer;%PATH%"
call "%VSROOT%\VC\Auxiliary\Build\vcvars32.bat" >nul || exit /b 1
set "PATH=%VSROOT%\Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja;%PATH%"
if /i "%1"=="clean" rmdir /s /q "%REPO%\build\L1split" 2>nul
cmake -S "%REPO%\plugin" -B "%REPO%\build\L1split" -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=cl -DNVCOOP_SPLIT_LOADER=ON -DNVCOOP_OUT_DIR="%REPO%/out/L1split" %CMAKE_EXTRA% || exit /b 1
cmake --build "%REPO%\build\L1split" || exit /b 1
endlocal
