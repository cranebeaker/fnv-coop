@echo off
rem One-command test run for the nvmp lane (x86 MSVC). Builds the tests, runs them, then builds the
rem mutation twin (safety checks removed) and requires it to FAIL.
setlocal
call "%~dp0..\..\tools\build\vsroot.cmd"
set HERE=%~dp0
set OUT=%HERE%..\..\out\nvmp
if not exist "%OUT%" mkdir "%OUT%"
rem vcvarsall looks for vswhere.exe on PATH: put the VS Installer dir there so it prints nothing
if exist "%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe" set "PATH=%ProgramFiles(x86)%\Microsoft Visual Studio\Installer;%PATH%"
call "%VSROOT%\VC\Auxiliary\Build\vcvarsall.bat" x86 >nul 2>&1
if errorlevel 1 (echo vcvarsall failed & exit /b 2)
set SRC="%HERE%tests\test_nvmp.cpp" "%HERE%sigscan.cpp" "%HERE%patches.cpp"
pushd "%HERE%tests"

echo === build: real ===
cl /nologo /std:c++17 /EHsc /W4 /O1 /Fo"%OUT%\\" /Fe"%OUT%\test_nvmp.exe" %SRC% >"%OUT%\build_real.log" 2>&1
if errorlevel 1 (type "%OUT%\build_real.log" & exit /b 3)
echo === run: real ===
"%OUT%\test_nvmp.exe" >"%OUT%\real.txt" 2>&1
set REAL=%errorlevel%
type "%OUT%\real.txt"
findstr /C:", 0 failed" "%OUT%\real.txt" >nul || set REAL=7

echo === build: mutant ===
cl /nologo /std:c++17 /EHsc /W4 /O1 /DSIGSCAN_MUTANT_ACCEPT_AMBIGUOUS /DPATCHES_MUTANT_SKIP_VERIFY /Fo"%OUT%\\" /Fe"%OUT%\test_nvmp_mutant.exe" %SRC% >"%OUT%\build_mut.log" 2>&1
if errorlevel 1 (type "%OUT%\build_mut.log" & exit /b 3)
echo === run: mutant (must FAIL) ===
"%OUT%\test_nvmp_mutant.exe" >"%OUT%\mut.txt" 2>&1
set MUTRC=%errorlevel%
findstr /C:"FAIL" /C:"checks" "%OUT%\mut.txt"
findstr /R /C:", [1-9][0-9]* failed" "%OUT%\mut.txt" >nul || set MUTRC=0
popd

del /q "%OUT%\*.obj" 2>nul
echo.
echo real test exit=%REAL%   mutant exit=%MUTRC% (expected: 0 and non-zero)
if not "%REAL%"=="0" exit /b 1
if "%MUTRC%"=="0" (echo MUTANT SURVIVED - tests are too weak & exit /b 1)
echo ALL OK
exit /b 0
