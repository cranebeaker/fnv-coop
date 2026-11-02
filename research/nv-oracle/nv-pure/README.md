# nv-pure: automatic oracle check for pure translations

Runs every pure translation on the real CPU through [`nv-call`](../README.md#nv-call)
and in the engine crate with the same inputs, and compares the results bit for
bit (docs/HANDOFF.md next task 10, #182). A wrong translation of a pure function
fails by itself, without a hand-written test.

It is a local check: CI has no copy of the game. Without `NV_ORACLE_EXE` it
prints that it skipped and exits 0.

## Pieces

| Step | What | Output |
| --- | --- | --- |
| 1 | `research/ghidra/NvPurity.java` on your own analysed project (read-only): per function, stores outside its own frame, indirect and imported calls, FS use, direct calls, `RET n`, ECX/EDX read on entry, stack-argument bytes. | `purity.tsv`, private |
| 2 | `nv-pure list --purity purity.tsv`: joins those facts with the `// Translated from` markers and `entry!` signatures of the engine crates (`crates/engine`, `crates/engine-*`). | `pure.tsv` (this folder): addresses, conventions, signatures |
| 3 | `nv-pure check`: generates inputs, runs them through `nv-call` and the engine, compares. | `%USERPROFILE%\nv-re\oracle\pure\` (vectors, results, `failures.tsv`), private |

Only `pure.tsv` is committed: addresses and signatures, no exe bytes or outputs.
The inputs are regenerated from a fixed seed (the function's address), so the
same vectors run everywhere.

## Which functions are checked

A translation is on the list when all of these hold:

- it has a `Translated from` marker and no `Translation incomplete` marker, and an
  `entry!` registration whose types the check knows;
- in the exe it is pure, transitively: it and everything it calls store only to
  their own stack frame, make no indirect or imported calls and don't touch FS
  (TLS, SEH frames), and none executes an instruction whose result no input
  determines (`RDTSC`, `RDTSCP`, `RDPMC`, `RDRAND`, `RDSEED`, `CPUID`: the clock
  and the like, skipped as `HiddenInput`; hkSystemClock 00dd8e00);
- its arguments and result are scalars (`u8`..`u64`, `bool`, `f32`, `f64`), and
  neither it nor its callees read through a pointer (so no object has to be built
  for it); **or** (pointer tier, phase 1 of #193) it takes pointer arguments that
  are only read, at offsets the data flow pins down (see below). A pointer
  function that reaches itself is skipped as `RecursivePointer`: the callee
  check can't cover a function's own recursive call, which may walk a pointer
  loaded from its buffers (00b15110 recurses on `m_pParent`);
- the calling convention follows from the exe and agrees with the signature:
  ECX/EDX read on entry (thiscall, fastcall), the `RET n` size (stdcall/thiscall
  pop exactly the stack words the signature leaves), and the stack-argument bytes
  the body reads (never more than the signature passes). Functions where nothing
  fits are listed by `nv-pure list` as convention mismatches; each is worth a look,
  since a missing or extra parameter in a translation shows up this way. A body
  with no `RET` (a thunk that tail-jumps) has no checkable convention and is
  skipped as `NoRet`, not listed as a mismatch; one that takes an argument in
  EAX, EBX, ESI, EDI or EBP (from the decompiler's data flow) is skipped as
  `RegisterInput`, since nv-call passes only ECX and EDX;
- it doesn't take a leading `f64` in x87 ST0 (`nv-call` can't preset ST0).

`check` also skips a listed function when it or one of its callees is not
registered in the engine at the time of the run.

### Pointer arguments (phase 1: read only)

`NvPurity.java` follows each input (ECX, EDX, each stack argument) through the
decompiler's data flow (copies, casts, constant offsets, merges) to every LOAD and
STORE it reaches (a merge keeps a known offset only when all its inputs come from
the same input at the same offset, so a pointer walked in a loop, `p = p->next`,
counts as an unknown offset; functions with x87 code are decompiled once more with
the return in ST0, since the decompiler otherwise drops loads that only feed a float
result), and writes per input the offsets and sizes read and stored, pointer
fields that are dereferenced, calls it is passed to and whether it escapes
(`arg_access`), plus the LOADs/STOREs no input explains (`untracked_loads`,
`untracked_stores`). A function with pointer parameters is listed when:

- every pointer parameter (mapped to its exe input through the calling convention)
  is only read, at known offsets: no unknown offset, no dereferenced pointer field
  (that needs nested buffers), not passed to a callee, not stored;
- nothing else is read or stored through a pointer (`untracked_* = 0`), and its
  callees read through no pointer;
- no parameter the Rust signature types as a scalar is used as a pointer by the exe,
  and no input that matches no parameter is dereferenced.

Each pointer parameter gets a buffer (`buffers` in `pure.tsv`:
`param:size:bias:off.size;...`): big enough for every field read plus slack, with a
bias for negative offsets. Both sides place the buffers in nv-call's arena at
0x30000000, 16-byte aligned in order, the engine with `Mem::map`; before and after
every vector the engine heap's top must be below the arena, and nv-call's header
must report its arena there, or the run stops with an error.

## Inputs

1. **Per parameter**, every value below with the other parameters at a plain
   value (1.5 or 3):
   - `f32`/`f64`: ±0, ±1, 0.5, 2, 0.001, 1000, π, -π/2, 360, 0.1, ±1e30, max,
     smallest normal, smallest and largest denormal, ±inf, ±qNaN, qNaN with
     payload, sNaN; rounding ties and conversion limits (±0.5, ±1.5, ±2.5, 255.5,
     65535.5, the floats on either side of 0.5, 2^23, 2^24, the floats around
     ±2^31, 2^32, ±1e10).
   - integers: 0, 1, -1, 2, 3, 7, INT_MAX, INT_MIN, 0xff, 0x100, 0xffff, 0x10000,
     0xffffff00, 0x400000.
   - `u8`/`u16`/`bool`: their own edges, passed as the zero- or sign-extended word
     the uniform form uses.
   - **The function's own constants** (NvPurity mines them, `consts` in
     `pure.tsv`, callees' included): c-1, c and c+1 of every immediate it compares
     or computes with (`CMP EAX,0x14`), and every float/double it loads from
     memory with its two neighbouring values. Branch boundaries that no generic
     list hits come from here.
2. **For two or three parameters, every combination** of a small special set per
   type (floats: ±0, ±1, 1.5, qNaN, ±inf; integers 0, 1, 3, -1, INT_MIN): signed
   zeros against each other, equal operands, NaN on either side.
3. **Seeded random lists** up to `--count` (default 64, at least 16): floats half
   raw bit patterns (any class), half in ±1000; integers mostly small, sometimes
   any pattern.

The FPU runs with nv-call's defaults (control word 0x027F: 53-bit precision,
round to nearest; MXCSR 0x1F80), matching the engine's "compute in f64, round at
float stores" rule.

4. **Buffer contents** (pointer tier), each paired with plain scalars and then
   with the scalar lists above: all zero, all 0xff, two seeded random fills, and
   structured fills where every field the exe reads takes the values of its pool
   (typed edges by size, the function's constants, and for 4-byte fields the
   pointer arguments' own values, so `this->x == other` can be true), first
   shifted per field, then in lockstep (all fields the same value, so fields can
   equal each other). Every distinct fill runs once whatever `--count` is (up to
   68: four plain fills, then two passes of at most 32 rounds), so a pointer
   function can get more vectors than `--count`; the scalar lists then add theirs,
   each paired with one of the fills.

## Verdicts

| Kind | Meaning | Fails the check |
| --- | --- | --- |
| `Match` | Same bits (`bool`: AL zero/non-zero; `u8`/`u16`: low byte/word; floats: ST0 rounded to the type). | no |
| `BothFailed` | Both failed the same way: the exe's access violation and the engine's unmapped read at the same address (or both divided by zero). | no |
| `OutOfImageRead` | The engine stopped at memory it never maps (outside `.rdata`..`CONST`, e.g. a random table index landing in `.text`), which nv-call's process happened to have mapped: when both sides failed, or when the exe returned a value and may itself read an address computed from its inputs (`computed_reads` in `pure.tsv`: the function or a callee has LOADs no input explains). Each one is listed in `coverage.tsv`. | no |
| `RustStrayRead` | The exe returned a value, but the engine read memory it never maps, in a function whose exe reads only its buffers, image constants and the stack (`computed_reads` 0): the translation read memory the exe didn't, such as a number used as a pointer. | yes |
| `Mismatch` | Different results. | yes |
| `NanBits` | Both NaN, different sign or payload. | yes |
| `ExeFaultOnly` / `RustPanicOnly` | One side failed. | yes |
| `FaultMismatch` | Both failed, but at different addresses (inside the engine's mapped data) or in different ways. | yes |
| `Convention` | The exe popped a different number of bytes than the listed convention: the list is wrong, not the translation. | yes |
| `BufferMismatch` | Same result, but a buffer differs after the call (one side stored where the other didn't). | yes |
| `DataFlowMiss` | The exe hit an access violation, ran past `--exe-timeout` or ended nv-call's process, in a pointer-tier function, which only reads its buffers at known offsets and so can neither fault nor loop on them: the data flow missed a pointer (one walked in a loop), and the list is wrong, not the translation. | yes |
| `ExeTimeout` | The exe did not finish the vector within `--exe-timeout` (default 10 s; typically a loop over a count taken from a fuzzed scalar; in a pointer-tier function it is a `DataFlowMiss`). nv-call is stopped and restarted after it; not run on the engine side. | no |
| `ExeSlow` | The exe took over `--slow-ms` (default 1000 ms); not run on the engine side, where a correct translation may take much longer. | no |
| `ExeCrash` | nv-call's process ended on the vector (an exception outside the harness's reach); restarted after it. In a pointer-tier function it is a `DataFlowMiss`. | no |

All three are listed in `coverage.tsv`. The engine side runs on a worker thread under
a watchdog: one vector running past `--rust-timeout` (default 120 s) on a vector the
exe finished quickly stops the check with an error naming the function and inputs,
since a translation that loops where the exe returns is itself a finding.

The engine side runs every vector on a fresh engine (the exe's data sections
mapped anew), as nv-call restores the image's writable memory before every
vector, so no vector sees another's state.

The summary also prints how many functions give **varying** exe results over
their inputs. A function whose results are all the same value (no parameters, or
a result that depends on globals the game fills at run time, zero in the file) is
only checked for that one value; they are listed in `coverage.tsv` in the output
folder.

## Validation

- **Mutation test**, fifteen realistic bugs planted in translations, one per
  function, in two rounds:

  | Genre | Planted bug | Caught |
  | --- | --- | --- |
  | range boundary | `10..=20` → `10..20` (TESFurniture::IsSitMarker) | yes, only by the mined constant 20 |
  | float compare strictness | `>` → `>=` in a float min (only +0/-0 differ) | yes, by the cross-parameter ±0 pair |
  | compare in a bool result | `>` → `>=` in an epsilon compare | yes |
  | wrong offset/constant | member +8 → +0xc | yes |
  | rounding mode | round-half-to-even → half-down | yes, by the rounding ties |
  | argument order | `second / first` → `first / second` | yes |
  | signed vs unsigned | `u32` max compared as `i32` | yes |
  | shift kind | arithmetic `>> 5` → logical | yes |
  | sign extension | `y as u16` → `y as i32` in a packed key | yes |
  | mask width | `& 0xffffff00` → `& 0x0000ff00` | yes |
  | float precision | intermediate `float` rounding dropped | yes |
  | integer overflow | wrapping `i8` add → saturating | yes |
  | float→int overflow | NaN/range guard deleted | yes |
  | wrong callee | `_ftol2` (truncate) → the rounding helper | yes |
  | NaN quieting | `FLD`/`FSTP` sNaN quieting dropped from an `f32` result | **no**, see limits |

  The first two were missed until the constant boundaries, the cross-parameter
  combinations and the rounding ties were added, which is why those exist.
- **Determinism.** Two runs give the same counts and the same failures; only
  which out-of-range reads fault can vary with nv-call's process layout, and
  those are never failures.
- **Non-vacuity.** 108 of 324 functions vary with their inputs and match bit for
  bit, which cannot happen with arguments in the wrong places; the `Convention`
  check (stack bytes the exe actually popped) passes for every listed function.

- **Pointer-tier mutation test**, nine bugs planted in pointer code:

  | Genre | Planted bug | Caught |
  | --- | --- | --- |
  | wrong field offset | `this+4` → `this+5` | yes |
  | wrong field width | a `u32` flag field read as `u8` | yes |
  | lost sign extension | `as i8 as i32` → `as i32` on a field | yes |
  | lost sign extension in address math | signed byte offset added unsigned | yes |
  | wrong struct | `plane+12` → `point+12` | yes |
  | wrong flag bit | `& 0x20` → `& 0x10` | yes |
  | float field read as integer | `f32` → `u32` read | yes |
  | spurious write | an extra store into `this` | yes (`BufferMismatch`) |
  | swapped pointer comparisons | two `this->field == other` tests reordered | yes, after pointer values joined the field pools and lockstep fills were added |

- **Stray-read mutation test**, a number used as a pointer, which used to pass as
  `OutOfImageRead` (found in review, 00448a60):

  | Genre | Planted bug | Caught |
  | --- | --- | --- |
  | arguments swapped | `this->+0xC & mask` → `(mask)->+0xC & this` (00448a60) | yes (`RustStrayRead`, 79 of 79 vectors) |
  | scalar argument dereferenced | `(key >> 4) % 1001` → `(*key >> 4) % 1001` (0043c530, scalar tier) | yes (`RustStrayRead`, 64 of 64) |
  | field value dereferenced | `this->+0x78` → `*(this->+0x78)` (00407820) | yes (16 `RustStrayRead`, 1 `Mismatch`) |

- **Fault injection** for the safety paths, with a stand-in for nv-call (through
  `NV_CALL`) and a planted translation:

  | Fault | Result |
  | --- | --- |
  | nv-call crashes on one vector, hangs on another, is slow on two | `ExeCrash`, `ExeTimeout`, `ExeSlow` on exactly those vectors (the first two read as `DataFlowMiss` in pointer-tier functions); nv-call restarted twice; every other vector's verdict identical to a clean run |
  | nv-call reports another arena base / exits before its header / exits 0 with vectors left | the check stops with that message |
  | a translation loops forever on one vector | the watchdog stops the check after `--rust-timeout`, naming the vector and its inputs |
  | a translation allocates 256 MB (heap top reaches the arena) | the heap guard stops the check |
  | the list misses a pointer walked in a loop (the listing from before the merge fix, for 005f65d0, 00a089c0, 00c8d0d0) | `DataFlowMiss` on all three (exe access violations and an exe timeout), where they had passed as inconclusive |

## Known limits

- **sNaN quieting in `f32` results is invisible.** The uniform form returns
  floats as `Ret::st0: f64`, and `f32 as f64` quiets a signalling NaN on x86-64,
  so a translation that forgets the `FLD`/`FSTP` quieting reads back the same
  quiet NaN as the exe. A Rust caller of the typed function would still see the
  sNaN. Fix: carry the raw `f32` bits in `Ret`. (`f64` results are not
  affected.)
- **Only pure functions with scalar arguments or read-only pointer arguments.**
  Stores through arguments (out-parameters) are phase 2 of #193; dereferenced
  pointer fields need nested buffers; functions that touch globals need state.
- **Runtime-initialised globals are zero.** Neither nv-call nor the engine runs
  the CRT's static initialisers, so a function reading a table the game fills at
  start-up (the 512-entry sine tables) returns 0 on both sides; such matches are
  listed in `coverage.tsv`.
- **One FPU mode per run.** `check` runs with control word 0x027F (53-bit) unless
  `--fpcw` gives another (see "x87 precision" below).

## Running it

```powershell
# once: build nv-call (32-bit)
cd research\nv-oracle
cargo build --release --target i686-pc-windows-msvc

# purity facts from your project (see research/ghidra/README.md for the .cmd wrapper)
analyzeHeadless <proj> FalloutNV -process FalloutNV_unpacked.exe -noanalysis -readOnly `
  -scriptPath research\ghidra -postScript NvPurity.java out=<private>\purity.tsv

cd research\nv-oracle\nv-pure
cargo run --release -- list --purity <private>\purity.tsv
$env:NV_ORACLE_EXE = '<path to the unpacked FalloutNV.exe 1.4.0.525>'
cargo run --release -- check            # all listed functions
cargo run --release -- check --limit 50 # the pilot
cargo run --release -- check --only 00ec62c0,00a01a20
```

## x87 precision

The game's main thread runs at control word 0x007F (24-bit precision) after
Direct3D created the renderer's device; the engine models that per thread
(`crates/engine-core/src/x87.rs`, "Precision control").

- `check --fpcw 0x007F` runs both sides under that word: nv-call loads it before
  every call, and every engine gets it as its main thread's control word
  (`crt::set_x87_control_word`). Translations that compute through the x87
  helpers follow it; translations in plain `f64` stay at 53-bit and show up as
  mismatches, which is the list of what still has to move to the helpers.
- `x87 [--count N]` (default 2000 random vectors per case) checks the helpers
  themselves and helper-built models of five pure x87 functions (`00c5d300`,
  `004b3ab0`, `00646d00`, `00646d40`, `00647920`) against the CPU. The helpers
  run through one-instruction stubs (`FADD`/`FSUB`/`FMUL`/`FDIV` with a `double`
  or `float` memory operand after an `FLD qword` or `FLD dword`, `FSQRT`, and
  `FMUL` by 1 for `x87_round`), our own code that nv-call maps from a snapshot
  file at 0x20000000; inputs are every pair of an edge list (rounding ties,
  values just off a tie by sticky bits only, values beyond the float range,
  float denormals, infinities, NaNs) plus random floats, doubles and
  constructed near-ties. Helpers run at 0x027F, 0x007F, 0x047F, 0x087F and
  0x0C7F; the functions at 0x027F and 0x007F. Output in the out folder:
  `x87-failures.tsv`, `x87-vectors.jsonl`, `x87-results.jsonl`.

```powershell
cargo run --release -- x87
cargo run --release -- check --fpcw 0x007F
```

`NV_CALL` overrides the nv-call.exe path (default:
`research/nv-oracle/target/i686-pc-windows-msvc/release/nv-call.exe`),
`NV_ORACLE_OUT` the output folder. Exit status 1 when any vector fails.

The MSVC build of nv-call was run here on Windows 11 (i9-12900H) against the
unpacked 1.4.0.525 exe.
