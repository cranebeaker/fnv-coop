@echo off
rem social (companions, crime, kill rewards): one-command tests, using the SHARED build tree (build\all, tools\build\build.py: build lock + MSVC env):
rem   the real run must report ", 0 failed"; every mutation twin (FNVCOOP_MUT=<n>, switched at RUN time, nothing recompiled per twin) must fail.
rem   Twins: 801..829 companions.cpp, 831..854 crime.cpp, 861..870 rewards.cpp, 880..883 wire.cpp, 884..891 companions.cpp (party companions), 892..895 + 899 + 900 slots.cpp, 896..898 escort.cpp. Unused numbers: 847.
setlocal
for %%I in ("%~dp0..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
set "CMAKE_BUILD_PARALLEL_LEVEL=2"
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target social_tests --label "social-core" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "social-core" --twins 801-829,831-846,848-854,861-870,880-900 --real-regex ", 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\synccore\social\social_tests.exe" || exit /b 1
echo social-core: the real suite is green and every mutation twin is caught - ALL OK
exit /b 0
