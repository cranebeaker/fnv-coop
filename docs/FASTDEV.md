# Fast co-op development loop

Goal: find a co-op bug in seconds, not minutes. Three layers, quickest first.

| Layer | What | Feedback |
|---|---|---|
| 1. Headless watch | `coop-sim watch`: rebuild the simulator and re-run the scenarios on every save | about 10 to 20 s per edit of a `coop-*` crate, table on one screen |
| 2. Live viewers | `tools/coop-run/live.py`: a host and a guest viewer stay running, driven over a control port | under 1 s per command, a scenario replay in the scenario's own seconds |
| 3. Fast viewer rebuild | `cargo build --profile play-quick` on the test PC through `sync_and_build.ps1` | one incremental viewer build (see "What is measured" below) |

Use layer 1 for session, core and world rules (no window, no game data beyond the ESMs), layer 2 for
anything that needs the real viewer (puppets on screen, animation, the Bevy glue), layer 3 only when
the viewer code itself changed.

## 1. Headless watch

```powershell
$env:CARGO_TARGET_DIR = 'C:/path/to/shared-target'   # one target dir for every cargo call, see below
$env:CARGO_BUILD_JOBS = '2'
cargo build -p coop-headless --bin coop-sim           # once; this is the watcher
& "$env:CARGO_TARGET_DIR/debug/coop-sim.exe" watch --seeds 1..5 --data 'C:/path/to/Fallout New Vegas/Data'
```

`FNV_ESM_DIR` replaces `--data`. Options: `--scenarios DIR` (default `tools/coop-run/scenarios`),
`--net PROFILE`, `--jobs N`, `--no-clear`, `--once` (one cycle, exit code 0 only if every run passed;
for scripts), `--poll-ms`.

What it does, by polling file times (std only, no new dependency):

* a change under `crates/coop-*/src`, `crates/world/src` or their `Cargo.toml`: incremental
  `cargo build -p coop-headless --example sim`, then the whole suite;
* a change to a scenario `.json`: just that scenario is re-run, no build;
* a build error: the first 12 error lines in compact form (`file:line:col: error[E...]: text`), the
  last good table stays on screen marked stale.

Output, one screen:

```
coop-sim watch  14:02:11  changed: crates/coop-session/src/live.rs (+1)
build 9.7 s
suite 2.1 s
scenario             seeds                      pass     wall  first failure
doc_stage_sync       . . . . .                   5/5   0.41s
live_loot_sequence   . F . . .                   4/5   0.63s  seed 2 desync: coop-sim run ...json --seed 2
RESULT: FAIL (10 runs, 1 failing)
```

`.` pass, `F` fail, `s` skipped (`needs_assets`: no game data). The failing row ends with the replay command.

Why an example: the watcher is `coop-sim.exe` itself, and Windows will not let cargo overwrite a running
exe. The loop therefore builds and runs `examples/sim.rs` (the same `main.rs`), a different file.
Start the watcher from `target/<profile>/coop-sim`, not with `cargo run --example sim`. Edits to
`watch.rs` need the watcher restarted; everything else is picked up.

### Keeping the rebuild short

* One target dir for everything (`CARGO_TARGET_DIR`), the same flags every time. Switching features,
  `RUSTFLAGS` or profiles between calls makes cargo rebuild `coop-core` and `world` (measured: 19 s and 45 s
  on a rebuild that should have touched neither). Do not run `cargo test --workspace` or a differently flagged build
  into the watcher's target dir while it runs; use a second `CARGO_TARGET_DIR` for those.
* `-p coop-headless` only, never `--workspace`. The incremental dev profile is already the default.
* `CARGO_BUILD_JOBS=2` is for a busy shared machine; on an idle one leave it unset.
* Cargo takes a lock on the target dir: a second cargo command (another agent, a rust-analyzer check
  with the same dir) shows `Blocking waiting for file lock`. That wait is not build time.
* Optional, not measured here: `CARGO_PROFILE_DEV_DEBUG=line-tables-only` (smaller link step) and
  `CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER=rust-lld.exe` (the linker that ships with rustup). Both change the
  fingerprint, so set them once for the whole session, not per command.

### What is measured (this PC, 2 jobs, idle target dir, debug profile, 2026-11-02)

Unit times from `cargo build --timings` after touching `coop-session/src/lib.rs`:
`coop-session` 7.8 s, `coop-headless` lib 5.8 s, the `sim` example 4.1 s, about 18 s end to end
(the next crate starts as soon as its dependency's metadata is ready). An edit that only touches
`coop-headless` is about 10 s. An edit in `world` costs 45 s for `world` alone, then the rest: that
crate is the slow one, and `coop-sim watch` does not make it faster. Keep rules code in `coop-*`
where you can and test `world` changes with its own unit tests (`cargo test -p world <name>`).

## 2. Live viewers

See `tools/coop-run/README.md`, "Live loop". In short:

```powershell
python -I tools/coop-run/live.py up --cell GSDocMitchellHouse        # host + guest, control ports, stays running
python -I tools/coop-run/live.py watch tools/coop-run/scenarios/live_loot_sequence.json
python -I tools/coop-run/live.py state guest --path items.guest
python -I tools/coop-run/live.py shot host C:/fnv/shots/host.png
python -I tools/coop-run/live.py down
```

Loading a place is the slow part (tens of seconds), paid once by `up`. After that a `run`, `bot`, `keys`
or `state` is a local TCP round trip answered on the viewer's next frame.

## 3. Fast viewer rebuild on the test PC

Build profile: `cargo build --profile play-quick` in `viewer/` (defined in `viewer/Cargo.toml`).
It is `play` (release inheritance, incremental, `debug = 0`, engine crates at opt-level 1) plus the
viewer crate and `world`, `coop-*` at opt-level 1. It is a new profile: `--release`, `play`, `play-fast` and
`play-opt` and therefore the packaging builds are unchanged. The exe is `<target>/play-quick/nv-viewer.exe`.

On the test PC use a target dir of its own and keep it:

```powershell
$env:CARGO_TARGET_DIR = 'C:\fnv\target-live'
$env:CARGO_INCREMENTAL = '1'
cd C:\fnv\nv-rs\viewer
cargo build --profile play-quick --message-format=short
```

Settings that were considered:

| Setting | Decision |
|---|---|
| `incremental = true`, `debug = 0`, 256 code-generation units | already in `[profile.play]`, inherited |
| opt-level 1 for `nv-viewer`, `world`, `coop-*` | in `[profile.play-quick]` only |
| opt-level 0 | not used: the Bevy glue at 0 is slow enough to disturb timing-based scenarios |
| `split-debuginfo` | no effect with `debug = 0` |
| `lld-link` | `-Lld` switch of the script (`CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER=rust-lld.exe`); an env var, not a `.cargo/config.toml`, so release builds are not affected |
| separate target dir | `C:\fnv\target-live`, never the release one: no fingerprint thrash with packaging builds |

### Sync from this PC

On this PC, make a bundle of the branch (no network, no push; never put it anywhere public):

```powershell
git bundle create C:/path/to/drop/nvrs.bundle worktree-coop      # first time: the whole branch
git bundle create C:/path/to/drop/nvrs.bundle OLDSHA..worktree-coop   # later: only the new commits
```

Copy it to the test PC under `C:\fnv\drop\` (the existing controller scripts, `Send-RemoteFile` in
`tools/coop-run/controller/lib.ps1`, do that), then run on the test PC:

```powershell
powershell -NoProfile -File C:\fnv\coop-run\sync_and_build.ps1 -Bundle C:\fnv\drop\nvrs.bundle -Ref worktree-coop [-Lld] [-SyncOnly]
```

It clones the bundle into `C:\fnv\nv-rs` the first time, fetches into `refs/remotes/bundle/*`, checks the
ref out detached (it stops if the checkout has local edits), builds with the fast profile into
`C:\fnv\target-live`, and prints either

```
RESULT: PASS build play-quick 41.2 s, 3 warning(s), C:\fnv\target-live\play-quick\nv-viewer.exe
```

or

```
BUILD FAILED after 12.4 s: 2 error(s), 0 warning(s); full log C:\fnv\target-live\last-build.log
  viewer\src\coop_control.rs:88:17: error[E0599]: no method named ...
RESULT: FAIL build play-quick 12.4 s (2 errors)
```

It writes only under `C:\fnv`, never deletes, never touches a running viewer. Old target contents are
left alone.

### What is not measured

No viewer build was run while this was written (the viewer is not built on the development PC), so the
`play-quick` build time is unknown. Expected: the first build is a full build of every dependency; later
builds recompile the viewer crate (one huge crate, incremental) and relink, which on the existing `play`
profile is the dominant cost. If `play-quick` turns out not to beat `play` on the test PC, delete the profile;
nothing else refers to it except the script's default.
