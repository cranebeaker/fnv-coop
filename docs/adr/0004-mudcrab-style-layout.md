# ADR-0004: Mudcrab-style repository layout

- **Status:** Rejected 2026-10-06 by the maintainer: the current crates and docs layout stays. The text below is kept for the record.
- **Date:** 2026-10-05

## Context

[Mudcrab](https://github.com/Mudcrab-Team/mudcrab) reimplements Skyrim SE in
Rust and Bevy. It is easier for newcomers to read: one README with a crate
table and roadmap checklist, a `LEGAL.md`, a terse root `SPEC.md`, and docs
split into `specs/`, `roadmap/`, `adr/` and `research/`. It has five crates,
where nv-rs has sixteen plus the viewer.

Mudcrab's engine design differs: it converts game files ahead of time into
glTF, KTX2, SQLite and Luau, uses many external crates and Bevy 0.19, and adds
features the original game lacks. The maintainer chose to adopt only the
layout and documentation style.

## Decision

nv-rs keeps its 1:1 approach. It reads original game files at runtime, keeps
core crates free of external libraries, and pins the viewer to Bevy 0.16. Its
layout and docs follow Mudcrab's shape.

The open branch `codex/m1-reload-update-order` edits `docs/MILESTONES.md`,
`docs/OPENING_LOOK_IK.md`, `docs/PERSISTENCE.md`, `docs/VIGOR.md`,
`docs/FACE_CREATION.md` and `viewer/src/main.rs`. File moves therefore wait
until that branch has landed. Only additive files were added on 2026-10-05:
this ADR series, `LEGAL.md` and the new README.

### Target layout

```
nv-rs/
├── Cargo.toml              # core workspace (no external crates)
├── README.md  LEGAL.md  SPEC.md  CONTRIBUTING.md  AGENTS.md  CLAUDE.md
├── crates/
│   ├── formats/            # game file readers
│   ├── game/               # rules and simulation
│   ├── tools/              # nvinspect command-line tool
│   ├── test-content/       # synthetic fixtures for tests
│   └── engine/             # Bevy app; its own workspace, Bevy 0.16
├── research/               # ghidra scripts, nv-oracle (own workspace)
├── docs/
│   ├── roadmap/            # milestones M1-M6 and the active tracker
│   ├── specs/              # formats/, game/, engine/, meta/
│   ├── research/           # method and topic findings
│   ├── adr/                # decisions
│   └── contributing/       # workflow, maintaining, playtesting, AI policy
├── scripts/
└── distribution/
```

### Crate mapping

The dependency graph allows a clean layering: formats, then game, then tools
and engine. `bsa` uses `esm`'s zlib and text helpers, `assets` uses `bsa`,
and `nif` has no dependencies. `world` uses `esm`, `nif` and `script`.

| New crate | Old crates (become modules) | Lines (2026-10-05) |
| --- | --- | --- |
| `formats` | esm, bsa, nif, dds, mp3, speedtree, shaders, assets | about 26k |
| `game` | world, physics, script, ui, preview, cellview | about 107k |
| `tools` (binary `nvinspect`) | nvinspect | about 17k |
| `test-content` | testdata | about 8k |
| `engine` (binary `nv-viewer`) | viewer (separate workspace) | about 32k |

`game` would be one large crate. Before merging it, record `cargo build
--timings` and an incremental rebuild after a one-line edit in `world`. If
the merged crate's incremental rebuild is more than 25% slower, keep `game`
as a directory of sub-crates instead (`crates/game/{world,physics,...}`).
That keeps the same reading layout and the same build parallelism.

### Docs mapping

| Now | Moves to |
| --- | --- |
| `docs/MILESTONES.md` | `docs/roadmap/README.md` (tracker and gates) plus `docs/roadmap/01-opening-and-persistent-world.md` ... `06-vr.md` |
| `docs/TECHNICAL_REFERENCE.md` | split by topic into `docs/specs/{formats,game,engine}/*.md` |
| `docs/ENGINE_REFERENCE.md` | split by topic into `docs/specs/*` (facts) and `docs/research/*` (investigations) |
| `docs/CLAUDE_REFERENCE.md` | deleted; it duplicates ENGINE_REFERENCE apart from its title (update `CLAUDE.md`) |
| `docs/VR.md` | `docs/specs/meta/vr.md` |
| `docs/METHODOLOGY.md`, `docs/RESEARCH_WORKFLOW.md` | `docs/research/` |
| `docs/OPENING.md`, `OPENING_LOOK_IK.md`, `FACE_CREATION.md`, `PERSISTENCE.md`, `VIGOR.md` | `docs/research/` |
| `docs/MAINTAINING.md`, `PLAYTESTING.md`, `GOVERNANCE.md` | `docs/contributing/` |
| "AI assistance" in `CONTRIBUTING.md` | `docs/AI_POLICY.md` |
| (new) | `SPEC.md`: goal, constraints, interfaces (nvinspect commands, viewer flags), milestone gates, in Mudcrab's terse style, built from AGENTS.md and the roadmap |
| (new) | `docs/specs/README.md`, `docs/roadmap/README.md` indexes |

### Steps

Run the full checks from AGENTS.md after every step, then commit.

1. Land `codex/m1-reload-update-order`. Then merge `claude/fnv-rust-bevy-port-ofxk8h`.
   `docs/OPENING_LOOK_IK.md` will conflict, because both branches edit it.
   Afterwards run `cargo test -p nvinspect --test repo_hygiene` and rewrite
   any new Ghidra automatic names the Codex text added.
2. Docs moves with `git mv` (keeps history), one group at a time. Update
   every link, including references in code comments and
   `scripts/package-playtest.ps1`. Check with `rg -n "docs/[A-Z_]+\.md"`.
3. Write `SPEC.md` and the `docs/specs/README.md` and `docs/roadmap/README.md` indexes.
4. Merge `formats` first, one old crate per commit:
   - `git mv crates/esm/src crates/formats/src/esm`, then turn its `lib.rs`
     into `mod.rs`.
   - Move its tests to `crates/formats/tests/esm_*.rs` and its `testdata/`
     fixtures with them.
   - Replace `esm::` paths with `formats::esm::` across the workspace and the
     viewer, and update the `Cargo.toml` dependencies.
5. Measure, then merge `game` the same way, or make it a sub-crate
   directory (see above).
6. Rename `nvinspect` to `crates/tools` (binary name unchanged) and
   `testdata` to `crates/test-content`.
7. Move `viewer/` to `crates/engine/`, keeping it a separate workspace (root
   `exclude`). Update `.github/workflows/ci.yml`, the packaging script,
   README, CONTRIBUTING and AGENTS.md, then rebuild the playtest package once
   to confirm the script.
8. Update the repository hygiene test's scanned paths (`crates/`, `viewer/`,
   `docs/`) to the new layout, and update the README crate table.

## Consequences

- Newcomers see five crates, one README, and docs grouped by purpose.
- Every in-flight branch must rebase after steps 2 and 4-7, so do them in
  quick succession and announce them in the tracker.
- Module paths change (`world::` becomes `game::world::`); comments that
  cite paths need the same update.
