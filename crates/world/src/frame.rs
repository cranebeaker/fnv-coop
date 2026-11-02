//! The game's frame: the calls of `Main::OnIdle` (Xbox PDB; PC `0086e650`,
//! 2,272 bytes, FalloutNV.exe 1.4.0.525) in the exe's order, with the tests
//! that decide which of them run. `main` (`0086a850`) calls it once per
//! frame (`0086b3e3`) on `[011dea0c]`, the `Main` object
//! (docs/FRAME_SKELETON.md, Phase 1 PR 2).
//!
//! [`STEPS`] holds one [`FrameStep`] per direct call of `0086e650`, in call
//! order: the depth-1 rows of `research/engine-map/frame.tsv` (143 call
//! sites, so a function called twice is two steps). Each step has its
//! [`Gate`], the condition under which the call is reached, read from the
//! branches of `0086e650`, and its [`Wiring`]: the nv-rs code that is that
//! step, or open. The stages ([`Stage`]) follow FRAME_SKELETON.md's grouping,
//! which is for reading only: the exe has no stage boundaries.
//!
//! The mode inputs ([`FrameState`]):
//!
//! - **Menu mode** (`[011dea2b]`): `Interface::IsInMenuMode` (`00702360`:
//!   the interface's mode, `[011d8a80]`+0xc, is not 1, `007023a0`) or
//!   `Interface::IsPipboyOpening` (`00709bc0`: the Pip-Boy's state
//!   `[011d8a80]`+0x4bc is 2). Set at the start (`0086e6e1`) and again after
//!   the player's update (`0086e7c9`); every gate from step 23 on reads the
//!   second value. The menu of V.A.T.S. (`VATSMenu`, 1056, V.A.T.S. modes 1
//!   to 3) is a menu-mode menu: `InterfaceManager::Idle` (`0070c4a0`) handles
//!   it under interface mode 5 (`0070ca14`), which is not 1. Sleeping or
//!   waiting (`SleepWaitMenu`, 1012), the pause menu, dialogue and the
//!   Pip-Boy are menus too; the frame has no separate "paused" test.
//! - **Fader visible** (`[011dea2d]`, `00701450(1)` on the fader manager
//!   `[011d8804]`: its fader 1 is on and at alpha 1): lets the process lists
//!   and the AI work run in menu mode.
//! - **Console visible** (`[011dea2e]`, `Interface::IsConsoleVisible`
//!   `00703d50`): stops them even outside menu mode.
//! - **World frozen** (`Main` +7, `[011dea0c]`+7): set only by `00961f50`,
//!   which `00961e30` calls for the console's `ToggleFlyCam` (`TFC`, handler
//!   `005bc260`) with 1, 2 or 5: the free camera with the world stopped.
//! - **Threads** (`iNumHWThreads:General`, `011c3ea4`, read through
//!   `0043d4d0`): 1 runs the AI work on this thread, more starts the AI
//!   linear task threads (FRAME_SKELETON.md, "The actor updates").
//! - **V.A.T.S. playback** (the V.A.T.S. manager's mode, `[011f2250]`+8
//!   through `0044ddc0`, is 4, [`crate::vats::mode::PLAYBACK`]): the scene
//!   graph's field of view is not set.
//! - **Loading**: the start menu (`[011daac0]`) with its flag 0x10000
//!   (`0086efa0`, `004a4080`) skips the loading block; the start menu up
//!   without its flag 1 (`XUserInterface::XUIIsUp`, `0070edf0`) suspends the
//!   loading menu's background thread; otherwise an open in-game loading
//!   menu (`[011da0c0]`, `00705ea0`) is shown (`TES::ShowLoadingMenu`,
//!   `00457d70`).
//!
//! Before any of it, `0086e650` returns at once (`0086e69c`) when Tab
//! (`GetAsyncKeyState(9)`, tested at `0086e682`) and Alt (`0x12`,
//! `0086e69a`) are both held: only step 1 has run
//! ([`FrameState::alt_tab_held`]).
//!
//! The model takes one value of each input for the whole frame. The exe
//! reads `IsInMenuMode` again at steps 18, 106 and 141 and the thread count
//! at each `0043d4d0` step; something that changes in between (a menu opened
//! by the player's update) is not modelled.
//!
//! The disassembly and the decompiler agree on every branch here. The
//! decompiler drops two blocks (`0086ebd7`, `0086ec00`) that test a local
//! the function sets to 0 just before (`0086ebbb`), so they are never taken.
//!
//! The viewer orders its per-frame systems by these stages and steps and
//! runs them under these gates (`viewer/src/frame_order.rs`, Phase 1 PR 3).
//! The player's step, `Main::OnIdle_UpdatePlayer`, is split further in
//! [`player`] (Phase 1 PR 4); the world and time stage's callees in
//! [`world_time`] (Phase 1 PR 5); the AI task stage (the AI linear task
//! threads' work, the Havok step, the actor updates, the sky) in
//! [`ai_stage`] (Phase 1 PR 6); the interface and render stages (the
//! interface idle, `LastMinuteUpdate`, sleeping, the menus' background,
//! `Main::Swap`, the post-swap and post-thread work, the console) and the
//! controls' poll in [`interface`] (Phase 1 PR 7); which poll the player
//! and the interface read in [`input`] (Phase 1 PR 8). [`coverage`] says, for
//! every call to depth 2, which model holds it and how it is wired (Phase
//! 1's gate).

// Translated from 0086e650 (decompiled, FalloutNV.exe 1.4.0.525)

pub mod ai_stage;
pub mod coverage;
pub mod input;
pub mod interface;
pub mod player;
pub mod world_time;

/// FRAME_SKELETON.md's stages, for grouping only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Stage {
    /// Steps 1-13: the menu state, the memory free.
    FrameStart,
    /// Steps 14-17: the player, Steam, the image space.
    Player,
    /// Steps 18-47: the menu state again, saves, the timer, the controls,
    /// loading, the menu background, faders, screen splatters.
    Housekeeping,
    /// Steps 48-78: cells, the calendar, the process lists, garbage
    /// collection, trees, the current grid cell.
    WorldAndTime,
    /// Steps 79-97: the interface idle (one thread), decals, the field of
    /// view, multibound visibility.
    InterfaceAndScene,
    /// Steps 98-120: the AI work started, animations and effects, the
    /// interface idle (threads), pathing, combat, sleeping.
    AiStart,
    /// Steps 121-127: the menu background while sleeping, `Main::Swap`, the
    /// post-swap work, a display mode change.
    Render,
    /// Steps 128-143: the AI work joined, the post-thread work, the console,
    /// the scripts' optimizations, the heap sort.
    AiJoin,
}

/// What `0086e650` tests before a call. [`Gate::branches`] names the
/// branch instructions; [`Gate::holds`] evaluates the test on a
/// [`FrameState`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gate {
    /// No test.
    Always,
    /// `Interface::IsPipboyOpening` is asked only when
    /// `Interface::IsInMenuMode` was false (`0086e6c0`, `0086e7a8`).
    NotInMenuMode,
    /// The second `GetCurrentRenderedMenu` and `GetPipboy` only when the
    /// first found a rendered menu (`0086e714`).
    RenderedMenuUp,
    /// `MemoryLevelManager::RunNonDestructiveFree`: the interface's mode
    /// (`[011d8a80]`+0xc as a byte, `00424940`) is 3 (`0086e74e`) and no
    /// rendered menu other than the Pip-Boy is up (`0086e756`; worked out at
    /// `0086e714`-`0086e724`).
    FreeMemory,
    /// `Pathing::ProfilePathing` and its argument's call: the byte
    /// `[011deefc]` is set (`0086e7fd`; no direct writer found, zero at
    /// start).
    ProfilePathing,
    /// Menu mode clear (`0086e87a`): `0086ef40` counts the frames outside
    /// menu mode (`[011f9138]`).
    MenuFlagClear,
    /// `XUserInterface::XUIIsUp` is asked unless the start menu has its flag
    /// 0x10000 (`0086e8a6`).
    StartMenuNotBusy,
    /// `LoadingMenu::SuspendBackgroundThread`: as `StartMenuNotBusy`, and the
    /// start menu is up without its flag 1 (`0086e8b2`).
    StartMenuUp,
    /// `Interface::IsInGameLoadingMenuOpen` is asked when neither start-menu
    /// test held (`0086e8a6`, `0086e8b2`).
    NoStartMenu,
    /// `TES::ShowLoadingMenu(0, 0, 0)`: as `NoStartMenu`, and the in-game
    /// loading menu is open (`0086e8c5`).
    LoadingMenuOpen,
    /// The world runs: menu mode clear (`0086e918`, `0086e946`) and the world
    /// not frozen (`0086e923`, `0086e955`). The screen splatters, the cell
    /// tests, the "World" scene graph's view distance (its slot +0x104 with the
    /// frame time, `BSSceneGraph::SetViewDistanceBasedOnFrameRate`,
    /// [`world_time::SCENE_GRAPH_UPDATE`]), `Calendar::Update` with the frame
    /// time (`0084d030`).
    WorldRuns,
    /// The second cell-test question (`0086ef70`, `TES` +0x52): as
    /// `WorldRuns`, and `00451530` (`TES` +0x51 or +0x52 set) held
    /// (`0086e96b`).
    CellTests,
    /// `TES::TestAllCells(0)`: as `CellTests`, and `TES` +0x52 clear
    /// (`0086e97d`).
    TestAllCells,
    /// `TES::RunAnimations`: as `WorldRuns`, and one thread (`0086e9dc`).
    WorldRunsSingleThread,
    /// The radiation, process-level and follower lists with threads other
    /// than 1 (`0086ea0c` not taken) when [`FrameState::process_lists`]
    /// (`0086ea17`, `0086ea22`, `0086ea2d`, `0086ea38`).
    ProcessListsThreaded,
    /// The same lists with one thread (`0086ea0c` taken), with
    /// `ProcessLists::PrintLists` (`008d0600`, empty in this build) after the
    /// first, when [`FrameState::process_lists`] (`0086ea63`, `0086ea6e`,
    /// `0086ea79`, `0086ea84`).
    ProcessListsSingleThread,
    /// `BSTexturePalette::PurgeUnusedTextures`: the byte `[011f4461]`
    /// (`0086ef60`) is set (`0086ead8`); written by `00a61ad0` and by the
    /// purge itself.
    PurgeTextures,
    /// `Main::OnIdle_DoInterfaceIdle` before the AI work: one thread
    /// (`0086eb31`).
    InterfaceIdleSingleThread,
    /// `BSSceneGraph::SetCameraFOV` and its arguments' calls: the V.A.T.S.
    /// manager's mode is not 4, playback (`0086eb6d`).
    NotVatsPlayback,
    /// The AI work this frame, [`FrameState::process_lists`] (worked out at
    /// `0086ec1d`-`0086ec47` before the start and `0086ee0c`-`0086ee36`
    /// before the join): the thread count's read before the start
    /// (`0086ec65`) and before the join.
    AiTasks,
    /// `AILinearTaskThreadManager::SetMainRendering(1)` and `StartThreads`:
    /// the AI work and threads > 1 (`0086ec74` not taken).
    AiThreadsStart,
    /// `AITaskManager::StartTasksDuringRendering`: the AI work and one
    /// thread (`0086ec74` taken).
    AiTaskQueueStart,
    /// `Main::OnIdle_DoInterfaceIdle` and `IsInMenuMode` after the
    /// animations: threads > 1 (`0086ecad`) and no AI work this frame
    /// (`0086ecb5`; with AI work the AI thread calls it, FRAME_SKELETON.md).
    InterfaceIdleThreaded,
    /// `Interface::LastMinuteUpdate`: as `InterfaceIdleThreaded`, and
    /// `IsInMenuMode`, asked again here, is true (`0086ecc9`).
    LastMinuteUpdate,
    /// `PathManager::QInstance` and `Update`: the world not frozen
    /// (`0086ecd9`).
    NotFrozen,
    /// The obstacle setting's read and `CombatManager::Update`: one thread
    /// (`0086ed00`).
    AiSingleThread,
    /// `NavMeshObstacleManager::GetInstance` and `Update`: one thread
    /// (`0086ed00`) and `bUseObstacleAvoidance:Pathfinding` (`011d73e4`,
    /// default 1, read through `00408d60`) on (`0086ed11`).
    ObstacleAvoidance,
    /// `NiParallelUpdateTaskManager::BeginUpdate` (a lead): the manager
    /// `[011f5b04]` exists (`00683a60`, `0086e936`).
    ParallelUpdateBegin,
    /// `NiParallelUpdateTaskManager::EndUpdate` (a lead): it exists and its
    /// byte +0x1b0 is set (`00714a00`, `0086ed64`).
    ParallelUpdateEnd,
    /// `Interface::UpdateSleeping`: the top menu is 1012, `SleepWaitMenu`
    /// (`0086ed75`; the class `Interface::CreateSleepMenu`, `007054f0`,
    /// makes).
    SleepWaitMenuTop,
    /// `Main::RenderMenuBackground`: as `SleepWaitMenuTop`, and
    /// `UpdateSleeping` (through `SleepWaitMenu::UpdateSleeping`) answered
    /// true (`0086ed81`).
    Sleeping,
    /// `004dc360` (the display mode change): the byte `[011c6fbb]` is set
    /// (`0086edfe`); cleared after the call (`0086ee05`).
    DisplayModeChange,
    /// `AILinearTaskThreadManager::WaitForThreads` and the thread manager's
    /// getter: the AI work and threads > 1 (`0086ee45` not taken).
    AiThreadsJoin,
    /// `AITaskManager::WaitForTasksDuringRendering` (a lead): the AI work and
    /// one thread (`0086ee45` taken).
    AiTaskQueueJoin,
    /// `0070ed20(0)` and `Interface::OpenConsole`: the open-console request
    /// `[011dea2f]` (`0070ed10`) is set (`0086ee79`). Its only direct writer
    /// is `0070ed20`, called with 0 here and by `InterfaceManager::Idle`
    /// (`0070e5d3`).
    ConsoleRequested,
    /// `00950090` (the byte `[011e07b8]`, docs/CAMERA.md: the view key held)
    /// is asked when `IsInMenuMode`, asked again, is false (`0086eeb9`) and
    /// the player `[011dea3c]` exists (`0086eec2`).
    AskViewKey,
    /// `00aa7290` (a heap's free-block sort, "SortFreeBlocks", on heap
    /// number `[011deef5]`, which then moves on, wrapping at 256): in menu
    /// mode (`IsInMenuMode` asked again, `0086eeb9`), with the view key held
    /// (`0086eed4`), or once 45 seconds (`010357e8`) of frame time have
    /// gathered in `[011deef8]` (`0086eee7`); the sum is then reset.
    SortFreeBlocks,
}

/// The frame's inputs to the gates. `Default` is a frame in game mode with
/// one thread, nothing loading and no requests.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrameState {
    /// Tab and Alt both held (`0086e682`, `0086e69a`): the frame ends after
    /// step 1.
    pub alt_tab_held: bool,
    /// `Interface::IsInMenuMode` (`00702360`).
    pub in_menu_mode: bool,
    /// `Interface::IsPipboyOpening` (`00709bc0`).
    pub pipboy_opening: bool,
    /// `FaderManager::IsFaderVisible(1)` (`00701450`, `[011dea2d]`).
    pub fader_visible: bool,
    /// `Interface::IsConsoleVisible` (`00703d50`, `[011dea2e]`).
    pub console_visible: bool,
    /// `Interface::GetCurrentRenderedMenu` (`00707ad0`) found a menu.
    pub rendered_menu_up: bool,
    /// That menu is the Pip-Boy (`Interface::GetPipboy`, `00705990`).
    pub rendered_menu_is_pipboy: bool,
    /// The interface's mode (`[011d8a80]`+0xc; `00424940` reads its low
    /// byte).
    pub interface_mode: u32,
    /// `Main` +7: the free camera's frozen world.
    pub world_frozen: bool,
    /// `iNumHWThreads:General` (`011c3ea4`).
    pub threads: i32,
    /// `[011deefc]`.
    pub profile_pathing: bool,
    /// The start menu (`[011daac0]`) exists.
    pub start_menu_up: bool,
    /// Its flags (`+0x1a8`, `004a4080`).
    pub start_menu_flags: u32,
    /// `[011da0c0]`: the in-game loading menu is open (`00705ea0`).
    pub loading_menu_open: bool,
    /// `[011f5b04]` exists (`00683a60`).
    pub parallel_update_manager: bool,
    /// Its byte +0x1b0 (`00714a00`).
    pub parallel_update_flag: bool,
    /// `TES` +0x51 (`00451530` asks it with +0x52).
    pub running_cell_tests: bool,
    /// `TES` +0x52 (`0086ef70`).
    pub running_cell_tests_2: bool,
    /// `[011f4461]` (`0086ef60`).
    pub purge_textures: bool,
    /// The V.A.T.S. manager's mode (`[011f2250]`+8, [`crate::vats::mode`]).
    pub vats_mode: u32,
    /// `bUseObstacleAvoidance:Pathfinding` (`011d73e4`).
    pub obstacle_avoidance: bool,
    /// `Interface::GetTopMenuID` (`007023c0`).
    pub top_menu: u32,
    /// `Interface::UpdateSleeping` (`007056f0`) answers true.
    pub sleeping: bool,
    /// `[011c6fbb]`.
    pub display_mode_change: bool,
    /// `[011dea2f]`.
    pub console_requested: bool,
    /// The player `[011dea3c]` exists.
    pub player_exists: bool,
    /// `00950090`: `[011e07b8]`.
    pub view_key_held: bool,
    /// `[011deef8]` once this frame's time is added (`0086eea3`).
    pub sort_timer: f32,
}

impl Default for FrameState {
    fn default() -> FrameState {
        FrameState {
            alt_tab_held: false,
            in_menu_mode: false,
            pipboy_opening: false,
            fader_visible: false,
            console_visible: false,
            rendered_menu_up: false,
            rendered_menu_is_pipboy: false,
            interface_mode: 1,
            world_frozen: false,
            threads: 1,
            profile_pathing: false,
            start_menu_up: false,
            start_menu_flags: 0,
            loading_menu_open: false,
            parallel_update_manager: false,
            parallel_update_flag: false,
            running_cell_tests: false,
            running_cell_tests_2: false,
            purge_textures: false,
            vats_mode: 0,
            obstacle_avoidance: true,
            top_menu: 0,
            sleeping: false,
            display_mode_change: false,
            console_requested: false,
            player_exists: true,
            view_key_held: false,
            sort_timer: 0.0,
        }
    }
}

/// The top menu `Interface::UpdateSleeping` is asked for (`SleepWaitMenu`).
pub const SLEEP_WAIT_MENU: u32 = 1012;
/// The interface mode `RunNonDestructiveFree` waits for (`0086e74e`).
pub const FREE_MEMORY_INTERFACE_MODE: u32 = 3;
/// The start menu's flag that skips the loading block (`0086efa0`).
pub const START_MENU_BUSY: u32 = 0x1_0000;
/// The start menu's flag `XUIIsUp` wants clear (`0070edf0`).
pub const START_MENU_FLAG_1: u32 = 1;
/// The seconds of frame time between heap sorts (`010357e8`, a double).
pub const SORT_SECONDS: f32 = 45.0;

impl FrameState {
    /// Menu mode, `[011dea2b]` (`0086e6c0`-`0086e6e1`, `0086e7a8`-`0086e7c9`).
    pub fn menu_flag(&self) -> bool {
        self.in_menu_mode || self.pipboy_opening
    }

    /// The world runs: not menu mode, not frozen (`0086e918`/`0086e923`,
    /// `0086e946`/`0086e955`).
    pub fn world_runs(&self) -> bool {
        !self.menu_flag() && !self.world_frozen
    }

    /// The process lists and the AI work run: menu mode clear or the fader
    /// visible, the console hidden, the world not frozen (`0086ea17`-
    /// `0086ea38`, `0086ea63`-`0086ea84`, `0086ec1d`-`0086ec47`,
    /// `0086ee0c`-`0086ee36`).
    pub fn process_lists(&self) -> bool {
        (!self.menu_flag() || self.fader_visible) && !self.console_visible && !self.world_frozen
    }

    /// The argument of `Main::OnIdle_UpdateImageSpace` (`0086e771`-
    /// `0086e795`): 1 when the menu mode of the frame's first read is clear
    /// and the world isn't frozen.
    pub fn image_space_argument(&self) -> bool {
        self.world_runs()
    }

    /// The argument `BSTreeManager::Update` gets (`0086eae9`-`0086eb0d`): 1
    /// in menu mode or with the world frozen.
    pub fn tree_manager_argument(&self) -> bool {
        !self.world_runs()
    }

    /// The argument of `CombatManager::Update` (`0086ed1f`-`0086ed4e`): 1 in
    /// menu mode, with the fader visible or with the world frozen.
    pub fn combat_manager_argument(&self) -> bool {
        self.menu_flag() || self.fader_visible || self.world_frozen
    }
}

impl Gate {
    /// Whether the call is reached, given that the frame got past the Tab
    /// and Alt test.
    pub fn holds(self, s: &FrameState) -> bool {
        let one_thread = s.threads == 1;
        let start_busy = s.start_menu_up && s.start_menu_flags & START_MENU_BUSY != 0;
        let xui_up = s.start_menu_up && s.start_menu_flags & START_MENU_FLAG_1 == 0;
        match self {
            Gate::Always => true,
            Gate::NotInMenuMode => !s.in_menu_mode,
            Gate::RenderedMenuUp => s.rendered_menu_up,
            Gate::FreeMemory => {
                s.interface_mode & 0xff == FREE_MEMORY_INTERFACE_MODE
                    && !(s.rendered_menu_up && !s.rendered_menu_is_pipboy)
            }
            Gate::ProfilePathing => s.profile_pathing,
            Gate::MenuFlagClear => !s.menu_flag(),
            Gate::StartMenuNotBusy => !start_busy,
            Gate::StartMenuUp => !start_busy && xui_up,
            Gate::NoStartMenu => !start_busy && !xui_up,
            Gate::LoadingMenuOpen => !start_busy && !xui_up && s.loading_menu_open,
            Gate::WorldRuns => s.world_runs(),
            Gate::CellTests => s.world_runs() && (s.running_cell_tests || s.running_cell_tests_2),
            Gate::TestAllCells => {
                s.world_runs()
                    && (s.running_cell_tests || s.running_cell_tests_2)
                    && !s.running_cell_tests_2
            }
            Gate::WorldRunsSingleThread => s.world_runs() && one_thread,
            Gate::ProcessListsThreaded => !one_thread && s.process_lists(),
            Gate::ProcessListsSingleThread => one_thread && s.process_lists(),
            Gate::PurgeTextures => s.purge_textures,
            Gate::InterfaceIdleSingleThread => one_thread,
            Gate::NotVatsPlayback => s.vats_mode != u32::from(crate::vats::mode::PLAYBACK),
            Gate::AiTasks => s.process_lists(),
            Gate::AiThreadsStart => s.process_lists() && s.threads > 1,
            Gate::AiTaskQueueStart => s.process_lists() && s.threads <= 1,
            Gate::InterfaceIdleThreaded => s.threads > 1 && !s.process_lists(),
            Gate::LastMinuteUpdate => s.threads > 1 && !s.process_lists() && s.in_menu_mode,
            Gate::NotFrozen => !s.world_frozen,
            Gate::AiSingleThread => one_thread,
            Gate::ObstacleAvoidance => one_thread && s.obstacle_avoidance,
            Gate::ParallelUpdateBegin => s.parallel_update_manager,
            Gate::ParallelUpdateEnd => s.parallel_update_manager && s.parallel_update_flag,
            Gate::SleepWaitMenuTop => s.top_menu == SLEEP_WAIT_MENU,
            Gate::Sleeping => s.top_menu == SLEEP_WAIT_MENU && s.sleeping,
            Gate::DisplayModeChange => s.display_mode_change,
            Gate::AiThreadsJoin => s.process_lists() && s.threads > 1,
            Gate::AiTaskQueueJoin => s.process_lists() && s.threads <= 1,
            Gate::ConsoleRequested => s.console_requested,
            Gate::AskViewKey => !s.in_menu_mode && s.player_exists,
            Gate::SortFreeBlocks => {
                s.in_menu_mode
                    || (s.player_exists && s.view_key_held)
                    || s.sort_timer >= SORT_SECONDS
            }
        }
    }

    /// The branch instructions of `0086e650` that make up the test.
    pub fn branches(self) -> &'static [u32] {
        match self {
            Gate::Always => &[],
            Gate::NotInMenuMode => &[0x0086_e6c0, 0x0086_e7a8],
            Gate::RenderedMenuUp => &[0x0086_e714],
            Gate::FreeMemory => &[0x0086_e714, 0x0086_e724, 0x0086_e74e, 0x0086_e756],
            Gate::ProfilePathing => &[0x0086_e7fd],
            Gate::MenuFlagClear => &[0x0086_e87a],
            Gate::StartMenuNotBusy => &[0x0086_e8a6],
            Gate::StartMenuUp => &[0x0086_e8a6, 0x0086_e8b2],
            Gate::NoStartMenu => &[0x0086_e8a6, 0x0086_e8b2],
            Gate::LoadingMenuOpen => &[0x0086_e8a6, 0x0086_e8b2, 0x0086_e8c5],
            Gate::WorldRuns => &[0x0086_e918, 0x0086_e923, 0x0086_e946, 0x0086_e955],
            Gate::CellTests => &[0x0086_e946, 0x0086_e955, 0x0086_e96b],
            Gate::TestAllCells => &[0x0086_e946, 0x0086_e955, 0x0086_e96b, 0x0086_e97d],
            Gate::WorldRunsSingleThread => &[0x0086_e946, 0x0086_e955, 0x0086_e9dc],
            Gate::ProcessListsThreaded => &[
                0x0086_ea0c,
                0x0086_ea17,
                0x0086_ea22,
                0x0086_ea2d,
                0x0086_ea38,
            ],
            Gate::ProcessListsSingleThread => &[
                0x0086_ea0c,
                0x0086_ea63,
                0x0086_ea6e,
                0x0086_ea79,
                0x0086_ea84,
            ],
            Gate::PurgeTextures => &[0x0086_ead8],
            Gate::InterfaceIdleSingleThread => &[0x0086_eb31],
            Gate::NotVatsPlayback => &[0x0086_eb6d],
            Gate::AiTasks => &[
                0x0086_ec65,
                0x0086_ee15,
                0x0086_ee20,
                0x0086_ee2b,
                0x0086_ee36,
            ],
            Gate::AiThreadsStart | Gate::AiTaskQueueStart => &[0x0086_ec65, 0x0086_ec74],
            Gate::InterfaceIdleThreaded => &[0x0086_ecad, 0x0086_ecb5],
            Gate::LastMinuteUpdate => &[0x0086_ecad, 0x0086_ecb5, 0x0086_ecc9],
            Gate::NotFrozen => &[0x0086_ecd9],
            Gate::AiSingleThread => &[0x0086_ed00],
            Gate::ObstacleAvoidance => &[0x0086_ed00, 0x0086_ed11],
            Gate::ParallelUpdateBegin => &[0x0086_e936],
            Gate::ParallelUpdateEnd => &[0x0086_ed64],
            Gate::SleepWaitMenuTop => &[0x0086_ed75],
            Gate::Sleeping => &[0x0086_ed75, 0x0086_ed81],
            Gate::DisplayModeChange => &[0x0086_edfe],
            Gate::AiThreadsJoin | Gate::AiTaskQueueJoin => &[0x0086_ee36, 0x0086_ee45],
            Gate::ConsoleRequested => &[0x0086_ee79],
            Gate::AskViewKey => &[0x0086_eeb9, 0x0086_eec2],
            Gate::SortFreeBlocks => &[0x0086_eeb9, 0x0086_eec2, 0x0086_eed4, 0x0086_eee7],
        }
    }
}

/// The nv-rs code that is a step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wiring {
    /// Nothing in nv-rs is this step yet.
    Open,
    /// Part of the step exists (named); the step as a whole is still open.
    Partial(&'static str),
    /// This nv-rs code is the step.
    System(&'static str),
}

impl Wiring {
    /// Open or only partly there.
    pub fn is_open(self) -> bool {
        !matches!(self, Wiring::System(_))
    }
}

/// One direct call of `0086e650`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameStep {
    /// The function called (its PC address).
    pub address: u32,
    /// Its Xbox PDB name, as `frame.tsv` gives it (none where the engine map
    /// has none).
    pub name: Option<&'static str>,
    pub stage: Stage,
    pub gate: Gate,
    pub wiring: Wiring,
}

const fn step(
    address: u32,
    name: Option<&'static str>,
    stage: Stage,
    gate: Gate,
    wiring: Wiring,
) -> FrameStep {
    FrameStep {
        address,
        name,
        stage,
        gate,
        wiring,
    }
}

/// The interface idle's wiring (both calls, steps 80 and 105).
const IDLE_WIRING: &str = "interface::DO_INTERFACE_IDLE_STEPS and the interface manager's \
    PreIdleStuff, Idle and PostIdleStuff (viewer: the menus, then the HUD and Pip-Boy, at the AI \
    idle's position after the controls' poll, frame_order::IdleSet; the one-thread and menu-mode \
    fallback positions open)";

/// The controls' poll's wiring (step 33, `world::frame::input`).
const POLL_WIRING: &str = "world::frame::input; viewer: poll::poll_controls puts the frame's \
    device read (keys, mouse buttons, mouse motion and wheel; no pad) into the controls' buffer, \
    which the player's step read as the last frame left it";

/// The calls of `Main::OnIdle` (`0086e650`), in order.
pub const STEPS: [FrameStep; 143] = [
    step(0x0086_a830, None, Stage::FrameStart, Gate::Always, Wiring::Open),
    step(0x00af_2640, Some("BSSystemUtility::QInstance"), Stage::FrameStart, Gate::Always, Wiring::Open),
    step(0x0070_2360, Some("Interface::IsInMenuMode"), Stage::FrameStart, Gate::Always, Wiring::Open),
    step(0x0070_9bc0, Some("Interface::IsPipboyOpening"), Stage::FrameStart, Gate::NotInMenuMode, Wiring::Open),
    step(0x0070_50d0, Some("Interface::InDialog"), Stage::FrameStart, Gate::Always, Wiring::Open),
    step(0x0070_1450, Some("FaderManager::IsFaderVisible"), Stage::FrameStart, Gate::Always, Wiring::Open),
    step(0x0070_3d50, Some("Interface::IsConsoleVisible"), Stage::FrameStart, Gate::Always, Wiring::Open),
    step(0x0070_7ad0, Some("Interface::GetCurrentRenderedMenu"), Stage::FrameStart, Gate::Always, Wiring::Open),
    step(0x0070_7ad0, Some("Interface::GetCurrentRenderedMenu"), Stage::FrameStart, Gate::RenderedMenuUp, Wiring::Open),
    step(0x0070_5990, Some("Interface::GetPipboy"), Stage::FrameStart, Gate::RenderedMenuUp, Wiring::Open),
    step(0x004b_7210, None, Stage::FrameStart, Gate::Always, Wiring::Open),
    step(0x0042_4940, None, Stage::FrameStart, Gate::Always, Wiring::Open),
    step(0x0087_82b0, Some("MemoryLevelManager::RunNonDestructiveFree"), Stage::FrameStart, Gate::FreeMemory, Wiring::Open),
    step(0x0086_f940, Some("Main::OnIdle_UpdatePlayer"), Stage::Player, Gate::Always, Wiring::Partial("world::frame::player (its calls and PlayerCharacter::Update's sub-steps, in order, with their gates; viewer: frame_order::PlayerSet)")),
    step(0x006f_f580, None, Stage::Player, Gate::Always, Wiring::Open),
    step(0x006f_f860, None, Stage::Player, Gate::Always, Wiring::Open),
    step(0x0086_fd90, Some("Main::OnIdle_UpdateImageSpace"), Stage::Player, Gate::Always, Wiring::Open),
    step(0x0070_2360, Some("Interface::IsInMenuMode"), Stage::Housekeeping, Gate::Always, Wiring::Open),
    step(0x0070_9bc0, Some("Interface::IsPipboyOpening"), Stage::Housekeeping, Gate::NotInMenuMode, Wiring::Open),
    step(0x0070_50d0, Some("Interface::InDialog"), Stage::Housekeeping, Gate::Always, Wiring::Open),
    step(0x0070_1450, Some("FaderManager::IsFaderVisible"), Stage::Housekeeping, Gate::Always, Wiring::Open),
    step(0x0070_3d50, Some("Interface::IsConsoleVisible"), Stage::Housekeeping, Gate::Always, Wiring::Open),
    step(0x008d_6f30, None, Stage::Housekeeping, Gate::ProfilePathing, Wiring::Open),
    step(0x006d_a7c0, Some("Pathing::ProfilePathing"), Stage::Housekeeping, Gate::ProfilePathing, Wiring::Open),
    step(0x0085_1d90, Some("BGSSaveLoadManager::UpdateQueuedSaves"), Stage::Housekeeping, Gate::Always, Wiring::Open),
    step(0x0086_ef30, None, Stage::Housekeeping, Gate::Always, Wiring::Open),
    step(0x0086_ef90, None, Stage::Housekeeping, Gate::Always, Wiring::Open),
    step(0x0086_f190, Some("Main::OnIdle_FixActorBones"), Stage::Housekeeping, Gate::Always, Wiring::Open),
    step(0x0048_3710, None, Stage::Housekeeping, Gate::Always, Wiring::Open),
    step(0x004e_1610, None, Stage::Housekeeping, Gate::Always, Wiring::Open),
    step(0x0086_ef40, None, Stage::Housekeeping, Gate::MenuFlagClear, Wiring::Open),
    step(0x0086_f260, Some("Main::OnIdle_UpdateTimer"), Stage::Housekeeping, Gate::Always, Wiring::Partial("physics::havok::Clock (the frame timer and Havok's delta time only; viewer: clutter::time_havok_frame with physics::havok::FrameTimer)")),
    step(0x0086_f390, Some("Main::OnIdle_PollControls"), Stage::Housekeeping, Gate::Always, Wiring::Partial(POLL_WIRING)),
    step(0x00c3_dbf0, Some("IOManager::UpdateQueue"), Stage::Housekeeping, Gate::Always, Wiring::Open),
    step(0x0086_efa0, None, Stage::Housekeeping, Gate::Always, Wiring::Open),
    step(0x0070_edf0, Some("XUserInterface::XUIIsUp"), Stage::Housekeeping, Gate::StartMenuNotBusy, Wiring::Open),
    step(0x0078_cfc0, Some("LoadingMenu::SuspendBackgroundThread"), Stage::Housekeeping, Gate::StartMenuUp, Wiring::Open),
    step(0x0070_5ea0, Some("Interface::IsInGameLoadingMenuOpen"), Stage::Housekeeping, Gate::NoStartMenu, Wiring::Open),
    step(0x0045_7d70, Some("TES::ShowLoadingMenu"), Stage::Housekeeping, Gate::LoadingMenuOpen, Wiring::Open),
    step(0x0086_efe0, Some("Main::OnIdle_ScaleLOD"), Stage::Housekeeping, Gate::Always, Wiring::Open),
    step(0x0052_4c90, None, Stage::Housekeeping, Gate::Always, Wiring::Open),
    step(0x004a_0ea0, None, Stage::Housekeeping, Gate::Always, Wiring::Open),
    step(0x00b6_dd00, None, Stage::Housekeeping, Gate::Always, Wiring::Open),
    step(0x0086_f450, Some("Main::OnIdle_HandleMenuBackground"), Stage::Housekeeping, Gate::Always, Wiring::System("world::menu_background::Background::update (viewer: menu_background::update)")),
    step(0x0070_11d0, Some("FaderManager::UpdateFaders"), Stage::Housekeeping, Gate::Always, Wiring::Partial("world::living::sleep::Fade and the viewer's fade to black in game_menus (two faders only)")),
    step(0x004e_0110, Some("ScreenSplatter::Update"), Stage::Housekeeping, Gate::WorldRuns, Wiring::Open),
    step(0x004d_e600, Some("ScreenCustomSplatter::Update"), Stage::Housekeeping, Gate::WorldRuns, Wiring::Open),
    step(0x0068_3a60, None, Stage::WorldAndTime, Gate::Always, Wiring::Open),
    step(0x00a8_1a20, None, Stage::WorldAndTime, Gate::ParallelUpdateBegin, Wiring::Open),
    step(0x0045_1530, None, Stage::WorldAndTime, Gate::WorldRuns, Wiring::Open),
    step(0x0086_ef70, None, Stage::WorldAndTime, Gate::CellTests, Wiring::Open),
    step(0x0045_56d0, Some("TES::TestAllCells"), Stage::WorldAndTime, Gate::TestAllCells, Wiring::Open),
    step(0x0055_9450, None, Stage::WorldAndTime, Gate::WorldRuns, Wiring::Open),
    step(0x0084_d030, None, Stage::WorldAndTime, Gate::WorldRuns, Wiring::Open),
    step(0x0084_d030, None, Stage::WorldAndTime, Gate::WorldRuns, Wiring::Open),
    step(0x0086_7a40, Some("Calendar::Update"), Stage::WorldAndTime, Gate::WorldRuns, Wiring::Partial("world::scripting GameState::advance_clock (game time; its calendar is labelled a guess; world_time::CALENDAR_UPDATE)")),
    step(0x0043_d4d0, None, Stage::WorldAndTime, Gate::WorldRuns, Wiring::Open),
    step(0x0045_5640, Some("TES::RunAnimations"), Stage::WorldAndTime, Gate::WorldRunsSingleThread, Wiring::Open),
    step(0x0040_fbf0, None, Stage::WorldAndTime, Gate::Always, Wiring::Open),
    step(0x0097_8550, Some("ProcessLists::RunActorScripts"), Stage::WorldAndTime, Gate::Always, Wiring::Partial("viewer: scripts::run_scripts, ordered here (it runs every script it knows, and the game clock; world_time::RUN_ACTOR_SCRIPTS)")),
    step(0x0043_d4d0, None, Stage::WorldAndTime, Gate::Always, Wiring::Open),
    step(0x0097_77a0, Some("ProcessLists::UpdateRadiationList"), Stage::WorldAndTime, Gate::ProcessListsThreaded, Wiring::Open),
    step(0x0096_eb40, Some("ProcessLists::ChangeProcessLevelTempList"), Stage::WorldAndTime, Gate::ProcessListsThreaded, Wiring::Open),
    step(0x0096_e9b0, Some("ProcessLists::UpdateFollowerTempList"), Stage::WorldAndTime, Gate::ProcessListsThreaded, Wiring::Open),
    step(0x0097_77a0, Some("ProcessLists::UpdateRadiationList"), Stage::WorldAndTime, Gate::ProcessListsSingleThread, Wiring::Open),
    step(0x008d_0600, Some("ProcessLists::PrintLists"), Stage::WorldAndTime, Gate::ProcessListsSingleThread, Wiring::Open),
    step(0x0096_eb40, Some("ProcessLists::ChangeProcessLevelTempList"), Stage::WorldAndTime, Gate::ProcessListsSingleThread, Wiring::Open),
    step(0x0096_e9b0, Some("ProcessLists::UpdateFollowerTempList"), Stage::WorldAndTime, Gate::ProcessListsSingleThread, Wiring::Open),
    step(0x0040_fba0, None, Stage::WorldAndTime, Gate::Always, Wiring::Open),
    step(0x0044_6ef0, None, Stage::WorldAndTime, Gate::Always, Wiring::Open),
    step(0x0087_8080, None, Stage::WorldAndTime, Gate::Always, Wiring::Open),
    step(0x0086_ef60, None, Stage::WorldAndTime, Gate::Always, Wiring::Open),
    step(0x00a6_1cd0, Some("BSTexturePalette::PurgeUnusedTextures"), Stage::WorldAndTime, Gate::PurgeTextures, Wiring::Open),
    step(0x0086_8850, Some("GarbageCollector::Update"), Stage::WorldAndTime, Gate::Always, Wiring::Open),
    step(0x0086_8d10, Some("GarbageCollector::ClearTempEffects"), Stage::WorldAndTime, Gate::Always, Wiring::Open),
    step(0x0052_4c90, None, Stage::WorldAndTime, Gate::Always, Wiring::Open),
    step(0x0066_52e0, Some("BSTreeManager::Update"), Stage::WorldAndTime, Gate::Always, Wiring::Partial("world_time::TREE_MANAGER_UPDATE (the wind update 006658b0 is speedtree::wind, viewer: trees::blow_wind; the camera's axes in trees::sway_trees)")),
    step(0x0086_fbe0, Some("Main::OnIdle_UpdateCurrentGridCell"), Stage::WorldAndTime, Gate::Always, Wiring::Partial("world_time::UPDATE_CURRENT_GRID_CELL (world::ref_scripts has its grid-move test; viewer: exterior::stream_squares and the distant land at its sub-steps)")),
    step(0x0043_d4d0, None, Stage::InterfaceAndScene, Gate::Always, Wiring::Open),
    step(0x0086_fd70, Some("Main::OnIdle_DoInterfaceIdle"), Stage::InterfaceAndScene, Gate::InterfaceIdleSingleThread, Wiring::Partial(IDLE_WIRING)),
    step(0x0048_3710, None, Stage::InterfaceAndScene, Gate::Always, Wiring::Open),
    step(0x0049_fef0, Some("BGSDecalManager::GetInstance"), Stage::InterfaceAndScene, Gate::Always, Wiring::Open),
    step(0x0049_fff0, Some("BGSDecalManager::UpdateDecals"), Stage::InterfaceAndScene, Gate::Always, Wiring::Open),
    step(0x0052_4c90, None, Stage::InterfaceAndScene, Gate::Always, Wiring::Open),
    step(0x0071_2e60, None, Stage::InterfaceAndScene, Gate::Always, Wiring::Open),
    step(0x0044_ddc0, None, Stage::InterfaceAndScene, Gate::Always, Wiring::Open),
    step(0x0071_0ab0, None, Stage::InterfaceAndScene, Gate::NotVatsPlayback, Wiring::Open),
    step(0x0045_c670, None, Stage::InterfaceAndScene, Gate::NotVatsPlayback, Wiring::Open),
    step(0x00c5_2020, Some("BSSceneGraph::SetCameraFOV"), Stage::InterfaceAndScene, Gate::NotVatsPlayback, Wiring::Open),
    step(0x0071_0ab0, None, Stage::InterfaceAndScene, Gate::Always, Wiring::Open),
    step(0x00b5_4000, Some("BSShaderManager::SetFOV"), Stage::InterfaceAndScene, Gate::Always, Wiring::Open),
    step(0x0055_9450, None, Stage::InterfaceAndScene, Gate::Always, Wiring::Open),
    step(0x0066_29f0, Some("BSFaceGenNiNode::GetAnimationData"), Stage::InterfaceAndScene, Gate::Always, Wiring::Open),
    step(0x0045_bc80, Some("TES::ResetAllMultiBoundNodes"), Stage::InterfaceAndScene, Gate::Always, Wiring::Open),
    step(0x0045_0b80, None, Stage::InterfaceAndScene, Gate::Always, Wiring::Open),
    step(0x00b5_ac90, Some("ShadowSceneNode::UpdateOcclusionPlaneVisibility"), Stage::InterfaceAndScene, Gate::Always, Wiring::Open),
    step(0x0045_b070, Some("TES::UpdateMultiBoundVisibility"), Stage::InterfaceAndScene, Gate::Always, Wiring::Open),
    step(0x0043_d4d0, None, Stage::AiStart, Gate::AiTasks, Wiring::Open),
    step(0x008c_80e0, Some("AILinearTaskThreadManager::SetMainRendering"), Stage::AiStart, Gate::AiThreadsStart, Wiring::Open),
    step(0x0071_3d80, None, Stage::AiStart, Gate::AiThreadsStart, Wiring::Open),
    step(0x008c_78c0, Some("AILinearTaskThreadManager::StartThreads"), Stage::AiStart, Gate::AiThreadsStart, Wiring::Open),
    step(0x008c_a070, Some("AITaskManager::StartTasksDuringRendering"), Stage::AiStart, Gate::AiTaskQueueStart, Wiring::Open),
    step(0x0086_fc60, Some("Main::OnIdle_UpdateAnimationsAndEffects"), Stage::AiStart, Gate::Always, Wiring::Open),
    step(0x0043_d4d0, None, Stage::AiStart, Gate::Always, Wiring::Open),
    step(0x0086_fd70, Some("Main::OnIdle_DoInterfaceIdle"), Stage::AiStart, Gate::InterfaceIdleThreaded, Wiring::Partial(IDLE_WIRING)),
    step(0x0070_2360, Some("Interface::IsInMenuMode"), Stage::AiStart, Gate::InterfaceIdleThreaded, Wiring::Open),
    step(0x0070_58e0, Some("Interface::LastMinuteUpdate"), Stage::AiStart, Gate::LastMinuteUpdate, Wiring::Open),
    step(0x0047_d0b0, Some("PathManager::QInstance"), Stage::AiStart, Gate::NotFrozen, Wiring::Open),
    step(0x006e_bc50, Some("PathManager::Update"), Stage::AiStart, Gate::NotFrozen, Wiring::Open),
    step(0x0055_2ba0, None, Stage::AiStart, Gate::Always, Wiring::Open),
    step(0x006a_61b0, Some("NavMeshRender::Update"), Stage::AiStart, Gate::Always, Wiring::Open),
    step(0x0043_d4d0, None, Stage::AiStart, Gate::Always, Wiring::Open),
    step(0x0040_8d60, None, Stage::AiStart, Gate::AiSingleThread, Wiring::Open),
    step(0x006c_0720, Some("NavMeshObstacleManager::GetInstance"), Stage::AiStart, Gate::ObstacleAvoidance, Wiring::Open),
    step(0x006c_3640, Some("NavMeshObstacleManager::Update"), Stage::AiStart, Gate::ObstacleAvoidance, Wiring::Open),
    step(0x0099_1500, Some("CombatManager::Update"), Stage::AiStart, Gate::AiSingleThread, Wiring::Open),
    step(0x0071_4a00, None, Stage::AiStart, Gate::Always, Wiring::Open),
    step(0x00a8_1a80, None, Stage::AiStart, Gate::ParallelUpdateEnd, Wiring::Open),
    step(0x0070_23c0, Some("Interface::GetTopMenuID"), Stage::AiStart, Gate::Always, Wiring::Open),
    step(0x0070_56f0, Some("Interface::UpdateSleeping"), Stage::AiStart, Gate::SleepWaitMenuTop, Wiring::Partial("interface::UPDATE_SLEEPING_STEPS (viewer: game_menus::update_sleeping)")),
    step(0x0087_1dc0, Some("Main::RenderMenuBackground"), Stage::Render, Gate::Sleeping, Wiring::Open),
    step(0x0057_ab70, None, Stage::Render, Gate::Always, Wiring::Open),
    step(0x00b6_0040, None, Stage::Render, Gate::Always, Wiring::Open),
    step(0x0047_e040, Some("TESActorBaseData::GetAlignmentForKarma"), Stage::Render, Gate::Always, Wiring::Open),
    step(0x0086_ff70, Some("Main::Swap"), Stage::Render, Gate::Always, Wiring::Open),
    step(0x0087_05d0, Some("Main::PostSwapProcess"), Stage::Render, Gate::Always, Wiring::Partial("interface::POST_SWAP_STEPS (viewer: sounds, radio, music, ai::move_offstage)")),
    step(0x004d_c360, None, Stage::Render, Gate::DisplayModeChange, Wiring::Open),
    step(0x0043_d4d0, None, Stage::AiJoin, Gate::AiTasks, Wiring::Open),
    step(0x0071_3d80, None, Stage::AiJoin, Gate::AiThreadsJoin, Wiring::Open),
    step(0x008c_7990, Some("AILinearTaskThreadManager::WaitForThreads"), Stage::AiJoin, Gate::AiThreadsJoin, Wiring::Open),
    step(0x008c_a300, Some("AITaskManager::WaitForTasksDuringRendering"), Stage::AiJoin, Gate::AiTaskQueueJoin, Wiring::Open),
    step(0x0086_f6a0, Some("Main::UpdateNonRenderSafeAITasks"), Stage::AiJoin, Gate::Always, Wiring::Open),
    step(0x0087_0610, Some("Main::OnIdle_PostThreadsProcess"), Stage::AiJoin, Gate::Always, Wiring::Open),
    step(0x0070_ed10, None, Stage::AiJoin, Gate::Always, Wiring::Open),
    step(0x0070_ed20, None, Stage::AiJoin, Gate::ConsoleRequested, Wiring::Open),
    step(0x0070_3e10, Some("Interface::OpenConsole"), Stage::AiJoin, Gate::ConsoleRequested, Wiring::Open),
    step(0x00a2_9680, None, Stage::AiJoin, Gate::Always, Wiring::Open),
    step(0x005a_e270, Some("Script::ClearOptimizations"), Stage::AiJoin, Gate::Always, Wiring::Open),
    step(0x005a_9d60, Some("ScriptLocals::ClearOptimizations"), Stage::AiJoin, Gate::Always, Wiring::Open),
    step(0x0084_d030, None, Stage::AiJoin, Gate::Always, Wiring::Open),
    step(0x0070_2360, Some("Interface::IsInMenuMode"), Stage::AiJoin, Gate::Always, Wiring::Open),
    step(0x0095_0090, None, Stage::AiJoin, Gate::AskViewKey, Wiring::Open),
    step(0x00aa_7290, None, Stage::AiJoin, Gate::SortFreeBlocks, Wiring::Open),
];

/// Whether step `i` (an index into [`STEPS`]) runs in a frame with these
/// inputs: the frame got past the Tab and Alt test (`0086e69c`), which
/// only step 1 comes before, and the step's gate holds.
pub fn step_runs(s: &FrameState, i: usize) -> bool {
    if s.alt_tab_held {
        return i == 0;
    }
    STEPS[i].gate.holds(s)
}

/// The steps that run in a frame with these inputs, as indices into
/// [`STEPS`]: only step 1 when Tab and Alt are held (`0086e69c`).
pub fn steps_run(s: &FrameState) -> Vec<usize> {
    (0..STEPS.len()).filter(|&i| step_runs(s, i)).collect()
}

/// The stages in the exe's order: the order in which [`STEPS`] reaches
/// them (each stage's steps follow each other).
pub fn stages() -> Vec<Stage> {
    let mut out: Vec<Stage> = Vec::new();
    for step in &STEPS {
        if out.last() != Some(&step.stage) {
            out.push(step.stage);
        }
    }
    out
}

/// Whether the frame gets into `stage` at all. No stage has a test of its
/// own (the stages are for reading); only the Tab and Alt return
/// (`0086e69c`) skips every stage after the first.
pub fn stage_reached(s: &FrameState, stage: Stage) -> bool {
    !s.alt_tab_held || stage == STEPS[0].stage
}

/// The first step that calls `address` (an index into [`STEPS`]).
pub fn step_of(address: u32) -> Option<usize> {
    STEPS.iter().position(|s| s.address == address)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `research/engine-map/frame.tsv`, the committed call tree.
    const FRAME_TSV: &str = include_str!("../../../research/engine-map/frame.tsv");

    fn index(address: u32, nth: usize) -> usize {
        STEPS
            .iter()
            .enumerate()
            .filter(|(_, s)| s.address == address)
            .nth(nth)
            .map(|(i, _)| i)
            .expect("step")
    }

    fn runs(s: &FrameState, address: u32, nth: usize) -> bool {
        steps_run(s).contains(&index(address, nth))
    }

    #[test]
    fn order_is_frame_tsv_depth_1() {
        let mut rows = Vec::new();
        for line in FRAME_TSV.lines() {
            if line.starts_with('#') || line.starts_with("seq\t") {
                continue;
            }
            let f: Vec<&str> = line.split('\t').collect();
            if f[1] != "1" {
                continue;
            }
            let call: usize = f[2].parse().unwrap();
            let address = u32::from_str_radix(f[3], 16).unwrap();
            let name = Some(f[4]).filter(|n| !n.is_empty());
            rows.push((call, address, name));
        }
        assert_eq!(rows.len(), STEPS.len());
        for (i, (call, address, name)) in rows.into_iter().enumerate() {
            assert_eq!(call, i + 1);
            assert_eq!(STEPS[i].address, address, "step {call}");
            assert_eq!(STEPS[i].name, name, "step {call}");
        }
    }

    #[test]
    fn stages_follow_each_other() {
        assert!(STEPS.windows(2).all(|w| w[0].stage <= w[1].stage));
        assert_eq!(
            stages(),
            [
                Stage::FrameStart,
                Stage::Player,
                Stage::Housekeeping,
                Stage::WorldAndTime,
                Stage::InterfaceAndScene,
                Stage::AiStart,
                Stage::Render,
                Stage::AiJoin,
            ]
        );
    }

    #[test]
    fn stages_reached_and_steps_found() {
        let game = FrameState::default();
        assert!(stages().into_iter().all(|st| stage_reached(&game, st)));
        let held = FrameState {
            alt_tab_held: true,
            ..game
        };
        assert_eq!(
            stages()
                .into_iter()
                .filter(|&st| stage_reached(&held, st))
                .collect::<Vec<_>>(),
            [Stage::FrameStart]
        );
        assert_eq!(step_of(0x0086_f450), Some(index(0x0086_f450, 0)));
        assert_eq!(step_of(0x0070_2360), Some(2));
        assert_eq!(step_of(0x1234_5678), None);
        assert!((0..STEPS.len()).all(|i| step_runs(&game, i) == STEPS[i].gate.holds(&game)));
    }

    #[test]
    fn game_mode_one_thread() {
        let run = steps_run(&FrameState::default());
        // Not run in game mode with one thread: the second rendered-menu
        // question, the memory free, profiling, the start menu's suspend,
        // the loading menu, the parallel update, the cell tests, the
        // threaded lists, the purge, the threads, the threaded interface
        // idle, sleeping, the display mode, the console, the heap sort.
        let not: Vec<usize> = (0..STEPS.len()).filter(|i| !run.contains(i)).collect();
        let addresses: Vec<u32> = not.iter().map(|&i| STEPS[i].address).collect();
        assert_eq!(
            addresses,
            [
                0x0070_7ad0,
                0x0070_5990,
                0x0087_82b0,
                0x008d_6f30,
                0x006d_a7c0,
                0x0078_cfc0,
                0x0045_7d70,
                0x00a8_1a20,
                0x0086_ef70,
                0x0045_56d0,
                0x0097_77a0,
                0x0096_eb40,
                0x0096_e9b0,
                0x00a6_1cd0,
                0x008c_80e0,
                0x0071_3d80,
                0x008c_78c0,
                0x0086_fd70,
                0x0070_2360,
                0x0070_58e0,
                0x00a8_1a80,
                0x0070_56f0,
                0x0087_1dc0,
                0x004d_c360,
                0x0071_3d80,
                0x008c_7990,
                0x0070_ed20,
                0x0070_3e10,
                0x00aa_7290,
            ]
        );
    }

    // `0086e682`, `0086e69a`: Tab and Alt held return at `0086e69c`.
    #[test]
    fn alt_tab_ends_the_frame() {
        let s = FrameState {
            alt_tab_held: true,
            ..FrameState::default()
        };
        assert_eq!(steps_run(&s), [0]);
    }

    // `0086e6c0`, `0086e7a8`: `IsPipboyOpening` only outside menu mode.
    #[test]
    fn pipboy_opening_asked_outside_menu_mode() {
        let game = FrameState::default();
        let menu = FrameState {
            in_menu_mode: true,
            ..game
        };
        for nth in 0..2 {
            assert!(runs(&game, 0x0070_9bc0, nth));
            assert!(!runs(&menu, 0x0070_9bc0, nth));
        }
        assert!(menu.menu_flag());
        assert!(FrameState {
            pipboy_opening: true,
            ..game
        }
        .menu_flag());
    }

    // `0086e714`: the second `GetCurrentRenderedMenu` and `GetPipboy`.
    #[test]
    fn rendered_menu_asked_again() {
        let s = FrameState {
            rendered_menu_up: true,
            ..FrameState::default()
        };
        assert!(runs(&s, 0x0070_7ad0, 1) && runs(&s, 0x0070_5990, 0));
        assert!(!runs(&FrameState::default(), 0x0070_7ad0, 1));
    }

    // `0086e74e`, `0086e756`: the memory free in interface mode 3 without a
    // rendered menu other than the Pip-Boy.
    #[test]
    fn memory_free() {
        let s = FrameState {
            interface_mode: 3,
            ..FrameState::default()
        };
        assert!(runs(&s, 0x0087_82b0, 0));
        let pipboy = FrameState {
            rendered_menu_up: true,
            rendered_menu_is_pipboy: true,
            ..s
        };
        assert!(runs(&pipboy, 0x0087_82b0, 0));
        let other = FrameState {
            rendered_menu_is_pipboy: false,
            ..pipboy
        };
        assert!(!runs(&other, 0x0087_82b0, 0));
        assert!(!runs(
            &FrameState {
                interface_mode: 2,
                ..s
            },
            0x0087_82b0,
            0
        ));
    }

    // `0086e7fd`: pathing profiled when `[011deefc]` is set.
    #[test]
    fn pathing_profile() {
        let s = FrameState {
            profile_pathing: true,
            ..FrameState::default()
        };
        assert!(runs(&s, 0x006d_a7c0, 0) && runs(&s, 0x008d_6f30, 0));
    }

    // `0086e87a`: the game-mode frame counter.
    #[test]
    fn game_mode_frame_counter() {
        assert!(runs(&FrameState::default(), 0x0086_ef40, 0));
        let pipboy = FrameState {
            pipboy_opening: true,
            ..FrameState::default()
        };
        assert!(!runs(&pipboy, 0x0086_ef40, 0));
    }

    // `0086e8a6`, `0086e8b2`, `0086e8c5`: the loading block.
    #[test]
    fn loading_block() {
        let loading = FrameState {
            loading_menu_open: true,
            ..FrameState::default()
        };
        assert!(runs(&loading, 0x0045_7d70, 0));
        assert!(!runs(&loading, 0x0078_cfc0, 0));
        // The start menu up without its flag 1: the background thread is
        // suspended, the loading menu not asked.
        let start = FrameState {
            start_menu_up: true,
            ..loading
        };
        assert!(runs(&start, 0x0078_cfc0, 0));
        assert!(!runs(&start, 0x0070_5ea0, 0) && !runs(&start, 0x0045_7d70, 0));
        // With its flag 1 the start menu doesn't count as up.
        let flag_1 = FrameState {
            start_menu_flags: START_MENU_FLAG_1,
            ..start
        };
        assert!(!runs(&flag_1, 0x0078_cfc0, 0) && runs(&flag_1, 0x0045_7d70, 0));
        // Its flag 0x10000 skips all of it.
        let busy = FrameState {
            start_menu_flags: START_MENU_BUSY,
            ..start
        };
        for address in [0x0070_edf0, 0x0078_cfc0, 0x0070_5ea0, 0x0045_7d70] {
            assert!(!runs(&busy, address, 0));
        }
    }

    // `0086e918`/`0086e923`, `0086e946`/`0086e955`: the world stops in menu
    // mode (V.A.T.S.'s menu, sleeping and waiting, dialogue, the Pip-Boy
    // opening) and with the free camera's frozen world.
    #[test]
    fn world_stops_in_menu_mode_and_frozen() {
        let world = [0x004e_0110, 0x004d_e600, 0x0045_1530, 0x0086_7a40];
        let game = FrameState::default();
        for s in [
            FrameState {
                in_menu_mode: true,
                ..game
            },
            FrameState {
                pipboy_opening: true,
                ..game
            },
            FrameState {
                world_frozen: true,
                ..game
            },
        ] {
            for address in world {
                assert!(!runs(&s, address, 0));
            }
            // The calendar's frame-time reads too.
            assert!(!runs(&s, 0x0084_d030, 0) && !runs(&s, 0x0084_d030, 1));
        }
        for address in world {
            assert!(runs(&game, address, 0));
        }
        // The last frame-time read (the heap sort's sum) always runs.
        assert!(runs(
            &FrameState {
                in_menu_mode: true,
                ..game
            },
            0x0084_d030,
            2
        ));
    }

    // `0086e96b`, `0086e97d`: `TestAllCells` with +0x51 and not +0x52.
    #[test]
    fn cell_tests() {
        let s = FrameState {
            running_cell_tests: true,
            ..FrameState::default()
        };
        assert!(runs(&s, 0x0086_ef70, 0) && runs(&s, 0x0045_56d0, 0));
        let both = FrameState {
            running_cell_tests_2: true,
            ..s
        };
        assert!(runs(&both, 0x0086_ef70, 0) && !runs(&both, 0x0045_56d0, 0));
        let menu = FrameState {
            in_menu_mode: true,
            ..s
        };
        assert!(!runs(&menu, 0x0045_56d0, 0));
    }

    // `0086e9dc`: `TES::RunAnimations` with one thread, the world running.
    #[test]
    fn run_animations() {
        assert!(runs(&FrameState::default(), 0x0045_5640, 0));
        assert!(!runs(
            &FrameState {
                threads: 2,
                ..FrameState::default()
            },
            0x0045_5640,
            0
        ));
        let menu = FrameState {
            in_menu_mode: true,
            ..FrameState::default()
        };
        assert!(!runs(&menu, 0x0045_5640, 0));
    }

    // `0086ea0c`: which copy of the lists runs; `0086ea17`-`0086ea38` and
    // `0086ea63`-`0086ea84`: menu mode stops them unless the fader is up,
    // the console and the frozen world stop them always.
    #[test]
    fn process_lists() {
        let one = FrameState::default();
        let two = FrameState { threads: 2, ..one };
        assert!(runs(&one, 0x0097_77a0, 1) && !runs(&one, 0x0097_77a0, 0));
        assert!(runs(&one, 0x008d_0600, 0));
        assert!(runs(&two, 0x0097_77a0, 0) && !runs(&two, 0x0097_77a0, 1));
        assert!(!runs(&two, 0x008d_0600, 0));
        for s in [one, two] {
            let menu = FrameState {
                in_menu_mode: true,
                ..s
            };
            assert!(!menu.process_lists());
            assert!(FrameState {
                fader_visible: true,
                ..menu
            }
            .process_lists());
            assert!(!FrameState {
                console_visible: true,
                ..s
            }
            .process_lists());
            assert!(!FrameState {
                world_frozen: true,
                fader_visible: true,
                ..s
            }
            .process_lists());
            let n = usize::from(s.threads == 1);
            assert!(!runs(&menu, 0x0096_e9b0, n));
            assert!(runs(
                &FrameState {
                    fader_visible: true,
                    ..menu
                },
                0x0096_e9b0,
                n
            ));
        }
        // The actors' scripts run whatever the mode.
        let menu = FrameState {
            in_menu_mode: true,
            console_visible: true,
            ..one
        };
        assert!(runs(&menu, 0x0097_8550, 0));
    }

    // `0086ead8`: the texture purge on request.
    #[test]
    fn purge_textures() {
        assert!(!runs(&FrameState::default(), 0x00a6_1cd0, 0));
        assert!(runs(
            &FrameState {
                purge_textures: true,
                ..FrameState::default()
            },
            0x00a6_1cd0,
            0
        ));
    }

    // `0086eb31`: the interface idle before the AI work, one thread only.
    #[test]
    fn interface_idle_single_thread() {
        assert!(runs(&FrameState::default(), 0x0086_fd70, 0));
        assert!(!runs(
            &FrameState {
                threads: 4,
                ..FrameState::default()
            },
            0x0086_fd70,
            0
        ));
    }

    // `0086eb6d`: no scene-graph field of view in V.A.T.S. playback.
    #[test]
    fn vats_playback_keeps_the_fov() {
        let playback = FrameState {
            vats_mode: u32::from(crate::vats::mode::PLAYBACK),
            ..FrameState::default()
        };
        assert!(!runs(&playback, 0x00c5_2020, 0) && !runs(&playback, 0x0071_0ab0, 0));
        // The shader manager's field of view is set either way.
        assert!(runs(&playback, 0x0071_0ab0, 1) && runs(&playback, 0x00b5_4000, 0));
        // V.A.T.S.'s menu (modes 1-3) sets it.
        for mode in 1..4 {
            let menu = FrameState {
                vats_mode: mode,
                in_menu_mode: true,
                ..FrameState::default()
            };
            assert!(runs(&menu, 0x00c5_2020, 0));
        }
    }

    // `0086ec65`, `0086ec74`: the AI work starts on the threads or the task
    // queue.
    #[test]
    fn ai_start() {
        let one = FrameState::default();
        let two = FrameState { threads: 2, ..one };
        assert!(runs(&one, 0x008c_a070, 0) && !runs(&one, 0x008c_78c0, 0));
        assert!(runs(&two, 0x008c_78c0, 0) && !runs(&two, 0x008c_a070, 0));
        let menu = FrameState {
            in_menu_mode: true,
            ..two
        };
        assert!(!runs(&menu, 0x0043_d4d0, 3) && !runs(&menu, 0x008c_78c0, 0));
    }

    // `0086ecad`, `0086ecb5`, `0086ecc9`: with threads and no AI work the
    // main thread runs the interface idle, and `LastMinuteUpdate` in menu
    // mode.
    #[test]
    fn interface_idle_threaded() {
        let two = FrameState {
            threads: 2,
            ..FrameState::default()
        };
        assert!(!runs(&two, 0x0086_fd70, 1));
        let menu = FrameState {
            in_menu_mode: true,
            ..two
        };
        assert!(runs(&menu, 0x0086_fd70, 1) && runs(&menu, 0x0070_58e0, 0));
        let pipboy = FrameState {
            pipboy_opening: true,
            ..two
        };
        assert!(runs(&pipboy, 0x0086_fd70, 1) && !runs(&pipboy, 0x0070_58e0, 0));
        assert!(!runs(
            &FrameState {
                in_menu_mode: true,
                ..FrameState::default()
            },
            0x0086_fd70,
            1
        ));
    }

    // `0086ecd9`: the path manager stops only with the frozen world.
    #[test]
    fn path_manager() {
        let menu = FrameState {
            in_menu_mode: true,
            ..FrameState::default()
        };
        assert!(runs(&menu, 0x006e_bc50, 0));
        assert!(!runs(
            &FrameState {
                world_frozen: true,
                ..FrameState::default()
            },
            0x006e_bc50,
            0
        ));
    }

    // `0086ed00`, `0086ed11`: obstacles and combat on the main thread with
    // one thread; obstacles with `bUseObstacleAvoidance`.
    #[test]
    fn obstacles_and_combat() {
        let one = FrameState::default();
        assert!(runs(&one, 0x006c_3640, 0) && runs(&one, 0x0099_1500, 0));
        let off = FrameState {
            obstacle_avoidance: false,
            ..one
        };
        assert!(!runs(&off, 0x006c_3640, 0) && runs(&off, 0x0099_1500, 0));
        let two = FrameState { threads: 2, ..one };
        assert!(!runs(&two, 0x0040_8d60, 0) && !runs(&two, 0x0099_1500, 0));
        // Combat's argument: menu mode, the fader or the frozen world.
        assert!(!one.combat_manager_argument());
        assert!(FrameState {
            fader_visible: true,
            ..one
        }
        .combat_manager_argument());
        assert!(FrameState {
            world_frozen: true,
            ..one
        }
        .combat_manager_argument());
        assert!(FrameState {
            pipboy_opening: true,
            ..one
        }
        .combat_manager_argument());
    }

    // `0086e936`, `0086ed64`: the parallel update's begin and end.
    #[test]
    fn parallel_update() {
        let s = FrameState {
            parallel_update_manager: true,
            ..FrameState::default()
        };
        assert!(runs(&s, 0x00a8_1a20, 0) && !runs(&s, 0x00a8_1a80, 0));
        let flag = FrameState {
            parallel_update_flag: true,
            ..s
        };
        assert!(runs(&flag, 0x00a8_1a80, 0));
    }

    // `0086ed75`, `0086ed81`: sleeping or waiting redraws the menu
    // background.
    #[test]
    fn sleeping() {
        let menu = FrameState {
            in_menu_mode: true,
            top_menu: SLEEP_WAIT_MENU,
            ..FrameState::default()
        };
        assert!(runs(&menu, 0x0070_56f0, 0) && !runs(&menu, 0x0087_1dc0, 0));
        assert!(runs(
            &FrameState {
                sleeping: true,
                ..menu
            },
            0x0087_1dc0,
            0
        ));
        let other = FrameState {
            top_menu: 1001,
            sleeping: true,
            ..menu
        };
        assert!(!runs(&other, 0x0070_56f0, 0) && !runs(&other, 0x0087_1dc0, 0));
    }

    // `0086edfe`: the display mode change on request.
    #[test]
    fn display_mode_change() {
        let s = FrameState {
            display_mode_change: true,
            ..FrameState::default()
        };
        assert!(runs(&s, 0x004d_c360, 0));
    }

    // `0086ee15`-`0086ee36`, `0086ee45`: the AI work joined as it started.
    #[test]
    fn ai_join() {
        let one = FrameState::default();
        let two = FrameState { threads: 3, ..one };
        assert!(runs(&one, 0x008c_a300, 0) && !runs(&one, 0x008c_7990, 0));
        assert!(runs(&two, 0x008c_7990, 0) && !runs(&two, 0x008c_a300, 0));
        let console = FrameState {
            console_visible: true,
            ..two
        };
        assert!(!runs(&console, 0x008c_7990, 0) && !runs(&console, 0x0043_d4d0, 6));
        for s in [
            one,
            two,
            console,
            FrameState {
                in_menu_mode: true,
                ..one
            },
        ] {
            assert_eq!(runs(&s, 0x008c_a070, 0), runs(&s, 0x008c_a300, 0));
            assert_eq!(runs(&s, 0x008c_78c0, 0), runs(&s, 0x008c_7990, 0));
        }
    }

    // `0086ee79`: the console opened on request.
    #[test]
    fn console_request() {
        let s = FrameState {
            console_requested: true,
            ..FrameState::default()
        };
        assert!(runs(&s, 0x0070_ed20, 0) && runs(&s, 0x0070_3e10, 0));
    }

    // `0086eeb9`, `0086eec2`, `0086eed4`, `0086eee7`: the heap sort.
    #[test]
    fn heap_sort() {
        let game = FrameState::default();
        assert!(runs(&game, 0x0095_0090, 0) && !runs(&game, 0x00aa_7290, 0));
        assert!(runs(
            &FrameState {
                sort_timer: 45.0,
                ..game
            },
            0x00aa_7290,
            0
        ));
        assert!(!runs(
            &FrameState {
                sort_timer: 44.9,
                ..game
            },
            0x00aa_7290,
            0
        ));
        assert!(runs(
            &FrameState {
                view_key_held: true,
                ..game
            },
            0x00aa_7290,
            0
        ));
        let menu = FrameState {
            in_menu_mode: true,
            ..game
        };
        assert!(!runs(&menu, 0x0095_0090, 0) && runs(&menu, 0x00aa_7290, 0));
        let no_player = FrameState {
            player_exists: false,
            view_key_held: true,
            ..game
        };
        assert!(!runs(&no_player, 0x0095_0090, 0) && !runs(&no_player, 0x00aa_7290, 0));
    }

    // `0086e771`-`0086e795`, `0086eae9`-`0086eb0d`: the image space's and
    // the tree manager's arguments.
    #[test]
    fn arguments() {
        let game = FrameState::default();
        assert!(game.image_space_argument() && !game.tree_manager_argument());
        for s in [
            FrameState {
                in_menu_mode: true,
                ..game
            },
            FrameState {
                pipboy_opening: true,
                ..game
            },
            FrameState {
                world_frozen: true,
                ..game
            },
        ] {
            assert!(!s.image_space_argument() && s.tree_manager_argument());
        }
    }

    #[test]
    fn every_gate_names_its_branches() {
        for s in &STEPS {
            let b = s.gate.branches();
            assert_eq!(b.is_empty(), s.gate == Gate::Always, "{:08x}", s.address);
            assert!(b.iter().all(|a| (0x0086_e650..0x0086_ef30).contains(a)));
        }
    }

    #[test]
    fn wiring() {
        let systems: Vec<u32> = STEPS
            .iter()
            .filter(|s| !s.wiring.is_open())
            .map(|s| s.address)
            .collect();
        assert_eq!(systems, [0x0086_f450]);
    }
}
