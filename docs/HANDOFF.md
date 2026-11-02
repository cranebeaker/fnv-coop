# Handoff: engine port push (updated 2026-11-01)

For the next Claude session leading the nv-rs engine port. Read AGENTS.md,
docs/MILESTONES.md, docs/ENGINE_PORT_PLAN.md, docs/FRAME_SKELETON.md and
docs/ENGINE_CRATE.md first. This file says where things stand and how the
machinery runs.

## Resume here

### Claude lead, 2026-11-01 (current: read this first, then 2026-10-31 below)

- **Merged:** #271. This PR: the DirectSound cursors as measured and the weapon idle sound's frame.
- **DirectSound cursors (`dsound.rs`, `mixer.rs`, `resampler.rs`; docs/SOUND.md):** `SetCurrentPosition` on a playing buffer (the open outdoor stop) is measured and provided. The write cursor is now the input frames the converter has taken: with the second converter (`dsound.dll` `100636a0`) `resampler::AplFrames` repeats `Apl::period`'s index arithmetic in the same `f32` steps without making samples, kept from the first `Play` through `Stop`, the end and `SetFrequency`; the first converter rounds up with the first `Play` 33 frames ahead; at the device's rate there is no lead. A `SetFrequency` to the frequency a buffer already has changes nothing; a non-looping buffer ends in the period its write cursor would reach its length (a stopped buffer is never at its end: the old "Play from the end" stop is gone); a looping buffer turned not looping plays out its lap; when the second converter turns on, every playing buffer's cursors go to its write cursor (`1005713d`, not measured). The ignored `measured_replay` test drives 1,058 measured lines (`scripts/dsound-capture` modes `setpos`, `endphase`, `freqphase`, `endstop`): 1,012 match every read (main: 105); the 42 first-converter lines at 11040/22071 Hz and 4 races inside DirectSound are listed in SOUND.md. The output samples hash is unchanged. Six rounds of two independent reviews.
- **Weapon idle sound (`009231d0`):** the record `004e75d0` copies nine words into is the frame's 0x24-byte local at `EBP-0x6c` (`009237e4`); the port gave it 12 bytes, so an NPC drawing or holstering a weapon wrote 24 bytes over the temporary sound handle and past the frame (found by the block check outdoors). The other twelve callers of `004e75d0` already give it 0x24 bytes.
- **Outdoors, open:** long walks and `cow` trips stop with the guest heap full (1.5 GB): at the stop 569 MB were live and 901 MB sat on the exact-size free lists, which are never split or merged. A good-fit allocator that splits and merges is in review (next PR, with the flat guest memory: about 4% faster in Doc's room).
- **Frame rate:** the next priority (maintainer, 2026-11-01): reach the original's ~120 fps in Doc's room. A measured investigation is under way (profiler with native stack walks and per-target call counts, build settings, call dispatch, the hot translated code, allocation, the platform layers and game threads, the port's call counts against the real game's).

### Claude lead, 2026-10-31 (evening; read after 2026-11-01 above)

- **Merged:** #270. This PR: outdoors (Goodsprings via `coc goodsprings`) from the first stop to walking through the town, Exit Game, and the host's speed work.
- **Outdoors, stop by stop, then whole subsystems audited.** Each fix exposed the next stop in code reached for the first time, so the code under each stop was audited against the exe instruction by instruction (functions not covered by the call audit of executed code): the navmesh obstacle cutter (164 functions, 8 fixed: e.g. `006997b0` passed `navmesh + 0xe0` where the exe loads the map stored there; `006bd040` gave the new-vertex/edge helpers the navmesh where the exe gives the cutter, `006bd398`; `00699690` stepped 12-byte triangle records by 2), exterior cell loading (385 functions, 36 fixed: e.g. `00527cb0` ran `QueueReferences` on the cell's form id instead of the cell, `00527ccb`; `00528080`/`005283c0` called the map's slots on the word at `this + 0` instead of `this`, and `005283c0` dropped the smart pointer it holds the task in). Also `hkpCapsuleCapsuleAgent::getClosestPoints` (`00d3baf0`, `00d3bd80`) handed the collector a contact without the two body pointers stored after it (`00d3bb36`), the first time two capsules met outdoors. Every fix was reviewed by two independent reviewers against the disassembly.
- **Open outdoors:** `IDirectSoundBuffer::SetCurrentPosition` on a playing buffer (`00aed080` < `00addc00`) is not measured yet (the platform layer stops); being measured (done 2026-11-01, above).
- **Exit Game:** the renderer frees its `D3D9.DLL` on exit (`FreeLibrary`, `00e6ec6d`): now a platform row. When main returns, the play build ends the process with main's answer, standing in for the startup's `exit` (`00ecc479`).
- **Speed:** the call path (inline dispatch, a non-`Option` call cache, a vcall fast path), the D3D9 layer's per-call costs and per-frame heap allocations: Doc's room from the post-race-menu autosave is 2-8% faster at equal work (A/B/A, about 40-80 fps by view; the original runs about 120). The rest of the frame is the translated code itself (about 80 % of samples): next, direct guest memory and direct calls.
- **Disk:** keep the project under 150 GB (it reached 270 GB; C: filled three times). Delete battery frames, old test exes and stale `target` folders as you go.
- **Scans that came back clean** (each flag judged against the exe): calls to functions the exe function never reaches (309 strong flags, all scanner artefacts), vtable calls on the word at `this + 0` (45 sites, all correct), calls through a stream's own read word at `+8` (36 sites, one wrong: `LoadTree`, fixed in #270).

### Claude lead, 2026-10-31 (morning; read after the evening above)

- **Merged:** #267 (the night below). This PR: the rest of the night's fixes.
- **Music:** (1) `IMediaControl::Pause` on a graph that never ran stopped the game on the music thread in 2 of 3 fight starts; `dshow.rs` now does what Windows does (measured with `scripts/dshow-measure --states/--rounds`, private `build43-scratch/measure/dshow-states-2026-10-31`): the first leave-stopped queues `EC_CLOCK_CHANGED`, answers `S_FALSE` until the graph settles (1 ms), `EC_PAUSED` 10 ms later, and every other transition the music thread (`00830b30`) can reach. (2) The fight-start stutter: `IGraphBuilder::Render` decoded the whole MP3 on the music thread's cooperative turn (334-406 ms frames). The stream now decodes on an OS thread of its own from `Render` on; a seek or `Run` past what is decoded starts a decode a few frames back (`mp3::Decoder::seek`: bit reservoir, IMDCT overlap and synthesis history refilled), so every sample is bit-identical to the whole decode and nothing the game reads changes.
- **Outdoors (`coc goodsprings`):** `CSpeedTreeRT::Compute`'s leaf pairing works on the leaf-vector copy in its argument area (`00b40b78`), not a moved copy, and the merge loop steps 8-byte pairs (`00b41d40` at `00b40d2c`, was the 4-byte `00b2abc0`: wrong merges, a read past the end). `RemoveEditorMarkers` (`004b5d10`) gives slot `0x90` (`SetAVObject`, `RET 8`) the 0 pushed before each getter as its second word (`004b5de5`).
- **Bug classes, each finding reviewed twice against the exe:** (1) a static scan of all 149,340 direct calls (results read from callees registered as returning nothing; word counts): 11 real (e.g. `hkMap::findOrInsertKey` returns the slot, `00cb1f40`; the weapon-mod menu's damage call, `006450f0`; `hkMatrix3::getDeterminant` returns `out`), 211 false alarms; (2) objects the exe builds in its outgoing argument area (`SUB ESP,n; MOV ECX,ESP; CALL ctor`, 104 sites, 38 constructors): self-referential checked vectors moved by the port in FaceGen (`00aba270`, `00aba2f0`, `00aade20`) and SpeedTree (`00b1ff50`, `00b336a0`, `00b17db0`) are now built and destroyed where the callee uses them; (3) calls through a stream's own read/write word at `+8`/`+0xC` (36 exe sites): no other site wrong.
- **Battery (p242, block check on):** Save 138 (weapons, Pip-Boy), a new game, the saloon, the wait menu, VATS and a fight (Doc damaged and in combat) run without a stop; outdoors stopped at the editor-marker call fixed here.
- **Doc's held props:** compared with the real game (private `measure/rec/prp1-*`): the blend and attach code matches; the real game also blends the stimpak idle's left arm with the aim pose and also waits about 48 frames for the weapon animations after an AI equip. The difference is the frame rate (the wait is 0.4 s at the real ~120 fps, 1-1.6 s in the port) and Doc standing still where the real Doc walks off (an AI comparison to do).
- **Tools (private scratchpad):** `callscan.pl` (unregistered targets, word counts, void results), `streamword.pl`, `argobj.pl` (exe byte scans). Run `cargo test -p nvinspect --test repo_hygiene` before pushing: no Ghidra auto-names (`local_*`, `auStack_*`) in comments.

### Claude lead, 2026-10-31 (night; read after the morning above)

- **The play branch's fixes, merged in one PR:** sound cuts (the DirectSound cursor model: a buffer played again leads by the frames rounded up, an ended buffer stops at 0 and restarts there; measured with `scripts/dsound-capture` `replays`), the sound output thread run as the player's dsound.dll runs its mixer (time-critical, MMCSS "Audio" at high priority), the real clock catching up after back-to-back readings, the window (the process is DPI-unaware like FalloutNV.exe, so winit no longer shrinks it; the window takes focus when shown), the `SteamAPI_Shutdown` platform row, the decal manager's skin-node `AttachChild` (`004a2636`), the TOI solve's slot `0x48` on the event's contact manager (`00d0f41e`), `009c0d80` re-reading the hit's collidable after its re-cast (`009c121d`) and `00df7230` re-reading `this + 0x18` after its slot-4 call (`00df727c`). Scripted runs (`NV_MOVE_KEYS`/`NV_MOVE_MOUSE`) no longer take the real window's keyboard and mouse (`NV_MOVE_REAL_INPUT=1` lets them in).
- **Already in #266 (not recorded there):** `XInputSetState` answers no controller connected, as `XInputGetState` does (the rumble code `00a255b0` reaches it when firing); dragging the title bar lets go of the cursor (`WM_ENTERSIZEMOVE`), the cursor is taken only on `WA_ACTIVE` or a click in the client area; `0093e860` at `00941fea`: with a weapon, the aim button raises the iron sights unless the weapon is a drawn melee weapon, which blocks.
- **Guest stack:** `Engine::with_stack` now takes each local from the running game thread's own 1 MB stack (frames go down, `FRAME_GAP` 0x40 above each, `STACK_TOP_RESERVE` 0x1000 kept free at the top, zeroed), as the exe's locals do; heap blocks only with the block check on (`NV_BLOCK_CHECK`). Tests predict a local with `Engine::next_stack_local`. 256 tests that had predicted heap addresses were rewritten. A caller that catches stops and goes on (the host's stages, static initializers, the quest VM) puts the stack back with `Engine::guest_stack_mark`/`guest_stack_restore`.
- **Bug classes fixed everywhere:** a `with_stack` scope closed before a pointer into it was used (14 sites), frames smaller than the object built in them (`BSSoundHandle` is 12 bytes: the Pip-Boy's sound frames; 4 more sites), and a static check of every direct call's words against the callee's registered signature (71 flagged, 15 real: missing unread words that shifted every later argument, two wrong calls; 56 false alarms such as two-word floats and variadic callees). The checks are in the private scratchpad (`arity.pl`, `framescan.pl`); `NV_BLOCK_CHECK=store,freed[,log]` now also quarantines freed blocks (64 MB, filled with `0xD0000000|addr>>4`) and names the freed block and both call paths on a read through a stale pointer.
- **Two stops fixed after a scripted battery (p239, `NV_BLOCK_CHECK=store,freed,log`: Save 138 with weapons and the Pip-Boy, a new game, the saloon, the wait menu and VATS ran without a stop):**
  - The first time an actor's ragdoll goes dynamic (Doc after `damageav health 70`), main stopped with "read of unmapped memory at 00000000 (in 00d1fb00 < 00ca1410 ...)": `hkAabb::isValid` (`00da0b30`) returns its `result` pointer in EAX at both RETs (`00da0b78`, `00da0b85`) and `detectPenetration` reads the answer through it (`00d1fc03`); the port's translation returned nothing. Main's coroutine ends after a stop, so this looked like a freeze inside Havok.
  - Outdoors in Goodsprings a tree load threw "read short": `CSpeedTreeRT::LoadTree` (`00b05420`) reads the file through the stream's own read word at `+8` of the object, cdecl with the file first (`00b0551f`..`00b05527`); the port made a virtual call at `+8`, which is the stream's `Seek` (`00aa1a90`). A byte scan of the exe for `MOV r,[r+8|0xC]; CALL r; ADD ESP,n` found 36 such sites in 20 functions; this was the only one the port got wrong.
- **Open from the same runs:** pausing a music graph that never ran (`IMediaControl::Pause` at `0083193b` on the music thread's first pass) stops the game in `dshow.rs` (not measured yet), 2 of 3 fight starts; and opening a music track decodes the whole MP3 inside `IGraphBuilder::Render` on the cooperative turn (350-400 ms frames: the fight-start stutter). Both are being measured and fixed.
- **Not port bugs:** the 9mm "firing 35 shots" and the shotgun's "self-hits" came from the maintainer's real mouse reaching scripted test windows (`crates/host/src/player_move.rs` merges the real input); every traced press fires once.
- **Open (being investigated):** Doc's held stimpak and laser pistol looking rotated (attach and transform paths match the exe; two leads need the real game: the stimpak idle's 50/50 arm blend with the aim pose, and about 48 frames of the pistol on the forearm after an AI equip).
- **Frame rate target (maintainer, 2026-10-30):** as fast as the original (about 120 fps in Doc's house); faster systems of our own later, opt-in.

### Claude lead, 2026-10-30 (night; read after 2026-10-31 above)

- **Merged this round:** #262 (the last C runtime functions: coverage 99.95%; the 29 left are 13 `platform` stand-ins for Windows and Microsoft code and 16 C runtime fragments that only raise an invalid-opcode exception), #263 (the Pip-Boy, crate and map-door crashes above), #264 (the call audit's first fixes and `SetWindowTextA`).
- **The call audit (lead, 2026-10-30):** every function the game ran in a play run (11,947 in all, 11,105 with calls) was packed with its exe disassembly into 278 review packets (private `build43-scratch/measure/audit/`, built by `measure/batches.pl` from `measure/executed-withcalls.tsv`); one auditor agent per packet compares every call (callee, word count/order, value vs address, float vs integer, result use) and a second agent tries to refute each finding (workflow `call-audit`, journal in the session's `subagents/workflows/wf_bbad4ca7-88a`). About 9 in 10 packets came back clean. Fixed and merged (#264): the ragdoll update's enabled test reads Actor slot `0x364` (008bd287), not `0x360`; `0057ab70` restarted its map walk on every pass (a freeze when an entry is skipped); `0095c730` called `00950290` for `00952290`; `00968730` swapped `SetRadiationLevel`'s words; `00886360` passed the TES pointer's address as `this`; `00446e10` read its setting through a pointer (the setting is the static itself), which then reached `SetWindowTextA` (now a platform function: the window record's title, forwarded to the real window); `006987d0` used the wrong comparator; flag tests `<`/`<=` in `005f3a20` and `0063b630`; `0050ee50` tested the clone's scale instead of the source's. Checked and harmless: `004b5d10`'s slot `0x90` on `NiControllerManager` is a bare `RET`. The rest of the findings are being verified; fix them from the workflow's confirmed list. To make the packets for another run: `NV_MOVE_CALL_PROFILE=FILE NV_MOVE_CALL_PROFILE_TOP=0` (host, diagnostic) lists every function a run executes.
- **Frame rate:** Doc's house from the post-race-menu autosave, uncapped: 41-50 fps before, 46-51 after the play-branch speedups (guest heap hash maps and a stack-frame fast path, a fixed-size page table, the x87 round-to-nearest fast path, `#[inline]` on small helpers, the D3D9 layer's borrowed device, flat shader constants and no per-call allocations, the viewer's frame overlapping the game's when its window is hidden). A design panel of five investigators and two judges put every remaining host-side idea together at about 3-4 ms of a 21 ms frame; 200 fps needs more than host-side trimming (the cost is spread through the translated code itself). Ideas left, best first: guarded direct calls for the hottest call edges, profile-guided optimisation of the release build (`cargo pgo` is installed), a flat 4 GB guest space. These speedups are on the play branch, not on main yet.
- **New stop, not looked into yet:** p225 from Save 138 stopped at the render-pass check (`NV_PASS_CHECK`-style diagnostic): `RenderPassArray::Add reuses slot 0 (below m_usESize) ... but the slot is null` in 00ba9ee0 < 00bdf650 < 00b67be0 < 00b64020.
- **Pip-Boy in Save 138 stopped at "read of unmapped memory at 00000004"** (00761540 < 007cb2e0 < 0054dcf0 < 0079ffb0 MapMenu < 0070c4a0 InterfaceManager::Idle): `0054dcf0` (the cell's door list for the map) put each door into its list with the array append `007cb2e0`; the exe uses the linked list's front insert `005ae3d0` (0054dda1). The cell's two other door lists (`0054db50`, `0054dc00`) really use the array append. Fixed; checked live: Tab opens the Pip-Boy (Items) in Save 138 and closes it (p223).
- **Pip-Boy stop fixed:** opening the Pip-Boy (any menu: stats, inventory, map) stopped the game at "open function 00ffa010". `interface.rs`'s `PIPBOY_SELECT_TAB` was written `0x007f_fa010 & 0x00ff_ffff` (one digit too many, then masked), which is `00ffa010`, zero bytes past the code. The exe calls `007fa010` (00704dbd). Every `Interface::Set*MenuVisible` ends with that refresh. Unit tests used the constant by name and could not see it. A scan of every `e.call` target in the port (literal or same-file const) against the exe's function starts and the port's entries found no other bad target (private `agents/scratch/lead/jmpscan/calltargets.pl`).
- **Word check (autosave route) fully triaged.** Real misassignments: NiCamera/NiSwitchNode stream read/write (the stream's own function words at +8/+0xc, not virtual slots); `HUDMainMenu` status mode (007739f0): the 0/0/1 the exe pushes before each GetActorValue are 00773e90's blink flag, so the third limb icon (0x4a) blinks (the port passed false). Harmless extra word removed: `TESFile::LoadForm`'s form id getter (008d8ac0). Everything else matches the exe's pushes: the 1-given-0-taken rows are vtable getters that ignore `this`; `00afc660`/`00afbf80`/`00aff9d0` (the binary stream's read words) really get 5 pushed words and read 3; `005b5e40` is variadic; `00b9f780 > 00b6c0d0` cleans its stack late (optimised code). Tool: private `jmpscan/cdeclcheck.pl` (ADD ESP after each direct call + the callee's ECX use). Still to run: the word check on the new-game opening and Save 138.
- **Frame-116 actors: not load order.** Both games have HonestHearts.esm at index 02 (empty plugins.txt; five ESMs share one file time). The port evaluates HonestHearts creatures 02010A19..1B (ACRE, persistent, base 02010B60) at frame 116; the real game never does in the opening, and later evaluates HonestHearts NPCs 0200B824..27. The second batch differs too (real 0013AA43/AAC4/AAC2/AAAD, port 0013AA43/AA2F/AAC4/AAA8/AA8D/AA85). The three creatures are the Ghost of She's duplicates (`NVDLC02GhostOfSheShaderCodeBox`, `NVDLC02MS01QuestScript`), flags 0xC00: persistent and **initially disabled**. The port calls `EvaluatePackage(actor, 0, 1)` then `(1, 1)` on them; every real call in the opening passes 0 as the last word. The exe's callers that pass 1 there: `Enable` (00573f40) and the `ResetAI` command (005c9530) with (0, 1), `BaseProcess::FinishInitLoadGame` (008d0e80) with (1, 1). If the port enables initially-disabled references at a new game, other disabled things could appear. Not settled: the trace list `build43-scratch/measure/enable-check.txt` is ready, but the scripted new game kept missing the menu (uncapped menu frames run about 240 a second; key frames at +1800, +9000 and +20000 all fired too early or late); run it with the `ngrm.sh` route on a capped build.
- **Frame rate:** the real game also draws about 1,300 times a frame in Doc's room (docroom-2: 6,494 draws over 5 frames), so the port's cost per draw is the gap (about 17 us against about 6). Play-fast builds engine-fallout at opt-level 1 with 256 code units a crate; the maintainer's packages are `--release`.
- **Crash walking into the Sunset Sarsaparilla crate fixed:** the physics thread stopped at "open function 010cc934" (that is `hkpSimpleContactConstraintData`'s vtable). `hkAgent1nMachine_Process` (00da4aa0) handed the agent cleanup at `+0x16b8` the **address** of the input record's contact-manager word (`ctx + 0x10`) instead of the word (`00da5670 MOV ECX,[ESP+0x84]`), so `hkGskManifold_cleanup` (00dad640) made its slot-0x14 call on the wrong object. Both pass sites (`One` and `Many`) fixed; three unit tests had encoded the misreading (`assert_ne!(..., 0x4444)`). Runs when a swept (moving) contact finishes.
- **Frame rate, measured honestly:** Doc's house from the post-race-menu autosave runs 41-50 fps uncapped even fully optimised (`play-opt`), so build settings were not the bottleneck (the main menu runs about 200). With the PDB next to the exe (`nv_viewer.pdb`; the profiler searches the exe's folder) `NV_PROFILE` names native functions. Findings: the guest heap's B-trees (every `with_stack` frame allocates) ~2.3 ms a frame, now hash maps; x87 24-bit rounding ~1.4 ms, now a fast path to nearest checked against the exact routine; small cross-crate helpers (`Ret` accessors, field load/store, x87 ops) were not inlined, now `#[inline]`; the D3D9 layer clones no COM pointers per call and its per-draw maps use the address hasher. The model loader's per-frame walk (`00448cc0`) visits 5,039 buckets (a prime: the map's bucket count, not stuck tasks), by design. Per frame about 345,000 `Engine::call`s. These speedups are on the play branch (`claude/build43-movement`), for the next PR, not in this one.
- **Not reproduced / need the maintainer:** "settings don't apply" (which ones?), the white orb flashes (occlusion queries go to the real GPU; a screenshot or F12 report would help), console names (probably vanilla: most references have no editor ID; compare in the real game), the crate's weight (the maintainer compares in the real game), "snappy" animations (blend-outs match the real game in the opening recording: immediate deactivations on both sides).

### Claude lead, 2026-10-29 (afternoon; current: read this first, then 2026-10-28/29 below)

**Merged:** #258 (the book-grab stop and five more jump-to-slot forwarders). **#259** (merged): Doc looking at the floor (two bugs: `00c78610` left the solver's reference direction and largest angle out of its setup; `00ce0290`'s roll polynomial misread `00ce0a49` `DC C9`), the crash talking to an NPC a second time (`00607990` passed the dialogue item to `0057b7c0` instead of the topic), Z grabbing (two bugs: `CreateMouseSpring` 0095f981 refused every non-actor object; then the spring's setup block lacked the found body at +4, `[EBP-0x6c]`, so the first real grab stopped the game in `hkpMouseSpringAction`'s constructor), `NV_MOVE_MOUSE` (scripted mouse) and the recording lists below. #260 (merged): two Havok multiplies misread from Ghidra's listing (box-box edge test, `ScaleSolverInfo`'s tau). The play branch `claude/build43-movement` (private `wt-build43`) is main + #259 + the handoff; p214 is built from it and lifts the paperweight on Doc's desk with Z (scripted run).

**Physics feel ("not heavy", the maintainer):** the Havok step time (0.016 s, `hkpWorld::stepDeltaTime` 00c91b10), the steps per second (real time on both sides) and gravity (0, 0, -98.1 at world +0x10) match the real game (`lists/physics-step.txt`, private `measure/rec/phys-1` and `port-phys-p213`). The solver tau fix of #260 (0.9 against the exe's 0.45) is the best candidate; ask the maintainer how p214 feels. A Z grab trace list is `lists/grab.txt` (the control check `00a24660(0x1b,1)`, the crosshair ref, the body, its mass against `fGrabMaxWeightWalking` 100). #259 merged (Doc's head, the second-talk crash, Z grab). The maintainer finds the Sunset Sarsaparilla crate "super light" when grabbed and is comparing it in the real game.

**Late 2026-10-29 (the maintainer playing p215/p216; the follow-up PR has these):** (a) the freeze after Doc sits down: `008a5580` (`008a5acf`) gave GetActorValue (00880580, RET 4) both pushed words and the face's `+0x11c` (0064b400, RET 8) one; (b) the brightness slider froze the game: no `IDirect3DDevice9::SetGammaRamp` (slot 21) in the D3D9 layer; (c) the mouse: the real window took cursor pixel deltas (Windows acceleration, cursor free); now raw input counts, the cursor clipped to and hidden in the active window, as for an exclusive foreground DirectInput mouse (not yet felt by the maintainer). `play.sh` now writes `TAG-play-stop.txt` (NV_MOVE_TRACE) so a hand-play stop is reported. With p216 the maintainer played Doc's house to the end of the farewell; then **opening the Pip-Boy stopped the game**: "open function 00ffa010" (an `.rdata` address: a virtual call on the wrong object) inside `Interface::SetStatsMenuVisible` (00704c10 < 00703500 < InterfaceManager::Idle); private `build43-scratch/p216-play-stop.txt` has the last calls. A scripted Tab from the autosave did not stop: it needs the state after the farewell.

**The bug class behind most stops this session: words handed to the wrong call** (0057b7c0's topic, 008a5580's 0, 00cfccb0's five words). The maintainer asked to fix them all. Built: `NV_WORD_CHECK=FILE` (engine-core `abi::extra_words`, called from `entry!`) appends caller, callee, words given and taken once each. First run (p217, the autosave route to the Vit-o-Matic and the Pip-Boy): 514 shapes, private `build43-scratch/measure/wordcheck-autosave.tsv`; filtered with the exe RET table (private `agents/scratch/lead/jmpscan/rets.tsv`, `wordfilter.pl`) into `measure/wordcheck-suspects.txt`: 2 clear suspects (00a71450 > 00aa1a90 gives 5, the exe pops 2; 00770430 > 0093ac20 gives 3, the exe pops 2), 14 callees Ghidra never made (read their RET by hand), and about 490 cdecl callees (check against the callers' pushes in bulk; a small bounded agent can do it with the disassembly dump). Next: run it on the new-game opening and Save 138 too, then fix each against the exe. **First suspect solved (not merged):** 00a71450 > 00aa1a90 is `NiCamera::LoadBinary`'s stream helper: the exe calls the binary stream's own function word (`MOV EAX,[EBX+0x8]; CALL EAX` with the object first, cdecl, 00a71482), the port made a virtual call at +8 and reached the stream's Seek. `NiSwitchNode` (00a945b0) has the same helper. Fixed on the play branch (`e54c6c6c`, WIP): the 8 unit tests of the two files mock the stream with a vtable and must be changed to a function word at +8 (read) and +0xc (write) before it can go to main. Other files' stream helpers already call the word (nilookatinterpolator.rs `stream_transfer`).

**Frame rate:** in Doc's house from the post-race-menu autosave the port runs about 43 fps, capped or with `NV_UNCAP=1` (Save 138's hallway: 230 uncapped), so the frames are slow, not the limiter. `NV_PROFILE` there: 23 ms a frame, 14.7 under 008706b0, about 10 in the shader accumulator and batch renderer (00b66520 > 00b65ae0 > 00b9a120 > 00b6ba20), 5.4 in `ShadowLightShader::SetupGeometryConstants` (00b7dab0), 1.2 in the D3D9 layer's DrawIndexedPrimitive, 0.8 in the model loader's 0044e550; about 1,300 draws a frame. The maintainer wants it smooth (target 200+).

**Other reports from the maintainer, not looked at:** sound sometimes choppy or distorted (probably slow frames starving the output); many settings "don't change what they should" (ask which; some do nothing windowed in the real game too); white orb-like flashes around some objects (renderer; a reflection or specular term?); the console's picked references often show no name.

**Recording once, diffing many times (the maintainer's request):** record the real game with one broad list and compare the port against it, instead of a new narrow real run per question. `research/nv-trace/lists/opening.txt` (new game through the race menu: the player's place every frame, speech, AI packages, head look, blend-outs, shadows, a Doc's-room D3D window after the menu) was recorded as private `build43-scratch/measure/rec/opening-1` (the Doc's-room window used a gate the port never reaches; fixed in the list since: gate only on functions the port enters through `Engine::call`, e.g. 007ac530 not 007ac560). `d3d-docroom-save.txt` (Doc's room from Save 138; after a save load the world-start gate 00dae3a0 never fires, so it gates on LoadingMenu 00788770) was recorded as `docroom-2`. To start a real run from one save only: `nv-trace-launch --prepare-only --no-saves --name X`, copy the save into the run folder's `Documents/My Games/FalloutNV/Saves`, then launch with `--no-saves --name X` (a27's recipe). The launcher and `nv-trace.exe` that read rdump records are a27's builds in `agents/wt-a27/research/nv-trace/target/i686-pc-windows-msvc/release`.

**A solver bug found with the exe's own code:** `nv-call` (a27's or a07's build in private `agents/scratch/a07/nvcall`) runs an exe function on the real CPU on recorded inputs. With the real recording's inputs it reproduced the real output exactly; zeroing parts of the setup split the solver into swing (right) and roll (wrong). The test `doc_s_first_look_turns_the_head_as_in_the_real_game` (runs with `NV_EXE`) keeps that call as a regression.

**Open, in order:**
1. **Ghidra prints `DC C8+i` (`FMUL ST(i),ST(0)`) as "FMUL STi", the same text as `D8 C8+i`.** All 162 distinct such instructions (111 functions; list `agents/scratch/lead/jmpscan/x87-dc-named.tsv`, read from the opcode bytes of the unpacked exe) were checked by agent a31 with a symbolic x87 simulator over the real bytes. Two more were wrong, both physics: `hkpBoxBoxCollisionDetection::isValidEdgeEdge` (00db172f) and `bhkWorld::ScaleSolverInfo`'s tau blend (00c66aa7), fixed in #260. Undecided: some translations do x87 work in plain f32/f64 in the right order (00a6fdb0, 00c6e820, 00c72410, 00d57d50); their last bits may differ. Whether the maintainer's "physics off" report is these two or something else is not known: ask what they see.
2. **The Vit-o-Matic doesn't finish** (character creation). From the post-race-menu autosave (private `build43-scratch/docs-ng`, written by the port: `NV_SAVE_OUT`) the scripted route reaches the machine and spends the points (`grab.sh p212` with `GR_DOCS=...docs-ng`, mouse 5600:7:0:48, W 5700 for 40 and 6000 for 70 frames, E at 6400, Up x5 from 7400, Right x7 from 7800). "Final Results" is button 4 (gamepad A); a mouse click goes to the full-screen hot rect 5, which picks the machine's 3D parts (`DoClick` case 5, NiPick). One scripted click reached `DoClick(5)`, one didn't (timing); whether the pick finds the stamp is not known yet (`lists/vit-click.txt`, gate the pick on the click's `DoClick`). Ask the maintainer whether a hand click finishes it in p212. Inputs act about 200 frames after their scripted number.
3. **Shiny/sharp surfaces:** Doc's room from Save 138 matches the real game draw for draw (6,445 aligned draws); the only render-state difference (ALPHAREF 0x78/0x80 on env-mapped clutter) is draw order from a 0.1-0.2 unit camera height difference: the same values reach the device the same number of times. Ask the maintainer which objects look wrong and where.
4. **Race menu: clean** (p212 against a27's real `rm-closed-2`): all 261 real draws a frame line up; the port's 24 extra draws are Doc into the shadow-depth target (item 1 of 10-28/29: depends on where Doc has walked); the other differences are inert (alpha function with alpha test off, a clip-plane state the real window never set).
5. **Performance:** `NV_UNCAP=1` gives about 230 fps in Doc's house (Save 138) and about 110 fps in the race menu. Race-menu profile (`NV_PROFILE=1`): 17 % under the model loader's queue (00446e40 > 00448cc0 > 0044b8d0), 0044e550 (a locked map lookup) 0.9 ms a frame; check whether the real game calls it as often. With the limiter off, frame-numbered keys run about 4x early: `RM_SHIFT=9000` for rmd.sh.
6. **Doc's walk, side finding:** in the same window the real game evaluates packages for actors 0200B824..27 and never 02010A19..1B; the port the reverse (same plugin). Probably the low-process list's order.
7. Jump-to-slot scan: done (`agents/scratch/lead/jmpscan/STATUS.md`): every direct tail jump forwards what its target pops; nothing new found.
8. Doc's back oval: the clothes' texture (the maintainer, 2026-10-29); not a bug.

**Scripts (private `build43-scratch`):** `grab.sh TAG [secs]` (Save 138 or `GR_DOCS`; `GR_KEYS`, `GR_MOUSE`, `GR_SHOT`, `GR_RUN`), `rmd.sh` (rm.sh with back-buffer frames only), `play.sh TAG [DOCS]` (hand play with the same tables), `ngrm.sh` takes `NG_SHOT`.

### Claude lead, 2026-10-28/29 (read this after 2026-10-29 afternoon, then 2026-10-26/27 below)

**This PR brings the play branch's work since #251 to main** (play commits after 3d9900fd, replayed on main; the race-menu file was merged by hand onto #248's exe-shaped frames). The play builds up to p205 did **not** have #248's changes to the files it and the play branch both touched (they were merged only on main), nor `GetFullPathNameA` in the platform layer. The play worktree (private `wt-build43`) is now reset to this PR's head (the old play head is the local branch `play-before-1029-reset`): p206, built from it, reaches the race menu with the face drawn and lit. Its branch is still named `claude/build43-movement`; after the merge, reset it to main. Tests whose expected values were measured on the reference machine skip on other processors (`RSQRTSS`/`RCPSS` estimates differ by maker; `d3dx::single::reference_estimates`) and without its `RESAMPLEDMO.DLL` build.

**What the play build does now (Windows, `NV_D3D9=real NV_SOUND=out`):** new game, Doc Mitchell's house, the opening speech, the name box, getting up from the bed, the race menu; the port's own saves load back (`NV_SAVE_OUT` copies them to a scratch folder). Sound matches the player's DirectSound bit for bit (both resamplers, the 3D stage, distortion; docs/SOUND.md); the maintainer wants sound exact, never approximate. Frame rate: 1:1 is the game's own limiter (about 133 fps here); `NV_UNCAP=1` (opt-in, not 1:1, `crates/host/src/uncap.rs`) removes it, and the maintainer's goal is 200+ fps in the same scenes. With `NV_D3D9=real` the viewer's own window is hidden and the game frame runs beside Bevy's (`NV_VIEWER_WINDOW=1`, `NV_GAME_FRAME_SYNC=1` give the old behaviour). `GetTickCount` moves in 15.625 ms steps, as measured here.

**New game through the race menu (p208, 2026-10-29):** with scripted keys only, a new game goes through the whole Reflectron (Down/Enter work in it: sex, race, hair, face, the confirmation) and back into Doc's house with the HUD and the "Use WASD to move" tutorial, 29,600 frames without a stop. Route: private `build43-scratch/ngrm.sh TAG [secs]` with `NG_MORE` = Down, Down, Enter every 900 frames from frame 10000 (`seq 10000 900 30000 | awk '{printf "%s%d:d0:3,%d:d0:3,%d:1c:3",(NR>1?",":""),$1,$1+300,$1+600}'`). Enter alone does nothing on the first page. Open: a light patch on Doc's back in the last frame (`frames-p208k-ng/frame-029633-*.png`) looks like a hole; the maintainer has reported see-through skin before. Check it against the real game at the same moment.

**Fixes this round (each against the exe):** the lip-sync and render-pass freezes (00553bb0's 0x24 local, 00518f00's 0x18 DoorLock), the Havok GSK manifold (00dae3a0) and capsule-triangle agent frame, the bed freeze (006f9850 returns its EAX), the save load (004c77d3's slot word), stretched heads (hkaPose local-from-model), speech pacing (00935f00 callback), BSAudio message 56's vector address, process-list slots, GetLocalTime, the race-menu frustum near/far (007fd194, 007fdd9b: left at 0 the far plane culled every face light), and the device passed once as the COM `this` (00876924, 0086b3a6).

**Diagnostics (all env-gated, not exe behaviour):** `NV_HKMEM_CHECK`, `NV_SPEECH_CHECK`, `NV_PASS_CHECK`, `NV_LIGHT_CHECK`, `NV_QUEUE_CHECK`, `NV_FACE_CHECK`, `NV_HEAD_CHECK`, `NV_BLOCK_CHECK` (guard bytes after heap blocks and `with_stack` locals: the fastest way to find an undersized local), `NV_AUDIO_CHECK`, `NV_WATCH=ADDR,LEN` or `*PTR+OFF,LEN` (who writes a range of game memory, with the call chain), `NV_PROFILE` (+`_THREADS`), `NV_SHOT=DIR[,ms]` (BMPs of the real back buffer: use these, not screen copies, for screenshots; a screen copy once caught the old viewer window), `NV_TRACE_OUT`/`NV_TRACE_LIST` (the port's call trace).

**research/nv-trace (new):** records the real FalloutNV.exe (a recorder DLL, DirectInput key scripts, the folder guard, PrintWindow shots checked by PID) and the port, then diffs the calls (`nv-trace diff`) or the Direct3D 9 calls draw by draw (`nv-trace d3d REAL PORT`). Never write into the player's folders; the launcher uses scratch INI copies.

**Next, in order:**
1. **Race menu: matches the real game by eye now (p207).** The face lights sat 80 units off in x: `008761e0` (the rendered-menu pass) set the main scene node's offset (`[011f91c8]+0x1e4`, which the shaders subtract from light positions) to -(the menu screen's position) every frame. In the exe that call (00876371) is inside the Pip-Boy test (00876295 jumps to 00876376); the port had moved it out. Found with a27's console-closed D3D diff (private `build43-scratch/measure/rec/rm-closed-2/diff-vs-p206.txt`), a27's real-game recording of the offset (0 on every race-menu frame; `rm-offset-*`) and a22's write watch (`NV_WATCH=*011f91c8+1e4,c`). The head gets the same 6 passes as the real game, with the same light radius and colours. Left from the diff (recheck on p207):
   - **Extra 1024x1024 head pass = a shadow depth map** (a27, `rm-shadows-*`, lists on `a27-trace` 772ae2b6): `ShadowSceneLight::MTRenderFrustumDepthMap` (00b9f780) from `ShadowSceneNode::UpdateDynamicShadows` (00b5b880) on the world node. Of its 4 shadow lights, the third one's NiLight flags (+0x30) are 0x0008000f in the real game (bit 0, app-culled: no depth map) and 0x0008000e in the port. The real flag is set in Doc's house before the menu opens (frames 9591..9659), on a moving light (probably an actor's); its position differs between the two runs. a22 (round 14): the light (real NiLight 33029100, no name, no parent, moving) goes app-culled after a ~68-frame gap in the list, which fits `fShadowFadeTime` 1.0: its actor stops being picked as a shadow caster. 00b5b060 (from 00871290 at 008713a7) culls every shadow light each frame, then 00871290's walk un-culls the picked ones (0087178e) or culls them (0087191d, 00871930); 00b9dfc0 culls when the map pool is empty (00b9e04b). The port's 00871290, 00b9dfc0 and 00b6d3f0 match the exe. An actor is picked (00b5cbd0) only if it is in the high process list and 009611e0 passes, 0087f200 passes, virtual 0x214 is not 4 (sitting?), it is not refractive, and its bound radius is under 3x its BBX extent (00871514..00871627). **a27 ran that list on both sides** (`rm-casters-1`, `port-rm-casters-p208.nvtrace`; decoded in `agents/scratch/a27/casters-*.txt`): in that pair the real game also keeps the walking actor (very likely Doc) as its one caster with the light on (0x0008000e) through the race menu, as the port does. So the extra pass is probably not a port bug: it depends on where Doc has walked when the menu opens, which differs by run (the console pauses him at a different point of his walk in each game; item 3). Next: record `MTRenderFrustumDepthMap` in a few real runs to see how often the real menu has the pass. SetAppCulled (00450f90) filled the list's cap early; gate it with after=007cb7d0 next time.
   - **The plate and the Vit-o-Matic without their point lights: fixed by #254** (a22, rounds 16-18). a27's diff was against p206, before #254. BoundWithinLightRadius (00b9dae0) adds the scene node's offset (`[011f91c8]+0x1e4`) to every light position, so the -80 offset also put the plate's lights out of range. In p208 the plate gets its 3 lights, its pass list matches the real game, and the draws use the real shaders (0xea: 478b/514e, 0x132: 2387/e52f, 0x14a: 3514/c331, 0x252: 5558/65ad; Vit-o-Matic 0xc9: 83da/a89b). Not one 9835/0d7d/be8f in 40,000 shader sets (a22q; count with private `agents/scratch/a22/psnames.pl`). The shader is fixed per pass object (+0x5c/+0x44), not picked per draw. A fresh a27 D3D diff against p208 or later should come out clean.
   - SetIndices/SetStreamSource with the device pushed twice: probably gone since e1d1de05. a22 found no such call in the p206c recording (13,478 calls of each, the right word counts) and no other call site in the exe or the port; a fresh diff should confirm it.
   - the maintainer's earlier reports (see-through skin, shininess) were made before this fix: check them with the maintainer on p207.

2. **Post-load sound stop: fixed** (a22, 2026-10-29). `hkpSimulation::integrateInternal` (00cf8da0) had a 0xa0-byte frame in the port against the exe's 0xb0 (`SUB ESP,0xa4` and three pushes, 00cf8dab..00cf8db3); the solver writes words +0x6c..+0x77 of the constraint query at frame+0x30 (00d88f94..00d88fa9), so +0x70..+0x77 landed in the next heap block. When that block was a sound's file object its vtable word became an entity index (0xb), and the sound's delete faulted in 00af1cc0. Found with `NV_AUDIO_CHECK`'s reader and file watch (and its `=fault` mode). Before the fix 3 of 4 traced runs stopped; after it 0 of 5.
3. **Doc's walk: no port bug found** (a27 and a22, 2026-10-29). From the Save 138 load both Docs walk the same route (every port point within 1 unit of the real path; the port takes about twice the frames, its frames being longer). An apparent stop in the port was the test route: rm.sh opens the console at frame 4500 (2700 + RM_SHIFT 1800), which pauses the game, and the port's Doc had reached a different point of his walk by then. With the load keys only (`ngdbg2.sh` with `NG_BASE_KEYS` and no console keys) the port's Doc walks on and turns back as in the real game; his turn point varies by run (1826, 1962, 1999 against the real 1830.7), not looked into yet. For walk comparisons, never use the console route. Note: the port calls ActorMover::ClearDataForNewPathingRequest (009db950) as a plain Rust call from 009db0e5 and 009dceb1, so nv-trace does not see it; SetPathingFailed (009de490) goes through Engine::call. Side finding, still open: at frame 116 the port evaluates and sets packages for three far-away actors (refs 02010a19..1b, base 02010b60, package form 00074fe6) that the real game does not touch then.
4. The `SStartingCell` INI route stops at frame 0 (00a0cb90).
5. 004b0680 writes past a 0xc block (`NV_BLOCK_CHECK`; probably harmless, unconfirmed).
6. **Jump-to-slot forwarders that drop words** (the bug class behind the book-grab stop, #258): a function that only jumps to a virtual slot (`MOV EAX,[ECX+n]; ... JMP EAX`) hands the slot its own stack words, so the port must forward as many as the target's `RET n`. #258 fixes 00cfccb0 and five more a scan found (00d42a00/40/50/60, 00d5ee60). A second pass checked the rest of the short single-call candidates (00c91270, 00ca8920, 00cac110, 00cd06d0, 00cd06e0, 00a7ff60, 00a99070, 00c21b90, 00a7f560, 00a7f5f0, 00a87790, 00d5e180, 00dd5e60, 00d5d720, and the NvAPI `JMP [cell]` thunks 009fead0..009feae8, 009febd8, which are cdecl): all fine. Not covered: tail jumps the port translated as longer bodies.

**Machinery (private):** `build43-scratch/brf2.sh TAG` builds the play viewer (`VB_LLD=1` links with rust-lld), `RM_SHIFT=1800 rm.sh TAG [secs]` runs the save-138 route into the race menu (scratch Documents in `docs-save138`; since p205 the main menu's items appear about 45 s in, so the keys need the shift; without it CONTINUE is pressed too early and `showracemenu` runs at the main menu with no player, which faults in 00559450; the route closes the console after `showracemenu`, since the menu draws its face only with the console closed), `ngdbg2.sh` runs under cdb without the debug heap. `crates/play-patch` is the fast loop for a single fix (host + link only). Every compile goes through `phase0/bin/heavy2.sh`.

### Claude lead, 2026-10-26/27 (read this after 2026-10-28/29, then 2026-10-23/24 below)

**Update, 2026-10-27 afternoon (read first):** this PR collects the fixes found by running the game's own code through the new game into Doc Mitchell's house: words pushed for the wrong call (SetCurrentWeapon in 0094e1d0, the obstacle walks, MoveTo, NavMesh::GetCenter's argument order, a vcall sweep), 65 Havok functions that return the out pointer or `this` their callers read through EAX (hkpEntity::isActive crashed the cell load), stack locals moved from heap blocks into exe-shaped frames (about 400 functions, with `_alloca16` tables), dropped return values, and clippy cleanups. On the play branch (not in this PR yet): the game's own renderer draws through the real Direct3D 9 in its own window (ADR-0008, `NV_D3D9=real`; loading screen, intro, main menu, new-game loading screen), sound to the real device (`NV_SOUND=out`), the thread host's time slice and real-time sleeps, input from the game window, D3DX maths bit-exact and the shader compiler through the player's DLL, and the functions Ghidra never created that the render path reaches. The next play-to-main merge brings those.

**This PR merges the play branch into main.** Main now has everything the play branch had (the CRT and BS* platform code, the platform layers for Windows, Direct3D 9, DirectInput, DirectSound, DirectShow, Bink, mmio, the thread host, the fast dispatch, the static initializers), plus this session's fixes. Coverage: 99.8 % of the exe's functions covered (47,591 by hand, 21,726 through the start-up and shut-down tables); every function of the six system groups and of the BS* code is translated. Fixes found by running the game's own code (each against the exe): 00c68050 threadInit's third word, the script MoveTo queueing (005ccf50/005cd0e0), 0096a540 clearing the map's own pointer, the cell obstacle walks, the bhkWorld step accumulator retranslated x87 operation by operation (a04; at 24-bit precision 0.05 s of 1/60 steps is 3 steps, not 2), D3DXMatrixRotationYawPitchRoll bit-exact against the player's DLL, SetUnhandledExceptionFilter, GetStringTypeW (a measured CT_CTYPE1 table), GetAsyncKeyState, the Steam no-client answers, and more of the agents' scans (words given to the wrong call, stack locals as heap blocks, dropped return values). The oracle now counts a vector where the exe calls a Windows import as inconclusive (ExeImport): those inputs reach the C runtime's errno path, which nv-call can't run.

**Play build (branch `claude/build43-movement`, private `wt-build43`):** the game's own code now runs its whole start-up, its main loop and the main menu at about 60 frames a second, starts a new game from injected keys (Down, Down, Enter on NEW, Enter on YES; host `NV_MOVE_KEYS`), plays the intro and runs on for more than 16,000 frames without a stop. Not yet seen in Doc's house: the next step is to find where the game is after the intro (host `NV_MOVE_WATCH` prints chosen words, `NV_MOVE_TRACE_FRAMES` writes a frame range's calls). **The play loop is fast now:** fixes go into `crates/play-patch` (registered over the engine's entries at start-up), a patch rebuild takes about 20 s and a run under a minute; the private compile queue starts a play build at once when nothing else runs. The patches move to their home files with each gated collection.

**The maintainer's decision (2026-10-27): renderer before more movement work, by forwarding.** On Windows the Direct3D 9 layer forwards the game's calls to the real d3d9.dll (ADR-0008, on a slot branch, not merged yet) so the game's own menu, movie and world appear in a real window; sound goes to the real audio device through the mixer (`NV_SOUND=out`). A cross-platform renderer can follow. The mixer's resampler, 3D law and distortion are approximate for now and listed open in docs/SOUND.md (the maintainer has not decided between exact and approximate).

### Claude lead, 2026-10-23/24 (read this after 2026-10-26/27, then 2026-10-22/23 below)

**Update, 2026-10-24 evening (read first, then the morning update):** #246 carries the LoadingMenu semaphore fix (0078d570) and the word-count scan: about 30 call sites where words pushed for one call had been given to a getter or fixed-RET call that runs between the pushes and the call they belong to (actor, AI, process and form code), each checked against the callee's RET. What the scan left unchecked (about 200 flagged rows, vcalls on receivers outside the actor and process families) is listed in the private `agents/scratch/a02/argscan/STATUS.md`, with its scripts. Coverage numbers are unchanged. **Dispatch is fast now** on the play branch (a fast hasher for addresses, a direct-mapped cache, no per-call allocation: about 4-5x), so the full plugin load finishes in about 143 s; it is not on the lead yet (port a09's a2bb131e and ed0d1e5b through the gate). **Play build:** past the plugin load into the menus. Fixed on the way: the uptime clock, a word given to the wrong call, Tile::ReadFile's word, StartMenu's two 4-value options, HeapAlloc on the CRT heap, BSNetworkSystemUtility. **Static initializers:** fonts 2-7 were missing because the start-up initializer runner skipped untranslated table entries silently; it now lists them in strict runs (`static_init::skips` lists them without a run), and all 269 skipped entries are translated (a03 on its branch agent-03, picked onto the play branch; 41 partial for their exception frame). The last stop (p132) was an access violation about 31 s in: Host::start ran the start-up stages on a local Engine and then moved it into Host while the game-thread coroutines kept its old address. It is fixed on the play branch (c35ca07b boxes Host::engine, head 13232ef7) but not built or run yet; the next command is at the top of the private `phase0/build43-status.md` (build play-fast as p133, run doc.sh p133 with NV_DOC_TIMEOUT=300).

**Update, 2026-10-24 morning (read first):** #244 merged (46,512 translated, 98.2 % covered; every function of all six system groups translated). The play build now runs past Main::InitTES, the path manager, the IO and music threads (critical sections are real locks in the thread host now; DirectShow's filter graph for the music is our own layer, MP3 through `crates/mp3` matched to Windows' decoder on 99.97 % of samples), into TESDataHandler's full plugin load. **The next play blocker is speed, not a bug:** the full load does not finish in 25 minutes because the game's O(n²) dialogue-topic sort (InsertInfo 00619820, 0061a380/0061a5a0, compare 0061a450) makes hundreds of millions of calls to one-line getters, and each `Engine::call` costs about 150 ns (hash lookup plus per-call allocations). Seven getters are 199 M of the first 300 M calls (private profile `build43-scratch/p121-profile.tsv`). Make dispatch cheap (a dense address table, no per-call `Vec`) without changing behaviour; the `play-fast` profile (opt-level 3) helps only by a constant factor. Sound (the maintainer said go): DirectSound, the music graph, `crates/vorbis` (exact against the player's libvorbisfile at 24-bit x87 precision on all 63,575 voice files; 53-bit not matched yet, the maintainer has since allowed reading the DLL to get its instruction order) and an exact 2D mixer with WASAPI output (off by default). Open for the maintainer: the mixer's resampler and 3D law (keep measuring until exact, or ship marked approximate); see `docs/SOUND.md` on the play branch. a01's sub-object scan left `0078d570`'s semaphore fix (private `agents/scratch/a01/scan4-status.md`).

**Merged:** #240, #241, #242 and #243 (the last written batches; 46,435 translated, 98.1 % covered once #243 lands), contributor PRs #231 (oracle: recursion, hidden inputs), #108 (native macOS and Metal) and #50 (Vance's Gun route). Issues #233-#238 (the oracle's store tier, 148 functions) are fixed and closed.

**Translation:** every written batch n001-n089 is done. By system, every function of physics, graphics, animation and faces, and audio is translated; gameplay 99.7 %. "Translated" includes partial functions, nearly all of which lack only the C++ exception frame (FS:[0]) or the /GS cookie check. What is left on the lead: the C runtime, compiler start-up/exit and BS* platform code (the play branch's), and the open items below.

**Bug classes found by the play build and scanned for on the whole lead:** functions in vtables translated without `this` (a08, 35 fixed); dropped return values of a final call (a05 earlier; a09 second scan, 11 fixed; 1,118 vtable-only candidates left undecided because slot-offset matching can't tell classes apart: needs the receiver's class at each call site); a vtable's address read as memory (two fixed, scan found no more); an extra `this` passed to the CRT's stdcall array iterators (3 fixed); functions that leave out trailing unread stack words (15 fixed). The review list (private `phase0/lists/fix/suspects.txt`) is worked through; each line fixed or explained.

**Play build (branch `claude/build43-movement`, private `wt-build43`, local, not pushed):** start-up now runs through the renderer, face generation, the movie player (the intro is skipped exactly as the real game skips it: `sIntroSequence` is empty in the player's INI), the game data files and into the player's creation and TESDataHandler's form lists. New platform layers on the branch, each matched against the player's DLL or Windows behaviour: Bink (all nine imports; the intro decodes 8,692/8,692 frames bit-identical to binkw32.dll; movie audio decode matches about two thirds of frames, the rest one step off, cause not found: docs/MOVIES.md), WinMM mmio, DirectSound (device answers captured into a git-ignored sound table, `scripts/dsound-capture`; no audible output yet: the mixer, Vorbis, DirectShow MP3 and Bink sound wait for the maintainer, survey in the private `agents/scratch/a11/audio-survey.md`), FindFirstFileA through the real file system on Windows (read only; short-name matching can't be modelled), one game clock (`Engine::clock_now_us/ms`) for every time import, and a large CRT batch (printf's own output path, setlocale, atexit, memcpy family with both paths, strtod/__fltin2, locale and multibyte code pages). The handover with the exact last stop is the top of the private `phase0/build43-status.md`.

**Machinery changes (private `phase0/bin`):** the gate builds incrementally, builds the oracle at opt-level 1, runs clippy with `--keep-going`, and leaves 00ba5060 out of the oracle (an 11-entry stack table with an unchecked index); `oracle-only.sh` reruns just the oracle step. The compile queue has a lane for hygiene-only runs, starts a big job after 5 idle minutes, starts nothing beside a play build, and runs play builds with one compile job (two overlapping crates ran the machine out of memory). The C: drive filled up once (2026-10-24 07:57 UTC): clean slot and stand-in targets after collecting.

**Contributor PRs:** #232 (oracle store tier) approved, waiting on the contributor's rebase; #29, #87, #91 changes requested; #62 asked to rebase after #108.

**Open:** the 1,118 vtable-only dropped-return candidates; the x87 status flags behind _statfp/_clrfp; the partial functions' exception frames (needs a design decision on modelling SEH); audible sound (the maintainer's call: after the renderer, or now).

### Claude lead, 2026-10-22/23 (read this after 2026-10-23/24, then 2026-10-20 evening below)

**Merged:** PR #229 (005e0830's slot result), PR #230 (recorded batches b054-b075, n001-n003, the last nixenonrenderer functions: 37,226 translated, 84.9% covered) and PR #239 (the animation, renderer, scene graph and AI batches n004-n029; 40,291 translated, 89.3% covered).

**Recorded play queue:** b001-b075 are all done. What the recorded game ran and is still untranslated is CRT and platform start-up code, which the play build picks up as it reaches it.

**Next queue (private `phase0/lists/next/`):** n001-n035, whole units, about 110 functions each, in this order: NiAnimation, NiXenonRenderer, BSShader, NiMain, fallout/ai, pathfinding, NiCollision. n001-n029 are done; n030-n035 are written and not assigned. `assigned.txt` says what is done and where. The rest of the exe after these: about 10,000 open functions; the biggest open groups are fallout shared, fallout/interface, BSShader, fallout/ai and (unplaced) (counts in docs/SYSTEMS.md).

**Play build (branch `claude/build43-movement`, private worktree `wt-build43`, local, not pushed):** the game's own `main` now runs its start-up through the INI settings, the CRT and archives, Main::Main, Renderer::Init (our own Direct3D 9 device, textures through D3DX and a capture cache, render states, shaders, the camera set-up, nvapi from the machine table) and the whole of InitSceneGraph (every shader built), into the next stage (00702250). No game frame has run yet, so there are no movement numbers and no play copy. The handover with the exact last stop and commands is the top of the private `phase0/build43-status.md`. Decisions made on the way (keep to them):
- atiadlxx.dll is reached only with bEnableEyefinity; not on this machine.
- nvapi.dll: answers captured into the machine table (`scripts/nvapi-capture`).
- D3DX: the maths are our own, measured bit for bit against the player's d3dx9_38.dll (`scripts/d3dx-measure`); texture creation copies the stored levels itself and takes everything D3DX makes (sub-block DXT levels, generated mips, format conversions) from a git-ignored capture cache (`scripts/d3dx-texture-capture`; each player runs it once); the shader compiler/assembler are to be captured the same way.
- CPUID: the player's leaf 0/1 answers are captured into the machine table, so the CRT's SSE2 maths switch is set as in the real game.
- The CRT maths (sin, cos, tan, pow, exp, log, asin, acos, atan, atan2, floor) are the exe's own x87 and SSE2 code now, verified against the exe run natively; no host-maths stand-ins remain.
- _msize/realloc are platform, like malloc (our allocator owns the blocks), with every edge read from the exe.
- The machine table and the D3DX cache describe the player's machine and files: git-ignored, never committed.

**Build speed:** the play loop builds the viewer with `--profile play` (opt-level 1 for the engine crates, incremental): a platform edit rebuilds in about 65 s instead of 28 min. Packaging the play copy stays on `--release`. The private queue (`phase0/bin/heavy2.sh`) puts the play build and the slots listed in `phase0/heavy.priority` first.

**Machine:** the memory watchdog is `phase0/bin/watchdog3.ps1` (it spares the biggest rustc). Never read its log with `tail -F`: a reader holding the file open made its writes fail while builds were still stopped, which looked like silent rustc crashes (exit 0xffffffff) for most of 2026-10-22. Agents test engine-fallout files only in stand-in crates; the lead runs engine-fallout once per collection (its lib-test build needs about 15 GB).

**Review notes for PR #239:** see the PR body. Open suspects (private `phase0/lists/fix/suspects.txt`): argument counts of 00a27ee0 and fn_00b984d0, fn_00a68d40's missing this, 00a291e0's guessed shape, the 01012060 constant written as a literal, the printf float stand-in in the play branch's crt.rs, and an unused computation left out of 00a58550.

**Contributor PRs:** #29, #50, #87, #91 and #108 (abusager13) were updated on 2026-10-21. Reviews are drafted in the private `agents/scratch/lead/review-*.md` (not posted: the maintainer said to leave the PRs for now). Their CI runs wait for the maintainer's approval (fork PRs).

### Claude lead, 2026-10-20 evening (read this after 2026-10-22/23, then the night section below)

**Merged today:** PR #226 (recorded batches, renderer start-up, CPU-exact x87
helpers) and PR #227 (recorded batches b008-b046 and Renderer::Init's layers
b000t-b000y). After #227: 33,973 functions translated by hand, 80.2 % of the
exe covered. This PR adds batches b047-b053: 34,580 translated, 81.0 % covered. Per system: physics about 98 %,
gameplay 78 %, audio 72 %, animation and faces 38 %, graphics 39 %.

**Recorded play queue:** batches b001-b053 are done.
Extra batches
b054-b075 are written and not assigned (private `phase0/lists/play/`,
`assigned.txt` says what is done). Each agent report lists the untranslated
callees the recorded game ran; those are the next batches after b075.

**Left in nixenonrenderer.rs (large x87 / D3D functions, not started):**
00e69ea0, 00e6ab40, 00e6c780 (Do_SetCameraData), 00e6cf40, 00e70140,
00e71fe0 (PackGeometryBuffer), 00e74120.

**Play build (branch `claude/build43-movement`, local in the private worktree
`wt-build43`, head 36057785, not pushed):** the game's own `main` (0086a850)
now runs as the main game thread, driven frame by frame (a `PeekMessageA` on an
empty queue ends a frame). Platform layers on the branch: window and message
queue, files, folders, INI reads, user name, Steam answering false, DirectInput8,
and the start of the Direct3D 9 layer (ADR-0007; IDirect3D9 answers from a
machine table that `scripts/d3d9-capture` writes; that table describes the
player's hardware, so it is git-ignored and must never be committed). **Not yet
run in Doc's house since the last fixes:** the last run stopped at
GetPrivateProfileIntA in 00876ad0, which is now provided. No game frame has run
yet, so there are no movement numbers to compare (walk about 85, run about 340,
jump apex about 60) and no play copy. Next, in order: build the viewer, run
Doc's house strict and with the probe, translate each stop; on `main`'s path
come the INI reads (004de490, 0045d180), Renderer::Init (004da670) and
IDirect3D9::CreateDevice with the whole IDirect3DDevice9 (not written; it must
call `platform::direct3d_device_created` for the x87 precision switch). Exact
commands: private `phase0/build43-status.md`, top section. On the next merge of
the lead into the branch, 00b4e270 exists on both: keep the lead's.

**Review notes for this and the last PR:** eight shader pass-setup functions in
`unplaced/run_00c048b0.rs` (00c0d200, 00c0e630, 00c0fdc0, 00c11980, 00c124c0,
00c16400, 00c17510, 00c072c0) and the five HLSL loaders' source tables in
`bsshader/run_00bcc570.rs` were written out by scripts from the disassembly
(call counts checked against the asm). The functions 00b43db0-00b49e20 look
like a bundled stream decompressor (version text "1.2.1"); they are translated
from their bodies, and whether the ledger should call them `library` is open.
`00a29680` is called in several places but has no `entry!`.

**Process lessons:** batches handed out at the same time overlapped on the
renderer files (a08's priority layers against a02, a07 and a08's recorded
batches). Each overlap was merged in the agent's own slot (keep one translation
per address, keep both sides' tests). Check overlaps against the lead's
unpushed commits, not only the running slots' units. The compile queue is the
bottleneck: one big job at a time, about 15-20 minutes per agent test run and
about 80 minutes for the gate, so plan batches around it.

### Claude lead, 2026-10-20 night (read this next, then 2026-10-19 below)

**The real game was recorded.** The maintainer played a new game with a
recorder loaded (private `nv-cover`, an INT3 on the first byte of every one of
the 69,474 ledger functions, first hit only; private script
`phase0/bin/record-coverage.ps1`, list built by `phase0/bin/mkcover.pl`). It ran
cleanly for about 8 minutes. 45,329 functions ran; 8,548 of them were
untranslated. Leaving out the renderer, menus, audio, faces and sky, about 4,900
of those are the play build's missing code. They are handed out in batches of
about 120 in the order the game first ran them (private
`phase0/lists/play/b001..b044`, `assigned.txt`); the tables are in
`captures/coverage/` (`ran.tsv`: address, size, name, subsystem, unit, ledger
status, first-run order). Batches b001-b004 and b006 are in this PR.

**Play build priority batches (b000, b000b-b000g):** the callees the start-up
stops on. Leaving the renderer out was wrong for start-up: Main::Main and
InitSceneGraph build the renderer's managers (BSShaderManager,
BSShaderAccumulator, BSBatchRenderer, the texture manager, ShadowSceneNode), so
those were batched as they were found. The play-build branch now runs NiInit to
the end, then Main::Main, InitSceneGraph and InitTES's head; it stops in the
input object's DirectInput8 set-up (a DirectInput8 platform layer is being built
on the branch, like the Direct3D 9 one).

**Ledger statuses that hide game code:** functions marked `platform` (162 ran,
mostly BSSystem, BSSystemUtilities, BSMovie) or `library` (CRT, 596 ran) are not
in the play queue. Some are game code: the tasklet manager (BSSystem) was
corrected on the play-build branch (ledger `GAME_FUNCS` override). The CRT's
per-thread data chain (`_LocaleUpdate`, `__getptd`) is being translated because
strtol needs it. Deciding whether those whole categories need review is the
maintainer's call.

**PR #200 merged** (contributor's oracle: pointer arguments, per-vector time
limit, RustStrayRead). Two lead fixes went into it: the engine side runs on a
256 MB stack, and an exe stack overflow counts as stuck. Its full run found two
real translation bugs, 00b14230 (an f32 exponent) and 00df2510 (NaN bits), which
are queued (private `phase0/lists/fix`). The gate now leaves out those two plus
00dd8e00 (hkSystemClock, reads the clock), 00e2f8c0/00e2f9f0 (the same divide
fault, worded differently) and 00b15110/00e3ec00 (DataFlowMiss: the list misses
a recursive pointer walk). The questions are with the contributor on #200.

**Machine:** five translators plus the play-build agent ran overnight with no
crash under the tightened compile queue (one big compile; one small beside it
only with 16 GB free) and the watchdog.

### Claude lead, 2026-10-19

**Havok is translated:** all 9,011 Havok functions on the full ledger list
(Ghidra-gap functions included). 739 of them keep a `Translation incomplete`
marker, almost all for C++ exception frames (FS:[0]) or the /GS cookie check;
every branch of their bodies is in. This PR carries the last ones: 00c95a80
(hkpWorld constructor), 00d1a330, 00d28a30, 00d29bf0, 00d6f460 (all 29 atom
types), 00d72190, 00d8d030 (hkSolveConstraints, block state machine), 00daf8e0,
00def570 and the last ten smaller ones, plus the game's Havok glue (120,
fallout shared/havok) and 009070a0 (LowProcess, a play-build blocker).
Engine total 28,511 translated (41.0 % of 69,475), 72.3 % covered.

**Notes on these batches:** 00def570 and its sister 00d9f6f0 panic where the
exe spins forever (commands 4, 0x1c, above 0x1f). 00ca35f5 is a tail of
getNextKey (00ca35d0) entered by a jump, registered with the registers it
reads. 00cf9c60's writes to its own argument slots (scratch nobody reads) are
not reproduced. The solver's `SolverMachine` (hkpsolver.rs) does x87 maths in
plain f64: identical at the 53-bit precision the Havok thread runs at, except
for the exponent range; revisit with the x87 helpers.

**Machine:** the PC crashed twice on 2026-10-19 with ten agents running: a
freeze, then a Windows bug check (0xC2 BAD_POOL_CALLER in audiodg.exe, an
audio driver; WinDbg read the dump). Since then at most four agents run, the
compile queue lets only one small compile beside a big one (16 GB free), the
gate runs with `CARGO_BUILD_JOBS=1`, and a memory watchdog logs low memory.

**Play build (branch `claude/build43-movement`, not pushed):** the Havok step
runs on AI linear task thread 2 (committed). NiInit now gets past the IO
thread and through NiMainSDM::Init. The player stage stops in
MobileObject::InitHavok on the TES object at [011dea10]: TES::TES needs
00c44980, 004ba280, 0084fc70, 00856380, 00c3da50 (being translated in slots
a01 and a02).

**Pending outside this PR:** a07's NiCollision batch (75 of 120, commit
99ed8c98, parked: not Havok or the play build); a06's nv-pure `--fpcw` oracle
option (3063abd3, branch parked/a06-fpcw-oracle, to land after #200); PR #200
merged with main and its list regenerated in wt-pr200 (uncommitted), oracle
check still to run.

**Suggested next step:** record every function the real game runs while
walking in Doc Mitchell's house (a tracer on the real exe, built on the
nv-probe hooks) and compare it with the ledger, so the play build's missing
functions can be handed out at once instead of one stop at a time.

### Claude lead, 2026-10-18 evening

Stopped at 93 % weekly usage (95-96 % rule). Maintainer priority: **finish
Havok, then only the next play build (the game's own physics and movement in
the game), nothing else.**

**Merged this session:** #218, #219, #220, #221 (all of the then-listed Havok,
SpeedTree complete, start-up and shut-down leftovers, the renderer scaffolded
under ADR-0007, the engine split into crates). The PR carrying this note adds
the batches collected after #221 (see below).

**Decisions (maintainer, this session):**
- **No bytes copied from the exe in the repo.** The Steam exe's `.text` is
  encrypted, so code-section tables are not read from the player's exe either:
  compiler switch tables are translated as `match` arms (done for 23 functions).
- **The engine is split** into `engine-core`, `engine-gamebryo`, `engine-bs`,
  `engine-havok`, `engine-speedtree`, `engine-fallout` behind the `engine`
  facade (docs/ENGINE_CRATE.md, "The engine crates"). Same registered addresses
  and tests; a Havok test build takes 3 minutes instead of 12.
- **Threads:** the approved thread host runs each game thread's `ThreadProc`
  (BSThread start function 00aa64d0) as a coroutine on the main thread (an OS
  thread used strictly as a coroutine is allowed), with per-thread TLS and x87
  control word, waits that yield, fixed switch points. Real threads can replace
  it later.

**Findings that change numbers:**
- **Ghidra never created 3,216 functions** the exe runs (reached only from
  vtables, data or tail jumps). The ledger now reads
  `research/engine-map/ghidra_gaps.tsv`: 69,475 functions. Havok was about 91 %
  done, not 100 %, when found; most of its gap functions are now translated
  (six batches) but not all (below). The README line "all of Havok is
  translated" on main is wrong until this PR's README change lands.
- **The ledger counted markers in the old viewer crates** (`crates/physics`,
  `crates/world`) as translated: 170 functions. Fixed: only engine crates count;
  those are `traced` now.
- **x87 precision, measured in the real game (a06):** main thread 0x027F until
  `CreateDevice` (00e72e60), then 0x007F (24-bit) for the rest of the run
  (player mover, character controller); the Havok step (`bhkWorld::Update`,
  `hkpWorld::stepDeltaTime`) runs every frame on AI linear task thread 2
  (008c7f50) at 0x027F; MXCSR 0x1F80 everywhere. The per-thread control word
  model is in (engine-core, switched at device creation, tests against the CPU);
  translations do not use it yet (the movement path's x87 maths must go through
  the helpers).
- **Unreferenced code (static pass):** of 1,233 blocks the gap sweep set aside,
  984 look dead (almost all Havok SDK), 34 are referenced (33 only from C++
  exception cleanup), the rest belong to known functions. The runtime coverage
  pass crashed the game early and is not done (private phase0/gap-check.tsv).

**Play build (Build 43, branch `claude/build43-movement`, worktree wt-build43,
not pushed):** no frame runs on the game's code yet. The host runs the game's
own `NiInit`. The thread host is built and tested on the branch (36674e94);
moving the Havok step onto AI thread 2 is written but uncommitted and untested
(see build43-status.md). Start-up needs `NiInit` also needs four registered start-up functions
(00a2d8a0, 00c1aa00, 00e98ff0, 00e91dc0) and their callees. The status file is
private phase0/build43-status.md; the missing list phase0/physics-missing.tsv;
the plan phase0/physics-wiring-plan.md. A replay test of the real game's
captured movement is ready for when frames run. 00aa1920 exists on both the
lead and the wiring branch: keep one when merging.

**Collected at the very end (in this PR):** a01 83 Havok gap functions, a09 42
functions only the old viewer crates had, a04 the last 16 of the game's Havok
glue, a02 NiInit's four start-up functions with their callees (28). **Havok
still open:** the big seven (00c95a80, 00d1a330, 00d28a30, 00d29bf0, 00d72190,
00def570, 00d8d030), 00daf8e0, and a09's 23 extras plus 4 fallout listeners
(agents/scratch/a09/STATUS.md).

**Left uncommitted in agent worktrees (each has a STATUS.md in
agents/scratch/aNN):** a01 (83 of 102 Havok gap functions, stand-in tested), a03
(the game's Havok glue, ~40 of 120), a07 (NiCollision, ~37 of 120, continue at
00e9e860), a09 (functions only the old viewer crates had), a06 (the nv-pure
24-bit oracle option), a02 (NiInit's four registered start-up functions and
callees, stand-in tested), a04 (the last 16 functions of the game's Havok glue);
and on the wiring branch, the world step moved onto AI thread 2 (untested). Resume each with "your worktree has
partial work: review, finish, test once, commit".

**Machinery changes:**
- Never chain `cd` in shell commands (the maintainer approves each by hand):
  absolute paths, `git -C`, `cargo --manifest-path`. Task templates say so.
- `heavy.sh` forwards to `heavy2.sh`: one big slot (engine-fallout, full
  suite, gate, viewer, host) plus two small slots (other part crates), with
  free-memory checks read from /proc/meminfo (spawning PowerShell every 10 s
  from each waiter, plus ~230 orphaned shell loops, made every process start
  take minutes on 2026-10-18).
- Agents check their code in a stand-in crate first, then make one real run;
  focused tests use `-p <crate> --lib`.
- The private gate's oracle leaves out 00cb17f0, 00e53090, 00e53130 (they
  loop forever in the exe on huge fuzzed counts) until nv-call has a per-vector
  time limit: PR #200 adds it.
- **PR #200 (contributor):** changes requested on 10-17 (an engine read of
  unmapped memory when the exe returned a value must count as a failure; my
  mutation test passed through it). Waiting on the contributor; I offered to
  make the change myself.

### Claude lead, 2026-10-16 evening (previous)

The session stopped at the weekly limit (95-96 % rule). Every finished
batch is merged or listed below; the next account continues from here.

**State:**
- **Merged this session:** #209 to #214 and the PR that carries this note.
  23,502 hand-translated functions (35.5 %), 15,762 start-up table
  rows (23.8 %) and 4,790 shut-down table rows (7.2 %): 66.5 % covered.
  Engine tests: 30,188.
- **Havok:** 3,981 of the 6,591 ledger rows translated, 392 traced and
  2,218 open.
  `bin/havoktask.sh NN 120` hands out the next whole units.
- **Shut-down table (#213):** research/initgen `--atexit` matches the 5,020
  atexit destructors against two shapes (`call`, `tail`) and writes
  `crates/engine/data/atexit_destructors.tsv`; `static_exit.rs` runs a row
  by shape and `Engine::call` falls back to the tables when nothing is
  registered at an address. The `_atexit` stand-in records into the exe's
  own table and `crt::doexit` (00eccd99) runs it in reverse, then the
  pre-terminator and terminator tables. Not wired into the host shutdown.
  229 irregular destructors are left for hand translation.
- **Game code vs platform (#213):** SpeedTree (all of it: tree building,
  splines, random numbers, wind) and the logic units of BSShader, BSSystem
  and d3dx9 (`LOGIC_UNITS` in scripts/ledger) are game code to translate:
  1,214 functions moved from platform to open. About 600 shader and file
  functions the engine map placed in no unit are still `platform` until
  someone sorts them by reading them. Unit files are scaffolded (#214).
- **SpeedTree** is under way: 177 translated, 456 open. Batch 1 (random
  numbers, splines, leaf and branch info) and batch 2 (branches, indexed
  geometry) are in the PR with this note; `frondengine.rs` is next.
- **The two largest solver functions** (00d8aac0, 00d8fb70, about 2,600
  instructions each) are translated as block state machines and were
  checked bit-exact against the real exe through nv-call (185 scenarios).

**Where the last batches stopped** (start at the address given):
- `hkdataobjectutil.rs`: none of its 37 functions done; start at 00dc1bf0.
  In 00dc1d30, subtype 0x1d fills its array twice because it falls through
  into the 0x21 body. 00dc3d40, 00dc4430 and 00dc4710 pass an `hkDataObject`
  temporary by address (`with_stack`).
- hkDataObjectDict: about 60 functions left after the 58 in this PR.
- `hkbinarytagfilewriter.rs`: 25 left, from 00dbe790.
- `hkaragdollutils.rs`: 10 left, from 00ce9880.
- Animation: 17 of `hkawaveletcompressedanimation` (from 00e2f7c0), its
  class (7), `hkbasesystem` (12), `hkclassenum` (5), and two large ones:
  00cdbae0 (two-joint IK solve) and 00ce61c0 (skeleton-mapping creation).
- SpeedTree: `frondengine.rs` (55, from 00b17fd0), then the remaining units.
- CRT gaps found by tests: acos 00eca190 and pow 00ec9ac0 have no entry
  (`crt.rs` registers 00eca1d0 and 00ec9b00), and `fabs` 00ec6cde calls
  00ed5abb, which is also unregistered.
- 229 irregular atexit destructors and 498 leftover start-up entries.

**Open questions for the maintainer:**
- **Renderer: decided (ADR-0007).** The game's renderer is translated and
  runs on our own Direct3D 9 layer on wgpu, with the game's shaders
  converted at load time. Settle the x87 question below first; Bevy stays
  the development viewer until the new path matches.
- **x87 precision: answered.** The game creates its device without
  `D3DCREATE_FPU_PRESERVE` (00e72e60 adds it only for renderer flag 0x40;
  the exe passes 0x29). Measured: the main thread runs at 0x027F until
  00e72e60, then 0x007F (24-bit) for the rest of the run; the AI linear task
  threads (Havok step) stay at 0x027F. The engine keeps a control word per
  thread, `crate::x87` rounds to its precision, and
  `platform::direct3d_device_created` switches the main thread (the host
  calls it until 00e72e60 is translated). Translations on the main thread's
  path still compute at 53-bit until they use the x87 helpers
  (docs/ENGINE_CRATE.md, "x87 precision").
- **SSE flush-to-zero.** Havok's job worker thread (00d2cfb0) sets the
  MXCSR flush-to-zero bit, so SSE results on Havok threads flush
  denormals. The engine does not model MXCSR yet (00d2cfb0 is marked
  incomplete for it); decide how the stepped threads carry it.
- **Two NaNs meeting (#199).** When two NaNs meet in an x87 `FMUL` or
  `FADD`, the exe keeps the one with the larger significand (the positive one
  on a tie); f64 arithmetic keeps the first operand. The pointer-argument
  check (#200) found it in 006e9000, 006b99e0, 00e0a750, 00e0a850, 00e436b0
  and 00e44340. Fix it once, in the x87 NaN helpers, not per function.
- **Multiply by -1.0 (#215).** LLVM folds `x * -1.0` into a negation in
  release builds, which flips a NaN's sign that the exe's `FMUL` keeps. The
  three known sites now multiply by `std::hint::black_box(-1.0)`; use the
  same form for any new one.

**Machinery notes:**
- Slot rules added today (private slot_rules.txt): avoid clippy's
  approx_constant, redundant closures, unnecessary to_vec and manual
  clamp/div_ceil/saturating_sub, or allow them with a one-line reason.
- Base new batches on the branch that holds every collected commit
  (havoktask.sh and listtask.sh reset slots to `claude/engine-crate`; move
  the slot to the collection branch when that is ahead). A slot started
  from a stale base once re-translated 31 functions main already had.
- The private purity facts missed the register inputs of 00d49c20 (reads
  EAX and ECX via SUB and LEA), so the oracle called it with stack words
  and failed; that row was patched by hand. The facts generator should
  count register reads by arithmetic and LEA, not only MOV.
- The README progress bar shows hand translation, the start-up table and
  the shut-down table as three segments (private bin/progress.sh).

### Claude lead, 2026-10-16 morning (previous)

**State:**
- **Merged this session:** #203 (GPL-3.0-or-later as the single licence),
  #204 to #206 and the PR that carries this note. About 20,500
  hand-translated functions (31 %), about 55 % covered with the start-up
  table, about 25,300 engine tests. #197 and #198 are fixed.
- **Contributor PRs:** reviewed on 10-16. #91 (Wayland): CI approved, Linux
  job needs `libwayland-dev` and `libxkbcommon-dev`, then rebase. #29 and
  #87: changes requested. #50, #62 and #108: rebase, then merge. #48, #59
  and #93 closed with thanks. #200 (pointer oracle): changes requested (the
  NvPurity phi rule must keep an offset only when all inputs match).

**Maintainer decisions (2026-10-16):**
- **Translate first, wire later.** Movement wiring (Build 43) is paused so
  the compile capacity goes to translation. The wiring branch
  `claude/build43-movement` is kept locally with a status note; it runs the
  game's own `PlayerMover::Update` path and falls back to `crates/physics`
  on any untranslated function. Resume it once Havok and the start-up code
  are translated: run its static_init test first.
- **All of Havok is being translated** (about 6,900 functions at the start,
  about 4,900 open now), BSHavok glue first, then hkp*, then the rest, in
  batches of about 120 whole units per agent. Most "Translation incomplete"
  markers in these batches are only the FS:[0] unwinding frame or the /GS
  cookie; every branch is translated.
- **Real-game movement capture** works (`nv-movecap`, kept on a local
  branch for now): it starts the real game unattended, dismisses the add-on
  notices like a player, drives scenarios and records every frame. Ground
  truth: walk about 85, run about 340, sneak about 49 units/s; jump apex
  59-61.6 (the controller's `fJumpHeight` is 64); take-off 275-286 units/s.
  Replay each frame's recorded dt when comparing.

**Machinery notes:**
- An engine test compile now peaks near 15 GB. Two at once filled the 31 GB
  machine and swapped, so the second compile slot opens only with 16 GB of
  free physical memory. Run about 5-6 translation agents, not 8. Splitting
  the engine crate would allow more parallel compiles.
- When collecting, a clean git cherry-pick can still duplicate a function
  when two batches add items next to each other: the compiler reports
  E0428, and the second copy goes. Never rewrite `funcs()` with a tool that
  only understands `entry!`: some units register hand-written
  `(addr, f as AbiFn)` tuples.
- Refresh the private ledger snapshot before handing out work, or batches
  get assigned functions that are already done.

### Claude lead, 2026-10-14 (previous)

The maintainer alternates two Claude Max accounts on this one machine, never
both at once. This session stopped at the weekly limit (95-96 % rule); the
next account continues from here. Talk to the maintainer in plain language.

**State:**
- **Build 42 is in the play copy** (`Desktop\nv-rs-play`, main 95144b1):
  16 command-free quests run through the game's own translated script
  interpreter (`QuestVm` hook, #170); `Host::start` runs the 15,762
  static initializers; the startup line and F12 report count interpreter runs.
- **Merged this session:** #161, #164 to #180: quest loaders (#169, #171:
  stages, `SetDone`, targets, conditions, condition items, objectives,
  topics and responses, TESFile chunks, CRT `sprintf`/`_snprintf`), the
  QuestVm hook (#170), lockfile hash (#172), CI build cache (#173), rolling
  batch and `docs/SYSTEMS.md` (#174), real `TESQuest::Load` through BSFile
  stand-ins (#175), the movement batch with all of bhkCharacterController
  (#176), README, contributing and milestones rewrites with screenshots,
  progress bar and logo (#177 to #180). The task list (docs/TASKS.md) and
  all `[task]` issues are retired; only the Linux issue #61 stays open.
- **Open:** nothing of ours. The contributor PRs (#29, #48, #50, #59, #62,
  #87, #91, #93, #108) are untouched and need the maintainer's review: keep
  platform and route PRs (ask for rebases), check the license of the
  research source in #48, and #93 changes original behaviour.
- **Ledger:** about 16,500 hand-translated (24.9 %) plus 15,762 table-run
  static initializers. Engine tests: about 20,700. Main is fully gated; the
  lead branch `claude/engine-crate` equals main.
- **Social preview:** `docs/media/social-preview.png` is made, but GitHub
  has no API for it. The maintainer uploads it in the repo settings (or
  Claude in Chrome does, once connected).

**Maintainer decisions (2026-10-14):**
- **Order:** movement and physics, then animation, audio, renderer, then the
  gameplay layer (AI, combat, quests, menus, VATS, magic, progression,
  crafting, minigames), then saves and the rest. The goal is about a month for
  the bulk. **docs/SYSTEMS.md is the master list of every system; keep it
  current and leave nothing out.**
- **Threads:** step game threads cooperatively on the main thread at fixed
  frame points, behind one seam in `units/platform.rs`
  (CreateThread/ResumeThread/waits/events/semaphores dispatch through a
  thread-host trait), so real OS threads can replace it later to replicate the
  original. Never bake single-thread assumptions into translations.
- **After the port:** an optional fixes layer (original behaviour always
  available for comparison; data fixes like YUP stay installed mods), and
  xNVSE / JIP LN compatibility (check licenses first). GOG/Epic exe support
  later. Optional real-time shadows off by default. Bevy upgrade one version
  per PR before the renderer phase.
- **Never copy code from other New Vegas reimplementations.**

**Correction:** the Havok SDK (5,081 functions) is NOT ported; only the B1
solver work (79 functions in `crates/physics`). Phase A (movement, physics,
animation, faces, audio) is about 8,000-9,000 functions of new translation.

**Next work, in order:**
1. Everything finished this session is collected, including the whole
   bhkCharacterController (98 functions, the first finished movement unit).
   Continuation points: combatstate 0099cb70, projectile 009c3d90 (24
   left), middlehighprocess 0091db40, fonts 00a1c130 (3 left).
   **Ghidra pitfall:** it prints both `FMUL ST(0),ST(1)` and
   `FMUL ST(1),ST(0)` as `FMUL ST1`. Check the instruction bytes whenever
   such an x87 operation matters (now in slot_rules.txt).
2. Phase A translation: the rest of bhkcharactercontroller and
   middlehighprocess, then `Havok SDK` (largest), `BSHavok`, `NiCollision`,
   `fallout shared/havok`, `NiAnimation`, FaceGen, BSAudio/fallout audio.
   Use `bin/assign.sh` only for units on this path; write custom task
   messages naming the unit, as this session did.
3. Quests (#175, real `TESQuest::Load` through BSFile stand-ins, all 300
   scripted quests load with their stages): stage items stop inside
   `TESQuestStage::SetDone` (0060fb60). Translate 004b0fc0, 004b10f0,
   0060da90, 0060ca30, 005ec060, 00681050, 00682320 (about 650 bytes);
   then stage effects need the player object in the host. Plugins with
   masters (DLC) and zlib-compressed records still fall back.
4. The thread-host seam in `units/platform.rs` (small, needed before audio).
5. Wiring for Build 43: switch the player's movement onto the translated
   character controller in the viewer (as `QuestVm` did for quests), with an
   acceptance route comparison.
6. `RSQRTSS` uses the host CPU's estimate (`tesobjectrefr::rsqrt_estimate`,
   used by camera caster and character point collector), which differs
   between Intel and AMD. Decide whether to emulate one CPU's table so
   movement is the same on every machine.
7. Consolidate the private x87 NaN helpers (they disagree) into
   `crate::x87` against real-CPU recordings.
8. Shared layouts agents keep asking for: Setting, NiNode, BSThread,
   ActorPackageData, NiExtraData, EffectSetting, ActiveEffect, ListBox,
   NiTPointerList node, PathBuilder, AStarSearch, hkVector4 and SSE helpers,
   LockFreeMap, ContainerObject/TESContainer, BGSListForm, TesResponse
   (two declarations).
9. **Look-alike families (speed-up, from a review of decomp.wtf).** Many
   exe functions are near-identical template instantiations (BSSimpleArray,
   BSScrapArray, NiTMap, LockFreeMap, NiTPointerList helpers, scalar
   deleting destructors, constructor and destructor pairs). Build a tool in
   `research/` that groups functions by normalised instruction shape
   (opcodes and operand kinds, addresses and constants masked) and lists
   each family with its translated members. Give translators the family
   and an already-translated sibling as the template in their task, and
   assign whole families at once. Measure functions per batch before and
   after.
10. **Done by contributor MeatisOmalley (#182, #190, merged 2026-10-14):**
    `research/ghidra/NvPurity.java` and `research/nv-oracle/nv-pure`. Its first
    run: 324 functions, 10,326 vectors, one real bug (#189, fixed in #191).
    **Next:** add `nv-pure check` to `phase0/bin/check-lead.sh` (set
    `NV_ORACLE_EXE` and `NV_CALL` locally), regenerate `pure.tsv` after each
    collection so new pure translations are checked, and file any mismatch as
    an issue. Their follow-ups in #190 (pointer arguments, runtime-state
    snapshots, presetting ST0, a ledger "oracle-checked" column) are good next
    steps for them or us. **Pointer arguments are in progress by
    MeatisOmalley (#193, branch `oracle/pointer-args`, approved
    2026-10-15):** about 2,000 read-only pointer functions in phase 1 and
    about 1,000 out-parameter functions in phase 2, with buffers at
    0x30000000 guarded against the engine heap top. Review their PR when it
    lands, the same way as #190 (Ghidra script read-only, no exe bytes
    committed, mutation-tested). The original task text follows.
    **Automatic oracle check for every pure translation.** `nv-call`
    (`research/nv-oracle`) runs a function on the real CPU. Classify
    translated functions that are pure (no globals written, no calls out, or
    only calls to other pure functions) and generate differential tests that
    run both the exe and the Rust on many inputs, NaN and edge cases
    included. Add it as a gate step, so a wrong translation fails by itself
    the way decomp.wtf's byte-for-byte judge does. Commit test inputs and
    addresses only, never exe bytes or outputs.
11. **Not adopted:** decomp.wtf itself. It does matching decompilation
    (C that recompiles to identical bytes), supports PC games only through
    existing community projects, and would still leave a Windows x86 exe to
    port. Behaviour translation into Rust stays the method.

**Pipeline notes learned this session:** after any Windows restart run
`phase0/bin/start-srv.ps1` (Ghidra 8090 does not come back by itself).
Agents follow `phase0/slot_rules.txt`: check pushed
arguments against the callee's RET size; filter `call_log` by address;
several cargo test filters go after `--`; poll heavy.sh logs instead of
waiting on monitors. An agent continuing a unit whose first part is not
collected yet resumes on its own commit (`git reset --hard <its commit>`),
and both commits are collected by hash. Check every collection for
duplicate `entry!` addresses.

### Claude lead, 2026-10-12 to 2026-10-13 (supersedes the Codex section below)

The Codex handoff below was audited against the user's takeover prompt and
found accurate. Since then:

- **Merged:** #147 to #160.
  Highlights: task-queue return fix (#147); inventory source, replay and
  world stacks (#148, #156, #154: 28,279 saved inventories reproduce byte
  for byte through 004d4160/004d4090); Phase 1 PR 8, buffered controls and
  menus at the interface idle (#152); the game's script interpreter in the
  engine, S0 to S3 (#157: 163 of 322 quest scripts run to completion in an
  opt-in test); S4, the engine host started in the viewer with the
  1.4.0.525 exe check (#160); contributor guide (#150). Rolling translation
  batches #149, #151, #153, #155, #158, #159.
- **Open:** rolling #162; S5 (#161, ScriptLocals bridge, being rebased by
  the wiring agent in `%USERPROFILE%/nv-re/work/wt-script`).
- **Translated:** 12,478 at #162 (18.8 % of 66,259); later collections
  are on the lead, gated (16,320 engine tests).
- **Maintainer decisions:** the wired engine reads the player's own
  FalloutNV.exe read-only from the install with Data and refuses anything
  but 1.4.0.525 (GOG/Epic later). Translation is the maintainer's work only
  (CONTRIBUTING.md). Stop rule: 95 % weekly usage.

**Pipeline (private, `%USERPROFILE%/nv-re/work/phase0`, details in
STATE.md):** up to about ten agents; Sonnet for translation slots, one Opus
agent for wiring. `bin/heavy.sh` is a two-slot, first-come compiler queue
(slot 2 only with 12 GB free commit memory; stale locks clear after two
minutes); compile only, never probes or routes. Agents compile once
(focused tests plus hygiene); the lead collects ready slots with
`SKIP_CHECKS=1 ADVANCE_PARTIAL_ONLY=1 bin/cycle.sh NN 0 0`, then runs
`bin/heavy.sh bin/check-lead.sh` once (fmt, name grep, clippy, full engine
suite, hygiene) and fixes trivial lints itself. Before collecting, compare
the slot's files with the lead since its base: if the lead changed one,
merge by hand. Ship rolling PRs with `HOLD_LATER_PUSH=1 bin/ship.sh N`,
then regenerate the ledger, gate and push. Compiler-only gaps (FS:[0]
frames, /GS cookie 00ec408c) always carry `Translation incomplete`
markers; float comparisons and x87 copies match the exe, NaN included.

**Next:** script wiring S6 (quest scripts through the game's interpreter in
the live game) once the remaining form constructors are on main (a11's
batches added about 125; package and topic constructors exist; actor
references go through CreateReference, already translated). Then Build 42.
Phase 2 step C (inventory section rebuild compared per change form).

### Final Codex handoff requested, 2026-10-11 evening

The user asked to finish current work, start nothing else, and hand off to
Claude. This supersedes all automatic restart instructions below. The Codex
heartbeat `keep-nv-rs-translation-moving` is PAUSED. No new slot, cycle or
compiler is authorized in this closing pass. Preserve the unfinished work;
Claude can use the resumption order below when asked to continue.

#### Immediate state and release hold

- Main is verified through #146, merge `05ee5a0`. The ordinary Desktop
  `nv-rs` checkout was not used for these changes; do not reset it.
- Lead: `%USERPROFILE%/nv-re/work/wt-phase0`, branch `claude/engine-crate`.
  #147 is DRAFT at published `d35327b`. Both checks pass for that old head,
  but **do not merge it**: it discards the callback return at `0087a890`.
  Local commit `331b99e` fixes the function and ABI entry to return `u8`,
  with noncanonical/nonzero-upper-byte regressions. PC and independent
  review are clean; formatting passes. Normal Cargo validation is UNRUN.
  The correction and this final handoff are local, not pushed.
- The last cycle01, session 27282, is DONE and must never be rerun.
  It collected the first taskqueue batch as `c766db2` and wrote the next
  task. Its 14,469 engine tests, doctest, strict lint and formatting pass.
- Water's release build was the only active compiler at the stop request.
  Its final result and commit are recorded in the closing note below.
  No collection or fresh water task will be started by Codex.
- Build 40 is still the user's play copy. No later build was promised or
  published. The usage reading at the stop request was 93%; retain the
  user's 98% weekly stop/handoff rule if this Codex session is resumed.

#### Closing result

Water release session 92015 finished successfully (25m45s). Slot `agent-04`
committed `22c75494201a826e0b6d19651262df7d2b0867ac`; its worktree is clean
and only `fallout_shared/bgsautowater.rs` changed. All 37 ordinary bodies
are translated: 32 complete, five SEH-only partials, none skipped. Focused44,
engine14,355 plus one doctest (one ignored), strict all-target lint,
workspace formatting, hygiene49 and the engine release build all pass.
The worker stopped. No cycle04, new task, collection or replacement worker
was started. Final process inspection found no Cargo, rustc or cycle shell
running. The lead return correction remains unvalidated by normal Cargo.

Uncommitted a01/a03 drafts have private patch backups
`work/phase0/codex-audit/handoff-20261011-a01.patch` and `-a03.patch`, with
source hashes in `handoff-20261011-source-manifest.json`. Both backups passed
reverse-apply checking against the preserved working files; no patch was
applied. Do not reapply them on top of the already modified trees.

#### Work preserved for Claude

| Checkout / branch | Preserved work | Verified / still required |
| --- | --- | --- |
| `work/agents/a01`, `agent-01`, task b0213 | Uncommitted `fallout_misc/taskqueueinterface.rs`; last 31 ordinary bodies, all 71 now drafted; 49 new tests, 91 total; 49 explicit compiler-only partials including earlier ones | Private standalone 91 tests and lint pass; 23 native cases and independent PC review pass. Fresh normal-registry Cargo tests/lint/hygiene/release UNRUN. Do not reset or run task.sh. |
| `work/agents/a03`, `agent-03`, task b0223 | Uncommitted `fallout_misc/bgssaveloadchangesmap.rs` and `bgssaveloadfile.rs`; 40 ordinary bodies, 43 tests, eight compiler-only partials | PC review and 33 native cases pass. Required Cargo checks UNRUN. Do not reset or run task.sh. |
| `work/agents/a04`, `agent-04`, task b0046 | Clean commit `22c7549`, `fallout_shared/bgsautowater.rs`; 37 ordinary bodies, 44 tests, five exception-only partials | Focused44, engine14,355 plus doctest, strict lint, formatting, hygiene49 and engine release pass. Not collected. |
| `.codex/worktrees/phase2-extra-data/wt-phase0`, `codex/phase2-inventory-source` | Clean local `e024a28`, source checkpoint `c1c2897`, based on actual #146 merge. Seven fos files plus FOS_SAVES/ENGINE_STATE docs | 38 package tests, strict package lint and formatting passed before rebase; private source and composition probes pass. Full workspace/release checks and final Cargo-artifact probes UNRUN. No PR. |

Paths in that table are relative to `%USERPROFILE%/nv-re` except the
`.codex` checkout, which is relative to `%USERPROFILE%`.

a01's private harness explicitly installs draft functions into an existing
release registry. It is useful evidence, not a substitute for rebuilding
the normal registry. Its first run found unmapped global pages in test
fixtures; explicit test maps resolved all 53 failures without changing
production logic. The 31 new bodies have 18 complete and 13 partial entries:
eight cookie omissions (`0087b650`, `0087b6b0`, `0087b700`, `0087b760`,
`0087b7c0`, `0087b810`, `0087b850`, `0087b890`) and five exception omissions
(`0087b8d0`, `0087b990`, `0087d030`, `0087d1b0`, `0087d230`). All 50 dispatcher
targets were independently compared with the PC jump table. Evidence lives
in `agents/scratch/a01/round20-status.txt`, its harness/native results, and
`agents/scratch/a03/taskqueue-round20-review/review.txt`.

a03's eight partials are `008455b0`, `008457b0`, `008458b0`, `00845960`,
`00845c00`, `008461e0`, `00846520` and `00846a20`. Water's five are
`0049c860`, `0049c930`, `0049db60`, `0049e410` and `0049ec80`. Their normal
paths are implemented; compiler cleanup is not silently declared complete.

#### What this lead completed

- Took over the existing slot work without discarding partial edits,
  collected validated batches, reviewed PC behavior and native probes,
  maintained the ledger, and merged only after both current-head CI checks.
  The merged rolling batches are #109, #112, #114, #116, #119, #120, #121,
  #123, #124, #126, #127, #129, #131, #133, #134, #137, #139, #141, #142,
  #143 and #145. They cover navigation/pathing, exterior/multibound loading,
  AI/Havok, explosions, animation, movement, effects, HUD, I/O and audio.
  See Git history for exact contents; these are translations, not proof of
  playable completeness. Unrelated contributor PRs were left alone.
- Finished Phase 1 PR 7 as #113 (`f917219`): interface/render call models and
  post-swap ordering, plus the active off-screen actor/menu/freeze and late
  SFX/radio/music order. Published Build 40. Core/viewer checks and scripted
  Doc, VCG02 and VMS16 routes passed (63, 410 and 339 seconds). Evidence is
  `%USERPROFILE%/nv-re/acceptance/build40-main-20261010`; no original-game
  comparison was claimed. Exact still-open input/render boundaries remain
  in FRAME_SKELETON.md.
- Advanced Phase 2 through #115, #117, #118, #122, #125, #128, #130, #132,
  #135, #136, #138, #140, #144 and #146: engine-owned extra-list/count/health
  adapters, reference accessors, native inventory read/write buffers and
  form maps, then script headers, action/BEGIN lists and saved-local values.
  ENGINE_STATE.md in the root Phase 2 checkout contains detailed evidence.
- #146 preserves raw integer flags, x87 signaling-NaN behavior, first-match
  variable lookup, reference bits, live callback field reloads and compact
  IDs. It passed 16,273 workspace tests including doctests, strict lint,
  formatting and release/debug builds. Native comparison covered 102 cases
  plus 18 complete save/load/save chains (36 additional native calls).
  Final normal-registry release probes matched all nine load buffers,
  eighteen save buffers and eighteen three-leg chains.
- Hardened private shipping/queue handling: retain explicit partials;
  only advance a partial-only lane after review; check exact remote heads
  with leases; stop on a failed remote-ref query; preserve later local
  commits when needed. task.sh retains caches after the prior automatic
  cleanup rejection. No deletion bypass was attempted; the user cleaned
  space manually. Do not repeat old cycles or task resets.
- Added a recurring Codex check after coordination paused between replies;
  it is now explicitly PAUSED for this handoff. It must not compete with
  Claude or resume without authorization.

#### Phase 2 evidence and remaining boundary

Original Count/Health item reads and untouched writes matched 10,201 item
payloads in earlier probes, with three full-record decode failures excluded
explicitly. Eighteen multi-stack items across eight private saves retain
separate stack order, counts and raw health bits. Script-bearing item probes
also round-trip selected payloads and run the real BEGIN handlers; one
official Save138 case uses installed script declarations and zero defaults.
Historical nonzero locals with missing mod scripts were not replaced with
guessed current scripts. Generated nonzero save/load cases are separately
labeled and are not historical mod-save acceptance.

The current `fos::inventory_source` batch captures immutable save context
and item/stack/extra spans through the existing complete record decoders.
Known-length opaque payloads stay byte-exact; unknown lengths and complete
record failures keep the whole record, with no partial inventory published.
Saved identity, resolution, signed item delta and source order remain
separate. Three known decode failures and three unresolved holders remain
explicit in the newer original-save probes. The new composition probe takes
actual envelopes through translated item loading and untouched writing;
owned snapshots survive engine disposal. Its generated form headers and
partial allocator setup are fixture infrastructure, not runtime startup.

The seven changed fos source files are `lib.rs`, `pipe.rs`, `decode.rs`,
`decode/actor.rs`, `decode/extra.rs`, `inventory_source.rs` and
`inventory_source/tests.rs`. Promotion from the frozen private mirror was
hash-verified and is DONE. Never blindly reapply that mirror: one intentional
invalid-range test was subsequently rewritten as a Range struct for Clippy.
Private manifests/probes are under `work/phase0/codex-audit`.

The live world still aggregates inventory count/health rather than owning
these independent stacks. Whole-FOS writing remains synthetic-only; no real
save round-trip or gameplay persistence gate has passed. Engine is not
Send/Sync/Clone; do not force it through unsafe traits or a shared mutex.
Keep owned plain snapshots and source context at the world boundary. No new
play-build date or visible Phase 2 improvement is established yet.

#### Resumption order for the next lead

1. Inspect status and any running processes before changing anything.
   Read AGENTS.md, CLAUDE.md, this top section, MILESTONES.md, ENGINE_STATE.md
   in the Phase 2 checkout, and private `work/phase0/STATE.md`. Historical
   sections below do not override this stop/handoff.
2. Preserve local `331b99e`. If water has its clean final commit, collect it
   exactly once after verifying all gates. Its combined lead checks can
   also validate that minimal return fix. Only after success publish the
   corrected #147 head, refresh the PR description and take it out of draft.
   Both new core/viewer checks must pass; the old green head is insufficient.
3. Validate a03's frozen draft, a01's final draft, and the root source batch
   in a deliberate single-compiler order. Do not repeatedly starve root
   integration with new leaf batches. Each slot commits only its owned files;
   cycle only validated clean commits. New agents/continuous cycling require
   the next lead's resumption instruction, not this closing session.
   Once the three slot drafts pass all gates, their ordinary bodies are
   exhausted and the reviewed compiler-only tails can remain explicit with
   `ADVANCE_PARTIAL_ONLY=1` during the authorized collection. Never use that
   switch to bypass unimplemented ordinary behavior or failed checks.
4. For the root batch, use private `check-inventory-source.ps1`, then
   `run-inventory-source-final.ps1`. Both are prepared/parser-checked but
   UNRUN. The final runner uses actual Cargo release fos/engine artifacts,
   writes separate final evidence and does not overwrite the frozen mirror.
   Publish only after required checks and final original-item probes pass.
5. Continue toward real world inventory integration and the Phase 2 real
   save-round-trip gate, then viewer acceptance before another play build.
   Original game installation, raw saves, exe/PDB bytes and decompiler
   exports remain private; only names/addresses/provenance belong in Git.

Use one heavy compiler and one cycle at a time, `CARGO_BUILD_JOBS=1` and
`CARGO_PROFILE_DEV_DEBUG=0`. Two engine compilers exhausted this 31 GB
machine's committed memory despite earlier estimates; do not repeat that.
Root focused tests should select the same workspace/features as full tests
to avoid unnecessary recompilation. Ghidra is GET-only on 127.0.0.1:8090,
never 8089. Use engine_map.tsv/frame.tsv and the private pdbdump tool;
TESForm Xbox offsets minus 0x10 still require PC verification. Mark Xbox PDB
names and keep all reference material outside Git.

### Release hold, 2026-10-11 20:22 EDT

PR #147 is DRAFT and must not ship its published `d35327b` head, even if
both checks pass. The next taskqueue pass exposed a discarded return in
`0087a890`: PC preserves the virtual callback's AL and caller `0087d160`
tests it. Root and an independent reader verified both disassemblies.
Lead now corrects the function and registry return type to `u8`, with the
existing regression checking noncanonical low byte `7f` and zero under
nonzero upper bytes. Formatting passes; normal Cargo checks are pending.
Keep this correction local until the next water collection validates the
combined lead, then push the corrected head and require both fresh CI checks.

Water's corrected 44 tests, 14,355 full engine tests plus doctest, strict
lint, formatting and hygiene49 pass. Release session 92015 owns the sole
heavy compiler. Fresh a01 has all 31 remaining bodies and 49 new tests
drafted, but its required Cargo checks are unrun. A private lightweight
harness and independent a03 PC review are in progress. a03's own save-file
slot stays compiler-held. Usage reached 90%; heartbeat remains active.

### Current state, 2026-10-11 20:05 EDT

This section supersedes the historical notes below.

- #146 merged `05ee5a0` at 19:31 EDT after both exact-head core/viewer
  checks passed. Lead's later documentation was rebased before collection.
- cycle01 session 27282 is DONE PASS; never rerun it. Taskqueue source
  `0ee2d55` collected as `c766db2`: 14,469 combined engine tests, one
  passing doctest (one ignored), strict Clippy and formatting pass. The
  slot also passed 42 focused tests, hygiene49 and its workspace release
  build. Forty ordinary bodies are implemented; four are complete and
  thirty-six retain reviewed compiler-only cookie/exception partials.
- Fresh worker a01_round20 immediately started the newly written b0213
  task. It continues the remaining thirty-one ordinary bodies beginning
  `0087b650`; do not loop over the already reviewed compiler-only partials.
  It is source-only until an explicit compiler grant.
- Water a04_water_complete now owns the sole heavy compiler. Rerun the
  corrected forty-four focused tests, then remaining full checks/release;
  commit cleanly before cycle04. Its source retains five explicit exception
  partials. a03_round20's save-file/change-map batch remains unchanged and
  compiler-held; it follows water. Run only one cycle at a time, and start
  a fresh worker immediately after each successful cycle writes its task.
- Root inventory-source branch `codex/phase2-inventory-source`, checkpoint
  `e024a28`, is rebased onto #146's actual merge. The 38 package tests,
  strict package lint and formatting passed before that rebase. Full
  workspace validation is queued after slots; no PR for this source yet.
  Private check-inventory-source.ps1 and run-inventory-source-final.ps1 are
  prepared and parser-checked only. Run them in that order; the final probe
  must use actual Cargo release artifacts rather than the frozen mirror.
- The new private composition probe passes eighteen original ItemEnvelopes
  through the current release engine, with complete reads, exact untouched
  writes and owned snapshots surviving engine disposal. Three decode
  failures and three unresolved holders remain explicit exclusions. Its
  generated form/allocator setup is not production startup, gameplay import
  or whole-save acceptance. Root ENGINE_STATE.md records this evidence.
- The thread heartbeat keep-nv-rs-translation-moving is active every fifteen
  minutes. Inspect live process state before resuming; never duplicate a
  cycle or compiler. Preserve the 98% weekly stop/handoff rule and pause the
  heartbeat there or on an explicit stop. Usage was 88% at 20:03 EDT.
  Build 40 remains current; no new play-build date is confirmed.

### Current state, 2026-10-11 17:21 EDT

This section supersedes the historical notes below.

- #145 is merged `fb4b846`; shipping session 30959 is DONE.
- Root #146 is open and attached at `787d282` on
  `codex/phase2-script-local-values`. All focused checks, 16,273 workspace
  tests including doctests, strict lint, formatting and release/debug builds
  pass. Session 74945 is DONE. Final release probe session 98029 is also
  DONE PASS: nine LoadGame buffers, eighteen SaveGame buffers and eighteen
  complete SaveGame/LoadGame/SaveGame chains match native expectations.
  Both current-head GitHub checks are running. Merge after both pass.
- The same root checkout now owns the next branch,
  `codex/phase2-inventory-source`, source checkpoint `446fefd`, based on #146.
  Seven fos files were promoted from the reviewed private mirror only after
  all source baseline and draft hashes matched. Promotion is DONE; never
  reapply the mirror blindly. A test's intentionally reversed range was
  expressed as a Range struct literal to satisfy Clippy without changing
  its rejection case. The 38 package tests, strict package lint and full
  workspace formatting pass. Full workspace tests/lint/release remain
  queued; no PR for this batch yet. Rebase onto the verified #146 merge
  when safe, before final full validation.
- The source envelope keeps exact immutable source bytes/context and
  ordered item/stack/extra spans through the existing complete decoder.
  A private probe against the frozen mirror matches all eighteen original
  multi-stack targets across eight saves, with three known decode failures
  and three null-holder records preserved explicitly. This is source
  preservation, not runtime import or whole-save writing. a03 now prepares
  a private composition probe from the new envelope API into the existing
  normal engine registry; no repository or slot changes are authorized there.
- a01 owns the sole heavy compiler lane, last reported strict Clippy
  session 75392 after its existing forty-two focused and 14,445 engine
  tests passed. Hygiene and release must finish before a clean commit,
  collection and immediate fresh worker. Water and a03 remain held with
  their documented source drafts. After a01 collection, validate water,
  then a03. Root's next full workspace pipeline waits for capacity.
- Do not rebase or otherwise edit lead during an active collection. If #146
  passes while a cycle runs, merge the passing remote PR first and defer
  lead synchronization until that cycle finishes. Run only one cycle at a
  time. Build 40 remains current, with no confirmed next-play date.
  Preserve the 98% weekly stop/handoff rule (81% around 17:02 EDT).

### Current state, 2026-10-11 16:47 EDT

This section supersedes the historical notes below.

- #145 merged `fb4b846` after both current-head core and viewer checks
  passed at head `58a404c`. Shipping session 30959 is DONE; never rerun it.
  Lead's two later documentation commits were rebased and retained locally
  with `HOLD_LATER_PUSH=1`. No new rolling PR is open yet.
- Root saved-local branch `codex/phase2-script-local-values` remains on
  source `af1de08`, based on merged #144. Session 74945 is the sole heavy
  compiler pipeline: all 1,343 script, eight reader and ten writer focused
  tests pass, followed by all 16,273 workspace tests including doctests.
  Strict lint is now running; formatting, release/debug builds and final
  normal-registry load/save/three-leg-chain probes remain pending. Only
  `docs/ENGINE_STATE.md` is dirty in that checkout. Three copied native
  overlap-fixture strings were corrected before the passing run; production
  behavior was unchanged by that test correction. Do not rebase or edit
  source during the running checks.
- a01 taskqueue retains its earlier focused/full-engine passes; interrupted
  Clippy, hygiene and release remain queued. Water's corrected forty-four
  tests and a03's forty-three save-file/change-map tests remain queued.
  All three slots stay compiler-held. After root checks, finish a01 and
  collect/restart it, then water, then a03. Use one Cargo job and one
  heavy compiler pipeline; only one cycle script may run at a time.
- Private inventory projection probes preserve eighteen original multi-stack
  item payloads and a generated three-stack item, including explicit zero
  counts, health bits and Worn markers. This is read-only feasibility, not
  world integration. a03 now prepares a private source-bound owner prototype
  with fresh save mappings and complete provenance; no compilation grant.
  Whole-save preservation, live inventory integration and play acceptance
  remain open. Build 40 stays current with no confirmed next-play date.
- Future root focused tests should use the same workspace selection as the
  full test run to reuse compiled tests. The current running pipeline used
  the old engine-only focused selection and must not be restarted solely
  for that optimization. Preserve the user's 98% weekly stop/handoff rule.

### Current state, 2026-10-11 16:19 EDT

This section supersedes the historical notes below.

- #143 merged `a24bc31` after both current-head CI checks. Its shipping
  session 78829 is DONE; never rerun ship143.
- #144 merged `4116868` after both current-head CI checks passed.
  Rolling #145 remains open and attached, now rebased onto that merge with
  its ledger regenerated. Both fresh current-head checks must pass before
  shipping; checks for the prior `df1a5e4` head do not validate this rebase.
- HUD source `aabfc8b` passed 121 focused and 14,386 engine tests, the
  doctest, strict lint, formatting, 49 hygiene checks and release build.
  cycle03 session 72751 is DONE: collection `34ee119` passes 14,410
  combined engine tests, the doctest, strict lint and formatting. Seventeen
  reviewed compiler-only partials remain explicit (two legacy comments
  needed their missing addresses). All ordinary HUD bodies are translated.
  Fresh `/root/a03_round20` started immediately on b0223, owning only
  `fallout_misc/bgssaveloadchangesmap.rs` and `bgssaveloadfile.rs`.
  It is compiler-held and must not reset the slot.
- `/root/a04_water_complete` owns `fallout_shared/bgsautowater.rs`:
  thirty-seven normal bodies, five SEH-only partials, forty-four drafted
  tests and fifteen passing native comparisons. It is validating now;
  focused tests reached 41/44. Three incorrect expectations were corrected
  against PC assembly (pop element address, plane threshold and independent
  Y-edge handling); production is unchanged and the corrected tests await
  a rerun. `/root/a01_round19` owns the draft for its
  forty TaskQueueInterface bodies and forty-two tests. Four are complete;
  thirty-six retain reviewed compiler-only partials. Its forty-two focused
  and 14,445 engine tests pass. Clippy session 69339 was interrupted without
  diagnostics when system committed-memory headroom fell below 400 MB;
  rerun it, then hygiene and release, after the memory-capacity grant.
  After collection its
  fresh worker must continue ordinary bodies at `0087b650`.
- Root PR #144 merged from `94e709c`, branch
  `codex/phase2-reference-predicate`. Three helpers and four regressions
  pass 16,238 workspace tests, strict lint, formatting and release/debug
  builds. Eighteen native cases pass. Both final release probes load a
  selected original scripted inventory item and write it byte-identically,
  with both expected BEGIN handlers. One uses an official save's exact
  ten-plugin order and actual three-variable defaults. Its saved variable
  list is empty. Historical plugin contents, ordinary script execution,
  whole-save preservation and world/viewer integration remain unverified.
  Both current-head CI checks passed before merging.
- Root follow-on source checkpoints `8ebef74` and `e705c1d` on
  `codex/phase2-script-local-values` owns `fallout_shared/tesscript.rs`,
  `fallout_misc/bgsloadgamebuffer.rs`, `bgssavegamebuffer.rs` and topic
  `ENGINE_STATE.md`. It fixes
  raw flag copying, signaling-NaN stores and live list-item lookup, and adds
  two saved-form reader helpers and the numeric-form writer. SaveGame
  reloads the reference ID and effect flag/pointer after helper calls.
  Thirteen regressions are drafted; root validation session 90996 now runs
  the prepared checks with one Cargo job. No completed result yet.
  The initial session 55330 was stopped immediately to change from two jobs;
  it produced no result. The 102 native cases pass.
  Independent PC/source review passes, including the corrected test fixtures.
  It is rebased onto #144; run the prepared full check pipeline and
  final normal-registry release probes (nine LoadGame and eighteen SaveGame
  vectors) when a lane is free. An additional eighteen native three-leg
  SaveGame/LoadGame/SaveGame chains pass, carrying actual results forward;
  their prepared Rust release probe remains unrun. Its baseline stops at
  the now-added writer `00865db0`.
- Fresh a03's b0223 batch now has forty normal bodies and forty-three
  drafted tests. Thirty-three native cases and independent PC review pass.
  Eight compiler-only partials remain explicit. Cargo checks are queued.
- Root currently owns the only compiler lane. Two large checks exhausted
  committed-memory headroom; stopping taskqueue Clippy recovered about
  8 GB. Gate subsequent checks on available memory and use one Cargo job
  to avoid simultaneous library/test compilation. All three slots are held.
  After root checks, finish a01 validation/collection and restart it, then
  water and a03. No deletion attempts after automatic
  review rejected cleanup and the user removed idle caches manually.
- Build 40 remains current, without a confirmed next-play date or new
  visible feature. Continue cycling and shipping until stopped. Preserve
  the user's 98% weekly stop/handoff rule (73% around 16:18 EDT).
### Active again, 2026-10-11 after the usage reset

This section supersedes the historical stopped state below. User requested
continuous work and manually removed the idle p3 build caches; free space
was 25,230,925,824 bytes afterward. Automatic review had rejected our
deletion attempt with only "blocked by policy"; no bypass was attempted.

- #135 merged `7907420` and #134 merged `89bbde4`, each after both CI checks.
  #136 also merged `95ab97a` after both checks. #137 merged `935701e`, and
  root #138 merged `1614889`, each after both current-head checks passed.
  #139 merged `575378a` after both checks. Collection was active, so the
  remote merge happened first; private `finish-ship139.sh` then completed
  rebase/ledger/push and opened attached rolling #141 at `7a6cc48`. This
  continuation is DONE; never rerun ship139. Ship #141 when both checks pass.
  #140 merged `4166432` after both current-head checks passed; its head was
  `192b46e`. #141 then passed both checks and merged `c56a2d1`.
  ship141 is DONE. Attached #142 publishes the HUD collection at `ed11d75`
  and awaits both checks; its collected source commit is now `c865825`.
- cycle04 session 86301 completed: source `5c914204` collected as `ddeecbd`,
  all normal effect-item bodies through `00405400`. Ten verified SEH-only
  partials were retained with reviewed partial-only advancement. Fresh
  `/root/a04_round16` immediately started b0626, owning only
  `bsmain/iomanager.rs`; next forty entries `00456520` through `00c3fe80`.
- HUD source `ecf56da` passed 64 focused, 14,103 engine and 49 hygiene tests,
  strict lint and formatting. cycle03 session 41605 completed, collection
  `8aaf167`, with 14,177 combined engine tests and strict checks passing.
  Fresh `/root/a03_round18` immediately started its next forty HUD entries
  from `00778930`, compiler held; it is extending the compass marker paths.
  Movement source `598c064` passed 145 focused and 14,138 engine tests,
  strict lint, formatting and 49 hygiene checks. cycle01 completed with
  14,189 combined engine tests; collected commit is now `4cc1b3f` after
  shipping. Its seventeen verified SEH-only partials remain explicit.
  Fresh `/root/a01_round17` immediately started BSAudio task b0694.
  I/O collection is complete: source `a802ff8` became `75bd3b5`, with
  14,239 combined engine tests and strict checks passing. Fresh
  `/root/a04_round17` immediately continued b0626 from `00c3ffb0`.
  a03's intermediate focused pass succeeded (93 tests); HUD continues
  toward forty entries. I/O's remaining twenty-four bodies and callback
  mutation regressions are source-ready. Audio source `fae652b` passed all
  checks; cycle01 session 80637 collected `4c16784` with 14,279 combined
  engine tests passing. Fresh `/root/a01_round18` immediately began the
  remaining twenty-two audio bodies from `00ad7e90`. Root's two-helper
  checks completed: all 16,062 workspace tests (14,209 engine unit tests),
  strict lint, formatting and release/debug builds pass. I/O now owns the
  sole compiler checks all passed: 82 focused, 14,271 engine, strict lint,
  formatting, 49 hygiene checks and release build. Source `49b2fe1` collected
  through cycle04 session 29733 with 14,311 combined tests; collection is
  `1fc794c` after shipping rebase. Fresh `/root/a04_round18` immediately began
  BGSAutoWater b0046 (source-only). HUD source `9d9427b` passed 114 focused,
  14,227 engine, strict lint, formatting, 49 hygiene checks and release.
  cycle03 session 6725 is DONE: collection `faae60d` passes 14,379 combined
  engine tests. Fresh `/root/a03_round19` immediately began the remaining
  normal bodies from `0077f5e0`, compiler held. Audio now owns the sole
  compiler, followed by cycle01 and root's predicate follow-on. HUD's batch
  has forty normal bodies through `0077f5c0`, and audio
  twenty-two through `00ad87d0`. Audio also has seventeen passing native
  timer/empty-call cases. Review fixed HUD callback capture timing and a
  full-EAX predicate; I/O has additional pointer/flag reload regressions.
  I/O's eighteen and audio's nineteen remaining markers are SEH-only;
  reviewed partial-only lane advancement is allowed after validation.
- Root #138 is attached/published at `e7cf965` on
  `codex/phase2-script-action-parser`: `expression.rs`,
  `tesscriptrunmanager.rs` and topic `ENGINE_STATE.md` contain six helpers
  and eighteen new regressions. Thirty native cases pass (six parser, nine
  constructors, four entry and eleven evaluator), with raw evidence private
  under `codex-audit`. All 16,011 workspace tests, strict lint, formatting
  and release/debug builds pass. CExpression is 0x710 bytes,
  not a pointer wrapper. EvaluateLine `005e1550` now drafts BEGIN and action-list
  paths; non-BEGIN ordinary execution/parse stays explicit partial and panics.
  Independent PC review of all six helpers passed. #140 now publishes
  `004013e0` and `00500940` at `192b46e`, with forty-one native cases and
  all local checks passing. Both PRs are attached; #138 is already merged.
  The actual-bytecode normal-registry probe now passes both real BEGIN
  handlers and its expected action entry. The original item-payload probe
  ran but stopped at `008d0360`; the rewrite assertion has not run.
  Root now owns `fallout_ai/baseprocess.rs` and
  `fallout_shared/tesactorbasedata.rs` on
  `codex/phase2-reference-predicate`, rebased on merged #141 at `c56a2d1` (checkpoint `e89af6b`).
  Its source checkpoint is `3c3bedb`, with Rust checks still pending. It drafts that true
  predicate and the following false predicate `0047c850`, confirmed from
  the real reference vtable and load-buffer call chain. Twelve native cases
  pass; independent PC/vtable/test-wiring review found no mismatch. Three
  new Rust regressions have not run. Private
  `check-reference-probe.ps1` builds debug and reruns the item probe first;
  `check-reference-predicates.ps1` supplies the later full check chain.
  No whole-save claim.
- Water has 34 of 37 normal bodies drafted; `0049c930`, `0049db60` and
  `0049e410` are still being translated. The initial water agent is
  interrupted; `/root/a04_water_complete` owns and audits its uncommitted
  source, including a still-incomplete early-exit draft for `0049db60`.
  Do not collect or reset this slot before the full normal paths and checks
  finish. The replacement also restores the external predicate call in
  `0049c860`, which the first agent incorrectly inlined while editing its
  description. Fifteen native comparator/plane
  cases pass. Review corrected array callback reloads, the membership
  argument, fourth-corner Y selection and NaN classification. The apparent
  constructor-size and uninitialized-pair conflicts were decompiler
  artifacts resolved against PC instructions; no guessed substitute remains.
- a04's collected I/O management batch has forty bodies and fifty tests,
  with twelve SEH-only partials. All checks passed. Review
  corrected callback argument words, live queue/array/count reloads, signed
  construction loops and the incompatible Xbox layout. A suspect Xbox
  name for `005a8040` was removed in favor of a neutral PC accessor.
- Build 40 is current, with no confirmed next-play-build date. These leaf
  translations alone do not justify a play publication. Continue cycling
  until stopped; retain the user's 98% usage handoff rule.

### Stopped at the requested 98% usage limit, 2026-10-11 05:40 local

Translation is stopped for handoff. All three agents were interrupted;
no cargo/rustc process remains. Preserve their uncommitted files: do not
run `task.sh`, reset a slot, or cycle it before finishing its checks and
committing its owned file. On an authorized resume, restart the agents on
their existing task files with these continuation notes, then resume the
continuous collect/cycle/fresh-agent loop and ship passing rolling PRs.

1. Check attached rolling #134 at `a762d6f91915812c2dc8f31a71a54d57e51001fa`
   and persistence #135 at `86d19a0b2f8343a28afd9c334fdde36ed7b9aa69`.
   Both core/viewer CI checks were still running at 05:38; neither merged.
   Ship #134 through `ship.sh 134` once both current-head checks pass.
   Merge #135 only after both checks, using its exact head. #132 merged
   `4ba17e6` and #133 merged `c975484`, both verified after passing checks.
   Lead has later local documentation commits; the ship script preserves
   them during rebase. Do not push them over the in-flight PR unnecessarily.
2. a04, batch b0677: only `fallout_shared_magic/effectitem.rs` is dirty.
   All 34 remaining normal bodies are drafted through `00405400`.
   Focused tests 78/78, full engine 14,129/14,129, doctest, strict clippy and
   formatting passed. The hygiene test was interrupted without a result;
   rerun it, inspect the diff, then commit and cycle04 exactly once.
   No active session remains (70580/91380/28464 finished). Ten explicit
   SEH-only partials remain; partial-only advancement is reviewed/allowed.
   Immediately start a fresh agent on the new task after collection.
3. a03, batch b0236: only `fallout_interface/hudmainmenu.rs` is dirty.
   The remaining normal body of `00775a00` is drafted through `0077886d`,
   with reviewed callback reloads, float storage, signed comparison,
   constructor-return and NaN fixes plus regressions. Direct rustfmt,
   diff and name scans pass; cargo checks have NOT run. Validate, commit,
   cycle03 and immediately start the fresh continuation. SEH stays partial.
4. a01, batch b0155: only `fallout_ai/mobileobject.rs` is dirty. Nine normal
   bodies are drafted from `00935f60` through `00cbc510`; seventeen total
   SEH-only partials remain. Reviewed mode reloads and face/activator call
   argument fixes have regressions. Source-only checks pass; cargo checks
   have NOT run. Validate before commit/cycle01, and review the remaining
   queue before opting into partial-only advancement.
5. Root's managed checkout `%USERPROFILE%\.codex\worktrees\phase2-extra-data\wt-phase0`
   is clean on `codex/phase2-inventory-script-locals`, commit `86d19a0`.
   #135 contains five helpers/ten regressions; all 15,876 workspace tests,
   strict lint/fmt, release/debug builds and ten native cases passed.
   Applied retained stash `c11039c` must never be applied again. Next work
   is the real Script form/save-ID fixture and interpreter action-list
   path, starting `005e1160`/`005e13a0`/`005e1550`. Six generated native
   parser cases pass; private `codex-audit/inventory-script-next.md` has
   the details. a04 owns its required TLS guards; do not duplicate them.

Build 40 remains the current play copy. Item payload evidence (10,201
count/health entries, eighteen multi-stack entries, three full-record decode
failures excluded) does not establish a whole-save round trip or playable
integration. No confirmed next-build date or feature list. Prior automatic
approval review blocked idle p3 target cleanup with only "blocked by policy";
no deletion was performed for that rejected action. Do not retry/bypass it.

Latest, 05:35 local: root's five script-loading helpers are committed and
pushed as `86d19a0` on `codex/phase2-inventory-script-locals`; attached
PR #135 is open with fresh core/viewer CI. The checkout is clean. All local
checks passed: 1,320 focused tests, 15,876 workspace tests including doctests
(14,023 engine unit tests), strict lint, formatting, release and debug builds.
Session `33662` is complete. Native evidence and the next parser's six cases
are recorded in `ENGINE_STATE.md` and private `inventory-script-next.md`.

`/root/a04_round15` now owns the compiler for its frozen 34-body batch.
After a clean commit, cycle04 once with reviewed `ADVANCE_PARTIAL_ONLY=1`
if only its ten verified SEH-only partials remain; immediately start a fresh
worker, compiler-held. Then grant `/root/a03_round17` its remaining normal
HUD body, followed by `/root/a01_round16`'s nine remaining normal movement
bodies. Those two drafts are uncompiled and restricted to their own unit
files. a01 now has seventeen total SEH-only partials; the earlier estimate
of six remaining normal bodies was incorrect, superseded by the ledger.
Its callback mode reloads and pending argument corrections are drafted
with regressions. The source-only formatting/hygiene scans pass.

Rolling #134 remains open at `a762d6f`, both CI checks still running.
Ship with `ship.sh 134` when both current-head checks pass. #135 also needs
both checks before merging. Usage is 97%; stop and hand off at 98%.
No new play copy is warranted by these unconnected engine dependencies.
Build 40 remains current, with whole-save/world integration still pending.

Latest, 05:26 local: #132 merged `4ba17e6` and rolling #133 merged
`c975484`, each after core and viewer CI passed. `ship.sh 133` completed
and created attached #134 at `a762d6f`, covering actor dialogue and HUD
target information. Its fresh CI must pass before shipping. Movement source
`d3269c3` was collected through cycle01; the rebased collection is
`c95399b`, with 14,126 combined engine tests, doctest, strict lint and
formatting passing. HUD collection is now `c9a17b4` after that rebase.
Fresh `/root/a01_round16` immediately continued at `00935f60`; sixteen
earlier SEH-only partials remain explicit and six normal bodies remain.

Root owns the heavy compiler, session `33662`, running the private
`codex-audit/check-inventory-locals.ps1`. Focused tests pass; workspace,
lint, formatting and release/debug builds are still in progress. The root
checkout is on `codex/phase2-inventory-script-locals`, based on #132's
merge, with only `tesscript.rs`, `tesscriptrunmanager.rs` and
`ENGINE_STATE.md` changed. Scoped stash `c11039c` is applied and retained;
never apply it again. a04's frozen 34-body batch gets the compiler next,
then a03's remaining HUD body. Both are drafted and uncompiled. The HUD
review fixes cover callback reloads, float storage, signed target types,
constructor return pointers and unordered comparisons.

Six further native cases for the next parser `005e13a0` pass, checking
signed opcode/length words, reference-prefix behavior and RET24; this
parser is still untranslated. Private `inventory-script-next.md` records
the evidence. Usage 94%; stop and hand off at 98% as requested. Build 40
remains current, without a confirmed next-build date or feature list.

Latest, 05:19 local: HUD source `6c4df88` was collected as `0eb6b3c`
through cycle03, passing 14,100 combined engine tests, doctest, strict lint
and formatting. Fresh `/root/a03_round17` immediately continued and has
drafted the remaining normal `00775a00` paths; review is correcting late
HUD reloads and float storage boundaries. No cargo grant for that worker.
`/root/a01_round15` owns the compiler for twenty movement/dialogue bodies:
lint and formatting pass; one fixture correction is undergoing tests.
After commit: cycle01 exactly once, immediately fresh worker, then root
compiler for `codex-audit/check-inventory-locals.ps1`, followed by a04/a03.
Root has five helpers and ten Rust regressions, still uncompiled; all ten
native constructor/manager cases pass. Both #132 (`6513c26`) and #133
(`f264324`) have passing core CI and pending viewer CI. Ship when both
current-head checks pass. Build 40 remains current with no confirmed ETA.

Latest: rolling #131 passed both CI checks and merged `7cda9bf` at 04:57
local. `ship.sh 131` completed and created attached #133 at `f264324`,
covering movement persistence and effect-item routines. Its CI is fresh.
Effect collection is now `6cdf2b2` (source `7cc22ff`), with 14,095 combined
engine tests plus doctest, strict lint and formatting passing. Fresh
`/root/a04_round15` immediately started the remaining normal effect-item
entries at `00404a80`; its 34-body draft is ready and compiler-held.

a03 owns the compiler for its large HUD partial, followed by cycle03/fresh
worker, a01's twenty-body dialogue/movement batch and cycle01, root's
persistence checks, then a04. Root's script follow-on now contains five
dependencies and nine prepared tests in `tesscript.rs` and
`tesscriptrunmanager.rs`, plus `ENGINE_STATE.md`. Six native constructor
vectors pass; independent source review found no mismatch. Rust checks
are pending. Interpreter `005e1160` and action-list semantics remain open.
PR #132 core passes, viewer CI is pending. Build 40 remains current.

Latest, 04:49 local: persistence follow-on #132 is published and attached at
`6513c26`, with three dependency helpers and four regressions. All 15,781
workspace tests (13,928 engine), strict lint, formatting and release/debug
builds pass. Both probes now pass using the normal generated registry:
the synthetic two-stack payload and 10,201 original count/health-only item
payloads from eight saves, including eighteen multi-stack entries. Three
containing records that fail full decoding remain excluded. Generated form,
map, mode and partial allocator startup are explicit boundaries; this is
still not whole-save or playable inventory acceptance.

Root's clean published map branch has a new follow-on branch,
`codex/phase2-inventory-script-locals`: constructor `005a8b80` and two tests
are drafted in `tesscript.rs`, uncompiled. Three guarded native vectors pass.
Do not claim script extras work: their actual script forms and save-ID maps
are not yet supplied by the broader exploratory fixture. a04 owns the
compiler; a03, a01, then root follow their respective collect/cycle steps.
Rolling #131 core passes and viewer CI remains pending at `d6dcc8e`.
Build 40 remains current, with no confirmed next-build date or feature list.
Usage 84%; retain the 98% stop/handoff rule.

Latest, 04:33 local: movement collection `757af70` passes 14,051 combined
engine tests, doctest, strict lint and formatting. Fresh `/root/a01_round15`
immediately starts its remaining normal bodies at `009339a0`; ten SEH-only
partials remain explicit. Root now owns the compiler, session `57033`, for
the persistence follow-on; a04 then a03 follow. a03's next HUD body remains
explicitly partial beyond `00776731`; reviewed argument/global reload fixes
are in its draft. a04's forty-body effect-item batch is ready, uncompiled.

Reader #130 merged `bad2f96` after both CI checks. Rolling #131 is attached,
head `d6dcc8e`, rebased onto that merge with only generated-ledger conflicts;
fresh core/viewer CI is required. Later movement `757af70` remains local.
Root branch `codex/phase2-inventory-form-map` is based on merged #130;
stash `a4d2a67` is applied and retained, never apply it again. Its three
helpers/four tests cover form-map allocation and a shared buffer setter.
The draft count/health probe reads eight real saves and rewrites all 10,201
selected item payloads byte-identically, including eighteen multi-stack
entries; three full-record decode failures are excluded. This is not a FOS
round trip. The broader inventory probe reaches missing `005a8b80` in script
loading; investigate next after this batch. Final generated-registry probes
remain pending after the build. Build 40 remains current. Usage 78%.

Latest, 04:22 local: HUD collection `5c12df2` passed combined engine tests,
strict lint and formatting, including the reviewed radiation audio correction.
Fresh `/root/a03_round16` immediately continues large `00775a00` in b0236.
The publication hold is released: publish the accumulated movement, HUD and
explosion batch after regenerating the ledger. a01 now owns the compiler for
its forty-body continuation; then cycle01/fresh worker, root's two map helpers,
then a04's forty effect-item routines. Effect-item review includes thirty-six
matching native float vectors and callback reload/argument ownership fixes.
Reader #130 core CI passes; viewer CI remains pending at `2b81e2a`.
Build 40 remains current, without a confirmed next play-build date or feature
list. Inventory stack preservation still has no playable consumer. Usage is
75%; retain the 98% stop/handoff rule.

Latest, 04:09 local: explosion collection `5380adf` passes 13,990 combined
engine tests, doctest, strict lint and formatting. Reviewed partial-only
advancement retains all seventeen exception-cleanup gaps. Fresh
`/root/a04_round14` immediately started b0677, effectitem.cpp, compiler held.
a03 now validates its eight-body HUD batch plus radiation audio correction;
then cycle03/fresh worker, a01/cycle01 and root's map follow-on. Keep the
later rolling push held until the HUD correction is collected. Root's
separately compiled map draft passes the full item load and byte-identical
41-byte rewrite; formal generated-registry validation is still pending.

Latest, 04:00 local: rolling #129 passed both checks and merged `1389dab`
at 03:56:28. `ship.sh 129` completed with `HOLD_LATER_PUSH=1`: the later
movement/HUD collections are rebased locally but unpublished. Do not ship
the queued HUD until a03's correction to `007738c0` is collected: its audio
singleton getter had been given another callee's pending stack arguments.
After that collection, regenerate the ledger and publish the next rolling
PR (no rolling PR is currently open). HUD collection is now `f34752a`.

Inventory reader #130 is open/attached at `2b81e2a`, with fresh CI. Local
checks on merged #128 passed 15,771 workspace / 13,918 engine tests, strict
lint, fmt and release/debug builds; it rebased cleanly onto #129. The
buffered two-stack reader probe passes, consuming all 41 bytes. The fuller
ItemChange probe stops while inserting its generated form into the real
map: allocator `0043a010` is missing. No byte-identical rewrite result yet.
Root is now on `codex/phase2-inventory-form-map`, based on #130, with two
helpers and three tests drafted in `fallout_shared/form_map_support.rs`
and its generated `mod.rs`, uncompiled. The allocator's refill `0043a080`
is included; PC evidence is private `codex-audit/inventory-map-*`.
a01 is reviewing this draft. a04 owns the heavy compiler, then cycle04,
a01/cycle01, a03/cycle03; root's map follow-on waits. a03's next collection
is eight normal bodies plus the radiation audio fix; three new SEH partials
are explicit. Large `00775a00` remains wholly open for a fresh continuation.
Build 40 remains current. Usage was 66% at 03:48; stop/handoff at 98%.

Latest, 03:46 local: inventory writer #128 passed both checks and merged
`14c77da` at 03:39:58. Root reader checkout is rebased onto that merge;
scoped stash `9ca22a2` is applied and retained (do not apply again).
Cycle03 completed HUD collection `74cd13b`: 13,970 combined engine tests,
one doctest, strict lint and formatting pass. Fresh `/root/a03_round15`
immediately continues at `00774b00`; preserve Create/DoIdle SEH partials.
Root now owns the compiler for `check-inventory-load.ps1`, session `3847`,
then the buffered-reader and full ItemChange payload probes. Both probes
remain unrun. a04 follows, then a01; both have reviewed drafts ready.
a01 has forty new normal entries through `00933890`, with sound argument
arity and signaling-NaN store corrections. a04 has both remaining normal
explosion bodies; reviewed partial-only advancement remains authorized
after successful checks. Rolling #129 core passes and viewer is pending
at `12f6153`; ship when both pass. Build 40 remains current, without a
confirmed next play-build date or shipped inventory consumer.

Latest, 03:30 local: #127 passed both CI checks and merged `72003ab` at
03:23:48. `ship.sh 127` completed; #129 is attached at `12f6153`, titled
"Engine: translate explosion effect processing", with fresh CI. That
explosion collection is now `dc89abd` after rebasing. #128 core passed;
viewer remains pending. It can merge at its exact checked head independently.
Movement source `6ae7a25` passed 65 focused / 13,889 source engine / doc /
strict lint / fmt / 49 hygiene tests. Cycle01 completed as `c757035` with
13,925 combined engine tests plus doc, lint and fmt. Fresh `/root/a01_round14`
immediately started the next normal entries at `00931880`; four reviewed
SEH-only partials remain in the ledger. a03 now owns the compiler, then
cycle03, root reader/probes, and a04. a04 has both remaining normal bodies
drafted and reviewed, with a full blocked-then-clear ray test; all seventeen
remaining partials have verified PC exception frames. After its checks,
partial-only lane advancement is reviewed/allowed. Root's fuller ItemChange
probe now requires byte-identical payloads after loading and writing again;
it has not run. Usage is 60% at 03:27; the 98% stop rule remains active.

Rolling #126 passed both checks and merged as `97e7572` at 02:47 local.
`ship.sh 126` completed; attached rolling #127 is at `cb42c4d` with fresh
CI, containing MobileObject collection now `7a1f056` and loader collection
now `43a8ee1`. Save-buffer #125 passed both checks and merged `68387b4`
at 03:04 local. Inventory writer follow-on is open/attached as #128 at
`4b68ea0`, with fresh CI after rebasing onto #125. Its branch is
`codex/phase2-inventory-payload`. Final session `30664` passed 15,691 core tests (13,838
engine), strict lint, formatting and release/debug builds. Seventeen
dependencies now let the real-registry writer produce a 41-byte payload
with delta -2 and two separate count/condition stacks (2/42.5 and 7/17.5).
The payload decoder consumes it fully and the save-ID map resolves the
item correctly. No FOS file, native load round trip or world consumer yet.
Private evidence: `codex-audit/inventory-buffer-*` and
`inventory-next-native*`; ten generated native vectors pass.
The managed phase2-extra-data checkout now uses `codex/phase2-inventory-load`,
based on #128. Its two dirty unit files are `bgsloadgamebuffer.rs` and
`bgsloadformbuffer.rs`: eleven reader helpers and eight tests, uncompiled.
Peer review is clean and ten generated native reader vectors pass.
`inventory-load-probe.rs` and its runner are prepared, unrun. The fuller
`inventory-item-load-probe.rs` also exercises the ItemChange loader through
the real form map and installed RTTI, with a generated TESBoundObject header;
it is prepared but unrun. `check-inventory-load.ps1` serializes root checks.
Scoped stash
`9e0dd23` has been applied and retained; do not apply it again.

Latest batch: animation loader source `99fb056` collected as `6c569b7`;
13,879 combined engine tests, strict lint and formatting pass. All 44
unit entries have normal behavior; only SEH cleanup at `005f21c0`,
`005f22d0`, `005f3a20` remains partial. Reviewed partial-only advancement
retains these gaps. Fresh `/root/a03_hudmainmenu` immediately started b0236,
HUDMainMenu, compiler held. Explosion `009ae6e0` source `64750e3` was
collected as `b9871e9` at 03:13 local: 113 focused, 13,830 source engine,
49 hygiene and 13,885 combined engine tests plus doc, lint and fmt pass.
Only compiler SEH cleanup is partial. Fresh `/root/a04_round13` immediately
started `009b00a0`, then `009b1810`, compiler held. a01 now owns the compiler;
then a03 and root. All six larger movement bodies have drafts and tests;
Move is now registered after the vector/control-flow review and 65 focused
tests pass; full engine checks are underway. a03 now has 40 registered HUD
bodies and 45 tests, Create and DoIdle SEH-only partials, and awaits its
compiler grant. Both remaining explosion bodies are drafted and under review.
ProcessTargets has no PC exception frame; its mistaken SEH partial marker
is being removed. RunLOSPick retains its actual compiler-cleanup partial.
Review corrections include vector
receiver/output argument order, post-callback globals and x87 boundaries.
Root reader validation waits after the slots. Build 40 remains
current; no reliable next-build date or confirmed feature list exists yet.

User resumed after the usage reset on 2026-10-11. Continuous translation
is active: collect each finished, committed slot with `cycle.sh NN 0 0`,
then immediately start a fresh agent on its new task. Never reset or cycle
a slot with unfinished work. Ship rolling PRs once both current-head checks
pass. The saved 98% usage stop/handoff rule still applies.

Rolling #124 passed both checks and merged as `c6ac7eb` at 02:12 local.
`ship.sh 124` succeeded; rolling #126 is open/attached at `9343898`, with
animation collection now `00a246f` and Init3D now `ca298df`. Fresh CI gates it.
Save-buffer PR #125 is based on #124, now at `685c26c`, with fresh CI.
Its generated-ledger delta was removed so rolling integration owns that
shared snapshot and independent source PRs can merge without count conflicts.
Source is unchanged. Follow-on safety stash `a7e3ae0` was reapplied and
retained; do not apply it again.
Its 18 dependencies pass 15,641 core tests (13,788 engine), strict workspace
lint, formatting, release and debug builds. The real-registry probe writes
count 5 / health 42.5 and the FOS payload decoder reads exactly both back.
It uses the executable's flag table, a generated mode receiver and partial
allocator startup; no game-function doubles, FOS file or full round trip.
Private evidence is in `phase0/codex-audit/save-buffer-probe-result.txt`.
Root session `37952` finished. Init3D source `d4fabeb` collected as
`407caf3`: 107 focused / 13,760 source / 13,824 combined engine tests,
49 hygiene tests, strict lint and formatting pass. Cycle04 session
`77634` completed and fresh `/root/a04_round12` immediately started
`009ae6e0`. a01 now owns the compiler; then cycle01, a03/cycle03, root.
Cycle01 has since completed as `8f92810`: 34 MobileObject functions,
13,864 combined engine tests, strict lint and formatting pass. Source
`f7e4c04` passed 40 focused, 13,830 engine and 49 hygiene tests. Fresh
`/root/a01_mobileobject_cont` immediately started the six remaining larger
bodies in address order, with compiler held. a03 now has the exclusive
compiler for its five-body loader batch; then cycle03, then root inventory.
a04's `009ae6e0` normal-path draft remains unregistered during review:
callback snapshots, x87 stores, distance comparison and contact-sort
pointer fixes have regressions prepared, uncompiled. Start fresh workers
after each cycle. Root's 16-helper/15-test inventory follow-on and private
two-stack payload probe remain prepared, uncompiled.

Reference #122 and rolling #123 passed both checks and merged as `6bea9a1`
and `8b5eebe`. Rolling #124 is open/attached at `9c519be`, containing loaded
area bounds (`e61c6a9`, formerly `86886d9`): 13,790 combined engine tests,
strict lint and formatting passed before rebasing. Fresh core/viewer CI
gates this head. Later animation collection `6dbf98f` stays local behind it
and passes 13,820 combined engine tests, strict lint and formatting.
Merged rolling PRs: #109 `b995218`, #112 `d854fbe`, #114 `89e1a0b`,
#116 `c23f744`, #119 `951c0c7`, #120 `3a92d0a`, #121 `eda98da`,
#123 `8b5eebe`.
Leave unrelated contributor PRs alone.

Slot state:

- a01 `/root/a01_mobileobject`, `agent-01`: fresh b0155 worker owns
  `fallout_ai/mobileobject.rs`, 34 functions/tests prepared with compiler held.
  Six larger bodies remain open: `0092ec40`, `0092ed60`, `0092f260`,
  `009308f0`, `00930c70`, `00931710`. Reviews corrected getter results,
  callback field reloads, a spurious vcall argument and x87 quieting;
  eligible cleanup and controller show/hide tests were added.
  LoadedAreaBound source `44c5bda` was collected as `86886d9`: 29 complete,
  three SEH partials, 34 focused / 13,758 source engine / 49 hygiene tests,
  strict lint and formatting pass. Reviewed argument arity, callback
  reloads and wrapping pointer arithmetic are corrected. Its cycle wrote
  the new MobileObject task and a fresh agent started immediately.
  cycle01 collected `a902f84`: `00699b60` normal path and the `0069ad40`
  immediate-vtable correction pass 121 focused and 13,724 combined engine
  tests, lint and formatting; the source also passed 49 hygiene tests.
  Exactly three NavMesh SEH partials remain: `006987d0`, `00699b60`,
  `006bd040`. Reviewed partial-only advancement retains them in the ledger.
- a04 `/root/a04_round12`, `agent-04`: fresh worker after cycle04 collected
  `d781c3b`. `009ae280` and `009b2940` normal paths remain SEH-partial;
  98 focused and 13,722 combined engine tests, lint and formatting pass.
  The source commit also passed 49 hygiene tests. Five larger paths remain
  untouched: `009acd60`, `009ad670`, `009ae6e0`, `009b00a0`, `009b1810`.
  `009acd60` is now collected as `e4de812`, source `6d0bb52`: five focused
  tests, 13,727 source engine tests, strict lint/fmt and 49 hygiene pass.
  Argument order, callback reloads, impact-object naming and x87 stores
  were reviewed against the PC instructions. It remains SEH partial.
  `009ad670` is now collected as `407caf3` with four tests; its fresh
  worker starts `009ae6e0` with the compiler held. `009b00a0` and
  `009b1810` remain untouched. Later-address evidence is in
  private `agents/scratch/a04/round9-009b1810-handoff.md`. Research/editing
  only until the compiler grant. Root PC review caught action-record
  offsets and CString argument arity; both are corrected, and x87 stores
  now have signaling-NaN coverage. The draft adds a fifteenth SEH partial.
  Prior worker reviewed root save-buffer code read-only with no further issue.
- a03 `/root/a03_loadanimgroup`, `agent-03`: fresh b0041 worker owns
  `fallout_shared/tesanimgroup.rs`, continuing the full `005f3a20` normal
  path with compiler held. Five new bodies and 14 tests are prepared;
  static checks pass (15 new tests, 28 total in the file). Peer review corrected note-getter callback reloads,
  empty-note log arity, sound error scanning and tail diagnostic reloads.
  The file has 41 complete bodies and three SEH partials including the
  loader; runtime checks remain queued. Its predecessor committed `f85e4d8` and cycle03
  collected `6dbf98f`: focused 13 / full 13,764 / hygiene 49 pass, strict
  lint/fmt pass. Seven queue entries remain, including two SEH partials.
  37 complete bodies and two SEH partials are registered. `005f3a20` remains
  unregistered/open; its full normal path is unfinished, with private
  `scratch/a03/005f3a20-handoff.md` for the next cycle. Reviews corrected
  array callback reloads, table stride, an erroneous constructor cleanup
  call, x87 stores and filename-parser callback ordering. Thirteen native
  leaf vectors passed; every registered address has test coverage.
  cycle03 collected all 30 multibound entries as `20c6123`; source
  `0947e87` passed 27 focused / 13,744 engine / 49 hygiene tests, strict
  lint and formatting. Combined lead tests pass 13,751. Seven generated
  shift cases and one matrix signaling-NaN case also passed the native
  executable. Three exception-cleanup partials remain: `004391c0`,
  `00439410`, `00439490`. The previous pathmanager lane retains
  17 exception-cleanup partials in the ledger; cycle advanced only after
  confirming no untouched entries, using `ADVANCE_PARTIAL_ONLY=1`.
- a05 b0158 and a06 b0360 remain prepared but idle. Runtime permits three
  children currently; the user permits up to six.

Lead Phase 2 work:

- ExtraData list #115 merged `94165e8`; count #117 merged `cbca26e`;
  health #118 merged `a0ee61d`, each after both CI checks. The real-save
  payload probe checks 20 health and 14 count values across 3,319 lists /
  3,365 forms with the source unchanged. This is not a save round trip.
- Managed checkout `%USERPROFILE%/.codex/worktrees/phase2-extra-data/wt-phase0`
  now uses follow-on branch `codex/phase2-inventory-payload`, based on #125.
  The checked save-buffer branch remains pushed separately at `574c3c5`.
  Inventory follow-on has 16 new helpers and 15 tests prepared but
  uncompiled: form-ID encoding, bidirectional maps, constructors and
  callback allocation/release. Root owns its twelve unit files; no other
  agent may edit them. `git status` lists the exact files. Seven native
  generated vectors pass (created-ID decoding and key equality), and
  independent static review of the first fifteen bodies found no concrete
  mismatch. The last helper constructs a list node (`00470440`), checked
  against PC assembly. Private `inventory-buffer-probe.rs` is prepared but
  not run; it writes one entry with delta -2 and two distinct count/health
  stacks, then uses the FOS inventory decoder. No world changes.
  Safety stash `063e7833f97201d3ff909bb96784ddd3f0050afb` was applied after
  rebasing the follow-on onto updated #125; retain but do not reapply it.
  PR #122 (`codex/phase2-reference-state`) merged as `6bea9a1` after both
  checks passed on head `8e11001`. Its ten form helpers,
  nine allocator/counter helpers, form callback-reload corrections and the
  new Reference adapter pass 15,587 core tests (13,734 engine), workspace
  clippy with warnings denied, formatting, release and debug builds.
  Native spot checks: five header, two zero-capacity array and three
  allocator-header vectors. The follow-on is now based on merged #123.
- The allocator test failure was a fixture expectation: `Mem::alloc` rounds
  to eight bytes. The fixed test verifies requests of 4/13 separately from
  backing blocks of 8/16. Twenty allocator-focused tests pass.
- Private `phase0/codex-audit/reference-bootstrap-allocator.rs` passes real
  registry construction of TESObjectREFR, embedded list, count/health updates
  and count removal while preserving health. It initializes only the PC
  allocator sequence through `00aa2020`, not full BSSystemUtility startup.
  No game-function doubles or game-file writes. No public creation API yet.
- `reference.rs` plus `lib.rs` add an existing-reference handle and embedded
  list accessor. Its generated isolation/aliasing regression and updated
  installed-executable probe pass. Independent source review found no
  concrete adapter issues. World consumers, payload conversion and FOS
  writing remain open. Next research traces buffered extra-data saving
  (`00426a30`) and the currently scaffolded save-buffer constructors.
- The follow-on is committed in #125: form_support, bgsloadformbuffer,
  bgssaveloadinitialdata, bgssavegamebuffer, bgssaveformbuffer,
  bgsloadgamesubbuffer, run_008660b0, run_008661e0 and run_00ec7ce3. Eighteen
  constructor/getter/writer dependencies and generated tests pass validation
  in completed root session `37952`; six guarded constructor vectors passed the native executable.
  Byte writing, growth, insertion and variable-size value paths are checked;
  CRT checked movement at `00ec7ce3` is checked with three native
  generated cases passing. Its error handler/TLS callees remain untraced;
  error-path tests use doubles. Private evidence uses the
  `codex-audit/save-boundary-*` and `save-buffer-headers-native*` files.
  `ENGINE_STATE.md` and the ledger are committed. Private `save-buffer-probe.rs` uses real
  executable tables, a generated mode receiver and the translated writer,
  then checks the bytes through the FOS payload decoder; it passes.

ONE heavy engine compiler across all worktrees, `-j2`, existing debug cache,
`CARGO_PROFILE_DEV_DEBUG=0`. a01 MobileObject owns the compiler; root
save-buffer session `37952` and cycle04 `77634` completed. Next: cycle01,
then a03/cycle03 and root inventory follow-on. Disk is about 2.6 GB free.
Automatic approval review previously blocked
idle p3 target deletion; nothing was deleted. Do not bypass that denial.
Private task.sh warns rather than deleting caches and retains scratch.
Private ship.sh now safely recreates an absent merged branch with an empty
lease, refuses an unexpected nonempty remote head, and avoids a push when
there are no later commits. After a transient query failure during #120
shipping, it also explicitly stops if the remote-ref query fails. #120's
merge and #121's published head were independently verified. Syntax passes.

Build 40 remains the published play copy from merged Phase 1 PR 7 (#113,
`f917219`). Core/viewer checks and three scripted routes passed: Doc 63 s,
VCG02 410 s, VMS16 339 s. App/source SHA-256 matched; WHATS-NEW is LF.
Evidence: `%USERPROFILE%/nv-re/acceptance/build40-main-20261010`.
No original-game comparison was performed. Exact menu-input/GPU renderer
boundaries remain in FRAME_SKELETON.md. These translation batches do not
establish new playable features.

Next action: publish the checked rolling batch and collect a01 after
validation. Merge #122 after both current-head checks pass, and continue
collecting and immediately restarting finished translation slots.

## Where things stand

- **Phase 0** (engine map, ledger) is merged (#72). `docs/LEDGER.md` is the
  coverage table; regenerate with
  `cargo run -q --release --manifest-path scripts/ledger/Cargo.toml`.
- **Engine crate** (`crates/engine`, ADR-0006): current function coverage
  and explicit partial counts are in LEDGER.md, each translation with a
  `// Translated from <addr>` marker and tests. They run in the crate's
  emulated 32-bit memory, not yet in play.
- **Previously translated units (dependency closure is not established):** main.cpp, actor.cpp (all 7 parts), playercharacter.cpp
  (all parts), interface.cpp, interfacemanager.cpp, processlists.cpp,
  tesobjectrefr.cpp, extradatalist.cpp, extradataobjects.cpp, tesscript*.cpp,
  tesform, tesfile, tesland, tesworldspace, teswater, tesnpc, package,
  inventorychanges, bipedanim, animation, modelloader, tesobjectcell,
  bgsdecalmanager, navmeshobstaclemanager, globalfunc, pathfind.cpp,
  BSMenu/tile.cpp, highprocess.cpp parts 1, 2, 4 and 5, navmesh.cpp,
  pathinglocation.cpp, bgssceneinfo.cpp, bgssaveloadgame.cpp, and others.
- **In progress (translation lanes; see "Resume here" for current owners):**
  loadedareabound.cpp (a01, prepared), tesanimgroup.cpp (a03, active), explosion.cpp
  (a04, active), aitaskmanager.cpp lane 2 (a05, prepared; partial behavior
  listed in the file header), bhkworld.obj (a06, prepared).
- **Phase 1 (frame skeleton) wiring:**
  - PR 1 frame map (#84), PR 2 `world::frame` (#86), PR 3 Bevy order from the
    frame (#90): merged.
  - PR 4 player stage (#101): merged. It added `world::frame::player` and
    `frame_order::PlayerSet`.
  - PR 5 world and time stage (#105): merged.
  - PR 6 AI task stage (#107): merged.
  - PR 7 interface and render (#113): merged as `f917219` after both checks.
    Acceptance passed; exact open boundaries remain in FRAME_SKELETON.md.
  - Phase 2 (state model in engine memory, save round trip; see
    ENGINE_PORT_PLAN.md).
- **Rolling translation PR:** #109, #112, #114, #116, #119, #120 and #121 merged; next batch on branch
  `claude/engine-crate` (see "Resume here").
- **Play build** (the user's `Desktop\nv-rs-play`): Build 40 = main `f917219` with #113.
  After the next wiring PR merges, build `viewer` in release from main
  (`cargo build --release --manifest-path viewer/Cargo.toml`), copy
  `viewer/target/release/nv-viewer.exe` into `app` (or `next` if the viewer is
  running) and add a plain-language entry at the top of `WHATS-NEW.txt` (LF
  line endings).

## How the translation machinery runs (private, not in the repo)

Everything private lives in `%USERPROFILE%\nv-re\work\phase0` (state notes in
`STATE.md`, scripts in `bin/`).

- Lead worktree: `%USERPROFILE%\nv-re\work\wt-phase0` (branch
  `claude/engine-crate`). Agent worktrees: `%USERPROFILE%\nv-re\work\agents\aNN`,
  task files `agents\tasks\aNN.md`, scratch `agents\scratch\aNN`.
- Private read-only Ghidra server for agents: 127.0.0.1:8090
  (`bin/start-srv.ps1`). Never write to the shared server on 8089.
- When an agent finishes: `cd ...\phase0 && bin/cycle.sh NN <tokens> <ms>`
  (collects its file, checks names, fmt, clippy, tests, commits, logs to
  `progress.tsv`, writes its next task), then launch an Agent (model sonnet,
  background) with: "Read the task file
  %USERPROFILE%\nv-re\work\agents\tasks\aNN.md and carry out the task it
  describes, exactly as written. It names your working directory, your files
  and the rules."
- Ship the rolling PR when both checks pass: `bin/ship.sh <pr>` (squash-merges,
  rebases, regenerates the ledger, opens the next PR).
- After any other PR merges, rebase `claude/engine-crate` onto main: on each
  stop where only `docs/LEDGER.md` conflicts, rerun the ledger, `git add` it,
  `GIT_EDITOR=true git rebase --continue`; check the rebase succeeded before
  running anything else; then rerun the ledger, commit, build, force-push.
- Concurrency: at most 6 agents (5 translators + 1 wiring agent). The machine
  has 31 GB and each engine compile takes 3-4 GB; agents pass `-j 2` to cargo.
  Slots listed in `parked.txt` are collected but not relaunched.
- `plan.tsv` is ordered by reachability from `Main::OnIdle` (0086e650).
  `pending_lanes.txt` holds lanes handed back; cycle.sh takes those first.
- Usage limit: the user's rule is to stop at 98% weekly usage, merge finished
  work and update this file. If agents die on a session limit, resume each
  with SendMessage after the reset (their uncommitted work stays in the
  worktree).

## Open items for the lead

- Platform stand-ins missing for Win32 imports the translated code calls
  (file search, registry, window, message box, semaphores, time; full list in
  phase0/STATE.md).
- 00aa13e0/00aa1070 allocate through the Gamebryo allocator object at
  `**0x011f6080`; the platform must provide it before running translated code.
- Shared layouts and helpers agents keep asking for: one `Actor`,
  `PlayerCharacter`, process (`HighProcess`/`MiddleHighProcess`) layout shared
  across part files (each part declares its own today), a `BSSimpleList`
  accessor set, `NiPoint3`/`NiMatrix3` helpers, x87 rounding helpers,
  `BSStringT`.
- Six functions Ghidra never defined (00435ac0, 00435bb0, 0041fdb0, 00420060,
  004203b0, 00420630): define them in the private project and rerun the engine
  map.
- 0084b720 (hash map base destructor body, vtable 0107f4c8) is called from
  bgssaveloadgame.cpp but sits in no unit's queue; it needs an owner.
- ADR-0006 awaits maintainer review.
- Stray scratch files agents left in the Git install folder
  (`C:/Program Files/Git/tmp_dummy`, `tmp_p3.rs`, `tmp_tests.rs`); deleting
  them there needs the user.
