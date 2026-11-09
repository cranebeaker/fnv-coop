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
| A recording or scripted run: `NV_TRACE_LIST`, `NV_MOVE_KEYS`, `NV_MOVE_MOUSE`, `NV_SOUND_RECORD`, `NV_SHOT`, `NV_TICK_COUNT`, `NV_FRAME_LOG`, the probes and `*_CHECK` switches (the list is `TEST_VARIABLES` in `fixes.rs`), with no `NV_PRESET` | Vanilla; and, with no `NV_THREADS`, stepped threads whatever the play default is (`Choice::threads`); with no `NV_RENDER_THREAD`, no render thread (`Choice::render_thread`) | A script that was never told about the preset or the threads still measures the original, and doesn't change when a play default does. `NV_SAVE_OUT` (the preview launcher's) does not pin |
| `NV_SETTINGS=FILE` | The settings file, when it isn't `settings.ini` beside the program or in the folder above it | Launchers |

It is read once at start-up and bound before the first frame (`fixes::install`): the log's `Preset ...`
line says what the run is. Anything it can't read stops the start with a message. A settings file with a UTF-8 mark or in UTF-16 (PowerShell 5.1's `>`) is decoded; one that exists and can't be read stops the start, unless `NV_PRESET` is set, which beats the file whatever it holds. `Host::start` runs
vanilla, for tests and tools; the viewer reads the choice and calls `Host::start_with`.

Anything that compares with the original says `NV_PRESET=vanilla`. Speed gates say `NV_PRESET=vanilla
NV_UNCAP=1` as before.

The game's threads are not part of the preset. In play they run side by side on their own cores
(`fixes::PLAY_THREADS`); the line `NV_THREADS=stepped` in `settings.ini` (Settings.cmd's "Game
threads": "One at a time") runs them one at a time, the fallback. A test or scripted run without
`NV_THREADS` runs them stepped whatever play's default is (the table above).

The render thread isn't part of the preset either: it changes where the real Direct3D 9 calls are
made, not which calls or in what order. In play they are made on a render thread of their own
(`fixes::PLAY_RENDER_THREAD`, since measure/render-thread: town +14%, the crowded main street +16%;
#347); the line `NV_RENDER_THREAD=0` in `settings.ini` (Settings.cmd's "Drawing thread": "Main
thread") makes them on the game's threads, as the original does, the fallback. A test or scripted
run without `NV_RENDER_THREAD` makes them on the game's threads whatever play's default is.

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

`NV_LOCK_STATS=FILE` (real threads only) writes a line per main frame with the game clock and how long
the main thread and the other game threads waited that frame for the kernel lock, for critical
sections, for events, semaphores and threads, and in sleeps, how often a section leave woke waiters, and
how long the host held the kernel lock before the frame (joined with `NV_FRAME_LOG` by the clock), then
how long main waited in the frame-end barrier (`barrier_us`) and the waits for the Direct3D device lock,
main's and the other threads' (`main_device_*`, `game_device_*`). Beside it, `FILE.sites` lists, for each
frame in which main's sleeps and object waits passed 50 ms, those sleeps and waits and the other threads'
by the exe function that made them (the caller of `Sleep` or the wait), with the sleep's milliseconds,
the first handle main waited on, or the thread's id. `NV_FRAME_LOG`'s last column, `qpc`, is the
Windows performance counter, for an outside sampler's timestamps.

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
| Dialogue topic index | `00619820`, `0061a310`, `0061a380`, `0061a5a0`, `0045dfe0` | `NV_TOPIC_INDEX` | Finding which topic holds a dialogue info (`TESTopic::GetOwnerTopic`) no longer walks the topic list and every info in it: the original does that for each info a plugin changes (YUP and the add-ons: 5,837 times), so a dialogue mod's load grew with its size times the game's. Every topic `TESTopic::InsertInfo` links an info into (the one place an info enters a topic) is kept beside it; a search asks only those topics, with the original's own search (`00619e10`), which hold this info first by its id, and takes the first of them in the topic list's order, from a copy of the list's order that is made again whenever the list may have changed in a way that matters (its sort, the data handler's `ClearData`, an insert of a topic the copy holds, which can list it again earlier, a changed head node, or a topic inserted since that the copy doesn't hold). So the answer is the walk's, tie-breaks included (an info in two topics, a twin id, the list's order and end). `NV_TOPIC_INDEX_CHECK=1` with `NV_TOPIC_INDEX=1` (alone it is a test variable, which makes the run vanilla) runs the original's walk beside every search and stops the game at the first difference. With the fix on, the typed `topic_search` family (the walk) goes by address for the run, since the fix registers over one of its members. Below |
| Shaders compiled ahead of first use | none (a flag the real Direct3D 9 backend reads) | `NV_SHADER_WARM` | The graphics driver compiles a shader when it is first drawn with, and the game makes all 960 of its packages' shaders at start-up but first draws some of them in play: the first shot draws 7 for the first time (`SKIN2005.vso`, `SKIN2002.pso`, `SLS2136.pso`, `SLS2148.pso`, `SM3000.pso`, `NOLIGHTTEXVCPMA.pso`; `d3d9-first-use`), and its frame took 29 ms where the others take 12.5 (46 ms without the render thread). It recurs at every launch, whatever the exe's path. With the row the real backend draws once with every shader the game makes, into a 1x1 target of its own, after a `Present`, and sets the game's bindings again with its own commands: 960 draws in 2 ms of the render thread at the first `Present`, compiled by the driver while the menus run. The first shot's frame: 28.6 and 29.2 ms without, 19.3 and 19.8 with (the shooting route, BAAB, 2026-11-09); loads not slower (Doc's house 2.26 and 2.28 s without, 2.14 and 2.10 s with); the real device's frames identical to wgpu's while it runs |
| Co-save keeps other plugins | none (a flag the extender reads) | `NV_COSAVE_KEEP` | A co-save's sections of plugins that are not built in (JIP LN NVSE's, until it is) are kept as they were read and written back on each save while the files loaded are the ones that co-save lists, so a save brought from a real modded installation keeps them. xNVSE writes only the plugins it has loaded, so off (vanilla) they go at the next save, as with xNVSE. Below |
| Script error loop guard | none (a flag the extender reads) | `NV_NVSE_ERROR_GUARD` | An error raised while a script's `RuntimeScriptError` handler runs (in the handler itself, or at the call depth limit when handlers call functions that fail) is not sent to the handlers again. xNVSE sends it again, which recurses until the stack runs out and the game stops; off (vanilla), the port does the same. Below |
| Image-space event form check | none (a flag the extender reads) | `NV_NVSE_IMOD_CHECK` | When an image-space modifier instance runs out, OnRemoveIMOD is raised only for an instance of a modifier form (the game's own `AsForm` test, as its `Stop` makes it), with that form. xNVSE reads a form at the same place in every instance, and a depth-of-field or radial blur instance (iron sights, V.A.T.S., `SetGlobalRadialBlur`) has a number there: a handler then reads garbage as a form. Off (vanilla), the word is read as xNVSE reads it, and a word that is not a readable form drops the event instead of stopping the game. Below |
| Inventory reference checks | none (a flag the extender reads) | `NV_NVSE_INVREF_CHECK` | Three places where xNVSE's inventory references free, read or keep memory they should not. `RemoveMeIR` on an unworn stack with data of its own (a weapon with its condition) frees that data through the entry the reference holds, which for a reference from `GetInvRefsForItem`, `ForEach`, OnSell or OnRepair is the reference's own copy: the container's entry keeps the freed data and its count, so the item stays with freed memory as its data. With the row the data leaves the container's own entry too (its count with it) before it is freed, and nothing is freed when no entry of the container holds it. OnSell's item stays kept after the event when no handler ran (every handler filtered out), so a later `GetSoldItemInvRef` reads a sale that is over; with the row it goes when the event ends. A container whose base list has an entry without a form stops the game in `ForEach`, as it does with xNVSE; with the row the entry is passed over. Off (vanilla), all three are xNVSE's, the stop included. Below |
| Script array and string size caps | none (a flag the extender reads) | `NV_NVSE_SIZE_CAPS` | An array `ar_Range`, `ar_Init`, `ar_Resize` or `ar_Generate` makes or grows stops at 8,388,608 elements (about 335 MB), and `sv_Replace` of an empty text a given number of times stops at 64 MiB. The keys `MenuTapKey`, `MenuHoldKey` and `MenuReleaseKey` queue for the game's buffered key reads (drained only when the game reads them, in menus) stop at 4,096; the rest are dropped. A format string the C runtime cannot make is made safely: a precision past 965,624,998 (more working memory than the runtime gets in a 32-bit process) gives a number with a point the runtime's own fallback of 163 digits and an integer the zeros asked, and a format the runtime refuses (`%Ld`, `%5n`, a second size such as `%hld`) gives an empty text. In xNVSE such an integer precision crashes the runtime, a refused format ends the game (the runtime's default invalid-parameter handler: nothing in xNVSE's process installs another, measured in the real game), and whether a number's precision gets its memory depends on the game's; off (vanilla), the port stops the game at all three. xNVSE goes on until it runs out of memory and the game stops; off (vanilla), the port stops too: an array past 33,554,432 elements or a string past 1 GiB or more than 67,108,864 queued menu keys stops the game as out of memory, about where xNVSE's 32-bit process runs out, so the computer is never asked for more. A range that never ends (a step of 0, or one that wraps past its end), `ar_Cat` of an array with itself and an empty text replaced without a count never end in xNVSE and are capped the same in both presets. Below |
| Script missing array guard | none (a flag the extender reads) | `NV_NVSE_NULL_GUARD` | `ar_Count` and `ar_GetNth` on a missing array answer 0, and a format string's `%s` (or `%S`, `%Z`) pointing outside the game's memory ends the text. xNVSE reads through the null array or the pointer and the game stops; off (vanilla), the port does the same. Below |
| Script array cycle guard | none (a flag the extender reads) | `NV_NVSE_CYCLE_GUARD` | A deep copy (`ar_DeepCopy`), a deep compare (`ar_DeepEquals`) or a dump (`ar_Dump`, `ar_DumpF`) of an array that holds itself, directly or through others, ends where it meets the array again: the copy holds its own copy there, the compare counts that pair as equal, the dump writes `[...]`. xNVSE walks on until the stack runs out and the game stops; off (vanilla), the port does the same. Below |
| Script inventory list guard | none (a flag the extender reads) | `NV_NVSE_LIST_GUARD` | A game list the inventory commands walk (a container's stacks, a stack's extra data lists) that runs back into itself ends the walk after 1,048,576 nodes. xNVSE walks it for ever and the game hangs; off (vanilla), the port stops the game there instead. Below |

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

### Dialogue topic index

`TESTopic::GetOwnerTopic` (`0061a310`) walks the data handler's topic list and asks each topic (`00619e10`, first to
last) for the info's id until a topic answers with this very info. Loading a plugin that changes an existing info
asks it once per such info (`TESTopicInfo::ClearData`, to take the info out of its owner): with YUP and the add-ons
5,837 searches of about 68,000 infos each. The fix (`crates/host/src/topic_index.rs`) notes every topic
`TESTopic::InsertInfo` (`00619820`, the one place an info enters a topic) links the info into, asks only those with
the same `00619e10`, and of the ones answering this info takes the first in the list's order. The order comes from a
copy of the list, made again after the sort (`0061a5a0`), the data handler's `ClearData` (`0045dfe0`), an insert
(`0061a380`) of a topic the copy already holds (it can list the topic again, earlier), a changed head node, or when
an answer is missing from it after an insert. So the answer is the walk's, tie-breaks included. JIP LN answers from
a word set as the load links the info, which is the last topic linked, not always the walk's answer. With the fix on,
the typed `topic_search` family goes by address for the run (the fix registers over its member `0061a310`).

Measured 2026-11-07 (measure/topic-index; YUP, the five add-ons and SR556, vanilla preset with only `NV_TOPIC_INDEX`
switched, default typed families, so "off" is the typed walk):

- `NV_TOPIC_INDEX_CHECK=1` with `NV_TOPIC_INDEX=1`: over 5,000 searches (the load, the main menu and Continue), the
  index gave the walk's answer every time; the list was copied twice.
- Profiled load (`NV_PROFILE`): the search goes from 27 of 32 profiled seconds to too small to list. The profiler
  slows every call, so this says where the time was, not how much.
- The final build, fix on, four runs: the first main frame 4.39-5.01 s from the process start every time, the load
  done (its last sort) at 3.2-3.8 s. The index's counters, the same in all four: 5,837 searches, each answered by
  the one topic holding the info, 3 copies of the list's order, 54 ms in all (7 ms of it copying).
- Off (the typed walk), six runs: 7.85-8.61 s. Add-ons only, no YUP, is 4.3 s (measure/yup-load).
- Five earlier runs with the fix on (builds before the counters' line printed) took 7.86-8.02 s, as slow as off;
  three took 4.49-4.64 s. Why is not known. The review suspects a wait before the first main frame that has
  nothing to do with the index: even in the fast runs the first main frame comes about 1.2 s after the load's
  end. So no fixed saving is claimed here.
- Continue (save to world) 3.2-4.0 s with the fix on and off.

### The script extender

The built-in xNVSE (ADR-0016) is the same in both presets, except for these rows. Like xNVSE, it
writes a `.nvse` co-save beside every save, an empty one too, in both presets: a save without
one loads as a new game for the extender without resetting what the game before held
(`Serialization.cpp`, `HandleLoadGame`), which only a save made without the extender (a vanilla
save) should get.

- **Co-save keeps other plugins** (the row above): the maintainer's decision, 2026-11-08. The
  sections are kept byte for byte from the co-save of the game loaded (none after a new game or a
  save without a co-save) and written after xNVSE's own section, with that co-save's file list:
  only while today's files are the same ones in the same order. Their data holds mod indexes of
  that list (JIP LN NVSE maps them through the list it was saved with), so under another list
  they go, as with xNVSE.
- **Script error loop guard** (the row above): the maintainer's decision, 2026-11-08 (review of
  #329). One error reaches the handlers once; the console still shows every error. Kept per game
  thread: an error another thread raises meanwhile is its own and reaches the handlers.
- **Image-space event form check** (the row above): from the review of #340, 2026-11-08. Vanilla
  keeps xNVSE's read and only refuses a word that is no readable form, since ordinary play must
  not stop.
- **Inventory reference checks** (the row above): from the review of #349, 2026-11-08. All three
  are xNVSE 6.4.9's own (`InventoryReference.cpp`'s `RemoveFromContainer`, `EventManager.cpp`'s
  `OnSell`, `GameObjects.cpp`'s `GetInventoryItems`). Vanilla is 1:1 with them, the stop on a
  base entry without a form included (the review's decision: xNVSE stops the game there too).
- **Script array and string size caps, missing array guard, array cycle guard** (the rows
  above): the coordinator's decision, 2026-11-08 (review of #346). Vanilla is xNVSE's string
  and array commands as they are, their crashes included, except an outright hang or an
  allocation without end, which is capped in every preset; every other difference is one of
  these rows, on by default. The out-of-memory stop (an array past 33,554,432 elements, a
  string past 1 GiB): the coordinator's decision, 2026-11-08 (follow-ups to #346); vanilla
  stops where xNVSE's 32-bit process would, instead of taking the computer's memory. The format
  strings' part (issue #357): the coordinator's decision, 2026-11-08: where the C runtime
  crashes, ends the process or needs output without bound, the size caps row makes the text
  safely and vanilla stops the game. That a refused format ends the game was measured in the real
  game (xNVSE 6.4.9 with JIP LN 57.30: `sv_Construct "refused [%Ld]" 5` ends it with
  0xC0000409, the next line never runs); the memory limit was measured in a 32-bit harness and
  moves a little from run to run.
- **Script inventory list guard** (the row above): from the review of #365, 2026-11-09. The game's
  own lists never run back into themselves; the bound only turns a hang into a stop, or with the row
  into the end of the walk.
- **Input** (I-11): the queue of menu keys is the size caps row's (review of #362); otherwise no
  row. xNVSE's DirectInput hook stops the game only on calls the game never makes (a buffered read
  that peeks with records, a mouse read of another size, a record size other than DirectInput 8's);
  every keyboard and mouse read the game makes goes through it as with xNVSE, in both presets.

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
