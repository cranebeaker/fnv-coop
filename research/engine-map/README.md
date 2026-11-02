# Engine map

The engine map gives every function of `FalloutNV.exe` 1.4.0.525 (unpacked,
SHA-256 `19406942…739f`) its Xbox 360 prototype name where one can be
transferred, its source unit (the prototype's `.cpp`) and its subsystem.
It is Phase 0 of [docs/ENGINE_PORT_PLAN.md](../../docs/ENGINE_PORT_PLAN.md)
and the input of the function ledger ([scripts/ledger](../../scripts/ledger),
[docs/LEDGER.md](../../docs/LEDGER.md)).

`engine_map.tsv` here is the committed result, with `frame.tsv` (the
per-frame call tree, "Frame table" below). They hold only addresses,
sizes, Xbox PDB names, Xbox addresses, source-unit paths and subsystems
(ADR-0002). No decompiled code, strings, bytes or databases. Everything
below runs on private inputs, and its intermediate files stay in the private
research tree (`%USERPROFILE%\nv-re\work\phase0` on the maintainer's
machine).

Names in the map come from `Fallout_Release_MemDebug.pdb` (Xbox PDB, GUID
`036BA387-1386-4F96-A709-DA70225CDF94`, age 1) and its PowerPC image
`Fallout_Release_MemDebug.exe`.

## Columns

| Column | Meaning |
| --- | --- |
| `address` | PC entry point |
| `size` | bytes in the function's body (Ghidra) |
| `name` | Xbox PDB name, sanitized for Ghidra (`*`→`P`, `&`→`R`, operators spelled out, repeats get `_ovN`); in the C/C++ runtime also a Function ID name |
| `name_tier` | evidence for the name: `vt`, `str`, `cg`, `fid` (below) |
| `xbox` | the matched Xbox function's address |
| `unit` | the Xbox object file's main source file, relative to the code root (`fallout/ai/processlists.cpp`), or `<library>/<file>` |
| `subsystem` | the unit's folder (`fallout/ai`, `fallout shared/pathfinding`) or library (`NiMain`, `Havok SDK`, `BSHavok`, `LIBCMT`) |
| `placed` | how the unit was found (below) |

## How it is made

```text
Ghidra project (private copy)         Xbox PDB + PowerPC image
  NvCreateFunctions.java (writes)       |
  NvEngineMap.java (read-only)          xbox  (this crate)
        |                                |
        +-------------- match -----------+   names.csv -> NvImportNameMap.java (writes, copy)
                          |
                         map  ------------>  engine_map.tsv  -> scripts/ledger -> docs/LEDGER.md
                                                   |                    ^
                         frame <-------------------+                    |
                           +--------------->  frame.tsv  ---------------+
```

1. **Complete the function set** on a private copy of the analyzed project
   (the shared server's project, `ghidra_mcp`, copied). Ghidra's analysis
   never made functions at many vtable slot targets: `bhkWorld::Update`
   (`00c6ae70`), which the Havok port cites, was one of them.
   `NvEngineMap.java` lists vtables from the exe's own MSVC RTTI;
   `NvCreateFunctions.java` creates a function at each slot target that is
   not one, then at the call targets of the new functions. On 2026-10-09:
   3,302 targets, 3,271 created, 28 inside another function's body (left
   alone, reported), 5 already functions. 62,983 → 66,259 functions.
2. **PC facts**: `NvEngineMap.java` (`research/ghidra`, read-only) writes
   every function's size, name, ordered calls, imports, referenced strings
   and reference counts, and every RTTI vtable with its slots.
3. **Xbox facts**: `cargo run --release --bin xbox -- <pdb> <exe> <dir>`
   reads the PDB (repacked in memory to 4096-byte pages, as
   `research/pdb/msf_repack.py` does) and the PowerPC image: 69,251
   functions (procedure records, plus 4,766 code publics that have none,
   such as `bhkWorld::Update`), their modules and source files, 3,650
   vtables (`??_7…`), ordered `bl` calls and referenced strings (`lis` +
   `addi`/load pairs).
4. **Match**: `cargo run --release --bin match -- <pc-dir> <xb-dir> <out>`
   (tiers below) writes `matches.tsv` and `names.csv`.
. **Frame**: `cargo run --release --bin frame -- <pc-dir> <xb-dir> engine_map.tsv frame.tsv [<report>]`
   writes the call tree of the per-frame function (below, "Frame table").
7. **Names into Ghidra**: `NvImportNameMap.java` with `names.csv` on the
   private copy (dry run, then real run; `research/ghidra/README.md`
   section 2). On 2026-10-09: 17,405 applied, 191 left as conflicts with
   existing Function ID names (not forced), 4 unchanged.

All seven steps run with one command, `.\scripts\engine-map.ps1` (69 s on the
maintainer's machine; `-FreshProject` copies the source project again and
redoes step 1, which is needed whenever the matcher's names change, because
the import does not overwrite names it set before). After Rust changes only
the ledger needs to run:
`cargo run --release --manifest-path scripts/ledger/Cargo.toml`.

## Frame table

`frame.tsv` is the call tree of `Main::OnIdle` (Xbox PDB; PC `0086e650`,
Xbox `8269f3c8`), the function `main` (`0086a850`) calls once per frame
([docs/FRAME_SKELETON.md](../../docs/FRAME_SKELETON.md)). Depth 0 is
`Main::OnIdle`, depth 1 its calls, down to depth 3; one row per call site in
call order, so a function called twice has two rows and two subtrees. Only
direct calls appear: calls through a vtable or a function pointer (the Havok
step, the AI task threads) do not; FRAME_SKELETON.md traces those by hand.

| Column | Meaning |
| --- | --- |
| `seq` | row number (depth-first order) |
| `depth` | 0 to 3 |
| `call` | position of the call in its caller's PC call list (1-based) |
| `address` | PC entry point of the called function |
| `name`, `xbox` | Xbox PDB name and address |
| `name_tier` | the map's tier (`vt`, `str`, `cg`, `fid`), or `frame`, `align-sim`, `align-gap` (below) |
| `unit`, `subsystem` | from the engine map |
| `note` | `recursive`: already on the path above, not expanded again |

Names the map does not have come from two places:

- `frame`: pairs confirmed by hand for the frame (`CONFIRMED` in
  `src/bin/frame.rs`), each with its evidence from the exe in
  FRAME_SKELETON.md's stage table.
- Alignment: where a caller's Xbox counterpart is known, its PC and Xbox
  call lists are aligned on calls already paired; inside the gaps, calls
  whose own callees and strings overlap in two or more members pair in order
  (`align-sim`), and a gap of exactly one call on each side pairs those two
  (`align-gap`). A proposal is kept only when every call site of the PC
  function agrees and no other PC function is proposed for the same Xbox
  function; kept pairs feed the next round. These are leads, not checked
  over the whole exe.

The ledger does not read a status from this table: it computes one per row
from the Rust sources and prints the coverage per depth, and the depth-1
list, in [docs/LEDGER.md](../../docs/LEDGER.md) ("Frame").

On 2026-10-09: 6,692 rows (1 / 143 / 1,575 / 4,973 at depths 0 to 3);
named 1 / 84 / 461 / 1,449, of which `frame` 1 / 42 / 25 / 20 and
alignment 0 / 1 / 24 / 93.

The optional fifth argument writes the root's alignment (both call lists
merged, with the Xbox calls the PC lacks); it stays private with the other
intermediate files.

## Name tiers

Every tier keeps a pair only if it is one-to-one: the PC function has
exactly one Xbox candidate, and that Xbox function exactly one PC candidate.

| Tier | Evidence | 2026-10-09 |
| --- | --- | ---: |
| `vt` | The PC RTTI type name equals the Xbox vftable's class, compared as mangled text (`.?AVbhkWorld@@` and `??_7bhkWorld@@6B@`); both tables have the same slot count; slot k pairs with slot k. Secondary tables pair in address order when both sides have the same number. 2,193 tables paired, 166 skipped for differing slot counts, 662 candidates dropped as not one-to-one (stubs shared by identical-code folding). | 8,571 |
| `str` | A string of 8+ characters referenced by exactly one PC function and exactly one Xbox function (same reading rule on both sides). | 1,716 new (2,036 pairs; 320 also `vt`, all 320 agree) |
| `cg` | Call graph, iterated to a fixed point from `vt` + `str`: (a) align the PC and Xbox call lists of a matched pair on calls already matched; a gap of equal length pairs position by position if it has length one or two matched callers propose the same pair; (b) a function's signature, the matched functions it calls plus the matched functions that call it, shared by exactly one unmatched function on each side. PowerPC save/restore helpers are left out of Xbox call lists. | 7,317 |
| `fid` | Ghidra Function ID name, only for unnamed functions placed in `LIBCMT`/`libcpmt` (elsewhere Function ID names are mostly wrong: `ExtraLock::ExtraLock` was `CPrintDialog`). | 377 |

**Measured error.** Hold-out: the `cg` tier seeded with half of the `vt`
pairs rediscovers 637 of the hidden half: 630 right, 7 wrong; of its 6,573
pairs outside `vt`, 4 take an Xbox function that `vt` gives to another PC
function. That is about 11 errors in 641 checkable pairs (1.7%). `str`
agrees with `vt` on all 320 shared functions. Independent check: of the 109
`Class::Method` (`address`) pairs already written in our Rust comments that
the map also names, 93 are identical and 3 differ only in the class
(`SeenData` / `IntSeenData`); the rest are our own descriptive names. The
`vt` tier cannot be checked against itself; PROTOTYPE_SYMBOLS.md's
100-row spot-check per tier is still to be done by hand.

## Placement

| `placed` | Rule | 2026-10-09 |
| --- | --- | ---: |
| `name` | the matched Xbox function's module | 17,502 |
| `range` | unnamed, and the nearest named functions before and after it come from one unit (MSVC lays out each object file contiguously) | 16,005 |
| `calls` | every placed caller is in one unit, which is also the unit of an adjacent named function | 1,275 |
| `range-sub` | the neighbours are different units of one subsystem; subsystem only | 7,789 |
| `init` / `atexit` | the compiler tail: after the last function with a caller or a vtable slot, functions referenced only from data are dynamic initializers of globals, functions whose address is pushed (to `atexit`) are their destructors | 16,315 / 5,020 |
| `none` | not placed: the ledger shows them as `(unplaced)` | 2,353 |

Hold-out: with every other named function hidden, `range` puts 6,230 of
6,369 in the right unit (97.8%) and `range-sub` 2,039 of 2,091 in the right
subsystem; `calls` 77 of 83.

The dynamic initializers carry the default values of many globals, game
settings included (for example `sHeadDamagedMessage`); the ledger counts
them as `library` code, but they remain a data source.

The `(unplaced)` functions sit where link order interleaves objects that
were not matched, such as a 23 KB block between `fallout/ai`'s last
package and `BSMenu`'s `Tile`. More names (or the spot-checks below) will
place them.

### Functions Ghidra never created

The map lists Ghidra's functions, and Ghidra made functions where its
analysis followed calls. Code reached only from data (vtable slots, function
tables, Havok's `hkSingletonInitNode` records, the `__cinit` table), from a
pointer immediate (`push offset f`, `mov [x], offset f`) or from a tail
jump in other such code never became a function, so the map and the ledger
missed it. `ghidra_gaps.tsv` (committed here; address, size, unit,
subsystem, evidence) lists it, and the ledger adds its rows to the map with
`placed` = `gap`. It was made on 2026-10-18 from the unpacked exe: every
relocation whose value lies in `.text` outside every map function (from
`.rdata`/`.data`, or an immediate operand of a decoded instruction), every
`CALL`/`JMP rel32` target of decoded code outside every function, iterated
to a fixed point over the code found; each entry decodes as ordinary code
up to a `RET`/`JMP` (its size runs to the last instruction before `int3`
padding, a jump table or the next function), and the private Ghidra
(read-only) answers no function at it. Left out: the C++ exception-handling
code (12,270 unwind funclets and 30 catch blocks from the `FuncInfo`
tables, 4,346 `__ehhandler` stubs), jump-table case labels and other
targets of jumps from the function just before them (pieces of that
function), and unreferenced code islands (dead code). The unit is the
neighbours' unit when the map function before and after it share one,
else their subsystem when they share one (the ledger then gives it a run
unit), else none (`(unplaced)`). Gap functions do not change the map's run
units: one without a unit joins the run before or after it, else gets its
own.

On 2026-10-18: 3,216 rows (8 of them inside the body Ghidra gave another
function, as that function's tail-jump chunk, but also vtable slots or
data targets): 1,436 dynamic initializers in the compiler tail (1,174
already run from the initializer table), 786 in `Havok SDK` (the 15
singleton creators among them; 354 are pointers that static initializers
store), 439 in `BSShader` (mostly vtable slots, many of them
this-adjusting thunks), 134 in `NiAnimation`, 103 in `NiMain` (`00a89cc0`,
`00a8a280` and `00a8a5f0`, which pushes both, among them), 115 in
`LIBCMT` and the rest in the other renderer and Gamebryo subsystems.
Evidence (a row can have several): 1,437 `.rdata` table entries, 480
vtable slots, 377 `.data` pointers, 573 pointer immediates, 314 pushed
pointers, 26 calls and 25 tail jumps.

The ledger gives every function without a unit one, on top of this map
(the map itself is not changed):

- `unit_overrides.tsv` (committed here; address, unit, subsystem, replaces,
  evidence): a function placed by `range-sub` that is an RTTI vtable slot of
  a class whose other methods sit in one unit joins that unit; the ledger
  shows it as placed by `vtable`. `replaces` is empty for such a function;
  a function the map placed in a unit by a wrong name match moves only when
  `replaces` names that unit, and any other mismatch is an error. On
  2026-10-17: 101 rows, 98 without a unit (44 `NiDX9Renderer` slots in
  `NiXenonRenderer/nixenonrenderer.cpp`, `NiDX9FragmentShader`,
  `NiDX9ShaderDeclaration` and other renderer classes, the Xbox system
  utilities, a `NiFile` method, the `Archive` and `BSFile` destructors) and
  3 wrong name matches (a `BSFileCache` slot named into `xaudio2`, two
  `NiTMap<const char*, char*>` slots named into `BSDiag`).
- Every other run of neighbouring functions with one subsystem and no unit
  becomes `<subsystem>/run_<first address>`, in every subsystem except the
  runtime libraries; a run in a platform subsystem keeps its status (the
  ledger's `GAME_FUNCS` lists the game code inside them).

## Known limits

- The Xbox build is from 2010-11-10, the PC build later. PC-only code (the
  D3D9 renderer, DirectInput, Steam) has no Xbox name; where it sits
  between named Xbox units it is placed by range like any other code, and
  where it does not it stays unplaced.
- `cg` names can be wrong at about the measured rate; treat a name as a
  strong lead, and confirm it (strings, callees, layout) before relying on
  it in a translation.
- Function sizes of public-only Xbox functions are upper bounds (they run
  to the next symbol); this only affects the call lists.
- The Ghidra project copies with names and the created functions are
  private; the shared server still serves the unnamed project until the
  maintainer restarts it on the named copy.
