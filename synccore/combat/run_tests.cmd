@echo off
rem combat-core: one-command tests, using the SHARED build tree (build\all, tools\build\build.py: build lock + MSVC env):
rem   1. the game-free rules (combat_tests): the real run must report ", 0 failed"; every mutation twin (FNVCOOP_MUT=301..365, switched at RUN time,
rem      nothing recompiled per twin) must fail (exit non-zero)
rem   2. the fake engine running those rules (fnvcoop_engine_tests): the combat group, and the engine twins that guard it
rem      (hit_no_validation, hit_by_non_owner, named_killable and the four combat-core faults) must all be caught
rem Prints ALL OK only if all of that holds.  (The 2-minute fuzz with 0 violations is a separate command: see docs/tools/combat-core.md.)
setlocal
for %%I in ("%~dp0..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
set "CMAKE_BUILD_PARALLEL_LEVEL=2"
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target combat_tests fnvcoop_engine_tests --label "combat-core" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "combat-core rules" --twins 301-366 --real-regex ", 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\synccore\combat\combat_tests.exe" || exit /b 1
set "ENG=%REPO%\build\all\synccore\enginemodel\fnvcoop_engine_tests.exe"
"%ENG%" --group combat > "%REPO%\build\all\synccore\enginemodel\combat_real.txt"
set "RC=%ERRORLEVEL%"
type "%REPO%\build\all\synccore\enginemodel\combat_real.txt"
if not "%RC%"=="0" ( echo combat-core: the engine-model combat group FAILED & exit /b 1 )
findstr /C:", 0 failed" "%REPO%\build\all\synccore\enginemodel\combat_real.txt" >nul || ( echo combat-core: no "0 failed" line & exit /b 1 )
for %%T in (hit_no_validation hit_by_non_owner named_killable death_not_idempotent limb_not_synced visual_shot_damages blast_runs_everywhere hit_no_validation_combat) do (
  "%ENG%" --group twin --name %%T > nul || ( echo combat-core: engine twin %%T SURVIVED & exit /b 1 )
)
echo combat-core: rules suite and all 66 twins caught, engine-model combat group green, 8 engine twins caught - ALL OK
exit /b 0
