# coop-sim: what it is, how far to trust it

`coop-sim` runs the scenario files of `tools/coop-run` (unchanged) in one process, with no window, no Bevy and
no renderer, on a virtual clock. A host and a guest `GameState` are driven by the real
`coop_session::Session` and live layers over coop-core's seeded `ChaosTransport`, with the real ESM data
(quests, scripts, people, dialogue, health, essential flags, policy table). 60 s of game time takes well under
a second.

```
coop-sim run   SCENARIO.json [--seed N] [--net PROFILE] [--data DIR] [--out DIR] [--twice] [--fps N]
coop-sim suite DIR|FILE [--seeds 1..50] [--jobs N] [--net PROFILE] [--soak] [--data DIR] [--out DIR] [--twice]
               [--talk-at-ready] [--glue-fixed]
```

* `--data DIR` or `FNV_ESM_DIR`: the folder with the ESMs (the five masters in `games\fnv-esm` are enough
  for every shipped scenario; the test PC's full Data folder works the same).
* `--out DIR` writes `DIR/<scenario>/seed<N>/{verdict.json,host.jsonl,guest.jsonl,host.state.json,guest.state.json}`
  in the schema of `tools/coop-run` (`class`, `reason`, `rerun`, `checks`, `relay`, `peers`, a `packet` under
  3 KB on failure). Extra keys: `engine: "headless"` and `modelled` (the stand-ins this run used).
* The last line is `RESULT: PASS ...`, `RESULT: PASS twin ...`, `RESULT: FAIL <class> <scenario>: <reason> | rerun: ...`
  like `runner.py`. Exit code 0 pass, 1 fail, 2 usage or data error, 3 only skips.
* `--twice` runs every job twice and compares a hash of both final dumps (same seed must give the same
  bytes). `--soak` runs every scenario on all 7 chaos profiles; the summary has one line per scenario and
  profile with the pass rate and the first failing seed and its replay command. Threads are `std::thread`, one
  simulation per thread, the ESM data is shared.
* `needs_assets`: a scenario that needs something this folder lacks (a cell, a reference, a viewer flag other
  than `--walk`, `new_game`) is not run: `RESULT: SKIP needs_assets <scenario>: <reason>`, verdict `result: skip`.
  Nothing is guessed. Meshes, animations and navmesh are never read, so a scenario that depends on collision or
  pathing would have to be refused the same way; none of the shipped ones does.
* `tools/coop-run/compare_headless.py` prints the agreement table against real `verdict.json` files.

## What is real and what is modelled

| Part | Headless |
|---|---|
| Session, handshake, snapshots, deltas, hashes, repairs, live layer, policy gate, shadows, loot ledger, actor ledger, dialogue, rules, tether | the real code (`coop-session`, `coop-core`, `world::coop`) |
| Scripts, quests, `SetStage`, `AddItem`, `Enable`, `MoveTo`, health, essential flags, dialogue conditions | the real `world` crate on the real ESM data |
| `coop VERB` bot lines, `local ` lines, guest `run` lines becoming host commands, ready-relative clocks | mirrored from `viewer/src/coop.rs` and `coop_live/*` in `peer.rs`, `bot.rs` |
| The link | coop-core `ChaosTransport` per direction, seeded from the run seed; scheduled outages cut datagrams and hold the reliable stream; the profile turns `perfect` at `chaos_s` after the guest first talks |
| Movement | MODELLED. `coop teleport` is exact. `key` lines move the player in its facing direction at 308 units a second (the viewer's Always Run default, 77 x `fMoveRunMult` 4; the fake viewer uses 150), no collision, no acceleration. `key` times count from process start, `run` times from `coop_ready` |
| `--talk` | MODELLED. The viewer opens the conversation through its dialogue menu. The stand-in picks the nearest placed person with a greeting, opens a session on the first frame (as the viewer does once its scene is loaded), picks the greeting, reports the line done after 2.5 s and closes. `--choose` and `--say` are not modelled (noted in `modelled`) |
| Which people the host replicates | MODELLED. The viewer tracks the people it spawned; the simulator tracks the cell's people plus the actors a script names, once enabled and standing in the player's cell, checked once a second |
| Frame rate and process start | MODELLED. One fixed frame rate (40 fps, `--fps`), both peers start at second 0; real processes start seconds apart and run uneven frames |
| Container actions of the bot | the viewer's own path by default (host `loottake` does nothing, see GAPS.md); `--glue-fixed` uses `Session::live_loot_ops` |
| Rendering, audio, HUD, physics, AI movement | absent. Actors do not move unless a script moves them |

The simulator's relay is not the Python relay: its random stream is coop-core's `SeededRng` and the reliable
channel never loses (it only delays), where the Python relay treats every datagram as unreliable and relies on
the viewer's own reliability. A seed reproduces a simulator run exactly, not a real run.

## Calibration against the real viewer (test PC, 2026-11-02)

"Before" is the simulator as first run on this branch's parent code (before the fixes of GAPS.md), measured in
the session that wrote it. "Now" is this branch.

| Scenario | Real viewer | Headless before | Headless now | Note |
|---|---|---|---|---|
| doc_stage_sync | pass | pass | pass | |
| doc_stage_bad_wifi | pass | pass | pass | |
| doc_player_quest_sync | pass | pass | pass | |
| doc_positions | pass | pass | pass | teleport is exact |
| live_avatars_visible | pass | pass | pass | needed the run speed (308, not 150) to reproduce; `modelled` |
| live_guest_kills_npc | pass | pass | pass | needed host tracking of a script-enabled actor from another cell |
| live_reconnect | pass | pass | pass | |
| live_loot_race | FAIL: sum 0 | FAIL: sum 0 | FAIL: container keeps 54 | same verdict, causes in GAPS.md; the first one (stacks thinned away) is fixed, the rest are the scenario's design and the viewer bot |
| live_dialogue_session | FAIL: lines_played 0 | FAIL: lines_played 0 | pass | reproduced by the stand-in talking on the first frame; fixed in `Session::live_intent` / `GuestSync` (the viewer on the test PC predates it) |
| live_tether_warp | FAIL: guest stays at 9000 | FAIL: 9000 | FAIL: 9000 | tether timeout is 120 s, the scenario lasts 51 s; `scenarios/live_tether_warp_long.json` passes |
| the 10 twins | fail as expected | same | same | |
| doc_flaky_items | not run | pass | pass | |
| live_guest_hits_essential | not run | FAIL | FAIL | Doc Mitchell is not essential in the data |

10 of 10 shipped verdicts reproduced before the fixes; 9 of 10 now, the tenth being the dialogue scenario that
this branch fixes. `compare_headless.py --known` prints this table from the simulator's side and counts that
scenario as "fixed since".

Where the headless model is weaker than the viewer, a run says so in `modelled`; `live_avatars_visible` and
`live_dialogue_session` carry such entries, so their `pass` means "passes with this stand-in". Whether the
dialogue stand-in is as late as the viewer's real menu (the first-frame open is a reading of
`intents::tap_dialogue`, not a measurement) is the one calibration point only a viewer run can settle.

## Speed (release build, 4 threads, `FNV_ESM_DIR` = the five masters)

* all 22 shipped scenarios, each run twice with identical dumps: 4.4 s wall including 0.5 s to open the ESMs
  (627 virtual seconds per pass);
* soak, 22 scenarios x 7 profiles x 20 seeds = 3080 runs, 88581 virtual seconds in 81 s wall (about 1100 times
  real time);
* the first run in a process pays the ESM index (about 12 s in a debug build, under 1 s in release).

Soak result: every scenario passes 20 of 20 seeds on all seven profiles except the three that fail by design
(`live_loot_race`, `live_tether_warp`, `live_guest_hits_essential`, 0 of 20 everywhere, deterministic).
`crates/coop-headless/scenarios/` holds the corrected loot and tether scenarios; with `--glue-fixed` they pass
210 of 210 runs (7 profiles x 10 seeds x 3 scenarios).

## Tests

`cargo test -p coop-headless`: scenario parsing and validation, every assertion op, verdict classes and
packet size, the link's outage and profile switch, and end-to-end runs on a generated world (no game data):
movement, a guest kill through `bad_wifi` and an outage, twins, determinism, `needs_assets`, the thread pool and
the soak summary, and 30 seeds of a regression for the join race.
