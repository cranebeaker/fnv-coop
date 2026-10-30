@echo off
rem One-command tests of the live-layer wiring (dll-wiring): ini switches, LiveHub lifecycle + channel routing, the avatar network service on a
rem loopback of three machines, the avatar wire codec. Uses the SHARED build tree (build\all, tools\build\build.py: build lock + MSVC env).
rem   real tests   must report ", 0 failed" (exit 0)
rem   every mutation twin (FNVCOOP_MUT = 1..4, 11..23, 31..33, 41..45, 47..51, 61..69, 209, 212, 213, 951..956, 1170..1178, switched at RUN time) must fail (exit non-zero)
rem Prints ALL OK and exits 0 only if all of that holds.
setlocal
for %%I in ("%~dp0..\..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target nvcoop_live_tests --label "live wiring" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "live wiring" --twins 1-4,11-23,31-33,41-45,47-51,61-69,209,212,213,951-956,1170-1178,1186,1187 --real-regex ", 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\live_tests\nvcoop_live_tests.exe" || exit /b 1
exit /b 0
