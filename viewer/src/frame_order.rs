//! The order of the viewer's per-frame systems, from the game's frame
//! (`Main::OnIdle`, Xbox PDB; PC `0086e650`; `world::frame`,
//! docs/FRAME_SKELETON.md "PR 3 result").
//!
//! Each of `world::frame`'s stages is a system set ([`FrameSet::Stage`]),
//! run in the exe's order ([`world::frame::stages`]), and so is each of its
//! steps ([`FrameSet::Step`], an index into `world::frame::STEPS`, inside
//! its stage, in call order). A system that is (part of) a step goes in
//! the step's set; one that belongs to a stage without being one step goes
//! in the stage's set. Each set runs under its gate from `world::frame`
//! ([`world::frame::step_runs`], [`world::frame::stage_reached`]),
//! evaluated on one [`FrameState`] ([`ThisFrame`]) filled once per frame
//! by [`begin_frame`], the first system of the first stage.
//!
//! Two sets run ahead of the stages and one after them ([`ViewerSet`]);
//! see there for what they hold and why. The menus run at the interface
//! idle ([`IdleSet`]), after the controls' poll (step 33, [`poll_set`],
//! `crate::poll`), which the player's step reads a frame later.
//!
//! The player's step (`Main::OnIdle_UpdatePlayer`, step 14) is split
//! further into its calls and `PlayerCharacter::Update`'s sub-steps
//! ([`PlayerSet`], `world::frame::player`, docs/FRAME_SKELETON.md "PR 4
//! result"), under their gates on [`ThisPlayer`].
//!
//! The world and time stage's callees (`TES::TestAllCells`, `Calendar::Update`,
//! the process lists' passes, the garbage collector, `BSTreeManager::Update`,
//! `Main::OnIdle_UpdateCurrentGridCell` and `TES::UpdateCurrentGridCell`) are
//! split into their sub-steps ([`WorldSet`], `world::frame::world_time`,
//! docs/FRAME_SKELETON.md "PR 5 result"), under their gates on [`ThisWorld`].
//!
//! The AI task stage (the AI linear task threads' work between
//! `AILinearTaskThreadManager::StartThreads` and `WaitForThreads`, and the
//! main thread's `Main::OnIdle_UpdateAnimationsAndEffects` with the Havok
//! step, the actor updates and the sky inside them) is split into the
//! threads' calls in one order the exe runs them in and the sub-steps of
//! the functions they call ([`AiSet`], `world::frame::ai_stage`,
//! docs/FRAME_SKELETON.md "PR 6 result"), under their gates on [`ThisAi`].

use bevy::prelude::*;
use world::frame::ai_stage::{self, AiState};
use world::frame::interface::{self, InterfaceState};
use world::frame::player::{self, PlayerState, UPDATE, UPDATE_PLAYER};
use world::frame::world_time::{self, WorldState, FUNCTIONS};
use world::frame::{self, FrameState, Stage, STEPS};

/// The game's frame as system sets.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrameSet {
    /// One of `world::frame`'s stages.
    Stage(Stage),
    /// One of its steps (an index into `world::frame::STEPS`).
    Step(usize),
}

/// Interface/render sub-steps with a unique main-thread caller. The idle's
/// three alternative callers remain modelled in core; the viewer's menus
/// run at one of them ([`IdleSet`]).
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterfaceSet(pub u32, pub usize);

impl InterfaceSet {
    pub fn at(function: u32, site: u32) -> Self {
        let f = interface::function(function).expect("interface function");
        Self(
            function,
            interface::step_at(f, site).expect("interface sub-step"),
        )
    }
}

/// Only frame-derived inputs drive wired systems. Unavailable manager,
/// renderer and input state remains model-only; no system uses those gates.
#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct ThisInterface(pub InterfaceState);

impl FrameSet {
    /// The set of the first step that calls `address`.
    pub fn step(address: u32) -> FrameSet {
        FrameSet::Step(frame::step_of(address).expect("a call of Main::OnIdle"))
    }

    /// The set of the step after the first step that calls `address`.
    pub fn after(address: u32) -> FrameSet {
        FrameSet::Step(frame::step_of(address).expect("a call of Main::OnIdle") + 1)
    }
}

/// `Main::OnIdle_UpdateTimer` (step 32).
pub const UPDATE_TIMER: u32 = 0x0086_f260;
/// `Main::OnIdle_HandleMenuBackground` (step 44).
pub const HANDLE_MENU_BACKGROUND: u32 = 0x0086_f450;
/// `BSTreeManager::Update` (step 77).
pub const TREE_MANAGER_UPDATE: u32 = 0x0066_52e0;

/// Viewer work outside the frame's stages.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ViewerSet {
    /// First: the controls' buffer put back as the last frame left it, this
    /// frame's device read (Bevy's `PreUpdate`) kept for the poll at step 33
    /// (`crate::poll`, `world::frame::input`). Not a call of the exe's: the
    /// buffer simply keeps its contents between frames there.
    Input,
    /// Then: putting a newly loaded interior or exterior on screen and
    /// streaming the land around the player. In the exe, loading belongs to
    /// the frame (`TES::ShowLoadingMenu`, step 39; the grid cells' attach
    /// in `Main::OnIdle_UpdateCurrentGridCell`, step 78), but the viewer's
    /// loaders aren't split along those calls yet and other systems rely on
    /// a scene being installed at the start of the frame (a door or a
    /// script's `MoveTo` asks for it one frame, it is in place the next).
    Loading,
    /// Last: what is not in `Main::OnIdle`, the viewer's own tools
    /// (screenshots, help, the cursor, exposure, the F12 report, the frame
    /// rate, the window's focus, the present mode, the console key's switch
    /// to its free camera, billboards turned to Bevy's camera); and output
    /// whose place in the frame isn't traced, which only needs the frame's
    /// work done (the GPU's grass and water around the camera).
    AfterFrame,
}

/// The interface idle's menus (`Interface::Idle`, `0070c4a0` through
/// `Main::OnIdle_DoInterfaceIdle`): the game's menus, the viewer's own
/// menus, V.A.T.S. and the lockpicking menu, which take the controls'
/// buffer (`reset_all`, as the exe's interface clears its keystrokes) after
/// the poll (step 33) has put this frame's input in, and so before the
/// player's step reads it in the next frame (`world::frame::input`,
/// docs/FRAME_SKELETON.md "PR 8 result").
///
/// Placed at the AI linear task threads' first call (`008c7be9` or
/// `008c7db9`, [`AiSet::Call`] of [`INTERFACE_IDLE`]), outside its gate, as
/// the HUD and the Pip-Boy's screen are (which follow it): that is where the
/// exe calls the idle when the AI work runs. With a thread count of 1
/// (which `main`'s raise to 2 keeps from happening) it is step 80's place.
/// Open: without the AI work (menu mode) the exe calls it at step 105,
/// after `Main::OnIdle_UpdateAnimationsAndEffects`; that is after the poll
/// too, but the viewer doesn't move the set there.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdleSet;

/// The player stage as system sets (`world::frame::player`, Phase 1 PR 4):
/// inside step 14's set (`Main::OnIdle_UpdatePlayer`, `0086f940`) each of
/// its calls in call order, and inside the call of
/// `PlayerCharacter::Update` (slot +0x2f8) each of that function's
/// sub-steps in call order; each under its gate on [`ThisPlayer`].
///
/// A player system that is (part of) a sub-step goes in the sub-step's set.
/// One that belongs at a sub-step but also does work while that sub-step's
/// gate is closed (in menu mode, in V.A.T.S.'s menu) is ordered at the
/// sub-step from outside its set, so that it keeps running: see where each
/// is added in `main.rs`.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerSet {
    /// A call of `0086f940` (an index into `world::frame::player::UPDATE_PLAYER`).
    Call(usize),
    /// A sub-step of `PlayerCharacter::Update` (an index into
    /// `world::frame::player::UPDATE`).
    Update(usize),
}

impl PlayerSet {
    /// The set of the first call of `0086f940` to `address`.
    pub fn call(address: u32) -> PlayerSet {
        PlayerSet::Call(player::call_of(address).expect("a call of Main::OnIdle_UpdatePlayer"))
    }

    /// The set of the call of `PlayerCharacter::Update`.
    pub fn update() -> PlayerSet {
        PlayerSet::Call(player::update_call())
    }

    /// The set of the sub-step of `PlayerCharacter::Update` whose call is at
    /// `site`.
    pub fn at(site: u32) -> PlayerSet {
        PlayerSet::Update(Self::index(site))
    }

    /// The set of the sub-step after the one at `site`.
    pub fn after(site: u32) -> PlayerSet {
        PlayerSet::Update(Self::index(site) + 1)
    }

    fn index(site: u32) -> usize {
        player::step_at(site).expect("a sub-step of PlayerCharacter::Update")
    }
}

/// `Main::OnIdle_UpdatePlayer` (step 14).
pub const UPDATE_PLAYER_STEP: u32 = player::UPDATE_PLAYER_ADDRESS;
/// `PlayerCharacter::UpdateFlyCamera` (`0086f940`'s call at `0086f9c7`).
pub const UPDATE_FLY_CAMERA: u32 = 0x0094_66d0;
/// `PlayerCharacter::UpdateHeadingAndLooking`'s call in
/// `PlayerCharacter::Update`.
pub const HEADING_AND_LOOKING_AT: u32 = 0x0093_f8d9;
/// The attack's call (`00948310`) on the free branch.
pub const ATTACK_AT: u32 = 0x0094_20fc;
/// The move vector given to the player's mover (`009ea570`) on the free
/// branch; its slot +0x14 then moves the player.
pub const MOVE_AT: u32 = 0x0094_280b;
/// The animation update of the view the player is in (the second
/// `Actor::UpdateAnimationMovement` of the free branch).
pub const OWN_VIEW_ANIMATION_AT: u32 = 0x0094_3806;

/// The world and time stage's callees as system sets
/// (`world::frame::world_time`, Phase 1 PR 5): each sub-step of each
/// modelled function, in call order inside the set of the frame step that
/// calls the function (its first, the one the viewer's thread count reaches:
/// the process lists are called twice, threads > 1 first) or of its
/// caller's sub-step (`TES::UpdateCurrentGridCell` inside
/// `Main::OnIdle_UpdateCurrentGridCell`'s call of it); each under its gate
/// on [`ThisWorld`].
///
/// As for the player's sets, a system that belongs at a sub-step but also
/// does work while the sub-step's gate is closed is ordered at it from
/// outside its set: see where each is added.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorldSet {
    /// Sub-step `.1` of the function at `.0` (an index into its
    /// `world::frame::world_time::Function::steps`).
    Sub(u32, usize),
}

impl WorldSet {
    /// The set of `function`'s sub-step whose call is at `site`.
    pub fn at(function: u32, site: u32) -> WorldSet {
        WorldSet::Sub(function, Self::index(function, site))
    }

    /// The set of the sub-step after the one at `site`.
    pub fn after(function: u32, site: u32) -> WorldSet {
        WorldSet::Sub(function, Self::index(function, site) + 1)
    }

    fn index(function: u32, site: u32) -> usize {
        let f = world_time::function(function).expect("a modelled function");
        world_time::step_at(f, site).expect("a sub-step")
    }
}

/// `ProcessLists::RunActorScripts` (step 60).
pub const RUN_ACTOR_SCRIPTS: u32 = 0x0097_8550;
/// `TESObjectREFR::RunScript`'s call in it.
pub const RUN_SCRIPT_AT: u32 = 0x0097_85bf;
/// The wind update's call (`006658b0`) in `BSTreeManager::Update`.
pub const TREE_WIND_AT: u32 = 0x0066_54dd;
/// `Main::OnIdle_UpdateCurrentGridCell` (step 78).
pub const UPDATE_CURRENT_GRID_CELL_STEP: u32 = 0x0086_fbe0;
/// `TES::UpdateCurrentGridCell`.
pub const UPDATE_CURRENT_GRID_CELL: u32 = 0x0045_2580;
/// `GridCellArray::SetCenter`'s call (the grid array's slot +0x10) in
/// `TES::UpdateCurrentGridCell`: the cells detached and attached around the
/// new centre.
pub const GRID_SET_CENTER_AT: u32 = 0x0045_2c40;
/// `BGSTerrainManager::Update`'s call in `TES::UpdateCurrentGridCell`.
pub const TERRAIN_UPDATE_AT: u32 = 0x0045_2d9f;

/// The world and time stage's inputs, filled with [`ThisFrame`]
/// ([`begin_frame`]).
#[derive(Resource, Debug, Clone, Copy, PartialEq, Default)]
pub struct ThisWorld(pub WorldState);

/// The world stage's inputs from the frame's and the viewer's values. Those
/// the viewer has no source for are fixed (docs/FRAME_SKELETON.md, "PR 5
/// result"):
///
/// - the world runs, menu mode, the frozen world, the cell tests: the
///   frame's ([`ThisFrame`]);
/// - an interior loaded, the sky: whether the viewer is indoors (no
///   `exterior::Exterior`); the scene graph's camera is there;
/// - the rest at `WorldState::default`: no new game's loading menu, no cell
///   test's walk, the calendar's carries, radiation, resting and the garbage
///   queues not tracked (nothing in the viewer sits under those gates), the
///   data handler there, the position inside the grid's centre cell (the
///   viewer's squares stream on their own, `exterior::stream_squares`, which
///   is ordered at the grid's sub-steps but not under their gates), no
///   script running.
pub fn viewer_world_state(frame: &FrameState, indoors: bool) -> WorldState {
    WorldState {
        interior_loaded: indoors,
        sky: !indoors,
        ..world_time::from_frame(frame)
    }
}

/// The AI task stage as system sets (`world::frame::ai_stage`, Phase 1
/// PR 6).
///
/// The AI linear task threads' work calls, in [`ai_stage::schedule`]'s
/// order for the thread count: those before the render wait inside
/// `AILinearTaskThreadManager::StartThreads`' step (stage 6), those after it
/// inside `WaitForThreads`' (stage 8), so they run under the frame's gate
/// for the AI work, and each under its own ([`ai_stage::thread_call_runs`]).
/// The thread split itself is platform: this one thread runs both threads'
/// calls in an order they can run in. Inside a call (or a frame step) the
/// sub-steps of the function it calls, in call order, under their gates on
/// [`ThisAi`].
///
/// As for the other stages' sets, a system that belongs at a call but also
/// does work while its gate is closed (in menu mode, in the dialogue menu)
/// is ordered at it from outside the sets: see where each is added.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AiSet {
    /// The threads' first work call of the function at `.0`.
    Call(u32),
    /// The other thread's call of the same function (each thread's own
    /// input and scripts' optimizations).
    Again(u32),
    /// Sub-step `.1` of the modelled function at `.0` (an index into its
    /// `world::frame::ai_stage::Function::steps`).
    Sub(u32, usize),
}

impl AiSet {
    /// The set of `function`'s sub-step whose call is at `site`.
    pub fn at(function: u32, site: u32) -> AiSet {
        let f = ai_stage::function(function).expect("a modelled function");
        AiSet::Sub(function, ai_stage::step_at(f, site).expect("a sub-step"))
    }

    /// The set of the threads' work call after the first one to `callee`,
    /// in this machine's order.
    pub fn next(callee: u32) -> AiSet {
        Self::next_with(callee, thread_count())
    }

    /// The set of the threads' work call before it.
    pub fn prev(callee: u32) -> AiSet {
        Self::prev_with(callee, thread_count())
    }

    /// [`AiSet::next`] for a thread count.
    pub fn next_with(callee: u32, threads: i32) -> AiSet {
        let sets = ai_call_sets(threads);
        let i = sets
            .iter()
            .position(|s| s.0 == AiSet::Call(callee))
            .expect("a call");
        sets[i + 1].0
    }

    /// [`AiSet::prev`] for a thread count.
    pub fn prev_with(callee: u32, threads: i32) -> AiSet {
        let sets = ai_call_sets(threads);
        let i = sets
            .iter()
            .position(|s| s.0 == AiSet::Call(callee))
            .expect("a call");
        sets[i - 1].0
    }
}

/// `AILinearTaskThreadManager::StartThreads` (stage 6).
pub const START_THREADS: u32 = ai_stage::START_THREADS;
/// `AILinearTaskThreadManager::WaitForThreads` (stage 8).
pub const WAIT_FOR_THREADS: u32 = ai_stage::WAIT_FOR_THREADS;
/// `Main::PostSwapProcess` (stage 7), whose `Main::OnIdle_UpdateProcessLists`
/// (`0086f890`) runs `ProcessLists::UpdateProcessLists` (`0096d810`): the
/// lower process levels' moves (`0096b810`, `0096b470`, `0096b050`).
#[cfg(test)]
pub const POST_SWAP_PROCESS: u32 = 0x0087_05d0;
/// `Main::OnIdle_UpdateAnimationsAndEffects` (stage 6, after the threads
/// start).
pub const UPDATE_ANIMATIONS_AND_EFFECTS: u32 = ai_stage::UPDATE_ANIMATIONS_AND_EFFECTS;
/// `TES::UpdateCellMainThread`, which the main thread calls there with
/// threads > 1.
pub const UPDATE_CELL_MAIN_THREAD: u32 = ai_stage::UPDATE_CELL_MAIN_THREAD;
/// `Sky::Update`'s call in it.
pub const SKY_UPDATE_AT: u32 = 0x0045_3811;
/// `TES::UpdateCellAnimations`, the Havok step's function.
pub const UPDATE_CELL_ANIMATIONS: u32 = ai_stage::UPDATE_CELL_ANIMATIONS;
/// `TES::LockHavokUpdateMT(1)`'s call in it, before the managed nodes.
pub const HAVOK_LOCK_AT: u32 = 0x0045_35ee;
/// `TES::LockHavokUpdateMT(0)`'s call, after them.
pub const HAVOK_UNLOCK_AT: u32 = 0x0045_3629;
/// The threads' calls the viewer's systems sit at.
pub use ai_stage::{ACTORS_MOVEMENT, ACTOR_ANIMATION_UPDATES, INTERFACE_IDLE, RUN_ANIMATIONS};

/// The threads' work calls as sets in [`ai_stage::scheduled_calls`]' order,
/// each with its thread function, step and whether it comes after the
/// render.
fn ai_call_sets(threads: i32) -> Vec<(AiSet, u32, usize, bool)> {
    let mut seen = Vec::new();
    ai_stage::scheduled_calls(threads)
        .into_iter()
        .map(|(t, i, after)| {
            let th = ai_stage::thread(t).expect("a thread function");
            let ai_stage::ThreadOp::Call { callee, .. } = th.steps[i].op else {
                unreachable!("a work call")
            };
            let set = if seen.contains(&callee) {
                AiSet::Again(callee)
            } else {
                seen.push(callee);
                AiSet::Call(callee)
            };
            (set, t, i, after)
        })
        .collect()
}

/// The AI stage's inputs, filled with [`ThisFrame`] ([`begin_frame`]).
#[derive(Resource, Debug, Clone, Copy, PartialEq, Default)]
pub struct ThisAi(pub AiState);

/// The AI stage's inputs from the frame's and the viewer's values
/// (docs/FRAME_SKELETON.md, "PR 6 result"):
///
/// - the thread count, menu mode, the fader, the frozen world, obstacle
///   avoidance: the frame's ([`ThisFrame`]);
/// - dialogue (`[011dea2c]`, `Interface::InDialog`): the dialogue menu up;
/// - an interior loaded: no `exterior::Exterior`;
/// - the rest at `AiState::default`: `Actor::Update`'s per-actor inputs
///   (no viewer system sits under its gates).
pub fn viewer_ai_state(frame: &FrameState, in_dialogue: bool, indoors: bool) -> AiState {
    ai_stage::from_frame(frame, in_dialogue, indoors)
}

/// The frame's inputs to the gates, filled once per frame ([`begin_frame`]).
#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub struct ThisFrame(pub FrameState);

/// The player stage's inputs, filled with [`ThisFrame`] ([`begin_frame`]).
#[derive(Resource, Debug, Clone, Copy, PartialEq, Default)]
pub struct ThisPlayer(pub PlayerState);

/// The player stage's inputs from the frame's and the viewer's values. Those
/// the viewer has no source for are fixed (docs/FRAME_SKELETON.md, "PR 4
/// result"):
///
/// - menu mode and the Pip-Boy's opening: the frame's ([`ThisFrame`]);
/// - the fly camera: the viewer's free camera (the ` key,
///   `walk::Player::walking` off), which is `TFC` without an argument: the
///   player isn't updated, the world runs (`world_frozen` stays off);
/// - dialogue (`Interface::InDialog`): the dialogue menu up;
/// - the rest at `PlayerState::default`: no position request (the viewer's
///   doors and `MoveTo` load in `ViewerSet::Loading`), the player's 3D there
///   (its systems test `walk::Player::ready` themselves), not
///   AI-controlled, not dead (the exe's dead player takes the controlled
///   branch, but the viewer's death countdown is in `combat::player_attack`,
///   which sits on the free branch, and would stop), not knocked down, no
///   fade, no forced activation, no muzzle flash.
pub fn viewer_player_state(frame: &FrameState, walking: bool, in_dialogue: bool) -> PlayerState {
    PlayerState {
        menu_flag: frame.menu_flag(),
        pipboy_opening: frame.pipboy_opening,
        fly_camera: !walking,
        in_dialogue,
        ..PlayerState::default()
    }
}

impl Default for ThisFrame {
    fn default() -> ThisFrame {
        ThisFrame(viewer_frame_state(FrameInputs::default()))
    }
}

/// The values the viewer has for the frame's inputs.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct FrameInputs {
    /// Tab and Alt held.
    pub alt_tab_held: bool,
    /// A menu is up: the game's own, the viewer's, the Pip-Boy, the
    /// lockpicking menu (`menus::Menus::is_open`), the dialogue menu, the
    /// V.A.T.S. menu.
    pub menu_up: bool,
    /// The V.A.T.S. manager's mode (`vats::Vats::manager_mode`).
    pub vats_mode: u8,
    /// The class of the top game menu on screen (`game_menus::MenuDraw`).
    pub top_menu: Option<i32>,
}

/// The thread count `main` leaves (`0086a950`-`0086a990`): the processor
/// count (`GetSystemInfo`; here the standard library's), raised to 2 when
/// it is 1. Whether the INI can lower it afterwards is not traced.
pub fn thread_count() -> i32 {
    let n = std::thread::available_parallelism().map_or(1, |n| n.get());
    i32::try_from(n).unwrap_or(i32::MAX).max(2)
}

/// The frame's inputs from the viewer's values. Those the viewer has no
/// source for are fixed at the normal PC case (docs/FRAME_SKELETON.md, "PR 3
/// result"):
///
/// - fader 1 (`FaderManager::IsFaderVisible(1)`): not visible. The viewer's
///   two faders (the sleep's and the menus' fade to black) are fader 0.
/// - the frozen world (`Main` +7, the console's `TFC` 1, 2 or 5): not
///   frozen. The viewer has no console window, and its free camera (the `
///   key, `walk::toggle_walking`) is its own, not `TFC`: the world runs.
/// - the interface mode (`[011d8a80]`+0xc): 1, game mode. Menu mode comes
///   from `menu_up`; only the memory free's gate (step 13, open) reads the
///   number itself.
/// - the thread count: [`thread_count`], at least 2 as `main` leaves it.
///
/// Also fixed: the console is hidden (no console window), the Pip-Boy's
/// opening is part of `menu_up` (`Menus::pipboy`), and the requests and
/// loading inputs of steps no viewer system implements keep
/// `FrameState::default`'s values.
pub fn viewer_frame_state(v: FrameInputs) -> FrameState {
    FrameState {
        alt_tab_held: v.alt_tab_held,
        in_menu_mode: v.menu_up,
        pipboy_opening: false,
        fader_visible: false,
        console_visible: false,
        interface_mode: 1,
        world_frozen: false,
        threads: thread_count(),
        vats_mode: u32::from(v.vats_mode),
        top_menu: v.top_menu.and_then(|c| u32::try_from(c).ok()).unwrap_or(0),
        ..FrameState::default()
    }
}

/// Fills [`ThisFrame`]: the menu state queries at the start of
/// `Main::OnIdle` (steps 3-12), with the Tab and Alt test (`0086e682`,
/// `0086e69a`: `GetAsyncKeyState` on Tab and either Alt).
#[allow(clippy::too_many_arguments)]
pub fn begin_frame(
    keys: Res<ButtonInput<KeyCode>>,
    buffer: Option<Res<crate::poll::ControlsBuffer>>,
    menus: Res<crate::menus::Menus>,
    conversation: Res<crate::dialogue::Conversation>,
    vats: Option<Res<crate::vats::Vats>>,
    draw: Res<crate::game_menus::MenuDraw>,
    walker: Res<crate::walk::Player>,
    exterior: Option<Res<crate::exterior::Exterior>>,
    mut this_interface: ResMut<ThisInterface>,
    (mut this, mut this_player, mut this_world, mut this_ai): (
        ResMut<ThisFrame>,
        ResMut<ThisPlayer>,
        ResMut<ThisWorld>,
        ResMut<ThisAi>,
    ),
) {
    let vats_mode = vats.as_ref().map_or(0, |v| v.manager_mode());
    let in_dialogue = conversation.0.as_ref().is_some_and(|t| !t.is_line_only());
    // `GetAsyncKeyState` reads the keyboard itself, not the controls'
    // buffer: this frame's device read (`world::frame::input::DEVICE_READS`).
    let keys = buffer
        .as_deref()
        .and_then(crate::poll::ControlsBuffer::device_keys)
        .unwrap_or(&keys);
    this.0 = viewer_frame_state(FrameInputs {
        alt_tab_held: keys.pressed(KeyCode::Tab)
            && (keys.pressed(KeyCode::AltLeft) || keys.pressed(KeyCode::AltRight)),
        menu_up: (menus.is_open() && crate::coop_live::menus_freeze_time())
            || in_dialogue
            || vats.is_some_and(|v| v.in_menu()),
        vats_mode,
        top_menu: draw.1.last().copied(),
    });
    this_player.0 = viewer_player_state(&this.0, walker.walking, in_dialogue);
    this_world.0 = viewer_world_state(&this.0, exterior.is_none());
    this_ai.0 = viewer_ai_state(&this.0, in_dialogue, exterior.is_none());
    this_interface.0 = interface::from_frame(&this.0, in_dialogue);
}

/// Configures the sets in `Update`: the viewer's two sets ahead, the
/// stages in the exe's order, each step inside its stage in call order,
/// the viewer's last set after; each stage and step under its gate on
/// [`ThisFrame`].
pub fn configure(app: &mut App) {
    configure_with(app, thread_count());
}

/// [`configure`] for a thread count (the AI stage's order depends on it).
pub fn configure_with(app: &mut App, threads: i32) {
    app.init_resource::<ThisFrame>();
    let stages = frame::stages();
    app.configure_sets(
        Update,
        (
            ViewerSet::Input,
            ViewerSet::Loading,
            FrameSet::Stage(stages[0]),
        )
            .chain(),
    );
    for pair in stages.windows(2) {
        app.configure_sets(
            Update,
            FrameSet::Stage(pair[0]).before(FrameSet::Stage(pair[1])),
        );
    }
    app.configure_sets(
        Update,
        FrameSet::Stage(*stages.last().expect("stages")).before(ViewerSet::AfterFrame),
    );
    for stage in stages {
        app.configure_sets(
            Update,
            FrameSet::Stage(stage)
                .run_if(move |this: Res<ThisFrame>| frame::stage_reached(&this.0, stage)),
        );
    }
    for (i, step) in STEPS.iter().enumerate() {
        app.configure_sets(
            Update,
            FrameSet::Step(i)
                .in_set(FrameSet::Stage(step.stage))
                .run_if(move |this: Res<ThisFrame>| frame::step_runs(&this.0, i)),
        );
        if i + 1 < STEPS.len() && STEPS[i + 1].stage == step.stage {
            app.configure_sets(Update, FrameSet::Step(i).before(FrameSet::Step(i + 1)));
        }
    }
    configure_player(app);
    configure_world(app);
    configure_ai(app, threads);
    configure_interface(app);
    // The menus at the interface idle, after the controls' poll: the AI
    // threads' first call; with one thread (no threads, which `main`'s
    // raise to 2 keeps the viewer from) step 80's place.
    if ai_call_sets(threads)
        .iter()
        .any(|s| s.0 == AiSet::Call(INTERFACE_IDLE))
    {
        app.configure_sets(
            Update,
            IdleSet
                .in_set(FrameSet::Stage(Stage::AiStart))
                .after(AiSet::Call(INTERFACE_IDLE))
                .before(AiSet::next_with(INTERFACE_IDLE, threads)),
        );
    } else {
        let i = frame::step_of(interface::DO_INTERFACE_IDLE).expect("step 80");
        app.configure_sets(
            Update,
            IdleSet
                .in_set(FrameSet::Stage(STEPS[i].stage))
                .after(FrameSet::Step(i - 1))
                .before(FrameSet::Step(i + 1)),
        );
    }
}

/// The controls' poll's set: `Controls::Poll`'s sub-step of
/// `Main::OnIdle_PollControls` (step 33, `world::frame::input`).
pub fn poll_set() -> InterfaceSet {
    InterfaceSet::at(
        world::frame::input::POLL_CONTROLS,
        world::frame::input::CONTROLS_POLL_SITE,
    )
}

/// The unique main-thread calls, and their children, in executable order.
/// Multiple-caller functions (the idle and menu background) are deliberately
/// excluded: selecting their first call would silently lose the fallback path.
fn configure_interface(app: &mut App) {
    app.init_resource::<ThisInterface>();
    let mut configured = Vec::new();
    for f in &interface::FUNCTIONS {
        if f.callers.len() != 1 {
            continue;
        }
        let caller = f.callers[0];
        let frame_parent =
            (caller.function == interface::ON_IDLE).then(|| FrameSet::step(f.address));
        let interface_parent = configured
            .contains(&caller.function)
            .then(|| InterfaceSet::at(caller.function, caller.site));
        if frame_parent.is_none() && interface_parent.is_none() {
            continue;
        }
        configured.push(f.address);
        for i in 0..f.steps.len() {
            let address = f.address;
            let set = InterfaceSet(address, i);
            if let Some(parent) = frame_parent {
                app.configure_sets(Update, set.in_set(parent));
            } else if let Some(parent) = interface_parent {
                app.configure_sets(Update, set.in_set(parent));
            }
            app.configure_sets(
                Update,
                set.run_if(move |this: Res<ThisInterface>| {
                    interface::step_runs(&this.0, interface::function(address).unwrap(), i)
                }),
            );
            if i + 1 < f.steps.len() {
                app.configure_sets(Update, set.before(InterfaceSet(address, i + 1)));
            }
        }
    }
}

/// The AI stage's sets ([`AiSet`]) for a thread count: the threads' calls
/// in their order inside `StartThreads`' step (before the render wait) and
/// `WaitForThreads`' (after it), each under its gate on [`ThisAi`]; each
/// modelled function's sub-steps inside the set of the call (or frame
/// step, or sub-step) through which this thread count reaches it first,
/// in call order under their gates.
pub fn configure_ai(app: &mut App, threads: i32) {
    app.init_resource::<ThisAi>();
    let sets = ai_call_sets(threads);
    // Each set with the ones around it, for the nested sets' anchors below.
    let mut around: Vec<(AiSet, Option<AiSet>, Option<AiSet>)> = Vec::new();
    for after in [false, true] {
        let step = if after {
            WAIT_FOR_THREADS
        } else {
            START_THREADS
        };
        let index = frame::step_of(step).expect("a call of Main::OnIdle");
        let part: Vec<&(AiSet, u32, usize, bool)> = sets.iter().filter(|s| s.3 == after).collect();
        for (k, &&(set, t, i, _)) in part.iter().enumerate() {
            app.configure_sets(
                Update,
                set.in_set(FrameSet::Step(index))
                    .run_if(move |this: Res<ThisAi>| {
                        let th = ai_stage::thread(t).expect("a thread function");
                        ai_stage::thread_call_runs(&this.0, th, i)
                    }),
            );
            if k + 1 < part.len() {
                app.configure_sets(Update, set.before(part[k + 1].0));
            }
            let prev = k.checked_sub(1).map(|j| part[j].0);
            let next = part.get(k + 1).map(|s| s.0);
            around.push((set, prev, next));
        }
        // The calls also follow the frame's step before `StartThreads`
        // (`WaitForThreads`) and come before the one after it, so that a
        // system ordered at an empty call set keeps its place.
        if let (Some(first), Some(last)) = (part.first(), part.last()) {
            app.configure_sets(
                Update,
                (
                    first.0.after(FrameSet::Step(index - 1)),
                    last.0.before(FrameSet::Step(index + 1)),
                ),
            );
        }
    }
    for f in &ai_stage::FUNCTIONS {
        let Some(caller) = ai_stage::first_caller(f, threads) else {
            continue;
        };
        let address = f.address;
        // The set the sub-steps go in, and its neighbours.
        let (parent, prev, next): (FrameSet, Option<FrameSet>, Option<FrameSet>);
        let (ai_parent, ai_prev, ai_next): (Option<AiSet>, Option<AiSet>, Option<AiSet>);
        if caller.function == ai_stage::ON_IDLE {
            let i = frame::step_of(address).expect("a call of Main::OnIdle");
            (parent, prev, next) = (
                FrameSet::Step(i),
                Some(FrameSet::Step(i - 1)),
                Some(FrameSet::Step(i + 1)),
            );
            (ai_parent, ai_prev, ai_next) = (None, None, None);
        } else if ai_stage::thread(caller.function).is_some() {
            let (set, p, n) = *around
                .iter()
                .find(|a| a.0 == AiSet::Call(address))
                .expect("the thread's call");
            (parent, prev, next) = (FrameSet::Step(0), None, None);
            (ai_parent, ai_prev, ai_next) = (Some(set), p, n);
        } else {
            let c = ai_stage::function(caller.function).expect("a modelled caller");
            let k = ai_stage::step_at(c, caller.site).expect("the caller's call");
            (parent, prev, next) = (FrameSet::Step(0), None, None);
            (ai_parent, ai_prev, ai_next) = (
                Some(AiSet::Sub(caller.function, k)),
                k.checked_sub(1).map(|j| AiSet::Sub(caller.function, j)),
                (k + 1 < c.steps.len()).then_some(AiSet::Sub(caller.function, k + 1)),
            );
        }
        let n = f.steps.len();
        for i in 0..n {
            let set = AiSet::Sub(address, i);
            match ai_parent {
                Some(p) => app.configure_sets(Update, set.in_set(p)),
                None => app.configure_sets(Update, set.in_set(parent)),
            };
            app.configure_sets(
                Update,
                set.run_if(move |this: Res<ThisAi>| {
                    let f = ai_stage::function(address).expect("modelled");
                    ai_stage::step_runs(&this.0, f, i)
                }),
            );
            if i + 1 < n {
                app.configure_sets(Update, set.before(AiSet::Sub(address, i + 1)));
            }
        }
        // The sub-steps also follow what comes before their call and come
        // before what follows it (the hierarchy alone doesn't order them),
        // so a system ordered at one from outside the sets keeps its place
        // when they are empty.
        let (first, last) = (AiSet::Sub(address, 0), AiSet::Sub(address, n - 1));
        match ai_parent {
            Some(_) => {
                if let Some(p) = ai_prev {
                    app.configure_sets(Update, first.after(p));
                }
                if let Some(x) = ai_next {
                    app.configure_sets(Update, last.before(x));
                }
            }
            None => {
                if let Some(p) = prev {
                    app.configure_sets(Update, first.after(p));
                }
                if let Some(x) = next {
                    app.configure_sets(Update, last.before(x));
                }
            }
        }
    }
}

/// The player stage's sets ([`PlayerSet`]): the calls of `0086f940` inside
/// its step, the sub-steps of `PlayerCharacter::Update` inside its call,
/// each in call order under its gate on [`ThisPlayer`].
fn configure_player(app: &mut App) {
    app.init_resource::<ThisPlayer>();
    let step = FrameSet::step(UPDATE_PLAYER_STEP);
    for i in 0..UPDATE_PLAYER.len() {
        app.configure_sets(
            Update,
            PlayerSet::Call(i)
                .in_set(step)
                .run_if(move |this: Res<ThisPlayer>| player::call_runs(&this.0, i)),
        );
        if i + 1 < UPDATE_PLAYER.len() {
            app.configure_sets(Update, PlayerSet::Call(i).before(PlayerSet::Call(i + 1)));
        }
    }
    for i in 0..UPDATE.len() {
        app.configure_sets(
            Update,
            PlayerSet::Update(i)
                .in_set(PlayerSet::update())
                .run_if(move |this: Res<ThisPlayer>| player::step_runs(&this.0, i)),
        );
        if i + 1 < UPDATE.len() {
            app.configure_sets(
                Update,
                PlayerSet::Update(i).before(PlayerSet::Update(i + 1)),
            );
        }
    }
    // The sub-steps also follow the calls before the update's and come
    // before the calls after it as an order of their own (the hierarchy
    // alone doesn't order them), so a system ordered at a sub-step from
    // outside the sets keeps its place when the update's sets are empty.
    let call = player::update_call();
    app.configure_sets(
        Update,
        (
            PlayerSet::Update(0).after(PlayerSet::Call(call - 1)),
            PlayerSet::Update(UPDATE.len() - 1).before(PlayerSet::Call(call + 1)),
        ),
    );
}

/// The world stage's sets ([`WorldSet`]): each modelled function's
/// sub-steps inside the set of its call (a frame step, or its caller's
/// sub-step), in call order under their gates on [`ThisWorld`].
fn configure_world(app: &mut App) {
    app.init_resource::<ThisWorld>();
    for f in &FUNCTIONS {
        let address = f.address;
        let (step, around) = if f.caller == world_time::ON_IDLE {
            let step = frame::step_of(address).expect("a call of Main::OnIdle");
            (Some(FrameSet::Step(step)), None)
        } else {
            let caller = world_time::function(f.caller).expect("a modelled caller");
            let k = world_time::step_at(caller, f.call_sites[0]).expect("the caller's call");
            let parent = WorldSet::Sub(f.caller, k);
            let before = k.checked_sub(1).map(|j| WorldSet::Sub(f.caller, j));
            let after = (k + 1 < caller.steps.len()).then_some(WorldSet::Sub(f.caller, k + 1));
            (None, Some((parent, before, after)))
        };
        let n = f.steps.len();
        for i in 0..n {
            let set = WorldSet::Sub(address, i);
            match (step, around) {
                (Some(step), _) => app.configure_sets(Update, set.in_set(step)),
                (None, Some((parent, _, _))) => app.configure_sets(Update, set.in_set(parent)),
                (None, None) => unreachable!(),
            };
            app.configure_sets(
                Update,
                set.run_if(move |this: Res<ThisWorld>| {
                    let f = world_time::function(address).expect("modelled");
                    world_time::step_runs(&this.0, f, i)
                }),
            );
            if i + 1 < n {
                app.configure_sets(
                    Update,
                    WorldSet::Sub(address, i).before(WorldSet::Sub(address, i + 1)),
                );
            }
        }
        // A nested function's sub-steps also follow its caller's sub-step
        // before the call and come before the one after it (as the player's
        // update's do), so a system ordered at one of them from outside the
        // sets keeps its place when they are empty.
        if let Some((_, before, after)) = around {
            if let Some(b) = before {
                app.configure_sets(Update, WorldSet::Sub(address, 0).after(b));
            }
            if let Some(a) = after {
                app.configure_sets(Update, WorldSet::Sub(address, n - 1).before(a));
            }
        }
    }
}

/// The frame's sets and [`begin_frame`].
pub struct FrameOrderPlugin;

impl Plugin for FrameOrderPlugin {
    fn build(&self, app: &mut App) {
        configure(app);
        crate::poll::add(app);
        app.add_systems(
            Update,
            begin_frame.in_set(FrameSet::Stage(Stage::FrameStart)),
        )
        // The controls' buffer: the last frame's poll back first, this
        // frame's at `Controls::Poll` (step 33).
        .add_systems(Update, crate::poll::hold_last_poll.in_set(ViewerSet::Input))
        .add_systems(Update, crate::poll::poll_controls.in_set(poll_set()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_swap_audio_precedes_process_lists_and_menu_holds_moves() {
        for threads in [1, 2, 8] {
            for menu in [false, true] {
                let mut app = App::new();
                configure_with(&mut app, threads);
                app.init_resource::<Ran>();
                let frame = FrameState {
                    threads,
                    in_menu_mode: menu,
                    ..FrameState::default()
                };
                app.insert_resource(ThisFrame(frame));
                app.insert_resource(ThisInterface(interface::from_frame(&frame, false)));
                // Reverse registration proves set order, including the nested
                // audio/process-list boundary, rather than insertion order.
                for (label, function, site) in [
                    ("moves", interface::UPDATE_PROCESS_LISTS, 0x0086_f905),
                    ("music", interface::AUDIO_UPDATE, 0x0086_f65f),
                    ("radio", interface::AUDIO_UPDATE, 0x0086_f657),
                    ("sounds", interface::AUDIO_UPDATE, 0x0086_f650),
                ] {
                    app.add_systems(Update, note(label).in_set(InterfaceSet::at(function, site)));
                }
                app.update();
                let want = if menu {
                    vec!["sounds", "radio", "music"]
                } else {
                    vec!["sounds", "radio", "music", "moves"]
                };
                assert_eq!(
                    app.world().resource::<Ran>().0,
                    want,
                    "threads={threads} menu={menu}"
                );
            }
        }
    }

    /// What ran, in order.
    #[derive(Resource, Default)]
    struct Ran(Vec<&'static str>);

    fn note(name: &'static str) -> impl FnMut(ResMut<Ran>) {
        move |mut ran: ResMut<Ran>| ran.0.push(name)
    }

    fn stage_name(stage: Stage) -> &'static str {
        match stage {
            Stage::FrameStart => "FrameStart",
            Stage::Player => "Player",
            Stage::Housekeeping => "Housekeeping",
            Stage::WorldAndTime => "WorldAndTime",
            Stage::InterfaceAndScene => "InterfaceAndScene",
            Stage::AiStart => "AiStart",
            Stage::Render => "Render",
            Stage::AiJoin => "AiJoin",
        }
    }

    /// An app with the sets and nothing else; the state is set by hand.
    fn frame_app(state: FrameState) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        configure(&mut app);
        app.insert_resource(ThisFrame(state)).init_resource::<Ran>();
        app
    }

    /// The configured order of the sets is `world::frame`'s stage order,
    /// with the viewer's sets ahead and after; and steps run in call order
    /// inside their stage. Systems are added in reverse so that only the
    /// sets can give the order.
    #[test]
    fn sets_run_in_the_frames_order() {
        let mut app = frame_app(FrameState::default());
        app.add_systems(Update, note("AfterFrame").in_set(ViewerSet::AfterFrame));
        for stage in frame::stages().into_iter().rev() {
            app.add_systems(
                Update,
                note(stage_name(stage)).in_set(FrameSet::Stage(stage)),
            );
        }
        app.add_systems(Update, note("Loading").in_set(ViewerSet::Loading))
            .add_systems(Update, note("Input").in_set(ViewerSet::Input));
        app.update();
        let mut want = vec!["Input", "Loading"];
        want.extend(frame::stages().into_iter().map(stage_name));
        want.push("AfterFrame");
        assert_eq!(app.world().resource::<Ran>().0, want);
        // The exe's stage order, as STEPS reaches the stages.
        let mut from_steps: Vec<Stage> = STEPS.iter().map(|s| s.stage).collect();
        from_steps.dedup();
        assert_eq!(frame::stages(), from_steps);

        // Steps: the tree manager (77) after the menu background (44)
        // after the timer (32), whatever order they're added in, and each
        // with its stage's other systems.
        let mut app = frame_app(FrameState::default());
        app.add_systems(
            Update,
            note("77").in_set(FrameSet::step(TREE_MANAGER_UPDATE)),
        )
        .add_systems(
            Update,
            note("44").in_set(FrameSet::step(HANDLE_MENU_BACKGROUND)),
        )
        .add_systems(Update, note("32").in_set(FrameSet::step(UPDATE_TIMER)))
        .add_systems(
            Update,
            note("Player").in_set(FrameSet::Stage(Stage::Player)),
        );
        app.update();
        assert_eq!(
            app.world().resource::<Ran>().0,
            ["Player", "32", "44", "77"]
        );
    }

    /// A system in a gated set doesn't run while its gate is closed: menu
    /// mode stops `Calendar::Update` (step 56, the world runs), not the rest
    /// of its stage; Tab and Alt held stop every stage after the first.
    #[test]
    fn gates_stop_their_sets() {
        let calendar = FrameSet::step(0x0086_7a40);
        let add = |app: &mut App| {
            app.add_systems(
                Update,
                note("start").in_set(FrameSet::Stage(Stage::FrameStart)),
            )
            .add_systems(Update, note("calendar").in_set(calendar))
            .add_systems(
                Update,
                note("world")
                    .in_set(FrameSet::Stage(Stage::WorldAndTime))
                    .after(calendar),
            )
            .add_systems(Update, note("after").in_set(ViewerSet::AfterFrame));
        };
        let game = viewer_frame_state(FrameInputs::default());

        let mut a = frame_app(game);
        add(&mut a);
        a.update();
        assert_eq!(
            a.world().resource::<Ran>().0,
            ["start", "calendar", "world", "after"]
        );

        let menu = viewer_frame_state(FrameInputs {
            menu_up: true,
            ..FrameInputs::default()
        });
        let mut a = frame_app(menu);
        add(&mut a);
        a.update();
        assert_eq!(a.world().resource::<Ran>().0, ["start", "world", "after"]);

        let held = viewer_frame_state(FrameInputs {
            alt_tab_held: true,
            ..FrameInputs::default()
        });
        let mut a = frame_app(held);
        add(&mut a);
        a.update();
        assert_eq!(a.world().resource::<Ran>().0, ["start", "after"]);
    }

    /// The fixed inputs: fader 1 hidden, the world not frozen, game mode's
    /// interface mode, two threads or more; V.A.T.S. playback reaches the
    /// field-of-view gate.
    #[test]
    fn fixed_inputs() {
        let s = viewer_frame_state(FrameInputs::default());
        assert!(!s.fader_visible && !s.world_frozen && !s.console_visible);
        assert_eq!(s.interface_mode, 1);
        assert!(s.threads >= 2);
        assert!(s.world_runs() && s.process_lists());
        let playback = viewer_frame_state(FrameInputs {
            vats_mode: world::vats::mode::PLAYBACK,
            ..FrameInputs::default()
        });
        assert!(!frame::Gate::NotVatsPlayback.holds(&playback));
        let sleeping = viewer_frame_state(FrameInputs {
            top_menu: Some(1012),
            ..FrameInputs::default()
        });
        assert!(frame::Gate::SleepWaitMenuTop.holds(&sleeping));
    }

    /// An app with the sets, the frame's state and the player's.
    fn player_app(player: PlayerState) -> App {
        let mut app = frame_app(FrameState::default());
        app.insert_resource(ThisPlayer(player));
        app
    }

    /// The player's sets run in the exe's order inside step 14: the calls of
    /// `0086f940` in call order, `PlayerCharacter::Update`'s sub-steps
    /// inside its call, in theirs; a system ordered at a sub-step from
    /// outside its set runs there. Systems are added in reverse.
    #[test]
    fn player_sets_run_in_the_exes_order() {
        let mut app = player_app(PlayerState::default());
        app.add_systems(
            Update,
            note("after step")
                .in_set(FrameSet::Stage(Stage::Player))
                .after(FrameSet::step(UPDATE_PLAYER_STEP)),
        )
        .add_systems(
            Update,
            note("animation").in_set(PlayerSet::at(OWN_VIEW_ANIMATION_AT)),
        )
        .add_systems(Update, note("move").in_set(PlayerSet::at(MOVE_AT)))
        .add_systems(Update, note("attack").in_set(PlayerSet::at(ATTACK_AT)))
        .add_systems(
            Update,
            note("look")
                .in_set(FrameSet::step(UPDATE_PLAYER_STEP))
                .after(PlayerSet::at(HEADING_AND_LOOKING_AT))
                .before(PlayerSet::after(HEADING_AND_LOOKING_AT)),
        )
        .add_systems(
            Update,
            note("heading").in_set(PlayerSet::at(HEADING_AND_LOOKING_AT)),
        )
        .add_systems(Update, note("cell").in_set(PlayerSet::call(0x0043_6aa0)))
        .add_systems(Update, note("request").in_set(PlayerSet::call(0x0093_bea0)))
        .add_systems(
            Update,
            note("before step")
                .in_set(FrameSet::Stage(Stage::Player))
                .before(FrameSet::step(UPDATE_PLAYER_STEP)),
        );
        app.update();
        assert_eq!(
            app.world().resource::<Ran>().0,
            [
                "before step",
                "request",
                "heading",
                "look",
                "attack",
                "move",
                "animation",
                "cell",
                "after step"
            ]
        );
    }

    /// The gates: in menu mode only the grenade hold of the player's update
    /// runs (`0086f968`, `0086f974`), with the fly camera only its update
    /// (`0086f98f`); a system ordered from outside the sets runs either way.
    #[test]
    fn player_gates_stop_their_sets() {
        let add = |app: &mut App| {
            app.add_systems(Update, note("grenade").in_set(PlayerSet::call(0x0094_81d0)))
                .add_systems(
                    Update,
                    note("fly").in_set(PlayerSet::call(UPDATE_FLY_CAMERA)),
                )
                .add_systems(Update, note("move").in_set(PlayerSet::at(MOVE_AT)))
                .add_systems(
                    Update,
                    note("look")
                        .in_set(FrameSet::step(UPDATE_PLAYER_STEP))
                        .after(PlayerSet::at(HEADING_AND_LOOKING_AT))
                        .before(PlayerSet::after(HEADING_AND_LOOKING_AT)),
                );
        };
        let run = |state: PlayerState| {
            let mut app = player_app(state);
            add(&mut app);
            app.update();
            app.world().resource::<Ran>().0.clone()
        };
        let menu = viewer_frame_state(FrameInputs {
            menu_up: true,
            ..FrameInputs::default()
        });
        assert_eq!(
            run(viewer_player_state(&FrameState::default(), true, false)),
            ["look", "move"]
        );
        assert_eq!(
            run(viewer_player_state(&menu, true, false)),
            ["grenade", "look"]
        );
        assert_eq!(
            run(viewer_player_state(&FrameState::default(), false, false)),
            ["fly", "look"]
        );
        // Menu mode wins over the fly camera.
        assert_eq!(
            run(viewer_player_state(&menu, false, false)),
            ["grenade", "look"]
        );
    }

    /// The player stage's fixed inputs: the free branch with nothing pending.
    #[test]
    fn player_fixed_inputs() {
        let s = viewer_player_state(&FrameState::default(), true, false);
        assert!(s.updates() && !s.controlled() && !s.position_request && s.has_3d);
        assert!(!s.fading && !s.forced_activation && !s.knocked_or_paralysed);
        assert!(!viewer_player_state(&FrameState::default(), false, false).updates());
        assert!(viewer_player_state(&FrameState::default(), true, true).in_dialogue);
    }

    /// An app with the sets, the frame's state and the world stage's.
    fn world_app(frame: FrameState, indoors: bool) -> App {
        let mut app = frame_app(frame);
        app.insert_resource(ThisWorld(viewer_world_state(&frame, indoors)));
        app
    }

    /// Adds the world stage's systems the way `main.rs` and `trees.rs`
    /// place them, in reverse.
    fn add_world_systems(app: &mut App) {
        let tree = TREE_MANAGER_UPDATE;
        let grid = UPDATE_CURRENT_GRID_CELL;
        app.add_systems(
            Update,
            note("grid query").in_set(WorldSet::at(UPDATE_CURRENT_GRID_CELL_STEP, 0x0086_fc28)),
        )
        .add_systems(
            Update,
            note("distant land")
                .in_set(FrameSet::step(UPDATE_CURRENT_GRID_CELL_STEP))
                .after(WorldSet::at(grid, TERRAIN_UPDATE_AT))
                .before(WorldSet::after(grid, TERRAIN_UPDATE_AT)),
        )
        .add_systems(
            Update,
            note("squares")
                .in_set(FrameSet::step(UPDATE_CURRENT_GRID_CELL_STEP))
                .after(WorldSet::at(grid, GRID_SET_CENTER_AT))
                .before(WorldSet::after(grid, GRID_SET_CENTER_AT)),
        )
        .add_systems(
            Update,
            note("sway")
                .in_set(FrameSet::step(tree))
                .after(WorldSet::at(tree, TREE_WIND_AT))
                .before(WorldSet::after(tree, TREE_WIND_AT)),
        )
        .add_systems(
            Update,
            note("wind").in_set(WorldSet::at(tree, TREE_WIND_AT)),
        )
        .add_systems(
            Update,
            note("doors")
                .in_set(FrameSet::Stage(Stage::WorldAndTime))
                .after(FrameSet::step(RUN_ACTOR_SCRIPTS))
                .before(FrameSet::after(RUN_ACTOR_SCRIPTS)),
        )
        .add_systems(
            Update,
            note("scripts").in_set(WorldSet::at(RUN_ACTOR_SCRIPTS, RUN_SCRIPT_AT)),
        )
        .add_systems(Update, note("calendar").in_set(FrameSet::step(0x0086_7a40)));
    }

    /// The world stage's sets run in the exe's order: the calendar (step 56),
    /// the scripts (step 60), what follows them, the tree manager's wind and
    /// the sway after it (step 77), the grid's squares and distant land at
    /// `TES::UpdateCurrentGridCell`'s sub-steps, then the rest of
    /// `Main::OnIdle_UpdateCurrentGridCell` (step 78).
    #[test]
    fn world_sets_run_in_the_exes_order() {
        let mut app = world_app(FrameState::default(), false);
        add_world_systems(&mut app);
        app.update();
        assert_eq!(
            app.world().resource::<Ran>().0,
            [
                "calendar",
                "scripts",
                "doors",
                "wind",
                "sway",
                "squares",
                "distant land",
                "grid query"
            ]
        );
    }

    /// The gates: in menu mode the calendar, the wind and the grid's update
    /// stop (`0086e946`, `006653a9`, `0086fbf1`); the scripts' step, the
    /// sway and the loaders ordered outside the gates run.
    #[test]
    fn world_gates_stop_their_sets() {
        let menu = viewer_frame_state(FrameInputs {
            menu_up: true,
            ..FrameInputs::default()
        });
        let mut app = world_app(menu, false);
        add_world_systems(&mut app);
        app.update();
        assert_eq!(
            app.world().resource::<Ran>().0,
            ["scripts", "doors", "sway", "squares", "distant land"]
        );
    }

    /// The world stage's fixed inputs: the frame's modes, indoors or out from
    /// the viewer, the rest at the normal case.
    #[test]
    fn world_fixed_inputs() {
        let game = viewer_frame_state(FrameInputs::default());
        let out = viewer_world_state(&game, false);
        assert!(out.world_runs && out.sky && !out.interior_loaded && out.scene_camera);
        assert_eq!(out.grid, world_time::GridMove::Inside);
        let inside = viewer_world_state(&game, true);
        assert!(inside.interior_loaded && !inside.sky);
        let menu = viewer_frame_state(FrameInputs {
            menu_up: true,
            ..FrameInputs::default()
        });
        let s = viewer_world_state(&menu, false);
        assert!(s.menu_flag && !s.world_runs);
    }

    /// An app with the sets for a thread count, the frame's state and the AI
    /// stage's (dialogue up or not, outdoors).
    fn ai_app(frame: FrameState, threads: i32, in_dialogue: bool) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        configure_with(&mut app, threads);
        let frame = FrameState { threads, ..frame };
        app.insert_resource(ThisFrame(frame))
            .insert_resource(ThisAi(viewer_ai_state(&frame, in_dialogue, false)))
            .init_resource::<Ran>();
        app
    }

    /// Adds systems the way `main.rs` and the plugins place the AI stage's,
    /// in reverse.
    fn add_ai_systems(app: &mut App, threads: i32) {
        let stage = FrameSet::Stage(Stage::AiStart);
        let cells = UPDATE_CELL_ANIMATIONS;
        app.add_systems(
            Update,
            note("offstage").in_set(FrameSet::step(POST_SWAP_PROCESS)),
        )
        .add_systems(
            Update,
            note("daylight")
                .in_set(stage)
                .after(AiSet::at(UPDATE_CELL_MAIN_THREAD, SKY_UPDATE_AT))
                .before(FrameSet::after(UPDATE_ANIMATIONS_AND_EFFECTS)),
        )
        .add_systems(
            Update,
            note("weather").in_set(AiSet::at(UPDATE_CELL_MAIN_THREAD, SKY_UPDATE_AT)),
        )
        .add_systems(
            Update,
            note("actor update").in_set(AiSet::Sub(ai_stage::ACTOR_UPDATE, 0)),
        )
        .add_systems(
            Update,
            note("havok")
                .in_set(AiSet::Call(cells))
                .after(AiSet::at(cells, HAVOK_LOCK_AT))
                .before(AiSet::at(cells, HAVOK_UNLOCK_AT)),
        )
        .add_systems(
            Update,
            note("hits")
                .in_set(stage)
                .after(AiSet::Call(ACTORS_MOVEMENT))
                .before(AiSet::next_with(ACTORS_MOVEMENT, threads)),
        )
        .add_systems(Update, note("bolts").in_set(AiSet::Call(ACTORS_MOVEMENT)))
        .add_systems(
            Update,
            note("move")
                .in_set(stage)
                .after(AiSet::prev_with(ACTORS_MOVEMENT, threads))
                .before(AiSet::Call(ACTORS_MOVEMENT)),
        )
        .add_systems(Update, note("pieces").in_set(AiSet::Call(RUN_ANIMATIONS)))
        .add_systems(
            Update,
            note("pose")
                .in_set(stage)
                .after(AiSet::Call(ACTOR_ANIMATION_UPDATES))
                .before(AiSet::next_with(ACTOR_ANIMATION_UPDATES, threads)),
        )
        .add_systems(
            Update,
            note("hud")
                .in_set(stage)
                .after(AiSet::Call(INTERFACE_IDLE))
                .before(AiSet::next_with(INTERFACE_IDLE, threads)),
        )
        .add_systems(
            Update,
            note("markers")
                .in_set(stage)
                .before(FrameSet::step(START_THREADS))
                .before(AiSet::Call(INTERFACE_IDLE)),
        );
    }

    /// The AI stage's sets run in the exe's order: with two threads the
    /// combined thread's (the interface idle, the animations, the cells'
    /// animations, the movement and its projectiles, the actor updates, the
    /// Havok step), with more the pair's (thread 2's Havok step before thread
    /// 1's actor updates, which wait only for its movement); then the main
    /// thread's sky, then the lower process levels after the render.
    #[test]
    fn ai_sets_run_in_the_exes_order() {
        let run = |threads: i32| {
            let mut app = ai_app(FrameState::default(), threads, false);
            add_ai_systems(&mut app, threads);
            app.update();
            app.world().resource::<Ran>().0.clone()
        };
        assert_eq!(
            run(2),
            [
                "markers",
                "hud",
                "pose",
                "pieces",
                "move",
                "bolts",
                "hits",
                "actor update",
                "havok",
                "weather",
                "daylight",
                "offstage"
            ]
        );
        assert_eq!(
            run(8),
            [
                "markers",
                "hud",
                "pose",
                "pieces",
                "move",
                "bolts",
                "hits",
                "havok",
                "actor update",
                "weather",
                "daylight",
                "offstage"
            ]
        );
    }

    /// The gates: in menu mode no AI work starts (`0086ec1d`-`0086ec74`), so
    /// the cells' animations, the projectiles, the actor updates and the
    /// Havok step stop, and the sky's update with them (`0086fc72`); the
    /// systems ordered at their calls from outside run. In the dialogue menu
    /// the sky goes on (`0086fc7d`).
    #[test]
    fn ai_gates_stop_their_sets() {
        let menu = viewer_frame_state(FrameInputs {
            menu_up: true,
            ..FrameInputs::default()
        });
        let ungated = [
            "markers", "hud", "pose", "move", "hits", "daylight", "offstage",
        ];
        for threads in [2, 8] {
            let mut app = ai_app(menu, threads, false);
            add_ai_systems(&mut app, threads);
            app.update();
            assert_eq!(app.world().resource::<Ran>().0, ungated);
            let mut app = ai_app(menu, threads, true);
            add_ai_systems(&mut app, threads);
            app.update();
            assert_eq!(
                app.world().resource::<Ran>().0,
                ["markers", "hud", "pose", "move", "hits", "weather", "daylight", "offstage"]
            );
        }
    }

    /// The AI stage's inputs: the frame's thread count and modes, dialogue
    /// and indoors from the viewer.
    #[test]
    fn ai_fixed_inputs() {
        let game = viewer_frame_state(FrameInputs::default());
        let s = viewer_ai_state(&game, false, true);
        assert!(s.threads >= 2 && s.interior_loaded && !s.menu_flag && s.obstacle_avoidance);
        assert!(s.animations_run());
        let menu = viewer_frame_state(FrameInputs {
            menu_up: true,
            ..FrameInputs::default()
        });
        assert!(!viewer_ai_state(&menu, false, false).animations_run());
        assert!(viewer_ai_state(&menu, true, false).animations_run());
        // The sets the viewer's systems use exist for this machine's count.
        let threads = thread_count();
        assert_eq!(
            AiSet::next_with(INTERFACE_IDLE, threads),
            AiSet::Call(0x0087_a6b0)
        );
        assert_eq!(
            AiSet::prev_with(ACTORS_MOVEMENT, threads),
            AiSet::Call(0x0096_c330)
        );
    }

    /// The controls' poll (step 33, `Controls::Poll`'s sub-step) comes after
    /// the player's step and before the menus at the interface idle, with
    /// the HUD after them, for both thread counts and in menu mode too
    /// (`world::frame::input`: the player reads the previous poll, the
    /// interface this frame's).
    #[test]
    fn the_poll_sits_between_the_player_and_the_menus() {
        use world::frame::input::{poll_seen, Poll, Reader};
        for threads in [2, 8] {
            for menu_up in [false, true] {
                let state = viewer_frame_state(FrameInputs {
                    menu_up,
                    ..FrameInputs::default()
                });
                let mut app = ai_app(state, threads, false);
                app.add_systems(
                    Update,
                    note("hud")
                        .in_set(FrameSet::Stage(Stage::AiStart))
                        .after(IdleSet)
                        .before(AiSet::next_with(INTERFACE_IDLE, threads)),
                )
                .add_systems(Update, note("menus").in_set(IdleSet))
                .add_systems(Update, note("poll").in_set(poll_set()))
                .add_systems(
                    Update,
                    note("player").in_set(FrameSet::step(UPDATE_PLAYER_STEP)),
                )
                .add_systems(Update, note("input").in_set(ViewerSet::Input));
                app.update();
                assert_eq!(
                    app.world().resource::<Ran>().0,
                    ["input", "player", "poll", "menus", "hud"]
                );
                let frame = app.world().resource::<ThisFrame>().0;
                assert_eq!(poll_seen(Reader::Player, &frame), Some(Poll::Previous));
                assert_eq!(poll_seen(Reader::InterfaceIdle, &frame), Some(Poll::This));
            }
        }
    }

    /// With the buffer's systems in their sets, a key pressed in one frame
    /// reaches the menus in that frame and the player's step in the next;
    /// a key the menus take never reaches the player.
    #[test]
    fn a_press_reaches_the_menus_then_the_player() {
        use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};

        #[derive(Resource, Default)]
        struct Seen(Vec<(&'static str, bool)>);
        #[derive(Resource)]
        struct MenusTake(bool);
        fn see(who: &'static str) -> impl FnMut(Res<ButtonInput<KeyCode>>, ResMut<Seen>) {
            move |keys, mut seen| seen.0.push((who, keys.just_pressed(KeyCode::KeyE)))
        }
        fn menus(take: Res<MenusTake>, mut keys: ResMut<ButtonInput<KeyCode>>) {
            if take.0 {
                keys.reset_all();
            }
        }

        let mut app = ai_app(FrameState::default(), thread_count(), false);
        app.init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<ButtonInput<MouseButton>>()
            .init_resource::<AccumulatedMouseMotion>()
            .init_resource::<AccumulatedMouseScroll>()
            .init_resource::<Seen>()
            .insert_resource(MenusTake(false));
        crate::poll::add(&mut app);
        app.add_systems(Update, (see("menus"), menus).chain().in_set(IdleSet))
            .add_systems(
                Update,
                see("player").in_set(FrameSet::step(UPDATE_PLAYER_STEP)),
            )
            .add_systems(Update, crate::poll::poll_controls.in_set(poll_set()))
            .add_systems(Update, crate::poll::hold_last_poll.in_set(ViewerSet::Input));
        let frame = |app: &mut App, press: bool| {
            app.world_mut().resource_mut::<Seen>().0.clear();
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.clear();
            if press {
                keys.press(KeyCode::KeyE);
            } else {
                keys.release(KeyCode::KeyE);
            }
            app.update();
            app.world().resource::<Seen>().0.clone()
        };
        assert_eq!(frame(&mut app, true), [("player", false), ("menus", true)]);
        assert_eq!(frame(&mut app, false), [("player", true), ("menus", false)]);
        app.insert_resource(MenusTake(true));
        assert_eq!(frame(&mut app, true), [("player", false), ("menus", true)]);
        app.insert_resource(MenusTake(false));
        assert_eq!(
            frame(&mut app, false),
            [("player", false), ("menus", false)]
        );
    }
}
