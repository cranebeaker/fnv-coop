@echo off
rem Build nvcoop.dll WITH the coop module into out\L10\plugin\ (does not touch out\L1). Usage: plugin\coop\build_plugin.cmd [clean]
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
if /i "%1"=="clean" rmdir /s /q "%REPO%\build\L10plug" 2>nul
cmake -S "%~dp0.." -B "%REPO%\build\L10plug" -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=cl -DNVCOOP_OUT_DIR="%REPO%/out/L10/plugin" -DCOOP_OUT="%REPO%/out/L10" %CMAKE_EXTRA% || exit /b 1
cmake --build "%REPO%\build\L10plug" --target nvcoop coop_peer || exit /b 1
rem generated data (rules.json copy, EDID map, INFO->quest map) from the committed sources, next to nvcoop.dll
python -I "%~dp0tools\gen_coop_data.py" --out "%REPO%\out\L10\plugin" || echo WARNING: gen_coop_data failed - the rules engine will have no EDID map
python -I "%~dp0tools\deploy_instances.py" || echo note: no private instances to deploy to
endlocal
