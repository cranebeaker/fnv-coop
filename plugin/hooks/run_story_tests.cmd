@echo off
rem One-command story-capture tests (shared build tree build\all; takes the build lock only while compiling):
rem   real tests   must report ", 0 failed" (exit 0)
rem   every runtime mutation twin (FNVCOOP_MUT=1..14, 97 and 98, switched at RUN time) must fail (exit non-zero)
rem Exit code 0 (and "ALL OK") only if all of that holds.
setlocal
for %%I in ("%~dp0..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target nvcoop_story_tests --label "story-capture" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "story-capture" --twins 1-14,97,98 --real-regex ", 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\hooks_tests\nvcoop_story_tests.exe" || exit /b 1
exit /b 0
