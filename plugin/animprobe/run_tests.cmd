@echo off
rem One-command animprobe core tests, using the SHARED build tree (build\all, tools\build\build.py):
rem   real tests must report ", 0 failed" (exit 0); every mutation twin (FNVCOOP_MUT=1..6, switched at RUN time) must fail (exit non-zero)
setlocal
for %%I in ("%~dp0..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target animprobe_tests --label "T7 animprobe" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "animprobe" --twins 1-7 --real-regex ", 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\animprobe_tests\animprobe_tests.exe" || exit /b 1
exit /b 0
