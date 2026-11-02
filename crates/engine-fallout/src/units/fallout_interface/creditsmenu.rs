//! `fallout/interface/menus/creditsmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// The handle of the empty `NiFixedString`.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;

// Translated from 007613f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the map files it under `creditsmenu.cpp`, a
/// range placement; the body is not menu code): stores the handle of the empty
/// `NiFixedString` (the word at `0109b220`) in the word at `this` and returns
/// `this`, the default constructor of an `NiFixedString`-sized object (the stream
/// code uses it as the element constructor of its fixed string array). The
/// decompiler shows the stored value as 0; the disassembly reads the global.
pub fn fn_007613f0(e: &mut Engine, this: Ptr) -> Ptr {
    let empty = e.global::<u32>(EMPTY_FIXED_STRING);
    e.mem.set_u32(this.addr(), empty);
    this
}

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VNiFixedString@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_01072514: u32 = 0x0107_2514;
/// `007615d0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_007615D0: u32 = 0x0076_15d0;

// Translated from 007614a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VNiFixedString@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `007615d0(this, 1)`.
pub fn fn_007614a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_01072514);
    e.call(ARRAY_RELEASE_007615D0, &args![this, 1u32]);
}

/// `00438b90` (unnamed in the engine map): `thiscall`; whether `iSize == iReservedSize` (the
/// array is full), in `AL`.
const ARRAY_IS_FULL: u32 = 0x0043_8b90;
/// `009a3910` (unnamed in the engine map): `thiscall`; the next capacity: `iReservedSize + 0x400`
/// when it is above `0x400`, otherwise twice `iReservedSize`.
const ARRAY_NEXT_CAPACITY: u32 = 0x009a_3910;
/// `0042fc00` (unnamed in the engine map): `thiscall`, `RET 8`; reallocates the buffer to the
/// given capacity, keeping the given number of elements.
const ARRAY_REALLOCATE: u32 = 0x0042_fc00;
/// Vtable slot (byte offset) that allocates a block of the requested number of elements.
const SLOT_ALLOCATE: u32 = 0x04;

// Translated from 00761540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`creditsmenu.cpp`; the body is a generic `BSSimpleArray` slot
/// reservation, folded from the template). `__fastcall`: when the array is full, allocates a
/// four-element buffer through virtual slot +4 if it has no capacity, otherwise reallocates to
/// the next capacity; then counts one more element and answers the index of the new slot.
pub fn fn_00761540(e: &mut Engine, this: Ptr<BSSimpleArray>) -> u32 {
    if e.call(ARRAY_IS_FULL, &args![this]).bool() {
        if e.get(this, BSSimpleArray::iReservedSize) == 0 {
            let capacity = 4u32;
            let buffer = e.vcall(this.addr(), SLOT_ALLOCATE, &args![capacity]).u32();
            e.set(this, BSSimpleArray::pBuffer, buffer);
            e.set(this, BSSimpleArray::iReservedSize, capacity);
        } else {
            let capacity = e.call(ARRAY_NEXT_CAPACITY, &args![this]).u32();
            let size = e.get(this, BSSimpleArray::iSize);
            e.call(ARRAY_REALLOCATE, &args![this, capacity, size]);
            e.set(this, BSSimpleArray::iReservedSize, capacity);
        }
    }
    let size = e.get(this, BSSimpleArray::iSize).wrapping_add(1);
    e.set(this, BSSimpleArray::iSize, size);
    size.wrapping_sub(1)
}

// Translated from 0075f6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 8`: a mask and a byte flag): sets the mask's bits in the word
/// at +0x1a8 when the flag is non-zero, clears them otherwise.
pub fn fn_0075f6f0(e: &mut Engine, this: Ptr, mask: u32, set: u8) {
    let field = this.addr().wrapping_add(0x1a8);
    let flags = e.mem.u32(field);
    let flags = if set != 0 {
        flags | mask
    } else {
        !mask & flags
    };
    e.mem.set_u32(field, flags);
}

/// The byte `00761460` sets.
const FLAG_011DCFA5: u32 = 0x011d_cfa5;

// Translated from 00761460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, one word): stores the argument's low byte at `011dcfa5`.
pub fn fn_00761460(e: &mut Engine, value: u8) {
    e.set_global(FLAG_011DCFA5, value);
}

layout! {
    /// `CreditsMenu` (Xbox PDB), 0x80 bytes there; the PC code uses these offsets.
    pub struct CreditsMenu: 0x80 {
        /// `pClickRect` (Xbox PDB).
        0x28 pClickRect: u32,
        /// `pBackgroundTile` (Xbox PDB).
        0x2C pBackgroundTile: u32,
        /// `spScrollRoot` (Xbox PDB): `NiPointer<NiNode>`.
        0x30 spScrollRoot: u32,
        /// `iFileOffset` (Xbox PDB): the read position in the credits text.
        0x34 iFileOffset: u32,
        /// `iFileSize` (Xbox PDB).
        0x38 iFileSize: u32,
        /// `pFileData` (Xbox PDB): `char *`.
        0x3C pFileData: u32,
        /// `eFont` (Xbox PDB).
        0x40 eFont: u32,
        /// `iJustification` (Xbox PDB).
        0x44 iJustification: i32,
        /// `iX` (Xbox PDB).
        0x48 iX: i32,
        /// `iY` (Xbox PDB).
        0x4C iY: i32,
        /// `Color` (Xbox PDB): `NiColorA`, its red component (the others follow).
        0x50 Color: f32,
        /// `uiCreditsTotalHeight` (Xbox PDB).
        0x70 uiCreditsTotalHeight: u32,
        /// `fBackgroundSwapInterval` (Xbox PDB).
        0x74 fBackgroundSwapInterval: f32,
        /// `fBackgroundSwapTimer` (Xbox PDB).
        0x78 fBackgroundSwapTimer: f32,
        /// `uiBackgroundIndexCounter` (Xbox PDB).
        0x7C uiBackgroundIndexCounter: u32,
    }
}

/// `BackgroundTextureFilenames` (Xbox PDB): the `BSSimpleArray<NiFixedString,1024>` at +0x60.
const BACKGROUNDS: u32 = 0x60;

/// The menu class (`CreditsMenu::GetClass`).
const CREDITS_MENU_CLASS: u32 = 0x417;
/// The `CreditsMenu` vtable.
const CREDITS_MENU_VTABLE: u32 = 0x0107_235c;
/// The start menu (`StartMenu *` global) and the save-load manager global.
const START_MENU: u32 = 0x011d_aac0;
const SAVE_LOAD_MANAGER: u32 = 0x011d_e134;
/// The save entry `Close` offers to load (a static it sets while searching).
const OFFERED_ENTRY_011D9474: u32 = 0x011d_9474;
/// A byte static `DoIdle` sets while menu 0x3e9 is visible.
const PAUSED_011D94FC: u32 = 0x011d_94fc;
/// A byte static `Create` tests before hiding the click tile.
const FLAG_011D947C: u32 = 0x011d_947c;
/// The audio object `Close` and the destructor give 0.0 (`0046dce0` on the word at
/// `011c3f3c`), and whose `00526ac0` value 2.0 selects the wacky credits.
const AUDIO_OBJECT_011C3F3C: u32 = 0x011c_3f3c;
/// The input manager (the word at `011dea0c`, `00877720`).
const INPUT_HOLDER: u32 = 0x011d_ea0c;
const INPUT_MANAGER: u32 = 0x0087_7720;
/// The player (`PlayerCharacter *` global).
const PLAYER: u32 = 0x011d_ea3c;
/// The frame timer (`011f6394`) and its seconds passed (`0084d030`, also used on a bound for
/// its radius: the float at +0xc).
const FRAME_TIMER: u32 = 0x011f_6394;
const FLOAT_AT_0XC: u32 = 0x0084_d030;
/// The scroll speed setting.
const SETTING_SCROLL_SPEED: u32 = 0x011d_94e4;
/// The settings of the "load the save" message.
const SETTING_LOAD_PROMPT: u32 = 0x011d_39b4;
const SETTING_LOAD_YES: u32 = 0x011d_34f8;
const SETTING_LOAD_NO: u32 = 0x011d_3684;

/// `Tile::GetMenuByClass` (cdecl), `Tile::GetMenu`, `Tile::GetMaximumDepth`,
/// `Tile::ReadFile` and the interface manager's tile reader.
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
const TILE_READ_FILE: u32 = 0x00a0_1b00;
const TILE_READER: u32 = 0x0058_6150;
const INTERFACE_MANAGER: u32 = 0x004b_7210;
/// The interface manager's flag `Create` writes into trait 0x1004 (`007023a0`).
const INTERFACE_FLAG_007023A0: u32 = 0x0070_23a0;
/// `Menu::SetMenuTile`, `Menu::StartFadeOut` and `Menu::~Menu`.
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
const MENU_DESTRUCTOR: u32 = 0x00a1_c520;
/// Tile traits: float getter/setter, integer setter, string setter.
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const TILE_SET_INT: u32 = 0x0070_0320;
const TILE_SET_STRING: u32 = 0x00a0_1350;
/// The console print (`005b5e40`, cdecl).
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// `__RTDynamicCast` (cdecl, five words) and the type descriptors `Create` casts with.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_0119F4CC: u32 = 0x0119_f4cc;
const TYPE_0119F49C: u32 = 0x0119_f49c;
const TYPE_0119F34C: u32 = 0x0119_f34c;
const TYPE_011A0034: u32 = 0x011a_0034;
const TYPE_01183028: u32 = 0x0118_3028;
const TYPE_011861DC: u32 = 0x0118_61dc;
/// The credits menu's validity test (`0073af60`).
const MENU_VALID_0073AF60: u32 = 0x0073_af60;
/// The word at +0 (`00559450`), the word at +4 (`00726070`), the word at +8 (`0044ddc0`, the
/// array's size), the address of a node's item (`006815c0`).
const FIRST_WORD: u32 = 0x0055_9450;
const WORD_AT_FOUR: u32 = 0x0072_6070;
const WORD_AT_EIGHT: u32 = 0x0044_ddc0;
const NODE_ITEM: u32 = 0x0068_15c0;
/// The array's element address (`00877a30`, index) and a fixed string's text (`0043b1b0`).
const ARRAY_ELEMENT: u32 = 0x0087_7a30;
const FIXED_STRING_TEXT: u32 = 0x0043_b1b0;
/// A setting's float address (`00403e20`), its string (`00403df0`) and its other string
/// accessor (`004c69f0`).
const SETTING_FLOAT_ADDRESS: u32 = 0x0040_3e20;
const SETTING_STRING: u32 = 0x0040_3df0;
const SETTING_STRING_004C69F0: u32 = 0x004c_69f0;
/// Node transforms: the local translate (`0043c490`), its setter (`00440460`, address of a
/// point), the `NiUpdateData` constructor (`0043d410`: 0.0, 0, 0), `UpdateDownwardPass`-style
/// update (`00a59c60`, data), the world bound (`0043d450`), the parent of a tile's node
/// (`0056c7f0`), an `NiPointer` assignment (`0066b0d0`).
const LOCAL_TRANSLATE: u32 = 0x0043_c490;
const SET_LOCAL_TRANSLATE: u32 = 0x0044_0460;
const UPDATE_DATA_CONSTRUCT: u32 = 0x0043_d410;
const NODE_UPDATE: u32 = 0x00a5_9c60;
const WORLD_BOUND: u32 = 0x0043_d450;
const TILE_NODE_0056C7F0: u32 = 0x0056_c7f0;
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// The renderer lock pair (`004bc3f0` returns the object; `004a0370` locks, `004a03c0`
/// unlocks).
const RENDER_LOCK_HOLDER: u32 = 0x004b_c3f0;
const RENDER_LOCK: u32 = 0x004a_0370;
const RENDER_UNLOCK: u32 = 0x004a_03c0;
/// The scroll root's test (`0052aa80`, 0), the object behind it (`009611e0`) and its slot
/// +0xe8 call.
const ROOT_TEST_0052AA80: u32 = 0x0052_aa80;
const ROOT_OBJECT_009611E0: u32 = 0x0096_11e0;
/// `StartMenu::_EndCredits`.
const START_MENU_END_CREDITS: u32 = 0x007d_6c90;
/// `NiPointer` release (`0045cec0`).
const POINTER_RELEASE: u32 = 0x0045_cec0;
/// The audio object's set (`0046dce0`, float) and value (`00526ac0`, `ST0`).
const AUDIO_SET_0046DCE0: u32 = 0x0046_dce0;
const AUDIO_VALUE_00526AC0: u32 = 0x0052_6ac0;
/// The start menu's flag test (`004a4080`, mask).
const START_MENU_FLAG: u32 = 0x004a_4080;
/// The save-load manager's refresh (`00851330`), an entry's test (`00846b10`), its data
/// load (`00846900`), the load of a save (`008467b0`) and the preparation `008304f0`.
const SAVES_REFRESH: u32 = 0x0085_1330;
const ENTRY_TEST_00846B10: u32 = 0x0084_6b10;
const ENTRY_LOAD_DATA: u32 = 0x0084_6900;
const ENTRY_LOAD_GAME: u32 = 0x0084_67b0;
const BEFORE_LOAD_008304F0: u32 = 0x0083_04f0;
/// `BSstristr` (cdecl: text, part) and the text length (`0044a670`, cdecl).
const STRISTR: u32 = 0x004b_75a0;
const TEXT_LENGTH: u32 = 0x0044_a670;
/// The message box (`00703e80`, cdecl, 11 words) and its result
/// (`Interface::GetMessageMenuResult`).
const MESSAGE_BOX: u32 = 0x0070_3e80;
const MESSAGE_RESULT: u32 = 0x0070_3fa0;
/// `Interface::IsTopMenuID` (cdecl), `Interface::IsMenuIDVisible` (cdecl: id, 0) and the
/// visibility test `007027b0` (cdecl: id).
const IS_TOP_MENU: u32 = 0x0070_2450;
const IS_MENU_VISIBLE: u32 = 0x0070_2680;
const MENU_SHOWN_007027B0: u32 = 0x0070_27b0;
/// The input tests `DoIdle` closes on: `00a23a50` (0, 1), `00a24660` (0x1c, 1) and
/// `InterfaceManager::CheckMenuButton` (cdecl: 9, 1).
const INPUT_TEST_00A23A50: u32 = 0x00a2_3a50;
const INPUT_TEST_00A24660: u32 = 0x00a2_4660;
const CHECK_MENU_BUTTON: u32 = 0x0071_7a40;
/// The input test `InitCreditsRoot` checks for keys 0x1d and 0x9d (`00a24180`, key, 0).
const INPUT_KEY_00A24180: u32 = 0x00a2_4180;
/// The form `InitCreditsRoot` looks up (`004839c0`, cdecl: form id) and its id.
const LOOKUP_FORM: u32 = 0x0048_39c0;
const WACKY_FORM_ID: u32 = 0x000e_d568;
/// `GetCurrentTexLanguage`, the language folder (`004b7650`, cdecl), the bounded copy
/// (`00406d30`) and append (`00406d50`), `memset` (`00ec61c0`).
const CURRENT_LANGUAGE: u32 = 0x004b_7220;
const LANGUAGE_FOLDER: u32 = 0x004b_7650;
const COPY_BOUNDED: u32 = 0x0040_6d30;
const APPEND_BOUNDED: u32 = 0x0040_6d50;
const PRINT_BOUNDED: u32 = 0x0040_6d00;
const MEMSET: u32 = 0x00ec_61c0;
/// `BSFile`: constructor (path, 0, 0x4000, 0), destructor, `Exist`; its slots +0x40 (open)
/// and +0x28 (size); the read `00462d80` (buffer, size).
const BS_FILE_CONSTRUCT: u32 = 0x00b0_0260;
const BS_FILE_DESTRUCT: u32 = 0x00af_f240;
const BS_FILE_EXIST: u32 = 0x00af_fc60;
const BS_FILE_READ: u32 = 0x0046_2d80;
/// The game's allocator entry and `operator delete`; Gamebryo's `NiNew` (`00aa13e0`) and
/// `NiAlloc` (`00aa1070`).
const ALLOCATE: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
const NI_NEW: u32 = 0x00aa_13e0;
const NI_ALLOC: u32 = 0x00aa_1070;
/// Scene graph constructors: `NiNode` (`00a5ecb0`, 0), the property `004391c0` (with
/// `0049ed90(1)`), `NiTriShape` (9 words), `TileShaderProperty` (0) with `SetTileTexture`,
/// `NiMaterialProperty`; `NiAVObject::AttachProperty`; the material setters `004bc450` and
/// `004534f0` (colour); the bound (`00537b10`) and `NiBound::ComputeFromData`;
/// `BSShaderManager::PrepareObject` (cdecl); the translate setter `004bc1f0` (x, y, z).
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
const PROPERTY_CONSTRUCT_004391C0: u32 = 0x0043_91c0;
const PROPERTY_SET_0049ED90: u32 = 0x0049_ed90;
const TRI_SHAPE_CONSTRUCT: u32 = 0x00a7_4410;
const TILE_SHADER_PROPERTY_CONSTRUCT: u32 = 0x00bb_7c30;
const TILE_SHADER_SET_TEXTURE: u32 = 0x00bb_7a10;
const SHADER_SET_ALPHA_00821600: u32 = 0x0082_1600;
const MATERIAL_PROPERTY_CONSTRUCT: u32 = 0x00a7_5650;
const ATTACH_PROPERTY: u32 = 0x0043_9410;
const MATERIAL_SET_004BC450: u32 = 0x004b_c450;
const MATERIAL_SET_004534F0: u32 = 0x0045_34f0;
const MATERIAL_COLOR: u32 = 0x011a_9b7c;
const MODEL_BOUND_00537B10: u32 = 0x0053_7b10;
const BOUND_COMPUTE: u32 = 0x00a7_ee30;
const PREPARE_OBJECT: u32 = 0x00b5_7e30;
const SET_TRANSLATE_004BC1F0: u32 = 0x004b_c1f0;
const NODE_ATTACH_SLOT: u32 = 0xdc;
/// The update of the scroll root (`00a29680`).
const ROOT_UPDATE_00A29680: u32 = 0x00a2_9680;
/// The vector constructor iterator (`00401050`: block, size, count, constructor), `NiPoint3`
/// (`00416870`: x, y, z) and `NiPoint2` (`00452dc0`: u, v) constructors.
const VECTOR_CONSTRUCTOR: u32 = 0x0040_1050;
const POINT3_CONSTRUCT: u32 = 0x0041_6870;
const POINT2_CONSTRUCT: u32 = 0x0045_2dc0;
/// `NiFixedString` from text (`00438170`), its destructor (`004381b0`) and assignment
/// (`0043ba10`); `NiSourceTexture::Create` (cdecl: name, format, 1, 0) and its format
/// object; `NiAVObject::SetName`-style `00a5b950`.
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
const FIXED_STRING_ASSIGN: u32 = 0x0043_ba10;
const SOURCE_TEXTURE_CREATE: u32 = 0x00a5_fd70;
const TEXTURE_FORMAT_011A9598: u32 = 0x011a_9598;
const OBJECT_SET_NAME_00A5B950: u32 = 0x00a5_b950;
/// `atol` (`00eca6d3`, cdecl).
const ATOL: u32 = 0x00ec_a6d3;
/// `Font` access: the font manager, a font by index, `Font::MakeString` (9 words) and the
/// line height (`00759450`, `ST0`).
const FONT_MANAGER: u32 = 0x005b_d5b0;
const FONT_MANAGER_FONT: u32 = 0x005b_d5c0;
const FONT_MAKE_STRING: u32 = 0x00a1_2460;
const FONT_LINE_HEIGHT: u32 = 0x0075_9450;
/// `_ftol2` (`ST0` passed as an `f64` argument).
const FTOL: u32 = 0x00ec_62c0;
/// `BSStringT`: constructor, destructor, `Set` (text, 0); `sprintf` into one (`00406f60`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTROY: u32 = 0x0040_37d0;
const STRING_SET: u32 = 0x0040_37f0;
const STRING_FORMAT: u32 = 0x0040_6f60;
/// The `BSSimpleArray` element constructor (`00761630`: address, count), the buffer release
/// (`006a8500`) and the element destructor (`004b9900`, 0).
const ARRAY_CONSTRUCT_ELEMENTS: u32 = 0x0076_1630;
const ARRAY_FREE_BUFFER: u32 = 0x006a_8500;
const ELEMENT_DESTRUCT: u32 = 0x004b_9900;

/// The two integers whose ratio `Create` compares with 0.75 (`004dc200` over `004dc1f0`;
/// read as the screen width and height, not confirmed).
const SCREEN_WIDTH_004DC200: u32 = 0x004d_c200;
const SCREEN_HEIGHT_004DC1F0: u32 = 0x004d_c1f0;
/// Strings and constants in the exe's data.
const MENU_FILE: u32 = 0x0107_23fc;
const CREATION_FAILED: u32 = 0x0107_23b0;
const DATA_FOLDER: u32 = 0x0107_24e8;
const BACKSLASH: u32 = 0x0101_3444;
const CREDITS_FILE: u32 = 0x0107_24c8;
const WACKY_FILE: u32 = 0x0107_24d4;
const DATA_CREDITS_FILE: u32 = 0x0107_249c;
const DATA_WACKY_FILE: u32 = 0x0107_24b0;
const PATCH_FILE: u32 = 0x0107_2470;
const WACKY_PATCH_FILE: u32 = 0x0107_2484;
const DATA_PATCH_FILE: u32 = 0x0107_243c;
const DATA_WACKY_PATCH_FILE: u32 = 0x0107_2454;
const BACKGROUND_FORMAT: u32 = 0x0107_2420;
const TEXTURE_FORMAT: u32 = 0x0107_24f0;
const ROOT_Z_01072438: u32 = 0x0107_2438;
const IMAGE_Y_01013EA4: u32 = 0x0101_3ea4;
const BYTE_SCALE: u32 = 0x0101_e568;
const ZERO: u32 = 0x0101_2060;
const TWO: u32 = 0x0101_1590;
const DEPTH_STACKING_A: u32 = 0x0107_04f0;
const DEPTH_STACKING_B: u32 = 0x0107_04e8;
const ASPECT_4_3: u32 = 0x0101_de30;
const WIDE_SHIFT: u32 = 0x0107_23a8;

/// Tile traits by number.
const TRAIT_VISIBLE: u32 = 0xfa3;
const TRAIT_STACKING_TYPE: u32 = 0xfa7;
const TRAIT_0XFA9: u32 = 0xfa9;
const TRAIT_DEPTH: u32 = 0xfad;
const TRAIT_IMAGE: u32 = 0xfcc;
const TRAIT_0X1004: u32 = 0x1004;
const TRAIT_0X1772: u32 = 0x1772;

fn first_word(e: &mut Engine, object: u32) -> u32 {
    e.call(FIRST_WORD, &args![object]).u32()
}

/// The scroll root node (`spScrollRoot`'s object).
fn scroll_root(e: &mut Engine, this: Ptr<CreditsMenu>) -> u32 {
    first_word(e, this.addr().wrapping_add(0x30))
}

/// Shows the next background: the file name at `uiBackgroundIndexCounter` (then counted on)
/// as the background tile's image.
fn show_next_background(e: &mut Engine, this: Ptr<CreditsMenu>) {
    let index = e.get(this, CreditsMenu::uiBackgroundIndexCounter);
    e.set(
        this,
        CreditsMenu::uiBackgroundIndexCounter,
        index.wrapping_add(1),
    );
    let element = e
        .call(
            ARRAY_ELEMENT,
            &args![this.addr().wrapping_add(BACKGROUNDS), index],
        )
        .u32();
    let name = e.call(FIXED_STRING_TEXT, &args![element]).u32();
    let tile = e.get(this, CreditsMenu::pBackgroundTile);
    e.call(TILE_SET_STRING, &args![tile, TRAIT_IMAGE, name, 1u32]);
}

/// A node's update: `NiUpdateData(0.0, 0, 0)` on the stack, then `00a59c60` with it.
fn update_node(e: &mut Engine, node: impl FnOnce(&mut Engine) -> u32) {
    e.with_stack(0xc, |e, data| {
        e.call(
            UPDATE_DATA_CONSTRUCT,
            &args![data, 0.0f32.to_bits(), 0u32, 0u32],
        );
        let node = node(e);
        e.call(NODE_UPDATE, &args![node, data]);
    });
}

// Translated from 0075f1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CreditsMenu::GetClass` (Xbox PDB; `thiscall`): 0x417.
pub fn credits_menu_get_class(_e: &mut Engine, _this: Ptr) -> u32 {
    CREDITS_MENU_CLASS
}

// Translated from 0075f1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CreditsMenu::`scalar deleting destructor'` (Xbox PDB; `thiscall`, `RET 4`): the
/// destructor, then `operator delete` when bit 0 of `flags` is set. Returns `this`.
pub fn credits_menu_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    credits_menu_destructor(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0075f200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CreditsMenu::~CreditsMenu` (Xbox PDB; `thiscall`): the vtable, the audio object's
/// `0046dce0(0.0)`, `StartMenu::_EndCredits` on the start menu when there is one, then the
/// background array (`007614a0`), the scroll root pointer and the `Menu` base.
// Translation incomplete: 0075f200 - the C++ exception frame (FS:[0], unwind states 2 to 0 for the members) is not translated.
pub fn credits_menu_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), CREDITS_MENU_VTABLE);
    let audio = e.global::<u32>(AUDIO_OBJECT_011C3F3C);
    e.call(AUDIO_SET_0046DCE0, &args![audio, 0.0f32.to_bits()]);
    if e.global::<u32>(START_MENU) != 0 {
        let start = e.global::<u32>(START_MENU);
        e.call(START_MENU_END_CREDITS, &args![start]);
    }
    fn_007614a0(e, Ptr::new(this.addr().wrapping_add(BACKGROUNDS)));
    e.call(POINTER_RELEASE, &args![this.addr().wrapping_add(0x30)]);
    e.call(MENU_DESTRUCTOR, &args![this]);
}

// Translated from 0075f2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CreditsMenu::Create` (Xbox PDB; no arguments): closes an open credits menu, reads
/// `Data\Menus\Options\credits_menu.xml` and, when its menu is of class 0x417, attaches it
/// and checks it (`0073af60`; prints the failure and returns false). Then the depth (0xfad,
/// when the stacking type is 6006 or 102), trait 0x1004 from the interface, the credits
/// (`InitCreditsRoot`); with background files the swap interval is `(total height - the
/// root's z) / scroll speed / count`, the timer starts at it and the first background shows;
/// without them (or with `011d947c` set) the click tile's 0xfa9 is cleared. Away from a 4:3
/// screen the root's parent node moves 214 along x and is updated. Returns true; a file whose
/// menu is of another class is deleted when its menu has a tile, and false is returned.
pub fn credits_menu_create(e: &mut Engine) -> bool {
    let old = e
        .call(TILE_GET_MENU_BY_CLASS, &args![CREDITS_MENU_CLASS])
        .u32();
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    let depth = x87_store32(e.call(TILE_GET_MAXIMUM_DEPTH, &args![]).f64());
    let reader = e.call(TILE_READER, &args![interface]).u32();
    let tile = e.call(TILE_READ_FILE, &args![reader, MENU_FILE]).u32();
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    if menu != 0 && e.vcall(menu, 0x34, &args![]).u32() == fn_0075f5f0(e) {
        let cast = e
            .call(
                RT_DYNAMIC_CAST,
                &args![tile, 0u32, TYPE_0119F4CC, TYPE_0119F49C, 0u32],
            )
            .u32();
        e.call(MENU_SET_MENU_TILE, &args![menu, cast, 0u32]);
        let credits = e
            .call(
                RT_DYNAMIC_CAST,
                &args![menu, 0u32, TYPE_0119F34C, TYPE_011A0034, 0u32],
            )
            .u32();
        let this = Ptr::<CreditsMenu>::new(credits);
        if !e.call(MENU_VALID_0073AF60, &args![credits]).bool() {
            e.call(PRINT_MESSAGE, &args![CREATION_FAILED]);
            return false;
        }
        // `FCOMP; TEST AH,0x44; JNP` (equal) on 6006, then `JP` (not equal) on 102.
        let stacking = e
            .call(TILE_GET_FLOAT, &args![tile, TRAIT_STACKING_TYPE])
            .f64();
        let set_depth = stacking == e.global::<f64>(DEPTH_STACKING_A)
            || e.call(TILE_GET_FLOAT, &args![tile, TRAIT_STACKING_TYPE])
                .f64()
                == e.global::<f64>(DEPTH_STACKING_B);
        if set_depth {
            e.call(TILE_SET_FLOAT, &args![tile, TRAIT_DEPTH, depth, 1u32]);
        }
        let flag = e.call(INTERFACE_FLAG_007023A0, &args![interface]).u8();
        e.call(TILE_SET_INT, &args![tile, TRAIT_0X1004, u32::from(flag)]);
        credits_menu_init_credits_root(e, this);
        let backgrounds = credits.wrapping_add(BACKGROUNDS);
        if e.call(WORD_AT_EIGHT, &args![backgrounds]).u32() != 0 {
            let total = f64::from(e.get(this, CreditsMenu::uiCreditsTotalHeight));
            let root = scroll_root(e, this);
            let translate = e.call(LOCAL_TRANSLATE, &args![root]).u32();
            let z = x87_m32(e.mem.u32(translate.wrapping_add(8)));
            let distance = x87_sub(total, z);
            let speed = e
                .call(SETTING_FLOAT_ADDRESS, &args![SETTING_SCROLL_SPEED])
                .u32();
            let time = x87_div(distance, x87_m32(e.mem.u32(speed)));
            let count = e.call(WORD_AT_EIGHT, &args![backgrounds]).u32();
            let interval = x87_store32(x87_div(time, f64::from(count)));
            e.set(
                this,
                CreditsMenu::fBackgroundSwapInterval,
                f32::from_bits(interval),
            );
            let interval =
                x87_quiet_bits(e.get(this, CreditsMenu::fBackgroundSwapInterval).to_bits());
            e.set(
                this,
                CreditsMenu::fBackgroundSwapTimer,
                f32::from_bits(interval),
            );
            show_next_background(e, this);
        } else if (e.global::<u8>(FLAG_011D947C) != 0
            || e.call(WORD_AT_EIGHT, &args![backgrounds]).u32() == 0)
            && e.get(this, CreditsMenu::pBackgroundTile) != 0
        {
            let background = e.get(this, CreditsMenu::pBackgroundTile);
            e.call(TILE_SET_INT, &args![background, TRAIT_0XFA9, 0u32]);
        }
        let width = e.call(SCREEN_WIDTH_004DC200, &args![]).i32();
        let height = e.call(SCREEN_HEIGHT_004DC1F0, &args![]).i32();
        let aspect = x87_div(f64::from(width), f64::from(height));
        // `FCOMP; TEST AH,0x44; JNP`: skipped only when exactly 0.75.
        if aspect != e.global::<f64>(ASPECT_4_3) {
            let click = e.get(this, CreditsMenu::pClickRect);
            let node = e.call(TILE_NODE_0056C7F0, &args![click]).u32();
            let translate = e.call(LOCAL_TRANSLATE, &args![node]).u32();
            let mut point = [0u32; 3];
            for (i, word) in point.iter_mut().enumerate() {
                *word = e.mem.u32(translate.wrapping_add(4 * i as u32));
            }
            point[0] = x87_store32(x87_add(x87_m32(point[0]), e.global::<f64>(WIDE_SHIFT)));
            e.with_stack(0xc, |e, moved| {
                for (i, word) in point.iter().enumerate() {
                    e.mem
                        .set_u32(moved.addr().wrapping_add(4 * i as u32), *word);
                }
                e.call(SET_LOCAL_TRANSLATE, &args![node, moved]);
            });
            update_node(e, |_| node);
        }
        return true;
    }
    if menu != 0 && e.call(WORD_AT_FOUR, &args![menu]).u32() != 0 && menu != 0 {
        e.vcall(menu, 0, &args![1u32]);
    }
    false
}

// Translated from 0075f5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no `this`, plain `RET`): the menu class 0x417 (the static `GetClass`).
pub fn fn_0075f5f0(_e: &mut Engine) -> u32 {
    CREDITS_MENU_CLASS
}

// Translated from 0075f600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): with a credits tile open, and its menu, under the render lock:
/// gives the scroll root's object (when the root passes `0052aa80(0)`) the root through its
/// slot +0xe8, clears the scroll root, sets the tile's 0x1772 to 1 and starts the fade out.
/// Then (with any tile) sets the start menu's flag 0x2000000.
pub fn fn_0075f600(e: &mut Engine) {
    let tile = e
        .call(TILE_GET_MENU_BY_CLASS, &args![CREDITS_MENU_CLASS])
        .u32();
    if tile == 0 {
        return;
    }
    let menu = e.call(TILE_GET_MENU, &args![tile]).u32();
    if menu != 0 {
        let lock = e.call(RENDER_LOCK_HOLDER, &args![]).u32();
        e.call(RENDER_LOCK, &args![lock]);
        let pointer = menu.wrapping_add(0x30);
        if e.call(ROOT_TEST_0052AA80, &args![pointer, 0u32]).bool() {
            let root = first_word(e, pointer);
            if e.call(ROOT_OBJECT_009611E0, &args![root]).u32() != 0 {
                let root = first_word(e, pointer);
                let object = e.call(ROOT_OBJECT_009611E0, &args![root]).u32();
                let root = first_word(e, pointer);
                e.vcall(object, 0xe8, &args![root]);
            }
        }
        e.call(POINTER_ASSIGN, &args![pointer, 0u32]);
        e.call(TILE_SET_INT, &args![tile, TRAIT_0X1772, 1u32]);
        e.call(MENU_START_FADE_OUT, &args![menu]);
        let lock = e.call(RENDER_LOCK_HOLDER, &args![]).u32();
        e.call(RENDER_UNLOCK, &args![lock]);
    }
    let start = e.global::<u32>(START_MENU);
    fn_0075f6f0(e, Ptr::new(start), 0x0200_0000, 1);
}

// Translated from 0075f750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CreditsMenu::Close` (Xbox PDB; no arguments): sets the audio object to 0.0; when the start
/// menu's flag 0x40000000 is set and the save-load manager has a player name, refreshes it
/// and looks for a save entry whose data loads (`0075f8c0`) and whose name contains that
/// name: the first one found becomes `011d9474` and a message box (`00703e80`, answered by
/// `00761410`) offers it, and the menu stays. Otherwise the flag is cleared and the menu
/// closes (`0075f600`).
pub fn credits_menu_close(e: &mut Engine) {
    let audio = e.global::<u32>(AUDIO_OBJECT_011C3F3C);
    e.call(AUDIO_SET_0046DCE0, &args![audio, 0.0f32.to_bits()]);
    let start = e.global::<u32>(START_MENU);
    if e.call(START_MENU_FLAG, &args![start, 0x4000_0000u32])
        .bool()
    {
        let saves = e.global::<u32>(SAVE_LOAD_MANAGER);
        let name = first_word(e, saves.wrapping_add(0x30));
        if e.call(TEXT_LENGTH, &args![name]).u32() != 0 {
            let saves = e.global::<u32>(SAVE_LOAD_MANAGER);
            e.call(SAVES_REFRESH, &args![saves]);
            if e.global::<u32>(SAVE_LOAD_MANAGER) != 0 {
                let saves = e.global::<u32>(SAVE_LOAD_MANAGER);
                if first_word(e, saves) != 0 {
                    let saves = e.global::<u32>(SAVE_LOAD_MANAGER);
                    let mut node = first_word(e, saves);
                    while node != 0 {
                        let slot = e.call(NODE_ITEM, &args![node]).u32();
                        if e.mem.u32(slot) == 0 {
                            break;
                        }
                        let slot = e.call(NODE_ITEM, &args![node]).u32();
                        let entry = e.mem.u32(slot);
                        e.set_global(OFFERED_ENTRY_011D9474, entry);
                        let entry = e.global::<u32>(OFFERED_ENTRY_011D9474);
                        if e.call(ENTRY_TEST_00846B10, &args![entry]).bool() {
                            let entry = e.global::<u32>(OFFERED_ENTRY_011D9474);
                            if fn_0075f8c0(e, Ptr::new(entry)) != 0 {
                                let entry = e.global::<u32>(OFFERED_ENTRY_011D9474);
                                let part = first_word(e, entry);
                                let saves = e.global::<u32>(SAVE_LOAD_MANAGER);
                                let text = first_word(e, saves.wrapping_add(0x30));
                                if e.call(STRISTR, &args![text, part]).u32() != 0 {
                                    let no = e.call(SETTING_STRING, &args![SETTING_LOAD_NO]).u32();
                                    let yes =
                                        e.call(SETTING_STRING, &args![SETTING_LOAD_YES]).u32();
                                    let prompt = e
                                        .call(SETTING_STRING_004C69F0, &args![SETTING_LOAD_PROMPT])
                                        .u32();
                                    e.call(
                                        MESSAGE_BOX,
                                        &args![
                                            prompt,
                                            0u32,
                                            0u32,
                                            0x0076_1410u32,
                                            0u32,
                                            0x12u32,
                                            0.0f32.to_bits(),
                                            0.0f32.to_bits(),
                                            yes,
                                            no,
                                            0u32
                                        ],
                                    );
                                    return;
                                }
                            }
                        }
                        node = e.call(WORD_AT_FOUR, &args![node]).u32();
                    }
                }
            }
        }
    }
    let start = e.global::<u32>(START_MENU);
    fn_0075f6f0(e, Ptr::new(start), 0x4000_0000, 0);
    fn_0075f600(e);
}

// Translated from 0075f8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on a `BGSSaveLoadFileEntry`): loads its data
/// (`BGSSaveLoadFileEntry::LoadData`) and returns the word at +0xc.
pub fn fn_0075f8c0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(ENTRY_LOAD_DATA, &args![this]);
    e.mem.u32(this.addr().wrapping_add(0xc))
}

// Translated from 0075f8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CreditsMenu::AttachTileByID` (Xbox PDB; `thiscall`, `RET 8`: id, tile): id 1 is the click
/// tile (+0x28), id 0 the background tile (+0x2c).
pub fn credits_menu_attach_tile_by_id(e: &mut Engine, this: Ptr<CreditsMenu>, id: i32, tile: u32) {
    if id == 1 {
        e.set(this, CreditsMenu::pClickRect, tile);
    } else if id == 0 {
        e.set(this, CreditsMenu::pBackgroundTile, tile);
    }
}

// Translated from 0075f910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CreditsMenu::DoIdle` (Xbox PDB; `thiscall`). While menu 0x3e9 is visible the click tile is
/// hidden (0xfad and 0xfa3 cleared) and `011d94fc` set; once it is gone the menu closes. The
/// menu also closes on the input tests (`00a23a50`, `00a24660`, `CheckMenuButton(9, 1)` == 2).
/// With a scroll root: the background timer runs down by the frame time and, once below 0,
/// the next background shows (while any are left) and the timer restarts; the root rises by
/// the frame time times the scroll speed and is updated; when twice its world bound's radius
/// is below its height the menu closes.
pub fn credits_menu_do_idle(e: &mut Engine, this: Ptr<CreditsMenu>) {
    let holder = e.global::<u32>(INPUT_HOLDER);
    let input = e.call(INPUT_MANAGER, &args![holder]).u32();
    if e.call(IS_MENU_VISIBLE, &args![0x3e9u32, 0u32]).bool() {
        let click = e.get(this, CreditsMenu::pClickRect);
        e.call(TILE_SET_INT, &args![click, TRAIT_DEPTH, 0u32]);
        let click = e.get(this, CreditsMenu::pClickRect);
        e.call(TILE_SET_INT, &args![click, TRAIT_VISIBLE, 0u32]);
        e.set_global(PAUSED_011D94FC, 1u8);
        return;
    }
    if e.global::<u8>(PAUSED_011D94FC) != 0 {
        e.set_global(PAUSED_011D94FC, 0u8);
        credits_menu_close(e);
    }
    if e.call(INPUT_TEST_00A23A50, &args![input, 0u32, 1u32]).u32() != 0
        || e.call(INPUT_TEST_00A24660, &args![input, 0x1cu32, 1u32])
            .u32()
            != 0
        || e.call(CHECK_MENU_BUTTON, &args![9u32, 1u32]).u32() == 2
    {
        credits_menu_close(e);
    }
    if scroll_root(e, this) == 0 {
        return;
    }
    let backgrounds = this.addr().wrapping_add(BACKGROUNDS);
    if e.call(WORD_AT_EIGHT, &args![backgrounds]).u32() != 0 {
        let elapsed = e.call(FLOAT_AT_0XC, &args![FRAME_TIMER]).f64();
        let timer = e.get(this, CreditsMenu::fBackgroundSwapTimer).to_bits();
        let timer = x87_store32(x87_sub(x87_m32(timer), elapsed));
        e.set(
            this,
            CreditsMenu::fBackgroundSwapTimer,
            f32::from_bits(timer),
        );
        let timer = e.get(this, CreditsMenu::fBackgroundSwapTimer).to_bits();
        // `FCOMP; TEST AH,5; JP`: on only when the timer is below 0.
        if x87_m32(timer) < e.global::<f64>(ZERO)
            && e.get(this, CreditsMenu::uiBackgroundIndexCounter)
                < e.call(WORD_AT_EIGHT, &args![backgrounds]).u32()
        {
            show_next_background(e, this);
            let interval =
                x87_quiet_bits(e.get(this, CreditsMenu::fBackgroundSwapInterval).to_bits());
            e.set(
                this,
                CreditsMenu::fBackgroundSwapTimer,
                f32::from_bits(interval),
            );
        }
    }
    let root = scroll_root(e, this);
    let translate = e.call(LOCAL_TRANSLATE, &args![root]).u32();
    let mut point = [0u32; 3];
    for (i, word) in point.iter_mut().enumerate() {
        *word = e.mem.u32(translate.wrapping_add(4 * i as u32));
    }
    let speed = e
        .call(SETTING_FLOAT_ADDRESS, &args![SETTING_SCROLL_SPEED])
        .u32();
    let elapsed = e.call(FLOAT_AT_0XC, &args![FRAME_TIMER]).f64();
    let rise = x87_mul(elapsed, x87_m32(e.mem.u32(speed)));
    point[2] = x87_store32(x87_add(rise, x87_m32(point[2])));
    e.with_stack(0xc, |e, moved| {
        for (i, word) in point.iter().enumerate() {
            e.mem
                .set_u32(moved.addr().wrapping_add(4 * i as u32), *word);
        }
        let root = scroll_root(e, this);
        e.call(SET_LOCAL_TRANSLATE, &args![root, moved]);
    });
    update_node(e, |e| scroll_root(e, this));
    let root = scroll_root(e, this);
    let bound = e.call(WORLD_BOUND, &args![root]).u32();
    let radius = e.call(FLOAT_AT_0XC, &args![bound]).f64();
    let reach = x87_store32(x87_add(radius, radius));
    // `FCOMPP; TEST AH,5; JP`: closes only when `reach < height`.
    if x87_m32(reach) < x87_m32(point[2]) {
        credits_menu_close(e);
    }
}

// Translated from 0075fb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CreditsMenu::DoGamepad` (Xbox PDB; `thiscall`, `RET 8`: button, an unread word): buttons
/// outside -1..=4 close the menu when it is on top and menu 0x3e9 is not shown; true then.
pub fn credits_menu_do_gamepad(e: &mut Engine, _this: Ptr, button: i32, _unused_2: u32) -> bool {
    if (-1..=4).contains(&button) {
        return false;
    }
    if !e.call(IS_TOP_MENU, &args![CREDITS_MENU_CLASS]).bool()
        || e.call(MENU_SHOWN_007027B0, &args![0x3e9u32]).bool()
    {
        return false;
    }
    credits_menu_close(e);
    true
}

/// The byte of the credits text at the read position plus `ahead`.
fn text_byte(e: &Engine, this: Ptr<CreditsMenu>, ahead: u32) -> u8 {
    let data = e.get(this, CreditsMenu::pFileData);
    let position = e.get(this, CreditsMenu::iFileOffset);
    e.mem.u8(data.wrapping_add(position).wrapping_add(ahead))
}

/// `atol` of the credits text at the read position plus `ahead`.
fn number_at(e: &mut Engine, this: Ptr<CreditsMenu>, ahead: u32) -> i32 {
    let position = e.get(this, CreditsMenu::iFileOffset);
    let data = e.get(this, CreditsMenu::pFileData);
    e.call(
        ATOL,
        &args![position.wrapping_add(data).wrapping_add(ahead)],
    )
    .i32()
}

/// Moves the read position by `step`.
fn advance(e: &mut Engine, this: Ptr<CreditsMenu>, step: u32) {
    let position = e.get(this, CreditsMenu::iFileOffset).wrapping_add(step);
    e.set(this, CreditsMenu::iFileOffset, position);
}

/// A font's line height, truncated (`00759450`, then `_ftol2`).
fn line_height(e: &mut Engine, font: u32) -> i32 {
    let height = e.call(FONT_LINE_HEIGHT, &args![font]).f64();
    e.call(FTOL, &args![height]).i32()
}

/// Moves down one line of `font`: `iY` decreases and the total height grows by its height.
fn next_line(e: &mut Engine, this: Ptr<CreditsMenu>, font: u32) {
    let height = line_height(e, font);
    let y = e.get(this, CreditsMenu::iY).wrapping_sub(height);
    e.set(this, CreditsMenu::iY, y);
    let height = line_height(e, font);
    let total = (height as u32).wrapping_add(e.get(this, CreditsMenu::uiCreditsTotalHeight));
    e.set(this, CreditsMenu::uiCreditsTotalHeight, total);
}

/// The font of `eFont` (`005bd5c0` on the font manager).
fn current_font(e: &mut Engine, this: Ptr<CreditsMenu>) -> u32 {
    let index = e.get(this, CreditsMenu::eFont);
    let fonts = e.call(FONT_MANAGER, &args![]).u32();
    e.call(FONT_MANAGER_FONT, &args![fonts, index]).u32()
}

/// A value of 0..255 in the text as a float 0..1 (`FILD`, `FDIV` by the double 255.0).
fn byte_fraction(e: &mut Engine, value: i32) -> u32 {
    x87_store32(x87_div(f64::from(value), e.global::<f64>(BYTE_SCALE)))
}

/// One `<` tag of the credits text, the read position on the tag letter.
fn credits_tag(e: &mut Engine, this: Ptr<CreditsMenu>, text: u32, font: &mut u32) {
    let base = this.addr();
    let tag = (text_byte(e, this, 0) as i8 as i32).wrapping_sub(0x42) as u32;
    match tag {
        // `B`: a background texture name.
        0 => {
            advance(e, this, 2);
            fn_00760bd0(e, this, text);
            e.with_stack(0xc, |e, locals| {
                let name = locals.addr();
                let fixed = name.wrapping_add(8);
                e.call(STRING_CONSTRUCT, &args![name]);
                let pointer = e.mem.u32(text);
                let length = e.mem.u32(text.wrapping_add(4));
                e.call(
                    STRING_FORMAT,
                    &args![name, BACKGROUND_FORMAT, pointer, length],
                );
                let name_text = first_word(e, name);
                let made = e
                    .call(FIXED_STRING_CONSTRUCT, &args![fixed, name_text])
                    .u32();
                fn_007614c0(e, Ptr::new(base.wrapping_add(BACKGROUNDS)), made);
                e.call(FIXED_STRING_DESTRUCT, &args![fixed]);
                e.call(STRING_DESTROY, &args![name]);
            });
        }
        // `C`: the colour, four numbers.
        1 => {
            for (ahead, offset) in [(2u32, 0x50u32), (6, 0x54), (10, 0x58), (0xe, 0x5c)] {
                let value = number_at(e, this, ahead);
                let fraction = byte_fraction(e, value);
                e.mem.set_u32(base.wrapping_add(offset), fraction);
            }
        }
        // `F`: the font number.
        4 => {
            let index = (text_byte(e, this, 2) as i8 as i32).wrapping_sub(0x31);
            e.set(this, CreditsMenu::eFont, index as u32);
            *font = current_font(e, this);
        }
        // `I`: an image with its width, height and alpha.
        7 => {
            let width = number_at(e, this, 2);
            let height = number_at(e, this, 7);
            let alpha = number_at(e, this, 0xc);
            let alpha = byte_fraction(e, alpha);
            advance(e, this, 0x10);
            fn_00760bd0(e, this, text);
            let image = credits_menu_create_image(
                e,
                this,
                text,
                width,
                height,
                f32::from_bits(x87_quiet_bits(alpha)),
            );
            let root = scroll_root(e, this);
            e.vcall(root, NODE_ATTACH_SLOT, &args![image, 1u32]);
            let y = e.get(this, CreditsMenu::iY).wrapping_sub(height);
            e.set(this, CreditsMenu::iY, y);
            let total = e
                .get(this, CreditsMenu::uiCreditsTotalHeight)
                .wrapping_add(height as u32);
            e.set(this, CreditsMenu::uiCreditsTotalHeight, total);
        }
        // `J`: the justification (`C` 2, `R` 4, else 1).
        8 => {
            let justification = if text_byte(e, this, 2) == b'C' {
                2
            } else if text_byte(e, this, 2) == b'R' {
                4
            } else {
                1
            };
            e.set(this, CreditsMenu::iJustification, justification);
        }
        // `X`: x, absolute or relative.
        0x16 => {
            if text_byte(e, this, 2) == b'+' {
                let value = number_at(e, this, 3);
                let x = value.wrapping_add(e.get(this, CreditsMenu::iX));
                e.set(this, CreditsMenu::iX, x);
            } else if text_byte(e, this, 2) == b'-' {
                let value = number_at(e, this, 3);
                let x = e.get(this, CreditsMenu::iX).wrapping_sub(value);
                e.set(this, CreditsMenu::iX, x);
            } else {
                let value = number_at(e, this, 2);
                e.set(this, CreditsMenu::iX, value);
            }
        }
        // `Y`: y, absolute or relative (`+` moves down).
        0x17 => {
            if text_byte(e, this, 2) == b'+' {
                let value = number_at(e, this, 3);
                let y = e.get(this, CreditsMenu::iY).wrapping_sub(value);
                e.set(this, CreditsMenu::iY, y);
            } else if text_byte(e, this, 2) == b'-' {
                let value = number_at(e, this, 3);
                let y = value.wrapping_add(e.get(this, CreditsMenu::iY));
                e.set(this, CreditsMenu::iY, y);
            } else {
                let value = number_at(e, this, 2);
                e.set(this, CreditsMenu::iY, value);
            }
        }
        _ => {}
    }
    fn_00760b80(e, this);
}

// Translated from 0075fb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CreditsMenu::InitCreditsRoot` (Xbox PDB; `thiscall`). Picks the wacky credits (keys 0x1d
/// or 0x9d held, the player's slot +0x4a0 on form 0xed568, or the audio value 2.0), opens the
/// four candidate files (`DATA\<language>\CREDITS[WACKY].TXT`, `DATA\CREDITS[WACKY].TXT` and
/// their `PATCH` forms; `BSFile` objects on the stack) and reads the patch file of whichever
/// exists (the language one first) into a buffer. It makes the scroll root (an `NiNode` at z
/// -1020 under the click tile's node, with a property) and lays the text out line by line:
/// `\r` and text lines move down a line of the current font, `*` comments run to the end of
/// the line, `<` tags (`B` background, `C` colour, `F` font, `I` image, `J` justification,
/// `X`, `Y` position) are read with `atol`, control characters are skipped, and text runs
/// become `Font::MakeString` shapes at (`iX`, `iY`). Then the buffer is freed, the root
/// prepared and updated under the render lock, and the files closed.
// Translation incomplete: 0075fb90 - the C++ exception frame (FS:[0], unwind states for the four files, the string and the temporaries) and the /GS stack cookie check (00ec408c) are not translated.
pub fn credits_menu_init_credits_root(e: &mut Engine, this: Ptr<CreditsMenu>) {
    use crate::units::bssystem::bsfile::BSFile;
    let base = this.addr();
    let holder = e.global::<u32>(INPUT_HOLDER);
    let input = e.call(INPUT_MANAGER, &args![holder]).u32();
    let form = e.call(LOOKUP_FORM, &args![WACKY_FORM_ID]).u32();
    let form = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_01183028, TYPE_011861DC, 0u32],
        )
        .u32();
    let mut wacky = e
        .call(INPUT_KEY_00A24180, &args![input, 0x1du32, 0u32])
        .u32()
        != 0
        || e.call(INPUT_KEY_00A24180, &args![input, 0x9du32, 0u32])
            .u32()
            != 0
        || {
            let player = e.global::<u32>(PLAYER);
            e.vcall(player, 0x4a0, &args![form, 0u32]).bool()
        };
    if e.global::<u32>(AUDIO_OBJECT_011C3F3C) != 0 {
        let audio = e.global::<u32>(AUDIO_OBJECT_011C3F3C);
        // `FCOMP; TEST AH,0x44; JP`: on only when exactly 2.0.
        if e.call(AUDIO_VALUE_00526AC0, &args![audio]).f64() == e.global::<f64>(TWO) {
            wacky = true;
        }
    }
    let file_size = BSFile::SIZE;
    // The path (0x104 bytes), the four files and the line string, as the code's locals.
    let frame_size = 0x104 + 4 * file_size + 8;
    e.with_stack(frame_size, |e, frame| {
        let path = frame.addr();
        let language_file = path.wrapping_add(0x104);
        let data_file = language_file.wrapping_add(file_size);
        let language_patch = data_file.wrapping_add(file_size);
        let data_patch = language_patch.wrapping_add(file_size);
        let text = data_patch.wrapping_add(file_size);
        let folder = e.mem.u32(DATA_FOLDER);
        e.mem.set_u32(path, folder);
        let folder_end = e.mem.u16(DATA_FOLDER.wrapping_add(4));
        e.mem.set_u16(path.wrapping_add(4), folder_end);
        e.call(MEMSET, &args![path.wrapping_add(6), 0u32, 0xfeu32]);
        let language = e.call(CURRENT_LANGUAGE, &args![]).u32();
        let folder = e.call(LANGUAGE_FOLDER, &args![language]).u32();
        e.call(APPEND_BOUNDED, &args![path, 0x104u32, folder]);
        e.call(APPEND_BOUNDED, &args![path, 0x104u32, BACKSLASH]);
        let name = if wacky { WACKY_FILE } else { CREDITS_FILE };
        e.call(APPEND_BOUNDED, &args![path, 0x104u32, name]);
        e.call(
            BS_FILE_CONSTRUCT,
            &args![language_file, path, 0u32, 0x4000u32, 0u32],
        );
        let name = if wacky {
            DATA_WACKY_FILE
        } else {
            DATA_CREDITS_FILE
        };
        e.call(
            BS_FILE_CONSTRUCT,
            &args![data_file, name, 0u32, 0x4000u32, 0u32],
        );
        e.call(COPY_BOUNDED, &args![path, 0x104u32, DATA_FOLDER]);
        let folder = e.call(LANGUAGE_FOLDER, &args![language]).u32();
        e.call(APPEND_BOUNDED, &args![path, 0x104u32, folder]);
        e.call(APPEND_BOUNDED, &args![path, 0x104u32, BACKSLASH]);
        let name = if wacky { WACKY_PATCH_FILE } else { PATCH_FILE };
        e.call(APPEND_BOUNDED, &args![path, 0x104u32, name]);
        e.call(
            BS_FILE_CONSTRUCT,
            &args![language_patch, path, 0u32, 0x4000u32, 0u32],
        );
        let name = if wacky {
            DATA_WACKY_PATCH_FILE
        } else {
            DATA_PATCH_FILE
        };
        e.call(
            BS_FILE_CONSTRUCT,
            &args![data_patch, name, 0u32, 0x4000u32, 0u32],
        );
        let file = if e.call(BS_FILE_EXIST, &args![language_file]).bool() {
            if e.call(BS_FILE_EXIST, &args![language_patch]).bool() {
                language_patch
            } else {
                language_file
            }
        } else if e.call(BS_FILE_EXIST, &args![data_patch]).bool() {
            data_patch
        } else {
            data_file
        };
        let close_files = |e: &mut Engine| {
            for file in [data_patch, language_patch, data_file, language_file] {
                e.call(BS_FILE_DESTRUCT, &args![file]);
            }
        };
        if file == 0 || !e.vcall(file, 0x40, &args![]).bool() {
            close_files(e);
            return;
        }
        let size = e.vcall(file, 0x28, &args![]).u32().wrapping_add(1);
        e.set(this, CreditsMenu::iFileSize, size);
        let size = e.get(this, CreditsMenu::iFileSize);
        let data = e.call(ALLOCATE, &args![size]).u32();
        e.set(this, CreditsMenu::pFileData, data);
        let size = e.get(this, CreditsMenu::iFileSize);
        let data = e.get(this, CreditsMenu::pFileData);
        e.mem.set_u8(size.wrapping_add(data).wrapping_sub(1), 0);
        let wanted = e.get(this, CreditsMenu::iFileSize).wrapping_sub(1);
        let data = e.get(this, CreditsMenu::pFileData);
        let read = e.call(BS_FILE_READ, &args![file, data, wanted]).u32();
        if read != e.get(this, CreditsMenu::iFileSize).wrapping_sub(1) {
            let data = e.get(this, CreditsMenu::pFileData);
            e.call(OPERATOR_DELETE, &args![data]);
            e.set(this, CreditsMenu::pFileData, 0);
            close_files(e);
            return;
        }
        let block = e.call(NI_NEW, &args![0xacu32]).u32();
        let node = if block != 0 {
            e.call(NI_NODE_CONSTRUCT, &args![block, 0u32]).u32()
        } else {
            0
        };
        e.call(POINTER_ASSIGN, &args![base.wrapping_add(0x30), node]);
        e.with_stack(0xc, |e, point| {
            let z = x87_quiet_bits(e.global::<u32>(ROOT_Z_01072438));
            let made = e
                .call(
                    POINT3_CONSTRUCT,
                    &args![point, 0.0f32.to_bits(), 0.0f32.to_bits(), z],
                )
                .u32();
            let root = scroll_root(e, this);
            e.call(SET_LOCAL_TRANSLATE, &args![root, made]);
        });
        let lock = e.call(RENDER_LOCK_HOLDER, &args![]).u32();
        e.call(RENDER_LOCK, &args![lock]);
        let click = e.get(this, CreditsMenu::pClickRect);
        let parent = e.call(TILE_NODE_0056C7F0, &args![click]).u32();
        let root = scroll_root(e, this);
        e.vcall(parent, NODE_ATTACH_SLOT, &args![root, 1u32]);
        let block = e.call(NI_NEW, &args![0x1cu32]).u32();
        let property = if block != 0 {
            e.call(PROPERTY_CONSTRUCT_004391C0, &args![block]).u32()
        } else {
            0
        };
        e.call(PROPERTY_SET_0049ED90, &args![property, 1u32]);
        let root = scroll_root(e, this);
        e.call(ATTACH_PROPERTY, &args![root, property]);
        let mut font = current_font(e, this);
        e.call(STRING_CONSTRUCT, &args![text]);
        while e.get(this, CreditsMenu::iFileOffset) < e.get(this, CreditsMenu::iFileSize) {
            if text_byte(e, this, 0) == b'\r' {
                next_line(e, this, font);
            }
            while text_byte(e, this, 0) != b'\n'
                && e.get(this, CreditsMenu::iFileOffset) < e.get(this, CreditsMenu::iFileSize)
            {
                let byte = text_byte(e, this, 0);
                if byte == b'*' {
                    fn_00760b40(e, this);
                } else if byte == b'<' {
                    advance(e, this, 1);
                    credits_tag(e, this, text, &mut font);
                } else if byte < 0x20 {
                    advance(e, this, 1);
                } else {
                    fn_00760bd0(e, this, text);
                    let shape = e.with_stack(4, |e, width| {
                        e.mem.set_u32(width.addr(), 0x500);
                        let justification = e.get(this, CreditsMenu::iJustification);
                        let zero = 0.0f32.to_bits();
                        e.call(
                            FONT_MAKE_STRING,
                            &args![
                                font,
                                zero,
                                zero,
                                zero,
                                text,
                                width,
                                justification,
                                base.wrapping_add(0x50),
                                1u32,
                                1u32
                            ],
                        )
                        .u32()
                    });
                    let x = x87_store32(f64::from(e.get(this, CreditsMenu::iX)));
                    let y = x87_store32(f64::from(e.get(this, CreditsMenu::iY)));
                    e.call(
                        SET_TRANSLATE_004BC1F0,
                        &args![shape, x, 0.0f32.to_bits(), y],
                    );
                    let root = scroll_root(e, this);
                    e.vcall(root, NODE_ATTACH_SLOT, &args![shape, 1u32]);
                    next_line(e, this, font);
                }
            }
            fn_00760b40(e, this);
            advance(e, this, 1);
        }
        let data = e.get(this, CreditsMenu::pFileData);
        e.call(OPERATOR_DELETE, &args![data]);
        e.set(this, CreditsMenu::pFileData, 0);
        let root = scroll_root(e, this);
        e.call(PREPARE_OBJECT, &args![root, 0u32, 0u32]);
        let root = scroll_root(e, this);
        e.call(ROOT_UPDATE_00A29680, &args![root]);
        update_node(e, |e| scroll_root(e, this));
        let lock = e.call(RENDER_LOCK_HOLDER, &args![]).u32();
        e.call(RENDER_UNLOCK, &args![lock]);
        e.call(STRING_DESTROY, &args![text]);
        close_files(e);
    });
}

// Translated from 00760b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the credits menu): moves the read position to the next `\n` (or the
/// end of the text).
pub fn fn_00760b40(e: &mut Engine, this: Ptr<CreditsMenu>) {
    while text_byte(e, this, 0) != b'\n'
        && e.get(this, CreditsMenu::iFileOffset) < e.get(this, CreditsMenu::iFileSize)
    {
        advance(e, this, 1);
    }
}

// Translated from 00760b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on the credits menu): moves the read position past the next `>` (or to
/// the end of the text; it always moves at least once).
pub fn fn_00760b80(e: &mut Engine, this: Ptr<CreditsMenu>) {
    loop {
        let byte = text_byte(e, this, 0);
        advance(e, this, 1);
        if byte == b'>'
            || e.get(this, CreditsMenu::iFileOffset) >= e.get(this, CreditsMenu::iFileSize)
        {
            return;
        }
    }
}

// Translated from 00760bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CreditsMenu::GetString` (Xbox PDB; `thiscall`, `RET 4`: a `BSStringT`): copies the text
/// from the read position up to `\r`, `<`, `>`, `*` or the end into a 0x108-byte buffer on the
/// stack (the code does not bound the copy), moving the read position, and sets the string to
/// it (`004037f0`).
// Translation incomplete: 00760bd0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn fn_00760bd0(e: &mut Engine, this: Ptr<CreditsMenu>, string: u32) {
    e.with_stack(0x108, |e, buffer| {
        let mut byte = text_byte(e, this, 0);
        let mut length = 0u32;
        while byte != b'\r'
            && byte != b'<'
            && byte != b'>'
            && byte != b'*'
            && e.get(this, CreditsMenu::iFileOffset) < e.get(this, CreditsMenu::iFileSize)
        {
            e.mem.set_u8(buffer.addr().wrapping_add(length), byte);
            length = length.wrapping_add(1);
            advance(e, this, 1);
            if e.get(this, CreditsMenu::iFileOffset) < e.get(this, CreditsMenu::iFileSize) {
                byte = text_byte(e, this, 0);
            }
        }
        e.mem.set_u8(buffer.addr().wrapping_add(length), 0);
        e.call(STRING_SET, &args![string, buffer, 0u32]);
    });
}

/// Copies the three words of a point the constructor at `made` built into `target`.
fn copy_words(e: &mut Engine, made: u32, target: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(made.wrapping_add(4 * i));
        e.mem.set_u32(target.wrapping_add(4 * i), word);
    }
}

// Translated from 00760cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CreditsMenu::CreateImage` (Xbox PDB; `thiscall`, `RET 0x10`: name string, width, height,
/// alpha): builds a textured quad: four vertices (y 0.01; x 0 or the width, z 0 or minus the
/// height) and texture coordinates in `NiAlloc`'d arrays, two triangles, an `NiNode` placed at
/// (`iX` moved by the justification, 0, `iY`) with a property, the `NiTriShape` named and
/// attached, `Textures\Interface\Credits\<name>` as a `TileShaderProperty` with the alpha, a
/// material, its bound computed and the node prepared. Returns the node.
// Translation incomplete: 00760cf0 - the C++ exception frame (FS:[0], unwind states for the allocations and fixed strings) and the /GS stack cookie check (00ec408c) are not translated.
pub fn credits_menu_create_image(
    e: &mut Engine,
    this: Ptr<CreditsMenu>,
    name: u32,
    width: i32,
    height: i32,
    alpha: f32,
) -> u32 {
    let block = e.call(ALLOCATE, &args![0x30u32]).u32();
    let vertices = if block != 0 {
        e.call(VECTOR_CONSTRUCTOR, &args![block, 0xcu32, 4u32, NODE_ITEM]);
        block
    } else {
        0
    };
    let block = e.call(ALLOCATE, &args![0x20u32]).u32();
    let coordinates = if block != 0 {
        e.call(VECTOR_CONSTRUCTOR, &args![block, 8u32, 4u32, NODE_ITEM]);
        block
    } else {
        0
    };
    let indices = e.call(NI_ALLOC, &args![0xcu32]).u32();
    let y = x87_quiet_bits(e.global::<u32>(IMAGE_Y_01013EA4));
    let zero = 0.0f32.to_bits();
    let wide = x87_store32(f64::from(width));
    let low = x87_store32(f64::from(height.wrapping_neg()));
    for (i, (x, z)) in [(zero, zero), (zero, low), (wide, zero), (wide, low)]
        .into_iter()
        .enumerate()
    {
        e.with_stack(0xc, |e, point| {
            let made = e.call(POINT3_CONSTRUCT, &args![point, x, y, z]).u32();
            copy_words(e, made, vertices.wrapping_add(0xc * i as u32), 3);
        });
    }
    let one = 1.0f32.to_bits();
    for (i, (u, v)) in [(zero, zero), (zero, one), (one, zero), (one, one)]
        .into_iter()
        .enumerate()
    {
        e.with_stack(8, |e, point| {
            let made = e.call(POINT2_CONSTRUCT, &args![point, u, v]).u32();
            copy_words(e, made, coordinates.wrapping_add(8 * i as u32), 2);
        });
    }
    for (i, index) in [0u16, 1, 2, 2, 1, 3].into_iter().enumerate() {
        e.mem.set_u16(indices.wrapping_add(2 * i as u32), index);
    }
    let block = e.call(NI_NEW, &args![0xacu32]).u32();
    let node = if block != 0 {
        e.call(NI_NODE_CONSTRUCT, &args![block, 0u32]).u32()
    } else {
        0
    };
    let mut x = e.get(this, CreditsMenu::iX);
    if e.get(this, CreditsMenu::iJustification) == 2 {
        x = x.wrapping_sub(width / 2);
    } else if e.get(this, CreditsMenu::iJustification) == 4 {
        x = x.wrapping_sub(width);
    }
    let x = x87_store32(f64::from(x));
    let y_position = x87_store32(f64::from(e.get(this, CreditsMenu::iY)));
    e.call(SET_TRANSLATE_004BC1F0, &args![node, x, zero, y_position]);
    let block = e.call(NI_NEW, &args![0x1cu32]).u32();
    let property = if block != 0 {
        e.call(PROPERTY_CONSTRUCT_004391C0, &args![block]).u32()
    } else {
        0
    };
    e.call(PROPERTY_SET_0049ED90, &args![property, 1u32]);
    e.call(ATTACH_PROPERTY, &args![node, property]);
    let block = e.call(NI_NEW, &args![0xc4u32]).u32();
    let shape = if block != 0 {
        e.call(
            TRI_SHAPE_CONSTRUCT,
            &args![
                block,
                4u32,
                vertices,
                0u32,
                0u32,
                coordinates,
                1u32,
                0u32,
                2u32,
                indices
            ],
        )
        .u32()
    } else {
        0
    };
    e.vcall(node, NODE_ATTACH_SLOT, &args![shape, 1u32]);
    // The locals: the empty fixed string (+0), two temporaries (+4, +8) and the path (+0xc).
    e.with_stack(0x110, |e, locals| {
        let empty = locals.addr();
        let first = empty.wrapping_add(4);
        let second = empty.wrapping_add(8);
        let path = empty.wrapping_add(0xc);
        fn_007613f0(e, Ptr::new(empty));
        let text = first_word(e, name);
        e.call(PRINT_BOUNDED, &args![path, 0x104u32, TEXTURE_FORMAT, text]);
        let fixed = e.call(FIXED_STRING_CONSTRUCT, &args![first, path]).u32();
        e.call(OBJECT_SET_NAME_00A5B950, &args![shape, fixed]);
        e.call(FIXED_STRING_DESTRUCT, &args![first]);
        let fixed = e.call(FIXED_STRING_CONSTRUCT, &args![second, path]).u32();
        let texture = e
            .call(
                SOURCE_TEXTURE_CREATE,
                &args![fixed, TEXTURE_FORMAT_011A9598, 1u32, 0u32],
            )
            .u32();
        e.call(FIXED_STRING_DESTRUCT, &args![second]);
        let block = e.call(NI_NEW, &args![0xb0u32]).u32();
        let shader = if block != 0 {
            e.call(TILE_SHADER_PROPERTY_CONSTRUCT, &args![block, 0u32])
                .u32()
        } else {
            0
        };
        e.call(TILE_SHADER_SET_TEXTURE, &args![shader, texture]);
        e.call(
            SHADER_SET_ALPHA_00821600,
            &args![shader, x87_quiet_bits(alpha.to_bits())],
        );
        e.call(ATTACH_PROPERTY, &args![shape, shader]);
        let block = e.call(NI_NEW, &args![0x4cu32]).u32();
        let material = if block != 0 {
            e.call(MATERIAL_PROPERTY_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        e.call(MATERIAL_SET_004BC450, &args![material, MATERIAL_COLOR]);
        e.call(MATERIAL_SET_004534F0, &args![material, MATERIAL_COLOR]);
        e.call(ATTACH_PROPERTY, &args![shape, material]);
        let bound = e.call(MODEL_BOUND_00537B10, &args![shape]).u32();
        e.call(BOUND_COMPUTE, &args![bound, 4u32, vertices]);
        e.call(PREPARE_OBJECT, &args![node, 0u32, 1u32]);
        e.call(FIXED_STRING_DESTRUCT, &args![empty]);
    });
    node
}

// Translated from 00761410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments; the message box callback `Close` gives): closes the credits
/// (`0075f600`), clears the start menu's flag 0x40000000 and, when the answer is 0 and a save
/// entry was offered, clears `011dcfa5` (`00761460`), runs `008304f0` and loads the save.
pub fn fn_00761410(e: &mut Engine) {
    fn_0075f600(e);
    let start = e.global::<u32>(START_MENU);
    fn_0075f6f0(e, Ptr::new(start), 0x4000_0000, 0);
    if e.call(MESSAGE_RESULT, &args![]).u8() == 0 && e.global::<u32>(OFFERED_ENTRY_011D9474) != 0 {
        fn_00761460(e, 0);
        e.call(BEFORE_LOAD_008304F0, &args![]);
        let entry = e.global::<u32>(OFFERED_ENTRY_011D9474);
        e.call(ENTRY_LOAD_GAME, &args![entry]);
    }
}

// Translated from 00761470 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the `BSSimpleArray<NiFixedString,1024>` instance (unnamed in the engine
/// map; `thiscall`): its vtable, then `007616d0(0, 0)`. Returns `this`.
pub fn fn_00761470(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_01072514);
    fn_007616d0(e, this, 0, 0);
    this
}

// Translated from 007614c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The append of the `BSSimpleArray<NiFixedString,1024>` instance (unnamed in the engine map;
/// `thiscall`, `RET 4`: the string to copy): reserves a slot (`00761540`), constructs it
/// (`00761630(slot, 1)`) and assigns the string (`0043ba10`). Returns the slot's index.
pub fn fn_007614c0(e: &mut Engine, this: Ptr<BSSimpleArray>, value: u32) -> u32 {
    let index = fn_00761540(e, this);
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    e.call(
        ARRAY_CONSTRUCT_ELEMENTS,
        &args![this, buffer.wrapping_add(index.wrapping_mul(4)), 1u32],
    );
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    e.call(
        FIXED_STRING_ASSIGN,
        &args![buffer.wrapping_add(index.wrapping_mul(4)), value],
    );
    index
}

// Translated from 00761510 (decompiled, FalloutNV.exe 1.4.0.525)
/// The deleting destructor of the `BSSimpleArray<NiFixedString,1024>` instance (engine map;
/// `thiscall`, `RET 4`): the destructor (`007614a0`), then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_00761510(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_007614a0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 007615d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiFixedString,1024>::_DestructItems` (engine map; `thiscall`, `RET 4`: whether
/// to free the buffer): with a buffer, destroys its elements (`00761760`); when asked, frees
/// it (`006a8500`) and clears the buffer and the reserved size; then clears the size.
pub fn fn_007615d0(e: &mut Engine, this: Ptr<BSSimpleArray>, free: u8) {
    if e.get(this, BSSimpleArray::pBuffer) == 0 {
        return;
    }
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let size = e.get(this, BSSimpleArray::iSize);
    fn_00761760(e, this, buffer, size);
    if free != 0 {
        e.call(ARRAY_FREE_BUFFER, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, 0);
        e.set(this, BSSimpleArray::iReservedSize, 0);
    }
    e.set(this, BSSimpleArray::iSize, 0);
}

// Translated from 007616d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The initializer of the `BSSimpleArray<NiFixedString,1024>` instance (unnamed in the engine
/// map; `thiscall`, `RET 8`: capacity, size): clears the array; reserves the larger of the
/// two through virtual slot +4 and constructs `size` elements (`00761630`).
pub fn fn_007616d0(e: &mut Engine, this: Ptr<BSSimpleArray>, capacity: u32, size: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let capacity = if size > capacity { size } else { capacity };
    if capacity != 0 {
        let buffer = e.vcall(this.addr(), 4, &args![capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    }
    if size != 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(ARRAY_CONSTRUCT_ELEMENTS, &args![this, buffer, size]);
        e.set(this, BSSimpleArray::iSize, size);
    }
}

// Translated from 00761760 (decompiled, FalloutNV.exe 1.4.0.525)
/// The element destructor of the `BSSimpleArray<NiFixedString,1024>` instance (unnamed in the
/// engine map; `thiscall`, `RET 8`: buffer, count; `this` unread): `004b9900(0)` on each of the
/// `count` fixed strings.
pub fn fn_00761760(e: &mut Engine, _this: Ptr<BSSimpleArray>, buffer: u32, count: u32) {
    for i in 0..count {
        e.call(
            ELEMENT_DESTRUCT,
            &args![buffer.wrapping_add(i.wrapping_mul(4)), 0u32],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0075_f1c0, credits_menu_get_class(Ptr) -> u32),
        entry!(
            0x0075_f1d0,
            credits_menu_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0075_f200, credits_menu_destructor(Ptr)),
        entry!(0x0075_f2a0, credits_menu_create() -> bool),
        entry!(0x0075_f5f0, fn_0075f5f0() -> u32),
        entry!(0x0075_f600, fn_0075f600()),
        entry!(0x0075_f750, credits_menu_close()),
        entry!(0x0075_f8c0, fn_0075f8c0(Ptr) -> u32),
        entry!(
            0x0075_f8e0,
            credits_menu_attach_tile_by_id(Ptr<CreditsMenu>, i32, u32)
        ),
        entry!(0x0075_f910, credits_menu_do_idle(Ptr<CreditsMenu>)),
        entry!(0x0075_fb30, credits_menu_do_gamepad(Ptr, i32, u32) -> bool),
        entry!(
            0x0075_fb90,
            credits_menu_init_credits_root(Ptr<CreditsMenu>)
        ),
        entry!(0x0076_0b40, fn_00760b40(Ptr<CreditsMenu>)),
        entry!(0x0076_0b80, fn_00760b80(Ptr<CreditsMenu>)),
        entry!(0x0076_0bd0, fn_00760bd0(Ptr<CreditsMenu>, u32)),
        entry!(
            0x0076_0cf0,
            credits_menu_create_image(Ptr<CreditsMenu>, u32, i32, i32, f32) -> u32
        ),
        entry!(0x0076_1410, fn_00761410()),
        entry!(
            0x0076_1470,
            fn_00761470(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x0076_14c0, fn_007614c0(Ptr<BSSimpleArray>, u32) -> u32),
        entry!(0x0076_1510, fn_00761510(Ptr, u32) -> Ptr),
        entry!(0x0076_15d0, fn_007615d0(Ptr<BSSimpleArray>, u8)),
        entry!(0x0076_16d0, fn_007616d0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x0076_1760, fn_00761760(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x0076_1540, fn_00761540(Ptr<BSSimpleArray>) -> u32),
        entry!(0x0076_13f0, fn_007613f0(Ptr) -> Ptr),
        entry!(0x0076_14a0, fn_007614a0(Ptr)),
        entry!(0x0075_f6f0, fn_0075f6f0(Ptr, u32, u8)),
        entry!(0x0076_1460, fn_00761460(u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_word_becomes_the_empty_string_handle() {
        let mut e = Engine::new();
        e.map(EMPTY_FIXED_STRING & !0xfff, 0x1000);
        e.set_global(EMPTY_FIXED_STRING, 0x0123_4568u32);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0xdead_beef);
        assert_eq!(e.call(0x0076_13f0, &args![object]).u32(), object);
        assert_eq!(e.mem.u32(object), 0x0123_4568);
    }
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_007614a0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0076_14a0, ARRAY_VTABLE_01072514, ARRAY_RELEASE_007615D0);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn rig(vtable: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.put_vtable(vtable, &[0, 0x0f00_7000]);
        e.register(0x0f00_7000, |_, a| (0x3000_0000 + a[1]).into_ret());
        e.register(ARRAY_IS_FULL, |e, a| {
            u32::from(e.mem.u32(a[0] + 8) == e.mem.u32(a[0] + 0xc)).into_ret()
        });
        e.register(ARRAY_NEXT_CAPACITY, |_, _| 0x40u32.into_ret());
        e.register(ARRAY_REALLOCATE, |_, _| Ret::default());
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, vtable);
        (e, array)
    }

    #[test]
    fn at_00761540_allocates_four_elements_for_an_array_without_capacity() {
        let (mut e, array) = rig(0x0f00_6100);
        assert_eq!(e.call(0x0076_1540, &args![array]).u32(), 0);
        assert_eq!(calls(&e, 0x0f00_7000), vec![vec![array, 4]]);
        assert_eq!(e.mem.u32(array + 4), 0x3000_0004);
        assert_eq!(e.mem.u32(array + 8), 1);
        assert_eq!(e.mem.u32(array + 0xc), 4);
        assert!(calls(&e, ARRAY_NEXT_CAPACITY).is_empty());
    }

    #[test]
    fn at_00761540_reallocates_a_full_array_to_the_next_capacity() {
        let (mut e, array) = rig(0x0f00_6200);
        e.mem.set_u32(array + 4, 0x4000);
        e.mem.set_u32(array + 8, 4);
        e.mem.set_u32(array + 0xc, 4);
        assert_eq!(e.call(0x0076_1540, &args![array]).u32(), 4);
        assert_eq!(calls(&e, ARRAY_NEXT_CAPACITY), vec![vec![array]]);
        assert_eq!(calls(&e, ARRAY_REALLOCATE), vec![vec![array, 0x40, 4]]);
        assert_eq!(e.mem.u32(array + 8), 5);
        assert_eq!(e.mem.u32(array + 0xc), 0x40);
        // The buffer pointer is the reallocation's business, not this function's.
        assert_eq!(e.mem.u32(array + 4), 0x4000);
    }

    #[test]
    fn at_00761540_only_counts_when_there_is_room() {
        let (mut e, array) = rig(0x0f00_6300);
        e.mem.set_u32(array + 4, 0x4000);
        e.mem.set_u32(array + 8, 2);
        e.mem.set_u32(array + 0xc, 8);
        assert_eq!(e.call(0x0076_1540, &args![array]).u32(), 2);
        assert_eq!(e.mem.u32(array + 8), 3);
        assert_eq!(e.mem.u32(array + 0xc), 8);
        assert!(calls(&e, ARRAY_NEXT_CAPACITY).is_empty());
        assert!(calls(&e, ARRAY_REALLOCATE).is_empty());
        assert!(calls(&e, 0x0f00_7000).is_empty());
    }
}

#[cfg(test)]
mod b069_tests {
    use super::*;

    #[test]
    fn fn_0075f6f0_sets_or_clears_the_mask() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1b0);
        e.mem.set_u32(this + 0x1a8, 0x0f0f);
        e.call(0x0075_f6f0, &args![this, 0x00f0u32, 0x1234_5601u32]);
        assert_eq!(e.mem.u32(this + 0x1a8), 0x0fff);
        // Only the low byte of the flag counts.
        e.call(0x0075_f6f0, &args![this, 0x0f00u32, 0x1234_5600u32]);
        assert_eq!(e.mem.u32(this + 0x1a8), 0x00ff);
    }

    #[test]
    fn fn_00761460_stores_the_low_byte() {
        let mut e = Engine::new();
        e.map(FLAG_011DCFA5 & !0xfff, 0x1000);
        e.mem.set_u32(FLAG_011DCFA5 - 1, 0xaaaa_aaaa);
        e.call(0x0076_1460, &args![0x0000_0101u32]);
        assert_eq!(e.mem.u32(FLAG_011DCFA5 - 1), 0xaaaa_01aa);
    }
}

#[cfg(test)]
mod n053_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The callees of this batch's functions outside this file, stubbed to return 0.
    const FOREIGN: &[u32] = &[
        TILE_GET_MENU_BY_CLASS,
        TILE_GET_MENU,
        TILE_GET_MAXIMUM_DEPTH,
        TILE_READ_FILE,
        TILE_READER,
        INTERFACE_MANAGER,
        INTERFACE_FLAG_007023A0,
        MENU_SET_MENU_TILE,
        MENU_START_FADE_OUT,
        MENU_DESTRUCTOR,
        TILE_GET_FLOAT,
        TILE_SET_FLOAT,
        TILE_SET_INT,
        TILE_SET_STRING,
        PRINT_MESSAGE,
        RT_DYNAMIC_CAST,
        MENU_VALID_0073AF60,
        FIRST_WORD,
        WORD_AT_FOUR,
        WORD_AT_EIGHT,
        NODE_ITEM,
        ARRAY_ELEMENT,
        FIXED_STRING_TEXT,
        SETTING_FLOAT_ADDRESS,
        SETTING_STRING,
        SETTING_STRING_004C69F0,
        LOCAL_TRANSLATE,
        SET_LOCAL_TRANSLATE,
        UPDATE_DATA_CONSTRUCT,
        NODE_UPDATE,
        WORLD_BOUND,
        TILE_NODE_0056C7F0,
        POINTER_ASSIGN,
        RENDER_LOCK_HOLDER,
        RENDER_LOCK,
        RENDER_UNLOCK,
        ROOT_TEST_0052AA80,
        ROOT_OBJECT_009611E0,
        START_MENU_END_CREDITS,
        POINTER_RELEASE,
        AUDIO_SET_0046DCE0,
        AUDIO_VALUE_00526AC0,
        START_MENU_FLAG,
        SAVES_REFRESH,
        ENTRY_TEST_00846B10,
        ENTRY_LOAD_DATA,
        ENTRY_LOAD_GAME,
        BEFORE_LOAD_008304F0,
        STRISTR,
        TEXT_LENGTH,
        MESSAGE_BOX,
        MESSAGE_RESULT,
        IS_TOP_MENU,
        IS_MENU_VISIBLE,
        MENU_SHOWN_007027B0,
        INPUT_TEST_00A23A50,
        INPUT_TEST_00A24660,
        CHECK_MENU_BUTTON,
        INPUT_KEY_00A24180,
        INPUT_MANAGER,
        FLOAT_AT_0XC,
        LOOKUP_FORM,
        CURRENT_LANGUAGE,
        LANGUAGE_FOLDER,
        COPY_BOUNDED,
        APPEND_BOUNDED,
        PRINT_BOUNDED,
        MEMSET,
        BS_FILE_CONSTRUCT,
        BS_FILE_DESTRUCT,
        BS_FILE_EXIST,
        BS_FILE_READ,
        ALLOCATE,
        OPERATOR_DELETE,
        NI_NEW,
        NI_ALLOC,
        NI_NODE_CONSTRUCT,
        PROPERTY_CONSTRUCT_004391C0,
        PROPERTY_SET_0049ED90,
        TRI_SHAPE_CONSTRUCT,
        TILE_SHADER_PROPERTY_CONSTRUCT,
        TILE_SHADER_SET_TEXTURE,
        SHADER_SET_ALPHA_00821600,
        MATERIAL_PROPERTY_CONSTRUCT,
        ATTACH_PROPERTY,
        MATERIAL_SET_004BC450,
        MATERIAL_SET_004534F0,
        MODEL_BOUND_00537B10,
        BOUND_COMPUTE,
        PREPARE_OBJECT,
        SET_TRANSLATE_004BC1F0,
        ROOT_UPDATE_00A29680,
        VECTOR_CONSTRUCTOR,
        POINT3_CONSTRUCT,
        POINT2_CONSTRUCT,
        FIXED_STRING_CONSTRUCT,
        FIXED_STRING_DESTRUCT,
        FIXED_STRING_ASSIGN,
        SOURCE_TEXTURE_CREATE,
        OBJECT_SET_NAME_00A5B950,
        ATOL,
        FONT_MANAGER,
        FONT_MANAGER_FONT,
        FONT_MAKE_STRING,
        FONT_LINE_HEIGHT,
        STRING_CONSTRUCT,
        STRING_DESTROY,
        STRING_SET,
        STRING_FORMAT,
        ARRAY_CONSTRUCT_ELEMENTS,
        ARRAY_FREE_BUFFER,
        ELEMENT_DESTRUCT,
        SCREEN_WIDTH_004DC200,
        SCREEN_HEIGHT_004DC1F0,
        ARRAY_IS_FULL,
        ARRAY_NEXT_CAPACITY,
        ARRAY_REALLOCATE,
    ];

    const PAGES: &[u32] = &[
        0x0101_1000,
        0x0101_2000,
        0x0101_3000,
        0x0101_d000,
        0x0101_e000,
        0x0107_0000,
        0x0107_2000,
        0x011c_3000,
        0x011d_9000,
        0x011d_a000,
        0x011d_c000,
        0x011d_e000,
        0x0109_b000,
    ];

    fn engine() -> Engine {
        let mut e = Engine::new();
        for &callee in FOREIGN {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            (value as i32).into_ret()
        });
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        for &page in PAGES {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO, 0.0f64);
        e.set_global(TWO, 2.0f64);
        e.set_global(BYTE_SCALE, 255.0f64);
        e.set_global(DEPTH_STACKING_A, 6006.0f64);
        e.set_global(DEPTH_STACKING_B, 102.0f64);
        e.set_global(ASPECT_4_3, 0.75f64);
        e.set_global(WIDE_SHIFT, 214.0f64);
        e.set_global(IMAGE_Y_01013EA4, 0.01f32.to_bits());
        e.set_global(ROOT_Z_01072438, (-1020.0f32).to_bits());
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn float_ret(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// An object whose vtable has the slots given (`(byte offset, target)`), all recorded.
    fn object(e: &mut Engine, vtable: u32, slots: &[(u32, u32)]) -> u32 {
        let mut table = vec![0u32; 0x100 / 4];
        for &(slot, target) in slots {
            table[(slot / 4) as usize] = target;
            e.register(target, |_, _| Ret::default());
        }
        e.put_vtable(vtable, &table);
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    fn new_credits(e: &mut Engine) -> Ptr<CreditsMenu> {
        Ptr::new(e.mem.alloc(0x80))
    }

    /// The start menu, with its flag word at +0x1a8.
    fn start_menu(e: &mut Engine) -> u32 {
        let start = e.mem.alloc(0x1b0);
        e.set_global(START_MENU, start);
        start
    }

    #[test]
    fn get_class_is_0x417() {
        let mut e = engine();
        assert_eq!(e.call(0x0075_f1c0, &args![0x1000u32]).u32(), 0x417);
        assert_eq!(e.call(0x0075_f5f0, &args![]).u32(), 0x417);
    }

    #[test]
    fn the_destructor_ends_the_credits_and_destroys_the_members() {
        let mut e = engine();
        e.set_global(AUDIO_OBJECT_011C3F3C, 0x4400u32);
        let this = new_credits(&mut e).addr();
        e.call(0x0075_f1d0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this), CREDITS_MENU_VTABLE);
        assert_eq!(calls(&e, AUDIO_SET_0046DCE0), vec![vec![0x4400, 0]]);
        assert!(calls(&e, START_MENU_END_CREDITS).is_empty());
        assert_eq!(e.mem.u32(this + BACKGROUNDS), ARRAY_VTABLE_01072514);
        assert_eq!(calls(&e, POINTER_RELEASE), vec![vec![this + 0x30]]);
        assert_eq!(calls(&e, MENU_DESTRUCTOR), vec![vec![this]]);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        let start = start_menu(&mut e);
        e.call(0x0075_f1d0, &args![this, 1u32]);
        assert_eq!(calls(&e, START_MENU_END_CREDITS), vec![vec![start]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
    }

    /// The credits file `Create` reads: a menu of class `class` whose `__RTDynamicCast` to
    /// the credits menu is `credits`. Returns the menu.
    fn credits_file(e: &mut Engine, class: u32, credits: u32) -> u32 {
        let menu = object(e, 0x0f20_0000, &[(0, 0x0f20_0100), (0x34, 0x0f20_0200)]);
        e.register_double(0x0f20_0200, move |_, _| class.into_ret());
        e.register(TILE_READ_FILE, |_, _| 0x6600u32.into_ret());
        e.register_double(TILE_GET_MENU, move |_, _| menu.into_ret());
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            (if a[0] == menu { credits } else { a[0] }).into_ret()
        });
        menu
    }

    /// A player whose slot +0x4a0 answers 0 (not the wacky credits).
    fn quiet_player(e: &mut Engine) {
        let mut table = vec![0u32; 0x4a4 / 4];
        table[0x4a0 / 4] = 0x0f28_0100;
        e.put_vtable(0x0f28_0000, &table);
        e.register(0x0f28_0100, |_, _| Ret::default());
        let player = e.mem.alloc(4);
        e.mem.set_u32(player, 0x0f28_0000);
        e.set_global(PLAYER, player);
    }

    /// Files that do not open (slot +0x40 answers 0), so `InitCreditsRoot` stops early.
    fn closed_files(e: &mut Engine) {
        quiet_player(e);
        e.put_vtable(0x0f21_0000, &[0; 0x44 / 4]);
        e.register_double(BS_FILE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x0f21_0000);
            Ret::default()
        });
        e.mem.set_u32(0x0f21_0040, 0x0f21_0100);
        e.register(0x0f21_0100, |_, _| Ret::default());
    }

    #[test]
    fn create_sets_up_the_backgrounds_and_the_wide_screen_shift() {
        let mut e = engine();
        let credits = new_credits(&mut e);
        credits_file(&mut e, 0x417, credits.addr());
        closed_files(&mut e);
        e.register(MENU_VALID_0073AF60, |_, _| 1u32.into_ret());
        e.register(TILE_GET_FLOAT, |_, _| float_ret(102.0));
        e.register(TILE_GET_MAXIMUM_DEPTH, |_, _| float_ret(9.0));
        e.register(INTERFACE_FLAG_007023A0, |_, _| 1u32.into_ret());
        e.register(WORD_AT_EIGHT, |_, _| 4u32.into_ret());
        e.set(credits, CreditsMenu::uiCreditsTotalHeight, 1000);
        e.set(credits, CreditsMenu::pBackgroundTile, 0x6200);
        let root = e.mem.alloc(4);
        e.mem.set_u32(credits.addr() + 0x30, root);
        let point = e.mem.alloc(0xc);
        e.mem.set_u32(point, 10.0f32.to_bits());
        e.mem.set_u32(point + 8, 200.0f32.to_bits());
        e.register_double(LOCAL_TRANSLATE, move |_, _| point.into_ret());
        let speed = e.mem.alloc(4);
        e.mem.set_u32(speed, 2.0f32.to_bits());
        e.register_double(SETTING_FLOAT_ADDRESS, move |_, _| speed.into_ret());
        e.register(ARRAY_ELEMENT, |_, a| (a[1] + 0x8800).into_ret());
        e.register(FIXED_STRING_TEXT, |_, a| (a[0] + 1).into_ret());
        e.register(SCREEN_WIDTH_004DC200, |_, _| 1280u32.into_ret());
        e.register(SCREEN_HEIGHT_004DC1F0, |_, _| 720u32.into_ret());
        e.register(TILE_NODE_0056C7F0, |_, _| 0x6700u32.into_ret());
        let moved = Rc::new(RefCell::new(Vec::new()));
        let seen = moved.clone();
        e.register_double(SET_LOCAL_TRANSLATE, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        assert!(e.call(0x0075_f2a0, &args![]).bool());
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![0x6600, TRAIT_DEPTH, 9.0f32.to_bits(), 1]]
        );
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![0x6600, TRAIT_0X1004, 1]]);
        assert_eq!(e.get(credits, CreditsMenu::fBackgroundSwapInterval), 100.0);
        assert_eq!(e.get(credits, CreditsMenu::fBackgroundSwapTimer), 100.0);
        assert_eq!(e.get(credits, CreditsMenu::uiBackgroundIndexCounter), 1);
        assert_eq!(
            calls(&e, TILE_SET_STRING),
            vec![vec![0x6200, TRAIT_IMAGE, 0x8801, 1]]
        );
        assert_eq!(moved.borrow().as_slice(), &[224.0f32.to_bits()]);
        assert_eq!(calls(&e, NODE_UPDATE)[0][0], 0x6700);
        assert_eq!(calls(&e, BS_FILE_DESTRUCT).len(), 4);
    }

    #[test]
    fn create_fails_on_another_class_or_an_invalid_menu() {
        let mut e = engine();
        let credits = new_credits(&mut e);
        let menu = credits_file(&mut e, 0x421, credits.addr());
        e.register(WORD_AT_FOUR, |_, _| 1u32.into_ret());
        assert!(!e.call(0x0075_f2a0, &args![]).bool());
        assert_eq!(calls(&e, 0x0f20_0100), vec![vec![menu, 1]]);
        let mut e = engine();
        credits_file(&mut e, 0x417, credits.addr());
        assert!(!e.call(0x0075_f2a0, &args![]).bool());
        assert_eq!(calls(&e, PRINT_MESSAGE), vec![vec![CREATION_FAILED]]);
    }

    #[test]
    fn fn_0075f600_hands_the_root_back_and_fades_out() {
        let mut e = engine();
        let start = start_menu(&mut e);
        fn_0075f600(&mut e);
        assert_eq!(e.mem.u32(start + 0x1a8), 0);
        e.register(TILE_GET_MENU_BY_CLASS, |_, _| 0x6600u32.into_ret());
        fn_0075f600(&mut e);
        assert_eq!(e.mem.u32(start + 0x1a8), 0x0200_0000);
        let menu = e.mem.alloc(0x40);
        e.mem.set_u32(menu + 0x30, 0x9900);
        e.register_double(TILE_GET_MENU, move |_, _| menu.into_ret());
        e.register(ROOT_TEST_0052AA80, |_, _| 1u32.into_ret());
        let holder = object(&mut e, 0x0f22_0000, &[(0xe8, 0x0f22_0100)]);
        e.register_double(ROOT_OBJECT_009611E0, move |_, _| holder.into_ret());
        e.register(RENDER_LOCK_HOLDER, |_, _| 0x4500u32.into_ret());
        e.call(0x0075_f600, &args![]);
        assert_eq!(calls(&e, RENDER_LOCK), vec![vec![0x4500]]);
        assert_eq!(calls(&e, ROOT_TEST_0052AA80), vec![vec![menu + 0x30, 0]]);
        assert_eq!(calls(&e, 0x0f22_0100), vec![vec![holder, 0x9900]]);
        assert_eq!(calls(&e, POINTER_ASSIGN), vec![vec![menu + 0x30, 0]]);
        assert_eq!(calls(&e, TILE_SET_INT), vec![vec![0x6600, TRAIT_0X1772, 1]]);
        assert_eq!(calls(&e, MENU_START_FADE_OUT), vec![vec![menu]]);
        assert_eq!(calls(&e, RENDER_UNLOCK), vec![vec![0x4500]]);
    }

    #[test]
    fn close_offers_a_matching_save_or_closes() {
        let mut e = engine();
        let start = start_menu(&mut e);
        e.mem.set_u32(start + 0x1a8, 0x4000_0000);
        e.call(0x0075_f750, &args![]);
        assert_eq!(e.mem.u32(start + 0x1a8), 0);
        assert_eq!(calls(&e, TILE_GET_MENU_BY_CLASS).len(), 1);
        // With the flag, a player name and a matching entry, the message box is offered.
        e.register(START_MENU_FLAG, |_, _| 1u32.into_ret());
        let saves = e.mem.alloc(0x40);
        e.set_global(SAVE_LOAD_MANAGER, saves);
        e.mem.set_u32(saves + 0x30, 0x9a00);
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry, 0x9b00);
        e.mem.set_u32(entry + 0xc, 1);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, entry);
        e.mem.set_u32(saves, node);
        e.register(TEXT_LENGTH, |_, _| 3u32.into_ret());
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(ENTRY_TEST_00846B10, |_, _| 1u32.into_ret());
        e.register(STRISTR, |_, _| 1u32.into_ret());
        e.register(SETTING_STRING, |_, a| (a[0] + 1).into_ret());
        e.register(SETTING_STRING_004C69F0, |_, a| (a[0] + 2).into_ret());
        e.call(0x0075_f750, &args![]);
        assert_eq!(e.global::<u32>(OFFERED_ENTRY_011D9474), entry);
        assert_eq!(calls(&e, STRISTR), vec![vec![0x9a00, 0x9b00]]);
        assert_eq!(
            calls(&e, MESSAGE_BOX),
            vec![vec![
                SETTING_LOAD_PROMPT + 2,
                0,
                0,
                0x0076_1410,
                0,
                0x12,
                0,
                0,
                SETTING_LOAD_YES + 1,
                SETTING_LOAD_NO + 1,
                0
            ]]
        );
        assert_eq!(calls(&e, ENTRY_LOAD_DATA), vec![vec![entry]]);
        assert_eq!(calls(&e, TILE_GET_MENU_BY_CLASS).len(), 1);
    }

    #[test]
    fn fn_0075f8c0_loads_the_entry_data() {
        let mut e = engine();
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry + 0xc, 0x77);
        assert_eq!(e.call(0x0075_f8c0, &args![entry]).u32(), 0x77);
        assert_eq!(calls(&e, ENTRY_LOAD_DATA), vec![vec![entry]]);
    }

    #[test]
    fn attach_tile_by_id_sets_the_two_tiles() {
        let mut e = engine();
        let this = new_credits(&mut e);
        e.call(0x0075_f8e0, &args![this, 1u32, 0x6100u32]);
        e.call(0x0075_f8e0, &args![this, 0u32, 0x6200u32]);
        e.call(0x0075_f8e0, &args![this, 2u32, 0x6300u32]);
        assert_eq!(e.get(this, CreditsMenu::pClickRect), 0x6100);
        assert_eq!(e.get(this, CreditsMenu::pBackgroundTile), 0x6200);
    }

    #[test]
    fn do_idle_pauses_scrolls_and_swaps_backgrounds() {
        let mut e = engine();
        let this = new_credits(&mut e);
        e.set(this, CreditsMenu::pClickRect, 0x6100);
        e.register(IS_MENU_VISIBLE, |_, _| 1u32.into_ret());
        e.call(0x0075_f910, &args![this]);
        assert_eq!(
            calls(&e, TILE_SET_INT),
            vec![vec![0x6100, TRAIT_DEPTH, 0], vec![0x6100, TRAIT_VISIBLE, 0]]
        );
        assert_eq!(e.global::<u8>(PAUSED_011D94FC), 1);
        e.register(IS_MENU_VISIBLE, |_, _| Ret::default());
        start_menu(&mut e);
        let root = e.mem.alloc(4);
        e.mem.set_u32(this.addr() + 0x30, root);
        e.register(WORD_AT_EIGHT, |_, _| 2u32.into_ret());
        e.set(this, CreditsMenu::fBackgroundSwapTimer, 0.25);
        e.set(this, CreditsMenu::fBackgroundSwapInterval, 7.0);
        e.register(FLOAT_AT_0XC, |_, a| {
            float_ret(if a[0] == FRAME_TIMER { 0.5 } else { 30.0 })
        });
        let point = e.mem.alloc(0xc);
        e.mem.set_u32(point + 8, 100.0f32.to_bits());
        e.register_double(LOCAL_TRANSLATE, move |_, _| point.into_ret());
        let speed = e.mem.alloc(4);
        e.mem.set_u32(speed, 40.0f32.to_bits());
        e.register_double(SETTING_FLOAT_ADDRESS, move |_, _| speed.into_ret());
        let moved = Rc::new(RefCell::new(Vec::new()));
        let seen = moved.clone();
        e.register_double(SET_LOCAL_TRANSLATE, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[1] + 8));
            Ret::default()
        });
        e.call(0x0075_f910, &args![this]);
        // Unpausing closes the menu (`Close` tests the start menu's flag).
        assert_eq!(e.global::<u8>(PAUSED_011D94FC), 0);
        assert_eq!(e.get(this, CreditsMenu::uiBackgroundIndexCounter), 1);
        assert_eq!(e.get(this, CreditsMenu::fBackgroundSwapTimer), 7.0);
        // The root rose by 0.5 * 40 to 120; twice the radius (60) is below it: closed
        // again, the second `Close` of the call.
        assert_eq!(moved.borrow().as_slice(), &[120.0f32.to_bits()]);
        assert_eq!(calls(&e, START_MENU_FLAG).len(), 2);
    }

    #[test]
    fn do_gamepad_closes_on_other_buttons() {
        let mut e = engine();
        start_menu(&mut e);
        assert!(!e.call(0x0075_fb30, &args![0x1000u32, 4u32, 0u32]).bool());
        assert!(!e.call(0x0075_fb30, &args![0x1000u32, 5u32, 0u32]).bool());
        e.register(IS_TOP_MENU, |_, _| 1u32.into_ret());
        assert!(e.call(0x0075_fb30, &args![0x1000u32, 5u32, 0u32]).bool());
        assert!(e
            .call(0x0075_fb30, &args![0x1000u32, (-2i32) as u32, 0u32])
            .bool());
        e.register(MENU_SHOWN_007027B0, |_, _| 1u32.into_ret());
        assert!(!e.call(0x0075_fb30, &args![0x1000u32, 5u32, 0u32]).bool());
        assert_eq!(calls(&e, MENU_SHOWN_007027B0)[0], vec![0x3e9]);
    }

    /// `atol` on the engine's memory: optional sign, then decimal digits.
    fn atol(e: &mut Engine, words: &[u32]) -> Ret {
        let mut at = words[0];
        let negative = match e.mem.u8(at) {
            b'-' => {
                at += 1;
                true
            }
            b'+' => {
                at += 1;
                false
            }
            _ => false,
        };
        let mut value = 0i32;
        while e.mem.u8(at).is_ascii_digit() {
            value = value * 10 + i32::from(e.mem.u8(at) - b'0');
            at += 1;
        }
        (if negative { -value } else { value }).into_ret()
    }

    #[test]
    fn init_credits_root_lays_out_the_credits_text() {
        let mut e = engine();
        let this = new_credits(&mut e);
        let text: &[u8] = b"<Y=-20><J=C>Hi\r\n*comment\r\n<C=255 000 000 255>\x01Yo\r\n";
        let length = text.len() as u32;
        let constructed = Rc::new(RefCell::new(Vec::new()));
        let record = constructed.clone();
        e.put_vtable(0x0f23_0000, &[0; 0x44 / 4]);
        e.mem.set_u32(0x0f23_0028, 0x0f23_0100);
        e.mem.set_u32(0x0f23_0040, 0x0f23_0200);
        e.register_double(0x0f23_0100, move |_, _| length.into_ret());
        e.register(0x0f23_0200, |_, _| 1u32.into_ret());
        e.register_double(BS_FILE_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0], 0x0f23_0000);
            record.borrow_mut().push(a[0]);
            Ret::default()
        });
        let files = constructed.clone();
        e.register_double(BS_FILE_EXIST, move |_, a| {
            u32::from(a[0] == files.borrow()[0]).into_ret()
        });
        let buffer = e.mem.alloc(length + 0x10);
        e.register_double(ALLOCATE, move |_, _| buffer.into_ret());
        e.register_double(BS_FILE_READ, move |e, a| {
            e.mem.write(a[1], text);
            a[2].into_ret()
        });
        e.register(INPUT_KEY_00A24180, |_, _| 1u32.into_ret());
        e.register(ATOL, atol);
        let root = object(&mut e, 0x0f24_0000, &[(0xdc, 0x0f24_0100)]);
        e.register_double(NI_NEW, move |_, _| root.into_ret());
        e.register(NI_NODE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let parent = object(&mut e, 0x0f25_0000, &[(0xdc, 0x0f25_0100)]);
        e.register_double(TILE_NODE_0056C7F0, move |_, _| parent.into_ret());
        e.register(FONT_MANAGER_FONT, |_, _| 0x5f00u32.into_ret());
        e.register(FONT_LINE_HEIGHT, |_, _| float_ret(15.0));
        e.register(FONT_MAKE_STRING, |_, _| 0x5e00u32.into_ret());
        let strings = Rc::new(RefCell::new(Vec::new()));
        let seen = strings.clone();
        e.register_double(STRING_SET, move |e, a| {
            seen.borrow_mut().push(e.mem.cstr(a[1]));
            Ret::default()
        });
        e.call(0x0075_fb90, &args![this]);
        // The wacky credits: the language file was found first.
        let paths: Vec<_> = calls(&e, BS_FILE_CONSTRUCT).iter().map(|w| w[1]).collect();
        assert_eq!(paths[1], DATA_WACKY_FILE);
        assert_eq!(paths[3], DATA_WACKY_PATCH_FILE);
        assert_eq!(calls(&e, 0x0f23_0200)[0][0], constructed.borrow()[0]);
        assert_eq!(e.get(this, CreditsMenu::iFileSize), length + 1);
        assert_eq!(e.get(this, CreditsMenu::pFileData), 0);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![buffer]]);
        assert_eq!(
            strings.borrow().as_slice(),
            &[b"Hi".to_vec(), b"Yo".to_vec()]
        );
        assert_eq!(e.get(this, CreditsMenu::iJustification), 2);
        // `-20` adds 20, each text line goes down 15.
        assert_eq!(e.get(this, CreditsMenu::iY), -10);
        assert_eq!(e.get(this, CreditsMenu::uiCreditsTotalHeight), 30);
        assert_eq!(e.mem.u32(this.addr() + 0x50), 1.0f32.to_bits());
        assert_eq!(e.mem.u32(this.addr() + 0x54), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x5c), 1.0f32.to_bits());
        let made = calls(&e, FONT_MAKE_STRING);
        assert_eq!(made.len(), 2);
        assert_eq!(made[0][..4], [0x5f00, 0, 0, 0]);
        assert_eq!(made[0][7..], [this.addr() + 0x50, 1, 1]);
        assert_eq!(made[0][6], 2);
        assert_eq!(
            calls(&e, SET_TRANSLATE_004BC1F0)[0],
            vec![0x5e00, 0, 0, 20.0f32.to_bits()]
        );
        assert_eq!(calls(&e, 0x0f24_0100).len(), 2);
        assert_eq!(calls(&e, 0x0f25_0100), vec![vec![parent, root, 1]]);
        assert_eq!(calls(&e, PREPARE_OBJECT), vec![vec![root, 0, 0]]);
        let closed: Vec<_> = calls(&e, BS_FILE_DESTRUCT).iter().map(|w| w[0]).collect();
        let opened = constructed.borrow();
        assert_eq!(closed, vec![opened[3], opened[2], opened[1], opened[0]]);
    }

    #[test]
    fn init_credits_root_stops_when_no_file_opens() {
        let mut e = engine();
        let this = new_credits(&mut e);
        closed_files(&mut e);
        e.call(0x0075_fb90, &args![this]);
        assert_eq!(calls(&e, BS_FILE_DESTRUCT).len(), 4);
        assert!(calls(&e, ALLOCATE).is_empty());
        let paths: Vec<_> = calls(&e, BS_FILE_CONSTRUCT).iter().map(|w| w[1]).collect();
        assert_eq!(paths[1], DATA_CREDITS_FILE);
        assert_eq!(paths[3], DATA_PATCH_FILE);
    }

    /// A credits menu reading `text`.
    fn reading(e: &mut Engine, text: &[u8]) -> Ptr<CreditsMenu> {
        let this = new_credits(e);
        let buffer = e.mem.alloc(text.len() as u32 + 0x10);
        e.mem.write(buffer, text);
        e.set(this, CreditsMenu::pFileData, buffer);
        e.set(this, CreditsMenu::iFileSize, text.len() as u32);
        this
    }

    #[test]
    fn the_read_position_skips_to_the_line_end_and_past_a_tag() {
        let mut e = engine();
        let this = reading(&mut e, b"ab>cd\nef");
        e.call(0x0076_0b40, &args![this]);
        assert_eq!(e.get(this, CreditsMenu::iFileOffset), 5);
        e.set(this, CreditsMenu::iFileOffset, 0);
        e.call(0x0076_0b80, &args![this]);
        assert_eq!(e.get(this, CreditsMenu::iFileOffset), 3);
        e.set(this, CreditsMenu::iFileOffset, 6);
        e.call(0x0076_0b80, &args![this]);
        assert_eq!(e.get(this, CreditsMenu::iFileOffset), 8);
        e.call(0x0076_0b40, &args![this]);
        assert_eq!(e.get(this, CreditsMenu::iFileOffset), 8);
    }

    #[test]
    fn get_string_reads_up_to_a_tag() {
        let mut e = engine();
        let this = reading(&mut e, b"Credits<B");
        let strings = Rc::new(RefCell::new(Vec::new()));
        let seen = strings.clone();
        e.register_double(STRING_SET, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.cstr(a[1]), a[2]));
            Ret::default()
        });
        e.call(0x0076_0bd0, &args![this, 0x7000u32]);
        assert_eq!(
            strings.borrow().as_slice(),
            &[(0x7000, b"Credits".to_vec(), 0)]
        );
        assert_eq!(e.get(this, CreditsMenu::iFileOffset), 7);
    }

    #[test]
    fn create_image_builds_a_textured_quad() {
        let mut e = engine();
        let this = new_credits(&mut e);
        e.set(this, CreditsMenu::iX, 100);
        e.set(this, CreditsMenu::iY, -40);
        e.set(this, CreditsMenu::iJustification, 2);
        let vertices = e.mem.alloc(0x30);
        let coordinates = e.mem.alloc(0x20);
        let mut blocks = [vertices, coordinates].into_iter();
        e.register_double(ALLOCATE, move |_, _| blocks.next().unwrap().into_ret());
        let indices = e.mem.alloc(0xc);
        e.register_double(NI_ALLOC, move |_, _| indices.into_ret());
        e.register(POINT3_CONSTRUCT, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            a[0].into_ret()
        });
        e.register(POINT2_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            a[0].into_ret()
        });
        let node = object(&mut e, 0x0f26_0000, &[(0xdc, 0x0f26_0100)]);
        let mut news = [node, 0x5100, 0x5200, 0x5300, 0x5400].into_iter();
        e.register_double(NI_NEW, move |_, _| news.next().unwrap().into_ret());
        e.register(NI_NODE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(PROPERTY_CONSTRUCT_004391C0, |_, a| a[0].into_ret());
        e.register(TRI_SHAPE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(TILE_SHADER_PROPERTY_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(MATERIAL_PROPERTY_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(SOURCE_TEXTURE_CREATE, |_, _| 0x5500u32.into_ret());
        e.register(MODEL_BOUND_00537B10, |_, _| 0x5600u32.into_ret());
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x7700);
        let made = e.call(0x0076_0cf0, &args![this, name, 50i32, 20i32, 0.5f32]);
        assert_eq!(made.u32(), node);
        let y = 0.01f32.to_bits();
        let wide = 50.0f32.to_bits();
        let low = (-20.0f32).to_bits();
        let corners: Vec<u32> = (0..12).map(|i| e.mem.u32(vertices + 4 * i)).collect();
        assert_eq!(corners, vec![0, y, 0, 0, y, low, wide, y, 0, wide, y, low]);
        let one = 1.0f32.to_bits();
        let uv: Vec<u32> = (0..8).map(|i| e.mem.u32(coordinates + 4 * i)).collect();
        assert_eq!(uv, vec![0, 0, 0, one, one, 0, one, one]);
        let order: Vec<u16> = (0..6).map(|i| e.mem.u16(indices + 2 * i)).collect();
        assert_eq!(order, vec![0, 1, 2, 2, 1, 3]);
        assert_eq!(
            calls(&e, SET_TRANSLATE_004BC1F0),
            vec![vec![node, 75.0f32.to_bits(), 0, (-40.0f32).to_bits()]]
        );
        assert_eq!(
            calls(&e, TRI_SHAPE_CONSTRUCT),
            vec![vec![
                0x5200,
                4,
                vertices,
                0,
                0,
                coordinates,
                1,
                0,
                2,
                indices
            ]]
        );
        assert_eq!(calls(&e, 0x0f26_0100), vec![vec![node, 0x5200, 1]]);
        assert_eq!(
            calls(&e, PRINT_BOUNDED)[0][1..],
            [0x104, TEXTURE_FORMAT, 0x7700]
        );
        assert_eq!(
            calls(&e, SOURCE_TEXTURE_CREATE)[0][1..],
            [TEXTURE_FORMAT_011A9598, 1, 0]
        );
        assert_eq!(
            calls(&e, TILE_SHADER_SET_TEXTURE),
            vec![vec![0x5300, 0x5500]]
        );
        assert_eq!(
            calls(&e, SHADER_SET_ALPHA_00821600),
            vec![vec![0x5300, 0.5f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, ATTACH_PROPERTY),
            vec![
                vec![node, 0x5100],
                vec![0x5200, 0x5300],
                vec![0x5200, 0x5400]
            ]
        );
        assert_eq!(calls(&e, BOUND_COMPUTE), vec![vec![0x5600, 4, vertices]]);
        assert_eq!(calls(&e, PREPARE_OBJECT), vec![vec![node, 0, 1]]);
        assert_eq!(calls(&e, FIXED_STRING_DESTRUCT).len(), 3);
    }

    #[test]
    fn the_message_answer_loads_the_offered_save() {
        let mut e = engine();
        let start = start_menu(&mut e);
        e.mem.set_u32(start + 0x1a8, 0x4000_0001);
        e.set_global(OFFERED_ENTRY_011D9474, 0x9c00u32);
        e.register(MESSAGE_RESULT, |_, _| 1u32.into_ret());
        e.call(0x0076_1410, &args![]);
        assert_eq!(e.mem.u32(start + 0x1a8), 1);
        assert!(calls(&e, ENTRY_LOAD_GAME).is_empty());
        e.register(MESSAGE_RESULT, |_, _| 0x100u32.into_ret());
        e.set_global(FLAG_011DCFA5, 1u8);
        e.call(0x0076_1410, &args![]);
        assert_eq!(e.global::<u8>(FLAG_011DCFA5), 0);
        assert_eq!(calls(&e, BEFORE_LOAD_008304F0).len(), 1);
        assert_eq!(calls(&e, ENTRY_LOAD_GAME), vec![vec![0x9c00]]);
    }

    #[test]
    fn the_background_array_constructor_append_and_destructors() {
        let mut e = engine();
        e.put_vtable(0x0f27_0000, &[0, 0x0f27_0100]);
        e.register(0x0f27_0100, |_, a| (0x4000 + a[1]).into_ret());
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, 0x0f27_0000);
        e.call(0x0076_16d0, &args![array, 2u32, 3u32]);
        assert_eq!(calls(&e, 0x0f27_0100), vec![vec![array, 3]]);
        assert_eq!(e.mem.u32(array + 4), 0x4003);
        assert_eq!(e.mem.u32(array + 8), 3);
        assert_eq!(e.mem.u32(array + 0xc), 3);
        assert_eq!(
            calls(&e, ARRAY_CONSTRUCT_ELEMENTS),
            vec![vec![array, 0x4003, 3]]
        );
        e.call(0x0076_16d0, &args![array, 0u32, 0u32]);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(calls(&e, 0x0f27_0100).len(), 1);
        // The constructor sets the vtable and an empty array.
        let made = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0076_1470, &args![made]).u32(), made);
        assert_eq!(e.mem.u32(made), ARRAY_VTABLE_01072514);
        // Append: the slot index is the old size.
        e.mem.set_u32(array + 4, 0x4000);
        e.mem.set_u32(array + 8, 1);
        e.mem.set_u32(array + 0xc, 4);
        assert_eq!(e.call(0x0076_14c0, &args![array, 0x9d00u32]).u32(), 1);
        assert_eq!(
            calls(&e, ARRAY_CONSTRUCT_ELEMENTS).last().unwrap(),
            &vec![array, 0x4004, 1]
        );
        assert_eq!(calls(&e, FIXED_STRING_ASSIGN), vec![vec![0x4004, 0x9d00]]);
        // `_DestructItems` with the buffer freed.
        e.call(0x0076_15d0, &args![array, 1u32]);
        assert_eq!(
            calls(&e, ELEMENT_DESTRUCT),
            vec![vec![0x4000, 0], vec![0x4004, 0]]
        );
        assert_eq!(calls(&e, ARRAY_FREE_BUFFER), vec![vec![array]]);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(e.mem.u32(array + 8), 0);
        assert_eq!(e.mem.u32(array + 0xc), 0);
        // Kept buffer: only the size is cleared.
        e.mem.set_u32(array + 4, 0x4000);
        e.mem.set_u32(array + 8, 1);
        e.mem.set_u32(array + 0xc, 4);
        e.call(0x0076_15d0, &args![array, 0u32]);
        assert_eq!(e.mem.u32(array + 4), 0x4000);
        assert_eq!(e.mem.u32(array + 0xc), 4);
        assert_eq!(e.mem.u32(array + 8), 0);
        // The deleting destructor.
        assert_eq!(e.call(0x0076_1510, &args![array, 1u32]).u32(), array);
        assert_eq!(e.mem.u32(array), ARRAY_VTABLE_01072514);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![array]]);
        e.call(0x0076_1760, &args![array, 0x4100u32, 1u32]);
        assert_eq!(
            calls(&e, ELEMENT_DESTRUCT).last().unwrap(),
            &vec![0x4100, 0]
        );
    }
}
