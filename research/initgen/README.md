# initgen: the static-initializer table

Before `WinMain`, FalloutNV.exe's `__cinit` (`00eccd14`) runs two tables of
function pointers: `_initterm_e(010113c4, 010113e8)` (8 C initializers) and
`_initterm(00fdf4a4, 010113c0)` (17,728 non-null C++ dynamic initializers).
They build thousands of game globals: the `Setting`s with their names and
defaults, the NiAnimation function tables, computed float constants, RTTI
records. The engine crate runs them at start-up
(`crates/engine/src/static_init.rs`, called by `host::Host::start`).

Most entries are a few instructions of one of three shapes. Instead of one
Rust function per entry, initgen writes one row per entry into
`crates/engine/data/static_initializers.tsv`, and the engine has one
translation per shape that executes the rows in `__cinit`'s order. The
ledger counts each row as the translation of its entry.

```text
cargo run --release -- --exe "<...>/FalloutNV.exe" --cache <private dir> \
    --out ../../crates/engine/data/static_initializers.tsv --report <refused.tsv>
```

- The entries come from the executable's own tables (`--exe`, read-only).
- Each entry's instructions come from the read-only Ghidra server
  (`disassemble_function`, default `http://127.0.0.1:8090`). For the ~1,400
  entries Ghidra made no function of, the entry's bytes (`read_memory`, the
  unpacked image) are decoded by `decode`, which knows only the encodings the
  shapes use and refuses any other byte. Both are cached in `--cache`;
  disassembly and bytes are never committed.
- An entry becomes a row only if its whole body matches one shape,
  instruction by instruction. Everything else is written to `--report` with
  the reason and stays for hand translation.

## Shapes and rows

Each row has three tab-separated columns: `entry`, `shape` and `parameters`.
Addresses are 8 hex digits. Values from the executable's data (defaults,
constants) are given as the addresses they are loaded from, so the table
holds no bytes of the executable.

| Shape | Code (between `PUSH EBP; MOV EBP,ESP` and `POP EBP; RET`, or frameless up to `RET`) | Parameters |
| --- | --- | --- |
| `reg` | `CALL f; MOV [g],EAX` | `f g` |
| `object` | pushed arguments; `MOV ECX,g; CALL ctor`; optionally `PUSH dtor; CALL _atexit; ADD ESP,4` (or `POP ECX`) | `ctor g dtor\|- args...` in the constructor's order: `w:<word>`, `f:<src>` (a float pushed through `PUSH ECX; <load>; FSTP float ptr [ESP]`), `d:<src>` (a double through `SUB ESP,8; <load>; FSTP double ptr [ESP]`); `<src>` is `s.<addr>` (`FLD float`), `d.<addr>` (`FLD double`), `1` (`FLD1`) or `0` (`FLDZ`) |
| `const` | straight-line x87 and moves, the x87 stack empty at both ends | `;`-separated ops: `fld.s/fld.d <a>`, `fld1`, `fldz`, `fchs`, `fmul/fdiv/fdivr/fadd/fsub/fsubr.s/.d <a>`, `fmulp/fdivp/fdivrp/faddp/fsubp/fsubrp`, `fstp.s/fstp.d <g>`, `sqrt <a>` (`PUSH ECX; FLD float ptr [a]; FSTP float ptr [ESP]; CALL 004019b0; ADD ESP,4`), `call <f>` (no arguments), `st.eax <g>` (after a call), `st.w/st.b <g> <imm>`, `cp.d <g> <a>` (`MOV reg,[a]; MOV [g],reg`) |

Not shapes: a call to `00a2e180` (it answers the incoming `ECX`, which
`_initterm` leaves from the previous initializer), anything with a branch,
stack frame locals or another instruction.

`--rust <addr,...>` prints Rust translations of the NiAnimation registration
functions the `reg` rows call and of their indexed table helpers, each
matched against its exact form (used once to write the functions in
`crates/engine/src/units/nianimation/run_*.rs` and
`units/unplaced/run_00a29470.rs`).

## Checks

`cargo test` checks each shape's matcher and the byte decoder; the engine's
`static_init` tests check the parser and each shape's translation (x87
precision, NaN rules, argument order); `crates/host/tests/static_initializers.rs`
(opt-in, `NV_EXE`) runs the whole table on the real executable and checks
settings against their initializers' values.

## The shut-down table (`--atexit`)

The counterpart of the start-up table. The engine map lists 5,020 functions
of the unit "compiler: atexit destructors" (`00fc7a70` onwards): the small
functions the static initializers hand to `_atexit` (`00ec658f`), which the
CRT runs when the program exits. `--atexit` reads their list from the engine
map and each one's code from the same read-only Ghidra server (and cache),
and writes `crates/engine/data/atexit_destructors.tsv`:

```text
cargo run --release -- --atexit --map ../engine-map/engine_map.tsv --cache <private dir> \
    --by-hand 00fd9bf0 \
    --out ../../crates/engine/data/atexit_destructors.tsv --report <refused.tsv>
```

Rows are `entry`, `shape`, `global destructor` (addresses). A function becomes
a row only if its whole body is one of these two shapes, instruction by
instruction, and its size in the engine map is the shape's size (so no other
byte exists); anything else goes to `--report` (`entry`, size, the reason; a
refused body is named by its instructions with every number replaced by `#`).
`--by-hand <addr,...>` leaves out functions that have a hand translation (the
hand translation is the translation; `00fd9bf0`).

| Shape | Code | Bytes | Rows |
| --- | --- | ---: | ---: |
| `call` | `PUSH EBP; MOV EBP,ESP; MOV ECX,global; CALL destructor; POP EBP; RET` | 15 | 4,698 |
| `tail` | `MOV ECX,global; JMP destructor` | 10 | 92 |

The two shapes have the same effect; they stay separate rows because the
code differs, and the engine checks each translation against its own code
(`crates/engine/src/static_exit.rs`). Both are faithful for the same reasons:
`ECX` is the global and the destructor is `__thiscall`, so the global is its
`this`; the frame of `call` restores `EBP` and `ESP` and its closing `POP EBP;
RET` touch neither `EAX`, `EDX` nor `ST0`, and `tail` jumps so that the
destructor's own `RET` is the shape's; so the function's answer is the
destructor's. initgen also checks that the destructor takes nothing from the
stack: every `RET` in its code (following direct tail jumps, at most four) is
a plain `RET`, never `RET n`. All 84 distinct destructors passed.

Everything else (229 functions) is refused and left for hand translation. By
shape, with the counts: 72 are 117-byte bodies with an `FS:[0]` exception
frame, a virtual call and a destructor call; 54 load a global object pointer, branch on it, call an
import on a field of it and end in a virtual tail call; 21 `PUSH` four words, `CALL` and `RET`; 12
`PUSH` a word, `CALL dword ptr [import]`, `RET`; 10 and 9 the same two in a
frame; 7 store a constant to a global; the rest are single cases. They are
regular enough to look alike but each has a branch, a frame handler, or an
imported callee whose argument count needs reading from its `RET n`; none was
made a shape.

The ledger counts each row as the translation of its function (status
`static init (table)`), as for the start-up table.
