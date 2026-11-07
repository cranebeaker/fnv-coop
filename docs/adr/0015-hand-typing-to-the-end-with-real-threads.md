# ADR-0015: Every system is typed by hand to 64-bit, with real threads built in

**Status:** Accepted. The maintainer decided points 1, 3 and 4 (2026-11-07); points 2, 5 and 6 are the lead's design under it, open to change.
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

## Consequences

- The maintainer's aim is one to two weeks. The pace is measured per batch and reported; the game
  stays playable throughout, and speed and threads arrive early with the families that carry them.
- The extender rule that guest memory is the only copy of game state is amended at the first
  stage-2 move of a family.
- The working plan, forecasts and first steps are kept with the measurements outside the
  repository; progress is recorded in docs/MILESTONES.md and docs/HANDOFF.md.
