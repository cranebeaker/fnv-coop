# nv-trace

Record-and-diff for the port: record the real FalloutNV.exe 1.4.0.525 and the port running
the same scene, then report the first function call whose arguments or answer differ. A bug
that shows up as a crash far away (a wrong word handed to a call, a dropped return value)
shows up here at the call where the two runs part.

This directory is its own Cargo workspace (like `research/nv-oracle`, whose decoder and
Windows declarations it uses). No external crates.

| Crate | What it is |
| --- | --- |
| `nv-trace-core` | The record format, function lists, the diff, the guard's path rules, the key script. Portable, unit-tested. |
| `nv-trace` | Command line: `diff`, `dump`, `stats`. Any host. |
| `nv-trace-rec` | The recorder DLL inside the real game (32-bit): guard, folder redirection, hooks, scripted keys into the game's own DirectInput keyboard. |
| `nv-trace-launch` | Prepares a scratch run folder, starts the real game suspended with the recorder, checks the player's folders before and after (32-bit). |

The port side is in the engine: `crates/engine-core/src/call_trace.rs`
(`NV_TRACE_OUT`, `NV_TRACE_LIST`).

## Safety: the player's folders are never written

The maintainer's rule: nothing may create, write or change a file in the game folder,
`Documents\My Games\FalloutNV` or `%LOCALAPPDATA%\FalloutNV`. The real game writes INI
files, `RendererInfo.txt` and saves there. How the recorder keeps that rule:

1. **Guard, first thing in the process.** The launcher starts the game suspended; the
   recorder DLL is loaded before the game runs an instruction, and the first thing it does is
   detour ntdll's `NtCreateFile`, `NtOpenFile` and `NtSetInformationFile`, through which every
   file creation, open, rename, hard link and delete of the process passes (the game, its
   DLLs, the drivers, the recorder itself). An open that could change anything (write-class
   access, any disposition but `FILE_OPEN`, delete-on-close) inside a protected folder or of
   one of its parents, and any rename or link whose target or source is protected, is refused
   with `STATUS_ACCESS_DENIED` before it reaches the file system, and written to the run's
   `rec.log`. Changing opens that succeed are checked again by the handle's final path, which
   catches a protected file reached under another name. If the guard can't be installed the
   DLL ends the process before the game starts.
2. **Redirection, so the game has somewhere to write.** The exe's own `SHGetFolderPathA`
   import answers the run folder's `Documents`, `LocalAppData` and `AppData` copies. The
   launcher fills them first with copies (read from the originals) of `Fallout.ini`,
   `FalloutPrefs.ini`, `FalloutCustom.ini`, `RendererInfo.txt`, the saves, `plugins.txt` and
   `NVDLCList.txt`. The exe copy itself lives in the run folder; the game folder is only its
   working directory (the game reads `Data\` relative to it).
3. **New files go to an overlay; absent files stay absent.** The game deletes
   `warnings.txt`, `programflow.txt` and `programmeroutput.txt` in its working folder at
   start-up; an open-only or delete of a file that doesn't exist is answered "not found", as
   Windows would, without opening anything ("guard: absent" in the log). A new file or folder
   the game makes in the game folder is made in the run folder's `game-overlay` under the same
   relative name instead, and later opens of that name go there ("guard: redirected").
   Changing an existing player file is always refused ("guard: refused"; a run with any such
   line should be stopped and looked at).
4. **No Steam.** The exe's `steam_api.dll` imports all answer as with no Steam client (as the
   port's platform layer does), so the Steam client is not involved in the run.
5. **Checked.** The launcher lists every file in the protected folders (size and modification
   time) before and after the run and reports any difference; its exit status is 0 only when
   there is none.

The protected folders are the game folder, `My Games\FalloutNV` under the Documents folder
the shell reports (on this machine OneDrive's) and under `%USERPROFILE%\Documents`,
`%LOCALAPPDATA%\FalloutNV` and `%APPDATA%\FalloutNV`, each also by its final path.

## Function lists

`lists/*.txt`, one function per line (format at the top of
[nv-trace-core/src/list.rs](nv-trace-core/src/list.rs)):

```text
# address  convention  words  return  name
0064aa00   thiscall    ppf    void    BSFaceGenAnimationData::AddPhonemeKeyframe
0064ca60   cdecl       fppb   u8      BSFaceGenAnimationData::UpdateKeyframeList
0086e650   frame       p      void    Main::OnIdle
```

One letter per 32-bit word as the port's `Engine::call` gets them (`this` first for
thiscall): `p` pointer (normalised), `w` word, `b` byte (low 8 bits compared), `h` 16 bits,
`f` float (bits). The recorder reads exactly that many stack words, so take the count from
the function's `RET n` or its translation, never a guess. `frame` marks the per-frame
function the key script counts frames by. `max=N` after the return kind records only the
first N calls of a hot function (the real side only; the port records all, so count gaps
past N are expected). To grow the list, add lines; the translations'
doc comments give the conventions.

## Record format

`NVTRACE1`, a header (`key=value` lines; `fn=` lines repeat the list), then blocks of
(thread id, length, records). Enter records carry the words, leave records `EAX`, `EDX` and
`ST0` (as a double), paired by a process-wide sequence number. A thread record before each
thread's first call names its start routine (0 for the main thread; the real recorder asks
`NtQueryInformationThread`, the port its thread host). Details in
[nv-trace-core/src/format.rs](nv-trace-core/src/format.rs). The real game's records are
written lock-free into a 1 MiB ring per thread, which a writer thread empties every 4 ms;
the port writes each record straight to the file (flushed, so a killed run keeps it).

## The diff

Threads are paired by their start routine when both traces name it (in order of their first
calls), then by their first calls (or `--pair REAL:PORT`). For each pair and each
Only functions both traces' headers list are compared (a function one side doesn't hook is
not a difference). function the k-th calls are compared: words by type, the answer by the return kind. A pointer
becomes "the n-th distinct pointer this function got on this thread" (`ptr#n`), so two
allocators compare; null and addresses inside the exe image (`0x00400000..0x01300000`,
`--image`) compare as they are. The report gives per-function counts, the first differing
call in the real game's order with the calls before it on both threads (`--context N`), and
the first count gap (a call one side made and the other never did), and the first
difference of each function. A thread's last call that never returned is where its
recording stopped, not a difference.

A port limit to keep in mind: a translation that calls another translation of the same unit
file directly (not through `Engine::call`) leaves no record, so such a function shows fewer
port calls than real ones without being wrong.

## Commands

Build (PowerShell or bash; on this machine through the compile queue):

```sh
cargo build --release --target i686-pc-windows-msvc --manifest-path research/nv-trace/Cargo.toml
cargo test --manifest-path research/nv-trace/Cargo.toml -p nv-trace-core -p nv-trace
```

`target/i686-pc-windows-msvc/release/` then holds `nv-trace-launch.exe`, `nv_trace_rec.dll`
(keep them together) and `nv-trace.exe`.

Record the real game (new game from the main menu, the port's `ng.sh` keys):

```sh
T=research/nv-trace
$T/target/i686-pc-windows-msvc/release/nv-trace-launch.exe --list $T/lists/speech.txt \
  --keys "1500:d0:1,1504:d0:1,1510:1c:1,1700:1c:1,2000:12:1,2300:12:1,2600:12:1,3000:12:1,3600:12:1" \
  --keys-fps 58 --keys-delay-ms 25000 --seconds 900 --name speech-1
```

It prints the protected folders, the run folder
(`build43-scratch/measure/rec/speech-1`: `real.nvtrace`, `rec.log`, the copies), the game's
exit and whether the player's folders are unchanged. `--prepare-only` makes the run folder
and checks the folders without starting the game. `--seconds` ends the run (the recorder
drains the trace first).

Keys: the script's frame numbers are the port's (`NV_MOVE_KEYS`). The port runs about 58
frames a second; the real game here runs uncapped (about 120), so pass `--keys-fps 58` to
read the numbers as times (frame / 58 s after the first frame). The real start-up takes about
15 s longer to its menu (splash and loading screens), so `--keys-delay-ms 25000` shifts the
whole script (measured 2026-10-28: menu background at about 40 s, first key then at 51 s).
The script also advances on the game's keyboard reads, since the intro movie's loop doesn't
run the frame function (the intro is about 290 s; Doc's first line comes after it). A
screenshot of the game's client area goes to `shots/` before the first key and 20 frames
after each press. The keys go into the game's
own DirectInput keyboard (held keys in `GetDeviceState`, changes as buffered records in
`GetDeviceData`), as the port's platform layer injects them; nothing system-wide is
synthesised. While the script runs the game's DirectInput mouse reads nothing (as in the
port). The keyboard device is foreground-only, so the recorder brings the game window to the
front before each key.

Record the port (a play build with `call_trace`, the `ng.sh` environment plus):

```sh
NV_TRACE_LIST=$T/lists/speech.txt NV_TRACE_OUT=%USERPROFILE%/nv-re/work/build43-scratch/measure/rec/port-speech.nvtrace bash build43-scratch/ng.sh TAG
```

Compare:

```sh
nv-trace diff rec/speech-1/real.nvtrace rec/port-speech.nvtrace --context 12 --out rec/speech-1/diff.txt
nv-trace stats rec/speech-1/real.nvtrace
nv-trace dump rec/port-speech.nvtrace --fn 0064aa00 --limit 40
```

Recordings are private (they come from the user's game): keep them in the scratch folder;
`*.nvtrace` is ignored here.

## Status (milestone 1, 2026-10-28) and next steps

Measured: real runs `speech-1` to `speech-8` left the player's folders unchanged (537
entries listed before and after each); after the "absent" answer no write was refused.
`speech-7` and `speech-8` start a new game from the menu with the port's keys, play the
intro and run in Doc Mitchell's house (Havok, face keyframes, render-pass arrays recorded).
Doc's first spoken line was not reached in 600 s of world time in `speech-8` (why is not
known: the world may have been paused while the window lacked focus, or the opening needs
input the port's script gives at other moments). Compared with the port (play build p189):
`Main::OnIdle` agrees call for call; the first value difference, in both real runs, is
`00dae3a0`'s record argument from its third call on the main thread (the real game passes the
record address of the first call again, the port a new one).

Milestone 2:

- Lists from the port itself: every function the port registers, with its word count and
  return kind, so lists grow toward every translated function; caps for hot ones.
- Lockstep input: start the key script at a game event (a function's first call, a global
  becoming non-zero) instead of a time offset; periodic screenshots in long runs.
- Memory side effects: bytes behind pointer arguments and `this` before and after a call,
  compared after the same pointer normalisation.
- Out-of-block writes in the port: a checked mode for `with_stack` and allocator blocks
  (guard words after each block, checked on return or free, reporting the function that wrote
  past the end) — the class of bug behind 00553bb0 giving 004e75d0 a 0x20-byte local it writes
  0x24 bytes into.
- Stack and heap pointers normalised separately, and values that come from random numbers or
  timers marked, so they don't show as differences.
- The port's direct calls inside one unit file recorded too (they skip `Engine::call`).

## Milestone 2 (in progress)

Done so far:

- **Block check in the port** (`crates/engine-core/src/mem.rs`, `NV_BLOCK_CHECK`): every
  `Mem::alloc` block (so every `Engine::with_stack` local and every platform heap block) gets 16
  guard bytes of `0xFD` right after the size asked for. `NV_BLOCK_CHECK=free` reports changed
  guard bytes when the block is freed, naming the functions running at its allocation and at
  its free; `NV_BLOCK_CHECK=store` also checks every store and names the writer at once;
  `,log` reports without stopping (the default stops at the first finding). Off: one flag per
  store (shared with the write watch) and one per call.
- **Real runs keep running unfocused**: the launcher sets `bAlwaysActive=1` (`[General]`) in
  the run folder's copies of `Fallout.ini` and `FalloutPrefs.ini` (`--no-always-active` leaves
  them as copied). The originals are never written.
- **Key script started by a game event**: `--keys-after ADDR:PORT_FRAME` starts the script's
  clock at the first call of `ADDR` (it must be in the list; `007cb7d0` `StartMenu::Create` is
  in `lists/speech-render.txt`) and reads the script's frames from `PORT_FRAME`, the port's
  frame of the same event. `--shot-every-s N` (default 30) saves a screenshot of the game's
  client area every N seconds (`shots/t-NNNN.bmp`).
- **Lists from the port's translations**: `nv-trace genlist --src crates [--max N] ADDR...`
  prints a list line per address from the `// Translated from ADDR` function's signature
  (words with `this` first, return kind) and its doc comment (convention, name). Check each
  line against the exe (a translation may drop a word or use a custom convention).
- **Stack and heap numbered apart**: both recorders write a stack record for words that point
  into a stack (real: the thread's stack, offset from the caller's stack pointer after its
  prologue; port: a live `with_stack` block, its depth and the offset into it). The diff shows
  such a word as `stk#n(frame+0x..)` and compares two stack words by their frame offsets when
  both are known, else by their numbers; a stack word never equals a heap pointer.
- **Recording from an event**: a list line's `after=ADDR:N` records the function only once the
  N-th call of `ADDR` (listed too) was entered (`after=ADDR` = the first call); `max=N` counts
  the calls recorded. Both recorders honour both (port: `CallTrace::admit`).
  `lists/racemenu-render.txt` records the render-pass functions from `RaceSexMenu::Create`.
- **Recording windows in the diff**: for each function with `max=` or `after=` the report
  prints each side's recorded frames; a side that recorded exactly its cap is marked "STOPPED
  AT ITS CAP (compare only these frames)" and its missing calls are not count gaps.
- **Screenshots of the real game only**: PrintWindow on the window owned by the game's own
  process (never a copy of the screen, which captured a port run's window lying on top in
  m2-1: those shots are marked unreliable); (almost) black frames are retried. `--mute-mouse`
  keeps the game's mouse silent for the whole run; `lists/keys-only.txt` hooks only what the
  key scripts need.
- **Memory dumps at entry**: `dump=EXPR:LEN` on a list line (several allowed) records `LEN`
  bytes at an address computed from the call's words: `wN` or `this` plus offsets, and
  `*(EXPR)` to follow a pointer, nested as deep as needed (e.g. `this+0x20:8`,
  `*(*(this+0x60)+8):0x120`). Both recorders write them; `nv-trace dump` prints them.
  `lists/racemenu-lights.txt` dumps `GetRenderPasses_2x`'s property flags, its light list and
  the first six lights in the race menu, with `GetNumberofActiveNonShadowLights`' answers.
- **More list tokens** (`nv-trace-core/src/dump.rs` has the grammar): `rdump=EXPR:LEN` takes
  the same bytes when the call returns (from the words it was entered with); `hash=`/`rhash=`
  record a 64-bit FNV-1a hash and the length instead of the bytes (up to 64 MiB, e.g.
  `hash=w1:w2` for a file in memory); `shader=EXPR` hashes Direct3D 9 shader bytecode to its
  end token; a base can be an absolute address (`0x011f9444:0x10`, a global) and a length a word
  of the call times a factor (`dump=w2:w3*16`). `when=EXPR==N` (or `!=`) records only the
  calls where the expression's value is `N` (`when=*(this)==0x010b7eb8`: objects of one
  vtable); the others are not counted by `max=`. `frames=N` records during N frames from the
  first call let through. Only the real recorder reads these so far; the port's engine reads
  `dump=` and `after=`/`max=`.
  `lists/frustum-cull.txt` probes `ShadowSceneLight::TestFrustumCull`'s inputs and results in
  the race menu.

## Milestone 3: the Direct3D 9 diff (in progress)

Record the device calls on both sides and compare the state of each draw.

- **Device methods** are listed by the port's addresses for them, `7e020200 + 4 * slot`
  (`IDirect3DDevice9` vtable order, `crates/engine-core/src/units/platform/d3d9device.rs`). The
  port records them natively (they are registered there). The real recorder redirects the exe's
  `Direct3DCreate9` import, patches `IDirect3D9::CreateDevice`, and when the device exists puts
  entry stubs in the listed slots of its vtable (only when the list names device methods).
  Exe import slots (the D3DX loaders at `00fdf3fc`, `00fdf400`, `00fdf404`) are hooked in the
  import table; the port registers its D3DX at the same slot addresses.
- **Objects by their creation**: a texture from a file by its file name (the enclosing call of
  the exe's texture-from-file function `00e68a80`, `dump=w1:0x104`), else by the hash of the
  file's bytes (`hash=w1:w2`), a shader by its bytecode (`shader=w1`, which also keeps the
  bytecode), a vertex declaration by its elements, other resources by their creation parameters
  and order (surfaces got from the device by how they were first got);
  `rdump=` of the out pointer ties each run's address to the name. Creation calls the real D3DX
  makes through the device inside a recorded D3DX call are left out (the port's D3DX does not
  go through the device).
- `nv-trace d3d REAL PORT [--limit N] [--out FILE]` replays the `Set*` calls, keeps for each
  draw the state it uses (the samplers and constants its shaders read, from the bytecode; stage
  states and transforms only without shaders), aligns each frame's draws by mesh (draw kind,
  primitive type, vertex and primitive counts, shaders) and lists each mismatching state with
  the draws it affects, most draws first, then the draws only one side made.
  `nv-trace d3d-draws TRACE` prints one run's draws with their state.
- Lists: `lists/d3d-racemenu.txt` (creation always; state from `RaceSexMenu::Create` to the end
  of the window; draws in frames 60..64 after it) and `lists/d3d-docroom.txt` (the same from the
  world start, frames 30..34). State set before the recording starts and never again shows as
  unknown on both sides.
- Port run: `NV_TRACE_OUT=FILE NV_TRACE_LIST=lists/d3d-racemenu.txt NV_TRACE_FLUSH=buffered`
  with the engine-core trace that reads these tokens.
- Tested without the game: `NV_FIXTURE_D3D=1 nv-trace-launch --exe nv-trace-fixture.exe ...`
  makes the fixture find `Direct3DCreate9` as the game does (`LoadLibraryA`, `GetProcAddress`),
  create a device, a vertex shader, set states, clear, draw a triangle and present; with a list
  of those slots (fixture-d3d-1) every call was recorded with its words and answer, the
  shader's hash and bytecode, its out pointer, the draw's vertex data, and `nv-trace d3d-draws`
  gave the draw its state.
