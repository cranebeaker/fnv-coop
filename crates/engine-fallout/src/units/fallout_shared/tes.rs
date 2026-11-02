//! `fallout shared/tes.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TES` is the game's world manager: the loaded exterior/interior cells, the
//! scene graph roots, the sky and the singletons the world needs (the
//! `TESDataHandler`, the `ModelLoader`, the save/load object...). Session 1
//! of this unit (the first 40 functions in address order, `0044fb20` to
//! `00450bf0`) holds `TES::TES`, `TES::~TES` and the small static helpers
//! next to them: they copy INI settings into the globals of other
//! subsystems (Havok, LOD fade) and read or delete the singletons.
//! Session 2 (the next 40 functions, `00450c20` to `00452420`) holds the
//! water system's creation and deletion, the culling and lighting of the
//! grid of loaded exterior cells, the grid-position tests,
//! `TES::LoadGridCell`, and the function that loads one reference into the
//! scene (`00451ef0`) with its small helpers.
//! Session 3 (the next 40 functions, `00452440` to `00454af0`) holds the
//! per-frame world update and the cell loading and switching code:
//! `TES::UpdateCurrentGridCell` (which follows the player across the exterior
//! grid and moves it), `TES::GridArrayLoad`, `TES::AllCellsInGridLoaded`,
//! `TES::CleanUpUnusedTextures`, the world update `00453550` and
//! `TES::UpdateCellMainThread`, the choice of a cell to unload (`00453a80`),
//! and the two cell switches (`00453dc0` into a cell, `00454450` back out to
//! the exterior), with their one-line accessors. The next session continues
//! at `00454b10`.
//!
//! The PC build lays `TES` out like the Xbox build up to `0xC4` bytes; the
//! offsets used so far are declared in [`TES`] below with the Xbox PDB names.
//! The INI setting objects (`SettingT<INISettingCollection>`: vtable at +0,
//! value at +4, which `0043d4d0`/`00403e20`/`00408d60` return a pointer to)
//! are named after the string their initializer in the exe's tail passes to
//! the setting constructor.
//!
//! Not translated: the compiler's exception-unwinding frames of `TES::TES`
//! and `TES::~TES` (the `__CxxFrameHandler` states stored in the frame).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleList;
use crate::units::fallout_shared::extradataobjects::NiPoint3;

layout! {
    /// `TES` (Xbox PDB), 0xC4 bytes on both builds.
    pub struct TES: 0xC4 {
        /// `pGridCellA` (Xbox PDB): `GridCellArray*`, 0x28 bytes.
        0x08 pGridCellA: Ptr,
        /// `pObjRoot` (Xbox PDB): `NiNode*`.
        0x0C pObjRoot: Ptr,
        /// `pObjLandRoot` (Xbox PDB): `NiNode*`.
        0x10 pObjLandRoot: Ptr,
        /// `pObjLODWaterRoot` (Xbox PDB): `NiNode*`.
        0x14 pObjLODWaterRoot: Ptr,
        /// `pTempNodeManager` (Xbox PDB): `BSTempNodeManager*` (0xB0 bytes on PC).
        0x18 pTempNodeManager: Ptr,
        /// `pObjLight` (Xbox PDB): taken from the sky's `pSun` by `004505a0`.
        0x1C pObjLight: Ptr,
        /// `pObjFog` (Xbox PDB): taken from the sky's `pAtmosphere` by `00450570`.
        0x20 pObjFog: Ptr,
        /// `iCurrentGridX` (Xbox PDB): x of the centre grid cell, in cell units.
        0x24 iCurrentGridX: i32,
        /// `iCurrentGridY` (Xbox PDB).
        0x28 iCurrentGridY: i32,
        /// `iCurrentQueuedX` (Xbox PDB).
        0x2C iCurrentQueuedX: i32,
        /// `iCurrentQueuedY` (Xbox PDB).
        0x30 iCurrentQueuedY: i32,
        /// `pInteriorCell` (Xbox PDB): `TESObjectCELL*`. `005f36f0` (named
        /// `ActorMover::GetPreferredMoveMode` by the map; identical code)
        /// returns it.
        0x34 pInteriorCell: Ptr,
        /// `pInteriorBuffer` (Xbox PDB): `TESObjectCELL**`, one slot per
        /// `uInterior Cell Buffer`.
        0x38 pInteriorBuffer: Ptr,
        /// `pExteriorBuffer` (Xbox PDB): `TESObjectCELL**`, one slot per
        /// `uExterior Cell Buffer`.
        0x3C pExteriorBuffer: Ptr,
        /// `iTempInteriorBufferSize` (Xbox PDB).
        0x40 iTempInteriorBufferSize: u32,
        /// `iTempExteriorBufferSize` (Xbox PDB).
        0x44 iTempExteriorBufferSize: u32,
        /// `iSaveGridX` (Xbox PDB).
        0x48 iSaveGridX: i32,
        /// `iSaveGridY` (Xbox PDB).
        0x4C iSaveGridY: i32,
        /// `bRunningCellTests` (Xbox PDB).
        0x51 bRunningCellTests: bool,
        /// `bRunningCellTests2` (Xbox PDB).
        0x52 bRunningCellTests2: bool,
        /// `pfnTACCallbackFunc` (Xbox PDB): the callback `TES::TestAllCells`
        /// calls with each tested cell (set from mode 6 or 7).
        0x54 pfnTACCallbackFunc: Ptr,
        /// `pTACCallbackData` (Xbox PDB): the second word the callback gets.
        0x58 pTACCallbackData: Ptr,
        /// `pTACRegionFilter` (Xbox PDB).
        0x5C pTACRegionFilter: Ptr,
        /// `bShowLANDborders` (Xbox PDB).
        0x60 bShowLANDborders: bool,
        /// `pWaterSystem` (Xbox PDB): `TESWaterSystem*` (0xA0 bytes), built by
        /// `00450c50` and deleted by `00450d00`.
        0x64 pWaterSystem: Ptr,
        /// `pSky` (Xbox PDB): `Sky*`.
        0x68 pSky: Ptr,
        /// `listActiveImageSpaceModifiers` (Xbox PDB).
        0x6C listActiveImageSpaceModifiers: Inline<BSSimpleList>,
        /// `bUpdateGridString` (Xbox PDB).
        0x7D bUpdateGridString: bool,
        /// `fCell_delta_x` (Xbox PDB): the distance, in world units, from the
        /// current position to the border of the cell in x (see
        /// `TES::UpdateCurrentGridCell`).
        0x80 fCell_delta_x: f32,
        /// `fCell_delta_y` (Xbox PDB).
        0x84 fCell_delta_y: f32,
        /// `pWorldSpace` (Xbox PDB).
        0x88 pWorldSpace: Ptr,
        /// `listLastLoadedExteriors` (Xbox PDB).
        0x8C listLastLoadedExteriors: Inline<BSSimpleList>,
        /// `ListofBedsAndChairs` (Xbox PDB).
        0x94 ListofBedsAndChairs: Inline<BSSimpleList>,
        /// `DeadCount` (Xbox PDB).
        0x9C DeadCount: Inline<BSSimpleList>,
        /// `spPreloadedAddonNodes` (Xbox PDB): `NiPointer<QueuedFile>`.
        0xA4 spPreloadedAddonNodes: Ptr,
        /// `BloodDecalPreload1` (Xbox PDB): `NiPointer<NiSourceTexture>`.
        0xA8 BloodDecalPreload1: Ptr,
        /// `spPreloadedForms` (Xbox PDB): `NiPointer<QueuedFile>`.
        0xAC spPreloadedForms: Ptr,
        /// `pParticleCacheHead` (Xbox PDB).
        0xB0 pParticleCacheHead: Ptr,
        /// `bFadeWhenLoading` (Xbox PDB).
        0xB4 bFadeWhenLoading: bool,
        /// `bAllowUnusedPurge` (Xbox PDB).
        0xB5 bAllowUnusedPurge: bool,
        /// `pNavMeshInfoMap` (Xbox PDB).
        0xBC pNavMeshInfoMap: Ptr,
        /// `spLoadedAreaBound` (Xbox PDB): `NiPointer<LoadedAreaBound>`.
        0xC0 spLoadedAreaBound: Ptr,
    }
}

/// `TES`'s vtable (`GetMapNameForLocation` first).
pub(crate) const TES_VTABLE: u32 = 0x0101_78b4;
/// `BSTempNodeManager`'s vtable.
pub(crate) const BS_TEMP_NODE_MANAGER_VTABLE: u32 = 0x0101_78bc;
/// `BSTempNodeManager`'s `NiRTTI` (what `GetRTTI` returns).
pub(crate) const BS_TEMP_NODE_MANAGER_RTTI: u32 = 0x0120_2df8;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout Shared\TES.cpp"`.
pub(crate) const TES_CPP_PATH: u32 = 0x0101_7874;

// The singletons the world manager creates and destroys.

/// The `TESDataHandler` singleton pointer (the object is 0x63C bytes).
pub(crate) const DATA_HANDLER: u32 = 0x011c_3f2c;
/// The `ModelLoader` singleton pointer (0x30 bytes).
pub(crate) const MODEL_LOADER: u32 = 0x011c_3b3c;
/// The `IOManager` singleton pointer (0xA0 bytes on PC; inferred from its
/// readers in `main` and the queued-file code).
pub(crate) const IO_MANAGER: u32 = 0x0120_2d98;
/// The `BSParallelTaskManager` singleton pointer (0x10 bytes), created only
/// when `iNumHWThreads:General` is above 1.
pub(crate) const PARALLEL_TASK_MANAGER: u32 = 0x0120_2df4;
/// The `TESSaveLoadGame` singleton pointer (0x1C8 bytes).
pub(crate) const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// An object whose vtable slot `0x1CC` `TES::~TES` calls (with two zeros).
pub(crate) const OBJECT_WITH_SLOT_1CC: u32 = 0x011d_ea3c;
/// An object `TES::~TES` passes to `00867840` as `this` (inline, not a pointer).
pub(crate) const OBJECT_011DE7B8: u32 = 0x011d_e7b8;
/// An object `TES::~TES` passes to `00977540` as `this` (inline).
pub(crate) const OBJECT_011E0E80: u32 = 0x011e_0e80;
/// The `VATS` object `TES::~TES` passes to `VATS::ClearData` (`009c6ba0`).
pub(crate) const VATS_OBJECT: u32 = 0x011f_2250;
/// Table of four `BSShadowSceneNode` pointers (`00450b80` reads entry `n`).
pub(crate) const SHADOW_SCENE_NODE_TABLE: u32 = 0x011f_91c8;
/// Number of entries in [`SHADOW_SCENE_NODE_TABLE`].
pub(crate) const SHADOW_SCENE_NODE_COUNT: u32 = 4;
/// High-water mark of `uGridsToLoad` squared, raised by `TES::TES`.
pub(crate) const GRID_CELL_COUNT_HIGH_WATER_MARK: u32 = 0x011c_3c00;

// INI settings (`SettingT<INISettingCollection>` objects).

/// `iUpdateType:HAVOK`.
pub(crate) const SETTING_HAVOK_UPDATE_TYPE: u32 = 0x011c_3c30;
/// `bAddBipedWhenKeyframed:HAVOK`.
pub(crate) const SETTING_ADD_BIPED_WHEN_KEYFRAMED: u32 = 0x011c_3da0;
/// `fDebrisMaxVelocity`.
pub(crate) const SETTING_DEBRIS_MAX_VELOCITY: u32 = 0x011c_3c74;
/// `fDebrisMinExtent`.
pub(crate) const SETTING_DEBRIS_MIN_EXTENT: u32 = 0x011c_3db0;
/// `fChaseDeltaMult:HAVOK`.
pub(crate) const SETTING_CHASE_DELTA_MULT: u32 = 0x011c_3d28;
/// `fMaxPickTime:HAVOK`.
pub(crate) const SETTING_MAX_PICK_TIME: u32 = 0x011c_3e54;
/// `fMaxPickTimeVATS:HAVOK`.
pub(crate) const SETTING_MAX_PICK_TIME_VATS: u32 = 0x011c_3e94;
/// `iEntityBatchRemoveRate:HAVOK`.
pub(crate) const SETTING_ENTITY_BATCH_REMOVE_RATE: u32 = 0x011c_3c3c;
/// `fMoveLimitMass:HAVOK`.
pub(crate) const SETTING_MOVE_LIMIT_MASS: u32 = 0x011c_3d4c;
/// `fCharacterControllerMultipleStepSpeed`.
pub(crate) const SETTING_CHARACTER_CONTROLLER_MULTIPLE_STEP_SPEED: u32 = 0x011c_3ce8;
/// `fPhysicsDamage1Mass`.
pub(crate) const SETTING_PHYSICS_DAMAGE_1_MASS: u32 = 0x011c_fb38;
/// `fFadeOutThreshold:LOD`.
pub(crate) const SETTING_FADE_OUT_THRESHOLD: u32 = 0x011c_3e18;
/// `fFadeInThreshold:LOD`.
pub(crate) const SETTING_FADE_IN_THRESHOLD: u32 = 0x011c_3d00;
/// `fFadeInTime:LOD`.
pub(crate) const SETTING_FADE_IN_TIME: u32 = 0x011c_3df4;
/// `fFadeOutTime:LOD`.
pub(crate) const SETTING_FADE_OUT_TIME: u32 = 0x011c_3c5c;
/// `fDistanceMultiplier:LOD`.
pub(crate) const SETTING_DISTANCE_MULTIPLIER: u32 = 0x011c_3d60;
/// `bQueueWarnings:General`.
pub(crate) const SETTING_QUEUE_WARNINGS: u32 = 0x011c_3e64;
/// `bCheckPurgedTextureList:General`.
pub(crate) const SETTING_CHECK_PURGED_TEXTURE_LIST: u32 = 0x011c_3ccc;
/// `iNumHWThreads:General`.
pub(crate) const SETTING_NUM_HW_THREADS: u32 = 0x011c_3ea4;
/// `uExterior Cell Buffer:General`.
pub(crate) const SETTING_EXTERIOR_CELL_BUFFER: u32 = 0x011c_3c90;
/// `uInterior Cell Buffer:General`.
pub(crate) const SETTING_INTERIOR_CELL_BUFFER: u32 = 0x011c_3e38;
/// `uGridsToLoad:General`.
pub(crate) const SETTING_GRIDS_TO_LOAD: u32 = 0x011c_63cc;
/// `fLODFadeOutMultItems:LOD`.
pub(crate) const SETTING_LOD_FADE_OUT_MULT_ITEMS: u32 = 0x011c_3d0c;
/// `fLODFadeOutMultActors:LOD`.
pub(crate) const SETTING_LOD_FADE_OUT_MULT_ACTORS: u32 = 0x011c_3ec0;
/// `fLODFadeOutMultObjects:LOD`.
pub(crate) const SETTING_LOD_FADE_OUT_MULT_OBJECTS: u32 = 0x011c_3d78;

// Callees outside this unit.

/// Setting value pointer, `int` flavour (`this + 4`).
pub(crate) const SETTING_VALUE_ADDRESS_INT: u32 = 0x0043_d4d0;
/// Setting value pointer, `float` flavour (identical code).
pub(crate) const SETTING_VALUE_ADDRESS_FLOAT: u32 = 0x0040_3e20;
/// Setting value pointer, `bool`/byte flavour (identical code).
pub(crate) const SETTING_VALUE_ADDRESS_BYTE: u32 = 0x0040_8d60;
/// Sets a `uint` setting's value (`this` = the setting).
pub(crate) const SETTING_SET_UINT: u32 = 0x0045_ce80;
/// `_memset` through the `std::_Fiopen` name the linker folded onto it.
pub(crate) const MEMSET: u32 = 0x0040_3d30;
/// `operator new` (`MemoryManager::Allocate`).
pub(crate) const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete`.
pub(crate) const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `NiMemObject::operator new` (the block comes back in EAX).
pub(crate) const NI_OPERATOR_NEW: u32 = 0x00aa_13e0;
/// Sized `NiMemObject::operator delete(block, size)`.
pub(crate) const NI_OPERATOR_DELETE: u32 = 0x00aa_1460;
/// The 4-byte scope guard the constructor keeps on its stack (sets the
/// memory-allocation tag to its first argument for the guard's lifetime).
pub(crate) const SCOPE_GUARD_CTOR: u32 = 0x0040_4eb0;
pub(crate) const SCOPE_GUARD_DTOR: u32 = 0x0040_4ee0;
/// Size of that guard.
pub(crate) const SCOPE_GUARD_SIZE: u32 = 4;

// Globals and settings used by the grid code (session 2).

/// `bUseWater:Water`.
pub(crate) const SETTING_USE_WATER: u32 = 0x011c_7adc;
/// `fFadeToBlackFadeSeconds`.
pub(crate) const SETTING_FADE_TO_BLACK_FADE_SECONDS: u32 = 0x011c_3e7c;
/// The `TES` singleton pointer.
pub(crate) const TES_SINGLETON: u32 = 0x011d_ea10;
/// The `PlayerCharacter` singleton pointer.
pub(crate) const PLAYER_CHARACTER: u32 = 0x011d_ea3c;
/// The `FaderManager` singleton pointer.
pub(crate) const FADER_MANAGER: u32 = 0x011d_8804;
/// The object `00451ef0` passes as `this` to `0042ce10` (a flag getter).
pub(crate) const SCRIPT_CONTEXT: u32 = 0x011d_df38;
/// `TESObjectCELL::spExteriorWorld` (Xbox PDB): a `NiPointer<bhkWorldM>`;
/// `00559450` (the `NiPointer` getter, returns the first word) reads it.
pub(crate) const EXTERIOR_WORLD: u32 = 0x011c_a0d8;
/// Byte `004512c0` sets while it runs and clears at its end; it returns at
/// once when the byte is already set (a re-entrancy guard).
pub(crate) const GRID_LOAD_IN_PROGRESS: u32 = 0x011c_3ed0;
/// Byte written by `00451520` (also written by `ClearCanopyShadowMaskTexture`
/// and read by `008705c0`).
pub(crate) const GRID_FLAG_01189184: u32 = 0x0118_9184;
/// Byte written by `00451590` (read by `00b61980`).
pub(crate) const GRID_FLAG_011AD86C: u32 = 0x011a_d86c;
/// Three words (a `NiPoint3`) `004512c0` passes to `0057d0a0` after the
/// camera position.
pub(crate) const POINT_011A9478: u32 = 0x011a_9478;
/// A `double` (200.0 in the exe) that `00451ef0` compares two floats with
/// (address of the constant; read at run time).
pub(crate) const LARGE_CELL_LIMIT: u32 = 0x0101_79e0;
/// `"Loading cell...%s (%i, %i) (%08X)"`.
pub(crate) const LOADING_CELL_FORMAT: u32 = 0x0101_79bc;
/// `__RTDynamicCast` source and target type descriptors (RTTI data in the
/// exe) that `00451ef0` casts the reference's base form with.
pub(crate) const RTTI_TESFORM: u32 = 0x0118_3108;
pub(crate) const RTTI_CAST_TARGET: u32 = 0x0118_6060;

// Callees outside this unit that several grid functions use.

/// `TESObjectCELL::GetDataX`/`GetDataY` (Xbox PDB).
pub(crate) const CELL_GET_DATA_X: u32 = 0x0054_4c30;
pub(crate) const CELL_GET_DATA_Y: u32 = 0x0054_4c60;
/// `TES::pInteriorCell` getter (the map calls it
/// `ActorMover::GetPreferredMoveMode`; the body returns `this + 0x34`).
pub(crate) const GET_INTERIOR_CELL: u32 = 0x005f_36f0;
/// `TES::GetWorldSpace` (Xbox PDB): `this + 0x88`.
pub(crate) const GET_WORLD_SPACE: u32 = 0x004f_d3e0;
/// `GridCellArray::Get(x, y)` (Xbox PDB): the address of the slot holding
/// the cell pointer.
pub(crate) const GRID_CELL_ARRAY_GET: u32 = 0x004b_a490;
/// `GridCellArray::AttachToWorld(root, x, y)` (Xbox PDB).
pub(crate) const GRID_CELL_ARRAY_ATTACH_TO_WORLD: u32 = 0x004b_a960;
/// `TESWorldSpace::GetTerrainManager` (Xbox PDB).
pub(crate) const GET_TERRAIN_MANAGER: u32 = 0x0058_6170;
/// A reference's base form (`this + 0x20`; the map names it
/// `BGSSaveFormBuffer::GetForm`).
pub(crate) const REFERENCE_GET_BASE_FORM: u32 = 0x007a_f430;
/// The form type byte (`this + 4`).
pub(crate) const FORM_GET_TYPE: u32 = 0x0040_1170;
/// `_ftol2_sse`: truncates the `ST0` float to an integer (here an `f64`
/// argument, see the engine guide).
pub(crate) const FTOL: u32 = 0x00ec_62c0;
/// `fistp` of a `float` pushed on the stack (round to nearest), returns it.
pub(crate) const FLOAT_TO_INT_ROUNDED: u32 = 0x0040_6d90;
/// Constructor of the 12-byte stack object (a `float` at +0, bytes at +4 and
/// +5, zeros at +6..+8) the grid code passes to [`NODE_UPDATE`]; arguments
/// `(float, byte, byte)`.
pub(crate) const UPDATE_OBJECT_CTOR: u32 = 0x0043_d410;
/// Takes a node and that 12-byte object and calls the node's vtable slot
/// `0xA4` with the object and 0 (then slot `0xFC` of the object at +0x18, when
/// set).
pub(crate) const NODE_UPDATE: u32 = 0x00a5_9c60;

/// Reads an INI setting's value as an unsigned integer.
pub(crate) fn setting_uint(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(SETTING_VALUE_ADDRESS_INT, &args![setting]).u32();
    e.mem.u32(value)
}

/// Reads an INI setting's value as a float.
pub(crate) fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(SETTING_VALUE_ADDRESS_FLOAT, &args![setting]).u32();
    e.mem.f32(value)
}

/// Reads an INI setting's value as a byte.
pub(crate) fn setting_byte(e: &mut Engine, setting: u32) -> u8 {
    let value = e.call(SETTING_VALUE_ADDRESS_BYTE, &args![setting]).u32();
    e.mem.u8(value)
}

/// `uGridsToLoad`, as the constructor reads it: one call per use.
pub(crate) fn grids_to_load(e: &mut Engine) -> u32 {
    setting_uint(e, SETTING_GRIDS_TO_LOAD)
}

/// `uGridsToLoad * uGridsToLoad + uGridsToLoad + uGridsToLoad + 1`, reading
/// the setting four times in the order the code does.
pub(crate) fn exterior_cell_buffer_needed(e: &mut Engine) -> u32 {
    let first = e
        .call(SETTING_VALUE_ADDRESS_INT, &args![SETTING_GRIDS_TO_LOAD])
        .u32();
    let second = e
        .call(SETTING_VALUE_ADDRESS_INT, &args![SETTING_GRIDS_TO_LOAD])
        .u32();
    let squared = e.mem.u32(first).wrapping_mul(e.mem.u32(second));
    let third = grids_to_load(e);
    let fourth = grids_to_load(e);
    squared
        .wrapping_add(third)
        .wrapping_add(fourth)
        .wrapping_add(1)
}

/// `count * 4` as the compiler computes an array allocation size: all ones
/// when the multiplication overflows 32 bits.
pub(crate) fn array_bytes(count: u32) -> u32 {
    count.saturating_mul(4)
}

// Translated from 0044fb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::TES` (Xbox PDB): builds the world manager. The roots, the sky and
/// the data-handler argument come from the caller (`main`).
///
/// Order of events: the member lists and `NiPointer`s are constructed, the
/// whole object is zeroed with `memset` (which also clears the vtable pointer
/// stored first, and the lists just built; the game does exactly this), the
/// Havok and LOD settings are copied into their globals, the singletons are
/// created (`GridCellArray`, `IOManager`, `ModelLoader`, the parallel task
/// manager when `iNumHWThreads` is above 1, `TESDataHandler`,
/// `TESSaveLoadGame`), the cell buffers are sized from `uExterior Cell
/// Buffer`, `uInterior Cell Buffer` and `uGridsToLoad` (raising the settings
/// when `uGridsToLoad` needs more), the temp node manager is attached to
/// `pObjRoot`, and the sky's light and fog objects are fetched.
pub fn tes_tes(
    e: &mut Engine,
    this: Ptr<TES>,
    data_handler_arg: u32,
    obj_root: Ptr,
    obj_land_root: Ptr,
    sky: Ptr,
    obj_lod_water_root: Ptr,
) -> Ptr<TES> {
    let base = this.addr();
    e.mem.set_u32(base, TES_VTABLE);
    e.call(0x004e_e810, &args![base + 0x6c]);
    e.call(0x0096_a2d0, &args![base + 0x8c]);
    e.call(0x0096_a2d0, &args![base + 0x94]);
    e.call(0x0096_a2d0, &args![base + 0x9c]);
    e.call(0x0052_8cb0, &args![base + 0xa4, 0u32]);
    e.call(0x0063_3c90, &args![base + 0xa8, 0u32]);
    e.call(0x0052_8cb0, &args![base + 0xac, 0u32]);
    e.call(0x0063_3c90, &args![base + 0xc0, 0u32]);
    let guard = e.mem.alloc(SCOPE_GUARD_SIZE);
    e.call(
        SCOPE_GUARD_CTOR,
        &args![guard, 0xau32, 1u32, TES_CPP_PATH, 0x16au32],
    );
    e.call(MEMSET, &args![base, 0u32, 0xc4u32]);

    fn_00450430(e, 1);
    e.call(0x0062_4bf0, &args![]);
    let update_type = fn_004503f0(e, Ptr::new(SETTING_HAVOK_UPDATE_TYPE));
    fn_00450450(e, update_type);
    fn_00450470(e, 0, 0x0057_64f0);
    let add_biped = setting_byte(e, SETTING_ADD_BIPED_WHEN_KEYFRAMED);
    fn_00450490(e, add_biped);
    let value = setting_float(e, SETTING_DEBRIS_MAX_VELOCITY);
    fn_004504a0(e, value);
    let value = setting_float(e, SETTING_DEBRIS_MIN_EXTENT);
    fn_004504b0(e, value);
    let value = setting_float(e, SETTING_CHASE_DELTA_MULT);
    fn_00450460(e, value);
    let value = setting_float(e, SETTING_MAX_PICK_TIME);
    fn_004504c0(e, value);
    let value = setting_float(e, SETTING_MAX_PICK_TIME_VATS);
    fn_004504d0(e, value);
    let value = setting_uint(e, SETTING_ENTITY_BATCH_REMOVE_RATE);
    fn_004504e0(e, value);
    let value = fn_00450410(e, Ptr::new(SETTING_MOVE_LIMIT_MASS));
    fn_00450670(e, value);
    // Both reads are separate calls; the product is stored as a `float`.
    let first = e
        .call(
            SETTING_VALUE_ADDRESS_FLOAT,
            &args![SETTING_CHARACTER_CONTROLLER_MULTIPLE_STEP_SPEED],
        )
        .u32();
    let second = e
        .call(
            SETTING_VALUE_ADDRESS_FLOAT,
            &args![SETTING_CHARACTER_CONTROLLER_MULTIPLE_STEP_SPEED],
        )
        .u32();
    let product = e.mem.f32(first) * e.mem.f32(second);
    fn_00450680(e, product);
    let value = setting_float(e, SETTING_PHYSICS_DAMAGE_1_MASS);
    fn_00450690(e, value);
    fn_004506b0(e);
    fn_004504f0(e, 6, 0.0);
    let minus_one: f32 = e.global(0x0101_2054);
    fn_004504f0(e, 7, minus_one);
    let value = setting_float(e, SETTING_FADE_OUT_THRESHOLD);
    fn_00450540(e, value);
    let value = setting_float(e, SETTING_FADE_IN_THRESHOLD);
    fn_00450530(e, value);
    let value = setting_float(e, SETTING_FADE_IN_TIME);
    fn_00450510(e, value);
    let value = setting_float(e, SETTING_FADE_OUT_TIME);
    fn_00450520(e, value);
    let value = setting_float(e, SETTING_DISTANCE_MULTIPLIER);
    fn_00450550(e, value);
    let value = setting_byte(e, SETTING_QUEUE_WARNINGS);
    fn_00450440(e, value);
    let value = setting_byte(e, SETTING_CHECK_PURGED_TEXTURE_LIST);
    fn_00450560(e, value);
    fn_004505d0(e, 0x0057_7450);
    fn_004505e0(e, 0x0057_7ca0);

    // `GridCellArray`: constructed, then its second virtual (reference
    // count increment) is called.
    let grid_cells = e.call(OPERATOR_NEW, &args![0x28u32]).u32();
    let grid_cells = if grid_cells == 0 {
        0
    } else {
        e.call(0x004b_a280, &args![grid_cells]).u32()
    };
    e.set(this, TES::pGridCellA, Ptr::new(grid_cells));
    e.vcall(grid_cells, 4, &args![]);

    let guard_for_singletons = e.mem.alloc(SCOPE_GUARD_SIZE);
    e.call(
        SCOPE_GUARD_CTOR,
        &args![guard_for_singletons, 0x12u32, 1u32, TES_CPP_PATH, 0x1aeu32],
    );
    e.call(0x0052_7eb0, &args![]);
    let io_manager = e.call(OPERATOR_NEW, &args![0xa0u32]).u32();
    let io_manager = if io_manager == 0 {
        0
    } else {
        e.call(0x00c3_da50, &args![io_manager]).u32()
    };
    e.set_global(IO_MANAGER, io_manager);
    let model_loader = e.call(OPERATOR_NEW, &args![0x30u32]).u32();
    let model_loader = if model_loader == 0 {
        0
    } else {
        e.call(0x0044_2650, &args![model_loader]).u32()
    };
    e.set_global(MODEL_LOADER, model_loader);
    e.call(SCOPE_GUARD_DTOR, &args![guard_for_singletons]);
    e.mem.free(guard_for_singletons);

    let hardware_threads = setting_uint(e, SETTING_NUM_HW_THREADS) as i32;
    if hardware_threads > 1 {
        let manager = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
        let manager = if manager == 0 {
            0
        } else {
            e.call(0x00c4_4980, &args![manager]).u32()
        };
        e.set_global(PARALLEL_TASK_MANAGER, manager);
    }
    let data_handler = e.call(OPERATOR_NEW, &args![0x63cu32]).u32();
    let data_handler = if data_handler == 0 {
        0
    } else {
        e.call(0x0045_d270, &args![data_handler]).u32()
    };
    e.set_global(DATA_HANDLER, data_handler);
    let data_handler: u32 = e.global(DATA_HANDLER);
    e.call(0x0046_24b0, &args![data_handler, data_handler_arg]);
    let save_load = e.call(OPERATOR_NEW, &args![0x1c8u32]).u32();
    let save_load = if save_load == 0 {
        0
    } else {
        e.call(0x0085_6380, &args![save_load]).u32()
    };
    e.set_global(SAVE_LOAD_GAME, save_load);
    e.call(0x0084_7480, &args![]);
    e.call(0x0084_fc70, &args![]);

    // Make sure the exterior cell buffer can hold the loaded grid.
    let exterior = fn_004503f0(e, Ptr::new(SETTING_EXTERIOR_CELL_BUFFER));
    let needed = exterior_cell_buffer_needed(e);
    if exterior < needed {
        let needed = exterior_cell_buffer_needed(e);
        e.call(
            SETTING_SET_UINT,
            &args![SETTING_EXTERIOR_CELL_BUFFER, needed],
        );
    }
    let interior = fn_004503f0(e, Ptr::new(SETTING_INTERIOR_CELL_BUFFER));
    if interior < 1 {
        e.call(SETTING_SET_UINT, &args![SETTING_INTERIOR_CELL_BUFFER, 1u32]);
    }

    let count = setting_uint(e, SETTING_INTERIOR_CELL_BUFFER);
    let interior_buffer = e.call(OPERATOR_NEW, &args![array_bytes(count)]).u32();
    e.set(this, TES::pInteriorBuffer, Ptr::new(interior_buffer));
    let count = setting_uint(e, SETTING_EXTERIOR_CELL_BUFFER);
    let exterior_buffer = e.call(OPERATOR_NEW, &args![array_bytes(count)]).u32();
    e.set(this, TES::pExteriorBuffer, Ptr::new(exterior_buffer));

    let mut index = 0u32;
    while index < fn_004503f0(e, Ptr::new(SETTING_INTERIOR_CELL_BUFFER)) {
        let buffer = e.get(this, TES::pInteriorBuffer).addr();
        e.mem.set_u32(buffer.wrapping_add(index.wrapping_mul(4)), 0);
        index = index.wrapping_add(1);
    }
    let mut index = 0u32;
    while index < fn_004503f0(e, Ptr::new(SETTING_EXTERIOR_CELL_BUFFER)) {
        let buffer = e.get(this, TES::pExteriorBuffer).addr();
        e.mem.set_u32(buffer.wrapping_add(index.wrapping_mul(4)), 0);
        index = index.wrapping_add(1);
    }
    e.set(this, TES::iTempExteriorBufferSize, 0);
    e.set(this, TES::iTempInteriorBufferSize, 0);
    e.call(0x0045_0d80, &args![this]);

    e.set(this, TES::pObjRoot, obj_root);
    let temp_node_manager = e.call(NI_OPERATOR_NEW, &args![0xb0u32]).u32();
    let temp_node_manager = if temp_node_manager == 0 {
        0
    } else {
        fn_004505f0(e, Ptr::new(temp_node_manager)).addr()
    };
    e.set(this, TES::pTempNodeManager, Ptr::new(temp_node_manager));
    let root = e.get(this, TES::pObjRoot);
    if !root.is_null() {
        // `NiNode::AttachChild(pTempNodeManager, true)`.
        let manager = e.get(this, TES::pTempNodeManager);
        e.vcall(root.addr(), 0xdc, &args![manager, 1u32]);
    }
    e.set(this, TES::pObjLandRoot, obj_land_root);
    e.set(this, TES::pObjLODWaterRoot, obj_lod_water_root);
    e.set(this, TES::pSky, sky);
    let sky = e.get(this, TES::pSky);
    let light = if sky.is_null() {
        Ptr::NULL
    } else {
        fn_004505a0(e, sky)
    };
    e.set(this, TES::pObjLight, light);
    let sky = e.get(this, TES::pSky);
    let fog = if sky.is_null() {
        Ptr::NULL
    } else {
        fn_00450570(e, sky)
    };
    e.set(this, TES::pObjFog, fog);
    e.set(this, TES::bUpdateGridString, false);
    e.set(this, TES::pWorldSpace, Ptr::NULL);
    e.set(this, TES::pParticleCacheHead, Ptr::NULL);
    e.set(this, TES::bFadeWhenLoading, true);

    let first = e
        .call(SETTING_VALUE_ADDRESS_INT, &args![SETTING_GRIDS_TO_LOAD])
        .u32();
    let second = e
        .call(SETTING_VALUE_ADDRESS_INT, &args![SETTING_GRIDS_TO_LOAD])
        .u32();
    let cells = e.mem.u32(first).wrapping_mul(e.mem.u32(second));
    if e.global::<u32>(GRID_CELL_COUNT_HIGH_WATER_MARK) < cells {
        let first = e
            .call(SETTING_VALUE_ADDRESS_INT, &args![SETTING_GRIDS_TO_LOAD])
            .u32();
        let second = e
            .call(SETTING_VALUE_ADDRESS_INT, &args![SETTING_GRIDS_TO_LOAD])
            .u32();
        let cells = e.mem.u32(first).wrapping_mul(e.mem.u32(second));
        e.set_global(GRID_CELL_COUNT_HIGH_WATER_MARK, cells);
    }
    e.set(this, TES::bShowLANDborders, false);
    e.set(this, TES::bRunningCellTests, false);
    e.set(this, TES::bRunningCellTests2, false);
    e.set(this, TES::bAllowUnusedPurge, true);
    e.set(this, TES::pNavMeshInfoMap, Ptr::NULL);
    fn_004506a0(e, 0x0045_c680);

    let loaded_area_bound = e.call(NI_OPERATOR_NEW, &args![0x44u32]).u32();
    let loaded_area_bound = if loaded_area_bound == 0 {
        0
    } else {
        // The four floats are the constants at 01017868, 0101786c, 01013974
        // and 01017870, in that order.
        let a: f32 = e.global(0x0101_7868);
        let b: f32 = e.global(0x0101_786c);
        let c: f32 = e.global(0x0101_3974);
        let d: f32 = e.global(0x0101_7870);
        e.call(0x0062_6fd0, &args![loaded_area_bound, a, b, c, d])
            .u32()
    };
    e.call(
        0x0066_b0d0,
        &args![base + TES::spLoadedAreaBound.off, loaded_area_bound],
    );
    e.set(this, TES::pTACRegionFilter, Ptr::NULL);
    e.call(SCOPE_GUARD_DTOR, &args![guard]);
    e.mem.free(guard);
    this
}

// Translated from 004503f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The integer value of the INI setting `this` (`*(this + 4)`, through the
/// setting's value-pointer function).
pub fn fn_004503f0(e: &mut Engine, this: Ptr) -> u32 {
    let value = e.call(SETTING_VALUE_ADDRESS_INT, &args![this]).u32();
    e.mem.u32(value)
}

// Translated from 00450410 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float value of the INI setting `this` (returned in ST0).
pub fn fn_00450410(e: &mut Engine, this: Ptr) -> f32 {
    let value = e.call(SETTING_VALUE_ADDRESS_FLOAT, &args![this]).u32();
    e.mem.f32(value)
}

// Translated from 00450430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a flag in `011f4300` (read by `004ad1b0` in `bipedanim.cpp`).
pub fn fn_00450430(e: &mut Engine, value: u32) {
    e.set_global(0x011f_4300, value);
}

// Translated from 00450440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `bQueueWarnings:General` in `01202d62`.
pub fn fn_00450440(e: &mut Engine, value: u8) {
    e.set_global(0x0120_2d62, value);
}

// Translated from 00450450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the Havok update type (`iUpdateType:HAVOK`) in `012677b8`.
pub fn fn_00450450(e: &mut Engine, value: u32) {
    e.set_global(0x0126_77b8, value);
}

// Translated from 00450460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `fChaseDeltaMult:HAVOK` in `011afe70`.
pub fn fn_00450460(e: &mut Engine, value: f32) {
    e.set_global(0x011a_fe70, value);
}

// Translated from 00450470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a function pointer in entry `index` of the Havok object callback
/// table at `011afe88`.
pub fn fn_00450470(e: &mut Engine, index: u32, callback: u32) {
    e.set_global(0x011a_fe88u32.wrapping_add(index.wrapping_mul(4)), callback);
}

// Translated from 00450490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `bAddBipedWhenKeyframed:HAVOK` in `011afe59`.
pub fn fn_00450490(e: &mut Engine, value: u8) {
    e.set_global(0x011a_fe59, value);
}

// Translated from 004504a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `fDebrisMaxVelocity` in `012677c0`.
pub fn fn_004504a0(e: &mut Engine, value: f32) {
    e.set_global(0x0126_77c0, value);
}

// Translated from 004504b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `fDebrisMinExtent` in `012677bc`.
pub fn fn_004504b0(e: &mut Engine, value: f32) {
    e.set_global(0x0126_77bc, value);
}

// Translated from 004504c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `fMaxPickTime:HAVOK` in `011afe74`.
pub fn fn_004504c0(e: &mut Engine, value: f32) {
    e.set_global(0x011a_fe74, value);
}

// Translated from 004504d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `fMaxPickTimeVATS:HAVOK` in `011afe78`.
pub fn fn_004504d0(e: &mut Engine, value: f32) {
    e.set_global(0x011a_fe78, value);
}

// Translated from 004504e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `iEntityBatchRemoveRate:HAVOK` in `011afe7c`.
pub fn fn_004504e0(e: &mut Engine, value: u32) {
    e.set_global(0x011a_fe7c, value);
}

// Translated from 004504f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a float in entry `index` of the LOD fade-out multiplier table at
/// `011ad7b8` (read by `BSFadeNode`'s `CheckFadeRadius`).
pub fn fn_004504f0(e: &mut Engine, index: u32, value: f32) {
    e.set_global(0x011a_d7b8u32.wrapping_add(index.wrapping_mul(4)), value);
}

// Translated from 00450510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `fFadeInTime:LOD` in `011ad7e4`.
pub fn fn_00450510(e: &mut Engine, value: f32) {
    e.set_global(0x011a_d7e4, value);
}

// Translated from 00450520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `fFadeOutTime:LOD` in `011ad7e8`.
pub fn fn_00450520(e: &mut Engine, value: f32) {
    e.set_global(0x011a_d7e8, value);
}

// Translated from 00450530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `fFadeInThreshold:LOD` in `011ad7ec`.
pub fn fn_00450530(e: &mut Engine, value: f32) {
    e.set_global(0x011a_d7ec, value);
}

// Translated from 00450540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `fFadeOutThreshold:LOD` in `011ad7f0`.
pub fn fn_00450540(e: &mut Engine, value: f32) {
    e.set_global(0x011a_d7f0, value);
}

// Translated from 00450550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `fDistanceMultiplier:LOD` in `011ad7f4`.
pub fn fn_00450550(e: &mut Engine, value: f32) {
    e.set_global(0x011a_d7f4, value);
}

// Translated from 00450560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `bCheckPurgedTextureList:General` in `011f4460`.
pub fn fn_00450560(e: &mut Engine, value: u8) {
    e.set_global(0x011f_4460, value);
}

// Translated from 00450570 (decompiled, FalloutNV.exe 1.4.0.525)
/// For a `Sky`: the object `0043b230` returns for its `pAtmosphere` (+0x20),
/// or null when there is no atmosphere. `TES::TES` stores it as `pObjFog`.
pub fn fn_00450570(e: &mut Engine, this: Ptr) -> Ptr {
    let atmosphere = e.mem.u32(this.addr() + 0x20); // Sky::pAtmosphere (Xbox PDB)
    if atmosphere == 0 {
        Ptr::NULL
    } else {
        e.call(0x0043_b230, &args![atmosphere]).ptr()
    }
}

// Translated from 004505a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// For a `Sky`: the object `006838b0` returns for its `pSun` (+0x28), or
/// null when there is no sun. `TES::TES` stores it as `pObjLight`.
pub fn fn_004505a0(e: &mut Engine, this: Ptr) -> Ptr {
    let sun = e.mem.u32(this.addr() + 0x28); // Sky::pSun (Xbox PDB)
    if sun == 0 {
        Ptr::NULL
    } else {
        e.call(0x0068_38b0, &args![sun]).ptr()
    }
}

// Translated from 004505d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `TESObjectREFR::TransChangeCallback` (the Havok transform-change
/// callback, read by `SetTransform`) in `01267b68`.
pub fn fn_004505d0(e: &mut Engine, callback: u32) {
    e.set_global(0x0126_7b68, callback);
}

// Translated from 004505e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `TESObjectREFR::DebugDisplayCallback` (read by `PrepareDebugGeom`)
/// in `01267b64`.
pub fn fn_004505e0(e: &mut Engine, callback: u32) {
    e.set_global(0x0126_7b64, callback);
}

// Translated from 004505f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempNodeManager`'s constructor: the `NiNode` constructor with a
/// capacity of 0, then the class's vtable.
pub fn fn_004505f0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x00a5_ecb0, &args![this, 0u32]);
    e.mem.set_u32(this.addr(), BS_TEMP_NODE_MANAGER_VTABLE);
    this
}

// Translated from 00450620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempNodeManager::GetRTTI` (Xbox PDB): the class's `NiRTTI`.
pub fn bs_temp_node_manager_get_rtti(_e: &mut Engine, _this: Ptr) -> Ptr {
    Ptr::new(BS_TEMP_NODE_MANAGER_RTTI)
}

// Translated from 00450630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempNodeManager::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor (`00c44e20`) and, when bit 0 of `flags` is set, frees the
/// 0xB0-byte object.
pub fn bs_temp_node_manager_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x00c4_4e20, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_OPERATOR_DELETE, &args![this, 0xb0u32]);
    }
    this
}

// Translated from 00450670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `fMoveLimitMass:HAVOK` in `011b0128`.
pub fn fn_00450670(e: &mut Engine, value: f32) {
    e.set_global(0x011b_0128, value);
}

// Translated from 00450680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the square of `fCharacterControllerMultipleStepSpeed` in
/// `011b0150`.
pub fn fn_00450680(e: &mut Engine, value: f32) {
    e.set_global(0x011b_0150, value);
}

// Translated from 00450690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `fPhysicsDamage1Mass` in `01267bc0`.
pub fn fn_00450690(e: &mut Engine, value: f32) {
    e.set_global(0x0126_7bc0, value);
}

// Translated from 004506a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the ranged-node visibility callback
/// (`TES::RangedNodeVisibilityChangedCB`, `0045c680`) in `01202e08`; read by
/// `SetCurrent`.
pub fn fn_004506a0(e: &mut Engine, callback: u32) {
    e.set_global(0x0120_2e08, callback);
}

// Translated from 004506b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the LOD fade-out multiplier table (`011ad7b8`): entries 2 and 3 from
/// `fLODFadeOutMultItems` and `fLODFadeOutMultActors`, entries 0, 1, 4, 5, 8
/// and 10 from `fLODFadeOutMultObjects`.
pub fn fn_004506b0(e: &mut Engine) {
    let items = setting_float(e, SETTING_LOD_FADE_OUT_MULT_ITEMS);
    fn_004504f0(e, 2, items);
    let actors = setting_float(e, SETTING_LOD_FADE_OUT_MULT_ACTORS);
    fn_004504f0(e, 3, actors);
    let objects = setting_float(e, SETTING_LOD_FADE_OUT_MULT_OBJECTS);
    for index in [0, 1, 4, 5, 8, 10] {
        fn_004504f0(e, index, objects);
    }
}

// Translated from 00450770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::~TES`: tears the world down. Restores the vtable, flags the data
/// handler as clearing, clears the cell, form and Havok state of the other
/// subsystems, deletes the singletons the constructor created (clearing their
/// globals; `pGridCellA` keeps its dangling pointer), frees the two cell
/// buffers and runs the member destructors.
///
/// Callees in this unit that nobody has translated yet (`004539a0`,
/// `00458f90`, `0045ac80`, `0045a520`, `0045a1c0`, `0045a350`, `00450d00`,
/// `00450c20`, `0045bb80`) are called by address.
pub fn fn_00450770(e: &mut Engine, this: Ptr<TES>) {
    let base = this.addr();
    e.mem.set_u32(base, TES_VTABLE);
    fn_004506a0(e, 0);
    let data_handler: u32 = e.global(DATA_HANDLER);
    fn_00450b40(e, Ptr::new(data_handler), 1);
    e.call(0x0086_7840, &args![OBJECT_011DE7B8]);
    e.call(0x0052_7f40, &args![]);
    let object: u32 = e.global(OBJECT_WITH_SLOT_1CC);
    e.vcall(object, 0x1cc, &args![0u32, 0u32]);
    let save_load: u32 = e.global(SAVE_LOAD_GAME);
    e.call(0x008a_8150, &args![save_load, 0u32]);
    e.set(this, TES::pObjFog, Ptr::NULL);
    e.set(this, TES::pObjLight, Ptr::NULL);
    e.set(this, TES::pSky, Ptr::NULL);
    e.call(0x0097_7540, &args![OBJECT_011E0E80]);
    e.call(0x0045_39a0, &args![this, 0u32, 0u32]);
    fn_00450b60(e, this);
    e.call(0x0045_8f90, &args![this]);
    e.call(0x0065_1e30, &args![]);
    e.call(0x0066_4cd0, &args![0u32]);
    e.call(0x0045_ac80, &args![this]);
    e.call(0x0045_a520, &args![this]);
    e.call(0x0045_a1c0, &args![this]);
    e.call(0x0045_a350, &args![this]);
    e.call(0x0068_b4a0, &args![]);
    e.call(0x004d_5d50, &args![]);
    e.call(0x009c_6ba0, &args![VATS_OBJECT]);
    // `TESDataHandler::ClearData`.
    let data_handler: u32 = e.global(DATA_HANDLER);
    e.call(0x0045_dfe0, &args![data_handler]);
    e.call(0x0045_0d00, &args![this]);
    let data_handler: u32 = e.global(DATA_HANDLER);
    fn_00450b40(e, Ptr::new(data_handler), 1);
    e.call(0x00c4_59d0, &args![0u32]);

    let model_loader: u32 = e.global(MODEL_LOADER);
    if model_loader != 0 {
        fn_00450b90(e, Ptr::new(model_loader), 1);
    }
    e.set_global(MODEL_LOADER, 0u32);
    let io_manager: u32 = e.global(IO_MANAGER);
    if io_manager != 0 {
        e.vcall(io_manager, 0, &args![1u32]);
    }
    e.set_global(IO_MANAGER, 0u32);
    let task_manager: u32 = e.global(PARALLEL_TASK_MANAGER);
    if task_manager != 0 {
        fn_00450bc0(e, Ptr::new(task_manager), 1);
    }
    e.set_global(PARALLEL_TASK_MANAGER, 0u32);
    let grid_cells = e.get(this, TES::pGridCellA);
    if !grid_cells.is_null() {
        e.vcall(grid_cells.addr(), 0, &args![1u32]);
    }
    let save_load: u32 = e.global(SAVE_LOAD_GAME);
    if save_load != 0 {
        fn_00450bf0(e, Ptr::new(save_load), 1);
    }
    e.set_global(SAVE_LOAD_GAME, 0u32);
    e.call(0x0084_7500, &args![]);
    e.call(0x0084_fd30, &args![]);
    e.call(0x0066_b0d0, &args![base + TES::spLoadedAreaBound.off, 0u32]);
    let data_handler: u32 = e.global(DATA_HANDLER);
    if data_handler != 0 {
        // `TESDataHandler`'s scalar deleting destructor.
        e.call(0x0045_0c20, &args![data_handler, 1u32]);
    }
    e.set_global(DATA_HANDLER, 0u32);
    let interior_buffer = e.get(this, TES::pInteriorBuffer);
    e.call(OPERATOR_DELETE, &args![interior_buffer]);
    let exterior_buffer = e.get(this, TES::pExteriorBuffer);
    e.call(OPERATOR_DELETE, &args![exterior_buffer]);
    let list = e.call(0x0045_bb80, &args![this]).u32();
    e.call(0x0047_0470, &args![list]);
    e.call(0x004e_8000, &args![]);

    for index in 0..SHADOW_SCENE_NODE_COUNT {
        if fn_00450b80(e, index) != 0 {
            let node = fn_00450b80(e, index);
            e.call(0x00b5_d180, &args![node]);
            fn_00450b80(e, index);
            e.call(0x00b6_0040, &args![]);
        }
    }
    e.call(0x0062_5fb0, &args![]);
    e.call(0x004e_e920, &args![base + 0x6c]);
    e.call(0x0045_cec0, &args![base + TES::spLoadedAreaBound.off]);
    e.call(0x0044_cbf0, &args![base + TES::spPreloadedForms.off]);
    e.call(0x0045_cec0, &args![base + TES::BloodDecalPreload1.off]);
    e.call(0x0044_cbf0, &args![base + TES::spPreloadedAddonNodes.off]);
    e.call(0x0046_ffb0, &args![base + TES::DeadCount.off]);
    e.call(0x0046_ffb0, &args![base + TES::ListofBedsAndChairs.off]);
    e.call(0x0046_ffb0, &args![base + TES::listLastLoadedExteriors.off]);
    e.call(0x004e_e840, &args![base + 0x6c]);
}

// Translated from 00450b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `TESDataHandler::bClearingData` (+0x61D, Xbox PDB) on the data handler
/// `this`.
pub fn fn_00450b40(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x61d, value); // TESDataHandler::bClearingData (Xbox PDB)
}

// Translated from 00450b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00470470` on `TES::ListofBedsAndChairs` (+0x94).
pub fn fn_00450b60(e: &mut Engine, this: Ptr<TES>) {
    e.call(
        0x0047_0470,
        &args![this.addr() + TES::ListofBedsAndChairs.off],
    );
}

// Translated from 00450b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Entry `index` of the table of four shadow scene nodes at `011f91c8`.
pub fn fn_00450b80(e: &mut Engine, index: u32) -> u32 {
    e.global(SHADOW_SCENE_NODE_TABLE.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00450b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader`'s scalar deleting destructor: `ModelLoader::~ModelLoader`
/// (`00442aa0`), then `operator delete` when bit 0 of `flags` is set.
pub fn fn_00450b90(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_2aa0, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00450bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParallelTaskManager`'s scalar deleting destructor (`00c44b70`, then
/// `operator delete` when bit 0 of `flags` is set).
pub fn fn_00450bc0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x00c4_4b70, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00450bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSaveLoadGame`'s scalar deleting destructor (`00856870`, then
/// `operator delete` when bit 0 of `flags` is set).
pub fn fn_00450bf0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0085_6870, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00450c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESDataHandler`'s scalar deleting destructor: the destructor (`0045d970`),
/// then `operator delete` when bit 0 of `flags` is set. `TES::~TES` calls it by
/// address.
pub fn fn_00450c20(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0045_d970, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00450c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the water system: a `TESWaterSystem` (0xA0 bytes, constructor
/// `004e1650`) allocated under the memory tag `0x1d`, stored in
/// `TES::pWaterSystem` (a failed allocation stores null).
///
/// Not translated: the compiler's exception-unwinding frame.
pub fn fn_00450c50(e: &mut Engine, this: Ptr<TES>) {
    e.with_stack(SCOPE_GUARD_SIZE, |e, guard| {
        e.call(
            SCOPE_GUARD_CTOR,
            &args![guard, 0x1du32, 1u32, TES_CPP_PATH, 0x286u32],
        );
        let block = e.call(OPERATOR_NEW, &args![0xa0u32]).u32();
        let water_system = if block == 0 {
            0
        } else {
            e.call(0x004e_1650, &args![block]).u32()
        };
        e.set(this, TES::pWaterSystem, Ptr::new(water_system));
        e.call(SCOPE_GUARD_DTOR, &args![guard]);
    });
}

// Translated from 00450d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the water system (through its scalar deleting destructor
/// `00450d50`, with the delete flag) and clears `TES::pWaterSystem`.
pub fn fn_00450d00(e: &mut Engine, this: Ptr<TES>) {
    let water_system = e.get(this, TES::pWaterSystem);
    if !water_system.is_null() {
        fn_00450d50(e, water_system, 1);
    }
    e.set(this, TES::pWaterSystem, Ptr::NULL);
}

// Translated from 00450d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWaterSystem`'s scalar deleting destructor: the destructor (`004e19c0`),
/// then `operator delete` when bit 0 of `flags` is set.
pub fn fn_00450d50(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004e_19c0, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00450d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the centre cell and the queued and saved centre cells to
/// `0x7fffffff` (no cell): `iCurrentGridX/Y`, `iCurrentQueuedX/Y`,
/// `iSaveGridX/Y`.
pub fn fn_00450d80(e: &mut Engine, this: Ptr<TES>) {
    e.set(this, TES::iCurrentGridX, i32::MAX);
    e.set(this, TES::iCurrentGridY, i32::MAX);
    e.set(this, TES::iCurrentQueuedX, i32::MAX);
    e.set(this, TES::iCurrentQueuedY, i32::MAX);
    e.set(this, TES::iSaveGridX, i32::MAX);
    e.set(this, TES::iSaveGridY, i32::MAX);
}

// Translated from 00450dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::CullGridCells` (Xbox PDB): attaches (`detach` = 0) or detaches
/// (`detach` != 0) every loaded grid cell, with the bookkeeping around it:
/// the exterior Havok world is told first (`00c66300`/`00c66310`) and last
/// (`00c6b540`/`00c68f00`), `pObjLandRoot` is flagged (`0043b370`), the
/// distant terrain is switched (`006fd1f0`) and the land root is updated.
/// The second stack word is not used. Does nothing without a data handler.
///
/// A cell is attached when its load state is 3 and detached when it is 6.
pub fn tes_cull_grid_cells(e: &mut Engine, this: Ptr<TES>, detach: u8, _unused: u32) {
    let data_handler: u32 = e.global(DATA_HANDLER);
    if data_handler == 0 {
        return;
    }
    let world = fn_00451010(e);
    if world != 0 {
        let world = fn_00451010(e);
        if detach != 0 {
            e.call(0x00c6_6310, &args![world]);
        } else {
            e.call(0x00c6_6300, &args![world]);
        }
    }
    let mut x = 0u32;
    while x < grids_to_load(e) {
        let mut y = 0u32;
        while y < grids_to_load(e) {
            let grid_cells = e.get(this, TES::pGridCellA);
            let slot = e.call(GRID_CELL_ARRAY_GET, &args![grid_cells, x, y]).u32();
            let cell = e.mem.u32(slot);
            if detach != 0 {
                if cell != 0 && fn_00450ff0(e, Ptr::new(cell)) {
                    e.call(0x0055_2bd0, &args![cell, 0u32]);
                }
            } else if cell != 0 && fn_00450fb0(e, Ptr::new(cell)) {
                let root = e.get(this, TES::pObjRoot);
                let grid_cells = e.get(this, TES::pGridCellA);
                e.call(
                    GRID_CELL_ARRAY_ATTACH_TO_WORLD,
                    &args![grid_cells, root, x, y],
                );
                e.call(0x0054_bcf0, &args![cell, 0u32]);
            }
            y += 1;
        }
        x += 1;
    }
    let world = fn_00451010(e);
    if world != 0 {
        let world = fn_00451010(e);
        if detach != 0 {
            e.call(0x00c6_8f00, &args![world, 0u32]);
        } else {
            e.call(0x00c6_b540, &args![world, 0u32]);
        }
    }
    if detach == 0 {
        e.call(0x0097_5f90, &args![OBJECT_011E0E80]);
    }
    let land_root = e.get(this, TES::pObjLandRoot);
    let node = e.call(0x0096_11e0, &args![land_root]).u32();
    if node != 0 {
        fn_00450f90(e, Ptr::new(node), detach);
        e.call(0x006f_d1f0, &args![(detach == 0) as u32]);
    }
    e.with_stack(12, |e, update| {
        e.call(UPDATE_OBJECT_CTOR, &args![update, 0.0f32, 0u8, 0u8]);
        let land_root = e.get(this, TES::pObjLandRoot);
        e.call(NODE_UPDATE, &args![land_root, update]);
    });
}

// Translated from 00450f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` != 0) or clears bit 0 of the flags word at +0x30 of `this`
/// (a node), through `0043b370(flag, 1)`.
pub fn fn_00450f90(e: &mut Engine, this: Ptr, flag: u8) {
    e.call(0x0043_b370, &args![this, flag, 1u32]);
}

// Translated from 00450fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether a `TESObjectCELL`'s load state is 3.
pub fn fn_00450fb0(e: &mut Engine, this: Ptr) -> bool {
    fn_00450fd0(e, this) == 3
}

// Translated from 00450fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `TESObjectCELL`'s load state: `cCellState` (Xbox PDB `+0x36`), at +0x26
/// on the PC build.
pub fn fn_00450fd0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x26) // TESObjectCELL::cCellState (Xbox PDB)
}

// Translated from 00450ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether a `TESObjectCELL`'s load state is 6.
pub fn fn_00450ff0(e: &mut Engine, this: Ptr) -> bool {
    fn_00450fd0(e, this) == 6
}

// Translated from 00451010 (decompiled, FalloutNV.exe 1.4.0.525)
/// The exterior Havok world: the first word of `TESObjectCELL::spExteriorWorld`
/// (a `NiPointer<bhkWorldM>`, read through `00559450`).
pub fn fn_00451010(e: &mut Engine) -> u32 {
    e.call(0x0055_9450, &args![EXTERIOR_WORLD]).u32()
}

// Translated from 00451020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the world-space rectangle of the loaded grid into `out` (four
/// words): `[0]` and `[3]` are the x and y of the first loaded cell's corner
/// plus `margin`, `[2]` and `[1]` the x and y of the far corner (one cell
/// further) minus `margin`. Cells are 0x1000 units; `uGridsToLoad` is read
/// four times, as the code does, and each corner is truncated with `_ftol2`.
pub fn fn_00451020(e: &mut Engine, this: Ptr<TES>, out: Ptr, margin: f32) {
    let half_x = grids_to_load(e) >> 1;
    let first_x = e.get(this, TES::iCurrentGridX).wrapping_sub(half_x as i32);
    let half_y = grids_to_load(e) >> 1;
    let first_y = e.get(this, TES::iCurrentGridY).wrapping_sub(half_y as i32);
    let half_x = grids_to_load(e) >> 1;
    let last_x = (half_x as i32).wrapping_add(e.get(this, TES::iCurrentGridX));
    let half_y = grids_to_load(e) >> 1;
    let last_y = (half_y as i32).wrapping_add(e.get(this, TES::iCurrentGridY));
    let margin = margin as f64;
    let corner = first_x.wrapping_shl(12);
    let value = e.call(FTOL, &args![corner as f64 + margin]).i32();
    e.mem.set_i32(out.addr(), value);
    let corner = first_y.wrapping_shl(12);
    let value = e.call(FTOL, &args![corner as f64 + margin]).i32();
    e.mem.set_i32(out.addr() + 0xc, value);
    let corner = last_x.wrapping_shl(12).wrapping_add(0x1000);
    let value = e.call(FTOL, &args![corner as f64 - margin]).i32();
    e.mem.set_i32(out.addr() + 8, value);
    let corner = last_y.wrapping_shl(12).wrapping_add(0x1000);
    let value = e.call(FTOL, &args![corner as f64 - margin]).i32();
    e.mem.set_i32(out.addr() + 4, value);
}

// Translated from 00451110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the position `pos` (two floats, x and y) lies in the loaded grid:
/// always true while an interior cell is loaded; otherwise `pos` is rounded
/// (`00406d90`), divided by 0x1000 into cell coordinates, and compared with
/// the `uGridsToLoad` square around the centre cell.
pub fn fn_00451110(e: &mut Engine, this: Ptr<TES>, pos: Ptr) -> bool {
    if e.call(GET_INTERIOR_CELL, &args![this]).u32() != 0 {
        return true;
    }
    let x_value = e.mem.f32(pos.addr());
    let cell_x = e.call(FLOAT_TO_INT_ROUNDED, &args![x_value]).i32() >> 12;
    let y_value = e.mem.f32(pos.addr() + 4);
    let cell_y = e.call(FLOAT_TO_INT_ROUNDED, &args![y_value]).i32() >> 12;
    let half = grids_to_load(e) >> 1;
    let first_x = e.get(this, TES::iCurrentGridX).wrapping_sub(half as i32);
    let half = grids_to_load(e) >> 1;
    let first_y = e.get(this, TES::iCurrentGridY).wrapping_sub(half as i32);
    if cell_x < first_x {
        return false;
    }
    let count = grids_to_load(e);
    if cell_x >= first_x.wrapping_add(count as i32) {
        return false;
    }
    if cell_y < first_y {
        return false;
    }
    let count = grids_to_load(e);
    cell_y < first_y.wrapping_add(count as i32)
}

// Translated from 004511e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::IsCellLoaded` (Xbox PDB): false for a null `cell`; by load state
/// (`this` is not used): states 2 to 4 give `flag == 0`, states 5 and 6 give
/// true, every other state false.
pub fn tes_is_cell_loaded(e: &mut Engine, _this: Ptr<TES>, cell: Ptr, flag: u8) -> bool {
    if cell.is_null() {
        return false;
    }
    let index = (fn_00450fd0(e, cell) as u32).wrapping_sub(2);
    match index {
        0..=2 => flag == 0,
        3 | 4 => true,
        _ => false,
    }
}

// Translated from 00451250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` != 0) or clears bit 1 of a `TESObjectCELL`'s `cCellFlags`
/// (+0x24 on PC; Xbox PDB `+0x34`): the "has water" bit, which `004518e0`
/// reads.
pub fn fn_00451250(e: &mut Engine, cell: Ptr, flag: u8) {
    let flags = e.mem.u8(cell.addr() + 0x24); // TESObjectCELL::cCellFlags (Xbox PDB)
    let flags = if flag != 0 { flags | 2 } else { flags & !2 };
    e.mem.set_u8(cell.addr() + 0x24, flags);
}

// Translated from 004512a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a `TESObjectCELL`'s load state (`cCellState`, +0x26).
pub fn fn_004512a0(e: &mut Engine, cell: Ptr, state: u8) {
    e.mem.set_u8(cell.addr() + 0x26, state); // TESObjectCELL::cCellState (Xbox PDB)
}

// Translated from 004512c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the loaded grid to the position `pos` (two floats) and loads it.
/// Does nothing without a data handler, with an interior cell loaded, or
/// while it runs (the byte at `011c3ed0` guards against re-entry).
///
/// With `reload` != 0 (a full reload): the grid cell array is told (slot 8),
/// the terrain manager is updated from the player's position, the grid is
/// repositioned (slot 0x10 with the centre cell), the models and textures
/// are reset and the grid loaded (`00452ff0`), the player's camera
/// position goes to `0057d0a0`, a fader is created from
/// `fFadeToBlackFadeSeconds` unless cell tests are running, and the
/// furniture and bed lists are rebuilt. Without it: the grid is
/// repositioned and loaded. Both finish by updating the land root.
pub fn fn_004512c0(e: &mut Engine, this: Ptr<TES>, pos: Ptr, reload: u8) {
    let data_handler: u32 = e.global(DATA_HANDLER);
    if data_handler == 0 {
        return;
    }
    if e.call(GET_INTERIOR_CELL, &args![this]).u32() != 0 {
        return;
    }
    if e.global::<u8>(GRID_LOAD_IN_PROGRESS) != 0 {
        return;
    }
    e.set_global(GRID_LOAD_IN_PROGRESS, 1u8);
    if reload != 0 {
        let grid_cells = e.get(this, TES::pGridCellA);
        e.vcall(grid_cells.addr(), 0x8, &args![]);
        fn_00451520(e, 1);
        fn_00451570(e, 0);
        let world_space = e.get(this, TES::pWorldSpace);
        let terrain = e.call(GET_TERRAIN_MANAGER, &args![world_space]).u32();
        e.call(0x006f_ce00, &args![terrain]);
        let player: u32 = e.global(PLAYER_CHARACTER);
        // `PlayerCharacter` slot 0x1f4 returns the position (three floats).
        let player_position = e.vcall(player, 0x1f4, &args![]).u32();
        let world_space = e.get(this, TES::pWorldSpace);
        let terrain = e.call(GET_TERRAIN_MANAGER, &args![world_space]).u32();
        e.call(0x006f_ca90, &args![terrain, player_position, 0xfu32]);
        let grid_x = e.get(this, TES::iCurrentGridX);
        let grid_y = e.get(this, TES::iCurrentGridY);
        let grid_cells = e.get(this, TES::pGridCellA);
        e.vcall(grid_cells.addr(), 0x10, &args![grid_x, grid_y]);
        e.call(0x0045_5200, &args![this, 0u32, 0u32]);
        e.call(0x0045_2490, &args![this, 0u32]);
        e.call(0x0045_7be0, &args![this, 0u32]);
        e.call(0x0045_2ff0, &args![this]);
        let io_manager: u32 = e.global(IO_MANAGER);
        e.call(0x00c3_dfa0, &args![io_manager, 4u32]);
        // A 12-byte local: the position (x, y, 0.0) built by `00416870`; the
        // third float is then overwritten by `004572e0`.
        e.with_stack(12, |e, point| {
            let x = e.mem.f32(pos.addr());
            let y = e.mem.f32(pos.addr() + 4);
            e.call(0x0041_6870, &args![point, x, y, 0.0f32]);
            e.call(0x0045_72e0, &args![this, pos, point.addr() + 8]);
            fn_00451590(e, 0);
            let point_words = [
                e.mem.u32(point.addr()),
                e.mem.u32(point.addr() + 4),
                e.mem.u32(point.addr() + 8),
            ];
            let global_words = [
                e.global::<u32>(POINT_011A9478),
                e.global::<u32>(POINT_011A9478 + 4),
                e.global::<u32>(POINT_011A9478 + 8),
            ];
            e.call(
                0x0057_d0a0,
                &args![
                    point_words[0],
                    point_words[1],
                    point_words[2],
                    global_words[0],
                    global_words[1],
                    global_words[2],
                    1.0f32
                ],
            );
        });
        fn_00451590(e, 1);
        if !fn_00451530(e, this) {
            let seconds = setting_float(e, SETTING_FADE_TO_BLACK_FADE_SECONDS);
            let fader_manager: u32 = e.global(FADER_MANAGER);
            // `FaderManager::CreateFader` (Xbox PDB): the trailing 0 is the
            // word the code pushed before reading the setting.
            e.call(0x0070_0960, &args![fader_manager, 1u32, seconds, 0u32]);
        }
        e.call(0x0045_9870, &args![this]);
        e.call(0x0097_2d30, &args![OBJECT_011E0E80]);
        e.call(0x0097_2bb0, &args![OBJECT_011E0E80]);
    } else {
        let grid_x = e.get(this, TES::iCurrentGridX);
        let grid_y = e.get(this, TES::iCurrentGridY);
        let grid_cells = e.get(this, TES::pGridCellA);
        e.vcall(grid_cells.addr(), 0x10, &args![grid_x, grid_y]);
        e.call(0x0045_7be0, &args![this, 1u32]);
        e.call(0x0045_2ff0, &args![this]);
    }
    e.call(0x0045_5490, &args![this]);
    e.with_stack(12, |e, update| {
        e.call(UPDATE_OBJECT_CTOR, &args![update, 0.0f32, 0u8, 0u8]);
        let land_root = e.get(this, TES::pObjLandRoot);
        e.call(NODE_UPDATE, &args![land_root, update]);
    });
    e.call(0x0045_2490, &args![this, 0u32]);
    let data_handler: u32 = e.global(DATA_HANDLER);
    e.call(0x0046_0360, &args![data_handler]);
    e.set_global(GRID_LOAD_IN_PROGRESS, 0u8);
}

// Translated from 00451520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte in `01189184` (also written by `ClearCanopyShadowMaskTexture`).
pub fn fn_00451520(e: &mut Engine, value: u8) {
    e.set_global(GRID_FLAG_01189184, value);
}

// Translated from 00451530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `bRunningCellTests` or `bRunningCellTests2` is set.
pub fn fn_00451530(e: &mut Engine, this: Ptr<TES>) -> bool {
    e.get(this, TES::bRunningCellTests) || e.get(this, TES::bRunningCellTests2)
}

// Translated from 00451570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `0066b0d0` on the object at `011f95e8` with `value`.
pub fn fn_00451570(e: &mut Engine, value: u32) {
    e.call(0x0066_b0d0, &args![0x011f_95e8u32, value]);
}

// Translated from 00451590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte in `011ad86c`.
pub fn fn_00451590(e: &mut Engine, value: u8) {
    e.set_global(GRID_FLAG_011AD86C, value);
}

// Translated from 004515a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Centres the grid on `pos` (two floats; null keeps the current centre) and
/// loads it (`004512c0` with `flag`), then calls `0062f460`. Runs when a data
/// handler exists, no interior cell is loaded and the data handler's
/// `bSaveLoad` (+0x61a) is clear, or when `0047c850` (called on the
/// `TESSaveLoadGame` object) says so. Without a world space, the data
/// handler's current one (the object `00460140` returns) is set first.
pub fn fn_004515a0(e: &mut Engine, this: Ptr<TES>, pos: Ptr, flag: u8) {
    let save_load: u32 = e.global(SAVE_LOAD_GAME);
    let forced = e.call(0x0047_c850, &args![save_load]).u8();
    let data_handler: u32 = e.global(DATA_HANDLER);
    let normal = data_handler != 0
        && e.call(GET_INTERIOR_CELL, &args![this]).u32() == 0
        && !fn_004516b0(e, Ptr::new(data_handler));
    if !normal && forced == 0 {
        return;
    }
    let saved_x = e.get(this, TES::iCurrentGridX);
    let saved_y = e.get(this, TES::iCurrentGridY);
    if e.call(GET_WORLD_SPACE, &args![this]).u32() == 0 {
        let data_handler: u32 = e.global(DATA_HANDLER);
        let holder = e.call(0x0046_0140, &args![data_handler]).u32();
        let slot = e.call(0x0068_15c0, &args![holder]).u32();
        let world_space = e.mem.u32(slot);
        e.call(0x0045_8200, &args![this, world_space]);
    }
    if pos.is_null() {
        e.set(this, TES::iCurrentGridX, saved_x);
        e.set(this, TES::iCurrentGridY, saved_y);
    } else {
        let x_value = e.mem.f32(pos.addr());
        let cell_x = e.call(FLOAT_TO_INT_ROUNDED, &args![x_value]).i32() >> 12;
        e.set(this, TES::iCurrentGridX, cell_x);
        let y_value = e.mem.f32(pos.addr() + 4);
        let cell_y = e.call(FLOAT_TO_INT_ROUNDED, &args![y_value]).i32() >> 12;
        e.set(this, TES::iCurrentGridY, cell_y);
    }
    let grid_x = e.get(this, TES::iCurrentGridX);
    e.set(this, TES::iCurrentQueuedX, grid_x);
    let grid_y = e.get(this, TES::iCurrentGridY);
    e.set(this, TES::iCurrentQueuedY, grid_y);
    fn_004512c0(e, this, pos, flag);
    e.call(0x0062_f460, &args![]);
}

// Translated from 004516b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESDataHandler`'s `bSaveLoad` (+0x61a, Xbox PDB).
pub fn fn_004516b0(e: &mut Engine, data_handler: Ptr) -> bool {
    e.mem.u8(data_handler.addr() + 0x61a) != 0 // TESDataHandler::bSaveLoad (Xbox PDB)
}

// Translated from 004516d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::FindFirstGridCellWithWater` (Xbox PDB): searches the loaded grid
/// outwards from its centre, ring by ring (the centre cell, then the square
/// ring `n` cells away: first the two rows, then the two columns), and
/// returns the address of the grid slot (`GridCellArray::Get`) of the first
/// loaded cell whose "has water" bit (`004518e0`) is set, or null.
pub fn tes_find_first_grid_cell_with_water(e: &mut Engine, this: Ptr<TES>) -> Ptr {
    let grids = grids_to_load(e);
    let half = grids >> 1;
    let center = half;
    let mut found = 0u32;
    let mut ring = 0u32;
    while ring < half {
        if ring == 0 {
            found = grid_slot_with_water(e, this, center, center);
        } else {
            let high = center.wrapping_add(ring);
            let low = center.wrapping_sub(ring);
            let mut x = low;
            while x <= high {
                found = grid_slot_with_water(e, this, x, high);
                if found != 0 {
                    break;
                }
                found = grid_slot_with_water(e, this, x, low);
                if found != 0 {
                    break;
                }
                x = x.wrapping_add(1);
            }
            if found == 0 {
                let mut y = low.wrapping_add(1);
                while y < high {
                    found = grid_slot_with_water(e, this, low, y);
                    if found != 0 {
                        break;
                    }
                    found = grid_slot_with_water(e, this, high, y);
                    if found != 0 {
                        break;
                    }
                    y = y.wrapping_add(1);
                }
            }
        }
        if found != 0 {
            break;
        }
        ring += 1;
    }
    Ptr::new(found)
}

/// The grid slot `(x, y)` when it holds a cell with the "has water" bit set,
/// else 0 (the test the water search repeats).
pub(crate) fn grid_slot_with_water(e: &mut Engine, this: Ptr<TES>, x: u32, y: u32) -> u32 {
    let grid_cells = e.get(this, TES::pGridCellA);
    let slot = e.call(GRID_CELL_ARRAY_GET, &args![grid_cells, x, y]).u32();
    if slot != 0 {
        let cell = e.mem.u32(slot);
        if cell != 0 && fn_004518e0(e, Ptr::new(cell)) {
            return slot;
        }
    }
    0
}

// Translated from 004518e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether a `TESObjectCELL`'s "has water" bit (bit 1 of `cCellFlags`, +0x24)
/// is set.
pub fn fn_004518e0(e: &mut Engine, cell: Ptr) -> bool {
    e.mem.u8(cell.addr() + 0x24) & 2 != 0 // TESObjectCELL::cCellFlags (Xbox PDB)
}

// Translated from 00451900 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cell at cell coordinates `(x, y)`: the loaded grid's cell when
/// `(x, y)` is inside the `uGridsToLoad` square around the centre cell and
/// the slot is set; otherwise, when there is a world space, what
/// `00461c20(data handler, x, y, world space, 0)` returns; else null.
pub fn fn_00451900(e: &mut Engine, this: Ptr<TES>, x: i32, y: i32) -> u32 {
    let half = grids_to_load(e) >> 1;
    let relative_x =
        (x as u32).wrapping_sub(e.get(this, TES::iCurrentGridX).wrapping_sub(half as i32) as u32);
    let half = grids_to_load(e) >> 1;
    let relative_y =
        (y as u32).wrapping_sub(e.get(this, TES::iCurrentGridY).wrapping_sub(half as i32) as u32);
    let mut cell = 0u32;
    if relative_x < grids_to_load(e)
        && relative_y < grids_to_load(e)
        && (relative_x as i32) >= 0
        && (relative_y as i32) >= 0
    {
        let grid_cells = e.get(this, TES::pGridCellA);
        let slot = e
            .call(
                GRID_CELL_ARRAY_GET,
                &args![grid_cells, relative_x, relative_y],
            )
            .u32();
        cell = e.mem.u32(slot);
    }
    if cell == 0 && e.call(GET_WORLD_SPACE, &args![this]).u32() != 0 {
        let world_space = e.call(GET_WORLD_SPACE, &args![this]).u32();
        let data_handler: u32 = e.global(DATA_HANDLER);
        cell = e
            .call(0x0046_1c20, &args![data_handler, x, y, world_space, 0u32])
            .u32();
    }
    cell
}

// Translated from 004519d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The loaded cell at the world position `(x, y)`: the interior cell when one
/// is loaded; otherwise the exterior cell (`00451900` at `x / 0x1000`,
/// `y / 0x1000`, truncated) when it is loaded (`TES::IsCellLoaded` on the
/// `TES` singleton with `flag` 1), else null. The third stack word is not
/// used.
pub fn fn_004519d0(e: &mut Engine, this: Ptr<TES>, x: f32, y: f32, _unused: u32) -> u32 {
    if e.call(GET_INTERIOR_CELL, &args![this]).u32() != 0 {
        return e.call(GET_INTERIOR_CELL, &args![this]).u32();
    }
    let cell_x = e.call(FTOL, &args![x as f64]).i32() >> 12;
    let cell_y = e.call(FTOL, &args![y as f64]).i32() >> 12;
    let cell = fn_00451900(e, this, cell_x, cell_y);
    if cell != 0 {
        let tes: u32 = e.global(TES_SINGLETON);
        if tes_is_cell_loaded(e, Ptr::new(tes), Ptr::new(cell), 1) {
            return cell;
        }
    }
    0
}

// Translated from 00451a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::LoadGridCell` (Xbox PDB): loads the cell at world cell
/// `(cell_x, cell_y)` and puts it in grid slot `(array_x, array_y)`.
/// Does nothing without a world space. A cell the world space does not have
/// is created by `00461c20` and gets its persistent references
/// (`TESWorldSpace::AssignPersistentRefsToCell`) and temp data. A slot that
/// is already attached only has the cell added to the buffer
/// (`TES::AddToBuffer`). Otherwise the cell is stored in the slot and
/// attached to the scene root, its land mesh is loaded (or updated) and its
/// "has water" bit set from the lowest land height against the water
/// height; with `bUseWater`, a cell with water gets its placeable water
/// generated; the land is then updated with `bShowLANDborders`.
///
/// The "Loading cell..." message is formatted into a 268-byte buffer and not
/// used further, as in the game.
pub fn tes_load_grid_cell(
    e: &mut Engine,
    this: Ptr<TES>,
    array_x: u32,
    array_y: u32,
    cell_x: i32,
    cell_y: i32,
) {
    if e.call(GET_WORLD_SPACE, &args![this]).u32() == 0 {
        return;
    }
    let world_space = e.call(GET_WORLD_SPACE, &args![this]).u32();
    let mut cell = e
        .call(0x0058_5b30, &args![world_space, cell_x, cell_y])
        .u32();
    if cell == 0 {
        let world_space = e.call(GET_WORLD_SPACE, &args![this]).u32();
        let data_handler: u32 = e.global(DATA_HANDLER);
        cell = e
            .call(
                0x0046_1c20,
                &args![data_handler, cell_x, cell_y, world_space, 1u32],
            )
            .u32();
        let world_space = e.call(GET_WORLD_SPACE, &args![this]).u32();
        e.call(0x0058_8150, &args![world_space, cell]);
        e.call(0x0055_1440, &args![cell, 1u32]);
    }
    let grid_cells = e.get(this, TES::pGridCellA);
    if e.call(0x004b_a5a0, &args![grid_cells, array_x, array_y])
        .bool()
    {
        e.call(0x0045_4b90, &args![this, cell]);
        return;
    }
    let form_id = e.call(0x0084_e3a0, &args![cell]).u32();
    let data_y = e.call(CELL_GET_DATA_Y, &args![cell]).u32();
    let data_x = e.call(CELL_GET_DATA_X, &args![cell]).u32();
    let name = fn_00451cb0(e, Ptr::new(cell));
    e.with_stack(268, |e, message| {
        e.call(
            0x00ec_623a,
            &args![message, LOADING_CELL_FORMAT, name, data_x, data_y, form_id],
        );
    });
    let grid_cells = e.get(this, TES::pGridCellA);
    e.call(0x004b_a550, &args![grid_cells, array_x, array_y, cell]);
    let root = e.get(this, TES::pObjRoot);
    let grid_cells = e.get(this, TES::pGridCellA);
    e.call(
        GRID_CELL_ARRAY_ATTACH_TO_WORLD,
        &args![grid_cells, root, array_x, array_y],
    );
    let land = e.call(0x0054_6fb0, &args![cell]).u32();
    if !e.call(0x0053_5b90, &args![land, 1u32]).bool() {
        e.call(0x0053_a090, &args![land, 0u32, 0u32]);
    }
    e.with_stack(8, |e, heights| {
        let cell_land = e.call(0x0054_6fb0, &args![cell]).u32();
        e.call(0x0053_f440, &args![cell_land, heights]);
        let water_height = e.call(0x0054_71e0, &args![cell]).f64();
        let lowest = e.mem.f32(heights.addr()) as f64;
        // `FCOMPP`: ordered and lowest land height below the water height.
        fn_00451250(e, Ptr::new(cell), (lowest < water_height) as u8);
    });
    if setting_byte(e, SETTING_USE_WATER) != 0
        && fn_004518e0(e, Ptr::new(cell))
        && fn_00451cd0(e, Ptr::new(cell)) == 0
    {
        e.call(0x0049_c860, &args![cell]);
    }
    let show_borders = e.get(this, TES::bShowLANDborders);
    e.call(0x0053_fa40, &args![land, show_borders]);
}

// Translated from 00451cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00401280` on the cell and returns its result (the map names it
/// `BaseProcess::GetCurrentHeadTrackTypeString`; the body returns a constant
/// string pointer, `01011584`).
pub fn fn_00451cb0(e: &mut Engine, cell: Ptr) -> u32 {
    e.call(0x0040_1280, &args![cell]).u32()
}

// Translated from 00451cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `TESObjectCELL`'s `bAutoWaterLoaded` (+0x54 on PC; Xbox PDB `+0x64`).
pub fn fn_00451cd0(e: &mut Engine, cell: Ptr) -> u8 {
    e.mem.u8(cell.addr() + 0x54) // TESObjectCELL::bAutoWaterLoaded (Xbox PDB)
}

// Translated from 00451cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::UpdateGridLights` (Xbox PDB): attaches the lights (`TESObjectCELL::
/// AttachLights`, flag 1) of every loaded cell whose load state is 6, then
/// updates shadow scene node 0 (`00b5ddb0(1, 1)`).
pub fn tes_update_grid_lights(e: &mut Engine, this: Ptr<TES>) {
    let mut x = 0u32;
    while x < grids_to_load(e) {
        let mut y = 0u32;
        while y < grids_to_load(e) {
            let grid_cells = e.get(this, TES::pGridCellA);
            let slot = e.call(GRID_CELL_ARRAY_GET, &args![grid_cells, x, y]).u32();
            let cell = e.mem.u32(slot);
            if cell != 0 && fn_00450ff0(e, Ptr::new(cell)) {
                e.call(0x0054_ba80, &args![cell, 1u32]);
            }
            y += 1;
        }
        x += 1;
    }
    let shadow_scene_node = fn_00450b80(e, 0);
    e.call(0x00b5_ddb0, &args![shadow_scene_node, 1u32, 1u32]);
}

// Translated from 00451da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// For a reference `refr` (`this` is not used) for which `00564e60` and
/// `005651e0` hold and whose virtual slot 0x1d0 returns an object: gives that
/// object to slot 0xe8 of the object `009611e0` derives from it (when there
/// is one), then calls `00570f70(refr, 0)` and
/// `TESActorBase::SetStartsDead(refr, 0)` (`00565210`).
pub fn fn_00451da0(e: &mut Engine, _this: Ptr<TES>, refr: Ptr) {
    let r = refr.addr();
    if !e.call(0x0056_4e60, &args![r]).bool() {
        return;
    }
    if e.vcall(r, 0x1d0, &args![]).u32() == 0 {
        return;
    }
    if !e.call(0x0056_51e0, &args![r]).bool() {
        return;
    }
    let parent = e.vcall(r, 0x1d0, &args![]).u32();
    let handler = e.call(0x0096_11e0, &args![parent]).u32();
    if handler != 0 {
        let parent = e.vcall(r, 0x1d0, &args![]).u32();
        e.vcall(handler, 0xe8, &args![parent]);
    }
    e.call(0x0057_0f70, &args![r, 0u32]);
    e.call(0x0056_5210, &args![r, 0u32]);
}

// Translated from 00451e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the reference `refr` (`this` is not used) is to be loaded: neither
/// `00440d80` nor `00440da0` says no. (The form type the code also reads
/// through `00401170` is not used.)
pub fn fn_00451e40(e: &mut Engine, _this: Ptr<TES>, refr: Ptr) -> bool {
    let form = e.call(REFERENCE_GET_BASE_FORM, &args![refr]).u32();
    e.call(FORM_GET_TYPE, &args![form]);
    if e.call(0x0044_0d80, &args![refr]).bool() {
        return false;
    }
    !e.call(0x0044_0da0, &args![refr]).bool()
}

// Translated from 00451ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// For an actor reference `refr` (`this` is not used; virtual slot 0x100
/// true): runs slot 0x260 on it and takes it off the temp-change list of
/// `ProcessLists` (`0096f400` on `011e0e80`).
pub fn fn_00451ea0(e: &mut Engine, _this: Ptr<TES>, refr: Ptr) {
    let r = refr.addr();
    if e.vcall(r, 0x100, &args![]).bool() {
        e.vcall(r, 0x260, &args![]);
        e.call(0x0096_f400, &args![OBJECT_011E0E80, r]);
    }
}

// Translated from 00451ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads one reference `refr` of `cell` into the scene (`this` is the `TES`).
/// `source` is an optional object that can supply the reference's 3D
/// (`005d8710` returns it); when `flag` is 0 the actor is first taken off the
/// temp-change list (`00451ea0`).
///
/// A null reference does nothing; a reference whose virtual slot 0x224 or
/// 0x220 is true is handed to slot 0x308 or 0x304 and nothing else is done.
/// The 3D (`node`) comes from `source`, or from slot 0x1c8. With a node: for
/// form types 0x2c and 0x2d it is given to slot 0xe8 of the object
/// `009611e0` derives from it; otherwise the cell's 3D is loaded, the node is
/// added to shadow scene node 0 (and registered for actors), the reference
/// is attached (`TESObjectCELL::AttachReference3D`) unless it is a type 0x23
/// form `00452440` rejects, the master particle addon nodes are removed
/// (form flag `0x1000000` and `0057b200`) or added (loaded-data flag 2, in a
/// cell of load state 6 or 5), and the object lighting is updated (form type
/// 0x1e takes `004534f0` instead). Always: the script is initialised and,
/// unless `0042ce10` says otherwise, its action list set up. For a type 0x25
/// form with the cast and size conditions, `00547ad0` is called with the
/// position and a scaled integer. An actor finally gets
/// `Actor::UpdateAlpha` and, unless it is the player, its reference ID set on
/// the scene graph.
///
/// Argument counts come from the disassembly: the words pushed before the
/// virtual calls `0x1bc` and `0x1f4` are arguments of the call that follows
/// (`00547ad0`, `RET 0x14`), not of the virtual calls.
pub fn fn_00451ef0(e: &mut Engine, this: Ptr<TES>, refr: Ptr, cell: Ptr, source: Ptr, flag: u8) {
    if refr.is_null() {
        return;
    }
    let r = refr.addr();
    if e.vcall(r, 0x224, &args![]).bool() {
        e.vcall(r, 0x308, &args![]);
        return;
    }
    if e.vcall(r, 0x220, &args![]).bool() {
        e.vcall(r, 0x304, &args![]);
        return;
    }
    fn_00451da0(e, this, refr);
    let form = e.call(REFERENCE_GET_BASE_FORM, &args![r]).u32();
    let kind = e.call(FORM_GET_TYPE, &args![form]).u8();
    if !fn_00451e40(e, this, refr) {
        return;
    }
    if flag == 0 {
        fn_00451ea0(e, this, refr);
    }
    let node;
    if !source.is_null() && e.call(0x005d_8710, &args![source]).u32() != 0 {
        node = e.call(0x005d_8710, &args![source]).u32();
        if e.call(0x0043_fcd0, &args![r]).u32() == 0 {
            e.call(0x0057_0f70, &args![r, node]);
        }
    } else {
        node = e.vcall(r, 0x1c8, &args![0u32]).u32();
    }
    if !source.is_null() {
        e.vcall(source.addr(), 0x40, &args![]);
    }
    if node != 0 {
        if kind == 0x2c || kind == 0x2d {
            let handler = e.call(0x0096_11e0, &args![node]).u32();
            if handler != 0 {
                let handler = e.call(0x0096_11e0, &args![node]).u32();
                e.vcall(handler, 0xe8, &args![node]);
            }
        } else {
            e.call(0x0054_5cf0, &args![cell]);
            let shadow_scene_node = fn_00450b80(e, 0);
            e.call(0x00b5_eeb0, &args![shadow_scene_node, node]);
            if e.vcall(r, 0x100, &args![]).bool() {
                let shadow_scene_node = fn_00450b80(e, 0);
                e.call(0x00b5_cbd0, &args![shadow_scene_node, node]);
            }
            if e.call(0x0056_4e60, &args![r]).bool() {
                fn_00452390(e, cell);
            }
            let mut attach = true;
            let form = e.call(REFERENCE_GET_BASE_FORM, &args![r]).u32();
            if e.call(FORM_GET_TYPE, &args![form]).u32() == 0x23 {
                let form = e.call(REFERENCE_GET_BASE_FORM, &args![r]).u32();
                if e.call(0x0045_2440, &args![form]).bool() {
                    attach = false;
                }
            }
            if attach {
                e.call(0x0054_8880, &args![cell, r, 0u32]);
                e.call(0x0056_f700, &args![r]);
            }
            if fn_00452370(e, refr) && e.call(0x0057_b200, &args![r]).bool() {
                e.call(0x0057_8170, &args![node]);
            } else if fn_004523e0(e, refr, 2) && (fn_00450ff0(e, cell) || fn_004523c0(e, cell)) {
                e.call(0x0057_8060, &args![node]);
            }
            if e.vcall(r, 0x100, &args![]).bool() {
                e.call(0x0048_3710, &args![r]);
            }
            if kind == 0x1e {
                let extra = e.call(0x0043_0830, &args![r]).u32();
                let form = e.call(REFERENCE_GET_BASE_FORM, &args![r]).u32();
                e.call(0x0045_34f0, &args![form, extra]);
            } else {
                let shadow_scene_node = fn_00450b80(e, 0);
                e.call(0x00b5_d9f0, &args![shadow_scene_node, node, 0u32]);
            }
        }
    }
    e.call(0x0056_5730, &args![r]);
    let script_context: u32 = e.global(SCRIPT_CONTEXT);
    if !e.call(0x0042_ce10, &args![script_context]).bool() {
        let actions = e.call(0x005d_43c0, &args![r]).u32();
        e.call(0x005a_c190, &args![r, actions]);
        let actions = e.call(0x005d_43c0, &args![r]).u32();
        e.call(0x005a_c750, &args![r, actions, 0x1000u32]);
    }
    if kind == 0x25 {
        let form = e.call(REFERENCE_GET_BASE_FORM, &args![r]).u32();
        let cast = e
            .call(
                0x00ec_43fb,
                &args![form, 0u32, RTTI_TESFORM, RTTI_CAST_TARGET, 0u32],
            )
            .u32();
        if cast != 0 && e.call(0x0045_2480, &args![]).bool() {
            let sizes = e.call(0x0045_bb80, &args![cast]).u32();
            let first = e.mem.f32(sizes);
            let second = e.mem.f32(sizes + 4);
            let limit: f64 = e.global(LARGE_CELL_LIMIT);
            if first as f64 > limit && second as f64 > limit {
                let radius = e.vcall(cast, 0x1bc, &args![]).f64();
                let scale = e.call(0x0056_7400, &args![r]).f64();
                // `fistp` with the rounding mode set to truncate; only the
                // low word of the 64-bit result is used.
                let count = (scale * radius) as i64 as i32;
                let position = e.vcall(r, 0x1f4, &args![]).u32();
                let x = e.mem.u32(position);
                let y = e.mem.u32(position + 4);
                let z = e.mem.u32(position + 8);
                e.call(0x0054_7ad0, &args![cell, x, y, z, count, 1.0f32]);
            }
        }
    }
    if e.vcall(r, 0x100, &args![]).bool() {
        e.call(0x008c_4640, &args![r]);
        let player: u32 = e.global(PLAYER_CHARACTER);
        if r != player {
            let form_id = e.call(0x0084_e3a0, &args![r]).u32();
            let node_3d = e.call(0x0043_fcd0, &args![r]).u32();
            e.call(0x004b_6dc0, &args![node_3d, form_id]);
        }
    }
}

// Translated from 00452370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `0x1000000` of a form's `iFormFlags` (+8) is set.
pub fn fn_00452370(e: &mut Engine, form: Ptr) -> bool {
    e.mem.u32(form.addr() + 8) & 0x0100_0000 != 0 // TESForm::iFormFlags (Xbox PDB)
}

// Translated from 00452390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds 1 to the 16-bit counter at +0xaa of a `TESObjectCELL`; by its use
/// (called for references visible when distant) it counts the loaded
/// references with visible distant (`sNumLoadedRefsWithVisibleDistant`, Xbox
/// PDB `+0x8a`).
pub fn fn_00452390(e: &mut Engine, cell: Ptr) {
    let count = e.mem.u16(cell.addr() + 0xaa);
    e.mem.set_u16(cell.addr() + 0xaa, count.wrapping_add(1));
}

// Translated from 004523c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether a `TESObjectCELL`'s load state is 5.
pub fn fn_004523c0(e: &mut Engine, cell: Ptr) -> bool {
    fn_00450fd0(e, cell) == 5
}

// Translated from 004523e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// For a `TESObjectREFR`: false without loaded data (`pLoadedData`, +0x64 on
/// PC; Xbox PDB `+0x74`), else whether the loaded data has any of the bits
/// `mask` (`00452420`).
pub fn fn_004523e0(e: &mut Engine, refr: Ptr, mask: u32) -> bool {
    let loaded_data = e.mem.u32(refr.addr() + 0x64); // TESObjectREFR::pLoadedData (Xbox PDB)
    if loaded_data == 0 {
        false
    } else {
        fn_00452420(e, Ptr::new(loaded_data), mask)
    }
}

// Translated from 00452420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the `LOADED_REF_DATA` flags word (+0x10) has any of the bits
/// `mask`.
pub fn fn_00452420(e: &mut Engine, loaded_data: Ptr, mask: u32) -> bool {
    e.mem.u32(loaded_data.addr() + 0x10) & mask != 0
}

// Globals, settings and callees of the cell-update code (session 3).

/// `bBackgroundCellLoads:BackgroundLoad`.
pub(crate) const SETTING_BACKGROUND_CELL_LOADS: u32 = 0x011c_9670;
/// `bPreemptivelyUnloadCells:General`.
pub(crate) const SETTING_PREEMPTIVELY_UNLOAD_CELLS: u32 = 0x011c_3d40;
/// `fAnimationMult:General`.
pub(crate) const SETTING_ANIMATION_MULT: u32 = 0x011c_5724;
/// `bHavokDebug:HAVOK`.
pub(crate) const SETTING_HAVOK_DEBUG: u32 = 0x0126_7b28;
/// The `ExteriorCellLoader` singleton pointer.
pub(crate) const EXTERIOR_CELL_LOADER: u32 = 0x011c_9618;
/// The `Main` singleton pointer (`Main::RenderMenuBackground` is `00871dc0`).
pub(crate) const MAIN_OBJECT: u32 = 0x011d_ea0c;
/// The movie player pointer (`MoviePlayer::GetPlayingSequence` is `00ec17c0`).
pub(crate) const MOVIE_PLAYER: u32 = 0x0126_fac4;
/// The `FOCollisionListener` singleton pointer (0xC bytes, built by
/// `00623430` on first use).
pub(crate) const COLLISION_LISTENER: u32 = 0x011c_c224;
/// An object `00453a80`'s sibling `004537b0` returns (passed to `0087aa90`).
pub(crate) const TASK_QUEUE_OBJECT: u32 = 0x011d_f1a8;
/// `BSAudio::QInstance` (Xbox PDB) returns this.
pub(crate) const AUDIO_INSTANCE: u32 = 0x011f_6d98;
/// The object `00453550` and `004537c0` pass to `0084d030` (a float getter).
pub(crate) const FRAME_TIME_OBJECT: u32 = 0x011f_6394;
/// The object `00452510` and `00452530` pass to `0040fbf0` and `0040fba0`
/// (a counted lock).
pub(crate) const CLEANUP_LOCK: u32 = 0x011f_4480;
/// Byte read by `00452540`: whether texture purging is allowed at all.
pub(crate) const PURGE_ALLOWED: u32 = 0x011c_70e9;
/// Byte `00452480` returns.
pub(crate) const FLAG_0119B8C8: u32 = 0x0119_b8c8;
/// Byte stored by `00452e40`.
pub(crate) const FLAG_011AF70C: u32 = 0x011a_f70c;
/// Byte stored by `00454440`.
pub(crate) const FLAG_011F9427: u32 = 0x011f_9427;
/// Byte `00454450` sets to 1 (`0118 9625`).
pub(crate) const FLAG_01189625: u32 = 0x0118_9625;
/// Byte `UpdateCurrentGridCell` sets while the grid centre has moved and
/// `00454450` clears; read with [`GRID_FLAG_MOVED_COPY`].
pub(crate) const GRID_FLAG_MOVED: u32 = 0x011c_3c0c;
/// Copy of [`GRID_FLAG_MOVED`] taken right after it is set or cleared.
pub(crate) const GRID_FLAG_MOVED_COPY: u32 = 0x011f_94aa;
/// Byte set by `00454450` from `00454b70` (called on the terrain manager).
pub(crate) const GRID_FLAG_TERRAIN: u32 = 0x011f_94ab;
/// Float stored by `UpdateCurrentGridCell` and `00454450` from `00452e70(0)`.
pub(crate) const GRID_FLOAT_011F94AC: u32 = 0x011f_94ac;
/// Two floats `00452e70` indexes with its argument.
pub(crate) const FLOAT_PAIR_011F940C: u32 = 0x011f_940c;
/// The world position (two floats) of the centre of the current grid cell.
pub(crate) const GRID_CENTRE: u32 = 0x011f_962c;
/// The previous value of [`GRID_CENTRE`] (two floats).
pub(crate) const GRID_PREVIOUS_CENTRE: u32 = 0x011f_9634;
/// A pair of floats `00452df0` compares [`GRID_PREVIOUS_CENTRE`] with.
pub(crate) const GRID_CENTRE_REFERENCE: u32 = 0x011f_4980;
/// Float time `00453550` adds its argument to.
pub(crate) const TIME_ACCUMULATOR: u32 = 0x011c_3c08;
/// A `double` 0.0 (`UpdateCurrentGridCell` compares the cell deltas with it).
pub(crate) const DOUBLE_ZERO: u32 = 0x0101_2060;
/// A `double` 0.5.
pub(crate) const DOUBLE_HALF: u32 = 0x0101_1588;
/// A `double` 4096.0, the edge of an exterior cell in world units.
pub(crate) const DOUBLE_CELL_SIZE: u32 = 0x0101_7a10;
/// A `float` 4096.0 and a `float` 3072.0: the extent `UpdateCurrentGridCell`
/// measures the cell deltas from (3072.0 when `uGridsToLoad` is 3).
pub(crate) const FLOAT_CELL_EXTENT: u32 = 0x0101_7a3c;
pub(crate) const FLOAT_CELL_EXTENT_SMALL_GRID: u32 = 0x0101_7a38;
/// `"Loading Area (queued cells)..."` and `"Loading Area (unloaded
/// cells)..."`.
pub(crate) const LOADING_AREA_QUEUED_CELLS: u32 = 0x0101_7a18;
pub(crate) const LOADING_AREA_UNLOADED_CELLS: u32 = 0x0101_79e8;

/// `abs` (the CRT's, `int` in, `int` out).
pub(crate) const ABS: u32 = 0x00ec_7d40;
/// `TESWorldSpace::GetCellFromCellCoord` (Xbox PDB): `(worldSpace, x, y)`.
pub(crate) const WORLD_SPACE_GET_CELL_FROM_CELL_COORD: u32 = 0x0058_75a0;
/// The data handler's lookup of the cell at `(x, y, worldSpace, 0)`
/// (`tesdatahandler.cpp`; a null world space means the current one).
pub(crate) const DATA_HANDLER_GET_CELL: u32 = 0x0046_1c20;
/// `TESDataHandler::UnloadCell` (Xbox PDB).
pub(crate) const DATA_HANDLER_UNLOAD_CELL: u32 = 0x0046_2290;
/// `ExteriorCellLoader::QueueCellLoad` (Xbox PDB): `(worldSpace, x, y)`.
pub(crate) const EXTERIOR_CELL_LOADER_QUEUE_CELL_LOAD: u32 = 0x0052_83c0;
/// `TES::AddToBuffer` (Xbox PDB): `(cell)`.
pub(crate) const TES_ADD_TO_BUFFER: u32 = 0x0045_4b90;
/// Returns `this + 0x8c`, the address of `listLastLoadedExteriors`.
pub(crate) const GET_LAST_LOADED_EXTERIORS: u32 = 0x0045_bb80;
/// `TES::SetWorldSpace` (the decompiler's name for `00458200`): `(world
/// space)`.
pub(crate) const SET_WORLD_SPACE: u32 = 0x0045_8200;
/// `(text)`: the call both "Loading Area (...)" messages are passed to.
pub(crate) const SET_LOADING_AREA_TEXT: u32 = 0x0045_81e0;
/// `TESObjectCELL::Load3D` (Xbox PDB).
pub(crate) const CELL_LOAD_3D: u32 = 0x0054_5cf0;
/// `NiAVObject::UpdateProperties` (Xbox PDB).
pub(crate) const NODE_UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// `TESObjectCELL::AttachLights` (Xbox PDB): `(cell, flag)`.
pub(crate) const CELL_ATTACH_LIGHTS: u32 = 0x0054_ba80;
/// `TESObjectCELL::Detach` (Xbox PDB): `(cell, flag)`.
pub(crate) const CELL_DETACH: u32 = 0x0055_2bd0;
/// `BGSTerrainManager::Update` (Xbox PDB): `(terrain, position, flags)`.
pub(crate) const TERRAIN_MANAGER_UPDATE: u32 = 0x006f_ca90;
/// Puts the item at the address passed at the head of a list: `(list,
/// &item)`. The list `00452ff0` fills is `listLastLoadedExteriors`.
pub(crate) const LIST_ADD_HEAD: u32 = 0x005a_e3d0;
/// The `NiPointer` getter (first word of `this`).
pub(crate) const NI_POINTER_GET: u32 = 0x0055_9450;
/// `NiPointer<NiSourceTexture>` constructor `(this, 0)` and destructor.
pub(crate) const NI_POINTER_CTOR: u32 = 0x0063_3c90;
pub(crate) const NI_POINTER_DTOR: u32 = 0x0045_cec0;
/// `TES::CreateTextureImage` (Xbox PDB): `(this, name, &texture, 1, 0)`.
pub(crate) const TES_CREATE_TEXTURE_IMAGE: u32 = 0x0045_68c0;
/// Returns `this + 0x64` (`TES::pWaterSystem`; the map names it
/// `DetailedActorPathHandler::GetCurrentNodeIndex`, identical code).
pub(crate) const TES_WATER_SYSTEM_GETTER: u32 = 0x0070_ec90;
/// `(waterSystem, texture)`: stores the texture through `0066b0d0` on
/// `this + 0x1c`.
pub(crate) const WATER_SYSTEM_SET_TEXTURE: u32 = 0x0052_d3b0;
/// `TESObjectCELL::LoadAllTempData` (Xbox PDB).
pub(crate) const CELL_LOAD_ALL_TEMP_DATA: u32 = 0x0055_0340;
/// `(modelLoader, cell)`: byte answer used to start loading a cell's models.
pub(crate) const MODEL_LOADER_CELL_NEEDS_MODELS: u32 = 0x0044_7950;
/// `TESObjectCELL::AttachToWorld` (Xbox PDB): `(cell, root)`.
pub(crate) const CELL_ATTACH_TO_WORLD: u32 = 0x0055_2970;
/// The exterior havok world's counters (`bhkworld.obj`): `00c66300` and
/// `00c66310` add one to the counters at `+0x18` and `+0x1c`; `00c6b540`
/// and `00c68f00` take one off (argument: a flag that re-enables the
/// work the counter held back).
pub(crate) const HAVOK_WORLD_ADD_COUNT_18: u32 = 0x00c6_6300;
pub(crate) const HAVOK_WORLD_ADD_COUNT_1C: u32 = 0x00c6_6310;
pub(crate) const HAVOK_WORLD_DROP_COUNT_18: u32 = 0x00c6_b540;
pub(crate) const HAVOK_WORLD_DROP_COUNT_1C: u32 = 0x00c6_8f00;
/// `bhkWorld::SetVisualDebugger` (Xbox PDB): `(world, flag)`.
pub(crate) const HAVOK_WORLD_SET_VISUAL_DEBUGGER: u32 = 0x00c6_a640;
/// Returns `this + 0x50` (`bstempeffectparticle.cpp`, identical code); the
/// grid move calls it on the havok world and drops the result.
pub(crate) const HAVOK_WORLD_GET_0X50: u32 = 0x0068_a830;
/// `FaderManager::CreateFader` (Xbox PDB): `(manager, 1, seconds, flag)`.
pub(crate) const FADER_CREATE_FADER: u32 = 0x0070_0960;
/// `PlayerCharacter` virtual slots: `0x1f4` returns the position (three
/// floats), `0x1d0` a pointer that is null while no fade is wanted.
pub(crate) const PLAYER_SLOT_POSITION: u32 = 0x1f4;
pub(crate) const PLAYER_SLOT_FADE_OBJECT: u32 = 0x1d0;

/// The `ExteriorCellLoader` singleton.
pub(crate) fn exterior_cell_loader(e: &mut Engine) -> u32 {
    e.global(EXTERIOR_CELL_LOADER)
}

/// The `abs` check `UpdateCurrentGridCell` and `00454450` make: the grid
/// centre moved by more than one cell in x (first) or y.
pub(crate) fn grid_moved_far(e: &mut Engine, delta_x: i32, delta_y: i32) -> bool {
    e.call(ABS, &args![delta_x]).i32() > 1 || e.call(ABS, &args![delta_y]).i32() > 1
}

/// `(grid + 0.5) * 4096.0` rounded to a `float`: the world coordinate of the
/// centre of grid column or row `grid`.
pub(crate) fn grid_cell_centre(e: &mut Engine, grid: i32) -> f32 {
    let half: f64 = e.global(DOUBLE_HALF);
    let size: f64 = e.global(DOUBLE_CELL_SIZE);
    ((grid as f64 + half) * size) as f32
}

/// Stores the world position of the centre of the current grid cell in
/// [`GRID_CENTRE`] (through `00452dc0`).
pub(crate) fn store_grid_centre(e: &mut Engine, this: Ptr<TES>) {
    let grid_y = e.get(this, TES::iCurrentGridY);
    let centre_y = grid_cell_centre(e, grid_y);
    let grid_x = e.get(this, TES::iCurrentGridX);
    let centre_x = grid_cell_centre(e, grid_x);
    let words = e.with_stack(8, |e, out| {
        let result = fn_00452dc0(e, out, centre_x, centre_y);
        [e.mem.u32(result.addr()), e.mem.u32(result.addr() + 4)]
    });
    e.set_global(GRID_CENTRE, words[0]);
    e.set_global(GRID_CENTRE + 4, words[1]);
}

/// `previous centre = centre` (two words).
pub(crate) fn copy_grid_centre_to_previous(e: &mut Engine) {
    let first: u32 = e.global(GRID_CENTRE);
    e.set_global(GRID_PREVIOUS_CENTRE, first);
    let second: u32 = e.global(GRID_CENTRE + 4);
    e.set_global(GRID_PREVIOUS_CENTRE + 4, second);
}

/// The tail both `00453dc0` and `00454450` end with: the water system is
/// given the image of the location name (`WATER_SYSTEM_SET_TEXTURE`).
/// `named` is whether the name starts with a character (the callers test it
/// with their own read of the name); `name_of` reads the name (called again
/// inside the scope, as the code does). Named: under a memory tag `0x1d`
/// guard, `TES::CreateTextureImage` builds the image into a
/// `NiPointer<NiSourceTexture>` temporary and its pointer is handed over;
/// otherwise a null image is.
///
/// Not translated: the compiler's exception-unwinding frame.
pub(crate) fn apply_location_image(
    e: &mut Engine,
    line: u32,
    named: bool,
    name_of: impl Fn(&mut Engine) -> u32,
) {
    let tes_singleton: u32 = e.global(TES_SINGLETON);
    if !named {
        let water_system = e.call(TES_WATER_SYSTEM_GETTER, &args![tes_singleton]).u32();
        e.call(WATER_SYSTEM_SET_TEXTURE, &args![water_system, 0u32]);
        return;
    }
    e.with_stack(SCOPE_GUARD_SIZE, |e, guard| {
        e.call(
            SCOPE_GUARD_CTOR,
            &args![guard, 0x1du32, 1u32, TES_CPP_PATH, line],
        );
        e.with_stack(4, |e, texture| {
            e.call(NI_POINTER_CTOR, &args![texture, 0u32]);
            let name = name_of(e);
            e.call(
                TES_CREATE_TEXTURE_IMAGE,
                &args![tes_singleton, name, texture, 1u32, 0u32],
            );
            let image = e.call(NI_POINTER_GET, &args![texture]).u32();
            let water_system = e.call(TES_WATER_SYSTEM_GETTER, &args![tes_singleton]).u32();
            e.call(WATER_SYSTEM_SET_TEXTURE, &args![water_system, image]);
            e.call(NI_POINTER_DTOR, &args![texture]);
        });
        e.call(SCOPE_GUARD_DTOR, &args![guard]);
    });
}

// Translated from 00452440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `0x40000000` of the flags word of `form` is set
/// (`00452460` with that mask). `00451ef0` asks it of a type 0x23 form.
pub fn fn_00452440(e: &mut Engine, form: Ptr) -> bool {
    fn_00452460(e, form, 0x4000_0000)
}

// Translated from 00452460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether any bit of `mask` is set in the flags word `00624700` finds in
/// `form` (it returns the address `form + 0x48`).
pub fn fn_00452460(e: &mut Engine, form: Ptr, mask: u32) -> bool {
    let flags = e.call(0x0062_4700, &args![form]).u32();
    e.mem.u32(flags) & mask != 0
}

// Translated from 00452480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte at `0119b8c8`.
pub fn fn_00452480(e: &mut Engine) -> u8 {
    e.global(FLAG_0119B8C8)
}

// Translated from 00452490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::CleanUpUnusedTextures` (Xbox PDB): when purging is allowed
/// (`00452540`), takes the counted lock (`00452510`), flushes (`00664cd0(1)`),
/// purges the unused textures (`BSTexturePalette::PurgeUnusedTextures`,
/// `00a61cd0`) when `bAllowUnusedPurge` (`004524f0`) or `force` is set,
/// flushes again and releases the lock (`00452530`).
pub fn tes_clean_up_unused_textures(e: &mut Engine, this: Ptr<TES>, force: u8) {
    if fn_00452540(e, this) == 0 {
        return;
    }
    fn_00452510(e);
    e.call(0x0066_4cd0, &args![1u32]);
    if fn_004524f0(e, this) != 0 || force != 0 {
        e.call(0x00a6_1cd0, &args![]);
    }
    e.call(0x0066_4cd0, &args![1u32]);
    fn_00452530(e);
}

// Translated from 004524f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::bAllowUnusedPurge` (Xbox PDB `+0xB5`): returns the byte as stored
/// (`MOV AL,[EAX+0xb5]`), not normalised to 0/1 (#198).
pub fn fn_004524f0(e: &mut Engine, this: Ptr<TES>) -> u8 {
    e.mem.u8(this.addr() + TES::bAllowUnusedPurge.off)
}

// Translated from 00452510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `0040fbf0(0)` on the object at `011f4480` (takes the counted lock).
pub fn fn_00452510(e: &mut Engine) {
    e.call(0x0040_fbf0, &args![CLEANUP_LOCK, 0u32]);
}

// Translated from 00452530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `0040fba0` on the object at `011f4480` (releases the counted lock).
pub fn fn_00452530(e: &mut Engine) {
    e.call(0x0040_fba0, &args![CLEANUP_LOCK]);
}

// Translated from 00452540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether textures may be purged: true unless the byte at `011c70e9` is set
/// and `005d4a40` says no. `this` is only the ECX the caller left; the code
/// never reads it, and `005d4a40` takes no word (`MOV AL,1; RET`).
pub fn fn_00452540(e: &mut Engine, _this: Ptr<TES>) -> u8 {
    // 0045254f: CALL 005d4a40 with nothing pushed.
    if e.global::<u8>(PURGE_ALLOWED) != 0 && !e.call(0x005d_4a40, &args![]).bool() {
        return 0;
    }
    1
}

// Translated from 00452580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::UpdateCurrentGridCell` (Xbox PDB): follows the player (or camera)
/// position `pos` (two floats, the x and y world coordinates) across the
/// exterior grid. `flag` is the "settle" flag the callers pass (`0` from the
/// cell switches, true when the save/load object does not answer yes).
///
/// First it clears the last-loaded list, refreshes the havok bound and the
/// moved flags. With no data handler it does nothing. When the grid centre
/// is not set yet (`0x7fffffff`) it takes it from `pos`, loads the area
/// (`00457d70(1, worldSpace, 0)`, `004515a0`) and returns true.
///
/// Otherwise the offset of `pos` from the centre of the current cell is
/// stored in `fCell_delta_x`/`y` as the distance left to the cell border
/// (`4096.0` or `3072.0` for three grids). While the position is still well
/// inside the cell (both distances positive, no script running) it only
/// queues the cell loads of the column and row that come into range when the
/// background loading settings allow, and returns false. Once the position
/// crosses into another cell the grid is moved: the new centre and the world
/// centre globals are stored, the pending loads cancelled and (without an
/// interior cell loaded, or while a script runs) the cells are loaded,
/// unloaded and attached again with the havok world locked, then the terrain
/// manager is updated at the player and true is returned.
pub fn tes_update_current_grid_cell(e: &mut Engine, this: Ptr<TES>, pos: Ptr, flag: u8) -> bool {
    let base = this.addr();
    let list = e.call(GET_LAST_LOADED_EXTERIORS, &args![this]).u32();
    e.call(0x0047_0470, &args![list]);
    e.call(0x0045_c840, &args![this]);
    if e.global::<u8>(GRID_FLAG_MOVED) != 0
        && e.global::<u8>(GRID_FLAG_MOVED_COPY) == 0
        && e.call(GET_INTERIOR_CELL, &args![this]).u32() == 0
    {
        e.set_global(GRID_FLAG_MOVED, 0u8);
        e.call(0x0045_c780, &args![this]);
    }
    let script_context: u32 = e.global(SCRIPT_CONTEXT);
    if e.call(0x0042_ce10, &args![script_context]).bool() {
        let bound = e.call(NI_POINTER_GET, &args![base + 0xc0]).u32();
        e.call(0x0062_8da0, &args![bound]);
    }
    let grid_cells = e.get(this, TES::pGridCellA);
    let cell = e.call(0x0045_7070, &args![this]).u32();
    let bound = e.call(NI_POINTER_GET, &args![base + 0xc0]).u32();
    e.call(0x0062_71b0, &args![bound, cell, grid_cells]);
    if e.global::<u32>(DATA_HANDLER) == 0 {
        return false;
    }
    if e.get(this, TES::iCurrentGridX) == 0x7fff_ffff
        || e.get(this, TES::iCurrentGridY) == 0x7fff_ffff
    {
        if !pos.is_null() {
            let x_value = e.mem.f32(pos.addr());
            let cell_x = e.call(FLOAT_TO_INT_ROUNDED, &args![x_value]).i32() >> 12;
            e.set(this, TES::iCurrentGridX, cell_x);
            let y_value = e.mem.f32(pos.addr() + 4);
            let cell_y = e.call(FLOAT_TO_INT_ROUNDED, &args![y_value]).i32() >> 12;
            e.set(this, TES::iCurrentGridY, cell_y);
            let grid_x = e.get(this, TES::iCurrentGridX);
            e.set(this, TES::iCurrentQueuedX, grid_x);
            let grid_y = e.get(this, TES::iCurrentGridY);
            e.set(this, TES::iCurrentQueuedY, grid_y);
        }
        let world_space = e.get(this, TES::pWorldSpace);
        e.call(0x0045_7d70, &args![this, 1u32, world_space, 0u32]);
        fn_004515a0(e, this, pos, 0);
        return true;
    }
    let x_value = e.mem.f32(pos.addr());
    let position_x = e.call(FLOAT_TO_INT_ROUNDED, &args![x_value]).i32();
    let y_value = e.mem.f32(pos.addr() + 4);
    let position_y = e.call(FLOAT_TO_INT_ROUNDED, &args![y_value]).i32();
    let grid_x = e.get(this, TES::iCurrentGridX);
    let offset_x = position_x.wrapping_sub(grid_x.wrapping_shl(12).wrapping_add(0x800));
    let grid_y = e.get(this, TES::iCurrentGridY);
    let offset_y = position_y.wrapping_sub(grid_y.wrapping_shl(12).wrapping_add(0x800));
    let mut extent: f32 = e.global(FLOAT_CELL_EXTENT);
    if setting_uint(e, SETTING_GRIDS_TO_LOAD) == 3 {
        extent = e.global(FLOAT_CELL_EXTENT_SMALL_GRID);
    }
    let distance_x = e.call(ABS, &args![offset_x]).i32();
    e.set(
        this,
        TES::fCell_delta_x,
        (extent as f64 - distance_x as f64) as f32,
    );
    let distance_y = e.call(ABS, &args![offset_y]).i32();
    e.set(
        this,
        TES::fCell_delta_y,
        (extent as f64 - distance_y as f64) as f32,
    );
    let script_running = e.call(0x0042_ce10, &args![script_context]).bool();
    let zero: f64 = e.global(DOUBLE_ZERO);
    if e.get(this, TES::fCell_delta_x) as f64 > zero
        && e.get(this, TES::fCell_delta_y) as f64 > zero
        && !script_running
    {
        if setting_byte(e, SETTING_BACKGROUND_CELL_LOADS) != 0
            && e.call(GET_INTERIOR_CELL, &args![this]).u32() == 0
        {
            let queued_x = position_x >> 12;
            let queued_y = position_y >> 12;
            if e.get(this, TES::iCurrentQueuedX) != queued_x
                || e.get(this, TES::iCurrentQueuedY) != queued_y
            {
                if setting_byte(e, SETTING_PREEMPTIVELY_UNLOAD_CELLS) != 0
                    && e.call(0x0045_4d50, &args![this, 1u32]).u32() != 0
                {
                    tes_clean_up_unused_textures(e, this, 0);
                }
                let half = setting_uint(e, SETTING_GRIDS_TO_LOAD) >> 1;
                let reach = half as i32;
                let mut column = e.get(this, TES::iCurrentQueuedX);
                let moved_x = queued_x.wrapping_sub(e.get(this, TES::iCurrentQueuedX));
                if moved_x != 0 {
                    column = ((half.wrapping_add(1)) as i32)
                        .wrapping_mul(moved_x)
                        .wrapping_add(column);
                    let mut offset = reach.wrapping_neg();
                    while offset <= reach {
                        let row = offset.wrapping_add(e.get(this, TES::iCurrentQueuedY));
                        let world_space = e.call(GET_WORLD_SPACE, &args![this]).u32();
                        let loader = exterior_cell_loader(e);
                        e.call(
                            EXTERIOR_CELL_LOADER_QUEUE_CELL_LOAD,
                            &args![loader, world_space, column, row],
                        );
                        offset += 1;
                    }
                }
                let mut row = e.get(this, TES::iCurrentQueuedY);
                let moved_y = queued_y.wrapping_sub(e.get(this, TES::iCurrentQueuedY));
                if moved_y != 0 {
                    row = ((half.wrapping_add(1)) as i32)
                        .wrapping_mul(moved_y)
                        .wrapping_add(row);
                    let mut offset = reach.wrapping_neg();
                    while offset <= reach {
                        let column_here = offset.wrapping_add(e.get(this, TES::iCurrentQueuedX));
                        let world_space = e.call(GET_WORLD_SPACE, &args![this]).u32();
                        let loader = exterior_cell_loader(e);
                        e.call(
                            EXTERIOR_CELL_LOADER_QUEUE_CELL_LOAD,
                            &args![loader, world_space, column_here, row],
                        );
                        offset += 1;
                    }
                }
                if moved_x != 0 && moved_y != 0 {
                    let world_space = e.call(GET_WORLD_SPACE, &args![this]).u32();
                    let loader = exterior_cell_loader(e);
                    e.call(
                        EXTERIOR_CELL_LOADER_QUEUE_CELL_LOAD,
                        &args![loader, world_space, column, row],
                    );
                }
            }
            e.set(this, TES::iCurrentQueuedX, queued_x);
            e.set(this, TES::iCurrentQueuedY, queued_y);
        }
        return false;
    }

    // The position left the cell: move the grid.
    fn_00452e40(e, 0);
    let mut loading_menu = e.call(0x0070_5e80, &args![]).u8();
    let loader = exterior_cell_loader(e);
    if loading_menu == 0
        && e.call(0x0052_85a0, &args![loader]).u32() != 0
        && !e.call(0x0042_ce10, &args![script_context]).bool()
    {
        e.call(
            SET_LOADING_AREA_TEXT,
            &args![this, LOADING_AREA_QUEUED_CELLS],
        );
        loading_menu = 1;
    }
    let mut new_x = position_x >> 12;
    let mut new_y = position_y >> 12;
    if e.call(0x0042_ce10, &args![script_context]).bool()
        && fn_00452e90(e, Ptr::new(script_context))
    {
        new_x = e.get(this, TES::iCurrentGridX);
        new_y = e.get(this, TES::iCurrentGridY);
    }
    let moved_x = new_x.wrapping_sub(e.get(this, TES::iCurrentGridX));
    let moved_y = new_y.wrapping_sub(e.get(this, TES::iCurrentGridY));
    e.set(this, TES::iCurrentGridX, new_x);
    e.set(this, TES::iCurrentGridY, new_y);
    let grid_x = e.get(this, TES::iCurrentGridX);
    e.set(this, TES::iCurrentQueuedX, grid_x);
    let grid_y = e.get(this, TES::iCurrentGridY);
    e.set(this, TES::iCurrentQueuedY, grid_y);
    copy_grid_centre_to_previous(e);
    store_grid_centre(e, this);
    if fn_00452df0(
        e,
        Ptr::new(GRID_PREVIOUS_CENTRE),
        Ptr::new(GRID_CENTRE_REFERENCE),
    ) || grid_moved_far(e, moved_x, moved_y)
    {
        copy_grid_centre_to_previous(e);
    }
    let wind = fn_00452e70(e, 0);
    e.set_global(GRID_FLOAT_011F94AC, wind);
    e.set_global(GRID_FLAG_MOVED, 1u8);
    let moved: u8 = e.global(GRID_FLAG_MOVED);
    e.set_global(GRID_FLAG_MOVED_COPY, moved);
    e.call(0x0045_c840, &args![this]);
    e.call(0x0045_c780, &args![this]);
    let loader = exterior_cell_loader(e);
    e.call(0x0052_8110, &args![loader]);
    let loader = exterior_cell_loader(e);
    if e.call(0x0052_81f0, &args![loader]).u32() != 0 {
        let loader = exterior_cell_loader(e);
        e.call(0x0052_82d0, &args![loader]);
    }
    if e.call(GET_INTERIOR_CELL, &args![this]).u32() == 0 || script_running {
        if setting_byte(e, SETTING_PREEMPTIVELY_UNLOAD_CELLS) != 0
            && e.call(0x0045_4d50, &args![this, 1u32]).u32() != 0
        {
            tes_clean_up_unused_textures(e, this, 0);
        }
        if e.call(GET_WORLD_SPACE, &args![this]).u32() == 0 {
            let data_handler: u32 = e.global(DATA_HANDLER);
            let holder = e.call(0x0046_0140, &args![data_handler]).u32();
            let slot = e.call(0x0068_15c0, &args![holder]).u32();
            let world_space = e.mem.u32(slot);
            e.call(SET_WORLD_SPACE, &args![this, world_space]);
        }
        let far = grid_moved_far(e, moved_x, moved_y);
        if fn_00451010(e) != 0 {
            if far || flag == 0 {
                let world = fn_00451010(e);
                e.call(HAVOK_WORLD_DROP_COUNT_18, &args![world, 0u32]);
                let world = fn_00451010(e);
                e.call(HAVOK_WORLD_DROP_COUNT_1C, &args![world, 0u32]);
            }
            let world = fn_00451010(e);
            e.call(HAVOK_WORLD_ADD_COUNT_1C, &args![world]);
        }
        let grid_cells = e.get(this, TES::pGridCellA);
        e.vcall(grid_cells.addr(), 0x10, &args![new_x, new_y]);
        if fn_00451010(e) != 0 {
            let loader = exterior_cell_loader(e);
            if loading_menu == 0 && (!far || e.call(0x0052_85a0, &args![loader]).u32() == 0) {
                let world = fn_00451010(e);
                e.call(HAVOK_WORLD_GET_0X50, &args![world]);
            }
            if far {
                let world = fn_00451010(e);
                e.call(HAVOK_WORLD_DROP_COUNT_1C, &args![world, 0u32]);
            } else {
                let world = fn_00451010(e);
                e.call(HAVOK_WORLD_DROP_COUNT_1C, &args![world, flag]);
            }
        }
        if loading_menu == 0
            && !tes_all_cells_in_grid_loaded(e, this)
            && !e.call(0x0042_ce10, &args![script_context]).bool()
        {
            e.call(
                SET_LOADING_AREA_TEXT,
                &args![this, LOADING_AREA_UNLOADED_CELLS],
            );
        }
        if grid_moved_far(e, moved_x, moved_y) {
            e.call(0x0045_7be0, &args![this, 0u32]);
        }
        if fn_00451010(e) != 0 {
            let world = fn_00451010(e);
            e.call(HAVOK_WORLD_ADD_COUNT_18, &args![world]);
        }
        tes_grid_array_load(e, this);
        fn_00452e50(e, this);
        if fn_00451010(e) != 0 {
            if far {
                let world = fn_00451010(e);
                e.call(HAVOK_WORLD_DROP_COUNT_18, &args![world, 0u32]);
            } else {
                let world = fn_00451010(e);
                e.call(HAVOK_WORLD_DROP_COUNT_18, &args![world, flag]);
            }
        }
    }
    tes_clean_up_unused_textures(e, this, 0);
    e.call(0x0062_f460, &args![]);
    if e.call(GET_INTERIOR_CELL, &args![this]).u32() == 0 {
        let player: u32 = e.global(PLAYER_CHARACTER);
        // `PlayerCharacter` slot 0x1f4 returns the position (three floats).
        let player_position = e.vcall(player, PLAYER_SLOT_POSITION, &args![]).u32();
        let world_space = e.get(this, TES::pWorldSpace);
        let terrain = e.call(GET_TERRAIN_MANAGER, &args![world_space]).u32();
        e.call(
            TERRAIN_MANAGER_UPDATE,
            &args![terrain, player_position, 0xfu32],
        );
    }
    fn_00452e40(e, 1);
    true
}

// Translated from 00452dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the two floats `x`, `y` in `this` and returns `this` (a
/// two-float point's constructor).
pub fn fn_00452dc0(e: &mut Engine, this: Ptr, x: f32, y: f32) -> Ptr {
    e.mem.set_f32(this.addr(), x87_quiet(x));
    e.mem.set_f32(this.addr() + 4, x87_quiet(y));
    this
}

// Translated from 00452df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the two floats at `this` equal those at `other` (a NaN never
/// equals).
pub fn fn_00452df0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.mem.f32(this.addr()) == e.mem.f32(other.addr())
        && e.mem.f32(this.addr() + 4) == e.mem.f32(other.addr() + 4)
}

// Translated from 00452e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte in `011af70c`.
pub fn fn_00452e40(e: &mut Engine, value: u8) {
    e.set_global(FLAG_011AF70C, value);
}

// Translated from 00452e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `TES::bUpdateGridString` (Xbox PDB `+0x7D`).
pub fn fn_00452e50(e: &mut Engine, this: Ptr<TES>) {
    e.set(this, TES::bUpdateGridString, true);
}

// Translated from 00452e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at `011f940c + 4 * (selector != 0)`, returned in `ST0`.
pub fn fn_00452e70(e: &mut Engine, selector: u8) -> f32 {
    let index = (selector != 0) as u32;
    e.global(FLOAT_PAIR_011F940C + 4 * index)
}

// Translated from 00452e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit `0x20` of the flags word at `+0x244` of `this` (the script context
/// object at `011ddf38`; `0042ce10` reads its own bit of the same word).
pub fn fn_00452e90(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x244) & 0x20 != 0
}

// Translated from 00452eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::CancelExteriorCellLoads` (Xbox PDB): calls
/// `ExteriorCellLoader::CancelAllCellLoads` (`00528540`) on the loader.
pub fn tes_cancel_exterior_cell_loads(e: &mut Engine, _this: Ptr<TES>) {
    let loader = exterior_cell_loader(e);
    e.call(0x0052_8540, &args![loader]);
}

// Translated from 00452ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::AllCellsInGridLoaded` (Xbox PDB): false without a world space (the
/// one of the `TES` singleton); otherwise whether, for every slot of the
/// `uGridsToLoad` square around the current grid cell that holds a cell (or
/// whose cell `TESWorldSpace::GetCellFromCellCoord`, `005875a0`, finds),
/// `00454e20` says the cell is loaded. A slot that holds nothing is skipped; a
/// missing cell or one that is not loaded makes the answer false.
pub fn tes_all_cells_in_grid_loaded(e: &mut Engine, this: Ptr<TES>) -> bool {
    let tes_singleton: u32 = e.global(TES_SINGLETON);
    if e.call(GET_WORLD_SPACE, &args![tes_singleton]).u32() == 0 {
        return false;
    }
    let half = setting_uint(e, SETTING_GRIDS_TO_LOAD) >> 1;
    let origin_x = e.get(this, TES::iCurrentGridX).wrapping_sub(half as i32);
    let half = setting_uint(e, SETTING_GRIDS_TO_LOAD) >> 1;
    let origin_y = e.get(this, TES::iCurrentGridY).wrapping_sub(half as i32);
    let mut x = 0u32;
    while x < grids_to_load(e) {
        let mut y = 0u32;
        while y < grids_to_load(e) {
            let grid_cells = e.get(this, TES::pGridCellA);
            let slot = e.call(GRID_CELL_ARRAY_GET, &args![grid_cells, x, y]).u32();
            if slot != 0 {
                let mut cell = e.mem.u32(slot);
                if cell == 0 {
                    let tes_singleton: u32 = e.global(TES_SINGLETON);
                    let world_space = e.call(GET_WORLD_SPACE, &args![tes_singleton]).u32();
                    cell = e
                        .call(
                            WORLD_SPACE_GET_CELL_FROM_CELL_COORD,
                            &args![
                                world_space,
                                origin_x.wrapping_add(x as i32),
                                origin_y.wrapping_add(y as i32)
                            ],
                        )
                        .u32();
                }
                if cell == 0 || !e.call(0x0045_4e20, &args![this, cell]).bool() {
                    return false;
                }
            }
            y += 1;
        }
        x += 1;
    }
    true
}

// Translated from 00452ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::GridArrayLoad` (Xbox PDB): loads every cell of the `uGridsToLoad`
/// square around the current grid cell, then attaches them.
///
/// Under a memory tag `0x1a` guard: for every grid slot `TES::LoadGridCell`
/// is run and the cell found at the grid position is entered in the
/// exterior buffer (`00454b90`) when `00453490` says so; a load-screen
/// counter advances when the loading menu is up or the save/load object says
/// yes twice (the counter and the percentage it computes live in locals the
/// code never reads again, so only the setting reads and the screen refresh
/// `00861ea0` remain). Then every cell of the square that exists is put on
/// the last-loaded list, and those in load state 5 are first attached
/// (`0054bcf0`), buffered and have their 3D properties updated. The havok
/// world's `+0x18` counter is raised around the loading and the work ends
/// with `00451570(0)` and `00451520(1)` (a byte at `01189184`).
///
/// Not translated: the compiler's exception-unwinding frame.
pub fn tes_grid_array_load(e: &mut Engine, this: Ptr<TES>) {
    e.with_stack(SCOPE_GUARD_SIZE, |e, guard| {
        e.call(
            SCOPE_GUARD_CTOR,
            &args![guard, 0x1au32, 1u32, TES_CPP_PATH, 0x8d6u32],
        );
        let half = setting_uint(e, SETTING_GRIDS_TO_LOAD) >> 1;
        let origin_x = e.get(this, TES::iCurrentGridX).wrapping_sub(half as i32);
        let half = setting_uint(e, SETTING_GRIDS_TO_LOAD) >> 1;
        let origin_y = e.get(this, TES::iCurrentGridY).wrapping_sub(half as i32);
        let mut loading_menu_visible = 0u8;
        if e.call(0x0070_5e80, &args![]).bool() {
            loading_menu_visible = 1;
        }
        let loading_menu_raw = e.call(0x0070_5e80, &args![]).u8();
        let save_load: u32 = e.global(SAVE_LOAD_GAME);
        let script_context: u32 = e.global(SCRIPT_CONTEXT);
        fn_004534f0(e, save_load, 1);
        fn_00453500(e, Ptr::new(script_context), 1);
        if fn_00451010(e) != 0 {
            let world = fn_00451010(e);
            e.call(HAVOK_WORLD_ADD_COUNT_18, &args![world]);
        }
        let mut x = 0u32;
        while x < grids_to_load(e) {
            let mut y = 0u32;
            while y < grids_to_load(e) {
                tes_load_grid_cell(
                    e,
                    this,
                    x,
                    y,
                    origin_x.wrapping_add(x as i32),
                    origin_y.wrapping_add(y as i32),
                );
                let world_space = e.call(GET_WORLD_SPACE, &args![this]).u32();
                let data_handler: u32 = e.global(DATA_HANDLER);
                let cell = e
                    .call(
                        DATA_HANDLER_GET_CELL,
                        &args![
                            data_handler,
                            origin_x.wrapping_add(x as i32),
                            origin_y.wrapping_add(y as i32),
                            world_space,
                            0u32
                        ],
                    )
                    .u32();
                let tes_singleton: u32 = e.global(TES_SINGLETON);
                if fn_00453490(e, Ptr::new(tes_singleton), Ptr::new(cell)) {
                    e.call(TES_ADD_TO_BUFFER, &args![this, cell]);
                }
                let save_load: u32 = e.global(SAVE_LOAD_GAME);
                let counts = loading_menu_raw != 0
                    || (e.call(0x0047_c850, &args![save_load]).bool()
                        && e.call(0x0047_c850, &args![save_load]).bool());
                if counts {
                    // The percentage (loaded / grids^2 * 100) goes to a local
                    // the code never reads; the two setting reads remain.
                    let first = e
                        .call(SETTING_VALUE_ADDRESS_INT, &args![SETTING_GRIDS_TO_LOAD])
                        .u32();
                    let second = e
                        .call(SETTING_VALUE_ADDRESS_INT, &args![SETTING_GRIDS_TO_LOAD])
                        .u32();
                    let _cells = e.mem.u32(first).wrapping_mul(e.mem.u32(second));
                    let save_load: u32 = e.global(SAVE_LOAD_GAME);
                    if e.call(0x0047_c850, &args![save_load]).bool()
                        && e.call(0x0047_c850, &args![save_load]).bool()
                    {
                        e.call(0x0086_1ea0, &args![save_load]);
                    }
                }
                y += 1;
            }
            x += 1;
        }
        if fn_00451010(e) != 0 {
            let world = fn_00451010(e);
            e.call(HAVOK_WORLD_DROP_COUNT_18, &args![world, 0u32]);
        }
        let save_load: u32 = e.global(SAVE_LOAD_GAME);
        fn_004534f0(e, save_load, 0);
        if e.call(0x0087_27b0, &args![save_load]).bool() {
            let first = e.call(0x0047_c850, &args![save_load]).u8();
            let second = e.call(0x0047_c850, &args![save_load]).u8();
            fn_004534f0(e, save_load, 1);
            fn_004534f0(e, save_load, 1);
            e.call(0x0085_8af0, &args![save_load, 0u32, 0u32, 0u32]);
            e.call(0x0085_f850, &args![save_load, 0u32]);
            fn_004534f0(e, save_load, second as u32);
            fn_004534f0(e, save_load, first as u32);
        }
        fn_00453500(e, Ptr::new(script_context), 0);
        if !e.call(0x0042_ce10, &args![script_context]).bool() {
            e.call(0x0084_92b0, &args![script_context, 0u32]);
        }
        if fn_00451010(e) != 0 {
            let world = fn_00451010(e);
            e.call(HAVOK_WORLD_ADD_COUNT_18, &args![world]);
        }
        let mut x = 0u32;
        while x < grids_to_load(e) {
            let mut y = 0u32;
            while y < grids_to_load(e) {
                if e.call(GET_WORLD_SPACE, &args![this]).u32() != 0 {
                    let world_space = e.call(GET_WORLD_SPACE, &args![this]).u32();
                    let data_handler: u32 = e.global(DATA_HANDLER);
                    let cell = e
                        .call(
                            DATA_HANDLER_GET_CELL,
                            &args![
                                data_handler,
                                origin_x.wrapping_add(x as i32),
                                origin_y.wrapping_add(y as i32),
                                world_space,
                                0u32
                            ],
                        )
                        .u32();
                    if cell != 0 {
                        if fn_00450fd0(e, Ptr::new(cell)) == 5 {
                            e.call(
                                0x0054_bcf0,
                                &args![cell, (loading_menu_visible == 0) as u32],
                            );
                            e.call(TES_ADD_TO_BUFFER, &args![this, cell]);
                            let node = e.call(CELL_LOAD_3D, &args![cell]).u32();
                            e.call(NODE_UPDATE_PROPERTIES, &args![node]);
                            if fn_00453470(e, Ptr::new(node)) == 0 {
                                e.with_stack(12, |e, update| {
                                    e.call(UPDATE_OBJECT_CTOR, &args![update, 0.0f32, 0u8, 0u8]);
                                    e.call(NODE_UPDATE, &args![node, update]);
                                });
                            }
                        }
                        e.with_stack(4, |e, slot| {
                            e.mem.set_u32(slot.addr(), cell);
                            let list = e.call(GET_LAST_LOADED_EXTERIORS, &args![this]).u32();
                            e.call(LIST_ADD_HEAD, &args![list, slot]);
                        });
                    }
                }
                y += 1;
            }
            x += 1;
        }
        if fn_00451010(e) != 0 {
            let world = fn_00451010(e);
            e.call(HAVOK_WORLD_DROP_COUNT_18, &args![world, 0u32]);
        }
        fn_00451570(e, 0);
        fn_00451520(e, 1);
        e.call(SCOPE_GUARD_DTOR, &args![guard]);
    });
}

// Translated from 00453470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `009938b0` on `this + 0x9c` and returns its result.
pub fn fn_00453470(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0099_38b0, &args![this.addr() + 0x9c]).u32()
}

// Translated from 00453490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this` is the `TES` the checks run on (`00454bd0`/`00454e20` take it as
/// `this`). Whether `cell` (null: false) is in the buffer: `00454bd0` when
/// `00425fd0` says the cell is an interior (its bit 0 at +0x24), `00454e20`
/// otherwise.
pub fn fn_00453490(e: &mut Engine, this: Ptr<TES>, cell: Ptr) -> bool {
    if cell.is_null() {
        return false;
    }
    if e.call(0x0042_5fd0, &args![cell]).bool() {
        e.call(0x0045_4bd0, &args![this, cell]).bool()
    } else {
        e.call(0x0045_4e20, &args![this, cell]).bool()
    }
}

// Translated from 004534f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Does nothing (`RET 4`); `this` is the `TESSaveLoadGame` object, the word
/// a flag the callers pass.
pub fn fn_004534f0(_e: &mut Engine, _this: u32, _value: u32) {}

// Translated from 00453500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`value` != 0) or clears bit `0x10` of the flags word at `+0x244` of
/// `this` (the script context object at `011ddf38`).
pub fn fn_00453500(e: &mut Engine, this: Ptr, value: u8) {
    let address = this.addr() + 0x244;
    let flags = e.mem.u32(address);
    if value != 0 {
        e.mem.set_u32(address, flags | 0x10);
    } else {
        e.mem.set_u32(address, flags & !0x10);
    }
}

// Translated from 00453550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Per-frame update of the world manager (`delta` seconds): feeds the sky's
/// wind speed and angle (`fWindSpeed` +0xCC, `fWindAngle` +0xD0, read by
/// `004536e0`/`00453700`; zeros with an interior cell loaded) to `00c468c0`
/// and `00c74550`, advances the time accumulator `011c3c08`, steps the
/// interior cell (`00551890`) or the grid cells (`004ba9a0`) with it,
/// runs the task queue object and the collision listener when more than one
/// hardware thread is configured / always, updates the sky (scaled by
/// `fAnimationMult` while in a dialog) on a single thread, and runs
/// `004537c0` there too.
pub fn fn_00453550(e: &mut Engine, this: Ptr<TES>, delta: f32) {
    let interior = e.call(GET_INTERIOR_CELL, &args![this]).u32();
    if interior == 0 {
        let sky = e.get(this, TES::pSky);
        let wind_speed = fn_004536e0(e, sky);
        let sky = e.get(this, TES::pSky);
        let wind_angle = fn_00453700(e, sky);
        // 004535bd / 00453576: the cdecl `00c468c0` reads two words (speed, angle) and
        // pops one (ADD ESP,4); the angle's slot is then reused for `00c74550`.
        e.call(0x00c4_68c0, &args![wind_speed, wind_angle]);
        e.call(0x00c7_4550, &args![wind_speed, wind_angle]);
    } else {
        e.call(0x00c4_68c0, &args![0.0f32, 0.0f32]);
        e.call(0x00c7_4550, &args![0.0f32, 0.0f32]);
    }
    let time = (e.global::<f32>(TIME_ACCUMULATOR) as f64 + delta as f64) as f32;
    e.set_global(TIME_ACCUMULATOR, time);
    e.call(0x0045_3860, &args![this, 1u32]);
    if interior != 0 {
        let time: f32 = e.global(TIME_ACCUMULATOR);
        e.call(0x0055_1890, &args![interior, time, 0u32]);
    } else {
        let grid_cells = e.get(this, TES::pGridCellA);
        let time: f32 = e.global(TIME_ACCUMULATOR);
        e.call(0x004b_a9a0, &args![grid_cells, time]);
    }
    e.call(0x0045_3860, &args![this, 0u32]);
    if setting_uint(e, SETTING_NUM_HW_THREADS) as i32 > 1 {
        let object = fn_004537b0(e);
        e.call(0x0087_aa90, &args![object]);
    }
    let listener = fn_00453720(e);
    e.call(0x0062_3640, &args![listener]);
    if setting_uint(e, SETTING_NUM_HW_THREADS) as i32 == 1 {
        let mut seconds = delta as f64;
        if e.call(0x0070_50d0, &args![]).bool() {
            let frame_time = e.call(0x0084_d030, &args![FRAME_TIME_OBJECT]).f64();
            seconds = setting_float(e, SETTING_ANIMATION_MULT) as f64 * frame_time;
        }
        let sky = e.get(this, TES::pSky);
        e.call(0x0063_ac70, &args![sky, seconds as f32]);
    }
    if setting_uint(e, SETTING_NUM_HW_THREADS) as i32 == 1 {
        tes_update_cell_main_thread(e, this);
    }
    e.call(0x0097_5080, &args![OBJECT_011E0E80]);
}

// Translated from 004536e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sky::fWindSpeed` (Xbox PDB `+0xCC`), returned in `ST0`.
pub fn fn_004536e0(e: &mut Engine, sky: Ptr) -> f32 {
    e.mem.f32(sky.addr() + 0xcc)
}

// Translated from 00453700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Sky::fWindAngle` (Xbox PDB `+0xD0`), returned in `ST0`.
pub fn fn_00453700(e: &mut Engine, sky: Ptr) -> f32 {
    e.mem.f32(sky.addr() + 0xd0)
}

// Translated from 00453720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The collision listener singleton (`011cc224`): the existing one, or a new
/// 0xC byte object built by `00623430` (null when the allocation fails).
///
/// Not translated: the compiler's exception-unwinding frame.
pub fn fn_00453720(e: &mut Engine) -> u32 {
    let existing: u32 = e.global(COLLISION_LISTENER);
    if existing != 0 {
        return existing;
    }
    let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    if block == 0 {
        0
    } else {
        e.call(0x0062_3430, &args![block]).u32()
    }
}

// Translated from 004537b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at `011df1a8`.
pub fn fn_004537b0(e: &mut Engine) -> u32 {
    e.global(TASK_QUEUE_OBJECT)
}

// Translated from 004537c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::UpdateCellMainThread` (Xbox PDB): updates the sky with the frame
/// time (`00453850`; in a dialog `fAnimationMult` times the value
/// `0084d030` gives) and runs the update of the temp node manager with an
/// update object holding the time accumulator `011c3c08`.
pub fn tes_update_cell_main_thread(e: &mut Engine, this: Ptr<TES>) {
    let seconds = if e.call(0x0070_50d0, &args![]).bool() {
        let frame_time = e.call(0x0084_d030, &args![FRAME_TIME_OBJECT]).f64();
        setting_float(e, SETTING_ANIMATION_MULT) as f64 * frame_time
    } else {
        e.call(0x0045_3850, &args![]).f64()
    };
    let sky = e.get(this, TES::pSky);
    e.call(0x0063_ac70, &args![sky, seconds as f32]);
    e.with_stack(12, |e, update| {
        let time: f32 = e.global(TIME_ACCUMULATOR);
        e.call(UPDATE_OBJECT_CTOR, &args![update, time, 0u8, 0u8]);
        let temp_node_manager = e.call(0x0096_11e0, &args![this]).u32();
        e.call(NODE_UPDATE, &args![temp_node_manager, update]);
    });
}

// Translated from 00453a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::QInstance` (Xbox PDB): returns the word at `011f6d98`.
pub fn bs_audio_q_instance(e: &mut Engine) -> u32 {
    e.global(AUDIO_INSTANCE)
}

// Translated from 00453a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes one cell out of the cell buffers to unload and unloads it
/// (`TESDataHandler::UnloadCell`, `00462290`); true when there was one.
///
/// With no interior cell loaded: the last non-empty slot of the interior
/// buffer (`uInterior Cell Buffer`) is taken; failing that the exterior
/// buffer is sorted (`TES::SortExteriorBuffer`, `00454fc0`) and its last slot
/// (above the first `grid * grid` slots, the loaded square) that is neither
/// loaded (`TES::IsCellLoaded`) nor pinned (`00557090`) is taken.
/// With an interior cell: the exterior buffer is searched the same way, then
/// the interior buffer for a slot other than the loaded interior cell, then
/// the whole exterior buffer from the front (which also runs `00453940`).
/// After an unload with the first exterior slot empty and a world space, the
/// terrain manager is told (`006fce00`).
pub fn fn_00453a80(e: &mut Engine, this: Ptr<TES>) -> bool {
    let mut taken = 0u32;
    if e.call(GET_INTERIOR_CELL, &args![this]).u32() == 0 {
        let mut index = setting_uint(e, SETTING_INTERIOR_CELL_BUFFER).wrapping_sub(1) as i32;
        while index >= 0 {
            let buffer = e.get(this, TES::pInteriorBuffer).addr();
            let slot = buffer.wrapping_add((index as u32).wrapping_mul(4));
            if e.mem.u32(slot) != 0 {
                taken = e.mem.u32(slot);
                e.mem.set_u32(slot, 0);
                break;
            }
            index = index.wrapping_sub(1);
        }
        if taken == 0 {
            e.call(0x0045_4fc0, &args![this]);
            let lower = loaded_square_cells(e, this);
            let mut index = setting_uint(e, SETTING_EXTERIOR_CELL_BUFFER).wrapping_sub(1);
            while index >= lower {
                let buffer = e.get(this, TES::pExteriorBuffer).addr();
                let slot = buffer.wrapping_add(index.wrapping_mul(4));
                let cell = e.mem.u32(slot);
                if cell != 0
                    && !tes_is_cell_loaded(e, this, Ptr::new(cell), 1)
                    && !e.call(0x0055_7090, &args![cell]).bool()
                {
                    taken = cell;
                    let buffer = e.get(this, TES::pExteriorBuffer).addr();
                    e.mem.set_u32(buffer.wrapping_add(index.wrapping_mul(4)), 0);
                    break;
                }
                index = index.wrapping_sub(1);
            }
        }
    } else {
        let lower = loaded_square_cells(e, this);
        let mut index = setting_uint(e, SETTING_EXTERIOR_CELL_BUFFER).wrapping_sub(1);
        while index >= lower {
            let buffer = e.get(this, TES::pExteriorBuffer).addr();
            let slot = buffer.wrapping_add(index.wrapping_mul(4));
            let cell = e.mem.u32(slot);
            if cell != 0
                && !tes_is_cell_loaded(e, this, Ptr::new(cell), 1)
                && !e.call(0x0055_7090, &args![cell]).bool()
            {
                taken = cell;
                let buffer = e.get(this, TES::pExteriorBuffer).addr();
                e.mem.set_u32(buffer.wrapping_add(index.wrapping_mul(4)), 0);
                break;
            }
            index = index.wrapping_sub(1);
        }
        if taken == 0 {
            let mut index = setting_uint(e, SETTING_INTERIOR_CELL_BUFFER).wrapping_sub(1) as i32;
            while index >= 0 {
                let buffer = e.get(this, TES::pInteriorBuffer).addr();
                let slot = buffer.wrapping_add((index as u32).wrapping_mul(4));
                let cell = e.mem.u32(slot);
                if cell != 0 {
                    let interior = e.call(GET_INTERIOR_CELL, &args![this]).u32();
                    if cell != interior {
                        taken = cell;
                        let buffer = e.get(this, TES::pInteriorBuffer).addr();
                        e.mem
                            .set_u32(buffer.wrapping_add((index as u32).wrapping_mul(4)), 0);
                        break;
                    }
                }
                index = index.wrapping_sub(1);
            }
        }
        if taken == 0 {
            let mut index = 0u32;
            while index < lower {
                let buffer = e.get(this, TES::pExteriorBuffer).addr();
                let slot = buffer.wrapping_add(index.wrapping_mul(4));
                let cell = e.mem.u32(slot);
                if cell != 0
                    && !tes_is_cell_loaded(e, this, Ptr::new(cell), 1)
                    && !e.call(0x0055_7090, &args![cell]).bool()
                {
                    taken = cell;
                    let buffer = e.get(this, TES::pExteriorBuffer).addr();
                    e.mem.set_u32(buffer.wrapping_add(index.wrapping_mul(4)), 0);
                    e.call(0x0045_3940, &args![this]);
                    break;
                }
                index += 1;
            }
        }
    }
    if taken == 0 {
        return false;
    }
    let data_handler: u32 = e.global(DATA_HANDLER);
    e.call(DATA_HANDLER_UNLOAD_CELL, &args![data_handler, taken]);
    let buffer = e.get(this, TES::pExteriorBuffer).addr();
    if e.mem.u32(buffer) == 0 && e.call(GET_WORLD_SPACE, &args![this]).u32() != 0 {
        let world_space = e.call(GET_WORLD_SPACE, &args![this]).u32();
        let terrain = e.call(GET_TERRAIN_MANAGER, &args![world_space]).u32();
        e.call(0x006f_ce00, &args![terrain]);
    }
    true
}

/// `GridCellArray`'s two `0084e3a0` counts multiplied: the number of slots
/// of the loaded square at the front of the exterior buffer.
pub(crate) fn loaded_square_cells(e: &mut Engine, this: Ptr<TES>) -> u32 {
    let grid_cells = e.get(this, TES::pGridCellA);
    let first = e.call(0x0084_e3a0, &args![grid_cells]).i32();
    let grid_cells = e.get(this, TES::pGridCellA);
    let second = e.call(0x0084_e3a0, &args![grid_cells]).i32();
    first.wrapping_mul(second) as u32
}

// Translated from 00453dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Switches the loaded area to `cell` (null: back to the exterior at `pos`,
/// two floats). When `cell` is already the loaded interior cell this is just
/// `UpdateCurrentGridCell(pos, 0)`.
///
/// Otherwise the old area is torn down (the menu background is drawn first
/// unless a cell test runs; the havok world flags are reset; the shadow scene
/// node is cleared; an old interior cell is detached; a cell that is not in
/// the buffer takes the place of the last interior-buffer slot). Then the new
/// cell becomes the loaded one (`008d7dc0`). A non-null `cell` is attached to
/// the object root, entered in the loaded state (havok world, models,
/// lights, furniture, fader) and its name's image is built and applied; for
/// the exterior case the grid shadow state is reset. Always ends with the
/// obstacle manager refresh `006c0720`/`006c39c0` unless a script runs.
///
/// Not translated: the compiler's exception-unwinding frame.
pub fn fn_00453dc0(e: &mut Engine, this: Ptr<TES>, cell: Ptr, pos: Ptr) {
    let script_context: u32 = e.global(SCRIPT_CONTEXT);
    if !fn_00451530(e, this) {
        let main: u32 = e.global(MAIN_OBJECT);
        e.call(0x0087_1dc0, &args![main]);
    }
    let world = fn_00451010(e);
    fn_00454380(e, Ptr::new(world), 0);
    let world = fn_00451010(e);
    e.call(HAVOK_WORLD_SET_VISUAL_DEBUGGER, &args![world, 0u32]);
    let mut current = e.call(GET_INTERIOR_CELL, &args![this]).u32();
    if current == cell.addr() {
        tes_update_current_grid_cell(e, this, pos, 0);
        return;
    }
    let accumulator = e.call(0x00b4_f5c0, &args![]).u32();
    if accumulator != 0 {
        let accumulator = e.call(0x00b4_f5c0, &args![]).u32();
        e.call(0x00b6_5570, &args![accumulator]);
    }
    if setting_byte(e, SETTING_PREEMPTIVELY_UNLOAD_CELLS) != 0 {
        if current == 0 {
            if e.call(0x0045_5200, &args![this, 1u32, 0u32]).u32() != 0 {
                tes_clean_up_unused_textures(e, this, 0);
            }
        } else {
            e.call(0x0045_3940, &args![this]);
            if e.call(0x0045_5200, &args![this, 1u32, 0u32]).u32() != 0 {
                tes_clean_up_unused_textures(e, this, 0);
            }
        }
    }
    if !e.call(0x0042_ce10, &args![script_context]).bool()
        && !e.call(0x0070_2680, &args![0x3efu32, 0u32]).bool()
    {
        let movie_player: u32 = e.global(MOVIE_PLAYER);
        if !e.call(0x00ec_17c0, &args![movie_player]).bool() {
            let main: u32 = e.global(MAIN_OBJECT);
            e.call(0x0086_ff70, &args![main]);
        }
    }
    let list = e.call(0x0054_6c20, &args![cell]).u32();
    if list != 0 {
        e.call(0x0052_6270, &args![list]);
    }
    let value = e.call(0x0054_7610, &args![cell]).u32();
    e.call(0x0058_eeb0, &args![value]);
    let shadow_scene_node = fn_00450b80(e, 0);
    e.call(0x00b5_ddf0, &args![shadow_scene_node, 0u32]);
    let shadow_scene_node = fn_00450b80(e, 0);
    e.call(0x00b5_fd60, &args![shadow_scene_node]);
    let shadow_scene_node = fn_00450b80(e, 0);
    e.call(0x00b5_d180, &args![shadow_scene_node]);
    let mut left_interior = false;
    if current != 0 {
        left_interior = true;
        e.call(CELL_DETACH, &args![current, 1u32]);
        e.call(0x00b4_f9a0, &args![]);
        let player: u32 = e.global(PLAYER_CHARACTER);
        let value = e.call(0x0095_0bb0, &args![player, 0u32]).u32();
        let shadow_scene_node = fn_00450b80(e, 0);
        e.call(0x00b5_cbd0, &args![shadow_scene_node, value]);
    }
    if !e.call(0x0045_4bd0, &args![this, cell]).bool() {
        let last = setting_uint(e, SETTING_INTERIOR_CELL_BUFFER).wrapping_sub(1);
        let buffered = fn_00454420(e, this, last as i32);
        if buffered != 0 {
            e.call(0x0045_5330, &args![this, buffered]);
        }
    }
    current = cell.addr();
    e.call(0x008d_7dc0, &args![this, cell]);
    fn_00454440(e, 1);
    if current != 0 {
        if e.get(this, TES::iSaveGridX) == 0x7fff_ffff && !left_interior {
            let grid_x = e.get(this, TES::iCurrentGridX);
            e.set(this, TES::iSaveGridX, grid_x);
            let grid_y = e.get(this, TES::iCurrentGridY);
            e.set(this, TES::iSaveGridY, grid_y);
            tes_cull_grid_cells(e, this, 1, 0);
        }
        let x_value = e.mem.f32(pos.addr());
        let cell_x = e.call(FLOAT_TO_INT_ROUNDED, &args![x_value]).i32() >> 12;
        e.set(this, TES::iCurrentGridX, cell_x);
        let y_value = e.mem.f32(pos.addr() + 4);
        let cell_y = e.call(FLOAT_TO_INT_ROUNDED, &args![y_value]).i32() >> 12;
        e.set(this, TES::iCurrentGridY, cell_y);
        e.call(CELL_LOAD_ALL_TEMP_DATA, &args![current]);
        let script_running = e.call(0x0042_ce10, &args![script_context]).bool();
        let save_load: u32 = e.global(SAVE_LOAD_GAME);
        // The save/load answer is stored in a local the code never reads.
        e.call(0x0047_c850, &args![save_load]);
        if fn_00454400(e, Ptr::new(current)) != 0 && !fn_00450fb0(e, Ptr::new(current)) {
            e.call(0x0045_7d70, &args![this, 1u32, current, 0u32]);
        }
        let object_root = e.get(this, TES::pObjRoot);
        e.call(CELL_ATTACH_TO_WORLD, &args![current, object_root]);
        let havok_world = fn_004543c0(e, Ptr::new(current));
        let base = this.addr();
        if e.call(NI_POINTER_GET, &args![base + 0xc0]).u32() != 0 {
            let bound = e.call(NI_POINTER_GET, &args![base + 0xc0]).u32();
            e.call(0x0062_8da0, &args![bound]);
        }
        if havok_world != 0 {
            e.call(HAVOK_WORLD_ADD_COUNT_18, &args![havok_world]);
        }
        e.call(0x0054_bcf0, &args![current, 1u32]);
        let model_loader: u32 = e.global(MODEL_LOADER);
        if e.call(
            MODEL_LOADER_CELL_NEEDS_MODELS,
            &args![model_loader, current],
        )
        .bool()
        {
            e.call(0x0045_7d70, &args![this, 1u32, current, 0u32]);
        }
        e.call(0x0045_5400, &args![this, current]);
        if !script_running {
            let io_manager: u32 = e.global(IO_MANAGER);
            e.call(0x00c3_dfa0, &args![io_manager, 1u32]);
        }
        if havok_world != 0 {
            e.call(HAVOK_WORLD_DROP_COUNT_18, &args![havok_world, 0u32]);
        }
        let tes_singleton: u32 = e.global(TES_SINGLETON);
        e.call(0x0045_cda0, &args![tes_singleton]);
        if !fn_00451530(e, this) && !script_running {
            let player: u32 = e.global(PLAYER_CHARACTER);
            if e.vcall(player, PLAYER_SLOT_FADE_OBJECT, &args![]).u32() != 0 {
                let fader_manager: u32 = e.global(FADER_MANAGER);
                e.call(
                    FADER_CREATE_FADER,
                    &args![fader_manager, 1u32, 0.0f32, 0u32],
                );
            }
        }
        if !script_running {
            e.call(0x0097_5f90, &args![OBJECT_011E0E80]);
            e.call(0x0045_9870, &args![this]);
            e.call(0x0097_2d30, &args![OBJECT_011E0E80]);
            e.call(0x0097_2bb0, &args![OBJECT_011E0E80]);
        }
        tes_clean_up_unused_textures(e, this, 0);
        let data_handler: u32 = e.global(DATA_HANDLER);
        e.call(0x0046_0360, &args![data_handler]);
        let node = e.call(CELL_LOAD_3D, &args![current]).u32();
        e.call(NODE_UPDATE_PROPERTIES, &args![node]);
        if fn_00453470(e, Ptr::new(node)) == 0 {
            e.with_stack(12, |e, update| {
                e.call(UPDATE_OBJECT_CTOR, &args![update, 0.0f32, 0u8, 0u8]);
                e.call(NODE_UPDATE, &args![node, update]);
            });
        }
        e.call(CELL_ATTACH_LIGHTS, &args![current, 1u32]);
        e.call(0x0045_4c20, &args![this, cell]);
    }
    if !left_interior {
        let accumulator = e.call(0x00b4_f5c0, &args![]).u32();
        if accumulator != 0 {
            e.call(0x00b6_31d0, &args![accumulator]);
        }
        e.call(0x00b6_2a20, &args![]);
        let grid_cells = e.get(this, TES::pGridCellA);
        e.call(0x004b_aec0, &args![grid_cells]);
    }
    if fn_00451530(e, this) {
        e.call(0x0045_7d70, &args![this, 0u32, 0u32, 0u32]);
    }
    tes_clean_up_unused_textures(e, this, 0);
    e.call(0x0070_38b0, &args![0u32]);
    let name = fn_004543a0(e, Ptr::new(current));
    let named = e.mem.u8(name) as i8 != 0;
    apply_location_image(e, 0xc74, named, |e| fn_004543a0(e, Ptr::new(current)));
    if !e.call(0x0042_ce10, &args![script_context]).bool() {
        let first = e.call(0x006c_0720, &args![]).u32();
        e.call(0x006c_39c0, &args![first]);
    }
}

// Translated from 00454380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte at `+0x15` of `this` (the havok world).
pub fn fn_00454380(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x15, value);
}

// Translated from 004543a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `MapMarkerData::GetLocationName` (Xbox PDB, `00408da0`) on
/// `this + 0x58` and returns its result (the string pointer, or the empty
/// string `01011584`). The code reads the result as the name of a cell or
/// world space.
pub fn fn_004543a0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0040_8da0, &args![this.addr() + 0x58]).u32()
}

// Translated from 004543c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The havok world of a cell: for a cell whose bit 0 of `+0x24` is set (an
/// interior; `00425fd0`) `0041b9a0(cell + 0x28)`, otherwise the exterior
/// world (`00451010`).
pub fn fn_004543c0(e: &mut Engine, cell: Ptr) -> u32 {
    if e.call(0x0042_5fd0, &args![cell]).bool() {
        e.call(0x0041_b9a0, &args![cell.addr() + 0x28]).u32()
    } else {
        fn_00451010(e)
    }
}

// Translated from 00454400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATS::GetCount`'s twin (Xbox PDB name `VATS::GetCount` for the shared
/// body): the number of non-null entries of the list at `this + 0xac`
/// (`005ae380`).
pub fn fn_00454400(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x005a_e380, &args![this.addr() + 0xac]).u32()
}

// Translated from 00454420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `index` of the interior cell buffer (`pInteriorBuffer`, +0x38).
pub fn fn_00454420(e: &mut Engine, this: Ptr<TES>, index: i32) -> u32 {
    let buffer = e.get(this, TES::pInteriorBuffer).addr();
    e.mem
        .u32(buffer.wrapping_add((index as u32).wrapping_mul(4)))
}

// Translated from 00454440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte in `011f9427`.
pub fn fn_00454440(e: &mut Engine, value: u8) {
    e.set_global(FLAG_011F9427, value);
}

// Translated from 00454450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Leaves the interior cell and returns to the exterior around `pos` (two
/// floats): the terrain manager is told the position, an interior cell is
/// detached and unloaded or buffered, the saved grid is restored (or reset),
/// the grid is rebuilt (`UpdateCurrentGridCell(pos, settle)`), the player
/// area is attached again, the fade started, the lights and furniture set up,
/// and the sky, image and obstacle state refreshed.
///
/// Not translated: the compiler's exception-unwinding frame.
pub fn fn_00454450(e: &mut Engine, this: Ptr<TES>, pos: Ptr) {
    let script_context: u32 = e.global(SCRIPT_CONTEXT);
    let world_space = e.call(GET_WORLD_SPACE, &args![this]).u32();
    if world_space != 0 {
        let terrain = e.call(GET_TERRAIN_MANAGER, &args![world_space]).u32();
        e.call(TERRAIN_MANAGER_UPDATE, &args![terrain, pos, 0xfu32]);
        let terrain = e.call(GET_TERRAIN_MANAGER, &args![world_space]).u32();
        e.call(0x006f_cdb0, &args![terrain, pos]);
    }
    fn_00454440(e, 0);
    let world_space = e.get(this, TES::pWorldSpace);
    if !world_space.is_null() {
        let value = e.call(0x0067_33e0, &args![world_space]).u32();
        e.call(0x0058_eeb0, &args![value]);
    }
    e.call(0x00b5_8070, &args![0u32]);
    let world = fn_00451010(e);
    fn_00454380(e, Ptr::new(world), 1);
    let interior = e.call(GET_INTERIOR_CELL, &args![this]).u32();
    if interior != 0 {
        let shadow_scene_node = fn_00450b80(e, 0);
        let node_cell = e.call(0x0045_4b30, &args![shadow_scene_node]).u32();
        let interior_cell = e.call(0x009d_9f20, &args![interior]).u32();
        if node_cell == interior_cell {
            let shadow_scene_node = fn_00450b80(e, 0);
            e.call(0x00b5_ddf0, &args![shadow_scene_node, 0u32]);
        }
        e.call(CELL_DETACH, &args![interior, 1u32]);
        if !e.call(0x0045_4bd0, &args![this, interior]).bool() {
            let data_handler: u32 = e.global(DATA_HANDLER);
            e.call(DATA_HANDLER_UNLOAD_CELL, &args![data_handler, interior]);
        } else {
            e.call(0x0054_af40, &args![interior, 1u32]);
        }
        e.call(0x008d_7dc0, &args![this, 0u32]);
        if !e.call(0x0042_ce10, &args![script_context]).bool() {
            if e.get(this, TES::iSaveGridX) == 0x7fff_ffff {
                if e.call(0x0045_7620, &args![this, pos]).u32() == 0 {
                    e.set(this, TES::iCurrentGridX, 0x7fff_ffff);
                    e.set(this, TES::iCurrentGridY, 0x7fff_ffff);
                    let grid_x = e.get(this, TES::iCurrentGridX);
                    e.set(this, TES::iCurrentQueuedX, grid_x);
                    let grid_y = e.get(this, TES::iCurrentGridY);
                    e.set(this, TES::iCurrentQueuedY, grid_y);
                }
            } else {
                let exterior_buffer = e.get(this, TES::pExteriorBuffer).addr();
                if e.mem.u32(exterior_buffer) != 0 {
                    let save_x = e.get(this, TES::iSaveGridX);
                    e.set(this, TES::iCurrentGridX, save_x);
                    let save_y = e.get(this, TES::iSaveGridY);
                    e.set(this, TES::iCurrentGridY, save_y);
                    tes_cull_grid_cells(e, this, 0, pos.addr());
                } else {
                    e.set(this, TES::iCurrentGridX, 0x7fff_ffff);
                    e.set(this, TES::iCurrentGridY, 0x7fff_ffff);
                    let grid_x = e.get(this, TES::iCurrentGridX);
                    e.set(this, TES::iCurrentQueuedX, grid_x);
                    let grid_y = e.get(this, TES::iCurrentGridY);
                    e.set(this, TES::iCurrentQueuedY, grid_y);
                }
                e.set(this, TES::iSaveGridX, 0x7fff_ffff);
                e.set(this, TES::iSaveGridY, 0x7fff_ffff);
            }
        }
        let shadow_scene_node = fn_00450b80(e, 0);
        e.call(0x00b5_d180, &args![shadow_scene_node]);
        let player: u32 = e.global(PLAYER_CHARACTER);
        let value = e.call(0x0095_0bb0, &args![player, 0u32]).u32();
        let shadow_scene_node = fn_00450b80(e, 0);
        e.call(0x00b5_cbd0, &args![shadow_scene_node, value]);
    }
    let land_root = e.get(this, TES::pObjLandRoot);
    if !land_root.is_null() {
        let node = e.call(0x0096_11e0, &args![land_root]).u32();
        if node != 0 {
            fn_00450f90(e, Ptr::new(node), 0);
        }
        e.call(0x006f_d1f0, &args![1u32]);
    }
    fn_00451010(e);
    let debugger = fn_00454ae0(e);
    let world = fn_00451010(e);
    e.call(
        HAVOK_WORLD_SET_VISUAL_DEBUGGER,
        &args![world, debugger as u32],
    );
    let script_running = e.call(0x0042_ce10, &args![script_context]).bool();
    let save_load: u32 = e.global(SAVE_LOAD_GAME);
    let save_load_answer = e.call(0x0047_c850, &args![save_load]).u8();
    tes_update_current_grid_cell(e, this, pos, (save_load_answer == 0) as u8);
    let grid_x = e.get(this, TES::iCurrentGridX);
    let grid_y = e.get(this, TES::iCurrentGridY);
    let centre_cell = fn_00451900(e, this, grid_x, grid_y);
    let model_loader: u32 = e.global(MODEL_LOADER);
    if e.call(
        MODEL_LOADER_CELL_NEEDS_MODELS,
        &args![model_loader, centre_cell],
    )
    .bool()
    {
        e.call(0x0045_7d70, &args![this, 1u32, centre_cell, 0u32]);
    }
    if fn_00451010(e) != 0 {
        let world = fn_00451010(e);
        e.call(HAVOK_WORLD_ADD_COUNT_18, &args![world]);
    }
    if !script_running {
        let io_manager: u32 = e.global(IO_MANAGER);
        e.call(0x00c3_dfa0, &args![io_manager, 4u32]);
    }
    let value = e.call(0x0044_ddc0, &args![this]).u32();
    e.call(0x004b_a7d0, &args![value]);
    e.call(0x0045_c840, &args![this]);
    e.call(0x0045_c780, &args![this]);
    if fn_00451010(e) != 0 {
        let world = fn_00451010(e);
        e.call(HAVOK_WORLD_DROP_COUNT_18, &args![world, 0u32]);
    }
    e.call(0x0045_cda0, &args![this]);
    // A 12-byte local: the position (x, y, 0.0) built by `00416870`; the
    // third float is then overwritten by `004572e0`.
    e.with_stack(12, |e, point| {
        let x = e.mem.f32(pos.addr());
        let y = e.mem.f32(pos.addr() + 4);
        e.call(0x0041_6870, &args![point, x, y, 0.0f32]);
        e.call(0x0045_72e0, &args![this, pos, point.addr() + 8]);
        fn_00451590(e, 0);
        let point_words = [
            e.mem.u32(point.addr()),
            e.mem.u32(point.addr() + 4),
            e.mem.u32(point.addr() + 8),
        ];
        let global_words = [
            e.global::<u32>(POINT_011A9478),
            e.global::<u32>(POINT_011A9478 + 4),
            e.global::<u32>(POINT_011A9478 + 8),
        ];
        e.call(
            0x0057_d0a0,
            &args![
                point_words[0],
                point_words[1],
                point_words[2],
                global_words[0],
                global_words[1],
                global_words[2],
                1.0f32
            ],
        );
    });
    fn_00451590(e, 1);
    if !fn_00451530(e, this) && !script_running {
        let seconds = setting_float(e, SETTING_FADE_TO_BLACK_FADE_SECONDS);
        let fader_manager: u32 = e.global(FADER_MANAGER);
        // The trailing 1 is the word the code pushed before reading the
        // setting.
        e.call(
            FADER_CREATE_FADER,
            &args![fader_manager, 1u32, seconds, 1u32],
        );
    }
    if !script_running {
        e.call(0x0097_5f90, &args![OBJECT_011E0E80]);
        e.call(0x0045_9870, &args![this]);
        e.call(0x0097_2d30, &args![OBJECT_011E0E80]);
        e.call(0x0097_2bb0, &args![OBJECT_011E0E80]);
    }
    if interior != 0 && !e.call(0x0045_4b10, &args![interior]).bool() {
        let sky = e.get(this, TES::pSky).addr();
        let first = e.call(0x0044_edb0, &args![sky]).u32();
        e.call(0x0063_ddb0, &args![sky, first]);
        let sky = e.get(this, TES::pSky).addr();
        let second = e.call(0x0082_5c00, &args![sky]).u32();
        let sky = e.get(this, TES::pSky).addr();
        e.call(0x0063_ddb0, &args![sky, second]);
    }
    tes_update_grid_lights(e, this);
    e.call(0x0045_5400, &args![this, 0u32]);
    let object_root = e.get(this, TES::pObjRoot);
    e.call(NODE_UPDATE_PROPERTIES, &args![object_root]);
    e.call(0x0070_38b0, &args![0u32]);
    if fn_00451530(e, this) {
        e.call(0x0045_7d70, &args![this, 0u32, 0u32, 0u32]);
    }
    tes_clean_up_unused_textures(e, this, 0);
    let mut named = false;
    let world_space = e.get(this, TES::pWorldSpace);
    if !world_space.is_null() {
        let text = e.call(0x0045_4b50, &args![world_space]).u32();
        named = e.mem.u8(text) as i8 != 0;
    }
    apply_location_image(e, 0xd6a, named, |e| {
        let world_space = e.get(this, TES::pWorldSpace);
        e.call(0x0045_4b50, &args![world_space]).u32()
    });
    e.set_global(FLAG_01189625, 1u8);
    store_grid_centre(e, this);
    copy_grid_centre_to_previous(e);
    let wind = fn_00452e70(e, 0);
    e.set_global(GRID_FLOAT_011F94AC, wind);
    e.set_global(GRID_FLAG_MOVED, 0u8);
    let moved: u8 = e.global(GRID_FLAG_MOVED);
    e.set_global(GRID_FLAG_MOVED_COPY, moved);
    let world_space = e.call(GET_WORLD_SPACE, &args![this]).u32();
    let terrain = e.call(GET_TERRAIN_MANAGER, &args![world_space]).u32();
    let terrain_flag = e.call(0x0045_4b70, &args![terrain]).bool();
    e.set_global(GRID_FLAG_TERRAIN, terrain_flag as u8);
    if !e.call(0x0042_ce10, &args![script_context]).bool() {
        let first = e.call(0x006c_0720, &args![]).u32();
        e.call(0x006c_39c0, &args![first]);
    }
}

// Translated from 00454ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bHavokDebug:HAVOK`'s value (`00454af0` on the setting at `01267b28`).
pub fn fn_00454ae0(e: &mut Engine) -> u8 {
    fn_00454af0(e, SETTING_HAVOK_DEBUG)
}

// Translated from 00454af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `bool` setting's value: the byte `00408d60` finds in the setting `this`.
pub fn fn_00454af0(e: &mut Engine, this: u32) -> u8 {
    setting_byte(e, this)
}

// ---------------------------------------------------------------------------
// Session 4 (the next 40 functions, `00454b10` to `00457990`): the interior
// and exterior cell buffers, the script and sound passes over the loaded
// cells, the cell test run `TES::TestAllCells`, the texture-image cache
// helpers, and the "land at a position" helpers.
// ---------------------------------------------------------------------------

/// `TESObjectCELL` test: bit 0 of the byte at +0x24 (`cCellFlags`; Xbox PDB
/// `+0x34`), which the cell buffers and the world code treat as "interior".
pub(crate) const CELL_IS_INTERIOR: u32 = 0x0042_5fd0;
/// A `TESObjectCELL` test on `cell + 0x80` (through `005570b0`); the buffer
/// code leaves a cell for which it is true where it is (neither unloaded nor
/// replaced).
pub(crate) const CELL_PINNED: u32 = 0x0055_7090;
/// `TESObjectCELL::GetWorldSpace` (Xbox PDB): `TESWorldSpace*`.
pub(crate) const CELL_GET_WORLD_SPACE: u32 = 0x0054_ddd0;
/// `TESObjectCELL::RunScripts` (Xbox PDB): `(cell, 0, 0)`, a byte answer.
pub(crate) const CELL_RUN_SCRIPTS: u32 = 0x0054_c740;
/// `TESObjectCELL::GetLand` (Xbox PDB): the cell's `TESObjectLAND*`.
pub(crate) const CELL_GET_LAND: u32 = 0x0054_6fb0;
/// `TESObjectLAND::GetLandHeight` (Xbox PDB): `(land, pos, &height)`.
pub(crate) const LAND_GET_LAND_HEIGHT: u32 = 0x0053_f180;
/// `TESObjectREFR::RunScript` (Xbox PDB) on the player.
pub(crate) const REFERENCE_RUN_SCRIPT: u32 = 0x0056_5870;
/// `Sky::SetMode` (Xbox PDB): `(sky, mode)`.
pub(crate) const SKY_SET_MODE: u32 = 0x0063_a3f0;
/// Returns `this + 0x68` (`TES::pSky`; the map names it
/// `MiddleHighProcess::GetSavedAcquireObject`, identical code).
pub(crate) const TES_GET_SKY: u32 = 0x008d_8520;
/// Returns the word at `this + 0xf8` of the sky (non-null while a sky is
/// built).
pub(crate) const SKY_GET_WORD_F8: u32 = 0x0047_6c90;
/// Returns `this` itself (the address of a list node's item word: an empty
/// constructor for a list item or a vector, and the item getter of a list
/// node).
pub(crate) const LIST_NODE_ITEM: u32 = 0x0068_15c0;
/// Returns the next list node (`this + 4`).
pub(crate) const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// The `TESDataHandler`'s first node of the world-space list (`this + 0x10`).
pub(crate) const DATA_HANDLER_FIRST_WORLD_SPACE: u32 = 0x0046_0140;
/// Runs the next script of the quest `this` (`TESQuest`, `fallout shared/tesquest.cpp`).
pub(crate) const QUEST_RUN: u32 = 0x0060_d4d0;
/// `Script::GetProcessScripts` (Xbox PDB): whether scripts run.
pub(crate) const SCRIPT_GET_PROCESS_SCRIPTS: u32 = 0x005a_c740;
/// Object `00455490` passes to `0040fbf0` and `0040fba0` around its script
/// pass.
pub(crate) const SCRIPT_PASS_OBJECT: u32 = 0x011f_11a0;
/// Byte set while `00455490` runs (re-entrancy guard).
pub(crate) const SCRIPT_PASS_RUNNING: u32 = 0x011c_3ed1;
/// `thiscall(this, callback)`: stores `callback` at `this + 0x54` (the map
/// calls it `IOManager::Pause`; identical code).
pub(crate) const SET_TEST_CALLBACK: u32 = 0x005a_8040;
/// Dword `00455680` clears.
pub(crate) const COUNTER_011C56E8: u32 = 0x011c_56e8;
/// `GridCellArray::Get` is `GRID_CELL_ARRAY_GET`; `GridCell` code behind the
/// grid-wide passes: `004baba0` and `004bac20` (`gridcell.cpp`) take the grid
/// array.
pub(crate) const GRID_CELL_ARRAY_PASS_A: u32 = 0x004b_aba0;
pub(crate) const GRID_CELL_ARRAY_PASS_B: u32 = 0x004b_ac20;
/// The same passes for one cell (`tesobjectcell.cpp`): `00553820` and
/// `005538e0` (`TESObjectCELL::UpdateRefSounds`, Xbox PDB).
pub(crate) const CELL_PASS_A: u32 = 0x0055_3820;
pub(crate) const CELL_PASS_B: u32 = 0x0055_38e0;
/// `00553bf0` and `00553d00` (`tesobjectcell.cpp`): the per-cell versions of
/// the two position tests `004570d0` and `004571c0`.
pub(crate) const CELL_TEST_A: u32 = 0x0055_3bf0;
pub(crate) const CELL_TEST_B: u32 = 0x0055_3d00;
/// `memmove` (CRT, cdecl `(dest, source, bytes)`).
pub(crate) const MEMMOVE: u32 = 0x00ec_7230;

// Globals of `TES::TestAllCells`.

/// `TestAllCells` state: base-object checks done.
pub(crate) const TEST_BASE_OBJECTS_CHECKED: u32 = 0x011c_3ef4;
/// The mode `TestAllCells` was started with.
pub(crate) const TEST_MODE: u32 = 0x011c_3ef0;
/// The current node of the data handler's world-space list (0 while the
/// interiors are tested).
pub(crate) const TEST_WORLD_SPACE_NODE: u32 = 0x011c_3eec;
/// Count of interior cells (set from `00461960`).
pub(crate) const TEST_INTERIOR_COUNT: u32 = 0x011c_3ee8;
/// Set when the next world space has to be entered.
pub(crate) const TEST_ENTER_WORLD_SPACE: u32 = 0x011c_3ee4;
/// Index of the next interior cell.
pub(crate) const TEST_INTERIOR_INDEX: u32 = 0x011c_3ee0;
/// Current exterior cell x and y.
pub(crate) const TEST_CELL_X: u32 = 0x011c_3edc;
pub(crate) const TEST_CELL_Y: u32 = 0x011c_3ed8;
/// Number of exterior cells tested.
pub(crate) const TEST_EXTERIOR_COUNT: u32 = 0x011c_3ed4;
/// Sign toggled by every `TestAllCells` turn (the mouse move it sends).
pub(crate) const TEST_INPUT_SIGN: u32 = 0x0118_6080;
/// Byte: whether `TestAllCells` writes its per-cell lines.
pub(crate) const TEST_LOGGING: u32 = 0x0118_607c;
/// Pointer to the output file name (`"TestAllCells.xls"`).
pub(crate) const TEST_OUTPUT_FILE: u32 = 0x0118_6084;
/// `"CELLS: ..."` line formats and texts the test prints.
pub(crate) const TEST_TEXT_BASE_OBJECT_MODELS: u32 = 0x0101_7c50;
pub(crate) const TEST_TEXT_BASE_OBJECT_ICONS: u32 = 0x0101_7c08;
pub(crate) const TEST_TEXT_WORLD_SPACE: u32 = 0x0101_7bb8;
pub(crate) const TEST_TEXT_INTERIORS: u32 = 0x0101_7b7c;
pub(crate) const TEST_TEXT_EXTERIOR_LINE: u32 = 0x0101_7a70;
pub(crate) const TEST_TEXT_EXTERIOR_ROW: u32 = 0x0101_7a48;
pub(crate) const TEST_TEXT_INTERIOR_LINE: u32 = 0x0101_7af8;
pub(crate) const TEST_TEXT_INTERIOR_ROW: u32 = 0x0101_7acc;
/// `"test"`: world spaces and cells whose name starts with it are skipped.
pub(crate) const TEST_PREFIX: u32 = 0x0101_7bfc;
/// `"Y"` and `"N"`.
pub(crate) const TEST_TEXT_YES: u32 = 0x0101_7b68;
pub(crate) const TEST_TEXT_NO: u32 = 0x0101_7b64;
/// A `float` (10.0 in the exe) `TestAllCells` passes to `00867a40` (a
/// calendar function) on the object at [`OBJECT_011DE7B8`].
pub(crate) const TEST_FLOAT_TEN: u32 = 0x0101_7b78;
/// A `double` (1000.0 in the exe): `clock()` milliseconds per second.
pub(crate) const TEST_DOUBLE_THOUSAND: u32 = 0x0101_7b70;
/// A `double` (2048.0 in the exe): half a cell, added to the cell's corner.
pub(crate) const TEST_DOUBLE_HALF_CELL: u32 = 0x0101_6968;
/// A `double` (0.0 in the exe) the heights are compared with.
pub(crate) const TEST_DOUBLE_ZERO: u32 = 0x0101_2060;
/// A `NiPoint3` global (all zeros in the exe): the position the test starts
/// from, the origin `NiPick::PickObjects` is given, and the first words of the
/// output of `004573f0`.
pub(crate) const NI_POINT3_ZERO: u32 = 0x011f_426c;
/// The land query cache of `00457720`: x and y of the 128-unit column and the
/// cached answer.
pub(crate) const CACHED_LAND_COLUMN_X: u32 = 0x0118_608c;
pub(crate) const CACHED_LAND_COLUMN_Y: u32 = 0x0118_6088;
pub(crate) const CACHED_LAND_ANSWER: u32 = 0x011c_3ef8;
/// The callback address stored by `TestAllCells` for modes 6 and 7.
pub(crate) const TEST_CALLBACK: u32 = 0x0055_7dd0;
/// `clock()` (CRT).
pub(crate) const CRT_CLOCK: u32 = 0x00ec_7d50;
/// Import slot of `SendInput`.
pub(crate) const IMPORT_SEND_INPUT: u32 = 0x00fd_f2d4;
/// `MessageHandler::IncDisableWarningCount` (Xbox PDB): `(flag)`.
pub(crate) const MESSAGE_HANDLER_INC_DISABLE_WARNING_COUNT: u32 = 0x0043_b2b0;
/// `BSSystemFile::DeleteFileA` (Xbox PDB): `(path)`.
pub(crate) const DELETE_FILE: u32 = 0x00af_f0b0;
/// The debug print `_MESSAGE`-style function: `(format, ...)`, returns 0
/// (its body only returns 0 in this build).
pub(crate) const DEBUG_PRINT: u32 = 0x005b_5e40;
/// `MessageHandler::Output` (Xbox PDB): `(file, format, ...)`, returns 0.
pub(crate) const MESSAGE_HANDLER_OUTPUT: u32 = 0x00c3_c220;
pub(crate) const DATA_HANDLER_CHECK_MODELS: u32 = 0x0046_ddb0;
pub(crate) const DATA_HANDLER_CHECK_ICONS: u32 = 0x0046_ec10;
pub(crate) const DATA_HANDLER_PREPARE_INTERIORS: u32 = 0x0046_19b0;
pub(crate) const DATA_HANDLER_INTERIOR_COUNT: u32 = 0x0046_1960;
pub(crate) const DATA_HANDLER_GET_INTERIOR: u32 = 0x0046_1980;
pub(crate) const WORLD_SPACE_LOAD_CELL: u32 = 0x0058_5b30;
/// The name of a form (`vtable + 0x130`; the text the test prints and compares with `"test"`).
pub(crate) const FORM_SLOT_NAME: u32 = 0x130;
/// `_strnicmp` through `004564f0`: `(a, b, count)`.
pub(crate) const STRNICMP: u32 = 0x0045_64f0;
pub(crate) const WORLD_SPACE_MIN_X: u32 = 0x0045_65f0;
pub(crate) const WORLD_SPACE_MIN_Y: u32 = 0x0081_2870;
pub(crate) const WORLD_SPACE_MAX_X: u32 = 0x009b_88a0;
pub(crate) const WORLD_SPACE_MAX_Y: u32 = 0x009b_88c0;
pub(crate) const CELL_HAS_FLAG_20: u32 = 0x0044_0d80;
pub(crate) const CELL_GET_FIRST_REFR: u32 = 0x0054_cee0;
pub(crate) const CELL_GET_REGION_LIST: u32 = 0x0054_7110;
pub(crate) const REGION_LIST_CONTAINS: u32 = 0x005f_65d0;
pub(crate) const REFERENCE_GET_WORLD_SPACE: u32 = 0x0057_5d70;
pub(crate) const TIME_OBJECT_ADVANCE: u32 = 0x0086_7a40;
pub(crate) const NI_POINT3_CONSTRUCT_FLOATS: u32 = 0x0041_6870;
pub(crate) const CELL_GET_COC_PLACEMENT_INFO: u32 = 0x0054_cfd0;
pub(crate) const POSITION_REQUEST_CONSTRUCT: u32 = 0x0045_6540;
pub(crate) const PLAYER_ROTATION_GETTER: u32 = 0x0043_0830;
pub(crate) const PLAYER_REQUEST_POSITION: u32 = 0x0093_be30;
pub(crate) const PLAYER_HANDLE_POSITION_REQUEST: u32 = 0x0093_bea0;
pub(crate) const IO_MANAGER_LOAD_QUEUED_PRIORITY: u32 = 0x0045_6520;
pub(crate) const FRAME_SCOPE_BEGIN: u32 = 0x0087_8160;
pub(crate) const FRAME_SCOPE_RELEASE: u32 = 0x0087_8250;
pub(crate) const FRAME_SCOPE_END: u32 = 0x0087_8200;
pub(crate) const TES_PURGE_UNUSED: u32 = 0x0045_39a0;
pub(crate) const STRING_ARRAY_CONSTRUCT: u32 = 0x0040_37b0;
pub(crate) const STRING_ARRAY_DESTRUCT: u32 = 0x0040_37d0;
pub(crate) const SCENE_INFO_TOTALS: u32 = 0x004a_8bb0;
pub(crate) const SCENE_INFO_BUILD_M_NUMBER: u32 = 0x004a_9120;
pub(crate) const CELL_GET_ACTOR_HOLDER: u32 = 0x009d_9f20;
pub(crate) const ACTOR_HOLDER_GET_FIELD_20: u32 = 0x0089_1170;
pub(crate) const FIELD_IS_EMPTY: u32 = 0x0076_b610;
pub(crate) const TEXTURE_LIST_HEAD: u32 = 0x0045_6510;
pub(crate) const TEXTURE_IS_LOADED: u32 = 0x0059_bb30;
pub(crate) const TEXTURE_LIST_NEXT: u32 = 0x0055_b980;
pub(crate) const ACTOR_COUNT: u32 = 0x0096_bc50;
pub(crate) const CELL_GET_LIGHT_COUNT: u32 = 0x0054_bb90;
pub(crate) const SAVE_LOAD_GAME_TEST_ALL_CELLS: u32 = 0x0086_1f20;

/// The address of slot `index` of the interior cell buffer.
pub(crate) fn interior_slot(e: &Engine, this: Ptr<TES>, index: u32) -> u32 {
    e.get(this, TES::pInteriorBuffer)
        .addr()
        .wrapping_add(index.wrapping_mul(4))
}

/// The address of slot `index` of the exterior cell buffer.
pub(crate) fn exterior_slot(e: &Engine, this: Ptr<TES>, index: u32) -> u32 {
    e.get(this, TES::pExteriorBuffer)
        .addr()
        .wrapping_add(index.wrapping_mul(4))
}

/// `TESDataHandler::UnloadCell` (`00462290`) on the data handler singleton.
pub(crate) fn unload_cell(e: &mut Engine, cell: u32) {
    let data_handler: u32 = e.global(DATA_HANDLER);
    e.call(DATA_HANDLER_UNLOAD_CELL, &args![data_handler, cell]);
}

/// `uGridsToLoad * uGridsToLoad`, reading the setting twice as the code does.
pub(crate) fn grids_squared(e: &mut Engine) -> u32 {
    let first = e
        .call(SETTING_VALUE_ADDRESS_INT, &args![SETTING_GRIDS_TO_LOAD])
        .u32();
    let second = e
        .call(SETTING_VALUE_ADDRESS_INT, &args![SETTING_GRIDS_TO_LOAD])
        .u32();
    e.mem.u32(first).wrapping_mul(e.mem.u32(second))
}

// Translated from 00454b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 0x80 of the byte at `this + 0x24` of a `TESObjectCELL` (`cCellFlags`,
/// Xbox PDB `+0x34`; bit 0 of the same byte is the interior test
/// `00425fd0`). `00455400` and `TES::TES`'s cell switch use it to choose the
/// sky mode of an interior cell.
pub fn fn_00454b10(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x24) & 0x80 != 0
}

// Translated from 00454b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `this + 0x1e0` (the cell `00454450` reads from the shadow
/// scene node).
pub fn fn_00454b30(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x1e0)
}

// Translated from 00454b48 (decompiled, FalloutNV.exe 1.4.0.525)
/// A single `INT3` byte, the alignment padding after `00454b30`. Nothing
/// calls it; executing it raises a breakpoint exception, which is a panic here.
pub fn fn_00454b48(_e: &mut Engine) {
    panic!("INT3 at 00454b48: breakpoint exception (alignment padding, never called)");
}

// Translated from 00454b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MapMarkerData::GetLocationName` (`00408da0`, Xbox PDB) on `this + 0xe0`
/// (the `MapMarkerData` of a `TESWorldSpace`): the name text.
pub fn fn_00454b50(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0040_8da0, &args![this.addr() + 0xe0]).u32()
}

// Translated from 00454b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the word at `this + 4` is non-null (`00453550` asks it of the
/// terrain manager).
pub fn fn_00454b70(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 4) != 0
}

// Translated from 00454b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::AddToBuffer` (Xbox PDB): puts a non-null `cell` into the interior
/// cell buffer (`00454c20`) when it is an interior cell (`00425fd0`), else
/// into the exterior buffer (`00454e70`).
pub fn tes_add_to_buffer(e: &mut Engine, this: Ptr<TES>, cell: Ptr) {
    if cell.is_null() {
        return;
    }
    if e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
        fn_00454c20(e, this, cell);
    } else {
        fn_00454e70(e, this, cell);
    }
}

// Translated from 00454bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `cell` is one of the `uInterior Cell Buffer` slots of the interior
/// cell buffer.
pub fn fn_00454bd0(e: &mut Engine, this: Ptr<TES>, cell: Ptr) -> bool {
    let mut index = 0u32;
    while index < setting_uint(e, SETTING_INTERIOR_CELL_BUFFER) {
        let slot = interior_slot(e, this, index);
        if e.mem.u32(slot) == cell.addr() {
            return true;
        }
        index += 1;
    }
    false
}

// Translated from 00454c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Puts `cell` at the front of the interior cell buffer. A cell already in
/// the buffer moves to slot 0 (the slots before it shift up by one). A new
/// cell shifts the whole buffer up by one: the last slot's cell, if any, is
/// unloaded (`TESDataHandler::UnloadCell`, `00462290`) and lost, then
/// `cell` takes slot 0.
pub fn fn_00454c20(e: &mut Engine, this: Ptr<TES>, cell: Ptr) {
    if fn_00454bd0(e, this, cell) {
        let mut index = 0u32;
        while index < setting_uint(e, SETTING_INTERIOR_CELL_BUFFER) {
            let slot = interior_slot(e, this, index);
            if e.mem.u32(slot) == cell.addr() {
                while index != 0 {
                    let from = interior_slot(e, this, index - 1);
                    let value = e.mem.u32(from);
                    let to = interior_slot(e, this, index);
                    e.mem.set_u32(to, value);
                    index -= 1;
                }
                let first = interior_slot(e, this, 0);
                e.mem.set_u32(first, cell.addr());
                return;
            }
            index += 1;
        }
    } else {
        let last = setting_uint(e, SETTING_INTERIOR_CELL_BUFFER).wrapping_sub(1) as i32;
        let mut index = last;
        while index >= 0 {
            if index == last {
                let slot = interior_slot(e, this, index as u32);
                let old = e.mem.u32(slot);
                if old != 0 {
                    unload_cell(e, old);
                }
            }
            if index == 0 {
                let first = interior_slot(e, this, 0);
                e.mem.set_u32(first, 0);
            } else {
                let from = interior_slot(e, this, index as u32 - 1);
                let value = e.mem.u32(from);
                let to = interior_slot(e, this, index as u32);
                e.mem.set_u32(to, value);
            }
            index -= 1;
        }
        let first = interior_slot(e, this, 0);
        e.mem.set_u32(first, cell.addr());
    }
}

// Translated from 00454d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unloads and empties every non-empty interior-buffer slot whose cell is
/// not pinned (`00557090`); when `keep_current` is set the loaded interior
/// cell (`00451... 005f36f0`) is also kept. Returns how many cells it
/// unloaded.
pub fn fn_00454d50(e: &mut Engine, this: Ptr<TES>, keep_current: u8) -> i32 {
    let mut unloaded = 0i32;
    let mut index = 0u32;
    while index < setting_uint(e, SETTING_INTERIOR_CELL_BUFFER) {
        let slot = interior_slot(e, this, index);
        let cell = e.mem.u32(slot);
        if cell != 0 {
            let mut keep = false;
            if keep_current != 0 && e.call(GET_INTERIOR_CELL, &args![this]).u32() != 0 {
                let buffer = e.get(this, TES::pInteriorBuffer).addr();
                let interior = e.call(GET_INTERIOR_CELL, &args![this]).u32();
                keep = e.mem.u32(buffer.wrapping_add(index.wrapping_mul(4))) == interior;
            }
            if !keep {
                let slot = interior_slot(e, this, index);
                let cell = e.mem.u32(slot);
                if !e.call(CELL_PINNED, &args![cell]).bool() {
                    let slot = interior_slot(e, this, index);
                    let cell = e.mem.u32(slot);
                    unload_cell(e, cell);
                    let slot = interior_slot(e, this, index);
                    e.mem.set_u32(slot, 0);
                    unloaded += 1;
                }
            }
        }
        index += 1;
    }
    unloaded
}

// Translated from 00454e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `cell` is one of the `uExterior Cell Buffer` slots of the
/// exterior cell buffer.
pub fn fn_00454e20(e: &mut Engine, this: Ptr<TES>, cell: Ptr) -> bool {
    let mut index = 0u32;
    while index < setting_uint(e, SETTING_EXTERIOR_CELL_BUFFER) {
        let slot = exterior_slot(e, this, index);
        if e.mem.u32(slot) == cell.addr() {
            return true;
        }
        index += 1;
    }
    false
}

// Translated from 00454e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Puts `cell` into the exterior cell buffer. The buffer is sorted first
/// (`TES::SortExteriorBuffer`, `00454fc0`). The cell is inserted at the front
/// of the part of the buffer after the loaded square (the first
/// `uGridsToLoad * uGridsToLoad` slots), or at slot 0 when it is pinned
/// (`00450ff0`, `004523c0`) or the slot before the end of the square is
/// empty; the cells from there on move one slot down until the old copy of
/// `cell` is overwritten, and a cell pushed off the end is unloaded
/// (`00462290`) and the grid string flag is set (`00452e50`).
pub fn fn_00454e70(e: &mut Engine, this: Ptr<TES>, cell: Ptr) {
    tes_sort_exterior_buffer(e, this);
    let mut pinned = fn_00450ff0(e, cell) || fn_004523c0(e, cell);
    let squared = grids_squared(e);
    let before_end = exterior_slot(e, this, squared.wrapping_sub(1));
    if e.mem.u32(before_end) == 0 {
        pinned = true;
    }
    let mut carry = cell.addr();
    let mut start = 0u32;
    if !pinned {
        let count = setting_uint(e, SETTING_EXTERIOR_CELL_BUFFER);
        let squared = grids_squared(e);
        if count > squared {
            start = grids_squared(e);
        }
    }
    let mut index = start;
    while index < setting_uint(e, SETTING_EXTERIOR_CELL_BUFFER) {
        let slot = exterior_slot(e, this, index);
        let old = e.mem.u32(slot);
        let slot = exterior_slot(e, this, index);
        e.mem.set_u32(slot, carry);
        if old == cell.addr() {
            return;
        }
        carry = old;
        index += 1;
    }
    if carry != 0 {
        unload_cell(e, carry);
        fn_00452e50(e, this);
    }
}

// Translated from 00454fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::SortExteriorBuffer` (Xbox PDB): when the queued grid x differs from
/// the current one, moves the cells of the exterior buffer (after the loaded
/// square) that lie within half a `uGridsToLoad` of the queued cell in x and y
/// before the ones that do not, keeping their order. (The test of the y
/// coordinate compares `iCurrentQueuedY` with itself in the game's code, so
/// only x decides.)
pub fn tes_sort_exterior_buffer(e: &mut Engine, this: Ptr<TES>) {
    let current_x = e.get(this, TES::iCurrentGridX);
    let queued_x = e.get(this, TES::iCurrentQueuedX);
    let queued_y_a = e.get(this, TES::iCurrentQueuedY);
    let queued_y_b = e.get(this, TES::iCurrentQueuedY);
    if current_x == queued_x && queued_y_a == queued_y_b {
        return;
    }
    let squared = grids_squared(e);
    let half = (setting_uint(e, SETTING_GRIDS_TO_LOAD) >> 1) as i32;
    let mut start = 0u32;
    let mut index = squared;
    while index < setting_uint(e, SETTING_EXTERIOR_CELL_BUFFER).wrapping_sub(1) {
        let slot = exterior_slot(e, this, index);
        let cell = e.mem.u32(slot);
        if cell == 0 {
            break;
        }
        let cell_x = e.call(CELL_GET_DATA_X, &args![cell]).i32();
        let cell_y = e.call(CELL_GET_DATA_Y, &args![cell]).i32();
        let delta_x = e.get(this, TES::iCurrentQueuedX).wrapping_sub(cell_x);
        let delta_y = e.get(this, TES::iCurrentQueuedY).wrapping_sub(cell_y);
        let near = delta_x >= half.wrapping_neg()
            && delta_x <= half
            && delta_y >= half.wrapping_neg()
            && delta_y <= half;
        if near && start != 0 {
            let destination = exterior_slot(e, this, start + 1);
            let source = exterior_slot(e, this, start);
            e.call(
                MEMMOVE,
                &args![destination, source, (index - start).wrapping_mul(4)],
            );
            let slot = exterior_slot(e, this, start);
            e.mem.set_u32(slot, cell);
            index = start + 1;
            start = 0;
        } else {
            start = index;
        }
        index += 1;
    }
}

// Translated from 00455130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves `cell` (null: nothing) towards the end of its run in the exterior
/// buffer: when it is found in the buffer, the following non-empty cells
/// for which `00450ff0` is true move one slot down, and `cell` takes the
/// slot after them. Stops at an empty slot before the cell.
pub fn fn_00455130(e: &mut Engine, this: Ptr<TES>, cell: Ptr) {
    if cell.is_null() {
        return;
    }
    let mut index = 0i32;
    let last = setting_uint(e, SETTING_EXTERIOR_CELL_BUFFER) as i32 - 1;
    while index < last {
        let slot = exterior_slot(e, this, index as u32);
        if cell.addr() == e.mem.u32(slot) {
            while index < last {
                let next_slot = exterior_slot(e, this, index as u32 + 1);
                let next = e.mem.u32(next_slot);
                if next == 0 || !fn_00450ff0(e, Ptr::new(next)) {
                    break;
                }
                let slot = exterior_slot(e, this, index as u32);
                e.mem.set_u32(slot, next);
                index += 1;
            }
            let slot = exterior_slot(e, this, index as u32);
            e.mem.set_u32(slot, cell.addr());
            return;
        }
        let slot = exterior_slot(e, this, index as u32);
        if e.mem.u32(slot) == 0 {
            return;
        }
        index += 1;
    }
}

// Translated from 00455200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unloads and empties every exterior-buffer slot except the cells that are
/// pinned (`00557090`), that are loaded when `loaded_only` is set
/// (`TES::IsCellLoaded` with flag 1), or that belong to another world space
/// than `world_space` (when that is non-null). Returns how many it
/// unloaded. When slot 0 is then empty and a world space is set, the terrain
/// manager is told (`006fce00`).
pub fn fn_00455200(e: &mut Engine, this: Ptr<TES>, loaded_only: u8, world_space: Ptr) -> i32 {
    let mut unloaded = 0i32;
    let mut index = 0u32;
    while index < setting_uint(e, SETTING_EXTERIOR_CELL_BUFFER) {
        let slot = exterior_slot(e, this, index);
        let cell = e.mem.u32(slot);
        if cell != 0 {
            let skip = (loaded_only != 0 && tes_is_cell_loaded(e, this, Ptr::new(cell), 1))
                || e.call(CELL_PINNED, &args![cell]).bool()
                || (!world_space.is_null()
                    && e.call(CELL_GET_WORLD_SPACE, &args![cell]).u32() != world_space.addr());
            if !skip {
                unload_cell(e, cell);
                let slot = exterior_slot(e, this, index);
                e.mem.set_u32(slot, 0);
                unloaded += 1;
            }
        }
        index += 1;
    }
    let first = exterior_slot(e, this, 0);
    if e.mem.u32(first) == 0 && e.call(GET_WORLD_SPACE, &args![this]).u32() != 0 {
        let world_space = e.call(GET_WORLD_SPACE, &args![this]).u32();
        if e.call(GET_TERRAIN_MANAGER, &args![world_space]).u32() != 0 {
            let world_space = e.call(GET_WORLD_SPACE, &args![this]).u32();
            let terrain = e.call(GET_TERRAIN_MANAGER, &args![world_space]).u32();
            e.call(0x006f_ce00, &args![terrain]);
        }
    }
    unloaded
}

// Translated from 00455330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes `cell` out of the interior cell buffer: when found, the cell is
/// unloaded (`00462290`) and its slot emptied; then the slots after the
/// search position shift down by one and the last slot is emptied (also
/// when the cell was not found, in which case nothing shifts), and the grid
/// string flag is set (`00452e50`).
pub fn fn_00455330(e: &mut Engine, this: Ptr<TES>, cell: Ptr) {
    let mut index = 0u32;
    while index < setting_uint(e, SETTING_INTERIOR_CELL_BUFFER) {
        let slot = interior_slot(e, this, index);
        if cell.addr() == e.mem.u32(slot) {
            unload_cell(e, cell.addr());
            let slot = interior_slot(e, this, index);
            e.mem.set_u32(slot, 0);
            break;
        }
        index += 1;
    }
    while index < setting_uint(e, SETTING_INTERIOR_CELL_BUFFER).wrapping_sub(1) {
        let from = interior_slot(e, this, index + 1);
        let value = e.mem.u32(from);
        let to = interior_slot(e, this, index);
        e.mem.set_u32(to, value);
        index += 1;
    }
    let count = setting_uint(e, SETTING_INTERIOR_CELL_BUFFER);
    let last = interior_slot(e, this, count.wrapping_sub(1));
    e.mem.set_u32(last, 0);
    fn_00452e50(e, this);
}

// Translated from 00455400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Chooses the sky's mode for `cell` (when the sky is built, `00476c90`):
/// 3 for no cell or an exterior cell, otherwise 2 when bit 0x80 of its
/// `cCellFlags` is set (`00454b10`) and 1 when not; `Sky::SetMode`
/// (`0063a3f0`).
pub fn fn_00455400(e: &mut Engine, this: Ptr<TES>, cell: Ptr) {
    let sky = e.call(TES_GET_SKY, &args![this]).u32();
    if e.call(SKY_GET_WORD_F8, &args![sky]).u32() == 0 {
        return;
    }
    let mode = if cell.is_null() || !e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
        3u32
    } else if fn_00454b10(e, cell) {
        2
    } else {
        1
    };
    let sky = e.call(TES_GET_SKY, &args![this]).u32();
    e.call(SKY_SET_MODE, &args![sky, mode]);
}

// Translated from 00455490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the scripts once (a byte guards against re-entry; nothing happens
/// while `Script::GetProcessScripts` is false): the quests of the data
/// handler's list that `00455620` accepts are run (`0060d4d0`); then the
/// scripts of the loaded interior cell (`TESObjectCELL::RunScripts`), or,
/// outside an interior, of the first cell of the loaded grid that
/// `00450ff0` accepts and whose scripts ran (`0054c740` true); and finally
/// the player's `RunScript` (`00565870`).
pub fn fn_00455490(e: &mut Engine, this: Ptr<TES>) {
    if e.global::<u8>(SCRIPT_PASS_RUNNING) != 0 {
        return;
    }
    if !e.call(SCRIPT_GET_PROCESS_SCRIPTS, &args![]).bool() {
        return;
    }
    e.set_global(SCRIPT_PASS_RUNNING, 1u8);
    e.call(0x0040_fbf0, &args![SCRIPT_PASS_OBJECT, 0u32]);
    let data_handler: u32 = e.global(DATA_HANDLER);
    let mut node = fn_00455600(e, Ptr::new(data_handler));
    while node != 0 {
        let item_slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let quest = e.mem.u32(item_slot);
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        if quest != 0 && fn_00455620(e, Ptr::new(quest)) {
            e.call(QUEST_RUN, &args![quest]);
        }
    }
    if e.call(GET_INTERIOR_CELL, &args![this]).u32() != 0 {
        let interior = e.call(GET_INTERIOR_CELL, &args![this]).u32();
        e.call(CELL_RUN_SCRIPTS, &args![interior, 0u32, 0u32]);
    } else {
        let mut found = false;
        let mut x = 0u32;
        while x < setting_uint(e, SETTING_GRIDS_TO_LOAD) {
            let mut y = 0u32;
            while y < setting_uint(e, SETTING_GRIDS_TO_LOAD) {
                let grid_cells = e.get(this, TES::pGridCellA);
                let slot = e.call(GRID_CELL_ARRAY_GET, &args![grid_cells, x, y]).u32();
                let cell = e.mem.u32(slot);
                if cell != 0
                    && fn_00450ff0(e, Ptr::new(cell))
                    && e.call(CELL_RUN_SCRIPTS, &args![cell, 0u32, 0u32]).bool()
                {
                    found = true;
                    break;
                }
                y += 1;
            }
            if found {
                break;
            }
            x += 1;
        }
    }
    let player: u32 = e.global(PLAYER_CHARACTER);
    e.call(REFERENCE_RUN_SCRIPT, &args![player]);
    e.call(0x0040_fba0, &args![SCRIPT_PASS_OBJECT]);
    e.set_global(SCRIPT_PASS_RUNNING, 0u8);
}

// Translated from 00455600 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address `this + 0x118` (of the data handler: the head of a list its
/// callers walk).
pub fn fn_00455600(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(0x118)
}

// Translated from 00455620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 0 of the byte `005a8080(this)` (`this + 0x3c`) points at.
pub fn fn_00455620(e: &mut Engine, this: Ptr) -> bool {
    let flags = e.call(0x005a_8080, &args![this]).u32();
    e.mem.u8(flags) & 1 != 0
}

// Translated from 00455640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the dword at `011c56e8` (`00455680`), then runs a pass over the
/// loaded interior cell (`00553820`) or, without one, over the grid
/// (`004baba0` on `pGridCellA`).
pub fn fn_00455640(e: &mut Engine, this: Ptr<TES>) {
    fn_00455680(e);
    if e.call(GET_INTERIOR_CELL, &args![this]).u32() != 0 {
        let interior = e.call(GET_INTERIOR_CELL, &args![this]).u32();
        e.call(CELL_PASS_A, &args![interior]);
    } else {
        let grid_cells = e.get(this, TES::pGridCellA);
        e.call(GRID_CELL_ARRAY_PASS_A, &args![grid_cells]);
    }
}

// Translated from 00455680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the dword at `011c56e8`.
pub fn fn_00455680(e: &mut Engine) {
    e.set_global(COUNTER_011C56E8, 0u32);
}

// Translated from 00455690 (decompiled, FalloutNV.exe 1.4.0.525)
/// A pass over the loaded interior cell (`TESObjectCELL::UpdateRefSounds`,
/// `005538e0`) or, without one, over the grid (`004bac20` on `pGridCellA`).
pub fn fn_00455690(e: &mut Engine, this: Ptr<TES>) {
    if e.call(GET_INTERIOR_CELL, &args![this]).u32() != 0 {
        let interior = e.call(GET_INTERIOR_CELL, &args![this]).u32();
        e.call(CELL_PASS_B, &args![interior]);
    } else {
        let grid_cells = e.get(this, TES::pGridCellA);
        e.call(GRID_CELL_ARRAY_PASS_B, &args![grid_cells]);
    }
}

/// `_ftol2` of a world-space bound (a `float` getter's `ST0` result) shifted
/// down to cell units.
pub(crate) fn world_space_bound_in_cells(e: &mut Engine, getter: u32, world_space: u32) -> i32 {
    let value = e.call(getter, &args![world_space]).f64();
    e.call(FTOL, &args![value]).i32() >> 12
}

/// The name text of a form (`vtable + 0x130`) compared with `"test"` on four
/// characters, as `TestAllCells` does to skip test cells and world spaces:
/// true when the name does not start with it.
pub(crate) fn test_name_differs(e: &mut Engine, form: u32) -> bool {
    let name = e.vcall(form, FORM_SLOT_NAME, &args![]).u32();
    e.call(STRNICMP, &args![name, TEST_PREFIX, 4u32]).i32() != 0
}

/// The item of a list node (`006815c0` returns the node's address; the item
/// is its first word).
pub(crate) fn list_node_item(e: &mut Engine, node: u32) -> u32 {
    let address = e.call(LIST_NODE_ITEM, &args![node]).u32();
    e.mem.u32(address)
}

/// The pointer to `"Y"` or `"N"` the test prints for the portals column.
pub(crate) fn portals_text(portals: bool) -> u32 {
    if portals {
        TEST_TEXT_YES
    } else {
        TEST_TEXT_NO
    }
}

// Translated from 004556d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::TestAllCells` (Xbox PDB): the debug run that loads every cell in
/// turn and prints a line per cell. `mode` selects what is walked: -1 stops a
/// running test (clears `bRunningCellTests`, the warning count and the region
/// filter); 0 only runs the loop below; 1, 4, 5, 9 start at the first world
/// space (4, 5 and 9 also set the "base objects checked" byte); 2 the
/// interiors only; 3 the loaded world space; 6 and 7 behave as 1 and 2 and
/// store the callback `0055 7dd0` (what the game calls "render test cell") at
/// `TES + 0x54`. A fresh start (mode other than 0) deletes the output file
/// and, once, prints the base-object model and icon checks
/// (`TESDataHandler::CheckModels`, `CheckIcons`).
///
/// Each turn (while `bRunningCellTests` is set) sends a mouse-move input so
/// the machine does not go idle, enters the next world space when asked
/// (skipping names that start with `"test"`), then takes cells (an exterior
/// cell by `TESWorldSpace::LoadCell` over the world space's cell rectangle,
/// an interior by index) until one has references and passes the region
/// filter. For that cell the player is placed (a position request,
/// `RequestPositionPlayer`), the queued loads are run, the memory is trimmed
/// and, when the logging byte is set, one line is printed with the cell
/// name, the counts (textures, lights, actors, portals) and the time it took
/// (to the debug print and to the output file); the callback at `TES + 0x54`
/// then runs, and in modes 4 and 5 `TESSaveLoadGame::TestAllCells`. The run
/// ends when the interior index reaches the interior count.
///
/// Not translated: the compiler's exception-unwinding frame.
pub fn tes_test_all_cells(e: &mut Engine, this: Ptr<TES>, mode: i32) {
    let mut mode = mode;
    if mode == 4 || mode == 5 || mode == 9 {
        e.set_global(TEST_BASE_OBJECTS_CHECKED, 1u8);
    }
    if (6..=7).contains(&mode) {
        mode -= 5;
        e.set_global(TEST_BASE_OBJECTS_CHECKED, 1u8);
        e.call(SET_TEST_CALLBACK, &args![this, TEST_CALLBACK]);
    }
    let data_handler: u32 = e.global(DATA_HANDLER);
    if mode == -1 {
        e.set(this, TES::bRunningCellTests, false);
        e.call(MESSAGE_HANDLER_INC_DISABLE_WARNING_COUNT, &args![0u32]);
        e.set(this, TES::pTACRegionFilter, Ptr::new(0));
    } else if mode != 0 {
        e.set_global(TEST_MODE, mode as u32);
        e.set(this, TES::bRunningCellTests, true);
        e.call(MESSAGE_HANDLER_INC_DISABLE_WARNING_COUNT, &args![1u32]);
        let output_file: u32 = e.global(TEST_OUTPUT_FILE);
        e.call(DELETE_FILE, &args![output_file]);
        if e.global::<u8>(TEST_BASE_OBJECTS_CHECKED) == 0 {
            e.call(DEBUG_PRINT, &args![TEST_TEXT_BASE_OBJECT_MODELS]);
            e.call(DATA_HANDLER_CHECK_MODELS, &args![data_handler, 0u32]);
            e.call(DEBUG_PRINT, &args![TEST_TEXT_BASE_OBJECT_ICONS]);
            e.call(DATA_HANDLER_CHECK_ICONS, &args![data_handler]);
            e.set_global(TEST_BASE_OBJECTS_CHECKED, 1u8);
        }
        if matches!(mode, 1 | 4 | 5 | 9)
            && e.global::<u32>(TEST_WORLD_SPACE_NODE) == 0
            && e.global::<u32>(TEST_INTERIOR_COUNT) == 0
        {
            let first = e
                .call(DATA_HANDLER_FIRST_WORLD_SPACE, &args![data_handler])
                .u32();
            e.set_global(TEST_WORLD_SPACE_NODE, first);
            e.set_global(TEST_ENTER_WORLD_SPACE, 1u8);
            start_interior_walk(e, data_handler);
        } else if mode == 3 {
            let node: u32 = e.global(TEST_WORLD_SPACE_NODE);
            let current = e.get(this, TES::pWorldSpace).addr();
            if node == 0 || list_node_item(e, node) != current {
                let first = e
                    .call(DATA_HANDLER_FIRST_WORLD_SPACE, &args![data_handler])
                    .u32();
                e.set_global(TEST_WORLD_SPACE_NODE, first);
                while e.global::<u32>(TEST_WORLD_SPACE_NODE) != 0 {
                    let node: u32 = e.global(TEST_WORLD_SPACE_NODE);
                    let world_space = list_node_item(e, node);
                    if world_space != 0 && world_space == e.get(this, TES::pWorldSpace).addr() {
                        break;
                    }
                    let node: u32 = e.global(TEST_WORLD_SPACE_NODE);
                    let next = e.call(LIST_NODE_NEXT, &args![node]).u32();
                    e.set_global(TEST_WORLD_SPACE_NODE, next);
                }
            }
            if e.global::<u32>(TEST_WORLD_SPACE_NODE) != 0 {
                e.set_global(TEST_ENTER_WORLD_SPACE, 1u8);
                start_interior_walk(e, data_handler);
            }
        } else if mode == 2 {
            if e.global::<u32>(TEST_WORLD_SPACE_NODE) != 0 {
                e.set_global(TEST_WORLD_SPACE_NODE, 0u32);
                e.set_global(TEST_ENTER_WORLD_SPACE, 0u8);
            }
            if e.global::<u32>(TEST_INTERIOR_COUNT) == 0 {
                start_interior_walk(e, data_handler);
            }
        }
    }
    let mut again = true;
    while again {
        again = false;
        if !e.get(this, TES::bRunningCellTests) {
            continue;
        }
        if mode == 9 {
            again = true;
        }
        let mut cell = 0u32;
        let started = e.call(CRT_CLOCK, &args![]).u32();
        // The mouse move that keeps the machine awake: an INPUT of type 0
        // (mouse) with dwFlags 1 (MOUSEEVENTF_MOVE) and dx 2 or -1.
        let sign: i32 = e.global(TEST_INPUT_SIGN);
        e.with_stack(0x1c, |e, input| {
            let delta_x: i32 = if sign > 0 { 2 } else { -1 };
            e.mem.set_i32(input.addr() + 4, delta_x);
            e.mem.set_u32(input.addr() + 0x10, 1);
            e.call(IMPORT_SEND_INPUT, &args![1u32, input.addr(), 0x1cu32]);
        });
        let sign: i32 = e.global(TEST_INPUT_SIGN);
        e.set_global(TEST_INPUT_SIGN, sign.wrapping_neg());
        if e.global::<u8>(TEST_ENTER_WORLD_SPACE) != 0 {
            if e.global::<u32>(TEST_WORLD_SPACE_NODE) != 0 {
                let node: u32 = e.global(TEST_WORLD_SPACE_NODE);
                let mut world_space = list_node_item(e, node);
                while world_space != 0 {
                    if !test_name_differs(e, world_space) {
                        let node: u32 = e.global(TEST_WORLD_SPACE_NODE);
                        let next = e.call(LIST_NODE_NEXT, &args![node]).u32();
                        e.set_global(TEST_WORLD_SPACE_NODE, next);
                        world_space = if next != 0 {
                            list_node_item(e, next)
                        } else {
                            0
                        };
                    } else {
                        break;
                    }
                }
                if world_space != 0 {
                    e.call(SET_WORLD_SPACE, &args![this, world_space]);
                    let current = e.get(this, TES::pWorldSpace).addr();
                    let low_x = world_space_bound_in_cells(e, WORLD_SPACE_MIN_X, current);
                    e.set_global(TEST_CELL_X, low_x);
                    let current = e.get(this, TES::pWorldSpace).addr();
                    let low_y = world_space_bound_in_cells(e, WORLD_SPACE_MIN_Y, current);
                    e.set_global(TEST_CELL_Y, low_y);
                    let current = e.get(this, TES::pWorldSpace).addr();
                    let high_x = world_space_bound_in_cells(e, WORLD_SPACE_MAX_X, current);
                    let current = e.get(this, TES::pWorldSpace).addr();
                    let high_y = world_space_bound_in_cells(e, WORLD_SPACE_MAX_Y, current);
                    let current = e.get(this, TES::pWorldSpace).addr();
                    let name = e.vcall(current, FORM_SLOT_NAME, &args![]).u32();
                    e.call(
                        DEBUG_PRINT,
                        &args![TEST_TEXT_WORLD_SPACE, name, low_x, low_y, high_x, high_y],
                    );
                }
            }
            e.set_global(TEST_ENTER_WORLD_SPACE, 0u8);
        }
        let mut found = false;
        let mut finished = false;
        while !found && e.global::<u8>(TEST_ENTER_WORLD_SPACE) == 0 && !finished {
            if e.global::<u32>(TEST_WORLD_SPACE_NODE) != 0 {
                let world_space = e.get(this, TES::pWorldSpace).addr();
                let (x, y): (i32, i32) = (e.global(TEST_CELL_X), e.global(TEST_CELL_Y));
                cell = e
                    .call(WORLD_SPACE_LOAD_CELL, &args![world_space, x, y])
                    .u32();
                if cell != 0 {
                    fn_00454e70(e, this, Ptr::new(cell));
                }
                let next_y = e.global::<i32>(TEST_CELL_Y).wrapping_add(1);
                e.set_global(TEST_CELL_Y, next_y);
                let current = e.get(this, TES::pWorldSpace).addr();
                if next_y > world_space_bound_in_cells(e, WORLD_SPACE_MAX_Y, current) {
                    let current = e.get(this, TES::pWorldSpace).addr();
                    let low_y = world_space_bound_in_cells(e, WORLD_SPACE_MIN_Y, current);
                    e.set_global(TEST_CELL_Y, low_y);
                    let next_x = e.global::<i32>(TEST_CELL_X).wrapping_add(1);
                    e.set_global(TEST_CELL_X, next_x);
                    let current = e.get(this, TES::pWorldSpace).addr();
                    if next_x > world_space_bound_in_cells(e, WORLD_SPACE_MAX_X, current) {
                        let node: u32 = e.global(TEST_WORLD_SPACE_NODE);
                        let next = e.call(LIST_NODE_NEXT, &args![node]).u32();
                        e.set_global(TEST_WORLD_SPACE_NODE, next);
                        if next != 0 {
                            e.set_global(TEST_ENTER_WORLD_SPACE, 1u8);
                        }
                    }
                }
            } else {
                if e.global::<u32>(TEST_INTERIOR_INDEX) == 0 {
                    e.call(DEBUG_PRINT, &args![TEST_TEXT_INTERIORS]);
                }
                let index: u32 = e.global(TEST_INTERIOR_INDEX);
                cell = e
                    .call(DATA_HANDLER_GET_INTERIOR, &args![data_handler, index])
                    .u32();
                e.set_global(TEST_INTERIOR_INDEX, index.wrapping_add(1));
                while cell != 0 {
                    if test_name_differs(e, cell) {
                        break;
                    }
                    let index: u32 = e.global(TEST_INTERIOR_INDEX);
                    cell = e
                        .call(DATA_HANDLER_GET_INTERIOR, &args![data_handler, index])
                        .u32();
                    e.set_global(TEST_INTERIOR_INDEX, index.wrapping_add(1));
                }
                if cell == 0 {
                    finished = true;
                }
            }
            if cell != 0
                && !e.call(CELL_HAS_FLAG_20, &args![cell]).bool()
                && e.call(CELL_GET_FIRST_REFR, &args![cell]).u32() != 0
            {
                found = true;
            }
        }
        if cell != 0 && found {
            let regions = e.call(CELL_GET_REGION_LIST, &args![cell, 1u32]).u32();
            if !e.get(this, TES::pTACRegionFilter).is_null()
                && (regions == 0
                    || !e
                        .call(
                            REGION_LIST_CONTAINS,
                            &args![regions + 4, this.addr() + TES::pTACRegionFilter.off],
                        )
                        .bool())
            {
                found = false;
            }
        } else if e.global::<u8>(TEST_ENTER_WORLD_SPACE) != 0 {
            let player: u32 = e.global(PLAYER_CHARACTER);
            let player_world_space = e.call(REFERENCE_GET_WORLD_SPACE, &args![player]).u32();
            if player_world_space != e.get(this, TES::pWorldSpace).addr() {
                again = true;
            }
        }
        if found {
            test_visit_cell(e, this, cell, started);
        }
        if e.global::<u32>(TEST_INTERIOR_INDEX) >= e.global::<u32>(TEST_INTERIOR_COUNT) {
            e.set_global(TEST_INTERIOR_COUNT, 0u32);
            e.set(this, TES::bRunningCellTests, false);
            e.call(MESSAGE_HANDLER_INC_DISABLE_WARNING_COUNT, &args![0u32]);
            e.set(this, TES::pTACRegionFilter, Ptr::new(0));
        }
    }
}

/// What `TestAllCells` does to begin walking the interior cells: the data
/// handler prepares its interior list (`004619b0`), the count comes from
/// `00461960`, and the index restarts at 0.
pub(crate) fn start_interior_walk(e: &mut Engine, data_handler: u32) {
    e.call(DATA_HANDLER_PREPARE_INTERIORS, &args![data_handler]);
    let count = e
        .call(DATA_HANDLER_INTERIOR_COUNT, &args![data_handler])
        .u32();
    e.set_global(TEST_INTERIOR_COUNT, count);
    e.set_global(TEST_INTERIOR_INDEX, 0u32);
}

/// The part of `TestAllCells` that runs for one chosen `cell`: places the
/// player at it, loads, measures and prints (see [`tes_test_all_cells`]);
/// `started` is the `clock()` value taken at the beginning of the turn.
pub(crate) fn test_visit_cell(e: &mut Engine, this: Ptr<TES>, cell: u32, started: u32) {
    let ten: f32 = e.global(TEST_FLOAT_TEN);
    e.call(TIME_OBJECT_ADVANCE, &args![OBJECT_011DE7B8, ten]);
    let position = e.mem.alloc(12);
    for word in 0..3u32 {
        let value: u32 = e.global(NI_POINT3_ZERO + 4 * word);
        e.mem.set_u32(position + 4 * word, value);
    }
    let interior = e.call(CELL_IS_INTERIOR, &args![cell]).bool();
    if !interior {
        let half_cell: f64 = e.global(TEST_DOUBLE_HALF_CELL);
        let y_corner = e.call(CELL_GET_DATA_Y, &args![cell]).i32().wrapping_shl(12);
        let y = (y_corner as f64 + half_cell) as f32;
        let x_corner = e.call(CELL_GET_DATA_X, &args![cell]).i32().wrapping_shl(12);
        let x = (x_corner as f64 + half_cell) as f32;
        let scratch = e.mem.alloc(12);
        let made = e
            .call(NI_POINT3_CONSTRUCT_FLOATS, &args![scratch, x, y, 0.0f32])
            .u32();
        for word in 0..3u32 {
            let value = e.mem.u32(made + 4 * word);
            e.mem.set_u32(position + 4 * word, value);
        }
        e.mem.free(scratch);
    }
    if interior {
        e.call(CELL_LOAD_ALL_TEMP_DATA, &args![cell]);
        let rotation = e.mem.alloc(12);
        e.call(LIST_NODE_ITEM, &args![rotation]);
        e.call(
            CELL_GET_COC_PLACEMENT_INFO,
            &args![cell, position, rotation],
        );
        fn_00453dc0(e, this, Ptr::new(cell), Ptr::new(position));
        e.mem.free(rotation);
    } else {
        let height = e.mem.alloc(4);
        if e.call(GET_INTERIOR_CELL, &args![this]).u32() != 0 {
            fn_00454450(e, this, Ptr::new(position));
        } else {
            tes_update_current_grid_cell(e, this, Ptr::new(position), 0);
        }
        let land = e.call(CELL_GET_LAND, &args![cell]).u32();
        e.call(LAND_GET_LAND_HEIGHT, &args![land, position, height]);
        let value = e.mem.f32(height);
        let zero: f64 = e.global(TEST_DOUBLE_ZERO);
        if value as f64 >= zero {
            e.mem.set_f32(position + 8, value);
        }
        e.mem.free(height);
    }
    let size = 0x34u32;
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    let request = if block != 0 {
        e.call(POSITION_REQUEST_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    for word in 0..3u32 {
        let value = e.mem.u32(position + 4 * word);
        e.mem.set_u32(request + 8 + 4 * word, value);
    }
    let player: u32 = e.global(PLAYER_CHARACTER);
    let rotation = e.call(PLAYER_ROTATION_GETTER, &args![player]).u32();
    for word in 0..3u32 {
        let value = e.mem.u32(rotation + 4 * word);
        e.mem.set_u32(request + 0x14 + 4 * word, value);
    }
    e.mem.set_u8(request + 0x20, 1);
    if e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
        e.mem.set_u32(request + 4, cell);
    } else {
        let world_space = e.call(CELL_GET_WORLD_SPACE, &args![cell]).u32();
        e.mem.set_u32(request, world_space);
    }
    let player: u32 = e.global(PLAYER_CHARACTER);
    e.call(PLAYER_REQUEST_POSITION, &args![player, request]);
    let player: u32 = e.global(PLAYER_CHARACTER);
    e.call(PLAYER_HANDLE_POSITION_REQUEST, &args![player]);
    let io_manager: u32 = e.global(IO_MANAGER);
    e.call(IO_MANAGER_LOAD_QUEUED_PRIORITY, &args![io_manager]);
    let finished = e.call(CRT_CLOCK, &args![]).u32();
    let thousand: f64 = e.global(TEST_DOUBLE_THOUSAND);
    let elapsed = (finished.wrapping_sub(started) as i32 as f64 / thousand) as f32;
    let scope = e.mem.alloc(12);
    e.call(FRAME_SCOPE_BEGIN, &args![scope, 1u32, 1u32, 1u32]);
    let tes_singleton: u32 = e.global(TES_SINGLETON);
    e.call(TES_PURGE_UNUSED, &args![tes_singleton, 1u32, 0u32]);
    let released = e.mem.u8(scope + 5);
    e.call(FRAME_SCOPE_RELEASE, &args![released]);
    e.call(FRAME_SCOPE_END, &args![scope]);
    e.mem.free(scope);
    if e.global::<u8>(TEST_LOGGING) != 0 {
        test_print_cell(e, cell, elapsed);
    }
    let callback = e.get(this, TES::pfnTACCallbackFunc).addr();
    if callback != 0 {
        let data = e.get(this, TES::pTACCallbackData).addr();
        e.call(callback, &args![cell, data]);
    }
    let mode: u32 = e.global(TEST_MODE);
    if mode == 4 || mode == 5 {
        let save_load: u32 = e.global(SAVE_LOAD_GAME);
        let mode: u32 = e.global(TEST_MODE);
        e.call(SAVE_LOAD_GAME_TEST_ALL_CELLS, &args![save_load, mode]);
    }
    e.mem.free(position);
}

/// The logging block of `TestAllCells`: the scene totals, the counts, and
/// the line printed to the debug output and appended to the output file
/// (an interior cell and an exterior cell have different lines).
pub(crate) fn test_print_cell(e: &mut Engine, cell: u32, elapsed: f32) {
    let total_time = e.mem.alloc(4);
    let second_value = e.mem.alloc(4);
    let m_number = e.mem.alloc(12);
    e.mem.set_f32(total_time, 0.0);
    e.mem.set_u32(second_value, 0);
    e.call(STRING_ARRAY_CONSTRUCT, &args![m_number]);
    e.call(
        SCENE_INFO_TOTALS,
        &args![total_time, second_value, 1u32, 1u32],
    );
    let second_word = e.mem.u32(second_value);
    e.call(
        SCENE_INFO_BUILD_M_NUMBER,
        &args![second_word, m_number, 0u32],
    );
    let mut portals = false;
    let holder = e.call(CELL_GET_ACTOR_HOLDER, &args![cell]).u32();
    if holder != 0 {
        let field = e.call(ACTOR_HOLDER_GET_FIELD_20, &args![holder]).u32();
        if !e.call(FIELD_IS_EMPTY, &args![field]).bool() {
            portals = true;
        }
    }
    let mut texture_total = 0u32;
    let mut texture = e.call(TEXTURE_LIST_HEAD, &args![]).u32();
    while texture != 0 {
        if e.call(TEXTURE_IS_LOADED, &args![texture]).u32() != 0 {
            let size = e.call(DEBUG_PRINT, &args![texture]).u32();
            texture_total = texture_total.wrapping_add(size);
        }
        texture = e.call(TEXTURE_LIST_NEXT, &args![texture]).u32();
    }
    let texture_megabytes = texture_total >> 20;
    let actors = e.call(ACTOR_COUNT, &args![OBJECT_011E0E80]).u32();
    let total_time_value = e.mem.f32(total_time) as f64;
    let elapsed = elapsed as f64;
    if e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
        let lights = e.call(CELL_GET_LIGHT_COUNT, &args![cell]).u32();
        let m_number_text = e.call(NI_POINTER_GET, &args![m_number]).u32();
        let name = e.vcall(cell, FORM_SLOT_NAME, &args![]).u32();
        e.call(
            DEBUG_PRINT,
            &args![
                TEST_TEXT_INTERIOR_LINE,
                name,
                m_number_text,
                total_time_value,
                elapsed,
                lights,
                portals_text(portals),
                texture_megabytes,
                actors
            ],
        );
        let lights = e.call(CELL_GET_LIGHT_COUNT, &args![cell]).u32();
        let m_number_text = e.call(NI_POINTER_GET, &args![m_number]).u32();
        let name = e.vcall(cell, FORM_SLOT_NAME, &args![]).u32();
        let output_file: u32 = e.global(TEST_OUTPUT_FILE);
        e.call(
            MESSAGE_HANDLER_OUTPUT,
            &args![
                output_file,
                TEST_TEXT_INTERIOR_ROW,
                name,
                m_number_text,
                total_time_value,
                elapsed,
                lights,
                portals_text(portals),
                texture_megabytes,
                actors
            ],
        );
    } else {
        let count = e.global::<u32>(TEST_EXTERIOR_COUNT).wrapping_add(1);
        e.set_global(TEST_EXTERIOR_COUNT, count);
        for to_file in [false, true] {
            let m_number_text = e.call(NI_POINTER_GET, &args![m_number]).u32();
            let y = e.call(CELL_GET_DATA_Y, &args![cell]).u32();
            let x = e.call(CELL_GET_DATA_X, &args![cell]).u32();
            let name = e.vcall(cell, FORM_SLOT_NAME, &args![]).u32();
            if !to_file {
                e.call(
                    DEBUG_PRINT,
                    &args![
                        TEST_TEXT_EXTERIOR_LINE,
                        name,
                        x,
                        y,
                        m_number_text,
                        total_time_value,
                        elapsed,
                        portals_text(portals),
                        texture_megabytes,
                        actors
                    ],
                );
            } else {
                let output_file: u32 = e.global(TEST_OUTPUT_FILE);
                e.call(
                    MESSAGE_HANDLER_OUTPUT,
                    &args![
                        output_file,
                        TEST_TEXT_EXTERIOR_ROW,
                        name,
                        x,
                        y,
                        m_number_text,
                        total_time_value,
                        elapsed,
                        portals_text(portals),
                        texture_megabytes,
                        actors
                    ],
                );
            }
        }
    }
    e.call(STRING_ARRAY_DESTRUCT, &args![m_number]);
    e.mem.free(m_number);
    e.mem.free(second_value);
    e.mem.free(total_time);
}

// Translated from 00456670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::GetFaceCount` (Xbox PDB): the number of faces under `node`. Each
/// child is held in a `NiPointer` for the visit; a child whose vtable slot
/// `0xc` answers (a node) is counted recursively; otherwise one whose slot
/// `0x1c` answers (a geometry object) adds the 16-bit count at `+0x40` of its
/// data (`004567b0`), unless `all` is zero and `00456610` says to leave it out.
///
/// Not translated: the compiler's exception-unwinding frame.
pub fn tes_get_face_count(e: &mut Engine, _this: Ptr<TES>, node: Ptr, all: u8) -> i32 {
    if node.is_null() {
        return 0;
    }
    let mut total = 0i32;
    let mut index = 0u32;
    while index < e.call(0x0043_b480, &args![node]).u32() {
        let child = e.call(0x0043_b4a0, &args![node, index]).u32();
        e.with_stack(4, |e, holder| {
            e.call(NI_POINTER_CTOR, &args![holder, child]);
            if e.call(NI_POINTER_GET, &args![holder]).u32() != 0 {
                let object = e.call(NI_POINTER_GET, &args![holder]).u32();
                if e.vcall(object, 0xc, &args![]).u32() != 0 {
                    let inner = e.call(NI_POINTER_GET, &args![holder]);
                    let faces = tes_get_face_count(e, _this, Ptr::new(inner.u32()), all);
                    total = total.wrapping_add(faces);
                } else {
                    let object = e.call(NI_POINTER_GET, &args![holder]).u32();
                    if e.vcall(object, 0x1c, &args![]).u32() != 0 {
                        let geometry = e.call(NI_POINTER_GET, &args![holder]).u32();
                        if all != 0 || !e.call(0x0045_6610, &args![geometry]).bool() {
                            let faces = fn_004567b0(e, Ptr::new(geometry));
                            total = total.wrapping_add(faces as i32);
                        }
                    }
                }
            }
            e.call(NI_POINTER_DTOR, &args![holder]);
        });
        index += 1;
    }
    total
}

// Translated from 004567b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 16-bit count at `+0x40` of the object the `NiPointer` at `this + 0xb8`
/// holds (`004567e0`).
pub fn fn_004567b0(e: &mut Engine, this: Ptr) -> u16 {
    let data = e.call(NI_POINTER_GET, &args![this.addr() + 0xb8]).u32();
    fn_004567e0(e, Ptr::new(data))
}

// Translated from 004567e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 16-bit word at `this + 0x40`.
pub fn fn_004567e0(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + 0x40)
}

// Translated from 00456800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::RemoveTextureImage` (Xbox PDB): looks the texture `name` up in the
/// texture palette (`BSTexturePalette::GetTexture`, `00a61b90`) and removes it
/// (`BSTexturePalette::RemoveTexture`, `00a61f30`) when only the palette and
/// at most `extra_references` other holders keep it (the count at `+4` of
/// the texture, `00726070`, is at most `extra_references + 2`). True when
/// removed. (`this` is not used.)
///
/// Not translated: the compiler's exception-unwinding frame.
pub fn tes_remove_texture_image(
    e: &mut Engine,
    _this: Ptr<TES>,
    name: Ptr,
    extra_references: u32,
) -> bool {
    e.with_stack(4, |e, holder| {
        e.call(NI_POINTER_CTOR, &args![holder, 0u32]);
        e.call(0x00a6_1b90, &args![name, holder]);
        let mut removed = false;
        if e.call(NI_POINTER_GET, &args![holder]).u32() != 0 {
            let texture = e.call(NI_POINTER_GET, &args![holder]).u32();
            let count = e.call(LIST_NODE_NEXT, &args![texture]).u32();
            if count <= extra_references.wrapping_add(2) {
                let texture = e.call(NI_POINTER_GET, &args![holder]).u32();
                e.call(0x00a6_1f30, &args![texture]);
                removed = true;
            }
        }
        e.call(NI_POINTER_DTOR, &args![holder]);
        removed
    })
}

// Translated from 004568c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::CreateTextureImage` (Xbox PDB): makes the `NiPointer` at `out` hold
/// the texture file `name`. The palette is asked first (`00a61b90`); a
/// texture already there is the answer. Otherwise, if the file exists
/// (`FileFinder::Exist`, `00456a20`; `alternate` selects a different
/// search mode, `(6, 2)` instead of `(0, 0xffff)`) or `quiet` is zero, a
/// source texture is created (`NiSourceTexture::Create`) from the name
/// through the texture format preferences at `011a9598`, stored in `out`
/// and entered in the palette (`BSTexturePalette::SetTexture`, `00a61c50`).
/// When the texture still could not be made and `quiet` is zero, the failure
/// is printed. (`this` is not used.)
///
/// Not translated: the compiler's exception-unwinding frame.
pub fn tes_create_texture_image(
    e: &mut Engine,
    _this: Ptr<TES>,
    name: Ptr,
    out: Ptr,
    quiet: u8,
    alternate: u8,
) {
    let mut mode = 0u32;
    let mut flags = 0xffffu32;
    e.call(0x0066_b0d0, &args![out, 0u32]);
    e.call(0x00a6_1b90, &args![name, out]);
    if e.call(NI_POINTER_GET, &args![out]).u32() != 0 {
        return;
    }
    e.with_stack(4, |e, holder| {
        e.call(NI_POINTER_CTOR, &args![holder, 0u32]);
        if alternate != 0 {
            mode = 6;
            flags = 2;
        }
        let found = e.call(0x0045_6a20, &args![name, 0u32, mode, flags]).u32();
        if found == 0 && quiet != 0 {
            e.call(NI_POINTER_DTOR, &args![holder]);
            return;
        }
        e.with_stack(4, |e, fixed_string| {
            let string = e.call(0x0043_8170, &args![fixed_string, name]).u32();
            let texture = e
                .call(0x00a5_fd70, &args![string, 0x011a_9598u32, 1u32, 0u32])
                .u32();
            e.call(0x0066_b0d0, &args![out, texture]);
            e.call(0x0043_81b0, &args![fixed_string]);
        });
        if e.call(NI_POINTER_GET, &args![out]).u32() != 0 {
            let texture = e.call(NI_POINTER_GET, &args![out]).u32();
            e.call(0x00a6_1c50, &args![texture, 0u32]);
        } else if quiet == 0 {
            e.call(DEBUG_PRINT, &args![0x0101_7ca8u32, name]);
        }
        e.call(NI_POINTER_DTOR, &args![holder]);
    });
}

// Translated from 00457070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TES::GetCurrentCell` (Xbox PDB): the loaded interior cell; without one,
/// the centre cell of the grid (`00451900` at `iCurrentGridX`,
/// `iCurrentGridY`), or null while either is `0x7fffffff` (none).
pub fn tes_get_current_cell(e: &mut Engine, this: Ptr<TES>) -> u32 {
    if e.call(GET_INTERIOR_CELL, &args![this]).u32() != 0 {
        return e.call(GET_INTERIOR_CELL, &args![this]).u32();
    }
    if e.get(this, TES::iCurrentGridX) == 0x7fff_ffff
        || e.get(this, TES::iCurrentGridY) == 0x7fff_ffff
    {
        return 0;
    }
    let x = e.get(this, TES::iCurrentGridX);
    let y = e.get(this, TES::iCurrentGridY);
    fn_00451900(e, this, x, y)
}

// Translated from 004570d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Asks the cells of the loaded area (`00553bf0(cell, a, b, c, d)`): the
/// interior cell alone (taken from the `TES` singleton, not from `this`), or,
/// without one, every non-empty cell of the `uGridsToLoad` square, all of
/// them. True when any answered true. False at once for a null `target`.
pub fn fn_004570d0(
    e: &mut Engine,
    this: Ptr<TES>,
    target: Ptr,
    second: u32,
    third: u32,
    fourth: f32,
) -> bool {
    let mut result = false;
    if target.is_null() {
        return false;
    }
    let tes_singleton: u32 = e.global(TES_SINGLETON);
    if e.call(GET_INTERIOR_CELL, &args![tes_singleton]).u32() != 0 {
        let interior = e.call(GET_INTERIOR_CELL, &args![tes_singleton]).u32();
        return e
            .call(CELL_TEST_A, &args![interior, target, second, third, fourth])
            .bool();
    }
    let mut x = 0u32;
    while x < setting_uint(e, SETTING_GRIDS_TO_LOAD) {
        let mut y = 0u32;
        while y < setting_uint(e, SETTING_GRIDS_TO_LOAD) {
            let grid_cells = e.get(this, TES::pGridCellA);
            let slot = e.call(GRID_CELL_ARRAY_GET, &args![grid_cells, x, y]).u32();
            let cell = e.mem.u32(slot);
            if cell != 0
                && e.call(CELL_TEST_A, &args![cell, target, second, third, fourth])
                    .bool()
            {
                result = true;
            }
            y += 1;
        }
        x += 1;
    }
    result
}

// Translated from 004571c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `004570d0` with one more byte argument and `00553d00` on the cells,
/// after a preparation of `target`: when `target` is non-null and `flag` is
/// non-zero, `00705fc0(target, 0)` and `NiPick::PickObjects`
/// (`00e98e20`, with the pick object at `011f426c` twice and 0) run on it.
/// The `TES` singleton's interior cell is asked alone, else every non-empty
/// cell of the grid; true when any answered true.
pub fn fn_004571c0(
    e: &mut Engine,
    this: Ptr<TES>,
    target: Ptr,
    second: u32,
    third: u32,
    fourth: f32,
    flag: u8,
) -> bool {
    let mut result = false;
    if target.is_null() || flag == 0 {
        return false;
    }
    e.call(0x0070_5fc0, &args![target, 0u32]);
    e.call(
        0x00e9_8e20,
        &args![target, NI_POINT3_ZERO, NI_POINT3_ZERO, 0u32],
    );
    let tes_singleton: u32 = e.global(TES_SINGLETON);
    if e.call(GET_INTERIOR_CELL, &args![tes_singleton]).u32() != 0 {
        let interior = e.call(GET_INTERIOR_CELL, &args![tes_singleton]).u32();
        return e
            .call(
                CELL_TEST_B,
                &args![interior, target, second, third, fourth, flag],
            )
            .bool();
    }
    let mut x = 0u32;
    while x < setting_uint(e, SETTING_GRIDS_TO_LOAD) {
        let mut y = 0u32;
        while y < setting_uint(e, SETTING_GRIDS_TO_LOAD) {
            let grid_cells = e.get(this, TES::pGridCellA);
            let slot = e.call(GRID_CELL_ARRAY_GET, &args![grid_cells, x, y]).u32();
            let cell = e.mem.u32(slot);
            if cell != 0
                && e.call(
                    CELL_TEST_B,
                    &args![cell, target, second, third, fourth, flag],
                )
                .bool()
            {
                result = true;
            }
            y += 1;
        }
        x += 1;
    }
    result
}

/// The rounding fix of the position code: `rounded` (from `00406d90`) is
/// moved one down when it is a multiple of 4096 and the exact `value` is
/// below it (so a position on the border of a cell belongs to the cell below).
pub(crate) fn adjust_rounded_position(rounded: i32, value: f32) -> i32 {
    let mut remainder = (rounded as u32) & 0x8000_0fff;
    if (remainder as i32) < 0 {
        remainder = (remainder.wrapping_sub(1) | 0xffff_f000).wrapping_add(1);
    }
    if remainder == 0 && (value as f64) < rounded as f64 {
        // 0045733a/0045736c: `SUB ECX,1` wraps (the integer indefinite 0x80000000 of an
        // out-of-range value becomes 0x7fffffff).
        rounded.wrapping_sub(1)
    } else {
        rounded
    }
}

/// The adjusted x and y of the position `pos` (two floats), both rounded
/// before either is adjusted.
pub(crate) fn rounded_position(e: &mut Engine, pos: Ptr) -> (i32, i32) {
    let x_value = e.mem.f32(pos.addr());
    let rounded_x = e.call(FLOAT_TO_INT_ROUNDED, &args![x_value]).i32();
    let y_value = e.mem.f32(pos.addr() + 4);
    let rounded_y = e.call(FLOAT_TO_INT_ROUNDED, &args![y_value]).i32();
    let x_value = e.mem.f32(pos.addr());
    let y_value = e.mem.f32(pos.addr() + 4);
    (
        adjust_rounded_position(rounded_x, x_value),
        adjust_rounded_position(rounded_y, y_value),
    )
}

/// The loaded cell under the position `pos` (two floats), or 0: the cell at
/// the adjusted rounded coordinates divided by 4096 (`00451900`), when
/// `TES::IsCellLoaded` (on the `TES` singleton, flag 0) says it is loaded.
pub(crate) fn loaded_cell_at(e: &mut Engine, this: Ptr<TES>, pos: Ptr) -> u32 {
    let (x, y) = rounded_position(e, pos);
    let cell = fn_00451900(e, this, x >> 12, y >> 12);
    let tes_singleton: u32 = e.global(TES_SINGLETON);
    if cell != 0 && tes_is_cell_loaded(e, Ptr::new(tes_singleton), Ptr::new(cell), 0) {
        cell
    } else {
        0
    }
}

// Translated from 004572e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The land height at `pos` (two floats): `*out` is first set to the "no
/// height" float at `01017824`; the cell under `pos` must be loaded
/// (`TES::IsCellLoaded`), and `TESObjectLAND::GetLandHeight`
/// (`0053f180`) of its land (`TESObjectCELL::GetLand`) answers in `*out`
/// and as the byte result.
pub fn fn_004572e0(e: &mut Engine, this: Ptr<TES>, pos: Ptr, out: Ptr) -> bool {
    let (x, y) = rounded_position(e, pos);
    let no_height: f32 = e.global(0x0101_7824);
    e.mem.set_f32(out.addr(), no_height);
    let cell = fn_00451900(e, this, x >> 12, y >> 12);
    let tes_singleton: u32 = e.global(TES_SINGLETON);
    if cell != 0 && tes_is_cell_loaded(e, Ptr::new(tes_singleton), Ptr::new(cell), 0) {
        let land = e.call(CELL_GET_LAND, &args![cell]).u32();
        e.call(LAND_GET_LAND_HEIGHT, &args![land, pos, out]).bool()
    } else {
        false
    }
}

// Translated from 004573f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `004572e0` for another `TESObjectLAND` query (`0053d2e0(land, pos, out,
/// third)`): `out` (a `NiPoint3`) is first set to the three words at
/// `011f426c` with z = 1.0, and the answer is that query's byte (false when
/// the cell under `pos` is not loaded).
pub fn fn_004573f0(e: &mut Engine, this: Ptr<TES>, pos: Ptr, out: Ptr, third: u32) -> bool {
    let (x, y) = rounded_position(e, pos);
    for word in 0..3u32 {
        let value: u32 = e.global(NI_POINT3_ZERO + 4 * word);
        e.mem.set_u32(out.addr() + 4 * word, value);
    }
    e.mem.set_f32(out.addr() + 8, 1.0);
    let cell = fn_00451900(e, this, x >> 12, y >> 12);
    let tes_singleton: u32 = e.global(TES_SINGLETON);
    if cell != 0 && tes_is_cell_loaded(e, Ptr::new(tes_singleton), Ptr::new(cell), 0) {
        let land = e.call(CELL_GET_LAND, &args![cell]).u32();
        e.call(0x0053_d2e0, &args![land, pos, out, third]).bool()
    } else {
        false
    }
}

// Translated from 00457520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `004572e0` for a `TESObjectLAND` query with one output
/// (`0053f570(land, pos, out)`); false when the cell under `pos` is not
/// loaded.
pub fn fn_00457520(e: &mut Engine, this: Ptr<TES>, pos: Ptr, third: u32) -> bool {
    let cell = loaded_cell_at(e, this, pos);
    if cell != 0 {
        let land = e.call(CELL_GET_LAND, &args![cell]).u32();
        e.call(0x0053_f570, &args![land, pos, third]).bool()
    } else {
        false
    }
}

// Translated from 00457620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `004572e0` for a `TESObjectLAND` query (`0053f0e0(land, pos)`) whose
/// whole `eax` answer is returned; 0 when the cell under `pos` is not loaded.
pub fn fn_00457620(e: &mut Engine, this: Ptr<TES>, pos: Ptr) -> u32 {
    let cell = loaded_cell_at(e, this, pos);
    if cell != 0 {
        let land = e.call(CELL_GET_LAND, &args![cell]).u32();
        e.call(0x0053_f0e0, &args![land, pos]).u32()
    } else {
        0
    }
}

// Translated from 00457720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The land query `0053a5e0` at `pos` (two floats), cached by the x of its
/// 128-unit column: when the adjusted x rounded down to a multiple of 128
/// equals the cached x at `0118608c`, the cached answer at `011c3ef8` is
/// returned (the y is not compared); otherwise the cell under `pos`
/// (`00451900`) is asked (`0053a5e0(land, pos)`) and a non-zero answer is
/// cached with the x and y of the 128-unit column (`0118608c`, `01186088`).
pub fn fn_00457720(e: &mut Engine, this: Ptr<TES>, pos: Ptr) -> u32 {
    let mut result = 0u32;
    let (x, y) = rounded_position(e, pos);
    let column_x = rounded_down_to_128(x);
    let column_y = rounded_down_to_128(y);
    if column_x == e.global::<i32>(CACHED_LAND_COLUMN_X) {
        result = e.global(CACHED_LAND_ANSWER);
    } else {
        let cell = fn_00451900(e, this, x >> 12, y >> 12);
        if cell != 0 {
            let land = e.call(CELL_GET_LAND, &args![cell]).u32();
            result = e.call(0x0053_a5e0, &args![land, pos]).u32();
            if result != 0 {
                e.set_global(CACHED_LAND_ANSWER, result);
                e.set_global(CACHED_LAND_COLUMN_X, column_x);
                e.set_global(CACHED_LAND_COLUMN_Y, column_y);
            }
        }
    }
    result
}

/// `value` minus its remainder by 128 (the remainder has the sign of the
/// value).
pub(crate) fn rounded_down_to_128(value: i32) -> i32 {
    let mut remainder = (value as u32) & 0x8000_007f;
    if (remainder as i32) < 0 {
        remainder = (remainder.wrapping_sub(1) | 0xffff_ff80).wrapping_add(1);
    }
    value.wrapping_sub(remainder as i32)
}

// Translated from 00457880 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x1c` of what `00457720` finds at `pos` (`00541570`); 2 when
/// it finds nothing.
pub fn fn_00457880(e: &mut Engine, this: Ptr<TES>, pos: Ptr) -> u32 {
    let mut result = 2u32;
    let found = fn_00457720(e, this, pos);
    if found != 0 {
        result = e.call(0x0054_1570, &args![found]).u32();
    }
    result
}

// Translated from 004578c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this -= other` on three floats (a `NiPoint3`); returns `this`.
pub fn fn_004578c0(e: &mut Engine, this: Ptr<NiPoint3>, other: Ptr<NiPoint3>) -> Ptr<NiPoint3> {
    let x = e.get(this, NiPoint3::x) as f64 - e.get(other, NiPoint3::x) as f64;
    e.set(this, NiPoint3::x, x as f32);
    let y = e.get(this, NiPoint3::y) as f64 - e.get(other, NiPoint3::y) as f64;
    e.set(this, NiPoint3::y, y as f32);
    let z = e.get(this, NiPoint3::z) as f64 - e.get(other, NiPoint3::z) as f64;
    e.set(this, NiPoint3::z, z as f32);
    this
}

// Translated from 00457910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint3::UnitizeGetLength` (Xbox PDB): scales `this` to length 1 and
/// returns its old length; a vector whose length is at most the `double`
/// at `01017cf8` (1e-6) (or not a number) becomes zero and the result is 0.
pub fn ni_point3_unitize_get_length(e: &mut Engine, this: Ptr<NiPoint3>) -> f32 {
    let length = fn_00457990(e, this) as f32;
    let threshold: f64 = e.global(0x0101_7cf8);
    if length as f64 > threshold {
        let inverse = (1.0 / length as f64) as f32;
        let x = e.get(this, NiPoint3::x) as f64 * inverse as f64;
        e.set(this, NiPoint3::x, x as f32);
        let y = e.get(this, NiPoint3::y) as f64 * inverse as f64;
        e.set(this, NiPoint3::y, y as f32);
        let z = e.get(this, NiPoint3::z) as f64 * inverse as f64;
        e.set(this, NiPoint3::z, z as f32);
        length
    } else {
        e.set(this, NiPoint3::x, 0.0);
        e.set(this, NiPoint3::y, 0.0);
        e.set(this, NiPoint3::z, 0.0);
        0.0
    }
}

// Translated from 00457990 (decompiled, FalloutNV.exe 1.4.0.525)
/// The length of the three floats of `this`, from the squared sum rounded to
/// a `float` and passed to `004579e0` (the square root); the result is that
/// function's `ST0`.
pub fn fn_00457990(e: &mut Engine, this: Ptr<NiPoint3>) -> f64 {
    let x = e.get(this, NiPoint3::x) as f64;
    let y = e.get(this, NiPoint3::y) as f64;
    let z = e.get(this, NiPoint3::z) as f64;
    let squared = (x * x + y * y + z * z) as f32;
    e.call(0x0045_79e0, &args![squared]).f64()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0044fb20,
            tes_tes(Ptr<TES>, u32, Ptr, Ptr, Ptr, Ptr) -> Ptr<TES>
        ),
        entry!(0x004503f0, fn_004503f0(Ptr) -> u32),
        entry!(0x00450410, fn_00450410(Ptr) -> f32),
        entry!(0x00450430, fn_00450430(u32)),
        entry!(0x00450440, fn_00450440(u8)),
        entry!(0x00450450, fn_00450450(u32)),
        entry!(0x00450460, fn_00450460(f32)),
        entry!(0x00450470, fn_00450470(u32, u32)),
        entry!(0x00450490, fn_00450490(u8)),
        entry!(0x004504a0, fn_004504a0(f32)),
        entry!(0x004504b0, fn_004504b0(f32)),
        entry!(0x004504c0, fn_004504c0(f32)),
        entry!(0x004504d0, fn_004504d0(f32)),
        entry!(0x004504e0, fn_004504e0(u32)),
        entry!(0x004504f0, fn_004504f0(u32, f32)),
        entry!(0x00450510, fn_00450510(f32)),
        entry!(0x00450520, fn_00450520(f32)),
        entry!(0x00450530, fn_00450530(f32)),
        entry!(0x00450540, fn_00450540(f32)),
        entry!(0x00450550, fn_00450550(f32)),
        entry!(0x00450560, fn_00450560(u8)),
        entry!(0x00450570, fn_00450570(Ptr) -> Ptr),
        entry!(0x004505a0, fn_004505a0(Ptr) -> Ptr),
        entry!(0x004505d0, fn_004505d0(u32)),
        entry!(0x004505e0, fn_004505e0(u32)),
        entry!(0x004505f0, fn_004505f0(Ptr) -> Ptr),
        entry!(0x00450620, bs_temp_node_manager_get_rtti(Ptr) -> Ptr),
        entry!(
            0x00450630,
            bs_temp_node_manager_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00450670, fn_00450670(f32)),
        entry!(0x00450680, fn_00450680(f32)),
        entry!(0x00450690, fn_00450690(f32)),
        entry!(0x004506a0, fn_004506a0(u32)),
        entry!(0x004506b0, fn_004506b0()),
        entry!(0x00450770, fn_00450770(Ptr<TES>)),
        entry!(0x00450b40, fn_00450b40(Ptr, u8)),
        entry!(0x00450b60, fn_00450b60(Ptr<TES>)),
        entry!(0x00450b80, fn_00450b80(u32) -> u32),
        entry!(0x00450b90, fn_00450b90(Ptr, u32) -> Ptr),
        entry!(0x00450bc0, fn_00450bc0(Ptr, u32) -> Ptr),
        entry!(0x00450bf0, fn_00450bf0(Ptr, u32) -> Ptr),
        entry!(0x00450c20, fn_00450c20(Ptr, u32) -> Ptr),
        entry!(0x00450c50, fn_00450c50(Ptr<TES>)),
        entry!(0x00450d00, fn_00450d00(Ptr<TES>)),
        entry!(0x00450d50, fn_00450d50(Ptr, u32) -> Ptr),
        entry!(0x00450d80, fn_00450d80(Ptr<TES>)),
        entry!(0x00450dd0, tes_cull_grid_cells(Ptr<TES>, u8, u32)),
        entry!(0x00450f90, fn_00450f90(Ptr, u8)),
        entry!(0x00450fb0, fn_00450fb0(Ptr) -> bool),
        entry!(0x00450fd0, fn_00450fd0(Ptr) -> u8),
        entry!(0x00450ff0, fn_00450ff0(Ptr) -> bool),
        entry!(0x00451010, fn_00451010() -> u32),
        entry!(0x00451020, fn_00451020(Ptr<TES>, Ptr, f32)),
        entry!(0x00451110, fn_00451110(Ptr<TES>, Ptr) -> bool),
        entry!(0x004511e0, tes_is_cell_loaded(Ptr<TES>, Ptr, u8) -> bool),
        entry!(0x00451250, fn_00451250(Ptr, u8)),
        entry!(0x004512a0, fn_004512a0(Ptr, u8)),
        entry!(0x004512c0, fn_004512c0(Ptr<TES>, Ptr, u8)),
        entry!(0x00451520, fn_00451520(u8)),
        entry!(0x00451530, fn_00451530(Ptr<TES>) -> bool),
        entry!(0x00451570, fn_00451570(u32)),
        entry!(0x00451590, fn_00451590(u8)),
        entry!(0x004515a0, fn_004515a0(Ptr<TES>, Ptr, u8)),
        entry!(0x004516b0, fn_004516b0(Ptr) -> bool),
        entry!(
            0x004516d0,
            tes_find_first_grid_cell_with_water(Ptr<TES>) -> Ptr
        ),
        entry!(0x004518e0, fn_004518e0(Ptr) -> bool),
        entry!(0x00451900, fn_00451900(Ptr<TES>, i32, i32) -> u32),
        entry!(0x004519d0, fn_004519d0(Ptr<TES>, f32, f32, u32) -> u32),
        entry!(0x00451a50, tes_load_grid_cell(Ptr<TES>, u32, u32, i32, i32)),
        entry!(0x00451cb0, fn_00451cb0(Ptr) -> u32),
        entry!(0x00451cd0, fn_00451cd0(Ptr) -> u8),
        entry!(0x00451cf0, tes_update_grid_lights(Ptr<TES>)),
        entry!(0x00451da0, fn_00451da0(Ptr<TES>, Ptr)),
        entry!(0x00451e40, fn_00451e40(Ptr<TES>, Ptr) -> bool),
        entry!(0x00451ea0, fn_00451ea0(Ptr<TES>, Ptr)),
        entry!(0x00451ef0, fn_00451ef0(Ptr<TES>, Ptr, Ptr, Ptr, u8)),
        entry!(0x00452370, fn_00452370(Ptr) -> bool),
        entry!(0x00452390, fn_00452390(Ptr)),
        entry!(0x004523c0, fn_004523c0(Ptr) -> bool),
        entry!(0x004523e0, fn_004523e0(Ptr, u32) -> bool),
        entry!(0x00452420, fn_00452420(Ptr, u32) -> bool),
        entry!(0x00452440, fn_00452440(Ptr) -> bool),
        entry!(0x00452460, fn_00452460(Ptr, u32) -> bool),
        entry!(0x00452480, fn_00452480() -> u8),
        entry!(0x00452490, tes_clean_up_unused_textures(Ptr<TES>, u8)),
        entry!(0x004524f0, fn_004524f0(Ptr<TES>) -> u8),
        entry!(0x00452510, fn_00452510()),
        entry!(0x00452530, fn_00452530()),
        entry!(0x00452540, fn_00452540(Ptr<TES>) -> u8),
        entry!(
            0x00452580,
            tes_update_current_grid_cell(Ptr<TES>, Ptr, u8) -> bool
        ),
        entry!(0x00452dc0, fn_00452dc0(Ptr, f32, f32) -> Ptr),
        entry!(0x00452df0, fn_00452df0(Ptr, Ptr) -> bool),
        entry!(0x00452e40, fn_00452e40(u8)),
        entry!(0x00452e50, fn_00452e50(Ptr<TES>)),
        entry!(0x00452e70, fn_00452e70(u8) -> f32),
        entry!(0x00452e90, fn_00452e90(Ptr) -> bool),
        entry!(0x00452eb0, tes_cancel_exterior_cell_loads(Ptr<TES>)),
        entry!(0x00452ed0, tes_all_cells_in_grid_loaded(Ptr<TES>) -> bool),
        entry!(0x00452ff0, tes_grid_array_load(Ptr<TES>)),
        entry!(0x00453470, fn_00453470(Ptr) -> u32),
        entry!(0x00453490, fn_00453490(Ptr<TES>, Ptr) -> bool),
        entry!(0x004534f0, fn_004534f0(u32, u32)),
        entry!(0x00453500, fn_00453500(Ptr, u8)),
        entry!(0x00453550, fn_00453550(Ptr<TES>, f32)),
        entry!(0x004536e0, fn_004536e0(Ptr) -> f32),
        entry!(0x00453700, fn_00453700(Ptr) -> f32),
        entry!(0x00453720, fn_00453720() -> u32),
        entry!(0x004537b0, fn_004537b0() -> u32),
        entry!(0x004537c0, tes_update_cell_main_thread(Ptr<TES>)),
        entry!(0x00453a70, bs_audio_q_instance() -> u32),
        entry!(0x00453a80, fn_00453a80(Ptr<TES>) -> bool),
        entry!(0x00453dc0, fn_00453dc0(Ptr<TES>, Ptr, Ptr)),
        entry!(0x00454380, fn_00454380(Ptr, u8)),
        entry!(0x004543a0, fn_004543a0(Ptr) -> u32),
        entry!(0x004543c0, fn_004543c0(Ptr) -> u32),
        entry!(0x00454400, fn_00454400(Ptr) -> u32),
        entry!(0x00454420, fn_00454420(Ptr<TES>, i32) -> u32),
        entry!(0x00454440, fn_00454440(u8)),
        entry!(0x00454450, fn_00454450(Ptr<TES>, Ptr)),
        entry!(0x00454ae0, fn_00454ae0() -> u8),
        entry!(0x00454af0, fn_00454af0(u32) -> u8),
        entry!(0x00454b10, fn_00454b10(Ptr) -> bool),
        entry!(0x00454b30, fn_00454b30(Ptr) -> u32),
        entry!(0x00454b48, fn_00454b48()),
        entry!(0x00454b50, fn_00454b50(Ptr) -> u32),
        entry!(0x00454b70, fn_00454b70(Ptr) -> bool),
        entry!(0x00454b90, tes_add_to_buffer(Ptr<TES>, Ptr)),
        entry!(0x00454bd0, fn_00454bd0(Ptr<TES>, Ptr) -> bool),
        entry!(0x00454c20, fn_00454c20(Ptr<TES>, Ptr)),
        entry!(0x00454d50, fn_00454d50(Ptr<TES>, u8) -> i32),
        entry!(0x00454e20, fn_00454e20(Ptr<TES>, Ptr) -> bool),
        entry!(0x00454e70, fn_00454e70(Ptr<TES>, Ptr)),
        entry!(0x00454fc0, tes_sort_exterior_buffer(Ptr<TES>)),
        entry!(0x00455130, fn_00455130(Ptr<TES>, Ptr)),
        entry!(0x00455200, fn_00455200(Ptr<TES>, u8, Ptr) -> i32),
        entry!(0x00455330, fn_00455330(Ptr<TES>, Ptr)),
        entry!(0x00455400, fn_00455400(Ptr<TES>, Ptr)),
        entry!(0x00455490, fn_00455490(Ptr<TES>)),
        entry!(0x00455600, fn_00455600(Ptr) -> u32),
        entry!(0x00455620, fn_00455620(Ptr) -> bool),
        entry!(0x00455640, fn_00455640(Ptr<TES>)),
        entry!(0x00455680, fn_00455680()),
        entry!(0x00455690, fn_00455690(Ptr<TES>)),
        entry!(0x004556d0, tes_test_all_cells(Ptr<TES>, i32)),
        entry!(0x00456670, tes_get_face_count(Ptr<TES>, Ptr, u8) -> i32),
        entry!(0x004567b0, fn_004567b0(Ptr) -> u16),
        entry!(0x004567e0, fn_004567e0(Ptr) -> u16),
        entry!(
            0x00456800,
            tes_remove_texture_image(Ptr<TES>, Ptr, u32) -> bool
        ),
        entry!(
            0x004568c0,
            tes_create_texture_image(Ptr<TES>, Ptr, Ptr, u8, u8)
        ),
        entry!(0x00457070, tes_get_current_cell(Ptr<TES>) -> u32),
        entry!(
            0x004570d0,
            fn_004570d0(Ptr<TES>, Ptr, u32, u32, f32) -> bool
        ),
        entry!(
            0x004571c0,
            fn_004571c0(Ptr<TES>, Ptr, u32, u32, f32, u8) -> bool
        ),
        entry!(0x004572e0, fn_004572e0(Ptr<TES>, Ptr, Ptr) -> bool),
        entry!(0x004573f0, fn_004573f0(Ptr<TES>, Ptr, Ptr, u32) -> bool),
        entry!(0x00457520, fn_00457520(Ptr<TES>, Ptr, u32) -> bool),
        entry!(0x00457620, fn_00457620(Ptr<TES>, Ptr) -> u32),
        entry!(0x00457720, fn_00457720(Ptr<TES>, Ptr) -> u32),
        entry!(0x00457880, fn_00457880(Ptr<TES>, Ptr) -> u32),
        entry!(
            0x004578c0,
            fn_004578c0(Ptr<NiPoint3>, Ptr<NiPoint3>) -> Ptr<NiPoint3>
        ),
        entry!(
            0x00457910,
            ni_point3_unitize_get_length(Ptr<NiPoint3>) -> f32
        ),
        entry!(0x00457990, fn_00457990(Ptr<NiPoint3>) -> f64),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine where the three setting value-pointer functions are the
    /// game's (`this + 4`).
    fn settings_engine() -> Engine {
        let mut e = Engine::new();
        for function in [
            SETTING_VALUE_ADDRESS_INT,
            SETTING_VALUE_ADDRESS_FLOAT,
            SETTING_VALUE_ADDRESS_BYTE,
        ] {
            e.register(function, |_, a| Ret {
                eax: a[0] + 4,
                ..Ret::default()
            });
        }
        e
    }

    /// A setting object at `addr` holding `value` at +4.
    fn put_setting<T: Scalar>(e: &mut Engine, addr: u32, value: T) {
        e.map(addr, 8);
        e.set_global(addr + 4, value);
    }

    /// Makes the callees do nothing.
    fn noop(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// A setter of a one-word global.
    fn check_word_setter(function: u32, global: u32) {
        let mut e = Engine::new();
        e.map(global, 4);
        e.call(function, &args![0x1234_5678u32]);
        assert_eq!(e.global::<u32>(global), 0x1234_5678);
    }

    fn check_byte_setter(function: u32, global: u32) {
        let mut e = Engine::new();
        e.map(global, 4);
        e.call(function, &args![0x5au8]);
        assert_eq!(e.global::<u32>(global), 0x5a);
    }

    fn check_float_setter(function: u32, global: u32) {
        let mut e = Engine::new();
        e.map(global, 4);
        e.call(function, &args![1.75f32]);
        assert_eq!(e.global::<f32>(global), 1.75);
    }

    #[test]
    fn setting_uint_reads_the_value_through_the_pointer_function() {
        let mut e = settings_engine();
        put_setting(&mut e, 0x011c_3c30, 7u32);
        assert_eq!(e.call(0x0045_03f0, &args![0x011c_3c30u32]).u32(), 7);
    }

    #[test]
    fn setting_float_comes_back_in_st0() {
        let mut e = settings_engine();
        put_setting(&mut e, 0x011c_3d4c, 12.5f32);
        assert_eq!(e.call(0x0045_0410, &args![0x011c_3d4cu32]).f32(), 12.5);
    }

    #[test]
    fn flag_setter_stores_a_word() {
        check_word_setter(0x0045_0430, 0x011f_4300);
    }

    #[test]
    fn queue_warnings_setter_stores_a_byte() {
        check_byte_setter(0x0045_0440, 0x0120_2d62);
    }

    #[test]
    fn havok_update_type_setter_stores_a_word() {
        check_word_setter(0x0045_0450, 0x0126_77b8);
    }

    #[test]
    fn chase_delta_setter_stores_a_float() {
        check_float_setter(0x0045_0460, 0x011a_fe70);
    }

    #[test]
    fn callback_table_setter_stores_at_the_index() {
        let mut e = Engine::new();
        e.map(0x011a_fe88, 0x20);
        e.call(0x0045_0470, &args![0u32, 0x0057_64f0u32]);
        e.call(0x0045_0470, &args![3u32, 0x1234u32]);
        assert_eq!(e.global::<u32>(0x011a_fe88), 0x0057_64f0);
        assert_eq!(e.global::<u32>(0x011a_fe88 + 12), 0x1234);
        assert_eq!(e.global::<u32>(0x011a_fe88 + 4), 0);
    }

    #[test]
    fn add_biped_setter_stores_a_byte() {
        check_byte_setter(0x0045_0490, 0x011a_fe59);
    }

    #[test]
    fn debris_max_velocity_setter_stores_a_float() {
        check_float_setter(0x0045_04a0, 0x0126_77c0);
    }

    #[test]
    fn debris_min_extent_setter_stores_a_float() {
        check_float_setter(0x0045_04b0, 0x0126_77bc);
    }

    #[test]
    fn max_pick_time_setter_stores_a_float() {
        check_float_setter(0x0045_04c0, 0x011a_fe74);
    }

    #[test]
    fn max_pick_time_vats_setter_stores_a_float() {
        check_float_setter(0x0045_04d0, 0x011a_fe78);
    }

    #[test]
    fn entity_batch_remove_rate_setter_stores_a_word() {
        check_word_setter(0x0045_04e0, 0x011a_fe7c);
    }

    #[test]
    fn lod_table_setter_stores_a_float_at_the_index() {
        let mut e = Engine::new();
        e.map(0x011a_d7b8, 0x40);
        e.call(0x0045_04f0, &args![2u32, 0.5f32]);
        e.call(0x0045_04f0, &args![10u32, -1.0f32]);
        assert_eq!(e.global::<f32>(0x011a_d7b8 + 8), 0.5);
        assert_eq!(e.global::<f32>(0x011a_d7b8 + 40), -1.0);
        assert_eq!(e.global::<f32>(0x011a_d7b8 + 12), 0.0);
    }

    #[test]
    fn fade_in_time_setter_stores_a_float() {
        check_float_setter(0x0045_0510, 0x011a_d7e4);
    }

    #[test]
    fn fade_out_time_setter_stores_a_float() {
        check_float_setter(0x0045_0520, 0x011a_d7e8);
    }

    #[test]
    fn fade_in_threshold_setter_stores_a_float() {
        check_float_setter(0x0045_0530, 0x011a_d7ec);
    }

    #[test]
    fn fade_out_threshold_setter_stores_a_float() {
        check_float_setter(0x0045_0540, 0x011a_d7f0);
    }

    #[test]
    fn distance_multiplier_setter_stores_a_float() {
        check_float_setter(0x0045_0550, 0x011a_d7f4);
    }

    #[test]
    fn check_purged_texture_list_setter_stores_a_byte() {
        check_byte_setter(0x0045_0560, 0x011f_4460);
    }

    #[test]
    fn sky_fog_is_null_without_an_atmosphere() {
        let mut e = Engine::new();
        let sky = e.mem.alloc(0x138);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0045_0570, &args![sky]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn sky_fog_asks_the_atmosphere() {
        let mut e = Engine::new();
        let sky = e.mem.alloc(0x138);
        e.mem.set_u32(sky + 0x20, 0x0a7a_0000);
        e.register(0x0043_b230, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0045_0570, &args![sky]).u32(), 0x0a7a_0001);
    }

    #[test]
    fn sky_light_is_null_without_a_sun() {
        let mut e = Engine::new();
        let sky = e.mem.alloc(0x138);
        assert_eq!(e.call(0x0045_05a0, &args![sky]).u32(), 0);
    }

    #[test]
    fn sky_light_asks_the_sun() {
        let mut e = Engine::new();
        let sky = e.mem.alloc(0x138);
        e.mem.set_u32(sky + 0x28, 0x5000_0000);
        e.register(0x0068_38b0, |_, a| Ret {
            eax: a[0] + 2,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0045_05a0, &args![sky]).u32(), 0x5000_0002);
    }

    #[test]
    fn trans_change_callback_setter_stores_a_word() {
        check_word_setter(0x0045_05d0, 0x0126_7b68);
    }

    #[test]
    fn debug_display_callback_setter_stores_a_word() {
        check_word_setter(0x0045_05e0, 0x0126_7b64);
    }

    #[test]
    fn temp_node_manager_constructor_sets_the_vtable() {
        let mut e = Engine::new();
        e.register(0x00a5_ecb0, |e, a| {
            // The `NiNode` constructor writes its own vtable and gets 0.
            assert_eq!(a[1], 0);
            e.mem.set_u32(a[0], 0xdead_beef);
            Ret::default()
        });
        let node = e.mem.alloc(0xb0);
        assert_eq!(e.call(0x0045_05f0, &args![node]).u32(), node);
        assert_eq!(e.mem.u32(node), 0x0101_78bc);
    }

    #[test]
    fn temp_node_manager_rtti_is_a_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0045_0620, &args![0u32]).u32(), 0x0120_2df8);
    }

    #[test]
    fn temp_node_manager_deleting_destructor_frees_only_with_bit_0() {
        let mut e = Engine::new();
        noop(&mut e, &[0x00c4_4e20]);
        let freed = Rc::new(RefCell::new(vec![]));
        let log = freed.clone();
        e.register_double(0x00aa_1460, move |_, a| {
            log.borrow_mut().push((a[0], a[1]));
            Ret::default()
        });
        let node = e.mem.alloc(0xb0);
        assert_eq!(e.call(0x0045_0630, &args![node, 0u32]).u32(), node);
        assert!(freed.borrow().is_empty());
        assert_eq!(e.call(0x0045_0630, &args![node, 1u32]).u32(), node);
        assert_eq!(*freed.borrow(), vec![(node, 0xb0)]);
    }

    #[test]
    fn move_limit_mass_setter_stores_a_float() {
        check_float_setter(0x0045_0670, 0x011b_0128);
    }

    #[test]
    fn step_speed_setter_stores_a_float() {
        check_float_setter(0x0045_0680, 0x011b_0150);
    }

    #[test]
    fn physics_damage_mass_setter_stores_a_float() {
        check_float_setter(0x0045_0690, 0x0126_7bc0);
    }

    #[test]
    fn ranged_node_callback_setter_stores_a_word() {
        check_word_setter(0x0045_06a0, 0x0120_2e08);
    }

    #[test]
    fn lod_fade_multipliers_fill_the_table_from_three_settings() {
        let mut e = settings_engine();
        put_setting(&mut e, 0x011c_3d0c, 1.5f32);
        put_setting(&mut e, 0x011c_3ec0, 2.5f32);
        put_setting(&mut e, 0x011c_3d78, 3.5f32);
        e.map(0x011a_d7b8, 0x40);
        e.call(0x0045_06b0, &args![]);
        let table: Vec<f32> = (0..11).map(|i| e.global(0x011a_d7b8 + 4 * i)).collect();
        assert_eq!(
            table,
            vec![3.5, 3.5, 1.5, 2.5, 3.5, 3.5, 0.0, 0.0, 3.5, 0.0, 3.5]
        );
    }

    #[test]
    fn data_handler_flag_is_a_byte_at_61d() {
        let mut e = Engine::new();
        let handler = e.mem.alloc(0x63c);
        e.call(0x0045_0b40, &args![handler, 1u32]);
        assert_eq!(e.mem.u8(handler + 0x61d), 1);
        e.call(0x0045_0b40, &args![handler, 0u32]);
        assert_eq!(e.mem.u8(handler + 0x61d), 0);
    }

    #[test]
    fn bed_list_call_uses_the_list_at_94() {
        let mut e = Engine::new();
        let tes: Ptr<TES> = e.new_object();
        e.register(0x0047_0470, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0045_0b60, &args![tes]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x0047_0470, vec![tes.addr() + 0x94]));
    }

    #[test]
    fn shadow_scene_node_table_is_indexed_by_four() {
        let mut e = Engine::new();
        e.map(0x011f_91c8, 0x10);
        e.set_global(0x011f_91c8 + 8, 0xabcdu32);
        assert_eq!(e.call(0x0045_0b80, &args![2u32]).u32(), 0xabcd);
        assert_eq!(e.call(0x0045_0b80, &args![1u32]).u32(), 0);
    }

    /// The three scalar deleting destructors share one shape: destructor
    /// call, then `operator delete` (`00401030`) only with bit 0 set.
    fn check_deleting_destructor(function: u32, destructor: u32) {
        let mut e = Engine::new();
        noop(&mut e, &[destructor]);
        let object = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(function, &args![object, 0u32]).u32(), object);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(log[1], (destructor, vec![object]));
        assert!(e.mem.block_size(object).is_some());

        e.call_log = Some(vec![]);
        assert_eq!(e.call(function, &args![object, 1u32]).u32(), object);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().any(|(a, w)| *a == 0x0040_1030 && w[0] == object));
        assert_eq!(e.mem.block_size(object), None);
    }

    #[test]
    fn model_loader_deleting_destructor() {
        check_deleting_destructor(0x0045_0b90, 0x0044_2aa0);
    }

    #[test]
    fn parallel_task_manager_deleting_destructor() {
        check_deleting_destructor(0x0045_0bc0, 0x00c4_4b70);
    }

    #[test]
    fn save_load_deleting_destructor() {
        check_deleting_destructor(0x0045_0bf0, 0x0085_6870);
    }

    // The constructor and destructor.

    /// Everything `TES::TES` touches, with doubles for the callees that are
    /// other units' work. Values: five grids to load, 10 exterior cells,
    /// no interior cells, `threads` hardware threads.
    fn constructor_engine(threads: i32) -> Engine {
        let mut e = settings_engine();
        put_setting(&mut e, SETTING_HAVOK_UPDATE_TYPE, 2u32);
        put_setting(&mut e, SETTING_ADD_BIPED_WHEN_KEYFRAMED, 1u8);
        put_setting(&mut e, SETTING_DEBRIS_MAX_VELOCITY, 3.5f32);
        put_setting(&mut e, SETTING_DEBRIS_MIN_EXTENT, 0.25f32);
        put_setting(&mut e, SETTING_CHASE_DELTA_MULT, 1.5f32);
        put_setting(&mut e, SETTING_MAX_PICK_TIME, 0.5f32);
        put_setting(&mut e, SETTING_MAX_PICK_TIME_VATS, 0.75f32);
        put_setting(&mut e, SETTING_ENTITY_BATCH_REMOVE_RATE, 7u32);
        put_setting(&mut e, SETTING_MOVE_LIMIT_MASS, 90.0f32);
        put_setting(
            &mut e,
            SETTING_CHARACTER_CONTROLLER_MULTIPLE_STEP_SPEED,
            3.0f32,
        );
        put_setting(&mut e, SETTING_PHYSICS_DAMAGE_1_MASS, 12.0f32);
        put_setting(&mut e, SETTING_FADE_OUT_THRESHOLD, 2.0f32);
        put_setting(&mut e, SETTING_FADE_IN_THRESHOLD, 4.0f32);
        put_setting(&mut e, SETTING_FADE_IN_TIME, 0.125f32);
        put_setting(&mut e, SETTING_FADE_OUT_TIME, 0.0625f32);
        put_setting(&mut e, SETTING_DISTANCE_MULTIPLIER, 8.0f32);
        put_setting(&mut e, SETTING_QUEUE_WARNINGS, 1u8);
        put_setting(&mut e, SETTING_CHECK_PURGED_TEXTURE_LIST, 1u8);
        put_setting(&mut e, SETTING_NUM_HW_THREADS, threads);
        put_setting(&mut e, SETTING_EXTERIOR_CELL_BUFFER, 10u32);
        put_setting(&mut e, SETTING_INTERIOR_CELL_BUFFER, 0u32);
        put_setting(&mut e, SETTING_GRIDS_TO_LOAD, 5u32);
        put_setting(&mut e, SETTING_LOD_FADE_OUT_MULT_ITEMS, 1.5f32);
        put_setting(&mut e, SETTING_LOD_FADE_OUT_MULT_ACTORS, 2.5f32);
        put_setting(&mut e, SETTING_LOD_FADE_OUT_MULT_OBJECTS, 3.5f32);
        // The globals, tables and constants.
        for addr in [
            0x011f_4300,
            0x0120_2d62,
            0x0126_77b8,
            0x011a_fe58,
            0x011a_fe70,
            0x011a_fe88,
            0x0126_77bc,
            0x011a_d7b8,
            0x011a_d7e0,
            0x011f_4460,
            0x0126_7b64,
            0x011b_0128,
            0x011b_0150,
            0x0126_7bc0,
            0x0120_2e08,
            0x0120_2d98,
            0x0120_2df4,
            0x011c_3b3c,
            0x011c_3f2c,
            0x011d_e45c,
            0x011c_3c00,
            0x0101_2054,
            0x0101_7868,
            0x0101_3974,
        ] {
            e.map(addr, 0x40);
        }
        e.set_global(0x0101_2054, -1.0f32);
        e.set_global(0x0101_7868, 20.0f32);
        e.set_global(0x0101_786c, 600.0f32);
        e.set_global(0x0101_3974, 1000.0f32);
        e.set_global(0x0101_7870, 0.3f32);
        noop(
            &mut e,
            &[
                0x004e_e810,
                0x0096_a2d0,
                0x0052_8cb0,
                0x0063_3c90,
                SCOPE_GUARD_CTOR,
                SCOPE_GUARD_DTOR,
                0x0062_4bf0,
                0x0052_7eb0,
                0x0084_7480,
                0x0084_fc70,
                0x0046_24b0,
                0x0045_0d80,
                0x0066_b0d0,
                0x00a5_ecb0,
            ],
        );
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            Ret::default()
        });
        for allocate in [OPERATOR_NEW, NI_OPERATOR_NEW] {
            e.register(allocate, |e, a| Ret {
                eax: e.mem.alloc(a[0]),
                ..Ret::default()
            });
        }
        // Constructors of the singletons return `this`.
        for constructor in [
            0x00c3_da50,
            0x0044_2650,
            0x00c4_4980,
            0x0045_d270,
            0x0085_6380,
            0x0062_6fd0,
        ] {
            e.register(constructor, |_, a| Ret {
                eax: a[0],
                ..Ret::default()
            });
        }
        // `GridCellArray`: its constructor writes a vtable whose second slot
        // is the reference-count increment.
        e.put_vtable(0x0300_0000, &[0x0300_1000, 0x0300_1004]);
        noop(&mut e, &[0x0300_1004]);
        e.register(0x004b_a280, |e, a| {
            e.mem.set_u32(a[0], 0x0300_0000);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        // Setting a `uint` setting writes its value.
        e.register(SETTING_SET_UINT, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e
    }

    /// A root node whose vtable slot `0xdc` is `0x0300_3000`.
    fn root_node(e: &mut Engine) -> Ptr {
        e.put_vtable(0x0300_2000, &[0x0300_3000; 60]);
        noop(e, &[0x0300_3000]);
        let root = e.mem.alloc(0x100);
        e.mem.set_u32(root, 0x0300_2000);
        Ptr::new(root)
    }

    fn run_constructor(e: &mut Engine, root: Ptr, sky: Ptr) -> Ptr<TES> {
        let tes: Ptr<TES> = e.new_object();
        // A stale list node, to show the `memset`.
        e.mem.set_u32(tes.addr() + 0x8c, 0x1111);
        e.call_log = Some(vec![]);
        let back = e
            .call(
                0x0044_fb20,
                &args![
                    tes,
                    0x77u32,
                    root,
                    Ptr::<()>::new(0x0400_0010),
                    sky,
                    Ptr::<()>::new(0x0400_0020)
                ],
            )
            .ptr::<TES>();
        assert_eq!(back, tes);
        tes
    }

    #[test]
    fn constructor_zeroes_the_object_after_building_the_members() {
        let mut e = constructor_engine(4);
        let root = root_node(&mut e);
        let tes = run_constructor(&mut e, root, Ptr::NULL);
        let log = e.call_log.take().unwrap();
        let position = |addr: u32| log.iter().position(|(a, _)| *a == addr).unwrap();
        // Members are built, then the object is zeroed, then Havok starts.
        assert!(position(0x004e_e810) < position(MEMSET));
        assert!(position(0x0096_a2d0) < position(MEMSET));
        assert!(position(MEMSET) < position(0x0062_4bf0));
        assert_eq!(log[position(MEMSET)].1, vec![tes.addr(), 0, 0xc4]);
        // The vtable pointer written first is gone, as in the game.
        assert_eq!(e.mem.u32(tes.addr()), 0);
        assert_eq!(e.mem.u32(tes.addr() + 0x8c), 0);
        // The first scope guard (0xa, line 0x16a) lives to the end.
        assert_eq!(
            log[position(SCOPE_GUARD_CTOR)].1[1..],
            [0xa, 1, TES_CPP_PATH, 0x16a]
        );
        assert_eq!(log.last().unwrap().0, SCOPE_GUARD_DTOR);
    }

    #[test]
    fn constructor_copies_the_settings_into_the_globals() {
        let mut e = constructor_engine(4);
        let root = root_node(&mut e);
        run_constructor(&mut e, root, Ptr::NULL);
        assert_eq!(e.global::<u32>(0x011f_4300), 1);
        assert_eq!(e.global::<u32>(0x0126_77b8), 2);
        assert_eq!(e.global::<u32>(0x011a_fe88), 0x0057_64f0);
        assert_eq!(e.global::<u8>(0x011a_fe59), 1);
        assert_eq!(e.global::<f32>(0x0126_77c0), 3.5);
        assert_eq!(e.global::<f32>(0x0126_77bc), 0.25);
        assert_eq!(e.global::<f32>(0x011a_fe70), 1.5);
        assert_eq!(e.global::<f32>(0x011a_fe74), 0.5);
        assert_eq!(e.global::<f32>(0x011a_fe78), 0.75);
        assert_eq!(e.global::<u32>(0x011a_fe7c), 7);
        assert_eq!(e.global::<f32>(0x011b_0128), 90.0);
        // The step speed is squared.
        assert_eq!(e.global::<f32>(0x011b_0150), 9.0);
        assert_eq!(e.global::<f32>(0x0126_7bc0), 12.0);
        // LOD table: items, actors, objects, and the two constants.
        assert_eq!(e.global::<f32>(0x011a_d7b8 + 8), 1.5);
        assert_eq!(e.global::<f32>(0x011a_d7b8 + 12), 2.5);
        assert_eq!(e.global::<f32>(0x011a_d7b8), 3.5);
        assert_eq!(e.global::<f32>(0x011a_d7b8 + 24), 0.0);
        assert_eq!(e.global::<f32>(0x011a_d7b8 + 28), -1.0);
        assert_eq!(e.global::<f32>(0x011a_d7f0), 2.0);
        assert_eq!(e.global::<f32>(0x011a_d7ec), 4.0);
        assert_eq!(e.global::<f32>(0x011a_d7e4), 0.125);
        assert_eq!(e.global::<f32>(0x011a_d7e8), 0.0625);
        assert_eq!(e.global::<f32>(0x011a_d7f4), 8.0);
        assert_eq!(e.global::<u8>(0x0120_2d62), 1);
        assert_eq!(e.global::<u8>(0x011f_4460), 1);
        assert_eq!(e.global::<u32>(0x0126_7b68), 0x0057_7450);
        assert_eq!(e.global::<u32>(0x0126_7b64), 0x0057_7ca0);
        assert_eq!(e.global::<u32>(0x0120_2e08), 0x0045_c680);
    }

    #[test]
    fn constructor_creates_the_singletons() {
        let mut e = constructor_engine(4);
        let root = root_node(&mut e);
        let sizes = Rc::new(RefCell::new(vec![]));
        let seen = sizes.clone();
        e.register_double(OPERATOR_NEW, move |e, a| {
            seen.borrow_mut().push(a[0]);
            Ret {
                eax: e.mem.alloc(a[0]),
                ..Ret::default()
            }
        });
        let tes = run_constructor(&mut e, root, Ptr::NULL);
        // Sizes in order: grid cell array, IO manager, model loader, task
        // manager (four threads), data handler, save/load, then the two
        // cell buffers (one interior slot after the raise, 36 exterior).
        assert_eq!(
            *sizes.borrow(),
            vec![0x28, 0xa0, 0x30, 0x10, 0x63c, 0x1c8, 4, 144]
        );
        for singleton in [
            0x0120_2d98,
            0x011c_3b3c,
            0x0120_2df4,
            0x011c_3f2c,
            0x011d_e45c,
        ] {
            assert_ne!(e.global::<u32>(singleton), 0);
        }
        let handler = e.global::<u32>(0x011c_3f2c);
        let log = e.call_log.take().unwrap();
        let init = log.iter().find(|(a, _)| *a == 0x0046_24b0).unwrap();
        assert_eq!(init.1, vec![handler, 0x77]);
        // The grid cell array is stored and its second virtual was called.
        let cells = e.get(tes, TES::pGridCellA);
        assert_eq!(e.mem.u32(cells.addr()), 0x0300_0000);
        assert!(log
            .iter()
            .any(|(a, w)| *a == 0x0300_1004 && w[0] == cells.addr()));
    }

    #[test]
    fn constructor_skips_the_task_manager_on_one_thread() {
        let mut e = constructor_engine(1);
        let root = root_node(&mut e);
        run_constructor(&mut e, root, Ptr::NULL);
        assert_eq!(e.global::<u32>(0x0120_2df4), 0);
        assert_ne!(e.global::<u32>(0x011c_3f2c), 0);
    }

    #[test]
    fn constructor_raises_the_cell_buffer_settings() {
        let mut e = constructor_engine(4);
        let root = root_node(&mut e);
        let tes = run_constructor(&mut e, root, Ptr::NULL);
        // 5 * 5 + 5 + 5 + 1 = 36 exterior cells, and at least one interior.
        assert_eq!(e.global::<u32>(SETTING_EXTERIOR_CELL_BUFFER + 4), 36);
        assert_eq!(e.global::<u32>(SETTING_INTERIOR_CELL_BUFFER + 4), 1);
        let interior = e.get(tes, TES::pInteriorBuffer).addr();
        let exterior = e.get(tes, TES::pExteriorBuffer).addr();
        assert_eq!(e.mem.block_size(interior), Some(8));
        assert_eq!(e.mem.block_size(exterior), Some(144));
        assert_eq!(e.mem.u32(interior), 0);
        assert_eq!(e.mem.u32(exterior + 140), 0);
        assert_eq!(e.get(tes, TES::iTempInteriorBufferSize), 0);
        assert_eq!(e.get(tes, TES::iTempExteriorBufferSize), 0);
    }

    #[test]
    fn constructor_leaves_large_cell_buffers_alone() {
        let mut e = constructor_engine(4);
        put_setting(&mut e, SETTING_EXTERIOR_CELL_BUFFER, 100u32);
        put_setting(&mut e, SETTING_INTERIOR_CELL_BUFFER, 3u32);
        let root = root_node(&mut e);
        let tes = run_constructor(&mut e, root, Ptr::NULL);
        assert_eq!(e.global::<u32>(SETTING_EXTERIOR_CELL_BUFFER + 4), 100);
        assert_eq!(e.global::<u32>(SETTING_INTERIOR_CELL_BUFFER + 4), 3);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a != SETTING_SET_UINT));
        let exterior = e.get(tes, TES::pExteriorBuffer).addr();
        assert_eq!(e.mem.block_size(exterior), Some(400));
    }

    #[test]
    fn constructor_raises_the_grid_high_water_mark_only_upwards() {
        let mut e = constructor_engine(4);
        let root = root_node(&mut e);
        run_constructor(&mut e, root, Ptr::NULL);
        assert_eq!(e.global::<u32>(0x011c_3c00), 25);

        let mut e = constructor_engine(4);
        e.set_global(0x011c_3c00, 100u32);
        let root = root_node(&mut e);
        run_constructor(&mut e, root, Ptr::NULL);
        assert_eq!(e.global::<u32>(0x011c_3c00), 100);
    }

    #[test]
    fn constructor_stores_the_arguments_and_attaches_the_temp_node_manager() {
        let mut e = constructor_engine(4);
        let root = root_node(&mut e);
        let tes = run_constructor(&mut e, root, Ptr::NULL);
        assert_eq!(e.get(tes, TES::pObjRoot), root);
        assert_eq!(e.get(tes, TES::pObjLandRoot), Ptr::new(0x0400_0010));
        assert_eq!(e.get(tes, TES::pObjLODWaterRoot), Ptr::new(0x0400_0020));
        assert_eq!(e.get(tes, TES::pSky), Ptr::NULL);
        assert_eq!(e.get(tes, TES::pObjLight), Ptr::NULL);
        assert_eq!(e.get(tes, TES::pObjFog), Ptr::NULL);
        let manager = e.get(tes, TES::pTempNodeManager);
        assert_eq!(e.mem.u32(manager.addr()), 0x0101_78bc);
        let log = e.call_log.take().unwrap();
        // `pObjRoot->vtable[0xdc](pTempNodeManager, true)`.
        let attach = log.iter().find(|(a, _)| *a == 0x0300_3000).unwrap();
        assert_eq!(attach.1, vec![root.addr(), manager.addr(), 1]);
        assert!(e.get(tes, TES::bFadeWhenLoading));
        assert!(e.get(tes, TES::bAllowUnusedPurge));
        assert!(!e.get(tes, TES::bRunningCellTests));
        assert!(!e.get(tes, TES::bRunningCellTests2));
        assert!(!e.get(tes, TES::bShowLANDborders));
        assert!(!e.get(tes, TES::bUpdateGridString));
        // The loaded area bound is built with the four constants and stored.
        let bound = log.iter().find(|(a, _)| *a == 0x0062_6fd0).unwrap();
        assert_eq!(
            bound.1[1..],
            [
                20.0f32.to_bits(),
                600.0f32.to_bits(),
                1000.0f32.to_bits(),
                0.3f32.to_bits()
            ]
        );
        let store = log.iter().find(|(a, _)| *a == 0x0066_b0d0).unwrap();
        assert_eq!(store.1, vec![tes.addr() + 0xc0, bound.1[0]]);
    }

    #[test]
    fn constructor_without_a_root_does_not_attach() {
        let mut e = constructor_engine(4);
        let tes = run_constructor(&mut e, Ptr::NULL, Ptr::NULL);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a != 0x0300_3000));
        assert_ne!(e.get(tes, TES::pTempNodeManager), Ptr::NULL);
    }

    #[test]
    fn constructor_takes_light_and_fog_from_the_sky() {
        let mut e = constructor_engine(4);
        let root = root_node(&mut e);
        let sky = e.mem.alloc(0x138);
        e.mem.set_u32(sky + 0x20, 0x0500_0000);
        e.mem.set_u32(sky + 0x28, 0x0600_0000);
        e.register(0x0043_b230, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(0x0068_38b0, |_, a| Ret {
            eax: a[0] + 2,
            ..Ret::default()
        });
        let tes = run_constructor(&mut e, root, Ptr::new(sky));
        assert_eq!(e.get(tes, TES::pSky), Ptr::new(sky));
        assert_eq!(e.get(tes, TES::pObjFog), Ptr::new(0x0500_0001));
        assert_eq!(e.get(tes, TES::pObjLight), Ptr::new(0x0600_0002));
    }

    /// `TES::~TES` with a double for every callee that is another unit's
    /// work.
    fn destructor_engine() -> (Engine, Ptr<TES>) {
        let mut e = settings_engine();
        for addr in [
            0x011f_4300,
            0x0120_2d98,
            0x0120_2df4,
            0x0120_2e08,
            0x011c_3b3c,
            0x011c_3f2c,
            0x011d_e45c,
            0x011d_ea3c,
            0x011f_91c8,
        ] {
            e.map(addr, 0x40);
        }
        noop(
            &mut e,
            &[
                0x0086_7840,
                0x0052_7f40,
                0x008a_8150,
                0x0097_7540,
                0x0045_39a0,
                0x0047_0470,
                0x0045_8f90,
                0x0065_1e30,
                0x0066_4cd0,
                0x0045_ac80,
                0x0045_a520,
                0x0045_a1c0,
                0x0045_a350,
                0x0068_b4a0,
                0x004d_5d50,
                0x009c_6ba0,
                0x0045_dfe0,
                0x0045_0d00,
                0x00c4_59d0,
                0x0084_7500,
                0x0084_fd30,
                0x0066_b0d0,
                0x0045_bb80,
                0x004e_8000,
                0x00b5_d180,
                0x00b6_0040,
                0x0062_5fb0,
                0x004e_e920,
                0x0045_cec0,
                0x0044_cbf0,
                0x0046_ffb0,
                0x004e_e840,
                0x0044_2aa0,
                0x00c4_4b70,
                0x0085_6870,
                0x0300_1000,
                0x0300_1100,
            ],
        );
        // Vtables: slot 0 is a deleting destructor; slot 0x1cc is called
        // with two zeros.
        e.put_vtable(0x0300_0000, &[0x0300_1000]);
        let mut slots = vec![0x0300_1000u32; 0x1d0 / 4];
        slots[0x1cc / 4] = 0x0300_1100;
        e.put_vtable(0x0300_0100, &slots);
        let tes: Ptr<TES> = e.new_object();
        (e, tes)
    }

    fn object_with_vtable(e: &mut Engine, vtable: u32) -> u32 {
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        object
    }

    fn run_destructor(e: &mut Engine, tes: Ptr<TES>) -> Vec<(u32, Vec<u32>)> {
        e.call_log = Some(vec![]);
        e.call(0x0045_0770, &args![tes]);
        e.call_log.take().unwrap()
    }

    #[test]
    fn destructor_with_nothing_loaded_clears_state_and_calls_the_cleanups() {
        let (mut e, tes) = destructor_engine();
        let handler = e.mem.alloc(0x63c);
        e.set_global(0x011c_3f2c, handler);
        e.set_global(0x0120_2e08, 0x1234u32);
        let object = object_with_vtable(&mut e, 0x0300_0100);
        e.set_global(0x011d_ea3c, object);
        // The data handler is deleted by `00450c20` (a double here).
        let deleted = Rc::new(RefCell::new(vec![]));
        let log = deleted.clone();
        e.register_double(0x0045_0c20, move |_, a| {
            log.borrow_mut().push((a[0], a[1]));
            Ret::default()
        });
        let calls = run_destructor(&mut e, tes);
        // The vtable is restored and the callback cleared.
        assert_eq!(e.mem.u32(tes.addr()), 0x0101_78b4);
        assert_eq!(e.global::<u32>(0x0120_2e08), 0);
        // `bClearingData` was set while the data was cleared, and the
        // handler is deleted through `00450c20`.
        assert_eq!(e.mem.u8(handler + 0x61d), 1);
        assert_eq!(*deleted.borrow(), vec![(handler, 1)]);
        assert_eq!(e.global::<u32>(0x011c_3f2c), 0);
        // Slot 0x1cc of the object at 011dea3c is called with two zeros.
        let slot = calls.iter().find(|(a, _)| *a == 0x0300_1100).unwrap();
        assert_eq!(slot.1, vec![object, 0, 0]);
        // Order of the main steps.
        let position = |addr: u32| calls.iter().position(|(a, _)| *a == addr).unwrap();
        assert!(position(0x0086_7840) < position(0x0052_7f40));
        assert!(position(0x0045_39a0) < position(0x0045_8f90));
        assert!(position(0x0045_dfe0) < position(0x0045_0d00));
        assert!(position(0x0084_7500) < position(0x0066_b0d0));
        assert!(position(0x0066_b0d0) < position(0x0045_0c20));
        assert!(position(0x0045_0c20) < position(0x004e_8000));
        assert!(position(0x004e_e920) < position(0x004e_e840));
        // The member destructors run last, in reverse order of the fields.
        let tail: Vec<(u32, u32)> = calls[position(0x004e_e920)..]
            .iter()
            .map(|(a, w)| (*a, w[0] - tes.addr()))
            .collect();
        assert_eq!(
            tail,
            vec![
                (0x004e_e920, 0x6c),
                (0x0045_cec0, 0xc0),
                (0x0044_cbf0, 0xac),
                (0x0045_cec0, 0xa8),
                (0x0044_cbf0, 0xa4),
                (0x0046_ffb0, 0x9c),
                (0x0046_ffb0, 0x94),
                (0x0046_ffb0, 0x8c),
                (0x004e_e840, 0x6c),
            ]
        );
        // Without a ModelLoader its destructor is not called.
        assert!(calls.iter().all(|(a, _)| *a != 0x0044_2aa0));
    }

    #[test]
    fn destructor_deletes_the_singletons_and_clears_their_globals() {
        let (mut e, tes) = destructor_engine();
        noop(&mut e, &[0x0045_0c20]);
        let handler = e.mem.alloc(0x63c);
        e.set_global(0x011c_3f2c, handler);
        let object = object_with_vtable(&mut e, 0x0300_0100);
        e.set_global(0x011d_ea3c, object);
        let model_loader = e.mem.alloc(0x30);
        e.set_global(0x011c_3b3c, model_loader);
        let io_manager = object_with_vtable(&mut e, 0x0300_0000);
        e.set_global(0x0120_2d98, io_manager);
        let task_manager = e.mem.alloc(0x10);
        e.set_global(0x0120_2df4, task_manager);
        let save_load = e.mem.alloc(0x1c8);
        e.set_global(0x011d_e45c, save_load);
        let grid_cells = object_with_vtable(&mut e, 0x0300_0000);
        e.set(tes, TES::pGridCellA, Ptr::new(grid_cells));
        let interior = e.mem.alloc(8);
        let exterior = e.mem.alloc(8);
        e.set(tes, TES::pInteriorBuffer, Ptr::new(interior));
        e.set(tes, TES::pExteriorBuffer, Ptr::new(exterior));

        let calls = run_destructor(&mut e, tes);
        for global in [0x011c_3b3c, 0x0120_2d98, 0x0120_2df4, 0x011d_e45c] {
            assert_eq!(e.global::<u32>(global), 0);
        }
        // Blocks freed with `operator delete`.
        assert_eq!(e.mem.block_size(model_loader), None);
        assert_eq!(e.mem.block_size(task_manager), None);
        assert_eq!(e.mem.block_size(save_load), None);
        assert_eq!(e.mem.block_size(interior), None);
        assert_eq!(e.mem.block_size(exterior), None);
        // The IO manager and grid cell array are deleted through slot 0 with
        // `1`; the pointer in `pGridCellA` is left as it was.
        let deleting: Vec<&Vec<u32>> = calls
            .iter()
            .filter(|(a, _)| *a == 0x0300_1000)
            .map(|(_, w)| w)
            .collect();
        assert_eq!(*deleting[0], vec![io_manager, 1]);
        assert_eq!(*deleting[1], vec![grid_cells, 1]);
        assert_eq!(e.get(tes, TES::pGridCellA), Ptr::new(grid_cells));
        // The destructors of the singletons were called.
        for destructor in [0x0044_2aa0, 0x00c4_4b70, 0x0085_6870] {
            assert!(calls.iter().any(|(a, _)| *a == destructor));
        }
    }

    #[test]
    fn destructor_cleans_up_each_shadow_scene_node_present() {
        let (mut e, tes) = destructor_engine();
        noop(&mut e, &[0x0045_0c20]);
        let object = object_with_vtable(&mut e, 0x0300_0100);
        e.set_global(0x011d_ea3c, object);
        let handler = e.mem.alloc(0x63c);
        e.set_global(0x011c_3f2c, handler);
        // Entries 1 and 3 are set.
        e.set_global(0x011f_91c8 + 4, 0x7001u32);
        e.set_global(0x011f_91c8 + 12, 0x7003u32);
        let calls = run_destructor(&mut e, tes);
        let nodes: Vec<u32> = calls
            .iter()
            .filter(|(a, _)| *a == 0x00b5_d180)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(nodes, vec![0x7001, 0x7003]);
        let static_cleanups = calls.iter().filter(|(a, _)| *a == 0x00b6_0040).count();
        assert_eq!(static_cleanups, 2);
    }

    // Session 2: the water system, the grid of loaded cells and loading
    // references.

    /// A cell object (0xC0 bytes) with the given load state (+0x26) and flags
    /// byte (+0x24).
    fn new_cell(e: &mut Engine, state: u8, flags: u8) -> u32 {
        let cell = e.mem.alloc(0xc0);
        e.mem.set_u8(cell + 0x26, state);
        e.mem.set_u8(cell + 0x24, flags);
        cell
    }

    /// Makes `addr` return `value` in EAX.
    fn returns(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    /// Makes `addr` return `value` in ST0.
    fn returns_float(e: &mut Engine, addr: u32, value: f64) {
        e.register_double(addr, move |_, _| Ret {
            st0: value,
            ..Ret::default()
        });
    }

    /// `_ftol2_sse` on its `f64` argument: truncation.
    fn install_ftol(e: &mut Engine) {
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            Ret {
                eax: value as i32 as u32,
                ..Ret::default()
            }
        });
    }

    /// The `fistp` helper: the float rounded to the nearest integer.
    fn install_float_to_int(e: &mut Engine) {
        e.register(FLOAT_TO_INT_ROUNDED, |_, a| Ret {
            eax: f32::from_bits(a[0]).round() as i32 as u32,
            ..Ret::default()
        });
    }

    /// `TES::GetWorldSpace` (`this + 0x88`) and the `NiPointer` getter
    /// (`00559450`, the first word of `this`).
    fn install_getters(e: &mut Engine) {
        e.register(GET_WORLD_SPACE, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x88),
            ..Ret::default()
        });
        e.register(0x0055_9450, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
    }

    /// A world manager with a `grids` x `grids` table of cell slots behind a
    /// `GridCellArray::Get` double (slot `(x, y)` is at
    /// `table + (x * grids + y) * 4`), `uGridsToLoad` set to `grids` and a
    /// non-null data handler. Returns the engine, the `TES` and the table.
    fn grid_engine(grids: u32) -> (Engine, Ptr<TES>, u32) {
        let mut e = settings_engine();
        put_setting(&mut e, SETTING_GRIDS_TO_LOAD, grids);
        install_getters(&mut e);
        e.map(DATA_HANDLER, 4);
        e.set_global(DATA_HANDLER, 0x1234u32);
        let tes: Ptr<TES> = e.new_object();
        let array = e.mem.alloc(0x28);
        e.set(tes, TES::pGridCellA, Ptr::new(array));
        let table = e.mem.alloc(grids * grids * 4);
        e.register_double(GRID_CELL_ARRAY_GET, move |_, a| Ret {
            eax: table + (a[1] * grids + a[2]) * 4,
            ..Ret::default()
        });
        (e, tes, table)
    }

    fn put_cell(e: &mut Engine, table: u32, grids: u32, x: u32, y: u32, cell: u32) {
        e.mem.set_u32(table + (x * grids + y) * 4, cell);
    }

    /// Runs `f` with the call log on and returns the log.
    fn run_logged(e: &mut Engine, f: impl FnOnce(&mut Engine)) -> Vec<(u32, Vec<u32>)> {
        e.call_log = Some(vec![]);
        f(e);
        e.call_log.take().unwrap()
    }

    /// The argument lists of the calls to `addr` in `log`.
    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The callee addresses of `log`, in order.
    fn addresses(log: &[(u32, Vec<u32>)]) -> Vec<u32> {
        log.iter().map(|(a, _)| *a).collect()
    }

    /// An object whose vtable (at `vtable`) has the `(byte offset, function)`
    /// entries filled in.
    fn object_with_slots(e: &mut Engine, vtable: u32, entries: &[(u32, u32)]) -> u32 {
        e.map(vtable, 0x400);
        for (slot, function) in entries {
            e.mem.set_u32(vtable + slot, *function);
        }
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn data_handler_deleting_destructor() {
        check_deleting_destructor(0x0045_0c20, 0x0045_d970);
    }

    #[test]
    fn water_system_is_built_under_the_memory_tag_and_stored() {
        let mut e = Engine::new();
        noop(&mut e, &[SCOPE_GUARD_CTOR, SCOPE_GUARD_DTOR]);
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(0x004e_1650, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let tes: Ptr<TES> = e.new_object();
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_0c50, &args![tes]);
        });
        let water = e.get(tes, TES::pWaterSystem);
        assert_eq!(e.mem.block_size(water.addr()), Some(0xa0));
        let guard = calls_to(&log, SCOPE_GUARD_CTOR);
        assert_eq!(guard[0][1..], [0x1d, 1, TES_CPP_PATH, 0x286]);
        assert_eq!(calls_to(&log, 0x004e_1650), vec![vec![water.addr()]]);
        // The guard is destroyed last, with the block the constructor got.
        let (last, words) = log.last().unwrap();
        assert_eq!(*last, SCOPE_GUARD_DTOR);
        assert_eq!(words[0], guard[0][0]);
    }

    #[test]
    fn water_system_with_a_failed_allocation_stores_null() {
        let mut e = Engine::new();
        noop(&mut e, &[SCOPE_GUARD_CTOR, SCOPE_GUARD_DTOR]);
        returns(&mut e, OPERATOR_NEW, 0);
        let tes: Ptr<TES> = e.new_object();
        e.set(tes, TES::pWaterSystem, Ptr::new(0x1234));
        // `004e1650` is not registered: calling it would panic.
        e.call(0x0045_0c50, &args![tes]);
        assert_eq!(e.get(tes, TES::pWaterSystem), Ptr::NULL);
    }

    #[test]
    fn water_system_deletion_deletes_and_clears_the_field() {
        let mut e = Engine::new();
        noop(&mut e, &[0x004e_19c0]);
        let tes: Ptr<TES> = e.new_object();
        let water = e.mem.alloc(0xa0);
        e.set(tes, TES::pWaterSystem, Ptr::new(water));
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_0d00, &args![tes]);
        });
        assert_eq!(calls_to(&log, 0x004e_19c0), vec![vec![water]]);
        assert_eq!(e.mem.block_size(water), None);
        assert_eq!(e.get(tes, TES::pWaterSystem), Ptr::NULL);
        // Nothing to delete the second time.
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_0d00, &args![tes]);
        });
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn water_system_deleting_destructor() {
        check_deleting_destructor(0x0045_0d50, 0x004e_19c0);
    }

    #[test]
    fn grid_positions_are_reset_to_the_no_cell_value() {
        let mut e = Engine::new();
        let tes: Ptr<TES> = e.new_object();
        e.mem.set_u32(tes.addr() + 0x34, 0x1234);
        e.mem.set_u32(tes.addr() + 0x24, 5);
        e.call(0x0045_0d80, &args![tes]);
        for offset in [0x24, 0x28, 0x2c, 0x30, 0x48, 0x4c] {
            assert_eq!(e.mem.u32(tes.addr() + offset), 0x7fff_ffff, "+{offset:#x}");
        }
        assert_eq!(e.mem.u32(tes.addr() + 0x34), 0x1234);
    }

    /// An engine for `tes_cull_grid_cells`: a 2 x 2 grid, a Havok world, and
    /// doubles for every callee. Cells: (0,0) state 3, (0,1) state 6, (1,0)
    /// none, (1,1) state 3.
    fn cull_engine() -> (Engine, Ptr<TES>, [u32; 3]) {
        let (mut e, tes, table) = grid_engine(2);
        e.map(EXTERIOR_WORLD, 4);
        e.set_global(EXTERIOR_WORLD, 0x5000u32);
        noop(
            &mut e,
            &[
                0x00c6_6300,
                0x00c6_6310,
                0x00c6_8f00,
                0x00c6_b540,
                0x0097_5f90,
                0x0043_b370,
                0x006f_d1f0,
                0x0043_d410,
                0x00a5_9c60,
                0x0055_2bd0,
                0x0054_bcf0,
                GRID_CELL_ARRAY_ATTACH_TO_WORLD,
            ],
        );
        returns(&mut e, 0x0096_11e0, 0x6000);
        e.set(tes, TES::pObjRoot, Ptr::new(0x7000));
        e.set(tes, TES::pObjLandRoot, Ptr::new(0x7100));
        let cells = [
            new_cell(&mut e, 3, 0),
            new_cell(&mut e, 6, 0),
            new_cell(&mut e, 3, 0),
        ];
        put_cell(&mut e, table, 2, 0, 0, cells[0]);
        put_cell(&mut e, table, 2, 0, 1, cells[1]);
        put_cell(&mut e, table, 2, 1, 1, cells[2]);
        (e, tes, cells)
    }

    #[test]
    fn culling_attaches_the_cells_in_state_3() {
        let (mut e, tes, cells) = cull_engine();
        let array = e.get(tes, TES::pGridCellA).addr();
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_0dd0, &args![tes, 0u8, 0u32]);
        });
        assert_eq!(
            calls_to(&log, GRID_CELL_ARRAY_ATTACH_TO_WORLD),
            vec![vec![array, 0x7000, 0, 0], vec![array, 0x7000, 1, 1]]
        );
        assert_eq!(
            calls_to(&log, 0x0054_bcf0),
            vec![vec![cells[0], 0], vec![cells[2], 0]]
        );
        assert!(calls_to(&log, 0x0055_2bd0).is_empty());
        // The Havok world first and last, the shadow flag, the terrain switch.
        let at = |address: u32| addresses(&log).iter().position(|a| *a == address).unwrap();
        assert_eq!(calls_to(&log, 0x00c6_6300), vec![vec![0x5000]]);
        assert!(at(0x00c6_6300) < at(0x0054_bcf0));
        assert!(at(0x0054_bcf0) < at(0x00c6_b540));
        assert_eq!(calls_to(&log, 0x00c6_b540), vec![vec![0x5000, 0]]);
        assert_eq!(calls_to(&log, 0x0097_5f90), vec![vec![OBJECT_011E0E80]]);
        assert_eq!(calls_to(&log, 0x0043_b370), vec![vec![0x6000, 0, 1]]);
        assert_eq!(calls_to(&log, 0x006f_d1f0), vec![vec![1]]);
        // The land root is updated with a freshly built 12-byte object.
        let update = calls_to(&log, 0x00a5_9c60);
        assert_eq!(update[0][0], 0x7100);
        let built = calls_to(&log, 0x0043_d410);
        assert_eq!(built[0], vec![update[0][1], 0, 0, 0]);
    }

    #[test]
    fn culling_with_the_detach_flag_detaches_the_cells_in_state_6() {
        let (mut e, tes, cells) = cull_engine();
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_0dd0, &args![tes, 1u8, 0u32]);
        });
        assert_eq!(calls_to(&log, 0x0055_2bd0), vec![vec![cells[1], 0]]);
        assert!(calls_to(&log, GRID_CELL_ARRAY_ATTACH_TO_WORLD).is_empty());
        assert_eq!(calls_to(&log, 0x00c6_6310), vec![vec![0x5000]]);
        assert!(calls_to(&log, 0x00c6_6300).is_empty());
        assert_eq!(calls_to(&log, 0x00c6_8f00), vec![vec![0x5000, 0]]);
        assert!(calls_to(&log, 0x0097_5f90).is_empty());
        assert_eq!(calls_to(&log, 0x0043_b370), vec![vec![0x6000, 1, 1]]);
        assert_eq!(calls_to(&log, 0x006f_d1f0), vec![vec![0]]);
    }

    #[test]
    fn culling_without_a_data_handler_does_nothing() {
        let (mut e, tes, _) = cull_engine();
        e.set_global(DATA_HANDLER, 0u32);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_0dd0, &args![tes, 0u8, 0u32]);
        });
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn culling_without_a_havok_world_skips_the_world_calls() {
        let (mut e, tes, _) = cull_engine();
        e.set_global(EXTERIOR_WORLD, 0u32);
        e.set(tes, TES::pObjLandRoot, Ptr::NULL);
        returns(&mut e, 0x0096_11e0, 0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_0dd0, &args![tes, 0u8, 0u32]);
        });
        for world_call in [0x00c6_6300, 0x00c6_b540, 0x0043_b370, 0x006f_d1f0] {
            assert!(calls_to(&log, world_call).is_empty(), "{world_call:08x}");
        }
        // The attaching still happens.
        assert_eq!(calls_to(&log, 0x0054_bcf0).len(), 2);
    }

    #[test]
    fn node_flag_setter_passes_the_flag_and_the_mask() {
        let mut e = Engine::new();
        noop(&mut e, &[0x0043_b370]);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_0f90, &args![0x6000u32, 1u8]);
        });
        assert_eq!(calls_to(&log, 0x0043_b370), vec![vec![0x6000, 1, 1]]);
    }

    #[test]
    fn cell_state_tests_compare_the_load_state() {
        let mut e = Engine::new();
        for (state, is_3, is_5, is_6) in [
            (0u8, false, false, false),
            (3, true, false, false),
            (5, false, true, false),
            (6, false, false, true),
        ] {
            let cell = new_cell(&mut e, state, 0);
            assert_eq!(e.call(0x0045_0fd0, &args![cell]).u32(), state as u32);
            assert_eq!(e.call(0x0045_0fb0, &args![cell]).bool(), is_3);
            assert_eq!(e.call(0x0045_23c0, &args![cell]).bool(), is_5);
            assert_eq!(e.call(0x0045_0ff0, &args![cell]).bool(), is_6);
        }
    }

    #[test]
    fn exterior_world_is_the_first_word_of_the_ni_pointer() {
        let mut e = Engine::new();
        install_getters(&mut e);
        e.map(EXTERIOR_WORLD, 4);
        e.set_global(EXTERIOR_WORLD, 0x5555u32);
        assert_eq!(e.call(0x0045_1010, &args![]).u32(), 0x5555);
    }

    #[test]
    fn loaded_grid_rectangle_is_cell_corners_inset_by_the_margin() {
        let (mut e, tes, _) = grid_engine(5);
        install_ftol(&mut e);
        e.set(tes, TES::iCurrentGridX, 10);
        e.set(tes, TES::iCurrentGridY, -3);
        let out = e.mem.alloc(16);
        e.call(0x0045_1020, &args![tes, out, 100.5f32]);
        // Cells 8..=12 in x and -5..=-1 in y; each cell is 0x1000 wide. The
        // near corner gets the margin added, the far corner (one cell
        // further) has it subtracted; `_ftol2` truncates towards zero.
        assert_eq!(e.mem.i32(out), 8 * 0x1000 + 100);
        // -20480 + 100.5 = -20379.5, truncated to -20379.
        assert_eq!(e.mem.i32(out + 12), -5 * 0x1000 + 101);
        // 53248 - 100.5 = 53147.5.
        assert_eq!(e.mem.i32(out + 8), 13 * 0x1000 - 101);
        // 0 - 100.5 = -100.5.
        assert_eq!(e.mem.i32(out + 4), -100);
    }

    /// Cell coordinates 10, 20 at the centre of a 5 x 5 grid: cells 8..13 and
    /// 18..23 are loaded.
    fn position_engine() -> (Engine, Ptr<TES>) {
        let (mut e, tes, _) = grid_engine(5);
        install_float_to_int(&mut e);
        returns(&mut e, GET_INTERIOR_CELL, 0);
        e.set(tes, TES::iCurrentGridX, 10);
        e.set(tes, TES::iCurrentGridY, 20);
        (e, tes)
    }

    fn position(e: &mut Engine, x: f32, y: f32) -> u32 {
        let pos = e.mem.alloc(12);
        e.mem.set_f32(pos, x);
        e.mem.set_f32(pos + 4, y);
        pos
    }

    #[test]
    fn position_test_accepts_cells_inside_the_loaded_square() {
        let (mut e, tes) = position_engine();
        let inside = position(&mut e, 8.0 * 4096.0, 22.0 * 4096.0 + 4095.0);
        assert!(e.call(0x0045_1110, &args![tes, inside]).bool());
        let inside = position(&mut e, 12.0 * 4096.0 + 2000.0, 18.0 * 4096.0);
        assert!(e.call(0x0045_1110, &args![tes, inside]).bool());
    }

    #[test]
    fn position_test_rejects_cells_outside_the_loaded_square() {
        let (mut e, tes) = position_engine();
        for (x, y) in [
            (7.0 * 4096.0 + 4000.0, 20.0 * 4096.0),
            (13.0 * 4096.0, 20.0 * 4096.0),
            (10.0 * 4096.0, 17.0 * 4096.0 + 100.0),
            (10.0 * 4096.0, 23.0 * 4096.0),
        ] {
            let pos = position(&mut e, x, y);
            assert!(!e.call(0x0045_1110, &args![tes, pos]).bool(), "{x} {y}");
        }
    }

    #[test]
    fn position_test_is_true_while_an_interior_cell_is_loaded() {
        let (mut e, tes) = position_engine();
        returns(&mut e, GET_INTERIOR_CELL, 0x4444);
        let far = position(&mut e, 1.0e9, -1.0e9);
        assert!(e.call(0x0045_1110, &args![tes, far]).bool());
    }

    #[test]
    fn is_cell_loaded_by_load_state() {
        let mut e = Engine::new();
        let tes: Ptr<TES> = e.new_object();
        let check = |e: &mut Engine, cell: u32, flag: u8| {
            e.call(0x0045_11e0, &args![tes, cell, flag]).bool()
        };
        assert!(!check(&mut e, 0, 0));
        for (state, flag_0, flag_1) in [
            (0u8, false, false),
            (1, false, false),
            (2, true, false),
            (3, true, false),
            (4, true, false),
            (5, true, true),
            (6, true, true),
            (7, false, false),
            (255, false, false),
        ] {
            let cell = new_cell(&mut e, state, 0);
            assert_eq!(check(&mut e, cell, 0), flag_0, "state {state} flag 0");
            assert_eq!(check(&mut e, cell, 1), flag_1, "state {state} flag 1");
        }
    }

    #[test]
    fn water_bit_setter_changes_only_bit_1() {
        let mut e = Engine::new();
        let cell = new_cell(&mut e, 0, 0b1010_0101);
        e.call(0x0045_1250, &args![cell, 1u8]);
        assert_eq!(e.mem.u8(cell + 0x24), 0b1010_0111);
        assert!(e.call(0x0045_18e0, &args![cell]).bool());
        e.call(0x0045_1250, &args![cell, 0u8]);
        assert_eq!(e.mem.u8(cell + 0x24), 0b1010_0101);
        assert!(!e.call(0x0045_18e0, &args![cell]).bool());
    }

    #[test]
    fn cell_state_setter_stores_the_byte() {
        let mut e = Engine::new();
        let cell = new_cell(&mut e, 1, 0xff);
        e.call(0x0045_12a0, &args![cell, 6u8]);
        assert_eq!(e.mem.u8(cell + 0x26), 6);
        assert_eq!(e.mem.u8(cell + 0x24), 0xff);
    }

    /// An engine for `fn_004512c0`: a 5 x 5 grid centred on (10, 20), a
    /// `GridCellArray` whose vtable has slots 8 and 0x10, a player whose slot
    /// 0x1f4 returns a position, and doubles for every callee.
    fn load_engine() -> (Engine, Ptr<TES>) {
        let (mut e, tes, _) = grid_engine(5);
        for (addr, len) in [
            (GRID_LOAD_IN_PROGRESS, 1),
            (GRID_FLAG_01189184, 1),
            (GRID_FLAG_011AD86C, 1),
            (POINT_011A9478, 12),
            (IO_MANAGER, 4),
            (PLAYER_CHARACTER, 4),
            (FADER_MANAGER, 4),
        ] {
            e.map(addr, len);
        }
        e.set_global(IO_MANAGER, 0x5a5au32);
        e.set_global(FADER_MANAGER, 0x6b6bu32);
        for (i, value) in [11.0f32, 12.0, 13.0].into_iter().enumerate() {
            e.set_global(POINT_011A9478 + 4 * i as u32, value);
        }
        put_setting(&mut e, SETTING_FADE_TO_BLACK_FADE_SECONDS, 0.3f32);
        let array = e.get(tes, TES::pGridCellA).addr();
        e.put_vtable(0x0300_1000, &[0, 0, 0x0300_0008, 0, 0x0300_0010]);
        e.mem.set_u32(array, 0x0300_1000);
        let player = object_with_slots(&mut e, 0x0300_2000, &[(0x1f4, 0x0300_01f4)]);
        e.set_global(PLAYER_CHARACTER, player);
        returns(&mut e, 0x0300_01f4, 0x8888);
        noop(&mut e, &[0x0300_0008, 0x0300_0010]);
        returns(&mut e, GET_INTERIOR_CELL, 0);
        returns(&mut e, GET_TERRAIN_MANAGER, 0x7000);
        noop(
            &mut e,
            &[
                0x006f_ce00,
                0x006f_ca90,
                0x0045_5200,
                0x0045_2490,
                0x0045_7be0,
                0x0045_2ff0,
                0x00c3_dfa0,
                0x0057_d0a0,
                0x0070_0960,
                0x0045_9870,
                0x0097_2d30,
                0x0097_2bb0,
                0x0045_5490,
                0x0043_d410,
                0x00a5_9c60,
                0x0046_0360,
                0x0066_b0d0,
            ],
        );
        // `00416870` stores its three floats.
        e.register(0x0041_6870, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            Ret::default()
        });
        // `004572e0` stores a float in the third word of its output.
        e.register(0x0045_72e0, |e, a| {
            e.mem.set_f32(a[2], 9.0);
            Ret::default()
        });
        e.set(tes, TES::iCurrentGridX, 10);
        e.set(tes, TES::iCurrentGridY, 20);
        e.set(tes, TES::pWorldSpace, Ptr::new(0x6000));
        (e, tes)
    }

    #[test]
    fn full_grid_reload_runs_the_steps_in_order() {
        let (mut e, tes) = load_engine();
        let pos = position(&mut e, 1.5, 2.5);
        let array = e.get(tes, TES::pGridCellA).addr();
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_12c0, &args![tes, pos, 1u8]);
        });
        assert_eq!(
            addresses(&log),
            vec![
                0x0045_12c0,
                GET_INTERIOR_CELL,
                0x0300_0008,
                0x0066_b0d0,
                GET_TERRAIN_MANAGER,
                0x006f_ce00,
                0x0300_01f4,
                GET_TERRAIN_MANAGER,
                0x006f_ca90,
                0x0300_0010,
                0x0045_5200,
                0x0045_2490,
                0x0045_7be0,
                0x0045_2ff0,
                0x00c3_dfa0,
                0x0041_6870,
                0x0045_72e0,
                0x0057_d0a0,
                SETTING_VALUE_ADDRESS_FLOAT,
                0x0070_0960,
                0x0045_9870,
                0x0097_2d30,
                0x0097_2bb0,
                0x0045_5490,
                0x0043_d410,
                0x00a5_9c60,
                0x0045_2490,
                0x0046_0360,
            ]
        );
        assert_eq!(calls_to(&log, 0x0300_0008), vec![vec![array]]);
        assert_eq!(calls_to(&log, 0x0066_b0d0), vec![vec![0x011f_95e8, 0]]);
        assert_eq!(calls_to(&log, 0x006f_ce00), vec![vec![0x7000]]);
        // The terrain is updated with the player's position and 0xf.
        assert_eq!(calls_to(&log, 0x006f_ca90), vec![vec![0x7000, 0x8888, 0xf]]);
        // The grid is repositioned at the centre cell.
        assert_eq!(calls_to(&log, 0x0300_0010), vec![vec![array, 10, 20]]);
        assert_eq!(calls_to(&log, 0x0045_5200), vec![vec![tes.addr(), 0, 0]]);
        assert_eq!(
            calls_to(&log, 0x0045_2490),
            vec![vec![tes.addr(), 0], vec![tes.addr(), 0]]
        );
        assert_eq!(calls_to(&log, 0x0045_7be0), vec![vec![tes.addr(), 0]]);
        assert_eq!(calls_to(&log, 0x00c3_dfa0), vec![vec![0x5a5a, 4]]);
        // The point is (x, y, 0), whose third float `004572e0` then replaces.
        let point = calls_to(&log, 0x0041_6870);
        assert_eq!(point[0][1..], [1.5f32.to_bits(), 2.5f32.to_bits(), 0]);
        assert_eq!(
            calls_to(&log, 0x0045_72e0),
            vec![vec![tes.addr(), pos, point[0][0] + 8]]
        );
        assert_eq!(
            calls_to(&log, 0x0057_d0a0),
            vec![vec![
                1.5f32.to_bits(),
                2.5f32.to_bits(),
                9.0f32.to_bits(),
                11.0f32.to_bits(),
                12.0f32.to_bits(),
                13.0f32.to_bits(),
                1.0f32.to_bits()
            ]]
        );
        // The fader: one second argument 1, the setting, and the stray 0.
        assert_eq!(
            calls_to(&log, 0x0070_0960),
            vec![vec![0x6b6b, 1, 0.3f32.to_bits(), 0]]
        );
        assert_eq!(e.global::<u8>(GRID_FLAG_01189184), 1);
        assert_eq!(e.global::<u8>(GRID_FLAG_011AD86C), 1);
        assert_eq!(e.global::<u8>(GRID_LOAD_IN_PROGRESS), 0);
    }

    #[test]
    fn grid_reload_does_not_fade_while_cell_tests_run() {
        let (mut e, tes) = load_engine();
        e.set(tes, TES::bRunningCellTests2, true);
        let pos = position(&mut e, 1.5, 2.5);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_12c0, &args![tes, pos, 1u8]);
        });
        assert!(calls_to(&log, 0x0070_0960).is_empty());
        assert_eq!(calls_to(&log, 0x0045_9870).len(), 1);
    }

    #[test]
    fn partial_grid_load_repositions_and_loads() {
        let (mut e, tes) = load_engine();
        let pos = position(&mut e, 1.5, 2.5);
        let array = e.get(tes, TES::pGridCellA).addr();
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_12c0, &args![tes, pos, 0u8]);
        });
        assert_eq!(
            addresses(&log),
            vec![
                0x0045_12c0,
                GET_INTERIOR_CELL,
                0x0300_0010,
                0x0045_7be0,
                0x0045_2ff0,
                0x0045_5490,
                0x0043_d410,
                0x00a5_9c60,
                0x0045_2490,
                0x0046_0360,
            ]
        );
        assert_eq!(calls_to(&log, 0x0300_0010), vec![vec![array, 10, 20]]);
        assert_eq!(calls_to(&log, 0x0045_7be0), vec![vec![tes.addr(), 1]]);
        assert_eq!(e.global::<u8>(GRID_LOAD_IN_PROGRESS), 0);
    }

    #[test]
    fn grid_load_is_skipped_without_a_data_handler_with_an_interior_or_when_running() {
        let (mut e, tes) = load_engine();
        let pos = position(&mut e, 1.5, 2.5);
        e.set_global(DATA_HANDLER, 0u32);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_12c0, &args![tes, pos, 1u8]);
        });
        assert_eq!(log.len(), 1);

        e.set_global(DATA_HANDLER, 0x1234u32);
        returns(&mut e, GET_INTERIOR_CELL, 0x4444);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_12c0, &args![tes, pos, 1u8]);
        });
        assert_eq!(addresses(&log), vec![0x0045_12c0, GET_INTERIOR_CELL]);

        returns(&mut e, GET_INTERIOR_CELL, 0);
        e.set_global(GRID_LOAD_IN_PROGRESS, 1u8);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_12c0, &args![tes, pos, 1u8]);
        });
        assert_eq!(addresses(&log), vec![0x0045_12c0, GET_INTERIOR_CELL]);
        // The guard byte is left as it was.
        assert_eq!(e.global::<u8>(GRID_LOAD_IN_PROGRESS), 1);
    }

    #[test]
    fn grid_flag_and_helper_functions() {
        let mut e = Engine::new();
        e.map(GRID_FLAG_01189184, 1);
        e.map(GRID_FLAG_011AD86C, 1);
        e.call(0x0045_1520, &args![1u8]);
        e.call(0x0045_1590, &args![1u8]);
        assert_eq!(e.global::<u8>(GRID_FLAG_01189184), 1);
        assert_eq!(e.global::<u8>(GRID_FLAG_011AD86C), 1);
        e.call(0x0045_1520, &args![0u8]);
        assert_eq!(e.global::<u8>(GRID_FLAG_01189184), 0);

        noop(&mut e, &[0x0066_b0d0]);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1570, &args![0x55u32]);
        });
        assert_eq!(calls_to(&log, 0x0066_b0d0), vec![vec![0x011f_95e8, 0x55]]);
    }

    #[test]
    fn running_cell_tests_is_either_flag() {
        let mut e = Engine::new();
        let tes: Ptr<TES> = e.new_object();
        assert!(!e.call(0x0045_1530, &args![tes]).bool());
        e.set(tes, TES::bRunningCellTests, true);
        assert!(e.call(0x0045_1530, &args![tes]).bool());
        e.set(tes, TES::bRunningCellTests, false);
        e.set(tes, TES::bRunningCellTests2, true);
        assert!(e.call(0x0045_1530, &args![tes]).bool());
    }

    /// The engine for `fn_004515a0`: a data handler whose `bSaveLoad` is
    /// clear, the `TESSaveLoadGame` object and doubles for the callees. The
    /// guard byte is set, so `004512c0` returns at once.
    fn centre_engine() -> (Engine, Ptr<TES>) {
        let (mut e, tes) = position_engine();
        let handler = e.mem.alloc(0x63c);
        e.set_global(DATA_HANDLER, handler);
        e.map(SAVE_LOAD_GAME, 4);
        e.set_global(SAVE_LOAD_GAME, 0x1111u32);
        e.map(GRID_LOAD_IN_PROGRESS, 1);
        e.set_global(GRID_LOAD_IN_PROGRESS, 1u8);
        returns(&mut e, 0x0047_c850, 0);
        returns(&mut e, GET_WORLD_SPACE, 0x6000);
        noop(&mut e, &[0x0062_f460, 0x0045_8200]);
        (e, tes)
    }

    #[test]
    fn centring_on_a_position_sets_the_grid_and_queued_cells() {
        let (mut e, tes) = centre_engine();
        let pos = position(&mut e, 3.0 * 4096.0 + 10.0, -4096.0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_15a0, &args![tes, pos, 1u8]);
        });
        assert_eq!(e.get(tes, TES::iCurrentGridX), 3);
        assert_eq!(e.get(tes, TES::iCurrentGridY), -1);
        assert_eq!(e.get(tes, TES::iCurrentQueuedX), 3);
        assert_eq!(e.get(tes, TES::iCurrentQueuedY), -1);
        assert_eq!(
            addresses(&log),
            vec![
                0x0045_15a0,
                0x0047_c850,
                GET_INTERIOR_CELL,
                GET_WORLD_SPACE,
                FLOAT_TO_INT_ROUNDED,
                FLOAT_TO_INT_ROUNDED,
                GET_INTERIOR_CELL,
                0x0062_f460,
            ]
        );
        assert_eq!(calls_to(&log, 0x0047_c850), vec![vec![0x1111]]);
    }

    #[test]
    fn centring_without_a_position_keeps_the_cell_and_sets_a_missing_world_space() {
        let (mut e, tes) = centre_engine();
        returns(&mut e, GET_WORLD_SPACE, 0);
        let holder = e.mem.alloc(8);
        let world_space_slot = e.mem.alloc(8);
        e.mem.set_u32(world_space_slot, 0x9000);
        returns(&mut e, 0x0046_0140, holder);
        returns(&mut e, 0x0068_15c0, world_space_slot);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_15a0, &args![tes, 0u32, 0u8]);
        });
        assert_eq!(e.get(tes, TES::iCurrentGridX), 10);
        assert_eq!(e.get(tes, TES::iCurrentGridY), 20);
        assert_eq!(e.get(tes, TES::iCurrentQueuedX), 10);
        assert_eq!(e.get(tes, TES::iCurrentQueuedY), 20);
        assert_eq!(calls_to(&log, 0x0045_8200), vec![vec![tes.addr(), 0x9000]]);
        assert!(calls_to(&log, FLOAT_TO_INT_ROUNDED).is_empty());
    }

    #[test]
    fn centring_is_skipped_during_a_save_load_unless_forced() {
        let (mut e, tes) = centre_engine();
        let handler: u32 = e.global(DATA_HANDLER);
        e.mem.set_u8(handler + 0x61a, 1);
        let pos = position(&mut e, 3.0 * 4096.0, 0.0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_15a0, &args![tes, pos, 0u8]);
        });
        assert_eq!(
            addresses(&log),
            vec![0x0045_15a0, 0x0047_c850, GET_INTERIOR_CELL]
        );
        assert_eq!(e.get(tes, TES::iCurrentGridX), 10);

        // `0047c850` says to go on regardless.
        returns(&mut e, 0x0047_c850, 1);
        e.call(0x0045_15a0, &args![tes, pos, 0u8]);
        assert_eq!(e.get(tes, TES::iCurrentGridX), 3);
    }

    #[test]
    fn save_load_flag_is_read_from_the_data_handler() {
        let mut e = Engine::new();
        let handler = e.mem.alloc(0x63c);
        assert!(!e.call(0x0045_16b0, &args![handler]).bool());
        e.mem.set_u8(handler + 0x61a, 1);
        assert!(e.call(0x0045_16b0, &args![handler]).bool());
    }

    /// A 5 x 5 grid (the centre is (2, 2)) with the given cells (x, y, flags).
    fn water_engine(cells: &[(u32, u32, u8)]) -> (Engine, Ptr<TES>, Vec<u32>) {
        let (mut e, tes, table) = grid_engine(5);
        let mut made = vec![];
        for (x, y, flags) in cells {
            let cell = new_cell(&mut e, 3, *flags);
            put_cell(&mut e, table, 5, *x, *y, cell);
            made.push(table + (x * 5 + y) * 4);
        }
        (e, tes, made)
    }

    #[test]
    fn water_search_prefers_the_centre_cell() {
        let (mut e, tes, slots) = water_engine(&[(2, 2, 2), (2, 3, 2), (1, 1, 2)]);
        assert_eq!(
            e.call(0x0045_16d0, &args![tes]).u32(),
            slots[0],
            "the centre"
        );
    }

    #[test]
    fn water_search_goes_ring_by_ring_and_stops_before_the_outermost_ring() {
        // The first ring wins over a cell further out.
        let (mut e, tes, slots) = water_engine(&[(3, 3, 2), (1, 2, 0)]);
        assert_eq!(e.call(0x0045_16d0, &args![tes]).u32(), slots[0]);
        // `ring < uGridsToLoad / 2`: the outermost ring of a 5 x 5 grid
        // (ring 2) is not searched.
        let (mut e, tes, _) = water_engine(&[(0, 4, 2), (0, 0, 2), (2, 0, 2)]);
        assert_eq!(e.call(0x0045_16d0, &args![tes]).u32(), 0);
    }

    #[test]
    fn water_search_scans_the_rows_then_the_columns_of_a_ring() {
        // Ring 1: rows y = 3 and y = 1 for x = 1..=3, then the columns x = 1
        // and x = 3 for y = 2.
        let (mut e, tes, slots) = water_engine(&[(3, 2, 2)]);
        assert_eq!(e.call(0x0045_16d0, &args![tes]).u32(), slots[0]);
        // At the same x the row at the high y comes before the low one.
        let (mut e, tes, slots) = water_engine(&[(1, 1, 2), (1, 3, 2)]);
        assert_eq!(e.call(0x0045_16d0, &args![tes]).u32(), slots[1]);
        // Rows come before columns.
        let (mut e, tes, slots) = water_engine(&[(3, 2, 2), (3, 1, 2)]);
        assert_eq!(e.call(0x0045_16d0, &args![tes]).u32(), slots[1]);
        // In the columns the low x comes before the high x.
        let (mut e, tes, slots) = water_engine(&[(3, 2, 2), (1, 2, 2)]);
        assert_eq!(e.call(0x0045_16d0, &args![tes]).u32(), slots[1]);
    }

    #[test]
    fn water_search_finds_nothing_without_water() {
        let (mut e, tes, _) = water_engine(&[(2, 2, 1), (1, 1, 0)]);
        assert_eq!(e.call(0x0045_16d0, &args![tes]).u32(), 0);
    }

    /// An engine for `fn_00451900`: a 5 x 5 grid centred on (10, 20) (cells
    /// 8..13, 18..23), the cell at (9, 19) loaded.
    fn lookup_engine() -> (Engine, Ptr<TES>, u32) {
        let (mut e, tes, table) = grid_engine(5);
        e.set(tes, TES::iCurrentGridX, 10);
        e.set(tes, TES::iCurrentGridY, 20);
        e.set(tes, TES::pWorldSpace, Ptr::new(0x6000));
        let cell = new_cell(&mut e, 3, 0);
        put_cell(&mut e, table, 5, 1, 1, cell);
        (e, tes, cell)
    }

    #[test]
    fn cell_lookup_inside_the_grid_uses_the_slot() {
        let (mut e, tes, cell) = lookup_engine();
        let log = run_logged(&mut e, |e| {
            assert_eq!(e.call(0x0045_1900, &args![tes, 9i32, 19i32]).u32(), cell);
        });
        assert!(calls_to(&log, 0x0046_1c20).is_empty());
        // Relative coordinates are passed to `GridCellArray::Get`.
        let array = e.get(tes, TES::pGridCellA).addr();
        assert_eq!(calls_to(&log, GRID_CELL_ARRAY_GET), vec![vec![array, 1, 1]]);
    }

    #[test]
    fn cell_lookup_falls_back_to_the_data_handler() {
        let (mut e, tes, _) = lookup_engine();
        returns(&mut e, 0x0046_1c20, 0x7777);
        // Outside the grid, an empty slot (8, 18 is empty) and below zero.
        for (x, y) in [(40i32, 19i32), (8, 18), (7, 18), (9, 40)] {
            let log = run_logged(&mut e, |e| {
                assert_eq!(e.call(0x0045_1900, &args![tes, x, y]).u32(), 0x7777);
            });
            assert_eq!(
                calls_to(&log, 0x0046_1c20),
                vec![vec![0x1234, x as u32, y as u32, 0x6000, 0]]
            );
        }
    }

    #[test]
    fn cell_lookup_without_a_world_space_is_null() {
        let (mut e, tes, _) = lookup_engine();
        e.set(tes, TES::pWorldSpace, Ptr::NULL);
        assert_eq!(e.call(0x0045_1900, &args![tes, 40i32, 19i32]).u32(), 0);
    }

    /// `fn_004519d0` on the lookup grid, with the `TES` singleton and the
    /// cell's load state in `state`.
    fn loaded_cell_engine(state: u8) -> (Engine, Ptr<TES>, u32) {
        let (mut e, tes, cell) = lookup_engine();
        install_ftol(&mut e);
        returns(&mut e, GET_INTERIOR_CELL, 0);
        e.map(TES_SINGLETON, 4);
        e.set_global(TES_SINGLETON, tes.addr());
        e.mem.set_u8(cell + 0x26, state);
        (e, tes, cell)
    }

    #[test]
    fn loaded_cell_at_a_position_is_the_exterior_cell_when_loaded() {
        // `TES::IsCellLoaded` with flag 1 accepts the states 5 and 6 only.
        let (mut e, tes, cell) = loaded_cell_engine(6);
        let x = 9.0f32 * 4096.0 + 100.0;
        let y = 19.0f32 * 4096.0 + 200.0;
        assert_eq!(e.call(0x0045_19d0, &args![tes, x, y, 0u32]).u32(), cell);
    }

    #[test]
    fn loaded_cell_at_a_position_is_null_when_not_loaded_or_unknown() {
        let (mut e, tes, _) = loaded_cell_engine(3);
        let x = 9.0f32 * 4096.0 + 100.0;
        let y = 19.0f32 * 4096.0 + 200.0;
        assert_eq!(e.call(0x0045_19d0, &args![tes, x, y, 0u32]).u32(), 0);
        // A cell outside the grid that the data handler does not have.
        let (mut e, tes, _) = loaded_cell_engine(6);
        returns(&mut e, 0x0046_1c20, 0);
        assert_eq!(e.call(0x0045_19d0, &args![tes, 4.0e6f32, y, 0u32]).u32(), 0);
    }

    #[test]
    fn loaded_cell_at_a_position_is_the_interior_cell_when_there_is_one() {
        let (mut e, tes, _) = loaded_cell_engine(3);
        returns(&mut e, GET_INTERIOR_CELL, 0x4444);
        assert_eq!(
            e.call(0x0045_19d0, &args![tes, 1.0f32, 2.0f32, 0u32]).u32(),
            0x4444
        );
    }

    /// An engine for `tes_load_grid_cell`: world space 0x6000, an existing
    /// cell, land at 0x7100 with lowest height 5.0 and water height 10.0.
    fn load_cell_engine() -> (Engine, Ptr<TES>, u32) {
        let (mut e, tes, _) = grid_engine(5);
        install_getters(&mut e);
        e.set(tes, TES::pWorldSpace, Ptr::new(0x6000));
        e.set(tes, TES::pObjRoot, Ptr::new(0x7000));
        let cell = new_cell(&mut e, 1, 0);
        returns(&mut e, 0x0058_5b30, cell);
        returns(&mut e, 0x004b_a5a0, 0);
        returns(&mut e, 0x0084_e3a0, 0xabcd);
        returns(&mut e, CELL_GET_DATA_X, 3);
        returns(&mut e, CELL_GET_DATA_Y, 4);
        returns(&mut e, 0x0040_1280, 0x0101_1584);
        returns(&mut e, 0x0054_6fb0, 0x7100);
        returns(&mut e, 0x0053_5b90, 1);
        e.register(0x0053_f440, |e, a| {
            e.mem.set_f32(a[1], 5.0);
            e.mem.set_f32(a[1] + 4, 99.0);
            Ret::default()
        });
        returns_float(&mut e, 0x0054_71e0, 10.0);
        noop(
            &mut e,
            &[
                0x0045_4b90,
                0x00ec_623a,
                0x004b_a550,
                GRID_CELL_ARRAY_ATTACH_TO_WORLD,
                0x0053_a090,
                0x0049_c860,
                0x0053_fa40,
                0x0058_8150,
                0x0055_1440,
            ],
        );
        put_setting(&mut e, SETTING_USE_WATER, 1u8);
        (e, tes, cell)
    }

    #[test]
    fn grid_cell_load_without_a_world_space_does_nothing() {
        let (mut e, tes, _) = load_cell_engine();
        e.set(tes, TES::pWorldSpace, Ptr::NULL);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1a50, &args![tes, 1u32, 2u32, 30i32, 40i32]);
        });
        assert_eq!(addresses(&log), vec![0x0045_1a50, GET_WORLD_SPACE]);
    }

    #[test]
    fn grid_cell_load_into_an_attached_slot_only_adds_to_the_buffer() {
        let (mut e, tes, cell) = load_cell_engine();
        returns(&mut e, 0x004b_a5a0, 1);
        let array = e.get(tes, TES::pGridCellA).addr();
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1a50, &args![tes, 1u32, 2u32, 30i32, 40i32]);
        });
        assert_eq!(calls_to(&log, 0x0058_5b30), vec![vec![0x6000, 30, 40]]);
        assert_eq!(calls_to(&log, 0x004b_a5a0), vec![vec![array, 1, 2]]);
        assert_eq!(calls_to(&log, 0x0045_4b90), vec![vec![tes.addr(), cell]]);
        assert!(calls_to(&log, 0x004b_a550).is_empty());
        assert!(calls_to(&log, 0x00ec_623a).is_empty());
    }

    #[test]
    fn grid_cell_load_attaches_the_cell_and_marks_water() {
        let (mut e, tes, cell) = load_cell_engine();
        let array = e.get(tes, TES::pGridCellA).addr();
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1a50, &args![tes, 1u32, 2u32, 30i32, 40i32]);
        });
        // Not created: the world space had the cell.
        assert!(calls_to(&log, 0x0046_1c20).is_empty());
        // The message: name, x, y, form id.
        let message = calls_to(&log, 0x00ec_623a);
        assert_eq!(
            message[0][1..],
            [LOADING_CELL_FORMAT, 0x0101_1584, 3, 4, 0xabcd]
        );
        assert_eq!(calls_to(&log, 0x004b_a550), vec![vec![array, 1, 2, cell]]);
        assert_eq!(
            calls_to(&log, GRID_CELL_ARRAY_ATTACH_TO_WORLD),
            vec![vec![array, 0x7000, 1, 2]]
        );
        assert_eq!(calls_to(&log, 0x0053_5b90), vec![vec![0x7100, 1]]);
        // Vertices loaded: no mesh update.
        assert!(calls_to(&log, 0x0053_a090).is_empty());
        // Lowest land height 5.0 below the water at 10.0: the water bit.
        assert_eq!(e.mem.u8(cell + 0x24) & 2, 2);
        // `bUseWater`, a water cell without auto water: placeable water.
        assert_eq!(calls_to(&log, 0x0049_c860), vec![vec![cell]]);
        assert_eq!(calls_to(&log, 0x0053_fa40), vec![vec![0x7100, 0]]);
        // The message buffer was a stack block, freed again.
        assert_eq!(e.mem.block_size(message[0][0]), None);
    }

    #[test]
    fn grid_cell_load_creates_a_missing_cell() {
        let (mut e, tes, _) = load_cell_engine();
        returns(&mut e, 0x0058_5b30, 0);
        let created = new_cell(&mut e, 1, 0);
        returns(&mut e, 0x0046_1c20, created);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1a50, &args![tes, 1u32, 2u32, 30i32, 40i32]);
        });
        assert_eq!(
            calls_to(&log, 0x0046_1c20),
            vec![vec![0x1234, 30, 40, 0x6000, 1]]
        );
        assert_eq!(calls_to(&log, 0x0058_8150), vec![vec![0x6000, created]]);
        assert_eq!(calls_to(&log, 0x0055_1440), vec![vec![created, 1]]);
        let array = e.get(tes, TES::pGridCellA).addr();
        assert_eq!(
            calls_to(&log, 0x004b_a550),
            vec![vec![array, 1, 2, created]]
        );
    }

    #[test]
    fn grid_cell_load_clears_the_water_bit_above_the_water_and_updates_the_mesh() {
        let (mut e, tes, cell) = load_cell_engine();
        e.mem.set_u8(cell + 0x24, 2);
        returns_float(&mut e, 0x0054_71e0, 5.0);
        returns(&mut e, 0x0053_5b90, 0);
        e.set(tes, TES::bShowLANDborders, true);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1a50, &args![tes, 1u32, 2u32, 30i32, 40i32]);
        });
        // 5.0 < 5.0 is false.
        assert_eq!(e.mem.u8(cell + 0x24) & 2, 0);
        assert!(calls_to(&log, 0x0049_c860).is_empty());
        assert_eq!(calls_to(&log, 0x0053_a090), vec![vec![0x7100, 0, 0]]);
        assert_eq!(calls_to(&log, 0x0053_fa40), vec![vec![0x7100, 1]]);
    }

    #[test]
    fn placeable_water_is_not_generated_twice_or_without_the_setting() {
        let (mut e, tes, cell) = load_cell_engine();
        e.mem.set_u8(cell + 0x54, 1); // bAutoWaterLoaded
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1a50, &args![tes, 1u32, 2u32, 30i32, 40i32]);
        });
        assert!(calls_to(&log, 0x0049_c860).is_empty());

        let (mut e, tes, _) = load_cell_engine();
        put_setting(&mut e, SETTING_USE_WATER, 0u8);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1a50, &args![tes, 1u32, 2u32, 30i32, 40i32]);
        });
        assert!(calls_to(&log, 0x0049_c860).is_empty());
    }

    #[test]
    fn cell_name_and_auto_water_accessors() {
        let mut e = Engine::new();
        returns(&mut e, 0x0040_1280, 0x0101_1584);
        let cell = new_cell(&mut e, 0, 0);
        assert_eq!(e.call(0x0045_1cb0, &args![cell]).u32(), 0x0101_1584);
        assert_eq!(e.call(0x0045_1cd0, &args![cell]).u32(), 0);
        e.mem.set_u8(cell + 0x54, 1);
        assert_eq!(e.call(0x0045_1cd0, &args![cell]).u32(), 1);
    }

    #[test]
    fn grid_lights_are_attached_for_cells_in_state_6() {
        let (mut e, tes, table) = grid_engine(2);
        noop(&mut e, &[0x0054_ba80, 0x00b5_ddb0]);
        e.map(SHADOW_SCENE_NODE_TABLE, 16);
        e.set_global(SHADOW_SCENE_NODE_TABLE, 0x7777u32);
        let six = new_cell(&mut e, 6, 0);
        let three = new_cell(&mut e, 3, 0);
        put_cell(&mut e, table, 2, 0, 1, six);
        put_cell(&mut e, table, 2, 1, 0, three);
        put_cell(&mut e, table, 2, 1, 1, six);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1cf0, &args![tes]);
        });
        assert_eq!(
            calls_to(&log, 0x0054_ba80),
            vec![vec![six, 1], vec![six, 1]]
        );
        assert_eq!(calls_to(&log, 0x00b5_ddb0), vec![vec![0x7777, 1, 1]]);
        assert_eq!(log.last().unwrap().0, 0x00b5_ddb0);
    }

    /// An actor-like reference for `fn_00451da0`: slot 0x1d0 returns the
    /// parent `0x5000`.
    fn parent_engine() -> (Engine, Ptr<TES>, u32, u32) {
        let mut e = Engine::new();
        let tes: Ptr<TES> = e.new_object();
        returns(&mut e, 0x0056_4e60, 1);
        returns(&mut e, 0x0056_51e0, 1);
        returns(&mut e, 0x0300_01d0, 0x5000);
        noop(&mut e, &[0x0057_0f70, 0x0056_5210, 0x0300_00e8]);
        let handler = object_with_slots(&mut e, 0x0300_4000, &[(0xe8, 0x0300_00e8)]);
        returns(&mut e, 0x0096_11e0, handler);
        let refr = object_with_slots(&mut e, 0x0300_3000, &[(0x1d0, 0x0300_01d0)]);
        (e, tes, refr, handler)
    }

    #[test]
    fn reference_cleanup_hands_the_parent_to_the_handler_and_clears_the_state() {
        let (mut e, tes, refr, handler) = parent_engine();
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1da0, &args![tes, refr]);
        });
        assert_eq!(calls_to(&log, 0x0096_11e0), vec![vec![0x5000]]);
        assert_eq!(calls_to(&log, 0x0300_00e8), vec![vec![handler, 0x5000]]);
        assert_eq!(calls_to(&log, 0x0057_0f70), vec![vec![refr, 0]]);
        assert_eq!(calls_to(&log, 0x0056_5210), vec![vec![refr, 0]]);
    }

    #[test]
    fn reference_cleanup_without_a_handler_skips_only_the_handler_call() {
        let (mut e, tes, refr, _) = parent_engine();
        returns(&mut e, 0x0096_11e0, 0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1da0, &args![tes, refr]);
        });
        assert!(calls_to(&log, 0x0300_00e8).is_empty());
        assert_eq!(calls_to(&log, 0x0056_5210), vec![vec![refr, 0]]);
    }

    #[test]
    fn reference_cleanup_stops_at_the_first_failed_condition() {
        // Not visible when distant.
        let (mut e, tes, refr, _) = parent_engine();
        returns(&mut e, 0x0056_4e60, 0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1da0, &args![tes, refr]);
        });
        assert_eq!(addresses(&log), vec![0x0045_1da0, 0x0056_4e60]);
        // No parent.
        let (mut e, tes, refr, _) = parent_engine();
        returns(&mut e, 0x0300_01d0, 0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1da0, &args![tes, refr]);
        });
        assert_eq!(addresses(&log), vec![0x0045_1da0, 0x0056_4e60, 0x0300_01d0]);
        // The flag is not set.
        let (mut e, tes, refr, _) = parent_engine();
        returns(&mut e, 0x0056_51e0, 0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1da0, &args![tes, refr]);
        });
        assert!(calls_to(&log, 0x0056_5210).is_empty());
    }

    #[test]
    fn reference_is_loadable_unless_one_of_the_two_tests_says_no() {
        let mut e = Engine::new();
        let tes: Ptr<TES> = e.new_object();
        returns(&mut e, REFERENCE_GET_BASE_FORM, 0x5000);
        returns(&mut e, FORM_GET_TYPE, 0x10);
        returns(&mut e, 0x0044_0d80, 0);
        returns(&mut e, 0x0044_0da0, 0);
        assert!(e.call(0x0045_1e40, &args![tes, 0x6000u32]).bool());
        returns(&mut e, 0x0044_0da0, 1);
        assert!(!e.call(0x0045_1e40, &args![tes, 0x6000u32]).bool());
        returns(&mut e, 0x0044_0da0, 0);
        returns(&mut e, 0x0044_0d80, 1);
        let log = run_logged(&mut e, |e| {
            assert!(!e.call(0x0045_1e40, &args![tes, 0x6000u32]).bool());
        });
        // The second test is not asked.
        assert!(calls_to(&log, 0x0044_0da0).is_empty());
    }

    #[test]
    fn actor_is_taken_off_the_temp_change_list() {
        let mut e = Engine::new();
        let tes: Ptr<TES> = e.new_object();
        returns(&mut e, 0x0300_0100, 1);
        noop(&mut e, &[0x0300_0260, 0x0096_f400]);
        let actor = object_with_slots(
            &mut e,
            0x0300_3000,
            &[(0x100, 0x0300_0100), (0x260, 0x0300_0260)],
        );
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1ea0, &args![tes, actor]);
        });
        assert_eq!(
            addresses(&log),
            vec![0x0045_1ea0, 0x0300_0100, 0x0300_0260, 0x0096_f400]
        );
        assert_eq!(
            calls_to(&log, 0x0096_f400),
            vec![vec![OBJECT_011E0E80, actor]]
        );
        // Not an actor: nothing more.
        returns(&mut e, 0x0300_0100, 0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1ea0, &args![tes, actor]);
        });
        assert_eq!(addresses(&log), vec![0x0045_1ea0, 0x0300_0100]);
    }

    /// An engine for `fn_00451ef0` with a non-actor reference of form type
    /// 0x10 whose slot 0x1c8 returns the node `0x9001`, in a cell of state 6.
    /// Returns the engine, the `TES`, the reference and the cell.
    fn reference_engine() -> (Engine, Ptr<TES>, u32, u32) {
        let mut e = Engine::new();
        let tes: Ptr<TES> = e.new_object();
        e.map(SHADOW_SCENE_NODE_TABLE, 16);
        e.set_global(SHADOW_SCENE_NODE_TABLE, 0x7777u32);
        for addr in [SCRIPT_CONTEXT, PLAYER_CHARACTER] {
            e.map(addr, 4);
        }
        e.set_global(PLAYER_CHARACTER, 0x3333u32);
        e.map(LARGE_CELL_LIMIT, 8);
        e.set_global(LARGE_CELL_LIMIT, 200.0f64);
        noop(
            &mut e,
            &[
                0x0057_0f70,
                0x0054_5cf0,
                0x00b5_eeb0,
                0x00b5_cbd0,
                0x0054_8880,
                0x0056_f700,
                0x0057_8170,
                0x0057_8060,
                0x0048_3710,
                0x0045_34f0,
                0x00b5_d9f0,
                0x0056_5730,
                0x005a_c190,
                0x005a_c750,
                0x008c_4640,
                0x004b_6dc0,
                0x0054_7ad0,
                0x0300_0308,
                0x0300_0304,
                0x0300_0260,
                0x0300_0040,
                0x0300_00e8,
                0x0096_f400,
            ],
        );
        returns(&mut e, REFERENCE_GET_BASE_FORM, 0x5000);
        returns(&mut e, FORM_GET_TYPE, 0x10);
        for addr in [
            0x0056_4e60,
            0x0044_0d80,
            0x0044_0da0,
            0x0057_b200,
            0x0045_2440,
            0x0300_0224,
            0x0300_0220,
            0x0300_0100,
            0x0096_11e0,
            0x005d_8710,
            0x0056_51e0,
            0x0300_01d0,
        ] {
            returns(&mut e, addr, 0);
        }
        returns(&mut e, 0x0043_fcd0, 0x9100);
        returns(&mut e, 0x0300_01c8, 0x9001);
        returns(&mut e, 0x0042_ce10, 1);
        returns(&mut e, 0x0084_e3a0, 0x1010);
        returns(&mut e, 0x0043_0830, 0x2020);
        returns(&mut e, 0x005d_43c0, 0x44);
        returns(&mut e, 0x0045_2480, 1);
        returns(&mut e, 0x00ec_43fb, 0);
        let refr = object_with_slots(
            &mut e,
            0x0300_3000,
            &[
                (0x100, 0x0300_0100),
                (0x1c8, 0x0300_01c8),
                (0x1d0, 0x0300_01d0),
                (0x220, 0x0300_0220),
                (0x224, 0x0300_0224),
                (0x260, 0x0300_0260),
                (0x304, 0x0300_0304),
                (0x308, 0x0300_0308),
            ],
        );
        // Loaded data with flag 2, read through `pLoadedData` (+0x64).
        let loaded_data = e.mem.alloc(0x20);
        e.mem.set_u32(loaded_data + 0x10, 2);
        e.mem.set_u32(refr + 0x64, loaded_data);
        let cell = new_cell(&mut e, 6, 0);
        (e, tes, refr, cell)
    }

    #[test]
    fn loading_a_null_reference_does_nothing() {
        let (mut e, tes, _, cell) = reference_engine();
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1ef0, &args![tes, 0u32, cell, 0u32, 1u8]);
        });
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn loading_a_reference_with_slot_0x224_or_0x220_calls_the_matching_slot_only() {
        let (mut e, tes, refr, cell) = reference_engine();
        returns(&mut e, 0x0300_0224, 1);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1ef0, &args![tes, refr, cell, 0u32, 1u8]);
        });
        assert_eq!(addresses(&log), vec![0x0045_1ef0, 0x0300_0224, 0x0300_0308]);
        returns(&mut e, 0x0300_0224, 0);
        returns(&mut e, 0x0300_0220, 1);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1ef0, &args![tes, refr, cell, 0u32, 1u8]);
        });
        assert_eq!(
            addresses(&log),
            vec![0x0045_1ef0, 0x0300_0224, 0x0300_0220, 0x0300_0304]
        );
    }

    #[test]
    fn loading_a_plain_reference_runs_the_whole_sequence() {
        let (mut e, tes, refr, cell) = reference_engine();
        // The action list is set up when `0042ce10` says no.
        returns(&mut e, 0x0042_ce10, 0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1ef0, &args![tes, refr, cell, 0u32, 1u8]);
        });
        assert_eq!(
            addresses(&log),
            vec![
                0x0045_1ef0,
                0x0300_0224,
                0x0300_0220,
                // The cleanup (`fn_00451da0`) asks `00564e60` and stops.
                0x0056_4e60,
                // The base form and its type.
                REFERENCE_GET_BASE_FORM,
                FORM_GET_TYPE,
                // Whether to load it (`fn_00451e40`).
                REFERENCE_GET_BASE_FORM,
                FORM_GET_TYPE,
                0x0044_0d80,
                0x0044_0da0,
                // The 3D from slot 0x1c8.
                0x0300_01c8,
                // The cell's 3D, then the node into the shadow scene node.
                0x0054_5cf0,
                0x00b5_eeb0,
                0x0300_0100,
                0x0056_4e60,
                REFERENCE_GET_BASE_FORM,
                FORM_GET_TYPE,
                0x0054_8880,
                0x0056_f700,
                // The master particle addon nodes (loaded-data flag 2, state 6).
                0x0057_8060,
                0x0300_0100,
                0x00b5_d9f0,
                // The script.
                0x0056_5730,
                0x0042_ce10,
                0x005d_43c0,
                0x005a_c190,
                0x005d_43c0,
                0x005a_c750,
                0x0300_0100,
            ]
        );
        assert_eq!(calls_to(&log, 0x0300_01c8), vec![vec![refr, 0]]);
        assert_eq!(calls_to(&log, 0x0054_5cf0), vec![vec![cell]]);
        assert_eq!(calls_to(&log, 0x00b5_eeb0), vec![vec![0x7777, 0x9001]]);
        assert_eq!(calls_to(&log, 0x0054_8880), vec![vec![cell, refr, 0]]);
        assert_eq!(calls_to(&log, 0x0056_f700), vec![vec![refr]]);
        assert_eq!(calls_to(&log, 0x0057_8060), vec![vec![0x9001]]);
        assert_eq!(calls_to(&log, 0x00b5_d9f0), vec![vec![0x7777, 0x9001, 0]]);
        assert_eq!(calls_to(&log, 0x0056_5730), vec![vec![refr]]);
        assert_eq!(calls_to(&log, 0x005a_c190), vec![vec![refr, 0x44]]);
        // The third argument is the 0x1000 pushed before the second
        // `005d43c0` call.
        assert_eq!(calls_to(&log, 0x005a_c750), vec![vec![refr, 0x44, 0x1000]]);
    }

    #[test]
    fn loading_an_actor_registers_its_node_and_sets_the_reference_id() {
        let (mut e, tes, refr, cell) = reference_engine();
        returns(&mut e, 0x0300_0100, 1);
        returns(&mut e, 0x0056_4e60, 1);
        let log = run_logged(&mut e, |e| {
            // With the flag 0 the actor is first taken off the list.
            e.call(0x0045_1ef0, &args![tes, refr, cell, 0u32, 0u8]);
        });
        assert_eq!(calls_to(&log, 0x0300_0260), vec![vec![refr]]);
        assert_eq!(
            calls_to(&log, 0x0096_f400),
            vec![vec![OBJECT_011E0E80, refr]]
        );
        assert_eq!(calls_to(&log, 0x00b5_cbd0), vec![vec![0x7777, 0x9001]]);
        assert_eq!(calls_to(&log, 0x0048_3710), vec![vec![refr]]);
        // The counter of references visible when distant goes up.
        assert_eq!(e.mem.u16(cell + 0xaa), 1);
        assert_eq!(calls_to(&log, 0x008c_4640), vec![vec![refr]]);
        assert_eq!(calls_to(&log, 0x0084_e3a0), vec![vec![refr]]);
        assert_eq!(calls_to(&log, 0x004b_6dc0), vec![vec![0x9100, 0x1010]]);
        // The last call is the reference-ID call.
        assert_eq!(log.last().unwrap().0, 0x004b_6dc0);
    }

    #[test]
    fn the_player_does_not_get_a_reference_id_on_the_scene_graph() {
        let (mut e, tes, refr, cell) = reference_engine();
        returns(&mut e, 0x0300_0100, 1);
        e.set_global(PLAYER_CHARACTER, refr);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1ef0, &args![tes, refr, cell, 0u32, 1u8]);
        });
        assert_eq!(calls_to(&log, 0x008c_4640), vec![vec![refr]]);
        assert!(calls_to(&log, 0x004b_6dc0).is_empty());
    }

    #[test]
    fn a_reference_that_is_not_to_be_loaded_stops_after_the_cleanup() {
        let (mut e, tes, refr, cell) = reference_engine();
        returns(&mut e, 0x0044_0d80, 1);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1ef0, &args![tes, refr, cell, 0u32, 1u8]);
        });
        assert!(calls_to(&log, 0x0300_01c8).is_empty());
        assert!(calls_to(&log, 0x0056_5730).is_empty());
    }

    #[test]
    fn a_node_from_the_source_is_attached_to_the_reference_and_the_source_is_notified() {
        let (mut e, tes, refr, cell) = reference_engine();
        let source = object_with_slots(&mut e, 0x0300_5000, &[(0x40, 0x0300_0040)]);
        returns(&mut e, 0x005d_8710, 0xa001);
        returns(&mut e, 0x0043_fcd0, 0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1ef0, &args![tes, refr, cell, source, 1u8]);
        });
        assert_eq!(
            calls_to(&log, 0x005d_8710),
            vec![vec![source], vec![source]]
        );
        assert_eq!(calls_to(&log, 0x0057_0f70), vec![vec![refr, 0xa001]]);
        assert_eq!(calls_to(&log, 0x0300_0040), vec![vec![source]]);
        assert!(calls_to(&log, 0x0300_01c8).is_empty());
        assert_eq!(calls_to(&log, 0x0054_8880), vec![vec![cell, refr, 0]]);
        // The node already attached to the reference is not set again.
        returns(&mut e, 0x0043_fcd0, 0x9100);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1ef0, &args![tes, refr, cell, source, 1u8]);
        });
        assert!(calls_to(&log, 0x0057_0f70).is_empty());
    }

    #[test]
    fn a_source_without_a_node_falls_back_to_slot_0x1c8() {
        let (mut e, tes, refr, cell) = reference_engine();
        let source = object_with_slots(&mut e, 0x0300_5000, &[(0x40, 0x0300_0040)]);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1ef0, &args![tes, refr, cell, source, 1u8]);
        });
        assert_eq!(calls_to(&log, 0x0300_01c8), vec![vec![refr, 0]]);
        assert_eq!(calls_to(&log, 0x0300_0040), vec![vec![source]]);
    }

    #[test]
    fn form_types_0x2c_and_0x2d_hand_the_node_to_the_handler() {
        for kind in [0x2c, 0x2d] {
            let (mut e, tes, refr, cell) = reference_engine();
            returns(&mut e, FORM_GET_TYPE, kind);
            let handler = object_with_slots(&mut e, 0x0300_4000, &[(0xe8, 0x0300_00e8)]);
            returns(&mut e, 0x0096_11e0, handler);
            let log = run_logged(&mut e, |e| {
                e.call(0x0045_1ef0, &args![tes, refr, cell, 0u32, 1u8]);
            });
            assert_eq!(calls_to(&log, 0x0300_00e8), vec![vec![handler, 0x9001]]);
            assert!(calls_to(&log, 0x0054_5cf0).is_empty());
            assert!(calls_to(&log, 0x0054_8880).is_empty());
            // The script is still initialised.
            assert_eq!(calls_to(&log, 0x0056_5730), vec![vec![refr]]);
        }
    }

    #[test]
    fn type_0x23_forms_that_00452440_rejects_are_not_attached() {
        let (mut e, tes, refr, cell) = reference_engine();
        returns(&mut e, FORM_GET_TYPE, 0x23);
        returns(&mut e, 0x0045_2440, 1);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1ef0, &args![tes, refr, cell, 0u32, 1u8]);
        });
        assert!(calls_to(&log, 0x0054_8880).is_empty());
        assert!(calls_to(&log, 0x0056_f700).is_empty());
        assert_eq!(calls_to(&log, 0x0045_2440), vec![vec![0x5000]]);
        // Accepted by `00452440`: attached as usual.
        returns(&mut e, 0x0045_2440, 0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1ef0, &args![tes, refr, cell, 0u32, 1u8]);
        });
        assert_eq!(calls_to(&log, 0x0054_8880).len(), 1);
    }

    #[test]
    fn type_0x1e_forms_use_the_extra_data_call_instead_of_the_lighting_update() {
        let (mut e, tes, refr, cell) = reference_engine();
        returns(&mut e, FORM_GET_TYPE, 0x1e);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1ef0, &args![tes, refr, cell, 0u32, 1u8]);
        });
        assert_eq!(calls_to(&log, 0x0043_0830), vec![vec![refr]]);
        assert_eq!(calls_to(&log, 0x0045_34f0), vec![vec![0x5000, 0x2020]]);
        assert!(calls_to(&log, 0x00b5_d9f0).is_empty());
    }

    #[test]
    fn master_particle_addon_nodes_are_removed_or_added() {
        // Removed: the form flag and `0057b200`.
        let (mut e, tes, refr, cell) = reference_engine();
        e.mem.set_u32(refr + 8, 0x0100_0000);
        returns(&mut e, 0x0057_b200, 1);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1ef0, &args![tes, refr, cell, 0u32, 1u8]);
        });
        assert_eq!(calls_to(&log, 0x0057_8170), vec![vec![0x9001]]);
        assert!(calls_to(&log, 0x0057_8060).is_empty());
        // The flag without `0057b200`: the add condition applies.
        returns(&mut e, 0x0057_b200, 0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1ef0, &args![tes, refr, cell, 0u32, 1u8]);
        });
        assert!(calls_to(&log, 0x0057_8170).is_empty());
        assert_eq!(calls_to(&log, 0x0057_8060), vec![vec![0x9001]]);
        // Added in a cell of state 5, not in one of state 4 or without the
        // loaded-data bit.
        e.mem.set_u8(cell + 0x26, 5);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1ef0, &args![tes, refr, cell, 0u32, 1u8]);
        });
        assert_eq!(calls_to(&log, 0x0057_8060).len(), 1);
        e.mem.set_u8(cell + 0x26, 4);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1ef0, &args![tes, refr, cell, 0u32, 1u8]);
        });
        assert!(calls_to(&log, 0x0057_8060).is_empty());
        e.mem.set_u8(cell + 0x26, 6);
        let loaded_data = e.mem.u32(refr + 0x64);
        e.mem.set_u32(loaded_data + 0x10, 4);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1ef0, &args![tes, refr, cell, 0u32, 1u8]);
        });
        assert!(calls_to(&log, 0x0057_8060).is_empty());
    }

    /// `fn_00451ef0` on a form of type 0x25 whose base form casts to `0xC000`
    /// (slot 0x1bc returns 2.5) with the two floats `first` and `second` at
    /// `+0x8c`; the reference's slot 0x1f4 returns the position (1, 2, 3) and
    /// `00567400` its scale 2.0.
    fn run_type_0x25(first: f32, second: f32, cast_exists: bool) -> Vec<(u32, Vec<u32>)> {
        let (mut e, tes, refr, cell) = reference_engine();
        returns(&mut e, FORM_GET_TYPE, 0x25);
        let cast = object_with_slots(&mut e, 0x0300_6000, &[(0x1bc, 0x0300_01bc)]);
        returns_float(&mut e, 0x0300_01bc, 2.5);
        returns(&mut e, 0x00ec_43fb, if cast_exists { cast } else { 0 });
        let sizes = e.mem.alloc(8);
        e.mem.set_f32(sizes, first);
        e.mem.set_f32(sizes + 4, second);
        returns(&mut e, 0x0045_bb80, sizes);
        returns_float(&mut e, 0x0056_7400, 2.0);
        let position = e.mem.alloc(12);
        for (i, value) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            e.mem.set_f32(position + 4 * i as u32, value);
        }
        e.mem.set_u32(0x0300_3000 + 0x1f4, 0x0300_01f4);
        returns(&mut e, 0x0300_01f4, position);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_1ef0, &args![tes, refr, cell, 0u32, 1u8]);
        });
        let casts = calls_to(&log, 0x00ec_43fb);
        assert_eq!(
            casts,
            vec![vec![0x5000, 0, RTTI_TESFORM, RTTI_CAST_TARGET, 0]]
        );
        // Every call is made with the cell the caller passed.
        for words in calls_to(&log, 0x0054_7ad0) {
            assert_eq!(words[0], cell);
        }
        log
    }

    #[test]
    fn type_0x25_forms_call_00547ad0_with_the_position_and_the_scaled_radius() {
        let log = run_type_0x25(300.0, 250.0, true);
        let calls = calls_to(&log, 0x0054_7ad0);
        assert_eq!(calls.len(), 1);
        // 2.0 * 2.5 = 5, then the literal 1.0.
        assert_eq!(
            calls[0][1..],
            [
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                5,
                1.0f32.to_bits()
            ]
        );
    }

    #[test]
    fn type_0x25_forms_with_small_sizes_or_no_cast_make_no_call() {
        for (first, second, cast) in [
            (200.0, 300.0, true),
            (300.0, 200.0, true),
            (f32::NAN, 300.0, true),
            (300.0, 300.0, false),
        ] {
            let log = run_type_0x25(first, second, cast);
            assert!(calls_to(&log, 0x0054_7ad0).is_empty(), "{first} {second}");
        }
    }

    #[test]
    fn small_reference_and_cell_accessors() {
        let mut e = Engine::new();
        let form = e.mem.alloc(0x20);
        assert!(!e.call(0x0045_2370, &args![form]).bool());
        e.mem.set_u32(form + 8, 0x0100_0000);
        assert!(e.call(0x0045_2370, &args![form]).bool());
        e.mem.set_u32(form + 8, 0xfeff_ffff);
        assert!(!e.call(0x0045_2370, &args![form]).bool());

        let cell = new_cell(&mut e, 0, 0);
        e.mem.set_u16(cell + 0xaa, 0xffff);
        e.call(0x0045_2390, &args![cell]);
        assert_eq!(e.mem.u16(cell + 0xaa), 0);
        e.call(0x0045_2390, &args![cell]);
        assert_eq!(e.mem.u16(cell + 0xaa), 1);
    }

    #[test]
    fn loaded_data_flags_are_tested_with_a_mask() {
        let mut e = Engine::new();
        let refr = e.mem.alloc(0x80);
        // No loaded data.
        assert!(!e.call(0x0045_23e0, &args![refr, 0xffff_ffffu32]).bool());
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(refr.wrapping_add(0x64), data);
        e.mem.set_u32(data + 0x10, 0b0110);
        assert!(e.call(0x0045_23e0, &args![refr, 2u32]).bool());
        assert!(e.call(0x0045_23e0, &args![refr, 0b1100u32]).bool());
        assert!(!e.call(0x0045_23e0, &args![refr, 0b1001u32]).bool());
        assert!(!e.call(0x0045_2420, &args![data, 0u32]).bool());
        assert!(e.call(0x0045_2420, &args![data, 4u32]).bool());
    }

    // ---- Session 3: the cell update code (00452440 to 00454af0) ----

    /// `abs` as the CRT computes it.
    fn install_abs(e: &mut Engine) {
        e.register(ABS, |_, a| Ret {
            eax: (a[0] as i32).wrapping_abs() as u32,
            ..Ret::default()
        });
    }

    /// The exe functions the cell-update code calls that a test does not
    /// care about: each is made to return 0 (a test overrides the few that
    /// matter).
    const UPDATE_CALLEES: &[u32] = &[
        0x0040_8da0,
        0x0040_fba0,
        0x0040_fbf0,
        0x0041_6870,
        0x0041_b9a0,
        0x0042_5fd0,
        0x0042_ce10,
        0x0044_7950,
        0x0044_ddc0,
        0x0044_edb0,
        0x0045_3850,
        0x0045_3860,
        0x0045_3940,
        0x0045_4b10,
        0x0045_4b30,
        0x0045_4b50,
        0x0045_4b70,
        0x0045_4b90,
        0x0045_4bd0,
        0x0045_4c20,
        0x0045_4d50,
        0x0045_4e20,
        0x0045_4fc0,
        0x0045_5200,
        0x0045_5330,
        0x0045_5400,
        0x0045_68c0,
        0x0045_7070,
        0x0045_72e0,
        0x0045_7620,
        0x0045_7be0,
        0x0045_7d70,
        0x0045_81e0,
        0x0045_8200,
        0x0045_9870,
        0x0045_bb80,
        0x0045_c780,
        0x0045_c840,
        0x0045_cda0,
        0x0045_cec0,
        0x0046_0140,
        0x0046_0360,
        0x0046_1c20,
        0x0046_2290,
        0x0047_0470,
        0x0047_c850,
        0x004b_a7d0,
        0x004b_a9a0,
        0x004b_aec0,
        0x0052_6270,
        0x0052_8110,
        0x0052_81f0,
        0x0052_82d0,
        0x0052_83c0,
        0x0052_8540,
        0x0052_85a0,
        0x0052_d3b0,
        0x0054_5cf0,
        0x0054_6c20,
        0x0054_7610,
        0x0054_af40,
        0x0054_ba80,
        0x0054_bcf0,
        0x0055_0340,
        0x0055_1890,
        0x0055_2970,
        0x0055_2bd0,
        0x0055_7090,
        0x0055_9450,
        0x0057_d0a0,
        0x0058_75a0,
        0x0058_eeb0,
        0x005a_e380,
        0x005a_e3d0,
        0x005d_4a40,
        0x0062_3430,
        0x0062_3640,
        0x0062_4700,
        0x0062_71b0,
        0x0062_8da0,
        0x0062_f460,
        0x0063_3c90,
        0x0063_ac70,
        0x0063_ddb0,
        0x0066_4cd0,
        0x0067_33e0,
        0x0068_15c0,
        0x0068_a830,
        0x006c_0720,
        0x006c_39c0,
        0x006f_ca90,
        0x006f_cdb0,
        0x006f_ce00,
        0x006f_d1f0,
        0x0070_0960,
        0x0070_2680,
        0x0070_38b0,
        0x0070_50d0,
        0x0070_5e80,
        0x0070_ec90,
        0x0082_5c00,
        0x0084_92b0,
        0x0084_d030,
        0x0084_e3a0,
        0x0085_8af0,
        0x0085_f850,
        0x0086_1ea0,
        0x0086_ff70,
        0x0087_1dc0,
        0x0087_27b0,
        0x0087_aa90,
        0x008d_7dc0,
        0x0095_0bb0,
        0x0096_11e0,
        0x0097_2bb0,
        0x0097_2d30,
        0x0097_5080,
        0x0097_5f90,
        0x0099_38b0,
        0x009d_9f20,
        0x00a5_a040,
        0x00a6_1cd0,
        0x00b4_f5c0,
        0x00b4_f9a0,
        0x00b5_8070,
        0x00b5_cbd0,
        0x00b5_d180,
        0x00b5_ddf0,
        0x00b5_fd60,
        0x00b6_2a20,
        0x00b6_31d0,
        0x00b6_5570,
        0x00c3_dfa0,
        0x00c4_68c0,
        0x00c6_6300,
        0x00c6_6310,
        0x00c6_8f00,
        0x00c6_a640,
        0x00c6_b540,
        0x00c7_4550,
        0x00ec_17c0,
        0x0066_b0d0,
        SCOPE_GUARD_CTOR,
        SCOPE_GUARD_DTOR,
        UPDATE_OBJECT_CTOR,
        NODE_UPDATE,
        GRID_CELL_ARRAY_ATTACH_TO_WORLD,
        0x0058_5b30,
        0x0058_8150,
        0x0055_1440,
        0x004b_a5a0,
        0x004b_a550,
        0x0054_6fb0,
        0x0053_5b90,
        0x0053_a090,
        0x0053_f440,
        0x0054_71e0,
        0x0054_4c30,
        0x0054_4c60,
        0x00ec_623a,
        0x0049_c860,
        0x0053_fa40,
        0x0040_1280,
    ];

    /// `0042ce10` reads bit 1 of the word at `+0x244` of the script context
    /// object; this installs the object (all flags clear) and that double.
    fn install_script_context(e: &mut Engine) -> u32 {
        e.register(0x0042_ce10, |e, a| Ret {
            eax: (e.mem.u32(a[0] + 0x244) & 2 != 0) as u32,
            ..Ret::default()
        });
        e.map(SCRIPT_CONTEXT, 4);
        let context = e.mem.alloc(0x300);
        e.set_global(SCRIPT_CONTEXT, context);
        context
    }

    /// An engine for the cell-update functions: `grids` x `grids` grid
    /// around cell (10, 20), the grid centre queued there too, a world space,
    /// the exterior havok world `0x8000`, the exterior cell loader `0x5000`,
    /// doubles for every callee, and the constants the code reads. Returns
    /// the engine, the `TES` (also the singleton), the script context
    /// object and the grid slot table.
    fn update_engine(grids: u32) -> (Engine, Ptr<TES>, u32, u32) {
        let (mut e, tes, table) = grid_engine(grids);
        noop(&mut e, UPDATE_CALLEES);
        install_getters(&mut e);
        install_float_to_int(&mut e);
        install_abs(&mut e);
        let context = install_script_context(&mut e);
        for (addr, len) in [
            (GRID_FLAG_MOVED, 1),
            (GRID_FLAG_MOVED_COPY, 1),
            (GRID_FLAG_TERRAIN, 1),
            (GRID_FLOAT_011F94AC, 4),
            (FLOAT_PAIR_011F940C, 8),
            (GRID_CENTRE, 8),
            (GRID_PREVIOUS_CENTRE, 8),
            (GRID_CENTRE_REFERENCE, 8),
            (EXTERIOR_CELL_LOADER, 4),
            (PLAYER_CHARACTER, 4),
            (FLAG_011AF70C, 1),
            (SAVE_LOAD_GAME, 4),
            (EXTERIOR_WORLD, 4),
            (FLOAT_CELL_EXTENT, 4),
            (FLOAT_CELL_EXTENT_SMALL_GRID, 4),
            (DOUBLE_ZERO, 8),
            (DOUBLE_HALF, 8),
            (DOUBLE_CELL_SIZE, 8),
            (PURGE_ALLOWED, 1),
            (TES_SINGLETON, 4),
            (GRID_LOAD_IN_PROGRESS, 1),
            (GRID_FLAG_01189184, 1),
            (GRID_FLAG_011AD86C, 1),
            (MODEL_LOADER, 4),
            (IO_MANAGER, 4),
            (FADER_MANAGER, 4),
            (MAIN_OBJECT, 4),
            (MOVIE_PLAYER, 4),
            (FLAG_011F9427, 1),
            (FLAG_01189625, 1),
            (TIME_ACCUMULATOR, 4),
            (POINT_011A9478, 12),
            (COLLISION_LISTENER, 4),
            (TASK_QUEUE_OBJECT, 4),
        ] {
            e.map(addr, len);
        }
        e.set_global(FLOAT_CELL_EXTENT, 4096.0f32);
        e.set_global(FLOAT_CELL_EXTENT_SMALL_GRID, 3072.0f32);
        e.set_global(DOUBLE_ZERO, 0.0f64);
        e.set_global(DOUBLE_HALF, 0.5f64);
        e.set_global(DOUBLE_CELL_SIZE, 4096.0f64);
        e.set_global(FLOAT_PAIR_011F940C, 1.5f32);
        e.set_global(FLOAT_PAIR_011F940C + 4, 2.5f32);
        put_setting(&mut e, SETTING_BACKGROUND_CELL_LOADS, 1u8);
        put_setting(&mut e, SETTING_PREEMPTIVELY_UNLOAD_CELLS, 0u8);
        put_setting(&mut e, SETTING_FADE_TO_BLACK_FADE_SECONDS, 0.25f32);
        let handler = e.mem.alloc(0x700);
        e.set_global(DATA_HANDLER, handler);
        e.set_global(TES_SINGLETON, tes.addr());
        e.set_global(EXTERIOR_CELL_LOADER, 0x5000u32);
        // The first word of `spExteriorWorld` is the havok world.
        e.set_global(EXTERIOR_WORLD, 0x8000u32);
        // The grid cell array has its slot 0x10 (move to a new centre).
        let array = e.get(tes, TES::pGridCellA).addr();
        e.put_vtable(0x0300_1000, &[0, 0, 0, 0, 0x0300_0010]);
        e.mem.set_u32(array, 0x0300_1000);
        noop(&mut e, &[0x0300_0010]);
        returns(&mut e, GET_INTERIOR_CELL, 0);
        returns(&mut e, GET_TERRAIN_MANAGER, 0x7001);
        returns(&mut e, 0x0045_bb80, 0x7000);
        returns(&mut e, 0x0045_7070, 0x6100);
        // `TES::LoadGridCell` finds its grid slot already filled.
        returns(&mut e, 0x0058_5b30, 0x7777);
        returns(&mut e, 0x004b_a5a0, 1);
        e.map(0x8000, 0x100);
        e.set(tes, TES::pWorldSpace, Ptr::new(0x6000));
        e.set(tes, TES::iCurrentGridX, 10);
        e.set(tes, TES::iCurrentGridY, 20);
        e.set(tes, TES::iCurrentQueuedX, 10);
        e.set(tes, TES::iCurrentQueuedY, 20);
        (e, tes, context, table)
    }

    /// The number of calls to `addr` in `log`.
    fn count_calls(log: &[(u32, Vec<u32>)], addr: u32) -> usize {
        calls_to(log, addr).len()
    }

    /// A player whose slot `0x1f4` returns `0x8888` and slot `0x1d0` returns
    /// `fade`.
    fn install_player(e: &mut Engine, fade: u32) {
        let player = object_with_slots(
            e,
            0x0300_2000,
            &[(0x1f4, 0x0300_01f4), (0x1d0, 0x0300_01d0)],
        );
        e.set_global(PLAYER_CHARACTER, player);
        returns(e, 0x0300_01f4, 0x8888);
        returns(e, 0x0300_01d0, fade);
    }

    #[test]
    fn form_flag_tests_read_the_flags_word_the_getter_points_at() {
        let mut e = Engine::new();
        e.register(0x0062_4700, |_, a| Ret {
            eax: a[0] + 0x48,
            ..Ret::default()
        });
        let form = e.mem.alloc(0x60);
        e.mem.set_u32(form + 0x48, 0x4000_0000);
        assert!(e.call(0x0045_2440, &args![form]).bool());
        assert!(e.call(0x0045_2460, &args![form, 0x4000_0000u32]).bool());
        assert!(!e.call(0x0045_2460, &args![form, 0x0000_0001u32]).bool());
        e.mem.set_u32(form + 0x48, 0xbfff_ffff);
        assert!(!e.call(0x0045_2440, &args![form]).bool());
        e.mem.set_u32(form + 0x48, 0x0000_0003);
        assert!(e.call(0x0045_2460, &args![form, 0x0000_0001u32]).bool());
    }

    #[test]
    fn byte_getter_returns_the_global() {
        let mut e = Engine::new();
        e.map(FLAG_0119B8C8, 1);
        assert_eq!(e.call(0x0045_2480, &args![]).u8(), 0);
        e.set_global(FLAG_0119B8C8, 0x5au8);
        assert_eq!(e.call(0x0045_2480, &args![]).u8(), 0x5a);
    }

    /// An engine for the texture clean-up: purging allowed (`purge_byte`),
    /// the script check (`005d4a40`) answering `check`.
    fn cleanup_engine(purge_byte: u8, check: u32) -> (Engine, Ptr<TES>) {
        let mut e = Engine::new();
        e.map(PURGE_ALLOWED, 1);
        e.set_global(PURGE_ALLOWED, purge_byte);
        returns(&mut e, 0x005d_4a40, check);
        noop(
            &mut e,
            &[0x0040_fbf0, 0x0040_fba0, 0x0066_4cd0, 0x00a6_1cd0],
        );
        let tes: Ptr<TES> = e.new_object();
        (e, tes)
    }

    #[test]
    fn texture_cleanup_does_nothing_when_purging_is_not_allowed() {
        let (mut e, tes) = cleanup_engine(1, 0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_2490, &args![tes, 1u8]);
        });
        assert_eq!(addresses(&log), vec![0x0045_2490, 0x005d_4a40]);
    }

    #[test]
    fn texture_cleanup_purges_only_when_the_setting_or_the_flag_says_so() {
        let (mut e, tes) = cleanup_engine(0, 0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_2490, &args![tes, 0u8]);
        });
        assert_eq!(
            addresses(&log),
            vec![
                0x0045_2490,
                0x0040_fbf0,
                0x0066_4cd0,
                0x0066_4cd0,
                0x0040_fba0
            ]
        );
        assert_eq!(calls_to(&log, 0x0040_fbf0), vec![vec![CLEANUP_LOCK, 0]]);
        assert_eq!(calls_to(&log, 0x0040_fba0), vec![vec![CLEANUP_LOCK]]);
        assert_eq!(calls_to(&log, 0x0066_4cd0), vec![vec![1], vec![1]]);
        // The flag argument forces the purge.
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_2490, &args![tes, 1u8]);
        });
        assert_eq!(
            addresses(&log),
            vec![
                0x0045_2490,
                0x0040_fbf0,
                0x0066_4cd0,
                0x00a6_1cd0,
                0x0066_4cd0,
                0x0040_fba0
            ]
        );
        // So does `bAllowUnusedPurge`.
        e.set(tes, TES::bAllowUnusedPurge, true);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_2490, &args![tes, 0u8]);
        });
        assert_eq!(count_calls(&log, 0x00a6_1cd0), 1);
    }

    #[test]
    fn purge_flag_getter_reads_the_field() {
        let mut e = Engine::new();
        let tes: Ptr<TES> = e.new_object();
        assert_eq!(e.call(0x0045_24f0, &args![tes]).u8(), 0);
        e.set(tes, TES::bAllowUnusedPurge, true);
        assert_eq!(e.call(0x0045_24f0, &args![tes]).u8(), 1);
        // Any other byte comes back as stored (#198).
        e.mem.set_u8(tes.addr() + 0xb5, 0xec);
        assert_eq!(e.call(0x0045_24f0, &args![tes]).u8(), 0xec);
    }

    #[test]
    fn cleanup_lock_wrappers_use_the_lock_object() {
        let mut e = Engine::new();
        noop(&mut e, &[0x0040_fbf0, 0x0040_fba0]);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_2510, &args![]);
            e.call(0x0045_2530, &args![]);
        });
        assert_eq!(calls_to(&log, 0x0040_fbf0), vec![vec![0x011f_4480, 0]]);
        assert_eq!(calls_to(&log, 0x0040_fba0), vec![vec![0x011f_4480]]);
    }

    #[test]
    fn purge_is_allowed_unless_the_byte_is_set_and_the_check_fails() {
        for (byte, check, allowed) in [(0u8, 0u32, 1u8), (0, 1, 1), (1, 1, 1), (1, 0, 0)] {
            let (mut e, tes) = cleanup_engine(byte, check);
            assert_eq!(
                e.call(0x0045_2540, &args![tes]).u8(),
                allowed,
                "byte {byte} check {check}"
            );
        }
    }

    /// 0045254f: `005d4a40` is called with no word pushed.
    #[test]
    fn purge_check_passes_no_word() {
        let (mut e, tes) = cleanup_engine(1, 1);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_2540, &args![tes]);
        });
        assert_eq!(calls_to(&log, 0x005d_4a40), vec![Vec::<u32>::new()]);
    }

    #[test]
    fn grid_update_stops_without_a_data_handler() {
        let (mut e, tes, _, _) = update_engine(5);
        e.set_global(DATA_HANDLER, 0u32);
        e.set_global(GRID_FLAG_MOVED, 1u8);
        e.set(tes, TES::spLoadedAreaBound, Ptr::new(0x9000));
        let pos = position(&mut e, 0.0, 0.0);
        let array = e.get(tes, TES::pGridCellA).addr();
        let log = run_logged(&mut e, |e| {
            assert!(!e.call(0x0045_2580, &args![tes, pos, 0u8]).bool());
        });
        // The last-loaded list is cleared, and the moved flag (set while the
        // second flag is clear and no interior is loaded) is reset.
        assert_eq!(calls_to(&log, 0x0047_0470), vec![vec![0x7000]]);
        assert_eq!(e.global::<u8>(GRID_FLAG_MOVED), 0);
        assert_eq!(calls_to(&log, 0x0045_c780), vec![vec![tes.addr()]]);
        // The havok bound is refreshed with the cell and the grid array.
        assert_eq!(
            calls_to(&log, 0x0062_71b0),
            vec![vec![0x9000, 0x6100, array]]
        );
        assert!(calls_to(&log, 0x0045_7d70).is_empty());
        assert!(calls_to(&log, 0x0052_83c0).is_empty());
    }

    #[test]
    fn grid_update_keeps_the_moved_flag_while_the_copy_is_set_or_an_interior_is_loaded() {
        let (mut e, tes, _, _) = update_engine(5);
        e.set_global(DATA_HANDLER, 0u32);
        let pos = position(&mut e, 0.0, 0.0);
        e.set_global(GRID_FLAG_MOVED, 1u8);
        e.set_global(GRID_FLAG_MOVED_COPY, 1u8);
        e.call(0x0045_2580, &args![tes, pos, 0u8]);
        assert_eq!(e.global::<u8>(GRID_FLAG_MOVED), 1);
        e.set_global(GRID_FLAG_MOVED_COPY, 0u8);
        returns(&mut e, GET_INTERIOR_CELL, 0x6200);
        e.call(0x0045_2580, &args![tes, pos, 0u8]);
        assert_eq!(e.global::<u8>(GRID_FLAG_MOVED), 1);
    }

    #[test]
    fn grid_update_with_an_unset_grid_takes_the_position() {
        let (mut e, tes, _, _) = update_engine(5);
        e.set(tes, TES::iCurrentGridX, 0x7fff_ffff);
        // `004512c0` returns at once while a grid load is in progress.
        e.set_global(GRID_LOAD_IN_PROGRESS, 1u8);
        let pos = position(&mut e, 5.0 * 4096.0 + 10.0, 7.0 * 4096.0 + 10.0);
        let log = run_logged(&mut e, |e| {
            assert!(e.call(0x0045_2580, &args![tes, pos, 0u8]).bool());
        });
        assert_eq!(e.get(tes, TES::iCurrentGridX), 5);
        assert_eq!(e.get(tes, TES::iCurrentGridY), 7);
        assert_eq!(e.get(tes, TES::iCurrentQueuedX), 5);
        assert_eq!(e.get(tes, TES::iCurrentQueuedY), 7);
        assert_eq!(
            calls_to(&log, 0x0045_7d70),
            vec![vec![tes.addr(), 1, 0x6000, 0]]
        );
        assert_eq!(count_calls(&log, 0x0062_f460), 1);
    }

    #[test]
    fn grid_update_with_an_unset_grid_and_no_position_keeps_it_unset() {
        let (mut e, tes, _, _) = update_engine(5);
        e.set(tes, TES::iCurrentGridY, 0x7fff_ffff);
        e.set_global(GRID_LOAD_IN_PROGRESS, 1u8);
        let log = run_logged(&mut e, |e| {
            assert!(e.call(0x0045_2580, &args![tes, 0u32, 0u8]).bool());
        });
        assert_eq!(e.get(tes, TES::iCurrentGridX), 10);
        assert_eq!(e.get(tes, TES::iCurrentGridY), 0x7fff_ffff);
        assert_eq!(count_calls(&log, 0x0045_7d70), 1);
    }

    /// The loader's `QueueCellLoad` calls as `(x, y)` pairs.
    fn queued_loads(log: &[(u32, Vec<u32>)]) -> Vec<(u32, u32)> {
        calls_to(log, EXTERIOR_CELL_LOADER_QUEUE_CELL_LOAD)
            .into_iter()
            .map(|w| {
                assert_eq!((w[0], w[1]), (0x5000, 0x6000));
                (w[2], w[3])
            })
            .collect()
    }

    #[test]
    fn grid_update_inside_the_cell_queues_the_new_column() {
        let (mut e, tes, _, _) = update_engine(5);
        // Cell 11 is one column over; the position is 2148 units from the
        // centre of cell 10 in x, so 1948 remain to the border.
        let pos = position(&mut e, 11.0 * 4096.0 + 100.0, 20.0 * 4096.0 + 2048.0);
        let log = run_logged(&mut e, |e| {
            assert!(!e.call(0x0045_2580, &args![tes, pos, 0u8]).bool());
        });
        assert_eq!(e.get(tes, TES::fCell_delta_x), 1948.0);
        assert_eq!(e.get(tes, TES::fCell_delta_y), 4096.0);
        // half = 2: the column is (2 + 1) * 1 + 10 = 13, rows 18 to 22.
        assert_eq!(
            queued_loads(&log),
            vec![(13, 18), (13, 19), (13, 20), (13, 21), (13, 22)]
        );
        assert_eq!(e.get(tes, TES::iCurrentQueuedX), 11);
        assert_eq!(e.get(tes, TES::iCurrentQueuedY), 20);
        // The grid itself did not move.
        assert_eq!(e.get(tes, TES::iCurrentGridX), 10);
        assert_eq!(e.get(tes, TES::iCurrentGridY), 20);
    }

    #[test]
    fn grid_update_inside_the_cell_queues_row_column_and_corner() {
        let (mut e, tes, _, _) = update_engine(5);
        let pos = position(&mut e, 11.0 * 4096.0 + 100.0, 21.0 * 4096.0 + 100.0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_2580, &args![tes, pos, 0u8]);
        });
        let loads = queued_loads(&log);
        assert_eq!(loads.len(), 11);
        assert_eq!(
            loads[..5],
            [(13, 18), (13, 19), (13, 20), (13, 21), (13, 22)]
        );
        // The row is (2 + 1) * 1 + 20 = 23, columns 8 to 12 (the queued x is
        // still 10 while they are issued), then the corner.
        assert_eq!(
            loads[5..10],
            [(8, 23), (9, 23), (10, 23), (11, 23), (12, 23)]
        );
        assert_eq!(loads[10], (13, 23));
        assert_eq!(e.get(tes, TES::iCurrentQueuedY), 21);
    }

    #[test]
    fn grid_update_inside_the_same_queued_cell_queues_nothing() {
        let (mut e, tes, _, _) = update_engine(5);
        let pos = position(&mut e, 10.0 * 4096.0 + 2048.0, 20.0 * 4096.0 + 2048.0);
        let log = run_logged(&mut e, |e| {
            assert!(!e.call(0x0045_2580, &args![tes, pos, 0u8]).bool());
        });
        assert!(queued_loads(&log).is_empty());
        assert_eq!(e.get(tes, TES::fCell_delta_x), 4096.0);
    }

    #[test]
    fn grid_update_inside_the_cell_cleans_up_textures_before_queueing_when_asked() {
        let (mut e, tes, _, _) = update_engine(5);
        put_setting(&mut e, SETTING_PREEMPTIVELY_UNLOAD_CELLS, 1u8);
        returns(&mut e, 0x0045_4d50, 1);
        let pos = position(&mut e, 11.0 * 4096.0 + 100.0, 20.0 * 4096.0 + 2048.0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_2580, &args![tes, pos, 0u8]);
        });
        assert_eq!(calls_to(&log, 0x0045_4d50), vec![vec![tes.addr(), 1]]);
        assert_eq!(count_calls(&log, 0x0040_fbf0), 1);
        assert_eq!(queued_loads(&log).len(), 5);
    }

    #[test]
    fn grid_update_inside_the_cell_does_not_queue_without_background_loading_or_in_an_interior() {
        let (mut e, tes, _, _) = update_engine(5);
        let pos = position(&mut e, 11.0 * 4096.0 + 100.0, 20.0 * 4096.0 + 2048.0);
        put_setting(&mut e, SETTING_BACKGROUND_CELL_LOADS, 0u8);
        let log = run_logged(&mut e, |e| {
            assert!(!e.call(0x0045_2580, &args![tes, pos, 0u8]).bool());
        });
        assert!(queued_loads(&log).is_empty());
        assert_eq!(e.get(tes, TES::iCurrentQueuedX), 10);
        put_setting(&mut e, SETTING_BACKGROUND_CELL_LOADS, 1u8);
        returns(&mut e, GET_INTERIOR_CELL, 0x6200);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_2580, &args![tes, pos, 0u8]);
        });
        assert!(queued_loads(&log).is_empty());
    }

    #[test]
    fn grid_update_distance_uses_three_thousand_for_three_grids() {
        let (mut e, tes, _, _) = update_engine(3);
        let pos = position(
            &mut e,
            10.0 * 4096.0 + 2048.0 + 100.0,
            20.0 * 4096.0 + 2048.0,
        );
        e.call(0x0045_2580, &args![tes, pos, 0u8]);
        assert_eq!(e.get(tes, TES::fCell_delta_x), 3072.0 - 100.0);
        assert_eq!(e.get(tes, TES::fCell_delta_y), 3072.0);
    }

    /// Runs a move of the grid from cell (10, 20) to the position
    /// `(x_cell, 20)` (the cell's centre), with `world_space` as the current
    /// one, and returns the log. The loader reports no queued loads
    /// (`005285a0` returns `queued`).
    fn grid_move(e: &mut Engine, tes: Ptr<TES>, x_cell: u32, flag: u8) -> Vec<(u32, Vec<u32>)> {
        let pos = position(e, x_cell as f32 * 4096.0 + 2048.0, 20.0 * 4096.0 + 2048.0);
        run_logged(e, |e| {
            assert!(e.call(0x0045_2580, &args![tes, pos, flag]).bool());
        })
    }

    #[test]
    fn grid_move_far_away_reloads_the_grid_around_the_new_cell() {
        let (mut e, tes, _, _) = update_engine(5);
        let array = e.get(tes, TES::pGridCellA).addr();
        install_player(&mut e, 0);
        e.set(tes, TES::pWorldSpace, Ptr::new(0));
        returns(&mut e, 0x0046_0140, 0x7100);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x6abc);
        returns(&mut e, 0x0068_15c0, slot);
        let log = grid_move(&mut e, tes, 12, 0);
        // The cell deltas are the (negative) distances to the border.
        assert_eq!(e.get(tes, TES::fCell_delta_x), -4096.0);
        assert_eq!(e.get(tes, TES::fCell_delta_y), 4096.0);
        // The grid is on cell (12, 20) and the queue follows it.
        assert_eq!(e.get(tes, TES::iCurrentGridX), 12);
        assert_eq!(e.get(tes, TES::iCurrentGridY), 20);
        assert_eq!(e.get(tes, TES::iCurrentQueuedX), 12);
        assert_eq!(e.get(tes, TES::iCurrentQueuedY), 20);
        // The centre globals: (12.5 * 4096, 20.5 * 4096); a move of two
        // cells resets the previous centre to the same value.
        for base in [GRID_CENTRE, GRID_PREVIOUS_CENTRE] {
            assert_eq!(e.global::<f32>(base), 51200.0);
            assert_eq!(e.global::<f32>(base + 4), 83968.0);
        }
        assert_eq!(e.global::<f32>(GRID_FLOAT_011F94AC), 1.5);
        assert_eq!(e.global::<u8>(GRID_FLAG_MOVED), 1);
        assert_eq!(e.global::<u8>(GRID_FLAG_MOVED_COPY), 1);
        assert!(e.get(tes, TES::bUpdateGridString));
        // No world space: the data handler's current one is set.
        assert_eq!(
            calls_to(&log, 0x0046_0140),
            vec![vec![e.global(DATA_HANDLER)]]
        );
        assert_eq!(calls_to(&log, 0x0068_15c0), vec![vec![0x7100]]);
        assert_eq!(calls_to(&log, 0x0045_8200), vec![vec![tes.addr(), 0x6abc]]);
        // The havok world counters (far move: both dropped to 0 first).
        let world = vec![0x8000u32];
        let with_zero = |w: &[u32]| vec![w[0], 0];
        assert_eq!(count_calls(&log, HAVOK_WORLD_ADD_COUNT_1C), 1);
        assert_eq!(
            calls_to(&log, HAVOK_WORLD_ADD_COUNT_1C),
            vec![world.clone()]
        );
        assert_eq!(
            calls_to(&log, HAVOK_WORLD_DROP_COUNT_1C),
            vec![with_zero(&world), with_zero(&world)]
        );
        // Block 1, the grid array load (twice each) and block 4.
        assert_eq!(count_calls(&log, HAVOK_WORLD_DROP_COUNT_18), 4);
        assert_eq!(count_calls(&log, HAVOK_WORLD_ADD_COUNT_18), 3);
        assert!(calls_to(&log, HAVOK_WORLD_DROP_COUNT_18)
            .iter()
            .all(|w| *w == with_zero(&world)));
        // The grid array moves to the new centre.
        assert_eq!(calls_to(&log, 0x0300_0010), vec![vec![array, 12, 20]]);
        // Nothing queued: the unloaded-cells message (no world space, so
        // not all cells are loaded), no queued-cells message.
        assert_eq!(
            calls_to(&log, SET_LOADING_AREA_TEXT),
            vec![vec![tes.addr(), LOADING_AREA_UNLOADED_CELLS]]
        );
        assert_eq!(calls_to(&log, HAVOK_WORLD_GET_0X50), vec![world.clone()]);
        assert_eq!(calls_to(&log, 0x0045_7be0), vec![vec![tes.addr(), 0]]);
        // The wind value comes from the table, and the terrain follows the
        // player.
        assert_eq!(
            calls_to(&log, TERRAIN_MANAGER_UPDATE),
            vec![vec![0x7001, 0x8888, 0xf]]
        );
        // `00452e40(1)` is the last thing done.
        assert_eq!(e.global::<u8>(FLAG_011AF70C), 1);
        assert_eq!(count_calls(&log, 0x0052_8110), 1);
        assert_eq!(count_calls(&log, 0x0052_82d0), 0);
    }

    #[test]
    fn grid_move_reports_queued_cells_and_cancels_the_loads() {
        let (mut e, tes, _, _) = update_engine(5);
        install_player(&mut e, 0);
        returns(&mut e, 0x0052_85a0, 1);
        returns(&mut e, 0x0052_81f0, 1);
        let log = grid_move(&mut e, tes, 12, 0);
        // Queued cells: the message, and neither the later message nor the
        // havok world's `0x50` query.
        assert_eq!(
            calls_to(&log, SET_LOADING_AREA_TEXT),
            vec![vec![tes.addr(), LOADING_AREA_QUEUED_CELLS]]
        );
        assert_eq!(count_calls(&log, HAVOK_WORLD_GET_0X50), 0);
        // The loader is told to cancel (`005281f0` found something).
        assert_eq!(calls_to(&log, 0x0052_82d0), vec![vec![0x5000]]);
    }

    #[test]
    fn grid_move_by_one_cell_keeps_the_previous_centre_and_passes_the_flag() {
        let (mut e, tes, _, _) = update_engine(5);
        install_player(&mut e, 0);
        // The previous centre differs from the reference pair.
        e.set_global(GRID_CENTRE, 1.0f32);
        e.set_global(GRID_CENTRE + 4, 2.0f32);
        e.set_global(GRID_CENTRE_REFERENCE, 3.0f32);
        e.set_global(GRID_CENTRE_REFERENCE + 4, 4.0f32);
        let log = grid_move(&mut e, tes, 11, 7);
        // One column: not far. The previous centre is the old centre.
        assert_eq!(e.global::<f32>(GRID_PREVIOUS_CENTRE), 1.0);
        assert_eq!(e.global::<f32>(GRID_PREVIOUS_CENTRE + 4), 2.0);
        assert_eq!(e.global::<f32>(GRID_CENTRE), 11.5 * 4096.0);
        // The flag is handed to the havok world counters; the block 1
        // drops are skipped.
        assert_eq!(
            calls_to(&log, HAVOK_WORLD_DROP_COUNT_1C),
            vec![vec![0x8000, 7]]
        );
        // Grid array load (two) and block 4 (flag).
        let drops = calls_to(&log, HAVOK_WORLD_DROP_COUNT_18);
        assert_eq!(drops.len(), 3);
        assert_eq!(drops[2], vec![0x8000, 7]);
        assert_eq!(count_calls(&log, 0x0045_7be0), 0);
    }

    #[test]
    fn grid_move_resets_the_previous_centre_when_it_equals_the_reference() {
        let (mut e, tes, _, _) = update_engine(5);
        install_player(&mut e, 0);
        e.set_global(GRID_CENTRE, 1.0f32);
        e.set_global(GRID_CENTRE + 4, 2.0f32);
        // The old centre is copied to the previous centre first; it equals
        // the reference pair, so the previous centre is then overwritten with
        // the new centre even for a one-cell move.
        e.set_global(GRID_PREVIOUS_CENTRE, 1.0f32);
        e.set_global(GRID_PREVIOUS_CENTRE + 4, 2.0f32);
        e.set_global(GRID_CENTRE_REFERENCE, 1.0f32);
        e.set_global(GRID_CENTRE_REFERENCE + 4, 2.0f32);
        grid_move(&mut e, tes, 11, 0);
        assert_eq!(e.global::<f32>(GRID_PREVIOUS_CENTRE), 11.5 * 4096.0);
    }

    #[test]
    fn grid_move_in_a_script_context_with_bit_20_stays_on_the_current_cell() {
        let (mut e, tes, context, _) = update_engine(5);
        let array = e.get(tes, TES::pGridCellA).addr();
        install_player(&mut e, 0);
        returns(&mut e, GET_INTERIOR_CELL, 0x6200);
        // Script running (bit 2) and bit 0x20.
        e.mem.set_u32(context + 0x244, 0x22);
        let log = grid_move(&mut e, tes, 15, 0);
        assert_eq!(e.get(tes, TES::iCurrentGridX), 10);
        assert_eq!(e.get(tes, TES::iCurrentGridY), 20);
        // An interior is loaded but a script runs: the cells are still
        // reloaded; the terrain is not updated (interior loaded).
        assert_eq!(calls_to(&log, 0x0300_0010), vec![vec![array, 10, 20]]);
        assert_eq!(count_calls(&log, TERRAIN_MANAGER_UPDATE), 0);
        // No loading-area message while a script runs.
        assert_eq!(count_calls(&log, SET_LOADING_AREA_TEXT), 0);
    }

    #[test]
    fn grid_move_with_an_interior_and_no_script_only_updates_the_bookkeeping() {
        let (mut e, tes, _, _) = update_engine(5);
        returns(&mut e, GET_INTERIOR_CELL, 0x6200);
        let log = grid_move(&mut e, tes, 12, 0);
        assert_eq!(e.get(tes, TES::iCurrentGridX), 12);
        assert_eq!(count_calls(&log, 0x0300_0010), 0);
        assert_eq!(count_calls(&log, TERRAIN_MANAGER_UPDATE), 0);
        assert_eq!(e.global::<u8>(FLAG_011AF70C), 1);
    }

    #[test]
    fn grid_move_cleans_up_textures_when_preemptive_unloading_is_on() {
        let (mut e, tes, _, _) = update_engine(5);
        install_player(&mut e, 0);
        put_setting(&mut e, SETTING_PREEMPTIVELY_UNLOAD_CELLS, 1u8);
        returns(&mut e, 0x0045_4d50, 1);
        let log = grid_move(&mut e, tes, 12, 0);
        assert_eq!(calls_to(&log, 0x0045_4d50), vec![vec![tes.addr(), 1]]);
        // Once before the move and once after.
        assert_eq!(count_calls(&log, 0x0040_fbf0), 2);
    }

    #[test]
    fn point_constructor_stores_two_floats() {
        let mut e = Engine::new();
        let point = e.mem.alloc(8);
        let result = e.call(0x0045_2dc0, &args![point, 1.5f32, -2.0f32]).u32();
        assert_eq!(result, point);
        assert_eq!(e.mem.f32(point), 1.5);
        assert_eq!(e.mem.f32(point + 4), -2.0);
    }

    #[test]
    fn point_equality_compares_both_floats() {
        let mut e = Engine::new();
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        for (a0, a1, b0, b1, equal) in [
            (1.0f32, 2.0f32, 1.0f32, 2.0f32, true),
            (1.0, 2.0, 1.0, 3.0, false),
            (1.0, 2.0, 0.0, 2.0, false),
            (f32::NAN, 2.0, f32::NAN, 2.0, false),
            (1.0, f32::NAN, 1.0, f32::NAN, false),
        ] {
            e.mem.set_f32(a, a0);
            e.mem.set_f32(a + 4, a1);
            e.mem.set_f32(b, b0);
            e.mem.set_f32(b + 4, b1);
            assert_eq!(e.call(0x0045_2df0, &args![a, b]).bool(), equal);
        }
    }

    #[test]
    fn small_stores_write_their_globals() {
        let mut e = Engine::new();
        e.map(FLAG_011AF70C, 1);
        e.call(0x0045_2e40, &args![9u8]);
        assert_eq!(e.global::<u8>(FLAG_011AF70C), 9);
        e.map(FLAG_011F9427, 1);
        e.call(0x0045_4440, &args![3u8]);
        assert_eq!(e.global::<u8>(FLAG_011F9427), 3);
        let tes: Ptr<TES> = e.new_object();
        assert!(!e.get(tes, TES::bUpdateGridString));
        e.call(0x0045_2e50, &args![tes]);
        assert!(e.get(tes, TES::bUpdateGridString));
    }

    #[test]
    fn float_table_is_indexed_by_whether_the_argument_is_set() {
        let mut e = Engine::new();
        e.map(FLOAT_PAIR_011F940C, 8);
        e.set_global(FLOAT_PAIR_011F940C, 1.25f32);
        e.set_global(FLOAT_PAIR_011F940C + 4, 8.5f32);
        assert_eq!(e.call(0x0045_2e70, &args![0u8]).f32(), 1.25);
        assert_eq!(e.call(0x0045_2e70, &args![1u8]).f32(), 8.5);
        assert_eq!(e.call(0x0045_2e70, &args![200u8]).f32(), 8.5);
    }

    #[test]
    fn script_context_bit_20_and_bit_10() {
        let mut e = Engine::new();
        let context = e.mem.alloc(0x300);
        assert!(!e.call(0x0045_2e90, &args![context]).bool());
        e.mem.set_u32(context + 0x244, 0x20);
        assert!(e.call(0x0045_2e90, &args![context]).bool());
        e.mem.set_u32(context + 0x244, 0xffff_ffdf);
        assert!(!e.call(0x0045_2e90, &args![context]).bool());
        // `00453500` sets and clears bit 0x10 only.
        e.mem.set_u32(context + 0x244, 0x20);
        e.call(0x0045_3500, &args![context, 1u8]);
        assert_eq!(e.mem.u32(context + 0x244), 0x30);
        e.call(0x0045_3500, &args![context, 0u8]);
        assert_eq!(e.mem.u32(context + 0x244), 0x20);
        e.mem.set_u32(context + 0x244, 0xffff_ffff);
        e.call(0x0045_3500, &args![context, 0u8]);
        assert_eq!(e.mem.u32(context + 0x244), 0xffff_ffef);
    }

    #[test]
    fn cancelling_the_exterior_loads_calls_the_loader() {
        let mut e = Engine::new();
        e.map(EXTERIOR_CELL_LOADER, 4);
        e.set_global(EXTERIOR_CELL_LOADER, 0x5000u32);
        noop(&mut e, &[0x0052_8540]);
        let tes: Ptr<TES> = e.new_object();
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_2eb0, &args![tes]);
        });
        assert_eq!(calls_to(&log, 0x0052_8540), vec![vec![0x5000]]);
    }

    /// A TES whose singleton and data are set for `AllCellsInGridLoaded`
    /// with `uGridsToLoad` 2 around (10, 20); every slot holds a cell and
    /// `00454e20` answers `loaded`.
    fn all_cells_engine(loaded: u32) -> (Engine, Ptr<TES>, u32, Vec<u32>) {
        let (mut e, tes, table, _) = {
            let (e, tes, context, table) = update_engine(2);
            (e, tes, table, context)
        };
        let mut cells = vec![];
        for x in 0..2 {
            for y in 0..2 {
                let cell = new_cell(&mut e, 3, 0);
                put_cell(&mut e, table, 2, x, y, cell);
                cells.push(cell);
            }
        }
        returns(&mut e, 0x0045_4e20, loaded);
        (e, tes, table, cells)
    }

    #[test]
    fn all_cells_in_grid_loaded_is_false_without_a_world_space() {
        let (mut e, tes, _, _) = all_cells_engine(1);
        e.set(tes, TES::pWorldSpace, Ptr::new(0));
        assert!(!e.call(0x0045_2ed0, &args![tes]).bool());
    }

    #[test]
    fn all_cells_in_grid_loaded_asks_about_every_cell() {
        let (mut e, tes, _, cells) = all_cells_engine(1);
        let log = run_logged(&mut e, |e| {
            assert!(e.call(0x0045_2ed0, &args![tes]).bool());
        });
        let asked: Vec<Vec<u32>> = cells.iter().map(|c| vec![tes.addr(), *c]).collect();
        assert_eq!(calls_to(&log, 0x0045_4e20), asked);
    }

    #[test]
    fn all_cells_in_grid_loaded_stops_at_the_first_cell_that_is_not_loaded() {
        let (mut e, tes, _, _) = all_cells_engine(0);
        let log = run_logged(&mut e, |e| {
            assert!(!e.call(0x0045_2ed0, &args![tes]).bool());
        });
        assert_eq!(count_calls(&log, 0x0045_4e20), 1);
    }

    #[test]
    fn all_cells_in_grid_loaded_looks_empty_slots_up_in_the_world_space() {
        let (mut e, tes, table, cells) = all_cells_engine(1);
        // Slot (0, 1) is empty; its cell comes from the world space at
        // (10 - 1 + 0, 20 - 1 + 1).
        put_cell(&mut e, table, 2, 0, 1, 0);
        let found = new_cell(&mut e, 3, 0);
        returns(&mut e, 0x0058_75a0, found);
        let log = run_logged(&mut e, |e| {
            assert!(e.call(0x0045_2ed0, &args![tes]).bool());
        });
        assert_eq!(calls_to(&log, 0x0058_75a0), vec![vec![0x6000, 9, 20]]);
        let asked = calls_to(&log, 0x0045_4e20);
        assert_eq!(asked[1], vec![tes.addr(), found]);
        assert_eq!(asked[0], vec![tes.addr(), cells[0]]);
        // A cell the world space does not know makes the answer false.
        returns(&mut e, 0x0058_75a0, 0);
        assert!(!e.call(0x0045_2ed0, &args![tes]).bool());
    }

    #[test]
    fn all_cells_in_grid_loaded_skips_grid_positions_without_a_slot() {
        let (mut e, tes, table, _) = all_cells_engine(1);
        // `GridCellArray::Get` finds no slot for (1, 1).
        e.register_double(GRID_CELL_ARRAY_GET, move |_, a| Ret {
            eax: if (a[1], a[2]) == (1, 1) {
                0
            } else {
                table + (a[1] * 2 + a[2]) * 4
            },
            ..Ret::default()
        });
        let log = run_logged(&mut e, |e| {
            assert!(e.call(0x0045_2ed0, &args![tes]).bool());
        });
        assert_eq!(count_calls(&log, 0x0045_4e20), 3);
    }

    /// A grid load engine: `uGridsToLoad` 2, cells in all four slots of the
    /// data handler's lookup (`00461c20`) with the given load `states`, a
    /// world space, and `004ba5a0` (grid slot already filled) true.
    fn array_load_engine(states: [u8; 4]) -> (Engine, Ptr<TES>, u32, Vec<u32>) {
        let (mut e, tes, context, _) = update_engine(2);
        let cells: Vec<u32> = states
            .iter()
            .map(|state| new_cell(&mut e, *state, 0))
            .collect();
        let lookup = cells.clone();
        // The data handler is asked for (x, y, worldSpace, 0) with x in
        // 9..=10 and y in 19..=20.
        e.register_double(0x0046_1c20, move |_, a| {
            let index = ((a[1] - 9) * 2 + (a[2] - 19)) as usize;
            Ret {
                eax: lookup[index],
                ..Ret::default()
            }
        });
        returns(&mut e, 0x0058_5b30, 0x7777);
        returns(&mut e, 0x004b_a5a0, 1);
        (e, tes, context, cells)
    }

    #[test]
    fn grid_array_load_without_a_world_space_only_brackets_the_work() {
        let (mut e, tes, context, _) = array_load_engine([5, 5, 5, 5]);
        e.set(tes, TES::pWorldSpace, Ptr::new(0));
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_2ff0, &args![tes]);
        });
        // The memory tag guard with tag 0x1a and the source line.
        assert_eq!(
            calls_to(&log, SCOPE_GUARD_CTOR)
                .iter()
                .map(|w| w[1..].to_vec())
                .collect::<Vec<_>>(),
            vec![vec![0x1a, 1, TES_CPP_PATH, 0x8d6]]
        );
        assert_eq!(count_calls(&log, SCOPE_GUARD_DTOR), 1);
        // The script context's bit 0x10 is set only while it runs.
        assert_eq!(e.mem.u32(context + 0x244) & 0x10, 0);
        // The script flag is clear: the fade is started again.
        assert_eq!(calls_to(&log, 0x0084_92b0), vec![vec![context, 0]]);
        // The havok world is locked twice (start and after the first loop).
        assert_eq!(count_calls(&log, HAVOK_WORLD_ADD_COUNT_18), 2);
        assert_eq!(count_calls(&log, HAVOK_WORLD_DROP_COUNT_18), 2);
        // End: `00451570(0)` and `00451520(1)`.
        assert_eq!(calls_to(&log, 0x0066_b0d0), vec![vec![0x011f_95e8, 0]]);
        assert_eq!(e.global::<u8>(GRID_FLAG_01189184), 1);
        // Nothing was attached.
        assert_eq!(count_calls(&log, 0x0054_bcf0), 0);
    }

    #[test]
    fn grid_array_load_loads_buffers_and_attaches_the_cells() {
        let (mut e, tes, _, cells) = array_load_engine([5, 3, 5, 6]);
        // The cells are not interiors; the buffer check says yes.
        returns(&mut e, 0x0045_4e20, 1);
        returns(&mut e, 0x0054_5cf0, 0xd000);
        let added = Rc::new(RefCell::new(vec![]));
        let seen = added.clone();
        e.register_double(LIST_ADD_HEAD, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_2ff0, &args![tes]);
        });
        // `TES::LoadGridCell` ran for the four slots: each asked the grid
        // array whether it was already filled and added the cell to the
        // buffer.
        assert_eq!(count_calls(&log, 0x004b_a5a0), 4);
        // The four cells went into the buffer once from the first loop; the
        // state 5 cells (0 and 2) again from the second.
        let buffered: Vec<u32> = calls_to(&log, TES_ADD_TO_BUFFER)
            .iter()
            .map(|w| w[1])
            .collect();
        assert_eq!(buffered.len(), 4 + 4 + 2);
        // Only the state 5 cells are attached (flag: loading menu not up).
        assert_eq!(
            calls_to(&log, 0x0054_bcf0),
            vec![vec![cells[0], 1], vec![cells[2], 1]]
        );
        // Their 3D is updated, once each, and `009938b0` (0) says to
        // update it with the zero update object.
        assert_eq!(
            calls_to(&log, NODE_UPDATE_PROPERTIES),
            vec![vec![0xd000]; 2]
        );
        assert_eq!(count_calls(&log, UPDATE_OBJECT_CTOR), 2);
        assert_eq!(count_calls(&log, NODE_UPDATE), 2);
        // Every cell found goes onto the last-loaded list (`this + 0x8c`),
        // whatever its state.
        assert_eq!(
            *added.borrow(),
            vec![
                (0x7000, cells[0]),
                (0x7000, cells[1]),
                (0x7000, cells[2]),
                (0x7000, cells[3])
            ]
        );
    }

    #[test]
    fn grid_array_load_counts_progress_when_the_loading_menu_is_up() {
        let (mut e, tes, _, _) = array_load_engine([3, 3, 3, 3]);
        returns(&mut e, 0x0070_5e80, 1);
        returns(&mut e, 0x0047_c850, 1);
        returns(&mut e, 0x0087_27b0, 1);
        let save_load: u32 = e.global(SAVE_LOAD_GAME);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_2ff0, &args![tes]);
        });
        // One progress step per cell: the setting read twice for the
        // percentage and the save/load screen refreshed.
        assert_eq!(calls_to(&log, 0x0086_1ea0), vec![vec![save_load]; 4]);
        // The final save/load block: the two flags are fetched, the screen
        // reset, then the flags restored.
        assert_eq!(calls_to(&log, 0x0085_8af0), vec![vec![save_load, 0, 0, 0]]);
        assert_eq!(calls_to(&log, 0x0085_f850), vec![vec![save_load, 0]]);
        // No cell is in state 5, so none is attached.
        assert_eq!(count_calls(&log, 0x0054_bcf0), 0);
    }

    #[test]
    fn grid_array_load_attaches_with_the_loading_menu_flag_when_it_is_up() {
        let (mut e, tes, _, cells) = array_load_engine([5, 3, 3, 3]);
        returns(&mut e, 0x0070_5e80, 1);
        returns(&mut e, 0x0054_5cf0, 0xd000);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_2ff0, &args![tes]);
        });
        assert_eq!(calls_to(&log, 0x0054_bcf0), vec![vec![cells[0], 0]]);
    }

    #[test]
    fn node_update_check_getter_adds_0x9c() {
        let mut e = Engine::new();
        returns(&mut e, 0x0099_38b0, 42);
        let log = run_logged(&mut e, |e| {
            assert_eq!(e.call(0x0045_3470, &args![0x1000u32]).u32(), 42);
        });
        assert_eq!(calls_to(&log, 0x0099_38b0), vec![vec![0x109c]]);
    }

    #[test]
    fn buffer_check_uses_the_interior_or_exterior_test() {
        let mut e = Engine::new();
        let tes: Ptr<TES> = e.new_object();
        returns(&mut e, 0x0045_4bd0, 1);
        returns(&mut e, 0x0045_4e20, 0);
        returns(&mut e, 0x0042_5fd0, 1);
        // Null: false without any call.
        let log = run_logged(&mut e, |e| {
            assert!(!e.call(0x0045_3490, &args![tes, 0u32]).bool());
        });
        assert_eq!(addresses(&log), vec![0x0045_3490]);
        let log = run_logged(&mut e, |e| {
            assert!(e.call(0x0045_3490, &args![tes, 0x4000u32]).bool());
        });
        assert_eq!(calls_to(&log, 0x0045_4bd0), vec![vec![tes.addr(), 0x4000]]);
        assert_eq!(count_calls(&log, 0x0045_4e20), 0);
        returns(&mut e, 0x0042_5fd0, 0);
        let log = run_logged(&mut e, |e| {
            assert!(!e.call(0x0045_3490, &args![tes, 0x4000u32]).bool());
        });
        assert_eq!(calls_to(&log, 0x0045_4e20), vec![vec![tes.addr(), 0x4000]]);
    }

    #[test]
    fn save_game_flag_function_does_nothing() {
        let mut e = Engine::new();
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_34f0, &args![0x1234u32, 1u32]);
        });
        assert_eq!(addresses(&log), vec![0x0045_34f0]);
    }

    #[test]
    fn wind_getters_read_the_sky_fields() {
        let mut e = Engine::new();
        let sky = e.mem.alloc(0x100);
        e.mem.set_f32(sky + 0xcc, 3.5);
        e.mem.set_f32(sky + 0xd0, -0.75);
        assert_eq!(e.call(0x0045_36e0, &args![sky]).f32(), 3.5);
        assert_eq!(e.call(0x0045_3700, &args![sky]).f32(), -0.75);
    }

    #[test]
    fn collision_listener_is_created_once() {
        let mut e = Engine::new();
        e.map(COLLISION_LISTENER, 4);
        returns(&mut e, OPERATOR_NEW, 0x9000);
        returns(&mut e, 0x0062_3430, 0x9000);
        let log = run_logged(&mut e, |e| {
            assert_eq!(e.call(0x0045_3720, &args![]).u32(), 0x9000);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0xc]]);
        assert_eq!(calls_to(&log, 0x0062_3430), vec![vec![0x9000]]);
        // Once the constructor has stored the pointer, it is returned as is.
        e.set_global(COLLISION_LISTENER, 0x9000u32);
        let log = run_logged(&mut e, |e| {
            assert_eq!(e.call(0x0045_3720, &args![]).u32(), 0x9000);
        });
        assert_eq!(addresses(&log), vec![0x0045_3720]);
        // A failed allocation gives null.
        e.set_global(COLLISION_LISTENER, 0u32);
        returns(&mut e, OPERATOR_NEW, 0);
        assert_eq!(e.call(0x0045_3720, &args![]).u32(), 0);
    }

    #[test]
    fn instance_getters_return_their_globals() {
        let mut e = Engine::new();
        e.map(TASK_QUEUE_OBJECT, 4);
        e.map(AUDIO_INSTANCE, 4);
        e.set_global(TASK_QUEUE_OBJECT, 0x1111u32);
        e.set_global(AUDIO_INSTANCE, 0x2222u32);
        assert_eq!(e.call(0x0045_37b0, &args![]).u32(), 0x1111);
        assert_eq!(e.call(0x0045_3a70, &args![]).u32(), 0x2222);
    }

    /// An engine for the per-frame update (`00453550`).
    fn frame_engine(threads: i32) -> (Engine, Ptr<TES>) {
        let (mut e, tes, _, _) = update_engine(3);
        put_setting(&mut e, SETTING_NUM_HW_THREADS, threads);
        put_setting(&mut e, SETTING_ANIMATION_MULT, 0.5f32);
        let sky = e.mem.alloc(0x100);
        e.mem.set_f32(sky + 0xcc, 3.5);
        e.mem.set_f32(sky + 0xd0, -0.75);
        e.set(tes, TES::pSky, Ptr::new(sky));
        e.set_global(TIME_ACCUMULATOR, 10.0f32);
        e.set_global(COLLISION_LISTENER, 0x9000u32);
        e.set_global(TASK_QUEUE_OBJECT, 0x1111u32);
        returns_float(&mut e, 0x0045_3850, 0.25);
        returns_float(&mut e, 0x0084_d030, 4.0);
        (e, tes)
    }

    #[test]
    fn frame_update_passes_the_wind_and_steps_the_grid_cells() {
        let (mut e, tes) = frame_engine(1);
        let array = e.get(tes, TES::pGridCellA).addr();
        let sky = e.get(tes, TES::pSky).addr();
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_3550, &args![tes, 0.5f32]);
        });
        assert_eq!(e.global::<f32>(TIME_ACCUMULATOR), 10.5);
        assert_eq!(
            calls_to(&log, 0x00c4_68c0),
            vec![vec![3.5f32.to_bits(), (-0.75f32).to_bits()]]
        );
        assert_eq!(
            calls_to(&log, 0x00c7_4550),
            vec![vec![3.5f32.to_bits(), (-0.75f32).to_bits()]]
        );
        // The time is passed to the grid array (no interior cell).
        assert_eq!(
            calls_to(&log, 0x004b_a9a0),
            vec![vec![array, 10.5f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, 0x0045_3860),
            vec![vec![tes.addr(), 1], vec![tes.addr(), 0]]
        );
        // One hardware thread: the sky is updated with the frame time
        // (the argument, as no dialog is up) here and again by `004537c0`
        // with `00453850`'s value.
        assert_eq!(
            calls_to(&log, 0x0063_ac70),
            vec![vec![sky, 0.5f32.to_bits()], vec![sky, 0.25f32.to_bits()]]
        );
        assert_eq!(count_calls(&log, 0x0087_aa90), 0);
        assert_eq!(calls_to(&log, 0x0062_3640), vec![vec![0x9000]]);
        assert_eq!(calls_to(&log, 0x0097_5080), vec![vec![OBJECT_011E0E80]]);
    }

    #[test]
    fn frame_update_in_an_interior_zeroes_the_wind_and_steps_the_cell() {
        let (mut e, tes) = frame_engine(1);
        returns(&mut e, GET_INTERIOR_CELL, 0x6200);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_3550, &args![tes, 1.0f32]);
        });
        assert_eq!(calls_to(&log, 0x00c4_68c0), vec![vec![0, 0]]);
        assert_eq!(calls_to(&log, 0x00c7_4550), vec![vec![0, 0]]);
        assert_eq!(
            calls_to(&log, 0x0055_1890),
            vec![vec![0x6200, 11.0f32.to_bits(), 0]]
        );
        assert_eq!(count_calls(&log, 0x004b_a9a0), 0);
    }

    #[test]
    fn frame_update_in_a_dialog_scales_the_sky_time() {
        let (mut e, tes) = frame_engine(1);
        returns(&mut e, 0x0070_50d0, 1);
        let sky = e.get(tes, TES::pSky).addr();
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_3550, &args![tes, 0.5f32]);
        });
        // `fAnimationMult` (0.5) times the frame time of `0084d030` (4.0),
        // for the update here and for `004537c0`.
        assert_eq!(
            calls_to(&log, 0x0063_ac70),
            vec![vec![sky, 2.0f32.to_bits()], vec![sky, 2.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, 0x0084_d030),
            vec![vec![FRAME_TIME_OBJECT], vec![FRAME_TIME_OBJECT]]
        );
    }

    #[test]
    fn frame_update_with_several_threads_runs_the_task_queue_and_skips_the_sky() {
        let (mut e, tes) = frame_engine(4);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_3550, &args![tes, 0.5f32]);
        });
        assert_eq!(calls_to(&log, 0x0087_aa90), vec![vec![0x1111]]);
        assert_eq!(count_calls(&log, 0x0063_ac70), 0);
        assert_eq!(count_calls(&log, 0x0045_37c0), 0);
        // The collision listener still runs.
        assert_eq!(count_calls(&log, 0x0062_3640), 1);
    }

    #[test]
    fn cell_main_thread_update_updates_the_sky_and_the_temp_node_manager() {
        let (mut e, tes) = frame_engine(1);
        e.set(tes, TES::pTempNodeManager, Ptr::new(0x9200));
        returns(&mut e, 0x0096_11e0, 0x9200);
        let sky = e.get(tes, TES::pSky).addr();
        let updates = Rc::new(RefCell::new(vec![]));
        let seen = updates.clone();
        e.register_double(UPDATE_OBJECT_CTOR, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_37c0, &args![tes]);
        });
        assert_eq!(
            calls_to(&log, 0x0063_ac70),
            vec![vec![sky, 0.25f32.to_bits()]]
        );
        // The update object holds the time accumulator, flags 0 and 0.
        let ctor = updates.borrow();
        assert_eq!(ctor.len(), 1);
        assert_eq!(ctor[0][1..], [10.0f32.to_bits(), 0, 0]);
        let node_updates = calls_to(&log, NODE_UPDATE);
        assert_eq!(node_updates.len(), 1);
        assert_eq!(node_updates[0][0], 0x9200);
        assert_eq!(node_updates[0][1], ctor[0][0]);
    }

    /// Stores the buffer contents of a `TES`: a table of `count` words.
    fn buffer(e: &mut Engine, words: &[u32]) -> u32 {
        let table = e.mem.alloc(4 * words.len() as u32 + 4);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(table + 4 * i as u32, *word);
        }
        table
    }

    /// An engine for `00453a80`: interior buffer of 3 slots, exterior buffer
    /// of 6 slots, a 2 x 2 loaded square, everything stubbed.
    fn unload_engine(interior: &[u32], exterior: &[u32]) -> (Engine, Ptr<TES>, u32, u32) {
        let (mut e, tes, _, _) = update_engine(2);
        put_setting(&mut e, SETTING_INTERIOR_CELL_BUFFER, interior.len() as u32);
        put_setting(&mut e, SETTING_EXTERIOR_CELL_BUFFER, exterior.len() as u32);
        let interior_table = buffer(&mut e, interior);
        let exterior_table = buffer(&mut e, exterior);
        e.set(tes, TES::pInteriorBuffer, Ptr::new(interior_table));
        e.set(tes, TES::pExteriorBuffer, Ptr::new(exterior_table));
        // The square holds 2 * 2 slots.
        returns(&mut e, 0x0084_e3a0, 2);
        (e, tes, interior_table, exterior_table)
    }

    #[test]
    fn unloading_takes_the_last_interior_buffer_cell_first() {
        let (mut e, tes, interior, _) = unload_engine(&[0x11, 0, 0x33], &[0; 6]);
        let handler: u32 = e.global(DATA_HANDLER);
        let log = run_logged(&mut e, |e| {
            assert!(e.call(0x0045_3a80, &args![tes]).bool());
        });
        assert_eq!(e.mem.u32(interior + 8), 0);
        assert_eq!(e.mem.u32(interior), 0x11);
        assert_eq!(calls_to(&log, 0x0046_2290), vec![vec![handler, 0x33]]);
        // The exterior buffer was not looked at.
        assert_eq!(count_calls(&log, 0x0045_4fc0), 0);
    }

    #[test]
    fn unloading_without_an_interior_cell_takes_a_buffered_exterior_cell() {
        // The first 4 slots are the loaded square; the search runs from the
        // end down to slot 4: slot 5 is loaded (cell state 3 is not what
        // `TES::IsCellLoaded` calls loaded, state 6 is), slot 4 is free.
        let loaded = 0x2000;
        let (mut e, tes, _, exterior) = unload_engine(&[0, 0, 0], &[0, 0, 0, 0, 0x2100, loaded]);
        e.map(loaded, 0x100);
        e.mem.set_u8(loaded + 0x26, 6);
        e.map(0x2100, 0x100);
        e.mem.set_u8(0x2100 + 0x26, 3);
        let log = run_logged(&mut e, |e| {
            assert!(e.call(0x0045_3a80, &args![tes]).bool());
        });
        assert_eq!(count_calls(&log, 0x0045_4fc0), 1);
        // Slot 4 was taken (slot 5 is loaded and stays).
        assert_eq!(e.mem.u32(exterior + 16), 0);
        assert_eq!(e.mem.u32(exterior + 20), loaded);
        assert_eq!(count_calls(&log, 0x0046_2290), 1);
        assert_eq!(calls_to(&log, 0x0046_2290)[0][1], 0x2100);
    }

    #[test]
    fn unloading_skips_cells_the_object_check_pins() {
        let (mut e, tes, _, exterior) = unload_engine(&[0, 0, 0], &[0, 0, 0, 0, 0x2100, 0x2200]);
        for cell in [0x2100u32, 0x2200] {
            e.map(cell, 0x100);
            e.mem.set_u8(cell + 0x26, 3);
        }
        // `00557090` pins the last one.
        e.register_double(0x0055_7090, |_, a| Ret {
            eax: (a[0] == 0x2200) as u32,
            ..Ret::default()
        });
        assert!(e.call(0x0045_3a80, &args![tes]).bool());
        assert_eq!(e.mem.u32(exterior + 16), 0);
        assert_eq!(e.mem.u32(exterior + 20), 0x2200);
    }

    #[test]
    fn unloading_nothing_returns_false() {
        let (mut e, tes, _, _) = unload_engine(&[0, 0, 0], &[0; 6]);
        let log = run_logged(&mut e, |e| {
            assert!(!e.call(0x0045_3a80, &args![tes]).bool());
        });
        assert_eq!(count_calls(&log, 0x0046_2290), 0);
    }

    #[test]
    fn unloading_an_exterior_cell_tells_the_terrain_when_the_first_slot_is_empty() {
        let (mut e, tes, _, _) = unload_engine(&[0, 0, 0], &[0, 0, 0, 0, 0x2100, 0]);
        e.map(0x2100, 0x100);
        e.mem.set_u8(0x2100 + 0x26, 3);
        let log = run_logged(&mut e, |e| {
            assert!(e.call(0x0045_3a80, &args![tes]).bool());
        });
        assert_eq!(calls_to(&log, 0x006f_ce00), vec![vec![0x7001]]);
    }

    #[test]
    fn unloading_with_an_interior_cell_searches_the_exterior_buffer_first() {
        let (mut e, tes, interior, exterior) =
            unload_engine(&[0x11, 0x6200, 0x33], &[0, 0, 0, 0, 0x2100, 0]);
        returns(&mut e, GET_INTERIOR_CELL, 0x6200);
        e.map(0x2100, 0x100);
        e.mem.set_u8(0x2100 + 0x26, 3);
        assert!(e.call(0x0045_3a80, &args![tes]).bool());
        assert_eq!(e.mem.u32(exterior + 16), 0);
        assert_eq!(e.mem.u32(interior + 8), 0x33);
    }

    #[test]
    fn unloading_with_an_interior_cell_falls_back_to_the_other_interiors() {
        // The loaded interior cell (0x6200) is never taken.
        let (mut e, tes, interior, _) = unload_engine(&[0x11, 0x33, 0x6200], &[0; 6]);
        returns(&mut e, GET_INTERIOR_CELL, 0x6200);
        let handler: u32 = e.global(DATA_HANDLER);
        let log = run_logged(&mut e, |e| {
            assert!(e.call(0x0045_3a80, &args![tes]).bool());
        });
        assert_eq!(calls_to(&log, 0x0046_2290), vec![vec![handler, 0x33]]);
        assert_eq!(e.mem.u32(interior + 4), 0);
        assert_eq!(e.mem.u32(interior + 8), 0x6200);
    }

    #[test]
    fn unloading_with_an_interior_cell_finally_takes_from_the_loaded_square() {
        // Only the front of the exterior buffer holds a cell; taking it runs
        // `00453940`.
        let (mut e, tes, _, exterior) = unload_engine(&[0, 0, 0], &[0x2100, 0, 0, 0, 0, 0]);
        returns(&mut e, GET_INTERIOR_CELL, 0x6200);
        e.map(0x2100, 0x100);
        e.mem.set_u8(0x2100 + 0x26, 3);
        let log = run_logged(&mut e, |e| {
            assert!(e.call(0x0045_3a80, &args![tes]).bool());
        });
        assert_eq!(e.mem.u32(exterior), 0);
        assert_eq!(calls_to(&log, 0x0045_3940), vec![vec![tes.addr()]]);
        assert_eq!(calls_to(&log, 0x0046_2290)[0][1], 0x2100);
    }

    /// An engine for the cell switches (`00453dc0`, `00454450`).
    fn switch_engine() -> (Engine, Ptr<TES>, u32) {
        let (mut e, tes, context, _) = update_engine(5);
        install_player(&mut e, 0);
        put_setting(&mut e, SETTING_HAVOK_DEBUG, 1u8);
        put_setting(&mut e, SETTING_INTERIOR_CELL_BUFFER, 3u32);
        // A shadow scene node table with one entry.
        e.map(SHADOW_SCENE_NODE_TABLE, 16);
        e.set_global(SHADOW_SCENE_NODE_TABLE, 0xa000u32);
        // The grid's interior buffer.
        let interior_table = buffer(&mut e, &[0x11, 0, 0x33]);
        e.set(tes, TES::pInteriorBuffer, Ptr::new(interior_table));
        e.set(tes, TES::iSaveGridX, 0x7fff_ffff);
        e.set(tes, TES::iSaveGridY, 0x7fff_ffff);
        e.set(tes, TES::pObjRoot, Ptr::new(0x9300));
        e.set(tes, TES::pObjLandRoot, Ptr::new(0x9400));
        let sky = e.mem.alloc(0x100);
        e.set(tes, TES::pSky, Ptr::new(sky));
        // `00408da0` returns the (empty) name string of a cell.
        let empty = e.mem.alloc(8);
        returns(&mut e, 0x0040_8da0, empty);
        noop(&mut e, &[0x00b5_ddb0]);
        (e, tes, context)
    }

    #[test]
    fn switching_to_the_loaded_interior_cell_only_updates_the_grid() {
        let (mut e, tes, _) = switch_engine();
        returns(&mut e, GET_INTERIOR_CELL, 0x6200);
        let pos = position(&mut e, 2048.0, 2048.0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_3dc0, &args![tes, 0x6200u32, pos]);
        });
        // The havok world's debug byte and visual debugger are cleared.
        assert_eq!(e.mem.u8(0x8000 + 0x15), 0);
        assert_eq!(
            calls_to(&log, HAVOK_WORLD_SET_VISUAL_DEBUGGER),
            vec![vec![0x8000, 0]]
        );
        // Nothing of the switch ran: no detach, no cell attached.
        assert_eq!(count_calls(&log, CELL_DETACH), 0);
        assert_eq!(count_calls(&log, CELL_ATTACH_TO_WORLD), 0);
        assert_eq!(count_calls(&log, 0x0087_1dc0), 1);
    }

    #[test]
    fn switching_into_an_interior_attaches_it_and_builds_the_name_image() {
        let (mut e, tes, _) = switch_engine();
        // No interior is loaded yet. The new cell is an interior (0x6300).
        let cell = e.mem.alloc(0x100);
        // The cell name: a string pointer whose first byte is set.
        let name = e.mem.alloc(8);
        e.mem.set_u8(name, b'X');
        returns(&mut e, 0x0040_8da0, name);
        returns(&mut e, 0x0054_5cf0, 0xd000);
        returns(&mut e, 0x0041_b9a0, 0x8100); // the interior's own havok world
        returns(&mut e, 0x0042_5fd0, 1);
        returns(&mut e, 0x0045_4400, 0);
        returns(&mut e, 0x0070_ec90, 0xe000);
        e.register(TES_CREATE_TEXTURE_IMAGE, |e, a| {
            e.mem.set_u32(a[2], 0xe100);
            Ret::default()
        });
        returns(&mut e, 0x0045_4bd0, 1);
        let pos = position(&mut e, 3.0 * 4096.0 + 5.0, 4.0 * 4096.0 + 5.0);
        e.set(tes, TES::iSaveGridX, 1);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_3dc0, &args![tes, cell, pos]);
        });
        // The grid is set to the position's cell.
        assert_eq!(e.get(tes, TES::iCurrentGridX), 3);
        assert_eq!(e.get(tes, TES::iCurrentGridY), 4);
        // The cell is attached to the object root, its lights attached, its
        // temp data loaded.
        assert_eq!(
            calls_to(&log, CELL_ATTACH_TO_WORLD),
            vec![vec![cell, 0x9300]]
        );
        assert_eq!(calls_to(&log, CELL_ATTACH_LIGHTS), vec![vec![cell, 1]]);
        assert_eq!(calls_to(&log, CELL_LOAD_ALL_TEMP_DATA), vec![vec![cell]]);
        // The name image: the memory tag guard 0x1d with the source line
        // 0xc74, the image built from the name and handed to the water
        // system.
        assert_eq!(
            calls_to(&log, SCOPE_GUARD_CTOR)
                .iter()
                .map(|w| w[1..].to_vec())
                .collect::<Vec<_>>(),
            vec![vec![0x1d, 1, TES_CPP_PATH, 0xc74]]
        );
        let create = calls_to(&log, TES_CREATE_TEXTURE_IMAGE);
        assert_eq!(create.len(), 1);
        assert_eq!(create[0][0], tes.addr());
        assert_eq!(create[0][1], name);
        assert_eq!((create[0][3], create[0][4]), (1, 0));
        assert_eq!(
            calls_to(&log, WATER_SYSTEM_SET_TEXTURE),
            vec![vec![0xe000, 0xe100]]
        );
        // The pointer temporary is built and destroyed around it.
        assert_eq!(count_calls(&log, NI_POINTER_CTOR), 1);
        assert_eq!(count_calls(&log, NI_POINTER_DTOR), 1);
        // The new cell is the loaded one.
        assert_eq!(calls_to(&log, 0x008d_7dc0), vec![vec![tes.addr(), cell]]);
    }

    #[test]
    fn switching_to_the_exterior_detaches_the_interior_and_clears_the_name() {
        let (mut e, tes, _) = switch_engine();
        returns(&mut e, GET_INTERIOR_CELL, 0x6200);
        // The interior cell's name is empty.
        let empty = e.mem.alloc(8);
        returns(&mut e, 0x0040_8da0, empty);
        returns(&mut e, 0x0070_ec90, 0xe000);
        let pos = position(&mut e, 2048.0, 2048.0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_3dc0, &args![tes, 0u32, pos]);
        });
        assert_eq!(calls_to(&log, CELL_DETACH), vec![vec![0x6200, 1]]);
        // The new cell is null: the loaded cell is set to null; no cell
        // attach.
        assert_eq!(calls_to(&log, 0x008d_7dc0), vec![vec![tes.addr(), 0]]);
        assert_eq!(count_calls(&log, CELL_ATTACH_TO_WORLD), 0);
        // No name: the water system gets a null image.
        assert_eq!(
            calls_to(&log, WATER_SYSTEM_SET_TEXTURE),
            vec![vec![0xe000, 0]]
        );
        assert_eq!(count_calls(&log, SCOPE_GUARD_CTOR), 0);
        // The buffered interior cell is released through `00455330` when
        // the new cell is not in the buffer.
        assert_eq!(calls_to(&log, 0x0045_5330), vec![vec![tes.addr(), 0x33]]);
        // The object-lighting reset of the leaving case.
        assert_eq!(count_calls(&log, 0x00b4_f9a0), 1);
        assert_eq!(count_calls(&log, 0x006c_0720), 1);
    }

    #[test]
    fn switching_does_not_buffer_a_cell_that_is_already_buffered() {
        let (mut e, tes, _) = switch_engine();
        returns(&mut e, 0x0045_4bd0, 1);
        returns(&mut e, GET_INTERIOR_CELL, 0x6200);
        let pos = position(&mut e, 2048.0, 2048.0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_3dc0, &args![tes, 0u32, pos]);
        });
        assert_eq!(count_calls(&log, 0x0045_5330), 0);
    }

    #[test]
    fn switching_with_a_script_running_skips_the_menu_background_and_the_obstacle_refresh() {
        let (mut e, tes, context) = switch_engine();
        e.mem.set_u32(context + 0x244, 2);
        returns(&mut e, GET_INTERIOR_CELL, 0x6200);
        let pos = position(&mut e, 2048.0, 2048.0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_3dc0, &args![tes, 0u32, pos]);
        });
        assert_eq!(count_calls(&log, 0x0086_ff70), 0);
        assert_eq!(count_calls(&log, 0x006c_0720), 0);
    }

    #[test]
    fn leaving_an_interior_restores_the_saved_grid_and_starts_the_fade() {
        let (mut e, tes, _) = switch_engine();
        returns(&mut e, GET_INTERIOR_CELL, 0x6200);
        e.set(tes, TES::iSaveGridX, 7);
        e.set(tes, TES::iSaveGridY, 8);
        // A cell in the first exterior buffer slot: the saved grid is
        // restored and the grid cells culled.
        let exterior = buffer(&mut e, &[0x2100, 0, 0]);
        e.set(tes, TES::pExteriorBuffer, Ptr::new(exterior));
        e.map(0x2100, 0x100);
        let empty = e.mem.alloc(8);
        returns(&mut e, 0x0040_8da0, empty);
        returns(&mut e, 0x0070_ec90, 0xe000);
        returns(&mut e, 0x0045_4b50, empty);
        // The position is the centre of the saved cell, so the grid stays.
        let pos = position(&mut e, 7.0 * 4096.0 + 2048.0, 8.0 * 4096.0 + 2048.0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_4450, &args![tes, pos]);
        });
        // The terrain manager is updated with the position and told to
        // update its morph parameters.
        assert_eq!(
            calls_to(&log, TERRAIN_MANAGER_UPDATE),
            vec![vec![0x7001, pos, 0xf]]
        );
        assert_eq!(calls_to(&log, 0x006f_cdb0), vec![vec![0x7001, pos]]);
        // The interior is detached; the cell test (`00454bd0` false) unloads it.
        assert_eq!(calls_to(&log, CELL_DETACH), vec![vec![0x6200, 1]]);
        let handler: u32 = e.global(DATA_HANDLER);
        assert_eq!(calls_to(&log, 0x0046_2290), vec![vec![handler, 0x6200]]);
        assert_eq!(e.get(tes, TES::iSaveGridX), 0x7fff_ffff);
        assert_eq!(e.get(tes, TES::iSaveGridY), 0x7fff_ffff);
        // Culling ran with the restored grid position.
        assert_eq!(e.get(tes, TES::iCurrentGridX), 7);
        assert_eq!(e.get(tes, TES::iCurrentGridY), 8);
        // The havok world: the visual debugger gets the setting's value.
        assert_eq!(
            calls_to(&log, HAVOK_WORLD_SET_VISUAL_DEBUGGER),
            vec![vec![0x8000, 1]]
        );
        // The fade is started (no cell test, no script) with the setting.
        assert_eq!(
            calls_to(&log, FADER_CREATE_FADER),
            vec![vec![e.global(FADER_MANAGER), 1, 0.25f32.to_bits(), 1]]
        );
        // The new grid centre and flags.
        assert_eq!(e.global::<u8>(FLAG_01189625), 1);
        assert_eq!(e.global::<u8>(GRID_FLAG_MOVED), 0);
        assert_eq!(e.global::<u8>(GRID_FLAG_MOVED_COPY), 0);
        for base in [GRID_CENTRE, GRID_PREVIOUS_CENTRE] {
            assert_eq!(e.global::<f32>(base), 7.5 * 4096.0);
            assert_eq!(e.global::<f32>(base + 4), 8.5 * 4096.0);
        }
        assert_eq!(e.global::<f32>(GRID_FLOAT_011F94AC), 1.5);
        assert_eq!(e.global::<u8>(GRID_FLAG_TERRAIN), 0);
    }

    #[test]
    fn cell_accessors_pass_their_offsets_on() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x100);
        e.call(0x0045_4380, &args![object, 7u8]);
        assert_eq!(e.mem.u8(object + 0x15), 7);
        returns(&mut e, 0x0040_8da0, 0x1234);
        returns(&mut e, 0x005a_e380, 3);
        let log = run_logged(&mut e, |e| {
            assert_eq!(e.call(0x0045_43a0, &args![object]).u32(), 0x1234);
            assert_eq!(e.call(0x0045_4400, &args![object]).u32(), 3);
        });
        assert_eq!(calls_to(&log, 0x0040_8da0), vec![vec![object + 0x58]]);
        assert_eq!(calls_to(&log, 0x005a_e380), vec![vec![object + 0xac]]);
    }

    #[test]
    fn interior_buffer_slots_are_read_by_index() {
        let mut e = Engine::new();
        let tes: Ptr<TES> = e.new_object();
        let table = buffer(&mut e, &[0x11, 0x22, 0x33]);
        e.set(tes, TES::pInteriorBuffer, Ptr::new(table));
        assert_eq!(e.call(0x0045_4420, &args![tes, 0u32]).u32(), 0x11);
        assert_eq!(e.call(0x0045_4420, &args![tes, 2u32]).u32(), 0x33);
    }

    #[test]
    fn a_cells_havok_world_is_its_own_for_an_interior() {
        let mut e = Engine::new();
        e.map(EXTERIOR_WORLD, 4);
        e.set_global(EXTERIOR_WORLD, 0x8000u32);
        install_getters(&mut e);
        returns(&mut e, 0x0041_b9a0, 0x8100);
        returns(&mut e, 0x0042_5fd0, 1);
        let cell = e.mem.alloc(0x100);
        let log = run_logged(&mut e, |e| {
            assert_eq!(e.call(0x0045_43c0, &args![cell]).u32(), 0x8100);
        });
        assert_eq!(calls_to(&log, 0x0041_b9a0), vec![vec![cell + 0x28]]);
        // An exterior cell uses the exterior world.
        returns(&mut e, 0x0042_5fd0, 0);
        assert_eq!(e.call(0x0045_43c0, &args![cell]).u32(), 0x8000);
    }

    #[test]
    fn havok_debug_setting_getters_read_the_byte() {
        let mut e = settings_engine();
        put_setting(&mut e, SETTING_HAVOK_DEBUG, 1u8);
        assert_eq!(e.call(0x0045_4ae0, &args![]).u8(), 1);
        put_setting(&mut e, 0x0300_0000, 0x37u8);
        assert_eq!(e.call(0x0045_4af0, &args![0x0300_0000u32]).u8(), 0x37);
        put_setting(&mut e, SETTING_HAVOK_DEBUG, 0u8);
        assert_eq!(e.call(0x0045_4ae0, &args![]).u8(), 0);
    }

    // BEGIN SESSION 4 TESTS

    // ------------------------------------------------------------------
    // Session 4 (`00454b10` to `00457990`)
    // ------------------------------------------------------------------

    /// An engine for the cell buffer functions: the settings (`grids` as
    /// `uGridsToLoad`, the buffer lengths from the slices), a `TES` whose
    /// buffers hold `interior` and `exterior`, the data handler pointer
    /// `0x6000`, and the unload and pin callees stubbed (nothing pinned,
    /// no interior cell loaded).
    fn buffers_engine(
        grids: u32,
        interior: &[u32],
        exterior: &[u32],
    ) -> (Engine, Ptr<TES>, u32, u32) {
        let mut e = settings_engine();
        put_setting(&mut e, SETTING_GRIDS_TO_LOAD, grids);
        put_setting(&mut e, SETTING_INTERIOR_CELL_BUFFER, interior.len() as u32);
        put_setting(&mut e, SETTING_EXTERIOR_CELL_BUFFER, exterior.len() as u32);
        let tes: Ptr<TES> = e.new_object();
        let interior_table = buffer(&mut e, interior);
        let exterior_table = buffer(&mut e, exterior);
        e.set(tes, TES::pInteriorBuffer, Ptr::new(interior_table));
        e.set(tes, TES::pExteriorBuffer, Ptr::new(exterior_table));
        e.map(DATA_HANDLER, 4);
        e.set_global(DATA_HANDLER, 0x6000u32);
        noop(&mut e, &[DATA_HANDLER_UNLOAD_CELL]);
        returns(&mut e, CELL_PINNED, 0);
        returns(&mut e, GET_INTERIOR_CELL, 0);
        returns(&mut e, CELL_IS_INTERIOR, 0);
        (e, tes, interior_table, exterior_table)
    }

    /// A cell object whose `cCellState` byte (+0x26) is `state`.
    fn cell_object(e: &mut Engine, state: u8) -> u32 {
        let cell = e.mem.alloc(0x100);
        e.mem.set_u8(cell + 0x26, state);
        cell
    }

    /// The words of a buffer table.
    fn words(e: &Engine, table: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(table + 4 * i)).collect()
    }

    #[test]
    fn cell_flags_bit_0x80_test() {
        let mut e = Engine::new();
        let cell = e.mem.alloc(0x40);
        e.mem.set_u8(cell + 0x24, 0x7f);
        assert!(!e.call(0x0045_4b10, &args![cell]).bool());
        e.mem.set_u8(cell + 0x24, 0x81);
        assert!(e.call(0x0045_4b10, &args![cell]).bool());
    }

    #[test]
    fn word_at_0x1e0_is_returned() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x200);
        e.mem.set_u32(object + 0x1e0, 0x1234);
        assert_eq!(e.call(0x0045_4b30, &args![object]).u32(), 0x1234);
    }

    #[test]
    #[should_panic(expected = "INT3")]
    fn the_int3_padding_byte_raises_a_breakpoint() {
        let mut e = Engine::new();
        e.call(0x0045_4b48, &args![]);
    }

    #[test]
    fn world_space_location_name_is_the_map_marker_name() {
        let mut e = Engine::new();
        returns(&mut e, 0x0040_8da0, 0x5555);
        let log = run_logged(&mut e, |e| {
            assert_eq!(e.call(0x0045_4b50, &args![0x3000u32]).u32(), 0x5555);
        });
        assert_eq!(calls_to(&log, 0x0040_8da0), vec![vec![0x3000 + 0xe0]]);
    }

    #[test]
    fn word_at_4_non_null_test() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        assert!(!e.call(0x0045_4b70, &args![object]).bool());
        e.mem.set_u32(object + 4, 9);
        assert!(e.call(0x0045_4b70, &args![object]).bool());
    }

    #[test]
    fn add_to_buffer_sends_an_interior_cell_to_the_interior_buffer() {
        let (mut e, tes, interior, exterior) = buffers_engine(1, &[0, 0, 0], &[0, 0, 0]);
        returns(&mut e, CELL_IS_INTERIOR, 1);
        let cell = cell_object(&mut e, 0);
        e.call(0x0045_4b90, &args![tes, cell]);
        assert_eq!(words(&e, interior, 3), vec![cell, 0, 0]);
        assert_eq!(words(&e, exterior, 3), vec![0, 0, 0]);
    }

    #[test]
    fn add_to_buffer_sends_an_exterior_cell_to_the_exterior_buffer() {
        let (mut e, tes, interior, exterior) = buffers_engine(1, &[0, 0, 0], &[0, 0, 0]);
        let cell = cell_object(&mut e, 0);
        e.call(0x0045_4b90, &args![tes, cell]);
        assert_eq!(words(&e, interior, 3), vec![0, 0, 0]);
        assert_eq!(words(&e, exterior, 3), vec![cell, 0, 0]);
    }

    #[test]
    fn add_to_buffer_ignores_a_null_cell() {
        let (mut e, tes, interior, _) = buffers_engine(1, &[7, 8, 9], &[0, 0, 0]);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_4b90, &args![tes, 0u32]);
        });
        assert_eq!(addresses(&log), vec![0x0045_4b90]);
        assert_eq!(words(&e, interior, 3), vec![7, 8, 9]);
    }

    #[test]
    fn interior_buffer_membership() {
        let (mut e, tes, _, _) = buffers_engine(1, &[0x11, 0x22, 0x33], &[0; 3]);
        assert!(e.call(0x0045_4bd0, &args![tes, 0x22u32]).bool());
        assert!(!e.call(0x0045_4bd0, &args![tes, 0x44u32]).bool());
    }

    #[test]
    fn a_new_interior_cell_pushes_the_buffer_down_and_unloads_the_last() {
        let (mut e, tes, interior, _) = buffers_engine(1, &[0x11, 0x22, 0x33], &[0; 3]);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_4c20, &args![tes, 0x44u32]);
        });
        assert_eq!(words(&e, interior, 3), vec![0x44, 0x11, 0x22]);
        assert_eq!(
            calls_to(&log, DATA_HANDLER_UNLOAD_CELL),
            vec![vec![0x6000, 0x33]]
        );
    }

    #[test]
    fn a_new_interior_cell_with_a_free_last_slot_unloads_nothing() {
        let (mut e, tes, interior, _) = buffers_engine(1, &[0x11, 0x22, 0], &[0; 3]);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_4c20, &args![tes, 0x44u32]);
        });
        assert_eq!(words(&e, interior, 3), vec![0x44, 0x11, 0x22]);
        assert_eq!(count_calls(&log, DATA_HANDLER_UNLOAD_CELL), 0);
    }

    #[test]
    fn a_buffered_interior_cell_moves_to_the_front() {
        let (mut e, tes, interior, _) = buffers_engine(1, &[0x11, 0x22, 0x33], &[0; 3]);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_4c20, &args![tes, 0x33u32]);
        });
        assert_eq!(words(&e, interior, 3), vec![0x33, 0x11, 0x22]);
        assert_eq!(count_calls(&log, DATA_HANDLER_UNLOAD_CELL), 0);
    }

    #[test]
    fn unloading_the_interior_buffer_keeps_pinned_cells_and_the_loaded_one() {
        let (mut e, tes, interior, _) = buffers_engine(1, &[0x11, 0, 0x22, 0x33], &[0; 3]);
        e.register_double(CELL_PINNED, |_, a| Ret {
            eax: (a[0] == 0x33) as u32,
            ..Ret::default()
        });
        returns(&mut e, GET_INTERIOR_CELL, 0x22);
        let log = run_logged(&mut e, |e| {
            assert_eq!(e.call(0x0045_4d50, &args![tes, 1u32]).i32(), 1);
        });
        // 0x11 went; 0x22 is the loaded interior cell; 0x33 is pinned.
        assert_eq!(words(&e, interior, 4), vec![0, 0, 0x22, 0x33]);
        assert_eq!(
            calls_to(&log, DATA_HANDLER_UNLOAD_CELL),
            vec![vec![0x6000, 0x11]]
        );
    }

    #[test]
    fn unloading_the_interior_buffer_without_the_keep_flag_takes_the_loaded_cell_too() {
        let (mut e, tes, interior, _) = buffers_engine(1, &[0x11, 0, 0x22, 0x33], &[0; 3]);
        e.register_double(CELL_PINNED, |_, a| Ret {
            eax: (a[0] == 0x33) as u32,
            ..Ret::default()
        });
        returns(&mut e, GET_INTERIOR_CELL, 0x22);
        assert_eq!(e.call(0x0045_4d50, &args![tes, 0u32]).i32(), 2);
        assert_eq!(words(&e, interior, 4), vec![0, 0, 0, 0x33]);
    }

    #[test]
    fn exterior_buffer_membership() {
        let (mut e, tes, _, _) = buffers_engine(1, &[0; 3], &[0x11, 0x22, 0x33]);
        assert!(e.call(0x0045_4e20, &args![tes, 0x33u32]).bool());
        assert!(!e.call(0x0045_4e20, &args![tes, 0x44u32]).bool());
    }

    #[test]
    fn a_new_exterior_cell_goes_after_the_loaded_square_and_pushes_the_rest_down() {
        // 2 x 2 loaded square (4 slots), then two buffered cells; the last
        // buffered cell falls off the end and is unloaded.
        let a = cell_object_for_buffers();
        let (mut e, tes, _, exterior) =
            buffers_engine(2, &[0; 3], &[a[0], a[1], a[2], a[3], 0x2100, 0x2200]);
        let cell = cell_object(&mut e, 3);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_4e70, &args![tes, cell]);
        });
        assert_eq!(
            words(&e, exterior, 6),
            vec![a[0], a[1], a[2], a[3], cell, 0x2100]
        );
        let handler: u32 = e.global(DATA_HANDLER);
        assert_eq!(
            calls_to(&log, DATA_HANDLER_UNLOAD_CELL),
            vec![vec![handler, 0x2200]]
        );
        assert!(e.get(tes, TES::bUpdateGridString));
    }

    /// Four stand-in cell pointers for the loaded-square slots.
    fn cell_object_for_buffers() -> [u32; 4] {
        [0x1000, 0x1100, 0x1200, 0x1300]
    }

    #[test]
    fn an_exterior_cell_already_buffered_is_moved_up_without_an_unload() {
        let a = cell_object_for_buffers();
        let (mut e, tes, _, exterior) =
            buffers_engine(2, &[0; 3], &[a[0], a[1], a[2], a[3], 0x2100, 0x2200]);
        e.map(0x2200, 0x100);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_4e70, &args![tes, 0x2200u32]);
        });
        // The cell takes the first slot after the square; the old copy at
        // the end is the one overwritten.
        assert_eq!(
            words(&e, exterior, 6),
            vec![a[0], a[1], a[2], a[3], 0x2200, 0x2100]
        );
        assert_eq!(count_calls(&log, DATA_HANDLER_UNLOAD_CELL), 0);
    }

    #[test]
    fn a_pinned_exterior_cell_goes_to_the_front() {
        let a = cell_object_for_buffers();
        let (mut e, tes, _, exterior) =
            buffers_engine(2, &[0; 3], &[a[0], a[1], a[2], a[3], 0x2100, 0]);
        // State 6 makes `00450ff0` true.
        let cell = cell_object(&mut e, 6);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_4e70, &args![tes, cell]);
        });
        assert_eq!(
            words(&e, exterior, 6),
            vec![cell, a[0], a[1], a[2], a[3], 0x2100]
        );
        // The cell pushed off the end is the old slot 5, which was empty.
        assert_eq!(count_calls(&log, DATA_HANDLER_UNLOAD_CELL), 0);
    }

    /// An engine for `TES::SortExteriorBuffer`: 3 x 3 loaded square (9
    /// slots, half width 1), 13 exterior slots; cells `0x2100` (far),
    /// `0x2200` (near) and `0x2300` (far) in slots 9 to 11; the queued
    /// cell is (0, 0) and the current one is `current_x`.
    fn sort_engine(current_x: i32) -> (Engine, Ptr<TES>, u32) {
        let mut slots = vec![0x1000u32; 9];
        slots.extend([0x2100, 0x2200, 0x2300, 0]);
        let (mut e, tes, _, exterior) = buffers_engine(3, &[0; 3], &slots);
        e.set(tes, TES::iCurrentGridX, current_x);
        e.set(tes, TES::iCurrentQueuedX, 0);
        e.set(tes, TES::iCurrentQueuedY, 0);
        let positions = [(0x2100u32, 5, 5), (0x2200, 1, 0), (0x2300, 9, 9)];
        e.register_double(CELL_GET_DATA_X, move |_, a| Ret {
            eax: positions
                .iter()
                .find(|p| p.0 == a[0])
                .map_or(0, |p| p.1 as u32),
            ..Ret::default()
        });
        e.register_double(CELL_GET_DATA_Y, move |_, a| Ret {
            eax: positions
                .iter()
                .find(|p| p.0 == a[0])
                .map_or(0, |p| p.2 as u32),
            ..Ret::default()
        });
        e.register_double(MEMMOVE, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        (e, tes, exterior)
    }

    #[test]
    fn sorting_moves_near_cells_before_far_ones() {
        let (mut e, tes, exterior) = sort_engine(1);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_4fc0, &args![tes]);
        });
        assert_eq!(words(&e, exterior + 36, 4), vec![0x2200, 0x2100, 0x2300, 0]);
        // One move: the far cell in slot 9 shifts up one slot (4 bytes).
        let moves = calls_to(&log, MEMMOVE);
        assert_eq!(moves, vec![vec![exterior + 40, exterior + 36, 4]]);
    }

    #[test]
    fn sorting_does_nothing_while_the_queued_x_is_the_current_x() {
        let (mut e, tes, exterior) = sort_engine(0);
        e.call(0x0045_4fc0, &args![tes]);
        assert_eq!(words(&e, exterior + 36, 4), vec![0x2100, 0x2200, 0x2300, 0]);
    }

    #[test]
    fn a_buffered_exterior_cell_moves_past_the_run_of_loaded_ones() {
        // Slots: other, cell, two cells for which `00450ff0` holds, empty.
        let other = 0x2100;
        let (mut e, tes, _, exterior) = buffers_engine(1, &[0; 3], &[0; 6]);
        let cell = cell_object(&mut e, 0);
        let first = cell_object(&mut e, 6);
        let second = cell_object(&mut e, 6);
        let table = [other, cell, first, second, 0, 0];
        for (i, value) in table.iter().enumerate() {
            e.mem.set_u32(exterior + 4 * i as u32, *value);
        }
        e.call(0x0045_5130, &args![tes, cell]);
        assert_eq!(
            words(&e, exterior, 6),
            vec![other, first, second, cell, 0, 0]
        );
    }

    #[test]
    fn a_cell_behind_an_empty_exterior_slot_is_left_alone() {
        let (mut e, tes, _, exterior) = buffers_engine(1, &[0; 3], &[0; 6]);
        let cell = cell_object(&mut e, 0);
        for (i, value) in [0, cell, 0, 0, 0, 0].iter().enumerate() {
            e.mem.set_u32(exterior + 4 * i as u32, *value);
        }
        e.call(0x0045_5130, &args![tes, cell]);
        assert_eq!(words(&e, exterior, 6), vec![0, cell, 0, 0, 0, 0]);
        // A null cell does nothing either.
        e.call(0x0045_5130, &args![tes, 0u32]);
        assert_eq!(words(&e, exterior, 6), vec![0, cell, 0, 0, 0, 0]);
    }

    /// An engine for `00455200`: the exterior buffer holds `loaded` (state 6),
    /// `plain` (state 3), `pinned` (state 3, pinned) and `foreign` (state 3,
    /// another world space); the world space of the others is `0x7001`.
    fn unload_all_engine() -> (Engine, Ptr<TES>, u32, [u32; 4]) {
        let cells = [0x3000u32, 0x3100, 0x3200, 0x3300];
        let (mut e, tes, _, exterior) = buffers_engine(1, &[0; 3], &cells);
        for (cell, state) in cells.iter().zip([6u8, 3, 3, 3]) {
            e.map(*cell, 0x100);
            e.mem.set_u8(cell + 0x26, state);
        }
        let pinned = cells[2];
        e.register_double(CELL_PINNED, move |_, a| Ret {
            eax: (a[0] == pinned) as u32,
            ..Ret::default()
        });
        let foreign = cells[3];
        e.register_double(CELL_GET_WORLD_SPACE, move |_, a| Ret {
            eax: if a[0] == foreign { 0x7002 } else { 0x7001 },
            ..Ret::default()
        });
        returns(&mut e, GET_WORLD_SPACE, 0x7001);
        returns(&mut e, GET_TERRAIN_MANAGER, 0x7100);
        noop(&mut e, &[0x006f_ce00]);
        (e, tes, exterior, cells)
    }

    #[test]
    fn unloading_the_exterior_buffer_spares_loaded_pinned_and_foreign_cells() {
        let (mut e, tes, exterior, cells) = unload_all_engine();
        let log = run_logged(&mut e, |e| {
            assert_eq!(e.call(0x0045_5200, &args![tes, 1u32, 0x7001u32]).i32(), 1);
        });
        assert_eq!(
            words(&e, exterior, 4),
            vec![cells[0], 0, cells[2], cells[3]]
        );
        assert_eq!(
            calls_to(&log, DATA_HANDLER_UNLOAD_CELL),
            vec![vec![0x6000, cells[1]]]
        );
        // Slot 0 is still set: the terrain manager is not told.
        assert_eq!(count_calls(&log, 0x006f_ce00), 0);
    }

    #[test]
    fn unloading_the_exterior_buffer_without_filters_takes_the_loaded_cell_and_tells_the_terrain() {
        let (mut e, tes, exterior, cells) = unload_all_engine();
        let log = run_logged(&mut e, |e| {
            assert_eq!(e.call(0x0045_5200, &args![tes, 0u32, 0u32]).i32(), 3);
        });
        assert_eq!(words(&e, exterior, 4), vec![0, 0, cells[2], 0]);
        assert_eq!(calls_to(&log, 0x006f_ce00), vec![vec![0x7100]]);
    }

    #[test]
    fn removing_an_interior_cell_unloads_it_and_closes_the_gap() {
        let (mut e, tes, interior, _) = buffers_engine(1, &[0x11, 0x22, 0x33], &[0; 3]);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_5330, &args![tes, 0x22u32]);
        });
        assert_eq!(words(&e, interior, 3), vec![0x11, 0x33, 0]);
        assert_eq!(
            calls_to(&log, DATA_HANDLER_UNLOAD_CELL),
            vec![vec![0x6000, 0x22]]
        );
        assert!(e.get(tes, TES::bUpdateGridString));
    }

    #[test]
    fn removing_a_missing_interior_cell_still_empties_the_last_slot() {
        let (mut e, tes, interior, _) = buffers_engine(1, &[0x11, 0x22, 0x33], &[0; 3]);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_5330, &args![tes, 0x99u32]);
        });
        assert_eq!(words(&e, interior, 3), vec![0x11, 0x22, 0]);
        assert_eq!(count_calls(&log, DATA_HANDLER_UNLOAD_CELL), 0);
        assert!(e.get(tes, TES::bUpdateGridString));
    }

    /// An engine for `00455400`: a sky at `0x9000` whose word at +0xf8 is
    /// `built`, recording the `SetMode` calls.
    fn sky_engine(built: u32) -> Engine {
        let mut e = Engine::new();
        returns(&mut e, TES_GET_SKY, 0x9000);
        returns(&mut e, SKY_GET_WORD_F8, built);
        returns(&mut e, CELL_IS_INTERIOR, 0);
        noop(&mut e, &[SKY_SET_MODE]);
        e
    }

    #[test]
    fn the_sky_mode_follows_the_cell() {
        let mut e = sky_engine(1);
        let tes = 0x4000u32;
        let cell = e.mem.alloc(0x40);
        let mode_of = |e: &mut Engine, cell: u32| {
            let log = run_logged(e, |e| {
                e.call(0x0045_5400, &args![tes, cell]);
            });
            calls_to(&log, SKY_SET_MODE)
        };
        assert_eq!(mode_of(&mut e, 0), vec![vec![0x9000, 3]]);
        // An exterior cell.
        assert_eq!(mode_of(&mut e, cell), vec![vec![0x9000, 3]]);
        // An interior cell without and with the 0x80 flag.
        returns(&mut e, CELL_IS_INTERIOR, 1);
        assert_eq!(mode_of(&mut e, cell), vec![vec![0x9000, 1]]);
        e.mem.set_u8(cell + 0x24, 0x80);
        assert_eq!(mode_of(&mut e, cell), vec![vec![0x9000, 2]]);
    }

    #[test]
    fn the_sky_mode_is_left_alone_while_no_sky_is_built() {
        let mut e = sky_engine(0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_5400, &args![0x4000u32, 0u32]);
        });
        assert_eq!(count_calls(&log, SKY_SET_MODE), 0);
    }

    /// An engine for `00455490`: two quests in the data handler's list (the
    /// first accepted by `00455620`, the second not; the first list node is
    /// embedded in the data handler at +0x118), a 2 x 2 grid of the cells
    /// `0x3000`, `0x3100`, `0x3200`, `0x3300` (state 6, so `00450ff0` is true)
    /// and the script calls stubbed; `TESObjectCELL::RunScripts` answers true
    /// for `scripted`. Returns the engine, the `TES` and the first quest.
    fn script_engine(scripted: u32) -> (Engine, Ptr<TES>, u32) {
        let mut e = settings_engine();
        put_setting(&mut e, SETTING_GRIDS_TO_LOAD, 2u32);
        e.map(SCRIPT_PASS_RUNNING, 4);
        e.map(DATA_HANDLER, 4);
        e.map(PLAYER_CHARACTER, 4);
        let handler = e.mem.alloc(0x200);
        e.set_global(DATA_HANDLER, handler);
        e.set_global(PLAYER_CHARACTER, 0x6500u32);
        let quest_a = e.mem.alloc(0x80);
        let quest_b = e.mem.alloc(0x80);
        e.mem.set_u8(quest_a + 0x3c, 1);
        e.mem.set_u8(quest_b + 0x3c, 0);
        let second_node = e.mem.alloc(8);
        e.mem.set_u32(second_node, quest_b);
        e.mem.set_u32(handler + 0x118, quest_a);
        e.mem.set_u32(handler + 0x118 + 4, second_node);
        e.register_double(LIST_NODE_ITEM, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register_double(LIST_NODE_NEXT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register_double(0x005a_8080, |_, a| Ret {
            eax: a[0] + 0x3c,
            ..Ret::default()
        });
        let tes: Ptr<TES> = e.new_object();
        let cells = [0x3000u32, 0x3100, 0x3200, 0x3300];
        let table = e.mem.alloc(16);
        for (i, cell) in cells.iter().enumerate() {
            e.map(*cell, 0x100);
            e.mem.set_u8(cell + 0x26, 6);
            e.mem.set_u32(table + 4 * i as u32, *cell);
        }
        e.set(tes, TES::pGridCellA, Ptr::new(0x6400));
        e.register_double(GRID_CELL_ARRAY_GET, move |_, a| Ret {
            eax: table + 4 * (a[1] * 2 + a[2]),
            ..Ret::default()
        });
        returns(&mut e, SCRIPT_GET_PROCESS_SCRIPTS, 1);
        returns(&mut e, GET_INTERIOR_CELL, 0);
        e.register_double(CELL_RUN_SCRIPTS, move |_, a| Ret {
            eax: (a[0] == scripted) as u32,
            ..Ret::default()
        });
        noop(
            &mut e,
            &[0x0040_fbf0, 0x0040_fba0, QUEST_RUN, REFERENCE_RUN_SCRIPT],
        );
        (e, tes, quest_a)
    }

    #[test]
    fn the_script_pass_runs_accepted_quests_and_stops_at_the_first_scripted_cell() {
        let (mut e, tes, quest_a) = script_engine(0x3100);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_5490, &args![tes]);
        });
        // Only the first quest (flag bit 0 set) runs.
        assert_eq!(calls_to(&log, QUEST_RUN), vec![vec![quest_a]]);
        // The grid is walked cell by cell; the walk stops at the cell whose
        // scripts ran.
        assert_eq!(
            calls_to(&log, CELL_RUN_SCRIPTS),
            vec![vec![0x3000, 0, 0], vec![0x3100, 0, 0]]
        );
        assert_eq!(calls_to(&log, REFERENCE_RUN_SCRIPT), vec![vec![0x6500]]);
        // The guard byte is back to zero and the pass object was entered and
        // left.
        assert_eq!(e.global::<u8>(SCRIPT_PASS_RUNNING), 0);
        assert_eq!(
            calls_to(&log, 0x0040_fbf0),
            vec![vec![SCRIPT_PASS_OBJECT, 0]]
        );
        assert_eq!(calls_to(&log, 0x0040_fba0), vec![vec![SCRIPT_PASS_OBJECT]]);
    }

    #[test]
    fn the_script_pass_walks_the_whole_grid_when_no_cell_ran_scripts() {
        let (mut e, tes, _) = script_engine(0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_5490, &args![tes]);
        });
        let cells: Vec<u32> = calls_to(&log, CELL_RUN_SCRIPTS)
            .iter()
            .map(|call| call[0])
            .collect();
        assert_eq!(cells, vec![0x3000, 0x3100, 0x3200, 0x3300]);
    }

    #[test]
    fn the_script_pass_uses_the_loaded_interior_cell_alone() {
        let (mut e, tes, _) = script_engine(0);
        returns(&mut e, GET_INTERIOR_CELL, 0x6200);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_5490, &args![tes]);
        });
        assert_eq!(calls_to(&log, CELL_RUN_SCRIPTS), vec![vec![0x6200, 0, 0]]);
        assert_eq!(count_calls(&log, GRID_CELL_ARRAY_GET), 0);
    }
    #[test]
    fn the_script_pass_waits_for_scripts_and_for_itself() {
        let (mut e, tes, _) = script_engine(0);
        returns(&mut e, SCRIPT_GET_PROCESS_SCRIPTS, 0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_5490, &args![tes]);
        });
        assert_eq!(count_calls(&log, 0x0040_fbf0), 0);
        returns(&mut e, SCRIPT_GET_PROCESS_SCRIPTS, 1);
        e.set_global(SCRIPT_PASS_RUNNING, 1u8);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_5490, &args![tes]);
        });
        assert_eq!(count_calls(&log, 0x0040_fbf0), 0);
        assert_eq!(e.global::<u8>(SCRIPT_PASS_RUNNING), 1);
    }

    #[test]
    fn the_data_handler_list_head_is_at_0x118() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0045_5600, &args![0x1000u32]).u32(), 0x1118);
    }

    #[test]
    fn a_quest_is_accepted_by_bit_0_of_its_flag_byte() {
        let mut e = Engine::new();
        let flags = e.mem.alloc(8);
        e.register_double(0x005a_8080, move |_, _| Ret {
            eax: flags,
            ..Ret::default()
        });
        assert!(!e.call(0x0045_5620, &args![0x1000u32]).bool());
        e.mem.set_u8(flags, 0x03);
        assert!(e.call(0x0045_5620, &args![0x1000u32]).bool());
        e.mem.set_u8(flags, 0x02);
        assert!(!e.call(0x0045_5620, &args![0x1000u32]).bool());
    }

    #[test]
    fn the_first_grid_pass_goes_to_the_interior_cell_or_the_grid() {
        let (mut e, tes, _, _) = buffers_engine(1, &[0; 3], &[0; 3]);
        e.map(COUNTER_011C56E8, 4);
        e.set_global(COUNTER_011C56E8, 77u32);
        e.set(tes, TES::pGridCellA, Ptr::new(0x6400));
        noop(&mut e, &[CELL_PASS_A, GRID_CELL_ARRAY_PASS_A]);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_5640, &args![tes]);
        });
        assert_eq!(e.global::<u32>(COUNTER_011C56E8), 0);
        assert_eq!(calls_to(&log, GRID_CELL_ARRAY_PASS_A), vec![vec![0x6400]]);
        assert_eq!(count_calls(&log, CELL_PASS_A), 0);
        returns(&mut e, GET_INTERIOR_CELL, 0x6200);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_5640, &args![tes]);
        });
        assert_eq!(calls_to(&log, CELL_PASS_A), vec![vec![0x6200]]);
        assert_eq!(count_calls(&log, GRID_CELL_ARRAY_PASS_A), 0);
    }

    #[test]
    fn the_counter_word_is_cleared() {
        let mut e = Engine::new();
        e.map(COUNTER_011C56E8, 4);
        e.set_global(COUNTER_011C56E8, 5u32);
        e.call(0x0045_5680, &args![]);
        assert_eq!(e.global::<u32>(COUNTER_011C56E8), 0);
    }

    #[test]
    fn the_second_grid_pass_goes_to_the_interior_cell_or_the_grid() {
        let (mut e, tes, _, _) = buffers_engine(1, &[0; 3], &[0; 3]);
        e.set(tes, TES::pGridCellA, Ptr::new(0x6400));
        noop(&mut e, &[CELL_PASS_B, GRID_CELL_ARRAY_PASS_B]);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_5690, &args![tes]);
        });
        assert_eq!(calls_to(&log, GRID_CELL_ARRAY_PASS_B), vec![vec![0x6400]]);
        returns(&mut e, GET_INTERIOR_CELL, 0x6200);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_5690, &args![tes]);
        });
        assert_eq!(calls_to(&log, CELL_PASS_B), vec![vec![0x6200]]);
    }

    // ---- the texture image cache and the face count ----

    /// `NiPointer` stand-ins: the constructor stores its argument in the
    /// holder, the getter reads it, the destructor clears it.
    fn install_ni_pointer(e: &mut Engine) {
        e.register_double(NI_POINTER_CTOR, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register_double(NI_POINTER_GET, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register_double(NI_POINTER_DTOR, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
    }

    #[test]
    fn the_face_count_sums_geometry_below_a_node_and_skips_hidden_geometry() {
        let mut e = Engine::new();
        install_ni_pointer(&mut e);
        // Slot 0xc answers for nodes (their own address), slot 0x1c for
        // geometry.
        e.register_double(0x0300_0001, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        noop(&mut e, &[0x0300_0002]);
        let node_a = object_with_slots(
            &mut e,
            0x0300_1000,
            &[(0xc, 0x0300_0001), (0x1c, 0x0300_0002)],
        );
        let node_b = object_with_slots(
            &mut e,
            0x0300_1000,
            &[(0xc, 0x0300_0001), (0x1c, 0x0300_0002)],
        );
        let geometry = |e: &mut Engine, faces: u16| {
            let object =
                object_with_slots(e, 0x0300_2000, &[(0xc, 0x0300_0002), (0x1c, 0x0300_0001)]);
            let data = e.mem.alloc(0x80);
            e.mem.set_u16(data + 0x40, faces);
            e.mem.set_u32(object + 0xb8, data);
            object
        };
        let shown = geometry(&mut e, 10);
        let deeper = geometry(&mut e, 5);
        let hidden = geometry(&mut e, 7);
        let other = object_with_slots(
            &mut e,
            0x0300_3000,
            &[(0xc, 0x0300_0002), (0x1c, 0x0300_0002)],
        );
        let children = vec![
            (node_a, vec![node_b, shown, other]),
            (node_b, vec![deeper, hidden]),
        ];
        let counts = children.clone();
        e.register_double(0x0043_b480, move |_, a| Ret {
            eax: counts
                .iter()
                .find(|c| c.0 == a[0])
                .map_or(0, |c| c.1.len() as u32),
            ..Ret::default()
        });
        e.register_double(0x0043_b4a0, move |_, a| Ret {
            eax: children
                .iter()
                .find(|c| c.0 == a[0])
                .map_or(0, |c| c.1[a[1] as usize]),
            ..Ret::default()
        });
        e.register_double(0x0045_6610, move |_, a| Ret {
            eax: (a[0] == hidden) as u32,
            ..Ret::default()
        });
        let tes = 0x4000u32;
        assert_eq!(e.call(0x0045_6670, &args![tes, node_a, 0u32]).i32(), 15);
        // With the "all" flag the hidden geometry counts too.
        assert_eq!(e.call(0x0045_6670, &args![tes, node_a, 1u32]).i32(), 22);
        assert_eq!(e.call(0x0045_6670, &args![tes, 0u32, 1u32]).i32(), 0);
    }

    #[test]
    fn the_geometry_face_count_is_read_through_the_data_pointer() {
        let mut e = Engine::new();
        install_getters(&mut e);
        let geometry = e.mem.alloc(0x100);
        let data = e.mem.alloc(0x80);
        e.mem.set_u16(data + 0x40, 0xfffe);
        e.mem.set_u32(geometry + 0xb8, data);
        assert_eq!(e.call(0x0045_67b0, &args![geometry]).u16(), 0xfffe);
    }

    #[test]
    fn the_data_face_count_is_the_word_at_0x40() {
        let mut e = Engine::new();
        let data = e.mem.alloc(0x80);
        e.mem.set_u16(data + 0x40, 0x1234);
        assert_eq!(e.call(0x0045_67e0, &args![data]).u16(), 0x1234);
    }

    /// An engine for the texture palette functions: a palette entry
    /// `texture` (a block with the count `references` at +4) found under
    /// `name`.
    fn palette_engine(references: u32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        install_ni_pointer(&mut e);
        let texture = e.mem.alloc(0x40);
        e.mem.set_u32(texture + 4, references);
        e.register_double(LIST_NODE_NEXT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        let name = e.mem.alloc(16);
        e.mem.set_cstr(name, b"tex.dds");
        e.register_double(0x00a6_1b90, move |e, a| {
            if a[0] == name {
                e.mem.set_u32(a[1], texture);
            }
            Ret::default()
        });
        noop(&mut e, &[0x00a6_1f30]);
        (e, name, texture)
    }

    #[test]
    fn a_texture_nobody_else_holds_is_removed_from_the_palette() {
        let (mut e, name, texture) = palette_engine(3);
        let log = run_logged(&mut e, |e| {
            assert!(e.call(0x0045_6800, &args![0x4000u32, name, 1u32]).bool());
        });
        assert_eq!(calls_to(&log, 0x00a6_1f30), vec![vec![texture]]);
        assert_eq!(count_calls(&log, NI_POINTER_DTOR), 1);
    }

    #[test]
    fn a_texture_others_still_hold_stays_in_the_palette() {
        let (mut e, name, _) = palette_engine(4);
        let log = run_logged(&mut e, |e| {
            assert!(!e.call(0x0045_6800, &args![0x4000u32, name, 1u32]).bool());
        });
        assert_eq!(count_calls(&log, 0x00a6_1f30), 0);
        assert_eq!(count_calls(&log, NI_POINTER_DTOR), 1);
    }

    #[test]
    fn a_texture_the_palette_does_not_know_is_not_removed() {
        let (mut e, _, _) = palette_engine(3);
        let other = e.mem.alloc(16);
        let log = run_logged(&mut e, |e| {
            assert!(!e.call(0x0045_6800, &args![0x4000u32, other, 1u32]).bool());
        });
        assert_eq!(count_calls(&log, 0x00a6_1f30), 0);
    }

    /// An engine for `TES::CreateTextureImage`: the palette knows nothing,
    /// `FileFinder::Exist` answers `exists`, the fixed-string constructor
    /// returns its first argument, `NiSourceTexture::Create` returns
    /// `created`; the assignment stores its second argument.
    fn create_texture_engine(exists: u32, created: u32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        install_ni_pointer(&mut e);
        e.register_double(0x0066_b0d0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        noop(
            &mut e,
            &[0x00a6_1b90, 0x00a6_1c50, DEBUG_PRINT, 0x0043_81b0],
        );
        returns(&mut e, 0x0045_6a20, exists);
        e.register_double(0x0043_8170, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        returns(&mut e, 0x00a5_fd70, created);
        let name = e.mem.alloc(16);
        e.mem.set_cstr(name, b"tex.dds");
        let out = e.mem.alloc(8);
        (e, name, out)
    }

    #[test]
    fn a_texture_already_in_the_palette_is_not_created_again() {
        let (mut e, name, out) = create_texture_engine(1, 0x5000);
        e.register_double(0x00a6_1b90, move |e, a| {
            e.mem.set_u32(a[1], 0x4444);
            Ret::default()
        });
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_68c0, &args![0x4000u32, name, out, 0u32, 0u32]);
        });
        assert_eq!(e.mem.u32(out), 0x4444);
        assert_eq!(count_calls(&log, 0x0045_6a20), 0);
        assert_eq!(count_calls(&log, 0x00a5_fd70), 0);
    }

    #[test]
    fn an_existing_texture_file_is_created_and_entered_in_the_palette() {
        let (mut e, name, out) = create_texture_engine(1, 0x5000);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_68c0, &args![0x4000u32, name, out, 1u32, 0u32]);
        });
        assert_eq!(e.mem.u32(out), 0x5000);
        assert_eq!(calls_to(&log, 0x0045_6a20), vec![vec![name, 0, 0, 0xffff]]);
        let create = calls_to(&log, 0x00a5_fd70);
        assert_eq!(create.len(), 1);
        assert_eq!(create[0][1..], [0x011a_9598, 1, 0]);
        assert_eq!(calls_to(&log, 0x00a6_1c50), vec![vec![0x5000, 0]]);
        assert_eq!(count_calls(&log, DEBUG_PRINT), 0);
    }

    #[test]
    fn the_alternate_flag_changes_the_search_mode() {
        let (mut e, name, out) = create_texture_engine(1, 0x5000);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_68c0, &args![0x4000u32, name, out, 0u32, 1u32]);
        });
        assert_eq!(calls_to(&log, 0x0045_6a20), vec![vec![name, 0, 6, 2]]);
    }

    #[test]
    fn a_missing_texture_file_is_skipped_when_quiet() {
        let (mut e, name, out) = create_texture_engine(0, 0x5000);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_68c0, &args![0x4000u32, name, out, 1u32, 0u32]);
        });
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(count_calls(&log, 0x00a5_fd70), 0);
        assert_eq!(count_calls(&log, DEBUG_PRINT), 0);
    }

    #[test]
    fn a_failed_texture_creation_is_reported_unless_quiet() {
        let (mut e, name, out) = create_texture_engine(0, 0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_68c0, &args![0x4000u32, name, out, 0u32, 0u32]);
        });
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(count_calls(&log, 0x00a6_1c50), 0);
        assert_eq!(calls_to(&log, DEBUG_PRINT), vec![vec![0x0101_7ca8, name]]);
    }

    // ---- the current cell and the land helpers ----

    #[test]
    fn the_current_cell_is_the_interior_cell_or_the_grid_centre() {
        let (mut e, tes, table) = grid_engine(3);
        returns(&mut e, GET_INTERIOR_CELL, 0);
        returns(&mut e, GET_WORLD_SPACE, 0);
        // No grid centre yet.
        e.set(tes, TES::iCurrentGridX, 0x7fff_ffff);
        e.set(tes, TES::iCurrentGridY, 4);
        assert_eq!(e.call(0x0045_7070, &args![tes]).u32(), 0);
        // The centre cell of the 3 x 3 square around (5, 6) is slot (1, 1).
        e.set(tes, TES::iCurrentGridX, 5);
        e.set(tes, TES::iCurrentGridY, 6);
        put_cell(&mut e, table, 3, 1, 1, 0x7777);
        assert_eq!(e.call(0x0045_7070, &args![tes]).u32(), 0x7777);
        // A loaded interior cell wins.
        returns(&mut e, GET_INTERIOR_CELL, 0x6200);
        assert_eq!(e.call(0x0045_7070, &args![tes]).u32(), 0x6200);
    }

    /// An engine for the cell tests of `004570d0`/`004571c0`: a 3 x 3 grid
    /// with cells in slots (0, 1) and (2, 2), the `TES` singleton set, no
    /// interior cell loaded.
    fn cell_tests_engine() -> (Engine, Ptr<TES>) {
        let (mut e, tes, table) = grid_engine(3);
        put_cell(&mut e, table, 3, 0, 1, 0x3100);
        put_cell(&mut e, table, 3, 2, 2, 0x3200);
        e.map(TES_SINGLETON, 4);
        e.set_global(TES_SINGLETON, tes.addr());
        returns(&mut e, GET_INTERIOR_CELL, 0);
        (e, tes)
    }

    #[test]
    fn the_first_cell_test_asks_every_grid_cell() {
        let (mut e, tes) = cell_tests_engine();
        e.register_double(CELL_TEST_A, |_, a| Ret {
            eax: (a[0] == 0x3200) as u32,
            ..Ret::default()
        });
        let fourth = 2.5f32;
        let log = run_logged(&mut e, |e| {
            assert!(e
                .call(0x0045_70d0, &args![tes, 0x8000u32, 1u32, 2u32, fourth])
                .bool());
        });
        assert_eq!(
            calls_to(&log, CELL_TEST_A),
            vec![
                vec![0x3100, 0x8000, 1, 2, fourth.to_bits()],
                vec![0x3200, 0x8000, 1, 2, fourth.to_bits()],
            ]
        );
    }

    #[test]
    fn the_first_cell_test_asks_only_the_interior_cell_when_there_is_one() {
        let (mut e, tes) = cell_tests_engine();
        returns(&mut e, GET_INTERIOR_CELL, 0x6200);
        returns(&mut e, CELL_TEST_A, 1);
        let log = run_logged(&mut e, |e| {
            assert!(e
                .call(0x0045_70d0, &args![tes, 0x8000u32, 1u32, 2u32, 0.0f32])
                .bool());
        });
        assert_eq!(count_calls(&log, CELL_TEST_A), 1);
        assert_eq!(calls_to(&log, CELL_TEST_A)[0][0], 0x6200);
        // Nothing to ask about without a target.
        let log = run_logged(&mut e, |e| {
            assert!(!e
                .call(0x0045_70d0, &args![tes, 0u32, 1u32, 2u32, 0.0f32])
                .bool());
        });
        assert_eq!(count_calls(&log, CELL_TEST_A), 0);
    }

    #[test]
    fn the_second_cell_test_prepares_the_target_and_asks_every_grid_cell() {
        let (mut e, tes) = cell_tests_engine();
        noop(&mut e, &[0x0070_5fc0, 0x00e9_8e20]);
        returns(&mut e, CELL_TEST_B, 0);
        let log = run_logged(&mut e, |e| {
            assert!(!e
                .call(
                    0x0045_71c0,
                    &args![tes, 0x8000u32, 1u32, 2u32, 1.0f32, 1u32]
                )
                .bool());
        });
        assert_eq!(calls_to(&log, 0x0070_5fc0), vec![vec![0x8000, 0]]);
        assert_eq!(
            calls_to(&log, 0x00e9_8e20),
            vec![vec![0x8000, NI_POINT3_ZERO, NI_POINT3_ZERO, 0]]
        );
        let asked = calls_to(&log, CELL_TEST_B);
        assert_eq!(asked.len(), 2);
        assert_eq!(asked[0], vec![0x3100, 0x8000, 1, 2, 1.0f32.to_bits(), 1]);
        // Without the flag nothing is prepared or asked.
        let log = run_logged(&mut e, |e| {
            assert!(!e
                .call(
                    0x0045_71c0,
                    &args![tes, 0x8000u32, 1u32, 2u32, 1.0f32, 0u32]
                )
                .bool());
        });
        assert_eq!(addresses(&log), vec![0x0045_71c0]);
    }

    #[test]
    fn the_second_cell_test_uses_the_interior_cell_alone_and_answers_with_it() {
        let (mut e, tes) = cell_tests_engine();
        noop(&mut e, &[0x0070_5fc0, 0x00e9_8e20]);
        returns(&mut e, GET_INTERIOR_CELL, 0x6200);
        returns(&mut e, CELL_TEST_B, 1);
        let log = run_logged(&mut e, |e| {
            assert!(e
                .call(
                    0x0045_71c0,
                    &args![tes, 0x8000u32, 1u32, 2u32, 1.0f32, 1u32]
                )
                .bool());
        });
        assert_eq!(count_calls(&log, CELL_TEST_B), 1);
        assert_eq!(calls_to(&log, CELL_TEST_B)[0][0], 0x6200);
    }

    /// A position engine (5 x 5 grid around cell (10, 20), a cell whose
    /// state is 3 (loaded for `TES::IsCellLoaded`) at the relative slot
    /// (0, 0), i.e. grid cell (8, 18)) with the `TES` singleton set, the
    /// land of every cell `0x9100` and `GetLandHeight` answering `height` in
    /// `*out` and true.
    fn land_engine(height: f32) -> (Engine, Ptr<TES>, u32) {
        let (mut e, tes, table) = grid_engine(5);
        install_float_to_int(&mut e);
        returns(&mut e, GET_INTERIOR_CELL, 0);
        e.set(tes, TES::iCurrentGridX, 10);
        e.set(tes, TES::iCurrentGridY, 20);
        e.map(TES_SINGLETON, 4);
        e.set_global(TES_SINGLETON, tes.addr());
        let cell = e.mem.alloc(0x100);
        e.mem.set_u8(cell + 0x26, 3);
        put_cell(&mut e, table, 5, 0, 0, cell);
        returns(&mut e, CELL_GET_LAND, 0x9100);
        e.register_double(LAND_GET_LAND_HEIGHT, move |e, a| {
            e.mem.set_f32(a[2], height);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.map(0x0101_7824, 4);
        e.set_global(0x0101_7824, -2048.0f32);
        returns(&mut e, GET_WORLD_SPACE, 0);
        (e, tes, cell)
    }

    #[test]
    fn the_land_height_is_asked_of_the_loaded_cell_under_the_position() {
        let (mut e, tes, _) = land_engine(123.5);
        let pos = position(&mut e, 8.0 * 4096.0 + 10.0, 18.0 * 4096.0 + 10.0);
        let out = e.mem.alloc(4);
        let log = run_logged(&mut e, |e| {
            assert!(e.call(0x0045_72e0, &args![tes, pos, out]).bool());
        });
        assert_eq!(e.mem.f32(out), 123.5);
        assert_eq!(
            calls_to(&log, LAND_GET_LAND_HEIGHT),
            vec![vec![0x9100, pos, out]]
        );
    }

    #[test]
    fn the_land_height_of_an_unloaded_cell_is_the_no_height_value() {
        let (mut e, tes, cell) = land_engine(123.5);
        e.mem.set_u8(cell + 0x26, 0);
        let pos = position(&mut e, 8.0 * 4096.0 + 10.0, 18.0 * 4096.0 + 10.0);
        let out = e.mem.alloc(4);
        assert!(!e.call(0x0045_72e0, &args![tes, pos, out]).bool());
        assert_eq!(e.mem.f32(out), -2048.0);
        // Outside the grid there is no cell at all.
        let far = position(&mut e, 100.0 * 4096.0, 100.0 * 4096.0);
        assert!(!e.call(0x0045_72e0, &args![tes, far, out]).bool());
    }

    #[test]
    fn a_position_on_a_cell_border_belongs_to_the_cell_below() {
        let (mut e, tes, _) = land_engine(1.0);
        let out = e.mem.alloc(4);
        // Exactly on the border x = 8 * 4096: cell 8 (the slot (0, 0)).
        let on_border = position(&mut e, 8.0 * 4096.0, 18.0 * 4096.0 + 1.0);
        assert!(e.call(0x0045_72e0, &args![tes, on_border, out]).bool());
        // Rounds up to the border from below: still cell 7, outside the grid.
        let below = position(&mut e, 8.0 * 4096.0 - 0.25, 18.0 * 4096.0 + 1.0);
        assert!(!e.call(0x0045_72e0, &args![tes, below, out]).bool());
    }

    #[test]
    fn a_position_below_the_integer_range_wraps_on_the_border_fix() {
        // 004572e0: `FISTP` gives 0x80000000 for -inf, a multiple of 4096 the exact value is
        // below, so `SUB ECX,1` (0045733a) wraps it to 0x7fffffff: outside the grid, no land.
        let (mut e, tes, _) = land_engine(1.0);
        let out = e.mem.alloc(4);
        let pos = position(&mut e, f32::NEG_INFINITY, 18.0 * 4096.0 + 1.0);
        assert!(!e.call(0x0045_72e0, &args![tes, pos, out]).bool());
        assert_eq!(e.mem.f32(out), -2048.0);
        assert_eq!(
            adjust_rounded_position(i32::MIN, f32::NEG_INFINITY),
            i32::MAX
        );
        // NaN compares unordered (`TEST AH,5` / `JP` taken): no step down.
        assert_eq!(adjust_rounded_position(i32::MIN, f32::NAN), i32::MIN);
    }

    #[test]
    fn the_second_land_query_fills_its_output_first() {
        let (mut e, tes, _) = land_engine(0.0);
        e.map(NI_POINT3_ZERO, 12);
        e.set_global(NI_POINT3_ZERO, 1.5f32);
        e.set_global(NI_POINT3_ZERO + 4, 2.5f32);
        e.register_double(0x0053_d2e0, |_, a| Ret {
            eax: (a[3] == 9) as u32,
            ..Ret::default()
        });
        let pos = position(&mut e, 8.0 * 4096.0 + 10.0, 18.0 * 4096.0 + 10.0);
        let out = e.mem.alloc(12);
        let log = run_logged(&mut e, |e| {
            assert!(e.call(0x0045_73f0, &args![tes, pos, out, 9u32]).bool());
        });
        assert_eq!(e.mem.f32(out), 1.5);
        assert_eq!(e.mem.f32(out + 4), 2.5);
        assert_eq!(e.mem.f32(out + 8), 1.0);
        assert_eq!(calls_to(&log, 0x0053_d2e0), vec![vec![0x9100, pos, out, 9]]);
        assert!(!e.call(0x0045_73f0, &args![tes, pos, out, 8u32]).bool());
    }

    #[test]
    fn the_third_land_query_answers_for_a_loaded_cell_only() {
        let (mut e, tes, cell) = land_engine(0.0);
        returns(&mut e, 0x0053_f570, 1);
        let pos = position(&mut e, 8.0 * 4096.0 + 10.0, 18.0 * 4096.0 + 10.0);
        let log = run_logged(&mut e, |e| {
            assert!(e.call(0x0045_7520, &args![tes, pos, 7u32]).bool());
        });
        assert_eq!(calls_to(&log, 0x0053_f570), vec![vec![0x9100, pos, 7]]);
        e.mem.set_u8(cell + 0x26, 0);
        assert!(!e.call(0x0045_7520, &args![tes, pos, 7u32]).bool());
    }

    #[test]
    fn the_fourth_land_query_returns_the_whole_answer() {
        let (mut e, tes, cell) = land_engine(0.0);
        returns(&mut e, 0x0053_f0e0, 0xabcd_1234);
        let pos = position(&mut e, 8.0 * 4096.0 + 10.0, 18.0 * 4096.0 + 10.0);
        let log = run_logged(&mut e, |e| {
            assert_eq!(e.call(0x0045_7620, &args![tes, pos]).u32(), 0xabcd_1234);
        });
        assert_eq!(calls_to(&log, 0x0053_f0e0), vec![vec![0x9100, pos]]);
        e.mem.set_u8(cell + 0x26, 0);
        assert_eq!(e.call(0x0045_7620, &args![tes, pos]).u32(), 0);
    }

    /// The cache globals of `00457720`.
    fn map_land_cache(e: &mut Engine, column_x: i32, answer: u32) {
        e.map(CACHED_LAND_COLUMN_X, 4);
        e.map(CACHED_LAND_COLUMN_Y, 4);
        e.map(CACHED_LAND_ANSWER, 4);
        e.set_global(CACHED_LAND_COLUMN_X, column_x);
        e.set_global(CACHED_LAND_ANSWER, answer);
    }

    #[test]
    fn the_cached_land_query_is_answered_from_the_cache_inside_a_column() {
        let (mut e, tes, _) = land_engine(0.0);
        // x = 8 * 4096 + 130: the column starts at +128.
        map_land_cache(&mut e, 8 * 4096 + 128, 0x7777);
        let pos = position(&mut e, 8.0 * 4096.0 + 130.0, 18.0 * 4096.0 + 10.0);
        let log = run_logged(&mut e, |e| {
            assert_eq!(e.call(0x0045_7720, &args![tes, pos]).u32(), 0x7777);
        });
        assert_eq!(count_calls(&log, 0x0053_a5e0), 0);
    }

    #[test]
    fn a_new_land_query_is_asked_and_cached() {
        let (mut e, tes, _) = land_engine(0.0);
        map_land_cache(&mut e, 0, 0x7777);
        returns(&mut e, 0x0053_a5e0, 0x8888);
        let pos = position(&mut e, 8.0 * 4096.0 + 130.0, 18.0 * 4096.0 + 200.0);
        let log = run_logged(&mut e, |e| {
            assert_eq!(e.call(0x0045_7720, &args![tes, pos]).u32(), 0x8888);
        });
        assert_eq!(calls_to(&log, 0x0053_a5e0), vec![vec![0x9100, pos]]);
        assert_eq!(e.global::<i32>(CACHED_LAND_COLUMN_X), 8 * 4096 + 128);
        assert_eq!(e.global::<i32>(CACHED_LAND_COLUMN_Y), 18 * 4096 + 128);
        assert_eq!(e.global::<u32>(CACHED_LAND_ANSWER), 0x8888);
    }

    #[test]
    fn an_empty_land_answer_or_a_missing_cell_is_not_cached() {
        let (mut e, tes, _) = land_engine(0.0);
        map_land_cache(&mut e, 0, 0x7777);
        returns(&mut e, 0x0053_a5e0, 0);
        let pos = position(&mut e, 8.0 * 4096.0 + 130.0, 18.0 * 4096.0 + 200.0);
        assert_eq!(e.call(0x0045_7720, &args![tes, pos]).u32(), 0);
        assert_eq!(e.global::<i32>(CACHED_LAND_COLUMN_X), 0);
        let far = position(&mut e, 100.0 * 4096.0 + 130.0, 100.0 * 4096.0);
        assert_eq!(e.call(0x0045_7720, &args![tes, far]).u32(), 0);
        assert_eq!(e.global::<u32>(CACHED_LAND_ANSWER), 0x7777);
    }

    #[test]
    fn negative_positions_are_rounded_toward_zero_for_the_column() {
        let (mut e, tes, _) = land_engine(0.0);
        map_land_cache(&mut e, -128, 0x4242);
        // x = -130: remainder -2, column -128 (the cache hit).
        let pos = position(&mut e, -130.0, 5.0);
        assert_eq!(e.call(0x0045_7720, &args![tes, pos]).u32(), 0x4242);
    }

    #[test]
    fn the_land_byte_is_read_from_what_the_cached_query_finds() {
        let (mut e, tes, _) = land_engine(0.0);
        map_land_cache(&mut e, 8 * 4096 + 128, 0x7777);
        returns(&mut e, 0x0054_1570, 5);
        let pos = position(&mut e, 8.0 * 4096.0 + 130.0, 18.0 * 4096.0 + 10.0);
        let log = run_logged(&mut e, |e| {
            assert_eq!(e.call(0x0045_7880, &args![tes, pos]).u32(), 5);
        });
        assert_eq!(calls_to(&log, 0x0054_1570), vec![vec![0x7777]]);
        // Nothing found: 2.
        e.set_global(CACHED_LAND_ANSWER, 0u32);
        assert_eq!(e.call(0x0045_7880, &args![tes, pos]).u32(), 2);
    }

    // ---- three-float helpers ----

    #[test]
    fn point_subtraction_works_on_each_component() {
        let mut e = Engine::new();
        let a = e.mem.alloc(12);
        let b = e.mem.alloc(12);
        for (i, v) in [5.0f32, 3.0, -1.0].iter().enumerate() {
            e.mem.set_f32(a + 4 * i as u32, *v);
        }
        for (i, v) in [1.0f32, 4.0, 0.5].iter().enumerate() {
            e.mem.set_f32(b + 4 * i as u32, *v);
        }
        assert_eq!(e.call(0x0045_78c0, &args![a, b]).u32(), a);
        assert_eq!(
            [e.mem.f32(a), e.mem.f32(a + 4), e.mem.f32(a + 8)],
            [4.0, -1.0, -1.5]
        );
    }

    /// `004579e0` as a square root of its `float` argument.
    fn install_sqrt(e: &mut Engine) {
        e.register_double(0x0045_79e0, |_, a| Ret {
            st0: (f32::from_bits(a[0]) as f64).sqrt(),
            ..Ret::default()
        });
    }

    #[test]
    fn the_vector_length_passes_the_squared_sum_to_the_square_root() {
        let mut e = Engine::new();
        install_sqrt(&mut e);
        let v = e.mem.alloc(12);
        for (i, x) in [1.0f32, 2.0, 2.0].iter().enumerate() {
            e.mem.set_f32(v + 4 * i as u32, *x);
        }
        let log = run_logged(&mut e, |e| {
            assert_eq!(e.call(0x0045_7990, &args![v]).f64(), 3.0);
        });
        assert_eq!(calls_to(&log, 0x0045_79e0), vec![vec![9.0f32.to_bits()]]);
    }

    #[test]
    fn unitizing_scales_to_length_one_and_returns_the_old_length() {
        let mut e = Engine::new();
        install_sqrt(&mut e);
        e.map(0x0101_7cf8, 8);
        e.set_global(0x0101_7cf8, 1e-6f64);
        let v = e.mem.alloc(12);
        for (i, x) in [3.0f32, 0.0, 4.0].iter().enumerate() {
            e.mem.set_f32(v + 4 * i as u32, *x);
        }
        let length = e.call(0x0045_7910, &args![v]).f32();
        assert_eq!(length, 5.0);
        assert_eq!(e.mem.f32(v), 3.0 * 0.2f32);
        assert_eq!(e.mem.f32(v + 4), 0.0);
        assert_eq!(e.mem.f32(v + 8), 4.0 * 0.2f32);
    }

    #[test]
    fn unitizing_a_tiny_vector_zeroes_it() {
        let mut e = Engine::new();
        install_sqrt(&mut e);
        e.map(0x0101_7cf8, 8);
        e.set_global(0x0101_7cf8, 1e-6f64);
        let v = e.mem.alloc(12);
        e.mem.set_f32(v, 1e-8);
        e.mem.set_f32(v + 4, -1e-8);
        e.mem.set_f32(v + 8, 0.0);
        let length = e.call(0x0045_7910, &args![v]).f32();
        assert_eq!(length, 0.0);
        assert_eq!([e.mem.f32(v), e.mem.f32(v + 4), e.mem.f32(v + 8)], [0.0; 3]);
    }

    // ---- TES::TestAllCells ----

    /// The `(dx, flags)` of every `SendInput` call.
    type SentInputs = Rc<RefCell<Vec<(i32, u32)>>>;
    /// A call log (`Engine::call_log`).
    type CallLog = Vec<(u32, Vec<u32>)>;
    /// The words of a captured position request.
    type RequestWords = Rc<RefCell<Vec<u32>>>;

    /// The name function of the test forms (vtable slot 0x130): the text
    /// pointer stored at +0x80 of the form.
    const TAC_NAME_FUNCTION: u32 = 0x0300_0130;

    /// A form whose name (slot 0x130) is `name`.
    fn named_form(e: &mut Engine, name: &str) -> u32 {
        let form = object_with_slots(e, 0x0300_4000, &[(0x130, TAC_NAME_FUNCTION)]);
        let text = e.mem.alloc(name.len() as u32 + 1);
        e.mem.set_cstr(text, name.as_bytes());
        e.mem.set_u32(form + 0x80, text);
        form
    }

    /// A list node `[item, next]`.
    fn list_node(e: &mut Engine, item: u32, next: u32) -> u32 {
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, item);
        e.mem.set_u32(node + 4, next);
        node
    }

    /// The globals and callees every `TestAllCells` test needs; returns the
    /// inputs `SendInput` was given as `(dx, flags)`.
    fn tac_common(e: &mut Engine) -> SentInputs {
        for (addr, len) in [
            (TEST_BASE_OBJECTS_CHECKED, 1),
            (TEST_MODE, 4),
            (TEST_WORLD_SPACE_NODE, 4),
            (TEST_INTERIOR_COUNT, 4),
            (TEST_ENTER_WORLD_SPACE, 1),
            (TEST_INTERIOR_INDEX, 4),
            (TEST_CELL_X, 4),
            (TEST_CELL_Y, 4),
            (TEST_EXTERIOR_COUNT, 4),
            (TEST_INPUT_SIGN, 4),
            (TEST_LOGGING, 1),
            (TEST_OUTPUT_FILE, 4),
            (TEST_FLOAT_TEN, 4),
            (TEST_DOUBLE_THOUSAND, 8),
            (TEST_DOUBLE_HALF_CELL, 8),
            (TEST_DOUBLE_ZERO, 8),
            (TEST_PREFIX, 8),
            (NI_POINT3_ZERO, 12),
            (DATA_HANDLER, 4),
            (PLAYER_CHARACTER, 4),
            (SAVE_LOAD_GAME, 4),
            (TES_SINGLETON, 4),
            (IO_MANAGER, 4),
        ] {
            e.map(addr, len);
        }
        for addr in [
            TEST_BASE_OBJECTS_CHECKED,
            TEST_MODE,
            TEST_WORLD_SPACE_NODE,
            TEST_INTERIOR_COUNT,
            TEST_ENTER_WORLD_SPACE,
            TEST_INTERIOR_INDEX,
            TEST_CELL_X,
            TEST_CELL_Y,
            TEST_EXTERIOR_COUNT,
            TEST_LOGGING,
        ] {
            e.set_global(addr, 0u32);
        }
        e.set_global(TEST_INPUT_SIGN, 1i32);
        let file_name = e.mem.alloc(32);
        e.mem.set_cstr(file_name, b"TestAllCells.xls");
        e.set_global(TEST_OUTPUT_FILE, file_name);
        e.mem.set_cstr(TEST_PREFIX, b"test");
        e.set_global(TEST_FLOAT_TEN, 10.0f32);
        e.set_global(TEST_DOUBLE_THOUSAND, 1000.0f64);
        e.set_global(TEST_DOUBLE_HALF_CELL, 2048.0f64);
        e.set_global(TEST_DOUBLE_ZERO, 0.0f64);
        e.register_double(TAC_NAME_FUNCTION, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x80),
            ..Ret::default()
        });
        e.register_double(STRNICMP, |e, a| {
            let lower = |s: Vec<u8>| -> Vec<u8> {
                s.iter()
                    .take(a[2] as usize)
                    .map(u8::to_ascii_lowercase)
                    .collect()
            };
            let first = lower(e.mem.cstr(a[0]));
            let second = lower(e.mem.cstr(a[1]));
            Ret {
                eax: (first != second) as u32,
                ..Ret::default()
            }
        });
        e.register_double(LIST_NODE_ITEM, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register_double(LIST_NODE_NEXT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register_double(SET_TEST_CALLBACK, |e, a| {
            e.mem.set_u32(a[0] + 0x54, a[1]);
            Ret::default()
        });
        e.register_double(SET_WORLD_SPACE, |e, a| {
            e.mem.set_u32(a[0] + 0x88, a[1]);
            Ret::default()
        });
        install_ftol(e);
        let clock = Rc::new(RefCell::new(0u32));
        e.register_double(CRT_CLOCK, move |_, _| {
            *clock.borrow_mut() += 1500;
            Ret {
                eax: *clock.borrow(),
                ..Ret::default()
            }
        });
        let inputs = Rc::new(RefCell::new(Vec::new()));
        let record = inputs.clone();
        e.register_double(IMPORT_SEND_INPUT, move |e, a| {
            assert_eq!((a[0], a[2]), (1, 0x1c));
            let dx = e.mem.i32(a[1] + 4);
            let flags = e.mem.u32(a[1] + 0x10);
            assert_eq!(e.mem.u32(a[1]), 0, "mouse input");
            record.borrow_mut().push((dx, flags));
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        noop(
            e,
            &[
                MESSAGE_HANDLER_INC_DISABLE_WARNING_COUNT,
                DELETE_FILE,
                DEBUG_PRINT,
                MESSAGE_HANDLER_OUTPUT,
                DATA_HANDLER_CHECK_MODELS,
                DATA_HANDLER_CHECK_ICONS,
                DATA_HANDLER_PREPARE_INTERIORS,
            ],
        );
        returns(e, DATA_HANDLER_INTERIOR_COUNT, 0);
        returns(e, DATA_HANDLER_FIRST_WORLD_SPACE, 0);
        returns(e, DATA_HANDLER_GET_INTERIOR, 0);
        returns(e, CELL_HAS_FLAG_20, 0);
        returns(e, CELL_GET_FIRST_REFR, 1);
        inputs
    }

    /// A `TestAllCells` engine without the cell-visiting machinery: settings,
    /// a `TES` with buffers (1 x 1 grid, 3 exterior slots), everything the
    /// loop uses stubbed.
    fn tac_engine() -> (Engine, Ptr<TES>, u32, SentInputs) {
        let (mut e, tes, _, exterior) = buffers_engine(1, &[0; 3], &[0; 3]);
        let inputs = tac_common(&mut e);
        // `buffers_engine` set the data handler pointer before the mapping
        // of `tac_common` zeroed it.
        e.set_global(DATA_HANDLER, 0x6000u32);
        (e, tes, exterior, inputs)
    }

    #[test]
    fn mode_minus_one_stops_a_running_test() {
        let (mut e, tes, _, _) = tac_engine();
        e.set(tes, TES::bRunningCellTests, true);
        e.set(tes, TES::pTACRegionFilter, Ptr::new(0x8800));
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_56d0, &args![tes, -1i32]);
        });
        assert!(!e.get(tes, TES::bRunningCellTests));
        assert!(e.get(tes, TES::pTACRegionFilter).is_null());
        assert_eq!(
            addresses(&log),
            vec![0x0045_56d0, MESSAGE_HANDLER_INC_DISABLE_WARNING_COUNT]
        );
        assert_eq!(
            calls_to(&log, MESSAGE_HANDLER_INC_DISABLE_WARNING_COUNT),
            vec![vec![0]]
        );
    }

    #[test]
    fn mode_zero_only_runs_the_loop() {
        let (mut e, tes, _, inputs) = tac_engine();
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_56d0, &args![tes, 0i32]);
        });
        // Not running: nothing happens.
        assert_eq!(addresses(&log), vec![0x0045_56d0]);
        assert!(inputs.borrow().is_empty());
    }

    #[test]
    fn modes_6_and_7_store_the_callback_and_run_as_modes_1_and_2() {
        let (mut e, tes, _, _) = tac_engine();
        e.call(0x0045_56d0, &args![tes, 6i32]);
        assert_eq!(e.get(tes, TES::pfnTACCallbackFunc).addr(), TEST_CALLBACK);
        assert_eq!(e.global::<u32>(TEST_MODE), 1);
        // The base objects count as checked.
        assert_eq!(e.global::<u8>(TEST_BASE_OBJECTS_CHECKED), 1);
        e.call(0x0045_56d0, &args![tes, 7i32]);
        assert_eq!(e.global::<u32>(TEST_MODE), 2);
    }

    #[test]
    fn a_fresh_run_checks_the_base_objects_and_walks_the_interiors() {
        let (mut e, tes, _, inputs) = tac_engine();
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_56d0, &args![tes, 2i32]);
        });
        let relevant: Vec<u32> = addresses(&log)
            .into_iter()
            .filter(|a| {
                [
                    MESSAGE_HANDLER_INC_DISABLE_WARNING_COUNT,
                    DELETE_FILE,
                    DEBUG_PRINT,
                    DATA_HANDLER_CHECK_MODELS,
                    DATA_HANDLER_CHECK_ICONS,
                    DATA_HANDLER_PREPARE_INTERIORS,
                    DATA_HANDLER_INTERIOR_COUNT,
                    DATA_HANDLER_GET_INTERIOR,
                ]
                .contains(a)
            })
            .collect();
        assert_eq!(
            relevant,
            vec![
                MESSAGE_HANDLER_INC_DISABLE_WARNING_COUNT,
                DELETE_FILE,
                DEBUG_PRINT,
                DATA_HANDLER_CHECK_MODELS,
                DEBUG_PRINT,
                DATA_HANDLER_CHECK_ICONS,
                DATA_HANDLER_PREPARE_INTERIORS,
                DATA_HANDLER_INTERIOR_COUNT,
                DEBUG_PRINT,
                DATA_HANDLER_GET_INTERIOR,
                MESSAGE_HANDLER_INC_DISABLE_WARNING_COUNT,
            ]
        );
        let prints = calls_to(&log, DEBUG_PRINT);
        assert_eq!(
            prints,
            vec![
                vec![TEST_TEXT_BASE_OBJECT_MODELS],
                vec![TEST_TEXT_BASE_OBJECT_ICONS],
                vec![TEST_TEXT_INTERIORS],
            ]
        );
        assert_eq!(
            calls_to(&log, DATA_HANDLER_CHECK_MODELS),
            vec![vec![0x6000, 0]]
        );
        assert_eq!(calls_to(&log, DELETE_FILE).len(), 1);
        // One mouse move was sent; the run ended since no interiors exist.
        assert_eq!(*inputs.borrow(), vec![(2, 1)]);
        assert!(!e.get(tes, TES::bRunningCellTests));
        // The second run sends the opposite move and does not check again.
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_56d0, &args![tes, 2i32]);
        });
        assert_eq!(*inputs.borrow(), vec![(2, 1), (-1, 1)]);
        assert_eq!(count_calls(&log, DATA_HANDLER_CHECK_MODELS), 0);
    }

    /// A world space list `[Test world -> world]` where the world space
    /// spans cells (-1, -1) to (0, 0); `load_cell` answers `cell` for (0, 0).
    fn world_space_list(e: &mut Engine, cell: u32) -> (u32, u32) {
        let skipped = named_form(e, "TestWorld");
        let world = named_form(e, "Tamriel");
        let second = list_node(e, world, 0);
        let first = list_node(e, skipped, second);
        returns(e, DATA_HANDLER_FIRST_WORLD_SPACE, first);
        returns_float(e, WORLD_SPACE_MIN_X, -4096.0);
        returns_float(e, WORLD_SPACE_MIN_Y, -4096.0);
        returns_float(e, WORLD_SPACE_MAX_X, 0.0);
        returns_float(e, WORLD_SPACE_MAX_Y, 0.0);
        e.register_double(WORLD_SPACE_LOAD_CELL, move |_, a| Ret {
            eax: if (a[1], a[2]) == (0, 0) { cell } else { 0 },
            ..Ret::default()
        });
        (world, second)
    }

    #[test]
    fn mode_1_walks_the_cells_of_the_first_real_world_space() {
        let (mut e, tes, exterior, _) = tac_engine();
        let cell = cell_object(&mut e, 0);
        // The first reference check says "no references": the cell is
        // buffered but not visited.
        let (world, _) = world_space_list(&mut e, cell);
        returns(&mut e, CELL_GET_FIRST_REFR, 0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_56d0, &args![tes, 1i32]);
        });
        // The "test" world space was skipped; the next one entered.
        assert_eq!(
            calls_to(&log, SET_WORLD_SPACE),
            vec![vec![tes.addr(), world]]
        );
        let name = e.mem.u32(world + 0x80);
        let minus_one = (-1i32) as u32;
        assert!(calls_to(&log, DEBUG_PRINT).contains(&vec![
            TEST_TEXT_WORLD_SPACE,
            name,
            minus_one,
            minus_one,
            0,
            0
        ]));
        let loads = calls_to(&log, WORLD_SPACE_LOAD_CELL);
        let coords: Vec<(u32, u32)> = loads.iter().map(|l| (l[1], l[2])).collect();
        assert_eq!(
            coords,
            vec![
                (minus_one, minus_one),
                (minus_one, 0),
                (0, minus_one),
                (0, 0)
            ]
        );
        assert!(loads.iter().all(|l| l[0] == world));
        // The loaded cell went into the exterior buffer.
        assert_eq!(e.mem.u32(exterior), cell);
        // The list is used up: the interiors follow.
        assert_eq!(e.global::<u32>(TEST_WORLD_SPACE_NODE), 0);
        assert!(calls_to(&log, DEBUG_PRINT).contains(&vec![TEST_TEXT_INTERIORS]));
        assert_eq!(count_calls(&log, TIME_OBJECT_ADVANCE), 0);
    }

    #[test]
    fn mode_3_finds_the_loaded_world_space_in_the_list() {
        let (mut e, tes, _, _) = tac_engine();
        let first = named_form(&mut e, "Other");
        let second = named_form(&mut e, "Loaded");
        let second_node = list_node(&mut e, second, 0);
        let first_node = list_node(&mut e, first, second_node);
        returns(&mut e, DATA_HANDLER_FIRST_WORLD_SPACE, first_node);
        returns_float(&mut e, WORLD_SPACE_MIN_X, 0.0);
        returns_float(&mut e, WORLD_SPACE_MIN_Y, 0.0);
        returns_float(&mut e, WORLD_SPACE_MAX_X, 0.0);
        returns_float(&mut e, WORLD_SPACE_MAX_Y, 0.0);
        returns(&mut e, WORLD_SPACE_LOAD_CELL, 0);
        e.set(tes, TES::pWorldSpace, Ptr::new(second));
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_56d0, &args![tes, 3i32]);
        });
        assert_eq!(
            calls_to(&log, SET_WORLD_SPACE),
            vec![vec![tes.addr(), second]]
        );
        assert_eq!(
            calls_to(&log, WORLD_SPACE_LOAD_CELL),
            vec![vec![second, 0, 0]]
        );
    }

    #[test]
    fn mode_3_without_the_loaded_world_space_in_the_list_does_nothing() {
        let (mut e, tes, _, inputs) = tac_engine();
        let other = named_form(&mut e, "Other");
        let node = list_node(&mut e, other, 0);
        returns(&mut e, DATA_HANDLER_FIRST_WORLD_SPACE, node);
        e.set(tes, TES::pWorldSpace, Ptr::new(0x1234_5678));
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_56d0, &args![tes, 3i32]);
        });
        assert_eq!(count_calls(&log, SET_WORLD_SPACE), 0);
        assert_eq!(count_calls(&log, DATA_HANDLER_PREPARE_INTERIORS), 0);
        assert_eq!(inputs.borrow().len(), 1);
    }

    /// An interior test: the interiors are `names`, every cell has
    /// references, and the region filter is set; `contains` is what the
    /// region test answers (`regions == 0` means no region list at all).
    fn interior_filter_run(names: &[&str], regions: u32) -> (Engine, CallLog, Vec<u32>) {
        let (mut e, tes, _, _) = tac_engine();
        let cells: Vec<u32> = names.iter().map(|n| named_form(&mut e, n)).collect();
        let table = cells.clone();
        returns(&mut e, DATA_HANDLER_INTERIOR_COUNT, names.len() as u32);
        e.register_double(DATA_HANDLER_GET_INTERIOR, move |_, a| Ret {
            eax: table.get(a[1] as usize).copied().unwrap_or(0),
            ..Ret::default()
        });
        returns(&mut e, CELL_GET_REGION_LIST, regions);
        returns(&mut e, REGION_LIST_CONTAINS, 0);
        e.set(tes, TES::pTACRegionFilter, Ptr::new(0x8800));
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_56d0, &args![tes, 2i32]);
        });
        (e, log, cells)
    }

    #[test]
    fn test_named_interiors_are_skipped_and_the_region_filter_applies() {
        let (e, log, cells) = interior_filter_run(&["testRoom", "Vault"], 0x9900);
        // Both interiors were asked for; only "Vault" was looked at further.
        let asked = calls_to(&log, DATA_HANDLER_GET_INTERIOR);
        assert_eq!(asked, vec![vec![0x6000, 0], vec![0x6000, 1]]);
        assert_eq!(
            calls_to(&log, CELL_GET_REGION_LIST),
            vec![vec![cells[1], 1]]
        );
        // The filter (at TES + 0x5c) did not accept: the cell was not visited.
        assert_eq!(
            calls_to(&log, REGION_LIST_CONTAINS)
                .iter()
                .map(|c| c[0])
                .collect::<Vec<_>>(),
            vec![0x9904]
        );
        assert_eq!(count_calls(&log, TIME_OBJECT_ADVANCE), 0);
        // The run ended: the filter was cleared.
        let _ = e;
    }

    #[test]
    fn an_interior_without_a_region_list_fails_the_region_filter() {
        let (_, log, _) = interior_filter_run(&["Vault"], 0);
        assert_eq!(count_calls(&log, REGION_LIST_CONTAINS), 0);
        assert_eq!(count_calls(&log, TIME_OBJECT_ADVANCE), 0);
    }

    // ---- visiting a cell ----

    /// Runs `TestAllCells(mode)` over one interior cell, which is also the
    /// loaded interior cell, with the visiting machinery stubbed.
    /// Returns the engine, the `TES`, the cell, the log, and the words of the
    /// position request the player was given.
    fn visit_run(mode: i32, logging: bool) -> (Engine, Ptr<TES>, u32, CallLog, RequestWords) {
        visit_run_with(mode, logging, |_, _| {})
    }

    /// [`visit_run`], with `before` run on the engine and the `TES` just before
    /// the call.
    fn visit_run_with(
        mode: i32,
        logging: bool,
        before: impl FnOnce(&mut Engine, Ptr<TES>),
    ) -> (Engine, Ptr<TES>, u32, CallLog, RequestWords) {
        let (mut e, tes, _) = switch_engine();
        let _ = tac_common(&mut e);
        let cell = named_form(&mut e, "Vault");
        returns(&mut e, GET_INTERIOR_CELL, cell);
        returns(&mut e, CELL_IS_INTERIOR, 1);
        returns(&mut e, DATA_HANDLER_INTERIOR_COUNT, 1);
        e.register_double(DATA_HANDLER_GET_INTERIOR, move |_, a| Ret {
            eax: if a[1] == 0 { cell } else { 0 },
            ..Ret::default()
        });
        e.set_global(TEST_LOGGING, logging as u8);
        noop(
            &mut e,
            &[
                TIME_OBJECT_ADVANCE,
                CELL_LOAD_ALL_TEMP_DATA,
                CELL_GET_COC_PLACEMENT_INFO,
                PLAYER_REQUEST_POSITION,
                PLAYER_HANDLE_POSITION_REQUEST,
                IO_MANAGER_LOAD_QUEUED_PRIORITY,
                TES_PURGE_UNUSED,
                FRAME_SCOPE_RELEASE,
                FRAME_SCOPE_END,
                STRING_ARRAY_CONSTRUCT,
                STRING_ARRAY_DESTRUCT,
                SCENE_INFO_TOTALS,
                SCENE_INFO_BUILD_M_NUMBER,
                SAVE_LOAD_GAME_TEST_ALL_CELLS,
            ],
        );
        e.register_double(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register_double(POSITION_REQUEST_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let rotation = e.mem.alloc(12);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(rotation + 4 * i as u32, *v);
        }
        returns(&mut e, PLAYER_ROTATION_GETTER, rotation);
        e.register_double(FRAME_SCOPE_BEGIN, |e, a| {
            e.mem.set_u8(a[0] + 5, 1);
            Ret::default()
        });
        if logging {
            install_print_callees(&mut e);
        }
        // Remember the position request block when the player is given it.
        let request = Rc::new(RefCell::new(Vec::new()));
        let seen = request.clone();
        e.register_double(PLAYER_REQUEST_POSITION, move |e, a| {
            *seen.borrow_mut() = (0..0x34 / 4).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            Ret::default()
        });
        // The cell has references, the region filter accepts.
        returns(&mut e, CELL_GET_REGION_LIST, 0x9900);
        returns(&mut e, REGION_LIST_CONTAINS, 1);
        e.set(tes, TES::pTACRegionFilter, Ptr::new(0x8800));
        let callback = Rc::new(RefCell::new(Vec::new()));
        let seen_callback = callback.clone();
        e.register_double(0x0300_0200, move |_, a| {
            seen_callback.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.set(tes, TES::pfnTACCallbackFunc, Ptr::new(0x0300_0200));
        e.set(tes, TES::pTACCallbackData, Ptr::new(0x1234));
        before(&mut e, tes);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_56d0, &args![tes, mode]);
        });
        assert_eq!(*callback.borrow(), vec![vec![cell, 0x1234]]);
        (e, tes, cell, log, request)
    }

    #[test]
    fn a_visited_interior_cell_is_entered_and_the_player_is_placed_in_it() {
        let (e, tes, cell, log, request) = visit_run(2, false);
        // The calendar-like object is advanced by 10.0.
        assert_eq!(
            calls_to(&log, TIME_OBJECT_ADVANCE),
            vec![vec![OBJECT_011DE7B8, 10.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&log, CELL_LOAD_ALL_TEMP_DATA), vec![vec![cell]]);
        let placement = calls_to(&log, CELL_GET_COC_PLACEMENT_INFO);
        assert_eq!(placement.len(), 1);
        assert_eq!(placement[0][0], cell);
        // The region filter test got the list + 4 and the address of the
        // filter field.
        assert_eq!(
            calls_to(&log, REGION_LIST_CONTAINS),
            vec![vec![0x9904, tes.addr() + 0x5c]]
        );
        // The request: position (zeros), the player's rotation, the flag,
        // and the interior cell (no world space).
        let words = request.borrow().clone();
        assert_eq!(words[0], 0);
        assert_eq!(words[1], cell);
        assert_eq!(words[2..5], [0, 0, 0]);
        assert_eq!(
            words[5..8],
            [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()]
        );
        assert_eq!(words[8] & 0xff, 1);
        // The order of the load steps.
        let order: Vec<u32> = addresses(&log)
            .into_iter()
            .filter(|a| {
                [
                    PLAYER_REQUEST_POSITION,
                    PLAYER_HANDLE_POSITION_REQUEST,
                    IO_MANAGER_LOAD_QUEUED_PRIORITY,
                    FRAME_SCOPE_BEGIN,
                    TES_PURGE_UNUSED,
                    FRAME_SCOPE_RELEASE,
                    FRAME_SCOPE_END,
                ]
                .contains(a)
            })
            .collect();
        assert_eq!(
            order,
            vec![
                PLAYER_REQUEST_POSITION,
                PLAYER_HANDLE_POSITION_REQUEST,
                IO_MANAGER_LOAD_QUEUED_PRIORITY,
                FRAME_SCOPE_BEGIN,
                TES_PURGE_UNUSED,
                FRAME_SCOPE_RELEASE,
                FRAME_SCOPE_END,
            ]
        );
        assert_eq!(calls_to(&log, TES_PURGE_UNUSED)[0][1..], [1, 0]);
        // The byte the frame scope set at +5 is released.
        assert_eq!(calls_to(&log, FRAME_SCOPE_RELEASE), vec![vec![1]]);
        // Logging was off, and mode 2 does not call the save/load test.
        assert_eq!(count_calls(&log, STRING_ARRAY_CONSTRUCT), 0);
        assert_eq!(count_calls(&log, SAVE_LOAD_GAME_TEST_ALL_CELLS), 0);
        // The test ended with the interior list.
        assert!(!e.get(tes, TES::bRunningCellTests));
    }

    #[test]
    fn modes_4_and_5_also_run_the_save_game_test_on_each_visited_cell() {
        let (mut e, _, _, log, _) = visit_run(5, false);
        let save_load: u32 = e.global(SAVE_LOAD_GAME);
        assert_eq!(
            calls_to(&log, SAVE_LOAD_GAME_TEST_ALL_CELLS),
            vec![vec![save_load, 5]]
        );
        let _ = &mut e;
    }

    #[test]
    fn a_visited_cell_prints_its_line_when_logging_is_on() {
        let (_, _, cell, log, _) = visit_run(2, true);
        assert_eq!(count_calls(&log, STRING_ARRAY_CONSTRUCT), 1);
        assert_eq!(count_calls(&log, STRING_ARRAY_DESTRUCT), 1);
        let interior_lines: Vec<_> = calls_to(&log, DEBUG_PRINT)
            .into_iter()
            .filter(|c| c[0] == TEST_TEXT_INTERIOR_LINE)
            .collect();
        assert_eq!(interior_lines.len(), 1);
        assert_eq!(interior_lines[0].len(), 11);
        let rows = calls_to(&log, MESSAGE_HANDLER_OUTPUT);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][1], TEST_TEXT_INTERIOR_ROW);
        let _ = cell;
    }

    // ---- the per-cell line ----

    /// `f64` as two argument words.
    fn double_words(value: f64) -> [u32; 2] {
        [value.to_bits() as u32, (value.to_bits() >> 32) as u32]
    }

    /// An engine for `test_print_cell`: the scene totals are 12.5 and 2.0
    /// (as float bits), the M# text is "M1", the cell holds an actor holder
    /// whose field is non-empty (portals), one texture of 5 MB, 4 lights, 7
    /// actors.
    fn print_engine() -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        let _ = tac_common(&mut e);
        let text = install_print_callees(&mut e);
        let cell = named_form(&mut e, "Vault");
        (e, cell, text, 0)
    }

    /// The callees of the logging block (see [`print_engine`]); returns the
    /// M# text pointer.
    fn install_print_callees(e: &mut Engine) -> u32 {
        e.register_double(NI_POINTER_GET, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"M1");
        e.register_double(SCENE_INFO_TOTALS, |e, a| {
            e.mem.set_f32(a[0], 12.5);
            e.mem.set_f32(a[1], 2.0);
            Ret::default()
        });
        e.register_double(SCENE_INFO_BUILD_M_NUMBER, move |e, a| {
            assert_eq!(a[0], 2.0f32.to_bits());
            e.mem.set_u32(a[1], text);
            Ret::default()
        });
        noop(e, &[STRING_ARRAY_CONSTRUCT, STRING_ARRAY_DESTRUCT]);
        returns(e, CELL_GET_ACTOR_HOLDER, 0x7100);
        returns(e, ACTOR_HOLDER_GET_FIELD_20, 0x7120);
        returns(e, FIELD_IS_EMPTY, 0);
        returns(e, TEXTURE_LIST_HEAD, 0x7200);
        e.register_double(TEXTURE_IS_LOADED, |_, a| Ret {
            eax: (a[0] == 0x7200) as u32,
            ..Ret::default()
        });
        returns(e, TEXTURE_LIST_NEXT, 0);
        e.register_double(DEBUG_PRINT, |_, a| Ret {
            eax: if a[0] == 0x7200 { 5 << 20 } else { 0 },
            ..Ret::default()
        });
        returns(e, ACTOR_COUNT, 7);
        returns(e, CELL_GET_LIGHT_COUNT, 4);
        text
    }

    #[test]
    fn an_interior_cell_line_has_its_lights_portals_textures_and_actors() {
        let (mut e, cell, text, _) = print_engine();
        returns(&mut e, CELL_IS_INTERIOR, 1);
        let name = e.mem.u32(cell + 0x80);
        let output_file: u32 = e.global(TEST_OUTPUT_FILE);
        let log = run_logged(&mut e, |e| test_print_cell(e, cell, 1.5));
        let [total_lo, total_hi] = double_words(12.5);
        let [time_lo, time_hi] = double_words(1.5);
        let expected_tail = vec![total_lo, total_hi, time_lo, time_hi, 4, TEST_TEXT_YES, 5, 7];
        let line: Vec<u32> = [
            vec![TEST_TEXT_INTERIOR_LINE, name, text],
            expected_tail.clone(),
        ]
        .concat();
        assert!(calls_to(&log, DEBUG_PRINT).contains(&line));
        let row: Vec<u32> = [
            vec![output_file, TEST_TEXT_INTERIOR_ROW, name, text],
            expected_tail,
        ]
        .concat();
        assert_eq!(calls_to(&log, MESSAGE_HANDLER_OUTPUT), vec![row]);
        // The texture total comes from the debug print of the texture.
        assert!(calls_to(&log, DEBUG_PRINT).contains(&vec![0x7200]));
    }

    #[test]
    fn an_exterior_cell_line_has_its_coordinates_and_counts_the_cell() {
        let (mut e, cell, text, _) = print_engine();
        returns(&mut e, CELL_IS_INTERIOR, 0);
        returns(&mut e, CELL_GET_DATA_X, 3);
        returns(&mut e, CELL_GET_DATA_Y, (-4i32) as u32);
        // No portals: the actor holder field is empty.
        returns(&mut e, FIELD_IS_EMPTY, 1);
        let name = e.mem.u32(cell + 0x80);
        let output_file: u32 = e.global(TEST_OUTPUT_FILE);
        let log = run_logged(&mut e, |e| test_print_cell(e, cell, 0.5));
        let [total_lo, total_hi] = double_words(12.5);
        let [time_lo, time_hi] = double_words(0.5);
        let tail = vec![
            text,
            total_lo,
            total_hi,
            time_lo,
            time_hi,
            TEST_TEXT_NO,
            5,
            7,
        ];
        let coordinates = vec![name, 3, (-4i32) as u32];
        let line: Vec<u32> = [
            vec![TEST_TEXT_EXTERIOR_LINE],
            coordinates.clone(),
            tail.clone(),
        ]
        .concat();
        assert!(calls_to(&log, DEBUG_PRINT).contains(&line));
        let row: Vec<u32> = [vec![output_file, TEST_TEXT_EXTERIOR_ROW], coordinates, tail].concat();
        assert_eq!(calls_to(&log, MESSAGE_HANDLER_OUTPUT), vec![row]);
        assert_eq!(e.global::<u32>(TEST_EXTERIOR_COUNT), 1);
    }
    // END SESSION 4 TESTS

    // ---- the frame's model (docs/FRAME_SKELETON.md, PR 5) ----

    /// The callees of a log as `world::frame::world_time` names them: the
    /// import slot and the callback double by what they are, the rest
    /// direct.
    fn model_callees(log: &CallLog) -> Vec<world::frame::world_time::Callee> {
        use world::frame::world_time::Callee;
        log.iter()
            .map(|(a, _)| match *a {
                IMPORT_SEND_INPUT => Callee::Import(IMPORT_SEND_INPUT),
                0x0300_0200 => Callee::Pointer(0x54),
                0x0300_0010 => Callee::Virtual(0x10),
                a => Callee::Direct(a),
            })
            .collect()
    }

    /// `TES::RunAnimations` (`00455640`) follows the model, indoors and out.
    #[test]
    fn run_animations_follows_the_frame_model() {
        use world::frame::world_time::{follows, function, Callee, WorldState};
        let model = function(0x0045_5640).expect("modelled");
        for interior in [false, true] {
            let (mut e, tes, _, _) = buffers_engine(1, &[0; 3], &[0; 3]);
            e.map(COUNTER_011C56E8, 4);
            e.set(tes, TES::pGridCellA, Ptr::new(0x6400));
            noop(&mut e, &[CELL_PASS_A, GRID_CELL_ARRAY_PASS_A]);
            if interior {
                returns(&mut e, GET_INTERIOR_CELL, 0x6200);
            }
            let log = run_logged(&mut e, |e| {
                e.call(0x0045_5640, &args![tes]);
            });
            let s = WorldState {
                interior_loaded: interior,
                ..WorldState::default()
            };
            // `00455680` is called as a Rust function (its count's clear).
            follows(
                model,
                &s,
                &model_callees(&log),
                &[Callee::Direct(0x0045_5680)],
            )
            .unwrap_or_else(|why| panic!("interior {interior}: {why}"));
        }
    }

    /// `TES::UpdateCurrentGridCell` (`00452580`) follows the model: the
    /// first load, inside the centre cell, and across a border outdoors and
    /// with an interior loaded.
    #[test]
    fn update_current_grid_cell_follows_the_frame_model() {
        use world::frame::world_time::{follows, function, Callee, GridMove, WorldState};
        let model = function(0x0045_2580).expect("modelled");
        // Called as Rust functions, so not in the log: the texture clean-up,
        // the first load, the loading flag and the grid array's load.
        let ignore = vec![
            Callee::Direct(0x0045_2490),
            Callee::Direct(0x0045_15a0),
            Callee::Direct(0x0045_2e40),
            Callee::Direct(0x0045_2ff0),
        ];
        let inside = WorldState::default();
        let (mut e, tes, _, _) = update_engine(5);
        let pos = position(&mut e, 10.0 * 4096.0 + 2048.0, 20.0 * 4096.0 + 2048.0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_2580, &args![tes, pos, 0u8]);
        });
        follows(model, &inside, &model_callees(&log), &ignore).expect("inside");
        // Into the next column: the cells coming into range are queued.
        let (mut e, tes, _, _) = update_engine(5);
        let pos = position(&mut e, 11.0 * 4096.0 + 100.0, 20.0 * 4096.0 + 2048.0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_2580, &args![tes, pos, 0u8]);
        });
        follows(model, &inside, &model_callees(&log), &ignore).expect("queueing");
        assert!(model_callees(&log).contains(&Callee::Direct(EXTERIOR_CELL_LOADER_QUEUE_CELL_LOAD)));

        let first = WorldState {
            grid: GridMove::First,
            ..inside
        };
        let (mut e, tes, _, _) = update_engine(5);
        e.set(tes, TES::iCurrentGridX, 0x7fff_ffff);
        e.set_global(GRID_LOAD_IN_PROGRESS, 1u8);
        let pos = position(&mut e, 5.0 * 4096.0 + 10.0, 7.0 * 4096.0 + 10.0);
        let log = run_logged(&mut e, |e| {
            e.call(0x0045_2580, &args![tes, pos, 0u8]);
        });
        // The first load (`004515a0`, a Rust function here) calls the water
        // listener's update itself.
        let mut first_ignore = ignore.clone();
        first_ignore.push(Callee::Direct(0x0062_f460));
        follows(model, &first, &model_callees(&log), &first_ignore).expect("first");

        let crossed = WorldState {
            grid: GridMove::Crossed,
            ..inside
        };
        let (mut e, tes, _, _) = update_engine(5);
        install_player(&mut e, 0);
        e.set(tes, TES::pWorldSpace, Ptr::new(0));
        returns(&mut e, 0x0046_0140, 0x7100);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x6abc);
        returns(&mut e, 0x0068_15c0, slot);
        let log = grid_move(&mut e, tes, 12, 0);
        follows(model, &crossed, &model_callees(&log), &ignore).expect("crossed");
        assert!(model_callees(&log).contains(&Callee::Virtual(0x10)));

        let indoors = WorldState {
            interior_loaded: true,
            ..crossed
        };
        let (mut e, tes, _, _) = update_engine(5);
        install_player(&mut e, 0);
        returns(&mut e, GET_INTERIOR_CELL, 0x6200);
        let log = grid_move(&mut e, tes, 12, 0);
        follows(model, &indoors, &model_callees(&log), &ignore).expect("indoors");
        assert!(!model_callees(&log).contains(&Callee::Virtual(0x10)));
    }

    /// `TES::TestAllCells(0)` (`004556d0`) as the frame calls it, while a test
    /// runs over one interior: a turn that visits it and ends the run follows
    /// the model, with and without the logging.
    #[test]
    fn test_all_cells_turn_follows_the_frame_model() {
        use world::frame::world_time::{follows, function, Callee, WorldState};
        let model = function(0x0045_56d0).expect("modelled");
        for logging in [false, true] {
            let (_, _, _, log, _) = visit_run_with(0, logging, |e, tes| {
                e.set(tes, TES::bRunningCellTests, true);
                e.set_global(TEST_MODE, 2u32);
                e.set_global(TEST_INTERIOR_COUNT, 1u32);
                e.set_global(TEST_INTERIOR_INDEX, 0u32);
            });
            let s = WorldState {
                interior_loaded: true,
                test_cell_found: true,
                test_cell_interior: true,
                test_logging: logging,
                test_callback: true,
                test_done: true,
                ..WorldState::default()
            };
            // `00453dc0` (placing the player in the interior) is called as a
            // Rust function.
            follows(
                model,
                &s,
                &model_callees(&log),
                &[Callee::Direct(0x0045_3dc0)],
            )
            .unwrap_or_else(|why| panic!("logging {logging}: {why}"));
        }
    }

    // ---- the AI stage's model (docs/FRAME_SKELETON.md, PR 6) ----

    /// `TES::UpdateCellAnimations` (`00453550`) follows the model: one thread
    /// and more, indoors and out, in dialogue.
    #[test]
    fn update_cell_animations_follows_the_frame_model() {
        use world::frame::ai_stage::{follows, function, AiState, Callee};
        let model = function(0x0045_3550).expect("modelled");
        for threads in [1, 2, 4] {
            for interior in [false, true] {
                for dialog in [false, true] {
                    let (mut e, tes) = frame_engine(threads);
                    if interior {
                        returns(&mut e, GET_INTERIOR_CELL, 0x6200);
                    }
                    if dialog {
                        returns(&mut e, 0x0070_50d0, 1);
                    }
                    let log = run_logged(&mut e, |e| {
                        e.call(0x0045_3550, &args![tes, 0.5f32]);
                    });
                    let s = AiState {
                        threads,
                        interior_loaded: interior,
                        in_dialog: dialog,
                        ..AiState::default()
                    };
                    // `004537c0` is called as a Rust function (its own calls
                    // are in the log).
                    follows(
                        model,
                        &s,
                        &model_callees(&log),
                        &[Callee::Direct(0x0045_37c0)],
                    )
                    .unwrap_or_else(|why| panic!("{s:?}: {why}"));
                }
            }
        }
    }

    /// `TES::UpdateCellMainThread` (`004537c0`) follows the model.
    #[test]
    fn update_cell_main_thread_follows_the_frame_model() {
        use world::frame::ai_stage::{follows, function, AiState};
        let model = function(0x0045_37c0).expect("modelled");
        for dialog in [false, true] {
            let (mut e, tes) = frame_engine(2);
            returns(&mut e, 0x0096_11e0, 0x9200);
            if dialog {
                returns(&mut e, 0x0070_50d0, 1);
            }
            let log = run_logged(&mut e, |e| {
                e.call(0x0045_37c0, &args![tes]);
            });
            let s = AiState {
                in_dialog: dialog,
                ..AiState::default()
            };
            follows(model, &s, &model_callees(&log), &[])
                .unwrap_or_else(|why| panic!("{s:?}: {why}"));
        }
    }
}
