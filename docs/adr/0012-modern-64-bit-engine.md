# ADR-0012: Look and play exactly like the original; a modern 64-bit engine underneath

**Status:** Accepted (maintainer, 2026-11-04)
**Date:** 2026-11-04

## Context

The engine port translates FalloutNV.exe 1.4.0.525 function by function into Rust (ADR-0006). The
translated code runs inside a model of the 32-bit program:
- the game's objects live in one 4 GB guest address space (heap 0x20000000-0x7FFF0000);
- every guest load and store is checked against a page table;
- calls go through `Engine::call`;
- the x87 FPU is modelled.

That made exact matching and the verification tools possible (recordings, the oracle, the
bit-exact sound). It also kept the original's 4 GB limit and costs speed: busy views run at 36-48
fps where the target is 200+.

fnv_decomp (ADR-0009) now gives typed C++ for the whole exe, byte-matched to it.

The maintainer (2026-11-04): "I didn't even know we were basically just emulating the 32bit. I
want 64bit with all the benefits. The goal is to look and act exactly as the original, but
things that aren't seen or felt don't matter if they're different." Also: "I don't think we
should emulate x87 if that's slower and we can replace it with something modern that does the
same thing", and "a main reason is no more 4gb limit, more modern systems able to be
implemented".

## Decision

1. **The reference.** What players see and feel must match the original: visuals, gameplay
   results, physics feel, timing, sound. The original (the exe, fnv_decomp, recordings) is the
   reference every change is tested against, not the way the code has to be built. Internals may
   differ when a recording or comparison shows no observable difference: memory layout, pointer
   size, threads, allocators, data structures, the maths units.
2. **Typed 64-bit migration.** The game's systems move one by one from the address-level
   translation to typed, native 64-bit Rust, translated from fnv_decomp's typed C++.
   - Objects moved to host memory are seen by not-yet-typed code through 32-bit handles, as the
     platform objects at 0x7E000000+ already are.
   - An object type moves when every function touching its fields is typed.
   - Each step is checked against the current translation and the recordings.
   - A pilot first: one self-contained object family, to test the handle boundary and measure
     the speed gain.
3. **x87.** It is modelled only where a test shows the result matters; elsewhere native SSE
   maths is used.
4. **Bridge.** Until the migration removes the limit, the guest heap uses the whole 4 GB space,
   and bulky data moves to host memory.
5. **Renderer.** wgpu is the only backend.
6. **Modern systems.** These are built on the typed systems; gameplay-changing ones are
   switchable (docs/MILESTONES.md "Direction"):
   - no memory ceiling, and fast loading;
   - a job system on every core;
   - a fixed-rate simulation with interpolated rendering;
   - wgpu upgrades;
   - stability;
   - modding beyond 255 plugins;
   - more platforms.
7. **Order.** The platform crate split, then the fnv_decomp audit and crash fixes, then the
   migration, hottest systems first: that is the main speed work. The quick wins that outlive
   it (a render thread, PGO, the light-view wait) go alongside. Then the modern systems. Deep
   optimisation of the address-level translation is skipped where typed code replaces it.

## Consequences

- Speed, the 4 GB limit and modern systems are open, without giving up the original's look and
  play.
- Exactness becomes something tests prove, not a rule that the internals must mirror the exe.
- Rust's memory and thread safety cover the multithreaded systems, and the game stays playable
  throughout the migration.
- The address-level translation stays as the fallback and the comparison target until each
  system's typed version passes.
- Standing decisions stay until the maintainer changes them, e.g. the bit-exact sound.
