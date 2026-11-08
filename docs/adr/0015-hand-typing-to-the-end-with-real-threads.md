# ADR-0015: Every system is typed by hand to 64-bit, with real threads built in

**Status:** Accepted. The maintainer decided points 1, 3 and 4 (2026-11-07); points 2, 5 and 6 are the lead's design under it, open to change. Amended 2026-11-08: guest memory stays plain (below).
**Date:** 2026-11-07
**Extends:** [ADR-0012](0012-modern-64-bit-engine.md), [ADR-0014](0014-typed-migration-in-two-stages.md)

## Context

ADR-0014 typed the hottest code first for speed and left open how every object leaves the 32-bit
space. Two routes were measured side by side on 2026-11-06 and 07:

- **Compressed pointers** (64-bit addresses in the code, 4-byte compressed words in the objects,
  applied mostly by a rewriting tool): after three rounds, about one pointer access in 37 still
  needed a hand fix against a go line of one in 300, and each round found new kinds of miss.
- **Hand typing**, family by family with differential tests: the first batch (25 lock-free map
  functions) took about a minute of active work per function, its tests caught every planted bug,
  and it found no translation bug.

A heap census showed that only about half of the live bytes sit in families touched by 300
functions or fewer: the game's scratch memory and its form records are touched far more widely. The
game's other threads, stepped one at a time on the main core for exactness, cost about 6-7 ms a
frame in Goodsprings.

## Decision

1. **Hand typing to the end.** Every system moves to typed, native 64-bit Rust by hand, family by
   family, with a differential test per function; the compressed-pointer route is dropped. Stage 2
   (objects out of the 32-bit space) follows per family as ADR-0014 describes; the platform's own
   bulk buffers (textures, vertex and index buffers, sound) move early, since no game code reads
   them in place.
2. **Order:** the hottest and thread-relevant families first (the code the game's other threads
   run, then the main thread's hot code), the form records last.
3. **Real threads now, with the migration.** The game's own threads run truly in parallel as soon
   as the shared machinery is safe: the game's locks and interlocked operations as real atomics, a
   thread-safe allocator, a per-thread engine context. More cores than the original used (per-actor
   AI, physics islands, animation, a render thread, background loading) follow on typed code.
4. **Real threads on by default after the race tests,** without waiting for all of the threads'
   code to be typed: a race campaign (shuffled schedules, long soaks) and comparisons with
   recordings of the original must pass first.
5. **Stepped threads stay** as a permanent switch, the reference mode for tests, recordings and
   traces, so exactness stays checkable. Runs with real threads are compared with the original by
   distributions over several recordings, since the original's own threads vary run to run.
6. **Races the original itself has** get fixes in the fixes preset, switchable like its other fixes.

## Guest memory stays plain (amendment, 2026-11-08)

The working plan's rule R1 (the plan is kept outside the repository) made every word load and
store of guest memory an atomic access, acquire for loads and release for stores, expecting it to
cost nothing on x86-64. It was built and measured with real threads: one BAAB set in Goodsprings,
real threads on both sides, atomic 51.9 and 50.7 fps against plain 55.3 and 53.8, about 6% slower.
The evidence is weak (one set, two runs a side, t about -3.5), but every atomic run was below every
plain one, and the plan's condition was no cost. So guest loads and stores stay plain moves through
raw pointers, a deliberate departure from R1 (`crates/engine-core/src/mem.rs`, "Threads").

**The hazard.** Two threads touching the same guest bytes without synchronisation, one writing, are
a data race: undefined behaviour in Rust's and LLVM's model, where the exe's races were defined by
the processor. The port relies on LLVM emitting each raw-pointer access as one move and moving none
across a call it cannot see through. Translated code is safe so: it reaches guest memory only
through such accesses, and the game's interlocked operations, critical sections, waits and sleeps
are opaque calls (real atomics and real waits with real threads). Typed code is where it can break:
a typed loop that waits on guest memory another thread writes, with no such call inside the loop,
can have its load hoisted out of the loop by the compiler and hang forever. Every typed loop that
waits on guest memory must make the platform call the exe makes there (the plan's R8;
docs/ENGINE_CRATE.md, "A risk for later"), and reviews check it.

**When to look again:** a hang traced to a hoisted guest access; more measurement sets that put the
cost within noise; ARM, where the orderings cost real instructions either way; or the stage-2
families, whose shared fields become atomic cells where the game uses interlocked operations or a
lock (the plan's R7).

## Consequences

- The maintainer's aim is one to two weeks. The pace is measured per batch and reported; the game
  stays playable throughout, and speed and threads arrive early with the families that carry them.
- The extender rule that guest memory is the only copy of game state is amended at the first
  stage-2 move of a family.
- The working plan, forecasts and first steps are kept with the measurements outside the
  repository; progress is recorded in docs/MILESTONES.md and docs/HANDOFF.md.
