# Fallout: New Vegas co-op (developer preview)

[![Discord](https://img.shields.io/badge/Discord-chat%20and%20help-5865F2?logo=discord&logoColor=white)](https://discord.gg/EXGB49R82t)

> **Status (October 2026): alpha 0.1.4.** The whole party starts together from a lobby, the opening movie plays for everyone
> at once, and Steam join, voice and text chat, Co-op balanced mode, friend markers and revive are in. Expect bugs: this build
> is for playtesting. Also worth a look: [New Vegas True Co-op](https://www.nexusmods.com/newvegas/mods/100267) by Zyrcxies
> ([source on GitHub](https://github.com/Zyrcxies/NewVegasCoop)), another co-op mod for New Vegas.
>
> | Area | Where it stands |
> |---|---|
> | Sync core (protocol, merge rules, rejoin, rules engine) | built, covered by game-free tests and a 4-player soak |
> | Co-op rules for every story quest | written for 481 quests |
> | Engine hooks that record world changes | cover about two thirds of the script commands the game uses |
> | Two game copies on one PC for automated tests | working |
> | Quest stage sync between two real game copies | working: stages arrive exactly once, with no scripts re-run on the client |
> | Goodsprings stage sweep | 65 of 104 stages pass |
> | Goodsprings story run (two copies play the opening quests start to finish) | 7 of 18 steps pass, including Doc Mitchell's farewell and the gecko fight |
> | Runs on its own, without NV:MP | yes: own networking, player avatars, NPC and combat sync |
> | Steam join, voice and text chat, Co-op balanced mode, friend markers and revive, enemies that split their attention | in the alpha, game-free tests pass, being playtested |
>
> Alpha 0.1.4 is out as a pre-release ([Releases](../../releases)).

Play the whole **Fallout: New Vegas** story together: 2 to 4 players, one shared world, one shared quest
state. This repo is the whole co-op mod: an xNVSE plugin plus a game-free sync core. It keeps quests, objectives,
dialogue, reputation and karma identical on every machine, and it has its own networking, player avatars, NPC and
enemy sync, combat, loot and world sync.

[![Support me on Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/cranebeaker)

### About NV:MP

This project does not use NV:MP. NV:MP (the New Vegas multiplayer mod by the NV:MP team) was only a starting point:
early on we studied how it hooks into the game. NV:MP's networking is closed source, and none of its code is in this
repo. Everything here is our own: the networking, the player avatars, NPC and enemy sync, combat, loot, and the whole
quest and dialogue layer, which NV:MP does not have. The mod runs without NV:MP installed.

There is one small, optional module (`plugin/nvmp`) left from that early phase. It only matters if you run NV:MP
at the same time: it then patches NV:MP's client at runtime so the two do not fight over the same actors. Without
NV:MP it does nothing.

## Roadmap: 10 steps to full co-op

After these ten steps the whole game plays in co-op: story, enemies, combat, loot and the world, for 2 to 4 players.
A step marked **Claimed** is being worked on right now. Want to take one? Open an issue and it gets claimed for you.

| # | Step | What it means when you play | Status |
|---|---|---|---|
| 1 | Enemy and NPC sync | One PC runs each enemy's AI and everyone sees a smooth copy. Hits count once, deaths happen once, encounters spawn once, and enemy attacks, flinches and death poses look right on every screen. | Code written, in-game test next |
| 2 | Enemy sync in the real game | The two-copy fight, ownership handoff, spawn and corpse-loot tests pass on a real PC, and the feature is switched on by default. | **Claimed**: switched on by default in the alpha. The actor adapter passes all 36 checks in game and the Goodsprings gecko fight passes on two copies; the full task list is in docs/WORK-QUEUE.md |
| 3 | Player avatars | You see your friends as characters that walk, run, aim and animate, without needing NV:MP. | Code written, in-game test next |
| 4 | Full combat | Real-time VATS for every player, projectiles and explosions, companions in fights, crime and faction hostility. | Code written (combat, real-time VATS, companions, crime), in-game test next |
| 5 | Loot, items and containers | Shared containers and corpses, item drops and pickups, and no duplicated items. | Code written, in-game test next |
| 6 | World, time and travel | Doors, terminals, switches and traps; time and weather; fast travel, waiting and sleeping by party vote; saving and rejoining. | Code written, in-game test next |
| 7 | Desync check and repair | Every area checks itself every few seconds and fixes any difference it finds. | Code written for enemies, loot and the world, in-game test next |
| 8 | The whole main story in game | Every main questline swept stage by stage with two game copies, from Goodsprings to the endings. | **Claimed**: alpha playtest of the campaign. Script calls synced: 99.98%. Goodsprings: 65 of 104 stages pass in game, and the scripted story run passes 7 of 18 steps |
| 9 | Every side quest and DLC | All side quests, companions and the four DLCs, played through by two-copy test bots. | Playthrough plans and test bots written, in-game runs next |
| 10 | Player release 1.0 | Installer, one-click host and join, split-screen on one PC, player guide. | Alpha package built (installer, launcher, player guide, every feature on by default). Alpha 0.1.4 adds the launcher lobby, a synced start for the whole party, join through Steam friends, voice and text chat and a Co-op balanced mode; playtesting now |

## Play in three steps

**Alpha 0.1.4 is out as a pre-release for playtesting.** Expect bugs, and report them on the issue tracker or on Discord.

1. Download `FNV-CoOp-0.1.4-alpha.zip` from [Releases](../../releases) and unzip it.
2. Run **Install.cmd**. It finds your New Vegas install and adds the plugin next to xNVSE.
3. Run **Launcher.cmd** and type your courier name. The host picks HOST A GAME, friends pick JOIN A FRIEND (an address, or a
   Steam friend with CONNECTION set to STEAM). Everyone readies up and the host starts: the whole party starts the new game together.

You need New Vegas 1.4.0.525, xNVSE 6.3.0 or newer and Python 3. `PLAYER-GUIDE.md` in the zip has the details.

## Status

Everything listed as proven is checked by automated tests; nobody drives the game by hand. See
[docs/STATUS.md](docs/STATUS.md) for the details.

Proven by automated tests (game-free suites plus in-game runs with a remote-control test API):

- The sync protocol and merge rules (stage = max, flags = OR, kills = union), with a simulator of 2 to 4 clients
  on a lossy, reordering link, seeded chaos fuzzing and a 1-hour 4-player soak that converged.
- Capture and replay of a quest-script batch in the real game: the host runs a stage script, our hooks record
  every world change it causes as one transaction, a guest applies it **without re-running the script**, and
  the snapshots match.
- Multiple game instances on one PC (the single-instance lock is worked around inside our own process), so
  tests can run a real host and a real client.
- Host stage change to client over our own ENet channel, baseline join, save/load rebase, barrier release and
  the version handshake, in the real game.

## Known issues

- Developer preview only: no installer, no player documentation.
- Many suites need your own game install or data generated from it. Without it they skip with a message.
- Windows and the Steam or GOG release of New Vegas 1.4.0.525 only.

## How it works

- **Effect capture, not script replay.** Only the host runs quest and dialogue result scripts. While one runs,
  hooks record every primitive world change it causes (stage, objective, global, ref enable/disable, item,
  reputation, karma, quest start/stop, move). That list ships as one ordered transaction, and clients apply it
  directly. No duplicate rewards, no missing side effects, and every transaction doubles as a test fixture.
- **Sync core** (`synccore/`): engine-agnostic C++ with the wire format, merge rules, echo guard, rejoin and
  late-join replay, desync digests and a rules engine for per-quest policies. Design in
  [docs/protocol.md](docs/protocol.md) and [docs/MASTER-PLAN.md](docs/MASTER-PLAN.md).
- **Plugin** (`plugin/`): the xNVSE plugin `nvcoop.dll`, with a remote-control bridge for tests, snapshots,
  engine hooks, dialogue state machines and a co-op runtime.
- **Quest data** (`tools/questdata`): a parser that reads your own game's `.esm` files and works out, for every
  quest stage, what a result script does and how risky it is for co-op.
- **Policies** (`docs/quest-policies/rules.json`): our own per-quest co-op rules (tether the party, share karma,
  rewards to everyone, initiator picks the dialogue).

Building and running the tests: see below and [docs/03-testing.md](docs/03-testing.md).

## Building and tests

You need Windows, Python 3.11+, and for the C++ parts CMake and the MSVC x86 toolchain. xNVSE sources go in
`third_party/NVSE` (see [third_party/README.md](third_party/README.md)).

```
python -I tools/test_all.py --quick     # game-free suites that need no compiler
python -I tools/test_all.py             # everything, including the C++ suites
```

What cannot run without your own game or mods (these suites print `SKIP <name>: <reason>`):

- Quest data tests: run against your install if it is found at the default Steam path, otherwise they skip.
  Generate the data other tools use with `python -I tools/questdata/run_all.py`.
- Coverage and snapshot tests: the parts that need generated game data skip; the rest run.
- NV:MP signature and patcher tests: need your own NV:MP 9.4 client files under `third_party/nvmp/`.
- Dialogue and questline reports: regenerated from your game data; the generated files are not in this repo.
- In-game tests (`tools/ingame_gate.py`, `plugin/coop/tools/`): need a licensed install and `tools/runner`.

This repo ships no game data. Dialogue text, quest dumps and script listings are left out on purpose; run the
tools in `tools/questdata` against your own install to regenerate them.

## Part of coop-anything

This is one entry in [coop-anything](https://github.com/cranebeaker/coop-anything), a community playbook and
directory for adding co-op to single-player classics. If your AI agent helped you fix something here, the
playbook explains how to send it back.

## Credits

- **Zacharia Raven** ([@ZachariaRaven](https://github.com/ZachariaRaven)): in-game testing on a real PC and a big part of
  the code: the split-screen focus guard, the bot, scenario, pool and runner fixes, the bridge `exec` on a reference, and
  the first proof scripts to pass in the game. Thanks a lot for helping with the code!
- **xNVSE** (the New Vegas Script Extender team): https://github.com/xNVSE/NVSE
- **NV:MP** by the NV:MP team (nv-mp.com): studied early on to learn how a multiplayer mod hooks the game. Not included
  and not required.
- **ENet** by Lee Salzman: https://github.com/lsalzman/enet (bundled in `third_party/enet`, MIT licensed).
- **Opus** by the Xiph.Org Foundation and contributors: https://opus-codec.org (bundled in `third_party/opus`, BSD licensed),
  the codec of the voice chat.
- **Zyrcxies** ([NewVegasCoop](https://github.com/Zyrcxies/NewVegasCoop)): some features here were inspired by their co-op
  mod: joining through Steam, positional voice chat and text chat, the Co-op balanced mode, enemies splitting their
  attention between the players, friend name tags and reviving a downed friend. Thanks!
- **Nucleus Co-op** (SplitScreen.Me): the techniques for running several copies of a single-instance game on one
  PC: https://github.com/SplitScreen-Me/splitscreenme-nucleus
- Fallout: New Vegas by Obsidian Entertainment and Bethesda Softworks. You need your own copy.

## License

The code and docs in this repository are MIT licensed, see [LICENSE](LICENSE). ENet and Opus keep their own licenses.
Fallout: New Vegas, xNVSE and NV:MP are the property of their owners and are not included.
