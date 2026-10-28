@echo off
rem One-command L11 tests, using the SHARED build tree (build\all, tools\build\build.py):
rem   router tests : real must report " 0 failed"; twins FNVCOOP_MUT=1..9 (switched at RUN time) must report ">0 failed"
rem   bridge tests : real must report " 0 failed"; twins FNVCOOP_MUT=10..20 must report ">0 failed"
rem Set NVUI_EXE=<dump of the running game image> to also compare every pinned prologue with the real image.
setlocal
for %%I in ("%~dp0..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target nvui_tests nvui_bridge_tests --label "L11 hooks_ui" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "L11 hooks_ui router" --twins 1-9,21-24,28 --real-regex " 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\ui_tests\nvui_tests.exe" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "L11 hooks_ui bridge" --twins 10-20,25-27,29 --real-regex " 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\ui_tests\nvui_bridge_tests.exe" || exit /b 1
echo L11 hooks_ui: real tests pass, all 29 twins rejected - ALL OK
exit /b 0
