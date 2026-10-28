@echo off
rem One-command guest-intents tests, using the SHARED build tree (build\all, tools\build\build.py: build lock + MSVC env):
rem   1. nvcoop_intents_tests: wire, ini, guest + host policy, container redirect, host pipeline, guest client, loopback end to end, the real coop
rem      Runtime with a host and two guests over ENet (service), SaveGate, engine pins, gear stashes, trigger presence, notes. Must report ", 0 failed".
rem   2. every runtime mutation twin (FNVCOOP_MUT=301,302,304..336,345,346,401..424,441) must fail at least one check.
rem   3. the in-game proof script's own pass / fail logic against a fake bridge (it is NOT run against a game here).
rem   4. the generated engine\intents_addrs.h is what tools\gen_intents_addrs.py writes from a reference exe (skipped when the exe is absent).
rem   5. data\nvcoop_intents.json is what tools\gen_intents_data.py writes from the game data (skipped when the EditorID map is absent).
rem Exit code 0 (and "ALL OK") only if all of that holds.
setlocal
for %%I in ("%~dp0..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target nvcoop_intents_tests --label "guest-intents" || exit /b 1
pushd "%REPO%"
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "guest intents" --twins 301,302,304-336,345,346,401-424,441 --real-regex ", 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\intents_tests\nvcoop_intents_tests.exe"
set "RC=%ERRORLEVEL%"
popd
if not "%RC%"=="0" exit /b 1
"%PYTHON%" -I "%REPO%\plugin\intents\tools\intents_proof.py" --selftest || exit /b 1
if exist "%REPO%\out\L5\re\fnv_reference.exe" "%PYTHON%" -I "%REPO%\plugin\intents\tools\gen_intents_addrs.py" "%REPO%\out\L5\re\fnv_reference.exe" --check || exit /b 1
if exist "%REPO%\out\L10\data\nvcoop_edids.json" "%PYTHON%" -I "%REPO%\plugin\intents\tools\gen_intents_data.py" --check || exit /b 1
echo ALL OK
exit /b 0
