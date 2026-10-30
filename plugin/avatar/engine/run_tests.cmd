@echo off
rem One-command tests of the real puppet engine (plugin/avatar/engine), using the SHARED build tree (build\all, tools\build\build.py):
rem   1. avatar_engine_tests must report ", 0 failed"; every runtime mutation twin (FNVCOOP_MUT=1..34, 40..56, 60..72, 80..98, 963, 964) must report ">0 failed"
rem   1b. avatar_lag_tests (tests/test_lag_sim.cpp): walking tracks over the chaos profiles through the real sender and receiver (the avatar layer's own chain and ActorSync's player profile), how far behind the
rem       puppet appears; must report ", 0 failed"; twins 201..208, 210, 211 must all be caught; avatar_lag_tests --gate (the numeric gate of docs/entity-core-plan.md S11: ActorSync's player stream against the avatar layer's own, same seeds and chaos profiles) must hold
rem   2. the game side compiles: nvcoop.dll with win/game_backend_win.cpp and win/avatar_module.cpp (xNVSE headers, x86)
rem   3. tools\avatar\verify_engine_pins.py: every pinned engine address byte for byte against a reference exe (SKIP without it) + its twins
rem   4. tools\avatar\proof_puppet.py --selftest: the in-game proof's analysis, engine / pin assertions and loopback split, with twins
rem Prints "avatar engine: ALL OK" at the end. Nothing here starts a game.
setlocal
for %%I in ("%~dp0..\..\..") do set "REPO=%%~fI"
if not defined PYTHON set "PYTHON=python"
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target avatar_engine_tests avatar_lag_tests --label "avatar engine" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "avatar engine" --twins 1-34,40-56,60-72,77,80-98,950,957,960-964 --real-regex ", 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\avatar_tests\avatar_engine_tests.exe" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\mut\run_twins.py" --label "avatar lag" --twins 201-208,210-211 --real-regex ", 0 failed" --caught-regex ", [1-9][0-9]* failed" -- "%REPO%\build\all\avatar_tests\avatar_lag_tests.exe" || exit /b 1
"%REPO%\build\all\avatar_tests\avatar_lag_tests.exe" --gate || (echo avatar engine: the S11 lag gate failed & exit /b 1)
"%PYTHON%" -I "%REPO%\tools\build\build.py" --target nvcoop --label "avatar engine (game side)" || (echo avatar engine: the game side does not compile & exit /b 1)
"%PYTHON%" -I "%REPO%\tools\avatar\verify_engine_pins.py" --selftest || exit /b 1
"%PYTHON%" -I "%REPO%\tools\avatar\verify_engine_pins.py" || exit /b 1
"%PYTHON%" -I "%REPO%\tools\avatar\proof_puppet.py" --selftest || exit /b 1
echo avatar engine: ALL OK
exit /b 0
