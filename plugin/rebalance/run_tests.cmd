@echo off
rem One-command plugin/rebalance tests, using the SHARED build tree (build\all, tools\build\build.py: build lock + MSVC env):
rem   the real suite must report " 0 failed"; every mutation twin (FNVCOOP_MUT=<n>, switched at RUN time) must report ">0 failed".
rem   Twins: 1200 1201 (campaign_settings.h), 1210 1211 1212 (balance.cpp), 1220 (ramp.cpp), 1230 1231 (gmst_applier.cpp), 1240 1241 1242 (fights.cpp),
rem   1251 1252 (director.cpp). The party-save half (1260 1262) is in plugin\coop\run_tests.cmd; the combat switch (813) in plugin\combat\run_tests.cmd.
setlocal
for %%I in ("%~dp0..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target nvcoop_rebalance_tests --label "rebalance" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "rebalance" --twins 1200-1201,1210-1212,1220,1230-1231,1240-1242,1251-1252 --real-regex ", 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\rebalance_tests\nvcoop_rebalance_tests.exe" || exit /b 1
exit /b 0
