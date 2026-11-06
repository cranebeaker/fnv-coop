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
python -I tools/coop-run/bench.py run --set full --profiles perfect,good_wifi,bad_wifi
python -I tools/coop-run/bench.py report <bench folder>
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
| `fake_viewer.py` | stand-in viewer with the same flags and logs, used to test the runner (and the live control port) |
| `bench.py` | sync benchmark on two real viewers: `run` runs a scenario set under several network profiles and samples both viewers on the control port 5 times a second; `report` writes `bench.md` and `bench.json` (position error and lag of NPCs and avatars, animation agreement, time until a shared value arrives, lost values, shared-hash lag and divergences, bandwidth, RTT, fps). `scenarios/bench_party_fight.json` is its long mixed scene |
| `live.py` | live loop: client library and CLI for the viewer's `--coop-control` port; `up`, `state`, `run`, `bot`, `keys`, `shot`, `entities`, `watch`, `down` |
| `remote/sync_and_build.ps1` | on the test PC: apply a git bundle under `C:\fnv`, fast viewer build, compact errors |
| `scenarios/*.json` | example scenarios (Doc Mitchell's house) |
| `fake_viewer.py` | stand-in viewer with the same flags and logs, used to test the runner |
| `scenarios/*.json` | example scenarios (Doc Mitchell's house). The player-world interaction ones are `live_barter`, `live_door_follow`, `live_pickup_world_item`, `live_lockpick_unlock_shared`, `live_craft_per_player` and `live_vendor_stock_shared`, each with a `twin_` mutation twin; their bot verbs (`coop barter`, `door`, `pickup`, `drop`, `steal`, `craft`, `use`, `equip`, `lockpick`, `hack`, `enter`, `give`, `setlock` ...) are listed in `viewer/src/coop_live/bot.rs`. The dump's `interact` object counts what each machine did (`interact.barter`, `interact.refused`, `interact.follows_in`, ...) and `objects` holds the shared locks, doors, terminals and mines. The same six run on a generated world in `crates/coop-headless/tests/interact.rs` |
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

Each peer writes its own rows under its own slot name: a third peer's own position is `positions.guest2`
in its own dump too. Story state, as each peer's game state holds it: `topics.<topic>`, `notes.<note>`,
`map_markers.<marker>.{shown,travel,found}` (the Pip-Boy map's rules), `karma`,
`reputations.<rep>.{fame,infamy}`, `factions.<faction>.<member>` (rank, the player is `player`),
`faction_relations.<faction>.<other>` (0 neutral, 1 enemy, 2 ally, 3 friend) and `crime`
(`factions.<faction>.{minor,major,enemy}`, `minor_total`, `major_total`, `enemies`, `player.{minor,major}`,
`murderer`, `steal_warnings`). `status` is the status line: `players` (this peer too), `peers.<slot>.{name,rtt_ms}`,
`rtt_ms`, `desync`, `desyncs`, `repairs`, `snapshots`, `line` and `compact` (the HUD texts).

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
* **Timeline.** Both peers ready (the runner then writes each peer's `--coop-go-file`, and every
  viewer starts its script clock at that moment, so `at` times count from the same instant), then
  `duration_s` seconds of chaos and script time, then the relay
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

## More than one guest (3 and 4 peer runs)

`"guests": [ {script, extra_args}, ... ]` (1 to 3 entries) replaces `"guest"`; `guest` stays the
first entry. The runner starts one host and one viewer per guest, each guest through its own chaos
relay (seed + 7919 x its number), waits for all of them to be ready and quits all of them. Checks
may say `"on": "guest2"` or `"guest3"` for a given peer, `"all"` for every peer, and `"equal"` then
compares the host with every guest (`id#guest2`). The automatic `hash.shared` check covers every
guest. Roles in the logs and verdict are `host`, `guest`, `guest2`, `guest3`. Guests 2 and 3 get
`--coop-name "Guest 2"` and `"Guest 3"`. `scenarios/live_four_viewers.json` (and its twin) is the
real-game four-viewer run for the test PC (four viewers fit in RAM there). The stand-in
`fake_viewer.py` speaks to one guest only, so the multi-process launch of several guests is proved
on the test PC; the parsing, command lines, checks and verdict are covered by `test_multi_guest.py`.
The headless twin runs the same shape in one process (`coop-sim`, `"guests"` too) and reports each
peer's bandwidth and CPU: `coop-sim budget` (see docs/COOP.md "Budget and four peers").

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

`remote\run_list.ps1` is the same as `run_suite.ps1` but reads the scenario names from a file, one per line
(`C:\fnv\coop-run\list-<tag>.txt`, `#` comments allowed) and `n` and `python` from `run-<tag>.json`. Use it for long
lists: a scheduled-task command line is cut at 261 characters, which turned a long `-Scenarios` list into a
truncated name (`No such file: ...live_rec.json`). Start it with
`task.ps1 -Script C:\fnv\coop-run\run_list.ps1 -Tag <tag>`; it names the first missing scenario file in `done-<tag>.txt`.
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

## Live loop: control port and `live.py`

For development, not for verdicts: two viewers stay running and a script drives them, so a change is checked in
seconds instead of a launch per run. See also `docs/FASTDEV.md` (rebuild times, `coop-sim watch`).

`--coop-control PORT` (needs `--coop-host`/`--coop-join`) opens `127.0.0.1:PORT` (never another address) on
a background thread (`crates/coop-session/src/control.rs`, std only); the viewer answers on its next frame
(`viewer/src/coop_control.rs`). One JSON object per line in, one per line out, in order. An optional `"id"` is echoed.
Every reply has `"ok"` and `"cmd"`; a failure has `"error"` and the connection stays open.

| Request | Reply (besides `ok`, `cmd`) |
|---|---|
| `{"cmd":"state"}` | `role`, `ready`, `cell`, `fps`, `t`, `npcs`, `npcs_visible`, `puppets`, `puppets_visible`, `counters` (`lines_played`, `chat_in`, `travels`, `hits_sent`), `state` = the whole `--coop-state-dump` document |
| `{"cmd":"run","line":"SetStage Q 10"}` | host (or `"local":true`, or a `local ` prefix): runs it at once, `how` = `host`/`local`, `flow` = the interpreter's result. Guest: sent to the host, `how` = `sent`. A `coop ...` line goes to the bot |
| `{"cmd":"bot","line":"teleport 1 2 3"}` | `how:"bot"`; the `coop ` prefix is optional; verbs as in `viewer/src/coop_live/bot.rs` |
| `{"cmd":"keys","key":"w","hold":2,"delay":0}` | queues the press like `--key-at` (`w`, `space`, `mouse-left`, `mouse-x=40`...); delay is seconds from now |
| `{"cmd":"screenshot","path":"C:/x/a.png"}` | replies after the PNG is written (the frame as the window shows it; the viewer keeps running) |
| `{"cmd":"entities"}` | `entities`: `form`, `name`, `kind` (`npc`, `remote` = owned by the other peer, `puppet` = the other player), `player`, `pos`, `heading`, `visible` (a mesh of it is in a camera's view), `anim`, `anim_names`; plus the four counts |
| `{"cmd":"quit"}` | replies, then leaves cleanly a few frames later (final dump and `shutdown` event as usual) |

`run` before the player is placed fails with `the player is not placed yet`. People are the viewer's `Walker`
entities; creatures that are not `Walker`s are not listed.

```
python -I tools/coop-run/live.py up --cell GSDocMitchellHouse [--visible] [--config F]   # host + guest, waits for both
python -I tools/coop-run/live.py state host [--json | --path items.player]
python -I tools/coop-run/live.py run guest "player.additem Caps001 5" [--local]
python -I tools/coop-run/live.py bot host "teleport 2400 1700 7360"
python -I tools/coop-run/live.py keys guest w --hold 2
python -I tools/coop-run/live.py shot host C:/tmp/host.png
python -I tools/coop-run/live.py entities guest --visible
python -I tools/coop-run/live.py watch tools/coop-run/scenarios/live_loot_sequence.json [--reset guest] [--once]
python -I tools/coop-run/live.py down
python -I tools/coop-run/runner.py run SCENARIO.json --attach host:PORT guest:PORT   # one replay, RESULT line, exit code
```

`up` uses the same config as the runner (`coop-run.local.json`: viewer, data, `background`, ports), joins the guest directly
(no relay), writes `<out_dir>/live/session.json` (ports, pids, commands, stdout files) and returns when both report
`ready`. Targets are `host`, `guest` (from that file), a port or `127.0.0.1:PORT`, so any viewer started by hand with
`--coop-control` works. `down` quits both through the port and only then ends the pids `up` recorded.

`watch` replays the scenario's script lines against the running pair after every save of the scenario file
(`run`, `stage`, `key` lines at their `at` seconds; startup flags `talk`/`choose`/`say` are skipped with a note),
then judges `expect` once the last command has gone out: PASS or FAIL per assertion, polling every 0.4 s until all
pass or `settle_s` runs out. Assertion paths see the state dump plus `live.*` (`live.fps`, `live.npcs_visible`,
`live.puppets_visible`, `live.counters.chat_in`, ...), so "the guest sees one visible puppet" is
`{"id":"pup","path":"live.puppets_visible","op":"ge","value":1,"on":"guest"}`. The pair is not restarted between replays:
the host keeps its state. Write live scenarios with absolute effects (`SetStage`, `set X to V`, a teleport) or `ge` bounds;
`--reset guest` quits and relaunches the guest first (the reconnect path, slow because it reloads the place).

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

`fake_viewer.py --fake MODE` modes: `crash hang no_sync double_apply silent no_connect error_event repair bad_exit`,
and for the control port `ctl_not_ready ctl_slow ctl_hidden`. With `--coop-control PORT` it speaks the live protocol
(`test_live.py` runs the client, replay, watch, `up`/`down`/reset and `runner.py --attach` against it).
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

## The start of a game, and the player scripts

`play/Host-Coop.cmd` and `play/Join-Coop.cmd` are templates for people (copy them next to
`nv-launcher.exe` and edit the `set` lines). They go through the launcher (`nv-launcher --start
host|join`), so the viewer command line, the save folders, the join code and the log are the
launcher's. The host starts a NEW GAME by default: the whole normal start (VCG00, Doc Mitchell,
the name box, the Vit-o-Matic, tags, traits). A guest joins any time and makes its own character.
Rules and the exact flow: docs/COOP.md, "The start of a game".

Scenarios: `new_game: true` gives both viewers `--new-game` (host: the opening; guest: "make my
own character", its cell is the host's place). Other scenarios give a guest `--coop-no-chargen`
unless they set `character_creation: true`, so existing runs are unchanged. The dump has a `start`
object per peer (`phase`: joining, character_creation, arriving, playing; `venue`, `opening`,
`sheet_acked`, `step`). `live_new_game_start` asserts the guest reaches `playing` with its own
`player_quests.guest.VCG00/VCG01` rows while the host is still in `character_creation`;
`twin_live_new_game_start` (guest `--coop-no-chargen`) must fail. The viewers need
`--answer-boxes` (the runner passes it): it accepts the name box, answers the tag skills, traits and
the Vit-o-Matic (spends the spare points on Strength, accepts) and the opening's message boxes.
coop-sim runs the same flow with the menus stood in for (`crates/coop-headless/tests/start.rs`).

`coop-sim watch [--scenarios DIR] [--seeds 1..5]` rebuilds and re-runs on every save of a `crates/coop-*`/`world`
source or a scenario file and prints a one-screen pass/fail table with timings (`docs/FASTDEV.md`).

## Not done here

* The viewer side of the contract (`viewer/src/coop`) is for the integrator; until then only the fake is exercised.
* The remote scripts were parse-checked but not run against the real test PC (no ssh from the build session).
* Relay seeds match the coop-core profile parameters but not its `SeededRng` stream.
