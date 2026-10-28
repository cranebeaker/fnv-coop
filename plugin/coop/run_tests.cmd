@echo off
rem L10: build (x86, shared tree build\all via tools\build\build.py) and run every coop test: in-process unit tests, two-process
rem integration, mutation twins. Twins are RUNTIME-switchable (FNVCOOP_MUT=<n>, tools/mut/fnvcoop_mut.h): nothing is rebuilt per twin.
setlocal enabledelayedexpansion
for %%I in ("%~dp0..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target coop_tests coop_peer --label "L10 coop" || exit /b 1
set "FNVCOOP_MUT="
echo === coop_tests (config, UDP transport, loopback convergence, lossy host) ===
"%REPO%\out\L10\coop_tests.exe" || exit /b 1
echo === L11 wiring twins (each dropped wiring call must make the --ui tests fail) ===
set "FNVCOOP_MUT=5"
for %%N in (1 2 3 4 5) do (
  set "COOP_UI_MUTANT=%%N"
  "%REPO%\out\L10\coop_tests.exe" --ui >nul 2>&1
  if !errorlevel! EQU 0 ( echo TWIN SURVIVED: COOP_UI_MUTANT=%%N & exit /b 1 )
  echo   twin %%N killed
)
set "COOP_UI_MUTANT="
echo === leave / join twins ===
for %%N in (1 2) do (
  set "COOP_LEAVE_MUTANT=%%N"
  "%REPO%\out\L10\coop_tests.exe" --leave >nul 2>&1
  if !errorlevel! EQU 0 ( echo TWIN SURVIVED: COOP_LEAVE_MUTANT=%%N & exit /b 1 )
  echo   twin %%N killed
)
set "COOP_LEAVE_MUTANT="
echo === guest presence reports never run as activations (twin 7) ===
set "FNVCOOP_MUT=7"
"%REPO%\out\L10\coop_tests.exe" >nul 2>&1
if !errorlevel! EQU 0 ( echo TWIN SURVIVED: FNVCOOP_MUT=7 & exit /b 1 )
echo   twin 7 killed
echo === a baseline-less host still polls the quest vars object scripts write (twin 8) ===
set "FNVCOOP_MUT=8"
"%REPO%\out\L10\coop_tests.exe" >nul 2>&1
if !errorlevel! EQU 0 ( echo TWIN SURVIVED: FNVCOOP_MUT=8 & exit /b 1 )
echo   twin 8 killed
set "FNVCOOP_MUT="
echo === wave 2 session wiring (--wiring; each twin must break one test) ===
"%REPO%\out\L10\coop_tests.exe" --wiring || exit /b 1
for %%N in (9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 97 98 99 100 140 960 961 962 963 964) do (
  set "FNVCOOP_MUT=%%N"
  "%REPO%\out\L10\coop_tests.exe" --wiring >nul 2>&1
  if !errorlevel! EQU 0 ( echo TWIN SURVIVED: FNVCOOP_MUT=%%N & exit /b 1 )
  echo   wiring twin %%N killed
)
set "FNVCOOP_MUT="
echo === gen_coop_data (cause map, unresolved names) + twin ===
"%PYTHON%" -I "%~dp0tests\test_gen_coop_data.py" || exit /b 1
echo === gen_intents_data (the Dead Money strip containers, W3d item 4) + twins ===
"%PYTHON%" -I "%REPO%\plugin\intents\tools\test_gen_intents_data.py" || exit /b 1
echo === module wiring (static check of coop_module.cpp / intents_module.cpp + its twins) ===
"%PYTHON%" -I "%~dp0tests\check_module_wiring.py" --self-test || exit /b 1
echo === two-process integration + mutation twins (FNVCOOP_MUT=1..4 peers) ===
"%PYTHON%" -I "%~dp0tests\test_integration.py" || exit /b 1
echo L10 coop: ALL OK
endlocal
