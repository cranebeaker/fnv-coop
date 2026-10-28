@echo off
rem One-command L12 tests, using the SHARED build tree (build\all, tools\build\build.py: build lock + MSVC env):
rem   real tests   must report ", 0 failed" (exit 0)
rem   every mutation twin (FNVCOOP_MUT=1..66, switched at RUN time, nothing recompiled per twin) must fail (exit non-zero)
rem Exit code 0 only if all of that holds.
setlocal
for %%I in ("%~dp0..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target nvcoop_world_tests --label "L12 hooks_world" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "L12 hooks_world" --twins 1-66 --real-regex ", 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\world_tests\nvcoop_world_tests.exe" || exit /b 1
exit /b 0
