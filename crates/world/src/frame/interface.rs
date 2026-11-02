//! The frame's interface and render stages: the interface idle
//! (`Main::OnIdle_DoInterfaceIdle`, `0086fd70`, with `Interface::PreIdleStuff`,
//! `Interface::Idle` and `Interface::PostIdleStuff` and the interface
//! manager's functions behind them), `Interface::LastMinuteUpdate`,
//! `Interface::UpdateSleeping`, `Main::RenderMenuBackground`, `Main::Swap`,
//! `Main::PostSwapProcess`, `Main::UpdateNonRenderSafeAITasks`,
//! `Main::OnIdle_PostThreadsProcess`, `Interface::OpenConsole`, and the
//! controls' poll (`Main::OnIdle_PollControls`, stage 3, which decides
//! when the interface and the player see each input), in the exe's order,
//! with the tests that decide which of their calls run
//! (docs/FRAME_SKELETON.md, "PR 7 result").
//!
//! As `world::frame::ai_stage` does for stage 6, [`FUNCTIONS`] holds for
//! each function the calls doing its work (not the queries: getters, the
//! interface manager's singleton `004b7210`, settings, tile values, the
//! controls' object `007fdf30`/`00877720`, list nodes), in the order of their
//! call sites, each with a [`SubGate`] (a test of the function's own
//! branches on an [`InterfaceState`]) and, where the call also depends on
//! its block's own tests (a key pressed, a menu found, a list entry), those
//! branches as [`SubStep::own_tests`], not modelled. `InterfaceManager::Idle`
//! (`0070c4a0`, 9,894 bytes, 398 calls) is split into the calls doing its
//! parts, not each of its calls.
//!
//! **Who calls the interface idle.** `Main::OnIdle_DoInterfaceIdle` is
//! called once a frame by one of: `Main::OnIdle` before the AI work with one
//! thread (step 80, `0086eb36`); the AI linear task thread's first call
//! (`008c7be9` in the combined function, `008c7db9` in thread 1) when the
//! AI work runs with more threads; or `Main::OnIdle` after
//! `Main::OnIdle_UpdateAnimationsAndEffects` (step 105, `0086ecba`) when it
//! doesn't ([`idle_caller`]).
//!
//! **Input.** `Main::OnIdle_PollControls` (step 33, `0086f390`) polls the
//! controls (`Controls::Poll`, `00a23010`) once a frame, after the player's
//! update (step 14), which so reads what the previous frame polled; the
//! interface idle (stage 5 or 6) reads this frame's. Each input is so seen by
//! the interface first and by the player in the next frame, if menu mode
//! (which the interface sets) lets the player's update run. The poll clears
//! the user actions while V.A.T.S. plays back or the Pip-Boy comes up
//! (`Controls::ClearUserActions`, `00a253d0`); the interface clears the
//! keystrokes as the menus take over (`InterfaceManager::PreIdleStuff`,
//! mode 3 → 5, `0070b96a`), as they give the game back
//! (`InterfaceManager::PostIdleStuff`, mode 4 → 1, `007122e7`), and with
//! the console's key (`0070e61f`); a menu that takes a key in `Idle` gets it
//! through its own handlers (slots +0x30 and +0xc of the frontmost menu).
//!
//! The interface manager's mode (`InterfaceManager` +0xc, `cMenuMode`):
//! 1 the game, 2 menus up, 3 menus coming up (fading in, `PreIdleStuff`
//! moves it to 5 once not locked for a fade, +0x11), 5 menus taking over
//! (`PreIdleStuff` moves it to 2 the next frame), 4 menus going away
//! (`PostIdleStuff` moves it to 1 once no fade is part-way).
//!
//! Names are Xbox PDB names (ADR-0002): the engine map's, or `frame.tsv`'s
//! for the pairs Phase 1 PR 1 made (`Interface::PreIdleStuff`, `Idle`,
//! `PostIdleStuff`, `Main::Swap`, `Main::PostSwapProcess`,
//! `Main::OnIdle_PostThreadsProcess`, `Main::OnIdle_DoInterfaceIdle`).
//! `00711ea0` is `InterfaceManager::PostIdleStuff` by its place only (the
//! third call of the interface idle, as on the Xbox), a lead. Every call
//! site and branch named is from the disassembly of FalloutNV.exe 1.4.0.525
//! (the read-only Ghidra server). `00711ea0` has no translation in the
//! engine crate; for the others its tests drive the translation and check
//! its calls against this model ([`follows`]).
//!
//! The viewer wires post-swap audio and process-list work to these sets,
//! the controls' poll to `Controls::Poll`'s sub-step, and runs its menus at
//! the AI idle's position after it ([`super::input`], Phase 1 PR 8). The
//! idle's fallback routing (one thread, menu mode), the menus' exact
//! sub-step places and actual render submission remain open; Bevy's render
//! schedule is not a call of `Main::Swap`.

// Translated from 0086fd70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 007027e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 0070b8f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 00702810 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 0070c4a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 00702840 (decompiled, FalloutNV.exe 1.4.0.525)
// InterfaceManager::PostIdleStuff (00711ea0, a lead): its calls read from the
// disassembly (FalloutNV.exe 1.4.0.525); no translation in the engine crate.
// Translated from 007058e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 00713c70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 007056f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 00871dc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 0086ff70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 008705d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 0086f640 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 0086f890 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 0086f6a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 00870610 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 00703e10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 0086f390 (decompiled, FalloutNV.exe 1.4.0.525)

pub use super::world_time::Callee;
use super::Wiring;

/// `Main::OnIdle` (Xbox PDB).
pub const ON_IDLE: u32 = 0x0086_e650;
/// `Main::OnIdle_DoInterfaceIdle`.
pub const DO_INTERFACE_IDLE: u32 = 0x0086_fd70;
/// `Interface::PreIdleStuff`.
pub const PRE_IDLE: u32 = 0x0070_27e0;
/// `Interface::Idle`.
pub const IDLE: u32 = 0x0070_2810;
/// `Interface::PostIdleStuff`.
pub const POST_IDLE: u32 = 0x0070_2840;
/// `InterfaceManager::PreIdleStuff`.
pub const MANAGER_PRE_IDLE: u32 = 0x0070_b8f0;
/// `InterfaceManager::Idle`.
pub const MANAGER_IDLE: u32 = 0x0070_c4a0;
/// `InterfaceManager::PostIdleStuff` (a lead).
pub const MANAGER_POST_IDLE: u32 = 0x0071_1ea0;
/// `Interface::LastMinuteUpdate`.
pub const LAST_MINUTE_UPDATE: u32 = 0x0070_58e0;
/// The interface manager's part of it (`interfacemanager.cpp`).
pub const MANAGER_LAST_MINUTE_UPDATE: u32 = 0x0071_3c70;
/// `Interface::UpdateSleeping`.
pub const UPDATE_SLEEPING: u32 = 0x0070_56f0;
/// `Main::RenderMenuBackground`.
pub const RENDER_MENU_BACKGROUND: u32 = 0x0087_1dc0;
/// `Main::Swap`.
pub const SWAP: u32 = 0x0086_ff70;
/// `Main::PostSwapProcess`.
pub const POST_SWAP_PROCESS: u32 = 0x0087_05d0;
/// The audio's update (`main.cpp`), `Main::PostSwapProcess`' first call.
pub const AUDIO_UPDATE: u32 = 0x0086_f640;
/// `Main::OnIdle_UpdateProcessLists`.
pub const UPDATE_PROCESS_LISTS: u32 = 0x0086_f890;
/// `Main::UpdateNonRenderSafeAITasks`.
pub const NON_RENDER_SAFE_AI_TASKS: u32 = 0x0086_f6a0;
/// `Main::OnIdle_PostThreadsProcess`.
pub const POST_THREADS_PROCESS: u32 = 0x0087_0610;
/// `Interface::OpenConsole`.
pub const OPEN_CONSOLE: u32 = 0x0070_3e10;
/// `Main::OnIdle_PollControls`.
pub const POLL_CONTROLS: u32 = 0x0086_f390;

/// The AI linear task threads' functions that call the interface idle
/// first (`world::frame::ai_stage`).
pub const COMBINED_THREAD: u32 = 0x008c_7bd0;
/// Thread 1 of the pair.
pub const THREAD_1: u32 = 0x008c_7da0;

/// The sleep/wait menu's class (`Interface::UpdateSleeping` looks it up,
/// `00705770`).
pub const SLEEP_WAIT_MENU: u32 = 1012;

/// The interface manager's modes (`cMenuMode`, +0xc).
pub mod mode {
    /// The game.
    pub const GAME: u32 = 1;
    /// Menus up.
    pub const MENUS: u32 = 2;
    /// Menus coming up.
    pub const OPENING: u32 = 3;
    /// Menus going away.
    pub const CLOSING: u32 = 4;
    /// Menus taking over (one frame).
    pub const TAKING_OVER: u32 = 5;
}

/// What a function tests before a sub-step. [`SubGate::holds`] evaluates
/// it on an [`InterfaceState`], [`SubGate::branches`] names the branches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SubGate {
    Always,
    // The wrappers in `interface.cpp`: the interface manager (`004b7210`)
    // exists and `009373f0` answers true for it.
    /// `Interface::PreIdleStuff` (`007027ea`, `007027fd`).
    PreIdleReady,
    /// `Interface::Idle` (`0070281a`, `0070282d`).
    IdleReady,
    /// `Interface::PostIdleStuff` (`0070284a`, `0070285d`).
    PostIdleReady,
    /// `Interface::LastMinuteUpdate` (`007058ea`, `007058fd`).
    LastMinuteReady,
    // `InterfaceManager::PreIdleStuff` (`0070b8f0`).
    /// Menus coming up and not locked for a fade (mode 3, `0070b952`; +0x11
    /// clear, `0070b961`): the keystrokes cleared, mode 5, the cursor, the
    /// HUD's target and the audio paused.
    MenusTakeOver,
    /// The game (mode 1, `0070ba9b`): the audio's pause follows whether the
    /// top menu pauses the game (`007023a0`).
    PreIdleGame,
    // `InterfaceManager::Idle` (`0070c4a0`).
    /// The mode changed since the last frame (`[0119f530]`, `0070c887`) to
    /// the game (`0070c897`): the audio unpaused, the HUD's mode picked.
    HudModeGame,
    /// Changed to mode 5 (`0070ca18`): the HUD's mode for the top menu.
    HudModeMenus,
    /// The game (mode 1, `0070cbd5`).
    IdleGame,
    /// Menus up or taking over (mode 2 or 5, `0070cc72`, `0070cc7e`): the
    /// pointer, the keys and the pad for the menus.
    IdleMenus,
    /// The Escape control's start menu with one thread: created at once
    /// (`0070e72a`).
    EscapeOneThread,
    /// With more: queued (`Interface::QueueMenuCreate`, opened by
    /// `Interface::HandleQueuedMenuOpen` in `Main::OnIdle_PostThreadsProcess`,
    /// stage 8) (`0070e72a`).
    EscapeThreads,
    /// No menu's own idle stopped the frame (`[011d8a85]`, `0070e820`): the
    /// Pip-Boy's keys, its update and the tutorials.
    IdleGoesOn,
    // `InterfaceManager::PostIdleStuff` (`00711ea0`).
    /// Menus going away (mode 4, `007122c1`) and no fade part-way (+0x11,
    /// `007122d0`) or the cell tests running (`00451530`, `007122e2`): the
    /// keystrokes cleared, the game's mode (1).
    MenusGiveBack,
    // `00713c70`.
    /// Threads > 1 (`00713c86`) and the AI threads running (`00713d90`,
    /// `00713c97`): wait for the interface idle on thread 0 (stage 1).
    WaitForIdleThread,
    /// The in-game loading menu open (`00713cd9`) and the start menu not up
    /// (`00713ce5`): only the loading menu's tiles updated.
    LoadingMenuTiles,
    /// Else every tile.
    AllTiles,
    /// The tiles' update array not empty (`0076b610` on `011d8b44` false,
    /// `00713d22`): its queued work under the lock.
    TileQueueFilled,
    // `Interface::UpdateSleeping` (`007056f0`).
    /// The manager ready (`0070572e`, `00705741`) and the sleep/wait menu
    /// there (`00705747`).
    SleepMenuReady,
    // `Main::RenderMenuBackground` (`00871dc0`).
    /// The player has a parent cell (`008d6f30`, `00871e19`) and its 3D
    /// (slot +0x1d0, `00871e32`).
    PlayerShown,
    // `Main::Swap` (`0086ff70`).
    /// A menu change asks for the background again (`[011d890a]`,
    /// `0086ffa2`).
    MenuChanged,
    /// The display reset (`[011c6fb8]`, `00870071`).
    DisplayReset,
    /// Fader 1 or 2 visible (`00701450`, `0087008a`, `0087009e`).
    FadersUp,
    /// The world drawn: the menus' background not held (`[011dea29]`,
    /// `00870190`), the start menu not up (`008701a0`), the player's 3D
    /// (`008701be`), fader 1 not covering (`00701400`, `008701d6`).
    WorldDrawn,
    /// Else, in a rendered menu (`007079b0`, `00870258`), in menu mode
    /// (`00870264`) or with menu 1013 shown (`0087027a`): the menus drawn.
    MenusDrawn,
    /// Else the world with the interface over it (`008702a9`).
    WorldUnderInterface,
    /// Threads > 1 (`008705a6`): the AI threads may go on past their render
    /// wait.
    SwapThreads,
    // `Main::PostSwapProcess` (`008705d0`).
    /// Threads > 1 (`008705f4`).
    PostSwapThreads,
    // `Main::OnIdle_UpdateProcessLists` (`0086f890`).
    /// One thread (`0086f8a4`).
    ListsOneThread,
    /// The lists run: menu mode clear or fader 1 visible (`0086f8b9`,
    /// `0086f8c4`), the console hidden (`0086f8cf`), the world not frozen
    /// (`0086f8da`).
    ListsRun,
    /// And one thread (`0086f8e9`).
    ListsRunOneThread,
    /// Not, in menu mode or with the world frozen (`0086f915`, `0086f920`).
    ListsHeld,
    // `Main::UpdateNonRenderSafeAITasks` (`0086f6a0`).
    /// In dialogue (`0086f6b3`), the console hidden (`0086f6bf`), and
    /// `0086f860` false (`0086f6cb`).
    DialogueUpdate,
    /// Threads > 1 (`0086f72b`).
    NonRenderSafeThreads,
    /// One thread (`0086f72b`).
    NonRenderSafeOneThread,
    /// Threads > 1 (`0086f791`): the queued attaches and the task queue.
    AttachThreads,
    // `Main::OnIdle_PostThreadsProcess` (`00870610`).
    /// Threads > 1 (`00870629`).
    PostThreadsThreads,
    // `Interface::OpenConsole` (`00703e10`).
    /// The manager ready (`00703e1a`, `00703e2d`), the console there
    /// (`00703e3b`) and hidden (`00703e53`).
    ConsoleOpens,
    // `Main::OnIdle_PollControls` (`0086f390`).
    /// The user actions cleared: not the fly camera (`Main` +6, `0086f3c6`),
    /// the player's slot +0x22c(0) false (`0086f3e5`), V.A.T.S. playing back
    /// with `007d1360` false (`0086f3f4`, `0086f406`) or the Pip-Boy coming
    /// up (`0086f412`), the message menu (1001) not shown (`0086f428`), and
    /// `004a4040` false (`0086f434`).
    ClearUserActions,
}

/// The stages' inputs to the gates. `Default` is a frame in game mode with
/// two threads (the least `main` leaves), the interface manager ready, no
/// menu, nothing requested, the world drawn.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InterfaceState {
    /// `iNumHWThreads:General`.
    pub threads: i32,
    /// The interface manager exists and `009373f0` answers true.
    pub manager_ready: bool,
    /// Its mode (`cMenuMode`, [`mode`]).
    pub mode: u32,
    /// Locked in menu mode for a fade (+0x11).
    pub fade_lock: bool,
    /// The mode isn't the last frame's (`[0119f530]`).
    pub mode_changed: bool,
    /// A menu's own idle stopped the interface's (`[011d8a85]`).
    pub idle_stopped: bool,
    /// A menu's fade is part-way after this frame's fades (+0x11 once
    /// `PostIdleStuff`'s loop has run).
    pub fading: bool,
    /// `00451530`: the cell tests (`TES` +0x51/+0x52).
    pub cell_tests: bool,
    /// The AI threads run (`00713d90`).
    pub threads_running: bool,
    /// The in-game loading menu open and the start menu not up.
    pub loading_menu: bool,
    /// The tiles' update array empty (`0076b610`).
    pub tile_queue_empty: bool,
    /// The sleep/wait menu there.
    pub sleep_menu: bool,
    /// The player has a parent cell and its 3D.
    pub player_shown: bool,
    /// `[011d890a]`.
    pub menu_changed: bool,
    /// `[011c6fb8]`.
    pub display_reset: bool,
    /// Fader 1 or 2 visible.
    pub faders_up: bool,
    /// The menus' background held (`[011dea29]`).
    pub background_held: bool,
    /// The start menu up (`XUserInterface::XUIIsUp`).
    pub start_menu_up: bool,
    /// Fader 1 covering (`00701400`).
    pub fader_1_covering: bool,
    /// In a rendered menu, in menu mode, or menu 1013 shown.
    pub menus_on_screen: bool,
    /// Menu mode, `[011dea2b]`.
    pub menu_flag: bool,
    /// Fader 1 visible, `[011dea2d]`.
    pub fader_visible: bool,
    /// The console shown, `[011dea2e]`.
    pub console_visible: bool,
    /// `Main` +7.
    pub world_frozen: bool,
    /// `Interface::InDialog`.
    pub in_dialog: bool,
    /// `0086f860`.
    pub dialogue_held: bool,
    /// The console there (`MenuConsole::Instance(0)`).
    pub console_exists: bool,
    /// `Main` +6, the fly camera.
    pub fly_camera: bool,
    /// The player's slot +0x22c(0).
    pub player_slot_22c: bool,
    /// The V.A.T.S. manager plays back (mode 4).
    pub vats_playback: bool,
    /// `007d1360` on the player.
    pub vats_test: bool,
    /// `Interface::IsPipboyOpening`.
    pub pipboy_opening: bool,
    /// The message menu (1001) shown.
    pub message_menu: bool,
    /// `004a4040`.
    pub test_004a4040: bool,
}

impl Default for InterfaceState {
    fn default() -> InterfaceState {
        InterfaceState {
            threads: 2,
            manager_ready: true,
            mode: mode::GAME,
            fade_lock: false,
            mode_changed: false,
            idle_stopped: false,
            fading: false,
            cell_tests: false,
            threads_running: false,
            loading_menu: false,
            tile_queue_empty: true,
            sleep_menu: false,
            player_shown: true,
            menu_changed: false,
            display_reset: false,
            faders_up: false,
            background_held: false,
            start_menu_up: false,
            fader_1_covering: false,
            menus_on_screen: false,
            menu_flag: false,
            fader_visible: false,
            console_visible: false,
            world_frozen: false,
            in_dialog: false,
            dialogue_held: false,
            console_exists: true,
            fly_camera: false,
            player_slot_22c: false,
            vats_playback: false,
            vats_test: false,
            pipboy_opening: false,
            message_menu: false,
            test_004a4040: false,
        }
    }
}

impl InterfaceState {
    /// The process lists run (`0086f8b9`-`0086f8da`), as the frame's
    /// [`super::FrameState::process_lists`].
    pub fn lists_run(&self) -> bool {
        (!self.menu_flag || self.fader_visible) && !self.console_visible && !self.world_frozen
    }

    /// `Main::Swap` draws the world.
    pub fn world_drawn(&self) -> bool {
        !self.background_held && !self.start_menu_up && self.player_shown && !self.fader_1_covering
    }
}

impl SubGate {
    /// Whether the sub-step is reached, given that its function is called.
    pub fn holds(self, s: &InterfaceState) -> bool {
        let one = s.threads == 1;
        match self {
            SubGate::Always => true,
            SubGate::PreIdleReady
            | SubGate::IdleReady
            | SubGate::PostIdleReady
            | SubGate::LastMinuteReady => s.manager_ready,
            SubGate::MenusTakeOver => s.mode == mode::OPENING && !s.fade_lock,
            SubGate::PreIdleGame => s.mode == mode::GAME,
            SubGate::HudModeGame => s.mode_changed && s.mode == mode::GAME,
            SubGate::HudModeMenus => s.mode_changed && s.mode == mode::TAKING_OVER,
            SubGate::IdleGame => s.mode == mode::GAME,
            SubGate::IdleMenus => s.mode == mode::MENUS || s.mode == mode::TAKING_OVER,
            SubGate::EscapeOneThread => one,
            SubGate::EscapeThreads => !one,
            SubGate::IdleGoesOn => !s.idle_stopped,
            SubGate::MenusGiveBack => s.mode == mode::CLOSING && (!s.fading || s.cell_tests),
            SubGate::WaitForIdleThread => s.threads > 1 && s.threads_running,
            SubGate::LoadingMenuTiles => s.loading_menu,
            SubGate::AllTiles => !s.loading_menu,
            SubGate::TileQueueFilled => !s.tile_queue_empty,
            SubGate::SleepMenuReady => s.manager_ready && s.sleep_menu,
            SubGate::PlayerShown => s.player_shown,
            SubGate::MenuChanged => s.menu_changed,
            SubGate::DisplayReset => s.display_reset,
            SubGate::FadersUp => s.faders_up,
            SubGate::WorldDrawn => s.world_drawn(),
            SubGate::MenusDrawn => !s.world_drawn() && s.menus_on_screen,
            SubGate::WorldUnderInterface => !s.world_drawn() && !s.menus_on_screen,
            SubGate::SwapThreads | SubGate::PostSwapThreads => s.threads > 1,
            SubGate::ListsOneThread => one,
            SubGate::ListsRun => s.lists_run(),
            SubGate::ListsRunOneThread => s.lists_run() && one,
            SubGate::ListsHeld => !s.lists_run() && (s.menu_flag || s.world_frozen),
            SubGate::DialogueUpdate => s.in_dialog && !s.console_visible && !s.dialogue_held,
            SubGate::NonRenderSafeThreads | SubGate::AttachThreads => s.threads > 1,
            SubGate::NonRenderSafeOneThread => s.threads <= 1,
            SubGate::PostThreadsThreads => s.threads > 1,
            SubGate::ConsoleOpens => s.manager_ready && s.console_exists && !s.console_visible,
            SubGate::ClearUserActions => {
                !s.fly_camera
                    && !s.player_slot_22c
                    && ((s.vats_playback && !s.vats_test) || s.pipboy_opening)
                    && !s.message_menu
                    && !s.test_004a4040
            }
        }
    }

    /// The branch instructions that make up the test.
    pub fn branches(self) -> &'static [u32] {
        match self {
            SubGate::Always => &[],
            SubGate::PreIdleReady => &[0x0070_27ea, 0x0070_27fd],
            SubGate::IdleReady => &[0x0070_281a, 0x0070_282d],
            SubGate::PostIdleReady => &[0x0070_284a, 0x0070_285d],
            SubGate::LastMinuteReady => &[0x0070_58ea, 0x0070_58fd],
            SubGate::MenusTakeOver => &[0x0070_b952, 0x0070_b961],
            SubGate::PreIdleGame => &[0x0070_ba9b],
            SubGate::HudModeGame => &[0x0070_c887, 0x0070_c897],
            SubGate::HudModeMenus => &[0x0070_c887, 0x0070_c897, 0x0070_ca18],
            SubGate::IdleGame => &[0x0070_cbd5],
            SubGate::IdleMenus => &[0x0070_cc72, 0x0070_cc7e],
            SubGate::EscapeOneThread | SubGate::EscapeThreads => &[0x0070_e72a],
            SubGate::IdleGoesOn => &[0x0070_e820],
            SubGate::MenusGiveBack => &[0x0071_22c1, 0x0071_22d0, 0x0071_22e2],
            SubGate::WaitForIdleThread => &[0x0071_3c86, 0x0071_3c97],
            SubGate::LoadingMenuTiles | SubGate::AllTiles => &[0x0071_3cd9, 0x0071_3ce5],
            SubGate::TileQueueFilled => &[0x0071_3d22],
            SubGate::SleepMenuReady => &[0x0070_572e, 0x0070_5741, 0x0070_5747],
            SubGate::PlayerShown => &[0x0087_1e19, 0x0087_1e32],
            SubGate::MenuChanged => &[0x0086_ffa2],
            SubGate::DisplayReset => &[0x0087_0071],
            SubGate::FadersUp => &[0x0087_008a, 0x0087_009e],
            SubGate::WorldDrawn => &[0x0087_0190, 0x0087_01a0, 0x0087_01be, 0x0087_01d6],
            SubGate::MenusDrawn | SubGate::WorldUnderInterface => &[
                0x0087_0190,
                0x0087_01a0,
                0x0087_01be,
                0x0087_01d6,
                0x0087_0258,
                0x0087_0264,
                0x0087_027a,
            ],
            SubGate::SwapThreads => &[0x0087_05a6],
            SubGate::PostSwapThreads => &[0x0087_05f4],
            SubGate::ListsOneThread => &[0x0086_f8a4],
            SubGate::ListsRun => &[0x0086_f8b9, 0x0086_f8c4, 0x0086_f8cf, 0x0086_f8da],
            SubGate::ListsRunOneThread => &[
                0x0086_f8b9,
                0x0086_f8c4,
                0x0086_f8cf,
                0x0086_f8da,
                0x0086_f8e9,
            ],
            SubGate::ListsHeld => &[
                0x0086_f8b9,
                0x0086_f8c4,
                0x0086_f8cf,
                0x0086_f8da,
                0x0086_f915,
                0x0086_f920,
            ],
            SubGate::DialogueUpdate => &[0x0086_f6b3, 0x0086_f6bf, 0x0086_f6cb],
            SubGate::NonRenderSafeThreads | SubGate::NonRenderSafeOneThread => &[0x0086_f72b],
            SubGate::AttachThreads => &[0x0086_f791],
            SubGate::PostThreadsThreads => &[0x0087_0629],
            SubGate::ConsoleOpens => &[0x0070_3e1a, 0x0070_3e2d, 0x0070_3e3b, 0x0070_3e53],
            SubGate::ClearUserActions => &[
                0x0086_f3c6,
                0x0086_f3e5,
                0x0086_f3f4,
                0x0086_f406,
                0x0086_f412,
                0x0086_f428,
                0x0086_f434,
            ],
        }
    }
}

/// One sub-step of a [`Function`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubStep {
    pub callee: Callee,
    /// Its call instructions.
    pub sites: &'static [u32],
    /// The callee's Xbox PDB name (the engine map's, or `frame.tsv`'s).
    pub name: Option<&'static str>,
    pub gate: SubGate,
    /// The branches of the step's own block that decide whether the call
    /// is made once the gate holds; not modelled.
    pub own_tests: &'static [u32],
    pub wiring: Wiring,
}

/// A call of a modelled function: who calls it, from where.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Caller {
    /// `Main::OnIdle` ([`ON_IDLE`]), an AI thread function or a function of
    /// [`FUNCTIONS`].
    pub function: u32,
    /// The call instruction.
    pub site: u32,
}

/// One modelled function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Function {
    pub address: u32,
    /// Its size in bytes (the engine map's).
    pub size: u32,
    pub name: Option<&'static str>,
    /// Its calls in the frame, in the order the frame can make them.
    pub callers: &'static [Caller],
    pub steps: &'static [SubStep],
}

impl Function {
    /// Whether `address` is inside the function.
    pub fn contains(&self, address: u32) -> bool {
        (self.address..self.address + self.size).contains(&address)
    }
}

const fn sub(
    callee: Callee,
    sites: &'static [u32],
    name: Option<&'static str>,
    gate: SubGate,
    own_tests: &'static [u32],
    wiring: Wiring,
) -> SubStep {
    SubStep {
        callee,
        sites,
        name,
        gate,
        own_tests,
        wiring,
    }
}

use Callee::{Direct, Virtual};
use SubGate as G;
use Wiring::{Open, Partial};

const MENUS_WIRING: Wiring = Partial(
    "viewer: menu behavior in frame_order::IdleSet, at the AI idle's position after the \
     controls' poll (world::frame::input); idle caller routing and pre/idle/post ordering \
     remain open",
);

/// `Main::OnIdle_DoInterfaceIdle` (`0086fd70`): the three wrappers.
pub const DO_INTERFACE_IDLE_STEPS: [SubStep; 3] = [
    sub(
        Direct(PRE_IDLE),
        &[0x0086_fd77],
        Some("Interface::PreIdleStuff"),
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(IDLE),
        &[0x0086_fd7c],
        Some("Interface::Idle"),
        G::Always,
        &[],
        MENUS_WIRING,
    ),
    sub(
        Direct(POST_IDLE),
        &[0x0086_fd81],
        Some("Interface::PostIdleStuff"),
        G::Always,
        &[],
        Open,
    ),
];

/// `Interface::PreIdleStuff` (`007027e0`).
pub const PRE_IDLE_STEPS: [SubStep; 1] = [sub(
    Direct(MANAGER_PRE_IDLE),
    &[0x0070_2806],
    Some("InterfaceManager::PreIdleStuff"),
    G::PreIdleReady,
    &[],
    Open,
)];

/// `Interface::Idle` (`00702810`).
pub const IDLE_STEPS: [SubStep; 1] = [sub(
    Direct(MANAGER_IDLE),
    &[0x0070_2836],
    Some("InterfaceManager::Idle"),
    G::IdleReady,
    &[],
    MENUS_WIRING,
)];

/// `Interface::PostIdleStuff` (`00702840`).
pub const POST_IDLE_STEPS: [SubStep; 1] = [sub(
    Direct(MANAGER_POST_IDLE),
    &[0x0070_2866],
    None,
    G::PostIdleReady,
    &[],
    Open,
)];

/// `InterfaceManager::PreIdleStuff` (`0070b8f0`): the fades moved on, the
/// menus taking over (mode 3 → 5, 5 → 2), the audio's pause in the game.
pub const MANAGER_PRE_IDLE_STEPS: [SubStep; 13] = [
    sub(
        Direct(0x0071_6320),
        &[0x0070_b946],
        Some("InterfaceManager::UpdateAllTimers"),
        G::Always,
        &[],
        Partial("viewer: ui::fade::Fades::update in game_menus::run_open_menus"),
    ),
    sub(
        Direct(0x00a2_37b0),
        &[0x0070_b96a],
        Some("Controls::ClearKeystrokes"),
        G::MenusTakeOver,
        &[],
        Open,
    ),
    // The cursor shown without a controller or with the mouse shown.
    sub(
        Direct(0x0056_c7f0),
        &[0x0070_b998],
        None,
        G::MenusTakeOver,
        &[0x0070_b983, 0x0070_b98e],
        Open,
    ),
    sub(
        Direct(0x00a0_1350),
        &[0x0070_b9ce],
        None,
        G::MenusTakeOver,
        &[],
        Open,
    ),
    sub(
        Direct(0x00a0_4640),
        &[0x0070_b9db],
        Some("Tile::UpdateTile"),
        G::MenusTakeOver,
        &[],
        Open,
    ),
    sub(
        Direct(0x0077_5a00),
        &[0x0070_b9e6],
        Some("HUDMainMenu::SetInfoForRef"),
        G::MenusTakeOver,
        &[],
        Open,
    ),
    sub(
        Direct(0x0071_d770),
        &[0x0070_ba32],
        Some("MenuConsole::OnEnterMenuMode"),
        G::MenusTakeOver,
        &[0x0070_ba2d],
        Open,
    ),
    // The sound and voice paused (`0070bba0`: type 0x40000000; `0070bbe0`).
    sub(
        Direct(0x0070_bba0),
        &[0x0070_ba56],
        None,
        G::MenusTakeOver,
        &[],
        Open,
    ),
    sub(
        Direct(0x0070_bbe0),
        &[0x0070_ba7a],
        None,
        G::MenusTakeOver,
        &[],
        Open,
    ),
    // In the game, a pausing top menu pauses them once; else they are
    // unpaused once.
    sub(
        Direct(0x0070_bba0),
        &[0x0070_bae0],
        None,
        G::PreIdleGame,
        &[0x0070_baae, 0x0070_bab9],
        Open,
    ),
    sub(
        Direct(0x0070_bbe0),
        &[0x0070_bb03],
        None,
        G::PreIdleGame,
        &[0x0070_baae, 0x0070_bab9],
        Open,
    ),
    sub(
        Direct(0x0070_bc00),
        &[0x0070_bb48],
        None,
        G::PreIdleGame,
        &[0x0070_bb13, 0x0070_bb27],
        Open,
    ),
    sub(
        Direct(0x0070_bbc0),
        &[0x0070_bb79],
        Some("BSAudio::UnPauseAllOfType"),
        G::PreIdleGame,
        &[0x0070_bb13, 0x0070_bb58],
        Open,
    ),
];

/// `InterfaceManager::Idle` (`0070c4a0`): the controller, the cursor, the
/// rendered menu, the effects, the HUD's mode, the game's part (the HUD,
/// level up, eating and enchantments), the menus' part (the pointer, the
/// keys, the pad, the console's own idle), the console's key, the Escape
/// control, each menu's own idle, the Pip-Boy's keys and update, the
/// tutorials.
pub const MANAGER_IDLE_STEPS: [SubStep; 39] = [
    sub(
        Direct(0x009f_996e),
        &[0x0070_c525],
        Some("XInputGetState"),
        G::Always,
        &[],
        Open,
    ),
    // A message when the controller comes or goes.
    sub(
        Direct(0x0070_3e80),
        &[0x0070_c5aa, 0x0070_c5e2],
        None,
        G::Always,
        &[0x0070_c56a, 0x0070_c57a],
        Open,
    ),
    sub(
        Direct(0x0071_7660),
        &[0x0070_c620],
        Some("InterfaceManager::PreLoadMainMenus"),
        G::Always,
        &[0x0070_c618],
        Open,
    ),
    // The cursor for the device: shown or hidden.
    sub(
        Direct(0x0056_c7f0),
        &[0x0070_c68f, 0x0070_c6c7, 0x0070_c70b],
        None,
        G::Always,
        &[0x0070_c66c, 0x0070_c678, 0x0070_c6f4],
        Open,
    ),
    // The rendered menu's per-frame call (`pCurrentRenderedMenu`, slot
    // +0x14).
    sub(
        Virtual(0x14),
        &[0x0070_c806],
        None,
        G::Always,
        &[0x0070_c7e5],
        Open,
    ),
    sub(
        Direct(0x007f_a540),
        &[0x0070_c852],
        Some("FOPipboyManager::UpdateLightEffect"),
        G::Always,
        &[0x0070_c81f],
        Open,
    ),
    // The HUD's and V.A.T.S.'s effect managers (`+0x178`, `+0x1dc`).
    sub(
        Direct(0x007f_7830),
        &[0x0070_c863],
        None,
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(0x0080_0370),
        &[0x0070_c874],
        None,
        G::Always,
        &[],
        Open,
    ),
    // Back in the game: the voice and sound unpaused while paused.
    sub(
        Direct(0x0070_bc00),
        &[0x0070_c8d2],
        None,
        G::HudModeGame,
        &[0x0070_c8a8],
        Open,
    ),
    sub(
        Direct(0x0070_bbc0),
        &[0x0070_c90e],
        Some("BSAudio::UnPauseAllOfType"),
        G::HudModeGame,
        &[0x0070_c8e4],
        Open,
    ),
    sub(
        Direct(0x0077_1700),
        &[
            0x0070_c936,
            0x0070_c95f,
            0x0070_c990,
            0x0070_c9ae,
            0x0070_c9da,
            0x0070_c9f5,
            0x0070_ca01,
        ],
        Some("HUDMainMenu::SetMenuMode"),
        G::HudModeGame,
        &[0x0070_c932, 0x0070_c95b, 0x0070_c974, 0x0070_c98c],
        Open,
    ),
    sub(
        Direct(0x0077_1700),
        &[
            0x0070_ca97,
            0x0070_caa6,
            0x0070_cab5,
            0x0070_cac1,
            0x0070_cacd,
            0x0070_cad9,
            0x0070_cae5,
            0x0070_caf1,
            0x0070_cafd,
            0x0070_cb09,
            0x0070_cb15,
            0x0070_cb21,
            0x0070_cb2d,
        ],
        Some("HUDMainMenu::SetMenuMode"),
        G::HudModeMenus,
        &[0x0070_ca39, 0x0070_ca7b],
        Open,
    ),
    sub(
        Direct(0x0071_8930),
        &[0x0070_cbc6],
        None,
        G::Always,
        &[0x0070_cb54, 0x0070_cb6e, 0x0070_cb88, 0x0070_cba2, 0x0070_cbbc],
        Open,
    ),
    sub(
        Direct(0x0076_bfe0),
        &[0x0070_cbf1],
        Some("HUDMainMenu::Create"),
        G::IdleGame,
        &[0x0070_cbef],
        Open,
    ),
    sub(
        Direct(0x0078_4c80),
        &[0x0070_cc46],
        Some("LevelUpMenu::Create"),
        G::IdleGame,
        &[0x0070_cc44],
        Open,
    ),
    sub(
        Direct(0x0095_d090),
        &[0x0070_cc58],
        Some("PlayerCharacter::CastEatDrinkItems"),
        G::IdleGame,
        &[],
        Open,
    ),
    sub(
        Direct(0x0095_ec40),
        &[0x0070_cc63],
        Some("PlayerCharacter::CastQueuedEnchantments"),
        G::IdleGame,
        &[],
        Open,
    ),
    sub(
        Direct(0x0071_7ef0),
        &[0x0070_cf97, 0x0070_d516],
        Some("InterfaceManager::DoLeave"),
        G::IdleMenus,
        &[0x0070_cf3c, 0x0070_d48c],
        Open,
    ),
    sub(
        Direct(0x0071_7e70),
        &[0x0070_d637],
        Some("InterfaceManager::DoEnter"),
        G::IdleMenus,
        &[0x0070_d5b3],
        Open,
    ),
    sub(
        Direct(0x0071_0ad0),
        &[0x0070_dc65, 0x0070_df94],
        Some("InterfaceManager::DisplayCurrentPickRef"),
        G::IdleMenus,
        &[0x0070_dc52, 0x0070_df7f],
        Open,
    ),
    sub(
        Direct(0x0071_8080),
        &[0x0070_deef],
        Some("InterfaceManager::DoWheelMove"),
        G::IdleMenus,
        &[0x0070_de93],
        Open,
    ),
    sub(
        Direct(0x0070_f6e0),
        &[0x0070_dfbb, 0x0070_e52c],
        Some("InterfaceManager::DoGamepadEvent"),
        G::IdleMenus,
        &[0x0070_dfb1, 0x0070_e51d],
        Open,
    ),
    sub(
        Direct(0x0071_b210),
        &[0x0070_e09e],
        Some("MenuConsole::Idle"),
        G::IdleMenus,
        &[0x0070_e089],
        Open,
    ),
    // The keys go to the frontmost menu (`MenuManager::GetFrontmostMenu`),
    // its slot +0x30 first, then a click on the tile under the pointer
    // (slot +0xc).
    sub(
        Direct(0x0072_0e60),
        &[0x0070_e1cb],
        Some("MenuManager::GetFrontmostMenu"),
        G::IdleMenus,
        &[0x0070_e1a3, 0x0070_e1b2],
        Open,
    ),
    sub(
        Virtual(0x30),
        &[0x0070_e27e],
        None,
        G::IdleMenus,
        &[0x0070_e259, 0x0070_e264],
        Open,
    ),
    sub(
        Virtual(0x0c),
        &[0x0070_e3ce],
        None,
        G::IdleMenus,
        &[0x0070_e36a],
        Open,
    ),
    sub(
        Direct(0x0071_7280),
        &[0x0070_e3d4],
        Some("InterfaceManager::PlayMenuSound"),
        G::IdleMenus,
        &[0x0070_e36a],
        Open,
    ),
    // The console's key (control 0x1d) or a request (`0070ed10`,
    // `[011dea2f]`, which it hands on to the frame's `OpenConsole`).
    sub(
        Direct(0x0070_ed20),
        &[0x0070_e5d3],
        None,
        G::Always,
        &[0x0070_e5cf],
        Open,
    ),
    sub(
        Direct(0x0071_d580),
        &[0x0070_e627],
        Some("MenuConsole::ToggleVisible"),
        G::Always,
        &[0x0070_e605, 0x0070_e60d],
        Open,
    ),
    // The Escape control (0x1c): the start menu closed, or opened.
    sub(
        Direct(0x007c_e7a0),
        &[0x0070_e708],
        Some("StartMenu::Close"),
        G::Always,
        &[0x0070_e65f, 0x0070_e680, 0x0070_e690, 0x0070_e6c2, 0x0070_e6f8, 0x0070_e704],
        Open,
    ),
    sub(
        Direct(0x007c_b7d0),
        &[0x0070_e730],
        Some("StartMenu::Create"),
        G::EscapeOneThread,
        &[0x0070_e65f, 0x0070_e680, 0x0070_e690, 0x0070_e6a3, 0x0070_e6b2, 0x0070_e71b],
        Partial("viewer: game_menus::escape_opens_start_menu (opens it at once)"),
    ),
    sub(
        Direct(0x0070_9470),
        &[0x0070_e752],
        Some("Interface::QueueMenuCreate"),
        G::EscapeThreads,
        &[
            0x0070_e65f,
            0x0070_e680,
            0x0070_e690,
            0x0070_e6a3,
            0x0070_e6b2,
            0x0070_e71b,
            0x0070_e744,
        ],
        Partial("viewer: game_menus::escape_opens_start_menu (opens it at once, not queued)"),
    ),
    // Each root menu's own idle (slot +0x2c), until one stops the frame.
    sub(
        Virtual(0x2c),
        &[0x0070_e7fb],
        None,
        G::Always,
        &[0x0070_e797, 0x0070_e7a2, 0x0070_e7ce, 0x0070_e7e8],
        Partial(
            "viewer: hud::update_hud and local_map::update_local_map (the HUD's and the map's \
             updates), at this sub-step",
        ),
    ),
    // The Pip-Boy: opened, closed (with the user actions cleared), or its
    // page changed by a function key.
    sub(
        Direct(0x0070_f4e0),
        &[0x0070_e913, 0x0070_ea34],
        None,
        G::IdleGoesOn,
        &[0x0070_e907, 0x0070_ea26],
        Partial("viewer: pipboy::pipboy_keys (with the menus, in the viewer's order)"),
    ),
    sub(
        Direct(0x0070_f690),
        &[0x0070_e99a, 0x0070_eade],
        None,
        G::IdleGoesOn,
        &[0x0070_e98e, 0x0070_eaa6],
        Open,
    ),
    sub(
        Direct(0x00a2_53d0),
        &[0x0070_e9ac],
        Some("Controls::ClearUserActions"),
        G::IdleGoesOn,
        &[0x0070_e98e],
        Open,
    ),
    sub(
        Direct(0x0070_48f0),
        &[0x0070_eab0],
        Some("Interface::SetInventoryMenuVisible"),
        G::IdleGoesOn,
        &[0x0070_ea8e],
        Open,
    ),
    sub(
        Direct(0x0070_ee80),
        &[0x0070_eb04],
        Some("InterfaceManager::UpdatePipboy"),
        G::IdleGoesOn,
        &[0x0070_eaf0, 0x0070_eafc],
        Partial("viewer: pipboy::update_pipboy and pipboy_light at the approximate AI idle position; sub-step and fallback routing open"),
    ),
    sub(
        Direct(0x0071_82e0),
        &[0x0070_eb15],
        Some("InterfaceManager::TutorialManager::Update"),
        G::IdleGoesOn,
        &[],
        Partial("viewer: game_menus::run_tutorials (with the menus, in the viewer's order)"),
    ),
];

/// `InterfaceManager::PostIdleStuff` (`00711ea0`, a lead): each root menu's
/// fade laid on (shown, faded, hidden or taken away, the image space
/// effect with the first and last menu), then the game given back.
pub const MANAGER_POST_IDLE_STEPS: [SubStep; 8] = [
    sub(
        Direct(0x0070_37c0),
        &[0x0071_204d, 0x0071_2229],
        Some("BGSMenuPacker::RecomputePacking"),
        G::Always,
        &[0x0071_1f13, 0x0071_1f7a],
        Open,
    ),
    // A faded-out menu that left the stack is deleted (slot 0 with 1).
    sub(
        Virtual(0x00),
        &[0x0071_209c],
        None,
        G::Always,
        &[0x0071_1f13, 0x0071_1f7a],
        Open,
    ),
    sub(
        Direct(0x0071_2450),
        &[0x0071_20ef, 0x0071_2165, 0x0071_221f, 0x0071_2295],
        Some("InterfaceManager::RecursiveFade"),
        G::Always,
        &[0x0071_1f13, 0x0071_1f7a],
        Partial("viewer: game_menus' Screen::end_fades (ui::fade::Fades::frame) in run_open_menus"),
    ),
    // The image space effect 15 faded with the first and last menu.
    sub(
        Direct(0x0071_23f0),
        &[0x0071_2109, 0x0071_218a, 0x0071_2243, 0x0071_22b0],
        None,
        G::Always,
        &[0x0071_1f13, 0x0071_1f7a],
        Open,
    ),
    sub(
        Direct(0x0071_23a0),
        &[0x0071_2117, 0x0071_2251],
        None,
        G::Always,
        &[0x0071_1f13, 0x0071_1f7a],
        Open,
    ),
    sub(
        Direct(0x00a2_37b0),
        &[0x0071_22e7],
        Some("Controls::ClearKeystrokes"),
        G::MenusGiveBack,
        &[],
        Open,
    ),
    sub(
        Direct(0x0056_c7f0),
        &[0x0071_2315],
        None,
        G::MenusGiveBack,
        &[0x0071_2300, 0x0071_230b],
        Open,
    ),
    sub(
        Direct(0x00a0_4640),
        &[0x0071_233b],
        Some("Tile::UpdateTile"),
        G::MenusGiveBack,
        &[],
        Open,
    ),
];

/// `Interface::LastMinuteUpdate` (`007058e0`).
pub const LAST_MINUTE_STEPS: [SubStep; 1] = [sub(
    Direct(MANAGER_LAST_MINUTE_UPDATE),
    &[0x0070_5906],
    None,
    G::LastMinuteReady,
    &[],
    Open,
)];

/// The interface manager's last-minute update (`00713c70`): after the AI
/// thread's interface idle, the tiles brought up to date (only the loading
/// menu's while it is up), their fades, then (the tile queue free) the
/// queued work under the lock.
pub const MANAGER_LAST_MINUTE_STEPS: [SubStep; 10] = [
    sub(
        Direct(0x008c_7a70),
        &[0x0071_3ca4],
        None,
        G::WaitForIdleThread,
        &[],
        Open,
    ),
    sub(
        Direct(0x0071_3c00),
        &[0x0071_3ca9],
        None,
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(0x00a0_4510),
        &[0x0071_3cc0],
        None,
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(0x00a0_4620),
        &[0x0071_3cf8],
        Some("Tile::UpdateChildren"),
        G::LoadingMenuTiles,
        &[],
        Open,
    ),
    sub(
        Direct(0x00a0_4200),
        &[0x0071_3d01],
        Some("Tile::UpdateAll"),
        G::AllTiles,
        &[],
        Open,
    ),
    sub(
        Direct(0x00a0_80d0),
        &[0x0071_3d09],
        Some("Tile::UpdateFadeControls"),
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(0x0071_3d60),
        &[0x0071_3d0e],
        None,
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(0x0045_38a0),
        &[0x0071_3d2d],
        None,
        G::TileQueueFilled,
        &[],
        Open,
    ),
    sub(
        Direct(0x0071_3e20),
        &[0x0071_3d35],
        None,
        G::TileQueueFilled,
        &[],
        Open,
    ),
    sub(
        Direct(0x0045_38c0),
        &[0x0071_3d41],
        None,
        G::TileQueueFilled,
        &[],
        Open,
    ),
];

/// `Interface::UpdateSleeping` (`007056f0`): the sleep/wait menu found, then
/// its `UpdateSleeping` (the Rest control's cancel, an hour each second).
pub const UPDATE_SLEEPING_STEPS: [SubStep; 3] = [
    sub(
        Direct(0x00a0_9030),
        &[0x0070_56fc],
        Some("Tile::GetMenuByClass"),
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(0x00a0_3c90),
        &[0x0070_5710],
        Some("Tile::GetMenu"),
        G::Always,
        &[0x0070_570b],
        Open,
    ),
    sub(
        Direct(0x007c_0580),
        &[0x0070_574c],
        Some("SleepWaitMenu::UpdateSleeping"),
        G::SleepMenuReady,
        &[],
        Partial("viewer: game_menus::update_sleeping (sleepwait::update_sleeping)"),
    ),
];

/// `Main::RenderMenuBackground` (`00871dc0`): the world drawn once under the
/// menus' modifier (and the rendered menu's scene with menu 1036), the
/// modifier taken away again.
pub const RENDER_MENU_BACKGROUND_STEPS: [SubStep; 8] = [
    sub(
        Direct(0x0071_8ab0),
        &[0x0087_212d],
        Some("InterfaceManager::GetCurrentImagespaceMod"),
        G::PlayerShown,
        &[],
        Partial("world::menu_background::modifier"),
    ),
    sub(
        Direct(0x0052_9c90),
        &[0x0087_213b],
        Some("ImageSpaceModifierInstanceForm::Stop"),
        G::PlayerShown,
        &[],
        Open,
    ),
    sub(
        Direct(0x0052_99a0),
        &[0x0087_2155],
        Some("ImageSpaceModifierInstanceForm::Trigger"),
        G::PlayerShown,
        &[0x0087_2147],
        Open,
    ),
    sub(
        Direct(0x0086_fd90),
        &[0x0087_2165],
        Some("Main::OnIdle_UpdateImageSpace"),
        G::PlayerShown,
        &[0x0087_2147],
        Open,
    ),
    // The world drawn into the background (`main.cpp`).
    sub(
        Direct(0x0087_06b0),
        &[0x0087_21a4],
        None,
        G::PlayerShown,
        &[],
        Open,
    ),
    sub(
        Direct(0x00b6_bee0),
        &[0x0087_225a],
        Some("BSShaderUtil::AccumulateScene"),
        G::PlayerShown,
        &[0x0087_21bd, 0x0087_21cd],
        Open,
    ),
    sub(
        Direct(0x0052_9c90),
        &[0x0087_22a8],
        Some("ImageSpaceModifierInstanceForm::Stop"),
        G::PlayerShown,
        &[0x0087_22a2],
        Open,
    ),
    sub(
        Direct(0x0086_fd90),
        &[0x0087_22b8],
        Some("Main::OnIdle_UpdateImageSpace"),
        G::PlayerShown,
        &[0x0087_22a2],
        Open,
    ),
];

// Bevy renders after Update; no viewer system executes these Swap calls.
const RENDER_WIRING: Wiring = Open;

/// `Main::Swap` (`0086ff70`): the menus' background again after a menu
/// change, a display reset, the faders' loading screen and their end, then
/// the world or the menus drawn, the click, the AI threads let past their
/// render wait.
pub const SWAP_STEPS: [SubStep; 15] = [
    sub(
        Direct(RENDER_MENU_BACKGROUND),
        &[0x0087_000f],
        Some("Main::RenderMenuBackground"),
        G::MenuChanged,
        &[],
        Open,
    ),
    sub(
        Direct(0x004d_cef0),
        &[0x0087_0073],
        None,
        G::DisplayReset,
        &[],
        Open,
    ),
    sub(
        Direct(0x0045_7d70),
        &[0x0087_00b9],
        Some("TES::ShowLoadingMenu"),
        G::FadersUp,
        &[0x0087_00ab],
        Open,
    ),
    sub(
        Direct(0x0096_cfa0),
        &[0x0087_00c8],
        None,
        G::FadersUp,
        &[0x0087_00ab],
        Open,
    ),
    sub(
        Direct(0x0070_10e0),
        &[0x0087_0116, 0x0087_0125],
        Some("FaderManager::RemoveFader"),
        G::FadersUp,
        &[0x0087_0101, 0x0087_010a],
        Open,
    ),
    sub(
        Direct(0x0087_7430),
        &[0x0087_0167],
        Some("Main::KillMenuBGTexture"),
        G::FadersUp,
        &[0x0087_0101, 0x0087_010a],
        Open,
    ),
    // The world drawn (`main.cpp`).
    sub(
        Direct(0x0087_06b0),
        &[0x0087_0244],
        None,
        G::WorldDrawn,
        &[],
        RENDER_WIRING,
    ),
    sub(
        Direct(0x0087_06b0),
        &[0x0087_02a9],
        None,
        G::WorldUnderInterface,
        &[0x0087_0294],
        RENDER_WIRING,
    ),
    sub(
        Direct(0x0087_1a50),
        &[0x0087_02f7],
        Some("Main::UpdateOffscreenInterface"),
        G::MenusDrawn,
        &[],
        Partial(
            "viewer: the menus' 3D scenes and the HUD's picture over them \
             (compose_hud_over_scene, vigor::draw, lockpick::show_lockpicking, \
             caravan_table::show_table, casino_scene::show_casino) remain in the early \
             viewer interface set; this sub-step's order and gates are not wired",
        ),
    ),
    // The menus over the background (`main.cpp`), or a rendered menu.
    sub(
        Direct(0x0087_2940),
        &[0x0087_03bc],
        None,
        G::MenusDrawn,
        &[0x0087_03b2],
        RENDER_WIRING,
    ),
    sub(
        Direct(0x0087_4b90),
        &[0x0087_03ec],
        None,
        G::MenusDrawn,
        &[0x0087_03cf],
        RENDER_WIRING,
    ),
    sub(
        Direct(0x0070_9b40),
        &[0x0087_0403],
        None,
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(0x0070_28b0),
        &[0x0087_0417],
        Some("Interface::Click"),
        G::Always,
        &[],
        Open,
    ),
    // The frame presented (`bgsdecalmanager.cpp`'s renderer call, as at the
    // start), Bevy's.
    sub(
        Direct(0x004a_03c0),
        &[0x0087_0594],
        None,
        G::Always,
        &[],
        RENDER_WIRING,
    ),
    sub(
        Direct(0x008c_80e0),
        &[0x0087_05aa],
        Some("AILinearTaskThreadManager::SetMainRendering"),
        G::SwapThreads,
        &[],
        Open,
    ),
];

/// `Main::PostSwapProcess` (`008705d0`).
pub const POST_SWAP_STEPS: [SubStep; 4] = [
    sub(
        Direct(AUDIO_UPDATE),
        &[0x0087_05da],
        None,
        G::Always,
        &[],
        Partial("viewer: sounds, the radio and the music, at its sub-steps"),
    ),
    sub(
        Direct(UPDATE_PROCESS_LISTS),
        &[0x0087_05e2],
        Some("Main::OnIdle_UpdateProcessLists"),
        G::Always,
        &[],
        Partial("viewer: ai::move_offstage, at its sub-step"),
    ),
    sub(
        Direct(0x0055_2570),
        &[0x0087_05f6],
        None,
        G::PostSwapThreads,
        &[],
        Open,
    ),
    sub(
        Direct(0x0086_f670),
        &[0x0087_05fe],
        None,
        G::Always,
        &[],
        Open,
    ),
];

/// The audio's update (`0086f640`): `BSAudio`'s (`00ad7740`, which calls
/// `BSAudioManager::Update`), the radio's, the music's
/// (`falloutaudiomedia.cpp`'s `0082fb70`: the media types, combat music,
/// the music's fades), the regions' sounds.
pub const AUDIO_UPDATE_STEPS: [SubStep; 4] = [
    sub(
        Direct(0x00ad_7740),
        &[0x0086_f650],
        None,
        G::Always,
        &[],
        Partial("viewer: sounds::play_sounds"),
    ),
    sub(
        Direct(0x0083_2ad0),
        &[0x0086_f657],
        Some("FalloutRadio::Update"),
        G::Always,
        &[],
        Partial("viewer: radio::run_radio"),
    ),
    sub(
        Direct(0x0082_fb70),
        &[0x0086_f65f],
        None,
        G::Always,
        &[],
        Partial("viewer: music::play_music"),
    ),
    sub(
        Direct(0x0082_d7c0),
        &[0x0086_f664],
        Some("FalloutAudio::UpdateRegionSounds"),
        G::Always,
        &[],
        Open,
    ),
];

/// `Main::OnIdle_UpdateProcessLists` (`0086f890`): with one thread the
/// process-level list first; then, when the lists run, the combatants (one
/// thread) and `ProcessLists::UpdateProcessLists` (the lower process
/// levels' moves); else in menu mode or frozen, the player's `00964260`.
pub const UPDATE_PROCESS_LISTS_STEPS: [SubStep; 5] = [
    sub(
        Direct(0x0096_eb40),
        &[0x0086_f8ab],
        Some("ProcessLists::ChangeProcessLevelTempList"),
        G::ListsOneThread,
        &[],
        Open,
    ),
    sub(
        Direct(0x008c_94e0),
        &[0x0086_f8f0],
        None,
        G::ListsRunOneThread,
        &[],
        Open,
    ),
    sub(
        Direct(0x0099_1dc0),
        &[0x0086_f8fb],
        Some("CombatManager::UpdateCombatants"),
        G::ListsRunOneThread,
        &[],
        Open,
    ),
    sub(
        Direct(0x0096_d810),
        &[0x0086_f905],
        Some("ProcessLists::UpdateProcessLists"),
        G::ListsRun,
        &[],
        Partial("viewer: ai::move_offstage (the lower process levels' moves)"),
    ),
    sub(
        Direct(0x0096_4260),
        &[0x0086_f928],
        None,
        G::ListsHeld,
        &[],
        Open,
    ),
];

/// `Main::UpdateNonRenderSafeAITasks` (`0086f6a0`).
pub const NON_RENDER_SAFE_STEPS: [SubStep; 15] = [
    // In dialogue: the dialogue menu's (`007624d0`) and its two slots.
    sub(
        Direct(0x0076_24d0),
        &[0x0086_f6cd],
        None,
        G::DialogueUpdate,
        &[],
        Open,
    ),
    sub(
        Direct(0x0096_c240),
        &[0x0086_f719],
        None,
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(0x0096_c970),
        &[0x0086_f732],
        Some("ProcessLists::ParallelUpdateAnimationScenegraph"),
        G::NonRenderSafeThreads,
        &[],
        Open,
    ),
    sub(
        Direct(0x0096_c860),
        &[0x0086_f73e],
        Some("ProcessLists::UpdateAnimationScenegraph"),
        G::NonRenderSafeOneThread,
        &[],
        Open,
    ),
    sub(
        Direct(0x0096_c710),
        &[0x0086_f748],
        None,
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(0x0097_81d0),
        &[0x0086_f760],
        None,
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(0x007f_a990),
        &[0x0086_f77f],
        None,
        G::Always,
        &[0x0086_f77a],
        Open,
    ),
    sub(
        Direct(0x0054_ae30),
        &[0x0086_f793],
        Some("TESObjectCELL::PerformQueuedChildAttaches"),
        G::AttachThreads,
        &[],
        Open,
    ),
    sub(
        Direct(0x0087_a6d0),
        &[0x0086_f79f],
        None,
        G::AttachThreads,
        &[],
        Open,
    ),
    sub(
        Direct(0x0087_a790),
        &[0x0086_f7ab],
        None,
        G::AttachThreads,
        &[],
        Open,
    ),
    sub(
        Direct(0x0055_2570),
        &[0x0086_f7b0],
        None,
        G::AttachThreads,
        &[],
        Open,
    ),
    sub(
        Direct(0x0087_a6b0),
        &[0x0086_f7ce],
        None,
        G::AttachThreads,
        &[],
        Open,
    ),
    sub(
        Direct(0x0070_34c0),
        &[0x0086_f7e0],
        None,
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(0x0070_3490),
        &[0x0086_f7e8],
        None,
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(0x0099_1600),
        &[0x0086_f823],
        None,
        G::Always,
        &[0x0086_f7f6, 0x0086_f801, 0x0086_f80c],
        Open,
    ),
];

/// `Main::OnIdle_PostThreadsProcess` (`00870610`): scripts' delayed actions,
/// the task queue (threads), the queued menus opened, the debug text.
pub const POST_THREADS_STEPS: [SubStep; 7] = [
    sub(
        Direct(0x005a_a720),
        &[0x0087_0617],
        Some("Script::RunDelayedScriptActionsOnReferences"),
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(0x0087_a710),
        &[0x0087_0632],
        None,
        G::PostThreadsThreads,
        &[],
        Open,
    ),
    sub(
        Direct(0x0087_a850),
        &[0x0087_063e],
        None,
        G::PostThreadsThreads,
        &[],
        Open,
    ),
    sub(
        Direct(0x0087_a6f0),
        &[0x0087_064a],
        None,
        G::PostThreadsThreads,
        &[],
        Open,
    ),
    sub(
        Direct(0x0070_94f0),
        &[0x0087_064f],
        Some("Interface::HandleQueuedMenuOpen"),
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(0x0087_0680),
        &[0x0087_065b],
        None,
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(0x00a0_e770),
        &[0x0087_066c],
        Some("DebugText::HandleQueuedPrints"),
        G::Always,
        &[],
        Open,
    ),
];

/// `Interface::OpenConsole` (`00703e10`).
pub const OPEN_CONSOLE_STEPS: [SubStep; 2] = [
    sub(
        Direct(0x0071_d580),
        &[0x0070_3e61],
        Some("MenuConsole::ToggleVisible"),
        G::ConsoleOpens,
        &[],
        Open,
    ),
    sub(
        Direct(0x0071_4d90),
        &[0x0070_3e6f],
        Some("InterfaceManager::AddToEnterStack"),
        G::ConsoleOpens,
        &[],
        Open,
    ),
];

/// `Main::OnIdle_PollControls` (`0086f390`).
pub const POLL_CONTROLS_STEPS: [SubStep; 3] = [
    sub(
        Direct(0x00a2_3010),
        &[0x0086_f39e],
        Some("Controls::Poll"),
        G::Always,
        &[],
        Partial("viewer: poll::poll_controls (world::frame::input)"),
    ),
    // The frame time handed to the controls.
    sub(
        Direct(0x00a2_57c0),
        &[0x0086_f3b8],
        None,
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(0x00a2_53d0),
        &[0x0086_f43d],
        Some("Controls::ClearUserActions"),
        G::ClearUserActions,
        &[],
        Open,
    ),
];

const fn caller(function: u32, site: u32) -> Caller {
    Caller { function, site }
}

/// The modelled functions.
pub const FUNCTIONS: [Function; 19] = [
    Function {
        address: DO_INTERFACE_IDLE,
        size: 26,
        name: Some("Main::OnIdle_DoInterfaceIdle"),
        callers: &[
            caller(ON_IDLE, 0x0086_eb36),
            caller(COMBINED_THREAD, 0x008c_7be9),
            caller(THREAD_1, 0x008c_7db9),
            caller(ON_IDLE, 0x0086_ecba),
        ],
        steps: &DO_INTERFACE_IDLE_STEPS,
    },
    Function {
        address: PRE_IDLE,
        size: 45,
        name: Some("Interface::PreIdleStuff"),
        callers: &[caller(DO_INTERFACE_IDLE, 0x0086_fd77)],
        steps: &PRE_IDLE_STEPS,
    },
    Function {
        address: MANAGER_PRE_IDLE,
        size: 684,
        name: Some("InterfaceManager::PreIdleStuff"),
        callers: &[caller(PRE_IDLE, 0x0070_2806)],
        steps: &MANAGER_PRE_IDLE_STEPS,
    },
    Function {
        address: IDLE,
        size: 45,
        name: Some("Interface::Idle"),
        callers: &[caller(DO_INTERFACE_IDLE, 0x0086_fd7c)],
        steps: &IDLE_STEPS,
    },
    Function {
        address: MANAGER_IDLE,
        size: 9894,
        name: Some("InterfaceManager::Idle"),
        callers: &[caller(IDLE, 0x0070_2836)],
        steps: &MANAGER_IDLE_STEPS,
    },
    Function {
        address: POST_IDLE,
        size: 45,
        name: Some("Interface::PostIdleStuff"),
        callers: &[caller(DO_INTERFACE_IDLE, 0x0086_fd81)],
        steps: &POST_IDLE_STEPS,
    },
    Function {
        address: MANAGER_POST_IDLE,
        size: 1237,
        name: None,
        callers: &[caller(POST_IDLE, 0x0070_2866)],
        steps: &MANAGER_POST_IDLE_STEPS,
    },
    Function {
        address: LAST_MINUTE_UPDATE,
        size: 45,
        name: Some("Interface::LastMinuteUpdate"),
        callers: &[caller(ON_IDLE, 0x0086_eccb)],
        steps: &LAST_MINUTE_STEPS,
    },
    Function {
        address: MANAGER_LAST_MINUTE_UPDATE,
        size: 225,
        name: None,
        callers: &[caller(LAST_MINUTE_UPDATE, 0x0070_5906)],
        steps: &MANAGER_LAST_MINUTE_STEPS,
    },
    Function {
        address: UPDATE_SLEEPING,
        size: 127,
        name: Some("Interface::UpdateSleeping"),
        callers: &[caller(ON_IDLE, 0x0086_ed77)],
        steps: &UPDATE_SLEEPING_STEPS,
    },
    Function {
        address: RENDER_MENU_BACKGROUND,
        size: 1446,
        name: Some("Main::RenderMenuBackground"),
        callers: &[caller(ON_IDLE, 0x0086_ed86), caller(SWAP, 0x0087_000f)],
        steps: &RENDER_MENU_BACKGROUND_STEPS,
    },
    Function {
        address: SWAP,
        size: 1616,
        name: Some("Main::Swap"),
        callers: &[caller(ON_IDLE, 0x0086_ede8)],
        steps: &SWAP_STEPS,
    },
    Function {
        address: POST_SWAP_PROCESS,
        size: 55,
        name: Some("Main::PostSwapProcess"),
        callers: &[caller(ON_IDLE, 0x0086_edf0)],
        steps: &POST_SWAP_STEPS,
    },
    Function {
        address: AUDIO_UPDATE,
        size: 45,
        name: None,
        callers: &[caller(POST_SWAP_PROCESS, 0x0087_05da)],
        steps: &AUDIO_UPDATE_STEPS,
    },
    Function {
        address: UPDATE_PROCESS_LISTS,
        size: 161,
        name: Some("Main::OnIdle_UpdateProcessLists"),
        callers: &[caller(POST_SWAP_PROCESS, 0x0087_05e2)],
        steps: &UPDATE_PROCESS_LISTS_STEPS,
    },
    Function {
        address: NON_RENDER_SAFE_AI_TASKS,
        size: 396,
        name: Some("Main::UpdateNonRenderSafeAITasks"),
        callers: &[caller(ON_IDLE, 0x0086_ee62)],
        steps: &NON_RENDER_SAFE_STEPS,
    },
    Function {
        address: POST_THREADS_PROCESS,
        size: 101,
        name: Some("Main::OnIdle_PostThreadsProcess"),
        callers: &[caller(ON_IDLE, 0x0086_ee6a)],
        steps: &POST_THREADS_STEPS,
    },
    Function {
        address: OPEN_CONSOLE,
        size: 102,
        name: Some("Interface::OpenConsole"),
        callers: &[caller(ON_IDLE, 0x0086_ee85)],
        steps: &OPEN_CONSOLE_STEPS,
    },
    Function {
        address: POLL_CONTROLS,
        size: 182,
        name: Some("Main::OnIdle_PollControls"),
        callers: &[caller(ON_IDLE, 0x0086_e88c)],
        steps: &POLL_CONTROLS_STEPS,
    },
];

/// The function at `address` in [`FUNCTIONS`].
pub fn function(address: u32) -> Option<&'static Function> {
    FUNCTIONS.iter().find(|f| f.address == address)
}

/// Whether sub-step `i` of `f` is reached, given that `f` is called.
pub fn step_runs(s: &InterfaceState, f: &Function, i: usize) -> bool {
    f.steps[i].gate.holds(s)
}

/// The sub-steps of `f` reached with these inputs.
pub fn steps_run(s: &InterfaceState, f: &Function) -> Vec<usize> {
    (0..f.steps.len()).filter(|&i| step_runs(s, f, i)).collect()
}

/// The index of `f`'s sub-step whose call is at `site`.
pub fn step_at(f: &Function, site: u32) -> Option<usize> {
    f.steps.iter().position(|s| s.sites.contains(&site))
}

/// Who makes the frame's interface idle call (`0086fd70`) with these
/// inputs: `Main::OnIdle` before the AI work with one thread (step 80,
/// `0086eb36`), the AI thread's first call when the AI work runs with more
/// (the combined function with 2, thread 1 above), else `Main::OnIdle` after
/// the animations (step 105, `0086ecba`). `None` only when Tab and Alt
/// end the frame.
pub fn idle_caller(frame: &super::FrameState) -> Option<Caller> {
    if frame.alt_tab_held {
        return None;
    }
    let f = &FUNCTIONS[0];
    Some(if frame.threads == 1 {
        f.callers[0]
    } else if frame.process_lists() {
        if frame.threads <= 2 {
            f.callers[1]
        } else {
            f.callers[2]
        }
    } else {
        f.callers[3]
    })
}

/// The stages' inputs a frame's [`super::FrameState`] gives (the thread
/// count, menu mode, fader 1, the console, the frozen world, V.A.T.S.'s
/// playback, the Pip-Boy coming up, the sleep/wait menu on top); the rest
/// at [`InterfaceState::default`].
pub fn from_frame(frame: &super::FrameState, in_dialog: bool) -> InterfaceState {
    InterfaceState {
        threads: frame.threads,
        menu_flag: frame.menu_flag(),
        fader_visible: frame.fader_visible,
        console_visible: frame.console_visible,
        world_frozen: frame.world_frozen,
        vats_playback: frame.vats_mode == u32::from(crate::vats::mode::PLAYBACK),
        pipboy_opening: frame.pipboy_opening,
        sleep_menu: frame.top_menu == SLEEP_WAIT_MENU,
        in_dialog,
        mode: if frame.in_menu_mode {
            mode::MENUS
        } else {
            mode::GAME
        },
        ..InterfaceState::default()
    }
}

/// Checks a translation's calls (`log`, in order) against `f`'s model with
/// inputs `s`, as `world::frame::world_time::follows` does: every reached
/// sub-step without own tests is called, in order, and no call goes to a
/// callee only unreached sub-steps have. `ignore`: callees the translation
/// calls as Rust functions (not in its log).
pub fn follows(
    f: &Function,
    s: &InterfaceState,
    log: &[Callee],
    ignore: &[Callee],
) -> Result<(), String> {
    let reached = steps_run(s, f);
    let must: Vec<Callee> = reached
        .iter()
        .map(|&i| &f.steps[i])
        .filter(|st| st.own_tests.is_empty() && !ignore.contains(&st.callee))
        .map(|st| st.callee)
        .collect();
    let known: Vec<Callee> = f.steps.iter().map(|st| st.callee).collect();
    let seen: Vec<Callee> = log.iter().copied().filter(|c| known.contains(c)).collect();
    let mut at = 0;
    for want in &must {
        match seen[at..].iter().position(|c| c == want) {
            Some(p) => at += p + 1,
            None => {
                return Err(format!(
                    "{want:?} not called in order; calls {seen:?}, want {must:?}"
                ))
            }
        }
    }
    let allowed: Vec<Callee> = reached.iter().map(|&i| f.steps[i].callee).collect();
    if let Some(c) = seen
        .iter()
        .find(|c| !allowed.contains(c) && !ignore.contains(c))
    {
        return Err(format!(
            "{c:?} called but only unreached sub-steps call it; calls {seen:?}"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests;
