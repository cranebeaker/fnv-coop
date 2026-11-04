//! `fallout/interface/menus/lovetestermenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00792760 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A folded float getter: `FLD dword [this+0x40]`, the value in `ST0`.
/// `Script::Run` (`005ac1e0`) calls it on the owner quest for the quest's
/// script delay. The `FLD` quiets a signaling NaN.
pub fn fn_00792760(e: &mut Engine, this: Ptr) -> f64 {
    e.mem.f32(this.addr() + 0x40) as f64
}

/// The `LoveTesterMenu` instance (`LoveTesterMenu *`, `011da2c8`).
const MENU_INSTANCE: u32 = 0x011d_a2c8;
/// The machine's camera (`NiPointer<NiCamera>`, `011da2f4`).
const CAMERA_POINTER: u32 = 0x011d_a2f4;
/// The player (`PlayerCharacter *`, `011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// The direction of the last page turn (a byte, `011a039c`: 1 forward), which is also the
/// start of the table of forward page sequence names (a word per state, `011a039c`); the
/// backward names are at `011a03c0`.
const TURN_FORWARD: u32 = 0x011a_039c;
const BACKWARD_SEQUENCES: u32 = 0x011a_03c0;
/// The menu's class id, `0x432`.
const MENU_ID: u32 = 0x432;

/// `"Data\Menus\CharGen\love_tester_menu.xml"`.
const MENU_FILE: u32 = 0x0107_46dc;
/// `"MENUS:  Special book menu creation failed..."` (the report format, with the index).
const CREATE_FAILED_MESSAGE: u32 = 0x0107_4698;
/// The two machine models (`"Meshes\architecture\Goodsprings\NV_VitoM..."`).
const DYNAMIC_MODEL: u32 = 0x0107_4750;
const CABINET_MODEL: u32 = 0x0107_4708;
/// `"Surgery3DCamera"`.
const CAMERA_NAME: u32 = 0x0107_0c70;
/// Sounds (`BSAudio::GetSoundHandleByName` names): page turn, finish, refused, value up,
/// value down, choice.
const SOUND_PAGE_TURN: u32 = 0x0107_4ba0;
const SOUND_FINISH: u32 = 0x0107_47f4;
const SOUND_REFUSED: u32 = 0x0107_47e0;
const SOUND_VALUE_UP: u32 = 0x0107_47ac;
const SOUND_VALUE_DOWN: u32 = 0x0107_47c4;
const SOUND_CHOICE: u32 = 0x0107_4798;

/// Callees outside this unit, by address (`thiscall` unless noted).
const GLOBAL_004B7210: u32 = 0x004b_7210;
const OBJECT_00586150: u32 = 0x0058_6150;
const TILE_READ_FILE: u32 = 0x00a0_1b00;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
const REPORT_005B5E40: u32 = 0x005b_5e40;
const EMERGENCY_CLOSE_ALL_MENUS: u32 = 0x0070_37e0;
const INIT_MACHINE: u32 = 0x0079_28e0;
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
const POINTER_CONSTRUCTOR: u32 = 0x0063_3c90;
const POINTER_DESTRUCTOR: u32 = 0x0045_cec0;
const POINTER_GET: u32 = 0x0055_9450;
const SCRIPT_005CFCB0: u32 = 0x005c_fcb0;
const WORD_4_00726070: u32 = 0x0072_6070;
const MODEL_LOADER: u32 = 0x011c_3b3c;
const RELEASE_MODEL_FILE_0045A5E0: u32 = 0x0045_a5e0;
const SCENE_GRAPH_00450B80: u32 = 0x0045_0b80;
const SCENE_RESET_004DEF90: u32 = 0x004d_ef90;
const SHADOW_SCENE_RESET_00B5D180: u32 = 0x00b5_d180;
const ARRAY_SIZE: u32 = 0x0044_ddc0;
const ARRAY_ELEMENT: u32 = 0x006a_7ad0;
const ARRAY_CLEAR: u32 = 0x0084_54f0;
const FREE: u32 = 0x0040_1030;
const TILE_SET_00700320: u32 = 0x0070_0320;
const IS_TOP_MENU_ID: u32 = 0x0070_2450;
const INTERFACE_OBJECT_011DEA0C: u32 = 0x011d_ea0c;
const OBJECT_00877720: u32 = 0x0087_7720;
const SETTING_STRING_00403DF0: u32 = 0x0040_3df0;
const SETTING_011C3CB4: u32 = 0x011c_3cb4;
const COMPARE_00408B20: u32 = 0x0040_8b20;
const COMPARE_STRING_0101FD00: u32 = 0x0101_fd00;
const CONTROL_PRESSED_00A24180: u32 = 0x00a2_4180;
const MODEL_ROOT_0043B230: u32 = 0x0043_b230;
const STRING_CONSTRUCTOR: u32 = 0x0043_8170;
const STRING_DESTRUCTOR: u32 = 0x0043_81b0;
const GET_SEQUENCE_0047A520: u32 = 0x0047_a520;
const FRAME_TIME_0084D030: u32 = 0x0084_d030;
const TIMER_011F6394: u32 = 0x011f_6394;
const SEQUENCE_UPDATE: u32 = 0x00a3_4ba0;
const SEQUENCE_END_00508120: u32 = 0x0050_8120;
const SEQUENCE_TIME_00508100: u32 = 0x0050_8100;
const CONTROLLER_MANAGER_00653270: u32 = 0x0065_3270;
const MANAGER_ARGUMENT_011F36BC: u32 = 0x011f_36bc;
const MANAGER_ARGUMENT_011F36AC: u32 = 0x011f_36ac;
const PARENT_004A8A90: u32 = 0x004a_8a90;
const UPDATE_BLOCK_0043D410: u32 = 0x0043_d410;
const UPDATE_00A59C60: u32 = 0x00a5_9c60;
const GET_TOTAL_POINTS_USED: u32 = 0x0079_4d80;
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_DESTRUCTOR: u32 = 0x0048_3710;
const SOUND_HANDLE_CONSTRUCTOR: u32 = 0x0041_a250;
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
const MIN_004A8F20: u32 = 0x004a_8f20;
const MAX_00647B70: u32 = 0x0064_7b70;
const SET_ACTOR_BASE_VALUE: u32 = 0x0093_a7c0;
const HARDCORE_004B71D0: u32 = 0x004b_71d0;
const CURSOR_OBJECT_004B7210: u32 = 0x004b_7210;
const CURSOR_X_009A9350: u32 = 0x009a_9350;
const FTOL: u32 = 0x00ec_62c0;
const OBJECT_0043C4B0: u32 = 0x0043_c4b0;
const SIZE_0084E3A0: u32 = 0x0084_e3a0;
const WIDESCREEN_011F9426: u32 = 0x011f_9426;
const VIEWPORT_011AD840: u32 = 0x011a_d840;
const POINT_CONSTRUCTOR_006815C0: u32 = 0x0068_15c0;
const PICK_CONSTRUCTOR: u32 = 0x00e9_8f20;
const PICK_DESTRUCTOR: u32 = 0x00e9_8fa0;
const PICK_OBJECTS: u32 = 0x00e9_8e20;
const PICK_SET_00458B30: u32 = 0x0045_8b30;
const PICK_SET_TARGET_00705FC0: u32 = 0x0070_5fc0;
const PICK_SET_006ECD40: u32 = 0x006e_cd40;
const PICK_RESULTS_00500940: u32 = 0x0050_0940;
const RESULTS_GET_00968670: u32 = 0x0096_8670;
const CAMERA_WINDOW_POINT_TO_RAY: u32 = 0x00a7_1080;
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
const CAMERA_CONSTRUCTOR: u32 = 0x00a7_12f0;
const MAKE_COLOR_00414430: u32 = 0x0041_4430;
const CAMERA_SET_COLOR_00712E60: u32 = 0x0071_2e60;
const FRUSTUM_CONSTRUCTOR: u32 = 0x00a7_1b70;
const MAIN_CAMERA_0045C670: u32 = 0x0045_c670;
const SETTING_VALUE: u32 = 0x0040_3e20;
const SETTING_01203144: u32 = 0x0120_3144;
const SETTING_01203150: u32 = 0x0120_3150;
const CAMERA_SET_RATIO_00507700: u32 = 0x0050_7700;
const SCREEN_WIDTH_004DC200: u32 = 0x004d_c200;
const SCREEN_HEIGHT_004DC1F0: u32 = 0x004d_c1f0;
const WIDESCREEN_WIDTH_011F9434: u32 = 0x011f_9434;
const FRUSTUM_SCALE_01016264: u32 = 0x0101_6264;
const DEGREES_TO_RADIANS_01023128: u32 = 0x0102_3128;
const HALF_ANGLE_01074B98: u32 = 0x0107_4b98;
const TANGENT_0057DD70: u32 = 0x0057_dd70;
const SET_VIEW_FRUSTUM: u32 = 0x00a6_faf0;
const SET_TRANSLATE_004BC1F0: u32 = 0x004b_c1f0;
const POINT_00416870: u32 = 0x0041_6870;
const LOOK_AT_WORLD_POINT: u32 = 0x00a7_01b0;
const SET_SCALE_00733FD0: u32 = 0x0073_3fd0;
const SET_NAME_00A5B950: u32 = 0x00a5_b950;
const INTERFACE_007148C0: u32 = 0x0071_48c0;
const SCENE_008D80E0: u32 = 0x008d_80e0;
const SCENE_SET_00C4F270: u32 = 0x00c4_f270;
const SCENE_RESET_00C4F2D0: u32 = 0x00c4_f2d0;
const RENDER_TO_TEXTURE_00709D50: u32 = 0x0070_9d50;
const RENDER_VALUE_01023934: u32 = 0x0102_3934;
const RENDERED_MENU_007FD640: u32 = 0x007f_d640;
const SCENE_SET_NODE_004A0FD0: u32 = 0x004a_0fd0;
const ACCUMULATE_SCENE: u32 = 0x00b6_bee0;
const RENDER_00B6C0D0: u32 = 0x00b6_c0d0;
const GET_CONTROLLER: u32 = 0x00a5_c570;
const DEACTIVATE_ALL: u32 = 0x0048_fef0;
const ACTIVATE_SEQUENCE_0047AAB0: u32 = 0x0047_aab0;
const SET_FLAG_0047AA40: u32 = 0x0047_aa40;
const SEQUENCE_START_004954C0: u32 = 0x0049_54c0;
const GET_PROPERTY: u32 = 0x00a5_9d30;
const PROPERTY_TYPE_00441110: u32 = 0x0044_1110;
const ARRAY_CONSTRUCTOR_00723FA0: u32 = 0x0072_3fa0;
const ARRAY_ADD_007CB2E0: u32 = 0x007c_b2e0;
const SET_CULLED_00450F90: u32 = 0x0045_0f90;
const FORMAT: u32 = 0x0040_6d00;
const SWAP_LANGUAGE_TEXTURE_PATH: u32 = 0x004b_7240;
const CREATE_TEXTURE_IMAGE: u32 = 0x0045_68c0;
const TES_011DEA10: u32 = 0x011d_ea10;
const LANGUAGE_00709D40: u32 = 0x0070_9d40;
const STRING_COPY_00406D30: u32 = 0x0040_6d30;

/// The value of the player's actor value `index` (the object at the player's +0xa4, its
/// slot +0x1c).
fn player_value(e: &mut Engine, index: u32) -> i32 {
    let player = e.mem.u32(PLAYER);
    e.vcall(player.wrapping_add(0xa4), 0x1c, &args![index])
        .i32()
}

/// The menu instance.
fn menu(e: &Engine) -> u32 {
    e.mem.u32(MENU_INSTANCE)
}

/// `0044ddc0`: the size of a `BSSimpleArray` (its word +8).
fn array_size(e: &mut Engine, array: u32) -> u32 {
    e.call(ARRAY_SIZE, &args![array]).u32()
}

/// `*006a7ad0(array, index)`: an element of a `BSSimpleArray`.
fn element(e: &mut Engine, array: u32, index: u32) -> u32 {
    let slot = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
    e.mem.u32(slot)
}

/// Plays a sound once: a 12-byte handle on the stack from
/// `BSAudio::GetSoundHandleByName` (flags 0x121), `Play` (with 0), then destroyed.
fn play_sound(e: &mut Engine, name: u32) {
    e.with_stack(0xc, |e, handle| {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let filled = e
            .call(
                GET_SOUND_HANDLE_BY_NAME,
                &args![audio, handle, name, 0x121u32],
            )
            .u32();
        e.call(SOUND_HANDLE_PLAY, &args![filled, 0u32]);
        e.call(SOUND_HANDLE_DESTRUCTOR, &args![handle]);
    });
}

/// Plays a sound through a constructed handle (`0041a250`) the found one is assigned to
/// (`00418900`): the found handle is destroyed, the handle played (with 0) and destroyed.
fn play_sound_held(e: &mut Engine, name: u32) {
    e.with_stack(0x18, |e, locals| {
        let handle = locals.addr();
        let found = locals.addr() + 0xc;
        e.call(SOUND_HANDLE_CONSTRUCTOR, &args![handle]);
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let filled = e
            .call(
                GET_SOUND_HANDLE_BY_NAME,
                &args![audio, found, name, 0x121u32],
            )
            .u32();
        e.call(SOUND_HANDLE_ASSIGN, &args![handle, filled]);
        e.call(SOUND_HANDLE_DESTRUCTOR, &args![found]);
        e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
        e.call(SOUND_HANDLE_DESTRUCTOR, &args![handle]);
    });
}

/// The controller sequence of the menu's state for the last turn direction, or null:
/// none when the turn was forward and the state is 0 or less; else the name from the
/// forward (`011a039c`) or backward (`011a03c0`) table, looked up with `0047a520` through a
/// stack string.
fn page_sequence(e: &mut Engine, this: u32, manager: u32) -> u32 {
    let state = e.mem.i32(this + 0x48);
    if e.mem.u8(TURN_FORWARD) != 0 && state <= 0 {
        return 0;
    }
    let table = if e.mem.u8(TURN_FORWARD) != 0 {
        TURN_FORWARD
    } else {
        BACKWARD_SEQUENCES
    };
    let state = e.mem.u32(this + 0x48);
    let name = e.mem.u32(table.wrapping_add(state.wrapping_mul(4)));
    e.with_stack(4, |e, string| {
        let name = e.call(STRING_CONSTRUCTOR, &args![string, name]).u32();
        let sequence = e.call(GET_SEQUENCE_0047A520, &args![manager, name]).u32();
        e.call(STRING_DESTRUCTOR, &args![string]);
        sequence
    })
}

/// The object's shader property: its property 3 (`NiAVObject::GetProperty`) when that
/// exists and its type (`00441110`) is 8 to 12, else null.
fn shader_property(e: &mut Engine, object: u32) -> u32 {
    if e.call(GET_PROPERTY, &args![object, 3u32]).u32() == 0 {
        return 0;
    }
    let property = e.call(GET_PROPERTY, &args![object, 3u32]).u32();
    let kind = e.call(PROPERTY_TYPE_00441110, &args![property]).i32();
    if !(8..=0xc).contains(&kind) {
        return 0;
    }
    e.call(GET_PROPERTY, &args![object, 3u32]).u32()
}

/// The child of `root` named `name` (its slot +0x9c with a stack string).
fn object_by_name(e: &mut Engine, root: u32, name: u32) -> u32 {
    e.with_stack(4, |e, string| {
        let name = e.call(STRING_CONSTRUCTOR, &args![string, name]).u32();
        let object = e.vcall(root, 0x9c, &args![name]).u32();
        e.call(STRING_DESTRUCTOR, &args![string]);
        object
    })
}

// Translated from 007915f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoveTesterMenu::Create_ov2` (engine map; `cdecl`, one word: the points to spend).
/// Reads the menu file (`Tile::ReadFile` with `"Data\Menus\CharGen\love_tester_menu.xml"`
/// on `00586150` of `004b7210`) and takes its menu (`Tile::GetMenu`). A love tester menu
/// (slot +0x34 answering [`fn_00791740`]) gets its tile (`Menu::SetMenuTile`) and becomes
/// the instance (`011da2c8`); when one of its first eight tiles is missing
/// ([`fn_00791750`]) the failure is reported with the index (`005b5e40`), the tile deleted
/// (slot 0 with 1) and all menus closed (`Interface::EmergencyCloseAllMenusAndBreakStuff`).
/// Otherwise it keeps the points (+0x58), builds the machine (`LoveTesterMenu::InitMachine`,
/// `007928e0`) and its camera ([`love_tester_menu_init_camera`], kept at `011da2f4`), runs
/// `005cfcb0(1)` and is prepared for visibility. Any other menu found is deleted when its
/// word +4 is set.
pub fn love_tester_menu_create(e: &mut Engine, points: u32) {
    let object = e.call(GLOBAL_004B7210, &args![]).u32();
    let object = e.call(OBJECT_00586150, &args![object]).u32();
    let tile = e.call(TILE_READ_FILE, &args![object, MENU_FILE]).u32();
    let found = e.call(TILE_GET_MENU, &args![tile]).u32();
    if found != 0 && e.vcall(found, 0x34, &args![]).u32() == fn_00791740(e) {
        e.call(MENU_SET_MENU_TILE, &args![found, tile, 0u32]);
        e.mem.set_u32(MENU_INSTANCE, found);
        let (complete, missing) = e.with_stack(4, |e, index| {
            let instance = menu(e);
            let complete = fn_00791750(e, Ptr::new(instance), index);
            (complete, e.mem.u32(index.addr()))
        });
        if !complete {
            e.call(REPORT_005B5E40, &args![CREATE_FAILED_MESSAGE, missing]);
            if tile != 0 {
                e.vcall(tile, 0, &args![1u32]);
            }
            e.call(EMERGENCY_CLOSE_ALL_MENUS, &args![]);
            return;
        }
        let instance = menu(e);
        e.mem.set_u32(instance + 0x58, points);
        let instance = menu(e);
        e.call(INIT_MACHINE, &args![instance]);
        let instance = menu(e);
        let camera = love_tester_menu_init_camera(e, Ptr::new(instance));
        e.call(POINTER_ASSIGN, &args![CAMERA_POINTER, camera]);
        e.call(SCRIPT_005CFCB0, &args![1u32]);
        e.call(MENU_PREP_FOR_VISIBILITY, &args![found, 0u32]);
        return;
    }
    if found != 0 && e.call(WORD_4_00726070, &args![found]).u32() != 0 {
        e.vcall(found, 0, &args![1u32]);
    }
}

// Translated from 00791740 (decompiled, FalloutNV.exe 1.4.0.525)
/// The menu's class id (unnamed in the engine map; reads nothing): 0x432.
pub fn fn_00791740(_e: &mut Engine) -> u32 {
    MENU_ID
}

// Translated from 00791750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: the address of an index word): whether
/// the first eight tiles (`pTiles`, +0x28) are all set, counting in the word; at a missing
/// one the word holds its index and the answer is false.
pub fn fn_00791750(e: &mut Engine, this: Ptr, index: Ptr) -> bool {
    e.mem.set_u32(index.addr(), 0);
    while e.mem.i32(index.addr()) < 8 {
        let i = e.mem.u32(index.addr());
        if e.mem.u32(this.addr() + i.wrapping_mul(4) + 0x28) == 0 {
            return false;
        }
        e.mem.set_u32(index.addr(), i.wrapping_add(1));
    }
    true
}

/// Deletes every button of `buttons` ([`fn_00791a80`] with 1) and frees every text item of
/// `texts`, then clears both arrays (`008454f0` with 1).
fn free_page(e: &mut Engine, buttons: u32, texts: u32) {
    let mut i = 0u32;
    while i < array_size(e, buttons) {
        let button = element(e, buttons, i);
        if button != 0 {
            fn_00791a80(e, Ptr::new(button), 1);
        }
        i += 1;
    }
    let mut i = 0u32;
    while i < array_size(e, texts) {
        let text = element(e, texts, i);
        e.call(FREE, &args![text]);
        i += 1;
    }
    e.call(ARRAY_CLEAR, &args![buttons, 1u32]);
    e.call(ARRAY_CLEAR, &args![texts, 1u32]);
}

// Translated from 007917a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no words): closes the machine. With the menu's tile
/// (`Tile::GetMenuByClass(0x432)`): both machine models are released (`0045a5e0`), the
/// scene graph (`00450b80(1)`) is reset (`004def90`, `00b5d180`), the menu's models (+0x5c,
/// +0x60) and the camera cleared, the nine pages' and the global page's buttons and text
/// items freed, the bulbs and glows cleared, and, while the instance exists, the tile gets
/// `00700320(0x1772, 1)` and the menu fades out (`Menu::StartFadeOut`).
pub fn fn_007917a0(e: &mut Engine) {
    let id = fn_00791740(e);
    let tile = e.call(TILE_GET_MENU_BY_CLASS, &args![id]).u32();
    if tile == 0 {
        return;
    }
    for model in [DYNAMIC_MODEL, CABINET_MODEL] {
        let loader = e.mem.u32(MODEL_LOADER);
        e.call(RELEASE_MODEL_FILE_0045A5E0, &args![loader, model]);
    }
    let graph = e.call(SCENE_GRAPH_00450B80, &args![1u32]).u32();
    e.call(SCENE_RESET_004DEF90, &args![graph]);
    let graph = e.call(SCENE_GRAPH_00450B80, &args![1u32]).u32();
    e.call(SHADOW_SCENE_RESET_00B5D180, &args![graph]);
    let instance = menu(e);
    e.mem.set_u32(instance + 0x5c, 0);
    let instance = menu(e);
    e.mem.set_u32(instance + 0x60, 0);
    e.call(POINTER_ASSIGN, &args![CAMERA_POINTER, 0u32]);
    for page in 0..9u32 {
        let buttons = menu(e) + (page << 5) + 0x84;
        let texts = menu(e) + (page << 5) + 0x94;
        free_page(e, buttons, texts);
    }
    let buttons = menu(e) + 0x1a4;
    let texts = menu(e) + 0x1b4;
    free_page(e, buttons, texts);
    for array in [0x64u32, 0x74] {
        let array = menu(e) + array;
        e.call(ARRAY_CLEAR, &args![array, 1u32]);
    }
    if menu(e) != 0 {
        e.call(TILE_SET_00700320, &args![tile, 0x1772u32, 1u32]);
        let instance = menu(e);
        e.call(MENU_START_FADE_OUT, &args![instance]);
    }
}

// Translated from 00791a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `LoveTesterButton` scalar deleting destructor (unnamed in the engine map;
/// `thiscall`, `RET 4`: the flags): runs [`fn_00795dc0`] and frees the button
/// (`00401030`) when bit 0 of the flags is set. Returns `this`.
pub fn fn_00791a80(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00795dc0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00791ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoveTesterMenu::DoIdle` (engine map; `thiscall`). Only while the menu is on top
/// (`Interface::IsTopMenuID(0x432)`). When the control 0x1e (0x10 when the string of the
/// `Setting` at `011c3cb4` equals the one at `0101fd00`, `00408b20`) is pressed
/// (`00a24180` on `00877720` of the object at `011dea0c`), the menu's slot +0xc gets 4 and
/// 0 and nothing else happens. Otherwise, with the dynamic model's root (`0043b230`): the
/// page's sequence (see [`page_sequence`]) is advanced to the frame timer (`fFrameAnimTimer`
/// +0x50 plus `0084d030` on `011f6394`, `NiControllerSequence::Update` with 1);
/// `bAnimFinished` (+0x54) is set unless the sequence's time (`00508100`) is past its end
/// (`00508120`) on a state above 0, and set when there is no sequence. The first node up
/// from the root (`004a8a90`) with a controller manager (`00653270` with `011f36bc`) gets
/// an update at the timer (its slot +0x94 with a 12-byte block from `0043d410`). Then the
/// model gets an update at 0.0 (`00a59c60`).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00791ab0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn love_tester_menu_do_idle(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.call(IS_TOP_MENU_ID, &args![MENU_ID]).u8() == 0 {
        return;
    }
    let interface = e.mem.u32(INTERFACE_OBJECT_011DEA0C);
    let input = e.call(OBJECT_00877720, &args![interface]).u32();
    let mut control = 0x1eu32;
    let setting = e
        .call(SETTING_STRING_00403DF0, &args![SETTING_011C3CB4])
        .u32();
    if e.call(COMPARE_00408B20, &args![setting, COMPARE_STRING_0101FD00])
        .u32()
        == 0
    {
        control = 0x10;
    }
    if e.call(CONTROL_PRESSED_00A24180, &args![input, control, 1u32])
        .u32()
        != 0
    {
        e.vcall(base, 0xc, &args![4u32, 0u32]);
        return;
    }
    e.with_stack(0x18, |e, blocks| {
        let model = e.mem.u32(base + 0x5c);
        let root = e.call(MODEL_ROOT_0043B230, &args![model]).u32();
        if root != 0 {
            let sequence = page_sequence(e, base, root);
            let elapsed = e.call(FRAME_TIME_0084D030, &args![TIMER_011F6394]).f64();
            let timer = x87_store32(x87_add(elapsed, x87_m32(e.mem.u32(base + 0x50))));
            e.mem.set_u32(base + 0x50, timer);
            if sequence != 0 {
                let timer = x87_quiet_bits(e.mem.u32(base + 0x50));
                e.call(SEQUENCE_UPDATE, &args![sequence, timer, 1u32]);
                let end = e.call(SEQUENCE_END_00508120, &args![sequence]).f64();
                let time = e.call(SEQUENCE_TIME_00508100, &args![sequence]).f64();
                // FCOMP double; TEST AH,0x41; JNZ: finished unless time > end.
                let finished = !(time > end && e.mem.i32(base + 0x48) > 0);
                e.mem.set_u8(base + 0x54, u8::from(finished));
            } else {
                e.mem.set_u8(base + 0x54, 1);
            }
            let mut node = root;
            while node != 0 {
                let manager = e
                    .call(
                        CONTROLLER_MANAGER_00653270,
                        &args![MANAGER_ARGUMENT_011F36BC, node],
                    )
                    .u32();
                if manager != 0 {
                    let timer = x87_quiet_bits(e.mem.u32(base + 0x50));
                    e.call(UPDATE_BLOCK_0043D410, &args![blocks, timer, 0u32, 0u32]);
                    e.vcall(node, 0x94, &args![blocks]);
                    break;
                }
                node = e.call(PARENT_004A8A90, &args![node]).u32();
            }
        }
        let block = blocks.addr() + 0xc;
        e.call(UPDATE_BLOCK_0043D410, &args![block, 0.0f32, 0u32, 0u32]);
        let model = e.mem.u32(base + 0x5c);
        e.call(UPDATE_00A59C60, &args![model, block]);
    });
}

/// The value buttons' change on a page with a value (`index`, which the code fetches again
/// from [`fn_00794d30`] before each later use): `id` 2 raises it (to at most 10,
/// `004a8f20`) while points remain, 3 lowers it (to at least 1, `00647b70`), through
/// `PlayerCharacter::SetActorBaseValue`; answers the value before and after (the value
/// owner's slot 0).
fn change_value(e: &mut Engine, this: u32, id: u32, index: u32) -> (i32, i32) {
    let before = player_value(e, index);
    if id == 2 {
        let used = e.call(GET_TOTAL_POINTS_USED, &args![this]).i32();
        if used < e.mem.i32(this + 0x58) {
            let value = e
                .call(MIN_004A8F20, &args![before.wrapping_add(1), 10u32])
                .u32();
            let index = fn_00794d30(e, Ptr::new(this)) as u32;
            let player = e.mem.u32(PLAYER);
            e.call(SET_ACTOR_BASE_VALUE, &args![player, index, value]);
        }
    } else {
        let value = e
            .call(MAX_00647B70, &args![before.wrapping_sub(1), 1u32])
            .u32();
        let index = fn_00794d30(e, Ptr::new(this)) as u32;
        let player = e.mem.u32(PLAYER);
        e.call(SET_ACTOR_BASE_VALUE, &args![player, index, value]);
    }
    let index = fn_00794d30(e, Ptr::new(this)) as u32;
    let player = e.mem.u32(PLAYER);
    let after = e.vcall(player.wrapping_add(0xa4), 0, &args![index]).i32();
    (before, after)
}

/// Picks the buttons of `buttons` under the cursor point: for each, a stack `NiPick`
/// (`00e98f20(0, 8)`, `00458b30(1)`, the button's object, `006ecd40(0)`, [`fn_00792780`]
/// with 1) gets the camera's ray through the point (`00a71080`); the first button whose
/// object is hit (`NiPick::PickObjects`, a result from `00500940`/`00968670`) is clicked
/// (its `pfButtonClick`, `cdecl`). Each pick is destroyed (`NiPick::~NiPick`).
fn pick_buttons(e: &mut Engine, buttons: impl Fn(&Engine) -> u32, x: i32, y: i32) {
    e.with_stack(0x50, |e, locals| {
        let pick = locals.addr();
        let origin = locals.addr() + 0x38;
        let direction = locals.addr() + 0x44;
        let mut i = 0u32;
        loop {
            let array = buttons(e);
            if i >= array_size(e, array) {
                break;
            }
            e.call(POINT_CONSTRUCTOR_006815C0, &args![origin]);
            e.call(POINT_CONSTRUCTOR_006815C0, &args![direction]);
            e.call(PICK_CONSTRUCTOR, &args![pick, 0u32, 8u32]);
            e.call(PICK_SET_00458B30, &args![pick, 1u32]);
            let array = buttons(e);
            let button = element(e, array, i);
            let object = e.call(POINTER_GET, &args![button + 8]).u32();
            e.call(PICK_SET_TARGET_00705FC0, &args![pick, object]);
            e.call(PICK_SET_006ECD40, &args![pick, 0u32]);
            fn_00792780(e, Ptr::new(pick), 1);
            let camera = e.call(POINTER_GET, &args![CAMERA_POINTER]).u32();
            e.call(
                CAMERA_WINDOW_POINT_TO_RAY,
                &args![camera, x, y, origin, direction, 0u32],
            );
            if e.call(PICK_OBJECTS, &args![pick, origin, direction, 0u32])
                .u8()
                != 0
            {
                let results = e.call(PICK_RESULTS_00500940, &args![pick]).u32();
                if e.call(RESULTS_GET_00968670, &args![results, 0u32]).u32() != 0 {
                    let array = buttons(e);
                    let button = element(e, array, i);
                    let array = buttons(e);
                    let button_again = element(e, array, i);
                    let click = e.mem.u32(button_again);
                    e.call(click, &args![button]);
                    e.call(PICK_DESTRUCTOR, &args![pick]);
                    break;
                }
            }
            e.call(PICK_DESTRUCTOR, &args![pick]);
            i += 1;
        }
    });
}

// Translated from 00791e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoveTesterMenu::DoClick` (engine map; `thiscall`, `RET 8`: the clicked id, an unread
/// word). By id:
///
/// - 0, 1: once the page animation is finished, the page turns (forward for 0,
///   [`love_tester_menu_turn_page`]) and is refreshed;
/// - 2, 3: the page's value (or the summary's chosen value when hardcore applies on the
///   summary page) is raised or lowered ([`change_value`]); a value that went down plays
///   the down sound, one that went up the up sound; the page is refreshed;
/// - 4: with all points spent the finish sound plays and the machine closes
///   ([`fn_007917a0`]), else the refused sound;
/// - 5: on a page (state at most 8) the cursor point (`009a9350` and the float +0x40 of
///   `004b7210`, truncated; in widescreen the y adjusted by the viewport at `011ad840`)
///   picks the page's buttons and then the global page's ([`pick_buttons`]);
/// - 6, 7: on the summary page the chosen value (+0x4c) moves down (above 1) or up (below
///   7) with the choice sound; the page is refreshed;
/// - anything else: nothing.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00791e40 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn love_tester_menu_do_click(e: &mut Engine, this: Ptr, id: u32, _unused_2: u32) {
    let base = this.addr();
    match id {
        0 | 1 => {
            if e.mem.u8(base + 0x54) != 0 {
                love_tester_menu_turn_page(e, this, id == 0);
                love_tester_menu_refresh_page(e, this);
            }
        }
        2 | 3 => {
            let mut values = (0i32, 0i32);
            if fn_00794d30(e, this) != -1 {
                let index = fn_00794d30(e, this) as u32;
                values = change_value(e, base, id, index);
            } else if e.call(HARDCORE_004B71D0, &args![]).u8() != 0 && e.mem.i32(base + 0x48) == 8 {
                let index = e.mem.u32(base + 0x4c).wrapping_add(4);
                values = summary_change(e, base, id, index);
            }
            let (before, after) = values;
            if before > after {
                play_sound(e, SOUND_VALUE_DOWN);
            }
            if before < after {
                play_sound(e, SOUND_VALUE_UP);
            }
            love_tester_menu_refresh_page(e, this);
        }
        4 => {
            let used = e.call(GET_TOTAL_POINTS_USED, &args![this]).u32();
            if used == e.mem.u32(base + 0x58) {
                play_sound(e, SOUND_FINISH);
                fn_007917a0(e);
            } else {
                play_sound(e, SOUND_REFUSED);
            }
        }
        5 => {
            if e.mem.i32(base + 0x48) > 8 {
                return;
            }
            let cursor = e.call(CURSOR_OBJECT_004B7210, &args![]).u32();
            let x = e.call(CURSOR_X_009A9350, &args![cursor]).f64();
            let x = e.call(FTOL, &args![x]).i32();
            let y = fn_00792760(e, Ptr::new(cursor));
            let mut y = e.call(FTOL, &args![y]).i32();
            if e.mem.u8(WIDESCREEN_011F9426) != 0 {
                let object = e.call(OBJECT_0043C4B0, &args![]).u32();
                let top = x87_load32(e.mem.u32(VIEWPORT_011AD840 + 0xc));
                let window = e.vcall(object, 0xd4, &args![]).u32();
                let height = e.call(SIZE_0084E3A0, &args![window]).u32();
                let scaled = x87_mul(f64::from(height), top);
                let span = x87_sub(
                    x87_m32(e.mem.u32(VIEWPORT_011AD840 + 8)),
                    x87_load32(e.mem.u32(VIEWPORT_011AD840 + 0xc)),
                );
                let span = x87_mul(span, f64::from(y));
                y = e.call(FTOL, &args![x87_add(scaled, span)]).i32();
            }
            pick_buttons(
                e,
                move |e| base + (e.mem.u32(base + 0x48) << 5) + 0x84,
                x,
                y,
            );
            pick_buttons(e, move |_| base + 0x1a4, x, y);
        }
        6 => {
            if e.mem.i32(base + 0x48) == 8 && e.mem.i32(base + 0x4c) > 1 {
                let chosen = e.mem.u32(base + 0x4c);
                e.mem.set_u32(base + 0x4c, chosen.wrapping_sub(1));
                play_sound(e, SOUND_CHOICE);
            }
            love_tester_menu_refresh_page(e, this);
        }
        7 => {
            if e.mem.i32(base + 0x48) == 8 && e.mem.i32(base + 0x4c) < 7 {
                let chosen = e.mem.u32(base + 0x4c);
                e.mem.set_u32(base + 0x4c, chosen.wrapping_add(1));
                play_sound(e, SOUND_CHOICE);
            }
            love_tester_menu_refresh_page(e, this);
        }
        _ => {}
    }
}

/// [`change_value`] for the summary page's fixed `index` (the code does not fetch it
/// again).
fn summary_change(e: &mut Engine, this: u32, id: u32, index: u32) -> (i32, i32) {
    let before = player_value(e, index);
    let player = e.mem.u32(PLAYER);
    if id == 2 {
        let used = e.call(GET_TOTAL_POINTS_USED, &args![this]).i32();
        if used < e.mem.i32(this + 0x58) {
            let value = e
                .call(MIN_004A8F20, &args![before.wrapping_add(1), 10u32])
                .u32();
            e.call(SET_ACTOR_BASE_VALUE, &args![player, index, value]);
        }
    } else {
        let value = e
            .call(MAX_00647B70, &args![before.wrapping_sub(1), 1u32])
            .u32();
        e.call(SET_ACTOR_BASE_VALUE, &args![player, index, value]);
    }
    let player = e.mem.u32(PLAYER);
    let after = e.vcall(player.wrapping_add(0xa4), 0, &args![index]).i32();
    (before, after)
}

// Translated from 00792780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: a byte): stores the byte at `this +
/// 0x30` (the love tester's picks use it on an `NiPick`).
pub fn fn_00792780(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x30, value);
}

// Translated from 007945f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoveTesterMenu::InitCamera` (engine map; `thiscall`): makes the machine's camera (0x114
/// bytes, `00a712f0`; null when the allocation fails) and returns it. Its colour is (0, 1,
/// 1, 0) (`00414430`), in widescreen the viewport words at `011ad840` (`00712e60`). The
/// frustum (`00a71b70(0)`): near is the value of the `Setting` at `01203144`, far the main
/// camera's (`0045c670`) slot +0x100, and their ratio is set (`00507700`); the aspect is
/// the screen width (`004dc200`, in widescreen the word at `011f9434`) over the height
/// (`004dc1f0`); from the half angle (the `Setting` at `01203150` times `01023128` and
/// `01074b98`) through `0057dd70` the sides are `-t*s`, `t*s`, `t*aspect*s`, `-t*aspect*s`
/// (left, right, top, bottom; `s` the float at `01016264`), not orthographic
/// (`NiCamera::SetViewFrustum`). It is placed at (0, 0, 1) (`004bc1f0`), updated at 0.0,
/// looks at the origin with up (0, 1, 0) (`NiCamera::LookAtWorldPoint`), gets
/// `00733fd0(1.0)` and the name `"Surgery3DCamera"` (`00a5b950`).
///
/// x87: each instruction is one helper call, in the exe's operand order.
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 007945f0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn love_tester_menu_init_camera(e: &mut Engine, _this: Ptr) -> u32 {
    let block = e.call(ALLOCATE_00AA13E0, &args![0x114u32]).u32();
    let camera = if block != 0 {
        e.call(CAMERA_CONSTRUCTOR, &args![block]).u32()
    } else {
        0
    };
    e.with_stack(0x58, |e, locals| {
        let color = locals.addr();
        let frustum = locals.addr() + 0x10;
        let update = locals.addr() + 0x2c;
        let up = locals.addr() + 0x38;
        let target = locals.addr() + 0x44;
        let name = locals.addr() + 0x50;
        e.call(
            MAKE_COLOR_00414430,
            &args![color, 0.0f32, 1.0f32, 1.0f32, 0.0f32],
        );
        if e.mem.u8(WIDESCREEN_011F9426) != 0 {
            for i in 0..4 {
                let word = e.mem.u32(VIEWPORT_011AD840 + 4 * i);
                e.mem.set_u32(color + 4 * i, word);
            }
        }
        e.call(CAMERA_SET_COLOR_00712E60, &args![camera, color]);
        e.call(FRUSTUM_CONSTRUCTOR, &args![frustum, 0u32]);
        let main = e.call(MAIN_CAMERA_0045C670, &args![]).u32();
        let far = x87_store32(e.vcall(main, 0x100, &args![]).f64());
        e.mem.set_u32(frustum + 0x14, far);
        let near = e.call(SETTING_VALUE, &args![SETTING_01203144]).u32();
        let near = x87_quiet_bits(e.mem.u32(near));
        e.mem.set_u32(frustum + 0x10, near);
        let ratio = x87_store32(x87_div(x87_load32(far), x87_m32(near)));
        e.call(CAMERA_SET_RATIO_00507700, &args![camera, ratio]);
        let width = if e.mem.u8(WIDESCREEN_011F9426) != 0 {
            e.mem.i32(WIDESCREEN_WIDTH_011F9434)
        } else {
            e.call(SCREEN_WIDTH_004DC200, &args![]).i32()
        };
        let height = e.call(SCREEN_HEIGHT_004DC1F0, &args![]).i32();
        let aspect = x87_store32(x87_div(f64::from(width), f64::from(height)));
        let scale = x87_quiet_bits(e.mem.u32(FRUSTUM_SCALE_01016264));
        let angle = e.call(SETTING_VALUE, &args![SETTING_01203150]).u32();
        let angle = x87_mul(
            x87_load32(e.mem.u32(angle)),
            e.mem.f64(DEGREES_TO_RADIANS_01023128),
        );
        let angle = x87_store32(x87_mul(angle, e.mem.f64(HALF_ANGLE_01074B98)));
        let tangent = |e: &mut Engine| {
            e.call(TANGENT_0057DD70, &args![x87_quiet_bits(angle)])
                .f64()
        };
        let t = tangent(e);
        let left = x87_store32(x87_mul(-t, x87_m32(scale)));
        e.mem.set_u32(frustum, left);
        let t = tangent(e);
        let right = x87_store32(x87_mul(t, x87_m32(scale)));
        e.mem.set_u32(frustum + 4, right);
        let t = tangent(e);
        let bottom = x87_store32(x87_mul(x87_mul(-t, x87_m32(aspect)), x87_m32(scale)));
        e.mem.set_u32(frustum + 0xc, bottom);
        let t = tangent(e);
        let top = x87_store32(x87_mul(x87_mul(t, x87_m32(aspect)), x87_m32(scale)));
        e.mem.set_u32(frustum + 8, top);
        e.mem.set_u8(frustum + 0x18, 0);
        e.call(SET_VIEW_FRUSTUM, &args![camera, frustum]);
        e.call(
            SET_TRANSLATE_004BC1F0,
            &args![camera, 0.0f32, 0.0f32, 1.0f32],
        );
        e.call(UPDATE_BLOCK_0043D410, &args![update, 0.0f32, 0u32, 0u32]);
        e.call(UPDATE_00A59C60, &args![camera, update]);
        let up = e
            .call(POINT_00416870, &args![up, 0.0f32, 1.0f32, 0.0f32])
            .u32();
        let target = e
            .call(POINT_00416870, &args![target, 0.0f32, 0.0f32, 0.0f32])
            .u32();
        e.call(LOOK_AT_WORLD_POINT, &args![camera, target, up]);
        e.call(SET_SCALE_00733FD0, &args![camera, 1.0f32]);
        let string = e.call(STRING_CONSTRUCTOR, &args![name, CAMERA_NAME]).u32();
        e.call(SET_NAME_00A5B950, &args![camera, string]);
        e.call(STRING_DESTRUCTOR, &args![name]);
    });
    camera
}

// Translated from 007948d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no words): renders the machine when the instance and its
/// camera exist: `007148c0(0, 4)` on `0043c4b0`; the scene (`008d80e0` of `0045c670`) gets
/// `00c4f270(1)`; the camera is set up for rendering to a texture (`00709d50` with the float
/// at `01023934`); a stack `NiPointer` holds the rendered menu's node (`007fd640` of
/// `004b7210`), which the scene takes (`004a0fd0`) while the scene is accumulated
/// (`BSShaderUtil::AccumulateScene` with the scene graph `00450b80(1)`) and rendered
/// (`00b6c0d0` with 0), then the scene drops it (`004a0fd0(0)`) and is reset (`00c4f2d0`).
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 007948d0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_007948d0(e: &mut Engine) {
    if menu(e) == 0 || e.call(POINTER_GET, &args![CAMERA_POINTER]).u32() == 0 {
        return;
    }
    let object = e.call(OBJECT_0043C4B0, &args![]).u32();
    e.call(INTERFACE_007148C0, &args![object, 0u32, 4u32]);
    let main = e.call(MAIN_CAMERA_0045C670, &args![]).u32();
    let scene = e.call(SCENE_008D80E0, &args![main]).u32();
    e.call(SCENE_SET_00C4F270, &args![scene, 1u32]);
    let camera = e.call(POINTER_GET, &args![CAMERA_POINTER]).u32();
    let value = x87_quiet_bits(e.mem.u32(RENDER_VALUE_01023934));
    e.call(RENDER_TO_TEXTURE_00709D50, &args![camera, value]);
    e.with_stack(4, |e, node| {
        let object = e.call(GLOBAL_004B7210, &args![]).u32();
        let rendered = e.call(RENDERED_MENU_007FD640, &args![object]).u32();
        e.call(POINTER_CONSTRUCTOR, &args![node, rendered]);
        let held = e.call(POINTER_GET, &args![node]).u32();
        e.call(SCENE_SET_NODE_004A0FD0, &args![scene, held]);
        let graph = e.call(SCENE_GRAPH_00450B80, &args![1u32]).u32();
        e.call(ACCUMULATE_SCENE, &args![camera, graph, scene]);
        let held = e.call(POINTER_GET, &args![node]).u32();
        e.call(RENDER_00B6C0D0, &args![camera, held, 0u32]);
        e.call(SCENE_SET_NODE_004A0FD0, &args![scene, 0u32]);
        e.call(SCENE_RESET_00C4F2D0, &args![scene]);
        e.call(POINTER_DESTRUCTOR, &args![node]);
    });
}

// Translated from 007949f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoveTesterMenu::TurnPage` (engine map; `thiscall`, `RET 4`: forward). Records the
/// direction (`011a039c`). Forward below the summary (state 8) moves to the next state,
/// backward above 0 to the previous, each with the page-turn sound (a held handle); a
/// forward turn on the summary instead plays the finish sound and closes the machine
/// ([`fn_007917a0`]) when all points are spent, or plays the refused sound, and answers
/// false; a backward turn at 0 answers false. After a move the dynamic model's controller
/// manager (`NiObjectNET::GetController` with `011f36ac`) plays the page's sequence (see
/// [`page_sequence`]): `DeactivateAll(0.0)`, `0047aab0(sequence, 0, 1, 1.0, 0.0, 0)`,
/// `0047aa40(1)`, `004954c0`. Answers true.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 007949f0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn love_tester_menu_turn_page(e: &mut Engine, this: Ptr, forward: bool) -> bool {
    let base = this.addr();
    let moved: bool;
    if forward {
        e.mem.set_u8(TURN_FORWARD, 1);
        if e.mem.i32(base + 0x48) < 8 {
            let state = e.mem.u32(base + 0x48);
            e.mem.set_u32(base + 0x48, state.wrapping_add(1));
            moved = true;
            play_sound_held(e, SOUND_PAGE_TURN);
        } else {
            let used = e.call(GET_TOTAL_POINTS_USED, &args![this]).u32();
            if used == e.mem.u32(base + 0x58) {
                play_sound(e, SOUND_FINISH);
                fn_007917a0(e);
            } else {
                play_sound(e, SOUND_REFUSED);
            }
            return false;
        }
    } else {
        e.mem.set_u8(TURN_FORWARD, 0);
        if e.mem.i32(base + 0x48) > 0 {
            let state = e.mem.u32(base + 0x48);
            e.mem.set_u32(base + 0x48, state.wrapping_sub(1));
            moved = true;
            play_sound_held(e, SOUND_PAGE_TURN);
        } else {
            return false;
        }
    }
    if moved {
        let model = e.mem.u32(menu(e) + 0x5c);
        let manager = e
            .call(GET_CONTROLLER, &args![model, MANAGER_ARGUMENT_011F36AC])
            .u32();
        if manager != 0 {
            let sequence = page_sequence(e, base, manager);
            if sequence != 0 {
                e.call(DEACTIVATE_ALL, &args![manager, 0.0f32]);
                e.call(
                    ACTIVATE_SEQUENCE_0047AAB0,
                    &args![manager, sequence, 0u32, 1u32, 1.0f32, 0.0f32, 0u32],
                );
                e.call(SET_FLAG_0047AA40, &args![manager, 1u32]);
                e.call(SEQUENCE_START_004954C0, &args![sequence]);
            }
        }
    }
    true
}

// Translated from 00794d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): the actor value the page shows: on the summary
/// (state 8) under hardcore (`004b71d0`) the chosen one (+0x4c) plus 4; on states 1 to 7
/// the state plus 4; otherwise -1.
pub fn fn_00794d30(e: &mut Engine, this: Ptr) -> i32 {
    let base = this.addr();
    if e.mem.i32(base + 0x48) == 8 && e.call(HARDCORE_004B71D0, &args![]).u8() != 0 {
        return e.mem.i32(base + 0x4c).wrapping_add(4);
    }
    let state = e.mem.i32(base + 0x48);
    if state > 0 && state < 8 {
        state + 4
    } else {
        -1
    }
}

// Translated from 00794e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `LoveTesterButton` constructor (unnamed in the engine map; `thiscall`, `RET 0xc`:
/// the click function, the refresh function, the object name): constructs `spButton`,
/// keeps the two functions, assigns `spButton` the dynamic model's object of that name
/// (slot +0x9c with a stack string), and takes `pImage` from that object's shader property
/// (see [`shader_property`]). Returns `this`.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00794e70 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00794e70(e: &mut Engine, this: Ptr, click: u32, refresh: u32, name: u32) -> Ptr {
    let base = this.addr();
    e.call(POINTER_CONSTRUCTOR, &args![base + 8, 0u32]);
    e.mem.set_u32(base, click);
    e.mem.set_u32(base + 4, refresh);
    let model = e.mem.u32(menu(e) + 0x5c);
    let object = object_by_name(e, model, name);
    e.call(POINTER_ASSIGN, &args![base + 8, object]);
    let model = e.mem.u32(menu(e) + 0x5c);
    let object = object_by_name(e, model, name);
    let image = shader_property(e, object);
    e.mem.set_u32(base + 0xc, image);
    this
}

// Translated from 00794ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoveTesterMenu::ActorValueButton::ActorValueButton` (engine map; `cdecl` with variable
/// arguments: the button, the click and refresh functions, the object name, the actor
/// value, a count, then that many text items): constructs the button part
/// ([`fn_00794e70`]) and `AffectedFields` (+0x14, `00723fa0`), keeps the actor value
/// (+0x24) and adds each text item (`007cb2e0` with the address of its argument word).
/// Returns the button.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00794ff0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn love_tester_menu_actor_value_button(e: &mut Engine, words: &[u32]) -> u32 {
    let button = words[0];
    e.call(0x0079_4e70, &args![button, words[1], words[2], words[3]]);
    e.call(ARRAY_CONSTRUCTOR_00723FA0, &args![button + 0x14]);
    e.mem.set_u32(button + 0x24, words[4]);
    let count = words[5] as i32;
    let extra = (count.max(0) as usize).min(words.len().saturating_sub(6));
    e.with_stack(4 * extra.max(1) as u32, |e, items| {
        for (i, word) in words[6..6 + extra].iter().enumerate() {
            e.mem.set_u32(items.addr() + 4 * i as u32, *word);
        }
        for i in 0..count {
            // The exe reads the argument words in place; past the given words it reads
            // the caller's stack, which this model does not have (it stops loudly).
            let slot = items.addr() + 4 * i as u32;
            assert!(
                (i as usize) < extra,
                "00794ff0: count {count} reads past the {extra} item words given"
            );
            e.call(ARRAY_ADD_007CB2E0, &args![button + 0x14, slot]);
        }
    });
    button
}

fn actor_value_button_entry(e: &mut Engine, words: &[u32]) -> Ret {
    love_tester_menu_actor_value_button(e, words).into_ret()
}

// Translated from 007950b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `LoveTesterTextItem` setup (unnamed in the engine map; `thiscall`, `RET 0x18`: the
/// refresh function, two object names, the actor value, a byte choosing the dynamic model
/// (+0x5c, set) or the cabinet (+0x60), the threshold): keeps the function, actor value and
/// threshold, takes the first object (`pTextObj[0]`, +0xc) by name and its shader property
/// (`pText[0]`, +4); with a second name the same for `pTextObj[1]` (+0x10) and `pText[1]`
/// (+8), else both null. Returns `this`.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 007950b0 - the FS:[0] exception frame (unwinding on a throw) is not translated
#[allow(clippy::too_many_arguments)]
pub fn fn_007950b0(
    e: &mut Engine,
    this: Ptr,
    refresh: u32,
    name: u32,
    second_name: u32,
    actor_value: u32,
    dynamic: u8,
    threshold: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, refresh);
    e.mem.set_u32(base + 0x14, actor_value);
    e.mem.set_u32(base + 0x18, threshold);
    let model_field = if dynamic != 0 { 0x5c } else { 0x60 };
    let model = e.mem.u32(menu(e) + model_field);
    let object = object_by_name(e, model, name);
    e.mem.set_u32(base + 0xc, object);
    let object = e.mem.u32(base + 0xc);
    let property = shader_property(e, object);
    e.mem.set_u32(base + 4, property);
    if second_name != 0 {
        let model = e.mem.u32(menu(e) + model_field);
        let object = object_by_name(e, model, second_name);
        e.mem.set_u32(base + 0x10, object);
        let object = e.mem.u32(base + 0x10);
        let property = shader_property(e, object);
        e.mem.set_u32(base + 8, property);
    } else {
        e.mem.set_u32(base + 0x10, 0);
        e.mem.set_u32(base + 8, 0);
    }
    this
}

/// Shows texture `texture` on the shader property `property` (its slot +0xf0 with 0, 0).
fn show_texture(e: &mut Engine, property: u32, texture: u32) {
    e.vcall(property, 0xf0, &args![0u32, 0u32, texture]);
}

// Translated from 00795390 (decompiled, FalloutNV.exe 1.4.0.525)
/// A text item's refresh (unnamed in the engine map; `cdecl`, the item): shows the
/// texture of the player's value of the item's actor value (`spAVValues`, +0x1c4) on
/// `pText[0]`.
pub fn fn_00795390(e: &mut Engine, item: u32) {
    let index = e.mem.u32(item + 0x14);
    let value = player_value(e, index) as u32;
    let slot = menu(e) + value.wrapping_mul(4) + 0x1c4;
    let texture = e.call(POINTER_GET, &args![slot]).u32();
    let text = e.mem.u32(item + 4);
    show_texture(e, text, texture);
}

// Translated from 007953f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The points text's refresh (unnamed in the engine map; `cdecl`, the item, which needs
/// both `pText` set): the remaining points (+0x58 minus `GetTotalPointsUsed`) as two digits
/// (tens limited to 9, units 0 for a negative remainder), each shown with the digit's
/// texture (`spAVValues`) after `00450f90(0)` on its object; the tens digit shows the 0
/// texture when it is not above 0.
pub fn fn_007953f0(e: &mut Engine, item: u32) {
    if e.mem.u32(item + 4) == 0 || e.mem.u32(item + 8) == 0 {
        return;
    }
    let instance = menu(e);
    let used = e.call(GET_TOTAL_POINTS_USED, &args![instance]).i32();
    let remaining = e.mem.i32(menu(e) + 0x58).wrapping_sub(used);
    let capped = if remaining / 10 < 9 {
        remaining / 10
    } else {
        9
    };
    let tens = if capped > 0 {
        if remaining / 10 < 9 {
            remaining / 10
        } else {
            9
        }
    } else {
        0
    };
    let units = if remaining % 10 >= 0 {
        remaining % 10
    } else {
        0
    };
    let object = e.mem.u32(item + 0xc);
    e.call(SET_CULLED_00450F90, &args![object, 0u32]);
    let slot = if tens > 0 {
        menu(e) + (tens as u32) * 4 + 0x1c4
    } else {
        menu(e) + 0x1c4
    };
    let texture = e.call(POINTER_GET, &args![slot]).u32();
    let text = e.mem.u32(item + 4);
    show_texture(e, text, texture);
    let object = e.mem.u32(item + 0x10);
    e.call(SET_CULLED_00450F90, &args![object, 0u32]);
    let slot = menu(e)
        .wrapping_add((units as u32).wrapping_mul(4))
        .wrapping_add(0x1c4);
    let texture = e.call(POINTER_GET, &args![slot]).u32();
    let text = e.mem.u32(item + 8);
    show_texture(e, text, texture);
}

/// Hides or shows the button's object (`00450f90` on `spButton`).
fn set_button_culled(e: &mut Engine, button: u32, culled: bool) {
    let object = e.call(POINTER_GET, &args![button + 8]).u32();
    e.call(SET_CULLED_00450F90, &args![object, u32::from(culled)]);
}

/// The hardcore summary rule: on the summary page under hardcore (`004b71d0`), a value
/// button for another value than the chosen one (+0x4c plus 4) is hidden.
fn hide_unchosen(e: &mut Engine, button: u32, index: i32) {
    if e.call(HARDCORE_004B71D0, &args![]).u8() != 0
        && e.mem.i32(menu(e) + 0x48) == 8
        && index != e.mem.i32(menu(e) + 0x4c).wrapping_add(4)
    {
        set_button_culled(e, button, true);
    }
}

// Translated from 00795580 (decompiled, FalloutNV.exe 1.4.0.525)
/// A down button's refresh (unnamed in the engine map; `cdecl`, the button): for a
/// button with an actor value (+0x24, not -1) it is shown while the player's value is above
/// 1, hidden otherwise, then [`hide_unchosen`] applies.
pub fn fn_00795580(e: &mut Engine, button: u32) {
    let index = e.mem.i32(button + 0x24);
    if index == -1 {
        return;
    }
    let value = player_value(e, index as u32);
    set_button_culled(e, button, value <= 1);
    hide_unchosen(e, button, index);
}

// Translated from 00795630 (decompiled, FalloutNV.exe 1.4.0.525)
/// An up button's refresh (unnamed in the engine map; `cdecl`, the button): for a button
/// with an actor value it is hidden once the player's value is 10 or more, or (with an
/// instance) all points are spent, shown otherwise, then [`hide_unchosen`] applies.
pub fn fn_00795630(e: &mut Engine, button: u32) {
    let index = e.mem.i32(button + 0x24);
    if index == -1 {
        return;
    }
    let value = player_value(e, index as u32);
    let culled = if value >= 10 {
        true
    } else if menu(e) == 0 {
        false
    } else {
        let instance = menu(e);
        let used = e.call(GET_TOTAL_POINTS_USED, &args![instance]).i32();
        used >= e.mem.i32(menu(e) + 0x58)
    };
    set_button_culled(e, button, culled);
    hide_unchosen(e, button, index);
}

// Translated from 00795700 (decompiled, FalloutNV.exe 1.4.0.525)
/// The next-page button's refresh (unnamed in the engine map; `cdecl`, the button): on
/// the summary page with points left the image is `spNextPageButtonImage[0]` (+0x200),
/// otherwise `[1]` (+0x204).
pub fn fn_00795700(e: &mut Engine, button: u32) {
    let instance = menu(e);
    let summary_with_points = e.mem.i32(instance + 0x48) == 8 && {
        let used = e.call(GET_TOTAL_POINTS_USED, &args![instance]).i32();
        used < e.mem.i32(menu(e) + 0x58)
    };
    let slot = if summary_with_points {
        menu(e) + 0x200
    } else {
        menu(e) + 0x204
    };
    let texture = e.call(POINTER_GET, &args![slot]).u32();
    let image = e.mem.u32(button + 0xc);
    show_texture(e, image, texture);
}

// Translated from 00795780 (decompiled, FalloutNV.exe 1.4.0.525)
/// The previous-page button's refresh (unnamed in the engine map; `cdecl`, the button):
/// its image shows `spPreviousPageButtonImage[0]` (+0x208).
pub fn fn_00795780(e: &mut Engine, button: u32) {
    let slot = menu(e) + 0x208;
    let texture = e.call(POINTER_GET, &args![slot]).u32();
    let image = e.mem.u32(button + 0xc);
    show_texture(e, image, texture);
}

// Translated from 00795b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value up handler (unnamed in the engine map; no words): with an instance and a page
/// value ([`fn_00794d30`]), plays the up sound (a held handle) while the value is below 10
/// and, while points remain, raises it (at most 10, `004a8f20`;
/// `PlayerCharacter::SetActorBaseValue`); then the page is refreshed.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00795b30 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00795b30(e: &mut Engine) {
    if menu(e) == 0 {
        return;
    }
    let instance = menu(e);
    let index = fn_00794d30(e, Ptr::new(instance));
    if index != -1 {
        let value = player_value(e, index as u32);
        if value < 10 {
            play_sound_held(e, SOUND_VALUE_UP);
        }
        let instance = menu(e);
        let used = e.call(GET_TOTAL_POINTS_USED, &args![instance]).i32();
        if used < e.mem.i32(menu(e) + 0x58) {
            let raised = e
                .call(MIN_004A8F20, &args![value.wrapping_add(1), 10u32])
                .u32();
            let player = e.mem.u32(PLAYER);
            e.call(SET_ACTOR_BASE_VALUE, &args![player, index, raised]);
        }
    }
    let instance = menu(e);
    love_tester_menu_refresh_page(e, Ptr::new(instance));
}

// Translated from 00795c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoveTesterMenu::NextPage` (engine map; no words): with an instance whose page
/// animation is finished, turns the page forward and refreshes it.
pub fn love_tester_menu_next_page(e: &mut Engine) {
    let instance = menu(e);
    if instance == 0 || e.mem.u8(instance + 0x54) == 0 {
        return;
    }
    love_tester_menu_turn_page(e, Ptr::new(instance), true);
    let instance = menu(e);
    love_tester_menu_refresh_page(e, Ptr::new(instance));
}

// Translated from 00795dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `LoveTesterButton` destructor body (unnamed in the engine map; `thiscall`): assigns
/// null to `spButton` and destroys it.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00795dc0 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn fn_00795dc0(e: &mut Engine, this: Ptr) {
    e.call(POINTER_ASSIGN, &args![this.addr() + 8, 0u32]);
    e.call(POINTER_DESTRUCTOR, &args![this.addr() + 8]);
}

/// Calls the function at `offset` of each element of `array` with the element (`cdecl`).
fn call_each(e: &mut Engine, array: impl Fn(&Engine) -> u32, offset: u32) {
    let mut i = 0u32;
    loop {
        let items = array(e);
        if i >= array_size(e, items) {
            break;
        }
        let items = array(e);
        let item = element(e, items, i);
        let items = array(e);
        let again = element(e, items, i);
        let function = e.mem.u32(again + offset);
        e.call(function, &args![item]);
        i += 1;
    }
}

// Translated from 00795e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoveTesterMenu::RefreshPage` (engine map; `thiscall`). Nothing past the summary
/// (state above 8). With an instance: each glow's card (`LightBulbGlows`, +0x74) is hidden
/// (`00450f90`) while the page's value (the player's value of [`fn_00794d30`], 0 without
/// one) is below its `m_nValueToGlowAt`; the bulbs (+0x64) are walked, their threshold read.
/// Then the page's text items and buttons, and the global page's text items and buttons,
/// get their refresh functions (`pfRefreshField`, `pfRefresh`).
pub fn love_tester_menu_refresh_page(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.i32(base + 0x48) > 8 {
        return;
    }
    if menu(e) != 0 {
        let instance = menu(e);
        let mut value = 0i32;
        if fn_00794d30(e, Ptr::new(instance)) != -1 {
            let instance = menu(e);
            let index = fn_00794d30(e, Ptr::new(instance)) as u32;
            value = player_value(e, index);
        }
        let mut i = 0u32;
        while i < array_size(e, menu(e) + 0x64) {
            let bulbs = menu(e) + 0x64;
            let bulb = element(e, bulbs, i);
            let _threshold = e.mem.u32(bulb + 0x18);
            let bulbs = menu(e) + 0x64;
            e.call(ARRAY_ELEMENT, &args![bulbs, i]);
            i += 1;
        }
        let mut i = 0u32;
        while i < array_size(e, menu(e) + 0x74) {
            let glows = menu(e) + 0x74;
            let glow = element(e, glows, i);
            let threshold = e.mem.i32(glow + 4);
            let glows = menu(e) + 0x74;
            let glow = element(e, glows, i);
            if e.mem.u32(glow) != 0 {
                let culled = u32::from(value < threshold);
                let glows = menu(e) + 0x74;
                let glow = element(e, glows, i);
                let card = e.mem.u32(glow);
                e.call(SET_CULLED_00450F90, &args![card, culled]);
            }
            i += 1;
        }
    }
    call_each(e, move |e| base + (e.mem.u32(base + 0x48) << 5) + 0x94, 0);
    call_each(e, move |e| base + (e.mem.u32(base + 0x48) << 5) + 0x84, 4);
    call_each(e, move |_| base + 0x1b4, 0);
    call_each(e, move |_| base + 0x1a4, 4);
}

/// One texture of [`love_tester_menu_load_button_and_text_images`]: the path formatted
/// into a 0x104-byte buffer (`00406d00` with `format` and `words`), swapped for the
/// platform's language (`SwapPlatformLanguageTexturePath` into a second buffer) and loaded
/// into `slot` (`TES::CreateTextureImage` on the object at `011dea10`, with 0, 0).
fn load_texture(e: &mut Engine, buffers: u32, format: u32, words: &[u32], slot: u32) {
    let path = buffers;
    let swapped = buffers + 0x104;
    let mut call = vec![path, 0x104, format];
    call.extend_from_slice(words);
    e.call(FORMAT, &call);
    e.call(SWAP_LANGUAGE_TEXTURE_PATH, &args![path, swapped, 0x104u32]);
    let tes = e.mem.u32(TES_011DEA10);
    e.call(CREATE_TEXTURE_IMAGE, &args![tes, swapped, slot, 0u32, 0u32]);
}

// Translated from 00796100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoveTesterMenu::LoadButtonAndTextImages` (engine map; `thiscall`): loads the machine's
/// textures (see [`load_texture`]): +0x228 and +0x22c (format `0101996c` with `01074c9c`
/// and `01074cc0` / `01074c78`), the eleven value digits +0x1c4.. (`01074c4c` with
/// `01074c58`, `01074c6c` and the index), the four points images +0x1f0.. (`01074c4c` with
/// `01074c9c`, `01074c40` and 1 to 4), then, with the language suffix (`0101fd1c` when
/// `00709d40` answers true, else `0101fd24`, copied into five bytes), the button images
/// +0x204, +0x200, +0x208, +0x210, +0x214 (`01074c2c` with `01074c58`, the suffix and
/// `01074c38`, `01074c24`, `01074c1c`, `01074c00`, `01074bf8`) and +0x20c, +0x218..+0x224
/// (`01074c08` with `01074c9c`/`01074c58` and `01074c14`, `01074be8`, `01074bd8`, `01074bc8`,
/// `01074bb8`).
///
/// The /GS stack cookie check (`00ec408c`) is not translated.
// Translation incomplete: 00796100 - the /GS stack cookie check (00ec408c) is not translated
pub fn love_tester_menu_load_button_and_text_images(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(0x210, |e, locals| {
        let buffers = locals.addr();
        let suffix = locals.addr() + 0x208;
        load_texture(
            e,
            buffers,
            0x0101_996c,
            &[0x0107_4c9c, 0x0107_4cc0],
            base + 0x228,
        );
        load_texture(
            e,
            buffers,
            0x0101_996c,
            &[0x0107_4c9c, 0x0107_4c78],
            base + 0x22c,
        );
        for i in 0..0xbu32 {
            load_texture(
                e,
                buffers,
                0x0107_4c4c,
                &[0x0107_4c58, 0x0107_4c6c, i],
                base + i * 4 + 0x1c4,
            );
        }
        for i in 1..5u32 {
            load_texture(
                e,
                buffers,
                0x0107_4c4c,
                &[0x0107_4c9c, 0x0107_4c40, i],
                base + i * 4 + 0x1ec,
            );
        }
        let language = if e.call(LANGUAGE_00709D40, &args![]).u8() != 0 {
            0x0101_fd1c
        } else {
            0x0101_fd24
        };
        e.call(STRING_COPY_00406D30, &args![suffix, 5u32, language]);
        for (name, slot) in [
            (0x0107_4c38u32, 0x204u32),
            (0x0107_4c24, 0x200),
            (0x0107_4c1c, 0x208),
        ] {
            load_texture(
                e,
                buffers,
                0x0107_4c2c,
                &[0x0107_4c58, suffix, name],
                base + slot,
            );
        }
        load_texture(
            e,
            buffers,
            0x0107_4c08,
            &[0x0107_4c9c, 0x0107_4c14],
            base + 0x20c,
        );
        for (name, slot) in [(0x0107_4c00u32, 0x210u32), (0x0107_4bf8, 0x214)] {
            load_texture(
                e,
                buffers,
                0x0107_4c2c,
                &[0x0107_4c58, suffix, name],
                base + slot,
            );
        }
        for (name, slot) in [
            (0x0107_4be8u32, 0x218u32),
            (0x0107_4bd8, 0x21c),
            (0x0107_4bc8, 0x220),
            (0x0107_4bb8, 0x224),
        ] {
            load_texture(e, buffers, 0x0107_4c08, &[0x0107_4c58, name], base + slot);
        }
    });
}

/// Callees and data of [`love_tester_menu_init_machine`] (`thiscall` unless noted).
/// `ModelLoader::LoadFile` (6 words) on the loader at `011c3b3c`.
const LOAD_FILE: u32 = 0x0044_7080;
/// `SwapPlatformLanguageTextures` (`cdecl`, the model).
const SWAP_LANGUAGE_TEXTURES: u32 = 0x004b_7660;
/// `NiMatrix3::MakeXRotation` (the angle).
const MAKE_X_ROTATION: u32 = 0x0052_4ac0;
/// Unnamed in the engine map: another one-angle matrix setup, on the second matrix.
const ROTATION_004A0C90: u32 = 0x004a_0c90;
/// `NiMatrix3::operator*` (the result's address, the other matrix; answers the result).
const MATRIX_PRODUCT: u32 = 0x0043_f8d0;
/// Unnamed (no words, no `this`): the screen height (`004dc1f0`) loaded into `ST0` (`FILD`).
const SCREEN_HEIGHT_00706E50: u32 = 0x0070_6e50;
/// Unnamed one-word setters of the models: `00440490` (a float), `0043fa80` (a matrix).
const MODEL_SET_00440490: u32 = 0x0044_0490;
const MODEL_SET_MATRIX_0043FA80: u32 = 0x0043_fa80;
/// `NiTimeController::StartAnimations` (`cdecl`, the object).
const START_ANIMATIONS: u32 = 0x00a6_d2d0;
/// `NiAVObject::GetWorldBound` (no words).
const GET_WORLD_BOUND: u32 = 0x0043_d450;
/// The float at `this + 0xc` in `ST0` (`0084d030`, also the frame timer's elapsed time).
const VALUE_AT_12_0084D030: u32 = FRAME_TIME_0084D030;
/// `ShadowSceneNode::AddAllLightsFromNode_Recurse` (the node, a float).
const ADD_ALL_LIGHTS_FROM_NODE: u32 = 0x00b5_ca70;
/// `ShadowSceneNode::AddLight` (the light, a word).
const ADD_LIGHT: u32 = 0x00b5_c940;
/// The light's constructor (`00a7d670`, unnamed; on a 0xfc-byte block).
const LIGHT_CONSTRUCTOR_00A7D670: u32 = 0x00a7_d670;
/// Unnamed one-word setters of the light: two points (`00440460`, `004bc2e0`), a point
/// (`0050dd50`) and a float (`0050dd20`).
const LIGHT_SET_00440460: u32 = 0x0044_0460;
const LIGHT_SET_004BC2E0: u32 = 0x004b_c2e0;
const LIGHT_SET_0050DD50: u32 = 0x0050_dd50;
const LIGHT_SET_0050DD20: u32 = 0x0050_dd20;
/// The allocation `00401000` (`cdecl`, the size).
const ALLOCATE_00401000: u32 = 0x0040_1000;
/// The glow's setup (`004b0680`, unnamed; the card object and the value to glow at).
const GLOW_SETUP_004B0680: u32 = 0x004b_0680;
/// Float constants the code loads: π/2 (`0101ff38`), π (`0102b3c8`), the tilt
/// (`01074b90`), the height limit (a double, 640.0, `01074b88`), the two placements
/// (`01074b7c`, `01074b80`), the light radius scale (a double, `0102fc70`) and the light's
/// grey level (`01019de0`).
const HALF_PI_0101FF38: u32 = 0x0101_ff38;
const PI_0102B3C8: u32 = 0x0102_b3c8;
const TILT_01074B90: u32 = 0x0107_4b90;
const HEIGHT_LIMIT_01074B88: u32 = 0x0107_4b88;
const PLACE_01074B7C: u32 = 0x0107_4b7c;
const PLACE_01074B80: u32 = 0x0107_4b80;
const RADIUS_SCALE_0102FC70: u32 = 0x0102_fc70;
const GREY_01019DE0: u32 = 0x0101_9de0;
/// `"Forward"`.
const FORWARD_SEQUENCE: u32 = 0x0104_8458;
/// The glow name formats: `"P1_PointVal_0%i_GLOW:%i"`, `"P1_PointVal_0%i_GLOW"`,
/// `"P1_PointVal_10_GLOW:%i"`, `"P1_PointVal_10_GLOW"`.
const GLOW_NAME_DIGIT_PART: u32 = 0x0107_4b24;
const GLOW_NAME_DIGIT: u32 = 0x0107_4b0c;
const GLOW_NAME_TEN_PART: u32 = 0x0107_4af4;
const GLOW_NAME_TEN: u32 = 0x0107_4ae0;
/// The click and refresh functions the buttons and text items keep (`cdecl`, the item).
const CLICK_VALUE_DOWN_007958E0: u32 = 0x0079_58e0;
const CLICK_VALUE_UP: u32 = 0x0079_5b30;
const CLICK_NEXT_PAGE: u32 = 0x0079_5c60;
const CLICK_PREVIOUS_PAGE: u32 = 0x0079_5ca0;
const CLICK_CLOSE: u32 = 0x0079_5ce0;
const CLICK_SUMMARY_DOWN_007957C0: u32 = 0x0079_57c0;
const CLICK_SUMMARY_UP_00795A00: u32 = 0x0079_5a00;
const REFRESH_DOWN: u32 = 0x0079_5580;
const REFRESH_UP: u32 = 0x0079_5630;
const REFRESH_NEXT: u32 = 0x0079_5700;
const REFRESH_PREVIOUS: u32 = 0x0079_5780;
const REFRESH_VALUE_TEXT: u32 = 0x0079_5390;
const REFRESH_POINTS_TEXT: u32 = 0x0079_53f0;
/// The summary page's rows: actor value, then the names of its value text, down button and
/// up button (`"Index_StrengthPointVal:0"`, `"Index_StrengthDecrease_Btn:0"`, ...).
const SUMMARY_ROWS: [(u32, u32, u32, u32); 7] = [
    (5, 0x0107_4a80, 0x0107_4a60, 0x0107_4a40),
    (6, 0x0107_4a24, 0x0107_4a04, 0x0107_49e4),
    (7, 0x0107_49c8, 0x0107_49a8, 0x0107_4988),
    (8, 0x0107_496c, 0x0107_494c, 0x0107_492c),
    (9, 0x0107_490c, 0x0107_48e8, 0x0107_48c4),
    (10, 0x0107_48ac, 0x0107_4890, 0x0107_4874),
    (11, 0x0107_485c, 0x0107_4840, 0x0107_4824),
];

/// The size of [`love_tester_menu_init_machine`]'s frame model: a local at `EBP - n` is at
/// `top - n` (see [`Frame`]).
const INIT_FRAME: u32 = 0x3f0;

/// The locals of [`love_tester_menu_init_machine`], at the exe's own `EBP` offsets.
#[derive(Clone, Copy)]
struct Frame {
    top: u32,
}

impl Frame {
    /// The local at `EBP - offset`.
    fn at(self, offset: u32) -> u32 {
        self.top - offset
    }
}

/// Stores `item` in the stack word `slot` and adds it (through the word's address) to the
/// array (`BSSimpleArray::AddUninitialized`, `007cb2e0`).
fn add_item(e: &mut Engine, array: u32, slot: u32, item: u32) {
    e.mem.set_u32(slot, item);
    e.call(ARRAY_ADD_007CB2E0, &args![array, slot]);
}

/// A love tester button ([`fn_00794e70`]) in a 0x14-byte block (`00401000`), or null when
/// the allocation fails.
fn new_button(e: &mut Engine, click: u32, refresh: u32, name: u32) -> u32 {
    let block = e.call(ALLOCATE_00401000, &args![0x14u32]).u32();
    if block == 0 {
        return 0;
    }
    fn_00794e70(e, Ptr::new(block), click, refresh, name).addr()
}

/// An actor value button ([`love_tester_menu_actor_value_button`] with `words` after the
/// button) in a 0x28-byte block, or null when the allocation fails.
fn new_value_button(e: &mut Engine, words: &[u32]) -> u32 {
    let block = e.call(ALLOCATE_00401000, &args![0x28u32]).u32();
    if block == 0 {
        return 0;
    }
    let mut all = vec![block];
    all.extend_from_slice(words);
    love_tester_menu_actor_value_button(e, &all)
}

/// A text item ([`fn_007950b0`]: the refresh function, the names, the actor value, the
/// dynamic model, threshold 0) in a 0x1c-byte block, or null when the allocation fails.
fn new_text_item(e: &mut Engine, refresh: u32, name: u32, second_name: u32, value: u32) -> u32 {
    let block = e.call(ALLOCATE_00401000, &args![0x1cu32]).u32();
    if block == 0 {
        return 0;
    }
    fn_007950b0(e, Ptr::new(block), refresh, name, second_name, value, 1, 0).addr()
}

/// The cabinet's (+0x60 of the instance) object named by the text at `text`, through a
/// string built at `string` (left for the caller to destroy).
fn cabinet_object(e: &mut Engine, string: u32, text: u32) -> u32 {
    let name = e.call(STRING_CONSTRUCTOR, &args![string, text]).u32();
    let cabinet = e.mem.u32(menu(e) + 0x60);
    e.vcall(cabinet, 0x9c, &args![name]).u32()
}

/// One light bulb glow: when the cabinet has an object named by the text at `text` (looked
/// up through the string at `string`), an 8-byte glow (`00401000`) is set up with the object
/// (looked up again through `second_string`) and `value` (`004b0680`) and added to
/// `LightBulbGlows` (+0x74) through the stack word `slot` (null when the allocation failed).
fn add_glow(e: &mut Engine, text: u32, string: u32, second_string: u32, slot: u32, value: u32) {
    let found = cabinet_object(e, string, text);
    e.call(STRING_DESTRUCTOR, &args![string]);
    if found == 0 {
        return;
    }
    let block = e.call(ALLOCATE_00401000, &args![8u32]).u32();
    let glow = if block != 0 {
        let object = cabinet_object(e, second_string, text);
        e.call(GLOW_SETUP_004B0680, &args![block, object, value])
            .u32()
    } else {
        0
    };
    if block != 0 {
        e.call(STRING_DESTRUCTOR, &args![second_string]);
    }
    let glows = menu(e) + 0x74;
    add_item(e, glows, slot, glow);
}

/// The models of [`love_tester_menu_init_machine`]: both are loaded (`ModelLoader::LoadFile`
/// with 0, 1, 0, 0, 0) into the instance (+0x5c, +0x60) and get the platform's language
/// textures. The rotation is `MakeXRotation(π/2)` times `004a0c90(π)`, then times
/// `MakeXRotation` of the tilt (`NiMatrix3::operator*`, each product copied back). Each
/// model gets `00440490(1.0)`, the translation (0, `01074b7c`, `01074b80`) when the screen
/// height (`00706e50`) is at most 640.0, else (0, `01074b7c`, `01074b7c`), and the rotation
/// (`0043fa80`); then each is updated at 0.0 (`0043d410`, `00a59c60`).
fn init_models(e: &mut Engine, frame: Frame) {
    for (field, model) in [(0x5cu32, DYNAMIC_MODEL), (0x60, CABINET_MODEL)] {
        let loader = e.mem.u32(MODEL_LOADER);
        let loaded = e
            .call(
                LOAD_FILE,
                &args![loader, model, 0u32, 1u32, 0u32, 0u32, 0u32],
            )
            .u32();
        e.mem.set_u32(menu(e) + field, loaded);
        let loaded = e.mem.u32(menu(e) + field);
        e.call(SWAP_LANGUAGE_TEXTURES, &args![loaded]);
    }
    let rotation = frame.at(0x4c);
    let turn = frame.at(0x9c);
    e.call(POINT_CONSTRUCTOR_006815C0, &args![rotation]);
    e.call(POINT_CONSTRUCTOR_006815C0, &args![turn]);
    let angle = x87_quiet_bits(e.mem.u32(HALF_PI_0101FF38));
    e.call(MAKE_X_ROTATION, &args![rotation, angle]);
    let angle = x87_quiet_bits(e.mem.u32(PI_0102B3C8));
    e.call(ROTATION_004A0C90, &args![turn, angle]);
    let product = e
        .call(MATRIX_PRODUCT, &args![rotation, frame.at(0x148), turn])
        .u32();
    copy_matrix(e, product, rotation);
    let angle = x87_quiet_bits(e.mem.u32(TILT_01074B90));
    e.call(MAKE_X_ROTATION, &args![turn, angle]);
    let product = e
        .call(MATRIX_PRODUCT, &args![rotation, frame.at(0x16c), turn])
        .u32();
    copy_matrix(e, product, rotation);
    let height = e.call(SCREEN_HEIGHT_00706E50, &args![]).f64();
    // FCOMP double; TEST AH,0x41; JP: set when below or equal (not when unordered).
    let small = height <= e.mem.f64(HEIGHT_LIMIT_01074B88);
    let last = if small {
        PLACE_01074B80
    } else {
        PLACE_01074B7C
    };
    for field in [0x5cu32, 0x60] {
        let model = e.mem.u32(menu(e) + field);
        e.call(MODEL_SET_00440490, &args![model, 1.0f32]);
        let z = x87_quiet_bits(e.mem.u32(last));
        let y = x87_quiet_bits(e.mem.u32(PLACE_01074B7C));
        let model = e.mem.u32(menu(e) + field);
        e.call(SET_TRANSLATE_004BC1F0, &args![model, 0.0f32, y, z]);
        let model = e.mem.u32(menu(e) + field);
        e.call(MODEL_SET_MATRIX_0043FA80, &args![model, rotation]);
    }
    let first = frame.at(0x28);
    let second = frame.at(0x1c);
    e.call(UPDATE_BLOCK_0043D410, &args![first, 0.0f32, 0u32, 0u32]);
    e.call(UPDATE_BLOCK_0043D410, &args![second, 0.0f32, 0u32, 0u32]);
    let model = e.mem.u32(menu(e) + 0x5c);
    e.call(UPDATE_00A59C60, &args![model, first]);
    let model = e.mem.u32(menu(e) + 0x60);
    e.call(UPDATE_00A59C60, &args![model, second]);
}

/// `REP MOVSD` of a 3x3 matrix (9 words) from `source` to `destination`.
fn copy_matrix(e: &mut Engine, source: u32, destination: u32) {
    for i in 0..9 {
        let word = e.mem.u32(source + 4 * i);
        e.mem.set_u32(destination + 4 * i, word);
    }
}

/// The animation of [`love_tester_menu_init_machine`]: the frame timer (+0x50) advances by
/// the elapsed time (`0084d030` on `011f6394`); the dynamic model's controller manager
/// (`NiObjectNET::GetController` with `011f36ac`) plays `"Forward"` (`0047a520`;
/// `DeactivateAll(0.0)`, `0047aab0(sequence, 0, 1, 1.0, 0.0, 0)`, `0047aa40(1)`,
/// `004954c0`); the animations start on `this`'s model (`NiTimeController::StartAnimations`).
/// From the manager up (`004a8a90`), the first object with `00653270(011f36bc, ..)` gets an
/// update at 0.0 (its slot +0x94 with a block from `0043d410`). Then `this`'s model is
/// updated at 0.0.
fn start_machine(e: &mut Engine, this: u32, frame: Frame) {
    let elapsed = e.call(FRAME_TIME_0084D030, &args![TIMER_011F6394]).f64();
    let timer = x87_store32(x87_add(elapsed, x87_m32(e.mem.u32(menu(e) + 0x50))));
    e.mem.set_u32(menu(e) + 0x50, timer);
    let model = e.mem.u32(menu(e) + 0x5c);
    let manager = e
        .call(GET_CONTROLLER, &args![model, MANAGER_ARGUMENT_011F36AC])
        .u32();
    if manager != 0 {
        let string = frame.at(0x170);
        let name = e
            .call(STRING_CONSTRUCTOR, &args![string, FORWARD_SEQUENCE])
            .u32();
        let sequence = e.call(GET_SEQUENCE_0047A520, &args![manager, name]).u32();
        e.call(STRING_DESTRUCTOR, &args![string]);
        if sequence != 0 {
            e.call(DEACTIVATE_ALL, &args![manager, 0.0f32]);
            e.call(
                ACTIVATE_SEQUENCE_0047AAB0,
                &args![manager, sequence, 0u32, 1u32, 1.0f32, 0.0f32, 0u32],
            );
            e.call(SET_FLAG_0047AA40, &args![manager, 1u32]);
            e.call(SEQUENCE_START_004954C0, &args![sequence]);
        }
    }
    let model = e.mem.u32(this + 0x5c);
    e.call(START_ANIMATIONS, &args![model]);
    let mut node = manager;
    while node != 0 {
        let found = e
            .call(
                CONTROLLER_MANAGER_00653270,
                &args![MANAGER_ARGUMENT_011F36BC, node],
            )
            .u32();
        if found != 0 {
            let block = frame.at(0xb4);
            e.call(UPDATE_BLOCK_0043D410, &args![block, 0.0f32, 0u32, 0u32]);
            e.vcall(node, 0x94, &args![block]);
            break;
        }
        node = e.call(PARENT_004A8A90, &args![node]).u32();
    }
    let block = frame.at(0x70);
    e.call(UPDATE_BLOCK_0043D410, &args![block, 0.0f32, 0u32, 0u32]);
    let model = e.mem.u32(this + 0x5c);
    e.call(UPDATE_00A59C60, &args![model, block]);
}

/// The light of [`love_tester_menu_init_machine`]: the radius is the dynamic model's world
/// bound's float +0xc times 20.0 (`0102fc70`). The scene graph (`00450b80(1)`) takes the
/// lights of the model's node (its slot +0xc; `AddAllLightsFromNode_Recurse` with the
/// radius); when it finds none a light is made (0xfc bytes, `00aa13e0`, `00a7d670`; null when
/// the allocation fails) with (0, 0, 0) (`00440460`), the grey level three times
/// (`004bc2e0`), (radius, 0, 0) (`0050dd50`) and 1.0 (`0050dd20`), updated at 0.0, attached
/// (slot +0xdc with 1) and added (`ShadowSceneNode::AddLight` with 0). Then both models are
/// attached (slot +0xdc with 1).
fn init_light(e: &mut Engine, frame: Frame) {
    let model = e.mem.u32(menu(e) + 0x5c);
    let bound = e.call(GET_WORLD_BOUND, &args![model]).u32();
    let size = e.call(VALUE_AT_12_0084D030, &args![bound]).f64();
    let radius = x87_store32(x87_mul(size, e.mem.f64(RADIUS_SCALE_0102FC70)));
    let model = e.mem.u32(menu(e) + 0x5c);
    let node = e.vcall(model, 0xc, &args![]).u32();
    let graph = e.call(SCENE_GRAPH_00450B80, &args![1u32]).u32();
    let found = e
        .call(
            ADD_ALL_LIGHTS_FROM_NODE,
            &args![graph, node, x87_quiet_bits(radius)],
        )
        .u32();
    if found == 0 {
        let block = e.call(ALLOCATE_00AA13E0, &args![0xfcu32]).u32();
        let light = if block != 0 {
            e.call(LIGHT_CONSTRUCTOR_00A7D670, &args![block]).u32()
        } else {
            0
        };
        let point = e
            .call(
                POINT_00416870,
                &args![frame.at(0x184), 0.0f32, 0.0f32, 0.0f32],
            )
            .u32();
        e.call(LIGHT_SET_00440460, &args![light, point]);
        let grey = x87_quiet_bits(e.mem.u32(GREY_01019DE0));
        let point = e
            .call(POINT_00416870, &args![frame.at(0x190), grey, grey, grey])
            .u32();
        e.call(LIGHT_SET_004BC2E0, &args![light, point]);
        let point = e
            .call(
                POINT_00416870,
                &args![frame.at(0x19c), x87_quiet_bits(radius), 0.0f32, 0.0f32],
            )
            .u32();
        e.call(LIGHT_SET_0050DD50, &args![light, point]);
        e.call(LIGHT_SET_0050DD20, &args![light, 1.0f32]);
        let block = frame.at(0xc4);
        e.call(UPDATE_BLOCK_0043D410, &args![block, 0.0f32, 0u32, 0u32]);
        e.call(UPDATE_00A59C60, &args![light, block]);
        let graph = e.call(SCENE_GRAPH_00450B80, &args![1u32]).u32();
        e.vcall(graph, 0xdc, &args![light, 1u32]);
        let graph = e.call(SCENE_GRAPH_00450B80, &args![1u32]).u32();
        e.call(ADD_LIGHT, &args![graph, light, 0u32]);
    }
    for field in [0x5cu32, 0x60] {
        let graph = e.call(SCENE_GRAPH_00450B80, &args![1u32]).u32();
        let model = e.mem.u32(menu(e) + field);
        e.vcall(graph, 0xdc, &args![model, 1u32]);
    }
}

/// The glows of [`love_tester_menu_init_machine`] (see [`add_glow`]): for each value 1 to 9
/// the parts `"P1_PointVal_0<v>_GLOW:<0..9>"` and then `"P1_PointVal_0<v>_GLOW"`, all glowing
/// at the value; then `"P1_PointVal_10_GLOW:<0..9>"` and `"P1_PointVal_10_GLOW"`, glowing at
/// 10. Each name is formatted into a 0x1e-byte buffer (`00406d00`).
fn init_glows(e: &mut Engine, frame: Frame) {
    let buffer = frame.at(0xec);
    for value in 1..10u32 {
        for part in 0..10u32 {
            e.call(
                FORMAT,
                &args![buffer, 0x1eu32, GLOW_NAME_DIGIT_PART, value, part],
            );
            add_glow(
                e,
                buffer,
                frame.at(0x1cc),
                frame.at(0x1d8),
                frame.at(0xf4),
                value,
            );
        }
        e.call(FORMAT, &args![buffer, 0x1eu32, GLOW_NAME_DIGIT, value]);
        add_glow(
            e,
            buffer,
            frame.at(0x1e0),
            frame.at(0x1ec),
            frame.at(0xf8),
            value,
        );
    }
    let buffer = frame.at(0x118);
    for part in 0..10u32 {
        e.call(FORMAT, &args![buffer, 0x1eu32, GLOW_NAME_TEN_PART, part]);
        add_glow(
            e,
            buffer,
            frame.at(0x1f4),
            frame.at(0x200),
            frame.at(0x120),
            10,
        );
    }
    e.call(FORMAT, &args![buffer, 0x1eu32, GLOW_NAME_TEN]);
    add_glow(
        e,
        buffer,
        frame.at(0x208),
        frame.at(0x214),
        frame.at(0x124),
        10,
    );
}

// Translated from 007928e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoveTesterMenu::InitMachine` (Xbox PDB; `thiscall`): builds the machine. The models,
/// their rotation and placement ([`init_models`]), the animation ([`start_machine`]) and the
/// light ([`init_light`]). The global page (the instance's +0x1a4) gets the value down
/// (`007958e0`, refresh `00795580`, `"P1_Decrease_Btn:0"`) and up (`00795b30`, `00795630`,
/// `"P1_Increase_Btn:0"`) buttons without an actor value or text items, and the next
/// (`LoveTesterMenu::NextPage`, `00795700`, `"P1_RT_Btn:0"`) and previous
/// (`LoveTesterMenu::PreviousPage`, `00795780`, `"P1_LT_Btn:0"`) buttons; then the glows
/// ([`init_glows`]). On `this`: the points text (`007953f0`, the two
/// `"P1_PointsRemainDigit"` names, no actor value) on the global page's texts (+0x1b4), the
/// look-inside button (`NextPage`, `00795700`, `"LookInside_Btn:0"`) on page 0 (+0x84), and
/// on the summary page (page 8: texts +0x194, buttons +0x184), for each of the actor values
/// 5 to 11, its value text (`00795390`), its down (`007957c0`, `00795580`) and up
/// (`00795a00`, `00795630`) buttons, each with that text and the points text, and finally
/// the all-done button (`LoveTesterMenu::CloseCallback`, `00795700`, `"AllDone_Btn:0"`).
/// Every button and text item is allocated with `00401000` and added (null when the
/// allocation fails) through a stack word (`007cb2e0`). Then `this`'s images are loaded
/// ([`love_tester_menu_load_button_and_text_images`]), the instance's page refreshed
/// ([`love_tester_menu_refresh_page`]) and the page turned on
/// ([`love_tester_menu_next_page`], `cdecl`, pushed a 0 it does not read).
///
/// The locals passed by address sit at the exe's own frame offsets ([`Frame`]).
/// The exception frame (`FS:[0]`) and the /GS stack cookie check (`00ec408c`) are not
/// translated.
// Translation incomplete: 007928e0 - the FS:[0] exception frame (unwinding on a throw) and the /GS stack cookie check (00ec408c) are not translated
pub fn love_tester_menu_init_machine(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.with_stack(INIT_FRAME, |e, locals| {
        let frame = Frame {
            top: locals.addr() + INIT_FRAME,
        };
        init_models(e, frame);
        start_machine(e, this, frame);
        init_light(e, frame);
        let points_slot = frame.at(0x78);
        e.mem.set_u32(points_slot, 0);
        let down = new_value_button(
            e,
            &[
                CLICK_VALUE_DOWN_007958E0,
                REFRESH_DOWN,
                0x0107_4b68,
                u32::MAX,
                0,
            ],
        );
        let global_buttons = menu(e) + 0x1a4;
        add_item(e, global_buttons, frame.at(0x1a8), down);
        let up = new_value_button(e, &[CLICK_VALUE_UP, REFRESH_UP, 0x0107_4b54, u32::MAX, 0]);
        let global_buttons = menu(e) + 0x1a4;
        add_item(e, global_buttons, frame.at(0x1b4), up);
        for (click, refresh, name) in [
            (CLICK_NEXT_PAGE, REFRESH_NEXT, 0x0107_4b48u32),
            (CLICK_PREVIOUS_PAGE, REFRESH_PREVIOUS, 0x0107_4b3c),
        ] {
            let button = new_button(e, click, refresh, name);
            let global_buttons = menu(e) + 0x1a4;
            add_item(e, global_buttons, frame.at(0x5c), button);
        }
        init_glows(e, frame);
        let points = new_text_item(e, REFRESH_POINTS_TEXT, 0x0107_4ab0, 0x0107_4ac8, u32::MAX);
        add_item(e, this + 0x1b4, points_slot, points);
        let look_inside = new_button(e, CLICK_NEXT_PAGE, REFRESH_NEXT, 0x0107_4a9c);
        add_item(e, this + 0x84, frame.at(0x5c), look_inside);
        let text_slot = frame.at(0x10);
        for (row, (value, text_name, down_name, up_name)) in (0u32..).zip(SUMMARY_ROWS) {
            let text = new_text_item(e, REFRESH_VALUE_TEXT, text_name, 0, value);
            add_item(e, this + 0x194, text_slot, text);
            for (click, refresh, name, slot) in [
                (CLICK_SUMMARY_DOWN_007957C0, REFRESH_DOWN, down_name, 0x238),
                (CLICK_SUMMARY_UP_00795A00, REFRESH_UP, up_name, 0x244),
            ] {
                let text = e.mem.u32(text_slot);
                let points = e.mem.u32(points_slot);
                let button = new_value_button(e, &[click, refresh, name, value, 2, text, points]);
                add_item(e, this + 0x184, frame.at(slot + row * 0x20), button);
            }
        }
        let done = new_button(e, CLICK_CLOSE, REFRESH_NEXT, 0x0107_4814);
        add_item(e, this + 0x184, frame.at(0x5c), done);
        love_tester_menu_load_button_and_text_images(e, Ptr::new(this));
        let instance = menu(e);
        love_tester_menu_refresh_page(e, Ptr::new(instance));
        love_tester_menu_next_page(e);
    });
}

// Translated from 007915e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoveTesterMenu::Create` (engine map; no words): the machine with 0x23
/// points ([`love_tester_menu_create`], `Create_ov2`). Named by address
/// because the overload it calls has the plain name.
pub fn fn_007915e0(e: &mut Engine) {
    love_tester_menu_create(e, 0x23);
}

/// [`love_tester_menu_do_gamepad`]'s and [`love_tester_menu_do_key`]'s raise:
/// with a page value (not -1), points left (`GetTotalPointsUsed` below
/// `iAvailablePoints`, +0x58) and the player's value below 10, clicks up
/// (vtable +0xc with 2). Answers whether it clicked.
fn raise_page_value(e: &mut Engine, this: u32, index: i32) -> bool {
    if index == -1 {
        return false;
    }
    let used = e.call(GET_TOTAL_POINTS_USED, &args![this]).i32();
    if used >= e.mem.i32(this + 0x58) {
        return false;
    }
    if player_value(e, index as u32) >= 10 {
        return false;
    }
    e.vcall(this, 0xc, &args![2u32, 0u32]);
    true
}

/// The lower counterpart: with a page value whose player value is above 1,
/// clicks down (vtable +0xc with 3).
fn lower_page_value(e: &mut Engine, this: u32, index: i32) -> bool {
    if index == -1 {
        return false;
    }
    if player_value(e, index as u32) <= 1 {
        return false;
    }
    e.vcall(this, 0xc, &args![3u32, 0u32]);
    true
}

// Translated from 00791cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoveTesterMenu::DoGamepad` (engine map). Only while the machine is the
/// top menu (`Interface::IsTopMenuID(0x432)`); the page value is
/// [`fn_00794d30`]'s. Buttons 1 and 9 raise the value ([`raise_page_value`]),
/// 2 and 10 lower it, except on the summary (state 8, where 1 and 2 answer
/// 0); 3 and 4 answer 0 in every state (their checks for state 8 lead to
/// the checks of 1 and 2). Buttons that raise or lower answer 1 whether or
/// not they clicked; other buttons 0.
pub fn love_tester_menu_do_gamepad(e: &mut Engine, this: Ptr, button: u32, _unused_1: u32) -> u8 {
    let base = this.addr();
    if !e.call(IS_TOP_MENU_ID, &args![MENU_ID]).bool() {
        return 0;
    }
    let index = fn_00794d30(e, this);
    match button {
        1 | 3 => {
            if button == 3 && e.mem.u32(base + 0x48) != 8 {
                return 0;
            }
            if e.mem.u32(base + 0x48) == 8 {
                return 0;
            }
            raise_page_value(e, base, index);
            1
        }
        2 | 4 => {
            if button == 4 && e.mem.u32(base + 0x48) != 8 {
                return 0;
            }
            if e.mem.u32(base + 0x48) == 8 {
                return 0;
            }
            lower_page_value(e, base, index);
            1
        }
        9 => {
            raise_page_value(e, base, index);
            1
        }
        10 => {
            lower_page_value(e, base, index);
            1
        }
        _ => 0,
    }
}

// Translated from 007927a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoveTesterMenu::DoKey` (engine map): keys `0x80000001` and `0x80000002`
/// click next (vtable +0xc with 1 for the first, 0 for the second) and
/// answer 1; `0x80000003`, `+` (0x2b) and `=` (0x3d) raise the page value
/// ([`raise_page_value`] on [`fn_00794d30`]'s value), `0x80000004`, `-`
/// (0x2d) and `_` (0x5f) lower it, answering whether they clicked; other
/// keys answer 0.
pub fn love_tester_menu_do_key(e: &mut Engine, this: Ptr, key: u32) -> u8 {
    let base = this.addr();
    match key {
        0x8000_0001 | 0x8000_0002 => {
            e.vcall(base, 0xc, &args![u32::from(key == 0x8000_0001), 0u32]);
            1
        }
        0x8000_0003 | 0x2b | 0x3d => {
            let index = fn_00794d30(e, this);
            u8::from(raise_page_value(e, base, index))
        }
        0x8000_0004 | 0x2d | 0x5f => {
            let index = fn_00794d30(e, this);
            u8::from(lower_page_value(e, base, index))
        }
        _ => 0,
    }
}

/// The summary down and value down handlers' body: with the actor value
/// `index` (not -1), plays the down sound (a held handle) while the value
/// is above 1 and lowers it (at least 1, `00647b70`;
/// `PlayerCharacter::SetActorBaseValue`); then the page is refreshed.
fn lower_and_refresh(e: &mut Engine, index: i32) {
    if index != -1 {
        let value = player_value(e, index as u32);
        if value > 1 {
            play_sound_held(e, SOUND_VALUE_DOWN);
        }
        let lowered = e
            .call(MAX_00647B70, &args![value.wrapping_sub(1), 1u32])
            .u32();
        let player = e.mem.u32(PLAYER);
        e.call(SET_ACTOR_BASE_VALUE, &args![player, index, lowered]);
    }
    let instance = menu(e);
    love_tester_menu_refresh_page(e, Ptr::new(instance));
}

// Translated from 007957c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007957c0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// The summary page's down handler (unnamed in the engine map; `cdecl`,
/// the button): with an instance, lowers the button's actor value (+0x24)
/// as [`lower_and_refresh`] says.
pub fn fn_007957c0(e: &mut Engine, button: u32) {
    if menu(e) == 0 {
        return;
    }
    let index = e.mem.i32(button + 0x24);
    lower_and_refresh(e, index);
}

// Translated from 007958e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007958e0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// The value down handler (unnamed in the engine map; `cdecl`, the button,
/// not read): with an instance, lowers the page's value ([`fn_00794d30`])
/// as [`lower_and_refresh`] says.
pub fn fn_007958e0(e: &mut Engine, _unused_0: u32) {
    if menu(e) == 0 {
        return;
    }
    let instance = menu(e);
    let index = fn_00794d30(e, Ptr::new(instance));
    lower_and_refresh(e, index);
}

// Translated from 00795a00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00795a00 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// The summary page's up handler (unnamed in the engine map; `cdecl`, the
/// button): with an instance and the button's actor value (+0x24, not -1),
/// plays the up sound (a held handle) while the value is below 10 and,
/// while points remain, raises it (at most 10, `004a8f20`;
/// `PlayerCharacter::SetActorBaseValue`); then the page is refreshed.
pub fn fn_00795a00(e: &mut Engine, button: u32) {
    if menu(e) == 0 {
        return;
    }
    let index = e.mem.i32(button + 0x24);
    if index != -1 {
        let value = player_value(e, index as u32);
        if value < 10 {
            play_sound_held(e, SOUND_VALUE_UP);
        }
        let instance = menu(e);
        let used = e.call(GET_TOTAL_POINTS_USED, &args![instance]).i32();
        if used < e.mem.i32(menu(e) + 0x58) {
            let raised = e
                .call(MIN_004A8F20, &args![value.wrapping_add(1), 10u32])
                .u32();
            let player = e.mem.u32(PLAYER);
            e.call(SET_ACTOR_BASE_VALUE, &args![player, index, raised]);
        }
    }
    let instance = menu(e);
    love_tester_menu_refresh_page(e, Ptr::new(instance));
}

// Translated from 00795ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoveTesterMenu::PreviousPage` (engine map; `cdecl`, the button, not
/// read): with an instance whose page animation is finished, turns the
/// page back and refreshes it.
pub fn love_tester_menu_previous_page(e: &mut Engine, _unused_0: u32) {
    let instance = menu(e);
    if instance == 0 || e.mem.u8(instance + 0x54) == 0 {
        return;
    }
    love_tester_menu_turn_page(e, Ptr::new(instance), false);
    let instance = menu(e);
    love_tester_menu_refresh_page(e, Ptr::new(instance));
}

// Translated from 00795ce0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00795ce0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `LoveTesterMenu::CloseCallback` (engine map; `cdecl`, the button, not
/// read): with an instance whose points are all used
/// (`GetTotalPointsUsed` equal to `iAvailablePoints`), plays the finish
/// sound and closes the machine (`007917a0`); otherwise plays the refused
/// sound.
pub fn love_tester_menu_close_callback(e: &mut Engine, _unused_0: u32) {
    if menu(e) != 0 {
        let instance = menu(e);
        let used = e.call(GET_TOTAL_POINTS_USED, &args![instance]).i32();
        if used == e.mem.i32(menu(e) + 0x58) {
            play_sound(e, SOUND_FINISH);
            fn_007917a0(e);
            return;
        }
    }
    play_sound(e, SOUND_REFUSED);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00792760, fn_00792760(Ptr) -> f64),
        entry!(0x0079_15f0, love_tester_menu_create(u32)),
        entry!(0x0079_1740, fn_00791740() -> u32),
        entry!(0x0079_1750, fn_00791750(Ptr, Ptr) -> bool),
        entry!(0x0079_17a0, fn_007917a0()),
        entry!(0x0079_1a80, fn_00791a80(Ptr, u32) -> Ptr),
        entry!(0x0079_1ab0, love_tester_menu_do_idle(Ptr)),
        entry!(0x0079_1e40, love_tester_menu_do_click(Ptr, u32, u32)),
        entry!(0x0079_2780, fn_00792780(Ptr, u8)),
        entry!(0x0079_28e0, love_tester_menu_init_machine(Ptr)),
        entry!(0x0079_45f0, love_tester_menu_init_camera(Ptr) -> u32),
        entry!(0x0079_48d0, fn_007948d0()),
        entry!(0x0079_49f0, love_tester_menu_turn_page(Ptr, bool) -> bool),
        entry!(0x0079_4d30, fn_00794d30(Ptr) -> i32),
        entry!(0x0079_4e70, fn_00794e70(Ptr, u32, u32, u32) -> Ptr),
        (0x0079_4ff0, actor_value_button_entry as AbiFn),
        entry!(0x0079_50b0, fn_007950b0(Ptr, u32, u32, u32, u32, u8, u32) -> Ptr),
        entry!(0x0079_5390, fn_00795390(u32)),
        entry!(0x0079_53f0, fn_007953f0(u32)),
        entry!(0x0079_5580, fn_00795580(u32)),
        entry!(0x0079_5630, fn_00795630(u32)),
        entry!(0x0079_5700, fn_00795700(u32)),
        entry!(0x0079_5780, fn_00795780(u32)),
        entry!(0x0079_5b30, fn_00795b30()),
        entry!(0x0079_5c60, love_tester_menu_next_page()),
        entry!(0x0079_5dc0, fn_00795dc0(Ptr)),
        entry!(0x0079_5e20, love_tester_menu_refresh_page(Ptr)),
        entry!(
            0x0079_6100,
            love_tester_menu_load_button_and_text_images(Ptr)
        ),
        entry!(0x0079_15e0, fn_007915e0()),
        entry!(0x0079_1cf0, love_tester_menu_do_gamepad(Ptr, u32, u32) -> u8),
        entry!(0x0079_27a0, love_tester_menu_do_key(Ptr, u32) -> u8),
        entry!(0x0079_57c0, fn_007957c0(u32)),
        entry!(0x0079_58e0, fn_007958e0(u32)),
        entry!(0x0079_5a00, fn_00795a00(u32)),
        entry!(0x0079_5ca0, love_tester_menu_previous_page(u32)),
        entry!(0x0079_5ce0, love_tester_menu_close_callback(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_float_at_0x40_into_st0() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x44);
        e.mem.set_f32(object + 0x3c, 9.0);
        for value in [0.0f32, -2.5, 5.0e-39, f32::INFINITY] {
            e.mem.set_f32(object + 0x40, value);
            assert_eq!(e.call(0x00792760, &[object]).f64(), value as f64);
        }
    }
}

#[cfg(test)]
mod menu_tests {
    use super::*;

    fn called(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == target)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn silence(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    /// A `BSSimpleArray` holding `items` (buffer +4, size +8).
    fn array_at(e: &mut Engine, array: u32, items: &[u32]) {
        let buffer = e.mem.alloc(4 * items.len() as u32 + 4);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *item);
        }
        e.mem.set_u32(array + 4, buffer);
        e.mem.set_u32(array + 8, items.len() as u32);
    }

    /// The menu instance, the player whose values are `values[index]` (slot +0x1c) and
    /// `after[index]` (slot 0), array and pointer helpers, the points total doubled.
    fn engine(values: [i32; 12], used: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        for page in [
            0x011a_0000,
            0x011c_3000,
            0x011d_a000,
            0x011d_e000,
            0x011f_9000,
            0x011a_d000,
        ] {
            e.map(page, 0x1000);
        }
        let this = e.mem.alloc(0x230);
        e.mem.set_u32(MENU_INSTANCE, this);
        let owner_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(owner_vtable + 0x1c, 0x601c);
        e.mem.set_u32(owner_vtable, 0x6000);
        e.register_double(0x601c, move |_, a| values[a[1] as usize].into_ret());
        e.register_double(0x6000, move |_, a| (values[a[1] as usize] + 100).into_ret());
        let player = e.mem.alloc(0xa8);
        e.mem.set_u32(player + 0xa4, owner_vtable);
        e.mem.set_u32(PLAYER, player);
        e.register(ARRAY_SIZE, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(ARRAY_ELEMENT, |e, a| {
            (e.mem.u32(a[0] + 4) + 4 * a[1]).into_ret()
        });
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register_double(GET_TOTAL_POINTS_USED, move |_, _| used.into_ret());
        e.register(AUDIO_INSTANCE, |_, _| 0xa0d0u32.into_ret());
        e.register(GET_SOUND_HANDLE_BY_NAME, |_, a| a[1].into_ret());
        silence(
            &mut e,
            &[
                SOUND_HANDLE_PLAY,
                SOUND_HANDLE_DESTRUCTOR,
                SOUND_HANDLE_CONSTRUCTOR,
                SOUND_HANDLE_ASSIGN,
                SET_CULLED_00450F90,
                SET_ACTOR_BASE_VALUE,
                FREE,
                ARRAY_CLEAR,
            ],
        );
        e.register(HARDCORE_004B71D0, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        (e, this)
    }

    fn sounds(e: &Engine) -> Vec<u32> {
        called(e, GET_SOUND_HANDLE_BY_NAME)
            .iter()
            .map(|w| w[2])
            .collect()
    }

    #[test]
    fn class_id_and_tile_check() {
        let (mut e, this) = engine([0; 12], 0);
        assert_eq!(e.call(0x0079_1740, &[]).u32(), 0x432);
        let index = e.mem.alloc(4);
        for i in 0..8 {
            e.mem.set_u32(this + 0x28 + 4 * i, 1);
        }
        assert!(e.call(0x0079_1750, &[this, index]).bool());
        assert_eq!(e.mem.u32(index), 8);
        e.mem.set_u32(this + 0x28 + 4 * 3, 0);
        assert!(!e.call(0x0079_1750, &[this, index]).bool());
        assert_eq!(e.mem.u32(index), 3);
    }

    /// The create doubles: the tile reader, a menu with the given id.
    fn create_engine(id: u32, tiles: bool) -> (Engine, u32) {
        let (mut e, this) = engine([0; 12], 0);
        e.mem.set_u32(MENU_INSTANCE, 0);
        let vtable = e.mem.alloc(0x38);
        e.mem.set_u32(vtable + 0x34, 0x7034);
        e.mem.set_u32(vtable, 0x7000);
        e.register_double(0x7034, move |_, _| id.into_ret());
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 4, 1);
        if tiles {
            for i in 0..8 {
                e.mem.set_u32(this + 0x28 + 4 * i, 1);
            }
        }
        let tile = e.mem.alloc(8);
        let tile_vtable = e.mem.alloc(4);
        e.mem.set_u32(tile_vtable, 0x7100);
        e.mem.set_u32(tile, tile_vtable);
        e.register(GLOBAL_004B7210, |_, _| 0x4b72u32.into_ret());
        e.register(OBJECT_00586150, |_, _| 0x5861u32.into_ret());
        e.register_double(TILE_READ_FILE, move |_, _| tile.into_ret());
        e.register_double(TILE_GET_MENU, move |_, _| this.into_ret());
        silence(
            &mut e,
            &[
                0x7000,
                0x7100,
                MENU_SET_MENU_TILE,
                REPORT_005B5E40,
                EMERGENCY_CLOSE_ALL_MENUS,
                INIT_MACHINE,
                SCRIPT_005CFCB0,
                MENU_PREP_FOR_VISIBILITY,
            ],
        );
        e.register(WORD_4_00726070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        camera_doubles(&mut e);
        (e, this)
    }

    /// Doubles for everything [`love_tester_menu_init_camera`] calls.
    fn camera_doubles(e: &mut Engine) {
        for page in [0x0101_6000, 0x0102_3000, 0x0107_4000, 0x0120_3000] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(FRUSTUM_SCALE_01016264, 0.75);
        e.mem.set_f64(DEGREES_TO_RADIANS_01023128, 0.5);
        e.mem.set_f64(HALF_ANGLE_01074B98, 0.5);
        e.register(ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CAMERA_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(MAKE_COLOR_00414430, |_, a| a[0].into_ret());
        e.register(POINT_00416870, |_, a| a[0].into_ret());
        let main = e.mem.alloc(8);
        let main_vtable = e.mem.alloc(0x104);
        e.mem.set_u32(main_vtable + 0x100, 0x7300);
        e.mem.set_u32(main, main_vtable);
        e.register(0x7300, |_, _| 1000.0f64.into_ret());
        e.register_double(MAIN_CAMERA_0045C670, move |_, _| main.into_ret());
        let settings = e.mem.alloc(8);
        e.mem.set_f32(settings, 10.0);
        e.mem.set_f32(settings + 4, 8.0);
        e.register_double(SETTING_VALUE, move |_, a| {
            if a[0] == SETTING_01203144 {
                settings
            } else {
                settings + 4
            }
            .into_ret()
        });
        e.register(SCREEN_WIDTH_004DC200, |_, _| 1280u32.into_ret());
        e.register(SCREEN_HEIGHT_004DC1F0, |_, _| 640u32.into_ret());
        e.register(TANGENT_0057DD70, |_, a| {
            (f64::from(f32::from_bits(a[0])) * 2.0).into_ret()
        });
        e.register(STRING_CONSTRUCTOR, |_, a| a[0].into_ret());
        silence(
            e,
            &[
                CAMERA_SET_COLOR_00712E60,
                FRUSTUM_CONSTRUCTOR,
                CAMERA_SET_RATIO_00507700,
                SET_VIEW_FRUSTUM,
                SET_TRANSLATE_004BC1F0,
                UPDATE_BLOCK_0043D410,
                UPDATE_00A59C60,
                LOOK_AT_WORLD_POINT,
                SET_SCALE_00733FD0,
                SET_NAME_00A5B950,
                STRING_DESTRUCTOR,
            ],
        );
    }

    #[test]
    fn create_builds_the_machine() {
        let (mut e, this) = create_engine(0x432, true);
        e.call(0x0079_15f0, &[12]);
        assert_eq!(called(&e, TILE_READ_FILE), vec![vec![0x5861, MENU_FILE]]);
        assert_eq!(e.mem.u32(MENU_INSTANCE), this);
        assert_eq!(e.mem.u32(this + 0x58), 12);
        assert_eq!(called(&e, INIT_MACHINE), vec![vec![this]]);
        assert_ne!(e.mem.u32(CAMERA_POINTER), 0);
        assert_eq!(called(&e, MENU_PREP_FOR_VISIBILITY), vec![vec![this, 0]]);
    }

    #[test]
    fn create_fails_without_tiles_or_with_another_menu() {
        let (mut e, this) = create_engine(0x432, false);
        e.call(0x0079_15f0, &[12]);
        assert_eq!(
            called(&e, REPORT_005B5E40),
            vec![vec![CREATE_FAILED_MESSAGE, 0]]
        );
        assert_eq!(called(&e, 0x7100).len(), 1);
        assert_eq!(called(&e, EMERGENCY_CLOSE_ALL_MENUS).len(), 1);
        assert!(called(&e, INIT_MACHINE).is_empty());
        // Another menu (word +4 set) is deleted.
        let (mut e, _) = create_engine(0x431, true);
        e.call(0x0079_15f0, &[12]);
        assert_eq!(called(&e, 0x7000).len(), 1);
        assert_eq!(e.mem.u32(MENU_INSTANCE), 0);
        let _ = this;
    }

    #[test]
    fn camera_frustum_from_the_settings() {
        let (mut e, this) = engine([0; 12], 0);
        camera_doubles(&mut e);
        let camera = e.call(0x0079_45f0, &[this]).u32();
        let frustum_call = called(&e, SET_VIEW_FRUSTUM);
        let frustum = frustum_call[0][1];
        // Half angle 8 * 0.5 * 0.5 = 2, tangent 4, scale 0.75, aspect 2.
        assert_eq!(e.mem.f32(frustum), -3.0);
        assert_eq!(e.mem.f32(frustum + 4), 3.0);
        assert_eq!(e.mem.f32(frustum + 8), 6.0);
        assert_eq!(e.mem.f32(frustum + 0xc), -6.0);
        assert_eq!(
            called(&e, CAMERA_SET_RATIO_00507700),
            vec![vec![camera, 100.0f32.to_bits()]]
        );
        assert_eq!(
            called(&e, SET_TRANSLATE_004BC1F0),
            vec![vec![camera, 0, 0, 1.0f32.to_bits()]]
        );
        let name = called(&e, STRING_CONSTRUCTOR);
        assert_eq!(name[0][1], CAMERA_NAME);
    }

    #[test]
    fn close_frees_the_pages() {
        let (mut e, this) = engine([0; 12], 0);
        e.register(TILE_GET_MENU_BY_CLASS, |_, _| 0x7171u32.into_ret());
        e.register(SCENE_GRAPH_00450B80, |_, _| 0x4500u32.into_ret());
        silence(
            &mut e,
            &[
                RELEASE_MODEL_FILE_0045A5E0,
                SCENE_RESET_004DEF90,
                SHADOW_SCENE_RESET_00B5D180,
                TILE_SET_00700320,
                MENU_START_FADE_OUT,
                POINTER_DESTRUCTOR,
            ],
        );
        let button = e.mem.alloc(0x14);
        array_at(&mut e, this + 0x84 + 0x20 * 2, &[button, 0]);
        array_at(&mut e, this + 0x94 + 0x20 * 2, &[0x7e00]);
        array_at(&mut e, this + 0x1b4, &[0x7e01]);
        e.mem.set_u32(this + 0x5c, 1);
        e.call(0x0079_17a0, &[]);
        assert_eq!(e.mem.u32(this + 0x5c), 0);
        let freed: Vec<u32> = called(&e, FREE).iter().map(|w| w[0]).collect();
        assert_eq!(freed, vec![button, 0x7e00, 0x7e01]);
        assert_eq!(called(&e, ARRAY_CLEAR).len(), 22);
        assert_eq!(called(&e, TILE_SET_00700320), vec![vec![0x7171, 0x1772, 1]]);
        assert_eq!(called(&e, MENU_START_FADE_OUT), vec![vec![this]]);
        // No tile: nothing.
        e.register(TILE_GET_MENU_BY_CLASS, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0079_17a0, &[]);
        assert!(called(&e, ARRAY_CLEAR).is_empty());
    }

    #[test]
    fn button_destructors() {
        let (mut e, _) = engine([0; 12], 0);
        e.register(POINTER_DESTRUCTOR, |_, _| Ret::default());
        let button = e.mem.alloc(0x14);
        e.mem.set_u32(button + 8, 0x55);
        e.call(0x0079_5dc0, &[button]);
        assert_eq!(e.mem.u32(button + 8), 0);
        assert_eq!(called(&e, POINTER_DESTRUCTOR), vec![vec![button + 8]]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0079_1a80, &[button, 1]).u32(), button);
        assert_eq!(called(&e, FREE), vec![vec![button]]);
    }

    #[test]
    fn do_idle_advances_the_sequence() {
        let (mut e, this) = engine([0; 12], 0);
        e.map(0x011f_6000, 0x1000);
        e.mem.set_u32(BACKWARD_SEQUENCES + 4 * 3, 0x5e03);
        e.mem.set_u32(this + 0x48, 3);
        e.mem.set_f32(this + 0x50, 1.0);
        e.register(IS_TOP_MENU_ID, |_, _| 1u32.into_ret());
        e.register(OBJECT_00877720, |_, _| 0x8777u32.into_ret());
        e.register(SETTING_STRING_00403DF0, |_, _| 0x3df0u32.into_ret());
        e.register(COMPARE_00408B20, |_, _| 1u32.into_ret());
        e.register(CONTROL_PRESSED_00A24180, |_, _| 0u32.into_ret());
        e.register(MODEL_ROOT_0043B230, |_, _| 0x8000u32.into_ret());
        e.register(STRING_CONSTRUCTOR, |_, a| a[1].into_ret());
        e.register(STRING_DESTRUCTOR, |_, _| Ret::default());
        e.register(GET_SEQUENCE_0047A520, |_, a| (a[1] + 0x10_0000).into_ret());
        e.register(FRAME_TIME_0084D030, |_, _| 0.5f64.into_ret());
        e.register(SEQUENCE_UPDATE, |_, _| Ret::default());
        e.register(SEQUENCE_END_00508120, |_, _| 2.0f64.into_ret());
        e.register(SEQUENCE_TIME_00508100, |_, _| 1.0f64.into_ret());
        e.register(CONTROLLER_MANAGER_00653270, |_, a| {
            u32::from(a[1] == 0x8100).into_ret()
        });
        e.register(PARENT_004A8A90, |_, a| (a[0] + 0x100).into_ret());
        silence(&mut e, &[UPDATE_BLOCK_0043D410, UPDATE_00A59C60, 0x8194]);
        let node = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x98);
        e.mem.set_u32(vtable + 0x94, 0x8194);
        e.mem.set_u32(node, vtable);
        e.register_double(PARENT_004A8A90, move |_, _| node.into_ret());
        e.register_double(CONTROLLER_MANAGER_00653270, move |_, a| {
            u32::from(a[1] == node).into_ret()
        });
        e.call(0x0079_1ab0, &[this]);
        assert_eq!(
            called(&e, CONTROL_PRESSED_00A24180),
            vec![vec![0x8777, 0x1e, 1]]
        );
        assert_eq!(
            called(&e, GET_SEQUENCE_0047A520),
            vec![vec![0x8000, 0x5e03]]
        );
        assert_eq!(e.mem.f32(this + 0x50), 1.5);
        assert_eq!(
            called(&e, SEQUENCE_UPDATE),
            vec![vec![0x10_5e03, 1.5f32.to_bits(), 1]]
        );
        // Time 1.0 is not past the end 2.0: finished.
        assert_eq!(e.mem.u8(this + 0x54), 1);
        assert_eq!(called(&e, 0x8194).len(), 1);
        assert_eq!(called(&e, UPDATE_00A59C60).len(), 1);
    }

    #[test]
    fn do_idle_stops_off_top_or_on_the_control() {
        let (mut e, this) = engine([0; 12], 0);
        e.register(IS_TOP_MENU_ID, |_, _| 0u32.into_ret());
        e.call(0x0079_1ab0, &[this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        let (mut e, this) = engine([0; 12], 0);
        e.register(IS_TOP_MENU_ID, |_, _| 1u32.into_ret());
        e.register(OBJECT_00877720, |_, _| 0x8777u32.into_ret());
        e.register(SETTING_STRING_00403DF0, |_, _| 0x3df0u32.into_ret());
        e.register(COMPARE_00408B20, |_, _| 0u32.into_ret());
        e.register(CONTROL_PRESSED_00A24180, |_, _| 1u32.into_ret());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 0xc, 0x800c);
        e.register(0x800c, |_, _| Ret::default());
        e.mem.set_u32(this, vtable);
        e.call(0x0079_1ab0, &[this]);
        assert_eq!(
            called(&e, CONTROL_PRESSED_00A24180),
            vec![vec![0x8777, 0x10, 1]]
        );
        assert_eq!(called(&e, 0x800c), vec![vec![this, 4, 0]]);
    }

    #[test]
    fn do_click_values_finish_and_choice() {
        // Raise strength (state 1 shows value 5).
        let mut values = [0; 12];
        values[5] = 4;
        let (mut e, this) = engine(values, 2);
        e.mem.set_u32(this + 0x48, 1);
        e.mem.set_u32(this + 0x58, 5);
        e.register(MIN_004A8F20, |_, a| a[0].min(a[1]).into_ret());
        e.call(0x0079_1e40, &[this, 2, 0]);
        let player = e.mem.u32(PLAYER);
        assert_eq!(called(&e, SET_ACTOR_BASE_VALUE), vec![vec![player, 5, 5]]);
        // After (104) above before (4): the up sound.
        assert_eq!(sounds(&e), vec![SOUND_VALUE_UP]);
        // Lower it: the down sound when the value after is below.
        e.register(MAX_00647B70, |_, a| {
            (a[0] as i32).max(a[1] as i32).into_ret()
        });
        e.register_double(0x6000, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0079_1e40, &[this, 3, 0]);
        assert_eq!(called(&e, SET_ACTOR_BASE_VALUE), vec![vec![player, 5, 3]]);
        assert_eq!(sounds(&e), vec![SOUND_VALUE_DOWN]);
        // Finish with all points spent closes the machine; otherwise refused.
        e.register(TILE_GET_MENU_BY_CLASS, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0079_1e40, &[this, 4, 0]);
        assert_eq!(sounds(&e), vec![SOUND_REFUSED]);
        e.mem.set_u32(this + 0x58, 2);
        e.call_log = Some(vec![]);
        e.call(0x0079_1e40, &[this, 4, 0]);
        assert_eq!(sounds(&e), vec![SOUND_FINISH]);
        assert_eq!(called(&e, TILE_GET_MENU_BY_CLASS).len(), 1);
        // Choice on the summary.
        e.mem.set_u32(this + 0x48, 8);
        e.mem.set_u32(this + 0x4c, 3);
        e.call_log = Some(vec![]);
        e.call(0x0079_1e40, &[this, 6, 0]);
        e.call(0x0079_1e40, &[this, 7, 0]);
        e.call(0x0079_1e40, &[this, 7, 0]);
        assert_eq!(e.mem.u32(this + 0x4c), 4);
        assert_eq!(sounds(&e), vec![SOUND_CHOICE; 3]);
        // An unknown id does nothing.
        e.call_log = Some(vec![]);
        e.call(0x0079_1e40, &[this, 9, 0]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn do_click_picks_buttons_under_the_cursor() {
        let (mut e, this) = engine([0; 12], 0);
        e.mem.set_u32(this + 0x48, 2);
        e.register(CURSOR_OBJECT_004B7210, |_, _| 0x4b72u32.into_ret());
        e.register(CURSOR_X_009A9350, |_, _| 10.0f64.into_ret());
        e.register(FTOL, |_, a| (f64::take(a, &mut 0) as i32).into_ret());
        e.map(0x0000_4000, 0x1000);
        e.mem.set_f32(0x4b72 + 0x40, 20.0);
        e.register(PICK_OBJECTS, |_, _| 1u32.into_ret());
        e.register(PICK_RESULTS_00500940, |_, _| 0x5009u32.into_ret());
        e.register(RESULTS_GET_00968670, |_, _| 1u32.into_ret());
        silence(
            &mut e,
            &[
                POINT_CONSTRUCTOR_006815C0,
                PICK_CONSTRUCTOR,
                PICK_DESTRUCTOR,
                PICK_SET_00458B30,
                PICK_SET_TARGET_00705FC0,
                PICK_SET_006ECD40,
                CAMERA_WINDOW_POINT_TO_RAY,
                0x7c11,
                0x7c22,
            ],
        );
        let button = e.mem.alloc(0x14);
        e.mem.set_u32(button, 0x7c11);
        let global = e.mem.alloc(0x14);
        e.mem.set_u32(global, 0x7c22);
        array_at(&mut e, this + 0x84 + 0x40, &[button, button]);
        array_at(&mut e, this + 0x1a4, &[global]);
        e.call(0x0079_1e40, &[this, 5, 0]);
        // The first page button and the global button are clicked once each.
        assert_eq!(called(&e, 0x7c11), vec![vec![button]]);
        assert_eq!(called(&e, 0x7c22), vec![vec![global]]);
        let rays = called(&e, CAMERA_WINDOW_POINT_TO_RAY);
        assert_eq!(rays[0][1..3], [10, 20]);
        assert_eq!(called(&e, PICK_DESTRUCTOR).len(), 2);
        // Past the summary nothing is picked.
        e.mem.set_u32(this + 0x48, 9);
        e.call_log = Some(vec![]);
        e.call(0x0079_1e40, &[this, 5, 0]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn pick_byte_setter() {
        let (mut e, _) = engine([0; 12], 0);
        let pick = e.mem.alloc(0x34);
        e.call(0x0079_2780, &[pick, 0x101]);
        assert_eq!(e.mem.u8(pick + 0x30), 1);
    }

    #[test]
    fn render_needs_the_camera() {
        let (mut e, _) = engine([0; 12], 0);
        e.call(0x0079_48d0, &[]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        e.mem.set_u32(CAMERA_POINTER, 0xca00);
        e.map(0x0102_3000, 0x1000);
        e.register(OBJECT_0043C4B0, |_, _| 0x43c4u32.into_ret());
        e.register(MAIN_CAMERA_0045C670, |_, _| 0x45c6u32.into_ret());
        e.register(SCENE_008D80E0, |_, _| 0x8d80u32.into_ret());
        e.register(GLOBAL_004B7210, |_, _| 0x4b72u32.into_ret());
        e.register(RENDERED_MENU_007FD640, |_, _| 0xd640u32.into_ret());
        e.register(POINTER_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(SCENE_GRAPH_00450B80, |_, _| 0x4500u32.into_ret());
        silence(
            &mut e,
            &[
                INTERFACE_007148C0,
                SCENE_SET_00C4F270,
                RENDER_TO_TEXTURE_00709D50,
                SCENE_SET_NODE_004A0FD0,
                ACCUMULATE_SCENE,
                RENDER_00B6C0D0,
                SCENE_RESET_00C4F2D0,
                POINTER_DESTRUCTOR,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0079_48d0, &[]);
        assert_eq!(
            called(&e, SCENE_SET_NODE_004A0FD0),
            vec![vec![0x8d80, 0xd640], vec![0x8d80, 0]]
        );
        assert_eq!(
            called(&e, ACCUMULATE_SCENE),
            vec![vec![0xca00, 0x4500, 0x8d80]]
        );
        assert_eq!(called(&e, RENDER_00B6C0D0), vec![vec![0xca00, 0xd640, 0]]);
    }

    /// Sequence doubles for turning pages.
    fn turn_engine() -> (Engine, u32) {
        let (mut e, this) = engine([0; 12], 0);
        e.register(GET_CONTROLLER, |_, _| 0x9c00u32.into_ret());
        e.register(STRING_CONSTRUCTOR, |_, a| a[1].into_ret());
        e.register(GET_SEQUENCE_0047A520, |_, a| (a[1] + 0x10_0000).into_ret());
        silence(
            &mut e,
            &[
                STRING_DESTRUCTOR,
                DEACTIVATE_ALL,
                ACTIVATE_SEQUENCE_0047AAB0,
                SET_FLAG_0047AA40,
                SEQUENCE_START_004954C0,
            ],
        );
        for state in 0..9u32 {
            e.mem
                .set_u32(BACKWARD_SEQUENCES + 4 * state, 0xb000 + state);
        }
        (e, this)
    }

    #[test]
    fn turn_page_moves_and_plays_the_sequence() {
        let (mut e, this) = turn_engine();
        e.mem.set_u32(this + 0x48, 2);
        assert!(e.call(0x0079_49f0, &[this, 1]).bool());
        assert_eq!(e.mem.u32(this + 0x48), 3);
        assert_eq!(e.mem.u8(TURN_FORWARD), 1);
        // The forward table starts at the direction byte itself.
        let name = e.mem.u32(TURN_FORWARD + 12);
        assert_eq!(called(&e, GET_SEQUENCE_0047A520), vec![vec![0x9c00, name]]);
        assert_eq!(sounds(&e), vec![SOUND_PAGE_TURN]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0079_49f0, &[this, 0]).bool());
        assert_eq!(e.mem.u32(this + 0x48), 2);
        assert_eq!(
            called(&e, GET_SEQUENCE_0047A520),
            vec![vec![0x9c00, 0xb002]]
        );
        assert_eq!(
            called(&e, ACTIVATE_SEQUENCE_0047AAB0),
            vec![vec![0x9c00, 0x10_b002, 0, 1, 1.0f32.to_bits(), 0, 0]]
        );
        // Backward at 0 answers false.
        e.mem.set_u32(this + 0x48, 0);
        assert!(!e.call(0x0079_49f0, &[this, 0]).bool());
    }

    #[test]
    fn turn_page_on_the_summary_finishes_or_refuses() {
        let (mut e, this) = turn_engine();
        e.register(TILE_GET_MENU_BY_CLASS, |_, _| 0u32.into_ret());
        e.mem.set_u32(this + 0x48, 8);
        e.mem.set_u32(this + 0x58, 3);
        assert!(!e.call(0x0079_49f0, &[this, 1]).bool());
        assert_eq!(sounds(&e), vec![SOUND_REFUSED]);
        e.mem.set_u32(this + 0x58, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0079_49f0, &[this, 1]).bool());
        assert_eq!(sounds(&e), vec![SOUND_FINISH]);
        assert_eq!(e.mem.u32(this + 0x48), 8);
    }

    #[test]
    fn page_value_index() {
        let (mut e, this) = engine([0; 12], 0);
        for (state, expected) in [(0u32, -1i32), (1, 5), (7, 11), (8, -1), (9, -1)] {
            e.mem.set_u32(this + 0x48, state);
            assert_eq!(e.call(0x0079_4d30, &[this]).i32(), expected);
        }
        e.register(HARDCORE_004B71D0, |_, _| 1u32.into_ret());
        e.mem.set_u32(this + 0x48, 8);
        e.mem.set_u32(this + 0x4c, 2);
        assert_eq!(e.call(0x0079_4d30, &[this]).i32(), 6);
    }

    /// The dynamic model and cabinet resolve names to `name + 0x100` / `name + 0x200`,
    /// objects with property 3 of type 9 (`object + 3`).
    fn object_engine() -> (Engine, u32) {
        let (mut e, this) = engine([0; 12], 0);
        for (field, slot, offset) in [(0x5cu32, 0x709cu32, 0x100u32), (0x60, 0x719c, 0x200)] {
            let vtable = e.mem.alloc(0xa0);
            e.mem.set_u32(vtable + 0x9c, slot);
            let model = e.mem.alloc(4);
            e.mem.set_u32(model, vtable);
            e.mem.set_u32(this + field, model);
            e.register_double(slot, move |_, a| (a[1] + offset).into_ret());
        }
        e.register(STRING_CONSTRUCTOR, |_, a| a[1].into_ret());
        e.register(STRING_DESTRUCTOR, |_, _| Ret::default());
        e.register(GET_PROPERTY, |_, a| (a[0] + a[1]).into_ret());
        e.register(PROPERTY_TYPE_00441110, |_, a| {
            if a[0] & 0x10 != 0 { 7u32 } else { 9 }.into_ret()
        });
        e.register(POINTER_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        (e, this)
    }

    #[test]
    fn button_constructor_finds_object_and_image() {
        let (mut e, _) = object_engine();
        let button = e.mem.alloc(0x14);
        assert_eq!(
            e.call(0x0079_4e70, &[button, 0xc1, 0xc2, 0x20]).u32(),
            button
        );
        assert_eq!(e.mem.u32(button), 0xc1);
        assert_eq!(e.mem.u32(button + 4), 0xc2);
        assert_eq!(e.mem.u32(button + 8), 0x120);
        assert_eq!(e.mem.u32(button + 0xc), 0x123);
        // A property of the wrong type gives no image.
        e.call(0x0079_4e70, &[button, 0xc1, 0xc2, 0x10]);
        assert_eq!(e.mem.u32(button + 0xc), 0);
    }

    #[test]
    fn actor_value_button_adds_its_fields() {
        let (mut e, _) = object_engine();
        e.register(ARRAY_CONSTRUCTOR_00723FA0, |_, _| Ret::default());
        // The double copies each added item (read through its address) to `0x9000..`.
        e.map(0x0000_9000, 0x10);
        e.register(ARRAY_ADD_007CB2E0, |e, a| {
            let item = e.mem.u32(a[1]);
            let count = e.mem.u32(0x9000);
            e.mem.set_u32(0x9004 + 4 * count, item);
            e.mem.set_u32(0x9000, count + 1);
            Ret::default()
        });
        let button = e.mem.alloc(0x28);
        let r = e.call(0x0079_4ff0, &[button, 0xc1, 0xc2, 0x20, 7, 2, 0xa1, 0xa2]);
        assert_eq!(r.u32(), button);
        assert_eq!(e.mem.u32(button + 0x24), 7);
        assert_eq!(e.mem.u32(0x9000), 2);
        assert_eq!(e.mem.u32(0x9004), 0xa1);
        assert_eq!(e.mem.u32(0x9008), 0xa2);
        assert!(called(&e, ARRAY_ADD_007CB2E0)
            .iter()
            .all(|w| w[0] == button + 0x14));
        assert_eq!(e.mem.u32(button + 8), 0x120);
    }

    #[test]
    fn text_item_setup_from_either_model() {
        let (mut e, _) = object_engine();
        let item = e.mem.alloc(0x1c);
        e.call(0x0079_50b0, &[item, 0xf0, 0x20, 0x40, 6, 1, 9]);
        assert_eq!(e.mem.u32(item), 0xf0);
        assert_eq!(e.mem.u32(item + 0xc), 0x120);
        assert_eq!(e.mem.u32(item + 4), 0x123);
        assert_eq!(e.mem.u32(item + 0x10), 0x140);
        assert_eq!(e.mem.u32(item + 8), 0x143);
        assert_eq!(e.mem.u32(item + 0x14), 6);
        assert_eq!(e.mem.u32(item + 0x18), 9);
        // The cabinet and no second name.
        e.call(0x0079_50b0, &[item, 0xf0, 0x20, 0, 6, 0, 9]);
        assert_eq!(e.mem.u32(item + 0xc), 0x220);
        assert_eq!(e.mem.u32(item + 0x10), 0);
        assert_eq!(e.mem.u32(item + 8), 0);
    }

    /// A shader property whose slot +0xf0 records the texture.
    fn property(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0xf4);
        e.mem.set_u32(vtable + 0xf0, 0x70f0);
        e.register(0x70f0, |e, a| {
            e.mem.set_u32(a[0] + 4, a[3]);
            Ret::default()
        });
        let property = e.mem.alloc(8);
        e.mem.set_u32(property, vtable);
        property
    }

    fn textures(e: &mut Engine, this: u32) {
        for i in 0..16u32 {
            e.mem.set_u32(this + 0x1c4 + 4 * i, 0x7700 + i);
        }
        for (slot, value) in [(0x200u32, 0x7a00u32), (0x204, 0x7a04), (0x208, 0x7a08)] {
            e.mem.set_u32(this + slot, value);
        }
    }

    #[test]
    fn value_and_points_texts() {
        let mut values = [0; 12];
        values[6] = 3;
        let (mut e, this) = engine(values, 13);
        textures(&mut e, this);
        let item = e.mem.alloc(0x1c);
        let first = property(&mut e);
        let second = property(&mut e);
        e.mem.set_u32(item + 4, first);
        e.mem.set_u32(item + 8, second);
        e.mem.set_u32(item + 0x14, 6);
        e.call(0x0079_5390, &[item]);
        assert_eq!(e.mem.u32(first + 4), 0x7703);
        // 40 - 13 = 27 points: tens 2, units 7.
        e.mem.set_u32(this + 0x58, 40);
        e.call(0x0079_53f0, &[item]);
        assert_eq!(e.mem.u32(first + 4), 0x7702);
        assert_eq!(e.mem.u32(second + 4), 0x7707);
        // Fewer points than used: tens 0 (texture 0), units 0 for the negative remainder.
        e.mem.set_u32(this + 0x58, 10);
        e.call(0x0079_53f0, &[item]);
        assert_eq!(e.mem.u32(first + 4), 0x7700);
        assert_eq!(e.mem.u32(second + 4), 0x7700);
        // 200 - 13: tens capped at 9.
        e.mem.set_u32(this + 0x58, 200);
        e.call(0x0079_53f0, &[item]);
        assert_eq!(e.mem.u32(first + 4), 0x7709);
        assert_eq!(e.mem.u32(second + 4), 0x7707);
        // Without both texts nothing.
        e.mem.set_u32(item + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x0079_53f0, &[item]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn value_buttons_hide_at_the_limits() {
        let mut values = [0; 12];
        values[5] = 1;
        values[6] = 10;
        values[7] = 5;
        let (mut e, this) = engine(values, 3);
        e.mem.set_u32(this + 0x58, 3);
        let button = e.mem.alloc(0x28);
        e.mem.set_u32(button + 8, 0xb0b0);
        let culled = |e: &Engine| -> Vec<u32> {
            called(e, SET_CULLED_00450F90)
                .iter()
                .map(|w| w[1])
                .collect()
        };
        for (index, down, up) in [(5u32, 1u32, 1u32), (6, 0, 1), (7, 0, 1)] {
            e.mem.set_u32(button + 0x24, index);
            e.call_log = Some(vec![]);
            e.call(0x0079_5580, &[button]);
            assert_eq!(culled(&e), vec![down], "down {index}");
            e.call_log = Some(vec![]);
            e.call(0x0079_5630, &[button]);
            assert_eq!(culled(&e), vec![up], "up {index}");
        }
        // Points left: the up button shows below 10.
        e.mem.set_u32(this + 0x58, 4);
        e.mem.set_u32(button + 0x24, 7);
        e.call_log = Some(vec![]);
        e.call(0x0079_5630, &[button]);
        assert_eq!(culled(&e), vec![0]);
        // Hardcore summary: another value's button is hidden as well.
        e.register(HARDCORE_004B71D0, |_, _| 1u32.into_ret());
        e.mem.set_u32(this + 0x48, 8);
        e.mem.set_u32(this + 0x4c, 1);
        e.call_log = Some(vec![]);
        e.call(0x0079_5580, &[button]);
        assert_eq!(culled(&e), vec![0, 1]);
        // No actor value: nothing.
        e.mem.set_u32(button + 0x24, u32::MAX);
        e.call_log = Some(vec![]);
        e.call(0x0079_5580, &[button]);
        e.call(0x0079_5630, &[button]);
        assert!(culled(&e).is_empty());
    }

    #[test]
    fn page_buttons_images() {
        let (mut e, this) = engine([0; 12], 3);
        textures(&mut e, this);
        let button = e.mem.alloc(0x14);
        let image = property(&mut e);
        e.mem.set_u32(button + 0xc, image);
        e.call(0x0079_5780, &[button]);
        assert_eq!(e.mem.u32(image + 4), 0x7a08);
        e.call(0x0079_5700, &[button]);
        assert_eq!(e.mem.u32(image + 4), 0x7a04);
        e.mem.set_u32(this + 0x48, 8);
        e.mem.set_u32(this + 0x58, 4);
        e.call(0x0079_5700, &[button]);
        assert_eq!(e.mem.u32(image + 4), 0x7a00);
    }

    #[test]
    fn value_up_handler_and_next_page() {
        let mut values = [0; 12];
        values[5] = 9;
        let (mut e, this) = engine(values, 0);
        e.register(MIN_004A8F20, |_, a| a[0].min(a[1]).into_ret());
        e.mem.set_u32(this + 0x48, 1);
        e.mem.set_u32(this + 0x58, 1);
        e.call(0x0079_5b30, &[]);
        assert_eq!(sounds(&e), vec![SOUND_VALUE_UP]);
        let player = e.mem.u32(PLAYER);
        assert_eq!(called(&e, SET_ACTOR_BASE_VALUE), vec![vec![player, 5, 10]]);
        // Next page only once the animation is finished.
        e.call_log = Some(vec![]);
        e.call(0x0079_5c60, &[]);
        assert_eq!(e.mem.u32(this + 0x48), 1);
        let (mut e, this) = turn_engine();
        e.mem.set_u8(this + 0x54, 1);
        e.call(0x0079_5c60, &[]);
        assert_eq!(e.mem.u32(this + 0x48), 1);
        // No instance: nothing.
        e.mem.set_u32(MENU_INSTANCE, 0);
        e.call_log = Some(vec![]);
        e.call(0x0079_5c60, &[]);
        e.call(0x0079_5b30, &[]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn refresh_page_glows_and_refresh_functions() {
        let mut values = [0; 12];
        values[6] = 5;
        let (mut e, this) = engine(values, 0);
        e.mem.set_u32(this + 0x48, 2);
        silence(&mut e, &[0x7f00, 0x7f04, 0x7f10, 0x7f14]);
        let card = |e: &mut Engine, threshold: u32| {
            let glow = e.mem.alloc(8);
            e.mem.set_u32(glow, 0xcd00 + threshold);
            e.mem.set_u32(glow + 4, threshold);
            glow
        };
        let low = card(&mut e, 3);
        let high = card(&mut e, 7);
        array_at(&mut e, this + 0x74, &[low, high]);
        let bulb = e.mem.alloc(0x1c);
        array_at(&mut e, this + 0x64, &[bulb]);
        let text = e.mem.alloc(0x1c);
        e.mem.set_u32(text, 0x7f00);
        let button = e.mem.alloc(0x14);
        e.mem.set_u32(button + 4, 0x7f04);
        array_at(&mut e, this + 0x94 + 0x40, &[text]);
        array_at(&mut e, this + 0x84 + 0x40, &[button]);
        let global_text = e.mem.alloc(0x1c);
        e.mem.set_u32(global_text, 0x7f10);
        let global_button = e.mem.alloc(0x14);
        e.mem.set_u32(global_button + 4, 0x7f14);
        array_at(&mut e, this + 0x1b4, &[global_text]);
        array_at(&mut e, this + 0x1a4, &[global_button]);
        e.call(0x0079_5e20, &[this]);
        assert_eq!(
            called(&e, SET_CULLED_00450F90),
            vec![vec![0xcd03, 0], vec![0xcd07, 1]]
        );
        assert_eq!(called(&e, 0x7f00), vec![vec![text]]);
        assert_eq!(called(&e, 0x7f04), vec![vec![button]]);
        assert_eq!(called(&e, 0x7f10), vec![vec![global_text]]);
        assert_eq!(called(&e, 0x7f14), vec![vec![global_button]]);
        // Past the summary nothing.
        e.mem.set_u32(this + 0x48, 9);
        e.call_log = Some(vec![]);
        e.call(0x0079_5e20, &[this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn loads_every_image() {
        let (mut e, this) = engine([0; 12], 0);
        e.mem.set_u32(TES_011DEA10, 0xdea1);
        e.register(LANGUAGE_00709D40, |_, _| 1u32.into_ret());
        silence(
            &mut e,
            &[
                FORMAT,
                SWAP_LANGUAGE_TEXTURE_PATH,
                CREATE_TEXTURE_IMAGE,
                STRING_COPY_00406D30,
            ],
        );
        e.call(0x0079_6100, &[this]);
        let slots: Vec<u32> = called(&e, CREATE_TEXTURE_IMAGE)
            .iter()
            .map(|w| {
                assert_eq!(w[0], 0xdea1);
                assert_eq!(w[3..], [0, 0]);
                w[2] - this
            })
            .collect();
        let mut expected = vec![0x228u32, 0x22c];
        expected.extend((0..11).map(|i| 0x1c4 + 4 * i));
        expected.extend((1..5).map(|i| 0x1ec + 4 * i));
        expected.extend([
            0x204, 0x200, 0x208, 0x20c, 0x210, 0x214, 0x218, 0x21c, 0x220, 0x224,
        ]);
        assert_eq!(slots, expected);
        let copy = called(&e, STRING_COPY_00406D30);
        assert_eq!(copy[0][1..], [5, 0x0101_fd1c]);
        let formats = called(&e, FORMAT);
        assert_eq!(formats[2][2..], [0x0107_4c4c, 0x0107_4c58, 0x0107_4c6c, 0]);
        assert_eq!(
            formats[17][2..],
            [0x0107_4c2c, 0x0107_4c58, copy[0][0], 0x0107_4c38]
        );
    }

    type Added = std::rc::Rc<std::cell::RefCell<Vec<(u32, u32)>>>;

    /// The key the format double writes into a name buffer: the format's low half, then the
    /// two numbers (0xff when not given).
    fn glow_key(format: u32, first: u32, second: u32) -> u32 {
        (format & 0xffff) << 16 | (first & 0xff) << 8 | (second & 0xff)
    }

    /// The machine's objects and doubles: the models (dynamic, cabinet), the manager and its
    /// parent, the scene graph and the light, and the items added to arrays (array, item).
    struct Machine {
        e: Engine,
        this: u32,
        dynamic: u32,
        cabinet: u32,
        manager: u32,
        parent: u32,
        graph: u32,
        added: Added,
        matrices: Added,
    }

    fn machine(height: f64, controller: bool, lights: u32, allocate: bool) -> Machine {
        let (mut e, this) = engine([0; 12], 0);
        for page in [
            0x0101_9000,
            0x0101_f000,
            0x0102_b000,
            0x0102_f000,
            0x0107_4000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(HALF_PI_0101FF38, std::f32::consts::FRAC_PI_2);
        e.mem.set_f32(PI_0102B3C8, std::f32::consts::PI);
        e.mem.set_u32(TILT_01074B90, 0x3e00_adfd);
        e.mem.set_f64(HEIGHT_LIMIT_01074B88, 640.0);
        e.mem.set_f32(PLACE_01074B7C, -85.0);
        e.mem.set_f32(PLACE_01074B80, -75.0);
        e.mem.set_f64(RADIUS_SCALE_0102FC70, 20.0);
        e.mem.set_f32(GREY_01019DE0, 0.85);
        e.mem.set_u32(MODEL_LOADER, 0x3b3c);
        e.mem.set_f32(this + 0x50, 0.5);
        // The models: slot +0xc gives the node, slot +0x9c finds a name (a string's word is
        // its text): the dynamic model every name, the cabinet the `0107xxxx` names and
        // the formatted glow names whose key is listed.
        let mut models = [0u32; 2];
        for (i, (slot, find)) in [(0x7a9cu32, 0x4000_0000u32), (0x7b9c, 0x5000_0000)]
            .into_iter()
            .enumerate()
        {
            let vtable = e.mem.alloc(0xa0);
            e.mem.set_u32(vtable + 0xc, 0x7a0c);
            e.mem.set_u32(vtable + 0x9c, slot);
            let model = e.mem.alloc(4);
            e.mem.set_u32(model, vtable);
            models[i] = model;
            let glows = [
                glow_key(GLOW_NAME_DIGIT_PART, 3, 2),
                glow_key(GLOW_NAME_DIGIT, 3, 0xff),
                glow_key(GLOW_NAME_TEN_PART, 7, 0xff),
                glow_key(GLOW_NAME_TEN, 0xff, 0xff),
            ];
            e.register_double(slot, move |e, a| {
                let text = e.mem.u32(a[1]);
                if find == 0x4000_0000 || text >> 16 == 0x0107 {
                    return (find | (text & 0x00ff_ffff)).into_ret();
                }
                let key = e.mem.u32(text);
                if glows.contains(&key) {
                    (0x6000_0000 | key).into_ret()
                } else {
                    0u32.into_ret()
                }
            });
        }
        let [dynamic, cabinet] = models;
        e.register(0x7a0c, |_, _| 0x0dd0u32.into_ret());
        e.register_double(LOAD_FILE, move |_, a| {
            if a[1] == DYNAMIC_MODEL {
                dynamic
            } else {
                cabinet
            }
            .into_ret()
        });
        e.register(STRING_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(FORMAT, |e, a| {
            let first = a.get(3).copied().unwrap_or(0xff);
            let second = a.get(4).copied().unwrap_or(0xff);
            e.mem.set_u32(a[0], glow_key(a[2], first, second));
            Ret::default()
        });
        // Each product's result is its own address plus the word's index.
        e.register(MATRIX_PRODUCT, |e, a| {
            for i in 0..9 {
                e.mem.set_u32(a[1] + 4 * i, a[1] + i);
            }
            a[1].into_ret()
        });
        let matrices = Added::default();
        let seen = matrices.clone();
        e.register_double(MODEL_SET_MATRIX_0043FA80, move |e, a| {
            for i in 0..9 {
                seen.borrow_mut().push((a[0], e.mem.u32(a[1] + 4 * i)));
            }
            Ret::default()
        });
        e.register_double(SCREEN_HEIGHT_00706E50, move |_, _| height.into_ret());
        let bound = e.mem.alloc(0x10);
        e.mem.set_f32(bound + 0xc, 3.0);
        e.register_double(GET_WORLD_BOUND, move |_, _| bound.into_ret());
        e.register(FRAME_TIME_0084D030, |e, a| {
            if a[0] == TIMER_011F6394 {
                0.25f64
            } else {
                f64::from(e.mem.f32(a[0] + 0xc))
            }
            .into_ret()
        });
        let manager = e.mem.alloc(4);
        let parent = e.mem.alloc(4);
        let parent_vtable = e.mem.alloc(0x98);
        e.mem.set_u32(parent_vtable + 0x94, 0x7c94);
        e.mem.set_u32(parent, parent_vtable);
        e.register_double(GET_CONTROLLER, move |_, _| {
            if controller { manager } else { 0 }.into_ret()
        });
        e.register(GET_SEQUENCE_0047A520, |_, _| 0x05e9u32.into_ret());
        e.register_double(CONTROLLER_MANAGER_00653270, move |_, a| {
            u32::from(a[1] == parent).into_ret()
        });
        e.register_double(PARENT_004A8A90, move |_, a| {
            if a[0] == manager { parent } else { 0 }.into_ret()
        });
        let graph = e.mem.alloc(4);
        let graph_vtable = e.mem.alloc(0xe0);
        e.mem.set_u32(graph_vtable + 0xdc, 0x7ddc);
        e.mem.set_u32(graph, graph_vtable);
        e.register_double(SCENE_GRAPH_00450B80, move |_, _| graph.into_ret());
        e.register_double(ADD_ALL_LIGHTS_FROM_NODE, move |_, _| lights.into_ret());
        e.register_double(ALLOCATE_00AA13E0, move |e, a| {
            if allocate { e.mem.alloc(a[0]) } else { 0 }.into_ret()
        });
        e.register_double(ALLOCATE_00401000, move |e, a| {
            if allocate { e.mem.alloc(a[0]) } else { 0 }.into_ret()
        });
        e.register(LIGHT_CONSTRUCTOR_00A7D670, |_, a| a[0].into_ret());
        e.register(POINT_00416870, |_, a| a[0].into_ret());
        e.register(GLOW_SETUP_004B0680, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            a[0].into_ret()
        });
        let added = Added::default();
        let items = added.clone();
        e.register_double(ARRAY_ADD_007CB2E0, move |e, a| {
            items.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.register(POINTER_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(GET_PROPERTY, |_, _| 0u32.into_ret());
        e.register(LANGUAGE_00709D40, |_, _| 1u32.into_ret());
        silence(
            &mut e,
            &[
                SWAP_LANGUAGE_TEXTURES,
                POINT_CONSTRUCTOR_006815C0,
                MAKE_X_ROTATION,
                ROTATION_004A0C90,
                MODEL_SET_00440490,
                SET_TRANSLATE_004BC1F0,
                UPDATE_BLOCK_0043D410,
                UPDATE_00A59C60,
                STRING_DESTRUCTOR,
                DEACTIVATE_ALL,
                ACTIVATE_SEQUENCE_0047AAB0,
                SET_FLAG_0047AA40,
                SEQUENCE_START_004954C0,
                START_ANIMATIONS,
                0x7c94,
                0x7ddc,
                LIGHT_SET_00440460,
                LIGHT_SET_004BC2E0,
                LIGHT_SET_0050DD50,
                LIGHT_SET_0050DD20,
                ADD_LIGHT,
                ARRAY_CONSTRUCTOR_00723FA0,
                SWAP_LANGUAGE_TEXTURE_PATH,
                CREATE_TEXTURE_IMAGE,
                STRING_COPY_00406D30,
            ],
        );
        Machine {
            e,
            this,
            dynamic,
            cabinet,
            manager,
            parent,
            graph,
            added,
            matrices,
        }
    }

    /// The items added to `array`.
    fn added_to(m: &Machine, array: u32) -> Vec<u32> {
        m.added
            .borrow()
            .iter()
            .filter(|(a, _)| *a == array)
            .map(|(_, item)| *item)
            .collect()
    }

    #[test]
    fn init_machine_builds_models_light_glows_and_pages() {
        let mut m = machine(640.0, true, 0, true);
        let this = m.this;
        m.e.call(0x0079_28e0, &[this]);
        let e = &m.e;
        let (dynamic, cabinet) = (m.dynamic, m.cabinet);
        assert_eq!(
            called(e, LOAD_FILE),
            vec![
                vec![0x3b3c, DYNAMIC_MODEL, 0, 1, 0, 0, 0],
                vec![0x3b3c, CABINET_MODEL, 0, 1, 0, 0, 0]
            ]
        );
        assert_eq!(
            (e.mem.u32(this + 0x5c), e.mem.u32(this + 0x60)),
            (dynamic, cabinet)
        );
        assert_eq!(
            called(e, SWAP_LANGUAGE_TEXTURES),
            vec![vec![dynamic], vec![cabinet]]
        );
        // The rotation: X(π/2) * 004a0c90(π), then * X(tilt), each product copied back.
        let products = called(e, MATRIX_PRODUCT);
        let (rotation, turn) = (products[0][0], products[0][2]);
        assert_eq!(turn, rotation - 0x50);
        assert_eq!(products[0][1], rotation - 0xfc);
        assert_eq!(products[1], vec![rotation, rotation - 0x120, turn]);
        assert_eq!(
            called(e, MAKE_X_ROTATION),
            vec![
                vec![rotation, std::f32::consts::FRAC_PI_2.to_bits()],
                vec![turn, 0x3e00_adfd]
            ]
        );
        assert_eq!(
            called(e, ROTATION_004A0C90),
            vec![vec![turn, std::f32::consts::PI.to_bits()]]
        );
        let second: Vec<(u32, u32)> = (0..9).map(|i| (dynamic, rotation - 0x120 + i)).collect();
        assert_eq!(m.matrices.borrow()[..9], second[..]);
        // A height of exactly 640 places both models at (0, -85, -75).
        let place = [0, (-85.0f32).to_bits(), (-75.0f32).to_bits()];
        let translations = called(e, SET_TRANSLATE_004BC1F0);
        assert_eq!(translations[0][0], dynamic);
        assert_eq!(translations[0][1..], place[..]);
        assert_eq!(translations[1][0], cabinet);
        assert_eq!(translations[1][1..], place[..]);
        assert_eq!(
            called(e, MODEL_SET_00440490),
            vec![
                vec![dynamic, 1.0f32.to_bits()],
                vec![cabinet, 1.0f32.to_bits()]
            ]
        );
        // The timer advanced, "Forward" played, the walk updated the manager's parent.
        assert_eq!(e.mem.f32(this + 0x50), 0.75);
        assert_eq!(
            called(e, ACTIVATE_SEQUENCE_0047AAB0),
            vec![vec![m.manager, 0x05e9, 0, 1, 1.0f32.to_bits(), 0, 0]]
        );
        let string = called(e, GET_SEQUENCE_0047A520)[0][1];
        assert_eq!(e.mem.u32(string), FORWARD_SEQUENCE);
        assert_eq!(called(e, START_ANIMATIONS), vec![vec![dynamic]]);
        assert_eq!(
            called(e, CONTROLLER_MANAGER_00653270),
            vec![
                vec![MANAGER_ARGUMENT_011F36BC, m.manager],
                vec![MANAGER_ARGUMENT_011F36BC, m.parent]
            ]
        );
        assert_eq!(called(e, 0x7c94).len(), 1);
        assert_eq!(called(e, 0x7c94)[0][0], m.parent);
        // No lights in the node: the machine's own light, radius 3 * 20.
        let radius = 60.0f32.to_bits();
        assert_eq!(
            called(e, ADD_ALL_LIGHTS_FROM_NODE),
            vec![vec![m.graph, 0x0dd0, radius]]
        );
        let light = called(e, ADD_LIGHT)[0][1];
        assert_eq!(called(e, ADD_LIGHT), vec![vec![m.graph, light, 0]]);
        let points = called(e, POINT_00416870);
        assert_eq!(points[1][1..], [0.85f32.to_bits(); 3]);
        assert_eq!(points[2][1..], [radius, 0, 0]);
        assert_eq!(
            called(e, LIGHT_SET_0050DD20),
            vec![vec![light, 1.0f32.to_bits()]]
        );
        assert_eq!(
            called(e, 0x7ddc),
            vec![
                vec![m.graph, light, 1],
                vec![m.graph, dynamic, 1],
                vec![m.graph, cabinet, 1]
            ]
        );
        // The glows found in the cabinet, at their values.
        let glows: Vec<(u32, u32)> = called(e, GLOW_SETUP_004B0680)
            .iter()
            .map(|w| (w[1], w[2]))
            .collect();
        assert_eq!(
            glows,
            vec![
                (0x6000_0000 | glow_key(GLOW_NAME_DIGIT_PART, 3, 2), 3),
                (0x6000_0000 | glow_key(GLOW_NAME_DIGIT, 3, 0xff), 3),
                (0x6000_0000 | glow_key(GLOW_NAME_TEN_PART, 7, 0xff), 10),
                (0x6000_0000 | glow_key(GLOW_NAME_TEN, 0xff, 0xff), 10)
            ]
        );
        assert_eq!(added_to(&m, this + 0x74).len(), 4);
        // The pages.
        let global = added_to(&m, this + 0x1a4);
        assert_eq!(global.len(), 4);
        assert_eq!(
            [
                e.mem.u32(global[0]),
                e.mem.u32(global[0] + 4),
                e.mem.u32(global[0] + 0x24)
            ],
            [CLICK_VALUE_DOWN_007958E0, REFRESH_DOWN, u32::MAX]
        );
        assert_eq!(e.mem.u32(global[1]), CLICK_VALUE_UP);
        assert_eq!(e.mem.u32(global[2]), CLICK_NEXT_PAGE);
        assert_eq!(e.mem.u32(global[3] + 4), REFRESH_PREVIOUS);
        assert_eq!(e.mem.u32(global[3] + 8), 0x4000_0000 | 0x0007_4b3c);
        let points_text = added_to(&m, this + 0x1b4);
        assert_eq!(points_text.len(), 1);
        assert_eq!(e.mem.u32(points_text[0]), REFRESH_POINTS_TEXT);
        assert_eq!(e.mem.u32(points_text[0] + 0x10), 0x4000_0000 | 0x0007_4ac8);
        let look_inside = added_to(&m, this + 0x84);
        assert_eq!(e.mem.u32(look_inside[0] + 8), 0x4000_0000 | 0x0007_4a9c);
        let texts = added_to(&m, this + 0x194);
        assert_eq!(texts.len(), 7);
        assert_eq!(e.mem.u32(texts[0] + 0x14), 5);
        assert_eq!(e.mem.u32(texts[6] + 0x14), 11);
        let buttons = added_to(&m, this + 0x184);
        assert_eq!(buttons.len(), 15);
        let strength_down = buttons[0];
        assert_eq!(e.mem.u32(strength_down), CLICK_SUMMARY_DOWN_007957C0);
        assert_eq!(e.mem.u32(strength_down + 0x24), 5);
        assert_eq!(
            added_to(&m, strength_down + 0x14),
            vec![texts[0], points_text[0]]
        );
        assert_eq!(e.mem.u32(buttons[13] + 0x24), 11);
        assert_eq!(e.mem.u32(buttons[13]), CLICK_SUMMARY_UP_00795A00);
        assert_eq!(e.mem.u32(buttons[14]), CLICK_CLOSE);
        // The images, the refresh, and no page turn while the animation runs.
        assert_eq!(called(e, CREATE_TEXTURE_IMAGE)[0][2], this + 0x228);
        assert!(called(e, ARRAY_SIZE).contains(&vec![this + 0x64]));
        assert_eq!(e.mem.u32(this + 0x48), 0);
    }

    #[test]
    fn init_machine_without_controller_allocations_or_with_lights() {
        // An unordered height places the models at (0, -85, -85).
        let mut m = machine(f64::NAN, false, 1, false);
        let this = m.this;
        m.e.mem.set_u8(this + 0x54, 1);
        m.e.call(0x0079_28e0, &[this]);
        let e = &m.e;
        let translations = called(e, SET_TRANSLATE_004BC1F0);
        let place = [0, (-85.0f32).to_bits(), (-85.0f32).to_bits()];
        assert_eq!(translations[0][1..], place);
        assert_eq!(translations[1][1..], place);
        assert!(called(e, GET_SEQUENCE_0047A520).is_empty());
        assert!(called(e, CONTROLLER_MANAGER_00653270).is_empty());
        assert_eq!(called(e, START_ANIMATIONS), vec![vec![m.dynamic]]);
        // The node had lights: no light of its own.
        assert!(called(e, ALLOCATE_00AA13E0).is_empty());
        assert!(called(e, ADD_LIGHT).is_empty());
        assert_eq!(
            called(e, 0x7ddc),
            vec![vec![m.graph, m.dynamic, 1], vec![m.graph, m.cabinet, 1]]
        );
        // Failed allocations: nulls added, nothing set up.
        assert!(called(e, GLOW_SETUP_004B0680).is_empty());
        assert_eq!(added_to(&m, this + 0x74), vec![0; 4]);
        assert_eq!(added_to(&m, this + 0x1a4), vec![0; 4]);
        assert_eq!(added_to(&m, this + 0x1b4), vec![0]);
        assert_eq!(added_to(&m, this + 0x84), vec![0]);
        assert_eq!(added_to(&m, this + 0x194), vec![0; 7]);
        assert_eq!(added_to(&m, this + 0x184), vec![0; 15]);
        assert!(called(e, 0x0079_4e70).is_empty());
        // The animation is finished: the page turns forward.
        assert_eq!(e.mem.u32(this + 0x48), 1);
    }

    /// The machine with a vtable whose click slot (+0xc) is logged and the
    /// top-menu check answering `top`.
    fn input_engine(values: [i32; 12], used: u32, top: bool) -> (Engine, u32) {
        let (mut e, this) = engine(values, used);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 0xc, 0x7a0c);
        e.mem.set_u32(this, vtable);
        e.register(0x7a0c, |_, _| Ret::default());
        e.register_double(IS_TOP_MENU_ID, move |_, _| u32::from(top).into_ret());
        e.mem.set_u32(this + 0x58, 3);
        (e, this)
    }

    #[test]
    fn create_without_words_gives_0x23_points() {
        let (mut e, this) = create_engine(0x432, true);
        e.call(0x0079_15e0, &[]);
        assert_eq!(e.mem.u32(this + 0x58), 0x23);
        assert_eq!(e.mem.u32(MENU_INSTANCE), this);
    }

    #[test]
    fn do_gamepad_raises_and_lowers_the_page_value() {
        let mut values = [0; 12];
        values[5] = 5;
        let (mut e, this) = input_engine(values, 0, true);
        e.mem.set_u32(this + 0x48, 1);
        assert_eq!(e.call(0x0079_1cf0, &[this, 9, 0]).u32() as u8, 1);
        assert_eq!(called(&e, 0x7a0c), vec![vec![this, 2, 0]]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0079_1cf0, &[this, 2, 0]).u32() as u8, 1);
        assert_eq!(called(&e, 0x7a0c), vec![vec![this, 3, 0]]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0079_1cf0, &[this, 1, 0]).u32() as u8, 1);
        assert_eq!(e.call(0x0079_1cf0, &[this, 10, 0]).u32() as u8, 1);
        // 3 and 4 answer 0 in any state; so do 1 and 2 on the summary.
        assert_eq!(e.call(0x0079_1cf0, &[this, 3, 0]).u32() as u8, 0);
        e.mem.set_u32(this + 0x48, 8);
        assert_eq!(e.call(0x0079_1cf0, &[this, 3, 0]).u32() as u8, 0);
        assert_eq!(e.call(0x0079_1cf0, &[this, 4, 0]).u32() as u8, 0);
        assert_eq!(e.call(0x0079_1cf0, &[this, 1, 0]).u32() as u8, 0);
        assert_eq!(e.call(0x0079_1cf0, &[this, 5, 0]).u32() as u8, 0);
        // On the summary (no hardcore) there is no page value: 9 still
        // answers 1 without a click.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0079_1cf0, &[this, 9, 0]).u32() as u8, 1);
        assert!(called(&e, 0x7a0c).is_empty());
        // At 10 the value is not raised.
        let mut values = [0; 12];
        values[5] = 10;
        let (mut e, this) = input_engine(values, 0, true);
        e.mem.set_u32(this + 0x48, 1);
        assert_eq!(e.call(0x0079_1cf0, &[this, 1, 0]).u32() as u8, 1);
        assert!(called(&e, 0x7a0c).is_empty());
        // Not the top menu: nothing.
        let (mut e, this) = input_engine(values, 0, false);
        assert_eq!(e.call(0x0079_1cf0, &[this, 9, 0]).u32() as u8, 0);
    }

    #[test]
    fn do_key_clicks_and_changes_the_page_value() {
        let mut values = [0; 12];
        values[6] = 1;
        let (mut e, this) = input_engine(values, 0, true);
        assert_eq!(e.call(0x0079_27a0, &[this, 0x8000_0001]).u32() as u8, 1);
        assert_eq!(e.call(0x0079_27a0, &[this, 0x8000_0002]).u32() as u8, 1);
        assert_eq!(called(&e, 0x7a0c), vec![vec![this, 1, 0], vec![this, 0, 0]]);
        e.mem.set_u32(this + 0x48, 2);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0079_27a0, &[this, 0x2b]).u32() as u8, 1);
        assert_eq!(e.call(0x0079_27a0, &[this, 0x3d]).u32() as u8, 1);
        assert_eq!(e.call(0x0079_27a0, &[this, 0x8000_0003]).u32() as u8, 1);
        assert_eq!(called(&e, 0x7a0c), vec![vec![this, 2, 0]; 3]);
        // The value is 1: lowering does nothing and answers 0.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0079_27a0, &[this, 0x2d]).u32() as u8, 0);
        assert_eq!(e.call(0x0079_27a0, &[this, 0x5f]).u32() as u8, 0);
        assert_eq!(e.call(0x0079_27a0, &[this, 0x8000_0004]).u32() as u8, 0);
        assert!(called(&e, 0x7a0c).is_empty());
        // All points used: raising answers 0; other keys 0.
        let (mut e, this) = input_engine(values, 3, true);
        e.mem.set_u32(this + 0x48, 2);
        assert_eq!(e.call(0x0079_27a0, &[this, 0x2b]).u32() as u8, 0);
        assert_eq!(e.call(0x0079_27a0, &[this, 0x41]).u32() as u8, 0);
        assert_eq!(e.call(0x0079_27a0, &[this, 0x8000_0005]).u32() as u8, 0);
    }

    #[test]
    fn summary_and_value_down_handlers_lower_the_value() {
        let mut values = [0; 12];
        values[5] = 5;
        values[7] = 1;
        let (mut e, this) = engine(values, 0);
        e.register(MAX_00647B70, |_, a| {
            ((a[0] as i32).max(a[1] as i32) as u32).into_ret()
        });
        let button = e.mem.alloc(0x28);
        e.mem.set_u32(button + 0x24, 5);
        e.call(0x0079_57c0, &[button]);
        let player = e.mem.u32(PLAYER);
        assert_eq!(sounds(&e), vec![SOUND_VALUE_DOWN]);
        assert_eq!(called(&e, SET_ACTOR_BASE_VALUE), vec![vec![player, 5, 4]]);
        // At 1: no sound, kept at 1.
        e.mem.set_u32(button + 0x24, 7);
        e.call_log = Some(vec![]);
        e.call(0x0079_57c0, &[button]);
        assert!(sounds(&e).is_empty());
        assert_eq!(called(&e, SET_ACTOR_BASE_VALUE), vec![vec![player, 7, 1]]);
        // No actor value: only the refresh.
        e.mem.set_u32(button + 0x24, 0xffff_ffff);
        e.call_log = Some(vec![]);
        e.call(0x0079_57c0, &[button]);
        assert!(called(&e, SET_ACTOR_BASE_VALUE).is_empty());
        // The value down handler uses the page's value (state 1: 5).
        e.mem.set_u32(this + 0x48, 1);
        e.call_log = Some(vec![]);
        e.call(0x0079_58e0, &[0]);
        assert_eq!(called(&e, SET_ACTOR_BASE_VALUE), vec![vec![player, 5, 4]]);
        // No instance: nothing.
        e.mem.set_u32(MENU_INSTANCE, 0);
        e.call_log = Some(vec![]);
        e.call(0x0079_57c0, &[button]);
        e.call(0x0079_58e0, &[0]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn summary_up_handler_raises_while_points_remain() {
        let mut values = [0; 12];
        values[5] = 9;
        let (mut e, this) = engine(values, 0);
        e.register(MIN_004A8F20, |_, a| a[0].min(a[1]).into_ret());
        e.mem.set_u32(this + 0x58, 1);
        let button = e.mem.alloc(0x28);
        e.mem.set_u32(button + 0x24, 5);
        e.call(0x0079_5a00, &[button]);
        let player = e.mem.u32(PLAYER);
        assert_eq!(sounds(&e), vec![SOUND_VALUE_UP]);
        assert_eq!(called(&e, SET_ACTOR_BASE_VALUE), vec![vec![player, 5, 10]]);
        // No points left: the sound but no change.
        let (mut e, this) = engine(values, 1);
        e.mem.set_u32(this + 0x58, 1);
        let button = e.mem.alloc(0x28);
        e.mem.set_u32(button + 0x24, 5);
        e.call(0x0079_5a00, &[button]);
        assert_eq!(sounds(&e), vec![SOUND_VALUE_UP]);
        assert!(called(&e, SET_ACTOR_BASE_VALUE).is_empty());
        e.mem.set_u32(MENU_INSTANCE, 0);
        e.call_log = Some(vec![]);
        e.call(0x0079_5a00, &[button]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn previous_page_turns_back_once_the_animation_finished() {
        let (mut e, this) = turn_engine();
        e.mem.set_u32(this + 0x48, 2);
        e.call(0x0079_5ca0, &[0]);
        assert_eq!(e.mem.u32(this + 0x48), 2);
        e.mem.set_u8(this + 0x54, 1);
        e.call(0x0079_5ca0, &[0]);
        assert_eq!(e.mem.u32(this + 0x48), 1);
        assert_eq!(e.mem.u8(TURN_FORWARD), 0);
        e.mem.set_u32(MENU_INSTANCE, 0);
        e.call_log = Some(vec![]);
        e.call(0x0079_5ca0, &[0]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn close_callback_closes_only_with_all_points_used() {
        let (mut e, this) = engine([0; 12], 3);
        e.register(TILE_GET_MENU_BY_CLASS, |_, _| Ret::default());
        e.mem.set_u32(this + 0x58, 3);
        e.call(0x0079_5ce0, &[0]);
        assert_eq!(sounds(&e), vec![SOUND_FINISH]);
        assert_eq!(called(&e, TILE_GET_MENU_BY_CLASS), vec![vec![0x432]]);
        e.mem.set_u32(this + 0x58, 4);
        e.call_log = Some(vec![]);
        e.call(0x0079_5ce0, &[0]);
        assert_eq!(sounds(&e), vec![SOUND_REFUSED]);
        assert!(called(&e, TILE_GET_MENU_BY_CLASS).is_empty());
        e.mem.set_u32(MENU_INSTANCE, 0);
        e.call_log = Some(vec![]);
        e.call(0x0079_5ce0, &[0]);
        assert_eq!(sounds(&e), vec![SOUND_REFUSED]);
    }
}
