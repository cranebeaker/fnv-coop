# Engine port plan: from feature tracing to porting whole systems

Drafted 2026-10-08 for the session after the usage reset (Monday). It
proposes changing the method: stop tracing features one at a time and port
FalloutNV.exe system by system, the way the Havok work (B1) was done, with a
ledger that gives every function in the exe a status. Phase 0 is done
(2026-10-09, below); Phase 1 is proposed in
[FRAME_SKELETON.md](FRAME_SKELETON.md). Rules from [AGENTS.md](../AGENTS.md), ADR-0002 and ADR-0003 still apply.

## Where we are (measured 2026-10-08)

- FalloutNV.exe 1.4.0.525 in the shared Ghidra project: **63,262 functions**.
  Only about 1,500 have names, mostly library signatures. The Xbox PDB names
  have not been transferred to this project in bulk.
- Our Rust code cites about **3,800 distinct exe addresses**. Some are data,
  not functions, and some are cited only as "traced", not translated.
- About 270k lines in `crates/` and 63k lines in `viewer/`.
- Most systems were built feature by feature, from playtest bugs. That is
  why parts are still our own code where the game's isn't traced, and why
  fixes often have to work around their neighbours.

## Phase 0 result (2026-10-09, `claude/phase0-ledger`)

Done; see [LEDGER.md](LEDGER.md) (generated) and
[research/engine-map](../research/engine-map/README.md) (method, tiers and
measured error rates).

- **The function set was incomplete.** Ghidra's analysis had missed the
  functions behind 3,302 vtable slots, `bhkWorld::Update` (`00c6ae70`)
  among them. On a private copy they were created: 62,983 → **66,259**
  functions. (The "63,262" above included 279 imports.)
- **Names:** 17,604 PC functions carry an Xbox PDB name (vtable slots
  8,571, unique strings 1,716, call graph 7,317), plus 377 runtime-library
  Function ID names. Hold-out error of the call-graph tier about 1.7%; the
  string tier agrees with the vtable tier on all 320 functions both name.
  The names are in the private Ghidra copy `%USERPROFILE%\nv-re\ghidra-phase0`
  (17,405 applied, `src:xbox_pdb.*` tags); the shared server still serves
  the unnamed project.
- **Every function has a subsystem**: from its own name (17,502), its named
  neighbours in link order (16,005 units, 7,789 subsystem only; hold-out
  97.8% right), its callers (1,275), or the compiler tail (21,335); 2,353
  (3.6%) are explicitly `(unplaced)`.
- **What is game code:**

  | | Functions | KB |
  | --- | ---: | ---: |
  | all | 66,259 | 10,999 |
  | library: compiler-generated static initializers and `atexit` destructors | 21,335 | 530 |
  | library: CRT, STL, zlib | 698 | 135 |
  | platform: renderer, shaders, XDK/D3D, system, movie, SpeedTree, debug transport | 3,703 | 1,088 |
  | **game code** (everything else, incl. 2,353 unplaced) | **40,523** | **9,247** |
  | of which translated or traced today | 3,476 (8.6%) | |
  | of which `open` | 37,047 | 7,019 |

- **One command:** `cargo run --release --manifest-path scripts/ledger/Cargo.toml`
  regenerates LEDGER.md from the committed map and the Rust sources
  (`--check` for CI). `scripts/engine-map.ps1` redoes the map itself from
  the private inputs (69 s).

### Day-1 forecast (measured)

A trial on 12 `open` functions drawn at random, stratified by size like the
37,047 open ones (3 of ≤32 bytes, 5 of 33–128, 3 of 129–512, 1 of 1.4 KB),
three Sonnet agents with a private named Ghidra copy each, translating into
a scratch crate with one test per function (not committed):

- 10 translated completely, 2 partly (one class has no Xbox name, so its
  element type is unknown; one Havok callback where the decompiler was
  wrong and the disassembly left one store width open). One translation
  checked by hand against the disassembly matched instruction for
  instruction, including a return value Ghidra's C dropped.
- **Cost: 231k Sonnet tokens for 12 functions, about 19k per function or
  95k per KB of x86. 4.2 minutes wall-clock for the three in parallel.**
- Not included: `nv-call` checks, wiring into the running game, review by
  the lead, and the struct layouts that a shared state model would give
  (each agent modelled only the fields it touched).

Projection for the 37,047 open game functions: about **700M Sonnet
tokens** for leaf translation alone (by count 715M, by size 670M), before
wiring and checking, which B1 suggests costs at least as much again in
lead (Opus) time. Throughput is not the limit (agents ran a function a
minute each); usage is. **A week of usage is not enough to translate the
game.** It is enough for a few thousand functions, which is the size of
one or two whole systems with their wiring: the script engine and
condition functions (table-driven: 846 command-table entries, 206
console and 640 script, each with its handlers) and the frame skeleton are the best first use,
because they are countable, mostly pure and need little state model.
Compare the trial's 231k tokens with the usage meter to turn this into a
share of the weekly limit.

## Why the Havok work went better

B1 (PRs 1–10, 2026-10-07 to 08) worked because:

1. **It was a whole system with one entry point.** The work started at
   `bhkWorld::Update` (`00c6ae70`) and followed the engine's own call tree
   down: world constants, step driver, integrator, sleeping, islands,
   contacts, solver, constraints, character proxy.
2. **The goal was "make this the engine's system", not "fix this
   symptom".** A narrow fix inside invented surroundings needs guesses to
   fit them. Porting the surroundings as well removes those guesses.
3. **Each PR replaced and deleted our version** (for example XPBD contacts
   in PR 7) instead of adding a layer on top.
4. **The layouts came from the Xbox PDB**, so the structures matched the
   game's from the start.
5. **Each PR named one next action**, so the sequence kept going across
   sessions.

The plan below applies this method to every system.

## Which parts get translated

Not all 63k functions should be translated. Each function gets one status:

| Status | Meaning | Examples |
| --- | --- | --- |
| `translated` | Rust translation, marked with its address (ADR-0003), with a test | Havok solver |
| `traced` | Behaviour read from it and written in our own code | most current systems |
| `platform` | Replaced by our own Rust layer under the translated code. Any behaviour the game shows is still traced (audio falloff) | Direct3D 9 and D3DX themselves (ADR-0007), DirectInput, DirectSound, Win32 threads and file finding, allocator, Xbox system utilities, Bink playback |
| `library` | Compiler or standard library code | CRT, STL templates, thunks |
| `open` | Not looked at yet | |

The layers to translate are Bethesda's game code (forms, references,
process/AI, scripts, quests, menus' logic, save/load), Havok, Gamebryo's
scene graph and animation controllers, FaceGen, and the renderer. The
renderer is translated too and runs on our own Direct3D 9 layer on wgpu,
with the game's shaders converted at load time
([ADR-0007](adr/0007-renderer-translated-on-own-d3d9-layer.md)); Bevy stays
the development viewer until that path matches.

All of `NiXenonRenderer` and `BSShader` is game code (ADR-0007; the
ledger keeps only a D3DX import thunk there as `platform`), and so is the
game code found inside platform subsystems (the ledger's `GAME_FUNCS`).
Inside the remaining platform layers, the parts that are logic rather than OS calls
are translated too (the ledger's `LOGIC_UNITS`): archive lookup and the
file layer above the OS reads, and D3DX's maths; SpeedTree (tree building,
splines, random numbers, wind) is game code as a whole. Each of the 2,115
functions the ledger counted as `platform` before ADR-0007 was sorted by
reading it; docs/SYSTEMS.md ("Renderer units") has the counts.

## What happens to the existing code

Nothing is thrown away and nothing is rewritten from scratch. The port
replaces pieces in place, the way B1 replaced the solver, while everything
around them keeps running.

- **Kept as is:** the file readers (`esm`, `nif`, `bsa`, `dds`, `bink`,
  `fos`, `mp3`, `speedtree`, `shaders`), the Bevy renderer and viewer, the
  Havok port, and the tests, acceptance routes and tools. None of this has
  a game counterpart to swap in, or it already is the game's.
- **Kept and counted:** code already traced from an address becomes
  `traced` in the ledger. It is only redone if a translation of the same
  function shows it differs.
- **Replaced gradually:** only code with no address behind it (our own
  logic where the game's wasn't traced). Even then the current version
  keeps the game running until its replacement lands, and its tests
  check the replacement.
- **Reshaped, not rewritten:** the state model (Phase 2) moves behind
  adapters one form type at a time. The systems built on it keep working.

## Phase 0: the map and the ledger (first Monday session)

Do this before any porting. The rest of the plan depends on it.

1. **Transfer the Xbox PDB names to the PC project** in bulk
   ([PROTOTYPE_SYMBOLS.md](PROTOTYPE_SYMBOLS.md) step 2: RTTI class names,
   the vtable-slot tier, then function matching). This needs writes, so it
   runs on a private copy of the project with `ghidra.ps1`, not on the
   shared server. Afterwards the server restarts on the named copy, still
   read-only.
2. **Generate the engine map** (a Ghidra script in `research/ghidra`): for
   every function, its address, size, name and class (Xbox PDB), the PDB's
   source file, its callers and callees, and a **subsystem**. The subsystem
   comes from the class or source file. Where neither exists, it comes from
   the address range, because MSVC places each object file's code together.
3. **Generate the ledger** (`scripts/ledger`, rerunnable): scan the Rust
   code for address markers and give each map entry a status from the
   table above. Commit only addresses, names (allowed by ADR-0002),
   statuses and the Rust locations. No decompiled code.
4. **Publish a coverage table per subsystem** (`docs/LEDGER.md`,
   generated): translated, traced, platform, library and open counts. This
   gives a measurable "how much of the game is the game's".

Gate: every function has a subsystem and a status, and the table can be
regenerated with one command.

## Phase 1: the frame skeleton

Translate the main loop's per-frame order, the way B1 started from
`bhkWorld::Update`: from the exe's main loop through `TES`/`Main` updates,
process lists, Havok, scripts, menus and rendering. Each call becomes a
function in our frame order that either calls our matching system or is
marked `open`. This tree becomes the master checklist, and it fixes "runs in
the wrong order" bugs (several past bugs were this kind). Bevy systems get
ordered by it.

## Phase 2: the game's state model and save/load (old saves)

Saves are the game's own runtime objects written out (change forms). If our
state mirrors those objects (`TESForm`, `TESObjectREFR` and its
`ExtraDataList`, `Actor`, the process levels, `PlayerCharacter`, `TESQuest`
and its variables), then loading and writing `.fos` saves becomes a direct
translation of the game's own save and load functions. We already have the
reader and the import ([FOS_SAVES.md](FOS_SAVES.md), `crates/fos`,
`world::fos_import`).

- Port in small steps. Put the ExtraData list in first, then move one
  form type at a time behind adapters. The acceptance routes must pass
  after every PR.
- **Gate: round trip.** Read a real save, build our state, write a `.fos`,
  and compare it with the original change form by change form. Every
  difference is a gap in the state model. This is a strict, automatic
  check that the state is complete.
- After that: load a real save, run the same seconds in nv-rs and in the
  original game, and compare (TASKS M10, the comparison harness).

## Phase 3 onward: one system at a time, Havok style

Suggested order. Each item is one track of PRs with its own entry point
from the map:

1. **Script engine.** The bytecode interpreter and every command handler
   (`NvLabelCommandTables.java` already lists the tables). This is easy to
   count: handlers translated out of the total.
2. **Condition functions** (the `CTDA` functions). Same shape: one table of
   handlers.
3. **Process and AI.** The process managers and levels, packages,
   combat, detection, pathing.
4. **Animation.** Gamebryo's controller manager and KF playback, and
   Bethesda's idle and anim-group system.
5. **Interface.** The Tile/XML engine and each menu class's logic.
6. **The rest of Havok** (TASKS B1 leftovers: GJK and penetration depth,
   motors, full TOI, moving platforms).
7. **Audio logic** (the game side of BSAudio; output is `platform`).
8. **Everything else in the ledger** that is still `open`, largest
   subsystem first.

DLC and mods (M5) come after this. They are mostly data on top of the same
engine code.

## How the work runs (usage budget)

- **Lead session:** works on the structure: types, layouts, frame order and
  the order of PRs in a track. One track at a time.
- **Small agents:** handle leaf functions the ledger marks as pure (math,
  getters, condition and command handlers), on a cheaper capable model,
  using the read-only Ghidra server. Each one is checked with `nv-call`
  against the real exe where it qualifies. One owner per file
  (AGENTS.md).
- **Every PR:** has address markers, a generated test, the acceptance
  routes, and a ledger regeneration that shows the counts moving.
- **Bugs from playtests** go to the track that owns them. They are fixed by
  porting that area, not patched locally, unless they crash or lose data.
- Usage stop rule as before: at 95%, merge finished work and write
  HANDOFF.md.

## Fast track: one week, checked by machine instead of playtests

The maintainer's preferred version: translate as much of the exe as
possible in one week, with no playtesting during the push.

**Checks that replace playtesting** (all run without anyone playing):

- **`nv-call` oracle:** runs the real exe's function on generated inputs
  and compares the result with the Rust translation. For pure functions
  this is stronger evidence than playing.
- **Save round trip** (Phase 2 gate): automatic and strict about state.
- **Compile, unit tests, clippy** on every PR (CI already does this).
- **Acceptance routes** run unattended in the background once per merged
  batch, not per PR. A failure sends the batch back; no one has to watch.
- **Comparison harness** (TASKS M10), when it exists, compares routes with
  the original automatically.
- The maintainer playtests once, at the end of the week.

**Two speeds, kept apart:**

1. **Translation (parallel, fast):** agents take ledger entries by
   subsystem and translate them into Rust, marked with addresses and
   checked with `nv-call` where possible. This is the part that scales
   with agents and usage.
2. **Wiring (sequential, slower):** connecting translated functions into
   the running game top-down, replacing our versions (the B1 method). The
   lead session does this. This is the part most likely to be unfinished
   at the end of the week.

**Day 1 decides the forecast.** The ledger gives the real number of game
functions once library and platform code are separated out. The first
day's translation rate per unit of usage, times that number, says whether
a week of usage is enough. Report that projection at the end of day 1
instead of guessing now.

## Risks

- **Scale.** 63k functions cannot all be done in one week. B1 was about 10
  PRs over two days for one subsystem. The ledger makes progress
  measurable and keeps the order right. It does not make the work small.
- **Refactor breakage.** Changing the state model touches everything.
  Hence adapters, one form type per PR, and the acceptance routes on every
  PR.
- **Wrong translations.** ADR-0003 notes that LLM translation without an
  oracle is often wrong. Pure functions get `nv-call`. Stateful code gets
  the save round trip and route comparisons.
- **Legal exposure grows with volume.** ADR-0003 accepted the risk for
  marked translations. Translating most of the exe makes the repository
  close to a port of the binary (the re3 precedent). The maintainer should
  confirm this is still acceptable before Phase 3 at scale. The
  `rg "decompiled, FalloutNV"` isolation still holds.
- **Ghidra writes.** Name transfer needs a writable copy. The shared
  server stays read-only.

## First prompt for Monday (draft)

> Read docs/ENGINE_PORT_PLAN.md. Do Phase 0: transfer the Xbox PDB names to
> a private copy of the PC Ghidra project, generate the engine map and the
> ledger, and publish the per-subsystem coverage table. Then propose the
> Phase 1 frame skeleton as a PR sequence. Work the way B1 did.

Later tracks: "Port <subsystem> from FalloutNV.exe the way B1 ported Havok:
find its entry point in the map, go top-down, replace and delete our
version, PRs in sequence, ledger updated in each." Give the goal and the
done-gate, not the implementation.
