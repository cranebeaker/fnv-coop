# ADR-0010: The game's own units are split by area into twelve crates

- **Status:** Proposed by the lead session, 2026-11-03 (the maintainer asked that checking a fix take minutes); for maintainer review with the pull request
- **Date:** 2026-11-03

## Context

After the October split ([ENGINE_CRATE.md](../ENGINE_CRATE.md), "The engine crates"),
`engine-fallout` still held the game's own units: 1,775 files, 2.1 million lines. Any fix in
one of them recompiled all of it: a play build after a one-file fix took 134-469 s (median
419 s, one crate at a time as the build queue runs it, with other agents' builds beside it;
178 s in 16 such builds of 2026-10-23 to 10-29) with one rustc at 8.6 GB, and the unit
tests of the fixed unit 525 s with one rustc at 23.4 GB, so the build queue ran it alone and
agents tested `engine-fallout` units in stand-in crates. The maintainer
(2026-11-03): checking a fix should not take more than a few minutes.

Translations reach each other almost only by exe address through `Engine` (25,341 literal
address calls in non-test code against 46 Rust paths between folders), so the crate can be
cut along areas without changing the code.

## Decision

- `engine-fallout` becomes twelve crates by area: `engine-fallout-facegen`, `-script`,
  `-pathfinding`, `-env`, `-interface`, `-cell`, `-forms`, `-magic`, `-ai`, `-misc`,
  `-unplaced-a`, `-unplaced-b` (what each holds and needs: ENGINE_CRATE.md, "The fallout
  crates"). There is no new base crate: `engine-core` is the base.
- Placement is by unit, not by folder: `scripts/ledger/fallout_crates.tsv` (folder pattern,
  unit pattern, crate; first match wins; part files follow their unit). `fallout_shared` is
  split between cell, forms and script, `unplaced` between nine crates (runs whose layouts a
  crate uses go with it). `ledger units` refuses a file in the wrong crate; `ledger scaffold`
  places new units with the rules.
- Paths do not change. Each crate's generated `units` modules re-export, by name, the units of
  a split folder that live in the crates below, so `super::<unit>`,
  `crate::units::<folder>::<unit>` and `engine::units::<folder>::<unit>` keep resolving and
  no translation had to change (five items became `pub`, one helper moved to
  `engine_core::x87`; FaceGen's speech check, diagnostic code, got a redirect so that its
  tests see one state: its five hooks called from other crates first read a `OnceLock` that
  only those tests set).
- Registration is unchanged: each crate's `units::funcs`, the facade's `assemble!` over all of
  them; a facade test checks that no address is registered by two crates.
- Tests, phase A: each fallout crate names every other fallout crate under
  `[dev-dependencies]` and registers them in its `cfg(test)` assemble, so every test sees
  every translated function as before the split. Phase B (later, per crate) drops the ones a
  crate's tests do not need, doubling a callee in the test or moving an integration test to
  the facade.
- Every carve commit moves its files as exact renames (`git mv`; the five unit files that
  had to change are edited in commits of their own), so branches from before the split rebase
  with git's rename detection; a branch that adds unit files replays its patches with the
  rename map.

## Consequences

- A one-file fix recompiles its own area and re-checks the crates above it (facegen seven,
  cell four, forms three, pathfinding two, ai and interface one, the others none); the
  measured times and memory are in ENGINE_CRATE.md and the pull request: the same five
  one-file fixes rebuild the play build in 18-52 s (median 30 s) with no rustc above 2.8 GB,
  and the unit tests of the fixed `fallout_ai` unit take 132 s at 5.1 GB.
- In phase A a crate's test build still compiles the other fallout crates as libraries (once,
  then only those above an edited crate); phase B removes that.
- Every crate keeps the same crate-level lints and the same `crate::` paths to the core, so a
  translation moves between crates by `git mv` and a row of `fallout_crates.tsv`.
- Workspace clippy and the full test suite still run on CI (windows-latest); the laptop runs
  per-crate checks.
