# Co-op benchmark

One scripted run of two game copies that returns a scorecard, instead of launching copies and reading logs by hand.

```
python -I tools/bench/coop_bench.py                       # boot 2 copies, load the save, run all phases (about 4 minutes of phases)
python -I tools/bench/coop_bench.py --runs 3              # three fresh boots; median per metric, spread and noise (about 25 minutes)
python -I tools/bench/coop_bench.py --no-boot             # use two copies that are already up, in the world, with the DLC boxes cleared
python -I tools/bench/coop_bench.py --quick               # shorter phases, a smoke run
python -I tools/bench/rescore.py out/bench/<run>          # re-analyse a recorded run offline with the current code and targets
python -I tools/bench/compare.py out/bench/<before> out/bench/<after>   # deltas per metric, exit 1 on a regression
```

Needs: the two instances provisioned (tools/runner), Steam running and signed in, DXVK installed in the instances (tools/runner/fetch_deps.py,
then provision), about 3 GB of free RAM, and the machine left alone while it runs (the bots drive the players).

## Scenarios: scripting a run

A run can be a short JSON script instead of the built-in phases: `python -I tools/bench/coop_bench.py --scenario standard` (or `smoke`, or a path).
Files live in `tools/bench/scenarios/`. Each step is `{"phase": ..., ...}`; an optional `"label"` renames the phase the sampler records, so each
step gets its own row of metrics.

| Phase | Fields | What it does |
|---|---|---|
| `warmup`, `idle`, `wait` | `secs` | Do nothing and measure |
| `walk` | `who` (host, client, both), `course` (square, line, figure8, circle), `secs` | The bots walk a named course |
| `probe` | | Does real movement input move a player? |
| `area` | | The host enters a building and comes back out; measures how long the puppet takes to catch up |
| `playbook` | `name` (fight, loot_race, any playbook in tools/bots) | Runs the repo's playbook and scores its steps |
| `net` | `profiles`, `secs`, `recover_secs` | Steps the client through the chaos relay profiles, then back to lan (starts the relay automatically) |

Add a phase type with `@register("name")` in `bench_scenario.py`. A phase that fails is recorded under `phase_errors` and the run carries on.
Courses and spawn rings come from `arena.py`: offsets from an origin, so a scenario never needs coordinates. `tools/bench/arena.json` can pin the
origin (and the `cow` cell) of a flat, open area; without it each course starts where the player stands.
`tools/bench/find_arena.py` looks for a flat, open exterior cell where real movement input works and writes `arena.json`; the `arena` phase then moves both
copies there at the start of a scenario. Both are written and parse-checked but have NOT been run in the game yet.
The scenario runner and the arena layout are tested without the game: `python -I -m unittest discover -s tools/bench/tests -p test_scenario.py`.

`real_fight` kills the way each side can: the host with the console `Kill` (it owns the world), the client by standing off and shooting (`bot.fire`): a
guest's console `Kill` on an actor it does not own is refused on purpose (guest gate), so a client console kill proves nothing. The shot goes to the
actor's owner, which applies it; `play.kill_latency_s` is the time from the killing volley until the host shows the actor dead.

## What a run does

1. Boots two copies (`run.py up`), loads the save on both, clears the DLC message boxes, waits 15 s (the warm-up is excluded from every number).
2. Runs the phases: idle, host walks a square, client walks a square, both walk, settle. The bots move by `step` mode by default (the bot sets
   positions in small increments: repeatable and independent of collisions). `--walk-mode input` uses real movement input instead.
3. A sampler thread reads both copies about four times a second through the bridge (read-only calls) and writes `samples.jsonl`.
4. Writes `results.json` and `report.md` in `out/bench/<time>/`, and one summary line to `out/bench/history.jsonl`.

## Metrics and scores

Each scored metric maps to 0..100 linearly between a GOOD and a BAD threshold (`TARGETS` in coop_bench.py). A category is the mean of its metrics,
the composite is the mean of the categories.

| Category | Metric | Good | Bad |
|---|---|---|---|
| performance | fps p5 (worst copy, from the plugin frame counter) | 55 | 15 |
| performance | share of samples under 30 fps (worst copy) | 0 % | 30 % |
| performance | bridge round trip p95 (how long the main thread takes to answer) | 40 ms | 400 ms |
| sync | message lag p95 between host and client (lastSeq) | 3 | 30 |
| sync | digest mismatches per minute | 0.5 | 6 |
| sync | repair effects per minute | 5 | 300 |
| avatars | distance between a player and the other copy's puppet of it, p95 | 200 | 1500 units |
| avatars | puppet teleports per minute | 0 | 10 |
| parity | actors with a confirmed host/guest difference, share of those compared (not in the composite) | 2 % | 25 % |
| parity | player puppets with a confirmed difference, share of those compared (not in the composite) | 2 % | 25 % |

The parity rows come from the plugin's own samples of both copies (docs/tools/parity.md); the copies need `--ini parity.enabled=1` when provisioned, and without the files that category is empty. It is shown next to the others but left out of the composite and of the sync number.

Reported but not scored: frame time p50 / p95 / p99, fps mean and p1, CPU, RAM and RAM growth per minute, network bytes per second, digest `ok`
count and `ever_converged`, sync latency (resolution about 0.25 s, so it reads 0 on a healthy link), walk legs that arrived and why not, and
the keys the digest still disagrees on (the host's last desync reports).

## Reading a result

- A change smaller than the run-to-run noise is not a change. Use `--runs 3` and look at the coefficient of variation of the composite.
  fps p5 in particular swings from run to run on this machine.
- `sync.ever_converged` false means the digest never matched: look at "Keys the digest still disagrees on" first.
- Avatar numbers are only meaningful when the walk legs arrived. In `input` mode most legs fail near fences; `step` mode always moves.
- fps and frame time come from a four-samples-a-second reading of a frame counter, not per-frame times. Puppet error includes the small gap
  between the two reads (a walking player covers about 55 units in 0.25 s).
- `GetSecondsPassed` is not a frame time; do not use it.

## Measured history (same save, same scenario)

| Run | Change | Composite | Notes |
|---|---|---|---|
| 1 | baseline | 28 | digest never matched, 1,429 repair effects per minute |
| 2 | client-only stage 0 keys settled by the host | 74 | 49 repair effects per minute, copy 1 fps p5 15 to 31 |
| 3 | same plugin, step-mode walking | 61 | avatar teleports now measured (6 per minute when the host walks), remaining digest keys listed |

## Not built yet

Fight, loot and quest-change phases, a cell-transition phase, a latency and loss matrix through the chaos relay (tools/chaos), and a marker-effect
probe for true sync latency.
