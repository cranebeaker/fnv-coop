@echo off
rem plugin/threat (enemies split their attention between the players): one-command tests, using the SHARED build tree (build\all, tools\build\build.py):
rem   the real suite must report " 0 failed"; every mutation twin (FNVCOOP_MUT=<n>, switched at RUN time) must report ">0 failed".
rem   Twins: 1001 1002 1004..1013 (threat_model.cpp), 1020..1027 1029..1031 (threat_director.cpp), 1040 1041 1043 1045 1046 1048 (tactics.cpp). 1003 1028 1042 1044 1047 are unused numbers.
setlocal
for %%I in ("%~dp0..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
set "CMAKE_BUILD_PARALLEL_LEVEL=2"
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target nvcoop_threat_tests --label "threat" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "threat" --twins 1001,1002,1004-1013,1020-1027,1029-1031,1040,1041,1043,1045,1046,1048 --real-regex " 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\threat_tests\nvcoop_threat_tests.exe" || exit /b 1
echo threat: the real suite is green and every mutation twin is caught - ALL OK
exit /b 0
