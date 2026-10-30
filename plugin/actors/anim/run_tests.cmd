@echo off
rem One-command NPC animation tests (plugin/actors/anim), using the SHARED build tree (build\all, tools\build\build.py):
rem   1. the group table check (python): anim_group_names.inc and the constants in anim_groups.h equal tools/animcheck/animgroups_table.py (+ its twin)
rem   2. the static slot check (python): the BaseProcess vtable slots the game backend may call, read from a reference exe (skipped without it) (+ its twin)
rem   3. the real suite must report ", 0 failed" and exit 0; every mutation twin (FNVCOOP_MUT=<n>, switched at RUN time) must be caught
rem Prints ALL OK on success.
setlocal
for %%I in ("%~dp0..\..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
"%PYTHON%" -I "%REPO%\plugin\actors\anim\gen_group_names.py" --check || exit /b 1
"%PYTHON%" -I "%REPO%\plugin\actors\anim\gen_group_names.py" --check --mutant >nul
if "%ERRORLEVEL%"=="0" ( echo anim: the group table checker twin SURVIVED & exit /b 1 )
"%PYTHON%" -I "%REPO%\plugin\actors\anim\verify_slots.py" || exit /b 1
"%PYTHON%" -I "%REPO%\plugin\actors\anim\verify_slots.py" --mutant >nul
if "%ERRORLEVEL%"=="0" ( echo anim: the slot checker twin SURVIVED & exit /b 1 )
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target nvcoop_anim_tests --label "enemy-anim" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "enemy-anim" --twins 1-8,20-22,30-56,60-63,70-92 --real-regex ", 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\anim_tests\nvcoop_anim_tests.exe" || exit /b 1
echo enemy-anim: ALL OK
exit /b 0
