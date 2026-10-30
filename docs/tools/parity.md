# Live sync parity checker

The benchmark scores each machine on its own. It never asked the question a player asks: "does my friend's screen show what mine shows?". The parity
checker answers it during a real session. Every machine records what it sees around its own player, the host compares its view with each guest's, and
the same data can be compared offline afterwards.

It is read-only. It reads the game through the avatar layer's backend (a second `WinBackend` with no console, so it cannot run a script line) and the
puppet directory; it writes nothing into the game. Nothing here has run in a game yet; the logic behind it is tested without the game.

## What is recorded

Every `intervalMs` (2 s) each machine takes one sample of its local view, while a co-op session is up:

| Entity | What | Where it comes from |
|---|---|---|
| Actors near the local player (nearest 40 within `radius`, 6000 units) | reference id, base, position, speed, seated, dead, disabled, health bucket, primary animation group | the loaded cells' actor lists, `ReadRef`, `ReadAnim`, health through the condition evaluator, `BaseProcess::GetSitSleepState` (as in plugin/animprobe) |
| Every remote player's puppet | the same, plus race, sex, hair, eyes, face-data hash (the avatar layer's `ReadAppearance`), and `acts`: how many attack animations started on it | the avatar layer's puppet directory |
| The local player ("self") | the same, with the same look fields and `acts` | reference 0x14 |

`acts` is counted every frame (about 30 Hz), because a punch lasts less than a sample interval. It is how "remote punches not showing" becomes a number:
the owner's own count against the count its puppet showed on the other machine.

Samples go to `Data\NVSE\Plugins\nvcoop_parity.jsonl`, one JSON line each (format in `plugin/parity/parity_json.h`), with the game time, a monotonic
millisecond stamp and the wall clock. At 8 MB the file moves to `nvcoop_parity.jsonl.1` and a new one starts; at every game start an older file is moved
to `.1` too, so `.1` is normally the previous run. The file holds positions, reference numbers and form ids only: no character names.

## In game (the host compares)

A guest sends every second sample (`sendEvery=2`, so every 4 s) to the host on channel `0x9E` (reliable, about 600 bytes, nearest 30 actors; section 23 of
docs/protocol.md). The host takes a fresh sample of its own the moment the guest's arrives, so both describe nearly the same moment (the assumed skew is one
way latency, 0.15 s), writes it to its file as a `"why":"cmp"` line that names the guest sample (`cmp.slot`, `cmp.seq`), and compares. Differences go to the
host's `nvcoop.log`:

```
parity: DESYNC <kind> ref <id> host=<what the host sees> client=<what the guest sees> slot=<guest slot>
parity: summary N refs compared, M mismatching slot=<guest slot>
```

`<id>` is an actor reference (`0x0010A1B2`) or a player slot (`player1`). One DESYNC line per reference and kind every 10 s; the summary comes every 30 s and counts
references (not differences): N distinct references compared in the window, M of them with at least one confirmed difference.

| Kind | A difference is | Excused when |
|---|---|---|
| `pos` | the two positions are further apart than 210 units (about 3 m) plus the average speed times 1.05 s | moving entities get the wider tolerance; more than 1400 units is reported at once |
| `seated` | one side sits or sleeps (or is going into furniture), the other does not | must last 3 s |
| `anim` | the primary animation class differs: idle, walk, run, death | one-shot groups (attacks, reloads, flinches, jumps, dodges, draw/holster, turns) are never compared; must last 3 s |
| `dead` | one side dead, the other alive | must last 3 s; dead actors are not compared further |
| `hp` | health buckets (25 hp each) two or more apart | must last 3 s |
| `disabled` | enabled on one side, disabled on the other | must last 3 s |
| `look` | a player's own look against its puppet: race, sex, hair, eyes, face hash; a puppet with no look at all | must last 3 s |
| `action` | attack starts seen on the owner against the starts its puppet showed differ by 2 or more | the gap at first sight is the baseline, so a puppet built late does not count the old swings; must last 3 s |
| `absent` | an actor (or player) one machine lists is missing on the other | only inside the other sample's coverage (`cov`: its radius, or the distance of the nearest actor the cap cut off); must last 3 s |

A round is skipped (nothing compared) when either side had a menu open or the two players are in different cells or worldspaces. Actors created during play (reference
`0xFFxxxxxx`) have a different local reference on each machine and are not compared. A difference "lasts" when it is seen again on a later round at least `persistMs`
(3 s) after it first appeared; a round without it starts the clock over.

## Offline

```
python -I tools/parity/compare.py HOST_FILE CLIENT_FILE        # the two nvcoop_parity.jsonl files of one run, in any order
python -I tools/parity/compare.py out/bench/<run>               # a benchmark folder (parity_host.jsonl, parity_client.jsonl)
python -I tools/parity/compare.py a.jsonl b.jsonl --json        # what the benchmark reads
```

It pairs each host `cmp` line with the guest line it names (no clock agreement needed; without links it falls back to the wall clock within 1.5 s), runs the same
rules as the plugin (`tools/parity/vectors.json` pins both to the same answers, including the animation class of all 256 group ids), and prints:

```
Parity report
  pairing: link, 61 pairs over 244 s of host time; 61 rounds compared, 0 skipped
Overall: 1830 references compared, 41 mismatching (2.2 %)  [actors 1769 / 30 (1.7 %), players 61 / 11 (18.0 %)]
kind          checks      bad    rate
anim             ...
Worst offenders (confirmed in rounds / rounds the reference was compared):
  0x000F2A31   seated      18/20     12s..240s  host=standing client=seated
```

Read it from the bottom: the worst offenders are the references to look at in game (open the console, click the reference or `prid` it). The per-kind rate says which
layer to blame: `look` and `action` on `player1` are the avatar layer, `seated` and `anim` on actors the NPC layer, `pos` on a moving player the tether or the stream. Use `--rotated` to
include `<file>.1` for a run longer than 8 MB of samples; use `--net-cap 0` to compare the whole file sample instead of what fits the datagram.

## In the benchmark

`tools/bench/coop_bench.py` copies each local copy's file into the run folder (`parity_host.jsonl` from copy 0, `parity_client.jsonl` from copy 1) and adds a `parity` row of the
scorecard next to avatars, sync and gameplay:

| Metric | Good | Bad |
|---|---|---|
| `parity.npc_mismatch_pct`: actors with a confirmed difference, share of those compared | 2 % | 25 % |
| `parity.puppet_mismatch_pct`: player puppets with a confirmed difference, share of those compared | 2 % | 25 % |

The parity category is shown but **not part of the composite** (`NOT_IN_COMPOSITE` in coop_bench.py), so every earlier composite and the sync number stay comparable; remove it from that
tuple once a few runs have set a baseline. `rescore.py` scores the saved files again with the current rules. The copies must be provisioned with `--ini parity.enabled=1` (the player release
ini has it on); without files the parity score is empty and nothing else changes. When the host copy runs on the second PC (`--remote`) its file is not copied: run `compare.py` on both files by hand.

## Settings

`[parity]` in nvcoop.ini (code default off, release ini on); every key and its range is documented at the top of `plugin/parity/parity_config.h`:
`enabled`, `intervalMs`, `sendEvery` (0 = file only), `radius`, `cap`, `netCap`, `file`, `maxFileMb`, `posTolUnits`, `posHardUnits`, `lagSeconds`, `persistMs`, `logEveryMs`, `summaryMs`.
`NVCOOP_PARITY_<KEY>` in the environment beats the ini. Bridge methods: `parity.status` (counters, per-kind checks, the last summary), `parity.sample` (one sample now as a JSON line).

## Tests

`plugin\parity\run_tests.cmd` builds `nvcoop_parity_tests`, runs the suite and 28 mutation twins (FNV_MUT 1800..1827, each must be caught), then `tools/parity/run_tests.py` (the offline
comparer, the benchmark hook, 8 twins, and that `vectors.json` is current: `python -I tools/parity/gen_vectors.py` rewrites it). `python -I tools/lint/check_parity_wiring.py [--self-test]`
checks the wiring that no unit test can run: the module is in the build, the frame call and bridge methods exist, the release ini switches it on, the channel byte is unique,
every twin is run, and the engine source only calls read-only backend functions.

## What is not proven

* Nothing has run in the real game. The engine reads are the ones the avatar layer and plugin/animprobe already use, but the sit-state byte, the health through the condition evaluator and the
  face-data hash have not been looked at through this checker. The face hash is the avatar layer's own ("TODO-RE: FaceGenData layout unproven"): if it differs between a player and its puppet by
  construction, every puppet will show a `look` difference; read that before blaming the look code.
* Only the host compares in game. A guest's log has no DESYNC lines; a third player's puppet on a guest is compared only offline, from that guest's file against the host's.
* Positions are compared as raw numbers without interpolation state, so a fast mover is excused by speed, not by how the puppet is actually interpolated; thresholds (210 units, 3 s, 2 buckets) are first guesses to be tuned on real runs.
* Actors the actor layer creates during play (`0xFFxxxxxx`) are not compared at all.
