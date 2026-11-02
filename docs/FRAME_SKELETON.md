# Phase 1: the frame skeleton (proposal)

Drafted 2026-10-09 at the end of Phase 0 ([ENGINE_PORT_PLAN.md](ENGINE_PORT_PLAN.md),
[LEDGER.md](LEDGER.md)); PR 1 (the frame map), PR 2 (`world::frame`), PR 3 (the viewer's order), PR 4 (the player stage), PR 5 (the world and time stage) done 2026-10-09 and PR 6 (the AI task stage) 2026-10-10.
PR 7's model and scoped wiring and PR 8's controls' buffer (2026-10-12) are described below, with remaining calls
explicitly open. Names are from the Xbox 360 prototype (Xbox PDB,
ADR-0002), PC addresses from `research/engine-map/engine_map.tsv` and
`research/engine-map/frame.tsv`.

## The entry point

The frame is `Main::OnIdle` (Xbox PDB): PC `0086e650`, 2,272 bytes, called
once per frame from `main` (`0086a850`, matched by its strings; the call is
at `0086b3e3`, its only caller). It was not named by the matcher; it is the
only PC function that calls both `Main::OnIdle_UpdatePlayer` (`0086f940`)
and `Main::OnIdle_PollControls` (`0086f390`), and its 143 call sites line
up with the Xbox `Main::OnIdle` (`8269f3c8`, 142 calls without the PowerPC
register save helpers) apart from profiling timers and debug-only calls.
The PC version polls the keyboard directly (`GetAsyncKeyState`, six calls).

## Its order (Xbox PDB names, PC addresses where matched)

Grouped into stages for readability; the order inside each stage is the
call order. Plain addresses come from the engine map; **bold** ones were
paired for the frame in Phase 1 PR 1 (evidence below; `CONFIRMED` in
`research/engine-map/src/bin/frame.rs`); *lead* marks a pairing by
position only; `?` means no PC counterpart was found. The full tree to
depth 3 is `research/engine-map/frame.tsv`, its coverage in
[LEDGER.md](LEDGER.md) ("Frame").

"Threads" below is the setting `iNumHWThreads:General` (`011c3ea4`, name
string at `0101801c`, static default 1 from `00f38360`), read through
`0043d4d0`. `main` sets it to `GetSystemInfo`'s processor count
(`0086a950`-`0086a966`) and raises 1 to 2 (`0086a977`-`0086a990`); whether
the INI can lower it afterwards is not traced. With threads > 1 `main`
creates the AI linear task threads (`0086b117`-`0086b12d`, below).

| Stage | Calls in order |
| --- | --- |
| 1. Frame start | `Main::UpdateTextures` ? (the PC's first call, `0086a830`, only returns 0), menu state queries (`Interface::IsInMenuMode` `00702360`, `IsPipboyOpening` `00709bc0`, `InDialog` `007050d0`, `FaderManager::IsFaderVisible` **`00701450`**, `IsConsoleVisible` `00703d50`, `GetCurrentRenderedMenu` `00707ad0`, `GetPipboy` `00705990`), `MemoryLevelManager::RunNonDestructiveFree` **`008782b0`** |
| 2. Player | `Main::OnIdle_UpdatePlayer` `0086f940`; PC only: the Steam API object (`006ff580`) and `SteamAPI_RunCallbacks` (`006ff860`); `Main::OnIdle_UpdateImageSpace` `0086fd90` |
| 3. Housekeeping | `Pathing::ProfilePathing` `006da7c0`, `BGSSaveLoadManager::UpdateQueuedSaves` `00851d90`, `BSConsoleCommand::PollForCommands` ?, `Main::OnIdle_FixActorBones` **`0086f190`**, `Main::OnIdle_UpdateMessageBox` ?, PC only: a requested `Sleep` of up to 20 ms (`004e1610`), `Main::OnIdle_UpdateTimer` **`0086f260`**, `Main::OnIdle_PollControls` `0086f390`, `IOManager::UpdateQueue` **`00c3dbf0`**, `LoadingMenu::SuspendBackgroundThread` `0078cfc0`, `Interface::IsInGameLoadingMenuOpen` `00705ea0`, `TES::ShowLoadingMenu` **`00457d70`**, `Main::OnIdle_ScaleLOD` **`0086efe0`**, `BSTextureManager::Update` *lead* `00b6dd00`, `Main::OnIdle_HandleMenuBackground` `0086f450`, `FaderManager::UpdateFaders` **`007011d0`**, `ScreenSplatter::Update` **`004e0110`**, `ScreenCustomSplatter::Update` **`004de600`** |
| 4. World and time | `NiParallelUpdateTaskManager::BeginUpdate` *lead* `00a81a20`, `TES::TestAllCells` `004556d0`, `Calendar::Update` **`00867a40`**, `TES::RunAnimations` **`00455640`** (threads = 1 only), `ProcessLists::RunActorScripts` **`00978550`**, `ProcessLists::UpdateRadiationList` **`009777a0`**, `ProcessLists::ChangeProcessLevelTempList` `0096eb40`, `ProcessLists::UpdateFollowerTempList` **`0096e9b0`**, `MemoryLevelManager::CheckMemoryLevel` ?, `BSTexturePalette::PurgeUnusedTextures` `00a61cd0`, `GarbageCollector::Update` `00868850`, `GarbageCollector::ClearTempEffects` `00868d10`, `BSTreeManager::Update` **`006652e0`**, `Main::OnIdle_UpdateCurrentGridCell` **`0086fbe0`** |
| 5. Interface and scene | `Main::OnIdle_DoInterfaceIdle` **`0086fd70`** (threads = 1 only; it calls `Interface::PreIdleStuff` **`007027e0`**, `Interface::Idle` **`00702810`**, `Interface::PostIdleStuff` **`00702840`**), `Main::DisplayDebugText` ?, `BGSDecalManager::GetInstance` **`0049fef0`**, `BGSDecalManager::UpdateDecals` **`0049fff0`**, `BSSceneGraph::SetCameraFOV` `00c52020`, `BSShaderManager::SetFOV` **`00b54000`**, `TES::ResetAllMultiBoundNodes` **`0045bc80`**, `ShadowSceneNode::UpdateOcclusionPlaneVisibility` `00b5ac90`, `TES::UpdateMultiBoundVisibility` **`0045b070`** |
| 6. AI threads start | threads > 1: `AILinearTaskThreadManager::SetMainRendering` **`008c80e0`**, `StartThreads` **`008c78c0`**; threads = 1: `AITaskManager::StartTasksDuringRendering` **`008ca070`**; then `Main::OnIdle_UpdateAnimationsAndEffects` **`0086fc60`**, `Main::OnIdle_DoInterfaceIdle` **`0086fd70`** again (threads > 1), `Interface::LastMinuteUpdate` `007058e0`, `PathManager::Update` **`006ebc50`**, `NavMeshRender::Update` **`006a61b0`**, `NavMeshObstacleManager::Update` **`006c3640`** and `CombatManager::Update` **`00991500`** (both threads = 1 only), `NiParallelUpdateTaskManager::EndUpdate` *lead* `00a81a80`, `Interface::UpdateSleeping` `007056f0` |
| 7. Render | `Main::RenderMenuBackground` `00871dc0`, `CheckWithinMultiBoundTask::DoAttachments` *lead* `0057ab70`, `ShadowSceneNode::ProcessAllQueuedLights` *lead* `00b60040`, `XGamerProfile::Update` ? (the PC calls `TESActorBaseData::GetAlignmentForKarma` `0047e040` there), `Main::Swap` **`0086ff70`**, `Main::PostSwapProcess` **`008705d0`** |
| 8. Threads join | threads > 1: `AILinearTaskThreadManager::WaitForThreads` **`008c7990`**; threads = 1: `AITaskManager::WaitForTasksDuringRendering` *lead* `008ca300`; `Main::UpdateNonRenderSafeAITasks` `0086f6a0`, `Main::OnIdle_PostThreadsProcess` **`00870610`**, `Interface::OpenConsole` `00703e10`, `Script::ClearOptimizations` **`005ae270`**, `ScriptLocals::ClearOptimizations` **`005a9d60`**; PC only: `00a29680`, the frame time (`0084d030`), `00950090`, `00aa7290` |

The remaining `?` rows are Xbox debug calls the PC does not make at this
level (`BSConsoleCommand::PollForCommands`, `Main::DisplayDebugText`,
`XGamerProfile::Update`) or were not found (`Main::UpdateTextures`,
`Main::OnIdle_UpdateMessageBox`, `MemoryLevelManager::CheckMemoryLevel`).
The Xbox build also brackets most stages with `BSPerformanceTimerInternal`
start/stop calls, which the PC build does not make.

### Evidence for the pairs

Each pair compares the PC function's direct calls (names from the engine
map), strings and unit with the Xbox function's.

- `FaderManager::IsFaderVisible` `00701450`: the only call between
  `InDialog` and `IsConsoleVisible`, both times, on both sides; `Main::Swap` calls
  it twice on both sides; unit `fadermanager.cpp`.
- `MemoryLevelManager::RunNonDestructiveFree` `008782b0`: calls
  `BSFaceGenManager::GetModelCache` three times and `Tile::GetMenuByClass`,
  as the Xbox function does; next to
  `MemoryLevelManager::FreeReleasedObjects` (`00878250`).
- `Main::OnIdle_FixActorBones` `0086f190`: calls
  `MobileObject::GetCurrentProcessType` and twice an empty function
  (`00483710`) where the Xbox calls the empty `RemoteLog::OnConnected` twice.
- `Main::OnIdle_UpdateTimer` `0086f260`: calls `bhkWorld::SetDeltaTime`
  (`00c66760`, docs/PHYSICS.md) and a `bstimer.cpp` function (`00aa4e40`),
  as the Xbox one calls `BSTimer::Update` and `bhkWorld::SetDeltaTime`.
- `IOManager::UpdateQueue` `00c3dbf0`: calls `BSPrecisionTimer::GetTimer`
  twice and an `iomanager.obj` function (`00c3e420`).
- `TES::ShowLoadingMenu` `00457d70`: 8 shared callees and strings; `Main::Swap`
  calls it on both sides.
- `Main::OnIdle_ScaleLOD` `0086efe0`: calls `TES::GetWorldSpace` twice and a
  `tesworldspace.cpp` function (`00586390`).
- `FaderManager::UpdateFaders` `007011d0`: the same `FaderManager.cpp`
  source-path string.
- `ScreenSplatter::Update` `004e0110` and `ScreenCustomSplatter::Update`
  `004de600`: both call `NiObjectNET::GetExtraData`; the second calls
  `004b3ab0` twice and the first once, as the Xbox custom splatter calls
  `FLerp` twice and the plain one once.
- `Calendar::Update` `00867a40`: unit `calendar.cpp`; calls `004b10d0`, a
  12-entry lookup by month, where the Xbox calls `Date::GetDaysInMonth`.
- `TES::RunAnimations` `00455640`: calls `00553820` (`tesobjectcell.cpp`)
  and `004baba0` (`gridcell.cpp`, which calls `GridCellArray::Get` and
  `00553820`), as the Xbox one calls `TESObjectCELL::RunAnimations` and
  `GridCellArray::RunAnimations` (so `00553820` and `004baba0` are those
  two).
- `ProcessLists::RunActorScripts` `00978550`: calls
  `TESObjectREFR::RunScript`.
- `ProcessLists::UpdateRadiationList` `009777a0`: `ExtraDataList::GetRadius`,
  `GetDistanceFromReference`, `Actor::GetRadiationResistanceMult`,
  `ShouldActorAvoidRadiation`, `HighProcess::AddAvoidPathingArea` and more,
  in the Xbox order.
- `ProcessLists::UpdateFollowerTempList` `0096e9b0`:
  `Actor::GetCurrentPackageTarget` twice, `Actor::AddFollower`.
- `BSTreeManager::Update` `006652e0`: `CSpeedTreeRT::SetCamera`, unit
  `bstreemanager.cpp`.
- `Main::OnIdle_UpdateCurrentGridCell` `0086fbe0`:
  `TES::UpdateCurrentGridCell`, `TESObjectREFR::GetWorldSpace`,
  `TES::GetWorldSpace`.
- `Main::OnIdle_DoInterfaceIdle` `0086fd70`: calls three wrappers
  (`007027e0`, `00702810`, `00702840`, which call
  `InterfaceManager::PreIdleStuff`, `InterfaceManager::Idle` and
  `00711ea0`), as the Xbox one calls `Interface::PreIdleStuff`, `Idle` and
  `PostIdleStuff`; it is also the first call of the combined AI thread
  function (`008c7bd0`), as on the Xbox.
- `BGSDecalManager::GetInstance` `0049fef0`: allocates (`00401000`) and
  constructs (`0049fcd0`); `UpdateDecals` `0049fff0`: calls
  `BGSDecalManager::UpdateSimpleDecals`.
- `BSShaderManager::SetFOV` `00b54000`: two sine/cosine pairs (`00eca0a0`,
  `00ec9f70`) as the Xbox `sin`/`cos`; unit `bsshadermanager.cpp`.
- `TES::ResetAllMultiBoundNodes` `0045bc80`: `BSCompoundFrustum::SetCamera`,
  `TESWorldSpace::GetTerrainManager`, `GridCellArray::Get`.
- `TES::UpdateMultiBoundVisibility` `0045b070`: `nicullingprocess.cpp`
  functions (`00a69400`, `00a694a0`), `GetTerrainManager`,
  `GridCellArray::Get`, `shadowscenenode.cpp` functions.
- `AITaskManager::StartTasksDuringRendering` `008ca070`: unit
  `aitaskmanager.cpp`, calls `MobileObjectTaskletData::RunToCompletion`
  twice, as the Xbox one does.
- `AILinearTaskThreadManager::SetMainRendering` `008c80e0`: the last call of
  `Main::Swap` on both sides. `StartThreads` `008c78c0` resets the 12 + 8
  event handles of the thread manager (`011dfa50`, returned by `00713d80`),
  sets the flag `011dfa19` and releases each thread slot's semaphore
  (`008c9fb0`, `ReleaseSemaphore`), as the Xbox `StartThreads` calls
  `ReleaseSemaphore`; `WaitForThreads` `008c7990` waits on each slot
  (`008c7490`, `WaitForSingleObject`, as on the Xbox) and clears the flag.
- `Main::OnIdle_UpdateAnimationsAndEffects` `0086fc60`:
  `ProcessLists::UpdateTempEffects` `00974420` (threads < 2) or
  `UpdateTempEffectsParallel` `009746c0` (which calls `00974420`, as on the
  Xbox), then `TES::UpdateCellAnimations` `00453550` or
  `TES::UpdateCellMainThread` `004537c0`, then
  `BSParticleSystemManager::UpdateParallel` (`00c50610`,
  `bsparticlesystemmanager.obj`).
- `TES::UpdateCellAnimations` `00453550`: wind (`00c468c0` in
  `bswindmodifier.obj`, `00c74550` in BSHavok) where the Xbox calls
  `BSWindModifier::SetWind` and `bhkWindListener::SetWind`;
  `TES::LockHavokUpdateMT` (`00453860`) before and after
  `TESObjectCELL::UpdateManagedNodes` (`00551890`) or
  `GridCellArray::UpdateManagedNodes` (`004ba9a0`); `TaskQueueInterface`
  (`0087aa90`), `focollisionlistener.cpp` (`00623640`),
  `Interface::InDialog`, `Sky::Update`, `TES::UpdateCellMainThread`,
  `processlists.cpp` (`00975080`): all in the Xbox order.
- `PathManager::Update` `006ebc50` (`pathmanager.cpp`) and
  `NavMeshRender::Update` `006a61b0` (`navmeshrender.cpp`): unit, and
  position after `PathManager::QInstance`.
- `NavMeshObstacleManager::GetInstance`/`Update` `006c0720`/`006c3640` and
  `CombatManager::Update` `00991500`: callee overlap, and the same calls in
  the combined AI thread function as on the Xbox.
- `Main::Swap` `0086ff70`: 11 shared callees and strings (`Main::RenderMenuBackground`,
  `FaderManager::RemoveFader`, `Main::KillMenuBGTexture`,
  `MTRenderingSystem::SetThreadStage`, `Main::UpdateOffscreenInterface`,
  `Interface::IsMenuIDVisible` eight times and others).
- `Main::PostSwapProcess` `008705d0`: calls
  `Main::OnIdle_UpdateProcessLists`.
- `Main::OnIdle_PostThreadsProcess` `00870610`: 4 shared callees and strings.
- `Script::ClearOptimizations` `005ae270`,
  `ScriptLocals::ClearOptimizations` `005a9d60`: the last two game calls of
  `Main::OnIdle` and of the AI thread functions, as on the Xbox.

Leads by position only: `00b6dd00` (`BSShader`, next to
`BSTextureManager::ReturnRenderedTexture` `00b6da10`); `00a81a20` and
`00a81a80` (`NiMain`, each behind a test, where `BeginUpdate` and
`EndUpdate` stand); `0057ab70` and `00b60040` (after
`RenderMenuBackground`; `00b60040` calls one function, as
`ProcessAllQueuedLights` calls `ProcessQueuedLights`); `008ca300` (the
only call between the threads-join branch and `UpdateNonRenderSafeAITasks`).

## The Havok step

`bhkWorld::Update` (`00c6ae70`) is slot `+0xc4` of the `bhkWorld` vtable
(`010c40b4`, slot at `010c4178`) and of the `bhkWorldM` vtable (`010c69f4`,
slot at `010c6ab8`). Three code sites call slot `+0xc4` on a world:

- `00554780` (`tesobjectcell.cpp`), on the exterior world in the global
  `011ca0d8`. `TESObjectCELL::InitStatics` (`00541b80`) stores there the
  world that `00554010` creates with `00c99ff0`, which writes the
  `bhkWorldM` vtable `010c69f4` (`00c9a00c`).
- `TESObjectCELL::UpdateManagedNodes` (`00551890`, call at `005518df`), on
  the cell's own world from `004543c0` (cell byte `+0x24` bit 0 set:
  `0041b9a0` on the cell's extra data at `+0x28`; clear: `00451010`, the
  global `011ca0d8`), only when that bit is set and `00450ff0` holds
  (`00450fd0` returns 6).
- `00554010` itself, once, right after creating the world (`0055450b`).

The per-frame path is `TES::UpdateCellAnimations` (`00453550`; `this` is
`[011dea10]` at the call from `0086fc60`). It adds the frame time to `011c3c08` and, between
`TES::LockHavokUpdateMT(1)` and `(0)` (`00453860`, a critical section only
when threads ≠ 1), calls either `TESObjectCELL::UpdateManagedNodes`
(`00551890`) on the cell at `this+0x34` when there is one, or
`GridCellArray::UpdateManagedNodes` (`004ba9a0`, on `this+0x8`). The
latter steps the exterior world first (`00554780`) and then runs
`00551890` on every loaded grid cell (`GridCellArray::Get`); those cells
have bit 0 clear, so the exterior world is stepped once per frame.

`TES::UpdateCellAnimations` has three callers:

- `Main::OnIdle_UpdateAnimationsAndEffects` (`0086fc60`, call at
  `0086fd08`) when threads = 1 and the menu flag `011dea2b` (set at the start of
  the frame from `IsInMenuMode`/`IsPipboyOpening`) is clear (otherwise it calls
  `TES::UpdateCellMainThread` `004537c0`);
- the combined AI linear task thread function `008c7bd0` (`008c7d03`);
- AI linear task thread 2's function `008c7f50` (`008c803d`).

So with threads > 1, which `main` makes the normal case on PC, the Havok
step runs on an AI linear task thread between `StartThreads` (stage 6) and
`WaitForThreads` (stage 8), after that thread's actor updates.

## The actor updates

`AILinearTaskThreadManager::CreateThreads` (`008c7290`, called from `main`
at `0086b12d` when threads > 1) creates one thread on `008c7bd0` ("AI Linear
Task Thread", string `0108561c`) when threads ≤ 2, else two, on `008c7da0`
("AI Linear Task Thread 1", `0108564c`) and `008c7f50` ("AI Linear Task
Thread 2", `01085634`). Their call lists line up with the Xbox
`AILinearTaskThreadManager::AIThreadCombined`, `AIThread1TaskFunc` and
`AIThread2TaskFunc`. `008c7bd0` calls, in order (Xbox PDB names from that
alignment; leads unless bold in the table above):

`Main::OnIdle_DoInterfaceIdle` **`0086fd70`**,
`TaskQueueInterface::ThreadBeginInput` (`004537b0`, `0087a6b0`),
`CombatManager::Update` **`00991500`**,
`ProcessLists::RunActorAnimationUpdates` `0096cca0`,
`ProcessLists::ParallelActorAnimationMovementUpdates` `0096cda0`,
`TES::RunAnimations` **`00455640`**,
`ProcessLists::RunActorMagicUpdates` `009784c0`,
`CombatManager::UpdateCombatants` `00991dc0`,
`ProcessLists::UpdateHighListPackages` `0096bcd0`,
`ProcessLists::UpdatePlayerFollowers` `0096d520`,
`ProcessLists::RunDetectionForAllActors` `0096c330`,
`ProcessLists::UpdateActorsMovement` `0096db30`,
`ProcessLists::RunActorUpdates` `0096c7c0`,
`ProcessLists::RunActorRagdollAnimationUpdates` `0096cb50`,
`BGSDestructibleObjectForm::UpdateDestructibleObjects` `004772f0`,
`TES::UpdateCellAnimations` **`00453550`** (the Havok step),
`NavMeshObstacleManager::GetInstance`/`Update` **`006c0720`**/**`006c3640`**,
`TaskQueueInterface::ThreadEndInput` (`004537b0`, `0087a6d0`),
`Script::ClearOptimizations` **`005ae270`**,
`ScriptLocals::ClearOptimizations` **`005a9d60`**. Between the groups it
signals and waits on thread stages (`00713d80` with `008c79e0`, `008c7a70`
or `008c7d80`). `008c7f50` runs the detection, movement, ragdoll,
destructible, cell-animation and script-clearing calls of that list;
`008c7da0` calls the combat, animation, magic, package, follower and
actor-update ones (callers of each from `get_xrefs_to`). `ProcessLists::RunActorUpdates` (`0096c7c0`) has no
caller besides `008c7bd0` and `008c7da0`.

The threads = 1 path (`008ca070`, the AI task queue with
`MobileObjectTaskletData::RunToCompletion`) is not followed further here.

## Coverage today

[LEDGER.md](LEDGER.md) ("Frame") counts the rows of `frame.tsv` by status
whenever it is regenerated. On 2026-10-09, after the rolling translation
batches up to #81, of the 143 depth-1 call sites 22 are `translated`, 37
`traced`, 5 `platform` and 79 `open`.

## PR 2 result: `world::frame`

`crates/world/src/frame.rs` (2026-10-09) is the control flow of
`0086e650`, translated (ADR-0003 marker): `STEPS` lists its 143 calls in
the exe's order (a test loads `frame.tsv` and compares address and name
row by row), each with a `Stage` (the table above), a `Gate` and a
`Wiring`. `steps_run(&FrameState)` says which run for given inputs. The
disassembly and the decompiler agree on every branch (the decompiler only
drops two blocks behind a local that is always 0, `0086ebd7`, `0086ec00`).

The gates (branch addresses in `0086e650`; each has a test):

| Gate | Steps | Branches |
| --- | --- | --- |
| Tab and Alt both held: return after step 1 | 2-143 | `0086e682`, `0086e69a` |
| Menu mode (`[011dea2b]` = `IsInMenuMode` or `IsPipboyOpening`; V.A.T.S.'s menu, sleep/wait, dialogue and the pause menu count) or the free camera's frozen world (`Main` +7, set only for `TFC` 1, 2 or 5: `005bc260` → `00961e30` → `00961f50`) stops the screen splatters, the cell tests, the sky update, `Calendar::Update` and (one thread) `TES::RunAnimations` | 46-47, 50-58 | `0086e918`, `0086e923`, `0086e946`, `0086e955`, `0086e9dc` |
| The radiation, process-level and follower lists, and the AI work's start and join, run when menu mode is clear or the fader is visible (`[011dea2d]`), the console is hidden (`[011dea2e]`) and the world is not frozen | 62-68, 98-102, 128-131 | `0086ea0c`-`0086ea84`, `0086ec1d`-`0086ec74`, `0086ee0c`-`0086ee45` |
| With threads > 1 and no AI work this frame, the main thread runs the interface idle, and `LastMinuteUpdate` in menu mode | 105-107 | `0086ecad`, `0086ecb5`, `0086ecc9` |
| One thread: the interface idle before the AI work, the obstacle manager (with `bUseObstacleAvoidance`, `011d73e4`) and `CombatManager::Update` on the main thread | 80, 113-116 | `0086eb31`, `0086ed00`, `0086ed11` |
| V.A.T.S. playback (manager mode 4, `[011f2250]`+8) skips `BSSceneGraph::SetCameraFOV` | 87-89 | `0086eb6d` |
| Loading: the start menu with flag 0x10000 skips the block; up without flag 1 it suspends the loading menu's thread; otherwise an open in-game loading menu is shown | 36-39 | `0086e8a6`, `0086e8b2`, `0086e8c5` |
| Sleeping or waiting: top menu 1012 asks `UpdateSleeping`; when true the menu background is redrawn | 120-121 | `0086ed75`, `0086ed81` |
| `PathManager::Update` stops only with the frozen world | 108-109 | `0086ecd9` |
| The memory free: interface mode 3 and no rendered menu other than the Pip-Boy | 13 | `0086e74e`, `0086e756` |
| The game-mode frame counter (`0086ef40`) | 31 | `0086e87a` |
| The heap sort (`00aa7290`): menu mode, the view key held, or 45 s gathered | 142-143 | `0086eeb9`, `0086eec2`, `0086eed4`, `0086eee7` |
| Requests and short-circuits: pathing profile `[011deefc]`, texture purge `[011f4461]`, display mode change `[011c6fbb]`, console open `[011dea2f]`, the parallel update's begin and end, the menu queries | 4, 9-10, 19, 23-24, 49, 51-52, 73, 118, 127, 135-136 | `0086e6c0`, `0086e714`, `0086e7a8`, `0086e7fd`, `0086e936`, `0086e96b`, `0086e97d`, `0086ead8`, `0086ed64`, `0086edfe`, `0086ee79` |

There is no separate "paused" test: the pause menu is a menu-mode menu.
The mode-dependent arguments are modelled too: `Main::OnIdle_UpdateImageSpace`
gets "the world runs", `BSTreeManager::Update` its negation and
`CombatManager::Update` "menu mode, the fader or the frozen world".

Steps wired to existing nv-rs code (checked by reading it):

- **Is the step**: 44 `Main::OnIdle_HandleMenuBackground` (`0086f450`) →
  `world::menu_background::Background::update`, run by the viewer's
  `menu_background::update`.
- **Partly there** (still open): 32 `Main::OnIdle_UpdateTimer` →
  `physics::havok::Clock` (the frame timer and Havok's delta only); 45
  `FaderManager::UpdateFaders` → `world::living::sleep::Fade` and the
  fade to black in the viewer's `game_menus` (two faders); 56
  `Calendar::Update` → `GameState::advance_clock` (its calendar is a
  labelled guess); 77 `BSTreeManager::Update` → `speedtree::wind` (its
  `006658b0` only); 78 `Main::OnIdle_UpdateCurrentGridCell` →
  `world::ref_scripts`' grid-move test (`00452580`).
- **Open**: the other 137.

Left for PR 3: the viewer's per-frame systems still run in their own
`.chain()`/`.before`/`.after` order, with menu mode approximated per
system (`menus::Menus::is_open`, the dialogue test, Bevy's virtual clock
paused in V.A.T.S.). PR 3 makes `Stage`/`FrameStep` system sets, puts each
system in the set of the step it belongs to, and runs each set under its
`Gate`, evaluated from one `FrameState` filled once per frame. Inputs with
no viewer counterpart yet (fader 1, the frozen world, the interface mode,
the thread count) need a source or a fixed value.

## PR 3 result: the viewer's order from the frame

`viewer/src/frame_order.rs` (2026-10-09) makes the frame the viewer's
`Update` order. Each `world::frame::Stage` is a system set
(`FrameSet::Stage`), configured in the order `world::frame::stages()` reads
off `STEPS`; each of the 143 steps is a set too (`FrameSet::Step(i)`,
inside its stage, in call order). Every stage set runs under
`world::frame::stage_reached` (only the Tab and Alt return stops a stage)
and every step set under `world::frame::step_runs` (its `Gate`), both
evaluated on one `FrameState` resource (`ThisFrame`) that `begin_frame`, the
first system of stage 1, fills each frame (the menu-state queries of steps
3-12). The gate logic stays in `world::frame`; the viewer only fills the
inputs.

Three viewer sets sit around the stages (`ViewerSet`):

- **Loading**, first: the new interior or exterior put on screen and the
  land streamed around the player. Loading is the frame's in the exe
  (`TES::ShowLoadingMenu`, step 39; grid cells in
  `Main::OnIdle_UpdateCurrentGridCell`, step 78), but the viewer's loaders
  aren't split along those calls yet and other systems rely on a scene
  asked for in one frame being in place at the start of the next.
- **Interface**, second: the game's menus, the viewer's own, V.A.T.S. and
  the lockpicking menu, which take Bevy's input (`reset_all`) before the
  player's systems read it. In the exe this is `Interface::Idle` (stage 5
  with one thread, else on the AI thread in stage 6); the player's update
  (stage 2) reads controls that `Main::OnIdle_PollControls` (step 33)
  polled in the previous frame, after that frame's interface idle, so the
  interface meets each input before the player there too. It moves into
  its stage once the player's update tests menu mode itself (PR 4, PR 7).
- **AfterFrame**, last: not in `Main::OnIdle` (the viewer's tools:
  screenshots, help, the cursor, exposure, the F12 report, the frame rate,
  the window's focus, the present mode, the console key's switch to the
  viewer's free camera, billboards turned to Bevy's camera), and output
  whose place isn't traced and which only needs the frame's work done (the
  GPU's grass and water around the camera, sounds, music, the radio).

Bevy's own schedules around `Update` (input in `PreUpdate`, transforms and
the camera's final placement in `PostUpdate`) are unchanged.

### Which systems landed where (121 `Update` systems)

| Set | Systems |
| --- | --- |
| Loading (5) | `spawn_scene`, `exterior::enter_exterior`, `stream_squares`, `stream_distant_land`, `lod_objects::stream_distant_objects` |
| Interface (20) | `game_menus`' seven (`start_menu` … `draw_menus`), `vigor::draw`, `compose_hud_over_scene`, `hacking::play_sounds`, `companion_wheel::play_voices`, `pipboy_keys`, `rendered_terminal::show_terminal`, `menus::run_menus`, `vats::run_vats`, `vats::scale_target_time`, `lockpick::pick_locks`, `show_lockpicking`, `caravan_table::show_table`, `casino_scene::show_casino` |
| 1. Frame start (1) | `frame_order::begin_frame` (steps 3-12's queries) |
| 2. Player (20) | the view (`scope_sway`, `player_camera::view_input`, `look_around`, `fly_camera`), the player's chain (`give_start_weapon`, `walk`, `player_furniture`, `player_idle::animate`, `player_attack`, `object_shots`, `report_facing_up`, `swap_textures`, `show_dropped_weapons`, `update_scope`, `update_view_model`, `apply_shot_camera`), `player_body::update_player_body`, `crosshair::pick`, `dialogue::talk`, `chatter::say_lines` |
| 3. Housekeeping (4) | step 32 `Main::OnIdle_UpdateTimer`: `clutter::time_havok_frame` (moved from `First`); step 44 `Main::OnIdle_HandleMenuBackground`: `menu_background::update`; with them `toggle_grade`, `effects::play_effects` |
| 4. World and time (13) | `scripts::start_use`, `run_scripts` (its `Runner::update` holds parts of steps 56 and 78), `walk::doors`, `doors::update_doors`, the movies, the weather, sky, daylight and emittance chain, `trees::stream_trees`; step 77 `BSTreeManager::Update`: `trees::sway_trees` |
| 5. Interface and scene | none yet |
| 6. AI start (39) | the people (`ai::move_offstage`, the three `bring_in_*`, `move_actors`, `fighting::resolve_shots`, `ground_log`, `look::*`, `sitting::idle_requests`, `actors::script_idles`, `animate_actors`, `dress::redress`, the four face systems), shots and hits (`bolts`' two, `explosives::fly_thrown`, `hiteffects::play_hits`, `impact_fx`'s two, `weapon_fx`' three), `clutter`'s Havok step (four), `move_pieces`, the interface idle's HUD and Pip-Boy (`update_hud`, `update_local_map`, `update_pipboy`, `pipboy_light`), `companions::come_along`, `map::find_markers`, `hud::follow_quest_targets`, `scripts::save_and_load` |
| 7. Render, 8. AI join | none yet |
| AfterFrame (19) | `toggle_walking`, `adjust_exposure`, `take_screenshot`, `update_help`, `grab_cursor`, `report::report_key`, `sounds::play_sounds`, `face_camera`, `report_fps`, `background::give_focus_back`, `present::use_mailbox`, grass (2), water (4), `music::play_music`, `radio::run_radio` |

Placed for order only, their place in the frame not traced (each says so
where it is added): objects' shots, the spine's facing, texture swaps,
dropped weapons and people's lines (in the player's chain); movies (after
the scripts); map markers, quest targets, saving and `come_along` (stage
6). The HUD and Pip-Boy go to stage 6 because the interface idle runs
there with threads > 1.

### Chains removed and kept

Removed, now given by the sets: the one `.chain()` over all of `main`'s
`Update` groups, and 21 `.before`/`.after`s: the menu background group's
`.after(draw_menus)`, the movies' `.after(run_scripts)`, `idle_requests`'
`.after(say_lines)`, `view_input`'s `.after(pick_locks)`,
`update_player_body`'s `.before(animate_actors)`, `move_pieces`'
`.after(spawn_scene)`, `face_camera`'s `.after(fly_camera).after(walk)`,
every `.after(player_attack)` (bolts, clutter, explosives, hit effects,
weapon effects) and `.after(walk)` (clutter), weapon effects'
`.after(object_shots)` and `.after(update_view_model)`, `update_pipboy`'s
`.after(run_scripts)` and `.after(update_view_model)`, and the HUD's,
music's and radio's `.after(run_scripts)`.

Kept, each with a comment where it is added, because they order systems
inside one set and the frame doesn't (yet): the loaders' chain; which menu
takes the input first (the game's menus before the viewer's, then
V.A.T.S., then lockpicking; the Pip-Boy's keys and the rendered terminal
between them; menu scenes drawn after their menus ran); the player's chain
(`PlayerCharacter::Update`'s own order is PR 4) with `view_input` before
`look_around` and the body after the seat; the grade switch, menu
background and effects; scripts, doors, movies and sky in the viewer's
order (the frame has the sky's update, step 53, before `Calendar::Update`,
step 56, but neither is wired to these systems yet); in stage 6, people
posed after they moved, faces and clothes after the pose, shots and thrown
things after people moved and before the hits' effects, the Havok step
after thrown things, the Pip-Boy's screen before the posing (the interface
idle is the AI thread's first call, `008c7bd0`) and its map before it
(the AI thread's order is PR 6); the trees on screen before they sway;
grass and water's own chains.

### The inputs

`viewer_frame_state` fills `FrameState`:

- menu mode: `menus::Menus::is_open` (the game's menus, the viewer's, the
  Pip-Boy, lockpicking), the dialogue menu (a conversation that isn't
  lines only) and the V.A.T.S. menu (modes 1 to 3; playback is not menu
  mode). The Pip-Boy's opening is inside `Menus::pipboy`, so
  `pipboy_opening` stays false.
- V.A.T.S. mode: `vats::Vats::manager_mode` (new; off, 1 to 3, 4 for
  playback). Top menu: the top game menu's class (`MenuDraw`).
- Tab and Alt: Bevy's keyboard (Tab, and either Alt, as
  `GetAsyncKeyState(0x12)`).
- The four inputs PR 2 left without a source, fixed at the normal PC case:
  **fader 1** not visible (the viewer's two faders, the sleep's and the
  menus' fade to black, are both fader 0; there is no fader 1);
  **the frozen world** not frozen (the viewer has no console window, and
  its free camera, the ` key, is its own, not `TFC`); **the interface
  mode** 1, game mode (menu mode comes from the menus above; only the
  memory free's gate, step 13, open, reads the number, and the exe's value
  in each menu isn't tracked); **the thread count** the processor count
  (`std::thread::available_parallelism`, standing for `GetSystemInfo`)
  raised to 2 when it is 1, as `main` does (`0086a950`-`0086a990`); every
  gate only asks 1 or more than 1, so the PC case is "more than 1".
- Also fixed: the console hidden (no console window); the requests and the
  loading inputs of steps no viewer system implements keep
  `FrameState::default`'s values.

With these inputs the only gate a viewer system sits under today is "the
frame reached the stage": the three wired step sets (32, 44, 77) are
ungated in the exe, and the systems that already handle menu mode their
own way (the AI's `frozen`, the scripts' time, Bevy's virtual clock in
V.A.T.S.) are in stage sets, not in the gated steps (the AI work's
`AiTasks`, the world's `WorldRuns`), because their exe counterparts aren't
split out yet; gating them now would change what they do in menus without
evidence. The gates are ready for PR 4 to PR 7.

### Behaviour differences

- The menu background, the screen effects and the grade switch now run
  before the scripts instead of after them: an effect a script applies
  plays from the next frame (as an effect from a hit already did).
- Systems that had no order now have one: the HUD, the Pip-Boy, the trees
  and the plugins' AI-stage systems run in stage 6 or 4; grass, water,
  sounds, music and the radio after the frame (sounds asked for by the
  AI stage play in the same frame).
- The console key's switch between walking and the free camera takes
  effect the next frame.
- Holding Tab and Alt stops every stage after the first, as the exe does
  (Bevy releases keys when the window loses focus, so an Alt+Tab away
  doesn't trigger it).
- The loaders now always run before the game's menus (they were
  unordered).

Tests (`frame_order::tests`): the sets run in `world::frame`'s stage order
with the viewer's sets around them, and the step sets in call order inside
their stage, with systems added in reverse; a system in a gated step set
(`Calendar::Update`, step 56) doesn't run in menu mode while the rest of
its stage does, and Tab+Alt stops every stage but the first; the fixed
inputs. `world::frame` gained `stages`, `stage_reached`, `step_runs` and
`step_of`, used by the viewer and tested there. Acceptance: doc, vcg02 and
vms16 pass; on `main` before the change vcg02 missed "XP +50" in one run
while doc and vms16 passed, so no route got worse.

## PR 4 result: the player stage

`crates/world/src/frame/player.rs` (`world::frame::player`, 2026-10-09)
models the player's step, `Main::OnIdle_UpdatePlayer` (`0086f940`, step 14),
and `PlayerCharacter::Update` (`0093e860`), which it calls through slot
+0x2f8 of the player's vtable (`0108aa3c`, written by `PlayerCharacter`'s
constructor at `009381c6`; the slot at `0108ad34` holds `0093e860`), the way
`world::frame` models `Main::OnIdle`: calls in the exe's order, each with a
gate read from the branches, with their addresses, and the viewer's sets in
that order (`frame_order::PlayerSet`). The disassembly of both functions
agrees with the engine's translations (`fallout_misc/main.rs`,
`fallout_ai/playercharacter.rs`) on every branch named here.

### `Main::OnIdle_UpdatePlayer`

All 30 calls (`UPDATE_PLAYER`): the 27 direct ones, which are `frame.tsv`'s
depth-2 rows under it (a test compares them), and three through the
player's vtable (+0x1d0, the player's 3D `00950b60`, twice; +0x2f8, the
update). In order, with their gates:

| Calls | Gate (branches) |
| --- | --- |
| `PlayerCharacter::HandlePositionPlayerRequest` `0093bea0` | none; a request carried out (a door, `MoveTo`, fast travel) returns (`0086f959`) |
| `Interface::IsPipboyOpening` `00709bc0` | menu mode, `[011dea2b]` as the frame's first read left it (`0086f968`) |
| `PlayerCharacter::ForceGrenadeHold` `009481d0` | menu mode and the Pip-Boy not opening (`0086f974`); returns. Its body holds a grenade throw's animation (anim action 5) in place |
| the 3D (+0x1d0), the frame time (`0084d030`), `PlayerCharacter::UpdateFlyCamera` `009466d0` (with `Main` +7, the frozen world) | the fly camera, `Main` +6 (`0086f98f`; toggled by `TFC`, `00961e30`); the update with the 3D (`0086f9a9`); returns |
| the 3D, the frame time, `VATS::GetPlayerUpdateMult` `009c8cc0`, **`PlayerCharacter::Update`** (+0x2f8) with their product | not the above; the 3D exists (`0086f9e9`) |
| `008d6f30` (the parent cell), `00436aa0` (the position) | not the above (also without the 3D) |
| `00425fd0` (interior), `00550200` (the position in the cell's square) | a cell (`0086fa72`); an exterior (`0086fa85`) |
| `TESObjectCELL::GetWorldSpace`, `TESDataHandler::GetCellFromWorldCoord` | the player left its cell's square (`0086fa9c`) |
| `00450fb0`, `00450ff0` (the grid cell's state 3, 6), `TES::UpdateCurrentGridCell`, `00451530`, `TES::ShowLoadingMenu` | the cell under it found (`0086facd`); `00450ff0` when not 3 (`0086fae0`); the tracker moved when neither (`0086faef`); the loading menu with the cell tests (`0086fb12`) |
| `0086fba0`, `0086fbc0`, `TESObjectCELL::AddReference`, `TESObjectCELL::GetAcousticSpace`, `0086fbb0`, `0086fbc0`, `0086fba0`, `BSShaderManager::GetAccumulator`, `BSShaderAccumulator::ClearAllBoundVolumes` | the cell found; the last with an accumulator (`0086fb81`) |

So the player's update stops in menu mode (only the grenade hold runs)
unless the Pip-Boy is opening, and the fly camera replaces it.

### `PlayerCharacter::Update`

58 sub-steps (`UPDATE`): not each of its 886 call instructions (most are
queries: the controls' states `00a24660`, settings, getters, vector
arithmetic, sound handles) but the calls that do the player's work, each at
its call sites. Its paths: the bookkeeping first; a forced activation
(`00944320`) ends it; the player's fade (`HighProcess::FadeUpdate`)
refreshes both views and ends it; an AI-controlled (`0093a740`:
`bAiControlledToPos`/`FromPos`/`Activate`/`Package`) or dead (slot +0x22c:
the life state, `004f8960`, 1, 2 or 6) player takes the controlled branch
and ends there; everyone else the free branch (`009408e4`). A step may also
depend on its own block's tests (a control pressed, a timer, the weapon's
state): those branches are listed with it (`own_tests`), not modelled.

| # | Sub-steps (call sites) | Gate (branches) |
| --- | --- | --- |
| 1 | `PlayerCharacter::SetFirstPerson` (`0093e8df`/`0093e8ee`) | V.A.T.S. ended, `[011f21d0]` (`0093e8ca`) |
| 2-5 | `HUDMainMenu::SetTargetType` (`0093edba`); with own tests `UpdateHardcoreMode` (`0093f374`), `ProcessLists::SortActorsCloseToPlayer` (`0093f5dc`), `ReturnToLastKnownGoodPosition` (`0093f62a`) | none |
| 6-7 | `TESObjectREFR::Activate` (`0093f64f`), `00519020` | a forced activation (`0093f639`); ends |
| 8-9 | `bhkRagdollPenetrationUtil::Update` (`0093f8c2`), **`PlayerCharacter::UpdateHeadingAndLooking`** (`0093f8d9`) | no forced activation |
| 10 | `HighProcess::FadeUpdate` (`0093f927`) | asked unless the view key's flags (`[011e07b8]`, `[011e07c1]`) are set and the process's slot +0x610 answers 0 (`0093f8f2`, `0093f8fd`, `0093f917`) |
| 11-16 | `ForceGrenadeHold`, the two views' animation updates (`008d3550`, `Actor::UpdateAnimationMovement`, twice), `PlayerCharacter::UpdateCamera` (`0093fa08`) | it answered true (`0093f931`); ends |
| 17-18 | `VATS::QuitVATSPlayback` (`0093fba7`), `ForceTemp3rdPerson` (`0093fbd3`) | knocked down (the process's slot +0x40c) or paralysed (slot +0x234) (`0093fb85`, `0093fb9c`); the second in first person (`0093fbcc`) |
| 19 | `Actor::GetOutofFurnitureQuick` (`0093fd3f`) | controlled (`0093fbe5`, `0093fbfe`) with the AI-control counter past its limit (`0093fc6a`, `0093fc82`, `0093fc97`); ends |
| 20-31 | `00886360` (own test `0093fe99`), the mover's move vector `009ea570`, `Actor::PickAnimations`, the two views' animations, `UpdateCamera`, `0054a070`, `ShadowSceneNode::UpdateObjectLighting`; `Actor::UpdateMagic` and the Activate control's `Activate` (own tests) when not dead (`009403d9`, `009403f4`) | controlled, no time-out; ends at `009408c2` |
| 32-35 | `PlayerCharacter::UpdateMenuModeButton` (`00940c78`), `HavokActivateDroppedReference`, `UpdateTemp3rdPerson`, `UpdateTemp1stPerson` | the free branch |
| 36 | **the attack** `00948310` (`009420fc`; own tests: dead, paralysed, the anim action, the grab type, the process's slot +0x3f8, the weapon flag 0x800) | free |
| 37 | `Actor::PickAnimations` (`009426ae`) | free |
| 38 | **the move**: the mover's move vector `009ea570` (`0094280b`), which its slot +0x14 then moves by | free |
| 39 | `PlayerCharacter::CheckBorderRegion` (`00942835`) | free |
| 40-42 | with own tests: the Toggle POV control's `SetFirstPerson` (`00942cc0`/`00942dc9`), the Activate control's `Activate` (`00943250`, `0094328e`, `00943348`), vanity mode's `ForceTemp3rdPerson` (`0094360e`) | free |
| 43-44 | the grab controls `0095f6c0` (`0094363c`), `Actor::UpdateMagic` (`0094371b`; the same life-state tests, `009436fb`, `00943712`) | free |
| 45 | the fields of view `0095de30` (`0094375e`) | free, not in dialogue (`00943752`) |
| 46-50 | the two views' animations (`00943787`..`00943806`: the view the player isn't in first, its own second), the process's pending animation flags `008ba600` (`0094380e`) | free |
| 51 | `PlayerCharacter::UpdateCamera` (`00943825`) | free, `[011f21d0]` not set again (`0094381c`) |
| 52-56 | `0054a070`, `UpdateObjectLighting`, `MuzzleFlash::Update` (with a muzzle flash, `0094387d`), `00555c20`, `CharacterProgression::BeginLevelUp` (own tests) | free |
| 57-58 | `PlayerCharacter::UpdatePlayerCombat` (`00943a7e`), `PlayerCharacter::UpdateAutoAimActor` (`009440a6`) | free |

Names are the engine map's; those it pairs by alignment only
(`ForceGrenadeHold`, `UpdateFlyCamera`, `UpdateHeadingAndLooking`) are
leads (`ForceGrenadeHold`'s body fits its name).

### Tests

`world::frame::player` (16): the direct calls equal `frame.tsv`; call
sites in order; every gate names branches inside its function; slot +0x2f8
is the update; one test per gate. The engine crate (a new dev-dependency on
`world`) drives the translations: `0086f940` under 16 input combinations
(each gate of the first table) logs exactly the calls `calls_run`
predicts, in order (the translation calls the small setters
`0086fba0`/`0086fbb0`/`0086fbc0` as Rust functions, so they aren't in its
log); `0093e860` under 13 (game mode, V.A.T.S. ended, a forced activation,
fading, the fade test skipped, knocked down in first and in third person,
AI-controlled, timed out, dead, dialogue, V.A.T.S. ending, a muzzle flash)
calls every reached sub-step without own tests, in order, and no listed
function outside that order; two own-test steps call once their tests
pass.

### The viewer

`viewer/src/frame_order.rs`: `PlayerSet::Call(i)` for each call of
`0086f940`, inside step 14's set, and `PlayerSet::Update(i)` for each
sub-step, inside the call of the update, each chained in call order and
run under its gate on `ThisPlayer`, which `begin_frame` fills with
`ThisFrame`: menu mode and the Pip-Boy's opening from the frame, the fly
camera from the viewer's free camera (`walk::Player::walking` off; it is
`TFC` without an argument: the world runs), dialogue from the dialogue
menu, the rest fixed (no position request, the 3D there, not
AI-controlled, not dead, not knocked down, no fade).

Where the stage's 20 systems went (checked by reading each):

| System | Place |
| --- | --- |
| `fly_camera` | in `UpdateFlyCamera`'s set (call 6): runs only flying, outside menu mode |
| `combat::player_attack` | in the attack's set (sub-step 36) |
| `walk::walk` | in the move's set (sub-step 38) |
| `scope::scope_sway` | inside the update's call, ahead of the look (its place isn't traced) |
| `sitting::player_furniture` | inside the update's call, right after the move (the exe's seated player is on the controlled branch, which isn't modelled) |
| `look_around` | at `UpdateHeadingAndLooking` (after sub-step 9, before 10), outside its gate: it also turns the camera to the view angles, which V.A.T.S.'s menu turns, and is the free camera's look |
| `player_idle::animate` | at the own view's animation update (after sub-step 49, before 50), outside its gate: it takes idle requests in menus too |
| `player_camera::view_input` | in step 14 ahead of its calls, ungated: its exe parts (the Toggle POV control, the temporary views, vanity mode, the knocked-down third person) come after the look, but the look reads its `mouse_taken` in the same frame, and it takes the Pip-Boy's and the dialogue's views, which are menu mode |
| `viewmodel::give_start_weapon` | ahead of step 14 (the viewer's `--weapon`) |
| `player_body::update_player_body` | after the furniture, ungated (it builds the third-person body, in menus too) |
| `object_shots`, `report_facing_up`, `swap_textures`, `show_dropped_weapons`, `update_scope`, `update_view_model`, `apply_shot_camera`, `crosshair::pick`, `dialogue::talk`, `chatter::say_lines` | after step 14, in the viewer's order: not the player's update (their place isn't traced), the HUD's and the renderer's work that goes on in menus, V.A.T.S.'s shot camera last, the crosshair and E on people |

In gated sets: 5 (`fly_camera`, `player_attack` and `walk` in their
sub-steps' sets, `scope_sway` and `player_furniture` in the update's
call). At their sub-step without its gate: 2 (`look_around`,
`player_idle::animate`). `world::frame::player` records the four mapped
ones as `Partial` wiring. Every call and sub-step stays open: none is a
world system yet.

### Behaviour differences

- In menu mode (the game's menus, the Pip-Boy, the dialogue menu,
  V.A.T.S.'s menu, a message box) walking, attacks, the scope's sway and the
  furniture stop, as the exe's player update does: the player no longer
  falls or slides while a menu is up, and `player_attack`'s HUD line, its
  counter-attack and Ammo Swap timers and its death countdown wait for the
  menu to close (they were updated every frame).
- The attack now runs before the move, the furniture and the first-person
  camera tracks (the exe's order, `009420fc` before `0094280b`): an attack
  aims from where the view was at the start of the frame. Sneak and Aim
  pressed in the same frame now leave the sights down (the viewer's sneak
  toggle is in `walk`; the exe's crouch block, `00940d5b`, comes before
  the aim).
- Flying, the free camera moves before the mouse look (`UpdateFlyCamera` is
  call 6, the look sits at the update's place): the move's direction
  follows the mouse a frame later.
- The menus (`ViewerSet::Interface`) stay ahead of the stages: the ungated
  player systems (`view_input`, the look) still read Bevy's input that the
  menus clear. That moves with PR 7.

## PR 5 result: the world and time stage

`crates/world/src/frame/world_time.rs` (`world::frame::world_time`,
2026-10-09) models the callees of stage 4 (steps 48-78) the way
`world::frame::player` models the player's step: for each function the
calls that do its work (not the queries: getters, settings, list nodes, the
frame time), in the order of their call sites, each with a gate read from
the function's own branches (`SubGate`, evaluated on a `WorldState`) and,
where the call also depends on its block's own tests (a list entry, a
per-actor value), those branches (`own_tests`, not modelled). Every call
site and branch named is from the disassembly of FalloutNV.exe 1.4.0.525
(the read-only Ghidra server); the Xbox PDB names are the engine map's
(left out where the map names a function only by the linker's folding).

### The functions

| Function | Sub-steps | Gates (branches) |
| --- | --- | --- |
| `TES::TestAllCells` `004556d0` (called with 0 while `bRunningCellTests`, `TES` +0x51) | 25: a mouse move (`SendInput`, so the machine doesn't idle), entering the next world space (`TES::SetWorldSpace`, a print), the exterior walk (`TESWorldSpace::LoadCell`, `00454e70`) or the interior walk (`00461980`), the visit (`Calendar::Update` by 10, placing the player: `00453dc0` indoors, `00454450` or `TES::UpdateCurrentGridCell` outdoors; `PlayerCharacter::RequestPositionPlayer`, `HandlePositionPlayerRequest`, `IOManager::LoadQueuedPriority`, four memory calls), the cell's line (print and `MessageHandler::Output`), the callback at `TES` +0x54, `TESSaveLoadGame::TestAllCells`, the end (`MessageHandler::IncDisableWarningCount`) | enter (`00455a2c`, `00455a39`, `00455ab1`); exterior/interior walk (`00455ba3`, `00455bbc`); a cell found and visited (`00455d5e`, `00455d66`, `00455dd7`); interior or not and an interior loaded (`00455ea3`, `00455eea`); logging (`004560c4`, `00456204`); the callback (`00456451`); mode 4/5 (`00456476`, `0045647f`); done (`0045649f`) |
| `Calendar::Update` `00867a40` | 7: the `TESGlobal` setter (`0046dce0`) on days passed (rewritten), the month's days (`004b10d0`), year, month, day, days passed, hour | the rewrite: the calendar's flag +0x1c or the hour more than 1 past the last (`00867a9b`, `00867ab3`); midnight (`00867b00`); the month ends (`00867b85`); the year ends (`00867bb1`) |
| `TES::RunAnimations` `00455640` | 3: the count `[011c56e8]` cleared (`00455680`), `TESObjectCELL::RunAnimations` or `GridCellArray::RunAnimations` | an interior loaded (`00455656`) |
| `ProcessLists::RunActorScripts` `00978550` | 1: `TESObjectREFR::RunScript` on each actor of process level 0 | own tests `0097858b`, `009785a3`, `009785ba` |
| `ProcessLists::UpdateRadiationList` `009777a0` | 9: per source and exposed actor `HighProcess::AddAvoidPathingArea`, `Actor::SetMoveMode`, `Actor::InitiateAvoidPackage`, the level kept (`00977c90`) and pushed (process slot +0x768); the player's; the levels reset when the sources end | sources (`009777cb`, `0097780b`); ended (`009777e0`-`009777fa`, `00977b99`) |
| `ProcessLists::ChangeProcessLevelTempList` `0096eb40` | 17: `PlayerCharacter::IsSleepingorResting`, then per entry of the `TempShouldMoveList`: `ProcessLists::AddReference`, `ProcessArray::RemoveActor`, deleted (slot +0x10), released (`00931e80`), dead and gone (slot +0x324, dropped items `00572270`), the move to its level (slot +0xd8 with 0, +0x224, process +0x364, `ProcessArray::AddActor`, +0x260), the rest flagged (slot +0xd8 with 1) | not resting (`0096eb5c`); per-entry own tests |
| `ProcessLists::UpdateFollowerTempList` `0096e9b0` | 2: the process's slot +0xc8 (an escort package, type 2) or `Actor::AddFollower` | per-entry own tests |
| `GarbageCollector::Update` `00868850` | 18: the collector's lock (`00867f50`/`00867f80`) around one batch of the first queue with entries: animations (`00418d20`), bipeds, 3D objects (`00868ce0`), references (slot +0x10), navmeshes (`00401970`), or a flag-clear array moved to its flag-set twin; the model loader's lock released (`004aaf10`) | the queues in order (`008688bf`, `00868976`, `00868a2d`, `00868a5c`, `00868ae6`, `00868b15`, `00868c0c`, `00868c3b`, `00868ca5`); the model loader's lock for the destroying ones (`008688d5`, `0086898c`, `00868b2b`) |
| `GarbageCollector::ClearTempEffects` `00868d10` | 4: the lock, `RemoveAll` on the flag-set effects, the model loader's lock released, the unlock | effects queued and the lock free (`00868d1f`, `00868d31`) |
| `BSTreeManager::Update(camera, stopped)` `006652e0` (not translated; disassembly only) | 7: SpeedTree's clock (`00b07060`), `CSpeedTreeRT::SetCamera`, the wind's speed from the sky (`TES` +0x68, its +0xcc; 0 indoors), its direction (`004bc450`), the wind update (`006658b0`), `iCanopyShadowScale:SpeedTree` (`00664720`), `fCanopyShadowGrassMult:SpeedTree` (`00665520`) | the world runs (`006652f4`, `006653a9`); a camera (`00665322`); the sky (`006653b6`, `006653c9`); outdoors (`006653dc`) |
| `Main::OnIdle_UpdateCurrentGridCell` `0086fbe0` | all 6 calls: the player's position, `TES::UpdateCurrentGridCell`, the cell tests' flag, then (cell tests outdoors) the world spaces refreshed | not menu mode (`0086fbf1`), no new game's loading menu (`[011d8907]`, set by the start menu's `ConfirmNewGame` at `007d3354`, cleared by the loading menu's destructor at `0078883c`; `0086fbfc`), not frozen (`0086fc07`); cell tests (`0086fc32`), outdoors (`0086fc41`) |
| `TES::UpdateCurrentGridCell` `00452580` | 22: always the last-loaded list cleared and the bounds refreshed; with no grid yet the area loaded at once (`TES::ShowLoadingMenu`, `004515a0`); inside the centre cell the loads coming into range queued (`ExteriorCellLoader::QueueCellLoad`, `TES::CleanUpUnusedTextures` first when asked); across a border the queued loads cancelled (`00528110`), the cells detached and attached around the new centre (`TES::SetWorldSpace` when none, `GridCellArray::SetCenter` through the grid array's slot +0x10, `TES::InitModelsToLoad`, `TES::GridArrayLoad`), textures cleaned up, `BGSTerrainManager::Update` at the player | the data handler (`00452624`); the first load (`00452637`, `00452643`); inside or crossed (`004527a4`, `004527be`, `004527ca`); the reload with no interior or a script running (`00452b5a`, `00452b62`); outdoors (`00452d75`) |

The rest of the stage is `0086e650`'s own queries and locks (the parallel
update's manager, the cell tests' flags, the frame time, the thread count,
the process lists' lock `011f11a0` around the lists, the memory manager's
call `00878080`, the texture purge request and `PurgeUnusedTextures`, the
scene graph's camera for the tree manager), and one call that isn't direct:
slot +0x104 of the "World" scene graph (`[011deb7c]`, vtable `01083b5c`)
with the frame time at `0086e9b5`. That slot holds `00c52590`,
`BSSceneGraph::SetViewDistanceBasedOnFrameRate`: PR 2 called it the sky's
update, which it is not (`Sky::Update` `0063ac70` is called from
`TES::UpdateCellAnimations` at `004536ae` and `TES::UpdateCellMainThread`
at `00453811`, stage 6); `world::frame`'s doc is corrected.

### Tests

`world::frame::world_time` (13): every direct call is in `frame.tsv`'s call
list under its function, in order (depth 3 for `00452580` under
`0086fbe0`); sites, gate branches and own tests inside their functions and
before their calls; the scene graph's call between the two frame-time
reads; a test per function's gates; the `follows` check. The engine crate
drives the translations and checks their calls against the model
(`follows`: every reached sub-step without own tests is called, in order,
and nothing only unreached sub-steps call; callees the translation calls as
Rust functions are named and left out): `0086fbe0` exactly under 6 input
combinations; `00867a40` under 5 (with the globals each setter writes);
`00455640` indoors and out; `00452580` inside the cell, queueing, the first
load, crossing outdoors and with an interior loaded; `004556d0` visiting an
interior with and without logging; `00978550`; `009777a0` with a source,
ended and without the iterator; `0096eb40` resting and not; `0096e9b0`;
`00868850` for each queue (and the busy lock); `00868d10` three ways.
`006652e0` has no translation, so no such test.

### The viewer

`viewer/src/frame_order.rs`: `WorldSet::Sub(function, i)` for each
sub-step, inside the set of the frame step that calls the function (its
first call: the process lists' threads > 1 steps, which the viewer's thread
count reaches) or, for `TES::UpdateCurrentGridCell`, inside
`0086fbe0`'s sub-step that calls it; each chained in call order and run
under its gate on `ThisWorld`, which `begin_frame` fills from `ThisFrame`
(the world runs, menu mode, the frozen world, the cell tests) and the
viewer (indoors: no `exterior::Exterior`; the sky outdoors), the rest
fixed at `WorldState::default` (no new game's loading menu, nothing for
the garbage collector, no radiation, not resting, the position inside the
grid's centre cell: none of these gates has a viewer system under it).

Where the stage's systems went (checked by reading each):

| System | Place |
| --- | --- |
| `trees::blow_wind` (new: the wind's step, split out of `sway_trees`) | in the wind update's set (`BSTreeManager::Update`, `006654dd`): under its gate, the world runs |
| `trees::sway_trees` | in step 77, right after the wind update, outside its gate: it applies the wind's last frame and the camera's axes (`CSpeedTreeRT::SetCamera`, called in menus too), the light and the trees' levels of detail (their places not traced) |
| `trees::stream_trees` | stage 4 before step 77 (for order: in the exe the trees come with the cells' attach, step 78; kept ahead so new trees get this frame's wind and light) |
| `scripts::start_use`, `scripts::run_scripts` | in `RunActorScripts`' `TESObjectREFR::RunScript` set (step 60; the step and sub-step are ungated): it runs every script it knows and, first, the game clock (`advance_clock`, `Calendar::Update`'s work, step 56, skipped in menus by its own test) |
| `walk::doors`, `doors::update_doors`, the movies, `weather::run_weather`, `follow_sky`, `daylight::follow_the_clock`, the emittance pair | for order, right after step 60 and before step 61; the weather and sky are `Sky::Update`'s work, which is in stage 6 (PR 6) |
| `exterior::stream_squares` (from `ViewerSet::Loading`) | in step 78 at `GridCellArray::SetCenter`'s sub-step, outside its gates: it also finishes the loads of a place just entered, which the exe does at once in the position request in any mode |
| `exterior::stream_distant_land`, `lod_objects::stream_distant_objects` (from `ViewerSet::Loading`) | in step 78 at `BGSTerrainManager::Update`'s sub-step, outside its gates (they follow the camera every frame; the exe's terrain update runs on a grid move) |
| `spawn_scene`, `exterior::enter_exterior` | stay in `ViewerSet::Loading` |

`world_time` records them as `Partial` wiring on `Calendar::Update`'s
setters, `RunScript`, `SetCamera`, the wind update, `QueueCellLoad`,
`SetCenter` and `BGSTerrainManager::Update` (13 sub-steps); every other
sub-step is open: `TES::TestAllCells` (the viewer has no cell test),
`TES::RunAnimations` (with threads > 1 it runs on the AI thread, stage 6,
where `move_pieces` stays), the radiation, process-level and follower
lists (the viewer's offstage movement, `ai::move_offstage`, and its
companions, `companions::come_along`, are its own and stay in stage 6),
the garbage collector and the temporary effects' clean-up (Bevy despawns
what the viewer removes), and the rest of the tree manager and the grid.

Tests (`frame_order::tests`): the world sets in the exe's order with the
systems added in reverse (the calendar's step, the scripts, what follows
them, the wind and the sway, the squares and the distant land, the grid's
last calls); menu mode stops the calendar's step, the wind and the grid's
sub-steps but not the scripts' step, the sway or the loaders; the fixed
inputs.

### Behaviour differences

- The trees hold still in menu mode (the game's menus, the Pip-Boy, the
  dialogue menu, V.A.T.S.'s menu), as the exe's tree manager stops its clock
  and wind there (`006652f4`, `006653a9`); they swayed before.
- The squares around the player and the distant land now stream after the
  player's update in the same frame (the exe's grid update, step 78, after
  the player's step 14), instead of at the start of the next frame; a
  square that finished loading is put on screen in the frame it is
  received either way.
- The doors, movies and weather run strictly after the scripts and before
  the trees (the weather and the trees had no order): the trees' light
  follows this frame's weather.

## PR 6 result: the AI task stage

`crates/world/src/frame/ai_stage.rs` (`world::frame::ai_stage`, 2026-10-10)
models the work `Main::OnIdle` starts in stage 6 and joins in stage 8 the
way `world::frame::world_time` models stage 4: the AI linear task threads'
calls in the exe's order, and inside them and the main thread's
`Main::OnIdle_UpdateAnimationsAndEffects` the calls doing the work, each
with a gate read from its function's own branches (`SubGate` on an
`AiState`) and, where its block tests more, those branches (`own_tests`,
not modelled). Every call site and branch is from the disassembly of
FalloutNV.exe 1.4.0.525 (the read-only Ghidra server).

### Which threads

`AILinearTaskThreadManager::CreateThreads` (`008c7290`) makes one thread on
the combined function `008c7bd0` when `iNumHWThreads` is at most 2
(`008c7307`: `cmp [eax],2` / `jle`), else the pair `008c7da0` and
`008c7f50`. `main` sets the count to the processor count and raises 1 to 2
(`0086a950`-`0086a990`), so a two-processor PC runs the combined function
and anything with more processors (nearly every PC today) the pair; with
a count of 1 (not reachable through `main`'s raise) no thread is made and
stage 6 starts the AI task queue (`008ca070`), which stays unmodelled.

The threads hand over by events: `008c79e0(thread, stage)` sets the event
of `stage` in the slot's table and records the stage (only when the slot
has a thread), `008c7a70(thread, stage)` waits for it, `008c7d80` records a
stage without an event (the combined function, which has no partner), and
`008c80d0` (→ `008c80b0`) waits for the render semaphore that
`SetMainRendering(0)` (`008c80e0` → `008c80c0`, `Main::Swap`'s last call)
signals: the calls after it run once the main thread has drawn the frame.

| Function | Calls in order (handshakes in brackets) |
| --- | --- |
| combined `008c7bd0` (count 2) | [set 0.0] `Main::OnIdle_DoInterfaceIdle` [set 0.1], `ThreadBeginInput`, `CombatManager::Update`, `RunActorAnimationUpdates`, `ParallelActorAnimationMovementUpdates`, `TES::RunAnimations`, `RunActorMagicUpdates`, `CombatManager::UpdateCombatants`, `UpdateHighListPackages`, `UpdatePlayerFollowers`, `RunDetectionForAllActors`, `UpdateActorsMovement`, `PrintLists` (empty), `RunActorUpdates`, `RunActorRagdollAnimationUpdates`, `UpdateDestructibleObjects`, `TES::UpdateCellAnimations` [mark 1.6, render wait], the obstacle manager (`bObstacleAvoidance`, `008c7d2c`), `ThreadEndInput`, the two `ClearOptimizations` [mark 0.11, 1.7] |
| thread 1 `008c7da0` | [set 0.0] interface idle [0.1], begin input, `CombatManager::Update`, the two animation passes [0.2], `TES::RunAnimations` [0.3], `UpdateCombatants` [0.4], magic [0.5], packages [0.6], followers [0.10, wait 1.1, 1.3], `RunActorUpdates` [0.7], end input [0.8, wait 1.7, render wait, 0.9], the obstacle manager (`008c7f23`), `ClearOptimizations` ×2 [0.11] |
| thread 2 `008c7f50` | [set 1.0, wait 0.1] begin input, detection [1.1, 1.2, wait 0.2], movement [1.3], `PrintLists`, ragdolls [1.4], destructibles [1.5, wait 0.4], `TES::UpdateCellAnimations` [1.6], end input [wait 0.8, 1.7], `ClearOptimizations` ×2 [wait 0.9, 0.11] |

(Names: `Main::OnIdle_DoInterfaceIdle`, `CombatManager::Update`,
`TES::RunAnimations`, `TES::UpdateCellAnimations`, the obstacle manager's
and the scripts' are PR 1's pairs or the engine map's; the rest are PR 1's
position-only leads, flagged `lead` in the model.) Note the pair's thread 1
does the combatants before the magic, the combined function the other way
round; the movement waits for the animation passes, the actor updates for
the movement, the Havok step for `UpdateCombatants`, but the actor updates
and the ragdolls, destructibles and Havok step aren't ordered against each
other.

`schedule(threads)` gives the viewer's one thread an order the exe can run:
the combined function's own with a count of 2; with more, thread 1 until it
waits for what thread 2 hasn't signalled, then thread 2, and so on, which
keeps both threads' orders and every wait (thread 2's movement, ragdolls,
destructibles and Havok step come before thread 1's actor updates). Both
orders split at the render wait. The thread split itself is platform.

### The functions

| Function | Sub-steps | Gates (branches) |
| --- | --- | --- |
| `Main::OnIdle_UpdateAnimationsAndEffects` `0086fc60` (main thread, step after `StartThreads`) | 6: the temporary effects (`ProcessLists::UpdateTempEffectsParallel` with threads > 1, else `UpdateTempEffects`), `TES::UpdateCellAnimations` (one thread outside menu mode) or `TES::UpdateCellMainThread`, `BSParticleSystemManager::UpdateParallel` (two arms) | all: menu mode clear, or the dialogue flag `[011dea2c]` (`Interface::InDialog`, set at `0086e6ec`/`0086e7d3`) or fader 1, and the world not frozen (`0086fc72`, `0086fc7d`, `0086fc88`, `0086fc97`); threads (`0086fcaa`, `0086fd3d`); one thread and no menu (`0086fceb`, `0086fcf6`) |
| `TES::UpdateCellAnimations` `00453550` (the AI thread; with one thread `0086fc60`) | 13: the wind for the trees' modifier and Havok's wind listener (zero indoors), `TES::LockHavokUpdateMT(1)`, `TESObjectCELL::UpdateManagedNodes` (an interior) or `GridCellArray::UpdateManagedNodes`, the unlock, the task queue (`0087aa90`), the collision listener (`00623640`), `Sky::Update` and `TES::UpdateCellMainThread` (one thread only), `ProcessLists::PostProcessProjectiles` | indoors (`00453568`, `004535f7`); threads > 1 (`0045363b`); one thread (`00453666`, `004536c0`) |
| `TES::UpdateCellMainThread` `004537c0` | 2: `Sky::Update` (in dialogue with `fAnimationMult` times the frame time), the temporary node manager's update (`00a59c60`) | none |
| `GridCellArray::UpdateManagedNodes` `004ba9a0` (disassembly only) | 2: the exterior world's step `00554780`, then each loaded grid cell's `00551890` | own tests (`004baa7d`) |
| `00554780` | `bhkWorld::Update` (slot +0xc4) on the world in `011ca0d8` | own test: a world (`00554790`) |
| `TESObjectCELL::UpdateManagedNodes` `00551890` | its first call only: `bhkWorld::Update` on the cell's own world; the rest of its 2,793 bytes not split | own tests: an interior, attached, with a world (`005518af`, `005518c1`, `005518d5`) |
| `ProcessLists::RunActorUpdates` `0096c7c0` (disassembly only) | slot +0x2f8 with 0.0 on each high-process actor with `bProcessMe` (+0xbc): `Actor::Update` (the `Actor` and `Creature` vtables, `0108454c`, `010873a4`) or the `Character` override `008d3550` (`01086d64`), which calls `Actor::Update` first | own tests (`0096c80c`, `0096c823`, `0096c837`) |
| `Actor::Update` `00888b50` | 19: the floats reset, the process woken (long steps), slot +0x44c, the animation speed, the controller's head target, the power attack flag, the process's +0x704, the life state, the weapon's attack sound, the move mode, the 3D placed (first-person player: `00440460`, `00952290`; others `0088b150`, the process's +0x428), the lighting, the water depth, `008b3230`, `008c1470` | ready: the 3D, a cell, the cell's state (`00888c5e`, `00888c64`, `00888c73`) and the time under `[010848d0]` (`00888c88`, else only the process's +0x5f8); a controller (`00888de3`) and not the player (`00888df5`); the first-person player (`00888d9e`, `00888db0`, `00889810`); swimming: not the first-person player, a controller, not menu mode (`008898ad`, `008898b7`, `008898c7`) |

Corrections to earlier sections: with AI threads, `Sky::Update` is not
called by `TES::UpdateCellAnimations` (its call, `004536ae`, is for one
thread only) but by `TES::UpdateCellMainThread` from the main thread's
`0086fc60`; and the lower process levels' moves (the viewer's
`ai::move_offstage`) are stage 7's: `Main::PostSwapProcess` →
`Main::OnIdle_UpdateProcessLists` (`0086f890`) →
`ProcessLists::UpdateProcessLists` (`0096d810`) → `0096b810`, `0096b470`,
`0096b050`. The actors' movement pass (`0096db30`) also moves the list's
projectiles (`009bec10`, the `Projectile` vtables' slot +0x2f8,
`Projectile::Update` `009becc0`) and explosions (`009ae580`).

### Tests

`world::frame::ai_stage` (14): `0086fc60`'s, `00453550`'s and `004537c0`'s
direct calls are in `frame.tsv` under them in order; every site and branch
inside its function and before its call; each caller calls there; the
threads made per count; the combined order for 2; the pair's order keeps
every wait and each thread's order and puts the render once; the frame
starts and joins the threads under one test with `Main::Swap` between; a
test per function's gates; the first caller each count reaches; the
inputs from the frame; `follows`. The engine crate drives the
translations: `0086fc60` under 15 input combinations (threads 1, 2, 4 ×
game mode, menu mode, dialogue, fader, frozen; exactly the reached calls);
`00453550` under 12 (threads 1, 2, 4 × indoors or not × dialogue);
`004537c0` with and without dialogue; `Actor::Update` for a person, a long
step, no 3D, the first-person player and a swimmer with a controller. The
thread functions, `0096c7c0` and `004ba9a0` have no translation.

### The viewer

`viewer/src/frame_order.rs`: `AiSet::Call(callee)` for each work call of
the schedule for the machine's thread count (`AiSet::Again` for the second
thread's own input and optimizations), chained in that order inside
`StartThreads`' step (before the render wait) or `WaitForThreads`' (after
it), so under the frame's gate for the AI work and each under its own on
`ThisAi`; `AiSet::Sub(function, i)` for each sub-step, inside the call (or
frame step, or sub-step) through which the count reaches the function
first; `ThisAi` filled by `begin_frame` from `ThisFrame` (threads, menu
mode, fader, frozen, obstacle avoidance), the dialogue menu and indoors.

| System | Place |
| --- | --- |
| `hud::update_hud`, `local_map::update_local_map`, `pipboy::update_pipboy`, `pipboy_light` | at the interface idle (the threads' first call), outside its gate: without AI work the main thread makes the call (step 105) |
| `look::set_up`, `sitting::idle_requests`, `look::follow_player`, `actors::script_idles`, `actors::animate_actors`, then `dress::redress`, the faces and `dialogue::focus_camera` | at `RunActorAnimationUpdates`, outside its gate (they handle menu mode and the dialogue menu's speaker themselves) |
| `move_pieces` | in `TES::RunAnimations`' call, under its gate |
| `ai::move_actors` (and `ground_log`) | at `UpdateActorsMovement`, just before it, outside its gate (its own menu and dialogue handling) |
| `fighting::resolve_shots`, `bolts::fly_bolts`, `explosives::fly_thrown` | in `UpdateActorsMovement`'s call, under its gate (the pass's projectiles) |
| `hiteffects::play_hits`, `impact_fx`'s two, `weapon_fx`' three, `bolts::hide_culled_bodies` | after that call, outside its gate, for order only |
| `clutter::simulate`, `play_contact_sounds` | in `TES::UpdateCellAnimations`' call between its lock and unlock sub-steps, under its gate (the Havok step); `grab_held` before it and `draw` after it, outside |
| `weather::run_weather` | in `TES::UpdateCellMainThread`'s `Sky::Update` sub-step (under `0086fc60`'s gates) |
| `follow_sky`, `daylight::follow_the_clock`, the emittance pair | right after it, outside its gate (the camera moves in menus; they change with the clock and the weather) |
| `ai::move_offstage` | stage 7, in `Main::PostSwapProcess`' step |
| `map::find_markers`, `hud::follow_quest_targets`, `bring_in_*`, `scripts::save_and_load`, `companions::come_along` | stage 6 before the threads' start, for order only (not traced) |
| doors (`walk::doors`, `doors::update_doors`), movies | stay in stage 4 after the scripts (movies run inside `PlayBink`'s script command, `005d15d0`; the doors' per-frame place isn't traced, their leaves move in `move_pieces`) |

`ai_stage` records Partial wiring on five thread calls (the interface
idle, the animation updates, `TES::RunAnimations`, the movement, the
ragdolls) and five sub-steps (the wind listener's wind, `Sky::Update`,
both Havok steps, `Actor::Update`'s 3D placement). Open: the task queue's
input, `CombatManager::Update` and `UpdateCombatants`, the second animation
pass, magic, packages and followers (`move_actors` evaluates packages
itself but isn't split), detection, `RunActorUpdates`/`Actor::Update`
(no system is the per-actor update), destructibles, the obstacle manager,
the scripts' optimizations, the temporary effects, particles,
`PostProcessProjectiles`, the temporary node manager, and the one-thread
task queue `008ca070`.

Tests (`frame_order::tests`): the AI sets in the exe's order for 2 and 8
threads with systems added in reverse (the interface idle, the poses, the
cells' animations, the movement and its projectiles, the hits, the actor
updates and the Havok step in each count's order, the sky, the offstage
moves); menu mode stops the gated sets (the cells' animations, the
projectiles, the actor updates, the Havok step and the sky) but not the
systems ordered outside them, and the dialogue menu keeps the sky; the
inputs.

Acceptance: doc, vcg02 and vms16 pass with the change (release viewer,
this machine's processor count, so the pair's order).

### Behaviour differences

- In menu mode (the game's menus, the Pip-Boy, V.A.T.S.'s menu, a message
  box; and the dialogue menu) loose objects (`clutter`), bolts, thrown
  things and people's shots hold still, as do models' own animations (fans,
  door leaves): the exe runs no AI threads then. Outside the dialogue menu
  the weather's step waits too (`0086fc60`).
- People are posed before they move (the exe's animation pass comes before
  the movement): a change of gait shows a frame later.
- The weather now steps after the trees sway (stage 4 reads the sky's wind
  from the last frame, as `BSTreeManager::Update` does).
- People out of sight move after the frame's render work (stage 7) instead
  of before the people on screen: someone walking into the loaded squares
  comes on screen a frame later.
- With more than two processors the Havok step runs before the actor
  updates' place (thread 2 before thread 1's `RunActorUpdates`); no viewer
  system sits at the actor updates, so nothing visible follows from it.
## PR 7 result: interface and render model

`world::frame::interface` records 19 functions: the interface idle's
pre/idle/post wrappers and manager bodies, last-minute tile updates,
sleeping, the menu background, Swap, post-swap audio and process lists,
non-render-safe AI, post-thread work, console opening and control polling.
Every modelled call names its PC call site, gate branches and additional
unmodelled local conditions. The manager's large Idle body is grouped by
work, not claimed to reproduce every query. Names are Xbox PDB names;
`00711ea0` remains a positional lead. Source: FalloutNV.exe 1.4.0.525,
private Ghidra server on port 8090 (read-only queries).

The preserved interrupted batch has tests comparing these models with the
translated engine functions' call logs, plus call-order checks against the
committed frame tree. This does not execute the translated engine in play.
The post-swap order was rechecked directly in Ghidra: `008705d0` calls
`0086f640` before `0086f890`; audio calls `00ad7740`, `00832ad0`,
`0082fb70`, then `0082d7c0`. Process-list work tests menu mode/fader,
console and frozen-world state before `0096d810`.

Resumption found a missing voice-pause call in the PreIdle model: the
game-mode path calls `0070bbe0` at `0070bb03`, guarded by `0070baae` and
`0070bab9`, after pausing sounds. The model and branch regression now
include it. The conformance fixtures also distinguish the deferred queue
from the tile-update array, and check direct Rust helpers through their
side effects rather than requiring entries in Engine's dispatch log.

The viewer's `InterfaceSet(function, sub-step)` nests unique main-thread
calls and their children under the existing frame sets. Sounds, radio and
music run at the audio calls, then `ai::move_offstage` at `0096d810` under
the traced process-list gate. The regions' sound update remains open.
These are CPU Update positions; Bevy still submits and presents later in
its own render schedule. They are not proof of original GPU ordering.

The interrupted WIP claimed several changes it had not implemented. Those
claims were removed: there is no `configure_input` or background `redraw`
system, and Bevy rendering does not count as wiring the Swap callees.
Menus still consume current Bevy input in `ViewerSet::Interface` before
the frame. Moving them to the executable's idle position needs the
previous-frame player input boundary first. HUD/Pip-Boy remain at the
existing approximate AI idle position; the one-thread and menu-mode idle
fallbacks are modelled and tested in core, not precisely routed in the
viewer. Fades, sleeping and menu scenes have existing partial behavior,
but their exact sub-step placement remains open. Last-minute tile work,
console and queued menu creation, background redraw and renderer gates
also remain open. No default model value is used to gate a newly wired
system unless its source is known from the viewer's frame state.

`research/engine-map/frame_wiring.tsv` enumerates every depth-1 and depth-2
call, including unsupported queries, as system, partial or open. The world
test regenerates it with `NVRS_WRITE_FRAME_WIRING=1` and otherwise rejects
stale output. The ledger reports these counts separately from translation
coverage. This meets the inventory gate, not playable completeness.

Validation: full core workspace tests passed (15,108 including doc tests;
13,255 in engine); the separate viewer release suite passed all 182 tests.
Both workspaces passed formatting and clippy with warnings denied (viewer
clippy used release), and both release builds passed, using two jobs.
The ledger generator's four tests and stale-output check passed. All three
release acceptance routes passed: Doc Mitchell (59 s), Back in the Saddle
(400 s) and Ghost Town Gunfight (337 s). Private logs and screenshots:
%USERPROFILE%\nv-re\acceptance\pr7-resume-20261010. Startup diagnostics
were read and Doc's screenshot inspected. No original-game comparison
has been made.
Outstanding interface follow-up: establish buffered player input before
relocating menu idle. Next shared action after validation and merge: publish
the play build, then start Phase 2 as directed in HANDOFF.md.

## PR 8 result: the controls' buffer and the menus at the idle

`crates/world/src/frame/input.rs` (`world::frame::input`, 2026-10-12)
records where the game polls the controls and which poll each reader sees;
the viewer's `poll.rs` makes Bevy's input that buffer, and its menus move
from ahead of the frame to the interface idle. Source: FalloutNV.exe
1.4.0.525, private Ghidra server on port 8090 (read-only queries), and the
engine crate's translations of `0093e860` and `009466d0`.

### What was traced

- **The poll.** `Main::OnIdle` calls `Main::OnIdle_PollControls`
  (`0086f390`) once, at `0086e88c` (step 33, ungated); it calls
  `Controls::Poll` (`00a23010`) at `0086f39e`. No other step calls it.
  `Controls::Poll` copies each device's current state over its previous one
  and then reads the device anew (DirectInput `GetDeviceState`, slot +0x24,
  after `Acquire`, +0x64; a lost device reads as nothing): the XInput pad
  (globals `011f35a8` to `011f35b8`), each joystick (`+0x34` to `+0x84`,
  0x50 bytes, 0xa0 apart), the keyboard (`+0x18f8` to `+0x19f8`, 256 bytes),
  the mouse (`+0x1b24` to `+0x1b38`, 0x14 bytes: the axes' counts and
  buttons). Its other callers are outside the frame: the window's
  activation (`00871cc8`), `CheckContinuePlaying` (`008674b2`, `008674f1`)
  and the modal message loop `00704010` (`007040fc`, used by
  `LoadHeader`), which polls and then calls the three idle wrappers, the
  same order as the frame.
- **The queries read only the buffer**: a control (`00a24660`, through
  the bindings at `+0x1b94`; state 0 held, 1 gone down, 2 come up, 3
  changed), a key (`00a24180`, `+0x18f8` against `+0x19f8`), a mouse button
  (`00a23a50`), a mouse axis (`00a239e0`).
- **The player reads the previous poll.** Step 14,
  `Main::OnIdle_UpdatePlayer` (`0086f940`), comes before step 33. Its
  `PlayerCharacter::Update` (`0093e860`, slot +0x2f8) reads the movement,
  attack, aim, Activate, Grab, Quick Save (`0094285d`) and Quick Load
  (`009403aa`, `00942878`) controls; with the fly camera `009466d0`
  (`0086f9c7`) reads them instead (Quick Save and Load at `00947aa5`,
  `00947ad9`). The translation calls `009466d0` "the player's per-frame
  input handling"; the PR 4 model's lead name `UpdateFlyCamera` is only its
  gate (`Main` +6).
- **The interface reads this frame's poll.** `InterfaceManager::Idle`
  (`0070c4a0`, 15 query sites) runs through `Main::OnIdle_DoInterfaceIdle`
  from step 80 (one thread, `0086eb36`), the AI threads' first call
  (`008c7be9`, `008c7db9`; they start at step 101, `StartThreads`) or step 105
  (`0086ecba`, no AI work: menu mode). All are after step 33.
- **What the player then sees** is the buffer as the interface left it:
  `Controls::ClearKeystrokes` as the menus take over and on the console's
  key, `Controls::ClearUserActions` in `Idle` (`0070e9ac`) as the Pip-Boy
  closes, and in the poll itself (`0086f43d`) while V.A.T.S. plays back or
  the Pip-Boy comes up.
- **Not the buffer**: the frame's Tab and Alt test (`GetAsyncKeyState`,
  `0086e682`, `0086e69a`), and the keyboard's event queue
  (`GetDeviceData`, slot +0x28, in `00a23820`), which `Idle` drains at
  `0070dfd8`, `0070e017`, `0070e0c5`, `0070e104` for the console and the
  frontmost menu.

So each poll reaches the interface in its frame and the player in the
next, after the interface. Tests (`world::frame::input`, 5): the poll is
step 33 and `Controls::Poll` is its first sub-step and `frame.tsv`'s row;
the player's step reads the previous poll for 1, 2 and 8 threads in and out
of menu mode; the idle's step is 80, 101 or 105 by thread count and mode,
always after the poll, with the four callers checked; Tab and Alt reach
neither; the buffer's parts and the device reads' sites.

### What changed in the viewer

- `viewer/src/poll.rs`: Bevy's keys, mouse buttons, mouse motion and wheel
  are the buffer. `hold_last_poll` (new `ViewerSet::Input`, first in
  `Update`) sets Bevy's fresh `PreUpdate` read aside and puts back the
  buffer as the last frame left it; `poll_controls` (at `Controls::Poll`'s
  sub-step of step 33) puts this frame's read in; `keep_buffer` (`Last`)
  keeps what the frame left, with the menus' `reset_all`, for the next
  frame's player. A system so reads the buffer as its place in the frame
  finds it. `PlayerInput` is the buffer as the player's step read it,
  copied at the poll, for player controls the viewer runs later for order:
  the doors' E (`walk::doors`; the Activate control), grabbing
  (`clutter::grab_held`; the Grab control), F5/F9 (`scripts::save_and_load`;
  Quick Save and Quick Load) and T (`run_scripts`' wait key; the Rest
  control, whose exe reader isn't traced: read from the player's buffer so
  a menu that took the key keeps it, as before).
- `begin_frame`'s Tab and Alt test reads this frame's device read, as
  `GetAsyncKeyState` does.
- The menus (`ViewerSet::Interface`, now `frame_order::IdleSet`: the game's
  menus, the viewer's, the Pip-Boy's keys, the rendered terminal,
  V.A.T.S., lockpicking and the menu scenes) run in stage 6 at the AI
  threads' interface idle call, outside its gate, before the HUD and the
  Pip-Boy's screen, which were already there and now follow them. Their
  internal order is unchanged.
- Wiring: step 33 and `Controls::Poll` are `Partial`; the idle's wiring
  text says where the menus are. `frame_wiring.tsv` and LEDGER.md were
  regenerated.
- Tests (viewer): `poll::tests` (3: a press reaches the interface in its
  frame and the player in the next, mouse motion too; what the interface
  takes the player never sees; a frame without a poll keeps the buffer);
  `frame_order::tests` (the input set first; the player's step, the poll,
  the menus and the HUD in that order for 2 and 8 threads in and out of
  menu mode, agreeing with `world::frame::input`; a press through the real
  systems in their sets).

### Behaviour differences

- The player's controls act a frame later (keys, mouse buttons, the mouse
  look and the wheel), as in the exe: the player's update reads the last
  poll.
- A menu opened or closed by a key changes menu mode from the next frame's
  start, not the same frame's: the player's step in the frame a menu opens
  ran before the menu did, on the previous poll.
- The menus see a script's request (a message box, a menu a script opens,
  stage 4) in the same frame instead of the next; menu saves and loads
  from the start menu reach `save_and_load` (stage 6, before the idle) a
  frame later.
- Between the poll and the idle, the scripts (stage 4) and the AI stage's
  systems read this frame's input before the menus take it. None of them
  reads a key except the player controls moved to `PlayerInput` above (and
  the viewer's own grade switch in stage 3, which reads the buffer wherever
  it falls in that stage).
- With Tab and Alt held the frame doesn't poll: the buffer keeps the last
  poll and Bevy's device state goes on, so a key going down or up during
  those frames is seen as held or not, not as a press or release.

### Validation

Core: `cargo test --workspace` (engine 14,427 and every other suite
passed), clippy with warnings denied, `cargo fmt --check`, release build.
Viewer: release tests (187 passed), release clippy with warnings denied,
fmt, release build. All checks went through the machine's compiler lock.
`frame_wiring.tsv` and LEDGER.md regenerated. All three release acceptance
routes passed: Doc Mitchell (57 s), Back in the Saddle (398 s), Ghost Town
Gunfight (337 s); private logs and screenshots in
%USERPROFILE%\nv-re\acceptance\pr8-input-buffer-20261012. Startup
diagnostics read (no panics; vcg02's 3,441 Bevy warnings about despawned
entities are the same count as PR 7's and earlier runs); Doc's screenshot
shows him in the hall with the HUD and "E) Talk". No original-game
comparison has been made.

### Still open

- The idle's menu-mode position: without AI work the exe calls it at step
  105, after `Main::OnIdle_UpdateAnimationsAndEffects` (whose sky update runs
  in the dialogue menu). That is after the poll too, but the viewer keeps the
  menus at the threads' call in every mode (with a thread count of 1, which
  `main` never leaves, they go to step 80's place).
- The menus' places inside `PreIdleStuff`/`Idle`/`PostIdleStuff` and the
  exe's clears (`ClearKeystrokes`, `ClearUserActions`, the poll's own clear
  for V.A.T.S. playback and the Pip-Boy coming up): the viewer's menus
  still take the input with their own `reset_all`.
- The menus' typed characters are Bevy's keyboard events, read where the
  menus run, as the exe's idle drains the key queue; the pad isn't polled
  (the viewer has none). The DirectInput state semantics (an edge is a
  change between two polls) are Bevy's event semantics here.
- VR (docs/VR.md): the buffer holds controls only. Head and hand poses are
  per-view, not polled controls, and must not go through it; the action
  layer VR.md proposes would read this buffer in place of Bevy's resources.

## PR sequence

Each PR names one next action, regenerates the ledger and passes the
acceptance routes, as in B1.

1. **Frame map.** Extend `research/engine-map` with a `frame` step that
   writes the `Main::OnIdle` tree (depth 3, in call order, PC and Xbox
   addresses, names, ledger status) as a committed table, and show its
   coverage in LEDGER.md. Pair the remaining `?` rows by aligning the PC
   and Xbox call lists (the two are close to identical here). Trace where
   the Havok step and the actor updates are called from (vtable calls and
   the AI task lists). No Rust behaviour changes. *Done*: the ledger
   computes the status instead of the table storing it (it would go
   stale); 38 depth-1 functions (42 call sites) newly paired, the rest
   listed above.
2. **`world::frame`.** Translate `0086e650`'s control flow: which stages run
   in menu mode, while loading, with the console or Pip-Boy open, in
   V.A.T.S. and while sleeping, as an ordered `FrameStep` list. Each step
   either calls our existing system or is marked `open` with its address.
   Tests: the step order equals the traced order; the mode gates match the
   branches of `0086e650`. *Done*: "PR 2 result" above.
3. **Bevy order from the frame.** The viewer's per-frame systems are ordered
   by `FrameStep` system sets instead of their own `.before`/`.after`
   chains; remove the chains it replaces. This is where "runs in the wrong
   order" bugs get fixed; acceptance routes must pass unchanged or with
   explained differences. *Done*: "PR 3 result" above (no "wrong order"
   bug was fixed by it: the menus stay ahead of the player, see there).
4. **Player stage.** `Main::OnIdle_UpdatePlayer` (`0086f940`) and the call
   order inside `PlayerCharacter::Update` (`0093e860`, 22.6 KB, the largest
   game function on this path): order only, calling our systems. *Done*:
   "PR 4 result" above.
5. **World and time stage.** `TES::TestAllCells`, `Calendar::Update`,
   `Main::OnIdle_UpdateCurrentGridCell` (cell attach and detach), the
   process-level temp lists, `GarbageCollector::Update`. *Done*: "PR 5
   result" above.
6. **AI task stage.** The tasks `AITaskManager` starts in stage 6 and joins
   in stage 8: actor process updates, animation, the Havok step. Our single
   thread runs them in the game's order; the thread split itself is
   `platform`. *Done*: "PR 6 result" above (the count of 2 runs the combined
   function, more the pair; the one-thread task queue stays open).
7. **Interface and render stage.** `Interface::Idle` and its pre/post
   steps, `LastMinuteUpdate`, the menu background, `Main::Swap`'s
   pre/post-swap work (the renderer itself stays Bevy). Model and scoped
   wiring: "PR 7 result" above; exact menu input and renderer work remain open.
8. **The controls' buffer.** The poll at step 33 as the input boundary: the
   player reads the previous poll, the interface this frame's; the menus at
   the interface idle. *Done*: "PR 8 result" above.

After PR 7 every call in `Main::OnIdle` to depth 2 is either wired to a
system or listed as `open` in the ledger, which is Phase 1's gate. Phase 2
(the state model) can start in parallel after PR 2.
