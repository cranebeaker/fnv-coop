//! `fallout/interface/menus/roulettemenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The chained container's destructor body (`0046ffb0`, `thiscall`, no stack words).
const CONTAINER_DESTRUCT: u32 = 0x0046_ffb0;

// Translated from 007b9000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): a forwarder that runs the chained
/// container's destructor body (`0046ffb0`) on `this`.
pub fn fn_007b9000(e: &mut Engine, this: Ptr) {
    e.call(CONTAINER_DESTRUCT, &args![this]);
}

layout! {
    /// `RouletteMenu` (Xbox PDB), 0x4014 bytes on PC; the offsets below are the PC code's
    /// (`pTiles` at +0x2c, eleven of them; the bet tile records at +0x5c, 0x64 bytes each).
    pub struct RouletteMenu: 0x4014 {
        /// `eRouletteState` (Xbox PDB).
        0x28 eRouletteState: i32,
        /// `uiPrevIdleTick` (Xbox PDB).
        0x58 uiPrevIdleTick: u32,
        /// `pBetList` (Xbox PDB): a `BSSimpleList<RouletteMenu::RouletteBet*>*`.
        0x3e78 pBetList: u32,
        /// `pRouletteTableGeometry` (Xbox PDB).
        0x3f14 pRouletteTableGeometry: u32,
        /// `pRouletteGridGeometry` (Xbox PDB).
        0x3f18 pRouletteGridGeometry: u32,
        /// `pCursorGeometry` (Xbox PDB).
        0x3f1c pCursorGeometry: u32,
        /// `pCurrentBetFace` (Xbox PDB).
        0x3f70 pCurrentBetFace: u32,
        /// `pValidBetFace` (Xbox PDB).
        0x3f74 pValidBetFace: u32,
        /// `pInvalidBetFace` (Xbox PDB).
        0x3f78 pInvalidBetFace: u32,
        /// `pTableAnimManager` (Xbox PDB).
        0x3f80 pTableAnimManager: u32,
        /// `fRadius` (Xbox PDB).
        0x3f90 fRadius: f32,
        /// `fValidRadius` (Xbox PDB).
        0x3f94 fValidRadius: f32,
        /// `iNextModelIndex` (Xbox PDB).
        0x3fe0 iNextModelIndex: i32,
        /// `iNumBetsSet` (Xbox PDB).
        0x3fe4 iNumBetsSet: i32,
        /// `iCurrentBet` (Xbox PDB).
        0x3fe8 iCurrentBet: i32,
        /// `iTotalBet` (Xbox PDB).
        0x3fec iTotalBet: i32,
        /// `iChipCount` (Xbox PDB).
        0x3ff0 iChipCount: i32,
        /// `iLuckValue` (Xbox PDB).
        0x3ff4 iLuckValue: i32,
        /// `iTotalWinnings` (Xbox PDB).
        0x3ff8 iTotalWinnings: i32,
        /// `iMinBet` (Xbox PDB).
        0x3ffc iMinBet: i32,
        /// `iMaxBet` (Xbox PDB).
        0x4000 iMaxBet: i32,
        /// `iResultValue` (Xbox PDB).
        0x4004 iResultValue: i32,
        /// `iSpinAnimIndex` (Xbox PDB).
        0x4008 iSpinAnimIndex: i32,
        /// `bCleanTable` (Xbox PDB).
        0x400c bCleanTable: bool,
        /// `bLucky` (Xbox PDB).
        0x400d bLucky: bool,
        /// `bUnlucky` (Xbox PDB).
        0x400e bUnlucky: bool,
        /// `bRunScriptOnExit` (Xbox PDB).
        0x400f bRunScriptOnExit: bool,
        /// `iCheatToggle` (Xbox PDB).
        0x4010 iCheatToggle: i32,
    }
}

/// `pTiles` (Xbox PDB, +0x2c): the menu's eleven tiles.
const TILES: u32 = 0x2c;
/// `betTileInfoArray` (Xbox PDB): 0x9f records of 0x64 bytes from +0x5c: the numbers the
/// bet covers (a list at +0), the colour (+8), the payout (+0xc), the position (+0x10), the
/// node name (+0x1c, 8 bytes) and the description (+0x24, 0x40 bytes).
const BET_TILES: u32 = 0x5c;
const BET_TILE_SIZE: u32 = 0x64;
const BET_TILE_COUNT: u32 = 0x9f;
/// `iNumberStrip` (Xbox PDB, +0x3e7c): the wheel's 38 numbers in order.
const NUMBER_STRIP: u32 = 0x3e7c;
/// `pBetGeometry` and `pBetShadowGeometry` (Xbox PDB, ten each at +0x3f20 and +0x3f48),
/// `pTableAnimations` (three at +0x3f84 on PC), `pWheelTexture` (+0x3f7c) and
/// `fSpecialRadius` (+0x3f98).
const BET_GEOMETRY: u32 = 0x3f20;
const BET_SHADOW_GEOMETRY: u32 = 0x3f48;
const TABLE_ANIMATIONS: u32 = 0x3f84;
const WHEEL_TEXTURE: u32 = 0x3f7c;
const SPECIAL_RADIUS: u32 = 0x3f98;

/// `RouletteMenu::pMe`, `pCasinoData` and `pCasino` (Xbox PDB statics; `011da878`,
/// `011da87c`, `011da880`), the model node and camera (`spModel`, `sp3DCamera`: the
/// `NiPointer`s at `011da8a0` and `011da8d0`) and the trait id at `011da870`.
const ROULETTE_MENU_011DA878: u32 = 0x011d_a878;
const CASINO_DATA_011DA87C: u32 = 0x011d_a87c;
const CASINO_011DA880: u32 = 0x011d_a880;
const MODEL_011DA8A0: u32 = 0x011d_a8a0;
const CAMERA_011DA8D0: u32 = 0x011d_a8d0;
const TRAIT_011DA870: u32 = 0x011d_a870;
/// The player (`PlayerCharacter*`) and the model loader (`011c3b3c`).
const PLAYER_011DEA3C: u32 = 0x011d_ea3c;
const MODEL_LOADER_011C3B3C: u32 = 0x011c_3b3c;
/// The menu's vtable (`01075f7c`).
const ROULETTE_MENU_VTABLE: u32 = 0x0107_5f7c;

/// `MemoryManager` free (`00401030`, `cdecl`).
const FREE: u32 = 0x0040_1030;
/// `00969b20` (`thiscall` on the player).
const PLAYER_00969B20: u32 = 0x0096_9b20;
/// `00504e00` (`thiscall` on the casino, `RET 4`: one of its models, whose vtable +0x14
/// gives the path).
const CASINO_MODEL_00504E00: u32 = 0x0050_4e00;
/// `ModelLoader::LoadFile` (`00447080`, `RET 0x18`) and `0045a5e0` (`thiscall` on the
/// loader, `RET 4`, a path).
const MODEL_LOADER_LOAD_FILE: u32 = 0x0044_7080;
const MODEL_LOADER_0045A5E0: u32 = 0x0045_a5e0;
/// The CRT string helpers: `sprintf_s` (`00406d00`), `strcpy_s` (`00406d30`), `strlen`
/// (`0044a670`) and the copy (`00401460`), all `cdecl`.
const SPRINTF_S: u32 = 0x0040_6d00;
const STRCPY_S: u32 = 0x0040_6d30;
const STRLEN: u32 = 0x0044_a670;
const COPY_00401460: u32 = 0x0040_1460;
/// `NiPointer` assignment (`0066b0d0`, `RET 4`), `00559450` (`thiscall`: the word at +0, an
/// `NiPointer`'s object or a string's text) and the `NiPointer` destructor (`0045cec0`).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const WORD_AT_0_00559450: u32 = 0x0055_9450;
const NI_POINTER_DESTROY: u32 = 0x0045_cec0;
/// `00450b80` (`cdecl`, a word: the scene node), `00b5d180` and `00572160` (`thiscall`).
const SCENE_00450B80: u32 = 0x0045_0b80;
const SCENE_00B5D180: u32 = 0x00b5_d180;
const NODE_00572160: u32 = 0x0057_2160;
/// List helpers: empty (`008256d0`), free the nodes (`00470470`), the list's deleting
/// destructor (`004702f0`, `RET 4`), append the pointed-to word (`005ae3d0`, `RET 4`).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_FREE_NODES: u32 = 0x0047_0470;
const LIST_DELETE: u32 = 0x0047_02f0;
const LIST_APPEND: u32 = 0x005a_e3d0;
/// The list constructor (`0096a2d0`) and `006815c0` (`thiscall`, returns `this`).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const NODE_SELF: u32 = 0x0068_15c0;
/// `eh vector destructor iterator` (`00ec5fce`, `stdcall`, four words) and the menu's base
/// destructor (`00a1c520`).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
const MENU_DESTRUCT: u32 = 0x00a1_c520;
/// `InterfaceManager` (`004b7210`), its word at +0x28 (`0045cd60`: the cursor tile) and
/// `Tile::SetFloat` (`00a012d0`, `RET 0xc`), `Tile::SetString` (`00a01350`, `RET 0xc`).
const INTERFACE_MANAGER: u32 = 0x004b_7210;
const WORD_AT_28_0045CD60: u32 = 0x0045_cd60;
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const TILE_SET_STRING: u32 = 0x00a0_1350;
/// The scene graph helpers: `004bc1f0` (`RET 0xc`, three `float`s: the local translation),
/// `004aae30` (`cdecl`: a child by name), `00450f90` (`RET 4`), `NiObject::Clone` (ov2,
/// `00a5d680`), `00aa13e0` (`cdecl`: the Gamebryo allocation), `00a5ecb0` (`RET 4`: the
/// node constructor), `0043d410` (`RET 0xc`: a point from three words), `00a59c60` (`RET
/// 4`: the translation from a point), `NiObjectNET::GetController` (`00a5c570`, `RET 4`),
/// `NiAVObject::GetWorldBound` (`0043d450`), the bound's radius (`0084d030`, `ST0`),
/// `0043c490` (the local translation's address), `004a0c90` (`RET 4`: a rotation by an
/// angle), `0043fa80` (`RET 4`: the rotation set), `ShadowSceneNode::AddAllLightsFromNode_Recurse`
/// (`00b5ca70`, `RET 8`).
const SET_TRANSLATE_004BC1F0: u32 = 0x004b_c1f0;
const GET_OBJECT_BY_NAME: u32 = 0x004a_ae30;
const SET_APP_CULLED_00450F90: u32 = 0x0045_0f90;
const NI_OBJECT_CLONE: u32 = 0x00a5_d680;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
const POINT_CONSTRUCT_0043D410: u32 = 0x0043_d410;
const SET_TRANSLATE_00A59C60: u32 = 0x00a5_9c60;
const GET_CONTROLLER: u32 = 0x00a5_c570;
const GET_WORLD_BOUND: u32 = 0x0043_d450;
const BOUND_RADIUS_0084D030: u32 = 0x0084_d030;
const LOCAL_TRANSLATE_0043C490: u32 = 0x0043_c490;
const ROTATION_004A0C90: u32 = 0x004a_0c90;
const SET_ROTATE_0043FA80: u32 = 0x0043_fa80;
const ADD_ALL_LIGHTS_FROM_NODE: u32 = 0x00b5_ca70;
/// `NiFixedString` from text (`00438170`, `RET 4`) and its destructor (`004381b0`),
/// `NiControllerManager::GetSequenceByName`-like `0047a520` (`RET 4`),
/// `NiControllerManager::DeactivateAll` (`0048fef0`, `RET 4`) and
/// `NiTimeController::StartAnimations` (`00a6d2d0`, `cdecl`).
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTROY: u32 = 0x0043_81b0;
const SEQUENCE_BY_NAME_0047A520: u32 = 0x0047_a520;
const DEACTIVATE_ALL: u32 = 0x0048_fef0;
const START_ANIMATIONS: u32 = 0x00a6_d2d0;
/// The point difference (`00439ef0`, `RET 8`), the point at +0x8c of a node (`0045bb80`)
/// and a point's length (`00457990`, `ST0`).
const POINT_SUBTRACT_00439EF0: u32 = 0x0043_9ef0;
const WORLD_TRANSLATE_0045BB80: u32 = 0x0045_bb80;
const POINT_LENGTH_00457990: u32 = 0x0045_7990;
/// `00944460` (`cdecl`, two words: a random number in a range).
const RANDOM_00944460: u32 = 0x0094_4460;
/// `0073ac90` (a list's first data slot), `006317a0` (`RET 4`: the data slot of a node) and
/// `007b52d0` (`RET 4`: the next node).
const LIST_HEAD_SLOT_0073AC90: u32 = 0x0073_ac90;
const NODE_SLOT_006317A0: u32 = 0x0063_17a0;
const NEXT_NODE_007B52D0: u32 = 0x007b_52d0;
/// `MapMarkerData::GetLocationName`-named string text (`00408da0`) and the game setting
/// string (`00403df0`), both `thiscall`.
const NAME_CHARS_00408DA0: u32 = 0x0040_8da0;
const SETTING_STRING: u32 = 0x0040_3df0;
/// The slot machine's camera maker (`007c2180`, `thiscall`).
const MAKE_CAMERA_007C2180: u32 = 0x007c_2180;

/// Paths, names and formats (`.rdata`).
const GRID_MODEL_PATH: u32 = 0x0107_5fc4;
const CURSOR_MODEL_PATH: u32 = 0x0107_5ff8;
const SHADOW_SUFFIX: u32 = 0x0107_0944;
const VALID_FACE_NAME: u32 = 0x0107_605c;
const INVALID_FACE_NAME: u32 = 0x0107_604c;
const CAMERA_NODE_NAME: u32 = 0x0107_6044;
const SEQUENCE_NAME_0107603C: u32 = 0x0107_603c;
const SEQUENCE_NAME_01076034: u32 = 0x0107_6034;
const SEQUENCE_NAME_0107602C: u32 = 0x0107_602c;
const WHEEL_NODE_NAME: u32 = 0x0107_6348;
const CONTROLLER_RTTI_011F36AC: u32 = 0x011f_36ac;
const BET_TILE_NAME_FORMAT: u32 = 0x0107_61d8;
const CHIPS_FORMAT_01070E80: u32 = 0x0107_0e80;
const NAME_FORMAT_01012058: u32 = 0x0101_2058;
const PADDED_FORMAT_01029CA8: u32 = 0x0102_9ca8;
/// The game settings the bet descriptions and the chip text use.
const SETTING_011D3F4C: u32 = 0x011d_3f4c;
const SETTING_011D4DBC: u32 = 0x011d_4dbc;
const SETTING_011D3E08: u32 = 0x011d_3e08;
/// `float`s and `double`s (`.rdata`): the chips' raised height (`0101e2bc`), the resting
/// height (`0102caf8`), the cursor fades (`01023cd8`, `0104ef80`), the light radius scale
/// (`0101db80`), the chip step (`01070e78`), the drop test (`01076340`), the wheel step and
/// scale (`01076360`, `01023128`), the special radius scales (`0106b168`, `010760f0`), 0.0
/// (`01012060`), the zero point (`011f426c`) and the identity matrix (`011a9448`).
const RAISED_HEIGHT_0101E2BC: u32 = 0x0101_e2bc;
const RESTING_HEIGHT_0102CAF8: u32 = 0x0102_caf8;
const CURSOR_FADE_01023CD8: u32 = 0x0102_3cd8;
const CURSOR_FADE_0104EF80: u32 = 0x0104_ef80;
const LIGHT_SCALE_0101DB80: u32 = 0x0101_db80;
const CHIP_STEP_01070E78: u32 = 0x0107_0e78;
const DROP_HEIGHT_01076340: u32 = 0x0107_6340;
const WHEEL_STEP_01076360: u32 = 0x0107_6360;
const WHEEL_SCALE_01023128: u32 = 0x0102_3128;
const POSITIVE_SCALE_0106B168: u32 = 0x0106_b168;
const NEGATIVE_SCALE_010760F0: u32 = 0x0107_60f0;
const ZERO_DOUBLE: u32 = 0x0101_2060;
const ZERO_POINT_011F426C: u32 = 0x011f_426c;
const IDENTITY_011A9448: u32 = 0x011a_9448;

/// The open roulette menu (`pMe`).
fn roulette_menu(e: &Engine) -> u32 {
    e.global(ROULETTE_MENU_011DA878)
}

/// `pTiles[index]`.
fn tile(e: &Engine, this: u32, index: u32) -> u32 {
    e.mem
        .u32(this.wrapping_add(TILES).wrapping_add(index.wrapping_mul(4)))
}

/// The item word of a list node, reached through `006815c0` as the code does.
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(NODE_SELF, &args![node]).u32();
    e.mem.u32(slot)
}

/// The address of bet tile record `index`.
fn bet_tile(this: u32, index: u32) -> u32 {
    this.wrapping_add(BET_TILES)
        .wrapping_add(index.wrapping_mul(BET_TILE_SIZE))
}

/// The model node (`spModel`).
fn model(e: &mut Engine) -> u32 {
    e.call(WORD_AT_0_00559450, &args![MODEL_011DA8A0]).u32()
}

/// A path from one of the casino's models (`00504e00(index)`, its vtable +0x14).
fn casino_model_path(e: &mut Engine, index: u32) -> u32 {
    let casino = e.global::<u32>(CASINO_011DA880);
    let model = e.call(CASINO_MODEL_00504E00, &args![casino, index]).u32();
    e.vcall(model, 0x14, &args![]).u32()
}

/// `ModelLoader::LoadFile(path, 0, 1, 0, 0, 0)` on the loader at `011c3b3c`.
fn load_model(e: &mut Engine, path: u32) -> u32 {
    let loader = e.global::<u32>(MODEL_LOADER_011C3B3C);
    e.call(
        MODEL_LOADER_LOAD_FILE,
        &args![loader, path, 0u32, 1u32, 0u32, 0u32, 0u32],
    )
    .u32()
}

/// `0045a5e0(path)` on the loader at `011c3b3c`.
fn release_model(e: &mut Engine, path: u32) {
    let loader = e.global::<u32>(MODEL_LOADER_011C3B3C);
    e.call(MODEL_LOADER_0045A5E0, &args![loader, path]);
}

/// The shadow model's path: the bet model's path copied into `buffer` (0x104 bytes) and
/// the part from the first `'.'` (signed bytes compared with 0x2e, the length re-read with
/// `strlen` each step) replaced with `"_Shadow.NIF"` (`strcpy_s(.., 0xf, ..)`); false when
/// there is no `'.'`.
fn shadow_path(e: &mut Engine, buffer: u32) -> bool {
    let path = casino_model_path(e, 6);
    e.call(STRCPY_S, &args![buffer, 0x104u32, path]);
    let mut dot = 0u32;
    let mut index = 0u32;
    while index < e.call(STRLEN, &args![buffer]).u32() {
        if e.mem.u8(buffer.wrapping_add(index)) as i8 as i32 == 0x2e {
            dot = buffer.wrapping_add(index);
            break;
        }
        index += 1;
    }
    if dot == 0 {
        return false;
    }
    e.call(STRCPY_S, &args![dot, 0xfu32, SHADOW_SUFFIX]);
    true
}

/// `004bc1f0(x, y, z)` on a node (its local translation).
fn set_translate(e: &mut Engine, node: u32, x: u32, y: u32, z: u32) {
    e.call(SET_TRANSLATE_004BC1F0, &args![node, x, y, z]);
}

/// The model node moved to the origin: a point from (0.0, 0, 0) (`0043d410`) given to
/// `00a59c60`.
fn reset_model_translate(e: &mut Engine) {
    e.with_stack(0xc, |e, point| {
        let made = e
            .call(
                POINT_CONSTRUCT_0043D410,
                &args![point, 0.0f32.to_bits(), 0u32, 0u32],
            )
            .u32();
        let node = model(e);
        e.call(SET_TRANSLATE_00A59C60, &args![node, made]);
    });
}

/// `Tile::SetFloat(0xfa9, value, 1)` on the interface manager's cursor tile (+0x28).
fn set_cursor_fade(e: &mut Engine, value: u32) {
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    let cursor = e.call(WORD_AT_28_0045CD60, &args![interface]).u32();
    e.call(TILE_SET_FLOAT, &args![cursor, 0xfa9u32, value, 1u32]);
}

// Translated from 007b8f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RouletteMenu::GetClass` (Xbox PDB; `thiscall`): 0x43a.
pub fn roulette_menu_get_class(_e: &mut Engine, _this: Ptr<RouletteMenu>) -> u32 {
    0x43a
}

// Translated from 007b8f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RouletteMenu::AttachTileByID` (Xbox PDB; `RET 8`: an id and a tile): ids 0 to 10 store
/// the tile in `pTiles`.
pub fn roulette_menu_attach_tile_by_id(
    e: &mut Engine,
    this: Ptr<RouletteMenu>,
    id: i32,
    tile: u32,
) {
    if (0..0xb).contains(&id) {
        e.mem.set_u32(this.addr() + TILES + 4 * id as u32, tile);
    }
}

// Translated from 007b8f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RouletteMenu::\`scalar deleting destructor'` (Xbox PDB; `RET 4`, the flags): the
/// destructor (`007b9020`), then the memory freed with flag 1. Returns `this`.
pub fn roulette_menu_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<RouletteMenu>,
    flags: u32,
) -> Ptr<RouletteMenu> {
    fn_007b9020(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 007b8fa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007b8fa0 - the C++ exception frame (FS:[0] chain, unwind state 0 after the list) is not translated.
/// Unnamed (`thiscall`: a bet tile record's constructor, the vector constructor's
/// argument): the number list (`0096a2d0`) and `006815c0` on +0x10. Returns `this`.
pub fn fn_007b8fa0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(LIST_CONSTRUCT, &args![this]);
    e.call(NODE_SELF, &args![this.addr() + 0x10]);
    this
}

// Translated from 007b9020 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007b9020 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2) and the /GS cookie check (00ec408c) are not translated.
/// The `RouletteMenu` destructor (`thiscall`): the vtable `01075f7c`; `00969b20` on the
/// player; the casino's table, grid, cursor, bet and bet shadow models released
/// (`0045a5e0`); the camera dropped; the model node detached from the scene (vtable +0xe8
/// of `00450b80(1)`), `00b5d180`, `00572160` and dropped; the geometry pointers cleared and
/// the wheel texture dropped; the bet list emptied (`00470470` unless empty) and deleted
/// (`004702f0(1)`), then tested again and emptied (on the deleted list, as the code does);
/// the animations and bet geometry cleared; the statics `pMe`, `pCasinoData` and
/// `pCasino` cleared; the wheel texture pointer destroyed, the bet tile records destroyed
/// (`007b9000`) and the menu base destroyed (`00a1c520`).
pub fn fn_007b9020(e: &mut Engine, this: Ptr<RouletteMenu>) {
    let base = this.addr();
    e.mem.set_u32(base, ROULETTE_MENU_VTABLE);
    let player = e.global::<u32>(PLAYER_011DEA3C);
    e.call(PLAYER_00969B20, &args![player]);
    let path = casino_model_path(e, 9);
    release_model(e, path);
    release_model(e, CURSOR_MODEL_PATH);
    release_model(e, GRID_MODEL_PATH);
    let path = casino_model_path(e, 6);
    release_model(e, path);
    e.with_stack(0x104, |e, buffer| {
        if shadow_path(e, buffer.addr()) {
            release_model(e, buffer.addr());
        }
    });
    e.call(NI_POINTER_ASSIGN, &args![CAMERA_011DA8D0, 0u32]);
    let scene = e.call(SCENE_00450B80, &args![1u32]).u32();
    let node = model(e);
    e.vcall(scene, 0xe8, &args![node]);
    let scene = e.call(SCENE_00450B80, &args![1u32]).u32();
    e.call(SCENE_00B5D180, &args![scene]);
    let node = model(e);
    e.call(NODE_00572160, &args![node]);
    e.call(NI_POINTER_ASSIGN, &args![MODEL_011DA8A0, 0u32]);
    e.set(this, RouletteMenu::pRouletteTableGeometry, 0);
    e.set(this, RouletteMenu::pRouletteGridGeometry, 0);
    e.set(this, RouletteMenu::pCursorGeometry, 0);
    e.call(NI_POINTER_ASSIGN, &args![CAMERA_011DA8D0, 0u32]);
    e.set(this, RouletteMenu::pCurrentBetFace, 0);
    e.set(this, RouletteMenu::pValidBetFace, 0);
    e.set(this, RouletteMenu::pInvalidBetFace, 0);
    e.call(NI_POINTER_ASSIGN, &args![base + WHEEL_TEXTURE, 0u32]);
    let list = e.get(this, RouletteMenu::pBetList);
    if list != 0 && !e.call(LIST_IS_EMPTY, &args![list]).bool() {
        e.call(LIST_FREE_NODES, &args![list]);
    }
    let list = e.get(this, RouletteMenu::pBetList);
    if list != 0 {
        e.call(LIST_DELETE, &args![list, 1u32]);
    }
    e.set(this, RouletteMenu::pTableAnimManager, 0);
    for index in 0..3u32 {
        e.mem.set_u32(base + TABLE_ANIMATIONS + 4 * index, 0);
    }
    for index in 0..10u32 {
        e.mem.set_u32(base + BET_GEOMETRY + 4 * index, 0);
        e.mem.set_u32(base + BET_SHADOW_GEOMETRY + 4 * index, 0);
    }
    let list = e.get(this, RouletteMenu::pBetList);
    if !e.call(LIST_IS_EMPTY, &args![list]).bool() {
        e.call(LIST_FREE_NODES, &args![list]);
    }
    e.set(this, RouletteMenu::pBetList, 0);
    e.set_global(CASINO_DATA_011DA87C, 0u32);
    e.set_global(CASINO_011DA880, 0u32);
    e.set_global(ROULETTE_MENU_011DA878, 0u32);
    e.call(NI_POINTER_DESTROY, &args![base + WHEEL_TEXTURE]);
    e.call(
        VECTOR_DESTRUCT,
        &args![
            base + BET_TILES,
            BET_TILE_SIZE,
            BET_TILE_COUNT,
            0x007b_9000u32
        ],
    );
    e.call(MENU_DESTRUCT, &args![this]);
}

// Translated from 007b9480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RouletteMenu::Close` (Xbox PDB; no arguments): with an open roulette menu, the cursor
/// tile's `0xfa9` gets the `float` at `01023cd8` and the menu's state becomes 5.
pub fn roulette_menu_close(e: &mut Engine) {
    if roulette_menu(e) == 0 {
        return;
    }
    let fade = x87_quiet_bits(e.global::<u32>(CURSOR_FADE_01023CD8));
    set_cursor_fade(e, fade);
    let menu = Ptr::<RouletteMenu>::new(roulette_menu(e));
    e.set(menu, RouletteMenu::eRouletteState, 5);
}

// Translated from 007b94c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007b94c0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3) and the /GS cookie check (00ec408c) are not translated.
/// `RouletteMenu::Prepare3DElements` (Xbox PDB; `thiscall`). Loads the table (the casino's
/// model 9), the grid, the cursor and the bet chip (the casino's model 6) and its shadow
/// (the chip's path with `"_Shadow.NIF"`); raises the grid and cursor (`004bc1f0(0, 0,
/// raised)`); finds the valid and invalid bet faces on the cursor (the invalid one
/// culled, `00450f90(1)`); clones the chip and shadow nine times; finds the camera node on
/// the table; makes the model node (an `NiNode` of 0xac bytes) and attaches the table,
/// grid, cursor, chips and shadows (vtable +0xdc with 1); sets the next model index to 3,
/// resets the chips (`ResetBetChips`) and sets the chip (`SetBetChip`); moves the model
/// node to the origin. With a controller manager on the table (`GetController(011f36ac)`)
/// it keeps it and its three sequences by name, deactivates all (0.0) and starts the
/// table's animations. Without a camera one is made (`007c2180`). The model node is
/// attached to the scene node (`00450b80(1)`) and lit (`AddAllLightsFromNode_Recurse` with
/// the bound radius times the `double` at `0101db80`); then `PrepareBetTiles`.
pub fn roulette_menu_prepare_3d_elements(e: &mut Engine, this: Ptr<RouletteMenu>) {
    let base = this.addr();
    let path = casino_model_path(e, 9);
    let table = load_model(e, path);
    e.set(this, RouletteMenu::pRouletteTableGeometry, table);
    let grid = load_model(e, GRID_MODEL_PATH);
    e.set(this, RouletteMenu::pRouletteGridGeometry, grid);
    let cursor = load_model(e, CURSOR_MODEL_PATH);
    e.set(this, RouletteMenu::pCursorGeometry, cursor);
    let path = casino_model_path(e, 6);
    let chip = load_model(e, path);
    e.mem.set_u32(base + BET_GEOMETRY, chip);
    let raised = x87_quiet_bits(e.global::<u32>(RAISED_HEIGHT_0101E2BC));
    let grid = e.get(this, RouletteMenu::pRouletteGridGeometry);
    set_translate(e, grid, 0.0f32.to_bits(), 0.0f32.to_bits(), raised);
    let raised = x87_quiet_bits(e.global::<u32>(RAISED_HEIGHT_0101E2BC));
    let cursor = e.get(this, RouletteMenu::pCursorGeometry);
    set_translate(e, cursor, 0.0f32.to_bits(), 0.0f32.to_bits(), raised);
    let cursor = e.get(this, RouletteMenu::pCursorGeometry);
    let valid = e
        .call(GET_OBJECT_BY_NAME, &args![cursor, VALID_FACE_NAME])
        .u32();
    e.set(this, RouletteMenu::pValidBetFace, valid);
    let cursor = e.get(this, RouletteMenu::pCursorGeometry);
    let invalid = e
        .call(GET_OBJECT_BY_NAME, &args![cursor, INVALID_FACE_NAME])
        .u32();
    e.set(this, RouletteMenu::pInvalidBetFace, invalid);
    e.call(SET_APP_CULLED_00450F90, &args![invalid, 1u32]);
    e.with_stack(0x104, |e, buffer| {
        if shadow_path(e, buffer.addr()) {
            let shadow = load_model(e, buffer.addr());
            e.mem.set_u32(base + BET_SHADOW_GEOMETRY, shadow);
        }
    });
    for index in 1..10u32 {
        let chip = e.mem.u32(base + BET_GEOMETRY);
        let copy = e.call(NI_OBJECT_CLONE, &args![chip]).u32();
        e.mem.set_u32(base + BET_GEOMETRY + 4 * index, copy);
        let shadow = e.mem.u32(base + BET_SHADOW_GEOMETRY);
        let copy = e.call(NI_OBJECT_CLONE, &args![shadow]).u32();
        e.mem.set_u32(base + BET_SHADOW_GEOMETRY + 4 * index, copy);
    }
    let table = e.get(this, RouletteMenu::pRouletteTableGeometry);
    let camera_node = e
        .call(GET_OBJECT_BY_NAME, &args![table, CAMERA_NODE_NAME])
        .u32();
    e.call(NI_POINTER_ASSIGN, &args![CAMERA_011DA8D0, camera_node]);
    let block = e.call(NI_ALLOCATE, &args![0xacu32]).u32();
    let node = if block != 0 {
        e.call(NI_NODE_CONSTRUCT, &args![block, 0u32]).u32()
    } else {
        0
    };
    e.call(NI_POINTER_ASSIGN, &args![MODEL_011DA8A0, node]);
    for field in [
        RouletteMenu::pRouletteTableGeometry,
        RouletteMenu::pRouletteGridGeometry,
        RouletteMenu::pCursorGeometry,
    ] {
        let node = model(e);
        let child = e.get(this, field);
        e.vcall(node, 0xdc, &args![child, 1u32]);
    }
    for array in [BET_GEOMETRY, BET_SHADOW_GEOMETRY] {
        for index in 0..10u32 {
            let node = model(e);
            let child = e.mem.u32(base + array + 4 * index);
            e.vcall(node, 0xdc, &args![child, 1u32]);
        }
    }
    e.set(this, RouletteMenu::iNextModelIndex, 3);
    roulette_menu_reset_bet_chips(e, this);
    roulette_menu_set_bet_chip(e, this);
    reset_model_translate(e);
    let table = e.get(this, RouletteMenu::pRouletteTableGeometry);
    let manager = e
        .call(GET_CONTROLLER, &args![table, CONTROLLER_RTTI_011F36AC])
        .u32();
    if manager != 0 {
        e.set(this, RouletteMenu::pTableAnimManager, manager);
        for (index, name) in [
            SEQUENCE_NAME_0107603C,
            SEQUENCE_NAME_01076034,
            SEQUENCE_NAME_0107602C,
        ]
        .into_iter()
        .enumerate()
        {
            e.with_stack(4, |e, fixed| {
                let made = e.call(FIXED_STRING_CONSTRUCT, &args![fixed, name]).u32();
                let manager = e.get(this, RouletteMenu::pTableAnimManager);
                let sequence = e
                    .call(SEQUENCE_BY_NAME_0047A520, &args![manager, made])
                    .u32();
                e.mem
                    .set_u32(base + TABLE_ANIMATIONS + 4 * index as u32, sequence);
                e.call(FIXED_STRING_DESTROY, &args![fixed]);
            });
        }
        let manager = e.get(this, RouletteMenu::pTableAnimManager);
        e.call(DEACTIVATE_ALL, &args![manager, 0.0f32.to_bits()]);
        let table = e.get(this, RouletteMenu::pRouletteTableGeometry);
        e.call(START_ANIMATIONS, &args![table]);
    }
    if e.call(WORD_AT_0_00559450, &args![CAMERA_011DA8D0]).u32() == 0 {
        let camera = e.call(MAKE_CAMERA_007C2180, &args![this]).u32();
        e.call(NI_POINTER_ASSIGN, &args![CAMERA_011DA8D0, camera]);
    }
    let scene = e.call(SCENE_00450B80, &args![1u32]).u32();
    let node = model(e);
    e.vcall(scene, 0xdc, &args![node, 1u32]);
    let node = model(e);
    let bound = e.call(GET_WORLD_BOUND, &args![node]).u32();
    let radius = e.call(BOUND_RADIUS_0084D030, &args![bound]).f64();
    let scale: f64 = e.global(LIGHT_SCALE_0101DB80);
    let radius = x87_store32(x87_mul(radius, scale));
    let node = model(e);
    let scene = e.call(SCENE_00450B80, &args![1u32]).u32();
    e.call(ADD_ALL_LIGHTS_FROM_NODE, &args![scene, node, radius]);
    roulette_menu_prepare_bet_tiles(e, this);
}

/// Appends a number to a bet tile record's list (`005ae3d0` given the address of a local
/// holding it).
fn add_number(e: &mut Engine, record: u32, number: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), number);
        e.call(LIST_APPEND, &args![record, slot]);
    });
}

/// A bet tile record's description: `sprintf_s(+0x24, 0x40, format, words..)`.
fn describe(e: &mut Engine, record: u32, format: u32, words: &[u32]) {
    let mut call = vec![record.wrapping_add(0x24), 0x40, format];
    call.extend_from_slice(words);
    e.call(SPRINTF_S, &call);
}

/// The colour (+8) and payout (+0xc) of a bet tile record.
fn set_odds(e: &mut Engine, record: u32, colour: u32, payout: u32) {
    e.mem.set_u32(record.wrapping_add(8), colour);
    e.mem.set_u32(record.wrapping_add(0xc), payout);
}

/// The two game setting strings every description uses, looked up in the code's order
/// (`011d3f4c` first, then `011d4dbc`): (`011d4dbc`, `011d3f4c`).
fn description_settings(e: &mut Engine) -> (u32, u32) {
    let last = e.call(SETTING_STRING, &args![SETTING_011D3F4C]).u32();
    let first = e.call(SETTING_STRING, &args![SETTING_011D4DBC]).u32();
    (first, last)
}

/// As `description_settings` with a third setting looked up between the two:
/// (`011d4dbc`, the setting, `011d3f4c`).
fn description_settings_with(e: &mut Engine, setting: u32) -> (u32, u32, u32) {
    let last = e.call(SETTING_STRING, &args![SETTING_011D3F4C]).u32();
    let middle = e.call(SETTING_STRING, &args![setting]).u32();
    let first = e.call(SETTING_STRING, &args![SETTING_011D4DBC]).u32();
    (first, middle, last)
}

/// `n % 2` as the code computes it for a signed value (`AND 0x80000001` with the sign
/// fix-up): the C remainder.
fn remainder_2(value: i32) -> i32 {
    value % 2
}

/// The special radius of the outside bets `PrepareBetTiles` sets: each difference is scaled
/// by the `double` at `010760f0` when below 0.0 (ordered; `TEST AH,5; JP`), else by the one
/// at `0106b168`; the result is the first unless the second is not above it (`FCOMPP;
/// TEST AH,0x41; JNZ`).
fn special_radius(e: &mut Engine, first: u32, second: u32) -> u32 {
    let zero: f64 = e.global(ZERO_DOUBLE);
    let negative: f64 = e.global(NEGATIVE_SCALE_010760F0);
    let positive: f64 = e.global(POSITIVE_SCALE_0106B168);
    let scale = |value: u32| {
        if x87_load32(value) < zero {
            x87_store32(x87_mul(x87_load32(value), negative))
        } else {
            x87_store32(x87_mul(x87_load32(value), positive))
        }
    };
    let first = scale(first);
    let second = scale(second);
    if x87_load32(second) > x87_load32(first) {
        x87_quiet_bits(first)
    } else {
        x87_quiet_bits(second)
    }
}

/// `FLD dword [a]; FSUB dword [b]; FSTP dword`.
fn float_difference(e: &Engine, a: u32, b: u32) -> u32 {
    x87_store32(x87_sub(x87_load32(e.mem.u32(a)), x87_m32(e.mem.u32(b))))
}

/// The x (+0x10) and y (+0x14) of bet tile record `index`.
fn bet_x(this: u32, index: u32) -> u32 {
    bet_tile(this, index).wrapping_add(0x10)
}

fn bet_y(this: u32, index: u32) -> u32 {
    bet_tile(this, index).wrapping_add(0x14)
}

/// `iNumberStrip`'s values, as `PrepareBetTiles` writes them into a local before copying.
const NUMBER_STRIP_VALUES: [u32; 38] = [
    0xe, 2, 0, 0x1c, 9, 0x1a, 0x1e, 0xb, 7, 0x14, 0x20, 0x11, 5, 0x16, 0x22, 0xf, 3, 0x18, 0x24,
    0xd, 1, 0x25, 0x1b, 0xa, 0x19, 0x1d, 0xc, 8, 0x13, 0x1f, 0x12, 6, 0x15, 0x21, 0x10, 4, 0x17,
    0x23,
];

// Translated from 007b9c40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007b9c40 - the C++ exception frame (FS:[0] chain, unwind state 0 around the fixed string) and the /GS cookie check (00ec408c) are not translated.
/// `RouletteMenu::PrepareBetTiles` (Xbox PDB; `thiscall`). Every bet tile record's
/// position starts at the zero point (`011f426c`). Then the 0x9f records are named
/// `"%c%c%c:0"` from a row letter (from `'A'`), a tens digit and a units digit (from `"01"`)
/// and filled by row: rows B, D and F hold the straight bets (even units digit: the
/// number, colour by the red/black ranges, payout 35), the splits between rows (17) and,
/// at the row ends, the zero splits and baskets (D: 37, 0 and the number, 11; F: 37 and the
/// number, 17; the others: 0 and the number, 17); row A the streets (11), six lines (5)
/// and the top line (0, 37, 1, 2, 3; 6); the other rows (not S) the splits (17), corners
/// (8) and the zero baskets (11). The digits roll over at `'9'`, and at `"25"` the row
/// advances (to S from record 0x8f on). Each record's node is the grid's child of that
/// name (vtable +0x9c with a fixed string when `004aae30` finds none), culled
/// (`00450f90(1)`), its world bound's centre the position; the name is kept. The radius is
/// the distance between the first two records (`fRadius`), the valid radius 0.49 of it
/// (the `double` at `0106b168`). The outside bets (0x90 to 0x9e) get their descriptions,
/// colours, payouts and special radii from fixed record differences, and the wheel's
/// number strip is copied in; `bRunScriptOnExit` is cleared.
// The colour test keeps the exe's chain of range tests, several with the same colour.
#[allow(clippy::if_same_then_else)]
pub fn roulette_menu_prepare_bet_tiles(e: &mut Engine, this: Ptr<RouletteMenu>) {
    let base = this.addr();
    for index in 0..BET_TILE_COUNT {
        let position = bet_x(base, index);
        for word in 0..3u32 {
            let value = e.global::<u32>(ZERO_POINT_011F426C + 4 * word);
            e.mem.set_u32(position + 4 * word, value);
        }
    }
    let mut row = b'A';
    let mut tens = b'0';
    let mut units = b'1';
    e.with_stack(0xc, |e, frame| {
        let name = frame.addr();
        let pair = frame.addr() + 8;
        for index in 0..BET_TILE_COUNT {
            let record = bet_tile(base, index);
            let i = index as i32;
            e.call(
                SPRINTF_S,
                &[
                    name,
                    8,
                    BET_TILE_NAME_FORMAT,
                    row as i8 as i32 as u32,
                    tens as i8 as i32 as u32,
                    units as i8 as i32 as u32,
                ],
            );
            if row == b'B' || row == b'D' || row == b'F' {
                if remainder_2(units as i8 as i32) == 0 {
                    let number = (i / 0x18 + 1) / 2 + (i % 0x18 / 2) * 3;
                    let (first, last) = description_settings(e);
                    describe(e, record, 0x0107_61c4, &[first, number as u32, last]);
                    add_number(e, record, number as u32);
                    let even = remainder_2(number) == 0;
                    let colour = if even && number < 0xb {
                        1
                    } else if number < 0xb {
                        2
                    } else if even && number < 0x13 {
                        2
                    } else if number < 0x13 {
                        1
                    } else if even && number < 0x1c {
                        1
                    } else if number < 0x1c {
                        2
                    } else if !even {
                        2
                    } else {
                        1
                    };
                    e.mem.set_u32(record + 8, colour);
                    e.mem.set_u32(record + 0xc, 0x23);
                } else if i % 0x18 != 0 {
                    let first_number = (i / 0x18 + 1) / 2 + ((i - 1) % 0x18 / 2) * 3;
                    let second_number = first_number + 3;
                    let (first, last) = description_settings(e);
                    describe(
                        e,
                        record,
                        0x0107_61b0,
                        &[first, first_number as u32, second_number as u32, last],
                    );
                    add_number(e, record, first_number as u32);
                    add_number(e, record, second_number as u32);
                    set_odds(e, record, 0, 0x11);
                } else {
                    let number = (i / 0x18 + 1) / 2 + ((i + 1) % 0x18 / 2) * 3;
                    let (first, last) = description_settings(e);
                    if row == b'D' {
                        describe(
                            e,
                            record,
                            0x0107_6194,
                            &[0x0107_61a8, first, number as u32, last],
                        );
                        add_number(e, record, 0x25);
                        add_number(e, record, 0);
                        add_number(e, record, number as u32);
                        set_odds(e, record, 0, 0xb);
                    } else if row == b'F' {
                        describe(
                            e,
                            record,
                            0x0107_617c,
                            &[0x0107_6190, first, number as u32, last],
                        );
                        add_number(e, record, 0x25);
                        add_number(e, record, number as u32);
                        set_odds(e, record, 0, 0x11);
                    } else {
                        describe(
                            e,
                            record,
                            0x0107_617c,
                            &[0x0102_0760, first, number as u32, last],
                        );
                        add_number(e, record, 0);
                        add_number(e, record, number as u32);
                        set_odds(e, record, 0, 0x11);
                    }
                }
            } else if row == b'A' {
                if remainder_2(i) == 1 {
                    let number = (i % 0x18 / 2) * 3 + 1;
                    let (first, last) = description_settings(e);
                    let numbers = [number, number + 1, number + 2].map(|n| n as u32);
                    describe(
                        e,
                        record,
                        0x0107_6164,
                        &[first, numbers[0], numbers[1], numbers[2], last],
                    );
                    for number in numbers {
                        add_number(e, record, number);
                    }
                    set_odds(e, record, 0, 0xb);
                } else if i != 0 {
                    let number = ((i - 1) % 0x18 / 2) * 3 + 1;
                    let (first, last) = description_settings(e);
                    let numbers: Vec<u32> = (0..6).map(|step| (number + step) as u32).collect();
                    let mut words = vec![first];
                    words.extend_from_slice(&numbers);
                    words.push(last);
                    describe(e, record, 0x0107_6144, &words);
                    for number in numbers {
                        add_number(e, record, number);
                    }
                    set_odds(e, record, 0, 5);
                } else {
                    let (first, last) = description_settings(e);
                    describe(e, record, 0x0107_612c, &[first, last]);
                    for number in [0u32, 0x25, 1, 2, 3] {
                        add_number(e, record, number);
                    }
                    set_odds(e, record, 0, 6);
                }
            } else if row != b'S' {
                if remainder_2(units as i8 as i32) == 0 {
                    let number = (i / 0x18 + 1) / 2 + (i % 0x18 / 2) * 3;
                    let next = number + 1;
                    let (first, last) = description_settings(e);
                    describe(
                        e,
                        record,
                        0x0107_61b0,
                        &[first, number as u32, next as u32, last],
                    );
                    add_number(e, record, number as u32);
                    add_number(e, record, next as u32);
                    set_odds(e, record, 0, 0x11);
                } else if i % 0x18 != 0 {
                    let number = (i / 0x18 + 1) / 2 + ((i - 1) % 0x18 / 2) * 3;
                    let second = number + 1;
                    let third = number + 3;
                    let fourth = third + 1;
                    let (first, last) = description_settings(e);
                    describe(
                        e,
                        record,
                        0x0107_6110,
                        &[
                            first,
                            number as u32,
                            second as u32,
                            third as u32,
                            fourth as u32,
                            last,
                        ],
                    );
                    for number in [number, second, third, fourth] {
                        add_number(e, record, number as u32);
                    }
                    set_odds(e, record, 0, 8);
                } else {
                    let other = u32::from(row != b'C');
                    let second = other + 1;
                    let third = other + 2;
                    let zero = if row != b'C' { 0x25 } else { 0 };
                    let source = if row == b'C' {
                        0x0102_0760
                    } else {
                        0x0107_6190
                    };
                    e.call(STRCPY_S, &args![pair, 3u32, source]);
                    let (first, last) = description_settings(e);
                    describe(e, record, 0x0107_60f8, &[first, pair, second, third, last]);
                    add_number(e, record, second);
                    add_number(e, record, third);
                    add_number(e, record, zero);
                    set_odds(e, record, 0, 0xb);
                }
            }
            units = units.wrapping_add(1);
            if units as i8 > b'9' as i8 {
                units = b'0';
                tens = tens.wrapping_add(1);
            }
            if units == b'5' && tens == b'2' {
                row = row.wrapping_add(1);
                if i >= 0x8f {
                    row = b'S';
                }
                tens = b'0';
                units = b'1';
            }
            let grid = e.get(this, RouletteMenu::pRouletteGridGeometry);
            let mut node = e.call(GET_OBJECT_BY_NAME, &args![grid, name]).u32();
            if node == 0 {
                node = e.with_stack(4, |e, fixed| {
                    let made = e.call(FIXED_STRING_CONSTRUCT, &args![fixed, name]).u32();
                    let grid = e.get(this, RouletteMenu::pRouletteGridGeometry);
                    let found = e.vcall(grid, 0x9c, &args![made]).u32();
                    e.call(FIXED_STRING_DESTROY, &args![fixed]);
                    found
                });
            }
            e.call(SET_APP_CULLED_00450F90, &args![node, 1u32]);
            let bound = e.call(GET_WORLD_BOUND, &args![node]).u32();
            let centre = e.call(NODE_SELF, &args![bound]).u32();
            let position = bet_x(base, index);
            for word in 0..3u32 {
                let value = e.mem.u32(centre + 4 * word);
                e.mem.set_u32(position + 4 * word, value);
            }
            e.call(STRCPY_S, &args![record + 0x1c, 8u32, name]);
        }
    });
    let radius = e.with_stack(0xc, |e, difference| {
        e.call(
            POINT_SUBTRACT_00439EF0,
            &args![bet_x(base, 0), difference, bet_x(base, 1)],
        );
        e.call(POINT_LENGTH_00457990, &args![difference]).f64()
    });
    e.mem.set_u32(base + 0x3f90, x87_store32(radius));
    let scale: f64 = e.global(POSITIVE_SCALE_0106B168);
    let valid = x87_store32(x87_mul(x87_load32(e.mem.u32(base + 0x3f90)), scale));
    e.mem.set_u32(base + 0x3f94, valid);
    for index in 0x90..BET_TILE_COUNT {
        if index == 0x90 {
            let first = float_difference(e, bet_x(base, index), bet_x(base, 0));
            let second = float_difference(e, bet_y(base, index), bet_y(base, index + 1));
            let radius = special_radius(e, first, second);
            for offset in [0x3fa0u32, 0x3f9c, 0x3f98] {
                e.mem.set_u32(base + offset, radius);
            }
            for (step, format) in [(0u32, 0x0107_60d8u32), (1, 0x0107_60c4), (2, 0x0107_60b4)] {
                let (first, last) = description_settings(e);
                describe(e, bet_tile(base, index + step), format, &[first, last]);
            }
            add_number(e, bet_tile(base, index), 0x25);
            add_number(e, bet_tile(base, index + 1), 0x25);
            add_number(e, bet_tile(base, index + 1), 0);
            add_number(e, bet_tile(base, index + 2), 0);
            for step in 0..3u32 {
                e.mem.set_u32(bet_tile(base, index + step) + 8, 0);
            }
            for (step, payout) in [(0u32, 0x23u32), (1, 0x11), (2, 0x23)] {
                e.mem.set_u32(bet_tile(base, index + step) + 0xc, payout);
            }
        }
        if index == 0x93 {
            let first = float_difference(e, bet_x(base, index), bet_y(base, index + 3));
            let second = float_difference(e, bet_y(base, index), bet_y(base, 0));
            let radius = special_radius(e, first, second);
            for offset in [0x3fbcu32, 0x3fb0, 0x3fa4] {
                e.mem.set_u32(base + offset, radius);
            }
            for (step, setting) in [(0u32, 0x011d_4b88u32), (3, 0x011d_264c), (6, 0x011d_2dcc)] {
                let (first, middle, last) = description_settings_with(e, setting);
                describe(
                    e,
                    bet_tile(base, index + step),
                    0x0107_60a4,
                    &[first, middle, last],
                );
            }
            for step in [0u32, 3, 6] {
                e.mem.set_u32(bet_tile(base, index + step) + 8, 0);
            }
            for step in [0u32, 3, 6] {
                e.mem.set_u32(bet_tile(base, index + step) + 0xc, 2);
            }
        }
        if index == 0x94 {
            let first = float_difference(e, bet_x(base, index), bet_y(base, index + 1));
            let second = float_difference(e, bet_y(base, index), bet_y(base, 3));
            let radius = special_radius(e, first, second);
            for offset in [0x3fc4u32, 0x3fc0, 0x3fb8, 0x3fb4, 0x3fac, 0x3fa8] {
                e.mem.set_u32(base + offset, radius);
            }
            let (first, last) = description_settings(e);
            describe(e, bet_tile(base, index), 0x0107_6090, &[first, last]);
            for (step, setting) in [
                (1u32, 0x011d_2694u32),
                (3, 0x011d_1df0),
                (4, 0x011d_40c0),
                (6, 0x011d_2d30),
            ] {
                let (first, middle, last) = description_settings_with(e, setting);
                describe(
                    e,
                    bet_tile(base, index + step),
                    0x0107_6080,
                    &[first, middle, last],
                );
            }
            let (first, last) = description_settings(e);
            describe(e, bet_tile(base, index + 7), 0x0107_606c, &[first, last]);
            for step in [0u32, 1, 3, 4, 6, 7] {
                e.mem.set_u32(bet_tile(base, index + step) + 8, 0);
            }
            for step in [0u32, 1, 3, 4, 6, 7] {
                e.mem.set_u32(bet_tile(base, index + step) + 0xc, 1);
            }
        }
        if index == 0x9c {
            let first = float_difference(e, bet_x(base, index), bet_x(base, 0x17));
            let second = float_difference(e, bet_y(base, index), bet_y(base, index + 1));
            let radius = special_radius(e, first, second);
            for offset in [0x3fd0u32, 0x3fcc, 0x3fc8] {
                e.mem.set_u32(base + offset, radius);
            }
            for step in 0..3u32 {
                let (first, middle, last) = description_settings_with(e, 0x011d_32f4);
                describe(
                    e,
                    bet_tile(base, index + step),
                    0x0107_60a4,
                    &[first, middle, last],
                );
            }
            for step in 0..3u32 {
                e.mem.set_u32(bet_tile(base, index + step) + 8, 0);
            }
            for step in 0..3u32 {
                e.mem.set_u32(bet_tile(base, index + step) + 0xc, 2);
            }
        }
    }
    e.with_stack(0x98, |e, strip| {
        for (index, value) in NUMBER_STRIP_VALUES.into_iter().enumerate() {
            e.mem.set_u32(strip.addr() + 4 * index as u32, value);
        }
        e.call(COPY_00401460, &args![base + NUMBER_STRIP, strip, 0x98u32]);
    });
    e.set(this, RouletteMenu::bRunScriptOnExit, false);
}

// Translated from 007bc820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): 0x43a, the roulette menu's class.
pub fn fn_007bc820(_e: &mut Engine) -> u32 {
    0x43a
}

// Translated from 007bc830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the roulette menu): whether all eleven `pTiles` are set.
pub fn fn_007bc830(e: &mut Engine, this: Ptr<RouletteMenu>) -> bool {
    (0..0xbu32).all(|index| tile(e, this.addr(), index) != 0)
}

// Translated from 007bed00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RouletteMenu::DoEnter` (engine map, vtable; `RET 8`, two unread words): the cursor
/// tile's `0xfa9` becomes 0.0.
pub fn roulette_menu_do_enter(
    e: &mut Engine,
    _this: Ptr<RouletteMenu>,
    _unused_1: u32,
    _unused_2: u32,
) {
    set_cursor_fade(e, 0.0f32.to_bits());
}

// Translated from 007bed30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RouletteMenu::DoLeave` (engine map, vtable; `RET 8`, two unread words): the cursor
/// tile's `0xfa9` becomes the `float` at `0104ef80`.
pub fn roulette_menu_do_leave(
    e: &mut Engine,
    _this: Ptr<RouletteMenu>,
    _unused_1: u32,
    _unused_2: u32,
) {
    let fade = x87_quiet_bits(e.global::<u32>(CURSOR_FADE_0104EF80));
    set_cursor_fade(e, fade);
}

// Translated from 007bf390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RouletteMenu::GetBetIndex` (Xbox PDB; `thiscall`): the first of the 0x90 inside bets
/// whose position is within the valid radius of the cursor (the cursor's +0x8c point minus
/// the record's, `00457990`; on when the radius is not below the distance), else the first
/// outside bet (0x90 to 0x9e) within its special radius; 0x9f when none.
pub fn roulette_menu_get_bet_index(e: &mut Engine, this: Ptr<RouletteMenu>) -> u32 {
    let base = this.addr();
    let distance = |e: &mut Engine, index: u32| {
        e.with_stack(0xc, |e, difference| {
            let cursor = e.get(this, RouletteMenu::pCursorGeometry);
            let point = e.call(WORLD_TRANSLATE_0045BB80, &args![cursor]).u32();
            e.call(
                POINT_SUBTRACT_00439EF0,
                &args![point, difference, bet_x(base, index)],
            );
            let length = e.call(POINT_LENGTH_00457990, &args![difference]).f64();
            x87_load32(x87_store32(length))
        })
    };
    let mut index = 0u32;
    while index < 0x90 {
        let length = distance(e, index);
        // `FLD radius; FCOMPP; TEST AH,1; JNZ`: goes on while below (or unordered).
        let radius = x87_load32(e.mem.u32(base + 0x3f94));
        if !(radius < length || radius.is_nan() || length.is_nan()) {
            break;
        }
        index += 1;
    }
    if index == 0x90 {
        while index < BET_TILE_COUNT {
            let length = distance(e, index);
            // `+0x3d58 + index * 4`: `fSpecialRadius[index - 0x90]`.
            let radius = x87_load32(e.mem.u32(base + SPECIAL_RADIUS + 4 * (index - 0x90)));
            if !(radius < length || radius.is_nan() || length.is_nan()) {
                break;
            }
            index += 1;
        }
    }
    index
}

// Translated from 007bf480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the roulette menu, `RET 4`, a model index): the geometry at
/// +0x3f14 and +0x3f3c of that index go back to (0.0, 0.0, the resting height at
/// `0102caf8`), then both get the translation of a point made from (0.0, 0, 0).
pub fn fn_007bf480(e: &mut Engine, this: Ptr<RouletteMenu>, index: u32) {
    let base = this.addr();
    for offset in [0x3f14u32, 0x3f3c] {
        let height = x87_quiet_bits(e.global::<u32>(RESTING_HEIGHT_0102CAF8));
        let node = e.mem.u32(
            base.wrapping_add(offset)
                .wrapping_add(index.wrapping_mul(4)),
        );
        set_translate(e, node, 0.0f32.to_bits(), 0.0f32.to_bits(), height);
    }
    for offset in [0x3f14u32, 0x3f3c] {
        e.with_stack(0xc, |e, point| {
            let made = e
                .call(
                    POINT_CONSTRUCT_0043D410,
                    &args![point, 0.0f32.to_bits(), 0u32, 0u32],
                )
                .u32();
            let node = e.mem.u32(
                base.wrapping_add(offset)
                    .wrapping_add(index.wrapping_mul(4)),
            );
            e.call(SET_TRANSLATE_00A59C60, &args![node, made]);
        });
    }
}

// Translated from 007bf530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the roulette menu, `RET 4`, a model index): whether that model's
/// (+0x3f14) local height (+8 of `0043c490`'s translation) is below the `double` at
/// `01076340` (`TEST AH,5; JP`: ordered).
pub fn fn_007bf530(e: &mut Engine, this: Ptr<RouletteMenu>, index: u32) -> bool {
    let node = e.mem.u32(
        this.addr()
            .wrapping_add(0x3f14)
            .wrapping_add(index.wrapping_mul(4)),
    );
    let translate = e.call(LOCAL_TRANSLATE_0043C490, &args![node]).u32();
    let height = x87_load32(e.mem.u32(translate.wrapping_add(8)));
    let limit: f64 = e.global(DROP_HEIGHT_01076340);
    height < limit
}

// Translated from 007bf570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RouletteMenu::ShiftWheelTexture` (Xbox PDB; `RET 4`, a number): the number's place on
/// the strip counted from its end (38 minus the first index holding it, or 0 when none),
/// plus 5, 3 or 10 for spin animations 0, 1 and 2, times the `double` at `01076360` and
/// the one at `01023128` (each product kept as a `double`, then a `float`) is the angle
/// of a rotation (`004a0c90` on a copy of the identity at `011a9448`) set on the wheel node
/// (`01076348`) when the table has it.
pub fn roulette_menu_shift_wheel_texture(e: &mut Engine, this: Ptr<RouletteMenu>, number: u32) {
    let base = this.addr();
    let step = x87_load64(e.global::<u64>(WHEEL_STEP_01076360));
    let mut index = 0u32;
    while (index as i32) < 0x26 {
        if e.mem.u32(base + NUMBER_STRIP + 4 * index) == number {
            break;
        }
        index += 1;
    }
    let mut count = 0x26i32.wrapping_sub(index as i32);
    match e.get(this, RouletteMenu::iSpinAnimIndex) {
        0 => count = count.wrapping_add(5),
        1 => count = count.wrapping_add(3),
        2 => count = count.wrapping_add(10),
        _ => {}
    }
    let product = x87_mul(f64::from(count), step);
    let scale: f64 = e.global(WHEEL_SCALE_01023128);
    let angle = x87_mul(x87_load64(product.to_bits()), scale);
    let angle = x87_store32(x87_load64(angle.to_bits()));
    e.with_stack(0x24, |e, matrix| {
        for word in 0..9u32 {
            let value = e.global::<u32>(IDENTITY_011A9448 + 4 * word);
            e.mem.set_u32(matrix.addr() + 4 * word, value);
        }
        e.call(ROTATION_004A0C90, &args![matrix, x87_quiet_bits(angle)]);
        let table = e.get(this, RouletteMenu::pRouletteTableGeometry);
        let wheel = e
            .call(GET_OBJECT_BY_NAME, &args![table, WHEEL_NODE_NAME])
            .u32();
        if wheel != 0 {
            e.call(SET_ROTATE_0043FA80, &args![wheel, matrix]);
        }
    });
}

// Translated from 007bf670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the roulette menu): the spin animation (`00944460(0, 3)`) and the
/// result (`00944460(0, 0x26)`) are drawn; the luck value against 5 sets `bLucky` (above)
/// or `bUnlucky` (below), both cleared at 5.
pub fn fn_007bf670(e: &mut Engine, this: Ptr<RouletteMenu>) {
    let spin = e.call(RANDOM_00944460, &args![0u32, 3u32]).i32();
    e.set(this, RouletteMenu::iSpinAnimIndex, spin);
    let result = e.call(RANDOM_00944460, &args![0u32, 0x26u32]).i32();
    e.set(this, RouletteMenu::iResultValue, result);
    let luck = e.get(this, RouletteMenu::iLuckValue).wrapping_sub(5);
    if luck == 0 {
        e.set(this, RouletteMenu::bUnlucky, false);
        e.set(this, RouletteMenu::bLucky, false);
    } else if luck > 0 {
        e.set(this, RouletteMenu::bUnlucky, false);
        e.set(this, RouletteMenu::bLucky, true);
    } else {
        e.set(this, RouletteMenu::bUnlucky, true);
        e.set(this, RouletteMenu::bLucky, false);
    }
}

// Translated from 007bf710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RouletteMenu::SetTileTextAlpha` (Xbox PDB; `RET 8`: a tile index and a `float`): every
/// child of the open menu's tile (the list at +4: the first node `00559450`, each node's
/// tile `006317a0`, the next `007b52d0`, while the list's first slot `0073ac90` holds one)
/// gets `0xfa9` set to the value.
pub fn roulette_menu_set_tile_text_alpha(
    e: &mut Engine,
    _this: Ptr<RouletteMenu>,
    index: u32,
    alpha: u32,
) {
    let menu = roulette_menu(e);
    let parent = tile(e, menu, index);
    let mut node = e
        .call(WORD_AT_0_00559450, &args![parent.wrapping_add(4)])
        .u32();
    let menu = roulette_menu(e);
    let list = tile(e, menu, index).wrapping_add(4);
    while node != 0 && list != 0 {
        let head = e.call(LIST_HEAD_SLOT_0073AC90, &args![list]).u32();
        if e.mem.u32(head) == 0 {
            break;
        }
        let slot = e.call(NODE_SLOT_006317A0, &args![list, node]).u32();
        let child = e.mem.u32(slot);
        e.call(
            TILE_SET_FLOAT,
            &args![child, 0xfa9u32, x87_quiet_bits(alpha), 1u32],
        );
        node = e.call(NEXT_NODE_007B52D0, &args![list, node]).u32();
    }
}

// Translated from 007bf7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RouletteMenu::ResetBetChips` (Xbox PDB; `thiscall`): the ten chips and their shadows go
/// back to (0.0, 0.0, the resting height at `0102caf8`) and the model node to the origin.
pub fn roulette_menu_reset_bet_chips(e: &mut Engine, this: Ptr<RouletteMenu>) {
    let base = this.addr();
    let mut index = 0i32;
    while index < 10 {
        for array in [BET_GEOMETRY, BET_SHADOW_GEOMETRY] {
            let height = x87_quiet_bits(e.global::<u32>(RESTING_HEIGHT_0102CAF8));
            let node = e.mem.u32(base + array + 4 * index as u32);
            set_translate(e, node, 0.0f32.to_bits(), 0.0f32.to_bits(), height);
        }
        index += 1;
    }
    reset_model_translate(e);
}

// Translated from 007bf840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RouletteMenu::SetBetChip` (Xbox PDB; `thiscall`): the chip stack height for the
/// current bet (1 below 5, then 10, 25, 40, 70, 100, 250, 500 and 1000 step it up to 10)
/// times the `double` at `01070e78` raises the next model (`iNextModelIndex` into the
/// geometry from +0x3f14) at its own x and y; its shadow (+0x3f3c) goes to that x and y at
/// 0.0; the model node to the origin.
pub fn roulette_menu_set_bet_chip(e: &mut Engine, this: Ptr<RouletteMenu>) {
    let base = this.addr();
    let bet = e.get(this, RouletteMenu::iCurrentBet);
    let height = if bet < 5 {
        1
    } else if bet < 10 {
        2
    } else if bet < 0x19 {
        3
    } else if bet < 0x28 {
        4
    } else if bet < 0x46 {
        5
    } else if bet < 0x64 {
        6
    } else if bet < 0xfa {
        7
    } else if bet < 0x1f4 {
        8
    } else if bet < 0x3e8 {
        9
    } else {
        10
    };
    let next = e.get(this, RouletteMenu::iNextModelIndex) as u32;
    let node = e
        .mem
        .u32(base.wrapping_add(0x3f14).wrapping_add(next.wrapping_mul(4)));
    let translate = e.call(LOCAL_TRANSLATE_0043C490, &args![node]).u32();
    let x = e.mem.u32(translate);
    let y = e.mem.u32(translate.wrapping_add(4));
    let step: f64 = e.global(CHIP_STEP_01070E78);
    let z = x87_store32(x87_mul(f64::from(height), step));
    let next = e.get(this, RouletteMenu::iNextModelIndex) as u32;
    let node = e
        .mem
        .u32(base.wrapping_add(0x3f14).wrapping_add(next.wrapping_mul(4)));
    set_translate(e, node, x87_quiet_bits(x), x87_quiet_bits(y), z);
    let next = e.get(this, RouletteMenu::iNextModelIndex) as u32;
    let shadow = e
        .mem
        .u32(base.wrapping_add(0x3f3c).wrapping_add(next.wrapping_mul(4)));
    set_translate(
        e,
        shadow,
        x87_quiet_bits(x),
        x87_quiet_bits(y),
        0.0f32.to_bits(),
    );
    reset_model_translate(e);
}

// Translated from 007bf9e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007bf9e0 - the /GS cookie check (00ec408c) is not translated.
/// Unnamed (`thiscall` on the roulette menu): the chip count text. `"%s %s %i"`-like
/// (`01070e80`) of the casino's name (+0x18), the game setting `011d3e08` and the casino
/// data's +4; under 0x17 characters it is rewritten from `01012058` (name and setting),
/// padded with spaces from one before the old length (each step writes a space and a 0,
/// the step counter on the stack just below the buffer as the code has it), and the
/// count appended (`01029ca8`). The text goes to tile +0x34 (`pTiles[2]`) with the trait
/// at `011da870`.
pub fn fn_007bf9e0(e: &mut Engine, this: Ptr<RouletteMenu>) {
    // The frame from `EBP - 0x118`: `this` (+0), the step counter (+4), the buffer (+8).
    e.with_stack(0x110, |e, frame| {
        let counter = frame.addr() + 4;
        let buffer = frame.addr() + 8;
        e.mem.set_u32(frame.addr(), this.addr());
        let data = e.global::<u32>(CASINO_DATA_011DA87C);
        let count = e.mem.u32(data.wrapping_add(4));
        let setting = e.call(SETTING_STRING, &args![SETTING_011D3E08]).u32();
        let casino = e.global::<u32>(CASINO_011DA880);
        let name = e
            .call(NAME_CHARS_00408DA0, &args![casino.wrapping_add(0x18)])
            .u32();
        e.call(
            SPRINTF_S,
            &[buffer, 0x104, CHIPS_FORMAT_01070E80, name, setting, count],
        );
        let length = e.call(STRLEN, &args![buffer]).i32();
        if length < 0x17 {
            let setting = e.call(SETTING_STRING, &args![SETTING_011D3E08]).u32();
            let casino = e.global::<u32>(CASINO_011DA880);
            let name = e
                .call(NAME_CHARS_00408DA0, &args![casino.wrapping_add(0x18)])
                .u32();
            e.call(
                SPRINTF_S,
                &[buffer, 0x104, NAME_FORMAT_01012058, name, setting],
            );
            e.mem.set_u32(counter, 0);
            while (e.mem.u32(counter) as i32) < 0x17i32.wrapping_sub(length) {
                let step = e.mem.u32(counter);
                let at = buffer
                    .wrapping_add(length as u32)
                    .wrapping_add(step)
                    .wrapping_sub(1);
                e.mem.set_u8(at, 0x20);
                let step = e.mem.u32(counter);
                let at = buffer.wrapping_add(length as u32).wrapping_add(step);
                e.mem.set_u8(at, 0);
                let step = e.mem.u32(counter).wrapping_add(1);
                e.mem.set_u32(counter, step);
            }
            let data = e.global::<u32>(CASINO_DATA_011DA87C);
            let count = e.mem.u32(data.wrapping_add(4));
            e.call(
                SPRINTF_S,
                &[buffer, 0x104, PADDED_FORMAT_01029CA8, buffer, count],
            );
        }
        let trait_id = e.global::<u32>(TRAIT_011DA870);
        let menu_tile = e.mem.u32(this.addr() + 0x34);
        e.call(TILE_SET_STRING, &args![menu_tile, trait_id, buffer, 1u32]);
    });
}
/// A menu sound by name: `BSAudio::QInstance` (`00453a70`), `BSAudio::GetSoundHandleByName`
/// (`00ad7550`, `RET 0xc`, the handle returned through its first word),
/// `BSSoundHandle::Play` (`00ad8830`, `RET 4`) and the handle's destructor (`00483710`).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_DESTROY: u32 = 0x0048_3710;
/// `0084e3a0` (`thiscall`: a form's key, as the casino data compares it), `00726070` (the
/// word at +4: a node's next, a menu's tile) and `0063f7b0` (removes a list's first node).
const KEY_0084E3A0: u32 = 0x0084_e3a0;
const WORD_AT_4_00726070: u32 = 0x0072_6070;
const LIST_REMOVE_FIRST_0063F7B0: u32 = 0x0063_f7b0;

/// The second trait id the bet and chip tiles use (`011da86c`), the "wrong bet" sound
/// toggle (`011da900`), the spin's elapsed time (`011da904`, a `float`), the state seen on
/// the last idle (`011a081c`) and `DoIdle`'s function-local static (`011da908`, its guard
/// `011da914`, constructed by `0041a250` and destroyed at exit by `00fd80d0`).
const TRAIT_011DA86C: u32 = 0x011d_a86c;
const INVALID_SOUND_PLAYED_011DA900: u32 = 0x011d_a900;
const SPIN_TIME_011DA904: u32 = 0x011d_a904;
const LAST_STATE_011A081C: u32 = 0x011a_081c;
const IDLE_STATIC_011DA908: u32 = 0x011d_a908;
const IDLE_GUARD_011DA914: u32 = 0x011d_a914;
const IDLE_STATIC_CONSTRUCT_0041A250: u32 = 0x0041_a250;
const IDLE_STATIC_DESTRUCT: u32 = 0x00fd_80d0;
const ATEXIT: u32 = 0x00ec_658f;
/// The timer object (`011f6394`) whose +0x14 (`00825c00`) is the time in milliseconds.
const TIMER_011F6394: u32 = 0x011f_6394;
const WORD_AT_14_00825C00: u32 = 0x0082_5c00;
/// `Interface::IsTopMenuID` (`00702450`, `cdecl`), `004b71d0` (no arguments, `AL`), the
/// input object (`011dea0c`, `00877720`), its stick (`00a23390`, `RET 8`) and mouse
/// (`00a239e0`, `RET 4`) values, `00408820` (`cdecl`, a `float`, `ST0`: the magnitude),
/// `InterfaceManager::GetScreenWidth` (`00715d40`, `ST0`) and `00706e50` (`ST0`).
const INTERFACE_IS_TOP_MENU_ID: u32 = 0x0070_2450;
const FLAG_004B71D0: u32 = 0x004b_71d0;
const INPUT_011DEA0C: u32 = 0x011d_ea0c;
const INPUT_00877720: u32 = 0x0087_7720;
const STICK_00A23390: u32 = 0x00a2_3390;
const MOUSE_00A239E0: u32 = 0x00a2_39e0;
const ABS_00408820: u32 = 0x0040_8820;
const SCREEN_WIDTH: u32 = 0x0071_5d40;
const SCREEN_00706E50: u32 = 0x0070_6e50;
/// The idle's `double`s and `float`s: 1000.0 (`01017b70`), the stick dead zone
/// (`010718f0`), the stick and mouse scales (`01029790`, `01020758`), 2.0 (`01011590`), the
/// cursor speed (`01021f44`), the cursor limits (`010762f8`/`010762f0`,
/// `010762e8`/`010762e0`, `010356d0`/`0101f414`, `010762d8`/`010762d0`), the payout step
/// (`010290b0`), the hidden-tile alpha (`0101e704`) and the message `float` (`010162c0`).
const THOUSAND_DOUBLE: u32 = 0x0101_7b70;
const STICK_DEAD_ZONE_010718F0: u32 = 0x0107_18f0;
const STICK_SCALE_01029790: u32 = 0x0102_9790;
const MOUSE_SCALE_01020758: u32 = 0x0102_0758;
const TWO_DOUBLE: u32 = 0x0101_1590;
const CURSOR_SPEED_01021F44: u32 = 0x0102_1f44;
const CURSOR_MIN_X_010762F8: u32 = 0x0107_62f8;
const CURSOR_MIN_X_SET_010762F0: u32 = 0x0107_62f0;
const CURSOR_MIN_Y_010762E8: u32 = 0x0107_62e8;
const CURSOR_MIN_Y_SET_010762E0: u32 = 0x0107_62e0;
const CURSOR_MAX_X_010356D0: u32 = 0x0103_56d0;
const CURSOR_MAX_X_SET_0101F414: u32 = 0x0101_f414;
const CURSOR_MAX_Y_010762D8: u32 = 0x0107_62d8;
const CURSOR_MAX_Y_SET_010762D0: u32 = 0x0107_62d0;
const PAYOUT_STEP_010290B0: u32 = 0x0102_90b0;
const HIDDEN_ALPHA_0101E704: u32 = 0x0101_e704;
const MESSAGE_VALUE_010162C0: u32 = 0x0101_62c0;
/// The menu's fade state (`0059bb30`, `thiscall`), `Menu::StartFadeOut` (`00a1d910`),
/// `Tile::SetInt`-like `00700320` (`RET 8`), `Tile::Poke` (`00a01290`, `RET 8`).
const MENU_FADE_STATE_0059BB30: u32 = 0x0059_bb30;
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
const TILE_SET_INT: u32 = 0x0070_0320;
const TILE_POKE: u32 = 0x00a0_1290;
/// The casino's item (`007316d0`), its data's limit (`00504e60`, the word at +0x28), its
/// end script (`00504e40`), `00737e30`, `Actor::GetItemCountinContainer` (`00575610`, `RET
/// 4`), `Actor::GetPickUpSoundName` (`008adcf0`, `RET 0xc`), `TESFullName::GetFullName`
/// (`00482720`, `cdecl`), the message box (`007052f0`, `cdecl`, six words), the script's
/// run (`0060c9c0`, `RET 4`) and the player's vtable +0x17c (removing an item, ten words).
const CASINO_ITEM_007316D0: u32 = 0x0073_16d0;
const CASINO_LIMITS_00504E60: u32 = 0x0050_4e60;
const CASINO_SCRIPT_00504E40: u32 = 0x0050_4e40;
const CASINO_00737E30: u32 = 0x0073_7e30;
const GET_ITEM_COUNT_IN_CONTAINER: u32 = 0x0057_5610;
const GET_PICK_UP_SOUND_NAME: u32 = 0x008a_dcf0;
const GET_FULL_NAME: u32 = 0x0048_2720;
const SHOW_MESSAGE_007052F0: u32 = 0x0070_52f0;
const SCRIPT_RUN_0060C9C0: u32 = 0x0060_c9c0;
/// The local container the winnings go through: constructor (`00481610`), add (`004818e0`,
/// `RET 0xc`), give (`004821a0`, `RET 8`) and destructor (`00481680`).
const CONTAINER_CONSTRUCT: u32 = 0x0048_1610;
const CONTAINER_ADD: u32 = 0x0048_18e0;
const CONTAINER_GIVE: u32 = 0x0048_21a0;
const WINNINGS_CONTAINER_DESTRUCT: u32 = 0x0048_1680;
/// The `BSString` helpers: construct (`004037b0`), destroy (`004037d0`), format
/// (`00406f60`, `cdecl`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTROY: u32 = 0x0040_37d0;
const STRING_FORMAT: u32 = 0x0040_6f60;
/// The controller manager's sequence start (`0047aab0`, `RET 0x18`) and stop (`0047b220`,
/// `RET 8`), a sequence's length (`00508100`, `ST0`), a node's child (`0043b4a0`, `RET 4`)
/// and whether a node is culled (`00456610`, `AL`).
const ACTIVATE_SEQUENCE_0047AAB0: u32 = 0x0047_aab0;
const DEACTIVATE_SEQUENCE_0047B220: u32 = 0x0047_b220;
const SEQUENCE_LENGTH_00508100: u32 = 0x0050_8100;
const NODE_CHILD_0043B4A0: u32 = 0x0043_b4a0;
const IS_CULLED_00456610: u32 = 0x0045_6610;
/// `MiscStatManager::Increment` (`004d5c60`, `cdecl`) and the bet list's count (`005ae380`).
const MISC_STAT_INCREMENT: u32 = 0x004d_5c60;
const LIST_COUNT_005AE380: u32 = 0x005a_e380;
/// `ALLOCATE` (`00401000`), `memset` (`00ec61c0`), and `00726c60` (`RET 4`: the list's
/// first item set).
const ALLOCATE: u32 = 0x0040_1000;
const MEMSET: u32 = 0x00ec_61c0;
const LIST_SET_FIRST_00726C60: u32 = 0x0072_6c60;
/// `Create`'s callees: `Tile::GetMenuByClass` (`00a09030`, `cdecl`), the player's casino
/// data list (`005a6260`), the casino data constructor (`00733f50`), `00969b80` and
/// `00969b40` (on the player), the interface manager's +0x9c (`00586150`), `Tile::ReadFile`
/// (`00a01b00`, `RET 4`), `Tile::GetMenu` (`00a03c90`), `Menu::SetMenuTile` (`00a1dc70`,
/// `RET 8`), the debug message (`005b5e40`, `cdecl`), `Tile::GetMaximumDepth` (`00a01530`,
/// `ST0`), `Tile::GetFloat` (`00a011b0`, `RET 4`), a child by name (`00a08b20`, `cdecl`),
/// `Tile::AddNeedsUpdate` (`00a07530`, `RET 4`), `Menu::PrepForVisibility` (`00a1dc20`,
/// `RET 4`) and `ActorValueOwner::GetClampedActorValue` (`0066ef20`, `RET 4`).
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
const CASINO_DATA_LIST_005A6260: u32 = 0x005a_6260;
const CASINO_DATA_CONSTRUCT: u32 = 0x0073_3f50;
const PLAYER_00969B80: u32 = 0x0096_9b80;
const PLAYER_00969B40: u32 = 0x0096_9b40;
const WORD_AT_9C_00586150: u32 = 0x0058_6150;
const TILE_READ_FILE: u32 = 0x00a0_1b00;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
const DEBUG_MESSAGE: u32 = 0x005b_5e40;
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
const TILE_CHILD_BY_NAME_00A08B20: u32 = 0x00a0_8b20;
const TILE_ADD_NEEDS_UPDATE: u32 = 0x00a0_7530;
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
const GET_CLAMPED_ACTOR_VALUE: u32 = 0x0066_ef20;
/// `6006.0` and `102.0` (`double`s).
const VALUE_6006_DOUBLE: u32 = 0x0107_04f0;
const VALUE_102_DOUBLE: u32 = 0x0107_04e8;
/// The render function's callees: `0043c4b0`, `007148c0` (`RET 8`), `0045c670`, `008d80e0`,
/// `00c4f270` (`RET 4`), `00709d50` (`cdecl`), `007fd640`, `00633c90` (`RET 4`: an
/// `NiPointer` from a pointer), `004a0fd0` (`RET 4`), `006629f0`,
/// `BSShaderUtil::AccumulateScene` (`00b6bee0`, `cdecl`), `00b6c0d0` (`cdecl`) and
/// `00c4f2d0`; the camera's `float` (`01030ff4`).
const RENDER_0043C4B0: u32 = 0x0043_c4b0;
const RENDER_007148C0: u32 = 0x0071_48c0;
const RENDER_0045C670: u32 = 0x0045_c670;
const RENDER_008D80E0: u32 = 0x008d_80e0;
const RENDER_00C4F270: u32 = 0x00c4_f270;
const RENDER_00709D50: u32 = 0x0070_9d50;
const RENDER_007FD640: u32 = 0x007f_d640;
const NI_POINTER_FROM_00633C90: u32 = 0x0063_3c90;
const RENDER_004A0FD0: u32 = 0x004a_0fd0;
const RENDER_006629F0: u32 = 0x0066_29f0;
const ACCUMULATE_SCENE: u32 = 0x00b6_bee0;
const RENDER_00B6C0D0: u32 = 0x00b6_c0d0;
const RENDER_00C4F2D0: u32 = 0x00c4_f2d0;
const CAMERA_VALUE_01030FF4: u32 = 0x0103_0ff4;
/// Sounds.
const SOUND_BET_01076324: u32 = 0x0107_6324;
const SOUND_REMOVE_01076294: u32 = 0x0107_6294;
const SOUND_CHANGE_0107627C: u32 = 0x0107_627c;
const SOUND_SPIN_01076310: u32 = 0x0107_6310;
const SOUND_WIN_01076300: u32 = 0x0107_6300;
const SOUND_INVALID_010762AC: u32 = 0x0107_62ac;
const SOUND_REFUSE_01025CC4: u32 = 0x0102_5cc4;
/// The menu's file (`01076254`), the creation failure message (`01076200`), the chip tile's
/// child name (`010761e4`), formats and other texts.
const MENU_FILE_TEXT: u32 = 0x0107_6254;
const CREATION_FAILED_TEXT: u32 = 0x0107_6200;
const CHIP_CHILD_NAME: u32 = 0x0107_61e4;
const BANNED_FORMAT_01076274: u32 = 0x0107_6274;
const MESSAGE_ICON_01049638: u32 = 0x0104_9638;
const EMPTY_STRING_01020770: u32 = 0x0102_0770;
const OUTSIDE_BET_FORMAT_010762C4: u32 = 0x0107_62c4;
const RESULT_FORMAT_01070D0C: u32 = 0x0107_0d0c;
const ONE_STRING_FORMAT: u32 = 0x0101_9f08;
const LOST_FORMAT_0101C178: u32 = 0x0101_c178;
const WON_FORMAT_01070D8C: u32 = 0x0107_0d8c;
const LOST_ICON_010208A0: u32 = 0x0102_08a0;
const WON_ICON_0103A830: u32 = 0x0103_a830;

/// Plays a menu sound by name (`QInstance`, `GetSoundHandleByName(&handle, name, 0x121)`,
/// `Play(0)`, the handle destroyed).
fn play_sound(e: &mut Engine, name: u32) {
    e.with_stack(0xc, |e, handle| {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let found = e
            .call(
                GET_SOUND_HANDLE_BY_NAME,
                &args![audio, handle, name, 0x121u32],
            )
            .u32();
        e.call(SOUND_HANDLE_PLAY, &args![found, 0u32]);
        e.call(SOUND_HANDLE_DESTROY, &args![handle]);
    });
}

/// `SetTileTextAlpha(index, alpha)` on the open menu with the `float` at an address (loaded
/// with `FLD`).
fn tile_alpha(e: &mut Engine, index: u32, alpha_at: u32) {
    let alpha = x87_quiet_bits(e.global::<u32>(alpha_at));
    let menu = Ptr::new(roulette_menu(e));
    roulette_menu_set_tile_text_alpha(e, menu, index, alpha);
}

/// `Tile::Poke(trait, 1.0)` then `SetInt(trait, value)` on the open menu's tile at an offset
/// (each read of the menu as the code does it).
fn show_number(e: &mut Engine, offset: u32, trait_at: u32, value: u32) {
    let trait_id = e.global::<u32>(trait_at);
    let tile = e.mem.u32(roulette_menu(e).wrapping_add(offset));
    e.call(TILE_POKE, &args![tile, trait_id, 1.0f32.to_bits()]);
    let trait_id = e.global::<u32>(trait_at);
    let tile = e.mem.u32(roulette_menu(e).wrapping_add(offset));
    e.call(TILE_SET_INT, &args![tile, trait_id, value]);
}

/// Child `index` of the model node (`0043b4a0`).
fn model_child(e: &mut Engine, index: u32) -> u32 {
    let node = model(e);
    e.call(NODE_CHILD_0043B4A0, &args![node, index]).u32()
}

/// A point from (0.0, 0, 0) (`0043d410`) given to `00a59c60` of a model child.
fn reset_child_translate(e: &mut Engine, index: u32) {
    e.with_stack(0xc, |e, point| {
        let made = e
            .call(
                POINT_CONSTRUCT_0043D410,
                &args![point, 0.0f32.to_bits(), 0u32, 0u32],
            )
            .u32();
        let child = model_child(e, index);
        e.call(SET_TRANSLATE_00A59C60, &args![child, made]);
    });
}

/// The message box `Create` and the idle show with a game setting's text as the format:
/// `sprintf_s(buffer, 0x104, setting)`, then `007052f0(buffer, 0, 01049638, 0, float at
/// 010162c0, 0)` and the refusal sound.
fn refuse(e: &mut Engine, setting: u32) {
    e.with_stack(0x104, |e, buffer| {
        let format = e.call(SETTING_STRING, &args![setting]).u32();
        e.call(SPRINTF_S, &args![buffer, 0x104u32, format]);
        show_message(e, buffer.addr(), MESSAGE_ICON_01049638, 0);
    });
    play_sound(e, SOUND_REFUSE_01025CC4);
}

/// `007052f0(text, 0, icon, sound, float at 010162c0, 0)`.
fn show_message(e: &mut Engine, text: u32, icon: u32, sound: u32) {
    let value = x87_quiet_bits(e.global::<u32>(MESSAGE_VALUE_010162C0));
    e.call(
        SHOW_MESSAGE_007052F0,
        &args![text, 0u32, icon, sound, value, 0u32],
    );
}

// Translated from 007bbcf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007bbcf0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the NiPointer local) is not translated.
/// Unnamed (no arguments): the roulette menu's render function (the interface calls it for
/// the menu). `007148c0(0, 4)` on `0043c4b0()`; with the accumulator `008d80e0` of
/// `0045c670()`: `00c4f270(1)`, the camera (`sp3DCamera`) given to `00709d50` with the
/// `float` at `01030ff4`, an `NiPointer` local from the interface manager's `007fd640`
/// set on the accumulator (`004a0fd0`); the scene (the camera, or `006629f0` of the object
/// without one) accumulated (`AccumulateScene(scene, 00450b80(1), accumulator)`) and drawn
/// (`00b6c0d0(scene, the pointer, 0)`); then `004a0fd0(0)`, the local cleared, `00c4f2d0` and
/// the local destroyed.
pub fn fn_007bbcf0(e: &mut Engine) {
    let object = e.call(RENDER_0043C4B0, &args![]).u32();
    e.call(RENDER_007148C0, &args![object, 0u32, 4u32]);
    let source = e.call(RENDER_0045C670, &args![]).u32();
    let accumulator = e.call(RENDER_008D80E0, &args![source]).u32();
    e.call(RENDER_00C4F270, &args![accumulator, 1u32]);
    let camera = e.call(WORD_AT_0_00559450, &args![CAMERA_011DA8D0]).u32();
    let value = x87_quiet_bits(e.global::<u32>(CAMERA_VALUE_01030FF4));
    e.call(RENDER_00709D50, &args![camera, value]);
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    let target = e.call(RENDER_007FD640, &args![interface]).u32();
    e.with_stack(4, |e, local| {
        e.call(NI_POINTER_FROM_00633C90, &args![local, target]);
        let pointer = e.call(WORD_AT_0_00559450, &args![local]).u32();
        e.call(RENDER_004A0FD0, &args![accumulator, pointer]);
        let scene = if camera != 0 {
            camera
        } else {
            e.call(RENDER_006629F0, &args![source]).u32()
        };
        let node = e.call(SCENE_00450B80, &args![1u32]).u32();
        e.call(ACCUMULATE_SCENE, &args![scene, node, accumulator]);
        let pointer = e.call(WORD_AT_0_00559450, &args![local]).u32();
        e.call(RENDER_00B6C0D0, &args![scene, pointer, 0u32]);
        e.call(RENDER_004A0FD0, &args![accumulator, 0u32]);
        e.call(NI_POINTER_ASSIGN, &args![local, 0u32]);
        e.call(RENDER_00C4F2D0, &args![accumulator]);
        e.call(NI_POINTER_DESTROY, &args![local]);
    });
}

/// `UpdateAll`-style labels `Create` sets: the tiles at +0x38 to +0x4c and their game
/// settings.
const CREATE_LABELS: [(u32, u32); 6] = [
    (0x38, 0x011d_41e0),
    (0x3c, 0x011d_2c1c),
    (0x40, 0x011d_2820),
    (0x44, 0x011d_3354),
    (0x48, 0x011d_487c),
    (0x4c, 0x011d_33fc),
];

// Translated from 007bbe20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007bbe20 - the C++ exception frame (FS:[0] chain, unwind states 0 to 4) and the /GS cookie check (00ec408c) are not translated.
/// `RouletteMenu::Create` (Xbox PDB; `cdecl`: the casino, the minimum bet, the maximum bet
/// and the chips needed). The casino becomes `pCasino`; its data is found in the player's
/// casino data list (`005a6260`, by `0084e3a0` of the casino) or made (0xc bytes,
/// `00733f50`: the casino's key, 0 chips) and added. Refused with a message box and the
/// refusal sound (returning 0): when `00969b80` holds on the player (the banned message
/// `01076274` with `00969b40`), when the casino data's winnings reached the casino's limit
/// (`00504e60` +0x28; setting `011d3abc`), with fewer chips than the minimum bet (setting
/// `011d2424`) or fewer winnings than the chips needed (setting `011d3594`). An open menu
/// of the class is deleted; the menu is read from `01076254`; one of another class (or
/// none) is deleted when it has a tile and 0 returned. Otherwise the bets, chips
/// (`GetItemCountinContainer` of the casino's item) and luck (actor value 0xb) are set,
/// the menu tile attached; without all its tiles a debug message is logged, the state set
/// to 5 and 0 returned. The depth trait, the bet, total and chip tiles, the casino's name
/// (or its vtable +0x130 name) with setting `011d3e08`, the chip text (`007bf9e0`), the
/// labels and the tile alphas are set up, the 3D elements prepared and the menu made
/// visible; the menu tile is returned.
pub fn roulette_menu_create(
    e: &mut Engine,
    casino: u32,
    minimum: i32,
    maximum: i32,
    needed: i32,
) -> u32 {
    let existing = e.call(TILE_GET_MENU_BY_CLASS, &args![0x43au32]).u32();
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    e.set_global(CASINO_011DA880, casino);
    let player = e.global::<u32>(PLAYER_011DEA3C);
    let mut node = e.call(CASINO_DATA_LIST_005A6260, &args![player]).u32();
    while node != 0 && node_item(e, node) != 0 {
        let casino = e.global::<u32>(CASINO_011DA880);
        let key = e.call(KEY_0084E3A0, &args![casino]).u32();
        let data = node_item(e, node);
        if key == e.mem.u32(data) {
            let data = node_item(e, node);
            e.set_global(CASINO_DATA_011DA87C, data);
            break;
        }
        if e.call(WORD_AT_4_00726070, &args![node]).u32() == 0 {
            break;
        }
        node = e.call(WORD_AT_4_00726070, &args![node]).u32();
    }
    if e.global::<u32>(CASINO_DATA_011DA87C) == 0 {
        e.with_stack(4, |e, local| {
            let block = e.call(ALLOCATE, &args![0xcu32]).u32();
            let data = if block != 0 {
                e.call(CASINO_DATA_CONSTRUCT, &args![block]).u32()
            } else {
                0
            };
            e.mem.set_u32(local.addr(), data);
            let casino = e.global::<u32>(CASINO_011DA880);
            let key = e.call(KEY_0084E3A0, &args![casino]).u32();
            let data = e.mem.u32(local.addr());
            e.mem.set_u32(data, key);
            e.mem.set_u32(data.wrapping_add(4), 0);
            let player = e.global::<u32>(PLAYER_011DEA3C);
            let list = e.call(CASINO_DATA_LIST_005A6260, &args![player]).u32();
            if list == 0 || e.call(LIST_IS_EMPTY, &args![list]).bool() {
                e.call(LIST_SET_FIRST_00726C60, &args![list, local]);
            } else {
                e.call(LIST_APPEND, &args![list, local]);
            }
            let data = e.mem.u32(local.addr());
            e.set_global(CASINO_DATA_011DA87C, data);
        });
    }
    let player = e.global::<u32>(PLAYER_011DEA3C);
    if e.call(PLAYER_00969B80, &args![player]).bool() {
        e.with_stack(0x104, |e, buffer| {
            let player = e.global::<u32>(PLAYER_011DEA3C);
            let count = e.call(PLAYER_00969B40, &args![player]).u32();
            let second = e.call(SETTING_STRING, &args![0x011d_2274u32]).u32();
            let first = e.call(SETTING_STRING, &args![0x011d_2d24u32]).u32();
            e.call(
                SPRINTF_S,
                &[
                    buffer.addr(),
                    0x104,
                    BANNED_FORMAT_01076274,
                    first,
                    second,
                    count,
                ],
            );
            show_message(e, buffer.addr(), MESSAGE_ICON_01049638, 0);
        });
        play_sound(e, SOUND_REFUSE_01025CC4);
        return 0;
    }
    let casino = e.global::<u32>(CASINO_011DA880);
    let limits = e.call(CASINO_LIMITS_00504E60, &args![casino]).u32();
    let data = e.global::<u32>(CASINO_DATA_011DA87C);
    if e.mem.u32(data.wrapping_add(4)) as i32 >= e.mem.u32(limits.wrapping_add(0x28)) as i32 {
        refuse(e, 0x011d_3abc);
        return 0;
    }
    let casino = e.global::<u32>(CASINO_011DA880);
    let item = e.call(CASINO_ITEM_007316D0, &args![casino]).u32();
    let player = e.global::<u32>(PLAYER_011DEA3C);
    if e.call(GET_ITEM_COUNT_IN_CONTAINER, &args![player, item])
        .i32()
        < minimum
    {
        refuse(e, 0x011d_2424);
        return 0;
    }
    let data = e.global::<u32>(CASINO_DATA_011DA87C);
    if (e.mem.u32(data.wrapping_add(4)) as i32) < needed && needed != 0 {
        refuse(e, 0x011d_3594);
        return 0;
    }
    if existing != 0 {
        e.vcall(existing, 0, &args![1u32]);
    }
    let root = e.call(WORD_AT_9C_00586150, &args![interface]).u32();
    let tile = e.call(TILE_READ_FILE, &args![root, MENU_FILE_TEXT]).u32();
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    e.set_global(ROULETTE_MENU_011DA878, menu);
    let created = roulette_menu(e) != 0 && {
        let menu = roulette_menu(e);
        let class = e.vcall(menu, 0x34, &args![]).u32();
        class == fn_007bc820(e)
    };
    if !created {
        let menu = roulette_menu(e);
        if menu != 0 && e.call(WORD_AT_4_00726070, &args![menu]).u32() != 0 {
            let menu = roulette_menu(e);
            e.vcall(menu, 0, &args![1u32]);
        }
        return 0;
    }
    let me = Ptr::<RouletteMenu>::new(roulette_menu(e));
    e.set(me, RouletteMenu::iMinBet, minimum);
    e.set(me, RouletteMenu::iMaxBet, maximum);
    e.set(me, RouletteMenu::iCurrentBet, minimum);
    let casino = e.global::<u32>(CASINO_011DA880);
    let item = e.call(CASINO_ITEM_007316D0, &args![casino]).u32();
    let player = e.global::<u32>(PLAYER_011DEA3C);
    let chips = e
        .call(GET_ITEM_COUNT_IN_CONTAINER, &args![player, item])
        .i32();
    e.set(me, RouletteMenu::iChipCount, chips);
    let player = e.global::<u32>(PLAYER_011DEA3C);
    let luck = e
        .call(
            GET_CLAMPED_ACTOR_VALUE,
            &args![player.wrapping_add(0xa4), 0xbu32],
        )
        .i32();
    e.set(me, RouletteMenu::iLuckValue, luck);
    e.set(me, RouletteMenu::iTotalBet, 0);
    e.call(MENU_SET_MENU_TILE, &args![me, tile, 0u32]);
    if !fn_007bc830(e, me) {
        e.call(DEBUG_MESSAGE, &args![CREATION_FAILED_TEXT]);
        e.set(me, RouletteMenu::eRouletteState, 5);
        return 0;
    }
    let depth = e.call(TILE_GET_FLOAT, &args![tile, 0xfa7u32]).f64();
    let high: f64 = e.global(VALUE_6006_DOUBLE);
    let depth_matches = depth == high || {
        let depth = e.call(TILE_GET_FLOAT, &args![tile, 0xfa7u32]).f64();
        let low: f64 = e.global(VALUE_102_DOUBLE);
        depth == low
    };
    if depth_matches {
        let maximum_depth = e.call(TILE_GET_MAXIMUM_DEPTH, &args![]).f64();
        e.call(
            TILE_SET_FLOAT,
            &args![tile, 0xfadu32, x87_store32(maximum_depth), 1u32],
        );
    }
    for (offset, setting, value) in [
        (TILES, 0x011d_3804u32, None),
        (TILES + 0x28, 0x011d_3168, Some(0u32)),
        (TILES + 4, 0x011d_3300, None),
    ] {
        let text = e.call(SETTING_STRING, &args![setting]).u32();
        let trait_id = e.global::<u32>(TRAIT_011DA870);
        let target = e.mem.u32(roulette_menu(e).wrapping_add(offset));
        e.call(TILE_SET_STRING, &args![target, trait_id, text, 1u32]);
        let value = match value {
            Some(value) => value,
            None if offset == TILES => e.get(
                Ptr::<RouletteMenu>::new(roulette_menu(e)),
                RouletteMenu::iCurrentBet,
            ) as u32,
            None => e.get(
                Ptr::<RouletteMenu>::new(roulette_menu(e)),
                RouletteMenu::iChipCount,
            ) as u32,
        };
        let trait_id = e.global::<u32>(TRAIT_011DA86C);
        let target = e.mem.u32(roulette_menu(e).wrapping_add(offset));
        e.call(TILE_POKE, &args![target, trait_id, 1.0f32.to_bits()]);
        let trait_id = e.global::<u32>(TRAIT_011DA86C);
        let target = e.mem.u32(roulette_menu(e).wrapping_add(offset));
        e.call(TILE_SET_INT, &args![target, trait_id, value]);
    }
    e.with_stack(0x40, |e, title| {
        let casino = e.global::<u32>(CASINO_011DA880);
        let name = e
            .call(NAME_CHARS_00408DA0, &args![casino.wrapping_add(0x18)])
            .u32();
        if e.call(STRLEN, &args![name]).u32() != 0 {
            let setting = e.call(SETTING_STRING, &args![SETTING_011D3E08]).u32();
            let casino = e.global::<u32>(CASINO_011DA880);
            let name = e
                .call(NAME_CHARS_00408DA0, &args![casino.wrapping_add(0x18)])
                .u32();
            e.call(
                SPRINTF_S,
                &[title.addr(), 0x40, NAME_FORMAT_01012058, name, setting],
            );
        } else {
            let setting = e.call(SETTING_STRING, &args![SETTING_011D3E08]).u32();
            let casino = e.global::<u32>(CASINO_011DA880);
            let name = e.vcall(casino, 0x130, &args![]).u32();
            e.call(
                SPRINTF_S,
                &[title.addr(), 0x40, NAME_FORMAT_01012058, name, setting],
            );
        }
        let trait_id = e.global::<u32>(TRAIT_011DA870);
        let target = e.mem.u32(roulette_menu(e) + 0x34);
        e.call(TILE_SET_STRING, &args![target, trait_id, title, 1u32]);
    });
    let target = e.mem.u32(roulette_menu(e) + 0x34);
    let child = e
        .call(TILE_CHILD_BY_NAME_00A08B20, &args![target, CHIP_CHILD_NAME])
        .u32();
    e.call(TILE_SET_INT, &args![child, 0xfa3u32, 0u32]);
    let target = e.mem.u32(roulette_menu(e) + 0x34);
    let child = e
        .call(TILE_CHILD_BY_NAME_00A08B20, &args![target, CHIP_CHILD_NAME])
        .u32();
    e.call(TILE_ADD_NEEDS_UPDATE, &args![child, 8u32]);
    let me = Ptr::<RouletteMenu>::new(roulette_menu(e));
    fn_007bf9e0(e, me);
    for (offset, setting) in CREATE_LABELS {
        let text = e.call(SETTING_STRING, &args![setting]).u32();
        let target = e.mem.u32(roulette_menu(e) + offset);
        e.call(TILE_SET_STRING, &args![target, 0xfc4u32, text, 1u32]);
    }
    let target = e.mem.u32(roulette_menu(e) + 0x50);
    e.call(
        TILE_SET_STRING,
        &args![target, 0xfc4u32, EMPTY_STRING_01020770, 1u32],
    );
    let target = e.mem.u32(roulette_menu(e) + 0x50);
    e.call(TILE_SET_INT, &args![target, 0xfa3u32, 1u32]);
    for index in [5u32, 4, 7] {
        tile_alpha(e, index, HIDDEN_ALPHA_0101E704);
    }
    let me = Ptr::<RouletteMenu>::new(roulette_menu(e));
    roulette_menu_prepare_3d_elements(e, me);
    let me = roulette_menu(e);
    e.call(MENU_PREP_FOR_VISIBILITY, &args![me, 0u32]);
    tile
}

// Translated from 007bc870 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007bc870 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 around the sound handles) is not translated.
/// `RouletteMenu::DoClick` (engine map, vtable; `RET 8`: a tile id and an unread tile).
/// Only while betting (state 4). Id 3 places a bet where the cursor is (`GetBetIndex`,
/// under the maximum, `SetBet`): the total shows, the bet is cut to what the maximum
/// allows (hiding the place, raise and lower tiles), the clear and spin tiles show, and at
/// ten bets the bet becomes 0. Id 4 removes the bet under the cursor (`RemoveBet`) with a
/// sound, showing the total; with no bets left the spin and clear tiles hide, at nine all
/// show again. Id 5 spins when there are bets (`007bf670`, the tiles hidden, state 1, misc
/// stat 0x28). Id 6 raises and id 7 lowers the bet by its step (1, 5, 10, 25, 100; lowering
/// rounds down to a step first), kept within the chips, the maximum and the minimum, the
/// chip stack updated, a sound when it changed and the bet shown. Id 8 closes.
pub fn roulette_menu_do_click(e: &mut Engine, this: Ptr<RouletteMenu>, id: u32, _unused_2: u32) {
    if e.get(this, RouletteMenu::eRouletteState) != 4 {
        return;
    }
    match id.wrapping_sub(3) {
        0 => {
            let index = roulette_menu_get_bet_index(e, this);
            if (index as i32) >= 0x9f
                || e.get(this, RouletteMenu::iTotalBet) >= e.get(this, RouletteMenu::iMaxBet)
                || !roulette_menu_set_bet(e, this, index)
            {
                return;
            }
            let total = e.get(this, RouletteMenu::iTotalBet) as u32;
            show_number(e, TILES + 0x28, TRAIT_011DA86C, total);
            if e.get(this, RouletteMenu::iTotalBet)
                .wrapping_add(e.get(this, RouletteMenu::iCurrentBet))
                >= e.get(this, RouletteMenu::iMaxBet)
            {
                let bet = e
                    .get(this, RouletteMenu::iMaxBet)
                    .wrapping_sub(e.get(this, RouletteMenu::iTotalBet));
                e.set(this, RouletteMenu::iCurrentBet, bet);
                show_number(e, TILES, TRAIT_011DA86C, bet as u32);
                for index in [3u32, 6, 7] {
                    tile_alpha(e, index, HIDDEN_ALPHA_0101E704);
                }
            }
            for index in [4u32, 5] {
                tile_alpha(e, index, CURSOR_FADE_01023CD8);
            }
            if e.get(this, RouletteMenu::iCurrentBet) == 0 {
                for index in [3u32, 7] {
                    tile_alpha(e, index, HIDDEN_ALPHA_0101E704);
                }
            }
            if e.get(this, RouletteMenu::iNumBetsSet) == 10 {
                e.set(this, RouletteMenu::iCurrentBet, 0);
                show_number(e, TILES, TRAIT_011DA86C, 0);
                for index in [3u32, 6, 7] {
                    tile_alpha(e, index, HIDDEN_ALPHA_0101E704);
                }
            }
        }
        1 => {
            let index = roulette_menu_get_bet_index(e, this);
            if (index as i32) >= 0x9f || !roulette_menu_remove_bet(e, this, index) {
                return;
            }
            play_sound(e, SOUND_REMOVE_01076294);
            let total = e.get(this, RouletteMenu::iTotalBet) as u32;
            show_number(e, TILES + 0x28, TRAIT_011DA86C, total);
            for index in [6u32, 3] {
                tile_alpha(e, index, CURSOR_FADE_01023CD8);
            }
            let list = e.get(this, RouletteMenu::pBetList);
            if e.call(LIST_COUNT_005AE380, &args![list]).u32() == 0 {
                for index in [5u32, 4] {
                    tile_alpha(e, index, HIDDEN_ALPHA_0101E704);
                }
            }
            if e.get(this, RouletteMenu::iNumBetsSet) == 9 {
                for index in [5u32, 4, 3, 6, 7, 8] {
                    tile_alpha(e, index, CURSOR_FADE_01023CD8);
                }
            }
        }
        2 => {
            let list = e.get(this, RouletteMenu::pBetList);
            if list == 0 || e.call(LIST_IS_EMPTY, &args![list]).bool() {
                return;
            }
            fn_007bf670(e, this);
            for index in [5u32, 4, 3, 6, 7, 8] {
                tile_alpha(e, index, HIDDEN_ALPHA_0101E704);
            }
            e.set(this, RouletteMenu::eRouletteState, 1);
            e.call(MISC_STAT_INCREMENT, &args![0x28u32]);
        }
        3 => {
            let previous = e.get(this, RouletteMenu::iCurrentBet);
            let bet = e.get(this, RouletteMenu::iCurrentBet);
            let step = if bet < 10 {
                1
            } else if bet < 0x28 {
                5
            } else if bet < 0x64 {
                10
            } else if bet < 0x1f4 {
                0x19
            } else {
                0x64
            };
            e.set(this, RouletteMenu::iCurrentBet, bet.wrapping_add(step));
            tile_alpha(e, 7, CURSOR_FADE_01023CD8);
            finish_bet_change(e, this, previous, true);
        }
        4 => {
            let previous = e.get(this, RouletteMenu::iCurrentBet);
            let bet = e.get(this, RouletteMenu::iCurrentBet);
            let lowered = if bet <= 10 {
                bet.wrapping_sub(1)
            } else {
                let step = if bet <= 0x28 {
                    5
                } else if bet <= 0x64 {
                    10
                } else if bet <= 0x1f4 {
                    0x19
                } else {
                    0x64
                };
                if bet % step != 0 {
                    bet / step * step
                } else {
                    bet - step
                }
            };
            e.set(this, RouletteMenu::iCurrentBet, lowered);
            tile_alpha(e, 6, CURSOR_FADE_01023CD8);
            finish_bet_change(e, this, previous, false);
        }
        5 => roulette_menu_close(e),
        _ => {}
    }
}

/// The end of `DoClick`'s raise (`raise`) and lower: the raise keeps the bet within the
/// chips and the maximum (hiding the raise tile), the lower raises it to the minimum when
/// that fits under the maximum and keeps it at least 0 (hiding the lower tile); the chip
/// stack is updated unless ten bets are set; a changed bet plays its sound; the bet shows.
fn finish_bet_change(e: &mut Engine, this: Ptr<RouletteMenu>, previous: i32, raise: bool) {
    if raise {
        if e.get(this, RouletteMenu::iCurrentBet)
            .wrapping_add(e.get(this, RouletteMenu::iTotalBet))
            >= e.get(this, RouletteMenu::iChipCount)
        {
            let bet = e
                .get(this, RouletteMenu::iChipCount)
                .wrapping_sub(e.get(this, RouletteMenu::iTotalBet));
            e.set(this, RouletteMenu::iCurrentBet, bet);
            tile_alpha(e, 6, HIDDEN_ALPHA_0101E704);
        }
        if e.get(this, RouletteMenu::iTotalBet)
            .wrapping_add(e.get(this, RouletteMenu::iCurrentBet))
            >= e.get(this, RouletteMenu::iMaxBet)
        {
            let bet = e
                .get(this, RouletteMenu::iMaxBet)
                .wrapping_sub(e.get(this, RouletteMenu::iTotalBet));
            e.set(this, RouletteMenu::iCurrentBet, bet);
            tile_alpha(e, 6, HIDDEN_ALPHA_0101E704);
        }
    } else {
        if e.get(this, RouletteMenu::iCurrentBet) <= e.get(this, RouletteMenu::iMinBet)
            && e.get(this, RouletteMenu::iMinBet)
                .wrapping_add(e.get(this, RouletteMenu::iTotalBet))
                < e.get(this, RouletteMenu::iMaxBet)
        {
            let minimum = e.get(this, RouletteMenu::iMinBet);
            e.set(this, RouletteMenu::iCurrentBet, minimum);
            tile_alpha(e, 7, HIDDEN_ALPHA_0101E704);
        }
        if e.get(this, RouletteMenu::iCurrentBet) <= 0 {
            e.set(this, RouletteMenu::iCurrentBet, 0);
            tile_alpha(e, 7, HIDDEN_ALPHA_0101E704);
        }
    }
    if e.get(this, RouletteMenu::iNumBetsSet) != 10 {
        roulette_menu_set_bet_chip(e, this);
    }
    if previous != e.get(this, RouletteMenu::iCurrentBet) {
        play_sound(e, SOUND_CHANGE_0107627C);
    }
    let bet = e.get(this, RouletteMenu::iCurrentBet) as u32;
    show_number(e, TILES, TRAIT_011DA86C, bet);
}

/// The magnitude test of the stick values: `00408820` of the value as a `float` compared
/// with the dead zone (`FCOMP; TEST AH,0x41; JP`): kept when above it (or unordered),
/// cleared otherwise.
fn stick_dead_zone(e: &mut Engine, value: i32) -> i32 {
    let magnitude = x87_store32(f64::from(value));
    let magnitude = e.call(ABS_00408820, &args![magnitude]).f64();
    let dead_zone: f64 = e.global(STICK_DEAD_ZONE_010718F0);
    if magnitude <= dead_zone {
        0
    } else {
        value
    }
}

/// The input object (`00877720` of the object at `011dea0c`).
fn input(e: &mut Engine) -> u32 {
    let object = e.global::<u32>(INPUT_011DEA0C);
    e.call(INPUT_00877720, &args![object]).u32()
}

/// A mouse axis as the idle scales it: `(width / 00706e50()) * value / 01020758` (each
/// call's `ST0` kept as a `double`).
fn mouse_axis(e: &mut Engine, value: i32) -> u32 {
    let value = f64::from(value);
    let width = e.call(SCREEN_WIDTH, &args![]).f64();
    let other = e.call(SCREEN_00706E50, &args![]).f64();
    let scale: f64 = e.global(MOUSE_SCALE_01020758);
    x87_store32(x87_div(x87_mul(x87_div(width, other), value), scale))
}

// Translated from 007bd2a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007bd2a0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 9) and the /GS cookie check (00ec408c) are not translated.
/// `RouletteMenu::DoIdle` (engine map, vtable; `thiscall`). The function-local static at
/// `011da908` is made once (`0041a250`, destroyed at exit). With the menu on top the spin
/// time (`011da904`) advances by the milliseconds since the last idle (at most 1000; cleared
/// when the state changed or on the first idle) over 1000; the cursor tile's `0xfa9` is 0.0
/// on top, else the `float` at `01023cd8`. The cursor input is the sticks (`00a23390` axes 7
/// and 8 past the dead zone, negated, over the `double` at `01029790`) with `004b71d0`, else
/// the mouse (`00a239e0` 1 negated and 2, `mouse_axis`). By state: 5 (closing) fades out
/// once the fade allows it, settles the chips with the player's inventory (removing the
/// losses with a message, giving the winnings through a container, with a message unless
/// over the casino's limit) and runs the casino's end script when due; 1 (spinning) starts
/// the spin animation and the wheel at 0.0, moves the table with the time, clears the
/// table at half the animation and goes to payout (3) at its end; 3 (payout) pays every
/// bet whose numbers hold the result (or the outside bets' ranges, columns, colours and
/// parities; `iCheatToggle` forces or blocks it), updates the chips, the casino data and
/// the message, resets the bets and checks the casino's payout levels (setting
/// `bRunScriptOnExit` and closing); 4 (betting) moves the cursor within its limits, the
/// next chip and its shadow with it, and shows the bet under it (or the outside message)
/// with the valid or invalid face; then the bet is kept within the chips and, below the
/// minimum bet, a message is shown and the menu closed.
pub fn roulette_menu_do_idle(e: &mut Engine, this: Ptr<RouletteMenu>) {
    let base = this.addr();
    let guard = e.global::<u32>(IDLE_GUARD_011DA914);
    if guard & 1 == 0 {
        e.set_global(IDLE_GUARD_011DA914, guard | 1);
        e.call(IDLE_STATIC_CONSTRUCT_0041A250, &args![IDLE_STATIC_011DA908]);
        e.call(ATEXIT, &args![IDLE_STATIC_DESTRUCT]);
    }
    let class = e.vcall(base, 0x34, &args![]).u32();
    let on_top = e.call(INTERFACE_IS_TOP_MENU_ID, &args![class]).bool();
    if e.get(this, RouletteMenu::uiPrevIdleTick) == 0 {
        e.set_global(SPIN_TIME_011DA904, 0.0f32.to_bits());
    } else if on_top {
        let now = e.call(WORD_AT_14_00825C00, &args![TIMER_011F6394]).u32();
        let mut elapsed = now.wrapping_sub(e.get(this, RouletteMenu::uiPrevIdleTick)) as i32;
        if elapsed > 1000 {
            elapsed = 0;
        }
        if e.global::<u32>(LAST_STATE_011A081C) as i32 != e.get(this, RouletteMenu::eRouletteState)
        {
            e.set_global(SPIN_TIME_011DA904, 0.0f32.to_bits());
        } else {
            let thousand: f64 = e.global(THOUSAND_DOUBLE);
            let time = x87_add(
                x87_div(f64::from(elapsed), thousand),
                x87_m32(e.global::<u32>(SPIN_TIME_011DA904)),
            );
            e.set_global(SPIN_TIME_011DA904, x87_store32(time));
        }
    }
    if !on_top {
        let fade = x87_quiet_bits(e.global::<u32>(CURSOR_FADE_01023CD8));
        set_cursor_fade(e, fade);
    } else {
        set_cursor_fade(e, 0.0f32.to_bits());
    }
    let mut other = 0i32;
    e.call(INTERFACE_MANAGER, &args![]);
    let (x, y) = if e.call(FLAG_004B71D0, &args![]).bool() {
        let object = input(e);
        let stick_x = e.call(STICK_00A23390, &args![object, 0u32, 7u32]).i32();
        let object = input(e);
        let stick_y = e.call(STICK_00A23390, &args![object, 0u32, 8u32]).i32();
        let stick_x = stick_dead_zone(e, stick_x);
        other = stick_dead_zone(e, other);
        let scale: f64 = e.global(STICK_SCALE_01029790);
        let x = x87_store32(x87_div(f64::from(stick_x.wrapping_neg()), scale));
        let stick_y = stick_dead_zone(e, stick_y);
        let scale: f64 = e.global(STICK_SCALE_01029790);
        let y = x87_store32(x87_div(f64::from(stick_y.wrapping_neg()), scale));
        (x, y)
    } else {
        let object = input(e);
        let value = e.call(MOUSE_00A239E0, &args![object, 1u32]).i32();
        let x = mouse_axis(e, value.wrapping_neg());
        let object = input(e);
        let value = e.call(MOUSE_00A239E0, &args![object, 2u32]).i32();
        (x, mouse_axis(e, value))
    };
    let now = e.call(WORD_AT_14_00825C00, &args![TIMER_011F6394]).u32();
    e.set(this, RouletteMenu::uiPrevIdleTick, now);
    let state = e.get(this, RouletteMenu::eRouletteState);
    e.set_global(LAST_STATE_011A081C, state as u32);
    match state.wrapping_sub(1) {
        0 => idle_spin(e, this),
        2 => idle_payout(e, this, other),
        3 => idle_betting(e, this, x, y),
        4 => idle_closing(e, this),
        _ => {}
    }
}

/// `DoIdle` while closing (state 5); see there.
fn idle_closing(e: &mut Engine, this: Ptr<RouletteMenu>) {
    if e.call(MENU_FADE_STATE_0059BB30, &args![this]).u32() == 2 {
        return;
    }
    let fade = x87_quiet_bits(e.global::<u32>(CURSOR_FADE_01023CD8));
    set_cursor_fade(e, fade);
    let root = e.call(WORD_AT_4_00726070, &args![this]).u32();
    e.call(TILE_SET_INT, &args![root, 0x1772u32, 1u32]);
    e.call(MENU_START_FADE_OUT, &args![this]);
    let casino = e.global::<u32>(CASINO_011DA880);
    let item = e.call(CASINO_ITEM_007316D0, &args![casino]).u32();
    let player = e.global::<u32>(PLAYER_011DEA3C);
    let count = e
        .call(GET_ITEM_COUNT_IN_CONTAINER, &args![player, item])
        .i32();
    let difference = e.get(this, RouletteMenu::iChipCount).wrapping_sub(count);
    if difference < 0 {
        let casino = e.global::<u32>(CASINO_011DA880);
        let item = e.call(CASINO_ITEM_007316D0, &args![casino]).u32();
        let player = e.global::<u32>(PLAYER_011DEA3C);
        e.vcall(
            player,
            0x17c,
            &args![
                item,
                0u32,
                difference.wrapping_neg() as u32,
                0u32,
                0u32,
                0u32,
                0u32,
                0u32,
                1u32,
                0u32
            ],
        );
        e.with_stack(8, |e, text| {
            e.call(STRING_CONSTRUCT, &args![text]);
            let last = e.call(SETTING_STRING, &args![0x011d_3db4u32]).u32();
            let middle = e.call(SETTING_STRING, &args![0x011d_3114u32]).u32();
            let casino = e.global::<u32>(CASINO_011DA880);
            let item = e.call(CASINO_ITEM_007316D0, &args![casino]).u32();
            let name = e.call(GET_FULL_NAME, &args![item]).u32();
            e.call(
                STRING_FORMAT,
                &[
                    text.addr(),
                    LOST_FORMAT_0101C178,
                    difference.wrapping_neg() as u32,
                    name,
                    middle,
                    last,
                ],
            );
            let casino = e.global::<u32>(CASINO_011DA880);
            let item = e.call(CASINO_ITEM_007316D0, &args![casino]).u32();
            let player = e.global::<u32>(PLAYER_011DEA3C);
            let sound = e
                .call(GET_PICK_UP_SOUND_NAME, &args![player, item, 1u32, 0u32])
                .u32();
            let chars = e.call(WORD_AT_0_00559450, &args![text]).u32();
            show_message(e, chars, LOST_ICON_010208A0, sound);
            e.call(STRING_DESTROY, &args![text]);
        });
    } else if difference > 0 {
        e.with_stack(0xc, |e, container| {
            e.call(CONTAINER_CONSTRUCT, &args![container]);
            let casino = e.global::<u32>(CASINO_011DA880);
            let item = e.call(CASINO_ITEM_007316D0, &args![casino]).u32();
            e.call(
                CONTAINER_ADD,
                &args![container, item, difference as u32, 0u32],
            );
            let casino = e.global::<u32>(CASINO_011DA880);
            let limit = e.call(CASINO_00737E30, &args![casino]).i32();
            let data = e.global::<u32>(CASINO_DATA_011DA87C);
            if (e.mem.u32(data.wrapping_add(4)) as i32) < limit {
                let player = e.global::<u32>(PLAYER_011DEA3C);
                e.call(CONTAINER_GIVE, &args![container, player, 0u32]);
            } else {
                let player = e.global::<u32>(PLAYER_011DEA3C);
                e.call(CONTAINER_GIVE, &args![container, player, 1u32]);
                e.with_stack(8, |e, text| {
                    e.call(STRING_CONSTRUCT, &args![text]);
                    let last = e.call(SETTING_STRING, &args![0x011d_3abcu32]).u32();
                    let fourth = e.call(SETTING_STRING, &args![0x011d_4e7cu32]).u32();
                    let third = e.call(SETTING_STRING, &args![0x011d_3114u32]).u32();
                    let casino = e.global::<u32>(CASINO_011DA880);
                    let item = e.call(CASINO_ITEM_007316D0, &args![casino]).u32();
                    let name = e.call(GET_FULL_NAME, &args![item]).u32();
                    e.call(
                        STRING_FORMAT,
                        &[
                            text.addr(),
                            WON_FORMAT_01070D8C,
                            difference as u32,
                            name,
                            third,
                            fourth,
                            last,
                        ],
                    );
                    let casino = e.global::<u32>(CASINO_011DA880);
                    let item = e.call(CASINO_ITEM_007316D0, &args![casino]).u32();
                    let player = e.global::<u32>(PLAYER_011DEA3C);
                    let sound = e
                        .call(GET_PICK_UP_SOUND_NAME, &args![player, item, 1u32, 0u32])
                        .u32();
                    let chars = e.call(WORD_AT_0_00559450, &args![text]).u32();
                    show_message(e, chars, WON_ICON_0103A830, sound);
                    e.call(STRING_DESTROY, &args![text]);
                });
            }
            e.call(WINNINGS_CONTAINER_DESTRUCT, &args![container]);
        });
    }
    if e.get(this, RouletteMenu::bRunScriptOnExit) {
        let casino = e.global::<u32>(CASINO_011DA880);
        let script = e.call(CASINO_SCRIPT_00504E40, &args![casino]).u32();
        if script != 0 {
            e.call(SCRIPT_RUN_0060C9C0, &args![script, 1u32]);
        }
    }
}

/// `DoIdle` while spinning (state 1); see there.
fn idle_spin(e: &mut Engine, this: Ptr<RouletteMenu>) {
    let base = this.addr();
    let animation = |e: &Engine| {
        let index = e.get(this, RouletteMenu::iSpinAnimIndex) as u32;
        e.mem.u32(base + TABLE_ANIMATIONS + 4 * index)
    };
    if animation(e) == 0 {
        return;
    }
    let zero: f64 = e.global(ZERO_DOUBLE);
    // `FCOMP 0.0; TEST AH,0x44; JP`: started only when exactly 0.0.
    if x87_load32(e.global::<u32>(SPIN_TIME_011DA904)) == zero {
        play_sound(e, SOUND_SPIN_01076310);
        let message = e.mem.u32(base + 0x50);
        e.call(TILE_SET_INT, &args![message, 0xfa3u32, 0u32]);
        let sequence = animation(e);
        let manager = e.get(this, RouletteMenu::pTableAnimManager);
        e.call(
            ACTIVATE_SEQUENCE_0047AAB0,
            &args![
                manager,
                sequence,
                0u32,
                0u32,
                1.0f32.to_bits(),
                0.0f32.to_bits(),
                0u32
            ],
        );
        e.set(this, RouletteMenu::bCleanTable, true);
        let result = e.get(this, RouletteMenu::iResultValue) as u32;
        roulette_menu_shift_wheel_texture(e, this, result);
    }
    e.with_stack(0xc, |e, point| {
        let time = x87_quiet_bits(e.global::<u32>(SPIN_TIME_011DA904));
        let made = e
            .call(POINT_CONSTRUCT_0043D410, &args![point, time, 1u32, 0u32])
            .u32();
        let table = model_child(e, 0);
        e.call(SET_TRANSLATE_00A59C60, &args![table, made]);
    });
    let sequence = animation(e);
    let length = e.call(SEQUENCE_LENGTH_00508100, &args![sequence]).f64();
    let two: f64 = e.global(TWO_DOUBLE);
    let half = x87_div(length, two);
    let time = x87_load32(e.global::<u32>(SPIN_TIME_011DA904));
    if time > half && e.get(this, RouletteMenu::bCleanTable) {
        roulette_menu_reset_bet_chips(e, this);
        e.set(this, RouletteMenu::iNextModelIndex, 3);
        let cursor = model_child(e, 2);
        let translate = e.call(LOCAL_TRANSLATE_0043C490, &args![cursor]).u32();
        let x = e.mem.u32(translate);
        let y = e.mem.u32(translate.wrapping_add(4));
        let next = e.get(this, RouletteMenu::iNextModelIndex) as u32;
        let chip = model_child(e, next);
        set_translate(
            e,
            chip,
            x87_quiet_bits(x),
            x87_quiet_bits(y),
            0.0f32.to_bits(),
        );
        roulette_menu_set_bet_chip(e, this);
        e.set(this, RouletteMenu::bCleanTable, false);
    }
    let sequence = animation(e);
    let length = e.call(SEQUENCE_LENGTH_00508100, &args![sequence]).f64();
    let time = x87_load32(e.global::<u32>(SPIN_TIME_011DA904));
    if time > length {
        e.set(this, RouletteMenu::eRouletteState, 3);
        let sequence = animation(e);
        let manager = e.get(this, RouletteMenu::pTableAnimManager);
        e.call(
            DEACTIVATE_SEQUENCE_0047B220,
            &args![manager, sequence, 0.0f32.to_bits()],
        );
    }
}

/// Whether an outside bet (record 0x93 to 0x9e) holds the result, as `DoIdle`'s switch on
/// the record index tests it.
fn outside_bet_wins(e: &Engine, this: Ptr<RouletteMenu>, index: u32) -> bool {
    let result = e.get(this, RouletteMenu::iResultValue);
    let strip_place = || {
        let mut place = 0i32;
        while place < 0x26 {
            if e.mem.u32(this.addr() + NUMBER_STRIP + 4 * place as u32) as i32 == result {
                break;
            }
            place += 1;
        }
        place
    };
    match index.wrapping_sub(0x93) {
        0 => result < 0xd && result > 0,
        1 => result > 0 && result < 0x13,
        2 => result % 2 == 0,
        3 => result < 0x19 && result > 0xc,
        4 => strip_place() % 2 == 0,
        5 => strip_place() % 2 == 1,
        6 => result < 0x25 && result > 0x18,
        7 => result % 2 == 1,
        8 => result > 0x12 && result < 0x25,
        9 => result.wrapping_sub(1) % 3 == 0,
        10 => result.wrapping_sub(2) % 3 == 0,
        11 => result % 3 == 0,
        _ => false,
    }
}

/// `DoIdle` paying out (state 3); `other` is the unused third stick word the payout test
/// reads. See there.
fn idle_payout(e: &mut Engine, this: Ptr<RouletteMenu>, other: i32) {
    let base = this.addr();
    e.set(this, RouletteMenu::eRouletteState, 4);
    let mut winnings = 0i32;
    tile_alpha(e, 5, CURSOR_FADE_01023CD8);
    tile_alpha(e, 3, CURSOR_FADE_01023CD8);
    tile_alpha(e, 4, HIDDEN_ALPHA_0101E704);
    if e.get(this, RouletteMenu::iCurrentBet) < e.get(this, RouletteMenu::iMinBet) {
        tile_alpha(e, 7, HIDDEN_ALPHA_0101E704);
    } else {
        tile_alpha(e, 7, CURSOR_FADE_01023CD8);
    }
    if e.get(this, RouletteMenu::iCurrentBet) > e.get(this, RouletteMenu::iMaxBet)
        || e.get(this, RouletteMenu::iCurrentBet) == e.get(this, RouletteMenu::iChipCount)
    {
        tile_alpha(e, 7, HIDDEN_ALPHA_0101E704);
    } else {
        tile_alpha(e, 7, CURSOR_FADE_01023CD8);
    }
    tile_alpha(e, 8, CURSOR_FADE_01023CD8);
    loop {
        let list = e.get(this, RouletteMenu::pBetList);
        if list == 0 || node_item(e, list) == 0 {
            break;
        }
        let mut won = false;
        let bet = node_item(e, list);
        let index = e.mem.u32(bet.wrapping_add(4));
        let record = bet_tile(base, index);
        if (index as i32) < 0x93 {
            let mut node = record;
            while node != 0 {
                let slot = e.call(NODE_SELF, &args![node]).u32();
                if e.get(this, RouletteMenu::iResultValue) as u32 == e.mem.u32(slot) {
                    won = true;
                    break;
                }
                node = e.call(WORD_AT_4_00726070, &args![node]).u32();
            }
        } else {
            won = outside_bet_wins(e, this, index);
        }
        let cheat = e.get(this, RouletteMenu::iCheatToggle);
        if (won || other != 0 || cheat > 0) && cheat >= 0 {
            let payout = e.mem.u32(record.wrapping_add(0xc)).wrapping_add(1);
            let list = e.get(this, RouletteMenu::pBetList);
            let bet = node_item(e, list);
            let amount = e.mem.u32(bet);
            winnings = (payout.wrapping_mul(amount) as i32).wrapping_add(winnings);
        }
        let list = e.get(this, RouletteMenu::pBetList);
        e.call(LIST_REMOVE_FIRST_0063F7B0, &args![list]);
    }
    winnings = winnings.wrapping_sub(e.get(this, RouletteMenu::iTotalBet));
    let chips = e.get(this, RouletteMenu::iChipCount).wrapping_add(winnings);
    e.set(this, RouletteMenu::iChipCount, chips);
    let trait_id = e.global::<u32>(TRAIT_011DA86C);
    let tile = e.mem.u32(base + TILES + 4);
    e.call(TILE_POKE, &args![tile, trait_id, 1.0f32.to_bits()]);
    let trait_id = e.global::<u32>(TRAIT_011DA86C);
    let tile = e.mem.u32(base + TILES + 4);
    e.call(TILE_SET_INT, &args![tile, trait_id, chips as u32]);
    let data = e.global::<u32>(CASINO_DATA_011DA87C);
    let won_total = (e.mem.u32(data.wrapping_add(4)) as i32).wrapping_add(winnings);
    e.mem.set_u32(data.wrapping_add(4), won_total as u32);
    fn_007bf9e0(e, this);
    e.with_stack(0x104, |e, buffer| {
        let buffer = buffer.addr();
        if winnings > 0 {
            let last = e.call(SETTING_STRING, &args![0x011d_3114u32]).u32();
            let third = e.call(SETTING_STRING, &args![0x011d_5050u32]).u32();
            let setting = if e.get(this, RouletteMenu::bLucky) {
                0x011d_3a98u32
            } else {
                0x011d_2ff4
            };
            let first = e.call(SETTING_STRING, &args![setting]).u32();
            e.call(
                SPRINTF_S,
                &[
                    buffer,
                    0x104,
                    RESULT_FORMAT_01070D0C,
                    first,
                    winnings as u32,
                    third,
                    last,
                ],
            );
            play_sound(e, SOUND_WIN_01076300);
        } else if winnings < 0 {
            let last = e.call(SETTING_STRING, &args![0x011d_3114u32]).u32();
            let third = e.call(SETTING_STRING, &args![0x011d_5050u32]).u32();
            let first = e.call(SETTING_STRING, &args![0x011d_3474u32]).u32();
            e.call(
                SPRINTF_S,
                &[
                    buffer,
                    0x104,
                    RESULT_FORMAT_01070D0C,
                    first,
                    winnings.wrapping_neg() as u32,
                    third,
                    last,
                ],
            );
        } else {
            let text = e.call(SETTING_STRING, &args![0x011d_3f88u32]).u32();
            e.call(SPRINTF_S, &[buffer, 0x104, ONE_STRING_FORMAT, text]);
        }
        let message = e.mem.u32(roulette_menu(e) + 0x50);
        e.call(TILE_POKE, &args![message, 0xfc4u32, 1.0f32.to_bits()]);
        let message = e.mem.u32(roulette_menu(e) + 0x50);
        e.call(TILE_SET_STRING, &args![message, 0xfc4u32, buffer, 1u32]);
        let message = e.mem.u32(roulette_menu(e) + 0x50);
        e.call(TILE_SET_INT, &args![message, 0xfa3u32, 1u32]);
    });
    e.set(this, RouletteMenu::iNumBetsSet, 0);
    e.set(this, RouletteMenu::iNextModelIndex, 3);
    e.set(this, RouletteMenu::iTotalBet, 0);
    e.set(this, RouletteMenu::iCurrentBet, 0);
    show_number(e, TILES + 0x28, TRAIT_011DA86C, 0);
    if e.get(this, RouletteMenu::iCurrentBet) == 0 {
        let minimum = e.get(this, RouletteMenu::iMinBet);
        e.set(this, RouletteMenu::iCurrentBet, minimum);
        let value = x87_store32(f64::from(minimum));
        let trait_id = e.global::<u32>(TRAIT_011DA86C);
        let tile = e.mem.u32(roulette_menu(e) + TILES);
        e.call(TILE_POKE, &args![tile, trait_id, value]);
        let bet = e.get(this, RouletteMenu::iCurrentBet) as u32;
        let trait_id = e.global::<u32>(TRAIT_011DA86C);
        let tile = e.mem.u32(roulette_menu(e) + TILES);
        e.call(TILE_SET_INT, &args![tile, trait_id, bet]);
    }
    e.set(this, RouletteMenu::eRouletteState, 4);
    let casino = e.global::<u32>(CASINO_011DA880);
    let limits = e.call(CASINO_LIMITS_00504E60, &args![casino]).u32();
    let limit = e.mem.u32(limits.wrapping_add(0x28)) as i32;
    let data = e.global::<u32>(CASINO_DATA_011DA87C);
    let won_total = e.mem.u32(data.wrapping_add(4)) as i32;
    let level = i32::from(e.mem.u16(data.wrapping_add(8)) as i16);
    let step: f64 = e.global(PAYOUT_STEP_010290B0);
    for index in 1..5i32 {
        if index <= level {
            e.set(this, RouletteMenu::bRunScriptOnExit, false);
        } else {
            let threshold = x87_mul(f64::from(limit), x87_mul(f64::from(index), step));
            // `FILD won; FCOMPP; TEST AH,1; JNZ`: reached when not below (ordered).
            if f64::from(won_total) >= threshold {
                let data = e.global::<u32>(CASINO_DATA_011DA87C);
                e.mem.set_u16(data.wrapping_add(8), index as u16);
                e.set(this, RouletteMenu::bRunScriptOnExit, true);
            }
        }
    }
    if e.get(this, RouletteMenu::bRunScriptOnExit) {
        roulette_menu_close(e);
    }
    tile_alpha(e, 6, CURSOR_FADE_01023CD8);
}

/// A cursor limit: below the `double` at `limit` (`TEST AH,5; JP`, ordered) or above it
/// (`TEST AH,0x41; JNZ`, strictly) the coordinate becomes the `float` at `value`.
fn clamp_cursor(e: &mut Engine, at: u32, limit: u32, value: u32, below: bool) {
    let current = x87_load32(e.mem.u32(at));
    let bound: f64 = e.global(limit);
    let outside = if below {
        current < bound
    } else {
        current > bound
    };
    if outside {
        let replacement = x87_quiet_bits(e.global::<u32>(value));
        e.mem.set_u32(at, replacement);
    }
}

/// `DoIdle` while betting (state 4) with the cursor input; see there.
fn idle_betting(e: &mut Engine, this: Ptr<RouletteMenu>, x: u32, y: u32) {
    let base = this.addr();
    let zero: f64 = e.global(ZERO_DOUBLE);
    // `TEST AH,0x44; JP` / `JNP`: the cursor moves unless both inputs are exactly 0.0.
    if x87_load32(x) != zero || x87_load32(y) != zero {
        let speed = x87_quiet_bits(e.global::<u32>(CURSOR_SPEED_01021F44));
        let moved_x = x87_store32(x87_sub(
            x87_m32(e.mem.u32(base + 0x3fd4)),
            x87_mul(x87_load32(x), x87_m32(speed)),
        ));
        e.mem.set_u32(base + 0x3fd4, moved_x);
        let moved_y = x87_store32(x87_sub(
            x87_m32(e.mem.u32(base + 0x3fd8)),
            x87_mul(x87_load32(y), x87_m32(speed)),
        ));
        e.mem.set_u32(base + 0x3fd8, moved_y);
        clamp_cursor(
            e,
            base + 0x3fd4,
            CURSOR_MIN_X_010762F8,
            CURSOR_MIN_X_SET_010762F0,
            true,
        );
        clamp_cursor(
            e,
            base + 0x3fd8,
            CURSOR_MIN_Y_010762E8,
            CURSOR_MIN_Y_SET_010762E0,
            true,
        );
        clamp_cursor(
            e,
            base + 0x3fd4,
            CURSOR_MAX_X_010356D0,
            CURSOR_MAX_X_SET_0101F414,
            false,
        );
        clamp_cursor(
            e,
            base + 0x3fd8,
            CURSOR_MAX_Y_010762D8,
            CURSOR_MAX_Y_SET_010762D0,
            false,
        );
        let fx = x87_quiet_bits(e.mem.u32(base + 0x3fd4));
        let fy = x87_quiet_bits(e.mem.u32(base + 0x3fd8));
        let fz = x87_quiet_bits(e.mem.u32(base + 0x3fdc));
        let cursor = model_child(e, 2);
        set_translate(e, cursor, fx, fy, fz);
        reset_child_translate(e, 2);
        if e.get(this, RouletteMenu::iNumBetsSet) < 10 {
            let next = e.get(this, RouletteMenu::iNextModelIndex) as u32;
            let chip = model_child(e, next);
            let translate = e.call(LOCAL_TRANSLATE_0043C490, &args![chip]).u32();
            let height = x87_quiet_bits(e.mem.u32(translate.wrapping_add(8)));
            let fx = x87_quiet_bits(e.mem.u32(base + 0x3fd4));
            let fy = x87_quiet_bits(e.mem.u32(base + 0x3fd8));
            let next = e.get(this, RouletteMenu::iNextModelIndex) as u32;
            let chip = model_child(e, next);
            set_translate(e, chip, fx, fy, height);
            let fx = x87_quiet_bits(e.mem.u32(base + 0x3fd4));
            let fy = x87_quiet_bits(e.mem.u32(base + 0x3fd8));
            let next = e.get(this, RouletteMenu::iNextModelIndex) as u32;
            let shadow = model_child(e, next.wrapping_add(10));
            set_translate(e, shadow, fx, fy, 0.0f32.to_bits());
            let next = e.get(this, RouletteMenu::iNextModelIndex) as u32;
            reset_child_translate(e, next);
            let next = e.get(this, RouletteMenu::iNextModelIndex) as u32;
            reset_child_translate(e, next.wrapping_add(10));
        }
        let index = roulette_menu_get_bet_index(e, this);
        if (index as i32) < 0x9f {
            let face = e.get(this, RouletteMenu::pCurrentBetFace);
            if face != 0 {
                e.call(SET_APP_CULLED_00450F90, &args![face, 1u32]);
            }
            let record = bet_tile(base, index);
            let grid = e.get(this, RouletteMenu::pRouletteGridGeometry);
            let found = e
                .call(GET_OBJECT_BY_NAME, &args![grid, record + 0x1c])
                .u32();
            e.set(this, RouletteMenu::pCurrentBetFace, found);
            if found == 0 {
                e.with_stack(4, |e, fixed| {
                    let made = e
                        .call(FIXED_STRING_CONSTRUCT, &args![fixed, record + 0x1c])
                        .u32();
                    let grid = e.get(this, RouletteMenu::pRouletteGridGeometry);
                    let found = e.vcall(grid, 0x9c, &args![made]).u32();
                    e.set(this, RouletteMenu::pCurrentBetFace, found);
                    e.call(FIXED_STRING_DESTROY, &args![fixed]);
                });
            }
            let message = e.mem.u32(base + 0x50);
            e.call(
                TILE_SET_STRING,
                &args![message, 0xfc4u32, record + 0x24, 1u32],
            );
            let valid = e.get(this, RouletteMenu::pValidBetFace);
            if e.call(IS_CULLED_00456610, &args![valid]).bool() {
                if e.global::<u8>(INVALID_SOUND_PLAYED_011DA900) != 0 {
                    let flag = u8::from(e.global::<u8>(INVALID_SOUND_PLAYED_011DA900) == 0);
                    e.set_global(INVALID_SOUND_PLAYED_011DA900, flag);
                    play_sound(e, SOUND_CHANGE_0107627C);
                }
                let valid = e.get(this, RouletteMenu::pValidBetFace);
                e.call(SET_APP_CULLED_00450F90, &args![valid, 0u32]);
                let invalid = e.get(this, RouletteMenu::pInvalidBetFace);
                e.call(SET_APP_CULLED_00450F90, &args![invalid, 1u32]);
            }
            let face = e.get(this, RouletteMenu::pCurrentBetFace);
            e.call(SET_APP_CULLED_00450F90, &args![face, 0u32]);
        } else {
            e.with_stack(0x104, |e, buffer| {
                let (first, last) = description_settings(e);
                e.call(
                    SPRINTF_S,
                    &[
                        buffer.addr(),
                        0x104,
                        OUTSIDE_BET_FORMAT_010762C4,
                        first,
                        last,
                    ],
                );
                let message = e.mem.u32(base + 0x50);
                e.call(TILE_SET_STRING, &args![message, 0xfc4u32, buffer, 1u32]);
            });
            let invalid = e.get(this, RouletteMenu::pInvalidBetFace);
            if e.call(IS_CULLED_00456610, &args![invalid]).bool() {
                if e.global::<u8>(INVALID_SOUND_PLAYED_011DA900) == 0 {
                    let flag = u8::from(e.global::<u8>(INVALID_SOUND_PLAYED_011DA900) == 0);
                    e.set_global(INVALID_SOUND_PLAYED_011DA900, flag);
                    play_sound(e, SOUND_INVALID_010762AC);
                }
                let valid = e.get(this, RouletteMenu::pValidBetFace);
                e.call(SET_APP_CULLED_00450F90, &args![valid, 1u32]);
                let invalid = e.get(this, RouletteMenu::pInvalidBetFace);
                e.call(SET_APP_CULLED_00450F90, &args![invalid, 0u32]);
            }
            let face = e.get(this, RouletteMenu::pCurrentBetFace);
            if face != 0 {
                e.call(SET_APP_CULLED_00450F90, &args![face, 1u32]);
            }
        }
    }
    if e.get(this, RouletteMenu::iChipCount)
        < e.get(this, RouletteMenu::iCurrentBet)
            .wrapping_add(e.get(this, RouletteMenu::iTotalBet))
    {
        let bet = e
            .get(this, RouletteMenu::iChipCount)
            .wrapping_sub(e.get(this, RouletteMenu::iTotalBet));
        e.set(this, RouletteMenu::iCurrentBet, bet);
        let trait_id = e.global::<u32>(TRAIT_011DA870);
        let tile = e.mem.u32(roulette_menu(e) + TILES);
        e.call(TILE_POKE, &args![tile, trait_id, 1.0f32.to_bits()]);
        let bet = e.get(this, RouletteMenu::iCurrentBet) as u32;
        let trait_id = e.global::<u32>(TRAIT_011DA870);
        let tile = e.mem.u32(roulette_menu(e) + TILES);
        e.call(TILE_SET_INT, &args![tile, trait_id, bet]);
        tile_alpha(e, 6, HIDDEN_ALPHA_0101E704);
    }
    if e.get(this, RouletteMenu::iChipCount) < e.get(this, RouletteMenu::iMinBet) {
        refuse(e, 0x011d_2424);
        roulette_menu_close(e);
    }
}

// Translated from 007bed70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007bed70 - the C++ exception frame (FS:[0] chain, unwind state 0 around the sound handle) is not translated.
/// `RouletteMenu::SetBet` (Xbox PDB; `RET 4`, a bet tile index): refused (false) with ten
/// bets, an index outside 0 to 0x9e, no current bet or a bet already on that index.
/// Otherwise (a point and the record's position copied into unused locals) a 0xc-byte bet
/// (zeroed with `memset`) holds the current bet, the index and the next model index and is
/// added to the bet list (`00726c60` when the list is missing or empty, else appended);
/// the chip is set, the counts and total advance, the bet sound plays; the next model
/// index wraps from 0xd to 3 and moves on past chips still on the table (`007bf530`) while
/// fewer than ten bets are set. Under ten bets and the maximum total the next chip and
/// its shadow go to the cursor (model child 2) and the chip is set. A bet past the chips
/// is cut to them (the bet tile poked and shown, tile 6 hidden). Returns true.
pub fn roulette_menu_set_bet(e: &mut Engine, this: Ptr<RouletteMenu>, index: u32) -> bool {
    let index_signed = index as i32;
    if e.get(this, RouletteMenu::iNumBetsSet) >= 10
        || !(0..0x9f).contains(&index_signed)
        || e.get(this, RouletteMenu::iCurrentBet) == 0
    {
        return false;
    }
    let mut node = e.get(this, RouletteMenu::pBetList);
    while node != 0 && node_item(e, node) != 0 {
        let bet = node_item(e, node);
        if e.mem.u32(bet.wrapping_add(4)) == index {
            return false;
        }
        node = e.call(WORD_AT_4_00726070, &args![node]).u32();
    }
    e.with_stack(0x1c, |e, frame| {
        e.call(
            POINT_CONSTRUCT_0043D410,
            &args![frame, 0.0f32.to_bits(), 0u32, 0u32],
        );
        let position = bet_x(this.addr(), index);
        for word in 0..3u32 {
            let value = e.mem.u32(position + 4 * word);
            e.mem.set_u32(frame.addr() + 0xc + 4 * word, value);
        }
        let bet_slot = frame.addr() + 0x18;
        let block = e.call(ALLOCATE, &args![0xcu32]).u32();
        let bet = if block != 0 {
            e.call(MEMSET, &args![block, 0u32, 0xcu32]);
            block
        } else {
            0
        };
        e.mem.set_u32(bet_slot, bet);
        let current = e.get(this, RouletteMenu::iCurrentBet) as u32;
        e.mem.set_u32(bet, current);
        e.mem.set_u32(bet.wrapping_add(4), index);
        let next = e.get(this, RouletteMenu::iNextModelIndex) as u32;
        e.mem.set_u32(bet.wrapping_add(8), next);
        let list = e.get(this, RouletteMenu::pBetList);
        if list == 0 || e.call(LIST_IS_EMPTY, &args![list]).bool() {
            let list = e.get(this, RouletteMenu::pBetList);
            e.call(LIST_SET_FIRST_00726C60, &args![list, bet_slot]);
        } else {
            let list = e.get(this, RouletteMenu::pBetList);
            e.call(LIST_APPEND, &args![list, bet_slot]);
        }
    });
    roulette_menu_set_bet_chip(e, this);
    let count = e.get(this, RouletteMenu::iNumBetsSet).wrapping_add(1);
    e.set(this, RouletteMenu::iNumBetsSet, count);
    let next = e.get(this, RouletteMenu::iNextModelIndex).wrapping_add(1);
    e.set(this, RouletteMenu::iNextModelIndex, next);
    let total = e
        .get(this, RouletteMenu::iTotalBet)
        .wrapping_add(e.get(this, RouletteMenu::iCurrentBet));
    e.set(this, RouletteMenu::iTotalBet, total);
    play_sound(e, SOUND_BET_01076324);
    if e.get(this, RouletteMenu::iNextModelIndex) >= 0xd {
        e.set(this, RouletteMenu::iNextModelIndex, 3);
    }
    loop {
        let next = e.get(this, RouletteMenu::iNextModelIndex) as u32;
        if fn_007bf530(e, this, next) || e.get(this, RouletteMenu::iNumBetsSet) >= 10 {
            break;
        }
        let next = e.get(this, RouletteMenu::iNextModelIndex).wrapping_add(1);
        e.set(this, RouletteMenu::iNextModelIndex, next);
        if e.get(this, RouletteMenu::iNextModelIndex) >= 0xd {
            e.set(this, RouletteMenu::iNextModelIndex, 3);
        }
    }
    if e.get(this, RouletteMenu::iNumBetsSet) < 10
        && e.get(this, RouletteMenu::iTotalBet) < e.get(this, RouletteMenu::iMaxBet)
    {
        let cursor = model_child(e, 2);
        let translate = e.call(LOCAL_TRANSLATE_0043C490, &args![cursor]).u32();
        let x = e.mem.u32(translate);
        let y = e.mem.u32(translate.wrapping_add(4));
        for offset in [0u32, 10] {
            let next = e.get(this, RouletteMenu::iNextModelIndex) as u32;
            let node = model_child(e, next.wrapping_add(offset));
            set_translate(
                e,
                node,
                x87_quiet_bits(x),
                x87_quiet_bits(y),
                0.0f32.to_bits(),
            );
        }
        roulette_menu_set_bet_chip(e, this);
    }
    if e.get(this, RouletteMenu::iTotalBet)
        .wrapping_add(e.get(this, RouletteMenu::iCurrentBet))
        > e.get(this, RouletteMenu::iChipCount)
    {
        let bet = e
            .get(this, RouletteMenu::iChipCount)
            .wrapping_sub(e.get(this, RouletteMenu::iTotalBet));
        e.set(this, RouletteMenu::iCurrentBet, bet);
        show_number(e, TILES, TRAIT_011DA86C, bet as u32);
        tile_alpha(e, 6, HIDDEN_ALPHA_0101E704);
    }
    true
}

// Translated from 007bf150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RouletteMenu::RemoveBet` (Xbox PDB; `RET 4`, a bet tile index): false outside 0 to 0x9e
/// or without a bet on that index. Otherwise the bet's amount leaves the total, its chip
/// goes back (`007bf480`); with ten bets set its model becomes the next one, the chip is
/// set and the chip and its shadow go to the cursor position (`fxTranslate`,
/// `fyTranslate`; the chip at its own height, the shadow at 0.0) and the origin
/// translation; the count drops and the first list item is removed (`0063f7b0` on the
/// node) and freed. Returns true.
pub fn roulette_menu_remove_bet(e: &mut Engine, this: Ptr<RouletteMenu>, index: u32) -> bool {
    let base = this.addr();
    let index_signed = index as i32;
    if !(0..0x9f).contains(&index_signed) {
        return false;
    }
    let mut node = e.get(this, RouletteMenu::pBetList);
    while node != 0 && node_item(e, node) != 0 {
        let bet = node_item(e, node);
        if e.mem.u32(bet.wrapping_add(4)) == index {
            let bet = node_item(e, node);
            let total = e
                .get(this, RouletteMenu::iTotalBet)
                .wrapping_sub(e.mem.u32(bet) as i32);
            e.set(this, RouletteMenu::iTotalBet, total);
            let bet = node_item(e, node);
            let model_index = e.mem.u32(bet.wrapping_add(8));
            fn_007bf480(e, this, model_index);
            if e.get(this, RouletteMenu::iNumBetsSet) == 10 {
                let bet = node_item(e, node);
                let model_index = e.mem.u32(bet.wrapping_add(8)) as i32;
                e.set(this, RouletteMenu::iNextModelIndex, model_index);
                roulette_menu_set_bet_chip(e, this);
                let next = e.get(this, RouletteMenu::iNextModelIndex) as u32;
                let chip = model_child(e, next);
                let translate = e.call(LOCAL_TRANSLATE_0043C490, &args![chip]).u32();
                let height = x87_quiet_bits(e.mem.u32(translate.wrapping_add(8)));
                let fx = x87_quiet_bits(e.mem.u32(base + 0x3fd4));
                let fy = x87_quiet_bits(e.mem.u32(base + 0x3fd8));
                let next = e.get(this, RouletteMenu::iNextModelIndex) as u32;
                let chip = model_child(e, next);
                set_translate(e, chip, fx, fy, height);
                let fx = x87_quiet_bits(e.mem.u32(base + 0x3fd4));
                let fy = x87_quiet_bits(e.mem.u32(base + 0x3fd8));
                let next = e.get(this, RouletteMenu::iNextModelIndex) as u32;
                let shadow = model_child(e, next.wrapping_add(10));
                set_translate(e, shadow, fx, fy, 0.0f32.to_bits());
                let next = e.get(this, RouletteMenu::iNextModelIndex) as u32;
                reset_child_translate(e, next);
                let next = e.get(this, RouletteMenu::iNextModelIndex) as u32;
                reset_child_translate(e, next.wrapping_add(10));
            }
            let count = e.get(this, RouletteMenu::iNumBetsSet).wrapping_sub(1);
            e.set(this, RouletteMenu::iNumBetsSet, count);
            let item = node_item(e, node);
            e.call(LIST_REMOVE_FIRST_0063F7B0, &args![node]);
            e.call(FREE, &args![item]);
            return true;
        }
        node = e.call(WORD_AT_4_00726070, &args![node]).u32();
    }
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007b_9000, fn_007b9000(Ptr)),
        entry!(
            0x007b_8f30,
            roulette_menu_get_class(Ptr<RouletteMenu>) -> u32
        ),
        entry!(
            0x007b_8f40,
            roulette_menu_attach_tile_by_id(Ptr<RouletteMenu>, i32, u32)
        ),
        entry!(
            0x007b_8f70,
            roulette_menu_scalar_deleting_destructor(Ptr<RouletteMenu>, u32) -> Ptr<RouletteMenu>
        ),
        entry!(0x007b_8fa0, fn_007b8fa0(Ptr) -> Ptr),
        entry!(0x007b_9020, fn_007b9020(Ptr<RouletteMenu>)),
        entry!(0x007b_9480, roulette_menu_close()),
        entry!(
            0x007b_94c0,
            roulette_menu_prepare_3d_elements(Ptr<RouletteMenu>)
        ),
        entry!(
            0x007b_9c40,
            roulette_menu_prepare_bet_tiles(Ptr<RouletteMenu>)
        ),
        entry!(0x007b_bcf0, fn_007bbcf0()),
        entry!(0x007b_be20, roulette_menu_create(u32, i32, i32, i32) -> u32),
        entry!(0x007b_c820, fn_007bc820() -> u32),
        entry!(0x007b_c830, fn_007bc830(Ptr<RouletteMenu>) -> bool),
        entry!(
            0x007b_c870,
            roulette_menu_do_click(Ptr<RouletteMenu>, u32, u32)
        ),
        entry!(0x007b_d2a0, roulette_menu_do_idle(Ptr<RouletteMenu>)),
        entry!(
            0x007b_ed00,
            roulette_menu_do_enter(Ptr<RouletteMenu>, u32, u32)
        ),
        entry!(
            0x007b_ed30,
            roulette_menu_do_leave(Ptr<RouletteMenu>, u32, u32)
        ),
        entry!(
            0x007b_ed70,
            roulette_menu_set_bet(Ptr<RouletteMenu>, u32) -> bool
        ),
        entry!(
            0x007b_f150,
            roulette_menu_remove_bet(Ptr<RouletteMenu>, u32) -> bool
        ),
        entry!(
            0x007b_f390,
            roulette_menu_get_bet_index(Ptr<RouletteMenu>) -> u32
        ),
        entry!(0x007b_f480, fn_007bf480(Ptr<RouletteMenu>, u32)),
        entry!(0x007b_f530, fn_007bf530(Ptr<RouletteMenu>, u32) -> bool),
        entry!(
            0x007b_f570,
            roulette_menu_shift_wheel_texture(Ptr<RouletteMenu>, u32)
        ),
        entry!(0x007b_f670, fn_007bf670(Ptr<RouletteMenu>)),
        entry!(
            0x007b_f710,
            roulette_menu_set_tile_text_alpha(Ptr<RouletteMenu>, u32, u32)
        ),
        entry!(
            0x007b_f7a0,
            roulette_menu_reset_bet_chips(Ptr<RouletteMenu>)
        ),
        entry!(0x007b_f840, roulette_menu_set_bet_chip(Ptr<RouletteMenu>)),
        entry!(0x007b_f9e0, fn_007bf9e0(Ptr<RouletteMenu>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_007b9000_forwards_this_to_the_container_destructor() {
        let mut e = Engine::new();
        e.register(CONTAINER_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x007b_9000, &args![0x0e00_5000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![(CONTAINER_DESTRUCT, vec![0x0e00_5000])]
        );
    }
}

#[cfg(test)]
mod n058_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, u32)>>>;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// The target of slot `slot` of fake vtable `index` (each slot its own double).
    fn slot(index: u32, slot: u32) -> u32 {
        0x0e00_0000 + index * 0x1000 + slot
    }

    fn fake_vtable(e: &mut Engine, index: u32) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for offset in (0..0x200).step_by(4) {
            let target = slot(index, offset);
            e.mem.set_u32(vtable + offset, target);
            e.register(target, |_, _| Ret::default());
        }
        vtable
    }

    fn fake_object(e: &mut Engine, index: u32) -> u32 {
        let vtable = fake_vtable(e, index);
        let object = e.mem.alloc(0x300);
        e.mem.set_u32(object, vtable);
        object
    }

    fn c_string(e: &mut Engine, at: u32, text: &str) {
        for (index, byte) in text.bytes().enumerate() {
            e.mem.set_u8(at + index as u32, byte);
        }
        e.mem.set_u8(at + text.len() as u32, 0);
    }

    fn read_string(e: &Engine, at: u32) -> String {
        let mut text = String::new();
        let mut index = 0;
        while e.mem.u8(at + index) != 0 {
            text.push(e.mem.u8(at + index) as char);
            index += 1;
        }
        text
    }

    /// Every callee outside this file, doubled (0 or nothing), the statics and constants
    /// mapped, and the list, string and model helpers acting on memory.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011d_a000,
            0x011a_0000,
            0x011a_9000,
            0x011d_e000,
            0x011c_3000,
            0x011f_3000,
            0x011f_4000,
            0x011f_6000,
            0x0101_1000,
            0x0101_2000,
            0x0101_6000,
            0x0101_7000,
            0x0101_d000,
            0x0101_e000,
            0x0101_f000,
            0x0102_0000,
            0x0102_1000,
            0x0102_3000,
            0x0102_9000,
            0x0102_c000,
            0x0103_0000,
            0x0103_5000,
            0x0104_e000,
            0x0106_b000,
            0x0107_0000,
            0x0107_1000,
            0x0107_5000,
            0x0107_6000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(TWO_DOUBLE, 2.0f64);
        e.set_global(THOUSAND_DOUBLE, 1000.0f64);
        e.set_global(POSITIVE_SCALE_0106B168, 0.49f64);
        e.set_global(NEGATIVE_SCALE_010760F0, -0.49f64);
        e.set_global(STICK_DEAD_ZONE_010718F0, 8689.0f64);
        e.set_global(STICK_SCALE_01029790, 10000.0f64);
        e.set_global(CURSOR_SPEED_01021F44, 2.0f32);
        for (limit, value) in [
            (CURSOR_MIN_X_010762F8, -100.0f64),
            (CURSOR_MIN_Y_010762E8, -100.0),
            (CURSOR_MAX_X_010356D0, 100.0),
            (CURSOR_MAX_Y_010762D8, 100.0),
        ] {
            e.set_global(limit, value);
        }
        e.set_global(CURSOR_FADE_01023CD8, 0.5f32);
        e.set_global(HIDDEN_ALPHA_0101E704, 0.25f32);
        e.set_global(PAYOUT_STEP_010290B0, 0.5f64);
        c_string(&mut e, SHADOW_SUFFIX, "_Shadow.NIF");
        for addr in [
            FREE,
            PLAYER_00969B20,
            MODEL_LOADER_LOAD_FILE,
            MODEL_LOADER_0045A5E0,
            SPRINTF_S,
            STRCPY_S,
            STRLEN,
            COPY_00401460,
            NI_POINTER_DESTROY,
            SCENE_00450B80,
            SCENE_00B5D180,
            NODE_00572160,
            LIST_IS_EMPTY,
            LIST_FREE_NODES,
            LIST_DELETE,
            LIST_APPEND,
            LIST_CONSTRUCT,
            VECTOR_DESTRUCT,
            MENU_DESTRUCT,
            INTERFACE_MANAGER,
            WORD_AT_28_0045CD60,
            TILE_SET_FLOAT,
            TILE_SET_STRING,
            SET_TRANSLATE_004BC1F0,
            GET_OBJECT_BY_NAME,
            SET_APP_CULLED_00450F90,
            NI_OBJECT_CLONE,
            NI_ALLOCATE,
            NI_NODE_CONSTRUCT,
            POINT_CONSTRUCT_0043D410,
            SET_TRANSLATE_00A59C60,
            GET_CONTROLLER,
            GET_WORLD_BOUND,
            BOUND_RADIUS_0084D030,
            ROTATION_004A0C90,
            SET_ROTATE_0043FA80,
            ADD_ALL_LIGHTS_FROM_NODE,
            FIXED_STRING_CONSTRUCT,
            FIXED_STRING_DESTROY,
            SEQUENCE_BY_NAME_0047A520,
            DEACTIVATE_ALL,
            START_ANIMATIONS,
            POINT_SUBTRACT_00439EF0,
            WORLD_TRANSLATE_0045BB80,
            POINT_LENGTH_00457990,
            RANDOM_00944460,
            LIST_HEAD_SLOT_0073AC90,
            NODE_SLOT_006317A0,
            NEXT_NODE_007B52D0,
            NAME_CHARS_00408DA0,
            MAKE_CAMERA_007C2180,
            AUDIO_INSTANCE,
            GET_SOUND_HANDLE_BY_NAME,
            SOUND_HANDLE_PLAY,
            SOUND_HANDLE_DESTROY,
            KEY_0084E3A0,
            LIST_REMOVE_FIRST_0063F7B0,
            IDLE_STATIC_CONSTRUCT_0041A250,
            ATEXIT,
            WORD_AT_14_00825C00,
            INTERFACE_IS_TOP_MENU_ID,
            FLAG_004B71D0,
            INPUT_00877720,
            STICK_00A23390,
            MOUSE_00A239E0,
            ABS_00408820,
            SCREEN_WIDTH,
            SCREEN_00706E50,
            MENU_FADE_STATE_0059BB30,
            MENU_START_FADE_OUT,
            TILE_SET_INT,
            TILE_POKE,
            CASINO_ITEM_007316D0,
            CASINO_LIMITS_00504E60,
            CASINO_SCRIPT_00504E40,
            CASINO_00737E30,
            GET_ITEM_COUNT_IN_CONTAINER,
            GET_PICK_UP_SOUND_NAME,
            GET_FULL_NAME,
            SHOW_MESSAGE_007052F0,
            SCRIPT_RUN_0060C9C0,
            CONTAINER_CONSTRUCT,
            CONTAINER_ADD,
            CONTAINER_GIVE,
            WINNINGS_CONTAINER_DESTRUCT,
            STRING_CONSTRUCT,
            STRING_DESTROY,
            STRING_FORMAT,
            ACTIVATE_SEQUENCE_0047AAB0,
            DEACTIVATE_SEQUENCE_0047B220,
            SEQUENCE_LENGTH_00508100,
            IS_CULLED_00456610,
            MISC_STAT_INCREMENT,
            LIST_COUNT_005AE380,
            MEMSET,
            LIST_SET_FIRST_00726C60,
            TILE_GET_MENU_BY_CLASS,
            CASINO_DATA_LIST_005A6260,
            CASINO_DATA_CONSTRUCT,
            PLAYER_00969B80,
            PLAYER_00969B40,
            WORD_AT_9C_00586150,
            TILE_READ_FILE,
            TILE_GET_MENU,
            MENU_SET_MENU_TILE,
            DEBUG_MESSAGE,
            TILE_GET_MAXIMUM_DEPTH,
            TILE_GET_FLOAT,
            TILE_CHILD_BY_NAME_00A08B20,
            TILE_ADD_NEEDS_UPDATE,
            MENU_PREP_FOR_VISIBILITY,
            GET_CLAMPED_ACTOR_VALUE,
            RENDER_0043C4B0,
            RENDER_007148C0,
            RENDER_0045C670,
            RENDER_008D80E0,
            RENDER_00C4F270,
            RENDER_00709D50,
            RENDER_007FD640,
            NI_POINTER_FROM_00633C90,
            RENDER_004A0FD0,
            RENDER_006629F0,
            ACCUMULATE_SCENE,
            RENDER_00B6C0D0,
            RENDER_00C4F2D0,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NODE_SELF, |_, a| a[0].into_ret());
        e.register(WORD_AT_4_00726070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(WORD_AT_0_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(LOCAL_TRANSLATE_0043C490, |_, a| (a[0] + 0x40).into_ret());
        e.register(NODE_CHILD_0043B4A0, |e, a| {
            e.mem.u32(a[0] + 0x80 + 4 * a[1]).into_ret()
        });
        e.register(SETTING_STRING, |_, a| a[0].into_ret());
        // The record names are written; the chip text is "abc".
        e.register(SPRINTF_S, |e, a| {
            if a[2] == BET_TILE_NAME_FORMAT {
                for (index, byte) in [a[3], a[4], a[5], 0x3a, 0x30, 0].into_iter().enumerate() {
                    e.mem.set_u8(a[0] + index as u32, byte as u8);
                }
            } else if a[2] == CHIPS_FORMAT_01070E80 {
                for (index, byte) in b"abc\0".iter().enumerate() {
                    e.mem.set_u8(a[0] + index as u32, *byte);
                }
            }
            Ret::default()
        });
        e.register(STRCPY_S, |e, a| {
            let mut index = 0;
            loop {
                let byte = e.mem.u8(a[2] + index);
                e.mem.set_u8(a[0] + index, byte);
                if byte == 0 {
                    break;
                }
                index += 1;
            }
            Ret::default()
        });
        e.register(STRLEN, |e, a| {
            let mut length = 0u32;
            while e.mem.u8(a[0] + length) != 0 {
                length += 1;
            }
            length.into_ret()
        });
        e
    }

    /// An open roulette menu (fake vtable 1, tiles 0x7000 + 4 * index), the model node
    /// (fake object 2) with 23 children, the casino (3) and its data, the player (4), the
    /// scene (5) and the geometry from +0x3f14 (the model's children).
    fn world(e: &mut Engine) -> u32 {
        let menu = e.mem.alloc(0x4100);
        let vtable = fake_vtable(e, 1);
        e.mem.set_u32(menu, vtable);
        for index in 0..0xbu32 {
            let tile = e.mem.alloc(0x10);
            e.mem.set_u32(menu + TILES + 4 * index, tile);
        }
        e.set_global(ROULETTE_MENU_011DA878, menu);
        let model = fake_object(e, 2);
        for index in 0..23u32 {
            let child = e.mem.alloc(0x100);
            e.mem.set_u32(model + 0x80 + 4 * index, child);
            e.mem.set_u32(menu + 0x3f14 + 4 * index, child);
        }
        e.set_global(MODEL_011DA8A0, model);
        let casino = fake_object(e, 3);
        e.set_global(CASINO_011DA880, casino);
        let data = e.mem.alloc(0x10);
        e.set_global(CASINO_DATA_011DA87C, data);
        let player = fake_object(e, 4);
        e.set_global(PLAYER_011DEA3C, player);
        let scene = fake_object(e, 5);
        e.register_double(SCENE_00450B80, move |_, _| scene.into_ret());
        menu
    }

    fn get(e: &Engine, menu: u32, field: Field<RouletteMenu, i32>) -> i32 {
        e.get(Ptr::<RouletteMenu>::new(menu), field)
    }

    fn set(e: &mut Engine, menu: u32, field: Field<RouletteMenu, i32>, value: i32) {
        e.set(Ptr::<RouletteMenu>::new(menu), field, value);
    }

    /// The casino's models: vtable +0x14 of `00504e00`'s object gives `"bet.nif"`.
    fn casino_models(e: &mut Engine) -> u32 {
        let path = e.mem.alloc(0x10);
        c_string(e, path, "bet.nif");
        let model = fake_object(e, 6);
        e.register_double(slot(6, 0x14), move |_, _| path.into_ret());
        e.register_double(CASINO_MODEL_00504E00, move |_, _| model.into_ret());
        path
    }

    /// The bet tile doubles: the record names written by the name format, a new node for
    /// every name looked up (node `n` at (n, 2n, 0)), its world bound the node, the numbers
    /// appended logged, the copy done, the distance 10.0.
    fn bet_tile_doubles(e: &mut Engine) -> Log {
        let nodes = e.mem.alloc(0xb0 * 0x10);
        for index in 0..0xb0u32 {
            e.mem
                .set_u32(nodes + 0x10 * index, (index as f32).to_bits());
            e.mem
                .set_u32(nodes + 0x10 * index + 4, (2.0 * index as f32).to_bits());
        }
        let mut next = 0u32;
        e.register_double(GET_OBJECT_BY_NAME, move |_, _| {
            next += 1;
            (nodes + 0x10 * (next - 1)).into_ret()
        });
        e.register(GET_WORLD_BOUND, |_, a| a[0].into_ret());
        let log: Log = Rc::new(RefCell::new(vec![]));
        let appended = log.clone();
        e.register_double(LIST_APPEND, move |e, a| {
            appended.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.register(COPY_00401460, |e, a| {
            for index in 0..a[2] {
                let byte = e.mem.u8(a[1] + index);
                e.mem.set_u8(a[0] + index, byte);
            }
            Ret::default()
        });
        e.register(POINT_LENGTH_00457990, |_, _| st0(10.0));
        log
    }

    fn numbers(log: &Log, record: u32) -> Vec<u32> {
        log.borrow()
            .iter()
            .filter(|(list, _)| *list == record)
            .map(|(_, number)| *number)
            .collect()
    }

    #[test]
    fn the_class_and_the_tiles() {
        let mut e = engine();
        let menu = world(&mut e);
        assert_eq!(e.call(0x007b_8f30, &args![menu]).u32(), 0x43a);
        assert_eq!(e.call(0x007b_c820, &args![]).u32(), 0x43a);
        assert!(e.call(0x007b_c830, &args![menu]).bool());
        e.call(0x007b_8f40, &args![menu, 0xau32, 0x5555u32]);
        assert_eq!(e.mem.u32(menu + 0x54), 0x5555);
        e.call(0x007b_8f40, &args![menu, 0xbu32, 0x6666u32]);
        e.call(0x007b_8f40, &args![menu, 0xffff_ffffu32, 0x6666u32]);
        assert_eq!(e.mem.u32(menu + 0x58), 0);
        assert_eq!(e.mem.u32(menu + 0x28), 0);
        e.call(0x007b_8f40, &args![menu, 3u32, 0u32]);
        assert!(!e.call(0x007b_c830, &args![menu]).bool());
    }

    #[test]
    fn the_bet_tile_record_constructor() {
        let mut e = engine();
        let record = e.mem.alloc(0x64);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007b_8fa0, &args![record]).u32(), record);
        assert_eq!(calls_to(&e, LIST_CONSTRUCT), vec![vec![record]]);
        assert_eq!(calls_to(&e, NODE_SELF), vec![vec![record + 0x10]]);
    }

    #[test]
    fn the_destructor_releases_everything() {
        let mut e = engine();
        let menu = world(&mut e);
        let path = casino_models(&mut e);
        e.set_global(MODEL_LOADER_011C3B3C, 0x00ab_0000u32);
        e.mem.set_u32(menu + 0x3e78, 0x00ab_1000);
        let releases: Log = Rc::new(RefCell::new(vec![]));
        let released = releases.clone();
        e.register_double(MODEL_LOADER_0045A5E0, move |e, a| {
            released
                .borrow_mut()
                .push((a[1], u32::from(e.mem.u8(a[1] + 3))));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x007b_8f70, &args![menu, 0u32]);
        assert_eq!(e.mem.u32(menu), ROULETTE_MENU_VTABLE);
        let released = releases.borrow().clone();
        assert_eq!(
            released
                .iter()
                .map(|(at, _)| *at)
                .take(4)
                .collect::<Vec<_>>(),
            vec![path, CURSOR_MODEL_PATH, GRID_MODEL_PATH, path]
        );
        assert_eq!(released.len(), 5);
        // The fifth is the local buffer holding "bet_Shadow.NIF" (its fourth byte '_').
        assert_eq!(released[4].1, u32::from(b'_'));
        assert_eq!(calls_to(&e, LIST_FREE_NODES).len(), 2);
        assert_eq!(calls_to(&e, LIST_DELETE), vec![vec![0x00ab_1000, 1]]);
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCT),
            vec![vec![menu + 0x5c, 0x64, 0x9f, 0x007b_9000]]
        );
        assert_eq!(calls_to(&e, MENU_DESTRUCT), vec![vec![menu]]);
        assert!(calls_to(&e, FREE).is_empty());
        assert_eq!(e.global::<u32>(ROULETTE_MENU_011DA878), 0);
        assert_eq!(e.global::<u32>(CASINO_011DA880), 0);
        assert_eq!(e.mem.u32(menu + 0x3e78), 0);
        e.call_log = Some(vec![]);
        e.call(0x007b_8f70, &args![menu, 1u32]);
        assert_eq!(calls_to(&e, FREE), vec![vec![menu]]);
    }

    #[test]
    fn close_sets_the_cursor_and_the_state() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x007b_9480, &args![]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        let menu = world(&mut e);
        e.call(0x007b_9480, &args![]);
        assert_eq!(get(&e, menu, RouletteMenu::eRouletteState), 5);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0, 0xfa9, 0.5f32.to_bits(), 1]]
        );
        e.call_log = Some(vec![]);
        e.call(0x007b_ed00, &args![menu, 0u32, 0u32]);
        e.call(0x007b_ed30, &args![menu, 0u32, 0u32]);
        e.set_global(CURSOR_FADE_0104EF80, 0.75f32);
        e.call(0x007b_ed30, &args![menu, 0u32, 0u32]);
        let fades: Vec<u32> = calls_to(&e, TILE_SET_FLOAT).iter().map(|w| w[2]).collect();
        assert_eq!(fades, vec![0, 0, 0.75f32.to_bits()]);
    }

    #[test]
    fn prepare_bet_tiles_fills_the_records() {
        let mut e = engine();
        let menu = world(&mut e);
        let grid = fake_object(&mut e, 7);
        e.mem.set_u32(menu + 0x3f18, grid);
        let log = bet_tile_doubles(&mut e);
        e.mem.set_u8(menu + 0x400f, 1);
        e.call_log = Some(vec![]);
        e.call(0x007b_9c40, &args![menu]);
        let record = |index: u32| menu + 0x5c + 0x64 * index;
        assert_eq!(numbers(&log, record(0)), vec![0, 0x25, 1, 2, 3]);
        assert_eq!(e.mem.u32(record(0) + 0xc), 6);
        assert_eq!(read_string(&e, record(0) + 0x1c), "A01:0");
        assert_eq!(numbers(&log, record(1)), vec![1, 2, 3]);
        assert_eq!(e.mem.u32(record(1) + 0xc), 0xb);
        assert_eq!(numbers(&log, record(2)), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(e.mem.u32(record(2) + 0xc), 5);
        // Row B: record 24 is the zero split with 1, record 25 the straight 1 (black).
        assert_eq!(read_string(&e, record(24) + 0x1c), "B01:0");
        assert_eq!(numbers(&log, record(24)), vec![0, 1]);
        assert_eq!(numbers(&log, record(25)), vec![1]);
        assert_eq!(e.mem.u32(record(25) + 8), 2);
        assert_eq!(e.mem.u32(record(25) + 0xc), 0x23);
        let descriptions = calls_to(&e, SPRINTF_S);
        assert!(descriptions.contains(&vec![
            record(0) + 0x24,
            0x40,
            0x0107_612c,
            SETTING_011D4DBC,
            SETTING_011D3F4C
        ]));
        assert!(descriptions.contains(&vec![
            record(24) + 0x24,
            0x40,
            0x0107_617c,
            0x0102_0760,
            SETTING_011D4DBC,
            1,
            SETTING_011D3F4C
        ]));
        // Positions from the nodes, the radii and the outside bets.
        assert_eq!(e.mem.u32(record(5) + 0x10), 5.0f32.to_bits());
        assert_eq!(e.mem.u32(menu + 0x3f90), 10.0f32.to_bits());
        assert_eq!(
            e.mem.u32(menu + 0x3f94),
            ((10.0f64 * 0.49) as f32).to_bits()
        );
        let special = ((-2.0f64) * -0.49) as f32;
        for offset in [0x3f98u32, 0x3f9c, 0x3fa0] {
            assert_eq!(e.mem.u32(menu + offset), special.to_bits());
        }
        assert_eq!(numbers(&log, record(0x91)), vec![0x25, 0]);
        assert_eq!(e.mem.u32(record(0x94) + 0xc), 1);
        assert_eq!(e.mem.u32(record(0x9e) + 0xc), 2);
        assert_eq!(e.mem.u32(menu + 0x3e7c), 0xe);
        assert_eq!(e.mem.u32(menu + 0x3e7c + 4 * 37), 0x23);
        assert_eq!(e.mem.u8(menu + 0x400f), 0);
    }

    /// The doubles `Prepare3DElements` (and so `Create`) needs beyond the bet tiles.
    fn three_d_doubles(e: &mut Engine) -> Log {
        casino_models(e);
        let mut made = 0u32;
        let nodes = e.mem.alloc(0x40 * 0x200);
        e.register_double(MODEL_LOADER_LOAD_FILE, move |e, _| {
            made += 1;
            let node = nodes + 0x200 * made;
            let vtable = e.mem.u32(e.global::<u32>(MODEL_011DA8A0));
            e.mem.set_u32(node, vtable);
            node.into_ret()
        });
        e.register(NI_OBJECT_CLONE, |_, a| (a[0] + 0x100).into_ret());
        let node = e.global::<u32>(MODEL_011DA8A0);
        e.register_double(NI_NODE_CONSTRUCT, move |_, _| node.into_ret());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(GET_CONTROLLER, |_, _| 0x00ab_5000u32.into_ret());
        e.register(BOUND_RADIUS_0084D030, |_, _| st0(2.0));
        e.set_global(LIGHT_SCALE_0101DB80, 1.5f64);
        bet_tile_doubles(e)
    }

    #[test]
    fn prepare_3d_elements_loads_and_attaches_the_models() {
        let mut e = engine();
        let menu = world(&mut e);
        three_d_doubles(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x007b_94c0, &args![menu]);
        let paths: Vec<u32> = calls_to(&e, MODEL_LOADER_LOAD_FILE)
            .iter()
            .map(|w| w[1])
            .collect();
        assert_eq!(paths.len(), 5);
        assert_eq!(paths[1..3], [GRID_MODEL_PATH, CURSOR_MODEL_PATH]);
        assert_eq!(calls_to(&e, NI_OBJECT_CLONE).len(), 18);
        assert_eq!(calls_to(&e, NI_ALLOCATE), vec![vec![0xac]]);
        assert_eq!(
            calls_to(&e, SEQUENCE_BY_NAME_0047A520)
                .iter()
                .map(|w| w[0])
                .collect::<Vec<_>>(),
            vec![0x00ab_5000; 3]
        );
        assert_eq!(calls_to(&e, DEACTIVATE_ALL), vec![vec![0x00ab_5000, 0]]);
        // The camera node was found on the table: no camera is made.
        assert!(calls_to(&e, MAKE_CAMERA_007C2180).is_empty());
        let model = e.global::<u32>(MODEL_011DA8A0);
        let lights = calls_to(&e, ADD_ALL_LIGHTS_FROM_NODE);
        assert_eq!(lights[0][1..], [model, 3.0f32.to_bits()]);
        assert_eq!(get(&e, menu, RouletteMenu::iNextModelIndex), 3);
        // 23 children attached to the model node, then the model to the scene.
        assert_eq!(calls_to(&e, slot(2, 0xdc)).len(), 23);
        let attached = calls_to(&e, slot(5, 0xdc));
        assert_eq!(attached.len(), 1);
        assert_eq!(attached[0][1..], [model, 1]);
    }

    #[test]
    fn the_render_function_draws_the_camera_or_the_object() {
        let mut e = engine();
        world(&mut e);
        e.register(RENDER_008D80E0, |_, _| 0x00ab_8000u32.into_ret());
        e.register(RENDER_006629F0, |_, _| 0x00ab_9000u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007b_bcf0, &args![]);
        let scene = e.call(SCENE_00450B80, &args![1u32]).u32();
        assert_eq!(
            calls_to(&e, ACCUMULATE_SCENE),
            vec![vec![0x00ab_9000, scene, 0x00ab_8000]]
        );
        e.set_global(CAMERA_011DA8D0, 0x00ab_a000u32);
        e.call_log = Some(vec![]);
        e.call(0x007b_bcf0, &args![]);
        assert_eq!(calls_to(&e, ACCUMULATE_SCENE)[0][0], 0x00ab_a000);
        assert_eq!(calls_to(&e, RENDER_007148C0), vec![vec![0, 0, 4]]);
        assert_eq!(calls_to(&e, RENDER_004A0FD0).len(), 2);
    }

    #[test]
    fn create_refuses_or_builds_the_menu() {
        let mut e = engine();
        let menu = world(&mut e);
        three_d_doubles(&mut e);
        e.set_global(CASINO_DATA_011DA87C, 0u32);
        e.register(PLAYER_00969B80, |_, _| 1u32.into_ret());
        e.register(KEY_0084E3A0, |_, _| 0x77u32.into_ret());
        e.register(CASINO_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        let empty = e.mem.alloc(4);
        e.register_double(NAME_CHARS_00408DA0, move |_, _| empty.into_ret());
        let casino = e.global::<u32>(CASINO_011DA880);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x007b_be20, &args![casino, 5u32, 50u32, 0u32]).u32(),
            0
        );
        let data = e.global::<u32>(CASINO_DATA_011DA87C);
        assert_eq!(e.mem.u32(data), 0x77);
        assert_eq!(calls_to(&e, LIST_SET_FIRST_00726C60).len(), 1);
        assert_eq!(
            calls_to(&e, SHOW_MESSAGE_007052F0)[0][2],
            MESSAGE_ICON_01049638
        );
        assert_eq!(calls_to(&e, SOUND_HANDLE_PLAY).len(), 1);
        // Too few chips.
        e.register(PLAYER_00969B80, |_, _| 0u32.into_ret());
        let limits = e.mem.alloc(0x40);
        e.mem.set_u32(limits + 0x28, 1000);
        e.register_double(CASINO_LIMITS_00504E60, move |_, _| limits.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x007b_be20, &args![casino, 5u32, 50u32, 0u32]).u32(),
            0
        );
        assert_eq!(calls_to(&e, SPRINTF_S)[0][1..], [0x104, 0x011d_2424]);
        // Built.
        e.register(GET_ITEM_COUNT_IN_CONTAINER, |_, _| 100u32.into_ret());
        e.register(TILE_READ_FILE, |_, _| 0x6100u32.into_ret());
        e.register_double(TILE_GET_MENU, move |_, _| menu.into_ret());
        e.register(slot(1, 0x34), |_, _| 0x43au32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x007b_be20, &args![casino, 5u32, 50u32, 0u32]).u32(),
            0x6100
        );
        assert_eq!(get(&e, menu, RouletteMenu::iMinBet), 5);
        assert_eq!(get(&e, menu, RouletteMenu::iMaxBet), 50);
        assert_eq!(get(&e, menu, RouletteMenu::iCurrentBet), 5);
        assert_eq!(get(&e, menu, RouletteMenu::iChipCount), 100);
        assert_eq!(
            calls_to(&e, MENU_SET_MENU_TILE),
            vec![vec![menu, 0x6100, 0]]
        );
        assert_eq!(calls_to(&e, MENU_PREP_FOR_VISIBILITY), vec![vec![menu, 0]]);
        assert_eq!(calls_to(&e, MODEL_LOADER_LOAD_FILE).len(), 5);
    }

    /// A bet list with one bet (amount, index, model) at a node; removing the first node
    /// empties it.
    fn bet_list(e: &mut Engine, menu: u32, amount: u32, index: u32, model: u32) -> u32 {
        let bet = e.mem.alloc(0xc);
        e.mem.set_u32(bet, amount);
        e.mem.set_u32(bet + 4, index);
        e.mem.set_u32(bet + 8, model);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, bet);
        e.mem.set_u32(menu + 0x3e78, list);
        e.register(LIST_REMOVE_FIRST_0063F7B0, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        bet
    }

    #[test]
    fn do_click_changes_and_places_bets() {
        let mut e = engine();
        let menu = world(&mut e);
        set(&mut e, menu, RouletteMenu::eRouletteState, 3);
        e.call_log = Some(vec![]);
        e.call(0x007b_c870, &args![menu, 6u32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        set(&mut e, menu, RouletteMenu::eRouletteState, 4);
        set(&mut e, menu, RouletteMenu::iChipCount, 100);
        set(&mut e, menu, RouletteMenu::iMaxBet, 50);
        e.set_global(DROP_HEIGHT_01076340, 1.0f64);
        set(&mut e, menu, RouletteMenu::iMinBet, 5);
        for (id, before, after) in [
            (6u32, 5i32, 6i32),
            (6, 45, 50),
            (7, 30, 25),
            (7, 33, 30),
            (7, 6, 5),
            (7, 1, 5),
        ] {
            set(&mut e, menu, RouletteMenu::iCurrentBet, before);
            e.call(0x007b_c870, &args![menu, id, 0u32]);
            assert_eq!(
                get(&e, menu, RouletteMenu::iCurrentBet),
                after,
                "{id} {before}"
            );
        }
        // Spinning needs bets.
        e.call(0x007b_c870, &args![menu, 5u32, 0u32]);
        assert_eq!(get(&e, menu, RouletteMenu::eRouletteState), 4);
        e.mem.set_u32(menu + 0x3e78, 0x00ab_1000);
        e.call_log = Some(vec![]);
        e.call(0x007b_c870, &args![menu, 5u32, 0u32]);
        assert_eq!(get(&e, menu, RouletteMenu::eRouletteState), 1);
        assert_eq!(calls_to(&e, MISC_STAT_INCREMENT), vec![vec![0x28]]);
        assert_eq!(
            calls_to(&e, RANDOM_00944460),
            vec![vec![0, 3], vec![0, 0x26]]
        );
        // Placing a bet at the cursor (index 0, the distance 0.0).
        set(&mut e, menu, RouletteMenu::eRouletteState, 4);
        e.mem.set_u32(menu + 0x3e78, 0);
        set(&mut e, menu, RouletteMenu::iCurrentBet, 10);
        e.call(0x007b_c870, &args![menu, 3u32, 0u32]);
        assert_eq!(get(&e, menu, RouletteMenu::iNumBetsSet), 1);
        assert_eq!(get(&e, menu, RouletteMenu::iTotalBet), 10);
        // Closing.
        e.call(0x007b_c870, &args![menu, 8u32, 0u32]);
        assert_eq!(get(&e, menu, RouletteMenu::eRouletteState), 5);
    }

    #[test]
    fn set_bet_and_remove_bet() {
        let mut e = engine();
        let menu = world(&mut e);
        set(&mut e, menu, RouletteMenu::iCurrentBet, 10);
        set(&mut e, menu, RouletteMenu::iChipCount, 15);
        set(&mut e, menu, RouletteMenu::iMaxBet, 100);
        set(&mut e, menu, RouletteMenu::iNextModelIndex, 3);
        e.set_global(DROP_HEIGHT_01076340, 1.0f64);
        assert!(!e.call(0x007b_ed70, &args![menu, 0x9fu32]).bool());
        assert!(!e.call(0x007b_ed70, &args![menu, 0xffff_ffffu32]).bool());
        let slot_log: Log = Rc::new(RefCell::new(vec![]));
        let first = slot_log.clone();
        e.register_double(LIST_SET_FIRST_00726C60, move |e, a| {
            first.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x007b_ed70, &args![menu, 7u32]).bool());
        let bet = slot_log.borrow()[0].1;
        assert_eq!(
            [e.mem.u32(bet), e.mem.u32(bet + 4), e.mem.u32(bet + 8)],
            [10, 7, 3]
        );
        assert_eq!(get(&e, menu, RouletteMenu::iNumBetsSet), 1);
        assert_eq!(get(&e, menu, RouletteMenu::iNextModelIndex), 4);
        assert_eq!(get(&e, menu, RouletteMenu::iTotalBet), 10);
        // The bet is cut to the chips left (15 - 10).
        assert_eq!(get(&e, menu, RouletteMenu::iCurrentBet), 5);
        assert_eq!(calls_to(&e, MEMSET)[0][1..], [0, 0xc]);
        // A bet on the same index is refused; removing it gives the total back.
        let placed = bet_list(&mut e, menu, 10, 7, 3);
        assert!(!e.call(0x007b_ed70, &args![menu, 7u32]).bool());
        assert!(!e.call(0x007b_f150, &args![menu, 8u32]).bool());
        e.call_log = Some(vec![]);
        assert!(e.call(0x007b_f150, &args![menu, 7u32]).bool());
        assert_eq!(get(&e, menu, RouletteMenu::iTotalBet), 0);
        assert_eq!(get(&e, menu, RouletteMenu::iNumBetsSet), 0);
        assert_eq!(calls_to(&e, FREE), vec![vec![placed]]);
        assert!(!e.call(0x007b_f150, &args![menu, 0x9fu32]).bool());
    }

    #[test]
    fn get_bet_index_inside_then_outside() {
        let mut e = engine();
        let menu = world(&mut e);
        e.mem.set_u32(menu + 0x3f94, 1.0f32.to_bits());
        let mut calls = 0u32;
        e.register_double(POINT_LENGTH_00457990, move |_, _| {
            calls += 1;
            st0(if calls == 4 { 0.5 } else { 5.0 })
        });
        assert_eq!(e.call(0x007b_f390, &args![menu]).u32(), 3);
        e.register(POINT_LENGTH_00457990, |_, _| st0(5.0));
        e.mem.set_u32(menu + 0x3f98 + 4 * 2, 6.0f32.to_bits());
        assert_eq!(e.call(0x007b_f390, &args![menu]).u32(), 0x92);
        e.mem.set_u32(menu + 0x3f98 + 4 * 2, 0);
        assert_eq!(e.call(0x007b_f390, &args![menu]).u32(), 0x9f);
    }

    #[test]
    fn the_chip_helpers() {
        let mut e = engine();
        let menu = world(&mut e);
        e.set_global(RESTING_HEIGHT_0102CAF8, 3.0f32);
        e.call_log = Some(vec![]);
        e.call(0x007b_f480, &args![menu, 4u32]);
        let moved = calls_to(&e, SET_TRANSLATE_004BC1F0);
        assert_eq!(moved.len(), 2);
        assert_eq!(
            moved[0],
            vec![e.mem.u32(menu + 0x3f24), 0, 0, 3.0f32.to_bits()]
        );
        assert_eq!(moved[1][0], e.mem.u32(menu + 0x3f4c));
        assert_eq!(calls_to(&e, SET_TRANSLATE_00A59C60).len(), 2);
        // Below the drop height.
        e.set_global(DROP_HEIGHT_01076340, 1.0f64);
        let node = e.mem.u32(menu + 0x3f1c);
        e.mem.set_u32(node + 0x48, 0.5f32.to_bits());
        assert!(e.call(0x007b_f530, &args![menu, 2u32]).bool());
        e.mem.set_u32(node + 0x48, 1.0f32.to_bits());
        assert!(!e.call(0x007b_f530, &args![menu, 2u32]).bool());
        // Reset: ten chips and ten shadows.
        e.call_log = Some(vec![]);
        e.call(0x007b_f7a0, &args![menu]);
        assert_eq!(calls_to(&e, SET_TRANSLATE_004BC1F0).len(), 20);
        // The chip stack: a bet of 30 is four chips high.
        e.set_global(CHIP_STEP_01070E78, 0.5f64);
        set(&mut e, menu, RouletteMenu::iCurrentBet, 30);
        set(&mut e, menu, RouletteMenu::iNextModelIndex, 3);
        let chip = e.mem.u32(menu + 0x3f20);
        e.mem.set_u32(chip + 0x40, 7.0f32.to_bits());
        e.call_log = Some(vec![]);
        e.call(0x007b_f840, &args![menu]);
        let moved = calls_to(&e, SET_TRANSLATE_004BC1F0);
        assert_eq!(moved[0], vec![chip, 7.0f32.to_bits(), 0, 2.0f32.to_bits()]);
        assert_eq!(moved[1][0], e.mem.u32(menu + 0x3f48));
        assert_eq!(moved[1][3], 0);
    }

    #[test]
    fn the_wheel_and_the_spin_draw() {
        let mut e = engine();
        let menu = world(&mut e);
        for index in 0..38u32 {
            e.mem.set_u32(menu + 0x3e7c + 4 * index, index + 100);
        }
        e.set_global(WHEEL_STEP_01076360, 0.25f64);
        e.set_global(WHEEL_SCALE_01023128, 2.0f64);
        set(&mut e, menu, RouletteMenu::iSpinAnimIndex, 1);
        e.register(GET_OBJECT_BY_NAME, |_, _| 0x00ab_c000u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007b_f570, &args![menu, 108u32]);
        // Place 8: 38 - 8 = 30, plus 3 for animation 1; 33 * 0.25 * 2.
        assert_eq!(calls_to(&e, ROTATION_004A0C90)[0][1], 16.5f32.to_bits());
        assert_eq!(calls_to(&e, SET_ROTATE_0043FA80)[0][0], 0x00ab_c000);
        e.register(RANDOM_00944460, |_, a| (a[1] - 1).into_ret());
        for (luck, lucky, unlucky) in [(5, 0, 0), (7, 1, 0), (3, 0, 1)] {
            set(&mut e, menu, RouletteMenu::iLuckValue, luck);
            e.call(0x007b_f670, &args![menu]);
            assert_eq!(e.mem.u8(menu + 0x400d), lucky);
            assert_eq!(e.mem.u8(menu + 0x400e), unlucky);
        }
        assert_eq!(get(&e, menu, RouletteMenu::iSpinAnimIndex), 2);
        assert_eq!(get(&e, menu, RouletteMenu::iResultValue), 0x25);
    }

    #[test]
    fn set_tile_text_alpha_walks_the_children() {
        let mut e = engine();
        let menu = world(&mut e);
        let parent = e.mem.alloc(0x20);
        e.mem.set_u32(menu + TILES + 4 * 6, parent);
        e.mem.set_u32(parent + 4, 0x00ab_d000);
        let head = e.mem.alloc(4);
        e.mem.set_u32(head, 1);
        e.register_double(LIST_HEAD_SLOT_0073AC90, move |_, _| head.into_ret());
        let child = e.mem.alloc(4);
        e.mem.set_u32(child, 0x5f00);
        e.register_double(NODE_SLOT_006317A0, move |_, _| child.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007b_f710, &args![menu, 6u32, 0.25f32.to_bits()]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x5f00, 0xfa9, 0.25f32.to_bits(), 1]]
        );
        assert_eq!(
            calls_to(&e, NEXT_NODE_007B52D0),
            vec![vec![parent + 4, 0x00ab_d000]]
        );
    }

    #[test]
    fn the_chip_text_is_padded_when_short() {
        let mut e = engine();
        let menu = world(&mut e);
        e.mem.set_u32(menu + 0x34, 0x5340);
        e.set_global(TRAIT_011DA870, 0x77u32);
        e.register(SPRINTF_S, |e, a| {
            let text = if a[2] == CHIPS_FORMAT_01070E80 {
                "abc"
            } else {
                "xy"
            };
            for (index, byte) in text.bytes().enumerate() {
                e.mem.set_u8(a[0] + index as u32, byte);
            }
            e.mem.set_u8(a[0] + text.len() as u32, 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x007b_f9e0, &args![menu]);
        let formats: Vec<u32> = calls_to(&e, SPRINTF_S).iter().map(|w| w[2]).collect();
        assert_eq!(
            formats,
            vec![
                CHIPS_FORMAT_01070E80,
                NAME_FORMAT_01012058,
                PADDED_FORMAT_01029CA8
            ]
        );
        let set_text = calls_to(&e, TILE_SET_STRING);
        assert_eq!(set_text[0][..2], [0x5340, 0x77]);
        assert_eq!(calls_to(&e, SPRINTF_S)[2][3], set_text[0][2]);
    }

    #[test]
    fn do_idle_betting_moves_the_cursor() {
        let mut e = engine();
        let menu = world(&mut e);
        set(&mut e, menu, RouletteMenu::eRouletteState, 4);
        set(&mut e, menu, RouletteMenu::iChipCount, 100);
        set(&mut e, menu, RouletteMenu::iCurrentBet, 5);
        set(&mut e, menu, RouletteMenu::iMinBet, 5);
        e.register(FLAG_004B71D0, |_, _| 1u32.into_ret());
        e.register(STICK_00A23390, |_, a| {
            (if a[2] == 7 { 20000u32 } else { 0 }).into_ret()
        });
        e.register(ABS_00408820, |_, a| {
            st0(f64::from(f32::from_bits(a[0]).abs()))
        });
        e.register(POINT_LENGTH_00457990, |_, _| st0(100.0));
        e.call_log = Some(vec![]);
        e.call(0x007b_d2a0, &args![menu]);
        // -20000 / 10000 = -2.0; the cursor moves by 2.0 * 2.0 the other way.
        assert_eq!(e.mem.u32(menu + 0x3fd4), 4.0f32.to_bits());
        assert_eq!(e.mem.u32(menu + 0x3fd8), 0);
        let cursor = e.mem.u32(e.global::<u32>(MODEL_011DA8A0) + 0x80 + 8);
        assert!(calls_to(&e, SET_TRANSLATE_004BC1F0).contains(&vec![
            cursor,
            4.0f32.to_bits(),
            0,
            0
        ]));
        assert_eq!(
            calls_to(&e, SPRINTF_S)[0][2..],
            [
                OUTSIDE_BET_FORMAT_010762C4,
                SETTING_011D4DBC,
                SETTING_011D3F4C
            ]
        );
        assert_eq!(calls_to(&e, IDLE_STATIC_CONSTRUCT_0041A250).len(), 1);
        // Below the minimum bet the menu closes.
        set(&mut e, menu, RouletteMenu::iChipCount, 4);
        e.call(0x007b_d2a0, &args![menu]);
        assert_eq!(get(&e, menu, RouletteMenu::eRouletteState), 5);
        assert_eq!(calls_to(&e, IDLE_STATIC_CONSTRUCT_0041A250).len(), 1);
    }

    #[test]
    fn do_idle_pays_out_a_winning_bet() {
        let mut e = engine();
        let menu = world(&mut e);
        set(&mut e, menu, RouletteMenu::eRouletteState, 3);
        set(&mut e, menu, RouletteMenu::iChipCount, 100);
        set(&mut e, menu, RouletteMenu::iTotalBet, 10);
        set(&mut e, menu, RouletteMenu::iMinBet, 5);
        set(&mut e, menu, RouletteMenu::iResultValue, 4);
        let record = menu + 0x5c + 0x64 * 0x95;
        e.mem.set_u32(record + 0xc, 1);
        bet_list(&mut e, menu, 10, 0x95, 3);
        let limits = e.mem.alloc(0x40);
        e.mem.set_u32(limits + 0x28, 1000);
        e.register_double(CASINO_LIMITS_00504E60, move |_, _| limits.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007b_d2a0, &args![menu]);
        assert_eq!(get(&e, menu, RouletteMenu::iChipCount), 110);
        let data = e.global::<u32>(CASINO_DATA_011DA87C);
        assert_eq!(e.mem.u32(data + 4), 10);
        let message = calls_to(&e, SPRINTF_S)
            .into_iter()
            .find(|w| w[2] == RESULT_FORMAT_01070D0C)
            .unwrap();
        assert_eq!(message[3..], [0x011d_2ff4, 10, 0x011d_5050, 0x011d_3114]);
        assert_eq!(get(&e, menu, RouletteMenu::eRouletteState), 4);
        assert_eq!(get(&e, menu, RouletteMenu::iCurrentBet), 5);
        assert_eq!(get(&e, menu, RouletteMenu::iTotalBet), 0);
        assert_eq!(e.mem.u8(menu + 0x400f), 0);
        // An odd result loses the even bet.
        bet_list(&mut e, menu, 10, 0x95, 3);
        set(&mut e, menu, RouletteMenu::eRouletteState, 3);
        set(&mut e, menu, RouletteMenu::iTotalBet, 10);
        set(&mut e, menu, RouletteMenu::iResultValue, 5);
        e.call(0x007b_d2a0, &args![menu]);
        assert_eq!(get(&e, menu, RouletteMenu::iChipCount), 100);
    }

    #[test]
    fn do_idle_spins_then_closes() {
        let mut e = engine();
        let menu = world(&mut e);
        set(&mut e, menu, RouletteMenu::eRouletteState, 1);
        e.mem.set_u32(menu + 0x3f84, 0x00ab_e000);
        e.register(SEQUENCE_LENGTH_00508100, |_, _| st0(0.5));
        e.call_log = Some(vec![]);
        e.call(0x007b_d2a0, &args![menu]);
        assert_eq!(calls_to(&e, ACTIVATE_SEQUENCE_0047AAB0).len(), 1);
        assert_eq!(e.mem.u8(menu + 0x400c), 1);
        assert_eq!(get(&e, menu, RouletteMenu::eRouletteState), 1);
        // On top, 600 milliseconds later: past the end of the spin.
        e.register(slot(1, 0x34), |_, _| 0x43au32.into_ret());
        e.register(INTERFACE_IS_TOP_MENU_ID, |_, _| 1u32.into_ret());
        e.mem.set_u32(menu + 0x58, 1000);
        e.register(WORD_AT_14_00825C00, |_, _| 1600u32.into_ret());
        e.set_global(LAST_STATE_011A081C, 1u32);
        e.call_log = Some(vec![]);
        e.call(0x007b_d2a0, &args![menu]);
        assert_eq!(e.global::<u32>(SPIN_TIME_011DA904), 0.6f32.to_bits());
        assert_eq!(e.mem.u8(menu + 0x400c), 0);
        assert_eq!(get(&e, menu, RouletteMenu::eRouletteState), 3);
        assert_eq!(calls_to(&e, DEACTIVATE_SEQUENCE_0047B220).len(), 1);
        // Closing: the lost chips are removed with a message, the script runs.
        set(&mut e, menu, RouletteMenu::eRouletteState, 5);
        set(&mut e, menu, RouletteMenu::iChipCount, 100);
        e.register(GET_ITEM_COUNT_IN_CONTAINER, |_, _| 120u32.into_ret());
        e.mem.set_u8(menu + 0x400f, 1);
        e.register(CASINO_SCRIPT_00504E40, |_, _| 0x00ab_f000u32.into_ret());
        let player = e.global::<u32>(PLAYER_011DEA3C);
        e.call_log = Some(vec![]);
        e.call(0x007b_d2a0, &args![menu]);
        assert_eq!(calls_to(&e, slot(4, 0x17c))[0][..4], [player, 0, 0, 20]);
        assert_eq!(
            calls_to(&e, SHOW_MESSAGE_007052F0)[0][2],
            LOST_ICON_010208A0
        );
        assert_eq!(
            calls_to(&e, SCRIPT_RUN_0060C9C0),
            vec![vec![0x00ab_f000, 1]]
        );
        // Winnings within the casino's limit are given without a message.
        e.register(GET_ITEM_COUNT_IN_CONTAINER, |_, _| 80u32.into_ret());
        e.register(CASINO_00737E30, |_, _| 50u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007b_d2a0, &args![menu]);
        assert_eq!(calls_to(&e, CONTAINER_GIVE)[0][1..], [player, 0]);
        assert!(calls_to(&e, SHOW_MESSAGE_007052F0).is_empty());
    }
}
