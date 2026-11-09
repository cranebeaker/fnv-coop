# nv-rs milestones

Updated 2026-11-06. The active plan and how each step is judged done. Keep
this file short: per-session detail goes in docs/HANDOFF.md, per-function
status in [LEDGER.md](LEDGER.md), and the full list of systems in
[SYSTEMS.md](SYSTEMS.md). The log up to 2026-10-14 is in
[history/milestones-2026-10-14.md](history/milestones-2026-10-14.md).

## Direction (maintainer, 2026-11-04; [ADR-0012](adr/0012-modern-64-bit-engine.md))

The game looks and plays exactly like the original; what players do not see or
feel may be different. The original (the exe, fnv_decomp, recordings) is the
reference every change is tested against, not the way it has to be built.
Today the translated code runs inside a model of the 32-bit program (one 4 GB
address space, the x87 FPU modelled); the game's systems move one by one to
typed, native 64-bit Rust (from fnv_decomp's typed C++, through a handle
boundary so the game stays playable), and the modern systems below are built on
them.

Order: Rust pinned (done) -> the platform crate split -> the fnv_decomp audit and
crash fixes (4 GB guest heap alongside) -> the typed 64-bit migration, the hottest
systems first (culling, shader setup, the AI tasks), which is also the main speed
work, with the quick wins that outlive it (a render thread, PGO, the light-view
wait) -> the modern systems. Deep optimisation of the address-level translation
(guard pages, generated direct calls) is skipped where typed code replaces it.

The fixes preset exists: on by default, vanilla a setting away ([FIXES.md](FIXES.md)).

Where it stands (2026-11-06): the platform split and the audit are done; every
hand-translated function has been compared with the exe and fnv_decomp, and the differences
found are fixed, apart from a few left on purpose (docs/HANDOFF.md). The typed migration is
starting. Rounding differences found by the audit are left to it, since it rewrites that maths.
Since 2026-11-08 the first nine typed families are on by default (leaves, the two topic families, and
lockfree, culling, pathing, shader, crt and dispatch after their speed gate: Goodsprings about +9%, the
crowded street +10%; docs/ENGINE_CRATE.md, "Typed code").

An interim Windows preview (maintainer, 2026-11-06) can be built with `scripts/package-playtest.ps1`:
a ZIP a player can run with no build, labelled a preview, wgpu remaining the big release. It has only
run on this PC; the maintainer decides on a release ([testers' checklist](PREVIEW_TESTERS.md)).

| Area | Goal |
| --- | --- |
| Memory and loading | No 4 GB limit (big mod lists, 4K/8K textures); more of the world loaded; memory-mapped archives, background loading, cached cells; per-thread allocators |
| CPU | A job system on every core (AI, animation, physics, culling, scene, LOD, sound, scripts) with results proven identical; pipelined frames; cache-friendly data and SIMD culling; scripts compiled to native code |
| Smoothness | 200+ fps: a fixed-rate simulation with interpolated rendering (none of the original's high-frame-rate physics and timing bugs); precompiled shaders |
| Graphics | wgpu (the only backend): a render thread, GPU-driven drawing; switchable real-time shadows, modern AA, HDR, upscaling, more lights, longer draw distance |
| Stability | Memory- and thread-safe code; crash reports with the exact place; optional safety autosaves; safe background saves |
| Modding | A built-in script extender (xNVSE, JIP commands); more than 255 plugins with existing mods unchanged; a better console, hot reload, clear errors |
| Platforms | One 64-bit codebase: Windows, Linux, macOS, ARM, consoles |
| Co-op | Intended (maintainer, 2026-11-06); no design yet |

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
| B: Graphics | The scene graph, shaders and lighting, sky and weather, terrain and trees, effects | Translated; the game's own renderer draws through the real Direct3D 9; the wgpu backend (`crates/d3d9-wgpu`) draws the same frames beside it (intro and menu identical, Doc's house within 0.3% of pixels), not selectable yet |
| C: Gameplay | Actor values and combat, AI and pathfinding, quests and dialogue, menus and V.A.T.S., magic, progression, crafting, minigames, world streaming | Translated and playable through the whole game, quests and DLCs included; translation errors cause bugs and crashes (the fnv_decomp audit) |
| D: The rest | Original saves, shut-down, video, settings, real threads, the remaining unplaced functions | Translated; saves load and are written, the open world plays; crashes from translation errors (the audit) |

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
  data, fnv_decomp and recordings of the original ([METHODOLOGY.md](METHODOLOGY.md)).
- What players see and feel stays the original's; internals may change when a
  recording or comparison shows no observable difference.
- The original behaviour stays available. Any later bug fixes go in a
  switchable fixes layer.
- Completion needs a reproducible run of the play build compared with the original,
  not a count of functions translated.
