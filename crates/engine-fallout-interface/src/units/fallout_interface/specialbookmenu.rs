//! `fallout/interface/menus/specialbookmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The player (`PlayerCharacter *`, `011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;

// Translated from 00794d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SPECIALBookMenu::GetTotalPointsUsed` (engine map; `thiscall`, the menu not read): the
/// sum (32-bit, wrapping) of the player's values 5 to 11 (the object at the player's
/// +0xa4, its slot +0x1c with each index, in order).
pub fn special_book_menu_get_total_points_used(e: &mut Engine, _this: Ptr) -> u32 {
    let mut total = 0u32;
    for index in 5u32..=11 {
        let player = e.mem.u32(PLAYER);
        let value = e
            .vcall(player.wrapping_add(0xa4), 0x1c, &args![index])
            .u32();
        total = total.wrapping_add(value);
    }
    total
}

/// `pSPECIALBookMenu` (Xbox PDB static of `SPECIALBookMenu`; the code reads the word at
/// `011daa38`, offsets matching the PDB's `pBookGeometry` +0x5C and `Pages` +0x60).
const SPECIAL_BOOK_MENU: u32 = 0x011d_aa38;
/// The `NiPointer` the close clears (`011daa9c`, through `0066b0d0`).
const POINTER_011DAA9C: u32 = 0x011d_aa9c;
/// The model loader (`011c3b3c`) and its release of a model by path (`0045a5e0`, thiscall,
/// one word).
const MODEL_LOADER: u32 = 0x011c_3b3c;
const MODEL_LOADER_RELEASE: u32 = 0x0045_a5e0;
/// `"Meshes\Terminals\Babybook02.NIF"`.
const BOOK_MODEL_PATH: u32 = 0x0107_689c;
/// `007c6110` (no arguments): answers the menu class (0x424).
const MENU_CLASS: u32 = 0x007c_6110;
/// `Tile::GetMenuByClass` (engine map, `00a09030`; cdecl, one word).
const GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// `00450b80` (cdecl, one word): the word at `011f91c8 + 4 * index`.
const OBJECT_BY_INDEX: u32 = 0x0045_0b80;
/// `004def90` and `00b5d180` (thiscall, no stack words) on the object `00450b80(1)` answers.
const OBJECT_CLEAR_004DEF90: u32 = 0x004d_ef90;
const OBJECT_RELEASE_00B5D180: u32 = 0x00b5_d180;
/// `NiPointer` assignment (`0066b0d0`, thiscall, one word).
const SMART_POINTER_SET: u32 = 0x0066_b0d0;
/// The page array's clear (`008454f0`, thiscall, one word: whether to free), for the
/// `BSSimpleArray`s `Buttons` (+0x00) and `TextItems` (+0x10) of each 0x20-byte
/// `SpecialBookPage` (Xbox PDB).
const ARRAY_CLEAR: u32 = 0x0084_54f0;
/// The trait setter from an integer (`00700320`, thiscall on a tile: trait, value; it hands
/// the value converted to a `float` to `00a012d0`).
const SET_TRAIT_FROM_INT: u32 = 0x0070_0320;
/// `Menu::StartFadeOut` (engine map, `00a1d910`; thiscall).
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;

// Translated from 007c6120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SPECIALBookMenu::Close` (engine map; no arguments). When the menu of class `007c6110()` is
/// open (`Tile::GetMenuByClass`): releases the book model (`0045a5e0` on the model loader
/// with `"Meshes\Terminals\Babybook02.NIF"`), runs `004def90` and `00b5d180` on
/// `00450b80(1)` (fetched once for each), clears `pBookGeometry` (+0x5C of the menu at
/// `011daa38`) and the pointer at `011daa9c`, frees both arrays of each of the nine pages
/// (`008454f0` with 1 on +0x60 + 0x20 * page and +0x70 + 0x20 * page); then, when the menu at
/// `011daa38` is still set, trait 0x1772 of the open menu set to 1 (`00700320`) and
/// `Menu::StartFadeOut` on `011daa38`'s menu. The global is read again at each use.
pub fn special_book_menu_close(e: &mut Engine) {
    let class = e.call(MENU_CLASS, &args![]).u32();
    let menu = e.call(GET_MENU_BY_CLASS, &args![class]).u32();
    if menu == 0 {
        return;
    }
    let loader: u32 = e.global(MODEL_LOADER);
    e.call(MODEL_LOADER_RELEASE, &args![loader, BOOK_MODEL_PATH]);
    let object = e.call(OBJECT_BY_INDEX, &args![1u32]).u32();
    e.call(OBJECT_CLEAR_004DEF90, &args![object]);
    let object = e.call(OBJECT_BY_INDEX, &args![1u32]).u32();
    e.call(OBJECT_RELEASE_00B5D180, &args![object]);
    let book: u32 = e.global(SPECIAL_BOOK_MENU);
    e.mem.set_u32(book.wrapping_add(0x5c), 0);
    e.call(SMART_POINTER_SET, &args![POINTER_011DAA9C, 0u32]);
    let mut page = 0i32;
    while page < 9 {
        let offset = (page as u32) << 5;
        let book: u32 = e.global(SPECIAL_BOOK_MENU);
        e.call(
            ARRAY_CLEAR,
            &args![book.wrapping_add(offset).wrapping_add(0x60), 1u32],
        );
        let book: u32 = e.global(SPECIAL_BOOK_MENU);
        e.call(
            ARRAY_CLEAR,
            &args![book.wrapping_add(offset).wrapping_add(0x70), 1u32],
        );
        page += 1;
    }
    if e.global::<u32>(SPECIAL_BOOK_MENU) != 0 {
        e.call(SET_TRAIT_FROM_INT, &args![menu, 0x1772u32, 1u32]);
        let book: u32 = e.global(SPECIAL_BOOK_MENU);
        e.call(MENU_START_FADE_OUT, &args![book]);
    }
}

/// `00559450` (`thiscall`, no stack words): the word at `this` (an `NiPointer`'s object).
const POINTER_GET: u32 = 0x0055_9450;
/// `0043c4b0` (no arguments: the word at `011f4748`) and `007148c0` (`thiscall` on it, two
/// words: when `"ClearBuffer"` is enabled, its vtable +0x188 with them).
const RENDERER_OBJECT: u32 = 0x0043_c4b0;
const CLEAR_BUFFER: u32 = 0x0071_48c0;
/// `0045c670` (no arguments: a global pointer's object) and `008d80e0` (`thiscall`: the word
/// at +0xB4; the map's `DialoguePackage::GetTargetOfConversation`, a shared body): the
/// culling process the render uses.
const SCENE_OBJECT: u32 = 0x0045_c670;
const WORD_AT_B4: u32 = 0x008d_80e0;
/// `00c4f270` (`thiscall`, one word: pushes the culling process's mode, +0x90, and sets the
/// new one) and `00c4f2d0` (`thiscall`: pops it).
const PUSH_CULL_MODE: u32 = 0x00c4_f270;
const POP_CULL_MODE: u32 = 0x00c4_f2d0;
/// `00709d50` (`cdecl`, two words: a camera and a `float`) and its `float` (0.15,
/// `01030ff4`).
const CAMERA_SETUP_00709D50: u32 = 0x0070_9d50;
const CAMERA_VALUE_01030FF4: u32 = 0x0103_0ff4;
/// `004b7210` (no arguments: the interface manager) and `007fd640` (`thiscall` on it: an
/// object's address through `00559450`).
const INTERFACE_MANAGER: u32 = 0x004b_7210;
const INTERFACE_OBJECT_007FD640: u32 = 0x007f_d640;
/// `00633c90` (`thiscall` on an `NiPointer`, one word: stores the object and adds a
/// reference) and `0045cec0` (the `NiPointer` destructor: releases the object).
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `004a0fd0` (`thiscall` on the culling process, one word: through `0066b0d0`).
const CULLER_SET_004A0FD0: u32 = 0x004a_0fd0;
/// `BSShaderUtil::AccumulateScene` (engine map, `00b6bee0`; `cdecl`, three words) and
/// `00b6c0d0` (`cdecl`, three words: camera, accumulator, 0).
const ACCUMULATE_SCENE: u32 = 0x00b6_bee0;
const RENDER_00B6C0D0: u32 = 0x00b6_c0d0;
/// The `NiPointer` the render reads (`011daa9c`, the one `Close` clears).
const CAMERA_POINTER: u32 = POINTER_011DAA9C;

// Translated from 007c9ca0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007c9ca0 - the compiler's exception frame (FS:[0] chain, handler 00f0c4d8, state 0 that releases the stack `NiPointer`) is not translated
/// Unnamed in the engine map (no arguments): renders the book when the menu (`011daa38`)
/// and its pointer at `011daa9c` are both set. Clears the buffer (`007148c0(0, 4)` on
/// `0043c4b0`'s object), takes the culling process (`008d80e0` of `0045c670`'s object),
/// pushes its mode 1, sets the camera up (`00709d50(camera, 0.15)`), holds the interface
/// object (`007fd640` of the interface manager) in a stack `NiPointer`, hands it to the
/// culling process (`004a0fd0`), accumulates the scene of `00450b80(1)`
/// (`BSShaderUtil::AccumulateScene(camera, object, culler)`), renders
/// (`00b6c0d0(camera, object, 0)`), clears the process's object (`004a0fd0(0)`), pops the
/// mode and releases the pointer.
pub fn fn_007c9ca0(e: &mut Engine) {
    if e.global::<u32>(SPECIAL_BOOK_MENU) == 0 {
        return;
    }
    if e.call(POINTER_GET, &args![CAMERA_POINTER]).u32() == 0 {
        return;
    }
    let renderer = e.call(RENDERER_OBJECT, &args![]).u32();
    e.call(CLEAR_BUFFER, &args![renderer, 0u32, 4u32]);
    let scene = e.call(SCENE_OBJECT, &args![]).u32();
    let culler = e.call(WORD_AT_B4, &args![scene]).u32();
    e.call(PUSH_CULL_MODE, &args![culler, 1u32]);
    let camera = e.call(POINTER_GET, &args![CAMERA_POINTER]).u32();
    let value = x87_quiet_bits(e.global::<u32>(CAMERA_VALUE_01030FF4));
    e.call(CAMERA_SETUP_00709D50, &args![camera, value]);
    let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
    let object = e.call(INTERFACE_OBJECT_007FD640, &args![manager]).u32();
    e.with_stack(4, |e, held| {
        e.call(POINTER_CONSTRUCT, &args![held, object]);
        let object = e.call(POINTER_GET, &args![held]).u32();
        e.call(CULLER_SET_004A0FD0, &args![culler, object]);
        let accumulator = e.call(OBJECT_BY_INDEX, &args![1u32]).u32();
        e.call(ACCUMULATE_SCENE, &args![camera, accumulator, culler]);
        let object = e.call(POINTER_GET, &args![held]).u32();
        e.call(RENDER_00B6C0D0, &args![camera, object, 0u32]);
        e.call(CULLER_SET_004A0FD0, &args![culler, 0u32]);
        e.call(POP_CULL_MODE, &args![culler]);
        e.call(POINTER_DESTRUCT, &args![held]);
    });
}

/// `SPECIALBookMenu::eState` (+0x48: the page, 0 to 8 the attribute pages and the index),
/// `eXBoxIndexState` (+0x4C), `fFrameAnimTimer` (+0x50), `bAnimFinished` (+0x54),
/// `iAvailablePoints` (+0x58), `pBookGeometry` (+0x5C), `Pages` (+0x60, nine 0x20-byte
/// `SpecialBookPage`s: `Buttons` at +0, `TextItems` at +0x10) and the textures from +0x180
/// (Xbox PDB offsets, matching the PC code).
const STATE: u32 = 0x48;
const INDEX_STATE: u32 = 0x4c;
const FRAME_ANIM_TIMER: u32 = 0x50;
const ANIM_FINISHED: u32 = 0x54;
const AVAILABLE_POINTS: u32 = 0x58;
const BOOK_GEOMETRY: u32 = 0x5c;
const PAGES: u32 = 0x60;
const PAGE_TEXT_ITEMS: u32 = 0x70;
/// `spAVValues` (+0x180, the digit textures), `spPointsText` (+0x1ac..), the button images
/// from +0x1bc.
const DIGIT_TEXTURES: u32 = 0x180;
/// The menu class (`0x424`).
const SPECIAL_BOOK_MENU_CLASS: u32 = 0x424;
/// The interface's root tile of the interface manager (`00586150`), `Tile::ReadFile`
/// (`00a01b00`), `Tile::GetMenu` (`00a03c90`), `Menu::SetMenuTile` (`00a1dc70`),
/// `Menu::PrepForVisibility` (`00a1dc20`) and the menu's tile (`00726070`).
const MANAGER_ROOT_00586150: u32 = 0x0058_6150;
const TILE_READ_FILE: u32 = 0x00a0_1b00;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
const MENU_TILE_00726070: u32 = 0x0072_6070;
/// `00791750` (`thiscall` on the menu, one word: where the missing tile index goes; a
/// byte: whether every tile is there), the debug message printer (`005b5e40`, `cdecl`),
/// `Interface::EmergencyCloseAllMenusAndBreakStuff` (`007037e0`) and `005cfcb0` (`cdecl`,
/// one word).
const MENU_CHECK_00791750: u32 = 0x0079_1750;
const DEBUG_MESSAGE: u32 = 0x005b_5e40;
const EMERGENCY_CLOSE_ALL_MENUS: u32 = 0x0070_37e0;
const SCRIPT_005CFCB0: u32 = 0x005c_fcb0;
/// `Interface::IsTopMenuID` (`00702450`, `cdecl`) and `004b71d0` (no arguments, a byte).
const IS_TOP_MENU_ID: u32 = 0x0070_2450;
const INTERFACE_TEST_004B71D0: u32 = 0x004b_71d0;
/// The controls (`00877720` on `011dea0c`'s object) and their state query (`00a24180`, two
/// words: control and mode).
const CONTROLS_HOLDER: u32 = 0x011d_ea0c;
const CONTROLS_OF: u32 = 0x0087_7720;
const CONTROL_STATE_00A24180: u32 = 0x00a2_4180;
/// A game setting's string (`00403df0`), the address of its value (`00403e20`) and the
/// case-insensitive compare (`00408b20`, `cdecl`, two words); the language setting
/// (`011c3cb4`) and `"FRENCH"`.
const SETTING_STRING: u32 = 0x0040_3df0;
const SETTING_VALUE: u32 = 0x0040_3e20;
const COMPARE_00408B20: u32 = 0x0040_8b20;
const LANGUAGE_SETTING: u32 = 0x011c_3cb4;
const FRENCH_TEXT: u32 = 0x0101_fd00;
/// The frame timer (`011f6394`) and its seconds (`0084d030`, `ST0`; also used on other
/// objects with the same shape).
const FRAME_TIMER: u32 = 0x011f_6394;
const SECONDS_0084D030: u32 = 0x0084_d030;
/// `NiFixedString` from a `char*` (`00438170`, one word) and its destructor (`004381b0`).
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTROY: u32 = 0x0043_81b0;
/// The book geometry's root object (`0043b230`), the animation calls: the controller of a
/// given RTTI (`NiObjectNET::GetController`, `00a5c570`, on `011f36ac`), the sequence by
/// name (`0047a520`), `NiControllerManager::DeactivateAll` (`0048fef0`, a `float`), the
/// sequence's activation (`0047aab0`, six words), `0047aa40` (one word), `004954c0`,
/// `NiControllerSequence::Update` (`00a34ba0`, two words), its two times (`00508120`,
/// `00508100`, `ST0`), `NiTimeController::StartAnimations` (`00a6d2d0`, `cdecl`), the RTTI
/// test (`00653270`, `cdecl`: `011f36bc` and an object) and the next controller
/// (`004a8a90`).
const GEOMETRY_ROOT_0043B230: u32 = 0x0043_b230;
const GET_CONTROLLER: u32 = 0x00a5_c570;
const CONTROLLER_RTTI_011F36AC: u32 = 0x011f_36ac;
const SEQUENCE_BY_NAME: u32 = 0x0047_a520;
const DEACTIVATE_ALL: u32 = 0x0048_fef0;
const ACTIVATE_SEQUENCE: u32 = 0x0047_aab0;
const CONTROLLER_0047AA40: u32 = 0x0047_aa40;
const SEQUENCE_004954C0: u32 = 0x0049_54c0;
const SEQUENCE_UPDATE: u32 = 0x00a3_4ba0;
const SEQUENCE_TIME_00508120: u32 = 0x0050_8120;
const SEQUENCE_TIME_00508100: u32 = 0x0050_8100;
const START_ANIMATIONS: u32 = 0x00a6_d2d0;
const IS_KIND_OF_00653270: u32 = 0x0065_3270;
const RTTI_011F36BC: u32 = 0x011f_36bc;
const NEXT_CONTROLLER_004A8A90: u32 = 0x004a_8a90;
/// A transform's translation (`0043d410`, three words: x, y, z), a point (`00416870`, three
/// `float`s) and the object's local translation setter (`00a59c60`, one word).
const TRANSLATION_0043D410: u32 = 0x0043_d410;
const POINT_00416870: u32 = 0x0041_6870;
const SET_TRANSLATE_00A59C60: u32 = 0x00a5_9c60;
/// The animation name tables the page turn reads (`011a0858` backward, `011a0878` forward,
/// by page) and the direction of the last turn (`011a0854`).
const BACKWARD_ANIMATIONS: u32 = 0x011a_0858;
const FORWARD_ANIMATIONS: u32 = 0x011a_0878;
const TURNED_FORWARD: u32 = 0x011a_0854;
/// `BSSimpleArray` helpers: the element count (`0044ddc0`: the word at +8), the element's
/// address (`006a7ad0`, one word) and `AddUninitialized` with a value (`007cb2e0`, one
/// word: the address of the value).
const ARRAY_COUNT: u32 = 0x0044_ddc0;
const ARRAY_AT: u32 = 0x006a_7ad0;
const ARRAY_ADD: u32 = 0x007c_b2e0;
/// `BSSimpleArray` constructor (`00723f00`).
const ARRAY_CONSTRUCT: u32 = 0x0072_3f00;
/// A node's child by name (vtable +0x9C), its property (`NiAVObject::GetProperty`,
/// `00a59d30`, one word) and the property's type (`00441110`).
const GET_PROPERTY: u32 = 0x00a5_9d30;
const PROPERTY_TYPE_00441110: u32 = 0x0044_1110;
/// `00450f90` (`thiscall` on a node, one word: hidden or shown).
const SET_HIDDEN_00450F90: u32 = 0x0045_0f90;
/// `BSAudio::QInstance` (`00453a70`), `GetSoundHandleByName` (`00ad7550`, three words),
/// `BSSoundHandle::Play` (`00ad8830`, one word), the handle's constructor (`0041a250`),
/// assignment (`00418900`) and destructor (`00483710`).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const AUDIO_GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_CONSTRUCT: u32 = 0x0041_a250;
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
const SOUND_HANDLE_DESTROY: u32 = 0x0048_3710;
/// `PlayerCharacter::SetActorBaseValue` (`0093a7c0`, two words), the clamps `004a8f20`
/// (`cdecl`: the value and 10) and `00647b70` (`cdecl`: the value and 1).
const SET_ACTOR_BASE_VALUE: u32 = 0x0093_a7c0;
const CLAMP_HIGH_004A8F20: u32 = 0x004a_8f20;
const CLAMP_LOW_00647B70: u32 = 0x0064_7b70;
/// `MemoryManager` allocations (`00401000` and `00aa13e0`, `cdecl`, one word).
const ALLOCATE: u32 = 0x0040_1000;
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
/// `_ftol2` (`00ec62c0`, `ST0`).
const FTOL2: u32 = 0x00ec_62c0;

/// Sound names: `"UIActivateNothing"`, `"OBJBookSpecialNumber"`, `"OBJBookSpecialFocus"`,
/// `"OBJBookSpecialPageTurn"`.
const ACTIVATE_NOTHING_SOUND: u32 = 0x0107_47e0;
const SPECIAL_NUMBER_SOUND: u32 = 0x0107_47ac;
const SPECIAL_FOCUS_SOUND: u32 = 0x0107_4798;
const PAGE_TURN_SOUND: u32 = 0x0107_4ba0;
/// `"Data\Menus\CharGen\SPECIALBookMenu.xml"` and the creation failure message.
const MENU_FILE_TEXT: u32 = 0x0107_6874;
const CREATION_FAILED_TEXT: u32 = 0x0107_4698;

/// The book menu (`pSPECIALBookMenu`).
fn book(e: &Engine) -> u32 {
    e.global(SPECIAL_BOOK_MENU)
}

/// The player's value `index` (its +0xa4 owner's vtable +0x1c).
fn player_value(e: &mut Engine, index: u32) -> i32 {
    let player = e.mem.u32(PLAYER);
    e.vcall(player.wrapping_add(0xa4), 0x1c, &args![index])
        .i32()
}

/// The player's base value `index` (its +0xa4 owner's vtable +0).
fn player_base_value(e: &mut Engine, index: u32) -> i32 {
    let player = e.mem.u32(PLAYER);
    e.vcall(player.wrapping_add(0xa4), 0, &args![index]).i32()
}

/// The texture held by the `NiPointer` at `slot` (`00559450`).
fn texture(e: &mut Engine, slot: u32) -> u32 {
    e.call(POINTER_GET, &args![slot]).u32()
}

/// `node`'s vtable +0xF0 with `(0, 0, texture)`: the texture shown on it.
fn show_texture(e: &mut Engine, node: u32, texture: u32) {
    e.vcall(node, 0xf0, &args![0u32, 0u32, texture]);
}

/// `00450f90` on `node` with `hidden`.
fn set_hidden(e: &mut Engine, node: u32, hidden: u32) {
    e.call(SET_HIDDEN_00450F90, &args![node, hidden]);
}

/// Plays a sound once through a local handle: `GetSoundHandleByName(local, name, 0x121)`,
/// `Play(0)` on the handle it answers and the local's destructor.
fn play_sound(e: &mut Engine, name: u32) {
    e.with_stack(0xc, |e, local| {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let handle = e
            .call(
                AUDIO_GET_SOUND_HANDLE_BY_NAME,
                &args![audio, local, name, 0x121u32],
            )
            .u32();
        e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
        e.call(SOUND_HANDLE_DESTROY, &args![local]);
    });
}

/// Plays a sound through a constructed handle: the handle (`0041a250`) is assigned the one
/// `GetSoundHandleByName(temporary, name, 0x121)` answers, the temporary destroyed, the
/// handle played (`Play(0)`) and destroyed.
fn play_sound_held(e: &mut Engine, name: u32) {
    e.with_stack(0x18, |e, locals| {
        let handle = locals.addr();
        let temporary = handle + 0xc;
        e.call(SOUND_HANDLE_CONSTRUCT, &args![handle]);
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let made = e
            .call(
                AUDIO_GET_SOUND_HANDLE_BY_NAME,
                &args![audio, temporary, name, 0x121u32],
            )
            .u32();
        e.call(SOUND_HANDLE_ASSIGN, &args![handle, made]);
        e.call(SOUND_HANDLE_DESTROY, &args![temporary]);
        e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
        e.call(SOUND_HANDLE_DESTROY, &args![handle]);
    });
}

/// Runs `body` with a stack `NiFixedString` of `name` (`00438170`, destroyed after with
/// `004381b0`); `body` gets the string's address as `00438170` answers it.
fn with_fixed_string<R>(e: &mut Engine, name: u32, body: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.with_stack(4, |e, local| {
        let string = e.call(FIXED_STRING_CONSTRUCT, &args![local, name]).u32();
        let answer = body(e, string);
        e.call(FIXED_STRING_DESTROY, &args![local]);
        answer
    })
}

/// The book geometry's object named `name` (vtable +0x9C of `pBookGeometry` with a stack
/// `NiFixedString`).
fn book_child(e: &mut Engine, name: u32) -> u32 {
    with_fixed_string(e, name, |e, string| {
        let geometry = e.mem.u32(book(e) + BOOK_GEOMETRY);
        e.vcall(geometry, 0x9c, &args![string]).u32()
    })
}

/// The node's texturing property: `GetProperty(3)` when it has one whose type (`00441110`)
/// is 8 to 12, else 0 (the code asks for the property again at each test).
fn texturing_property(e: &mut Engine, node: u32) -> u32 {
    let kept = e.call(GET_PROPERTY, &args![node, 3u32]).u32() != 0 && {
        let property = e.call(GET_PROPERTY, &args![node, 3u32]).u32();
        e.call(PROPERTY_TYPE_00441110, &args![property]).i32() >= 8 && {
            let property = e.call(GET_PROPERTY, &args![node, 3u32]).u32();
            e.call(PROPERTY_TYPE_00441110, &args![property]).i32() <= 0xc
        }
    };
    if kept {
        e.call(GET_PROPERTY, &args![node, 3u32]).u32()
    } else {
        0
    }
}

/// The page's `Buttons` (`text` false) or `TextItems` array of `this`'s current page.
fn page_array(e: &Engine, this: u32, text: bool) -> u32 {
    let page = e.mem.u32(this + STATE) << 5;
    let base = if text { PAGE_TEXT_ITEMS } else { PAGES };
    this.wrapping_add(page).wrapping_add(base)
}

/// The item at `index` of a page array (`006a7ad0`, then the word there).
fn array_item(e: &mut Engine, array: u32, index: u32) -> u32 {
    let slot = e.call(ARRAY_AT, &args![array, index]).u32();
    e.mem.u32(slot)
}

/// Runs the callback at `offset` of every item of `array` (`cdecl`, the item): the code
/// reads the item twice through `006a7ad0`.
fn run_items(e: &mut Engine, array: impl Fn(&Engine) -> u32, offset: u32) {
    let mut index = 0u32;
    loop {
        let list = array(e);
        if index >= e.call(ARRAY_COUNT, &args![list]).u32() {
            return;
        }
        let item = array_item(e, list, index);
        let holder = array_item(e, list, index);
        let callback = e.mem.u32(holder + offset);
        e.call(callback, &args![item]);
        index += 1;
    }
}

// Translated from 007c5fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SPECIALBookMenu::Create` (engine map; no arguments): `Create_ov2(0x23)`.
pub fn special_book_menu_create(e: &mut Engine) {
    special_book_menu_create_ov2(e, 0x23);
}

// Translated from 007c5fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SPECIALBookMenu::Create_ov2` (engine map; `cdecl`, one word: the points to spend). Reads
/// `SPECIALBookMenu.xml` (`Tile::ReadFile` on the interface manager's root); when its menu
/// is of class `0x424` (vtable +0x34): the tile becomes the menu's, the menu becomes
/// `pSPECIALBookMenu`; when `00791750` reports a missing tile the message is logged with
/// its index, the tile deleted (vtable slot 0 with 1) and every menu closed
/// (`EmergencyCloseAllMenusAndBreakStuff`); otherwise `iAvailablePoints` is set, the book
/// built (`InitBook`), its camera made (`InitCamera`, into the `NiPointer` at `011daa9c`),
/// `005cfcb0(1)` run and the menu prepared for visibility. A menu of another class that has
/// a tile is deleted (vtable slot 0 with 1).
pub fn special_book_menu_create_ov2(e: &mut Engine, points: u32) {
    let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
    let root = e.call(MANAGER_ROOT_00586150, &args![manager]).u32();
    let tile = e.call(TILE_READ_FILE, &args![root, MENU_FILE_TEXT]).u32();
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    let made = menu != 0 && e.vcall(menu, 0x34, &args![]).u32() == fn_007c6110(e);
    if !made {
        if menu != 0 && e.call(MENU_TILE_00726070, &args![menu]).u32() != 0 && menu != 0 {
            e.vcall(menu, 0, &args![1u32]);
        }
        return;
    }
    e.call(MENU_SET_MENU_TILE, &args![menu, tile, 0u32]);
    e.set_global(SPECIAL_BOOK_MENU, menu);
    let complete = e.with_stack(4, |e, missing| {
        let book = book(e);
        let complete = e.call(MENU_CHECK_00791750, &args![book, missing]).bool();
        if !complete {
            let index = e.mem.u32(missing.addr());
            e.call(DEBUG_MESSAGE, &args![CREATION_FAILED_TEXT, index]);
        }
        complete
    });
    if !complete {
        if tile != 0 {
            e.vcall(tile, 0, &args![1u32]);
        }
        e.call(EMERGENCY_CLOSE_ALL_MENUS, &args![]);
        return;
    }
    e.mem.set_u32(book(e) + AVAILABLE_POINTS, points);
    let book_menu = Ptr::new(book(e));
    special_book_menu_init_book(e, book_menu);
    let book_menu = Ptr::new(book(e));
    let camera = special_book_menu_init_camera(e, book_menu);
    e.call(SMART_POINTER_SET, &args![POINTER_011DAA9C, camera]);
    e.call(SCRIPT_005CFCB0, &args![1u32]);
    e.call(MENU_PREP_FOR_VISIBILITY, &args![menu, 0u32]);
}

// Translated from 007c6110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): the book menu's class, `0x424`.
pub fn fn_007c6110(_e: &mut Engine) -> u32 {
    SPECIAL_BOOK_MENU_CLASS
}

// Translated from 007c6200 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007c6200 - the C++ exception frame (FS:[0] chain, handler 00f0dfa8, unwind state 0 for the sequence name) is not translated
/// `SPECIALBookMenu::DoIdle` (engine map; `thiscall`). Only while the book is the top menu:
/// the cancel control (0x1e, or 0x10 when the language setting is `"FRENCH"`; mode 1)
/// clicks button 4 (vtable +0xC with 4, 0) and ends the frame. Otherwise, with the book
/// geometry's root object (`0043b230`): unless the last turn was backward onto page 0, the
/// page's turn animation (the forward or backward table by the last turn, by page) is
/// looked up on it; the frame time is added to `fFrameAnimTimer`; with an animation it is
/// updated to the timer and the turn is finished once its end time is not above the other
/// (or on page 0), else finished at once; then the first controller of the `011f36bc`
/// kind moves to the timer (vtable +0x94 with a translation of it). Finally the geometry
/// is put at the origin.
pub fn special_book_menu_do_idle(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if !e
        .call(IS_TOP_MENU_ID, &args![SPECIAL_BOOK_MENU_CLASS])
        .bool()
    {
        return;
    }
    let holder: u32 = e.global(CONTROLS_HOLDER);
    let controls = e.call(CONTROLS_OF, &args![holder]).u32();
    let language = e.call(SETTING_STRING, &args![LANGUAGE_SETTING]).u32();
    let control = if e
        .call(COMPARE_00408B20, &args![language, FRENCH_TEXT])
        .u32()
        == 0
    {
        0x10u32
    } else {
        0x1e
    };
    if e.call(CONTROL_STATE_00A24180, &args![controls, control, 1u32])
        .u32()
        != 0
    {
        e.vcall(base, 0xc, &args![4u32, 0u32]);
        return;
    }
    let geometry = e.mem.u32(base + BOOK_GEOMETRY);
    let mut node = e.call(GEOMETRY_ROOT_0043B230, &args![geometry]).u32();
    if node != 0 {
        let root = node;
        let mut sequence = 0u32;
        if e.mem.u8(TURNED_FORWARD) == 0 || e.mem.i32(base + STATE) > 0 {
            sequence = turn_sequence(e, base, root);
        }
        let seconds = e.call(SECONDS_0084D030, &args![FRAME_TIMER]).f64();
        let timer = x87_store32(x87_add(
            seconds,
            x87_m32(e.mem.u32(base + FRAME_ANIM_TIMER)),
        ));
        e.mem.set_u32(base + FRAME_ANIM_TIMER, timer);
        if sequence != 0 {
            let timer = x87_quiet_bits(e.mem.u32(base + FRAME_ANIM_TIMER));
            e.call(SEQUENCE_UPDATE, &args![sequence, timer, 1u32]);
            let end = e.call(SEQUENCE_TIME_00508120, &args![sequence]).f64();
            let now = e.call(SEQUENCE_TIME_00508100, &args![sequence]).f64();
            // FCOMP; TEST AH,0x41; JNZ: finished unless strictly above.
            let running = now > end && e.mem.i32(base + STATE) > 0;
            e.mem.set_u8(base + ANIM_FINISHED, u8::from(!running));
        } else {
            e.mem.set_u8(base + ANIM_FINISHED, 1);
        }
        while node != 0 {
            if e.call(IS_KIND_OF_00653270, &args![RTTI_011F36BC, node])
                .u32()
                != 0
            {
                let timer = x87_quiet_bits(e.mem.u32(base + FRAME_ANIM_TIMER));
                e.with_stack(0xc, |e, translation| {
                    e.call(TRANSLATION_0043D410, &args![translation, timer, 0u32, 0u32]);
                    e.vcall(node, 0x94, &args![translation]);
                });
                break;
            }
            node = e.call(NEXT_CONTROLLER_004A8A90, &args![node]).u32();
        }
    }
    e.with_stack(0xc, |e, translation| {
        e.call(
            TRANSLATION_0043D410,
            &args![translation, 0.0f32.to_bits(), 0u32, 0u32],
        );
        let geometry = e.mem.u32(base + BOOK_GEOMETRY);
        e.call(SET_TRANSLATE_00A59C60, &args![geometry, translation]);
    });
}

/// The page's turn animation on `controller`: the name from the forward table after a
/// forward turn, the backward table otherwise (by page), through a stack `NiFixedString`
/// (`0047a520`).
fn turn_sequence(e: &mut Engine, this: u32, controller: u32) -> u32 {
    let table = if e.mem.u8(TURNED_FORWARD) != 0 {
        FORWARD_ANIMATIONS
    } else {
        BACKWARD_ANIMATIONS
    };
    let page = e.mem.u32(this + STATE);
    let name = e.mem.u32(table.wrapping_add(page.wrapping_mul(4)));
    with_fixed_string(e, name, |e, string| {
        e.call(SEQUENCE_BY_NAME, &args![controller, string]).u32()
    })
}

// Translated from 007c6440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SPECIALBookMenu::DoGamepad` (engine map; `RET 8`: the button and a word it does not
/// read; the answer in `AL`). Only while the book is the top menu, for the selected value
/// (`007ca030`): buttons 3 and 9 click "increase" (vtable +0xC with 2, 0) when a value is
/// selected, points are left and the value is below 10; buttons 4 and 10 click "decrease"
/// (3, 0) when the value is above 1; those buttons answer true, the others false.
pub fn special_book_menu_do_gamepad(
    e: &mut Engine,
    this: Ptr,
    button: u32,
    _unused_2: u32,
) -> bool {
    let base = this.addr();
    if !e
        .call(IS_TOP_MENU_ID, &args![SPECIAL_BOOK_MENU_CLASS])
        .bool()
    {
        return false;
    }
    let selected = fn_007ca030(e, this);
    match button {
        3 | 9 => {
            if selected != -1
                && (special_book_menu_get_total_points_used(e, this) as i32)
                    < e.mem.i32(base + AVAILABLE_POINTS)
                && player_value(e, selected as u32) < 10
            {
                e.vcall(base, 0xc, &args![2u32, 0u32]);
            }
            true
        }
        4 | 10 => {
            if selected != -1 && player_value(e, selected as u32) > 1 {
                e.vcall(base, 0xc, &args![3u32, 0u32]);
            }
            true
        }
        _ => false,
    }
}

/// The value changed by a click on "increase" (`increase`) or "decrease": `004a8f20(value
/// + 1, 10)` while points are left, `00647b70(value - 1, 1)`, set as the base value.
fn change_value(
    e: &mut Engine,
    this: Ptr,
    index: impl Fn(&mut Engine) -> u32,
    value: i32,
    increase: bool,
) {
    if increase {
        if (special_book_menu_get_total_points_used(e, this) as i32)
            < e.mem.i32(this.addr() + AVAILABLE_POINTS)
        {
            let changed = e
                .call(CLAMP_HIGH_004A8F20, &args![value.wrapping_add(1), 10u32])
                .u32();
            let index = index(e);
            let player = e.mem.u32(PLAYER);
            e.call(SET_ACTOR_BASE_VALUE, &args![player, index, changed]);
        }
    } else {
        let changed = e
            .call(CLAMP_LOW_00647B70, &args![value.wrapping_sub(1), 1u32])
            .u32();
        let index = index(e);
        let player = e.mem.u32(PLAYER);
        e.call(SET_ACTOR_BASE_VALUE, &args![player, index, changed]);
    }
}

// Translated from 007c6550 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007c6550 - the C++ exception frame (FS:[0] chain, handler 00f0e00c, unwind states 0 to 5 for the sound handles and the pick) is not translated
/// `SPECIALBookMenu::DoClick` (engine map; `RET 8`: the button and a word it does not read;
/// the code's jump table). 0 and 1 turn the page forward or back (`TurnPage`) when the
/// last turn finished, then refresh it. 2 (increase) and 3 (decrease) change the selected
/// value (`007ca030`), or on the index page with `004b71d0` the value `eXBoxIndexState + 4`
/// (see `change_value`); when the value then differs from its base value
/// `"OBJBookSpecialNumber"` plays; the page is refreshed. 4 closes the book when every
/// point is spent, else plays `"UIActivateNothing"`. 5 (on pages 0 to 8) picks with the
/// cursor (the interface manager's `009a9350` and `00792760`, `_ftol2`; with the flag at
/// `011f9426` the y is mapped to the viewport at `011ad848`/`011ad84c` with the renderer
/// object's height) through the camera (`00a71080`) and an `NiPick` (`00e98f20(0, 8)`,
/// `00458b30(1)`, `00705fc0` with the button's node, `006ecd40(0)`, `00792780(1)`,
/// `PickObjects`); the first button whose node is hit (`00968670(0)` of the results)
/// has its click callback run (`cdecl`, the button). 6 and 7 move the index selection
/// down (above 1) or up (below 7) with `"OBJBookSpecialFocus"`, then refresh the page.
pub fn special_book_menu_do_click(e: &mut Engine, this: Ptr, button: u32, _unused_2: u32) {
    let base = this.addr();
    match button {
        0 | 1 => {
            if e.mem.u8(base + ANIM_FINISHED) != 0 {
                special_book_menu_turn_page(e, this, u8::from(button == 0));
                special_book_menu_refresh_page(e, this);
            }
        }
        2 | 3 => {
            let increase = button == 2;
            let mut value = 0i32;
            let mut base_value = 0i32;
            if fn_007ca030(e, this) != -1 {
                let index = fn_007ca030(e, this) as u32;
                value = player_value(e, index);
                change_value(e, this, |e| fn_007ca030(e, this) as u32, value, increase);
                let index = fn_007ca030(e, this) as u32;
                base_value = player_base_value(e, index);
            } else if e.call(INTERFACE_TEST_004B71D0, &args![]).bool()
                && e.mem.i32(base + STATE) == 8
            {
                let index = e.mem.u32(base + INDEX_STATE).wrapping_add(4);
                value = player_value(e, index);
                change_value(e, this, |_| index, value, increase);
                base_value = player_base_value(e, index);
            }
            if value != base_value {
                play_sound_held(e, SPECIAL_NUMBER_SOUND);
            }
            special_book_menu_refresh_page(e, this);
        }
        4 => {
            if special_book_menu_get_total_points_used(e, this)
                == e.mem.u32(base + AVAILABLE_POINTS)
            {
                special_book_menu_close(e);
            } else {
                play_sound(e, ACTIVATE_NOTHING_SOUND);
            }
        }
        5 => {
            if e.mem.i32(base + STATE) < 9 {
                pick_button(e, this);
            }
        }
        6 => {
            if e.mem.i32(base + STATE) == 8 && e.mem.i32(base + INDEX_STATE) > 1 {
                let index = e.mem.u32(base + INDEX_STATE).wrapping_sub(1);
                e.mem.set_u32(base + INDEX_STATE, index);
                play_sound(e, SPECIAL_FOCUS_SOUND);
            }
            special_book_menu_refresh_page(e, this);
        }
        7 => {
            if e.mem.i32(base + STATE) == 8 && e.mem.i32(base + INDEX_STATE) < 7 {
                let index = e.mem.u32(base + INDEX_STATE).wrapping_add(1);
                e.mem.set_u32(base + INDEX_STATE, index);
                play_sound(e, SPECIAL_FOCUS_SOUND);
            }
            special_book_menu_refresh_page(e, this);
        }
        _ => {}
    }
}

/// The interface manager's cursor x and y (`009a9350`, `00792760`).
const CURSOR_X_009A9350: u32 = 0x009a_9350;
const CURSOR_Y_00792760: u32 = 0x0079_2760;
/// The viewport flag (`011f9426`), its top and bottom (`011ad848`, `011ad84c`, `float`s) and
/// its width (`011f9434`); the renderer object (`0043c4b0`) and its vtable +0xD4 object whose
/// `0084e3a0` is the height.
const VIEWPORT_FLAG: u32 = 0x011f_9426;
const VIEWPORT_TOP: u32 = 0x011a_d848;
const VIEWPORT_BOTTOM: u32 = 0x011a_d84c;
const VIEWPORT_WIDTH: u32 = 0x011f_9434;
const VALUE_OF_0084E3A0: u32 = 0x0084_e3a0;
/// `NiPick`'s constructor (`00e98f20`, two words), its settings (`00458b30`, `00705fc0`,
/// `006ecd40`, `00792780`, one word each), `PickObjects` (`00e98e20`, three words), its
/// results (`00500940`) with `00968670` (one word) and the destructor (`00e98fa0`); the
/// camera's ray from a screen point (`00a71080`, five words) and the empty point
/// constructor (`006815c0`).
const PICK_CONSTRUCT: u32 = 0x00e9_8f20;
const PICK_00458B30: u32 = 0x0045_8b30;
const PICK_SET_TARGET_00705FC0: u32 = 0x0070_5fc0;
const PICK_006ECD40: u32 = 0x006e_cd40;
const PICK_00792780: u32 = 0x0079_2780;
const PICK_OBJECTS: u32 = 0x00e9_8e20;
const PICK_RESULTS_00500940: u32 = 0x0050_0940;
const PICK_RESULT_00968670: u32 = 0x0096_8670;
const PICK_DESTROY: u32 = 0x00e9_8fa0;
const CAMERA_WINDOW_POINT_TO_RAY: u32 = 0x00a7_1080;
const POINT_SELF_006815C0: u32 = 0x0068_15c0;

/// `DoClick`'s case 5 (see there).
fn pick_button(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
    let value = e.call(CURSOR_X_009A9350, &args![manager]).f64();
    let x = e.call(FTOL2, &args![value]).u32();
    let value = e.call(CURSOR_Y_00792760, &args![manager]).f64();
    let mut y = e.call(FTOL2, &args![value]).u32();
    if e.mem.u8(VIEWPORT_FLAG) != 0 {
        let renderer = e.call(RENDERER_OBJECT, &args![]).u32();
        // FLD bottom; FSTP double (exact).
        let bottom = x87_load32(e.mem.u32(VIEWPORT_BOTTOM));
        let object = e.vcall(renderer, 0xd4, &args![]).u32();
        let height = e.call(VALUE_OF_0084E3A0, &args![object]).u32();
        let scaled = x87_mul(f64::from(height), bottom);
        let span = x87_sub(
            x87_m32(e.mem.u32(VIEWPORT_TOP)),
            x87_load32(e.mem.u32(VIEWPORT_BOTTOM)),
        );
        let offset = x87_mul(span, f64::from(y as i32));
        y = e.call(FTOL2, &args![x87_add(scaled, offset)]).u32();
    }
    let mut index = 0u32;
    loop {
        let buttons = page_array(e, base, false);
        if index >= e.call(ARRAY_COUNT, &args![buttons]).u32() {
            return;
        }
        let hit = e.with_stack(0x58, |e, locals| {
            let origin = locals.addr();
            let direction = origin + 0xc;
            let pick = origin + 0x18;
            e.call(POINT_SELF_006815C0, &args![origin]);
            e.call(POINT_SELF_006815C0, &args![direction]);
            e.call(PICK_CONSTRUCT, &args![pick, 0u32, 8u32]);
            e.call(PICK_00458B30, &args![pick, 1u32]);
            let buttons = page_array(e, base, false);
            let button = array_item(e, buttons, index);
            let node = e.call(POINTER_GET, &args![button + 8]).u32();
            e.call(PICK_SET_TARGET_00705FC0, &args![pick, node]);
            e.call(PICK_006ECD40, &args![pick, 0u32]);
            e.call(PICK_00792780, &args![pick, 1u32]);
            let camera = e.call(POINTER_GET, &args![POINTER_011DAA9C]).u32();
            e.call(
                CAMERA_WINDOW_POINT_TO_RAY,
                &args![camera, x, y, origin, direction, 0u32],
            );
            let mut hit = false;
            if e.call(PICK_OBJECTS, &args![pick, origin, direction, 0u32])
                .bool()
            {
                let results = e.call(PICK_RESULTS_00500940, &args![pick]).u32();
                if e.call(PICK_RESULT_00968670, &args![results, 0u32]).u32() != 0 {
                    let buttons = page_array(e, base, false);
                    let item = array_item(e, buttons, index);
                    let holder = array_item(e, buttons, index);
                    let callback = e.mem.u32(holder);
                    e.call(callback, &args![item]);
                    hit = true;
                }
            }
            e.call(PICK_DESTROY, &args![pick]);
            hit
        });
        if hit {
            return;
        }
        index += 1;
    }
}

// Translated from 007c6c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SPECIALBookMenu::DoKey` (engine map; `RET 4`; the answer in `AL`): keys `0x80000001`
/// and `0x80000002` click button 1 or 0 (vtable +0xC with whether it is `0x80000001`, 0);
/// keys `0x80000003` and `0x80000004` press gamepad button 3 or 4 (vtable +0x38 with it and
/// 1.0). Those answer true.
pub fn special_book_menu_do_key(e: &mut Engine, this: Ptr, key: u32) -> bool {
    let base = this.addr();
    let mut handled = false;
    if key == 0x8000_0001 || key == 0x8000_0002 {
        e.vcall(base, 0xc, &args![u32::from(key == 0x8000_0001), 0u32]);
        handled = true;
    }
    if key == 0x8000_0003 || key == 0x8000_0004 {
        let button = u32::from(key != 0x8000_0003) + 3;
        e.vcall(base, 0x38, &args![button, 1.0f32.to_bits()]);
        handled = true;
    }
    handled
}

/// One construction step of `InitBook`'s page tables, in the exe's order. Locals are
/// `EBP`-relative offsets of the function's frame (the code passes their addresses).
#[derive(Clone, Copy)]
enum Build {
    /// A 0x14-byte button (`007ca080`: click callback, refresh callback, node name), into
    /// the local.
    Button(u32, u32, u32, i32),
    /// A 0x18-byte text item (`007ca2c0`: callback, first and second node names, value),
    /// into the local.
    Text(u32, u32, u32, u32, i32),
    /// A 0x28-byte value button (`ActorValueButton`: click, refresh, name, value, and the
    /// two items in the locals -0x14 and -0x4c), into the local.
    Value(u32, u32, u32, u32, i32),
    /// A copy between locals.
    Copy(i32, i32),
    /// `AddUninitialized` of the local onto the array at `this + offset`.
    Add(u32, i32),
}

/// `InitBook`'s construction steps (read from the exe's code, 007c7101 to 007c98a8).
const BOOK_BUILD: [Build; 164] = [
    Build::Button(0x007c_abb0, 0x007c_a860, 0x0107_4a9c, -0x30),
    Build::Add(0x60, -0x30),
    Build::Text(0x007c_a4d0, 0x0107_6c84, 0, 0x5, -0x14),
    Build::Text(0x007c_a530, 0x0107_6c74, 0, 0xffff_ffff, -0x10),
    Build::Text(0x007c_a5f0, 0x0107_4ab0, 0x0107_4ac8, 0xffff_ffff, -0x4c),
    Build::Value(0x007c_a960, 0x007c_a700, 0x0107_4b68, 0x5, -0x38),
    Build::Value(0x007c_aa80, 0x007c_a7a0, 0x0107_4b54, 0x5, -0x34),
    Build::Button(0x007c_abb0, 0x007c_a860, 0x0107_4b48, -0x30),
    Build::Button(0x007c_abf0, 0x007c_a8a0, 0x0107_4b3c, -0x2c),
    Build::Add(0x90, -0x14),
    Build::Add(0x90, -0x10),
    Build::Add(0x90, -0x4c),
    Build::Copy(-0xf4, -0x38),
    Build::Add(0x80, -0xf4),
    Build::Copy(-0xf8, -0x34),
    Build::Add(0x80, -0xf8),
    Build::Add(0x80, -0x30),
    Build::Add(0x80, -0x2c),
    Build::Text(0x007c_a4d0, 0x0107_6c64, 0, 0x6, -0x14),
    Build::Text(0x007c_a530, 0x0107_6c54, 0, 0xffff_ffff, -0x10),
    Build::Text(0x007c_a5f0, 0x0107_6c24, 0x0107_6c3c, 0xffff_ffff, -0x4c),
    Build::Value(0x007c_a960, 0x007c_a700, 0x0107_6c10, 0x6, -0x38),
    Build::Value(0x007c_aa80, 0x007c_a7a0, 0x0107_6bfc, 0x6, -0x34),
    Build::Button(0x007c_abb0, 0x007c_a860, 0x0107_6bf0, -0x30),
    Build::Button(0x007c_abf0, 0x007c_a8a0, 0x0107_6be4, -0x2c),
    Build::Add(0xb0, -0x14),
    Build::Add(0xb0, -0x10),
    Build::Add(0xb0, -0x4c),
    Build::Copy(-0x134, -0x38),
    Build::Add(0xa0, -0x134),
    Build::Copy(-0x138, -0x34),
    Build::Add(0xa0, -0x138),
    Build::Add(0xa0, -0x30),
    Build::Add(0xa0, -0x2c),
    Build::Text(0x007c_a4d0, 0x0107_6bd4, 0, 0x7, -0x14),
    Build::Text(0x007c_a530, 0x0107_6bc4, 0, 0xffff_ffff, -0x10),
    Build::Text(0x007c_a5f0, 0x0107_6b94, 0x0107_6bac, 0xffff_ffff, -0x4c),
    Build::Value(0x007c_a960, 0x007c_a700, 0x0107_6b80, 0x7, -0x38),
    Build::Value(0x007c_aa80, 0x007c_a7a0, 0x0107_6b6c, 0x7, -0x34),
    Build::Button(0x007c_abb0, 0x007c_a860, 0x0107_6b60, -0x30),
    Build::Button(0x007c_abf0, 0x007c_a8a0, 0x0107_6b54, -0x2c),
    Build::Add(0xd0, -0x14),
    Build::Add(0xd0, -0x10),
    Build::Add(0xd0, -0x4c),
    Build::Copy(-0x174, -0x38),
    Build::Add(0xc0, -0x174),
    Build::Copy(-0x178, -0x34),
    Build::Add(0xc0, -0x178),
    Build::Add(0xc0, -0x30),
    Build::Add(0xc0, -0x2c),
    Build::Text(0x007c_a4d0, 0x0107_6b44, 0, 0x8, -0x14),
    Build::Text(0x007c_a530, 0x0107_6b34, 0, 0xffff_ffff, -0x10),
    Build::Text(0x007c_a5f0, 0x0107_6b04, 0x0107_6b1c, 0xffff_ffff, -0x4c),
    Build::Value(0x007c_a960, 0x007c_a700, 0x0107_6af0, 0x8, -0x38),
    Build::Value(0x007c_aa80, 0x007c_a7a0, 0x0107_6adc, 0x8, -0x34),
    Build::Button(0x007c_abb0, 0x007c_a860, 0x0107_6ad0, -0x30),
    Build::Button(0x007c_abf0, 0x007c_a8a0, 0x0107_6ac4, -0x2c),
    Build::Add(0xf0, -0x14),
    Build::Add(0xf0, -0x10),
    Build::Add(0xf0, -0x4c),
    Build::Copy(-0x1b4, -0x38),
    Build::Add(0xe0, -0x1b4),
    Build::Copy(-0x1b8, -0x34),
    Build::Add(0xe0, -0x1b8),
    Build::Add(0xe0, -0x30),
    Build::Add(0xe0, -0x2c),
    Build::Text(0x007c_a4d0, 0x0107_6ab4, 0, 0x9, -0x14),
    Build::Text(0x007c_a530, 0x0107_6aa4, 0, 0xffff_ffff, -0x10),
    Build::Text(0x007c_a5f0, 0x0107_6a74, 0x0107_6a8c, 0xffff_ffff, -0x4c),
    Build::Value(0x007c_a960, 0x007c_a700, 0x0107_6a60, 0x9, -0x38),
    Build::Value(0x007c_aa80, 0x007c_a7a0, 0x0107_6a4c, 0x9, -0x34),
    Build::Button(0x007c_abb0, 0x007c_a860, 0x0107_6a40, -0x30),
    Build::Button(0x007c_abf0, 0x007c_a8a0, 0x0107_6a34, -0x2c),
    Build::Add(0x110, -0x14),
    Build::Add(0x110, -0x10),
    Build::Add(0x110, -0x4c),
    Build::Copy(-0x1f4, -0x38),
    Build::Add(0x100, -0x1f4),
    Build::Copy(-0x1f8, -0x34),
    Build::Add(0x100, -0x1f8),
    Build::Add(0x100, -0x30),
    Build::Add(0x100, -0x2c),
    Build::Text(0x007c_a4d0, 0x0107_6a24, 0, 0xa, -0x14),
    Build::Text(0x007c_a530, 0x0107_6a14, 0, 0xffff_ffff, -0x10),
    Build::Text(0x007c_a5f0, 0x0107_69e4, 0x0107_69fc, 0xffff_ffff, -0x4c),
    Build::Value(0x007c_a960, 0x007c_a700, 0x0107_69d0, 0xa, -0x38),
    Build::Value(0x007c_aa80, 0x007c_a7a0, 0x0107_69bc, 0xa, -0x34),
    Build::Button(0x007c_abb0, 0x007c_a860, 0x0107_69b0, -0x30),
    Build::Button(0x007c_abf0, 0x007c_a8a0, 0x0107_69a4, -0x2c),
    Build::Add(0x130, -0x14),
    Build::Add(0x130, -0x10),
    Build::Add(0x130, -0x4c),
    Build::Copy(-0x234, -0x38),
    Build::Add(0x120, -0x234),
    Build::Copy(-0x238, -0x34),
    Build::Add(0x120, -0x238),
    Build::Add(0x120, -0x30),
    Build::Add(0x120, -0x2c),
    Build::Text(0x007c_a4d0, 0x0107_6994, 0, 0xb, -0x14),
    Build::Text(0x007c_a530, 0x0107_6984, 0, 0xffff_ffff, -0x10),
    Build::Text(0x007c_a5f0, 0x0107_6954, 0x0107_696c, 0xffff_ffff, -0x4c),
    Build::Value(0x007c_a960, 0x007c_a700, 0x0107_6940, 0xb, -0x38),
    Build::Value(0x007c_aa80, 0x007c_a7a0, 0x0107_692c, 0xb, -0x34),
    Build::Button(0x007c_abb0, 0x007c_a860, 0x0107_6920, -0x30),
    Build::Button(0x007c_abf0, 0x007c_a8a0, 0x0107_6914, -0x2c),
    Build::Add(0x150, -0x14),
    Build::Add(0x150, -0x10),
    Build::Add(0x150, -0x4c),
    Build::Copy(-0x274, -0x38),
    Build::Add(0x140, -0x274),
    Build::Copy(-0x278, -0x34),
    Build::Add(0x140, -0x278),
    Build::Add(0x140, -0x30),
    Build::Add(0x140, -0x2c),
    Build::Text(0x007c_a4d0, 0x0107_4a80, 0, 0x5, -0x14),
    Build::Add(0x170, -0x14),
    Build::Value(0x007c_a960, 0x007c_a700, 0x0107_4a60, 0x5, -0x28c),
    Build::Add(0x160, -0x28c),
    Build::Value(0x007c_aa80, 0x007c_a7a0, 0x0107_4a40, 0x5, -0x298),
    Build::Add(0x160, -0x298),
    Build::Text(0x007c_a4d0, 0x0107_4a24, 0, 0x6, -0x14),
    Build::Add(0x170, -0x14),
    Build::Value(0x007c_a960, 0x007c_a700, 0x0107_4a04, 0x6, -0x2ac),
    Build::Add(0x160, -0x2ac),
    Build::Value(0x007c_aa80, 0x007c_a7a0, 0x0107_49e4, 0x6, -0x2b8),
    Build::Add(0x160, -0x2b8),
    Build::Text(0x007c_a4d0, 0x0107_49c8, 0, 0x7, -0x14),
    Build::Add(0x170, -0x14),
    Build::Value(0x007c_a960, 0x007c_a700, 0x0107_49a8, 0x7, -0x2cc),
    Build::Add(0x160, -0x2cc),
    Build::Value(0x007c_aa80, 0x007c_a7a0, 0x0107_4988, 0x7, -0x2d8),
    Build::Add(0x160, -0x2d8),
    Build::Text(0x007c_a4d0, 0x0107_496c, 0, 0x8, -0x14),
    Build::Add(0x170, -0x14),
    Build::Value(0x007c_a960, 0x007c_a700, 0x0107_494c, 0x8, -0x2ec),
    Build::Add(0x160, -0x2ec),
    Build::Value(0x007c_aa80, 0x007c_a7a0, 0x0107_492c, 0x8, -0x2f8),
    Build::Add(0x160, -0x2f8),
    Build::Text(0x007c_a4d0, 0x0107_490c, 0, 0x9, -0x14),
    Build::Add(0x170, -0x14),
    Build::Value(0x007c_a960, 0x007c_a700, 0x0107_48e8, 0x9, -0x30c),
    Build::Add(0x160, -0x30c),
    Build::Value(0x007c_aa80, 0x007c_a7a0, 0x0107_48c4, 0x9, -0x318),
    Build::Add(0x160, -0x318),
    Build::Text(0x007c_a4d0, 0x0107_48ac, 0, 0xa, -0x14),
    Build::Add(0x170, -0x14),
    Build::Value(0x007c_a960, 0x007c_a700, 0x0107_4890, 0xa, -0x32c),
    Build::Add(0x160, -0x32c),
    Build::Value(0x007c_aa80, 0x007c_a7a0, 0x0107_4874, 0xa, -0x338),
    Build::Add(0x160, -0x338),
    Build::Text(0x007c_a4d0, 0x0107_485c, 0, 0xb, -0x14),
    Build::Add(0x170, -0x14),
    Build::Value(0x007c_a960, 0x007c_a700, 0x0107_4840, 0xb, -0x34c),
    Build::Add(0x160, -0x34c),
    Build::Value(0x007c_aa80, 0x007c_a7a0, 0x0107_4824, 0xb, -0x358),
    Build::Add(0x160, -0x358),
    Build::Text(0x007c_a530, 0x0107_6904, 0, 0xffff_ffff, -0x10),
    Build::Add(0x170, -0x10),
    Build::Text(0x007c_a5f0, 0x0107_68cc, 0x0107_68e8, 0xffff_ffff, -0x4c),
    Build::Add(0x170, -0x4c),
    Build::Button(0x007c_ac30, 0x007c_a8e0, 0x0107_4814, -0x30),
    Build::Add(0x160, -0x30),
    Build::Button(0x007c_abf0, 0x007c_a8a0, 0x0107_68bc, -0x2c),
    Build::Add(0x160, -0x2c),
];

/// `ModelLoader::LoadFile` (`00447080`, `thiscall` on the model loader, six words) and
/// `SwapPlatformLanguageTextures` (`004b7660`, `cdecl`, one word; the caller leaves the word
/// on the stack, where the next call's `float` goes).
const MODEL_LOADER_LOAD_FILE: u32 = 0x0044_7080;
const SWAP_PLATFORM_LANGUAGE_TEXTURES: u32 = 0x004b_7660;
/// The geometry's scale (`00440490`, a `float`), rotation (`004bc1f0`, three `float`s) and
/// `006404b0` (four `float`s: an angle and an axis).
const SET_SCALE_00440490: u32 = 0x0044_0490;
const SET_ROTATION_004BC1F0: u32 = 0x004b_c1f0;
const ROTATE_006404B0: u32 = 0x0064_04b0;
/// The scale (`010181bc`), the rotation's z (`0106503c`) and angle (`0101ff38`) `float`s;
/// the light's colour (`01019de0`, a `float`) and the bound scale (`0102fc70`, a `double`).
const BOOK_SCALE: u32 = 0x0101_81bc;
const BOOK_ROTATION_Z: u32 = 0x0106_503c;
const BOOK_ANGLE: u32 = 0x0101_ff38;
const LIGHT_COLOUR: u32 = 0x0101_9de0;
const BOUND_SCALE_DOUBLE: u32 = 0x0102_fc70;
/// `"Forward"`.
const FORWARD_TEXT: u32 = 0x0104_8458;
/// `NiAVObject::GetWorldBound` (`0043d450`), `ShadowSceneNode::AddAllLightsFromNode_Recurse`
/// (`00b5ca70`, two words), `ShadowSceneNode::AddLight` (`00b5c940`, two words), the light's
/// constructor (`00a7d670`), its ambient, diffuse and position (`00440460`, `004bc2e0`,
/// `0050dd50`, one word each) and dimmer (`0050dd20`, a `float`).
const GET_WORLD_BOUND: u32 = 0x0043_d450;
const ADD_ALL_LIGHTS_FROM_NODE: u32 = 0x00b5_ca70;
const ADD_LIGHT: u32 = 0x00b5_c940;
const LIGHT_CONSTRUCT: u32 = 0x00a7_d670;
const LIGHT_AMBIENT_00440460: u32 = 0x0044_0460;
const LIGHT_DIFFUSE_004BC2E0: u32 = 0x004b_c2e0;
const LIGHT_POSITION_0050DD50: u32 = 0x0050_dd50;
const LIGHT_DIMMER_0050DD20: u32 = 0x0050_dd20;

/// One of `InitBook`'s frame locals (`EBP`-relative).
fn local(ebp: u32, offset: i32) -> u32 {
    ebp.wrapping_add(offset as u32)
}

// Translated from 007c6ce0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007c6ce0 - the C++ exception frame (FS:[0] chain, handler 00f0e46a, unwind states 0 to 0x3c for the sequence name and each new page object) is not translated
/// `SPECIALBookMenu::InitBook` (engine map; `thiscall`). Loads
/// `"Meshes\Terminals\Babybook02.NIF"` into `pBookGeometry` (the model loader, swapped to
/// the platform's language textures), scales, rotates and places it at the origin, adds
/// the frame time to `fFrameAnimTimer` and starts its `"Forward"` animation (deactivating
/// the others) when its controller has one; starts its animations
/// (`NiTimeController::StartAnimations`) and moves the first controller of the
/// `011f36bc` kind to the origin. It is placed at the origin again; 20 times its bound's
/// `0084d030` is the light distance. The scene (`00450b80(1)`) adds the lights under the
/// geometry's vtable +0xC object; without one a point light is made (`00aa13e0(0xfc)`,
/// colour 0.85, at the distance on x, dimmer 1.0) and added (scene vtable +0xDC, then
/// `AddLight`). The geometry is attached to the scene (vtable +0xDC). Then every page's
/// buttons and texts are built and added (`BOOK_BUILD`), the button and text images loaded
/// (`LoadButtonAndTextImages`) and every item of the nine pages refreshed (texts: callback
/// +0; buttons: callback +4).
pub fn special_book_menu_init_book(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let loader: u32 = e.global(MODEL_LOADER);
    let model = e
        .call(
            MODEL_LOADER_LOAD_FILE,
            &args![loader, BOOK_MODEL_PATH, 0u32, 1u32, 0u32, 0u32, 0u32],
        )
        .u32();
    e.mem.set_u32(book(e) + BOOK_GEOMETRY, model);
    let geometry = e.mem.u32(book(e) + BOOK_GEOMETRY);
    e.call(SWAP_PLATFORM_LANGUAGE_TEXTURES, &args![geometry]);
    let geometry = e.mem.u32(book(e) + BOOK_GEOMETRY);
    let scale = x87_quiet_bits(e.mem.u32(BOOK_SCALE));
    e.call(SET_SCALE_00440490, &args![geometry, scale]);
    let geometry = e.mem.u32(book(e) + BOOK_GEOMETRY);
    let z = x87_quiet_bits(e.mem.u32(BOOK_ROTATION_Z));
    e.call(
        SET_ROTATION_004BC1F0,
        &args![geometry, 0.0f32.to_bits(), 0.0f32.to_bits(), z],
    );
    let geometry = e.mem.u32(book(e) + BOOK_GEOMETRY);
    let angle = x87_quiet_bits(e.mem.u32(BOOK_ANGLE));
    e.call(
        ROTATE_006404B0,
        &args![
            geometry,
            angle,
            1.0f32.to_bits(),
            0.0f32.to_bits(),
            0.0f32.to_bits()
        ],
    );
    place_at_origin(e, |e| e.mem.u32(book(e) + BOOK_GEOMETRY));
    let seconds = e.call(SECONDS_0084D030, &args![FRAME_TIMER]).f64();
    let timer = x87_store32(x87_add(
        seconds,
        x87_m32(e.mem.u32(book(e) + FRAME_ANIM_TIMER)),
    ));
    e.mem.set_u32(book(e) + FRAME_ANIM_TIMER, timer);
    let geometry = e.mem.u32(book(e) + BOOK_GEOMETRY);
    let mut node = e
        .call(GET_CONTROLLER, &args![geometry, CONTROLLER_RTTI_011F36AC])
        .u32();
    if node != 0 {
        let controller = node;
        let sequence = with_fixed_string(e, FORWARD_TEXT, |e, string| {
            e.call(SEQUENCE_BY_NAME, &args![controller, string]).u32()
        });
        if sequence != 0 {
            start_sequence(e, controller, sequence);
        }
    }
    let geometry = e.mem.u32(base + BOOK_GEOMETRY);
    e.call(START_ANIMATIONS, &args![geometry]);
    while node != 0 {
        if e.call(IS_KIND_OF_00653270, &args![RTTI_011F36BC, node])
            .u32()
            != 0
        {
            e.with_stack(0xc, |e, translation| {
                e.call(
                    TRANSLATION_0043D410,
                    &args![translation, 0.0f32.to_bits(), 0u32, 0u32],
                );
                e.vcall(node, 0x94, &args![translation]);
            });
            break;
        }
        node = e.call(NEXT_CONTROLLER_004A8A90, &args![node]).u32();
    }
    place_at_origin(e, |e| e.mem.u32(base + BOOK_GEOMETRY));
    let geometry = e.mem.u32(book(e) + BOOK_GEOMETRY);
    let bound = e.call(GET_WORLD_BOUND, &args![geometry]).u32();
    let radius = e.call(SECONDS_0084D030, &args![bound]).f64();
    let factor: f64 = e.global(BOUND_SCALE_DOUBLE);
    let distance = x87_store32(x87_mul(radius, factor));
    let geometry = e.mem.u32(book(e) + BOOK_GEOMETRY);
    let object = e.vcall(geometry, 0xc, &args![]).u32();
    let scene = e.call(OBJECT_BY_INDEX, &args![1u32]).u32();
    let light = e
        .call(
            ADD_ALL_LIGHTS_FROM_NODE,
            &args![scene, object, x87_quiet_bits(distance)],
        )
        .u32();
    if light == 0 {
        add_book_light(e, distance);
    }
    let scene = e.call(OBJECT_BY_INDEX, &args![1u32]).u32();
    let geometry = e.mem.u32(book(e) + BOOK_GEOMETRY);
    e.vcall(scene, 0xdc, &args![geometry, 1u32]);
    e.with_stack(0x4b0, |e, frame| {
        let ebp = frame.addr() + 0x4b0;
        for step in BOOK_BUILD {
            build_step(e, base, ebp, step);
        }
    });
    special_book_menu_load_button_and_text_images(e, this);
    for page in 0..9u32 {
        let texts = base + (page << 5) + PAGE_TEXT_ITEMS;
        run_items(e, |_| texts, 0);
        let buttons = base + (page << 5) + PAGES;
        run_items(e, |_| buttons, 4);
    }
}

/// `0043d410` of (0.0, 0, 0) into a stack translation, set on the object `of` answers
/// (`00a59c60`).
fn place_at_origin(e: &mut Engine, of: impl FnOnce(&mut Engine) -> u32) {
    e.with_stack(0xc, |e, translation| {
        e.call(
            TRANSLATION_0043D410,
            &args![translation, 0.0f32.to_bits(), 0u32, 0u32],
        );
        let object = of(e);
        e.call(SET_TRANSLATE_00A59C60, &args![object, translation]);
    });
}

/// Starts `sequence` on `controller` from the beginning: `DeactivateAll(0.0)`,
/// `0047aab0(sequence, 0, 1, 1.0, 0.0, 0)`, `0047aa40(1)` and the sequence's `004954c0`.
fn start_sequence(e: &mut Engine, controller: u32, sequence: u32) {
    e.call(DEACTIVATE_ALL, &args![controller, 0.0f32.to_bits()]);
    e.call(
        ACTIVATE_SEQUENCE,
        &args![
            controller,
            sequence,
            0u32,
            1u32,
            1.0f32.to_bits(),
            0.0f32.to_bits(),
            0u32
        ],
    );
    e.call(CONTROLLER_0047AA40, &args![controller, 1u32]);
    e.call(SEQUENCE_004954C0, &args![sequence]);
}

/// `InitBook`'s point light (see there).
fn add_book_light(e: &mut Engine, distance: u32) {
    let memory = e.call(ALLOCATE_00AA13E0, &args![0xfcu32]).u32();
    let light = if memory != 0 {
        e.call(LIGHT_CONSTRUCT, &args![memory]).u32()
    } else {
        0
    };
    let colour = x87_quiet_bits(e.mem.u32(LIGHT_COLOUR));
    let zero = 0.0f32.to_bits();
    for (setter, point) in [
        (LIGHT_AMBIENT_00440460, [zero, zero, zero]),
        (LIGHT_DIFFUSE_004BC2E0, [colour, colour, colour]),
        (
            LIGHT_POSITION_0050DD50,
            [x87_quiet_bits(distance), zero, zero],
        ),
    ] {
        e.with_stack(0xc, |e, local| {
            let made = e
                .call(POINT_00416870, &args![local, point[0], point[1], point[2]])
                .u32();
            e.call(setter, &args![light, made]);
        });
    }
    e.call(LIGHT_DIMMER_0050DD20, &args![light, 1.0f32.to_bits()]);
    place_at_origin(e, |_| light);
    let scene = e.call(OBJECT_BY_INDEX, &args![1u32]).u32();
    e.vcall(scene, 0xdc, &args![light, 1u32]);
    let scene = e.call(OBJECT_BY_INDEX, &args![1u32]).u32();
    e.call(ADD_LIGHT, &args![scene, light, 0u32]);
}

/// One [`Build`] step of `InitBook` on its frame (`ebp`).
fn build_step(e: &mut Engine, this: u32, ebp: u32, step: Build) {
    let make = |e: &mut Engine, size: u32, construct: &dyn Fn(&mut Engine, u32) -> u32| {
        let memory = e.call(ALLOCATE, &args![size]).u32();
        if memory != 0 {
            construct(e, memory)
        } else {
            0
        }
    };
    match step {
        Build::Button(click, refresh, name, slot) => {
            let made = make(e, 0x14, &|e, memory| {
                fn_007ca080(e, Ptr::new(memory), click, refresh, name).addr()
            });
            e.mem.set_u32(local(ebp, slot), made);
        }
        Build::Text(callback, first, second, value, slot) => {
            let made = make(e, 0x18, &|e, memory| {
                fn_007ca2c0(e, Ptr::new(memory), callback, first, second, value).addr()
            });
            e.mem.set_u32(local(ebp, slot), made);
        }
        Build::Value(click, refresh, name, value, slot) => {
            let made = make(e, 0x28, &|e, memory| {
                let items = [e.mem.u32(local(ebp, -0x14)), e.mem.u32(local(ebp, -0x4c))];
                special_book_menu_actor_value_button_actor_value_button(
                    e, memory, click, refresh, name, value, 2, &items,
                )
            });
            e.mem.set_u32(local(ebp, slot), made);
        }
        Build::Copy(target, source) => {
            let value = e.mem.u32(local(ebp, source));
            e.mem.set_u32(local(ebp, target), value);
        }
        Build::Add(offset, slot) => {
            e.call(
                ARRAY_ADD,
                &args![this.wrapping_add(offset), local(ebp, slot)],
            );
        }
    }
}

/// The camera's setting objects (`01203144` the far plane's divisor, `01203150` the field
/// of view) and constants: 0.75 (`01016264`, a `float`), degrees to radians and 0.15
/// (`01023128`, `01070c80`, `double`s); `"Surgery3DCamera"`.
const CAMERA_FAR_SETTING: u32 = 0x0120_3144;
const CAMERA_FOV_SETTING: u32 = 0x0120_3150;
const FRUSTUM_SCALE: u32 = 0x0101_6264;
const DEGREES_TO_RADIANS_DOUBLE: u32 = 0x0102_3128;
const FOV_FACTOR_DOUBLE: u32 = 0x0107_0c80;
const CAMERA_NAME_TEXT: u32 = 0x0107_0c70;
/// The viewport's four words (`011ad840`), the renderer's width and height (`004dc200`,
/// `004dc1f0`), the camera's constructor (`00a712f0`), the viewport constructor
/// (`00414430`, four `float`s) and setter (`00712e60`), `NiFrustum::NiFrustum` (`00a71b70`,
/// one word), the far plane (`00507700`, a `float`), the half-angle helper (`0057dd70`,
/// `cdecl`, a `float`, `ST0`), `NiCamera::SetViewFrustum` (`00a6faf0`),
/// `NiCamera::LookAtWorldPoint` (`00a701b0`, two words), `00733fd0` (a `float`) and the
/// name setter (`00a5b950`).
const VIEWPORT: u32 = 0x011a_d840;
const RENDERER_WIDTH: u32 = 0x004d_c200;
const RENDERER_HEIGHT: u32 = 0x004d_c1f0;
const CAMERA_CONSTRUCT: u32 = 0x00a7_12f0;
const VIEWPORT_CONSTRUCT: u32 = 0x0041_4430;
const CAMERA_SET_VIEWPORT: u32 = 0x0071_2e60;
const FRUSTUM_CONSTRUCT: u32 = 0x00a7_1b70;
const CAMERA_FAR_00507700: u32 = 0x0050_7700;
const HALF_ANGLE_0057DD70: u32 = 0x0057_dd70;
const CAMERA_SET_VIEW_FRUSTUM: u32 = 0x00a6_faf0;
const CAMERA_LOOK_AT_WORLD_POINT: u32 = 0x00a7_01b0;
const CAMERA_00733FD0: u32 = 0x0073_3fd0;
const SET_NAME_00A5B950: u32 = 0x00a5_b950;

// Translated from 007c99c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007c99c0 - the C++ exception frame (FS:[0] chain, handler 00f0de9a, unwind states 0 and 1 for the new camera and its name) is not translated
/// `SPECIALBookMenu::InitCamera` (engine map; `thiscall`, the menu not read). Makes a
/// camera (`00aa13e0(0x114)`, `00a712f0`; 0 when the allocation fails), sets its viewport
/// (0, 1, 1, 0, or the four words at `011ad840` with the flag at `011f9426`), its far plane
/// (the scene's vtable +0x100 over the setting at `01203144`), its frustum (left and right
/// `∓t * 0.75`, top and bottom `±t * aspect * 0.75` with `t` = `0057dd70` of the field of
/// view setting times `01023128` times `01070c80`, near and far from the same stack words
/// as the far plane; the aspect is the width (`011f9434` with the flag, else `004dc200`)
/// over `004dc1f0`), its rotation (0, 0, 1), translation (origin) and look-at (origin, up
/// 0, 1, 0), `00733fd0(1.0)` and the name `"Surgery3DCamera"`. Returns the camera.
pub fn special_book_menu_init_camera(e: &mut Engine, _this: Ptr) -> u32 {
    let memory = e.call(ALLOCATE_00AA13E0, &args![0x114u32]).u32();
    let camera = if memory != 0 {
        e.call(CAMERA_CONSTRUCT, &args![memory]).u32()
    } else {
        0
    };
    e.with_stack(0x78, |e, frame| {
        let ebp = frame.addr() + 0x78;
        let viewport = local(ebp, -0x28);
        let frustum = local(ebp, -0x4c);
        let one = 1.0f32.to_bits();
        let zero = 0.0f32.to_bits();
        e.call(VIEWPORT_CONSTRUCT, &args![viewport, zero, one, one, zero]);
        if e.mem.u8(VIEWPORT_FLAG) != 0 {
            for word in 0..4u32 {
                let value = e.mem.u32(VIEWPORT + 4 * word);
                e.mem.set_u32(viewport + 4 * word, value);
            }
        }
        e.call(CAMERA_SET_VIEWPORT, &args![camera, viewport]);
        e.call(FRUSTUM_CONSTRUCT, &args![frustum, 0u32]);
        let scene = e.call(SCENE_OBJECT, &args![]).u32();
        let far = x87_store32(e.vcall(scene, 0x100, &args![]).f64());
        e.mem.set_u32(local(ebp, -0x38), far);
        let setting = e.call(SETTING_VALUE, &args![CAMERA_FAR_SETTING]).u32();
        let near = x87_quiet_bits(e.mem.u32(setting));
        e.mem.set_u32(local(ebp, -0x3c), near);
        let quotient = x87_store32(x87_div(
            x87_load32(e.mem.u32(local(ebp, -0x38))),
            x87_m32(e.mem.u32(local(ebp, -0x3c))),
        ));
        e.call(
            CAMERA_FAR_00507700,
            &args![camera, x87_quiet_bits(quotient)],
        );
        let width = if e.mem.u8(VIEWPORT_FLAG) != 0 {
            e.mem.u32(VIEWPORT_WIDTH)
        } else {
            e.call(RENDERER_WIDTH, &args![]).u32()
        };
        let height = e.call(RENDERER_HEIGHT, &args![]).u32();
        // FILD width; FSTP double; FILD height; FDIVR the double.
        let aspect = x87_store32(x87_div(f64::from(width as i32), f64::from(height as i32)));
        let scale = x87_quiet_bits(e.mem.u32(FRUSTUM_SCALE));
        let setting = e.call(SETTING_VALUE, &args![CAMERA_FOV_SETTING]).u32();
        let radians: f64 = e.global(DEGREES_TO_RADIANS_DOUBLE);
        let factor: f64 = e.global(FOV_FACTOR_DOUBLE);
        let fov = x87_store32(x87_mul(
            x87_mul(x87_load32(e.mem.u32(setting)), radians),
            factor,
        ));
        let half = |e: &mut Engine| {
            e.call(HALF_ANGLE_0057DD70, &args![x87_quiet_bits(fov)])
                .f64()
        };
        let value = half(e);
        let left = x87_store32(x87_mul(-value, x87_m32(scale)));
        e.mem.set_u32(local(ebp, -0x4c), left);
        let value = half(e);
        let right = x87_store32(x87_mul(value, x87_m32(scale)));
        e.mem.set_u32(local(ebp, -0x48), right);
        let value = half(e);
        let bottom = x87_store32(x87_mul(x87_mul(-value, x87_m32(aspect)), x87_m32(scale)));
        e.mem.set_u32(local(ebp, -0x40), bottom);
        let value = half(e);
        let top = x87_store32(x87_mul(x87_mul(value, x87_m32(aspect)), x87_m32(scale)));
        e.mem.set_u32(local(ebp, -0x44), top);
        e.mem.set_u8(local(ebp, -0x34), 0);
        e.call(CAMERA_SET_VIEW_FRUSTUM, &args![camera, frustum]);
        e.call(SET_ROTATION_004BC1F0, &args![camera, zero, zero, one]);
        let translation = local(ebp, -0x18);
        e.call(TRANSLATION_0043D410, &args![translation, zero, 0u32, 0u32]);
        e.call(SET_TRANSLATE_00A59C60, &args![camera, translation]);
        let up = e
            .call(POINT_00416870, &args![local(ebp, -0x68), zero, one, zero])
            .u32();
        let target = e
            .call(POINT_00416870, &args![local(ebp, -0x74), zero, zero, zero])
            .u32();
        e.call(CAMERA_LOOK_AT_WORLD_POINT, &args![camera, target, up]);
        e.call(CAMERA_00733FD0, &args![camera, one]);
    });
    with_fixed_string(e, CAMERA_NAME_TEXT, |e, name| {
        e.call(SET_NAME_00A5B950, &args![camera, name]);
    });
    camera
}

// Translated from 007c9dc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007c9dc0 - the C++ exception frame (FS:[0] chain, handler 00f0e4b8, unwind states 0 to 4 for the sound handles and the sequence name) is not translated
/// `SPECIALBookMenu::TurnPage` (engine map; `RET 4`: a byte, forward; the answer in `AL`).
/// Stores the direction (`011a0854`); a page past the last (9) or before the first answers
/// false. Otherwise the page moves, `"OBJBookSpecialPageTurn"` plays and, when the geometry
/// has its controller (`011f36ac`), the page's turn animation (unless turning back onto
/// page 0) is started (see `start_sequence`). Answers true.
pub fn special_book_menu_turn_page(e: &mut Engine, this: Ptr, forward: u8) -> bool {
    let base = this.addr();
    let page = e.mem.i32(base + STATE);
    if forward != 0 {
        e.mem.set_u8(TURNED_FORWARD, 1);
        if page >= 9 {
            return false;
        }
        e.mem.set_u32(base + STATE, page.wrapping_add(1) as u32);
    } else {
        e.mem.set_u8(TURNED_FORWARD, 0);
        if page < 1 {
            return false;
        }
        e.mem.set_u32(base + STATE, page.wrapping_sub(1) as u32);
    }
    play_sound_held(e, PAGE_TURN_SOUND);
    let geometry = e.mem.u32(book(e) + BOOK_GEOMETRY);
    let controller = e
        .call(GET_CONTROLLER, &args![geometry, CONTROLLER_RTTI_011F36AC])
        .u32();
    if controller != 0 {
        let mut sequence = 0u32;
        if e.mem.u8(TURNED_FORWARD) == 0 || e.mem.i32(base + STATE) > 0 {
            sequence = turn_sequence(e, base, controller);
        }
        if sequence != 0 {
            start_sequence(e, controller, sequence);
        }
    }
    true
}

// Translated from 007ca030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): the value the current page edits: -1 off pages
/// 1 to 8; on the index page (8) with `004b71d0` `eXBoxIndexState + 4`, else the page + 4.
pub fn fn_007ca030(e: &mut Engine, this: Ptr) -> i32 {
    let base = this.addr();
    let page = e.mem.i32(base + STATE);
    if !(1..9).contains(&page) {
        return -1;
    }
    if e.call(INTERFACE_TEST_004B71D0, &args![]).bool() && e.mem.i32(base + STATE) == 8 {
        e.mem.i32(base + INDEX_STATE).wrapping_add(4)
    } else {
        e.mem.i32(base + STATE).wrapping_add(4)
    }
}

// Translated from 007ca080 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007ca080 - the C++ exception frame (FS:[0] chain, handler 00f0e4fb, unwind states 0 to 2 for the node pointer and the names) is not translated
/// The book button constructor (unnamed in the engine map; `RET 0xC`): the node pointer
/// (+8, `00633c90` with 0), the click (+0) and refresh (+4) callbacks, the node named `name`
/// in the book geometry into the pointer (`0066b0d0`), and that node's texturing property
/// (+0xC, see `texturing_property`). Returns `this`.
pub fn fn_007ca080(e: &mut Engine, this: Ptr, click: u32, refresh: u32, name: u32) -> Ptr {
    let base = this.addr();
    e.call(POINTER_CONSTRUCT, &args![base + 8, 0u32]);
    e.mem.set_u32(base, click);
    e.mem.set_u32(base + 4, refresh);
    let node = book_child(e, name);
    e.call(SMART_POINTER_SET, &args![base + 8, node]);
    let node = book_child(e, name);
    let property = texturing_property(e, node);
    e.mem.set_u32(base + 0xc, property);
    this
}

// Translated from 007ca200 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007ca200 - the C++ exception frame (FS:[0] chain, handler 00f0e533, unwind states 0 and 1 for the button base and the array) is not translated
/// `SPECIALBookMenu::ActorValueButton::ActorValueButton` (engine map; `cdecl` with extra
/// words: `this`, click, refresh, name, the value, a count and that many item words). The
/// button base (`007ca080`), the item array (+0x14, `00723f00`), the value (+0x24), and each
/// item added (`AddUninitialized` with the address of its word, as the code passes its
/// argument slots). Returns `this`.
#[allow(clippy::too_many_arguments)]
pub fn special_book_menu_actor_value_button_actor_value_button(
    e: &mut Engine,
    this: u32,
    click: u32,
    refresh: u32,
    name: u32,
    value: u32,
    count: i32,
    items: &[u32],
) -> u32 {
    fn_007ca080(e, Ptr::new(this), click, refresh, name);
    e.call(ARRAY_CONSTRUCT, &args![this + 0x14]);
    e.mem.set_u32(this + 0x24, value);
    let size = (items.len() as u32) * 4;
    e.with_stack(size.max(4), |e, slots| {
        for (index, word) in items.iter().enumerate() {
            e.mem.set_u32(slots.addr() + 4 * index as u32, *word);
        }
        let mut index = 0i32;
        while index < count {
            // The code reads the argument slots past `count` without a bound.
            let slot = slots.addr().wrapping_add((index as u32).wrapping_mul(4));
            e.call(ARRAY_ADD, &args![this + 0x14, slot]);
            index += 1;
        }
    });
    this
}

/// [`special_book_menu_actor_value_button_actor_value_button`] in the uniform form: the
/// words after the count are the items.
fn actor_value_button_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let this = words[0];
    let made = special_book_menu_actor_value_button_actor_value_button(
        e,
        this,
        words[1],
        words[2],
        words[3],
        words[4],
        words[5] as i32,
        &words[6..],
    );
    made.into_ret()
}

// Translated from 007ca2c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007ca2c0 - the C++ exception frame (FS:[0] chain, handler 00f0e560, unwind states 0 and 1 for the names) is not translated
/// The book text item constructor (unnamed in the engine map; `RET 0x10`): the callback
/// (+0), the value (+0x14), the node named `first` (+0xC) and its texturing property (+4);
/// with a `second` name that node (+0x10) and its property (+8). Returns `this`.
pub fn fn_007ca2c0(
    e: &mut Engine,
    this: Ptr,
    callback: u32,
    first: u32,
    second: u32,
    value: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, callback);
    e.mem.set_u32(base + 0x14, value);
    let node = book_child(e, first);
    e.mem.set_u32(base + 0xc, node);
    let node = e.mem.u32(base + 0xc);
    let property = texturing_property(e, node);
    e.mem.set_u32(base + 4, property);
    if second != 0 {
        let node = book_child(e, second);
        e.mem.set_u32(base + 0x10, node);
        let node = e.mem.u32(base + 0x10);
        let property = texturing_property(e, node);
        e.mem.set_u32(base + 8, property);
    }
    this
}

// Translated from 007ca4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, a text item; the point-value refresh): the item's
/// property (+4) shows the digit texture of the player's value (item +0x14).
pub fn fn_007ca4d0(e: &mut Engine, item: u32) {
    let index = e.mem.u32(item + 0x14);
    let value = player_value(e, index) as u32;
    let slot = book(e)
        .wrapping_add(value.wrapping_mul(4))
        .wrapping_add(DIGIT_TEXTURES);
    let digit = texture(e, slot);
    let property = e.mem.u32(item + 4);
    show_texture(e, property, digit);
}

// Translated from 007ca530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, a text item; the message refresh): with more than
/// one point left the item's property gets the texture at +0x1b0 (vtable +0xFC with 0);
/// with one left the one at +0x1b4, with none the one at +0x1b8 (vtable +0xF0).
pub fn fn_007ca530(e: &mut Engine, item: u32) {
    let used = special_book_menu_get_total_points_used(e, Ptr::new(book(e)));
    let left = e.mem.u32(book(e) + AVAILABLE_POINTS).wrapping_sub(used) as i32;
    let property = e.mem.u32(item + 4);
    if left > 1 {
        let image = texture(e, book(e) + 0x1b0);
        e.vcall(property, 0xfc, &args![0u32, image]);
    } else if left == 1 {
        let image = texture(e, book(e) + 0x1b4);
        show_texture(e, property, image);
    } else if left == 0 {
        let image = texture(e, book(e) + 0x1b8);
        show_texture(e, property, image);
    }
}

// Translated from 007ca5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, a text item; the points-left refresh). With both
/// properties (+4, +8): no points left hides both nodes (+0xC, +0x10; `00450f90(1)`);
/// otherwise the tens digit (signed division by 10) is shown on the first when above 0
/// (else its node hidden) and the ones digit on the second.
pub fn fn_007ca5f0(e: &mut Engine, item: u32) {
    if e.mem.u32(item + 4) == 0 || e.mem.u32(item + 8) == 0 {
        return;
    }
    let used = special_book_menu_get_total_points_used(e, Ptr::new(book(e)));
    let left = e.mem.u32(book(e) + AVAILABLE_POINTS).wrapping_sub(used) as i32;
    if left == 0 {
        let node = e.mem.u32(item + 0xc);
        set_hidden(e, node, 1);
        let node = e.mem.u32(item + 0x10);
        set_hidden(e, node, 1);
        return;
    }
    let tens = left.wrapping_div(10);
    let ones = left.wrapping_rem(10);
    if tens > 0 {
        let node = e.mem.u32(item + 0xc);
        set_hidden(e, node, 0);
        let slot = book(e)
            .wrapping_add((tens as u32).wrapping_mul(4))
            .wrapping_add(DIGIT_TEXTURES);
        let digit = texture(e, slot);
        let property = e.mem.u32(item + 4);
        show_texture(e, property, digit);
    } else {
        let node = e.mem.u32(item + 0xc);
        set_hidden(e, node, 1);
    }
    let node = e.mem.u32(item + 0x10);
    set_hidden(e, node, 0);
    let slot = book(e)
        .wrapping_add((ones as u32).wrapping_mul(4))
        .wrapping_add(DIGIT_TEXTURES);
    let digit = texture(e, slot);
    let property = e.mem.u32(item + 8);
    show_texture(e, property, digit);
}

/// The value buttons' shared tail: on the index page (8) with `004b71d0`, a button for a
/// value other than the selected one is hidden.
fn hide_unselected(e: &mut Engine, item: u32, index: u32) {
    if e.call(INTERFACE_TEST_004B71D0, &args![]).bool()
        && e.mem.i32(book(e) + STATE) == 8
        && index != e.mem.u32(book(e) + INDEX_STATE).wrapping_add(4)
    {
        let node = e.call(POINTER_GET, &args![item + 8]).u32();
        set_hidden(e, node, 1);
    }
}

// Translated from 007ca700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, a value button; the decrease refresh): the button's
/// node (+8) shows while the value (+0x24) is above 1, else hides; see `hide_unselected`.
pub fn fn_007ca700(e: &mut Engine, item: u32) {
    let index = e.mem.u32(item + 0x24);
    let hidden = u32::from(player_value(e, index) <= 1);
    let node = e.call(POINTER_GET, &args![item + 8]).u32();
    set_hidden(e, node, hidden);
    hide_unselected(e, item, index);
}

// Translated from 007ca7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, a value button; the increase refresh): the button's
/// node shows while the value is below 10 and (with a book) points are left, else hides;
/// see `hide_unselected`.
pub fn fn_007ca7a0(e: &mut Engine, item: u32) {
    let index = e.mem.u32(item + 0x24);
    let shown = player_value(e, index) < 10
        && (book(e) == 0 || {
            let used = special_book_menu_get_total_points_used(e, Ptr::new(book(e)));
            (used as i32) < e.mem.i32(book(e) + AVAILABLE_POINTS)
        });
    let node = e.call(POINTER_GET, &args![item + 8]).u32();
    set_hidden(e, node, u32::from(!shown));
    hide_unselected(e, item, index);
}

// Translated from 007ca860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, a button; the next-page refresh): its property
/// (+0xC) shows the texture at +0x1c0.
pub fn fn_007ca860(e: &mut Engine, item: u32) {
    let image = texture(e, book(e) + 0x1c0);
    let property = e.mem.u32(item + 0xc);
    show_texture(e, property, image);
}

// Translated from 007ca8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, a button; the previous-page refresh): its property
/// shows the texture at +0x1c4.
pub fn fn_007ca8a0(e: &mut Engine, item: u32) {
    let image = texture(e, book(e) + 0x1c4);
    let property = e.mem.u32(item + 0xc);
    show_texture(e, property, image);
}

// Translated from 007ca8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, a button; the all-done refresh): its property shows
/// the texture at +0x1d0 while points are left, else the one at +0x1cc.
pub fn fn_007ca8e0(e: &mut Engine, item: u32) {
    let used = special_book_menu_get_total_points_used(e, Ptr::new(book(e)));
    let offset = if (used as i32) < e.mem.i32(book(e) + AVAILABLE_POINTS) {
        0x1d0
    } else {
        0x1cc
    };
    let image = texture(e, book(e) + offset);
    let property = e.mem.u32(item + 0xc);
    show_texture(e, property, image);
}

// Translated from 007ca960 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007ca960 - the C++ exception frame (FS:[0] chain, handler 00f0c5e0, unwind states 0 and 1 for the sound handles) is not translated
/// `SPECIALBookMenu::DecreaseActorValue` (engine map; `cdecl`, a value button). With a book
/// and a value (+0x24, not -1): above 1 `"OBJBookSpecialNumber"` plays; the base value
/// becomes `00647b70(value - 1, 1)`; the page is refreshed.
pub fn special_book_menu_decrease_actor_value(e: &mut Engine, item: u32) {
    if book(e) == 0 {
        return;
    }
    let index = e.mem.u32(item + 0x24);
    if index != 0xffff_ffff {
        let value = player_value(e, index);
        if value > 1 {
            play_sound_held(e, SPECIAL_NUMBER_SOUND);
        }
        let changed = e
            .call(CLAMP_LOW_00647B70, &args![value.wrapping_sub(1), 1u32])
            .u32();
        let player = e.mem.u32(PLAYER);
        e.call(SET_ACTOR_BASE_VALUE, &args![player, index, changed]);
    }
    let book_menu = Ptr::new(book(e));
    special_book_menu_refresh_page(e, book_menu);
}

// Translated from 007caa80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007caa80 - the C++ exception frame (FS:[0] chain, handler 00f0c5e0, unwind states 0 and 1 for the sound handles) is not translated
/// `SPECIALBookMenu::IncreaseActorValue` (engine map; `cdecl`, a value button). With a book
/// and a value: below 10 `"OBJBookSpecialNumber"` plays; while points are left the base
/// value becomes `004a8f20(value + 1, 10)`; the page is refreshed.
pub fn special_book_menu_increase_actor_value(e: &mut Engine, item: u32) {
    if book(e) == 0 {
        return;
    }
    let index = e.mem.u32(item + 0x24);
    if index != 0xffff_ffff {
        let value = player_value(e, index);
        if value < 10 {
            play_sound_held(e, SPECIAL_NUMBER_SOUND);
        }
        let used = special_book_menu_get_total_points_used(e, Ptr::new(book(e)));
        if (used as i32) < e.mem.i32(book(e) + AVAILABLE_POINTS) {
            let changed = e
                .call(CLAMP_HIGH_004A8F20, &args![value.wrapping_add(1), 10u32])
                .u32();
            let player = e.mem.u32(PLAYER);
            e.call(SET_ACTOR_BASE_VALUE, &args![player, index, changed]);
        }
    }
    let book_menu = Ptr::new(book(e));
    special_book_menu_refresh_page(e, book_menu);
}

/// The page buttons' turn: with a book whose last turn finished, `TurnPage(forward)` and
/// `RefreshPage`.
fn turn_from_button(e: &mut Engine, forward: u8) {
    if book(e) != 0 && e.mem.u8(book(e) + ANIM_FINISHED) != 0 {
        let book_menu = Ptr::new(book(e));
        special_book_menu_turn_page(e, book_menu, forward);
        let book_menu = Ptr::new(book(e));
        special_book_menu_refresh_page(e, book_menu);
    }
}

// Translated from 007cabb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments read; the next-page button's click): turns the
/// page forward (see `turn_from_button`).
pub fn fn_007cabb0(e: &mut Engine, _unused_1: u32) {
    turn_from_button(e, 1);
}

// Translated from 007cabf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments read; the previous-page button's click): turns
/// the page back (see `turn_from_button`).
pub fn fn_007cabf0(e: &mut Engine, _unused_1: u32) {
    turn_from_button(e, 0);
}

// Translated from 007cac30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007cac30 - the C++ exception frame (FS:[0] chain, handler 00f0e588, unwind state 0 for the sound handle) is not translated
/// Unnamed in the engine map (no arguments read; the all-done button's click): closes the
/// book (`Close`) when every point is spent, else plays `"UIActivateNothing"`.
pub fn fn_007cac30(e: &mut Engine, _unused_1: u32) {
    if book(e) != 0 {
        let used = special_book_menu_get_total_points_used(e, Ptr::new(book(e)));
        if used == e.mem.u32(book(e) + AVAILABLE_POINTS) {
            special_book_menu_close(e);
            return;
        }
    }
    play_sound(e, ACTIVATE_NOTHING_SOUND);
}

// Translated from 007cacd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SPECIALBookMenu::RefreshPage` (engine map; `thiscall`): on pages 0 to 8, every text item
/// of the page runs its callback (+0) and every button its refresh callback (+4).
pub fn special_book_menu_refresh_page(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.i32(base + STATE) >= 9 {
        return;
    }
    run_items(e, |e| page_array(e, base, true), 0);
    run_items(e, |e| page_array(e, base, false), 4);
}

/// `LoadButtonAndTextImages`' calls: `SwapPlatformLanguageTexturePath` (`004b7240`,
/// `cdecl`: from, to, size), `TES::CreateTextureImage` (`004568c0`, on the object at
/// `011dea10`: path, slot, 0, 0), `00709d40` (no arguments: a byte, whether the Xbox
/// images are used) and the bounded copy (`00406d30`, `cdecl`).
const SWAP_LANGUAGE_TEXTURE_PATH: u32 = 0x004b_7240;
const CREATE_TEXTURE_IMAGE: u32 = 0x0045_68c0;
const TES_OBJECT: u32 = 0x011d_ea10;
const XBOX_IMAGES_00709D40: u32 = 0x0070_9d40;
const COPY_00406D30: u32 = 0x0040_6d30;
const FORMAT_00406D00: u32 = 0x0040_6d00;
/// The paths: `"%s%s%i.dds"`, `"%s%s.dds"`, `"%s\%s\%s.dds"`, `"Textures\Terminals\"`,
/// `"Textures\Terminals\ENGLISH\"`, `"BBNumber"`, `"BBMessage0"`, `"XBOX"` and `"PC"`.
const NUMBERED_FORMAT: u32 = 0x0107_4c4c;
const IMAGE_FORMAT: u32 = 0x0107_4c08;
const PLATFORM_FORMAT: u32 = 0x0107_6c94;
const TERMINALS_PATH: u32 = 0x0107_6cc0;
const ENGLISH_TERMINALS_PATH: u32 = 0x0107_6ca4;
const NUMBER_IMAGE: u32 = 0x0107_4c6c;
const MESSAGE_IMAGE: u32 = 0x0107_4c40;
const XBOX_TEXT: u32 = 0x0101_fd1c;
const PC_TEXT: u32 = 0x0101_fd24;

/// How `LoadButtonAndTextImages` makes one image path.
#[derive(Clone, Copy)]
enum ImagePath {
    /// `"%s%s.dds"` of the terminals path and the name, swapped to the language.
    Language(u32),
    /// `"%s\%s\%s.dds"` of the terminals path, the platform and the name.
    Platform(u32),
}

/// The button images after the digits and messages, in the exe's order: the name and the
/// slot (book offset).
const BUTTON_IMAGES: [(ImagePath, u32); 10] = [
    (ImagePath::Platform(0x0107_4c38), 0x1c0),
    (ImagePath::Language(0x0107_4c24), 0x1bc),
    (ImagePath::Platform(0x0107_4c1c), 0x1c4),
    (ImagePath::Language(0x0107_4c14), 0x1c8),
    (ImagePath::Platform(0x0107_4c00), 0x1cc),
    (ImagePath::Platform(0x0107_4bf8), 0x1d0),
    (ImagePath::Language(0x0107_4be8), 0x1d4),
    (ImagePath::Language(0x0107_4bd8), 0x1d8),
    (ImagePath::Language(0x0107_4bc8), 0x1dc),
    (ImagePath::Language(0x0107_4bb8), 0x1e0),
];

// Translated from 007cadd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007cadd0 - the /GS stack cookie check (00ec408c) is not translated
/// `SPECIALBookMenu::LoadButtonAndTextImages` (engine map; `thiscall`). Creates
/// (`TES::CreateTextureImage`) the eleven digit images `BBNumber0` to `BBNumber10` into
/// +0x180, the messages `BBMessage01` to `04` (from the English path) into +0x1ac, and the
/// button images (`BUTTON_IMAGES`; the platform folder `"XBOX"` or `"PC"` by `00709d40`).
/// Paths with `"%s%s%i.dds"` or `"%s%s.dds"` are swapped to the language
/// (`SwapPlatformLanguageTexturePath`) first.
pub fn special_book_menu_load_button_and_text_images(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(0x22c, |e, frame| {
        let ebp = frame.addr() + 0x22c;
        let name = local(ebp, -0x218);
        let path = local(ebp, -0x110);
        let platform = local(ebp, -0x220);
        let create = |e: &mut Engine, slot: u32| {
            let tes: u32 = e.global(TES_OBJECT);
            e.call(CREATE_TEXTURE_IMAGE, &args![tes, path, slot, 0u32, 0u32]);
        };
        let swap = |e: &mut Engine| {
            e.call(SWAP_LANGUAGE_TEXTURE_PATH, &args![name, path, 0x104u32]);
        };
        for index in 0..0xbu32 {
            e.call(
                FORMAT_00406D00,
                &args![
                    name,
                    0x104u32,
                    NUMBERED_FORMAT,
                    TERMINALS_PATH,
                    NUMBER_IMAGE,
                    index
                ],
            );
            swap(e);
            create(e, base + DIGIT_TEXTURES + 4 * index);
        }
        for index in 1..5u32 {
            e.call(
                FORMAT_00406D00,
                &args![
                    name,
                    0x104u32,
                    NUMBERED_FORMAT,
                    ENGLISH_TERMINALS_PATH,
                    MESSAGE_IMAGE,
                    index
                ],
            );
            swap(e);
            create(e, base + 0x1a8 + 4 * index);
        }
        let folder = if e.call(XBOX_IMAGES_00709D40, &args![]).bool() {
            XBOX_TEXT
        } else {
            PC_TEXT
        };
        e.call(COPY_00406D30, &args![platform, 5u32, folder]);
        for (image, slot) in BUTTON_IMAGES {
            match image {
                ImagePath::Platform(file) => {
                    e.call(
                        FORMAT_00406D00,
                        &args![
                            path,
                            0x104u32,
                            PLATFORM_FORMAT,
                            TERMINALS_PATH,
                            platform,
                            file
                        ],
                    );
                }
                ImagePath::Language(file) => {
                    e.call(
                        FORMAT_00406D00,
                        &args![name, 0x104u32, IMAGE_FORMAT, TERMINALS_PATH, file],
                    );
                    swap(e);
                }
            }
            create(e, base + slot);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007c_9ca0, fn_007c9ca0()),
        entry!(
            0x0079_4d80,
            special_book_menu_get_total_points_used(Ptr) -> u32
        ),
        entry!(0x007c_6120, special_book_menu_close()),
        entry!(0x007c_5fb0, special_book_menu_create()),
        entry!(0x007c_5fc0, special_book_menu_create_ov2(u32)),
        entry!(0x007c_6110, fn_007c6110() -> u32),
        entry!(0x007c_6200, special_book_menu_do_idle(Ptr)),
        entry!(
            0x007c_6440,
            special_book_menu_do_gamepad(Ptr, u32, u32) -> bool
        ),
        entry!(0x007c_6550, special_book_menu_do_click(Ptr, u32, u32)),
        entry!(0x007c_6c50, special_book_menu_do_key(Ptr, u32) -> bool),
        entry!(0x007c_6ce0, special_book_menu_init_book(Ptr)),
        entry!(0x007c_99c0, special_book_menu_init_camera(Ptr) -> u32),
        entry!(0x007c_9dc0, special_book_menu_turn_page(Ptr, u8) -> bool),
        entry!(0x007c_a030, fn_007ca030(Ptr) -> i32),
        entry!(0x007c_a080, fn_007ca080(Ptr, u32, u32, u32) -> Ptr),
        (0x007c_a200, actor_value_button_entry as AbiFn),
        entry!(
            0x007c_a2c0,
            fn_007ca2c0(Ptr, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x007c_a4d0, fn_007ca4d0(u32)),
        entry!(0x007c_a530, fn_007ca530(u32)),
        entry!(0x007c_a5f0, fn_007ca5f0(u32)),
        entry!(0x007c_a700, fn_007ca700(u32)),
        entry!(0x007c_a7a0, fn_007ca7a0(u32)),
        entry!(0x007c_a860, fn_007ca860(u32)),
        entry!(0x007c_a8a0, fn_007ca8a0(u32)),
        entry!(0x007c_a8e0, fn_007ca8e0(u32)),
        entry!(0x007c_a960, special_book_menu_decrease_actor_value(u32)),
        entry!(0x007c_aa80, special_book_menu_increase_actor_value(u32)),
        entry!(0x007c_abb0, fn_007cabb0(u32)),
        entry!(0x007c_abf0, fn_007cabf0(u32)),
        entry!(0x007c_ac30, fn_007cac30(u32)),
        entry!(0x007c_acd0, special_book_menu_refresh_page(Ptr)),
        entry!(
            0x007c_add0,
            special_book_menu_load_button_and_text_images(Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_the_seven_values() {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        let player = e.mem.alloc(0xa8);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x1c, 0x701c);
        e.register(0x701c, |_, a| (a[1] * 10).into_ret());
        e.mem.set_u32(player + 0xa4, vtable);
        e.mem.set_u32(PLAYER, player);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0079_4d80, &[0]).u32(), 560);
        let log = e.call_log.take().unwrap();
        let indices: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == 0x701c)
            .map(|(_, w)| {
                assert_eq!(w[0], player + 0xa4);
                w[1]
            })
            .collect();
        assert_eq!(indices, vec![5, 6, 7, 8, 9, 10, 11]);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    fn close_engine(open: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_a000, 0x1000);
        e.register(MENU_CLASS, |_, _| 0x424u32.into_ret());
        e.register_double(GET_MENU_BY_CLASS, move |_, a| {
            assert_eq!(a[0], 0x424);
            open.into_ret()
        });
        e.register(OBJECT_BY_INDEX, |_, a| (0x7000 + a[0]).into_ret());
        for callee in [
            MODEL_LOADER_RELEASE,
            OBJECT_CLEAR_004DEF90,
            OBJECT_RELEASE_00B5D180,
            SMART_POINTER_SET,
            ARRAY_CLEAR,
            SET_TRAIT_FROM_INT,
            MENU_START_FADE_OUT,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.set_global(MODEL_LOADER, 0x0c00_0000u32);
        e
    }

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn close_releases_the_book_and_fades_out() {
        let mut e = close_engine(0x5000);
        let book = e.mem.alloc(0x1e4);
        e.mem.set_u32(book + 0x5c, 0x1234);
        e.set_global(SPECIAL_BOOK_MENU, book);
        e.call_log = Some(vec![]);
        e.call(0x007c_6120, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MODEL_LOADER_RELEASE),
            vec![vec![0x0c00_0000, BOOK_MODEL_PATH]]
        );
        assert_eq!(calls_to(&log, OBJECT_CLEAR_004DEF90), vec![vec![0x7001]]);
        assert_eq!(calls_to(&log, OBJECT_RELEASE_00B5D180), vec![vec![0x7001]]);
        assert_eq!(e.mem.u32(book + 0x5c), 0);
        assert_eq!(
            calls_to(&log, SMART_POINTER_SET),
            vec![vec![POINTER_011DAA9C, 0]]
        );
        let clears = calls_to(&log, ARRAY_CLEAR);
        let mut want = Vec::new();
        for page in 0..9u32 {
            want.push(vec![book + 0x60 + 0x20 * page, 1]);
            want.push(vec![book + 0x70 + 0x20 * page, 1]);
        }
        assert_eq!(clears, want);
        assert_eq!(
            calls_to(&log, SET_TRAIT_FROM_INT),
            vec![vec![0x5000, 0x1772, 1]]
        );
        assert_eq!(calls_to(&log, MENU_START_FADE_OUT), vec![vec![book]]);
    }

    #[test]
    fn close_without_the_open_menu_does_nothing() {
        let mut e = close_engine(0);
        e.call_log = Some(vec![]);
        e.call(0x007c_6120, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 3);
        assert!(calls_to(&log, MODEL_LOADER_RELEASE).is_empty());
    }

    #[test]
    fn close_without_the_book_global_skips_the_fade() {
        // The book global is 0: the clears run on the low addresses, no fade.
        let mut e = close_engine(0x5000);
        e.map(0, 0x1000);
        e.call_log = Some(vec![]);
        e.call(0x007c_6120, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ARRAY_CLEAR).len(), 18);
        assert!(calls_to(&log, SET_TRAIT_FROM_INT).is_empty());
        assert!(calls_to(&log, MENU_START_FADE_OUT).is_empty());
    }
}

#[cfg(test)]
mod la1_render_tests {
    use super::*;

    fn render_engine(camera: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x0103_0000, 0x1000);
        e.map(0x011d_a000, 0x1000);
        e.set_global(CAMERA_VALUE_01030FF4, 0.15f32);
        e.set_global(CAMERA_POINTER, camera);
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(RENDERER_OBJECT, |_, _| 0x0c00_0000u32.into_ret());
        e.register(SCENE_OBJECT, |_, _| 0x0c10_0000u32.into_ret());
        e.register(WORD_AT_B4, |_, _| 0x0c20_0000u32.into_ret());
        e.register(INTERFACE_MANAGER, |_, _| 0x0c30_0000u32.into_ret());
        e.register(INTERFACE_OBJECT_007FD640, |_, _| 0x0c40_0000u32.into_ret());
        e.register(POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(OBJECT_BY_INDEX, |_, a| (0x7000 + a[0]).into_ret());
        for callee in [
            CLEAR_BUFFER,
            PUSH_CULL_MODE,
            POP_CULL_MODE,
            CAMERA_SETUP_00709D50,
            CULLER_SET_004A0FD0,
            ACCUMULATE_SCENE,
            RENDER_00B6C0D0,
            POINTER_DESTRUCT,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn render_runs_the_scene_through_the_culler() {
        let camera = 0x0d00_0000;
        let mut e = render_engine(camera);
        e.set_global(SPECIAL_BOOK_MENU, 0x0e00_0000u32);
        e.call_log = Some(vec![]);
        e.call(0x007c_9ca0, &args![]);
        let log = e.call_log.take().unwrap();
        let held = log.iter().find(|(a, _)| *a == POINTER_CONSTRUCT).unwrap().1[0];
        let calls: Vec<(u32, Vec<u32>)> = log
            .into_iter()
            .skip(1)
            .filter(|(a, _)| *a != POINTER_GET)
            .collect();
        let culler = 0x0c20_0000;
        assert_eq!(
            calls,
            vec![
                (RENDERER_OBJECT, vec![]),
                (CLEAR_BUFFER, vec![0x0c00_0000, 0, 4]),
                (SCENE_OBJECT, vec![]),
                (WORD_AT_B4, vec![0x0c10_0000]),
                (PUSH_CULL_MODE, vec![culler, 1]),
                (CAMERA_SETUP_00709D50, vec![camera, 0.15f32.to_bits()]),
                (INTERFACE_MANAGER, vec![]),
                (INTERFACE_OBJECT_007FD640, vec![0x0c30_0000]),
                (POINTER_CONSTRUCT, vec![held, 0x0c40_0000]),
                (CULLER_SET_004A0FD0, vec![culler, 0x0c40_0000]),
                (OBJECT_BY_INDEX, vec![1]),
                (ACCUMULATE_SCENE, vec![camera, 0x7001, culler]),
                (RENDER_00B6C0D0, vec![camera, 0x0c40_0000, 0]),
                (CULLER_SET_004A0FD0, vec![culler, 0]),
                (POP_CULL_MODE, vec![culler]),
                (POINTER_DESTRUCT, vec![held]),
            ]
        );
    }

    #[test]
    fn render_needs_the_menu_and_the_camera() {
        let mut e = render_engine(0);
        e.set_global(SPECIAL_BOOK_MENU, 0x0e00_0000u32);
        e.call_log = Some(vec![]);
        e.call(0x007c_9ca0, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
        let mut e = render_engine(0x0d00_0000);
        e.call_log = Some(vec![]);
        e.call(0x007c_9ca0, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod n087_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The test vtable every object gets, and its slot functions (`SLOT + offset`).
    const VTABLE: u32 = 0x0e00_0000;
    const SLOT: u32 = 0x0e10_0000;

    const EXTERNAL: &[u32] = &[
        MODEL_LOADER_RELEASE,
        GET_MENU_BY_CLASS,
        OBJECT_BY_INDEX,
        OBJECT_CLEAR_004DEF90,
        OBJECT_RELEASE_00B5D180,
        SMART_POINTER_SET,
        ARRAY_CLEAR,
        SET_TRAIT_FROM_INT,
        MENU_START_FADE_OUT,
        POINTER_GET,
        RENDERER_OBJECT,
        CLEAR_BUFFER,
        SCENE_OBJECT,
        WORD_AT_B4,
        PUSH_CULL_MODE,
        POP_CULL_MODE,
        CAMERA_SETUP_00709D50,
        INTERFACE_MANAGER,
        INTERFACE_OBJECT_007FD640,
        POINTER_CONSTRUCT,
        POINTER_DESTRUCT,
        CULLER_SET_004A0FD0,
        ACCUMULATE_SCENE,
        RENDER_00B6C0D0,
        MANAGER_ROOT_00586150,
        TILE_READ_FILE,
        TILE_GET_MENU,
        MENU_SET_MENU_TILE,
        MENU_PREP_FOR_VISIBILITY,
        MENU_TILE_00726070,
        MENU_CHECK_00791750,
        DEBUG_MESSAGE,
        EMERGENCY_CLOSE_ALL_MENUS,
        SCRIPT_005CFCB0,
        IS_TOP_MENU_ID,
        INTERFACE_TEST_004B71D0,
        CONTROLS_OF,
        CONTROL_STATE_00A24180,
        SETTING_STRING,
        SETTING_VALUE,
        COMPARE_00408B20,
        SECONDS_0084D030,
        FIXED_STRING_CONSTRUCT,
        FIXED_STRING_DESTROY,
        GEOMETRY_ROOT_0043B230,
        GET_CONTROLLER,
        SEQUENCE_BY_NAME,
        DEACTIVATE_ALL,
        ACTIVATE_SEQUENCE,
        CONTROLLER_0047AA40,
        SEQUENCE_004954C0,
        SEQUENCE_UPDATE,
        SEQUENCE_TIME_00508120,
        SEQUENCE_TIME_00508100,
        START_ANIMATIONS,
        IS_KIND_OF_00653270,
        NEXT_CONTROLLER_004A8A90,
        TRANSLATION_0043D410,
        POINT_00416870,
        SET_TRANSLATE_00A59C60,
        ARRAY_COUNT,
        ARRAY_AT,
        ARRAY_ADD,
        ARRAY_CONSTRUCT,
        GET_PROPERTY,
        PROPERTY_TYPE_00441110,
        SET_HIDDEN_00450F90,
        AUDIO_INSTANCE,
        AUDIO_GET_SOUND_HANDLE_BY_NAME,
        SOUND_HANDLE_PLAY,
        SOUND_HANDLE_CONSTRUCT,
        SOUND_HANDLE_ASSIGN,
        SOUND_HANDLE_DESTROY,
        SET_ACTOR_BASE_VALUE,
        CLAMP_HIGH_004A8F20,
        CLAMP_LOW_00647B70,
        ALLOCATE,
        ALLOCATE_00AA13E0,
        FTOL2,
        CURSOR_X_009A9350,
        CURSOR_Y_00792760,
        VALUE_OF_0084E3A0,
        PICK_CONSTRUCT,
        PICK_00458B30,
        PICK_SET_TARGET_00705FC0,
        PICK_006ECD40,
        PICK_00792780,
        PICK_OBJECTS,
        PICK_RESULTS_00500940,
        PICK_RESULT_00968670,
        PICK_DESTROY,
        CAMERA_WINDOW_POINT_TO_RAY,
        POINT_SELF_006815C0,
        MODEL_LOADER_LOAD_FILE,
        SWAP_PLATFORM_LANGUAGE_TEXTURES,
        SET_SCALE_00440490,
        SET_ROTATION_004BC1F0,
        ROTATE_006404B0,
        GET_WORLD_BOUND,
        ADD_ALL_LIGHTS_FROM_NODE,
        ADD_LIGHT,
        LIGHT_CONSTRUCT,
        LIGHT_AMBIENT_00440460,
        LIGHT_DIFFUSE_004BC2E0,
        LIGHT_POSITION_0050DD50,
        LIGHT_DIMMER_0050DD20,
        RENDERER_WIDTH,
        RENDERER_HEIGHT,
        CAMERA_CONSTRUCT,
        VIEWPORT_CONSTRUCT,
        CAMERA_SET_VIEWPORT,
        FRUSTUM_CONSTRUCT,
        CAMERA_FAR_00507700,
        HALF_ANGLE_0057DD70,
        CAMERA_SET_VIEW_FRUSTUM,
        CAMERA_LOOK_AT_WORLD_POINT,
        CAMERA_00733FD0,
        SET_NAME_00A5B950,
        SWAP_LANGUAGE_TEXTURE_PATH,
        CREATE_TEXTURE_IMAGE,
        XBOX_IMAGES_00709D40,
        COPY_00406D30,
        FORMAT_00406D00,
    ];

    struct Rig {
        e: Engine,
        book: u32,
        player: u32,
        values: Rc<RefCell<[i32; 16]>>,
        added: Rc<RefCell<Vec<(u32, u32)>>>,
    }

    fn object(e: &mut Engine, size: u32) -> u32 {
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, VTABLE);
        object
    }

    fn rig() -> Rig {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x8_0000);
        e.map(0x011a_0000, 0x1000);
        e.map(0x011c_0000, 0x4_0000);
        e.map(0x0120_3000, 0x1000);
        for &callee in EXTERNAL {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(FTOL2, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            (value as i32).into_ret()
        });
        e.register(ARRAY_COUNT, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(ARRAY_AT, |e, a| (e.mem.u32(a[0] + 4) + 4 * a[1]).into_ret());
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        let added = Rc::new(RefCell::new(Vec::new()));
        let log = added.clone();
        e.register_double(ARRAY_ADD, move |e, a| {
            log.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        let slots: Vec<u32> = (0..0x100).map(|i| SLOT + 4 * i).collect();
        e.put_vtable(VTABLE, &slots);
        for slot in slots {
            e.register(slot, |_, _| Ret::default());
        }
        let values = Rc::new(RefCell::new([0i32; 16]));
        let read = values.clone();
        e.register_double(SLOT + 0x1c, move |_, a| {
            read.borrow()[a[1] as usize & 0xf].into_ret()
        });
        let cell = e.mem.alloc(8);
        e.register_double(SETTING_VALUE, move |_, _| cell.into_ret());
        let book = object(&mut e, 0x1e4);
        let player = e.mem.alloc(0xa8);
        e.mem.set_u32(player + 0xa4, VTABLE);
        e.set_global(PLAYER, player);
        e.set_global(SPECIAL_BOOK_MENU, book);
        e.call_log = Some(vec![]);
        Rig {
            e,
            book,
            player,
            values,
            added,
        }
    }

    fn answer(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| value.into_ret());
    }

    fn answer_f64(e: &mut Engine, address: u32, value: f64) {
        e.register_double(address, move |_, _| value.into_ret());
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Puts `items` into the page array at `array` (data at +4, count at +8, the model the
    /// array doubles read).
    fn fill(e: &mut Engine, array: u32, items: &[u32]) {
        let data = e.mem.alloc(4 * items.len().max(1) as u32);
        for (index, item) in items.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *item);
        }
        e.mem.set_u32(array + 4, data);
        e.mem.set_u32(array + 8, items.len() as u32);
    }

    /// An item whose callbacks (+0, +4) are `click` and `refresh`.
    fn item(e: &mut Engine, click: u32, refresh: u32) -> u32 {
        let item = e.mem.alloc(0x28);
        e.mem.set_u32(item, click);
        e.mem.set_u32(item + 4, refresh);
        item
    }

    /// Sets the points spent: the player's values 5 to 11 (all `each`).
    fn spent(r: &Rig, each: i32) {
        let mut values = r.values.borrow_mut();
        for value in values.iter_mut().take(12).skip(5) {
            *value = each;
        }
    }

    #[test]
    fn create_uses_0x23_points() {
        let mut r = rig();
        r.e.call(0x007c_5fb0, &args![]);
        assert_eq!(calls(&r.e, TILE_READ_FILE)[0][1], MENU_FILE_TEXT);
    }

    #[test]
    fn create_ov2_builds_the_book() {
        let mut r = rig();
        let book = r.book;
        r.e.set_global(SPECIAL_BOOK_MENU, 0u32);
        answer(&mut r.e, TILE_READ_FILE, 0x0c40_0000);
        answer(&mut r.e, TILE_GET_MENU, book);
        answer(&mut r.e, SLOT + 0x34, 0x424);
        answer(&mut r.e, MENU_CHECK_00791750, 1);
        let geometry = object(&mut r.e, 0x40);
        answer(&mut r.e, MODEL_LOADER_LOAD_FILE, geometry);
        let scene = object(&mut r.e, 0x40);
        answer(&mut r.e, OBJECT_BY_INDEX, scene);
        answer(&mut r.e, SCENE_OBJECT, scene);
        r.e.call(0x007c_5fc0, &args![7u32]);
        assert_eq!(r.e.global::<u32>(SPECIAL_BOOK_MENU), book);
        assert_eq!(r.e.mem.u32(book + AVAILABLE_POINTS), 7);
        assert_eq!(r.e.mem.u32(book + BOOK_GEOMETRY), geometry);
        assert_eq!(
            calls(&r.e, SMART_POINTER_SET),
            vec![vec![POINTER_011DAA9C, 0]]
        );
        assert_eq!(calls(&r.e, SCRIPT_005CFCB0), vec![vec![1]]);
        assert_eq!(calls(&r.e, MENU_PREP_FOR_VISIBILITY), vec![vec![book, 0]]);
    }

    #[test]
    fn create_ov2_closes_everything_on_a_missing_tile() {
        let mut r = rig();
        let book = r.book;
        let tile = object(&mut r.e, 0x10);
        answer(&mut r.e, TILE_READ_FILE, tile);
        answer(&mut r.e, TILE_GET_MENU, book);
        answer(&mut r.e, SLOT + 0x34, 0x424);
        r.e.register(MENU_CHECK_00791750, |e, a| {
            e.mem.set_u32(a[1], 12);
            Ret::default()
        });
        r.e.call(0x007c_5fc0, &args![7u32]);
        assert_eq!(
            calls(&r.e, DEBUG_MESSAGE),
            vec![vec![CREATION_FAILED_TEXT, 12]]
        );
        assert_eq!(calls(&r.e, SLOT), vec![vec![tile, 1]]);
        assert_eq!(calls(&r.e, EMERGENCY_CLOSE_ALL_MENUS).len(), 1);
        assert_eq!(r.e.mem.u32(book + AVAILABLE_POINTS), 0);
        // Another class: nothing is made.
        answer(&mut r.e, SLOT + 0x34, 0x425);
        r.e.call(0x007c_5fc0, &args![7u32]);
        assert_eq!(calls(&r.e, DEBUG_MESSAGE).len(), 1);
    }

    #[test]
    fn menu_class_is_0x424() {
        let mut r = rig();
        assert_eq!(r.e.call(0x007c_6110, &args![]).u32(), 0x424);
    }

    #[test]
    fn do_idle_cancels_or_runs_the_turn_animation() {
        let mut r = rig();
        let book = r.book;
        answer(&mut r.e, IS_TOP_MENU_ID, 1);
        answer(&mut r.e, CONTROL_STATE_00A24180, 1);
        r.e.call(0x007c_6200, &args![book]);
        assert_eq!(calls(&r.e, CONTROL_STATE_00A24180)[0][1..], [0x10, 1]);
        assert_eq!(calls(&r.e, SLOT + 0xc), vec![vec![book, 4, 0]]);
        let mut r = rig();
        let book = r.book;
        answer(&mut r.e, IS_TOP_MENU_ID, 1);
        answer(&mut r.e, COMPARE_00408B20, 1);
        let root = object(&mut r.e, 0x10);
        answer(&mut r.e, GEOMETRY_ROOT_0043B230, root);
        r.e.mem.set_u32(book + STATE, 2);
        r.e.mem.set_u32(BACKWARD_ANIMATIONS + 8, 0x0c50_0000);
        answer(&mut r.e, SEQUENCE_BY_NAME, 0x0c51_0000);
        answer_f64(&mut r.e, SECONDS_0084D030, 0.5);
        answer_f64(&mut r.e, SEQUENCE_TIME_00508120, 1.0);
        answer_f64(&mut r.e, SEQUENCE_TIME_00508100, 2.0);
        answer(&mut r.e, IS_KIND_OF_00653270, 1);
        r.e.call(0x007c_6200, &args![book]);
        assert_eq!(calls(&r.e, CONTROL_STATE_00A24180)[0][1], 0x1e);
        assert_eq!(calls(&r.e, FIXED_STRING_CONSTRUCT)[0][1], 0x0c50_0000);
        assert_eq!(r.e.mem.f32(book + FRAME_ANIM_TIMER), 0.5);
        assert_eq!(
            calls(&r.e, SEQUENCE_UPDATE),
            vec![vec![0x0c51_0000, 0.5f32.to_bits(), 1]]
        );
        assert_eq!(r.e.mem.u8(book + ANIM_FINISHED), 0);
        assert_eq!(calls(&r.e, SLOT + 0x94).len(), 1);
        assert_eq!(calls(&r.e, SET_TRANSLATE_00A59C60).len(), 1);
    }

    #[test]
    fn do_gamepad_raises_and_lowers_the_selected_value() {
        let mut r = rig();
        let book = r.book;
        assert!(!r.e.call(0x007c_6440, &args![book, 3u32, 0u32]).bool());
        answer(&mut r.e, IS_TOP_MENU_ID, 1);
        r.e.mem.set_u32(book + STATE, 1);
        r.e.mem.set_u32(book + AVAILABLE_POINTS, 30);
        spent(&r, 2);
        assert!(r.e.call(0x007c_6440, &args![book, 9u32, 0u32]).bool());
        assert_eq!(calls(&r.e, SLOT + 0xc), vec![vec![book, 2, 0]]);
        assert!(r.e.call(0x007c_6440, &args![book, 4u32, 0u32]).bool());
        assert_eq!(calls(&r.e, SLOT + 0xc)[1], vec![book, 3, 0]);
        assert!(!r.e.call(0x007c_6440, &args![book, 5u32, 0u32]).bool());
        // No page selected: the buttons answer true and do nothing.
        r.e.mem.set_u32(book + STATE, 0);
        assert!(r.e.call(0x007c_6440, &args![book, 3u32, 0u32]).bool());
        assert_eq!(calls(&r.e, SLOT + 0xc).len(), 2);
    }

    #[test]
    fn do_click_turns_changes_and_closes() {
        let mut r = rig();
        let book = r.book;
        r.e.mem.set_u8(book + ANIM_FINISHED, 1);
        r.e.call(0x007c_6550, &args![book, 0u32, 0u32]);
        assert_eq!(r.e.mem.u32(book + STATE), 1);
        assert_eq!(r.e.mem.u8(TURNED_FORWARD), 1);
        // Increase on page 1 (value 5): base value 3 differs from 3? set it to differ.
        r.e.mem.set_u32(book + AVAILABLE_POINTS, 40);
        spent(&r, 3);
        r.e.register(CLAMP_HIGH_004A8F20, |_, a| a[0].into_ret());
        answer(&mut r.e, SLOT, 4);
        r.e.call(0x007c_6550, &args![book, 2u32, 0u32]);
        let player = r.player;
        assert_eq!(calls(&r.e, SET_ACTOR_BASE_VALUE), vec![vec![player, 5, 4]]);
        assert_eq!(
            calls(&r.e, AUDIO_GET_SOUND_HANDLE_BY_NAME).last().unwrap()[2],
            SPECIAL_NUMBER_SOUND
        );
        // Close with points left plays "nothing".
        r.e.call(0x007c_6550, &args![book, 4u32, 0u32]);
        assert_eq!(
            calls(&r.e, AUDIO_GET_SOUND_HANDLE_BY_NAME).last().unwrap()[2],
            ACTIVATE_NOTHING_SOUND
        );
        // Index selection moves only on page 8.
        r.e.mem.set_u32(book + STATE, 8);
        r.e.mem.set_u32(book + INDEX_STATE, 3);
        r.e.call(0x007c_6550, &args![book, 6u32, 0u32]);
        assert_eq!(r.e.mem.u32(book + INDEX_STATE), 2);
        r.e.call(0x007c_6550, &args![book, 7u32, 0u32]);
        assert_eq!(r.e.mem.u32(book + INDEX_STATE), 3);
    }

    #[test]
    fn do_click_picks_the_button_under_the_cursor() {
        let mut r = rig();
        let book = r.book;
        let button = item(&mut r.e, 0x0c60_0000, 0);
        r.e.mem.set_u32(button + 8, 0x0c61_0000);
        fill(&mut r.e, book + PAGES, &[button]);
        r.e.register(0x0c60_0000, |_, _| Ret::default());
        answer_f64(&mut r.e, CURSOR_X_009A9350, 10.0);
        answer_f64(&mut r.e, CURSOR_Y_00792760, 20.0);
        answer(&mut r.e, PICK_OBJECTS, 1);
        answer(&mut r.e, PICK_RESULT_00968670, 1);
        r.e.call(0x007c_6550, &args![book, 5u32, 0u32]);
        assert_eq!(calls(&r.e, PICK_SET_TARGET_00705FC0)[0][1], 0x0c61_0000);
        assert_eq!(calls(&r.e, CAMERA_WINDOW_POINT_TO_RAY)[0][1..3], [10, 20]);
        assert_eq!(calls(&r.e, 0x0c60_0000), vec![vec![button]]);
        assert_eq!(calls(&r.e, PICK_DESTROY).len(), 1);
    }

    #[test]
    fn do_key_maps_the_page_and_value_keys() {
        let mut r = rig();
        let book = r.book;
        assert!(r.e.call(0x007c_6c50, &args![book, 0x8000_0001u32]).bool());
        assert_eq!(calls(&r.e, SLOT + 0xc), vec![vec![book, 1, 0]]);
        assert!(r.e.call(0x007c_6c50, &args![book, 0x8000_0004u32]).bool());
        assert_eq!(
            calls(&r.e, SLOT + 0x38),
            vec![vec![book, 4, 1.0f32.to_bits()]]
        );
        assert!(!r.e.call(0x007c_6c50, &args![book, 5u32]).bool());
    }

    #[test]
    fn init_book_builds_every_page() {
        let mut r = rig();
        let book = r.book;
        let geometry = object(&mut r.e, 0x40);
        answer(&mut r.e, MODEL_LOADER_LOAD_FILE, geometry);
        let scene = object(&mut r.e, 0x40);
        answer(&mut r.e, OBJECT_BY_INDEX, scene);
        r.e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        r.e.call(0x007c_6ce0, &args![book]);
        assert_eq!(
            calls(&r.e, MODEL_LOADER_LOAD_FILE),
            vec![vec![0, BOOK_MODEL_PATH, 0, 1, 0, 0, 0]]
        );
        assert_eq!(calls(&r.e, ALLOCATE).len(), 75);
        let added = r.added.borrow();
        assert_eq!(added.len(), 75 + 2 * 28);
        let (array, first) = added[0];
        assert_eq!(array, book + 0x60);
        assert_eq!(r.e.mem.u32(first), 0x007c_abb0);
        assert_eq!(r.e.mem.u32(first + 4), 0x007c_a860);
        // The light is made when the scene has none.
        assert_eq!(calls(&r.e, ADD_LIGHT).len(), 1);
        assert_eq!(calls(&r.e, CREATE_TEXTURE_IMAGE).len(), 25);
    }

    #[test]
    fn init_camera_sets_the_frustum() {
        let mut r = rig();
        let book = r.book;
        let scene = object(&mut r.e, 0x40);
        answer(&mut r.e, SCENE_OBJECT, scene);
        answer(&mut r.e, ALLOCATE_00AA13E0, 0x0c70_0000);
        answer(&mut r.e, CAMERA_CONSTRUCT, 0x0c71_0000);
        answer_f64(&mut r.e, HALF_ANGLE_0057DD70, 2.0);
        answer(&mut r.e, RENDERER_WIDTH, 800);
        answer(&mut r.e, RENDERER_HEIGHT, 400);
        r.e.set_global(FRUSTUM_SCALE, 0.75f32);
        let frustum = Rc::new(RefCell::new(Vec::new()));
        let seen = frustum.clone();
        r.e.register_double(CAMERA_SET_VIEW_FRUSTUM, move |e, a| {
            for word in 0..4 {
                seen.borrow_mut().push(e.mem.f32(a[1] + 4 * word));
            }
            Ret::default()
        });
        assert_eq!(r.e.call(0x007c_99c0, &args![book]).u32(), 0x0c71_0000);
        assert_eq!(*frustum.borrow(), vec![-1.5, 1.5, 3.0, -3.0]);
        assert_eq!(calls(&r.e, SET_NAME_00A5B950).len(), 1);
    }

    #[test]
    fn turn_page_stops_at_the_ends() {
        let mut r = rig();
        let book = r.book;
        r.e.mem.set_u32(book + STATE, 9);
        assert!(!r.e.call(0x007c_9dc0, &args![book, 1u32]).bool());
        assert_eq!(r.e.mem.u32(book + STATE), 9);
        assert!(r.e.call(0x007c_9dc0, &args![book, 0u32]).bool());
        assert_eq!(r.e.mem.u32(book + STATE), 8);
        assert_eq!(r.e.mem.u8(TURNED_FORWARD), 0);
        r.e.mem.set_u32(book + STATE, 0);
        assert!(!r.e.call(0x007c_9dc0, &args![book, 0u32]).bool());
        assert_eq!(
            calls(&r.e, AUDIO_GET_SOUND_HANDLE_BY_NAME)[0][2],
            PAGE_TURN_SOUND
        );
    }

    #[test]
    fn selected_value_follows_the_page() {
        let mut r = rig();
        let book = r.book;
        assert_eq!(r.e.call(0x007c_a030, &args![book]).i32(), -1);
        r.e.mem.set_u32(book + STATE, 8);
        r.e.mem.set_u32(book + INDEX_STATE, 3);
        assert_eq!(r.e.call(0x007c_a030, &args![book]).i32(), 12);
        answer(&mut r.e, INTERFACE_TEST_004B71D0, 1);
        assert_eq!(r.e.call(0x007c_a030, &args![book]).i32(), 7);
    }

    #[test]
    fn button_constructor_finds_its_node() {
        let mut r = rig();
        let geometry = object(&mut r.e, 0x40);
        r.e.mem.set_u32(r.book + BOOK_GEOMETRY, geometry);
        answer(&mut r.e, SLOT + 0x9c, 0x0c80_0000);
        answer(&mut r.e, GET_PROPERTY, 0x0c81_0000);
        answer(&mut r.e, PROPERTY_TYPE_00441110, 9);
        let button = r.e.mem.alloc(0x14);
        r.e.call(0x007c_a080, &args![button, 1u32, 2u32, 0x0c82_0000u32]);
        assert_eq!([r.e.mem.u32(button), r.e.mem.u32(button + 4)], [1, 2]);
        assert_eq!(
            calls(&r.e, SMART_POINTER_SET),
            vec![vec![button + 8, 0x0c80_0000]]
        );
        assert_eq!(r.e.mem.u32(button + 0xc), 0x0c81_0000);
        answer(&mut r.e, PROPERTY_TYPE_00441110, 13);
        r.e.call(0x007c_a080, &args![button, 1u32, 2u32, 0x0c82_0000u32]);
        assert_eq!(r.e.mem.u32(button + 0xc), 0);
    }

    #[test]
    fn actor_value_button_adds_its_items() {
        let mut r = rig();
        let geometry = object(&mut r.e, 0x40);
        r.e.mem.set_u32(r.book + BOOK_GEOMETRY, geometry);
        let button = r.e.mem.alloc(0x28);
        let made =
            r.e.call(
                0x007c_a200,
                &args![button, 1u32, 2u32, 3u32, 6u32, 2u32, 0x11u32, 0x22u32],
            )
            .u32();
        assert_eq!(made, button);
        assert_eq!(r.e.mem.u32(button + 0x24), 6);
        assert_eq!(calls(&r.e, ARRAY_CONSTRUCT), vec![vec![button + 0x14]]);
        assert_eq!(
            *r.added.borrow(),
            vec![(button + 0x14, 0x11), (button + 0x14, 0x22)]
        );
    }

    #[test]
    fn text_item_constructor_finds_one_or_two_nodes() {
        let mut r = rig();
        let geometry = object(&mut r.e, 0x40);
        r.e.mem.set_u32(r.book + BOOK_GEOMETRY, geometry);
        answer(&mut r.e, SLOT + 0x9c, 0x0c80_0000);
        let text = r.e.mem.alloc(0x18);
        r.e.mem.set_u32(text + 0x10, 0x99);
        r.e.call(0x007c_a2c0, &args![text, 1u32, 2u32, 0u32, 5u32]);
        assert_eq!(r.e.mem.u32(text), 1);
        assert_eq!(r.e.mem.u32(text + 0x14), 5);
        assert_eq!(r.e.mem.u32(text + 0xc), 0x0c80_0000);
        assert_eq!(r.e.mem.u32(text + 0x10), 0x99);
        r.e.call(0x007c_a2c0, &args![text, 1u32, 2u32, 3u32, 5u32]);
        assert_eq!(r.e.mem.u32(text + 0x10), 0x0c80_0000);
    }

    #[test]
    fn point_value_shows_the_digit() {
        let mut r = rig();
        let book = r.book;
        r.values.borrow_mut()[6] = 7;
        r.e.mem.set_u32(book + DIGIT_TEXTURES + 28, 0x0c90_0007);
        let text = object(&mut r.e, 0x18);
        r.e.mem.set_u32(text + 4, text);
        r.e.mem.set_u32(text + 0x14, 6);
        r.e.call(0x007c_a4d0, &args![text]);
        assert_eq!(
            calls(&r.e, SLOT + 0xf0),
            vec![vec![text, 0, 0, 0x0c90_0007]]
        );
    }

    #[test]
    fn message_follows_the_points_left() {
        for (points, slot, texture) in [
            (40u32, 0xfcu32, 0x1b0u32),
            (36, 0xf0, 0x1b4),
            (35, 0xf0, 0x1b8),
        ] {
            let mut r = rig();
            let book = r.book;
            spent(&r, 5);
            r.e.mem.set_u32(book + AVAILABLE_POINTS, points);
            r.e.mem.set_u32(book + texture, 0x0c90_0000 + texture);
            let text = object(&mut r.e, 0x18);
            r.e.mem.set_u32(text + 4, text);
            r.e.call(0x007c_a530, &args![text]);
            let shown = calls(&r.e, SLOT + slot);
            assert_eq!(shown[0].last(), Some(&(0x0c90_0000 + texture)));
        }
    }

    #[test]
    fn points_left_shows_two_digits() {
        let mut r = rig();
        let book = r.book;
        spent(&r, 1);
        r.e.mem.set_u32(book + AVAILABLE_POINTS, 7 + 23);
        for digit in 0..11u32 {
            r.e.mem
                .set_u32(book + DIGIT_TEXTURES + 4 * digit, 0x0c90_0000 + digit);
        }
        let text = r.e.mem.alloc(0x18);
        let tens = object(&mut r.e, 0x10);
        let ones = object(&mut r.e, 0x10);
        for (offset, word) in [
            (4, tens),
            (8, ones),
            (0xc, 0x0ca0_0001),
            (0x10, 0x0ca0_0002),
        ] {
            r.e.mem.set_u32(text + offset, word);
        }
        r.e.call(0x007c_a5f0, &args![text]);
        assert_eq!(
            calls(&r.e, SET_HIDDEN_00450F90),
            vec![vec![0x0ca0_0001, 0], vec![0x0ca0_0002, 0]]
        );
        assert_eq!(
            calls(&r.e, SLOT + 0xf0),
            vec![vec![tens, 0, 0, 0x0c90_0002], vec![ones, 0, 0, 0x0c90_0003]]
        );
        r.e.mem.set_u32(book + AVAILABLE_POINTS, 7);
        r.e.call(0x007c_a5f0, &args![text]);
        assert_eq!(
            calls(&r.e, SET_HIDDEN_00450F90)[2..],
            [vec![0x0ca0_0001, 1], vec![0x0ca0_0002, 1]]
        );
    }

    #[test]
    fn decrease_button_hides_at_one() {
        let mut r = rig();
        let button = r.e.mem.alloc(0x28);
        r.e.mem.set_u32(button + 8, 0x0cb0_0000);
        r.e.mem.set_u32(button + 0x24, 6);
        r.values.borrow_mut()[6] = 1;
        r.e.call(0x007c_a700, &args![button]);
        r.values.borrow_mut()[6] = 2;
        r.e.call(0x007c_a700, &args![button]);
        assert_eq!(
            calls(&r.e, SET_HIDDEN_00450F90),
            vec![vec![0x0cb0_0000, 1], vec![0x0cb0_0000, 0]]
        );
        // On the index page another value's button is hidden too.
        answer(&mut r.e, INTERFACE_TEST_004B71D0, 1);
        r.e.mem.set_u32(r.book + STATE, 8);
        r.e.call(0x007c_a700, &args![button]);
        assert_eq!(calls(&r.e, SET_HIDDEN_00450F90)[3], vec![0x0cb0_0000, 1]);
    }

    #[test]
    fn increase_button_hides_at_ten_or_without_points() {
        let mut r = rig();
        let button = r.e.mem.alloc(0x28);
        r.e.mem.set_u32(button + 8, 0x0cb0_0000);
        r.e.mem.set_u32(button + 0x24, 6);
        r.e.mem.set_u32(r.book + AVAILABLE_POINTS, 10);
        r.e.call(0x007c_a7a0, &args![button]);
        spent(&r, 2);
        r.e.call(0x007c_a7a0, &args![button]);
        assert_eq!(
            calls(&r.e, SET_HIDDEN_00450F90),
            vec![vec![0x0cb0_0000, 0], vec![0x0cb0_0000, 1]]
        );
    }

    #[test]
    fn page_buttons_show_their_images() {
        let mut r = rig();
        let book = r.book;
        let button = r.e.mem.alloc(0x14);
        r.e.mem.set_u32(button + 0xc, button);
        r.e.mem.set_u32(button, VTABLE);
        for offset in [0x1c0u32, 0x1c4, 0x1cc, 0x1d0] {
            r.e.mem.set_u32(book + offset, 0x0c90_0000 + offset);
        }
        r.e.call(0x007c_a860, &args![button]);
        r.e.call(0x007c_a8a0, &args![button]);
        r.e.mem.set_u32(book + AVAILABLE_POINTS, 1);
        r.e.call(0x007c_a8e0, &args![button]);
        r.e.mem.set_u32(book + AVAILABLE_POINTS, 0);
        r.e.call(0x007c_a8e0, &args![button]);
        let shown: Vec<u32> = calls(&r.e, SLOT + 0xf0).iter().map(|w| w[3]).collect();
        assert_eq!(
            shown,
            vec![0x0c90_01c0, 0x0c90_01c4, 0x0c90_01d0, 0x0c90_01cc]
        );
    }

    #[test]
    fn decrease_and_increase_change_the_base_value() {
        let mut r = rig();
        let book = r.book;
        let player = r.player;
        let button = r.e.mem.alloc(0x28);
        r.e.mem.set_u32(button + 0x24, 6);
        r.values.borrow_mut()[6] = 4;
        r.e.mem.set_u32(book + AVAILABLE_POINTS, 40);
        r.e.register(CLAMP_LOW_00647B70, |_, a| a[0].into_ret());
        r.e.register(CLAMP_HIGH_004A8F20, |_, a| a[0].into_ret());
        r.e.call(0x007c_a960, &args![button]);
        r.e.call(0x007c_aa80, &args![button]);
        assert_eq!(
            calls(&r.e, SET_ACTOR_BASE_VALUE),
            vec![vec![player, 6, 3], vec![player, 6, 5]]
        );
        assert_eq!(calls(&r.e, AUDIO_GET_SOUND_HANDLE_BY_NAME).len(), 2);
        // No points left: increase only plays.
        r.e.mem.set_u32(book + AVAILABLE_POINTS, 0);
        r.e.call(0x007c_aa80, &args![button]);
        assert_eq!(calls(&r.e, SET_ACTOR_BASE_VALUE).len(), 2);
    }

    #[test]
    fn page_turn_buttons_need_a_finished_turn() {
        let mut r = rig();
        let book = r.book;
        r.e.call(0x007c_abb0, &args![0u32]);
        assert_eq!(r.e.mem.u32(book + STATE), 0);
        r.e.mem.set_u8(book + ANIM_FINISHED, 1);
        r.e.call(0x007c_abb0, &args![0u32]);
        assert_eq!(r.e.mem.u32(book + STATE), 1);
        r.e.call(0x007c_abf0, &args![0u32]);
        assert_eq!(r.e.mem.u32(book + STATE), 0);
    }

    #[test]
    fn all_done_closes_when_every_point_is_spent() {
        let mut r = rig();
        let book = r.book;
        r.e.mem.set_u32(book + AVAILABLE_POINTS, 1);
        r.e.call(0x007c_ac30, &args![0u32]);
        assert_eq!(
            calls(&r.e, AUDIO_GET_SOUND_HANDLE_BY_NAME)[0][2],
            ACTIVATE_NOTHING_SOUND
        );
        r.e.mem.set_u32(book + AVAILABLE_POINTS, 0);
        r.e.call(0x007c_ac30, &args![0u32]);
        assert_eq!(calls(&r.e, AUDIO_GET_SOUND_HANDLE_BY_NAME).len(), 1);
        assert_eq!(calls(&r.e, GET_MENU_BY_CLASS).len(), 1);
    }

    #[test]
    fn refresh_page_runs_texts_then_buttons() {
        let mut r = rig();
        let book = r.book;
        r.e.mem.set_u32(book + STATE, 1);
        let text = item(&mut r.e, 0x0cc0_0000, 0);
        let button = item(&mut r.e, 0, 0x0cc0_0001);
        fill(&mut r.e, book + 0x90, &[text]);
        fill(&mut r.e, book + 0x80, &[button]);
        r.e.register(0x0cc0_0000, |_, _| Ret::default());
        r.e.register(0x0cc0_0001, |_, _| Ret::default());
        r.e.call(0x007c_acd0, &args![book]);
        assert_eq!(calls(&r.e, 0x0cc0_0000), vec![vec![text]]);
        assert_eq!(calls(&r.e, 0x0cc0_0001), vec![vec![button]]);
        r.e.mem.set_u32(book + STATE, 9);
        r.e.call(0x007c_acd0, &args![book]);
        assert_eq!(calls(&r.e, 0x0cc0_0000).len(), 1);
    }

    #[test]
    fn images_load_into_their_slots() {
        let mut r = rig();
        let book = r.book;
        answer(&mut r.e, XBOX_IMAGES_00709D40, 1);
        r.e.call(0x007c_add0, &args![book]);
        let slots: Vec<u32> = calls(&r.e, CREATE_TEXTURE_IMAGE)
            .iter()
            .map(|w| w[2] - book)
            .collect();
        let mut want: Vec<u32> = (0..11).map(|i| 0x180 + 4 * i).collect();
        want.extend((1..5).map(|i| 0x1a8 + 4 * i));
        want.extend(BUTTON_IMAGES.iter().map(|(_, slot)| *slot));
        assert_eq!(slots, want);
        assert_eq!(calls(&r.e, COPY_00406D30)[0][1..], [5, XBOX_TEXT]);
        assert_eq!(calls(&r.e, SWAP_LANGUAGE_TEXTURE_PATH).len(), 11 + 4 + 6);
    }
}
