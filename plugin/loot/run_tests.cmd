@echo off
rem One-command plugin/loot tests, using the SHARED build tree (build\all, tools\build\build.py: build lock + MSVC env):
rem   1. synccore/loot core: real tests must report ", 0 failed"; every mutation twin (FNVCOOP_MUT=101..137, 170..173 and 180..183, switched at RUN time) must fail
rem   2. plugin/loot:        real tests must report ", 0 failed"; twins 141..167, 170, 172..180, 184..186 (171 is guarded by the core suite only) must fail
rem   3. the in-game proof script's own pass / fail logic against a fake world (it is NOT run against a game here)
rem Exit code 0 (and "ALL OK") only if all of that holds.
setlocal
for %%I in ("%~dp0..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target loot_tests nvcoop_loot_tests --label "loot" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "loot core" --twins 101-137,170-173,180-183 --real-regex ", 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\loot_tests\loot_tests.exe" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "loot plugin" --twins 141-167,170,172-180,184-186 --real-regex ", 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\loot_plugin_tests\nvcoop_loot_tests.exe" || exit /b 1
"%PYTHON%" -I "%REPO%\plugin\loot\tools\loot_proof.py" --selftest || exit /b 1
echo ALL OK
exit /b 0
