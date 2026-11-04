# nv-rs milestones

Updated 2026-11-03. The active plan and how each step is judged done. Keep
this file short: per-session detail goes in docs/HANDOFF.md, per-function
status in [LEDGER.md](LEDGER.md), and the full list of systems in
[SYSTEMS.md](SYSTEMS.md). The log up to 2026-10-14 is in
[history/milestones-2026-10-14.md](history/milestones-2026-10-14.md).

## The plan

nv-rs ports FalloutNV.exe 1.4.0.525 system by system
([ENGINE_PORT_PLAN.md](ENGINE_PORT_PLAN.md)). The translated code (`crates/engine*`) runs the game in play through
`nv-viewer`; the hand-written reimplementation it replaced was removed on 2026-11-03
([ADR-0011](adr/0011-old-layer-removed.md)).

A milestone is done when all three of these hold:
1. **Translated.** Every function in the system's units is translated, or
   partial only for compiler-only gaps (exception frames, stack cookies).
2. **Running.** The play build reaches the system in play (no stop in it on the
   route that uses it).
3. **Verified.** The behaviour matches the original game: a recording
   (`research/nv-trace`), the oracle (`research/nv-oracle`), a side-by-side
   comparison, or the original's own outputs.

## Engine port

| Milestone | What it covers | Status |
| --- | --- | --- |
| P0: Map and ledger | Every exe function has a unit, a subsystem and a status | Done |
| P1: Frame skeleton | The game's per-frame order as a model (`crates/frame-model`) ([FRAME_SKELETON.md](FRAME_SKELETON.md)) | Done |
| P2: Engine in play | The exe check at start-up, the start-up setup (static initializers), and the script interpreter live for quests | Done (Build 42) |
| A1: Movement and physics | Character controller, player and actor movement, camera collision, the Havok SDK and Bethesda's Havok layer, ragdolls | **Active.** Translated; running in the play build, being checked against the original |
| A2: Animation and faces | NiAnimation, animation choice and blending, FaceGen and lip sync | Translated; running in the play build, being compared with the original |
| A3: Audio | BSAudio, the radio and music; game threads stepped behind the thread seam | Translated; the game's own sound plays through DirectSound, measured against Windows |
| B: Graphics | The scene graph, shaders and lighting, sky and weather, terrain and trees, effects | Translated; the game's own renderer draws through the real Direct3D 9 |
| C: Gameplay | Actor values and combat, AI and pathfinding, quests and dialogue, menus and V.A.T.S., magic, progression, crafting, minigames, world streaming | Translated; the opening's dialogue, menus and Pip-Boy run, weapons draw, aim and fire, combat is being checked |
| D: The rest | Original saves, shut-down, video, settings, real threads, the remaining unplaced functions | Translated; saves load and are written in play, the open world is being switched on |

Approximate timing (maintainer's goal): A and B in about a month, the whole
game in about two months, depending on the weekly budget.

## Product goals

These were the original milestones. The engine port now delivers them.

| Goal | Gate |
| --- | --- |
| Opening to Goodsprings | The opening, Doc Mitchell's character creation and Goodsprings play through on the original's logic; save and reload keep all state |
| Core loop | Sunny's tutorial and Ghost Town Gunfight on translated combat, AI, dialogue and progression |
| Campaign | The base game completes; old saves load |
| DLC and mods | The four DLCs, plugins and archives behave as in the original; script-extender (xNVSE, JIP) compatibility as a later layer |
| Stability | Long play without crashes or lost state, with measured frame times |
| VR | Shared simulation with independent aim and views, tested on a headset ([VR.md](VR.md)) |

## Rules that hold throughout

- Nothing is invented or tuned by eye. Behaviour comes from the exe, its
  data and recordings of the original ([METHODOLOGY.md](METHODOLOGY.md)).
- The original behaviour stays available. Any later bug fixes go in a
  switchable fixes layer.
- Completion needs a reproducible run of the play build compared with the original,
  not a count of functions translated.
