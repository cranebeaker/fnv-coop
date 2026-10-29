@echo off
rem One-command plugin/chat tests, using the SHARED build tree (build\all, tools\build\build.py: build lock + MSVC env):
rem   the real suite must report ", 0 failed"; every mutation twin (FNVCOOP_MUT=<n>, switched at RUN time) must report a failure:
rem      1001..1008 (text) 1010..1013 (rate limit) 1020..1025 (history) 1030..1039 (line editor) 1040 1042..1056 (core) 1060..1064 (wire)
rem      1070..1079 (input) 1080..1087 (key blocker) 1090..1094 (view) 1100..1107 (controller) 1110..1113 (ini) 1120 1121 (api)
rem      1130..1136 (session) 1140..1146 (surfaces) 1150 1151 (overlay window)
rem Exit code 0 (and "ALL OK") only if all of that holds.
setlocal
for %%I in ("%~dp0..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target nvcoop_chat_tests --label "chat" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "chat" --twins 1001-1008,1010-1013,1020-1025,1030-1039,1040,1042-1056,1060-1064,1070-1079,1080-1087,1090-1094,1100-1107,1110-1113,1120-1121,1130-1136,1140-1146,1150-1151 --real-regex ", 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\chat_tests\nvcoop_chat_tests.exe" || exit /b 1
echo ALL OK
exit /b 0
