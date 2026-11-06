# ADR-0014: The typed 64-bit migration goes in two stages, per system

**Status:** Accepted. The maintainer decided points 3, 6's speed gates and 7 (2026-11-06); the rest is the lead's design under ADR-0012, open to change. The questions under "Not decided here" stay open.
**Date:** 2026-11-06
**Extends:** [ADR-0012](0012-modern-64-bit-engine.md) (point 2, the typed migration)

## Context

ADR-0012 moves the game's systems from the address-level translation to typed, native 64-bit
Rust, one by one, with objects moved to host memory behind 32-bit handles. A design pass on
2026-11-06 (the migration plan, kept with the measurements outside the repository; nothing here
was compiled or run for it) looked at what that means for speed and for the 4 GB limit:

- The speed lost to the translation's machinery (calls by address, argument words, modelled x87,
  stack frames in guest memory) is about 31% of the frame in Doc Mitchell's house. The memory
  check on a field read is at most 1.6%.
- Speed follows the hot functions: 90% of the calls go to about 200 functions. The limit follows
  bytes, and which objects hold them isn't known until a heap census is taken. They are different
  lists.
- Typing all 47,800 translated functions is far more work than the fnv_decomp audit was. The speed
  part is about 900 functions.

## Decision

1. **Two stages, per system.**
   - **Stage 1:** the code becomes typed and native; its objects stay in guest memory and are read
     through `Ptr<T>`, `Field` and `Mem`, with the page check kept. Nearly all the speed comes from
     this stage, and none of the 4 GB limit does.
   - **Stage 2:** a family's objects move to host memory behind 32-bit stubs, once nothing untyped
     reads their fields. This is what lifts the limit, and later allows real threads and new data
     layouts. A family moves only when it has a reason: bytes, threads or layout.
   - Stage 1 comes first, always.
2. **Each typed function stays callable by address.** It is registered at its exe address in the
   uniform form, so vtables, callback words and untyped callers keep working, and the translation
   stays linked as the fallback and the comparison target until the family is retired.
3. **One switch, bound before the first frame:** `NV_TYPED`, per family, read once at start-up. With
   a family off the game runs the translation. Past its gate a family is on by default, with the
   translation kept as a fallback for a while (maintainer, 2026-11-06).
4. **A diagnostic sees everything.** While any per-call diagnostic is on (a trace, a recording, a
   profile, a hook), typed calls go by address.
5. **An address a fix or a built-in can override is called by address**, in typed code always
   (`crates/engine-core/data/overridable.tsv`, enforced by the repository's hygiene test).
6. **Checking.** Every typed function has a differential test against its translation (return
   value, memory written, outgoing calls, both branches of a typed call in a family the test names, on generated inputs that include NaN, zero, denormal and
   huge floats). Above that, by what the family touches: the exe oracle, dual runs, the Direct3D
   draw comparison and the recording tables. Each step has a go or no-go gate. Speed gates are read in
   Goodsprings, where the baseline can resolve them (spread 0.75% over four runs on 2026-11-06), with
   Doc Mitchell's house (2.66%) still measured and reported as the named target (maintainer,
   2026-11-06).
7. **Floats** (maintainer, 2026-11-06). Code only the main thread runs: plain `f32`. Code on other
   threads, or both: `f64` locals, rounded at each `float` store. In SSE code: `f32` as now. Float to
   integer never goes through `as`. The exact x87 helper stays where a recording shows it matters, and
   says why. Sound keeps its x87 modelling and its gates (the standing bit-exact decision).
8. **Portable.** Typed code has no `core::arch` and no x86 assumption from the first step on.

The work order and gates are in the plan: the seam and a baseline first, then a census of who
touches what, then a pilot family with real handles (LockFreeMap, chosen because it can really
move and a mistake in it is loud), then the hot systems.

## Not decided here

These are open for the maintainer, and nothing in this record settles them:

- what "no 4 GB limit" means for the release, and when;
- whether rewriting call sites to typed leaf functions by script counts as the "generated direct
  calls" ADR-0012 skips;
- how the extender notes' rule that guest memory is the only copy of game state changes once a
  family has moved.

## Consequences

- The game stays playable and switchable at every step, and each family can be rolled back at
  start-up while its translation exists.
- Stage 1 alone does not remove the 4 GB limit, and the migration alone does not reach 200+ fps:
  the plan forecasts about 108 fps in Doc's house and 46 in town, from older uncapped figures; the 2026-11-06 baseline is already about 103 and 45, so the forecast needs redoing. Threads and the render thread have
  to follow; both get easier on typed code.
- Direct calls change how the thread host's time slice counts calls, so each batch that adds them
  checks the hand-offs per frame.
- The first step adds no cost: `Engine::call`'s fast path is untouched
  ([docs/ENGINE_CRATE.md](../ENGINE_CRATE.md), "Typed code").
