@echo off
rem Build + test the nvcoop-side NV:MP client patcher and nvmp.players (x86 MSVC), then the mutation twins, which must FAIL.
rem NOTE: exe names must not contain patch/setup/install/update: Windows installer-detection demands elevation for them.
setlocal
call "%~dp0..\..\..\tools\build\vsroot.cmd"
set HERE=%~dp0
set OUT=%HERE%..\..\..\out\nvmp\client_patches
if not exist "%OUT%" mkdir "%OUT%"
rem vcvarsall looks for vswhere.exe on PATH: put the VS Installer dir there so it prints nothing
if exist "%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe" set "PATH=%ProgramFiles(x86)%\Microsoft Visual Studio\Installer;%PATH%"
call "%VSROOT%\VC\Auxiliary\Build\vcvarsall.bat" x86 >nul 2>&1
if errorlevel 1 (echo vcvarsall failed & exit /b 2)
pushd "%HERE%"
set CL_COMMON=/nologo /std:c++17 /EHsc /W4 /O1 /MT /D_CRT_SECURE_NO_WARNINGS /I..\..\..\tools\mut /Fo"%OUT%\\"
set SRC=tests\test_client_patches.cpp client_patcher.cpp players.cpp client_patches_module.cpp ..\patches.cpp ..\sigscan.cpp ..\..\common\nvcoop_api.cpp ..\..\reload\reload_state.cpp
set PSRC=tests\test_players.cpp players.cpp ..\sigscan.cpp

echo === build: real ===
cl %CL_COMMON% /Fe"%OUT%\test_nvcli.exe" %SRC% >"%OUT%\build_real.log" 2>&1
if errorlevel 1 (type "%OUT%\build_real.log" & popd & exit /b 3)
"%OUT%\test_nvcli.exe" >"%OUT%\real.txt" 2>&1
set REAL=%errorlevel%
type "%OUT%\real.txt"
findstr /C:", 0 failed" "%OUT%\real.txt" >nul || set REAL=7

echo === build+run: players ===
cl %CL_COMMON% /Fe"%OUT%\test_nvplr.exe" %PSRC% >"%OUT%\build_plr.log" 2>&1
if errorlevel 1 (type "%OUT%\build_plr.log" & popd & exit /b 3)
"%OUT%\test_nvplr.exe" >"%OUT%\plr.txt" 2>&1
set PLR=%errorlevel%
type "%OUT%\plr.txt"
findstr /C:", 0 failed" "%OUT%\plr.txt" >nul || set PLR=7

echo === build: mutants ===
cl %CL_COMMON% /DCLIENT_PATCHER_MUTANT_ALL /DPATCHES_MUTANT_SKIP_VERIFY /Fe"%OUT%\test_nvcli_mutant.exe" %SRC% >"%OUT%\build_mut.log" 2>&1
if errorlevel 1 (type "%OUT%\build_mut.log" & popd & exit /b 3)
cl %CL_COMMON% /DPLAYERS_MUTANT_SWAP_SLOT /Fe"%OUT%\test_nvplr_mutant.exe" %PSRC% >"%OUT%\build_plrmut.log" 2>&1
if errorlevel 1 (type "%OUT%\build_plrmut.log" & popd & exit /b 3)
echo === run: mutants (must FAIL) ===
"%OUT%\test_nvcli_mutant.exe" >"%OUT%\mut.txt" 2>&1
set MUTRC=%errorlevel%
findstr /C:"FAIL" /C:"checks" "%OUT%\mut.txt"
findstr /R /C:", [1-9][0-9]* failed" "%OUT%\mut.txt" >nul || set MUTRC=0
"%OUT%\test_nvplr_mutant.exe" >"%OUT%\plrmut.txt" 2>&1
set PMUT=%errorlevel%
findstr /C:"FAIL" /C:"checks" "%OUT%\plrmut.txt"
findstr /R /C:", [1-9][0-9]* failed" "%OUT%\plrmut.txt" >nul || set PMUT=0
popd
del /q "%OUT%\*.obj" 2>nul
echo.
echo real=%REAL% players=%PLR% mutant=%MUTRC% players-mutant=%PMUT% (expected: 0, 0, non-zero, non-zero)
if not "%REAL%"=="0" exit /b 1
if not "%PLR%"=="0" exit /b 1
if "%MUTRC%"=="0" (echo MUTANT SURVIVED & exit /b 1)
if "%PMUT%"=="0" (echo PLAYERS MUTANT SURVIVED & exit /b 1)
echo ALL OK
exit /b 0
