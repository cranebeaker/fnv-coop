@echo off
rem One-command area-desync tests, using the SHARED build tree (build\all, tools\build\build.py: build lock + MSVC env):
rem   1. synccore/areadigest core: real tests must report ", 0 failed"; every mutation twin (FNVCOOP_MUT=<n>, switched at RUN time) must fail:
rem      1..32 34..39 (areadigest.cpp, ad_wire.cpp) and 41 42 (the flight recorder's heal export, switched on by the test itself)
rem   2. plugin/areadigest: real tests must report ", 0 failed"; twins 61..68 70 72..74 76..82 must fail
rem   3. the Python half: dashboard + playbook hook + 6 twins (tools/areadigest/run_tests.py)
rem   4. the in-game proof script's own pass / fail logic against a fake world (it is NOT run against a game here)
rem Exit code 0 (and "ALL OK") only if all of that holds.
setlocal
for %%I in ("%~dp0..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target areadigest_tests nvcoop_areadigest_tests --label "area-desync" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "area digest core" --twins 1-32,34-39,41,42 --real-regex ", 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\areadigest_tests\areadigest_tests.exe" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "area digest plugin" --twins 61-68,70,72-74,76-82 --real-regex ", 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\areadigest_plugin_tests\nvcoop_areadigest_tests.exe" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\areadigest\run_tests.py" || exit /b 1
"%PYTHON%" -I "%REPO%\plugin\areadigest\tools\area_proof.py" --selftest || exit /b 1
echo ALL OK
exit /b 0
