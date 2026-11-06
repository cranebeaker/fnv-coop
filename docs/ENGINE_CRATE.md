# The engine crates: translating FalloutNV.exe function by function

> **2026-11-03:** the hand-written game layer and the Bevy viewer this file also describes (`world::`, `physics::`, `cellview`, `preview`, the `nvinspect` commands, `viewer/src/...`) were removed ([ADR-0011](adr/0011-old-layer-removed.md)); their code is in git history at `93922e5a`. The executable research here stands; nv-rs now runs the game through the engine port only.

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
  Each `Mem` reserves one 4 GB block of host address space, so guest
  address `a` is host address `base + a`, with a byte per 4 KiB page just
  below it saying which pages are mapped; pages cost memory once mapped
  (`mem.rs`). The guest space is that of a large-address-aware 32-bit
  process on 64-bit Windows (what the community's 4 GB patch makes of
  FalloutNV.exe): the heap has a lower range `0x20000000..0x7E000000` and an
  upper one `0x80000000..0xF0000000` (about 3.2 GB together), the upper used
  only once the lower cannot serve a request, so ordinary play places every
  block as before; between them are the platform layer's own addresses
  (`PLATFORM_AREA`: its COM objects' code addresses, module handles, the Bink
  state page, D3DX handles), and above the heap the freed-block check's words
  and room for reservations at a chosen address. `GetSystemInfo` answers the
  4 GB process's highest address, 0xFFFEFFFF, and `GlobalMemoryStatus(Ex)` its
  total virtual size, 0xFFFE0000. A stopgap: the modern systems lift the limit
  for good. `mem.rs` has the whole layout.
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
  `Mem`'s heap, a good-fit allocator that splits and merges freed blocks
  (`mem/heap.rs`). The game's threads run under the thread host
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
| `engine-core` | `Engine`, `Mem`, `abi`, `ptr`, `x87`, `types`, `exe`, `typed` (the typed migration's seam, [below](#typed-code)), the static initializer and `atexit` tables (`static_init`, `static_exit`, `data/`), the prelude and the `entry!`/`args!`/`layout!` macros; units `crt.rs` and `platform.rs` (the core of the platform layer: kernel objects, threads, `VirtualAlloc`, the memory manager, the clock, the loader's module handles; its `Layer` joins `engine-platform`) | - |
| `engine-platform` | the rest of the platform layer, not translation units: the window, Direct3D 9 and D3DX (modelled and real), DirectInput, DirectSound and the mixer, DirectShow, Bink, Ogg Vorbis, `mmio`, files, `LoadLibraryA`, the process, NVAPI (`src/*.rs`, formerly `engine-core`'s `units/platform/`; [below](#the-platform-crate)) | core |
| `engine-gamebryo` | `nimain`, `nianimation`, `nicollision`, `niparticle`, `nisystem`, `nixenonrenderer` | core |
| `engine-bs` | `bsaudio`, `bscore`, `bsmain`, `bsmenu`, `bsmovie`, `bsshader`, `bssystem` | core, gamebryo |
| `engine-havok` | `havok_sdk`, `bshavok` | core, gamebryo |
| `engine-speedtree` | `speedtree` | core |
| `engine-fallout-*` (twelve crates, [below](#the-fallout-crates)) | the game's own units: every `fallout_*` subsystem, `unplaced`, `facegen`, `d3dx9`, `remotelog`, split by area; the `extra_data` and `reference` adapters (in `engine-fallout-cell`) | all of the above, and the fallout crates below them |
| `engine` | nothing of its own: re-exports every crate (`engine::units::<subsystem>`, `engine::prelude`, ...) and registers every function | all (`engine-platform` too) |

The grouping follows the code: Rust references between subsystems (layouts,
shared helpers) only point down, and nearly every unit test only reaches
functions of its own crate or below (measured on the full suite before the
split: 7 of 34,679 tests reached further). Those 7 changed with the split:
three integration tests moved to the unit whose crate links everything they
run (`fabs_restores_the_control_word_through_ctrlfp` from `crt.rs` to
`fallout_shared_magic/effectsetting.rs`, the scrap-heap buffer test from
`platform.rs` to `bscore/scrapheap.rs`, the allocator start-up test from
`nisystem/allocator_support.rs` to `unplaced/run_00aa1010.rs`), and four
double a callee in the game's own units (then `engine-fallout`: `__allshl` in two tests of
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
  `units/mod.rs` under `cfg(test)`; a fallout crate also every other fallout
  crate, [below](#the-fallout-crates)), so a test there reaches exactly those
  functions; a call into a crate above stops with `open function <addr>`
  like an untranslated one, and the test doubles it. A program that links
  `engine-core` without an `assemble!` does not link.
- **Tests that need a crate above** (rare): double the callee in the test
  (`e.register(addr, ...)`), as for an untranslated one.
- `engine-core`'s `test-support` feature exposes the form-constructor
  checker (`units::crt::ctor_test`) to the other crates' tests.

### The platform crate

The platform layer's Windows side is its own crate, `engine-platform`, so a change to the
window, Direct3D 9, DirectInput, DirectSound, files or the other platform code rebuilds only it
and the crates above it (`engine`, `host`, the viewer), not every engine part crate. Its files
were `engine-core`'s `units/platform/` folder (moved whole; `src/*.rs`); they are not
translation units, so the crate has no `units/` folder for `ledger units` to generate.

- **What stays in the core** (`units/platform.rs`, `units/platform/loader.rs`): what the part
  crates and their tests use: the kernel objects behind handles, threads and their waits,
  `VirtualAlloc`, the memory manager, `BSFile` memory files, the clock, the critical sections
  and interlocked calls, TLS, and the loader's module handles, KERNEL32 procedures by name and
  fiber-local storage (the CRT's start-up finds them). Part crates do not depend on
  `engine-platform`; only `engine` (which registers it like a part) and `host` do.
- **How it joins** (`engine_core::units::platform::Layer`, `engine_platform::LAYER`): the
  layer's `units::funcs` installs it, and `Engine::new` runs that before it makes the kernel,
  in every program that assembles the layer (the `engine` facade, the layer's own tests).
  Then `platform::imports()` answers the layer's imports after the core's (no name is in
  both, so `register_imports` finds the same function as before); each engine's
  `Kernel::platform` holds the layer's `State` (the DirectInput, DirectSound and Direct3D 9
  objects, the windows, the run's disk and the process side; `state!(e).sound` in the layer
  where it was `e.kernel.sound`, `state_ref!(e).sound` to read it through a shared borrow,
  `engine_platform::state(&e.kernel.platform)` outside);
  its file and search handles are `KernelObject::File`/`FindFile` holding an `Opaque`
  object, and `CloseHandle` on a file runs the layer's `file_closed`; `GetProcAddress` on
  `D3D9.DLL` or `nvapi.dll` asks the layer's `proc_address`. A program without the layer (a
  part crate's tests) has the core's imports only.
- **Tests:** the layer's tests are its own (`cargo test -p engine-platform --lib`), with the
  C runtime's stdio tests that open files (`crt_stdio_tests.rs`, from `crt/stdio.rs`, which reach
  the core's addresses through `engine-core`'s `test-support` feature).

### The fallout crates

The game's own units (2.1 million lines, 1,749 files) were one crate,
`engine-fallout`, until 2026-11-03: any fix in them recompiled all of it. They
are now twelve crates by area, so a fix recompiles its own area and re-checks
the few crates above it. Measured on the laptop with the build queue's settings
(one crate at a time), one statement added to a function of five typical units,
2026-11-03:

| | Before (one crate) | After (twelve) |
| --- | --- | --- |
| Play build after the fix (`--profile play`, incremental) | 134-469 s, median 419 s; one rustc at 8.6 GB | 18-52 s, median 30 s; no rustc above 2.8 GB |
| Unit tests of the fixed unit (`cargo test -p <crate> --lib <unit>`, not incremental), `fallout_ai/highprocess` | 525 s, one rustc at 23.4 GB | 132 s (`engine-fallout-ai`), 5.1 GB |
| Every unit test of the game's units, built and run | 727 s, 23.6 GB | 861 s, 5.2 GB |

Other agents' builds ran beside both series. The before median (419 s) is 2.4 times the 178 s
median of 16 earlier play builds after an engine-fallout edit (2026-10-23 to 10-29, the queue
at one job); against that figure the play build is about 6 times faster, not 14.

The crates:

| Crate | Holds | Fallout crates its code needs |
| --- | --- | --- |
| `engine-fallout-facegen` | `facegen`, `fallout_shared_facegen`, `unplaced/run_00649ba0` | - |
| `engine-fallout-script` | `fallout_shared`: scripts, script functions, conditions, the compiler, quests; `unplaced/run_005c8a50` | - |
| `engine-fallout-pathfinding` | `fallout_shared_pathfinding`; `unplaced/run_006f50b0` | - |
| `engine-fallout-env` | `fallout_shared_sky`, `_distant_terrain_system`, `_region_system`, `_speedtree`, `_havok`; `unplaced/run_00664440`, `run_006271b0` | - |
| `engine-fallout-interface` | `fallout_interface`, `fallout_dialogue` | - |
| `engine-fallout-cell` | `fallout_shared`: references, cells, `TES`, extra data, model loading, animation, land, water, worldspaces, decals; the `extra_data` and `reference` adapters | facegen |
| `engine-fallout-forms` | `fallout_shared`: the form classes, the data handler, `TESFile`, inventory, save game, packages, formulas and the folder's other runs; seven `unplaced` runs whose layouts are forms | cell, facegen |
| `engine-fallout-magic` | `fallout_magic`, `fallout_shared_magic`, `fallout_shared_tempeffects` | forms |
| `engine-fallout-ai` | `fallout_ai`; `unplaced/run_008a81e0`, `run_00988750` | forms, pathfinding |
| `engine-fallout-misc` | `fallout_misc` (the main loop, save and load), `fallout_audio`, `fallout_debug`, `fallout_shared_settings`, `remotelog`, `xapilib`, `d3dx9`; `unplaced/run_0070bbe0` | ai, interface, cell |
| `engine-fallout-unplaced-a` | the other `unplaced` runs below 0x00800000 | facegen |
| `engine-fallout-unplaced-b` | the other `unplaced` runs from 0x00800000 | facegen |

Each `Cargo.toml` also names every fallout crate under these (ai: forms,
pathfinding, cell, facegen), because the generated `units/mod.rs` re-exports
every crate below and Rust reaches only a direct dependency by name. A fix in
one crate re-checks the crates above it: facegen seven, cell four (forms, magic,
ai, misc), forms three, pathfinding two, ai and interface one (misc); script,
env, magic, misc and the two unplaced crates none.

- **Where a unit goes:** `scripts/ledger/fallout_crates.tsv`, one row per
  (folder, unit) pattern, first match wins; part files (`actor_p3`) follow
  their unit. Two folders are split by unit between several crates:
  `fallout_shared` (cell, forms, script) and `unplaced` (nine crates). `ledger
  units` refuses to run while a file sits in another crate than the rules
  say, and `ledger scaffold` places new units with them; a new fallout folder
  needs a row first.
- **Paths stay.** A split folder keeps its one name: each crate's
  `units/<folder>/mod.rs` also re-exports, by name, that folder's units in the
  crates below it, and a crate that holds none of a split folder gets an
  inline `units::<folder>` module that re-exports them. So `super::<unit>` and
  `crate::units::fallout_shared::<unit>` reach a unit in a crate below, and
  `engine::units::fallout_shared::<unit>` reaches every one. The translations
  did not change, except that five items became `pub` for a crate above
  (`extradatalist::list_item`, `list_next`; the `bsfacegenmodel::face_check`
  module, its `enabled` and `shader_textures`), the `FISTP` helpers
  `x87_truncate_i32/i64` became `engine_core::x87::fistp_chop`/`fistp_chop64`,
  and FaceGen's speech check got a redirect for its tests (below: its five
  hooks called from other crates, `linked`, `removing`, `removed`,
  `node_freeing` and `node_allocated`, first read a `OnceLock` that only its
  tests set; diagnostic code only, no effect on the game).
- **Calls between the crates** go by address through `Engine`, as before; the
  facade registers all twelve (`units::PARTS`; a facade test checks that no
  address is registered by two crates, so the order never matters).
- **Tests (phase A).** Each fallout crate names every other fallout crate
  under `[dev-dependencies]`, and its generated `cfg(test)` assemble
  registers them, so its tests reach every translated function exactly as
  before the split (the same tests, under the same names). A dev-dependency
  that depends on the crate under test links that crate a second time (the
  library beside the test build); nothing in these crates is `#[no_mangle]`,
  and calls between them go by address, so that is harmless, except that the
  test build's statics are not the ones the other crates' code sees: FaceGen's
  speech check (`bsfacegenkeyframe::speech_check`) keeps its state in statics
  and is called from cell, forms and unplaced-a, so its tests send the library
  build's hooks to the test build's own (`speech_check::redirect`). Phase B
  (later) drops the dev-dependencies a crate's tests do not need, so a test
  build compiles its own crate only.
- **Rebasing a branch from before the split:** `git rebase origin/main`
  carries edits of existing files (every carve commit moves its files as
  exact renames; five unit files are edited in commits of their own).
  A branch that adds unit files or edits a generated `mod.rs` replays its
  patches with the paths rewritten (`git format-patch`, then the rename map
  `crates/engine-fallout/... -> crates/engine-fallout-<area>/...`, then `git am
  -3`), and runs `ledger units`.

## One unit, one file, one owner

Translations live in `crates/<crate>/src/units/<subsystem>/<unit>.rs`, one
file per source unit of the Xbox 360 prototype (Xbox PDB, ADR-0002): for
example `fallout/ai/processlists.cpp` is
`crates/engine-fallout-ai/src/units/fallout_ai/processlists.rs`. `ledger queue`
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
     `GetCurrentProcess`; the window, Direct3D, sound, input and file ones are
     `engine-platform`'s, which a part crate's tests do not have); report
     others you need.
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
5. **Not translated**: C++ exception frames (`__CxxFrameHandler` states)
   and SEH frames are left out (say so if a function relies on them); the
   compiler-generated initializers and `atexit` destructors in the tail of
   `.text` are `library`. Behaviour is kept instead (ADR-0013): a throw is a
   Rust unwind (`engine_core::cxx_exception`) that `try_catch` takes in the
   exe's 38 functions whose catch goes on, around their try states; a throw
   with no region open is a stop. A function's locals that the exe keeps in
   its frame and reads after the catch stay outside the closure.
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

## Typed code

The migration of [ADR-0012](adr/0012-modern-64-bit-engine.md) and
[ADR-0014](adr/0014-typed-migration-in-two-stages.md) replaces a translation with typed, native
Rust, a family of functions at a time. `engine_core::typed` is the seam it plugs into. No typed
game function exists yet and the family table is empty.

**The switch.** `NV_TYPED` is read once per process, and `Engine::new` runs `typed::apply` after
all registration: each family that is on is registered over the translations with
`Engine::register`, so "registered twice" doesn't fire and the translation stays linked. The value
is a comma-separated list, read left to right:

| Item | Means |
| --- | --- |
| (unset or empty) | The default: a family is on when it is past its gate |
| `off` | Every family off: the game runs the translations |
| `all` | Back to the default |
| `<family>` | That family on, past its gate or not |
| `<family>:dual` | On, with both forms live, for families that keep them (`Engine::typed_dual`) |
| `<family>:off` | That family off |

Anything else stops the start with a message. A family is a row in `typed::FAMILIES`; a family in a
crate above the core calls `typed::provide` from its `units::funcs`, as the platform layer installs
itself. Past its gate a family is on by default, with the translation kept as the fallback for a while
(ADR-0014). Its row also says which threads it runs on (`Threads`), which the float rule below and
the differential test read.

**Calls.**

- From anywhere to typed code by address (`e.call`, `e.vcall`, a callback word): unchanged. The typed
  function is registered at its exe address, so vtables and callback words keep exe addresses.
- From typed code to untyped: `e.call` as ever. A local whose address reaches untyped code stays in
  `with_stack`.
- Between typed functions: `typed_call!(FAMILY, 0x00abc123, path::f(e, a, b))`, with the address
  written as a literal. It is a Rust call when the family's direct flag is set, else `e.call` with
  the same words. Its callee's crate must be the caller's or one below it; a callee above stays
  `e.call`. The hygiene test checks that the literal is registered as that function, as far as text
  can read its path and renames, and that the function has the typed marker for that address. A test
  build stops when the address isn't in the named family.
- The two forms agree but for one thing: an `f32` result by address goes through the uniform form's
  `f64`, which quiets a signalling NaN, as the exe's `FLD` does, and a direct one keeps its bits. A
  typed function that can return one writes `.allow_nan_payloads()` in its differential test, and
  stores nothing it needs quieted.
- A plain Rust call of a registered function isn't allowed in typed code: it would skip the flag,
  doubles and diagnostics. A helper nothing registers is ordinary code.

**The direct flag** lives in the `Engine`, one bit per family, so each engine has its own:

- It is clear while any per-call diagnostic is on (what `Engine::diagnosing` lists: the call log,
  budget and counts, the hook, nv-trace, the return trace, the block check, `NV_PROFILE`). The
  full call path, which every call takes while a diagnostic is on, brings it up to date, so a tool
  sees every call from its first by-address call on. A build with debug assertions (the tests)
  also asks the diagnostics at each `typed_call!`, so a test sees the very next one.
- It clears for good on a family once anything is registered over one of its addresses: a test
  double or a built-in fix. So doubles keep working whichever crate's code makes the call, also
  a crate below the one under test, which isn't built with `cfg(test)`. That only holds if a call
  names the family that holds its address: what is registered clears that family, not the one the
  call names. A test build stops on the mismatch; a play build doesn't look.

**What it costs.** `Engine::call` gains nothing: its fast path doesn't read the seam, and its machine
code is the same as before, instruction for instruction, apart from field offsets (the engine grew
by 48 bytes). The full path, taken on a cache miss or with a diagnostic on, gains one load and one
branch while no family is on. A `typed_call!` with the flag set is one load and one test before the
Rust call, in play and release builds, and the checks above exist only with debug assertions on. A
direct call doesn't count against the thread host's time slice and isn't on `call_stack`.

**A risk for later.** The thread host hands over at calls by address: the time slice counts them and
its hook runs at them. A typed loop that waits on guest memory for another game thread, making only
direct calls or none, never reaches that hook, so it hangs where the translation handed off. Such
a loop has to make a call by address, or the platform wait the exe makes, in each pass. The same
count is why each batch of direct calls checks `WAKE_DELAY_CALLS` and the hand-offs per frame.

Measured on 2026-11-06 (`spread_dispatch_speed`, 48,000 functions registered, 8,000 called, release
build, the two builds alternating, 8 runs each):

| | Median | Best | Worst |
| --- | ---: | ---: | ---: |
| `Engine::call` before the seam | 4.72 ns | 4.60 ns | 4.77 ns |
| `Engine::call` with the seam, no family | 4.69 ns | 4.61 ns | 4.73 ns |

**A typed function.**

- Read fnv_decomp for flow and the exe for what decides; the audit's traps still apply. No decompiled
  text, paths or symbols in the repository, and no placeholder names. The marker goes directly above
  the function, as for a translation, in its own words: `// Typed from <addr>`, with
  `, read against fnv_decomp` after the address when it was (ADR-0009).
- A difference from the translation: the exe and the recordings decide, and a translation bug found
  this way gets noted.
- Native locals only where no address reaches untyped code. Layouts are declared once per class.
- **Float rule (ADR-0014):** code only the main thread runs computes in plain `f32`; code on other
  threads or both, in `f64` locals rounded at each `float` store; the exact x87 helper stays where a
  recording shows it matters. The family's `threads` row (`Threads::Main`, `Other`, `Both`) says
  which applies, and the differential test runs that row's control words: plain `f32` differs under
  the other threads' word, and `f64` rounded at the store under the main thread's. A family with no
  such claim stays `Both`.
- Float to integer goes through `x87::fistp_chop` and its family, never `as`, which saturates where
  the exe gets the integer indefinite. A typed folder's root denies clippy's
  `cast_possible_truncation`, which knows types. It also stops `u32 as u8` and `f64 as f32`: where
  the cut or the rounding is what the code means, it goes in a small named helper with a local
  `allow`.
- No `core::arch` and no x86 assumption. The core's check for `aarch64-apple-darwin` runs at each
  core change, once that target is installed.

**The differential test** (`typed::diff`, in the core's tests and behind its `test-support` feature)
compares the translation and the typed version, on fresh engines with the same setup, once per
generated input and control word:

```rust
Diff::new(entry!(0x00abc123, typed_version(Ptr<Thing>, f32) -> f32))
    .family(&THE_FAMILY)
    .inputs(&[Input::Word, Input::F32])
    .setup(|e, case| {
        let thing = e.mem.alloc(0x40);
        e.mem.set_u32(thing + 8, case.word(0));
        vec![thing, case.f32(1).to_bits()]
    })
    .run()
    .assert_same();
```

- **Three runs per case:** the translation; the typed version with every `typed_call!` by address,
  against the translation; the typed version with each `typed_call!` into a named family direct,
  against its by-address run, call for call. The last needs debug assertions (`Report::direct_runs` says
  if it ran). It finds a `typed_call!` naming the wrong function, and it stops on one naming a family
  that doesn't hold the address.
- **Families:** `.family(..)` once per family the call tree should have on, as many as it reaches. An
  address in no named family goes by address, as in play with that family off, so a real callee or a
  rewritten site further down is no finding. `Report::direct_calls` says how many went direct: 0 for a
  function with a `typed_call!` means its direct branch has run nowhere, so name its family. The
  control words come from the row of the family holding the function, and `Report::control_words` says
  which ran; `.control_words(..)` overrides them.
- **A non-leaf function:** a translation calls functions of its own unit plainly, which isn't logged,
  and typed code may not. `.translation_calls_plainly(&[addr])` drops those calls from the typed side
  only. Their words can't compare, but their effects do, so a wrong word shows by what it changes. The
  real function has to run on both sides, so a setup can't replace it. An overridable address can't be
  listed: its words have to compare, so that family's step moves the translation's call to `e.call`
  and lowers its frozen count. `crates/engine-core/src/typed/diff.rs` has the whole path in
  `a_non_leaf_caller_in_two_families_passes_when_right_and_fails_when_wrong`.
- **Inputs:** `Input::F32` and `F64` hold NaNs (quiet, signalling, with a payload, the x87 default),
  both zeros, denormals, values that round (1/3, 0.1, the next float above 1), and values around the
  integer range and past `f32`'s; `Input::Word`, `Flag` and `Of(&[..])` give the integers, one per
  branch that matters. All combinations up to `max_cases` (512), else every value of each argument
  and then a fixed pseudo-random sample.
- **Compared, by bits:** the return value; every outgoing call in order with its words and, at its
  start, a hash of all mapped memory outside the stack for locals and the clock, so a store moved
  across a call shows; the memory after the call and which pages are mapped; how the run stopped, a
  thrown object by its type and address or, on the stack, its bytes; the heap's top and free bytes,
  the control word and the clock after the call. Each case runs under both the main thread's and the
  other threads' control word, because x87 rounding differs.
- **Stack words:** a word pointing into the stack for locals compares as its offset in its frame,
  the frame's size and the bytes behind it (`stack_bytes`, 64), not by value, so a typed version may
  have fewer frames. A local that reaches a call, in a frame the typed version shrinks or splits, takes
  `.local_at(callee, word, size)`: that word compares as which local it is, in order of first
  appearance, and its own `size` bytes, not its offset or its frame's size. It then can't see a
  callee reading past `size`, into what the translation had beside the local, or a later local at the
  address of one already gone, which reads as the same one.
- **A stack address stored** in memory, or in the bytes behind a stack word, or returned, compares by
  value: it fails when the frames differ. With a `.local_at` in the test, the stored ones compare as
  "some stack address", whichever local they point at, and a returned one still by value.
- **A comparison that compared nothing fails:** `run` must be used, and `assert_same` fails when no
  case ran. A run where both sides stop alike fails unless `.expect_stops(|case| ..)` says that input
  stops, and an input it names that doesn't stop fails too. A stop at an untranslated address always
  fails: that is a double the setup lacks. A function can't be its own translation: the harness refuses
  one pointer on both sides, and hygiene the same name in both `entry!`s.
- **Not compared,** so a test of these has to look itself: the rest of the stack for locals; the
  kernel's objects and everything else in `Engine::kernel`; the heap's block map and free bins; the time slice and `call_stack`, which a
  direct call doesn't touch by design; calls another game thread makes; a by-address call that runs a
  row of the `atexit` table. The memory hash doesn't say where it differs; a difference that lasts to the
  end also shows with its address.
- **Letting something through:** `ignore_calls` is for a callee the translation calls and the typed
  version doesn't call at all, doing its work in its own body. It drops those calls from the
  translation's side only, so the typed version calling one still shows, and it refuses an overridable
  address. `allow_nan_payloads` lets NaN payloads differ where the float rule accepts it, in the return
  and in aligned words of memory and calls; an infinity or a number against a NaN still differs.
- **A double in the setup:** over a family's member, or over a callee the translation calls plainly,
  is refused: it would run on one side only. Over any other callee it is the usual way to stand in.
- The engines hold translations only, whatever `NV_TYPED` says, so only the function and the families
  named differ. A run costs about 1 ms of engine in the core's tests and more in a crate with more
  functions registered; `max_cases` caps it. A crate above the core needs `engine-core`'s `test-support`
  feature in its dev-dependencies (`engine`, `engine-gamebryo`, `engine-bs`, `engine-havok` and
  `engine-speedtree` don't have it yet). A test in `engine_core::typed` can't name a translation in a
  crate above: a typed leaf's test goes where its translation is reachable, in that crate or in
  `crates/engine/tests/`.

**Hygiene** (`crates/nvinspect/tests/repo_hygiene.rs`). Typed code is everything under
`crates/engine-core/src/typed/` and `crates/engine-typed/src/`, and every function under a typed
marker elsewhere, in the engine crates, `crates/host` and `viewer`. The rules read text, not types:

- In typed code: no `as` from a float to an integer (outside test modules), where the operand shows
  it is a float: a declared float in the same function, a float literal, or a float call. This is the
  first catch; the type-aware one is clippy's lint on each typed folder's root, which this test also
  requires. In a function under a marker in a translation's file only the text rule applies.
- In typed code: no `core::arch`, `std::arch` or `target_arch`; no decompiled text, comments included
  (C++ casts, `this->`, the decompilation's annotations, paths with forward or back slashes, `.cpp`,
  `.h`); no identifier holding an exe address but exactly the project's own `fn_<addr>` and
  `run_<addr>`. In a file with a typed marker, the layouts and constants get the same reading, with
  their comments.
- Each `typed_call!`, in any bracket form, has a literal address. The function it calls is the one
  registered there, as far as text can read its path, `use` lines and renames (75 names are registered
  at more than one address), and carries the typed marker for that address, so it can't silently call
  the translation. In typed code, no plain call of a registered function; a helper nothing registers,
  or one in a module that doesn't register that name (`helpers::read`, `f32::sqrt`), is ordinary code.
- Each marked function is registered at its address, and a `Diff::new` somewhere names that address.
- Markers are well formed and directly above a function; a function registered in a typed folder has
  the marker for that address above it, in any of the folder's files. A marker has one definition, `scripts/ledger/src/typed_marker.rs`,
  which the ledger and this test share: a line this test refuses, the ledger doesn't count.
- `crates/engine-core/data/overridable.tsv` lists the addresses a fix or a built-in can override: the
  plugins' hooked function entries (a lower bound: literal addresses only), `NV_UNCAP`'s timer step,
  and the four fixes-preset guards. `typed_call!` may not name one, and typed code may not call one
  directly. The translations' direct calls to them (131 today, one of them `NV_UNCAP` chaining to the translation) are frozen by count: a new one fails,
  and a removed one asks for the number to be lowered. Moving them to calls by address would change the
  calls the game makes, so it is left for each family's own step.
- **What stays review-only,** because this reads text: a call through a function pointer or a `let` of
  the function; a call a macro builds; a devirtualised or copied body; a bare name a local of the same
  name hides; a re-export under another module's name; whether a `Diff::new` that names the address
  really covers the function's branches; and a `typed_call!`'s family, which only a test build with the
  flag set checks, so a site no test runs that way is only as right as its review.

The ledger counts a typed marker in an engine crate with a registered address like a translation
marker, and `docs/LEDGER.md` has a `typed` column. CI runs the ledger's own tests and not `ledger
--check`: the ledger holds line numbers, which every merge moves, so regenerate it by hand and check
before a merge that touches markers.

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
