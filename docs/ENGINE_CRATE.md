# The engine crates: translating FalloutNV.exe function by function

The engine crates (`crates/engine` and `crates/engine-*`, see
[below](#the-engine-crates)) hold FalloutNV.exe 1.4.0.525 translated into
Rust, one function at a time, on a model of the game's own memory. Decision and
alternatives: [ADR-0006](adr/0006-engine-crate-memory-model.md). The ledger
([LEDGER.md](LEDGER.md)) counts the translations; the engine map
([research/engine-map](../research/engine-map/README.md)) says which unit
each function belongs to.

## The model in one page

- **Memory** (`Mem`): the game's 32-bit address space, paged and sparse.
  Objects sit at their real field offsets; a pointer is the game's address
  ([`Ptr<T>`]). Reading memory nobody provided panics with the address.
- **The exe's data**: `Engine::with_exe(path)` maps `.rdata`, `.data`,
  `.tls` and `CONST` of the player's installed FalloutNV.exe (checked
  against 1.4.0.525) at their addresses. Constants, strings, vtables and
  initial globals are therefore the game's own; nothing from the exe is in
  the repository. Tests run without the exe and set what they need.
  The code section (`.text`) is not mapped (the Steam exe encrypts it): the
  compiler's `switch` jump tables and byte maps are translated as ordinary
  `match` arms on the case value (control flow), and the few data-like tables
  that live there (colours, type names) as `match` arms too. No table is
  copied into a constant array and none is read from the exe's code.
- **Functions**: every translation is a typed Rust function and is also
  registered under its exe address in a uniform form (argument words,
  `this` first; see `abi.rs`). `e.call(addr, &args![...])` calls any
  function by address, `e.vcall(this, slot, &args![...])` through the
  object's vtable. A call to a function nobody has translated panics
  with `open function <addr>`, so translations can be written in any
  order and connect as the coverage grows.
- **Classes**: `layout!` declares a class marker with its PC size and the
  fields used so far (`e.get(p, Class::field)`, `e.set(...)`).
- **Platform**: the allocator (`MemoryManager::Allocate`/`Deallocate`) is
  `Mem`'s heap. The game's threads run under the thread host
  (`engine_core::thread_host`): each `CreateThread` gets an OS thread used
  strictly as a coroutine (it holds the engine, a baton, only while it runs;
  exactly one thread runs at a time), with its own TLS block and call stack
  in `GameThread` (the per-thread x87 control word goes there too). Switch
  points are fixed: a wait on an unsignalled event or semaphore, and
  `Sleep`; the main thread's wait runs the runnable threads in creation
  order until its object is signalled, a game thread's wait hands back to
  the main thread, `SetEvent`/`ReleaseSemaphore` wake waiters, and a wait no
  thread can satisfy stops the run naming the object. So runs are
  deterministic (for one processor count, which `iNumHWThreads` follows).
  This avoids unsafe stack switching; it costs an OS thread per game thread
  and a channel round trip per switch.
  Functions replaced this way are marked
  `// Platform: replaces <addr> (...)`; the ledger counts them as
  `platform`.

## The engine crates

The engine is split into crates so a compile fits the machine (one crate of
2.4 million lines needed about 28 GB to test). Each crate below sees only the
crates under it; the graph has no cycles:

| Crate | Holds | Depends on |
| --- | --- | --- |
| `engine-core` | `Engine`, `Mem`, `abi`, `ptr`, `x87`, `types`, `exe`, the static initializer and `atexit` tables (`static_init`, `static_exit`, `data/`), the prelude and the `entry!`/`args!`/`layout!` macros; units `crt.rs` and `platform.rs` | - |
| `engine-gamebryo` | `nimain`, `nianimation`, `nicollision`, `niparticle`, `nisystem`, `nixenonrenderer` | core |
| `engine-bs` | `bsaudio`, `bscore`, `bsmain`, `bsmenu`, `bsmovie`, `bsshader`, `bssystem` | core, gamebryo |
| `engine-havok` | `havok_sdk`, `bshavok` | core, gamebryo |
| `engine-speedtree` | `speedtree` | core |
| `engine-fallout` | every `fallout_*` subsystem, `unplaced`, `facegen`, `d3dx9`, `remotelog`; the `extra_data` and `reference` adapters | all of the above |
| `engine` | nothing of its own: re-exports every crate (`engine::units::<subsystem>`, `engine::prelude`, ...) and registers every function | all |

The grouping follows the code: Rust references between subsystems (layouts,
shared helpers) only point down, and nearly every unit test only reaches
functions of its own crate or below (measured on the full suite before the
split: 7 of 34,679 tests reached further). Those 7 changed with the split:
three integration tests moved to the unit whose crate links everything they
run (`fabs_restores_the_control_word_through_ctrlfp` from `crt.rs` to
`fallout_shared_magic/effectsetting.rs`, the scrap-heap buffer test from
`platform.rs` to `bscore/scrapheap.rs`, the allocator start-up test from
`nisystem/allocator_support.rs` to `unplaced/run_00aa1010.rs`), and four
double a callee in `engine-fallout` (`__allshl` in two tests of
`havok_sdk/hkbinarytagfilereader.rs`, `memmove_s` in
`speedtree/run_00b23f10.rs`, four small helpers in
`bshavok/bhkblendcontroller.rs`).

- **Paths stay.** Each part crate re-exports the core (`crate::types`,
  `crate::exe`, `crate::prelude`, ...) and its `units` module re-exports the
  subsystems of the crates below it, so `crate::units::<subsystem>::...`
  works as before for anything in the same crate or below. A layout in a
  crate above is not reachable: read the field at its offset with a comment,
  as for a class without a layout. Users of the engine keep `engine::...`.
- **Registration.** `Engine::new()` registers the core's functions and those
  the program assembles with `engine_core::assemble!` (one crate per
  program defines it): the `engine` crate assembles every part crate, so
  the full engine has every translated function. A part crate's own unit
  tests assemble that crate and the crates below it (generated into its
  `units/mod.rs` under `cfg(test)`), so a test there reaches exactly those
  functions; a call into a crate above stops with `open function <addr>`
  like an untranslated one, and the test doubles it. A program that links
  `engine-core` without an `assemble!` does not link.
- **Tests that need a crate above** (rare): double the callee in the test
  (`e.register(addr, ...)`), as for an untranslated one.
- `engine-core`'s `test-support` feature exposes the form-constructor
  checker (`units::crt::ctor_test`) to the other crates' tests.

## One unit, one file, one owner

Translations live in `crates/<crate>/src/units/<subsystem>/<unit>.rs`, one
file per source unit of the Xbox 360 prototype (Xbox PDB, ADR-0002): for
example `fallout/ai/processlists.cpp` is
`crates/engine-fallout/src/units/fallout_ai/processlists.rs`. `ledger queue`
prints each unit's file and crate.
The lead creates the files (`ledger scaffold <unit or subsystem>`); the
`mod.rs` files are generated (`ledger units`) and never edited by hand.
**A translator edits only the unit files assigned to them.**

```sh
L="cargo run -q --release --manifest-path scripts/ledger/Cargo.toml --"
$L queue                          # open functions per unit, largest first
$L queue "fallout/ai/processlists.cpp"   # one unit's work list
$L scaffold "fallout/ai/processlists.cpp" # create its file (lead)
```

## How to translate a function

1. **Read it.** Decompile and disassemble it in the named Ghidra project
   (17,600 functions carry Xbox PDB names, so callees are often named).
   Where the C is ambiguous (stack slots, return registers, x87), the
   disassembly decides. Class layouts (names, offsets): the Xbox PDB
   (`pdbdump type <pdb> <Class>`), checked against the PC code's own
   offsets, since the 2010 Xbox build can differ.
2. **Write it** in the unit file:

   ```rust
   // Translated from 00d87410 (decompiled, FalloutNV.exe 1.4.0.525)
   /// `hkCachedHashMap<hkStringMapOperations,hkContainerHeapAllocator>::getIterator`
   /// (Xbox PDB): index of the first occupied slot, or `hashMod + 1`.
   pub fn get_iterator(e: &mut Engine, this: Ptr<HkCachedHashMap>) -> i32 {
       let hash_mod = e.get(this, HkCachedHashMap::m_hashMod);
       ...
   }
   ```

   - The marker comment goes directly above the function, with the
     function's entry address. The doc comment names the Xbox PDB name
     (marked `(Xbox PDB)`) and says what the function does.
   - Name: snake_case of the class and method, `tes_form_get_form_id`;
     overloads keep the map's `_ovN` (`garbage_collector_add_ov2`);
     unnamed functions are `fn_<addr>`. Check the body before trusting a
     map name: identical-code folding can put another method's name on a
     shared body (a destructor named `NiTArray<..>::SetSize`). If the name
     is wrong, use `fn_<addr>` and say what the body is.
   - Parameters in declaration order, `this` first. Types: `u32`, `i32`,
     `u16`, `i16`, `u8`, `i8`, `bool`, `f32`, `f64`, `u64`, `Ptr<T>`.
     A struct returned by value is an explicit pointer parameter.
   - Register it in the file's `funcs()`:
     `entry!(0x00d87410, get_iterator(Ptr<HkCachedHashMap>) -> i32)`.
3. **Calls.**
   - Same unit: call the Rust function directly.
   - Anything else: `e.call(0x00abcdef, &args![a, b])` (result:
     `.u32()`, `.f32()`, `.bool()`, `.ptr::<T>()`, or
     `e.call_as::<T>(...)`). Never re-implement another unit's function
     inline, and never guess what a callee does.
   - Virtual: `e.vcall(this.addr(), 0x38, &args![...])`, with the byte
     offset of the slot, exactly as the code indexes the vtable.
   - CRT functions (`memcpy`, `strlen`, `sprintf`, ...) are called by
     address like any other; the runtime library is translated or
     replaced separately.
   - Imported Windows functions (`call dword ptr [0x00fdf0e4]`) are called
     by the address of their import slot: `e.call(0x00fdf0e4, ..)`.
     `units/platform.rs` provides the ones with a Rust stand-in
     (interlocked operations, critical sections, clocks, kernel objects,
     virtual memory, thread-local storage, `GetSystemInfo`,
     `GetCurrentProcess`); report others you need.
   - A value the code passes in x87 `ST0` (compiler helpers such as
     `_ftol2`, `00ec62c0`) is a leading `f64` argument (two words).
   - A local the game keeps on its stack and passes by address (a scope
     guard, a `NiPointer` temporary, an out parameter):
     `e.with_stack(size, |e, p| ...)`.
4. **Data.**
   - Fields of a class this unit owns (the class's own `.cpp` is this
     unit): declare them with `layout!` in this file, Xbox PDB names,
     PC offsets.
   - Containers, strings and reference-counted bases (`NiTArray`,
     `BSSimpleArray`, `BSSimpleList`, `BSStringT`, `NiFixedString`,
     `NiRefObject`) are in `crate::types`; use them, never redeclare them.
     Template instances that a unit contains (an `NiTArray<T>::SetSize`
     emitted in your `.cpp`) are your functions; their layout is still the
     shared one.
   - Fields of other classes: use their `layout!` if it exists
     (`crate::units::<sub>::<unit>::Class`, in the same crate or below); otherwise read at the offset
     (`e.mem.u32(p.addr() + 0x20)`) with a comment naming the class and
     field (`// TESForm::iFormID (Xbox PDB) +0x0C`). Do not edit another
     unit's file.
   - Globals and constants: read them from memory at their address
     (`e.global::<f32>(0x01012e10)`), as the game does; they come from the
     exe at run time. String literals are passed as their address.
   - Floats: compute in `f64` and round to `f32` at each `float` store,
     the closest match to x87 code (results can differ only in the last bit,
     in double-rounding cases). SSE code (`MOVSS`, `ADDSS`) computes in
     `f32`.
   - x87 precision: the game's main thread runs at 24-bit precision once
     Direct3D has created the renderer's device (control word `0x007F`;
     `crate::x87`, "Precision control"), the other threads at 53-bit. A
     translation whose results must match the exe on that thread does each
     x87 `FADD`/`FSUB`/`FMUL`/`FDIV` (and the `P`/`R`/`I` forms) through
     `x87_add`/`x87_sub`/`x87_mul`/`x87_div`, and `FSQRT` through
     `x87_sqrt`, one call per instruction in the exe's order and operand
     order, with the operands as the instruction sees them (`x87_load32`
     for an `FLD dword`, `x87_load64` for an `FLD qword`, `x87_m32` for a
     `float` memory operand, the plain `f64` for a `double` one,
     `f64::from(i)` for `FILD`/`FIADD`), and
     `x87_store32` at each `FSTP dword`. They round to the running thread's
     precision, so the same translation is right on either thread. Where a
     translation keeps plain `f64` arithmetic, `x87_round` rounds a value to
     the current precision at one point (exact for an operation on floats
     with round to nearest; see its comment).
5. **Not translated**: C++ exception unwinding (`__CxxFrameHandler`
   states) and SEH frames are left out (say so if a function relies on
   them); the compiler-generated initializers and `atexit` destructors
   in the tail of `.text` are `library`.
6. **Test it** in the file's `#[cfg(test)] mod tests`: build the objects
   it reads (`e.new_object::<T>()`, `e.map`, `e.set_global`,
   `e.put_vtable`), stand in for callees with test doubles
   (`e.register(0x00abcdef, |e, a| ...)`, or `e.register_double` for a
   double that keeps state), and check results and calls
   (`e.call_log = Some(vec![])`). Inputs come from the decompiled
   logic, one case per branch that matters. Then:

   ```sh
   cargo test -p <crate> --lib units::<subsystem>::<unit>
   cargo clippy -p <crate> --all-targets
   cargo fmt --all
   ```

7. **Report** per function: translated / partial / skipped, and why.
   Partial translations keep the marker only if the missing part is named
   in a comment (`// open: ...`). Also add `// Translation incomplete:
   <address> - <missing behavior>` on one line. The ledger keeps such
   addresses `partial` in the engine queue instead of `done`; remove that
   incomplete marker only when the documented gap is closed and tested.
   A panic-only placeholder has no translated branch: leave it skipped,
   without a translation marker or a new registry entry.

## Checking against the real exe

Pure functions can be compared with the real CPU through `nv-call`
(`research/nv-oracle`): the same inputs, the game's function and the
translation, bit for bit. The ledger will gain a column for that check.
Stateful code is checked by the save round trip (Phase 2) and the
acceptance routes once it is wired.

## The script interpreter (opt-in run on real scripts)

The interpreter's trunk (`Script::Run` → `ScriptRunManager::Run` →
`ScriptRunner::Run` → `EvaluateLine` → `CExpression::Eval`) is translated;
`INTERPRETER_TRUNK` in `units/fallout_shared/tesscriptrunmanager.rs` lists
it and fails when a member is not registered. CI runs it on generated
compiled scripts only. With the player's installation, an ignored test runs
every quest script of `FalloutNV.esm` once through it on generated forms
and prints which untranslated functions stop them:

```sh
NV_DATA="<...>/Fallout New Vegas/Data" cargo test -p engine --test real_scda -- --ignored --nocapture
```

The executable is found next to `Data` (`exe::installed_exe`) and refused
unless it is 1.4.0.525 (`exe::verify`), the check a game start-up can reuse.

## In the running game

The `host` crate starts the engine for the running game. The viewer calls
`host::Host::start(<Data>)` before loading anything: it opens the
`FalloutNV.exe` next to `Data` (read-only), refuses every build but
1.4.0.525 with the file and the reason, maps the data sections and runs the
allocator start-up the probes run (`00401000`, `00aa2230`, `00aa21d0`,
holder at `011f6118`, `00aa2020`: the body of `00866690`, which the memory
manager's constructor runs in the game); after the static initializers it
runs `NiInit` (`00aa20b0`, as `main` does at `0086a9f8`, `host::start_ni`)
once that is translated, through the movement's miss recorder (stage
`niinit`), so a stop is recorded and start-up goes on. `NiInit` keeps the
holder already made, and `00aa2020` runs twice in the game too (`00866690`
and `NiInit`'s `00aa3050`), so nothing is done twice that the game does once.
The rest of `WinMain`'s start-up is not reproduced. The startup line ends
with `NiInit: ran` or `NiInit: not run (why)`. The `Host` is a Bevy non-send
resource (main thread only; the engine is not `Send`).

Nothing runs through it yet (script track step S4): the game's scripts
still run through `crates/script`.

`host::script_form` (step S5) makes a script record into a `Script` form
in the game's memory and carries a script instance's variables between
the world (`script::interp::Locals`, by name) and the game's
`ScriptLocals` (by `SLSD` id, through `ScriptLocals::SetVariable` and
`GetVariable`; references as form ids in the low word). `Script::Load`
itself is bridged: its file reading (`TESFile`, `004726f0`, `00401660`,
`ScriptVariable::Load` `005e2a20`) is not translated, so the record's
bytes are put where it puts them; referenced forms stay unresolved.
With the installation, an ignored test takes all 322 quest scripts of
`FalloutNV.esm` (3,642 variables, 178 references) through the game's
locals and back unchanged:

```sh
NV_DATA="<...>/Fallout New Vegas/Data" cargo test -p host --release --test real_quest_locals -- --ignored --nocapture
```

`host::global_form` makes a global (`CreateFormOfType(6)`, the `TESGlobal`
constructor `005a6320`, then `FNAM`/`FLTV` where `TESGlobal::Load` puts
them) and carries its value to and from the float at +0x24 that the
interpreter reads (`00526ac0`); the same ignored test checks every global
of `FalloutNV.esm`.

`host::quest_form` (step S6) makes a quest the way the game loads a form:
the constructor (`CreateFormOfType(0x47)`, `TESQuest` `0060c4b0`), then its
record read by the game's own code through the plugin's `TESFile`
(`host::game_file`): `TESFile::SetOffset` (`004723a0`: seek to the record,
read its header), `TESQuest::Load` (`0060ceb0`: every chunk through
`GetTESChunk`/`GetChunkData`/`NextChunk`, `TESFile::LoadForm` giving it
its form id and file, the stages and their items with result scripts,
objectives and targets, conditions), and the quest's `InitItem`
(`0060cba0`: the script resolved through the game's form map, its
`ScriptLocals`, stages, objectives, conditions). Its script runs through
the game's interpreter: `0060d4d0` → `Script::Run` →
`ScriptRunManager::Run` → `ScriptRunner::Run` → `EvaluateLine` /
`CExpression::Eval`.

Only the file's bytes are platform. `BSFile` (`bsfile_xenon.cpp`) is
platform: `units::platform::open_memory_file` makes a `BSFile` (the game's
vtable, ready, position 0) over an in-memory copy of the parts of the file
the host supplies, at their real offsets (the record and the 0x18 bytes
after it, from `esm`); `BSFile::ReadF` (`00b00650`) and its seek (vtable
+0x14, `00b00590`) are stand-ins that read and move in it, and a read of
anything not supplied stops loudly. Both are also translated
(`bssystem::bsfile`, for the files the game opens itself through the CRT,
the archives); `host::game_file` routes the two addresses to the stand-ins
for its in-memory files only (`platform::is_memory_file`). Bridged: `TESFile::OpenTES`
(`00470c70`, which finds the file, constructs the `BSFile` and reads the
file's `TES4` header and masters); `host::game_file::GameFile::open`
constructs the `TESFile` with the game's constructor (no name: it opens
nothing) and sets what `OpenTES` leaves (`m_pFile`, `m_filesize`,
`cCompileIndex`). So plugins with masters aren't supported yet, and
compressed records (zlib, `TESFile::DecompressCurrentForm`) are refused
with a message. `Script::Load` is still bridged (`host::script_form`, the
record's bytes put where it puts them); its form goes into the form map
through `TESForm::SetFormID`.

All 300 scripted quests of `FalloutNV.esm` load this way: 126 have stages
(913 in all), each matching the record's `INDX` list. `SetStage`'s path
(`TESQuest::SetStageDone` `0060d510` → the stage's mark-done `0060f4a0` →
`TESQuestStage::SetDone` `0060fb60` for each item) runs: on a stage without
items it completes and `GetStage` (`TESQuest::GetCurrentStage` `0060d700`)
reads the new stage; on a stage with items `SetDone` stops at the first of
its untranslated callees (`004b0fc0`, `004b10f0`, `0060da90`, `0060ca30`,
`005ec060`, `00681050`, `00682320`) and beyond them needs game objects the
host doesn't have (the player).

Start-up makes the save/load game singleton the constructor needs
(`BGSSaveLoadGame::Create` `00847480`, from `TES::TES`; the rest of
`TES::TES` doesn't run: `00c44980`, `004ba280`, `00528cb0`, `00626fd0`,
`0084fc70`, `00856380`, `00c3da50`, `00624bf0` are open).

The acceptance test runs `VMS49Timer` (`VMS49TimerSCRIPT`): its first
GameMode run sets `bInitialized` to 1 and `fTimer` to 1.0; with `fTimer`
then 0.5, a second run keeps it. A survey loads and runs every scripted quest of
`FalloutNV.esm` once: 296 of 300 run to the end, 13 change variables on
their first run. The four that stop need game objects the host doesn't
have yet: the player (`GetPlayerControlsDisabled`,
`DisablePlayerControls`), the `BSAudio` singleton (`PlayBink`) and
`BSDLCSystemUtility` (`00af26c0`). Referenced forms (`SCRO`) are still
unresolved, so a line that uses one fails as the game fails on an
unresolved object.

```sh
NV_DATA="<...>/Fallout New Vegas/Data" cargo test -p host --release --test live_quest_script -- --ignored --nocapture
NV_DATA="<...>/Fallout New Vegas/Data" cargo test -p host --release --test live_quest_survey -- --ignored --nocapture
NV_DATA="<...>/Fallout New Vegas/Data" cargo test -p host --release --test live_quest_stages -- --ignored --nocapture
```

In the viewer, quests run through it (`host::quest_vm`, the world's
`world::scripting::QuestVm` hook on `Runner`): each frame's quest
`GameMode` runs (`Runner::update`) offer each quest to the host, which
takes those whose script has `GameMode` blocks only, no `ref` variables,
only its own variables and no function but `GetSecondsPassed`
(`host::quest_vm::takes`; 16 of 465 scripted quests with the DLCs, all of
them variable holders today); the rest run through `crates/script` as
before. A taken quest is made on its first run (`host::quest_vm::make_quest`,
the one place to swap in the real `TESQuest::Load` and `InitItem` when they
are translated), its variables are copied into its `ScriptLocals` before
each run and back after, and the world's `seconds_passed` is the frame's
elapsed time `GetSecondsPassed` reads (`011f6394` +0xC, `0084d030`, through
`0059c430`) with the script's own delay and get-seconds buffer (`Script`
+0x38, +0x3C) cleared: the world keeps the quests' run schedule. A quest
whose making or run stops goes back to `crates/script` for good. The
startup prints `Game's interpreter: runs N of M scripted quests`, and the
F12 report how many quests ran (and how many went back). Opt-in check:
`live_quest_vm` (a new game's taken quests run through it; `VMS49Timer`,
taken by the test though the viewer doesn't, counts down with the world's
time exactly as `crates/script` does, run by run).

Not yet: commands (the handlers aren't connected to the world, so
scripts that call one stay on `crates/script`), stages (`SetStage`/
`GetStage` need the stage list), and the frame's call site (`00455490` at
PostSwapProcess).

### The player's movement (Build 43)

The viewer offers each frame's player move to the game's code first
(`host::player_move`, `Host::move_player`; the viewer's `walk::walk`):
`PlayerMover::Update` (`009e9e50`, the mover's vtable +0x14) →
`Actor::Move` (`008a62b0`) → `MobileObject::Move` (`0092f260`) →
`bhkCharacterController::Move` (`00c73170`) → the proxy, casts, agents and
shapes. The state it needs is made by the game's own code, in the game's
order, each stage once (the cell's once per cell): `InitHavokSystem`
(`00624bf0`); the player as `Main::InitTES` makes it (`operator new`
0xE50, `PlayerCharacter::PlayerCharacter` `00938180`, `011dea3c`, form id
0x14; its base `NPC_` 7, `SetObjectReference`, position, angle,
`InitItem`); the player's cell (`CreateFormOfType`,
`TESDataHandler::LoadForm`, `InitItem`; its references' base forms first;
`TESObjectCELL::LoadTempDataFromFile` `00550500`, `Load3D` `00545cf0`, which
makes the cell's `bhkWorld` in `00552dc0`, and each reference's `Load3D`
`0056b2d0` and `InitHavok` (vtable +0x1C4, `005768b0`: its collision into the
cell's world through `bhkWorld::AddObjects`)); then `SetParentCell`, `PlayerCharacter::Load3D` (`0094e1d0`)
and `MobileObject::InitHavok` (`00930c70`, the controller). Each frame the
mover's bits are set with `PlayerMover::ForceMoveMode` (`009ea3e0`) and its
`Update` runs; the player's position (vtable +0x1F4) and the controller's
`OutVelocity` come back.

Bridged, and said so in the module: the file reading (as for quests); the
keys (the control handler `0093e860` reads DirectInput; the viewer gives the
same move bits); a resync of the position after a frame the viewer moved
itself; each form's `InitItem` right after its load (the game runs them all
after loading); each reference's `InitHavok` right after its `Load3D` (the
game calls it when the reference's queued 3D task is unpacked,
`TaskUnpackFunc` `0087b990`); `InitTES`'s `CastPermanentMagic` left out;
jumping not connected (a jump frame goes to `crates/physics`).

The loose objects can move by the game's whole Havok world too
(`--physics engine`; the default `old` keeps `crates/physics`), stepped
where the game steps it, on AI linear task thread 2. The stage `threads`
(after `havok` and `player`) sets `iNumHWThreads` from the processor count
(1 made 2) and runs `AILinearTaskThreadManager::CreateThreads` (`008c7290`)
as `main` does. Each frame `Host::run_ai_threads`, in the AI stage where the
viewer stepped its own bodies, stands in for the frame timer (the viewer's
seconds to `bhkWorld::SetDeltaTime` `00c66760` and to `011dea30`) and runs
`008c80e0(1)`, `StartThreads` (`008c78c0`) and `WaitForThreads` (`008c7990`)
as `Main::OnIdle` does: the wait runs threads 1 (`008c7da0`) and 2
(`008c7f50`) through their stage events, and thread 2 runs
`TES::UpdateCellAnimations` (`00453550`): the cell's `UpdateManagedNodes`
(`00551890`, `bhkWorld::Update`, `hkpWorld::stepDeltaTime`) and the
scene-graph update that copies each moving body's transform to its node.
The viewer then draws each placed object where its root node has moved; a
frame that stops (recorded under the stage `world`) goes back to
`crates/physics`, and a stopped step isn't tried again in that cell.

When a run reaches an untranslated address, `Engine::on_open` records it
with the nearest caller entered by address (`Engine::call_stack`), the run
stops, and the viewer moves the player with `crates/physics` for that frame.
`--movement old` keeps the old path only, `--movement compare` runs both
(the old one moves you) and `--movement-log FILE` writes both per frame;
`--movement-missing FILE` writes the addresses reached with counts, stage
and callers (and the other stops beside it, `FILE.stops.tsv`). Every 5 s the
viewer prints `Game's movement: ...` with each stage's state.
`--movement-probe` goes on past each miss with a zero result to list more
of them (indicative only; it never moves the player). Opt-in headless run:

```sh
NV_DATA="<...>/Fallout New Vegas/Data" cargo test -p host --release --test live_player_move -- --ignored --nocapture
```

`NV_MOVE_TRACE=FILE` appends, for each stage that stops, the last 4000 calls
it returned from (address, argument words, `EAX`; `Engine::trace`): the
quickest way to see which callee answered wrong before a fault.

State on 2026-10-18: no frame runs on the game's code yet. Havok's base
system starts (`hkBaseSystem` through `00cbbb10`, its 15 singletons); the
havok stage stops at `bhkCollisionFilter::InitFlags` (`00c828f0`), and after
it `NiStream::RegisterLoader` needs the loader map that `NiInit` (`00aa20b0`,
which `main` calls and the host does not yet) makes through
`NiStream::_SDMInit`. The player stage stops in the constructor's process
swap (`00907510`, `00906fb0`).

### Static initializers

The game constructs many globals before `WinMain`: `__cinit` (`00eccd14`)
runs `_initterm_e(010113c4, 010113e8)` (8 C initializers) and
`_initterm(00fdf4a4, 010113c0)` (17,728 non-null C++ dynamic initializers:
`Setting`s with their names and defaults, the NiAnimation dispatch tables,
computed float constants, `NiRTTI` records, Havok class records, ...).
`host::Host::start` runs both tables in `__cinit`'s order
(`engine::static_init::run`) right after mapping the executable.

Most entries are a few instructions of one of three shapes. They are not
one Rust function each: `research/initgen` reads every entry's code
(Ghidra's disassembly, or its bytes for the ~1,400 entries Ghidra made no
function of), checks it instruction by instruction against a shape, and
writes one row per matching entry to `crates/engine-core/data/static_initializers.tsv`
(16,937 rows; 799 entries refused, left for hand translation). The engine
has one translation per shape (`crates/engine-core/src/static_init.rs`):

- `reg f g`: `CALL f; MOV [g],EAX` (the 25 one-shot NiAnimation
  registrations, translated by initgen's `--rust` and checked by hand).
- `object ctor g dtor args`: a constructor run on a global with pushed
  words, floats or doubles (x87), then its destructor given to `_atexit`.
- `const ops`: x87 loads, arithmetic and stores (double precision, x87 NaN
  rules), the float square-root helper `004019b0`, calls without
  arguments, plain moves.

The ledger (`scripts/ledger`) counts each row as the translation of its
entry. The values come from the executable at run time (rows hold
addresses, never the executable's bytes).

Entry `00fb66a0` builds the `MemoryManager` singleton (`00aa3880`); the
memory manager is platform, so in its place, at the same point of the
table, runs the Ni allocator set-up `host::start_allocator` (the settings'
maps allocate through it): `host::STAND_INS`.

An entry without a row runs by address when translated. An entry whose
row needs an untranslated callee is skipped, and an entry that reaches an
untranslated function part-way is recorded with the stop; all are
counted (`Report`, and the start-up line). Measured 2026-10-13: 16,688
entries run from the table, 799 without a row, 243 with an untranslated
callee, 5 stopped part-way.

`crates/host/tests/static_initializers.rs` (opt-in, `NV_EXE`) runs it on the
real executable: three sample settings (`fGunShellLifetime`,
`fPlayerMaxResistance`, `fMaxArmorRating`) hold the name and default their
initializers pass, and 55 settings across all seven `Setting` constructors
and their three collections hold their names, values and vtables.


### Shut-down: the atexit destructors and the exit path

`research/initgen --atexit` does for shut-down what it does for start-up:
4,790 of the 5,020 `atexit` destructor functions (`00fc7a70` onwards) are
rows of `crates/engine-core/data/atexit_destructors.tsv` (shapes `call` and `tail`,
`engine::static_exit`; one more has a hand translation and the 229 others stay for hand translation). A row is
callable at its own address: when no function is registered at an address,
`Engine::call` and `Engine::is_translated` consult that table (a registered
function or a test double wins; a row whose destructor is untranslated is
not translated). `_atexit` (`00ec658f`) is a platform stand-in that records
its function in the CRT's own table (`01271a8c`/`01271a88`), and
`units::crt::doexit` (`00eccd99`) runs the exit sequence in the exe's order:
the registered functions last first, then the pre-terminator table
(`010113f4..01011400`), then the terminator table (`01011404..01011408`);
what is untranslated is skipped and counted (`ExitReport`), and the final
`ExitProcess` is only reported. The host does not call it yet.


## Lessons from the first sessions (2026-10-09)

- **Test doubles that keep state.** `e.register` takes a plain `fn`; a double that
  captures anything goes through `e.register_double`. `Engine::new()` registers
  every translated function, so a test doubles every callee outside its file
  or that callee runs for real.
- **Variable-argument functions** (cdecl `...`, e.g. `sprintf_s`, `PrintLine`,
  `Script::ParseParameters`): write the typed function taking the extra words
  as a slice, and register a small entry `fn(&mut Engine, &[u32]) -> Ret` as
  `(addr, entry as AbiFn)` (see `parse_parameters_entry` in `tesscript.rs`).
  Where the callee receives a `va_list`, copy the words into game memory and
  pass that address.
- **Structs passed by value** (points, matrices, `BSString`) are consecutive
  words in `args!`, in push order (first member at the lowest address).
- **Compiler-folded masks.** Blocks guarded by `flags & 0` (the compiler folded
  the mask to zero) never run; leave them out and say so in the doc comment.
- **Arguments come from the disassembly.** The decompiler often hangs a
  pushed word on the wrong call (`PUSH x; CALL getter; MOV ECX,EAX; CALL
  method`: `x` belongs to `method`), drops `this`, and mis-sizes `this`
  (indexing in `wchar_t` units). A function's parameters are exactly the
  words its `RET n` (or its callers' cleanup, for cdecl) shows; a word it
  never reads is still a parameter, named `_unused_<n>`.
- **PC layouts differ from the Xbox PDB in known ways.** `TESForm` is 0x18
  on PC (no editor ID or version-control fields), so fields of every class
  derived from it sit 0x10 lower than the PDB says (`TESObjectREFR`,
  `TESObjectCELL`, ...). Some classes lose the MemDebug build's leading
  accounting words (`Model`: 8 bytes). Gamebryo transforms are 16-byte
  aligned on Xbox only. Always confirm offsets against the PC code.
- **Map names can be wrong for folded code.** Identical-code folding puts
  one method's name on another's body (`005f36f0`, named
  `ActorMover::GetPreferredMoveMode`, returns `TES::pInteriorCell`). Name
  by the body.
- **Script command bodies** share one cdecl signature of 8 words
  (`ScriptArgs` in `units/fallout_shared/tesscriptfunctions.rs`).
- **Split units.** The largest units are split into part files
  (`<stem>_p2.rs`, ...), each with its own owner and address range; the
  main file's owner declares shared constants and helpers `pub(crate)`.
- **Tests.** `e.call_log` includes the top-level call the test makes;
  calls between functions of one file are direct and not logged. Doubles
  that keep state use `register_double`. Tests that read exe data map the
  page and write the values the code reads.
- **Scratch files** never go to `/tmp` or a shared folder: translators run
  in parallel.
