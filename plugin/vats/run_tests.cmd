@echo off
rem One-command plugin/vats tests, using the SHARED build tree (build\all, tools\build\build.py: build lock + MSVC env):
rem   1. the real suite must report " 0 failed"; every mutation twin (FNVCOOP_MUT=<n>, switched at RUN time) must report ">0 failed"
rem      twins 801 802 (types) 811..819 (policy, 818 819 the interface-update handoff) 821..831 (slow-mo) 841 842 (wire) 851..857 (limb API) 861..865 871..875 (perks)
rem      881..899 905 906 (sync) 901..904 (patcher)
rem   2. the pin verifier's own logic (and, when out\L5\re\fnv_reference.exe exists, the pins against the real image)
rem   3. the in-game proof checker's selftest
rem Exit code 0 (and "ALL OK") only if all of that holds.
setlocal
for %%I in ("%~dp0..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target nvcoop_vats_tests --label "vats" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "vats" --twins 801-802,811-819,821-829,831,841-842,851-857,861-865,871-875,881-887,891-899,901-906 --real-regex " 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\vats_tests\nvcoop_vats_tests.exe" || exit /b 1
"%PYTHON%" -I "%REPO%\plugin\vats\tools\verify_pins.py" --selftest || exit /b 1
"%PYTHON%" -I "%REPO%\plugin\vats\proof\vats_proof.py" --selftest || exit /b 1
echo ALL OK
exit /b 0
