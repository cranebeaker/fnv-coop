<h1 align="center">FNV Co-Op</h1>

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
[![Engine](https://img.shields.io/badge/Engine-nv--rs_(Rust%2C_Bevy_0.16)-orange.svg)](https://github.com/slaterain/nv-rs)
[![Status](https://img.shields.io/badge/Status-alpha-lightgrey.svg)](docs/PROGRESS.md)

**Play the Fallout: New Vegas story together.** One player hosts, friends join with a join code, and
everyone is in the same Mojave: the same quests, the same NPCs, the same fights, the same cutscenes.

FNV Co-Op is built on [nv-rs](https://github.com/slaterain/nv-rs) by **slaterain**, a from-scratch
Fallout: New Vegas engine written in Rust that runs the game from your own copy's files. Co-op needs an
engine we can change, which is why it lives there. The engine is theirs and is licensed GPL-3.0; the
co-op layer in this repository is GPL-3.0 as well. This project is not affiliated with Bethesda,
ZeniMax, Obsidian or Microsoft and contains no game files: you need your own copy of the game
(see [LEGAL.md](LEGAL.md)).

> [!NOTE]
> The earlier version of FNV Co-Op, an xNVSE plugin for the original game, lives on the
> [`legacy-xnvse`](https://github.com/cranebeaker/fnv-coop/tree/legacy-xnvse) branch and in the older
> releases. `main` is the Rust engine version described here.

<p align="center">
  <img src="docs/media/goodsprings-aerial.jpg" alt="Goodsprings from above, rendered by nv-rs" width="100%">
</p>

## What co-op does

- **One shared story.** The host runs the quest scripts. Quest stages, objectives, globals, topics,
  notes and map markers are the same for everyone. Rewards are given once, to the right players.
- **Per-player where it should be.** Each player has their own character, inventory, skills, perks
  and followers. The campaign rules decide which quests are shared and which each player runs on
  their own (487 quests across the base game and all four story DLCs).
- **Seeing each other.** Other players appear with name tags and animate: moving, aiming, attacking,
  reloading, getting hurt, dying. NPCs and creatures are spawned once and die once on every machine.
- **Playing together.** Kills are decided by the host and counted once. Loot comes out of a container
  once, with no duplication. Dialogue is a shared session. Barter, doors and cell changes, lockpicking,
  hacking, crafting, companions, V.A.T.S. (which never freezes the other players), downed and revive,
  and a friendly-fire guard are covered by the rules.
- **Cutscenes and time.** Scripted scenes and movies start for everyone, controls lock per player,
  endings play as a slideshow, and sleeping or waiting is a party vote.
- **Starting and saving.** The host starts the normal opening; each guest makes their own character.
  Party saves keep the host's world plus a save per player, so a campaign can be continued.
- **Staying in sync.** UDP transport with a Steam (Spacewar) option, snapshots on join, rejoin after a
  lost connection, hash checks that detect and repair drift, and a version and mod-list check at join.
- **Voice, split-screen and four players.** Optional voice chat, couch split-screen on one PC, and up
  to four players in a session.

## How it is tested

Everything is held to a master checklist ([docs/COOP-CHECKLIST.md](docs/COOP-CHECKLIST.md), 709 items,
plain-language version in [docs/PROGRESS.md](docs/PROGRESS.md)). A feature counts as done when its row is
green and a mutation twin of the test (the same test with the feature deliberately broken) fails.

- The headless simulator (`coop-sim`) runs a real host and guests over a simulated bad network on real
  game data, and walks every one of the 487 quest rules quest by quest.
- A second harness (`tools/coop-run`) starts two real game processes and checks the result.
- At the latest run: **611 of 709 items green, none failing**; the rest are rows still waiting for their
  test. The checklist file lists which are which and says what each check does and does not cover.

## Try it

There is no packaged release of the Rust co-op build yet; the existing releases are the earlier
xNVSE version. To run it today:

1. Install stable Rust (MSVC toolchain on Windows) and have your own Fallout: New Vegas 1.4.0.525 install.
2. Build and start the launcher:

```sh
cargo build --release
cd viewer && cargo build --release && cd ..
target/release/nv-launcher --print-command host     # prints the exact viewer command without starting it
target/release/nv-launcher                          # the terminal menu: host or join
```

The launcher finds your install (read only), hosts or joins with a join code (`NVRS-...`) or
`ip:port`, keeps party saves per campaign, tests the UDP port and collects shareable logs with your
Windows user name removed. `scripts/package-coop.ps1` bundles a ready-to-share folder with
`Play-Coop.cmd`. Session details are in [docs/COOP.md](docs/COOP.md).

For development, run the checklist and the simulator:

```sh
cargo build --release -p coop-headless
target/release/coop-sim checklist --data "<path to Fallout New Vegas>/Data" --area WORLD
```

Use `--release` for tests on this workspace; it is large.

## Where things are

| Path | What |
| --- | --- |
| `crates/coop-core` | sync core: deltas, snapshots, hashes, live layer |
| `crates/coop-net`, `coop-session` | UDP and Steam transport, the session driver |
| `crates/coop-policy` | the 487 quest rules (`data/policy.json`) |
| `crates/coop-headless` | `coop-sim`: simulator, scenarios, the checklist |
| `crates/world/src/coop` | game-logic side: authority, shadows, policy gate, adapters |
| `crates/coop-voice`, `crates/nv-launcher` | voice chat and the player-facing launcher |
| `viewer/` | the game client (its own workspace) |
| `tools/coop-run` | real two-process scenario runner |
| `docs/COOP*.md` | design and how-to for each co-op part |

## Credits and licence

- **slaterain** and the nv-rs contributors for the engine this stands on:
  [github.com/slaterain/nv-rs](https://github.com/slaterain/nv-rs). Engine documentation, its method
  (every behaviour translated from the original and checked against it) and its contribution rules are
  in [docs/](docs/), [CONTRIBUTING.md](CONTRIBUTING.md) and [AGENTS.md](AGENTS.md).
- Co-op layer: the `cranebeaker` team. This repository is built with AI coding agents under human
  direction, in the same open way as the engine.

Licensed under the [GNU GPL v3 or later](LICENSE). Fallout and Fallout: New Vegas are trademarks of
their respective owners; read [LEGAL.md](LEGAL.md) for the full notice.
