# ADR-0013: C++ exceptions by behaviour, no frames in guest memory

**Status:** Accepted (maintainer, 2026-11-05)
**Date:** 2026-11-05

## Context

FalloutNV.exe 1.4.0.525 has 102 throw sites and 152 `throw;`, all through `_CxxThrowException`
(`00ec7894`) and all in STL, SpeedTree and FaceGen code; the game's own code never throws. It has
219 catch handlers in 153 functions: 67 handlers in 38 functions take the exception and carry on
(29 public `CSpeedTreeRT` functions with `catch(std::exception&)` and `catch(...)`; 9 FaceGen stream
functions with `catch(...)`), and 152 are `catch(...) { cleanup; throw; }`. Every function is built
`/EHs`, so `catch(...)` never takes a structured exception. Besides those, 6,355 functions have an
exception frame only to run destructors while unwinding, there are 74 SEH frames, and 7,026 `/GS`
cookie checks.

Until now the port stopped at a throw (`RaiseException` panicked), as in a Goodsprings exterior run
where `LoadTree` threw on a short read, which the exe's own catch (`00b057f2`) handles.

An older decision (2026-11-02) asked for all of it 1:1: frames in guest memory, the `FS:[0]` chain,
every unwind funclet, the cookies. The maintainer, 2026-11-05:

> "We should keep the behavior only, as long as it won't affect anything. Again we are fine with
> modern systems that do things better as long as it keeps the same look, sound, feel, etc of the
> original and doesn't affect mods script extenders etc."

## Decision

**A throw reaches the right catch and the function gives its normal result.** A throw is a Rust
unwind with a typed payload (the thrown object and its `ThrowInfo`), caught by `catch_unwind` in the
38 functions only, around the exe's try states (`crates/engine-core/src/cxx_exception.rs`).

Kept, from the exe:

- Which function catches, and what its handler and its continuation do: the message it reports, the
  object it deletes, the state it sets, the value it returns.
- `catch(std::exception&)` against `catch(...)`, matched on the throw's type chain read from its
  `ThrowInfo` in the exe's data.
- `throw;` from a catch block or a function it calls (`ios_base::clear`, `00ac9ff0`): it goes to the
  next open region, the object not destroyed until the catch that ends it.
- The thrown object where the thrower built it, alive through the catch block, destroyed by the
  `ThrowInfo`'s destructor when the block ends or a new throw leaves it (the exe's `__finally` at
  `00ece76b`); `throw;` passes it on. The catch block runs on the guest stack below the
  thrower's frames, as the exe's funclet runs on the machine stack above them.
- A throw nothing catches: `terminate` in the exe, a stop here, at the throw, naming the type and
  the message. Every one of the 38 ends in `catch(...)`, so whether any region is open on the OS
  thread is the exe's handler search; the regions are counted per OS thread because a Rust unwind
  stays on its own.
- A stop (any other panic) passes through a region untouched, as a structured exception passes
  `catch(...)` under `/EHs`.

On a catch the stacks a stop's catcher restores go back (the guest stack mark, `Engine::call_stack`,
and the trace's and profiler's copies). `resume_unwind` runs no panic hook, so a caught throw prints
nothing.

Dropped, and why:

- **The frames in guest memory, the `FS:[0]` chain, the state words, the handler stubs and the
  12,270 unwind funclets.** Nothing outside the exception machinery reads them, and a modern unwinder
  does the rest. They would add work to about 6,500 functions for a path that may never run.
- **The `catch(...)`-and-rethrow frames and the destructor-only frames.** A throw through them skips
  their destructors in guest memory: guest objects leak. See below for what else that could do.
- **`/GS`.** The 34 function bodies and 9 stub owners that call the check keep it; nothing is added
  and nothing removed. No cookie is placed next to a buffer.
- **SEH frames.** `SetThreadName` (`00aa2740`) and `00d87940` need `RaiseException(0x406D1388)` to
  come back, and it does, directly. `00a74a10`'s three `__except` blocks only cover a `CPUID` fault;
  the Rust asks the host's `CPUID`, which answers what the exe's probe would find on this CPU (mode 6
  on anything current). Neither needed a change.

## What the skipped cleanups affect

A throw path here is a thrower with a handling catch above it. Read from the exe's direct calls
(virtual calls not followed): 372 functions lie between a catcher and a thrower, 45 of them with
rethrowing catch blocks (65 blocks) and 113 with destructor-only frames, 474 distinct unwind actions.

| Throw path | What is skipped | Effect |
| --- | --- | --- |
| The parsers' `IdvFileError` (47 sites) up to `00b059e0` | Local strings, the error's text string, vectors of the part being parsed | Leak. The tree stays as far as it was parsed, as in the exe, where nothing rolls back either |
| `LoadTree`'s three `std::exception`s, to its own catch | Nothing: its message string goes just before the throw, its catch blocks delete the file | None. Its buffer leaks as in the exe, whose catch blocks forget it without freeing |
| `Random`/`PosGen`, `CFrondEngine::Compute`, up to `Compute` or the constructor | Local vectors and strings | Leak |
| STL `bad_alloc`, `length_error`, `out_of_range`, from any of the above | Vector and string temporaries; `_Insert_n`'s new block | Leak. Needs a count that overflows 32 bits or a string past its limit |
| The constructor `00b02ef0`'s member constructors, if one throws | The block allocated for that member (its unwind states 1 to 9) | Leak |

Every unwind action on those paths and every rethrowing catch block was read. They run string and
vector destructors, element-range destructors and frees, with five kinds of exception:

| Cleanup | Where | Could it change behaviour after a catch? |
| --- | --- | --- |
| A stream sentry's unlock (`00aca9a0`, `00aca780`, `00ad02f0`) | `00ac9df0`, `00aca560`, `00aca7a0`, `00ad43a0`, `00ad47f0`, `00ad4ba0`, `00ad5770` | No: a sentry is given back on the normal path after their own catch. Only a throw leaving those functions skips it, and nothing above them catches, so that is a stop |
| `_Lockit` in `use_facet` (`00ad38d0`, `00ad3ca0`) on `bad_cast` | Under `00ace390` and `00ad43a0`, outside their try states | No: it leaves them with no catch above, so it is a stop |
| The locale's reference (`00acb720`) | The same two frames | No: the same |
| The access object of `00b059e0` | Its own frame, inside its try | **Translated at the catch** (`00f23c20`): it holds the whole file's bytes |
| A string reset to empty (`0044aa10`'s catch) and a vector being built tidied (`00b200f0` and others) | STL growth and copies | Only if the object is used afterward. The string's path needs `_Allocate<char>` (`0065fa30`) to throw, which its test makes impossible; the vectors are new objects the rethrow leaves unreachable. Not proven for every caller |

Held locks with no cleanup in the exe stay held in the port too: the critical section `011f8bc4` in
`00b02ef0` and `00b08ec0` is inside their try states with no unwind action, and neither catch block
leaves it.

The stream exception mask, which makes `ios_base::clear` throw or rethrow, is written by nothing in
the exe but `ios_base::_Init`, which writes 0 (all 15 callers of `clear` read). So the FaceGen catch
blocks always swallow in the exe; the rethrow is translated and tested with a mask set by hand.

## Cost

Nothing outside the 38 functions changes. In them a region costs about 0.2 to 1.3 ns per entry
(`cxx_exception::tests::try_region_speed`, release build, 2026-11-05, against a 5 ns `Engine::call`):
a `catch_unwind` with nothing to do and a thread-local count. The busiest are per-frame or per-tree:
`SetCamera` and `SetTime` (a run of 2026-10-30 counted 15,201 and 4,001 calls), `ComputeLodLevel` and
`GetGeometry`. `Compute` runs once per tree. A throw costs one Rust unwind.

## What would show this wrong

- A recording of the real game (`exception:` lines in `rec.log`, research/nv-trace) with a throw the
  port does not make, or the reverse. Which throws fire in vanilla play is not yet known. The port's
  side is printed: every catch counts, the first 64 go to stderr (`C++ exception <type> "<text>"
  caught by <function>, thrown in <call stack>`, read from guest memory with no guest call) and the
  host's summary line says how many. A catch there that the recording does not have is a port bug:
  `gsext`'s "only read 0 of 0x1af7" in `LoadTree` used to stop, and now gives a tree deleted at
  `0066ad8f`..`0066af29` with only that line to say why.
- A catch that fires in play and is then followed by a difference from the original traced to a
  skipped cleanup: then that one cleanup is translated at its catch, as `00b059e0`'s was.
- A guest heap that fills up because a throw repeats: the leaks above would have to be closed.
- A plugin or mod relying on a frame, the `FS:[0]` chain or a cookie. The port never had them, and a
  translated plugin's `_CxxThrowException` works through `00ec7894`; its own try blocks would use
  `try_catch`. A plugin DLL throwing through translated frames has no way in yet.

## Consequences

- A bad tree file no longer stops the game: `LoadTree` reports through `SetError` and returns 0.
- The 38 functions lose their `Translation incomplete` markers. The other framed functions keep
  theirs: the frame is not built, by this decision.
- Port bugs that make a throw (a short read that the original does not have, say) now go into the
  failure result instead of stopping. The first thing to check on a missing tree is the
  `C++ exception` lines on stderr and the recording.
