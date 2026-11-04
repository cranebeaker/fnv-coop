# Contributing to nv-rs

nv-rs is maintained by **slaterain**. Many people now work on it at once,
most of them with AI agents. This page says how to do that so that your
work can actually be merged. Read it, then give it and
[AGENTS.md](AGENTS.md) to your agent.

The short version:

1. Work from the latest `main`, in a fork, on a topic branch.
2. Open an issue saying what you want to do, and wait for the maintainer's
   go-ahead before you start.
3. Trace every behaviour in the original game. No invented behaviour.
4. Open a small pull request against `main` that passes the checks.

## Where help is needed now

The whole game runs on its own translated code: every function of `FalloutNV.exe`
([docs/LEDGER.md](docs/LEDGER.md)) is translated into Rust in the engine crates
(`crates/engine*`), marked with its address, and `nv-viewer` runs it. The open world,
quests, DLCs and mods play, but translation errors still cause bugs and crashes
everywhere. The plan ([ADR-0012](docs/adr/0012-modern-64-bit-engine.md),
[docs/MILESTONES.md](docs/MILESTONES.md)): the game looks and plays exactly like the
original, while everything players don't see is rebuilt as a modern 64-bit engine.
First an audit of the translation against
[fnv_decomp](https://github.com/momusdigital/fnv_decomp), then the move of the game's
systems to typed, native 64-bit Rust, then the modern systems.

Where help is useful, in order:

1. **Play and report bugs.** This is the most useful help right now. Play the build
   ([docs/PLAYTESTING.md](docs/PLAYTESTING.md)) anywhere in the game and open an issue
   for each crash or difference from the original: where you were, what you did,
   what the original does, and the log's last lines. When the game's code stops, the
   log names the place (`Game's main stopped in frame N: ... (in <addresses>)`); those
   addresses are what we need.
2. **Compare translations with fnv_decomp or the executable.** Pick a function from a
   crash report or the ledger, compare its Rust translation with fnv_decomp's C++ for
   the same address (or run it on the real CPU with `nv-call`,
   [`research/nv-oracle`](research/nv-oracle/)), and report a mismatch as an issue with
   the address and what differs. Fix pull requests for a single function are welcome
   after the issue (see Provenance below); never commit fnv_decomp's C++.
3. **Compare with the original game.** Recording the original and comparing it with
   nv-rs automatically is the most valuable check the port can have. Propose a plan in
   an issue before building tools for it.
4. **Platforms.** Other platforms come through wgpu (the only planned renderer
   backend) once the platform layer is in its own crate. Discuss in an issue first.

Not open for contributions, because the maintainer's sessions are doing them and
parallel versions would conflict: the audit's system-wide passes, the move of systems to
typed 64-bit Rust, the platform layer and the play build. The hand-written
reimplementation that came before the port (the `world` crate and the Bevy viewer) was
removed on 2026-11-03 ([ADR-0011](docs/adr/0011-old-layer-removed.md)).

## Updating an older fork

`main` moves quickly. If your fork is more than a few days old, do this
before anything else:

1. **Update your fork.** On GitHub press **Sync fork** on your fork's
   `main`, or:

   ```sh
   git remote add upstream https://github.com/slaterain/nv-rs.git   # once
   git fetch upstream
   git switch main
   git reset --hard upstream/main      # your fork's main = upstream main
   git push --force-with-lease origin main
   ```

   Keep your own work on topic branches, never on your fork's `main`.
2. **Don't build on copied `claude/*` or `codex/*` branches.** They are
   the maintainer's working branches; pull requests built on them will be
   closed.
3. **Check that your work isn't already in `main`**: search the code,
   including the engine crates (`crates/engine*`) for the exe address you
   traced.
4. **Claim it.** Find the matching issue, or open one, and
   comment `Claiming this` with your branch name, even if you started
   before claiming. If someone else already claimed it, say what you have
   on the issue and agree who continues; don't open a competing PR.
5. **Rebase your branch on the new `main`**
   (`git rebase upstream/main`), resolve conflicts in favour of traced
   behaviour, and run the checks below.
6. **Open a pull request against `slaterain/nv-rs` `main`**, one system
   per pull request (a draft is fine while it's unfinished). Big work goes
   in as a stack of smaller pull requests, each reviewable on its own.

Work that changes how the game behaves without tracing it (tuned
constants, "feels better" camera or AI changes, procedural effects the
game doesn't have) can't be merged, however good it looks. Leave it out, and say in
the pull request which parts are traced.

## Claiming a task

There is no open task list right now. To work on something, open an
issue describing what you want to do and wait for the maintainer's go-ahead
before starting; ongoing claims are tracked on their issues.

1. Once the maintainer agrees, comment `Claiming this` on the issue with
   the branch name you'll use. First claim wins; don't start on work
   someone else has claimed.
2. Post a short progress comment at least once a day. A claim with no
   comment or pull request for 48 hours can be released.
3. Link the pull request to the issue (`Closes #N`). If you give up, say
   so on the issue so someone else can take it.

Found a new bug? Open an issue for it; don't fix it inside an unrelated
pull request.

## Areas

| Area | Owner | Notes |
| --- | --- | --- |
| Engine port: the audit's system-wide passes, the typed 64-bit migration, the platform layer, the play build | maintainer | ADR-0012, ENGINE_PORT_PLAN, ENGINE_CRATE, FRAME_SKELETON. Not open for contributions. |
| Bug reports from play | open | One issue per bug, with the log's last lines. |
| Single-function fixes found by comparing with fnv_decomp or the executable | open | Issue first, then a small pull request with a regression test. |
| Other platforms (wgpu) | later | Discuss in an issue first. |

For anything else, open an issue first.

## What every pull request must show

1. **Checks.** From the root (Rust as pinned in `rust-toolchain.toml`): `cargo test --workspace`,
   `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo fmt --all -- --check`. GitHub runs the same checks (`check`) and they must
   pass. `cargo test` includes a repository hygiene check
   (`crates/nvinspect/tests/repo_hygiene.rs`) that rejects game or binary files, absolute
   user-profile paths and Ghidra's automatic names (`FUN_…`, `DAT_…`): write the address
   instead.
2. **A play-build run** for a change to the game's behaviour: the play build
   ([docs/PLAYTESTING.md](docs/PLAYTESTING.md)) from the save or start you name, what you
   did, and the log's last lines. Say whether it ran the same on `main`.
3. **Provenance.** Every behaviour comes from the original game:
   translated from `FalloutNV.exe` 1.4.0.525 with the address in a
   comment ([ADR-0003](docs/adr/0003-decompiled-code-in-the-project.md)),
   named from the Xbox prototype symbols where they help
   ([ADR-0002](docs/adr/0002-xbox-prototype-symbols.md)), read from the
   game's data, or recorded from the original game. No stand-ins, no
   "close enough" constants. Anything you couldn't trace is listed in the
   pull request and must not change the base-game routes. The
   executable's own built-in default strings (for example the reputation
   titles' descriptions) may be committed, each marked with its address
   (the maintainer's decision of 2026-10-07).
   The community's matching decompilation
   [fnv_decomp](https://github.com/momusdigital/fnv_decomp) may be read
   alongside the exe and used for names, types, layouts and logic, with its
   authors' permission ([ADR-0009](docs/adr/0009-fnv-decomp.md)). Settle
   any difference in the exe, mark what came from it as the ADR says, and
   never commit its C++.
4. **Evidence of play.** Say what you checked by playing (keys, mouse or
   pad) and what you only checked by script (`NV_MOVE_KEYS`, `NV_MOVE_MOUSE`) or unit
   test. Keep logs and screenshots private, or check
   them for usernames and local paths before attaching.
5. **Tests.** A regression test for each behaviour fix, with inputs built
   from scratch in the test (or the oracle's cases). Tests never need game files.
6. **Docs.** The system's topic page in `docs/` says what is done, what is
   traced and what is missing. The maintainer updates
   [docs/MILESTONES.md](docs/MILESTONES.md) when a milestone moves.
7. **AI assistance.** Say which tools helped and with what. You remain
   responsible for the change: review the code and the claims, and run
   the checks yourself.

Never commit game files, extracted assets, executable bytes, decompiler
databases or exports, recordings or logs.

## How the code is laid out

The executable is translated function by function into the engine crates
(`crates/engine*`, [docs/ENGINE_CRATE.md](docs/ENGINE_CRATE.md)); `crates/host` starts it
on the player's FalloutNV.exe and runs its frames, and `viewer/` (`nv-viewer`) is the program
that does that. `crates/frame-model` models the frame's calls (Phase 1),
`crates/esm` reads plugins (for the host), `bsa` archives (for the measurement tools),
`bink`, `mp3` and `vorbis` are decoders the engine uses, and `fos` reads the original saves
(Phase 2's comparator, [docs/FOS_SAVES.md](docs/FOS_SAVES.md)). Keep the player/camera, aim/head, action-input and per-view boundaries
in [docs/VR.md](docs/VR.md). The method is in
[docs/METHODOLOGY.md](docs/METHODOLOGY.md), the research tools in
[`research/`](research/) and the day-to-day steps in
[docs/RESEARCH_WORKFLOW.md](docs/RESEARCH_WORKFLOW.md).

## How pull requests are merged

`main` only changes through pull requests. Each needs the CI checks
and a review from the maintainer (CODEOWNERS), and is squash-merged.

Every pull request, from a person, an agent or the project's own lead, is
also checked independently before it is merged: reviewers compare its
translations with the original game's code and against these rules, try to
prove each finding wrong, and only what holds up is sent back to be fixed.
Changes to CI, build scripts or tooling get the same check: they must not
weaken what the checks catch. Keep
your branch rebased on `main` (at least daily while it's open, and right
before asking for review). When two traced versions of the same thing
disagree, say so in the pull request; the maintainer decides.

The maintainer has the final say on priorities, design and what is
merged ([docs/GOVERNANCE.md](docs/GOVERNANCE.md)).

## Handoffs

If you stop partway, leave the next person (or agent) a short note in the
system's topic page: what works, what's traced, what's left, and one next
action, with evidence rather than claims.
