@echo off
rem One-command plugin/combat tests, using the SHARED build tree (build\all, tools\build\build.py):
rem   the real suite must report " 0 failed"; every mutation twin (FNVCOOP_MUT=<n>, switched at RUN time) must report ">0 failed".
rem   Twins: 701..712 714 716..724 731 789..800 801..807 and 809..813, 820..822 (plugin/threat damage feed, tests/test_social_hooks.cpp; 798..800 = G04 kill attribution; 801..812 = a guest's blows on host-owned copies, tests/test_melee.cpp; combat_sync.cpp and, for 806 and 807, combat_wire.cpp), 761..763 (call_patch.cpp), 771..777 and 808 (hit_edit.cpp), 778..780 and 781..788 (combat_sync.cpp, the plugin/companions hook points: tests/test_social_hooks.cpp), and the synccore/combat rule twins that THIS binary also
rem   catches through the choreography tests (301 302 307 308 309 317 318 329 330 331 334 337 338 344; the rest are caught by synccore\combat\run_tests.cmd).
rem   713 and 715 are retired numbers (redundant guards, no behaviour to twin).
setlocal
for %%I in ("%~dp0..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target nvcoop_combat_tests --label "combat-hooks" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "combat-hooks" --twins 701-712,714,716-724,731,789-807,808-813,820-822,761-763,771-780,781-788,301,302,307,308,309,317,318,329,330,331,334,337,338,344 --real-regex " 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\combat_tests\nvcoop_combat_tests.exe" || exit /b 1
exit /b 0
