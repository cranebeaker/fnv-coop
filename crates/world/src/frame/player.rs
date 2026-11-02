//! The frame's player stage: the calls of `Main::OnIdle_UpdatePlayer`
//! (Xbox PDB; PC `0086f940`, step 14 of [`super::STEPS`]) and of
//! `PlayerCharacter::Update` (Xbox PDB; PC `0093e860`, 22.6 KB), which the
//! first calls through slot +0x2f8 of the player's vtable (`0108aa3c`, the
//! one `PlayerCharacter`'s constructor writes at `009381c6`; the slot holds
//! `0093e860` at `0108ad34`), in the exe's order, with the tests that decide
//! which of them run (docs/FRAME_SKELETON.md, "PR 4 result").
//!
//! [`UPDATE_PLAYER`] holds every call of `0086f940`: its 27 direct calls
//! (the depth-2 rows of `research/engine-map/frame.tsv` under it) and the
//! three calls through the player's vtable (slot +0x1d0, the player's 3D,
//! twice; slot +0x2f8, `PlayerCharacter::Update`). Each has its
//! [`CallGate`], read from the branches of `0086f940`.
//!
//! [`UPDATE`] holds the sub-steps of `PlayerCharacter::Update`: not each of
//! its 886 call instructions (most are queries: the controls' states,
//! settings, getters, vector arithmetic, sound handles), but the calls that
//! do the player's work, in the order of their call sites, each with its
//! [`UpdateGate`] (the paths through the function and the modes on them)
//! and, where the call depends on its own block's further tests (a control
//! pressed, a timer run out, the weapon's state), those tests' branches as
//! [`UpdateStep::own_tests`]. A step without own tests is called whenever
//! its gate holds; one with them may still not be.
//!
//! The paths through `0093e860`: the bookkeeping first (always), then a
//! forced activation ends it (`0093f639`); the player's fade
//! (`HighProcess::FadeUpdate`, `0093f931`) refreshes both views and the
//! camera and ends it; an AI-controlled or dead player (`0093fbe5`,
//! `0093fbfe`) takes the controlled branch and ends there; everyone else
//! takes the free branch (`009408e4`), the player's own control: movement,
//! the view, attacks, activation, then the animation, camera and combat
//! updates.
//!
//! The model takes one value of each input for the whole update. The
//! disassembly and the decompiler agree on every branch here (`0086f940`
//! in full; `0093e860` at each branch named). Where the compiler copied a
//! call into both arms of a test (`SetFirstPerson` with 1 or 0) the step
//! lists both call sites.
//!
//! The viewer orders its player systems by these sets
//! (`viewer/src/frame_order.rs`, Phase 1 PR 4).

// Translated from 0086f940 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 0093e860 (decompiled, FalloutNV.exe 1.4.0.525)

use super::Wiring;

/// Whom a call goes to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Callee {
    /// A direct call of this PC address.
    Direct(u32),
    /// A call through this byte offset of the player's vtable (`0108aa3c`).
    Player(u32),
}

impl Callee {
    /// The PC address of the function called (through the player's vtable
    /// for [`Callee::Player`]).
    pub fn address(self) -> u32 {
        match self {
            Callee::Direct(a) => a,
            Callee::Player(slot) => player_slot(slot),
        }
    }
}

/// What the player's vtable (`0108aa3c`) holds at the slots
/// `0086f940` calls: +0x1d0 `00950b60` (the player's 3D), +0x2f8
/// `0093e860` (`PlayerCharacter::Update`).
pub fn player_slot(slot: u32) -> u32 {
    match slot {
        GET_3D_SLOT => 0x0095_0b60,
        UPDATE_SLOT => UPDATE_ADDRESS,
        _ => 0,
    }
}

/// The player's vtable slot of its 3D.
pub const GET_3D_SLOT: u32 = 0x1d0;
/// The player's vtable slot of `PlayerCharacter::Update`.
pub const UPDATE_SLOT: u32 = 0x2f8;
/// `PlayerCharacter::Update` (Xbox PDB).
pub const UPDATE_ADDRESS: u32 = 0x0093_e860;
/// `Main::OnIdle_UpdatePlayer` (Xbox PDB).
pub const UPDATE_PLAYER_ADDRESS: u32 = 0x0086_f940;

/// The player stage's inputs. `Default` is a player in game mode, walking
/// (no fly camera), with its 3D, standing in its exterior cell's square, in
/// the free branch of `PlayerCharacter::Update` with nothing pending.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerState {
    /// `PlayerCharacter::HandlePositionPlayerRequest` (`0093bea0`) moved
    /// the player (a door, `MoveTo`, fast travel): `0086f940` returns.
    pub position_request: bool,
    /// Menu mode, `[011dea2b]`, as the frame's first read leaves it
    /// ([`super::FrameState::menu_flag`]).
    pub menu_flag: bool,
    /// `Interface::IsPipboyOpening` (`00709bc0`).
    pub pipboy_opening: bool,
    /// The fly camera, `Main` +6 (`[011dea0c]`+6): toggled by the console's
    /// `ToggleFlyCam` (`00961e30` through `00961f70`); `Main` +7, the frozen
    /// world, is set with it for `TFC` 1, 2 or 5.
    pub fly_camera: bool,
    /// The player's 3D (slot +0x1d0, `00950b60`) exists.
    pub has_3d: bool,
    /// The player has a parent cell (`008d6f30`).
    pub in_cell: bool,
    /// That cell is an interior (`00425fd0`: its byte +0x24 bit 0).
    pub interior: bool,
    /// The player's position is outside its exterior cell's square
    /// (`00550200` false: x >> 12 or y >> 12 isn't the cell's X or Y).
    pub left_cell: bool,
    /// `TESDataHandler::GetCellFromWorldCoord` (`00461bc0`) found the cell
    /// under the player.
    pub grid_cell_found: bool,
    /// That cell's state (`00450fd0`; `00450fb0` asks for 3, `00450ff0`
    /// for 6).
    pub grid_cell_state: u32,
    /// `TES` +0x51 or +0x52 (`00451530`).
    pub cell_tests: bool,
    /// `BSShaderManager::GetAccumulator` (`00b4f5c0`) answers one.
    pub accumulator: bool,
    /// `[011f21d0]` at the start of the update: V.A.T.S. ended and the view
    /// it left is to be restored.
    pub vats_ended: bool,
    /// `00944320` answers a reference to activate (a forced activation).
    pub forced_activation: bool,
    /// The fade test is skipped: the view key's flags (`[011e07b8]` or
    /// `[011e07c1]`) are set and the process's slot +0x610 answers 0.
    pub fade_test_skipped: bool,
    /// `HighProcess::FadeUpdate` (`008fec10`) answers true.
    pub fading: bool,
    /// The process's slot +0x40c answers non-zero, or the player's slot
    /// +0x234 (`008be4f0`: the cached paralysis above 0) answers true.
    pub knocked_or_paralysed: bool,
    /// `b3rdPerson` (+0x64a).
    pub third_person: bool,
    /// One of `bAiControlledToPos`, `bAiControlledFromPos`,
    /// `bAiControlledActivate`, `bAiControlledPackage` (`0093a740`).
    pub ai_controlled: bool,
    /// The player's slot +0x22c with 0 (`008844f0`): the life state
    /// (`004f8960`) is 1, 2 or 6.
    pub dead: bool,
    /// The AI-control counter `[011e0798]` is past its limit
    /// (`01015a38`) while the sit-sleep state (slot +0x214) is 0 or 4.
    pub control_time_out: bool,
    /// `Interface::InDialog` (`007050d0`), asked at `00943748`.
    pub in_dialogue: bool,
    /// `[011f21d0]` set again during the update, tested at `0094381c`.
    pub vats_ending: bool,
    /// The process's slot +0x6b8 answers a muzzle flash.
    pub muzzle_flash: bool,
}

impl Default for PlayerState {
    fn default() -> PlayerState {
        PlayerState {
            position_request: false,
            menu_flag: false,
            pipboy_opening: false,
            fly_camera: false,
            has_3d: true,
            in_cell: true,
            interior: false,
            left_cell: false,
            grid_cell_found: true,
            grid_cell_state: 0,
            cell_tests: false,
            accumulator: true,
            vats_ended: false,
            forced_activation: false,
            fade_test_skipped: false,
            fading: false,
            knocked_or_paralysed: false,
            third_person: false,
            ai_controlled: false,
            dead: false,
            control_time_out: false,
            in_dialogue: false,
            vats_ending: false,
            muzzle_flash: false,
        }
    }
}

/// A grid cell's state `00450fb0` asks for.
pub const GRID_CELL_STATE_3: u32 = 3;
/// A grid cell's state `00450ff0` asks for.
pub const GRID_CELL_STATE_6: u32 = 6;

impl PlayerState {
    /// The menu branch: menu mode and the Pip-Boy not opening
    /// (`0086f968`, `0086f974`).
    pub fn menu_branch(&self) -> bool {
        self.menu_flag && !self.pipboy_opening
    }

    /// `PlayerCharacter::Update` is called (`0086f959`, `0086f968`,
    /// `0086f974`, `0086f98f`, `0086f9e9`).
    pub fn updates(&self) -> bool {
        CallGate::Update.holds(self)
    }

    /// The controlled branch of `0093e860`: AI-controlled or dead.
    pub fn controlled(&self) -> bool {
        self.ai_controlled || self.dead
    }
}

/// What `0086f940` tests before a call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CallGate {
    /// No test.
    Always,
    /// No position request (`0086f959`); `IsPipboyOpening` is asked in menu
    /// mode (`0086f968`).
    MenuFlag,
    /// The menu branch: no request, menu mode, the Pip-Boy not opening
    /// (`0086f968`, `0086f974`). It ends the update.
    Menu,
    /// The fly camera's branch asks for the 3D: no request, not the menu
    /// branch, the fly camera on (`0086f98f`).
    FlyCamera,
    /// `PlayerCharacter::UpdateFlyCamera` with the frame time and the frozen
    /// world: as `FlyCamera`, and the 3D exists (`0086f9a9`). The branch ends
    /// the update.
    FlyCamera3d,
    /// The player's own update path: no request, not the menu branch, the fly
    /// camera off (`0086f98f`).
    Walking,
    /// `PlayerCharacter::Update` with the frame time times
    /// `VATS::GetPlayerUpdateMult`: as `Walking`, and the 3D exists
    /// (`0086f9e9`).
    Update,
    /// As `Walking`, and the player has a cell (`0086fa72`).
    InCell,
    /// As `InCell`, and it isn't an interior (`0086fa85`).
    Exterior,
    /// As `Exterior`, and the player is outside its cell's square
    /// (`0086fa9c`).
    LeftCell,
    /// As `LeftCell`, and the cell under it was found (`0086facd`).
    GridCellFound,
    /// `00450ff0`: as `GridCellFound`, and its state isn't 3 (`0086fae0`).
    GridCellNot3,
    /// `TES::UpdateCurrentGridCell` and `00451530`: as `GridCellFound`, and
    /// its state is neither 3 nor 6 (`0086fae0`, `0086faef`).
    MoveGridCell,
    /// `TES::ShowLoadingMenu(0, 0, 0)`: as `MoveGridCell`, and `00451530`
    /// held (`0086fb12`).
    CellTestsLoading,
    /// `BSShaderAccumulator::ClearAllBoundVolumes`: as `GridCellFound`, and
    /// there is an accumulator (`0086fb81`).
    Accumulator,
}

impl CallGate {
    /// Whether the call is reached.
    pub fn holds(self, s: &PlayerState) -> bool {
        let walking = !s.position_request && !s.menu_branch() && !s.fly_camera;
        let in_cell = walking && s.in_cell;
        let exterior = in_cell && !s.interior;
        let left = exterior && s.left_cell;
        let found = left && s.grid_cell_found;
        let not_3 = found && s.grid_cell_state != GRID_CELL_STATE_3;
        let move_cell = not_3 && s.grid_cell_state != GRID_CELL_STATE_6;
        match self {
            CallGate::Always => true,
            CallGate::MenuFlag => !s.position_request && s.menu_flag,
            CallGate::Menu => !s.position_request && s.menu_branch(),
            CallGate::FlyCamera => !s.position_request && !s.menu_branch() && s.fly_camera,
            CallGate::FlyCamera3d => {
                !s.position_request && !s.menu_branch() && s.fly_camera && s.has_3d
            }
            CallGate::Walking => walking,
            CallGate::Update => walking && s.has_3d,
            CallGate::InCell => in_cell,
            CallGate::Exterior => exterior,
            CallGate::LeftCell => left,
            CallGate::GridCellFound => found,
            CallGate::GridCellNot3 => not_3,
            CallGate::MoveGridCell => move_cell,
            CallGate::CellTestsLoading => move_cell && s.cell_tests,
            CallGate::Accumulator => found && s.accumulator,
        }
    }

    /// The branch instructions of `0086f940` that make up the test.
    pub fn branches(self) -> &'static [u32] {
        const WALKING: [u32; 4] = [0x0086_f959, 0x0086_f968, 0x0086_f974, 0x0086_f98f];
        match self {
            CallGate::Always => &[],
            CallGate::MenuFlag => &[0x0086_f959, 0x0086_f968],
            CallGate::Menu => &[0x0086_f959, 0x0086_f968, 0x0086_f974],
            CallGate::FlyCamera => &WALKING,
            CallGate::FlyCamera3d => &[
                0x0086_f959,
                0x0086_f968,
                0x0086_f974,
                0x0086_f98f,
                0x0086_f9a9,
            ],
            CallGate::Walking => &WALKING,
            CallGate::Update => &[
                0x0086_f959,
                0x0086_f968,
                0x0086_f974,
                0x0086_f98f,
                0x0086_f9e9,
            ],
            CallGate::InCell => &[0x0086_f98f, 0x0086_fa72],
            CallGate::Exterior => &[0x0086_f98f, 0x0086_fa72, 0x0086_fa85],
            CallGate::LeftCell => &[0x0086_fa72, 0x0086_fa85, 0x0086_fa9c],
            CallGate::GridCellFound => &[0x0086_fa9c, 0x0086_facd],
            CallGate::GridCellNot3 => &[0x0086_facd, 0x0086_fae0],
            CallGate::MoveGridCell => &[0x0086_facd, 0x0086_fae0, 0x0086_faef],
            CallGate::CellTestsLoading => &[0x0086_fae0, 0x0086_faef, 0x0086_fb12],
            CallGate::Accumulator => &[0x0086_facd, 0x0086_fb81],
        }
    }
}

/// One call of `0086f940`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerCall {
    pub callee: Callee,
    /// The call instruction.
    pub site: u32,
    /// The Xbox PDB name `frame.tsv` gives (none where the engine map has
    /// none).
    pub name: Option<&'static str>,
    pub gate: CallGate,
    pub wiring: Wiring,
}

const fn call(
    callee: Callee,
    site: u32,
    name: Option<&'static str>,
    gate: CallGate,
    wiring: Wiring,
) -> PlayerCall {
    PlayerCall {
        callee,
        site,
        name,
        gate,
        wiring,
    }
}

use CallGate as C;
use Callee::{Direct, Player};

/// The calls of `Main::OnIdle_UpdatePlayer` (`0086f940`), in order.
pub const UPDATE_PLAYER: [PlayerCall; 30] = [
    call(Direct(0x0093_bea0), 0x0086_f94f, Some("PlayerCharacter::HandlePositionPlayerRequest"), C::Always, Wiring::Open),
    call(Direct(0x0070_9bc0), 0x0086_f96a, Some("Interface::IsPipboyOpening"), C::MenuFlag, Wiring::Open),
    call(Direct(0x0094_81d0), 0x0086_f97c, Some("PlayerCharacter::ForceGrenadeHold"), C::Menu, Wiring::Open),
    call(Player(GET_3D_SLOT), 0x0086_f9a5, None, C::FlyCamera, Wiring::Open),
    call(Direct(0x0084_d030), 0x0086_f9b8, None, C::FlyCamera3d, Wiring::Open),
    call(Direct(0x0094_66d0), 0x0086_f9c7, Some("PlayerCharacter::UpdateFlyCamera"), C::FlyCamera3d, Wiring::Partial("viewer: fly_camera, the viewer's free camera (` key), with the world running as TFC without an argument leaves it")),
    call(Player(GET_3D_SLOT), 0x0086_f9e5, None, C::Walking, Wiring::Open),
    call(Direct(0x0084_d030), 0x0086_f9fc, None, C::Update, Wiring::Open),
    call(Direct(0x009c_8cc0), 0x0086_fa09, Some("VATS::GetPlayerUpdateMult"), C::Update, Wiring::Open),
    call(Player(UPDATE_SLOT), 0x0086_fa2f, Some("PlayerCharacter::Update"), C::Update, Wiring::Partial("UPDATE, its sub-steps")),
    call(Direct(0x008d_6f30), 0x0086_fa4a, None, C::Walking, Wiring::Open),
    call(Direct(0x0043_6aa0), 0x0086_fa58, None, C::Walking, Wiring::Open),
    call(Direct(0x0042_5fd0), 0x0086_fa7b, None, C::InCell, Wiring::Open),
    call(Direct(0x0055_0200), 0x0086_fa92, None, C::Exterior, Wiring::Open),
    call(Direct(0x0054_ddd0), 0x0086_faa7, Some("TESObjectCELL::GetWorldSpace"), C::LeftCell, Wiring::Open),
    call(Direct(0x0046_1bc0), 0x0086_fac1, Some("TESDataHandler::GetCellFromWorldCoord"), C::LeftCell, Wiring::Open),
    call(Direct(0x0045_0fb0), 0x0086_fad6, None, C::GridCellFound, Wiring::Open),
    call(Direct(0x0045_0ff0), 0x0086_fae5, None, C::GridCellNot3, Wiring::Open),
    call(Direct(0x0045_2580), 0x0086_fafd, Some("TES::UpdateCurrentGridCell"), C::MoveGridCell, Wiring::Open),
    call(Direct(0x0045_1530), 0x0086_fb08, None, C::MoveGridCell, Wiring::Open),
    call(Direct(0x0045_7d70), 0x0086_fb20, Some("TES::ShowLoadingMenu"), C::CellTestsLoading, Wiring::Open),
    call(Direct(0x0086_fba0), 0x0086_fb27, None, C::GridCellFound, Wiring::Open),
    call(Direct(0x0086_fbc0), 0x0086_fb37, None, C::GridCellFound, Wiring::Open),
    call(Direct(0x0054_8230), 0x0086_fb48, Some("TESObjectCELL::AddReference"), C::GridCellFound, Wiring::Open),
    call(Direct(0x0054_7590), 0x0086_fb50, Some("TESObjectCELL::GetAcousticSpace"), C::GridCellFound, Wiring::Open),
    call(Direct(0x0086_fbb0), 0x0086_fb56, None, C::GridCellFound, Wiring::Open),
    call(Direct(0x0086_fbc0), 0x0086_fb66, None, C::GridCellFound, Wiring::Open),
    call(Direct(0x0086_fba0), 0x0086_fb6d, None, C::GridCellFound, Wiring::Open),
    call(Direct(0x00b4_f5c0), 0x0086_fb75, Some("BSShaderManager::GetAccumulator"), C::GridCellFound, Wiring::Open),
    call(Direct(0x00b6_55b0), 0x0086_fb86, Some("BSShaderAccumulator::ClearAllBoundVolumes"), C::Accumulator, Wiring::Open),
];

/// What `0093e860` tests before a sub-step: the path it is on and the mode
/// tests on that path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UpdateGate {
    /// No test.
    Always,
    /// `[011f21d0]` set (`0093e8ca`): back to the view V.A.T.S. left, first
    /// or third person by `[011f21d1]`.
    VatsEnded,
    /// A forced activation (`0093f639`): activated, then the update ends
    /// (`0093f679`).
    ForcedActivation,
    /// No forced activation (`0093f639` taken).
    Main,
    /// `HighProcess::FadeUpdate` is asked: as `Main`, unless the view key's
    /// flags are set (`0093f8f2`, `0093f8fd`) and the process's slot +0x610
    /// answers 0 (`0093f917`).
    FadeAsked,
    /// It answered true (`0093f931`): both views' animations and the camera
    /// refreshed, then the update ends.
    Fading,
    /// Knocked down or paralysed and not fading (`0093fb85`, `0093fb9c`):
    /// `VATS::QuitVATSPlayback`.
    KnockedOrParalysed,
    /// As `KnockedOrParalysed`, in first person (`0093fbcc`):
    /// `PlayerCharacter::ForceTemp3rdPerson(1)`.
    KnockedFirstPerson,
    /// The controlled branch with the AI-control counter past its limit
    /// (`0093fbe5`, `0093fbfe`, `0093fc6a`, `0093fc82`, `0093fc97`): out of
    /// the furniture, and the update ends (`0093fd67`).
    ControlTimeOut,
    /// The controlled branch (`0093fbe5`: AI-controlled, or `0093fbfe`:
    /// dead) without the time-out.
    Controlled,
    /// As `Controlled`, and not dead (`009403d9`: slot +0x22c(0),
    /// `009403f4`: slot +0x2e8, life state 1). The branch ends at
    /// `009408c2`.
    ControlledAlive,
    /// The free branch (`009408e4`): not fading, neither AI-controlled nor
    /// dead (`0093fbe5`, `0093fbfe`).
    Free,
    /// `Actor::UpdateMagic` on the free branch: the same two tests as
    /// `ControlledAlive` (`009436fb`, `00943712`), which hold there.
    FreeMagic,
    /// As `Free`, and not in dialogue (`00943752`).
    FreeNotInDialogue,
    /// As `Free`, and `[011f21d0]` not set again (`0094381c`).
    FreeCamera,
    /// As `Free`, and the process has a muzzle flash (slot +0x6b8,
    /// `0094387d`).
    FreeMuzzleFlash,
}

impl UpdateGate {
    /// Whether the sub-step is reached, given that `PlayerCharacter::Update`
    /// is called.
    pub fn holds(self, s: &PlayerState) -> bool {
        let main = !s.forced_activation;
        let fade_asked = main && !s.fade_test_skipped;
        let fading = fade_asked && s.fading;
        let not_fading = main && !fading;
        let controlled = not_fading && s.controlled();
        let free = not_fading && !s.controlled();
        match self {
            UpdateGate::Always => true,
            UpdateGate::VatsEnded => s.vats_ended,
            UpdateGate::ForcedActivation => s.forced_activation,
            UpdateGate::Main => main,
            UpdateGate::FadeAsked => fade_asked,
            UpdateGate::Fading => fading,
            UpdateGate::KnockedOrParalysed => not_fading && s.knocked_or_paralysed,
            UpdateGate::KnockedFirstPerson => {
                not_fading && s.knocked_or_paralysed && !s.third_person
            }
            UpdateGate::ControlTimeOut => controlled && s.control_time_out,
            UpdateGate::Controlled => controlled && !s.control_time_out,
            UpdateGate::ControlledAlive => controlled && !s.control_time_out && !s.dead,
            UpdateGate::Free => free,
            UpdateGate::FreeMagic => free && !s.dead,
            UpdateGate::FreeNotInDialogue => free && !s.in_dialogue,
            UpdateGate::FreeCamera => free && !s.vats_ending,
            UpdateGate::FreeMuzzleFlash => free && s.muzzle_flash,
        }
    }

    /// The branch instructions of `0093e860` that make up the test.
    pub fn branches(self) -> &'static [u32] {
        const MAIN: u32 = 0x0093_f639;
        const FADE: [u32; 5] = [MAIN, 0x0093_f8f2, 0x0093_f8fd, 0x0093_f917, 0x0093_f931];
        const CONTROLLED: [u32; 7] = [
            MAIN,
            0x0093_f931,
            0x0093_fbe5,
            0x0093_fbfe,
            0x0093_fc6a,
            0x0093_fc82,
            0x0093_fc97,
        ];
        const FREE: [u32; 4] = [MAIN, 0x0093_f931, 0x0093_fbe5, 0x0093_fbfe];
        match self {
            UpdateGate::Always => &[],
            UpdateGate::VatsEnded => &[0x0093_e8ca],
            UpdateGate::ForcedActivation | UpdateGate::Main => &[MAIN],
            UpdateGate::FadeAsked => &[MAIN, 0x0093_f8f2, 0x0093_f8fd, 0x0093_f917],
            UpdateGate::Fading => &FADE,
            UpdateGate::KnockedOrParalysed => &[MAIN, 0x0093_f931, 0x0093_fb85, 0x0093_fb9c],
            UpdateGate::KnockedFirstPerson => {
                &[MAIN, 0x0093_f931, 0x0093_fb85, 0x0093_fb9c, 0x0093_fbcc]
            }
            UpdateGate::ControlTimeOut | UpdateGate::Controlled => &CONTROLLED,
            UpdateGate::ControlledAlive => &[
                MAIN,
                0x0093_f931,
                0x0093_fbe5,
                0x0093_fbfe,
                0x0093_fc6a,
                0x0093_fc82,
                0x0093_fc97,
                0x0094_03d9,
                0x0094_03f4,
            ],
            UpdateGate::Free => &FREE,
            UpdateGate::FreeMagic => &[
                MAIN,
                0x0093_f931,
                0x0093_fbe5,
                0x0093_fbfe,
                0x0094_36fb,
                0x0094_3712,
            ],
            UpdateGate::FreeNotInDialogue => {
                &[MAIN, 0x0093_f931, 0x0093_fbe5, 0x0093_fbfe, 0x0094_3752]
            }
            UpdateGate::FreeCamera => &[MAIN, 0x0093_f931, 0x0093_fbe5, 0x0093_fbfe, 0x0094_381c],
            UpdateGate::FreeMuzzleFlash => {
                &[MAIN, 0x0093_f931, 0x0093_fbe5, 0x0093_fbfe, 0x0094_387d]
            }
        }
    }
}

/// One sub-step of `PlayerCharacter::Update`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateStep {
    /// The function called.
    pub address: u32,
    /// Its call instructions in `0093e860` (two where the compiler copied
    /// the call into both arms of a test, or where the two views take one
    /// site each).
    pub sites: &'static [u32],
    /// Its Xbox PDB name, as the engine map gives it (none where it has
    /// none).
    pub name: Option<&'static str>,
    pub gate: UpdateGate,
    /// The branches of the step's own block that decide whether the call
    /// is made once the gate holds (a control pressed, a timer run out,
    /// the weapon's state); not modelled. Empty: the call is made whenever
    /// the gate holds.
    pub own_tests: &'static [u32],
    pub wiring: Wiring,
}

const fn sub(
    address: u32,
    sites: &'static [u32],
    name: Option<&'static str>,
    gate: UpdateGate,
    own_tests: &'static [u32],
    wiring: Wiring,
) -> UpdateStep {
    UpdateStep {
        address,
        sites,
        name,
        gate,
        own_tests,
        wiring,
    }
}

use UpdateGate as G;

/// The sub-steps of `PlayerCharacter::Update` (`0093e860`), in order.
pub const UPDATE: [UpdateStep; 58] = [
    // The bookkeeping: V.A.T.S.'s end and sounds, the kill camera's and the
    // slow motion's timers, the queued weapon, the crosshair's target, the
    // night vision and spotter perks, the action list, hardcore mode, the
    // warning timers.
    sub(0x0095_0110, &[0x0093_e8df, 0x0093_e8ee], Some("PlayerCharacter::SetFirstPerson"), G::VatsEnded, &[], Wiring::Open),
    sub(0x0077_2d10, &[0x0093_edba], Some("HUDMainMenu::SetTargetType"), G::Always, &[], Wiring::Open),
    sub(0x0096_9c30, &[0x0093_f374], Some("PlayerCharacter::UpdateHardcoreMode"), G::Always, &[0x0093_f36f], Wiring::Open),
    sub(0x0096_e570, &[0x0093_f5dc], Some("ProcessLists::SortActorsCloseToPlayer"), G::Always, &[0x0093_f5d5], Wiring::Open),
    sub(0x0094_dbe0, &[0x0093_f62a], Some("PlayerCharacter::ReturnToLastKnownGoodPosition"), G::Always, &[0x0093_f619], Wiring::Open),
    // A forced activation ends the update.
    sub(0x0057_3170, &[0x0093_f64f], Some("TESObjectREFR::Activate"), G::ForcedActivation, &[], Wiring::Open),
    sub(0x0051_9020, &[0x0093_f659], None, G::ForcedActivation, &[], Wiring::Open),
    // The process's flags, the sneak, swim and run timers, then the look.
    sub(0x00ca_1410, &[0x0093_f8c2], Some("bhkRagdollPenetrationUtil::Update"), G::Main, &[], Wiring::Open),
    sub(0x0094_45b0, &[0x0093_f8d9], Some("PlayerCharacter::UpdateHeadingAndLooking"), G::Main, &[], Wiring::Partial("viewer: look_around (the mouse look; ordered here, not under the gate)")),
    sub(0x008f_ec10, &[0x0093_f927], Some("HighProcess::FadeUpdate"), G::FadeAsked, &[], Wiring::Open),
    // Fading: both views and the camera, then the update ends.
    sub(0x0094_81d0, &[0x0093_f954], Some("PlayerCharacter::ForceGrenadeHold"), G::Fading, &[], Wiring::Open),
    sub(0x008d_3550, &[0x0093_f97d], None, G::Fading, &[], Wiring::Open),
    sub(0x0088_85e0, &[0x0093_f9a8], Some("Actor::UpdateAnimationMovement"), G::Fading, &[], Wiring::Open),
    sub(0x008d_3550, &[0x0093_f9d1], None, G::Fading, &[], Wiring::Open),
    sub(0x0088_85e0, &[0x0093_f9fc], Some("Actor::UpdateAnimationMovement"), G::Fading, &[], Wiring::Open),
    sub(0x0094_ae40, &[0x0093_fa08], Some("PlayerCharacter::UpdateCamera"), G::Fading, &[], Wiring::Open),
    // Knocked down or paralysed: out of V.A.T.S.'s playback, third person.
    sub(0x009c_8950, &[0x0093_fba7], Some("VATS::QuitVATSPlayback"), G::KnockedOrParalysed, &[], Wiring::Open),
    sub(0x0095_0340, &[0x0093_fbd3], Some("PlayerCharacter::ForceTemp3rdPerson"), G::KnockedFirstPerson, &[], Wiring::Open),
    // The controlled branch.
    sub(0x0088_d640, &[0x0093_fd3f], Some("Actor::GetOutofFurnitureQuick"), G::ControlTimeOut, &[], Wiring::Open),
    sub(0x0088_6360, &[0x0093_fea5], None, G::Controlled, &[0x0093_fe99], Wiring::Open),
    sub(0x009e_a570, &[0x0093_feb8], None, G::Controlled, &[], Wiring::Open),
    sub(0x0089_5110, &[0x0094_006c, 0x0094_00b0], Some("Actor::PickAnimations"), G::Controlled, &[], Wiring::Open),
    sub(0x008d_3550, &[0x0094_00bf], None, G::Controlled, &[], Wiring::Open),
    sub(0x0088_85e0, &[0x0094_00ea], Some("Actor::UpdateAnimationMovement"), G::Controlled, &[], Wiring::Open),
    sub(0x008d_3550, &[0x0094_0113], None, G::Controlled, &[], Wiring::Open),
    sub(0x0088_85e0, &[0x0094_013e], Some("Actor::UpdateAnimationMovement"), G::Controlled, &[], Wiring::Open),
    sub(0x0094_ae40, &[0x0094_014a], Some("PlayerCharacter::UpdateCamera"), G::Controlled, &[], Wiring::Open),
    sub(0x0054_a070, &[0x0094_0161], None, G::Controlled, &[], Wiring::Open),
    sub(0x00b5_d9f0, &[0x0094_0185], Some("ShadowSceneNode::UpdateObjectLighting"), G::Controlled, &[], Wiring::Open),
    sub(0x008c_3c40, &[0x0094_0401], Some("Actor::UpdateMagic"), G::ControlledAlive, &[], Wiring::Open),
    sub(0x0057_3170, &[0x0094_0706, 0x0094_0741, 0x0094_07b3], Some("TESObjectREFR::Activate"), G::ControlledAlive, &[0x0094_0414, 0x0094_0427, 0x0094_0437, 0x0094_0452, 0x0094_0465, 0x0094_065e], Wiring::Open),
    // The free branch: the player's controls.
    sub(0x0096_73d0, &[0x0094_0c78], Some("PlayerCharacter::UpdateMenuModeButton"), G::Free, &[], Wiring::Open),
    sub(0x0096_2350, &[0x0094_10dd], Some("PlayerCharacter::HavokActivateDroppedReference"), G::Free, &[], Wiring::Open),
    sub(0x0095_03d0, &[0x0094_1d83], Some("PlayerCharacter::UpdateTemp3rdPerson"), G::Free, &[], Wiring::Open),
    sub(0x0095_0530, &[0x0094_1d8b], Some("PlayerCharacter::UpdateTemp1stPerson"), G::Free, &[], Wiring::Open),
    sub(0x0094_8310, &[0x0094_20fc], None, G::Free, &[0x0094_1e80, 0x0094_1e97, 0x0094_1ea3, 0x0094_1eb0, 0x0094_1eca, 0x0094_1f0d, 0x0094_1f22, 0x0094_1f32, 0x0094_1f42, 0x0094_20eb, 0x0094_20f7], Wiring::Partial("viewer: combat::player_attack (the attack, and before it the Aim control at 00941f4f and the Ammo Swap control's 009462c0)")),
    sub(0x0089_5110, &[0x0094_26ae], Some("Actor::PickAnimations"), G::Free, &[], Wiring::Open),
    sub(0x009e_a570, &[0x0094_280b], None, G::Free, &[], Wiring::Partial("viewer: walk::walk (the movement, with the Sneak, Always Run and Auto Move controls read earlier in the branch)")),
    sub(0x0094_7b10, &[0x0094_2835], Some("PlayerCharacter::CheckBorderRegion"), G::Free, &[], Wiring::Open),
    sub(0x0095_0110, &[0x0094_2cc0, 0x0094_2dc9], Some("PlayerCharacter::SetFirstPerson"), G::Free, &[0x0094_2bf5, 0x0094_2c02, 0x0094_2c14, 0x0094_2c26, 0x0094_2c31, 0x0094_2c40, 0x0094_2c54, 0x0094_2c64, 0x0094_2c79, 0x0094_2c89, 0x0094_2c94, 0x0094_2caa, 0x0094_2d1b, 0x0094_2d26, 0x0094_2d35, 0x0094_2d74, 0x0094_2da6, 0x0094_2db1], Wiring::Open),
    sub(0x0057_3170, &[0x0094_3250, 0x0094_328e, 0x0094_3348], Some("TESObjectREFR::Activate"), G::Free, &[0x0094_2e4f, 0x0094_2e5e, 0x0094_2e71, 0x0094_2e81, 0x0094_2e9c, 0x0094_2eab, 0x0094_2ebe, 0x0094_320b, 0x0094_3228], Wiring::Open),
    sub(0x0095_0340, &[0x0094_360e], Some("PlayerCharacter::ForceTemp3rdPerson"), G::Free, &[0x0094_359d, 0x0094_35bc, 0x0094_35cd], Wiring::Open),
    sub(0x0095_f6c0, &[0x0094_363c], None, G::Free, &[], Wiring::Open),
    sub(0x008c_3c40, &[0x0094_371b], Some("Actor::UpdateMagic"), G::FreeMagic, &[], Wiring::Open),
    sub(0x0095_de30, &[0x0094_375e], None, G::FreeNotInDialogue, &[], Wiring::Open),
    // The two views' animations: `b3rdPerson` (+0x64a) is toggled before
    // each pair, so the first pair updates the view the player isn't in
    // (`PlayerCharacter::GetAnimation` with the toggled flag) and the second,
    // after toggling back, the one it is in.
    sub(0x008d_3550, &[0x0094_3787], None, G::Free, &[], Wiring::Open),
    sub(0x0088_85e0, &[0x0094_37b2], Some("Actor::UpdateAnimationMovement"), G::Free, &[], Wiring::Open),
    sub(0x008d_3550, &[0x0094_37db], None, G::Free, &[], Wiring::Open),
    sub(0x0088_85e0, &[0x0094_3806], Some("Actor::UpdateAnimationMovement"), G::Free, &[], Wiring::Partial("viewer: player_idle::animate (the first-person view's camera tracks; ordered here, not under the gate)")),
    sub(0x008b_a600, &[0x0094_380e], None, G::Free, &[], Wiring::Open),
    sub(0x0094_ae40, &[0x0094_3825], Some("PlayerCharacter::UpdateCamera"), G::FreeCamera, &[], Wiring::Open),
    sub(0x0054_a070, &[0x0094_383c], None, G::Free, &[], Wiring::Open),
    sub(0x00b5_d9f0, &[0x0094_3860], Some("ShadowSceneNode::UpdateObjectLighting"), G::Free, &[], Wiring::Open),
    sub(0x009b_b080, &[0x0094_38a2], Some("MuzzleFlash::Update"), G::FreeMuzzleFlash, &[], Wiring::Open),
    sub(0x0055_5c20, &[0x0094_38f6], None, G::Free, &[], Wiring::Open),
    // The level up, the reputation, the combat updates.
    sub(0x008d_5210, &[0x0094_39ea], Some("CharacterProgression::BeginLevelUp"), G::Free, &[0x0094_3984, 0x0094_399a, 0x0094_39ab, 0x0094_39b7, 0x0094_39c3, 0x0094_39cf, 0x0094_39de], Wiring::Open),
    sub(0x0094_44d0, &[0x0094_3a7e], Some("PlayerCharacter::UpdatePlayerCombat"), G::Free, &[], Wiring::Open),
    sub(0x0096_4260, &[0x0094_40a6], Some("PlayerCharacter::UpdateAutoAimActor"), G::Free, &[], Wiring::Open),
];

/// Whether call `i` of `0086f940` (an index into [`UPDATE_PLAYER`]) is
/// reached.
pub fn call_runs(s: &PlayerState, i: usize) -> bool {
    UPDATE_PLAYER[i].gate.holds(s)
}

/// The calls of `0086f940` reached with these inputs, as indices into
/// [`UPDATE_PLAYER`].
pub fn calls_run(s: &PlayerState) -> Vec<usize> {
    (0..UPDATE_PLAYER.len())
        .filter(|&i| call_runs(s, i))
        .collect()
}

/// Whether sub-step `i` of `PlayerCharacter::Update` (an index into
/// [`UPDATE`]) is reached: the update is called and the step's gate holds.
/// A step with [`UpdateStep::own_tests`] may still not make its call.
pub fn step_runs(s: &PlayerState, i: usize) -> bool {
    s.updates() && UPDATE[i].gate.holds(s)
}

/// The sub-steps reached with these inputs, as indices into [`UPDATE`].
pub fn steps_run(s: &PlayerState) -> Vec<usize> {
    (0..UPDATE.len()).filter(|&i| step_runs(s, i)).collect()
}

/// The index of the call of `PlayerCharacter::Update` in
/// [`UPDATE_PLAYER`].
pub fn update_call() -> usize {
    UPDATE_PLAYER
        .iter()
        .position(|c| c.callee == Callee::Player(UPDATE_SLOT))
        .expect("PlayerCharacter::Update is called")
}

/// The first call of `0086f940` that goes to `address`.
pub fn call_of(address: u32) -> Option<usize> {
    UPDATE_PLAYER
        .iter()
        .position(|c| c.callee.address() == address)
}

/// The first sub-step of `PlayerCharacter::Update` whose call site is
/// `site`.
pub fn step_at(site: u32) -> Option<usize> {
    UPDATE.iter().position(|s| s.sites.contains(&site))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `research/engine-map/frame.tsv`, the committed call tree.
    const FRAME_TSV: &str = include_str!("../../../../research/engine-map/frame.tsv");

    fn names(indices: &[usize]) -> Vec<u32> {
        indices
            .iter()
            .map(|&i| UPDATE_PLAYER[i].callee.address())
            .collect()
    }

    /// The direct calls are `frame.tsv`'s depth-2 rows under
    /// `Main::OnIdle_UpdatePlayer`, address and name, in order.
    #[test]
    fn direct_calls_are_frame_tsv_depth_2() {
        let mut rows = Vec::new();
        let mut inside = false;
        for line in FRAME_TSV.lines() {
            if line.starts_with('#') || line.starts_with("seq\t") {
                continue;
            }
            let f: Vec<&str> = line.split('\t').collect();
            match f[1] {
                "1" => inside = f[3] == "0086f940",
                "2" if inside => {
                    let address = u32::from_str_radix(f[3], 16).unwrap();
                    rows.push((address, Some(f[4]).filter(|n| !n.is_empty())));
                }
                _ => {}
            }
        }
        let direct: Vec<(u32, Option<&str>)> = UPDATE_PLAYER
            .iter()
            .filter_map(|c| match c.callee {
                Callee::Direct(a) => Some((a, c.name)),
                Callee::Player(_) => None,
            })
            .collect();
        assert_eq!(rows.len(), 27);
        assert_eq!(direct, rows);
    }

    #[test]
    fn sites_are_in_order() {
        assert!(UPDATE_PLAYER.windows(2).all(|w| w[0].site < w[1].site));
        assert!(UPDATE_PLAYER
            .iter()
            .all(|c| (0x0086_f940..0x0086_fb93).contains(&c.site)));
        let sites: Vec<u32> = UPDATE
            .iter()
            .flat_map(|s| s.sites.iter().copied())
            .collect();
        assert!(sites.windows(2).all(|w| w[0] < w[1]));
        assert!(sites.iter().all(|a| (0x0093_e860..0x0094_4210).contains(a)));
    }

    #[test]
    fn every_gate_names_its_branches() {
        for c in &UPDATE_PLAYER {
            let b = c.gate.branches();
            assert_eq!(b.is_empty(), c.gate == CallGate::Always, "{:08x}", c.site);
            assert!(b.iter().all(|a| (0x0086_f940..0x0086_fb93).contains(a)));
        }
        for s in &UPDATE {
            let b = s.gate.branches();
            assert_eq!(
                b.is_empty(),
                s.gate == UpdateGate::Always,
                "{:08x}",
                s.sites[0]
            );
            assert!(b
                .iter()
                .chain(s.own_tests)
                .all(|a| (0x0093_e860..0x0094_4210).contains(a)));
            // Each own test comes before the step's (last) call.
            assert!(s.own_tests.iter().all(|t| t < s.sites.last().unwrap()));
        }
    }

    #[test]
    fn the_update_is_slot_0x2f8() {
        assert_eq!(player_slot(UPDATE_SLOT), UPDATE_ADDRESS);
        assert_eq!(UPDATE_PLAYER[update_call()].site, 0x0086_fa2f);
        assert_eq!(call_of(UPDATE_ADDRESS), Some(update_call()));
        assert_eq!(call_of(0x0094_66d0), Some(5));
        assert_eq!(step_at(0x0094_280b), Some(37));
        assert_eq!(step_at(0x1234_5678), None);
        assert_eq!(super::super::step_of(UPDATE_PLAYER_ADDRESS), Some(13));
    }

    // `0086f959`: a position request ends the update after its call.
    #[test]
    fn position_request() {
        let s = PlayerState {
            position_request: true,
            ..PlayerState::default()
        };
        assert_eq!(calls_run(&s), [0]);
        assert!(steps_run(&s).is_empty());
    }

    // `0086f968`, `0086f974`: in menu mode only the grenade hold, unless the
    // Pip-Boy is opening.
    #[test]
    fn menu_mode() {
        let menu = PlayerState {
            menu_flag: true,
            ..PlayerState::default()
        };
        assert_eq!(
            names(&calls_run(&menu)),
            [0x0093_bea0, 0x0070_9bc0, 0x0094_81d0]
        );
        assert!(steps_run(&menu).is_empty());
        let opening = PlayerState {
            pipboy_opening: true,
            ..menu
        };
        assert!(opening.updates());
        assert!(!calls_run(&opening).contains(&2));
        // The Pip-Boy's opening alone isn't asked for outside menu mode.
        assert!(!call_runs(&PlayerState::default(), 1));
    }

    // `0086f98f`, `0086f9a9`: the fly camera instead of the update, and no
    // cell check.
    #[test]
    fn fly_camera() {
        let fly = PlayerState {
            fly_camera: true,
            ..PlayerState::default()
        };
        assert_eq!(
            names(&calls_run(&fly)),
            [0x0093_bea0, 0x0095_0b60, 0x0084_d030, 0x0094_66d0]
        );
        assert!(!fly.updates() && steps_run(&fly).is_empty());
        let no_3d = PlayerState {
            has_3d: false,
            ..fly
        };
        assert_eq!(names(&calls_run(&no_3d)), [0x0093_bea0, 0x0095_0b60]);
    }

    // `0086f9e9`: without its 3D the player isn't updated; its cell is still
    // checked.
    #[test]
    fn no_3d() {
        let s = PlayerState {
            has_3d: false,
            ..PlayerState::default()
        };
        assert!(!s.updates());
        assert_eq!(
            names(&calls_run(&s)),
            [
                0x0093_bea0,
                0x0095_0b60,
                0x008d_6f30,
                0x0043_6aa0,
                0x0042_5fd0,
                0x0055_0200
            ]
        );
    }

    // `0086fa72`-`0086fb81`: crossing into another exterior cell.
    #[test]
    fn crossing_cells() {
        let game = PlayerState::default();
        let update = [
            0x0093_bea0,
            0x0095_0b60,
            0x0084_d030,
            0x009c_8cc0,
            UPDATE_ADDRESS,
        ];
        let ask = [0x008d_6f30, 0x0043_6aa0];
        assert_eq!(
            names(&calls_run(&game)),
            [&update[..], &ask[..], &[0x0042_5fd0, 0x0055_0200]].concat()
        );
        let interior = PlayerState {
            interior: true,
            left_cell: true,
            ..game
        };
        assert_eq!(
            names(&calls_run(&interior)),
            [&update[..], &ask[..], &[0x0042_5fd0]].concat()
        );
        let nowhere = PlayerState {
            in_cell: false,
            ..game
        };
        assert_eq!(
            names(&calls_run(&nowhere)),
            [&update[..], &ask[..]].concat()
        );
        let left = PlayerState {
            left_cell: true,
            ..game
        };
        let attach = [
            0x0086_fba0,
            0x0086_fbc0,
            0x0054_8230,
            0x0054_7590,
            0x0086_fbb0,
            0x0086_fbc0,
            0x0086_fba0,
            0x00b4_f5c0,
            0x00b6_55b0,
        ];
        let found = [
            0x0042_5fd0,
            0x0055_0200,
            0x0054_ddd0,
            0x0046_1bc0,
            0x0045_0fb0,
            0x0045_0ff0,
        ];
        assert_eq!(
            names(&calls_run(&left)),
            [
                &update[..],
                &ask[..],
                &found[..],
                &[0x0045_2580, 0x0045_1530],
                &attach[..]
            ]
            .concat()
        );
        let loading = PlayerState {
            cell_tests: true,
            ..left
        };
        assert!(names(&calls_run(&loading)).contains(&0x0045_7d70));
        let state_3 = PlayerState {
            grid_cell_state: 3,
            ..left
        };
        assert_eq!(
            names(&calls_run(&state_3)),
            [&update[..], &ask[..], &found[..5], &attach[..]].concat()
        );
        let state_6 = PlayerState {
            grid_cell_state: 6,
            ..left
        };
        assert_eq!(
            names(&calls_run(&state_6)),
            [&update[..], &ask[..], &found[..], &attach[..]].concat()
        );
        let not_found = PlayerState {
            grid_cell_found: false,
            ..left
        };
        assert_eq!(
            names(&calls_run(&not_found)),
            [&update[..], &ask[..], &found[..4]].concat()
        );
        let no_accumulator = PlayerState {
            accumulator: false,
            ..left
        };
        assert!(!names(&calls_run(&no_accumulator)).contains(&0x00b6_55b0));
    }

    fn reached(s: &PlayerState) -> Vec<u32> {
        steps_run(s).iter().map(|&i| UPDATE[i].sites[0]).collect()
    }

    // `0093f639`: a forced activation ends the update after the bookkeeping.
    #[test]
    fn forced_activation() {
        let s = PlayerState {
            forced_activation: true,
            ..PlayerState::default()
        };
        assert_eq!(
            reached(&s),
            [
                0x0093_edba,
                0x0093_f374,
                0x0093_f5dc,
                0x0093_f62a,
                0x0093_f64f,
                0x0093_f659
            ]
        );
    }

    // `0093e8ca`: the view V.A.T.S. left.
    #[test]
    fn vats_ended() {
        let s = PlayerState {
            vats_ended: true,
            ..PlayerState::default()
        };
        assert_eq!(reached(&s)[0], 0x0093_e8df);
        assert_ne!(reached(&PlayerState::default())[0], 0x0093_e8df);
    }

    // `0093f8f2`-`0093f931`: the fade.
    #[test]
    fn fading() {
        let s = PlayerState {
            fading: true,
            ..PlayerState::default()
        };
        let r = reached(&s);
        assert_eq!(
            &r[r.len() - 7..],
            [
                0x0093_f927,
                0x0093_f954,
                0x0093_f97d,
                0x0093_f9a8,
                0x0093_f9d1,
                0x0093_f9fc,
                0x0093_fa08
            ]
        );
        let skipped = PlayerState {
            fade_test_skipped: true,
            ..s
        };
        let r = reached(&skipped);
        assert!(!r.contains(&0x0093_f927) && r.contains(&0x0094_0c78));
    }

    // `0093fb85`, `0093fb9c`, `0093fbcc`: knocked down or paralysed.
    #[test]
    fn knocked_down() {
        let s = PlayerState {
            knocked_or_paralysed: true,
            ..PlayerState::default()
        };
        assert!(reached(&s).contains(&0x0093_fba7) && reached(&s).contains(&0x0093_fbd3));
        let third = PlayerState {
            third_person: true,
            ..s
        };
        assert!(reached(&third).contains(&0x0093_fba7) && !reached(&third).contains(&0x0093_fbd3));
    }

    // `0093fbe5`, `0093fbfe`, `0093fc6a`-`0093fc97`, `009403d9`, `009403f4`:
    // the controlled branch, the dead player, the time-out.
    #[test]
    fn controlled_and_dead() {
        let ai = PlayerState {
            ai_controlled: true,
            ..PlayerState::default()
        };
        let r = reached(&ai);
        assert!(r.contains(&0x0094_0401) && r.contains(&0x0094_0706));
        assert!(!r.contains(&0x0094_0c78) && !r.contains(&0x0093_fd3f));
        let dead = PlayerState {
            dead: true,
            ..PlayerState::default()
        };
        let r = reached(&dead);
        assert!(r.contains(&0x0094_0185) && !r.contains(&0x0094_0401));
        assert!(!r.contains(&0x0094_0c78));
        let out = PlayerState {
            control_time_out: true,
            ..ai
        };
        assert_eq!(*reached(&out).last().unwrap(), 0x0093_fd3f);
        // The time-out means nothing on the free branch.
        assert_eq!(
            reached(&PlayerState {
                control_time_out: true,
                ..PlayerState::default()
            }),
            reached(&PlayerState::default())
        );
    }

    // `00943752`, `0094381c`, `0094387d`: dialogue, V.A.T.S. ending, the
    // muzzle flash.
    #[test]
    fn free_branch_tests() {
        let game = reached(&PlayerState::default());
        assert!(game.contains(&0x0094_375e) && game.contains(&0x0094_3825));
        assert!(!game.contains(&0x0094_38a2));
        let talking = reached(&PlayerState {
            in_dialogue: true,
            ..PlayerState::default()
        });
        assert!(!talking.contains(&0x0094_375e));
        let ending = reached(&PlayerState {
            vats_ending: true,
            ..PlayerState::default()
        });
        assert!(!ending.contains(&0x0094_3825));
        let flash = reached(&PlayerState {
            muzzle_flash: true,
            ..PlayerState::default()
        });
        assert!(flash.contains(&0x0094_38a2));
    }

    #[test]
    fn wiring() {
        let open = |w: Wiring| w.is_open();
        assert!(UPDATE_PLAYER.iter().all(|c| open(c.wiring)));
        assert!(UPDATE.iter().all(|s| open(s.wiring)));
        let partial: Vec<u32> = UPDATE
            .iter()
            .filter(|s| matches!(s.wiring, Wiring::Partial(_)))
            .map(|s| s.sites[0])
            .collect();
        assert_eq!(
            partial,
            [0x0093_f8d9, 0x0094_20fc, 0x0094_280b, 0x0094_3806]
        );
    }
}
