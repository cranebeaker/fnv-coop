# ADR-0011: The hand-written reimplementation is removed; nv-viewer runs the engine port only

**Status:** Accepted (maintainer, 2026-11-03)
**Date:** 2026-11-03

## Context

Until 2026-10-09 nv-rs reimplemented the game by hand: the `world` crate (scripts, AI, quests,
dialogue, combat), `physics`, `ui` (the menus), `cellview`, `preview`, `nif`, `dds`, `assets`,
`speedtree`, `shaders`, `script` and `scriptgen`, drawn by a Bevy 0.16 viewer in its own
workspace. ADR-0006 moved the work to translating FalloutNV.exe function by function into the
engine crates, and from Build 43 the game's own `WinMain` ran as the play build's main thread
with its own Direct3D 9 window (ADR-0008) and DirectSound output. With `NV_D3D9=real` the Bevy
window was hidden and drew nothing: of the viewer's ~100 systems the play build used two (the
frame calls into `crates/host`). Everything else, about 330,000 lines, only served the
hand-written layer, and contributor work on that layer (routes, systems, platform ports of the
Bevy viewer) could no longer reach the game that runs.

The maintainer (2026-11-03): "remove anything not used or needed and organize everything
cleanly"; remove the old world/viewer reimplementation; nv-viewer.exe stays as the engine port's
host; "remove those contributors work if its not needed since we are getting rid of the old
viewer".

## Decision

- Removed: the crates listed above and `testdata`; every Bevy module of `viewer/`; the
  `nvinspect` commands (they read data through the removed crates); `crates/host`'s quest
  bridges (`quest_vm`, `quest_form`, `global_form`, `script_form`) and their tests;
  `scripts/acceptance.ps1` (it replayed routes in the Bevy viewer), `scripts/dlc-data-pass.ps1`,
  `characters/`; the CI jobs for the Bevy viewer on Linux and macOS; the Bevy dependency
  notices; the docs that described the hand-written systems, their routes and contributor
  branches ([history/README.md](../history/README.md) lists them with the commit to read them at).
- Kept, rewired: `viewer/` is a thin program (`nv-viewer`) in the root workspace that finds the
  installation, starts `crates/host` and calls the game's frames until its `main` ends. The
  play profiles moved to the root `Cargo.toml`. `world::frame` (the frame model, Phase 1's gate
  and the source of `research/engine-map/frame_wiring.tsv`) became `crates/frame-model`, which
  the engine crates' tests compare their call orders with. `nvinspect` keeps only
  `tests/repo_hygiene.rs`. The plugin fixtures `testdata::mods` moved into `crates/esm/tests`.
- Kept as they were: the engine crates, `host`, `play-patch`, `esm`, `bsa`, `bink`, `mp3`,
  `vorbis`, `fos` (the original saves' format, Phase 2's comparator), `research/`, the
  measurement tools in `scripts/`, the ledger.

## Consequences

- One workspace, one CI job (Windows: fmt, tests, clippy). The play build is
  `cargo build --profile play -p nv-viewer`; the old flags launchers pass (`--walk`, a place
  name, ...) are accepted and ignored.
- Without `NV_D3D9=real` nothing is shown: the packaged launchers set it and `NV_SOUND=out`.
- There is no scripted acceptance route any more; a change is checked by a play-build run and
  against the original (recordings, the oracle). F12 reports went with the Bevy viewer.
- The removed code and docs stay in git history at `93922e5a`; the traced exe research in the
  docs is one `git show` away (history/README.md).
