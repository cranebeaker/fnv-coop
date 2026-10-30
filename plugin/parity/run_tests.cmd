@echo off
rem One-command plugin/parity tests, using the SHARED build tree (build\all, tools\build\build.py: build lock + MSVC env):
rem   1. the real suite must report ", 0 failed"; every mutation twin (FNVCOOP_MUT=<n>, switched at RUN time) must report a failure:
rem      1800..1803 (wire) 1804..1807 1817 1819..1821 (compare) 1808 (anim) 1809..1811 (reporter) 1812 1813 1816 1822 1823 (sampler) 1814 (config)
rem      1815 (file) 1818 1824..1827 (service)
rem   2. the Python half: the offline comparer, its tests and its twins (tools/parity/run_tests.py), which also checks tools/parity/vectors.json is current
rem   3. the wiring check of the module (tools/lint/check_parity_wiring.py --self-test)
rem Exit code 0 (and "ALL OK") only if all of that holds.
setlocal
for %%I in ("%~dp0..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target nvcoop_parity_tests --label "parity" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "parity" --twins 1800-1827 --real-regex ", 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\parity_tests\nvcoop_parity_tests.exe" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\parity\run_tests.py" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\lint\check_parity_wiring.py" --self-test || exit /b 1
echo ALL OK
exit /b 0
