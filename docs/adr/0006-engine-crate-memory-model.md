# ADR-0006: Bulk translation goes into an engine crate on a model of the game's memory

- **Status:** Accepted by the lead session on 2026-10-09 under the
  maintainer's instruction to decide the method and continue overnight;
  open for the maintainer to confirm or overturn.
- **Date:** 2026-10-09

## Context

The maintainer's plan ([ENGINE_PORT_PLAN.md](../ENGINE_PORT_PLAN.md), "Fast
track") is to translate as much of FalloutNV.exe as possible in one week
with many agents working in parallel. Phase 0 counted 37,047 open game
functions and measured about 19k tokens per function; the maintainer's
budget (about 1.9 billion tokens a week) covers that.

The B1 method (Havok) translated one subsystem into idiomatic Rust: the lead
designed the structs and ownership, and the translations were wired into
`crates/physics` one by one. That does not scale to dozens of agents at once:
each would design its own partial structs and stand-ins for callees (the
Phase 0 trial agents did exactly that), and the pieces would not fit
together.

The game's code is C++ that passes raw pointers, reads fields at fixed
offsets, calls through vtables and keeps its state in globals and heap
objects. Its saves are those objects written out (Phase 2).

## Decision

Bulk translation goes into a new crate, `crates/engine`, built on a model of
the game's memory ([ENGINE_CRATE.md](../ENGINE_CRATE.md)):

- a sparse 32-bit address space in which objects live at their real
  offsets and pointers are the game's addresses;
- the data sections of the player's own installed executable, mapped at run
  time after a hash check, so constants, strings, vtables and initial
  globals are the game's (nothing from the executable is committed);
- every translated function registered under its exe address in one
  calling form, so translations call each other by address, directly or
  through vtables, before or after the callee is translated; a call to an
  untranslated function stops with its address;
- one file per source unit of the Xbox prototype, one owner per file;
- the allocator, threads and other `platform` code replaced by Rust and
  marked as such.

Existing crates are unchanged. Their systems switch to the engine crate's
versions as they are wired, top-down from the frame skeleton
([FRAME_SKELETON.md](../FRAME_SKELETON.md)), with adapters between the
engine's memory and the world crate's state, as B1 replaced its solver.

## Alternatives considered

- **Idiomatic structs per class, designed up front** (B1 at scale): best
  Rust, but every class's design is a lead-session decision; the lead would
  be the bottleneck for 37,000 functions.
- **Agents translate into the existing crates directly**: they would have
  to understand and change our own state model for every function, and
  most of the game's state has no counterpart there yet.
- **Running the original code** (emulating x86 or calling into the exe):
  not a reimplementation, and not 64-bit or VR-ready.

## Consequences

- Translations compose without coordination and can be tested in isolation
  (test doubles registered by address).
- The memory model is close to the binary: it makes save/load (change forms
  are these objects) and `nv-call` comparisons direct, but the code reads
  `e.get(p, Class::field)` rather than `p.field`. Idiomatic rewrites of
  hot or central parts can follow once they are translated and tested.
- Performance: every field access is a page lookup. Acceptable for game
  logic; measure before wiring per-frame hot paths, and port those to
  native structs if needed.
- The player's executable becomes a run-time input like the Data folder.
  The Steam file works (only `.text` is encrypted); other builds are
  refused by the hash check.
- The legal note in ENGINE_PORT_PLAN.md ("Legal exposure grows with
  volume") applies in full; every translation stays marked (ADR-0003).
