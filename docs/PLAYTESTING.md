# Playtesting

Playtesting uses the tester's own Fallout: New Vegas installation (FalloutNV.exe 1.4.0.525).
Never include game data in a test package or copy, modify or redistribute the installation.

## What runs

`nv-viewer` runs the engine port only: the player's FalloutNV.exe is mapped, the game's own
code (translated in `crates/engine*`) runs its `WinMain` as the main thread, and the game draws
in its own window. Until 2026-11-03 nv-viewer was also a hand-written reimplementation drawn
with Bevy; that layer, its acceptance routes and its macOS and Linux builds were removed
([ADR-0011](adr/0011-old-layer-removed.md)).

## Build and run (Windows)

```sh
cargo build --profile play -p nv-viewer      # the quick play build: target/play/nv-viewer.exe
cargo build --release -p nv-viewer           # the packaged build: target/release/nv-viewer.exe
```

Run it with the game's own window and sound:

```powershell
$env:NV_D3D9 = 'real'; $env:NV_SOUND = 'out'
.\target\play\nv-viewer.exe 'C:\Games\Steam\steamapps\common\Fallout New Vegas\Data'
```

The argument is the `Data` folder or the installation above it. The run ends where the game's
`main` ends: with the game's own exit, or, when the game's code stops (a function not translated
yet, a fault, a call the platform layer does not provide), with status 3 and a last line
`Game's main stopped in frame N: <why> (in <exe addresses>)`. A status line is printed every
5 s. `--movement-missing FILE` writes the untranslated functions reached, the stops and the
skipped static initializers; `--movement-probe` runs past untranslated functions to list more.
The removed viewer's arguments (a place name, `--walk`, `--physics`, `--wait`, `--background`,
`--new-game`, `--official`) are accepted and ignored.

## The machine's tables

The game's code answers Windows' questions from tables measured on the machine
([ADR-0008](adr/0008-d3d9-layer-forwarded-to-the-real-runtime.md)):

| Table | Made by | Read from |
| --- | --- | --- |
| Display adapter, folders, metrics, time zones | `scripts/d3d9-capture` | `NV_D3D9_ADAPTER`, default `%LOCALAPPDATA%\nv-rs\d3d9_adapter.tsv` |
| DirectSound devices and mixing | `scripts/dsound-capture`, `scripts/dsmix-measure` | `NV_DSOUND_TABLE` |
| D3DX texture loads | `scripts/d3dx-texture-capture` | `NV_D3DX_CACHE` |

The folders in the adapter table are the machine's own, so the game reads and writes its
settings and saves where the original does (`Documents\My Games\FalloutNV`). For test runs,
point the table's folders at a scratch copy of Documents; never at a player's own folders.

## Settings read by the host and the engine

| Variable | Effect |
| --- | --- |
| `NV_D3D9=real` | The game's own window through the real Direct3D 9 (without it nothing is shown) |
| `NV_SOUND=out` | The game's sound through DirectSound |
| `NV_D3D9_BACKGROUND=1` | The window opens without taking the focus (scripted runs) |
| `NV_MOVE_KEYS`, `NV_MOVE_MOUSE` | Scripted keys and mouse by main's frame (`crates/host/src/player_move.rs`) |
| `NV_MOVE_TRACE` | The last calls before a stop, to a file (slow) |
| `NV_SAVE_OUT=DIR` | The saves the game writes, copied out to DIR (`crates/host/src/save_out.rs`) |
| `NV_PRESET=fixes` or `vanilla` | What differs from the exe: the fixes set (the default) or the original's behaviour; comparisons and gates pin vanilla ([FIXES.md](FIXES.md)) |
| `NV_FRAME_LIMIT=n` | Frames a second at most under the fixes preset (0: none) |
| `NV_UNCAP=0` or `1`, `NV_HF_LANDING=0` or `1` | One fix alone; with no `NV_PRESET` the rest of the run is vanilla |
| `NV_FRAME_LOG=FILE` | One line per main frame (clock, step, player position), with no per-call cost | 

## Packages

`scripts/package-playtest.ps1` builds `--release` and zips the exe with `distribution/`
(`Play.cmd` and `Play.ps1` set `NV_D3D9=real` and `NV_SOUND=out`). A package does not make the
machine's tables yet, so it runs only on a machine that has them; the packages on GitHub
Releases from before 2026-11-03 are of the removed viewer.

## Reporting

A screenshot or a launch alone does not establish that something works. Record the starting
state (the save), the steps, the result, the log's last lines and a comparison with the original
game (a recording of FalloutNV.exe through `research/nv-trace`, or the original run side by
side).
