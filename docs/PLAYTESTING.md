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
| Display adapter, folders, metrics, time zones | `scripts/d3d9-capture`, then `scripts/nvapi-capture`, then the `CreateQuery` lines of `d3d9-capture device queries` | `NV_D3D9_ADAPTER`, default `%LOCALAPPDATA%\nv-rs\d3d9_adapter.tsv` |
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
| `NV_D3D9_TEXTURES=guest` or `check` | Texture pixels in the game's 32-bit memory as before ADR-0015's step 3 (`guest`, for comparisons), or kept outside it with checks printed (`check`); the default keeps them outside it, in the game's memory only while a level is locked |
| `NV_MOVE_KEYS`, `NV_MOVE_MOUSE` | Scripted keys and mouse by main's frame (`crates/host/src/player_move.rs`) |
| `NV_MOVE_TRACE` | The last calls before a stop, to a file (slow) |
| `NV_SAVE_OUT=DIR` | The saves the game writes, copied out to DIR, and the ones it deletes removed there (`crates/host/src/save_out.rs`) |
| `NV_PREFS_OUT=FILE` | The `FalloutPrefs.ini` the game writes, copied out to FILE (same file) |
| `NV_REPORT_DIR=DIR`, `NV_REPORT_LOG=FILE` | F12 in the game's window makes `DIR\NNN\` with the frame, the end of FILE and a `note.txt` for the player (`crates/engine-platform/src/d3d9real.rs`) |
| `NV_PRESET=fixes` or `vanilla` | What differs from the exe: the fixes set (the default) or the original's behaviour; comparisons and gates pin vanilla ([FIXES.md](FIXES.md)) |
| `NV_FRAME_LIMIT=n` | Frames a second at most under the fixes preset (0: none) |
| `NV_UNCAP=0` or `1`, `NV_HF_LANDING=0` or `1` | One fix alone; with no `NV_PRESET` the rest of the run is vanilla |
| `NV_FRAME_LOG=FILE` | One line per main frame (clock, step, player position), with no per-call cost | 

## Packages

`scripts/package-playtest.ps1` builds the Windows preview: one ZIP of `Play.exe`
(`scripts/launcher`), `app\FalloutNV.exe` (nv-viewer `--release` under that name), the table tools
and `distribution/`. docs/MAINTAINING.md lists what is in it and why. The first one, `preview-2026-11-07.2`, is on
GitHub Releases; the packages there from before 2026-11-03 are of the removed viewer.

`Play.exe` does what the steps above do by hand, on a PC with nothing but the game:

- **The game:** `--game`, the folder it used last time, then Steam (registry, `libraryfolders.vdf`,
  `appmanifest_22380.acf`), GOG (registry), Epic (`.item` manifests) and the game's installer's
  registry key, all read only, then a folder picker. `app\FalloutNV.exe --check` makes the same check
  of the game's program the game's start-up makes, so a refusal is said in a window before anything runs.
- **The folders:** everything in `userdata\`: the game's Documents and AppData (one set per way of
  playing, `game` and `mods`), the tables (`machine`), `logs`, `reports` and `settings.ini`. The table's
  `SHGetFolderPathA` answers point at them. The game's own writes stay in memory as ever
  (`engine-platform` files.rs); only `NV_SAVE_OUT` and `NV_PREFS_OUT` reach the disk, inside `userdata`.
- **The tables:** the adapter table is made again at every start (about 1.3 s here), so a new graphics
  card, driver, monitor, time zone or Windows language can't leave it stale. If its tools fail, the
  last good table is used. The sound table (about 19 s here) is made again when Windows' build or the
  default sound device changes. Without a sound device the table says so and the game goes on silent.
- **The first settings:** the player's `Fallout.ini` and `FalloutPrefs.ini` copied once (read only); where
  the game's own launcher never ran, what it leaves behind: `Fallout_default.ini` and the `high.ini` preset,
  from the game's folder. `sD3DDevice` is set to this PC's card at every start, because `main` asks for
  `FalloutNVLauncher.exe` when it differs (`004da570`); `ShellExecuteA` answers for it that nothing started,
  but after that the game's start-up still stops, in the renderer (a run without `sD3DDevice`): the name must match.
- **The name:** the program is simply named `FalloutNV.exe` in the package, in `app\`: no link is needed,
  and NVIDIA's driver profile (the white orbs) and Windows' per-program sound go by the name.
- **Mods:** `Play with mods.cmd` builds `userdata\modgame`: the game's folder as hard links (no copy,
  same NTFS drive needed), the `mods` folder's files copied over them with each link removed first.

### What the tables can't cover

The tables hold what a PC answers. These don't come from them; they were measured or read on the one
PC this has run on (Windows 11 10.0.26200, RTX 5070 Ti, a sound device of 2 or 8 channels), and hold
elsewhere only as far as Windows and drivers behave the same:

- The rules of the Direct3D 9 model (`d3d9-capture device`: reference counts, what a draw refuses, the
  `Reset` rules) and the D3DX behaviour, measured on NVIDIA's driver. A card that answers differently
  shows up as `NV_D3D9=real: ...: the model answers .., the real runtime ..` lines in the log, or a stop.
- The sound layer's translations of `dsound.dll`, `RESAMPLEDMO.DLL` and `dsdmo.dll` (the mixer, the
  converters, the 3D stage, the distortion) are from this Windows' builds. Only the resampler's tables
  and the distortion's edge table are read from the player's own DLLs. Another Windows' DLLs may mix
  differently without anyone noticing; the sound would still play. Where the resampler's tables
  aren't found, sound goes through DirectSound's second converter (named once in the log: not exact).
  One channel, quad, surround and the "wide" layouts follow the DLL's code only; only stereo and 7.1 ran.
- `scripts/dsmix-measure`, `scripts/dsound-oracle`, `scripts/mmio-measure` and `scripts/dshow-measure`
  are measuring tools: their results are built in, from this PC. They aren't re-run on a player's.
- The C runtime's character table (`nls`, `ctype1_measured.tsv`), Windows' file-system answers in `files.rs`
  (tunnelling, 8.3 names, `FindFirstFile` order) and `GetPrivateProfile*` rules were measured here on
  NTFS under an English Windows. Text conversion itself asks the real Windows.
- `d3d9-capture` covers adapter 0, and only the display modes of that moment. nvapi answers come from
  the DLL, but only an NVIDIA driver has run them.
- An AMD card: the game asks for the player's 32-bit `ATIMGPUD.dll` and, with Eyefinity on, ADL; this 64-bit
  process can't load them, so they answer as missing: one GPU, no Eyefinity. The game picks its vendor path
  from the driver's name. AMD and Intel have only been run as an edited table (the driver's name changed, no
  nvapi) on this PC's NVIDIA card, so the game's vendor paths ran, but no AMD or Intel driver answered.
- The first settings on a PC where the game's launcher never ran: its detection isn't translated (it is a separate
  program), so the High preset stands in for what it would pick.

The game asks for a Windows language's code page, and it reads paths one byte at a time. On a Chinese,
Japanese or Korean Windows, the package's folder must have a plain-letter path (or an 8.3 name that has
one); `Play.exe` says so. That setup has not been run.

## Reporting

A screenshot or a launch alone does not establish that something works. Record the starting
state (the save), the steps, the result, the log's last lines and a comparison with the original
game (a recording of FalloutNV.exe through `research/nv-trace`, or the original run side by
side).
