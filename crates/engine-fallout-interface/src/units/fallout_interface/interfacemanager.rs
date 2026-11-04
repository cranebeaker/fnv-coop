//! `fallout/interface/interfacemanager.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit is the interface manager (`InterfaceManager`, Xbox PDB): the
//! one object (the word at `011d8a80`) that owns the menu scene graphs, the
//! cursor, the menu stack and the per-frame interface update. This file is
//! the unit's main file: it declares the layouts and helpers the part files
//! (`interfacemanager_p2.rs`, ...) share, and holds the functions from
//! `00709fd0` up to `00717230`.
//!
//! Offsets of the manager object on PC match the Xbox PDB up to the effect
//! managers (`+0x178 HUDEffects`, `+0x1dc VATSEffect`); after those the PC
//! build differs (the fields from `+0x4ac` on have no PDB name here).
//!
//! The compiler's exception-unwinding frames are not translated. Every
//! function that opens the scope guard (`00404eb0` before, `00404ee0`
//! after; neither has a PDB name, the guard is a 4-byte local that is given
//! the source line) does so through [`with_scope_guard`].

#[allow(unused_imports)]
use crate::prelude::*;

/// The interface manager singleton (a pointer to the object).
pub(crate) const MANAGER_SINGLETON: u32 = 0x011d_8a80;
/// `bHas360Controller` (Xbox PDB, static): set by `Initialize` from
/// `XInputGetState`.
pub(crate) const HAS_360_CONTROLLER: u32 = 0x011d_8a84;
/// `fCurrAlphaDPS`, `bAlphaGoingDown`, `iLastDPSFadeTime` (Xbox PDB, statics
/// of the manager, matched by type and the constructor's initial values).
pub(crate) const CURR_ALPHA_DPS: u32 = 0x011d_8a68;
pub(crate) const ALPHA_GOING_DOWN: u32 = 0x011d_8a50;
pub(crate) const LAST_DPS_FADE_TIME: u32 = 0x011d_8a64;
/// The `"...\InterfaceManager.cpp"` path the scope guard is given.
pub(crate) const SOURCE_FILE: u32 = 0x0106_ef50;
/// `"MenuRoot"` and `"Menu3DRoot"`.
pub(crate) const MENU_ROOT_NAME: u32 = 0x0106_ef34;
pub(crate) const MENU_3D_ROOT_NAME: u32 = 0x0106_ef40;
/// `"Cursor"`.
const CURSOR_NAME: u32 = 0x0106_f010;
/// `"Data\Menus\globals.xml"`.
const GLOBALS_XML_PATH: u32 = 0x0106_eff8;
/// `"_background_fill_alpha"` and `"_Has360Controller"`.
const BACKGROUND_FILL_ALPHA_NAME: u32 = 0x0106_efe0;
const HAS_360_CONTROLLER_NAME: u32 = 0x0106_efcc;
/// `"InterfaceManager: DebugText Root"`.
const DEBUG_TEXT_ROOT_NAME: u32 = 0x0106_efa8;
/// `255.0` (`double`).
const TWO_FIFTY_FIVE: u32 = 0x0101_e568;
/// `2.0` (`double`).
const TWO: u32 = 0x0101_1590;
/// The 16:10 aspect ratio, `1.6` (`double`), and `0.6` (`double`), which
/// `1 / ratio` is compared with (5:3 is `1.666..`, so this is the 10:6 test
/// for the inverse).
const WIDESCREEN_RATIO: u32 = 0x0106_efa0;
const WIDESCREEN_INVERSE_RATIO: u32 = 0x0102_90b8;
/// The angle (`float`) the one-to-one distance is derived from.
const ONE_TO_ONE_ANGLE: u32 = 0x0106_ef4c;
/// The `float` the cursor node's look direction takes as its second
/// component (`-1.0`).
const CURSOR_DIRECTION_Y: u32 = 0x0101_2054;
/// Default `fEnd` of a [`Timer`] (`float`).
const TIMER_DEFAULT_END: u32 = 0x0101_7d00;

/// `XInputGetState(user, state)` (`stdcall`); 0 means a controller answered.
const XINPUT_GET_STATE: u32 = 0x009f_996e;
/// `operator new(size)` and `operator delete(block)`.
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `NiAlloc(size)` (the Gamebryo allocator, `cdecl`).
const NI_ALLOC: u32 = 0x00aa_13e0;
/// Scope guard constructor `(guard, 0xd, 1, source file, line)` and
/// destructor.
const SCOPE_GUARD_BEGIN: u32 = 0x0040_4eb0;
const SCOPE_GUARD_END: u32 = 0x0040_4ee0;
/// `NiPointer<T>` members, called on the embedded pointer: constructor from
/// a pointer (adds a reference), assignment (releases the old pointer),
/// `get` and destructor.
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const NI_POINTER_GET: u32 = 0x0055_9450;
const NI_POINTER_DESTROY: u32 = 0x0045_cec0;
/// An empty member constructor (the body only returns `this`).
const MEMBER_CONSTRUCTOR_EMPTY: u32 = 0x0068_15c0;
/// Constructor / destructor of an embedded `BSSimpleList`.
const SIMPLE_LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const SIMPLE_LIST_DESTROY: u32 = 0x0046_ffb0;
/// `NiPoint3::NiPoint3(x, y, z)`: stores the three floats, returns `this`.
const NI_POINT3_CONSTRUCT: u32 = 0x0041_6870;
/// Constructor of the struct the node update takes: `(this, float, byte,
/// byte)` stores the float and the two bytes and zeroes the rest (the
/// Gamebryo update data).
const NI_UPDATE_DATA_CONSTRUCT: u32 = 0x0043_d410;
/// Updates a node from that struct (`(node, &data)`).
const NODE_UPDATE: u32 = 0x00a5_9c60;
/// `NiAVObject::UpdateProperties` (Xbox PDB).
const NODE_UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// `(node, bool)`: forwards to `0043b370(node, flag, 1)`; the cursor and
/// debug text roots are given 0 and 1 here.
const NODE_SET_FLAG: u32 = 0x0045_0f90;
/// `FOHUDEffectManager` constructor (Xbox PDB) and destructor.
const HUD_EFFECT_MANAGER_CONSTRUCT: u32 = 0x007f_7610;
const HUD_EFFECT_MANAGER_DESTROY: u32 = 0x007f_7790;
/// `FOVATSEffectManager` constructor (Xbox PDB) and destructor.
const VATS_EFFECT_MANAGER_CONSTRUCT: u32 = 0x007f_fec0;
const VATS_EFFECT_MANAGER_DESTROY: u32 = 0x0080_01b0;
/// `FOPipboyManager` constructor.
const PIPBOY_MANAGER_CONSTRUCT: u32 = 0x007f_7a70;
/// Tile trait setters, called on a tile: `(trait id, int)` (converted to
/// float and forwarded to [`TILE_SET_FLOAT`]), `(trait id, float, bool)` and
/// `(trait id, string, bool)`.
const TILE_SET_INT: u32 = 0x0070_0320;
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const TILE_SET_STRING: u32 = 0x00a0_1350;
/// `Tile::UpdateTile(bool)` (Xbox PDB).
const TILE_UPDATE_TILE: u32 = 0x00a0_4640;
/// `Tile::TextToTrait(name)` (Xbox PDB, `cdecl`): the trait id of a name.
const TILE_TEXT_TO_TRAIT: u32 = 0x00a0_1860;
/// `Tile::AddDirtyTile(tile)` (Xbox PDB, `cdecl`).
const TILE_ADD_DIRTY_TILE: u32 = 0x00a0_7690;
/// `Tile::UpdateAll(bool)` (Xbox PDB, `cdecl`).
const TILE_UPDATE_ALL: u32 = 0x00a0_4200;
/// `Tile::ReadFile(path)` (Xbox PDB).
const TILE_READ_FILE: u32 = 0x00a0_1b00;
/// `Tile::SetParent(parent, bool)` (Xbox PDB).
const TILE_SET_PARENT: u32 = 0x00a0_87d0;
/// `Tile::LoadTextTable` (Xbox PDB).
const TILE_LOAD_TEXT_TABLE: u32 = 0x009f_f970;
/// `Tile::FreeTraitList` (Xbox PDB).
const TILE_FREE_TRAIT_LIST: u32 = 0x00a0_0a30;
/// `Tile::~Tile` and `Tile::Release` (Xbox PDB).
const TILE_DESTRUCT: u32 = 0x009f_f340;
const TILE_RELEASE: u32 = 0x009f_f690;
/// `Menu::RemoveFromXMLArchive(path, bool)` and `Menu::FreeXMLArchive`
/// (Xbox PDB).
const MENU_REMOVE_FROM_XML_ARCHIVE: u32 = 0x00a1_ca20;
const MENU_FREE_XML_ARCHIVE: u32 = 0x00a1_c800;
/// `InterfaceManager::GetScreenWidth` (Xbox PDB, `float` in `ST0`), and the
/// getters used next to it by `Init`.
const GET_SCREEN_WIDTH: u32 = 0x0071_5d40;
const GET_SCREEN_HEIGHT: u32 = 0x0071_5da0;
const GET_INTERFACE_MARGIN_A: u32 = 0x0071_77c0;
const GET_INTERFACE_MARGIN_B: u32 = 0x0071_7820;
const GET_INTERFACE_SCALE: u32 = 0x0070_7b60;
/// `InterfaceManager::CreateSceneGraph(parent, name, flag)` (Xbox PDB).
const CREATE_SCENE_GRAPH: u32 = 0x0071_2e90;
/// The root pointers `5bac70` and `5bac60` read (the parent each scene
/// graph is created under).
const SCENE_GRAPH_PARENT_3D: u32 = 0x005b_ac70;
const SCENE_GRAPH_PARENT: u32 = 0x005b_ac60;
/// `MenuManager::Instance(create)` (Xbox PDB, `cdecl`).
const MENU_MANAGER_INSTANCE: u32 = 0x0071_e290;
/// `NiAlphaProperty` constructor and the setters it is given: bit `0x1` of
/// the flag word, bit `0x2000`, the mask `0x1e` (source mode) and the mask
/// `0x1e0` (destination mode).
const ALPHA_PROPERTY_CONSTRUCT: u32 = 0x0043_91c0;
const ALPHA_PROPERTY_SET_BLEND: u32 = 0x0049_ed90;
const ALPHA_PROPERTY_SET_NO_SORTER: u32 = 0x0063_6000;
const ALPHA_PROPERTY_SET_SOURCE_MODE: u32 = 0x0043_9340;
const ALPHA_PROPERTY_SET_DESTINATION_MODE: u32 = 0x0043_9390;
/// `BSShaderAccumulator::BSShaderAccumulator(this, 100 or 99, 1, 0x2f7)`
/// (Xbox PDB) and its two single-word setters (`+0x194`, `+0x19c`).
const SHADER_ACCUMULATOR_CONSTRUCT: u32 = 0x00b6_60d0;
const SHADER_ACCUMULATOR_SET_194: u32 = 0x004a_1020;
const SHADER_ACCUMULATOR_SET_19C: u32 = 0x004a_1040;
/// `ShadowSceneNode::ShadowSceneNode` (Xbox PDB) and
/// `BSShaderManager::SetShadowSceneNode(this, slot)` (Xbox PDB).
const SHADOW_SCENE_NODE_CONSTRUCT: u32 = 0x00b5_e0f0;
const SET_SHADOW_SCENE_NODE: u32 = 0x0070_b760;
/// Constructors of the objects the manager owns: the 0xc-byte view caster
/// and the 0x3c-byte root tile.
const VIEW_CASTER_CONSTRUCT: u32 = 0x0063_19e0;
const MENU_ROOT_TILE_CONSTRUCT: u32 = 0x0070_9840;
/// `Timer` list head helpers: the table constructor `0070a8e0` calls, and
/// the callee that destroys the 0x4d4 member.
const KEY_TABLE_CONSTRUCT: u32 = 0x0070_6b90;
/// Reads the word at `+0x14` of the object at `011f6394` (the tick count
/// the fade code compares with).
const FADE_CLOCK_READ: u32 = 0x0082_5c00;
const FADE_CLOCK: u32 = 0x011f_6394;
/// `cdecl(float) -> ST0` wrappers of the two functions the one-to-one
/// distance is computed from.
const ANGLE_FUNCTION_A: u32 = 0x004e_44b0;
const ANGLE_FUNCTION_B: u32 = 0x004e_4470;
/// Setters of the pointer `5585e0` / `703060` and the font manager.
const FONT_MANAGER_INIT: u32 = 0x00a1_67c0;
const SYSTEM_COLOR_MANAGER_GET_INSTANCE: u32 = 0x0071_8b60;
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
const NODE_SET_NAME: u32 = 0x00a5_b950;
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTROY: u32 = 0x0043_81b0;
const FOG_PROPERTY_CONSTRUCT: u32 = 0x00bb_8180;
const TILE_IMAGE_NODE: u32 = 0x0056_c7f0;
const NODE_SET_TRANSLATE: u32 = 0x004b_c1f0;
const GET_FRAME_SCENE_NODE: u32 = 0x007f_a950;
const GET_SECOND_SCENE_NODE: u32 = 0x0055_85e0;
const SET_DEBUG_TEXT_VISIBLE: u32 = 0x0070_3060;
const GET_DESKTOP_WIDTH: u32 = 0x0070_6e50;
const GET_DESKTOP_HEIGHT: u32 = 0x0070_6e20;
const STRING_LIST_SET_LIMIT: u32 = 0x0045_ce80;
const LOADING_MENU_CREATE: u32 = 0x0070_5e00;
const HAS_360_CONTROLLER_GETTER: u32 = 0x004b_71d0;
const FLOAT_HOLDER_GET: u32 = 0x0040_3e20;
const STRING_HOLDER_GET: u32 = 0x0040_3df0;
const HUD_MAIN_MENU_CREATE: u32 = 0x0076_bfe0;
const HUD_MAIN_MENU_SET_MENU_MODE: u32 = 0x0077_1700;

layout! {
    /// `InterfaceManager` (Xbox PDB), 0x580 bytes on PC (0x478 on the Xbox).
    /// Offsets up to `+0x178` are the PDB's; the fields after the effect
    /// managers are PC only and named by their offset.
    pub struct InterfaceManager: 0x580 {
        /// `bFirstInit` (Xbox PDB).
        0x000 bFirstInit: u8,
        /// `bSecondInit` (Xbox PDB).
        0x001 bSecondInit: u8,
        /// `spSceneGraph` (Xbox PDB): `NiPointer<SceneGraph>`.
        0x004 spSceneGraph: u32,
        /// `sp3DSceneGraph` (Xbox PDB): `NiPointer<SceneGraph>`.
        0x008 sp3DSceneGraph: u32,
        /// `cMenuMode` (Xbox PDB).
        0x00c cMenuMode: u32,
        /// `bExternalForcedMenuMode` (Xbox PDB).
        0x010 bExternalForcedMenuMode: u8,
        /// `bLockMenuModeForFade` (Xbox PDB).
        0x011 bLockMenuModeForFade: u8,
        /// `cStatsPageNumber`, `cInventoryPageNumber`, `cMagicPageNumber`,
        /// `cMapPageNumber` (Xbox PDB), each a `char`.
        0x012 cStatsPageNumber: u8,
        0x013 cInventoryPageNumber: u8,
        0x014 cMagicPageNumber: u8,
        0x015 cMapPageNumber: u8,
        /// `iPickDistance` (Xbox PDB).
        0x018 iPickDistance: i32,
        /// `iSoundPauses` (Xbox PDB).
        0x020 iSoundPauses: u8,
        /// `iVoicePauses` (Xbox PDB).
        0x021 iVoicePauses: u8,
        /// `pPlayerLight` (Xbox PDB).
        0x024 pPlayerLight: u32,
        /// `pCursor` (Xbox PDB): `TileImage *`.
        0x028 pCursor: u32,
        /// `fMouseWheel` (Xbox PDB).
        0x044 fMouseWheel: f32,
        /// `fMouseHeldTime` (Xbox PDB).
        0x048 fMouseHeldTime: f32,
        /// `fDragOffsetLastX` and `fDragOffsetLastY` (Xbox PDB).
        0x054 fDragOffsetLastX: f32,
        0x058 fDragOffsetLastY: f32,
        /// `pDragTarget` (Xbox PDB).
        0x04c pDragTarget: u32,
        /// `iDragStartX` (Xbox PDB).
        0x050 iDragStartX: i32,
        /// `iDragStartY` (Xbox PDB).
        0x05c iDragStartY: i32,
        /// `iCursorOffsetX` (Xbox PDB).
        0x060 iCursorOffsetX: i32,
        /// `iCursorOffsetY` (Xbox PDB).
        0x064 iCursorOffsetY: i32,
        /// `iCurrentPickIndex` (Xbox PDB).
        0x078 iCurrentPickIndex: i32,
        /// `bDebugTextVisible` (Xbox PDB).
        0x07c bDebugTextVisible: u8,
        /// `bShowMouse` (Xbox PDB).
        0x07d bShowMouse: u8,
        /// `pInterfaceRoot` (Xbox PDB): `NiNode *`.
        0x080 pInterfaceRoot: u32,
        /// `pCursorRoot` (Xbox PDB): `NiNode *`.
        0x084 pCursorRoot: u32,
        /// `pPlayerRoot` (Xbox PDB).
        0x088 pPlayerRoot: u32,
        /// `spInterfaceAccum` (Xbox PDB): `NiPointer<BSShaderAccumulator>`.
        0x08c spInterfaceAccum: u32,
        /// `sp3dInterfaceAccum` (Xbox PDB).
        0x090 sp3dInterfaceAccum: u32,
        /// `pShadowNode` (Xbox PDB): `ShadowSceneNode *`.
        0x094 pShadowNode: u32,
        /// `pUIPlayerNode` (Xbox PDB): `ShadowSceneNode *`.
        0x098 pUIPlayerNode: u32,
        /// `pMenusRoot` (Xbox PDB): the root `Tile`.
        0x09c pMenusRoot: u32,
        /// `pStringRoot` (Xbox PDB): the tile of the globals file.
        0x0a0 pStringRoot: u32,
        /// `pDebugTextRoot` (Xbox PDB): `NiNode *`.
        0x0a4 pDebugTextRoot: u32,
        /// `fOneToOneDistance` (Xbox PDB).
        0x0a8 fOneToOneDistance: f32,
        /// `spAlphaProp` (Xbox PDB): `NiPointer<NiAlphaProperty>`.
        0x0ac spAlphaProp: u32,
        /// `bNeedToUpdate` (Xbox PDB).
        0x0b0 bNeedToUpdate: u8,
        /// `iCharHit` (Xbox PDB).
        0x0b2 iCharHit: u16,
        /// `pTargetReticle` (Xbox PDB).
        0x0b4 pTargetReticle: u32,
        /// `pSafeZone` (Xbox PDB).
        0x0b8 pSafeZone: u32,
        /// `pMouseOverTarget` (Xbox PDB).
        0x0bc pMouseOverTarget: u32,
        /// `iLastXDefault` (Xbox PDB).
        0x0c0 iLastXDefault: u32,
        /// `bPreLoadMainMenus` (Xbox PDB).
        0x0c8 bPreLoadMainMenus: u8,
        /// `bFirstChanceLoad` (Xbox PDB).
        0x0c9 bFirstChanceLoad: u8,
        /// `pOverTileTarget`, `pOverTileMenu`, `pDragOverTileTarget`,
        /// `pDragOverTileMenu` (Xbox PDB).
        0x0cc pOverTileTarget: u32,
        0x0d0 pOverTileMenu: u32,
        0x0d4 pDragOverTileTarget: u32,
        0x0d8 pDragOverTileMenu: u32,
        /// `bFullHelp` (Xbox PDB).
        0x0dd bFullHelp: u8,
        /// `cLastMessageButtonClicked` (Xbox PDB).
        0x0e4 cLastMessageButtonClicked: u8,
        /// `fpResult` (Xbox PDB).
        0x0e8 fpResult: u32,
        /// `bTextureRelease` (Xbox PDB).
        0x0ec bTextureRelease: u8,
        /// `bMouseInMotion` (Xbox PDB).
        0x0ed bMouseInMotion: u8,
        /// `pPickRef` (Xbox PDB).
        0x0f0 pPickRef: u32,
        /// `pReticleRef` (Xbox PDB).
        0x0f4 pReticleRef: u32,
        /// `pCrossHairRef` (Xbox PDB).
        0x0f8 pCrossHairRef: u32,
        /// `pActivateRef` (Xbox PDB).
        0x0fc pActivateRef: u32,
        /// `pTelekinesisRef` (Xbox PDB).
        0x100 pTelekinesisRef: u32,
        /// `bFuzzyActivatePick` (Xbox PDB).
        0x110 bFuzzyActivatePick: u8,
        /// `iEnterStack` (Xbox PDB): ten words on PC.
        0x114 iEnterStack: u32,
        /// `pViewCaster` (Xbox PDB).
        0x13c pViewCaster: u32,
        /// `bClickMultithreaded` (Xbox PDB).
        0x148 bClickMultithreaded: u8,
        /// `bShiftDown` (Xbox PDB).
        0x149 bShiftDown: u8,
        /// `iModifierKeys` (Xbox PDB).
        0x14c iModifierKeys: i32,
        /// `iRepeatingKey` (Xbox PDB).
        0x150 iRepeatingKey: i32,
        /// `uKeyDownTime` (Xbox PDB).
        0x154 uKeyDownTime: u32,
        /// `iLastGamepadEvent` (Xbox PDB).
        0x15c iLastGamepadEvent: i32,
        /// `uGamepadRepeatStartTime` (Xbox PDB).
        0x160 uGamepadRepeatStartTime: u32,
        /// `pTimers` (Xbox PDB): the head [`Timer`].
        0x164 pTimers: u32,
        /// `bIsInRenderedMenu` (Xbox PDB).
        0x168 bIsInRenderedMenu: u8,
        /// `pCurrentRenderedMenu` (Xbox PDB).
        0x16c pCurrentRenderedMenu: u32,
        /// `bMouseOverRenderedMenu` (Xbox PDB).
        0x170 bMouseOverRenderedMenu: u8,
        /// `pPipboy` (Xbox PDB): `FOPipboyManager *`.
        0x174 pPipboy: u32,
        /// PC only: float the constructor sets to 0.
        0x4ac field_4ac: f32,
        /// PC only: float the constructor sets to 0.
        0x4b0 field_4b0: f32,
        /// PC only: word the constructor sets to 0.
        0x4b8 field_4b8: u32,
        /// PC only: a mode word; `3` is tested by `00709c00`.
        0x4bc field_4bc: u32,
        /// PC only: word the constructor sets to 0.
        0x4c0 field_4c0: u32,
        /// PC only: byte the constructor sets to 0.
        0x4cc field_4cc: u8,
        /// PC only: byte the constructor sets to 0.
        0x4cd field_4cd: u8,
        /// PC only: float the constructor sets to 1.0.
        0x4d0 field_4d0: f32,
    }

    /// `InterfaceManager::Timer` (Xbox PDB), 0x14 bytes: a node of the timer
    /// list that `pTimers` heads.
    pub struct Timer: 0x14 {
        /// `pIndex` (Xbox PDB).
        0x00 pIndex: u32,
        /// `fElapsed` (Xbox PDB).
        0x04 fElapsed: f32,
        /// `fEnd` (Xbox PDB).
        0x08 fEnd: f32,
        /// `pPrev` (Xbox PDB).
        0x0c pPrev: u32,
        /// `pNext` (Xbox PDB).
        0x10 pNext: u32,
    }

    /// `TileImage` (Xbox PDB), 0x48 bytes: a `Tile` (0x38 bytes) plus the
    /// image fields.
    pub struct TileImage: 0x48 {
        /// `uiFlags` of the `Tile` base (Xbox PDB); bit `0x2000` is tested by
        /// `00709a20`.
        0x30 uiFlags: u32,
        /// `fScale` (Xbox PDB).
        0x38 fScale: f32,
        /// `spTexture` (Xbox PDB): `NiPointer<NiTexture>`.
        0x3c spTexture: u32,
        /// `spTSP` (Xbox PDB): `NiPointer<TileShaderProperty>`.
        0x40 spTSP: u32,
        /// `bIsScissor` (Xbox PDB).
        0x44 bIsScissor: u8,
    }
}

/// The vtable of `TileImage` (`__scalar_deleting_destructor_` first).
const TILE_IMAGE_VTABLE: u32 = 0x0106_f01c;
/// `Tile::IsReleased`: true when bit `0x2000` of the tile flags is set
/// (`00709a20`); the tile's destructor skips `Tile::Release` then.
const TILE_FLAG_TEST: u32 = 0x0070_9a20;
/// Base-class constructor of `TileImage` (`Tile::Tile`).
const TILE_CONSTRUCT: u32 = 0x0070_9860;
/// `TileImage::GetType` returns this.
const TILE_IMAGE_TYPE: u32 = 0x386;
/// `TileImage::GetTypeName` returns this string.
const TILE_IMAGE_TYPE_NAME: u32 = 0x0106_f044;

/// Runs `body` between the scope guard's constructor and destructor. The
/// guard is a 4-byte local that the game builds with the source file and
/// line (`00404eb0`) and tears down on every exit (`00404ee0`).
pub(crate) fn with_scope_guard<R>(
    e: &mut Engine,
    line: u32,
    body: impl FnOnce(&mut Engine) -> R,
) -> R {
    e.with_stack(4, |e, guard| {
        e.call(
            SCOPE_GUARD_BEGIN,
            &args![guard, 0xdu32, 1u32, SOURCE_FILE, line],
        );
        let result = body(e);
        e.call(SCOPE_GUARD_END, &args![guard]);
        result
    })
}

/// `new T` followed by `T::T(...)`: allocates `size` bytes through
/// `allocator` and runs `construct` on the block, or yields null when the
/// allocation fails.
fn construct_new(
    e: &mut Engine,
    allocator: u32,
    size: u32,
    construct: impl FnOnce(&mut Engine, Ptr) -> Ptr,
) -> Ptr {
    let block: Ptr = e.call(allocator, &args![size]).ptr();
    if block.is_null() {
        Ptr::NULL
    } else {
        construct(e, block)
    }
}

/// `delete object` through the virtual destructor (slot 0, flag 1).
fn delete_virtual(e: &mut Engine, object: u32) {
    e.vcall(object, 0, &args![1u32]);
}

fn tile_set_int(e: &mut Engine, tile: u32, trait_id: u32, value: u32) {
    e.call(TILE_SET_INT, &args![tile, trait_id, value]);
}

fn tile_set_float(e: &mut Engine, tile: u32, trait_id: u32, value: f32) {
    e.call(TILE_SET_FLOAT, &args![tile, trait_id, value, 1u32]);
}

/// The object a `NiPointer` member at `member` holds.
fn ni_pointer_get(e: &mut Engine, member: u32) -> u32 {
    e.call(NI_POINTER_GET, &args![member]).u32()
}

// Addresses used by the functions of this file (see each function).
const DEBUG_TEXT_SHUTDOWN: u32 = 0x00a0_dae0;
const FONT_MANAGER_SHUTDOWN: u32 = 0x00a1_6850;
/// A member function whose body only stores `this` (it does nothing).
const EMPTY_MEMBER_FUNCTION: u32 = 0x0048_3710;
/// Byte set while the root tiles are deleted.
const TILE_DELETING: u32 = 0x011d_8909;
const MENU_MANAGER_SHUTDOWN: u32 = 0x0071_e360;
const FREE_TIMERS: u32 = 0x0071_6450;
const LIST_CLEAR: u32 = 0x0047_0470;
const LIST_AT_011D8B54: u32 = 0x011d_8b54;
const STATIC_OBJECT_SHUTDOWN_A: u32 = 0x0071_8bf0;
const MESSAGE_MENU_SHUTDOWN: u32 = 0x007a_a480;
const CLEAR_TEMP_MODEL: u32 = 0x0070_7820;
const START_MENU_SHUTDOWN: u32 = 0x007c_e6b0;
const STATIC_OBJECT_SHUTDOWN_B: u32 = 0x0071_3c00;
const POINTER_LIST_REMOVE_ALL: u32 = 0x004e_d900;
const SCRIPT_POINTER_LIST: u32 = 0x011f_3324;
const GLOBAL_OBJECT_8CE8: u32 = 0x011d_8ce8;
const OBJECT_8CE8_DESTRUCT: u32 = 0x0071_b040;
const GLOBAL_OBJECT_A0C4: u32 = 0x011d_a0c4;
const VIEW_CASTER_DESTRUCT: u32 = 0x0063_1a50;
const FLOAT_HOLDER_BACKGROUND: u32 = 0x011d_3174;
const STRING_HOLDER_A: u32 = 0x011d_475c;
const STRING_HOLDER_B: u32 = 0x011f_3368;
const STRING_LIST_A: u32 = 0x011d_eec0;
const STRING_LIST_B: u32 = 0x011d_8d44;
/// `BSFogProperty` setter of two floats (`+0x2c`, `+0x30`).
const SET_FOG_RANGE: u32 = 0x005b_f9a0;
/// The global the `Controls` object is found from, and the getter.
const CONTROLS_OWNER: u32 = 0x011d_ea0c;
const CONTROLS_GET: u32 = 0x0087_7720;
/// `Controls::ClearKeystrokes` (Xbox PDB).
const CLEAR_KEYSTROKES: u32 = 0x00a2_37b0;
/// `HUDMainMenu::SetInfoForRef` (Xbox PDB, `cdecl`).
const HUD_SET_INFO_FOR_REF: u32 = 0x0077_5a00;
/// `MenuConsole::Instance(create)` (Xbox PDB, `cdecl`) and
/// `MenuConsole::OnEnterMenuMode` (Xbox PDB).
const MENU_CONSOLE_INSTANCE: u32 = 0x0071_b160;
const MENU_CONSOLE_ON_ENTER_MENU_MODE: u32 = 0x0071_d770;
/// The manager object itself (`004b7210`, the word at `011d8a80`).
const GET_MANAGER: u32 = 0x004b_7210;
/// `BSAudio::QInstance` (Xbox PDB) and the pause functions called on it.
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const AUDIO_PAUSE_TYPE: u32 = 0x00ad_8510;
const AUDIO_PAUSE_TYPE_6: u32 = 0x0070_bbe0;
const AUDIO_UNPAUSE_TYPE_6: u32 = 0x0070_bc00;
const AUDIO_UNPAUSE_TYPE_4000_0000: u32 = 0x0070_bbc0;
/// True when the manager's mode word is not 1 (`007023a0`).
const MENU_MODE_IS_NOT_ONE: u32 = 0x0070_23a0;
/// `InterfaceManager::UpdateAllTimers` (Xbox PDB).
const UPDATE_ALL_TIMERS: u32 = 0x0071_6320;

// Globals, objects and callees used by `Idle` and the gamepad/pick code.
const PLAYER_SINGLETON: u32 = 0x011d_ea3c;
const LAST_MENU_MODE: u32 = 0x0119_f530;
const CONTROLLER_STATE: u32 = 0x011d_8a6c;
const LAST_HAS_CONTROLLER: u32 = 0x011d_8c70;
const LAST_HAS_CONTROLLER_GUARD: u32 = 0x011d_8c74;
const MESSAGE_TITLE_HOLDER: u32 = 0x011d_38b8;
const TEXT_HOLDER_CONTROLLER_WAS_SET: u32 = 0x011d_49b4;
const TEXT_HOLDER_CONTROLLER_WAS_CLEAR: u32 = 0x011d_2760;
const SHOW_MESSAGE_BOX: u32 = 0x0070_3e80;
const CONTROLLER_CHANGED: u32 = 0x0071_9630;
const PRELOAD_MAIN_MENUS: u32 = 0x0071_7660;
/// Controls object queries (see each use for the arguments).
const CONTROLS_QUERY_00A239E0: u32 = 0x00a2_39e0;
const CONTROLS_QUERY_00A23A50: u32 = 0x00a2_3a50;
const CONTROLS_QUERY_00A24660: u32 = 0x00a2_4660;
const CONTROLS_QUERY_00A24180: u32 = 0x00a2_4180;
const CONTROLS_QUERY_00A238A0: u32 = 0x00a2_38a0;
const CONTROLS_AXIS_00A23390: u32 = 0x00a2_3390;
const CONTROLS_NEXT_EVENT: u32 = 0x00a2_3820;
const CONTROLS_CLEAR_USER_ACTIONS: u32 = 0x00a2_53d0;
const GET_PIPBOY: u32 = 0x0070_4370;
const PIPBOY_UPDATE_LIGHT_EFFECT: u32 = 0x007f_a540;
const HUD_EFFECTS_UPDATE: u32 = 0x007f_7830;
const VATS_EFFECTS_UPDATE: u32 = 0x0080_0370;
const GAME_STATE_ID: u32 = 0x0045_8030;
const CONDITION_CHECK_005A03F0: u32 = 0x005a_03f0;
const PLAYER_FLAG_00950090: u32 = 0x0095_0090;
const ACTOR_GET_IRON_SIGHTS: u32 = 0x008b_bc10;
const WEAPON_STATE_GETTER: u32 = 0x0044_ddc0;
const FORM_GETTER_0044DDC0: u32 = 0x0044_ddc0;
const WEAPON_STATE_OBJECT: u32 = 0x011f_2250;
const HUD_SET_MENU_MODE: u32 = 0x0077_1700;
const GET_ENTER_STACK_TOP: u32 = 0x0071_4f00;
const QUERY_00719AE0: u32 = 0x0071_9ae0;
const ACTION_00718930: u32 = 0x0071_8930;
const IS_TOP_MENU_ID: u32 = 0x0070_2450;
const IS_TOP_MENU_FADED_IN: u32 = 0x0070_24e0;
/// Takes no `this`: none of its 8 exe callers (0070c72c ...) sets ECX for it.
const IS_PIPBOY_MENU_TOPMOST: u32 = 0x0071_7920;
const OBJECT_FLAG_0070ED80: u32 = 0x0070_ed80;
const PIPBOY_MENU_CLASS: u32 = 0x0070_ede0;
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
const IS_CONSOLE_VISIBLE: u32 = 0x0070_3d50;
const IS_IN_GAME_LOADING_MENU_OPEN: u32 = 0x0070_5ea0;
const XUI_IS_UP: u32 = 0x0070_edf0;
const GET_MOUSE_OVER_TARGET: u32 = 0x0097_ae90;
const TILE_IS_VISIBLE: u32 = 0x00a0_40a0;
const TILE_IS_TRUE: u32 = 0x00a0_1230;
const GET_DEFAULT_FOCUS: u32 = 0x0071_5ca0;
const CONSOLE_PRIMARY_DOWN: u32 = 0x004a_4020;
const FRAME_TIME_GETTER: u32 = 0x0084_d030;
const GET_SCREEN_SCALE: u32 = 0x0070_ecb0;
const NI_POINT2_CONSTRUCT: u32 = 0x0045_2dc0;
const GET_MENU_HEIGHT: u32 = 0x0071_7860;
const PIPBOY_MARGIN: u32 = 0x0106_f070;
const TILE_GET_VALUE: u32 = 0x00a0_11b0;
const TILE_GET_VALUE_Q: u32 = 0x00a0_0e90;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
const PICK_TILE: u32 = 0x0071_26c0;
const DO_LEAVE: u32 = 0x0071_7ef0;
const DO_ENTER: u32 = 0x0071_7e70;
const DO_WHEEL_MOVE: u32 = 0x0071_8080;
const SET_CURSOR_OFFSET_X: u32 = 0x009e_3360;
const SET_CURSOR_OFFSET_Y: u32 = 0x008d_8500;
const GET_CURSOR_OFFSET_X: u32 = 0x005f_5f80;
const GET_CURSOR_OFFSET_Y: u32 = 0x0070_ec90;
const TILE_GET_POSITION_X: u32 = 0x00a0_13d0;
const TILE_GET_POSITION_Y: u32 = 0x00a0_1440;
const TILE_PLAY_TILE_SOUND: u32 = 0x00a0_b110;
const TILE_CHILD_LIST_FIND: u32 = 0x0049_c680;
const CLEAR_MOUSE_OVER_TARGET: u32 = 0x0070_6cb0;
const TILE_PARENT: u32 = 0x0045_cd60;
const GET_WHEEL_STEPS: u32 = 0x0070_ec70;
const PICK_LIST_COUNT: u32 = 0x005a_e380;
const MENU_STATE: u32 = 0x0059_bb30;
const FTOL: u32 = 0x00ec_62c0;
const RENDERED_MENU_OR_PIPBOY: u32 = 0x0070_7af0;
const MENU_FLAG_QUERY_004A4040: u32 = 0x004a_4040;
const SOUND_HANDLE_CONSTRUCT: u32 = 0x0041_a250;
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
const AUDIO_GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
const UI_MENU_MODE_SOUND: u32 = 0x0106_f060;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_RELEASE: u32 = 0x00ad_8d10;
const SET_STATS_MENU_VISIBLE: u32 = 0x0070_4c10;
const SET_INVENTORY_MENU_VISIBLE: u32 = 0x0070_48f0;
const SET_MAP_MENU_VISIBLE: u32 = 0x0070_4170;
const MENU_CONSOLE_IDLE: u32 = 0x0071_b210;
const KEY_REPEAT_RESET: u32 = 0x0071_66f0;
const TRANSLATE_KEY_EVENT: u32 = 0x0071_54b0;
const KEY_REPEAT_00716730: u32 = 0x0071_6730;
const MODIFIER_FOUR_DOWN: u32 = 0x0070_ecf0;
const GET_FRONTMOST_MENU: u32 = 0x0072_0e60;
const FORMAT_STRING: u32 = 0x0040_6d00;
const FORMAT_PC_BUTTON: u32 = 0x0106_f04c;
const PC_BUTTON_PREFIX: u32 = 0x0106_f054;
/// `[this + 4]` of the object (a plain getter used on tiles, lists and
/// menus).
const GET_FIELD_AT_4: u32 = 0x0072_6070;
const TILE_GET_STRING: u32 = 0x00a0_11f0;
const TILE_GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
const PLAY_MENU_SOUND: u32 = 0x0071_7280;
const SHOW_ICON_MESSAGE: u32 = 0x0070_52f0;
const MESSAGE_HOLDER_19: u32 = 0x011d_4cb4;
const MESSAGE_HOLDER_1A: u32 = 0x011d_3fd0;
const MESSAGE_ICON_SIZE: u32 = 0x0101_62c0;
const MESSAGE_ICON_PATH: u32 = 0x0102_08a0;
const CONSOLE_KEY_FLAG_GET: u32 = 0x0070_ed10;
const CONSOLE_KEY_FLAG_SET: u32 = 0x0070_ed20;
const MENU_CONSOLE_TOGGLE_VISIBLE: u32 = 0x0071_d580;
const ADD_TO_ENTER_STACK: u32 = 0x0071_4d90;
const POP_FROM_ENTER_STACK: u32 = 0x0071_4fd0;
const MENU_MANAGER_IS_MENU_OPEN: u32 = 0x0072_0f90;
const START_MENU_ALLOWED: u32 = 0x0119_f348;
const START_MENU_CLOSE: u32 = 0x007c_e7a0;
const START_MENU_CREATE: u32 = 0x007c_b7d0;
const START_MENU_SETTING: u32 = 0x011c_3ea4;
const QUEUE_PENDING_00070EC00: u32 = 0x0070_ec00;
const QUEUE_MENU_CREATE: u32 = 0x0070_9470;
const DEBUG_TEXT_UPDATE: u32 = 0x00a0_df80;
const MENU_LOOP_ABORT: u32 = 0x011d_8a85;
const LIST_NEXT_ELEMENT: u32 = 0x0057_cbe0;
const CHECK_MENU_BUTTON: u32 = 0x0071_7a40;
const CLEAR_MENU_BUTTON: u32 = 0x0071_7de0;
const TUTORIAL_MANAGER_UPDATE: u32 = 0x0071_82e0;
const GET_MENUS_ROOT: u32 = 0x0058_6150;
const LEVEL_UP_MENU_CREATE: u32 = 0x0078_4c80;
const CURSOR_STRING_PENDING: u32 = 0x011d_8904;
const LEVEL_UP_MENU_PENDING: u32 = 0x011d_8905;
const PLAYER_CAST_EAT_DRINK_ITEMS: u32 = 0x0095_d090;
const PLAYER_CAST_QUEUED_ENCHANTMENTS: u32 = 0x0095_ec40;
// `UpdatePipboy`.
const CONTAINER_MENU_CLOSE: u32 = 0x0075_b750;
const RENDERED_MENU_CLOSE: u32 = 0x007f_fd50;
const ACTOR_GET_ANIM_ACTION: u32 = 0x008a_7570;
const PLAYER_GET_ANIMATION: u32 = 0x0095_0a60;
const ANIMATION_CLEAR_GROUP: u32 = 0x0049_6080;
const PLAYER_FORCE_TEMP_1ST_PERSON: u32 = 0x0095_0460;
const ANIMATION_PLAY_GROUP: u32 = 0x0049_4740;
const GET_PIPBOY_STATIC: u32 = 0x0070_5990;
const PIPBOY_FADE_LIGHT_EFFECT: u32 = 0x007f_a7a0;
const PIPBOY_ACCESS_UP_SOUND: u32 = 0x0106_f0c4;
const IMAGE_SPACE_GET_HIT: u32 = 0x005d_2860;
const IMAGE_SPACE_STOP: u32 = 0x0052_9c90;
const PLAYER_STATE_00962590: u32 = 0x0096_2590;
const INTERFACE_SHOW_MENUS: u32 = 0x0070_3500;
const ANIMATION_GET_SEQUENCE: u32 = 0x0049_1040;
const MAP_MENU_NEEDS_TIDY: u32 = 0x007a_1750;
const MAP_MENU_TIDY: u32 = 0x0079_ffb0;
const PLAYER_IS_PIPBOY_ACTIVE: u32 = 0x0096_7ae0;
const ANIMATION_GROUP_ID: u32 = 0x0043_01b0;
const MAP_MENU_CLEAR_MAP_MEMORY: u32 = 0x007a_1490;
const RELOAD_DYNAMIC_IDLE_ON_1ST_PERSON: u32 = 0x0092_1250;
const GET_SAVED_ACQUIRE_OBJECT: u32 = 0x008d_8520;
const ACTOR_IS_WEAPON_DRAWN: u32 = 0x008a_16d0;
const WEAPON_TYPE_INDEX: u32 = 0x0044_6390;
const WEAPON_TABLE: u32 = 0x0118_a838;
const CLEAR_GUN_WOBBLE: u32 = 0x008d_6a80;
const ANIMATION_NODE: u32 = 0x0055_85e0;
const SEQUENCE_END_TIME: u32 = 0x0063_9aa0;
const ANIMATION_ZERO_GLOBAL_TRANSFORM: u32 = 0x0048_f7f0;
const ANIM_GROUP_GET_TIME: u32 = 0x005f_3780;
const ANIMATION_UPDATE: u32 = 0x0049_1180;
const ANIMATION_UPDATE_BIP_ONLY: u32 = 0x0049_bca0;
const ANIMATION_UPDATE_SCENE_GRAPH_NO_CONTROLLER: u32 = 0x0049_3bd0;
const IDLE_MANAGER_BUSY: u32 = 0x011c_ab24;
const LOG_WARNING: u32 = 0x005b_5e40;
const MISSING_DYNAMIC_IDLE_MESSAGE: u32 = 0x0106_f078;
const PIPBOY_MANAGER_UPDATE: u32 = 0x007f_8a80;
const ENTER_RENDERED_MENU: u32 = 0x0071_81d0;
const PLAYER_QUERY_005737E0: u32 = 0x0057_37e0;
const PLAYER_QUERY_004EAF60: u32 = 0x004e_af60;
const PLAYER_QUERY_0093A740: u32 = 0x0093_a740;
const PLAYER_QUERY_005721E0: u32 = 0x0057_21e0;
const HOT_KEYS_OWNER: u32 = 0x011d_96c0;
const TILE_IS_ACCEPTING_EVENTS: u32 = 0x0071_6910;
const PLAYER_BUSY_FLAG: u32 = 0x011e_0780;
const WEAPON_STATE_FLOAT: u32 = 0x008d_1cf0;
const FLAGS_OWNER_8804: u32 = 0x011d_8804;
const FLAGS_QUERY_00701450: u32 = 0x0070_1450;
const SUB_OBJECT_QUERY_00701740: u32 = 0x0070_1740;
const FLAGS_OWNER: u32 = 0x011d_aac0;
const HAS_FLAG: u32 = 0x004a_4080;
const INVENTORY_MENU: u32 = 0x011d_9ea4;
const KEYRING_TRAIT: u32 = 0x011d_9eb8;
// `GetTopGamepadButton` and `DoGamepadEvent`.
const REPEAT_DELAY_HOLDER_FAST: u32 = 0x011d_8aa4;
const REPEAT_DELAY_HOLDER_FIRST: u32 = 0x011d_8b38;
const DO_GAMEPAD: u32 = 0x0071_7f80;
const KEY_REPEAT_00705B10: u32 = 0x0070_5b10;
const LIST_PREVIOUS_ELEMENT: u32 = 0x0068_3520;
const SCAN_FOR_MAX_FOCUS: u32 = 0x0071_60f0;
const SET_CURRENT_FOCUS_TARGET: u32 = 0x0071_5860;
const TILE_GET_FIRST_REF_COPY: u32 = 0x00a0_3f70;
const LIST_PUSH_FRONT_005AE3D0: u32 = 0x005a_e3d0;
const LIST_CONTAINS: u32 = 0x005f_65d0;
// `00710880`.
const NI_PICK_CONSTRUCT: u32 = 0x00e9_8f20;
const NI_PICK_DESTRUCT: u32 = 0x00e9_8fa0;
const NI_PICK_SET_FLAG: u32 = 0x0045_8b30;
const GET_PICK_ROOT: u32 = 0x0084_e3a0;
const PICK_ROOT_OWNER: u32 = 0x011d_ea10;
const NI_PICK_SET_ROOT: u32 = 0x0070_5fc0;
const NI_PICK_SET_MODE: u32 = 0x006e_cd40;
const GET_SCENE_GRAPH: u32 = 0x0045_c670;
const SCENE_GRAPH_SET_CAMERA_FOV: u32 = 0x00c5_2020;
const SCENE_GRAPH_GET_CAMERA: u32 = 0x0066_29f0;
const CAMERA_BUILD_PICK_RAY: u32 = 0x00a7_1080;
const NI_PICK_PICK_OBJECTS: u32 = 0x00e9_8e20;
const NI_PICK_GET_RESULTS: u32 = 0x0050_0940;
const RESULTS_COUNT: u32 = 0x0044_edb0;
const RESULTS_GET: u32 = 0x0096_8670;
const RESULT_OBJECT: u32 = 0x0045_8b50;
const FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;
const LIST_ADD: u32 = 0x0090_5820;
// `DisplayCurrentPickRef`.
const GET_SETTING_VALUE: u32 = 0x0043_d4d0;
const DEBUG_TEXT_TOP_SETTING: u32 = 0x011d_ebac;
const FONT_LEVEL_SETTING: u32 = 0x011f_33c8;
const FONT_MANAGER_GET: u32 = 0x005b_d5b0;
const FONT_MANAGER_GET_FONT: u32 = 0x005b_d5c0;
const FONT_HEIGHT: u32 = 0x005b_d580;
const SET_PICK_REF_DISPLAY: u32 = 0x0071_4d70;
const FORM_ID_GETTER: u32 = 0x0084_e3a0;
const REFR_NAME: u32 = 0x0055_d520;
const FORMAT_INTO_STRING: u32 = 0x0040_6f60;
const FORMAT_NAME_AND_ID: u32 = 0x0103_a1c8;
const DEBUG_TEXT_X: u32 = 0x0103_a1c4;
const DEBUG_TEXT_INSTANCE: u32 = 0x00a0_d9e0;
const DEBUG_TEXT_PRINT: u32 = 0x00a0_f8b0;
const EMPTY_TEXT: u32 = 0x0101_1584;
const DEBUG_TEXT_BOTTOM: u32 = 0x011d_8c78;
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTROY: u32 = 0x0040_37d0;
const REFR_EXTRA_DATA_LIST: u32 = 0x005d_43c0;
const EXTRA_GET_SCRIPT: u32 = 0x0041_8800;
const SCRIPT_NAME_FORMAT: u32 = 0x0106_f274;
const FURNITURE_FORMAT: u32 = 0x0106_f254;
const REFR_GET_OWNER: u32 = 0x0056_7790;
const OWNER_FORMAT: u32 = 0x0106_f23c;
const EXTRA_GET_COUNT: u32 = 0x0041_8770;
const COUNT_FORMAT: u32 = 0x0106_f230;
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const FROM_TYPE: u32 = 0x0118_41cc;
const ACTOR_TYPE: u32 = 0x0118_46d4;
const ACTOR_PACKAGE_FORMAT: u32 = 0x0106_f208;
const ACTOR_FURNITURE_FORMAT: u32 = 0x0106_f1e8;
const REFR_IS_FURNITURE: u32 = 0x0056_8680;
const REFR_GET_ROTATION: u32 = 0x0043_0830;
const RADIANS_TO_DEGREES: u32 = 0x0102_f248;
const HEADING_FORMAT: u32 = 0x0106_f1d4;
const MARKER_CONSTRUCT: u32 = 0x0050_9890;
const REFR_GET_MARKER_AT_INDEX: u32 = 0x0056_8500;
const REFR_GET_FURNITURE_DATA: u32 = 0x007a_f430;
const MARKER_KIND: u32 = 0x0050_94b0;
const FURNITURE_GET_MARKER_ENABLED: u32 = 0x0050_9450;
const FURNITURE_IS_SIT_MARKER: u32 = 0x0050_9510;
const SIT_TEXT: u32 = 0x0106_4acc;
const SLEEP_TEXT: u32 = 0x0106_37d4;
const DISABLED_MARKER_FORMAT: u32 = 0x0106_f178;
const REFR_GET_SCALE: u32 = 0x0056_7400;
const FURNITURE_GET_MARKER_TARGET_OFFSET: u32 = 0x0050_9920;
const REFR_GET_MARKER_USED: u32 = 0x0056_7f80;
const USED_TEXT: u32 = 0x0106_f1cc;
const UNUSED_TEXT: u32 = 0x0106_f1c4;
const FURNITURE_GET_MARKER_ANGLE: u32 = 0x0050_99c0;
const MARKER_FORMAT: u32 = 0x0106_f190;
const REFR_GET_LOCK: u32 = 0x0056_9160;
const LOCK_IS_LOCKED: u32 = 0x0050_21a0;
const LOCKED_TEXT: u32 = 0x0106_0314;
const UNLOCKED_TEXT: u32 = 0x0106_f16c;
const LOCK_GET_LEVEL: u32 = 0x0043_09e0;
const LOCK_LEVEL_TABLE: u32 = 0x0118_4a98;
const LOCK_FORMAT: u32 = 0x0106_f15c;
const MAP_MARKER_GET_LOCATION_NAME: u32 = 0x0040_8da0;
const KEY_FORMAT: u32 = 0x0106_f150;
const REFR_GET_TELEPORT_DATA: u32 = 0x0056_8e50;
const DOOR_TELEPORT_GET_CELL: u32 = 0x0043_a2b0;
const UNKNOWN_TEXT: u32 = 0x0106_f148;
const PERSISTENT_TEXT: u32 = 0x0106_f13c;
const EXTRA_IS_PERSISTENT: u32 = 0x0041_d460;
const TELEPORT_FORMAT: u32 = 0x0106_f11c;
const REFR_FLAGS: u32 = 0x0057_2d10;
const STRING_COPY: u32 = 0x0040_6d30;
const FLAGS_PREFIX: u32 = 0x0106_f10c;
const STRING_APPEND: u32 = 0x0040_6d50;
const FLAG_TEXT_1: u32 = 0x0106_f100;
const FLAG_TEXT_2: u32 = 0x0106_f0f4;
const FLAG_TEXT_4: u32 = 0x0106_f0ec;
const FLAG_TEXT_8: u32 = 0x0106_f0d8;
const TRIM_TEXT: u32 = 0x0046_4f30;
// `007118d0`.
const CONTROLS_FLAG_00711E00: u32 = 0x0071_1e00;
const STICK_SCALE_HOLDER: u32 = 0x011d_8c5c;
const MOUSE_SETTING_00711DA0: u32 = 0x0071_1da0;
const MOUSE_SETTING_00711D80: u32 = 0x0071_1d80;
const MOUSE_LEVEL_00711D60: u32 = 0x0071_1d60;
const MOUSE_SETTING_00711DC0: u32 = 0x0071_1dc0;
const MOUSE_SETTING_00711DE0: u32 = 0x0071_1de0;
const NODE_TRANSLATION: u32 = 0x0043_c490;
const EDGE_RIGHT: u32 = 0x0071_1e40;
const EDGE_LEFT: u32 = 0x0071_1e20;
const EDGE_TOP: u32 = 0x0071_1e80;
const EDGE_BOTTOM: u32 = 0x0071_1e60;
const CURSOR_TILT_SCALE: u32 = 0x0106_f290;
const NODE_SET_TRANSLATE_VECTOR: u32 = 0x0044_0460;
const FLOAT_MIN: u32 = 0x0040_ebd0;
const FLOAT_MAX: u32 = 0x0040_4010;
const ABS: u32 = 0x00ec_7d40;
const MOUSE_LEVEL_DIVISOR: u32 = 0x0102_fc70;
const CURSOR_HORIZONTAL_SCALE: u32 = 0x0106_e960;
const CURSOR_VERTICAL_SCALE: u32 = 0x0106_e7f8;
const ONE_DOUBLE: u32 = 0x0101_2070;

// Callees and data of `00712450` .. `00714c20`.
/// `Tile::GetTileFromNode(node)` (Xbox PDB, `cdecl`): the tile a scene
/// graph node belongs to, or null.
const TILE_FROM_NODE: u32 = 0x00a0_1630;
/// The word at `+0x18` of a scene graph object: its parent.
const NODE_PARENT: u32 = 0x0096_11e0;
/// `cdecl(float) -> ST0`: a float wrapper of a CRT function (`00408860`
/// calls `00ec6cde` on a `double`); the fade code compares its result with
/// `1.0` and `0.0001`.
const FLOAT_WRAPPER_00408840: u32 = 0x0040_8840;
/// `NiAVObject::GetProperty(this, type)` (Xbox PDB).
const NODE_GET_PROPERTY: u32 = 0x00a5_9d30;
/// Returns the constant 3: the property type the fade and pick code ask
/// for.
const PROPERTY_TYPE_THREE: u32 = 0x0043_8220;
/// `(property, float)`: stores the faded value in the property.
const PROPERTY_SET_FADE: u32 = 0x0082_1600;
/// The number of children of a node, and the child at an index
/// (`(node, index)`).
const NODE_CHILD_COUNT: u32 = 0x0043_b480;
const NODE_CHILD_AT: u32 = 0x0043_b4a0;
/// `(node, index)`: the pointer held in the node's child array at `index`,
/// or null past the end.
const NODE_CHILD_POINTER: u32 = 0x0045_bc00;
/// `9000.0` (`double`): the fade value of a tile whose children are not
/// faded; `0.0001` (`double` of a `float`): the smallest fade that is not
/// zero.
const FADE_STOP_VALUE: u32 = 0x0106_f298;
const FADE_MINIMUM: u32 = 0x0103_2980;
/// The word at `+0x84` of the manager: `pCursorRoot`.
const CURSOR_ROOT_GETTER: u32 = 0x004f_d3c0;
/// `NiPick` byte setter `(pick, value)`: stores a byte at `+0x10`.
const NI_PICK_SET_BYTE_10: u32 = 0x0063_2d20;
/// `00408d60(holder)`: a pointer to the byte a setting holder keeps (the
/// address of a static zero when `holder` is null).
const SETTING_BYTE: u32 = 0x0040_8d60;
/// The setting holders the interface code reads.
const SETTING_HOLDER_011DB2CC: u32 = 0x011d_b2cc;
const SETTING_HOLDER_011D8BA0: u32 = 0x011d_8ba0;
const SETTING_HOLDER_011D8AB0: u32 = 0x011d_8ab0;
/// The byte that enables the custom viewport rectangle, and that
/// rectangle (four `float`s, `NiRect` `left right top bottom`, `0 1 1 0` by
/// default).
const VIEWPORT_RECT_ENABLED: u32 = 0x011f_9426;
const VIEWPORT_RECT: u32 = 0x011a_d840;
/// `NiRect<float>` constructor `(rect, left, right, top, bottom)`, returns
/// the rectangle.
const RECT_CONSTRUCT: u32 = 0x0041_4430;
/// `InterfaceManager::IsInPipboyMenu` (Xbox PDB) and
/// `InterfaceManager::IsCurrentRenderedMenuTopmost` (Xbox PDB).
const IS_IN_PIPBOY_MENU: u32 = 0x0071_78a0;
const IS_CURRENT_RENDERED_MENU_TOPMOST: u32 = 0x0071_7990;
/// `Interface::GetRealScreenWidth` (Xbox PDB, `ST0`), and the `float`
/// constant `00706e80` returns in `ST0`.
const REAL_SCREEN_WIDTH: u32 = 0x0070_6e40;
const SCREEN_WIDTH_CONSTANT: u32 = 0x0070_6e80;
/// The address `+0x9c` of a property (a rectangle of four `int`s).
const PROPERTY_RECT: u32 = 0x0050_d100;
/// Called on a node with no arguments (works on the node's array at
/// `+0x9c`); the result is not used.
const NODE_SIBLING_UPDATE: u32 = 0x004a_dd70;
/// The 16-bit word at `+0x18` of the pick result object.
const PICK_RESULT_HIT_WORD: u32 = 0x0047_d3d0;
/// `(node) -> bool`: calls `00456630(node, 1)`.
const NODE_FLAG_TEST_00456610: u32 = 0x0045_6610;
/// `112.0` (`double`): the value of the tile trait `0xfaa` of the menu tile
/// that makes the pick position scale by the manager's `+0x4d0`.
const PICK_SCALED_TILE_VALUE: u32 = 0x0106_ebd0;

// `CreateSceneGraph`.
/// Identity function (returns its first argument), called with `(name, 0)`.
const PASS_THROUGH: u32 = 0x0046_4f30;
/// `SceneGraph::SceneGraph(this, name, 0, 0)` (Xbox PDB).
const SCENE_GRAPH_CONSTRUCT: u32 = 0x0087_8610;
/// The renderer singleton (the word at `011f4748`).
const RENDERER_GET: u32 = 0x0043_c4b0;
/// `(camera, &matrix)`: sets the camera rotation from a 3x3 matrix.
const CAMERA_SET_ROTATION: u32 = 0x0043_fa80;
/// Width and height of the renderer (`ECX` is the renderer).
const RENDERER_WIDTH: u32 = 0x004d_ee10;
const RENDERER_HEIGHT: u32 = 0x004d_ee70;
/// `NiFrustum::NiFrustum(this, ortho)` (Xbox PDB) and
/// `NiCamera::SetViewFrustum(camera, &frustum)` (Xbox PDB).
const FRUSTUM_CONSTRUCT: u32 = 0x00a7_1b70;
const CAMERA_SET_FRUSTUM: u32 = 0x00a6_faf0;
/// The `FaderManager` object (the word at `011d8804`) and
/// `FaderManager::AddRoot(manager, graph, distance, flag)` (Xbox PDB).
const FADER_MANAGER: u32 = 0x011d_8804;
const FADER_ADD_ROOT: u32 = 0x0070_06c0;
/// The constants of the frustum: the aspect divisor (`double`), the top
/// (`float`), the far plane (`float`) and `0.5` (`double`); the width factor
/// is `CURSOR_HORIZONTAL_SCALE`.
const ASPECT_DIVISOR: u32 = 0x0102_1790;
const FRUSTUM_TOP: u32 = 0x0106_f2dc;
const FRUSTUM_FAR: u32 = 0x0102_2958;
const HALF: u32 = 0x0101_1588;
/// The field of view (`float`) a 3D scene graph's camera is given.
const FOV_3D: u32 = 0x0104_ef40;
/// `"InterfaceManager: Main Root"` and `"InterfaceManager: Cursor Root"`.
const MAIN_ROOT_NAME: u32 = 0x0106_f2c0;
const CURSOR_ROOT_NAME: u32 = 0x0106_f2a0;

// The render pass (`007134d0`, `00713fb0`).
/// `(process, word)` and `(process)`: push and pop of a culling
/// process's stack of accumulators (`+0x90`, `+0xbc`, `+0x94`).
const CULLING_PUSH: u32 = 0x00c4_f270;
const CULLING_POP: u32 = 0x00c4_f2d0;
/// The multithreaded rendering system (a constant address), its
/// `SetThreadStage(stage, 0x17)` (Xbox PDB), the wait `(stage, 0x17)`
/// and `AddAccumTask` (Xbox PDB, nine words).
const RENDERING_SYSTEM: u32 = 0x004e_a970;
const RENDERING_WAIT_STAGE: u32 = 0x00ba_3130;
const RENDERING_SET_STAGE: u32 = 0x00ba_30f0;
const RENDERING_ADD_ACCUM_TASK: u32 = 0x00ba_3390;
/// `BSCullingProcess::BSCullingProcess(this, word)` (Xbox PDB), its
/// destructor, and the setter `(process, accumulator)`.
const CULLING_PROCESS_CONSTRUCT: u32 = 0x004a_0eb0;
const CULLING_PROCESS_DESTRUCT: u32 = 0x004a_0f60;
const CULLING_SET_ACCUMULATOR: u32 = 0x004a_0fd0;
/// `BSShaderUtil::AccumulateScene(camera, scene, process)` (Xbox PDB,
/// `cdecl`) and the finishing `cdecl(camera, accumulator, 0)`.
const ACCUMULATE_SCENE: u32 = 0x00b6_bee0;
const ACCUMULATE_FINISH: u32 = 0x00b6_c0d0;
/// `Interface::IsolateMenuElements(a, b)` (Xbox PDB, `cdecl`) and
/// `Interface::RestoreMenuElements` (Xbox PDB, `cdecl`).
const ISOLATE_MENU_ELEMENTS: u32 = 0x0070_2c80;
const RESTORE_MENU_ELEMENTS: u32 = 0x0070_2dd0;
/// `(manager) -> bool` of the interface unit.
const MENU_PREDICATE_007079F0: u32 = 0x0070_79f0;
/// `(renderer, camera)`.
const RENDERER_SET_CAMERA: u32 = 0x004e_9bb0;
/// The locks the tile update takes: `enter(lock, 0)` and `leave(lock)`
/// (wrappers of `EnterCriticalSection` / `RtlLeaveCriticalSection`), the two
/// lock objects, and the queue of objects whose deletion is deferred with
/// the emptiness test (the count word at `+8` is zero) and pop.
const LOCK_ENTER: u32 = 0x0045_38a0;
const LOCK_LEAVE: u32 = 0x0045_38c0;
const TILE_LOCK: u32 = 0x011d_8c18;
const UPDATE_LOCK: u32 = 0x011f_3330;
const DEFERRED_QUEUE: u32 = 0x011d_8b2c;
const COLLECTION_IS_EMPTY: u32 = 0x0076_b610;
const DEFERRED_QUEUE_POP: u32 = 0x007b_5390;
/// The array of tiles to update (a `BSSimpleArray`), its `Find(&item)`
/// (`bool`), `Add(&item)`, the index search `(&item, 0, compare)`, the
/// comparison, `RemoveAt(index, 1)`, the size (`ECX` is the array), the
/// element address `(array, index)` and `Clear(flag)`.
const UPDATE_ARRAY: u32 = 0x011d_8b44;
const ARRAY_FIND: u32 = 0x0099_62f0;
const ARRAY_ADD: u32 = 0x007c_b2e0;
const ARRAY_FIND_INDEX: u32 = 0x0071_9b20;
const ARRAY_COMPARE: u32 = 0x009a_3830;
const ARRAY_REMOVE_AT: u32 = 0x009a_4320;
const WORD_AT_8: u32 = 0x0044_ddc0;
const ARRAY_ELEMENT: u32 = 0x0087_7a30;
const ARRAY_CLEAR: u32 = 0x0084_54f0;
/// The setting object `0043d4d0` is called on: a pointer to an `int`.
const SETTING_HOLDER_011C3EA4: u32 = 0x011c_3ea4;
/// An object with tables of handles at `+0x8c`; `(object, table, index)`
/// waits for ever on the handle `index` of the table (`WaitForSingleObject`).
/// The byte at `011dfa19` says whether it is in use.
const SEMAPHORE_POOL: u32 = 0x011d_fa50;
const SEMAPHORE_POOL_IN_USE: u32 = 0x011d_fa19;
const SEMAPHORE_POOL_WAIT: u32 = 0x008c_7a70;
/// The counter `00713d60` clears.
const TILE_UPDATE_DEPTH: u32 = 0x011f_32d4;
/// The byte set while the tiles update (`011d8908`).
const TILES_UPDATING: u32 = 0x011d_8908;
/// `Tile::UpdateFadeControls` (Xbox PDB), `Tile::UpdateChildren` (Xbox PDB)
/// and the unnamed `cdecl()` that runs before the update.
const TILE_UPDATE_FADE_CONTROLS: u32 = 0x00a0_80d0;
const TILE_UPDATE_CHILDREN: u32 = 0x00a0_4620;
const TILE_UPDATE_PREPARE: u32 = 0x00a0_4510;
/// `GetCurrentThreadId` wrapper and the word at `+0x10` of the object at
/// `011dea0c` (the owner thread's id).
const CURRENT_THREAD_ID: u32 = 0x0040_fc90;
const OWNER_THREAD_ID: u32 = 0x0044_edb0;
const OWNER_OBJECT: u32 = 0x011d_ea0c;
/// The device getter (`ECX` is the renderer; the device is a COM object,
/// so its methods get the device again as their first word) and the
/// renderer's state object getter.
const DEVICE_GET: u32 = 0x004d_c020;
const RENDERER_STATE_OBJECT: u32 = 0x004e_caf0;
/// `cdecl()` and `cdecl(7, 0)` of the renderer state code.
const RENDER_PASS_RESET: u32 = 0x00b6_b730;
const RENDER_PASS_SET_MODE: u32 = 0x00b6_b890;
/// `byte` getters of the renderer: is the pass active, is the clear
/// enabled.
const RENDER_PASS_ACTIVE: u32 = 0x004e_9510;
const RENDER_CLEAR_ENABLED: u32 = 0x004d_e080;
/// The clear colour object passed to the renderer's slot `0xac`.
const CLEAR_COLOR: u32 = 0x011a_9bd0;
/// `float`s of the two clear rectangles.
const CLEAR_RECT_ONE_TOP: u32 = 0x0102_6994;
const CLEAR_RECT_TWO_LEFT: u32 = 0x0103_40a4;
/// `(scene) -> object`: the object `DialoguePackage::GetTargetOfConversation`
/// (Xbox PDB name by identical-code folding) returns, here the source of
/// the culling process's word.
const CULLING_SOURCE_GET: u32 = 0x008d_80e0;
/// `() -> object`: the word at `011f5b04` (`WORKER_OBJECT`), and an
/// unnamed `cdecl()` that works on that object's queues; the render pass
/// calls it when the object's byte at `+0x1b0` is set.
const WORKER_GET: u32 = 0x0068_3a60;
const WORKER_FLUSH: u32 = 0x00a8_1a80;

// The semaphore wrapper (`00714900` .. `00714a00`).
/// The object (a pointer at `011f5b04`) whose `+0x190` member wraps a
/// semaphore handle (at `+8` of the member), and the byte `007149f0` sets.
const WORKER_OBJECT: u32 = 0x011f_5b04;
const WORKER_WAS_BLOCKED: u32 = 0x011f_5b08;
/// Imported `ReleaseSemaphore` and `WaitForSingleObject` (import slots).
const RELEASE_SEMAPHORE: u32 = 0x00fd_f1b4;
const WAIT_FOR_SINGLE_OBJECT: u32 = 0x00fd_f1c8;
/// `cdecl(object)` called with the wrapper after a wait that did not time
/// out (Xbox PDB name `WaitForSingleObjectEx` by identical-code folding),
/// and the `cdecl(object)` called before the release.
const WAIT_RESULT_HANDLER: u32 = 0x0040_19a0;
const BEFORE_RELEASE: u32 = 0x0040_b460;
/// `(object, string, flag) -> bool`, asked of the renderer with the string
/// at `KIND_NAME` before it is told to fill a strip.
const OBJECT_IS_KIND: u32 = 0x004a_0e10;
const KIND_NAME: u32 = 0x0106_f2e0;

// The render-state calls and counters of `00714a40` .. `00714c20`.
const RENDER_STATE_00B97DE0: u32 = 0x00b9_7de0;
const RENDER_STATE_00B97E30: u32 = 0x00b9_7e30;
const RENDER_STATE_00B97E80: u32 = 0x00b9_7e80;
const RENDER_STATE_00B97ED0: u32 = 0x00b9_7ed0;
const RENDER_STATE_00B97F20: u32 = 0x00b9_7f20;
const RENDER_STATE_00B97FA0: u32 = 0x00b9_7fa0;
const RENDER_STATE_00B97FF0: u32 = 0x00b9_7ff0;
const RENDER_STATE_00B980C0: u32 = 0x00b9_80c0;
const RENDER_STATE_00B98180: u32 = 0x00b9_8180;
const RENDER_STATE_00B98230: u32 = 0x00b9_8230;
const RENDER_STATE_00B984F0: u32 = 0x00b9_84f0;
const RENDER_STATE_00B98320: u32 = 0x00b9_8320;
const RENDER_STATE_00B98480: u32 = 0x00b9_8480;
const RENDER_STATE_004ECED0: u32 = 0x004e_ced0;
const RENDER_STATE_004ECB40: u32 = 0x004e_cb40;
const RENDER_STATE_004EB510: u32 = 0x004e_b510;
const RENDER_STATE_00714C40: u32 = 0x0071_4c40;
const RENDER_COUNTER_011FF9D8: u32 = 0x011f_f9d8;
const RENDER_COUNTER_011FF9DC: u32 = 0x011f_f9dc;
const RENDER_COUNTER_011FF9E0: u32 = 0x011f_f9e0;
const RENDER_COUNTER_011FF9E8: u32 = 0x011f_f9e8;
const RENDER_COUNTER_011FF9EC: u32 = 0x011f_f9ec;
const RENDER_COUNTER_011FF9F0: u32 = 0x011f_f9f0;
const RENDER_COUNTER_011FF9F4: u32 = 0x011f_f9f4;
const RENDER_COUNTER_011FFA00: u32 = 0x011f_fa00;
const RENDER_COUNTER_011FFA04: u32 = 0x011f_fa04;
const RENDER_COUNTER_011FFA08: u32 = 0x011f_fa08;
const RENDER_COUNTER_011FFA0C: u32 = 0x011f_fa0c;
const RENDER_COUNTER_011FFA10: u32 = 0x011f_fa10;
const RENDER_COUNTER_011FF9E4: u32 = 0x011f_f9e4;
const RENDER_COUNTER_011FFA20: u32 = 0x011f_fa20;
const WORD_AT_C: u32 = 0x0084_e3a0;

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00709fd0, interface_manager_initialize(i32, u8)),
        entry!(0x0070a0b0, fn_0070a0b0()),
        entry!(
            0x0070a100,
            interface_manager_scalar_deleting_destructor(
                Ptr<InterfaceManager>,
                u32,
            ) -> Ptr<InterfaceManager>
        ),
        entry!(
            0x0070a130,
            interface_manager_construct(Ptr<InterfaceManager>) -> Ptr<InterfaceManager>
        ),
        entry!(0x0070a8a0, timer_construct(Ptr<Timer>) -> Ptr<Timer>),
        entry!(0x0070a8e0, fn_0070a8e0(Ptr) -> Ptr),
        entry!(0x0070a900, fn_0070a900(Ptr, u8)),
        entry!(
            0x0070a920,
            interface_manager_destruct(Ptr<InterfaceManager>)
        ),
        entry!(0x0070aca0, fn_0070aca0()),
        entry!(0x0070acb0, fn_0070acb0()),
        entry!(0x0070ad00, fn_0070ad00(Ptr, u32) -> Ptr),
        entry!(0x0070ad30, fn_0070ad30()),
        entry!(0x0070ad80, fn_0070ad80(Ptr, u32) -> Ptr),
        entry!(
            0x0070adb0,
            interface_manager_init(Ptr<InterfaceManager>, u8)
        ),
        entry!(
            0x0070b240,
            interface_manager_init_second(Ptr<InterfaceManager>)
        ),
        entry!(
            0x0070b5d0,
            tile_image_construct(Ptr<TileImage>) -> Ptr<TileImage>
        ),
        entry!(0x0070b670, tile_image_get_type(Ptr<TileImage>) -> u32),
        entry!(0x0070b680, tile_image_get_type_name(Ptr<TileImage>) -> u32),
        entry!(
            0x0070b690,
            tile_image_scalar_deleting_destructor(Ptr<TileImage>, u32) -> Ptr<TileImage>
        ),
        entry!(0x0070b6c0, tile_image_destruct(Ptr<TileImage>)),
        entry!(
            0x0070b8f0,
            interface_manager_pre_idle_stuff(Ptr<InterfaceManager>)
        ),
        entry!(0x0070bba0, fn_0070bba0(Ptr)),
        entry!(0x0070c4a0, interface_manager_idle(Ptr<InterfaceManager>)),
        entry!(0x0070ec20, fn_0070ec20(Ptr, i32, i32)),
        entry!(0x0070ec50, fn_0070ec50(Ptr) -> u8),
        entry!(0x0070ecd0, fn_0070ecd0(Ptr<InterfaceManager>) -> bool),
        entry!(0x0070ed30, inventory_menu_is_keyring_open() -> bool),
        entry!(0x0070edc0, fn_0070edc0(Ptr) -> u8),
        entry!(0x0070ee30, fn_0070ee30() -> bool),
        entry!(
            0x0070ee80,
            interface_manager_update_pipboy(Ptr<InterfaceManager>)
        ),
        entry!(0x0070f490, fn_0070f490(Ptr, i32) -> u32),
        entry!(0x0070f4e0, fn_0070f4e0(Ptr<InterfaceManager>, u32, u32)),
        entry!(0x0070f670, fn_0070f670(Ptr) -> u32),
        entry!(0x0070f690, fn_0070f690(Ptr<InterfaceManager>, u32)),
        entry!(
            0x0070f6e0,
            interface_manager_do_gamepad_event(Ptr<InterfaceManager>, i32)
        ),
        entry!(
            0x00710060,
            interface_manager_get_top_gamepad_button(Ptr<InterfaceManager>, u8, u8, Ptr) -> i32
        ),
        entry!(0x00710880, fn_00710880(Ptr<InterfaceManager>)),
        entry!(0x00710ab0, fn_00710ab0(Ptr) -> f32),
        entry!(
            0x00710ad0,
            interface_manager_display_current_pick_ref(Ptr<InterfaceManager>)
        ),
        entry!(0x007118d0, fn_007118d0(Ptr<InterfaceManager>)),
        entry!(
            0x00712450,
            interface_manager_recursive_fade(Ptr<InterfaceManager>, u32, f32, f32)
        ),
        entry!(0x007126c0, fn_007126c0(Ptr<InterfaceManager>, u8) -> u32),
        entry!(0x00712e60, fn_00712e60(Ptr, u32)),
        entry!(
            0x00712e90,
            interface_manager_create_scene_graph(Ptr<InterfaceManager>, u32, u32, u8) -> u32
        ),
        entry!(0x007133b0, fn_007133b0(Ptr, u32, f32, f32, f32)),
        entry!(0x007133f0, fn_007133f0(Ptr<InterfaceManager>)),
        entry!(0x007134d0, fn_007134d0(Ptr<InterfaceManager>, u32, u32)),
        entry!(0x00713c00, fn_00713c00()),
        entry!(0x00713c70, fn_00713c70(Ptr<InterfaceManager>)),
        entry!(0x00713d60, fn_00713d60()),
        entry!(0x00713d70, fn_00713d70() -> u32),
        entry!(0x00713d80, fn_00713d80() -> u32),
        entry!(0x00713d90, fn_00713d90() -> u8),
        entry!(
            0x00713da0,
            interface_manager_add_tile_to_update_list(Ptr<InterfaceManager>, u32)
        ),
        entry!(0x00713de0, fn_00713de0(Ptr<InterfaceManager>, u32)),
        entry!(0x00713e20, fn_00713e20(Ptr<InterfaceManager>)),
        entry!(0x00713ee0, fn_00713ee0(Ptr)),
        entry!(0x00713f00, fn_00713f00(Ptr<InterfaceManager>)),
        entry!(0x00713fb0, fn_00713fb0(Ptr<InterfaceManager>, u32, u32)),
        entry!(0x007148c0, fn_007148c0(Ptr, u32, u32)),
        entry!(0x00714900, fn_00714900() -> u32),
        entry!(0x00714930, fn_00714930(Ptr) -> u32),
        entry!(0x00714960, fn_00714960(u32) -> u32),
        entry!(0x007149b0, fn_007149b0(Ptr, u32) -> u32),
        entry!(0x007149f0, fn_007149f0()),
        entry!(0x00714a00, fn_00714a00() -> bool),
        entry!(0x00714a40, fn_00714a40(u32)),
        entry!(0x00714a60, fn_00714a60(u32)),
        entry!(0x00714a80, fn_00714a80(u32)),
        entry!(0x00714aa0, fn_00714aa0(u32)),
        entry!(0x00714ac0, fn_00714ac0(u32)),
        entry!(0x00714ae0, fn_00714ae0(u32)),
        entry!(0x00714b00, fn_00714b00(u32)),
        entry!(0x00714b20, fn_00714b20(u32)),
        entry!(0x00714b50, fn_00714b50(u32)),
        entry!(0x00714b80, fn_00714b80(u32)),
        entry!(0x00714bb0, fn_00714bb0(u32)),
        entry!(0x00714bd0, fn_00714bd0(u32)),
        entry!(0x00714bf0, fn_00714bf0(u32)),
        entry!(0x00714c20, fn_00714c20(u32)),
        entry!(0x00714c40, fn_00714c40(u32)),
        entry!(
            0x00714c60,
            interface_manager_emergency_close_all_menus_and_break_stuff(Ptr<InterfaceManager>)
        ),
        entry!(0x00714d70, fn_00714d70(Ptr<InterfaceManager>, u32)),
        entry!(
            0x00714d90,
            interface_manager_add_to_enter_stack(Ptr<InterfaceManager>, i32) -> i32
        ),
        entry!(
            0x00714f00,
            interface_manager_get_enter_stack_top(Ptr<InterfaceManager>) -> u32
        ),
        entry!(
            0x00714f70,
            interface_manager_get_enter_stack(Ptr<InterfaceManager>, u32) -> u32
        ),
        entry!(0x00714f90, fn_00714f90(Ptr<InterfaceManager>, i32) -> bool),
        entry!(
            0x00714fd0,
            interface_manager_pop_from_enter_stack(Ptr<InterfaceManager>, i32, u8) -> i32
        ),
        entry!(0x007151b0, fn_007151b0(Ptr<InterfaceManager>, f32, f32)),
        entry!(
            0x007154b0,
            fn_007154b0(Ptr<InterfaceManager>, i32, i32) -> u32
        ),
        entry!(0x00715770, fn_00715770() -> bool),
        entry!(
            0x007157b0,
            interface_manager_clear_over_tile_target(Ptr<InterfaceManager>, u8)
        ),
        entry!(
            0x00715860,
            interface_manager_set_current_focus_target(Ptr<InterfaceManager>, u32, u32, u8)
        ),
        entry!(0x00715c60, fn_00715c60(u32, u32, u32)),
        entry!(
            0x00715ca0,
            interface_manager_get_default_focus(Ptr<InterfaceManager>)
        ),
        entry!(0x00715d40, interface_manager_get_screen_width() -> f32),
        entry!(0x00715da0, fn_00715da0() -> f32),
        entry!(0x00715e00, fn_00715e00(Ptr<InterfaceManager>, i32, i32)),
        entry!(
            0x00715ec0,
            interface_manager_toggle_safe_zone(Ptr<InterfaceManager>, i32)
        ),
        entry!(0x00716010, fn_00716010(Ptr<InterfaceManager>, u32)),
        entry!(
            0x007160b0,
            interface_manager_force_texture_release(Ptr<InterfaceManager>)
        ),
        entry!(
            0x007160f0,
            interface_manager_scan_for_max_focus(Ptr<InterfaceManager>, u32, u32) -> u32
        ),
        entry!(
            0x00716320,
            interface_manager_update_all_timers(Ptr<InterfaceManager>)
        ),
        entry!(0x00716440, fn_00716440() -> f32),
        entry!(0x00716450, fn_00716450(Ptr<InterfaceManager>)),
        entry!(0x007164c0, interface_manager_new_timer(u32, f32)),
        entry!(0x007165d0, interface_manager_clear_timer(u32)),
        entry!(0x00716660, fn_00716660(u32) -> f32),
        entry!(0x007166f0, fn_007166f0(Ptr<InterfaceManager>, u32)),
        entry!(0x00716730, fn_00716730(Ptr<InterfaceManager>, f32) -> u32),
        entry!(
            0x00716910,
            interface_manager_tile_is_accepting_events(Ptr<InterfaceManager>, u32) -> bool
        ),
        entry!(0x00716980, fn_00716980(Ptr<TextEntry>) -> Ptr<TextEntry>),
        entry!(0x00716a10, fn_00716a10(Ptr<TextEntry>)),
        entry!(0x00716a70, fn_00716a70(Ptr<TextEntry>, u32)),
        entry!(0x00716aa0, fn_00716aa0(Ptr<TextEntry>, i32)),
        entry!(0x00716ae0, fn_00716ae0(Ptr<TextEntry>) -> u8),
        entry!(0x00716b00, fn_00716b00(Ptr<TextEntry>, u32)),
        entry!(0x00717010, fn_00717010(Ptr<TextEntry>, u8)),
        entry!(0x00717050, fn_00717050(Ptr<TextEntry>)),
        entry!(0x007170a0, fn_007170a0(Ptr<TextEntry>) -> u32),
        // @@ENTRIES@@
    ]
}

// Translated from 00709fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::Initialize` (Xbox PDB, static, `cdecl`): records
/// whether an Xbox 360 controller answers, creates the manager if there is
/// none, runs the first (`phase` 1, with `flag`) or second (`phase` 2)
/// initialization, and clears the sound and voice pause counters.
pub fn interface_manager_initialize(e: &mut Engine, phase: i32, flag: u8) {
    let state = e.with_stack(0x10, |e, state| {
        e.call(XINPUT_GET_STATE, &args![0u32, state]).u32()
    });
    e.set_global(HAS_360_CONTROLLER, (state == 0) as u8);
    if e.global::<u32>(MANAGER_SINGLETON) == 0 {
        let manager = construct_new(e, OPERATOR_NEW, 0x580, |e, block| {
            interface_manager_construct(e, block.cast()).cast()
        });
        e.set_global(MANAGER_SINGLETON, manager.addr());
    }
    let manager: Ptr<InterfaceManager> = Ptr::new(e.global(MANAGER_SINGLETON));
    if phase == 1 {
        interface_manager_init(e, manager, flag);
    } else if phase == 2 {
        interface_manager_init_second(e, manager);
    }
    let manager: Ptr<InterfaceManager> = Ptr::new(e.global(MANAGER_SINGLETON));
    e.set(manager, InterfaceManager::iSoundPauses, 0);
    let manager: Ptr<InterfaceManager> = Ptr::new(e.global(MANAGER_SINGLETON));
    e.set(manager, InterfaceManager::iVoicePauses, 0);
}

// Translated from 0070a0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the interface manager (through its deleting destructor) and
/// clears the singleton; does nothing when there is none.
pub fn fn_0070a0b0(e: &mut Engine) {
    let manager: Ptr<InterfaceManager> = Ptr::new(e.global(MANAGER_SINGLETON));
    if !manager.is_null() {
        interface_manager_scalar_deleting_destructor(e, manager, 1);
        e.set_global(MANAGER_SINGLETON, 0u32);
    }
}

// Translated from 0070a100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::__scalar_deleting_destructor_` (Xbox PDB): destroys
/// the manager and, when bit 0 of `flags` is set, frees its memory.
pub fn interface_manager_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<InterfaceManager>,
    flags: u32,
) -> Ptr<InterfaceManager> {
    interface_manager_destruct(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0070a130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::InterfaceManager` (Xbox PDB): constructs the embedded
/// members, zeroes or defaults every field, derives the one-to-one
/// distance from the screen width, creates the alpha property, the two
/// scene graphs (`Menu3DRoot`, `MenuRoot`), the menu manager, the view
/// caster, the timer list head and the two shader accumulators. Returns
/// `this`.
pub fn interface_manager_construct(
    e: &mut Engine,
    this: Ptr<InterfaceManager>,
) -> Ptr<InterfaceManager> {
    let base = this.addr();
    e.call(NI_POINTER_CONSTRUCT, &args![base + 0x4, 0u32]);
    e.call(NI_POINTER_CONSTRUCT, &args![base + 0x8, 0u32]);
    e.call(MEMBER_CONSTRUCTOR_EMPTY, &args![base + 0x2c]);
    e.call(MEMBER_CONSTRUCTOR_EMPTY, &args![base + 0x38]);
    e.call(SIMPLE_LIST_CONSTRUCT, &args![base + 0x70]);
    e.call(NI_POINTER_CONSTRUCT, &args![base + 0x8c, 0u32]);
    e.call(NI_POINTER_CONSTRUCT, &args![base + 0x90, 0u32]);
    e.call(NI_POINTER_CONSTRUCT, &args![base + 0xac, 0u32]);
    e.call(MEMBER_CONSTRUCTOR_EMPTY, &args![base + 0x104]);
    e.call(HUD_EFFECT_MANAGER_CONSTRUCT, &args![base + 0x178]);
    e.call(VATS_EFFECT_MANAGER_CONSTRUCT, &args![base + 0x1dc]);
    e.call(SIMPLE_LIST_CONSTRUCT, &args![base + 0x4c4]);
    fn_0070a8e0(e, Ptr::new(base + 0x4d4));
    with_scope_guard(e, 0x11d, |e| {
        e.set(this, InterfaceManager::field_4d0, 1.0);
        e.set(this, InterfaceManager::bFirstInit, 0);
        e.set(this, InterfaceManager::bSecondInit, 0);
        e.set(this, InterfaceManager::pInterfaceRoot, 0);
        e.set(this, InterfaceManager::pCursorRoot, 0);
        e.set(this, InterfaceManager::pPlayerRoot, 0);
        e.set(this, InterfaceManager::pMenusRoot, 0);
        e.set(this, InterfaceManager::pDebugTextRoot, 0);
        // `NiPoint3(0, 0, 0)` built in a temporary and copied into the
        // activate pick location (`+0x104`) word by word.
        let origin = e.with_stack(0xc, |e, temp| {
            let at = e
                .call(NI_POINT3_CONSTRUCT, &args![temp, 0.0f32, 0.0f32, 0.0f32])
                .u32();
            [e.mem.u32(at), e.mem.u32(at + 4), e.mem.u32(at + 8)]
        });
        for (i, word) in origin.iter().enumerate() {
            e.mem.set_u32(base + 0x104 + 4 * i as u32, *word);
        }
        e.set(this, InterfaceManager::bFuzzyActivatePick, 0);
        e.set(this, InterfaceManager::pActivateRef, 0);
        e.set(this, InterfaceManager::pTelekinesisRef, 0);
        e.set(this, InterfaceManager::pPickRef, 0);
        e.set(this, InterfaceManager::pReticleRef, 0);
        e.set(this, InterfaceManager::pCrossHairRef, 0);
        e.set(this, InterfaceManager::pPlayerLight, 0);
        e.set(this, InterfaceManager::fOneToOneDistance, 0.0);
        e.set(this, InterfaceManager::cMenuMode, 1);
        e.set(this, InterfaceManager::bLockMenuModeForFade, 0);
        e.set(this, InterfaceManager::bExternalForcedMenuMode, 0);
        e.set(this, InterfaceManager::cStatsPageNumber, 0xff);
        e.set(this, InterfaceManager::cInventoryPageNumber, 0xff);
        e.set(this, InterfaceManager::cMagicPageNumber, 0xff);
        e.set(this, InterfaceManager::cMapPageNumber, 0xff);
        e.set(this, InterfaceManager::iCharHit, 0);
        e.set(this, InterfaceManager::pMouseOverTarget, 0);
        e.set(this, InterfaceManager::pOverTileTarget, 0);
        e.set(this, InterfaceManager::pOverTileMenu, 0);
        e.set(this, InterfaceManager::pDragOverTileTarget, 0);
        e.set(this, InterfaceManager::pDragOverTileMenu, 0);
        e.call(NI_POINTER_ASSIGN, &args![base + 0xac, 0u32]);
        e.set(this, InterfaceManager::iPickDistance, 0x50);
        e.set(this, InterfaceManager::fMouseWheel, 0.0);
        e.set(this, InterfaceManager::iDragStartX, 0);
        e.set(this, InterfaceManager::iDragStartY, 0);
        e.set(this, InterfaceManager::iCursorOffsetX, 0);
        e.set(this, InterfaceManager::iCursorOffsetY, 0);
        e.set(this, InterfaceManager::fMouseHeldTime, 0.0);
        e.set(this, InterfaceManager::pCursor, 0);
        e.set(this, InterfaceManager::bShiftDown, 0);
        e.set(this, InterfaceManager::iModifierKeys, 0);
        e.set(this, InterfaceManager::iRepeatingKey, 0);
        e.set(this, InterfaceManager::uKeyDownTime, 0);
        e.set(this, InterfaceManager::iLastGamepadEvent, 0);
        e.set(this, InterfaceManager::pTargetReticle, 0);
        e.set(this, InterfaceManager::pSafeZone, 0);
        e.set(this, InterfaceManager::pDragTarget, 0);
        e.set(this, InterfaceManager::bFullHelp, 0);
        e.set(this, InterfaceManager::cLastMessageButtonClicked, 0xff);
        e.set(this, InterfaceManager::fpResult, 0);
        e.set(this, InterfaceManager::bTextureRelease, 0);
        e.set(this, InterfaceManager::field_4cc, 0);
        e.set(this, InterfaceManager::bMouseInMotion, 1);
        e.set(this, InterfaceManager::iLastXDefault, 100);
        e.set(this, InterfaceManager::bFirstChanceLoad, 0);
        e.set(this, InterfaceManager::bPreLoadMainMenus, 1);
        e.set_global(CURR_ALPHA_DPS, 0.0f32);
        e.set_global(ALPHA_GOING_DOWN, 0u8);
        let tick = e.call(FADE_CLOCK_READ, &args![FADE_CLOCK]).u32();
        e.set_global(LAST_DPS_FADE_TIME, tick);
        for i in 0..10u32 {
            e.mem.set_u32(base + 0x114 + 4 * i, 0);
        }

        // One interface unit in world distance: half the screen width over
        // the first angle function, times the second.
        let angle: f32 = e.global(ONE_TO_ONE_ANGLE);
        let half_width = e.call(GET_SCREEN_WIDTH, &args![]).f64() / e.global::<f64>(TWO);
        let first = e.call(ANGLE_FUNCTION_A, &args![angle]).f64();
        let ratio = (half_width / first) as f32;
        let second = e.call(ANGLE_FUNCTION_B, &args![angle]).f64();
        e.set(
            this,
            InterfaceManager::fOneToOneDistance,
            (second * ratio as f64) as f32,
        );

        if ni_pointer_get(e, base + 0xac) == 0 {
            let alpha = construct_new(e, NI_ALLOC, 0x1c, |e, block| {
                e.call(ALPHA_PROPERTY_CONSTRUCT, &args![block]).ptr()
            });
            e.call(NI_POINTER_ASSIGN, &args![base + 0xac, alpha]);
            for (setter, value) in [
                (ALPHA_PROPERTY_SET_BLEND, 1u32),
                (ALPHA_PROPERTY_SET_NO_SORTER, 0),
                (ALPHA_PROPERTY_SET_SOURCE_MODE, 6),
                (ALPHA_PROPERTY_SET_DESTINATION_MODE, 7),
            ] {
                let property = ni_pointer_get(e, base + 0xac);
                e.call(setter, &args![property, value]);
            }
        }
        let parent = e.call(SCENE_GRAPH_PARENT_3D, &args![]).u32();
        let graph = e
            .call(
                CREATE_SCENE_GRAPH,
                &args![this, parent, MENU_3D_ROOT_NAME, 1u32],
            )
            .u32();
        e.call(NI_POINTER_ASSIGN, &args![base + 0x8, graph]);
        let parent = e.call(SCENE_GRAPH_PARENT, &args![]).u32();
        let graph = e
            .call(
                CREATE_SCENE_GRAPH,
                &args![this, parent, MENU_ROOT_NAME, 0u32],
            )
            .u32();
        e.call(NI_POINTER_ASSIGN, &args![base + 0x4, graph]);
        e.call(MENU_MANAGER_INSTANCE, &args![1u32]);
        let view_caster = construct_new(e, OPERATOR_NEW, 0xc, |e, block| {
            e.call(VIEW_CASTER_CONSTRUCT, &args![block]).ptr()
        });
        e.set(this, InterfaceManager::pViewCaster, view_caster.addr());
        let timers = construct_new(e, OPERATOR_NEW, 0x14, |e, block| {
            timer_construct(e, block.cast()).cast()
        });
        e.set(this, InterfaceManager::pTimers, timers.addr());
        // The head timer's `pPrev` points at itself.
        e.mem.set_u32(timers.addr() + 0xc, timers.addr());
        e.set(this, InterfaceManager::field_4ac, 0.0);
        e.set(this, InterfaceManager::field_4b0, 0.0);
        e.set(this, InterfaceManager::bMouseOverRenderedMenu, 0);
        e.set(this, InterfaceManager::bIsInRenderedMenu, 0);
        e.set(this, InterfaceManager::pCurrentRenderedMenu, 0);
        e.set(this, InterfaceManager::pPipboy, 0);
        e.set(this, InterfaceManager::field_4bc, 0);
        e.set(this, InterfaceManager::field_4c0, 0);
        e.set(this, InterfaceManager::pShadowNode, 0);
        // The interface accumulator (id 100: setter `+0x19c` = 10, byte
        // `+0x32` = 1) and the 3D one (id 99).
        let accumulator = construct_new(e, NI_ALLOC, 0x280, |e, block| {
            e.call(
                SHADER_ACCUMULATOR_CONSTRUCT,
                &args![block, 100u32, 1u32, 0x2f7u32],
            )
            .ptr()
        });
        e.call(NI_POINTER_ASSIGN, &args![base + 0x8c, accumulator]);
        let held = ni_pointer_get(e, base + 0x8c);
        e.call(SHADER_ACCUMULATOR_SET_19C, &args![held, 10u32]);
        let held = ni_pointer_get(e, base + 0x8c);
        fn_0070a900(e, Ptr::new(held), 1);
        let accumulator = construct_new(e, NI_ALLOC, 0x280, |e, block| {
            e.call(
                SHADER_ACCUMULATOR_CONSTRUCT,
                &args![block, 99u32, 1u32, 0x2f7u32],
            )
            .ptr()
        });
        e.call(NI_POINTER_ASSIGN, &args![base + 0x90, accumulator]);
        e.set(this, InterfaceManager::bClickMultithreaded, 0);
        e.set(this, InterfaceManager::field_4b8, 0);
        e.set(this, InterfaceManager::bShowMouse, 0);
        e.set(this, InterfaceManager::field_4cd, 0);
        e.set(this, InterfaceManager::iCurrentPickIndex, 0);
    });
    this
}

// Translated from 0070a8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::Timer` constructor (Xbox PDB layout): no neighbours,
/// zero elapsed time, the default end time. Returns `this`.
pub fn timer_construct(e: &mut Engine, this: Ptr<Timer>) -> Ptr<Timer> {
    e.set(this, Timer::pPrev, 0);
    e.set(this, Timer::pNext, 0);
    e.set(this, Timer::fElapsed, 0.0);
    let default_end: f32 = e.global(TIMER_DEFAULT_END);
    e.set(this, Timer::fEnd, default_end);
    e.set(this, Timer::pIndex, 0);
    this
}

// Translated from 0070a8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the table embedded at `+0x4d4` of the manager: runs the
/// table initializer `00706b90` on it and returns it.
pub fn fn_0070a8e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(KEY_TABLE_CONSTRUCT, &args![this]);
    this
}

// Translated from 0070a900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the byte at `+0x32` of the object (called on the
/// interface shader accumulator).
pub fn fn_0070a900(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x32, value);
}

// Translated from 0070a920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::~InterfaceManager` (Xbox PDB): deletes the target
/// reticle, shuts down the helper singletons, detaches both scene graphs,
/// deletes the root tiles, the cursor, the view caster, the pipboy and the
/// shadow nodes, clears the singleton and finally destroys the embedded
/// members in reverse order. The compiler's unwinding states are not
/// translated.
pub fn interface_manager_destruct(e: &mut Engine, this: Ptr<InterfaceManager>) {
    let base = this.addr();
    let reticle = e.get(this, InterfaceManager::pTargetReticle);
    if reticle != 0 {
        delete_virtual(e, reticle);
    }
    e.set(this, InterfaceManager::pTargetReticle, 0);
    fn_0070ad30(e);
    fn_0070acb0(e);
    e.call(DEBUG_TEXT_SHUTDOWN, &args![]);
    e.call(FONT_MANAGER_SHUTDOWN, &args![]);
    for member in [base + 0x4, base + 0x8] {
        if ni_pointer_get(e, member) != 0 {
            let graph = ni_pointer_get(e, member);
            e.call(EMPTY_MEMBER_FUNCTION, &args![graph]);
            e.call(NI_POINTER_ASSIGN, &args![member, 0u32]);
        }
    }
    e.set_global(TILE_DELETING, 1u8);
    let string_root = e.get(this, InterfaceManager::pStringRoot);
    if string_root != 0 {
        delete_virtual(e, string_root);
    }
    let menus_root = e.get(this, InterfaceManager::pMenusRoot);
    if menus_root != 0 {
        delete_virtual(e, menus_root);
    }
    e.set_global(TILE_DELETING, 0u8);
    e.call(MENU_MANAGER_SHUTDOWN, &args![]);
    let cursor = e.get(this, InterfaceManager::pCursor);
    if cursor != 0 {
        delete_virtual(e, cursor);
    }
    e.call(NI_POINTER_ASSIGN, &args![base + 0xac, 0u32]);
    e.set_global(MANAGER_SINGLETON, 0u32);
    e.call(TILE_FREE_TRAIT_LIST, &args![]);
    fn_0070aca0(e);
    let view_caster = e.get(this, InterfaceManager::pViewCaster);
    if view_caster != 0 {
        fn_0070ad80(e, Ptr::new(view_caster), 1);
    }
    e.call(FREE_TIMERS, &args![this]);
    e.call(MENU_FREE_XML_ARCHIVE, &args![]);
    e.call(LIST_CLEAR, &args![LIST_AT_011D8B54]);
    e.call(STATIC_OBJECT_SHUTDOWN_A, &args![]);
    e.call(MESSAGE_MENU_SHUTDOWN, &args![]);
    e.call(CLEAR_TEMP_MODEL, &args![]);
    let pipboy = e.get(this, InterfaceManager::pPipboy);
    if pipboy != 0 {
        delete_virtual(e, pipboy);
    }
    e.call(NI_POINTER_ASSIGN, &args![base + 0x8c, 0u32]);
    let shadow = e.get(this, InterfaceManager::pShadowNode);
    if shadow != 0 {
        delete_virtual(e, shadow);
    }
    let ui_player = e.get(this, InterfaceManager::pUIPlayerNode);
    if ui_player != 0 {
        delete_virtual(e, ui_player);
    }
    e.call(START_MENU_SHUTDOWN, &args![]);
    e.call(STATIC_OBJECT_SHUTDOWN_B, &args![]);
    e.call(EMPTY_MEMBER_FUNCTION, &args![base + 0x4d4]);
    e.call(SIMPLE_LIST_DESTROY, &args![base + 0x4c4]);
    e.call(VATS_EFFECT_MANAGER_DESTROY, &args![base + 0x1dc]);
    e.call(HUD_EFFECT_MANAGER_DESTROY, &args![base + 0x178]);
    e.call(NI_POINTER_DESTROY, &args![base + 0xac]);
    e.call(NI_POINTER_DESTROY, &args![base + 0x90]);
    e.call(NI_POINTER_DESTROY, &args![base + 0x8c]);
    e.call(SIMPLE_LIST_DESTROY, &args![base + 0x70]);
    e.call(NI_POINTER_DESTROY, &args![base + 0x8]);
    e.call(NI_POINTER_DESTROY, &args![base + 0x4]);
}

// Translated from 0070aca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the `NiTPointerList<unsigned int>` at `011f3324`
/// (`NiTPointerListBase<NiTPointerAllocator<unsigned_int>_Script_P>::RemoveAll`,
/// the engine map's name for `004ed900`).
pub fn fn_0070aca0(e: &mut Engine) {
    e.call(POINTER_LIST_REMOVE_ALL, &args![SCRIPT_POINTER_LIST]);
}

// Translated from 0070acb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the object the global at `011d8ce8` points to (through
/// `0070ad00`) and clears the global.
pub fn fn_0070acb0(e: &mut Engine) {
    let object = e.global::<u32>(GLOBAL_OBJECT_8CE8);
    if object != 0 {
        fn_0070ad00(e, Ptr::new(object), 1);
    }
    e.set_global(GLOBAL_OBJECT_8CE8, 0u32);
}

// Translated from 0070ad00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor of the object `fn_0070acb0` owns: runs `0071b040`
/// on it and frees it when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0070ad00(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(OBJECT_8CE8_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0070ad30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes (virtual destructor, flag 1) the object the global at
/// `011da0c4` points to and clears the global.
pub fn fn_0070ad30(e: &mut Engine) {
    let object = e.global::<u32>(GLOBAL_OBJECT_A0C4);
    if object != 0 {
        delete_virtual(e, object);
    }
    e.set_global(GLOBAL_OBJECT_A0C4, 0u32);
}

// Translated from 0070ad80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor of the view caster (`0x0c` bytes): runs `00631a50`
/// on it and frees it when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0070ad80(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(VIEW_CASTER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0070adb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::Init` (Xbox PDB): first initialization. Optionally
/// loads the text table, builds the root tile (`MenuRoot`) with the screen
/// metrics as traits, reads `globals.xml`, creates the debug text root and
/// the pipboy manager, and corrects the line limit for 16:10 screens.
/// `load_text_table` is the byte argument (non-zero also creates the
/// loading menu at the end).
pub fn interface_manager_init(e: &mut Engine, this: Ptr<InterfaceManager>, load_text_table: u8) {
    with_scope_guard(e, 0x21a, |e| {
        if load_text_table != 0 {
            e.call(TILE_LOAD_TEXT_TABLE, &args![]);
        }
        let root = construct_new(e, OPERATOR_NEW, 0x3c, |e, block| {
            e.call(MENU_ROOT_TILE_CONSTRUCT, &args![block]).ptr()
        });
        e.set(this, InterfaceManager::pMenusRoot, root.addr());
        let root = e.get(this, InterfaceManager::pMenusRoot);
        // `Tile::Init` (virtual slot 4): no parent, the name, flag 0.
        e.vcall(root, 4, &args![0u32, MENU_ROOT_NAME, 0u32]);
        let root = e.get(this, InterfaceManager::pMenusRoot);
        tile_set_int(e, root, 0xfa8, 1);
        tile_set_int(e, root, 0x1771, 0x3eb);
        tile_set_float(e, root, 0xfa9, 0.0);
        for (trait_id, getter) in [
            (0xfb1u32, GET_SCREEN_WIDTH),
            (0xfb0, GET_SCREEN_HEIGHT),
            (0xfc0, GET_INTERFACE_MARGIN_A),
            (0xfbf, GET_INTERFACE_MARGIN_B),
            (0xff8, GET_INTERFACE_SCALE),
        ] {
            let value = e.call(getter, &args![]).f32();
            let root = e.get(this, InterfaceManager::pMenusRoot);
            tile_set_float(e, root, trait_id, value);
        }
        let root = e.get(this, InterfaceManager::pMenusRoot);
        e.call(TILE_ADD_DIRTY_TILE, &args![root]);
        e.call(TILE_UPDATE_ALL, &args![0u32]);
        e.call(SYSTEM_COLOR_MANAGER_GET_INSTANCE, &args![]);
        let root = e.get(this, InterfaceManager::pMenusRoot);
        let globals = e.call(TILE_READ_FILE, &args![root, GLOBALS_XML_PATH]).u32();
        e.set(this, InterfaceManager::pStringRoot, globals);
        e.call(MENU_REMOVE_FROM_XML_ARCHIVE, &args![GLOBALS_XML_PATH, 0u32]);
        let globals = e.get(this, InterfaceManager::pStringRoot);
        e.call(TILE_SET_PARENT, &args![globals, 0u32, 0u32]);

        // `_background_fill_alpha` = (value * 255) as a float, from the
        // float the holder at `011d3174` keeps.
        let holder = e
            .call(FLOAT_HOLDER_GET, &args![FLOAT_HOLDER_BACKGROUND])
            .u32();
        let fraction: f32 = e.mem.f32(holder);
        let scale: f64 = e.global(TWO_FIFTY_FIVE);
        let alpha = (fraction as f64 * scale) as f32;
        let trait_id = e
            .call(TILE_TEXT_TO_TRAIT, &args![BACKGROUND_FILL_ALPHA_NAME])
            .u32();
        let globals = e.get(this, InterfaceManager::pStringRoot);
        tile_set_float(e, globals, trait_id, alpha);

        let has_controller = e.call(HAS_360_CONTROLLER_GETTER, &args![]).u8() as u32;
        let trait_id = e
            .call(TILE_TEXT_TO_TRAIT, &args![HAS_360_CONTROLLER_NAME])
            .u32();
        let globals = e.get(this, InterfaceManager::pStringRoot);
        tile_set_int(e, globals, trait_id, has_controller);

        let debug_root = construct_new(e, NI_ALLOC, 0xac, |e, block| {
            e.call(NI_NODE_CONSTRUCT, &args![block, 0u32]).ptr()
        });
        e.set(this, InterfaceManager::pDebugTextRoot, debug_root.addr());
        e.with_stack(4, |e, name| {
            let handle = e
                .call(FIXED_STRING_CONSTRUCT, &args![name, DEBUG_TEXT_ROOT_NAME])
                .u32();
            let debug_root = e.get(this, InterfaceManager::pDebugTextRoot);
            e.call(NODE_SET_NAME, &args![debug_root, handle]);
            e.call(FIXED_STRING_DESTROY, &args![name]);
        });
        let debug_root = e.get(this, InterfaceManager::pDebugTextRoot);
        e.call(NODE_SET_FLAG, &args![debug_root, 0u32]);
        // The interface root attaches the debug text root (virtual slot
        // `0xdc`, second argument 1).
        let interface_root = e.get(this, InterfaceManager::pInterfaceRoot);
        let debug_root = e.get(this, InterfaceManager::pDebugTextRoot);
        e.vcall(interface_root, 0xdc, &args![debug_root, 1u32]);
        let node = e.call(GET_FRAME_SCENE_NODE, &args![this]).u32();
        e.call(NODE_UPDATE_PROPERTIES, &args![node]);
        e.with_stack(0xc, |e, update_data| {
            e.call(
                NI_UPDATE_DATA_CONSTRUCT,
                &args![update_data, 0.0f32, 0u32, 0u32],
            );
            e.call(NODE_UPDATE, &args![node, update_data]);
        });

        // 16:10 (and its inverse 10:16 portrait) screens get a smaller
        // limit on two string lists.
        let width = e.call(GET_DESKTOP_WIDTH, &args![]).f64();
        let height = e.call(GET_DESKTOP_HEIGHT, &args![]).f64();
        let ratio = (width / height) as f32;
        let widescreen: f64 = e.global(WIDESCREEN_RATIO);
        let inverse: f64 = e.global(WIDESCREEN_INVERSE_RATIO);
        if ratio as f64 == widescreen || 1.0 / ratio as f64 == inverse {
            e.call(STRING_LIST_SET_LIMIT, &args![STRING_LIST_A, 0x50u32]);
            e.call(STRING_LIST_SET_LIMIT, &args![STRING_LIST_B, 0x50u32]);
        }

        let fog = construct_new(e, NI_ALLOC, 0x64, |e, block| {
            e.call(FOG_PROPERTY_CONSTRUCT, &args![block]).ptr()
        });
        e.with_stack(4, |e, held| {
            e.call(NI_POINTER_CONSTRUCT, &args![held, fog]);
            let fog = ni_pointer_get(e, held.addr());
            e.call(SET_FOG_RANGE, &args![fog, 0.0f32, 0.0f32]);
            e.call(GET_SECOND_SCENE_NODE, &args![this]);
            e.call(SET_DEBUG_TEXT_VISIBLE, &args![this, 0u32]);
            let pipboy = construct_new(e, OPERATOR_NEW, 0x170, |e, block| {
                e.call(PIPBOY_MANAGER_CONSTRUCT, &args![block]).ptr()
            });
            e.set(this, InterfaceManager::pPipboy, pipboy.addr());
            e.set(this, InterfaceManager::bFirstInit, 1);
            if load_text_table != 0 {
                e.call(LOADING_MENU_CREATE, &args![0u32, 0u32]);
            }
            e.call(NI_POINTER_DESTROY, &args![held]);
        });
    });
}

// Translated from 0070b240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::InitSecond` (Xbox PDB): second initialization. Starts
/// the font manager, creates the cursor tile (`Cursor`, 32x32) and its
/// node orientation, the two shadow scene nodes and their accumulators,
/// creates the HUD main menu and marks the manager initialized.
pub fn interface_manager_init_second(e: &mut Engine, this: Ptr<InterfaceManager>) {
    with_scope_guard(e, 0x29b, |e| {
        e.call(FONT_MANAGER_INIT, &args![]);
        let cursor = construct_new(e, OPERATOR_NEW, 0x48, |e, block| {
            tile_image_construct(e, block.cast()).cast()
        })
        .addr();
        // `Tile::Init` (virtual slot 4): no parent, the name, flag 0.
        e.vcall(cursor, 4, &args![0u32, CURSOR_NAME, 0u32]);
        tile_set_int(e, cursor, 0xfad, 2000);
        tile_set_int(e, cursor, 0xfb1, 0x20);
        tile_set_int(e, cursor, 0xfb0, 0x20);
        for (trait_id, holder) in [(0xfccu32, STRING_HOLDER_A), (0xff9, STRING_HOLDER_B)] {
            let string = e.call(STRING_HOLDER_GET, &args![holder]).u32();
            e.call(TILE_SET_STRING, &args![cursor, trait_id, string, 1u32]);
        }
        for trait_id in [0xfa1u32, 0xfa2, 0xfa3] {
            tile_set_int(e, cursor, trait_id, 0);
        }
        for trait_id in [0xfa9u32, 0xfb2, 0xfb3, 0xfb4] {
            tile_set_int(e, cursor, trait_id, 0xff);
        }
        e.call(TILE_UPDATE_TILE, &args![cursor, 1u32]);
        let node = e.call(TILE_IMAGE_NODE, &args![cursor]).u32();
        let cursor_root = e.get(this, InterfaceManager::pCursorRoot);
        e.vcall(cursor_root, 0xdc, &args![node, 1u32]);
        let node = e.call(TILE_IMAGE_NODE, &args![cursor]).u32();
        let direction_y: f32 = e.global(CURSOR_DIRECTION_Y);
        e.call(
            NODE_SET_TRANSLATE,
            &args![node, 0.0f32, direction_y, 0.0f32],
        );
        let node = e.call(TILE_IMAGE_NODE, &args![cursor]).u32();
        e.call(NODE_UPDATE_PROPERTIES, &args![node]);
        e.with_stack(0xc, |e, update_data| {
            e.call(
                NI_UPDATE_DATA_CONSTRUCT,
                &args![update_data, 0.0f32, 0u32, 0u32],
            );
            let node = e.call(TILE_IMAGE_NODE, &args![cursor]).u32();
            e.call(NODE_UPDATE, &args![node, update_data]);
        });
        e.set(this, InterfaceManager::pCursor, cursor);
        fn_007118d0(e, this);
        let cursor = e.get(this, InterfaceManager::pCursor);
        let node = e.call(TILE_IMAGE_NODE, &args![cursor]).u32();
        e.call(NODE_SET_FLAG, &args![node, 1u32]);

        let shadow = construct_new(e, NI_ALLOC, 0x200, |e, block| {
            e.call(SHADOW_SCENE_NODE_CONSTRUCT, &args![block]).ptr()
        });
        e.set(this, InterfaceManager::pShadowNode, shadow.addr());
        e.call(SET_SHADOW_SCENE_NODE, &args![shadow, 1u32]);
        let shadow = construct_new(e, NI_ALLOC, 0x200, |e, block| {
            e.call(SHADOW_SCENE_NODE_CONSTRUCT, &args![block]).ptr()
        });
        e.set(this, InterfaceManager::pUIPlayerNode, shadow.addr());
        e.call(SET_SHADOW_SCENE_NODE, &args![shadow, 3u32]);
        // Both accumulators are given the first shadow node (setter
        // `+0x194`).
        for member in [0x8cu32, 0x90] {
            let shadow_node = e.get(this, InterfaceManager::pShadowNode);
            let accumulator = ni_pointer_get(e, this.addr() + member);
            e.call(SHADER_ACCUMULATOR_SET_194, &args![accumulator, shadow_node]);
        }
        let hud = e.call(HUD_MAIN_MENU_CREATE, &args![]).u32();
        e.call(HUD_MAIN_MENU_SET_MENU_MODE, &args![4u32]);
        if hud != 0 {
            tile_set_int(e, hud, 0xfa3, 0);
        }
        let cursor = e.get(this, InterfaceManager::pCursor);
        let node = e.call(TILE_IMAGE_NODE, &args![cursor]).u32();
        e.call(NODE_SET_FLAG, &args![node, 1u32]);
        let cursor = e.get(this, InterfaceManager::pCursor);
        tile_set_int(e, cursor, 0xfa3, 0);
        let cursor = e.get(this, InterfaceManager::pCursor);
        e.call(TILE_UPDATE_TILE, &args![cursor, 1u32]);
        e.set(this, InterfaceManager::bSecondInit, 1);
    });
}

// Translated from 0070b5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileImage::TileImage` (Xbox PDB): the `Tile` base constructor, the
/// `TileImage` vtable, empty texture and shader property pointers, scale
/// 1.0 and `bIsScissor` false. Returns `this`.
pub fn tile_image_construct(e: &mut Engine, this: Ptr<TileImage>) -> Ptr<TileImage> {
    e.call(TILE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), TILE_IMAGE_VTABLE);
    e.call(NI_POINTER_CONSTRUCT, &args![this.addr() + 0x3c, 0u32]);
    e.call(NI_POINTER_CONSTRUCT, &args![this.addr() + 0x40, 0u32]);
    e.set(this, TileImage::fScale, 1.0);
    e.call(NI_POINTER_ASSIGN, &args![this.addr() + 0x3c, 0u32]);
    e.set(this, TileImage::bIsScissor, 0);
    this
}

// Translated from 0070b670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileImage::GetType` (Xbox PDB): the tile type id.
pub fn tile_image_get_type(_e: &mut Engine, _this: Ptr<TileImage>) -> u32 {
    TILE_IMAGE_TYPE
}

// Translated from 0070b680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileImage::GetTypeName` (Xbox PDB): the address of the type name
/// string.
pub fn tile_image_get_type_name(_e: &mut Engine, _this: Ptr<TileImage>) -> u32 {
    TILE_IMAGE_TYPE_NAME
}

// Translated from 0070b690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileImage::__scalar_deleting_destructor_` (Xbox PDB): destroys the tile
/// and, when bit 0 of `flags` is set, frees it. Returns `this`.
pub fn tile_image_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<TileImage>,
    flags: u32,
) -> Ptr<TileImage> {
    tile_image_destruct(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0070b6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TileImage::~TileImage` (Xbox PDB): restores the `TileImage` vtable,
/// clears the shader property, calls `Tile::Release` unless the tile
/// flags say it was already released (bit `0x2000`), destroys the two
/// pointer members and runs `Tile::~Tile`.
pub fn tile_image_destruct(e: &mut Engine, this: Ptr<TileImage>) {
    e.mem.set_u32(this.addr(), TILE_IMAGE_VTABLE);
    e.call(NI_POINTER_ASSIGN, &args![this.addr() + 0x40, 0u32]);
    if !e.call(TILE_FLAG_TEST, &args![this]).bool() {
        e.call(TILE_RELEASE, &args![this]);
    }
    e.call(NI_POINTER_DESTROY, &args![this.addr() + 0x40]);
    e.call(NI_POINTER_DESTROY, &args![this.addr() + 0x3c]);
    e.call(TILE_DESTRUCT, &args![this]);
}

// Translated from 0070b8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::PreIdleStuff` (Xbox PDB): per-frame mode transitions
/// before the idle. Leaving mode 3 (menus fading out, not locked) clears
/// the keystrokes, enters mode 5, restores the cursor and the HUD, enters
/// the console and pauses sound and voice once more; mode 5 becomes 2.
/// While in mode 1 the pause counters follow whether the top menu is a
/// game-pausing one (`007023a0`).
pub fn interface_manager_pre_idle_stuff(e: &mut Engine, this: Ptr<InterfaceManager>) {
    with_scope_guard(e, 0x33a, |e| {
        let controls_owner = e.global::<u32>(CONTROLS_OWNER);
        let controls = e.call(CONTROLS_GET, &args![controls_owner]).u32();
        e.call(UPDATE_ALL_TIMERS, &args![this]);
        if e.get(this, InterfaceManager::cMenuMode) == 3
            && e.get(this, InterfaceManager::bLockMenuModeForFade) == 0
        {
            e.call(CLEAR_KEYSTROKES, &args![controls]);
            e.set(this, InterfaceManager::cMenuMode, 5);
            let has_controller = e.call(HAS_360_CONTROLLER_GETTER, &args![]).bool();
            if !has_controller || e.get(this, InterfaceManager::bShowMouse) != 0 {
                let cursor = e.get(this, InterfaceManager::pCursor);
                let node = e.call(TILE_IMAGE_NODE, &args![cursor]).u32();
                e.call(NODE_SET_FLAG, &args![node, 0u32]);
                let cursor = e.get(this, InterfaceManager::pCursor);
                tile_set_int(e, cursor, 0xfa3, 1);
            }
            let string = e.call(STRING_HOLDER_GET, &args![STRING_HOLDER_A]).u32();
            let cursor = e.get(this, InterfaceManager::pCursor);
            e.call(TILE_SET_STRING, &args![cursor, 0xfccu32, string, 1u32]);
            let cursor = e.get(this, InterfaceManager::pCursor);
            e.call(TILE_UPDATE_TILE, &args![cursor, 1u32]);
            e.call(HUD_SET_INFO_FOR_REF, &args![0u32, 0u32, 0u32]);
            e.set(this, InterfaceManager::pMouseOverTarget, 0);
            let has_controller = e.call(HAS_360_CONTROLLER_GETTER, &args![]).bool();
            if has_controller && e.get(this, InterfaceManager::bShowMouse) == 0 {
                e.set(this, InterfaceManager::bMouseInMotion, 0);
            }
            let console = e.call(MENU_CONSOLE_INSTANCE, &args![0u32]).u32();
            if console != 0 {
                e.call(MENU_CONSOLE_ON_ENTER_MENU_MODE, &args![console]);
            }
            pause_sound_and_voice(e);
        } else if e.get(this, InterfaceManager::cMenuMode) == 5 {
            e.set(this, InterfaceManager::cMenuMode, 2);
        }
        if e.get(this, InterfaceManager::cMenuMode) == 1 {
            if e.call(MENU_MODE_IS_NOT_ONE, &args![this]).bool() {
                if e.get(this, InterfaceManager::bExternalForcedMenuMode) == 0 {
                    e.set(this, InterfaceManager::bExternalForcedMenuMode, 1);
                    pause_sound_and_voice(e);
                }
            } else if e.get(this, InterfaceManager::bExternalForcedMenuMode) != 0 {
                e.set(this, InterfaceManager::bExternalForcedMenuMode, 0);
                unpause_voice_and_sound(e);
            }
        }
    });
}

/// Adds one to the sound pause counter (`+0x20` of the manager object that
/// `004b7210` returns) and pauses the audio, then the same for the voice
/// counter (`+0x21`).
fn pause_sound_and_voice(e: &mut Engine) {
    let manager = e.call(GET_MANAGER, &args![]).u32();
    let sound = e.mem.u8(manager + 0x20).wrapping_add(1);
    e.mem.set_u8(manager + 0x20, sound);
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    fn_0070bba0(e, Ptr::new(audio));
    let manager = e.call(GET_MANAGER, &args![]).u32();
    let voice = e.mem.u8(manager + 0x21).wrapping_add(1);
    e.mem.set_u8(manager + 0x21, voice);
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    e.call(AUDIO_PAUSE_TYPE_6, &args![audio]);
}

/// The inverse of [`pause_sound_and_voice`], each counter only when it is
/// not zero: voice first, then sound.
fn unpause_voice_and_sound(e: &mut Engine) {
    let manager = e.call(GET_MANAGER, &args![]).u32();
    if e.mem.u8(manager + 0x21) != 0 {
        let manager = e.call(GET_MANAGER, &args![]).u32();
        let voice = e.mem.u8(manager + 0x21).wrapping_sub(1);
        e.mem.set_u8(manager + 0x21, voice);
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        e.call(AUDIO_UNPAUSE_TYPE_6, &args![audio]);
    }
    let manager = e.call(GET_MANAGER, &args![]).u32();
    if e.mem.u8(manager + 0x20) != 0 {
        let manager = e.call(GET_MANAGER, &args![]).u32();
        let sound = e.mem.u8(manager + 0x20).wrapping_sub(1);
        e.mem.set_u8(manager + 0x20, sound);
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        e.call(AUDIO_UNPAUSE_TYPE_4000_0000, &args![audio]);
    }
}

// Translated from 0070bba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pauses the audio sounds of type `0x40000000` (`00ad8510(this,
/// 0x40000000, 1)`); called on the audio instance.
pub fn fn_0070bba0(e: &mut Engine, this: Ptr) {
    e.call(AUDIO_PAUSE_TYPE, &args![this, 0x4000_0000u32, 1u32]);
}

// Translated from 0070c4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::Idle` (Xbox PDB): the per-frame interface update.
///
/// In order: reads the Xbox 360 controller and announces a change of
/// connection; shows or hides the cursor for the input device; updates the
/// HUD mode when the menu mode changed; in modes 2 and 5 (menus open or
/// opening) runs the mouse handling (pick, drag, enter/leave/click through
/// the menus, wheel), the gamepad events, the console and the menu keys;
/// then handles the console and start menu keys, the pipboy hot keys and
/// the tutorial manager. Nothing after the scope guard.
///
/// The three mouse-button queries `00a23a50(0, n)` (or `00a24660(5, n)` on
/// the console's own controls) are kept as `input_0`, `input_1`, `input_2`;
/// the code never names them.
pub fn interface_manager_idle(e: &mut Engine, this: Ptr<InterfaceManager>) {
    with_scope_guard(e, 0x47a, |e| {
        let mut state = IdleState::default();
        idle_controller_state(e, this);
        if e.get(this, InterfaceManager::bPreLoadMainMenus) != 0 {
            e.call(PRELOAD_MAIN_MENUS, &args![this]);
        }
        let controls_owner = e.global::<u32>(CONTROLS_OWNER);
        state.controls = e.call(CONTROLS_GET, &args![controls_owner]).u32();
        e.call(CONTROLS_QUERY_00A239E0, &args![state.controls, 1u32]);
        e.call(CONTROLS_QUERY_00A239E0, &args![state.controls, 2u32]);
        idle_cursor_visibility(e, this);
        state.menu_blocks_pointer = idle_pointer_blocked(e, this);
        let rendered_menu = e.get(this, InterfaceManager::pCurrentRenderedMenu);
        if rendered_menu != 0 {
            e.vcall(rendered_menu, 0x14, &args![]);
        }
        let pipboy = e.call(GET_PIPBOY, &args![this]).u32();
        if fn_0070ec50(e, Ptr::new(pipboy)) != 0 {
            let tick = e.call(FADE_CLOCK_READ, &args![FADE_CLOCK]).u32();
            let pipboy = e.call(GET_PIPBOY, &args![this]).u32();
            e.call(PIPBOY_UPDATE_LIGHT_EFFECT, &args![pipboy, tick as f32]);
        }
        e.call(HUD_EFFECTS_UPDATE, &args![this.addr() + 0x178]);
        e.call(VATS_EFFECTS_UPDATE, &args![this.addr() + 0x1dc]);
        idle_hud_mode(e, this);
        idle_gamepad_menu_key(e, this);
        if e.get(this, InterfaceManager::cMenuMode) == 1 {
            idle_game_mode(e, this);
        }
        let mode = e.get(this, InterfaceManager::cMenuMode);
        if mode == 2 || mode == 5 {
            idle_menu_mode(e, this, &mut state);
        }
        idle_console_and_hot_keys(e, this, &mut state);
    });
}

/// What `Idle` keeps in locals across its parts.
#[derive(Default)]
struct IdleState {
    /// The controls object (`00877720`).
    controls: u32,
    /// The console (`MenuConsole::Instance`), or 0.
    console: u32,
    /// True when a menu keeps the pointer from acting (`menu_blocks_pointer`).
    menu_blocks_pointer: bool,
    /// The three mouse-button queries.
    input_0: u32,
    input_1: u32,
    input_2: u32,
    /// The tile under the pointer and its menu.
    picked_tile: u32,
    picked_menu: u32,
}

/// Part of `Idle`: copies the previous controller state words, polls
/// `XInputGetState` and reports a change of the connected state.
fn idle_controller_state(e: &mut Engine, this: Ptr<InterfaceManager>) {
    for (from, to) in [
        (0x011d_8a6cu32, 0x011d_8a54u32),
        (0x011d_8a70, 0x011d_8a58),
        (0x011d_8a74, 0x011d_8a5c),
        (0x011d_8a78, 0x011d_8a60),
    ] {
        let word = e.mem.u32(from);
        e.mem.set_u32(to, word);
    }
    let result = e
        .call(XINPUT_GET_STATE, &args![0u32, CONTROLLER_STATE])
        .u32();
    e.set_global(HAS_360_CONTROLLER, (result == 0) as u8);
    // `static bool lastHasController` with its init guard.
    if e.global::<u32>(LAST_HAS_CONTROLLER_GUARD) & 1 == 0 {
        let guard: u32 = e.global(LAST_HAS_CONTROLLER_GUARD);
        e.set_global(LAST_HAS_CONTROLLER_GUARD, guard | 1);
        let now: u8 = e.global(HAS_360_CONTROLLER);
        e.set_global(LAST_HAS_CONTROLLER, now);
    }
    let last: u8 = e.global(LAST_HAS_CONTROLLER);
    let now: u8 = e.global(HAS_360_CONTROLLER);
    if last != now {
        // A message box about the change: the first string comes from the
        // holder at `011d38b8`, the second from one of two holders depending
        // on whether the controller was connected before (`last` is 1).
        let text = if last == 1 {
            TEXT_HOLDER_CONTROLLER_WAS_SET
        } else {
            TEXT_HOLDER_CONTROLLER_WAS_CLEAR
        };
        let title = e
            .call(STRING_HOLDER_GET, &args![MESSAGE_TITLE_HOLDER])
            .u32();
        let body = e.call(STRING_HOLDER_GET, &args![text]).u32();
        e.call(
            SHOW_MESSAGE_BOX,
            &args![body, 0u32, 0u32, 0u32, 0u32, 0x17u32, 0.0f32, 0.0f32, title, 0u32],
        );
        e.call(CONTROLLER_CHANGED, &args![this, now as u32]);
        let now: u8 = e.global(HAS_360_CONTROLLER);
        e.set_global(LAST_HAS_CONTROLLER, now);
    }
}

/// Part of `Idle`: with a controller and no console the cursor tile is
/// hidden (`bShowMouse` false); otherwise it is shown unless the manager is
/// in mode 1.
fn idle_cursor_visibility(e: &mut Engine, this: Ptr<InterfaceManager>) {
    if e.call(HAS_360_CONTROLLER_GETTER, &args![]).bool() {
        if !e.call(IS_CONSOLE_VISIBLE, &args![]).bool() {
            e.set(this, InterfaceManager::bShowMouse, 0);
            set_cursor_hidden(e, this, false);
        } else {
            e.set(this, InterfaceManager::bShowMouse, 1);
            set_cursor_hidden(e, this, true);
        }
    } else if e.get(this, InterfaceManager::cMenuMode) != 1 {
        e.set(this, InterfaceManager::bShowMouse, 1);
        set_cursor_hidden(e, this, true);
    }
}

/// The three calls that show or hide the cursor tile: its node flag
/// (`0` when hiding, `1` when showing) and trait `0xfa3` (`1` when hiding).
fn set_cursor_hidden(e: &mut Engine, this: Ptr<InterfaceManager>, hidden: bool) {
    let cursor = e.get(this, InterfaceManager::pCursor);
    let node = e.call(TILE_IMAGE_NODE, &args![cursor]).u32();
    e.call(NODE_SET_FLAG, &args![node, !hidden as u32]);
    let cursor = e.get(this, InterfaceManager::pCursor);
    tile_set_int(e, cursor, 0xfa3, hidden as u32);
}

/// Part of `Idle`: `menu_blocks_pointer`, true when the pipboy (or the menu the code
/// names) keeps the pointer from acting.
fn idle_pointer_blocked(e: &mut Engine, _this: Ptr<InterfaceManager>) -> bool {
    if !e.call(IS_PIPBOY_MENU_TOPMOST, &args![]).bool() {
        return true;
    }
    if e.call(HAS_360_CONTROLLER_GETTER, &args![]).bool()
        && (is_top_menu(e, 0x40b) || is_top_menu(e, 0x425))
    {
        return true;
    }
    if is_top_menu(e, 0x3ea) && inventory_menu_is_keyring_open(e) {
        return true;
    }
    if e.call(OBJECT_FLAG_0070ED80, &args![]).bool() && e.global::<u8>(HAS_360_CONTROLLER) != 0 {
        return true;
    }
    let class = e.call(PIPBOY_MENU_CLASS, &args![]).u32();
    e.call(TILE_GET_MENU_BY_CLASS, &args![class]).u32() != 0
}

/// `Interface::IsTopMenuID(id)` (Xbox PDB, `cdecl`).
fn is_top_menu(e: &mut Engine, id: u32) -> bool {
    e.call(IS_TOP_MENU_ID, &args![id]).bool()
}

/// Part of `Idle`: when the menu mode differs from the one last seen
/// (`0119f530`), pauses/unpauses the audio and picks the HUD mode for the
/// new menu mode.
fn idle_hud_mode(e: &mut Engine, this: Ptr<InterfaceManager>) {
    let mode = e.get(this, InterfaceManager::cMenuMode);
    if e.global::<u32>(LAST_MENU_MODE) == mode {
        return;
    }
    if mode == 1 {
        loop {
            let manager = e.call(GET_MANAGER, &args![]).u32();
            if e.mem.u8(manager + 0x21) == 0 {
                break;
            }
            let manager = e.call(GET_MANAGER, &args![]).u32();
            let voice = e.mem.u8(manager + 0x21).wrapping_sub(1);
            e.mem.set_u8(manager + 0x21, voice);
            let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
            e.call(AUDIO_UNPAUSE_TYPE_6, &args![audio]);
        }
        loop {
            let manager = e.call(GET_MANAGER, &args![]).u32();
            if e.mem.u8(manager + 0x20) == 0 {
                break;
            }
            let manager = e.call(GET_MANAGER, &args![]).u32();
            let sound = e.mem.u8(manager + 0x20).wrapping_sub(1);
            e.mem.set_u8(manager + 0x20, sound);
            let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
            e.call(AUDIO_UNPAUSE_TYPE_4000_0000, &args![audio]);
        }
        let player = e.global::<u32>(PLAYER_SINGLETON);
        let hud_mode = if e.vcall(player, 0x22c, &args![0u32]).bool() {
            7
        } else if e.vcall(player, 0x214, &args![]).u32() != 0 {
            1
        } else if e.call(GAME_STATE_ID, &args![]).u32() == 0x18 {
            // No HUD mode is set while this state id is current.
            0
        } else if e
            .call(CONDITION_CHECK_005A03F0, &args![player, 1u32])
            .bool()
        {
            0xc
        } else if e.call(PLAYER_FLAG_00950090, &args![player]).bool() {
            0x14
        } else {
            let iron_sights = e.call(ACTOR_GET_IRON_SIGHTS, &args![player]).bool();
            if iron_sights
                && e.call(WEAPON_STATE_GETTER, &args![WEAPON_STATE_OBJECT])
                    .u32()
                    == 0
            {
                0x16
            } else if e
                .call(WEAPON_STATE_GETTER, &args![WEAPON_STATE_OBJECT])
                .u32()
                == 4
            {
                8
            } else {
                2
            }
        };
        if hud_mode != 0 {
            e.call(HUD_SET_MENU_MODE, &args![hud_mode]);
        }
    } else if mode == 5 {
        let top = e.call(GET_ENTER_STACK_TOP, &args![this]).i32();
        let hud_mode = match top {
            0x3e9 => 0x11,
            1 | 0x13 | 0x3ea | 0x3eb | 0x3f6 | 0x3ff | 0x40b | 0x425 => 3,
            0x3ef => 5,
            0x3f0 => 9,
            0x3f1 => 6,
            0x403 | 0x418 => 0xe,
            0x41f => 0xf,
            0x420 => 7,
            0x421 => 0x10,
            0x424 => 0x12,
            0x432 => 0x13,
            0x438..=0x43b => 0x19,
            _ => 4,
        };
        e.call(HUD_SET_MENU_MODE, &args![hud_mode as u32]);
    }
    e.set_global(LAST_MENU_MODE, mode);
}

/// Part of `Idle`: when `00719ae0` reports true and none of the four
/// `00a24660(n, 0)` queries (n = 2, 3, 0, 1) is set, calls `00718930(0)`.
fn idle_gamepad_menu_key(e: &mut Engine, this: Ptr<InterfaceManager>) {
    if !e.call(QUERY_00719AE0, &args![this]).bool() {
        return;
    }
    for button in [2u32, 3, 0, 1] {
        let controls_owner = e.global::<u32>(CONTROLS_OWNER);
        let controls = e.call(CONTROLS_GET, &args![controls_owner]).u32();
        if e.call(CONTROLS_QUERY_00A24660, &args![controls, button, 0u32])
            .u32()
            != 0
        {
            return;
        }
    }
    e.call(ACTION_00718930, &args![this, 0u32]);
}

/// Part of `Idle` in menu mode 1 (the game): makes sure the HUD exists,
/// shows the pending cursor string, creates the level-up menu when queued
/// and lets the player cast the queued eat/drink and enchantment effects.
fn idle_game_mode(e: &mut Engine, this: Ptr<InterfaceManager>) {
    let hud = e.call(TILE_GET_MENU_BY_CLASS, &args![0x3ecu32]).u32();
    if hud == 0 {
        e.call(HUD_MAIN_MENU_CREATE, &args![]);
    }
    if e.global::<u8>(CURSOR_STRING_PENDING) != 0 {
        let string = e.call(STRING_HOLDER_GET, &args![STRING_HOLDER_A]).u32();
        let cursor = e.get(this, InterfaceManager::pCursor);
        e.call(TILE_SET_STRING, &args![cursor, 0xfccu32, string, 1u32]);
        let cursor = e.get(this, InterfaceManager::pCursor);
        e.call(TILE_UPDATE_TILE, &args![cursor, 1u32]);
        e.set_global(CURSOR_STRING_PENDING, 0u8);
    }
    if e.global::<u8>(LEVEL_UP_MENU_PENDING) != 0 {
        e.call(LEVEL_UP_MENU_CREATE, &args![]);
        e.set_global(LEVEL_UP_MENU_PENDING, 0u8);
    }
    let player = e.global::<u32>(PLAYER_SINGLETON);
    e.call(PLAYER_CAST_EAT_DRINK_ITEMS, &args![player]);
    let player = e.global::<u32>(PLAYER_SINGLETON);
    e.call(PLAYER_CAST_QUEUED_ENCHANTMENTS, &args![player]);
}

/// Part of `Idle` in menu modes 2 and 5: cursor and default focus, the
/// mouse buttons, picking and enter/leave/click/drag through the menu
/// virtuals, the wheel, the gamepad and the menu keys.
fn idle_menu_mode(e: &mut Engine, this: Ptr<InterfaceManager>, state: &mut IdleState) {
    idle_menu_cursor_and_focus(e, this);
    idle_read_mouse(e, this, state);
    if e.call(HAS_360_CONTROLLER_GETTER, &args![]).bool()
        || e.get(this, InterfaceManager::bMouseInMotion) != 0
    {
        idle_pick(e, this, state);
    }
    idle_menu_key_state(e, this, state);
    idle_click(e, this, state);
    idle_button_and_wheel(e, this, state);
    idle_gamepad_events(e, this, state);
}

/// `ftol(Tile::GetValue(tile, 0xfaa))`: the id of the tile's menu entry that
/// the menu virtuals take.
fn tile_menu_id(e: &mut Engine, tile: u32) -> i32 {
    let value = e.call(TILE_GET_VALUE, &args![tile, 0xfaau32]).f64();
    e.call(FTOL, &args![value]).i32()
}

fn pointer_shown(e: &mut Engine, this: Ptr<InterfaceManager>) -> bool {
    !e.call(HAS_360_CONTROLLER_GETTER, &args![]).bool()
        || e.get(this, InterfaceManager::bShowMouse) != 0
}

/// The menu state word at `+0x24` of a menu (`0059bb30`, a plain getter the
/// linker folded with `D3DTexture_LockRect`); 1 means the menu is active.
fn menu_is_active(e: &mut Engine, menu: u32) -> bool {
    e.call(MENU_STATE, &args![menu]).u32() == 1
}

/// Part of `idle_menu_mode`: with a controller and no console, shows the
/// cursor tile as hidden or shown; keeps the default focus when the pointer
/// is not in motion.
fn idle_menu_cursor_and_focus(e: &mut Engine, this: Ptr<InterfaceManager>) {
    let controller = e.call(HAS_360_CONTROLLER_GETTER, &args![]).bool();
    if controller && e.get(this, InterfaceManager::bShowMouse) == 0 {
        if !e.call(IS_CONSOLE_VISIBLE, &args![]).bool() {
            e.set(this, InterfaceManager::bMouseInMotion, 0);
        }
    } else {
        let loading = e.call(IS_IN_GAME_LOADING_MENU_OPEN, &args![]).bool();
        if loading && !e.call(XUI_IS_UP, &args![]).bool() {
            let cursor = e.get(this, InterfaceManager::pCursor);
            let node = e.call(TILE_IMAGE_NODE, &args![cursor]).u32();
            e.call(NODE_SET_FLAG, &args![node, 1u32]);
            let cursor = e.get(this, InterfaceManager::pCursor);
            tile_set_int(e, cursor, 0xfa3, 0);
        } else {
            let cursor = e.get(this, InterfaceManager::pCursor);
            let node = e.call(TILE_IMAGE_NODE, &args![cursor]).u32();
            e.call(NODE_SET_FLAG, &args![node, 0u32]);
            let cursor = e.get(this, InterfaceManager::pCursor);
            tile_set_int(e, cursor, 0xfa3, 1);
        }
    }
    fn_007118d0(e, this);
    if e.get(this, InterfaceManager::bMouseInMotion) == 0 {
        let over = e.call(GET_MOUSE_OVER_TARGET, &args![this]).u32();
        let keep_focus = over != 0 && {
            let over = e.call(GET_MOUSE_OVER_TARGET, &args![this]).u32();
            e.call(TILE_IS_VISIBLE, &args![over]).bool() && {
                let over = e.call(GET_MOUSE_OVER_TARGET, &args![this]).u32();
                e.call(TILE_IS_TRUE, &args![over, 0xfafu32]).bool()
            }
        };
        if !keep_focus {
            e.call(GET_DEFAULT_FOCUS, &args![this]);
        }
    }
}

/// Part of `idle_menu_mode`: reads the three mouse-button queries (from the
/// console's own controls when it is visible and the pointer is hidden) and
/// accumulates the held time; stores the wheel as a float.
fn idle_read_mouse(e: &mut Engine, this: Ptr<InterfaceManager>, state: &mut IdleState) {
    state.input_1 = 0;
    state.input_2 = 0;
    state.input_0 = 0;
    let controller = e.call(HAS_360_CONTROLLER_GETTER, &args![]).bool();
    if !controller || e.get(this, InterfaceManager::bShowMouse) != 0 {
        state.input_1 = e
            .call(CONTROLS_QUERY_00A23A50, &args![state.controls, 0u32, 1u32])
            .u32();
        state.input_2 = e
            .call(CONTROLS_QUERY_00A23A50, &args![state.controls, 0u32, 2u32])
            .u32();
        state.input_0 = e
            .call(CONTROLS_QUERY_00A23A50, &args![state.controls, 0u32, 0u32])
            .u32();
        accumulate_held_time(e, this, state.input_0);
    } else {
        let console = e.call(MENU_CONSOLE_INSTANCE, &args![1u32]).u32();
        if e.call(CONSOLE_PRIMARY_DOWN, &args![console]).bool() {
            state.input_1 = e
                .call(CONTROLS_QUERY_00A24660, &args![state.controls, 5u32, 1u32])
                .u32();
            state.input_2 = e
                .call(CONTROLS_QUERY_00A24660, &args![state.controls, 5u32, 2u32])
                .u32();
            state.input_0 = e
                .call(CONTROLS_QUERY_00A24660, &args![state.controls, 5u32, 0u32])
                .u32();
            accumulate_held_time(e, this, state.input_0);
        }
    }
    let wheel = e
        .call(CONTROLS_QUERY_00A239E0, &args![state.controls, 3u32])
        .i32();
    e.set(this, InterfaceManager::fMouseWheel, wheel as f32);
}

/// `fMouseHeldTime += frame time` while the button is down, 0 otherwise.
fn accumulate_held_time(e: &mut Engine, this: Ptr<InterfaceManager>, held: u32) {
    if held == 0 {
        e.set(this, InterfaceManager::fMouseHeldTime, 0.0);
    } else {
        let frame_time = e.call(FRAME_TIME_GETTER, &args![FADE_CLOCK]).f64();
        let total = frame_time + e.get(this, InterfaceManager::fMouseHeldTime) as f64;
        e.set(this, InterfaceManager::fMouseHeldTime, total as f32);
    }
}

/// The pointer position the drag code works in: the cursor fields
/// `+0x38`/`+0x40` divided by the screen scale (`0070ecb0`), as the
/// `NiPoint2` the code builds from them.
fn pointer_scaled(e: &mut Engine, this: Ptr<InterfaceManager>) -> (f32, f32) {
    let scale = e.call(GET_SCREEN_SCALE, &args![]).f64();
    let y = (e.mem.f32(this.addr() + 0x40) as f64 / scale) as f32;
    let scale = e.call(GET_SCREEN_SCALE, &args![]).f64();
    let x = (e.mem.f32(this.addr() + 0x38) as f64 / scale) as f32;
    e.with_stack(8, |e, point| {
        e.call(NI_POINT2_CONSTRUCT, &args![point, x, y]);
        (e.mem.f32(point.addr()), e.mem.f32(point.addr() + 4))
    })
}

/// With the pipboy menu topmost the point is in pipboy coordinates:
/// `value / menu height - 195`.
fn pointer_to_pipboy_space(
    e: &mut Engine,
    _this: Ptr<InterfaceManager>,
    point: (f32, f32),
) -> (f32, f32) {
    let (mut x, mut y) = point;
    if e.call(IS_PIPBOY_MENU_TOPMOST, &args![]).bool() {
        let margin: f64 = e.global(PIPBOY_MARGIN);
        let height = e.call(GET_MENU_HEIGHT, &args![]).f64();
        x = (x as f64 / height - margin) as f32;
        let height = e.call(GET_MENU_HEIGHT, &args![]).f64();
        y = (y as f64 / height - margin) as f32;
    }
    (x, y)
}

/// Part of `idle_menu_mode`, when the pointer is in motion (or a
/// controller is attached): finds the tile under the pointer, updates the
/// drag of the dragged tile, and runs drag-over, release, enter and leave
/// on the menu virtuals.
fn idle_pick(e: &mut Engine, this: Ptr<InterfaceManager>, state: &mut IdleState) {
    state.picked_tile = e.call(PICK_TILE, &args![this, 0u32]).u32();
    state.picked_menu = 0;
    if state.picked_tile != 0 {
        state.picked_menu = e.call(TILE_GET_MENU, &args![state.picked_tile]).u32();
        let over = e.get(this, InterfaceManager::pMouseOverTarget);
        if pointer_shown(e, this) && over != 0 {
            tile_set_int(e, over, 0xfc3, 0);
            let target = e.get(this, InterfaceManager::pOverTileTarget);
            let id = tile_menu_id(e, over);
            let menu = e.call(TILE_GET_MENU, &args![over]).u32();
            e.call(DO_LEAVE, &args![this, menu, id, target]);
            e.set(this, InterfaceManager::pMouseOverTarget, 0);
        }
    }
    let drag = e.get(this, InterfaceManager::pDragTarget);
    if pointer_shown(e, this) && drag != 0 {
        let point = pointer_scaled(e, this);
        let (x, y) = pointer_to_pipboy_space(e, this, point);
        let drag = e.get(this, InterfaceManager::pDragTarget);
        let offset = tile_get_float(e, drag, 0xfea);
        let offset_x = e.call(FTOL, &args![x as f64 - offset]).i32();
        e.call(SET_CURSOR_OFFSET_X, &args![this, offset_x]);
        let offset = tile_get_float(e, drag, 0xfeb);
        let offset_y = e.call(FTOL, &args![y as f64 - offset]).i32();
        e.call(SET_CURSOR_OFFSET_Y, &args![this, offset_y]);
        let tile_x = e.call(TILE_GET_POSITION_X, &args![drag]).f64();
        let origin_x = tile_get_float(e, drag, 0xfa1);
        tile_set_float(e, drag, 0xff0, (x as f64 - (tile_x - origin_x)) as f32);
        let tile_y = e.call(TILE_GET_POSITION_Y, &args![drag]).f64();
        let origin_y = tile_get_float(e, drag, 0xfa2);
        tile_set_float(e, drag, 0xff1, (y as f64 - (tile_y - origin_y)) as f32);
    }
    let current = e.get(this, InterfaceManager::pOverTileTarget);
    if state.picked_tile != current && state.input_0 != 0 {
        // Dragging over another tile: the old menu hears the drag leave
        // (slot 0x1c), the new one the drag enter (slot 0x18).
        let drag_menu = e.get(this, InterfaceManager::pDragOverTileMenu);
        if drag_menu != 0 && menu_is_active(e, drag_menu) {
            let drag_tile = e.get(this, InterfaceManager::pDragOverTileTarget);
            let id = tile_menu_id(e, drag_tile);
            let current = e.get(this, InterfaceManager::pOverTileTarget);
            let drag_menu = e.get(this, InterfaceManager::pDragOverTileMenu);
            e.vcall(drag_menu, 0x1c, &args![id, drag_tile, current]);
        }
        e.set(
            this,
            InterfaceManager::pDragOverTileTarget,
            state.picked_tile,
        );
        e.set(this, InterfaceManager::pDragOverTileMenu, state.picked_menu);
        let drag_menu = e.get(this, InterfaceManager::pDragOverTileMenu);
        if drag_menu != 0 && menu_is_active(e, drag_menu) {
            let drag_tile = e.get(this, InterfaceManager::pDragOverTileTarget);
            let id = tile_menu_id(e, drag_tile);
            let current = e.get(this, InterfaceManager::pOverTileTarget);
            let drag_menu = e.get(this, InterfaceManager::pDragOverTileMenu);
            e.vcall(drag_menu, 0x18, &args![id, drag_tile, current]);
        }
    }
    if state.input_2 != 0 {
        idle_release(e, this, state);
    }
    idle_enter_leave(e, this, state);
}

/// `Tile::GetValue(tile, trait)` as the `double` it is read as in `ST0`.
fn tile_get_float(e: &mut Engine, tile: u32, trait_id: u32) -> f64 {
    e.call(TILE_GET_VALUE, &args![tile, trait_id]).f64()
}

/// The release of the button: the menu under the pointer is told (slot
/// `0xc`) and the tile's click trait pulses, when the press began on the
/// tile that is still under the pointer; the drag ends.
fn idle_release(e: &mut Engine, this: Ptr<InterfaceManager>, state: &mut IdleState) {
    let menu = e.get(this, InterfaceManager::pOverTileMenu);
    let tile = e.get(this, InterfaceManager::pOverTileTarget);
    let pressed = e.get(this, InterfaceManager::pDragOverTileTarget);
    if menu != 0
        && pressed == tile
        && e.call(TILE_GET_VALUE_Q, &args![tile, 0xfaau32]).u32() != 0
        && menu_is_active(e, menu)
    {
        e.call(TILE_PLAY_TILE_SOUND, &args![tile, 0xfcbu32]);
        let tile = e.get(this, InterfaceManager::pOverTileTarget);
        let menu = e.get(this, InterfaceManager::pOverTileMenu);
        if tile != 0 {
            tile_set_int(e, tile, 0xfc7, 1);
            tile_set_int(e, tile, 0xfc7, 0);
            if menu != 0 {
                let id = tile_menu_id(e, tile);
                e.vcall(menu, 0xc, &args![id, tile]);
            }
        }
        if e.get(this, InterfaceManager::pOverTileTarget) == 0 {
            state.picked_tile = 0;
            state.picked_menu = 0;
        }
    }
    e.set(this, InterfaceManager::pDragOverTileTarget, 0);
    e.set(this, InterfaceManager::pDragOverTileMenu, 0);
    let drag = e.get(this, InterfaceManager::pDragTarget);
    if drag != 0 {
        tile_set_int(e, drag, 0xfea, -1i32 as u32);
        let drag = e.get(this, InterfaceManager::pDragTarget);
        tile_set_int(e, drag, 0xfeb, -1i32 as u32);
        e.set(this, InterfaceManager::pDragTarget, 0);
    }
}

/// The pointer moved to another tile with no button down: leaves the old
/// tile and enters the new one (`DoLeave`, `DoEnter`, the menu virtuals).
fn idle_enter_leave(e: &mut Engine, this: Ptr<InterfaceManager>, state: &mut IdleState) {
    let current = e.get(this, InterfaceManager::pOverTileTarget);
    if state.picked_tile == current || state.input_0 != 0 || !pointer_shown(e, this) {
        return;
    }
    let menu = e.get(this, InterfaceManager::pOverTileMenu);
    if menu != 0 && menu_is_active(e, menu) {
        let tile = e.get(this, InterfaceManager::pOverTileTarget);
        tile_set_int(e, tile, 0xfc3, 0);
        // Is the picked tile a child of the tile being left?
        let found = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), state.picked_tile);
            e.call(TILE_CHILD_LIST_FIND, &args![tile + 4, slot, 0u32])
                .u32()
        });
        let tile = e.get(this, InterfaceManager::pOverTileTarget);
        let id = tile_menu_id(e, tile);
        let menu = e.get(this, InterfaceManager::pOverTileMenu);
        if found != 0 {
            e.vcall(menu, 0x14, &args![id, tile]);
        } else {
            e.call(DO_LEAVE, &args![this, menu, id, tile]);
        }
    }
    if state.picked_menu != 0 && menu_is_active(e, state.picked_menu) {
        e.set(this, InterfaceManager::pOverTileTarget, state.picked_tile);
        e.set(this, InterfaceManager::pOverTileMenu, state.picked_menu);
    } else {
        e.set(this, InterfaceManager::pOverTileTarget, 0);
        e.set(this, InterfaceManager::pOverTileMenu, 0);
    }
    let menu = e.get(this, InterfaceManager::pOverTileMenu);
    if menu != 0 && menu_is_active(e, menu) {
        if pointer_shown(e, this) {
            let tile = e.get(this, InterfaceManager::pOverTileTarget);
            e.set(this, InterfaceManager::pMouseOverTarget, tile);
        }
        let tile = e.get(this, InterfaceManager::pOverTileTarget);
        e.call(TILE_PLAY_TILE_SOUND, &args![tile, 0xfe8u32]);
        let tile = e.get(this, InterfaceManager::pOverTileTarget);
        tile_set_int(e, tile, 0xfc3, 1);
        let tile = e.get(this, InterfaceManager::pOverTileTarget);
        let id = tile_menu_id(e, tile);
        let menu = e.get(this, InterfaceManager::pOverTileMenu);
        e.call(DO_ENTER, &args![this, menu, id, tile]);
    }
    if e.get(this, InterfaceManager::bMouseInMotion) != 0 {
        e.call(CLEAR_MOUSE_OVER_TARGET, &args![this]);
    }
}

/// Part of `idle_menu_mode`, when the pointer is shown: keeps the flag at
/// `+0x4b4` and decides `bMouseOverRenderedMenu` (`+0x170`); when the
/// rendered/pipboy menu is unchanged and `+0x4b8` is set, plays the
/// `UIMenuMode` sound and opens the stats, inventory or map page.
fn idle_menu_key_state(e: &mut Engine, this: Ptr<InterfaceManager>, state: &mut IdleState) {
    if !pointer_shown(e, this) {
        return;
    }
    if e.call(RENDERED_MENU_OR_PIPBOY, &args![this]).u32() == 0 {
        e.mem.set_u8(this.addr() + 0x4b4, 0);
    }
    let current = e.call(RENDERED_MENU_OR_PIPBOY, &args![this]).u32();
    let pipboy = e.call(GET_PIPBOY, &args![this]).u32();
    let rendered = e.get(this, InterfaceManager::pCurrentRenderedMenu);
    if current == pipboy {
        let blocked = e.get(this, InterfaceManager::bMouseInMotion) == 0
            || rendered == 0
            || e.call(MENU_FLAG_QUERY_004A4040, &args![]).bool()
            || is_top_menu(e, 0x3e9)
            || is_top_menu(e, 0x41b)
            || e.mem.u8(this.addr() + 0x4b4) == 0;
        e.set(
            this,
            InterfaceManager::bMouseOverRenderedMenu,
            !blocked as u8,
        );
        if (e.mem.u32(this.addr() + 0x4b8) as i32) > 0 {
            idle_play_menu_mode_sound(e, this);
        }
    } else {
        let blocked = e.get(this, InterfaceManager::bMouseInMotion) == 0
            || rendered == 0
            || e.call(MENU_FLAG_QUERY_004A4040, &args![]).bool()
            || is_top_menu(e, 0x3e9)
            || is_top_menu(e, 0x41b)
            || is_top_menu(e, 0x423)
            || {
                let rendered = e.get(this, InterfaceManager::pCurrentRenderedMenu);
                let held = (state.input_2 as i32 > 0) as u32;
                !e.vcall(
                    rendered,
                    0x18,
                    &args![held, this.addr() + 0x4ac, this.addr() + 0x4b0],
                )
                .bool()
            };
        e.set(
            this,
            InterfaceManager::bMouseOverRenderedMenu,
            !blocked as u8,
        );
    }
}

/// Plays the `UIMenuMode` sound and opens the page `+0x4b8` names (1 stats,
/// 2 inventory, 3 map); any other value releases the sound handle.
fn idle_play_menu_mode_sound(e: &mut Engine, this: Ptr<InterfaceManager>) {
    e.with_stack(12, |e, handle| {
        e.call(SOUND_HANDLE_CONSTRUCT, &args![handle]);
        e.with_stack(12, |e, found| {
            let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
            let result = e
                .call(
                    AUDIO_GET_SOUND_HANDLE_BY_NAME,
                    &args![audio, found, UI_MENU_MODE_SOUND, 0x121u32],
                )
                .u32();
            e.call(SOUND_HANDLE_ASSIGN, &args![handle, result]);
            e.call(EMPTY_MEMBER_FUNCTION, &args![found]);
        });
        match e.mem.u32(this.addr() + 0x4b8) {
            1 => {
                e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
                e.call(SET_STATS_MENU_VISIBLE, &args![1u32, 0u32]);
            }
            2 => {
                e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
                e.call(SET_INVENTORY_MENU_VISIBLE, &args![1u32, 0u32, 1u32]);
            }
            3 => {
                e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
                e.call(SET_MAP_MENU_VISIBLE, &args![1u32, 0u32]);
            }
            _ => {
                e.call(SOUND_HANDLE_RELEASE, &args![handle]);
            }
        }
        e.call(EMPTY_MEMBER_FUNCTION, &args![handle]);
    });
}

/// Part of `idle_menu_mode`, on `input_1` with the pointer in motion: the
/// tile under the pointer is clicked (menu virtual slot 8); a draggable
/// tile (`0xfe9`) starts a drag. Without a menu under the pointer the
/// console's pick display is refreshed.
fn idle_click(e: &mut Engine, this: Ptr<InterfaceManager>, state: &mut IdleState) {
    if e.get(this, InterfaceManager::bMouseInMotion) == 0 || state.input_1 == 0 {
        return;
    }
    let tile = e.get(this, InterfaceManager::pOverTileTarget);
    let menu = e.get(this, InterfaceManager::pOverTileMenu);
    if tile != 0 && menu != 0 && menu_is_active(e, menu) {
        e.set(this, InterfaceManager::pDragOverTileTarget, tile);
        e.set(this, InterfaceManager::pDragOverTileMenu, menu);
        let id = tile_menu_id(e, tile);
        e.vcall(menu, 8, &args![id, tile]);
        let tile = e.get(this, InterfaceManager::pOverTileTarget);
        if tile != 0 && e.call(TILE_IS_TRUE, &args![tile, 0xfe9u32]).bool() {
            idle_start_drag(e, this, tile);
        }
    } else {
        let console = e.call(MENU_CONSOLE_INSTANCE, &args![1u32]).u32();
        if e.call(CONSOLE_PRIMARY_DOWN, &args![console]).bool() {
            fn_00710880(e, this);
            interface_manager_display_current_pick_ref(e, this);
        }
    }
}

/// The drag start of [`idle_click`]: the dragged tile is `tile`, the
/// pointer position (in menu space) is stored as the drag start and the
/// tile's drag traits are set from it.
fn idle_start_drag(e: &mut Engine, this: Ptr<InterfaceManager>, tile: u32) {
    e.set(this, InterfaceManager::pDragTarget, tile);
    let point = pointer_scaled(e, this);
    let drag = e.get(this, InterfaceManager::pDragTarget);
    let tile_x = e.call(TILE_GET_POSITION_X, &args![drag]).f32();
    let tile_y = e.call(TILE_GET_POSITION_Y, &args![drag]).f32();
    let (x, y) = pointer_to_pipboy_space(e, this, point);
    let start_x = e.call(FTOL, &args![x as f64]).i32();
    e.set(this, InterfaceManager::iDragStartX, start_x);
    let start_y = e.call(FTOL, &args![y as f64]).i32();
    e.set(this, InterfaceManager::iDragStartY, start_y);
    e.call(SET_CURSOR_OFFSET_X, &args![this, 0u32]);
    e.call(SET_CURSOR_OFFSET_Y, &args![this, 0u32]);
    let origin_x = tile_get_float(e, drag, 0xfa1);
    tile_set_float(
        e,
        drag,
        0xff0,
        (x as f64 - (tile_x as f64 - origin_x)) as f32,
    );
    let origin_y = tile_get_float(e, drag, 0xfa2);
    tile_set_float(
        e,
        drag,
        0xff1,
        (y as f64 - (tile_y as f64 - origin_y)) as f32,
    );
    let start_x = e.get(this, InterfaceManager::iDragStartX);
    tile_set_int(e, drag, 0xfea, start_x as u32);
    let start_y = e.get(this, InterfaceManager::iDragStartY);
    tile_set_int(e, drag, 0xfeb, start_y as u32);
    tile_set_float(e, drag, 0xfec, (x as f64 - tile_x as f64) as f32);
    tile_set_float(e, drag, 0xfed, (y as f64 - tile_y as f64) as f32);
    e.set(this, InterfaceManager::fDragOffsetLastX, 0.0);
    e.set(this, InterfaceManager::fDragOffsetLastY, 0.0);
}

/// Part of `idle_menu_mode`: with `input_0` (a button held) the menu under
/// the pointer and the drag-over menu are told (slots 0x20 and 0x24) and
/// the dragged tile follows the cursor offset; with no button held the
/// wheel scrolls the menu (`DoWheelMove`) or steps the console's pick list.
fn idle_button_and_wheel(e: &mut Engine, this: Ptr<InterfaceManager>, state: &mut IdleState) {
    if state.input_0 != 0 {
        let menu = e.get(this, InterfaceManager::pOverTileMenu);
        if menu != 0 {
            let tile = e.get(this, InterfaceManager::pOverTileTarget);
            let id = tile_menu_id(e, tile);
            e.vcall(menu, 0x20, &args![id, tile]);
        }
        let drag_menu = e.get(this, InterfaceManager::pDragOverTileMenu);
        let tile = e.get(this, InterfaceManager::pOverTileTarget);
        if drag_menu != 0 && tile != 0 {
            let id = tile_menu_id(e, tile);
            e.vcall(drag_menu, 0x24, &args![id, tile]);
        }
        let drag = e.get(this, InterfaceManager::pDragTarget);
        if drag != 0 {
            let offset_x = e.call(GET_CURSOR_OFFSET_X, &args![this]).i32();
            let last_x = e.get(this, InterfaceManager::fDragOffsetLastX);
            tile_set_float(e, drag, 0xfee, (offset_x as f64 - last_x as f64) as f32);
            let offset_y = e.call(GET_CURSOR_OFFSET_Y, &args![this]).i32();
            let last_y = e.get(this, InterfaceManager::fDragOffsetLastY);
            let drag = e.get(this, InterfaceManager::pDragTarget);
            tile_set_float(e, drag, 0xfef, (offset_y as f64 - last_y as f64) as f32);
            let offset_x = e.call(GET_CURSOR_OFFSET_X, &args![this]).i32();
            e.set(this, InterfaceManager::fDragOffsetLastX, offset_x as f32);
            let offset_y = e.call(GET_CURSOR_OFFSET_Y, &args![this]).i32();
            e.set(this, InterfaceManager::fDragOffsetLastY, offset_y as f32);
        }
    } else if e.get(this, InterfaceManager::fMouseWheel) != 0.0 {
        let menu = e.get(this, InterfaceManager::pOverTileMenu);
        if menu != 0 {
            let mut tile = e.get(this, InterfaceManager::pOverTileTarget);
            // The nearest tile (starting at the one under the pointer)
            // that wants the wheel (`0xff2`), walking up through
            // `Tile::GetParent`.
            while !e.call(TILE_IS_TRUE, &args![tile, 0xff2u32]).bool()
                && e.call(TILE_PARENT, &args![tile]).u32() != 0
            {
                tile = e.call(TILE_PARENT, &args![tile]).u32();
            }
            if tile != 0 && e.call(TILE_IS_TRUE, &args![tile, 0xff2u32]).bool() {
                let wheel = e.call(GET_WHEEL_STEPS, &args![this]).i32();
                tile_set_int(e, tile, 0xff3, (wheel / -120) as u32);
                tile_set_int(e, tile, 0xff3, 0);
                let id = tile_menu_id(e, tile);
                let menu = e.get(this, InterfaceManager::pOverTileMenu);
                e.call(DO_WHEEL_MOVE, &args![this, menu, id, tile]);
            }
        } else {
            let console = e.call(MENU_CONSOLE_INSTANCE, &args![1u32]).u32();
            if e.call(CONSOLE_PRIMARY_DOWN, &args![console]).bool() {
                let wheel = e.call(GET_WHEEL_STEPS, &args![this]).i32();
                let step = if wheel >= 0 { 1 } else { -1 };
                let index = e.get(this, InterfaceManager::iCurrentPickIndex) + step;
                e.set(this, InterfaceManager::iCurrentPickIndex, index);
                let last = e.call(PICK_LIST_COUNT, &args![this.addr() + 0x70]).u32();
                if index < -1 {
                    e.set(
                        this,
                        InterfaceManager::iCurrentPickIndex,
                        last.wrapping_sub(1) as i32,
                    );
                } else if (last.wrapping_sub(1)) < index as u32 {
                    e.set(this, InterfaceManager::iCurrentPickIndex, -1);
                }
                interface_manager_display_current_pick_ref(e, this);
            }
        }
    }
}

/// Reads the next controls event: `(kind, key code)` from
/// `Controls`' event queue (`00a23820`, which writes the key code to the
/// word at `slot`).
fn read_controls_event(e: &mut Engine, slot: Ptr) -> u32 {
    let owner = e.global::<u32>(CONTROLS_OWNER);
    let controls = e.call(CONTROLS_GET, &args![owner]).u32();
    e.call(CONTROLS_NEXT_EVENT, &args![controls, slot]).u32()
}

/// Reads events until one that is not kind 2, running `007166f0(0)` and
/// `007154b0` on each kind-2 event (key modifier changes), and returns the
/// `007154b0` result for the last one.
fn translate_controls_events(e: &mut Engine, this: Ptr<InterfaceManager>, slot: Ptr) -> u32 {
    let mut kind = read_controls_event(e, slot);
    while kind == 2 {
        e.call(KEY_REPEAT_RESET, &args![this, 0u32]);
        let code = e.mem.u32(slot.addr());
        e.call(TRANSLATE_KEY_EVENT, &args![this, kind, code]);
        kind = read_controls_event(e, slot);
    }
    let code = e.mem.u32(slot.addr());
    e.call(TRANSLATE_KEY_EVENT, &args![this, kind, code]).u32()
}

/// Part of `idle_menu_mode`: gamepad events, then the key events: each is
/// translated to a menu key (`007154b0`), offered to the console, then to
/// the frontmost menu (virtual slot `0x30`), then to the `_PCButton_<key>`
/// child tile of the menu; keys nobody took become gamepad events
/// (`DoGamepadEvent`).
fn idle_gamepad_events(e: &mut Engine, this: Ptr<InterfaceManager>, state: &mut IdleState) {
    if e.call(HAS_360_CONTROLLER_GETTER, &args![]).bool()
        && e.get(this, InterfaceManager::bShowMouse) == 0
    {
        interface_manager_do_gamepad_event(e, this, 0);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let mut key = translate_controls_events(e, this, slot);
        if key == 0 {
            key = e.call(KEY_REPEAT_00716730, &args![this, 1.0f32]).u32();
        }
        let mut consumed = false;
        state.console = e.call(MENU_CONSOLE_INSTANCE, &args![0u32]).u32();
        if key != 0 && state.console != 0 {
            loop {
                let code = e.mem.u32(slot.addr());
                consumed = if code == 0x29 {
                    false
                } else {
                    e.call(MENU_CONSOLE_IDLE, &args![state.console, key]).bool()
                };
                if consumed {
                    key = translate_controls_events(e, this, slot);
                }
                if !(key != 0 && consumed) {
                    break;
                }
            }
        }
        if key == 0
            && pointer_shown(e, this)
            && e.call(
                CONTROLS_QUERY_00A24660,
                &args![state.controls, 0x1cu32, 1u32],
            )
            .u32()
                != 0
            && is_top_menu(e, 0x3e9)
        {
            key = 1;
            fn_0070ec20(e, Ptr::new(state.controls), 0x1c, 0);
        }
        if key != 0 && !consumed {
            let code = e.mem.u32(slot.addr());
            idle_dispatch_key(e, this, state, key, code);
        }
    });
    // Sad-face messages for the two controller buttons `0x19` and `0x1a`.
    for (button, holder) in [(0x19u32, MESSAGE_HOLDER_19), (0x1a, MESSAGE_HOLDER_1A)] {
        if e.call(
            CONTROLS_QUERY_00A24660,
            &args![state.controls, button, 1u32],
        )
        .u32()
            != 0
            && !e.call(XUI_IS_UP, &args![]).bool()
        {
            let text = e.call(STRING_HOLDER_GET, &args![holder]).u32();
            let size: f32 = e.global(MESSAGE_ICON_SIZE);
            e.call(
                SHOW_ICON_MESSAGE,
                &args![text, 0u32, MESSAGE_ICON_PATH, 0u32, size, 0u32],
            );
        }
    }
}

/// A key `Idle` has translated and nobody consumed: the frontmost menu may
/// take it (slot `0x30`), else the matching `_PCButton_` tile is clicked,
/// else the key becomes a gamepad event.
fn idle_dispatch_key(
    e: &mut Engine,
    this: Ptr<InterfaceManager>,
    state: &mut IdleState,
    key: u32,
    code: u32,
) {
    let menu_manager = e.call(MENU_MANAGER_INSTANCE, &args![1u32]).u32();
    let frontmost = e.call(GET_FRONTMOST_MENU, &args![menu_manager]).u32();
    let mut taken = false;
    let mut pipboy_key = false;
    if e.call(IS_PIPBOY_MENU_TOPMOST, &args![]).bool() {
        let controls_owner = e.global::<u32>(CONTROLS_OWNER);
        let controls = e.call(CONTROLS_GET, &args![controls_owner]).u32();
        let page_key = e
            .call(CONTROLS_QUERY_00A24660, &args![controls, 0xeu32, 1u32])
            .u32()
            != 0
            || {
                let controls = e.call(CONTROLS_GET, &args![controls_owner]).u32();
                e.call(CONTROLS_QUERY_00A24660, &args![controls, 0xeu32, 2u32])
                    .u32()
                    != 0
            };
        if page_key {
            let controls = e.call(CONTROLS_GET, &args![controls_owner]).u32();
            pipboy_key = e
                .call(CONTROLS_QUERY_00A24180, &args![controls, 0xfu32, 0u32])
                .u32()
                == 0;
        }
    }
    if frontmost != 0 && !pipboy_key {
        taken = e.vcall(frontmost, 0x30, &args![key]).bool();
    }
    if !taken && frontmost != 0 && !pipboy_key {
        idle_click_key_tile(e, frontmost, code);
    }
    if !taken {
        let event = match key {
            0x8000_0001 => 4,
            0x8000_0002 => 3,
            0x8000_0003 => 1,
            0x8000_0004 => 2,
            0x8000_0008 => {
                if e.call(MODIFIER_FOUR_DOWN, &args![this]).bool() {
                    0xb
                } else if fn_0070ecd0(e, this) {
                    0xc
                } else {
                    -2
                }
            }
            0x8000_0009 => 0xf,
            0x8000_000a => 0x10,
            _ => 0,
        };
        let event = if event == 4 && e.call(MODIFIER_FOUR_DOWN, &args![this]).bool() {
            0xd
        } else {
            event
        };
        let event = if event == 3 && e.call(MODIFIER_FOUR_DOWN, &args![this]).bool() {
            0xe
        } else {
            event
        };
        if event != 0 {
            interface_manager_do_gamepad_event(e, this, event);
            e.call(CLEAR_KEYSTROKES, &args![state.controls]);
        }
    }
}

/// The `_PCButton_<key>` child tile of the frontmost menu: when it is
/// enabled (`0xfa3`) it is clicked (`0xfaf` set: sound, click pulse and the
/// menu's slot `0xc`), else the "not allowed" menu sound plays.
fn idle_click_key_tile(e: &mut Engine, frontmost: u32, code: u32) {
    let controls_owner = e.global::<u32>(CONTROLS_OWNER);
    let controls = e.call(CONTROLS_GET, &args![controls_owner]).u32();
    let key_char = e
        .call(CONTROLS_QUERY_00A238A0, &args![controls, code, 0xffu32])
        .u32();
    let name = e.with_stack(0x10, |e, buffer| {
        e.call(
            FORMAT_STRING,
            &args![
                buffer,
                0x10u32,
                FORMAT_PC_BUTTON,
                PC_BUTTON_PREFIX,
                key_char
            ],
        );
        let trait_id = e.call(TILE_TEXT_TO_TRAIT, &args![buffer]).u32();
        let root = e.call(GET_FIELD_AT_4, &args![frontmost]).u32();
        let string = e.call(TILE_GET_STRING, &args![root, trait_id]).u32();
        let root = e.call(GET_FIELD_AT_4, &args![frontmost]).u32();
        e.call(TILE_GET_CHILD_BY_NAME, &args![root, string]).u32()
    });
    if name != 0 && e.call(TILE_IS_TRUE, &args![name, 0xfa3u32]).bool() {
        if e.call(TILE_IS_TRUE, &args![name, 0xfafu32]).bool() {
            e.call(TILE_PLAY_TILE_SOUND, &args![name, 0xfcbu32]);
            tile_set_int(e, name, 0xfc7, 1);
            tile_set_int(e, name, 0xfc7, 0);
            let id = tile_menu_id(e, name);
            e.vcall(frontmost, 0xc, &args![id, name]);
        } else {
            e.call(PLAY_MENU_SOUND, &args![2u32]);
        }
    }
}

/// The last part of `Idle`, after the mode-dependent work: the console
/// toggle (key `0x1d`), the start menu key (`0x1c`), the menus' per-frame
/// virtual (slot `0x2c`), the pipboy hot keys (`0x3b`..`0x3d`), the dialog
/// pause, and the pipboy/tutorial updates.
fn idle_console_and_hot_keys(e: &mut Engine, this: Ptr<InterfaceManager>, state: &mut IdleState) {
    let mut console_wanted = false;
    if e.call(CONSOLE_KEY_FLAG_GET, &args![]).bool() {
        e.call(CONSOLE_KEY_FLAG_SET, &args![0u32]);
        let console = e.call(MENU_CONSOLE_INSTANCE, &args![1u32]).u32();
        if !e.call(CONSOLE_PRIMARY_DOWN, &args![console]).bool() {
            console_wanted = true;
        }
    }
    if e.call(
        CONTROLS_QUERY_00A24660,
        &args![state.controls, 0x1du32, 1u32],
    )
    .u32()
        != 0
        || console_wanted
    {
        state.console = e.call(MENU_CONSOLE_INSTANCE, &args![1u32]).u32();
        e.call(CLEAR_KEYSTROKES, &args![state.controls]);
        if e.call(MENU_CONSOLE_TOGGLE_VISIBLE, &args![state.console])
            .bool()
        {
            e.call(ADD_TO_ENTER_STACK, &args![this, 3u32]);
        } else {
            e.call(POP_FROM_ENTER_STACK, &args![this, 3u32, 0u32]);
        }
    }
    if e.call(
        CONTROLS_QUERY_00A24660,
        &args![state.controls, 0x1cu32, 1u32],
    )
    .u32()
        != 0
    {
        idle_start_menu_key(e, this);
    }
    e.call(DEBUG_TEXT_UPDATE, &args![]);
    // The unreachable `Interface::InDialog` / `SetDialogPause` path of the
    // compiled code (guarded by a local that is always 0) is omitted.
    e.set_global(MENU_LOOP_ABORT, 0u8);
    let root = e.get(this, InterfaceManager::pMenusRoot);
    let first = e.call(NI_POINTER_GET, &args![root + 4]).u32();
    e.with_stack(4, |e, cursor| {
        e.mem.set_u32(cursor.addr(), first);
        while e.mem.u32(cursor.addr()) != 0 && e.global::<u8>(MENU_LOOP_ABORT) == 0 {
            // Returns the element pointer and advances the cursor.
            let element = e.call(LIST_NEXT_ELEMENT, &args![root + 4, cursor]).u32();
            let child = e.mem.u32(element);
            if child != 0 {
                let menu = e.call(TILE_GET_MENU, &args![child]).u32();
                if menu != 0 {
                    e.vcall(menu, 0x2c, &args![]);
                }
            }
        }
    });
    if e.global::<u8>(MENU_LOOP_ABORT) != 0 {
        e.set_global(MENU_LOOP_ABORT, 0u8);
        return;
    }
    let mode_word = e.get(this, InterfaceManager::field_4bc);
    let key_down = if mode_word == 3 {
        if !e.call(HAS_360_CONTROLLER_GETTER, &args![]).bool() {
            e.call(
                CONTROLS_QUERY_00A24660,
                &args![state.controls, 0xeu32, 2u32],
            )
            .u32()
                != 0
                && !state.menu_blocks_pointer
        } else {
            (e.call(CHECK_MENU_BUTTON, &args![10u32, 1u32]).i32() == 1
                || e.call(CHECK_MENU_BUTTON, &args![6u32, 1u32]).i32() == 1)
                && !state.menu_blocks_pointer
        }
    } else {
        let player = e.global::<u32>(PLAYER_SINGLETON);
        fn_0070edc0(e, Ptr::new(player)) != 0
    };
    if key_down && e.get(this, InterfaceManager::field_4bc) == 0 {
        fn_0070f4e0(e, this, 0, 0);
    } else {
        let weapon_state = e
            .call(WEAPON_STATE_GETTER, &args![WEAPON_STATE_OBJECT])
            .u32();
        let player = e.global::<u32>(PLAYER_SINGLETON);
        let anything = weapon_state != 0 || e.vcall(player, 0x22c, &args![0u32]).bool() || key_down;
        let mode_word = e.get(this, InterfaceManager::field_4bc) as i32;
        if anything && pointer_shown(e, this) && mode_word > 0 && mode_word <= 3 {
            fn_0070f690(e, this, 0);
            let owner = e.global::<u32>(CONTROLS_OWNER);
            let controls = e.call(CONTROLS_GET, &args![owner]).u32();
            e.call(CONTROLS_CLEAR_USER_ACTIONS, &args![controls]);
        }
    }
    idle_pipboy_hot_keys(e, this, state);
    let top = e.call(GET_ENTER_STACK_TOP, &args![this]).u32();
    if top == 0 || e.call(IS_PIPBOY_MENU_TOPMOST, &args![]).bool() {
        interface_manager_update_pipboy(e, this);
    }
    e.call(TUTORIAL_MANAGER_UPDATE, &args![this.addr() + 0x4d4]);
}

/// The start menu key (`0x1c`): opens the start menu or closes it again,
/// or queues the pause menu, depending on the menus already open.
fn idle_start_menu_key(e: &mut Engine, this: Ptr<InterfaceManager>) {
    let manager = e.call(MENU_MANAGER_INSTANCE, &args![1u32]).u32();
    if e.call(MENU_MANAGER_IS_MENU_OPEN, &args![manager, 0x3efu32])
        .bool()
    {
        return;
    }
    if e.call(XUI_IS_UP, &args![]).bool() {
        return;
    }
    let mode_word = e.get(this, InterfaceManager::field_4bc);
    if mode_word != 3 && mode_word != 0 {
        return;
    }
    if e.call(MENU_FLAG_QUERY_004A4040, &args![]).bool()
        && is_top_menu(e, 0x3f5)
        && e.call(IS_TOP_MENU_FADED_IN, &args![]).bool()
    {
        let top = e.call(GET_ENTER_STACK_TOP, &args![this]).i32();
        let start_menu = e.call(PIPBOY_MENU_CLASS, &args![]).i32();
        if top == start_menu && !fn_0070ee30(e) {
            e.call(START_MENU_CLOSE, &args![1u32]);
        }
    } else if e.global::<u8>(START_MENU_ALLOWED) != 0 {
        let flag = e.call(GET_SETTING_VALUE, &args![START_MENU_SETTING]).u32();
        if e.mem.u32(flag) == 1 {
            e.call(START_MENU_CREATE, &args![1u32, 0u32]);
        } else if !e.call(QUEUE_PENDING_00070EC00, &args![]).bool() {
            e.call(
                QUEUE_MENU_CREATE,
                &args![6u32, 0u32, 0u32, 0u32, 1u32, 0u32],
            );
        }
    }
}

/// The pipboy hot keys (`0x3b` stats, `0x3c` inventory, `0x3d` map) in the
/// two pipboy modes: in mode 0 they open the pipboy page, in mode 3 they
/// switch to it (or leave the pipboy when it is already shown).
fn idle_pipboy_hot_keys(e: &mut Engine, this: Ptr<InterfaceManager>, state: &mut IdleState) {
    let mut menu = 0u32;
    let mut pressed = false;
    for (key, id) in [(0x3bu32, 0x3ebu32), (0x3c, 0x3ea), (0x3d, 0x3ff)] {
        if e.call(CONTROLS_QUERY_00A24180, &args![state.controls, key, 1u32])
            .u32()
            != 0
        {
            menu = id;
            pressed = true;
            break;
        }
    }
    if pressed && e.get(this, InterfaceManager::field_4bc) == 0 {
        fn_0070f4e0(e, this, 0, menu);
    }
    if pressed && e.get(this, InterfaceManager::field_4bc) == 3 {
        let root = e.call(GET_MENUS_ROOT, &args![this]).u32();
        let shown = tile_get_float(e, root, 0x1771);
        if menu as f64 == shown {
            fn_0070f690(e, this, 0);
        } else if menu == 0x3ea {
            e.call(SET_INVENTORY_MENU_VISIBLE, &args![1u32, 0u32, 1u32]);
        } else if menu == 0x3eb {
            e.call(SET_STATS_MENU_VISIBLE, &args![1u32, 0u32]);
        } else if menu == 0x3ff {
            e.call(SET_MAP_MENU_VISIBLE, &args![1u32, 0u32]);
        }
    }
}

// Translated from 0070ec20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `+0x1b94 + b * 0x1c + a` of the controls object to
/// `0xff` (clears one key binding slot: `a` is the action, `b` the slot).
pub fn fn_0070ec20(e: &mut Engine, this: Ptr, a: i32, b: i32) {
    let offset = (b.wrapping_mul(0x1c)).wrapping_add(a) as u32;
    e.mem
        .set_u8(this.addr().wrapping_add(offset).wrapping_add(0x1b94), 0xff);
}

// Translated from 0070ec50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte at `+0x162` of the object (called on the pipboy
/// manager).
pub fn fn_0070ec50(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x162)
}

// Translated from 0070ecd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when bit 0 of `iModifierKeys` (`+0x14c`) is set.
pub fn fn_0070ecd0(e: &mut Engine, this: Ptr<InterfaceManager>) -> bool {
    e.get(this, InterfaceManager::iModifierKeys) & 1 != 0
}

// Translated from 0070ed30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InventoryMenu::IsKeyringOpen` (Xbox PDB): true when the inventory menu
/// exists and its keyring tile (the tile `00726070` returns for it, trait
/// stored at `011d9eb8`) is true.
pub fn inventory_menu_is_keyring_open(e: &mut Engine) -> bool {
    let menu = e.global::<u32>(INVENTORY_MENU);
    if menu == 0 {
        return false;
    }
    let trait_id = e.global::<u32>(KEYRING_TRAIT);
    let tile = e.call(GET_FIELD_AT_4, &args![menu]).u32();
    e.call(TILE_IS_TRUE, &args![tile, trait_id]).bool()
}

// Translated from 0070edc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte at `+0xd54` of the object (called on the player).
pub fn fn_0070edc0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0xd54)
}

// Translated from 0070ee30 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the object at `011daac0` has both flag bits `0x8000` and `0x8`
/// (`004a4080`).
pub fn fn_0070ee30(e: &mut Engine) -> bool {
    let flags_owner = e.global::<u32>(FLAGS_OWNER);
    e.call(HAS_FLAG, &args![flags_owner, 0x8000u32]).bool() && {
        let flags_owner = e.global::<u32>(FLAGS_OWNER);
        e.call(HAS_FLAG, &args![flags_owner, 8u32]).bool()
    }
}

// Translated from 0070ee80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::UpdatePipboy` (Xbox PDB): the pipboy state machine
/// (`+0x4bc`): 1 raising it (closes the container menu or the terminal, or
/// starts the pipboy-up animation, sound and menu), 2 waiting for the
/// animation to reach the pipboy pose, 3 open (keeps the map tidy), 4
/// lowering, 5 waiting for the player to stop being in the pipboy.
/// Callbacks stored at `+0x4c0` run when a transition finishes.
pub fn interface_manager_update_pipboy(e: &mut Engine, this: Ptr<InterfaceManager>) {
    let player = e.global::<u32>(PLAYER_SINGLETON);
    match e.get(this, InterfaceManager::field_4bc) {
        1 => {
            if is_top_menu(e, 0x3f0) {
                e.call(CONTAINER_MENU_CLOSE, &args![]);
            } else if is_top_menu(e, 0x421) || is_top_menu(e, 0x41f) {
                let rendered = e.get(this, InterfaceManager::pCurrentRenderedMenu);
                e.call(RENDERED_MENU_CLOSE, &args![rendered]);
            } else {
                let action = e.call(ACTOR_GET_ANIM_ACTION, &args![player]).u32();
                // The animation actions during which the pipboy cannot be
                // raised: 0-6, 8-0xd and 0x11.
                let blocked = matches!(action, 0..=6 | 8..=0xd | 0x11);
                if !blocked {
                    pipboy_raise(e, this, player);
                }
            }
        }
        2 => {
            let mut animation = e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32();
            if animation != 0
                && e.call(ANIMATION_GET_SEQUENCE, &args![animation, 2u32])
                    .u32()
                    != 0
            {
                if fn_0070f490(e, Ptr::new(animation), 2) == 1 {
                    pipboy_pose_reached(e, this, player, &mut animation);
                }
            } else {
                let pipboy = e.call(GET_PIPBOY_STATIC, &args![]).u32();
                e.call(PIPBOY_FADE_LIGHT_EFFECT, &args![pipboy, 0u32]);
                e.set(this, InterfaceManager::field_4bc, 4);
            }
            if e.call(MAP_MENU_NEEDS_TIDY, &args![]).bool() {
                e.call(MAP_MENU_TIDY, &args![0u32]);
            }
        }
        3 => {
            if e.call(MAP_MENU_NEEDS_TIDY, &args![]).bool() {
                e.call(MAP_MENU_TIDY, &args![0u32]);
            }
        }
        4 => {
            if (e.call(POP_FROM_ENTER_STACK, &args![this, 1u32, 0u32]).i32()) >= 0 {
                let pipboy = e.call(GET_PIPBOY_STATIC, &args![]).u32();
                e.call(PIPBOY_FADE_LIGHT_EFFECT, &args![pipboy, 0u32]);
                if e.vcall(player, 0x214, &args![]).i32() == 4 {
                    let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![player]).u32();
                    e.call(RELOAD_DYNAMIC_IDLE_ON_1ST_PERSON, &args![process, player]);
                }
                let animation = e.call(PLAYER_GET_ANIMATION, &args![player, 0u32]).u32();
                let group = e.call(ANIMATION_GROUP_ID, &args![animation, 0u32]).u16();
                e.vcall(player, 0x4b0, &args![group as u32, 1u32]);
                e.call(MAP_MENU_CLEAR_MAP_MEMORY, &args![]);
                loop {
                    let manager = e.call(GET_MANAGER, &args![]).u32();
                    if e.mem.u8(manager + 0x20) == 0 {
                        break;
                    }
                    let manager = e.call(GET_MANAGER, &args![]).u32();
                    let sound = e.mem.u8(manager + 0x20).wrapping_sub(1);
                    e.mem.set_u8(manager + 0x20, sound);
                    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
                    e.call(AUDIO_UNPAUSE_TYPE_4000_0000, &args![audio]);
                }
                e.set(this, InterfaceManager::field_4bc, 5);
            }
        }
        5 if !e.call(PLAYER_IS_PIPBOY_ACTIVE, &args![player]).bool() => {
            let animation = e.call(PLAYER_GET_ANIMATION, &args![player, 0u32]).u32();
            let group = e.call(ANIMATION_GROUP_ID, &args![animation, 2u32]).u16();
            e.vcall(player, 0x4b0, &args![group as u32, 1u32]);
            e.set(this, InterfaceManager::field_4bc, 0);
            run_pipboy_callback(e, this);
        }
        _ => {}
    }
}

/// Runs and clears the callback stored at `+0x4c0`, if any.
fn run_pipboy_callback(e: &mut Engine, this: Ptr<InterfaceManager>) {
    let callback = e.get(this, InterfaceManager::field_4c0);
    if callback != 0 {
        e.call(callback, &args![]);
        e.set(this, InterfaceManager::field_4c0, 0);
    }
}

/// State 1 of `UpdatePipboy`: clears the player's current animation groups,
/// plays the pipboy-up group (`0xe2`), fades the light effect in, plays
/// `UIPipBoyAccessUp`, stops the get-hit image space modifier, pauses the
/// sound once more and opens the pipboy menu (state 2).
fn pipboy_raise(e: &mut Engine, this: Ptr<InterfaceManager>, player: u32) {
    let mut animation = e.call(PLAYER_GET_ANIMATION, &args![player, 0u32]).u32();
    if animation != 0 {
        e.call(ANIMATION_CLEAR_GROUP, &args![animation, 1u32, 0.0f32]);
        animation = e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32();
    }
    if animation == 0 {
        return;
    }
    e.call(ANIMATION_CLEAR_GROUP, &args![animation, 1u32, 0.0f32]);
    e.call(PLAYER_FORCE_TEMP_1ST_PERSON, &args![player, 0u32]);
    e.call(
        ANIMATION_PLAY_GROUP,
        &args![animation, 0xe2u32, 1u32, -1i32, -1i32],
    );
    let pipboy = e.call(GET_PIPBOY_STATIC, &args![]).u32();
    e.call(PIPBOY_FADE_LIGHT_EFFECT, &args![pipboy, 1u32]);
    e.with_stack(12, |e, found| {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let handle = e
            .call(
                AUDIO_GET_SOUND_HANDLE_BY_NAME,
                &args![audio, found, PIPBOY_ACCESS_UP_SOUND, 0x121u32],
            )
            .u32();
        e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
        e.call(EMPTY_MEMBER_FUNCTION, &args![found]);
    });
    let get_hit = e.call(IMAGE_SPACE_GET_HIT, &args![]).u32();
    e.call(IMAGE_SPACE_STOP, &args![get_hit]);
    let manager = e.call(GET_MANAGER, &args![]).u32();
    let sound = e.mem.u8(manager + 0x20).wrapping_add(1);
    e.mem.set_u8(manager + 0x20, sound);
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    fn_0070bba0(e, Ptr::new(audio));
    e.call(PLAYER_STATE_00962590, &args![player]);
    e.call(ADD_TO_ENTER_STACK, &args![this, 1u32]);
    e.call(INTERFACE_SHOW_MENUS, &args![]);
    e.set(this, InterfaceManager::field_4bc, 2);
}

/// State 2 of `UpdatePipboy` once the animation reached the pipboy pose:
/// shows the HUD for it, lets a drawn weapon's wobble clear, advances the
/// animation to the end of the pose, runs the pending callback and enters
/// the rendered pipboy menu (state 3).
fn pipboy_pose_reached(
    e: &mut Engine,
    this: Ptr<InterfaceManager>,
    player: u32,
    animation: &mut u32,
) {
    e.call(HUD_SET_MENU_MODE, &args![3u32]);
    if e.call(ACTOR_IS_WEAPON_DRAWN, &args![player]).bool() {
        let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![player]).u32();
        let weapon = if e.vcall(process, 0x148, &args![]).u32() == 0 {
            0
        } else {
            let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![player]).u32();
            let object = e.vcall(process, 0x148, &args![]).u32();
            e.call(FORM_GETTER_0044DDC0, &args![object]).u32()
        };
        if weapon != 0 && e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32() != 0 {
            let first = e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32();
            if e.call(ANIMATION_NODE, &args![first]).u32() != 0 {
                let second = e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32();
                let node = e.call(ANIMATION_NODE, &args![second]).u32();
                let index = e.call(WEAPON_TYPE_INDEX, &args![weapon]).u32();
                let entry = e.mem.u32(WEAPON_TABLE + index * 4);
                e.call(CLEAR_GUN_WOBBLE, &args![entry, node]);
            }
        }
    }
    let sequence = e
        .call(ANIMATION_GET_SEQUENCE, &args![*animation, 2u32])
        .u32();
    let end_time = e.call(SEQUENCE_END_TIME, &args![sequence]).f64();
    let mut time = (-end_time) as f32;
    let sequence = e
        .call(ANIMATION_GET_SEQUENCE, &args![*animation, 2u32])
        .u32();
    let global_transform = e
        .call(ANIMATION_ZERO_GLOBAL_TRANSFORM, &args![sequence])
        .u32();
    let group_time = e
        .call(ANIM_GROUP_GET_TIME, &args![global_transform, 1u32])
        .f64();
    time = (group_time + time as f64) as f32;
    e.call(ANIMATION_UPDATE, &args![*animation, player, 0.0f32, time]);
    e.call(
        ANIMATION_UPDATE_BIP_ONLY,
        &args![*animation, time, 0u32, 1u32],
    );
    e.call(
        ANIMATION_UPDATE_SCENE_GRAPH_NO_CONTROLLER,
        &args![*animation],
    );
    e.set_global(IDLE_MANAGER_BUSY, 1u8);
    let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![player]).u32();
    if e.vcall(process, 0x4d8, &args![player]).bool() {
        let first = e.call(PLAYER_GET_ANIMATION, &args![player, 0u32]).u32();
        e.call(ANIMATION_CLEAR_GROUP, &args![first, 0u32, 0.0f32]);
    } else {
        e.call(LOG_WARNING, &args![MISSING_DYNAMIC_IDLE_MESSAGE]);
    }
    e.set_global(IDLE_MANAGER_BUSY, 0u8);
    run_pipboy_callback(e, this);
    let manager = e.call(GET_MANAGER, &args![]).u32();
    let pipboy = e.call(GET_PIPBOY, &args![manager]).u32();
    e.call(PIPBOY_MANAGER_UPDATE, &args![pipboy]);
    let manager = e.call(GET_MANAGER, &args![]).u32();
    let pipboy = e.call(GET_PIPBOY, &args![manager]).u32();
    let manager = e.call(GET_MANAGER, &args![]).u32();
    e.call(ENTER_RENDERED_MENU, &args![manager, pipboy]);
    e.set(this, InterfaceManager::field_4bc, 3);
}

// Translated from 0070f490 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `this + 0x5c + index * 4`, with `index` 0x14 read as 1 and
/// 0x15 as 4 (the same code as `00491040`, which the linker folded; the
/// pipboy code calls it on an animation object).
pub fn fn_0070f490(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    let index = match index {
        0x14 => 1,
        0x15 => 4,
        other => other,
    };
    e.mem.u32(
        this.addr()
            .wrapping_add(0x5c)
            .wrapping_add((index as u32).wrapping_mul(4)),
    )
}

// Translated from 0070f4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Asks for the pipboy to open: only when the player is idle enough (not
/// in the states `005a03f0(4)`, `0x22c`, not mid-transition, no hot keys
/// wheel up, tile events accepted, the weapon state quiet, ...).
/// `value`, when not zero, is stored in the `0x1771` trait of the root
/// tile; the callback is run at once while the pipboy is open (state 3), or
/// stored for the transition when the pipboy is closed (state 0 becomes 1).
pub fn fn_0070f4e0(e: &mut Engine, this: Ptr<InterfaceManager>, callback: u32, value: u32) {
    let player = e.global::<u32>(PLAYER_SINGLETON);
    if e.call(CONDITION_CHECK_005A03F0, &args![player, 4u32])
        .bool()
    {
        return;
    }
    if e.vcall(player, 0x22c, &args![0u32]).bool() {
        return;
    }
    let first = e.call(PLAYER_QUERY_005737E0, &args![player]).u8();
    let second = e.call(PLAYER_QUERY_004EAF60, &args![player]).u8();
    if first != second {
        return;
    }
    if e.call(PLAYER_QUERY_0093A740, &args![player]).bool() {
        return;
    }
    if e.call(PLAYER_QUERY_005721E0, &args![player]).bool() {
        return;
    }
    let hot_keys = e.global::<u32>(HOT_KEYS_OWNER);
    if hot_keys != 0 && fn_0070f670(e, Ptr::new(hot_keys)) != 0 {
        return;
    }
    if !e.call(TILE_IS_ACCEPTING_EVENTS, &args![this, 0u32]).bool() {
        return;
    }
    if e.vcall(player, 0x230, &args![]).bool() {
        return;
    }
    if e.call(FORM_GETTER_0044DDC0, &args![WEAPON_STATE_OBJECT])
        .u32()
        != 0
    {
        return;
    }
    if e.global::<u8>(PLAYER_BUSY_FLAG) != 0 {
        return;
    }
    if e.call(WEAPON_STATE_FLOAT, &args![WEAPON_STATE_OBJECT])
        .f64()
        != 0.0
    {
        return;
    }
    let owner = e.global::<u32>(FLAGS_OWNER_8804);
    if e.call(FLAGS_QUERY_00701450, &args![owner, 1u32]).bool() {
        return;
    }
    if value != 0 {
        let root = e.get(this, InterfaceManager::pMenusRoot);
        tile_set_int(e, root, 0x1771, value);
    }
    let mode_word = e.get(this, InterfaceManager::field_4bc);
    if mode_word == 3 && callback != 0 {
        e.call(callback, &args![]);
    } else if mode_word == 0 {
        e.set(this, InterfaceManager::field_4bc, 1);
        e.set(this, InterfaceManager::field_4c0, callback);
    }
}

// Translated from 0070f670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `00701740` on the sub-object at `+0x1c8` and returns its
/// result.
pub fn fn_0070f670(e: &mut Engine, this: Ptr) -> u32 {
    e.call(SUB_OBJECT_QUERY_00701740, &args![this.addr() + 0x1c8])
        .u32()
}

// Translated from 0070f690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `callback` at once when the pipboy is in state 0 and a callback
/// is given; in state 3 stores it and starts the lowering (state 4).
pub fn fn_0070f690(e: &mut Engine, this: Ptr<InterfaceManager>, callback: u32) {
    let mode_word = e.get(this, InterfaceManager::field_4bc);
    if mode_word == 0 && callback != 0 {
        e.call(callback, &args![]);
    } else if mode_word == 3 {
        e.set(this, InterfaceManager::field_4bc, 4);
        e.set(this, InterfaceManager::field_4c0, callback);
    }
}

// Translated from 0070f6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::DoGamepadEvent` (Xbox PDB): handles one gamepad event
/// (`event` 0 reads the top one with `GetTopGamepadButton`). The menus are
/// offered the event from the topmost down (`DoGamepad`); a direction event
/// (1 to 4) moves the focus to the nearest tile that reacts to it; other
/// buttons click the tile bound to them; with no menu the console's pick
/// list steps.
pub fn interface_manager_do_gamepad_event(e: &mut Engine, this: Ptr<InterfaceManager>, event: i32) {
    let mut event = event;
    let mut released = false;
    let root = e.call(GET_MENUS_ROOT, &args![this]).u32();
    let mut tail = e.call(GET_FIELD_AT_4, &args![root + 4]).u32();
    // Every visible root child gets `DoGamepad(menu, -1, 0.0)` first, from
    // the last child to the first; the first that takes it ends the event.
    while tail != 0 {
        // The frame of `0070f6e0` (0x84 bytes): the tail element at `EBP-0x50` (+0x34).
        let child = e.with_stack(0x84, |e, frame| {
            let slot = Ptr::<()>::new(frame.addr() + 0x34);
            e.mem.set_u32(slot.addr(), tail);
            let root = e.call(GET_MENUS_ROOT, &args![this]).u32();
            let element = e.call(LIST_PREVIOUS_ELEMENT, &args![root + 4, slot]).u32();
            tail = e.mem.u32(slot.addr());
            e.mem.u32(element)
        });
        let menu = if child != 0 {
            e.call(TILE_GET_MENU, &args![child]).u32()
        } else {
            0
        };
        if menu != 0 && e.call(TILE_IS_VISIBLE, &args![child]).bool() {
            let taken = e.call(DO_GAMEPAD, &args![this, menu, -1i32, 0.0f32]).bool();
            if taken {
                return;
            }
        }
    }
    if e.call(HAS_360_CONTROLLER_GETTER, &args![]).bool()
        && e.get(this, InterfaceManager::bShowMouse) == 0
    {
        let mut flag_a = 0u8;
        let mut flag_b = 0u8;
        let over = e.call(GET_MOUSE_OVER_TARGET, &args![this]).u32();
        if over != 0 && e.call(TILE_GET_MENU, &args![over]).u32() != 0 {
            let menu = e.call(TILE_GET_MENU, &args![over]).u32();
            if !menu_is_active(e, menu) {
                return;
            }
        }
        if over != 0 {
            flag_a = e.call(TILE_IS_TRUE, &args![over, 0xfcfu32]).u8();
            flag_b = e.call(TILE_IS_TRUE, &args![over, 0xfd0u32]).u8();
        }
        if event == 0 {
            // The exe's frame of `0070f6e0` (`SUB ESP,0x74` under the exception record, then the
            // record link): the released byte at `EBP-0xd` (+0x77).
            event = e.with_stack(0x84, |e, frame| {
                let out = Ptr::new(frame.addr() + 0x77);
                let result = interface_manager_get_top_gamepad_button(e, this, flag_a, flag_b, out);
                released = e.mem.u8(out.addr()) != 0;
                result
            });
        }
        if event != 0 && event != 4 && event != 3 && event != 1 && event != 2 {
            e.call(KEY_REPEAT_00705B10, &args![this, 0u32]);
        }
    }
    let menu_manager = e.call(MENU_MANAGER_INSTANCE, &args![1u32]).u32();
    let frontmost = e.call(GET_FRONTMOST_MENU, &args![menu_manager]).u32();
    let frontmost_tile = if frontmost != 0 {
        e.call(GET_FIELD_AT_4, &args![frontmost]).u32()
    } else {
        0
    };
    if frontmost != 0
        && !released
        && e.call(DO_GAMEPAD, &args![this, frontmost, event, 1.0f32])
            .bool()
    {
        return;
    }
    if released {
        if frontmost != 0 {
            e.vcall(frontmost, 0x3c, &args![event, 1.0f32]);
        }
        return;
    }
    if frontmost != 0 && frontmost_tile != 0 && event != 0 {
        if matches!(event, 1..=4) {
            gamepad_direction_event(e, this, frontmost, frontmost_tile, event);
        } else {
            gamepad_button_event(e, this, frontmost, frontmost_tile, event);
        }
    } else {
        let console = e.call(MENU_CONSOLE_INSTANCE, &args![1u32]).u32();
        if e.call(CONSOLE_PRIMARY_DOWN, &args![console]).bool() && (event == 1 || event == 2) {
            let step = if event != 2 { 1 } else { -1 };
            let index = e.get(this, InterfaceManager::iCurrentPickIndex) + step;
            e.set(this, InterfaceManager::iCurrentPickIndex, index);
            let count = e.call(PICK_LIST_COUNT, &args![this.addr() + 0x70]).u32();
            if index < -1 {
                e.set(
                    this,
                    InterfaceManager::iCurrentPickIndex,
                    count.wrapping_sub(1) as i32,
                );
            } else if count.wrapping_sub(1) < index as u32 {
                e.set(this, InterfaceManager::iCurrentPickIndex, -1);
            }
            interface_manager_display_current_pick_ref(e, this);
        }
    }
}

/// The trait that names the tile reacting to each direction event.
fn direction_trait(event: i32) -> u32 {
    match event {
        1 => 0xfd7,
        2 => 0xfd8,
        4 => 0xfd9,
        3 => 0xfda,
        _ => 0,
    }
}

/// Part of `DoGamepadEvent`, for a direction event (1 to 4) and a frontmost
/// menu with a root tile: moves the focus.
fn gamepad_direction_event(
    e: &mut Engine,
    this: Ptr<InterfaceManager>,
    frontmost: u32,
    root_tile: u32,
    event: i32,
) {
    e.set(this, InterfaceManager::bMouseInMotion, 0);
    let trait_id = direction_trait(event);
    // The trait id the reacting tile was found under (out parameter). The frame is that of
    // `0070f6e0` (0x84 bytes): the trait at `EBP-0x30` (+0x54), the focus seed at `EBP-0x40`
    // (+0x44), the visited list at `EBP-0x48` (+0x3c), the current tile at `EBP-0x34` (+0x50)
    // and the candidate at `EBP-0x4c` (+0x38).
    e.with_stack(0x84, |e, frame| {
        let found_trait = Ptr::<()>::new(frame.addr() + 0x54);
        e.mem.set_u32(found_trait.addr(), 0xfc3);
        let current = e.call(GET_MOUSE_OVER_TARGET, &args![this]).u32();
        if current == 0 {
            // No tile has the focus: take the one with the largest focus
            // value, or the first reacting tile of the menu.
            let seed = frame.addr() + 0x44;
            e.mem.set_u32(seed, 0x8000_0000);
            let mut best = e.call(SCAN_FOR_MAX_FOCUS, &args![this, seed, 0u32]).u32();
            e.call(SET_CURRENT_FOCUS_TARGET, &args![this, best, 0xfc3u32, 1u32]);
            if best == 0 {
                best = e
                    .call(
                        TILE_GET_FIRST_REF_COPY,
                        &args![root_tile, trait_id, found_trait],
                    )
                    .u32();
                if best != 0 {
                    e.call(TILE_PLAY_TILE_SOUND, &args![best, 0xfcbu32]);
                    tile_set_int(e, best, 0xfc7, 1);
                    tile_set_int(e, best, 0xfc7, 0);
                    let id = tile_menu_id(e, best);
                    e.vcall(frontmost, 0xc, &args![id, best]);
                } else if e.call(HAS_360_CONTROLLER_GETTER, &args![]).bool()
                    || e.get(this, InterfaceManager::bShowMouse) == 0
                {
                    e.call(
                        DO_GAMEPAD,
                        &args![this, frontmost, event, trait_id as i32 as f32],
                    );
                }
            }
            return;
        }
        // The tiles already tried are kept in a `BSSimpleList` (`0096a2d0`,
        // add `005ae3d0`, contains `005f65d0`, clear `00470470`, destroy
        // `0046ffb0`); the add and contains functions take the address of
        // the variable that holds the tile.
        let visited = Ptr::<()>::new(frame.addr() + 0x3c);
        let current_cell = Ptr::<()>::new(frame.addr() + 0x50);
        let candidate_cell = Ptr::<()>::new(frame.addr() + 0x38);
        let mut candidate = e
            .call(
                TILE_GET_FIRST_REF_COPY,
                &args![current, trait_id, found_trait],
            )
            .u32();
        e.call(SIMPLE_LIST_CONSTRUCT, &args![visited]);
        e.mem.set_u32(current_cell.addr(), current);
        e.call(LIST_PUSH_FRONT_005AE3D0, &args![visited, current_cell]);
        loop {
            e.mem.set_u32(candidate_cell.addr(), candidate);
            if candidate == 0
                || e.call(LIST_CONTAINS, &args![visited, candidate_cell])
                    .bool()
                || (e.call(TILE_IS_TRUE, &args![candidate, 0xfa3u32]).bool()
                    && e.call(TILE_IS_TRUE, &args![candidate, 0xfafu32]).bool())
            {
                break;
            }
            e.call(LIST_PUSH_FRONT_005AE3D0, &args![visited, candidate_cell]);
            candidate = e
                .call(
                    TILE_GET_FIRST_REF_COPY,
                    &args![candidate, trait_id, found_trait],
                )
                .u32();
        }
        if candidate != 0
            && e.call(LIST_CONTAINS, &args![visited, candidate_cell])
                .bool()
            && (!e.call(TILE_IS_TRUE, &args![candidate, 0xfa3u32]).bool()
                || !e.call(TILE_IS_TRUE, &args![candidate, 0xfafu32]).bool())
        {
            candidate = 0;
        }
        e.call(LIST_CLEAR, &args![visited]);
        let found = e.mem.u32(found_trait.addr());
        if candidate != 0 {
            if candidate != current && found == 0xfc3 {
                e.call(TILE_PLAY_TILE_SOUND, &args![candidate, 0xfe8u32]);
            }
            if candidate != current && found == 0xfc7 {
                e.call(TILE_PLAY_TILE_SOUND, &args![candidate, 0xfcbu32]);
            }
            e.call(
                SET_CURRENT_FOCUS_TARGET,
                &args![this, candidate, found, 1u32],
            );
        } else {
            let taken = e
                .call(DO_GAMEPAD, &args![this, frontmost, event, 1.0f32])
                .bool();
            let over = e.get(this, InterfaceManager::pMouseOverTarget);
            if !taken && current != over {
                e.call(SET_CURRENT_FOCUS_TARGET, &args![this, current, found, 1u32]);
                if current != 0 {
                    e.call(TILE_PLAY_TILE_SOUND, &args![current, 0xfe8u32]);
                }
            } else if current != 0 && found == 0xfc7 {
                e.call(TILE_PLAY_TILE_SOUND, &args![current, 0xfcbu32]);
            }
        }
        e.call(SIMPLE_LIST_DESTROY, &args![visited]);
    });
}

/// Part of `DoGamepadEvent`, for a button event (not a direction): offers
/// it to every visible root menu with its button state, then clicks the tile
/// bound to the button (a trait per button) in the focused tile's menu.
fn gamepad_button_event(
    e: &mut Engine,
    this: Ptr<InterfaceManager>,
    frontmost: u32,
    root_tile: u32,
    event: i32,
) {
    let mut event = event;
    let root = e.call(GET_MENUS_ROOT, &args![this]).u32();
    let mut tail = e.call(GET_FIELD_AT_4, &args![root + 4]).u32();
    while tail != 0 {
        let child = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), tail);
            let root = e.call(GET_MENUS_ROOT, &args![this]).u32();
            let element = e.call(LIST_PREVIOUS_ELEMENT, &args![root + 4, slot]).u32();
            tail = e.mem.u32(slot.addr());
            e.mem.u32(element)
        });
        let menu = if child != 0 {
            e.call(TILE_GET_MENU, &args![child]).u32()
        } else {
            0
        };
        if menu != 0 && e.call(TILE_IS_VISIBLE, &args![child]).bool() {
            let state = e.call(CHECK_MENU_BUTTON, &args![event, 1u32]).i32();
            if e.call(DO_GAMEPAD, &args![this, menu, event, state as f32])
                .bool()
            {
                e.call(CLEAR_MENU_BUTTON, &args![event]);
                return;
            }
        }
    }
    let mut trait_id = match event {
        9 => 0xfdd,
        10 => 0xfde,
        0xb => 0xfdf,
        0xc => 0xfe0,
        0xe => 0xfe2,
        0xd => 0xfe1,
        0x10 => 0xfe4,
        0xf => 0xfe3,
        5 => 0xfe7,
        _ => 0,
    };
    let mut focus = e.call(GET_MOUSE_OVER_TARGET, &args![this]).u32();
    if focus == 0 {
        focus = e.get(this, InterfaceManager::pOverTileTarget);
    }
    if focus == 0 {
        focus = root_tile;
    }
    while focus != 0 && !e.call(TILE_IS_VISIBLE, &args![focus]).bool() {
        focus = e.call(TILE_PARENT, &args![focus]).u32();
    }
    if focus == 0 {
        return;
    }
    // The frame of `0070f6e0` (0x84 bytes): the trait at `EBP-0x68` (+0x1c).
    let (mut candidate, found) = e.with_stack(0x84, |e, frame| {
        let found_trait = Ptr::<()>::new(frame.addr() + 0x1c);
        let mut candidate = e
            .call(
                TILE_GET_FIRST_REF_COPY,
                &args![focus, trait_id, found_trait],
            )
            .u32();
        let mut walker = focus;
        while candidate == 0 && walker != 0 {
            walker = e.call(TILE_PARENT, &args![walker]).u32();
            if walker != 0 {
                candidate = e
                    .call(
                        TILE_GET_FIRST_REF_COPY,
                        &args![walker, trait_id, found_trait],
                    )
                    .u32();
            }
        }
        (candidate, e.mem.u32(found_trait.addr()))
    });
    if event == -2 {
        candidate = focus;
        event = 9;
        trait_id = 0xfdd;
    }
    let _ = trait_id;
    if candidate != 0
        && e.call(TILE_IS_VISIBLE, &args![candidate]).bool()
        && (found == 0xfc7 || found == 0xfc5)
    {
        if e.call(TILE_IS_TRUE, &args![candidate, 0xfafu32]).bool() {
            if found == 0xfc5 {
                let keys = e.get(this, InterfaceManager::iModifierKeys);
                e.set(this, InterfaceManager::iModifierKeys, keys | 4);
            }
            e.call(TILE_PLAY_TILE_SOUND, &args![candidate, 0xfcbu32]);
            tile_set_int(e, candidate, 0xfc7, 1);
            tile_set_int(e, candidate, 0xfc7, 0);
            let menu = e.call(TILE_GET_MENU, &args![candidate]).u32();
            let id = tile_menu_id(e, candidate);
            e.vcall(menu, 0xc, &args![id, candidate]);
            if found == 0xfc5 {
                let keys = e.get(this, InterfaceManager::iModifierKeys);
                e.set(this, InterfaceManager::iModifierKeys, keys & 0xfffb);
            }
            e.call(CLEAR_MENU_BUTTON, &args![event]);
            event = 0xa;
            if event != 0 {
                e.call(CLEAR_MENU_BUTTON, &args![6u32]);
            }
        } else {
            e.call(PLAY_MENU_SOUND, &args![2u32]);
        }
    } else if event == 9 && e.call(TILE_IS_TRUE, &args![focus, 0xfafu32]).bool() {
        e.call(TILE_PLAY_TILE_SOUND, &args![focus, 0xfcbu32]);
        tile_set_int(e, focus, 0xfc7, 1);
        tile_set_int(e, focus, 0xfc7, 0);
        let menu = e.call(TILE_GET_MENU, &args![focus]).u32();
        let id = tile_menu_id(e, focus);
        e.vcall(menu, 0xc, &args![id, focus]);
        e.call(CLEAR_MENU_BUTTON, &args![event]);
    }
    let _ = frontmost;
}

// Translated from 00710060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::GetTopGamepadButton` (Xbox PDB): the gamepad button
/// that should act now, with key-repeat handling. Needs a controller and no
/// mouse. The buttons 9, 10, 0xb, 0xc, 0x10, 6 and 0xe act on their press
/// (`CheckMenuButton` 2) or hold (1) and `*released` is set when the button
/// is not held (state 1 is the held state). The four directions 1 to 4
/// repeat: the first press sets `iLastGamepadEvent` and the repeat time
/// (`+0x160`, from the tick count and the delay setting at `011d8b38`, or
/// `011d8aa4` for the repeats); the shoulder buttons 0xf, 0x10, 0xd, 0xe and
/// the button 5 act on their press. `flag_a` and `flag_b` say whether the
/// tile under the pointer wants the repeat for the left/right (1, 2) or
/// up/down (3, 4) pairs. Returns 0 when no button acts.
pub fn interface_manager_get_top_gamepad_button(
    e: &mut Engine,
    this: Ptr<InterfaceManager>,
    flag_a: u8,
    flag_b: u8,
    released: Ptr,
) -> i32 {
    let mut result = 0i32;
    if !e.call(HAS_360_CONTROLLER_GETTER, &args![]).bool()
        || e.get(this, InterfaceManager::bShowMouse) != 0
    {
        return 0;
    }
    let owner = e.global::<u32>(CONTROLS_OWNER);
    e.call(CONTROLS_GET, &args![owner]);
    let mut reset_repeat = true;
    for button in [9i32, 10, 0xb, 0xc, 0x10, 6, 0xe] {
        let state = e.call(CHECK_MENU_BUTTON, &args![button, 1u32]).i32();
        if state == 2 || e.call(CHECK_MENU_BUTTON, &args![button, 1u32]).i32() == 1 {
            result = button;
            e.set(this, InterfaceManager::iLastGamepadEvent, 0);
            reset_repeat = false;
            let held = e.call(CHECK_MENU_BUTTON, &args![button, 1u32]).i32();
            e.mem.set_u8(released.addr(), (held == 1) as u8);
            return finish_top_gamepad_button(e, this, result, reset_repeat);
        }
    }
    // The four directions: pressed (state 2) and idle (state 0) flags.
    let pressed_right = e.call(CHECK_MENU_BUTTON, &args![4u32, 1u32]).i32() == 2;
    let pressed_left = e.call(CHECK_MENU_BUTTON, &args![3u32, 1u32]).i32() == 2;
    let pressed_up = e.call(CHECK_MENU_BUTTON, &args![1u32, 1u32]).i32() == 2;
    let pressed_down = e.call(CHECK_MENU_BUTTON, &args![2u32, 1u32]).i32() == 2;
    let idle_right = e.call(CHECK_MENU_BUTTON, &args![4u32, 1u32]).i32() == 0;
    let idle_left = e.call(CHECK_MENU_BUTTON, &args![3u32, 1u32]).i32() == 0;
    let idle_up = e.call(CHECK_MENU_BUTTON, &args![1u32, 1u32]).i32() == 0;
    let idle_down = e.call(CHECK_MENU_BUTTON, &args![2u32, 1u32]).i32() == 0;
    let pressed_11 = e.call(CHECK_MENU_BUTTON, &args![0x11u32, 1u32]).i32() == 2;
    let pressed_12 = e.call(CHECK_MENU_BUTTON, &args![0x12u32, 1u32]).i32() == 2;
    // Which single direction is pressed alone (and no other is pressed or
    // idle... the other three are neither pressed nor idle-released).
    if pressed_up && !pressed_right && !pressed_left && !idle_right && !idle_left {
        result = 1;
    } else if pressed_down && !pressed_right && !pressed_left && !idle_right && !idle_left {
        result = 2;
    } else if pressed_right && !pressed_up && !pressed_down && !idle_up && !idle_down {
        result = 4;
    } else if pressed_left && !pressed_up && !pressed_down && !idle_up && !idle_down {
        result = 3;
    } else if pressed_11 {
        result = 0x11;
    } else if pressed_12 {
        result = 0x12;
    }
    let last = e.get(this, InterfaceManager::iLastGamepadEvent);
    let mut held_direction = 0;
    if idle_up && last == 1 {
        held_direction = 1;
    } else if idle_down && last == 2 {
        held_direction = 2;
    } else if idle_right && last == 4 {
        held_direction = 4;
    } else if idle_left && last == 3 {
        held_direction = 3;
    }
    if result == 0
        && held_direction == 0
        && (idle_right as u32 + idle_left as u32 + idle_up as u32 + idle_down as u32) != 0
    {
        if idle_up {
            result = 1;
        } else if idle_down {
            result = 2;
        } else if idle_right {
            result = 4;
        } else if idle_left {
            result = 3;
        }
    }
    let wants_repeat = (flag_a != 0
        && (pressed_up as u32 + pressed_down as u32 + idle_up as u32 + idle_down as u32) != 0)
        || (flag_b != 0
            && (pressed_right as u32 + pressed_left as u32 + idle_right as u32 + idle_left as u32)
                != 0);
    let last = e.get(this, InterfaceManager::iLastGamepadEvent);
    if result == 0 || last == result {
        let last = e.get(this, InterfaceManager::iLastGamepadEvent);
        let tick = e.call(FADE_CLOCK_READ, &args![FADE_CLOCK]).u32();
        if last == held_direction && e.get(this, InterfaceManager::uGamepadRepeatStartTime) <= tick
        {
            result = last;
            if wants_repeat {
                set_repeat_time(e, this, REPEAT_DELAY_HOLDER_FAST);
            }
        } else if last == held_direction {
            result = 0;
        } else {
            result = 0;
            e.set(this, InterfaceManager::iLastGamepadEvent, 0);
        }
    } else {
        e.set(this, InterfaceManager::iLastGamepadEvent, result);
        if wants_repeat {
            set_repeat_time(e, this, REPEAT_DELAY_HOLDER_FIRST);
        } else {
            let tick = e.call(FADE_CLOCK_READ, &args![FADE_CLOCK]).u32();
            e.set(this, InterfaceManager::uGamepadRepeatStartTime, tick << 1);
        }
    }
    let any_direction = pressed_right
        || pressed_left
        || pressed_up
        || pressed_down
        || idle_right
        || idle_left
        || idle_up
        || idle_down;
    if any_direction {
        reset_repeat = false;
    }
    if result == 0 {
        let button_f = e.call(CHECK_MENU_BUTTON, &args![0xfu32, 1u32]).i32() == 2;
        let button_10 = e.call(CHECK_MENU_BUTTON, &args![0x10u32, 1u32]).i32() == 2;
        let button_d = e.call(CHECK_MENU_BUTTON, &args![0xdu32, 1u32]).i32() == 2;
        let button_e = e.call(CHECK_MENU_BUTTON, &args![0xeu32, 1u32]).i32() == 2;
        if button_f || button_10 || button_d || button_e {
            e.set(this, InterfaceManager::iLastGamepadEvent, 0);
            reset_repeat = false;
        }
        if button_f && !button_10 {
            result = 0xf;
        } else if button_10 && !button_f {
            result = 0x10;
        } else if button_d && !button_e {
            result = 0xd;
        } else if button_e && !button_d {
            result = 0xe;
        }
    }
    if result == 0 && e.call(CHECK_MENU_BUTTON, &args![5u32, 1u32]).i32() == 2 {
        result = 5;
        e.set(this, InterfaceManager::iLastGamepadEvent, 0);
        reset_repeat = false;
    }
    finish_top_gamepad_button(e, this, result, reset_repeat)
}

/// `uGamepadRepeatStartTime = trunc(float at holder + tick count)`.
fn set_repeat_time(e: &mut Engine, this: Ptr<InterfaceManager>, holder: u32) {
    let tick = e.call(FADE_CLOCK_READ, &args![FADE_CLOCK]).u32();
    let delay_ptr = e.call(FLOAT_HOLDER_GET, &args![holder]).u32();
    let delay = e.mem.f32(delay_ptr) as f64;
    let time = (delay + tick as f64) as i64 as u32;
    e.set(this, InterfaceManager::uGamepadRepeatStartTime, time);
}

/// The end of `GetTopGamepadButton`: with nothing acting, the repeat state
/// is reset (no last event, repeat time twice the tick count).
fn finish_top_gamepad_button(
    e: &mut Engine,
    this: Ptr<InterfaceManager>,
    result: i32,
    reset_repeat: bool,
) -> i32 {
    if reset_repeat {
        e.set(this, InterfaceManager::iLastGamepadEvent, 0);
        let tick = e.call(FADE_CLOCK_READ, &args![FADE_CLOCK]).u32();
        e.set(this, InterfaceManager::uGamepadRepeatStartTime, tick << 1);
    }
    result
}

// Translated from 00710880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Rebuilds the console's pick list (`+0x70`) by casting a ray through the
/// pointer position (`NiPick`) and collecting the references hit; the pick
/// index returns to 0. The list is only replaced when the references differ
/// from the ones already listed.
pub fn fn_00710880(e: &mut Engine, this: Ptr<InterfaceManager>) {
    let base = this.addr();
    // Locals: origin and direction (12 bytes each), the `NiPick` (0x34) and
    // the new list (8).
    e.with_stack(0x34, |e, pick| {
        e.with_stack(0xc, |e, origin| {
            e.with_stack(0xc, |e, direction| {
                e.with_stack(8, |e, list| {
                    e.call(MEMBER_CONSTRUCTOR_EMPTY, &args![origin]);
                    e.call(MEMBER_CONSTRUCTOR_EMPTY, &args![direction]);
                    e.call(NI_PICK_CONSTRUCT, &args![pick, 0u32, 8u32]);
                    e.call(SIMPLE_LIST_CONSTRUCT, &args![list]);
                    let mut same_as_before = true;
                    e.call(NI_PICK_SET_FLAG, &args![pick, 1u32]);
                    let root = e
                        .call(GET_PICK_ROOT, &args![e.global::<u32>(PICK_ROOT_OWNER)])
                        .u32();
                    e.call(NI_PICK_SET_ROOT, &args![pick, root]);
                    e.call(NI_PICK_SET_MODE, &args![pick, 0u32]);
                    let player = e.global::<u32>(PLAYER_SINGLETON);
                    let fov = fn_00710ab0(e, Ptr::new(player));
                    let scene = e.call(GET_SCENE_GRAPH, &args![]).u32();
                    e.call(
                        SCENE_GRAPH_SET_CAMERA_FOV,
                        &args![scene, fov, 0u32, 0u32, 0u32],
                    );
                    let y = e.call(FTOL, &args![e.mem.f32(base + 0x40) as f64]).i32();
                    let x = e.call(FTOL, &args![e.mem.f32(base + 0x38) as f64]).i32();
                    let scene = e.call(GET_SCENE_GRAPH, &args![]).u32();
                    let camera = e.call(SCENE_GRAPH_GET_CAMERA, &args![scene]).u32();
                    e.call(
                        CAMERA_BUILD_PICK_RAY,
                        &args![camera, x, y, origin, direction, 0u32],
                    );
                    if e.call(NI_PICK_PICK_OBJECTS, &args![pick, origin, direction, 0u32])
                        .bool()
                    {
                        let mut node = base + 0x70;
                        let mut index = 0u32;
                        loop {
                            let results = e.call(NI_PICK_GET_RESULTS, &args![pick]).u32();
                            let count = e.call(RESULTS_COUNT, &args![results]).u32();
                            if index >= count {
                                break;
                            }
                            let results = e.call(NI_PICK_GET_RESULTS, &args![pick]).u32();
                            let entry = e.call(RESULTS_GET, &args![results, index]).u32();
                            let object = e.call(RESULT_OBJECT, &args![entry]).u32();
                            let reference = e.call(FIND_REFERENCE_FOR_3D, &args![object]).u32();
                            let seen = e.with_stack(4, |e, slot| {
                                e.mem.set_u32(slot.addr(), reference);
                                let seen = e.call(LIST_CONTAINS, &args![list, slot]).bool();
                                if !seen {
                                    e.call(LIST_ADD, &args![list, slot]);
                                }
                                seen
                            });
                            if !seen {
                                let same_item = node == 0 || {
                                    let item = e.call(MEMBER_CONSTRUCTOR_EMPTY, &args![node]).u32();
                                    e.mem.u32(item) == reference
                                };
                                if !same_item {
                                    same_as_before = false;
                                } else if same_as_before && node != 0 {
                                    node = e.call(GET_FIELD_AT_4, &args![node]).u32();
                                }
                            }
                            index += 1;
                        }
                    }
                    e.call(LIST_CLEAR, &args![base + 0x70]);
                    e.set(this, InterfaceManager::iCurrentPickIndex, 0);
                    if !same_as_before {
                        let mut node = list.addr();
                        while node != 0 {
                            let item = e.call(MEMBER_CONSTRUCTOR_EMPTY, &args![node]).u32();
                            if e.mem.u32(item) == 0 {
                                break;
                            }
                            let item = e.call(MEMBER_CONSTRUCTOR_EMPTY, &args![node]).u32();
                            e.call(LIST_ADD, &args![base + 0x70, item]);
                            node = e.call(GET_FIELD_AT_4, &args![node]).u32();
                        }
                    }
                    e.call(LIST_CLEAR, &args![list]);
                    e.call(SIMPLE_LIST_DESTROY, &args![list]);
                    e.call(NI_PICK_DESTRUCT, &args![pick]);
                });
            });
        });
    });
}

// Translated from 00710ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the `float` at `+0x670` of the object (called on the player; the
/// pick code passes it to the camera as the field of view).
pub fn fn_00710ab0(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x670)
}

// Translated from 00710ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::DisplayCurrentPickRef` (Xbox PDB): the debug text
/// for the reference the console is looking at. Walks the pick list (`+0x70`)
/// to entry `iCurrentPickIndex`; with no entry clears `pPickRef` and prints
/// an empty line, otherwise sets `pPickRef` and prints its name and form id,
/// then (with `bFullHelp`) its script, current furniture, owner, count, the
/// furniture markers, lock, teleport door and flags. Lines the previous call
/// printed below the new last line are blanked.
pub fn interface_manager_display_current_pick_ref(e: &mut Engine, this: Ptr<InterfaceManager>) {
    let base = this.addr();
    let mut pick = 0u32;
    let line_height = debug_text_line_height(e);
    let start_y = setting_value(e, DEBUG_TEXT_TOP_SETTING);
    let mut y = start_y + 0x1e;
    let mut index = 0i32;
    let mut node = base + 0x70;
    while node != 0 {
        let item = e.call(MEMBER_CONSTRUCTOR_EMPTY, &args![node]).u32();
        if e.mem.u32(item) == 0 || e.get(this, InterfaceManager::iCurrentPickIndex) < index {
            break;
        }
        pick = e.mem.u32(item);
        index += 1;
        node = e.call(GET_FIELD_AT_4, &args![node]).u32();
    }
    if pick == 0 {
        e.set(this, InterfaceManager::pPickRef, 0);
        e.call(SET_PICK_REF_DISPLAY, &args![this, 0u32]);
        // The code works the line height out again here and does not use it.
        debug_text_line_height(e);
        let top = setting_value(e, DEBUG_TEXT_TOP_SETTING);
        debug_text_print_literal(e, EMPTY_TEXT, top);
    } else {
        with_string(e, |e, text| {
            e.call(SET_PICK_REF_DISPLAY, &args![this, pick]);
            e.set(this, InterfaceManager::pPickRef, pick);
            let form_id = e.call(FORM_ID_GETTER, &args![pick]).u32();
            let name = e.call(REFR_NAME, &args![pick]).u32();
            e.call(
                FORMAT_INTO_STRING,
                &args![text, FORMAT_NAME_AND_ID, name, form_id],
            );
            debug_text_print(e, text, start_y);
            if e.get(this, InterfaceManager::bFullHelp) != 0 {
                pick_ref_full_help(e, this, text, pick, &mut y, line_height);
            }
        });
    }
    let last = y;
    let bottom = e.global::<i32>(DEBUG_TEXT_BOTTOM);
    while y < bottom {
        debug_text_print_literal(e, EMPTY_TEXT, y);
        y += line_height;
    }
    e.set_global(DEBUG_TEXT_BOTTOM, last);
}

/// The word a setting holder keeps (`0043d4d0` returns a pointer to it).
fn setting_value(e: &mut Engine, holder: u32) -> i32 {
    let at = e.call(GET_SETTING_VALUE, &args![holder]).u32();
    e.mem.u32(at) as i32
}

/// Line height of the debug text: the height of the font the setting
/// `011f33c8` minus one selects, plus 3.
fn debug_text_line_height(e: &mut Engine) -> i32 {
    let level = setting_value(e, FONT_LEVEL_SETTING);
    let fonts = e.call(FONT_MANAGER_GET, &args![]).u32();
    let font = e
        .call(FONT_MANAGER_GET_FONT, &args![fonts, level - 1])
        .u32();
    let height = e.call(FONT_HEIGHT, &args![font]).f64();
    e.call(FTOL, &args![height]).i32() + 3
}

/// Runs `body` with a `BSStringT` local (`004037b0` / `004037d0`).
fn with_string<R>(e: &mut Engine, body: impl FnOnce(&mut Engine, Ptr) -> R) -> R {
    e.with_stack(8, |e, text| {
        e.call(STRING_CONSTRUCT, &args![text]);
        let result = body(e, text);
        e.call(STRING_DESTROY, &args![text]);
        result
    })
}

/// Prints the text of the string `text` at column `640.0` (the global at
/// `0103a1c4`) and row `y`: `DebugText::Instance(1)->Print(text, x, y, 2, -1,
/// -1.0, 0, 0)`.
fn debug_text_print(e: &mut Engine, text: Ptr, y: i32) {
    let chars = ni_pointer_get(e, text.addr());
    debug_text_print_literal(e, chars, y);
}

fn debug_text_print_literal(e: &mut Engine, chars: u32, y: i32) {
    let x: f32 = e.global(DEBUG_TEXT_X);
    let colour: f32 = e.global(CURSOR_DIRECTION_Y);
    let instance = e.call(DEBUG_TEXT_INSTANCE, &args![1u32]).u32();
    e.call(
        DEBUG_TEXT_PRINT,
        &args![instance, chars, x, y as f32, 2u32, -1i32, colour, 0u32, 0u32],
    );
}

/// One formatted debug line: `format` fills `text`, then it is printed at
/// `*y`, and `*y` moves down a line.
fn pick_line(e: &mut Engine, text: Ptr, y: &mut i32, line_height: i32, words: &[u32]) {
    let mut call = vec![text.addr()];
    call.extend_from_slice(words);
    e.call(FORMAT_INTO_STRING, &call);
    debug_text_print(e, text, *y);
    *y += line_height;
}

/// The `bFullHelp` part of `DisplayCurrentPickRef`.
fn pick_ref_full_help(
    e: &mut Engine,
    this: Ptr<InterfaceManager>,
    text: Ptr,
    pick: u32,
    y: &mut i32,
    line_height: i32,
) {
    let extra = e.call(REFR_EXTRA_DATA_LIST, &args![pick]).u32();
    if e.call(EXTRA_GET_SCRIPT, &args![extra]).u32() != 0 {
        let extra = e.call(REFR_EXTRA_DATA_LIST, &args![pick]).u32();
        let script = e.call(EXTRA_GET_SCRIPT, &args![extra]).u32();
        let extra = e.call(REFR_EXTRA_DATA_LIST, &args![pick]).u32();
        let script_again = e.call(EXTRA_GET_SCRIPT, &args![extra]).u32();
        let id = e.call(FORM_ID_GETTER, &args![script_again]).u32();
        let name = e.vcall(script, 0x130, &args![]).u32();
        pick_line(
            e,
            text,
            y,
            line_height,
            &args![SCRIPT_NAME_FORMAT, name, id],
        );
    }
    if e.vcall(pick, 0x100, &args![]).bool() {
        let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![pick]).u32();
        if process != 0 {
            let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![pick]).u32();
            if e.vcall(process, 0x4c8, &args![]).u32() != 0 {
                let first = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![pick]).u32();
                let second = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![pick]).u32();
                let furniture_a = e.vcall(second, 0x4c8, &args![]).u32();
                let furniture_b = e.vcall(first, 0x4c8, &args![]).u32();
                let id = e.call(FORM_ID_GETTER, &args![furniture_b]).u32();
                let name = e.vcall(furniture_a, 0x130, &args![]).u32();
                pick_line(e, text, y, line_height, &args![FURNITURE_FORMAT, name, id]);
            }
        }
    }
    if e.call(REFR_GET_OWNER, &args![pick]).u32() != 0 {
        let owner = e.call(REFR_GET_OWNER, &args![pick]).u32();
        let owner_again = e.call(REFR_GET_OWNER, &args![pick]).u32();
        let id = e.call(FORM_ID_GETTER, &args![owner_again]).u32();
        let name = e.vcall(owner, 0x130, &args![]).u32();
        pick_line(e, text, y, line_height, &args![OWNER_FORMAT, name, id]);
    }
    let extra = e.call(REFR_EXTRA_DATA_LIST, &args![pick]).u32();
    if e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16 != 1 {
        let extra = e.call(REFR_EXTRA_DATA_LIST, &args![pick]).u32();
        let count = e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16 as i32;
        pick_line(e, text, y, line_height, &args![COUNT_FORMAT, count]);
    }
    if e.vcall(pick, 0x100, &args![]).bool() {
        let actor = e
            .call(
                DYNAMIC_CAST,
                &args![pick, 0u32, FROM_TYPE, ACTOR_TYPE, 0u32],
            )
            .u32();
        if e.vcall(actor, 0x214, &args![]).u32() != 0 {
            let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
            if e.vcall(process, 0x4c8, &args![]).u32() == 0 {
                let process = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
                let word = e.vcall(process, 0x4d0, &args![]).u32();
                pick_line(e, text, y, line_height, &args![ACTOR_PACKAGE_FORMAT, word]);
            } else {
                let a = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
                let b = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
                let c = e.call(GET_SAVED_ACQUIRE_OBJECT, &args![actor]).u32();
                let furniture = e.vcall(c, 0x4c8, &args![]).u32();
                let word = e.vcall(a, 0x4d0, &args![]).u32();
                let other = e.vcall(b, 0x4c8, &args![]).u32();
                let id = e.call(FORM_ID_GETTER, &args![other]).u32();
                let name = e.vcall(furniture, 0x130, &args![]).u32();
                pick_line(
                    e,
                    text,
                    y,
                    line_height,
                    &args![ACTOR_FURNITURE_FORMAT, name, id, word],
                );
            }
        }
    }
    if e.call(REFR_IS_FURNITURE, &args![pick]).bool() {
        pick_furniture_lines(e, text, pick, y, line_height);
    }
    pick_lock_and_door_lines(e, this, text, pick, y, line_height);
}

/// The furniture part: the heading angle and one line for each marker.
fn pick_furniture_lines(e: &mut Engine, text: Ptr, pick: u32, y: &mut i32, line_height: i32) {
    let direction = e.call(REFR_GET_ROTATION, &args![pick]).u32();
    let z = e.mem.f32(direction + 8) as f64;
    let factor: f64 = e.global(RADIANS_TO_DEGREES);
    let degrees = e.call(FTOL, &args![z * factor]).i32();
    pick_line(e, text, y, line_height, &args![HEADING_FORMAT, degrees]);
    e.with_stack(0x14, |e, marker| {
        e.call(MARKER_CONSTRUCT, &args![marker, 0xffu32]);
        let mut index = 0i32;
        while e
            .call(REFR_GET_MARKER_AT_INDEX, &args![pick, index, marker])
            .bool()
        {
            let furniture = e.call(REFR_GET_FURNITURE_DATA, &args![pick]).u32();
            let marker_kind = e.call(MARKER_KIND, &args![marker]).u32();
            let enabled = e
                .call(FURNITURE_GET_MARKER_ENABLED, &args![furniture, index])
                .bool();
            let is_sit =
                |e: &mut Engine| e.call(FURNITURE_IS_SIT_MARKER, &args![marker_kind]).bool();
            if !enabled {
                let kind = if is_sit(e) { SIT_TEXT } else { SLEEP_TEXT };
                pick_line(
                    e,
                    text,
                    y,
                    line_height,
                    &args![DISABLED_MARKER_FORMAT, kind, marker_kind],
                );
            } else {
                let scale = e
                    .call(REFR_GET_SCALE, &args![e.global::<u32>(PLAYER_SINGLETON)])
                    .f32();
                let mut delta = [0f32; 3];
                e.with_stack(12, |e, out| {
                    let marker_pos = e.call(MARKER_KIND, &args![marker]).u32();
                    e.call(
                        FURNITURE_GET_MARKER_TARGET_OFFSET,
                        &args![furniture, out, marker_pos, scale],
                    );
                    for (i, d) in delta.iter_mut().enumerate() {
                        *d = e.mem.f32(out.addr() + 4 * i as u32);
                    }
                });
                let used = if e
                    .call(REFR_GET_MARKER_USED, &args![pick, index, 1u32])
                    .bool()
                {
                    USED_TEXT
                } else {
                    UNUSED_TEXT
                };
                let kind = if is_sit(e) { SIT_TEXT } else { SLEEP_TEXT };
                let angle = e
                    .call(FURNITURE_GET_MARKER_ANGLE, &args![furniture, marker_kind])
                    .f64();
                let degrees = e
                    .call(FTOL, &args![angle * e.global::<f64>(RADIANS_TO_DEGREES)])
                    .i32();
                pick_line(
                    e,
                    text,
                    y,
                    line_height,
                    &args![
                        MARKER_FORMAT,
                        kind,
                        marker_kind,
                        delta[0] as f64,
                        delta[1] as f64,
                        delta[2] as f64,
                        degrees,
                        used
                    ],
                );
            }
            index += 1;
        }
    });
}

/// The lock, teleport door and flag lines.
fn pick_lock_and_door_lines(
    e: &mut Engine,
    _this: Ptr<InterfaceManager>,
    text: Ptr,
    pick: u32,
    y: &mut i32,
    line_height: i32,
) {
    if e.call(REFR_GET_LOCK, &args![pick]).u32() != 0 {
        let lock = e.call(REFR_GET_LOCK, &args![pick]).u32();
        let locked = if e.call(LOCK_IS_LOCKED, &args![lock]).bool() {
            LOCKED_TEXT
        } else {
            UNLOCKED_TEXT
        };
        let level = e.call(LOCK_GET_LEVEL, &args![lock, pick]).u32();
        let holder = e.mem.u32(LOCK_LEVEL_TABLE + level * 4);
        let level_text = e.call(STRING_HOLDER_GET, &args![holder]).u32();
        pick_line(
            e,
            text,
            y,
            line_height,
            &args![LOCK_FORMAT, level_text, locked],
        );
        let key = e.mem.u32(lock + 4);
        if key != 0 {
            let name = e
                .call(MAP_MARKER_GET_LOCATION_NAME, &args![key + 0x30])
                .u32();
            pick_line(e, text, y, line_height, &args![KEY_FORMAT, name]);
        }
    }
    if e.call(REFR_GET_TELEPORT_DATA, &args![pick]).u32() != 0 {
        let door = e.call(REFR_GET_TELEPORT_DATA, &args![pick]).u32();
        let cell = e.call(DOOR_TELEPORT_GET_CELL, &args![door]).u32();
        let reference = ni_pointer_get(e, door);
        let mut cell_name = UNKNOWN_TEXT;
        let mut door_name = UNKNOWN_TEXT;
        let mut named = false;
        if cell != 0 {
            let name = e
                .call(MAP_MARKER_GET_LOCATION_NAME, &args![cell + 0x18])
                .u32();
            if name != 0 {
                cell_name = e
                    .call(MAP_MARKER_GET_LOCATION_NAME, &args![cell + 0x18])
                    .u32();
                named = true;
            }
        }
        if !named && reference != 0 {
            let extra = e.call(REFR_EXTRA_DATA_LIST, &args![reference]).u32();
            if e.call(EXTRA_IS_PERSISTENT, &args![extra]).u32() != 0 {
                cell_name = PERSISTENT_TEXT;
            }
        }
        if reference != 0 {
            let name = e.call(REFR_NAME, &args![reference]).u32();
            if name != 0 {
                door_name = e.call(REFR_NAME, &args![reference]).u32();
            }
        }
        pick_line(
            e,
            text,
            y,
            line_height,
            &args![TELEPORT_FORMAT, cell_name, door_name],
        );
    }
    let flags = e.call(REFR_FLAGS, &args![pick]).u32();
    if flags != 0 {
        e.with_stack(0x100, |e, buffer| {
            e.call(STRING_COPY, &args![buffer, 0x100u32, FLAGS_PREFIX]);
            for (bit, suffix) in [
                (1u32, FLAG_TEXT_1),
                (2, FLAG_TEXT_2),
                (4, FLAG_TEXT_4),
                (8, FLAG_TEXT_8),
            ] {
                if flags & bit != 0 {
                    e.call(STRING_APPEND, &args![buffer, 0x100u32, suffix]);
                }
            }
            let line = e.call(TRIM_TEXT, &args![buffer, 0u32]).u32();
            pick_line(e, text, y, line_height, &args![line]);
        });
    }
}

// Translated from 007118d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the cursor tile's node with the mouse or the controller's cursor
/// stick (only when the cursor is wanted: no controller or the mouse in
/// use, and not while the console steals it). The movement comes from the
/// two mouse axes (or the stick axes scaled by the setting at `011d8c5c`),
/// speeded up by the acceleration settings, converted to screen units, kept
/// inside the screen and stored in the node's translation; when the position
/// changed, `sCursorPos`, `bMouseInMotion` and the real cursor position
/// (`+0x38`, `+0x3c`, `+0x40`) are updated, and the node is updated.
pub fn fn_007118d0(e: &mut Engine, this: Ptr<InterfaceManager>) {
    let controller = e.call(HAS_360_CONTROLLER_GETTER, &args![]).bool();
    if controller && e.get(this, InterfaceManager::bShowMouse) == 0 {
        let console = e.call(MENU_CONSOLE_INSTANCE, &args![1u32]).u32();
        if !e.call(CONSOLE_PRIMARY_DOWN, &args![console]).bool() {
            return;
        }
    }
    with_scope_guard(e, 0xc51, |e| {
        let owner = e.global::<u32>(CONTROLS_OWNER);
        let controls = e.call(CONTROLS_GET, &args![owner]).u32();
        let controller = e.call(HAS_360_CONTROLLER_GETTER, &args![]).bool();
        if controller && e.get(this, InterfaceManager::bShowMouse) == 0 {
            return;
        }
        if e.call(CONTROLS_FLAG_00711E00, &args![controls]).u32() != 0 {
            return;
        }
        let cursor = e.get(this, InterfaceManager::pCursor);
        let node = e.call(TILE_IMAGE_NODE, &args![cursor]).u32();
        e.call(NODE_SET_FLAG, &args![node, 0u32]);
        let cursor = e.get(this, InterfaceManager::pCursor);
        tile_set_int(e, cursor, 0xfa3, 1);
        let (mut mouse_x, mut mouse_y);
        if e.call(HAS_360_CONTROLLER_GETTER, &args![]).bool()
            && e.get(this, InterfaceManager::bShowMouse) == 0
        {
            mouse_x = e
                .call(CONTROLS_AXIS_00A23390, &args![controls, 0u32, 7u32])
                .i32();
            mouse_y = e
                .call(CONTROLS_AXIS_00A23390, &args![controls, 0u32, 8u32])
                .i32();
            let stick_scale = float_holder_value(e, STICK_SCALE_HOLDER);
            mouse_x = e.call(FTOL, &args![stick_scale * mouse_x as f64]).i32();
            let stick_scale = float_holder_value(e, STICK_SCALE_HOLDER);
            mouse_y = e.call(FTOL, &args![stick_scale * -(mouse_y as f64)]).i32();
        } else {
            mouse_x = e
                .call(CONTROLS_QUERY_00A239E0, &args![controls, 1u32])
                .i32();
            mouse_y = e
                .call(CONTROLS_QUERY_00A239E0, &args![controls, 2u32])
                .i32();
        }
        let mut speed = 1.0f32;
        if mouse_x != 0 || mouse_y != 0 {
            if e.call(MOUSE_SETTING_00711DA0, &args![controls]).f64() > 0.0 {
                let base = e.call(MOUSE_SETTING_00711DA0, &args![controls]).f64();
                let step = e.call(MOUSE_SETTING_00711D80, &args![controls]).f64();
                let level = e.call(MOUSE_LEVEL_00711D60, &args![controls]).u8() as i8 as i32;
                let twenty: f64 = e.global(MOUSE_LEVEL_DIVISOR);
                speed = ((level as f64 / twenty) * step + base) as f32;
            }
            if e.call(MOUSE_SETTING_00711DC0, &args![controls]).f64() > 0.0 {
                let bias = e.call(MOUSE_SETTING_00711DE0, &args![controls]).f64();
                let abs_x = e.call(ABS, &args![mouse_x]).i32();
                let abs_y = e.call(ABS, &args![mouse_y]).i32();
                let total = abs_x.wrapping_add(abs_y);
                let divisor = e.call(MOUSE_SETTING_00711DC0, &args![controls]).f64();
                speed = ((total as f64 / divisor + bias) * speed as f64) as f32;
            }
        }
        let cursor = e.get(this, InterfaceManager::pCursor);
        let node = e.call(TILE_IMAGE_NODE, &args![cursor]).u32();
        let translation = e.call(NODE_TRANSLATION, &args![node]).u32();
        let old = [
            e.mem.f32(translation),
            e.mem.f32(translation + 4),
            e.mem.f32(translation + 8),
        ];
        let width = e.call(GET_SCREEN_WIDTH, &args![]).f64();
        let horizontal: f64 = e.global(CURSOR_HORIZONTAL_SCALE);
        let moved_x =
            (((width * mouse_x as f64) / horizontal) * speed as f64 + old[0] as f64) as f32;
        let half_width = clamp_edge(e, moved_x, EDGE_RIGHT, EDGE_LEFT);
        let tilt = tile_get_float(e, e.get(this, InterfaceManager::pCursor), 0xfad);
        let moved_y_part = tilt * e.global::<f64>(CURSOR_TILT_SCALE);
        let height = e.call(GET_SCREEN_HEIGHT, &args![]).f64();
        let vertical: f64 = e.global(CURSOR_VERTICAL_SCALE);
        let moved_z =
            (old[2] as f64 - ((height * mouse_y as f64) / vertical) * speed as f64) as f32;
        let z = clamp_edge(e, moved_z, EDGE_TOP, EDGE_BOTTOM);
        let position = [half_width, moved_y_part as f32, z];
        let cursor = e.get(this, InterfaceManager::pCursor);
        let node = e.call(TILE_IMAGE_NODE, &args![cursor]).u32();
        e.with_stack(12, |e, vector| {
            for (i, v) in position.iter().enumerate() {
                e.mem.set_f32(vector.addr() + 4 * i as u32, *v);
            }
            e.call(NODE_SET_TRANSLATE_VECTOR, &args![node, vector]);
        });
        if e.mem.f32(this.addr() + 0x2c) != position[0]
            || e.mem.f32(this.addr() + 0x34) != position[2]
        {
            for (i, v) in position.iter().enumerate() {
                e.mem.set_f32(this.addr() + 0x2c + 4 * i as u32, *v);
            }
            e.set(this, InterfaceManager::bMouseInMotion, 1);
            let desktop_width = e.call(GET_DESKTOP_WIDTH, &args![]).f64();
            let width = e.call(GET_SCREEN_WIDTH, &args![]).f64();
            let cursor_x = e.mem.f32(this.addr() + 0x2c) as f64;
            let desktop_width_again = e.call(GET_DESKTOP_WIDTH, &args![]).f64();
            let two: f64 = e.global(TWO);
            let real_x = (desktop_width_again / two + (desktop_width / width) * cursor_x) as f32;
            e.mem.set_f32(this.addr() + 0x38, real_x);
            let cursor = e.get(this, InterfaceManager::pCursor);
            let tilt = tile_get_float(e, cursor, 0xfad);
            e.mem.set_f32(
                this.addr() + 0x3c,
                (tilt * e.global::<f64>(CURSOR_TILT_SCALE)) as f32,
            );
            let desktop_height = e.call(GET_DESKTOP_HEIGHT, &args![]).f64();
            let half_height = desktop_height / e.global::<f64>(TWO);
            let desktop_height_again = e.call(GET_DESKTOP_HEIGHT, &args![]).f64();
            let screen_height = e.call(GET_SCREEN_HEIGHT, &args![]).f64();
            let cursor_z = e.mem.f32(this.addr() + 0x34) as f64;
            let real_z = (half_height - (desktop_height_again / screen_height) * cursor_z) as f32;
            e.mem.set_f32(this.addr() + 0x40, real_z);
        }
        e.with_stack(12, |e, update_data| {
            e.call(
                NI_UPDATE_DATA_CONSTRUCT,
                &args![update_data, 0.0f32, 0u32, 0u32],
            );
            let cursor = e.get(this, InterfaceManager::pCursor);
            let node = e.call(TILE_IMAGE_NODE, &args![cursor]).u32();
            e.call(NODE_UPDATE, &args![node, update_data]);
        });
    });
}

/// `*holder` as a `float` widened to `double` (`00403e20` returns a pointer
/// to the float).
fn float_holder_value(e: &mut Engine, holder: u32) -> f64 {
    let at = e.call(FLOAT_HOLDER_GET, &args![holder]).u32();
    e.mem.f32(at) as f64
}

/// Clamps a cursor coordinate to the screen: `max(min(value, edge_high - 2),
/// edge_low - 2)` for x and `+ 1` for z, with the two clamp helpers
/// `0040ebd0` (min) and `00404010` (max).
fn clamp_edge(e: &mut Engine, value: f32, high: u32, low: u32) -> f32 {
    let high_edge = e.call(high, &args![]).f64();
    let margin: f64 = if high == EDGE_RIGHT {
        -e.global::<f64>(TWO)
    } else {
        e.global::<f64>(ONE_DOUBLE)
    };
    let upper = (high_edge + margin) as f32;
    let min = e.call(FLOAT_MIN, &args![upper, value]).f32();
    let low_edge = e.call(low, &args![]).f64();
    let lower = (low_edge + margin) as f32;
    e.call(FLOAT_MAX, &args![lower, min]).f32()
}

/// The x87 test the fade code makes after `FCOMP`: true unless `value` is
/// below `limit` (an unordered compare counts as not below).
fn not_below(value: f64, limit: f64) -> bool {
    value.partial_cmp(&limit) != Some(std::cmp::Ordering::Less)
}

// Translated from 00712450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::RecursiveFade` (Xbox PDB): fades a scene graph node
/// and its children. `fade` is the fade being applied and `base_alpha` the
/// alpha handed down; a tile the node belongs to may replace the alpha with
/// its own (trait `0xfa9` over 255) or, when its trait `0xfab` is set, only
/// shows or hides itself (trait `0xfa3`) and stops. A geometry node with a
/// property of type 3 gets the faded value; the children are visited unless
/// the tile's trait `0xfaa` is `9000.0`.
pub fn interface_manager_recursive_fade(
    e: &mut Engine,
    _this: Ptr<InterfaceManager>,
    node: u32,
    fade: f32,
    base_alpha: f32,
) {
    with_scope_guard(e, 0xd2b, |e| {
        if node == 0 {
            return;
        }
        // Virtual slot `0xc` is the node's own `NiNode` view (null for
        // leaves); it is asked again below for the children.
        let mut owner = e.vcall(node, 0xc, &args![]).u32();
        let mut tile = e.call(TILE_FROM_NODE, &args![owner]).u32();
        while tile == 0 && owner != 0 {
            owner = e.call(NODE_PARENT, &args![owner]).u32();
            tile = e.call(TILE_FROM_NODE, &args![owner]).u32();
        }
        let children = e.vcall(node, 0xc, &args![]).u32();
        let mut alpha = base_alpha;
        if tile != 0 {
            if e.call(TILE_IS_TRUE, &args![tile, 0xfabu32]).bool() {
                let magnitude = e.call(FLOAT_WRAPPER_00408840, &args![fade]).f64();
                let shown = not_below(magnitude, e.global::<f64>(ONE_DOUBLE));
                tile_set_int(e, tile, 0xfa3, shown as u32);
                return;
            }
            let opacity = tile_get_float(e, tile, 0xfa9);
            alpha = (opacity / e.global::<f64>(TWO_FIFTY_FIVE)) as f32;
        }
        // Virtual slot `0x1c` is the node's geometry view.
        let geometry = e.vcall(node, 0x1c, &args![]).u32();
        if geometry != 0 {
            let magnitude = e.call(FLOAT_WRAPPER_00408840, &args![fade]).f64();
            let faded = if not_below(magnitude, e.global::<f64>(FADE_MINIMUM)) {
                let product = (fade as f64 * alpha as f64) as f32;
                let least = e.call(FLOAT_MIN, &args![product, alpha]).f32();
                e.call(FLOAT_MAX, &args![0.0f32, least]).f32()
            } else {
                0.0
            };
            let kind = e.call(PROPERTY_TYPE_THREE, &args![]).u32();
            let property = e.call(NODE_GET_PROPERTY, &args![geometry, kind]).u32();
            if property != 0 {
                e.call(PROPERTY_SET_FADE, &args![property, faded]);
            }
        }
        if children != 0 {
            if tile != 0 {
                let stop = tile_get_float(e, tile, 0xfaa);
                if stop == e.global::<f64>(FADE_STOP_VALUE) {
                    return;
                }
            }
            let mut index = 0u32;
            while index < e.call(NODE_CHILD_COUNT, &args![children]).u32() {
                let child = e.call(NODE_CHILD_AT, &args![children, index]).u32();
                interface_manager_recursive_fade(e, _this, child, fade, alpha);
                index += 1;
            }
        }
    });
}

// Translated from 007126c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Picks the tile under the pointer by casting a ray through the scene
/// graph (the interface scene graph, or the rendered menu's when one is
/// shown) and walking the results: each hit object is followed up to the
/// tile it belongs to, which must accept events, pass its pick rectangle
/// test when it has one, answer its own hit test (virtual slot `0x14`) and
/// have trait `0xfaf`. Unless `keep_exact_tile` is set a tile without trait
/// `0xfaa` is replaced by the first ancestor that has it. Returns the tile
/// (null when nothing was hit) and stores the character index of a text
/// tile (type `0x387`) in `iCharHit` (`0xffff` otherwise). The cursor root
/// is hidden during the pick.
pub fn fn_007126c0(e: &mut Engine, this: Ptr<InterfaceManager>, keep_exact_tile: u8) -> u32 {
    with_scope_guard(e, 0xd5c, |e| {
        let cursor_root = e.call(CURSOR_ROOT_GETTER, &args![this]).u32();
        e.call(NODE_SET_FLAG, &args![cursor_root, 1u32]);
        let mut root = e.call(GET_FRAME_SCENE_NODE, &args![this]).u32();
        let menu = e.get(this, InterfaceManager::pCurrentRenderedMenu);
        if menu != 0 {
            e.vcall(menu, 0x1c, &args![]);
            if e.call(MENU_FLAG_QUERY_004A4040, &args![]).bool() {
                let top = e.call(GET_ENTER_STACK_TOP, &args![this]).u32();
                let rendered = e.call(TILE_GET_MENU_BY_CLASS, &args![top]).u32();
                root = e.call(TILE_IMAGE_NODE, &args![rendered]).u32();
            }
        }
        e.with_stack(0x34, |e, pick| {
            e.call(NI_PICK_CONSTRUCT, &args![pick, 0u32, 8u32]);
            let found = pick_tile_search(e, this, keep_exact_tile != 0, root, pick);
            e.call(NI_PICK_DESTRUCT, &args![pick]);
            found
        })
    })
}

/// The body of [`fn_007126c0`] between the construction and the
/// destruction of its `NiPick`.
fn pick_tile_search(
    e: &mut Engine,
    this: Ptr<InterfaceManager>,
    keep_exact_tile: bool,
    root: u32,
    pick: Ptr,
) -> u32 {
    e.call(NI_PICK_SET_BYTE_10, &args![pick, 0u32]);
    e.call(NI_PICK_SET_FLAG, &args![pick, 1u32]);
    e.call(NI_PICK_SET_ROOT, &args![pick, root]);
    let base = this.addr();
    e.with_stack(0xc, |e, origin| {
        e.with_stack(0xc, |e, direction| {
            e.call(MEMBER_CONSTRUCTOR_EMPTY, &args![origin]);
            e.call(MEMBER_CONSTRUCTOR_EMPTY, &args![direction]);
            // The pointer position (`+0x38`, `+0x40`) kept inside the
            // desktop.
            let desktop_width = e.call(GET_DESKTOP_WIDTH, &args![]).f32();
            let x = e.mem.f32(base + 0x38);
            let x = e.call(FLOAT_MAX, &args![0.0f32, x]).f32();
            let mut x = e.call(FLOAT_MIN, &args![x, desktop_width]).f32();
            let desktop_height = e.call(GET_DESKTOP_HEIGHT, &args![]).f32();
            let y = e.mem.f32(base + 0x40);
            let y = e.call(FLOAT_MAX, &args![0.0f32, y]).f32();
            let mut y = e.call(FLOAT_MIN, &args![y, desktop_height]).f32();
            let setting = e.call(SETTING_BYTE, &args![SETTING_HOLDER_011DB2CC]).u32();
            if e.mem.u8(setting) != 0 {
                if e.get(this, InterfaceManager::bMouseOverRenderedMenu) != 0 {
                    x = e.get(this, InterfaceManager::field_4ac);
                    y = e.get(this, InterfaceManager::field_4b0);
                } else if e.call(IS_CURRENT_RENDERED_MENU_TOPMOST, &args![]).bool() {
                    return 0;
                }
            }
            if e.global::<u8>(VIEWPORT_RECT_ENABLED) != 0 {
                e.with_stack(0x10, |e, rect| {
                    let rect = e
                        .call(RECT_CONSTRUCT, &args![rect, 0.0f32, 1.0f32, 1.0f32, 0.0f32])
                        .u32();
                    let scene = e.call(GET_FRAME_SCENE_NODE, &args![this]).u32();
                    let camera = e.call(SCENE_GRAPH_GET_CAMERA, &args![scene]).u32();
                    fn_00712e60(e, Ptr::new(camera), rect);
                });
            }
            let row = e.call(FTOL, &args![y as f64]).i32();
            let column = e.call(FTOL, &args![x as f64]).i32();
            let scene = e.call(GET_FRAME_SCENE_NODE, &args![this]).u32();
            let camera = e.call(SCENE_GRAPH_GET_CAMERA, &args![scene]).u32();
            e.call(
                CAMERA_BUILD_PICK_RAY,
                &args![camera, column, row, origin, direction, 0u32],
            );
            let mut found = 0u32;
            if e.call(NI_PICK_PICK_OBJECTS, &args![pick, origin, direction, 0u32])
                .bool()
            {
                found = pick_tile_results(e, this, keep_exact_tile, pick, x, y);
            }
            // Show the cursor again and tell the rendered menu.
            let cursor_root = e.call(CURSOR_ROOT_GETTER, &args![this]).u32();
            e.call(NODE_SET_FLAG, &args![cursor_root, 0u32]);
            let menu = e.get(this, InterfaceManager::pCurrentRenderedMenu);
            if menu != 0 {
                e.vcall(menu, 0x20, &args![]);
            }
            found
        })
    })
}

/// The walk over the pick results of [`fn_007126c0`]: returns the first
/// tile that qualifies, or null.
fn pick_tile_results(
    e: &mut Engine,
    this: Ptr<InterfaceManager>,
    keep_exact_tile: bool,
    pick: Ptr,
    x: f32,
    y: f32,
) -> u32 {
    let mut index = 0i32;
    loop {
        let results = e.call(NI_PICK_GET_RESULTS, &args![pick]).u32();
        let count = e.call(RESULTS_COUNT, &args![results]).i32();
        if index >= count {
            return 0;
        }
        let results = e.call(NI_PICK_GET_RESULTS, &args![pick]).u32();
        let entry = e.call(RESULTS_GET, &args![results, index]).u32();
        index += 1;
        // The node of the hit object, or of its parent.
        let hit = e.call(RESULT_OBJECT, &args![entry]).u32();
        let mut node = if hit != 0 {
            let object = e.call(RESULT_OBJECT, &args![entry]).u32();
            e.vcall(object, 0xc, &args![]).u32()
        } else {
            0
        };
        if node == 0 {
            let object = e.call(RESULT_OBJECT, &args![entry]).u32();
            if e.call(NODE_PARENT, &args![object]).u32() != 0 {
                let object = e.call(RESULT_OBJECT, &args![entry]).u32();
                let parent = e.call(NODE_PARENT, &args![object]).u32();
                node = e.vcall(parent, 0xc, &args![]).u32();
            }
        }
        let mut tile = 0;
        if node != 0 {
            tile = e.call(TILE_FROM_NODE, &args![node]).u32();
        }
        while tile == 0 && node != 0 {
            node = e.call(NODE_PARENT, &args![node]).u32();
            tile = e.call(TILE_FROM_NODE, &args![node]).u32();
        }
        if tile == 0 {
            continue;
        }
        if !e.call(TILE_IS_ACCEPTING_EVENTS, &args![this, tile]).bool() {
            continue;
        }
        if e.call(TILE_IS_TRUE, &args![tile, 0xfaeu32]).bool()
            && e.call(TILE_IMAGE_NODE, &args![tile]).u32() != 0
        {
            // The tile's menu tile says whether the position is scaled.
            let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
            let menu_tile = e.call(GET_FIELD_AT_4, &args![menu]).u32();
            let value = tile_get_float(e, menu_tile, 0xfaa);
            let scaled = value == e.global::<f64>(PICK_SCALED_TILE_VALUE);
            let image = e.call(TILE_IMAGE_NODE, &args![tile]).u32();
            e.call(NODE_SIBLING_UPDATE, &args![image]);
            let image = e.call(TILE_IMAGE_NODE, &args![tile]).u32();
            if e.call(NODE_CHILD_POINTER, &args![image, 0u32]).u32() != 0 {
                let kind = e.call(PROPERTY_TYPE_THREE, &args![]).u32();
                let image = e.call(TILE_IMAGE_NODE, &args![tile]).u32();
                let child = e.call(NODE_CHILD_AT, &args![image, 0u32]).u32();
                let property = e.call(NODE_GET_PROPERTY, &args![child, kind]).u32();
                if property != 0 {
                    let rect = e.call(PROPERTY_RECT, &args![property]).u32();
                    // Left, top, right and bottom of the pick rectangle.
                    let mut bounds = [
                        e.mem.i32(rect),
                        e.mem.i32(rect + 4),
                        e.mem.i32(rect + 8),
                        e.mem.i32(rect + 0xc),
                    ];
                    let setting = e.call(SETTING_BYTE, &args![SETTING_HOLDER_011DB2CC]).u32();
                    if e.mem.u8(setting) != 0
                        && e.get(this, InterfaceManager::bMouseOverRenderedMenu) != 0
                    {
                        let width = e.call(REAL_SCREEN_WIDTH, &args![]).f64();
                        let reference = e.call(SCREEN_WIDTH_CONSTANT, &args![]).f64();
                        let scale = (width / reference) as f32;
                        for slot in [0usize, 2, 1, 3] {
                            bounds[slot] = e
                                .call(FTOL, &args![bounds[slot] as f64 * scale as f64])
                                .i32();
                        }
                    }
                    if bounds.iter().any(|bound| *bound != 0) {
                        let mut across = x;
                        if scaled {
                            let manager = e.call(GET_MANAGER, &args![]).u32();
                            let factor = e.mem.f32(manager + 0x4d0);
                            across = (across as f64 * factor as f64) as f32;
                        }
                        let (across, down) = (across as f64, y as f64);
                        if across < bounds[0] as f64
                            || across >= bounds[2] as f64
                            || down < bounds[1] as f64
                            || down >= bounds[3] as f64
                        {
                            continue;
                        }
                    }
                }
            }
        }
        // The tile's own hit test (virtual slot `0x14`) with the pointer.
        if !e.vcall(tile, 0x14, &args![x, y]).bool() {
            continue;
        }
        if !e.call(TILE_IS_TRUE, &args![tile, 0xfafu32]).bool() {
            continue;
        }
        let mut chosen = tile;
        if !keep_exact_tile && e.call(TILE_GET_VALUE_Q, &args![tile, 0xfaau32]).u32() == 0 {
            let mut ancestor = e.call(TILE_PARENT, &args![tile]).u32();
            while ancestor != 0 && e.call(TILE_GET_VALUE_Q, &args![ancestor, 0xfaau32]).u32() == 0 {
                ancestor = e.call(TILE_PARENT, &args![ancestor]).u32();
            }
            if ancestor != 0 {
                chosen = ancestor;
            }
        }
        // Virtual slot `0xc` is the tile type; `0x387` is the text tile.
        let kind = e.vcall(chosen, 0xc, &args![]).u32();
        if kind == 0x387 {
            let word = e.call(PICK_RESULT_HIT_WORD, &args![entry]).u32() as u16;
            e.set(this, InterfaceManager::iCharHit, word / 2);
        } else {
            e.set(this, InterfaceManager::iCharHit, 0xffff);
        }
        return chosen;
    }
}

// Translated from 00712e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the four words at `rect` (a `NiRect<float>`: `left right top
/// bottom`) to `+0x100` of the object (the camera's viewport).
pub fn fn_00712e60(e: &mut Engine, this: Ptr, rect: u32) {
    for word in 0..4 {
        let value = e.mem.u32(rect + 4 * word);
        e.mem.set_u32(this.addr() + 0x100 + 4 * word, value);
    }
}

// Translated from 00712e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::CreateSceneGraph` (Xbox PDB): wraps `existing` (or a
/// new `SceneGraph` built from `name`) for the menus. Orients and shapes
/// its camera: the rotation rows `(0 0 1) (1 0 0) (0 1 0)`, the aspect
/// ratio of the renderer (divided by 4/3, kept at `+0x4d0`), an orthographic
/// frustum `1280 * aspect` wide and `960` high, and a translation that
/// centres it. For the 2D graph (`is_3d` zero) it also creates the
/// `InterfaceManager: Main Root` and `Cursor Root` nodes (`+0x80`, `+0x84`),
/// attaches them and registers the graph with the `FaderManager`; for the
/// 3D graph it sets the camera's field of view. Returns the graph.
pub fn interface_manager_create_scene_graph(
    e: &mut Engine,
    this: Ptr<InterfaceManager>,
    existing: u32,
    name: u32,
    is_3d: u8,
) -> u32 {
    with_scope_guard(e, 0xe11, |e| {
        let mut graph = existing;
        if graph == 0 {
            graph = construct_new(e, NI_ALLOC, 0xc0, |e, block| {
                let name = e.call(PASS_THROUGH, &args![name, 0u32]).u32();
                e.call(SCENE_GRAPH_CONSTRUCT, &args![block, name, 0u32, 0u32])
                    .ptr()
            })
            .addr();
        }
        let renderer = e.call(RENDERER_GET, &args![]).u32();
        e.with_stack(0x24, |e, matrix| {
            e.call(MEMBER_CONSTRUCTOR_EMPTY, &args![matrix]);
            fn_007133b0(e, matrix, 0, 0.0, 0.0, 1.0);
            fn_007133b0(e, matrix, 1, 1.0, 0.0, 0.0);
            fn_007133b0(e, matrix, 2, 0.0, 1.0, 0.0);
            let camera = e.call(SCENE_GRAPH_GET_CAMERA, &args![graph]).u32();
            e.call(CAMERA_SET_ROTATION, &args![camera, matrix]);
        });
        // The aspect ratio of the renderer against 4:3.
        let width = e.call(RENDERER_WIDTH, &args![renderer]).u32();
        let height = e.call(RENDERER_HEIGHT, &args![renderer]).u32();
        let ratio = (width as f64 / height as f64) as f32;
        let aspect = (ratio as f64 / e.global::<f64>(ASPECT_DIVISOR)) as f32;
        e.set(this, InterfaceManager::field_4d0, aspect);
        let (right, top) = e.with_stack(0x1c, |e, frustum| {
            e.call(FRUSTUM_CONSTRUCT, &args![frustum, 0u32]);
            e.mem.set_u8(frustum.addr() + 0x18, 1);
            e.mem.set_f32(frustum.addr(), 0.0);
            let horizontal: f64 = e.global(CURSOR_HORIZONTAL_SCALE);
            let right = (aspect as f64 * horizontal) as f32;
            e.mem.set_f32(frustum.addr() + 4, right);
            let top: f32 = e.global(FRUSTUM_TOP);
            e.mem.set_f32(frustum.addr() + 8, top);
            e.mem.set_f32(frustum.addr() + 0xc, 0.0);
            e.mem.set_f32(frustum.addr() + 0x10, 0.0);
            let far: f32 = e.global(FRUSTUM_FAR);
            e.mem.set_f32(frustum.addr() + 0x14, far);
            let camera = e.call(SCENE_GRAPH_GET_CAMERA, &args![graph]).u32();
            e.call(CAMERA_SET_FRUSTUM, &args![camera, frustum]);
            (right, top)
        });
        // The camera is moved so that the frustum is centred.
        let half: f64 = e.global(HALF);
        let across = (-(right as f64) * half) as f32;
        let down = (-(top as f64) * half) as f32;
        e.with_stack(0xc, |e, position| {
            let position = e
                .call(NI_POINT3_CONSTRUCT, &args![position, across, 0.0f32, down])
                .u32();
            let camera = e.call(SCENE_GRAPH_GET_CAMERA, &args![graph]).u32();
            e.call(NODE_SET_TRANSLATE_VECTOR, &args![camera, position]);
        });
        if is_3d == 0 {
            let distance = e.get(this, InterfaceManager::fOneToOneDistance);
            let fader = e.global::<u32>(FADER_MANAGER);
            e.call(FADER_ADD_ROOT, &args![fader, graph, distance, 1u32]);
            let main_root = construct_new(e, NI_ALLOC, 0xac, |e, block| {
                e.call(NI_NODE_CONSTRUCT, &args![block, 0u32]).ptr()
            });
            e.set(this, InterfaceManager::pInterfaceRoot, main_root.addr());
            attach_root(
                e,
                this,
                graph,
                InterfaceManager::pInterfaceRoot,
                MAIN_ROOT_NAME,
            );
            let cursor_root = construct_new(e, NI_ALLOC, 0xac, |e, block| {
                e.call(NI_NODE_CONSTRUCT, &args![block, 0u32]).ptr()
            });
            e.set(this, InterfaceManager::pCursorRoot, cursor_root.addr());
            attach_root(
                e,
                this,
                graph,
                InterfaceManager::pCursorRoot,
                CURSOR_ROOT_NAME,
            );
            let distance = e.get(this, InterfaceManager::fOneToOneDistance);
            let fader = e.global::<u32>(FADER_MANAGER);
            e.call(FADER_ADD_ROOT, &args![fader, graph, distance, 0u32]);
        } else {
            e.call(
                SCENE_GRAPH_SET_CAMERA_FOV,
                &args![graph, e.global::<f32>(FOV_3D), 0u32, 0u32, 0u32],
            );
        }
        graph
    })
}

/// Names the node in `root_field`, attaches it to `graph` (virtual slot
/// `0xdc`, second argument 1) and moves it down by the one-to-one distance.
fn attach_root(
    e: &mut Engine,
    this: Ptr<InterfaceManager>,
    graph: u32,
    root_field: Field<InterfaceManager, u32>,
    name: u32,
) {
    let node = e.get(this, root_field);
    e.with_stack(4, |e, slot| {
        let handle = e.call(FIXED_STRING_CONSTRUCT, &args![slot, name]).u32();
        e.call(NODE_SET_NAME, &args![node, handle]);
        e.call(FIXED_STRING_DESTROY, &args![slot]);
    });
    e.vcall(graph, 0xdc, &args![node, 1u32]);
    // The node's translation is read (and not used) before it is set.
    e.call(NODE_TRANSLATION, &args![node]);
    let distance = e.get(this, InterfaceManager::fOneToOneDistance);
    e.with_stack(0xc, |e, position| {
        let position = e
            .call(
                NI_POINT3_CONSTRUCT,
                &args![position, 0.0f32, distance, 0.0f32],
            )
            .u32();
        e.call(NODE_SET_TRANSLATE_VECTOR, &args![node, position]);
    });
}

// Translated from 007133b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the three floats as row `row` (0xc bytes each) of the 3x3 matrix
/// at `this`.
pub fn fn_007133b0(e: &mut Engine, this: Ptr, row: u32, x: f32, y: f32, z: f32) {
    let at = this.addr().wrapping_add(row.wrapping_mul(0xc));
    e.mem.set_f32(at, x);
    e.mem.set_f32(at + 4, y);
    e.mem.set_f32(at + 8, z);
}

// Translated from 007133f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the pipboy manager (`+0x174`): deletes the old one, builds a new
/// `FOPipboyManager` (0x170 bytes) and calls its virtual slot `0x24`.
pub fn fn_007133f0(e: &mut Engine, this: Ptr<InterfaceManager>) {
    let old = e.get(this, InterfaceManager::pPipboy);
    if old != 0 {
        delete_virtual(e, old);
    }
    let pipboy = construct_new(e, OPERATOR_NEW, 0x170, |e, block| {
        e.call(PIPBOY_MANAGER_CONSTRUCT, &args![block]).ptr()
    });
    e.set(this, InterfaceManager::pPipboy, pipboy.addr());
    let pipboy = e.get(this, InterfaceManager::pPipboy);
    e.vcall(pipboy, 0x24, &args![]);
}

/// The byte a setting holder keeps (`00408d60` returns its address).
fn setting_flag(e: &mut Engine, holder: u32) -> bool {
    let at = e.call(SETTING_BYTE, &args![holder]).u32();
    e.mem.u8(at) != 0
}

/// Waits for the rendering system's stage 1 (`(1, 0x17)`).
fn wait_rendering_stage(e: &mut Engine) {
    let system = e.call(RENDERING_SYSTEM, &args![]).u32();
    e.call(RENDERING_WAIT_STAGE, &args![system, 1u32, 0x17u32]);
}

/// Accumulates the menu scene `scene` into the culling process, with the
/// manager's accumulator (`+0x8c`).
fn accumulate_menus(e: &mut Engine, this: Ptr<InterfaceManager>, scene: u32, culling: u32) {
    let accumulator = ni_pointer_get(e, this.addr() + 0x8c);
    e.call(CULLING_SET_ACCUMULATOR, &args![culling, accumulator]);
    let camera = e.call(SCENE_GRAPH_GET_CAMERA, &args![scene]).u32();
    e.call(ACCUMULATE_SCENE, &args![camera, scene, culling]);
}

/// Finishes the accumulation and detaches the accumulator.
fn finish_menus(e: &mut Engine, this: Ptr<InterfaceManager>, scene: u32, culling: u32) {
    let accumulator = ni_pointer_get(e, this.addr() + 0x8c);
    let camera = e.call(SCENE_GRAPH_GET_CAMERA, &args![scene]).u32();
    e.call(ACCUMULATE_FINISH, &args![camera, accumulator, 0u32]);
    e.call(CULLING_SET_ACCUMULATOR, &args![culling, 0u32]);
}

/// Shows a menu's image node when it is hidden; returns the menu when it
/// did (so it can be hidden again), else null.
fn show_hidden_menu(e: &mut Engine, menu: u32) -> u32 {
    if menu != 0 && e.call(TILE_IMAGE_NODE, &args![menu]).u32() != 0 {
        let image = e.call(TILE_IMAGE_NODE, &args![menu]).u32();
        if !e.call(NODE_FLAG_TEST_00456610, &args![image]).bool() {
            let image = e.call(TILE_IMAGE_NODE, &args![menu]).u32();
            e.call(NODE_SET_FLAG, &args![image, 1u32]);
            return menu;
        }
    }
    0
}

/// Hides the menu that [`show_hidden_menu`] showed, if its node is still
/// flagged.
fn hide_shown_menu(e: &mut Engine, menu: u32) {
    if menu != 0 && e.call(TILE_IMAGE_NODE, &args![menu]).u32() != 0 {
        let image = e.call(TILE_IMAGE_NODE, &args![menu]).u32();
        if e.call(NODE_FLAG_TEST_00456610, &args![image]).bool() {
            let image = e.call(TILE_IMAGE_NODE, &args![menu]).u32();
            e.call(NODE_SET_FLAG, &args![image, 0u32]);
        }
    }
}

// Translated from 007134d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Accumulates the menu scene graph into the culling process `culling`
/// (pushed on the process's stack for the duration) according to the three
/// settings at `011d8ba0` (A), `011db2cc` (B) and `011d8ab0`. A and B both
/// set: the scene graph is prepared (`00713c70` and the menu elements
/// isolated, unless the click runs multithreaded, which waits for the
/// rendering stage instead), the menus `0x421`, `0x40c` and `0x41f` are
/// shown while it accumulates, and the renderer's viewport is reset when it
/// is its own. Only one of them: the elements are isolated and accumulated
/// in the one or two passes that setting needs. Neither: a plain pass.
/// `_unused_1` is the second word the callers push.
pub fn fn_007134d0(e: &mut Engine, this: Ptr<InterfaceManager>, culling: u32, _unused_1: u32) {
    e.call(CULLING_PUSH, &args![culling, 1u32]);
    let renderer = e.call(RENDERER_GET, &args![]).u32();
    let multithreaded = |e: &mut Engine| e.get(this, InterfaceManager::bClickMultithreaded) != 0;
    if setting_flag(e, SETTING_HOLDER_011D8BA0) && setting_flag(e, SETTING_HOLDER_011DB2CC) {
        let scene = e.call(GET_FRAME_SCENE_NODE, &args![this]).u32();
        if multithreaded(e) {
            wait_rendering_stage(e);
        } else {
            if !e.call(MENU_MODE_IS_NOT_ONE, &args![this]).bool()
                && !e.call(IS_IN_GAME_LOADING_MENU_OPEN, &args![]).bool()
            {
                fn_00713c70(e, this);
            }
            if e.call(MENU_PREDICATE_007079F0, &args![this]).bool()
                && !e.call(IS_IN_GAME_LOADING_MENU_OPEN, &args![]).bool()
            {
                e.call(ISOLATE_MENU_ELEMENTS, &args![0u32, 0u32]);
            }
        }
        let first = e.call(TILE_GET_MENU_BY_CLASS, &args![0x421u32]).u32();
        let second = e.call(TILE_GET_MENU_BY_CLASS, &args![0x40cu32]).u32();
        let third = e.call(TILE_GET_MENU_BY_CLASS, &args![0x41fu32]).u32();
        let first = show_hidden_menu(e, first);
        let second = show_hidden_menu(e, second);
        let third = show_hidden_menu(e, third);
        if !multithreaded(e) {
            accumulate_menus(e, this, scene, culling);
            let system = e.call(RENDERING_SYSTEM, &args![]).u32();
            e.call(RENDERING_SET_STAGE, &args![system, 0u32, 0x17u32]);
            wait_rendering_stage(e);
        }
        let renderer_a = e.call(RENDERER_GET, &args![]).u32();
        let renderer_b = e.call(RENDERER_GET, &args![]).u32();
        let slot_c8 = e.vcall(renderer_b, 0xc8, &args![]).u32();
        let slot_cc = e.vcall(renderer_a, 0xcc, &args![]).u32();
        if slot_cc == slot_c8 {
            let camera = e.call(SCENE_GRAPH_GET_CAMERA, &args![scene]).u32();
            fn_00712e60(e, Ptr::new(camera), VIEWPORT_RECT);
        }
        finish_menus(e, this, scene, culling);
        hide_shown_menu(e, first);
        hide_shown_menu(e, second);
        hide_shown_menu(e, third);
        if e.call(MENU_PREDICATE_007079F0, &args![this]).bool()
            && !e.call(IS_IN_GAME_LOADING_MENU_OPEN, &args![]).bool()
        {
            e.call(RESTORE_MENU_ELEMENTS, &args![]);
        }
        if setting_flag(e, SETTING_HOLDER_011D8AB0) {
            e.call(ISOLATE_MENU_ELEMENTS, &args![1u32, 0u32]);
            let scene = e.call(GET_FRAME_SCENE_NODE, &args![this]).u32();
            accumulate_menus(e, this, scene, culling);
            finish_menus(e, this, scene, culling);
            e.call(RESTORE_MENU_ELEMENTS, &args![]);
        }
    } else if !setting_flag(e, SETTING_HOLDER_011D8BA0) && !setting_flag(e, SETTING_HOLDER_011DB2CC)
    {
        // Neither setting: one plain pass.
        if multithreaded(e) {
            wait_rendering_stage(e);
        }
        e.call(RESTORE_MENU_ELEMENTS, &args![]);
        let scene = e.call(GET_FRAME_SCENE_NODE, &args![this]).u32();
        if !multithreaded(e) {
            accumulate_menus(e, this, scene, culling);
        }
        finish_menus(e, this, scene, culling);
    } else if !setting_flag(e, SETTING_HOLDER_011D8BA0) {
        // Only B: the isolated pass, then the rest.
        if multithreaded(e) {
            wait_rendering_stage(e);
        }
        e.call(ISOLATE_MENU_ELEMENTS, &args![1u32, 0u32]);
        let scene = e.call(GET_FRAME_SCENE_NODE, &args![this]).u32();
        let camera = e.call(SCENE_GRAPH_GET_CAMERA, &args![scene]).u32();
        e.call(RENDERER_SET_CAMERA, &args![renderer, camera]);
        if !multithreaded(e) {
            accumulate_menus(e, this, scene, culling);
        }
        finish_menus(e, this, scene, culling);
        e.call(RESTORE_MENU_ELEMENTS, &args![]);
        e.call(ISOLATE_MENU_ELEMENTS, &args![0u32, 0u32]);
        let scene = e.call(GET_FRAME_SCENE_NODE, &args![this]).u32();
        let camera = e.call(SCENE_GRAPH_GET_CAMERA, &args![scene]).u32();
        e.call(RENDERER_SET_CAMERA, &args![renderer, camera]);
        accumulate_menus(e, this, scene, culling);
        finish_menus(e, this, scene, culling);
        e.call(RESTORE_MENU_ELEMENTS, &args![]);
    } else if !setting_flag(e, SETTING_HOLDER_011DB2CC) {
        // Only A: elements isolated with both flags; the viewport is
        // reset when the renderer has no target of its own.
        if multithreaded(e) {
            wait_rendering_stage(e);
        }
        e.call(ISOLATE_MENU_ELEMENTS, &args![1u32, 1u32]);
        let scene = e.call(GET_FRAME_SCENE_NODE, &args![this]).u32();
        let camera = e.call(SCENE_GRAPH_GET_CAMERA, &args![scene]).u32();
        e.call(RENDERER_SET_CAMERA, &args![renderer, camera]);
        if !multithreaded(e) {
            accumulate_menus(e, this, scene, culling);
        }
        let current = e.call(RENDERER_GET, &args![]).u32();
        if e.vcall(current, 0xcc, &args![]).u32() == 0 {
            let camera = e.call(SCENE_GRAPH_GET_CAMERA, &args![scene]).u32();
            fn_00712e60(e, Ptr::new(camera), VIEWPORT_RECT);
        }
        finish_menus(e, this, scene, culling);
        e.call(RESTORE_MENU_ELEMENTS, &args![]);
    } else if multithreaded(e) {
        // Both settings set cannot reach here (the first branch took it).
        wait_rendering_stage(e);
    }
    e.call(CULLING_POP, &args![culling]);
}

// Translated from 00713c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the tile lock (`011d8c18`), then deletes every object queued in
/// the deferred queue (`011d8b2c`; the virtual destructor with flag 1) and
/// releases the lock.
pub fn fn_00713c00(e: &mut Engine) {
    e.call(LOCK_ENTER, &args![TILE_LOCK, 0u32]);
    while !e.call(COLLECTION_IS_EMPTY, &args![DEFERRED_QUEUE]).bool() {
        let object = e.call(DEFERRED_QUEUE_POP, &args![DEFERRED_QUEUE]).u32();
        if object != 0 {
            delete_virtual(e, object);
        }
    }
    e.call(LOCK_LEAVE, &args![TILE_LOCK]);
}

// Translated from 00713c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates the tiles: when the setting at `011c3ea4` is above 1 and the byte
/// at `011dfa19` is set, waits on the object at `011dfa50` with `(0, 1)`;
/// flushes the deferred deletions ([`fn_00713c00`]), runs `00a04510`, clears
/// `+0xdc` and updates either the loading menu's children (the in-game
/// loading menu is open and the XUI is not up) or every tile, then the fade
/// controls. Unless the update array at `011d8b44` is empty (its count word
/// at `+8` is zero) it takes the update lock, resets the flags of the tiles
/// in it ([`fn_00713e20`]) and releases the lock. `011d8908` is set for the
/// duration.
pub fn fn_00713c70(e: &mut Engine, this: Ptr<InterfaceManager>) {
    let setting = e
        .call(GET_SETTING_VALUE, &args![SETTING_HOLDER_011C3EA4])
        .u32();
    if e.mem.i32(setting) > 1 {
        fn_00713d80(e);
        if fn_00713d90(e) != 0 {
            let pool = fn_00713d80(e);
            e.call(SEMAPHORE_POOL_WAIT, &args![pool, 0u32, 1u32]);
        }
    }
    fn_00713c00(e);
    e.set_global(TILES_UPDATING, 1u8);
    e.call(GET_FRAME_SCENE_NODE, &args![this]);
    e.call(TILE_UPDATE_PREPARE, &args![]);
    e.mem.set_u8(this.addr() + 0xdc, 0);
    if e.call(IS_IN_GAME_LOADING_MENU_OPEN, &args![]).bool() && !e.call(XUI_IS_UP, &args![]).bool()
    {
        let menu = e.call(TILE_GET_MENU_BY_CLASS, &args![0x3efu32]).u32();
        e.call(TILE_UPDATE_CHILDREN, &args![menu, 0u32]);
    } else {
        e.call(TILE_UPDATE_ALL, &args![0u32]);
    }
    e.call(TILE_UPDATE_FADE_CONTROLS, &args![]);
    fn_00713d60(e);
    if !e.call(COLLECTION_IS_EMPTY, &args![UPDATE_ARRAY]).bool() {
        let lock = fn_00713d70(e);
        e.call(LOCK_ENTER, &args![lock, 0u32]);
        fn_00713e20(e, this);
        let lock = fn_00713d70(e);
        e.call(LOCK_LEAVE, &args![lock]);
    }
    e.set_global(TILES_UPDATING, 0u8);
}

// Translated from 00713d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the word at `011f32d4`.
pub fn fn_00713d60(e: &mut Engine) {
    e.set_global(TILE_UPDATE_DEPTH, 0u32);
}

// Translated from 00713d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the address of the update lock object (`011f3330`).
pub fn fn_00713d70(_e: &mut Engine) -> u32 {
    UPDATE_LOCK
}

// Translated from 00713d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the address of the semaphore pool object (`011dfa50`).
pub fn fn_00713d80(_e: &mut Engine) -> u32 {
    SEMAPHORE_POOL
}

// Translated from 00713d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte at `011dfa19` (whether the semaphore pool is in use).
pub fn fn_00713d90(e: &mut Engine) -> u8 {
    e.global(SEMAPHORE_POOL_IN_USE)
}

// Translated from 00713da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::AddTileToUpdateList` (Xbox PDB): appends `tile` to the
/// update array at `011d8b44` unless it is null or already in it.
pub fn interface_manager_add_tile_to_update_list(
    e: &mut Engine,
    _this: Ptr<InterfaceManager>,
    tile: u32,
) {
    if tile != 0 {
        e.with_stack(4, |e, item| {
            e.mem.set_u32(item.addr(), tile);
            if !e.call(ARRAY_FIND, &args![UPDATE_ARRAY, item]).bool() {
                e.call(ARRAY_ADD, &args![UPDATE_ARRAY, item]);
            }
        });
    }
}

// Translated from 00713de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `tile` from the update array at `011d8b44` when it is in it.
pub fn fn_00713de0(e: &mut Engine, _this: Ptr<InterfaceManager>, tile: u32) {
    e.with_stack(4, |e, item| {
        e.mem.set_u32(item.addr(), tile);
        let index = e
            .call(
                ARRAY_FIND_INDEX,
                &args![UPDATE_ARRAY, item, 0u32, ARRAY_COMPARE],
            )
            .i32();
        if index != -1 {
            e.call(ARRAY_REMOVE_AT, &args![UPDATE_ARRAY, index, 1u32]);
        }
    });
}

// Translated from 00713e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// For every tile of the update array: updates its image node (a zeroed
/// update data) and clears the tile's byte at `+0x34`; then, when the
/// current thread is the owner thread, clears the array
/// (`Clear(1)`).
pub fn fn_00713e20(e: &mut Engine, _this: Ptr<InterfaceManager>) {
    let mut index = 0u32;
    while index < e.call(WORD_AT_8, &args![UPDATE_ARRAY]).u32() {
        let slot = e.call(ARRAY_ELEMENT, &args![UPDATE_ARRAY, index]).u32();
        let tile = e.mem.u32(slot);
        let node = if tile != 0 {
            e.call(TILE_IMAGE_NODE, &args![tile]).u32()
        } else {
            0
        };
        if node != 0 {
            e.with_stack(12, |e, update_data| {
                e.call(
                    NI_UPDATE_DATA_CONSTRUCT,
                    &args![update_data, 0.0f32, 0u32, 0u32],
                );
                e.call(NODE_UPDATE, &args![node, update_data]);
            });
        }
        fn_00713ee0(e, Ptr::new(tile));
        index += 1;
    }
    let current = e.call(CURRENT_THREAD_ID, &args![]).u32();
    let owner = e.global::<u32>(OWNER_OBJECT);
    if current == e.call(OWNER_THREAD_ID, &args![owner]).u32() {
        e.call(ARRAY_CLEAR, &args![UPDATE_ARRAY, 1u32]);
    }
}

// Translated from 00713ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the byte at `+0x34` of the object (a tile's update flag).
pub fn fn_00713ee0(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 0x34, 0);
}

// Translated from 00713f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks the click as multithreaded (`+0x148`), updates the tiles unless
/// the manager's menu-mode test or the in-game loading menu says
/// otherwise ([`fn_00713c70`]), isolates the menu elements when
/// `007079f0` holds (and the loading menu is not open), and queues the
/// interface scene graph's accumulation as a task of the rendering system
/// (`AddAccumTask`), then sets the thread stage.
pub fn fn_00713f00(e: &mut Engine, this: Ptr<InterfaceManager>) {
    e.set(this, InterfaceManager::bClickMultithreaded, 1);
    let scene = e.call(GET_FRAME_SCENE_NODE, &args![this]).u32();
    if !e.call(MENU_MODE_IS_NOT_ONE, &args![this]).bool()
        && !e.call(IS_IN_GAME_LOADING_MENU_OPEN, &args![]).bool()
    {
        fn_00713c70(e, this);
    }
    if e.call(MENU_PREDICATE_007079F0, &args![this]).bool()
        && !e.call(IS_IN_GAME_LOADING_MENU_OPEN, &args![]).bool()
    {
        e.call(ISOLATE_MENU_ELEMENTS, &args![0u32, 0u32]);
    }
    let accumulator = ni_pointer_get(e, this.addr() + 0x8c);
    let camera = e.call(SCENE_GRAPH_GET_CAMERA, &args![scene]).u32();
    let system = e.call(RENDERING_SYSTEM, &args![]).u32();
    e.call(
        RENDERING_ADD_ACCUM_TASK,
        &args![
            system,
            camera,
            0u32,
            scene,
            0u32,
            0u32,
            accumulator,
            1u32,
            0x17u32,
            0u32
        ],
    );
    let system = e.call(RENDERING_SYSTEM, &args![]).u32();
    e.call(RENDERING_SET_STAGE, &args![system, 0u32, 0x17u32]);
}

/// The renderer's device (`004dc020`, a COM object: its methods take the
/// device again as their first argument word).
fn render_device(e: &mut Engine) -> u32 {
    let renderer = e.call(RENDERER_GET, &args![]).u32();
    e.call(DEVICE_GET, &args![renderer]).u32()
}

/// Sets the device viewport from the rectangle at [`VIEWPORT_RECT`], scaled
/// to the size of the renderer's target (slot `0xd4` of the renderer):
/// `x = width * left`, `width * (right - left)`, `y = height * bottom`,
/// `height * (top - bottom)`, depth `0` to `1`.
fn set_viewport_from_rect(e: &mut Engine) {
    let renderer = e.call(RENDERER_GET, &args![]).u32();
    let target = e.vcall(renderer, 0xd4, &args![]).u32();
    let width = e.call(WORD_AT_8, &args![target]).u32();
    let renderer = e.call(RENDERER_GET, &args![]).u32();
    let target = e.vcall(renderer, 0xd4, &args![]).u32();
    let height = e.call(WORD_AT_C, &args![target]).u32();
    let rect: Vec<f64> = (0..4)
        .map(|i| e.global::<f32>(VIEWPORT_RECT + 4 * i) as f64)
        .collect();
    let truncate = |value: f64| value.trunc() as i64 as i32;
    let x = truncate(width as f64 * rect[0]);
    let across = truncate((rect[1] - rect[0]) * width as f64);
    let y = truncate(height as f64 * rect[3]);
    let down = truncate((rect[2] - rect[3]) * height as f64);
    e.with_stack(0x18, |e, viewport| {
        let at = viewport.addr();
        e.mem.set_i32(at, x);
        e.mem.set_i32(at + 4, y);
        e.mem.set_i32(at + 8, across);
        e.mem.set_i32(at + 0xc, down);
        e.mem.set_f32(at + 0x10, 0.0);
        e.mem.set_f32(at + 0x14, 1.0);
        let device = render_device(e);
        e.vcall(device, 0xbc, &args![viewport]);
    });
}

/// Clears the two rectangles the render pass starts with: the renderer's
/// current clear rectangle is saved (slot `0xb4`), its colour set (slot
/// `0xac`), two strips are filled through [`fn_007148c0`] and the saved
/// rectangle is restored.
fn clear_menu_strips(e: &mut Engine) {
    e.with_stack(0x10, |e, saved| {
        e.with_stack(0x10, |e, strip| {
            e.call(
                RECT_CONSTRUCT,
                &args![saved, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
            );
            let renderer = e.call(RENDERER_GET, &args![]).u32();
            e.vcall(renderer, 0xb4, &args![saved]);
            let renderer = e.call(RENDERER_GET, &args![]).u32();
            e.vcall(renderer, 0xac, &args![CLEAR_COLOR]);
            e.call(
                RECT_CONSTRUCT,
                &args![strip, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
            );
            let top: f32 = e.global(CLEAR_RECT_ONE_TOP);
            for (i, v) in [0.0f32, top, 1.0, 0.0].iter().enumerate() {
                e.mem.set_f32(strip.addr() + 4 * i as u32, *v);
            }
            let renderer = e.call(RENDERER_GET, &args![]).u32();
            fn_007148c0(e, Ptr::new(renderer), strip.addr(), 1);
            let left: f32 = e.global(CLEAR_RECT_TWO_LEFT);
            for (i, v) in [left, 1.0f32, 1.0, 0.0].iter().enumerate() {
                e.mem.set_f32(strip.addr() + 4 * i as u32, *v);
            }
            let renderer = e.call(RENDERER_GET, &args![]).u32();
            fn_007148c0(e, Ptr::new(renderer), strip.addr(), 1);
            let renderer = e.call(RENDERER_GET, &args![]).u32();
            e.vcall(renderer, 0xac, &args![saved]);
        });
    });
}

/// Resets the device after the menus rendered: viewport to the render
/// target's size (the target is the renderer's slot `0xcc` or `0xc8`), the
/// render states and the sampler / texture bindings, then the 18
/// render-state counters ([`fn_00714a40`] ... `00714c40`) and the renderer
/// state object's saved values (slots `0xb0`/`0xb8`, `0x90`/`0x98`,
/// `0x80`/`0x88`).
fn reset_render_pass(e: &mut Engine) {
    let renderer = e.call(RENDERER_GET, &args![]).u32();
    let mut target = e.vcall(renderer, 0xcc, &args![]).u32();
    if target == 0 {
        let renderer = e.call(RENDERER_GET, &args![]).u32();
        target = e.vcall(renderer, 0xc8, &args![]).u32();
    }
    let width = e.vcall(target, 0x8c, &args![0u32]).u32();
    let height = e.vcall(target, 0x90, &args![0u32]).u32();
    e.with_stack(0x1c, |e, region| {
        let at = region.addr();
        e.mem.set_u32(at, 0);
        e.mem.set_u32(at + 4, 0);
        e.mem.set_u32(at + 8, width);
        e.mem.set_u32(at + 0xc, height);
        e.mem.set_f32(at + 0x10, 0.0);
        e.mem.set_f32(at + 0x14, 1.0);
        let device = render_device(e);
        e.vcall(device, 0xbc, &args![region]);
        e.with_stack(4, |e, saved| {
            e.mem.set_u32(saved.addr(), 0);
            let device = render_device(e);
            e.vcall(device, 0x160, &args![saved]);
            let device = render_device(e);
            e.vcall(device, 0xe4, &args![0x1bu32, 0u32]);
            let renderer = e.call(RENDERER_GET, &args![]).u32();
            let state = e.call(RENDERER_STATE_OBJECT, &args![renderer]).u32();
            e.vcall(state, 0x8c, &args![0u32, 0u32]);
            let renderer = e.call(RENDERER_GET, &args![]).u32();
            let state = e.call(RENDERER_STATE_OBJECT, &args![renderer]).u32();
            e.vcall(state, 0x7c, &args![0u32, 0u32]);
            let device = render_device(e);
            e.vcall(device, 0x1ac, &args![0u32]);
            let device = render_device(e);
            e.vcall(device, 0x170, &args![0u32]);
            let device = render_device(e);
            let value = e.mem.u32(saved.addr());
            e.vcall(device, 0x15c, &args![value]);
        });
    });
    fn_00714a40(e, 0);
    fn_00714a60(e, 0);
    fn_00714a80(e, 0);
    fn_00714aa0(e, 0);
    fn_00714ac0(e, 0);
    fn_00714ae0(e, 0);
    fn_00714b00(e, 0);
    e.call(RENDER_STATE_004ECED0, &args![0u32]);
    e.call(RENDER_STATE_004ECB40, &args![0u32]);
    fn_00714b20(e, 0);
    fn_00714b50(e, 0);
    fn_00714b80(e, 0);
    fn_00714bb0(e, 0);
    fn_00714bd0(e, 0);
    e.call(RENDER_STATE_004EB510, &args![0u32]);
    fn_00714bf0(e, 0);
    fn_00714c20(e, 0);
    e.call(RENDER_STATE_00714C40, &args![0u32]);
    e.call(RENDER_PASS_RESET, &args![]);
    let renderer = e.call(RENDERER_GET, &args![]).u32();
    let state = e.call(RENDERER_STATE_OBJECT, &args![renderer]).u32();
    for (get, set) in [(0xb0u32, 0xb8u32), (0x90, 0x98), (0x80, 0x88)] {
        let value = e.vcall(state, get, &args![]).u32();
        e.vcall(state, set, &args![value]);
    }
}

// Translated from 00713fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Renders the menus. When the render pass is active it sets the device
/// viewport from the custom rectangle (if enabled), the pass mode `(7, 0)`,
/// clears the menu strips (if the renderer's clear is enabled) and gives the
/// scene graph's camera the custom viewport rectangle. Then, guarded and
/// only when the interface scene graph exists, it builds a culling process
/// from the scene graph, flushes the worker object when its byte at `+0x1b0`
/// is set ([`fn_00714a00`]) and waits on it ([`fn_00714960`]), accumulates
/// the menus ([`fn_007134d0`]; the flag computed for its second word, which
/// it does not read, is the setting at `011db2cc` together with the mode
/// word `2` or a pipboy / rendered menu that is topmost) and, when the pass
/// was active, resets the device ([`reset_render_pass`]). Finally it
/// releases the worker object's semaphore ([`fn_00714900`]). `_unused_1`
/// and `_unused_2` are the two words the callers push.
pub fn fn_00713fb0(e: &mut Engine, this: Ptr<InterfaceManager>, _unused_1: u32, _unused_2: u32) {
    let renderer = e.call(RENDERER_GET, &args![]).u32();
    let mut pass_active = false;
    if e.call(RENDER_PASS_ACTIVE, &args![]).bool() {
        if e.global::<u8>(VIEWPORT_RECT_ENABLED) != 0 {
            set_viewport_from_rect(e);
        }
        e.call(RENDER_PASS_SET_MODE, &args![7u32, 0u32]);
        pass_active = true;
        if e.call(RENDER_CLEAR_ENABLED, &args![]).bool() {
            clear_menu_strips(e);
        }
        if e.global::<u8>(VIEWPORT_RECT_ENABLED) != 0 {
            let scene = e.call(GET_FRAME_SCENE_NODE, &args![this]).u32();
            let camera = e.call(SCENE_GRAPH_GET_CAMERA, &args![scene]).u32();
            fn_00712e60(e, Ptr::new(camera), VIEWPORT_RECT);
        }
    }
    with_scope_guard(e, 0x1069, |e| {
        let scene = e.call(GET_FRAME_SCENE_NODE, &args![this]).u32();
        if scene == 0 {
            return;
        }
        let source = e.call(CULLING_SOURCE_GET, &args![scene]).u32();
        let word = e.call(WORD_AT_8, &args![source]).u32();
        e.with_stack(0xc8, |e, culling| {
            e.call(CULLING_PROCESS_CONSTRUCT, &args![culling, word]);
            e.with_stack(4, |e, holder| {
                let node = e.call(GET_SECOND_SCENE_NODE, &args![renderer]).u32();
                e.call(NI_POINTER_CONSTRUCT, &args![holder, node]);
                if e.call(WORKER_GET, &args![]).u32() != 0 {
                    if fn_00714a00(e) {
                        e.call(WORKER_FLUSH, &args![]);
                    }
                    fn_00714960(e, 0xffff_ffff);
                }
                let topmost = setting_flag(e, SETTING_HOLDER_011DB2CC)
                    && (e.get(this, InterfaceManager::field_4bc) == 2
                        || ((e.call(IS_IN_PIPBOY_MENU, &args![]).bool()
                            || e.get(this, InterfaceManager::bIsInRenderedMenu) != 0)
                            && e.call(IS_CURRENT_RENDERED_MENU_TOPMOST, &args![]).bool()));
                fn_007134d0(e, this, culling.addr(), topmost as u32);
                if pass_active {
                    reset_render_pass(e);
                }
                if e.call(WORKER_GET, &args![]).u32() != 0 {
                    fn_00714900(e);
                }
                e.call(NI_POINTER_DESTROY, &args![holder]);
            });
            e.call(CULLING_PROCESS_DESTRUCT, &args![culling]);
        });
    });
}

// Translated from 007148c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the object answers `004a0e10(string 0106f2e0, 1)`, calls its
/// virtual slot `0x188` with the two words.
pub fn fn_007148c0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    if e.call(OBJECT_IS_KIND, &args![this, KIND_NAME, 1u32]).bool() {
        e.vcall(this.addr(), 0x188, &args![first, second]);
    }
}

// Translated from 00714900 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the worker object (`011f5b04`) exists, releases the semaphore of its
/// wrapper at `+0x190` ([`fn_00714930`]) and returns the wrapper's first
/// word; otherwise 0.
pub fn fn_00714900(e: &mut Engine) -> u32 {
    let manager = e.global::<u32>(WORKER_OBJECT);
    if manager == 0 {
        return 0;
    }
    fn_00714930(e, Ptr::new(manager + 0x190))
}

// Translated from 00714930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `0040b460(this)` and releases the semaphore handle at `+8` once
/// (`ReleaseSemaphore(handle, 1, 0)`); returns the first word of the
/// object.
pub fn fn_00714930(e: &mut Engine, this: Ptr) -> u32 {
    e.call(BEFORE_RELEASE, &args![this]);
    let handle = e.mem.u32(this.addr() + 8);
    e.call(RELEASE_SEMAPHORE, &args![handle, 1u32, 0u32]);
    e.mem.u32(this.addr())
}

// Translated from 00714960 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the worker object exists, waits on its wrapper with `timeout`
/// ([`fn_007149b0`]); if the wait timed out (it returned 1) it sets the
/// byte at `011f5b08` ([`fn_007149f0`]) and waits again without a timeout
/// (`-1`). Always returns 0.
pub fn fn_00714960(e: &mut Engine, timeout: u32) -> u32 {
    let manager = e.global::<u32>(WORKER_OBJECT);
    if manager != 0 {
        let wrapper = Ptr::new(manager + 0x190);
        if fn_007149b0(e, wrapper, timeout) == 1 {
            fn_007149f0(e);
            fn_007149b0(e, wrapper, 0xffff_ffff);
        }
    }
    0
}

// Translated from 007149b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Waits on the handle at `+8` (`WaitForSingleObject(handle, timeout)`):
/// `0x102` (timeout) gives 1; anything else gives the result of
/// `004019a0(this)`.
pub fn fn_007149b0(e: &mut Engine, this: Ptr, timeout: u32) -> u32 {
    let handle = e.mem.u32(this.addr() + 8);
    let status = e
        .call(WAIT_FOR_SINGLE_OBJECT, &args![handle, timeout])
        .u32();
    if status == 0x102 {
        1
    } else {
        e.call(WAIT_RESULT_HANDLER, &args![this]).u32()
    }
}

// Translated from 007149f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `011f5b08`.
pub fn fn_007149f0(e: &mut Engine) {
    e.set_global(WORKER_WAS_BLOCKED, 1u8);
}

// Translated from 00714a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the worker object (`011f5b04`) exists and its byte at `+0x1b0` is
/// set.
pub fn fn_00714a00(e: &mut Engine) -> bool {
    let manager = e.global::<u32>(WORKER_OBJECT);
    manager != 0 && e.mem.u8(manager + 0x1b0) != 0
}

// The render-state counters: each subtracts `count` from its counter
// global and runs one render-state call with fixed arguments.

// Translated from 00714a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Subtracts `count` from the counter at `011ff9d8`, then calls
/// `00b97de0(1, 0)`.
pub fn fn_00714a40(e: &mut Engine, count: u32) {
    render_state_counter(e, RENDER_COUNTER_011FF9D8, count);
    e.call(RENDER_STATE_00B97DE0, &args![1u32, 0u32]);
}

// Translated from 00714a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Subtracts `count` from the counter at `011ff9dc`, then calls
/// `BSRenderState::SetZWriteEnable` (Xbox PDB, `00b97e30`) with `(1, 0)`.
pub fn fn_00714a60(e: &mut Engine, count: u32) {
    render_state_counter(e, RENDER_COUNTER_011FF9DC, count);
    e.call(RENDER_STATE_00B97E30, &args![1u32, 0u32]);
}

// Translated from 00714a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Subtracts `count` from the counter at `011ff9e0`, then calls
/// `00b97e80(3, 0)`.
pub fn fn_00714a80(e: &mut Engine, count: u32) {
    render_state_counter(e, RENDER_COUNTER_011FF9E0, count);
    e.call(RENDER_STATE_00B97E80, &args![3u32, 0u32]);
}

// Translated from 00714aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Subtracts `count` from the counter at `011ff9e8`, then calls
/// `00b97ed0(0, 0)`.
pub fn fn_00714aa0(e: &mut Engine, count: u32) {
    render_state_counter(e, RENDER_COUNTER_011FF9E8, count);
    e.call(RENDER_STATE_00B97ED0, &args![0u32, 0u32]);
}

// Translated from 00714ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Subtracts `count` from the counter at `011ff9ec`, then calls
/// `00b97f20(0, 0, 0)`.
pub fn fn_00714ac0(e: &mut Engine, count: u32) {
    render_state_counter(e, RENDER_COUNTER_011FF9EC, count);
    e.call(RENDER_STATE_00B97F20, &args![0u32, 0u32, 0u32]);
}

// Translated from 00714ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Subtracts `count` from the counter at `011ff9f0`, then calls
/// `00b97fa0(0, 0)`.
pub fn fn_00714ae0(e: &mut Engine, count: u32) {
    render_state_counter(e, RENDER_COUNTER_011FF9F0, count);
    e.call(RENDER_STATE_00B97FA0, &args![0u32, 0u32]);
}

// Translated from 00714b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Subtracts `count` from the counter at `011ff9f4`, then calls
/// `00b97ff0(0, 1, 0)`.
pub fn fn_00714b00(e: &mut Engine, count: u32) {
    render_state_counter(e, RENDER_COUNTER_011FF9F4, count);
    e.call(RENDER_STATE_00B97FF0, &args![0u32, 1u32, 0u32]);
}

// Translated from 00714b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Subtracts `count` from the counter at `011ffa00`, then calls
/// `00b980c0(0, 0, 0, 0)`.
pub fn fn_00714b20(e: &mut Engine, count: u32) {
    render_state_counter(e, RENDER_COUNTER_011FFA00, count);
    e.call(RENDER_STATE_00B980C0, &args![0u32, 0u32, 0u32, 0u32]);
}

// Translated from 00714b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Subtracts `count` from the counter at `011ffa04`, then calls
/// `00b98180(0, 0, 0xff, 0)`.
pub fn fn_00714b50(e: &mut Engine, count: u32) {
    render_state_counter(e, RENDER_COUNTER_011FFA04, count);
    e.call(RENDER_STATE_00B98180, &args![0u32, 0u32, 0xffu32, 0u32]);
}

// Translated from 00714b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Subtracts `count` from the counter at `011ffa08`, then calls
/// `00b98230(0xff, 0)`.
pub fn fn_00714b80(e: &mut Engine, count: u32) {
    render_state_counter(e, RENDER_COUNTER_011FFA08, count);
    e.call(RENDER_STATE_00B98230, &args![0xffu32, 0u32]);
}

// Translated from 00714bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Subtracts `count` from the counter at `011ffa0c`, then calls
/// `00b984f0(0, 0)`.
pub fn fn_00714bb0(e: &mut Engine, count: u32) {
    render_state_counter(e, RENDER_COUNTER_011FFA0C, count);
    e.call(RENDER_STATE_00B984F0, &args![0u32, 0u32]);
}

// Translated from 00714bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Subtracts `count` from the counter at `011ffa10`.
pub fn fn_00714bd0(e: &mut Engine, count: u32) {
    render_state_counter(e, RENDER_COUNTER_011FFA10, count);
}

// Translated from 00714bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Subtracts `count` from the counter at `011ff9e4`, then calls
/// `BSRenderState::SetDepthBias` (Xbox PDB, `00b98320`) with `(0.0, 0)`.
pub fn fn_00714bf0(e: &mut Engine, count: u32) {
    render_state_counter(e, RENDER_COUNTER_011FF9E4, count);
    e.call(RENDER_STATE_00B98320, &args![0.0f32, 0u32]);
}

// Translated from 00714c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Subtracts `count` from the counter at `011ffa20`, then calls
/// `00b98480(0, 0)`.
pub fn fn_00714c20(e: &mut Engine, count: u32) {
    render_state_counter(e, RENDER_COUNTER_011FFA20, count);
    e.call(RENDER_STATE_00B98480, &args![0u32, 0u32]);
}

/// `*counter -= count` (wrapping), the first step of every render-state
/// counter function.
fn render_state_counter(e: &mut Engine, counter: u32, count: u32) {
    let value = e.global::<u32>(counter).wrapping_sub(count);
    e.set_global(counter, value);
}

// Callees and data of `00714c40` .. `007170a0`.
const RENDER_COUNTER_011FFA28: u32 = 0x011f_fa28;
const RENDER_STATE_00B98540: u32 = 0x00b9_8540;
/// `Interface::CloseLoadingMenu` (Xbox PDB, `cdecl`) and
/// `Interface::HideMenus` (Xbox PDB, `cdecl`).
const CLOSE_LOADING_MENU: u32 = 0x0070_5e30;
const HIDE_MENUS: u32 = 0x0070_3610;
/// The class number of the loading menu.
const LOADING_MENU_CLASS: u32 = 0x3ef;
/// The class number of the console in the menu stack.
const CONSOLE_STACK_ENTRY: u32 = 3;
/// `() -> object`: the word at `011f91ac`, the object the menu stack tells
/// when its first or second entry is set, and `(object, byte)`, the call
/// that tells it.
const STACK_NOTIFY_OWNER_GET: u32 = 0x004e_3270;
const STACK_NOTIFY: u32 = 0x0071_23a0;
/// The text `AddToEnterStack` logs when the stack is full.
const STACK_FULL_MESSAGE: u32 = 0x0106_f2f0;
/// `HUDMainMenu` call that forgets a reference (`cdecl(reference)`).
const HUD_FORGET_REFERENCE: u32 = 0x0077_8b40;
/// `FORenderedTerminal::ReleaseStaticGeometry` (Xbox PDB),
/// `MapMenu::ClearWorldMapTexture` (Xbox PDB) and
/// `TES::CleanUpUnusedTextures(bool)` (Xbox PDB, called on the object at
/// `011dea10`).
const RELEASE_STATIC_GEOMETRY: u32 = 0x007f_fe00;
const CLEAR_WORLD_MAP_TEXTURE: u32 = 0x007a_1670;
const CLEAN_UP_UNUSED_TEXTURES: u32 = 0x0045_2490;
/// The renderer's width and height getters (`int` in `EAX`).
const RENDER_TARGET_WIDTH: u32 = 0x004d_c1f0;
const RENDER_TARGET_HEIGHT: u32 = 0x004d_c200;
/// `1280.0` and `960.0` as `float`s, the same as `double`s: the base
/// resolution of the interface.
const BASE_WIDTH_FLOAT: u32 = 0x0106_ec38;
const BASE_HEIGHT_FLOAT: u32 = 0x0106_f2dc;
const BASE_WIDTH_DOUBLE: u32 = 0x0106_e960;
const BASE_HEIGHT_DOUBLE: u32 = 0x0106_e7f8;
/// `0.0` (`double`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `3.0` (`double`), the offset added to the cursor position.
const CURSOR_X_OFFSET: u32 = 0x0102_1928;
/// The `NiPoint3` static the cursor code remembers the last position in,
/// and the guard byte word that tells whether its constructor has run.
const LAST_CURSOR_POINT: u32 = 0x011d_8c7c;
const LAST_CURSOR_POINT_GUARD: u32 = 0x011d_8c88;
/// Whether two points differ (`!(a == b)`, `004390c0` underneath):
/// `(point, other) -> bool`.
const POINT_NOT_EQUAL: u32 = 0x0043_9090;
/// The byte at `+0xc` of the manager (`cMenuMode`), read through a getter.
const MENU_MODE_BYTE_GETTER: u32 = 0x0042_4940;
/// `"Data\Menus\Main\safe_zone.xml"`.
const SAFE_ZONE_PATH: u32 = 0x0106_f334;
/// `Tile::GetValue`-style lookup that returns the entry of a trait (or 0);
/// the entry's `float` at `+8` is read through `00488d50`.
const TILE_GET_TRAIT_ENTRY: u32 = 0x00a0_0f30;
const TRAIT_ENTRY_VALUE: u32 = 0x0048_8d50;
/// The `float` at `011ac3a0`, the divisor `UpdateAllTimers` applies in game
/// state 4.
const TIMER_RATE: u32 = 0x011a_c3a0;
/// The object `TES::CleanUpUnusedTextures` is called on.
const TEXTURE_CLEANUP_OWNER: u32 = 0x011d_ea10;
/// The two string lists whose limits `00715e00` sets.
const LIMIT_LIST_WIDTH: u32 = 0x011d_8bd8;
const LIMIT_LIST_HEIGHT: u32 = 0x011d_8bf0;
/// The second repeat delay holder (`REPEAT_DELAY_HOLDER_FIRST` is the first).
const REPEAT_DELAY_HOLDER_NEXT: u32 = 0x011d_8b88;
/// The string (`01020770`) the shown copy of an empty text falls back to.
const DISPLAY_FALLBACK_STRING: u32 = 0x0102_0770;
/// `BSStringT::GetLength` (the length, computed when not cached), `strcpy`
/// (`cdecl(dest, source)`) and `BSStringT::Set` (`(this, text)`).
const STRING_LENGTH: u32 = 0x0040_48e0;
const TEXT_COPY: u32 = 0x0040_46f0;
const STRING_SET: u32 = 0x0043_8390;
/// `(text_entry, text) -> bool`: whether the edited text may grow to that
/// text (`fn_00717230`, in another part of this unit).
const TEXT_ENTRY_ACCEPTS_TEXT: u32 = 0x0071_7230;

layout! {
    /// An unnamed text-entry line (constructed by `00716980`; the menu
    /// console embeds one at `+0x34`), 0x24 bytes. Two `BSStringT`s open it:
    /// the text at `+0` and the copy shown on screen, with the caret in it,
    /// at `+8`.
    pub struct TextEntry: 0x24 {
        /// Index of the caret in the text.
        0x10 cursor: i32,
        /// Longest width the text may have (`-1` is no limit).
        0x14 max_width: i32,
        /// Word the constructor sets to 1.
        0x18 field_18: u32,
        /// Tick count of the last caret blink.
        0x1c last_blink_time: u32,
        /// Caret phase: the shown caret is `|` when set, `0x7f` when clear.
        0x20 caret_phase: u8,
        /// Whether the entry takes keys and shows a caret.
        0x21 active: u8,
        /// Set aside by every key; when set, the next editing key first
        /// empties the text.
        0x22 clears_text_on_edit: u8,
    }
}

// Translated from 00714c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Subtracts `count` from the counter at `011ffa28`, then calls
/// `00b98540(0, 0)` (the render-state counter functions of `00714a40`).
pub fn fn_00714c40(e: &mut Engine, count: u32) {
    render_state_counter(e, RENDER_COUNTER_011FFA28, count);
    e.call(RENDER_STATE_00B98540, &args![0u32, 0u32]);
}

/// The word `index` of the menu stack (`iEnterStack`, ten words).
fn enter_stack_word(e: &Engine, this: Ptr<InterfaceManager>, index: u32) -> u32 {
    e.mem.u32(this.addr() + 0x114 + 4 * index)
}

fn set_enter_stack_word(e: &mut Engine, this: Ptr<InterfaceManager>, index: u32, value: u32) {
    e.mem.set_u32(this.addr() + 0x114 + 4 * index, value);
}

/// Index of the first empty word of the menu stack among the first
/// `limit`, or `limit`.
fn first_empty_enter_stack_word(e: &Engine, this: Ptr<InterfaceManager>, limit: u32) -> u32 {
    let mut index = 0;
    while index < limit && enter_stack_word(e, this, index) != 0 {
        index += 1;
    }
    index
}

/// Tells the object at `011f91ac` (when there is one) about the stack:
/// `007123a0(owner, 1)`; the caller has already checked the owner exists.
fn notify_stack_owner(e: &mut Engine, flag: u32) {
    let owner = e.call(STACK_NOTIFY_OWNER_GET, &args![]).u32();
    e.call(STACK_NOTIFY, &args![owner, flag]);
}

// Translated from 00714c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::EmergencyCloseAllMenusAndBreakStuff` (Xbox PDB): walks
/// the menu stack from its last word to its first; every entry is closed
/// (the console is hidden, the loading menu is closed through its own call,
/// any other menu gets its virtual destructor with flag 1) and cleared. Then
/// the menus are hidden, `cMenuMode` becomes 4 and the byte at `0119f348`
/// is set.
pub fn interface_manager_emergency_close_all_menus_and_break_stuff(
    e: &mut Engine,
    this: Ptr<InterfaceManager>,
) {
    for index in (0..10u32).rev() {
        let class = enter_stack_word(e, this, index);
        if class == 0 {
            continue;
        }
        let tile = e.call(TILE_GET_MENU_BY_CLASS, &args![class]).u32();
        if tile == 0 {
            if class == CONSOLE_STACK_ENTRY
                && e.call(MENU_CONSOLE_INSTANCE, &args![0u32]).u32() != 0
            {
                let console = e.call(MENU_CONSOLE_INSTANCE, &args![1u32]).u32();
                e.call(MENU_CONSOLE_TOGGLE_VISIBLE, &args![console]);
            }
        } else {
            let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
            if menu != 0 {
                let loading = e
                    .call(TILE_GET_MENU_BY_CLASS, &args![LOADING_MENU_CLASS])
                    .u32();
                if tile == loading {
                    e.call(CLOSE_LOADING_MENU, &args![]);
                } else {
                    delete_virtual(e, menu);
                }
            }
        }
        set_enter_stack_word(e, this, index, 0);
    }
    e.call(HIDE_MENUS, &args![]);
    e.set(this, InterfaceManager::cMenuMode, 4);
    e.set_global(START_MENU_ALLOWED, 1u8);
}

// Translated from 00714d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `reference` as `pPickRef` (Xbox PDB).
pub fn fn_00714d70(e: &mut Engine, this: Ptr<InterfaceManager>, reference: u32) {
    e.set(this, InterfaceManager::pPickRef, reference);
}

// Translated from 00714d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::AddToEnterStack` (Xbox PDB): puts a menu class on the
/// menu stack and returns the slot it went to (`-1` when the stack is
/// full). Class 3 (the console) goes first and pushes the others down by
/// one, logging when the last of the nine words is lost; any other class
/// goes in the first free word. When the stack gets its first entry the mode
/// becomes 3, and the object at `011f91ac` is told (except for the class
/// `0x3e9` as the first entry); it is told too when the class `0x3e9`
/// becomes the first of two.
pub fn interface_manager_add_to_enter_stack(
    e: &mut Engine,
    this: Ptr<InterfaceManager>,
    class: i32,
) -> i32 {
    if class == CONSOLE_STACK_ENTRY as i32 {
        let mut carried = enter_stack_word(e, this, 0);
        set_enter_stack_word(e, this, 0, CONSOLE_STACK_ENTRY);
        let mut index = 1;
        while index < 9 {
            let displaced = enter_stack_word(e, this, index);
            set_enter_stack_word(e, this, index, carried);
            carried = displaced;
            if displaced == 0 {
                break;
            }
            index += 1;
        }
        if index == 9 {
            e.call(LOG_WARNING, &args![STACK_FULL_MESSAGE]);
        }
        if index == 1 {
            e.set(this, InterfaceManager::cMenuMode, 3);
            if e.call(STACK_NOTIFY_OWNER_GET, &args![]).u32() != 0 {
                notify_stack_owner(e, 1);
            }
        }
        return index as i32;
    }
    let index = first_empty_enter_stack_word(e, this, 10);
    if index >= 10 {
        return -1;
    }
    set_enter_stack_word(e, this, index, class as u32);
    if index == 0 {
        e.set(this, InterfaceManager::cMenuMode, 3);
        if class != 0x3e9 && e.call(STACK_NOTIFY_OWNER_GET, &args![]).u32() != 0 {
            notify_stack_owner(e, 1);
        }
    } else if index == 1
        && enter_stack_word(e, this, 0) == 0x3e9
        && e.call(STACK_NOTIFY_OWNER_GET, &args![]).u32() != 0
    {
        notify_stack_owner(e, 1);
    }
    index as i32
}

// Translated from 00714f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::GetEnterStackTop` (Xbox PDB): the last used word of
/// the menu stack (0 when it is empty; the last word when it is full).
pub fn interface_manager_get_enter_stack_top(e: &mut Engine, this: Ptr<InterfaceManager>) -> u32 {
    let index = first_empty_enter_stack_word(e, this, 10);
    if index < 10 {
        if index == 0 {
            0
        } else {
            e.mem.u32(this.addr() + 0x110 + 4 * index)
        }
    } else {
        e.mem.u32(this.addr() + 0x138)
    }
}

// Translated from 00714f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::GetEnterStack` (Xbox PDB): word `index` of the menu
/// stack, unchecked.
pub fn interface_manager_get_enter_stack(
    e: &mut Engine,
    this: Ptr<InterfaceManager>,
    index: u32,
) -> u32 {
    enter_stack_word(e, this, index)
}

// Translated from 00714f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `class` is one of the ten words of the menu stack.
pub fn fn_00714f90(e: &mut Engine, this: Ptr<InterfaceManager>, class: i32) -> bool {
    (0..10u32).any(|index| interface_manager_get_enter_stack(e, this, index) == class as u32)
}

// Translated from 00714fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::PopFromEnterStack` (Xbox PDB): takes a menu class off
/// the menu stack. Returns `-1` when the stack is empty or (without
/// `force`) the class is not on it, `-2` when (without `force`) it is on it
/// below an entry of class `0x3e9` or lower and is not the console. The
/// stack closes up over every word equal to `class`. Afterwards: a console
/// left alone on the stack is hidden and popped as well; an empty stack hides
/// the menus when the class was 1, sets `cMenuMode` to 4 and tells the object
/// at `011f91ac` (flag: the class is not `0x3e9`) and returns 0; otherwise the
/// object is told when `0x3e9` is the only entry, and the word under the
/// last used one is returned.
pub fn interface_manager_pop_from_enter_stack(
    e: &mut Engine,
    this: Ptr<InterfaceManager>,
    class: i32,
    force: u8,
) -> i32 {
    if enter_stack_word(e, this, 0) == 0 {
        return -1;
    }
    let class_word = class as u32;
    if force == 0 {
        let mut found = false;
        let mut index = 0;
        while index < 10 && (!found || (enter_stack_word(e, this, index) as i32) <= 0x3e9) {
            if enter_stack_word(e, this, index) == class_word {
                found = true;
            }
            index += 1;
        }
        if found && index < 10 && class != CONSOLE_STACK_ENTRY as i32 {
            return -2;
        }
        if !found {
            return -1;
        }
    }
    let mut source = 0;
    let mut index = 0;
    while index < 10 {
        if enter_stack_word(e, this, index) == class_word {
            source += 1;
        }
        let word = enter_stack_word(e, this, source);
        set_enter_stack_word(e, this, index, word);
        if word == 0 {
            break;
        }
        index += 1;
        source += 1;
    }
    if enter_stack_word(e, this, 0) == CONSOLE_STACK_ENTRY && enter_stack_word(e, this, 1) == 0 {
        if e.call(MENU_CONSOLE_INSTANCE, &args![0u32]).u32() != 0 {
            let console = e.call(MENU_CONSOLE_INSTANCE, &args![1u32]).u32();
            e.call(MENU_CONSOLE_TOGGLE_VISIBLE, &args![console]);
        }
        interface_manager_pop_from_enter_stack(e, this, CONSOLE_STACK_ENTRY as i32, 0)
    } else if index < 1 {
        if class == 1 {
            e.call(HIDE_MENUS, &args![]);
        }
        e.set(this, InterfaceManager::cMenuMode, 4);
        notify_stack_owner(e, (class != 0x3e9) as u32);
        0
    } else {
        if index == 1 && enter_stack_word(e, this, 0) == 0x3e9 {
            notify_stack_owner(e, 1);
        }
        e.mem.u32(this.addr() + 0x110 + 4 * index) as i32
    }
}

// Translated from 007151b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Puts the cursor at the fraction `(x_fraction, z_fraction)` of the
/// screen: the cursor node's position (kept at `+0x2c`, `+0x34`) becomes
/// `(width * x - width / 2, height / 2 - z * height)` in interface units, the
/// node is moved and updated, and the position in desktop units is worked
/// out. When it changed, it is kept in the static point at `011d8c7c`, the
/// tilt of the cursor sets its middle component, and the real position goes
/// to `+0x38 .. +0x40`; with the mouse shown the motion flag is set, and in
/// menu mode 2 the cursor node is shown and the cursor tile's trait `0xfa3`
/// is set. When the real position still differs from the stored one it is
/// scaled again the way `Idle` does. The first call constructs the static
/// point.
pub fn fn_007151b0(e: &mut Engine, this: Ptr<InterfaceManager>, x_fraction: f32, z_fraction: f32) {
    if e.global::<u32>(LAST_CURSOR_POINT_GUARD) & 1 == 0 {
        let guard = e.global::<u32>(LAST_CURSOR_POINT_GUARD);
        e.set_global(LAST_CURSOR_POINT_GUARD, guard | 1);
        e.call(MEMBER_CONSTRUCTOR_EMPTY, &args![LAST_CURSOR_POINT]);
    }
    let base = this.addr();
    let two: f64 = e.global(TWO);
    let cursor = e.get(this, InterfaceManager::pCursor);
    let node = e.call(TILE_IMAGE_NODE, &args![cursor]).u32();
    let translation = e.call(NODE_TRANSLATION, &args![node]).u32();
    for word in 0..3 {
        let value = e.mem.u32(translation + 4 * word);
        e.mem.set_u32(base + 0x2c + 4 * word, value);
    }
    let width = interface_manager_get_screen_width(e) as f64;
    let scaled_x = width * x_fraction as f64;
    let width = interface_manager_get_screen_width(e) as f64;
    e.mem.set_f32(base + 0x2c, (scaled_x - width / two) as f32);
    let height = fn_00715da0(e) as f64;
    let scaled_z = height * -(z_fraction as f64);
    let height = fn_00715da0(e) as f64;
    e.mem.set_f32(base + 0x34, (height / two + scaled_z) as f32);
    let cursor = e.get(this, InterfaceManager::pCursor);
    let node = e.call(TILE_IMAGE_NODE, &args![cursor]).u32();
    e.call(NODE_SET_TRANSLATE_VECTOR, &args![node, base + 0x2c]);
    e.with_stack(12, |e, update_data| {
        e.call(
            NI_UPDATE_DATA_CONSTRUCT,
            &args![update_data, 0.0f32, 0u32, 0u32],
        );
        let cursor = e.get(this, InterfaceManager::pCursor);
        let node = e.call(TILE_IMAGE_NODE, &args![cursor]).u32();
        e.call(NODE_UPDATE, &args![node, update_data]);
    });
    e.with_stack(12, |e, point| {
        e.call(MEMBER_CONSTRUCTOR_EMPTY, &args![point]);
        let offset: f64 = e.global(CURSOR_X_OFFSET);
        let shifted_x = e.mem.f32(base + 0x2c) as f64 + offset;
        let desktop_width = e.call(GET_DESKTOP_WIDTH, &args![]).f64();
        let width = interface_manager_get_screen_width(e) as f64;
        let scaled = (desktop_width / width) * shifted_x;
        let desktop_width = e.call(GET_DESKTOP_WIDTH, &args![]).f64();
        e.mem
            .set_f32(point.addr(), (desktop_width / two + scaled) as f32);
        e.mem.set_f32(point.addr() + 4, 0.0);
        let desktop_height = e.call(GET_DESKTOP_HEIGHT, &args![]).f64();
        let half_height = desktop_height / two;
        let z = e.mem.f32(base + 0x34) as f64;
        let desktop_height = e.call(GET_DESKTOP_HEIGHT, &args![]).f64();
        let height = fn_00715da0(e) as f64;
        e.mem.set_f32(
            point.addr() + 8,
            (half_height - (desktop_height / height) * z) as f32,
        );
        if e.call(POINT_NOT_EQUAL, &args![point, LAST_CURSOR_POINT])
            .bool()
        {
            for word in 0..3 {
                let value = e.mem.u32(point.addr() + 4 * word);
                e.mem.set_u32(LAST_CURSOR_POINT + 4 * word, value);
            }
            let cursor = e.get(this, InterfaceManager::pCursor);
            let tilt = tile_get_float(e, cursor, 0xfad);
            let tilt_scale: f64 = e.global(CURSOR_TILT_SCALE);
            e.mem.set_f32(point.addr() + 4, (tilt * tilt_scale) as f32);
            for word in 0..3 {
                let value = e.mem.u32(point.addr() + 4 * word);
                e.mem.set_u32(base + 0x38 + 4 * word, value);
            }
            if !e.call(HAS_360_CONTROLLER_GETTER, &args![]).bool()
                || e.get(this, InterfaceManager::bShowMouse) != 0
            {
                e.set(this, InterfaceManager::bMouseInMotion, 1);
                if e.call(MENU_MODE_BYTE_GETTER, &args![this]).u8() as i8 == 2 {
                    let cursor = e.get(this, InterfaceManager::pCursor);
                    let node = e.call(TILE_IMAGE_NODE, &args![cursor]).u32();
                    e.call(NODE_SET_FLAG, &args![node, 0u32]);
                    let cursor = e.get(this, InterfaceManager::pCursor);
                    tile_set_int(e, cursor, 0xfa3, 1);
                }
            }
        }
        let new_x = e.mem.f32(point.addr());
        let new_z = e.mem.f32(point.addr() + 8);
        if e.mem.f32(base + 0x38) != new_x || e.mem.f32(base + 0x40) != new_z {
            e.set(this, InterfaceManager::bMouseInMotion, 1);
            let desktop_width = e.call(GET_DESKTOP_WIDTH, &args![]).f64();
            let width = interface_manager_get_screen_width(e) as f64;
            let scaled = (desktop_width / width) * e.mem.f32(base + 0x38) as f64;
            let desktop_width = e.call(GET_DESKTOP_WIDTH, &args![]).f64();
            e.mem
                .set_f32(base + 0x38, (desktop_width / two + scaled) as f32);
            let cursor = e.get(this, InterfaceManager::pCursor);
            let tilt = tile_get_float(e, cursor, 0xfad);
            let tilt_scale: f64 = e.global(CURSOR_TILT_SCALE);
            e.mem.set_f32(base + 0x3c, (tilt * tilt_scale) as f32);
            let desktop_height = e.call(GET_DESKTOP_HEIGHT, &args![]).f64();
            let half_height = desktop_height / two;
            let desktop_height = e.call(GET_DESKTOP_HEIGHT, &args![]).f64();
            let height = fn_00715da0(e) as f64;
            let z = e.mem.f32(base + 0x40) as f64;
            e.mem.set_f32(
                base + 0x40,
                (half_height - (desktop_height / height) * z) as f32,
            );
        }
    });
}

// Translated from 007154b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Turns a key event of the interface into the code the text entries take:
/// `event` 2 is a key release (shift, alt and control clear their bits of
/// `iModifierKeys`, shift also `bShiftDown`; the key repeat is reset) and 1 a
/// key press. A press asks the controls for the character of the scan code
/// with the shift state; Escape (`0x1b`) yields 0 unless `00715770` says a
/// text entry is up; backspace, the arrows, home, end, delete, page up/down
/// and enter yield the codes `0x8000000x` (and the arrows, backspace and
/// delete start the key repeat); the modifier keys set their bits; any other
/// key yields the character. Any other event yields 0.
pub fn fn_007154b0(e: &mut Engine, this: Ptr<InterfaceManager>, event: i32, key: i32) -> u32 {
    let modifiers = |e: &mut Engine, keep: Option<u32>, set: u32| {
        let value = e.get(this, InterfaceManager::iModifierKeys);
        let value = match keep {
            Some(mask) => value & mask as i32,
            None => value | set as i32,
        };
        e.set(this, InterfaceManager::iModifierKeys, value);
    };
    if event == 2 {
        if key == 0x2a || key == 0x36 {
            modifiers(e, Some(0xfffb), 0);
            e.set(this, InterfaceManager::bShiftDown, 0);
        } else if key == 0x38 || key == 0xb8 {
            modifiers(e, Some(0xfffe), 0);
        } else if key == 0x1d || key == 0x9d {
            modifiers(e, Some(0xfffd), 0);
        }
        fn_007166f0(e, this, 0);
        return 0;
    }
    if event != 1 {
        return 0;
    }
    let shift = e.get(this, InterfaceManager::bShiftDown) as u32;
    let controls_owner = e.global::<u32>(CONTROLS_OWNER);
    let controls = e.call(CONTROLS_GET, &args![controls_owner]).u32();
    let character = e
        .call(CONTROLS_QUERY_00A238A0, &args![controls, key, shift])
        .u32();
    if character == 0x1b && !fn_00715770(e) {
        0
    } else if character == 8 {
        fn_007166f0(e, this, 0x8000_0000);
        0x8000_0000
    } else if character == 0x7c {
        0
    } else if character == 0xd {
        0x8000_0008
    } else if key == 0x2a || key == 0x36 {
        modifiers(e, None, 4);
        e.set(this, InterfaceManager::bShiftDown, 1);
        // The key itself is not a character (the code falls out here).
        0
    } else if key == 0x38 || key == 0xb8 {
        modifiers(e, None, 1);
        0
    } else if key == 0x1d || key == 0x9d {
        modifiers(e, None, 2);
        0
    } else if let Some(&(_, code, repeats)) = TEXT_ENTRY_KEYS.iter().find(|(k, _, _)| *k == key) {
        if repeats {
            fn_007166f0(e, this, code);
        }
        code
    } else {
        character
    }
}

/// The scan codes `fn_007154b0` turns into editing codes: arrows (`0xcb`
/// left, `0xcd` right, `0xc8` up, `0xd0` down; these start the key repeat),
/// home `0xc7`, end `0xcf`, page down `0xd1`, page up `0xc9` and delete
/// `0xd3` (which starts it too).
const TEXT_ENTRY_KEYS: [(i32, u32, bool); 9] = [
    (0xcb, 0x8000_0001, true),
    (0xcd, 0x8000_0002, true),
    (0xc8, 0x8000_0003, true),
    (0xd0, 0x8000_0004, true),
    (0xc7, 0x8000_0005, false),
    (0xcf, 0x8000_0006, false),
    (0xd1, 0x8000_000a, false),
    (0xc9, 0x8000_0009, false),
    (0xd3, 0x8000_0007, true),
];

// Translated from 00715770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the extra interface (`XUserInterface::XUIIsUp`, `0070edf0`) is up
/// and the object at `011daac0` has the flag `0x80000`.
pub fn fn_00715770(e: &mut Engine) -> bool {
    if e.call(XUI_IS_UP, &args![]).bool() {
        let owner = e.global::<u32>(FLAGS_OWNER);
        if e.call(HAS_FLAG, &args![owner, 0x80000u32]).bool() {
            return true;
        }
    }
    false
}

/// Tells the menu of `tile` that the pointer or focus left it: the tile's
/// trait `0xfc3` is cleared and `DoLeave(menu, id, tile)` is called with the
/// menu of the tile and the id in its trait `0xfaa`.
fn leave_tile(e: &mut Engine, this: Ptr<InterfaceManager>, tile: u32) {
    tile_set_int(e, tile, 0xfc3, 0);
    let id = tile_menu_id(e, tile);
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    e.call(DO_LEAVE, &args![this, menu, id as u32, tile]);
}

// Translated from 007157b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::ClearOverTileTarget` (Xbox PDB): with `leave` set and
/// both the tile under the pointer (`pOverTileTarget`) and its menu
/// (`pOverTileMenu`) known, the tile's traits `0xfc3` and `0xfc7` are cleared
/// and the menu is told (`DoLeave(menu, id, tile)`, id from trait `0xfaa`).
/// Both are forgotten whatever `leave` is.
pub fn interface_manager_clear_over_tile_target(
    e: &mut Engine,
    this: Ptr<InterfaceManager>,
    leave: u8,
) {
    let tile = e.get(this, InterfaceManager::pOverTileTarget);
    let menu = e.get(this, InterfaceManager::pOverTileMenu);
    if leave != 0 && menu != 0 && tile != 0 {
        tile_set_int(e, tile, 0xfc3, 0);
        tile_set_int(e, tile, 0xfc7, 0);
        let id = tile_menu_id(e, tile);
        e.call(DO_LEAVE, &args![this, menu, id as u32, tile]);
    }
    e.set(this, InterfaceManager::pOverTileTarget, 0);
    e.set(this, InterfaceManager::pOverTileMenu, 0);
}

/// Counts a focus change of the tile `tile` when its trait `0xfd6` is
/// positive: `iLastXDefault` goes up by one and becomes that trait.
fn count_focus_change(e: &mut Engine, this: Ptr<InterfaceManager>, tile: u32) {
    let value = tile_get_float(e, tile, 0xfd6);
    if value > e.global::<f64>(ZERO_DOUBLE) {
        let count = e.get(this, InterfaceManager::iLastXDefault).wrapping_add(1);
        e.set(this, InterfaceManager::iLastXDefault, count);
        fn_00715c60(e, tile, 0xfd6, count);
    }
}

// Translated from 00715860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::SetCurrentFocusTarget` (Xbox PDB): moves the focus
/// (`pMouseOverTarget`) to `tile`. `trait_id` says which kind of focus it is:
/// `0xfc3` (the tile is entered; its menu is told with `DoEnter`, and with
/// `play_sound` the tile sound `0xfe8` plays) or `0xfc7` (the tile is
/// clicked: the sound named by its trait `0xfcb` plays with `play_sound`, its
/// trait `0xfc7` pulses, its menu is called through slot `0xc` with the id and
/// the tile, its children are updated, and the old focus is left unless it
/// is still visible and has trait `0xfaf`). A null tile clears the focus. The
/// tile under the pointer (`pOverTileTarget`) is left whenever a tile is
/// given.
pub fn interface_manager_set_current_focus_target(
    e: &mut Engine,
    this: Ptr<InterfaceManager>,
    tile: u32,
    trait_id: u32,
    play_sound: u8,
) {
    let focus = e.get(this, InterfaceManager::pMouseOverTarget);
    if focus != 0 && focus != tile {
        count_focus_change(e, this, focus);
    }
    if tile != 0 {
        let over = e.get(this, InterfaceManager::pOverTileTarget);
        if over != 0 {
            leave_tile(e, this, over);
        }
        e.set(this, InterfaceManager::pOverTileTarget, 0);
        e.set(this, InterfaceManager::pOverTileMenu, 0);
    }
    if tile == 0 {
        let focus = e.get(this, InterfaceManager::pMouseOverTarget);
        if focus != 0 {
            leave_tile(e, this, focus);
        }
        interface_manager_clear_over_tile_target(e, this, 0);
        e.set(this, InterfaceManager::pMouseOverTarget, 0);
    } else if trait_id == 0xfc3 && e.get(this, InterfaceManager::pMouseOverTarget) != tile {
        let focus = e.get(this, InterfaceManager::pMouseOverTarget);
        if focus != 0 {
            leave_tile(e, this, focus);
        }
        e.set(this, InterfaceManager::pMouseOverTarget, tile);
        count_focus_change(e, this, tile);
        tile_set_int(e, tile, 0xfc3, 1);
        if play_sound != 0 {
            e.call(TILE_PLAY_TILE_SOUND, &args![tile, 0xfe8u32]);
        }
        let id = tile_menu_id(e, tile);
        let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
        e.call(DO_ENTER, &args![this, menu, id as u32, tile]);
    } else if trait_id == 0xfc7 {
        let sound = {
            let value = tile_get_float(e, tile, 0xfcb);
            e.call(FTOL, &args![value]).i32()
        };
        if play_sound != 0 && sound != 0 {
            e.call(PLAY_MENU_SOUND, &args![sound as u32]);
        }
        tile_set_int(e, tile, 0xfc7, 1);
        tile_set_int(e, tile, 0xfc7, 0);
        let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
        let id = tile_menu_id(e, tile);
        e.vcall(menu, 0xc, &args![id as u32, tile]);
        e.call(TILE_UPDATE_CHILDREN, &args![tile, 0u32]);
        let focus = e.get(this, InterfaceManager::pMouseOverTarget);
        if focus != 0 {
            if e.call(TILE_IS_VISIBLE, &args![focus]).bool()
                && e.call(TILE_IS_TRUE, &args![focus, 0xfafu32]).bool()
            {
                return;
            }
            leave_tile(e, this, focus);
            interface_manager_clear_over_tile_target(e, this, 0);
            e.set(this, InterfaceManager::pMouseOverTarget, 0);
        }
    }
}

// Translated from 00715c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the trait `trait_id` of `tile` to `value` as a float
/// (`Tile::SetFloat(trait, (float)value, true)`; the integer is unsigned).
pub fn fn_00715c60(e: &mut Engine, tile: u32, trait_id: u32, value: u32) {
    tile_set_float(e, tile, trait_id, (value as u64) as f32);
}

// Translated from 00715ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::GetDefaultFocus` (Xbox PDB): scans the menu for the
/// tile with the greatest focus (`ScanForMaxFocus`) and makes it the focus
/// (trait `0xfc3`, with sound); then, unless a 360 controller is in use
/// without the mouse, the cursor node is shown and its trait `0xfa3`
/// cleared. `bMouseInMotion` is cleared. With nothing found the focus is
/// cleared.
pub fn interface_manager_get_default_focus(e: &mut Engine, this: Ptr<InterfaceManager>) {
    let found = e.with_stack(4, |e, best| {
        e.mem.set_u32(best.addr(), 0x8000_0000);
        interface_manager_scan_for_max_focus(e, this, best.addr(), 0)
    });
    if found == 0 {
        interface_manager_set_current_focus_target(e, this, 0, 0xfc3, 1);
        return;
    }
    interface_manager_set_current_focus_target(e, this, found, 0xfc3, 1);
    if !e.call(HAS_360_CONTROLLER_GETTER, &args![]).bool()
        || e.get(this, InterfaceManager::bShowMouse) != 0
    {
        let cursor = e.get(this, InterfaceManager::pCursor);
        let node = e.call(TILE_IMAGE_NODE, &args![cursor]).u32();
        e.call(NODE_SET_FLAG, &args![node, 1u32]);
        let cursor = e.get(this, InterfaceManager::pCursor);
        tile_set_int(e, cursor, 0xfa3, 0);
    }
    e.set(this, InterfaceManager::bMouseInMotion, 0);
}

/// `(float)width`, `(float)height` of the renderer as the screen-shape
/// functions read them.
fn render_target_size(e: &mut Engine) -> (f32, f32) {
    let width = e.call(RENDER_TARGET_WIDTH, &args![]).i32() as f32;
    let height = e.call(RENDER_TARGET_HEIGHT, &args![]).i32() as f32;
    (width, height)
}

// Translated from 00715d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::GetScreenWidth` (Xbox PDB, `float` in `ST0`): the
/// width of the interface space, `1280` unless the render target is wider
/// than tall, then `width / height * 960`.
pub fn interface_manager_get_screen_width(e: &mut Engine) -> f32 {
    let (width, height) = render_target_size(e);
    if height < width {
        let base: f64 = e.global(BASE_HEIGHT_DOUBLE);
        (width as f64 / height as f64 * base) as f32
    } else {
        e.global(BASE_WIDTH_FLOAT)
    }
}

// Translated from 00715da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The height of the interface space (`float` in `ST0`): `960` unless the
/// render target is taller than wide, then `height / width * 1280`.
pub fn fn_00715da0(e: &mut Engine) -> f32 {
    let (width, height) = render_target_size(e);
    if width < height {
        let base: f64 = e.global(BASE_WIDTH_DOUBLE);
        (height as f64 / width as f64 * base) as f32
    } else {
        e.global(BASE_HEIGHT_FLOAT)
    }
}

// Translated from 00715e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the screen metrics on the root menus tile: the string lists at
/// `011d8bd8` and `011d8bf0` get the limits `width` and `height` (the second
/// takes `width` when `height` is not positive; both are left alone when
/// `width` is not positive), then the traits `0xfc0`, `0xfbf` (the interface
/// margins), `0xfb1` (the interface width) and `0xfb0` (its height) are set.
pub fn fn_00715e00(e: &mut Engine, this: Ptr<InterfaceManager>, width: i32, height: i32) {
    if width > 0 {
        e.call(STRING_LIST_SET_LIMIT, &args![LIMIT_LIST_WIDTH, width]);
        let second = if height > 0 { height } else { width };
        e.call(STRING_LIST_SET_LIMIT, &args![LIMIT_LIST_HEIGHT, second]);
    }
    let root = e.get(this, InterfaceManager::pMenusRoot);
    let margin_a = e.call(GET_INTERFACE_MARGIN_A, &args![]).f32();
    tile_set_float(e, root, 0xfc0, margin_a);
    let root = e.get(this, InterfaceManager::pMenusRoot);
    let margin_b = e.call(GET_INTERFACE_MARGIN_B, &args![]).f32();
    tile_set_float(e, root, 0xfbf, margin_b);
    let root = e.get(this, InterfaceManager::pMenusRoot);
    let interface_width = interface_manager_get_screen_width(e);
    tile_set_float(e, root, 0xfb1, interface_width);
    let root = e.get(this, InterfaceManager::pMenusRoot);
    let interface_height = fn_00715da0(e);
    tile_set_float(e, root, 0xfb0, interface_height);
}

// Translated from 00715ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::ToggleSafeZone` (Xbox PDB): with `mode` 3 the safe
/// zone tile (`pSafeZone`) is deleted. If there is none now, it is read from
/// `Data\Menus\Main\safe_zone.xml` under the menus root, shown (trait
/// `0xfa3`) and faded in (`RecursiveFade(node, 1.0, 0.0)`); if there is one,
/// it is deleted. (With a zone and `mode` 2 the code asks the zone's trait
/// `0xfa3`; with `mode` 1 or 0 it computes a flag it never uses.)
pub fn interface_manager_toggle_safe_zone(e: &mut Engine, this: Ptr<InterfaceManager>, mode: i32) {
    let zone = e.get(this, InterfaceManager::pSafeZone);
    if zone != 0 && mode == 2 {
        e.call(TILE_IS_TRUE, &args![zone, 0xfa3u32]);
    }
    if mode == 3 {
        let zone = e.get(this, InterfaceManager::pSafeZone);
        if zone != 0 {
            delete_virtual(e, zone);
        }
        e.set(this, InterfaceManager::pSafeZone, 0);
    }
    if e.get(this, InterfaceManager::pSafeZone) == 0 {
        let root = e.call(GET_MENUS_ROOT, &args![this]).u32();
        let zone = e.call(TILE_READ_FILE, &args![root, SAFE_ZONE_PATH]).u32();
        e.set(this, InterfaceManager::pSafeZone, zone);
        tile_set_int(e, zone, 0xfa3, 1);
        let zone = e.get(this, InterfaceManager::pSafeZone);
        let node = e.call(TILE_IMAGE_NODE, &args![zone]).u32();
        interface_manager_recursive_fade(e, this, node, 1.0, 0.0);
    } else {
        let zone = e.get(this, InterfaceManager::pSafeZone);
        delete_virtual(e, zone);
        e.set(this, InterfaceManager::pSafeZone, 0);
    }
}

// Translated from 00716010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forgets `reference` wherever the manager keeps it (`pPickRef`,
/// `pReticleRef`, `pCrossHairRef`, `pActivateRef`, `pTelekinesisRef`; Xbox
/// PDB) and tells the HUD to forget it too (`00778b40`).
pub fn fn_00716010(e: &mut Engine, this: Ptr<InterfaceManager>, reference: u32) {
    for offset in [0xf0u32, 0xf4, 0xf8, 0xfc, 0x100] {
        if e.mem.u32(this.addr() + offset) == reference {
            e.mem.set_u32(this.addr() + offset, 0);
        }
    }
    e.call(HUD_FORGET_REFERENCE, &args![reference]);
}

// Translated from 007160b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::ForceTextureRelease` (Xbox PDB): releases the
/// terminal's static geometry, calls slot `0x1c` of the root menus tile
/// object, clears the world map texture and cleans up the unused textures
/// (`TES::CleanUpUnusedTextures(true)` on the object at `011dea10`).
pub fn interface_manager_force_texture_release(e: &mut Engine, this: Ptr<InterfaceManager>) {
    e.call(RELEASE_STATIC_GEOMETRY, &args![]);
    let root = e.call(GET_MENUS_ROOT, &args![this]).u32();
    e.vcall(root, 0x1c, &args![]);
    e.call(CLEAR_WORLD_MAP_TEXTURE, &args![]);
    let owner = e.global::<u32>(TEXTURE_CLEANUP_OWNER);
    e.call(CLEAN_UP_UNUSED_TEXTURES, &args![owner, 1u32]);
}

/// `ftol(float of the trait entry)`: the integer value of the trait entry
/// `00488d50` reads.
fn trait_entry_int(e: &mut Engine, entry: u32) -> i32 {
    let value = e.call(TRAIT_ENTRY_VALUE, &args![entry]).f64();
    e.call(FTOL, &args![value]).i32()
}

// Translated from 007160f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::ScanForMaxFocus` (Xbox PDB): finds the tile with the
/// highest focus value (trait `0xfd6`) in the tree below `tile`, or, with
/// `tile` null, below the frontmost menu's tile (null when there is no menu,
/// or when it is a tile of type `0x389` whose menu is neither in state 1
/// nor 8). Returns null for an invisible tile (trait `0xfa3` clear). The best
/// value found is stored at `best` (the value there on entry is the one to
/// beat, `0x80000000` when the call is not a recursion). Equal values go to
/// the tile with the smaller order (trait `0xfac`). A tile with traits `0xfaf`
/// and `0xfa3` set competes with its children by its own `0xfd6` value.
pub fn interface_manager_scan_for_max_focus(
    e: &mut Engine,
    _this: Ptr<InterfaceManager>,
    best: u32,
    tile: u32,
) -> u32 {
    let mut found = 0u32;
    let mut best_value = e.mem.i32(best);
    let mut best_order = 0x7fff_ffffi32;
    e.mem.set_i32(best, i32::MIN);
    let mut tile = tile;
    if tile == 0 {
        let manager = e.call(MENU_MANAGER_INSTANCE, &args![1u32]).u32();
        let frontmost = e.call(GET_FRONTMOST_MENU, &args![manager]).u32();
        if frontmost == 0 {
            return 0;
        }
        tile = e.call(GET_FIELD_AT_4, &args![frontmost]).u32();
        if e.vcall(tile, 0xc, &args![]).u32() == 0x389 {
            let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
            if e.call(MENU_STATE, &args![menu]).u32() != 1 {
                let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
                if e.call(MENU_STATE, &args![menu]).u32() != 8 {
                    return 0;
                }
            }
        }
    }
    if !e.call(TILE_IS_TRUE, &args![tile, 0xfa3u32]).bool() {
        return 0;
    }
    let children = tile + 4;
    let first = e.call(NI_POINTER_GET, &args![children]).u32();
    e.with_stack(8, |e, cells| {
        let cursor = cells.addr();
        let child_best = cells.addr() + 4;
        e.mem.set_u32(cursor, first);
        while e.mem.u32(cursor) != 0 {
            let element = e.call(LIST_NEXT_ELEMENT, &args![children, cursor]).u32();
            let child = e.mem.u32(element);
            e.mem.set_i32(child_best, i32::MIN);
            let candidate = interface_manager_scan_for_max_focus(e, _this, child_best, child);
            let value = e.mem.i32(child_best);
            if candidate != 0 && value > best_value {
                best_value = value;
                found = candidate;
            } else if candidate != 0 && value == best_value {
                let order = e.call(TILE_GET_VALUE_Q, &args![candidate, 0xfacu32]).u32();
                if order != 0 {
                    let order = trait_entry_int(e, order);
                    if order < best_order {
                        best_order = order;
                        found = candidate;
                    }
                }
            }
        }
    });
    if e.call(TILE_IS_TRUE, &args![tile, 0xfafu32]).bool()
        && e.call(TILE_IS_TRUE, &args![tile, 0xfa3u32]).bool()
    {
        let entry = e.call(TILE_GET_TRAIT_ENTRY, &args![tile, 0xfd6u32]).u32();
        if entry != 0 {
            let own = trait_entry_int(e, entry);
            if own > best_value {
                best_value = own;
                found = tile;
            } else if own == best_value {
                let order = e.call(TILE_GET_VALUE_Q, &args![tile, 0xfacu32]).u32();
                if order != 0 {
                    let order = trait_entry_int(e, order);
                    if order < best_order {
                        found = tile;
                    }
                }
            }
        }
    }
    e.mem.set_i32(best, best_value);
    found
}

// Translated from 00716320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::UpdateAllTimers` (Xbox PDB): adds the frame time to
/// every timer of the list (divided by the rate of `00716440` while the
/// object at `011f2250` reports state 4) and removes (and frees) each timer
/// whose elapsed time has reached its end (`clamp(elapsed / end, 0, 1)` is 1).
pub fn interface_manager_update_all_timers(e: &mut Engine, this: Ptr<InterfaceManager>) {
    let head = e.get(this, InterfaceManager::pTimers);
    let mut timer = e.mem.u32(head + 0x10);
    while timer != 0 {
        let frame_time = |e: &mut Engine| e.call(FRAME_TIME_GETTER, &args![FADE_CLOCK]).f64();
        let elapsed = e.mem.f32(timer + 4) as f64;
        let state = e
            .call(WEAPON_STATE_GETTER, &args![WEAPON_STATE_OBJECT])
            .i32();
        let elapsed = if state == 4 {
            let time = frame_time(e);
            let rate = fn_00716440(e) as f64;
            time / rate + elapsed
        } else {
            frame_time(e) + elapsed
        };
        e.mem.set_f32(timer + 4, elapsed as f32);
        let ratio = (e.mem.f32(timer + 4) as f64 / e.mem.f32(timer + 8) as f64) as f32;
        let least = e.call(FLOAT_MIN, &args![1.0f32, ratio]).f32();
        let progress = e.call(FLOAT_MAX, &args![0.0f32, least]).f64();
        if progress == e.global::<f64>(ONE_DOUBLE) {
            let next = e.mem.u32(timer + 0x10);
            let previous = e.mem.u32(timer + 0xc);
            e.mem.set_u32(previous + 0x10, next);
            if next != 0 {
                e.mem.set_u32(next + 0xc, previous);
            }
            if next == 0 {
                let manager = e.call(GET_MANAGER, &args![]).u32();
                let list = e.mem.u32(manager + 0x164);
                e.mem.set_u32(list + 0xc, previous);
            }
            e.call(OPERATOR_DELETE, &args![timer]);
            timer = next;
        } else {
            timer = e.mem.u32(timer + 0x10);
        }
    }
}

// Translated from 00716440 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at `011ac3a0` (`ST0`): the rate the timers are divided by in
/// state 4.
pub fn fn_00716440(e: &mut Engine) -> f32 {
    e.global(TIMER_RATE)
}

// Translated from 00716450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees every timer of the list and its head, and clears `pTimers`.
pub fn fn_00716450(e: &mut Engine, this: Ptr<InterfaceManager>) {
    let head = e.get(this, InterfaceManager::pTimers);
    let mut timer = e.mem.u32(head + 0x10);
    while timer != 0 {
        let next = e.mem.u32(timer + 0x10);
        e.call(OPERATOR_DELETE, &args![timer]);
        timer = next;
    }
    let head = e.get(this, InterfaceManager::pTimers);
    e.call(OPERATOR_DELETE, &args![head]);
    e.set(this, InterfaceManager::pTimers, 0);
}

// Translated from 007164c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::NewTimer` (Xbox PDB, static): replaces the timer with
/// the index `index` by a new one (`ClearTimer`, then a new node from
/// [`timer_construct`]) whose end is `end`, at least 0, appended at the tail
/// of the list. The exception frame is not translated; the scope guard
/// carries the source line `0x145a`.
pub fn interface_manager_new_timer(e: &mut Engine, index: u32, end: f32) {
    with_scope_guard(e, 0x145a, |e| {
        interface_manager_clear_timer(e, index);
        let timer = construct_new(e, OPERATOR_NEW, 0x14, |e, block| {
            timer_construct(e, block.cast()).cast()
        });
        // The compiled code stores through the result without a null check.
        e.set(timer.cast::<Timer>(), Timer::pIndex, index);
        let least = e.call(FLOAT_MAX, &args![end, 0.0f32]).f32();
        e.set(timer.cast::<Timer>(), Timer::fEnd, least);
        let manager = e.call(GET_MANAGER, &args![]).u32();
        let list = e.mem.u32(manager + 0x164);
        let tail = e.mem.u32(list + 0xc);
        e.set(timer.cast::<Timer>(), Timer::pPrev, tail);
        let manager = e.call(GET_MANAGER, &args![]).u32();
        let list = e.mem.u32(manager + 0x164);
        let tail = e.mem.u32(list + 0xc);
        e.mem.set_u32(tail + 0x10, timer.addr());
        let manager = e.call(GET_MANAGER, &args![]).u32();
        let list = e.mem.u32(manager + 0x164);
        e.mem.set_u32(list + 0xc, timer.addr());
    });
}

// Translated from 007165d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::ClearTimer` (Xbox PDB, static): unlinks and frees the
/// first timer with the index `index`, moving the list's tail back when it
/// was the last.
pub fn interface_manager_clear_timer(e: &mut Engine, index: u32) {
    let manager = e.call(GET_MANAGER, &args![]).u32();
    let list = e.mem.u32(manager + 0x164);
    let mut timer = e.mem.u32(list + 0x10);
    while timer != 0 {
        if e.mem.u32(timer) == index {
            let next = e.mem.u32(timer + 0x10);
            let previous = e.mem.u32(timer + 0xc);
            e.mem.set_u32(previous + 0x10, next);
            if next != 0 {
                e.mem.set_u32(next + 0xc, previous);
            }
            if e.mem.u32(timer + 0x10) == 0 {
                let manager = e.call(GET_MANAGER, &args![]).u32();
                let list = e.mem.u32(manager + 0x164);
                e.mem.set_u32(list + 0xc, previous);
            }
            e.call(OPERATOR_DELETE, &args![timer]);
            return;
        }
        timer = e.mem.u32(timer + 0x10);
    }
}

// Translated from 00716660 (decompiled, FalloutNV.exe 1.4.0.525)
/// How far the timer with the index `index` has come (`float` in `ST0`):
/// `clamp(elapsed / end, 0, 1)`; 1 when there is no such timer and `-1` when
/// its end is not positive.
pub fn fn_00716660(e: &mut Engine, index: u32) -> f32 {
    let manager = e.call(GET_MANAGER, &args![]).u32();
    let list = e.mem.u32(manager + 0x164);
    let mut timer = e.mem.u32(list + 0x10);
    while timer != 0 {
        if index == e.mem.u32(timer) {
            let end = e.mem.f32(timer + 8);
            if end as f64 > e.global::<f64>(ZERO_DOUBLE) {
                let ratio = (e.mem.f32(timer + 4) as f64 / end as f64) as f32;
                let least = e.call(FLOAT_MIN, &args![1.0f32, ratio]).f32();
                return e.call(FLOAT_MAX, &args![0.0f32, least]).f32();
            }
            return e.global(CURSOR_DIRECTION_Y);
        }
        timer = e.mem.u32(timer + 0x10);
    }
    1.0
}

// Translated from 007166f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts the key repeat for `key` (`iRepeatingKey`, Xbox PDB): the start
/// time (`uKeyDownTime`) is now, the last repeat time is cleared.
pub fn fn_007166f0(e: &mut Engine, this: Ptr<InterfaceManager>, key: u32) {
    let now = e.call(FADE_CLOCK_READ, &args![FADE_CLOCK]).u32();
    e.set(this, InterfaceManager::uKeyDownTime, now);
    // +0x158: the time of the last repeat (no PDB name).
    e.mem.set_u32(this.addr() + 0x158, 0);
    e.set(this, InterfaceManager::iRepeatingKey, key as i32);
}

// Translated from 00716730 (decompiled, FalloutNV.exe 1.4.0.525)
/// The key repeat: returns the key code to repeat now, or 0. Nothing
/// repeats while the first delay (`011d8b38`) is negative. A repeating
/// arrow code (`0x80000001` .. `0x80000004`) stops, returning 0, once the
/// controls say its key is no longer held. Otherwise the first repeat comes
/// when the time since the key went down reaches the first delay, later
/// ones when the time since the last reaches the second delay (`011d8b88`)
/// divided by `rate`.
pub fn fn_00716730(e: &mut Engine, this: Ptr<InterfaceManager>, rate: f32) -> u32 {
    let now = e.call(FADE_CLOCK_READ, &args![FADE_CLOCK]).u32();
    let first_delay = e
        .call(FLOAT_HOLDER_GET, &args![REPEAT_DELAY_HOLDER_FIRST])
        .u32();
    if e.mem.f32(first_delay) < 0.0 {
        return 0;
    }
    let repeating = e.get(this, InterfaceManager::iRepeatingKey) as u32;
    let mut released = false;
    for (code, key) in [
        (0x8000_0001u32, 0xcbu32),
        (0x8000_0002, 0xcd),
        (0x8000_0003, 0xc8),
        (0x8000_0004, 0xd0),
    ] {
        if repeating == code {
            let controls_owner = e.global::<u32>(CONTROLS_OWNER);
            let controls = e.call(CONTROLS_GET, &args![controls_owner]).u32();
            if e.call(CONTROLS_QUERY_00A24180, &args![controls, key, 0u32])
                .u32()
                == 0
            {
                released = true;
                break;
            }
        }
    }
    if released {
        e.set(this, InterfaceManager::iRepeatingKey, 0);
        return 0;
    }
    let last = e.mem.u32(this.addr() + 0x158);
    if last == 0 {
        let since = now.wrapping_sub(e.get(this, InterfaceManager::uKeyDownTime)) as f64;
        let first_delay = e
            .call(FLOAT_HOLDER_GET, &args![REPEAT_DELAY_HOLDER_FIRST])
            .u32();
        if e.mem.f32(first_delay) as f64 <= since {
            e.mem.set_u32(this.addr() + 0x158, now);
            e.get(this, InterfaceManager::iRepeatingKey) as u32
        } else {
            0
        }
    } else {
        let since = now.wrapping_sub(last) as f64;
        let next_delay = e
            .call(FLOAT_HOLDER_GET, &args![REPEAT_DELAY_HOLDER_NEXT])
            .u32();
        if e.mem.f32(next_delay) as f64 / rate as f64 <= since {
            e.mem.set_u32(this.addr() + 0x158, now);
            e.get(this, InterfaceManager::iRepeatingKey) as u32
        } else {
            0
        }
    }
}

// Translated from 00716910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InterfaceManager::TileIsAcceptingEvents` (Xbox PDB): false when the menu
/// on top of the stack has a type (trait `0xfa7`) of `0x66` or `0x1776` and
/// `tile` belongs to another menu; true otherwise.
pub fn interface_manager_tile_is_accepting_events(
    e: &mut Engine,
    this: Ptr<InterfaceManager>,
    tile: u32,
) -> bool {
    let top = interface_manager_get_enter_stack_top(e, this);
    let top_tile = e.call(TILE_GET_MENU_BY_CLASS, &args![top]).u32();
    if top_tile == 0 {
        return true;
    }
    let value = tile_get_float(e, top_tile, 0xfa7);
    let kind = e.call(FTOL, &args![value]).i32();
    if kind == 0x66 || kind == 0x1776 {
        let own_menu = e.call(TILE_GET_MENU, &args![tile]).u32();
        let top_menu = e.call(TILE_GET_MENU, &args![top_tile]).u32();
        if own_menu != top_menu {
            return false;
        }
    }
    true
}

// Translated from 00716980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the text entry: both strings are constructed, the entry is
/// inactive with the caret at 0, no width limit (`-1`), the blink time 0,
/// the caret phase clear and `field_18` 1. Returns `this`. The exception
/// frame is not translated.
pub fn fn_00716980(e: &mut Engine, this: Ptr<TextEntry>) -> Ptr<TextEntry> {
    e.call(STRING_CONSTRUCT, &args![this]);
    e.call(STRING_CONSTRUCT, &args![this.addr() + 8]);
    e.set(this, TextEntry::active, 0);
    e.set(this, TextEntry::cursor, 0);
    e.set(this, TextEntry::caret_phase, 0);
    e.set(this, TextEntry::max_width, -1);
    e.set(this, TextEntry::last_blink_time, 0);
    e.set(this, TextEntry::field_18, 1);
    this
}

// Translated from 00716a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the text entry: frees the shown string, then the text
/// (`BSStringT` destructor, `004037d0`, on each). `00716a10` is a body the
/// linker shares with unrelated library destructors. The exception frame is
/// not translated.
pub fn fn_00716a10(e: &mut Engine, this: Ptr<TextEntry>) {
    e.call(STRING_DESTROY, &args![this.addr() + 8]);
    e.call(STRING_DESTROY, &args![this]);
}

// Translated from 00716a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the text of the entry: `text` goes to both strings (the shown copy
/// is rebuilt with the caret on the next draw).
pub fn fn_00716a70(e: &mut Engine, this: Ptr<TextEntry>, text: u32) {
    e.call(STRING_SET, &args![this, text]);
    e.call(STRING_SET, &args![this.addr() + 8, text]);
}

// Translated from 00716aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the width limit to `width - 5`; when that is negative, to the
/// interface width (`GetScreenWidth`, converted to an integer).
pub fn fn_00716aa0(e: &mut Engine, this: Ptr<TextEntry>, width: i32) {
    e.set(this, TextEntry::max_width, width.wrapping_sub(5));
    if e.get(this, TextEntry::max_width) < 0 {
        e.call(GET_MANAGER, &args![]);
        let screen_width = interface_manager_get_screen_width(e) as f64;
        let limit = e.call(FTOL, &args![screen_width]).i32();
        e.set(this, TextEntry::max_width, limit);
    }
}

// Translated from 00716ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the entry is active.
pub fn fn_00716ae0(e: &mut Engine, this: Ptr<TextEntry>) -> u8 {
    e.get(this, TextEntry::active)
}

/// Copies the text of the entry into the buffer at `buffer` (the empty
/// string when it has none) and returns its length.
fn text_entry_copy_text(e: &mut Engine, this: Ptr<TextEntry>, buffer: u32) -> i32 {
    if e.call(STRING_LENGTH, &args![this]).u32() == 0 {
        e.call(TEXT_COPY, &args![buffer, EMPTY_TEXT]);
    } else {
        let text = e.call(NI_POINTER_GET, &args![this]).u32();
        e.call(TEXT_COPY, &args![buffer, text]);
    }
    e.call(STRING_LENGTH, &args![this]).i32()
}

/// Empties the text: the buffer gets a terminator at 0, the caret goes to
/// 0 and the text is set from the buffer.
fn text_entry_clear(e: &mut Engine, this: Ptr<TextEntry>, buffer: u32) {
    e.mem.set_u8(buffer, 0);
    e.set(this, TextEntry::cursor, 0);
    e.call(STRING_SET, &args![this, buffer]);
    e.set(this, TextEntry::clears_text_on_edit, 0);
}

// Translated from 00716b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies an editing code to the text of an active entry (the codes
/// `fn_007154b0` makes). Backspace and delete remove the character before or
/// at the caret, or empty the text when `clears_text_on_edit` is set; left,
/// right, home and end move the caret (and clear that flag); enter empties
/// the text when the flag is set and otherwise deactivates the entry; codes
/// `0x80000009` and `0x8000000a` only clear the flag, and `9` is ignored. Any
/// other code (including `0x80000003` and `0x80000004`) is inserted as its low
/// byte at the caret, if the entry has no width limit or the new text is
/// accepted by `00717230`.
pub fn fn_00716b00(e: &mut Engine, this: Ptr<TextEntry>, code: u32) {
    if e.get(this, TextEntry::active) == 0 {
        return;
    }
    e.with_stack(0x404, |e, buffer| {
        let buffer = buffer.addr();
        let mut length = text_entry_copy_text(e, this, buffer);
        let signed = code as i32;
        if signed == 9 {
            return;
        }
        let selector = code.wrapping_sub(0x8000_0000);
        if signed < 9 && selector <= 10 && selector != 3 && selector != 4 {
            match selector {
                0 | 7 => {
                    if e.get(this, TextEntry::clears_text_on_edit) != 0 {
                        text_entry_clear(e, this, buffer);
                    } else {
                        let cursor = e.get(this, TextEntry::cursor);
                        let backspace = selector == 0;
                        if (backspace && cursor > 0) || (!backspace && cursor < length) {
                            let from = if backspace { cursor - 1 } else { cursor };
                            for at in from..length {
                                let next = e.mem.u8(buffer + at as u32 + 1);
                                e.mem.set_u8(buffer + at as u32, next);
                            }
                            length -= 1;
                            if backspace {
                                e.set(this, TextEntry::cursor, cursor - 1);
                            }
                            e.mem.set_u8(buffer + length as u32, 0);
                            e.call(STRING_SET, &args![this, buffer]);
                        }
                    }
                }
                1 => {
                    let cursor = e.get(this, TextEntry::cursor);
                    if cursor > 0 {
                        e.set(this, TextEntry::cursor, cursor - 1);
                    }
                    e.set(this, TextEntry::clears_text_on_edit, 0);
                }
                2 => {
                    let cursor = e.get(this, TextEntry::cursor);
                    if cursor < length {
                        e.set(this, TextEntry::cursor, cursor + 1);
                    }
                    e.set(this, TextEntry::clears_text_on_edit, 0);
                }
                5 => {
                    e.set(this, TextEntry::cursor, 0);
                    e.set(this, TextEntry::clears_text_on_edit, 0);
                }
                6 => {
                    e.set(this, TextEntry::cursor, length);
                    e.set(this, TextEntry::clears_text_on_edit, 0);
                }
                8 => {
                    if e.get(this, TextEntry::clears_text_on_edit) != 0 {
                        text_entry_clear(e, this, buffer);
                    } else {
                        e.set(this, TextEntry::active, 0);
                    }
                }
                _ => e.set(this, TextEntry::clears_text_on_edit, 0),
            }
            return;
        }
        // Any other code: insert its low byte at the caret.
        if e.get(this, TextEntry::clears_text_on_edit) != 0 {
            length = 0;
            text_entry_clear(e, this, buffer);
        }
        let cursor = e.get(this, TextEntry::cursor);
        let mut at = length;
        while at > cursor {
            let previous = e.mem.u8(buffer + at as u32 - 1);
            e.mem.set_u8(buffer + at as u32, previous);
            at -= 1;
        }
        e.mem.set_u8(buffer + at as u32, code as u8);
        e.mem.set_u8(buffer + length as u32 + 1, 0);
        if e.get(this, TextEntry::max_width) == -1
            || e.call(TEXT_ENTRY_ACCEPTS_TEXT, &args![this, buffer]).bool()
        {
            let cursor = e.get(this, TextEntry::cursor);
            e.set(this, TextEntry::cursor, cursor + 1);
            e.call(STRING_SET, &args![this, buffer]);
        }
    });
}

// Translated from 00717010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Activates or deactivates the entry. Activating an inactive entry puts the
/// caret at the end of the text.
pub fn fn_00717010(e: &mut Engine, this: Ptr<TextEntry>, active: u8) {
    if e.get(this, TextEntry::active) == 0 && active != 0 {
        let length = e.call(STRING_LENGTH, &args![this]).i32();
        e.set(this, TextEntry::cursor, length);
    }
    e.set(this, TextEntry::active, active);
}

// Translated from 00717050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flips the caret phase and remembers the time, when more than 500 ticks
/// have passed since the last flip.
pub fn fn_00717050(e: &mut Engine, this: Ptr<TextEntry>) {
    let now = e.call(FADE_CLOCK_READ, &args![FADE_CLOCK]).u32();
    if now.wrapping_sub(e.get(this, TextEntry::last_blink_time)) > 500 {
        let phase = e.get(this, TextEntry::caret_phase);
        e.set(this, TextEntry::caret_phase, (phase == 0) as u8);
        e.set(this, TextEntry::last_blink_time, now);
    }
}

// Translated from 007170a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007170a0 - the /GS stack cookie check (00ec408c) is not translated
/// Rebuilds the shown copy of the text (the string at `+8`): the text with
/// the caret character (`|` or `0x7f`, by the caret phase) inserted at the
/// caret while the entry is active. An empty result is replaced by the string
/// at `01020770`. The answer is the shown text's pointer (`00559450` on it): the exe ends on
/// that call and keeps its `EAX` (`007e6700` passes it to the tile).
pub fn fn_007170a0(e: &mut Engine, this: Ptr<TextEntry>) -> u32 {
    e.with_stack(0x800, |e, scratch| {
        let source = scratch.addr();
        let shown = scratch.addr() + 0x400;
        let length = e.call(STRING_LENGTH, &args![this]).i32();
        if e.call(STRING_LENGTH, &args![this]).u32() == 0 {
            e.call(TEXT_COPY, &args![source, EMPTY_TEXT]);
        } else {
            let text = e.call(NI_POINTER_GET, &args![this]).u32();
            e.call(TEXT_COPY, &args![source, text]);
        }
        let mut out = 0u32;
        for at in 0..=length {
            if e.get(this, TextEntry::active) != 0 && at == e.get(this, TextEntry::cursor) {
                let caret = if e.get(this, TextEntry::caret_phase) != 0 {
                    0x7c
                } else {
                    0x7f
                };
                e.mem.set_u8(shown + out, caret);
                out += 1;
            }
            let character = e.mem.u8(source + at as u32);
            e.mem.set_u8(shown + out, character);
            out += 1;
        }
        e.mem.set_u8(shown + out, 0);
        let display = this.addr() + 8;
        e.call(STRING_SET, &args![display, shown]);
        if e.call(STRING_LENGTH, &args![display]).u32() == 0 {
            e.call(STRING_SET, &args![display, DISPLAY_FALLBACK_STRING]);
        }
        e.call(NI_POINTER_GET, &args![display]).u32()
    })
}

// @@FUNCS-END@@

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Every callee outside this file: tests replace them all with doubles
    /// that return 0, then give the ones they care about a behaviour.
    const EXTERNAL: &[u32] = &[
        RENDER_STATE_00B98540,
        CLOSE_LOADING_MENU,
        HIDE_MENUS,
        STACK_NOTIFY_OWNER_GET,
        STACK_NOTIFY,
        HUD_FORGET_REFERENCE,
        RELEASE_STATIC_GEOMETRY,
        CLEAR_WORLD_MAP_TEXTURE,
        CLEAN_UP_UNUSED_TEXTURES,
        RENDER_TARGET_WIDTH,
        RENDER_TARGET_HEIGHT,
        POINT_NOT_EQUAL,
        MENU_MODE_BYTE_GETTER,
        TILE_GET_TRAIT_ENTRY,
        TRAIT_ENTRY_VALUE,
        TEXT_COPY,
        STRING_LENGTH,
        STRING_SET,
        TEXT_ENTRY_ACCEPTS_TEXT,
        STRING_CONSTRUCT,
        STRING_DESTROY,
        GET_MENUS_ROOT,
        TILE_UPDATE_CHILDREN,
        TILE_PLAY_TILE_SOUND,
        PLAY_MENU_SOUND,
        FTOL,
        FLOAT_MIN,
        FLOAT_MAX,
        TILE_GET_MENU_BY_CLASS,
        TILE_GET_MENU,
        TILE_IS_VISIBLE,
        TILE_IS_TRUE,
        TILE_GET_VALUE_Q,
        DO_ENTER,
        DO_LEAVE,
        XUI_IS_UP,
        HAS_FLAG,
        CONTROLS_GET,
        CONTROLS_QUERY_00A238A0,
        CONTROLS_QUERY_00A24180,
        MENU_STATE,
        GET_FIELD_AT_4,
        GET_FRONTMOST_MENU,
        MENU_MANAGER_INSTANCE,
        WEAPON_STATE_GETTER,
        FRAME_TIME_GETTER,
        FLOAT_HOLDER_GET,
        GET_MANAGER,
        NI_POINTER_GET,
        LIST_NEXT_ELEMENT,
        MENU_CONSOLE_INSTANCE,
        MENU_CONSOLE_TOGGLE_VISIBLE,
        LOG_WARNING,
        TILE_GET_VALUE,
        FADE_CLOCK_READ,
        CURSOR_NAME,
        GLOBALS_XML_PATH,
        BACKGROUND_FILL_ALPHA_NAME,
        HAS_360_CONTROLLER_NAME,
        DEBUG_TEXT_ROOT_NAME,
        TWO_FIFTY_FIVE,
        TWO,
        WIDESCREEN_RATIO,
        WIDESCREEN_INVERSE_RATIO,
        ONE_TO_ONE_ANGLE,
        CURSOR_DIRECTION_Y,
        TIMER_DEFAULT_END,
        XINPUT_GET_STATE,
        OPERATOR_NEW,
        OPERATOR_DELETE,
        NI_ALLOC,
        SCOPE_GUARD_BEGIN,
        SCOPE_GUARD_END,
        NI_POINTER_CONSTRUCT,
        NI_POINTER_ASSIGN,
        NI_POINTER_GET,
        NI_POINTER_DESTROY,
        MEMBER_CONSTRUCTOR_EMPTY,
        SIMPLE_LIST_CONSTRUCT,
        SIMPLE_LIST_DESTROY,
        NI_POINT3_CONSTRUCT,
        NI_UPDATE_DATA_CONSTRUCT,
        NODE_UPDATE,
        NODE_UPDATE_PROPERTIES,
        NODE_SET_FLAG,
        HUD_EFFECT_MANAGER_CONSTRUCT,
        HUD_EFFECT_MANAGER_DESTROY,
        VATS_EFFECT_MANAGER_CONSTRUCT,
        VATS_EFFECT_MANAGER_DESTROY,
        PIPBOY_MANAGER_CONSTRUCT,
        TILE_SET_INT,
        TILE_SET_FLOAT,
        TILE_SET_STRING,
        TILE_UPDATE_TILE,
        TILE_TEXT_TO_TRAIT,
        TILE_ADD_DIRTY_TILE,
        TILE_UPDATE_ALL,
        TILE_READ_FILE,
        TILE_SET_PARENT,
        TILE_LOAD_TEXT_TABLE,
        TILE_FREE_TRAIT_LIST,
        TILE_DESTRUCT,
        TILE_RELEASE,
        MENU_REMOVE_FROM_XML_ARCHIVE,
        MENU_FREE_XML_ARCHIVE,
        GET_SCREEN_WIDTH,
        GET_SCREEN_HEIGHT,
        GET_INTERFACE_MARGIN_A,
        GET_INTERFACE_MARGIN_B,
        GET_INTERFACE_SCALE,
        CREATE_SCENE_GRAPH,
        SCENE_GRAPH_PARENT_3D,
        SCENE_GRAPH_PARENT,
        MENU_MANAGER_INSTANCE,
        ALPHA_PROPERTY_CONSTRUCT,
        ALPHA_PROPERTY_SET_BLEND,
        ALPHA_PROPERTY_SET_NO_SORTER,
        ALPHA_PROPERTY_SET_SOURCE_MODE,
        ALPHA_PROPERTY_SET_DESTINATION_MODE,
        SHADER_ACCUMULATOR_CONSTRUCT,
        SHADER_ACCUMULATOR_SET_194,
        SHADER_ACCUMULATOR_SET_19C,
        SHADOW_SCENE_NODE_CONSTRUCT,
        SET_SHADOW_SCENE_NODE,
        VIEW_CASTER_CONSTRUCT,
        MENU_ROOT_TILE_CONSTRUCT,
        KEY_TABLE_CONSTRUCT,
        FADE_CLOCK_READ,
        FADE_CLOCK,
        ANGLE_FUNCTION_A,
        ANGLE_FUNCTION_B,
        FONT_MANAGER_INIT,
        SYSTEM_COLOR_MANAGER_GET_INSTANCE,
        NI_NODE_CONSTRUCT,
        NODE_SET_NAME,
        FIXED_STRING_CONSTRUCT,
        FIXED_STRING_DESTROY,
        FOG_PROPERTY_CONSTRUCT,
        TILE_IMAGE_NODE,
        NODE_SET_TRANSLATE,
        GET_FRAME_SCENE_NODE,
        GET_SECOND_SCENE_NODE,
        SET_DEBUG_TEXT_VISIBLE,
        GET_DESKTOP_WIDTH,
        GET_DESKTOP_HEIGHT,
        STRING_LIST_SET_LIMIT,
        LOADING_MENU_CREATE,
        HAS_360_CONTROLLER_GETTER,
        FLOAT_HOLDER_GET,
        STRING_HOLDER_GET,
        HUD_MAIN_MENU_CREATE,
        HUD_MAIN_MENU_SET_MENU_MODE,
        TILE_IMAGE_VTABLE,
        TILE_FLAG_TEST,
        TILE_CONSTRUCT,
        TILE_IMAGE_TYPE,
        TILE_IMAGE_TYPE_NAME,
        DEBUG_TEXT_SHUTDOWN,
        FONT_MANAGER_SHUTDOWN,
        EMPTY_MEMBER_FUNCTION,
        TILE_DELETING,
        MENU_MANAGER_SHUTDOWN,
        FREE_TIMERS,
        LIST_CLEAR,
        LIST_AT_011D8B54,
        STATIC_OBJECT_SHUTDOWN_A,
        MESSAGE_MENU_SHUTDOWN,
        CLEAR_TEMP_MODEL,
        START_MENU_SHUTDOWN,
        STATIC_OBJECT_SHUTDOWN_B,
        POINTER_LIST_REMOVE_ALL,
        SCRIPT_POINTER_LIST,
        GLOBAL_OBJECT_8CE8,
        OBJECT_8CE8_DESTRUCT,
        GLOBAL_OBJECT_A0C4,
        VIEW_CASTER_DESTRUCT,
        FLOAT_HOLDER_BACKGROUND,
        STRING_HOLDER_A,
        STRING_HOLDER_B,
        STRING_LIST_A,
        STRING_LIST_B,
        SET_FOG_RANGE,
        CONTROLS_OWNER,
        CONTROLS_GET,
        CLEAR_KEYSTROKES,
        HUD_SET_INFO_FOR_REF,
        MENU_CONSOLE_INSTANCE,
        MENU_CONSOLE_ON_ENTER_MENU_MODE,
        GET_MANAGER,
        AUDIO_INSTANCE,
        AUDIO_PAUSE_TYPE,
        AUDIO_PAUSE_TYPE_6,
        AUDIO_UNPAUSE_TYPE_6,
        AUDIO_UNPAUSE_TYPE_4000_0000,
        MENU_MODE_IS_NOT_ONE,
        UPDATE_ALL_TIMERS,
        PLAYER_SINGLETON,
        LAST_MENU_MODE,
        CONTROLLER_STATE,
        LAST_HAS_CONTROLLER,
        LAST_HAS_CONTROLLER_GUARD,
        MESSAGE_TITLE_HOLDER,
        TEXT_HOLDER_CONTROLLER_WAS_SET,
        TEXT_HOLDER_CONTROLLER_WAS_CLEAR,
        SHOW_MESSAGE_BOX,
        CONTROLLER_CHANGED,
        PRELOAD_MAIN_MENUS,
        CONTROLS_QUERY_00A239E0,
        CONTROLS_QUERY_00A23A50,
        CONTROLS_QUERY_00A24660,
        CONTROLS_QUERY_00A24180,
        CONTROLS_QUERY_00A238A0,
        CONTROLS_AXIS_00A23390,
        CONTROLS_NEXT_EVENT,
        CONTROLS_CLEAR_USER_ACTIONS,
        GET_PIPBOY,
        PIPBOY_UPDATE_LIGHT_EFFECT,
        HUD_EFFECTS_UPDATE,
        VATS_EFFECTS_UPDATE,
        GAME_STATE_ID,
        CONDITION_CHECK_005A03F0,
        PLAYER_FLAG_00950090,
        ACTOR_GET_IRON_SIGHTS,
        WEAPON_STATE_GETTER,
        FORM_GETTER_0044DDC0,
        WEAPON_STATE_OBJECT,
        HUD_SET_MENU_MODE,
        GET_ENTER_STACK_TOP,
        QUERY_00719AE0,
        ACTION_00718930,
        IS_TOP_MENU_ID,
        IS_TOP_MENU_FADED_IN,
        IS_PIPBOY_MENU_TOPMOST,
        OBJECT_FLAG_0070ED80,
        PIPBOY_MENU_CLASS,
        TILE_GET_MENU_BY_CLASS,
        IS_CONSOLE_VISIBLE,
        IS_IN_GAME_LOADING_MENU_OPEN,
        XUI_IS_UP,
        GET_MOUSE_OVER_TARGET,
        TILE_IS_VISIBLE,
        TILE_IS_TRUE,
        GET_DEFAULT_FOCUS,
        CONSOLE_PRIMARY_DOWN,
        FRAME_TIME_GETTER,
        GET_SCREEN_SCALE,
        NI_POINT2_CONSTRUCT,
        GET_MENU_HEIGHT,
        PIPBOY_MARGIN,
        TILE_GET_VALUE,
        TILE_GET_VALUE_Q,
        TILE_GET_MENU,
        PICK_TILE,
        DO_LEAVE,
        DO_ENTER,
        DO_WHEEL_MOVE,
        SET_CURSOR_OFFSET_X,
        SET_CURSOR_OFFSET_Y,
        GET_CURSOR_OFFSET_X,
        GET_CURSOR_OFFSET_Y,
        TILE_GET_POSITION_X,
        TILE_GET_POSITION_Y,
        TILE_PLAY_TILE_SOUND,
        TILE_CHILD_LIST_FIND,
        CLEAR_MOUSE_OVER_TARGET,
        TILE_PARENT,
        GET_WHEEL_STEPS,
        PICK_LIST_COUNT,
        MENU_STATE,
        FTOL,
        RENDERED_MENU_OR_PIPBOY,
        MENU_FLAG_QUERY_004A4040,
        SOUND_HANDLE_CONSTRUCT,
        SOUND_HANDLE_ASSIGN,
        AUDIO_GET_SOUND_HANDLE_BY_NAME,
        UI_MENU_MODE_SOUND,
        SOUND_HANDLE_PLAY,
        SOUND_HANDLE_RELEASE,
        SET_STATS_MENU_VISIBLE,
        SET_INVENTORY_MENU_VISIBLE,
        SET_MAP_MENU_VISIBLE,
        MENU_CONSOLE_IDLE,
        KEY_REPEAT_RESET,
        TRANSLATE_KEY_EVENT,
        KEY_REPEAT_00716730,
        MODIFIER_FOUR_DOWN,
        GET_FRONTMOST_MENU,
        FORMAT_STRING,
        FORMAT_PC_BUTTON,
        PC_BUTTON_PREFIX,
        GET_FIELD_AT_4,
        TILE_GET_STRING,
        TILE_GET_CHILD_BY_NAME,
        PLAY_MENU_SOUND,
        SHOW_ICON_MESSAGE,
        MESSAGE_HOLDER_19,
        MESSAGE_HOLDER_1A,
        MESSAGE_ICON_SIZE,
        MESSAGE_ICON_PATH,
        CONSOLE_KEY_FLAG_GET,
        CONSOLE_KEY_FLAG_SET,
        MENU_CONSOLE_TOGGLE_VISIBLE,
        ADD_TO_ENTER_STACK,
        POP_FROM_ENTER_STACK,
        MENU_MANAGER_IS_MENU_OPEN,
        START_MENU_ALLOWED,
        START_MENU_CLOSE,
        START_MENU_CREATE,
        START_MENU_SETTING,
        QUEUE_PENDING_00070EC00,
        QUEUE_MENU_CREATE,
        DEBUG_TEXT_UPDATE,
        MENU_LOOP_ABORT,
        LIST_NEXT_ELEMENT,
        CHECK_MENU_BUTTON,
        CLEAR_MENU_BUTTON,
        TUTORIAL_MANAGER_UPDATE,
        GET_MENUS_ROOT,
        LEVEL_UP_MENU_CREATE,
        CURSOR_STRING_PENDING,
        LEVEL_UP_MENU_PENDING,
        PLAYER_CAST_EAT_DRINK_ITEMS,
        PLAYER_CAST_QUEUED_ENCHANTMENTS,
        CONTAINER_MENU_CLOSE,
        RENDERED_MENU_CLOSE,
        ACTOR_GET_ANIM_ACTION,
        PLAYER_GET_ANIMATION,
        ANIMATION_CLEAR_GROUP,
        PLAYER_FORCE_TEMP_1ST_PERSON,
        ANIMATION_PLAY_GROUP,
        GET_PIPBOY_STATIC,
        PIPBOY_FADE_LIGHT_EFFECT,
        PIPBOY_ACCESS_UP_SOUND,
        IMAGE_SPACE_GET_HIT,
        IMAGE_SPACE_STOP,
        PLAYER_STATE_00962590,
        INTERFACE_SHOW_MENUS,
        ANIMATION_GET_SEQUENCE,
        MAP_MENU_NEEDS_TIDY,
        MAP_MENU_TIDY,
        PLAYER_IS_PIPBOY_ACTIVE,
        ANIMATION_GROUP_ID,
        MAP_MENU_CLEAR_MAP_MEMORY,
        RELOAD_DYNAMIC_IDLE_ON_1ST_PERSON,
        GET_SAVED_ACQUIRE_OBJECT,
        ACTOR_IS_WEAPON_DRAWN,
        WEAPON_TYPE_INDEX,
        WEAPON_TABLE,
        CLEAR_GUN_WOBBLE,
        ANIMATION_NODE,
        SEQUENCE_END_TIME,
        ANIMATION_ZERO_GLOBAL_TRANSFORM,
        ANIM_GROUP_GET_TIME,
        ANIMATION_UPDATE,
        ANIMATION_UPDATE_BIP_ONLY,
        ANIMATION_UPDATE_SCENE_GRAPH_NO_CONTROLLER,
        IDLE_MANAGER_BUSY,
        LOG_WARNING,
        MISSING_DYNAMIC_IDLE_MESSAGE,
        PIPBOY_MANAGER_UPDATE,
        ENTER_RENDERED_MENU,
        PLAYER_QUERY_005737E0,
        PLAYER_QUERY_004EAF60,
        PLAYER_QUERY_0093A740,
        PLAYER_QUERY_005721E0,
        HOT_KEYS_OWNER,
        TILE_IS_ACCEPTING_EVENTS,
        PLAYER_BUSY_FLAG,
        WEAPON_STATE_FLOAT,
        FLAGS_OWNER_8804,
        FLAGS_QUERY_00701450,
        SUB_OBJECT_QUERY_00701740,
        FLAGS_OWNER,
        HAS_FLAG,
        INVENTORY_MENU,
        KEYRING_TRAIT,
        REPEAT_DELAY_HOLDER_FAST,
        REPEAT_DELAY_HOLDER_FIRST,
        DO_GAMEPAD,
        KEY_REPEAT_00705B10,
        LIST_PREVIOUS_ELEMENT,
        SCAN_FOR_MAX_FOCUS,
        SET_CURRENT_FOCUS_TARGET,
        TILE_GET_FIRST_REF_COPY,
        LIST_PUSH_FRONT_005AE3D0,
        LIST_CONTAINS,
        NI_PICK_CONSTRUCT,
        NI_PICK_DESTRUCT,
        NI_PICK_SET_FLAG,
        GET_PICK_ROOT,
        PICK_ROOT_OWNER,
        NI_PICK_SET_ROOT,
        NI_PICK_SET_MODE,
        GET_SCENE_GRAPH,
        SCENE_GRAPH_SET_CAMERA_FOV,
        SCENE_GRAPH_GET_CAMERA,
        CAMERA_BUILD_PICK_RAY,
        NI_PICK_PICK_OBJECTS,
        NI_PICK_GET_RESULTS,
        RESULTS_COUNT,
        RESULTS_GET,
        RESULT_OBJECT,
        FIND_REFERENCE_FOR_3D,
        LIST_ADD,
        GET_SETTING_VALUE,
        DEBUG_TEXT_TOP_SETTING,
        FONT_LEVEL_SETTING,
        FONT_MANAGER_GET,
        FONT_MANAGER_GET_FONT,
        FONT_HEIGHT,
        SET_PICK_REF_DISPLAY,
        FORM_ID_GETTER,
        REFR_NAME,
        FORMAT_INTO_STRING,
        FORMAT_NAME_AND_ID,
        DEBUG_TEXT_X,
        DEBUG_TEXT_INSTANCE,
        DEBUG_TEXT_PRINT,
        EMPTY_TEXT,
        DEBUG_TEXT_BOTTOM,
        STRING_CONSTRUCT,
        STRING_DESTROY,
        REFR_EXTRA_DATA_LIST,
        EXTRA_GET_SCRIPT,
        SCRIPT_NAME_FORMAT,
        FURNITURE_FORMAT,
        REFR_GET_OWNER,
        OWNER_FORMAT,
        EXTRA_GET_COUNT,
        COUNT_FORMAT,
        DYNAMIC_CAST,
        FROM_TYPE,
        ACTOR_TYPE,
        ACTOR_PACKAGE_FORMAT,
        ACTOR_FURNITURE_FORMAT,
        REFR_IS_FURNITURE,
        REFR_GET_ROTATION,
        RADIANS_TO_DEGREES,
        HEADING_FORMAT,
        MARKER_CONSTRUCT,
        REFR_GET_MARKER_AT_INDEX,
        REFR_GET_FURNITURE_DATA,
        MARKER_KIND,
        FURNITURE_GET_MARKER_ENABLED,
        FURNITURE_IS_SIT_MARKER,
        SIT_TEXT,
        SLEEP_TEXT,
        DISABLED_MARKER_FORMAT,
        REFR_GET_SCALE,
        FURNITURE_GET_MARKER_TARGET_OFFSET,
        REFR_GET_MARKER_USED,
        USED_TEXT,
        UNUSED_TEXT,
        FURNITURE_GET_MARKER_ANGLE,
        MARKER_FORMAT,
        REFR_GET_LOCK,
        LOCK_IS_LOCKED,
        LOCKED_TEXT,
        UNLOCKED_TEXT,
        LOCK_GET_LEVEL,
        LOCK_LEVEL_TABLE,
        LOCK_FORMAT,
        MAP_MARKER_GET_LOCATION_NAME,
        KEY_FORMAT,
        REFR_GET_TELEPORT_DATA,
        DOOR_TELEPORT_GET_CELL,
        UNKNOWN_TEXT,
        PERSISTENT_TEXT,
        EXTRA_IS_PERSISTENT,
        TELEPORT_FORMAT,
        REFR_FLAGS,
        STRING_COPY,
        FLAGS_PREFIX,
        STRING_APPEND,
        FLAG_TEXT_1,
        FLAG_TEXT_2,
        FLAG_TEXT_4,
        FLAG_TEXT_8,
        TRIM_TEXT,
        CONTROLS_FLAG_00711E00,
        STICK_SCALE_HOLDER,
        MOUSE_SETTING_00711DA0,
        MOUSE_SETTING_00711D80,
        MOUSE_LEVEL_00711D60,
        MOUSE_SETTING_00711DC0,
        MOUSE_SETTING_00711DE0,
        NODE_TRANSLATION,
        EDGE_RIGHT,
        EDGE_LEFT,
        EDGE_TOP,
        EDGE_BOTTOM,
        CURSOR_TILT_SCALE,
        NODE_SET_TRANSLATE_VECTOR,
        FLOAT_MIN,
        FLOAT_MAX,
        ABS,
        MOUSE_LEVEL_DIVISOR,
        CURSOR_HORIZONTAL_SCALE,
        CURSOR_VERTICAL_SCALE,
        ONE_DOUBLE,
        TILE_FROM_NODE,
        NODE_PARENT,
        FLOAT_WRAPPER_00408840,
        NODE_GET_PROPERTY,
        PROPERTY_TYPE_THREE,
        PROPERTY_SET_FADE,
        NODE_CHILD_COUNT,
        NODE_CHILD_AT,
        NODE_CHILD_POINTER,
        CURSOR_ROOT_GETTER,
        NI_PICK_SET_BYTE_10,
        SETTING_BYTE,
        RECT_CONSTRUCT,
        IS_IN_PIPBOY_MENU,
        IS_CURRENT_RENDERED_MENU_TOPMOST,
        REAL_SCREEN_WIDTH,
        SCREEN_WIDTH_CONSTANT,
        PROPERTY_RECT,
        NODE_SIBLING_UPDATE,
        PICK_RESULT_HIT_WORD,
        NODE_FLAG_TEST_00456610,
        PASS_THROUGH,
        SCENE_GRAPH_CONSTRUCT,
        RENDERER_GET,
        CAMERA_SET_ROTATION,
        RENDERER_WIDTH,
        RENDERER_HEIGHT,
        FRUSTUM_CONSTRUCT,
        CAMERA_SET_FRUSTUM,
        FADER_ADD_ROOT,
        CULLING_PUSH,
        CULLING_POP,
        RENDERING_SYSTEM,
        RENDERING_WAIT_STAGE,
        RENDERING_SET_STAGE,
        RENDERING_ADD_ACCUM_TASK,
        CULLING_PROCESS_CONSTRUCT,
        CULLING_PROCESS_DESTRUCT,
        CULLING_SET_ACCUMULATOR,
        ACCUMULATE_SCENE,
        ACCUMULATE_FINISH,
        ISOLATE_MENU_ELEMENTS,
        RESTORE_MENU_ELEMENTS,
        MENU_PREDICATE_007079F0,
        RENDERER_SET_CAMERA,
        LOCK_ENTER,
        LOCK_LEAVE,
        COLLECTION_IS_EMPTY,
        DEFERRED_QUEUE_POP,
        ARRAY_FIND,
        ARRAY_ADD,
        ARRAY_FIND_INDEX,
        ARRAY_REMOVE_AT,
        WORD_AT_8,
        WORD_AT_C,
        ARRAY_ELEMENT,
        ARRAY_CLEAR,
        SEMAPHORE_POOL_WAIT,
        TILE_UPDATE_FADE_CONTROLS,
        TILE_UPDATE_CHILDREN,
        TILE_UPDATE_PREPARE,
        CURRENT_THREAD_ID,
        OWNER_THREAD_ID,
        DEVICE_GET,
        RENDERER_STATE_OBJECT,
        RENDER_PASS_RESET,
        RENDER_PASS_SET_MODE,
        RENDER_PASS_ACTIVE,
        RENDER_CLEAR_ENABLED,
        CULLING_SOURCE_GET,
        WORKER_GET,
        WORKER_FLUSH,
        RELEASE_SEMAPHORE,
        WAIT_FOR_SINGLE_OBJECT,
        WAIT_RESULT_HANDLER,
        BEFORE_RELEASE,
        OBJECT_IS_KIND,
        RENDER_STATE_00B97DE0,
        RENDER_STATE_00B97E30,
        RENDER_STATE_00B97E80,
        RENDER_STATE_00B97ED0,
        RENDER_STATE_00B97F20,
        RENDER_STATE_00B97FA0,
        RENDER_STATE_00B97FF0,
        RENDER_STATE_00B980C0,
        RENDER_STATE_00B98180,
        RENDER_STATE_00B98230,
        RENDER_STATE_00B984F0,
        RENDER_STATE_00B98320,
        RENDER_STATE_00B98480,
        RENDER_STATE_004ECED0,
        RENDER_STATE_004ECB40,
        RENDER_STATE_004EB510,
        RENDER_STATE_00714C40,
        GET_SETTING_VALUE,
        TILE_IS_ACCEPTING_EVENTS,
        TILE_GET_VALUE_Q,
        TILE_IS_TRUE,
        TILE_PARENT,
        GET_FIELD_AT_4,
        TILE_GET_MENU,
        GET_MANAGER,
        NI_PICK_DESTRUCT,
    ];

    /// An engine whose callees are all doubles, with the data pages the
    /// code reads mapped (and page 0, so a null object reads as zeros).
    fn world() -> Engine {
        let mut e = Engine::new();
        e.map(0, 0x1000);
        e.map(0x0100_0000, 0x0030_0000);
        for &address in EXTERNAL {
            e.register(address, |_, _| Ret::default());
        }
        // The empty member constructor returns its object.
        echo(&mut e, MEMBER_CONSTRUCTOR_EMPTY);
        e.call_log = Some(vec![]);
        e
    }

    fn returns(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    fn returns_st0(e: &mut Engine, address: u32, value: f64) {
        e.register_double(address, move |_, _| Ret {
            st0: value,
            ..Ret::default()
        });
    }

    /// A constructor-like double: returns its first argument.
    fn echo(e: &mut Engine, address: u32) {
        e.register(address, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
    }

    /// A double that allocates the number of bytes in its first argument.
    fn allocator(e: &mut Engine, address: u32) {
        e.register(address, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
    }

    /// A double that records its argument words and returns `value`.
    fn recorder(e: &mut Engine, address: u32, value: u32) -> Log {
        let log: Log = Rc::new(RefCell::new(vec![]));
        let seen = log.clone();
        e.register_double(address, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
        log
    }

    /// The argument words of every logged call to `address`.
    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// The addresses of the logged calls, in order, restricted to `only`.
    fn order(e: &Engine, only: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| only.contains(a))
            .collect()
    }

    fn manager(e: &mut Engine) -> Ptr<InterfaceManager> {
        let object = e.new_object::<InterfaceManager>();
        e.set_global(MANAGER_SINGLETON, object.addr());
        object
    }

    /// An object whose virtual table has `targets` at the given byte
    /// offsets; each target is a recorder returning `value`.
    fn virtual_object(e: &mut Engine, targets: &[(u32, u32)], value: u32) -> (u32, Vec<Log>) {
        let table = e.mem.alloc(0x600);
        let object = e.mem.alloc(0x1000);
        e.mem.set_u32(object, table);
        let mut logs = vec![];
        for (offset, target) in targets {
            e.mem.set_u32(table + offset, *target);
            logs.push(recorder(e, *target, value));
        }
        (object, logs)
    }

    /// Sets the `float` result of a getter double that returns through
    /// memory (a pointer to the float).
    fn float_cell(e: &mut Engine, value: f32) -> u32 {
        let cell = e.mem.alloc(4);
        e.mem.set_f32(cell, value);
        cell
    }

    #[test]
    fn initialize_polls_the_controller_and_clears_the_pause_counters() {
        let mut e = world();
        let m = manager(&mut e);
        e.set(m, InterfaceManager::iSoundPauses, 3);
        e.set(m, InterfaceManager::iVoicePauses, 2);
        returns(&mut e, XINPUT_GET_STATE, 0);
        interface_manager_initialize(&mut e, 3, 0);
        assert_eq!(e.global::<u8>(HAS_360_CONTROLLER), 1);
        assert_eq!(e.get(m, InterfaceManager::iSoundPauses), 0);
        assert_eq!(e.get(m, InterfaceManager::iVoicePauses), 0);
        returns(&mut e, XINPUT_GET_STATE, 0x48f);
        interface_manager_initialize(&mut e, 3, 0);
        assert_eq!(e.global::<u8>(HAS_360_CONTROLLER), 0);
        assert_eq!(calls(&e, XINPUT_GET_STATE)[0][0], 0);
    }

    #[test]
    fn initialize_builds_the_manager_when_there_is_none() {
        let mut e = world();
        allocator(&mut e, OPERATOR_NEW);
        allocator(&mut e, NI_ALLOC);
        echo(&mut e, NI_POINT3_CONSTRUCT);
        interface_manager_initialize(&mut e, 3, 0);
        let m = e.global::<u32>(MANAGER_SINGLETON);
        assert_ne!(m, 0);
        assert_eq!(calls(&e, OPERATOR_NEW)[0], vec![0x580]);
        assert_eq!(e.mem.u32(m + 0xc), 1, "constructed: cMenuMode is 1");
    }

    #[test]
    fn destroying_the_manager_deletes_it_and_clears_the_singleton() {
        let mut e = world();
        let m = manager(&mut e);
        fn_0070a0b0(&mut e);
        assert_eq!(e.global::<u32>(MANAGER_SINGLETON), 0);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![m.addr()]]);
        fn_0070a0b0(&mut e);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 1, "no manager: nothing");
    }

    #[test]
    fn the_deleting_destructor_frees_only_when_asked() {
        let mut e = world();
        let m = manager(&mut e);
        assert_eq!(
            interface_manager_scalar_deleting_destructor(&mut e, m, 0),
            m
        );
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        interface_manager_scalar_deleting_destructor(&mut e, m, 1);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![m.addr()]]);
    }

    #[test]
    fn the_constructor_sets_the_defaults_and_creates_the_scene_graphs() {
        let mut e = world();
        allocator(&mut e, OPERATOR_NEW);
        allocator(&mut e, NI_ALLOC);
        echo(&mut e, NI_POINT3_CONSTRUCT);
        echo(&mut e, ALPHA_PROPERTY_CONSTRUCT);
        echo(&mut e, VIEW_CASTER_CONSTRUCT);
        echo(&mut e, SHADER_ACCUMULATOR_CONSTRUCT);
        returns(&mut e, CREATE_SCENE_GRAPH, 0x5000);
        returns(&mut e, SCENE_GRAPH_PARENT_3D, 0x6001);
        returns(&mut e, SCENE_GRAPH_PARENT, 0x6002);
        returns_st0(&mut e, GET_SCREEN_WIDTH, 1280.0);
        returns_st0(&mut e, ANGLE_FUNCTION_A, 2.0);
        returns_st0(&mut e, ANGLE_FUNCTION_B, 3.0);
        e.mem.set_f64(TWO, 2.0);
        e.mem.set_f32(TIMER_DEFAULT_END, 0.001);
        e.mem.set_u32(FADE_CLOCK + 0x14, 77);
        returns(&mut e, FADE_CLOCK_READ, 77);
        let m = Ptr::<InterfaceManager>::new(e.mem.alloc(0x580));
        assert_eq!(interface_manager_construct(&mut e, m), m);
        assert_eq!(e.get(m, InterfaceManager::cMenuMode), 1);
        assert_eq!(e.get(m, InterfaceManager::iPickDistance), 0x50);
        assert_eq!(e.get(m, InterfaceManager::cStatsPageNumber), 0xff);
        assert_eq!(e.get(m, InterfaceManager::iLastXDefault), 100);
        assert_eq!(e.get(m, InterfaceManager::bPreLoadMainMenus), 1);
        assert_eq!(e.get(m, InterfaceManager::bMouseInMotion), 1);
        assert_eq!(e.get(m, InterfaceManager::field_4d0), 1.0);
        // 640 / 2.0, divided by the first helper, times the second.
        assert_eq!(e.get(m, InterfaceManager::fOneToOneDistance), 960.0);
        assert_eq!(e.global::<u32>(LAST_DPS_FADE_TIME), 77);
        // Two scene graphs, the 3D one first.
        let graphs = calls(&e, CREATE_SCENE_GRAPH);
        assert_eq!(graphs[0], vec![m.addr(), 0x6001, MENU_3D_ROOT_NAME, 1]);
        assert_eq!(graphs[1], vec![m.addr(), 0x6002, MENU_ROOT_NAME, 0]);
        assert_eq!(calls(&e, NI_POINTER_ASSIGN).len(), 6);
        // The timer list head links to itself.
        let timers = e.get(m, InterfaceManager::pTimers);
        assert_ne!(timers, 0);
        assert_eq!(e.mem.u32(timers + 0xc), timers);
        assert_ne!(e.get(m, InterfaceManager::pViewCaster), 0);
        // The accumulators are numbered 100 and 99.
        let accumulators = calls(&e, SHADER_ACCUMULATOR_CONSTRUCT);
        assert_eq!(accumulators[0][1..], [100, 1, 0x2f7]);
        assert_eq!(accumulators[1][1..], [99, 1, 0x2f7]);
        assert_eq!(
            calls(&e, SHADER_ACCUMULATOR_SET_19C)[0][1],
            10,
            "the interface accumulator gets 10"
        );
    }

    #[test]
    fn a_timer_starts_idle_with_the_default_end() {
        let mut e = world();
        e.mem.set_f32(TIMER_DEFAULT_END, 0.5);
        let t = e.new_object::<Timer>();
        e.set(t, Timer::pPrev, 9);
        assert_eq!(timer_construct(&mut e, t), t);
        assert_eq!(e.get(t, Timer::pPrev), 0);
        assert_eq!(e.get(t, Timer::pNext), 0);
        assert_eq!(e.get(t, Timer::fElapsed), 0.0);
        assert_eq!(e.get(t, Timer::fEnd), 0.5);
        assert_eq!(e.get(t, Timer::pIndex), 0);
    }

    #[test]
    fn the_key_table_constructor_calls_the_initializer() {
        let mut e = world();
        let p = Ptr::new(0x2000_0100);
        assert_eq!(fn_0070a8e0(&mut e, p), p);
        assert_eq!(calls(&e, KEY_TABLE_CONSTRUCT), vec![vec![0x2000_0100]]);
    }

    #[test]
    fn the_accumulator_byte_is_stored() {
        let mut e = world();
        let p = Ptr::<()>::new(e.mem.alloc(0x40));
        fn_0070a900(&mut e, p, 1);
        assert_eq!(e.mem.u8(p.addr() + 0x32), 1);
        fn_0070a900(&mut e, p, 0);
        assert_eq!(e.mem.u8(p.addr() + 0x32), 0);
    }

    #[test]
    fn the_destructor_deletes_what_the_manager_owns_in_order() {
        let mut e = world();
        let m = manager(&mut e);
        // Offsets of the owned objects and the order the code deletes them.
        let fields = [0xb4u32, 0xa0, 0x9c, 0x28, 0x174, 0x94, 0x98];
        let mut owned = vec![];
        for (n, field) in fields.iter().enumerate() {
            let (object, mut logs) = virtual_object(&mut e, &[(0, 0x0900_0000 + n as u32)], 0);
            e.mem.set_u32(m.addr() + field, object);
            owned.push((object, logs.remove(0)));
        }
        let view_caster = e.mem.alloc(0xc);
        e.set(m, InterfaceManager::pViewCaster, view_caster);
        let held = m.addr() + 4;
        e.register_double(NI_POINTER_GET, move |_, a| Ret {
            eax: if a[0] == held { 0x4444 } else { 0 },
            ..Ret::default()
        });
        interface_manager_destruct(&mut e, m);
        for (object, log) in &owned {
            assert_eq!(log.borrow().as_slice(), &[vec![*object, 1]]);
        }
        assert_eq!(e.global::<u32>(MANAGER_SINGLETON), 0);
        assert_eq!(e.get(m, InterfaceManager::pTargetReticle), 0);
        assert_eq!(calls(&e, VIEW_CASTER_DESTRUCT), vec![vec![view_caster]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![view_caster]]);
        // The scene graph that is set is detached and released.
        assert_eq!(calls(&e, EMPTY_MEMBER_FUNCTION)[0], vec![0x4444]);
        assert!(calls(&e, NI_POINTER_ASSIGN).contains(&vec![held, 0]));
        assert_eq!(e.global::<u8>(TILE_DELETING), 0);
        let sequence = order(
            &e,
            &[
                TILE_FREE_TRAIT_LIST,
                MENU_FREE_XML_ARCHIVE,
                CLEAR_TEMP_MODEL,
                START_MENU_SHUTDOWN,
                STATIC_OBJECT_SHUTDOWN_B,
            ],
        );
        assert_eq!(
            sequence,
            vec![
                TILE_FREE_TRAIT_LIST,
                MENU_FREE_XML_ARCHIVE,
                CLEAR_TEMP_MODEL,
                START_MENU_SHUTDOWN,
                STATIC_OBJECT_SHUTDOWN_B
            ]
        );
        // The embedded members are destroyed last, in reverse order.
        let tail: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .rev()
            .take(2)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(tail, vec![vec![m.addr() + 4], vec![m.addr() + 8]]);
    }

    #[test]
    fn the_script_pointer_list_is_emptied() {
        let mut e = world();
        fn_0070aca0(&mut e);
        assert_eq!(
            calls(&e, POINTER_LIST_REMOVE_ALL),
            vec![vec![SCRIPT_POINTER_LIST]]
        );
    }

    #[test]
    fn the_first_global_object_is_deleted_and_cleared() {
        let mut e = world();
        e.set_global(GLOBAL_OBJECT_8CE8, 0x2000_0100u32);
        fn_0070acb0(&mut e);
        assert_eq!(calls(&e, OBJECT_8CE8_DESTRUCT), vec![vec![0x2000_0100]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x2000_0100]]);
        assert_eq!(e.global::<u32>(GLOBAL_OBJECT_8CE8), 0);
        fn_0070acb0(&mut e);
        assert_eq!(calls(&e, OBJECT_8CE8_DESTRUCT).len(), 1);
    }

    #[test]
    fn the_first_deleting_destructor_frees_on_request() {
        let mut e = world();
        let p = Ptr::new(0x2000_0200);
        assert_eq!(fn_0070ad00(&mut e, p, 0), p);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        fn_0070ad00(&mut e, p, 1);
        assert_eq!(calls(&e, OBJECT_8CE8_DESTRUCT).len(), 2);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x2000_0200]]);
    }

    #[test]
    fn the_second_global_object_gets_its_virtual_destructor() {
        let mut e = world();
        let (object, mut logs) = virtual_object(&mut e, &[(0, 0x0900_0001)], 0);
        e.set_global(GLOBAL_OBJECT_A0C4, object);
        fn_0070ad30(&mut e);
        assert_eq!(logs.remove(0).borrow().as_slice(), &[vec![object, 1]]);
        assert_eq!(e.global::<u32>(GLOBAL_OBJECT_A0C4), 0);
        fn_0070ad30(&mut e);
        assert_eq!(calls(&e, 0x0900_0001).len(), 1);
    }

    #[test]
    fn the_view_caster_deleting_destructor_frees_on_request() {
        let mut e = world();
        let p = Ptr::new(0x2000_0300);
        assert_eq!(fn_0070ad80(&mut e, p, 0), p);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        fn_0070ad80(&mut e, p, 3);
        assert_eq!(calls(&e, VIEW_CASTER_DESTRUCT).len(), 2);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x2000_0300]]);
    }

    /// The stubs both `Init` tests need: allocation and constructors that
    /// return their object.
    fn init_world() -> (Engine, Ptr<InterfaceManager>) {
        let mut e = world();
        allocator(&mut e, OPERATOR_NEW);
        allocator(&mut e, NI_ALLOC);
        for constructor in [
            MENU_ROOT_TILE_CONSTRUCT,
            NI_NODE_CONSTRUCT,
            FOG_PROPERTY_CONSTRUCT,
            PIPBOY_MANAGER_CONSTRUCT,
            SHADOW_SCENE_NODE_CONSTRUCT,
            FIXED_STRING_CONSTRUCT,
        ] {
            echo(&mut e, constructor);
        }
        let m = manager(&mut e);
        (e, m)
    }

    #[test]
    fn init_builds_the_root_tile_and_the_debug_text_root() {
        let (mut e, m) = init_world();
        let (interface_root, mut root_logs) = virtual_object(&mut e, &[(0xdc, 0x0900_0010)], 0);
        e.set(m, InterfaceManager::pInterfaceRoot, interface_root);
        // The root tile: slot 4 is `Tile::Init`.
        e.register(MENU_ROOT_TILE_CONSTRUCT, |e, a| {
            let table = e.mem.alloc(0x40);
            e.mem.set_u32(a[0], table);
            e.mem.set_u32(table + 4, 0x0900_0011);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let init_log = recorder(&mut e, 0x0900_0011, 0);
        returns_st0(&mut e, GET_SCREEN_WIDTH, 1920.0);
        returns_st0(&mut e, GET_SCREEN_HEIGHT, 1080.0);
        returns_st0(&mut e, GET_INTERFACE_SCALE, 0.5);
        returns(&mut e, TILE_READ_FILE, 0x7777);
        returns(&mut e, TILE_TEXT_TO_TRAIT, 0x1234);
        returns(&mut e, HAS_360_CONTROLLER_GETTER, 1);
        returns(&mut e, FLOAT_HOLDER_GET, 0);
        let cell = float_cell(&mut e, 0.5);
        returns(&mut e, FLOAT_HOLDER_GET, cell);
        e.mem.set_f64(TWO_FIFTY_FIVE, 255.0);
        returns_st0(&mut e, GET_DESKTOP_WIDTH, 1500.0);
        returns_st0(&mut e, GET_DESKTOP_HEIGHT, 1000.0);
        e.mem.set_f64(WIDESCREEN_RATIO, 1.5);
        e.mem.set_f64(WIDESCREEN_INVERSE_RATIO, 0.6);
        e.mem.set_f64(TWO, 2.0);
        interface_manager_init(&mut e, m, 1);
        let root = e.get(m, InterfaceManager::pMenusRoot);
        assert_ne!(root, 0);
        assert_eq!(
            init_log.borrow().as_slice(),
            &[vec![root, 0, MENU_ROOT_NAME, 0]]
        );
        // The screen width goes to trait 0xfb1 as a float.
        let float_sets = calls(&e, TILE_SET_FLOAT);
        assert!(float_sets.contains(&vec![root, 0xfb1, 1920.0f32.to_bits(), 1]));
        assert!(float_sets.contains(&vec![root, 0xff8, 0.5f32.to_bits(), 1]));
        assert_eq!(calls(&e, TILE_LOAD_TEXT_TABLE).len(), 1);
        // `globals.xml` becomes the string root; its alpha trait is 0.5 * 255.
        assert_eq!(e.get(m, InterfaceManager::pStringRoot), 0x7777);
        assert!(float_sets.contains(&vec![0x7777, 0x1234, 127.5f32.to_bits(), 1]));
        assert_eq!(
            calls(&e, TILE_READ_FILE),
            vec![vec![root, GLOBALS_XML_PATH]]
        );
        // The has-controller trait is set from the getter.
        assert!(calls(&e, TILE_SET_INT).contains(&vec![0x7777, 0x1234, 1]));
        // The debug text root is a node attached to the interface root.
        let debug_root = e.get(m, InterfaceManager::pDebugTextRoot);
        assert_ne!(debug_root, 0);
        assert_eq!(
            root_logs.remove(0).borrow().as_slice(),
            &[vec![interface_root, debug_root, 1]]
        );
        // 1500 / 1000 equals the stored ratio: both string lists are limited.
        let limits = calls(&e, STRING_LIST_SET_LIMIT);
        assert_eq!(
            limits,
            vec![vec![STRING_LIST_A, 0x50], vec![STRING_LIST_B, 0x50]]
        );
        // The pipboy manager and the first-init flag.
        assert_ne!(e.get(m, InterfaceManager::pPipboy), 0);
        assert_eq!(e.get(m, InterfaceManager::bFirstInit), 1);
        assert_eq!(calls(&e, LOADING_MENU_CREATE), vec![vec![0, 0]]);
        // The scope guard is opened with the source line and closed.
        assert_eq!(
            calls(&e, SCOPE_GUARD_BEGIN)[0][1..],
            [0xd, 1, SOURCE_FILE, 0x21a]
        );
        assert_eq!(calls(&e, SCOPE_GUARD_END).len(), 1);
    }

    #[test]
    fn init_without_the_text_table_skips_it_and_the_loading_menu() {
        let (mut e, m) = init_world();
        e.register(MENU_ROOT_TILE_CONSTRUCT, |e, a| {
            let table = e.mem.alloc(0x40);
            e.mem.set_u32(a[0], table);
            e.mem.set_u32(table + 4, 0x0900_0012);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        recorder(&mut e, 0x0900_0012, 0);
        let (interface_root, _) = virtual_object(&mut e, &[(0xdc, 0x0900_0013)], 0);
        e.set(m, InterfaceManager::pInterfaceRoot, interface_root);
        let cell = float_cell(&mut e, 0.0);
        returns(&mut e, FLOAT_HOLDER_GET, cell);
        e.mem.set_f64(WIDESCREEN_RATIO, 1.5);
        e.mem.set_f64(WIDESCREEN_INVERSE_RATIO, 0.6);
        returns_st0(&mut e, GET_DESKTOP_WIDTH, 1280.0);
        returns_st0(&mut e, GET_DESKTOP_HEIGHT, 1024.0);
        interface_manager_init(&mut e, m, 0);
        assert!(calls(&e, TILE_LOAD_TEXT_TABLE).is_empty());
        assert!(calls(&e, LOADING_MENU_CREATE).is_empty());
        assert!(
            calls(&e, STRING_LIST_SET_LIMIT).is_empty(),
            "1.25 is not 1.5"
        );
        assert_eq!(e.get(m, InterfaceManager::bFirstInit), 1);
    }

    #[test]
    fn init_second_creates_the_cursor_the_shadow_nodes_and_the_hud() {
        let (mut e, m) = init_world();
        e.put_vtable(TILE_IMAGE_VTABLE, &[0, 0x0900_0020]);
        let init_log = recorder(&mut e, 0x0900_0020, 0);
        echo(&mut e, TILE_CONSTRUCT);
        let (cursor_root, mut root_logs) = virtual_object(&mut e, &[(0xdc, 0x0900_0021)], 0);
        e.set(m, InterfaceManager::pCursorRoot, cursor_root);
        returns(&mut e, TILE_IMAGE_NODE, 0x5555);
        returns(&mut e, HUD_MAIN_MENU_CREATE, 0x8888);
        returns(&mut e, STRING_HOLDER_GET, 0x6666);
        returns(&mut e, NI_POINTER_GET, 0x4040);
        e.mem.set_f32(CURSOR_DIRECTION_Y, -1.0);
        interface_manager_init_second(&mut e, m);
        let cursor = e.get(m, InterfaceManager::pCursor);
        assert_ne!(cursor, 0);
        assert_eq!(e.mem.u32(cursor), TILE_IMAGE_VTABLE);
        assert_eq!(
            init_log.borrow().as_slice(),
            &[vec![cursor, 0, CURSOR_NAME, 0]]
        );
        // The cursor is 32 by 32 and its node joins the cursor root.
        assert!(calls(&e, TILE_SET_INT).contains(&vec![cursor, 0xfb1, 0x20]));
        assert!(calls(&e, TILE_SET_INT).contains(&vec![cursor, 0xfad, 2000]));
        assert_eq!(
            root_logs.remove(0).borrow().as_slice(),
            &[vec![cursor_root, 0x5555, 1]]
        );
        // The node looks along -y.
        assert_eq!(
            calls(&e, NODE_SET_TRANSLATE),
            vec![vec![
                0x5555,
                0.0f32.to_bits(),
                (-1.0f32).to_bits(),
                0.0f32.to_bits()
            ]]
        );
        // Two shadow scene nodes, in slots 1 and 3.
        let shadow = e.get(m, InterfaceManager::pShadowNode);
        let ui_player = e.get(m, InterfaceManager::pUIPlayerNode);
        assert_ne!(shadow, 0);
        assert_ne!(ui_player, 0);
        assert_eq!(
            calls(&e, SET_SHADOW_SCENE_NODE),
            vec![vec![shadow, 1], vec![ui_player, 3]]
        );
        assert_eq!(
            calls(&e, SHADER_ACCUMULATOR_SET_194),
            vec![vec![0x4040, shadow], vec![0x4040, shadow]]
        );
        // The HUD menu is shown and the manager is initialized.
        assert!(calls(&e, TILE_SET_INT).contains(&vec![0x8888, 0xfa3, 0]));
        assert_eq!(calls(&e, HUD_MAIN_MENU_SET_MENU_MODE), vec![vec![4]]);
        assert_eq!(e.get(m, InterfaceManager::bSecondInit), 1);
        assert_eq!(
            calls(&e, SCOPE_GUARD_BEGIN)[0][1..],
            [0xd, 1, SOURCE_FILE, 0x29b]
        );
    }

    #[test]
    fn a_tile_image_starts_with_its_vtable_a_full_scale_and_no_texture() {
        let mut e = world();
        echo(&mut e, TILE_CONSTRUCT);
        let t = e.new_object::<TileImage>();
        e.set(t, TileImage::bIsScissor, 1);
        assert_eq!(tile_image_construct(&mut e, t), t);
        assert_eq!(e.mem.u32(t.addr()), TILE_IMAGE_VTABLE);
        assert_eq!(e.get(t, TileImage::fScale), 1.0);
        assert_eq!(e.get(t, TileImage::bIsScissor), 0);
        assert_eq!(
            calls(&e, NI_POINTER_CONSTRUCT),
            vec![vec![t.addr() + 0x3c, 0], vec![t.addr() + 0x40, 0]]
        );
        assert_eq!(calls(&e, NI_POINTER_ASSIGN), vec![vec![t.addr() + 0x3c, 0]]);
    }

    #[test]
    fn a_tile_image_knows_its_type() {
        let mut e = world();
        let t = e.new_object::<TileImage>();
        assert_eq!(tile_image_get_type(&mut e, t), 0x386);
        assert_eq!(tile_image_get_type_name(&mut e, t), 0x0106_f044);
    }

    #[test]
    fn the_tile_image_deleting_destructor_frees_on_request() {
        let mut e = world();
        let t = e.new_object::<TileImage>();
        assert_eq!(tile_image_scalar_deleting_destructor(&mut e, t, 0), t);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        tile_image_scalar_deleting_destructor(&mut e, t, 1);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![t.addr()]]);
        assert_eq!(calls(&e, TILE_DESTRUCT).len(), 2);
    }

    #[test]
    fn a_tile_image_is_released_unless_it_already_was() {
        let mut e = world();
        let t = e.new_object::<TileImage>();
        returns(&mut e, TILE_FLAG_TEST, 0);
        tile_image_destruct(&mut e, t);
        assert_eq!(calls(&e, TILE_RELEASE), vec![vec![t.addr()]]);
        assert_eq!(e.mem.u32(t.addr()), TILE_IMAGE_VTABLE);
        assert_eq!(
            order(
                &e,
                &[
                    NI_POINTER_ASSIGN,
                    TILE_RELEASE,
                    NI_POINTER_DESTROY,
                    TILE_DESTRUCT
                ]
            ),
            vec![
                NI_POINTER_ASSIGN,
                TILE_RELEASE,
                NI_POINTER_DESTROY,
                NI_POINTER_DESTROY,
                TILE_DESTRUCT
            ]
        );
        returns(&mut e, TILE_FLAG_TEST, 1);
        tile_image_destruct(&mut e, t);
        assert_eq!(calls(&e, TILE_RELEASE).len(), 1);
    }

    /// The audio stubs the pause code needs.
    fn audio_world() -> Engine {
        let mut e = world();
        let manager_block = e.mem.alloc(0x40);
        returns(&mut e, GET_MANAGER, manager_block);
        returns(&mut e, AUDIO_INSTANCE, 0x3030);
        e
    }

    #[test]
    fn leaving_the_fade_out_enters_mode_five_and_pauses_the_audio() {
        let mut e = audio_world();
        let m = manager(&mut e);
        let sounds = e.call(GET_MANAGER, &args![]).u32();
        e.call_log = Some(vec![]);
        e.set(m, InterfaceManager::cMenuMode, 3);
        e.set(m, InterfaceManager::pCursor, 0x7000);
        returns(&mut e, TILE_IMAGE_NODE, 0x5555);
        returns(&mut e, GET_FIELD_AT_4, 0);
        returns(&mut e, MENU_CONSOLE_INSTANCE, 0x1111);
        returns(&mut e, HAS_360_CONTROLLER_GETTER, 1);
        returns(&mut e, CONTROLS_GET, 0x2222);
        returns(&mut e, STRING_HOLDER_GET, 0x6666);
        e.set(m, InterfaceManager::bShowMouse, 1);
        interface_manager_pre_idle_stuff(&mut e, m);
        assert_eq!(e.get(m, InterfaceManager::cMenuMode), 5);
        assert_eq!(calls(&e, CLEAR_KEYSTROKES).len(), 1);
        // The mouse shows, so the cursor is hidden through its node.
        assert_eq!(calls(&e, NODE_SET_FLAG), vec![vec![0x5555, 0]]);
        assert!(calls(&e, TILE_SET_INT).contains(&vec![0x7000, 0xfa3, 1]));
        assert_eq!(calls(&e, HUD_SET_INFO_FOR_REF), vec![vec![0, 0, 0]]);
        assert_eq!(e.get(m, InterfaceManager::pMouseOverTarget), 0);
        // Console entered, both pause counters up by one.
        assert_eq!(
            calls(&e, MENU_CONSOLE_ON_ENTER_MENU_MODE),
            vec![vec![0x1111]]
        );
        assert_eq!(e.mem.u8(sounds + 0x20), 1);
        assert_eq!(e.mem.u8(sounds + 0x21), 1);
        assert_eq!(
            calls(&e, AUDIO_PAUSE_TYPE),
            vec![vec![0x3030, 0x4000_0000, 1]]
        );
        assert_eq!(calls(&e, AUDIO_PAUSE_TYPE_6), vec![vec![0x3030]]);
    }

    #[test]
    fn mode_five_settles_to_mode_two_and_a_locked_fade_is_left_alone() {
        let mut e = audio_world();
        let m = manager(&mut e);
        e.set(m, InterfaceManager::cMenuMode, 5);
        interface_manager_pre_idle_stuff(&mut e, m);
        assert_eq!(e.get(m, InterfaceManager::cMenuMode), 2);
        e.set(m, InterfaceManager::cMenuMode, 3);
        e.set(m, InterfaceManager::bLockMenuModeForFade, 1);
        interface_manager_pre_idle_stuff(&mut e, m);
        assert_eq!(e.get(m, InterfaceManager::cMenuMode), 3);
        assert!(calls(&e, CLEAR_KEYSTROKES).is_empty());
    }

    #[test]
    fn in_the_game_mode_the_audio_pauses_follow_the_menu_mode_check() {
        let mut e = audio_world();
        let m = manager(&mut e);
        let sounds = e.call(GET_MANAGER, &args![]).u32();
        e.set(m, InterfaceManager::cMenuMode, 1);
        // `007023a0` says the manager is not in mode 1: not paused.
        returns(&mut e, MENU_MODE_IS_NOT_ONE, 1);
        interface_manager_pre_idle_stuff(&mut e, m);
        assert_eq!(e.get(m, InterfaceManager::bExternalForcedMenuMode), 1);
        assert_eq!(e.mem.u8(sounds + 0x20), 1);
        assert_eq!(e.mem.u8(sounds + 0x21), 1);
        // Asked again with the flag already set: nothing more.
        interface_manager_pre_idle_stuff(&mut e, m);
        assert_eq!(e.mem.u8(sounds + 0x20), 1);
        // Now it reports mode 1: the flag clears and the audio resumes.
        returns(&mut e, MENU_MODE_IS_NOT_ONE, 0);
        interface_manager_pre_idle_stuff(&mut e, m);
        assert_eq!(e.get(m, InterfaceManager::bExternalForcedMenuMode), 0);
        assert_eq!(e.mem.u8(sounds + 0x20), 0);
        assert_eq!(e.mem.u8(sounds + 0x21), 0);
        assert_eq!(calls(&e, AUDIO_UNPAUSE_TYPE_6).len(), 1);
        assert_eq!(calls(&e, AUDIO_UNPAUSE_TYPE_4000_0000).len(), 1);
    }

    #[test]
    fn the_audio_pause_wrapper_pauses_type_0x40000000() {
        let mut e = world();
        fn_0070bba0(&mut e, Ptr::new(0x3030));
        assert_eq!(
            calls(&e, AUDIO_PAUSE_TYPE),
            vec![vec![0x3030, 0x4000_0000, 1]]
        );
    }

    /// A world with the two helpers that do real arithmetic: the float to
    /// integer conversion and `abs`, and the min/max helpers.
    fn arithmetic_world() -> Engine {
        let mut e = world();
        e.register(FTOL, |_, a| Ret {
            eax: f64::from_bits(a[0] as u64 | (a[1] as u64) << 32) as i32 as u32,
            ..Ret::default()
        });
        e.register(ABS, |_, a| Ret {
            eax: (a[0] as i32).wrapping_abs() as u32,
            ..Ret::default()
        });
        e.register(FLOAT_MIN, |_, a| {
            let (first, second) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            Ret {
                st0: if second <= first { second } else { first } as f64,
                ..Ret::default()
            }
        });
        e.register(FLOAT_MAX, |_, a| {
            let (first, second) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            Ret {
                st0: if second < first { first } else { second } as f64,
                ..Ret::default()
            }
        });
        e
    }

    /// A double for a controls query: `(this, key, state)` answers `answer`
    /// when `key` matches, else 0.
    fn key_query(e: &mut Engine, address: u32, key: u32, answer: u32) {
        e.register_double(address, move |_, a| Ret {
            eax: if a[1] == key { answer } else { 0 },
            ..Ret::default()
        });
    }

    #[test]
    fn a_binding_slot_is_cleared() {
        let mut e = world();
        let controls = Ptr::<()>::new(e.mem.alloc(0x2000));
        fn_0070ec20(&mut e, controls, 0x1c, 2);
        assert_eq!(e.mem.u8(controls.addr() + 0x1b94 + 2 * 0x1c + 0x1c), 0xff);
        assert_eq!(e.mem.u8(controls.addr() + 0x1b94), 0);
    }

    #[test]
    fn small_getters_read_their_fields() {
        let mut e = world();
        let object = Ptr::<()>::new(e.mem.alloc(0x1000));
        e.mem.set_u8(object.addr() + 0x162, 9);
        assert_eq!(fn_0070ec50(&mut e, object), 9);
        e.mem.set_u8(object.addr() + 0xd54, 4);
        assert_eq!(fn_0070edc0(&mut e, object), 4);
        e.mem.set_f32(object.addr() + 0x670, 1.25);
        assert_eq!(fn_00710ab0(&mut e, object), 1.25);
        let m = manager(&mut e);
        assert!(!fn_0070ecd0(&mut e, m));
        e.set(m, InterfaceManager::iModifierKeys, 0b110);
        assert!(!fn_0070ecd0(&mut e, m));
        e.set(m, InterfaceManager::iModifierKeys, 0b101);
        assert!(fn_0070ecd0(&mut e, m));
    }

    #[test]
    fn the_keyring_is_open_when_the_inventory_menu_tile_says_so() {
        let mut e = world();
        assert!(!inventory_menu_is_keyring_open(&mut e));
        e.set_global(INVENTORY_MENU, 0x2000_0400u32);
        e.set_global(KEYRING_TRAIT, 0xfd5u32);
        returns(&mut e, GET_FIELD_AT_4, 0x7100);
        returns(&mut e, TILE_IS_TRUE, 1);
        assert!(inventory_menu_is_keyring_open(&mut e));
        assert_eq!(calls(&e, GET_FIELD_AT_4), vec![vec![0x2000_0400]]);
        assert_eq!(calls(&e, TILE_IS_TRUE), vec![vec![0x7100, 0xfd5]]);
        returns(&mut e, TILE_IS_TRUE, 0);
        assert!(!inventory_menu_is_keyring_open(&mut e));
    }

    #[test]
    fn two_flag_bits_must_both_be_set() {
        let mut e = world();
        e.set_global(FLAGS_OWNER, 0x2000_0500u32);
        returns(&mut e, HAS_FLAG, 1);
        assert!(fn_0070ee30(&mut e));
        assert_eq!(
            calls(&e, HAS_FLAG),
            vec![vec![0x2000_0500, 0x8000], vec![0x2000_0500, 8]]
        );
        e.register(HAS_FLAG, |_, a| Ret {
            eax: (a[1] == 0x8000) as u32,
            ..Ret::default()
        });
        assert!(!fn_0070ee30(&mut e));
        e.register(HAS_FLAG, |_, _| Ret::default());
        assert!(!fn_0070ee30(&mut e));
    }

    fn top_menu(e: &mut Engine, id: u32) {
        e.register_double(IS_TOP_MENU_ID, move |_, a| Ret {
            eax: (a[0] == id) as u32,
            ..Ret::default()
        });
    }

    #[test]
    fn raising_the_pipboy_closes_the_container_menu_first() {
        let mut e = world();
        let m = manager(&mut e);
        e.set(m, InterfaceManager::field_4bc, 1);
        top_menu(&mut e, 0x3f0);
        update_pipboy(&mut e, m);
        assert_eq!(calls(&e, CONTAINER_MENU_CLOSE).len(), 1);
        assert_eq!(e.get(m, InterfaceManager::field_4bc), 1);
    }

    fn update_pipboy(e: &mut Engine, m: Ptr<InterfaceManager>) {
        interface_manager_update_pipboy(e, m)
    }

    #[test]
    fn raising_the_pipboy_closes_the_rendered_terminal_menus() {
        let mut e = world();
        let m = manager(&mut e);
        e.set(m, InterfaceManager::field_4bc, 1);
        e.set(m, InterfaceManager::pCurrentRenderedMenu, 0x2000_0600);
        top_menu(&mut e, 0x41f);
        update_pipboy(&mut e, m);
        assert_eq!(calls(&e, RENDERED_MENU_CLOSE), vec![vec![0x2000_0600]]);
        assert!(calls(&e, CONTAINER_MENU_CLOSE).is_empty());
    }

    #[test]
    fn the_pipboy_rises_unless_the_player_is_busy() {
        let mut e = audio_world();
        let m = manager(&mut e);
        e.set(m, InterfaceManager::field_4bc, 1);
        let player = e.mem.alloc(0x1000);
        e.set_global(PLAYER_SINGLETON, player);
        returns(&mut e, ACTOR_GET_ANIM_ACTION, 3);
        update_pipboy(&mut e, m);
        assert_eq!(e.get(m, InterfaceManager::field_4bc), 1, "busy: nothing");
        assert!(calls(&e, PLAYER_GET_ANIMATION).is_empty());

        returns(&mut e, ACTOR_GET_ANIM_ACTION, 7);
        returns(&mut e, PLAYER_GET_ANIMATION, 0x2000_0700);
        returns(&mut e, GET_PIPBOY_STATIC, 0x8080);
        update_pipboy(&mut e, m);
        assert_eq!(e.get(m, InterfaceManager::field_4bc), 2);
        let play = calls(&e, ANIMATION_PLAY_GROUP);
        assert_eq!(
            play,
            vec![vec![0x2000_0700, 0xe2, 1, 0xffff_ffff, 0xffff_ffff]]
        );
        assert_eq!(calls(&e, PIPBOY_FADE_LIGHT_EFFECT), vec![vec![0x8080, 1]]);
        assert_eq!(calls(&e, ADD_TO_ENTER_STACK), vec![vec![m.addr(), 1]]);
        assert_eq!(calls(&e, INTERFACE_SHOW_MENUS).len(), 1);
        assert_eq!(calls(&e, IMAGE_SPACE_STOP).len(), 1);
        assert_eq!(calls(&e, ANIMATION_CLEAR_GROUP).len(), 2);
        // The `UIPipBoyAccessUp` sound is looked up and played.
        assert_eq!(
            calls(&e, AUDIO_GET_SOUND_HANDLE_BY_NAME)[0][2..],
            [PIPBOY_ACCESS_UP_SOUND, 0x121]
        );
        assert_eq!(calls(&e, SOUND_HANDLE_PLAY).len(), 1);
    }

    #[test]
    fn the_pipboy_gives_up_waiting_when_the_animation_is_gone() {
        let mut e = world();
        let m = manager(&mut e);
        e.set(m, InterfaceManager::field_4bc, 2);
        e.set_global(PLAYER_SINGLETON, 0x2000_0800u32);
        returns(&mut e, GET_PIPBOY_STATIC, 0x8080);
        returns(&mut e, MAP_MENU_NEEDS_TIDY, 1);
        update_pipboy(&mut e, m);
        assert_eq!(e.get(m, InterfaceManager::field_4bc), 4);
        assert_eq!(calls(&e, PIPBOY_FADE_LIGHT_EFFECT), vec![vec![0x8080, 0]]);
        assert_eq!(calls(&e, MAP_MENU_TIDY), vec![vec![0]]);
    }

    #[test]
    fn the_pipboy_pose_enters_the_rendered_menu() {
        let mut e = audio_world();
        let m = manager(&mut e);
        e.set(m, InterfaceManager::field_4bc, 2);
        let player = e.mem.alloc(0x1000);
        e.set_global(PLAYER_SINGLETON, player);
        // The animation and the word `0070f490(anim, 2)` reads: 1 means
        // the pose was reached.
        let animation = e.mem.alloc(0x100);
        e.mem.set_u32(animation + 0x64, 1);
        returns(&mut e, PLAYER_GET_ANIMATION, animation);
        returns(&mut e, ANIMATION_GET_SEQUENCE, 0x2000_0900);
        returns_st0(&mut e, SEQUENCE_END_TIME, 2.0);
        returns_st0(&mut e, ANIM_GROUP_GET_TIME, 5.0);
        // A pending callback runs and is cleared.
        let callback_log = recorder(&mut e, 0x0900_0030, 0);
        e.set(m, InterfaceManager::field_4c0, 0x0900_0030);
        // The saved acquire object answers "can reload" for the idle.
        let (process, _) = virtual_object(&mut e, &[(0x4d8, 0x0900_0031)], 1);
        returns(&mut e, GET_SAVED_ACQUIRE_OBJECT, process);
        returns(&mut e, GET_PIPBOY, 0x6060);
        interface_manager_update_pipboy(&mut e, m);
        assert_eq!(e.get(m, InterfaceManager::field_4bc), 3);
        assert_eq!(calls(&e, HUD_SET_MENU_MODE), vec![vec![3]]);
        // The update time is the pose time minus the sequence end time.
        let update = calls(&e, ANIMATION_UPDATE);
        assert_eq!(
            update,
            vec![vec![animation, player, 0.0f32.to_bits(), 3.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, ANIMATION_UPDATE_BIP_ONLY),
            vec![vec![animation, 3.0f32.to_bits(), 0, 1]]
        );
        assert_eq!(callback_log.borrow().len(), 1);
        assert_eq!(e.get(m, InterfaceManager::field_4c0), 0);
        assert_eq!(e.global::<u8>(IDLE_MANAGER_BUSY), 0);
        assert_eq!(calls(&e, ANIMATION_CLEAR_GROUP).len(), 1);
        assert_eq!(calls(&e, ENTER_RENDERED_MENU).len(), 1);
        assert_eq!(calls(&e, PIPBOY_MANAGER_UPDATE), vec![vec![0x6060]]);
    }

    #[test]
    fn the_pipboy_open_state_keeps_the_map_tidy() {
        let mut e = world();
        let m = manager(&mut e);
        e.set(m, InterfaceManager::field_4bc, 3);
        e.set_global(PLAYER_SINGLETON, 0x2000_0800u32);
        interface_manager_update_pipboy(&mut e, m);
        assert!(calls(&e, MAP_MENU_TIDY).is_empty());
        returns(&mut e, MAP_MENU_NEEDS_TIDY, 1);
        interface_manager_update_pipboy(&mut e, m);
        assert_eq!(calls(&e, MAP_MENU_TIDY), vec![vec![0]]);
        assert_eq!(e.get(m, InterfaceManager::field_4bc), 3);
    }

    #[test]
    fn lowering_the_pipboy_waits_for_the_enter_stack() {
        let mut e = audio_world();
        let m = manager(&mut e);
        let sounds = e.call(GET_MANAGER, &args![]).u32();
        e.mem.set_u8(sounds + 0x20, 2);
        e.set(m, InterfaceManager::field_4bc, 4);
        let (player, _) = virtual_object(&mut e, &[(0x214, 0x0900_0040), (0x4b0, 0x0900_0041)], 4);
        e.set_global(PLAYER_SINGLETON, player);
        returns(&mut e, POP_FROM_ENTER_STACK, 0xffff_ffff);
        update_pipboy(&mut e, m);
        assert_eq!(
            e.get(m, InterfaceManager::field_4bc),
            4,
            "pop failed: still lowering"
        );
        returns(&mut e, POP_FROM_ENTER_STACK, 0);
        returns(&mut e, GET_PIPBOY_STATIC, 0x8080);
        returns(&mut e, PLAYER_GET_ANIMATION, 0x2000_0a00);
        returns(&mut e, ANIMATION_GROUP_ID, 0x1234);
        let reload = recorder(&mut e, RELOAD_DYNAMIC_IDLE_ON_1ST_PERSON, 0);
        returns(&mut e, GET_SAVED_ACQUIRE_OBJECT, 0x2000_0b00);
        update_pipboy(&mut e, m);
        assert_eq!(e.get(m, InterfaceManager::field_4bc), 5);
        // The player's virtual slot 0x214 answers 4: the idle is reloaded.
        assert_eq!(reload.borrow().as_slice(), &[vec![0x2000_0b00, player]]);
        // Slot 0x4b0 gets the animation group and 1.
        assert_eq!(calls(&e, 0x0900_0041), vec![vec![player, 0x1234, 1]]);
        assert_eq!(calls(&e, MAP_MENU_CLEAR_MAP_MEMORY).len(), 1);
        assert_eq!(e.mem.u8(sounds + 0x20), 0, "the sound pauses are released");
        assert_eq!(calls(&e, AUDIO_UNPAUSE_TYPE_4000_0000).len(), 2);
    }

    #[test]
    fn the_pipboy_returns_to_idle_when_the_player_leaves_it() {
        let mut e = world();
        let m = manager(&mut e);
        e.set(m, InterfaceManager::field_4bc, 5);
        let (player, _) = virtual_object(&mut e, &[(0x4b0, 0x0900_0050)], 0);
        e.set_global(PLAYER_SINGLETON, player);
        e.set(m, InterfaceManager::field_4c0, 0x0900_0051);
        let callback = recorder(&mut e, 0x0900_0051, 0);
        returns(&mut e, PLAYER_IS_PIPBOY_ACTIVE, 1);
        update_pipboy(&mut e, m);
        assert_eq!(e.get(m, InterfaceManager::field_4bc), 5);
        returns(&mut e, PLAYER_IS_PIPBOY_ACTIVE, 0);
        returns(&mut e, ANIMATION_GROUP_ID, 0x4321);
        update_pipboy(&mut e, m);
        assert_eq!(e.get(m, InterfaceManager::field_4bc), 0);
        assert_eq!(callback.borrow().len(), 1);
        assert_eq!(e.get(m, InterfaceManager::field_4c0), 0);
        assert_eq!(calls(&e, 0x0900_0050), vec![vec![player, 0x4321, 1]]);
        assert_eq!(calls(&e, ANIMATION_GROUP_ID)[0][1], 2);
    }

    #[test]
    fn the_animation_word_getter_maps_two_indices() {
        let mut e = world();
        let anim = Ptr::<()>::new(e.mem.alloc(0x100));
        for i in 0..8u32 {
            e.mem.set_u32(anim.addr() + 0x5c + 4 * i, 100 + i);
        }
        assert_eq!(fn_0070f490(&mut e, anim, 2), 102);
        assert_eq!(fn_0070f490(&mut e, anim, 0x14), 101);
        assert_eq!(fn_0070f490(&mut e, anim, 0x15), 104);
        assert_eq!(fn_0070f490(&mut e, anim, 0), 100);
    }

    fn pipboy_request_world() -> (Engine, Ptr<InterfaceManager>, u32) {
        let mut e = world();
        let m = manager(&mut e);
        let (player, _) = virtual_object(&mut e, &[(0x22c, 0x0900_0060), (0x230, 0x0900_0061)], 0);
        e.set_global(PLAYER_SINGLETON, player);
        e.set(m, InterfaceManager::pMenusRoot, 0x9999);
        returns(&mut e, TILE_IS_ACCEPTING_EVENTS, 1);
        (e, m, player)
    }

    #[test]
    fn a_pipboy_request_when_closed_stores_the_callback() {
        let (mut e, m, _) = pipboy_request_world();
        fn_0070f4e0(&mut e, m, 0x0900_0070, 0x3eb);
        assert_eq!(e.get(m, InterfaceManager::field_4bc), 1);
        assert_eq!(e.get(m, InterfaceManager::field_4c0), 0x0900_0070);
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![0x9999, 0x1771, 0x3eb]]);
        // Asked again while opening: nothing changes.
        fn_0070f4e0(&mut e, m, 0x0900_0071, 0);
        assert_eq!(e.get(m, InterfaceManager::field_4c0), 0x0900_0070);
    }

    #[test]
    fn a_pipboy_request_when_open_runs_the_callback_at_once() {
        let (mut e, m, _) = pipboy_request_world();
        e.set(m, InterfaceManager::field_4bc, 3);
        let callback = recorder(&mut e, 0x0900_0072, 0);
        fn_0070f4e0(&mut e, m, 0x0900_0072, 0);
        assert_eq!(callback.borrow().len(), 1);
        assert_eq!(e.get(m, InterfaceManager::field_4bc), 3);
    }

    #[test]
    fn a_pipboy_request_is_refused_while_the_player_is_busy() {
        let (mut e, m, _) = pipboy_request_world();
        returns(&mut e, CONDITION_CHECK_005A03F0, 1);
        fn_0070f4e0(&mut e, m, 0x0900_0073, 5);
        assert_eq!(e.get(m, InterfaceManager::field_4bc), 0);
        assert!(calls(&e, TILE_SET_INT).is_empty());
        returns(&mut e, CONDITION_CHECK_005A03F0, 0);
        // A weapon state value that is not 0.0 refuses too.
        returns_st0(&mut e, WEAPON_STATE_FLOAT, 0.5);
        fn_0070f4e0(&mut e, m, 0x0900_0073, 5);
        assert_eq!(e.get(m, InterfaceManager::field_4bc), 0);
        returns_st0(&mut e, WEAPON_STATE_FLOAT, 0.0);
        // The hot keys object blocks it when its sub-object says so.
        e.set_global(HOT_KEYS_OWNER, 0x2000_0c00u32);
        returns(&mut e, SUB_OBJECT_QUERY_00701740, 1);
        fn_0070f4e0(&mut e, m, 0x0900_0073, 5);
        assert_eq!(e.get(m, InterfaceManager::field_4bc), 0);
        assert_eq!(calls(&e, SUB_OBJECT_QUERY_00701740).len(), 1);
        returns(&mut e, SUB_OBJECT_QUERY_00701740, 0);
        fn_0070f4e0(&mut e, m, 0x0900_0073, 5);
        assert_eq!(e.get(m, InterfaceManager::field_4bc), 1);
        assert_eq!(
            calls(&e, SUB_OBJECT_QUERY_00701740),
            vec![vec![0x2000_0c00 + 0x1c8]; 2]
        );
    }

    #[test]
    fn the_hot_keys_query_forwards_to_the_sub_object() {
        let mut e = world();
        returns(&mut e, SUB_OBJECT_QUERY_00701740, 7);
        assert_eq!(fn_0070f670(&mut e, Ptr::new(0x2000_0d00)), 7);
        assert_eq!(
            calls(&e, SUB_OBJECT_QUERY_00701740),
            vec![vec![0x2000_0d00 + 0x1c8]]
        );
    }

    #[test]
    fn lowering_is_requested_in_the_open_state_only() {
        let mut e = world();
        let m = manager(&mut e);
        let callback = recorder(&mut e, 0x0900_0080, 0);
        // Closed with a callback: run at once.
        fn_0070f690(&mut e, m, 0x0900_0080);
        assert_eq!(callback.borrow().len(), 1);
        // Closed without one: nothing.
        fn_0070f690(&mut e, m, 0);
        assert_eq!(e.get(m, InterfaceManager::field_4bc), 0);
        // Open: lowering starts and the callback is kept.
        e.set(m, InterfaceManager::field_4bc, 3);
        fn_0070f690(&mut e, m, 0x0900_0080);
        assert_eq!(e.get(m, InterfaceManager::field_4bc), 4);
        assert_eq!(e.get(m, InterfaceManager::field_4c0), 0x0900_0080);
        assert_eq!(callback.borrow().len(), 1);
        // Another state: nothing.
        e.set(m, InterfaceManager::field_4bc, 2);
        fn_0070f690(&mut e, m, 0);
        assert_eq!(e.get(m, InterfaceManager::field_4bc), 2);
    }

    /// A manager with a cursor and a pipboy object, in the given menu mode,
    /// whose last-seen menu mode is already that mode.
    fn idle_world(mode: u32) -> (Engine, Ptr<InterfaceManager>) {
        let mut e = arithmetic_world();
        let m = manager(&mut e);
        e.set(m, InterfaceManager::cMenuMode, mode);
        e.set(m, InterfaceManager::pCursor, 0x2000_0e00);
        e.set_global(LAST_MENU_MODE, mode);
        let pipboy = e.mem.alloc(0x200);
        returns(&mut e, GET_PIPBOY, pipboy);
        let (player, _) = virtual_object(
            &mut e,
            &[
                (0x214, 0x0900_00c0),
                (0x22c, 0x0900_00c1),
                (0x230, 0x0900_00c2),
                (0x4b0, 0x0900_00c3),
            ],
            0,
        );
        e.set_global(PLAYER_SINGLETON, player);
        returns(&mut e, XINPUT_GET_STATE, 0x48f);
        e.mem.set_f64(TWO, 2.0);
        e.mem.set_f64(ONE_DOUBLE, 1.0);
        // The screen edges the cursor is clamped to.
        returns_st0(&mut e, EDGE_RIGHT, 640.0);
        returns_st0(&mut e, EDGE_LEFT, -640.0);
        returns_st0(&mut e, EDGE_TOP, 480.0);
        returns_st0(&mut e, EDGE_BOTTOM, -480.0);
        e.mem.set_f64(CURSOR_HORIZONTAL_SCALE, 1280.0);
        e.mem.set_f64(CURSOR_VERTICAL_SCALE, 960.0);
        e.mem.set_f64(MOUSE_LEVEL_DIVISOR, 20.0);
        (e, m)
    }

    /// The calls logged since `e.call_log` was set, as the frame model's
    /// callees.
    fn model_log(e: &mut Engine) -> Vec<world::frame::interface::Callee> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .map(|(a, _)| world::frame::interface::Callee::Direct(a))
            .collect()
    }

    fn check_model(
        address: u32,
        s: &world::frame::interface::InterfaceState,
        log: &[world::frame::interface::Callee],
        ignore: &[u32],
    ) {
        use world::frame::interface::{follows, function, Callee};
        let model = function(address).expect("modelled");
        let ignore: Vec<Callee> = ignore.iter().map(|&a| Callee::Direct(a)).collect();
        follows(model, s, log, &ignore).unwrap_or_else(|why| panic!("{address:08x} {s:?}: {why}"));
    }

    /// `InterfaceManager::PreIdleStuff` (`0070b8f0`) follows
    /// `world::frame::interface`'s model in each mode (docs/FRAME_SKELETON.md,
    /// PR 7). The sound pause `0070bba0` is called as a Rust function.
    #[test]
    fn pre_idle_follows_the_frame_model() {
        use world::frame::interface::{mode, InterfaceState};
        for (m, locked, pausing) in [
            (mode::GAME, false, true),
            (mode::GAME, false, false),
            (mode::OPENING, false, false),
            (mode::OPENING, true, false),
            (mode::TAKING_OVER, false, false),
            (mode::MENUS, false, false),
            (mode::CLOSING, false, false),
        ] {
            let mut e = audio_world();
            let im = manager(&mut e);
            e.set(im, InterfaceManager::cMenuMode, m);
            e.set(im, InterfaceManager::bLockMenuModeForFade, u8::from(locked));
            e.set(im, InterfaceManager::pCursor, 0x7000);
            returns(&mut e, MENU_CONSOLE_INSTANCE, 0x1111);
            returns(&mut e, MENU_MODE_IS_NOT_ONE, u32::from(pausing));
            e.call_log = Some(vec![]);
            interface_manager_pre_idle_stuff(&mut e, im);
            let log = model_log(&mut e);
            let s = InterfaceState {
                mode: m,
                fade_lock: locked,
                ..InterfaceState::default()
            };
            check_model(0x0070_b8f0, &s, &log, &[0x0070_bba0]);
        }
    }

    /// `InterfaceManager::Idle` (`0070c4a0`) in the game, back in the game,
    /// with menus up and taking over: the game's part only in the game, the
    /// menus' part only with menus. `0070f4e0`, `0070f690`, `0070ee80`,
    /// `0070f6e0` and `00710ad0` are called as Rust functions.
    #[test]
    fn idle_follows_the_frame_model() {
        use world::frame::interface::{mode, InterfaceState};
        let rust = [
            0x0070_f4e0,
            0x0070_f690,
            0x0070_ee80,
            0x0070_f6e0,
            0x0071_0ad0,
        ];
        for (m, changed) in [
            (mode::GAME, false),
            (mode::GAME, true),
            (mode::MENUS, false),
            (mode::TAKING_OVER, false),
            (mode::TAKING_OVER, true),
        ] {
            let (mut e, im) = idle_world(m);
            if changed {
                e.set_global(LAST_MENU_MODE, 0u32);
            }
            e.call_log = Some(vec![]);
            interface_manager_idle(&mut e, im);
            let log = model_log(&mut e);
            let s = InterfaceState {
                mode: m,
                mode_changed: changed,
                ..InterfaceState::default()
            };
            check_model(0x0070_c4a0, &s, &log, &rust);
        }
    }

    /// The last-minute update (`00713c70`) with the tiles' array empty or
    /// not, the loading menu up, and the AI threads' pool in use. `00713c00`,
    /// `00713d60` and `00713e20` are called as Rust functions, and the first
    /// takes the lock too.
    #[test]
    fn last_minute_update_follows_the_frame_model() {
        use world::frame::interface::InterfaceState;
        for (empty, loading, threads, running) in [
            (true, false, 1, false),
            (false, false, 1, false),
            (true, true, 1, false),
            (true, false, 2, true),
            (true, false, 2, false),
        ] {
            let (mut e, im) = tile_update_world();
            e.register_double(COLLECTION_IS_EMPTY, move |_, a| {
                // PreIdle's deferred queue is separate from the tile update
                // array; making both permanently nonempty hangs its drain.
                match a[0] {
                    UPDATE_ARRAY => Ret {
                        eax: u32::from(empty),
                        ..Ret::default()
                    },
                    DEFERRED_QUEUE => Ret {
                        eax: 1,
                        ..Ret::default()
                    },
                    other => panic!("unexpected collection {other:08x}"),
                }
            });
            returns(&mut e, IS_IN_GAME_LOADING_MENU_OPEN, u32::from(loading));
            let setting = e.call(GET_SETTING_VALUE, &args![0u32]).u32();
            e.mem.set_i32(setting, threads);
            e.set_global(SEMAPHORE_POOL_IN_USE, u8::from(running));
            e.call_log = Some(vec![]);
            fn_00713c70(&mut e, im);
            let log = model_log(&mut e);
            let s = InterfaceState {
                threads,
                threads_running: running,
                loading_menu: loading,
                tile_queue_empty: empty,
                ..InterfaceState::default()
            };
            check_model(
                0x0071_3c70,
                &s,
                &log,
                &[
                    0x0071_3c00,
                    0x0071_3d60,
                    0x0071_3e20,
                    LOCK_ENTER,
                    LOCK_LEAVE,
                ],
            );
        }
    }

    #[test]
    fn idle_in_the_game_mode_keeps_the_hud_and_the_player_effects_going() {
        let (mut e, m) = idle_world(1);
        e.set_global(CURSOR_STRING_PENDING, 1u8);
        e.set_global(LEVEL_UP_MENU_PENDING, 1u8);
        returns(&mut e, STRING_HOLDER_GET, 0x6666);
        interface_manager_idle(&mut e, m);
        // No controller: the has-controller flag is clear and no message.
        assert_eq!(e.global::<u8>(HAS_360_CONTROLLER), 0);
        assert!(calls(&e, SHOW_MESSAGE_BOX).is_empty());
        // The scope guard carries the source line.
        assert_eq!(
            calls(&e, SCOPE_GUARD_BEGIN)[0][1..],
            [0xd, 1, SOURCE_FILE, 0x47a]
        );
        // The HUD main menu is created because no tile has class `0x3ec`.
        assert_eq!(calls(&e, TILE_GET_MENU_BY_CLASS)[0], vec![0x3ec]);
        assert_eq!(calls(&e, HUD_MAIN_MENU_CREATE).len(), 1);
        // The pending cursor string is set once and the level-up menu made.
        assert_eq!(
            calls(&e, TILE_SET_STRING),
            vec![vec![0x2000_0e00, 0xfcc, 0x6666, 1]]
        );
        assert_eq!(e.global::<u8>(CURSOR_STRING_PENDING), 0);
        assert_eq!(calls(&e, LEVEL_UP_MENU_CREATE).len(), 1);
        assert_eq!(e.global::<u8>(LEVEL_UP_MENU_PENDING), 0);
        let player = e.global::<u32>(PLAYER_SINGLETON);
        assert_eq!(calls(&e, PLAYER_CAST_EAT_DRINK_ITEMS), vec![vec![player]]);
        assert_eq!(
            calls(&e, PLAYER_CAST_QUEUED_ENCHANTMENTS),
            vec![vec![player]]
        );
        // The effect managers inside the manager update every frame.
        assert_eq!(calls(&e, HUD_EFFECTS_UPDATE), vec![vec![m.addr() + 0x178]]);
        assert_eq!(calls(&e, VATS_EFFECTS_UPDATE), vec![vec![m.addr() + 0x1dc]]);
        assert_eq!(
            calls(&e, TUTORIAL_MANAGER_UPDATE),
            vec![vec![m.addr() + 0x4d4]]
        );
        assert_eq!(calls(&e, SCOPE_GUARD_END).len(), 1);
    }

    #[test]
    fn idle_announces_a_controller_connecting_once() {
        let (mut e, m) = idle_world(1);
        returns(&mut e, XINPUT_GET_STATE, 0x48f);
        interface_manager_idle(&mut e, m);
        assert!(calls(&e, SHOW_MESSAGE_BOX).is_empty());
        // A controller appears: the message box and the notification.
        returns(&mut e, XINPUT_GET_STATE, 0);
        returns(&mut e, STRING_HOLDER_GET, 0x6666);
        returns(&mut e, HAS_360_CONTROLLER_GETTER, 1);
        interface_manager_idle(&mut e, m);
        assert_eq!(e.global::<u8>(HAS_360_CONTROLLER), 1);
        assert_eq!(
            calls(&e, SHOW_MESSAGE_BOX),
            vec![vec![
                0x6666,
                0,
                0,
                0,
                0,
                0x17,
                0.0f32.to_bits(),
                0.0f32.to_bits(),
                0x6666,
                0
            ]]
        );
        assert_eq!(calls(&e, STRING_HOLDER_GET)[0], vec![MESSAGE_TITLE_HOLDER]);
        assert_eq!(
            calls(&e, STRING_HOLDER_GET)[1],
            vec![TEXT_HOLDER_CONTROLLER_WAS_CLEAR]
        );
        assert_eq!(calls(&e, CONTROLLER_CHANGED), vec![vec![m.addr(), 1]]);
        assert_eq!(e.global::<u8>(LAST_HAS_CONTROLLER), 1);
        interface_manager_idle(&mut e, m);
        assert_eq!(calls(&e, SHOW_MESSAGE_BOX).len(), 1, "announced once");
    }

    #[test]
    fn idle_picks_the_hud_mode_when_returning_to_the_game() {
        let (mut e, m) = idle_world(1);
        e.set_global(LAST_MENU_MODE, 2u32);
        let manager_block = e.mem.alloc(0x40);
        e.mem.set_u8(manager_block + 0x20, 1);
        e.mem.set_u8(manager_block + 0x21, 2);
        returns(&mut e, GET_MANAGER, manager_block);
        returns(&mut e, AUDIO_INSTANCE, 0x3030);
        // The player's virtual slot 0x22c answers true: HUD mode 7.
        let (player, _) = virtual_object(&mut e, &[(0x22c, 0x0900_0090), (0x214, 0x0900_0091)], 1);
        e.set_global(PLAYER_SINGLETON, player);
        returns(&mut e, TILE_GET_MENU_BY_CLASS, 0x1);
        interface_manager_idle(&mut e, m);
        assert_eq!(calls(&e, HUD_SET_MENU_MODE), vec![vec![7]]);
        assert_eq!(e.global::<u32>(LAST_MENU_MODE), 1);
        assert_eq!(e.mem.u8(manager_block + 0x20), 0);
        assert_eq!(e.mem.u8(manager_block + 0x21), 0);
        assert_eq!(calls(&e, AUDIO_UNPAUSE_TYPE_6).len(), 2);
        assert_eq!(calls(&e, AUDIO_UNPAUSE_TYPE_4000_0000).len(), 1);
        // The HUD exists (a tile has class 0x3ec): not created.
        assert!(calls(&e, HUD_MAIN_MENU_CREATE).is_empty());
    }

    #[test]
    fn idle_maps_the_top_menu_to_a_hud_mode_in_the_opening_state() {
        for (top, expected) in [
            (0x3e9, 0x11),
            (0x13, 3),
            (0x3ef, 5),
            (0x438, 0x19),
            (0x401, 4),
            (0x41f, 0xf),
        ] {
            let (mut e, m) = idle_world(5);
            e.set_global(LAST_MENU_MODE, 2u32);
            returns(&mut e, GET_ENTER_STACK_TOP, top);
            interface_manager_idle(&mut e, m);
            assert_eq!(
                calls(&e, HUD_SET_MENU_MODE),
                vec![vec![expected]],
                "top {top:#x}"
            );
            assert_eq!(e.global::<u32>(LAST_MENU_MODE), 5);
        }
    }

    #[test]
    fn idle_in_the_menus_clicks_the_tile_under_the_pointer() {
        let (mut e, m) = idle_world(2);
        // The tile under the pointer and its menu (a virtual object).
        let (menu, mut menu_logs) = virtual_object(
            &mut e,
            &[
                (8, 0x0900_00a0),
                (0x18, 0x0900_00a3),
                (0x20, 0x0900_00a1),
                (0x24, 0x0900_00a2),
            ],
            1,
        );
        e.set(m, InterfaceManager::bMouseInMotion, 1);
        e.set(m, InterfaceManager::pOverTileTarget, 0x7100);
        e.set(m, InterfaceManager::pOverTileMenu, menu);
        returns(&mut e, PICK_TILE, 0x7100);
        returns(&mut e, TILE_GET_MENU, menu);
        returns(&mut e, MENU_STATE, 1);
        returns_st0(&mut e, TILE_GET_VALUE, 42.0);
        returns(&mut e, CONTROLS_GET, 0x2222);
        // The primary button was pressed and is held.
        e.register(CONTROLS_QUERY_00A23A50, |_, a| Ret {
            eax: (a[2] == 1 || a[2] == 0) as u32,
            ..Ret::default()
        });
        interface_manager_idle(&mut e, m);
        // The tile is already entered: the press goes to the menu's slot 8
        // with the menu entry id (42) and the tile, becoming the drag-over
        // tile; the held button reaches slots 0x20 and 0x24.
        assert_eq!(e.get(m, InterfaceManager::pDragOverTileTarget), 0x7100);
        assert_eq!(e.get(m, InterfaceManager::pDragOverTileMenu), menu);
        let click = menu_logs.remove(0);
        let _drag_over = menu_logs.remove(0);
        let held = menu_logs.remove(0);
        let drag_held = menu_logs.remove(0);
        assert_eq!(click.borrow().as_slice(), &[vec![menu, 42, 0x7100]]);
        assert_eq!(held.borrow().as_slice(), &[vec![menu, 42, 0x7100]]);
        assert_eq!(drag_held.borrow().as_slice(), &[vec![menu, 42, 0x7100]]);
        assert!(calls(&e, DO_ENTER).is_empty(), "same tile: nothing entered");
    }

    #[test]
    fn idle_scrolls_the_menu_with_the_wheel() {
        let (mut e, m) = idle_world(2);
        let (menu, _) = virtual_object(&mut e, &[], 0);
        e.set(m, InterfaceManager::pOverTileTarget, 0x7200);
        e.set(m, InterfaceManager::pOverTileMenu, menu);
        returns(&mut e, CONTROLS_GET, 0x2222);
        // The wheel query answers 240; the tile wants the wheel.
        e.register(CONTROLS_QUERY_00A239E0, |_, a| Ret {
            eax: if a[1] == 3 { 240 } else { 0 },
            ..Ret::default()
        });
        returns(&mut e, GET_WHEEL_STEPS, 240);
        returns(&mut e, TILE_IS_TRUE, 1);
        returns_st0(&mut e, TILE_GET_VALUE, 9.0);
        interface_manager_idle(&mut e, m);
        assert_eq!(e.get(m, InterfaceManager::fMouseWheel), 240.0);
        // 240 / -120: the wheel trait is pulsed with -2 and cleared.
        let sets = calls(&e, TILE_SET_INT);
        assert!(sets.contains(&vec![0x7200, 0xff3, (-2i32) as u32]));
        assert!(sets.contains(&vec![0x7200, 0xff3, 0]));
        assert_eq!(
            calls(&e, DO_WHEEL_MOVE),
            vec![vec![m.addr(), menu, 9, 0x7200]]
        );
    }

    #[test]
    fn idle_opens_the_pipboy_page_for_a_hot_key() {
        let (mut e, m) = idle_world(1);
        returns(&mut e, TILE_GET_MENU_BY_CLASS, 1);
        returns(&mut e, CONTROLS_GET, 0x2222);
        key_query(&mut e, CONTROLS_QUERY_00A24180, 0x3c, 1);
        returns(&mut e, GET_MENUS_ROOT, 0x9999);
        // Closed pipboy: the request goes to `0070f4e0` (refused here).
        returns(&mut e, CONDITION_CHECK_005A03F0, 1);
        interface_manager_idle(&mut e, m);
        let player = e.global::<u32>(PLAYER_SINGLETON);
        assert_eq!(calls(&e, CONDITION_CHECK_005A03F0), vec![vec![player, 4]]);
        // Open pipboy on that page already: the request lowers it.
        let (mut e, m) = idle_world(1);
        returns(&mut e, TILE_GET_MENU_BY_CLASS, 1);
        returns(&mut e, CONTROLS_GET, 0x2222);
        key_query(&mut e, CONTROLS_QUERY_00A24180, 0x3c, 1);
        returns(&mut e, GET_MENUS_ROOT, 0x9999);
        e.set(m, InterfaceManager::field_4bc, 3);
        returns_st0(&mut e, TILE_GET_VALUE, 1002.0);
        returns(&mut e, POP_FROM_ENTER_STACK, 0xffff_ffff);
        interface_manager_idle(&mut e, m);
        assert_eq!(e.get(m, InterfaceManager::field_4bc), 4);
        // On another page: the page is switched instead.
        let (mut e, m) = idle_world(1);
        returns(&mut e, TILE_GET_MENU_BY_CLASS, 1);
        returns(&mut e, CONTROLS_GET, 0x2222);
        key_query(&mut e, CONTROLS_QUERY_00A24180, 0x3c, 1);
        returns(&mut e, GET_MENUS_ROOT, 0x9999);
        e.set(m, InterfaceManager::field_4bc, 3);
        returns_st0(&mut e, TILE_GET_VALUE, 1.0);
        interface_manager_idle(&mut e, m);
        assert_eq!(calls(&e, SET_INVENTORY_MENU_VISIBLE), vec![vec![1, 0, 1]]);
    }

    #[test]
    fn idle_toggles_the_console_with_its_key() {
        let (mut e, m) = idle_world(1);
        returns(&mut e, TILE_GET_MENU_BY_CLASS, 1);
        returns(&mut e, CONTROLS_GET, 0x2222);
        key_query(&mut e, CONTROLS_QUERY_00A24660, 0x1d, 1);
        returns(&mut e, MENU_CONSOLE_INSTANCE, 0x1111);
        returns(&mut e, MENU_CONSOLE_TOGGLE_VISIBLE, 1);
        interface_manager_idle(&mut e, m);
        assert_eq!(calls(&e, CLEAR_KEYSTROKES), vec![vec![0x2222]]);
        assert_eq!(calls(&e, MENU_CONSOLE_TOGGLE_VISIBLE), vec![vec![0x1111]]);
        assert_eq!(calls(&e, ADD_TO_ENTER_STACK), vec![vec![m.addr(), 3]]);
        returns(&mut e, MENU_CONSOLE_TOGGLE_VISIBLE, 0);
        interface_manager_idle(&mut e, m);
        assert_eq!(calls(&e, POP_FROM_ENTER_STACK), vec![vec![m.addr(), 3, 0]]);
    }

    #[test]
    fn idle_runs_each_root_menus_per_frame_virtual() {
        let (mut e, m) = idle_world(1);
        returns(&mut e, TILE_GET_MENU_BY_CLASS, 1);
        let (menu, mut logs) = virtual_object(&mut e, &[(0x2c, 0x0900_00b0)], 0);
        e.set(m, InterfaceManager::pMenusRoot, 0x2000_0f00);
        // The children list: one node holding the child tile.
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(node + 8, 0x7300);
        returns(&mut e, NI_POINTER_GET, node);
        e.register(LIST_NEXT_ELEMENT, |e, a| {
            let node = e.mem.u32(a[1]);
            e.mem.set_u32(a[1], 0);
            Ret {
                eax: node + 8,
                ..Ret::default()
            }
        });
        returns(&mut e, TILE_GET_MENU, menu);
        interface_manager_idle(&mut e, m);
        assert_eq!(logs.remove(0).borrow().as_slice(), &[vec![menu]]);
        assert_eq!(calls(&e, LIST_NEXT_ELEMENT).len(), 1);
    }

    /// `CheckMenuButton(button, 1)` answers from `states`; any other button
    /// reports 3 (neither pressed, held nor released).
    fn button_states(e: &mut Engine, states: &[(u32, u32)]) {
        let table: Vec<(u32, u32)> = states.to_vec();
        e.register_double(CHECK_MENU_BUTTON, move |_, a| Ret {
            eax: table
                .iter()
                .find(|(button, _)| *button == a[0])
                .map(|(_, state)| *state)
                .unwrap_or(3),
            ..Ret::default()
        });
    }

    fn gamepad_world() -> (Engine, Ptr<InterfaceManager>, Ptr) {
        let mut e = arithmetic_world();
        let m = manager(&mut e);
        returns(&mut e, HAS_360_CONTROLLER_GETTER, 1);
        returns(&mut e, FADE_CLOCK_READ, 1000);
        let out = Ptr::<()>::new(e.mem.alloc(4));
        (e, m, out)
    }

    #[test]
    fn without_a_controller_no_button_acts() {
        let (mut e, m, out) = gamepad_world();
        returns(&mut e, HAS_360_CONTROLLER_GETTER, 0);
        assert_eq!(
            interface_manager_get_top_gamepad_button(&mut e, m, 0, 0, out),
            0
        );
        assert!(calls(&e, CHECK_MENU_BUTTON).is_empty());
        returns(&mut e, HAS_360_CONTROLLER_GETTER, 1);
        e.set(m, InterfaceManager::bShowMouse, 1);
        assert_eq!(
            interface_manager_get_top_gamepad_button(&mut e, m, 0, 0, out),
            0
        );
    }

    #[test]
    fn a_pressed_or_held_face_button_acts_at_once() {
        let (mut e, m, out) = gamepad_world();
        e.set(m, InterfaceManager::iLastGamepadEvent, 4);
        button_states(&mut e, &[(9, 2)]);
        assert_eq!(
            interface_manager_get_top_gamepad_button(&mut e, m, 0, 0, out),
            9
        );
        assert_eq!(e.mem.u8(out.addr()), 0, "pressed, not held");
        assert_eq!(e.get(m, InterfaceManager::iLastGamepadEvent), 0);
        button_states(&mut e, &[(10, 1)]);
        assert_eq!(
            interface_manager_get_top_gamepad_button(&mut e, m, 0, 0, out),
            10
        );
        assert_eq!(e.mem.u8(out.addr()), 1, "held");
        // A press of button 6 wins over the shoulder buttons.
        button_states(&mut e, &[(6, 2), (0xf, 2)]);
        assert_eq!(
            interface_manager_get_top_gamepad_button(&mut e, m, 0, 0, out),
            6
        );
    }

    #[test]
    fn a_direction_repeats_after_its_delay() {
        let (mut e, m, out) = gamepad_world();
        let slow = float_cell(&mut e, 250.0);
        let fast = float_cell(&mut e, 50.0);
        e.register_double(FLOAT_HOLDER_GET, move |_, a| Ret {
            eax: if a[0] == REPEAT_DELAY_HOLDER_FIRST {
                slow
            } else {
                fast
            },
            ..Ret::default()
        });
        // Up is pressed: the first press, with the repeat wanted by tile.
        button_states(&mut e, &[(1, 2)]);
        assert_eq!(
            interface_manager_get_top_gamepad_button(&mut e, m, 1, 0, out),
            1
        );
        assert_eq!(e.get(m, InterfaceManager::iLastGamepadEvent), 1);
        assert_eq!(e.get(m, InterfaceManager::uGamepadRepeatStartTime), 1250);
        // Without a repeating tile the delay is twice the tick count.
        e.set(m, InterfaceManager::iLastGamepadEvent, 0);
        assert_eq!(
            interface_manager_get_top_gamepad_button(&mut e, m, 0, 0, out),
            1
        );
        assert_eq!(e.get(m, InterfaceManager::uGamepadRepeatStartTime), 2000);
        // Released: the previous event repeats once its time has come.
        button_states(&mut e, &[(1, 0)]);
        e.set(m, InterfaceManager::iLastGamepadEvent, 1);
        e.set(m, InterfaceManager::uGamepadRepeatStartTime, 1500);
        assert_eq!(
            interface_manager_get_top_gamepad_button(&mut e, m, 1, 0, out),
            0,
            "not yet"
        );
        e.set(m, InterfaceManager::uGamepadRepeatStartTime, 900);
        assert_eq!(
            interface_manager_get_top_gamepad_button(&mut e, m, 1, 0, out),
            1
        );
        assert_eq!(e.get(m, InterfaceManager::uGamepadRepeatStartTime), 1050);
    }

    #[test]
    fn idle_gamepad_resets_the_repeat_state() {
        let (mut e, m, out) = gamepad_world();
        e.set(m, InterfaceManager::iLastGamepadEvent, 3);
        button_states(&mut e, &[]);
        assert_eq!(
            interface_manager_get_top_gamepad_button(&mut e, m, 0, 0, out),
            0
        );
        assert_eq!(e.get(m, InterfaceManager::iLastGamepadEvent), 0);
        assert_eq!(e.get(m, InterfaceManager::uGamepadRepeatStartTime), 2000);
        // A shoulder button pair: only 0xf pressed.
        button_states(&mut e, &[(0xf, 2)]);
        assert_eq!(
            interface_manager_get_top_gamepad_button(&mut e, m, 0, 0, out),
            0xf
        );
        button_states(&mut e, &[(0xd, 2), (5, 2)]);
        assert_eq!(
            interface_manager_get_top_gamepad_button(&mut e, m, 0, 0, out),
            0xd
        );
        button_states(&mut e, &[(5, 2)]);
        assert_eq!(
            interface_manager_get_top_gamepad_button(&mut e, m, 0, 0, out),
            5
        );
    }

    /// A root menu list with the given children (tile addresses), iterated
    /// from the last: the previous-element double walks `children`.
    fn menu_list(e: &mut Engine, m: Ptr<InterfaceManager>, children: &[u32]) {
        e.set(m, InterfaceManager::pMenusRoot, 0x2000_1000);
        returns(e, GET_MENUS_ROOT, 0x2000_1000);
        let tiles: Vec<u32> = children.to_vec();
        let cursor = Rc::new(RefCell::new(tiles.len()));
        let tail = if tiles.is_empty() { 0 } else { 1 };
        e.register_double(GET_FIELD_AT_4, move |_, a| Ret {
            eax: if a[0] == 0x2000_1004 { tail } else { 0x7f00 },
            ..Ret::default()
        });
        let cells: Vec<u32> = tiles
            .iter()
            .map(|t| {
                let cell = e.mem.alloc(4);
                e.mem.set_u32(cell, *t);
                cell
            })
            .collect();
        e.register_double(LIST_PREVIOUS_ELEMENT, move |e, a| {
            let mut index = cursor.borrow_mut();
            *index -= 1;
            let cell = cells[*index];
            e.mem.set_u32(a[1], *index as u32);
            Ret {
                eax: cell,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn a_root_menu_can_take_the_event_before_anything_else() {
        let (mut e, m, _) = gamepad_world();
        returns(&mut e, HAS_360_CONTROLLER_GETTER, 0);
        menu_list(&mut e, m, &[0x7a00, 0x7b00]);
        returns(&mut e, TILE_GET_MENU, 0x7c00);
        returns(&mut e, TILE_IS_VISIBLE, 1);
        returns(&mut e, DO_GAMEPAD, 1);
        interface_manager_do_gamepad_event(&mut e, m, 5);
        assert_eq!(
            calls(&e, DO_GAMEPAD),
            vec![vec![m.addr(), 0x7c00, 0xffff_ffff, 0.0f32.to_bits()]]
        );
        assert!(calls(&e, MENU_MANAGER_INSTANCE).is_empty());
    }

    #[test]
    fn the_frontmost_menu_gets_the_event_with_full_strength() {
        let (mut e, m, _) = gamepad_world();
        returns(&mut e, HAS_360_CONTROLLER_GETTER, 0);
        menu_list(&mut e, m, &[]);
        returns(&mut e, MENU_MANAGER_INSTANCE, 0x2000_2000);
        returns(&mut e, GET_FRONTMOST_MENU, 0x7d00);
        returns(&mut e, DO_GAMEPAD, 1);
        interface_manager_do_gamepad_event(&mut e, m, 5);
        assert_eq!(
            calls(&e, DO_GAMEPAD),
            vec![vec![m.addr(), 0x7d00, 5, 1.0f32.to_bits()]]
        );
        assert_eq!(calls(&e, GET_FRONTMOST_MENU), vec![vec![0x2000_2000]]);
    }

    #[test]
    fn a_released_button_goes_to_the_frontmost_menu_slot_0x3c() {
        let (mut e, m, _) = gamepad_world();
        menu_list(&mut e, m, &[]);
        let (menu, mut logs) = virtual_object(&mut e, &[(0x3c, 0x0900_00d0)], 0);
        returns(&mut e, MENU_MANAGER_INSTANCE, 0x2000_2000);
        returns(&mut e, GET_FRONTMOST_MENU, menu);
        returns(&mut e, GET_MOUSE_OVER_TARGET, 0);
        button_states(&mut e, &[(9, 1)]);
        interface_manager_do_gamepad_event(&mut e, m, 0);
        // Button 9 is held: the key repeat is reset only for other events.
        assert_eq!(
            logs.remove(0).borrow().as_slice(),
            &[vec![menu, 9, 1.0f32.to_bits()]]
        );
        assert!(calls(&e, KEY_REPEAT_00705B10).len() == 1);
        assert!(calls(&e, DO_GAMEPAD).is_empty());
    }

    #[test]
    fn with_no_menu_the_console_pick_list_steps() {
        let (mut e, m, _) = gamepad_world();
        returns(&mut e, HAS_360_CONTROLLER_GETTER, 0);
        menu_list(&mut e, m, &[]);
        returns(&mut e, MENU_CONSOLE_INSTANCE, 0x1111);
        returns(&mut e, CONSOLE_PRIMARY_DOWN, 1);
        returns(&mut e, PICK_LIST_COUNT, 3);
        e.set(m, InterfaceManager::iCurrentPickIndex, 1);
        interface_manager_do_gamepad_event(&mut e, m, 1);
        assert_eq!(e.get(m, InterfaceManager::iCurrentPickIndex), 2);
        interface_manager_do_gamepad_event(&mut e, m, 1);
        assert_eq!(
            e.get(m, InterfaceManager::iCurrentPickIndex),
            -1,
            "wraps past the end"
        );
        interface_manager_do_gamepad_event(&mut e, m, 2);
        assert_eq!(
            e.get(m, InterfaceManager::iCurrentPickIndex),
            2,
            "wraps to the last"
        );
    }

    fn menu_world() -> (Engine, Ptr<InterfaceManager>, u32) {
        let (mut e, m, _) = gamepad_world();
        returns(&mut e, HAS_360_CONTROLLER_GETTER, 0);
        menu_list(&mut e, m, &[]);
        let (frontmost, _) = virtual_object(&mut e, &[], 0);
        returns(&mut e, MENU_MANAGER_INSTANCE, 0x2000_2000);
        returns(&mut e, GET_FRONTMOST_MENU, frontmost);
        (e, m, frontmost)
    }

    #[test]
    fn a_direction_with_no_focus_clicks_the_first_reacting_tile() {
        let (mut e, m, _) = menu_world();
        let (menu, mut logs) = virtual_object(&mut e, &[(0xc, 0x0900_00e0)], 0);
        e.set_global(MANAGER_SINGLETON, m.addr());
        // Re-register the frontmost menu with a slot 0xc.
        returns(&mut e, GET_FRONTMOST_MENU, menu);
        returns(&mut e, GET_MOUSE_OVER_TARGET, 0);
        returns(&mut e, SCAN_FOR_MAX_FOCUS, 0);
        returns(&mut e, TILE_GET_FIRST_REF_COPY, 0x9300);
        returns_st0(&mut e, TILE_GET_VALUE, 11.0);
        interface_manager_do_gamepad_event(&mut e, m, 3);
        assert_eq!(e.get(m, InterfaceManager::bMouseInMotion), 0);
        // Direction 3 uses trait 0xfda; the first tile found is clicked.
        assert_eq!(calls(&e, TILE_GET_FIRST_REF_COPY)[0][0..2], [0x7f00, 0xfda]);
        assert_eq!(
            calls(&e, SET_CURRENT_FOCUS_TARGET),
            vec![vec![m.addr(), 0, 0xfc3, 1]]
        );
        assert_eq!(calls(&e, TILE_PLAY_TILE_SOUND), vec![vec![0x9300, 0xfcb]]);
        assert_eq!(
            logs.remove(0).borrow().as_slice(),
            &[vec![menu, 11, 0x9300]]
        );
    }

    #[test]
    fn a_direction_moves_the_focus_to_the_tile_that_reacts() {
        let (mut e, m, _) = menu_world();
        returns(&mut e, GET_MOUSE_OVER_TARGET, 0x9400);
        // The first reacting tile is usable (traits 0xfa3 and 0xfaf true)
        // and was found under trait 0xfc3.
        e.register(TILE_GET_FIRST_REF_COPY, |e, a| {
            e.mem.set_u32(a[2], 0xfc3);
            Ret {
                eax: 0x9500,
                ..Ret::default()
            }
        });
        returns(&mut e, TILE_IS_TRUE, 1);
        interface_manager_do_gamepad_event(&mut e, m, 1);
        assert_eq!(calls(&e, TILE_PLAY_TILE_SOUND), vec![vec![0x9500, 0xfe8]]);
        assert_eq!(
            calls(&e, SET_CURRENT_FOCUS_TARGET),
            vec![vec![m.addr(), 0x9500, 0xfc3, 1]]
        );
        // The tried tiles are kept in a list that is built and torn down.
        assert_eq!(calls(&e, SIMPLE_LIST_CONSTRUCT).len(), 1);
        assert_eq!(calls(&e, SIMPLE_LIST_DESTROY).len(), 1);
        assert_eq!(calls(&e, LIST_PUSH_FRONT_005AE3D0).len(), 1);
        assert_eq!(calls(&e, LIST_CLEAR).len(), 1);
    }

    #[test]
    fn a_direction_with_nowhere_to_go_is_offered_to_the_menu() {
        let (mut e, m, _) = menu_world();
        returns(&mut e, GET_MOUSE_OVER_TARGET, 0x9400);
        returns(&mut e, TILE_GET_FIRST_REF_COPY, 0);
        e.set(m, InterfaceManager::pMouseOverTarget, 0);
        interface_manager_do_gamepad_event(&mut e, m, 2);
        // The frontmost menu is offered the event twice (before and after);
        // nobody takes it, so the focus is set to the current tile.
        assert_eq!(calls(&e, DO_GAMEPAD).len(), 2);
        assert_eq!(
            calls(&e, SET_CURRENT_FOCUS_TARGET),
            vec![vec![m.addr(), 0x9400, 0xfc3, 1]]
        );
        assert_eq!(calls(&e, TILE_PLAY_TILE_SOUND), vec![vec![0x9400, 0xfe8]]);
    }

    #[test]
    fn a_button_event_clicks_the_tile_bound_to_it() {
        let (mut e, m, _) = menu_world();
        let (menu, mut logs) = virtual_object(&mut e, &[(0xc, 0x0900_00f0)], 0);
        returns(&mut e, GET_MOUSE_OVER_TARGET, 0x9600);
        returns(&mut e, TILE_IS_VISIBLE, 1);
        returns(&mut e, TILE_IS_TRUE, 1);
        returns(&mut e, TILE_GET_MENU, menu);
        returns_st0(&mut e, TILE_GET_VALUE, 7.0);
        e.register(TILE_GET_FIRST_REF_COPY, |e, a| {
            e.mem.set_u32(a[2], 0xfc5);
            Ret {
                eax: 0x9700,
                ..Ret::default()
            }
        });
        interface_manager_do_gamepad_event(&mut e, m, 9);
        // Trait 0xfdd belongs to button 9; the modifier bit is raised only
        // while the click runs.
        assert_eq!(calls(&e, TILE_GET_FIRST_REF_COPY)[0][0..2], [0x9600, 0xfdd]);
        assert_eq!(calls(&e, TILE_PLAY_TILE_SOUND), vec![vec![0x9700, 0xfcb]]);
        assert_eq!(logs.remove(0).borrow().as_slice(), &[vec![menu, 7, 0x9700]]);
        assert_eq!(e.get(m, InterfaceManager::iModifierKeys), 0);
        assert_eq!(calls(&e, CLEAR_MENU_BUTTON), vec![vec![9], vec![6]]);
    }

    #[test]
    fn a_button_with_a_disabled_tile_plays_the_refusal_sound() {
        let (mut e, m, _) = menu_world();
        returns(&mut e, GET_MOUSE_OVER_TARGET, 0x9600);
        returns(&mut e, TILE_IS_VISIBLE, 1);
        e.register(TILE_IS_TRUE, |_, _| Ret::default());
        e.register(TILE_GET_FIRST_REF_COPY, |e, a| {
            e.mem.set_u32(a[2], 0xfc7);
            Ret {
                eax: 0x9700,
                ..Ret::default()
            }
        });
        interface_manager_do_gamepad_event(&mut e, m, 10);
        assert_eq!(calls(&e, PLAY_MENU_SOUND), vec![vec![2]]);
        assert!(calls(&e, CLEAR_MENU_BUTTON).is_empty());
    }

    /// A pick list with the given references hit by the ray; the added list
    /// really stores them.
    fn pick_world(references: &[u32]) -> (Engine, Ptr<InterfaceManager>, u32) {
        let mut e = arithmetic_world();
        let m = manager(&mut e);
        let player = e.mem.alloc(0x1000);
        e.mem.set_f32(player + 0x670, 1.5);
        e.set_global(PLAYER_SINGLETON, player);
        e.set(m, InterfaceManager::iCurrentPickIndex, 4);
        e.mem.set_f32(m.addr() + 0x38, 640.0);
        returns(&mut e, NI_PICK_PICK_OBJECTS, 1);
        returns(&mut e, NI_PICK_GET_RESULTS, 0x2000_3000);
        returns(&mut e, RESULTS_COUNT, references.len() as u32);
        let objects: Vec<u32> = references.iter().map(|r| r + 0x1000).collect();
        e.register_double(RESULT_OBJECT, move |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register_double(RESULTS_GET, move |_, a| Ret {
            eax: objects[a[1] as usize],
            ..Ret::default()
        });
        let references: Vec<u32> = references.to_vec();
        e.register_double(FIND_REFERENCE_FOR_3D, move |_, a| Ret {
            eax: a[0] - 0x1000,
            ..Ret::default()
        });
        let _ = references;
        // The list stores what is added to it (the head holds the first).
        e.register(LIST_ADD, |e, a| {
            let head = a[0];
            if e.mem.u32(head) == 0 {
                let slot = e.mem.u32(a[1]);
                e.mem.set_u32(head, slot);
            }
            Ret::default()
        });
        (e, m, player)
    }

    #[test]
    fn the_console_pick_list_is_rebuilt_from_the_ray() {
        let (mut e, m, _) = pick_world(&[0x500, 0x600]);
        fn_00710880(&mut e, m);
        // The ray goes through the cursor position and the camera.
        let ray = calls(&e, CAMERA_BUILD_PICK_RAY);
        assert_eq!(ray.len(), 1);
        assert_eq!(ray[0][1..3], [640, 0]);
        // The field of view comes from the player.
        assert_eq!(
            calls(&e, SCENE_GRAPH_SET_CAMERA_FOV)[0][1..],
            [1.5f32.to_bits(), 0, 0, 0]
        );
        // Every reference hit is added to the new list, then copied in
        // (the old list is cleared) and the index goes back to 0.
        assert_eq!(calls(&e, LIST_ADD).len(), 3);
        assert_eq!(calls(&e, LIST_CLEAR)[0], vec![m.addr() + 0x70]);
        assert_eq!(e.get(m, InterfaceManager::iCurrentPickIndex), 0);
        assert_eq!(e.mem.u32(m.addr() + 0x70), 0x500);
        assert_eq!(calls(&e, NI_PICK_CONSTRUCT)[0][1..], [0, 8]);
        assert_eq!(calls(&e, NI_PICK_DESTRUCT).len(), 1);
        assert_eq!(calls(&e, SIMPLE_LIST_DESTROY).len(), 1);
    }

    #[test]
    fn an_unchanged_pick_list_is_not_copied_again() {
        let (mut e, m, _) = pick_world(&[0x500]);
        // The manager's list already starts with that reference.
        e.mem.set_u32(m.addr() + 0x70, 0x500);
        fn_00710880(&mut e, m);
        assert_eq!(e.get(m, InterfaceManager::iCurrentPickIndex), 0);
        // Only the one add into the new list: nothing copied into the
        // manager's list.
        assert_eq!(calls(&e, LIST_ADD).len(), 1);
        assert_eq!(calls(&e, LIST_CLEAR).len(), 2);
    }

    fn f64_words(value: f64) -> [u32; 2] {
        let bits = value.to_bits();
        [bits as u32, (bits >> 32) as u32]
    }

    /// The debug text world: line height 17 (font height 14 + 3), text top
    /// 100, chars pointer 0xc0c0 for every string, print position 600.
    fn debug_text_world() -> (Engine, Ptr<InterfaceManager>) {
        let mut e = arithmetic_world();
        let m = manager(&mut e);
        let setting = e.mem.alloc(4);
        e.mem.set_u32(setting, 100);
        returns(&mut e, GET_SETTING_VALUE, setting);
        returns_st0(&mut e, FONT_HEIGHT, 14.0);
        returns(&mut e, DEBUG_TEXT_INSTANCE, 0xd0d0);
        returns(&mut e, NI_POINTER_GET, 0xc0c0);
        e.mem.set_f32(DEBUG_TEXT_X, 600.0);
        e.mem.set_f32(CURSOR_DIRECTION_Y, -1.0);
        (e, m)
    }

    fn printed(e: &Engine) -> Vec<(u32, u32)> {
        calls(e, DEBUG_TEXT_PRINT)
            .iter()
            .map(|c| (c[0], f32::from_bits(c[3]) as u32))
            .collect()
    }

    #[test]
    fn with_no_pick_the_debug_text_is_cleared() {
        let (mut e, m) = debug_text_world();
        e.set_global(DEBUG_TEXT_BOTTOM, 200i32);
        e.set(m, InterfaceManager::pPickRef, 0x9000);
        interface_manager_display_current_pick_ref(&mut e, m);
        assert_eq!(e.get(m, InterfaceManager::pPickRef), 0);
        assert_eq!(calls(&e, SET_PICK_REF_DISPLAY), vec![vec![m.addr(), 0]]);
        // One empty line at the top, then the old lines are blanked down to
        // the previous bottom, a line apart.
        let rows: Vec<u32> = printed(&e).iter().map(|p| p.1).collect();
        assert_eq!(rows, vec![100, 130, 147, 164, 181, 198]);
        assert_eq!(
            calls(&e, DEBUG_TEXT_PRINT)[0],
            vec![
                0xd0d0,
                EMPTY_TEXT,
                600.0f32.to_bits(),
                100.0f32.to_bits(),
                2,
                0xffff_ffff,
                (-1.0f32).to_bits(),
                0,
                0
            ]
        );
        assert_eq!(e.global::<i32>(DEBUG_TEXT_BOTTOM), 130);
    }

    #[test]
    fn the_picked_reference_is_printed_with_its_name_and_form_id() {
        let (mut e, m) = debug_text_world();
        e.mem.set_u32(m.addr() + 0x70, 0x9000);
        returns(&mut e, FORM_ID_GETTER, 0x1234);
        returns(&mut e, REFR_NAME, 0x5555);
        interface_manager_display_current_pick_ref(&mut e, m);
        assert_eq!(e.get(m, InterfaceManager::pPickRef), 0x9000);
        assert_eq!(
            calls(&e, SET_PICK_REF_DISPLAY),
            vec![vec![m.addr(), 0x9000]]
        );
        let format = calls(&e, FORMAT_INTO_STRING);
        assert_eq!(format.len(), 1);
        assert_eq!(format[0][1..], [FORMAT_NAME_AND_ID, 0x5555, 0x1234]);
        assert_eq!(calls(&e, DEBUG_TEXT_PRINT)[0][1], 0xc0c0);
        assert_eq!(printed(&e), vec![(0xd0d0, 100)]);
        assert_eq!(calls(&e, STRING_CONSTRUCT).len(), 1);
        assert_eq!(calls(&e, STRING_DESTROY).len(), 1);
    }

    /// A picked reference object (virtual: slot 0x100 is the actor test)
    /// in a world where help is shown.
    fn help_world(actor: u32) -> (Engine, Ptr<InterfaceManager>, u32) {
        let (mut e, m) = debug_text_world();
        let (pick, _) = virtual_object(&mut e, &[(0x100, 0x0900_0100)], actor);
        e.mem.set_u32(m.addr() + 0x70, pick);
        e.set(m, InterfaceManager::bFullHelp, 1);
        returns(&mut e, FORM_ID_GETTER, 0x1234);
        returns(&mut e, REFR_NAME, 0x5555);
        returns(&mut e, REFR_EXTRA_DATA_LIST, 0x7a7a);
        returns(&mut e, EXTRA_GET_COUNT, 1);
        (e, m, pick)
    }

    #[test]
    fn the_help_lines_list_the_owner_and_the_stack_count() {
        let (mut e, m, pick) = help_world(0);
        let (owner, _) = virtual_object(&mut e, &[(0x130, 0x0900_0101)], 0x7777);
        returns(&mut e, REFR_GET_OWNER, owner);
        returns(&mut e, EXTRA_GET_COUNT, 3);
        interface_manager_display_current_pick_ref(&mut e, m);
        let formats = calls(&e, FORMAT_INTO_STRING);
        assert_eq!(formats.len(), 3);
        assert_eq!(formats[0][1..], [FORMAT_NAME_AND_ID, 0x5555, 0x1234]);
        assert_eq!(formats[1][1..], [OWNER_FORMAT, 0x7777, 0x1234]);
        assert_eq!(formats[2][1..], [COUNT_FORMAT, 3]);
        // The first line is at the top; the help lines start 30 below and
        // are a line (17) apart.
        let rows: Vec<u32> = printed(&e).iter().map(|p| p.1).collect();
        assert_eq!(rows, vec![100, 130, 147]);
        assert_eq!(e.global::<i32>(DEBUG_TEXT_BOTTOM), 164);
        let _ = pick;
    }

    #[test]
    fn the_help_lines_show_the_lock_and_the_teleport_door() {
        let (mut e, m, _) = help_world(0);
        let lock = e.mem.alloc(0x40);
        e.mem.set_u32(lock + 4, 0x4000_0000);
        returns(&mut e, REFR_GET_LOCK, lock);
        returns(&mut e, LOCK_IS_LOCKED, 1);
        returns(&mut e, LOCK_GET_LEVEL, 2);
        e.mem.set_u32(LOCK_LEVEL_TABLE + 8, 0x0102_0000);
        returns(&mut e, STRING_HOLDER_GET, 0x4545);
        returns(&mut e, MAP_MARKER_GET_LOCATION_NAME, 0x1111);
        let door = e.mem.alloc(0x20);
        returns(&mut e, REFR_GET_TELEPORT_DATA, door);
        returns(&mut e, DOOR_TELEPORT_GET_CELL, 0x2000_5000);
        interface_manager_display_current_pick_ref(&mut e, m);
        let formats = calls(&e, FORMAT_INTO_STRING);
        assert!(formats
            .iter()
            .any(|f| f[1..] == [LOCK_FORMAT, 0x4545, LOCKED_TEXT]));
        assert!(formats.iter().any(|f| f[1..] == [KEY_FORMAT, 0x1111]));
        assert!(formats
            .iter()
            .any(|f| f[1..] == [TELEPORT_FORMAT, 0x1111, 0x5555]));
        assert_eq!(calls(&e, LOCK_GET_LEVEL).len(), 1);
        assert_eq!(calls(&e, STRING_HOLDER_GET), vec![vec![0x0102_0000]]);
    }

    #[test]
    fn the_help_lines_spell_out_the_flags_and_the_furniture_markers() {
        let (mut e, m, pick) = help_world(0);
        returns(&mut e, REFR_FLAGS, 5);
        returns(&mut e, TRIM_TEXT, 0x6a6a);
        // The furniture: heading and two markers (enabled, disabled).
        returns(&mut e, REFR_IS_FURNITURE, 1);
        let rotation = e.mem.alloc(0xc);
        e.mem.set_f32(rotation + 8, 1.0);
        returns(&mut e, REFR_GET_ROTATION, rotation);
        e.mem.set_f64(RADIANS_TO_DEGREES, 57.0);
        e.register(REFR_GET_MARKER_AT_INDEX, |_, a| Ret {
            eax: (a[1] < 2) as u32,
            ..Ret::default()
        });
        e.register(FURNITURE_GET_MARKER_ENABLED, |_, a| Ret {
            eax: (a[1] == 0) as u32,
            ..Ret::default()
        });
        returns(&mut e, MARKER_KIND, 5);
        returns(&mut e, FURNITURE_IS_SIT_MARKER, 1);
        returns(&mut e, REFR_GET_MARKER_USED, 1);
        returns_st0(&mut e, FURNITURE_GET_MARKER_ANGLE, 2.0);
        returns_st0(&mut e, REFR_GET_SCALE, 1.0);
        e.register(FURNITURE_GET_MARKER_TARGET_OFFSET, |e, a| {
            for (i, v) in [1.5f32, 2.5, 3.5].iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, *v);
            }
            Ret::default()
        });
        interface_manager_display_current_pick_ref(&mut e, m);
        let formats = calls(&e, FORMAT_INTO_STRING);
        let words = |v: f64| f64_words(v);
        // Heading: 1.0 * 57 degrees.
        assert!(formats.iter().any(|f| f[1..] == [HEADING_FORMAT, 57]));
        // The enabled marker, with its three offsets as doubles.
        let mut enabled = vec![MARKER_FORMAT, SIT_TEXT, 5];
        for v in [1.5, 2.5, 3.5] {
            enabled.extend(words(v));
        }
        enabled.extend([114, USED_TEXT]);
        assert!(formats.iter().any(|f| f[1..] == enabled[..]));
        assert!(formats
            .iter()
            .any(|f| f[1..] == [DISABLED_MARKER_FORMAT, SIT_TEXT, 5]));
        // Flags 1 and 4 were appended to the prefix, in order.
        assert_eq!(calls(&e, STRING_COPY)[0][1..], [0x100, FLAGS_PREFIX]);
        let appended: Vec<u32> = calls(&e, STRING_APPEND).iter().map(|c| c[2]).collect();
        assert_eq!(appended, vec![FLAG_TEXT_1, FLAG_TEXT_4]);
        assert!(formats.iter().any(|f| f[1..] == [0x6a6a]));
        let _ = pick;
    }

    fn cursor_world() -> (Engine, Ptr<InterfaceManager>) {
        let (mut e, m) = idle_world(2);
        e.set(m, InterfaceManager::pCursor, 0x7000);
        returns(&mut e, CONTROLS_GET, 0x2222);
        returns(&mut e, TILE_IMAGE_NODE, 0x5555);
        let translation = e.mem.alloc(0xc);
        returns(&mut e, NODE_TRANSLATION, translation);
        returns_st0(&mut e, GET_SCREEN_WIDTH, 1280.0);
        returns_st0(&mut e, GET_SCREEN_HEIGHT, 960.0);
        returns_st0(&mut e, GET_DESKTOP_WIDTH, 1920.0);
        returns_st0(&mut e, GET_DESKTOP_HEIGHT, 1080.0);
        returns_st0(&mut e, TILE_GET_VALUE, 2.0);
        e.mem.set_f64(CURSOR_TILT_SCALE, -0.008);
        // The mouse moved 10 right and 5 up.
        e.register(CONTROLS_QUERY_00A239E0, |_, a| Ret {
            eax: match a[1] {
                1 => 10,
                2 => (-5i32) as u32,
                _ => 0,
            },
            ..Ret::default()
        });
        (e, m)
    }

    /// Records the translation vectors written to the node.
    fn vector_recorder(e: &mut Engine) -> Rc<RefCell<Vec<[f32; 3]>>> {
        let seen = Rc::new(RefCell::new(vec![]));
        let log = seen.clone();
        e.register_double(NODE_SET_TRANSLATE_VECTOR, move |e, a| {
            log.borrow_mut()
                .push([e.mem.f32(a[1]), e.mem.f32(a[1] + 4), e.mem.f32(a[1] + 8)]);
            Ret::default()
        });
        seen
    }

    #[test]
    fn the_cursor_follows_the_mouse() {
        let (mut e, m) = cursor_world();
        let vectors = vector_recorder(&mut e);
        fn_007118d0(&mut e, m);
        // x = 1280 * 10 / 1280 and z = -(960 * -5 / 960), kept on screen.
        assert_eq!(vectors.borrow().len(), 1);
        assert_eq!(vectors.borrow()[0], [10.0, (2.0f64 * -0.008) as f32, 5.0]);
        assert_eq!(calls(&e, NODE_SET_TRANSLATE_VECTOR)[0][0], 0x5555);
        // The manager remembers the position and moves the real cursor.
        assert_eq!(e.mem.f32(m.addr() + 0x2c), 10.0);
        assert_eq!(e.mem.f32(m.addr() + 0x34), 5.0);
        assert_eq!(e.get(m, InterfaceManager::bMouseInMotion), 1);
        assert_eq!(e.mem.f32(m.addr() + 0x38), 975.0);
        assert_eq!(e.mem.f32(m.addr() + 0x40), 534.375);
        // The cursor node is shown through trait 0xfa3 and node updated.
        assert!(calls(&e, TILE_SET_INT).contains(&vec![0x7000, 0xfa3, 1]));
        assert_eq!(calls(&e, NODE_SET_FLAG), vec![vec![0x5555, 0]]);
        assert_eq!(calls(&e, NODE_UPDATE).len(), 1);
        // Same input again, from the same place: nothing moved.
        e.set(m, InterfaceManager::bMouseInMotion, 0);
        fn_007118d0(&mut e, m);
        assert_eq!(e.get(m, InterfaceManager::bMouseInMotion), 0);
    }

    #[test]
    fn the_cursor_stays_inside_the_screen() {
        let (mut e, m) = cursor_world();
        let vectors = vector_recorder(&mut e);
        e.register(CONTROLS_QUERY_00A239E0, |_, a| Ret {
            eax: if a[1] == 1 { 100_000 } else { 0 },
            ..Ret::default()
        });
        fn_007118d0(&mut e, m);
        // Half the 1280 screen minus 2.
        assert_eq!(vectors.borrow()[0][0], 638.0);
    }

    #[test]
    fn with_a_controller_and_no_console_the_cursor_does_not_move() {
        let (mut e, m) = cursor_world();
        returns(&mut e, HAS_360_CONTROLLER_GETTER, 1);
        returns(&mut e, CONSOLE_PRIMARY_DOWN, 0);
        fn_007118d0(&mut e, m);
        assert!(calls(&e, SCOPE_GUARD_BEGIN).is_empty());
        assert!(calls(&e, NODE_SET_TRANSLATE_VECTOR).is_empty());
        // With the mouse shown it moves even so.
        e.set(m, InterfaceManager::bShowMouse, 1);
        fn_007118d0(&mut e, m);
        assert_eq!(calls(&e, NODE_SET_TRANSLATE_VECTOR).len(), 1);
    }

    // ---- 00712450 .. 00714c20 ----

    /// An object whose virtual table answers `value` at each `(slot, value)`
    /// (the slots are doubles that record their argument words).
    fn fake_object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x200);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, table);
        for &(slot, value) in slots {
            let target = object.wrapping_add(0x4000_0000).wrapping_add(slot);
            e.mem.set_u32(table + slot, target);
            e.register_double(target, move |_, _| Ret {
                eax: value,
                ..Ret::default()
            });
        }
        object
    }

    /// The address a [`fake_object`] slot call is logged under.
    fn slot_target(object: u32, slot: u32) -> u32 {
        object.wrapping_add(0x4000_0000).wrapping_add(slot)
    }

    fn fade_world() -> (Engine, Ptr<InterfaceManager>) {
        let mut e = arithmetic_world();
        let m = manager(&mut e);
        e.mem.set_f64(ONE_DOUBLE, 1.0);
        e.mem.set_f64(TWO_FIFTY_FIVE, 255.0);
        e.mem.set_f64(FADE_MINIMUM, 0.0001f32 as f64);
        e.mem.set_f64(FADE_STOP_VALUE, 9000.0);
        returns(&mut e, PROPERTY_TYPE_THREE, 3);
        (e, m)
    }

    #[test]
    fn fading_a_null_node_does_nothing_but_the_guard() {
        let (mut e, m) = fade_world();
        interface_manager_recursive_fade(&mut e, m, 0, 1.0, 1.0);
        assert_eq!(
            calls(&e, SCOPE_GUARD_BEGIN)[0][1..],
            [0xd, 1, SOURCE_FILE, 0xd2b]
        );
        assert_eq!(calls(&e, SCOPE_GUARD_END).len(), 1);
        assert!(calls(&e, TILE_FROM_NODE).is_empty());
    }

    #[test]
    fn a_geometry_node_gets_the_fade_times_the_alpha_capped_at_the_alpha() {
        let (mut e, m) = fade_world();
        let node = fake_object(&mut e, &[(0xc, 0), (0x1c, 0x7000)]);
        returns_st0(&mut e, FLOAT_WRAPPER_00408840, 0.5);
        returns(&mut e, NODE_GET_PROPERTY, 0x7100);
        interface_manager_recursive_fade(&mut e, m, node, 0.5, 0.8);
        // min(0.5 * 0.8, 0.8) = 0.4, then max(0, 0.4).
        assert_eq!(
            calls(&e, PROPERTY_SET_FADE),
            vec![vec![0x7100, ((0.5f64 * 0.8f32 as f64) as f32).to_bits()]]
        );
        assert_eq!(calls(&e, NODE_GET_PROPERTY), vec![vec![0x7000, 3]]);
    }

    #[test]
    fn a_fade_below_the_minimum_gives_zero() {
        let (mut e, m) = fade_world();
        let node = fake_object(&mut e, &[(0xc, 0), (0x1c, 0x7000)]);
        returns_st0(&mut e, FLOAT_WRAPPER_00408840, 0.00005);
        returns(&mut e, NODE_GET_PROPERTY, 0x7100);
        interface_manager_recursive_fade(&mut e, m, node, 0.00005, 0.8);
        assert_eq!(calls(&e, PROPERTY_SET_FADE), vec![vec![0x7100, 0]]);
        assert!(calls(&e, FLOAT_MIN).is_empty());
    }

    #[test]
    fn no_property_means_nothing_to_set() {
        let (mut e, m) = fade_world();
        let node = fake_object(&mut e, &[(0xc, 0), (0x1c, 0x7000)]);
        returns_st0(&mut e, FLOAT_WRAPPER_00408840, 1.0);
        interface_manager_recursive_fade(&mut e, m, node, 1.0, 1.0);
        assert!(calls(&e, PROPERTY_SET_FADE).is_empty());
    }

    #[test]
    fn a_tile_with_trait_0xfab_only_shows_or_hides_itself() {
        let (mut e, m) = fade_world();
        let node = fake_object(&mut e, &[(0xc, 0x5000), (0x1c, 0x7000)]);
        returns(&mut e, TILE_FROM_NODE, 0x6000);
        e.register(TILE_IS_TRUE, |_, a| Ret {
            eax: (a[1] == 0xfab) as u32,
            ..Ret::default()
        });
        returns_st0(&mut e, FLOAT_WRAPPER_00408840, 1.0);
        interface_manager_recursive_fade(&mut e, m, node, 1.0, 1.0);
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![0x6000, 0xfa3, 1]]);
        returns_st0(&mut e, FLOAT_WRAPPER_00408840, 0.25);
        interface_manager_recursive_fade(&mut e, m, node, 0.25, 1.0);
        assert_eq!(calls(&e, TILE_SET_INT)[1], vec![0x6000, 0xfa3, 0]);
        // The geometry and children are left alone.
        assert!(calls(&e, slot_target(node, 0x1c)).is_empty());
        assert!(calls(&e, NODE_CHILD_COUNT).is_empty());
    }

    #[test]
    fn a_tile_replaces_the_alpha_with_its_own() {
        let (mut e, m) = fade_world();
        let node = fake_object(&mut e, &[(0xc, 0), (0x1c, 0x7000)]);
        // The node's own owner is null, so its parent chain gives the tile.
        let owner = fake_object(&mut e, &[(0xc, 0)]);
        let _ = owner;
        returns_st0(&mut e, FLOAT_WRAPPER_00408840, 1.0);
        returns(&mut e, NODE_GET_PROPERTY, 0x7100);
        interface_manager_recursive_fade(&mut e, m, node, 1.0, 0.8);
        // No tile: the alpha handed down (0.8) is used.
        assert_eq!(
            calls(&e, PROPERTY_SET_FADE),
            vec![vec![0x7100, 0.8f32.to_bits()]]
        );
    }

    #[test]
    fn the_owner_chain_is_followed_up_to_a_tile() {
        let (mut e, m) = fade_world();
        let node = fake_object(&mut e, &[(0xc, 0x5000), (0x1c, 0x7000)]);
        // 0x5000 has no tile, its parent 0x5100 does.
        e.register(TILE_FROM_NODE, |_, a| Ret {
            eax: if a[0] == 0x5100 { 0x6000 } else { 0 },
            ..Ret::default()
        });
        e.register(NODE_PARENT, |_, a| Ret {
            eax: if a[0] == 0x5000 { 0x5100 } else { 0 },
            ..Ret::default()
        });
        // Trait 0xfa9 is 127.5 (alpha 0.5); 0xfaa is 9000: no children.
        e.register(TILE_GET_VALUE, |_, a| Ret {
            st0: if a[1] == 0xfa9 { 127.5 } else { 9000.0 },
            ..Ret::default()
        });
        returns_st0(&mut e, FLOAT_WRAPPER_00408840, 1.0);
        returns(&mut e, NODE_GET_PROPERTY, 0x7100);
        interface_manager_recursive_fade(&mut e, m, node, 1.0, 0.8);
        assert_eq!(
            calls(&e, PROPERTY_SET_FADE),
            vec![vec![0x7100, 0.5f32.to_bits()]]
        );
        assert!(calls(&e, NODE_CHILD_COUNT).is_empty());
    }

    #[test]
    fn the_children_are_faded_in_turn() {
        let (mut e, m) = fade_world();
        let child_a = fake_object(&mut e, &[(0xc, 0), (0x1c, 0x7a00)]);
        let child_b = fake_object(&mut e, &[(0xc, 0), (0x1c, 0x7b00)]);
        let node = fake_object(&mut e, &[(0xc, 0x5000), (0x1c, 0)]);
        returns(&mut e, NODE_CHILD_COUNT, 2);
        e.register_double(NODE_CHILD_AT, move |_, a| Ret {
            eax: if a[1] == 0 { child_a } else { child_b },
            ..Ret::default()
        });
        returns_st0(&mut e, FLOAT_WRAPPER_00408840, 1.0);
        returns(&mut e, NODE_GET_PROPERTY, 0x7100);
        interface_manager_recursive_fade(&mut e, m, node, 1.0, 0.8);
        assert_eq!(
            calls(&e, NODE_GET_PROPERTY),
            vec![vec![0x7a00, 3], vec![0x7b00, 3]]
        );
        assert_eq!(
            calls(&e, NODE_CHILD_AT),
            vec![vec![0x5000, 0], vec![0x5000, 1]]
        );
    }

    /// A pick world: the pointer at (100, 50) on a 1920x1080 desktop, one
    /// result whose node maps to the tile `0x6000`.
    fn tile_pick_world() -> (Engine, Ptr<InterfaceManager>, u32) {
        let mut e = arithmetic_world();
        let m = manager(&mut e);
        returns(&mut e, CURSOR_ROOT_GETTER, 0xc0de);
        returns(&mut e, GET_FRAME_SCENE_NODE, 0x1111);
        returns(&mut e, SCENE_GRAPH_GET_CAMERA, 0x2222);
        returns_st0(&mut e, GET_DESKTOP_WIDTH, 1920.0);
        returns_st0(&mut e, GET_DESKTOP_HEIGHT, 1080.0);
        e.mem.set_f32(m.addr() + 0x38, 100.0);
        e.mem.set_f32(m.addr() + 0x40, 50.0);
        returns(&mut e, NI_PICK_PICK_OBJECTS, 1);
        returns(&mut e, NI_PICK_GET_RESULTS, 0x3333);
        returns(&mut e, RESULTS_COUNT, 1);
        returns(&mut e, RESULTS_GET, 0x4444);
        let hit = fake_object(&mut e, &[(0xc, 0x5000)]);
        returns(&mut e, RESULT_OBJECT, hit);
        returns(&mut e, TILE_FROM_NODE, 0x6000);
        returns(&mut e, TILE_IS_ACCEPTING_EVENTS, 1);
        let tile = fake_object(&mut e, &[(0x14, 1), (0xc, 0x200)]);
        returns(&mut e, TILE_FROM_NODE, tile);
        e.register(TILE_IS_TRUE, |_, a| Ret {
            eax: (a[1] == 0xfaf) as u32,
            ..Ret::default()
        });
        (e, m, tile)
    }

    #[test]
    fn a_ray_through_the_pointer_finds_the_tile_and_hides_the_cursor_meanwhile() {
        let (mut e, m, tile) = tile_pick_world();
        e.set(m, InterfaceManager::iCharHit, 7);
        returns(&mut e, TILE_GET_VALUE_Q, 1);
        assert_eq!(fn_007126c0(&mut e, m, 0), tile);
        assert_eq!(calls(&e, CAMERA_BUILD_PICK_RAY)[0][..3], [0x2222, 100, 50]);
        assert_eq!(
            calls(&e, NODE_SET_FLAG),
            vec![vec![0xc0de, 1], vec![0xc0de, 0]]
        );
        assert_eq!(calls(&e, NI_PICK_DESTRUCT).len(), 1);
        assert_eq!(calls(&e, SCOPE_GUARD_BEGIN)[0][4], 0xd5c);
        // Not a text tile: no character.
        assert_eq!(e.get(m, InterfaceManager::iCharHit), 0xffff);
        // The tile's hit test got the pointer position.
        assert_eq!(
            calls(&e, slot_target(tile, 0x14)),
            vec![vec![tile, 100.0f32.to_bits(), 50.0f32.to_bits()]]
        );
    }

    #[test]
    fn the_pointer_is_kept_inside_the_desktop() {
        let (mut e, m, _) = tile_pick_world();
        e.mem.set_f32(m.addr() + 0x38, -30.0);
        e.mem.set_f32(m.addr() + 0x40, 5000.0);
        returns(&mut e, TILE_GET_VALUE_Q, 1);
        fn_007126c0(&mut e, m, 0);
        assert_eq!(calls(&e, CAMERA_BUILD_PICK_RAY)[0][..3], [0x2222, 0, 1080]);
    }

    #[test]
    fn a_text_tile_records_the_character_hit_halved() {
        let (mut e, m, tile) = tile_pick_world();
        let text = fake_object(&mut e, &[(0x14, 1), (0xc, 0x387)]);
        let _ = tile;
        returns(&mut e, TILE_FROM_NODE, text);
        returns(&mut e, TILE_GET_VALUE_Q, 1);
        returns(&mut e, PICK_RESULT_HIT_WORD, 11);
        assert_eq!(fn_007126c0(&mut e, m, 0), text);
        assert_eq!(e.get(m, InterfaceManager::iCharHit), 5);
        assert_eq!(calls(&e, PICK_RESULT_HIT_WORD), vec![vec![0x4444]]);
    }

    #[test]
    fn a_tile_that_refuses_events_is_skipped() {
        let (mut e, m, _) = tile_pick_world();
        returns(&mut e, TILE_IS_ACCEPTING_EVENTS, 0);
        assert_eq!(fn_007126c0(&mut e, m, 0), 0);
        // The cursor is still shown again.
        assert_eq!(calls(&e, NODE_SET_FLAG)[1], vec![0xc0de, 0]);
        assert_eq!(calls(&e, TILE_IS_ACCEPTING_EVENTS)[0][0], m.addr());
    }

    #[test]
    fn a_tile_that_fails_its_hit_test_or_lacks_trait_0xfaf_is_skipped() {
        let (mut e, m, _) = tile_pick_world();
        let miss = fake_object(&mut e, &[(0x14, 0), (0xc, 0x200)]);
        returns(&mut e, TILE_FROM_NODE, miss);
        assert_eq!(fn_007126c0(&mut e, m, 0), 0);
        let (mut e, m, _) = tile_pick_world();
        e.register(TILE_IS_TRUE, |_, _| Ret::default());
        assert_eq!(fn_007126c0(&mut e, m, 0), 0);
    }

    #[test]
    fn without_trait_0xfaa_the_first_ancestor_that_has_it_is_returned() {
        let (mut e, m, tile) = tile_pick_world();
        let top = fake_object(&mut e, &[(0xc, 0x200)]);
        e.register_double(TILE_PARENT, move |_, a| Ret {
            eax: if a[0] == tile { 0x6100 } else { top },
            ..Ret::default()
        });
        e.register_double(TILE_GET_VALUE_Q, move |_, a| Ret {
            eax: (a[0] == top) as u32,
            ..Ret::default()
        });
        assert_eq!(fn_007126c0(&mut e, m, 0), top);
        // With `keep_exact_tile` the tile itself stays.
        assert_eq!(fn_007126c0(&mut e, m, 1), tile);
    }

    #[test]
    fn a_pick_rectangle_limits_the_hit() {
        let (mut e, m, tile) = tile_pick_world();
        returns(&mut e, TILE_GET_VALUE_Q, 1);
        returns(&mut e, TILE_IS_ACCEPTING_EVENTS, 1);
        e.register(TILE_IS_TRUE, |_, a| Ret {
            eax: (a[1] == 0xfaf || a[1] == 0xfae) as u32,
            ..Ret::default()
        });
        returns(&mut e, TILE_IMAGE_NODE, 0x8000);
        returns(&mut e, NODE_CHILD_POINTER, 1);
        returns(&mut e, NODE_CHILD_AT, 0x8100);
        returns(&mut e, NODE_GET_PROPERTY, 0x8200);
        returns(&mut e, PROPERTY_TYPE_THREE, 3);
        let rect = e.mem.alloc(0x10);
        for (i, v) in [10i32, 20, 200, 100].iter().enumerate() {
            e.mem.set_i32(rect + 4 * i as u32, *v);
        }
        returns(&mut e, PROPERTY_RECT, rect);
        returns_st0(&mut e, TILE_GET_VALUE, 0.0);
        e.mem.set_f64(PICK_SCALED_TILE_VALUE, 112.0);
        // (100, 50) is inside left 10, top 20, right 200, bottom 100.
        assert_eq!(fn_007126c0(&mut e, m, 0), tile);
        // Left of the rectangle, below it, right of it: skipped.
        for (x, y) in [
            (5.0f32, 50.0f32),
            (100.0, 100.0),
            (200.0, 50.0),
            (100.0, 19.0),
        ] {
            e.mem.set_f32(m.addr() + 0x38, x);
            e.mem.set_f32(m.addr() + 0x40, y);
            assert_eq!(fn_007126c0(&mut e, m, 0), 0, "({x}, {y})");
        }
        // All zero: no limit.
        for i in 0..4 {
            e.mem.set_i32(rect + 4 * i, 0);
        }
        e.mem.set_f32(m.addr() + 0x38, 5.0);
        assert_eq!(fn_007126c0(&mut e, m, 0), tile);
        // The tile's node was updated first.
        assert!(!calls(&e, NODE_SIBLING_UPDATE).is_empty());
    }

    #[test]
    fn a_scaled_menu_scales_the_pointer_by_the_managers_factor() {
        let (mut e, m, tile) = tile_pick_world();
        returns(&mut e, TILE_GET_VALUE_Q, 1);
        e.register(TILE_IS_TRUE, |_, a| Ret {
            eax: (a[1] == 0xfaf || a[1] == 0xfae) as u32,
            ..Ret::default()
        });
        returns(&mut e, TILE_IMAGE_NODE, 0x8000);
        returns(&mut e, NODE_CHILD_POINTER, 1);
        returns(&mut e, NODE_GET_PROPERTY, 0x8200);
        let rect = e.mem.alloc(0x10);
        for (i, v) in [10i32, 20, 150, 100].iter().enumerate() {
            e.mem.set_i32(rect + 4 * i as u32, *v);
        }
        returns(&mut e, PROPERTY_RECT, rect);
        // The menu's tile value is 112.0.
        returns_st0(&mut e, TILE_GET_VALUE, 112.0);
        e.mem.set_f64(PICK_SCALED_TILE_VALUE, 112.0);
        let manager_object = e.mem.alloc(0x600);
        e.mem.set_f32(manager_object + 0x4d0, 2.0);
        returns(&mut e, GET_MANAGER, manager_object);
        // 100 * 2 = 200 is past the right edge 150.
        assert_eq!(fn_007126c0(&mut e, m, 0), 0);
        e.mem.set_f32(manager_object + 0x4d0, 1.0);
        assert_eq!(fn_007126c0(&mut e, m, 0), tile);
    }

    #[test]
    fn a_pointer_over_a_rendered_menu_uses_the_menus_position_and_scales_the_rectangle() {
        let (mut e, m, tile) = tile_pick_world();
        returns(&mut e, TILE_GET_VALUE_Q, 1);
        // The setting byte holds 1.
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, 1);
        returns(&mut e, SETTING_BYTE, flag);
        e.set(m, InterfaceManager::bMouseOverRenderedMenu, 1);
        e.set(m, InterfaceManager::field_4ac, 321.0);
        e.set(m, InterfaceManager::field_4b0, 123.0);
        e.register(TILE_IS_TRUE, |_, a| Ret {
            eax: (a[1] == 0xfaf || a[1] == 0xfae) as u32,
            ..Ret::default()
        });
        returns(&mut e, TILE_IMAGE_NODE, 0x8000);
        returns(&mut e, NODE_CHILD_POINTER, 1);
        returns(&mut e, NODE_GET_PROPERTY, 0x8200);
        let rect = e.mem.alloc(0x10);
        for (i, v) in [100i32, 100, 400, 200].iter().enumerate() {
            e.mem.set_i32(rect + 4 * i as u32, *v);
        }
        returns(&mut e, PROPERTY_RECT, rect);
        returns_st0(&mut e, REAL_SCREEN_WIDTH, 1920.0);
        returns_st0(&mut e, SCREEN_WIDTH_CONSTANT, 960.0);
        e.mem.set_f64(PICK_SCALED_TILE_VALUE, 112.0);
        // The rectangle is doubled (1920 / 960) to 200 200 800 400: the
        // ray at (321, 123) is above its top edge 200 and the hit refused.
        assert_eq!(fn_007126c0(&mut e, m, 0), 0);
        assert_eq!(calls(&e, CAMERA_BUILD_PICK_RAY)[0][..3], [0x2222, 321, 123]);
        e.set(m, InterfaceManager::field_4b0, 250.0);
        assert_eq!(fn_007126c0(&mut e, m, 0), tile);
    }

    #[test]
    fn a_topmost_rendered_menu_refuses_the_pick_before_the_cursor_is_restored() {
        let (mut e, m, _) = tile_pick_world();
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, 1);
        returns(&mut e, SETTING_BYTE, flag);
        returns(&mut e, IS_CURRENT_RENDERED_MENU_TOPMOST, 1);
        assert_eq!(fn_007126c0(&mut e, m, 0), 0);
        // Only the hide happened, the pick was destroyed.
        assert_eq!(calls(&e, NODE_SET_FLAG), vec![vec![0xc0de, 1]]);
        assert_eq!(calls(&e, NI_PICK_DESTRUCT).len(), 1);
        assert!(calls(&e, CAMERA_BUILD_PICK_RAY).is_empty());
        assert_eq!(calls(&e, SCOPE_GUARD_END).len(), 1);
    }

    #[test]
    fn the_rendered_menu_is_told_and_its_scene_is_the_pick_root() {
        let (mut e, m, tile) = tile_pick_world();
        returns(&mut e, TILE_GET_VALUE_Q, 1);
        let menu = fake_object(&mut e, &[(0x1c, 0), (0x20, 0)]);
        e.set(m, InterfaceManager::pCurrentRenderedMenu, menu);
        returns(&mut e, MENU_FLAG_QUERY_004A4040, 1);
        returns(&mut e, GET_ENTER_STACK_TOP, 0x3ef);
        returns(&mut e, TILE_GET_MENU_BY_CLASS, 0x9100);
        // The image node of the menu tile is the root of the pick.
        e.register(TILE_IMAGE_NODE, |_, a| Ret {
            eax: if a[0] == 0x9100 { 0x9200 } else { 0 },
            ..Ret::default()
        });
        assert_eq!(fn_007126c0(&mut e, m, 0), tile);
        assert_eq!(
            calls(&e, NI_PICK_SET_ROOT),
            vec![vec![calls(&e, NI_PICK_SET_ROOT)[0][0], 0x9200]]
        );
        assert_eq!(calls(&e, slot_target(menu, 0x1c)).len(), 1);
        assert_eq!(calls(&e, slot_target(menu, 0x20)).len(), 1);
        // Without the flag the frame scene node is the root.
        let (mut e, m, _) = tile_pick_world();
        let menu = fake_object(&mut e, &[(0x1c, 0), (0x20, 0)]);
        e.set(m, InterfaceManager::pCurrentRenderedMenu, menu);
        fn_007126c0(&mut e, m, 0);
        assert_eq!(calls(&e, NI_PICK_SET_ROOT)[0][1], 0x1111);
    }

    #[test]
    fn the_custom_viewport_rectangle_is_set_on_the_camera_before_the_ray() {
        let (mut e, m, _) = tile_pick_world();
        e.set_global(VIEWPORT_RECT_ENABLED, 1u8);
        let camera = e.mem.alloc(0x200);
        returns(&mut e, SCENE_GRAPH_GET_CAMERA, camera);
        // The constructor double leaves a rectangle the camera copies.
        let rect = e.mem.alloc(0x10);
        for (i, v) in [0.25f32, 0.75, 0.5, 0.125].iter().enumerate() {
            e.mem.set_f32(rect + 4 * i as u32, *v);
        }
        returns(&mut e, RECT_CONSTRUCT, rect);
        returns(&mut e, TILE_GET_VALUE_Q, 1);
        fn_007126c0(&mut e, m, 0);
        assert_eq!(e.mem.f32(camera + 0x100), 0.25);
        assert_eq!(e.mem.f32(camera + 0x10c), 0.125);
        assert_eq!(
            calls(&e, RECT_CONSTRUCT)[0][1..],
            [0, 0x3f80_0000, 0x3f80_0000, 0]
        );
    }

    #[test]
    fn the_viewport_copy_moves_four_words_to_0x100() {
        let mut e = world();
        let source = e.mem.alloc(0x10);
        let target = e.mem.alloc(0x200);
        for i in 0..4 {
            e.mem.set_u32(source + 4 * i, 0x100 + i);
        }
        fn_00712e60(&mut e, Ptr::new(target), source);
        for i in 0..4 {
            assert_eq!(e.mem.u32(target + 0x100 + 4 * i), 0x100 + i);
        }
        assert_eq!(e.mem.u32(target + 0xfc), 0);
    }

    #[test]
    fn the_matrix_row_setter_writes_three_floats_at_row_times_0xc() {
        let mut e = world();
        let matrix = e.mem.alloc(0x24);
        fn_007133b0(&mut e, Ptr::new(matrix), 2, 1.5, 2.5, 3.5);
        assert_eq!(e.mem.f32(matrix + 0x18), 1.5);
        assert_eq!(e.mem.f32(matrix + 0x1c), 2.5);
        assert_eq!(e.mem.f32(matrix + 0x20), 3.5);
        assert_eq!(e.mem.f32(matrix), 0.0);
    }

    /// A scene graph creation world: a 1920x1080 renderer, the camera object
    /// `camera`, and the graph `graph` with a slot `0xdc` recorder.
    fn scene_graph_world() -> (Engine, Ptr<InterfaceManager>, u32, u32) {
        let mut e = arithmetic_world();
        let m = manager(&mut e);
        allocator(&mut e, NI_ALLOC);
        echo(&mut e, PASS_THROUGH);
        echo(&mut e, NI_NODE_CONSTRUCT);
        echo(&mut e, NI_POINT3_CONSTRUCT);
        returns(&mut e, RENDERER_GET, 0x8000);
        returns(&mut e, RENDERER_WIDTH, 1920);
        returns(&mut e, RENDERER_HEIGHT, 1080);
        e.mem.set_f64(ASPECT_DIVISOR, 1.3333334);
        e.mem.set_f64(CURSOR_HORIZONTAL_SCALE, 1280.0);
        e.mem.set_f64(HALF, 0.5);
        e.mem.set_f32(FRUSTUM_TOP, 960.0);
        e.mem.set_f32(FRUSTUM_FAR, 10000.0);
        e.mem.set_f32(FOV_3D, 35.0);
        e.set_global(FADER_MANAGER, 0xfade);
        e.set(m, InterfaceManager::fOneToOneDistance, 800.0);
        let camera = e.mem.alloc(0x200);
        returns(&mut e, SCENE_GRAPH_GET_CAMERA, camera);
        let graph = fake_object(&mut e, &[(0xdc, 0)]);
        (e, m, camera, graph)
    }

    #[test]
    fn the_2d_scene_graph_gets_its_camera_roots_and_fader_registration() {
        let (mut e, m, camera, graph) = scene_graph_world();
        // The matrix rows and the frustum are copied when they are used.
        let rows = Rc::new(RefCell::new(vec![]));
        let seen = rows.clone();
        e.register_double(CAMERA_SET_ROTATION, move |e, a| {
            let row: Vec<f32> = (0..9).map(|i| e.mem.f32(a[1] + 4 * i)).collect();
            seen.borrow_mut().push((a[0], row));
            Ret::default()
        });
        let frustums = Rc::new(RefCell::new(vec![]));
        let seen = frustums.clone();
        e.register_double(CAMERA_SET_FRUSTUM, move |e, a| {
            let words: Vec<u32> = (0..7).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            let ortho = e.mem.u8(a[1] + 0x18);
            seen.borrow_mut().push((a[0], words, ortho));
            Ret::default()
        });
        let vectors = vector_recorder(&mut e);
        let result = interface_manager_create_scene_graph(&mut e, m, graph, 0x1234, 0);
        assert_eq!(result, graph);
        assert_eq!(
            calls(&e, SCOPE_GUARD_BEGIN)[0][1..],
            [0xd, 1, SOURCE_FILE, 0xe11]
        );
        // Rows (0 0 1) (1 0 0) (0 1 0) on the graph's camera.
        assert_eq!(
            rows.borrow().as_slice(),
            &[(camera, vec![0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0])]
        );
        // The aspect (1920/1080 over 4/3) is kept in the manager.
        let aspect = ((1920.0f64 / 1080.0) as f32 as f64 / 1.3333334) as f32;
        assert_eq!(e.get(m, InterfaceManager::field_4d0), aspect);
        // An orthographic frustum, `aspect * 1280` wide and 960 high.
        let right = (aspect as f64 * 1280.0) as f32;
        let seen = frustums.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0, camera);
        assert_eq!(
            seen[0].1,
            vec![
                0,
                right.to_bits(),
                960.0f32.to_bits(),
                0,
                0,
                10000.0f32.to_bits(),
                seen[0].1[6]
            ]
        );
        assert_eq!(seen[0].2, 1);
        // The camera is moved by minus half of the frustum.
        assert_eq!(
            calls(&e, NI_POINT3_CONSTRUCT)[0][1..],
            [
                (((-(right as f64)) * 0.5) as f32).to_bits(),
                0,
                ((-960.0f64 * 0.5) as f32).to_bits()
            ]
        );
        assert_eq!(vectors.borrow().len(), 3);
        // Two roots, named and attached with flag 1, then moved down by the
        // one-to-one distance; the fader is told before and after.
        let main_root = e.get(m, InterfaceManager::pInterfaceRoot);
        let cursor_root = e.get(m, InterfaceManager::pCursorRoot);
        assert_ne!(main_root, 0);
        assert_ne!(cursor_root, 0);
        assert_ne!(main_root, cursor_root);
        assert_eq!(
            calls(&e, FIXED_STRING_CONSTRUCT)
                .iter()
                .map(|c| c[1])
                .collect::<Vec<_>>(),
            vec![MAIN_ROOT_NAME, CURSOR_ROOT_NAME]
        );
        assert_eq!(
            calls(&e, slot_target(graph, 0xdc)),
            vec![vec![graph, main_root, 1], vec![graph, cursor_root, 1]]
        );
        assert_eq!(
            calls(&e, FADER_ADD_ROOT),
            vec![
                vec![0xfade, graph, 800.0f32.to_bits(), 1],
                vec![0xfade, graph, 800.0f32.to_bits(), 0]
            ]
        );
        assert_eq!(
            order(&e, &[FADER_ADD_ROOT, NI_NODE_CONSTRUCT]),
            vec![
                FADER_ADD_ROOT,
                NI_NODE_CONSTRUCT,
                NI_NODE_CONSTRUCT,
                FADER_ADD_ROOT
            ]
        );
        assert!(calls(&e, SCENE_GRAPH_SET_CAMERA_FOV).is_empty());
    }

    #[test]
    fn a_missing_graph_is_built_from_the_name_and_the_3d_one_gets_a_field_of_view() {
        let (mut e, m, camera, _) = scene_graph_world();
        echo(&mut e, SCENE_GRAPH_CONSTRUCT);
        let graph = interface_manager_create_scene_graph(&mut e, m, 0, 0x1234, 1);
        assert_ne!(graph, 0);
        assert_eq!(calls(&e, NI_ALLOC)[0], vec![0xc0]);
        assert_eq!(calls(&e, PASS_THROUGH), vec![vec![0x1234, 0]]);
        assert_eq!(
            calls(&e, SCENE_GRAPH_CONSTRUCT),
            vec![vec![graph, 0x1234, 0, 0]]
        );
        // No roots and no fader registration for the 3D graph.
        assert!(calls(&e, FADER_ADD_ROOT).is_empty());
        assert!(calls(&e, NI_NODE_CONSTRUCT).is_empty());
        assert_eq!(e.get(m, InterfaceManager::pInterfaceRoot), 0);
        assert_eq!(
            calls(&e, SCENE_GRAPH_SET_CAMERA_FOV),
            vec![vec![graph, 35.0f32.to_bits(), 0, 0, 0]]
        );
        let _ = camera;
    }

    #[test]
    fn a_failed_graph_allocation_still_runs_the_camera_code_on_null() {
        let (mut e, m, _, _) = scene_graph_world();
        returns(&mut e, NI_ALLOC, 0);
        let graph = interface_manager_create_scene_graph(&mut e, m, 0, 0x1234, 1);
        assert_eq!(graph, 0);
        assert!(calls(&e, SCENE_GRAPH_CONSTRUCT).is_empty());
    }

    #[test]
    fn the_pipboy_manager_is_replaced() {
        let mut e = world();
        let m = manager(&mut e);
        allocator(&mut e, OPERATOR_NEW);
        let old = fake_object(&mut e, &[(0, 0)]);
        e.set(m, InterfaceManager::pPipboy, old);
        let fresh = fake_object(&mut e, &[(0x24, 0)]);
        returns(&mut e, PIPBOY_MANAGER_CONSTRUCT, fresh);
        fn_007133f0(&mut e, m);
        // The old one got its virtual destructor with flag 1.
        assert_eq!(calls(&e, slot_target(old, 0)), vec![vec![old, 1]]);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x170]]);
        assert_eq!(e.get(m, InterfaceManager::pPipboy), fresh);
        assert_eq!(calls(&e, slot_target(fresh, 0x24)), vec![vec![fresh]]);
        // Without an old one nothing is deleted.
        e.set(m, InterfaceManager::pPipboy, 0);
        let before = calls(&e, slot_target(old, 0)).len();
        fn_007133f0(&mut e, m);
        assert_eq!(calls(&e, slot_target(old, 0)).len(), before);
    }

    #[test]
    fn the_deferred_queue_is_emptied_under_the_tile_lock() {
        let mut e = world();
        let first = fake_object(&mut e, &[(0, 0)]);
        let second = fake_object(&mut e, &[(0, 0)]);
        let queue = Rc::new(RefCell::new(vec![first, 0, second]));
        let pending = queue.clone();
        e.register_double(COLLECTION_IS_EMPTY, move |_, _| Ret {
            eax: pending.borrow().is_empty() as u32,
            ..Ret::default()
        });
        e.register_double(DEFERRED_QUEUE_POP, move |_, _| Ret {
            eax: queue.borrow_mut().remove(0),
            ..Ret::default()
        });
        fn_00713c00(&mut e);
        assert_eq!(calls(&e, LOCK_ENTER), vec![vec![TILE_LOCK, 0]]);
        assert_eq!(calls(&e, LOCK_LEAVE), vec![vec![TILE_LOCK]]);
        assert_eq!(calls(&e, slot_target(first, 0)), vec![vec![first, 1]]);
        assert_eq!(calls(&e, slot_target(second, 0)), vec![vec![second, 1]]);
        assert_eq!(calls(&e, DEFERRED_QUEUE_POP), vec![vec![DEFERRED_QUEUE]; 3]);
    }

    fn tile_update_world() -> (Engine, Ptr<InterfaceManager>) {
        let mut e = world();
        let m = manager(&mut e);
        // The setting holder answers a pointer to an int.
        let setting = e.mem.alloc(4);
        returns(&mut e, GET_SETTING_VALUE, setting);
        returns(&mut e, COLLECTION_IS_EMPTY, 1);
        (e, m)
    }

    #[test]
    fn updating_the_tiles_updates_them_all_and_brackets_the_flag() {
        let (mut e, m) = tile_update_world();
        e.set_global(TILE_UPDATE_DEPTH, 9u32);
        fn_00713c70(&mut e, m);
        assert_eq!(e.global::<u8>(TILES_UPDATING), 0);
        assert_eq!(e.global::<u32>(TILE_UPDATE_DEPTH), 0);
        assert_eq!(calls(&e, TILE_UPDATE_ALL), vec![vec![0]]);
        assert_eq!(calls(&e, TILE_UPDATE_FADE_CONTROLS).len(), 1);
        assert_eq!(calls(&e, TILE_UPDATE_PREPARE).len(), 1);
        // The flush happened before the prepare, and the setting 0 did not
        // wait on the pool.
        assert_eq!(
            order(
                &e,
                &[
                    LOCK_ENTER,
                    TILE_UPDATE_PREPARE,
                    TILE_UPDATE_ALL,
                    TILE_UPDATE_FADE_CONTROLS
                ]
            ),
            vec![
                LOCK_ENTER,
                TILE_UPDATE_PREPARE,
                TILE_UPDATE_ALL,
                TILE_UPDATE_FADE_CONTROLS
            ]
        );
        assert!(calls(&e, SEMAPHORE_POOL_WAIT).is_empty());
        // The update array was empty: no second lock.
        assert_eq!(calls(&e, LOCK_ENTER).len(), 1);
        assert_eq!(e.mem.u8(m.addr() + 0xdc), 0);
    }

    #[test]
    fn the_flag_is_set_while_the_tiles_update() {
        let (mut e, m) = tile_update_world();
        let seen = Rc::new(RefCell::new(vec![]));
        let log = seen.clone();
        e.register_double(TILE_UPDATE_PREPARE, move |e, _| {
            log.borrow_mut().push(e.global::<u8>(TILES_UPDATING));
            Ret::default()
        });
        e.mem.set_u8(m.addr() + 0xdc, 1);
        fn_00713c70(&mut e, m);
        assert_eq!(seen.borrow().as_slice(), &[1]);
        assert_eq!(e.mem.u8(m.addr() + 0xdc), 0);
    }

    #[test]
    fn a_big_setting_waits_on_the_pool_when_it_is_in_use() {
        let (mut e, m) = tile_update_world();
        let setting = e.call(GET_SETTING_VALUE, &args![0u32]).u32();
        e.mem.set_i32(setting, 2);
        e.set_global(SEMAPHORE_POOL_IN_USE, 0u8);
        fn_00713c70(&mut e, m);
        assert!(calls(&e, SEMAPHORE_POOL_WAIT).is_empty());
        e.set_global(SEMAPHORE_POOL_IN_USE, 1u8);
        fn_00713c70(&mut e, m);
        assert_eq!(
            calls(&e, SEMAPHORE_POOL_WAIT),
            vec![vec![SEMAPHORE_POOL, 0, 1]]
        );
        // A setting of exactly 1 does not wait.
        e.mem.set_i32(setting, 1);
        fn_00713c70(&mut e, m);
        assert_eq!(calls(&e, SEMAPHORE_POOL_WAIT).len(), 1);
    }

    #[test]
    fn with_the_loading_menu_open_only_its_children_update() {
        let (mut e, m) = tile_update_world();
        returns(&mut e, IS_IN_GAME_LOADING_MENU_OPEN, 1);
        returns(&mut e, TILE_GET_MENU_BY_CLASS, 0x9100);
        fn_00713c70(&mut e, m);
        assert_eq!(calls(&e, TILE_GET_MENU_BY_CLASS), vec![vec![0x3ef]]);
        assert_eq!(calls(&e, TILE_UPDATE_CHILDREN), vec![vec![0x9100, 0]]);
        assert!(calls(&e, TILE_UPDATE_ALL).is_empty());
        // With the XUI up everything updates anyway.
        returns(&mut e, XUI_IS_UP, 1);
        fn_00713c70(&mut e, m);
        assert_eq!(calls(&e, TILE_UPDATE_ALL).len(), 1);
    }

    #[test]
    fn a_non_empty_update_array_is_processed_under_the_update_lock() {
        let (mut e, m) = tile_update_world();
        // Only the update array has entries; the deferred queue is empty.
        e.register(COLLECTION_IS_EMPTY, |_, a| Ret {
            eax: (a[0] != UPDATE_ARRAY) as u32,
            ..Ret::default()
        });
        returns(&mut e, WORD_AT_8, 0);
        fn_00713c70(&mut e, m);
        assert_eq!(
            calls(&e, LOCK_ENTER),
            vec![vec![TILE_LOCK, 0], vec![UPDATE_LOCK, 0]]
        );
        assert_eq!(
            calls(&e, LOCK_LEAVE),
            vec![vec![TILE_LOCK], vec![UPDATE_LOCK]]
        );
    }

    #[test]
    fn the_small_getters_return_their_constants() {
        let mut e = world();
        assert_eq!(fn_00713d70(&mut e), 0x011f_3330);
        assert_eq!(fn_00713d80(&mut e), 0x011d_fa50);
        e.set_global(SEMAPHORE_POOL_IN_USE, 5u8);
        assert_eq!(fn_00713d90(&mut e), 5);
        e.set_global(TILE_UPDATE_DEPTH, 3u32);
        fn_00713d60(&mut e);
        assert_eq!(e.global::<u32>(TILE_UPDATE_DEPTH), 0);
        let tile = e.mem.alloc(0x40);
        e.mem.set_u8(tile + 0x34, 9);
        fn_00713ee0(&mut e, Ptr::new(tile));
        assert_eq!(e.mem.u8(tile + 0x34), 0);
    }

    #[test]
    fn a_tile_is_added_to_the_update_array_once() {
        let mut e = world();
        let m = manager(&mut e);
        let items = Rc::new(RefCell::new(vec![]));
        let known = items.clone();
        e.register_double(ARRAY_FIND, move |e, a| Ret {
            eax: known.borrow().contains(&e.mem.u32(a[1])) as u32,
            ..Ret::default()
        });
        let added = items.clone();
        e.register_double(ARRAY_ADD, move |e, a| {
            added.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        interface_manager_add_tile_to_update_list(&mut e, m, 0);
        interface_manager_add_tile_to_update_list(&mut e, m, 0x6000);
        interface_manager_add_tile_to_update_list(&mut e, m, 0x6000);
        interface_manager_add_tile_to_update_list(&mut e, m, 0x6100);
        assert_eq!(items.borrow().as_slice(), &[0x6000, 0x6100]);
        // Null was not even searched for.
        assert_eq!(calls(&e, ARRAY_FIND).len(), 3);
        assert_eq!(calls(&e, ARRAY_FIND)[0][0], UPDATE_ARRAY);
    }

    #[test]
    fn a_tile_is_removed_from_the_update_array_when_found() {
        let mut e = world();
        let m = manager(&mut e);
        e.register_double(ARRAY_FIND_INDEX, |e, a| Ret {
            eax: if e.mem.u32(a[1]) == 0x6000 {
                4
            } else {
                u32::MAX
            },
            ..Ret::default()
        });
        fn_00713de0(&mut e, m, 0x6000);
        fn_00713de0(&mut e, m, 0x6100);
        assert_eq!(calls(&e, ARRAY_REMOVE_AT), vec![vec![UPDATE_ARRAY, 4, 1]]);
        let search = calls(&e, ARRAY_FIND_INDEX);
        assert_eq!(search[0][0], UPDATE_ARRAY);
        assert_eq!(search[0][2..], [0, ARRAY_COMPARE]);
    }

    fn update_array_world(tiles: &[u32]) -> (Engine, Ptr<InterfaceManager>) {
        let mut e = world();
        let m = manager(&mut e);
        let slots: Vec<u32> = tiles
            .iter()
            .map(|t| {
                let slot = e.mem.alloc(4);
                e.mem.set_u32(slot, *t);
                slot
            })
            .collect();
        let count = tiles.len() as u32;
        returns(&mut e, WORD_AT_8, count);
        e.register_double(ARRAY_ELEMENT, move |_, a| Ret {
            eax: slots[a[1] as usize],
            ..Ret::default()
        });
        e.set_global(OWNER_OBJECT, 0x5150);
        returns(&mut e, CURRENT_THREAD_ID, 77);
        (e, m)
    }

    #[test]
    fn every_tile_of_the_array_is_updated_and_its_flag_cleared() {
        let tile_a = 0x2000_5000u32;
        let tile_b = 0x2000_5100u32;
        let (mut e, m) = update_array_world(&[tile_a, 0, tile_b]);
        e.map(tile_a, 0x100);
        e.map(tile_b, 0x100);
        e.map(0, 0x1000);
        e.mem.set_u8(tile_a + 0x34, 1);
        e.mem.set_u8(tile_b + 0x34, 1);
        e.register(TILE_IMAGE_NODE, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        fn_00713e20(&mut e, m);
        // The two tiles with image nodes were updated with a zero time.
        assert_eq!(calls(&e, NODE_UPDATE).len(), 2);
        assert_eq!(calls(&e, NODE_UPDATE)[0][0], tile_a + 1);
        assert_eq!(calls(&e, NODE_UPDATE)[1][0], tile_b + 1);
        assert_eq!(
            calls(&e, NI_UPDATE_DATA_CONSTRUCT)[0][1..],
            [0.0f32.to_bits(), 0, 0]
        );
        assert_eq!(e.mem.u8(tile_a + 0x34), 0);
        assert_eq!(e.mem.u8(tile_b + 0x34), 0);
        assert_eq!(
            calls(&e, TILE_IMAGE_NODE).len(),
            2,
            "no node for the null tile"
        );
    }

    #[test]
    fn the_array_is_cleared_only_by_the_owner_thread() {
        let (mut e, m) = update_array_world(&[]);
        returns(&mut e, OWNER_THREAD_ID, 76);
        fn_00713e20(&mut e, m);
        assert!(calls(&e, ARRAY_CLEAR).is_empty());
        returns(&mut e, OWNER_THREAD_ID, 77);
        fn_00713e20(&mut e, m);
        assert_eq!(calls(&e, ARRAY_CLEAR), vec![vec![UPDATE_ARRAY, 1]]);
        assert_eq!(calls(&e, OWNER_THREAD_ID)[0], vec![0x5150]);
    }

    /// Makes the three settings (`011d8ba0`, `011db2cc`, `011d8ab0`) answer
    /// the given flags.
    fn settings(e: &mut Engine, a: bool, b: bool, c: bool) {
        let cells = [e.mem.alloc(4), e.mem.alloc(4), e.mem.alloc(4)];
        for (cell, flag) in cells.iter().zip([a, b, c]) {
            e.mem.set_u8(*cell, flag as u8);
        }
        e.register_double(SETTING_BYTE, move |_, args| Ret {
            eax: match args[0] {
                SETTING_HOLDER_011D8BA0 => cells[0],
                SETTING_HOLDER_011DB2CC => cells[1],
                SETTING_HOLDER_011D8AB0 => cells[2],
                other => panic!("unexpected holder {other:x}"),
            },
            ..Ret::default()
        });
    }

    /// A world for the menu accumulation: scene `0x1111`, camera `0x2222`
    /// (an object, so its viewport can be written), accumulator `0x3333`,
    /// the rendering system `0xe500`, the renderer with equal or different
    /// slot `0xc8` / `0xcc` answers.
    fn accumulate_world() -> (Engine, Ptr<InterfaceManager>, u32) {
        let mut e = arithmetic_world();
        let m = manager(&mut e);
        returns(&mut e, GET_FRAME_SCENE_NODE, 0x1111);
        let camera = e.mem.alloc(0x200);
        returns(&mut e, SCENE_GRAPH_GET_CAMERA, camera);
        returns(&mut e, NI_POINTER_GET, 0x3333);
        returns(&mut e, RENDERING_SYSTEM, 0xe500);
        // The deferred queue and the update array are empty.
        returns(&mut e, COLLECTION_IS_EMPTY, 1);
        let setting = e.mem.alloc(4);
        returns(&mut e, GET_SETTING_VALUE, setting);
        let renderer = fake_object(&mut e, &[(0xc8, 7), (0xcc, 7)]);
        returns(&mut e, RENDERER_GET, renderer);
        for (i, v) in [0.0f32, 1.0, 1.0, 0.0].iter().enumerate() {
            e.mem.set_f32(VIEWPORT_RECT + 4 * i as u32, *v);
        }
        (e, m, camera)
    }

    /// The addresses of the accumulation steps, in the order they ran.
    fn accumulation_steps(e: &Engine) -> Vec<u32> {
        order(
            e,
            &[
                CULLING_PUSH,
                CULLING_POP,
                RESTORE_MENU_ELEMENTS,
                ISOLATE_MENU_ELEMENTS,
                ACCUMULATE_SCENE,
                ACCUMULATE_FINISH,
                RENDERING_SET_STAGE,
                RENDERING_WAIT_STAGE,
                RENDERER_SET_CAMERA,
            ],
        )
    }

    #[test]
    fn with_neither_setting_one_plain_pass_is_made() {
        let (mut e, m, _) = accumulate_world();
        settings(&mut e, false, false, false);
        fn_007134d0(&mut e, m, 0xc000, 0);
        assert_eq!(calls(&e, CULLING_PUSH), vec![vec![0xc000, 1]]);
        assert_eq!(
            accumulation_steps(&e),
            vec![
                CULLING_PUSH,
                RESTORE_MENU_ELEMENTS,
                ACCUMULATE_SCENE,
                ACCUMULATE_FINISH,
                CULLING_POP
            ]
        );
        assert_eq!(
            calls(&e, CULLING_SET_ACCUMULATOR),
            vec![vec![0xc000, 0x3333], vec![0xc000, 0]]
        );
        assert_eq!(calls(&e, ACCUMULATE_SCENE)[0][1..], [0x1111, 0xc000]);
        assert_eq!(calls(&e, ACCUMULATE_FINISH)[0][1..], [0x3333, 0]);
        assert_eq!(calls(&e, CULLING_POP), vec![vec![0xc000]]);
    }

    #[test]
    fn a_multithreaded_click_waits_for_the_stage_instead_of_accumulating() {
        let (mut e, m, _) = accumulate_world();
        settings(&mut e, false, false, false);
        e.set(m, InterfaceManager::bClickMultithreaded, 1);
        fn_007134d0(&mut e, m, 0xc000, 0);
        assert_eq!(
            accumulation_steps(&e),
            vec![
                CULLING_PUSH,
                RENDERING_WAIT_STAGE,
                RESTORE_MENU_ELEMENTS,
                ACCUMULATE_FINISH,
                CULLING_POP
            ]
        );
        assert_eq!(calls(&e, RENDERING_WAIT_STAGE), vec![vec![0xe500, 1, 0x17]]);
    }

    #[test]
    fn setting_b_alone_makes_an_isolated_pass_then_a_plain_isolated_one() {
        let (mut e, m, camera) = accumulate_world();
        settings(&mut e, false, true, false);
        fn_007134d0(&mut e, m, 0xc000, 0);
        assert_eq!(
            accumulation_steps(&e),
            vec![
                CULLING_PUSH,
                ISOLATE_MENU_ELEMENTS,
                RENDERER_SET_CAMERA,
                ACCUMULATE_SCENE,
                ACCUMULATE_FINISH,
                RESTORE_MENU_ELEMENTS,
                ISOLATE_MENU_ELEMENTS,
                RENDERER_SET_CAMERA,
                ACCUMULATE_SCENE,
                ACCUMULATE_FINISH,
                RESTORE_MENU_ELEMENTS,
                CULLING_POP
            ]
        );
        assert_eq!(
            calls(&e, ISOLATE_MENU_ELEMENTS),
            vec![vec![1, 0], vec![0, 0]]
        );
        let renderer = e.call(RENDERER_GET, &args![]).u32();
        assert_eq!(
            calls(&e, RENDERER_SET_CAMERA),
            vec![vec![renderer, camera]; 2]
        );
    }

    #[test]
    fn setting_a_alone_resets_the_viewport_when_the_renderer_has_no_target() {
        let (mut e, m, camera) = accumulate_world();
        settings(&mut e, true, false, false);
        let renderer = fake_object(&mut e, &[(0xc8, 7), (0xcc, 0)]);
        returns(&mut e, RENDERER_GET, renderer);
        e.mem.set_f32(VIEWPORT_RECT, 0.5);
        fn_007134d0(&mut e, m, 0xc000, 0);
        assert_eq!(calls(&e, ISOLATE_MENU_ELEMENTS), vec![vec![1, 1]]);
        assert_eq!(e.mem.f32(camera + 0x100), 0.5);
        // With a target the viewport stays.
        let renderer = fake_object(&mut e, &[(0xc8, 7), (0xcc, 9)]);
        returns(&mut e, RENDERER_GET, renderer);
        e.mem.set_f32(VIEWPORT_RECT, 0.25);
        fn_007134d0(&mut e, m, 0xc000, 0);
        assert_eq!(e.mem.f32(camera + 0x100), 0.5);
    }

    #[test]
    fn both_settings_show_the_menus_during_the_pass_and_hide_them_after() {
        let (mut e, m, camera) = accumulate_world();
        settings(&mut e, true, true, false);
        // Menus 0x421 and 0x40c exist (image nodes 0x8001 / 0x8002, hidden);
        // 0x41f has an image node that is already shown.
        e.register(TILE_GET_MENU_BY_CLASS, |_, a| Ret {
            eax: a[0] + 0x1000,
            ..Ret::default()
        });
        e.register(TILE_IMAGE_NODE, |_, a| Ret {
            eax: a[0] + 0x7000,
            ..Ret::default()
        });
        let flagged = Rc::new(RefCell::new(vec![0x41f + 0x1000 + 0x7000]));
        let tested = flagged.clone();
        e.register_double(NODE_FLAG_TEST_00456610, move |_, a| Ret {
            eax: tested.borrow().contains(&a[0]) as u32,
            ..Ret::default()
        });
        e.register_double(NODE_SET_FLAG, move |_, a| {
            let mut nodes = flagged.borrow_mut();
            nodes.retain(|node| *node != a[0]);
            if a[1] != 0 {
                nodes.push(a[0]);
            }
            Ret::default()
        });
        fn_007134d0(&mut e, m, 0xc000, 0);
        let first = 0x421 + 0x1000 + 0x7000;
        let second = 0x40c + 0x1000 + 0x7000;
        assert_eq!(
            calls(&e, NODE_SET_FLAG),
            vec![
                vec![first, 1],
                vec![second, 1],
                vec![first, 0],
                vec![second, 0]
            ]
        );
        assert_eq!(
            accumulation_steps(&e),
            vec![
                CULLING_PUSH,
                ACCUMULATE_SCENE,
                RENDERING_SET_STAGE,
                RENDERING_WAIT_STAGE,
                ACCUMULATE_FINISH,
                CULLING_POP
            ]
        );
        assert_eq!(calls(&e, RENDERING_SET_STAGE), vec![vec![0xe500, 0, 0x17]]);
        // The renderer's two slots agree: the camera got the viewport.
        assert_eq!(e.mem.f32(camera + 0x104), 1.0);
        // The third setting adds a pass.
        settings(&mut e, true, true, true);
        fn_007134d0(&mut e, m, 0xc000, 0);
        assert_eq!(calls(&e, ISOLATE_MENU_ELEMENTS), vec![vec![1, 0]]);
        assert_eq!(calls(&e, RESTORE_MENU_ELEMENTS).len(), 1);
    }

    #[test]
    fn both_settings_prepare_the_tiles_unless_the_click_is_multithreaded() {
        let (mut e, m, _) = accumulate_world();
        settings(&mut e, true, true, false);
        returns(&mut e, COLLECTION_IS_EMPTY, 1);
        let setting = e.mem.alloc(4);
        returns(&mut e, GET_SETTING_VALUE, setting);
        fn_007134d0(&mut e, m, 0xc000, 0);
        // `00713c70` ran (it calls the tile update preparation) and the
        // menu elements were isolated for the pass.
        assert_eq!(calls(&e, TILE_UPDATE_PREPARE).len(), 1);
        e.set(m, InterfaceManager::bClickMultithreaded, 1);
        fn_007134d0(&mut e, m, 0xc000, 0);
        assert_eq!(calls(&e, TILE_UPDATE_PREPARE).len(), 1);
        assert_eq!(calls(&e, RENDERING_WAIT_STAGE).len(), 2);
    }

    #[test]
    fn the_multithreaded_click_queues_the_accumulation() {
        let (mut e, m, camera) = accumulate_world();
        returns(&mut e, COLLECTION_IS_EMPTY, 1);
        let setting = e.mem.alloc(4);
        returns(&mut e, GET_SETTING_VALUE, setting);
        fn_00713f00(&mut e, m);
        assert_eq!(e.get(m, InterfaceManager::bClickMultithreaded), 1);
        assert_eq!(
            calls(&e, RENDERING_ADD_ACCUM_TASK),
            vec![vec![0xe500, camera, 0, 0x1111, 0, 0, 0x3333, 1, 0x17, 0]]
        );
        assert_eq!(calls(&e, RENDERING_SET_STAGE), vec![vec![0xe500, 0, 0x17]]);
        // The tiles were updated (the menu-mode test and the loading menu
        // did not object); the menu elements are isolated only when the
        // predicate holds.
        assert_eq!(calls(&e, TILE_UPDATE_PREPARE).len(), 1);
        assert!(calls(&e, ISOLATE_MENU_ELEMENTS).is_empty());
        returns(&mut e, MENU_PREDICATE_007079F0, 1);
        returns(&mut e, MENU_MODE_IS_NOT_ONE, 1);
        fn_00713f00(&mut e, m);
        assert_eq!(calls(&e, TILE_UPDATE_PREPARE).len(), 1);
        assert_eq!(calls(&e, ISOLATE_MENU_ELEMENTS), vec![vec![0, 0]]);
        returns(&mut e, IS_IN_GAME_LOADING_MENU_OPEN, 1);
        fn_00713f00(&mut e, m);
        assert_eq!(calls(&e, ISOLATE_MENU_ELEMENTS).len(), 1);
    }

    /// The full render world: a renderer object whose slots are recorders,
    /// a COM-style device with recorders, and a scene graph.
    fn render_world() -> (Engine, Ptr<InterfaceManager>, u32, u32) {
        let (mut e, m, _) = accumulate_world();
        settings(&mut e, false, false, false);
        let device = fake_object(
            &mut e,
            &[
                (0xbc, 0),
                (0x160, 0),
                (0xe4, 0),
                (0x1ac, 0),
                (0x170, 0),
                (0x15c, 0),
            ],
        );
        let target = fake_object(&mut e, &[(0x8c, 640), (0x90, 480)]);
        let renderer = fake_object(
            &mut e,
            &[
                (0xd4, target),
                (0xcc, target),
                (0xc8, 0),
                (0xb4, 0),
                (0xac, 0),
                (0x188, 0),
            ],
        );
        returns(&mut e, RENDERER_GET, renderer);
        returns(&mut e, DEVICE_GET, device);
        let state = fake_object(
            &mut e,
            &[
                (0x8c, 0),
                (0x7c, 0),
                (0xb0, 11),
                (0xb8, 0),
                (0x90, 12),
                (0x98, 0),
                (0x80, 13),
                (0x88, 0),
            ],
        );
        returns(&mut e, RENDERER_STATE_OBJECT, state);
        returns(&mut e, RENDER_PASS_ACTIVE, 1);
        (e, m, device, renderer)
    }

    #[test]
    fn an_inactive_pass_only_builds_and_runs_the_culling_process() {
        let (mut e, m, _device, _) = render_world();
        returns(&mut e, RENDER_PASS_ACTIVE, 0);
        returns(&mut e, GET_SECOND_SCENE_NODE, 0x4444);
        returns(&mut e, CULLING_SOURCE_GET, 0x5550);
        returns(&mut e, WORD_AT_8, 0x99);
        fn_00713fb0(&mut e, m, 0, 0);
        assert_eq!(calls(&e, CULLING_SOURCE_GET), vec![vec![0x1111]]);
        let construct = calls(&e, CULLING_PROCESS_CONSTRUCT);
        assert_eq!(construct.len(), 1);
        assert_eq!(construct[0][1], 0x99);
        let culling = construct[0][0];
        assert_eq!(calls(&e, NI_POINTER_CONSTRUCT)[0][1], 0x4444);
        assert_eq!(calls(&e, CULLING_PUSH), vec![vec![culling, 1]]);
        assert_eq!(calls(&e, CULLING_PROCESS_DESTRUCT), vec![vec![culling]]);
        assert_eq!(calls(&e, NI_POINTER_DESTROY).len(), 1);
        assert_eq!(calls(&e, SCOPE_GUARD_BEGIN)[0][4], 0x1069);
        assert!(calls(&e, RENDER_PASS_SET_MODE).is_empty());
        assert!(calls(&e, RENDER_PASS_RESET).is_empty());
    }

    #[test]
    fn without_a_scene_graph_the_render_stops_after_the_pass_setup() {
        let (mut e, m, _, _) = render_world();
        returns(&mut e, GET_FRAME_SCENE_NODE, 0);
        fn_00713fb0(&mut e, m, 0, 0);
        assert!(calls(&e, CULLING_PROCESS_CONSTRUCT).is_empty());
        assert_eq!(calls(&e, SCOPE_GUARD_END).len(), 1);
        // The setup still ran: the mode was set.
        assert_eq!(calls(&e, RENDER_PASS_SET_MODE), vec![vec![7, 0]]);
    }

    #[test]
    fn an_active_pass_sets_the_viewport_clears_the_strips_and_resets_the_device() {
        let (mut e, m, device, renderer) = render_world();
        e.set_global(VIEWPORT_RECT_ENABLED, 1u8);
        for (i, v) in [0.25f32, 0.75, 0.5, 0.125].iter().enumerate() {
            e.mem.set_f32(VIEWPORT_RECT + 4 * i as u32, *v);
        }
        returns(&mut e, WORD_AT_8, 800);
        returns(&mut e, WORD_AT_C, 600);
        returns(&mut e, RENDER_CLEAR_ENABLED, 1);
        // The device's viewport calls are captured as they happen.
        let viewports = Rc::new(RefCell::new(vec![]));
        let seen = viewports.clone();
        e.register_double(slot_target(device, 0xbc), move |e, a| {
            let words: Vec<u32> = (0..6).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            seen.borrow_mut().push((a[0], words));
            Ret::default()
        });
        e.mem.set_f32(CLEAR_RECT_ONE_TOP, 0.5);
        e.mem.set_f32(CLEAR_RECT_TWO_LEFT, 0.75);
        // Strips are filled through the renderer's kind test.
        returns(&mut e, OBJECT_IS_KIND, 1);
        let rect_cell = e.mem.alloc(0x10);
        returns(&mut e, RECT_CONSTRUCT, rect_cell);
        returns(&mut e, GET_SECOND_SCENE_NODE, 0x4444);
        returns(&mut e, WORD_AT_8, 800);
        fn_00713fb0(&mut e, m, 0, 0);
        // First viewport: 800 * 0.25, 600 * 0.125, 800 * 0.5, 600 * 0.375.
        let seen = viewports.borrow();
        assert_eq!(seen[0].0, device);
        assert_eq!(
            seen[0].1,
            vec![200, 75, 400, 225, 0.0f32.to_bits(), 1.0f32.to_bits()]
        );
        // The second is the render target's size (640x480), depth 0..1.
        assert_eq!(seen[1].1[..4], [0, 0, 640, 480]);
        assert_eq!(seen[1].1[4..], [0.0f32.to_bits(), 1.0f32.to_bits()]);
        drop(seen);
        assert_eq!(calls(&e, RENDER_PASS_SET_MODE), vec![vec![7, 0]]);
        // The render states of the device.
        assert_eq!(
            calls(&e, slot_target(device, 0xe4)),
            vec![vec![device, 0x1b, 0]]
        );
        assert_eq!(calls(&e, slot_target(device, 0x1ac)), vec![vec![device, 0]]);
        assert_eq!(calls(&e, slot_target(device, 0x170)), vec![vec![device, 0]]);
        assert_eq!(calls(&e, slot_target(device, 0x15c)), vec![vec![device, 0]]);
        assert_eq!(calls(&e, slot_target(device, 0x160)).len(), 1);
        // Both 7148c0 strips were filled (slot 0x188 of the renderer is not
        // a fake slot, so the kind test answered with the double).
        assert_eq!(calls(&e, OBJECT_IS_KIND).len(), 2);
        let _ = renderer;
        // The 18 counters ran and the state object's values were copied.
        assert_eq!(calls(&e, RENDER_PASS_RESET).len(), 1);
        assert_eq!(calls(&e, RENDER_STATE_00714C40), vec![vec![0]]);
        assert_eq!(calls(&e, RENDER_STATE_004ECED0), vec![vec![0]]);
    }

    #[test]
    fn the_topmost_flag_reaches_the_accumulation_pass() {
        let (mut e, m, _, _) = render_world();
        returns(&mut e, RENDER_PASS_ACTIVE, 0);
        // Setting B on, the mode word is 2: topmost without asking.
        settings(&mut e, false, true, false);
        e.set(m, InterfaceManager::field_4bc, 2);
        fn_00713fb0(&mut e, m, 0, 0);
        assert!(calls(&e, IS_IN_PIPBOY_MENU).is_empty());
        assert_eq!(calls(&e, IS_CURRENT_RENDERED_MENU_TOPMOST).len(), 0);
        // Another mode word: the pipboy test and then the topmost test.
        e.set(m, InterfaceManager::field_4bc, 0);
        returns(&mut e, IS_IN_PIPBOY_MENU, 1);
        returns(&mut e, IS_CURRENT_RENDERED_MENU_TOPMOST, 1);
        fn_00713fb0(&mut e, m, 0, 0);
        assert_eq!(calls(&e, IS_IN_PIPBOY_MENU).len(), 1);
        assert_eq!(calls(&e, IS_CURRENT_RENDERED_MENU_TOPMOST).len(), 1);
        // Neither the pipboy nor the rendered menu: no topmost test.
        returns(&mut e, IS_IN_PIPBOY_MENU, 0);
        fn_00713fb0(&mut e, m, 0, 0);
        assert_eq!(calls(&e, IS_CURRENT_RENDERED_MENU_TOPMOST).len(), 1);
        e.set(m, InterfaceManager::bIsInRenderedMenu, 1);
        fn_00713fb0(&mut e, m, 0, 0);
        assert_eq!(calls(&e, IS_CURRENT_RENDERED_MENU_TOPMOST).len(), 2);
    }

    #[test]
    fn the_task_manager_is_flushed_waited_on_and_released() {
        let (mut e, m, _, _) = render_world();
        returns(&mut e, RENDER_PASS_ACTIVE, 0);
        // A worker object with the wrapper at +0x190 and a finished
        // flag at +0x1b0.
        let manager_object = e.mem.alloc(0x200);
        e.set_global(WORKER_OBJECT, manager_object);
        returns(&mut e, WORKER_GET, manager_object);
        e.mem.set_u32(manager_object + 0x190 + 8, 0xface);
        e.mem.set_u8(manager_object + 0x1b0, 1);
        returns(&mut e, WAIT_FOR_SINGLE_OBJECT, 0x102);
        fn_00713fb0(&mut e, m, 0, 0);
        assert_eq!(calls(&e, WORKER_FLUSH).len(), 1);
        // The wait timed out: the blocked byte is set and it waited for
        // ever; the semaphore is released at the end.
        assert_eq!(
            calls(&e, WAIT_FOR_SINGLE_OBJECT),
            vec![vec![0xface, 0xffff_ffff], vec![0xface, 0xffff_ffff]]
        );
        assert_eq!(e.global::<u8>(WORKER_WAS_BLOCKED), 1);
        assert_eq!(calls(&e, RELEASE_SEMAPHORE), vec![vec![0xface, 1, 0]]);
        // Not finished: no flush.
        e.mem.set_u8(manager_object + 0x1b0, 0);
        fn_00713fb0(&mut e, m, 0, 0);
        assert_eq!(calls(&e, WORKER_FLUSH).len(), 1);
    }

    #[test]
    fn the_renderer_object_is_asked_to_fill_only_if_it_is_of_the_kind() {
        let mut e = world();
        let target = fake_object(&mut e, &[(0x188, 0)]);
        returns(&mut e, OBJECT_IS_KIND, 0);
        fn_007148c0(&mut e, Ptr::new(target), 5, 6);
        assert!(calls(&e, slot_target(target, 0x188)).is_empty());
        returns(&mut e, OBJECT_IS_KIND, 1);
        fn_007148c0(&mut e, Ptr::new(target), 5, 6);
        assert_eq!(
            calls(&e, slot_target(target, 0x188)),
            vec![vec![target, 5, 6]]
        );
        assert_eq!(calls(&e, OBJECT_IS_KIND)[0], vec![target, KIND_NAME, 1]);
    }

    #[test]
    fn the_task_manager_wrapper_functions() {
        let mut e = world();
        // No manager: everything is a no-op.
        assert_eq!(fn_00714900(&mut e), 0);
        assert!(!fn_00714a00(&mut e));
        assert_eq!(fn_00714960(&mut e, 5), 0);
        assert!(calls(&e, WAIT_FOR_SINGLE_OBJECT).is_empty());
        let object = e.mem.alloc(0x200);
        e.set_global(WORKER_OBJECT, object);
        e.mem.set_u32(object + 0x190, 0xabcd);
        e.mem.set_u32(object + 0x190 + 8, 0xface);
        // Release: the pre-step, the semaphore, the first word.
        assert_eq!(fn_00714900(&mut e), 0xabcd);
        assert_eq!(calls(&e, BEFORE_RELEASE), vec![vec![object + 0x190]]);
        assert_eq!(calls(&e, RELEASE_SEMAPHORE), vec![vec![0xface, 1, 0]]);
        // The finished flag.
        assert!(!fn_00714a00(&mut e));
        e.mem.set_u8(object + 0x1b0, 1);
        assert!(fn_00714a00(&mut e));
        // A wait that is not a time-out hands over to the result handler.
        returns(&mut e, WAIT_RESULT_HANDLER, 42);
        returns(&mut e, WAIT_FOR_SINGLE_OBJECT, 0);
        assert_eq!(fn_007149b0(&mut e, Ptr::new(object + 0x190), 9), 42);
        assert_eq!(calls(&e, WAIT_FOR_SINGLE_OBJECT)[0], vec![0xface, 9]);
        assert_eq!(calls(&e, WAIT_RESULT_HANDLER), vec![vec![object + 0x190]]);
        // A time-out is 1; the outer wait then sets the byte and waits
        // again for ever.
        returns(&mut e, WAIT_FOR_SINGLE_OBJECT, 0x102);
        assert_eq!(fn_007149b0(&mut e, Ptr::new(object + 0x190), 9), 1);
        assert_eq!(fn_00714960(&mut e, 5), 0);
        assert_eq!(e.global::<u8>(WORKER_WAS_BLOCKED), 1);
        let waits = calls(&e, WAIT_FOR_SINGLE_OBJECT);
        assert_eq!(
            waits[waits.len() - 2..],
            [vec![0xface, 5], vec![0xface, 0xffff_ffff]]
        );
        // A wait that did not time out ends it.
        e.set_global(WORKER_WAS_BLOCKED, 0u8);
        returns(&mut e, WAIT_FOR_SINGLE_OBJECT, 0);
        assert_eq!(fn_00714960(&mut e, 5), 0);
        assert_eq!(e.global::<u8>(WORKER_WAS_BLOCKED), 0);
        fn_007149f0(&mut e);
        assert_eq!(e.global::<u8>(WORKER_WAS_BLOCKED), 1);
    }

    #[test]
    fn each_render_state_counter_is_reduced_and_its_state_restored() {
        type Counter = fn(&mut Engine, u32);
        let table: [(Counter, u32, u32, &[u32]); 14] = [
            (fn_00714a40, 0x011f_f9d8, 0x00b9_7de0, &[1, 0]),
            (fn_00714a60, 0x011f_f9dc, 0x00b9_7e30, &[1, 0]),
            (fn_00714a80, 0x011f_f9e0, 0x00b9_7e80, &[3, 0]),
            (fn_00714aa0, 0x011f_f9e8, 0x00b9_7ed0, &[0, 0]),
            (fn_00714ac0, 0x011f_f9ec, 0x00b9_7f20, &[0, 0, 0]),
            (fn_00714ae0, 0x011f_f9f0, 0x00b9_7fa0, &[0, 0]),
            (fn_00714b00, 0x011f_f9f4, 0x00b9_7ff0, &[0, 1, 0]),
            (fn_00714b20, 0x011f_fa00, 0x00b9_80c0, &[0, 0, 0, 0]),
            (fn_00714b50, 0x011f_fa04, 0x00b9_8180, &[0, 0, 0xff, 0]),
            (fn_00714b80, 0x011f_fa08, 0x00b9_8230, &[0xff, 0]),
            (fn_00714bb0, 0x011f_fa0c, 0x00b9_84f0, &[0, 0]),
            (fn_00714bf0, 0x011f_f9e4, 0x00b9_8320, &[0, 0]),
            (fn_00714c20, 0x011f_fa20, 0x00b9_8480, &[0, 0]),
            (fn_00714bd0, 0x011f_fa10, 0, &[]),
        ];
        for (function, counter, state, arguments) in table {
            let mut e = world();
            e.set_global(counter, 10u32);
            function(&mut e, 3);
            assert_eq!(e.global::<u32>(counter), 7, "{counter:x}");
            if state != 0 {
                assert_eq!(calls(&e, state), vec![arguments.to_vec()], "{state:x}");
            }
            // Wrapping, as the `sub` does.
            e.set_global(counter, 1u32);
            function(&mut e, 2);
            assert_eq!(e.global::<u32>(counter), 0xffff_ffff);
        }
    }

    // ---- 00714c40 .. 007170a0 ----

    fn set_stack(e: &mut Engine, m: Ptr<InterfaceManager>, words: &[u32]) {
        for index in 0..10u32 {
            let word = words.get(index as usize).copied().unwrap_or(0);
            e.mem.set_u32(m.addr() + 0x114 + 4 * index, word);
        }
    }

    fn get_stack(e: &Engine, m: Ptr<InterfaceManager>) -> Vec<u32> {
        (0..10u32)
            .map(|index| e.mem.u32(m.addr() + 0x114 + 4 * index))
            .collect()
    }

    /// A world with a manager, and the stack notification object present.
    fn stack_world(words: &[u32]) -> (Engine, Ptr<InterfaceManager>) {
        let mut e = world();
        let m = manager(&mut e);
        returns(&mut e, STACK_NOTIFY_OWNER_GET, 0x77);
        set_stack(&mut e, m, words);
        (e, m)
    }

    /// The interface width of a 1920 x 1080 target, as the game keeps it
    /// (a `float`).
    fn wide_screen_width() -> f64 {
        (1920.0f64 / 1080.0 * 960.0) as f32 as f64
    }

    /// The interface-space constants the screen-shape functions read.
    fn screen_constants(e: &mut Engine) {
        e.set_global(BASE_WIDTH_FLOAT, 1280.0f32);
        e.set_global(BASE_HEIGHT_FLOAT, 960.0f32);
        e.set_global(BASE_WIDTH_DOUBLE, 1280.0f64);
        e.set_global(BASE_HEIGHT_DOUBLE, 960.0f64);
        e.set_global(TWO, 2.0f64);
    }

    #[test]
    fn a_render_state_counter_and_call_for_011ffa28() {
        let mut e = world();
        e.set_global(RENDER_COUNTER_011FFA28, 10u32);
        fn_00714c40(&mut e, 3);
        assert_eq!(e.global::<u32>(RENDER_COUNTER_011FFA28), 7);
        assert_eq!(calls(&e, RENDER_STATE_00B98540), vec![vec![0, 0]]);
    }

    #[test]
    fn emergency_close_closes_every_menu_in_its_own_way() {
        let mut e = world();
        let m = manager(&mut e);
        let (generic_menu, generic_log) = virtual_object(&mut e, &[(0, 0x00aa_0000)], 0);
        let (loading_menu, loading_log) = virtual_object(&mut e, &[(0, 0x00aa_0004)], 0);
        e.register_double(TILE_GET_MENU_BY_CLASS, |_, a| Ret {
            eax: match a[0] {
                0x400 => 0x5000,
                0x3ef => 0x6000,
                _ => 0,
            },
            ..Ret::default()
        });
        e.register_double(TILE_GET_MENU, move |_, a| Ret {
            eax: match a[0] {
                0x5000 => generic_menu,
                0x6000 => loading_menu,
                _ => 0,
            },
            ..Ret::default()
        });
        returns(&mut e, MENU_CONSOLE_INSTANCE, 0x9000);
        e.set(m, InterfaceManager::cMenuMode, 3);
        set_stack(&mut e, m, &[0x3ef, 3, 0x400]);
        interface_manager_emergency_close_all_menus_and_break_stuff(&mut e, m);
        assert_eq!(get_stack(&e, m), vec![0; 10]);
        // From the last used word back: the generic menu is deleted, the
        // console hidden, the loading menu closed by its own call.
        assert_eq!(generic_log[0].borrow().clone(), vec![vec![generic_menu, 1]]);
        assert!(loading_log[0].borrow().is_empty());
        assert_eq!(calls(&e, MENU_CONSOLE_TOGGLE_VISIBLE), vec![vec![0x9000]]);
        assert_eq!(
            order(
                &e,
                &[MENU_CONSOLE_TOGGLE_VISIBLE, CLOSE_LOADING_MENU, HIDE_MENUS]
            ),
            vec![MENU_CONSOLE_TOGGLE_VISIBLE, CLOSE_LOADING_MENU, HIDE_MENUS]
        );
        assert_eq!(e.get(m, InterfaceManager::cMenuMode), 4);
        assert_eq!(e.global::<u8>(START_MENU_ALLOWED), 1);
    }

    #[test]
    fn emergency_close_leaves_a_console_alone_when_there_is_no_console() {
        let mut e = world();
        let m = manager(&mut e);
        returns(&mut e, MENU_CONSOLE_INSTANCE, 0);
        set_stack(&mut e, m, &[3]);
        interface_manager_emergency_close_all_menus_and_break_stuff(&mut e, m);
        assert!(calls(&e, MENU_CONSOLE_TOGGLE_VISIBLE).is_empty());
        assert_eq!(get_stack(&e, m)[0], 0);
    }

    #[test]
    fn the_pick_reference_is_stored() {
        let mut e = world();
        let m = manager(&mut e);
        fn_00714d70(&mut e, m, 0x1234);
        assert_eq!(e.get(m, InterfaceManager::pPickRef), 0x1234);
    }

    #[test]
    fn a_menu_class_goes_on_the_first_free_stack_word() {
        // First entry: mode 3, the owner is told.
        let (mut e, m) = stack_world(&[]);
        assert_eq!(interface_manager_add_to_enter_stack(&mut e, m, 5), 0);
        assert_eq!(get_stack(&e, m)[..2], [5, 0]);
        assert_eq!(e.get(m, InterfaceManager::cMenuMode), 3);
        assert_eq!(calls(&e, STACK_NOTIFY), vec![vec![0x77, 1]]);
        // Second entry: nobody is told.
        assert_eq!(interface_manager_add_to_enter_stack(&mut e, m, 6), 1);
        assert_eq!(get_stack(&e, m)[..3], [5, 6, 0]);
        assert_eq!(calls(&e, STACK_NOTIFY).len(), 1);
        // The class 0x3e9 as the first entry: mode 3 but nobody is told.
        let (mut e, m) = stack_world(&[]);
        assert_eq!(interface_manager_add_to_enter_stack(&mut e, m, 0x3e9), 0);
        assert_eq!(e.get(m, InterfaceManager::cMenuMode), 3);
        assert!(calls(&e, STACK_NOTIFY).is_empty());
        // ... and the owner is told when something comes under it.
        assert_eq!(interface_manager_add_to_enter_stack(&mut e, m, 6), 1);
        assert_eq!(calls(&e, STACK_NOTIFY), vec![vec![0x77, 1]]);
        // Without the owner nobody is told.
        let mut e = world();
        let m = manager(&mut e);
        assert_eq!(interface_manager_add_to_enter_stack(&mut e, m, 5), 0);
        assert!(calls(&e, STACK_NOTIFY).is_empty());
        // A full stack refuses.
        let (mut e, m) = stack_world(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(interface_manager_add_to_enter_stack(&mut e, m, 6), -1);
        assert_eq!(get_stack(&e, m), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn the_console_class_goes_first_and_pushes_the_others_down() {
        let (mut e, m) = stack_world(&[]);
        assert_eq!(interface_manager_add_to_enter_stack(&mut e, m, 3), 1);
        assert_eq!(get_stack(&e, m)[..2], [3, 0]);
        assert_eq!(e.get(m, InterfaceManager::cMenuMode), 3);
        assert_eq!(calls(&e, STACK_NOTIFY), vec![vec![0x77, 1]]);
        let (mut e, m) = stack_world(&[5, 6]);
        assert_eq!(interface_manager_add_to_enter_stack(&mut e, m, 3), 2);
        assert_eq!(get_stack(&e, m)[..4], [3, 5, 6, 0]);
        assert!(calls(&e, STACK_NOTIFY).is_empty());
        // A full stack loses its last pushed word and logs.
        let (mut e, m) = stack_world(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(interface_manager_add_to_enter_stack(&mut e, m, 3), 9);
        assert_eq!(get_stack(&e, m), vec![3, 1, 2, 3, 4, 5, 6, 7, 8, 10]);
        assert_eq!(calls(&e, LOG_WARNING), vec![vec![STACK_FULL_MESSAGE]]);
    }

    #[test]
    fn the_top_of_the_menu_stack_is_the_last_used_word() {
        let (mut e, m) = stack_world(&[]);
        assert_eq!(interface_manager_get_enter_stack_top(&mut e, m), 0);
        set_stack(&mut e, m, &[7]);
        assert_eq!(interface_manager_get_enter_stack_top(&mut e, m), 7);
        set_stack(&mut e, m, &[7, 8, 9]);
        assert_eq!(interface_manager_get_enter_stack_top(&mut e, m), 9);
        set_stack(&mut e, m, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(interface_manager_get_enter_stack_top(&mut e, m), 10);
    }

    #[test]
    fn a_stack_word_is_read_and_searched_for() {
        let (mut e, m) = stack_world(&[4, 5, 6]);
        assert_eq!(interface_manager_get_enter_stack(&mut e, m, 1), 5);
        assert_eq!(interface_manager_get_enter_stack(&mut e, m, 3), 0);
        assert!(fn_00714f90(&mut e, m, 6));
        assert!(!fn_00714f90(&mut e, m, 7));
        set_stack(&mut e, m, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 9]);
        assert!(fn_00714f90(&mut e, m, 9));
    }

    #[test]
    fn popping_from_an_empty_stack_or_for_an_absent_class_fails() {
        let (mut e, m) = stack_world(&[]);
        assert_eq!(interface_manager_pop_from_enter_stack(&mut e, m, 5, 1), -1);
        set_stack(&mut e, m, &[7]);
        assert_eq!(interface_manager_pop_from_enter_stack(&mut e, m, 5, 0), -1);
        assert_eq!(get_stack(&e, m)[0], 7);
        // A class under an entry above 0x3e9 is not popped without force.
        set_stack(&mut e, m, &[9, 0x400]);
        assert_eq!(interface_manager_pop_from_enter_stack(&mut e, m, 9, 0), -2);
        assert_eq!(get_stack(&e, m)[..2], [9, 0x400]);
        // ... but the console may be.
        set_stack(&mut e, m, &[3, 0x400]);
        assert_ne!(interface_manager_pop_from_enter_stack(&mut e, m, 3, 0), -2);
    }

    #[test]
    fn popping_closes_the_stack_up_and_returns_the_word_below() {
        let (mut e, m) = stack_world(&[7, 9]);
        assert_eq!(interface_manager_pop_from_enter_stack(&mut e, m, 9, 0), 7);
        assert_eq!(get_stack(&e, m)[..3], [7, 0, 0]);
        // The closing up compares the word it is writing to, not the one
        // it reads, so a second equal word that moved down survives.
        set_stack(&mut e, m, &[4, 9, 5, 9, 6]);
        assert_eq!(interface_manager_pop_from_enter_stack(&mut e, m, 9, 1), 9);
        assert_eq!(get_stack(&e, m)[..4], [4, 5, 9, 0]);
        // Forced, the class need not be present.
        set_stack(&mut e, m, &[7]);
        assert_eq!(interface_manager_pop_from_enter_stack(&mut e, m, 9, 1), 7);
        assert_eq!(get_stack(&e, m)[..2], [7, 0]);
        // 0x3e9 left alone as the second word tells the owner.
        let (mut e, m) = stack_world(&[0x3e9, 5]);
        assert_eq!(
            interface_manager_pop_from_enter_stack(&mut e, m, 5, 0),
            0x3e9
        );
        assert_eq!(calls(&e, STACK_NOTIFY), vec![vec![0x77, 1]]);
    }

    #[test]
    fn popping_the_last_menu_ends_the_menu_mode() {
        let (mut e, m) = stack_world(&[1]);
        e.set(m, InterfaceManager::cMenuMode, 3);
        assert_eq!(interface_manager_pop_from_enter_stack(&mut e, m, 1, 0), 0);
        assert_eq!(e.get(m, InterfaceManager::cMenuMode), 4);
        assert_eq!(calls(&e, HIDE_MENUS).len(), 1);
        assert_eq!(calls(&e, STACK_NOTIFY), vec![vec![0x77, 1]]);
        // Another class: no hide; the class 0x3e9 tells with 0.
        let (mut e, m) = stack_world(&[0x3e9]);
        assert_eq!(
            interface_manager_pop_from_enter_stack(&mut e, m, 0x3e9, 0),
            0
        );
        assert!(calls(&e, HIDE_MENUS).is_empty());
        assert_eq!(calls(&e, STACK_NOTIFY), vec![vec![0x77, 0]]);
    }

    #[test]
    fn a_console_left_alone_is_hidden_and_popped_too() {
        let (mut e, m) = stack_world(&[3, 9]);
        returns(&mut e, MENU_CONSOLE_INSTANCE, 0x9000);
        assert_eq!(interface_manager_pop_from_enter_stack(&mut e, m, 9, 0), 0);
        assert_eq!(get_stack(&e, m), vec![0; 10]);
        assert_eq!(calls(&e, MENU_CONSOLE_TOGGLE_VISIBLE), vec![vec![0x9000]]);
        assert_eq!(e.get(m, InterfaceManager::cMenuMode), 4);
    }

    /// A world for the cursor placement: 1920 x 1080 target, a desktop of
    /// 1500 x 1000, a cursor tile 0xc0 with node 0x4000 whose translation
    /// is (1, 2, 3).
    fn placement_world() -> (Engine, Ptr<InterfaceManager>, u32) {
        let mut e = world();
        screen_constants(&mut e);
        e.set_global(CURSOR_X_OFFSET, 3.0f64);
        e.set_global(CURSOR_TILT_SCALE, -0.5f64);
        let m = manager(&mut e);
        e.set(m, InterfaceManager::pCursor, 0xc0);
        returns(&mut e, RENDER_TARGET_WIDTH, 1920);
        returns(&mut e, RENDER_TARGET_HEIGHT, 1080);
        returns_st0(&mut e, GET_DESKTOP_WIDTH, 1500.0);
        returns_st0(&mut e, GET_DESKTOP_HEIGHT, 1000.0);
        returns_st0(&mut e, TILE_GET_VALUE, 10.0);
        returns(&mut e, TILE_IMAGE_NODE, 0x4000);
        let translation = e.mem.alloc(12);
        e.mem.set_f32(translation, 1.0);
        e.mem.set_f32(translation + 4, 2.0);
        e.mem.set_f32(translation + 8, 3.0);
        returns(&mut e, NODE_TRANSLATION, translation);
        (e, m, translation)
    }

    #[test]
    fn the_cursor_is_placed_at_a_fraction_of_the_screen() {
        let (mut e, m, _) = placement_world();
        returns(&mut e, POINT_NOT_EQUAL, 1);
        returns(&mut e, HAS_360_CONTROLLER_GETTER, 0);
        returns(&mut e, MENU_MODE_BYTE_GETTER, 2);
        fn_007151b0(&mut e, m, 0.25, 0.5);
        let width = wide_screen_width();
        let x = (width * 0.25 - width / 2.0) as f32;
        let z = (960.0 / 2.0 + 960.0 * -0.5f64) as f32;
        assert_eq!(e.mem.f32(m.addr() + 0x2c), x);
        // The middle component keeps the translation's.
        assert_eq!(e.mem.f32(m.addr() + 0x30), 2.0);
        assert_eq!(e.mem.f32(m.addr() + 0x34), z);
        assert_eq!(
            calls(&e, NODE_SET_TRANSLATE_VECTOR),
            vec![vec![0x4000, m.addr() + 0x2c]]
        );
        assert_eq!(calls(&e, NODE_UPDATE).len(), 1);
        // The desktop-space point, with the tilt (10 * -0.5) in the middle.
        let real_x = (1500.0 / 2.0 + (1500.0 / width) * (x as f64 + 3.0)) as f32;
        let real_z = (1000.0 / 2.0 - (1000.0 / 960.0) * z as f64) as f32;
        assert_eq!(e.mem.f32(LAST_CURSOR_POINT), real_x);
        assert_eq!(e.mem.f32(LAST_CURSOR_POINT + 4), 0.0);
        assert_eq!(e.mem.f32(LAST_CURSOR_POINT + 8), real_z);
        assert_eq!(e.mem.f32(m.addr() + 0x38), real_x);
        assert_eq!(e.mem.f32(m.addr() + 0x3c), -5.0);
        assert_eq!(e.mem.f32(m.addr() + 0x40), real_z);
        // No 360 controller: the motion flag, and in mode 2 the cursor
        // node is shown and its trait 0xfa3 set.
        assert_eq!(e.get(m, InterfaceManager::bMouseInMotion), 1);
        assert_eq!(calls(&e, NODE_SET_FLAG), vec![vec![0x4000, 0]]);
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![0xc0, 0xfa3, 1]]);
        // The static point is constructed the first time only.
        assert_eq!(
            calls(&e, MEMBER_CONSTRUCTOR_EMPTY)
                .iter()
                .filter(|c| c[0] == LAST_CURSOR_POINT)
                .count(),
            1
        );
        fn_007151b0(&mut e, m, 0.25, 0.5);
        assert_eq!(
            calls(&e, MEMBER_CONSTRUCTOR_EMPTY)
                .iter()
                .filter(|c| c[0] == LAST_CURSOR_POINT)
                .count(),
            1
        );
    }

    #[test]
    fn an_unchanged_cursor_point_is_left_alone() {
        let (mut e, m, _) = placement_world();
        returns(&mut e, POINT_NOT_EQUAL, 0);
        returns(&mut e, HAS_360_CONTROLLER_GETTER, 1);
        e.set(m, InterfaceManager::bShowMouse, 0);
        // The stored real position equals the computed one: nothing more.
        let width = wide_screen_width();
        let x = (width * 0.0 - width / 2.0) as f32;
        let z = (960.0 / 2.0 + 960.0 * -0.0f64) as f32;
        let real_x = (1500.0 / 2.0 + (1500.0 / width) * (x as f64 + 3.0)) as f32;
        let real_z = (1000.0 / 2.0 - (1000.0 / 960.0) * z as f64) as f32;
        e.mem.set_f32(m.addr() + 0x38, real_x);
        e.mem.set_f32(m.addr() + 0x40, real_z);
        fn_007151b0(&mut e, m, 0.0, 0.0);
        assert_eq!(e.mem.f32(LAST_CURSOR_POINT), 0.0);
        assert_eq!(e.mem.f32(m.addr() + 0x38), real_x);
        assert_eq!(e.get(m, InterfaceManager::bMouseInMotion), 0);
        assert!(calls(&e, NODE_SET_FLAG).is_empty());
    }

    #[test]
    fn a_moved_real_cursor_position_is_scaled_again() {
        let (mut e, m, _) = placement_world();
        returns(&mut e, POINT_NOT_EQUAL, 0);
        returns(&mut e, HAS_360_CONTROLLER_GETTER, 1);
        e.mem.set_f32(m.addr() + 0x38, 100.0);
        e.mem.set_f32(m.addr() + 0x40, 40.0);
        fn_007151b0(&mut e, m, 0.0, 0.0);
        let width = wide_screen_width();
        assert_eq!(
            e.mem.f32(m.addr() + 0x38),
            (1500.0 / 2.0 + (1500.0 / width) * 100.0) as f32
        );
        assert_eq!(e.mem.f32(m.addr() + 0x3c), -5.0);
        assert_eq!(
            e.mem.f32(m.addr() + 0x40),
            (1000.0 / 2.0 - (1000.0 / 960.0) * 40.0) as f32
        );
        assert_eq!(e.get(m, InterfaceManager::bMouseInMotion), 1);
    }

    #[test]
    fn modifier_keys_are_tracked_by_press_and_release() {
        let mut e = world();
        let m = manager(&mut e);
        returns(&mut e, CONTROLS_GET, 0xc0de);
        // A press of a modifier key sets its bit and is no character.
        returns(&mut e, CONTROLS_QUERY_00A238A0, 0);
        for (key, bit) in [
            (0x2a, 4),
            (0x36, 4),
            (0x38, 1),
            (0xb8, 1),
            (0x1d, 2),
            (0x9d, 2),
        ] {
            e.set(m, InterfaceManager::iModifierKeys, 0);
            assert_eq!(fn_007154b0(&mut e, m, 1, key), 0);
            assert_eq!(e.get(m, InterfaceManager::iModifierKeys), bit);
            // The release clears it again (and the shift flag).
            assert_eq!(fn_007154b0(&mut e, m, 2, key), 0);
            assert_eq!(e.get(m, InterfaceManager::iModifierKeys), 0);
        }
        e.set(m, InterfaceManager::iModifierKeys, 7);
        fn_007154b0(&mut e, m, 2, 0x2a);
        assert_eq!(e.get(m, InterfaceManager::iModifierKeys), 3);
        fn_007154b0(&mut e, m, 1, 0x2a);
        assert_eq!(e.get(m, InterfaceManager::bShiftDown), 1);
        fn_007154b0(&mut e, m, 2, 0x36);
        assert_eq!(e.get(m, InterfaceManager::bShiftDown), 0);
        // Any other event is nothing.
        assert_eq!(fn_007154b0(&mut e, m, 0, 0x30), 0);
        assert_eq!(fn_007154b0(&mut e, m, 3, 0x30), 0);
    }

    #[test]
    fn a_key_press_becomes_a_character_or_an_editing_code() {
        let mut e = world();
        let m = manager(&mut e);
        e.set(m, InterfaceManager::bShiftDown, 1);
        returns(&mut e, CONTROLS_GET, 0xc0de);
        e.set_global(CONTROLS_OWNER, 0x1234u32);
        returns(&mut e, FADE_CLOCK_READ, 99);
        // An ordinary key: the character of the controls, asked with the
        // shift state.
        returns(&mut e, CONTROLS_QUERY_00A238A0, b'a' as u32);
        assert_eq!(fn_007154b0(&mut e, m, 1, 0x1e), b'a' as u32);
        assert_eq!(
            calls(&e, CONTROLS_QUERY_00A238A0),
            vec![vec![0xc0de, 0x1e, 1]]
        );
        assert_eq!(calls(&e, CONTROLS_GET), vec![vec![0x1234]]);
        // Backspace and enter and the pipe.
        returns(&mut e, CONTROLS_QUERY_00A238A0, 8);
        assert_eq!(fn_007154b0(&mut e, m, 1, 0x0e), 0x8000_0000);
        assert_eq!(e.get(m, InterfaceManager::iRepeatingKey), i32::MIN);
        assert_eq!(e.get(m, InterfaceManager::uKeyDownTime), 99);
        returns(&mut e, CONTROLS_QUERY_00A238A0, 0xd);
        assert_eq!(fn_007154b0(&mut e, m, 1, 0x1c), 0x8000_0008);
        returns(&mut e, CONTROLS_QUERY_00A238A0, 0x7c);
        assert_eq!(fn_007154b0(&mut e, m, 1, 0x2b), 0);
        // The navigation keys; those that repeat start the repeat.
        returns(&mut e, CONTROLS_QUERY_00A238A0, 0x41);
        for (key, code, repeats) in TEXT_ENTRY_KEYS {
            e.set(m, InterfaceManager::iRepeatingKey, 0);
            assert_eq!(fn_007154b0(&mut e, m, 1, key), code, "{key:x}");
            assert_eq!(
                e.get(m, InterfaceManager::iRepeatingKey),
                if repeats { code as i32 } else { 0 },
                "{key:x}"
            );
        }
    }

    #[test]
    fn escape_is_a_key_only_while_a_text_entry_is_up() {
        let mut e = world();
        let m = manager(&mut e);
        returns(&mut e, CONTROLS_GET, 0xc0de);
        returns(&mut e, CONTROLS_QUERY_00A238A0, 0x1b);
        assert_eq!(fn_007154b0(&mut e, m, 1, 1), 0);
        returns(&mut e, XUI_IS_UP, 1);
        e.set_global(FLAGS_OWNER, 0x88u32);
        returns(&mut e, HAS_FLAG, 1);
        assert_eq!(fn_007154b0(&mut e, m, 1, 1), 0x1b);
    }

    #[test]
    fn the_flag_check_needs_the_extra_interface_and_the_flag() {
        let mut e = world();
        e.set_global(FLAGS_OWNER, 0x88u32);
        assert!(!fn_00715770(&mut e));
        returns(&mut e, XUI_IS_UP, 1);
        assert!(!fn_00715770(&mut e));
        returns(&mut e, HAS_FLAG, 1);
        assert!(fn_00715770(&mut e));
        assert_eq!(
            calls(&e, HAS_FLAG),
            vec![vec![0x88, 0x80000], vec![0x88, 0x80000]]
        );
    }

    #[test]
    fn clearing_the_over_tile_target_tells_its_menu_only_when_asked() {
        let mut e = world();
        let m = manager(&mut e);
        returns_st0(&mut e, TILE_GET_VALUE, 7.0);
        returns(&mut e, FTOL, 7);
        e.set(m, InterfaceManager::pOverTileTarget, 0x100);
        e.set(m, InterfaceManager::pOverTileMenu, 0x200);
        interface_manager_clear_over_tile_target(&mut e, m, 0);
        assert!(calls(&e, DO_LEAVE).is_empty());
        assert_eq!(e.get(m, InterfaceManager::pOverTileTarget), 0);
        assert_eq!(e.get(m, InterfaceManager::pOverTileMenu), 0);
        e.set(m, InterfaceManager::pOverTileTarget, 0x100);
        e.set(m, InterfaceManager::pOverTileMenu, 0x200);
        interface_manager_clear_over_tile_target(&mut e, m, 1);
        assert_eq!(
            calls(&e, TILE_SET_INT),
            vec![vec![0x100, 0xfc3, 0], vec![0x100, 0xfc7, 0]]
        );
        assert_eq!(calls(&e, DO_LEAVE), vec![vec![m.addr(), 0x200, 7, 0x100]]);
        assert_eq!(e.get(m, InterfaceManager::pOverTileTarget), 0);
        // A tile without its menu is only forgotten.
        e.set(m, InterfaceManager::pOverTileTarget, 0x100);
        e.set(m, InterfaceManager::pOverTileMenu, 0);
        interface_manager_clear_over_tile_target(&mut e, m, 1);
        assert_eq!(calls(&e, DO_LEAVE).len(), 1);
        assert_eq!(e.get(m, InterfaceManager::pOverTileTarget), 0);
    }

    /// A world for the focus code: every trait reads 0.0 and the id 9, the
    /// menu of every tile is 0x7000.
    fn focus_world() -> (Engine, Ptr<InterfaceManager>) {
        let mut e = world();
        let m = manager(&mut e);
        returns_st0(&mut e, TILE_GET_VALUE, 0.0);
        returns(&mut e, FTOL, 9);
        returns(&mut e, TILE_GET_MENU, 0x7000);
        (e, m)
    }

    #[test]
    fn a_null_focus_target_clears_the_focus() {
        let (mut e, m) = focus_world();
        // The old focus counts a change (trait 0xfd6 is positive).
        returns_st0(&mut e, TILE_GET_VALUE, 1.0);
        e.set(m, InterfaceManager::pMouseOverTarget, 0x300);
        e.set(m, InterfaceManager::iLastXDefault, 5);
        interface_manager_set_current_focus_target(&mut e, m, 0, 0xfc3, 1);
        assert_eq!(e.get(m, InterfaceManager::pMouseOverTarget), 0);
        assert_eq!(e.get(m, InterfaceManager::iLastXDefault), 6);
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![0x300, 0xfd6, 6.0f32.to_bits(), 1]]
        );
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![0x300, 0xfc3, 0]]);
        assert_eq!(calls(&e, DO_LEAVE), vec![vec![m.addr(), 0x7000, 9, 0x300]]);
        // Nothing to leave: nothing happens.
        let (mut e, m) = focus_world();
        interface_manager_set_current_focus_target(&mut e, m, 0, 0xfc3, 1);
        assert!(calls(&e, DO_LEAVE).is_empty());
    }

    #[test]
    fn entering_a_tile_leaves_the_old_one_and_tells_the_new_menu() {
        let (mut e, m) = focus_world();
        e.set(m, InterfaceManager::pMouseOverTarget, 0x300);
        e.set(m, InterfaceManager::pOverTileTarget, 0x500);
        e.set(m, InterfaceManager::pOverTileMenu, 0x600);
        interface_manager_set_current_focus_target(&mut e, m, 0x400, 0xfc3, 1);
        assert_eq!(e.get(m, InterfaceManager::pMouseOverTarget), 0x400);
        assert_eq!(e.get(m, InterfaceManager::pOverTileTarget), 0);
        assert_eq!(e.get(m, InterfaceManager::pOverTileMenu), 0);
        assert_eq!(
            calls(&e, DO_LEAVE),
            vec![
                vec![m.addr(), 0x7000, 9, 0x500],
                vec![m.addr(), 0x7000, 9, 0x300]
            ]
        );
        assert_eq!(
            calls(&e, TILE_SET_INT),
            vec![
                vec![0x500, 0xfc3, 0],
                vec![0x300, 0xfc3, 0],
                vec![0x400, 0xfc3, 1]
            ]
        );
        assert_eq!(calls(&e, TILE_PLAY_TILE_SOUND), vec![vec![0x400, 0xfe8]]);
        assert_eq!(calls(&e, DO_ENTER), vec![vec![m.addr(), 0x7000, 9, 0x400]]);
        // Entering the tile that is the focus already does nothing; and no
        // sound without the flag.
        let (mut e, m) = focus_world();
        e.set(m, InterfaceManager::pMouseOverTarget, 0x400);
        interface_manager_set_current_focus_target(&mut e, m, 0x400, 0xfc3, 0);
        assert!(calls(&e, DO_ENTER).is_empty());
        let (mut e, m) = focus_world();
        interface_manager_set_current_focus_target(&mut e, m, 0x400, 0xfc3, 0);
        assert!(calls(&e, TILE_PLAY_TILE_SOUND).is_empty());
        assert_eq!(calls(&e, DO_ENTER).len(), 1);
    }

    #[test]
    fn clicking_a_tile_calls_its_menu_and_may_drop_the_focus() {
        let (mut e, m) = focus_world();
        let (menu, logs) = virtual_object(&mut e, &[(0xc, 0x00aa_0010)], 0);
        returns(&mut e, TILE_GET_MENU, menu);
        returns_st0(&mut e, TILE_GET_VALUE, 3.0);
        returns(&mut e, FTOL, 3);
        e.set(m, InterfaceManager::pMouseOverTarget, 0x300);
        returns(&mut e, TILE_IS_VISIBLE, 1);
        returns(&mut e, TILE_IS_TRUE, 1);
        e.set(m, InterfaceManager::iLastXDefault, 0);
        interface_manager_set_current_focus_target(&mut e, m, 0x400, 0xfc7, 1);
        assert_eq!(calls(&e, PLAY_MENU_SOUND), vec![vec![3]]);
        assert_eq!(
            calls(&e, TILE_SET_INT)
                .iter()
                .filter(|c| c[0] == 0x400)
                .cloned()
                .collect::<Vec<_>>(),
            vec![vec![0x400, 0xfc7, 1], vec![0x400, 0xfc7, 0]]
        );
        assert_eq!(logs[0].borrow().clone(), vec![vec![menu, 3, 0x400]]);
        assert_eq!(calls(&e, TILE_UPDATE_CHILDREN), vec![vec![0x400, 0]]);
        // The old focus is visible and has trait 0xfaf: kept.
        assert_eq!(e.get(m, InterfaceManager::pMouseOverTarget), 0x300);
        assert_eq!(calls(&e, TILE_IS_TRUE), vec![vec![0x300, 0xfaf]]);
        assert!(calls(&e, DO_LEAVE).is_empty());
        // Without the trait the old focus is left and cleared.
        returns(&mut e, TILE_IS_TRUE, 0);
        interface_manager_set_current_focus_target(&mut e, m, 0x400, 0xfc7, 0);
        assert_eq!(e.get(m, InterfaceManager::pMouseOverTarget), 0);
        assert_eq!(calls(&e, DO_LEAVE), vec![vec![m.addr(), menu, 3, 0x300]]);
        // No sound without the flag (the second click).
        assert_eq!(calls(&e, PLAY_MENU_SOUND).len(), 1);
    }

    #[test]
    fn a_trait_is_set_from_an_unsigned_integer() {
        let mut e = world();
        fn_00715c60(&mut e, 0x400, 0xfd6, 0x8000_0001);
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![0x400, 0xfd6, 2147483648.0f32.to_bits(), 1]]
        );
    }

    #[test]
    fn the_default_focus_goes_to_the_best_tile_found() {
        let mut e = world();
        let m = manager(&mut e);
        e.set(m, InterfaceManager::pCursor, 0xc0);
        e.set(m, InterfaceManager::bMouseInMotion, 1);
        // The frontmost menu has a root tile that is visible, focusable
        // and has a focus value of 5, and no children.
        let (root, _) = virtual_object(&mut e, &[(0xc, 0x00aa_0020)], 0x111);
        returns(&mut e, MENU_MANAGER_INSTANCE, 0x42);
        returns(&mut e, GET_FRONTMOST_MENU, 0xa000);
        returns(&mut e, GET_FIELD_AT_4, root);
        returns(&mut e, TILE_IS_TRUE, 1);
        let entry = e.mem.alloc(16);
        returns(&mut e, TILE_GET_TRAIT_ENTRY, entry);
        returns_st0(&mut e, TRAIT_ENTRY_VALUE, 5.0);
        returns(&mut e, FTOL, 5);
        returns(&mut e, TILE_IMAGE_NODE, 0x4000);
        returns(&mut e, HAS_360_CONTROLLER_GETTER, 0);
        interface_manager_get_default_focus(&mut e, m);
        assert_eq!(e.get(m, InterfaceManager::pMouseOverTarget), root);
        assert_eq!(calls(&e, NODE_SET_FLAG), vec![vec![0x4000, 1]]);
        assert!(calls(&e, TILE_SET_INT).contains(&vec![0xc0, 0xfa3, 0]));
        assert_eq!(e.get(m, InterfaceManager::bMouseInMotion), 0);
        // With nothing found the focus is cleared and the cursor is left
        // alone.
        let mut e2 = world();
        let m2 = manager(&mut e2);
        e2.set(m2, InterfaceManager::bMouseInMotion, 1);
        returns(&mut e2, GET_FRONTMOST_MENU, 0);
        e2.set(m2, InterfaceManager::pMouseOverTarget, 0x300);
        interface_manager_get_default_focus(&mut e2, m2);
        assert_eq!(e2.get(m2, InterfaceManager::pMouseOverTarget), 0);
        assert_eq!(e2.get(m2, InterfaceManager::bMouseInMotion), 1);
        assert!(calls(&e2, NODE_SET_FLAG).is_empty());
    }

    #[test]
    fn the_interface_width_follows_the_shape_of_the_screen() {
        let mut e = world();
        screen_constants(&mut e);
        returns(&mut e, RENDER_TARGET_WIDTH, 1920);
        returns(&mut e, RENDER_TARGET_HEIGHT, 1080);
        assert_eq!(
            interface_manager_get_screen_width(&mut e),
            (1920.0f32 as f64 / 1080.0f32 as f64 * 960.0) as f32
        );
        returns(&mut e, RENDER_TARGET_WIDTH, 1024);
        returns(&mut e, RENDER_TARGET_HEIGHT, 768);
        assert_eq!(interface_manager_get_screen_width(&mut e), 1280.0);
        // As wide as tall or taller: the base width.
        returns(&mut e, RENDER_TARGET_WIDTH, 800);
        returns(&mut e, RENDER_TARGET_HEIGHT, 800);
        assert_eq!(interface_manager_get_screen_width(&mut e), 1280.0);
        e.set_global(BASE_WIDTH_FLOAT, 1111.0f32);
        returns(&mut e, RENDER_TARGET_HEIGHT, 1000);
        assert_eq!(interface_manager_get_screen_width(&mut e), 1111.0);
    }

    #[test]
    fn the_interface_height_follows_the_shape_of_the_screen() {
        let mut e = world();
        screen_constants(&mut e);
        returns(&mut e, RENDER_TARGET_WIDTH, 1080);
        returns(&mut e, RENDER_TARGET_HEIGHT, 1920);
        assert_eq!(
            fn_00715da0(&mut e),
            (1920.0f32 as f64 / 1080.0f32 as f64 * 1280.0) as f32
        );
        returns(&mut e, RENDER_TARGET_WIDTH, 1920);
        returns(&mut e, RENDER_TARGET_HEIGHT, 1080);
        assert_eq!(fn_00715da0(&mut e), 960.0);
        returns(&mut e, RENDER_TARGET_HEIGHT, 1920);
        assert_eq!(fn_00715da0(&mut e), 960.0);
    }

    #[test]
    fn the_screen_metrics_go_to_the_string_lists_and_the_root_tile() {
        let mut e = world();
        screen_constants(&mut e);
        let m = manager(&mut e);
        e.set(m, InterfaceManager::pMenusRoot, 0x900);
        returns(&mut e, RENDER_TARGET_WIDTH, 1920);
        returns(&mut e, RENDER_TARGET_HEIGHT, 1080);
        returns_st0(&mut e, GET_INTERFACE_MARGIN_A, 8.0);
        returns_st0(&mut e, GET_INTERFACE_MARGIN_B, 9.0);
        fn_00715e00(&mut e, m, 20, 30);
        assert_eq!(
            calls(&e, STRING_LIST_SET_LIMIT),
            vec![vec![LIMIT_LIST_WIDTH, 20], vec![LIMIT_LIST_HEIGHT, 30]]
        );
        let width = (1920.0f32 as f64 / 1080.0f32 as f64 * 960.0) as f32;
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![
                vec![0x900, 0xfc0, 8.0f32.to_bits(), 1],
                vec![0x900, 0xfbf, 9.0f32.to_bits(), 1],
                vec![0x900, 0xfb1, width.to_bits(), 1],
                vec![0x900, 0xfb0, 960.0f32.to_bits(), 1],
            ]
        );
        // The height falls back to the width; no limits without a width.
        fn_00715e00(&mut e, m, 20, 0);
        assert_eq!(
            calls(&e, STRING_LIST_SET_LIMIT)[3],
            vec![LIMIT_LIST_HEIGHT, 20]
        );
        fn_00715e00(&mut e, m, 0, 5);
        assert_eq!(calls(&e, STRING_LIST_SET_LIMIT).len(), 4);
    }

    #[test]
    fn the_safe_zone_is_created_faded_in_and_toggled_away() {
        let mut e = world();
        let m = manager(&mut e);
        returns(&mut e, GET_MENUS_ROOT, 0x900);
        returns(&mut e, TILE_READ_FILE, 0x7777);
        returns(&mut e, TILE_IMAGE_NODE, 0);
        interface_manager_toggle_safe_zone(&mut e, m, 1);
        assert_eq!(e.get(m, InterfaceManager::pSafeZone), 0x7777);
        assert_eq!(calls(&e, TILE_READ_FILE), vec![vec![0x900, SAFE_ZONE_PATH]]);
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![0x7777, 0xfa3, 1]]);
        // RecursiveFade ran (its scope guard).
        assert_eq!(calls(&e, SCOPE_GUARD_BEGIN).len(), 1);
        // A second toggle removes it (virtual destructor with flag 1).
        let (zone, logs) = virtual_object(&mut e, &[(0, 0x00aa_0030)], 0);
        e.set(m, InterfaceManager::pSafeZone, zone);
        interface_manager_toggle_safe_zone(&mut e, m, 0);
        assert_eq!(logs[0].borrow().clone(), vec![vec![zone, 1]]);
        assert_eq!(e.get(m, InterfaceManager::pSafeZone), 0);
        assert_eq!(calls(&e, TILE_READ_FILE).len(), 1);
    }

    #[test]
    fn mode_three_replaces_the_safe_zone_and_mode_two_asks_it() {
        let mut e = world();
        let m = manager(&mut e);
        returns(&mut e, TILE_READ_FILE, 0x7777);
        returns(&mut e, TILE_IMAGE_NODE, 0);
        let (zone, logs) = virtual_object(&mut e, &[(0, 0x00aa_0040)], 0);
        e.set(m, InterfaceManager::pSafeZone, zone);
        interface_manager_toggle_safe_zone(&mut e, m, 3);
        assert_eq!(logs[0].borrow().clone(), vec![vec![zone, 1]]);
        assert_eq!(e.get(m, InterfaceManager::pSafeZone), 0x7777);
        // Mode 2 with a zone asks its trait 0xfa3 first, then removes it.
        let (zone, _) = virtual_object(&mut e, &[(0, 0x00aa_0044)], 0);
        e.set(m, InterfaceManager::pSafeZone, zone);
        interface_manager_toggle_safe_zone(&mut e, m, 2);
        assert_eq!(calls(&e, TILE_IS_TRUE), vec![vec![zone, 0xfa3]]);
        assert_eq!(e.get(m, InterfaceManager::pSafeZone), 0);
    }

    #[test]
    fn a_reference_is_forgotten_everywhere() {
        let mut e = world();
        let m = manager(&mut e);
        for offset in [0xf0u32, 0xf4, 0xf8, 0xfc, 0x100] {
            e.mem.set_u32(m.addr() + offset, 0x55);
        }
        e.mem.set_u32(m.addr() + 0xf8, 0x66);
        fn_00716010(&mut e, m, 0x55);
        let words: Vec<u32> = [0xf0u32, 0xf4, 0xf8, 0xfc, 0x100]
            .iter()
            .map(|offset| e.mem.u32(m.addr() + offset))
            .collect();
        assert_eq!(words, vec![0, 0, 0x66, 0, 0]);
        assert_eq!(calls(&e, HUD_FORGET_REFERENCE), vec![vec![0x55]]);
    }

    #[test]
    fn the_textures_are_released_in_order() {
        let mut e = world();
        let m = manager(&mut e);
        let (root, logs) = virtual_object(&mut e, &[(0x1c, 0x00aa_0050)], 0);
        returns(&mut e, GET_MENUS_ROOT, root);
        e.set_global(TEXTURE_CLEANUP_OWNER, 0x321u32);
        interface_manager_force_texture_release(&mut e, m);
        assert_eq!(logs[0].borrow().clone(), vec![vec![root]]);
        assert_eq!(
            order(
                &e,
                &[
                    RELEASE_STATIC_GEOMETRY,
                    GET_MENUS_ROOT,
                    CLEAR_WORLD_MAP_TEXTURE,
                    CLEAN_UP_UNUSED_TEXTURES
                ]
            ),
            vec![
                RELEASE_STATIC_GEOMETRY,
                GET_MENUS_ROOT,
                CLEAR_WORLD_MAP_TEXTURE,
                CLEAN_UP_UNUSED_TEXTURES
            ]
        );
        assert_eq!(calls(&e, GET_MENUS_ROOT), vec![vec![m.addr()]]);
        assert_eq!(calls(&e, CLEAN_UP_UNUSED_TEXTURES), vec![vec![0x321, 1]]);
    }

    /// A tile object: `+4` is the list of its children (a cursor-style
    /// list walked with `LIST_NEXT_ELEMENT`).
    struct FocusTiles {
        children: std::collections::HashMap<u32, Vec<u32>>,
        focus: std::collections::HashMap<u32, i32>,
        order: std::collections::HashMap<u32, i32>,
    }

    /// Doubles that describe a tree of tiles for `ScanForMaxFocus`: every
    /// tile is visible; those with an entry in `focus` are focusable (traits
    /// `0xfaf`, `0xfd6`); `order` gives trait `0xfac`.
    fn scan_world(tiles: FocusTiles) -> (Engine, Ptr<InterfaceManager>) {
        let mut e = world();
        let m = manager(&mut e);
        let tiles = Rc::new(tiles);
        // The children of a tile: handed out as a one-word-per-step cursor
        // (the cursor is the index plus one).
        let list = tiles.clone();
        e.register_double(NI_POINTER_GET, move |_, a| Ret {
            eax: if list
                .children
                .get(&(a[0] - 4))
                .is_some_and(|c| !c.is_empty())
            {
                1
            } else {
                0
            },
            ..Ret::default()
        });
        let list = tiles.clone();
        e.register_double(LIST_NEXT_ELEMENT, move |e, a| {
            let tile = a[0] - 4;
            let cursor = e.mem.u32(a[1]);
            let children = &list.children[&tile];
            let slot = e.mem.alloc(4);
            e.mem.set_u32(slot, children[cursor as usize - 1]);
            e.mem.set_u32(
                a[1],
                if cursor as usize == children.len() {
                    0
                } else {
                    cursor + 1
                },
            );
            Ret {
                eax: slot,
                ..Ret::default()
            }
        });
        let list = tiles.clone();
        e.register_double(TILE_IS_TRUE, move |_, a| Ret {
            eax: match a[1] {
                0xfa3 => 1,
                0xfaf => list.focus.contains_key(&a[0]) as u32,
                _ => 0,
            },
            ..Ret::default()
        });
        // The entries are the tile and the trait folded into one word.
        let list = tiles.clone();
        e.register_double(TILE_GET_TRAIT_ENTRY, move |_, a| Ret {
            eax: if list.focus.contains_key(&a[0]) {
                a[0] | 0x1000_0000
            } else {
                0
            },
            ..Ret::default()
        });
        let list = tiles.clone();
        e.register_double(TILE_GET_VALUE_Q, move |_, a| Ret {
            eax: if list.order.contains_key(&a[0]) {
                a[0] | 0x2000_0000
            } else {
                0
            },
            ..Ret::default()
        });
        let list = tiles;
        e.register_double(TRAIT_ENTRY_VALUE, move |_, a| Ret {
            st0: if a[0] & 0x2000_0000 != 0 {
                list.order[&(a[0] & 0xfff_ffff)] as f64
            } else {
                list.focus[&(a[0] & 0xfff_ffff)] as f64
            },
            ..Ret::default()
        });
        e.register(FTOL, |_, a| Ret {
            eax: f64::from_bits(((a[1] as u64) << 32) | a[0] as u64) as i32 as u32,
            ..Ret::default()
        });
        (e, m)
    }

    fn scan(e: &mut Engine, m: Ptr<InterfaceManager>, tile: u32, seed: i32) -> (u32, i32) {
        e.with_stack(4, |e, best| {
            e.mem.set_i32(best.addr(), seed);
            let found = interface_manager_scan_for_max_focus(e, m, best.addr(), tile);
            (found, e.mem.i32(best.addr()))
        })
    }

    #[test]
    fn the_scan_finds_the_child_with_the_highest_focus() {
        use std::collections::HashMap;
        // Tile 0x10 (focusable, 3) has children 0x20 (7), 0x30 (9), 0x40 (9, order 1)
        // and 0x30 has order 5.
        let (mut e, m) = scan_world(FocusTiles {
            children: HashMap::from([
                (0x10, vec![0x20, 0x30, 0x40]),
                (0x20, vec![]),
                (0x30, vec![]),
                (0x40, vec![]),
            ]),
            focus: HashMap::from([(0x10, 3), (0x20, 7), (0x30, 9), (0x40, 9)]),
            order: HashMap::from([(0x30, 5), (0x40, 1)]),
        });
        assert_eq!(scan(&mut e, m, 0x10, i32::MIN), (0x40, 9));
        // A leaf competes by its own value.
        assert_eq!(scan(&mut e, m, 0x20, i32::MIN), (0x20, 7));
        // The value on entry is the one to beat.
        assert_eq!(scan(&mut e, m, 0x20, 8), (0, 8));
    }

    #[test]
    fn the_scan_gives_a_tie_to_the_lower_order_and_skips_hidden_tiles() {
        use std::collections::HashMap;
        let (mut e, m) = scan_world(FocusTiles {
            children: HashMap::from([
                (0x10, vec![0x20, 0x30, 0x40, 0x50]),
                (0x20, vec![]),
                (0x30, vec![]),
                (0x40, vec![]),
                (0x50, vec![]),
            ]),
            focus: HashMap::from([(0x20, 4), (0x30, 4), (0x40, 4), (0x50, 4)]),
            order: HashMap::from([(0x20, 2), (0x30, 6), (0x40, 3), (0x50, 9)]),
        });
        // The parent has no focus of its own. The first child to reach the
        // value wins outright (its order is not remembered); each later
        // equal one wins when its order is below the best order of the
        // ties so far: 0x30 (6 < the initial limit), 0x40 (3 < 6), not
        // 0x50 (9).
        assert_eq!(scan(&mut e, m, 0x10, i32::MIN), (0x40, 4));
        // A hidden tile finds nothing.
        e.register_double(TILE_IS_TRUE, |_, _| Ret::default());
        assert_eq!(scan(&mut e, m, 0x10, i32::MIN), (0, i32::MIN));
    }

    #[test]
    fn the_scan_without_a_tile_starts_at_the_frontmost_menu() {
        use std::collections::HashMap;
        let (mut e, m) = scan_world(FocusTiles {
            children: HashMap::from([(0x10, vec![])]),
            focus: HashMap::from([(0x10, 6)]),
            order: HashMap::new(),
        });
        // No menu in front: nothing.
        returns(&mut e, MENU_MANAGER_INSTANCE, 0x42);
        returns(&mut e, GET_FRONTMOST_MENU, 0);
        assert_eq!(scan(&mut e, m, 0, 3), (0, i32::MIN));
        assert_eq!(calls(&e, MENU_MANAGER_INSTANCE)[0], vec![1]);
        // A menu with a tile of another type: that tile is the root.
        let (tile_object, _) = virtual_object(&mut e, &[(0xc, 0x00aa_0060)], 0x111);
        returns(&mut e, GET_FRONTMOST_MENU, 0xa000);
        returns(&mut e, GET_FIELD_AT_4, tile_object);
        e.register_double(TILE_IS_TRUE, move |_, a| Ret {
            eax: (a[0] == tile_object) as u32,
            ..Ret::default()
        });
        returns(&mut e, TILE_GET_TRAIT_ENTRY, 0x1000_0001);
        returns_st0(&mut e, TRAIT_ENTRY_VALUE, 6.0);
        assert_eq!(scan(&mut e, m, 0, 3), (tile_object, 6));
        assert_eq!(calls(&e, GET_FRONTMOST_MENU)[1], vec![0x42]);
        assert_eq!(calls(&e, GET_FIELD_AT_4)[0], vec![0xa000]);
    }

    #[test]
    fn a_tile_of_type_0x389_must_have_a_menu_in_state_1_or_8() {
        let mut e = world();
        let m = manager(&mut e);
        let (tile_object, _) = virtual_object(&mut e, &[(0xc, 0x00aa_0064)], 0x389);
        returns(&mut e, MENU_MANAGER_INSTANCE, 0x42);
        returns(&mut e, GET_FRONTMOST_MENU, 0xa000);
        returns(&mut e, GET_FIELD_AT_4, tile_object);
        returns(&mut e, TILE_GET_MENU, 0xb000);
        returns(&mut e, TILE_IS_TRUE, 0);
        returns(&mut e, MENU_STATE, 2);
        assert_eq!(scan(&mut e, m, 0, 3), (0, i32::MIN));
        // The state check stopped it: the tile was never asked for 0xfa3.
        assert!(calls(&e, TILE_IS_TRUE).is_empty());
        returns(&mut e, MENU_STATE, 8);
        scan(&mut e, m, 0, 3);
        assert_eq!(calls(&e, TILE_IS_TRUE)[0], vec![tile_object, 0xfa3]);
        returns(&mut e, MENU_STATE, 1);
        scan(&mut e, m, 0, 3);
        assert_eq!(calls(&e, TILE_IS_TRUE).len(), 2);
    }

    /// A timer list: a head node with the first timer at `+0x10` and the
    /// last at `+0xc`; the manager's `pTimers` is the head.
    fn timer_world() -> (Engine, Ptr<InterfaceManager>, u32) {
        let mut e = world();
        let m = manager(&mut e);
        let head = e.mem.alloc(0x14);
        e.mem.set_u32(head + 0xc, head);
        e.set(m, InterfaceManager::pTimers, head);
        returns(&mut e, GET_MANAGER, m.addr());
        (e, m, head)
    }

    fn add_timer(e: &mut Engine, head: u32, index: u32, elapsed: f32, end: f32) -> u32 {
        let timer = e.mem.alloc(0x14);
        e.mem.set_u32(timer, index);
        e.mem.set_f32(timer + 4, elapsed);
        e.mem.set_f32(timer + 8, end);
        let tail = e.mem.u32(head + 0xc);
        e.mem.set_u32(timer + 0xc, tail);
        e.mem.set_u32(tail + 0x10, timer);
        e.mem.set_u32(head + 0xc, timer);
        timer
    }

    /// `FLOAT_MIN` and `FLOAT_MAX` as the game's helpers: smaller and larger
    /// of two floats (the arguments are `float`s; the result is in `ST0`).
    fn clamp_helpers(e: &mut Engine) {
        e.register(FLOAT_MIN, |_, a| Ret {
            st0: f32::from_bits(a[0]).min(f32::from_bits(a[1])) as f64,
            ..Ret::default()
        });
        e.register(FLOAT_MAX, |_, a| Ret {
            st0: f32::from_bits(a[0]).max(f32::from_bits(a[1])) as f64,
            ..Ret::default()
        });
    }

    #[test]
    fn timers_advance_by_the_frame_time_and_finished_ones_are_freed() {
        let (mut e, m, head) = timer_world();
        clamp_helpers(&mut e);
        e.set_global(ONE_DOUBLE, 1.0f64);
        returns_st0(&mut e, FRAME_TIME_GETTER, 0.5);
        returns(&mut e, WEAPON_STATE_GETTER, 0);
        let slow = add_timer(&mut e, head, 1, 0.0, 2.0);
        let done = add_timer(&mut e, head, 2, 0.75, 1.0);
        let last = add_timer(&mut e, head, 3, 0.0, 10.0);
        interface_manager_update_all_timers(&mut e, m);
        assert_eq!(e.mem.f32(slow + 4), 0.5);
        assert_eq!(e.mem.f32(last + 4), 0.5);
        // The finished timer is unlinked and freed.
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![done]]);
        assert_eq!(e.mem.u32(slow + 0x10), last);
        assert_eq!(e.mem.u32(last + 0xc), slow);
        // Finishing the last timer moves the tail back.
        e.mem.set_f32(last + 4, 9.9);
        interface_manager_update_all_timers(&mut e, m);
        assert_eq!(e.mem.u32(head + 0xc), slow);
        assert_eq!(e.mem.u32(slow + 0x10), 0);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 2);
    }

    #[test]
    fn timers_run_at_the_game_rate_in_state_four() {
        let (mut e, m, head) = timer_world();
        clamp_helpers(&mut e);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(TIMER_RATE, 4.0f32);
        returns_st0(&mut e, FRAME_TIME_GETTER, 2.0);
        returns(&mut e, WEAPON_STATE_GETTER, 4);
        let timer = add_timer(&mut e, head, 1, 0.25, 10.0);
        interface_manager_update_all_timers(&mut e, m);
        assert_eq!(e.mem.f32(timer + 4), 0.75);
        assert_eq!(
            calls(&e, WEAPON_STATE_GETTER),
            vec![vec![WEAPON_STATE_OBJECT]]
        );
        assert_eq!(fn_00716440(&mut e), 4.0);
    }

    #[test]
    fn all_the_timers_and_the_head_are_freed() {
        let (mut e, m, head) = timer_world();
        let first = add_timer(&mut e, head, 1, 0.0, 1.0);
        let second = add_timer(&mut e, head, 2, 0.0, 1.0);
        fn_00716450(&mut e, m);
        assert_eq!(
            calls(&e, OPERATOR_DELETE),
            vec![vec![first], vec![second], vec![head]]
        );
        assert_eq!(e.get(m, InterfaceManager::pTimers), 0);
    }

    #[test]
    fn a_new_timer_replaces_the_one_with_its_index_and_joins_the_tail() {
        let (mut e, _, head) = timer_world();
        clamp_helpers(&mut e);
        allocator(&mut e, OPERATOR_NEW);
        e.mem.set_f32(TIMER_DEFAULT_END, 0.5);
        let old = add_timer(&mut e, head, 7, 0.0, 1.0);
        let other = add_timer(&mut e, head, 8, 0.0, 1.0);
        interface_manager_new_timer(&mut e, 7, 3.0);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![old]]);
        let timer = e.mem.u32(head + 0xc);
        assert_ne!(timer, other);
        assert_eq!(e.mem.u32(timer), 7);
        assert_eq!(e.mem.f32(timer + 4), 0.0);
        assert_eq!(e.mem.f32(timer + 8), 3.0);
        assert_eq!(e.mem.u32(timer + 0xc), other);
        assert_eq!(e.mem.u32(other + 0x10), timer);
        // A negative end is raised to 0; the scope guard has the line.
        interface_manager_new_timer(&mut e, 9, -1.0);
        let timer = e.mem.u32(head + 0xc);
        assert_eq!(e.mem.f32(timer + 8), 0.0);
        assert_eq!(calls(&e, SCOPE_GUARD_BEGIN)[0][4], 0x145a);
        assert_eq!(calls(&e, SCOPE_GUARD_END).len(), 2);
    }

    #[test]
    fn clearing_a_timer_unlinks_the_first_with_the_index() {
        let (mut e, _, head) = timer_world();
        let first = add_timer(&mut e, head, 1, 0.0, 1.0);
        let second = add_timer(&mut e, head, 2, 0.0, 1.0);
        let third = add_timer(&mut e, head, 2, 0.0, 1.0);
        interface_manager_clear_timer(&mut e, 2);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![second]]);
        assert_eq!(e.mem.u32(first + 0x10), third);
        assert_eq!(e.mem.u32(third + 0xc), first);
        // Clearing the last moves the tail back.
        interface_manager_clear_timer(&mut e, 2);
        assert_eq!(e.mem.u32(head + 0xc), first);
        assert_eq!(e.mem.u32(first + 0x10), 0);
        // An absent index changes nothing.
        interface_manager_clear_timer(&mut e, 99);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 2);
    }

    #[test]
    fn a_timers_progress_is_clamped_and_one_for_unknown_timers() {
        let (mut e, _, head) = timer_world();
        clamp_helpers(&mut e);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(CURSOR_DIRECTION_Y, -1.0f32);
        add_timer(&mut e, head, 1, 0.5, 2.0);
        add_timer(&mut e, head, 2, 5.0, 2.0);
        add_timer(&mut e, head, 3, 1.0, 0.0);
        assert_eq!(fn_00716660(&mut e, 1), 0.25);
        assert_eq!(fn_00716660(&mut e, 2), 1.0);
        assert_eq!(fn_00716660(&mut e, 3), -1.0);
        assert_eq!(fn_00716660(&mut e, 4), 1.0);
    }

    #[test]
    fn the_key_repeat_starts_at_the_clock() {
        let mut e = world();
        let m = manager(&mut e);
        returns(&mut e, FADE_CLOCK_READ, 1234);
        e.mem.set_u32(m.addr() + 0x158, 9);
        fn_007166f0(&mut e, m, 0x8000_0001);
        assert_eq!(e.get(m, InterfaceManager::uKeyDownTime), 1234);
        assert_eq!(e.mem.u32(m.addr() + 0x158), 0);
        assert_eq!(
            e.get(m, InterfaceManager::iRepeatingKey),
            0x8000_0001u32 as i32
        );
        assert_eq!(calls(&e, FADE_CLOCK_READ), vec![vec![FADE_CLOCK]]);
    }

    /// A world for the key repeat: the clock reads `now`, the first delay
    /// is `first`, the second `next`.
    fn repeat_world(now: u32, first: f32, next: f32) -> (Engine, Ptr<InterfaceManager>) {
        let mut e = world();
        let m = manager(&mut e);
        returns(&mut e, FADE_CLOCK_READ, now);
        let first_cell = float_cell(&mut e, first);
        let next_cell = float_cell(&mut e, next);
        e.register_double(FLOAT_HOLDER_GET, move |_, a| Ret {
            eax: if a[0] == REPEAT_DELAY_HOLDER_FIRST {
                first_cell
            } else {
                next_cell
            },
            ..Ret::default()
        });
        returns(&mut e, CONTROLS_GET, 0xc0de);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        (e, m)
    }

    #[test]
    fn the_first_repeat_waits_for_the_first_delay() {
        let (mut e, m) = repeat_world(1000, 300.0, 50.0);
        e.set(m, InterfaceManager::iRepeatingKey, 0x8000_0000u32 as i32);
        e.set(m, InterfaceManager::uKeyDownTime, 800);
        assert_eq!(fn_00716730(&mut e, m, 1.0), 0);
        assert_eq!(e.mem.u32(m.addr() + 0x158), 0);
        e.set(m, InterfaceManager::uKeyDownTime, 700);
        assert_eq!(fn_00716730(&mut e, m, 1.0), 0x8000_0000);
        assert_eq!(e.mem.u32(m.addr() + 0x158), 1000);
        // Later ones come at the second delay divided by the rate.
        assert_eq!(fn_00716730(&mut e, m, 1.0), 0);
        let (mut e, m) = repeat_world(1000, 300.0, 50.0);
        e.set(m, InterfaceManager::iRepeatingKey, 0x8000_0000u32 as i32);
        e.mem.set_u32(m.addr() + 0x158, 950);
        assert_eq!(fn_00716730(&mut e, m, 1.0), 0x8000_0000);
        e.mem.set_u32(m.addr() + 0x158, 960);
        assert_eq!(fn_00716730(&mut e, m, 0.5), 0);
        assert_eq!(e.mem.u32(m.addr() + 0x158), 960);
        e.mem.set_u32(m.addr() + 0x158, 890);
        assert_eq!(fn_00716730(&mut e, m, 0.5), 0x8000_0000);
        assert_eq!(e.mem.u32(m.addr() + 0x158), 1000);
    }

    #[test]
    fn a_negative_first_delay_turns_the_repeat_off() {
        let (mut e, m) = repeat_world(1000, -1.0, 50.0);
        e.set(m, InterfaceManager::iRepeatingKey, 5);
        e.set(m, InterfaceManager::uKeyDownTime, 0);
        assert_eq!(fn_00716730(&mut e, m, 1.0), 0);
        assert_eq!(e.get(m, InterfaceManager::iRepeatingKey), 5);
    }

    #[test]
    fn a_released_arrow_key_ends_the_repeat() {
        for (code, key) in [
            (0x8000_0001u32, 0xcbu32),
            (0x8000_0002, 0xcd),
            (0x8000_0003, 0xc8),
            (0x8000_0004, 0xd0),
        ] {
            let (mut e, m) = repeat_world(1000, 300.0, 50.0);
            e.set_global(CONTROLS_OWNER, 0x1234u32);
            e.set(m, InterfaceManager::iRepeatingKey, code as i32);
            e.set(m, InterfaceManager::uKeyDownTime, 0);
            // Still held: the repeat goes on.
            returns(&mut e, CONTROLS_QUERY_00A24180, 1);
            assert_eq!(fn_00716730(&mut e, m, 1.0), code);
            assert_eq!(
                calls(&e, CONTROLS_QUERY_00A24180),
                vec![vec![0xc0de, key, 0]]
            );
            // Released: it ends.
            returns(&mut e, CONTROLS_QUERY_00A24180, 0);
            assert_eq!(fn_00716730(&mut e, m, 1.0), 0);
            assert_eq!(e.get(m, InterfaceManager::iRepeatingKey), 0);
        }
    }

    #[test]
    fn a_tile_of_another_menu_is_refused_by_a_top_menu_of_the_two_kinds() {
        let mut e = world();
        let m = manager(&mut e);
        set_stack(&mut e, m, &[4, 6]);
        returns(&mut e, TILE_GET_MENU_BY_CLASS, 0x5000);
        returns_st0(&mut e, TILE_GET_VALUE, 0x66 as f64);
        e.register(FTOL, |_, a| Ret {
            eax: f64::from_bits(((a[1] as u64) << 32) | a[0] as u64) as i32 as u32,
            ..Ret::default()
        });
        e.register(TILE_GET_MENU, |_, a| Ret {
            eax: if a[0] == 0x5000 { 0xaaa } else { 0xbbb },
            ..Ret::default()
        });
        assert!(!interface_manager_tile_is_accepting_events(
            &mut e, m, 0x6000
        ));
        assert_eq!(calls(&e, TILE_GET_MENU_BY_CLASS)[0], vec![6]);
        // A tile of the same menu is accepted.
        assert!(interface_manager_tile_is_accepting_events(
            &mut e, m, 0x5000
        ));
        // The other kind is the same.
        returns_st0(&mut e, TILE_GET_VALUE, 0x1776 as f64);
        assert!(!interface_manager_tile_is_accepting_events(
            &mut e, m, 0x6000
        ));
        // Any other kind accepts everything.
        returns_st0(&mut e, TILE_GET_VALUE, 5.0);
        assert!(interface_manager_tile_is_accepting_events(
            &mut e, m, 0x6000
        ));
        // No menu for the top class: accepted.
        returns(&mut e, TILE_GET_MENU_BY_CLASS, 0);
        assert!(interface_manager_tile_is_accepting_events(
            &mut e, m, 0x6000
        ));
    }
    /// The strings a text-entry test world keeps: (string object, text).
    type StringTable = Rc<RefCell<Vec<(u32, Vec<u8>)>>>;

    /// A text entry whose string members are doubles over a table: `Set`
    /// stores the text a pointer names, `GetLength` and `c_str` read it back.
    fn text_world() -> (Engine, Ptr<TextEntry>, StringTable) {
        let mut e = world();
        let entry = e.new_object::<TextEntry>();
        e.set(entry, TextEntry::max_width, -1);
        let strings: StringTable = Rc::new(RefCell::new(vec![]));
        let table = strings.clone();
        e.register_double(STRING_SET, move |e, a| {
            let text = if a[1] == 0 { vec![] } else { e.mem.cstr(a[1]) };
            let mut table = table.borrow_mut();
            table.retain(|(this, _)| *this != a[0]);
            table.push((a[0], text));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let table = strings.clone();
        e.register_double(STRING_LENGTH, move |_, a| Ret {
            eax: table
                .borrow()
                .iter()
                .find(|(this, _)| *this == a[0])
                .map_or(0, |(_, text)| text.len() as u32),
            ..Ret::default()
        });
        let table = strings.clone();
        e.register_double(NI_POINTER_GET, move |e, a| {
            let found = table
                .borrow()
                .iter()
                .find(|(this, _)| *this == a[0])
                .map(|(_, text)| text.clone());
            let block = e.mem.alloc(0x400);
            e.mem.set_cstr(block, &found.unwrap_or_default());
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        e.register(TEXT_COPY, |e, a| {
            let text = e.mem.cstr(a[1]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.mem.set_cstr(EMPTY_TEXT, b"");
        (e, entry, strings)
    }

    fn entry_text(strings: &StringTable, at: u32) -> Option<String> {
        strings
            .borrow()
            .iter()
            .find(|(this, _)| *this == at)
            .map(|(_, text)| String::from_utf8_lossy(text).into_owned())
    }

    #[test]
    fn a_new_text_entry_is_inactive_without_a_width_limit() {
        let mut e = world();
        let entry = e.new_object::<TextEntry>();
        e.set(entry, TextEntry::cursor, 5);
        e.set(entry, TextEntry::caret_phase, 1);
        assert_eq!(fn_00716980(&mut e, entry), entry);
        assert_eq!(e.get(entry, TextEntry::active), 0);
        assert_eq!(e.get(entry, TextEntry::cursor), 0);
        assert_eq!(e.get(entry, TextEntry::caret_phase), 0);
        assert_eq!(e.get(entry, TextEntry::max_width), -1);
        assert_eq!(e.get(entry, TextEntry::last_blink_time), 0);
        assert_eq!(e.get(entry, TextEntry::field_18), 1);
        assert_eq!(
            calls(&e, STRING_CONSTRUCT),
            vec![vec![entry.addr()], vec![entry.addr() + 8]]
        );
    }

    #[test]
    fn the_text_entry_destructor_frees_the_shown_string_first() {
        let mut e = world();
        let entry = e.new_object::<TextEntry>();
        fn_00716a10(&mut e, entry);
        assert_eq!(
            calls(&e, STRING_DESTROY),
            vec![vec![entry.addr() + 8], vec![entry.addr()]]
        );
    }

    #[test]
    fn setting_the_text_sets_both_strings() {
        let mut e = world();
        let entry = e.new_object::<TextEntry>();
        fn_00716a70(&mut e, entry, 0x4321);
        assert_eq!(
            calls(&e, STRING_SET),
            vec![vec![entry.addr(), 0x4321], vec![entry.addr() + 8, 0x4321]]
        );
    }

    #[test]
    fn the_width_limit_is_five_less_or_the_screen_width() {
        let mut e = world();
        screen_constants(&mut e);
        let entry = e.new_object::<TextEntry>();
        fn_00716aa0(&mut e, entry, 105);
        assert_eq!(e.get(entry, TextEntry::max_width), 100);
        // Negative: the interface width, as an integer.
        returns(&mut e, RENDER_TARGET_WIDTH, 1024);
        returns(&mut e, RENDER_TARGET_HEIGHT, 768);
        returns(&mut e, FTOL, 1280);
        fn_00716aa0(&mut e, entry, 2);
        assert_eq!(e.get(entry, TextEntry::max_width), 1280);
        assert_eq!(calls(&e, FTOL), vec![vec![0, 0x4094_0000]]);
        assert_eq!(calls(&e, GET_MANAGER).len(), 1);
    }

    #[test]
    fn the_active_flag_is_read() {
        let mut e = world();
        let entry = e.new_object::<TextEntry>();
        assert_eq!(fn_00716ae0(&mut e, entry), 0);
        e.set(entry, TextEntry::active, 1);
        assert_eq!(fn_00716ae0(&mut e, entry), 1);
    }

    fn key(e: &mut Engine, entry: Ptr<TextEntry>, code: u32) {
        fn_00716b00(e, entry, code);
    }

    #[test]
    fn typing_inserts_at_the_caret_and_moves_it() {
        let (mut e, entry, strings) = text_world();
        // Inactive entries take nothing.
        key(&mut e, entry, b'a' as u32);
        assert_eq!(entry_text(&strings, entry.addr()), None);
        e.set(entry, TextEntry::active, 1);
        key(&mut e, entry, b'a' as u32);
        key(&mut e, entry, b'c' as u32);
        assert_eq!(entry_text(&strings, entry.addr()).unwrap(), "ac");
        assert_eq!(e.get(entry, TextEntry::cursor), 2);
        // Left, then insert in the middle.
        key(&mut e, entry, 0x8000_0001);
        key(&mut e, entry, b'b' as u32);
        assert_eq!(entry_text(&strings, entry.addr()).unwrap(), "abc");
        assert_eq!(e.get(entry, TextEntry::cursor), 2);
        // The up and down codes are inserted as bytes, like any other code.
        key(&mut e, entry, 0x8000_0003);
        assert_eq!(
            entry_text(&strings, entry.addr()).unwrap().as_bytes(),
            b"ab\x03c"
        );
    }

    #[test]
    fn the_caret_moves_with_the_arrows_home_and_end() {
        let (mut e, entry, strings) = text_world();
        e.set(entry, TextEntry::active, 1);
        for byte in b"abc" {
            key(&mut e, entry, *byte as u32);
        }
        e.set(entry, TextEntry::clears_text_on_edit, 1);
        key(&mut e, entry, 0x8000_0001);
        assert_eq!(e.get(entry, TextEntry::cursor), 2);
        assert_eq!(e.get(entry, TextEntry::clears_text_on_edit), 0);
        key(&mut e, entry, 0x8000_0005);
        assert_eq!(e.get(entry, TextEntry::cursor), 0);
        // Left at the start and right at the end stay.
        key(&mut e, entry, 0x8000_0001);
        assert_eq!(e.get(entry, TextEntry::cursor), 0);
        key(&mut e, entry, 0x8000_0002);
        assert_eq!(e.get(entry, TextEntry::cursor), 1);
        key(&mut e, entry, 0x8000_0006);
        assert_eq!(e.get(entry, TextEntry::cursor), 3);
        key(&mut e, entry, 0x8000_0002);
        assert_eq!(e.get(entry, TextEntry::cursor), 3);
        assert_eq!(entry_text(&strings, entry.addr()).unwrap(), "abc");
    }

    #[test]
    fn backspace_and_delete_remove_one_character() {
        let (mut e, entry, strings) = text_world();
        e.set(entry, TextEntry::active, 1);
        for byte in b"abcd" {
            key(&mut e, entry, *byte as u32);
        }
        key(&mut e, entry, 0x8000_0001);
        // Backspace removes the character before the caret.
        key(&mut e, entry, 0x8000_0000);
        assert_eq!(entry_text(&strings, entry.addr()).unwrap(), "abd");
        assert_eq!(e.get(entry, TextEntry::cursor), 2);
        // Delete removes the one at the caret.
        key(&mut e, entry, 0x8000_0007);
        assert_eq!(entry_text(&strings, entry.addr()).unwrap(), "ab");
        assert_eq!(e.get(entry, TextEntry::cursor), 2);
        // ... and nothing at the end.
        key(&mut e, entry, 0x8000_0007);
        assert_eq!(entry_text(&strings, entry.addr()).unwrap(), "ab");
        // Backspace at the start does nothing.
        key(&mut e, entry, 0x8000_0005);
        key(&mut e, entry, 0x8000_0000);
        assert_eq!(entry_text(&strings, entry.addr()).unwrap(), "ab");
    }

    #[test]
    fn a_flagged_entry_is_emptied_by_the_next_editing_key() {
        let (mut e, entry, strings) = text_world();
        e.set(entry, TextEntry::active, 1);
        for byte in b"abc" {
            key(&mut e, entry, *byte as u32);
        }
        for code in [0x8000_0000u32, 0x8000_0007, 0x8000_0008] {
            e.set(entry, TextEntry::clears_text_on_edit, 1);
            key(&mut e, entry, code);
            assert_eq!(entry_text(&strings, entry.addr()).unwrap(), "", "{code:x}");
            assert_eq!(e.get(entry, TextEntry::clears_text_on_edit), 0);
            assert_eq!(e.get(entry, TextEntry::cursor), 0);
            assert_eq!(e.get(entry, TextEntry::active), 1);
            for byte in b"abc" {
                key(&mut e, entry, *byte as u32);
            }
        }
        // Typing replaces the text.
        e.set(entry, TextEntry::clears_text_on_edit, 1);
        key(&mut e, entry, b'z' as u32);
        assert_eq!(entry_text(&strings, entry.addr()).unwrap(), "z");
        assert_eq!(e.get(entry, TextEntry::cursor), 1);
        assert_eq!(e.get(entry, TextEntry::clears_text_on_edit), 0);
    }

    #[test]
    fn enter_deactivates_and_the_page_codes_only_clear_the_flag() {
        let (mut e, entry, strings) = text_world();
        e.set(entry, TextEntry::active, 1);
        key(&mut e, entry, b'q' as u32);
        for code in [0x8000_0009u32, 0x8000_000a] {
            e.set(entry, TextEntry::clears_text_on_edit, 1);
            key(&mut e, entry, code);
            assert_eq!(e.get(entry, TextEntry::clears_text_on_edit), 0);
            assert_eq!(e.get(entry, TextEntry::active), 1);
        }
        // The code 9 is ignored altogether.
        e.set(entry, TextEntry::clears_text_on_edit, 1);
        key(&mut e, entry, 9);
        assert_eq!(e.get(entry, TextEntry::clears_text_on_edit), 1);
        e.set(entry, TextEntry::clears_text_on_edit, 0);
        key(&mut e, entry, 0x8000_0008);
        assert_eq!(e.get(entry, TextEntry::active), 0);
        assert_eq!(entry_text(&strings, entry.addr()).unwrap(), "q");
    }

    #[test]
    fn typing_stops_at_the_width_limit() {
        let (mut e, entry, strings) = text_world();
        e.set(entry, TextEntry::active, 1);
        e.set(entry, TextEntry::max_width, 100);
        returns(&mut e, TEXT_ENTRY_ACCEPTS_TEXT, 0);
        key(&mut e, entry, b'a' as u32);
        assert_eq!(entry_text(&strings, entry.addr()), None);
        assert_eq!(e.get(entry, TextEntry::cursor), 0);
        assert_eq!(calls(&e, TEXT_ENTRY_ACCEPTS_TEXT).len(), 1);
        assert_eq!(calls(&e, TEXT_ENTRY_ACCEPTS_TEXT)[0][0], entry.addr());
        returns(&mut e, TEXT_ENTRY_ACCEPTS_TEXT, 1);
        key(&mut e, entry, b'a' as u32);
        assert_eq!(entry_text(&strings, entry.addr()).unwrap(), "a");
        assert_eq!(e.get(entry, TextEntry::cursor), 1);
    }

    #[test]
    fn activating_an_entry_puts_the_caret_at_the_end() {
        let (mut e, entry, _) = text_world();
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"abcd");
        e.call(STRING_SET, &args![entry, text]);
        fn_00717010(&mut e, entry, 1);
        assert_eq!(e.get(entry, TextEntry::active), 1);
        assert_eq!(e.get(entry, TextEntry::cursor), 4);
        // Already active: the caret stays.
        e.set(entry, TextEntry::cursor, 1);
        fn_00717010(&mut e, entry, 1);
        assert_eq!(e.get(entry, TextEntry::cursor), 1);
        // Deactivating leaves the caret where it is.
        fn_00717010(&mut e, entry, 0);
        assert_eq!(e.get(entry, TextEntry::active), 0);
        assert_eq!(e.get(entry, TextEntry::cursor), 1);
        // Deactivating an inactive entry changes nothing.
        fn_00717010(&mut e, entry, 0);
        assert_eq!(e.get(entry, TextEntry::cursor), 1);
    }

    #[test]
    fn the_caret_blinks_every_half_second() {
        let mut e = world();
        let entry = e.new_object::<TextEntry>();
        e.set(entry, TextEntry::last_blink_time, 1000);
        returns(&mut e, FADE_CLOCK_READ, 1500);
        fn_00717050(&mut e, entry);
        assert_eq!(e.get(entry, TextEntry::caret_phase), 0);
        assert_eq!(e.get(entry, TextEntry::last_blink_time), 1000);
        returns(&mut e, FADE_CLOCK_READ, 1501);
        fn_00717050(&mut e, entry);
        assert_eq!(e.get(entry, TextEntry::caret_phase), 1);
        assert_eq!(e.get(entry, TextEntry::last_blink_time), 1501);
        returns(&mut e, FADE_CLOCK_READ, 2100);
        fn_00717050(&mut e, entry);
        assert_eq!(e.get(entry, TextEntry::caret_phase), 0);
        assert_eq!(calls(&e, FADE_CLOCK_READ)[0], vec![FADE_CLOCK]);
    }

    #[test]
    fn the_shown_text_answers_its_pointer() {
        let (mut e, entry, _strings) = text_world();
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"abc");
        e.call(STRING_SET, &args![entry, text]);
        e.mem.set_cstr(DISPLAY_FALLBACK_STRING, b"-");
        // The exe ends on the shown string's pointer getter and keeps its EAX.
        let shown = e.call(0x0071_70a0, &args![entry]).u32();
        assert_eq!(e.mem.cstr(shown), b"abc".to_vec());
        assert_eq!(
            calls(&e, NI_POINTER_GET).last().unwrap()[0],
            entry.addr() + 8
        );
    }

    #[test]
    fn the_shown_text_has_the_caret_in_it() {
        let (mut e, entry, strings) = text_world();
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"abc");
        e.call(STRING_SET, &args![entry, text]);
        e.mem.set_cstr(DISPLAY_FALLBACK_STRING, b"-");
        e.set(entry, TextEntry::active, 1);
        e.set(entry, TextEntry::cursor, 1);
        e.set(entry, TextEntry::caret_phase, 1);
        fn_007170a0(&mut e, entry);
        assert_eq!(entry_text(&strings, entry.addr() + 8).unwrap(), "a|bc");
        // The other phase shows 0x7f; the caret may be at the end.
        e.set(entry, TextEntry::caret_phase, 0);
        e.set(entry, TextEntry::cursor, 3);
        fn_007170a0(&mut e, entry);
        assert_eq!(
            entry_text(&strings, entry.addr() + 8).unwrap().as_bytes(),
            b"abc\x7f"
        );
        // An inactive entry shows the text alone.
        e.set(entry, TextEntry::active, 0);
        fn_007170a0(&mut e, entry);
        assert_eq!(entry_text(&strings, entry.addr() + 8).unwrap(), "abc");
        // Nothing to show: the fallback string.
        e.call(STRING_SET, &args![entry, 0u32]);
        fn_007170a0(&mut e, entry);
        assert_eq!(
            calls(&e, STRING_SET).last().unwrap()[1],
            DISPLAY_FALLBACK_STRING
        );
    }
    /// The local the next `with_stack(size)` on this thread takes (asked of the engine).
    fn next_frame(e: &mut Engine, size: u32) -> u32 {
        e.next_stack_local(size)
    }

    #[test]
    fn the_gamepad_event_locals_are_at_the_frame_offsets_of_0070f6e0() {
        // A direction: the trait at +0x54, the visited list at +0x3c, the current tile at +0x50
        // and the candidate at +0x38 of the 0x84-byte frame.
        let (mut e, m, _) = menu_world();
        returns(&mut e, GET_MOUSE_OVER_TARGET, 0x9400);
        e.register(TILE_GET_FIRST_REF_COPY, |e, a| {
            e.mem.set_u32(a[2], 0xfc3);
            Ret {
                eax: 0x9500,
                ..Ret::default()
            }
        });
        returns(&mut e, TILE_IS_TRUE, 1);
        let frame = next_frame(&mut e, 0x84);
        interface_manager_do_gamepad_event(&mut e, m, 1);
        assert_eq!(calls(&e, TILE_GET_FIRST_REF_COPY)[0][2], frame + 0x54);
        assert_eq!(calls(&e, SIMPLE_LIST_CONSTRUCT), vec![vec![frame + 0x3c]]);
        assert_eq!(
            calls(&e, LIST_PUSH_FRONT_005AE3D0),
            vec![vec![frame + 0x3c, frame + 0x50]]
        );
        assert_eq!(
            calls(&e, LIST_CONTAINS)[0],
            vec![frame + 0x3c, frame + 0x38]
        );
        // No focus: the seed at +0x44.
        let (mut e, m, _) = menu_world();
        returns(&mut e, GET_MOUSE_OVER_TARGET, 0);
        returns(&mut e, TILE_GET_FIRST_REF_COPY, 0);
        let frame = next_frame(&mut e, 0x84);
        interface_manager_do_gamepad_event(&mut e, m, 3);
        assert_eq!(calls(&e, SCAN_FOR_MAX_FOCUS)[0][1], frame + 0x44);
        // A button: the trait at +0x1c.
        let (mut e, m, _) = menu_world();
        returns(&mut e, GET_MOUSE_OVER_TARGET, 0x9600);
        returns(&mut e, TILE_IS_VISIBLE, 1);
        let frame = next_frame(&mut e, 0x84);
        interface_manager_do_gamepad_event(&mut e, m, 10);
        assert_eq!(calls(&e, TILE_GET_FIRST_REF_COPY)[0][2], frame + 0x1c);
    }
    // @@TESTS-END@@
}
