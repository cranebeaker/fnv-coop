# nv-rs: session instructions

Goal: Fallout: New Vegas running 1:1 in Rust from the user's own game installation:
FalloutNV.exe 1.4.0.525 translated function by function (the engine port), with a stable,
efficient runtime, mod support and an architecture that supports VR.

Read `docs/MILESTONES.md` first. Read only relevant source and reference
sections. For research work, `docs/METHODOLOGY.md` describes the method and
`research/*/README.md` the tools. `README.md` documents features and commands, `docs/README.md` indexes the docs.
The earlier instructions (`docs/ENGINE_REFERENCE.md`) were removed with the hand-written
layer on 2026-11-03 (ADR-0011; `docs/history/README.md`).

## Working rules

- Derive behavior and values from game files, compiled shaders, Ghidra or
  recordings. No stand-ins or invented behavior; leave unsupported features
  out. Label unresolved guesses in code and topic findings.
- Since 2026-10-05 (decisions in `docs/adr/`): names, types and layouts from
  the Xbox 360 prototype symbols may be used, marked `(Xbox PDB)` (ADR-0002),
  and decompiled logic may be translated into Rust, marked with its address
  (ADR-0003). This replaces the older reference-doc rule "never copy
  decompiled code into the project". Raw decompiler exports, databases and
  executable bytes are still never committed.
- Since 2026-11-02, with its authors' permission: the community's matching
  decompilation `momusdigital/fnv_decomp` may be used for names, types,
  layouts and logic translated into Rust (ADR-0009). Read it alongside FNV's
  Ghidra decompilation, not instead of it: the exe stays the reference.
  Settle every difference in the disassembly; mark what came from it
  (`, read against fnv_decomp` after a translation marker, `(fnv_decomp)` on a
  name or layout's first use in a file); never commit its C++; don't adopt
  its placeholder names (about half its functions still have names made from
  their addresses, in every folder). Its comments were written by machine:
  check them against the exe before using them.
- For behavior investigations, start with FNV's Ghidra decompilation, then
  inspect the associated game data and verify in the original game. The user
  explicitly prefers this order over trial-and-error changes.
- Keep provenance: executable address/build, asset/record or recording,
  branch conditions and a regression test for each behavioral fix.
- The game's code lives in the engine crates (`crates/engine*`, docs/ENGINE_CRATE.md);
  `crates/host` starts it and runs its frames; `viewer/` (`nv-viewer`) is the thin program
  around the host. There is no hand-written game layer any more (ADR-0011).
- Preserve the player/camera, aim/head, action-input and per-view boundaries
  in `docs/VR.md`. VR feature adaptations there are proposals.
- Never redistribute game files or modify the user's installation.
- Focus on the active milestone. Small fixes come first only for milestone
  blockers, crashes/data loss or explicit user requests. Keep other work in
  the deferred list.
- Distinguish implemented, tested and compared against the original game.
  Module existence and coverage counts do not prove playable completeness.
- Update the milestone tracker after each batch with evidence, blockers and
  one next action. Detailed research belongs in topic files, not this file.
- Claude and Codex share this file and the milestone tracker. Record active
  processes, changed files and unfinished checks in the linked topic handoff;
  do not maintain competing backlogs. Use the checked-out branch and GitHub
  milestone issues as the shared queue. Use small, bounded agents only when useful;
  use a cheaper capable model and keep one owner for each edited file.
- Explain outcomes and uncertainties in plain English.

## Machine and checking

Project: current checkout, Windows/PowerShell is the tested platform.
Game Data: the contributor's own installed Data folder; never commit it.
Reference place: Doc Mitchell's house (Save 138). GPU: RTX 5070 Ti Laptop.
Research/tools: `%USERPROFILE%\nv-re`, especially findings and decomp.
Read the archive's Ghidra/recording sections when needed.

Inspect Git status before edits. Use a branch and pull request;
do not claim commits, merges or remote synchronization without verification.

For code batches run appropriate checks, including:

```powershell
cargo test --workspace
cargo clippy --workspace --all-targets
cargo fmt --all -- --check
cargo build --profile play -p nv-viewer
```

Run the play build with `NV_D3D9=real NV_SOUND=out` and the Data path
(docs/PLAYTESTING.md); read its start-up line and the last line when it ends.
Tests build their inputs from scratch.

Play copy: `%USERPROFILE%\OneDrive\Desktop\nv-rs-play`. Reproduce what a player reports
in the original game and compare.
After merged code batches pass checks, the maintainer packages official builds
using `scripts/package-playtest.ps1`; see docs/MAINTAINING.md. A local play
copy may have its own publication helper outside the repository.
Documentation-only work does not require play publication.
