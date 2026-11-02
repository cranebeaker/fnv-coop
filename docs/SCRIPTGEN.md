# Script translation (`scriptgen`)

`crates/scriptgen` turns every script in a plugin (every `SCTX`: script
records, dialogue lines', quest stages', packages', terminals' and placed
references' result scripts) into a Rust crate. The generated code makes the
same `Host` calls, variable reads and writes, in the same order, with the
same `Flow`, as `script::interp` running the source text, including the
game's own quirks (evaluation order, values left on the stack, whole-number
truncation, a function the host can't work out stopping the run). It shares
the interpreter's primitives (`interp::apply`, `var`, `set`,
`call_function`), so engine behaviour is still implemented once, in the
`Host`.

The generated crate is built from the user's own plugin and must stay out of
this repository:

```powershell
cargo run --release -p scriptgen -- "<Data>\DeadMoney.esm" D:\Projects\nv-dlc\dead_money dead_money
cd D:\Projects\nv-dlc\dead_money
cargo test --release
```

## Evidence (2026-10-05)

- `DeadMoney.esm`: 1,298 scripts translated, 0 rejected by the parser
  (SCPT 347, INFO 679, QUST 107, TERM 113, PACK 34, REFR 18).
- `tests/parity.rs` in the generated crate: every script against the
  interpreter, 16 seeds x 4 rounds of every block kind and the body, with a
  recording host that answers from a seeded sequence and sometimes can't.
  All match. A deliberately broken comparison in one script is reported at
  its first host call.

## Not done

- Parity is with `script::interp`, not with the original game. Differences
  between the interpreter and FalloutNV.exe (e.g. the unconfirmed comparison
  precedence noted in `parser.rs`) carry over; resolve them in the
  interpreter, from the Ghidra decompilation, and regenerate.
- The world still runs scripts through the interpreter. Using the generated
  code needs a lookup in `world::scripting` (by plugin and local form ID).
- Engine functions Dead Money calls that the `Host` doesn't carry out are
  unchanged: `nvinspect <Data> functions` lists them.

Next action: list the functions Dead Money calls that `world::scripting`
doesn't implement, ordered by use.
