# ADR-0001: Reimplement from the game's own program, data and recordings, checked by oracles

- **Status:** Accepted
- **Date:** 2026-10-05

## Context

The goal is a 1:1 Fallout: New Vegas in Rust and Bevy. Research into about
35 comparable projects ([METHODOLOGY.md](../METHODOLOGY.md)) found that
matching decompilation would take 13-22 team-years for FNV's roughly 42,000
first-party functions and would produce C++, not Rust. Clean-room engines
built from data alone took 9-18+ years. Projects that had both a naming
accelerator and a mechanical check against the original reached a playable
state in months to about two years.

## Decision

Behaviour is taken from `FalloutNV.exe` (read in Ghidra), the game's data and
recordings of the original game, and implemented in Rust. "1:1" means the
same outputs, constants, branch conditions and operation order, checked by
oracles built from the original game: function-level vectors on the real CPU
(`research/nv-oracle` `nv-call`), live hooks in the running game (`nv-probe`),
WinDbg TTD, apitrace and `scof` console logs. Work is scoped to what the
active milestone's route actually executes.

## Consequences

- Names come first: the exe's own RTTI and command, setting and form tables,
  community PC addresses, and the Xbox 360 prototype symbols (ADR-0002).
- Exports must be complete by construction (`research/ghidra`), not parsed
  from C text.
- A behavioural fix carries provenance (exe build and address, record, or
  recording) and a regression test at a declared CPU-fidelity tier.
- Matching decompilation, static recompilation and unverified LLM
  transliteration are rejected as primary methods.
