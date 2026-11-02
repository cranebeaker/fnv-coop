# coop-run: scripted two-process co-op runs, judged by a script

No human playtests. A scenario file says what the host and the guest do and what the shared
state must look like at the end. `runner.py` starts a host viewer and a guest viewer, puts a
seeded chaos relay between them, reads their logs and state dumps, and writes one
`verdict.json` per run. Python 3 standard library only; run everything with `python -I`.

```
python -I tools/coop-run/runner.py run     tools/coop-run/scenarios/doc_stage_sync.json
python -I tools/coop-run/runner.py clean   tools/coop-run/scenarios/doc_stage_bad_wifi.json --n 3
python -I tools/coop-run/runner.py suite   tools/coop-run/scenarios --n 3
python -I tools/coop-run/runner.py selftest
cd tools/coop-run && python -I -m unittest        # the harness's own tests, no game needed (about 70 s)
```

The last output line is `RESULT: PASS ...` or `RESULT: FAIL <class> ...`. Exit code 0 pass, 1 fail,
2 config or scenario error.

## Files

| File | What it is |
|---|---|
| `runner.py` | launches relay, host, guest; waits; judges; `run`, `clean`, `suite`, `selftest` |
| `scenario.py` | scenario load and validation, viewer command lines, end-state assertions |
| `verdict.py` | log parsing, failure classes, failure packet, scorer self-test |
| `chaos_relay.py` | seeded UDP relay with the coop-core profiles |
| `fake_viewer.py` | stand-in viewer with the same flags and logs, used to test the runner |
| `scenarios/*.json` | example scenarios (Doc Mitchell's house) |
| `remote/*.ps1` | scripts that run ON the test PC |
| `controller/*.ps1` | scripts that run on the PC you work at and drive the test PC |
| `test_*.py` | unit and end-to-end tests |
| `config.example.json` | copy to `coop-run.local.json` (git-ignored) |

## Config

Copy `config.example.json` to `coop-run.local.json` next to `runner.py` (or pass `--config`).

| Key | Meaning |
|---|---|
| `viewer` | the viewer exe, or a list: exe plus leading arguments (the tests use `[python, -I, fake_viewer.py]`) |
| `data` | the game's Data folder, passed as the viewer's first argument |
| `out_dir` | where run folders go (default `tools/coop-run/out`, git-ignored) |
| `base_port` | first UDP port tried for the host (default 27960); the relay takes the next free one |
| `launch_timeout_s` | how long to wait for the host to listen and for both peers to be ready |
| `grace_s` | how long a viewer gets to exit after the quit request before the runner stops it |
| `background`, `screen_size` | adds `--background --screen-size W,H` so test windows stay out of the way |

Paths in public files use `<game dir>` and `%USERPROFILE%`, never a real user name.

## The viewer contract (what the viewer integrator implements)

The runner uses the existing flags (`--run`, `--run-at`, `--stage`, `--talk`, `--choose`,
`--say`, `--character`, `--new-game`, `--background`, `--screen-size`, `--no-movies`, `--no-hud`)
and needs these new ones. `fake_viewer.py` implements all of them and rejects unknown flags,
so it is the executable reference.

| Flag | Meaning |
|---|---|
| `--coop-host PORT` | start as host: listen on UDP `127.0.0.1:PORT` (bind on any interface is fine) |
| `--coop-join ADDR` | start as guest: join the host at `ADDR` (`HOST:PORT`; in tests this is the chaos relay) |
| `--coop-test-token` | both roles: use the fixed session token. Required with a plain `HOST:PORT`; real hosts use a random token carried by their `NVRS-` join code (the builder in `scenario.py` always adds it) |
| `--coop-log FILE` | write the JSONL event log here (below), flushed per line |
| `--coop-state-dump FILE` | write the shared-state JSON here (below) |
| `--coop-quit-file FILE` | when this file appears, write a final state dump, log `shutdown`, exit 0. Checked at all times, also before the session is ready |
| `--coop-quit-after SECONDS` | backstop: same clean exit this many seconds after `coop_ready` |

Behaviour rules:

1. **Script clock.** In co-op mode the clock of `--run-at` (and the time `--run`, `--stage`, `--talk`
   take effect) starts at `coop_ready`, not at "loaded". Host: first guest connected. Guest: joined and
   the join snapshot applied. That puts both peers' scripts on nearly the same clock.
2. **Single player fallback is forbidden.** If `--coop-host`/`--coop-join` is given and the session cannot
   be established, never log `coop_ready`. The runner then reports class `hollow`.
3. **State dump.** Write atomically (write a temp file, rename), once per second, and a final time at exit.
4. **Exit code** 0 after a clean shutdown. Any other code is a crash to the runner.
5. Everything the guest does that changes shared state must go through the same path a player's input would.

### Event log (JSONL, one object per line, `t` = seconds since process start)

| `ev` | Fields | Needed by the runner |
|---|---|---|
| `start` | `role`, `pid` | yes (else `hollow`) |
| `loaded` | `cell` | no |
| `coop_listening` | `port` (host) | yes, the runner starts the guest after it |
| `coop_connected` | `role`, `peers` | no |
| `coop_ready` | `role`, `peers` | yes |
| `script` | `line` | no |
| `state_dump` | `path`, `seq` | no |
| `coop_repair` | `kind`, `subsystem` | counted against `max_repairs` |
| `error` | `msg` | any error event fails the run (class `log`) |
| `shutdown` | `reason`, `clean`, `code` | yes: `clean: true` or the peer counts as crashed |

### State dump (JSON object)

```json
{ "schema": 1, "role": "host", "t": 31.2, "seq": 31,
  "quests":    { "VCG03": { "stage": 10, "running": true, "completed": false } },
  "player_quests": { "host": { "VCG01": { "stage": 10, "running": true, "completed": false } } },
  "globals":   { "GameHour": 9.5 },
  "items":     { "host": { "Caps001": 100 }, "guest": { "Caps001": 125 } },
  "positions": { "host": [x, y, z, heading], "guest": [x, y, z, heading] },
  "disabled":  { "SomeRef": true },
  "hash":      { "shared": "9f2c..." } }
```

Keys are editor IDs where the record has one, otherwise 8-digit hex form IDs. `items` is keyed by
the owning peer (`host`, `guest`) and then by item; other holders may be added. With the live layer
(per-player shadows) `items.host` and `items.guest` are each player's own inventory, the same on both
peers; later guests are `guest2`... `positions` holds the avatars of both peers as each side sees them:
its own from the game state, the other from the live layer (`[x, y, z, heading]`). Also `dead` (dead
actors) and `live` (`avatars`, `lines_played`, `chat_in`, `travels`, `hits_sent`). `hash.shared` is a hash of the shared subset only, with
per-machine noise (clocks, local refs, padding) excluded; the runner fails any run where host and
guest differ (class `desync`) unless the scenario sets `"hash_check": false`. Extra keys are ignored,
so the integrator can add more paths and assert on them.

### Script prefixes for the live layer

A guest's `run` lines normally go to the host. Two prefixes change that (both in `run` and timed `run`):

* `local LINE` runs on the machine that has it (e.g. `local player.additem X 1`). Do not use `player.setpos` to move an avatar: it only edits the script state and the viewer re-reads the physical player every frame, so nobody moves. Use `coop teleport` or `key`.
* `coop VERB ...` is the test bot, run on the machine that has it: `hit REF N`, `talk REF`, `pick INFO`,
  `linedone`, `close`, `lootopen C`, `loottake C ITEM N`, `lootput C ITEM N`, `lootclose C`, `say TEXT`,
  `travel MARKER`, `tether REF`, `untether`, `revive SLOT`, `unrevive`, `teleport X Y Z` (puts this machine's physical player at those feet coordinates through the same path as a MoveTo arrival; the live layer then publishes the new avatar position). REF, INFO, ... are editor IDs or hex
  form IDs. `fake_viewer.py` implements only `teleport` (and the keys below), not the rest.

## Scenario format

```json
{ "name": "doc_stage_bad_wifi", "description": "...",
  "cell": "GSDocMitchellHouse",   "new_game": false,   "character": "characters/x.txt",
  "network": "bad_wifi",          "seed": 21,
  "duration_s": 20, "settle_s": 8, "timeout_s": 300,
  "outages": [ { "at": 6, "seconds": 3, "dir": "both" } ],
  "max_repairs": 5,
  "host":  { "script": [ { "at": 3, "stage": "VCG03 10" }, { "at": 7, "run": "set GSMyGlobal to 3" } ],
             "extra_args": [] },
  "guest": { "script": [ { "at": 5, "run": "player.additem Caps001 25" } ] },
  "expect": [ { "id": "stage", "path": "quests.VCG03.stage", "op": "eq", "value": 10, "on": "both" },
              { "id": "same",  "path": "hash.shared", "on": "equal" } ] }
```

* `cell` or `new_game` is required. `network` is a profile name (below). `seed` drives the relay.
* **Script lines**, one key each: `run` (a script-language line; `--run`, or `--run-at` with `at`),
  `stage` (`"QUEST N"`; `--stage`, or `--run-at at "SetStage QUEST N"`), `talk`, `choose` (list), `say`
  (startup flags, no `at`), `key` (needs `at`; `{"at": 3, "key": "w:2"}` becomes `--key-at 3 w:2`: a key or mouse
  button held that many seconds, `KEY[:HOLD]`; real input, so add `"extra_args": ["--walk"]` to walk).
* **Timeline.** Both peers ready, then `duration_s` seconds of chaos and script time, then the relay
  switches to `perfect` (`chaos_s` overrides when), then `settle_s` seconds of quiet, then the quit request.
  The final state dump is compared only after that quiet window.
* `outages` cut the relay for a while (`both`, `g2h` guest to host, `h2g`), seconds after the first guest datagram.
* `expect` checks run on the final dumps. `on`: `host`, `guest`, `both` (one result per peer), or `equal`
  (host value must equal guest value, optional `tol`). `op`: `eq ne ge le gt lt near present absent dist_from sum`;
  `near` needs `tol` and compares number lists on their common prefix, so `[x,y,z]` works against
  `[x,y,z,heading]`. `path` is dotted into the dump. Optional `dims: 2` on `near`, `equal` and the comparison
  ops keeps only the first two list components (the XY plane).
  `dist_from` takes `from: [x, y(, z)]`, `cmp` (`ge le gt lt`) and `value`: the XY distance in game units between
  the path's position and `from` (a missing path fails). `{"path": "positions.host", "op": "dist_from",
  "from": [2352, 1603.6], "cmp": "ge", "value": 150}` fails for an avatar that never left the spawn point.
  `sum` adds `path` and every path in `add` (missing counts 0) and compares with `cmp` (`eq ge le gt lt`) and `value`.
* `expect_fail: true` makes a **mutation twin**: the same scenario with the feature removed (`scenarios/twin_*.json`).
  The runner inverts the verdict: an `assert` or `desync` failure is the expected outcome and reports
  `RESULT: PASS twin ...`; a twin whose assertions all hold reports `FAIL twin` (the assertions are vacuous). Crashes,
  hollow runs and timeouts are never hidden by it. See `AUDIT.md` for what each assertion proves.
* `max_repairs`: allowed `coop_repair` events across both peers. Default 0 on a `perfect` link, unlimited otherwise.

## What the verdict says

`verdict.json` per run, in `<out>/<scenario>/<stamp>-<n>/` next to `host.jsonl`, `guest.jsonl`, both state
dumps, `relay.stats.json`, `relay.log.jsonl` (per-datagram log) and the viewers' stdout.

* `result`: `pass` or `fail`. `class`: first matching of `harness`, `launch`, `crash`, `timeout`, `log`,
  `hollow`, `desync`, `assert`, `repair`. `reason`: one line. `rerun`: the exact command with the seed.
* `checks`, `relay` (per direction in/forwarded/dropped/duplicated/reordered), `peers` (exit codes, ready,
  clean shutdown, repair counts).
* `packet` (failures only): a self-contained failure packet kept under 3 KB of JSON: class, reason, the first
  failing checks, relay counters, exit codes, the last events of each peer, the last stdout bytes.

Hollow-pass guards: both peers must reach `coop_ready`, write a state dump with the right role, and the relay
must have seen traffic in both directions. If a profile should have dropped at least 8 datagrams and dropped
none, or a scheduled outage never fired, the run is `hollow`. A run where nothing proves co-op happened
never passes.

**N=3 clean runs rule.** `clean` and `suite` run a scenario three times in a row (same seed unless
`--vary-seed`); a scenario is clean only if all three pass. The first failure stops it, and the summary
(`clean-<stamp>.json`) says on which run it failed. A failure after earlier passes is a flake: rerun with the same
seed; if it repeats it is a sync bug, if not say so in the report. `suite` first runs the **scorer self-test**
(known-bad runs fed to the judge must come out FAIL) and refuses to start if the scorer is broken.

## Chaos relay

`chaos_relay.py` listens on a port, forwards to the host, and applies a seeded impairment per direction.
Profiles have the names and numbers of `crates/coop-core/src/transport/profile.rs`:

| Profile | Latency + jitter | Loss | Dup | Reorder (+extra) | Outage |
|---|---|---|---|---|---|
| `perfect` | 0 | 0 | 0 | 0 | none |
| `good_wifi` | 15 + 0..5 ms | 0.5 % | 0 | 0 | none |
| `bad_wifi` | 50 + 0..40 ms | 5 % | 1 % | 5 % (+80 ms) | none |
| `mobile` | 90 + 0..60 ms | 3 % | 0.5 % | 3 % (+120 ms) | none |
| `congested` | 150 + 0..100 ms | 10 % | 2 % | 10 % (+200 ms) | none |
| `flaky` | 40 + 0..30 ms | 25 % | 5 % | 15 % (+100 ms) | none |
| `partition` | 30 + 0..10 ms | 2 % | 0 | 0 | dead 1.5 s at the end of every 6 s |

The relay works on raw datagrams, so every datagram is treated as unreliable. The viewer's own reliability
layer is what is being tested. The random stream is Python's, so a seed reproduces a relay run, not a coop-core
simulation. Standalone use: `python -I chaos_relay.py --listen 27961 --target 127.0.0.1:27960 --profile bad_wifi
--seed 7 --stats s.json --log l.jsonl --stop-file stop`. Windows ICMP "connection reset" errors on UDP are ignored.

## Running on the second test PC

The game windows need a logged-in desktop, which an ssh session does not have, so the run is started through a
scheduled task in that desktop session and the results are copied back. One command from this PC:

```powershell
$env:COOPRUN_REMOTE = 'dino'     # ssh alias of the test PC (default 'dino')
powershell -File tools\coop-run\controller\run_remote.ps1 -Scenarios doc_stage_sync,doc_stage_bad_wifi -N 3 `
    -Viewer C:\fnv\nv-rs-coop\target\release\nv-viewer.exe -Data "C:\fnv\Fallout New Vegas\Data"
```

What it does, in order:

1. `controller\push.ps1` zips the harness (no local config, no `out\`), uploads `remote\install.ps1` and the zip,
   and runs the installer, which unpacks into a new `C:\fnv\coop-run\v<stamp>` and points `current.txt` at it.
   Old versions stay; nothing is overwritten or deleted.
2. Generates the remote config (viewer, Data) and the run settings locally and uploads them as files.
3. `remote\preflight.ps1` (read-only): viewer and Data exist, free RAM, a desktop user is logged in, and no
   `nv-viewer` process is already running (it refuses to start if one is; it never touches it).
4. `remote\task.ps1` creates a one-shot scheduled task for `remote\run_suite.ps1` as the logged-in user with `/IT`,
   runs it, and deletes only the task it created. `run_suite.ps1` runs `runner.py suite ... --n N` and writes
   `C:\fnv\coop-run\done-<tag>.txt` when finished.
5. Polls `remote\status.ps1`, then `scp -r` copies `C:\fnv\coop-run\results\<tag>` to `tools\coop-run\out\remote-<tag>`.
   Exit code 0 only if the suite printed `exit=0`.

The test PC needs Python 3.11 (path via `-Python`, default `C:\Program Files\Python311\python.exe`), the built
viewer, and a Data folder. Several `-Scenarios` run one after another, never in parallel, because two copies of the
game already fill the PC.

`remote
un_list.ps1` is the same as `run_suite.ps1` but reads the scenario names from a file, one per line
(`C:nv\coop-run\list-<tag>.txt`, `#` comments allowed) and `n` and `python` from `run-<tag>.json`. Use it for long
lists: a scheduled-task command line is cut at 261 characters, which turned a long `-Scenarios` list into a
truncated name (`No such file: ...live_rec.json`). Start it with
`task.ps1 -Script C:nv\coop-run
un_list.ps1 -Tag <tag>`; it names the first missing scenario file in `done-<tag>.txt`.
`push.ps1` uploads it with the other remote scripts.

### Hard safety rules (encoded in the scripts, checked by `test_safety.py`)

* Never `del`, `rd /s`, `rmdir /s`, `rm -r` or `Remove-Item -Recurse` over ssh, or anywhere in these scripts. The
  scripts contain no delete or kill command at all; the one scheduled task is removed with `schtasks`, by its own name.
* Remote writes only under `C:\fnv\`. Every remote command starts with `cd /d C:\fnv`, every remote script starts
  with `Set-Location C:\fnv`, and `Assert-RemotePath` refuses any other path or any `..`.
* Remote scripts are uploaded as files and run with `-File`. No hand-escaped one-liners; arguments are limited to
  `[A-Za-z0-9_.,:\/=-]`.
* Never stop a process the run did not start. The runner holds the `Popen` handle of each process it started and
  only ever stops those. There is no stop-by-name anywhere.
* No simulated mouse or keyboard input, and no desktop capture. Viewers are driven by flags and scripts, logs and
  state dumps are the evidence.
* This harness never runs ssh by itself; you start `run_remote.ps1`.

## Tests

`cd tools/coop-run && python -I -m unittest` (unittest discovery adds the folder to `sys.path`, which `-I` otherwise
removes). Covered:

* the profile table against the coop-core numbers, the partition-window vectors, seeded reproducibility, loss,
  duplicate and reorder rates, real UDP round trips and drops through the relay;
* scenario validation, viewer argument building (also parsed by the strict fake), assertion ops;
* end-to-end runs of runner + relay + `fake_viewer.py`: a pass on a clean link and under `flaky` with an outage,
  and **mutation twins**, each a broken viewer the runner must catch with the right class: guest that never syncs
  (desync/assert), host that applies intents twice (assert), crash, hang (timeout), silent (hollow),
  never joins (hollow), error event (log), repair spam on a perfect link (repair), bad exit code (crash), missing
  exe (launch), bad flag; the failure packet stays under 3 KB; the 3-run clean rule and its stop at first failure;
* the scorer self-test, and source scans: no delete or process-kill calls, remote scripts only mention `C:\fnv`,
  no private identifiers, no emoji, no em dashes.

`fake_viewer.py --fake MODE` modes: `crash hang no_sync double_apply silent no_connect error_event repair bad_exit`.
It speaks a small resend-until-acked UDP protocol so the chaos profiles genuinely stress it. It supports the script
lines `SetStage Q N`, `set X to V`, `player.additem ITEM N`, `coop teleport X Y Z` and the `--key-at` keys w a s d
(150 units a second). Both avatars start at the spawn point [2352, 1603.6, 7360]. `player.setpos` is logged as
`setpos_ignored` and moves nobody, like the real viewer.

## Headless twin: `coop-sim`

`crates/coop-headless` (`coop-sim run|suite`) reads the same scenario files and runs them in one process, with
the real session and live layers over coop-core's chaos transport on a virtual clock and the real ESM data:
seconds instead of minutes, many seeds, all seven profiles (`--soak`), and a determinism check (`--twice`).
It writes `verdict.json` in the schema above and prints the same `RESULT:` lines. A scenario it cannot run on
the data it was given is `RESULT: SKIP needs_assets`, never a guess. `compare_headless.py` prints a table of
agreement with real `verdict.json` files (`python -I tools/coop-run/compare_headless.py <out dir>`, or
`--known` for the 2026-11-02 test PC results). What is modelled and how far to trust it:
`crates/coop-headless/CALIBRATION.md`; what it found: `crates/coop-headless/GAPS.md`.

## Not done here

* The viewer side of the contract (`viewer/src/coop`) is for the integrator; until then only the fake is exercised.
* The remote scripts were parse-checked but not run against the real test PC (no ssh from the build session).
* Relay seeds match the coop-core profile parameters but not its `SeededRng` stream.
