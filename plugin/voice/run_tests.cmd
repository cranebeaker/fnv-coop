@echo off
rem One-command voice chat tests, using the SHARED build tree (build\all, tools\build\build.py: build lock + MSVC env):
rem   plugin/voice core (wire, jitter buffer, placement, transmitter, receiver, mixer, resampler, session, settings, the real Opus codec):
rem   the real tests must report ", 0 failed"; every mutation twin (FNVCOOP_MUT=301..399, switched at RUN time) must fail.
rem Exit code 0 (and "ALL OK") only if all of that holds.
setlocal
for %%I in ("%~dp0..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target nvcoop_voice_tests --label "voice" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "voice" --twins 301-306,311-317,321-324,331-335,341-344,351-356,361-363,371,381-388,391-399 --real-regex ", 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\voice_tests\nvcoop_voice_tests.exe" || exit /b 1
echo ALL OK
exit /b 0
