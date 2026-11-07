# Maintaining the official project

slaterain controls the official main branch and releases. External contributors
use forks and pull requests; no collaborator invitations are needed.

## Accepting changes

Main requires a pull request, code-owner approval, resolved conversations and
the CI job `check` (fmt, tests and clippy on Windows). New pushes dismiss old
approvals. Force pushes and deletion are blocked. CODEOWNERS includes workflows.
The owner has a PR-only ruleset bypass for their own changes (GitHub cannot
accept a self-approval); use it deliberately, after checks pass. No other
contributors receive bypass rights. Changing repository settings remains an
owner responsibility.

Keep forks' CI unprivileged: use pull_request, read-only token permissions,
GitHub-hosted runners and no secrets. Never execute contributed code in a
privileged pull_request_target workflow. Inspect workflow changes carefully.

## Releasing a Windows preview

An interim, labelled preview before the wgpu release (approved by the maintainer). Only the maintainer
decides on a release: agents build and test locally, and tag, upload and post nothing.

1. Merge a checked source commit and compare a play-build run against the
   original game. Record outstanding limitations, not just screenshots.
2. From a clean Windows checkout, run
   `./scripts/package-playtest.ps1 -Version YYYY-MM-DD.N -Game '<your FalloutNV.exe>'`.
   It builds with the lockfile, a static C runtime and no build paths in the programs, stages only
   the list below, runs `scripts/package-check` over it and makes the ZIP and its `.sha256`. The 32-bit
   tools need a toolchain with `i686-pc-windows-msvc` (`-Toolchain32`, default `stable`). `-Game` is required: the
   check that none of the game's bytes are in the programs fails the package. The script refuses a
   release ZIP unless HEAD is on origin (BUILD.txt links to it); `-TestBuild` makes a test package
   instead, marked "test build, not for release" in BUILD.txt and in its name.
3. Test the extracted ZIP from a fresh folder, a path with spaces and another drive: first start,
   Save or New Game, a couple of minutes in the world, no game folder, a wrong exe, tables deleted
   (`docs/PREVIEW_TESTERS.md` is the checklist for other PCs). Verify SHA256 and BUILD.txt.
4. Create a release tag at that exact source commit, which must be public: BUILD.txt points at it, and
   that is how the GPL's source is offered. Create a draft pre-release, attach the ZIP and its
   .sha256 file, and describe verified behavior, known issues, controls, required owned game data and
   reporting instructions. Say which PCs it has run on.
5. Publish after reviewing the draft. Official builds are maintainer-published;
   ordinary pushes and contributor PRs never publish a release automatically.

What the ZIP holds, and why each is allowed:

| File | What | Why it may ship |
| --- | --- | --- |
| `Play.exe` | The launcher (`scripts/launcher`) | Our code, GPL |
| `app\FalloutNV.exe` | `nv-viewer` under that name | Our build. LEGAL.md covers the translated code; `package-check` looks for data of the game's own program in it, and its report goes with the release notes |
| `app\tools\d3d9-capture.exe`, `nvapi-capture.exe`, `dsound-capture.exe` | The table tools (`scripts/`) | Our code, GPL. They read the player's Windows and drivers, on the player's PC |
| `Play with mods.cmd`, `Import my saves.cmd`, `Settings.cmd`, `Settings.ps1` | Launchers and the settings tool | Our text |
| `READ ME.txt`, `mods\READ ME.txt`, `BUILD.txt` | For players; the exact commit | Our text |
| `LICENSE`, `LEGAL.md`, `THIRD-PARTY-NOTICES.txt`, `licenses\` | GPL, the MIT notice, the independence statement, the Rust crates' licences, libvorbis's BSD notice | Required by those licences and LEGAL.md |

Never in it: game files, saves, reports, machine tables (`.tsv`), `.ini`, DLLs, debug files (the
script's type list refuses them), anything from fnv_decomp, or this PC's paths and user name
(`package-check`'s needles). Windows' and the graphics driver's DLLs stay on the player's own PC.

An unsigned exe: SmartScreen warns on a downloaded one ("More info", "Run anyway"), Smart App
Control blocks it with no way through, and an antivirus may hold or remove it, more so as it starts other
programs. The READ ME says so plainly. A code-signing certificate is the only real answer; none is planned yet.

Dependency updates require refreshing distribution/licenses and
THIRD-PARTY-NOTICES.txt before packaging. Keep their upstream provenance and
copyright notices. Packaging a license list alone is insufficient.

## Work queue and AI handoff

Keep a small set of milestone issues. Contributors comment to claim a bounded
task before starting; assignment is not automatic acceptance of a design.
Each handoff identifies branch/commit, changed files, verified evidence, test
results, unresolved questions and the next action. Do not paste entire chats.

The initial private upload history was retained separately because it included
build outputs. The clean public history starts with the reconciled source;
do not publish private research or old build directories to restore that history.
