//! The frame's world and time stage (steps 48-78 of [`super::STEPS`],
//! [`super::Stage::WorldAndTime`]): the calls inside the functions that
//! `Main::OnIdle` (`0086e650`) calls there, in the exe's order, with the
//! tests that decide which of them run (docs/FRAME_SKELETON.md, "PR 5
//! result").
//!
//! [`FUNCTIONS`] holds one [`Function`] per modelled callee of the stage:
//! `TES::TestAllCells` (`004556d0`), `Calendar::Update` (`00867a40`),
//! `TES::RunAnimations` (`00455640`), `ProcessLists::RunActorScripts`
//! (`00978550`), `ProcessLists::UpdateRadiationList` (`009777a0`),
//! `ProcessLists::ChangeProcessLevelTempList` (`0096eb40`),
//! `ProcessLists::UpdateFollowerTempList` (`0096e9b0`),
//! `GarbageCollector::Update` (`00868850`) and `ClearTempEffects`
//! (`00868d10`), `BSTreeManager::Update` (`006652e0`),
//! `Main::OnIdle_UpdateCurrentGridCell` (`0086fbe0`) and the
//! `TES::UpdateCurrentGridCell` (`00452580`) it calls, which attaches and
//! detaches the exterior grid's cells. Names are Xbox PDB names (ADR-0002)
//! from the engine map; where the map names a function only through the
//! linker's folding of identical code, the name is left out.
//!
//! Each function's [`SubStep`]s are not each of its call instructions
//! (many are queries: getters, settings, list nodes, the frame time) but the
//! calls that do its work, in the order of their call sites, each with its
//! [`SubGate`] (a test of the function's own branches on a [`WorldState`])
//! and, where the call depends on further tests of its own block (an entry
//! of a list, a per-actor value), those tests' branches as
//! [`SubStep::own_tests`], not modelled. A step without own tests is called
//! whenever its gate holds and its function is called (its frame step's
//! gate, [`super::Gate`]).
//!
//! The rest of the stage's steps are queries and locks of `0086e650`
//! itself: the parallel update's manager and begin (`00683a60`, `00a81a20`,
//! a lead), the cell tests' flags (`00451530`, `0086ef70`), the "World"
//! scene graph's pointer (`00559450` on `011deb7c`) and the frame time
//! (`0084d030`) for its slot +0x104, the thread count (`0043d4d0`), the
//! process lists' lock around the lists (`0040fbf0`/`0040fba0` on
//! `011f11a0`), the memory manager's getter and its call (`00446ef0(1)`,
//! `00878080`), the texture purge request (`0086ef60`) and
//! `BSTexturePalette::PurgeUnusedTextures` (`00a61cd0`), and the scene
//! graph's camera for the tree manager (`00524c90`). One call of the stage
//! is not a direct call and so not in `STEPS`: [`SCENE_GRAPH_UPDATE`].
//!
//! The model takes one value of each input for the whole stage. The
//! disassembly is the source for every branch and call site named here;
//! where the engine crate has a translation (all but `006652e0`), its tests
//! drive it and check its calls against this model (`world` is a
//! dev-dependency of the engine crate).
//!
//! The viewer orders the stage's systems by these sets
//! (`viewer/src/frame_order.rs`, Phase 1 PR 5).

// Translated from 004556d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 00867a40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 00455640 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 00978550 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 009777a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 0096eb40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 0096e9b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 00868850 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 00868d10 (decompiled, FalloutNV.exe 1.4.0.525)
// BSTreeManager::Update (006652e0): its control flow read from the disassembly
// (FalloutNV.exe 1.4.0.525); the engine crate has no translation of it yet.
// Translated from 0086fbe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translated from 00452580 (decompiled, FalloutNV.exe 1.4.0.525)

use super::Wiring;

/// `Main::OnIdle` (Xbox PDB).
pub const ON_IDLE: u32 = 0x0086_e650;

/// The call of the stage that isn't a direct call: slot +0x104 of the
/// "World" scene graph (`[011deb7c]`, the `SceneGraph` `Main::InitSceneGraph`
/// makes with the name "World", vtable `01083b5c`) with the frame time, at
/// `0086e9b5`, between steps 54 and 55, under the world-runs gate
/// ([`super::Gate::WorldRuns`]). The slot (`01083c60`) holds `00c52590`,
/// `BSSceneGraph::SetViewDistanceBasedOnFrameRate`; it is not the sky's
/// update, which `TES::UpdateCellAnimations` (`004536ae`) and
/// `TES::UpdateCellMainThread` (`00453811`) call in stage 6.
pub const SCENE_GRAPH_UPDATE: SceneGraphUpdate = SceneGraphUpdate {
    site: 0x0086_e9b5,
    slot: 0x104,
    function: 0x00c5_2590,
    after_step: 53,
};

/// [`SCENE_GRAPH_UPDATE`]'s fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SceneGraphUpdate {
    /// The call instruction in `0086e650`.
    pub site: u32,
    /// The vtable slot.
    pub slot: u32,
    /// What the slot holds.
    pub function: u32,
    /// The index in [`super::STEPS`] of the step it follows (the frame
    /// time's read, `0084d030` at `0086e99e`).
    pub after_step: usize,
}

/// Whom a sub-step calls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Callee {
    /// A direct call of this PC address.
    Direct(u32),
    /// A call through this byte offset of the object's vtable.
    Virtual(u32),
    /// A call through this import table slot.
    Import(u32),
    /// A call of the function pointer stored at this offset of `this`.
    Pointer(u32),
}

/// What a function tests before a sub-step. [`SubGate::holds`] evaluates
/// it on a [`WorldState`], [`SubGate::branches`] names the branch
/// instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SubGate {
    /// No test.
    Always,
    // `TES::TestAllCells(0)` (`004556d0`), called while
    // `bRunningCellTests` (`TES` +0x51) is set, so its turn runs
    // (`004559a7`).
    /// The next world space is entered: the "enter" byte `[011c3ee4]`
    /// (`00455a2c`), a world space list node `[011c3eec]` (`00455a39`) and a
    /// world space found whose name doesn't start with "test" (`00455ab1`).
    TestEnterWorldSpace,
    /// The exterior walk: not entering (`00455ba3`) and a world space node
    /// (`00455bbc`).
    TestExteriorWalk,
    /// The interior walk: not entering, no world space node (`00455bbc`).
    TestInteriorWalk,
    /// A cell was found (`00455d5e`, `00455d66`; the region filter at
    /// `00455d7f`-`00455da1`) and is visited (`00455dd7`).
    TestVisit,
    /// The visited cell is an interior (`00455ea3`).
    TestVisitInterior,
    /// An exterior visited while an interior is loaded (`00455ea3`,
    /// `00455eea`).
    TestVisitExteriorFromInterior,
    /// An exterior visited from the exterior (`00455ea3`, `00455eea`).
    TestVisitExterior,
    /// The visit's line printed: the logging byte `[0118607c]` (`004560c4`)
    /// for an interior (`00456204`).
    TestLogInterior,
    /// The same for an exterior (`00456204`).
    TestLogExterior,
    /// The callback at `TES` +0x54 (`00456451`).
    TestCallback,
    /// The test's mode `[011c3ef0]` is 4 or 5 (`00456476`, `0045647f`).
    TestSaveGame,
    /// The interior index reached the interior count (`0045649f`): the run
    /// ends.
    TestDone,
    // `Calendar::Update` (`00867a40`).
    /// The days passed rewritten from the whole days and the hour: the
    /// calendar's flag +0x1c (`00867a9b`) or the hour more than one past
    /// the last update's (`00867ab3`).
    CalendarRewrite,
    /// The hour passed 24 (`00867b00`).
    CalendarMidnight,
    /// And the day passed the month's days (`00867b85`).
    CalendarMonthEnds,
    /// And the month reached 12 (`00867bb1`).
    CalendarYearEnds,
    // `TES::RunAnimations` (`00455640`).
    /// An interior cell is loaded (`TES` +0x34, `005f36f0`; `00455656`).
    Interior,
    /// No interior cell (`00455656`).
    Exterior,
    // `ProcessLists::UpdateRadiationList` (`009777a0`).
    /// The radiation sources' iterator (`009c1a50` on `[011c95c8]`) exists
    /// (`009777cb`) and has a first source (`0097780b`).
    RadiationSources,
    /// It exists without a source while `[011f12d8]` says the last frame had
    /// one (`009777e0`, `009777ef`, `009777fa`): every actor's level is
    /// reset (`00977b99`).
    RadiationEnded,
    // `ProcessLists::ChangeProcessLevelTempList` (`0096eb40`).
    /// The player is not sleeping or resting (`0096eb5c`).
    NotResting,
    // `GarbageCollector::Update` (`00868850`): the first queue with
    // entries is worked on.
    /// Queued animations (`008688bf`), the model loader's lock free
    /// (`008688d5`).
    GcAnimations,
    /// Else flag-set biped animations (`00868976`), the lock free
    /// (`0086898c`).
    GcBipedsSet,
    /// Else flag-clear bipeds (`00868a2d`): moved to the flag-set array.
    GcBipedsClear,
    /// Else flag-set 3D objects (`00868a5c`).
    GcObjects3dSet,
    /// Else flag-clear 3D objects (`00868ae6`).
    GcObjects3dClear,
    /// Else flag-set references (`00868b15`), the lock free (`00868b2b`).
    GcReferencesSet,
    /// Else flag-clear references (`00868c0c`).
    GcReferencesClear,
    /// Else flag-set navmeshes (`00868c3b`).
    GcNavMeshesSet,
    /// Else flag-clear navmeshes (`00868ca5`).
    GcNavMeshesClear,
    // `GarbageCollector::ClearTempEffects` (`00868d10`).
    /// Flag-set temporary effects queued (`00868d1f`) and the model
    /// loader's lock free (`00868d31`).
    TempEffects,
    // `BSTreeManager::Update(camera, stopped)` (`006652e0`), `stopped` being
    // menu mode or the frozen world ([`super::FrameState::tree_manager_argument`]).
    /// The world runs (`006652f4`): SpeedTree's clock advances.
    TreesWorldRuns,
    /// The scene graph has a camera (`00665322`).
    TreesCamera,
    /// The world runs (`006653a9`), the sky exists (`TES` and its +0x68,
    /// `006653b6`, `006653c9`) and no interior is loaded (`006653dc`): the
    /// wind's speed from the sky.
    TreesOutdoorSky,
    /// The world runs and the sky exists (`006653a9`, `006653b6`,
    /// `006653c9`).
    TreesSky,
    /// The world runs (`006653a9`): the wind update.
    TreesWind,
    // `Main::OnIdle_UpdateCurrentGridCell` (`0086fbe0`).
    /// Not menu mode (`0086fbf1`), no new game's loading menu (`[011d8907]`,
    /// set by the start menu's `ConfirmNewGame` at `007d3354`, cleared by the
    /// loading menu's destructor at `0078883c`; `0086fbfc`), the world not
    /// frozen (`0086fc07`).
    GridUpdate,
    /// And the cell tests are running (`00451530`, `0086fc32`).
    GridCellTests,
    /// And no interior is loaded (`0086fc41`).
    GridCellTestsExterior,
    // `TES::UpdateCurrentGridCell` (`00452580`).
    /// The data handler `[011c3f2c]` exists (`00452624`).
    GridData,
    /// And the grid's centre is unset (`0x7fffffff`, `00452637`,
    /// `00452643`): the area around the position is loaded at once.
    GridFirst,
    /// The position is still inside the grid's centre cell, both distances
    /// to its border positive and no script running (`004527a4`, `004527be`,
    /// `004527ca`).
    GridInside,
    /// The position crossed into another cell: the grid moves.
    GridCrossed,
    /// And no interior is loaded or a script runs (`00452b5a`, `00452b62`):
    /// the cells are detached and attached around the new centre.
    GridReload,
    /// A crossing with no interior loaded (`00452d75`): the terrain follows
    /// the player.
    GridCrossedExterior,
}

/// Which of the garbage collector's queues is the first with entries
/// (`00868850` tests them in this order).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Garbage {
    None,
    Animations,
    BipedsSet,
    BipedsClear,
    Objects3dSet,
    Objects3dClear,
    ReferencesSet,
    ReferencesClear,
    NavMeshesSet,
    NavMeshesClear,
}

/// Where the position `TES::UpdateCurrentGridCell` follows is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GridMove {
    /// The grid's centre isn't set yet.
    First,
    /// Still inside the centre cell.
    Inside,
    /// Crossed into another cell.
    Crossed,
}

/// The stage's inputs to the sub-gates. `Default` is a frame in game mode
/// outdoors (no interior loaded, the sky there), the player inside the
/// grid's centre cell, no cell test, no radiation, nothing queued for
/// garbage collection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WorldState {
    /// The world runs ([`super::FrameState::world_runs`]): menu mode clear
    /// and the world not frozen. `BSTreeManager::Update` gets its negation.
    pub world_runs: bool,
    /// Menu mode, `[011dea2b]` ([`super::FrameState::menu_flag`]).
    pub menu_flag: bool,
    /// `Main` +7.
    pub world_frozen: bool,
    /// `[011d8907]`: a new game's loading menu is up.
    pub new_game_loading: bool,
    /// `TES` +0x51 or +0x52 (`00451530`).
    pub cell_tests: bool,
    /// `TES` +0x34, the loaded interior cell (`005f36f0`).
    pub interior_loaded: bool,
    /// `TestAllCells`: the next world space is to be entered.
    pub test_enter_world_space: bool,
    /// `TestAllCells`: walking a world space's cells (else the interiors).
    pub test_exterior_walk: bool,
    /// `TestAllCells`: a cell with references that passes the region
    /// filter was found.
    pub test_cell_found: bool,
    /// That cell is an interior.
    pub test_cell_interior: bool,
    /// `[0118607c]`: the test prints a line per cell.
    pub test_logging: bool,
    /// `TES` +0x54 holds a callback.
    pub test_callback: bool,
    /// The test's mode is 4 or 5.
    pub test_save_mode: bool,
    /// The interior index reached the interior count.
    pub test_done: bool,
    /// `Calendar::Update` rewrites the days passed.
    pub calendar_rewrite: bool,
    /// The hour passes 24.
    pub midnight: bool,
    /// The day passes the month's days.
    pub month_ends: bool,
    /// The month reaches 12.
    pub year_ends: bool,
    /// The radiation sources' iterator has a source.
    pub radiation_sources: bool,
    /// It has none, after a frame that had one.
    pub radiation_ended: bool,
    /// `PlayerCharacter::IsSleepingorResting` (`0094df60`).
    pub resting: bool,
    /// The garbage collector's first queue with entries.
    pub garbage: Garbage,
    /// The model loader's lock can be taken (`00868250`).
    pub model_loader_free: bool,
    /// Flag-set temporary effects are queued.
    pub temp_effects: bool,
    /// The scene graph has a camera (`00524c90`).
    pub scene_camera: bool,
    /// The sky (`TES` +0x68).
    pub sky: bool,
    /// The data handler exists.
    pub data_handler: bool,
    /// Where the followed position is.
    pub grid: GridMove,
    /// A script runs (`0042ce10` on the script context `[011ddf38]`).
    pub script_running: bool,
}

impl Default for WorldState {
    fn default() -> WorldState {
        WorldState {
            world_runs: true,
            menu_flag: false,
            world_frozen: false,
            new_game_loading: false,
            cell_tests: false,
            interior_loaded: false,
            test_enter_world_space: false,
            test_exterior_walk: false,
            test_cell_found: false,
            test_cell_interior: false,
            test_logging: false,
            test_callback: false,
            test_save_mode: false,
            test_done: false,
            calendar_rewrite: false,
            midnight: false,
            month_ends: false,
            year_ends: false,
            radiation_sources: false,
            radiation_ended: false,
            resting: false,
            garbage: Garbage::None,
            model_loader_free: true,
            temp_effects: false,
            scene_camera: true,
            sky: true,
            data_handler: true,
            grid: GridMove::Inside,
            script_running: false,
        }
    }
}

impl SubGate {
    /// Whether the sub-step is reached, given that its function is called.
    pub fn holds(self, s: &WorldState) -> bool {
        let visit = s.test_cell_found;
        let exterior_visit = visit && !s.test_cell_interior;
        let grid_update = !s.menu_flag && !s.new_game_loading && !s.world_frozen;
        let crossed = s.data_handler && s.grid == GridMove::Crossed;
        let locked = s.model_loader_free;
        match self {
            SubGate::Always => true,
            SubGate::TestEnterWorldSpace => s.test_enter_world_space,
            SubGate::TestExteriorWalk => !s.test_enter_world_space && s.test_exterior_walk,
            SubGate::TestInteriorWalk => !s.test_enter_world_space && !s.test_exterior_walk,
            SubGate::TestVisit => visit,
            SubGate::TestVisitInterior => visit && s.test_cell_interior,
            SubGate::TestVisitExteriorFromInterior => exterior_visit && s.interior_loaded,
            SubGate::TestVisitExterior => exterior_visit && !s.interior_loaded,
            SubGate::TestLogInterior => visit && s.test_logging && s.test_cell_interior,
            SubGate::TestLogExterior => visit && s.test_logging && !s.test_cell_interior,
            SubGate::TestCallback => visit && s.test_callback,
            SubGate::TestSaveGame => visit && s.test_save_mode,
            SubGate::TestDone => s.test_done,
            SubGate::CalendarRewrite => s.calendar_rewrite,
            SubGate::CalendarMidnight => s.midnight,
            SubGate::CalendarMonthEnds => s.midnight && s.month_ends,
            SubGate::CalendarYearEnds => s.midnight && s.month_ends && s.year_ends,
            SubGate::Interior => s.interior_loaded,
            SubGate::Exterior => !s.interior_loaded,
            SubGate::RadiationSources => s.radiation_sources,
            SubGate::RadiationEnded => !s.radiation_sources && s.radiation_ended,
            SubGate::NotResting => !s.resting,
            SubGate::GcAnimations => s.garbage == Garbage::Animations && locked,
            SubGate::GcBipedsSet => s.garbage == Garbage::BipedsSet && locked,
            SubGate::GcBipedsClear => s.garbage == Garbage::BipedsClear,
            SubGate::GcObjects3dSet => s.garbage == Garbage::Objects3dSet,
            SubGate::GcObjects3dClear => s.garbage == Garbage::Objects3dClear,
            SubGate::GcReferencesSet => s.garbage == Garbage::ReferencesSet && locked,
            SubGate::GcReferencesClear => s.garbage == Garbage::ReferencesClear,
            SubGate::GcNavMeshesSet => s.garbage == Garbage::NavMeshesSet,
            SubGate::GcNavMeshesClear => s.garbage == Garbage::NavMeshesClear,
            SubGate::TempEffects => s.temp_effects && locked,
            SubGate::TreesWorldRuns | SubGate::TreesWind => s.world_runs,
            SubGate::TreesCamera => s.scene_camera,
            SubGate::TreesOutdoorSky => s.world_runs && s.sky && !s.interior_loaded,
            SubGate::TreesSky => s.world_runs && s.sky,
            SubGate::GridUpdate => grid_update,
            SubGate::GridCellTests => grid_update && s.cell_tests,
            SubGate::GridCellTestsExterior => grid_update && s.cell_tests && !s.interior_loaded,
            SubGate::GridData => s.data_handler,
            SubGate::GridFirst => s.data_handler && s.grid == GridMove::First,
            SubGate::GridInside => s.data_handler && s.grid == GridMove::Inside,
            SubGate::GridCrossed => crossed,
            SubGate::GridReload => crossed && (!s.interior_loaded || s.script_running),
            SubGate::GridCrossedExterior => crossed && !s.interior_loaded,
        }
    }

    /// The branch instructions that make up the test.
    pub fn branches(self) -> &'static [u32] {
        const VISIT: [u32; 3] = [0x0045_5d5e, 0x0045_5d66, 0x0045_5dd7];
        const GRID: [u32; 3] = [0x0086_fbf1, 0x0086_fbfc, 0x0086_fc07];
        const GRID_INSIDE: [u32; 6] = [
            0x0045_2624,
            0x0045_2637,
            0x0045_2643,
            0x0045_27a4,
            0x0045_27be,
            0x0045_27ca,
        ];
        match self {
            SubGate::Always => &[],
            SubGate::TestEnterWorldSpace => &[0x0045_5a2c, 0x0045_5a39, 0x0045_5ab1],
            SubGate::TestExteriorWalk | SubGate::TestInteriorWalk => &[0x0045_5ba3, 0x0045_5bbc],
            SubGate::TestVisit => &VISIT,
            SubGate::TestVisitInterior => &[0x0045_5dd7, 0x0045_5ea3],
            SubGate::TestVisitExteriorFromInterior | SubGate::TestVisitExterior => {
                &[0x0045_5dd7, 0x0045_5ea3, 0x0045_5eea]
            }
            SubGate::TestLogInterior | SubGate::TestLogExterior => {
                &[0x0045_5dd7, 0x0045_60c4, 0x0045_6204]
            }
            SubGate::TestCallback => &[0x0045_5dd7, 0x0045_6451],
            SubGate::TestSaveGame => &[0x0045_5dd7, 0x0045_6476, 0x0045_647f],
            SubGate::TestDone => &[0x0045_649f],
            SubGate::CalendarRewrite => &[0x0086_7a9b, 0x0086_7ab3],
            SubGate::CalendarMidnight => &[0x0086_7b00],
            SubGate::CalendarMonthEnds => &[0x0086_7b00, 0x0086_7b85],
            SubGate::CalendarYearEnds => &[0x0086_7b00, 0x0086_7b85, 0x0086_7bb1],
            SubGate::Interior | SubGate::Exterior => &[0x0045_5656],
            SubGate::RadiationSources => &[0x0097_77cb, 0x0097_780b],
            SubGate::RadiationEnded => &[
                0x0097_77cb,
                0x0097_77e0,
                0x0097_77ef,
                0x0097_77fa,
                0x0097_7b99,
            ],
            SubGate::NotResting => &[0x0096_eb5c],
            SubGate::GcAnimations => &[0x0086_88bf, 0x0086_88d5],
            SubGate::GcBipedsSet => &[0x0086_88bf, 0x0086_8976, 0x0086_898c],
            SubGate::GcBipedsClear => &[0x0086_8976, 0x0086_8a2d],
            SubGate::GcObjects3dSet => &[0x0086_8a2d, 0x0086_8a5c],
            SubGate::GcObjects3dClear => &[0x0086_8a5c, 0x0086_8ae6],
            SubGate::GcReferencesSet => &[0x0086_8ae6, 0x0086_8b15, 0x0086_8b2b],
            SubGate::GcReferencesClear => &[0x0086_8b15, 0x0086_8c0c],
            SubGate::GcNavMeshesSet => &[0x0086_8c0c, 0x0086_8c3b],
            SubGate::GcNavMeshesClear => &[0x0086_8c3b, 0x0086_8ca5],
            SubGate::TempEffects => &[0x0086_8d1f, 0x0086_8d31],
            SubGate::TreesWorldRuns => &[0x0066_52f4],
            SubGate::TreesCamera => &[0x0066_5322],
            SubGate::TreesOutdoorSky => &[0x0066_53a9, 0x0066_53b6, 0x0066_53c9, 0x0066_53dc],
            SubGate::TreesSky => &[0x0066_53a9, 0x0066_53b6, 0x0066_53c9],
            SubGate::TreesWind => &[0x0066_53a9],
            SubGate::GridUpdate => &GRID,
            SubGate::GridCellTests => &[0x0086_fbf1, 0x0086_fbfc, 0x0086_fc07, 0x0086_fc32],
            SubGate::GridCellTestsExterior => &[
                0x0086_fbf1,
                0x0086_fbfc,
                0x0086_fc07,
                0x0086_fc32,
                0x0086_fc41,
            ],
            SubGate::GridData => &[0x0045_2624],
            SubGate::GridFirst => &[0x0045_2624, 0x0045_2637, 0x0045_2643],
            SubGate::GridInside | SubGate::GridCrossed => &GRID_INSIDE,
            SubGate::GridReload => &[
                0x0045_27a4,
                0x0045_27be,
                0x0045_27ca,
                0x0045_2b5a,
                0x0045_2b62,
            ],
            SubGate::GridCrossedExterior => &[0x0045_27a4, 0x0045_27be, 0x0045_27ca, 0x0045_2d75],
        }
    }
}

/// One sub-step of a [`Function`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubStep {
    pub callee: Callee,
    /// Its call instructions (more than one where the compiler copied the
    /// call into several arms, or where a loop's arms each make it).
    pub sites: &'static [u32],
    /// The callee's Xbox PDB name, from the engine map (none where it has
    /// none, or names it only by folding).
    pub name: Option<&'static str>,
    pub gate: SubGate,
    /// The branches of the step's own block that decide whether the call
    /// is made once the gate holds; not modelled. Empty: the call is made
    /// whenever the gate holds.
    pub own_tests: &'static [u32],
    pub wiring: Wiring,
}

/// One modelled function of the stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Function {
    pub address: u32,
    /// Its size in bytes (the engine map's).
    pub size: u32,
    pub name: Option<&'static str>,
    /// Who calls it in the frame: `Main::OnIdle` ([`ON_IDLE`]) or another
    /// function of [`FUNCTIONS`].
    pub caller: u32,
    /// The call instructions in the caller.
    pub call_sites: &'static [u32],
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

use Callee::{Direct, Import, Pointer, Virtual};
use SubGate as G;
use Wiring::Open;

/// `TES::TestAllCells` (`004556d0`), as `Main::OnIdle` calls it: with 0,
/// while `bRunningCellTests` is set, so one turn of its loop (another turn
/// follows at once when the next world space was entered but the player is
/// not in it yet, `00455dcb`). It is the debug run that loads every cell in
/// turn (the console's `TestAllCells`).
pub const TEST_ALL_CELLS: [SubStep; 25] = [
    // A mouse move, so the machine doesn't go idle.
    sub(
        Import(0x00fd_f2d4),
        &[0x0045_5a11],
        Some("SendInput"),
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(0x0045_8200),
        &[0x0045_5ac1],
        Some("TES::SetWorldSpace"),
        G::TestEnterWorldSpace,
        &[],
        Open,
    ),
    sub(
        Direct(0x005b_5e40),
        &[0x0045_5b77],
        None,
        G::TestEnterWorldSpace,
        &[],
        Open,
    ),
    sub(
        Direct(0x0058_5b30),
        &[0x0045_5bdc],
        Some("TESWorldSpace::LoadCell"),
        G::TestExteriorWalk,
        &[],
        Open,
    ),
    sub(
        Direct(0x0045_4e70),
        &[0x0045_5bf4],
        None,
        G::TestExteriorWalk,
        &[0x0045_5be8],
        Open,
    ),
    sub(
        Direct(0x005b_5e40),
        &[0x0045_5caa],
        None,
        G::TestInteriorWalk,
        &[0x0045_5ca3],
        Open,
    ),
    sub(
        Direct(0x0046_1980),
        &[0x0045_5cbf, 0x0045_5d0d],
        None,
        G::TestInteriorWalk,
        &[],
        Open,
    ),
    // The visit: the calendar on by 10, the player placed in the cell and the
    // loads run, the memory trimmed.
    sub(
        Direct(0x0086_7a40),
        &[0x0045_5dec],
        Some("Calendar::Update"),
        G::TestVisit,
        &[],
        Open,
    ),
    sub(
        Direct(0x0045_3dc0),
        &[0x0045_5ed3],
        None,
        G::TestVisitInterior,
        &[],
        Open,
    ),
    sub(
        Direct(0x0045_4450),
        &[0x0045_5ef6],
        None,
        G::TestVisitExteriorFromInterior,
        &[],
        Open,
    ),
    sub(
        Direct(0x0045_2580),
        &[0x0045_5f3f],
        Some("TES::UpdateCurrentGridCell"),
        G::TestVisitExterior,
        &[],
        Open,
    ),
    sub(
        Direct(0x0093_be30),
        &[0x0045_6043],
        Some("PlayerCharacter::RequestPositionPlayer"),
        G::TestVisit,
        &[],
        Open,
    ),
    sub(
        Direct(0x0093_bea0),
        &[0x0045_604e],
        Some("PlayerCharacter::HandlePositionPlayerRequest"),
        G::TestVisit,
        &[],
        Open,
    ),
    sub(
        Direct(0x0045_6520),
        &[0x0045_6059],
        Some("IOManager::LoadQueuedPriority"),
        G::TestVisit,
        &[],
        Open,
    ),
    sub(
        Direct(0x0087_8160),
        &[0x0045_608b],
        None,
        G::TestVisit,
        &[],
        Open,
    ),
    sub(
        Direct(0x0045_39a0),
        &[0x0045_609d],
        None,
        G::TestVisit,
        &[],
        Open,
    ),
    sub(
        Direct(0x0087_8250),
        &[0x0045_60a7],
        Some("MemoryLevelManager::FreeReleasedObjects"),
        G::TestVisit,
        &[],
        Open,
    ),
    sub(
        Direct(0x0087_8200),
        &[0x0045_60b3],
        None,
        G::TestVisit,
        &[],
        Open,
    ),
    // The cell's line, to the debug print and the output file.
    sub(
        Direct(0x005b_5e40),
        &[0x0045_6280],
        None,
        G::TestLogInterior,
        &[],
        Open,
    ),
    sub(
        Direct(0x00c3_c220),
        &[0x0045_6305],
        Some("MessageHandler::Output"),
        G::TestLogInterior,
        &[],
        Open,
    ),
    sub(
        Direct(0x005b_5e40),
        &[0x0045_63a0],
        None,
        G::TestLogExterior,
        &[],
        Open,
    ),
    sub(
        Direct(0x00c3_c220),
        &[0x0045_642d],
        Some("MessageHandler::Output"),
        G::TestLogExterior,
        &[],
        Open,
    ),
    // The callback, the save game's test, the end.
    sub(
        Pointer(0x54),
        &[0x0045_646a],
        None,
        G::TestCallback,
        &[],
        Open,
    ),
    sub(
        Direct(0x0086_1f20),
        &[0x0045_648e],
        Some("TESSaveLoadGame::TestAllCells"),
        G::TestSaveGame,
        &[],
        Open,
    ),
    sub(
        Direct(0x0043_b2b0),
        &[0x0045_64b7],
        Some("MessageHandler::IncDisableWarningCount"),
        G::TestDone,
        &[],
        Open,
    ),
];

/// `Calendar::Update(seconds)` (`00867a40`): the game hours the seconds
/// make at the time scale (`TimeScale`, 3600 seconds an hour) added to the
/// hour, the day, month and year carried over at midnight, through the
/// `TESGlobal` value setter (`0046dce0`; the getter `00526ac0` is a
/// query).
pub const CALENDAR_UPDATE: [SubStep; 7] = [
    sub(Direct(0x0046_dce0), &[0x0086_7ae6], None, G::CalendarRewrite, &[], Wiring::Partial("world::scripting GameState::advance_clock (GameDaysPassed by the hours / 24 at every step)")),
    sub(Direct(0x004b_10d0), &[0x0086_7b3b], None, G::CalendarMidnight, &[], Open),
    sub(Direct(0x0046_dce0), &[0x0086_7bd7], None, G::CalendarYearEnds, &[], Wiring::Partial("world::scripting GameState::advance_clock (its year carry; its calendar is labelled a guess)")),
    sub(Direct(0x0046_dce0), &[0x0086_7be9], None, G::CalendarMonthEnds, &[], Wiring::Partial("world::scripting GameState::advance_clock (its month carry; Gregorian month lengths, labelled a guess)")),
    sub(Direct(0x0046_dce0), &[0x0086_7bfb], None, G::CalendarMidnight, &[], Wiring::Partial("world::scripting GameState::advance_clock (its day carry)")),
    sub(Direct(0x0046_dce0), &[0x0086_7c35], None, G::Always, &[], Wiring::Partial("world::scripting GameState::advance_clock (GameDaysPassed)")),
    sub(Direct(0x0046_dce0), &[0x0086_7c50], None, G::Always, &[], Wiring::Partial("world::scripting GameState::advance_clock (GameHour; viewer: inside scripts::run_scripts' Runner::update)")),
];

/// `TES::RunAnimations` (`00455640`): the count `[011c56e8]` cleared, then
/// the loaded interior's animated references (`TESObjectCELL::RunAnimations`,
/// `00553820`) or the grid's (`GridCellArray::RunAnimations`, `004baba0`;
/// both paired in FRAME_SKELETON.md).
pub const RUN_ANIMATIONS: [SubStep; 3] = [
    sub(
        Direct(0x0045_5680),
        &[0x0045_5647],
        None,
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(0x0055_3820),
        &[0x0045_5662],
        Some("TESObjectCELL::RunAnimations"),
        G::Interior,
        &[],
        Open,
    ),
    sub(
        Direct(0x004b_aba0),
        &[0x0045_566f],
        Some("GridCellArray::RunAnimations"),
        G::Exterior,
        &[],
        Open,
    ),
];

/// `ProcessLists::RunActorScripts` (`00978550`): the script of every actor
/// on process level 0 (the high process array at +4, an entry whose slot
/// +0x100, `IsActor`, answers true).
pub const RUN_ACTOR_SCRIPTS: [SubStep; 1] = [sub(
    Direct(0x0056_5870),
    &[0x0097_85bf],
    Some("TESObjectREFR::RunScript"),
    G::Always,
    &[0x0097_858b, 0x0097_85a3, 0x0097_85ba],
    Wiring::Partial("viewer: scripts::run_scripts (ordered here; it runs every script it knows, references', quests' and items', and the game clock)"),
)];

/// `ProcessLists::UpdateRadiationList` (`009777a0`): for each radiation
/// source, every high actor exposed past `fRadsMinExposure`-like settings
/// (`011d0db0`, `011cd874`) avoids it (a pathing area, a move mode, an
/// avoid package) and keeps the new level (`00977c90`), pushed to its
/// process (slot +0x768); the player the same without avoiding.
pub const UPDATE_RADIATION_LIST: [SubStep; 9] = [
    sub(
        Direct(0x0090_42a0),
        &[0x0097_7a23],
        Some("HighProcess::AddAvoidPathingArea"),
        G::RadiationSources,
        &[
            0x0097_78f0,
            0x0097_7920,
            0x0097_7971,
            0x0097_798d,
            0x0097_79a5,
            0x0097_79b8,
            0x0097_79d7,
        ],
        Open,
    ),
    sub(
        Direct(0x008b_39f0),
        &[0x0097_7a41],
        Some("Actor::SetMoveMode"),
        G::RadiationSources,
        &[
            0x0097_78f0,
            0x0097_7920,
            0x0097_79b8,
            0x0097_79d7,
            0x0097_7a37,
        ],
        Open,
    ),
    sub(
        Direct(0x0089_82c0),
        &[0x0097_7a55],
        Some("Actor::InitiateAvoidPackage"),
        G::RadiationSources,
        &[
            0x0097_78f0,
            0x0097_7920,
            0x0097_79b8,
            0x0097_79d7,
            0x0097_7a37,
        ],
        Open,
    ),
    sub(
        Direct(0x0097_7c90),
        &[0x0097_7a64],
        None,
        G::RadiationSources,
        &[
            0x0097_78f0,
            0x0097_7920,
            0x0097_7971,
            0x0097_798d,
            0x0097_79a5,
        ],
        Open,
    ),
    sub(
        Virtual(0x768),
        &[0x0097_7a83],
        None,
        G::RadiationSources,
        &[0x0097_78f0, 0x0097_7920],
        Open,
    ),
    sub(
        Direct(0x0097_7c90),
        &[0x0097_7b2c],
        None,
        G::RadiationSources,
        &[0x0097_7afc, 0x0097_7b0c, 0x0097_7b20],
        Open,
    ),
    sub(
        Virtual(0x768),
        &[0x0097_7b60],
        None,
        G::RadiationSources,
        &[],
        Open,
    ),
    sub(
        Virtual(0x768),
        &[0x0097_7c32],
        None,
        G::RadiationEnded,
        &[0x0097_7bd9, 0x0097_7bf0, 0x0097_7c1c],
        Open,
    ),
    sub(
        Virtual(0x768),
        &[0x0097_7c61],
        None,
        G::RadiationEnded,
        &[0x0097_7c4b],
        Open,
    ),
];

/// The tests of `ChangeProcessLevelTempList`'s fourth case (`0096f0c4`
/// talking, `0096f0d0` the menu, `0096f0e2` the form flag 0x20000,
/// `0096f0f8`): the entry is taken out of the list and moved to the level
/// it asks for.
const MOVE: [u32; 4] = [0x0096_f0c4, 0x0096_f0d0, 0x0096_f0e2, 0x0096_f0f8];

/// `ProcessLists::ChangeProcessLevelTempList` (`0096eb40`): unless the
/// player sleeps or rests, each mobile object of the `TempShouldMoveList`
/// (+0x78) is either added as a reference (an actor without a process, or
/// someone the player isn't the one talking to), deleted (no file to reload
/// it from), released (its model being loaded), removed for good (dead past
/// its time, its cell unloaded) or moved to the process level it asks for
/// (`ProcessArray::AddActor`); whatever is left is flagged to be looked at
/// again (slot +0xd8 with 1).
pub const CHANGE_PROCESS_LEVEL_TEMP_LIST: [SubStep; 17] = [
    sub(
        Direct(0x0094_df60),
        &[0x0096_eb52],
        Some("PlayerCharacter::IsSleepingorResting"),
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(0x0096_d450),
        &[0x0096_ecaf],
        Some("ProcessLists::AddReference"),
        G::NotResting,
        &[
            0x0096_ec4e,
            0x0096_ec60,
            0x0096_ec71,
            0x0096_ec87,
            0x0096_ec99,
        ],
        Open,
    ),
    sub(
        Direct(0x0096_aaf0),
        &[0x0096_ed2b],
        Some("ProcessArray::RemoveActor"),
        G::NotResting,
        &[
            0x0096_ec4e,
            0x0096_ec60,
            0x0096_ec71,
            0x0096_ec87,
            0x0096_ec99,
            0x0096_ecc9,
            0x0096_ecda,
            0x0096_ecec,
            0x0096_ed02,
            0x0096_ed11,
        ],
        Open,
    ),
    sub(
        Virtual(0x10),
        &[0x0096_ed5e],
        None,
        G::NotResting,
        &[
            0x0096_ec4e,
            0x0096_ec60,
            0x0096_ec71,
            0x0096_ecc9,
            0x0096_ecda,
            0x0096_ecec,
            0x0096_ed02,
            0x0096_ed4f,
        ],
        Open,
    ),
    sub(
        Direct(0x0093_1e80),
        &[0x0096_ede0],
        None,
        G::NotResting,
        &[
            0x0096_ec4e,
            0x0096_ec60,
            0x0096_ec71,
            0x0096_ed88,
            0x0096_ed9a,
            0x0096_eda9,
            0x0096_edba,
        ],
        Open,
    ),
    sub(
        Virtual(0x324),
        &[0x0096_ef55],
        None,
        G::NotResting,
        &[
            0x0096_ee2c,
            0x0096_ee49,
            0x0096_ee5c,
            0x0096_ee6f,
            0x0096_ee90,
            0x0096_eebc,
            0x0096_ef17,
            0x0096_ef2c,
            0x0096_ef3b,
        ],
        Open,
    ),
    sub(
        Direct(0x0057_2270),
        &[0x0096_efbc],
        None,
        G::NotResting,
        &[
            0x0096_ef17,
            0x0096_ef3b,
            0x0096_ef6d,
            0x0096_ef73,
            0x0096_ef82,
            0x0096_efb7,
        ],
        Open,
    ),
    sub(
        Direct(0x0093_1e80),
        &[0x0096_f06c],
        None,
        G::NotResting,
        &[0x0096_f022, 0x0096_f037],
        Open,
    ),
    sub(
        Virtual(0xd8),
        &[0x0096_f114],
        None,
        G::NotResting,
        &MOVE,
        Open,
    ),
    sub(
        Virtual(0x224),
        &[0x0096_f15d],
        None,
        G::NotResting,
        &MOVE,
        Open,
    ),
    sub(
        Virtual(0x10),
        &[0x0096_f1f1],
        None,
        G::NotResting,
        &[
            0x0096_f0c4,
            0x0096_f0d0,
            0x0096_f0e2,
            0x0096_f0f8,
            0x0096_f17f,
            0x0096_f194,
            0x0096_f1e2,
        ],
        Open,
    ),
    sub(
        Direct(0x0093_1e80),
        &[0x0096_f276],
        None,
        G::NotResting,
        &[
            0x0096_f0c4,
            0x0096_f0d0,
            0x0096_f0e2,
            0x0096_f0f8,
            0x0096_f21e,
            0x0096_f230,
            0x0096_f23f,
            0x0096_f250,
        ],
        Open,
    ),
    sub(
        Virtual(0x364),
        &[0x0096_f337],
        None,
        G::NotResting,
        &[
            0x0096_f0c4,
            0x0096_f0d0,
            0x0096_f0e2,
            0x0096_f0f8,
            0x0096_f297,
            0x0096_f2c0,
            0x0096_f2f4,
            0x0096_f301,
            0x0096_f30e,
        ],
        Open,
    ),
    sub(
        Direct(0x0096_a970),
        &[0x0096_f359],
        Some("ProcessArray::AddActor"),
        G::NotResting,
        &[
            0x0096_f0c4,
            0x0096_f0d0,
            0x0096_f0e2,
            0x0096_f0f8,
            0x0096_f297,
            0x0096_f2c0,
            0x0096_f2f4,
            0x0096_f301,
            0x0096_f30e,
        ],
        Open,
    ),
    sub(
        Virtual(0x260),
        &[0x0096_f374],
        None,
        G::NotResting,
        &[
            0x0096_f0c4,
            0x0096_f0d0,
            0x0096_f0e2,
            0x0096_f0f8,
            0x0096_f297,
            0x0096_f2c0,
        ],
        Open,
    ),
    sub(
        Virtual(0xd8),
        &[0x0096_f3a6],
        None,
        G::NotResting,
        &[0x0096_f37d, 0x0096_f38f],
        Open,
    ),
    sub(
        Virtual(0xd8),
        &[0x0096_f3e9],
        None,
        G::NotResting,
        &[0x0096_f3bd, 0x0096_f3ca],
        Open,
    ),
];

/// `ProcessLists::UpdateFollowerTempList` (`0096e9b0`): each actor of the
/// `TempShouldMoveList` with a package (its process's slot +0x27c) follows
/// the package's target: an escort-type package (type 2) through the
/// process's slot +0xc8, otherwise `Actor::AddFollower` on a target that
/// is an actor other than the player.
pub const UPDATE_FOLLOWER_TEMP_LIST: [SubStep; 2] = [
    sub(
        Virtual(0xc8),
        &[0x0096_eaf2],
        None,
        G::Always,
        &[
            0x0096_ea1f,
            0x0096_ea32,
            0x0096_ea42,
            0x0096_ea65,
            0x0096_ea98,
            0x0096_eaa5,
            0x0096_eab6,
            0x0096_eacd,
        ],
        Open,
    ),
    sub(
        Direct(0x008b_c790),
        &[0x0096_eb25],
        Some("Actor::AddFollower"),
        G::Always,
        &[
            0x0096_ea1f,
            0x0096_ea32,
            0x0096_ea42,
            0x0096_ea65,
            0x0096_ea98,
            0x0096_eaa5,
            0x0096_eafa,
            0x0096_eb11,
            0x0096_eb1c,
        ],
        Open,
    ),
];

/// `GarbageCollector::Update` (`00868850`): under the collector's lock
/// (`00867f50`/`00867f80`), one batch of the first queue with entries (10,
/// 20 or 5 a frame, twice that when `00878360` says the frame is slow):
/// destroyed, or a flag-clear array moved into its flag-set twin.
pub const GARBAGE_COLLECTOR_UPDATE: [SubStep; 18] = [
    sub(Direct(0x0086_7f50), &[0x0086_887c], None, G::Always, &[], Open),
    sub(Direct(0x0041_8d20), &[0x0086_8947], None, G::GcAnimations, &[0x0086_88f5, 0x0086_8940], Open),
    sub(Direct(0x004a_af10), &[0x0086_8960], None, G::GcAnimations, &[], Open),
    sub(Direct(0x0041_8e00), &[0x0086_89fe], None, G::GcBipedsSet, &[0x0086_89ac, 0x0086_89f7], Open),
    sub(Direct(0x004a_af10), &[0x0086_8a17], None, G::GcBipedsSet, &[], Open),
    sub(Direct(0x0086_94c0), &[0x0086_8a39], None, G::GcBipedsClear, &[], Open),
    sub(Direct(0x005e_03d0), &[0x0086_8a46], None, G::GcBipedsClear, &[], Open),
    sub(Direct(0x0086_8ce0), &[0x0086_8abf], None, G::GcObjects3dSet, &[0x0086_8a78, 0x0086_8aba], Open),
    sub(Direct(0x0086_9420), &[0x0086_8af2], None, G::GcObjects3dClear, &[], Open),
    sub(Direct(0x004d_ffa0), &[0x0086_8aff], Some("NiTArray<NiPointer<BSTempEffect>_NiTNewInterface<NiPointer<BSTempEffect>_>_>::RemoveAll"), G::GcObjects3dClear, &[], Open),
    sub(Virtual(0x10), &[0x0086_8bdd], None, G::GcReferencesSet, &[0x0086_8b52, 0x0086_8bce], Open),
    sub(Direct(0x004a_af10), &[0x0086_8bf6], None, G::GcReferencesSet, &[], Open),
    sub(Direct(0x0086_94c0), &[0x0086_8c18], None, G::GcReferencesClear, &[], Open),
    sub(Direct(0x005e_03d0), &[0x0086_8c25], None, G::GcReferencesClear, &[], Open),
    sub(Direct(0x0040_1970), &[0x0086_8c82], None, G::GcNavMeshesSet, &[0x0086_8c57], Open),
    sub(Direct(0x0086_9510), &[0x0086_8cb1], None, G::GcNavMeshesClear, &[], Open),
    sub(Direct(0x0084_54f0), &[0x0086_8cc0], None, G::GcNavMeshesClear, &[], Open),
    sub(Direct(0x0086_7f80), &[0x0086_8cc7], None, G::Always, &[], Open),
];

/// `GarbageCollector::ClearTempEffects` (`00868d10`): the flag-set
/// temporary effects released under the collector's lock, then the model
/// loader's lock released.
pub const CLEAR_TEMP_EFFECTS: [SubStep; 4] = [
    sub(Direct(0x0086_7f50), &[0x0086_8d3a], None, G::TempEffects, &[], Open),
    sub(Direct(0x004d_ffa0), &[0x0086_8d47], Some("NiTArray<NiPointer<BSTempEffect>_NiTNewInterface<NiPointer<BSTempEffect>_>_>::RemoveAll"), G::TempEffects, &[], Open),
    sub(Direct(0x004a_af10), &[0x0086_8d52], None, G::TempEffects, &[], Open),
    sub(Direct(0x0086_7f80), &[0x0086_8d59], None, G::TempEffects, &[], Open),
];

/// `BSTreeManager::Update(camera, stopped)` (`006652e0`; not translated in
/// the engine crate, read from the disassembly): SpeedTree's clock
/// (`[011d5dd4]` plus the frame time, `00b07060`), the camera, the wind's
/// speed (the sky's +0xcc, 0 indoors) and direction from the sky, the wind
/// update (`006658b0`), then the canopy shadow settings
/// (`iCanopyShadowScale:SpeedTree`, `011d5d98`;
/// `fCanopyShadowGrassMult:SpeedTree`, `011d5d4c`, kept to 0-1).
pub const TREE_MANAGER_UPDATE: [SubStep; 7] = [
    sub(
        Direct(0x00b0_7060),
        &[0x0066_5316],
        None,
        G::TreesWorldRuns,
        &[],
        Open,
    ),
    sub(
        Direct(0x00b0_6c10),
        &[0x0066_539b],
        Some("CSpeedTreeRT::SetCamera"),
        G::TreesCamera,
        &[],
        Wiring::Partial("viewer: trees::sway_trees (the camera's axes for the leaves)"),
    ),
    sub(
        Direct(0x0045_36e0),
        &[0x0066_53fc],
        None,
        G::TreesOutdoorSky,
        &[],
        Open,
    ),
    sub(
        Direct(0x004b_c450),
        &[0x0066_54cc],
        None,
        G::TreesSky,
        &[0x0066_542a, 0x0066_5444],
        Open,
    ),
    sub(
        Direct(0x0066_58b0),
        &[0x0066_54dd],
        None,
        G::TreesWind,
        &[],
        Wiring::Partial("speedtree::wind (viewer: trees::blow_wind)"),
    ),
    sub(
        Direct(0x0066_4720),
        &[0x0066_54ef],
        None,
        G::Always,
        &[],
        Open,
    ),
    sub(
        Direct(0x0066_5520),
        &[0x0066_5507],
        None,
        G::Always,
        &[],
        Open,
    ),
];

/// `Main::OnIdle_UpdateCurrentGridCell` (`0086fbe0`): every call. The grid
/// follows the player's position (`00436aa0`); while the cell tests run
/// outdoors, the player's and the `TES`'s world spaces are refreshed.
pub const UPDATE_CURRENT_GRID_CELL_MAIN: [SubStep; 6] = [
    sub(
        Direct(0x0043_6aa0),
        &[0x0086_fc11],
        None,
        G::GridUpdate,
        &[],
        Open,
    ),
    sub(
        Direct(0x0045_2580),
        &[0x0086_fc1d],
        Some("TES::UpdateCurrentGridCell"),
        G::GridUpdate,
        &[],
        Wiring::Partial("world::ref_scripts (its grid-move test only)"),
    ),
    sub(
        Direct(0x0045_1530),
        &[0x0086_fc28],
        None,
        G::GridUpdate,
        &[],
        Open,
    ),
    sub(
        Direct(0x005f_36f0),
        &[0x0086_fc3a],
        None,
        G::GridCellTests,
        &[],
        Open,
    ),
    sub(
        Direct(0x0057_5d70),
        &[0x0086_fc49],
        Some("TESObjectREFR::GetWorldSpace"),
        G::GridCellTestsExterior,
        &[],
        Open,
    ),
    sub(
        Direct(0x004f_d3e0),
        &[0x0086_fc54],
        Some("TES::GetWorldSpace"),
        G::GridCellTestsExterior,
        &[],
        Open,
    ),
];

/// `TES::UpdateCurrentGridCell(position, settle)` (`00452580`): the
/// exterior grid follows the position. First (always) the last-loaded list
/// cleared (`00470470`), the moved flags and the loaded area's bound
/// refreshed; with no grid yet, the area is loaded at once; inside the
/// centre cell, the cells coming into range are queued for loading in the
/// background (`bBackgroundCellLoads`); across a cell border, the queued
/// loads are cancelled and the grid moves: the cells are detached and
/// attached around the new centre (`GridCellArray::SetCenter`, the grid
/// array's slot +0x10, `004ba630`, and `TES::GridArrayLoad`), unused
/// textures cleaned up, and the terrain manager updated at the player.
pub const UPDATE_CURRENT_GRID_CELL: [SubStep; 22] = [
    sub(Direct(0x0047_0470), &[0x0045_2593], None, G::Always, &[], Open),
    sub(Direct(0x0045_c840), &[0x0045_259b], None, G::Always, &[], Open),
    sub(Direct(0x0045_c780), &[0x0045_25cc], None, G::Always, &[0x0045_25a9, 0x0045_25b4, 0x0045_25c0], Open),
    sub(Direct(0x0062_8da0), &[0x0045_25f3], None, G::Always, &[0x0045_25e1], Open),
    sub(Direct(0x0062_71b0), &[0x0045_2618], None, G::Always, &[], Open),
    sub(Direct(0x0045_7d70), &[0x0045_26ad], Some("TES::ShowLoadingMenu"), G::GridFirst, &[], Open),
    sub(Direct(0x0045_15a0), &[0x0045_26bb], None, G::GridFirst, &[], Open),
    sub(Direct(0x0045_2490), &[0x0045_2845], Some("TES::CleanUpUnusedTextures"), G::GridInside, &[0x0045_27df, 0x0045_27ef, 0x0045_2810, 0x0045_281b, 0x0045_2830, 0x0045_283e], Open),
    sub(Direct(0x0052_83c0), &[0x0045_28ba, 0x0045_2920, 0x0045_294a], Some("ExteriorCellLoader::QueueCellLoad"), G::GridInside, &[0x0045_27df, 0x0045_27ef, 0x0045_2810, 0x0045_281b, 0x0045_2870, 0x0045_28d6, 0x0045_292b, 0x0045_2931], Partial("viewer: exterior::stream_squares (its background loads of the squares around the player)")),
    sub(Direct(0x0045_2e40), &[0x0045_296a], None, G::GridCrossed, &[], Open),
    sub(Direct(0x0045_81e0), &[0x0045_29ab], None, G::GridCrossed, &[0x0045_2980, 0x0045_298f, 0x0045_29a1], Open),
    sub(Direct(0x0045_c840), &[0x0045_2b19], None, G::GridCrossed, &[], Open),
    sub(Direct(0x0045_c780), &[0x0045_2b21], None, G::GridCrossed, &[], Open),
    sub(Direct(0x0052_8110), &[0x0045_2b2c], None, G::GridCrossed, &[], Open),
    sub(Direct(0x0052_82d0), &[0x0045_2b4b], None, G::GridCrossed, &[0x0045_2b43], Open),
    sub(Direct(0x0045_8200), &[0x0045_2bb5], Some("TES::SetWorldSpace"), G::GridReload, &[0x0045_2b9b], Open),
    sub(Virtual(0x10), &[0x0045_2c40], Some("GridCellArray::SetCenter"), G::GridReload, &[], Partial("viewer: exterior::stream_squares (finished squares put on screen, far ones dropped)")),
    sub(Direct(0x0045_7be0), &[0x0045_2d00], Some("TES::InitModelsToLoad"), G::GridReload, &[0x0045_2ce8, 0x0045_2cf9], Open),
    sub(Direct(0x0045_2ff0), &[0x0045_2d1d], Some("TES::GridArrayLoad"), G::GridReload, &[], Open),
    sub(Direct(0x0045_2490), &[0x0045_2d61], Some("TES::CleanUpUnusedTextures"), G::GridCrossed, &[], Open),
    sub(Direct(0x006f_ca90), &[0x0045_2d9f], Some("BGSTerrainManager::Update"), G::GridCrossedExterior, &[], Partial("viewer: exterior::stream_distant_land and lod_objects::stream_distant_objects (every frame)")),
    sub(Direct(0x0045_2e40), &[0x0045_2da6], None, G::GridCrossed, &[], Open),
];

use Wiring::Partial;

/// The modelled functions, in the order the stage first reaches them.
pub const FUNCTIONS: [Function; 12] = [
    Function {
        address: 0x0045_56d0,
        size: 3611,
        name: Some("TES::TestAllCells"),
        caller: ON_IDLE,
        call_sites: &[0x0086_e987],
        steps: &TEST_ALL_CELLS,
    },
    Function {
        address: 0x0086_7a40,
        size: 539,
        name: Some("Calendar::Update"),
        caller: ON_IDLE,
        call_sites: &[0x0086_e9ca],
        steps: &CALENDAR_UPDATE,
    },
    Function {
        address: 0x0045_5640,
        size: 56,
        name: Some("TES::RunAnimations"),
        caller: ON_IDLE,
        call_sites: &[0x0086_e9e4],
        steps: &RUN_ANIMATIONS,
    },
    Function {
        address: 0x0097_8550,
        size: 122,
        name: Some("ProcessLists::RunActorScripts"),
        caller: ON_IDLE,
        call_sites: &[0x0086_e9fa],
        steps: &RUN_ACTOR_SCRIPTS,
    },
    Function {
        address: 0x0097_77a0,
        size: 1223,
        name: Some("ProcessLists::UpdateRadiationList"),
        caller: ON_IDLE,
        call_sites: &[0x0086_ea3f, 0x0086_ea8b],
        steps: &UPDATE_RADIATION_LIST,
    },
    Function {
        address: 0x0096_eb40,
        size: 2236,
        name: Some("ProcessLists::ChangeProcessLevelTempList"),
        caller: ON_IDLE,
        call_sites: &[0x0086_ea49, 0x0086_eaa7],
        steps: &CHANGE_PROCESS_LEVEL_TEMP_LIST,
    },
    Function {
        address: 0x0096_e9b0,
        size: 398,
        name: Some("ProcessLists::UpdateFollowerTempList"),
        caller: ON_IDLE,
        call_sites: &[0x0086_ea53, 0x0086_eab1],
        steps: &UPDATE_FOLLOWER_TEMP_LIST,
    },
    Function {
        address: 0x0086_8850,
        size: 1166,
        name: Some("GarbageCollector::Update"),
        caller: ON_IDLE,
        call_sites: &[0x0086_eadf],
        steps: &GARBAGE_COLLECTOR_UPDATE,
    },
    Function {
        address: 0x0086_8d10,
        size: 83,
        name: Some("GarbageCollector::ClearTempEffects"),
        caller: ON_IDLE,
        call_sites: &[0x0086_eae4],
        steps: &CLEAR_TEMP_EFFECTS,
    },
    Function {
        address: 0x0066_52e0,
        size: 563,
        name: Some("BSTreeManager::Update"),
        caller: ON_IDLE,
        call_sites: &[0x0086_eb14],
        steps: &TREE_MANAGER_UPDATE,
    },
    Function {
        address: 0x0086_fbe0,
        size: 126,
        name: Some("Main::OnIdle_UpdateCurrentGridCell"),
        caller: ON_IDLE,
        call_sites: &[0x0086_eb1f],
        steps: &UPDATE_CURRENT_GRID_CELL_MAIN,
    },
    Function {
        address: 0x0045_2580,
        size: 2105,
        name: Some("TES::UpdateCurrentGridCell"),
        caller: 0x0086_fbe0,
        call_sites: &[0x0086_fc1d],
        steps: &UPDATE_CURRENT_GRID_CELL,
    },
];

/// The function at `address` in [`FUNCTIONS`].
pub fn function(address: u32) -> Option<&'static Function> {
    FUNCTIONS.iter().find(|f| f.address == address)
}

/// Whether sub-step `i` of `f` is reached, given that `f` is called.
pub fn step_runs(s: &WorldState, f: &Function, i: usize) -> bool {
    f.steps[i].gate.holds(s)
}

/// The sub-steps of `f` reached with these inputs, as indices into its
/// steps.
pub fn steps_run(s: &WorldState, f: &Function) -> Vec<usize> {
    (0..f.steps.len()).filter(|&i| step_runs(s, f, i)).collect()
}

/// The index of `f`'s sub-step whose call is at `site`.
pub fn step_at(f: &Function, site: u32) -> Option<usize> {
    f.steps.iter().position(|s| s.sites.contains(&site))
}

/// The world-stage inputs a frame's [`super::FrameState`] gives; the rest
/// at [`WorldState::default`].
pub fn from_frame(frame: &super::FrameState) -> WorldState {
    WorldState {
        world_runs: frame.world_runs(),
        menu_flag: frame.menu_flag(),
        world_frozen: frame.world_frozen,
        cell_tests: frame.running_cell_tests || frame.running_cell_tests_2,
        ..WorldState::default()
    }
}

/// Checks a translation's calls (`log`, in order, each as the [`Callee`] it
/// is) against `f`'s model with inputs `s`: every reached sub-step without
/// own tests is called, in the model's order, and no call goes to a callee
/// only unreached sub-steps have. `ignore`: callees left out of both checks
/// (those the translation calls as Rust functions, so not in its log, or
/// that its other callees call too).
pub fn follows(
    f: &Function,
    s: &WorldState,
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
mod tests {
    use super::*;
    use crate::frame::{step_of, Gate, Stage, STEPS};

    /// `research/engine-map/frame.tsv`, the committed call tree.
    const FRAME_TSV: &str = include_str!("../../../../research/engine-map/frame.tsv");

    /// The rows of `frame.tsv` as (depth, address).
    fn rows() -> Vec<(u32, u32)> {
        FRAME_TSV
            .lines()
            .filter(|l| !l.starts_with('#') && !l.starts_with("seq\t"))
            .map(|l| {
                let f: Vec<&str> = l.split('\t').collect();
                (
                    f[1].parse().unwrap(),
                    u32::from_str_radix(f[3], 16).unwrap(),
                )
            })
            .collect()
    }

    /// The direct calls `frame.tsv` lists under the row of `f` inside the
    /// frame (under its caller for a depth-2 function).
    fn tsv_callees(f: &Function) -> Vec<u32> {
        let rows = rows();
        let (depth, start) = if f.caller == ON_IDLE {
            let i = rows
                .iter()
                .position(|&(d, a)| d == 1 && a == f.address)
                .expect("a depth-1 row");
            (1, i)
        } else {
            let parent = rows
                .iter()
                .position(|&(d, a)| d == 1 && a == f.caller)
                .expect("the caller's row");
            let i = parent
                + rows[parent..]
                    .iter()
                    .position(|&(d, a)| d == 2 && a == f.address)
                    .expect("a depth-2 row");
            (2, i)
        };
        rows[start + 1..]
            .iter()
            .take_while(|&&(d, _)| d > depth)
            .filter(|&&(d, _)| d == depth + 1)
            .map(|&(_, a)| a)
            .collect()
    }

    /// Each function's direct calls appear in `frame.tsv`'s call list under
    /// it, in the same order (the model leaves queries out).
    #[test]
    fn direct_calls_are_in_frame_tsv_in_order() {
        for f in &FUNCTIONS {
            let listed = tsv_callees(f);
            assert!(!listed.is_empty(), "{:08x}", f.address);
            let mut at = 0;
            for st in f.steps {
                if let Callee::Direct(a) = st.callee {
                    for _ in st.sites {
                        let p = listed[at..]
                            .iter()
                            .position(|&x| x == a)
                            .unwrap_or_else(|| panic!("{:08x}: {a:08x} not in order", f.address));
                        at += p + 1;
                    }
                }
            }
        }
    }

    #[test]
    fn sites_and_tests_are_inside_their_functions_in_order() {
        for f in &FUNCTIONS {
            let sites: Vec<u32> = f
                .steps
                .iter()
                .flat_map(|s| s.sites.iter().copied())
                .collect();
            assert!(sites.windows(2).all(|w| w[0] < w[1]), "{:08x}", f.address);
            assert!(sites.iter().all(|&a| f.contains(a)), "{:08x}", f.address);
            for st in f.steps {
                let b = st.gate.branches();
                assert_eq!(
                    b.is_empty(),
                    st.gate == SubGate::Always,
                    "{:08x}",
                    st.sites[0]
                );
                assert!(
                    b.iter().chain(st.own_tests).all(|&a| f.contains(a)),
                    "{:08x}",
                    st.sites[0]
                );
                assert!(
                    b.iter().all(|t| t < st.sites.last().unwrap()),
                    "{:08x}",
                    st.sites[0]
                );
                assert!(
                    st.own_tests.iter().all(|t| t < st.sites.last().unwrap()),
                    "{:08x}",
                    st.sites[0]
                );
            }
            // Each function is called from its caller's steps or sub-steps.
            if f.caller == ON_IDLE {
                let i = step_of(f.address).expect("a step");
                assert_eq!(STEPS[i].stage, Stage::WorldAndTime);
            } else {
                let caller = function(f.caller).expect("a modelled caller");
                for &site in f.call_sites {
                    let i = step_at(caller, site).expect("a sub-step");
                    assert_eq!(caller.steps[i].callee, Callee::Direct(f.address));
                }
            }
        }
    }

    // `0086e9b5`: the scene graph's slot +0x104 comes after the frame
    // time's read (step 54) and before the second (step 55).
    #[test]
    fn the_scene_graph_update_sits_between_the_frame_time_reads() {
        let u = SCENE_GRAPH_UPDATE;
        assert_eq!(STEPS[u.after_step].address, 0x0084_d030);
        assert_eq!(STEPS[u.after_step + 1].address, 0x0084_d030);
        assert_eq!(STEPS[u.after_step - 1].address, 0x0055_9450);
        assert_eq!(STEPS[u.after_step].gate, Gate::WorldRuns);
    }

    fn reached(f: &Function, s: &WorldState) -> Vec<u32> {
        steps_run(s, f)
            .iter()
            .map(|&i| f.steps[i].sites[0])
            .collect()
    }

    // `004559a7`-`0045649f`: a cell test's turn.
    #[test]
    fn test_all_cells_turn() {
        let f = function(0x0045_56d0).unwrap();
        let idle = WorldState::default();
        // Not entering, no world space node: the interior walk, nothing found.
        assert_eq!(reached(f, &idle), [0x0045_5a11, 0x0045_5caa, 0x0045_5cbf]);
        let enter = WorldState {
            test_enter_world_space: true,
            ..idle
        };
        assert_eq!(reached(f, &enter), [0x0045_5a11, 0x0045_5ac1, 0x0045_5b77]);
        let interior = WorldState {
            test_cell_found: true,
            test_cell_interior: true,
            test_logging: true,
            test_callback: true,
            test_done: true,
            ..idle
        };
        let r = reached(f, &interior);
        assert!(r.contains(&0x0045_5ed3) && r.contains(&0x0045_6280) && r.contains(&0x0045_646a));
        assert!(!r.contains(&0x0045_5f3f) && !r.contains(&0x0045_63a0));
        assert_eq!(*r.last().unwrap(), 0x0045_64b7);
        let outdoors = WorldState {
            test_exterior_walk: true,
            test_cell_found: true,
            test_save_mode: true,
            ..idle
        };
        let r = reached(f, &outdoors);
        assert!(r.contains(&0x0045_5bdc) && r.contains(&0x0045_5f3f) && r.contains(&0x0045_648e));
        let indoors_now = WorldState {
            interior_loaded: true,
            ..outdoors
        };
        assert!(reached(f, &indoors_now).contains(&0x0045_5ef6));
    }

    // `00867a9b`-`00867bb1`: the calendar's carries.
    #[test]
    fn calendar_carries() {
        let f = function(0x0086_7a40).unwrap();
        assert_eq!(
            reached(f, &WorldState::default()),
            [0x0086_7c35, 0x0086_7c50]
        );
        let s = WorldState {
            calendar_rewrite: true,
            midnight: true,
            month_ends: true,
            year_ends: true,
            ..WorldState::default()
        };
        assert_eq!(steps_run(&s, f).len(), 7);
        let day = WorldState {
            midnight: true,
            month_ends: false,
            year_ends: true,
            ..WorldState::default()
        };
        assert_eq!(
            reached(f, &day),
            [0x0086_7b3b, 0x0086_7bfb, 0x0086_7c35, 0x0086_7c50]
        );
    }

    // `00455656`: the interior's animations or the grid's.
    #[test]
    fn run_animations() {
        let f = function(0x0045_5640).unwrap();
        assert_eq!(
            reached(f, &WorldState::default()),
            [0x0045_5647, 0x0045_566f]
        );
        let inside = WorldState {
            interior_loaded: true,
            ..WorldState::default()
        };
        assert_eq!(reached(f, &inside), [0x0045_5647, 0x0045_5662]);
    }

    // `009777cb`-`00977b99`: radiation.
    #[test]
    fn radiation() {
        let f = function(0x0097_77a0).unwrap();
        assert!(reached(f, &WorldState::default()).is_empty());
        let on = WorldState {
            radiation_sources: true,
            ..WorldState::default()
        };
        assert_eq!(reached(f, &on).len(), 7);
        let ended = WorldState {
            radiation_ended: true,
            ..WorldState::default()
        };
        assert_eq!(reached(f, &ended), [0x0097_7c32, 0x0097_7c61]);
    }

    // `0096eb5c`: nothing moves while the player sleeps or rests.
    #[test]
    fn process_levels_wait_for_rest() {
        let f = function(0x0096_eb40).unwrap();
        let resting = WorldState {
            resting: true,
            ..WorldState::default()
        };
        assert_eq!(reached(f, &resting), [0x0096_eb52]);
        assert_eq!(steps_run(&WorldState::default(), f).len(), f.steps.len());
    }

    // `008688bf`-`00868ca5`: one queue a frame.
    #[test]
    fn garbage_queues() {
        let f = function(0x0086_8850).unwrap();
        assert_eq!(
            reached(f, &WorldState::default()),
            [0x0086_887c, 0x0086_8cc7]
        );
        for (garbage, sites) in [
            (Garbage::Animations, &[0x0086_8947, 0x0086_8960][..]),
            (Garbage::BipedsSet, &[0x0086_89fe, 0x0086_8a17]),
            (Garbage::BipedsClear, &[0x0086_8a39, 0x0086_8a46]),
            (Garbage::Objects3dSet, &[0x0086_8abf]),
            (Garbage::Objects3dClear, &[0x0086_8af2, 0x0086_8aff]),
            (Garbage::ReferencesSet, &[0x0086_8bdd, 0x0086_8bf6]),
            (Garbage::ReferencesClear, &[0x0086_8c18, 0x0086_8c25]),
            (Garbage::NavMeshesSet, &[0x0086_8c82]),
            (Garbage::NavMeshesClear, &[0x0086_8cb1, 0x0086_8cc0]),
        ] {
            let s = WorldState {
                garbage,
                ..WorldState::default()
            };
            assert_eq!(
                reached(f, &s),
                [&[0x0086_887c][..], sites, &[0x0086_8cc7]].concat()
            );
        }
        // The destroying queues wait for the model loader's lock.
        let busy = WorldState {
            garbage: Garbage::Animations,
            model_loader_free: false,
            ..WorldState::default()
        };
        assert_eq!(reached(f, &busy), [0x0086_887c, 0x0086_8cc7]);
        let t = function(0x0086_8d10).unwrap();
        assert!(reached(t, &WorldState::default()).is_empty());
        let effects = WorldState {
            temp_effects: true,
            ..WorldState::default()
        };
        assert_eq!(reached(t, &effects).len(), 4);
    }

    // `006652f4`-`006653dc`: the trees in menu mode only get the camera and
    // the canopy settings.
    #[test]
    fn trees() {
        let f = function(0x0066_52e0).unwrap();
        assert_eq!(steps_run(&WorldState::default(), f).len(), 7);
        let menu = from_frame(&crate::frame::FrameState {
            in_menu_mode: true,
            ..crate::frame::FrameState::default()
        });
        assert_eq!(reached(f, &menu), [0x0066_539b, 0x0066_54ef, 0x0066_5507]);
        let inside = WorldState {
            interior_loaded: true,
            ..WorldState::default()
        };
        assert!(!reached(f, &inside).contains(&0x0066_53fc));
    }

    // `0086fbf1`-`0086fc41`, `00452624`-`00452d75`: the grid.
    #[test]
    fn grid() {
        let m = function(0x0086_fbe0).unwrap();
        assert_eq!(
            reached(m, &WorldState::default()),
            [0x0086_fc11, 0x0086_fc1d, 0x0086_fc28]
        );
        for blocked in [
            WorldState {
                menu_flag: true,
                ..WorldState::default()
            },
            WorldState {
                new_game_loading: true,
                ..WorldState::default()
            },
            WorldState {
                world_frozen: true,
                ..WorldState::default()
            },
        ] {
            assert!(reached(m, &blocked).is_empty());
        }
        let tests = WorldState {
            cell_tests: true,
            ..WorldState::default()
        };
        assert_eq!(reached(m, &tests).len(), 6);
        let g = function(0x0045_2580).unwrap();
        let always = [
            0x0045_2593,
            0x0045_259b,
            0x0045_25cc,
            0x0045_25f3,
            0x0045_2618,
        ];
        assert_eq!(
            reached(g, &WorldState::default()),
            [&always[..], &[0x0045_2845, 0x0045_28ba]].concat()
        );
        let first = WorldState {
            grid: GridMove::First,
            ..WorldState::default()
        };
        assert_eq!(
            reached(g, &first),
            [&always[..], &[0x0045_26ad, 0x0045_26bb]].concat()
        );
        let no_data = WorldState {
            data_handler: false,
            ..first
        };
        assert_eq!(reached(g, &no_data), always);
        let crossed = WorldState {
            grid: GridMove::Crossed,
            ..WorldState::default()
        };
        let r = reached(g, &crossed);
        assert!(r.contains(&0x0045_2c40) && r.contains(&0x0045_2d9f));
        assert!(!r.contains(&0x0045_28ba));
        let indoors = WorldState {
            interior_loaded: true,
            ..crossed
        };
        let r = reached(g, &indoors);
        assert!(!r.contains(&0x0045_2c40) && !r.contains(&0x0045_2d9f) && r.contains(&0x0045_2d61));
        let scripted = WorldState {
            script_running: true,
            ..indoors
        };
        assert!(reached(g, &scripted).contains(&0x0045_2c40));
    }

    #[test]
    fn follows_checks_order_and_unreached_calls() {
        let f = function(0x0045_5640).unwrap();
        let s = WorldState::default();
        let ok = [
            Callee::Direct(0x0045_5680),
            Callee::Direct(0x0005_0000),
            Callee::Direct(0x004b_aba0),
        ];
        assert!(follows(f, &s, &ok, &[]).is_ok());
        let wrong = [Callee::Direct(0x004b_aba0), Callee::Direct(0x0045_5680)];
        assert!(follows(f, &s, &wrong, &[]).is_err());
        let unreached = [
            Callee::Direct(0x0045_5680),
            Callee::Direct(0x0055_3820),
            Callee::Direct(0x004b_aba0),
        ];
        assert!(follows(f, &s, &unreached, &[]).is_err());
        assert!(follows(f, &s, &unreached, &[Callee::Direct(0x0055_3820)]).is_ok());
    }

    #[test]
    fn wiring() {
        let partial: Vec<u32> = FUNCTIONS
            .iter()
            .flat_map(|f| f.steps.iter())
            .filter(|s| matches!(s.wiring, Wiring::Partial(_)))
            .map(|s| s.sites[0])
            .collect();
        assert!(FUNCTIONS
            .iter()
            .flat_map(|f| f.steps.iter())
            .all(|s| s.wiring.is_open()));
        assert_eq!(
            partial,
            [
                0x0086_7ae6,
                0x0086_7bd7,
                0x0086_7be9,
                0x0086_7bfb,
                0x0086_7c35,
                0x0086_7c50,
                0x0097_85bf,
                0x0066_539b,
                0x0066_54dd,
                0x0086_fc1d,
                0x0045_28ba,
                0x0045_2c40,
                0x0045_2d9f
            ]
        );
    }
}
