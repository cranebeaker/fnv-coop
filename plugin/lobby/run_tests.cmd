@echo off
rem Build and run the game-free lobby tests (plugin/lobby) + mutation twins 1-4 (see tests/test_lobby.cpp). Prints ALL OK on success.
setlocal
set "PATH=C:\Program Files (x86)\Microsoft Visual Studio\Installer;%PATH%"
call "%~dp0..\..\tools\build\vsroot.cmd"
set "HERE=%~dp0"
for %%I in ("%~dp0..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
call "%VSROOT%\VC\Auxiliary\Build\vcvars32.bat" >nul || exit /b 1
set "OUT=%REPO%\out\lobby"
if not exist "%OUT%" mkdir "%OUT%"
cl /nologo /std:c++17 /EHsc /W4 /O1 /DFNVCOOP_RUNTIME_MUT /I"%HERE%." /I"%REPO%\tools\mut" /Fo"%OUT%\\" /Fe"%OUT%\test_lobby.exe" "%HERE%lobby.cpp" "%HERE%tests\test_lobby.cpp" >"%OUT%\build.log" 2>&1 || (type "%OUT%\build.log" & echo BUILD FAILED & exit /b 1)
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "lobby" --twins 1-4 --jobs 1 --real-regex ", 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%OUT%\test_lobby.exe" || (echo TESTS FAILED & exit /b 1)
echo ALL OK
endlocal
exit /b 0
