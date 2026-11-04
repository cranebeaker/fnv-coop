//! `fallout/interface/menus/slotmachinemenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `SlotMachineMenu` (Xbox PDB); the offsets below are the PC code's (`pTiles` at +0xa0,
    /// ten of them; three reels from +0xdc, +0xe8, +0x108 and +0x148).
    pub struct SlotMachineMenu: 0x158 {
        /// `eSlotState` (Xbox PDB).
        0x28 eSlotState: i32,
        /// `eSpinResult` (Xbox PDB).
        0x9c eSpinResult: i32,
        /// `iLuckSkill` (Xbox PDB).
        0xc8 iLuckSkill: i32,
        /// `iCurrentBet` (Xbox PDB).
        0xcc iCurrentBet: i32,
        /// `iNumActiveSymbols` (Xbox PDB).
        0xd0 iNumActiveSymbols: i32,
        /// `iChipCount` (Xbox PDB).
        0xd4 iChipCount: i32,
        /// `bLucky` (Xbox PDB).
        0xd8 bLucky: bool,
        /// `uiPrevIdleTick` (Xbox PDB).
        0xf4 uiPrevIdleTick: u32,
        /// `bSwapTextures` (Xbox PDB).
        0xf8 bSwapTextures: bool,
        /// `iNextSoundIndex` (Xbox PDB).
        0xfc iNextSoundIndex: i32,
        /// `bRunScriptOnExit` (Xbox PDB).
        0x100 bRunScriptOnExit: bool,
        /// `pSlotMachineGeometry` (Xbox PDB).
        0x104 pSlotMachineGeometry: u32,
        /// `pSlotAnimManager` (Xbox PDB).
        0x134 pSlotAnimManager: u32,
        /// `pSlotLeverAnim` (Xbox PDB).
        0x138 pSlotLeverAnim: u32,
        /// `pSlotCoinAnim` (Xbox PDB).
        0x13c pSlotCoinAnim: u32,
    }
}

/// `eActiveReelSymbols` (+0x2c, 28 words), `pTiles` (+0xa0), `eReelStartSymbols` (+0xdc),
/// `eReelResults` (+0xe8), `pReelGeometry` (+0x108), `pSlotReelTextures` (+0x114: eight
/// `NiPointer`s and the blank one at +0x130), `pSlotPayoutListAnim` (+0x140, +0x144) and
/// `pSlotReelAnim` (+0x148).
const ACTIVE_SYMBOLS: u32 = 0x2c;
const TILES: u32 = 0xa0;
const REEL_START: u32 = 0xdc;
const REEL_RESULTS: u32 = 0xe8;
const REEL_GEOMETRY: u32 = 0x108;
const REEL_TEXTURES: u32 = 0x114;
const PAYOUT_ANIMATIONS: u32 = 0x140;
const REEL_ANIMATIONS: u32 = 0x148;

/// `SlotMachineMenu::pMe`, `casinoData`, `pCasino`, `iMaxBet` and `iMinBet` (Xbox PDB
/// statics; `011da9a4`, `011da9a8`, `011da9ac`, `011a0828`, `011a082c`), the model node and
/// camera (`NiPointer`s at `011da9fc` and `011da9d0`), the two trait ids (`011da998`,
/// `011da99c`), the idle's elapsed time and last state (`011daa30`, a `float`, and
/// `011daa2c`) and the object the reel textures are made through (`011dea10`).
const SLOT_MENU_011DA9A4: u32 = 0x011d_a9a4;
const CASINO_DATA_011DA9A8: u32 = 0x011d_a9a8;
const CASINO_011DA9AC: u32 = 0x011d_a9ac;
const MAX_BET_011A0828: u32 = 0x011a_0828;
const MIN_BET_011A082C: u32 = 0x011a_082c;
const MODEL_011DA9FC: u32 = 0x011d_a9fc;
const CAMERA_011DA9D0: u32 = 0x011d_a9d0;
const TRAIT_011DA998: u32 = 0x011d_a998;
const TRAIT_011DA99C: u32 = 0x011d_a99c;
const SPIN_TIME_011DAA30: u32 = 0x011d_aa30;
const LAST_STATE_011DAA2C: u32 = 0x011d_aa2c;
const TEXTURES_011DEA10: u32 = 0x011d_ea10;
const PLAYER_011DEA3C: u32 = 0x011d_ea3c;
const MODEL_LOADER_011C3B3C: u32 = 0x011c_3b3c;
/// The menu's vtable (`010764dc`).
const SLOT_MENU_VTABLE: u32 = 0x0107_64dc;
/// The timer object (`011f6394`) whose +0x14 (`00825c00`) is the time in milliseconds.
const TIMER_011F6394: u32 = 0x011f_6394;
const WORD_AT_14_00825C00: u32 = 0x0082_5c00;

/// `MemoryManager` allocate and free (`00401000`, `00401030`, `cdecl`).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// The helpers the roulette menu uses too (see `roulettemenu.rs`): the player's
/// `00969b20`, the casino's models (`00504e00`, `RET 4`) and textures (`00504e20`, `RET 4`),
/// `ModelLoader::LoadFile` (`00447080`), the loader's `0045a5e0`, the scene node
/// (`00450b80`), `00b5d180`, `00572160`, `NiPointer` assignment (`0066b0d0`), `00559450`
/// (the word at +0), the `NiPointer` destructor (`0045cec0`), the vector destructor
/// (`00ec5fce`), the menu base destructor (`00a1c520`).
const PLAYER_00969B20: u32 = 0x0096_9b20;
const CASINO_MODEL_00504E00: u32 = 0x0050_4e00;
const CASINO_TEXTURE_00504E20: u32 = 0x0050_4e20;
const MODEL_LOADER_LOAD_FILE: u32 = 0x0044_7080;
const MODEL_LOADER_0045A5E0: u32 = 0x0045_a5e0;
const SCENE_00450B80: u32 = 0x0045_0b80;
const SCENE_00B5D180: u32 = 0x00b5_d180;
const NODE_00572160: u32 = 0x0057_2160;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const WORD_AT_0_00559450: u32 = 0x0055_9450;
const NI_POINTER_DESTROY: u32 = 0x0045_cec0;
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;
const MENU_DESTRUCT: u32 = 0x00a1_c520;
/// `InterfaceManager` (`004b7210`), its word at +0x28 (`0045cd60`), and the tile setters:
/// `SetFloat` (`00a012d0`), `SetString` (`00a01350`), `00700320` (the integer), `Poke`
/// (`00a01290`), `GetFloat` (`00a011b0`).
const INTERFACE_MANAGER: u32 = 0x004b_7210;
const WORD_AT_28_0045CD60: u32 = 0x0045_cd60;
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const TILE_SET_STRING: u32 = 0x00a0_1350;
const TILE_SET_INT: u32 = 0x0070_0320;
const TILE_POKE: u32 = 0x00a0_1290;
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
/// CRT string helpers (`cdecl`): `sprintf_s` (`00406d00`), `strlen` (`0044a670`).
const SPRINTF_S: u32 = 0x0040_6d00;
const STRLEN: u32 = 0x0044_a670;
/// `SwapPlatformLanguageTexturePath` (`004b7240`, `cdecl`) and `TES::CreateTextureImage`
/// (`004568c0`, `RET 0x10`).
const SWAP_TEXTURE_PATH: u32 = 0x004b_7240;
const CREATE_TEXTURE_IMAGE: u32 = 0x0045_68c0;
/// The scene graph helpers (as in `roulettemenu.rs`).
const SET_TRANSLATE_004BC1F0: u32 = 0x004b_c1f0;
const GET_OBJECT_BY_NAME: u32 = 0x004a_ae30;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
const POINT_CONSTRUCT_0043D410: u32 = 0x0043_d410;
const SET_TRANSLATE_00A59C60: u32 = 0x00a5_9c60;
const GET_CONTROLLER: u32 = 0x00a5_c570;
const GET_WORLD_BOUND: u32 = 0x0043_d450;
const BOUND_RADIUS_0084D030: u32 = 0x0084_d030;
const ROTATION_004A0C90: u32 = 0x004a_0c90;
const SET_ROTATE_0043FA80: u32 = 0x0043_fa80;
const ADD_ALL_LIGHTS_FROM_NODE: u32 = 0x00b5_ca70;
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTROY: u32 = 0x0043_81b0;
const SEQUENCE_BY_NAME_0047A520: u32 = 0x0047_a520;
const DEACTIVATE_ALL: u32 = 0x0048_fef0;
const START_ANIMATIONS: u32 = 0x00a6_d2d0;
const ACTIVATE_SEQUENCE_0047AAB0: u32 = 0x0047_aab0;
const DEACTIVATE_SEQUENCE_0047B220: u32 = 0x0047_b220;
const SEQUENCE_LENGTH_00508100: u32 = 0x0050_8100;
const NODE_CHILD_0043B4A0: u32 = 0x0043_b4a0;
const NODE_SELF: u32 = 0x0068_15c0;
/// `NiMatrix3::MakeXRotation` (`00524ac0`, `RET 4`) and `NiMatrix3::operator*`
/// (`0043f8d0`, `RET 8`).
const MAKE_X_ROTATION: u32 = 0x0052_4ac0;
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
/// `NiAVObject::GetProperty` (`00a59d30`, `RET 4`), a property's type (`00441110`) and its
/// vtable +0xf0 (the texture set).
const GET_PROPERTY: u32 = 0x00a5_9d30;
const PROPERTY_TYPE_00441110: u32 = 0x0044_1110;
/// The camera maker's callees: the camera constructor (`00a712f0`), `00414430` (`RET
/// 0x10`), `00712e60` (`RET 4`), `NiFrustum::NiFrustum` (`00a71b70`, `RET 4`), `0045c670`,
/// `00403e20` (a game setting's value), `00507700` (`RET 4`), the screen size words
/// (`011f9426`, `011f9434`, `004dc200`, `004dc1f0`), `0057dd70` (`cdecl`, `ST0`),
/// `NiCamera::SetViewFrustum` (`00a6faf0`, `RET 4`), a point (`00416870`, `RET 0xc`),
/// `NiCamera::LookAtWorldPoint` (`00a701b0`, `RET 8`), `00733fd0` (`RET 4`) and `00a5b950`
/// (`RET 4`: the name).
const CAMERA_CONSTRUCT: u32 = 0x00a7_12f0;
const CAMERA_00414430: u32 = 0x0041_4430;
const CAMERA_00712E60: u32 = 0x0071_2e60;
const FRUSTUM_CONSTRUCT: u32 = 0x00a7_1b70;
const RENDER_0045C670: u32 = 0x0045_c670;
const SETTING_VALUE_00403E20: u32 = 0x0040_3e20;
const CAMERA_00507700: u32 = 0x0050_7700;
const WINDOWED_011F9426: u32 = 0x011f_9426;
const WIDTH_011F9434: u32 = 0x011f_9434;
const SCREEN_004DC200: u32 = 0x004d_c200;
const SCREEN_004DC1F0: u32 = 0x004d_c1f0;
const TAN_0057DD70: u32 = 0x0057_dd70;
const SET_VIEW_FRUSTUM: u32 = 0x00a6_faf0;
const POINT_00416870: u32 = 0x0041_6870;
const LOOK_AT_WORLD_POINT: u32 = 0x00a7_01b0;
const CAMERA_00733FD0: u32 = 0x0073_3fd0;
const SET_NAME_00A5B950: u32 = 0x00a5_b950;
/// Sounds: `BSAudio::QInstance` (`00453a70`), `GetSoundHandleByName` (`00ad7550`),
/// `BSSoundHandle::Play` (`00ad8830`), the handle's destructor (`00483710`).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_DESTROY: u32 = 0x0048_3710;
/// The menu, casino, list and message helpers shared with the roulette menu (see there).
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
const CASINO_DATA_LIST_005A6260: u32 = 0x005a_6260;
const CASINO_DATA_CONSTRUCT: u32 = 0x0073_3f50;
const KEY_0084E3A0: u32 = 0x0084_e3a0;
const WORD_AT_4_00726070: u32 = 0x0072_6070;
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_APPEND: u32 = 0x005a_e3d0;
const LIST_SET_FIRST_00726C60: u32 = 0x0072_6c60;
const DATA_VALID_004839C0: u32 = 0x0048_39c0;
const PLAYER_00969B80: u32 = 0x0096_9b80;
const PLAYER_00969B40: u32 = 0x0096_9b40;
const SETTING_STRING: u32 = 0x0040_3df0;
const SHOW_MESSAGE_007052F0: u32 = 0x0070_52f0;
const CASINO_LIMITS_00504E60: u32 = 0x0050_4e60;
const CASINO_ITEM_007316D0: u32 = 0x0073_16d0;
const GET_ITEM_COUNT_IN_CONTAINER: u32 = 0x0057_5610;
const WORD_AT_9C_00586150: u32 = 0x0058_6150;
const TILE_READ_FILE: u32 = 0x00a0_1b00;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
const DEBUG_MESSAGE: u32 = 0x005b_5e40;
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
const GET_CLAMPED_ACTOR_VALUE: u32 = 0x0066_ef20;
const NAME_CHARS_00408DA0: u32 = 0x0040_8da0;
const TILE_CHILD_BY_NAME_00A08B20: u32 = 0x00a0_8b20;
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
const RANDOM_00944460: u32 = 0x0094_4460;
const LUCK_007C58F0: u32 = 0x007c_58f0;
const MISC_STAT_INCREMENT: u32 = 0x004d_5c60;
const MENU_FADE_STATE_0059BB30: u32 = 0x0059_bb30;
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
const GET_PICK_UP_SOUND_NAME: u32 = 0x008a_dcf0;
const GET_FULL_NAME: u32 = 0x0048_2720;
const CASINO_00737E30: u32 = 0x0073_7e30;
const CASINO_SCRIPT_00504E40: u32 = 0x0050_4e40;
const SCRIPT_RUN_0060C9C0: u32 = 0x0060_c9c0;
const CONTAINER_CONSTRUCT: u32 = 0x0048_1610;
const CONTAINER_ADD: u32 = 0x0048_18e0;
const CONTAINER_GIVE: u32 = 0x0048_21a0;
const WINNINGS_CONTAINER_DESTRUCT: u32 = 0x0048_1680;
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTROY: u32 = 0x0040_37d0;
const STRING_FORMAT: u32 = 0x0040_6f60;
const INTERFACE_IS_TOP_MENU_ID: u32 = 0x0070_2450;
/// The render function's callees (as in `roulettemenu.rs`).
const RENDER_0043C4B0: u32 = 0x0043_c4b0;
const RENDER_007148C0: u32 = 0x0071_48c0;
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

/// Paths, names, formats and texts (`.rdata`).
const REEL_MODEL_PATH_0107659C: u32 = 0x0107_659c;
const REEL_MODEL_PATH_01076560: u32 = 0x0107_6560;
const REEL_MODEL_PATH_01076524: u32 = 0x0107_6524;
const TEXTURE_FORMAT_0101996C: u32 = 0x0101_996c;
const TEXTURE_FOLDER_0101D7FC: u32 = 0x0101_d7fc;
const BLANK_FORMAT_010766B8: u32 = 0x0107_66b8;
const BLANK_FOLDER_010766D8: u32 = 0x0107_66d8;
const BLANK_NAME_01063DD4: u32 = 0x0106_3dd4;
const NO_SYMBOLS_MESSAGE_01076700: u32 = 0x0107_6700;
const LEVER_SEQUENCE_01048458: u32 = 0x0104_8458;
const COIN_SEQUENCE_0104844C: u32 = 0x0104_844c;
const PAYOUT_SEQUENCE_01076780: u32 = 0x0107_6780;
const PAYOUT_SEQUENCE_01076774: u32 = 0x0107_6774;
const REEL_FACE_FORMAT_0107681C: u32 = 0x0107_681c;
const CAMERA_NAME_01070C70: u32 = 0x0107_0c70;
const CHIPS_FORMAT_01070E80: u32 = 0x0107_0e80;
const NAME_FORMAT_01012058: u32 = 0x0101_2058;
const PADDED_FORMAT_01029CA8: u32 = 0x0102_9ca8;
const BET_SOUND_FORMAT_010767A8: u32 = 0x0107_67a8;
const SOUND_LOWER_01076790: u32 = 0x0107_6790;
const SOUND_SPIN_01076808: u32 = 0x0107_6808;
const SOUND_WIN_4_010767F4: u32 = 0x0107_67f4;
const SOUND_WIN_5_010767E4: u32 = 0x0107_67e4;
const SOUND_WIN_6_010767D0: u32 = 0x0107_67d0;
const SOUND_LUCKY_LOSS_010767C0: u32 = 0x0107_67c0;
const SOUND_OPEN_01076600: u32 = 0x0107_6600;
const SOUND_REFUSE_01025CC4: u32 = 0x0102_5cc4;
const MENU_FILE_TEXT: u32 = 0x0107_6670;
const CREATION_FAILED_TEXT: u32 = 0x0107_6618;
const NO_DATA_MESSAGE_01076694: u32 = 0x0107_6694;
const BANNED_FORMAT_01070C58: u32 = 0x0107_0c58;
const UNNAMED_FORMAT_010765F0: u32 = 0x0107_65f0;
const CHIP_CHILD_NAME: u32 = 0x0107_65d8;
const MESSAGE_ICON_01049638: u32 = 0x0104_9638;
const RESULT_FORMAT_01070D0C: u32 = 0x0107_0d0c;
const LOST_FORMAT_0101C178: u32 = 0x0101_c178;
const WON_FORMAT_01070D8C: u32 = 0x0107_0d8c;
const LOST_ICON_010208A0: u32 = 0x0102_08a0;
const WON_ICON_0103A830: u32 = 0x0103_a830;
const SETTING_011D3E08: u32 = 0x011d_3e08;
/// `float`s and `double`s: the cursor fade (`01023cd8`), the message `float` (`010162c0`),
/// 0.0 and 1.0 (`01012060`, `01012070`), 1000.0 (`01017b70`), the payout step
/// (`010290b0`), `6006.0` and `102.0` (`010704f0`, `010704e8`), the model's rotation and
/// position (`0101ff38`; `0105478c`, `0107678c`), the light scale (`01021928`) and the
/// camera's values (`01016264`, `01023128`, `01070c80`; the settings `01203144`,
/// `01203150`).
const CURSOR_FADE_01023CD8: u32 = 0x0102_3cd8;
const MESSAGE_VALUE_010162C0: u32 = 0x0101_62c0;
const ZERO_DOUBLE: u32 = 0x0101_2060;
const ONE_DOUBLE: u32 = 0x0101_2070;
const THOUSAND_DOUBLE: u32 = 0x0101_7b70;
const PAYOUT_STEP_010290B0: u32 = 0x0102_90b0;
const VALUE_6006_DOUBLE: u32 = 0x0107_04f0;
const VALUE_102_DOUBLE: u32 = 0x0107_04e8;
const MODEL_ROTATION_0101FF38: u32 = 0x0101_ff38;
const MODEL_X_0105478C: u32 = 0x0105_478c;
const MODEL_Y_0107678C: u32 = 0x0107_678c;
const LIGHT_SCALE_01021928: u32 = 0x0102_1928;
const CAMERA_SCALE_01016264: u32 = 0x0101_6264;
const CAMERA_DEGREES_01023128: u32 = 0x0102_3128;
const CAMERA_HALF_01070C80: u32 = 0x0107_0c80;
const NEAR_SETTING_01203144: u32 = 0x0120_3144;
const FOV_SETTING_01203150: u32 = 0x0120_3150;
const CAMERA_VALUE_01030FF4: u32 = 0x0103_0ff4;
const CONTROLLER_RTTI_011F36AC: u32 = 0x011f_36ac;

/// The open slot machine menu (`pMe`).
fn slot_menu(e: &Engine) -> u32 {
    e.global(SLOT_MENU_011DA9A4)
}

/// The item word of a list node, reached through `006815c0` as the code does.
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(NODE_SELF, &args![node]).u32();
    e.mem.u32(slot)
}

/// The model node (`spSlotModel`).
fn model(e: &mut Engine) -> u32 {
    e.call(WORD_AT_0_00559450, &args![MODEL_011DA9FC]).u32()
}

/// Child `index` of the model node (`0043b4a0`).
fn model_child(e: &mut Engine, index: u32) -> u32 {
    let node = model(e);
    e.call(NODE_CHILD_0043B4A0, &args![node, index]).u32()
}

/// A path from one of the casino's models (`00504e00(index)`, its vtable +0x14).
fn casino_model_path(e: &mut Engine, index: u32) -> u32 {
    let casino = e.global::<u32>(CASINO_011DA9AC);
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

/// `Tile::SetFloat(0xfa9, value, 1)` on the interface manager's cursor tile (+0x28).
fn set_cursor_fade(e: &mut Engine, value: u32) {
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    let cursor = e.call(WORD_AT_28_0045CD60, &args![interface]).u32();
    e.call(TILE_SET_FLOAT, &args![cursor, 0xfa9u32, value, 1u32]);
}

/// `007052f0(text, 0, 01049638, sound, float at 010162c0, 0)`.
fn show_message(e: &mut Engine, text: u32, icon: u32, sound: u32) {
    let value = x87_quiet_bits(e.global::<u32>(MESSAGE_VALUE_010162C0));
    e.call(
        SHOW_MESSAGE_007052F0,
        &args![text, 0u32, icon, sound, value, 0u32],
    );
}

/// A refusal: `sprintf_s(buffer, 0x104, text)` (the text as the format), the message box
/// and the refusal sound.
fn refuse_with(e: &mut Engine, text: u32) {
    e.with_stack(0x104, |e, buffer| {
        e.call(SPRINTF_S, &args![buffer, 0x104u32, text]);
        show_message(e, buffer.addr(), MESSAGE_ICON_01049638, 0);
    });
    play_sound(e, SOUND_REFUSE_01025CC4);
}

/// `Poke(trait, 1.0)` then the integer of the same trait on a tile of the open menu.
fn show_number(e: &mut Engine, offset: u32, value: u32) {
    let trait_id = e.global::<u32>(TRAIT_011DA998);
    let tile = e.mem.u32(slot_menu(e).wrapping_add(offset));
    e.call(TILE_POKE, &args![tile, trait_id, 1.0f32.to_bits()]);
    let trait_id = e.global::<u32>(TRAIT_011DA998);
    let tile = e.mem.u32(slot_menu(e).wrapping_add(offset));
    e.call(TILE_SET_INT, &args![tile, trait_id, value]);
}

/// A point from (`x`, 1, 0) (`0043d410`; the 1 is the integer word the code pushes) given to
/// `00a59c60` of a model child.
fn translate_child_by_time(e: &mut Engine, index: u32) {
    e.with_stack(0xc, |e, point| {
        let time = x87_quiet_bits(e.global::<u32>(SPIN_TIME_011DAA30));
        let made = e
            .call(POINT_CONSTRUCT_0043D410, &args![point, time, 1u32, 0u32])
            .u32();
        let child = model_child(e, index);
        e.call(SET_TRANSLATE_00A59C60, &args![child, made]);
    });
}

/// The idle's elapsed time as `FLD` loads it.
fn spin_time(e: &Engine) -> f64 {
    x87_load32(e.global::<u32>(SPIN_TIME_011DAA30))
}

/// `ActivateSequence(sequence, 0, 0, 1.0, 0.0, 0)` on the slot animation manager.
fn activate(e: &mut Engine, this: Ptr<SlotMachineMenu>, sequence: u32) {
    let manager = e.get(this, SlotMachineMenu::pSlotAnimManager);
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
}

/// `DeactivateSequence(sequence, 0.0)` on the slot animation manager.
fn deactivate(e: &mut Engine, this: Ptr<SlotMachineMenu>, sequence: u32) {
    let manager = e.get(this, SlotMachineMenu::pSlotAnimManager);
    e.call(
        DEACTIVATE_SEQUENCE_0047B220,
        &args![manager, sequence, 0.0f32.to_bits()],
    );
}

// Translated from 007c0800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SlotMachineMenu::GetClass` (Xbox PDB; `thiscall`): 0x438.
pub fn slot_machine_menu_get_class(_e: &mut Engine, _this: Ptr<SlotMachineMenu>) -> u32 {
    0x438
}

// Translated from 007c0810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SlotMachineMenu::AttachTileByID` (Xbox PDB; `RET 8`: an id and a tile): ids 0 to 9
/// store the tile in `pTiles`.
pub fn slot_machine_menu_attach_tile_by_id(
    e: &mut Engine,
    this: Ptr<SlotMachineMenu>,
    id: i32,
    tile: u32,
) {
    if (0..0xa).contains(&id) {
        e.mem.set_u32(this.addr() + TILES + 4 * id as u32, tile);
    }
}

// Translated from 007c0840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SlotMachineMenu::\`scalar deleting destructor'` (Xbox PDB; `RET 4`, the flags): the
/// destructor (`007c0870`), then the memory freed with flag 1. Returns `this`.
pub fn slot_machine_menu_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<SlotMachineMenu>,
    flags: u32,
) -> Ptr<SlotMachineMenu> {
    fn_007c0870(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 007c0870 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007c0870 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1) is not translated.
/// The `SlotMachineMenu` destructor (`thiscall`): the vtable `010764dc`; `00969b20` on the
/// player; the casino's machine model (7) and the three reel models released; the model
/// node detached from the scene (vtable +0xe8 of `00450b80(1)`), `00b5d180`, `00572160`; the
/// geometry, camera, model and animation manager cleared, the reels and reel animations
/// cleared, the eight reel textures dropped; `pCasino` and `pMe` cleared; the reel textures
/// destroyed (`00ec5fce` with `0045cec0`) and the menu base destroyed.
pub fn fn_007c0870(e: &mut Engine, this: Ptr<SlotMachineMenu>) {
    let base = this.addr();
    e.mem.set_u32(base, SLOT_MENU_VTABLE);
    let player = e.global::<u32>(PLAYER_011DEA3C);
    e.call(PLAYER_00969B20, &args![player]);
    let path = casino_model_path(e, 7);
    release_model(e, path);
    for path in [
        REEL_MODEL_PATH_0107659C,
        REEL_MODEL_PATH_01076560,
        REEL_MODEL_PATH_01076524,
    ] {
        release_model(e, path);
    }
    let scene = e.call(SCENE_00450B80, &args![1u32]).u32();
    let node = model(e);
    e.vcall(scene, 0xe8, &args![node]);
    let scene = e.call(SCENE_00450B80, &args![1u32]).u32();
    e.call(SCENE_00B5D180, &args![scene]);
    let node = model(e);
    e.call(NODE_00572160, &args![node]);
    e.set(this, SlotMachineMenu::pSlotMachineGeometry, 0);
    e.call(NI_POINTER_ASSIGN, &args![CAMERA_011DA9D0, 0u32]);
    e.call(NI_POINTER_ASSIGN, &args![MODEL_011DA9FC, 0u32]);
    e.set(this, SlotMachineMenu::pSlotAnimManager, 0);
    for reel in 0..3u32 {
        e.mem.set_u32(base + REEL_GEOMETRY + 4 * reel, 0);
        e.mem.set_u32(base + REEL_ANIMATIONS + 4 * reel, 0);
    }
    for index in 0..8u32 {
        e.call(
            NI_POINTER_ASSIGN,
            &args![base + REEL_TEXTURES + 4 * index, 0u32],
        );
    }
    e.set_global(CASINO_011DA9AC, 0u32);
    e.set_global(SLOT_MENU_011DA9A4, 0u32);
    e.call(
        VECTOR_DESTRUCT,
        &args![base + REEL_TEXTURES, 4u32, 8u32, NI_POINTER_DESTROY],
    );
    e.call(MENU_DESTRUCT, &args![this]);
}

// Translated from 007c0a40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007c0a40 - the C++ exception frame (FS:[0] chain, unwind states 0 to 5) and the /GS cookie check (00ec408c) are not translated.
/// `SlotMachineMenu::Create` (Xbox PDB; `cdecl`: the casino, the minimum bet, the maximum
/// bet and the chips needed). The bets go to the statics (`iMinBet`, `iMaxBet`), the casino
/// to `pCasino`, `casinoData` is cleared and then found in the player's casino data list
/// (an item without a key or one `004839c0` refuses shows the message `01076694` and
/// returns 0) or made. Refused (message and sound, 0) when banned (`00969b80`; `01070c58`
/// with setting `011d2ac0` and `00969b40`), at the casino's winnings limit (setting
/// `011d3abc`), with fewer chips than the minimum (`011d2424`) or fewer winnings than needed
/// (`011d3594`). An open menu of the class is deleted; the menu is read from `01076670`;
/// one of another class, or without a casino, is deleted when it has a tile and 0
/// returned. Otherwise the menu tile is attached (without all tiles a debug message and
/// 0), the depth trait set, the open sound played, the luck (actor value 0xb) and bet set,
/// the bet and chip tiles, the title (the casino's name with setting `011d3e08`, or its
/// vtable +0x130 name with `010765f0`), the chip text (`007c5e60`) and the labels set up;
/// the casino data is initialised (`007c1490`; on failure the menu closes and 0 is
/// returned), the 3D elements prepared and the menu made visible; the menu tile returned.
pub fn slot_machine_menu_create(
    e: &mut Engine,
    casino: u32,
    minimum: i32,
    maximum: i32,
    needed: i32,
) -> u32 {
    let existing = e.call(TILE_GET_MENU_BY_CLASS, &args![0x438u32]).u32();
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    e.set_global(MAX_BET_011A0828, maximum);
    e.set_global(MIN_BET_011A082C, minimum);
    e.set_global(CASINO_011DA9AC, casino);
    e.set_global(CASINO_DATA_011DA9A8, 0u32);
    let player = e.global::<u32>(PLAYER_011DEA3C);
    let mut node = e.call(CASINO_DATA_LIST_005A6260, &args![player]).u32();
    while node != 0 && node_item(e, node) != 0 {
        let data = node_item(e, node);
        let valid = e.mem.u32(data) != 0 && {
            let data = node_item(e, node);
            let key = e.mem.u32(data);
            e.call(DATA_VALID_004839C0, &args![key]).u32() != 0
        };
        if !valid {
            show_message(e, NO_DATA_MESSAGE_01076694, MESSAGE_ICON_01049638, 0);
            return 0;
        }
        let casino = e.global::<u32>(CASINO_011DA9AC);
        let key = e.call(KEY_0084E3A0, &args![casino]).u32();
        let data = node_item(e, node);
        if key == e.mem.u32(data) {
            let data = node_item(e, node);
            e.set_global(CASINO_DATA_011DA9A8, data);
            break;
        }
        if e.call(WORD_AT_4_00726070, &args![node]).u32() == 0 {
            break;
        }
        node = e.call(WORD_AT_4_00726070, &args![node]).u32();
    }
    if e.global::<u32>(CASINO_DATA_011DA9A8) == 0 {
        e.with_stack(4, |e, local| {
            let block = e.call(ALLOCATE, &args![0xcu32]).u32();
            let data = if block != 0 {
                e.call(CASINO_DATA_CONSTRUCT, &args![block]).u32()
            } else {
                0
            };
            e.mem.set_u32(local.addr(), data);
            let casino = e.global::<u32>(CASINO_011DA9AC);
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
            e.set_global(CASINO_DATA_011DA9A8, data);
        });
    }
    let player = e.global::<u32>(PLAYER_011DEA3C);
    if e.call(PLAYER_00969B80, &args![player]).bool() {
        e.with_stack(0x104, |e, buffer| {
            let player = e.global::<u32>(PLAYER_011DEA3C);
            let count = e.call(PLAYER_00969B40, &args![player]).u32();
            let setting = e.call(SETTING_STRING, &args![0x011d_2ac0u32]).u32();
            e.call(
                SPRINTF_S,
                &[buffer.addr(), 0x104, BANNED_FORMAT_01070C58, setting, count],
            );
            show_message(e, buffer.addr(), MESSAGE_ICON_01049638, 0);
        });
        play_sound(e, SOUND_REFUSE_01025CC4);
        return 0;
    }
    let casino = e.global::<u32>(CASINO_011DA9AC);
    let limits = e.call(CASINO_LIMITS_00504E60, &args![casino]).u32();
    let data = e.global::<u32>(CASINO_DATA_011DA9A8);
    if e.mem.u32(data.wrapping_add(4)) as i32 >= e.mem.u32(limits.wrapping_add(0x28)) as i32 {
        let text = e.call(SETTING_STRING, &args![0x011d_3abcu32]).u32();
        refuse_with(e, text);
        return 0;
    }
    let casino = e.global::<u32>(CASINO_011DA9AC);
    let item = e.call(CASINO_ITEM_007316D0, &args![casino]).u32();
    let player = e.global::<u32>(PLAYER_011DEA3C);
    if e.call(GET_ITEM_COUNT_IN_CONTAINER, &args![player, item])
        .i32()
        < e.global::<u32>(MIN_BET_011A082C) as i32
    {
        let text = e.call(SETTING_STRING, &args![0x011d_2424u32]).u32();
        refuse_with(e, text);
        return 0;
    }
    let data = e.global::<u32>(CASINO_DATA_011DA9A8);
    if (e.mem.u32(data.wrapping_add(4)) as i32) < needed && needed != 0 {
        let text = e.call(SETTING_STRING, &args![0x011d_3594u32]).u32();
        refuse_with(e, text);
        return 0;
    }
    if existing != 0 {
        e.vcall(existing, 0, &args![1u32]);
    }
    let root = e.call(WORD_AT_9C_00586150, &args![interface]).u32();
    let tile = e.call(TILE_READ_FILE, &args![root, MENU_FILE_TEXT]).u32();
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    e.set_global(SLOT_MENU_011DA9A4, menu);
    let created = slot_menu(e) != 0
        && {
            let menu = slot_menu(e);
            let class = e.vcall(menu, 0x34, &args![]).u32();
            class == fn_007c1440(e)
        }
        && e.global::<u32>(CASINO_011DA9AC) != 0;
    if !created {
        let menu = slot_menu(e);
        if menu != 0 && e.call(WORD_AT_4_00726070, &args![menu]).u32() != 0 {
            let menu = slot_menu(e);
            e.vcall(menu, 0, &args![1u32]);
        }
        return 0;
    }
    let menu = slot_menu(e);
    e.call(MENU_SET_MENU_TILE, &args![menu, tile, 0u32]);
    let me = Ptr::<SlotMachineMenu>::new(slot_menu(e));
    if !fn_007c1450(e, me) {
        e.call(DEBUG_MESSAGE, &args![CREATION_FAILED_TEXT]);
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
    play_sound(e, SOUND_OPEN_01076600);
    let player = e.global::<u32>(PLAYER_011DEA3C);
    let luck = e
        .call(
            GET_CLAMPED_ACTOR_VALUE,
            &args![player.wrapping_add(0xa4), 0xbu32],
        )
        .i32();
    let me = Ptr::<SlotMachineMenu>::new(slot_menu(e));
    e.set(me, SlotMachineMenu::iLuckSkill, luck);
    let minimum = e.global::<u32>(MIN_BET_011A082C) as i32;
    e.set(me, SlotMachineMenu::iCurrentBet, minimum);
    let text = e.call(SETTING_STRING, &args![0x011d_3804u32]).u32();
    let trait_id = e.global::<u32>(TRAIT_011DA99C);
    let target = e.mem.u32(slot_menu(e) + TILES);
    e.call(TILE_SET_STRING, &args![target, trait_id, text, 1u32]);
    let minimum = e.global::<u32>(MIN_BET_011A082C);
    show_number(e, TILES, minimum);
    let text = e.call(SETTING_STRING, &args![0x011d_3300u32]).u32();
    let trait_id = e.global::<u32>(TRAIT_011DA99C);
    let target = e.mem.u32(slot_menu(e) + TILES + 4);
    e.call(TILE_SET_STRING, &args![target, trait_id, text, 1u32]);
    let trait_id = e.global::<u32>(TRAIT_011DA998);
    let target = e.mem.u32(slot_menu(e) + TILES + 4);
    e.call(TILE_POKE, &args![target, trait_id, 1.0f32.to_bits()]);
    let casino = e.global::<u32>(CASINO_011DA9AC);
    let item = e.call(CASINO_ITEM_007316D0, &args![casino]).u32();
    let player = e.global::<u32>(PLAYER_011DEA3C);
    let chips = e
        .call(GET_ITEM_COUNT_IN_CONTAINER, &args![player, item])
        .u32();
    let trait_id = e.global::<u32>(TRAIT_011DA998);
    let target = e.mem.u32(slot_menu(e) + TILES + 4);
    e.call(TILE_SET_INT, &args![target, trait_id, chips]);
    e.with_stack(0x40, |e, title| {
        let casino = e.global::<u32>(CASINO_011DA9AC);
        let name = e
            .call(NAME_CHARS_00408DA0, &args![casino.wrapping_add(0x18)])
            .u32();
        if e.call(STRLEN, &args![name]).u32() != 0 {
            let setting = e.call(SETTING_STRING, &args![SETTING_011D3E08]).u32();
            let casino = e.global::<u32>(CASINO_011DA9AC);
            let name = e
                .call(NAME_CHARS_00408DA0, &args![casino.wrapping_add(0x18)])
                .u32();
            e.call(
                SPRINTF_S,
                &[title.addr(), 0x40, NAME_FORMAT_01012058, name, setting],
            );
        } else {
            let casino = e.global::<u32>(CASINO_011DA9AC);
            let name = e.vcall(casino, 0x130, &args![]).u32();
            e.call(
                SPRINTF_S,
                &[title.addr(), 0x40, UNNAMED_FORMAT_010765F0, name],
            );
        }
        let trait_id = e.global::<u32>(TRAIT_011DA99C);
        let target = e.mem.u32(slot_menu(e) + TILES + 8);
        e.call(TILE_SET_STRING, &args![target, trait_id, title, 1u32]);
    });
    let target = e.mem.u32(slot_menu(e) + TILES + 8);
    let child = e
        .call(TILE_CHILD_BY_NAME_00A08B20, &args![target, CHIP_CHILD_NAME])
        .u32();
    e.call(TILE_SET_INT, &args![child, 0xfa3u32, 0u32]);
    let me = Ptr::<SlotMachineMenu>::new(slot_menu(e));
    fn_007c5e60(e, me);
    for (offset, setting) in [
        (0xacu32, 0x011d_2fd0u32),
        (0xb0, 0x011d_3354),
        (0xb4, 0x011d_487c),
        (0xb8, 0x011d_4054),
        (0xbc, 0x011d_37f8),
        (0xc4, 0x011d_387c),
    ] {
        let text = e.call(SETTING_STRING, &args![setting]).u32();
        let target = e.mem.u32(slot_menu(e) + offset);
        e.call(TILE_SET_STRING, &args![target, 0xfc4u32, text, 1u32]);
    }
    if e.mem.u32(slot_menu(e) + 0xc0) != 0 {
        let text = e.call(SETTING_STRING, &args![0x011d_33fcu32]).u32();
        let target = e.mem.u32(slot_menu(e) + 0xc0);
        e.call(TILE_SET_STRING, &args![target, 0xfc4u32, text, 1u32]);
    }
    let me = Ptr::<SlotMachineMenu>::new(slot_menu(e));
    if !slot_machine_menu_initialize_casino_data(e, me) {
        fn_007c2420(e);
        return 0;
    }
    let me = Ptr::<SlotMachineMenu>::new(slot_menu(e));
    slot_machine_menu_prepare_3d_elements(e, me);
    let menu = slot_menu(e);
    e.call(MENU_PREP_FOR_VISIBILITY, &args![menu, 0u32]);
    tile
}

// Translated from 007c1440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): 0x438, the slot machine menu's class.
pub fn fn_007c1440(_e: &mut Engine) -> u32 {
    0x438
}

// Translated from 007c1450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the slot machine menu): whether all ten `pTiles` are set.
pub fn fn_007c1450(e: &mut Engine, this: Ptr<SlotMachineMenu>) -> bool {
    (0..0xau32).all(|index| e.mem.u32(this.addr() + TILES + 4 * index) != 0)
}

// Translated from 007c1490 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007c1490 - the /GS cookie check (00ec408c) is not translated.
/// `SlotMachineMenu::InitializeCasinoData` (Xbox PDB; `thiscall`): false without a casino.
/// Otherwise the seven reel textures (the casino's texture `i` as `"%s%s"`-like `0101996c`
/// with the folder `0101d7fc`, through `SwapPlatformLanguageTexturePath`) and the blank one
/// (`010766b8` with `010766d8`, `01063dd4`) are made (`TES::CreateTextureImage` on the
/// object at `011dea10`); the machine model (the casino's model 7) and the three reels are
/// loaded; `bRunScriptOnExit` is cleared; returns `SetupActiveReels`.
pub fn slot_machine_menu_initialize_casino_data(
    e: &mut Engine,
    this: Ptr<SlotMachineMenu>,
) -> bool {
    let base = this.addr();
    if e.global::<u32>(CASINO_011DA9AC) == 0 {
        return false;
    }
    e.with_stack(0x208, |e, frame| {
        let path = frame.addr();
        let swapped = frame.addr() + 0x108;
        for index in 0..7u32 {
            let casino = e.global::<u32>(CASINO_011DA9AC);
            if casino == 0 {
                continue;
            }
            let texture = e.call(CASINO_TEXTURE_00504E20, &args![casino, index]).u32();
            let name = e.call(NAME_CHARS_00408DA0, &args![texture]).u32();
            e.call(
                SPRINTF_S,
                &[
                    path,
                    0x104,
                    TEXTURE_FORMAT_0101996C,
                    TEXTURE_FOLDER_0101D7FC,
                    name,
                ],
            );
            e.call(SWAP_TEXTURE_PATH, &args![path, swapped, 0x104u32]);
            let maker = e.global::<u32>(TEXTURES_011DEA10);
            e.call(
                CREATE_TEXTURE_IMAGE,
                &args![maker, swapped, base + REEL_TEXTURES + 4 * index, 0u32, 0u32],
            );
        }
        e.call(
            SPRINTF_S,
            &[
                path,
                0x104,
                BLANK_FORMAT_010766B8,
                BLANK_FOLDER_010766D8,
                BLANK_NAME_01063DD4,
            ],
        );
        e.call(SWAP_TEXTURE_PATH, &args![path, swapped, 0x104u32]);
        let maker = e.global::<u32>(TEXTURES_011DEA10);
        e.call(
            CREATE_TEXTURE_IMAGE,
            &args![maker, swapped, base + 0x130, 0u32, 0u32],
        );
    });
    let path = casino_model_path(e, 7);
    let machine = load_model(e, path);
    e.set(this, SlotMachineMenu::pSlotMachineGeometry, machine);
    for (reel, path) in [
        REEL_MODEL_PATH_0107659C,
        REEL_MODEL_PATH_01076560,
        REEL_MODEL_PATH_01076524,
    ]
    .into_iter()
    .enumerate()
    {
        let geometry = load_model(e, path);
        e.mem
            .set_u32(base + REEL_GEOMETRY + 4 * reel as u32, geometry);
    }
    e.set(this, SlotMachineMenu::bRunScriptOnExit, false);
    slot_machine_menu_setup_active_reels(e, this)
}

// Translated from 007c16b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007c16b0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the sound handle) and the /GS cookie check (00ec408c) are not translated.
/// `SlotMachineMenu::SetupActiveReels` (Xbox PDB; `thiscall`). The 28 active symbols start
/// at 0xe; the seven symbol counts are the casino data's words +8 to +0x20 (`00504e60`,
/// read one by one); `iNumActiveSymbols` is 7 less one for each zero count. While fewer
/// than 28 symbols are placed, every symbol with a count left adds its two faces (2k and
/// 2k + 1) and uses up one; a pass that adds none shows the message `01076700`, plays the
/// refusal sound, closes (`007c2420`) and answers false. The writes go on past the 28th
/// word when a pass ends beyond it, as the code does. The reels then show faces 0, 2, 4
/// from start 0xe; true.
pub fn slot_machine_menu_setup_active_reels(e: &mut Engine, this: Ptr<SlotMachineMenu>) -> bool {
    let base = this.addr();
    let mut placed = 0u32;
    for index in 0..0x1cu32 {
        e.mem.set_u32(base + ACTIVE_SYMBOLS + 4 * index, 0xe);
    }
    let mut counts = [0i32; 7];
    for (index, count) in counts.iter_mut().enumerate() {
        let casino = e.global::<u32>(CASINO_011DA9AC);
        let limits = e.call(CASINO_LIMITS_00504E60, &args![casino]).u32();
        *count = e.mem.u32(limits.wrapping_add(8 + 4 * index as u32)) as i32;
    }
    e.set(this, SlotMachineMenu::iNumActiveSymbols, 7);
    for count in counts {
        if count == 0 {
            let active = e.get(this, SlotMachineMenu::iNumActiveSymbols) - 1;
            e.set(this, SlotMachineMenu::iNumActiveSymbols, active);
        }
    }
    while (placed as i32) < 0x1c {
        let start = placed;
        let mut symbol = 0u32;
        for count in counts.iter_mut() {
            if *count != 0 {
                e.mem.set_u32(base + ACTIVE_SYMBOLS + 4 * placed, symbol);
                placed += 1;
                symbol += 1;
                e.mem.set_u32(base + ACTIVE_SYMBOLS + 4 * placed, symbol);
                placed += 1;
                *count -= 1;
            }
            symbol += 1;
        }
        if start == placed {
            e.with_stack(0x104, |e, buffer| {
                e.call(
                    SPRINTF_S,
                    &args![buffer, 0x104u32, NO_SYMBOLS_MESSAGE_01076700],
                );
                show_message(e, buffer.addr(), MESSAGE_ICON_01049638, 0);
            });
            play_sound(e, SOUND_REFUSE_01025CC4);
            fn_007c2420(e);
            return false;
        }
    }
    for reel in 0..3u32 {
        e.mem.set_u32(base + REEL_RESULTS + 4 * reel, reel * 2);
        e.mem.set_u32(base + REEL_START + 4 * reel, 0xe);
    }
    true
}

// Translated from 007c19c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007c19c0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the NiPointer local) is not translated.
/// Unnamed (no arguments): the slot machine menu's render function, as the roulette
/// menu's (`007bbcf0`) with this menu's camera (`011da9d0`).
pub fn fn_007c19c0(e: &mut Engine) {
    let object = e.call(RENDER_0043C4B0, &args![]).u32();
    e.call(RENDER_007148C0, &args![object, 0u32, 4u32]);
    let source = e.call(RENDER_0045C670, &args![]).u32();
    let accumulator = e.call(RENDER_008D80E0, &args![source]).u32();
    e.call(RENDER_00C4F270, &args![accumulator, 1u32]);
    let camera = e.call(WORD_AT_0_00559450, &args![CAMERA_011DA9D0]).u32();
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

/// Looks up an animation sequence by name (`0047a520` with a fixed string local).
fn sequence_by_name(e: &mut Engine, manager: u32, name: u32) -> u32 {
    e.with_stack(4, |e, fixed| {
        let made = e.call(FIXED_STRING_CONSTRUCT, &args![fixed, name]).u32();
        let sequence = e
            .call(SEQUENCE_BY_NAME_0047A520, &args![manager, made])
            .u32();
        e.call(FIXED_STRING_DESTROY, &args![fixed]);
        sequence
    })
}

// Translated from 007c1af0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007c1af0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 5) is not translated.
/// `SlotMachineMenu::Prepare3DElements` (Xbox PDB; `thiscall`). The model node (an
/// `NiNode` of 0xac bytes) takes the machine and the three reels; it is rotated by the
/// product of an x rotation and a rotation by the same angle (`0101ff38`), placed at the
/// `float`s at `0105478c` and `0107678c` (z 0.0) and then given the origin translation. With
/// the machine's controller manager its lever, coin and two payout list sequences are
/// found, all deactivated and the lever started; each reel's controller manager gives its
/// sequence (by the lever's name), deactivated and started. The camera is made
/// (`007c2180`), the model attached to the scene and lit (the bound radius times the
/// `double` at `01021928`), and the reel faces set (`007c4d70`, `007c40b0`).
pub fn slot_machine_menu_prepare_3d_elements(e: &mut Engine, this: Ptr<SlotMachineMenu>) {
    let base = this.addr();
    let block = e.call(NI_ALLOCATE, &args![0xacu32]).u32();
    let node = if block != 0 {
        e.call(NI_NODE_CONSTRUCT, &args![block, 0u32]).u32()
    } else {
        0
    };
    e.call(NI_POINTER_ASSIGN, &args![MODEL_011DA9FC, node]);
    for offset in [0x104u32, 0x108, 0x10c, 0x110] {
        let node = model(e);
        let child = e.mem.u32(base + offset);
        e.vcall(node, 0xdc, &args![child, 1u32]);
    }
    e.with_stack(0x6c, |e, frame| {
        let first = frame.addr();
        let second = frame.addr() + 0x24;
        let product = frame.addr() + 0x48;
        e.call(NODE_SELF, &args![first]);
        e.call(NODE_SELF, &args![second]);
        let angle = x87_quiet_bits(e.global::<u32>(MODEL_ROTATION_0101FF38));
        e.call(MAKE_X_ROTATION, &args![first, angle]);
        let angle = x87_quiet_bits(e.global::<u32>(MODEL_ROTATION_0101FF38));
        e.call(ROTATION_004A0C90, &args![second, angle]);
        let made = e
            .call(MATRIX_MULTIPLY, &args![first, product, second])
            .u32();
        let node = model(e);
        e.call(SET_ROTATE_0043FA80, &args![node, made]);
    });
    let x = x87_quiet_bits(e.global::<u32>(MODEL_X_0105478C));
    let y = x87_quiet_bits(e.global::<u32>(MODEL_Y_0107678C));
    let node = model(e);
    e.call(SET_TRANSLATE_004BC1F0, &args![node, x, y, 0.0f32.to_bits()]);
    reset_model_translate(e);
    let machine = e.get(this, SlotMachineMenu::pSlotMachineGeometry);
    let manager = e
        .call(GET_CONTROLLER, &args![machine, CONTROLLER_RTTI_011F36AC])
        .u32();
    if manager != 0 {
        e.set(this, SlotMachineMenu::pSlotAnimManager, manager);
        for (offset, name) in [
            (0x138u32, LEVER_SEQUENCE_01048458),
            (0x13c, COIN_SEQUENCE_0104844C),
            (0x140, PAYOUT_SEQUENCE_01076780),
            (0x144, PAYOUT_SEQUENCE_01076774),
        ] {
            let manager = e.get(this, SlotMachineMenu::pSlotAnimManager);
            let sequence = sequence_by_name(e, manager, name);
            e.mem.set_u32(base + offset, sequence);
        }
        let manager = e.get(this, SlotMachineMenu::pSlotAnimManager);
        e.call(DEACTIVATE_ALL, &args![manager, 0.0f32.to_bits()]);
        let lever = e.get(this, SlotMachineMenu::pSlotLeverAnim);
        if lever != 0 {
            activate_on(e, manager, lever);
        }
        let machine = e.get(this, SlotMachineMenu::pSlotMachineGeometry);
        e.call(START_ANIMATIONS, &args![machine]);
    }
    let mut managers = [0u32; 3];
    for (reel, manager) in managers.iter_mut().enumerate() {
        let geometry = e.mem.u32(base + REEL_GEOMETRY + 4 * reel as u32);
        *manager = e
            .call(GET_CONTROLLER, &args![geometry, CONTROLLER_RTTI_011F36AC])
            .u32();
    }
    for (reel, manager) in managers.into_iter().enumerate() {
        let reel = reel as u32;
        if manager == 0 {
            continue;
        }
        let sequence = sequence_by_name(e, manager, LEVER_SEQUENCE_01048458);
        e.mem.set_u32(base + REEL_ANIMATIONS + 4 * reel, sequence);
        e.call(DEACTIVATE_ALL, &args![manager, 0.0f32.to_bits()]);
        let sequence = e.mem.u32(base + REEL_ANIMATIONS + 4 * reel);
        if sequence != 0 {
            activate_on(e, manager, sequence);
        }
        let geometry = e.mem.u32(base + REEL_GEOMETRY + 4 * reel);
        e.call(START_ANIMATIONS, &args![geometry]);
    }
    reset_model_translate(e);
    let camera = fn_007c2180(e, this.cast());
    e.call(NI_POINTER_ASSIGN, &args![CAMERA_011DA9D0, camera]);
    let scene = e.call(SCENE_00450B80, &args![1u32]).u32();
    let node = model(e);
    e.vcall(scene, 0xdc, &args![node, 1u32]);
    let node = model(e);
    let bound = e.call(GET_WORLD_BOUND, &args![node]).u32();
    let radius = e.call(BOUND_RADIUS_0084D030, &args![bound]).f64();
    let scale: f64 = e.global(LIGHT_SCALE_01021928);
    let radius = x87_store32(x87_mul(radius, scale));
    let node = model(e);
    let scene = e.call(SCENE_00450B80, &args![1u32]).u32();
    e.call(ADD_ALL_LIGHTS_FROM_NODE, &args![scene, node, radius]);
    fn_007c4d70(e, this);
    fn_007c40b0(e, this);
}

/// `ActivateSequence(sequence, 0, 0, 1.0, 0.0, 0)` on a given manager.
fn activate_on(e: &mut Engine, manager: u32, sequence: u32) {
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

// Translated from 007c2180 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007c2180 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1) is not translated.
/// Unnamed (`thiscall` on a casino menu; the roulette menu calls it too): makes the menu
/// camera. A camera of 0x114 bytes (`00a712f0`) gets `00712e60` of a local built by
/// `00414430(0.0, 1.0, 1.0, 0.0)`; a frustum local (`NiFrustum(false)`) takes the far
/// value (vtable +0x100 of `0045c670()`), the near value (the setting at `01203144`), and
/// the sides: the field of view (the setting at `01203150` times the `double`s at
/// `01023128` and `01070c80`) through `0057dd70`, times the `float` at `01016264` (left
/// negated), and for top and bottom also times the screen's aspect (the width word, or
/// `004dc200`, over `004dc1f0`); the camera gets `00507700(far / near)`, the frustum, looks
/// along (0, 0, 1) from (0, 0, 1) (`LookAtWorldPoint`), `00733fd0(1.0)`, the origin
/// translation and the name `01070c70`. Returns the camera.
pub fn fn_007c2180(e: &mut Engine, _this: Ptr) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![0x114u32]).u32();
    let camera = if block != 0 {
        e.call(CAMERA_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    // The locals from `EBP - 0x94`: the `double` (+0), the frustum (+0x48: left, right,
    // top, bottom, near, far, the orthographic byte), the `00414430` local (+0x6c), the
    // points (+0x20, +0x2c, +0x7c) and the fixed string (+0x1c).
    e.with_stack(0x94, |e, frame| {
        let at = |offset: i32| frame.addr().wrapping_add((0x94 + offset) as u32);
        e.call(
            CAMERA_00414430,
            &args![
                at(-0x28),
                0.0f32.to_bits(),
                1.0f32.to_bits(),
                1.0f32.to_bits(),
                0.0f32.to_bits()
            ],
        );
        e.call(CAMERA_00712E60, &args![camera, at(-0x28)]);
        e.call(FRUSTUM_CONSTRUCT, &args![at(-0x4c), 0u32]);
        let object = e.call(RENDER_0045C670, &args![]).u32();
        let far = e.vcall(object, 0x100, &args![]).f64();
        e.mem.set_u32(at(-0x38), x87_store32(far));
        let near = e
            .call(SETTING_VALUE_00403E20, &args![NEAR_SETTING_01203144])
            .u32();
        let near = x87_quiet_bits(e.mem.u32(near));
        e.mem.set_u32(at(-0x3c), near);
        let ratio = x87_store32(x87_div(
            x87_load32(e.mem.u32(at(-0x38))),
            x87_m32(e.mem.u32(at(-0x3c))),
        ));
        e.call(CAMERA_00507700, &args![camera, x87_quiet_bits(ratio)]);
        let width = if e.global::<u8>(WINDOWED_011F9426) != 0 {
            e.global::<u32>(WIDTH_011F9434) as i32
        } else {
            e.call(SCREEN_004DC200, &args![]).i32()
        };
        let height = e.call(SCREEN_004DC1F0, &args![]).i32();
        let aspect = x87_store32(x87_div(f64::from(width), f64::from(height)));
        let scale = x87_quiet_bits(e.global::<u32>(CAMERA_SCALE_01016264));
        let fov = e
            .call(SETTING_VALUE_00403E20, &args![FOV_SETTING_01203150])
            .u32();
        let degrees: f64 = e.global(CAMERA_DEGREES_01023128);
        let half: f64 = e.global(CAMERA_HALF_01070C80);
        let fov = x87_store32(x87_mul(x87_mul(x87_load32(e.mem.u32(fov)), degrees), half));
        let tangent = |e: &mut Engine| e.call(TAN_0057DD70, &args![x87_quiet_bits(fov)]).f64();
        let value = tangent(e);
        let left = x87_store32(x87_mul(-value, x87_m32(scale)));
        e.mem.set_u32(at(-0x4c), left);
        let value = tangent(e);
        let right = x87_store32(x87_mul(value, x87_m32(scale)));
        e.mem.set_u32(at(-0x48), right);
        let value = tangent(e);
        let bottom = x87_store32(x87_mul(x87_mul(-value, x87_m32(aspect)), x87_m32(scale)));
        e.mem.set_u32(at(-0x40), bottom);
        let value = tangent(e);
        let top = x87_store32(x87_mul(x87_mul(value, x87_m32(aspect)), x87_m32(scale)));
        e.mem.set_u32(at(-0x44), top);
        e.mem.set_u8(at(-0x34), 0);
        e.call(SET_VIEW_FRUSTUM, &args![camera, at(-0x4c)]);
        let up = e
            .call(
                POINT_00416870,
                &args![
                    at(-0x68),
                    0.0f32.to_bits(),
                    0.0f32.to_bits(),
                    1.0f32.to_bits()
                ],
            )
            .u32();
        let target = e
            .call(
                POINT_00416870,
                &args![
                    at(-0x74),
                    0.0f32.to_bits(),
                    0.0f32.to_bits(),
                    1.0f32.to_bits()
                ],
            )
            .u32();
        e.call(LOOK_AT_WORLD_POINT, &args![camera, target, up]);
        e.call(CAMERA_00733FD0, &args![camera, 1.0f32.to_bits()]);
        e.call(
            POINT_CONSTRUCT_0043D410,
            &args![at(-0x18), 0.0f32.to_bits(), 0u32, 0u32],
        );
        e.call(SET_TRANSLATE_00A59C60, &args![camera, at(-0x18)]);
        let name = e
            .call(
                FIXED_STRING_CONSTRUCT,
                &args![at(-0x78), CAMERA_NAME_01070C70],
            )
            .u32();
        e.call(SET_NAME_00A5B950, &args![camera, name]);
        e.call(FIXED_STRING_DESTROY, &args![at(-0x78)]);
    });
    camera
}

// Translated from 007c2420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments; the slot machine menu's close): with an open menu its state
/// becomes 7 and the cursor tile's `0xfa9` the `float` at `01023cd8`.
pub fn fn_007c2420(e: &mut Engine) {
    let menu = slot_menu(e);
    if menu == 0 {
        return;
    }
    e.set(
        Ptr::<SlotMachineMenu>::new(menu),
        SlotMachineMenu::eSlotState,
        7,
    );
    let fade = x87_quiet_bits(e.global::<u32>(CURSOR_FADE_01023CD8));
    set_cursor_fade(e, fade);
}

/// Plays the bet sound `iNextSoundIndex` names (`010767a8`) and advances it (1 to 3).
fn play_bet_sound(e: &mut Engine, this: Ptr<SlotMachineMenu>) {
    e.with_stack(0x104, |e, name| {
        let index = e.get(this, SlotMachineMenu::iNextSoundIndex) as u32;
        e.call(
            SPRINTF_S,
            &[name.addr(), 0x104, BET_SOUND_FORMAT_010767A8, index],
        );
        play_sound(e, name.addr());
    });
    let next = e
        .get(this, SlotMachineMenu::iNextSoundIndex)
        .wrapping_add(1);
    let next = if next > 3 { 1 } else { next };
    e.set(this, SlotMachineMenu::iNextSoundIndex, next);
}

// Translated from 007c2460 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007c2460 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2) and the /GS cookie check (00ec408c) are not translated.
/// `SlotMachineMenu::DoClick` (engine map, vtable; `RET 8`: a tile id and an unread tile).
/// Outside the betting state (6) only state 4 moves on to 5. Id 3 spins when the chips
/// cover the bet (the sound index reset, the reels drawn (`007c56e0`), the luck test
/// `007c58f0`, the result interpreted, state 1, misc stat 0x2a); id 4 raises the bet by its
/// step (within the chips and the maximum) and id 7 sets it to the maximum (within the
/// chips), both with the next bet sound when it changed; id 5 lowers it (at least the
/// minimum) with sound `01076790`; id 6 goes to state 4; id 8 closes. The bet tile shows
/// the bet.
pub fn slot_machine_menu_do_click(
    e: &mut Engine,
    this: Ptr<SlotMachineMenu>,
    id: u32,
    _unused_2: u32,
) {
    if e.get(this, SlotMachineMenu::eSlotState) != 6 {
        if e.get(this, SlotMachineMenu::eSlotState) == 4 {
            e.set(this, SlotMachineMenu::eSlotState, 5);
        }
        return;
    }
    let max_bet = |e: &Engine| e.global::<u32>(MAX_BET_011A0828) as i32;
    match id.wrapping_sub(3) {
        0 => {
            if e.get(this, SlotMachineMenu::iChipCount) < e.get(this, SlotMachineMenu::iCurrentBet)
            {
                return;
            }
            e.set(this, SlotMachineMenu::iNextSoundIndex, 1);
            let drawn = fn_007c56e0(e, this);
            e.set(this, SlotMachineMenu::eSpinResult, drawn as i32);
            let result = e.get(this, SlotMachineMenu::eSpinResult) as u32;
            let lucky = e.call(LUCK_007C58F0, &args![this, result]).bool();
            e.set(this, SlotMachineMenu::bLucky, lucky);
            let result = slot_machine_menu_interpret_spin(e, this);
            e.set(this, SlotMachineMenu::eSpinResult, result as i32);
            e.set(this, SlotMachineMenu::eSlotState, 1);
            e.call(MISC_STAT_INCREMENT, &args![0x2au32]);
        }
        1 => {
            let previous = e.get(this, SlotMachineMenu::iCurrentBet);
            let bet = e.get(this, SlotMachineMenu::iCurrentBet);
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
            e.set(this, SlotMachineMenu::iCurrentBet, bet.wrapping_add(step));
            if e.get(this, SlotMachineMenu::iCurrentBet) > e.get(this, SlotMachineMenu::iChipCount)
            {
                let chips = e.get(this, SlotMachineMenu::iChipCount);
                e.set(this, SlotMachineMenu::iCurrentBet, chips);
            }
            if e.get(this, SlotMachineMenu::iCurrentBet) > max_bet(e) {
                let maximum = max_bet(e);
                e.set(this, SlotMachineMenu::iCurrentBet, maximum);
            }
            let bet = e.get(this, SlotMachineMenu::iCurrentBet) as u32;
            show_number(e, TILES, bet);
            if previous != e.get(this, SlotMachineMenu::iCurrentBet) {
                play_bet_sound(e, this);
            }
        }
        2 => {
            let previous = e.get(this, SlotMachineMenu::iCurrentBet);
            let bet = e.get(this, SlotMachineMenu::iCurrentBet);
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
            e.set(this, SlotMachineMenu::iCurrentBet, lowered);
            let minimum = e.global::<u32>(MIN_BET_011A082C) as i32;
            if e.get(this, SlotMachineMenu::iCurrentBet) < minimum {
                e.set(this, SlotMachineMenu::iCurrentBet, minimum);
            }
            let bet = e.get(this, SlotMachineMenu::iCurrentBet) as u32;
            show_number(e, TILES, bet);
            if previous != e.get(this, SlotMachineMenu::iCurrentBet) {
                play_sound(e, SOUND_LOWER_01076790);
                e.set(this, SlotMachineMenu::iNextSoundIndex, 1);
            }
        }
        3 => e.set(this, SlotMachineMenu::eSlotState, 4),
        4 => {
            if e.get(this, SlotMachineMenu::iCurrentBet) == max_bet(e) {
                return;
            }
            let maximum = max_bet(e);
            e.set(this, SlotMachineMenu::iCurrentBet, maximum);
            if e.get(this, SlotMachineMenu::iCurrentBet) > e.get(this, SlotMachineMenu::iChipCount)
            {
                let chips = e.get(this, SlotMachineMenu::iChipCount);
                e.set(this, SlotMachineMenu::iCurrentBet, chips);
            }
            let bet = e.get(this, SlotMachineMenu::iCurrentBet) as u32;
            show_number(e, TILES, bet);
            play_bet_sound(e, this);
        }
        5 => fn_007c2420(e),
        _ => {}
    }
}

// Translated from 007c2c70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007c2c70 - the C++ exception frame (FS:[0] chain, unwind states 0 to 8) and the /GS cookie check (00ec408c) are not translated.
/// `SlotMachineMenu::DoIdle` (engine map, vtable; `thiscall`). With the menu on top the
/// elapsed time (`011daa30`) advances by the milliseconds since the last idle over 1000
/// (cleared when the state changed, and with the last state on the first idle); off top
/// the cursor tile's `0xfa9` is the `float` at `01023cd8`. By state: 0 goes to betting (6);
/// 1 (spinning) stops the coin, hides the message after a second, turns the machine and
/// the reels with the time (the spin sound and texture swap at the start when chips are
/// shown), swaps the reel faces (`007c4d70`) once the lever has run and goes to 2 with the
/// new faces (`007c40b0`) once the reels have too; 2 pays the result (`InterpretSpin`:
/// three of a kind by symbol, two or one wild, else the bet lost), updates the chips, the
/// casino data and the chip text, shows the message and goes to 3 for a win, else 6; 3
/// plays the coin animation and its win sound, then the casino's payout levels (setting
/// `bRunScriptOnExit` and closing) or betting; 4 and 5 run the two payout list
/// animations; 6 (betting) hides the message after a second, lowers the bet to 1 when the
/// chips run short and closes with a message when no chips are left; 7 (closing) settles
/// the chips as the roulette menu does and runs the end script.
pub fn slot_machine_menu_do_idle(e: &mut Engine, this: Ptr<SlotMachineMenu>) {
    let base = this.addr();
    let class = e.vcall(base, 0x34, &args![]).u32();
    let on_top = e.call(INTERFACE_IS_TOP_MENU_ID, &args![class]).bool();
    if e.get(this, SlotMachineMenu::uiPrevIdleTick) == 0 {
        e.set_global(SPIN_TIME_011DAA30, 0.0f32.to_bits());
        e.set_global(LAST_STATE_011DAA2C, 0u32);
    } else if on_top {
        let now = e.call(WORD_AT_14_00825C00, &args![TIMER_011F6394]).u32();
        let elapsed = now.wrapping_sub(e.get(this, SlotMachineMenu::uiPrevIdleTick)) as i32;
        if e.global::<u32>(LAST_STATE_011DAA2C) as i32 != e.get(this, SlotMachineMenu::eSlotState) {
            e.set_global(SPIN_TIME_011DAA30, 0.0f32.to_bits());
        } else {
            let thousand: f64 = e.global(THOUSAND_DOUBLE);
            let time = x87_add(
                x87_div(f64::from(elapsed), thousand),
                x87_m32(e.global::<u32>(SPIN_TIME_011DAA30)),
            );
            e.set_global(SPIN_TIME_011DAA30, x87_store32(time));
        }
    }
    let now = e.call(WORD_AT_14_00825C00, &args![TIMER_011F6394]).u32();
    e.set(this, SlotMachineMenu::uiPrevIdleTick, now);
    let state = e.get(this, SlotMachineMenu::eSlotState);
    e.set_global(LAST_STATE_011DAA2C, state as u32);
    if !on_top {
        let fade = x87_quiet_bits(e.global::<u32>(CURSOR_FADE_01023CD8));
        set_cursor_fade(e, fade);
    }
    match state {
        0 => e.set(this, SlotMachineMenu::eSlotState, 6),
        1 => idle_spinning(e, this),
        2 => idle_paying(e, this),
        3 => idle_coins(e, this),
        4 => idle_payout_list(e, this, PAYOUT_ANIMATIONS, true),
        5 => idle_payout_list(e, this, PAYOUT_ANIMATIONS + 4, false),
        6 => idle_betting(e, this),
        7 => idle_closing(e, this),
        _ => {}
    }
}

/// Hides the message tile (+0xc4) once the elapsed time passed 1.0 (strictly).
fn hide_message_after_a_second(e: &mut Engine, this: Ptr<SlotMachineMenu>) {
    let one: f64 = e.global(ONE_DOUBLE);
    if spin_time(e) > one {
        let message = e.mem.u32(this.addr() + 0xc4);
        e.call(TILE_SET_INT, &args![message, 0xfa3u32, 0u32]);
    }
}

/// `DoIdle` while spinning (state 1); see there.
fn idle_spinning(e: &mut Engine, this: Ptr<SlotMachineMenu>) {
    let base = this.addr();
    if e.get(this, SlotMachineMenu::pSlotLeverAnim) == 0 || e.mem.u32(base + REEL_ANIMATIONS) == 0 {
        return;
    }
    let coin = e.get(this, SlotMachineMenu::pSlotCoinAnim);
    if coin != 0 {
        deactivate(e, this, coin);
    }
    hide_message_after_a_second(e, this);
    e.with_stack(0x30, |e, points| {
        let time = x87_quiet_bits(e.global::<u32>(SPIN_TIME_011DAA30));
        for index in 0..4u32 {
            e.call(
                POINT_CONSTRUCT_0043D410,
                &args![points.addr() + 0xc * index, time, 1u32, 0u32],
            );
        }
        let machine = model_child(e, 0);
        e.call(SET_TRANSLATE_00A59C60, &args![machine, points]);
        let zero: f64 = e.global(ZERO_DOUBLE);
        // `TEST AH,0x44; JP`: at exactly 0.0, with chips shown (above 0.0), the spin starts.
        if spin_time(e) == zero {
            let trait_id = e.global::<u32>(TRAIT_011DA998);
            let chips = e.mem.u32(base + TILES + 4);
            let shown = e.call(TILE_GET_FLOAT, &args![chips, trait_id]).f64();
            if shown > zero {
                play_sound(e, SOUND_SPIN_01076808);
                e.set(this, SlotMachineMenu::bSwapTextures, true);
            }
        }
        // `TEST AH,0x44; JNP`: the reels move at exactly 0.0, or with chips shown.
        let moves = spin_time(e) == zero || {
            let trait_id = e.global::<u32>(TRAIT_011DA998);
            let chips = e.mem.u32(base + TILES + 4);
            let shown = e.call(TILE_GET_FLOAT, &args![chips, trait_id]).f64();
            shown > zero
        };
        if moves {
            for index in 1..4u32 {
                let reel = model_child(e, index);
                e.call(
                    SET_TRANSLATE_00A59C60,
                    &args![reel, points.addr() + 0xc * index],
                );
            }
        }
    });
    let lever = e.get(this, SlotMachineMenu::pSlotLeverAnim);
    let length = e.call(SEQUENCE_LENGTH_00508100, &args![lever]).f64();
    // `FCOMPP; TEST AH,1; JNZ`: on once the time is not below the length (ordered).
    if spin_time(e) >= length && e.mem.u8(base + 0xf8) == 1 {
        e.set(this, SlotMachineMenu::bSwapTextures, false);
        fn_007c4d70(e, this);
    }
    let lever = e.get(this, SlotMachineMenu::pSlotLeverAnim);
    let length = e.call(SEQUENCE_LENGTH_00508100, &args![lever]).f64();
    if spin_time(e) >= length {
        let reels = e.mem.u32(base + REEL_ANIMATIONS);
        let length = e.call(SEQUENCE_LENGTH_00508100, &args![reels]).f64();
        if spin_time(e) >= length {
            e.set(this, SlotMachineMenu::eSlotState, 2);
            fn_007c40b0(e, this);
        }
    }
}

/// `DoIdle` paying the result (state 2); see there.
fn idle_paying(e: &mut Engine, this: Ptr<SlotMachineMenu>) {
    let base = this.addr();
    let mut winnings = 0i32;
    let result = slot_machine_menu_interpret_spin(e, this);
    e.set(this, SlotMachineMenu::eSpinResult, result as i32);
    let bet = e.get(this, SlotMachineMenu::iCurrentBet);
    let result = e.get(this, SlotMachineMenu::eSpinResult);
    if result == 6 {
        let multiplier = match e.mem.u32(base + REEL_RESULTS) {
            0 | 0xc => Some(10),
            2 => Some(0x14),
            4 => Some(0x1e),
            6 => Some(0x28),
            8 => Some(0x3c),
            10 => Some(0x64),
            _ => None,
        };
        match multiplier {
            Some(multiplier) => {
                winnings = bet.wrapping_mul(multiplier);
                let chips = e
                    .get(this, SlotMachineMenu::iChipCount)
                    .wrapping_add(winnings);
                e.set(this, SlotMachineMenu::iChipCount, chips);
            }
            None => {
                let chips = e.get(this, SlotMachineMenu::iChipCount).wrapping_sub(bet);
                e.set(this, SlotMachineMenu::iChipCount, chips);
            }
        }
    } else {
        winnings = if result == 5 {
            bet.wrapping_mul(5)
        } else if result == 4 {
            bet.wrapping_shl(1)
        } else {
            bet.wrapping_neg()
        };
        let chips = e
            .get(this, SlotMachineMenu::iChipCount)
            .wrapping_add(winnings);
        e.set(this, SlotMachineMenu::iChipCount, chips);
    }
    let trait_id = e.global::<u32>(TRAIT_011DA998);
    let tile = e.mem.u32(base + TILES + 4);
    e.call(TILE_POKE, &args![tile, trait_id, 1.0f32.to_bits()]);
    let chips = e.get(this, SlotMachineMenu::iChipCount) as u32;
    let trait_id = e.global::<u32>(TRAIT_011DA998);
    let tile = e.mem.u32(base + TILES + 4);
    e.call(TILE_SET_INT, &args![tile, trait_id, chips]);
    let data = e.global::<u32>(CASINO_DATA_011DA9A8);
    let total = (e.mem.u32(data.wrapping_add(4)) as i32).wrapping_add(winnings);
    e.mem.set_u32(data.wrapping_add(4), total as u32);
    fn_007c5e60(e, this);
    let won = e.get(this, SlotMachineMenu::eSpinResult) > 3;
    e.with_stack(0x104, |e, buffer| {
        let buffer = buffer.addr();
        let lucky = e.get(this, SlotMachineMenu::bLucky);
        if won {
            let last = e.call(SETTING_STRING, &args![0x011d_3114u32]).u32();
            let third = e.call(SETTING_STRING, &args![0x011d_5050u32]).u32();
            let setting = if lucky { 0x011d_3a98u32 } else { 0x011d_2ff4 };
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
        } else {
            if lucky {
                play_sound(e, SOUND_LUCKY_LOSS_010767C0);
            }
            let last = e.call(SETTING_STRING, &args![0x011d_3114u32]).u32();
            let third = e.call(SETTING_STRING, &args![0x011d_5050u32]).u32();
            let setting = if lucky { 0x011d_25e0u32 } else { 0x011d_3474 };
            let first = e.call(SETTING_STRING, &args![setting]).u32();
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
        }
        let message = e.mem.u32(slot_menu(e) + 0xc4);
        e.call(TILE_POKE, &args![message, 0xfc4u32, 1.0f32.to_bits()]);
        let message = e.mem.u32(slot_menu(e) + 0xc4);
        e.call(TILE_SET_STRING, &args![message, 0xfc4u32, buffer, 1u32]);
        let message = e.mem.u32(slot_menu(e) + 0xc4);
        e.call(TILE_SET_INT, &args![message, 0xfa3u32, 1u32]);
    });
    e.set(this, SlotMachineMenu::eSlotState, if won { 3 } else { 6 });
}

/// `DoIdle` playing the coins (state 3); see there.
fn idle_coins(e: &mut Engine, this: Ptr<SlotMachineMenu>) {
    let coin = e.get(this, SlotMachineMenu::pSlotCoinAnim);
    if coin == 0 {
        e.set(this, SlotMachineMenu::eSlotState, 6);
        return;
    }
    activate(e, this, coin);
    let lever = e.get(this, SlotMachineMenu::pSlotLeverAnim);
    if lever != 0 {
        deactivate(e, this, lever);
    }
    translate_child_by_time(e, 0);
    let zero: f64 = e.global(ZERO_DOUBLE);
    if spin_time(e) == zero {
        match e.get(this, SlotMachineMenu::eSpinResult) {
            4 => play_sound(e, SOUND_WIN_4_010767F4),
            5 => play_sound(e, SOUND_WIN_5_010767E4),
            6 => play_sound(e, SOUND_WIN_6_010767D0),
            _ => {}
        }
    }
    hide_message_after_a_second(e, this);
    let coin = e.get(this, SlotMachineMenu::pSlotCoinAnim);
    let length = e.call(SEQUENCE_LENGTH_00508100, &args![coin]).f64();
    if !(spin_time(e) >= length) {
        return;
    }
    let coin = e.get(this, SlotMachineMenu::pSlotCoinAnim);
    deactivate(e, this, coin);
    let lever = e.get(this, SlotMachineMenu::pSlotLeverAnim);
    if lever != 0 {
        activate(e, this, lever);
    }
    let casino = e.global::<u32>(CASINO_011DA9AC);
    let limits = e.call(CASINO_LIMITS_00504E60, &args![casino]).u32();
    let limit = e.mem.u32(limits.wrapping_add(0x28)) as i32;
    let data = e.global::<u32>(CASINO_DATA_011DA9A8);
    let won_total = e.mem.u32(data.wrapping_add(4)) as i32;
    let level = i32::from(e.mem.u16(data.wrapping_add(8)) as i16);
    let step: f64 = e.global(PAYOUT_STEP_010290B0);
    for index in 1..5i32 {
        if index <= level {
            e.set(this, SlotMachineMenu::bRunScriptOnExit, false);
        } else {
            let threshold = x87_mul(f64::from(limit), x87_mul(f64::from(index), step));
            // `FILD won; FCOMPP; TEST AH,1; JNZ`: reached when not below (ordered).
            if f64::from(won_total) >= threshold {
                let data = e.global::<u32>(CASINO_DATA_011DA9A8);
                e.mem.set_u16(data.wrapping_add(8), index as u16);
                e.set(this, SlotMachineMenu::bRunScriptOnExit, true);
            }
        }
    }
    if e.get(this, SlotMachineMenu::bRunScriptOnExit) {
        fn_007c2420(e);
    } else {
        e.set(this, SlotMachineMenu::eSlotState, 6);
    }
}

/// `DoIdle` running a payout list animation (states 4 and 5, the sequence at `offset`); the
/// first shows the payout list text, the second hides it and, at its end, starts the lever
/// and goes to betting.
fn idle_payout_list(e: &mut Engine, this: Ptr<SlotMachineMenu>, offset: u32, first: bool) {
    let base = this.addr();
    if e.mem.u32(base + offset) == 0 {
        return;
    }
    let zero: f64 = e.global(ZERO_DOUBLE);
    if spin_time(e) == zero {
        let message = e.mem.u32(base + 0xc4);
        if first {
            e.call(TILE_SET_INT, &args![message, 0xfa3u32, 1u32]);
            let text = e.call(SETTING_STRING, &args![0x011d_387cu32]).u32();
            let message = e.mem.u32(base + 0xc4);
            e.call(TILE_SET_STRING, &args![message, 0xfc4u32, text, 1u32]);
        } else {
            e.call(TILE_SET_INT, &args![message, 0xfa3u32, 0u32]);
        }
        let manager = e.get(this, SlotMachineMenu::pSlotAnimManager);
        e.call(DEACTIVATE_ALL, &args![manager, 0.0f32.to_bits()]);
        let sequence = e.mem.u32(base + offset);
        activate(e, this, sequence);
    }
    let sequence = e.mem.u32(base + offset);
    let length = e.call(SEQUENCE_LENGTH_00508100, &args![sequence]).f64();
    // `FCOMPP; TEST AH,0x41; JP`: on while not past the length (below or equal, ordered).
    if spin_time(e) <= length {
        translate_child_by_time(e, 0);
    } else {
        let sequence = e.mem.u32(base + offset);
        deactivate(e, this, sequence);
        if !first {
            let lever = e.get(this, SlotMachineMenu::pSlotLeverAnim);
            activate(e, this, lever);
            e.set(this, SlotMachineMenu::eSlotState, 6);
        }
    }
}

/// `DoIdle` while betting (state 6); see there.
fn idle_betting(e: &mut Engine, this: Ptr<SlotMachineMenu>) {
    hide_message_after_a_second(e, this);
    if e.get(this, SlotMachineMenu::iChipCount) < e.get(this, SlotMachineMenu::iCurrentBet) {
        e.set(this, SlotMachineMenu::iCurrentBet, 1);
        show_number(e, TILES, 1);
    }
    if e.get(this, SlotMachineMenu::iChipCount) == 0 {
        let text = e.call(SETTING_STRING, &args![0x011d_2424u32]).u32();
        refuse_with(e, text);
        fn_007c2420(e);
    }
}

/// `DoIdle` while closing (state 7): as the roulette menu's (see `roulettemenu.rs`).
fn idle_closing(e: &mut Engine, this: Ptr<SlotMachineMenu>) {
    if e.call(MENU_FADE_STATE_0059BB30, &args![this]).u32() == 2 {
        return;
    }
    let fade = x87_quiet_bits(e.global::<u32>(CURSOR_FADE_01023CD8));
    set_cursor_fade(e, fade);
    let root = e.call(WORD_AT_4_00726070, &args![this]).u32();
    e.call(TILE_SET_INT, &args![root, 0x1772u32, 1u32]);
    e.call(MENU_START_FADE_OUT, &args![this]);
    let casino = e.global::<u32>(CASINO_011DA9AC);
    let item = e.call(CASINO_ITEM_007316D0, &args![casino]).u32();
    let player = e.global::<u32>(PLAYER_011DEA3C);
    let count = e
        .call(GET_ITEM_COUNT_IN_CONTAINER, &args![player, item])
        .i32();
    let difference = e.get(this, SlotMachineMenu::iChipCount).wrapping_sub(count);
    if difference < 0 {
        let casino = e.global::<u32>(CASINO_011DA9AC);
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
            let casino = e.global::<u32>(CASINO_011DA9AC);
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
            let casino = e.global::<u32>(CASINO_011DA9AC);
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
            let casino = e.global::<u32>(CASINO_011DA9AC);
            let item = e.call(CASINO_ITEM_007316D0, &args![casino]).u32();
            e.call(
                CONTAINER_ADD,
                &args![container, item, difference as u32, 0u32],
            );
            let casino = e.global::<u32>(CASINO_011DA9AC);
            let limit = e.call(CASINO_00737E30, &args![casino]).i32();
            let data = e.global::<u32>(CASINO_DATA_011DA9A8);
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
                    let casino = e.global::<u32>(CASINO_011DA9AC);
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
                    let casino = e.global::<u32>(CASINO_011DA9AC);
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
    if e.get(this, SlotMachineMenu::bRunScriptOnExit) {
        let casino = e.global::<u32>(CASINO_011DA9AC);
        let script = e.call(CASINO_SCRIPT_00504E40, &args![casino]).u32();
        if script != 0 {
            e.call(SCRIPT_RUN_0060C9C0, &args![script, 1u32]);
        }
    }
}

/// Sets the texture of one reel face: the face's node (`"%i_%i"`-like `0107681c` of `6 -
/// reel` and `face`) on the reel's geometry; its property 3 (`GetProperty`), when its type
/// (`00441110`) is 8 to 0xc, gets texture `texture` (vtable +0xf0 with 0, 0 and the
/// texture's pointer).
fn set_reel_face(e: &mut Engine, this: Ptr<SlotMachineMenu>, reel: u32, face: i32, texture: i32) {
    let base = this.addr();
    let object = e.with_stack(0x104, |e, name| {
        e.call(
            SPRINTF_S,
            &[
                name.addr(),
                0x104,
                REEL_FACE_FORMAT_0107681C,
                6u32.wrapping_sub(reel),
                face as u32,
            ],
        );
        let geometry = e.mem.u32(base + REEL_GEOMETRY + 4 * reel);
        e.call(GET_OBJECT_BY_NAME, &args![geometry, name]).u32()
    });
    let usable = e.call(GET_PROPERTY, &args![object, 3u32]).u32() != 0 && {
        let property = e.call(GET_PROPERTY, &args![object, 3u32]).u32();
        let kind = e.call(PROPERTY_TYPE_00441110, &args![property]).i32();
        kind >= 8 && {
            let property = e.call(GET_PROPERTY, &args![object, 3u32]).u32();
            e.call(PROPERTY_TYPE_00441110, &args![property]).i32() <= 0xc
        }
    };
    let property = if usable {
        e.call(GET_PROPERTY, &args![object, 3u32]).u32()
    } else {
        0
    };
    if property != 0 {
        let slot = base
            .wrapping_add(REEL_TEXTURES)
            .wrapping_add((texture as u32).wrapping_mul(4));
        let image = e.call(WORD_AT_0_00559450, &args![slot]).u32();
        e.vcall(property, 0xf0, &args![0u32, 0u32, image]);
    }
}

/// A face index past 0xd wraps by 14 (`LEA; CMP 0xd; JLE`), one below 0 wraps up by 14.
fn wrap_face(face: i32) -> i32 {
    if face > 0xd {
        face - 14
    } else if face < 0 {
        face + 14
    } else {
        face
    }
}

// Translated from 007c40b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007c40b0 - the /GS cookie check (00ec408c) is not translated.
/// Unnamed (`thiscall` on the slot machine menu): for each reel whose result differs from
/// its start, the eight faces `2r + 4` to `2r + 11` (wrapping by 14 past 0xd) get the
/// textures around the result: for an even result `r/2 + 3` to `r/2 + 6` (wrapping by 7
/// past 6) on faces +4, +6, +8, +10 and the blank one (7) on the others; for an odd result
/// `(r + 3)/2` to `(r + 6)/2` on faces +5, +7, +9, +11 and the blank one on the others.
pub fn fn_007c40b0(e: &mut Engine, this: Ptr<SlotMachineMenu>) {
    let base = this.addr();
    for reel in 0..3u32 {
        let result = e.mem.u32(base + REEL_RESULTS + 4 * reel) as i32;
        let start = e.mem.u32(base + REEL_START + 4 * reel) as i32;
        if result.wrapping_sub(start) == 0 {
            continue;
        }
        let wrap = |value: i32| if value > 6 { value - 7 } else { value };
        let textures: [i32; 8] = if result % 2 == 0 {
            let half = result / 2;
            [
                wrap(half + 3),
                7,
                wrap(half + 4),
                7,
                wrap(half + 5),
                7,
                wrap(half + 6),
                7,
            ]
        } else {
            [
                7,
                wrap((result + 3) / 2),
                7,
                wrap((result + 4) / 2),
                7,
                wrap((result + 5) / 2),
                7,
                wrap((result + 6) / 2),
            ]
        };
        for (step, texture) in textures.into_iter().enumerate() {
            let face = wrap_face(2 * reel as i32 + 4 + step as i32);
            set_reel_face(e, this, reel, face, texture);
        }
    }
}

// Translated from 007c4d70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007c4d70 - the /GS cookie check (00ec408c) is not translated.
/// Unnamed (`thiscall` on the slot machine menu): for each reel whose result differs from
/// its start, the six faces `2r + 3` down to `2r - 2` (wrapping by 14) get the textures
/// around the result: for an even result `r/2` on face `2r`, `r/2 + 1` on `2r + 2` and
/// `r/2 - 1` on `2r - 2` (wrapping by 7), the blank one (7) on the others; for an odd result
/// `(r + 1)/2` on `2r + 1`, `(r + 3)/2` on `2r + 3`, `(r - 1)/2` on `2r - 1`, the blank one on
/// the others.
pub fn fn_007c4d70(e: &mut Engine, this: Ptr<SlotMachineMenu>) {
    let base = this.addr();
    for reel in 0..3u32 {
        let result = e.mem.u32(base + REEL_RESULTS + 4 * reel) as i32;
        let start = e.mem.u32(base + REEL_START + 4 * reel) as i32;
        if result.wrapping_sub(start) == 0 {
            continue;
        }
        let (first, second, third, fourth, fifth, sixth) = if result % 2 == 0 {
            let half = result / 2;
            let mut above = half + 1;
            let mut below = half - 1;
            if above > 6 {
                above -= 7;
            }
            if below < 0 {
                below += 7;
            }
            (7, above, 7, half, 7, below)
        } else {
            let mut above = (result + 1) / 2;
            let mut top = (result + 3) / 2;
            let mut below = (result - 1) / 2;
            if above > 6 {
                above -= 7;
            }
            if top > 6 {
                top -= 7;
            }
            if below < 0 {
                below += 7;
            }
            (top, 7, above, 7, below, 7)
        };
        let reel_face = 2 * reel as i32;
        for (offset, texture) in [
            (3, first),
            (2, second),
            (1, third),
            (0, fourth),
            (-1, fifth),
            (-2, sixth),
        ] {
            set_reel_face(e, this, reel, wrap_face(reel_face + offset), texture);
        }
    }
}

// Translated from 007c56e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the slot machine menu): each reel's start becomes its last result
/// and its result a random active symbol (`00944460(0, 2 * iNumActiveSymbols - 1)`);
/// returns `InterpretSpin`.
pub fn fn_007c56e0(e: &mut Engine, this: Ptr<SlotMachineMenu>) -> u32 {
    let base = this.addr();
    for reel in 0..3u32 {
        let last = e.mem.u32(base + REEL_RESULTS + 4 * reel);
        e.mem.set_u32(base + REEL_START + 4 * reel, last);
        let symbols = e.get(this, SlotMachineMenu::iNumActiveSymbols);
        let limit = symbols.wrapping_mul(2).wrapping_sub(1) as u32;
        let index = e.call(RANDOM_00944460, &args![0u32, limit]).u32();
        let symbol = e.mem.u32(
            base.wrapping_add(ACTIVE_SYMBOLS)
                .wrapping_add(index.wrapping_mul(4)),
        );
        e.mem.set_u32(base + REEL_RESULTS + 4 * reel, symbol);
    }
    slot_machine_menu_interpret_spin(e, this)
}

// Translated from 007c5760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SlotMachineMenu::InterpretSpin` (Xbox PDB; `thiscall`): 6 for three equal results with
/// an even middle one, 5 for two wild symbols (0xc), 4 for one, 2 when any is wild (left
/// over from the counts), 1 for two equal neighbours (middle and right, left and right,
/// or middle and left) with an even value, else 0.
pub fn slot_machine_menu_interpret_spin(e: &mut Engine, this: Ptr<SlotMachineMenu>) -> u32 {
    let base = this.addr();
    let result = |e: &Engine, reel: u32| e.mem.u32(base + REEL_RESULTS + 4 * reel) as i32;
    let wild = (0..3u32).filter(|reel| result(e, *reel) == 0xc).count();
    let (left, middle, right) = (result(e, 0), result(e, 1), result(e, 2));
    if left == middle && left == right && middle % 2 == 0 {
        return 6;
    }
    if wild == 2 {
        return 5;
    }
    if wild == 1 {
        return 4;
    }
    if left == 0xc || middle == 0xc || right == 0xc {
        return 2;
    }
    if (middle == right && middle % 2 == 0)
        || (left == right && left % 2 == 0)
        || (middle == left && left % 2 == 0)
    {
        return 1;
    }
    0
}

// Translated from 007c5e60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007c5e60 - the /GS cookie check (00ec408c) is not translated.
/// Unnamed (`thiscall` on the slot machine menu): the chip count text, as the roulette
/// menu's `007bf9e0` with this menu's casino data, casino, trait (`011da99c`) and tile
/// (+0xa8).
pub fn fn_007c5e60(e: &mut Engine, this: Ptr<SlotMachineMenu>) {
    e.with_stack(0x110, |e, frame| {
        let counter = frame.addr() + 4;
        let buffer = frame.addr() + 8;
        e.mem.set_u32(frame.addr(), this.addr());
        let data = e.global::<u32>(CASINO_DATA_011DA9A8);
        let count = e.mem.u32(data.wrapping_add(4));
        let setting = e.call(SETTING_STRING, &args![SETTING_011D3E08]).u32();
        let casino = e.global::<u32>(CASINO_011DA9AC);
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
            let casino = e.global::<u32>(CASINO_011DA9AC);
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
            let data = e.global::<u32>(CASINO_DATA_011DA9A8);
            let count = e.mem.u32(data.wrapping_add(4));
            e.call(
                SPRINTF_S,
                &[buffer, 0x104, PADDED_FORMAT_01029CA8, buffer, count],
            );
        }
        let trait_id = e.global::<u32>(TRAIT_011DA99C);
        let menu_tile = e.mem.u32(this.addr() + 0xa8);
        e.call(TILE_SET_STRING, &args![menu_tile, trait_id, buffer, 1u32]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x007c_0800,
            slot_machine_menu_get_class(Ptr<SlotMachineMenu>) -> u32
        ),
        entry!(
            0x007c_0810,
            slot_machine_menu_attach_tile_by_id(Ptr<SlotMachineMenu>, i32, u32)
        ),
        entry!(
            0x007c_0840,
            slot_machine_menu_scalar_deleting_destructor(
                Ptr<SlotMachineMenu>,
                u32,
            ) -> Ptr<SlotMachineMenu>
        ),
        entry!(0x007c_0870, fn_007c0870(Ptr<SlotMachineMenu>)),
        entry!(0x007c_0a40, slot_machine_menu_create(u32, i32, i32, i32) -> u32),
        entry!(0x007c_1440, fn_007c1440() -> u32),
        entry!(0x007c_1450, fn_007c1450(Ptr<SlotMachineMenu>) -> bool),
        entry!(
            0x007c_1490,
            slot_machine_menu_initialize_casino_data(Ptr<SlotMachineMenu>) -> bool
        ),
        entry!(
            0x007c_16b0,
            slot_machine_menu_setup_active_reels(Ptr<SlotMachineMenu>) -> bool
        ),
        entry!(0x007c_19c0, fn_007c19c0()),
        entry!(
            0x007c_1af0,
            slot_machine_menu_prepare_3d_elements(Ptr<SlotMachineMenu>)
        ),
        entry!(0x007c_2180, fn_007c2180(Ptr) -> u32),
        entry!(0x007c_2420, fn_007c2420()),
        entry!(
            0x007c_2460,
            slot_machine_menu_do_click(Ptr<SlotMachineMenu>, u32, u32)
        ),
        entry!(0x007c_2c70, slot_machine_menu_do_idle(Ptr<SlotMachineMenu>)),
        entry!(0x007c_40b0, fn_007c40b0(Ptr<SlotMachineMenu>)),
        entry!(0x007c_4d70, fn_007c4d70(Ptr<SlotMachineMenu>)),
        entry!(0x007c_56e0, fn_007c56e0(Ptr<SlotMachineMenu>) -> u32),
        entry!(
            0x007c_5760,
            slot_machine_menu_interpret_spin(Ptr<SlotMachineMenu>) -> u32
        ),
        entry!(0x007c_5e60, fn_007c5e60(Ptr<SlotMachineMenu>)),
    ]
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

    /// Every callee outside this file doubled (0 or nothing), the statics and constants
    /// mapped; the word, node, `NiPointer` and child helpers act on memory, the setting
    /// strings are their own address and the chip text format writes `"abc"`.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011d_a000,
            0x011a_0000,
            0x011d_e000,
            0x011c_3000,
            0x011f_3000,
            0x011f_6000,
            0x011f_9000,
            0x0120_3000,
            0x0101_2000,
            0x0101_6000,
            0x0101_7000,
            0x0101_f000,
            0x0102_1000,
            0x0102_3000,
            0x0102_9000,
            0x0103_0000,
            0x0105_4000,
            0x0107_0000,
            0x0107_6000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(THOUSAND_DOUBLE, 1000.0f64);
        e.set_global(PAYOUT_STEP_010290B0, 0.5f64);
        e.set_global(CURSOR_FADE_01023CD8, 0.5f32);
        for addr in [
            FREE,
            PLAYER_00969B20,
            CASINO_MODEL_00504E00,
            CASINO_TEXTURE_00504E20,
            MODEL_LOADER_LOAD_FILE,
            MODEL_LOADER_0045A5E0,
            SCENE_00450B80,
            SCENE_00B5D180,
            NODE_00572160,
            NI_POINTER_DESTROY,
            VECTOR_DESTRUCT,
            MENU_DESTRUCT,
            INTERFACE_MANAGER,
            WORD_AT_28_0045CD60,
            TILE_SET_FLOAT,
            TILE_SET_STRING,
            TILE_SET_INT,
            TILE_POKE,
            TILE_GET_FLOAT,
            STRLEN,
            SWAP_TEXTURE_PATH,
            CREATE_TEXTURE_IMAGE,
            SET_TRANSLATE_004BC1F0,
            GET_OBJECT_BY_NAME,
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
            ACTIVATE_SEQUENCE_0047AAB0,
            DEACTIVATE_SEQUENCE_0047B220,
            SEQUENCE_LENGTH_00508100,
            MAKE_X_ROTATION,
            MATRIX_MULTIPLY,
            GET_PROPERTY,
            PROPERTY_TYPE_00441110,
            CAMERA_CONSTRUCT,
            CAMERA_00414430,
            CAMERA_00712E60,
            FRUSTUM_CONSTRUCT,
            RENDER_0045C670,
            CAMERA_00507700,
            SCREEN_004DC200,
            SCREEN_004DC1F0,
            TAN_0057DD70,
            SET_VIEW_FRUSTUM,
            POINT_00416870,
            LOOK_AT_WORLD_POINT,
            CAMERA_00733FD0,
            SET_NAME_00A5B950,
            AUDIO_INSTANCE,
            GET_SOUND_HANDLE_BY_NAME,
            SOUND_HANDLE_PLAY,
            SOUND_HANDLE_DESTROY,
            TILE_GET_MENU_BY_CLASS,
            CASINO_DATA_LIST_005A6260,
            CASINO_DATA_CONSTRUCT,
            KEY_0084E3A0,
            LIST_IS_EMPTY,
            LIST_APPEND,
            LIST_SET_FIRST_00726C60,
            DATA_VALID_004839C0,
            PLAYER_00969B80,
            PLAYER_00969B40,
            SHOW_MESSAGE_007052F0,
            CASINO_LIMITS_00504E60,
            CASINO_ITEM_007316D0,
            GET_ITEM_COUNT_IN_CONTAINER,
            WORD_AT_9C_00586150,
            TILE_READ_FILE,
            TILE_GET_MENU,
            MENU_SET_MENU_TILE,
            DEBUG_MESSAGE,
            TILE_GET_MAXIMUM_DEPTH,
            GET_CLAMPED_ACTOR_VALUE,
            NAME_CHARS_00408DA0,
            TILE_CHILD_BY_NAME_00A08B20,
            MENU_PREP_FOR_VISIBILITY,
            RANDOM_00944460,
            LUCK_007C58F0,
            MISC_STAT_INCREMENT,
            MENU_FADE_STATE_0059BB30,
            MENU_START_FADE_OUT,
            GET_PICK_UP_SOUND_NAME,
            GET_FULL_NAME,
            CASINO_00737E30,
            CASINO_SCRIPT_00504E40,
            SCRIPT_RUN_0060C9C0,
            CONTAINER_CONSTRUCT,
            CONTAINER_ADD,
            CONTAINER_GIVE,
            WINNINGS_CONTAINER_DESTRUCT,
            STRING_CONSTRUCT,
            STRING_DESTROY,
            STRING_FORMAT,
            INTERFACE_IS_TOP_MENU_ID,
            WORD_AT_14_00825C00,
            RENDER_0043C4B0,
            RENDER_007148C0,
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
        e.register(NODE_CHILD_0043B4A0, |e, a| {
            e.mem.u32(a[0] + 0x80 + 4 * a[1]).into_ret()
        });
        e.register(SETTING_STRING, |_, a| a[0].into_ret());
        e.register(SPRINTF_S, |e, a| {
            if a[2] == CHIPS_FORMAT_01070E80 {
                for (index, byte) in b"abc\0".iter().enumerate() {
                    e.mem.set_u8(a[0] + index as u32, *byte);
                }
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

    /// An open slot machine menu (fake vtable 1, real tiles), the model node (fake object
    /// 2) with four children, the casino (3) and its data, the player (4) and the scene (5).
    fn world(e: &mut Engine) -> u32 {
        let menu = e.mem.alloc(0x200);
        let vtable = fake_vtable(e, 1);
        e.mem.set_u32(menu, vtable);
        for index in 0..0xau32 {
            let tile = e.mem.alloc(0x10);
            e.mem.set_u32(menu + TILES + 4 * index, tile);
        }
        e.set_global(SLOT_MENU_011DA9A4, menu);
        let model = fake_object(e, 2);
        for index in 0..4u32 {
            let child = e.mem.alloc(0x100);
            e.mem.set_u32(model + 0x80 + 4 * index, child);
        }
        e.set_global(MODEL_011DA9FC, model);
        let casino = fake_object(e, 3);
        e.set_global(CASINO_011DA9AC, casino);
        let data = e.mem.alloc(0x10);
        e.set_global(CASINO_DATA_011DA9A8, data);
        let player = fake_object(e, 4);
        e.set_global(PLAYER_011DEA3C, player);
        let scene = fake_object(e, 5);
        e.register_double(SCENE_00450B80, move |_, _| scene.into_ret());
        let model_object = fake_object(e, 6);
        e.register_double(CASINO_MODEL_00504E00, move |_, _| model_object.into_ret());
        menu
    }

    fn get(e: &Engine, menu: u32, field: Field<SlotMachineMenu, i32>) -> i32 {
        e.get(Ptr::<SlotMachineMenu>::new(menu), field)
    }

    fn set(e: &mut Engine, menu: u32, field: Field<SlotMachineMenu, i32>, value: i32) {
        e.set(Ptr::<SlotMachineMenu>::new(menu), field, value);
    }

    fn results(e: &mut Engine, menu: u32, values: [u32; 3]) {
        for (reel, value) in values.into_iter().enumerate() {
            e.mem.set_u32(menu + REEL_RESULTS + 4 * reel as u32, value);
        }
    }

    #[test]
    fn the_class_the_tiles_and_close() {
        let mut e = engine();
        let menu = world(&mut e);
        assert_eq!(e.call(0x007c_0800, &args![menu]).u32(), 0x438);
        assert_eq!(e.call(0x007c_1440, &args![]).u32(), 0x438);
        assert!(e.call(0x007c_1450, &args![menu]).bool());
        e.call(0x007c_0810, &args![menu, 9u32, 0x5555u32]);
        assert_eq!(e.mem.u32(menu + 0xc4), 0x5555);
        e.call(0x007c_0810, &args![menu, 0xau32, 0x6666u32]);
        assert_ne!(e.mem.u32(menu + 0xc8), 0x6666);
        e.call(0x007c_0810, &args![menu, 2u32, 0u32]);
        assert!(!e.call(0x007c_1450, &args![menu]).bool());
        e.call_log = Some(vec![]);
        e.call(0x007c_2420, &args![]);
        assert_eq!(get(&e, menu, SlotMachineMenu::eSlotState), 7);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0, 0xfa9, 0.5f32.to_bits(), 1]]
        );
        e.set_global(SLOT_MENU_011DA9A4, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x007c_2420, &args![]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn the_destructor_releases_everything() {
        let mut e = engine();
        let menu = world(&mut e);
        e.register(slot(6, 0x14), |_, _| 0x00ab_0000u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007c_0840, &args![menu, 1u32]);
        assert_eq!(e.mem.u32(menu), SLOT_MENU_VTABLE);
        let released: Vec<u32> = calls_to(&e, MODEL_LOADER_0045A5E0)
            .iter()
            .map(|w| w[1])
            .collect();
        assert_eq!(
            released,
            vec![
                0x00ab_0000,
                REEL_MODEL_PATH_0107659C,
                REEL_MODEL_PATH_01076560,
                REEL_MODEL_PATH_01076524
            ]
        );
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCT),
            vec![vec![menu + 0x114, 4, 8, NI_POINTER_DESTROY]]
        );
        assert_eq!(e.global::<u32>(SLOT_MENU_011DA9A4), 0);
        assert_eq!(e.global::<u32>(MODEL_011DA9FC), 0);
        assert_eq!(calls_to(&e, FREE), vec![vec![menu]]);
    }

    #[test]
    fn interpret_spin_and_the_draw() {
        let mut e = engine();
        let menu = world(&mut e);
        for (values, expected) in [
            ([4u32, 4, 4], 6u32),
            ([5, 5, 5], 0),
            ([0xc, 0xc, 3], 5),
            ([0xc, 1, 3], 4),
            ([3, 2, 2], 1),
            ([2, 3, 2], 1),
            ([3, 3, 1], 0),
            ([1, 3, 5], 0),
        ] {
            results(&mut e, menu, values);
            assert_eq!(
                e.call(0x007c_5760, &args![menu]).u32(),
                expected,
                "{values:?}"
            );
        }
        // The draw: start = the last result, result = active symbol `random(0, 2n - 1)`.
        set(&mut e, menu, SlotMachineMenu::iNumActiveSymbols, 4);
        for index in 0..8u32 {
            e.mem.set_u32(menu + ACTIVE_SYMBOLS + 4 * index, 6);
        }
        e.register(RANDOM_00944460, |_, a| a[1].into_ret());
        results(&mut e, menu, [1, 2, 3]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007c_56e0, &args![menu]).u32(), 6);
        assert_eq!(calls_to(&e, RANDOM_00944460), vec![vec![0, 7]; 3]);
        assert_eq!(e.mem.u32(menu + REEL_START + 8), 3);
    }

    #[test]
    fn setup_active_reels_places_the_symbol_pairs() {
        let mut e = engine();
        let menu = world(&mut e);
        let limits = e.mem.alloc(0x40);
        // Symbols 0 and 3 have counts (7 and 7): every pass adds four faces.
        e.mem.set_u32(limits + 8, 7);
        e.mem.set_u32(limits + 0x14, 7);
        e.register_double(CASINO_LIMITS_00504E60, move |_, _| limits.into_ret());
        assert!(e.call(0x007c_16b0, &args![menu]).bool());
        assert_eq!(get(&e, menu, SlotMachineMenu::iNumActiveSymbols), 2);
        let symbols: Vec<u32> = (0..8)
            .map(|index| e.mem.u32(menu + ACTIVE_SYMBOLS + 4 * index))
            .collect();
        // A symbol with a count adds two and moves on by two, one without moves on by one.
        assert_eq!(symbols, vec![0, 1, 4, 5, 0, 1, 4, 5]);
        assert_eq!(e.mem.u32(menu + REEL_RESULTS + 8), 4);
        assert_eq!(e.mem.u32(menu + REEL_START + 8), 0xe);
        // No counts: a message, the refusal and the close.
        e.mem.set_u32(limits + 8, 0);
        e.mem.set_u32(limits + 0x14, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x007c_16b0, &args![menu]).bool());
        assert_eq!(calls_to(&e, SPRINTF_S)[0][2], NO_SYMBOLS_MESSAGE_01076700);
        assert_eq!(get(&e, menu, SlotMachineMenu::eSlotState), 7);
    }

    #[test]
    fn initialize_casino_data_makes_the_textures_and_models() {
        let mut e = engine();
        let menu = world(&mut e);
        let limits = e.mem.alloc(0x40);
        e.mem.set_u32(limits + 8, 20);
        e.register_double(CASINO_LIMITS_00504E60, move |_, _| limits.into_ret());
        e.set_global(TEXTURES_011DEA10, 0x00ab_2000u32);
        e.register(MODEL_LOADER_LOAD_FILE, |_, a| (a[1] & 0xffff).into_ret());
        e.mem.set_u8(menu + 0x100, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x007c_1490, &args![menu]).bool());
        let images = calls_to(&e, CREATE_TEXTURE_IMAGE);
        assert_eq!(images.len(), 8);
        assert_eq!(images[3][0], 0x00ab_2000);
        assert_eq!(images[3][2], menu + 0x114 + 12);
        assert_eq!(images[7][2], menu + 0x130);
        assert_eq!(e.mem.u32(menu + 0x10c), 0x6560);
        assert_eq!(e.mem.u8(menu + 0x100), 0);
        e.set_global(CASINO_011DA9AC, 0u32);
        assert!(!e.call(0x007c_1490, &args![menu]).bool());
    }

    #[test]
    fn the_reel_faces_follow_the_results() {
        let mut e = engine();
        let menu = world(&mut e);
        // Every face node has property 3 of type 9; the texture pointers are 0x700 + index.
        e.register(GET_OBJECT_BY_NAME, |_, _| 0x00ab_3000u32.into_ret());
        let property = fake_object(&mut e, 7);
        e.register_double(GET_PROPERTY, move |_, _| property.into_ret());
        e.register(PROPERTY_TYPE_00441110, |_, _| 9u32.into_ret());
        for index in 0..9u32 {
            e.mem
                .set_u32(menu + REEL_TEXTURES + 4 * index, 0x700 + index);
        }
        let faces: Log = Rc::new(RefCell::new(vec![]));
        let named = faces.clone();
        e.register_double(SPRINTF_S, move |_, a| {
            if a[2] == REEL_FACE_FORMAT_0107681C {
                named.borrow_mut().push((a[3], a[4]));
            }
            Ret::default()
        });
        results(&mut e, menu, [4, 0, 0]);
        for reel in 0..3u32 {
            e.mem.set_u32(menu + REEL_START + 4 * reel, 0);
        }
        e.call_log = Some(vec![]);
        e.call(0x007c_40b0, &args![menu]);
        let set: Vec<u32> = calls_to(&e, slot(7, 0xf0)).iter().map(|w| w[3]).collect();
        // Reel 0, result 4: faces 4 to 11 get 5, 7, 6, 7, 0, 7, 1, 7.
        assert_eq!(
            set,
            [5u32, 7, 6, 7, 0, 7, 1, 7]
                .map(|index| 0x700 + index)
                .to_vec()
        );
        assert_eq!(
            faces
                .borrow()
                .iter()
                .map(|(_, face)| *face)
                .collect::<Vec<_>>(),
            vec![4, 5, 6, 7, 8, 9, 10, 11]
        );
        assert!(faces.borrow().iter().all(|(reel, _)| *reel == 6));
        // Reel 2 with an odd result 13: faces 2r + 3 .. 2r - 2 = 7, 6, 5, 4, 3, 2.
        faces.borrow_mut().clear();
        results(&mut e, menu, [0, 0, 13]);
        e.call_log = Some(vec![]);
        e.call(0x007c_4d70, &args![menu]);
        let set: Vec<u32> = calls_to(&e, slot(7, 0xf0)).iter().map(|w| w[3]).collect();
        // (13 + 3) / 2 = 8 - 7 = 1, (13 + 1) / 2 = 7 - 7 = 0, (13 - 1) / 2 = 6.
        assert_eq!(
            set,
            [1u32, 7, 0, 7, 6, 7].map(|index| 0x700 + index).to_vec()
        );
        assert_eq!(
            faces
                .borrow()
                .iter()
                .map(|(_, face)| *face)
                .collect::<Vec<_>>(),
            vec![7, 6, 5, 4, 3, 2]
        );
        // A property of another type is left alone.
        e.register(PROPERTY_TYPE_00441110, |_, _| 7u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007c_4d70, &args![menu]);
        assert!(calls_to(&e, slot(7, 0xf0)).is_empty());
    }

    #[test]
    fn the_camera_and_the_render_function() {
        let mut e = engine();
        let menu = world(&mut e);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CAMERA_CONSTRUCT, |_, a| a[0].into_ret());
        let source = fake_object(&mut e, 8);
        e.register_double(RENDER_0045C670, move |_, _| source.into_ret());
        e.register(slot(8, 0x100), |_, _| st0(500.0));
        let near = e.mem.alloc(4);
        e.mem.set_u32(near, 5.0f32.to_bits());
        e.register_double(SETTING_VALUE_00403E20, move |_, _| near.into_ret());
        e.register(SCREEN_004DC200, |_, _| 1600u32.into_ret());
        e.register(SCREEN_004DC1F0, |_, _| 800u32.into_ret());
        e.register(TAN_0057DD70, |_, _| st0(0.5));
        e.set_global(CAMERA_SCALE_01016264, 2.0f32);
        let frustum: Log = Rc::new(RefCell::new(vec![]));
        let seen = frustum.clone();
        e.register_double(SET_VIEW_FRUSTUM, move |e, a| {
            for word in 0..6 {
                seen.borrow_mut().push((word, e.mem.u32(a[1] + 4 * word)));
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let camera = e.call(0x007c_2180, &args![menu]).u32();
        assert_ne!(camera, 0);
        assert_eq!(calls_to(&e, CAMERA_00507700)[0][1], 100.0f32.to_bits());
        let values: Vec<f32> = frustum
            .borrow()
            .iter()
            .map(|(_, v)| f32::from_bits(*v))
            .collect();
        // Left, right, top, bottom, near, far: aspect 2.0, tangent 0.5, scale 2.0.
        assert_eq!(values, vec![-1.0, 1.0, 2.0, -2.0, 5.0, 500.0]);
        assert_eq!(calls_to(&e, SET_NAME_00A5B950).len(), 1);
        // The render function draws through the slot camera.
        e.set_global(CAMERA_011DA9D0, 0x00ab_4000u32);
        e.call_log = Some(vec![]);
        e.call(0x007c_19c0, &args![]);
        assert_eq!(calls_to(&e, ACCUMULATE_SCENE)[0][0], 0x00ab_4000);
    }

    #[test]
    fn prepare_3d_elements_builds_the_model() {
        let mut e = engine();
        let menu = world(&mut e);
        e.register(CAMERA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(SETTING_VALUE_00403E20, |_, _| {
            NEAR_SETTING_01203144.into_ret()
        });
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let model = e.global::<u32>(MODEL_011DA9FC);
        e.register_double(NI_NODE_CONSTRUCT, move |_, _| model.into_ret());
        e.register(GET_CONTROLLER, |_, a| (a[0] + 0x10).into_ret());
        e.register(SEQUENCE_BY_NAME_0047A520, |_, _| 0x00ab_6000u32.into_ret());
        let source = fake_object(&mut e, 8);
        e.register_double(RENDER_0045C670, move |_, _| source.into_ret());
        e.register(SCREEN_004DC1F0, |_, _| 1u32.into_ret());
        e.register(BOUND_RADIUS_0084D030, |_, _| st0(2.0));
        e.set_global(LIGHT_SCALE_01021928, 3.0f64);
        e.mem.set_u32(menu + 0x104, 0x5104);
        for reel in 0..3u32 {
            e.mem
                .set_u32(menu + REEL_GEOMETRY + 4 * reel, 0x5108 + 4 * reel);
        }
        e.call_log = Some(vec![]);
        e.call(0x007c_1af0, &args![menu]);
        assert_eq!(calls_to(&e, slot(2, 0xdc)).len(), 4);
        assert_eq!(e.mem.u32(menu + 0x134), 0x5114);
        assert_eq!(e.mem.u32(menu + 0x144), 0x00ab_6000);
        assert_eq!(e.mem.u32(menu + REEL_ANIMATIONS + 8), 0x00ab_6000);
        // The machine's lever and the three reels start.
        assert_eq!(calls_to(&e, ACTIVATE_SEQUENCE_0047AAB0).len(), 4);
        assert_eq!(calls_to(&e, START_ANIMATIONS).len(), 4);
        let lights = calls_to(&e, ADD_ALL_LIGHTS_FROM_NODE);
        assert_eq!(lights[0][1..], [model, 6.0f32.to_bits()]);
        assert_ne!(e.global::<u32>(CAMERA_011DA9D0), 0);
    }

    #[test]
    fn create_refuses_or_builds_the_menu() {
        let mut e = engine();
        let menu = world(&mut e);
        let casino = e.global::<u32>(CASINO_011DA9AC);
        // A casino data item without a key: the message and 0.
        let list = e.mem.alloc(8);
        let bad = e.mem.alloc(0xc);
        e.mem.set_u32(list, bad);
        e.register_double(CASINO_DATA_LIST_005A6260, move |_, _| list.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x007c_0a40, &args![casino, 5u32, 50u32, 0u32]).u32(),
            0
        );
        assert_eq!(
            calls_to(&e, SHOW_MESSAGE_007052F0)[0][0],
            NO_DATA_MESSAGE_01076694
        );
        assert_eq!(e.global::<u32>(MIN_BET_011A082C), 5);
        assert_eq!(e.global::<u32>(MAX_BET_011A0828), 50);
        // Banned.
        e.mem.set_u32(bad, 0x77);
        e.register(DATA_VALID_004839C0, |_, _| 1u32.into_ret());
        e.register(KEY_0084E3A0, |_, _| 0x77u32.into_ret());
        e.register(PLAYER_00969B80, |_, _| 1u32.into_ret());
        e.register(PLAYER_00969B40, |_, _| 3u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x007c_0a40, &args![casino, 5u32, 50u32, 0u32]).u32(),
            0
        );
        assert_eq!(e.global::<u32>(CASINO_DATA_011DA9A8), bad);
        assert_eq!(
            calls_to(&e, SPRINTF_S)[0][2..],
            [BANNED_FORMAT_01070C58, 0x011d_2ac0, 3]
        );
        // Built.
        e.register(PLAYER_00969B80, |_, _| 0u32.into_ret());
        let limits = e.mem.alloc(0x40);
        e.mem.set_u32(limits + 0x28, 1000);
        e.mem.set_u32(limits + 8, 20);
        e.register_double(CASINO_LIMITS_00504E60, move |_, _| limits.into_ret());
        e.register(GET_ITEM_COUNT_IN_CONTAINER, |_, _| 100u32.into_ret());
        e.register(TILE_READ_FILE, |_, _| 0x6100u32.into_ret());
        e.register_double(TILE_GET_MENU, move |_, _| menu.into_ret());
        e.register(slot(1, 0x34), |_, _| 0x438u32.into_ret());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let model = e.global::<u32>(MODEL_011DA9FC);
        e.register_double(NI_NODE_CONSTRUCT, move |_, _| model.into_ret());
        let source = fake_object(&mut e, 8);
        e.register_double(RENDER_0045C670, move |_, _| source.into_ret());
        e.register(SCREEN_004DC1F0, |_, _| 1u32.into_ret());
        let empty = e.mem.alloc(4);
        e.register_double(NAME_CHARS_00408DA0, move |_, _| empty.into_ret());
        e.register(SETTING_VALUE_00403E20, |_, _| {
            NEAR_SETTING_01203144.into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x007c_0a40, &args![casino, 5u32, 50u32, 0u32]).u32(),
            0x6100
        );
        assert_eq!(get(&e, menu, SlotMachineMenu::iCurrentBet), 5);
        assert_eq!(
            calls_to(&e, MENU_SET_MENU_TILE),
            vec![vec![menu, 0x6100, 0]]
        );
        assert_eq!(calls_to(&e, MENU_PREP_FOR_VISIBILITY), vec![vec![menu, 0]]);
        assert_eq!(calls_to(&e, CREATE_TEXTURE_IMAGE).len(), 8);
        assert_eq!(
            calls_to(&e, SPRINTF_S)
                .iter()
                .find(|w| w[2] == UNNAMED_FORMAT_010765F0)
                .map(|w| w.len()),
            Some(4)
        );
    }

    #[test]
    fn do_click_spins_and_changes_the_bet() {
        let mut e = engine();
        let menu = world(&mut e);
        set(&mut e, menu, SlotMachineMenu::eSlotState, 4);
        e.call(0x007c_2460, &args![menu, 3u32, 0u32]);
        assert_eq!(get(&e, menu, SlotMachineMenu::eSlotState), 5);
        set(&mut e, menu, SlotMachineMenu::eSlotState, 6);
        set(&mut e, menu, SlotMachineMenu::iChipCount, 100);
        e.set_global(MAX_BET_011A0828, 50u32);
        e.set_global(MIN_BET_011A082C, 5u32);
        for (id, before, after) in [
            (4u32, 5i32, 6i32),
            (4, 45, 50),
            (5, 33, 30),
            (5, 6, 5),
            (7, 10, 50),
        ] {
            set(&mut e, menu, SlotMachineMenu::iCurrentBet, before);
            e.call(0x007c_2460, &args![menu, id, 0u32]);
            assert_eq!(
                get(&e, menu, SlotMachineMenu::iCurrentBet),
                after,
                "{id} {before}"
            );
        }
        // The bet sounds cycle 1, 2, 3, 1.
        set(&mut e, menu, SlotMachineMenu::iNextSoundIndex, 3);
        set(&mut e, menu, SlotMachineMenu::iCurrentBet, 5);
        e.call_log = Some(vec![]);
        e.call(0x007c_2460, &args![menu, 4u32, 0u32]);
        assert_eq!(
            calls_to(&e, SPRINTF_S)[0][2..],
            [BET_SOUND_FORMAT_010767A8, 3]
        );
        assert_eq!(get(&e, menu, SlotMachineMenu::iNextSoundIndex), 1);
        // Spinning: a draw of two wilds and one other.
        set(&mut e, menu, SlotMachineMenu::iNumActiveSymbols, 1);
        e.mem.set_u32(menu + ACTIVE_SYMBOLS, 0xc);
        e.mem.set_u32(menu + ACTIVE_SYMBOLS + 4, 0xc);
        e.register(LUCK_007C58F0, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x007c_2460, &args![menu, 3u32, 0u32]);
        assert_eq!(get(&e, menu, SlotMachineMenu::eSlotState), 1);
        assert_eq!(get(&e, menu, SlotMachineMenu::eSpinResult), 6);
        assert_eq!(calls_to(&e, LUCK_007C58F0), vec![vec![menu, 6]]);
        assert_eq!(e.mem.u8(menu + 0xd8), 1);
        assert_eq!(calls_to(&e, MISC_STAT_INCREMENT), vec![vec![0x2a]]);
        // Too few chips: nothing.
        set(&mut e, menu, SlotMachineMenu::eSlotState, 6);
        set(&mut e, menu, SlotMachineMenu::iChipCount, 1);
        set(&mut e, menu, SlotMachineMenu::iCurrentBet, 5);
        e.call(0x007c_2460, &args![menu, 3u32, 0u32]);
        assert_eq!(get(&e, menu, SlotMachineMenu::eSlotState), 6);
        e.call(0x007c_2460, &args![menu, 6u32, 0u32]);
        assert_eq!(get(&e, menu, SlotMachineMenu::eSlotState), 4);
        set(&mut e, menu, SlotMachineMenu::eSlotState, 6);
        e.call(0x007c_2460, &args![menu, 8u32, 0u32]);
        assert_eq!(get(&e, menu, SlotMachineMenu::eSlotState), 7);
    }

    #[test]
    fn do_idle_pays_and_bets() {
        let mut e = engine();
        let menu = world(&mut e);
        // Paying: three 2s pay 20 times the bet; the message and state 3.
        set(&mut e, menu, SlotMachineMenu::eSlotState, 2);
        set(&mut e, menu, SlotMachineMenu::iChipCount, 100);
        set(&mut e, menu, SlotMachineMenu::iCurrentBet, 3);
        results(&mut e, menu, [2, 2, 2]);
        e.call_log = Some(vec![]);
        e.call(0x007c_2c70, &args![menu]);
        assert_eq!(get(&e, menu, SlotMachineMenu::iChipCount), 160);
        let data = e.global::<u32>(CASINO_DATA_011DA9A8);
        assert_eq!(e.mem.u32(data + 4), 60);
        let message = calls_to(&e, SPRINTF_S)
            .into_iter()
            .find(|w| w[2] == RESULT_FORMAT_01070D0C)
            .unwrap();
        assert_eq!(message[3..], [0x011d_2ff4, 60, 0x011d_5050, 0x011d_3114]);
        assert_eq!(get(&e, menu, SlotMachineMenu::eSlotState), 3);
        // Three of a kind of an unlisted symbol: only the bet is taken, no message win.
        set(&mut e, menu, SlotMachineMenu::eSlotState, 2);
        results(&mut e, menu, [14, 14, 14]);
        e.call(0x007c_2c70, &args![menu]);
        assert_eq!(get(&e, menu, SlotMachineMenu::iChipCount), 157);
        assert_eq!(get(&e, menu, SlotMachineMenu::eSlotState), 3);
        // A loss: the bet is taken, state 6.
        set(&mut e, menu, SlotMachineMenu::eSlotState, 2);
        results(&mut e, menu, [1, 3, 5]);
        e.call(0x007c_2c70, &args![menu]);
        assert_eq!(get(&e, menu, SlotMachineMenu::iChipCount), 154);
        assert_eq!(get(&e, menu, SlotMachineMenu::eSlotState), 6);
        // Betting: more bet than chips drops the bet to 1; no chips closes the menu.
        set(&mut e, menu, SlotMachineMenu::iChipCount, 2);
        e.call(0x007c_2c70, &args![menu]);
        assert_eq!(get(&e, menu, SlotMachineMenu::iCurrentBet), 1);
        set(&mut e, menu, SlotMachineMenu::iChipCount, 0);
        e.call_log = Some(vec![]);
        e.call(0x007c_2c70, &args![menu]);
        assert_eq!(get(&e, menu, SlotMachineMenu::eSlotState), 7);
        assert_eq!(calls_to(&e, SPRINTF_S)[0][2], 0x011d_2424);
        // State 0 goes to betting.
        set(&mut e, menu, SlotMachineMenu::eSlotState, 0);
        e.call(0x007c_2c70, &args![menu]);
        assert_eq!(get(&e, menu, SlotMachineMenu::eSlotState), 6);
    }

    #[test]
    fn do_idle_spins_the_reels_and_plays_the_coins() {
        let mut e = engine();
        let menu = world(&mut e);
        set(&mut e, menu, SlotMachineMenu::eSlotState, 1);
        e.mem.set_u32(menu + 0x138, 0x00ab_7000);
        e.mem.set_u32(menu + 0x13c, 0x00ab_7100);
        e.mem.set_u32(menu + REEL_ANIMATIONS, 0x00ab_7200);
        e.register(SEQUENCE_LENGTH_00508100, |_, _| st0(0.5));
        e.register(TILE_GET_FLOAT, |_, _| st0(5.0));
        e.call_log = Some(vec![]);
        e.call(0x007c_2c70, &args![menu]);
        // At 0.0: the spin sound, the texture swap set, the machine and three reels moved.
        assert_eq!(e.mem.u8(menu + 0xf8), 1);
        assert_eq!(calls_to(&e, SET_TRANSLATE_00A59C60).len(), 4);
        assert_eq!(calls_to(&e, DEACTIVATE_SEQUENCE_0047B220).len(), 1);
        assert_eq!(get(&e, menu, SlotMachineMenu::eSlotState), 1);
        // On top, 600 ms later: past the lever and the reels.
        e.register(slot(1, 0x34), |_, _| 0x438u32.into_ret());
        e.register(INTERFACE_IS_TOP_MENU_ID, |_, _| 1u32.into_ret());
        e.mem.set_u32(menu + 0xf4, 1000);
        e.register(WORD_AT_14_00825C00, |_, _| 1600u32.into_ret());
        e.set_global(LAST_STATE_011DAA2C, 1u32);
        e.call(0x007c_2c70, &args![menu]);
        assert_eq!(e.global::<u32>(SPIN_TIME_011DAA30), 0.6f32.to_bits());
        assert_eq!(e.mem.u8(menu + 0xf8), 0);
        assert_eq!(get(&e, menu, SlotMachineMenu::eSlotState), 2);
        // Coins at 0.0 with result 5: the coin animation and its sound.
        set(&mut e, menu, SlotMachineMenu::eSlotState, 3);
        set(&mut e, menu, SlotMachineMenu::eSpinResult, 5);
        e.set_global(SPIN_TIME_011DAA30, 0.0f32.to_bits());
        e.mem.set_u32(menu + 0xf4, 0);
        e.call_log = Some(vec![]);
        e.call(0x007c_2c70, &args![menu]);
        assert_eq!(
            calls_to(&e, GET_SOUND_HANDLE_BY_NAME)[0][2],
            SOUND_WIN_5_010767E4
        );
        assert_eq!(calls_to(&e, ACTIVATE_SEQUENCE_0047AAB0)[0][1], 0x00ab_7100);
        // Without a coin animation the state goes back to betting.
        e.mem.set_u32(menu + 0x13c, 0);
        e.call(0x007c_2c70, &args![menu]);
        assert_eq!(get(&e, menu, SlotMachineMenu::eSlotState), 6);
        // The payout lists: 4 starts its sequence at 0.0 and moves the machine.
        set(&mut e, menu, SlotMachineMenu::eSlotState, 4);
        e.mem.set_u32(menu + 0x140, 0x00ab_7300);
        e.call_log = Some(vec![]);
        e.call(0x007c_2c70, &args![menu]);
        assert_eq!(calls_to(&e, ACTIVATE_SEQUENCE_0047AAB0)[0][1], 0x00ab_7300);
        assert_eq!(calls_to(&e, SET_TRANSLATE_00A59C60).len(), 1);
        // 5 past its sequence's length starts the lever and goes to betting.
        set(&mut e, menu, SlotMachineMenu::eSlotState, 5);
        e.mem.set_u32(menu + 0x144, 0x00ab_7400);
        e.set_global(SPIN_TIME_011DAA30, 0.75f32.to_bits());
        e.set_global(LAST_STATE_011DAA2C, 5u32);
        e.call_log = Some(vec![]);
        e.call(0x007c_2c70, &args![menu]);
        assert_eq!(
            calls_to(&e, DEACTIVATE_SEQUENCE_0047B220)[0][1],
            0x00ab_7400
        );
        assert_eq!(calls_to(&e, ACTIVATE_SEQUENCE_0047AAB0)[0][1], 0x00ab_7000);
        assert_eq!(get(&e, menu, SlotMachineMenu::eSlotState), 6);
    }

    #[test]
    fn do_idle_closing_settles_the_chips() {
        let mut e = engine();
        let menu = world(&mut e);
        set(&mut e, menu, SlotMachineMenu::eSlotState, 7);
        set(&mut e, menu, SlotMachineMenu::iChipCount, 100);
        e.register(GET_ITEM_COUNT_IN_CONTAINER, |_, _| 70u32.into_ret());
        e.register(CASINO_00737E30, |_, _| 0u32.into_ret());
        let player = e.global::<u32>(PLAYER_011DEA3C);
        e.call_log = Some(vec![]);
        e.call(0x007c_2c70, &args![menu]);
        assert_eq!(calls_to(&e, CONTAINER_ADD)[0][2], 30);
        assert_eq!(calls_to(&e, CONTAINER_GIVE)[0][1..], [player, 1]);
        assert_eq!(calls_to(&e, SHOW_MESSAGE_007052F0)[0][2], WON_ICON_0103A830);
        assert_eq!(calls_to(&e, MENU_START_FADE_OUT), vec![vec![menu]]);
        // The chip text, as the roulette menu's.
        e.mem.set_u32(menu + 0xa8, 0x5a80);
        e.set_global(TRAIT_011DA99C, 0x99u32);
        e.call_log = Some(vec![]);
        e.call(0x007c_5e60, &args![menu]);
        assert_eq!(calls_to(&e, TILE_SET_STRING)[0][..2], [0x5a80, 0x99]);
        assert_eq!(calls_to(&e, SPRINTF_S).len(), 3);
    }
}
