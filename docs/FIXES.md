# The fixes preset

A player who changes nothing gets the `fixes` set; `vanilla` is what FalloutNV.exe 1.4.0.525 does, and
every comparison with the original, gate and recording runs on it (the maintainer, 2026-11-02;
[ADR-0014](adr/0014-typed-migration-in-two-stages.md) point 5, `crates/host/src/fixes.rs`).

## Choosing

| Where | What | Who |
| --- | --- | --- |
| `Settings.cmd`'s `settings.ini` | `Preset=fixes` (the default) or `Preset=vanilla`; `FrameLimit=n` (0: none); `MenuFrameLimit=n` (120 when missing or empty, 0: none of its own) | The player |
| `NV_PRESET=fixes` or `vanilla`, `NV_FRAME_LIMIT=n`, `NV_MENU_FRAME_LIMIT=n` | Over the file | Tests, scripts |
| `NV_UNCAP=0` or `1`, and each fix's own variable below | That fix alone; with no `NV_PRESET` the rest of the run is vanilla | Gates, tests |
| A recording or scripted run: `NV_TRACE_LIST`, `NV_MOVE_KEYS`, `NV_MOVE_MOUSE`, `NV_SOUND_RECORD`, `NV_SHOT`, `NV_TICK_COUNT`, `NV_FRAME_LOG`, the probes and `*_CHECK` switches (the list is `TEST_VARIABLES` in `fixes.rs`), with no `NV_PRESET` | Vanilla | A script that was never told about the preset still measures the original. `NV_SAVE_OUT` (the preview launcher's) does not pin |
| `NV_SETTINGS=FILE` | The settings file, when it isn't `settings.ini` beside the program or in the folder above it | Launchers |

It is read once at start-up and bound before the first frame (`fixes::install`): the log's `Preset ...`
line says what the run is. Anything it can't read stops the start with a message. A settings file with a UTF-8 mark or in UTF-16 (PowerShell 5.1's `>`) is decoded; one that exists and can't be read stops the start, unless `NV_PRESET` is set, which beats the file whatever it holds. `Host::start` runs
vanilla, for tests and tools; the viewer reads the choice and calls `Host::start_with`.

Anything that compares with the original says `NV_PRESET=vanilla`. Speed gates say `NV_PRESET=vanilla
NV_UNCAP=1` as before.

## How a fix is built

- An override keyed by exe address, a plain function registered over the translation when it is on.
  Off, nothing is registered, so the translation runs as it is and no call tests a flag.
- Translations are never edited for a fix.
- Its addresses are in `crates/engine-core/data/overridable.tsv`, so typed code keeps calling them by
  address (`docs/ENGINE_CRATE.md`, "Typed code"; a test checks).
- Tests in both modes: the translation's behaviour stays what its own tests say; the fix's tests
  install it.
- Its row says why it is in the set, from measurements.

## Measuring

`NV_FRAME_LOG=FILE` writes a line per main frame (clock, step, Havok accumulator, player position)
without a per-call cost, so the frame rate stays the player's. The game's own `iFPSClamp` INI line
(`[General]`, 1 to 199) gives exact frame steps at any wall rate, which is how a recording sees a
high frame rate's steps: a trace itself holds the game near 30 frames a second.

`NV_PHYS_BLEND_LOG=FILE`, with the smooth physics drawing on (say `NV_PRESET=fixes`: the variable pins
vanilla otherwise), writes per drawn frame each free body's node as the original draws it and as the
fix draws it, with the drawing time and the world's last step end; `NV_SHOT_FRAMES=FROM-TO` limits
`NV_SHOT` to those presents, so a short stretch can be taken every frame.

## The fixes

The game's own `iFPSClamp` ignores `FrameLimit`: a clamp is a fixed step with no wait, which the uncapped step doesn't pace, so a recording with `iFPSClamp` runs as fast as it can.

| Fix | Address | Switch | What it does |
| --- | --- | --- | --- |
| Uncapped frame rate | `00aa4ee0` | `NV_UNCAP` | `BSTimer`'s step without its sleep and its 10 ms floor, on the microsecond clock. `FrameLimit` paces it. Where the original sleeps (a step under 10 ms by the tick count) the other threads get a `Sleep(0)` pass instead, about one frame in three, and the first step after a disable answers as the original does (10 ms) |
| High-frame-rate landing | `00c73170` | `NV_HF_LANDING` | The player's character, landing standing still in a frame under 16 ms, falls on as far as at 16 ms frames: one more integration with `Move`'s own velocity, large-creature mode word and step. Measured on the player only, so other characters run `Move` as it is. The extra time is 32 ms minus the frame, so it jumps at 16 ms (15.9 ms gets about 32 ms in all, 16 ms none); the fall stops at the floor either way, and the difference is under half a unit (0.43 at 16 ms frames), not visible, so it is not smoothed |
| High-frame-rate slow motion | `00aa4e40` | `NV_HF_SLOWMO` | The time multiplier moves to its target by the time passed, not by the frame |
| Menu frame limit | none of its own (paces the uncapped step) | `NV_MENU_CAP` | A menu (main menu, Pip-Boy, dialogue, inventory, pause, message boxes) runs at most `MenuFrameLimit` frames a second (120 unless set; `FrameLimit` when lower), a loading screen with no limit, even under a `FrameLimit` (the maintainer, 2026-11-07). What a frame shows is the game's own test (`Main::OnIdle`, `0086e650`): a menu is the menu-mode byte `011dea2b` the frame sets first (`Interface::IsInMenuMode` or the Pip-Boy opening); loading is the loading menu up (`LoadingMenu::pMe`) with neither the main menu (`XUIIsUp`) nor the credits over it, the frames in which `OnIdle` asks `TES` whether the load is done. The load's own work, while the main thread is busy and the loading menu's thread draws, has no main-timer step and was never limited. With the uncapped step off nothing changes. `NV_FRAME_LOG` has a `screen` column (play, menu, loading) |
| Dragged bodies by time | `0095f6c0`, `009613c0`, `00960520` | `NV_GRAB_PUSH` | The grab update (`00960520`) pulls a held body that is more than `fZKeyComplexHelperMinDistance` (6) units from its place by adding `(distance - 6) x fZKeyComplexHelperScale (2) / distance x -1.0 x` a weight factor (`fZKeyComplexHelperWeightMin` 0 to `Max` 100) to its velocity, once a rendered frame (`AddVelocity` for a ragdoll or a flagged body, the velocity plus the pull for others), so at 144 frames a second it is pulled 2.4 times as hard as at 60. Its three entries (the player's grab handling and the dropped-object spring, which call it in Rust, and the grab update itself, which telekinesis calls) run with `fZKeyComplexHelperScale` scaled by the frame's seconds times 60, at most 4 (TEMPO's rule in Fallout 3), and the setting back afterwards; the grab update is its only reader. The spring itself is a Havok action on the 16 ms steps and is left alone |
| Smooth physics drawing | `00c65a80`, `008706b0` | `NV_PHYS_BLEND` | Free bodies (dropped and thrown items, ragdolls, debris) are drawn between Havok's 16 ms steps instead of frozen on frames without a step: around the frame's drawing only, each moving body's node shows the body at the game's time minus one step, blended with Havok's own swept-transform formula, and is put back after. The simulation and what game logic reads are the original's. Bodies are drawn one step (16 ms) behind the clock, about 5 ms later than the original on average. Below |
| Mod fit worn question | `007838a0` | `NV_MOD_WORN` | Fitting a weapon mod asks whether the mod is worn before the mod is removed from the inventory, not after. The original asks after (a slip: it means the weapon), through the mod's extra list the removal has just deleted; it nearly always answers no, but a yes rebuilds the inventory list, which frees the weapon entry the mod menu keeps, and closing the menu then re-equips through it and the game stops. A mod is never worn, so the answer is the original's usual no, and no released memory is read. Every other click of the menu is the original's. Below |

### What high frame rates do to the original's code

Measured on the port's vanilla translation at exact frame steps (the game's `iFPSClamp`), against the
real game on the same route (2026-11-06, the recorded route's Doc Mitchell's hall, Vikki and Vance,
Goodsprings; real frames 10, 15 and 16 ms):

| Frame step | 16.67 ms | 10 ms | 8.33 ms | 5.03 ms | Real game |
| --- | --- | --- | --- | --- | --- |
| Height above the floor after a standing jump | 0.4 | 3.2 to 5.1 | 4.0 to 5.5 | 6.6 to 7.0 | 1.1 to 2.8 |
| Jump height | 61.54 | 62.52 | 62.77 | 63.26 | 61.5 to 61.9 |
| Rise at rest after a load: player | 0.269 | 0.007 | 0.006 | 0.004 | 0.029 |
| Rise at rest after a load: an NPC in Vikki and Vance | 0.599 | 0.336 | 0.186 | 0.006 | 0.346 |
| Player's run, units a second | 334 | 334 | 334 | 334 | 336 |
| Dropped stimpak, steps until asleep | 93 | 101 | 93 | 85 | 101 |
| Doc's body, steps until asleep | 220 | 176 | 151 | 173 | 128 (140, 232, 140 in other real runs) |
| Havok steps | 16 ms | 16 ms | 16 ms | 16 ms | 16 ms |

- **Rigid bodies don't care:** the world steps 16 ms with 4 solver steps at any frame rate
  (`00c66760`, `00c6af..`); under 8 ms a frame's time is carried and it has no step. Settling
  has no trend with the frame step. NVTF sets `fMaxTime` to the frame's time; nothing measured here asks for
  that, and it changes the solver's work a second. It goes with the fixed-rate simulation, later.
- **The game's clock.** A frame shorter than a `GetTickCount` tick is reported as 10 ms, so the game's
  time runs ahead of the real time as frames get short: the real game at 94 frames a second in the hall has
  13.7 ms of game time a frame in 10.66 ms of real time, 1.29 times real time. Above 100 frames a second
  the original would go faster still (the "sped-up game" of the Viva New Vegas guide's FAQ). The uncapped step follows the real time.
- **The original's sleep.** `Sleep(9)` in about one frame in three in the hall (732 of 2300), and that frame
  takes 11.8 ms against 7.6 without: it isn't rounded up to the 15.6 ms tick.
- **Rest height follows the frame step alone.** A still character's rise from its start stops at the first step
  whose contacts move under 0.001 Havok units. Nothing here depends on foot placement.
- **Not settled:** animation (NVTF's `bSpiderHandsFix`) has no measure here. The jump is 2.4% higher at
  200 frames a second, which is the integration's own step, and stays. An NPC at rest after a load stands 0.34 units
  lower than in the real game at 200 frames a second and 0.25 higher at 60; left.

### Before and after

Height above the floor (7360.36 in Doc's hall) where the player rests after three standing jumps, one
after the other. Real frames with `NV_FRAME_LOG`, the `iFPSClamp` rows with a recording:

| Frames | Uncapped alone | With the landing fix |
| --- | --- | --- |
| Limit 60 | 0.69 | 0.69 |
| Limit 120 (the hall reaches 93 a second here) | 4.57, 3.04, 1.36 | 0.69 |
| No limit (about 86 in the hall) | 2.83, 3.01, 1.78 | 0.69 |
| Exactly 120 | 3.96, 5.53, 4.65 | 0.69 |
| Exactly 199 | 6.98, 6.77, 6.55 | 0.69 |
| The real game at its own rate | 1.10, 1.90, 2.83 | |

It holds up to the 199 a second tested. Frames of 16 ms and longer run `Move` untouched. The game's time against the
real time is 1.001 to 1.005 at every limit.

Frame rate at the maintainer's settings (2560x1440, 8x anti-aliasing, borderless; about two minutes a run,
2026-11-06): Doc's room 70.2 frames a second on vanilla, 104.7 with the fixes; Goodsprings about 43 and about 44 (the original never sleeps there; the second from 15 s only).

### The menu frame limit, measured

Port runs on the play build (2026-11-07, 1280x720 window in the background, `NV_FRAME_LOG`'s `screen`
column; Save 138 from the main menu, then the Pip-Boy in Doc's house):

| | Main menu | Pip-Boy | Play (Doc's house) | Save load |
| --- | --- | --- | --- | --- |
| `NV_MENU_CAP=0` | 179.9 | 157.2 | 74 | 3.87 s, no main-loop frame |
| The fixes preset (120) | 119.9 | 119.0 | 76 | 4.08 s, no main-loop frame |

A load draws on the loading menu's own thread (a round every 50 ms, as the original) while the main
thread does the load, so there is no main-timer step to limit: the save load has no frame of its own
and a `coc` cell load one (with `FrameLimit=90`), which ran unpaced. The loading screens' speed is the
original's either way.

### Smooth physics drawing

Havok steps the world 16 ms at a time at any frame rate, and the original draws each free body as the
scene graph's update last set its node (`hktoNiTransform`, `00c65a80`): above 62.5 frames a second many
frames have no step, so dropped items, ragdolls and debris stand still on them and jump on the next. The
fix keeps the simulation and every node game logic reads exactly as the original's; only around the
frame's drawing (`008706b0`) it writes each moving body's node as the body was one step before the game's
time, blended between the two Havok steps around it with Havok's own swept-transform formula
(`approxTransformAt`: centre of mass and rotation moved linearly, rotation normalised), refreshes the
nodes below it with the game's own world update, draws, and puts every node and bound back as it was
(`crates/host/src/phys_blend.rs`). Nodes whose class has its own world update (world-space particle systems,
fixed-rotation `BSValueNode`s, billboards) and everything below them, and keyframed bodies, are drawn as the
original draws them.
The original's nodes are set before the frame's world step, so they show the body 11 ms behind the game's
clock on average (0 to 23 ms at 144 frames a second); the fix draws it 16 ms behind, about 5 ms later.

Measured on the hf-motion route in Doc's hall (three dropped items and Doc's ragdoll, 60 to 76 moving
bodies; `NV_PHYS_BLEND_LOG`, both series from the same frames; 2026-11-07). Frozen: a frame in which the
body's path moves and the drawn body doesn't. Shown/ideal: the drawn move in a frame against the path's.

| Frames | Frozen, original | Frozen, fix | Over 1.5 times the path's move, original / fix | Shown/ideal p10, p50, p90, original | Fix |
| --- | --- | --- | --- | --- | --- |
| Exactly 144 (`iFPSClamp`) | 56.6% | 0.4% | 43.2% / 0.5% | 0.00, 0.00, 2.31 | 0.98, 1.00, 1.02 |
| Limit 60 (68.5 a second of game time measured) | 7.7% | 0.0% | 9.9% / 1.0% | 0.53, 1.03, 1.50 | 0.99, 1.00, 1.03 |

The fix's frozen frames are bodies that just woke or were moved by the game (a `SetPosition`): they are
drawn from their first step's start until the drawing time reaches it, one or two frames. Off the path:
0.04 units (rms) at 144 against the original's 0.75. With `NV_PHYS_BLEND=0` the same route shows 56.5%
frozen, as before the fix, and the three dropped items settle step for step as with the fix on (same
steps active, speeds and resting places). Vanilla at exactly 60: one step in 95.8% of frames, two in 4.2%,
as before.

On screen (`NV_SHOT` every frame at exactly 144, looking down at the dropped shovel): with the fix off every
second frame of its fall is the same picture as the one before and the next moves twice as far; with the fix
on the picture changes every frame, by half as much (one frame stays still where the shovel hits the floor).

### Mod fit worn question

`ItemModMenu::DoClick` (`007838a0`), on a click of a mod in the item mod menu, fits the mod to the weapon, removes one
of the mod from the player (`007839ca`), plays the sound, takes the entry out of the list and rebuilds the menu, and
only then asks `ItemChange::GetWorn` of the mod's list entry (`00783a3f`). The removal deletes the mod's extra list
when the last one goes, so the question reads a released block. In the original the block nearly always still says
"not worn" (the list's flags were cleared before the delete, and the 32-byte pool keeps a freed block's bytes past
the first eight). A "worn" makes the player update its equipment and rebuilds the inventory list
(`InventoryMenu::UpdateInventoryMenu`, `00782a90`), which frees the weapon entry the mod menu keeps (`011d9f58`);
closing the menu (E) re-equips the weapon through it and the game stops: the play report of 2026-11-07
(measure/weapon-mod-crash). The port's heap hands freed blocks back more widely than the original's pools, which
makes the "worn" more likely; that is fixed in the allocator, separately.

The fix runs the list's click as the original's, step for step, with the worn question asked of the same entry just
before the removal (`crates/host/src/mod_fit_worn.rs`). A mod is never worn, so the answer is the original's usual
"no" and nothing more happens; no released memory is read. Tests: with the fix the click reads nothing released and
rebuilds nothing; vanilla still asks after the removal and rebuilds on a "yes".

## Credits

Written with the fix plugins' knowledge, by their authors' permission (2026-11-03); no code is
translated from them in these fixes.

- **New Vegas Tick Fix (NVTF)**, carxt, lStewieAl and jazzisparis, LGPL-2.1: `BSTimer`'s fixed
  step from the measured frame and the survey of what goes wrong at high frame rates.
- **ITR NVSE** (Into the Rough contributors, MIT): `SlowMotionPhysicsFix`, read for what slow motion does
  to physics.
- **TEMPO** (Fallout 3): scaling the grab's per-frame pull by the frame's time times 60, at most 4.
- **Viva New Vegas** (ModdingLinked, MIT): the guide's frame-rate rules, which these numbers were checked
  against.
