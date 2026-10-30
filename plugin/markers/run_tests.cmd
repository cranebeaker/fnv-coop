@echo off
rem One-command plugin/markers tests (friend markers and revive), using the SHARED build tree (build\all, tools\build\build.py: build lock + MSVC env):
rem   1. the game-free core: real tests must report ", 0 failed"; every mutation twin (FNVCOOP_MUT, switched at RUN time) must fail
rem      1301..1322 board / text / map projection, 1331..1338 wire, 1351..1368 revive authority, 1383..1386 hold tracker, 1401..1419 service, 1420..1434 layout (compass tile, Pip-Boy map tiles), 1441..1447 projection, 1450..1462 + 1464 painter (tags, compass, map, list, banner)
rem   2. the static wiring check of plugin/coop/coop_module.cpp (the downed / respawn / revive / party order hooks)
rem Exit code 0 (and "ALL OK") only if all of that holds.
setlocal
for %%I in ("%~dp0..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target nvcoop_markers_tests --label "markers" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "markers" --twins 1301-1318,1321,1322,1331-1338,1351-1368,1383-1386,1401-1407,1409-1414,1416-1419,1420-1434,1441-1447,1450-1462,1464 --real-regex ", 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\markers_tests\nvcoop_markers_tests.exe" || exit /b 1
"%PYTHON%" -I "%REPO%\plugin\coop\tests\check_module_wiring.py" --self-test || exit /b 1
echo ALL OK
exit /b 0
