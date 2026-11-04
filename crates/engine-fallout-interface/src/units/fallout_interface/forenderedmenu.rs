//! `fallout/interface/forenderedmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `FORenderedMenu`'s virtual slots on PC sit 0xc above the Xbox PDB's: the
//! effect updates `OnIdle` calls are at +0x38 (`UpdateBurst`), +0x3c
//! (`UpdateVertHold`), +0x40 (`UpdateShudder`), +0x44 (`UpdatePulse`), +0x48
//! (`UpdateLight`) and +0x4c (`UpdateDistort`), each gated by its own
//! active flag as the PDB's fields say; the slot names are inferred from that
//! order, not confirmed.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `FORenderedMenu` (Xbox PDB), 0xd8 bytes there; the PC code uses the
    /// same field offsets.
    pub struct FORenderedMenu: 0xd8 {
        /// `spScreenTriShape` (Xbox PDB): `NiPointer<NiGeometry>`.
        0x04 spScreenTriShape: u32,
        /// `spDefaultTex` (Xbox PDB): `NiPointer<NiTexture>`.
        0x10 spDefaultTex: u32,
        /// `spMenuRoot` (Xbox PDB): `NiPointer<NiAVObject>`.
        0x14 spMenuRoot: u32,
        /// `spLight` (Xbox PDB): `NiPointer<NiPointLight>`.
        0x1C spLight: u32,
        /// `pLightShadowSceneNode` (Xbox PDB): `ShadowSceneNode*`.
        0x20 pLightShadowSceneNode: u32,
        /// `bInitialized` (Xbox PDB).
        0x28 bInitialized: u8,
        /// `bUsePartialScreenGeometry` (Xbox PDB).
        0x2A bUsePartialScreenGeometry: u8,
        /// `fScreenLightBaseIntensity` (Xbox PDB).
        0x38 fScreenLightBaseIntensity: f32,
        /// `fScreenLightFlickerIntensity` (Xbox PDB).
        0x3C fScreenLightFlickerIntensity: f32,
        /// `fBlurRadius` (Xbox PDB).
        0x40 fBlurRadius: f32,
        /// `fBlurIntensity` (Xbox PDB).
        0x44 fBlurIntensity: f32,
        /// `bBurstActive` (Xbox PDB).
        0x4D bBurstActive: u8,
        /// `fBurstStartTime` (Xbox PDB).
        0x50 fBurstStartTime: f32,
        /// `fBurstDuration` (Xbox PDB).
        0x54 fBurstDuration: f32,
        /// `fBurstMaximumIntensity` (Xbox PDB).
        0x58 fBurstMaximumIntensity: f32,
        /// `fBurstCurrentIntensity` (Xbox PDB).
        0x5C fBurstCurrentIntensity: f32,
        /// `bVertHoldActive` (Xbox PDB).
        0x60 bVertHoldActive: u8,
        /// `bShudderActive` (Xbox PDB).
        0x74 bShudderActive: u8,
        /// `fPulseCurrentBrighten` (Xbox PDB).
        0x8C fPulseCurrentBrighten: f32,
        /// `fPulseCurrentRadius` (Xbox PDB).
        0x90 fPulseCurrentRadius: f32,
        /// `bPulseAffectsLight` (Xbox PDB).
        0x94 bPulseAffectsLight: u8,
        /// `bDistortActive` (Xbox PDB).
        0x95 bDistortActive: u8,
        /// `fDistortStartTime` (Xbox PDB).
        0x98 fDistortStartTime: f32,
        /// `fDistortPosition` (Xbox PDB).
        0x9C fDistortPosition: f32,
        /// `fNextDistortTime` (Xbox PDB).
        0xA4 fNextDistortTime: f32,
        /// `bMenuOpen` (Xbox PDB).
        0xB8 bMenuOpen: u8,
        /// `bMenuRendering` (Xbox PDB).
        0xB9 bMenuRendering: u8,
        /// `bHidQuantity` (Xbox PDB).
        0xD0 bHidQuantity: u8,
        /// `bHidTutorial` (Xbox PDB).
        0xD1 bHidTutorial: u8,
        /// `bHidMessage` (Xbox PDB).
        0xD2 bHidMessage: u8,
    }
}

/// `ScreenLightColor` (Xbox PDB): the `NiColor` at +0x2c.
const SCREEN_LIGHT_COLOR: u32 = 0x2c;
/// `BackgroundColor` (Xbox PDB): the `NiColorA` at +0xa8.
const BACKGROUND_COLOR: u32 = 0xa8;
/// `spScanlinesTexture` (Xbox PDB) at +0xbc.
const SCANLINES_TEXTURE: u32 = 0xbc;
/// `spAccumulator` (Xbox PDB) at +0xd4.
const ACCUMULATOR: u32 = 0xd4;

/// `00559450` (thiscall on an `NiPointer`, returns the object; unnamed).
const POINTER_GET: u32 = 0x0055_9450;
/// `0066b0d0` (thiscall on an `NiPointer`, one word: assigns it; unnamed).
const POINTER_SET: u32 = 0x0066_b0d0;
/// `00633c90` (thiscall on an `NiPointer`, one word: constructs it; unnamed).
const POINTER_INIT: u32 = 0x0063_3c90;
/// `0045cec0` (thiscall on an `NiPointer`: destroys it; unnamed).
const POINTER_DESTROY: u32 = 0x0045_cec0;
/// `005b5e40` (cdecl, a format and its arguments: a debug message).
const DEBUG_MESSAGE: u32 = 0x005b_5e40;
/// `00438220` (no arguments: returns a property type word; unnamed).
const PROPERTY_TYPE_00438220: u32 = 0x0043_8220;
/// `NiAVObject::GetProperty` (`00a59d30`, thiscall: type).
const NI_AV_OBJECT_GET_PROPERTY: u32 = 0x00a5_9d30;
/// `00438230` (thiscall on the property, one word: sets its texture; unnamed).
const PROPERTY_SET_TEXTURE_00438230: u32 = 0x0043_8230;
/// `00825c00` (thiscall on `011f6394`: returns the time in milliseconds as the
/// callers use it; unnamed).
const TIMER_GET_00825C00: u32 = 0x0082_5c00;
/// The timer object `00825c00` is asked.
const TIMER: u32 = 0x011f_6394;
/// `004b7210` (no arguments, returns the interface object; unnamed).
const INTERFACE_GET_004B7210: u32 = 0x004b_7210;
/// `007079f0` (thiscall on the interface, returns `AL`; unnamed, `interface.cpp`).
const INTERFACE_TEST_007079F0: u32 = 0x0070_79f0;
/// `009611e0` (thiscall, returns a pointer; unnamed, `playercharacter.cpp`).
const GETTER_009611E0: u32 = 0x0096_11e0;
/// `Tile::GetMenu` (`00a03c90`).
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `00867a20` (thiscall on the menu; unnamed, `calendar.cpp` by the map's folding).
const MENU_00867A20: u32 = 0x0086_7a20;
/// `Interface::GetMenusRoot` (`00706cd0`).
const INTERFACE_GET_MENUS_ROOT: u32 = 0x0070_6cd0;
/// `Tile::UpdateClipwindows` (`00a079d0`).
const TILE_UPDATE_CLIPWINDOWS: u32 = 0x00a0_79d0;
/// `00450f90` (thiscall on an `NiAVObject`, one word: sets whether it is hidden).
const SET_APP_CULLED_00450F90: u32 = 0x0045_0f90;
/// `00456610` (thiscall on an `NiAVObject`, returns `AL`: whether it is hidden).
const IS_APP_CULLED_00456610: u32 = 0x0045_6610;
/// The `bool` `TriggerBurst` checks before its sound (`011db378`; perhaps the
/// PDB's `static bool bSoundsActive`, unconfirmed).
const SOUNDS_ACTIVE_011DB378: u32 = 0x011d_b378;
/// `004dff00` (cdecl, one `float`, returns `AL`; unnamed).
const FLOAT_TEST_004DFF00: u32 = 0x004d_ff00;
/// `0041a250` (thiscall: constructs a sound handle; unnamed).
const SOUND_HANDLE_INIT: u32 = 0x0041_a250;
/// `BSAudio::QInstance` (`00453a70`).
const BS_AUDIO_Q_INSTANCE: u32 = 0x0045_3a70;
/// `BSAudio::GetSoundHandleByName` (`00ad7550`, thiscall: handle, name, flags).
const BS_AUDIO_GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
/// `00418900` (thiscall, one word: assigns a sound handle; unnamed).
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
/// `00483710` (thiscall: the sound handle's destructor here; unnamed).
const SOUND_HANDLE_DESTROY: u32 = 0x0048_3710;
/// `BSSoundHandle::Play` (`00ad8830`, one word).
const BS_SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
/// `"UIPipBoyStatic"`.
const PIPBOY_STATIC_SOUND: u32 = 0x0107_84c0;
/// `00403e20` (thiscall on a setting, returns the address of its value; unnamed).
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `00c5d320` (cdecl, one `float`, returns `ST0`; unnamed, `BSMain`).
const FLOAT_FUNCTION_00C5D320: u32 = 0x00c5_d320;
/// `004e44d0` (cdecl, one `float`, returns `ST0`; unnamed).
const FLOAT_FUNCTION_004E44D0: u32 = 0x004e_44d0;
/// `00633b40` (thiscall on an `NiColor`: result, factor; `NiColor * float` by its use).
const COLOR_SCALE_00633B40: u32 = 0x0063_3b40;
/// `007fb730` (thiscall on an `NiColor`: result, other color; this unit, not translated).
const COLOR_COMBINE_007FB730: u32 = 0x007f_b730;
/// `004bc2e0` (thiscall on the light, one word: a color; unnamed).
const LIGHT_SET_COLOR_004BC2E0: u32 = 0x004b_c2e0;
/// `0043d410` (thiscall: constructs an update data block from a `float` and two words).
const UPDATE_DATA_INIT_0043D410: u32 = 0x0043_d410;
/// `00a59c60` (thiscall on an `NiAVObject`, one word: an update data block; unnamed).
const UPDATE_00A59C60: u32 = 0x00a5_9c60;
/// The three words `fn_007fb5a0` starts its light color from (`011f4998`).
const BASE_LIGHT_COLOR: u32 = 0x011f_4998;
/// `0.5` (`double`).
const HALF: u32 = 0x0101_1588;
/// `0.63` (`double`).
const PICK_U_SCALE: u32 = 0x0107_84e0;
/// `0.37` (`double`).
const PICK_U_OFFSET: u32 = 0x0107_84d8;
/// `0.65` (`double`).
const PICK_V_SCALE: u32 = 0x0107_4b98;
/// `0.34` (`double`).
const PICK_V_OFFSET: u32 = 0x0107_84d0;
/// `1.333333` (`double`).
const ASPECT: u32 = 0x0107_8128;
/// `1280.0` (`float`).
const FRUSTUM_RIGHT: u32 = 0x0106_ec38;
/// `960.0` (`float`).
const FRUSTUM_TOP: u32 = 0x0106_f2dc;
/// `10000.0` (`float`).
const FRUSTUM_FAR: u32 = 0x0102_2958;
/// `006815c0` (thiscall: constructs an `NiPoint3` here; returns `this`).
const POINT_INIT_006815C0: u32 = 0x0068_15c0;
/// `NiPick::NiPick` (`00e98f20`, thiscall: two words).
const NI_PICK_CONSTRUCT: u32 = 0x00e9_8f20;
/// `NiPick::~NiPick` (`00e98fa0`).
const NI_PICK_DESTROY: u32 = 0x00e9_8fa0;
/// `NiPick::PickObjects` (`00e98e20`, thiscall: origin, direction, word).
const NI_PICK_PICK_OBJECTS: u32 = 0x00e9_8e20;
/// `008b6200` (thiscall on the menu, returns a camera; unnamed, `actor.cpp`).
const CAMERA_008B6200: u32 = 0x008b_6200;
/// `0045c670` (no arguments, returns a pointer; unnamed, `tes.cpp`).
const GETTER_0045C670: u32 = 0x0045_c670;
/// `006629f0` (thiscall, returns a pointer; named `BSFaceGenNiNode::GetAnimationData`
/// by folded code).
const GETTER_006629F0: u32 = 0x0066_29f0;
/// `00458b30` (thiscall on the pick, one word; unnamed).
const PICK_00458B30: u32 = 0x0045_8b30;
/// `00705fc0` (thiscall on the pick, one word: the target; unnamed).
const PICK_SET_TARGET_00705FC0: u32 = 0x0070_5fc0;
/// `006ecd40` (thiscall on the pick, one word; unnamed).
const PICK_006ECD40: u32 = 0x006e_cd40;
/// `00792780` (thiscall on the pick, one word; unnamed).
const PICK_00792780: u32 = 0x0079_2780;
/// `00792760` (thiscall on the interface, returns a `float` in `ST0`; unnamed).
const INTERFACE_REAL_00792760: u32 = 0x0079_2760;
/// `009a9350` (thiscall on the interface, returns a `float` in `ST0`; unnamed).
const INTERFACE_REAL_009A9350: u32 = 0x009a_9350;
/// `_ftol2` (`00ec62c0`): the `ST0` value as an integer (a leading `f64` argument).
const FTOL: u32 = 0x00ec_62c0;
/// `00a71080` (thiscall on the camera, five words: the pick ray; `nicamera.cpp`).
const CAMERA_WINDOW_POINT_TO_RAY_00A71080: u32 = 0x00a7_1080;
/// `00500940` (thiscall on the pick, returns its results; unnamed).
const PICK_RESULTS_00500940: u32 = 0x0050_0940;
/// `00968670` (thiscall on the results, one word: an index; unnamed).
const PICK_RECORD_00968670: u32 = 0x0096_8670;
/// `00458b50` (thiscall on a record, returns its object; unnamed).
const RECORD_OBJECT_00458B50: u32 = 0x0045_8b50;
/// `00891170` (thiscall on a record, returns the address of its texture coordinates; unnamed).
const RECORD_TEXTURE_COORDINATES_00891170: u32 = 0x0089_1170;
/// `Interface::IsMenuIDVisible` (`00702680`, cdecl: menu id, word).
const INTERFACE_IS_MENU_ID_VISIBLE: u32 = 0x0070_2680;
/// `0043c4b0` (no arguments, returns the renderer; unnamed).
const RENDERER_0043C4B0: u32 = 0x0043_c4b0;
/// `0084e3a0` (thiscall, returns a word; unnamed getter).
const GETTER_0084E3A0: u32 = 0x0084_e3a0;
/// `00644640` (thiscall on the interface, returns `AL`; unnamed).
const INTERFACE_TEST_00644640: u32 = 0x0064_4640;
/// `005e3fc0` (thiscall on the interface, returns an `NiAVObject`; unnamed).
const INTERFACE_NODE_005E3FC0: u32 = 0x005e_3fc0;
/// `007fa950` (thiscall on the interface, returns a pointer; unnamed).
const INTERFACE_007FA950: u32 = 0x007f_a950;
/// `008d80e0` (thiscall, returns the culling process; the map's name
/// `DialoguePackage::GetTargetOfConversation` may come from folded code).
const CULLER_008D80E0: u32 = 0x008d_80e0;
/// `004a0fd0` (thiscall on the culling process, one word: an accumulator).
const CULLER_SET_ACCUMULATOR_004A0FD0: u32 = 0x004a_0fd0;
/// `00414430` (thiscall: constructs an `NiColorA` from four `float`s).
const COLOR_A_INIT_00414430: u32 = 0x0041_4430;
/// `004a0ea0` (no arguments, returns the texture manager; unnamed).
const TEXTURE_MANAGER_004A0EA0: u32 = 0x004a_0ea0;
/// `00b6e110` (thiscall on the texture manager, five words: borrows a rendered texture).
const TEXTURE_MANAGER_GET_RENDERED_00B6E110: u32 = 0x00b6_e110;
/// `BSTextureManager::ReturnRenderedTexture` (`00b6da10`).
const TEXTURE_MANAGER_RETURN_RENDERED: u32 = 0x00b6_da10;
/// `004e9510` (no arguments, returns `AL`; unnamed).
const TEST_004E9510: u32 = 0x004e_9510;
/// `BSRenderedTexture::StopOffscreen` (`00b6b260`; the result is passed on).
const RENDERED_TEXTURE_00B6B260: u32 = 0x00b6_b260;
/// `00b6b8d0` (cdecl, two words; unnamed, `BSShader`).
const SHADER_00B6B8D0: u32 = 0x00b6_b8d0;
/// `00712e60` (thiscall on the camera, one word: a color; unnamed).
const CAMERA_00712E60: u32 = 0x0071_2e60;
/// `0045bbe0` (thiscall on the camera, returns the address of its 0x1c-byte frustum).
const CAMERA_FRUSTUM_0045BBE0: u32 = 0x0045_bbe0;
/// `NiCamera::SetViewFrustum` (`00a6faf0`).
const NI_CAMERA_SET_VIEW_FRUSTUM: u32 = 0x00a6_faf0;
/// `004e9bb0` (thiscall on the renderer, one word: the camera; unnamed).
const RENDERER_SET_CAMERA_004E9BB0: u32 = 0x004e_9bb0;
/// `0045cd60` (thiscall on the interface, returns a menu; unnamed).
const INTERFACE_MENU_0045CD60: u32 = 0x0045_cd60;
/// `0056c7f0` (thiscall, returns the object's node; unnamed).
const NODE_0056C7F0: u32 = 0x0056_c7f0;
/// `Interface::GetDebugTextRoot` (`00706d10`).
const INTERFACE_GET_DEBUG_TEXT_ROOT: u32 = 0x0070_6d10;
/// The word `fn_007fba00` passes to `00726070` (`011d96c0`).
const MENU_LIST_011D96C0: u32 = 0x011d_96c0;
/// `00726070` (thiscall, returns a pointer; unnamed).
const GETTER_00726070: u32 = 0x0072_6070;
/// `Tile::GetMenuTile` (`00a03d40`).
const TILE_GET_MENU_TILE: u32 = 0x00a0_3d40;
/// `Interface::GetPipboy` (`00705990`).
const INTERFACE_GET_PIPBOY: u32 = 0x0070_5990;
/// `00c4f270` (thiscall on the culling process, one word).
const CULLER_00C4F270: u32 = 0x00c4_f270;
/// `BSShaderUtil::AccumulateScene` (`00b6bee0`, cdecl: camera, scene, culling process).
const BS_SHADER_UTIL_ACCUMULATE_SCENE: u32 = 0x00b6_bee0;
/// `00b6c0d0` (cdecl, three words: camera, accumulator, word; unnamed).
const RENDER_00B6C0D0: u32 = 0x00b6_c0d0;
/// `00c4f2d0` (thiscall on the culling process).
const CULLER_00C4F2D0: u32 = 0x00c4_f2d0;
/// `00b6b790` (no arguments; unnamed, `BSShader`).
const SHADER_00B6B790: u32 = 0x00b6_b790;
/// `004e3270` (no arguments, returns a pointer; unnamed).
const GETTER_004E3270: u32 = 0x004e_3270;
/// `007fbeb0` (thiscall, two words; this unit, not translated).
const FUNCTION_007FBEB0: u32 = 0x007f_beb0;
/// `00aa13e0` (cdecl: size; the Ni allocator).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `BSShaderAccumulator::BSShaderAccumulator` (`00b660d0`, thiscall: three words).
const BS_SHADER_ACCUMULATOR_CONSTRUCT: u32 = 0x00b6_60d0;
/// `004a1040` (thiscall on the accumulator, one word; unnamed).
const ACCUMULATOR_004A1040: u32 = 0x004a_1040;
/// `BSShaderManager::GetTexture` (`00b55840`, cdecl: path, word, out pointer, two words).
const BS_SHADER_MANAGER_GET_TEXTURE: u32 = 0x00b5_5840;
/// `"Data\\Textures\\Pipboy3000\\screen.dds"`.
const SCREEN_TEXTURE_PATH: u32 = 0x0107_8534;
/// `"MESHES: Trying to clear a rendered menu screen but the screen trishape is NULL."`.
const NO_SCREEN_MESSAGE: u32 = 0x0107_8470;
/// `"MENUS: Default renderedmenu texture NOT FOUND.  Rendered menu may not work."`.
const NO_TEXTURE_MESSAGE: u32 = 0x0107_84e8;
/// `"MENUS: FORenderedMenu::CreateMenuLight could not find light node '%s' ..."`.
const NO_LIGHT_MESSAGE: u32 = 0x0107_8558;
/// `"RenderedMenuLight"`.
const RENDERED_MENU_LIGHT: u32 = 0x0107_85c4;
/// `00403df0` (thiscall on a setting, returns its string; unnamed).
const SETTING_STRING_00403DF0: u32 = 0x0040_3df0;
/// The settings `Initialize` reads: two floats and the scanlines texture path.
const BLUR_RADIUS_SETTING: u32 = 0x011d_b44c;
const BLUR_INTENSITY_SETTING: u32 = 0x011d_b470;
const SCANLINES_SETTING: u32 = 0x011d_b3d8;
/// `004391c0` (thiscall on a new 0x1c-byte property: constructs it; unnamed).
const PROPERTY_INIT_004391C0: u32 = 0x0043_91c0;
/// `0049ed90`, `00439340`, `00439390` (thiscall on that property, one word each; unnamed).
const PROPERTY_0049ED90: u32 = 0x0049_ed90;
const PROPERTY_00439340: u32 = 0x0043_9340;
const PROPERTY_00439390: u32 = 0x0043_9390;
/// `NiAVObject::AttachProperty` (`00439410`).
const NI_AV_OBJECT_ATTACH_PROPERTY: u32 = 0x0043_9410;
/// `004af350` (no arguments: returns another property type word; unnamed).
const PROPERTY_TYPE_004AF350: u32 = 0x004a_f350;
/// `004bc450` (thiscall on that property, one word: a color; unnamed).
const PROPERTY_SET_COLOR_004BC450: u32 = 0x004b_c450;
/// `NiAVObject::UpdateProperties` (`00a5a040`).
const NI_AV_OBJECT_UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// `00450b80` (cdecl, one word: returns a shadow scene node; unnamed).
const SHADOW_SCENE_NODE_00450B80: u32 = 0x0045_0b80;
/// `005585e0` (thiscall on the menu, returns its root node; unnamed).
const MENU_ROOT_005585E0: u32 = 0x0055_85e0;
/// `00438170` (thiscall: constructs an `NiFixedString` from a C string; returns it).
const FIXED_STRING_INIT: u32 = 0x0043_8170;
/// `004381b0` (thiscall: destroys an `NiFixedString`).
const FIXED_STRING_DESTROY: u32 = 0x0043_81b0;
/// `00a7d670` (thiscall on a new 0xfc-byte light: constructs it; unnamed).
const POINT_LIGHT_INIT_00A7D670: u32 = 0x00a7_d670;
/// `0043c490` (thiscall, returns the address of a 12-byte position; unnamed).
const POSITION_0043C490: u32 = 0x0043_c490;
/// `00440460` (thiscall on the light, one word: a position; unnamed).
const LIGHT_SET_POSITION_00440460: u32 = 0x0044_0460;
/// `ShadowSceneNode::AddLight` (`00b5c940`).
const SHADOW_SCENE_NODE_ADD_LIGHT: u32 = 0x00b5_c940;
/// `ShadowSceneNode::RemoveLight` (`00b5eed0`).
const SHADOW_SCENE_NODE_REMOVE_LIGHT: u32 = 0x00b5_eed0;
/// `00a5b950` (thiscall on the light, one word: its name; unnamed).
const SET_NAME_00A5B950: u32 = 0x00a5_b950;
/// `00653270` (cdecl: two words; unnamed, `bsfacegenmanager.cpp`).
const FIND_LIGHT_00653270: u32 = 0x0065_3270;
/// The first word `CreateMenuLight` passes to `00653270` (`011f4a98`).
const LIGHT_TYPE_011F4A98: u32 = 0x011f_4a98;
/// `00413f40` (thiscall, returns a pointer; unnamed).
const GETTER_00413F40: u32 = 0x0041_3f40;
/// `0043b1b0` (thiscall, returns the model path; unnamed).
const MODEL_PATH_0043B1B0: u32 = 0x0043_b1b0;
/// `Tile::GetMenuByClass` (`00a09030`, cdecl: menu class).
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;

/// The time `00825c00` gives, converted to a `float` (`FILD qword` of the
/// zero-extended word, `FSTP dword`).
fn now_milliseconds(e: &mut Engine) -> u32 {
    let ms = e.call(TIMER_GET_00825C00, &args![TIMER]).u32();
    x87_store32(f64::from(ms))
}

/// The value at the address `00403e20` gives for `setting`, as a `float`
/// memory operand.
fn setting_float(e: &mut Engine, setting: u32) -> f64 {
    let address = e.call(SETTING_VALUE, &args![setting]).u32();
    x87_m32(e.mem.u32(address))
}

// Translated from 007fae70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedMenu::SetScreenTexture` (Xbox PDB): once initialized, sets the texture
/// of the screen geometry's property (type from `00438220`) to `texture`, or to the
/// default texture when it is null. Without screen geometry it logs
/// "MESHES: Trying to clear a rendered menu screen but the screen trishape is NULL.".
pub fn fo_rendered_menu_set_screen_texture(
    e: &mut Engine,
    this: Ptr<FORenderedMenu>,
    texture: u32,
) {
    let base = this.addr();
    if e.get(this, FORenderedMenu::bInitialized) == 0 {
        return;
    }
    if e.call(POINTER_GET, &args![base + 4]).u32() == 0 {
        e.call(DEBUG_MESSAGE, &args![NO_SCREEN_MESSAGE]);
        return;
    }
    let kind = e.call(PROPERTY_TYPE_00438220, &[]).u32();
    let geometry = e.call(POINTER_GET, &args![base + 4]).u32();
    let property = e
        .call(NI_AV_OBJECT_GET_PROPERTY, &args![geometry, kind])
        .u32();
    if texture != 0 {
        e.call(PROPERTY_SET_TEXTURE_00438230, &args![property, texture]);
    } else {
        let default = e.call(POINTER_GET, &args![base + 0x10]).u32();
        e.call(PROPERTY_SET_TEXTURE_00438230, &args![property, default]);
    }
}

// Translated from 007faf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedMenu::OnIdle` (Xbox PDB): records whether the menu is open (`007079f0`
/// on the interface) and, when it is or just was, runs the effect updates with the
/// current time in milliseconds: the light (slot +0x48), burst (+0x38), vertical
/// hold (+0x3c) and shudder (+0x40) when active, the pulse (+0x44), then the distortion
/// (+0x4c) when active, or else starts one (`fn_007fb100`) once its next time has
/// passed. Returns whether it ran them.
pub fn fo_rendered_menu_on_idle(e: &mut Engine, this: Ptr<FORenderedMenu>) -> bool {
    let base = this.addr();
    let now = now_milliseconds(e);
    let was_open = e.get(this, FORenderedMenu::bMenuOpen);
    let interface = e.call(INTERFACE_GET_004B7210, &[]).u32();
    let open = e.call(INTERFACE_TEST_007079F0, &args![interface]).u8();
    e.set(this, FORenderedMenu::bMenuOpen, open);
    if e.get(this, FORenderedMenu::bMenuOpen) == 0 && was_open == 0 {
        return false;
    }
    let now_arg = x87_quiet_bits(now);
    e.vcall(base, 0x48, &args![now_arg]);
    if e.get(this, FORenderedMenu::bBurstActive) != 0 {
        e.vcall(base, 0x38, &args![now_arg]);
    }
    if e.get(this, FORenderedMenu::bVertHoldActive) != 0 {
        e.vcall(base, 0x3c, &args![now_arg]);
    }
    if e.get(this, FORenderedMenu::bShudderActive) != 0 {
        e.vcall(base, 0x40, &args![now_arg]);
    }
    e.vcall(base, 0x44, &args![now_arg]);
    if e.get(this, FORenderedMenu::bDistortActive) != 0 {
        e.vcall(base, 0x4c, &args![now_arg]);
    } else {
        let next = x87_load32(e.mem.u32(base + 0xa4));
        let current = x87_load32(now);
        // FCOMPP (next against now); TEST AH,5; JP: started only when next is below.
        if next < current {
            fn_007fb100(e, this);
        }
    }
    true
}

// Translated from 007fb040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedMenu::OnMenuOpen` (Xbox PDB): when `009611e0` gives an object, calls
/// `00867a20` on its menu (`Tile::GetMenu`); updates the menus root's clip windows,
/// shows the light (`00450f90` with 0) and sets `bMenuRendering`.
pub fn fo_rendered_menu_on_menu_open(e: &mut Engine, this: Ptr<FORenderedMenu>) {
    let base = this.addr();
    if e.call(GETTER_009611E0, &args![this]).u32() != 0 {
        let object = e.call(GETTER_009611E0, &args![this]).u32();
        let menu = e.call(TILE_GET_MENU, &args![object]).u32();
        e.call(MENU_00867A20, &args![menu]);
    }
    let root = e.call(INTERFACE_GET_MENUS_ROOT, &[]).u32();
    e.call(TILE_UPDATE_CLIPWINDOWS, &args![root]);
    if e.call(POINTER_GET, &args![base + 0x1c]).u32() != 0 {
        let light = e.call(POINTER_GET, &args![base + 0x1c]).u32();
        e.call(SET_APP_CULLED_00450F90, &args![light, 0u32]);
    }
    e.set(this, FORenderedMenu::bMenuRendering, 1);
}

// Translated from 007fb0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the menu-close counterpart of `OnMenuOpen`): clears `bMenuRendering`,
/// `bBurstActive` and `bVertHoldActive`, hides the light and restores the default
/// screen texture (`SetScreenTexture` with null).
pub fn fn_007fb0b0(e: &mut Engine, this: Ptr<FORenderedMenu>) {
    let base = this.addr();
    e.set(this, FORenderedMenu::bMenuRendering, 0);
    e.set(this, FORenderedMenu::bBurstActive, 0);
    e.set(this, FORenderedMenu::bVertHoldActive, 0);
    if e.call(POINTER_GET, &args![base + 0x1c]).u32() != 0 {
        let light = e.call(POINTER_GET, &args![base + 0x1c]).u32();
        e.call(SET_APP_CULLED_00450F90, &args![light, 1u32]);
    }
    fo_rendered_menu_set_screen_texture(e, this, 0);
}

// Translated from 007fb100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: starts a distortion: sets `bDistortActive`, its start time to now (in
/// milliseconds, as a `float`) and its position to 0.
pub fn fn_007fb100(e: &mut Engine, this: Ptr<FORenderedMenu>) {
    e.set(this, FORenderedMenu::bDistortActive, 1);
    let now = now_milliseconds(e);
    e.mem.set_u32(this.addr() + 0x98, now);
    e.mem.set_u32(this.addr() + 0x9c, x87_store32(0.0));
}

// Translated from 007fb150 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007fb150 - the exception-unwinding frame (FS:[0]) is not translated.
/// `FORenderedMenu::TriggerBurst` (Xbox PDB): starts a burst of `duration` with
/// `intensity` (current and maximum) at the current time; when the sounds flag at
/// `011db378` is set and `004dff00(chance)` is true, plays "UIPipBoyStatic"
/// (`BSAudio::GetSoundHandleByName` with flags 0x121).
pub fn fo_rendered_menu_trigger_burst(
    e: &mut Engine,
    this: Ptr<FORenderedMenu>,
    duration: f32,
    intensity: f32,
    chance: f32,
) {
    let base = this.addr();
    e.set(this, FORenderedMenu::bBurstActive, 1);
    e.mem
        .set_u32(base + 0x54, x87_quiet_bits(duration.to_bits()));
    e.mem
        .set_u32(base + 0x5c, x87_quiet_bits(intensity.to_bits()));
    e.mem
        .set_u32(base + 0x58, x87_quiet_bits(intensity.to_bits()));
    let now = now_milliseconds(e);
    e.mem.set_u32(base + 0x50, now);
    if e.global::<u8>(SOUNDS_ACTIVE_011DB378) == 0 {
        return;
    }
    if !e
        .call(
            FLOAT_TEST_004DFF00,
            &args![x87_quiet_bits(chance.to_bits())],
        )
        .bool()
    {
        return;
    }
    // The frame below `EBP`: `local(n)` is `EBP - n`.
    e.with_stack(0x40, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x40 - offset;
        let sound = local(0x18);
        let temporary = local(0x24);
        e.call(SOUND_HANDLE_INIT, &args![sound]);
        let audio = e.call(BS_AUDIO_Q_INSTANCE, &[]).u32();
        let handle = e
            .call(
                BS_AUDIO_GET_SOUND_HANDLE_BY_NAME,
                &args![audio, temporary, PIPBOY_STATIC_SOUND, 0x121u32],
            )
            .u32();
        e.call(SOUND_HANDLE_ASSIGN, &args![sound, handle]);
        e.call(SOUND_HANDLE_DESTROY, &args![temporary]);
        e.call(BS_SOUND_HANDLE_PLAY, &args![sound, 0u32]);
        e.call(SOUND_HANDLE_DESTROY, &args![sound]);
    });
}

// Translated from 007fb530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the pulse update, slot +0x44 by its place in `OnIdle`): with
/// `v = 2 * 00c5d320(time * s1)` sets `fPulseCurrentBrighten` to `v * s2` and
/// `fPulseCurrentRadius` to `v * s3` (`s1`..`s3` the settings at `011db408`,
/// `011db3a4`, `011db424`).
pub fn fn_007fb530(e: &mut Engine, this: Ptr<FORenderedMenu>, time: f32) {
    let scale = setting_float(e, 0x011d_b408);
    let product = x87_store32(x87_mul(x87_load32(time.to_bits()), scale));
    let wave = e
        .call(FLOAT_FUNCTION_00C5D320, &args![x87_quiet_bits(product)])
        .f64();
    let doubled = x87_store32(x87_add(wave, wave));
    let brighten = setting_float(e, 0x011d_b3a4);
    e.mem.set_u32(
        this.addr() + 0x8c,
        x87_store32(x87_mul(x87_load32(doubled), brighten)),
    );
    let radius = setting_float(e, 0x011d_b424);
    e.mem.set_u32(
        this.addr() + 0x90,
        x87_store32(x87_mul(x87_load32(doubled), radius)),
    );
}

// Translated from 007fb5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the light update, slot +0x48 by its place in `OnIdle`): with a light, its
/// color starts from the three words at `011f4998`; when the pulse affects the light it
/// becomes `ScreenLightColor * ((burst + pulse) * hold) * flicker` (burst: the current
/// burst intensity times the setting at `011db430`, 0 without a burst; pulse:
/// `fPulseCurrentBrighten` times `011db4a4`; hold: `004e44d0(time * 011db3e8) + 0.5`
/// with a vertical hold, else 1). The light gets `007fb730` of `ScreenLightColor *
/// base intensity` and that color (`004bc2e0`), then an update (`00a59c60`).
pub fn fn_007fb5a0(e: &mut Engine, this: Ptr<FORenderedMenu>, time: f32) {
    let base = this.addr();
    if e.call(POINTER_GET, &args![base + 0x1c]).u32() == 0 {
        return;
    }
    // The frame below `EBP`: `local(n)` is `EBP - n`.
    e.with_stack(0x70, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x70 - offset;
        let color = local(0xc);
        for i in 0..3 {
            let word = e.global::<u32>(BASE_LIGHT_COLOR + 4 * i);
            e.mem.set_u32(color + 4 * i, word);
        }
        if e.get(this, FORenderedMenu::bPulseAffectsLight) != 0 {
            let burst = if e.get(this, FORenderedMenu::bBurstActive) != 0 {
                let scale = setting_float(e, 0x011d_b430);
                x87_mul(x87_load32(e.mem.u32(base + 0x5c)), scale)
            } else {
                0.0
            };
            let burst = x87_store32(burst);
            let scale = setting_float(e, 0x011d_b4a4);
            let pulse = x87_store32(x87_mul(x87_load32(e.mem.u32(base + 0x8c)), scale));
            let hold = if e.get(this, FORenderedMenu::bVertHoldActive) != 0 {
                let scale = setting_float(e, 0x011d_b3e8);
                let product = x87_store32(x87_mul(x87_load32(time.to_bits()), scale));
                let wave = e
                    .call(FLOAT_FUNCTION_004E44D0, &args![x87_quiet_bits(product)])
                    .f64();
                x87_add(wave, e.global::<f64>(HALF))
            } else {
                1.0
            };
            let hold = x87_store32(hold);
            let factor = x87_mul(x87_add(x87_load32(burst), x87_m32(pulse)), x87_m32(hold));
            let factor = x87_store32(factor);
            let flicker = x87_quiet_bits(e.mem.u32(base + 0x3c));
            let scaled = e
                .call(
                    COLOR_SCALE_00633B40,
                    &args![
                        base + SCREEN_LIGHT_COLOR,
                        local(0x40),
                        x87_quiet_bits(factor)
                    ],
                )
                .u32();
            let result = e
                .call(COLOR_SCALE_00633B40, &args![scaled, local(0x4c), flicker])
                .u32();
            for i in 0..3 {
                let word = e.mem.u32(result + 4 * i);
                e.mem.set_u32(color + 4 * i, word);
            }
        }
        let intensity = x87_quiet_bits(e.mem.u32(base + 0x38));
        let scaled = e
            .call(
                COLOR_SCALE_00633B40,
                &args![base + SCREEN_LIGHT_COLOR, local(0x58), intensity],
            )
            .u32();
        e.call(COLOR_COMBINE_007FB730, &args![scaled, local(0x24), color]);
        let light = e.call(POINTER_GET, &args![base + 0x1c]).u32();
        e.call(LIGHT_SET_COLOR_004BC2E0, &args![light, local(0x24)]);
        e.call(
            UPDATE_DATA_INIT_0043D410,
            &args![local(0x18), x87_store32(0.0), 0u32, 0u32],
        );
        let light = e.call(POINTER_GET, &args![base + 0x1c]).u32();
        e.call(UPDATE_00A59C60, &args![light, local(0x18)]);
    });
}

// Translated from 007fb790 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007fb790 - the exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed: picks the screen geometry under the cursor. Hides nothing first but shows
/// the menu root (`00450f90` with 0); without screen geometry returns false. Otherwise
/// an `NiPick` (`NiPick::NiPick(0, 8)`, its target the screen geometry) is cast along
/// the camera's ray through the cursor (`00a71080` with the integer cursor coordinates
/// `00792760` and `009a9350` give, through `_ftol2`); when the first record's object
/// is the screen geometry, `*u` and `*v` get its texture coordinates (rescaled by
/// `0.63`/`0.37` and `0.65`/`0.34` while menu 0x40c is visible) times the size word
/// (`0084e3a0` of the renderer's slot +0xd4), `u` also times 1.333333, and the result
/// is true. The first stack word is never read.
pub fn fn_007fb790(
    e: &mut Engine,
    this: Ptr<FORenderedMenu>,
    _unused_1: u32,
    u: u32,
    v: u32,
) -> bool {
    let base = this.addr();
    let root = e.call(POINTER_GET, &args![base + 0x14]).u32();
    e.call(SET_APP_CULLED_00450F90, &args![root, 0u32]);
    let interface = e.call(INTERFACE_GET_004B7210, &[]).u32();
    if e.call(POINTER_GET, &args![base + 4]).u32() == 0 {
        return false;
    }
    // The frame below `EBP`: `local(n)` is `EBP - n`.
    e.with_stack(0x90, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x90 - offset;
        let origin = local(0x58);
        let direction = local(0x64);
        let pick = local(0x4c);
        let mut found = false;
        e.call(POINT_INIT_006815C0, &args![origin]);
        e.call(POINT_INIT_006815C0, &args![direction]);
        e.call(NI_PICK_CONSTRUCT, &args![pick, 0u32, 8u32]);
        let camera = if e.call(CAMERA_008B6200, &args![this]).u32() != 0 {
            e.call(CAMERA_008B6200, &args![this]).u32()
        } else {
            let object = e.call(GETTER_0045C670, &[]).u32();
            e.call(GETTER_006629F0, &args![object]).u32()
        };
        e.call(PICK_00458B30, &args![pick, 1u32]);
        let geometry = e.call(POINTER_GET, &args![base + 4]).u32();
        e.call(PICK_SET_TARGET_00705FC0, &args![pick, geometry]);
        e.call(PICK_006ECD40, &args![pick, 0u32]);
        e.call(PICK_00792780, &args![pick, 1u32]);
        let first = e.call(INTERFACE_REAL_00792760, &args![interface]).f64();
        let first = e.call(FTOL, &args![first]).u32();
        let second = e.call(INTERFACE_REAL_009A9350, &args![interface]).f64();
        let second = e.call(FTOL, &args![second]).u32();
        e.call(
            CAMERA_WINDOW_POINT_TO_RAY_00A71080,
            &args![camera, second, first, origin, direction, 0u32],
        );
        if e.call(NI_PICK_PICK_OBJECTS, &args![pick, origin, direction, 0u32])
            .bool()
        {
            let results = e.call(PICK_RESULTS_00500940, &args![pick]).u32();
            let record = e.call(PICK_RECORD_00968670, &args![results, 0u32]).u32();
            if record != 0 {
                let object = e.call(RECORD_OBJECT_00458B50, &args![record]).u32();
                let geometry = e.call(POINTER_GET, &args![base + 4]).u32();
                if object == geometry {
                    let coordinates = e
                        .call(RECORD_TEXTURE_COORDINATES_00891170, &args![record])
                        .u32();
                    let mut texture_u = e.mem.u32(coordinates);
                    let mut texture_v = e.mem.u32(coordinates + 4);
                    if e.call(INTERFACE_IS_MENU_ID_VISIBLE, &args![0x40cu32, 0u32])
                        .bool()
                    {
                        let value = x87_mul(x87_load32(texture_u), e.global::<f64>(PICK_U_SCALE));
                        texture_u = x87_store32(value);
                        let value = x87_add(x87_load32(texture_u), e.global::<f64>(PICK_U_OFFSET));
                        texture_u = x87_store32(value);
                        let value = x87_mul(x87_load32(texture_v), e.global::<f64>(PICK_V_SCALE));
                        texture_v = x87_store32(value);
                        let value = x87_add(x87_load32(texture_v), e.global::<f64>(PICK_V_OFFSET));
                        texture_v = x87_store32(value);
                    }
                    let renderer = e.call(RENDERER_0043C4B0, &[]).u32();
                    let target = e.vcall(renderer, 0xd4, &[]).u32();
                    let size = e.call(GETTER_0084E3A0, &args![target]).u32();
                    let height = x87_store32(f64::from(size));
                    let width = x87_store32(x87_mul(x87_load32(height), e.global::<f64>(ASPECT)));
                    e.mem.set_u32(
                        u,
                        x87_store32(x87_mul(x87_load32(texture_u), x87_m32(width))),
                    );
                    e.mem.set_u32(
                        v,
                        x87_store32(x87_mul(x87_load32(texture_v), x87_m32(height))),
                    );
                    found = true;
                }
            }
        }
        e.call(NI_PICK_DESTROY, &args![pick]);
        found
    })
}

// Translated from 007fba00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007fba00 - the exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed (the menu's render into its texture): once initialized and with a menu root,
/// renders the menu scene with the menu's camera into `texture` (or a rendered texture
/// borrowed from the texture manager, `00b6e110` with 0x31, and returned afterwards):
/// sets the culling accumulator and the renderer's background color, a 1280 x 960
/// orthographic frustum (restored afterwards), hides the debug text, the menu given by
/// `0045cd60`, the menu tile of the list at `011d96c0` and (unless this is the Pip-Boy
/// menu) shows the menu root only for the render, hides the quantity/tutorial/message
/// menus (`fn_007fc790`, `fn_007fc8d0`), accumulates and renders the scene, then
/// restores every hidden state, calls slot +0x50 with the texture and `first`,
/// `second`, the partial-screen update `007fbeb0` when used, and slot +0x10.
pub fn fn_007fba00(
    e: &mut Engine,
    this: Ptr<FORenderedMenu>,
    first: u32,
    second: u32,
    texture: u32,
) {
    let base = this.addr();
    if e.get(this, FORenderedMenu::bInitialized) == 0 {
        return;
    }
    if e.call(POINTER_GET, &args![base + 0x14]).u32() == 0 {
        return;
    }
    // The frame below `EBP`: `local(n)` is `EBP - n`.
    e.with_stack(0xa0, |e, frame| {
        let local = |offset: u32| frame.addr() + 0xa0 - offset;
        let renderer = e.call(RENDERER_0043C4B0, &[]).u32();
        let interface = e.call(INTERFACE_GET_004B7210, &[]).u32();
        if e.call(INTERFACE_TEST_00644640, &args![interface]).bool() {
            let node = e.call(INTERFACE_NODE_005E3FC0, &args![interface]).u32();
            e.call(SET_APP_CULLED_00450F90, &args![node, 1u32]);
        }
        let scene = e.call(INTERFACE_007FA950, &args![interface]).u32();
        let culler = e.call(CULLER_008D80E0, &args![scene]).u32();
        let accumulator = e.call(POINTER_GET, &args![base + ACCUMULATOR]).u32();
        e.call(CULLER_SET_ACCUMULATOR_004A0FD0, &args![culler, accumulator]);
        let zero = x87_store32(0.0);
        let one = x87_store32(1.0);
        let saved_color = local(0x80);
        e.call(
            COLOR_A_INIT_00414430,
            &args![saved_color, zero, zero, zero, zero],
        );
        e.vcall(renderer, 0xb4, &args![saved_color]);
        e.vcall(renderer, 0xac, &args![base + BACKGROUND_COLOR]);
        let camera = e.call(GETTER_006629F0, &args![scene]).u32();
        let target = local(0x70);
        e.call(POINTER_INIT, &args![target, 0u32]);
        if texture == 0 {
            let renderer_object = e.call(RENDERER_0043C4B0, &[]).u32();
            let manager = e.call(TEXTURE_MANAGER_004A0EA0, &[]).u32();
            let borrowed = e
                .call(
                    TEXTURE_MANAGER_GET_RENDERED_00B6E110,
                    &args![manager, renderer_object, 0x31u32, 0u32, 0u32, 0u32],
                )
                .u32();
            e.call(POINTER_SET, &args![target, borrowed]);
        } else {
            e.call(POINTER_SET, &args![target, texture]);
        }
        if e.call(TEST_004E9510, &[]).bool() {
            let rendered = e.call(POINTER_GET, &args![target]).u32();
            let value = e.call(RENDERED_TEXTURE_00B6B260, &args![rendered]).u32();
            e.call(SHADER_00B6B8D0, &args![second, value]);
            let clear = local(0x90);
            e.call(COLOR_A_INIT_00414430, &args![clear, zero, one, one, zero]);
            e.call(CAMERA_00712E60, &args![camera, clear]);
        }
        let frustum = e.call(CAMERA_FRUSTUM_0045BBE0, &args![camera]).u32();
        let new_frustum = local(0x28);
        let saved_frustum = local(0x6c);
        for i in 0..7 {
            let word = e.mem.u32(frustum + 4 * i);
            e.mem.set_u32(new_frustum + 4 * i, word);
        }
        for i in 0..7 {
            let word = e.mem.u32(new_frustum + 4 * i);
            e.mem.set_u32(saved_frustum + 4 * i, word);
        }
        e.mem.set_u8(local(0x10), 1);
        e.mem.set_u32(new_frustum, zero);
        let right = x87_quiet_bits(e.global::<u32>(FRUSTUM_RIGHT));
        e.mem.set_u32(new_frustum + 4, right);
        let top = x87_quiet_bits(e.global::<u32>(FRUSTUM_TOP));
        e.mem.set_u32(new_frustum + 8, top);
        e.mem.set_u32(new_frustum + 0xc, zero);
        e.mem.set_u32(new_frustum + 0x10, zero);
        let far = x87_quiet_bits(e.global::<u32>(FRUSTUM_FAR));
        e.mem.set_u32(new_frustum + 0x14, far);
        e.call(NI_CAMERA_SET_VIEW_FRUSTUM, &args![camera, new_frustum]);
        e.call(RENDERER_SET_CAMERA_004E9BB0, &args![renderer, camera]);
        let menu = e.call(INTERFACE_MENU_0045CD60, &args![interface]).u32();
        let mut menu_hidden = 0u8;
        if menu != 0 {
            let node = e.call(NODE_0056C7F0, &args![menu]).u32();
            menu_hidden = e.call(IS_APP_CULLED_00456610, &args![node]).u8();
            let node = e.call(NODE_0056C7F0, &args![menu]).u32();
            e.call(SET_APP_CULLED_00450F90, &args![node, 1u32]);
        }
        let debug_text = e.call(INTERFACE_GET_DEBUG_TEXT_ROOT, &[]).u32();
        let debug_hidden = if debug_text == 0 {
            0
        } else {
            e.call(IS_APP_CULLED_00456610, &args![debug_text]).u8()
        };
        if debug_text != 0 {
            e.call(SET_APP_CULLED_00450F90, &args![debug_text, 1u32]);
        }
        let list = e.global::<u32>(MENU_LIST_011D96C0);
        let menu_owner = e.call(GETTER_00726070, &args![list]).u32();
        let tile = e.call(TILE_GET_MENU_TILE, &args![menu_owner]).u32();
        let node = e.call(NODE_0056C7F0, &args![tile]).u32();
        let tile_hidden = e.call(IS_APP_CULLED_00456610, &args![node]).u8();
        let tile = e.call(TILE_GET_MENU_TILE, &args![menu_owner]).u32();
        let node = e.call(NODE_0056C7F0, &args![tile]).u32();
        e.call(SET_APP_CULLED_00450F90, &args![node, 1u32]);
        if base != e.call(INTERFACE_GET_PIPBOY, &[]).u32() {
            let root = e.call(POINTER_GET, &args![base + 0x14]).u32();
            e.call(SET_APP_CULLED_00450F90, &args![root, 0u32]);
        }
        fn_007fc790(e, this);
        e.call(CULLER_00C4F270, &args![culler, 1u32]);
        let root = e.call(POINTER_GET, &args![base + 0x14]).u32();
        let scene_camera = e.call(GETTER_006629F0, &args![scene]).u32();
        e.call(
            BS_SHADER_UTIL_ACCUMULATE_SCENE,
            &args![scene_camera, root, culler],
        );
        let accumulator = e.call(POINTER_GET, &args![base + ACCUMULATOR]).u32();
        let scene_camera = e.call(GETTER_006629F0, &args![scene]).u32();
        e.call(RENDER_00B6C0D0, &args![scene_camera, accumulator, 0u32]);
        e.call(CULLER_00C4F2D0, &args![culler]);
        fn_007fc8d0(e, this);
        e.call(CULLER_SET_ACCUMULATOR_004A0FD0, &args![culler, 0u32]);
        if base != e.call(INTERFACE_GET_PIPBOY, &[]).u32() {
            let root = e.call(POINTER_GET, &args![base + 0x14]).u32();
            e.call(SET_APP_CULLED_00450F90, &args![root, 1u32]);
        }
        let tile = e.call(TILE_GET_MENU_TILE, &args![menu_owner]).u32();
        let node = e.call(NODE_0056C7F0, &args![tile]).u32();
        e.call(
            SET_APP_CULLED_00450F90,
            &args![node, u32::from(tile_hidden)],
        );
        if menu != 0 {
            let node = e.call(NODE_0056C7F0, &args![menu]).u32();
            e.call(
                SET_APP_CULLED_00450F90,
                &args![node, u32::from(menu_hidden)],
            );
        }
        if debug_text != 0 {
            e.call(
                SET_APP_CULLED_00450F90,
                &args![debug_text, u32::from(debug_hidden)],
            );
        }
        e.call(SHADER_00B6B790, &[]);
        let rendered = e.call(POINTER_GET, &args![target]).u32();
        e.vcall(base, 0x50, &args![rendered, first, second]);
        if texture == 0 {
            let rendered = e.call(POINTER_GET, &args![target]).u32();
            let manager = e.call(TEXTURE_MANAGER_004A0EA0, &[]).u32();
            e.call(TEXTURE_MANAGER_RETURN_RENDERED, &args![manager, rendered]);
        }
        e.vcall(renderer, 0xac, &args![saved_color]);
        e.call(NI_CAMERA_SET_VIEW_FRUSTUM, &args![camera, saved_frustum]);
        if e.get(this, FORenderedMenu::bUsePartialScreenGeometry) != 0 {
            let object = e.call(GETTER_004E3270, &[]).u32();
            e.call(FUNCTION_007FBEB0, &args![object, 0u32, 0u32]);
        }
        e.vcall(base, 0x10, &[]);
        if e.call(INTERFACE_TEST_00644640, &args![interface]).bool() {
            let node = e.call(INTERFACE_NODE_005E3FC0, &args![interface]).u32();
            e.call(SET_APP_CULLED_00450F90, &args![node, 0u32]);
        }
        e.call(POINTER_DESTROY, &args![target]);
    });
}

// Translated from 007fc2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: sets the default texture pointer (`spDefaultTex`) to `texture`.
pub fn fn_007fc2b0(e: &mut Engine, this: Ptr<FORenderedMenu>, texture: u32) {
    e.call(POINTER_SET, &args![this.addr() + 0x10, texture]);
}

// Translated from 007fc150 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007fc150 - the exception-unwinding frame (FS:[0]) is not translated.
/// `FORenderedMenu::Initialize` (Xbox PDB): makes the menu's shader accumulator
/// (0x280 bytes, `BSShaderAccumulator(100, 1, 0x2f7)`, `004a1040` with 10), loads the
/// default screen texture "Data\Textures\Pipboy3000\screen.dds" (logging when it is
/// missing), reads the blur radius and intensity settings, loads the scanlines texture
/// named by its setting into `spScanlinesTexture`, and sets `bInitialized`.
pub fn fo_rendered_menu_initialize(e: &mut Engine, this: Ptr<FORenderedMenu>) {
    let base = this.addr();
    let block = e.call(NI_ALLOCATE, &args![0x280u32]).u32();
    let accumulator = if block == 0 {
        0
    } else {
        e.call(
            BS_SHADER_ACCUMULATOR_CONSTRUCT,
            &args![block, 100u32, 1u32, 0x2f7u32],
        )
        .u32()
    };
    e.call(POINTER_SET, &args![base + ACCUMULATOR, accumulator]);
    let accumulator = e.call(POINTER_GET, &args![base + ACCUMULATOR]).u32();
    e.call(ACCUMULATOR_004A1040, &args![accumulator, 10u32]);
    e.with_stack(0x10, |e, frame| {
        let texture_pointer = frame.addr();
        e.call(POINTER_INIT, &args![texture_pointer, 0u32]);
        e.call(
            BS_SHADER_MANAGER_GET_TEXTURE,
            &args![SCREEN_TEXTURE_PATH, 1u32, texture_pointer, 1u32, 0u32],
        );
        let texture = e.call(POINTER_GET, &args![texture_pointer]).u32();
        if texture == 0 {
            e.call(DEBUG_MESSAGE, &args![NO_TEXTURE_MESSAGE]);
        }
        fn_007fc2b0(e, this, texture);
        let address = e.call(SETTING_VALUE, &args![BLUR_RADIUS_SETTING]).u32();
        let value = x87_quiet_bits(e.mem.u32(address));
        e.mem.set_u32(base + 0x40, value);
        let address = e.call(SETTING_VALUE, &args![BLUR_INTENSITY_SETTING]).u32();
        let value = x87_quiet_bits(e.mem.u32(address));
        e.mem.set_u32(base + 0x44, value);
        let path = e
            .call(SETTING_STRING_00403DF0, &args![SCANLINES_SETTING])
            .u32();
        e.call(
            BS_SHADER_MANAGER_GET_TEXTURE,
            &args![path, 1u32, base + SCANLINES_TEXTURE, 0u32, 0u32],
        );
        e.set(this, FORenderedMenu::bInitialized, 1);
        e.call(POINTER_DESTROY, &args![texture_pointer]);
    });
}

// Translated from 007fc2d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007fc2d0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `FORenderedMenu::PrepareGeometry` (Xbox PDB): gives `geometry`'s texture property the
/// default texture; attaches a new 0x1c-byte property (`004391c0`, then `0049ed90(0)`,
/// `00439340(6)`, `00439390(0)`) when it has none of type 0; sets the color of its
/// property of type `004af350` to the words at `011f4998`; then updates its properties
/// and the object (`00a59c60` with a zeroed update block).
pub fn fo_rendered_menu_prepare_geometry(e: &mut Engine, this: Ptr<FORenderedMenu>, geometry: u32) {
    let kind = e.call(PROPERTY_TYPE_00438220, &[]).u32();
    let property = e
        .call(NI_AV_OBJECT_GET_PROPERTY, &args![geometry, kind])
        .u32();
    let default = e.call(POINTER_GET, &args![this.addr() + 0x10]).u32();
    e.call(PROPERTY_SET_TEXTURE_00438230, &args![property, default]);
    if e.call(NI_AV_OBJECT_GET_PROPERTY, &args![geometry, 0u32])
        .u32()
        == 0
    {
        let block = e.call(NI_ALLOCATE, &args![0x1cu32]).u32();
        let property = if block == 0 {
            0
        } else {
            e.call(PROPERTY_INIT_004391C0, &args![block]).u32()
        };
        e.call(PROPERTY_0049ED90, &args![property, 0u32]);
        e.call(PROPERTY_00439340, &args![property, 6u32]);
        e.call(PROPERTY_00439390, &args![property, 0u32]);
        e.call(NI_AV_OBJECT_ATTACH_PROPERTY, &args![geometry, property]);
    }
    let kind = e.call(PROPERTY_TYPE_004AF350, &[]).u32();
    let property = e
        .call(NI_AV_OBJECT_GET_PROPERTY, &args![geometry, kind])
        .u32();
    if property != 0 {
        e.call(
            PROPERTY_SET_COLOR_004BC450,
            &args![property, BASE_LIGHT_COLOR],
        );
    }
    e.call(NI_AV_OBJECT_UPDATE_PROPERTIES, &args![geometry]);
    e.with_stack(0xc, |e, update| {
        e.call(
            UPDATE_DATA_INIT_0043D410,
            &args![update, x87_store32(0.0), 0u32, 0u32],
        );
        e.call(UPDATE_00A59C60, &args![geometry, update]);
    });
}

// Translated from 007fc400 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007fc400 - the exception-unwinding frame (FS:[0]) is not translated.
/// `FORenderedMenu::CreateMenuLight` (Xbox PDB): replaces the menu's light. With the
/// menu root's node named `name` (slot +0x9c) whose slot +0xc gives a light source, a
/// new 0xfc-byte point light (`00a7d670`) at the source's position is added to
/// `shadow_scene_node` (or the one `00450b80(0)` gives), attached to the source's parent
/// (slot +0xdc), updated, hidden and named "RenderedMenuLight". Otherwise the root's
/// node "RenderedMenuLight" becomes the light through `00653270`, or a message names the
/// missing node and the model. Returns whether a light was made.
pub fn fo_rendered_menu_create_menu_light(
    e: &mut Engine,
    this: Ptr<FORenderedMenu>,
    name: u32,
    shadow_scene_node: u32,
) -> bool {
    let base = this.addr();
    let mut created = false;
    let shadow_scene_node = if shadow_scene_node == 0 {
        e.call(SHADOW_SCENE_NODE_00450B80, &args![0u32]).u32()
    } else {
        shadow_scene_node
    };
    if e.call(MENU_ROOT_005585E0, &args![this]).u32() == 0 {
        return false;
    }
    fo_rendered_menu_clear_menu_light(e, this);
    // The frame below `EBP`: `local(n)` is `EBP - n`.
    e.with_stack(0x80, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x80 - offset;
        let root = e.call(MENU_ROOT_005585E0, &args![this]).u32();
        let name_string = e.call(FIXED_STRING_INIT, &args![local(0x3c), name]).u32();
        let node = e.vcall(root, 0x9c, &args![name_string]).u32();
        e.call(FIXED_STRING_DESTROY, &args![local(0x3c)]);
        let source = if node == 0 {
            0
        } else {
            e.vcall(node, 0xc, &[]).u32()
        };
        if source != 0 {
            let block = e.call(NI_ALLOCATE, &args![0xfcu32]).u32();
            let light = if block == 0 {
                0
            } else {
                e.call(POINT_LIGHT_INIT_00A7D670, &args![block]).u32()
            };
            e.call(POINTER_SET, &args![base + 0x1c, light]);
            let position = e.call(POSITION_0043C490, &args![source]).u32();
            let copy = local(0x34);
            for i in 0..3 {
                let word = e.mem.u32(position + 4 * i);
                e.mem.set_u32(copy + 4 * i, word);
            }
            let light = e.call(POINTER_GET, &args![base + 0x1c]).u32();
            e.call(LIGHT_SET_POSITION_00440460, &args![light, copy]);
            let light = e.call(POINTER_GET, &args![base + 0x1c]).u32();
            e.call(
                SHADOW_SCENE_NODE_ADD_LIGHT,
                &args![shadow_scene_node, light, 1u32],
            );
            e.set(
                this,
                FORenderedMenu::pLightShadowSceneNode,
                shadow_scene_node,
            );
            let parent = e.call(GETTER_009611E0, &args![source]).u32();
            let light = e.call(POINTER_GET, &args![base + 0x1c]).u32();
            e.vcall(parent, 0xdc, &args![light, 1u32]);
            e.call(
                UPDATE_DATA_INIT_0043D410,
                &args![local(0x28), x87_store32(0.0), 0u32, 0u32],
            );
            let geometry = e.call(POINTER_GET, &args![base + 4]).u32();
            e.call(UPDATE_00A59C60, &args![geometry, local(0x28)]);
            let light = e.call(POINTER_GET, &args![base + 0x1c]).u32();
            e.call(SET_APP_CULLED_00450F90, &args![light, 1u32]);
            let light_name = e
                .call(FIXED_STRING_INIT, &args![local(0x48), RENDERED_MENU_LIGHT])
                .u32();
            let light = e.call(POINTER_GET, &args![base + 0x1c]).u32();
            e.call(SET_NAME_00A5B950, &args![light, light_name]);
            e.call(FIXED_STRING_DESTROY, &args![local(0x48)]);
            created = true;
        } else {
            let root = e.call(MENU_ROOT_005585E0, &args![this]).u32();
            let light_name = e
                .call(FIXED_STRING_INIT, &args![local(0x4c), RENDERED_MENU_LIGHT])
                .u32();
            let node = e.vcall(root, 0x9c, &args![light_name]).u32();
            e.call(FIXED_STRING_DESTROY, &args![local(0x4c)]);
            if node != 0 {
                let light = e
                    .call(FIND_LIGHT_00653270, &args![LIGHT_TYPE_011F4A98, node])
                    .u32();
                e.call(POINTER_SET, &args![base + 0x1c, light]);
                if e.call(POINTER_GET, &args![base + 0x1c]).u32() != 0 {
                    created = true;
                }
            } else {
                let root = e.call(MENU_ROOT_005585E0, &args![this]).u32();
                let model = e.call(GETTER_00413F40, &args![root]).u32();
                let path = e.call(MODEL_PATH_0043B1B0, &args![model]).u32();
                e.call(DEBUG_MESSAGE, &args![NO_LIGHT_MESSAGE, name, path]);
            }
        }
    });
    created
}

// Translated from 007fc6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedMenu::ClearMenuLight` (Xbox PDB): with a light, detaches it from the
/// menu root's parent (slot +0xc of the root, then its slot +0xe8), removes it from its
/// shadow scene node and clears the pointer.
pub fn fo_rendered_menu_clear_menu_light(e: &mut Engine, this: Ptr<FORenderedMenu>) {
    let base = this.addr();
    if e.call(POINTER_GET, &args![base + 0x1c]).u32() == 0 {
        return;
    }
    let root = e.call(MENU_ROOT_005585E0, &args![this]).u32();
    if e.vcall(root, 0xc, &[]).u32() != 0 {
        let root = e.call(MENU_ROOT_005585E0, &args![this]).u32();
        let parent = e.vcall(root, 0xc, &[]).u32();
        let light = e.call(POINTER_GET, &args![base + 0x1c]).u32();
        e.vcall(parent, 0xe8, &args![light]);
    }
    let light = e.call(POINTER_GET, &args![base + 0x1c]).u32();
    let node = e.get(this, FORenderedMenu::pLightShadowSceneNode);
    e.call(SHADOW_SCENE_NODE_REMOVE_LIGHT, &args![node, light]);
    e.call(POINTER_SET, &args![base + 0x1c, 0u32]);
}

/// The three menus `fn_007fc790` and `fn_007fc8d0` hide and show (classes 0x3e9,
/// 0x3f8, 0x423) and the flag each one's state is kept in.
const HIDDEN_MENUS: [(u32, u32); 3] = [(0x3e9, 0xd2), (0x3f8, 0xd0), (0x423, 0xd1)];

// Translated from 007fc790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: hides the menus of classes 0x3e9, 0x3f8 and 0x423 (`Tile::GetMenuByClass`)
/// for the render, remembering in `bHidMessage`, `bHidQuantity` and `bHidTutorial`
/// which of them were shown (and so are hidden now).
pub fn fn_007fc790(e: &mut Engine, this: Ptr<FORenderedMenu>) {
    let base = this.addr();
    let menus: Vec<u32> = HIDDEN_MENUS
        .iter()
        .map(|(class, _)| e.call(TILE_GET_MENU_BY_CLASS, &args![*class]).u32())
        .collect();
    for (menu, (_, flag)) in menus.iter().zip(HIDDEN_MENUS) {
        let shown = if *menu == 0 {
            false
        } else {
            let node = e.call(NODE_0056C7F0, &args![*menu]).u32();
            !e.call(IS_APP_CULLED_00456610, &args![node]).bool()
        };
        e.mem.set_u8(base + flag, u8::from(shown));
    }
    for (menu, (_, flag)) in menus.iter().zip(HIDDEN_MENUS) {
        if e.mem.u8(base + flag) != 0 {
            let node = e.call(NODE_0056C7F0, &args![*menu]).u32();
            e.call(SET_APP_CULLED_00450F90, &args![node, 1u32]);
        }
    }
}

// Translated from 007fc8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: shows again the menus `fn_007fc790` hid.
pub fn fn_007fc8d0(e: &mut Engine, this: Ptr<FORenderedMenu>) {
    let base = this.addr();
    let menus: Vec<u32> = HIDDEN_MENUS
        .iter()
        .map(|(class, _)| e.call(TILE_GET_MENU_BY_CLASS, &args![*class]).u32())
        .collect();
    for (menu, (_, flag)) in menus.iter().zip(HIDDEN_MENUS) {
        if e.mem.u8(base + flag) != 0 {
            let node = e.call(NODE_0056C7F0, &args![*menu]).u32();
            e.call(SET_APP_CULLED_00450F90, &args![node, 0u32]);
        }
    }
}

// ---------------------------------------------------------------------------
// Batch n051

/// `FORenderedMenu::FORenderedMenu` (`007faa20`), the `FORenderedTerminal` vtable
/// (`0106ebe4`) and the `NiPointer` constructor (`00633c90`, `RET 4`).
const RENDERED_MENU_CONSTRUCT: u32 = 0x007f_aa20;
const RENDERED_TERMINAL_VTABLE: u32 = 0x0106_ebe4;
/// The vertical hold's speed scale (`01016978`, a `double`) and the shudder a finished
/// vertical hold starts: its duration (`0101b2d0`) and intensity and frequency
/// (`010181bc`).
const VERT_HOLD_SPEED_SCALE: u32 = 0x0101_6978;
const SHUDDER_DURATION_0101B2D0: u32 = 0x0101_b2d0;
const SHUDDER_VALUE_010181BC: u32 = 0x0101_81bc;
/// The distortion settings: its length (`011db520`) and the random part of the next
/// distortion time (`011db4f8`, an integer); `rand` (`00ecadb8`).
const DISTORT_LENGTH_SETTING: u32 = 0x011d_b520;
const DISTORT_INTERVAL_SETTING: u32 = 0x011d_b4f8;
const SETTING_INT_VALUE: u32 = 0x0043_d4d0;
const RAND: u32 = 0x00ec_adb8;
/// `00404040` (cdecl, a `float`, `ST0`): the whole part the vertical hold wraps by.
const WRAP_00404040: u32 = 0x0040_4040;
/// `1.0` and `0.0` (`double`s) and `-1.0` (a `float`).
const ONE_DOUBLE: u32 = 0x0101_2070;
const ZERO_DOUBLE: u32 = 0x0101_2060;
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
/// The effect parameters `DoEffect` hands the image-space shader (statics; their names
/// are not confirmed): the blur radius and intensity (`011ae5e8`, `011ae5ec`), the
/// scanline scale (`011ae5f0`), the height (`011ae5f4`), the distortion setting
/// (`011ae5f8`), the vertical offset (`012007e8`), the third `DoEffect` word
/// (`012007ec`), the distortion length and position (`012007f0`, `012007f4`), its flag
/// (`012007f8`), the tint color (`01200800`, four words) and two texture pointers
/// (`01200810`, `01200818`).
const EFFECT_BLUR_RADIUS: u32 = 0x011a_e5e8;
const EFFECT_BLUR_INTENSITY: u32 = 0x011a_e5ec;
const EFFECT_SCANLINE_SCALE: u32 = 0x011a_e5f0;
const EFFECT_HEIGHT: u32 = 0x011a_e5f4;
const EFFECT_DISTORT_SETTING: u32 = 0x011a_e5f8;
const EFFECT_VERTICAL_OFFSET: u32 = 0x0120_07e8;
const EFFECT_THIRD_012007EC: u32 = 0x0120_07ec;
const EFFECT_DISTORT_LENGTH: u32 = 0x0120_07f0;
const EFFECT_DISTORT_POSITION: u32 = 0x0120_07f4;
const EFFECT_DISTORT_FLAG: u32 = 0x0120_07f8;
const EFFECT_TINT: u32 = 0x0120_0800;
const EFFECT_SCANLINES_POINTER: u32 = 0x0120_0810;
const EFFECT_TEXTURE_POINTER: u32 = 0x0120_0818;
/// The distortion setting `DoEffect` copies (`011db464`).
const DISTORT_EFFECT_SETTING: u32 = 0x011d_b464;
/// `NiPointer` assignment from a pointer (`006e5cc0`, `RET 4`; the map's folded name),
/// `BSRenderedTexture::GetTexture` (`004bc320`, `RET 4`),
/// `InterfaceManager::GetMenuHeight` (`00717860`, `ST0`), the tile float getter
/// (`00a011b0`, `RET 4`), `ImageSpaceManager::CreatePartialScreenGeometry` (`00b8c3b0`,
/// `RET 0xc`) and the image-space effect render (`00b97550`, `RET 0x18`).
const POINTER_ASSIGN_006E5CC0: u32 = 0x006e_5cc0;
const RENDERED_TEXTURE_GET_TEXTURE: u32 = 0x004b_c320;
const GET_MENU_HEIGHT: u32 = 0x0071_7860;
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
const CREATE_PARTIAL_SCREEN_GEOMETRY: u32 = 0x00b8_c3b0;
const IMAGE_SPACE_RENDER_00B97550: u32 = 0x00b9_7550;

// Translated from 00705fe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00705fe0 - the C++ exception-unwinding frame (FS:[0] around the base) is not translated.
/// `FORenderedMenu::FORenderedMenu` in the engine map, by its body the
/// `FORenderedTerminal` constructor: the `FORenderedMenu` base (`007faa20`), the
/// `FORenderedTerminal` vtable `0106ebe4`, `OriginalPos` (+0xd8, `006815c0`) and
/// `spExitButton` (+0xe4, `NiPointer` with 0). Returns `this`.
pub fn fn_00705fe0(e: &mut Engine, this: Ptr<FORenderedMenu>) -> Ptr<FORenderedMenu> {
    e.call(RENDERED_MENU_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), RENDERED_TERMINAL_VTABLE);
    e.call(POINT_INIT_006815C0, &args![this.addr() + 0xd8]);
    e.call(POINTER_INIT, &args![this.addr() + 0xe4, 0u32]);
    this
}

// Translated from 007faef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): `bMenuRendering` (+0xb9), in `AL`.
pub fn fn_007faef0(e: &mut Engine, this: Ptr<FORenderedMenu>) -> u8 {
    e.get(this, FORenderedMenu::bMenuRendering)
}

// Translated from 007fb250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 8`: two `float`s; starts a vertical hold): sets
/// `bVertHoldActive`, `fVertHoldDuration` (+0x68) to the first, `fVertHoldSpeed` (+0x6c)
/// to the second times the `double` at `01016978`, `fVertHoldStartTime` (+0x64) to now
/// in milliseconds and `fVertHoldCurrentPosition` (+0x70) to 0.
pub fn fn_007fb250(e: &mut Engine, this: Ptr<FORenderedMenu>, duration: f32, speed: f32) {
    let base = this.addr();
    e.set(this, FORenderedMenu::bVertHoldActive, 1);
    e.mem
        .set_u32(base + 0x68, x87_quiet_bits(duration.to_bits()));
    let scale: f64 = e.global(VERT_HOLD_SPEED_SCALE);
    e.mem.set_u32(
        base + 0x6c,
        x87_store32(x87_mul(x87_load32(speed.to_bits()), scale)),
    );
    let now = now_milliseconds(e);
    e.mem.set_u32(base + 0x64, now);
    e.mem.set_u32(base + 0x70, x87_store32(0.0));
}

// Translated from 007fb2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 0xc`: three `float`s; starts a shudder): sets
/// `bShudderActive`, `fShudderDuration` (+0x7c), `fShudderIntensity` (+0x80),
/// `fShudderFrequency` (+0x84), `fShudderStartTime` (+0x78) to now in milliseconds and
/// `fShudderCurrentPosition` (+0x88) to 0.
pub fn fn_007fb2b0(
    e: &mut Engine,
    this: Ptr<FORenderedMenu>,
    duration: f32,
    intensity: f32,
    frequency: f32,
) {
    let base = this.addr();
    e.set(this, FORenderedMenu::bShudderActive, 1);
    e.mem
        .set_u32(base + 0x7c, x87_quiet_bits(duration.to_bits()));
    e.mem
        .set_u32(base + 0x80, x87_quiet_bits(intensity.to_bits()));
    e.mem
        .set_u32(base + 0x84, x87_quiet_bits(frequency.to_bits()));
    let now = now_milliseconds(e);
    e.mem.set_u32(base + 0x78, now);
    e.mem.set_u32(base + 0x88, x87_store32(0.0));
}

// Translated from 007fb310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: the time; the distortion update, slot +0x4c by its place
/// in `OnIdle`): `fDistortPosition` (+0x9c) is the time since `fDistortStartTime` over
/// `fDistortDuration` (+0xa0); past the setting at `011db520` plus 1.0 the distortion
/// stops and `fNextDistortTime` becomes the time plus `rand() %` the setting at
/// `011db4f8`.
pub fn fn_007fb310(e: &mut Engine, this: Ptr<FORenderedMenu>, time: f32) {
    let base = this.addr();
    let elapsed = x87_store32(x87_sub(
        x87_load32(time.to_bits()),
        x87_m32(e.mem.u32(base + 0x98)),
    ));
    e.mem.set_u32(
        base + 0x9c,
        x87_store32(x87_div(
            x87_load32(elapsed),
            x87_m32(e.mem.u32(base + 0xa0)),
        )),
    );
    let length = setting_float(e, DISTORT_LENGTH_SETTING);
    let limit = x87_add(length, e.global::<f64>(ONE_DOUBLE));
    let position = x87_load32(e.mem.u32(base + 0x9c));
    // FCOMPP (position against the limit); TEST AH,0x41; JNZ: only when above it.
    if position > limit {
        e.set(this, FORenderedMenu::bDistortActive, 0);
        let random = e.call(RAND, &args![]).i32();
        let interval = e
            .call(SETTING_INT_VALUE, &args![DISTORT_INTERVAL_SETTING])
            .u32();
        let part = random.wrapping_rem(e.mem.i32(interval));
        e.mem.set_u32(
            base + 0xa4,
            x87_store32(x87_add(f64::from(part), x87_m32(time.to_bits()))),
        );
    }
}

// Translated from 007fb3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: the time; the shudder update, slot +0x40): past
/// `fShudderDuration` the shudder stops; otherwise `fShudderCurrentPosition` (+0x88) is
/// `004e44d0(elapsed * frequency) * intensity * (1 - elapsed / duration)`.
pub fn fn_007fb3a0(e: &mut Engine, this: Ptr<FORenderedMenu>, time: f32) {
    let base = this.addr();
    let elapsed = x87_store32(x87_sub(
        x87_load32(time.to_bits()),
        x87_m32(e.mem.u32(base + 0x78)),
    ));
    let duration = x87_load32(e.mem.u32(base + 0x7c));
    // FCOMPP (duration against elapsed); TEST AH,5; JP: stops when the duration is below.
    if duration < x87_load32(elapsed) {
        e.set(this, FORenderedMenu::bShudderActive, 0);
        return;
    }
    let left = x87_store32(x87_sub(
        1.0,
        x87_div(x87_load32(elapsed), x87_m32(e.mem.u32(base + 0x7c))),
    ));
    let phase = x87_store32(x87_mul(
        x87_load32(elapsed),
        x87_m32(e.mem.u32(base + 0x84)),
    ));
    let wave = e
        .call(FLOAT_FUNCTION_004E44D0, &args![x87_quiet_bits(phase)])
        .f64();
    let value = x87_mul(wave, x87_m32(e.mem.u32(base + 0x80)));
    e.mem
        .set_u32(base + 0x88, x87_store32(x87_mul(value, x87_m32(left))));
}

// Translated from 007fb420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: the time; the burst update, slot +0x38): past
/// `fBurstDuration` the burst stops; otherwise `fBurstCurrentIntensity` is
/// `fBurstMaximumIntensity * (1 - elapsed / duration)`.
pub fn fn_007fb420(e: &mut Engine, this: Ptr<FORenderedMenu>, time: f32) {
    let base = this.addr();
    let elapsed = x87_store32(x87_sub(
        x87_load32(time.to_bits()),
        x87_m32(e.mem.u32(base + 0x50)),
    ));
    let duration = x87_load32(e.mem.u32(base + 0x54));
    // FCOMPP (duration against elapsed); TEST AH,5; JP: stops when the duration is below.
    if duration < x87_load32(elapsed) {
        e.set(this, FORenderedMenu::bBurstActive, 0);
        return;
    }
    let left = x87_store32(x87_sub(
        1.0,
        x87_div(x87_load32(elapsed), x87_m32(e.mem.u32(base + 0x54))),
    ));
    e.mem.set_u32(
        base + 0x5c,
        x87_store32(x87_mul(x87_load32(e.mem.u32(base + 0x58)), x87_m32(left))),
    );
}

// Translated from 007fb480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: the time; the vertical hold update, slot +0x3c): past
/// `fVertHoldDuration` the hold stops and a shudder starts (`007fb2b0` with the floats at
/// `0101b2d0`, `010181bc`, `010181bc`); otherwise `fVertHoldCurrentPosition` is the
/// elapsed time times `fVertHoldSpeed`, wrapped into [0, 1) (less `00404040` of it) when
/// at least 1.0 or below 0.0.
pub fn fn_007fb480(e: &mut Engine, this: Ptr<FORenderedMenu>, time: f32) {
    let base = this.addr();
    let elapsed = x87_store32(x87_sub(
        x87_load32(time.to_bits()),
        x87_m32(e.mem.u32(base + 0x64)),
    ));
    let duration = x87_load32(e.mem.u32(base + 0x68));
    if duration < x87_load32(elapsed) {
        e.set(this, FORenderedMenu::bVertHoldActive, 0);
        let length = f32::from_bits(x87_quiet_bits(e.global::<u32>(SHUDDER_DURATION_0101B2D0)));
        let value = f32::from_bits(x87_quiet_bits(e.global::<u32>(SHUDDER_VALUE_010181BC)));
        fn_007fb2b0(e, this, length, value, value);
        return;
    }
    e.mem.set_u32(
        base + 0x70,
        x87_store32(x87_mul(
            x87_load32(elapsed),
            x87_m32(e.mem.u32(base + 0x6c)),
        )),
    );
    let position = x87_load32(e.mem.u32(base + 0x70));
    // FCOMP 1.0; TEST AH,1; JZ: wraps at 1.0 and above; else FCOMP 0.0; TEST AH,5; JP:
    // wraps below 0.0 (neither for NaN).
    let wraps = position >= e.global::<f64>(ONE_DOUBLE) || position < e.global::<f64>(ZERO_DOUBLE);
    if wraps {
        let whole = e
            .call(
                WRAP_00404040,
                &args![x87_quiet_bits(e.mem.u32(base + 0x70))],
            )
            .f64();
        e.mem.set_u32(
            base + 0x70,
            x87_store32(x87_sub(x87_m32(e.mem.u32(base + 0x70)), whole)),
        );
    }
}

// Translated from 007fb730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall` on an `NiColor`, `RET 8`: the result and the other color): the
/// component-wise sum (each an `FLD`/`FADD`/`FSTP` of `float`s) written to the result,
/// which is returned.
pub fn fn_007fb730(e: &mut Engine, this: u32, result: u32, other: u32) -> u32 {
    let mut sum = [0u32; 3];
    for (i, word) in sum.iter_mut().enumerate() {
        let offset = 4 * i as u32;
        *word = e.mem.u32(this + offset);
    }
    for (i, word) in sum.iter_mut().enumerate() {
        let offset = 4 * i as u32;
        *word = x87_store32(x87_add(
            x87_load32(*word),
            x87_m32(e.mem.u32(other + offset)),
        ));
    }
    for (i, word) in sum.iter().enumerate() {
        e.mem.set_u32(result + 4 * i as u32, *word);
    }
    result
}

// Translated from 007fbeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 8`: a `bool` and a pointer): stores the flag at +0x1fc and
/// the pointer in the `NiPointer` at +0x1f8 (`0066b0d0`).
pub fn fn_007fbeb0(e: &mut Engine, this: u32, flag: u8, pointer: u32) {
    e.mem.set_u8(this + 0x1fc, flag);
    e.call(POINTER_SET, &args![this + 0x1f8, pointer]);
}

// Translated from 007fbee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedMenu::DoEffect` (Xbox PDB, `RET 0xc`: a rendered texture and two words):
/// fills the image-space effect's statics from the menu: the blur radius and intensity
/// and the scanline scale (+0x40..+0x48), no vertical offset, the scanlines texture
/// (+0xbc into `01200810`), the texture of the rendered texture (`01200818`), the height
/// (`GetMenuHeight` for the Pip-Boy menu, else the menu tile's trait `0xfb0`); a burst
/// scales the blur by `1 + fBurstCurrentIntensity`; a vertical hold sets the offset to
/// `height * position`, a shudder adds `height * position`; the pulse adds to the blur;
/// the distortion copies its settings (`011db520`, `011db464`), its position and sets
/// its flag, or clears them (position -1.0); the tint color (+0xc0) is copied. With
/// partial-screen geometry it is made when missing (`CreatePartialScreenGeometry` with
/// the height twice) and handed to `004e3270()` (`007fbeb0` with 1). Stores the third
/// word, renders effect 0xa (`00b97550` with the renderer `0043c4b0`, the texture, the
/// second word, 0 and 1) and clears `01200818`.
pub fn fo_rendered_menu_do_effect(
    e: &mut Engine,
    this: Ptr<FORenderedMenu>,
    texture: u32,
    second: u32,
    third: u32,
) {
    let base = this.addr();
    e.call(INTERFACE_GET_004B7210, &args![]);
    for (offset, global) in [
        (0x40u32, EFFECT_BLUR_RADIUS),
        (0x44, EFFECT_BLUR_INTENSITY),
        (0x48, EFFECT_SCANLINE_SCALE),
    ] {
        let value = x87_quiet_bits(e.mem.u32(base + offset));
        e.set_global(global, value);
    }
    e.set_global(EFFECT_VERTICAL_OFFSET, x87_store32(0.0));
    e.call(
        POINTER_ASSIGN_006E5CC0,
        &args![EFFECT_SCANLINES_POINTER, base + SCANLINES_TEXTURE],
    );
    let inner = e
        .call(RENDERED_TEXTURE_GET_TEXTURE, &args![texture, 0u32])
        .u32();
    e.call(POINTER_SET, &args![EFFECT_TEXTURE_POINTER, inner]);
    let pipboy = e.call(INTERFACE_GET_PIPBOY, &args![]).u32();
    let height = if base == pipboy {
        e.call(GET_MENU_HEIGHT, &args![]).f64()
    } else {
        let tile = e.mem.u32(base + 0x18);
        e.call(TILE_GET_FLOAT, &args![tile, 0xfb0u32]).f64()
    };
    e.set_global(EFFECT_HEIGHT, x87_store32(height));
    if e.get(this, FORenderedMenu::bBurstActive) != 0 {
        for global in [EFFECT_BLUR_INTENSITY, EFFECT_BLUR_RADIUS] {
            let factor = x87_add(x87_load32(e.mem.u32(base + 0x5c)), 1.0);
            let value = x87_mul(factor, x87_m32(e.global::<u32>(global)));
            e.set_global(global, x87_store32(value));
        }
    }
    if e.get(this, FORenderedMenu::bVertHoldActive) != 0 {
        let height = e.call(GET_MENU_HEIGHT, &args![]).f64();
        let value = x87_mul(height, x87_m32(e.mem.u32(base + 0x70)));
        e.set_global(EFFECT_VERTICAL_OFFSET, x87_store32(value));
    }
    if e.get(this, FORenderedMenu::bShudderActive) != 0 {
        let height = e.call(GET_MENU_HEIGHT, &args![]).f64();
        let value = x87_mul(height, x87_m32(e.mem.u32(base + 0x88)));
        let value = x87_add(value, x87_m32(e.global::<u32>(EFFECT_VERTICAL_OFFSET)));
        e.set_global(EFFECT_VERTICAL_OFFSET, x87_store32(value));
    }
    for (global, offset) in [(EFFECT_BLUR_INTENSITY, 0x8cu32), (EFFECT_BLUR_RADIUS, 0x90)] {
        let value = x87_add(
            x87_load32(e.global::<u32>(global)),
            x87_m32(e.mem.u32(base + offset)),
        );
        e.set_global(global, x87_store32(value));
    }
    if e.get(this, FORenderedMenu::bDistortActive) != 0 {
        let length = e.call(SETTING_VALUE, &args![DISTORT_LENGTH_SETTING]).u32();
        let length = x87_quiet_bits(e.mem.u32(length));
        e.set_global(EFFECT_DISTORT_LENGTH, length);
        let setting = e.call(SETTING_VALUE, &args![DISTORT_EFFECT_SETTING]).u32();
        let setting = x87_quiet_bits(e.mem.u32(setting));
        e.set_global(EFFECT_DISTORT_SETTING, setting);
        let position = x87_quiet_bits(e.mem.u32(base + 0x9c));
        e.set_global(EFFECT_DISTORT_POSITION, position);
        e.set_global(EFFECT_DISTORT_FLAG, 1u8);
    } else {
        e.set_global(EFFECT_DISTORT_LENGTH, x87_store32(0.0));
        e.set_global(EFFECT_DISTORT_SETTING, x87_store32(0.0));
        let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE_FLOAT));
        e.set_global(EFFECT_DISTORT_POSITION, minus_one);
    }
    for i in 0..4 {
        let word = e.mem.u32(base + 0xc0 + 4 * i);
        e.set_global(EFFECT_TINT + 4 * i, word);
    }
    if e.get(this, FORenderedMenu::bUsePartialScreenGeometry) != 0
        && e.call(POINTER_GET, &args![base + 0xc]).u32() == 0
    {
        let first = e.call(GET_MENU_HEIGHT, &args![]).f64();
        let first = x87_store32(first);
        let second_height = e.call(GET_MENU_HEIGHT, &args![]).f64();
        let second_height = x87_store32(second_height);
        let manager = e.call(GETTER_004E3270, &args![]).u32();
        e.call(
            CREATE_PARTIAL_SCREEN_GEOMETRY,
            &args![manager, second_height, first, base + 0xc],
        );
    }
    if e.get(this, FORenderedMenu::bUsePartialScreenGeometry) != 0 {
        let geometry = e.call(POINTER_GET, &args![base + 0xc]).u32();
        let manager = e.call(GETTER_004E3270, &args![]).u32();
        fn_007fbeb0(e, manager, 1, geometry);
    }
    e.set_global(EFFECT_THIRD_012007EC, third);
    let renderer = e.call(RENDERER_0043C4B0, &args![]).u32();
    let manager = e.call(GETTER_004E3270, &args![]).u32();
    e.call(
        IMAGE_SPACE_RENDER_00B97550,
        &args![manager, 0xau32, renderer, texture, second, 0u32, 1u32],
    );
    e.call(POINTER_SET, &args![EFFECT_TEXTURE_POINTER, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0070_5fe0,
            fn_00705fe0(Ptr<FORenderedMenu>) -> Ptr<FORenderedMenu>
        ),
        entry!(0x007f_aef0, fn_007faef0(Ptr<FORenderedMenu>) -> u8),
        entry!(0x007f_b250, fn_007fb250(Ptr<FORenderedMenu>, f32, f32)),
        entry!(0x007f_b2b0, fn_007fb2b0(Ptr<FORenderedMenu>, f32, f32, f32)),
        entry!(0x007f_b310, fn_007fb310(Ptr<FORenderedMenu>, f32)),
        entry!(0x007f_b3a0, fn_007fb3a0(Ptr<FORenderedMenu>, f32)),
        entry!(0x007f_b420, fn_007fb420(Ptr<FORenderedMenu>, f32)),
        entry!(0x007f_b480, fn_007fb480(Ptr<FORenderedMenu>, f32)),
        entry!(0x007f_b730, fn_007fb730(u32, u32, u32) -> u32),
        entry!(0x007f_beb0, fn_007fbeb0(u32, u8, u32)),
        entry!(
            0x007f_bee0,
            fo_rendered_menu_do_effect(Ptr<FORenderedMenu>, u32, u32, u32)
        ),
        entry!(
            0x007f_ae70,
            fo_rendered_menu_set_screen_texture(Ptr<FORenderedMenu>, u32)
        ),
        entry!(
            0x007f_af10,
            fo_rendered_menu_on_idle(Ptr<FORenderedMenu>) -> bool
        ),
        entry!(
            0x007f_b040,
            fo_rendered_menu_on_menu_open(Ptr<FORenderedMenu>)
        ),
        entry!(0x007f_b0b0, fn_007fb0b0(Ptr<FORenderedMenu>)),
        entry!(0x007f_b100, fn_007fb100(Ptr<FORenderedMenu>)),
        entry!(
            0x007f_b150,
            fo_rendered_menu_trigger_burst(Ptr<FORenderedMenu>, f32, f32, f32)
        ),
        entry!(0x007f_b530, fn_007fb530(Ptr<FORenderedMenu>, f32)),
        entry!(0x007f_b5a0, fn_007fb5a0(Ptr<FORenderedMenu>, f32)),
        entry!(
            0x007f_b790,
            fn_007fb790(Ptr<FORenderedMenu>, u32, u32, u32) -> bool
        ),
        entry!(0x007f_ba00, fn_007fba00(Ptr<FORenderedMenu>, u32, u32, u32)),
        entry!(
            0x007f_c150,
            fo_rendered_menu_initialize(Ptr<FORenderedMenu>)
        ),
        entry!(0x007f_c2b0, fn_007fc2b0(Ptr<FORenderedMenu>, u32)),
        entry!(
            0x007f_c2d0,
            fo_rendered_menu_prepare_geometry(Ptr<FORenderedMenu>, u32)
        ),
        entry!(
            0x007f_c400,
            fo_rendered_menu_create_menu_light(Ptr<FORenderedMenu>, u32, u32) -> bool
        ),
        entry!(
            0x007f_c6f0,
            fo_rendered_menu_clear_menu_light(Ptr<FORenderedMenu>)
        ),
        entry!(0x007f_c790, fn_007fc790(Ptr<FORenderedMenu>)),
        entry!(0x007f_c8d0, fn_007fc8d0(Ptr<FORenderedMenu>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    fn answer(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    fn answer_float(e: &mut Engine, addr: u32, st0: f64) {
        e.register_double(addr, move |_, _| Ret {
            st0,
            ..Ret::default()
        });
    }

    fn calls(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An object whose vtable has `slots` (byte offset, target).
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        this
    }

    /// `NiPointer` reads return the word the pointer holds.
    fn pointers(e: &mut Engine) {
        e.register(POINTER_GET, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(POINTER_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
    }

    fn quiet_all(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            answer(e, *a, 0);
        }
    }

    fn menu(e: &mut Engine, slots: &[(u32, u32)]) -> Ptr<FORenderedMenu> {
        Ptr::new(object(e, slots))
    }

    #[test]
    fn set_screen_texture_needs_initialization_and_geometry() {
        let mut e = Engine::new();
        pointers(&mut e);
        quiet_all(&mut e, &[DEBUG_MESSAGE, PROPERTY_SET_TEXTURE_00438230]);
        answer(&mut e, PROPERTY_TYPE_00438220, 0x11);
        answer(&mut e, NI_AV_OBJECT_GET_PROPERTY, 0x7000);
        let this = menu(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x007f_ae70, &args![this, 0x55u32]);
        assert!(calls(&e, POINTER_GET).is_empty());
        e.set(this, FORenderedMenu::bInitialized, 1);
        e.call(0x007f_ae70, &args![this, 0x55u32]);
        assert_eq!(calls(&e, DEBUG_MESSAGE), vec![vec![NO_SCREEN_MESSAGE]]);
        e.set(this, FORenderedMenu::spScreenTriShape, 0x6000);
        e.set(this, FORenderedMenu::spDefaultTex, 0x6100);
        e.call_log = Some(vec![]);
        e.call(0x007f_ae70, &args![this, 0x55u32]);
        e.call(0x007f_ae70, &args![this, 0u32]);
        assert_eq!(
            calls(&e, NI_AV_OBJECT_GET_PROPERTY),
            vec![vec![0x6000, 0x11], vec![0x6000, 0x11]]
        );
        assert_eq!(
            calls(&e, PROPERTY_SET_TEXTURE_00438230),
            vec![vec![0x7000, 0x55], vec![0x7000, 0x6100]]
        );
    }

    const SLOT_BASE: u32 = 0x0310_0000;

    fn idle_menu(e: &mut Engine, open: u32) -> Ptr<FORenderedMenu> {
        let slots: Vec<(u32, u32)> = (0x38..=0x4c)
            .step_by(4)
            .map(|s| (s, SLOT_BASE + s))
            .collect();
        for (_, target) in &slots {
            answer(e, *target, 0);
        }
        let this = menu(e, &slots);
        answer(e, TIMER_GET_00825C00, 1500);
        answer(e, INTERFACE_GET_004B7210, 0x5000);
        answer(e, INTERFACE_TEST_007079F0, open);
        e.call_log = Some(vec![]);
        this
    }

    fn slot_calls(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 & 0xfff0_0000 == SLOT_BASE)
            .map(|c| c.0 - SLOT_BASE)
            .collect()
    }

    #[test]
    fn on_idle_does_nothing_while_closed() {
        let mut e = Engine::new();
        let this = idle_menu(&mut e, 0);
        assert!(!e.call(0x007f_af10, &args![this]).bool());
        assert!(slot_calls(&e).is_empty());
        assert_eq!(calls(&e, TIMER_GET_00825C00), vec![vec![TIMER]]);
    }

    #[test]
    fn on_idle_runs_the_active_effects_with_the_time() {
        let mut e = Engine::new();
        let this = idle_menu(&mut e, 1);
        e.set(this, FORenderedMenu::bBurstActive, 1);
        e.set(this, FORenderedMenu::bShudderActive, 1);
        e.set(this, FORenderedMenu::bDistortActive, 1);
        assert!(e.call(0x007f_af10, &args![this]).bool());
        assert_eq!(slot_calls(&e), vec![0x48, 0x38, 0x40, 0x44, 0x4c]);
        assert_eq!(e.get(this, FORenderedMenu::bMenuOpen), 1);
        let time = calls(&e, SLOT_BASE + 0x44);
        assert_eq!(time[0][1], 1500.0f32.to_bits());
    }

    #[test]
    fn on_idle_starts_a_distortion_once_its_time_passed() {
        for (next, starts) in [(1000.0f32, true), (1500.0, false), (f32::NAN, false)] {
            let mut e = Engine::new();
            // Just closed: still runs once.
            let this = idle_menu(&mut e, 0);
            e.set(this, FORenderedMenu::bMenuOpen, 1);
            e.set(this, FORenderedMenu::fNextDistortTime, next);
            e.set(this, FORenderedMenu::bVertHoldActive, 1);
            assert!(e.call(0x007f_af10, &args![this]).bool());
            assert_eq!(slot_calls(&e), vec![0x48, 0x3c, 0x44]);
            assert_eq!(
                e.get(this, FORenderedMenu::bDistortActive),
                u8::from(starts)
            );
            if starts {
                assert_eq!(e.get(this, FORenderedMenu::fDistortStartTime), 1500.0);
            }
        }
    }

    #[test]
    fn on_menu_open_and_close_show_and_hide_the_light() {
        let mut e = Engine::new();
        pointers(&mut e);
        quiet_all(
            &mut e,
            &[
                TILE_UPDATE_CLIPWINDOWS,
                SET_APP_CULLED_00450F90,
                MENU_00867A20,
            ],
        );
        answer(&mut e, GETTER_009611E0, 0x4100);
        answer(&mut e, TILE_GET_MENU, 0x4200);
        answer(&mut e, INTERFACE_GET_MENUS_ROOT, 0x4300);
        let this = menu(&mut e, &[]);
        e.set(this, FORenderedMenu::spLight, 0x4400);
        e.call_log = Some(vec![]);
        e.call(0x007f_b040, &args![this]);
        assert_eq!(calls(&e, MENU_00867A20), vec![vec![0x4200]]);
        assert_eq!(calls(&e, TILE_UPDATE_CLIPWINDOWS), vec![vec![0x4300]]);
        assert_eq!(calls(&e, SET_APP_CULLED_00450F90), vec![vec![0x4400, 0]]);
        assert_eq!(e.get(this, FORenderedMenu::bMenuRendering), 1);
        e.set(this, FORenderedMenu::bBurstActive, 1);
        e.set(this, FORenderedMenu::bVertHoldActive, 1);
        e.call_log = Some(vec![]);
        e.call(0x007f_b0b0, &args![this]);
        assert_eq!(e.get(this, FORenderedMenu::bMenuRendering), 0);
        assert_eq!(e.get(this, FORenderedMenu::bBurstActive), 0);
        assert_eq!(e.get(this, FORenderedMenu::bVertHoldActive), 0);
        assert_eq!(calls(&e, SET_APP_CULLED_00450F90), vec![vec![0x4400, 1]]);
        // No object: the menu is not told.
        answer(&mut e, GETTER_009611E0, 0);
        e.call_log = Some(vec![]);
        e.call(0x007f_b040, &args![this]);
        assert!(calls(&e, MENU_00867A20).is_empty());
    }

    #[test]
    fn trigger_burst_sets_the_burst_and_plays_its_sound() {
        for (sounds, chance, plays) in [(0u8, 1u32, false), (1, 0, false), (1, 1, true)] {
            let mut e = Engine::new();
            answer(&mut e, TIMER_GET_00825C00, 250);
            e.map(SOUNDS_ACTIVE_011DB378, 1);
            e.mem.set_u8(SOUNDS_ACTIVE_011DB378, sounds);
            answer(&mut e, FLOAT_TEST_004DFF00, chance);
            answer(&mut e, BS_AUDIO_Q_INSTANCE, 0x5800);
            answer(&mut e, BS_AUDIO_GET_SOUND_HANDLE_BY_NAME, 0x5900);
            quiet_all(
                &mut e,
                &[
                    SOUND_HANDLE_INIT,
                    SOUND_HANDLE_ASSIGN,
                    SOUND_HANDLE_DESTROY,
                    BS_SOUND_HANDLE_PLAY,
                ],
            );
            let this = menu(&mut e, &[]);
            e.call_log = Some(vec![]);
            e.call(0x007f_b150, &args![this, 2.0f32, 0.75f32, 0.5f32]);
            assert_eq!(e.get(this, FORenderedMenu::bBurstActive), 1);
            assert_eq!(e.get(this, FORenderedMenu::fBurstDuration), 2.0);
            assert_eq!(e.get(this, FORenderedMenu::fBurstCurrentIntensity), 0.75);
            assert_eq!(e.get(this, FORenderedMenu::fBurstMaximumIntensity), 0.75);
            assert_eq!(e.get(this, FORenderedMenu::fBurstStartTime), 250.0);
            assert_eq!(calls(&e, FLOAT_TEST_004DFF00).len(), usize::from(sounds));
            let named = calls(&e, BS_AUDIO_GET_SOUND_HANDLE_BY_NAME);
            assert_eq!(!named.is_empty(), plays);
            if plays {
                assert_eq!(named[0][0], 0x5800);
                assert_eq!(named[0][2..], [PIPBOY_STATIC_SOUND, 0x121]);
                let sound = calls(&e, SOUND_HANDLE_INIT)[0][0];
                assert_eq!(calls(&e, SOUND_HANDLE_ASSIGN), vec![vec![sound, 0x5900]]);
                assert_eq!(calls(&e, BS_SOUND_HANDLE_PLAY), vec![vec![sound, 0]]);
                assert_eq!(calls(&e, SOUND_HANDLE_DESTROY).len(), 2);
            }
        }
    }

    /// Settings whose value lives at `setting + 8`, filled with `values`.
    fn settings(e: &mut Engine, values: &[(u32, f32)]) {
        for (setting, value) in values {
            e.map(*setting, 0x10);
            e.mem.set_f32(setting + 8, *value);
        }
        e.register(SETTING_VALUE, |_, a| Ret {
            eax: a[0] + 8,
            ..Ret::default()
        });
    }

    #[test]
    fn pulse_update_scales_the_wave_by_the_settings() {
        let mut e = Engine::new();
        settings(
            &mut e,
            &[(0x011d_b408, 2.0), (0x011d_b3a4, 3.0), (0x011d_b424, 0.5)],
        );
        e.register(FLOAT_FUNCTION_00C5D320, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])) + 1.0,
            ..Ret::default()
        });
        let this = menu(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x007f_b530, &args![this, 1.5f32]);
        // 1.5 * 2 = 3; wave 4; doubled 8.
        assert_eq!(
            calls(&e, FLOAT_FUNCTION_00C5D320),
            vec![vec![3.0f32.to_bits()]]
        );
        assert_eq!(e.get(this, FORenderedMenu::fPulseCurrentBrighten), 24.0);
        assert_eq!(e.get(this, FORenderedMenu::fPulseCurrentRadius), 4.0);
    }

    fn light_engine() -> Engine {
        let mut e = Engine::new();
        pointers(&mut e);
        e.map(BASE_LIGHT_COLOR, 12);
        for i in 0..3 {
            e.mem.set_u32(BASE_LIGHT_COLOR + 4 * i, 0x100 + i);
        }
        e.map(HALF, 8);
        e.mem.set_f64(HALF, 0.5);
        settings(
            &mut e,
            &[(0x011d_b430, 2.0), (0x011d_b4a4, 4.0), (0x011d_b3e8, 1.0)],
        );
        // The color scale writes its factor into the result's first word.
        e.register(COLOR_SCALE_00633B40, |e, a| {
            e.mem.set_u32(a[1], a[2]);
            e.mem.set_u32(a[1] + 4, a[0]);
            e.mem.set_u32(a[1] + 8, 0);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(FLOAT_FUNCTION_004E44D0, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
        quiet_all(
            &mut e,
            &[
                COLOR_COMBINE_007FB730,
                LIGHT_SET_COLOR_004BC2E0,
                UPDATE_DATA_INIT_0043D410,
                UPDATE_00A59C60,
            ],
        );
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn light_update_without_pulse_uses_the_base_color() {
        let mut e = light_engine();
        let this = menu(&mut e, &[]);
        e.call(0x007f_b5a0, &args![this, 1.0f32]);
        assert!(calls(&e, COLOR_SCALE_00633B40).is_empty());
        e.set(this, FORenderedMenu::spLight, 0x4400);
        e.set(this, FORenderedMenu::fScreenLightBaseIntensity, 0.25);
        e.call(0x007f_b5a0, &args![this, 1.0f32]);
        let scaled = calls(&e, COLOR_SCALE_00633B40);
        assert_eq!(scaled.len(), 1);
        assert_eq!(scaled[0][0], this.addr() + SCREEN_LIGHT_COLOR);
        assert_eq!(scaled[0][2], 0.25f32.to_bits());
        let combined = calls(&e, COLOR_COMBINE_007FB730);
        let color = combined[0][2];
        assert_eq!(combined[0][0], scaled[0][1]);
        assert_eq!(
            (0..3).map(|i| e.mem.u32(color + 4 * i)).collect::<Vec<_>>(),
            vec![0x100, 0x101, 0x102]
        );
        assert_eq!(
            calls(&e, LIGHT_SET_COLOR_004BC2E0),
            vec![vec![0x4400, combined[0][1]]]
        );
        let update = calls(&e, UPDATE_DATA_INIT_0043D410);
        assert_eq!(update[0][1..], [0, 0, 0]);
        assert_eq!(calls(&e, UPDATE_00A59C60), vec![vec![0x4400, update[0][0]]]);
    }

    #[test]
    fn light_update_with_pulse_scales_by_burst_pulse_and_hold() {
        let mut e = light_engine();
        let this = menu(&mut e, &[]);
        e.set(this, FORenderedMenu::spLight, 0x4400);
        e.set(this, FORenderedMenu::bPulseAffectsLight, 1);
        e.set(this, FORenderedMenu::bBurstActive, 1);
        e.set(this, FORenderedMenu::fBurstCurrentIntensity, 0.5);
        e.set(this, FORenderedMenu::fPulseCurrentBrighten, 0.25);
        e.set(this, FORenderedMenu::bVertHoldActive, 1);
        e.set(this, FORenderedMenu::fScreenLightFlickerIntensity, 3.0);
        e.call(0x007f_b5a0, &args![this, 1.5f32]);
        // (0.5 * 2 + 0.25 * 4) * (1.5 * 1 + 0.5) = 4.
        let scaled = calls(&e, COLOR_SCALE_00633B40);
        assert_eq!(scaled.len(), 3);
        assert_eq!(scaled[0][2], 4.0f32.to_bits());
        assert_eq!(scaled[1][0], scaled[0][1]);
        assert_eq!(scaled[1][2], 3.0f32.to_bits());
        // The color is the second scale's result.
        let color = calls(&e, COLOR_COMBINE_007FB730)[0][2];
        assert_eq!(e.mem.u32(color), 3.0f32.to_bits());
        assert_eq!(e.mem.u32(color + 4), scaled[0][1]);
    }

    fn pick_engine(visible: u32) -> (Engine, Ptr<FORenderedMenu>) {
        let mut e = Engine::new();
        pointers(&mut e);
        quiet_all(
            &mut e,
            &[
                SET_APP_CULLED_00450F90,
                POINT_INIT_006815C0,
                NI_PICK_CONSTRUCT,
                NI_PICK_DESTROY,
                PICK_00458B30,
                PICK_SET_TARGET_00705FC0,
                PICK_006ECD40,
                PICK_00792780,
                CAMERA_WINDOW_POINT_TO_RAY_00A71080,
                GETTER_0045C670,
            ],
        );
        answer(&mut e, INTERFACE_GET_004B7210, 0x5000);
        answer(&mut e, CAMERA_008B6200, 0x5100);
        answer_float(&mut e, INTERFACE_REAL_00792760, 10.0);
        answer_float(&mut e, INTERFACE_REAL_009A9350, 20.0);
        e.register(FTOL, |_, a| Ret {
            eax: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32,
            ..Ret::default()
        });
        answer(&mut e, NI_PICK_PICK_OBJECTS, 1);
        answer(&mut e, PICK_RESULTS_00500940, 0x5200);
        answer(&mut e, PICK_RECORD_00968670, 0x5300);
        answer(&mut e, RECORD_OBJECT_00458B50, 0x6000);
        let coordinates = e.mem.alloc(8);
        e.mem.set_f32(coordinates, 0.5);
        e.mem.set_f32(coordinates + 4, 0.25);
        answer(&mut e, RECORD_TEXTURE_COORDINATES_00891170, coordinates);
        answer(&mut e, INTERFACE_IS_MENU_ID_VISIBLE, visible);
        let renderer = object(&mut e, &[(0xd4, 0x0310_1000)]);
        answer(&mut e, 0x0310_1000, 0x5400);
        answer(&mut e, RENDERER_0043C4B0, renderer);
        answer(&mut e, GETTER_0084E3A0, 600);
        for (addr, value) in [
            (PICK_U_SCALE, 0.5f64),
            (PICK_U_OFFSET, 0.25),
            (PICK_V_SCALE, 2.0),
            (PICK_V_OFFSET, 0.125),
            (ASPECT, 1.5),
        ] {
            e.map(addr, 8);
            e.mem.set_f64(addr, value);
        }
        let this = menu(&mut e, &[]);
        e.set(this, FORenderedMenu::spMenuRoot, 0x6100);
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn pick_needs_the_screen_geometry() {
        let (mut e, this) = pick_engine(0);
        let out = e.mem.alloc(8);
        assert!(!e.call(0x007f_b790, &args![this, 0u32, out, out + 4]).bool());
        assert_eq!(calls(&e, SET_APP_CULLED_00450F90), vec![vec![0x6100, 0]]);
        assert!(calls(&e, NI_PICK_CONSTRUCT).is_empty());
    }

    #[test]
    fn pick_returns_the_texture_coordinates_scaled_to_the_screen() {
        for (visible, u, v) in [(0u32, 450.0f32, 150.0f32), (1, 450.0, 375.0)] {
            let (mut e, this) = pick_engine(visible);
            e.set(this, FORenderedMenu::spScreenTriShape, 0x6000);
            let out = e.mem.alloc(8);
            assert!(e.call(0x007f_b790, &args![this, 0u32, out, out + 4]).bool());
            // Not visible: 0.5 * 600 * 1.5 and 0.25 * 600; visible: u = 0.5 * 0.5 + 0.25
            // (unchanged), v = 0.25 * 2 + 0.125 = 0.625.
            assert_eq!(e.mem.f32(out), u);
            assert_eq!(e.mem.f32(out + 4), v);
            let ray = calls(&e, CAMERA_WINDOW_POINT_TO_RAY_00A71080);
            assert_eq!(ray[0][..3], [0x5100, 20, 10]);
            let pick = calls(&e, NI_PICK_CONSTRUCT)[0][0];
            assert_eq!(calls(&e, NI_PICK_CONSTRUCT)[0][1..], [0, 8]);
            assert_eq!(
                calls(&e, PICK_SET_TARGET_00705FC0),
                vec![vec![pick, 0x6000]]
            );
            assert_eq!(calls(&e, PICK_RECORD_00968670), vec![vec![0x5200, 0]]);
            assert_eq!(calls(&e, NI_PICK_DESTROY), vec![vec![pick]]);
        }
        // Another object under the cursor: false.
        let (mut e, this) = pick_engine(0);
        e.set(this, FORenderedMenu::spScreenTriShape, 0x6001);
        let out = e.mem.alloc(8);
        assert!(!e.call(0x007f_b790, &args![this, 0u32, out, out + 4]).bool());
        assert_eq!(calls(&e, NI_PICK_DESTROY).len(), 1);
    }

    const RENDER_SLOTS: u32 = 0x0310_2000;

    fn render_engine() -> (Engine, Ptr<FORenderedMenu>) {
        let mut e = Engine::new();
        pointers(&mut e);
        let renderer = object(
            &mut e,
            &[(0xb4, RENDER_SLOTS + 0xb4), (0xac, RENDER_SLOTS + 0xac)],
        );
        answer(&mut e, RENDER_SLOTS + 0xb4, 0);
        answer(&mut e, RENDER_SLOTS + 0xac, 0);
        answer(&mut e, RENDERER_0043C4B0, renderer);
        answer(&mut e, INTERFACE_GET_004B7210, 0x5000);
        answer(&mut e, INTERFACE_TEST_00644640, 1);
        answer(&mut e, INTERFACE_NODE_005E3FC0, 0x5010);
        answer(&mut e, INTERFACE_007FA950, 0x5020);
        answer(&mut e, CULLER_008D80E0, 0x5030);
        answer(&mut e, GETTER_006629F0, 0x5040);
        answer(&mut e, TEXTURE_MANAGER_004A0EA0, 0x5050);
        answer(&mut e, TEXTURE_MANAGER_GET_RENDERED_00B6E110, 0x5060);
        answer(&mut e, TEST_004E9510, 0);
        let frustum = e.mem.alloc(0x1c);
        for i in 0..7 {
            e.mem.set_u32(frustum + 4 * i, 0x900 + i);
        }
        answer(&mut e, CAMERA_FRUSTUM_0045BBE0, frustum);
        answer(&mut e, INTERFACE_MENU_0045CD60, 0);
        answer(&mut e, INTERFACE_GET_DEBUG_TEXT_ROOT, 0);
        e.map(MENU_LIST_011D96C0, 4);
        answer(&mut e, GETTER_00726070, 0x5070);
        answer(&mut e, TILE_GET_MENU_TILE, 0x5080);
        answer(&mut e, NODE_0056C7F0, 0x5090);
        answer(&mut e, IS_APP_CULLED_00456610, 1);
        answer(&mut e, INTERFACE_GET_PIPBOY, 0);
        answer(&mut e, TILE_GET_MENU_BY_CLASS, 0);
        for (addr, value) in [
            (FRUSTUM_RIGHT, 1280.0f32),
            (FRUSTUM_TOP, 960.0),
            (FRUSTUM_FAR, 10000.0),
        ] {
            e.map(addr, 4);
            e.mem.set_f32(addr, value);
        }
        e.register(NI_CAMERA_SET_VIEW_FRUSTUM, |e, a| {
            // Record the frustum's words where the test can read them.
            let words: Vec<u32> = (0..7).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            let log = e.mem.alloc(0x1c);
            for (i, w) in words.iter().enumerate() {
                e.mem.set_u32(log + 4 * i as u32, *w);
            }
            Ret {
                eax: log,
                ..Ret::default()
            }
        });
        quiet_all(
            &mut e,
            &[
                SET_APP_CULLED_00450F90,
                CULLER_SET_ACCUMULATOR_004A0FD0,
                COLOR_A_INIT_00414430,
                POINTER_INIT,
                POINTER_DESTROY,
                RENDERER_SET_CAMERA_004E9BB0,
                CULLER_00C4F270,
                BS_SHADER_UTIL_ACCUMULATE_SCENE,
                RENDER_00B6C0D0,
                CULLER_00C4F2D0,
                SHADER_00B6B790,
                TEXTURE_MANAGER_RETURN_RENDERED,
                RENDER_SLOTS + 0x50,
                RENDER_SLOTS + 0x10,
            ],
        );
        let this = menu(
            &mut e,
            &[(0x50, RENDER_SLOTS + 0x50), (0x10, RENDER_SLOTS + 0x10)],
        );
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn render_needs_initialization_and_a_menu_root() {
        let (mut e, this) = render_engine();
        e.call(0x007f_ba00, &args![this, 1u32, 2u32, 0u32]);
        e.set(this, FORenderedMenu::bInitialized, 1);
        e.call(0x007f_ba00, &args![this, 1u32, 2u32, 0u32]);
        assert!(calls(&e, RENDERER_0043C4B0).is_empty());
        assert_eq!(calls(&e, POINTER_GET), vec![vec![this.addr() + 0x14]]);
    }

    #[test]
    fn render_draws_into_a_borrowed_texture_and_restores_the_state() {
        let (mut e, this) = render_engine();
        e.set(this, FORenderedMenu::bInitialized, 1);
        e.set(this, FORenderedMenu::spMenuRoot, 0x6100);
        e.mem.set_u32(this.addr() + ACCUMULATOR, 0x6200);
        e.call(0x007f_ba00, &args![this, 1u32, 2u32, 0u32]);
        let borrowed = calls(&e, TEXTURE_MANAGER_GET_RENDERED_00B6E110);
        assert_eq!(borrowed[0][2..], [0x31, 0, 0, 0]);
        assert_eq!(
            calls(&e, RENDER_SLOTS + 0x50),
            vec![vec![this.addr(), 0x5060, 1, 2]]
        );
        assert_eq!(
            calls(&e, TEXTURE_MANAGER_RETURN_RENDERED),
            vec![vec![0x5050, 0x5060]]
        );
        assert_eq!(calls(&e, RENDER_SLOTS + 0x10).len(), 1);
        let frustums = calls(&e, NI_CAMERA_SET_VIEW_FRUSTUM);
        assert_eq!(frustums.len(), 2);
        assert_eq!(
            calls(&e, BS_SHADER_UTIL_ACCUMULATE_SCENE),
            vec![vec![0x5040, 0x6100, 0x5030]]
        );
        assert_eq!(calls(&e, RENDER_00B6C0D0), vec![vec![0x5040, 0x6200, 0]]);
        // The interface node is hidden for the render and shown after it; the
        // menu root is shown only for it (this is not the Pip-Boy menu).
        let culled = calls(&e, SET_APP_CULLED_00450F90);
        assert_eq!(culled[0], vec![0x5010, 1]);
        assert!(culled.contains(&vec![0x6100, 0]));
        assert!(culled.contains(&vec![0x6100, 1]));
        assert_eq!(culled.last().unwrap(), &vec![0x5010, 0]);
        assert_eq!(
            calls(&e, CULLER_SET_ACCUMULATOR_004A0FD0),
            vec![vec![0x5030, 0x6200], vec![0x5030, 0]]
        );
    }

    #[test]
    fn render_sets_and_restores_the_orthographic_frustum() {
        let (mut e, this) = render_engine();
        e.set(this, FORenderedMenu::bInitialized, 1);
        e.set(this, FORenderedMenu::spMenuRoot, 0x6100);
        let mut seen = vec![];
        e.register_double(NI_CAMERA_SET_VIEW_FRUSTUM, move |e, a| {
            seen.push(
                (0..7)
                    .map(|i| e.mem.u32(a[1] + 4 * i))
                    .collect::<Vec<u32>>(),
            );
            if seen.len() == 2 {
                assert_eq!(seen[1], (0..7).map(|i| 0x900 + i).collect::<Vec<u32>>());
                let first = &seen[0];
                assert_eq!(first[0], 0);
                assert_eq!(first[1], 1280.0f32.to_bits());
                assert_eq!(first[2], 960.0f32.to_bits());
                assert_eq!(first[5], 10000.0f32.to_bits());
                assert_eq!(first[6], 0x906 & !0xff | 1);
            }
            Ret::default()
        });
        e.call(0x007f_ba00, &args![this, 1u32, 2u32, 0x7000u32]);
        assert!(calls(&e, TEXTURE_MANAGER_GET_RENDERED_00B6E110).is_empty());
        assert!(calls(&e, TEXTURE_MANAGER_RETURN_RENDERED).is_empty());
        assert_eq!(calls(&e, NI_CAMERA_SET_VIEW_FRUSTUM).len(), 2);
    }

    #[test]
    fn initialize_makes_the_accumulator_and_loads_the_textures() {
        for (block, texture) in [(0u32, 0u32), (0x8000, 0x8100)] {
            let mut e = Engine::new();
            pointers(&mut e);
            answer(&mut e, NI_ALLOCATE, block);
            answer(&mut e, BS_SHADER_ACCUMULATOR_CONSTRUCT, 0x8200);
            quiet_all(
                &mut e,
                &[ACCUMULATOR_004A1040, POINTER_DESTROY, DEBUG_MESSAGE],
            );
            e.register(POINTER_INIT, |e, a| {
                e.mem.set_u32(a[0], a[1]);
                Ret::default()
            });
            e.register_double(BS_SHADER_MANAGER_GET_TEXTURE, move |e, a| {
                if a[0] == SCREEN_TEXTURE_PATH {
                    e.mem.set_u32(a[2], texture);
                }
                Ret::default()
            });
            settings(
                &mut e,
                &[(BLUR_RADIUS_SETTING, 1.5), (BLUR_INTENSITY_SETTING, 0.75)],
            );
            answer(&mut e, SETTING_STRING_00403DF0, 0x8300);
            let this = menu(&mut e, &[]);
            e.call_log = Some(vec![]);
            e.call(0x007f_c150, &args![this]);
            let accumulator = if block == 0 { 0 } else { 0x8200 };
            assert_eq!(e.mem.u32(this.addr() + ACCUMULATOR), accumulator);
            assert_eq!(calls(&e, ACCUMULATOR_004A1040), vec![vec![accumulator, 10]]);
            assert_eq!(e.get(this, FORenderedMenu::spDefaultTex), texture);
            assert_eq!(calls(&e, DEBUG_MESSAGE).len(), usize::from(texture == 0));
            assert_eq!(e.get(this, FORenderedMenu::fBlurRadius), 1.5);
            assert_eq!(e.get(this, FORenderedMenu::fBlurIntensity), 0.75);
            let loads = calls(&e, BS_SHADER_MANAGER_GET_TEXTURE);
            assert_eq!(loads[0][..2], [SCREEN_TEXTURE_PATH, 1]);
            assert_eq!(loads[0][3..], [1, 0]);
            assert_eq!(
                loads[1],
                vec![0x8300, 1, this.addr() + SCANLINES_TEXTURE, 0, 0]
            );
            assert_eq!(
                calls(&e, SETTING_STRING_00403DF0),
                vec![vec![SCANLINES_SETTING]]
            );
            assert_eq!(e.get(this, FORenderedMenu::bInitialized), 1);
            if block != 0 {
                assert_eq!(
                    calls(&e, BS_SHADER_ACCUMULATOR_CONSTRUCT),
                    vec![vec![0x8000, 100, 1, 0x2f7]]
                );
            }
        }
    }

    #[test]
    fn set_default_texture_assigns_the_pointer() {
        let mut e = Engine::new();
        pointers(&mut e);
        let this = menu(&mut e, &[]);
        e.call(0x007f_c2b0, &args![this, 0x4321u32]);
        assert_eq!(e.get(this, FORenderedMenu::spDefaultTex), 0x4321);
    }

    #[test]
    fn prepare_geometry_attaches_a_missing_property_and_sets_colors() {
        for (missing, colored) in [(true, true), (false, false)] {
            let mut e = Engine::new();
            pointers(&mut e);
            answer(&mut e, PROPERTY_TYPE_00438220, 0x11);
            answer(&mut e, PROPERTY_TYPE_004AF350, 0x22);
            e.register_double(NI_AV_OBJECT_GET_PROPERTY, move |_, a| Ret {
                eax: match a[1] {
                    0x11 => 0x7000,
                    0 if missing => 0,
                    0 => 0x7100,
                    _ if colored => 0x7200,
                    _ => 0,
                },
                ..Ret::default()
            });
            answer(&mut e, NI_ALLOCATE, 0x7300);
            answer(&mut e, PROPERTY_INIT_004391C0, 0x7400);
            quiet_all(
                &mut e,
                &[
                    PROPERTY_SET_TEXTURE_00438230,
                    PROPERTY_0049ED90,
                    PROPERTY_00439340,
                    PROPERTY_00439390,
                    NI_AV_OBJECT_ATTACH_PROPERTY,
                    PROPERTY_SET_COLOR_004BC450,
                    NI_AV_OBJECT_UPDATE_PROPERTIES,
                    UPDATE_DATA_INIT_0043D410,
                    UPDATE_00A59C60,
                ],
            );
            let this = menu(&mut e, &[]);
            e.set(this, FORenderedMenu::spDefaultTex, 0x6100);
            e.call_log = Some(vec![]);
            e.call(0x007f_c2d0, &args![this, 0x6000u32]);
            assert_eq!(
                calls(&e, PROPERTY_SET_TEXTURE_00438230),
                vec![vec![0x7000, 0x6100]]
            );
            let attached = calls(&e, NI_AV_OBJECT_ATTACH_PROPERTY);
            if missing {
                assert_eq!(attached, vec![vec![0x6000, 0x7400]]);
                assert_eq!(calls(&e, PROPERTY_00439340), vec![vec![0x7400, 6]]);
                assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x1c]]);
            } else {
                assert!(attached.is_empty());
            }
            assert_eq!(
                calls(&e, PROPERTY_SET_COLOR_004BC450).len(),
                usize::from(colored)
            );
            assert_eq!(
                calls(&e, NI_AV_OBJECT_UPDATE_PROPERTIES),
                vec![vec![0x6000]]
            );
            assert_eq!(calls(&e, UPDATE_00A59C60)[0][0], 0x6000);
        }
    }

    const NODE_SLOTS: u32 = 0x0310_3000;

    fn light_menu_engine(source: u32, fallback: u32) -> (Engine, Ptr<FORenderedMenu>) {
        let mut e = Engine::new();
        pointers(&mut e);
        let node = object(&mut e, &[(0xc, NODE_SLOTS + 0xc)]);
        answer(&mut e, NODE_SLOTS + 0xc, source);
        let parent = object(&mut e, &[(0xdc, NODE_SLOTS + 0xdc)]);
        answer(&mut e, NODE_SLOTS + 0xdc, 0);
        let root = object(
            &mut e,
            &[(0x9c, NODE_SLOTS + 0x9c), (0xc, NODE_SLOTS + 0x10)],
        );
        answer(&mut e, NODE_SLOTS + 0x10, 0);
        e.register_double(NODE_SLOTS + 0x9c, move |e, a| Ret {
            eax: if e.mem.u32(a[1]) == RENDERED_MENU_LIGHT {
                fallback
            } else {
                node
            },
            ..Ret::default()
        });
        answer(&mut e, MENU_ROOT_005585E0, root);
        // The fixed string holds the C string's address.
        e.register(FIXED_STRING_INIT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        answer(&mut e, NI_ALLOCATE, 0x7300);
        answer(&mut e, POINT_LIGHT_INIT_00A7D670, 0x7400);
        let position = e.mem.alloc(12);
        e.mem.set_u32(position, 0xa1);
        answer(&mut e, POSITION_0043C490, position);
        answer(&mut e, GETTER_009611E0, parent);
        answer(&mut e, SHADOW_SCENE_NODE_00450B80, 0x7500);
        answer(&mut e, FIND_LIGHT_00653270, 0x7600);
        answer(&mut e, GETTER_00413F40, 0x7700);
        answer(&mut e, MODEL_PATH_0043B1B0, 0x7800);
        quiet_all(
            &mut e,
            &[
                FIXED_STRING_DESTROY,
                LIGHT_SET_POSITION_00440460,
                SHADOW_SCENE_NODE_ADD_LIGHT,
                SHADOW_SCENE_NODE_REMOVE_LIGHT,
                UPDATE_DATA_INIT_0043D410,
                UPDATE_00A59C60,
                SET_APP_CULLED_00450F90,
                SET_NAME_00A5B950,
                DEBUG_MESSAGE,
            ],
        );
        let this = menu(&mut e, &[]);
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn create_menu_light_makes_a_point_light_at_the_named_node() {
        let (mut e, this) = light_menu_engine(0x7900, 0);
        assert!(e.call(0x007f_c400, &args![this, 0x1234u32, 0u32]).bool());
        assert_eq!(e.get(this, FORenderedMenu::spLight), 0x7400);
        assert_eq!(e.get(this, FORenderedMenu::pLightShadowSceneNode), 0x7500);
        assert_eq!(
            calls(&e, SHADOW_SCENE_NODE_ADD_LIGHT),
            vec![vec![0x7500, 0x7400, 1]]
        );
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0xfc]]);
        let position = calls(&e, LIGHT_SET_POSITION_00440460)[0][1];
        assert_eq!(e.mem.u32(position), 0xa1);
        assert_eq!(calls(&e, NODE_SLOTS + 0xdc)[0][1..], [0x7400, 1]);
        assert_eq!(calls(&e, SET_APP_CULLED_00450F90), vec![vec![0x7400, 1]]);
        let named = calls(&e, SET_NAME_00A5B950);
        assert_eq!(e.mem.u32(named[0][1]), RENDERED_MENU_LIGHT);
        assert_eq!(calls(&e, FIXED_STRING_DESTROY).len(), 2);
    }

    #[test]
    fn create_menu_light_falls_back_to_the_rendered_menu_light_node() {
        let (mut e, this) = light_menu_engine(0, 0x7a00);
        assert!(e
            .call(0x007f_c400, &args![this, 0x1234u32, 0x7b00u32])
            .bool());
        assert_eq!(
            calls(&e, FIND_LIGHT_00653270),
            vec![vec![LIGHT_TYPE_011F4A98, 0x7a00]]
        );
        assert_eq!(e.get(this, FORenderedMenu::spLight), 0x7600);
        assert!(calls(&e, SHADOW_SCENE_NODE_00450B80).is_empty());
        // Neither node: a message, no light.
        let (mut e, this) = light_menu_engine(0, 0);
        assert!(!e.call(0x007f_c400, &args![this, 0x1234u32, 0u32]).bool());
        assert_eq!(
            calls(&e, DEBUG_MESSAGE),
            vec![vec![NO_LIGHT_MESSAGE, 0x1234, 0x7800]]
        );
        // No menu root: nothing.
        let (mut e, this) = light_menu_engine(0x7900, 0);
        answer(&mut e, MENU_ROOT_005585E0, 0);
        assert!(!e.call(0x007f_c400, &args![this, 0x1234u32, 0u32]).bool());
        assert!(calls(&e, FIXED_STRING_INIT).is_empty());
    }

    #[test]
    fn clear_menu_light_detaches_and_removes_the_light() {
        let (mut e, this) = light_menu_engine(0, 0);
        e.call(0x007f_c6f0, &args![this]);
        assert!(calls(&e, SHADOW_SCENE_NODE_REMOVE_LIGHT).is_empty());
        e.set(this, FORenderedMenu::spLight, 0x7400);
        e.set(this, FORenderedMenu::pLightShadowSceneNode, 0x7500);
        let parent = object(&mut e, &[(0xe8, NODE_SLOTS + 0xe8)]);
        answer(&mut e, NODE_SLOTS + 0xe8, 0);
        answer(&mut e, NODE_SLOTS + 0x10, parent);
        e.call(0x007f_c6f0, &args![this]);
        assert_eq!(calls(&e, NODE_SLOTS + 0xe8), vec![vec![parent, 0x7400]]);
        assert_eq!(
            calls(&e, SHADOW_SCENE_NODE_REMOVE_LIGHT),
            vec![vec![0x7500, 0x7400]]
        );
        assert_eq!(e.get(this, FORenderedMenu::spLight), 0);
    }

    #[test]
    fn hide_and_show_the_menus_around_the_render() {
        let mut e = Engine::new();
        e.register(TILE_GET_MENU_BY_CLASS, |_, a| Ret {
            eax: if a[0] == 0x3f8 { 0 } else { a[0] << 8 },
            ..Ret::default()
        });
        e.register(NODE_0056C7F0, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        // Menu 0x3e9 is shown, 0x423 already hidden.
        e.register(IS_APP_CULLED_00456610, |_, a| Ret {
            eax: u32::from(a[0] == (0x423 << 8) + 1),
            ..Ret::default()
        });
        answer(&mut e, SET_APP_CULLED_00450F90, 0);
        let this = menu(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x007f_c790, &args![this]);
        assert_eq!(e.get(this, FORenderedMenu::bHidMessage), 1);
        assert_eq!(e.get(this, FORenderedMenu::bHidQuantity), 0);
        assert_eq!(e.get(this, FORenderedMenu::bHidTutorial), 0);
        assert_eq!(
            calls(&e, SET_APP_CULLED_00450F90),
            vec![vec![(0x3e9 << 8) + 1, 1]]
        );
        e.call_log = Some(vec![]);
        e.call(0x007f_c8d0, &args![this]);
        assert_eq!(
            calls(&e, SET_APP_CULLED_00450F90),
            vec![vec![(0x3e9 << 8) + 1, 0]]
        );
    }
}

#[cfg(test)]
mod n051_tests {
    use super::*;

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

    fn answer(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| value.into_ret());
    }

    fn stub(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    /// An engine with the constants and statics mapped and the clock at 2000 ms.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_2000,
            0x0101_6000,
            0x0101_8000,
            0x0101_b000,
            0x011a_e000,
            0x0120_0000,
            0x011d_b000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(MINUS_ONE_FLOAT, -1.0f32);
        e.set_global(VERT_HOLD_SPEED_SCALE, 0.5f64);
        e.set_global(SHUDDER_DURATION_0101B2D0, 250.0f32);
        e.set_global(SHUDDER_VALUE_010181BC, 0.05f32);
        answer(&mut e, TIMER_GET_00825C00, 2000);
        e
    }

    fn menu(e: &mut Engine) -> Ptr<FORenderedMenu> {
        Ptr::new(e.mem.alloc(0x200))
    }

    fn float(e: &Engine, address: u32) -> f32 {
        e.mem.f32(address)
    }

    #[test]
    fn the_terminal_constructor_builds_on_the_menu() {
        let mut e = engine();
        stub(
            &mut e,
            &[RENDERED_MENU_CONSTRUCT, POINT_INIT_006815C0, POINTER_INIT],
        );
        let this = menu(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0070_5fe0, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), RENDERED_TERMINAL_VTABLE);
        assert_eq!(
            calls_to(&e, RENDERED_MENU_CONSTRUCT),
            vec![vec![this.addr()]]
        );
        assert_eq!(
            calls_to(&e, POINT_INIT_006815C0),
            vec![vec![this.addr() + 0xd8]]
        );
        assert_eq!(
            calls_to(&e, POINTER_INIT),
            vec![vec![this.addr() + 0xe4, 0]]
        );
    }

    #[test]
    fn menu_rendering_is_read_from_its_byte() {
        let mut e = engine();
        let this = menu(&mut e);
        e.mem.set_u8(this.addr() + 0xb9, 1);
        assert_eq!(e.call(0x007f_aef0, &args![this]).u8(), 1);
    }

    #[test]
    fn starting_a_vertical_hold_and_a_shudder() {
        let mut e = engine();
        let this = menu(&mut e);
        e.call(0x007f_b250, &args![this, 300.0f32, 4.0f32]);
        assert_eq!(e.mem.u8(this.addr() + 0x60), 1);
        assert_eq!(float(&e, this.addr() + 0x68), 300.0);
        assert_eq!(float(&e, this.addr() + 0x6c), 2.0);
        assert_eq!(float(&e, this.addr() + 0x64), 2000.0);
        e.call(0x007f_b2b0, &args![this, 1.0f32, 2.0f32, 3.0f32]);
        assert_eq!(e.mem.u8(this.addr() + 0x74), 1);
        assert_eq!(
            [0x7c, 0x80, 0x84, 0x78].map(|o| float(&e, this.addr() + o)),
            [1.0, 2.0, 3.0, 2000.0]
        );
    }

    #[test]
    fn the_distortion_ends_after_its_length() {
        let mut e = engine();
        let this = menu(&mut e);
        let length = e.mem.alloc(4);
        e.mem.set_f32(length, 1.0);
        let interval = e.mem.alloc(4);
        e.mem.set_i32(interval, 100);
        e.register_double(SETTING_VALUE, move |_, _| length.into_ret());
        answer(&mut e, SETTING_INT_VALUE, interval);
        answer(&mut e, RAND, 1234);
        e.set(this, FORenderedMenu::bDistortActive, 1);
        e.mem.set_f32(this.addr() + 0x98, 1000.0);
        e.mem.set_f32(this.addr() + 0xa0, 500.0);
        e.call(0x007f_b310, &args![this, 1500.0f32]);
        assert_eq!(float(&e, this.addr() + 0x9c), 1.0);
        assert_eq!(e.get(this, FORenderedMenu::bDistortActive), 1);
        e.call(0x007f_b310, &args![this, 2100.0f32]);
        assert_eq!(float(&e, this.addr() + 0x9c), 2.2);
        assert_eq!(e.get(this, FORenderedMenu::bDistortActive), 0);
        assert_eq!(float(&e, this.addr() + 0xa4), 2134.0);
    }

    #[test]
    fn the_shudder_fades_out() {
        let mut e = engine();
        let this = menu(&mut e);
        e.register(FLOAT_FUNCTION_004E44D0, |_, a| {
            st0(f64::from(f32::from_bits(a[0])))
        });
        e.set(this, FORenderedMenu::bShudderActive, 1);
        e.mem.set_f32(this.addr() + 0x78, 100.0);
        e.mem.set_f32(this.addr() + 0x7c, 200.0);
        e.mem.set_f32(this.addr() + 0x80, 2.0);
        e.mem.set_f32(this.addr() + 0x84, 0.5);
        e.call(0x007f_b3a0, &args![this, 150.0f32]);
        // sin-like(25) * 2 * (1 - 50/200).
        assert_eq!(float(&e, this.addr() + 0x88), 37.5);
        e.call(0x007f_b3a0, &args![this, 301.0f32]);
        assert_eq!(e.get(this, FORenderedMenu::bShudderActive), 0);
    }

    #[test]
    fn the_burst_fades_out() {
        let mut e = engine();
        let this = menu(&mut e);
        e.set(this, FORenderedMenu::bBurstActive, 1);
        e.mem.set_f32(this.addr() + 0x50, 100.0);
        e.mem.set_f32(this.addr() + 0x54, 200.0);
        e.mem.set_f32(this.addr() + 0x58, 4.0);
        e.call(0x007f_b420, &args![this, 250.0f32]);
        assert_eq!(float(&e, this.addr() + 0x5c), 1.0);
        e.call(0x007f_b420, &args![this, 300.0f32]);
        assert_eq!(e.get(this, FORenderedMenu::bBurstActive), 1);
        assert_eq!(float(&e, this.addr() + 0x5c), 0.0);
        e.call(0x007f_b420, &args![this, 300.5f32]);
        assert_eq!(e.get(this, FORenderedMenu::bBurstActive), 0);
    }

    #[test]
    fn the_vertical_hold_wraps_and_ends_in_a_shudder() {
        let mut e = engine();
        let this = menu(&mut e);
        e.register(WRAP_00404040, |_, a| {
            st0(f64::from(f32::from_bits(a[0])).floor())
        });
        e.set(this, FORenderedMenu::bVertHoldActive, 1);
        e.mem.set_f32(this.addr() + 0x64, 0.0);
        e.mem.set_f32(this.addr() + 0x68, 100.0);
        e.mem.set_f32(this.addr() + 0x6c, 0.025);
        e.call(0x007f_b480, &args![this, 20.0f32]);
        assert_eq!(float(&e, this.addr() + 0x70), 0.5);
        e.call(0x007f_b480, &args![this, 60.0f32]);
        assert_eq!(float(&e, this.addr() + 0x70), 0.5);
        e.call(0x007f_b480, &args![this, 100.5f32]);
        assert_eq!(e.get(this, FORenderedMenu::bVertHoldActive), 0);
        assert_eq!(e.get(this, FORenderedMenu::bShudderActive), 1);
        assert_eq!(float(&e, this.addr() + 0x7c), 250.0);
        assert_eq!(float(&e, this.addr() + 0x84), 0.05);
    }

    #[test]
    fn colors_add_component_wise() {
        let mut e = engine();
        let a = e.mem.alloc(12);
        let b = e.mem.alloc(12);
        let out = e.mem.alloc(12);
        for (i, (x, y)) in [(1.0f32, 0.5f32), (2.0, 0.25), (3.0, -1.0)]
            .iter()
            .enumerate()
        {
            e.mem.set_f32(a + 4 * i as u32, *x);
            e.mem.set_f32(b + 4 * i as u32, *y);
        }
        assert_eq!(e.call(0x007f_b730, &args![a, out, b]).u32(), out);
        assert_eq!([0, 4, 8].map(|o| float(&e, out + o)), [1.5, 2.25, 2.0]);
        assert_eq!(float(&e, a), 1.0);
    }

    #[test]
    fn the_partial_screen_flag_and_pointer_are_stored() {
        let mut e = engine();
        stub(&mut e, &[POINTER_SET]);
        let object = e.mem.alloc(0x200);
        e.call_log = Some(vec![]);
        e.call(0x007f_beb0, &args![object, 1u32, 0x7000u32]);
        assert_eq!(e.mem.u8(object + 0x1fc), 1);
        assert_eq!(
            calls_to(&e, POINTER_SET),
            vec![vec![object + 0x1f8, 0x7000]]
        );
    }

    #[test]
    fn do_effect_fills_the_shader_statics() {
        let mut e = engine();
        stub(
            &mut e,
            &[
                INTERFACE_GET_004B7210,
                POINTER_ASSIGN_006E5CC0,
                RENDERED_TEXTURE_GET_TEXTURE,
                POINTER_SET,
                POINTER_GET,
                INTERFACE_GET_PIPBOY,
                CREATE_PARTIAL_SCREEN_GEOMETRY,
                IMAGE_SPACE_RENDER_00B97550,
            ],
        );
        e.register(GET_MENU_HEIGHT, |_, _| st0(10.0));
        e.register(TILE_GET_FLOAT, |_, _| st0(20.0));
        answer(&mut e, RENDERER_0043C4B0, 0x6600);
        let manager = e.mem.alloc(0x200);
        answer(&mut e, GETTER_004E3270, manager);
        let this = menu(&mut e);
        let base = this.addr();
        e.mem.set_u32(base + 0x18, 0x5000);
        e.mem.set_f32(base + 0x40, 2.0);
        e.mem.set_f32(base + 0x44, 3.0);
        e.mem.set_f32(base + 0x48, 4.0);
        e.set(this, FORenderedMenu::bBurstActive, 1);
        e.mem.set_f32(base + 0x5c, 1.0);
        e.set(this, FORenderedMenu::bShudderActive, 1);
        e.mem.set_f32(base + 0x88, 0.5);
        e.mem.set_f32(base + 0x8c, 0.25);
        e.mem.set_f32(base + 0x90, 0.5);
        e.mem.set_u32(base + 0xc0, 0x11);
        e.mem.set_u32(base + 0xcc, 0x44);
        e.mem.set_u8(base + 0x2a, 1);
        e.call_log = Some(vec![]);
        e.call(0x007f_bee0, &args![this, 0x7100u32, 0x7200u32, 0x7300u32]);
        assert_eq!(float(&e, EFFECT_BLUR_RADIUS), 4.5);
        assert_eq!(float(&e, EFFECT_BLUR_INTENSITY), 6.25);
        assert_eq!(float(&e, EFFECT_SCANLINE_SCALE), 4.0);
        assert_eq!(float(&e, EFFECT_HEIGHT), 20.0);
        assert_eq!(float(&e, EFFECT_VERTICAL_OFFSET), 5.0);
        assert_eq!(float(&e, EFFECT_DISTORT_POSITION), -1.0);
        assert_eq!(e.global::<u32>(EFFECT_TINT), 0x11);
        assert_eq!(e.global::<u32>(EFFECT_TINT + 0xc), 0x44);
        assert_eq!(e.global::<u32>(EFFECT_THIRD_012007EC), 0x7300);
        let ten = 10.0f32.to_bits();
        assert_eq!(
            calls_to(&e, CREATE_PARTIAL_SCREEN_GEOMETRY),
            vec![vec![manager, ten, ten, base + 0xc]]
        );
        assert_eq!(
            calls_to(&e, IMAGE_SPACE_RENDER_00B97550),
            vec![vec![manager, 0xa, 0x6600, 0x7100, 0x7200, 0, 1]]
        );
        assert_eq!(e.mem.u8(manager + 0x1fc), 1);
        assert_eq!(
            calls_to(&e, POINTER_SET).last().unwrap(),
            &vec![EFFECT_TEXTURE_POINTER, 0]
        );
    }
}
