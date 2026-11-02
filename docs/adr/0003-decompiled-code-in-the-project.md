# ADR-0003: Decompiled code may be translated into the project, marked

- **Status:** Accepted (maintainer decision); changes the earlier rule
  "never copy decompiled code into the project"
- **Date:** 2026-10-05

## Context

Until now the project learned facts from the decompiled exe and wrote its
own code from them. Translating decompiled functions directly into Rust is
faster for logic-heavy code. The research found that it raises the risk of a
takedown (as with re3), and that LLM translation without an oracle is often
wrong. The maintainer chose speed and accepted the risk.

## Decision

Logic decompiled from the user's own `FalloutNV.exe` may be translated into
Rust in the project. Each translated function or block is marked with a
comment naming its source, for example `// Translated from 00c755e0
(decompiled, FalloutNV.exe 1.4.0.525).` A translation still needs a
regression test, and an oracle check where the function is eligible. Raw
Ghidra exports, project databases, executable bytes and recordings are
still not committed. Readable names replace Ghidra's automatic ones
(`FUN_...`, `DAT_...`, `param_N`); the repository hygiene test enforces
that in code and docs.

## Consequences

- Faster porting of pure and logic-heavy functions. The clean-room lanes in
  METHODOLOGY.md become optional.
- The MIT/Apache licence covers only original contributions, not material
  translated from the game's program; [LEGAL.md](../../LEGAL.md) says so.
- Marked code can be found with `rg "decompiled, FalloutNV"` and isolated or
  removed if ever required.
- This ADR supersedes METHODOLOGY.md's rules against translating decompiled
  functions and its requirement of OS-isolated clean-room lanes. Oracle
  checks, provenance and regression tests still apply.
