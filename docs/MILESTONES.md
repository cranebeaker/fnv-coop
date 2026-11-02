# nv-rs milestones

Updated 2026-10-14. The active plan and how each step is judged done. Keep
this file short: per-session detail goes in docs/HANDOFF.md, per-function
status in [LEDGER.md](LEDGER.md), and the full list of systems in
[SYSTEMS.md](SYSTEMS.md). The log up to 2026-10-14 is in
[history/milestones-2026-10-14.md](history/milestones-2026-10-14.md).

## The plan

nv-rs ports FalloutNV.exe 1.4.0.525 system by system
([ENGINE_PORT_PLAN.md](ENGINE_PORT_PLAN.md)). Translated code in the
`engine` crate replaces the hand-written systems one at a time. Each
switch-over ships as a play build.

A milestone is done when all three of these hold:
1. **Translated.** Every function in the system's units is translated, or
   partial only for compiler-only gaps (exception frames, stack cookies).
2. **Switched on.** The viewer runs the translation in play instead of the
   hand-written system, with a fallback where one is still needed.
3. **Verified.** The acceptance routes pass, and the behaviour matches the
   original game: a recording, a side-by-side comparison, or the
   original's own outputs.

## Engine port

| Milestone | What it covers | Status |
| --- | --- | --- |
| P0: Map and ledger | Every exe function has a unit, a subsystem and a status | Done |
| P1: Frame skeleton | The game's per-frame order drives the app's systems ([FRAME_SKELETON.md](FRAME_SKELETON.md)) | Done |
| P2: Engine in play | The exe check at start-up, the start-up setup (static initializers), and the script interpreter live for quests | Done (Build 42). Quests load through the game's own loader; stage items still need seven small functions (see HANDOFF) |
| A1: Movement and physics | Character controller, player and actor movement, camera collision, the Havok SDK and Bethesda's Havok layer, ragdolls | **Active.** The character controller and player mover are done; the Havok SDK (about 5,100 functions) has not started |
| A2: Animation and faces | NiAnimation, animation choice and blending, FaceGen and lip sync | Not started |
| A3: Audio | BSAudio, the radio and music; game threads stepped behind the thread seam | About two-thirds translated; the seam is not built |
| B: Graphics | Bevy upgrade first, then the scene graph, shaders and lighting, sky and weather, terrain and trees, effects | Not started |
| C: Gameplay | Actor values and combat, AI and pathfinding, quests and dialogue, menus and V.A.T.S., magic, progression, crafting, minigames, world streaming | Mostly translated; switch-overs not started |
| D: The rest | Original saves, shut-down, video, settings, real threads, the remaining unplaced functions | Not started |

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

## Linux viewer

The release viewer built and launched on Fedora in a native Wayland session
with Vulkan, with `DISPLAY` unset, and loaded `GSDocMitchellHouse` from the
user's local game data (2026-10-15). The GPU is an NVIDIA GTX 1650. On
2026-10-26, `nvidia-smi` reported driver version 615.78.08 on the Fedora
machine. Gameplay has not been compared against the original on Linux, and no
packaged Linux build is provided. **Next action:** run and compare a gameplay
acceptance route on Linux.

## Rules that hold throughout

- Nothing is invented or tuned by eye. Behaviour comes from the exe, its
  data and recordings of the original ([METHODOLOGY.md](METHODOLOGY.md)).
- The original behaviour stays available. Any later bug fixes go in a
  switchable fixes layer.
- Completion needs a reproducible acceptance run, not a count of functions
  translated.
