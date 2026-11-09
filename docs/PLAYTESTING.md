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
| D3DX texture loads (only with `NV_TEXTURES=dll`, or for files `crates/d3dx-tex` does not translate, where the DLL is missing) | `scripts/d3dx-texture-capture` | `NV_D3DX_CACHE` |

The folders in the adapter table are the machine's own, so the game reads and writes its
settings and saves where the original does (`Documents\My Games\FalloutNV`). For test runs,
point the table's folders at a scratch copy of Documents; never at a player's own folders.

## Switches

The game reads its switches from environment variables named `NV_...`. Players need only the
first group, and Settings.cmd sets those. Players need none of the others: Play.exe sets what it
needs, and the rest are for testers, scripts and the maintainers.

The groups, by who uses them:

1. **Player settings** (Settings.cmd, or a line in `userdata\settings.ini`).
2. **Game fixes** (one per row of [FIXES.md](FIXES.md); `Preset=` turns them all on or off, a row's own variable turns one alone, for gates and tests).
3. **Set by Play.exe** (the launcher sets these; a `NV_` line in `settings.ini` cannot replace them).
4. **Tests and diagnostics** (profiling, tracing, recording, checks, measurements).
5. **Obsolete candidates** (the feature is settled as the default and only the "off" side remains, which nothing uses; for the maintainer to approve, nothing removed yet).

Unset means the default in the table. A switch that is off by default needs a value to turn on
(`1`, or a file or folder name). A test or script run that sets a starred name (the list is
`TEST_VARIABLES` in `crates/host/src/fixes.rs`) and no `NV_PRESET` runs vanilla, the original's
behaviour ([FIXES.md](FIXES.md)).

### The game's Fallout.ini

Groups 1 and 2 are also keys of the run's own `Fallout.ini` (in `My Games\FalloutNV` of the
Documents folder the run uses), in four sections of their own: `[nv-rs General]`, `[nv-rs Display]`,
`[nv-rs Threads]` and `[nv-rs Fixes]`. The original game never asks for them, and the rest of the file
keeps the original's sections, keys and meaning. `distribution/nv-rs-sections.ini` is the block with one
note line above each key; `scripts/game-ini-sections.sh` writes it from `crates/host/data/game_ini.tsv`,
and the host's tests check one against the other. The fix lines are written with a `;` in front, so
`sPreset` decides; removing the `;` turns that one fix on (`1`) or off (`0`).

- Lowest to highest: the built-in default, the preset, the `Fallout.ini` keys, then the `NV_...`
  variables (for tests and scripts).
- `settings.ini` is still read, below `Fallout.ini`, until it is moved over: a line `Fallout.ini`
  already sets is not read.
- A key nv-rs does not know is passed over, as the game passes over one it never asks for. A value it
  can't use leaves the default, and one line in the log names it; the game still starts.
- Only a launcher's play run reads these sections: `NV_SETTINGS` set (Play.exe; the
  play folder's Play.cmd), with no starred name, no fix's own variable and no test flag on the viewer's
  command line. Every scripted, gated or measured run ignores them, so a player's file can't change a
  result.
- Play.exe passes the player options on as variables; for each one `Fallout.ini` sets, it passes the
  file's value instead of `settings.ini`'s, so the file wins there too (but not in a run a settings.ini line makes a test, which never reads the file).

### 1. Player settings

| Variable (Fallout.ini key; settings.ini line) | What it does | Default |
| --- | --- | --- |
| `NV_PRESET=fixes` or `vanilla` (`sPreset`; `Preset=`) | What differs from the original program: the fixes set, or the original's behaviour exactly | `fixes` |
| `NV_FRAME_LIMIT=n` (`iFrameLimit`; `FrameLimit=`) | Frames a second at most in play (0: none) | none under `fixes`; the original's own pacing under `vanilla` |
| `NV_MENU_FRAME_LIMIT=n` (`iMenuFrameLimit`; `MenuFrameLimit=`) | Frames a second at most in menus (0: none) | 120 |
| `NV_THREADS=stepped` (`bSteppedThreads=1`; a line in `settings.ini`; Settings.cmd's threads option) | The game's threads one at a time (the fallback, and the reference mode) instead of side by side on their own cores | side by side in play; a scripted or measured run is stepped whatever the default |
| `NV_RENDER_THREAD=0` (`bRenderThread=0`; a line in `settings.ini`; Settings.cmd's drawing thread option) | The real Direct3D 9 calls on the game's own threads, as the original makes them, instead of a render thread of their own | render thread in play; a scripted or measured run draws on the game's threads |
| `NV_BORDERLESS=1` (`bBorderless=1`; `Borderless=`) | The game's window without a frame, filling the screen | off |

`Resolution=`, `AntiAliasing=`, `Anisotropy=` and `VSync=` are the other lines Settings.cmd writes; the launcher turns them into the game's own settings, not into switches.

### 2. Game fixes (the 18 rows of FIXES.md)

Each takes `1` (on) or `0` (off) whatever the preset says; with no `NV_PRESET` the rest of that run is vanilla.
Each also has a `[nv-rs Fixes]` key in `Fallout.ini` (`bUncap` ... `bNVSEListGuard`, in the same order:
`distribution/nv-rs-sections.ini`), which a player can set without making the run a test.

| Variable | The fix | Read in |
| --- | --- | --- |
| `NV_UNCAP` | Uncapped frame rate | `host/uncap.rs` |
| `NV_HF_LANDING` | High-frame-rate landing | `host/uncap.rs` |
| `NV_HF_SLOWMO` | High-frame-rate slow motion | `host/uncap.rs` |
| `NV_MENU_CAP` | Menu frame limit | `host/uncap.rs` |
| `NV_GRAB_PUSH` | Dragged bodies by time | `host/uncap.rs` |
| `NV_PHYS_BLEND` | Smooth physics drawing | `host/phys_blend.rs` |
| `NV_MOD_WORN` | Mod fit worn question | `host/mod_fit_worn.rs` |
| `NV_TOPIC_INDEX` | Dialogue topic index | `host/topic_index.rs` |
| `NV_SHADER_WARM` | Shaders compiled ahead of first use | `host/fixes.rs` |
| `NV_IO_WAIT` | IO wait ends on completion | `host/fixes.rs` |
| `NV_COSAVE_KEEP` | Co-save keeps other plugins | `host/fixes.rs` (flag read by `nvse`) |
| `NV_NVSE_ERROR_GUARD` | Script error loop guard | `host/fixes.rs` |
| `NV_NVSE_IMOD_CHECK` | Image-space event form check | `host/fixes.rs` |
| `NV_NVSE_INVREF_CHECK` | Inventory reference checks | `host/fixes.rs` |
| `NV_NVSE_SIZE_CAPS` | Script array and string size caps | `host/fixes.rs` |
| `NV_NVSE_NULL_GUARD` | Script missing array guard | `host/fixes.rs` |
| `NV_NVSE_CYCLE_GUARD` | Script array cycle guard | `host/fixes.rs` |
| `NV_NVSE_LIST_GUARD` | Script inventory list guard | `host/fixes.rs` |

### 3. Set by Play.exe

Play.exe sets these for every run; set them by hand only to run `nv-viewer` without Play.exe.

| Variable | What it does | Default without Play.exe |
| --- | --- | --- |
| `NV_D3D9=real`, `wgpu` or `real,wgpu` | The game's own window, drawn by the real Direct3D 9 (`real`), by wgpu alone (`wgpu`, `crates/d3d9-wgpu`: for testing, not yet complete), or by the real one with wgpu drawing every frame off screen beside it (`real,wgpu`, for comparison: with `NV_SHOT` both are captured, numbered alike, wgpu's as `wgpu-frame-*.bmp`) | nothing is shown |
| `NV_SOUND=out` (or `device`) | The game's sound through DirectSound (`device` keeps the device's pace instead of the game's clock) | silent |
| `NV_D3D9_ADAPTER=FILE` | The display adapter table | `%LOCALAPPDATA%\nv-rs\d3d9_adapter.tsv` |
| `NV_DSOUND_TABLE=FILE` | The DirectSound devices and mixing table | none: the run stops and says how to capture it |
| `NV_D3DX_CACHE=DIR` | The D3DX texture cache (used only with `NV_TEXTURES=dll`, or for files `crates/d3dx-tex` does not translate) | `%LOCALAPPDATA%\nv-rs\d3dx-cache` |
| `NV_SAVE_OUT=DIR` | The saves the game writes, copied out to DIR, and the ones it deletes removed there (`crates/host/src/save_out.rs`) | saves stay in memory |
| `NV_PREFS_OUT=FILE` | The `FalloutPrefs.ini` the game writes, copied out to FILE | stays in memory |
| `NV_REPORT_DIR=DIR`, `NV_REPORT_LOG=FILE` | F12 in the game's window makes `DIR\NNN\` with the frame, the end of FILE and a `note.txt` for the player (`crates/engine-platform/src/d3d9real.rs`) | no F12 report |
| `NV_SETTINGS=FILE` | The settings file | `settings.ini` beside the program or in the folder above it |
| `NV_RS_GAME`, `NV_RS_REAL_DOCUMENTS`, `NV_RS_SYSTEM_FOLDER`, `NV_RS_NO_STORES`, `NV_RS_NO_SHORT_PATHS`, `NV_RS_NO_WINDOWS` | The launcher's own test hooks: the game folder, where the player's Documents are read from, the system folder, no look in game stores, no short paths, no windows shown (`scripts/launcher`; `NV_RS_NO_WINDOWS` is also read in `host/save_out.rs`) | the player's own folders, windows shown |

`NV_FIXES` was never a switch of the game's: Play.exe ignores a `NV_FIXES` line in `settings.ini` and Settings.cmd drops it.

### 4. Tests and diagnostics

None of these is for players.

| Variable | What it does | Default |
| --- | --- | --- |
| **Running and pacing** | | |
| `NV_TYPED=off` (or a list: `docs/ENGINE_CRATE.md`, "Typed code") | Turns the typed (native Rust) families off and runs the translations only, to rule them out when a trace or recording diverges from the original's. A run recorded for nv-trace (`NV_TRACE_OUT`, `NV_TRACE_LIST`) has them off unless `NV_TYPED` names some | on in every preset |
| `NV_WAIT_SPIN_US=n` | Real threads: how long a blocked wait watches for its signal before the thread sleeps, in microseconds (0 sleeps at once) | 50 with more than 2 logical processors, else 0 |
| `NV_RENDER_THREAD_FRAMES=n` | With the render thread, `Present` returns with at most n earlier frames still to draw | 1 |
| `NV_RENDER_THREAD_CHUNK=n` | With the render thread, how many commands are handed over at a time | built-in size |
| `NV_RENDER_THREAD_STATS=1` | With the render thread: a line a second on its hand-overs and the game's waits | off |
| `NV_D3D9_STREAM_CHECK=1` | With the render thread: every command hashed as recorded and as carried out, the run stopped at a difference | off |
| `NV_D3D9_RECORD=FILE` | Every Direct3D 9 command to FILE (`d3d9_stream::wire`), to replay into another backend | off |
| `NV_D3D9_BACKGROUND=1` | The window opens without taking the focus (scripted runs) | off |
| `NV_SLICE=n` | Stepped threads: calls between turns (0: only at waits and sleeps) | 200000 |
| `NV_WAKE_DELAY=n` | Stepped threads: calls after a signal before the threads it woke run | built-in |
| `NV_TEXTURES=dll` | The game's textures from the player's 64-bit `d3dx9_38.dll` (with `NV_D3D9=real`) or the texture cache, as before `crates/d3dx-tex`; files `crates/d3dx-tex` does not translate, and a thread at 64-bit precision, go to the DLL in any case, and without one are refused as D3DX refuses a file it cannot read | `crates/d3dx-tex` makes them, byte for byte the 32-bit DLL's |
| `NV_D3D9_PRESENT=immediate` | wgpu backend: present without waiting for the display | waits |
| `NV_WGPU_BACKEND=dx12`, `vulkan` or `gl`; `NV_WGPU_DEBUG` | wgpu backend: which graphics interface; wgpu's own debug checks | Direct3D 12 on Windows; off |
| **Scripted input and movement runs** | | |
| `NV_MOVE_KEYS`*, `NV_MOVE_MOUSE`* | Scripted keys and mouse by main's frame (`crates/host/src/player_move.rs`) | none |
| `NV_MOVE_REAL_INPUT=1`* | Real input reaches the game even with a script set | off |
| `NV_MOVE_PROBE`*, `NV_MOVE_THREADS` | Run past untranslated functions and list them; print who ran when | off |
| `NV_MOVE_TRACE`*, `NV_MOVE_TRACE_FRAMES=A-B`, `NV_MOVE_TRACE_KEEP=N` | The last calls before a stop, to a file (slow); also the calls of main's frames A to B; keep the last N calls | off |
| `NV_MOVE_CALL_PROFILE`*, `NV_MOVE_CALL_PROFILE_TOP=N` | Count the calls of a run; rows listed (0: all) | off; 200 |
| `NV_MOVE_CALL_BUDGET`*, `NV_MOVE_BUDGET_FROM=F` | Stop after N more calls (finds where a run loops); only from main's frame F on | off |
| `NV_MOVE_WATCH`* | The words at the listed addresses, printed with the status line | off |
| `NV_MOVE_TSV`*, `NV_DATA`, `NV_CAPTURE`, `NV_FEET` | Inputs of the ignored live tests (`crates/host/tests`, `crates/engine/tests`) | none |
| **Frames, sound and logs** | | |
| `NV_SHOT=DIR[,every_ms]`*, `NV_SHOT_FRAMES=FROM-TO`, `NV_SHOT_LIST`, `NV_SHOT_WGPU=DIR` | Screenshots of the game's frames to DIR; only those presents; a list of presents; the wgpu backend's own folder | off |
| `NV_FRAME_LOG=FILE`* | One line per main frame (clock, step, player position), with no per-call cost | off |
| `NV_LOCK_STATS=FILE`*, `NV_REAL_DUMP`* | Real threads: a line per main frame on the game clock and the waits; a dump of thread-local-storage misuse | off |
| `NV_PHYS_BLEND_LOG=FILE`* | What the smooth physics drawing did each frame | off |
| `NV_SOUND_RECORD=FILE.wav`* | What goes to the sound device, recorded, with a log | off |
| `NV_NVSE_EVENT_LOG=FILE`* | The script events raised, to FILE | off |
| **Profiling and tracing** | | |
| `NV_PROFILE`*, `NV_PROFILE_SECONDS`, `NV_PROFILE_TOP`, `NV_PROFILE_CALLS`, `NV_PROFILE_STACK`, `NV_PROFILE_SYMBOLS`, `NV_PROFILE_THREADS`, `NV_PROFILE_THREAD_CALLS`* | The sampling profiler and its table: period, rows, call rows, stack depth, symbol folders, all threads, a file of calls per thread | off |
| `NV_TRACE_OUT`, `NV_TRACE_LIST`*, `NV_TRACE_FLUSH=buffered` | A call trace for nv-trace: the file, the list of functions, no flush after each call | off |
| `NV_BENCH_CALLS`*, `NV_BENCH_CALL_COUNTS`*, `NV_BENCH_FUNCS`, `NV_BENCH_HOT` | The dispatch-speed benchmark's sizes (ignored test) | built-in sizes |
| `NV_CENSUS`*, `NV_TOUCH`*, `NV_RACE_CENSUS` | A census of the game's memory at chosen frames; which typed family touches what; races between threads | off |
| **Checks** (each compares the port with the original's way and says or stops at a difference) | | |
| `NV_WORD_CHECK`*, `NV_WATCH`*, `NV_WATCH_REPORTS=N`, `NV_QUEUE_CHECK`*, `NV_BLOCK_CHECK`*, `NV_HKMEM_CHECK`*, `NV_TOPIC_INDEX_CHECK`* | Words given to a call and not taken; memory changing at an address; queues; memory blocks; Havok memory; the topic index against the original's walk (goes with `NV_TOPIC_INDEX=1`) | off |
| `NV_SKIN_CHECK`*, `NV_HEAD_CHECK`*, `NV_FACE_CHECK`*, `NV_FACE_CHECK_DIR`, `NV_SPEECH_CHECK`*, `NV_LIGHT_CHECK`*, `NV_PASS_CHECK`*, `NV_AUDIO_CHECK`* | Bone matrices, heads, faces, lip-sync, lights, shader passes and sound mixing against the original's | off |
| `NV_D3D9_TEXTURES=check`, `NV_D3D9_BUFFERS=check` | Texture and buffer bytes kept outside the game's memory, with checks printed | off |
| `NV_D3D9_REFS=FILE`, `NV_D3D9_STUB_LOG` | Every texture reference-count change to FILE; the calls the Direct3D 9 model test makes | off |
| **Inputs of ignored tests** (need the player's game files or a measured capture) | | |
| `NV_EXE`, `NV_SCRIPT`, `NV_SCRIPT_TRACE`, `NV_INFLATE_EVERY`, `NV_INFLATE_PART`, `NV_VORBIS_TEST_DIR`, `NV_MUSIC_MP3`, `NV_D3DX_FUNCTIONS`, `NV_D3DX_IMAGE_INFO`, `NV_DSMIX_SETPOS`, `NV_DSMIX_SRC`, `NV_DSMIX_SURROUND`, `NV_DSOUND_FREQPHASE`, `NV_REPLAY_PROGRAMS`, `NV_REPLAY_OUT` | Where an ignored test finds the exe, a script, the game's archives, sounds or its own measured captures, or writes its result | the test skips or stops and says what to set |

### 5. Obsolete candidates

The feature is settled as the default. The variable only turns it off, for a comparison that nothing in the repo does any more. Nothing is removed yet; the PR that added this section lists them for the maintainer to approve.

| Variable | The settled default | What "off" does |
| --- | --- | --- |
| `NV_NATIVE_INFLATE=0` | The host decodes compressed archive files and records (the same bytes, faster loads) | The exe's own zlib translation does |
| `NV_REAL_SYNC=0` | Real threads' events and semaphores go off the kernel lock | They take the kernel lock's path, as before |
| `NV_SOUND_DECODE=inline` | Sound files decode ahead on a thread of their own | Decoding inside the game's read |
| `NV_TICK_COUNT=1ms`* | The tick count runs on the microsecond clock | The millisecond clock, as before |
| `NV_D3D9_TEXTURES=guest`, `NV_D3D9_BUFFERS=guest` | Texture pixels and static buffers live outside the game's 32-bit memory | They stay in it, as before ADR-0015's step 3 |

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
