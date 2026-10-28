@echo off
rem plugin/companions (SocialSync: companions, crime, kill rewards across machines): one-command tests, using the SHARED build tree (build\all, tools\build\build.py):
rem   the real suite must report " 0 failed"; every mutation twin (FNVCOOP_MUT=<n>, switched at RUN time) must report ">0 failed".
rem   Twins: 901..944 social_sync.cpp (908 918 929 930 unused; 944 = W3a commit vet; 933..943 are the G34 set pieces, test_sets_flow.cpp). The synccore/social rule twins (801..883) are caught by synccore\social\run_tests.cmd.
setlocal
for %%I in ("%~dp0..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
set "CMAKE_BUILD_PARALLEL_LEVEL=2"
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target nvcoop_companions_tests --label "companions-crime" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "companions-crime" --twins 901-907,909-917,919-928,931-944 --real-regex " 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\companions_tests\nvcoop_companions_tests.exe" || exit /b 1
echo companions-crime: the real suite is green and every mutation twin is caught - ALL OK
exit /b 0
