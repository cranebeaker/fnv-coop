//! The player's movement through the game's own translated code (Build 43).
//!
//! The viewer moves the player each frame. With this hook it first offers
//! the frame's move to the game's code in the host ([`PlayerMovement::step`]):
//! `PlayerMover::Update` (`009e9e50`, the mover's vtable +0x14, as
//! `PlayerCharacter::Update` calls it) → `Actor::Move` (`008a62b0`, player
//! vtable +0x250) → `MobileObject::Move` (`0092f260`) →
//! `bhkCharacterController::Move` (`00c73170`) → the character proxy, the
//! phantom's linear casts, the collision agents and shapes. When that run
//! reaches an address nobody has translated yet (or memory nobody made),
//! the miss is recorded once per address with its count and the nearest
//! caller entered by address ([`Miss`]), and the viewer moves the player
//! with its own `crates/physics` step for that frame, so the game stays
//! playable. [`PlayerMovement::missing_tsv`] is the list of those
//! addresses, for the translators.
//!
//! The game state the move needs is made in the game's memory by the
//! game's own code, in the order the game makes it ([`PlayerMovement::prepare`]):
//!
//! 1. At start-up ([`PlayerMovement::start_up`]), as `main` runs them after `NiInit`: `main`
//!    (`Main::Main` `0086c160`, kept at `011dea0c`), `scenegraph` (`Main::InitSceneGraph`
//!    `0086d590`) and `tes` (`InitTES`'s head: `TES::TES` `0044fb20`, which runs
//!    `InitHavokSystem` `00624bf0`, kept at `011dea10`).
//! 2. `player`: as `Main::InitTES` (`0086cf20`) makes it: `operator new`
//!    (0xE50 bytes), `PlayerCharacter::PlayerCharacter` (`00938180`), the
//!    pointer at `011dea3c`, form id 0x14 (vtable +0x128, `(0x14, 1)`); then,
//!    once the files are loaded, its base form (`NPC_` 0x7, loaded below) by
//!    `TESObjectREFR::SetObjectReference` (`00575690`), its position (vtable
//!    +0x2A8), its angle (`TESObjectREFR::SetAngleOnReference`, `0086d490`)
//!    and `InitItem` (vtable +0x88). `InitTES`'s `Actor::CastPermanentMagic`
//!    is left out (no movement state).
//! 3. `threads`: the AI linear task threads, as `main` makes them after
//!    `InitTES` (`0086b117`..`0086b12d`, `AILinearTaskThreadManager::CreateThreads`
//!    `008c7290`), run by the thread host (`engine::thread_host`). Each frame
//!    [`PlayerMovement::run_ai_threads`] starts and waits for them as
//!    `Main::OnIdle` does; AI thread 2 (`008c7f50`) steps the Havok world.
//! 4. `cell`: the player's cell, loaded as the game loads a form (below),
//!    its temporary references by `TESObjectCELL::LoadTempDataFromFile`
//!    (`00550500`, the file standing at the cell's record), their base forms
//!    first; then its 3D by `TESObjectCELL::Load3D` (`00545cf0`, which makes
//!    the cell's `bhkWorld` in `00552dc0`) and each reference's by
//!    `TESObjectREFR::Load3D` (`0056b2d0`), which adds its collision to the
//!    cell's world.
//! 5. `controller`: the player in the cell: `TESObjectREFR::SetParentCell`
//!    (`00575bb0`), `PlayerCharacter::Load3D` (`0094e1d0`) and
//!    `MobileObject::InitHavok` (`00930c70`, which constructs the
//!    `bhkCharacterController` and its proxy, `00c741e0`).
//!
//! A form is loaded as `TESDataHandler` loads one: `CreateFormOfType`
//! (`00465110`), `TESFile::SetOffset` at its record ([`GameFile::supply`]),
//! `TESDataHandler::LoadForm` (`004601d0`: the form's `Load`, vtable +0x20)
//! and `InitItem` (vtable +0x88). The game runs every form's `InitItem` once
//! all files are loaded; here each runs right after its own load (the order
//! is the bridge's, not the game's).
//!
//! Bridged (stand-ins for exe code, each named):
//! - The file reading below `TESFile` (`BSFile`, platform; `crate::game_file`):
//!   each plugin is opened by the game's `OpenTES` and header reader with
//!   an in-memory `BSFile` already in place, and its masters resolved by
//!   `TESFile::GenIndexTable` against a file list made here (standing in
//!   for the data handler's, [`GameFile::open_with_header`]). Compressed
//!   records are decompressed by the game's own reading (zlib included).
//! - The keys: the game's control handler (`PlayerCharacter::Update`'s
//!   `0093e860`) reads them through `OSInputGlobals` (DirectInput); here the
//!   viewer's keys give the mover's move bits, set with the game's own
//!   `PlayerMover::ForceMoveMode` (`009ea3e0`), and the camera's heading is
//!   set with `SetAngleOnReference`.
//! - Back in step: after a frame the viewer moved itself (or the first
//!   frame), the player is put where the viewer has it (vtable +0x2A8, the
//!   game's `SetPosition`), so the two paths start each frame together.
//! - Jumping is not connected (the jump control's path, player vtable
//!   +0x254, isn't wired): a frame with a jump goes to `crates/physics`.

use crate::game_file::GameFile;
use engine::{Engine, Ret};
use esm::{FormId, LoadOrder};
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::rc::Rc;

/// `operator new` (`00401000`).
const NEW: u32 = 0x0040_1000;
/// `Main::Main` (Xbox PDB, `0086c160`), the `Main` object's size and its global (`011dea0c`).
const MAIN_CONSTRUCT: u32 = 0x0086_c160;
const MAIN_SIZE: u32 = 0xa4;
const MAIN_OBJECT: u32 = 0x011d_ea0c;
/// The module handle `WinMain` gets (`main`'s first argument): the image base.
const IMAGE_BASE: u32 = 0x0040_0000;
/// `main` (Xbox PDB, `0086a850`): the game's `WinMain`, and the `SW_SHOWDEFAULT` the CRT's
/// start-up hands it last. `main` reads none of its last three words (`main.rs`), so the
/// previous instance and the command line are passed as 0.
const WIN_MAIN: u32 = 0x0086_a850;
const SW_SHOWDEFAULT: u32 = 10;
/// `main`'s part between `Main::Main` and `InitSceneGraph` (`0086afa4`..`0086b066`): the TLS
/// setting guard's enter and leave (`00404f00`/`00404f70`, a 4-byte object on `main`'s
/// stack; enter with 0xD, 1, the source name `010829c4` and line 0x9F8), the 8-byte object
/// `007005d0` makes into `011d8804`, `Error(0108295c)` (`0040fbe0`, empty), the renderer
/// (`0086d500` on `Main`), the byte `011f4509`, the getter `0043c4b0` and the three calls on
/// what it answers (`004a0370`; `0086bbd0` and `0086bae0`, each with a function and 0), then
/// `0086bf70`.
const SETTING_GUARD_ENTER: u32 = 0x0040_4f00;
const SETTING_GUARD_LEAVE: u32 = 0x0040_4f70;
const MAIN_SOURCE_NAME: u32 = 0x0108_29c4;
const MAKE_011D8804: u32 = 0x0070_05d0;
const OBJECT_011D8804: u32 = 0x011d_8804;
const ERROR_0040FBE0: u32 = 0x0040_fbe0;
const ERROR_TEXT_0108295C: u32 = 0x0108_295c;
const INIT_RENDERER: u32 = 0x0086_d500;
const RENDERER_MADE_011F4509: u32 = 0x011f_4509;
const GETTER_0043C4B0: u32 = 0x0043_c4b0;
const CALL_004A0370: u32 = 0x004a_0370;
const CALL_0086BBD0: u32 = 0x0086_bbd0;
const CALL_0086BAE0: u32 = 0x0086_bae0;
const CALL_0086BF70: u32 = 0x0086_bf70;
/// `Main::InitSceneGraph` (Xbox PDB, `0086d590`).
const INIT_SCENE_GRAPH: u32 = 0x0086_d590;
/// `InitTES`'s head (`0086cf81`..`0086d076`): `Sky::GetInstance` (`0046dd00`, kept at
/// `011dea20`), `Main::InitSky` (`00877730`), the getters `0086d470` (`011dea18`) and
/// `007a1480` (`011dea14`), the helper chain (`004505a0`, `00a89af0`, `0045bc00`, slot +0xC),
/// `TES::TES` (`0044fb20`, 0xC4 bytes, with `010724e8` and the water LOD node `011dea1c`) kept
/// at `011dea10`, then `00450c50` on it.
const SKY_GET_INSTANCE: u32 = 0x0046_dd00;
const SKY_OBJECT: u32 = 0x011d_ea20;
const INIT_SKY: u32 = 0x0087_7730;
const OBJECT_LOD_ROOT: u32 = 0x0086_d470;
const LOD_ROOT: u32 = 0x007a_1480;
const SKY_HELPER: u32 = 0x0045_05a0;
const HELPER_ATTACH: u32 = 0x00a8_9af0;
const HELPER_FIND: u32 = 0x0045_bc00;
const TES_SIZE: u32 = 0xc4;
const TES_CONSTRUCT: u32 = 0x0044_fb20;
const TES_DATA_PATH: u32 = 0x0107_24e8;
const WATER_LOD_NODE: u32 = 0x011d_ea1c;
const TES_OBJECT: u32 = 0x011d_ea10;
const TES_AFTER: u32 = 0x0045_0c50;
/// `PlayerCharacter::PlayerCharacter` (Xbox PDB, `00938180`) and its size.
const PLAYER_CONSTRUCT: u32 = 0x0093_8180;
const PLAYER_SIZE: u32 = 0xe50;
/// The player (`PlayerCharacter *`, `011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// The player's base form (`NPC_` 0x7, `InitTES`: `004839c0(7)`).
const PLAYER_BASE: FormId = FormId(7);
/// `TESForm` vtable slots: `SetFormID(id, 1)` (+0x128), `InitItem` (+0x88).
const SET_FORM_ID: u32 = 0x128;
const INIT_ITEM: u32 = 0x88;
/// `TESObjectREFR` vtable slots: `SetPosition(NiPoint3 *)` (+0x2A8), the
/// position (`GetPos`, +0x1F4).
const SET_POSITION: u32 = 0x2a8;
const GET_POSITION: u32 = 0x1f4;
/// `TESObjectREFR::SetObjectReference` (`00575690`), `SetAngleOnReference`
/// (`0086d490`), `SetParentCell` (`00575bb0`), `Load3D` (`0056b2d0`).
const SET_OBJECT_REFERENCE: u32 = 0x0057_5690;
const SET_ANGLE: u32 = 0x0086_d490;
const SET_PARENT_CELL: u32 = 0x0057_5bb0;
const REFERENCE_LOAD_3D: u32 = 0x0056_b2d0;
/// `PlayerCharacter::Load3D` (`0094e1d0`), `MobileObject::InitHavok`
/// (`00930c70`), `MobileObject::GetCharController` (`009306d0`).
const PLAYER_LOAD_3D: u32 = 0x0094_e1d0;
const INIT_HAVOK: u32 = 0x0093_0c70;
const GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
/// `TESDataHandler::CreateFormOfType` (`00465110`), `TESDataHandler::LoadForm`
/// (`004601d0`), `LookupFormByID` (`004839c0`).
const CREATE_FORM_OF_TYPE: u32 = 0x0046_5110;
const LOAD_FORM: u32 = 0x0046_01d0;
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `TESObjectCELL::LoadTempDataFromFile` (`00550500`), `Load3D` (`00545cf0`).
const CELL_LOAD_TEMP_DATA: u32 = 0x0055_0500;
const CELL_LOAD_3D: u32 = 0x0054_5cf0;
/// `Actor::pActorMover` (Xbox PDB) +0x190; the mover's `Update(seconds)`
/// (vtable +0x14); `PlayerMover::ForceMoveMode` (`009ea3e0`).
const ACTOR_MOVER: u32 = 0x190;
const MOVER_UPDATE: u32 = 0x14;
const FORCE_MOVE_MODE: u32 = 0x009e_a3e0;
/// A record header's size, and a group's.
const HEADER_SIZE: usize = 0x18;
/// `TESObjectREFR` vtable slots: `InitHavok` (+0x1C4, Xbox PDB name; PC
/// `005768b0`: adds the 3D's collision to the cell's `bhkWorld` through
/// `bhkWorld::AddObjects`, world vtable +0xD0) and `Get3D` (+0x1D0).
const REFERENCE_INIT_HAVOK: u32 = 0x1c4;
const GET_3D: u32 = 0x1d0;
/// `bhkWorld::SetDeltaTime(seconds, flag, mode 4)` (`00c66760`, as
/// `Main::OnIdle_UpdateTimer` `0086f260` calls it each frame), the flag
/// `00525420` answers (the byte at `011dea2a`).
const SET_DELTA_TIME: u32 = 0x00c6_6760;
const HAVOK_STEP_FLAG: u32 = 0x0052_5420;
/// The frame's seconds `0086f260` keeps for the AI threads' world step
/// (`011dea30`: the seconds times the float setting at `011c5724`, 0 in a
/// menu), read by `00453850`.
const FRAME_SECONDS: u32 = 0x011d_ea30;
const TIME_MULTIPLIER: u32 = 0x011c_5724;
/// `0043d4d0` / `00403e20`: the address of an integer / float setting's value
/// (`thiscall` on the setting).
const SETTING_INT_PTR: u32 = 0x0043_d4d0;
const SETTING_FLOAT_PTR: u32 = 0x0040_3e20;
/// `iNumHWThreads:General` (`011c3ea4`): `main` sets it to the processor
/// count (`GetSystemInfo`, import slot `00fdf0f8`; `SYSTEM_INFO` is 0x24
/// bytes, `dwNumberOfProcessors` at +0x14), 1 made 2.
const NUM_HW_THREADS: u32 = 0x011c_3ea4;
const GET_SYSTEM_INFO: u32 = 0x00fd_f0f8;
const SYSTEM_INFO_SIZE: u32 = 0x24;
const SYSTEM_INFO_PROCESSORS: u32 = 0x14;
/// The AI linear task thread manager (`00713d80` answers it) and its
/// `CreateThreads` (`008c7290`), `StartThreads` (`008c78c0`) and
/// `WaitForThreads` (`008c7990`); `008c80e0(1)`, which `Main::OnIdle`
/// (`0086e650`) calls before starting them.
const AI_THREAD_MANAGER: u32 = 0x0071_3d80;
const CREATE_AI_THREADS: u32 = 0x008c_7290;
const START_AI_THREADS: u32 = 0x008c_78c0;
const WAIT_FOR_AI_THREADS: u32 = 0x008c_7990;
const AI_THREADS_RENDER_SAFE: u32 = 0x008c_80e0;
/// `NiAVObject` world rotation (`NiMatrix3`, rows) and translation, PC
/// offsets (`bhkCollisionObject::hktoNiTransform` `00c65a80` reads them).
const WORLD_ROTATION: u32 = 0x68;
const WORLD_TRANSLATION: u32 = 0x8c;

/// Where the game's world step put a reference's 3D: its root node's world
/// rotation (rows) and translation, game units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BodyPose {
    pub reference: FormId,
    pub rotation: [[f32; 3]; 3],
    pub translation: [f32; 3],
}

/// The mover's move bits (`PlayerMover::MoveMode`, as `0093e860` sets them).
pub mod move_bits {
    pub const FORWARD: u32 = 0x1;
    pub const BACKWARD: u32 = 0x2;
    pub const LEFT: u32 = 0x4;
    pub const RIGHT: u32 = 0x8;
    pub const WALKING: u32 = 0x100;
    pub const RUNNING: u32 = 0x200;
    pub const SNEAKING: u32 = 0x400;
}

/// A frame's move as the viewer has it.
#[derive(Debug, Clone, Copy, Default)]
pub struct MoveInput {
    /// Where the viewer has the player's feet (game units).
    pub feet: [f32; 3],
    /// The heading (the game's z angle, radians).
    pub heading: f32,
    /// [`move_bits`].
    pub move_bits: u32,
    /// The frame's time step (seconds).
    pub seconds: f32,
    /// A jump asked for this frame.
    pub jump: bool,
}

/// Where the game's code put the player.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct MoveOutput {
    pub feet: [f32; 3],
    /// The character controller's `OutVelocity` (+0x4F0), game units/s.
    pub velocity: [f32; 3],
}

/// An untranslated address the movement reached.
#[derive(Debug, Clone, Default)]
pub struct Miss {
    /// Times reached (each a stopped stage or frame; in a probe run, every
    /// call).
    pub count: u64,
    /// The nearest caller entered by address, with how often.
    pub callers: BTreeMap<u32, u64>,
    /// The stage that reached it first (`havok`, `player`, `cell`,
    /// `controller`, `frame`).
    pub stage: &'static str,
    /// Its place in the order the addresses were first reached (0 first).
    pub first: usize,
}

/// A stop that isn't an untranslated function (memory nobody made, a
/// refusal of the bridge), by stage and first line.
pub type Stops = BTreeMap<(&'static str, String), u64>;

#[derive(Debug, Clone, PartialEq)]
enum Stage<T> {
    NotTried,
    Done(T),
    Failed(String),
}

impl<T: Copy> Stage<T> {
    fn done(&self) -> Option<T> {
        match self {
            Stage::Done(v) => Some(*v),
            _ => None,
        }
    }
}

/// The player's movement through the game's code: what was made, and what
/// was missing.
pub struct PlayerMovement {
    /// Go on past an untranslated address with a zero result instead of
    /// stopping (a diagnostic run: the addresses reached after the first
    /// miss are only indicative, and the state made is not used to move
    /// the player).
    pub probe: bool,
    pub misses: BTreeMap<u32, Miss>,
    pub stops: Stops,
    /// Frames the game's code moved the player, and frames it didn't.
    pub engine_frames: u64,
    pub fallback_frames: u64,
    /// The `Main` object (`main`, `0086af75`) and the scene graph (`0086b071`), made at
    /// start-up ([`PlayerMovement::start_up`]).
    main_object: Stage<u32>,
    /// `main` from `Main::Main` to `InitSceneGraph`, the renderer's start-up among it (stage
    /// `renderer`, [`run_main_up_to_scene_graph`]).
    renderer: Stage<()>,
    scene_graph: Stage<()>,
    /// `TES` as `InitTES` makes it (its `TES::TES` runs `InitHavokSystem`); stage `tes`.
    havok: Stage<()>,
    player: Stage<u32>,
    /// The cell made, by its form id.
    cell: Option<(FormId, Stage<u32>)>,
    controller: Stage<u32>,
    files: HashMap<u8, GameFile>,
    failed_files: HashMap<u8, String>,
    forms: HashMap<FormId, u32>,
    /// The last frame's result came from the game's code (no resync needed).
    in_step: bool,
    /// The references of the made cell whose 3D loaded, by form id.
    cell_refs: Vec<(FormId, u32)>,
    /// The AI linear task thread manager once its threads are made
    /// (`threads`, after `havok` as in the game).
    threads: Stage<u32>,
    /// The game's `main` (`0086a850`) run as its main thread ([`PlayerMovement::start_main`]),
    /// its frames (each a pass of its message loop) and how its last turn ended.
    main_thread: Option<engine::thread_host::MainThread>,
    pub main_frames: u64,
    main_stage: Stage<()>,
    /// When the status line ([`PlayerMovement::summary`]) was last printed from [`PlayerMovement::main_frame`].
    main_reported: Option<std::time::Instant>,
    /// Frames the AI threads (and the world step on thread 2) ran, and
    /// frames they didn't ([`PlayerMovement::run_ai_threads`]).
    pub world_frames: u64,
    pub world_fallback_frames: u64,
    /// The world step stopped once; it isn't tried again in this cell.
    world_failed: Option<String>,
    /// The miss recorder of a frame of main's handed over and not finished yet
    /// ([`PlayerMovement::begin_main_frame`]).
    frame_guard: Option<Guard>,
}

impl Default for PlayerMovement {
    fn default() -> Self {
        PlayerMovement {
            probe: false,
            misses: BTreeMap::new(),
            stops: BTreeMap::new(),
            engine_frames: 0,
            fallback_frames: 0,
            main_object: Stage::NotTried,
            renderer: Stage::NotTried,
            scene_graph: Stage::NotTried,
            havok: Stage::NotTried,
            player: Stage::NotTried,
            cell: None,
            controller: Stage::NotTried,
            files: HashMap::new(),
            failed_files: HashMap::new(),
            forms: HashMap::new(),
            in_step: false,
            cell_refs: Vec::new(),
            threads: Stage::NotTried,
            main_thread: None,
            main_frames: 0,
            main_stage: Stage::NotTried,
            main_reported: None,
            world_frames: 0,
            world_fallback_frames: 0,
            world_failed: None,
            frame_guard: None,
        }
    }
}

/// What a guarded run reached: untranslated addresses with their nearest
/// caller, in order.
type Reached = Rc<RefCell<Vec<(u32, u32)>>>;

/// What [`PlayerMovement::guard_open`] set up for one guarded run, for
/// [`PlayerMovement::guard_close`].
pub(crate) struct Guard {
    stage: &'static str,
    reached: Reached,
    trace_file: Option<std::ffi::OsString>,
    profile_file: Option<std::ffi::OsString>,
}

impl std::fmt::Debug for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Guard").field("stage", &self.stage).finish()
    }
}

/// Whether a panic on a game thread's OS thread (`game main thread`, `game thread N`) goes
/// unprinted: while a frame of main's runs beside the host ([`PlayerMovement::begin_main_frame`]),
/// as a guarded run on the host keeps every panic quiet (its stop is reported instead).
static QUIET_GAME_PANICS: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Installs (once) the panic hook that keeps game threads' panics quiet while
/// [`QUIET_GAME_PANICS`] is set, and passes every other panic to the hook before it.
fn quiet_game_panics(on: bool) {
    static INSTALLED: std::sync::Once = std::sync::Once::new();
    INSTALLED.call_once(|| {
        let before = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let game = std::thread::current()
                .name()
                .is_some_and(|n| n.starts_with("game "));
            if game && QUIET_GAME_PANICS.load(std::sync::atomic::Ordering::Relaxed) {
                return;
            }
            before(info)
        }));
    });
    QUIET_GAME_PANICS.store(on, std::sync::atomic::Ordering::Relaxed);
}

impl PlayerMovement {
    /// Runs `f` on the engine with the miss recorder in place; a stop comes
    /// back as its first line. The misses are counted under `stage`.
    pub(crate) fn guarded<R>(
        &mut self,
        e: &mut Engine,
        stage: &'static str,
        f: impl FnOnce(&mut Engine) -> Result<R, String>,
    ) -> Result<R, String> {
        let guard = self.guard_open(e, stage);
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let mark = e.guest_stack_mark();
        let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(e)));
        std::panic::set_hook(hook);
        if out.is_err() {
            // The stage runs again on later frames: the stop's locals go back.
            e.guest_stack_restore(mark);
        }
        self.guard_close(e, guard, out)
    }

    /// The first half of [`PlayerMovement::guarded`]: the miss recorder and the diagnostics in
    /// place for a run of `stage`.
    pub(crate) fn guard_open(&mut self, e: &mut Engine, stage: &'static str) -> Guard {
        let reached: Reached = Rc::new(RefCell::new(Vec::new()));
        let record = reached.clone();
        let probe = self.probe;
        e.on_open = Some(Rc::new(RefCell::new(
            move |e: &mut Engine, addr: u32, _: &[u32]| {
                let caller = e.call_stack.last().copied().unwrap_or(0);
                // A probe run can go on for a long time past a miss (or never come back): each
                // new miss is written to stderr at once, so a run that hangs shows where.
                if probe && !record.borrow().iter().any(|&(a, _)| a == addr) {
                    let path: Vec<String> = e
                        .call_stack
                        .iter()
                        .rev()
                        .take(6)
                        .map(|a| format!("{a:08x}"))
                        .collect();
                    eprintln!(
                        "probe ({stage}): {addr:08x} not provided, zero answered (in {})",
                        path.join(" < ")
                    );
                }
                record.borrow_mut().push((addr, caller));
                probe.then(Ret::default)
            },
        )));
        e.call_stack.clear();
        // `NV_MOVE_TRACE=FILE`: the calls a stopped run returned from, last
        // first-to-last, appended to FILE (a diagnostic).
        let trace_file = std::env::var_os("NV_MOVE_TRACE");
        if trace_file.is_some() {
            e.trace = Some(Default::default());
            // `NV_MOVE_CALL_BUDGET=N`: stop after N more calls (finds where a run loops); with
            // `NV_MOVE_BUDGET_FROM=F`, only in main's frames from F on (start-up is frame 0).
            let from: u64 = std::env::var("NV_MOVE_BUDGET_FROM")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);
            let budgeted = from == 0 || (stage == "main" && self.main_frames >= from);
            if let Some(n) = std::env::var("NV_MOVE_CALL_BUDGET")
                .ok()
                .and_then(|v| v.parse().ok())
                .filter(|_| budgeted)
            {
                e.call_budget = Some(n);
            }
            // `NV_MOVE_TRACE_KEEP=N`: keep the last N calls instead of `TRACE_KEEP`.
            if let Some(keep) = std::env::var("NV_MOVE_TRACE_KEEP")
                .ok()
                .and_then(|v| v.parse().ok())
            {
                e.trace_keep = keep;
            }
        }
        // `NV_MOVE_CALL_PROFILE=FILE`: how many times each address was called in this stage, most
        // first, appended to FILE (where a long run spends its calls).
        let profile_file = std::env::var_os("NV_MOVE_CALL_PROFILE");
        if profile_file.is_some() {
            e.call_counts = Some(Default::default());
            if let Some(n) = std::env::var("NV_MOVE_CALL_BUDGET")
                .ok()
                .and_then(|v| v.parse().ok())
            {
                e.call_budget = Some(n);
            }
        }
        Guard {
            stage,
            reached,
            trace_file,
            profile_file,
        }
    }

    /// The second half of [`PlayerMovement::guarded`]: the run's outcome `out` (a stop as its
    /// panic) recorded with what it reached, the traces written.
    pub(crate) fn guard_close<R>(
        &mut self,
        e: &mut Engine,
        guard: Guard,
        out: std::thread::Result<Result<R, String>>,
    ) -> Result<R, String> {
        let Guard {
            stage,
            reached,
            trace_file,
            profile_file,
        } = guard;
        e.on_open = None;
        // After a panic the stack still holds the functions it unwound
        // through (`Engine::call` pops only on return): the innermost few go
        // with the stop, so a fault inside translated code says where.
        let unwound: Vec<u32> = e.call_stack.iter().rev().take(6).copied().collect();
        e.call_stack.clear();
        for &(addr, caller) in reached.borrow().iter() {
            let first = self.misses.len();
            let miss = self.misses.entry(addr).or_insert_with(|| Miss {
                stage,
                first,
                ..Miss::default()
            });
            miss.count += 1;
            *miss.callers.entry(caller).or_default() += 1;
        }
        let out = match out {
            Ok(result) => result,
            Err(payload) => {
                let text = payload
                    .downcast_ref::<String>()
                    .cloned()
                    .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                    .unwrap_or_default();
                let first = text.lines().next().unwrap_or("");
                if unwound.is_empty() {
                    Err(first.to_string())
                } else {
                    let path: Vec<String> = unwound.iter().map(|a| format!("{a:08x}")).collect();
                    Err(format!("{first} (in {})", path.join(" < ")))
                }
            }
        };
        if let Err(why) = &out {
            if !why.starts_with("open function") {
                *self.stops.entry((stage, why.clone())).or_default() += 1;
            }
        }
        let trace = e.trace.take();
        // `NV_MOVE_TRACE_FRAMES=A-B`: main's frames A to B (counted from 0) have their traces
        // written too, not only a stop's (what the game did on those frames).
        let traced_frame = stage == "main"
            && std::env::var("NV_MOVE_TRACE_FRAMES").ok().is_some_and(|v| {
                let mut ends = v.split('-').filter_map(|n| n.trim().parse::<u64>().ok());
                let from = ends.next().unwrap_or(u64::MAX);
                let to = ends.next().unwrap_or(from);
                (from..=to).contains(&self.main_frames)
            });
        let header = match &out {
            Err(why) => Some(format!("== {stage}: {why}\n")),
            Ok(_) if traced_frame => Some(format!("== main frame {}\n", self.main_frames)),
            Ok(_) => None,
        };
        if let (Some(path), Some(trace), Some(mut text)) = (trace_file, trace, header) {
            for (addr, args, eax) in trace {
                let words: Vec<String> = args.iter().map(|w| format!("{w:08x}")).collect();
                let _ = writeln!(text, "{addr:08x}({}) -> {eax:08x}", words.join(","));
            }
            use std::io::Write as _;
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
            {
                let _ = file.write_all(text.as_bytes());
            }
        }
        if let (Some(path), Some(counts)) = (profile_file, e.call_counts.take()) {
            let mut by_addr: HashMap<u32, u64> = HashMap::new();
            for (&key, &n) in &counts {
                *by_addr.entry(key as u32).or_default() += n;
            }
            let mut rows: Vec<(u32, u64)> = by_addr.into_iter().collect();
            rows.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
            let total: u64 = rows.iter().map(|r| r.1).sum();
            // `NV_MOVE_CALL_PROFILE_TOP=N`: list N rows of each table instead of 200 (0: all).
            let top = std::env::var("NV_MOVE_CALL_PROFILE_TOP")
                .ok()
                .and_then(|v| v.parse::<usize>().ok())
                .map_or(200, |n| if n == 0 { usize::MAX } else { n });
            let mut text = format!(
                "== {stage}: {total} calls
"
            );
            for (addr, n) in rows.iter().take(top) {
                let _ = writeln!(text, "{addr:08x}	{n}");
            }
            let mut pairs: Vec<(u64, u64)> = counts.into_iter().collect();
            pairs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
            let _ = writeln!(text, "-- {stage}: caller -> address");
            for (key, n) in pairs.iter().take(top) {
                let _ = writeln!(text, "{:08x} -> {:08x}	{n}", key >> 32, *key as u32);
            }
            use std::io::Write as _;
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
            {
                let _ = file.write_all(text.as_bytes());
            }
        }
        // In a probe run a miss doesn't stop the run, so the stages after it
        // run too; the state made is not the game's, and `step` never moves
        // the player with it.
        out
    }

    /// The game's start-up from `Main::Main` to `TES`, as `main` runs it after `NiInit`, each
    /// part a stage that records what it reaches: `main`: `operator new(0xA4)` and `Main::Main`
    /// (`0086c160`) with the main window (`engine::units::platform::create_window`, where `main`
    /// calls `CreateWindowExA` at `0086af42`) and the module handle, kept at `011dea0c`
    /// (`0086af4b`..`0086af9e`); `renderer`: `main` on to the scene graph ([`run_main_up_to_scene_graph`], the renderer `0086d500` among it); then `before_scene_graph` (the renderer's start-up point,
    /// [`crate::start_renderer_fpu`] in the host); `scenegraph`: `Main::InitSceneGraph`
    /// (`0086d590`) on it (`0086b071`); `tes`: `InitTES`'s head up to `TES::TES` and
    /// `00450c50` (`0086cf81`..`0086d076`), whose `TES::TES` runs `InitHavokSystem`. Not run
    /// (listed): the rest of `main` after the scene graph (`00702250`, `006519f0`, `00871c90`, `006a2920`, `0086c010`, the
    /// movie, ...) and `InitTES`'s scope and log objects.
    pub fn start_up(&mut self, e: &mut Engine, before_scene_graph: impl FnOnce(&mut Engine)) {
        self.main_object = match self.guarded(e, "main", |e| {
            let window = engine::units::platform::create_window(e);
            let block = e.call(NEW, &[MAIN_SIZE]).u32();
            let main = if block == 0 {
                0
            } else {
                e.call(MAIN_CONSTRUCT, &[block, window, IMAGE_BASE]).u32()
            };
            e.mem.set_u32(MAIN_OBJECT, main);
            Ok(main)
        }) {
            Ok(main) => Stage::Done(main),
            Err(why) => Stage::Failed(why),
        };
        if let Some(main) = self.main_object.done() {
            self.renderer = match self.guarded(e, "renderer", |e| {
                run_main_up_to_scene_graph(e, main);
                Ok(())
            }) {
                Ok(()) => Stage::Done(()),
                Err(why) => Stage::Failed(why),
            };
        }
        before_scene_graph(e);
        if let (Some(main), Some(())) = (self.main_object.done(), self.renderer.done()) {
            self.scene_graph = match self.guarded(e, "scenegraph", |e| {
                e.call(INIT_SCENE_GRAPH, &[main]);
                Ok(())
            }) {
                Ok(()) => Stage::Done(()),
                Err(why) => Stage::Failed(why),
            };
        }
        if self.scene_graph.done().is_some() {
            self.havok = match self.guarded(e, "tes", make_tes) {
                Ok(()) => Stage::Done(()),
                Err(why) => Stage::Failed(why),
            };
        }
    }

    /// Makes what the move needs that isn't made yet (the module's stages),
    /// for the player in `cell` at `feet`. Each stage runs once (per cell for
    /// the cell's); one that stopped isn't run again. True when everything is
    /// made.
    pub fn prepare(
        &mut self,
        e: &mut Engine,
        order: &LoadOrder,
        cell: Option<FormId>,
        input: &MoveInput,
    ) -> bool {
        // The player is made in `InitTES` after `TES::TES` (011dea10, which its constructor
        // reads), so only once the tes stage passed.
        if self.player == Stage::NotTried {
            self.player = if self.havok.done().is_some() {
                match self.make_player(e, order, input) {
                    Ok(p) => Stage::Done(p),
                    Err(why) => Stage::Failed(why),
                }
            } else {
                Stage::Failed("needs the tes stage".into())
            };
        }
        if self.threads == Stage::NotTried && self.havok.done().is_some() {
            self.threads = match self.guarded(e, "threads", start_ai_threads) {
                Ok(manager) => Stage::Done(manager),
                Err(why) => Stage::Failed(why),
            };
        }
        let Some(cell) = cell else {
            return false;
        };
        if self.cell.as_ref().map(|(id, _)| *id) != Some(cell) {
            // A new cell: the player's controller is made again in it.
            self.cell_refs.clear();
            self.world_failed = None;
            let stage = if self.havok.done().is_some() {
                match self.make_cell(e, order, cell) {
                    Ok(c) => Stage::Done(c),
                    Err(why) => Stage::Failed(why),
                }
            } else {
                Stage::Failed("needs the tes stage".into())
            };
            self.cell = Some((cell, stage));
            self.controller = Stage::NotTried;
        }
        let made_cell = self.cell.as_ref().and_then(|(_, s)| s.done());
        if self.controller == Stage::NotTried {
            self.controller = match (self.player.done(), made_cell) {
                (Some(player), Some(cell)) => match self.guarded(e, "controller", |e| {
                    e.call(SET_PARENT_CELL, &[player, cell]);
                    set_position(e, player, input.feet);
                    e.call(PLAYER_LOAD_3D, &[player, 0]);
                    e.call(INIT_HAVOK, &[player]);
                    let controller = e.call(GET_CHAR_CONTROLLER, &[player]).u32();
                    if controller == 0 {
                        return Err("MobileObject::InitHavok made no character controller".into());
                    }
                    Ok(controller)
                }) {
                    Ok(c) => Stage::Done(c),
                    Err(why) => Stage::Failed(why),
                },
                _ => Stage::Failed("needs the player and cell stages".into()),
            };
        }
        self.controller.done().is_some()
    }

    /// The player as `Main::InitTES` makes it (the module, stage 2).
    fn make_player(
        &mut self,
        e: &mut Engine,
        order: &LoadOrder,
        input: &MoveInput,
    ) -> Result<u32, String> {
        let player = self.guarded(e, "player", |e| {
            let block = e.call(NEW, &[PLAYER_SIZE]).u32();
            let player = e.call(PLAYER_CONSTRUCT, &[block]).u32();
            e.mem.set_u32(PLAYER, player);
            e.vcall(player, SET_FORM_ID, &[0x14, 1]);
            Ok(player)
        })?;
        let base = self.load_form(e, order, PLAYER_BASE)?;
        self.guarded(e, "player", |e| {
            e.call(SET_OBJECT_REFERENCE, &[player, base]);
            set_position(e, player, input.feet);
            set_heading(e, player, input.heading);
            e.vcall(player, INIT_ITEM, &[]);
            Ok(player)
        })
    }

    /// The plugin's file in the game's memory, made on first need.
    /// Opened by the game's `OpenTES` and header reader
    /// ([`GameFile::open_with_header`]), its masters first; then its master
    /// table by `TESFile::GenIndexTable` ([`crate::game_file::gen_index_table`])
    /// against every file opened so far.
    fn file(
        &mut self,
        e: &mut Engine,
        order: &LoadOrder,
        plugin: &esm::LoadedPlugin,
    ) -> Result<GameFile, String> {
        if let Some(file) = self.files.get(&plugin.load_index) {
            return Ok(*file);
        }
        if let Some(why) = self.failed_files.get(&plugin.load_index) {
            return Err(why.clone());
        }
        let made = self.open_file(e, order, plugin);
        if let Err(why) = &made {
            self.failed_files.insert(plugin.load_index, why.clone());
        }
        made
    }

    fn open_file(
        &mut self,
        e: &mut Engine,
        order: &LoadOrder,
        plugin: &esm::LoadedPlugin,
    ) -> Result<GameFile, String> {
        for master in &plugin.plugin.header().masters {
            let Some(m) = order
                .plugins()
                .iter()
                .find(|p| p.name.eq_ignore_ascii_case(master))
            else {
                return Err(format!("{}: master {master} not loaded", plugin.name));
            };
            self.file(e, order, m)?;
        }
        let directory = plugin
            .path
            .as_ref()
            .and_then(|p| p.parent())
            .map(|d| format!("{}\\", d.display()))
            .unwrap_or_else(|| "Data\\".into());
        let file = self.guarded(e, "files", |e| {
            GameFile::open_with_header(e, plugin, &directory)
        })?;
        self.files.insert(plugin.load_index, file);
        let mut opened: Vec<(u8, GameFile)> = self.files.iter().map(|(i, f)| (*i, *f)).collect();
        opened.sort_by_key(|(i, _)| *i);
        let opened: Vec<GameFile> = opened.into_iter().map(|(_, f)| f).collect();
        self.guarded(e, "files", |e| {
            if crate::game_file::gen_index_table(e, &file, &opened) {
                Ok(())
            } else {
                Err(format!(
                    "{}: TESFile::GenIndexTable missed a master",
                    plugin.name
                ))
            }
        })?;
        Ok(file)
    }

    /// Loads the form `id` as `TESDataHandler` does (see the module), once.
    fn load_form(&mut self, e: &mut Engine, order: &LoadOrder, id: FormId) -> Result<u32, String> {
        if let Some(form) = self.forms.get(&id) {
            return Ok(*form);
        }
        let record = order
            .get(id)
            .ok_or_else(|| format!("form {id}: not in the load order"))?;
        let kind = record.entry.header.kind;
        let form_type = world::more_functions::form_type_number(kind)
            .ok_or_else(|| format!("form {id}: record type {kind} has no form type"))?;
        let file = self.file(e, order, record.plugin)?;
        let plugin = record.plugin;
        let entry = record.entry;
        let form = self.guarded(e, "forms", |e| {
            let form = e.call(CREATE_FORM_OF_TYPE, &[u32::from(form_type)]).u32();
            if form == 0 {
                return Err(format!(
                    "form {id}: CreateFormOfType({form_type:#x}) gave none"
                ));
            }
            file.supply_any(e, plugin, entry)?;
            if !e.call(LOAD_FORM, &[form, file.file]).bool() {
                return Err(format!("form {id}: {kind} Load failed"));
            }
            e.vcall(form, INIT_ITEM, &[]);
            Ok(form)
        })?;
        self.forms.insert(id, form);
        Ok(form)
    }

    /// The player's cell, its temporary references and their 3D (the
    /// module, stage 3).
    fn make_cell(&mut self, e: &mut Engine, order: &LoadOrder, id: FormId) -> Result<u32, String> {
        let record = order
            .get(id)
            .ok_or_else(|| format!("cell {id}: not in the load order"))?;
        let plugin = record.plugin;
        let local = record.entry.header.form_id;
        // The cell's placed records (its children groups) and their base
        // forms.
        let children: Vec<&esm::RecordEntry> = plugin
            .plugin
            .records()
            .iter()
            .filter(|r| r.cell == Some(local))
            .collect();
        let mut bases = Vec::new();
        for child in &children {
            let kind = child.header.kind.as_bytes();
            if kind != b"REFR" && kind != b"ACHR" && kind != b"ACRE" {
                continue;
            }
            let name = plugin
                .plugin
                .subrecord_of(child, esm::FourCC(*b"NAME"))
                .map_err(|err| err.to_string())?;
            if let Some(name) = name.filter(|n| n.len() >= 4) {
                let base = FormId(u32::from_le_bytes([name[0], name[1], name[2], name[3]]));
                bases.push(plugin.to_global(base));
            }
        }
        bases.sort();
        bases.dedup();
        let mut unloaded = 0;
        for base in bases {
            if self.load_form(e, order, base).is_err() {
                unloaded += 1;
            }
        }
        if unloaded > 0 {
            *self
                .stops
                .entry((
                    "cell",
                    format!("{unloaded} base forms of the cell's references didn't load"),
                ))
                .or_default() += 1;
        }
        let cell = self.load_form(e, order, id)?;
        let file = self.file(e, order, plugin)?;
        // The cell's record and its children groups, as one range of the
        // file (the groups follow the record), and the 0x18 bytes after it.
        let start = record.entry.offset;
        let end = children
            .iter()
            .map(|r| r.offset + plugin.plugin.record_bytes(r).len())
            .chain(std::iter::once(
                start + plugin.plugin.record_bytes(record.entry).len(),
            ))
            .max()
            .unwrap_or(start)
            + HEADER_SIZE;
        let bytes = plugin
            .plugin
            .file_bytes(start..end.min(plugin.plugin.file_size()));
        let refs: Vec<FormId> = children
            .iter()
            .filter(|r| r.header.kind.as_bytes() == b"REFR")
            .map(|r| plugin.to_global(r.header.form_id))
            .collect();
        self.guarded(e, "cell", |e| {
            engine::units::platform::supply_file_bytes(e, file.bs_file, start as u32, bytes);
            if !e.call(0x0047_23a0, &[file.file, start as u32]).bool() {
                return Err(format!("cell {id}: TESFile::SetOffset failed"));
            }
            if !e.call(CELL_LOAD_TEMP_DATA, &[cell, file.file]).bool() {
                return Err(format!("cell {id}: LoadTempDataFromFile failed"));
            }
            e.call(CELL_LOAD_3D, &[cell]);
            Ok(())
        })?;
        for reference in refs {
            let made =
                self.guarded(e, "cell", |e| Ok(e.call(LOOKUP_FORM, &[reference.0]).u32()))?;
            if made == 0 {
                continue;
            }
            if self
                .guarded(e, "cell", |e| {
                    e.call(REFERENCE_LOAD_3D, &[made, 0]);
                    Ok(())
                })
                .is_ok()
            {
                self.cell_refs.push((reference, made));
                // Bridge: the game adds a reference's collision to the
                // world when its queued 3D task is unpacked on the main
                // thread (`TaskUnpackFunc` `0087b990`, the reference's vtable
                // +0x1C4 at `0087cbf3`); here right after its load.
                let _ = self.guarded(e, "cell", |e| {
                    e.vcall(made, REFERENCE_INIT_HAVOK, &[]);
                    Ok(())
                });
            }
        }
        Ok(cell)
    }

    /// The frame's move through the game's code; `None` when it couldn't run
    /// (the misses are recorded) and the viewer moves the player itself.
    pub fn step(
        &mut self,
        e: &mut Engine,
        order: &LoadOrder,
        cell: Option<FormId>,
        input: &MoveInput,
    ) -> Option<MoveOutput> {
        let ready = self.prepare(e, order, cell, input);
        let (player, controller) = (self.player.done(), self.controller.done());
        // A probe run goes through the frame with whatever was made (and
        // never moves the player).
        let runs = if self.probe {
            player.is_some()
        } else {
            ready && player.is_some() && controller.is_some()
        };
        if !runs || input.jump {
            self.fallback_frames += 1;
            self.in_step = false;
            return None;
        }
        let (player, controller) = (player.unwrap_or(0), controller.unwrap_or(0));
        let resync = !self.in_step;
        let out = self.guarded(e, "frame", |e| {
            if resync {
                set_position(e, player, input.feet);
            }
            set_heading(e, player, input.heading);
            let mover = e.mem.u32(player + ACTOR_MOVER);
            e.call(FORCE_MOVE_MODE, &[mover, input.move_bits]);
            e.vcall(mover, MOVER_UPDATE, &[input.seconds.to_bits()]);
            let position = e.vcall(player, GET_POSITION, &[]).u32();
            let feet = [0, 4, 8].map(|o| e.mem.f32(position + o));
            let velocity = [0, 4, 8].map(|o| e.mem.f32(controller + 0x4f0 + o));
            Ok(MoveOutput { feet, velocity })
        });
        match out {
            Ok(out) if !self.probe => {
                self.engine_frames += 1;
                self.in_step = true;
                Some(out)
            }
            _ => {
                self.fallback_frames += 1;
                self.in_step = false;
                None
            }
        }
    }

    /// The frame's AI linear task threads through the game's code
    /// (`--physics engine`), where the game steps its Havok world: as
    /// `Main::OnIdle` (`0086e650`) runs them with more than one processor,
    /// `008c80e0(1)` and `StartThreads` (`008c78c0`), then `WaitForThreads`
    /// (`008c7990`). Waiting, the main thread hands the engine to the game
    /// threads ([`engine::thread_host`]): thread 1 (`008c7da0`) and thread 2
    /// (`008c7f50`) step through their stage events, and thread 2, once
    /// thread 1 is past its stage 4, runs `TES::UpdateCellAnimations`
    /// (`00453550`) with `00453850`'s seconds: the cell's `UpdateManagedNodes`
    /// (`00551890`, the world's `bhkWorld::Update`) and the scene-graph update
    /// that copies each moving body's transform to its node. Returns every
    /// loaded reference's root node where it is now, or `None` when the
    /// threads or the cell aren't made or the frame stopped (the misses are
    /// recorded under `world`, those of the game threads with their own
    /// callers; a stopped frame isn't tried again in the cell) and the viewer
    /// steps its own bodies.
    ///
    /// Stand-in for the frame timer (`Main::OnIdle_UpdateTimer`, `0086f260`):
    /// the viewer's seconds instead of the game's timer, given to
    /// `bhkWorld::SetDeltaTime` (`00c66760`, with `00525420`'s flag; the
    /// menu mode isn't consulted, so not mode 4) and kept at `011dea30`
    /// times the setting at `011c5724`. The rest of `0086e650` (what the main
    /// thread does while the threads run) doesn't run.
    pub fn run_ai_threads(&mut self, e: &mut Engine, seconds: f32) -> Option<Vec<BodyPose>> {
        let manager = match (&self.threads, &self.cell, &self.world_failed) {
            (Stage::Done(manager), Some((_, Stage::Done(_))), None) => *manager,
            _ => {
                self.world_fallback_frames += 1;
                return None;
            }
        };
        let refs = self.cell_refs.clone();
        let out = self.guarded(e, "world", |e| {
            let flag = e.call(HAVOK_STEP_FLAG, &[]).u8() as u32;
            e.call(SET_DELTA_TIME, &[seconds.to_bits(), flag, 0]);
            let multiplier = e.call(SETTING_FLOAT_PTR, &[TIME_MULTIPLIER]).u32();
            let scaled = (e.mem.f32(multiplier) as f64 * seconds as f64) as f32;
            e.mem.set_f32(FRAME_SECONDS, scaled);
            e.call(AI_THREADS_RENDER_SAFE, &[1]);
            e.call(START_AI_THREADS, &[manager]);
            e.call(WAIT_FOR_AI_THREADS, &[manager]);
            let mut poses = Vec::with_capacity(refs.len());
            for &(reference, made) in &refs {
                let root = e.vcall(made, GET_3D, &[]).u32();
                if root == 0 {
                    continue;
                }
                let r = |i: u32| e.mem.f32(root + WORLD_ROTATION + 4 * i);
                let rotation = [[r(0), r(1), r(2)], [r(3), r(4), r(5)], [r(6), r(7), r(8)]];
                let translation = [0, 4, 8].map(|o| e.mem.f32(root + WORLD_TRANSLATION + o));
                poses.push(BodyPose {
                    reference,
                    rotation,
                    translation,
                });
            }
            Ok(poses)
        });
        match out {
            Ok(poses) if !self.probe => {
                self.world_frames += 1;
                Some(poses)
            }
            Ok(_) => {
                self.world_fallback_frames += 1;
                None
            }
            Err(why) => {
                self.world_fallback_frames += 1;
                self.world_failed = Some(why);
                None
            }
        }
    }

    /// Starts the game's `main` (`0086a850`, `WinMain`: the module handle, no previous instance,
    /// no command line pointer, `SW_SHOWDEFAULT`) as its main thread
    /// (`engine::thread_host::start_main_thread`) and runs its start-up: everything up to the
    /// first time its message loop finds the queue empty, the end of its first frame. Recorded
    /// as stage `main`.
    pub fn start_main(&mut self, e: &mut Engine) {
        self.main_thread = Some(engine::thread_host::start_main_thread(
            WIN_MAIN,
            vec![IMAGE_BASE, 0, 0, SW_SHOWDEFAULT],
        ));
        self.main_frame(e);
    }

    /// Whether the game's `main` runs as the main thread ([`PlayerMovement::start_main`]).
    pub fn main_runs(&self) -> bool {
        self.main_thread.is_some()
    }

    /// Whether the movement's own stages can touch the engine ([`PlayerMovement::step`],
    /// [`PlayerMovement::run_ai_threads`]): only once the `tes` stage made Havok's world. With
    /// the game's `main` running they never are made.
    pub fn stages_touch_engine(&self) -> bool {
        self.havok.done().is_some()
    }

    /// One frame of the game's main loop (a pass of `main`'s message loop, `Main::OnIdle`
    /// among it), until its message queue is empty again; true when it ended a frame. Once it
    /// stopped it isn't resumed.
    pub fn main_frame(&mut self, e: &mut Engine) -> bool {
        self.begin_main_frame(e) && self.finish_main_frame(e)
    }

    /// The first half of [`PlayerMovement::main_frame`]: hands the engine to the game's main
    /// thread for its next frame and returns at once (`engine::thread_host::begin_main_frame`),
    /// the miss recorder in place as for a guarded run. Until
    /// [`PlayerMovement::finish_main_frame`] the caller must not touch the engine. False when
    /// main doesn't run or stopped; true too when a frame is already in flight.
    pub fn begin_main_frame(&mut self, e: &mut Engine) -> bool {
        if self.frame_guard.is_some() {
            return true;
        }
        if matches!(self.main_stage, Stage::Failed(_)) {
            return false;
        }
        let Some(mut main) = self.main_thread.take() else {
            return false;
        };
        let guard = self.guard_open(e, "main");
        quiet_game_panics(true);
        // Nothing handed over (main finished or stopped): the finish answers how it ended.
        engine::thread_host::begin_main_frame(e, &mut main);
        self.main_thread = Some(main);
        self.frame_guard = Some(guard);
        true
    }

    /// Whether a frame of main's is in flight ([`PlayerMovement::begin_main_frame`]).
    pub fn frame_in_flight(&self) -> bool {
        self.frame_guard.is_some()
    }

    /// The second half of [`PlayerMovement::main_frame`]: waits for the frame in flight, takes
    /// the engine back and records how it went; true when it ended a frame. False without one.
    pub fn finish_main_frame(&mut self, e: &mut Engine) -> bool {
        let Some(guard) = self.frame_guard.take() else {
            return false;
        };
        let Some(mut main) = self.main_thread.take() else {
            return false;
        };
        let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            match engine::thread_host::finish_main_frame(e, &mut main) {
                engine::thread_host::MainTurn::Frame => Ok(()),
                engine::thread_host::MainTurn::Finished(code) => {
                    // The exe's startup (`00ecc35d`, which the host stands in for by calling
                    // main itself) hands main's answer to `exit` (`00ecc479`), which ends the
                    // process: with the game's own window (the play build, `NV_D3D9=real`) the
                    // run ends here too, as when its window is closed. Without that window (tests,
                    // the viewer's own renderer) it is reported as main's end.
                    #[cfg(windows)]
                    if engine::units::platform::d3d9real::wanted() {
                        eprintln!("The game's main returned {code}: the run ends");
                        std::process::exit(code as i32);
                    }
                    Err(format!("main returned {code}"))
                }
                engine::thread_host::MainTurn::Stopped(why) => Err(why),
            }
        }));
        quiet_game_panics(false);
        let turn = self.guard_close(e, guard, out);
        self.main_thread = Some(main);
        match turn {
            Ok(()) => {
                self.main_frames += 1;
                self.main_stage = Stage::Done(());
                // The status line every 5 s of the run while main's frames go on (the viewer's own
                // print stops when its walking stops).
                if self
                    .main_reported
                    .map_or(true, |t| t.elapsed().as_secs() >= 5)
                {
                    self.main_reported = Some(std::time::Instant::now());
                    println!("{}", self.summary());
                    // `NV_MOVE_WATCH=addr,addr..`: the words at those exe addresses with it (a
                    // diagnostic of the game's state: which menus exist, flags).
                    if let Ok(list) = std::env::var("NV_MOVE_WATCH") {
                        let words: Vec<String> = list
                            .split(',')
                            // `A>O>O..`: the word at A, then the word at that plus O, and on.
                            .filter_map(|item| {
                                let mut parts = item.trim().split('>');
                                let mut word =
                                    e.mem.u32(u32::from_str_radix(parts.next()?, 16).ok()?);
                                for offset in parts {
                                    let offset = u32::from_str_radix(offset, 16).ok()?;
                                    if word == 0 {
                                        break;
                                    }
                                    word = e.mem.u32(word.wrapping_add(offset));
                                }
                                Some(format!("{}={word:08x}", item.trim()))
                            })
                            .collect();
                        println!("watch: {}", words.join(" "));
                    }
                    // `NV_MOVE_THREADS`: the game threads' states with it (a diagnostic: which
                    // thread sleeps, waits or has turns, `thread_host::describe`).
                    if std::env::var_os("NV_MOVE_THREADS").is_some() {
                        println!("threads: {}", engine::thread_host::describe(e));
                    }
                    // `NV_SAVE_OUT=DIR` (a test aid): the saves the game wrote, copied out
                    // ([`crate::save_out`]).
                    for name in crate::save_out::copy_saves(e) {
                        println!("NV_SAVE_OUT: copied {name}");
                    }
                }
                true
            }
            Err(why) => {
                self.main_stage = Stage::Failed(why);
                false
            }
        }
    }

    /// Whether the stages are made, as a line.
    pub fn summary(&self) -> String {
        fn word<T>(s: &Stage<T>) -> String {
            match s {
                Stage::NotTried => "not tried".into(),
                Stage::Done(_) => "made".into(),
                Stage::Failed(why) => format!("stopped ({why})"),
            }
        }
        let cell = match &self.cell {
            Some((id, s)) => format!("{id}: {}", word(s)),
            None => "none".into(),
        };
        if self.main_thread.is_some() {
            // The game's `main` runs the start-up and the frames itself.
            return format!(
                "Game's movement: main {} frames; {}; {} untranslated addresses reached.",
                self.main_frames,
                match &self.main_stage {
                    Stage::Failed(why) => format!("stopped ({why})"),
                    Stage::Done(()) => "running".to_string(),
                    Stage::NotTried => "not started".to_string(),
                },
                self.misses.len()
            );
        }
        let mut line = format!(
            "Game's movement: {} frames moved by the game's code, {} by crates/physics; \
             Main {}; renderer {}; scene graph {}; TES {}; player {}; AI threads {}; cell {cell}; \
             controller {}; {} untranslated addresses reached.",
            self.engine_frames,
            self.fallback_frames,
            word(&self.main_object),
            word(&self.renderer),
            word(&self.scene_graph),
            word(&self.havok),
            word(&self.player),
            word(&self.threads),
            word(&self.controller),
            self.misses.len()
        );
        if self.world_frames + self.world_fallback_frames > 0 {
            let _ = write!(
                line,
                " Game's physics: {} world steps by the game's code (AI thread 2), {} by \
                 crates/physics{}.",
                self.world_frames,
                self.world_fallback_frames,
                match &self.world_failed {
                    Some(why) => format!(" (stopped: {why})"),
                    None => String::new(),
                }
            );
        }
        line
    }

    /// The other stops (memory nobody made, a refusal of the bridge), most
    /// first: count, stage, first line.
    pub fn stops_tsv(&self) -> String {
        let mut rows: Vec<_> = self.stops.iter().collect();
        rows.sort_by(|a, b| b.1.cmp(a.1));
        let mut out = String::from("count\tstage\tstop\n");
        for ((stage, why), n) in rows {
            let _ = writeln!(out, "{n}\t{stage}\t{why}");
        }
        out
    }

    /// The untranslated addresses reached, most reached first: address,
    /// count, stage, callers (`caller×count`), the order they were first
    /// reached in, and the ledger's unit and name when `names` has them.
    pub fn missing_tsv(&self, names: &dyn Fn(u32) -> Option<(String, String)>) -> String {
        let mut rows: Vec<(&u32, &Miss)> = self.misses.iter().collect();
        rows.sort_by(|a, b| b.1.count.cmp(&a.1.count).then(a.0.cmp(b.0)));
        let mut out = String::from("address\thits\tstage\tcallers\tfirst\tunit\tname\n");
        for (addr, miss) in rows {
            let callers = miss
                .callers
                .iter()
                .map(|(c, n)| format!("{c:08x}x{n}"))
                .collect::<Vec<_>>()
                .join(",");
            let (unit, name) = names(*addr).unwrap_or_default();
            let _ = writeln!(
                out,
                "{addr:08x}\t{}\t{}\t{callers}\t{}\t{unit}\t{name}",
                miss.count, miss.stage, miss.first
            );
        }
        out
    }
}

/// `NV_MOVE_KEYS=FRAME:DIK:HOLD,..` (hex DIK key codes): keys held down from main's frame FRAME
/// for HOLD frames, on top of the viewer's (a diagnostic: drives the game's menus with exact
/// frame timing).
fn key_script() -> &'static [(u64, u8, u64)] {
    static SCRIPT: std::sync::OnceLock<Vec<(u64, u8, u64)>> = std::sync::OnceLock::new();
    SCRIPT.get_or_init(|| {
        std::env::var("NV_MOVE_KEYS")
            .unwrap_or_default()
            .split(',')
            .filter_map(|item| {
                let mut part = item.trim().split(':');
                let frame = part.next()?.parse().ok()?;
                let code = u8::from_str_radix(part.next()?, 16).ok()?;
                let hold = part.next().map_or(Some(1), |h| h.parse().ok())?;
                Some((frame, code, hold))
            })
            .collect()
    })
}

/// Whether the real game window's keyboard and mouse reach the game: always in a run without a
/// script; in a scripted run (`NV_MOVE_KEYS` or `NV_MOVE_MOUSE`) only with `NV_MOVE_REAL_INPUT=1`.
/// A key or click on a test window had mixed into scripted runs (whole clips fired, the Pip-Boy
/// opened) and read as port bugs.
fn real_input_reaches_game() -> bool {
    static REACHES: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *REACHES.get_or_init(|| {
        let set = |name| std::env::var(name).is_ok_and(|v| !v.trim().is_empty());
        !(set("NV_MOVE_KEYS") || set("NV_MOVE_MOUSE"))
            || std::env::var("NV_MOVE_REAL_INPUT").as_deref() == Ok("1")
    })
}

/// One `NV_MOVE_MOUSE` item: the first frame, the motion per frame, how many frames, the buttons.
type MouseStep = (u64, i32, i32, u64, u8);

/// `NV_MOVE_MOUSE=FRAME:DX:DY:HOLD[:BUTTONS],..` (decimal): the mouse moves by (DX, DY) on each
/// of main's frames FRAME..FRAME+HOLD, with the buttons of the BUTTONS bit mask (bit 0 the left
/// button) held, on top of the viewer's (a diagnostic, like `NV_MOVE_KEYS`: turns the player in
/// a scripted run).
fn mouse_script() -> &'static [MouseStep] {
    static SCRIPT: std::sync::OnceLock<Vec<MouseStep>> = std::sync::OnceLock::new();
    SCRIPT.get_or_init(|| {
        std::env::var("NV_MOVE_MOUSE")
            .unwrap_or_default()
            .split(',')
            .filter_map(|item| {
                let mut part = item.trim().split(':');
                let frame = part.next()?.parse().ok()?;
                let dx = part.next()?.parse().ok()?;
                let dy = part.next()?.parse().ok()?;
                let hold = part.next().map_or(Some(1), |h| h.parse().ok())?;
                let buttons = part.next().map_or(Some(0), |b| b.parse().ok())?;
                Some((frame, dx, dy, hold, buttons))
            })
            .collect()
    })
}

impl crate::Host {
    /// One frame of the game's own main loop when its `main` runs as the main thread
    /// ([`PlayerMovement::main_frame`]); false when it doesn't run or stopped.
    pub fn main_frame(&mut self) -> bool {
        self.join();
        self.movement.main_frame(&mut self.engine)
    }

    /// Hands the engine to the game's `main` for its next frame and returns at once (the
    /// frame runs on main's OS thread beside the caller, [`PlayerMovement::begin_main_frame`]).
    /// Every use of the engine through the host waits for it first ([`crate::Host::join`]);
    /// [`crate::Host::finish_main_frame`] takes it back. The engine itself (`Host::engine`)
    /// must not be touched in between.
    pub fn begin_main_frame(&mut self) -> bool {
        self.movement.begin_main_frame(&mut self.engine)
    }

    /// Waits for the frame [`crate::Host::begin_main_frame`] handed over; true when it ended a
    /// frame, false without one in flight.
    pub fn finish_main_frame(&mut self) -> bool {
        self.movement.finish_main_frame(&mut self.engine)
    }

    /// Waits for a frame of main's in flight, if there is one: every use of the engine
    /// through the host does this first.
    pub fn join(&mut self) {
        if self.movement.frame_in_flight() {
            self.finish_main_frame();
        }
    }

    /// The frame's move of the player through the game's code
    /// ([`PlayerMovement::step`]); `None`: the caller moves the player itself.
    pub fn move_player(
        &mut self,
        order: &LoadOrder,
        cell: Option<FormId>,
        input: &MoveInput,
    ) -> Option<MoveOutput> {
        // Before the movement's stages exist (never, while the game's `main` runs) the step
        // doesn't touch the engine: a frame in flight goes on.
        if self.movement.stages_touch_engine() {
            self.join();
        }
        self.movement.step(&mut self.engine, order, cell, input)
    }

    /// The viewer's keyboard for the game's DirectInput keyboard
    /// (`engine::units::platform::dinput::key`): `down` holds every DirectInput key code (the
    /// scan code) down this frame; each change becomes a key event.
    pub fn set_keys_down(&mut self, down: &[u8]) {
        self.join();
        let script = key_script();
        let frame = self.movement.main_frames;
        // The real game window's keyboard (`NV_D3D9=real`, Windows) holds keys too, unless a
        // script drives the run ([`real_input_reaches_game`]).
        #[cfg(windows)]
        let window_keys = if real_input_reaches_game() {
            engine::units::platform::d3d9real::held_keys()
        } else {
            [false; 256]
        };
        #[cfg(not(windows))]
        let window_keys = [false; 256];
        for code in 0..=255u8 {
            let scripted = script
                .iter()
                .any(|&(at, c, hold)| c == code && frame >= at && frame < at + hold);
            let held = down.contains(&code) || scripted || window_keys[code as usize];
            engine::units::platform::dinput::key(&mut self.engine, code, held);
        }
    }

    /// The viewer's mouse for the game's DirectInput mouse: motion since the last frame
    /// (`dinput::mouse_move`) and the buttons down (`dinput::mouse_button`).
    pub fn mouse(&mut self, motion: [i32; 3], buttons: [bool; 8]) {
        self.join();
        // The real game window's mouse (`NV_D3D9=real`, Windows) moves and clicks too.
        #[cfg(windows)]
        let (motion, buttons) = {
            let w = engine::units::platform::d3d9real::take_input();
            let mut b = buttons;
            for (held, window) in b.iter_mut().zip(w.buttons) {
                *held |= window;
            }
            (
                [
                    motion[0] + w.motion[0],
                    motion[1] + w.motion[1],
                    motion[2] + w.motion[2],
                ],
                b,
            )
        };
        // A scripted run (`NV_MOVE_KEYS`) drives the menus with keys; the real mouse moving over
        // the window would switch the game's interface to its mouse path and the keys would miss.
        // With `NV_MOVE_REAL_INPUT=1` the mouse is the player's again once the last scripted key
        // is released; otherwise a scripted run never takes it ([`real_input_reaches_game`]).
        let scripted_until = key_script()
            .iter()
            .map(|&(at, _, hold)| at + hold)
            .max()
            .unwrap_or(0);
        let (mut motion, mut buttons) =
            if !real_input_reaches_game() || self.movement.main_frames < scripted_until {
                ([0; 3], [false; 8])
            } else {
                (motion, buttons)
            };
        let frame = self.movement.main_frames;
        for &(at, dx, dy, hold, mask) in mouse_script() {
            if frame >= at && frame < at + hold {
                motion[0] += dx;
                motion[1] += dy;
                for (bit, held) in buttons.iter_mut().enumerate() {
                    *held |= mask & (1 << bit) != 0;
                }
            }
        }
        let e = &mut self.engine;
        engine::units::platform::dinput::mouse_move(e, motion[0], motion[1], motion[2]);
        for (button, down) in buttons.into_iter().enumerate() {
            engine::units::platform::dinput::mouse_button(e, button, down);
        }
    }

    /// The frame's AI linear task threads, with the Havok world step on
    /// thread 2, through the game's code ([`PlayerMovement::run_ai_threads`]);
    /// `None`: the caller steps its own bodies.
    pub fn run_ai_threads(&mut self, seconds: f32) -> Option<Vec<BodyPose>> {
        if self.movement.stages_touch_engine() {
            self.join();
        }
        self.movement.run_ai_threads(&mut self.engine, seconds)
    }

    /// The static initializers start-up skipped (`static_init` in
    /// [`crate::Host::start`]), one row each: the entry, why (`open`: not
    /// translated; `callee`: a callee isn't; `stopped`: stopped part-way),
    /// the untranslated callee, and the global the entry makes (its table
    /// row's object or register global; empty for others). The movement
    /// path reads globals these make (the process lists, `011e0e80`, among
    /// them), so a fault in translated code there can come from a skipped
    /// one.
    pub fn static_skips_tsv(&self) -> String {
        use engine::static_init::Row;
        let rows = engine::static_init::table();
        let global = |entry: u32| match rows.get(&entry) {
            Some(Row::Object { this, .. }) => format!("{this:08x}"),
            Some(Row::Reg { global, .. }) => format!("{global:08x}"),
            _ => String::new(),
        };
        let report = &self.static_init;
        let mut out = String::from("entry\twhy\tcallee\tglobal\tstop\n");
        for &entry in &report.open {
            let _ = writeln!(out, "{entry:08x}\topen\t\t{}\t", global(entry));
        }
        for &(entry, callee) in &report.callee_open {
            let _ = writeln!(
                out,
                "{entry:08x}\tcallee\t{callee:08x}\t{}\t",
                global(entry)
            );
        }
        for (entry, stop) in &report.stopped {
            let stop = stop.lines().next().unwrap_or("");
            let _ = writeln!(out, "{entry:08x}\tstopped\t\t{}\t{stop}", global(*entry));
        }
        out
    }
}

/// `InitTES`'s head (`0086cf81`..`0086d076`): the sky, `InitSky`, the two scene-graph
/// getters, the helper chain when `011dea14` holds a node, `TES::TES` into `011dea10`, then
/// `00450c50` on it. Its scope object and log lines don't run.
/// `main` between `Main::Main` and `InitSceneGraph` (`0086afa4`..`0086b066`), in its order:
/// the setting guard's scope around the object `007005d0` makes into `011d8804` (null when
/// `operator new` fails), `Error` (`0040fbe0`), the renderer `0086d500` on `Main`, the byte
/// `011f4509` set, then `004a0370`, `0086bbd0(0x5d4a40, 0)` and `0086bae0(0x872570, 0)` each on
/// what `0043c4b0` answers, and `0086bf70`.
fn run_main_up_to_scene_graph(e: &mut Engine, main: u32) {
    e.with_stack(4, |e, guard| {
        let guard = guard.addr();
        e.call(
            SETTING_GUARD_ENTER,
            &[guard, 0xd, 1, MAIN_SOURCE_NAME, 0x9f8],
        );
        let block = e.call(NEW, &[8]).u32();
        let object = if block == 0 {
            0
        } else {
            e.call(MAKE_011D8804, &[block]).u32()
        };
        e.mem.set_u32(OBJECT_011D8804, object);
        e.call(SETTING_GUARD_LEAVE, &[guard]);
    });
    e.call(ERROR_0040FBE0, &[ERROR_TEXT_0108295C]);
    e.call(INIT_RENDERER, &[main]);
    e.mem.set_u8(RENDERER_MADE_011F4509, 1);
    let owner = e.call(GETTER_0043C4B0, &[]).u32();
    e.call(CALL_004A0370, &[owner]);
    let owner = e.call(GETTER_0043C4B0, &[]).u32();
    e.call(CALL_0086BBD0, &[owner, 0x005d_4a40, 0]);
    let owner = e.call(GETTER_0043C4B0, &[]).u32();
    e.call(CALL_0086BAE0, &[owner, 0x0087_2570, 0]);
    e.call(CALL_0086BF70, &[]);
}

fn make_tes(e: &mut Engine) -> Result<(), String> {
    let sky = e.call(SKY_GET_INSTANCE, &[]).u32();
    e.mem.set_u32(SKY_OBJECT, sky);
    e.call(INIT_SKY, &[]);
    let object_lod_root = e.call(OBJECT_LOD_ROOT, &[]).u32();
    let mut helper = e.call(LOD_ROOT, &[]).u32();
    if helper != 0 {
        let sky = e.mem.u32(SKY_OBJECT);
        let result = e.call(SKY_HELPER, &[sky]).u32();
        e.call(HELPER_ATTACH, &[result, helper]);
        let object = e.call(HELPER_FIND, &[helper, 0]).u32();
        helper = if object != 0 {
            e.vcall(object, 0xc, &[]).u32()
        } else {
            0
        };
    }
    let block = e.call(NEW, &[TES_SIZE]).u32();
    let tes = if block == 0 {
        0
    } else {
        let sky = e.mem.u32(SKY_OBJECT);
        let water_lod = e.mem.u32(WATER_LOD_NODE);
        e.call(
            TES_CONSTRUCT,
            &[
                block,
                TES_DATA_PATH,
                object_lod_root,
                helper,
                sky,
                water_lod,
            ],
        )
        .u32()
    };
    e.mem.set_u32(TES_OBJECT, tes);
    let tes = e.mem.u32(TES_OBJECT);
    e.call(TES_AFTER, &[tes]);
    Ok(())
}

/// The AI linear task threads as `main` makes them (the module, stage
/// `threads`): `iNumHWThreads` set from `GetSystemInfo`'s processor count,
/// 1 made 2 (`0086a950`..`0086a990`; `main`'s INI path set-up `00876ad0`
/// between them doesn't run), then, with more than one, the manager
/// (`00713d80`) and its `CreateThreads` (`008c7290`, `0086b117`..`0086b12d`):
/// its stage events and, with three or more, threads 1 and 2 (else the one
/// combined thread `008c7bd0`), each a `BSThread` the thread host runs
/// ([`engine::thread_host`]). Answers the manager (0 when no threads).
fn start_ai_threads(e: &mut Engine) -> Result<u32, String> {
    let processors = e.with_stack(SYSTEM_INFO_SIZE, |e, info| {
        e.call(GET_SYSTEM_INFO, &[info.addr()]);
        e.mem.u32(info.addr() + SYSTEM_INFO_PROCESSORS)
    });
    let setting = e.call(SETTING_INT_PTR, &[NUM_HW_THREADS]).u32();
    e.mem.set_u32(setting, processors);
    let setting = e.call(SETTING_INT_PTR, &[NUM_HW_THREADS]).u32();
    if e.mem.i32(setting) == 1 {
        e.mem.set_u32(setting, 2);
    }
    let setting = e.call(SETTING_INT_PTR, &[NUM_HW_THREADS]).u32();
    if e.mem.i32(setting) <= 1 {
        return Ok(0);
    }
    let manager = e.call(AI_THREAD_MANAGER, &[]).u32();
    e.call(CREATE_AI_THREADS, &[manager]);
    Ok(manager)
}

/// The game's `SetPosition` (vtable +0x2A8) with a point on the game's stack.
fn set_position(e: &mut Engine, reference: u32, feet: [f32; 3]) {
    e.with_stack(12, |e, p| {
        for (i, v) in feet.iter().enumerate() {
            e.mem.set_f32(p.addr() + 4 * i as u32, *v);
        }
        e.vcall(reference, SET_POSITION, &[p.addr()]);
    });
}

/// `TESObjectREFR::SetAngleOnReference` (`0086d490`) with (0, 0, heading).
fn set_heading(e: &mut Engine, reference: u32, heading: f32) {
    e.with_stack(12, |e, p| {
        e.mem.set_f32(p.addr(), 0.0);
        e.mem.set_f32(p.addr() + 4, 0.0);
        e.mem.set_f32(p.addr() + 8, heading);
        e.call(SET_ANGLE, &[reference, p.addr()]);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A guarded run records each untranslated address with its nearest
    /// caller and the stage, stops at the first, and comes back as `Err`;
    /// the list puts the most reached first.
    #[test]
    fn misses_are_recorded_with_their_caller_and_counted() {
        let mut e = Engine::new();
        e.register(0x00aa_0000, |e, _| e.call(0x00ab_cdef, &[]));
        let mut m = PlayerMovement::default();
        for _ in 0..3 {
            let out = m.guarded(&mut e, "frame", |e| {
                e.call(0x00aa_0000, &[]);
                Ok(())
            });
            assert!(out.unwrap_err().starts_with("open function 00abcdef"));
        }
        let _ = m.guarded(&mut e, "cell", |e| {
            e.call(0x00ab_0001, &[]);
            Ok(())
        });
        let miss = &m.misses[&0x00ab_cdef];
        assert_eq!(miss.count, 3);
        assert_eq!(miss.stage, "frame");
        assert_eq!(miss.callers[&0x00aa_0000], 3);
        assert!(m.stops.is_empty());
        let tsv = m.missing_tsv(&|_| None);
        let lines: Vec<&str> = tsv.lines().collect();
        assert_eq!(lines[1], "00abcdef\t3\tframe\t00aa0000x3\t0\t\t");
        assert_eq!(lines[2], "00ab0001\t1\tcell\t00000000x1\t1\t\t");
        assert!(e.on_open.is_none() && e.call_stack.is_empty());
    }

    /// A probe run goes on past a miss with a zero result, records every
    /// miss; `step` never moves the player in a probe run.
    #[test]
    fn a_probe_goes_on_past_misses() {
        let mut e = Engine::new();
        let mut m = PlayerMovement {
            probe: true,
            ..PlayerMovement::default()
        };
        let out = m.guarded(&mut e, "frame", |e| {
            let a = e.call(0x00ab_0001, &[]).u32();
            let b = e.call(0x00ab_0002, &[]).u32();
            Ok(a + b)
        });
        assert_eq!(out, Ok(0));
        assert_eq!(m.misses.len(), 2);
    }

    /// Without the threads and cell stages the AI threads don't run and the
    /// frame is counted as the viewer's.
    #[test]
    fn the_ai_threads_wait_for_their_stages() {
        let mut e = Engine::new();
        let mut m = PlayerMovement::default();
        assert_eq!(m.run_ai_threads(&mut e, 0.016), None);
        assert_eq!((m.world_frames, m.world_fallback_frames), (0, 1));
        assert!(m
            .summary()
            .contains("0 world steps by the game's code (AI thread 2), 1 by crates/physics"));
    }

    /// Start-up: `Main::Main` with the window and the module handle into `011dea0c`, the
    /// renderer point, `InitSceneGraph` on `Main`, then `InitTES`'s head: the sky into
    /// `011dea20`, `InitSky`, the two getters, `TES::TES` (no helper chain when `011dea14` is
    /// empty) into `011dea10` and `00450c50` on it. A stop in the scene graph leaves `tes`
    /// not tried.
    #[test]
    fn start_up_runs_main_the_scene_graph_and_tes_in_mains_order() {
        let mut e = Engine::new();
        e.mem.map(MAIN_OBJECT & !0xfff, 0x1000);
        e.register(NEW, |_, a| {
            engine::abi::RetVal::into_ret(0x0200_0000 + a[0])
        });
        e.register(MAIN_CONSTRUCT, |_, a| engine::abi::RetVal::into_ret(a[0]));
        for f in [INIT_SCENE_GRAPH, INIT_SKY, TES_AFTER] {
            e.register(f, |_, _| Ret::default());
        }
        // `main` between `Main::Main` and the scene graph (stage `renderer`).
        e.mem.map(OBJECT_011D8804 & !0xfff, 0x1000);
        e.mem.map(RENDERER_MADE_011F4509 & !0xfff, 0x1000);
        for f in [
            SETTING_GUARD_ENTER,
            SETTING_GUARD_LEAVE,
            ERROR_0040FBE0,
            INIT_RENDERER,
            CALL_004A0370,
            CALL_0086BBD0,
            CALL_0086BAE0,
            CALL_0086BF70,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(MAKE_011D8804, |_, a| engine::abi::RetVal::into_ret(a[0]));
        e.register(GETTER_0043C4B0, |_, _| {
            engine::abi::RetVal::into_ret(0x0300_0000u32)
        });
        e.register(SKY_GET_INSTANCE, |_, _| {
            engine::abi::RetVal::into_ret(0x5000u32)
        });
        e.register(OBJECT_LOD_ROOT, |_, _| {
            engine::abi::RetVal::into_ret(0x6000u32)
        });
        e.register(LOD_ROOT, |_, _| Ret::default());
        e.register(TES_CONSTRUCT, |_, a| engine::abi::RetVal::into_ret(a[0]));
        e.mem.set_u32(WATER_LOD_NODE, 0x7000);
        e.call_log = Some(vec![]);
        let mut m = PlayerMovement::default();
        let renderer_point = Rc::new(RefCell::new(false));
        let seen = renderer_point.clone();
        m.start_up(&mut e, |_| *seen.borrow_mut() = true);
        assert!(*renderer_point.borrow());
        let log = e.call_log.take().unwrap();
        let order: Vec<(u32, Vec<u32>)> = log
            .into_iter()
            .filter(|(a, _)| {
                [
                    MAIN_CONSTRUCT,
                    INIT_SCENE_GRAPH,
                    SKY_GET_INSTANCE,
                    INIT_SKY,
                    OBJECT_LOD_ROOT,
                    LOD_ROOT,
                    TES_CONSTRUCT,
                    TES_AFTER,
                ]
                .contains(a)
            })
            .collect();
        let main = 0x0200_0000 + MAIN_SIZE;
        let tes = 0x0200_0000 + TES_SIZE;
        assert_eq!(order[0].0, MAIN_CONSTRUCT);
        assert_eq!(order[0].1[0], main);
        assert_eq!(order[0].1[2], IMAGE_BASE);
        assert_ne!(order[0].1[1], 0, "the window handle");
        assert_eq!(
            order[1..].to_vec(),
            vec![
                (INIT_SCENE_GRAPH, vec![main]),
                (SKY_GET_INSTANCE, vec![]),
                (INIT_SKY, vec![]),
                (OBJECT_LOD_ROOT, vec![]),
                (LOD_ROOT, vec![]),
                (
                    TES_CONSTRUCT,
                    vec![tes, TES_DATA_PATH, 0x6000, 0, 0x5000, 0x7000]
                ),
                (TES_AFTER, vec![tes]),
            ]
        );
        assert_eq!(e.mem.u32(MAIN_OBJECT), main);
        assert_eq!(e.mem.u32(SKY_OBJECT), 0x5000);
        assert_eq!(e.mem.u32(TES_OBJECT), tes);
        assert_eq!(m.havok, Stage::Done(()));
        assert_eq!(e.mem.u32(OBJECT_011D8804), 0x0200_0008);
        assert_eq!(e.mem.u8(RENDERER_MADE_011F4509), 1);
        assert_eq!(m.renderer, Stage::Done(()));
        // A stop in the scene graph: recorded, and `tes` isn't tried.
        let mut m = PlayerMovement::default();
        e.register(INIT_SCENE_GRAPH, |e, _| e.call(0x00ab_cdef, &[]));
        m.start_up(&mut e, |_| {});
        assert_eq!(m.misses[&0x00ab_cdef].stage, "scenegraph");
        assert_eq!(m.havok, Stage::NotTried);
    }

    /// `renderer`: `main`'s calls from `0086afa4` to `0086b066`, in its order, with its words.
    #[test]
    fn main_up_to_the_scene_graph_runs_in_mains_order() {
        let mut e = Engine::new();
        e.mem.map(OBJECT_011D8804 & !0xfff, 0x1000);
        e.mem.map(RENDERER_MADE_011F4509 & !0xfff, 0x1000);
        let glue = [
            SETTING_GUARD_ENTER,
            SETTING_GUARD_LEAVE,
            ERROR_0040FBE0,
            INIT_RENDERER,
            CALL_004A0370,
            CALL_0086BBD0,
            CALL_0086BAE0,
            CALL_0086BF70,
        ];
        for f in glue {
            e.register(f, |_, _| Ret::default());
        }
        e.register(NEW, |_, _| Ret::default());
        e.register(GETTER_0043C4B0, |_, _| {
            engine::abi::RetVal::into_ret(0x0300_0000u32)
        });
        e.mem.set_u32(OBJECT_011D8804, 0x55);
        e.call_log = Some(vec![]);
        run_main_up_to_scene_graph(&mut e, 0x0200_0000);
        let log = e.call_log.take().unwrap();
        let calls: Vec<(u32, Vec<u32>)> = log
            .into_iter()
            .filter(|(a, _)| glue.contains(a) || *a == NEW || *a == MAKE_011D8804)
            .collect();
        let guard = calls[0].1[0];
        assert_eq!(
            calls,
            vec![
                (
                    SETTING_GUARD_ENTER,
                    vec![guard, 0xd, 1, MAIN_SOURCE_NAME, 0x9f8]
                ),
                (NEW, vec![8]),
                (SETTING_GUARD_LEAVE, vec![guard]),
                (ERROR_0040FBE0, vec![ERROR_TEXT_0108295C]),
                (INIT_RENDERER, vec![0x0200_0000]),
                (CALL_004A0370, vec![0x0300_0000]),
                (CALL_0086BBD0, vec![0x0300_0000, 0x005d_4a40, 0]),
                (CALL_0086BAE0, vec![0x0300_0000, 0x0087_2570, 0]),
                (CALL_0086BF70, vec![]),
            ]
        );
        // `operator new` failed: null stored, `007005d0` not run.
        assert_eq!(e.mem.u32(OBJECT_011D8804), 0);
        assert_eq!(e.mem.u8(RENDERER_MADE_011F4509), 1);
    }

    /// `threads`: `iNumHWThreads` from the processor count (1 made 2), then
    /// the manager's `CreateThreads`.
    #[test]
    fn the_threads_stage_sets_the_thread_count_and_creates_the_threads() {
        for (processors, threads) in [(1u32, 2u32), (8, 8)] {
            let mut e = Engine::new();
            e.mem.map(NUM_HW_THREADS & !0xfff, 0x1000);
            e.mem.map(0x2000, 0x1000);
            e.mem.set_u32(0x2000, processors);
            e.register(GET_SYSTEM_INFO, |e, a| {
                let processors = e.mem.u32(0x2000);
                e.mem.set_u32(a[0] + SYSTEM_INFO_PROCESSORS, processors);
                Ret::default()
            });
            e.register(SETTING_INT_PTR, |_, a| engine::abi::RetVal::into_ret(a[0]));
            e.register(AI_THREAD_MANAGER, |_, _| {
                engine::abi::RetVal::into_ret(0x2100u32)
            });
            e.register(CREATE_AI_THREADS, |e, a| {
                e.mem.set_u32(0x2004, a[0]);
                Ret::default()
            });
            assert_eq!(start_ai_threads(&mut e), Ok(0x2100));
            assert_eq!(e.mem.u32(NUM_HW_THREADS), threads);
            assert_eq!(e.mem.u32(0x2004), 0x2100);
        }
    }

    /// A frame: the timer's stand-in (`SetDeltaTime`, the scaled seconds at
    /// `011dea30`), `008c80e0(1)`, `StartThreads` and `WaitForThreads` on the
    /// manager, in that order; then each reference's root node is read. A
    /// stop is recorded under `world` and not tried again.
    #[test]
    fn a_frame_starts_and_waits_for_the_ai_threads_and_reads_the_nodes() {
        let mut e = Engine::new();
        e.mem.map(FRAME_SECONDS & !0xfff, 0x1000);
        e.mem.map(0x2000, 0x1000);
        e.mem.set_f32(0x2008, 2.0);
        e.register(HAVOK_STEP_FLAG, |_, _| Ret::default());
        e.register(SET_DELTA_TIME, |e, a| {
            e.mem.set_u32(0x0000_2000, a[0]);
            Ret::default()
        });
        e.register(SETTING_FLOAT_PTR, |_, _| {
            engine::abi::RetVal::into_ret(0x2008u32)
        });
        for f in [
            AI_THREADS_RENDER_SAFE,
            START_AI_THREADS,
            WAIT_FOR_AI_THREADS,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        // A reference whose Get3D (+0x1D0) answers a root node at 0x2400.
        let vtable = 0x2100;
        e.mem.set_u32(0x2200, vtable);
        e.mem.set_u32(vtable + GET_3D, 0x00ab_0000);
        e.register(0x00ab_0000, |_, _| engine::abi::RetVal::into_ret(0x2400u32));
        for i in 0..9 {
            e.mem.set_f32(
                0x2400 + WORLD_ROTATION + 4 * i,
                if i % 4 == 0 { 1.0 } else { 0.0 },
            );
        }
        e.mem.set_f32(0x2400 + WORLD_TRANSLATION, 5.0);
        let mut m = PlayerMovement {
            threads: Stage::Done(0x2600),
            cell: Some((FormId(0x1234), Stage::Done(0x3000))),
            cell_refs: vec![(FormId(0x99), 0x2200)],
            ..PlayerMovement::default()
        };
        e.call_log = Some(vec![]);
        let poses = m.run_ai_threads(&mut e, 0.5).expect("ran");
        let log = e.call_log.take().unwrap();
        let order: Vec<(u32, Vec<u32>)> = log
            .into_iter()
            .filter(|(a, _)| {
                [
                    AI_THREADS_RENDER_SAFE,
                    START_AI_THREADS,
                    WAIT_FOR_AI_THREADS,
                ]
                .contains(a)
            })
            .collect();
        assert_eq!(
            order,
            vec![
                (AI_THREADS_RENDER_SAFE, vec![1]),
                (START_AI_THREADS, vec![0x2600]),
                (WAIT_FOR_AI_THREADS, vec![0x2600]),
            ]
        );
        assert_eq!(e.mem.f32(0x2000), 0.5);
        assert_eq!(e.mem.f32(FRAME_SECONDS), 1.0);
        assert_eq!(poses.len(), 1);
        assert_eq!(poses[0].reference, FormId(0x99));
        assert_eq!(poses[0].translation, [5.0, 0.0, 0.0]);
        assert_eq!(poses[0].rotation[1], [0.0, 1.0, 0.0]);
        assert_eq!(m.world_frames, 1);
        // A stop: recorded, and the frame isn't tried again.
        e.register(WAIT_FOR_AI_THREADS, |e, _| e.call(0x00ab_cdef, &[]));
        assert_eq!(m.run_ai_threads(&mut e, 0.5), None);
        assert_eq!(m.misses[&0x00ab_cdef].stage, "world");
        assert_eq!(m.run_ai_threads(&mut e, 0.5), None);
        assert_eq!(m.misses[&0x00ab_cdef].count, 1);
        assert_eq!(m.world_fallback_frames, 2);
    }

    /// A stop that isn't a missing function (memory nobody made) is kept
    /// with its stage.
    #[test]
    fn other_stops_are_kept_by_stage() {
        let mut e = Engine::new();
        let mut m = PlayerMovement::default();
        let _ = m.guarded(&mut e, "frame", |e| Ok(e.mem.u32(0x0000_1000)));
        assert_eq!(m.stops.len(), 1);
        let ((stage, why), n) = m.stops.iter().next().unwrap();
        assert_eq!((*stage, *n), ("frame", 1));
        assert!(why.contains("00001000"), "{why}");
    }
}
