# fnv-coop

Story co-op for Fallout: New Vegas, built on the Rust engine reimplementation
[nv-rs](https://github.com/slaterain/nv-rs) by slaterain.

Two to four players share one campaign: one world, one quest log, one story. The host runs the
real simulation. Guests run the same engine with a different authority table: they send what they
do, and apply what the host decided.

## Credit

This project is a fork of, and is built entirely on, **nv-rs by slaterain**
(<https://github.com/slaterain/nv-rs>, GPL-3.0-or-later). The engine, the translated game code, the
viewer and the research tooling are theirs. Everything co-op is added on top: `crates/coop-*`,
`crates/nv-launcher`, `crates/world/src/coop`, `viewer/src/coop*`, `tools/coop-run`, `services/`.
The upstream README is kept in [docs/UPSTREAM-README.md](docs/UPSTREAM-README.md). The earlier
exe-hooking version of this project is preserved on the
[`legacy-xnvse`](https://github.com/cranebeaker/fnv-coop/tree/legacy-xnvse) branch.

This is an independent project. It is not affiliated with or endorsed by Bethesda Softworks,
ZeniMax Media, Obsidian Entertainment or Microsoft, and it contains no game files. You need your own
copy of Fallout: New Vegas (Steam version 1.4.0.525). See [LEGAL.md](LEGAL.md).

## What works

Tested on real game data with two real viewer processes, host and guest, over a simulated lossy
network (good, bad and flaky Wi-Fi, blackouts):

- shared quest stages, objectives, globals, topics, items and rewards
- per-player inventories, skills and quests where the campaign rules say so (487 quest rules)
- the other player visible as a moving character, with name tag
- shared combat and kills decided once by the host, loot leases, dialogue sessions
- join snapshot, rejoin after a dropped connection, desync detection and repair
- chat, difficulty scaling by player count, revive window, story tether

Known limits: only one cell is simulated at a time, so the guest follows the host. The engine
itself currently plays Doc Mitchell's house and Goodsprings, so the shared story goes as far as
upstream does. Steam (Spacewar) transport is built but has not been tried with two real accounts.
Direct UDP works today.

## Try it

You need Rust (stable, MSVC on Windows) and your own Fallout: New Vegas install.

```
cd viewer
cargo build --release
nv-viewer.exe "<path to Fallout New Vegas\Data>" GSDocMitchellHouse --coop-host 27961 --coop-name Host --coop-test-token
nv-viewer.exe "<path to Fallout New Vegas\Data>" --coop-join 127.0.0.1:27961 --coop-name Guest --coop-test-token
```

`--coop-test-token` is for same-PC or LAN testing. A hosted session prints a join code instead.
More in [docs/COOP.md](docs/COOP.md) and [docs/COOP-STEAM.md](docs/COOP-STEAM.md).

## How it is tested

Co-op is tested by scripts, not by people:

- `crates/coop-core` and `crates/world/tests/coop*`: in-process host and guests over a seeded
  network simulator, run in seconds.
- `crates/coop-headless` (`coop-sim`): runs the same scenarios as the two-process harness against
  the real game data with no window, across all 7 network profiles and many seeds.
- `tools/coop-run`: starts a real host and a real guest viewer, injects lag, judges the shared
  state, and writes a verdict file. Each scenario has a twin that must fail, so the checks cannot
  pass by doing nothing.

## Layout

| Path | What |
|---|---|
| `crates/coop-core` | deltas, snapshots, hashing and repair, wire protocol, live layer (avatars, actors, dialogue, loot, rules, chat) |
| `crates/coop-net` | UDP and Steam transports, join codes |
| `crates/coop-session` | the session driver the viewer calls each frame |
| `crates/coop-policy` | per-quest campaign rules (who may advance a stage, rewards, barriers) |
| `crates/coop-voice` | voice packets, jitter buffer, mixer |
| `crates/coop-headless` | headless scenario simulator |
| `crates/nv-launcher` | player-facing launcher |
| `services/join-service` | join-code lookup for a Cloudflare Worker (not deployed) |

## License

GPL-3.0-or-later, as nv-rs. See [LICENSE](LICENSE).
