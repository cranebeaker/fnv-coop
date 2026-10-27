@echo off
rem One-command build + test for the server hook (x86 MSVC). Also builds the DLL and the launcher, smoke-loads the
rem DLL into a fake "storyserver" exe (no signatures, so it must abort cleanly), runs the launcher with --dry-run
rem (never starts a server), then builds the mutation twin and requires it to FAIL.
setlocal
call "%~dp0..\..\..\tools\build\vsroot.cmd"
set HERE=%~dp0
set NV=%HERE%..
set OUT=%NV%\..\..\out\nvmp\server_hook
if not exist "%OUT%" mkdir "%OUT%"
rem vcvarsall looks for vswhere.exe on PATH: put the VS Installer dir there so it prints nothing
if exist "%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe" set "PATH=%ProgramFiles(x86)%\Microsoft Visual Studio\Installer;%PATH%"
call "%VSROOT%\VC\Auxiliary\Build\vcvarsall.bat" x86 >nul 2>&1
if errorlevel 1 (echo vcvarsall failed & exit /b 2)
pushd "%HERE%"
set CL_COMMON=/nologo /std:c++17 /EHsc /W4 /O1 /MT /Fo"%OUT%\\"
set TESTSRC=tests\test_hook.cpp hostfilter.cpp hookcore.cpp resolve.cpp ..\sigscan.cpp

echo === build: DLL ===
cl %CL_COMMON% /LD /Fe"%OUT%\nvmp_server_hook.dll" server_hook.cpp hostfilter.cpp hookcore.cpp resolve.cpp ..\sigscan.cpp user32.lib >"%OUT%\build_dll.log" 2>&1
if errorlevel 1 (type "%OUT%\build_dll.log" & popd & exit /b 3)
echo === build: launcher ===
cl %CL_COMMON% /Fe"%OUT%\nvmp_hook_launcher.exe" launcher.cpp >"%OUT%\build_launcher.log" 2>&1
if errorlevel 1 (type "%OUT%\build_launcher.log" & popd & exit /b 3)
echo === build+run: tests ===
cl %CL_COMMON% /Fe"%OUT%\test_hook.exe" %TESTSRC% >"%OUT%\build_test.log" 2>&1
if errorlevel 1 (type "%OUT%\build_test.log" & popd & exit /b 3)
"%OUT%\test_hook.exe" >"%OUT%\real.txt" 2>&1
set REAL=%errorlevel%
type "%OUT%\real.txt"
findstr /C:", 0 failed" "%OUT%\real.txt" >nul || set REAL=7

echo === DLL smoke (fake_storyserver.exe: no signatures present, must abort cleanly) ===
cl %CL_COMMON% /Fe"%OUT%\fake_storyserver.exe" tests\dll_smoke.cpp >"%OUT%\build_smoke.log" 2>&1
if errorlevel 1 (type "%OUT%\build_smoke.log" & popd & exit /b 3)
del /q "%OUT%\server_hook.log" 2>nul
(echo # smoke) > "%OUT%\hostowned.txt"
(echo 00104C6D # Trudy) >> "%OUT%\hostowned.txt"
(echo 00104C6E) >> "%OUT%\hostowned.txt"
(echo bogus) >> "%OUT%\hostowned.txt"
(echo host_mode=first) > "%OUT%\server_hook.cfg"
"%OUT%\fake_storyserver.exe" "%OUT%\nvmp_server_hook.dll"
set SMOKE=%errorlevel%
type "%OUT%\server_hook.log"
findstr /C:"ABORT: encounter handler not found" "%OUT%\server_hook.log" >nul
if errorlevel 1 (echo smoke log check failed & set SMOKE=9)

echo === launcher dry run (starts nothing) ===
set SRV=%NV%\..\..\third_party\nvmp\nvmp_storyserver.exe
"%OUT%\nvmp_hook_launcher.exe" --dry-run --server "%SRV%" --dll "%OUT%\nvmp_server_hook.dll"

echo === build: mutant ===
cl %CL_COMMON% /DHOOK_MUTANT_INVERT_FILTER /DHOOK_MUTANT_SKIP_PROLOGUE_CHECK /Fe"%OUT%\test_hook_mutant.exe" %TESTSRC% >"%OUT%\build_mut.log" 2>&1
if errorlevel 1 (type "%OUT%\build_mut.log" & popd & exit /b 3)
echo === run: mutant (must FAIL) ===
"%OUT%\test_hook_mutant.exe" >"%OUT%\mut.txt" 2>&1
set MUTRC=%errorlevel%
findstr /C:"FAIL" /C:"checks" "%OUT%\mut.txt"
findstr /R /C:", [1-9][0-9]* failed" "%OUT%\mut.txt" >nul || set MUTRC=0
popd
del /q "%OUT%\*.obj" 2>nul
echo.
echo real exit=%REAL%  smoke exit=%SMOKE%  mutant exit=%MUTRC% (expected: 0, 0, non-zero)
if not "%REAL%"=="0" exit /b 1
if not "%SMOKE%"=="0" (echo DLL smoke failed & exit /b 1)
if "%MUTRC%"=="0" (echo MUTANT SURVIVED & exit /b 1)
echo ALL OK
exit /b 0
