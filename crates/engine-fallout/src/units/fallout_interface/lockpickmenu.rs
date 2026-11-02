//! `fallout/interface/menus/lockpickmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `LockPickMenu` (Xbox PDB, 0x100 bytes) is the lock picking menu: the
//! `Menu` base (`pMenu` +4, `xFadeState` +0x24), `eLockState` (+0x28, 0 to
//! 7), sixteen tiles (`pTiles`, +0x2C), the target (`pLockTarget` +0x6C),
//! `eLockLevel` (+0x70), the skill (`iLockpickSkill` +0x74,
//! `fLockpickSkill` +0x78), `uiPrevIdleTick` (+0x80), the sweet spot
//! (`fSweetSpot` +0x84, `iTotalSweetSpotRadius` +0x88), the angles
//! (`fLockAngle` +0x8C, `fMaxLockAngle` +0x90), `fPickHealth` (+0x94), the
//! flags `bForceLockSuccess` (+0x98) and `bShowingTutorial` (+0x99), the
//! animations (`pIntroAnim` +0x9C, `pLockRotateAnim` +0xA0,
//! `ppickAnimManager` +0xA4, `pPickIntroAnim` +0xA8, `pPickBendAnim` +0xAC,
//! `pPickBreakAnim` +0xB0) and the pick's original rotation (+0xB4, a
//! matrix) and translation (+0xD8; the Xbox PDB has them 16-byte aligned at
//! +0xC0 and +0xF0). `pMe` (`011da204`) is the open menu; the lock model
//! and the 3D camera are the `NiPointer` statics at `011da24c` and
//! `011da224` (`spLockModel` and `sp3DCamera` in the Xbox PDB, matched by
//! use). The table at `011da28c` holds the skill each lock level needs
//! (one word per level, the next level's word right after).
//!
//! Notes that apply to every function:
//! - x87: each `FADD`/`FSUB`/`FMUL`/`FDIV` goes through the prelude's
//!   helpers in the code's order; `FLD`/`FSTP` copies of floats quiet a
//!   signalling NaN; floats pushed as arguments are their bit patterns; a
//!   callee's `ST0` result is its `f64`; `_ftol2_sse` (`00ec62c0`) gets
//!   `ST0` as a leading `f64` argument.
//! - Sounds are played as the code plays them: `BSAudio::QInstance`, then
//!   `GetSoundHandleByName(&handle, name, flags)` (three words), then
//!   `BSSoundHandle::Play` with the word the code pushed before the lookup,
//!   then the handle's destructor (a 0xC-byte local).
//! - The compiler's stack cookie checks (`00ec408c`) and exception frames
//!   (`FS:[0]`) are not translated; the functions that have them carry a
//!   `Translation incomplete` marker.

#[allow(unused_imports)]
use crate::prelude::*;

/// `LockPickMenu`'s class id (`GetClass`, `0078e080`).
const LOCK_PICK_MENU_CLASS: u32 = 0x3f6;
/// `LockPickMenu`'s vtable (RTTI `.?AVLockPickMenu@@`).
const LOCK_PICK_MENU_VTABLE: u32 = 0x0107_439c;
/// `LockPickMenu::pMe` (by its use): the open menu.
const LOCK_PICK_MENU_ME: u32 = 0x011d_a204;
/// `LockPickMenu::iUT_Value` and `iUT_X` (Xbox PDB statics; the names `_Value`, `_x`).
const USER_TRAIT_VALUE: u32 = 0x011d_a1f8;
const USER_TRAIT_X: u32 = 0x011d_a1fc;
const VALUE_TRAIT_NAME: u32 = 0x0106_ed18;
const X_TRAIT_NAME: u32 = 0x0106_ec90;
/// `spLockModel` and `sp3DCamera` (Xbox PDB statics, matched by use).
const LOCK_MODEL: u32 = 0x011d_a24c;
const CAMERA: u32 = 0x011d_a224;
/// The skill each lock level needs (word per level).
const SKILL_FOR_LEVEL: u32 = 0x011d_a28c;
/// The two sweet-spot widths `CreateSweetSpot` computes.
const SWEET_SPOT_WIDTH_A: u32 = 0x011d_a208;
const SWEET_SPOT_WIDTH_B: u32 = 0x011d_a20c;
/// `DoIdle`'s statics: a flag set when the pick broke (`011da2a4`), the state
/// of the last idle (`011da2a8`), the seconds spent in it (`011da2ac`),
/// the looping sound (`011da2b0`, a `BSSoundHandle`) with its guard word
/// (`011da2bc`, bit 0), and `RotateLockpick`'s last turn (`011da2c0`).
const PICK_BROKE: u32 = 0x011d_a2a4;
const LAST_STATE: u32 = 0x011d_a2a8;
const STATE_SECONDS: u32 = 0x011d_a2ac;
const LOOP_SOUND: u32 = 0x011d_a2b0;
const LOOP_SOUND_GUARD: u32 = 0x011d_a2bc;
const LAST_TURN: u32 = 0x011d_a2c0;
/// The sign of the pick's shake while it bends (`011a0394`, negated each idle).
const SHAKE_SIGN: u32 = 0x011a_0394;

/// The player (`PlayerCharacter *`) and the controls object (`011dea0c`).
const PLAYER: u32 = 0x011d_ea3c;
const CONTROLS: u32 = 0x011d_ea0c;
/// The lockpick item (`011ca268`).
const LOCKPICK_ITEM: u32 = 0x011c_a268;
/// The tick source (`011f6394`, its word at +0x14 through `00825c00`).
const TICK_SOURCE: u32 = 0x011f_6394;
/// The model loader (`011c3b3c`).
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// The identity matrix copied as the pick's rotation (`011a9448`, 9 words).
const IDENTITY_MATRIX: u32 = 0x011a_9448;
/// The controller type `GetController` looks for (`011f36ac`).
const CONTROLLER_MANAGER_TYPE: u32 = 0x011f_36ac;
/// The process lists (`011e0e80`).
const PROCESS_LISTS: u32 = 0x011e_0e80;

// Offsets in `LockPickMenu` (Xbox PDB names, PC offsets).
const LOCK_STATE: u32 = 0x28;
const TILES: u32 = 0x2c;
const LOCK_TARGET: u32 = 0x6c;
const LOCK_LEVEL: u32 = 0x70;
const LOCKPICK_SKILL: u32 = 0x74;
const LOCKPICK_SKILL_FLOAT: u32 = 0x78;
const PREV_IDLE_TICK: u32 = 0x80;
const SWEET_SPOT: u32 = 0x84;
const SWEET_SPOT_RADIUS: u32 = 0x88;
const LOCK_ANGLE: u32 = 0x8c;
const MAX_LOCK_ANGLE: u32 = 0x90;
const PICK_HEALTH: u32 = 0x94;
const FORCE_LOCK_SUCCESS: u32 = 0x98;
const SHOWING_TUTORIAL: u32 = 0x99;
const INTRO_ANIM: u32 = 0x9c;
const LOCK_ROTATE_ANIM: u32 = 0xa0;
const PICK_ANIM_MANAGER: u32 = 0xa4;
const PICK_INTRO_ANIM: u32 = 0xa8;
const PICK_BEND_ANIM: u32 = 0xac;
const PICK_BREAK_ANIM: u32 = 0xb0;
const PICK_ROTATE: u32 = 0xb4;
const PICK_TRANSLATE: u32 = 0xd8;

// Callees outside this unit.
const FREE: u32 = 0x0040_1030;
const SETTING_STRING: u32 = 0x0040_3df0;
const SETTING_FLOAT: u32 = 0x0040_3e20;
const SETTING_INT: u32 = 0x0043_d4d0;
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTROY: u32 = 0x0040_37d0;
const STRING_FORMAT: u32 = 0x0040_6f60;
const PRINT_FORMATTED: u32 = 0x0040_6d00;
const APPEND_STRING: u32 = 0x0040_6d50;
const COMPARE_STRINGS: u32 = 0x0040_8b20;
/// cdecl (float): the absolute value in ST0 (`00408820`).
const FLOAT_ABS: u32 = 0x0040_8820;
/// cdecl (a, b): `a` unless `b` is at least as large (or a NaN), in ST0 (`00404010`).
const FLOAT_MAX: u32 = 0x0040_4010;
/// cdecl (a, b): `a` unless `b` is at most as large (or a NaN), in ST0 (`0040ebd0`).
const FLOAT_MIN: u32 = 0x0040_ebd0;
/// `_ftol2_sse`: `ST0` truncated.
const FTOL: u32 = 0x00ec_62c0;
/// `_atexit`.
const ATEXIT: u32 = 0x00ec_658f;
/// The static sound handle's `atexit` destructor (`00fd8000`).
const LOOP_SOUND_EXIT: u32 = 0x00fd_8000;
/// `BSSoundHandle` constructor, assignment and destructor.
const SOUND_HANDLE_CONSTRUCT: u32 = 0x0041_a250;
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
const SOUND_HANDLE_DESTROY: u32 = 0x0048_3710;
/// `BSAudio::QInstance`, `GetSoundHandleByName`, `BSSoundHandle::Play` and
/// `FadeOutAndRelease` (engine map).
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
const SOUND_PLAY: u32 = 0x00ad_8830;
const SOUND_FADE_OUT: u32 = 0x00ad_8da0;
/// The node list helpers: the node itself and its object.
const NODE_ITEM: u32 = 0x0068_15c0;
const POINTER_GET: u32 = 0x0055_9450;
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const POINTER_DESTROY: u32 = 0x0045_cec0;
const TICKS: u32 = 0x0082_5c00;
/// cdecl (), the interface object (`011d8a80`).
const INTERFACE_OBJECT: u32 = 0x004b_7210;
/// cdecl (): whether a gamepad drives the menus (`004b71d0`).
const USING_GAMEPAD: u32 = 0x004b_71d0;
/// thiscall on the interface object: its root tile (`0045cd60`).
const INTERFACE_ROOT_TILE: u32 = 0x0045_cd60;
const TILE_READER: u32 = 0x0058_6150;
const TILE_READ_FILE: u32 = 0x00a0_1b00;
const TILE_GET_MENU: u32 = 0x00a0_3c90;
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
const TILE_GET_FLOAT: u32 = 0x00a0_11b0;
const TILE_POKE: u32 = 0x00a0_1290;
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;
const TILE_SET_STRING: u32 = 0x00a0_1350;
const TILE_SET_VALUE: u32 = 0x0070_0320;
const TILE_GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
const TILE_TEXT_TO_TRAIT: u32 = 0x00a0_1860;
const TILE_GET_CHILD_BY_NAME: u32 = 0x00a0_3da0;
const TILE_DELETE_CHILDREN: u32 = 0x00a0_4150;
const ADD_USER_TRAIT: u32 = 0x00a0_0940;
const MENU_CONSTRUCT: u32 = 0x00a1_c4a0;
const MENU_DESTROY: u32 = 0x00a1_c520;
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;
const MENU_RENDER_TEMPLATE: u32 = 0x00a1_ddb0;
/// thiscall on a tile: the menu (+0x3c) `RenderTemplate` runs on (`00639b40`).
const TILE_MENU_3C: u32 = 0x0063_9b40;
const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// thiscall on the tile list (`+4` of a tile), `(&iterator)`: the item and the next node (`0057cbe0`).
const TILE_LIST_NEXT: u32 = 0x0057_cbe0;
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// cdecl (text, 0, sound, 0, seconds, 0): shows a message (`007052f0`).
const SHOW_MESSAGE: u32 = 0x0070_52f0;
/// `Interface::IsTopMenuID` (engine map), cdecl (class): whether the menu is on top.
const IS_TOP_MENU: u32 = 0x0070_2450;
/// `ActorValueOwner::GetClampedActorValue` (engine map), thiscall (owner, value).
const GET_CLAMPED_ACTOR_VALUE: u32 = 0x0066_ef20;
const LOCKPICK_ACTOR_VALUE: u32 = 0x24;
/// `TESObjectREFR::GetLockLevel`, `REFR_LOCK::NumericValueToEnum` (engine map).
const GET_LOCK_LEVEL: u32 = 0x0056_91b0;
const LOCK_VALUE_TO_LEVEL: u32 = 0x0043_0b40;
/// cdecl (a, b) and cdecl (value, limit): the clamps of the level (`00647b70`, `004a8f20`).
const CLAMP_LOW_647B70: u32 = 0x0064_7b70;
const CLAMP_HIGH_4A8F20: u32 = 0x004a_8f20;
/// `GamePlayFormulas::GetForceLockChance` (engine map), cdecl (skill, required).
const GET_FORCE_LOCK_CHANCE: u32 = 0x0064_8d30;
/// `Actor::GetItemCountinContainer` (engine map), thiscall (actor, item).
const GET_ITEM_COUNT: u32 = 0x0057_5610;
/// cdecl (n): one of the scene roots (`00450b80`, the word at `011f91c8 + 4n`).
const SCENE_ROOT: u32 = 0x0045_0b80;
const NODE_SLOT_DETACH_CHILD: u32 = 0xe8;
const NODE_SLOT_ATTACH_CHILD: u32 = 0xdc;
/// thiscall on a node: removes its lights (`00572160`).
const NODE_CLEAN_UP_572160: u32 = 0x0057_2160;
/// thiscall on the scene root (`00b5d180`).
const SCENE_ROOT_B5D180: u32 = 0x00b5_d180;
const LOAD_FILE: u32 = 0x0044_7080;
const NODE_SET_TRANSLATE: u32 = 0x0044_0460;
const NODE_SET_ROTATE: u32 = 0x0043_fa80;
const NODE_SET_TRANSLATE_XYZ: u32 = 0x004b_c1f0;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
const NODE_CHILD: u32 = 0x0043_b4a0;
const POINT_CONSTRUCT: u32 = 0x0041_6870;
const MATRIX_MAKE_X_ROTATION: u32 = 0x0052_4ac0;
const MATRIX_MAKE_Y_ROTATION: u32 = 0x0043_f850;
/// thiscall (matrix, angle): another rotation (`004a0c90`).
const MATRIX_ROTATION_4A0C90: u32 = 0x004a_0c90;
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
const MATRIX_TIMES_POINT: u32 = 0x004b_4500;
/// thiscall (point, out): the point copied/negated into `out` (`004a0bd0`).
const POINT_OP_4A0BD0: u32 = 0x004a_0bd0;
/// thiscall (point, out, other): point difference and sum (`00439e90`, `00439ef0`).
const POINT_OP_439E90: u32 = 0x0043_9e90;
const POINT_OP_439EF0: u32 = 0x0043_9ef0;
const UPDATE_DATA_CONSTRUCT: u32 = 0x0043_d410;
const UPDATE_DOWNWARD: u32 = 0x00a5_9c60;
const GET_CONTROLLER: u32 = 0x00a5_c570;
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTROY: u32 = 0x0043_81b0;
const GET_SEQUENCE_BY_NAME: u32 = 0x0047_a520;
const ACTIVATE_SEQUENCE: u32 = 0x0047_aab0;
const DEACTIVATE_SEQUENCE: u32 = 0x0047_b220;
const DEACTIVATE_ALL: u32 = 0x0048_fef0;
/// thiscall (sequence, 2): sets the sequence's cycle type (`007408d0`).
const SEQUENCE_SET_CYCLE: u32 = 0x0074_08d0;
const START_ANIMATIONS: u32 = 0x00a6_d2d0;
const GET_WORLD_BOUND: u32 = 0x0043_d450;
const BOUND_RADIUS: u32 = 0x0084_d030;
const ADD_LIGHTS_FROM_NODE: u32 = 0x00b5_ca70;
const SEQUENCE_BEGIN_KEY: u32 = 0x0075_9450;
const SEQUENCE_END_KEY: u32 = 0x0050_8100;
const SEQUENCE_SET_PHASE: u32 = 0x00a3_28b0;
const POINT_OF_NODE: u32 = 0x0043_c490;
/// Camera construction and set-up.
const CAMERA_CONSTRUCT: u32 = 0x00a7_12f0;
const RECT_CONSTRUCT: u32 = 0x0041_4430;
const CAMERA_SET_VIEWPORT: u32 = 0x0071_2e60;
const FRUSTUM_CONSTRUCT: u32 = 0x00a7_1b70;
const CAMERA_SET_ASPECT: u32 = 0x0050_7700;
const SCREEN_WIDTH_OVERRIDE_SET: u32 = 0x011f_9426;
const SCREEN_WIDTH_OVERRIDE: u32 = 0x011f_9434;
const SCREEN_WIDTH: u32 = 0x004d_c200;
const SCREEN_HEIGHT: u32 = 0x004d_c1f0;
const TANGENT: u32 = 0x0057_dd70;
const CAMERA_SET_FRUSTUM: u32 = 0x00a6_faf0;
const CAMERA_LOOK_AT: u32 = 0x00a7_01b0;
const CAMERA_SET_LOD: u32 = 0x0073_3fd0;
/// cdecl (): the scene object (`0045c670`) and its camera far distance slot (+0x100).
const SCENE_OBJECT: u32 = 0x0045_c670;
const SCENE_SLOT_FAR: u32 = 0x100;
/// Render helpers of `0078e7c0`.
const RENDERER: u32 = 0x0043_c4b0;
const RENDERER_BEGIN_7148C0: u32 = 0x0071_48c0;
const CULLING_PROCESS: u32 = 0x008d_80e0;
const CULLING_SET_C4F270: u32 = 0x00c4_f270;
const CULLING_RESET_C4F2D0: u32 = 0x00c4_f2d0;
const CAMERA_SET_FOV_709D50: u32 = 0x0070_9d50;
const RENDERED_TEXTURE_7FD640: u32 = 0x007f_d640;
const CULLING_SET_TARGET: u32 = 0x004a_0fd0;
const SCENE_CAMERA_6629F0: u32 = 0x0066_29f0;
const ACCUMULATE_SCENE: u32 = 0x00b6_bee0;
const RENDER_SCENE_B6C0D0: u32 = 0x00b6_c0d0;
/// The controls helpers.
const CONTROLS_DEVICE: u32 = 0x0087_7720;
const CONTROL_AXIS: u32 = 0x00a2_3390;
const CONTROL_MOUSE_AXIS: u32 = 0x00a2_39e0;
const CONTROL_PRESSED: u32 = 0x00a2_4660;
const CONTROL_RUMBLE: u32 = 0x00a2_55b0;
const CONTROL_STOP_RUMBLE: u32 = 0x00a2_5810;
const INTERFACE_SCREEN_WIDTH: u32 = 0x0071_5d40;
const INTERFACE_MOUSE_SCALE: u32 = 0x0070_6e50;
/// cdecl (a, b): the turn speed formula (`006494b0`).
const TURN_FORMULA: u32 = 0x0064_94b0;
/// The random number generator and `BSRandom::UnsignedInt`.
const RANDOM_GENERATOR: u32 = 0x0047_6c00;
const RANDOM_UNSIGNED: u32 = 0x00aa_5230;
const RANDOM_FLOAT: u32 = 0x004d_ff60;
/// The tutorial helpers (`00790290`'s message id, `InterfaceManager::TutorialManager`).
const INTERFACE_MANAGER_TUTORIALS: u32 = 0x4d4;
const TUTORIAL_SEEN: u32 = 0x0071_8840;
const TUTORIAL_SHOW: u32 = 0x0071_8630;
const TUTORIAL_FLAG_5D4A40: u32 = 0x005d_4a40;
const INTERFACE_UPDATE_718930: u32 = 0x0071_8930;
const GAMEPAD_HINT_709D40: u32 = 0x0070_9d40;
/// Unlock helpers.
const MISC_STAT_INCREMENT: u32 = 0x004d_5c60;
const GET_LOCK: u32 = 0x0056_9160;
const GET_EXPERIENCE: u32 = 0x0067_05b0;
const PLAYER_SLOT_REWARD_XP: u32 = 0x488;
const ADD_LOCK_CHANGE: u32 = 0x0057_89b0;
const GET_OWNER: u32 = 0x0056_7790;
const FORM_TYPE: u32 = 0x0040_1170;
const ACTOR_REF_FOR_BASE: u32 = 0x0097_0b30;
const ACTOR_REF_IN_HIGH: u32 = 0x0097_0a20;
const SET_WANT_WEAPON_DRAWN: u32 = 0x0091_5d20;
const STEAL_ALARM: u32 = 0x008b_fa40;
const BASE_IS_EVIL_47D7C0: u32 = 0x0047_d7c0;
const ALIGNMENT_FOR_KARMA: u32 = 0x0047_e040;
const REWARD_KARMA: u32 = 0x0094_fd30;
const UNLOCK: u32 = 0x0056_92a0;
const SAVE_FORM_LINK: u32 = 0x007a_f430;
const CONTAINER_MENU_75B6E0: u32 = 0x0075_b6e0;
const ACTIVATE: u32 = 0x0057_3170;
const PLAYER_SLOT_REMOVE_ITEM: u32 = 0x17c;
const D3D_FADE_STATE: u32 = 0x0059_bb30;
const POINT_PERCENT_4BD510: u32 = 0x004b_d510;

// Data: names, formats and numbers read from the exe.
const MENU_FILE: u32 = 0x0107_44a0;
const CREATION_FAILED_MESSAGE: u32 = 0x0107_4448;
const ROOT_VALUE_A: u32 = 0x0107_04f0;
const ROOT_VALUE_B: u32 = 0x0107_04e8;
const SKILL_TOO_LOW_FORMAT: u32 = 0x011d_1efc;
const SKILL_TOO_LOW_SOUND: u32 = 0x0102_b0d0;
const MESSAGE_SECONDS: u32 = 0x0101_62c0;
const FAIL_SOUND_NAME: u32 = 0x0102_5cc4;
const TRAIT_NAME_74438: u32 = 0x0107_4438;
const INTEGER_FORMAT: u32 = 0x0102_0764;
const CHANCE_FORMAT: u32 = 0x0107_4430;
const CHANCE_SETTING: u32 = 0x011d_345c;
const PERCENT_TEXT: u32 = 0x0107_442c;
const LEVEL_NAMES: u32 = 0x0118_4a98;
const TEXT_74418: u32 = 0x0107_4418;
const LANGUAGE_SETTING: u32 = 0x011c_3cb4;
const LANGUAGE_TEXT: u32 = 0x0101_8348;
const CAPTIONS: [(u32, u32); 3] = [
    (0x011d_4d2c, 0x0107_4408),
    (0x011d_4360, 0x0107_43f4),
    (0x011d_25bc, 0x0107_43e4),
];
const CAPTION_FOR_TILE_54: u32 = 0x011d_33fc;
const SKILL_SCALE_SETTING: u32 = 0x011d_0de0;
const LOCK_MODEL_FILE: u32 = 0x0107_44e4;
const PICK_MODEL_FILE: u32 = 0x0107_44c4;
const PICK_DEPTH: u32 = 0x0107_44c0;
const LOCK_TILT: u32 = 0x0101_ff38;
const LOCK_MODEL_X: u32 = 0x0103_128c;
const INTRO_NAME: u32 = 0x0104_8458;
const ROTATE_NAME: u32 = 0x0104_844c;
const BREAK_NAME: u32 = 0x0104_8444;
const LIGHT_RADIUS_FACTOR: u32 = 0x0102_1928;
const RENDER_FOV: u32 = 0x0103_0ff4;
const NEAR_SETTING: u32 = 0x0120_3144;
const FOV_SETTING: u32 = 0x0120_3150;
const FRUSTUM_SCALE: u32 = 0x0101_6264;
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
const HALF_70C80: u32 = 0x0107_0c80;
const PICK_START_Z: u32 = 0x0101_8180;
const ANGLE_SCALE_6B160: u32 = 0x0106_b160;
const ANGLE_OFFSET_66808: u32 = 0x0106_6808;
const CONTROLLER_SCALE: u32 = 0x0102_978c;
const DEAD_ZONE_718F0: u32 = 0x0107_18f0;
const TURN_FORMULA_ARG: u32 = 0x0101_7718;
const TURN_SCALE: u32 = 0x0107_2310;
const ZERO: u32 = 0x0101_2060;
const TURN_SOUND: u32 = 0x0107_462c;
const SPOT_SCALE: u32 = 0x0101_1590;
const FAA_OFFSET: u32 = 0x0103_56f8;
const FAA_SCALE: u32 = 0x0103_4208;
const OUTSIDE_SCALE: u32 = 0x0101_5a38;
const TICKS_PER_SECOND: u32 = 0x0101_7b70;
const DEAD_ZONE_745F8: u32 = 0x0107_45f8;
const STICK_SCALE: u32 = 0x0102_9790;
const ENTER_ALPHA: u32 = 0x0102_3cd8;
const BLOCKED_SOUND: u32 = 0x0107_45e0;
const NO_PICKS_TEXT: u32 = 0x011d_34ec;
const NO_PICKS_SOUND: u32 = 0x0104_9638;
const START_SOUND: u32 = 0x0107_45c4;
const CLICK_ANGLE_A: u32 = 0x0101_db88;
const CLICK_ANGLE_B: u32 = 0x0101_2638;
const CLICK_SOUND_A: u32 = 0x0107_45a4;
const CLICK_SOUND_B: u32 = 0x0107_4584;
const RUMBLE_STRENGTH: u32 = 0x0101_e2bc;
const UNLOCK_SOUND: u32 = 0x0102_b13c;
const KARMA_SETTING: u32 = 0x011c_de20;
const UNLOCKED_TEXT: u32 = 0x0107_4570;
const BEND_SOUND: u32 = 0x0107_4554;
const STRAIN_SOUND: u32 = 0x0107_4538;
const HEALTH_SETTING: u32 = 0x011d_41c8;
const SHAKE_ANGLE: u32 = 0x0102_90b0;
const BEND_RUMBLE: u32 = 0x0101_6248;
const SHAKE_TURN: u32 = 0x0101_8c00;
const HEALTH_PERCENT: u32 = 0x0101_7a40;
const HEALTH_FORMAT: u32 = 0x0107_4524;
const HEALTH_SUFFIX: u32 = 0x0101_2320;
const BREAK_SOUND: u32 = 0x0107_450c;
const ONE: u32 = 0x0101_2070;
const SWEET_SPOT_TEMPLATE: u32 = 0x0107_4648;
const SPOT_COLOR_SCALE: u32 = 0x0101_e568;
const SPOT_STEPS: u32 = 0x0102_0998;
/// The four per-level setting tables the sweet spot widths come from.
const SPOT_TABLE_A_LOW: u32 = 0x011a_0328;
const SPOT_TABLE_A_HIGH: u32 = 0x011a_033c;
const SPOT_TABLE_B_LOW: u32 = 0x011a_0350;
const SPOT_TABLE_B_HIGH: u32 = 0x011a_0364;
/// The colour helpers of `CreateSweetSpot` (four floats each).
const COLOR_SUBTRACT: u32 = 0x0053_2ec0;
const COLOR_SCALE: u32 = 0x0053_2f60;
const COLOR_ADD: u32 = 0x0053_2e40;

fn tile_at(e: &Engine, base: u32, index: u32) -> u32 {
    e.mem.u32(base + TILES + 4 * index)
}

fn lock_model(e: &mut Engine) -> u32 {
    e.call(POINTER_GET, &args![LOCK_MODEL]).u32()
}

fn lock_model_child(e: &mut Engine, index: u32) -> u32 {
    let model = lock_model(e);
    e.call(NODE_CHILD, &args![model, index]).u32()
}

fn quiet_global(e: &Engine, address: u32) -> u32 {
    x87_quiet_bits(e.global::<u32>(address))
}

fn quiet_field(e: &Engine, address: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(address))
}

/// Plays a sound by name the way the code does (see the module comment).
fn play_sound(e: &mut Engine, name: u32, flags: u32) {
    e.with_stack(0xc, |e, handle| {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let found = e
            .call(GET_SOUND_HANDLE_BY_NAME, &args![audio, handle, name, flags])
            .u32();
        e.call(SOUND_PLAY, &args![found, 0u32]);
        e.call(SOUND_HANDLE_DESTROY, &args![handle]);
    });
}

fn update_data(e: &mut Engine, local: u32, value: u32, first: u32, second: u32) {
    e.call(UPDATE_DATA_CONSTRUCT, &args![local, value, first, second]);
}

fn ftol(e: &mut Engine, value: f64) -> i32 {
    e.call(FTOL, &args![value]).i32()
}

/// `ActivateSequence(sequence, 0, 1, 1.0, 0.0, 0)` on a controller manager.
fn activate_sequence(e: &mut Engine, manager: u32, sequence: u32) {
    e.call(
        ACTIVATE_SEQUENCE,
        &args![
            manager,
            sequence,
            0u32,
            1u32,
            1.0f32.to_bits(),
            0.0f32.to_bits(),
            0u32
        ],
    );
}

fn deactivate_sequence(e: &mut Engine, manager: u32, sequence: u32) {
    e.call(
        DEACTIVATE_SEQUENCE,
        &args![manager, sequence, 0.0f32.to_bits()],
    );
}

fn tile_value(e: &mut Engine, tile: u32, trait_id: u32) -> f64 {
    e.call(TILE_GET_FLOAT, &args![tile, trait_id]).f64()
}

fn the_player(e: &Engine) -> u32 {
    e.global(PLAYER)
}

fn controls_device(e: &mut Engine) -> u32 {
    let controls: u32 = e.global(CONTROLS);
    e.call(CONTROLS_DEVICE, &args![controls]).u32()
}

// Translated from 0078d930 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0078d930 - the C++ exception-unwinding frame (FS:[0]) that destroys the base when a later call throws is not translated
/// `LockPickMenu::LockPickMenu` (Xbox PDB), thiscall without stack words:
/// constructs the `Menu` base, sets the vtable, runs the two members'
/// constructors (`006815c0` on +0xB4 and +0xD8), registers the user traits
/// `_Value` and `_x`, takes the player's lockpick skill (actor value 0x24)
/// as an integer and a float, clears the lock angle, sets the pick health
/// to `skill * <setting 011d0de0> + 1`, and clears the state, the flags, the
/// animations and the idle tick. Answers `this`.
pub fn lock_pick_menu_lock_pick_menu(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(MENU_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, LOCK_PICK_MENU_VTABLE);
    e.call(NODE_ITEM, &args![base + PICK_ROTATE]);
    e.call(NODE_ITEM, &args![base + PICK_TRANSLATE]);
    let value = e
        .call(ADD_USER_TRAIT, &args![VALUE_TRAIT_NAME, 0xffff_ffffu32])
        .u32();
    e.set_global(USER_TRAIT_VALUE, value);
    let x = e
        .call(ADD_USER_TRAIT, &args![X_TRAIT_NAME, 0xffff_ffffu32])
        .u32();
    e.set_global(USER_TRAIT_X, x);
    let owner = the_player(e) + 0xa4;
    let skill = e
        .call(GET_CLAMPED_ACTOR_VALUE, &args![owner, LOCKPICK_ACTOR_VALUE])
        .u32();
    e.mem.set_u32(base + LOCKPICK_SKILL, skill);
    let skill = e.mem.i32(base + LOCKPICK_SKILL);
    e.mem
        .set_u32(base + LOCKPICK_SKILL_FLOAT, x87_store32(f64::from(skill)));
    e.mem.set_u32(base + LOCK_ANGLE, 0.0f32.to_bits());
    let scale_at = e.call(SETTING_FLOAT, &args![SKILL_SCALE_SETTING]).u32();
    let health = x87_add(
        x87_mul(
            x87_load32(e.mem.u32(base + LOCKPICK_SKILL_FLOAT)),
            x87_m32(e.mem.u32(scale_at)),
        ),
        1.0,
    );
    e.mem.set_u32(base + PICK_HEALTH, x87_store32(health));
    e.mem.set_u32(base + LOCK_STATE, 0);
    e.mem.set_u8(base + FORCE_LOCK_SUCCESS, 0);
    for offset in [
        INTRO_ANIM,
        LOCK_ROTATE_ANIM,
        PICK_INTRO_ANIM,
        PICK_BEND_ANIM,
        PICK_BREAK_ANIM,
    ] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u8(base + SHOWING_TUTORIAL, 0);
    e.mem.set_u32(base + PREV_IDLE_TICK, 0);
    this
}

// Translated from 0078da90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockPickMenu::GetClass` (Xbox PDB): the menu class id, `0x3f6`.
pub fn lock_pick_menu_get_class(_e: &mut Engine, _this: Ptr) -> u32 {
    LOCK_PICK_MENU_CLASS
}

// Translated from 0078daa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockPickMenu::AttachTileByID` (Xbox PDB): stores the tile in `pTiles`
/// (+0x2C) for ids 0 to 15 (signed compare); other ids do nothing.
pub fn lock_pick_menu_attach_tile_by_id(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    if (0..0x10).contains(&id) {
        e.mem.set_u32(this.addr() + TILES + 4 * id as u32, tile);
    }
}

// Translated from 0078dad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockPickMenu::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor (`0078e0f0`) and, when bit 0 of the flags is set, frees the
/// object. Answers `this`.
pub fn lock_pick_menu_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0078e0f0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// The two traits of the root tile the creation resizes on (as `TraitMenu`).
fn resize_root(e: &mut Engine, root: u32) {
    let first = tile_value(e, root, 0xfa7);
    let resize = first == e.global::<f64>(ROOT_VALUE_A) || {
        let second = tile_value(e, root, 0xfa7);
        second == e.global::<f64>(ROOT_VALUE_B)
    };
    if resize {
        let depth = e.call(TILE_GET_MAXIMUM_DEPTH, &args![]).f64();
        e.call(
            TILE_SET_FLOAT,
            &args![root, 0xfadu32, x87_store32(depth), 1u32],
        );
    }
}

// Translated from 0078db00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0078db00 - the C++ exception-unwinding frame (FS:[0]) that releases the sound handle when a call throws, and the /GS stack cookie check (00ec408c), are not translated
/// `LockPickMenu::Create` (Xbox PDB), `cdecl (target, level, flag)`: the
/// lock level is the target's (through `REFR_LOCK::NumericValueToEnum`)
/// or `level` without a target, clamped by `00647b70(.., 0)` and
/// `004a8f20(.., 4)`. With a target and too little lockpick skill for the
/// level, shows the "skill too low" message (formatted into a 0x104-byte
/// local), plays the failure sound and answers 0. Otherwise deletes an open
/// lock pick menu, reads `Data\Menus\...` (`010744a0`) and makes the menu
/// `pMe`; when its class is the lock pick menu's, takes the tile, checks
/// all sixteen tiles (else prints the failure and answers 0), resizes the
/// root like the other menus, keeps the target and level, sets the captions,
/// the skill, the force-lock chance, the level name and the lockpick count,
/// the language-dependent captions, prepares the 3D elements and the sweet
/// spot and prepares the menu for visibility; answers the root tile.
/// Otherwise deletes a menu that has a tile and answers 0.
pub fn lock_pick_menu_create(e: &mut Engine, target: u32, level: u32, flag: u32) -> u32 {
    let existing = e
        .call(TILE_GET_MENU_BY_CLASS, &args![LOCK_PICK_MENU_CLASS])
        .u32();
    let interface = e.call(INTERFACE_OBJECT, &args![]).u32();
    let level = if target != 0 {
        let value = e.call(GET_LOCK_LEVEL, &args![target]).u32();
        e.call(LOCK_VALUE_TO_LEVEL, &args![value]).u32()
    } else {
        level
    };
    let clamped = e.call(CLAMP_LOW_647B70, &args![level, 0u32]).u32();
    let level = e.call(CLAMP_HIGH_4A8F20, &args![clamped, 4u32]).u32();
    if target != 0 {
        let owner = the_player(e) + 0xa4;
        let skill = e
            .call(GET_CLAMPED_ACTOR_VALUE, &args![owner, LOCKPICK_ACTOR_VALUE])
            .i32();
        if skill < e.mem.i32(SKILL_FOR_LEVEL + 4 * level) {
            let required = e.mem.u32(SKILL_FOR_LEVEL + 4 * level);
            e.with_stack(0x104, |e, text| {
                let format = e.call(SETTING_STRING, &args![SKILL_TOO_LOW_FORMAT]).u32();
                e.call(PRINT_FORMATTED, &args![text, 0x104u32, format, required]);
                let seconds = quiet_global(e, MESSAGE_SECONDS);
                e.call(
                    SHOW_MESSAGE,
                    &args![text, 0u32, SKILL_TOO_LOW_SOUND, 0u32, seconds, 0u32],
                );
            });
            play_sound(e, FAIL_SOUND_NAME, 0x121);
            return 0;
        }
    }
    if existing != 0 {
        e.vcall(existing, 0, &args![1u32]);
    }
    let reader = e.call(TILE_READER, &args![interface]).u32();
    let root = e.call(TILE_READ_FILE, &args![reader, MENU_FILE]).u32();
    let menu = e.call(TILE_GET_MENU, &args![root]).u32();
    e.set_global(LOCK_PICK_MENU_ME, menu);
    if menu != 0 {
        let class = e.vcall(menu, 0x34, &args![]).u32();
        if class == fn_0078e080(e) {
            return create_contents(e, root, target, level, flag);
        }
    }
    let menu: u32 = e.global(LOCK_PICK_MENU_ME);
    if menu != 0 && e.call(LIST_NODE_NEXT, &args![menu]).u32() != 0 {
        let menu: u32 = e.global(LOCK_PICK_MENU_ME);
        if menu != 0 {
            e.vcall(menu, 0, &args![1u32]);
        }
    }
    0
}

/// The part of [`lock_pick_menu_create`] once the menu is the lock pick menu.
fn create_contents(e: &mut Engine, root: u32, target: u32, level: u32, flag: u32) -> u32 {
    let me = |e: &Engine| -> u32 { e.global(LOCK_PICK_MENU_ME) };
    let menu = me(e);
    e.call(MENU_SET_MENU_TILE, &args![menu, root, 0u32]);
    let menu = me(e);
    if !fn_0078e090(e, Ptr::new(menu)) {
        e.call(PRINT_MESSAGE, &args![CREATION_FAILED_MESSAGE]);
        return 0;
    }
    resize_root(e, root);
    let menu = me(e);
    e.mem.set_u32(menu + LOCK_TARGET, target);
    e.mem.set_u32(menu + LOCK_LEVEL, level);
    let trait_id = e.call(TILE_TEXT_TO_TRAIT, &args![TRAIT_NAME_74438]).u32();
    let menu = me(e);
    let tile = e.call(LIST_NODE_NEXT, &args![menu]).u32();
    e.call(TILE_SET_VALUE, &args![tile, trait_id, flag & 0xff]);
    e.with_stack(0x40, |e, text| {
        let menu = me(e);
        let skill = e.mem.u32(menu + LOCKPICK_SKILL);
        e.call(
            PRINT_FORMATTED,
            &args![text, 0x40u32, INTEGER_FORMAT, skill],
        );
        let menu = me(e);
        let value_trait: u32 = e.global(USER_TRAIT_VALUE);
        let tile = tile_at(e, menu, 0);
        e.call(TILE_SET_STRING, &args![tile, value_trait, text, 1u32]);
        let menu = me(e);
        let required = e
            .mem
            .u32(SKILL_FOR_LEVEL + 4 * e.mem.u32(menu + LOCK_LEVEL));
        let owner = the_player(e) + 0xa4;
        let skill = e
            .call(GET_CLAMPED_ACTOR_VALUE, &args![owner, LOCKPICK_ACTOR_VALUE])
            .u32();
        let chance = e.call(GET_FORCE_LOCK_CHANCE, &args![skill, required]).u32();
        let caption = e.call(SETTING_STRING, &args![CHANCE_SETTING]).u32();
        e.call(
            PRINT_FORMATTED,
            &args![text, 0x40u32, CHANCE_FORMAT, caption, chance],
        );
        e.call(APPEND_STRING, &args![text, 0x40u32, PERCENT_TEXT]);
        let menu = me(e);
        let tile = tile_at(e, menu, 9);
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
    });
    let menu = me(e);
    let level_setting = e.mem.u32(LEVEL_NAMES + 4 * e.mem.u32(menu + LOCK_LEVEL));
    let level_name = e.call(SETTING_STRING, &args![level_setting]).u32();
    let value_trait: u32 = e.global(USER_TRAIT_VALUE);
    let menu = me(e);
    let tile = tile_at(e, menu, 2);
    e.call(TILE_SET_STRING, &args![tile, value_trait, level_name, 1u32]);
    let menu = me(e);
    let tile = tile_at(e, menu, 12);
    e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, TEXT_74418, 1u32]);
    let value_trait: u32 = e.global(USER_TRAIT_VALUE);
    let menu = me(e);
    let tile = tile_at(e, menu, 1);
    e.call(TILE_POKE, &args![tile, value_trait, 1.0f32.to_bits()]);
    let count = if target != 0 {
        let item: u32 = e.global(LOCKPICK_ITEM);
        let player = the_player(e);
        e.call(GET_ITEM_COUNT, &args![player, item]).u32()
    } else {
        0
    };
    let value_trait: u32 = e.global(USER_TRAIT_VALUE);
    let menu = me(e);
    let tile = tile_at(e, menu, 1);
    e.call(TILE_SET_VALUE, &args![tile, value_trait, count]);
    let language = e.call(SETTING_STRING, &args![LANGUAGE_SETTING]).u32();
    if e.call(COMPARE_STRINGS, &args![language, LANGUAGE_TEXT])
        .u32()
        != 0
    {
        for (setting, name) in CAPTIONS {
            let text = e.call(SETTING_STRING, &args![setting]).u32();
            let child = e.call(TILE_GET_CHILD_BY_NAME, &args![root, name]).u32();
            e.call(TILE_SET_STRING, &args![child, 0xfc4u32, text, 1u32]);
        }
        let text = e.call(SETTING_STRING, &args![CAPTION_FOR_TILE_54]).u32();
        let menu = me(e);
        let tile = tile_at(e, menu, 10);
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
    }
    let menu = me(e);
    lock_pick_menu_prepare_3d_elements(e, Ptr::new(menu));
    let menu = me(e);
    lock_pick_menu_create_sweet_spot(e, Ptr::new(menu));
    let menu = me(e);
    e.call(MENU_PREP_FOR_VISIBILITY, &args![menu, 0u32]);
    root
}

// Translated from 0078e080 (decompiled, FalloutNV.exe 1.4.0.525)
/// The lock pick menu's class id, `0x3f6`, as a function without `this`
/// (the static counterpart of `GetClass`, unnamed in the engine map).
pub fn fn_0078e080(_e: &mut Engine) -> u32 {
    LOCK_PICK_MENU_CLASS
}

// Translated from 0078e090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether all sixteen tiles of `pTiles` (+0x2C) are attached (unnamed in
/// the engine map); stops at the first missing one.
pub fn fn_0078e090(e: &mut Engine, this: Ptr) -> bool {
    (0..0x10).all(|index| tile_at(e, this.addr(), index) != 0)
}

// Translated from 0078e0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, static: sets the open menu's `eLockState` to
/// 7 (closing) when there is one.
pub fn fn_0078e0d0(e: &mut Engine) {
    let menu: u32 = e.global(LOCK_PICK_MENU_ME);
    if menu != 0 {
        let menu: u32 = e.global(LOCK_PICK_MENU_ME);
        e.mem.set_u32(menu + LOCK_STATE, 7);
    }
}

// Translated from 0078e0f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0078e0f0 - the C++ exception-unwinding frame (FS:[0]) that destroys the base when a call throws is not translated
/// `LockPickMenu::~LockPickMenu` (unnamed in the engine map; the body is
/// the destructor): sets the vtable, detaches the lock model from scene
/// root 1, cleans it up (`00572160`), releases the lock model and the
/// camera, clears `pMe`, runs `00b5d180` on scene root 1 and destroys the
/// `Menu` base.
pub fn fn_0078e0f0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, LOCK_PICK_MENU_VTABLE);
    let root = e.call(SCENE_ROOT, &args![1u32]).u32();
    let model = lock_model(e);
    e.vcall(root, NODE_SLOT_DETACH_CHILD, &args![model]);
    let model = lock_model(e);
    e.call(NODE_CLEAN_UP_572160, &args![model]);
    e.call(POINTER_ASSIGN, &args![LOCK_MODEL, 0u32]);
    e.call(POINTER_ASSIGN, &args![CAMERA, 0u32]);
    e.set_global(LOCK_PICK_MENU_ME, 0u32);
    let root = e.call(SCENE_ROOT, &args![1u32]).u32();
    e.call(SCENE_ROOT_B5D180, &args![root]);
    e.call(MENU_DESTROY, &args![base]);
}

/// Takes three named sequences of a controller manager (`GetSequenceByName`
/// through a temporary `NiFixedString`, a 4-byte local each).
fn sequence_by_name(e: &mut Engine, manager: u32, name: u32) -> u32 {
    e.with_stack(4, |e, text| {
        let fixed = e.call(FIXED_STRING_CONSTRUCT, &args![text, name]).u32();
        let sequence = e.call(GET_SEQUENCE_BY_NAME, &args![manager, fixed]).u32();
        e.call(FIXED_STRING_DESTROY, &args![text]);
        sequence
    })
}

// Translated from 0078e1c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0078e1c0 - the C++ exception-unwinding frame (FS:[0]) that frees the node allocation and the fixed strings when a call throws is not translated
/// `LockPickMenu::Prepare3DElements` (Xbox PDB): loads the lock and pick
/// models, keeps the pick's original rotation (the identity at `011a9448`)
/// and translation `(0, 0, <010744c0>)` and applies them, makes a new
/// `NiNode` (0xAC bytes) the lock model with both models attached, turns it
/// (an X rotation by the float at `0101ff38` times a second rotation by the
/// same angle), places it at `(<0103128c>, 0, 0)`, updates it, takes the
/// lock's intro and rotate sequences and the pick's intro, bend and break
/// sequences (activating the intros, the rotate and break ones cycling),
/// starts both models' animations, makes the 3D camera, attaches the lock
/// model to scene root 1 and adds the lights of its bound (radius times
/// the `double` at `01021928`).
pub fn lock_pick_menu_prepare_3d_elements(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let loader: u32 = e.global(MODEL_LOADER);
    let lock = e
        .call(
            LOAD_FILE,
            &args![loader, LOCK_MODEL_FILE, 0u32, 1u32, 0u32, 0u32, 0u32],
        )
        .u32();
    let loader: u32 = e.global(MODEL_LOADER);
    let pick = e
        .call(
            LOAD_FILE,
            &args![loader, PICK_MODEL_FILE, 0u32, 1u32, 0u32, 0u32, 0u32],
        )
        .u32();
    for word in 0..9 {
        let value: u32 = e.global(IDENTITY_MATRIX + 4 * word);
        e.mem.set_u32(base + PICK_ROTATE + 4 * word, value);
    }
    e.with_stack(0xc, |e, point| {
        let depth = quiet_global(e, PICK_DEPTH);
        let made = e
            .call(POINT_CONSTRUCT, &args![point, 0u32, 0u32, depth])
            .u32();
        for word in 0..3 {
            let value = e.mem.u32(made + 4 * word);
            e.mem.set_u32(base + PICK_TRANSLATE + 4 * word, value);
        }
    });
    e.call(NODE_SET_TRANSLATE, &args![pick, base + PICK_TRANSLATE]);
    e.call(NODE_SET_ROTATE, &args![pick, base + PICK_ROTATE]);
    let block = e.call(NI_ALLOCATE, &args![0xacu32]).u32();
    let node = if block != 0 {
        e.call(NI_NODE_CONSTRUCT, &args![block, 0u32]).u32()
    } else {
        0
    };
    e.call(POINTER_ASSIGN, &args![LOCK_MODEL, node]);
    for model in [lock, pick] {
        let node = lock_model(e);
        e.vcall(node, NODE_SLOT_ATTACH_CHILD, &args![model, 1u32]);
    }
    e.with_stack(0x24, |e, tilt| {
        e.with_stack(0x24, |e, turn| {
            e.with_stack(0x24, |e, product| {
                e.call(NODE_ITEM, &args![tilt]);
                e.call(NODE_ITEM, &args![turn]);
                let angle = quiet_global(e, LOCK_TILT);
                e.call(MATRIX_MAKE_X_ROTATION, &args![tilt, angle]);
                let angle = quiet_global(e, LOCK_TILT);
                e.call(MATRIX_ROTATION_4A0C90, &args![turn, angle]);
                let rotation = e.call(MATRIX_MULTIPLY, &args![tilt, product, turn]).u32();
                let node = lock_model(e);
                e.call(NODE_SET_ROTATE, &args![node, rotation]);
            });
        });
    });
    let x = quiet_global(e, LOCK_MODEL_X);
    let node = lock_model(e);
    e.call(
        NODE_SET_TRANSLATE_XYZ,
        &args![node, x, 0.0f32.to_bits(), 0.0f32.to_bits()],
    );
    e.with_stack(0xc, |e, data| {
        update_data(e, data.addr(), 0.0f32.to_bits(), 0, 0);
        let node = lock_model(e);
        e.call(UPDATE_DOWNWARD, &args![node, data]);
    });
    let lock_manager = e
        .call(GET_CONTROLLER, &args![lock, CONTROLLER_MANAGER_TYPE])
        .u32();
    if lock_manager != 0 {
        let intro = sequence_by_name(e, lock_manager, INTRO_NAME);
        e.mem.set_u32(base + INTRO_ANIM, intro);
        let rotate = sequence_by_name(e, lock_manager, ROTATE_NAME);
        e.mem.set_u32(base + LOCK_ROTATE_ANIM, rotate);
        e.call(DEACTIVATE_ALL, &args![lock_manager, 0.0f32.to_bits()]);
        if e.mem.u32(base + INTRO_ANIM) != 0 {
            let intro = e.mem.u32(base + INTRO_ANIM);
            activate_sequence(e, lock_manager, intro);
        }
        if e.mem.u32(base + LOCK_ROTATE_ANIM) != 0 {
            let rotate = e.mem.u32(base + LOCK_ROTATE_ANIM);
            e.call(SEQUENCE_SET_CYCLE, &args![rotate, 2u32]);
        }
        e.call(START_ANIMATIONS, &args![lock]);
    }
    let pick_manager = e
        .call(GET_CONTROLLER, &args![pick, CONTROLLER_MANAGER_TYPE])
        .u32();
    if pick_manager != 0 {
        e.mem.set_u32(base + PICK_ANIM_MANAGER, pick_manager);
        for (name, offset) in [
            (INTRO_NAME, PICK_INTRO_ANIM),
            (ROTATE_NAME, PICK_BEND_ANIM),
            (BREAK_NAME, PICK_BREAK_ANIM),
        ] {
            let manager = e.mem.u32(base + PICK_ANIM_MANAGER);
            let sequence = sequence_by_name(e, manager, name);
            e.mem.set_u32(base + offset, sequence);
        }
        let manager = e.mem.u32(base + PICK_ANIM_MANAGER);
        e.call(DEACTIVATE_ALL, &args![manager, 0.0f32.to_bits()]);
        if e.mem.u32(base + PICK_INTRO_ANIM) != 0 {
            let manager = e.mem.u32(base + PICK_ANIM_MANAGER);
            let intro = e.mem.u32(base + PICK_INTRO_ANIM);
            activate_sequence(e, manager, intro);
        }
        if e.mem.u32(base + PICK_BREAK_ANIM) != 0 {
            let sequence = e.mem.u32(base + PICK_BREAK_ANIM);
            e.call(SEQUENCE_SET_CYCLE, &args![sequence, 2u32]);
        }
        e.call(START_ANIMATIONS, &args![pick]);
    }
    let camera = lock_pick_menu_create_3d_camera(e, this);
    e.call(POINTER_ASSIGN, &args![CAMERA, camera]);
    let root = e.call(SCENE_ROOT, &args![1u32]).u32();
    let node = lock_model(e);
    e.vcall(root, NODE_SLOT_ATTACH_CHILD, &args![node, 1u32]);
    let node = lock_model(e);
    let bound = e.call(GET_WORLD_BOUND, &args![node]).u32();
    let radius = e.call(BOUND_RADIUS, &args![bound]).f64();
    let factor: f64 = e.global(LIGHT_RADIUS_FACTOR);
    let radius = x87_store32(x87_mul(radius, factor));
    let node = lock_model(e);
    let root = e.call(SCENE_ROOT, &args![1u32]).u32();
    e.call(
        ADD_LIGHTS_FROM_NODE,
        &args![root, node, x87_quiet_bits(radius)],
    );
}

// Translated from 0078e7c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0078e7c0 - the C++ exception-unwinding frame (FS:[0]) that releases the texture pointer when a call throws is not translated
/// The lock pick menu's 3D render (unnamed in the engine map; called by
/// the interface), cdecl without words: with a lock model, starts the
/// renderer's pass (`007148c0(0, 4)`), sets the culling process up for the
/// camera (`sp3DCamera`, with the field of view at `01030ff4`) and the
/// interface's rendered texture (held in an `NiPointer` local),
/// accumulates scene root 1 through the camera (or the scene's own camera
/// when there is none), renders it into the texture, and clears the
/// culling target, the local pointer and the culling process.
pub fn fn_0078e7c0(e: &mut Engine) {
    if lock_model(e) == 0 {
        return;
    }
    let renderer = e.call(RENDERER, &args![]).u32();
    e.call(RENDERER_BEGIN_7148C0, &args![renderer, 0u32, 4u32]);
    let scene = e.call(SCENE_OBJECT, &args![]).u32();
    let culling = e.call(CULLING_PROCESS, &args![scene]).u32();
    e.call(CULLING_SET_C4F270, &args![culling, 1u32]);
    let camera = e.call(POINTER_GET, &args![CAMERA]).u32();
    let fov = quiet_global(e, RENDER_FOV);
    e.call(CAMERA_SET_FOV_709D50, &args![camera, fov]);
    let interface = e.call(INTERFACE_OBJECT, &args![]).u32();
    let texture = e.call(RENDERED_TEXTURE_7FD640, &args![interface]).u32();
    e.with_stack(4, |e, held| {
        e.call(POINTER_CONSTRUCT, &args![held, texture]);
        let target = e.call(POINTER_GET, &args![held]).u32();
        e.call(CULLING_SET_TARGET, &args![culling, target]);
        let source = if camera != 0 {
            camera
        } else {
            e.call(SCENE_CAMERA_6629F0, &args![scene]).u32()
        };
        let root = e.call(SCENE_ROOT, &args![1u32]).u32();
        e.call(ACCUMULATE_SCENE, &args![source, root, culling]);
        let target = e.call(POINTER_GET, &args![held]).u32();
        e.call(RENDER_SCENE_B6C0D0, &args![source, target, 0u32]);
        e.call(CULLING_SET_TARGET, &args![culling, 0u32]);
        e.call(POINTER_ASSIGN, &args![held, 0u32]);
        e.call(CULLING_RESET_C4F2D0, &args![culling]);
        e.call(POINTER_DESTROY, &args![held]);
    });
}

// Translated from 0078e900 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0078e900 - the C++ exception-unwinding frame (FS:[0]) that frees the camera allocation when its constructor throws is not translated
/// `LockPickMenu::Create3DCamera` (Xbox PDB): makes a camera (0x114
/// bytes), gives it the viewport `(0, 1, 1, 0)`, the aspect of the scene's
/// far distance over the near setting, and a frustum (a 0x1C-byte local:
/// near the setting at `01203144`, far the scene's slot +0x100) from the
/// field of view setting (`01203150`, in degrees times the `double`s at
/// `01023128` and `01070c80`) and the screen aspect (the override at
/// `011f9434` or the screen width over the height), looks at
/// `(0, 0, 1)` with up `(0, 0, 1)`, sets its LOD scale to 1.0 and updates
/// it. Answers the camera.
pub fn lock_pick_menu_create_3d_camera(e: &mut Engine, _this: Ptr) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![0x114u32]).u32();
    let camera = if block != 0 {
        e.call(CAMERA_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.with_stack(0x10, |e, rect| {
        e.call(
            RECT_CONSTRUCT,
            &args![
                rect,
                0.0f32.to_bits(),
                1.0f32.to_bits(),
                1.0f32.to_bits(),
                0.0f32.to_bits()
            ],
        );
        e.call(CAMERA_SET_VIEWPORT, &args![camera, rect]);
    });
    e.with_stack(0x1c, |e, frustum| {
        let frustum = frustum.addr();
        e.call(FRUSTUM_CONSTRUCT, &args![frustum, 0u32]);
        let scene = e.call(SCENE_OBJECT, &args![]).u32();
        let far = e.vcall(scene, SCENE_SLOT_FAR, &args![]).f64();
        e.mem.set_u32(frustum + 0x14, x87_store32(far));
        let near_at = e.call(SETTING_FLOAT, &args![NEAR_SETTING]).u32();
        e.mem.set_u32(frustum + 0x10, quiet_field(e, near_at));
        let ratio = x87_store32(x87_div(
            x87_load32(e.mem.u32(frustum + 0x14)),
            x87_m32(e.mem.u32(frustum + 0x10)),
        ));
        e.call(CAMERA_SET_ASPECT, &args![camera, x87_quiet_bits(ratio)]);
        let width = if e.global::<u8>(SCREEN_WIDTH_OVERRIDE_SET) != 0 {
            e.global::<i32>(SCREEN_WIDTH_OVERRIDE)
        } else {
            e.call(SCREEN_WIDTH, &args![]).i32()
        };
        let width = f64::from(width);
        let height = f64::from(e.call(SCREEN_HEIGHT, &args![]).i32());
        let aspect = x87_store32(x87_div(width, height));
        let scale = quiet_global(e, FRUSTUM_SCALE);
        let fov_at = e.call(SETTING_FLOAT, &args![FOV_SETTING]).u32();
        let first: f64 = e.global(DEGREES_TO_RADIANS);
        let second: f64 = e.global(HALF_70C80);
        let fov = x87_store32(x87_mul(
            x87_mul(x87_load32(e.mem.u32(fov_at)), first),
            second,
        ));
        let tangent = |e: &mut Engine| e.call(TANGENT, &args![x87_quiet_bits(fov)]).f64();
        let value = tangent(e);
        e.mem
            .set_u32(frustum, x87_store32(x87_mul(-value, x87_m32(scale))));
        let value = tangent(e);
        e.mem
            .set_u32(frustum + 4, x87_store32(x87_mul(value, x87_m32(scale))));
        let value = tangent(e);
        e.mem.set_u32(
            frustum + 0xc,
            x87_store32(x87_mul(x87_mul(-value, x87_m32(aspect)), x87_m32(scale))),
        );
        let value = tangent(e);
        e.mem.set_u32(
            frustum + 8,
            x87_store32(x87_mul(x87_mul(value, x87_m32(aspect)), x87_m32(scale))),
        );
        e.mem.set_u8(frustum + 0x18, 0);
        e.call(CAMERA_SET_FRUSTUM, &args![camera, frustum]);
    });
    e.with_stack(0xc, |e, up| {
        e.with_stack(0xc, |e, point| {
            let up = e
                .call(
                    POINT_CONSTRUCT,
                    &args![up, 0.0f32.to_bits(), 0.0f32.to_bits(), 1.0f32.to_bits()],
                )
                .u32();
            let point = e
                .call(
                    POINT_CONSTRUCT,
                    &args![point, 0.0f32.to_bits(), 0.0f32.to_bits(), 1.0f32.to_bits()],
                )
                .u32();
            e.call(CAMERA_LOOK_AT, &args![camera, point, up]);
        });
    });
    e.call(CAMERA_SET_LOD, &args![camera, 1.0f32.to_bits()]);
    e.with_stack(0xc, |e, data| {
        update_data(e, data.addr(), 0.0f32.to_bits(), 0, 0);
        e.call(UPDATE_DOWNWARD, &args![camera, data]);
    });
    camera
}

/// The state of one `DoIdle` call shared by its parts.
struct Idle {
    base: u32,
    /// The ticks since the last idle (`dt`).
    ticks: i32,
    /// The gamepad's turn input (1.0 without a gamepad), as float bits.
    input: u32,
    /// Whether the menu is the top menu.
    top: bool,
}

fn state_seconds(e: &Engine) -> u32 {
    quiet_global(e, STATE_SECONDS)
}

fn seconds_reached(e: &mut Engine, sequence: u32) -> bool {
    let end = e.call(SEQUENCE_END_KEY, &args![sequence]).f64();
    let seconds = x87_load32(e.global::<u32>(STATE_SECONDS));
    // `TEST AH,1`: the state goes on unless the seconds are below the end (or a NaN).
    seconds >= end
}

// Translated from 0078eb50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0078eb50 - the C++ exception-unwinding frames (FS:[0]) of the sound handles and the static sound's construction, and the /GS stack cookie check (00ec408c), are not translated
/// `LockPickMenu::DoIdle` (Xbox PDB): constructs the looping sound static
/// once, counts the seconds spent in the current state (ticks over the
/// `double` at `01017b70`, reset when the state changed), keeps the tick
/// and state, waits for the tutorial when it is showing, dims the root
/// when the menu is not on top, reads the gamepad's turn input, then runs
/// the state (`eLockState`): 0 and 1 play the lock and pick intros and go
/// to 2; 2 shows the tutorial or lets the player turn the pick and starts
/// the turning (3) on input, turning the lock back otherwise; 3 turns the
/// lock up to the pick's limit, opens the lock (or reports it unlocked)
/// at the end, or starts bending the pick (4); 4 bends the pick, wears its
/// health down and breaks it (5) when the health runs out; 5 waits for the
/// break animation (6); 6 puts a new pick in after a second or ends (1 or
/// 2); 7 closes the menu. See the parts below for every step.
pub fn lock_pick_menu_do_idle(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let guard: u32 = e.global(LOOP_SOUND_GUARD);
    if guard & 1 == 0 {
        e.set_global(LOOP_SOUND_GUARD, guard | 1);
        e.call(SOUND_HANDLE_CONSTRUCT, &args![LOOP_SOUND]);
        e.call(ATEXIT, &args![LOOP_SOUND_EXIT]);
    }
    let class = e.vcall(base, 0x34, &args![]).u32();
    let top = e.call(IS_TOP_MENU, &args![class]).bool();
    let mut ticks = 0i32;
    if e.mem.u32(base + PREV_IDLE_TICK) == 0 {
        e.set_global(STATE_SECONDS, 0.0f32.to_bits());
        e.set_global(LAST_STATE, 0u32);
    } else {
        let now = e.call(TICKS, &args![TICK_SOURCE]).u32();
        ticks = now.wrapping_sub(e.mem.u32(base + PREV_IDLE_TICK)) as i32;
        if e.global::<u32>(LAST_STATE) != e.mem.u32(base + LOCK_STATE) {
            e.set_global(STATE_SECONDS, 0.0f32.to_bits());
        } else {
            let per_second: f64 = e.global(TICKS_PER_SECOND);
            let seconds = x87_add(
                x87_div(f64::from(ticks), per_second),
                x87_m32(e.global::<u32>(STATE_SECONDS)),
            );
            e.set_global(STATE_SECONDS, x87_store32(seconds));
        }
    }
    let now = e.call(TICKS, &args![TICK_SOURCE]).u32();
    e.mem.set_u32(base + PREV_IDLE_TICK, now);
    let state = e.mem.u32(base + LOCK_STATE);
    e.set_global(LAST_STATE, state);
    if e.mem.u8(base + SHOWING_TUTORIAL) != 0 {
        if top && tutorial_seen(e) {
            e.mem.set_u8(base + SHOWING_TUTORIAL, 0);
        } else {
            return;
        }
    }
    if !top {
        set_root_alpha(e);
    }
    let mut input = 1.0f32.to_bits();
    // The interface object is fetched and not used.
    e.call(INTERFACE_OBJECT, &args![]);
    if e.call(USING_GAMEPAD, &args![]).bool() {
        let device = controls_device(e);
        let mut axis = e.call(CONTROL_AXIS, &args![device, 0u32, 9u32]).i32();
        let magnitude = e
            .call(FLOAT_ABS, &args![x87_store32(f64::from(axis))])
            .f64();
        // `TEST AH,0x41; JP`: a value at most the dead zone clears the axis.
        if magnitude <= e.global::<f64>(DEAD_ZONE_745F8) {
            axis = 0;
        }
        let scale: f64 = e.global(STICK_SCALE);
        input = x87_store32(x87_div(f64::from(axis.wrapping_neg()), scale));
    }
    let mut idle = Idle {
        base,
        ticks,
        input,
        top,
    };
    match e.mem.u32(base + LOCK_STATE) {
        0 => idle_intro(e, &idle),
        1 => idle_pick_intro(e, &idle),
        2 => idle_ready(e, &idle),
        3 => idle_turning(e, &mut idle),
        4 => idle_bending(e, &idle),
        5 => idle_breaking(e, &idle),
        6 => idle_new_pick(e, &idle),
        7 => idle_closing(e, &idle),
        _ => {}
    }
}

/// Whether the lock pick tutorial (`00790290`'s message) was seen.
fn tutorial_seen(e: &mut Engine) -> bool {
    let message = fn_00790290(e);
    let interface = e.call(INTERFACE_OBJECT, &args![]).u32();
    e.call(
        TUTORIAL_SEEN,
        &args![interface + INTERFACE_MANAGER_TUTORIALS, message],
    )
    .bool()
}

/// Sets the interface root's trait 0xfa9 to the float at `01023cd8`.
fn set_root_alpha(e: &mut Engine) {
    let alpha = quiet_global(e, ENTER_ALPHA);
    let interface = e.call(INTERFACE_OBJECT, &args![]).u32();
    let root = e.call(INTERFACE_ROOT_TILE, &args![interface]).u32();
    e.call(TILE_SET_FLOAT, &args![root, 0xfa9u32, alpha, 1u32]);
}

/// The lock picks left are shown in trait `iUT_Value` of tile +0x30.
fn picks_left(e: &mut Engine, base: u32) -> f64 {
    let value_trait: u32 = e.global(USER_TRAIT_VALUE);
    let tile = tile_at(e, base, 1);
    tile_value(e, tile, value_trait)
}

fn show_no_picks(e: &mut Engine) {
    let seconds = quiet_global(e, MESSAGE_SECONDS);
    let text = e.call(SETTING_STRING, &args![NO_PICKS_TEXT]).u32();
    e.call(
        SHOW_MESSAGE,
        &args![text, 0u32, NO_PICKS_SOUND, 0u32, seconds, 0u32],
    );
}

/// State 0: the lock's and the pick's intro animations.
fn idle_intro(e: &mut Engine, idle: &Idle) {
    let base = idle.base;
    if e.mem.u32(base + INTRO_ANIM) == 0 || e.mem.u32(base + PICK_INTRO_ANIM) == 0 {
        return;
    }
    e.with_stack(0xc, |e, lock_data| {
        e.with_stack(0xc, |e, pick_data| {
            let seconds = state_seconds(e);
            update_data(e, lock_data.addr(), seconds, 1, 0);
            let seconds = state_seconds(e);
            update_data(e, pick_data.addr(), seconds, 1, 0);
            let child = lock_model_child(e, 0);
            e.call(UPDATE_DOWNWARD, &args![child, lock_data]);
            let zero: f64 = e.global(ZERO);
            let seconds = x87_load32(e.global::<u32>(STATE_SECONDS));
            let update_pick =
                seconds == zero || e.mem.u32(base + LOCK_TARGET) == 0 || picks_left(e, base) > zero;
            if update_pick {
                let child = lock_model_child(e, 1);
                e.call(UPDATE_DOWNWARD, &args![child, pick_data]);
            }
            let seconds = x87_load32(e.global::<u32>(STATE_SECONDS));
            if seconds == zero && e.mem.u32(base + LOCK_TARGET) != 0 {
                if picks_left(e, base) <= zero {
                    show_no_picks(e);
                } else {
                    play_sound(e, BLOCKED_SOUND, 0x121);
                }
            }
        });
    });
    let intro = e.mem.u32(base + INTRO_ANIM);
    if !seconds_reached(e, intro) {
        return;
    }
    let child = lock_model_child(e, 0);
    let manager = e
        .call(GET_CONTROLLER, &args![child, CONTROLLER_MANAGER_TYPE])
        .u32();
    let model = lock_model(e);
    let first = e.call(NODE_CHILD, &args![model, 0u32]).u32();
    let second = e.call(NODE_CHILD, &args![first, 0u32]).u32();
    let keyhole = e.call(NODE_CHILD, &args![second, 2u32]).u32();
    let point = e.call(POINT_OF_NODE, &args![keyhole]).u32();
    e.with_stack(0xc, |e, copy| {
        for word in 0..3 {
            let value = e.mem.u32(point + 4 * word);
            e.mem.set_u32(copy.addr() + 4 * word, value);
        }
    });
    e.mem.set_u32(base + LOCK_STATE, 2);
    let intro = e.mem.u32(base + INTRO_ANIM);
    deactivate_sequence(e, manager, intro);
    let rotate = e.mem.u32(base + LOCK_ROTATE_ANIM);
    activate_sequence(e, manager, rotate);
    let pick_manager = e.mem.u32(base + PICK_ANIM_MANAGER);
    let pick_intro = e.mem.u32(base + PICK_INTRO_ANIM);
    deactivate_sequence(e, pick_manager, pick_intro);
    let pick_manager = e.mem.u32(base + PICK_ANIM_MANAGER);
    let bend = e.mem.u32(base + PICK_BEND_ANIM);
    activate_sequence(e, pick_manager, bend);
}

/// State 1: only the pick's intro animation.
fn idle_pick_intro(e: &mut Engine, idle: &Idle) {
    let base = idle.base;
    if e.mem.u32(base + PICK_INTRO_ANIM) == 0 {
        return;
    }
    e.with_stack(0xc, |e, data| {
        let seconds = state_seconds(e);
        update_data(e, data.addr(), seconds, 1, 0);
        let child = lock_model_child(e, 1);
        e.call(UPDATE_DOWNWARD, &args![child, data]);
    });
    let intro = e.mem.u32(base + PICK_INTRO_ANIM);
    if seconds_reached(e, intro) {
        e.mem.set_u32(base + LOCK_STATE, 2);
        let manager = e.mem.u32(base + PICK_ANIM_MANAGER);
        let intro = e.mem.u32(base + PICK_INTRO_ANIM);
        deactivate_sequence(e, manager, intro);
        let manager = e.mem.u32(base + PICK_ANIM_MANAGER);
        let bend = e.mem.u32(base + PICK_BEND_ANIM);
        activate_sequence(e, manager, bend);
    }
}

/// Plays the click sounds when the lock angle crossed the two click angles
/// from `before` to `after` in the direction `rising`.
fn click_sounds(e: &mut Engine, before: u32, after: u32, rising: bool) {
    for (angle, sound) in [
        (CLICK_ANGLE_A, CLICK_SOUND_A),
        (CLICK_ANGLE_B, CLICK_SOUND_B),
    ] {
        let mark: f64 = e.global(angle);
        let (before, after) = (x87_load32(before), x87_load32(after));
        let crossed = if rising {
            before < mark && after > mark
        } else {
            after < mark && before > mark
        };
        if crossed {
            play_sound(e, sound, 0x101);
        }
    }
}

/// State 2: the pick is ready.
fn idle_ready(e: &mut Engine, idle: &Idle) {
    let base = idle.base;
    if e.call(TUTORIAL_FLAG_5D4A40, &args![]).bool() && !tutorial_seen(e) {
        let class = fn_0078e080(e);
        let message = fn_00790290(e);
        let interface = e.call(INTERFACE_OBJECT, &args![]).u32();
        let shown = e
            .call(
                TUTORIAL_SHOW,
                &args![
                    interface + INTERFACE_MANAGER_TUTORIALS,
                    message,
                    class,
                    0u32
                ],
            )
            .u8();
        e.mem.set_u8(base + SHOWING_TUTORIAL, shown);
        return;
    }
    if !idle.top {
        return;
    }
    let zero: f64 = e.global(ZERO);
    let blocked = e.mem.u32(base + LOCK_TARGET) != 0 && !(picks_left(e, base) > zero);
    if !blocked {
        let start = if e.call(USING_GAMEPAD, &args![]).bool() && x87_load32(idle.input) > zero {
            true
        } else if e.call(USING_GAMEPAD, &args![]).bool() {
            false
        } else {
            fn_00791540(e, Ptr::new(base), 1)
        };
        if start {
            play_sound(e, START_SOUND, 0x121);
            e.mem.set_u32(base + LOCK_STATE, 3);
        }
        lock_pick_menu_rotate_lockpick(e, Ptr::new(base));
    }
    if !(x87_load32(e.mem.u32(base + LOCK_ANGLE)) > zero) {
        return;
    }
    let before = quiet_field(e, base + LOCK_ANGLE);
    let per_second: f64 = e.global(TICKS_PER_SECOND);
    let turn = x87_div(f64::from(idle.ticks.wrapping_mul(0x5a)), per_second);
    let angle = x87_sub(x87_m32(e.mem.u32(base + LOCK_ANGLE)), turn);
    e.mem.set_u32(base + LOCK_ANGLE, x87_store32(angle));
    let angle = quiet_field(e, base + LOCK_ANGLE);
    let angle = e.call(FLOAT_MAX, &args![angle, 0.0f32.to_bits()]).f64();
    e.mem.set_u32(base + LOCK_ANGLE, x87_store32(angle));
    lock_pick_menu_rotate_lock(e, Ptr::new(base), 0.0f32.to_bits());
    let after = e.mem.u32(base + LOCK_ANGLE);
    click_sounds(e, before, after, false);
}

/// The input test of states 3 and 4: whether the player keeps turning.
fn keeps_turning(e: &mut Engine, idle: &Idle) -> bool {
    if !idle.top {
        return false;
    }
    let zero: f64 = e.global(ZERO);
    if e.call(USING_GAMEPAD, &args![]).bool() && x87_load32(idle.input) <= zero {
        return false;
    }
    if e.call(USING_GAMEPAD, &args![]).bool() {
        return true;
    }
    fn_00791540(e, Ptr::new(idle.base), 0)
}

/// State 3: the lock turns while the player turns it.
fn idle_turning(e: &mut Engine, idle: &mut Idle) {
    let base = idle.base;
    if e.mem.u8(base + FORCE_LOCK_SUCCESS) == 0 && !keeps_turning(e, idle) {
        let device = controls_device(e);
        e.call(CONTROL_STOP_RUMBLE, &args![device]);
        e.mem.set_u32(base + LOCK_STATE, 2);
        return;
    }
    let before = quiet_field(e, base + LOCK_ANGLE);
    if e.mem.u8(base + FORCE_LOCK_SUCCESS) != 0 {
        idle.input = 1.0f32.to_bits();
    }
    let angle = x87_load32(e.mem.u32(base + LOCK_ANGLE));
    let limit = x87_load32(e.mem.u32(base + MAX_LOCK_ANGLE));
    if limit >= angle {
        let per_second: f64 = e.global(TICKS_PER_SECOND);
        let turn = x87_div(
            x87_mul(
                f64::from(idle.ticks.wrapping_mul(0x5a)),
                x87_m32(idle.input),
            ),
            per_second,
        );
        let angle = x87_add(turn, x87_m32(e.mem.u32(base + LOCK_ANGLE)));
        e.mem.set_u32(base + LOCK_ANGLE, x87_store32(angle));
        let limit = quiet_field(e, base + MAX_LOCK_ANGLE);
        let angle = quiet_field(e, base + LOCK_ANGLE);
        let angle = e.call(FLOAT_MIN, &args![angle, limit]).f64();
        e.mem.set_u32(base + LOCK_ANGLE, x87_store32(angle));
    }
    lock_pick_menu_rotate_lock(e, Ptr::new(base), 0.0f32.to_bits());
    if e.call(USING_GAMEPAD, &args![]).bool() {
        let strength = quiet_global(e, RUMBLE_STRENGTH);
        let device = controls_device(e);
        e.call(
            CONTROL_RUMBLE,
            &args![
                device,
                strength,
                0.0f32.to_bits(),
                0xfau32,
                0x7du32,
                3u32,
                1u32,
                0u32
            ],
        );
    }
    let after = e.mem.u32(base + LOCK_ANGLE);
    click_sounds(e, before, after, true);
    let angle = x87_load32(e.mem.u32(base + LOCK_ANGLE));
    let limit = x87_load32(e.mem.u32(base + MAX_LOCK_ANGLE));
    if !(limit <= angle) {
        return;
    }
    let open: f64 = e.global(ANGLE_OFFSET_66808);
    if angle >= open {
        if e.mem.u32(base + LOCK_TARGET) != 0 {
            open_lock(e, base);
        } else {
            let tile = tile_at(e, base, 11);
            e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, UNLOCKED_TEXT, 1u32]);
            let tile = tile_at(e, base, 11);
            e.call(TILE_SET_VALUE, &args![tile, 0xfa3u32, 1u32]);
            if e.mem.u32(base + LOCK_TARGET) == 0 {
                let value_trait: u32 = e.global(USER_TRAIT_VALUE);
                let tile = tile_at(e, base, 1);
                e.call(TILE_SET_VALUE, &args![tile, value_trait, 0u32]);
            }
            e.set_global(PICK_BROKE, 1u8);
            e.mem.set_u32(base + LOCK_STATE, 6);
        }
    } else {
        play_sound(e, BEND_SOUND, 0x101);
        e.with_stack(0xc, |e, handle| {
            let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
            let found = e
                .call(
                    GET_SOUND_HANDLE_BY_NAME,
                    &args![audio, handle, STRAIN_SOUND, 1u32],
                )
                .u32();
            e.call(SOUND_HANDLE_ASSIGN, &args![LOOP_SOUND, found]);
            e.call(SOUND_HANDLE_DESTROY, &args![handle]);
        });
        e.call(SOUND_PLAY, &args![LOOP_SOUND, 1u32]);
        e.mem.set_u32(base + LOCK_STATE, 4);
    }
    let device = controls_device(e);
    e.call(CONTROL_STOP_RUMBLE, &args![device]);
}

/// The unlocking of state 3 with a target.
fn open_lock(e: &mut Engine, base: u32) {
    e.call(MISC_STAT_INCREMENT, &args![4u32]);
    let target = e.mem.u32(base + LOCK_TARGET);
    let lock = e.call(GET_LOCK, &args![target]).u32();
    if e.mem.u32(lock + 0x10) == 0 {
        let level = e.mem.u32(base + LOCK_LEVEL);
        let experience = e.call(GET_EXPERIENCE, &args![3u32, level]).u32();
        let player = the_player(e);
        e.vcall(player, PLAYER_SLOT_REWARD_XP, &args![experience]);
        let target = e.mem.u32(base + LOCK_TARGET);
        let lock = e.call(GET_LOCK, &args![target]).u32();
        let count = e.mem.u32(lock + 0x10).wrapping_add(1);
        e.mem.set_u32(lock + 0x10, count);
        let target = e.mem.u32(base + LOCK_TARGET);
        e.call(ADD_LOCK_CHANGE, &args![target]);
    }
    let target = e.mem.u32(base + LOCK_TARGET);
    if e.call(GET_OWNER, &args![target]).u32() != 0 {
        let target = e.mem.u32(base + LOCK_TARGET);
        let owner = e.call(GET_OWNER, &args![target]).u32();
        let actor = if e.call(FORM_TYPE, &args![owner]).u32() == 8 {
            e.call(ACTOR_REF_FOR_BASE, &args![PROCESS_LISTS, owner, 1u32, 1u32])
                .u32()
        } else {
            e.call(ACTOR_REF_IN_HIGH, &args![PROCESS_LISTS, owner, 1u32])
                .u32()
        };
        if actor != 0 {
            let player = the_player(e);
            e.call(SET_WANT_WEAPON_DRAWN, &args![player, 1u32]);
            let player = the_player(e);
            e.call(STEAL_ALARM, &args![player, actor, 0u32, 0u32, 0u32, owner]);
        } else {
            let mut karma = false;
            if owner != 0 && e.call(FORM_TYPE, &args![owner]).u32() == 8 {
                if !e.call(BASE_IS_EVIL_47D7C0, &args![owner]).bool() {
                    karma = true;
                }
            } else if owner != 0 {
                let value = e.vcall(owner + 0x100, 0xc, &args![0x17u32]).f64();
                let alignment = e
                    .call(ALIGNMENT_FOR_KARMA, &args![x87_store32(value)])
                    .u32();
                if alignment != 2 && alignment != 4 {
                    karma = true;
                }
            }
            if karma {
                let amount_at = e.call(SETTING_FLOAT, &args![KARMA_SETTING]).u32();
                let amount = ftol(e, x87_load32(e.mem.u32(amount_at)));
                let player = the_player(e);
                e.call(REWARD_KARMA, &args![player, amount as u32]);
            }
        }
    }
    play_sound(e, UNLOCK_SOUND, 0x101);
    let target = e.mem.u32(base + LOCK_TARGET);
    e.call(UNLOCK, &args![target]);
    let target = e.mem.u32(base + LOCK_TARGET);
    if e.call(FORM_TYPE, &args![target]).u32() == 0x1c {
        let player = the_player(e);
        e.mem.set_u8(player + 0x20c, 1);
    }
    let target = e.mem.u32(base + LOCK_TARGET);
    if e.call(SAVE_FORM_LINK, &args![target]).u32() != 0 {
        let target = e.mem.u32(base + LOCK_TARGET);
        let linked = e.call(SAVE_FORM_LINK, &args![target]).u32();
        if e.call(FORM_TYPE, &args![linked]).u32() == 0x1b {
            e.call(CONTAINER_MENU_75B6E0, &args![1u32]);
        }
    }
    let player = the_player(e);
    let target = e.mem.u32(base + LOCK_TARGET);
    e.call(ACTIVATE, &args![target, player, 0u32, 0u32, 1u32]);
    let target = e.mem.u32(base + LOCK_TARGET);
    if e.call(FORM_TYPE, &args![target]).u32() == 0x1c {
        let player = the_player(e);
        e.mem.set_u8(player + 0x20c, 0);
    }
    fn_0078e0d0(e);
}

/// State 4: the pick bends while the player keeps turning.
fn idle_bending(e: &mut Engine, idle: &Idle) {
    let base = idle.base;
    if !bending_input(e, idle) {
        e.call(SOUND_FADE_OUT, &args![LOOP_SOUND, 0x1f4u32]);
        let device = controls_device(e);
        e.call(CONTROL_STOP_RUMBLE, &args![device]);
        e.mem.set_u32(base + LOCK_STATE, 2);
        return;
    }
    e.with_stack(0xc, |e, data| {
        update_data(e, data.addr(), 0.0f32.to_bits(), 1, 0);
        let per_second: f64 = e.global(TICKS_PER_SECOND);
        let seconds = x87_store32(x87_div(f64::from(idle.ticks), per_second));
        let zero: f64 = e.global(ZERO);
        let health_at = e.call(SETTING_FLOAT, &args![HEALTH_SETTING]).u32();
        if x87_load32(e.mem.u32(health_at)) != zero {
            let health_at = e.call(SETTING_FLOAT, &args![HEALTH_SETTING]).u32();
            let wear = x87_div(x87_load32(seconds), x87_m32(e.mem.u32(health_at)));
            let health = x87_sub(x87_m32(e.mem.u32(base + PICK_HEALTH)), wear);
            e.mem.set_u32(base + PICK_HEALTH, x87_store32(health));
        }
        let health = quiet_field(e, base + PICK_HEALTH);
        let health = e.call(FLOAT_MAX, &args![health, 0.0f32.to_bits()]).f64();
        e.mem.set_u32(base + PICK_HEALTH, x87_store32(health));
        let sign = f64::from(e.global::<i32>(SHAKE_SIGN));
        let factor: f64 = e.global(SHAKE_ANGLE);
        let shake = x87_store32(x87_mul(sign, factor));
        lock_pick_menu_rotate_lock(e, Ptr::new(base), x87_quiet_bits(shake));
        if e.call(USING_GAMEPAD, &args![]).bool() {
            let strength = quiet_global(e, BEND_RUMBLE);
            let device = controls_device(e);
            e.call(
                CONTROL_RUMBLE,
                &args![
                    device,
                    strength,
                    0.0f32.to_bits(),
                    0xfau32,
                    0u32,
                    0u32,
                    0u32,
                    0u32
                ],
            );
        }
        if e.mem.u32(base + PICK_BEND_ANIM) != 0 {
            let bend = e.mem.u32(base + PICK_BEND_ANIM);
            let begin = e.call(SEQUENCE_BEGIN_KEY, &args![bend]).f64();
            let worn = x87_sub(1.0, x87_load32(e.mem.u32(base + PICK_HEALTH)));
            let end = e.call(SEQUENCE_END_KEY, &args![bend]).f64();
            let begin_again = e.call(SEQUENCE_BEGIN_KEY, &args![bend]).f64();
            let phase = x87_store32(x87_add(x87_mul(x87_sub(end, begin_again), worn), begin));
            e.call(
                SEQUENCE_SET_PHASE,
                &args![bend, x87_quiet_bits(phase), 0u32],
            );
            let model = lock_model(e);
            e.call(UPDATE_DOWNWARD, &args![model, data]);
        }
    });
    let x_trait: u32 = e.global(USER_TRAIT_X);
    let tile = tile_at(e, base, 15);
    let x = tile_value(e, tile, x_trait);
    let factor: f64 = e.global(SHAKE_TURN);
    let turn = x87_mul(f64::from(e.global::<i32>(SHAKE_SIGN)), factor);
    let angle = x87_store32(x87_add(x, turn));
    lock_pick_menu_rotate_lockpick_model(e, Ptr::new(base), x87_quiet_bits(angle));
    let sign = e.global::<i32>(SHAKE_SIGN).wrapping_neg();
    e.set_global(SHAKE_SIGN, sign);
    let factor: f64 = e.global(HEALTH_PERCENT);
    let percent = x87_store32(x87_mul(x87_load32(e.mem.u32(base + PICK_HEALTH)), factor));
    let rounded = e
        .call(
            POINT_PERCENT_4BD510,
            &args![x87_quiet_bits(percent), 1.0f32.to_bits()],
        )
        .f64();
    let rounded = ftol(e, rounded);
    e.with_stack(0x20, |e, text| {
        e.call(
            PRINT_FORMATTED,
            &args![text, 0x20u32, HEALTH_FORMAT, rounded as u32, HEALTH_SUFFIX],
        );
        let tile = tile_at(e, base, 12);
        e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, text, 1u32]);
    });
    let zero: f64 = e.global(ZERO);
    if !(x87_load32(e.mem.u32(base + PICK_HEALTH)) <= zero) {
        return;
    }
    e.call(SOUND_FADE_OUT, &args![LOOP_SOUND, 0x64u32]);
    play_sound(e, BREAK_SOUND, 0x101);
    if e.mem.u32(base + LOCK_TARGET) != 0 {
        let item: u32 = e.global(LOCKPICK_ITEM);
        let player = the_player(e);
        e.vcall(
            player,
            PLAYER_SLOT_REMOVE_ITEM,
            &args![item, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
        );
    }
    let value_trait: u32 = e.global(USER_TRAIT_VALUE);
    let tile = tile_at(e, base, 1);
    let count = tile_value(e, tile, value_trait);
    let one: f64 = e.global(ONE);
    let count = x87_store32(x87_sub(count, one));
    let value_trait: u32 = e.global(USER_TRAIT_VALUE);
    let tile = tile_at(e, base, 1);
    e.call(
        TILE_SET_FLOAT,
        &args![tile, value_trait, x87_quiet_bits(count), 1u32],
    );
    let manager = e.mem.u32(base + PICK_ANIM_MANAGER);
    let bend = e.mem.u32(base + PICK_BEND_ANIM);
    deactivate_sequence(e, manager, bend);
    let manager = e.mem.u32(base + PICK_ANIM_MANAGER);
    let broken = e.mem.u32(base + PICK_BREAK_ANIM);
    activate_sequence(e, manager, broken);
    let device = controls_device(e);
    e.call(CONTROL_STOP_RUMBLE, &args![device]);
    e.mem.set_u32(base + LOCK_STATE, 5);
}

/// The input test of state 4 (the interface object is fetched twice, as the
/// code does).
fn bending_input(e: &mut Engine, idle: &Idle) -> bool {
    if !idle.top {
        return false;
    }
    e.call(INTERFACE_OBJECT, &args![]);
    let zero: f64 = e.global(ZERO);
    if e.call(USING_GAMEPAD, &args![]).bool() && x87_load32(idle.input) <= zero {
        return false;
    }
    e.call(INTERFACE_OBJECT, &args![]);
    if e.call(USING_GAMEPAD, &args![]).bool() {
        return true;
    }
    fn_00791540(e, Ptr::new(idle.base), 0)
}

/// State 5: the pick's break animation.
fn idle_breaking(e: &mut Engine, idle: &Idle) {
    let base = idle.base;
    if e.mem.u32(base + PICK_BREAK_ANIM) == 0 || !idle.top {
        return;
    }
    e.with_stack(0xc, |e, data| {
        let seconds = state_seconds(e);
        update_data(e, data.addr(), seconds, 1, 0);
        let child = lock_model_child(e, 1);
        e.call(UPDATE_DOWNWARD, &args![child, data]);
    });
    let broken = e.mem.u32(base + PICK_BREAK_ANIM);
    if seconds_reached(e, broken) {
        e.mem.set_u32(base + LOCK_STATE, 6);
    }
}

/// State 6: a new pick after a second.
fn idle_new_pick(e: &mut Engine, idle: &Idle) {
    let base = idle.base;
    let one: f64 = e.global(ONE);
    let seconds = x87_load32(e.global::<u32>(STATE_SECONDS));
    if !(seconds >= one) || !idle.top {
        return;
    }
    let tile = tile_at(e, base, 11);
    e.call(TILE_SET_VALUE, &args![tile, 0xfa3u32, 0u32]);
    let tile = tile_at(e, base, 12);
    e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, TEXT_74418, 1u32]);
    let scale_at = e.call(SETTING_FLOAT, &args![SKILL_SCALE_SETTING]).u32();
    let health = x87_add(
        x87_mul(
            x87_load32(e.mem.u32(base + LOCKPICK_SKILL_FLOAT)),
            x87_m32(e.mem.u32(scale_at)),
        ),
        1.0,
    );
    e.mem.set_u32(base + PICK_HEALTH, x87_store32(health));
    e.mem.set_u32(base + LOCK_ANGLE, 0.0f32.to_bits());
    lock_pick_menu_rotate_lock(e, Ptr::new(base), 0.0f32.to_bits());
    let tile = tile_at(e, base, 14);
    let width = tile_value(e, tile, 0xfb1);
    let scale: f64 = e.global(SPOT_SCALE);
    let middle = x87_store32(x87_div(width, scale));
    let x_trait: u32 = e.global(USER_TRAIT_X);
    let tile = tile_at(e, base, 15);
    e.call(
        TILE_SET_FLOAT,
        &args![tile, x_trait, x87_quiet_bits(middle), 1u32],
    );
    let child = lock_model_child(e, 1);
    e.call(NODE_SET_TRANSLATE, &args![child, base + PICK_TRANSLATE]);
    let child = lock_model_child(e, 1);
    e.call(NODE_SET_ROTATE, &args![child, base + PICK_ROTATE]);
    let manager = e.mem.u32(base + PICK_ANIM_MANAGER);
    e.call(DEACTIVATE_ALL, &args![manager, 0.0f32.to_bits()]);
    let manager = e.mem.u32(base + PICK_ANIM_MANAGER);
    let intro = e.mem.u32(base + PICK_INTRO_ANIM);
    activate_sequence(e, manager, intro);
    if e.global::<u8>(PICK_BROKE) != 0 {
        lock_pick_menu_create_sweet_spot(e, Ptr::new(base));
        e.set_global(PICK_BROKE, 0u8);
    }
    let zero: f64 = e.global(ZERO);
    if e.mem.u32(base + LOCK_TARGET) != 0 && picks_left(e, base) == zero {
        show_no_picks(e);
        let tile = tile_at(e, base, 11);
        e.call(TILE_SET_VALUE, &args![tile, 0xfa3u32, 0u32]);
        e.with_stack(0xc, |e, data| {
            update_data(e, data.addr(), 0.0f32.to_bits(), 1, 0);
            let child = lock_model_child(e, 1);
            e.call(UPDATE_DOWNWARD, &args![child, data]);
        });
        e.mem.set_u32(base + LOCK_STATE, 2);
    } else {
        e.mem.set_u32(base + LOCK_STATE, 1);
    }
}

/// State 7: closing.
fn idle_closing(e: &mut Engine, idle: &Idle) {
    let base = idle.base;
    if e.call(D3D_FADE_STATE, &args![base]).u32() == 2 {
        return;
    }
    set_root_alpha(e);
    let interface = e.call(INTERFACE_OBJECT, &args![]).u32();
    e.call(INTERFACE_UPDATE_718930, &args![interface, 1u32]);
    let tile = e.call(LIST_NODE_NEXT, &args![base]).u32();
    e.call(TILE_SET_VALUE, &args![tile, 0x1772u32, 1u32]);
    e.call(MENU_START_FADE_OUT, &args![base]);
}

// Translated from 00790290 (decompiled, FalloutNV.exe 1.4.0.525)
/// The lock pick tutorial's message id (unnamed in the engine map), cdecl:
/// 0x1c when `00709d40` answers true (the gamepad hint), else 0x14.
pub fn fn_00790290(e: &mut Engine) -> u32 {
    if e.call(GAMEPAD_HINT_709D40, &args![]).bool() {
        0x1c
    } else {
        0x14
    }
}

// Translated from 007902b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockPickMenu::DoEnter` (Xbox PDB): for id 13, sets the interface
/// root's trait 0xfa9 to 0.0 (flag 1); other ids do nothing.
pub fn lock_pick_menu_do_enter(e: &mut Engine, _this: Ptr, id: u32, _tile: u32) {
    if id != 0xd {
        return;
    }
    let interface = e.call(INTERFACE_OBJECT, &args![]).u32();
    let root = e.call(INTERFACE_ROOT_TILE, &args![interface]).u32();
    e.call(
        TILE_SET_FLOAT,
        &args![root, 0xfa9u32, 0.0f32.to_bits(), 1u32],
    );
}

// Translated from 007908f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007908f0 - the C++ exception-unwinding frame (FS:[0]) of the sound handles is not translated
/// `LockPickMenu::RotateLockpick` (Xbox PDB): takes the turn from the
/// gamepad (its axis 7 outside the dead zone, through the turn formula and
/// the `double` at `01072310`) or the mouse (axis 1 times the screen width
/// over `00706e50`), adds it to the pick's position (trait `iUT_X` of tile
/// +0x68), clamps it to `[0, width]` (the width is trait 0xfb1 of tile
/// +0x64), plays the turning sound when the pick crossed a whole hundred
/// (or started moving, 1 in 10 at random for a crossing), keeps the
/// position and the turn, moves the pick model, and sets the lock's
/// maximum angle from the distance to the sweet spot: inside it from the
/// closest of the spot's child tiles (their colours copied to the pick
/// tile), outside it from the distance alone.
pub fn lock_pick_menu_rotate_lockpick(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    // The turn starts at 0.0 (`FLDZ`/`FSTP`); both branches overwrite it.
    let scale = quiet_global(e, CONTROLLER_SCALE);
    let turn = if e.call(USING_GAMEPAD, &args![]).bool() {
        let device = controls_device(e);
        let mut axis = e.call(CONTROL_AXIS, &args![device, 0u32, 7u32]).i32();
        let magnitude = e
            .call(FLOAT_ABS, &args![x87_store32(f64::from(axis))])
            .f64();
        if magnitude <= e.global::<f64>(DEAD_ZONE_718F0) {
            axis = 0;
        }
        let ratio = x87_store32(x87_div(f64::from(axis), x87_m32(scale)));
        let other = quiet_global(e, TURN_FORMULA_ARG);
        let value = e
            .call(TURN_FORMULA, &args![x87_quiet_bits(ratio), other])
            .f64();
        let factor: f64 = e.global(TURN_SCALE);
        x87_store32(x87_mul(value, factor))
    } else {
        let device = controls_device(e);
        let axis = e.call(CONTROL_MOUSE_AXIS, &args![device, 1u32]).i32();
        let axis = f64::from(axis);
        let width = e.call(INTERFACE_SCREEN_WIDTH, &args![]).f64();
        let mouse = e.call(INTERFACE_MOUSE_SCALE, &args![]).f64();
        x87_store32(x87_mul(x87_div(width, mouse), axis))
    };
    let x_trait: u32 = e.global(USER_TRAIT_X);
    let pick_tile = tile_at(e, base, 15);
    let current = tile_value(e, pick_tile, x_trait);
    let mut position = x87_store32(x87_add(current, x87_m32(turn)));
    let x_trait: u32 = e.global(USER_TRAIT_X);
    let pick_tile = tile_at(e, base, 15);
    let current = tile_value(e, pick_tile, x_trait);
    let before = ftol(e, current) % 100;
    let after = ftol(e, x87_load32(position)) % 100;
    position = x87_store32(
        e.call(
            FLOAT_MAX,
            &args![x87_quiet_bits(position), 0.0f32.to_bits()],
        )
        .f64(),
    );
    let width_tile = tile_at(e, base, 14);
    let width = x87_store32(tile_value(e, width_tile, 0xfb1));
    position = x87_store32(
        e.call(FLOAT_MIN, &args![x87_quiet_bits(position), width])
            .f64(),
    );
    let zero: f64 = e.global(ZERO);
    let turn_value = x87_load32(turn);
    let crossed = (turn_value > zero && before > after) || (turn_value < zero && after > before);
    let mut played = false;
    if crossed {
        let generator = e.call(RANDOM_GENERATOR, &args![]).u32();
        if e.call(RANDOM_UNSIGNED, &args![generator, 0xau32]).u32() == 0 {
            play_sound(e, TURN_SOUND, 0x101);
            played = true;
        }
    }
    if !played && turn_value != zero && x87_load32(e.global::<u32>(LAST_TURN)) == zero {
        play_sound(e, TURN_SOUND, 0x101);
    }
    let x_trait: u32 = e.global(USER_TRAIT_X);
    let pick_tile = tile_at(e, base, 15);
    e.call(
        TILE_SET_FLOAT,
        &args![pick_tile, x_trait, x87_quiet_bits(position), 1u32],
    );
    e.set_global(LAST_TURN, x87_quiet_bits(turn));
    lock_pick_menu_rotate_lockpick_model(e, this, x87_quiet_bits(position));
    let offset = x87_store32(x87_sub(
        x87_load32(position),
        x87_m32(e.mem.u32(base + SWEET_SPOT)),
    ));
    let distance = x87_store32(e.call(FLOAT_ABS, &args![x87_quiet_bits(offset)]).f64());
    let radius = f64::from(e.mem.i32(base + SWEET_SPOT_RADIUS));
    if x87_load32(distance) < radius {
        inside_sweet_spot(e, base, distance);
    } else {
        outside_sweet_spot(e, base, distance);
    }
}

/// Copies traits 0xfb2 to 0xfb4 of `source` to the pick tile (+0x68).
fn copy_colour(e: &mut Engine, base: u32, source: u32) {
    for trait_id in [0xfb2u32, 0xfb3, 0xfb4] {
        let value = x87_store32(tile_value(e, source, trait_id));
        let tile = tile_at(e, base, 15);
        e.call(TILE_SET_FLOAT, &args![tile, trait_id, value, 1u32]);
    }
}

/// The sweet spot part of [`lock_pick_menu_rotate_lockpick`].
fn inside_sweet_spot(e: &mut Engine, base: u32, distance: u32) {
    e.with_stack(4, |e, iterator| {
        let spot = tile_at(e, base, 14);
        let first = e.call(POINTER_GET, &args![spot + 4]).u32();
        e.mem.set_u32(iterator.addr(), first);
        let spot = tile_at(e, base, 14);
        let mut best = x87_store32(tile_value(e, spot, 0xfb1));
        let mut best_tile = 0u32;
        while e.mem.u32(iterator.addr()) != 0 {
            let spot = tile_at(e, base, 14);
            let at = e.call(TILE_LIST_NEXT, &args![spot + 4, iterator]).u32();
            let child = e.mem.u32(at);
            let scale: f64 = e.global(SPOT_SCALE);
            let half = x87_store32(x87_div(tile_value(e, child, 0xfb1), scale));
            if x87_load32(best) > x87_load32(half) && x87_load32(half) > x87_load32(distance) {
                best = x87_quiet_bits(half);
                best_tile = child;
            }
        }
        if best_tile == 0 {
            return;
        }
        let colour = tile_value(e, best_tile, 0xfaa);
        let offset: f64 = e.global(FAA_OFFSET);
        let scale: f64 = e.global(FAA_SCALE);
        let open: f64 = e.global(ANGLE_OFFSET_66808);
        let share = x87_div(x87_sub(colour, offset), scale);
        let limit = x87_mul(x87_sub(1.0, share), open);
        e.mem.set_u32(base + MAX_LOCK_ANGLE, x87_store32(limit));
        copy_colour(e, base, best_tile);
    });
}

/// The outside part of [`lock_pick_menu_rotate_lockpick`].
fn outside_sweet_spot(e: &mut Engine, base: u32, distance: u32) {
    let radius = f64::from(e.mem.i32(base + SWEET_SPOT_RADIUS));
    let reach = x87_add(radius, x87_m32(distance));
    let spot = tile_at(e, base, 14);
    let width = tile_value(e, spot, 0xfb1);
    let over = x87_sub(reach, width);
    let scale: f64 = e.global(SPOT_SCALE);
    let span = x87_mul(f64::from(e.mem.i32(base + SWEET_SPOT_RADIUS)), scale);
    let spot = tile_at(e, base, 14);
    let width = tile_value(e, spot, 0xfb1);
    let share = x87_store32(x87_div(over, x87_sub(span, width)));
    let factor: f64 = e.global(OUTSIDE_SCALE);
    let limit = x87_mul(x87_load32(share), factor);
    e.mem.set_u32(base + MAX_LOCK_ANGLE, x87_store32(limit));
    let spot = tile_at(e, base, 14);
    copy_colour(e, base, spot);
}

// Translated from 00790df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockPickMenu::RotateLockpickModel` (Xbox PDB), thiscall `(position)`:
/// turns the pick model (lock model child 1) about the Y axis by the
/// position as an angle (`position / width * <0106b160> + <01066808>`, in
/// degrees times the `double` at `01023128`) around the point `(0, 0,
/// <01018180>)` relative to its original translation, applies the
/// original rotation after it and updates the pick.
pub fn lock_pick_menu_rotate_lockpick_model(e: &mut Engine, this: Ptr, position: u32) {
    let base = this.addr();
    e.with_stack(0x24, |e, rotation| {
        let rotation = rotation.addr();
        e.call(NODE_ITEM, &args![rotation]);
        let pick = lock_model_child(e, 1);
        e.with_stack(0xc, |e, data| {
            update_data(e, data.addr(), 0.0f32.to_bits(), 0, 0);
            e.with_stack(0x60, |e, locals| {
                let locals = locals.addr();
                let (centre, negated, offset, turned, moved, sum) = (
                    locals,
                    locals + 0xc,
                    locals + 0x18,
                    locals + 0x24,
                    locals + 0x30,
                    locals + 0x3c,
                );
                let depth = quiet_global(e, PICK_START_Z);
                let centre = e
                    .call(POINT_CONSTRUCT, &args![centre, 0u32, 0u32, depth])
                    .u32();
                let first = e
                    .call(POINT_OP_4A0BD0, &args![base + PICK_TRANSLATE, negated])
                    .u32();
                e.call(POINT_OP_439E90, &args![first, offset, centre]);
                let spot = tile_at(e, base, 14);
                let width = tile_value(e, spot, 0xfb1);
                let scale: f64 = e.global(ANGLE_SCALE_6B160);
                let shift: f64 = e.global(ANGLE_OFFSET_66808);
                let radians: f64 = e.global(DEGREES_TO_RADIANS);
                let angle = x87_mul(
                    x87_add(x87_mul(x87_div(x87_m32(position), width), scale), shift),
                    radians,
                );
                let angle = x87_store32(angle);
                e.call(
                    MATRIX_MAKE_Y_ROTATION,
                    &args![rotation, x87_quiet_bits(angle)],
                );
                let rotated = e
                    .call(MATRIX_TIMES_POINT, &args![rotation, turned, offset])
                    .u32();
                let difference = e
                    .call(
                        POINT_OP_439E90,
                        &args![base + PICK_TRANSLATE, moved, offset],
                    )
                    .u32();
                let translate = e
                    .call(POINT_OP_439EF0, &args![difference, sum, rotated])
                    .u32();
                e.call(NODE_SET_TRANSLATE, &args![pick, translate]);
            });
            e.with_stack(0x24, |e, product| {
                let combined = e
                    .call(
                        MATRIX_MULTIPLY,
                        &args![rotation, product, base + PICK_ROTATE],
                    )
                    .u32();
                for word in 0..9 {
                    let value = e.mem.u32(combined + 4 * word);
                    e.mem.set_u32(rotation + 4 * word, value);
                }
            });
            e.call(NODE_SET_ROTATE, &args![pick, rotation]);
            e.call(UPDATE_DOWNWARD, &args![pick, data]);
        });
    });
}

// Translated from 00790f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockPickMenu::RotateLock` (Xbox PDB), thiscall `(extra)`: sets the
/// lock's rotate animation to the phase `(angle + extra) / <01066808>` of
/// its key range and updates the lock (lock model child 0).
pub fn lock_pick_menu_rotate_lock(e: &mut Engine, this: Ptr, extra: u32) {
    let base = this.addr();
    let open: f64 = e.global(ANGLE_OFFSET_66808);
    let share = x87_store32(x87_div(
        x87_add(x87_load32(e.mem.u32(base + LOCK_ANGLE)), x87_m32(extra)),
        open,
    ));
    e.with_stack(0xc, |e, data| {
        update_data(e, data.addr(), 0.0f32.to_bits(), 1, 0);
        let rotate = e.mem.u32(base + LOCK_ROTATE_ANIM);
        if rotate == 0 {
            return;
        }
        let begin = e.call(SEQUENCE_BEGIN_KEY, &args![rotate]).f64();
        let rotate = e.mem.u32(base + LOCK_ROTATE_ANIM);
        let end = e.call(SEQUENCE_END_KEY, &args![rotate]).f64();
        let rotate = e.mem.u32(base + LOCK_ROTATE_ANIM);
        let begin_again = e.call(SEQUENCE_BEGIN_KEY, &args![rotate]).f64();
        let phase = x87_store32(x87_add(
            x87_mul(x87_sub(end, begin_again), x87_m32(share)),
            begin,
        ));
        let rotate = e.mem.u32(base + LOCK_ROTATE_ANIM);
        e.call(
            SEQUENCE_SET_PHASE,
            &args![rotate, x87_quiet_bits(phase), 0u32],
        );
        let child = lock_model_child(e, 0);
        e.call(UPDATE_DOWNWARD, &args![child, data]);
    });
}

// Translated from 00791000 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00791000 - the C++ exception-unwinding frame (FS:[0]) that destroys the string when a call throws is not translated
/// `LockPickMenu::CreateSweetSpot` (Xbox PDB): from how far the skill is
/// into the level's range (clamped to `[0, 1]`), computes the two spot
/// widths from the level's settings (`011a0328`/`011a033c` and
/// `011a0350`/`011a0364`), shows them, sets the spot radius (half the first
/// plus the second), places the spot at random inside the width, and builds
/// the six rings (the template at `01074648` under tile +0x64): their ids
/// (0x11 on), widths, starts, depths (6 down) and colours (from green to red
/// in six steps).
pub fn lock_pick_menu_create_sweet_spot(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let level = e.mem.u32(base + LOCK_LEVEL);
    let span = e
        .mem
        .i32(SKILL_FOR_LEVEL + 4 + 4 * level)
        .wrapping_sub(e.mem.i32(SKILL_FOR_LEVEL + 4 * level));
    let into = e.mem.i32(base + LOCKPICK_SKILL).wrapping_sub(
        e.mem
            .i32(SKILL_FOR_LEVEL + 4 * e.mem.u32(base + LOCK_LEVEL)),
    );
    let doubled = x87_add(f64::from(span), f64::from(span));
    let ratio = x87_store32(x87_div(f64::from(into), doubled));
    let low = e
        .call(FLOAT_MIN, &args![x87_quiet_bits(ratio), 1.0f32.to_bits()])
        .f64();
    let share = x87_store32(
        e.call(FLOAT_MAX, &args![x87_store32(low), 0.0f32.to_bits()])
            .f64(),
    );
    e.with_stack(0x40, |e, colours| {
        let colours = colours.addr();
        let (start, end, colour) = (colours, colours + 0x10, colours + 0x20);
        e.call(
            RECT_CONSTRUCT,
            &args![
                start,
                0.0f32.to_bits(),
                1.0f32.to_bits(),
                0.0f32.to_bits(),
                1.0f32.to_bits()
            ],
        );
        e.call(
            RECT_CONSTRUCT,
            &args![
                end,
                1.0f32.to_bits(),
                0.0f32.to_bits(),
                0.0f32.to_bits(),
                1.0f32.to_bits()
            ],
        );
        for word in 0..4 {
            let value = e.mem.u32(start + 4 * word);
            e.mem.set_u32(colour + 4 * word, value);
        }
        let spot = tile_at(e, base, 14);
        e.call(TILE_DELETE_CHILDREN, &args![spot]);
        e.with_stack(8, |e, text| {
            e.call(STRING_CONSTRUCT, &args![text]);
            for (low_table, high_table, target) in [
                (SPOT_TABLE_A_LOW, SPOT_TABLE_A_HIGH, SWEET_SPOT_WIDTH_A),
                (SPOT_TABLE_B_LOW, SPOT_TABLE_B_HIGH, SWEET_SPOT_WIDTH_B),
            ] {
                let level = e.mem.u32(base + LOCK_LEVEL);
                let low_setting = e.mem.u32(low_table + 4 * level);
                let low_at = e.call(SETTING_INT, &args![low_setting]).u32();
                let low_value = f64::from(e.mem.i32(low_at));
                let level = e.mem.u32(base + LOCK_LEVEL);
                let high_setting = e.mem.u32(high_table + 4 * level);
                let high_at = e.call(SETTING_INT, &args![high_setting]).u32();
                let level = e.mem.u32(base + LOCK_LEVEL);
                let low_setting = e.mem.u32(low_table + 4 * level);
                let low_at = e.call(SETTING_INT, &args![low_setting]).u32();
                let range = e.mem.i32(high_at).wrapping_sub(e.mem.i32(low_at));
                let width = x87_add(x87_mul(f64::from(range), x87_m32(share)), low_value);
                let width = ftol(e, width);
                e.set_global(target, width);
            }
            for (width, tile_index) in [(SWEET_SPOT_WIDTH_A, 4u32), (SWEET_SPOT_WIDTH_B, 7)] {
                let value: u32 = e.global(width);
                e.call(STRING_FORMAT, &args![text, INTEGER_FORMAT, value]);
                let shown = e.call(POINTER_GET, &args![text]).u32();
                let tile = tile_at(e, base, tile_index);
                e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, shown, 1u32]);
            }
            let radius = (e.global::<i32>(SWEET_SPOT_WIDTH_A) / 2)
                .wrapping_add(e.global::<i32>(SWEET_SPOT_WIDTH_B));
            e.mem.set_i32(base + SWEET_SPOT_RADIUS, radius);
            let spot = tile_at(e, base, 14);
            let width = tile_value(e, spot, 0xfb1);
            let generator = e.call(RANDOM_GENERATOR, &args![]).u32();
            let random = e.call(RANDOM_FLOAT, &args![generator]).f64();
            e.mem
                .set_u32(base + SWEET_SPOT, x87_store32(x87_mul(random, width)));
            let spot = tile_at(e, base, 14);
            let width = tile_value(e, spot, 0xfb1);
            let room = x87_store32(x87_sub(
                width,
                f64::from(e.mem.i32(base + SWEET_SPOT_RADIUS)),
            ));
            let radius_value = x87_store32(f64::from(e.mem.i32(base + SWEET_SPOT_RADIUS)));
            let place = quiet_field(e, base + SWEET_SPOT);
            let low = e.call(FLOAT_MAX, &args![place, radius_value]).f64();
            let place = e
                .call(FLOAT_MIN, &args![x87_store32(low), x87_quiet_bits(room)])
                .f64();
            e.mem.set_u32(base + SWEET_SPOT, x87_store32(place));
            let half = e.global::<i32>(SWEET_SPOT_WIDTH_A) / 2;
            let first = x87_sub(x87_m32(e.mem.u32(base + SWEET_SPOT)), f64::from(half));
            let mut start_value = ftol(e, first);
            let mut depth = 6i32;
            e.with_stack(0x30, |e, steps| {
                let steps = steps.addr();
                let (difference, scaled, sum) = (steps, steps + 0x10, steps + 0x20);
                for ring in 0..6u32 {
                    let menu_tile = e.call(LIST_NODE_NEXT, &args![base]).u32();
                    let menu = e.call(TILE_MENU_3C, &args![menu_tile]).u32();
                    let spot = tile_at(e, base, 14);
                    let made = e
                        .call(
                            MENU_RENDER_TEMPLATE,
                            &args![menu, spot, SWEET_SPOT_TEMPLATE, 0u32],
                        )
                        .u32();
                    e.call(TILE_SET_VALUE, &args![made, 0xfaau32, ring + 0x11]);
                    let offset = x87_sub(
                        x87_m32(e.mem.u32(base + SWEET_SPOT)),
                        f64::from(start_value),
                    );
                    let width = x87_store32(x87_add(offset, offset));
                    e.call(
                        TILE_SET_FLOAT,
                        &args![made, 0xfb1u32, x87_quiet_bits(width), 1u32],
                    );
                    e.call(TILE_SET_VALUE, &args![made, 0xfa1u32, start_value as u32]);
                    e.call(TILE_SET_VALUE, &args![made, 0xfadu32, depth as u32]);
                    let factor: f64 = e.global(SPOT_COLOR_SCALE);
                    for (word, trait_id) in [0xfb2u32, 0xfb3, 0xfb4].into_iter().enumerate() {
                        let value = x87_store32(x87_mul(
                            x87_load32(e.mem.u32(colour + 4 * word as u32)),
                            factor,
                        ));
                        e.call(
                            TILE_SET_FLOAT,
                            &args![made, trait_id, x87_quiet_bits(value), 1u32],
                        );
                    }
                    start_value = start_value.wrapping_sub(e.global::<i32>(SWEET_SPOT_WIDTH_B) / 5);
                    depth -= 1;
                    let steps_value: f64 = e.global(SPOT_STEPS);
                    let fraction = x87_store32(x87_div(f64::from(ring as i32 + 1), steps_value));
                    let first = e.call(COLOR_SUBTRACT, &args![end, difference, start]).u32();
                    let second = e
                        .call(COLOR_SCALE, &args![first, scaled, x87_quiet_bits(fraction)])
                        .u32();
                    let third = e.call(COLOR_ADD, &args![start, sum, second]).u32();
                    for word in 0..4 {
                        let value = e.mem.u32(third + 4 * word);
                        e.mem.set_u32(colour + 4 * word, value);
                    }
                }
            });
            e.call(STRING_DESTROY, &args![text]);
        });
    });
}

// Translated from 00791540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, thiscall `(state)` (`this` is not read):
/// whether any of the controls 2, 3, 0 and 1 (asked in that order, until
/// one answers) is in the given state (`00a24660`).
pub fn fn_00791540(e: &mut Engine, _this: Ptr, state: u32) -> bool {
    for control in [2u32, 3, 0, 1] {
        let device = controls_device(e);
        if e.call(CONTROL_PRESSED, &args![device, control, state])
            .u32()
            != 0
        {
            return true;
        }
    }
    false
}

/// `BGSEntryPoint::HandleEntryPoint` (`cdecl`: entry point, actor, the
/// value's address).
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// thiscall (setting, value): an int game setting's value set (`0045ce80`).
const SETTING_SET_INT: u32 = 0x0045_ce80;
const TILE_GET_CHILD_BY_ID: u32 = 0x00a0_3eb0;
/// `90.0` (`float`), the lock's angle when it is forced open.
const FORCE_LOCK_ANGLE: u32 = 0x0102_49d8;
/// `"LOCK BROKEN!  :("`.
const LOCK_BROKEN_TEXT: u32 = 0x0107_4618;
/// `"UILockpickingForceFail"` and the vault boy icon
/// (`"Interface\Icons\Message Icons\glow_message_vaultboy_surprised.dds"`).
const FORCE_FAIL_SOUND: u32 = 0x0107_4600;
const FORCE_FAIL_ICON: u32 = 0x0104_9638;
/// The game setting (string) of the force-fail message.
const FORCE_FAIL_SETTING: u32 = 0x011d_3e98;
/// The step `DoClick` widens or narrows the sweet spot by.
const SWEET_SPOT_STEP: i32 = 5;

// Translated from 007902f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockPickMenu::DoLeave` (engine map): for id 13, sets the interface
/// root's trait `0xfa9` to the `float` at `01023cd8` (255.0, flag 1); other
/// ids do nothing.
pub fn lock_pick_menu_do_leave(e: &mut Engine, _this: Ptr, id: u32, _tile: u32) {
    if id != 0xd {
        return;
    }
    let alpha = quiet_field(e, ENTER_ALPHA);
    let interface = e.call(INTERFACE_OBJECT, &args![]).u32();
    let root = e.call(INTERFACE_ROOT_TILE, &args![interface]).u32();
    e.call(TILE_SET_FLOAT, &args![root, 0xfa9u32, alpha, 1u32]);
}

/// [`lock_pick_menu_do_click`]'s fraction of the lock level's skill range
/// the player has: `(skill - needed) / (2 * range)`, kept within 0..1
/// (`0040ebd0` with 1.0, then `00404010` with 0.0), stored as a `float`.
fn skill_fraction(e: &mut Engine, base: u32) -> u32 {
    let level = e.mem.u32(base + LOCK_LEVEL);
    let needed_at = SKILL_FOR_LEVEL.wrapping_add(level.wrapping_mul(4));
    // The next level's word follows.
    let range = e
        .mem
        .i32(needed_at.wrapping_add(4))
        .wrapping_sub(e.mem.i32(needed_at));
    let have = e
        .mem
        .i32(base + LOCKPICK_SKILL)
        .wrapping_sub(e.mem.i32(needed_at));
    let range = f64::from(range);
    let doubled = x87_add(range, range);
    let quotient = x87_store32(x87_div(f64::from(have), doubled));
    let low = e
        .call(
            FLOAT_MIN,
            &args![x87_quiet_bits(quotient), 1.0f32.to_bits()],
        )
        .f64();
    let low = x87_store32(low);
    let fraction = e.call(FLOAT_MAX, &args![low, 0.0f32.to_bits()]).f64();
    x87_store32(fraction)
}

/// Adds the click's share to the two sweet-spot settings of the lock
/// level (`low` table: `step * (1 - fraction)`, `high` table: `step *
/// fraction`, each truncated and added to the setting's value).
fn widen_settings(e: &mut Engine, base: u32, low: u32, high: u32, step: i32, fraction: u32) {
    let level = e.mem.u32(base + LOCK_LEVEL);
    let setting = e.mem.u32(low.wrapping_add(level.wrapping_mul(4)));
    let value = e.call(SETTING_INT, &args![setting]).u32();
    let rest = x87_sub(1.0, x87_load32(fraction));
    let share = ftol(e, x87_mul(f64::from(step), rest));
    let total = share.wrapping_add(e.mem.i32(value));
    let level = e.mem.u32(base + LOCK_LEVEL);
    let setting = e.mem.u32(low.wrapping_add(level.wrapping_mul(4)));
    e.call(SETTING_SET_INT, &args![setting, total]);
    let level = e.mem.u32(base + LOCK_LEVEL);
    let setting = e.mem.u32(high.wrapping_add(level.wrapping_mul(4)));
    let value = e.call(SETTING_INT, &args![setting]).u32();
    let share = ftol(e, x87_mul(f64::from(step), x87_m32(fraction)));
    let total = share.wrapping_add(e.mem.i32(value));
    let level = e.mem.u32(base + LOCK_LEVEL);
    let setting = e.mem.u32(high.wrapping_add(level.wrapping_mul(4)));
    e.call(SETTING_SET_INT, &args![setting, total]);
}

/// Shows a sweet-spot width (`"%d"` into the stack `BSString` `text`) as
/// text `0xfc4` of the tile at `tile_field`.
fn show_width(e: &mut Engine, text: Ptr, tile_field: u32, width: u32) {
    e.call(STRING_FORMAT, &args![text, INTEGER_FORMAT, width]);
    let shown = e.call(POINTER_GET, &args![text]).u32();
    let tile = e.mem.u32(tile_field);
    e.call(TILE_SET_STRING, &args![tile, 0xfc4u32, shown, 1u32]);
}

// Translated from 00790330 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00790330 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the two stack strings) is not translated.
/// `LockPickMenu::DoClick` (engine map), the sweet-spot tuning and force
/// buttons. 3 and 5 (wider, narrower by 5): the width `011da208` and half
/// the step on `iTotalSweetSpotRadius`, the width shown on tile 4, the
/// level's first two width settings (`011a0328`, `011a033c`) shared out by
/// the skill fraction, and the six spot children (ids 0x11 to 0x16 of tile
/// 14) widened (`0xfb1`) and moved back by half the step (`0xfa1`). 6 and 8:
/// the same on `011da20c`, the whole step on the radius, tile 7 and the
/// other two settings (`011a0350`, `011a0364`); children 1 to 5 by
/// `i * 2 * step / 5` and `i * step / 5`. 9 (force): entry point 0x20 on a
/// stack float; a roll of 100 below the force chance (the player's clamped
/// lockpick skill against the level's) opens the lock (`bForceLockSuccess`,
/// `fMaxLockAngle` 90.0, state 3); else without a target and a zero entry
/// value the text `"LOCK BROKEN!  :("` is shown (tile 11) and the state is
/// 6; else the target's lock gets one more try (`+0xC`, `AddLockChange`),
/// the force-fail message is shown and the menu closes (`0078e0d0`). 10
/// closes the menu.
pub fn lock_pick_menu_do_click(e: &mut Engine, this: Ptr, id: u32, _tile: u32) {
    let base = this.addr();
    match id {
        3 | 5 => {
            let fraction = skill_fraction(e, base);
            let sign = if id != 5 { 1 } else { -1 };
            let step = sign * SWEET_SPOT_STEP;
            e.with_stack(8, |e, text| {
                e.call(STRING_CONSTRUCT, &args![text]);
                let width = e.mem.i32(SWEET_SPOT_WIDTH_A).wrapping_add(step);
                e.mem.set_u32(SWEET_SPOT_WIDTH_A, width as u32);
                let radius = (step / 2).wrapping_add(e.mem.i32(base + SWEET_SPOT_RADIUS));
                e.mem.set_u32(base + SWEET_SPOT_RADIUS, radius as u32);
                let width = e.mem.u32(SWEET_SPOT_WIDTH_A);
                show_width(e, text, base + TILES + 4 * 4, width);
                widen_settings(e, base, SPOT_TABLE_A_LOW, SPOT_TABLE_A_HIGH, step, fraction);
                for index in 0..6u32 {
                    let parent = e.mem.u32(base + TILES + 4 * 14);
                    let child = e
                        .call(TILE_GET_CHILD_BY_ID, &args![parent, index + 0x11])
                        .u32();
                    let value = tile_value(e, child, 0xfb1);
                    let wider = x87_store32(x87_add(value, f64::from(step)));
                    e.call(TILE_SET_FLOAT, &args![child, 0xfb1u32, wider, 1u32]);
                    let value = tile_value(e, child, 0xfa1);
                    let moved = x87_store32(x87_sub(value, f64::from(step / 2)));
                    e.call(TILE_SET_FLOAT, &args![child, 0xfa1u32, moved, 1u32]);
                }
                e.call(STRING_DESTROY, &args![text]);
            });
        }
        6 | 8 => {
            let fraction = skill_fraction(e, base);
            let sign = if id != 8 { 1 } else { -1 };
            let step = sign * SWEET_SPOT_STEP;
            e.with_stack(8, |e, text| {
                e.call(STRING_CONSTRUCT, &args![text]);
                let width = e.mem.i32(SWEET_SPOT_WIDTH_B).wrapping_add(step);
                e.mem.set_u32(SWEET_SPOT_WIDTH_B, width as u32);
                let radius = e.mem.i32(base + SWEET_SPOT_RADIUS).wrapping_add(step);
                e.mem.set_u32(base + SWEET_SPOT_RADIUS, radius as u32);
                let width = e.mem.u32(SWEET_SPOT_WIDTH_B);
                show_width(e, text, base + TILES + 4 * 7, width);
                widen_settings(e, base, SPOT_TABLE_B_LOW, SPOT_TABLE_B_HIGH, step, fraction);
                let two: f64 = e.global(SPOT_SCALE);
                let five: f64 = e.global(SPOT_STEPS);
                for index in 1..6i32 {
                    let parent = e.mem.u32(base + TILES + 4 * 14);
                    let child = e
                        .call(TILE_GET_CHILD_BY_ID, &args![parent, (index + 0x11) as u32])
                        .u32();
                    let value = tile_value(e, child, 0xfb1);
                    let scaled = x87_mul(x87_mul(f64::from(index), two), f64::from(step));
                    let wider = x87_store32(x87_add(value, x87_div(scaled, five)));
                    e.call(TILE_SET_FLOAT, &args![child, 0xfb1u32, wider, 1u32]);
                    let value = tile_value(e, child, 0xfa1);
                    let shift = x87_div(f64::from(index.wrapping_mul(step)), five);
                    let moved = x87_store32(x87_sub(value, shift));
                    e.call(TILE_SET_FLOAT, &args![child, 0xfa1u32, moved, 1u32]);
                }
                e.call(STRING_DESTROY, &args![text]);
            });
        }
        9 => lock_pick_menu_force(e, base),
        10 => fn_0078e0d0(e),
        _ => {}
    }
}

/// [`lock_pick_menu_do_click`]'s force button (id 9).
fn lock_pick_menu_force(e: &mut Engine, base: u32) {
    e.with_stack(4, |e, chance| {
        e.mem.set_u32(chance.addr(), 0.0f32.to_bits());
        let player = the_player(e);
        e.call(HANDLE_ENTRY_POINT, &args![0x20u32, player, chance]);
        let generator = e.call(RANDOM_GENERATOR, &args![]).u32();
        let roll = e.call(RANDOM_UNSIGNED, &args![generator, 100u32]).u32();
        let level = e.mem.u32(base + LOCK_LEVEL);
        let needed = e
            .mem
            .u32(SKILL_FOR_LEVEL.wrapping_add(level.wrapping_mul(4)));
        let values = the_player(e).wrapping_add(0xa4);
        let skill = e
            .call(
                GET_CLAMPED_ACTOR_VALUE,
                &args![values, LOCKPICK_ACTOR_VALUE],
            )
            .u32();
        let force = e.call(GET_FORCE_LOCK_CHANCE, &args![skill, needed]).u32();
        if roll < force {
            e.mem.set_u8(base + FORCE_LOCK_SUCCESS, 1);
            let angle = quiet_field(e, FORCE_LOCK_ANGLE);
            e.mem.set_u32(base + MAX_LOCK_ANGLE, angle);
            e.mem.set_u32(base + LOCK_STATE, 3);
            return;
        }
        let zero: f64 = e.global(ZERO);
        if e.mem.u32(base + LOCK_TARGET) == 0 && x87_m32(e.mem.u32(chance.addr())) == zero {
            let tile = e.mem.u32(base + TILES + 4 * 11);
            e.call(
                TILE_SET_STRING,
                &args![tile, 0xfc4u32, LOCK_BROKEN_TEXT, 1u32],
            );
            let tile = e.mem.u32(base + TILES + 4 * 11);
            e.call(TILE_SET_VALUE, &args![tile, 0xfa3u32, 1u32]);
            e.mem.set_u32(base + LOCK_STATE, 6);
            return;
        }
        let target = e.mem.u32(base + LOCK_TARGET);
        let lock = e.call(GET_LOCK, &args![target]).u32();
        let tries = e.mem.u32(lock + 0xc).wrapping_add(1);
        e.mem.set_u32(lock + 0xc, tries);
        let target = e.mem.u32(base + LOCK_TARGET);
        e.call(ADD_LOCK_CHANGE, &args![target]);
        let seconds = quiet_field(e, MESSAGE_SECONDS);
        let text = e.call(SETTING_STRING, &args![FORCE_FAIL_SETTING]).u32();
        e.call(
            SHOW_MESSAGE,
            &args![text, 0u32, FORCE_FAIL_ICON, FORCE_FAIL_SOUND, seconds, 0u32],
        );
        fn_0078e0d0(e);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0078d930, lock_pick_menu_lock_pick_menu(Ptr) -> Ptr),
        entry!(0x0078da90, lock_pick_menu_get_class(Ptr) -> u32),
        entry!(0x0078daa0, lock_pick_menu_attach_tile_by_id(Ptr, i32, u32)),
        entry!(
            0x0078dad0,
            lock_pick_menu_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0078db00, lock_pick_menu_create(u32, u32, u32) -> u32),
        entry!(0x0078e080, fn_0078e080() -> u32),
        entry!(0x0078e090, fn_0078e090(Ptr) -> bool),
        entry!(0x0078e0d0, fn_0078e0d0()),
        entry!(0x0078e0f0, fn_0078e0f0(Ptr)),
        entry!(0x0078e1c0, lock_pick_menu_prepare_3d_elements(Ptr)),
        entry!(0x0078e7c0, fn_0078e7c0()),
        entry!(0x0078e900, lock_pick_menu_create_3d_camera(Ptr) -> u32),
        entry!(0x0078eb50, lock_pick_menu_do_idle(Ptr)),
        entry!(0x00790290, fn_00790290() -> u32),
        entry!(0x007902b0, lock_pick_menu_do_enter(Ptr, u32, u32)),
        entry!(0x007908f0, lock_pick_menu_rotate_lockpick(Ptr)),
        entry!(0x00790df0, lock_pick_menu_rotate_lockpick_model(Ptr, u32)),
        entry!(0x00790f40, lock_pick_menu_rotate_lock(Ptr, u32)),
        entry!(0x00791000, lock_pick_menu_create_sweet_spot(Ptr)),
        entry!(0x00791540, fn_00791540(Ptr, u32) -> bool),
        entry!(0x0079_02f0, lock_pick_menu_do_leave(Ptr, u32, u32)),
        entry!(0x0079_0330, lock_pick_menu_do_click(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const DELETE_DOUBLE: u32 = 0xdead_a000;
    const CLASS_DOUBLE: u32 = 0xdead_a010;
    const REWARD_XP_DOUBLE: u32 = 0xdead_a020;
    const REMOVE_ITEM_DOUBLE: u32 = 0xdead_a030;
    const ATTACH_DOUBLE: u32 = 0xdead_a040;
    const DETACH_DOUBLE: u32 = 0xdead_a050;
    const FAR_DOUBLE: u32 = 0xdead_a060;

    /// Every callee of the file answers 0 unless a test says otherwise.
    const CALLEES: [u32; 149] = [
        FREE,
        SETTING_STRING,
        SETTING_FLOAT,
        SETTING_INT,
        STRING_CONSTRUCT,
        STRING_DESTROY,
        STRING_FORMAT,
        PRINT_FORMATTED,
        APPEND_STRING,
        COMPARE_STRINGS,
        FLOAT_ABS,
        FLOAT_MAX,
        FLOAT_MIN,
        FTOL,
        ATEXIT,
        SOUND_HANDLE_CONSTRUCT,
        SOUND_HANDLE_ASSIGN,
        SOUND_HANDLE_DESTROY,
        AUDIO_INSTANCE,
        GET_SOUND_HANDLE_BY_NAME,
        SOUND_PLAY,
        SOUND_FADE_OUT,
        POINTER_CONSTRUCT,
        POINTER_DESTROY,
        TICKS,
        USING_GAMEPAD,
        INTERFACE_ROOT_TILE,
        TILE_READER,
        TILE_READ_FILE,
        TILE_GET_MENU,
        TILE_GET_MENU_BY_CLASS,
        TILE_GET_FLOAT,
        TILE_POKE,
        TILE_SET_FLOAT,
        TILE_SET_STRING,
        TILE_SET_VALUE,
        TILE_GET_MAXIMUM_DEPTH,
        TILE_TEXT_TO_TRAIT,
        TILE_GET_CHILD_BY_NAME,
        TILE_DELETE_CHILDREN,
        ADD_USER_TRAIT,
        MENU_CONSTRUCT,
        MENU_DESTROY,
        MENU_SET_MENU_TILE,
        MENU_PREP_FOR_VISIBILITY,
        MENU_START_FADE_OUT,
        MENU_RENDER_TEMPLATE,
        TILE_MENU_3C,
        TILE_LIST_NEXT,
        PRINT_MESSAGE,
        SHOW_MESSAGE,
        IS_TOP_MENU,
        GET_CLAMPED_ACTOR_VALUE,
        GET_LOCK_LEVEL,
        LOCK_VALUE_TO_LEVEL,
        CLAMP_LOW_647B70,
        CLAMP_HIGH_4A8F20,
        GET_FORCE_LOCK_CHANCE,
        GET_ITEM_COUNT,
        SCENE_ROOT,
        NODE_CLEAN_UP_572160,
        SCENE_ROOT_B5D180,
        LOAD_FILE,
        NODE_SET_TRANSLATE,
        NODE_SET_ROTATE,
        NODE_SET_TRANSLATE_XYZ,
        NI_ALLOCATE,
        NI_NODE_CONSTRUCT,
        NODE_CHILD,
        POINT_CONSTRUCT,
        MATRIX_MAKE_X_ROTATION,
        MATRIX_MAKE_Y_ROTATION,
        MATRIX_ROTATION_4A0C90,
        MATRIX_MULTIPLY,
        MATRIX_TIMES_POINT,
        POINT_OP_4A0BD0,
        POINT_OP_439E90,
        POINT_OP_439EF0,
        UPDATE_DATA_CONSTRUCT,
        UPDATE_DOWNWARD,
        GET_CONTROLLER,
        FIXED_STRING_CONSTRUCT,
        FIXED_STRING_DESTROY,
        GET_SEQUENCE_BY_NAME,
        ACTIVATE_SEQUENCE,
        DEACTIVATE_SEQUENCE,
        DEACTIVATE_ALL,
        SEQUENCE_SET_CYCLE,
        START_ANIMATIONS,
        GET_WORLD_BOUND,
        BOUND_RADIUS,
        ADD_LIGHTS_FROM_NODE,
        SEQUENCE_BEGIN_KEY,
        SEQUENCE_END_KEY,
        SEQUENCE_SET_PHASE,
        POINT_OF_NODE,
        CAMERA_CONSTRUCT,
        RECT_CONSTRUCT,
        CAMERA_SET_VIEWPORT,
        FRUSTUM_CONSTRUCT,
        CAMERA_SET_ASPECT,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        TANGENT,
        CAMERA_SET_FRUSTUM,
        CAMERA_LOOK_AT,
        CAMERA_SET_LOD,
        SCENE_OBJECT,
        RENDERER,
        RENDERER_BEGIN_7148C0,
        CULLING_PROCESS,
        CULLING_SET_C4F270,
        CULLING_RESET_C4F2D0,
        CAMERA_SET_FOV_709D50,
        RENDERED_TEXTURE_7FD640,
        CULLING_SET_TARGET,
        SCENE_CAMERA_6629F0,
        ACCUMULATE_SCENE,
        RENDER_SCENE_B6C0D0,
        CONTROL_AXIS,
        CONTROL_MOUSE_AXIS,
        CONTROL_PRESSED,
        CONTROL_RUMBLE,
        CONTROL_STOP_RUMBLE,
        INTERFACE_SCREEN_WIDTH,
        INTERFACE_MOUSE_SCALE,
        TURN_FORMULA,
        RANDOM_GENERATOR,
        RANDOM_UNSIGNED,
        RANDOM_FLOAT,
        TUTORIAL_SEEN,
        TUTORIAL_SHOW,
        TUTORIAL_FLAG_5D4A40,
        INTERFACE_UPDATE_718930,
        GAMEPAD_HINT_709D40,
        MISC_STAT_INCREMENT,
        GET_LOCK,
        GET_EXPERIENCE,
        ADD_LOCK_CHANGE,
        GET_OWNER,
        FORM_TYPE,
        ACTOR_REF_FOR_BASE,
        ACTOR_REF_IN_HIGH,
        SET_WANT_WEAPON_DRAWN,
        STEAL_ALARM,
        BASE_IS_EVIL_47D7C0,
        ALIGNMENT_FOR_KARMA,
        REWARD_KARMA,
        UNLOCK,
    ];

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn float(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

    /// An engine whose callees answer 0, with the exe's data pages mapped,
    /// the pointer and list helpers doubled as the game's, a player (slots
    /// +0x488 and +0x17c logged) and the remaining file-local callees.
    fn world() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x7_0000);
        e.map(0x0118_0000, 0x3_0000);
        e.map(0x011c_0000, 0x5_0000);
        for callee in CALLEES
            .into_iter()
            .chain([
                SAVE_FORM_LINK,
                CONTAINER_MENU_75B6E0,
                ACTIVATE,
                D3D_FADE_STATE,
            ])
            .chain([POINT_PERCENT_4BD510, COLOR_SUBTRACT, COLOR_SCALE, COLOR_ADD])
            .chain([
                REWARD_XP_DOUBLE,
                REMOVE_ITEM_DOUBLE,
                ATTACH_DOUBLE,
                DETACH_DOUBLE,
            ])
            .chain([DELETE_DOUBLE, FAR_DOUBLE])
        {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(LIST_NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(INTERFACE_OBJECT, |_, _| ret(0x4400));
        e.register(CONTROLS_DEVICE, |_, _| ret(0x3300));
        e.register(FTOL, |_, a| {
            ret(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32)
        });
        e.register(CLASS_DOUBLE, |_, _| ret(LOCK_PICK_MENU_CLASS));
        e.register(GET_SOUND_HANDLE_BY_NAME, |_, a| ret(a[2]));
        e.register(INTERFACE_MOUSE_SCALE, |_, _| float(1.0));
        e.register(MATRIX_MULTIPLY, |_, a| ret(a[1]));
        e.register(POINT_CONSTRUCT, |_, a| ret(a[0]));
        e.register(CAMERA_CONSTRUCT, |_, a| ret(a[0]));
        for colour in [COLOR_SUBTRACT, COLOR_SCALE, COLOR_ADD] {
            e.register(colour, |_, a| ret(a[1]));
        }
        e.register(NI_ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        let zeros = e.mem.alloc(0x40);
        for getter in [SETTING_FLOAT, SETTING_INT, POINT_OF_NODE] {
            e.register_double(getter, move |_, _| ret(zeros));
        }
        let node_vtable = e.mem.alloc(0x100);
        e.mem
            .set_u32(node_vtable + NODE_SLOT_ATTACH_CHILD, ATTACH_DOUBLE);
        e.mem
            .set_u32(node_vtable + NODE_SLOT_DETACH_CHILD, DETACH_DOUBLE);
        e.register_double(NI_NODE_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0], node_vtable);
            ret(a[0])
        });
        let root = e.mem.alloc(0x10);
        e.mem.set_u32(root, node_vtable);
        e.register_double(SCENE_ROOT, move |_, _| ret(root));
        let scene_vtable = e.mem.alloc(0x110);
        e.mem.set_u32(scene_vtable + SCENE_SLOT_FAR, FAR_DOUBLE);
        let scene = e.mem.alloc(8);
        e.mem.set_u32(scene, scene_vtable);
        e.register_double(SCENE_OBJECT, move |_, _| ret(scene));
        let vtable = e.mem.alloc(0x500);
        e.mem
            .set_u32(vtable + PLAYER_SLOT_REWARD_XP, REWARD_XP_DOUBLE);
        e.mem
            .set_u32(vtable + PLAYER_SLOT_REMOVE_ITEM, REMOVE_ITEM_DOUBLE);
        let player = e.mem.alloc(0x300);
        e.mem.set_u32(player, vtable);
        e.set_global(PLAYER, player);
        e
    }

    /// A lock pick menu with every tile attached (tile `i` at 0x9000 + 0x10 i),
    /// a menu tile at +4 and the class slot.
    fn menu(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable, DELETE_DOUBLE);
        e.mem.set_u32(vtable + 0x34, CLASS_DOUBLE);
        let menu = e.mem.alloc(0x100);
        e.mem.set_u32(menu, vtable);
        e.mem.set_u32(menu + 4, 0x7777);
        for index in 0..0x10 {
            e.mem
                .set_u32(menu + TILES + 4 * index, 0x9000 + 0x10 * index);
        }
        menu
    }

    /// A node with the attach and detach slots doubled.
    fn node(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem
            .set_u32(vtable + NODE_SLOT_ATTACH_CHILD, ATTACH_DOUBLE);
        e.mem
            .set_u32(vtable + NODE_SLOT_DETACH_CHILD, DETACH_DOUBLE);
        let node = e.mem.alloc(0x10);
        e.mem.set_u32(node, vtable);
        node
    }

    #[test]
    fn constructor_takes_the_skill_and_the_pick_health() {
        let mut e = world();
        e.register(GET_CLAMPED_ACTOR_VALUE, |_, _| ret(40));
        let scale = e.mem.alloc(4);
        e.mem.set_f32(scale, 0.5);
        e.register_double(SETTING_FLOAT, move |_, _| ret(scale));
        e.register(ADD_USER_TRAIT, |_, a| ret(a[0] & 0xfff));
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + LOCK_STATE, 3);
        e.mem.set_u32(this + PICK_ANIM_MANAGER, 0x44);
        e.mem.set_u8(this + SHOWING_TUTORIAL, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_d930, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), LOCK_PICK_MENU_VTABLE);
        assert_eq!(
            calls(&e, NODE_ITEM),
            vec![vec![this + 0xb4], vec![this + 0xd8]]
        );
        assert_eq!(e.global::<u32>(USER_TRAIT_VALUE), 0xd18);
        assert_eq!(e.global::<u32>(USER_TRAIT_X), 0xc90);
        let player: u32 = e.global(PLAYER);
        assert_eq!(
            calls(&e, GET_CLAMPED_ACTOR_VALUE),
            vec![vec![player + 0xa4, 0x24]]
        );
        assert_eq!(e.mem.u32(this + LOCKPICK_SKILL), 40);
        assert_eq!(e.mem.f32(this + LOCKPICK_SKILL_FLOAT), 40.0);
        assert_eq!(e.mem.f32(this + PICK_HEALTH), 21.0);
        assert_eq!(e.mem.u32(this + LOCK_STATE), 0);
        assert_eq!(e.mem.u8(this + SHOWING_TUTORIAL), 0);
        assert_eq!(e.mem.u32(this + PICK_ANIM_MANAGER), 0x44);
    }

    #[test]
    fn class_tiles_and_small_helpers() {
        let mut e = world();
        assert_eq!(e.call(0x0078_da90, &args![0x10u32]).u32(), 0x3f6);
        assert_eq!(e.call(0x0078_e080, &args![]).u32(), 0x3f6);
        let this = menu(&mut e);
        assert!(e.call(0x0078_e090, &args![this]).bool());
        e.call(0x0078_daa0, &args![this, 15u32, 0u32]);
        assert!(!e.call(0x0078_e090, &args![this]).bool());
        e.call(0x0078_daa0, &args![this, 16u32, 0x55u32]);
        assert_eq!(e.mem.u32(this + 0x6c), 0);
        e.call(0x0078_e0d0, &args![]);
        e.set_global(LOCK_PICK_MENU_ME, this);
        e.call(0x0078_e0d0, &args![]);
        assert_eq!(e.mem.u32(this + LOCK_STATE), 7);
        assert_eq!(e.call(0x0079_0290, &args![]).u32(), 0x14);
        e.register(GAMEPAD_HINT_709D40, |_, _| ret(1));
        assert_eq!(e.call(0x0079_0290, &args![]).u32(), 0x1c);
    }

    #[test]
    fn do_enter_clears_the_root_alpha_for_id_13() {
        let mut e = world();
        e.register(INTERFACE_ROOT_TILE, |_, _| ret(0x6600));
        e.call_log = Some(vec![]);
        e.call(0x0079_02b0, &args![0x10u32, 0xcu32, 0u32]);
        assert!(calls(&e, TILE_SET_FLOAT).is_empty());
        e.call(0x0079_02b0, &args![0x10u32, 0xdu32, 0u32]);
        assert_eq!(calls(&e, TILE_SET_FLOAT), vec![vec![0x6600, 0xfa9, 0, 1]]);
    }

    #[test]
    fn deleting_destructor_releases_the_scene() {
        let mut e = world();
        let root = node(&mut e);
        e.register_double(SCENE_ROOT, move |_, _| ret(root));
        e.set_global(LOCK_MODEL, 0x5500u32);
        e.set_global(CAMERA, 0x5600u32);
        let this = menu(&mut e);
        e.set_global(LOCK_PICK_MENU_ME, this);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_dad0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), LOCK_PICK_MENU_VTABLE);
        assert_eq!(calls(&e, DETACH_DOUBLE), vec![vec![root, 0x5500]]);
        assert_eq!(calls(&e, NODE_CLEAN_UP_572160), vec![vec![0x5500]]);
        assert_eq!(e.global::<u32>(LOCK_MODEL), 0);
        assert_eq!(e.global::<u32>(CAMERA), 0);
        assert_eq!(e.global::<u32>(LOCK_PICK_MENU_ME), 0);
        assert_eq!(calls(&e, SCENE_ROOT_B5D180), vec![vec![root]]);
        assert_eq!(calls(&e, MENU_DESTROY), vec![vec![this]]);
        assert_eq!(calls(&e, FREE), vec![vec![this]]);
    }

    #[test]
    fn create_refuses_a_lock_above_the_skill() {
        let mut e = world();
        e.register(LOCK_VALUE_TO_LEVEL, |_, _| ret(2));
        e.register(CLAMP_LOW_647B70, |_, a| ret(a[0]));
        e.register(CLAMP_HIGH_4A8F20, |_, a| ret(a[0]));
        e.register(GET_CLAMPED_ACTOR_VALUE, |_, _| ret(49));
        e.register(GET_SOUND_HANDLE_BY_NAME, |_, _| ret(0x2200));
        e.set_global(SKILL_FOR_LEVEL + 8, 50u32);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_db00, &args![0x8800u32, 0u32, 0u32]).u32(), 0);
        assert_eq!(calls(&e, PRINT_FORMATTED)[0][1..], [0x104, 0, 50]);
        let shown = calls(&e, SHOW_MESSAGE);
        assert_eq!(shown[0][1..4], [0, SKILL_TOO_LOW_SOUND, 0]);
        assert_eq!(
            calls(&e, GET_SOUND_HANDLE_BY_NAME)[0][2..],
            [FAIL_SOUND_NAME, 0x121]
        );
        assert_eq!(calls(&e, SOUND_PLAY), vec![vec![0x2200, 0]]);
        assert!(calls(&e, TILE_READ_FILE).is_empty());
    }

    #[test]
    fn create_builds_the_menu() {
        let mut e = world();
        let this = menu(&mut e);
        e.register(TILE_READ_FILE, |_, _| ret(0x6000));
        e.register_double(TILE_GET_MENU, move |_, _| ret(this));
        e.register(CLAMP_LOW_647B70, |_, a| ret(a[0]));
        e.register(CLAMP_HIGH_4A8F20, |_, a| ret(a[0]));
        e.register(COMPARE_STRINGS, |_, _| ret(1));
        e.register(TILE_GET_CHILD_BY_NAME, |_, a| ret(a[1] & 0xffff));
        e.register(GET_ITEM_COUNT, |_, _| ret(5));
        e.register(GET_FORCE_LOCK_CHANCE, |_, _| ret(33));
        e.call_log = Some(vec![]);
        // No target: the level comes from the argument and the skill is not checked.
        assert_eq!(
            e.call(0x0078_db00, &args![0u32, 3u32, 0x101u32]).u32(),
            0x6000
        );
        assert_eq!(calls(&e, CLAMP_LOW_647B70), vec![vec![3, 0]]);
        assert_eq!(e.global::<u32>(LOCK_PICK_MENU_ME), this);
        assert_eq!(calls(&e, MENU_SET_MENU_TILE), vec![vec![this, 0x6000, 0]]);
        assert_eq!(e.mem.u32(this + LOCK_TARGET), 0);
        assert_eq!(e.mem.u32(this + LOCK_LEVEL), 3);
        let values = calls(&e, TILE_SET_VALUE);
        assert_eq!(values[0], vec![0x7777, 0, 1]);
        assert_eq!(values[1], vec![0x9010, 0, 0]);
        assert_eq!(calls(&e, GET_ITEM_COUNT).len(), 0);
        let printed = calls(&e, PRINT_FORMATTED);
        assert_eq!(printed[1][2..], [CHANCE_FORMAT, 0, 33]);
        assert_eq!(calls(&e, TILE_GET_CHILD_BY_NAME).len(), 3);
        assert_eq!(calls(&e, MENU_PREP_FOR_VISIBILITY), vec![vec![this, 0]]);
        // Another class: the menu is deleted.
        e.register(CLASS_DOUBLE, |_, _| ret(0x400));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_db00, &args![0u32, 3u32, 0u32]).u32(), 0);
        assert_eq!(calls(&e, DELETE_DOUBLE), vec![vec![this, 1]]);
    }

    #[test]
    fn prepare_3d_elements_builds_the_lock_model() {
        let mut e = world();
        let this = menu(&mut e);
        let model = node(&mut e);
        let root = node(&mut e);
        e.register(LOAD_FILE, |_, a| {
            ret(if a[1] == LOCK_MODEL_FILE {
                0x5100
            } else {
                0x5200
            })
        });
        e.register(NI_ALLOCATE, |_, a| ret(0x5300 + a[0]));
        e.register_double(NI_NODE_CONSTRUCT, move |_, _| ret(model));
        e.register_double(SCENE_ROOT, move |_, _| ret(root));
        e.register(GET_CONTROLLER, |_, a| ret(a[0] + 1));
        e.register(GET_SEQUENCE_BY_NAME, |e, a| {
            ret(a[0] * 0x10 + e.mem.u32(a[1]) % 0x10)
        });
        e.register(FIXED_STRING_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(POINT_CONSTRUCT, |_, a| ret(a[0]));
        e.register(BOUND_RADIUS, |_, _| float(2.0));
        e.set_global(LIGHT_RADIUS_FACTOR, 1.5f64);
        e.register(CAMERA_CONSTRUCT, |_, _| ret(0x5900));
        e.register(TANGENT, |_, _| float(1.0));
        e.register(SCREEN_HEIGHT, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0078_e1c0, &args![this]);
        assert_eq!(e.global::<u32>(LOCK_MODEL), model);
        assert_eq!(
            calls(&e, ATTACH_DOUBLE),
            vec![
                vec![model, 0x5100, 1],
                vec![model, 0x5200, 1],
                vec![root, model, 1]
            ]
        );
        assert_eq!(calls(&e, NODE_SET_TRANSLATE)[0], vec![0x5200, this + 0xd8]);
        assert_eq!(e.mem.u32(this + INTRO_ANIM), 0x5101 * 0x10 + 8);
        assert_eq!(e.mem.u32(this + PICK_ANIM_MANAGER), 0x5201);
        assert_eq!(e.mem.u32(this + PICK_BREAK_ANIM), 0x5201 * 0x10 + 4);
        assert_eq!(
            calls(&e, SEQUENCE_SET_CYCLE),
            vec![vec![0x5101 * 0x10 + 0xc, 2], vec![0x5201 * 0x10 + 4, 2]]
        );
        assert_eq!(
            calls(&e, START_ANIMATIONS),
            vec![vec![0x5100], vec![0x5200]]
        );
        assert_eq!(e.global::<u32>(CAMERA), 0x5900);
        assert_eq!(
            calls(&e, ADD_LIGHTS_FROM_NODE),
            vec![vec![root, model, bits(3.0)]]
        );
    }

    #[test]
    fn render_runs_only_with_a_lock_model() {
        let mut e = world();
        e.call_log = Some(vec![]);
        e.call(0x0078_e7c0, &args![]);
        assert!(calls(&e, RENDERER_BEGIN_7148C0).is_empty());
        e.set_global(LOCK_MODEL, 0x5500u32);
        e.register(RENDERER, |_, _| ret(0x4100));
        e.register(SCENE_OBJECT, |_, _| ret(0x4200));
        e.register(CULLING_PROCESS, |_, _| ret(0x4300));
        e.register(SCENE_CAMERA_6629F0, |_, _| ret(0x4500));
        e.register(SCENE_ROOT, |_, _| ret(0x4600));
        e.register(RENDERED_TEXTURE_7FD640, |_, _| ret(0x4700));
        e.register(POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0078_e7c0, &args![]);
        assert_eq!(calls(&e, RENDERER_BEGIN_7148C0), vec![vec![0x4100, 0, 4]]);
        assert_eq!(
            calls(&e, ACCUMULATE_SCENE),
            vec![vec![0x4500, 0x4600, 0x4300]]
        );
        assert_eq!(
            calls(&e, RENDER_SCENE_B6C0D0),
            vec![vec![0x4500, 0x4700, 0]]
        );
        assert_eq!(
            calls(&e, CULLING_SET_TARGET),
            vec![vec![0x4300, 0x4700], vec![0x4300, 0]]
        );
        assert_eq!(calls(&e, CULLING_RESET_C4F2D0), vec![vec![0x4300]]);
    }

    #[test]
    fn camera_gets_its_frustum() {
        let mut e = world();
        e.register(NI_ALLOCATE, |_, _| ret(0x5900));
        e.register(CAMERA_CONSTRUCT, |_, a| ret(a[0]));
        let scene = {
            let vtable = e.mem.alloc(0x110);
            e.mem.set_u32(vtable + SCENE_SLOT_FAR, FAR_DOUBLE);
            let scene = e.mem.alloc(8);
            e.mem.set_u32(scene, vtable);
            scene
        };
        e.register_double(SCENE_OBJECT, move |_, _| ret(scene));
        e.register(FAR_DOUBLE, |_, _| float(100.0));
        let settings = e.mem.alloc(8);
        e.mem.set_f32(settings, 4.0);
        e.mem.set_f32(settings + 4, 90.0);
        e.register_double(SETTING_FLOAT, move |_, a| {
            ret(settings + if a[0] == NEAR_SETTING { 0 } else { 4 })
        });
        e.register(SCREEN_WIDTH, |_, _| ret(4));
        e.register(SCREEN_HEIGHT, |_, _| ret(2));
        e.register(TANGENT, |_, _| float(0.5));
        e.set_global(FRUSTUM_SCALE, 2.0f32);
        e.set_global(DEGREES_TO_RADIANS, 1.0f64);
        e.set_global(HALF_70C80, 0.5f64);
        let frustum = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = frustum.clone();
        e.register_double(CAMERA_SET_FRUSTUM, move |e, a| {
            for word in 0..6 {
                seen.borrow_mut().push(e.mem.f32(a[1] + 4 * word));
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_e900, &args![0x10u32]).u32(), 0x5900);
        assert_eq!(*frustum.borrow(), vec![-1.0, 1.0, 2.0, -2.0, 4.0, 100.0]);
        assert_eq!(calls(&e, CAMERA_SET_ASPECT), vec![vec![0x5900, bits(25.0)]]);
        assert_eq!(calls(&e, TANGENT)[0], vec![bits(45.0)]);
        let look = &calls(&e, CAMERA_LOOK_AT)[0];
        assert_eq!(look[0], 0x5900);
        assert_eq!(calls(&e, CAMERA_SET_LOD), vec![vec![0x5900, bits(1.0)]]);
    }

    /// A menu ready for `DoIdle`: on top, a previous tick 100 ticks ago,
    /// 1000 ticks per second, the lock model's children.
    fn idle_menu(e: &mut Engine, state: u32) -> u32 {
        let this = menu(e);
        e.mem.set_u32(this + LOCK_STATE, state);
        e.mem.set_u32(this + PREV_IDLE_TICK, 900);
        e.set_global(LAST_STATE, state);
        e.set_global(TICKS_PER_SECOND, 1000.0f64);
        e.set_global(ANGLE_OFFSET_66808, 90.0f64);
        e.set_global(ONE, 1.0f64);
        e.register(TICKS, |_, _| ret(1000));
        e.register(IS_TOP_MENU, |_, _| ret(1));
        e.set_global(LOCK_MODEL, 0x5500u32);
        e.register(NODE_CHILD, |_, a| ret(0x5600 + a[1]));
        e.register(GET_SOUND_HANDLE_BY_NAME, |_, a| ret(a[2]));
        this
    }

    fn sounds(e: &Engine) -> Vec<u32> {
        calls(e, SOUND_PLAY).iter().map(|call| call[0]).collect()
    }

    #[test]
    fn do_idle_counts_seconds_and_waits_for_the_tutorial() {
        let mut e = world();
        let this = idle_menu(&mut e, 9);
        e.set_global(STATE_SECONDS, 0.25f32);
        e.call_log = Some(vec![]);
        e.call(0x0078_eb50, &args![this]);
        assert_eq!(calls(&e, SOUND_HANDLE_CONSTRUCT), vec![vec![LOOP_SOUND]]);
        assert_eq!(calls(&e, ATEXIT), vec![vec![LOOP_SOUND_EXIT]]);
        assert_eq!(e.global::<f32>(STATE_SECONDS), 0.35);
        assert_eq!(e.mem.u32(this + PREV_IDLE_TICK), 1000);
        // A new state restarts the count; the static is made once.
        e.mem.set_u32(this + LOCK_STATE, 8);
        e.mem.set_u8(this + SHOWING_TUTORIAL, 1);
        e.call_log = Some(vec![]);
        e.call(0x0078_eb50, &args![this]);
        assert!(calls(&e, SOUND_HANDLE_CONSTRUCT).is_empty());
        assert_eq!(e.global::<f32>(STATE_SECONDS), 0.0);
        assert_eq!(e.global::<u32>(LAST_STATE), 8);
        assert_eq!(e.mem.u8(this + SHOWING_TUTORIAL), 1);
        assert!(calls(&e, USING_GAMEPAD).is_empty());
        e.register(TUTORIAL_SEEN, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0078_eb50, &args![this]);
        assert_eq!(e.mem.u8(this + SHOWING_TUTORIAL), 0);
        assert_eq!(calls(&e, TUTORIAL_SEEN)[0], vec![0x4400 + 0x4d4, 0x14]);
    }

    #[test]
    fn do_idle_intro_moves_to_the_ready_state() {
        let mut e = world();
        let this = idle_menu(&mut e, 0);
        e.mem.set_u32(this + INTRO_ANIM, 0x31);
        e.mem.set_u32(this + LOCK_ROTATE_ANIM, 0x32);
        e.mem.set_u32(this + PICK_ANIM_MANAGER, 0x33);
        e.mem.set_u32(this + PICK_INTRO_ANIM, 0x34);
        e.mem.set_u32(this + PICK_BEND_ANIM, 0x35);
        e.set_global(STATE_SECONDS, 2.0f32);
        e.register(SEQUENCE_END_KEY, |_, _| float(1.0));
        e.register(GET_CONTROLLER, |_, _| ret(0x3800));
        let point = e.mem.alloc(0xc);
        e.register_double(POINT_OF_NODE, move |_, _| ret(point));
        e.call_log = Some(vec![]);
        e.call(0x0078_eb50, &args![this]);
        assert_eq!(calls(&e, UPDATE_DOWNWARD).len(), 2);
        assert_eq!(e.mem.u32(this + LOCK_STATE), 2);
        assert_eq!(calls(&e, DEACTIVATE_SEQUENCE)[0][..2], [0x3800, 0x31]);
        assert_eq!(calls(&e, ACTIVATE_SEQUENCE)[1][..2], [0x33, 0x35]);
        // Before the end of the intro nothing changes.
        e.mem.set_u32(this + LOCK_STATE, 0);
        e.register(SEQUENCE_END_KEY, |_, _| float(9.0));
        e.call_log = Some(vec![]);
        e.call(0x0078_eb50, &args![this]);
        assert_eq!(e.mem.u32(this + LOCK_STATE), 0);
        assert!(calls(&e, ACTIVATE_SEQUENCE).is_empty());
    }

    #[test]
    fn do_idle_ready_starts_turning_and_lowers_the_lock() {
        let mut e = world();
        let this = idle_menu(&mut e, 2);
        e.register(CONTROL_PRESSED, |_, a| {
            ret(u32::from(a[1] == 0 && a[2] == 1))
        });
        e.mem.set_f32(this + LOCK_ANGLE, 20.0);
        e.register(FLOAT_MAX, |_, a| {
            float(f64::from(f32::from_bits(a[0]).max(0.0)))
        });
        e.register(FLOAT_MIN, |_, a| float(f64::from(f32::from_bits(a[0]))));
        e.register(FLOAT_ABS, |_, a| {
            float(f64::from(f32::from_bits(a[0]).abs()))
        });
        e.set_global(CLICK_ANGLE_A, 15.0f64);
        e.set_global(CLICK_ANGLE_B, 5.0f64);
        e.call_log = Some(vec![]);
        e.call(0x0078_eb50, &args![this]);
        assert_eq!(e.mem.u32(this + LOCK_STATE), 3);
        // 100 ticks of 90 degrees a second lower the lock by 9.
        assert_eq!(e.mem.f32(this + LOCK_ANGLE), 11.0);
        assert_eq!(sounds(&e), vec![START_SOUND, CLICK_SOUND_A]);
    }

    #[test]
    fn do_idle_turning_opens_an_unowned_lock() {
        let mut e = world();
        let this = idle_menu(&mut e, 3);
        e.register(CONTROL_PRESSED, |_, _| ret(1));
        e.register(FLOAT_MIN, |_, a| {
            float(f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))))
        });
        e.mem.set_f32(this + LOCK_ANGLE, 85.0);
        e.mem.set_f32(this + MAX_LOCK_ANGLE, 90.0);
        e.mem.set_u32(this + LOCK_TARGET, 0x8800);
        let lock = e.mem.alloc(0x14);
        e.register_double(GET_LOCK, move |_, _| ret(lock));
        e.register(GET_EXPERIENCE, |_, _| ret(12));
        e.register(FORM_TYPE, |_, _| ret(0x1c));
        e.set_global(LOCK_PICK_MENU_ME, this);
        e.call_log = Some(vec![]);
        e.call(0x0078_eb50, &args![this]);
        assert_eq!(e.mem.f32(this + LOCK_ANGLE), 90.0);
        let player: u32 = e.global(PLAYER);
        assert_eq!(calls(&e, REWARD_XP_DOUBLE), vec![vec![player, 12]]);
        assert_eq!(e.mem.u32(lock + 0x10), 1);
        assert_eq!(calls(&e, UNLOCK), vec![vec![0x8800]]);
        assert_eq!(calls(&e, ACTIVATE), vec![vec![0x8800, player, 0, 0, 1]]);
        assert_eq!(e.mem.u8(player + 0x20c), 0);
        assert!(sounds(&e).contains(&UNLOCK_SOUND));
        assert_eq!(e.mem.u32(this + LOCK_STATE), 7);
        assert_eq!(calls(&e, CONTROL_STOP_RUMBLE), vec![vec![0x3300]]);
    }

    #[test]
    fn do_idle_turning_releases_or_bends_the_pick() {
        let mut e = world();
        let this = idle_menu(&mut e, 3);
        e.call_log = Some(vec![]);
        e.call(0x0078_eb50, &args![this]);
        assert_eq!(e.mem.u32(this + LOCK_STATE), 2);
        assert_eq!(calls(&e, CONTROL_STOP_RUMBLE).len(), 1);
        // Turning at the pick's limit below the open angle bends it.
        e.register(CONTROL_PRESSED, |_, _| ret(1));
        e.mem.set_u32(this + LOCK_STATE, 3);
        e.mem.set_f32(this + LOCK_ANGLE, 30.0);
        e.mem.set_f32(this + MAX_LOCK_ANGLE, 30.0);
        e.register(FLOAT_MIN, |_, a| float(f64::from(f32::from_bits(a[1]))));
        e.register(GET_SOUND_HANDLE_BY_NAME, |_, a| ret(a[2]));
        e.call_log = Some(vec![]);
        e.call(0x0078_eb50, &args![this]);
        assert_eq!(e.mem.u32(this + LOCK_STATE), 4);
        assert_eq!(
            calls(&e, SOUND_HANDLE_ASSIGN),
            vec![vec![LOOP_SOUND, STRAIN_SOUND]]
        );
        assert_eq!(calls(&e, SOUND_PLAY).last().unwrap(), &vec![LOOP_SOUND, 1]);
    }

    #[test]
    fn do_idle_bending_breaks_a_worn_pick() {
        let mut e = world();
        let this = idle_menu(&mut e, 4);
        e.register(CONTROL_PRESSED, |_, _| ret(1));
        e.mem.set_f32(this + PICK_HEALTH, 0.05);
        let health = e.mem.alloc(4);
        e.mem.set_f32(health, 1.0);
        e.register_double(SETTING_FLOAT, move |_, _| ret(health));
        e.register(FLOAT_MAX, |_, a| {
            float(f64::from(f32::from_bits(a[0]).max(0.0)))
        });
        e.register(TILE_GET_FLOAT, |_, _| float(3.0));
        e.mem.set_u32(this + LOCK_TARGET, 0x8800);
        e.mem.set_u32(this + PICK_ANIM_MANAGER, 0x33);
        e.mem.set_u32(this + PICK_BEND_ANIM, 0x35);
        e.mem.set_u32(this + PICK_BREAK_ANIM, 0x36);
        e.set_global(SHAKE_SIGN, 1u32);
        e.set_global(LOCKPICK_ITEM, 0x7100u32);
        e.call_log = Some(vec![]);
        e.call(0x0078_eb50, &args![this]);
        assert_eq!(e.mem.f32(this + PICK_HEALTH), 0.0);
        assert_eq!(e.global::<i32>(SHAKE_SIGN), -1);
        assert_eq!(calls(&e, SOUND_FADE_OUT), vec![vec![LOOP_SOUND, 0x64]]);
        let player: u32 = e.global(PLAYER);
        assert_eq!(
            calls(&e, REMOVE_ITEM_DOUBLE)[0][..4],
            [player, 0x7100, 0, 1]
        );
        let set = calls(&e, TILE_SET_FLOAT);
        assert!(set.contains(&vec![0x9010, 0, bits(2.0), 1]));
        assert_eq!(calls(&e, ACTIVATE_SEQUENCE)[0][..2], [0x33, 0x36]);
        assert_eq!(e.mem.u32(this + LOCK_STATE), 5);
        // Letting go fades the strain out.
        e.register(CONTROL_PRESSED, |_, _| ret(0));
        e.mem.set_u32(this + LOCK_STATE, 4);
        e.call_log = Some(vec![]);
        e.call(0x0078_eb50, &args![this]);
        assert_eq!(calls(&e, SOUND_FADE_OUT), vec![vec![LOOP_SOUND, 0x1f4]]);
        assert_eq!(e.mem.u32(this + LOCK_STATE), 2);
    }

    #[test]
    fn do_idle_break_new_pick_and_closing() {
        let mut e = world();
        let this = idle_menu(&mut e, 5);
        e.mem.set_u32(this + PICK_BREAK_ANIM, 0x36);
        e.set_global(STATE_SECONDS, 2.0f32);
        e.register(SEQUENCE_END_KEY, |_, _| float(1.0));
        e.call(0x0078_eb50, &args![this]);
        assert_eq!(e.mem.u32(this + LOCK_STATE), 6);
        // State 6 after a second: a new pick, then state 1.
        e.mem.set_f32(this + LOCKPICK_SKILL_FLOAT, 10.0);
        let scale = e.mem.alloc(4);
        e.mem.set_f32(scale, 0.5);
        e.register_double(SETTING_FLOAT, move |_, _| ret(scale));
        e.set_global(STATE_SECONDS, 1.5f32);
        e.set_global(LAST_STATE, 6u32);
        e.call_log = Some(vec![]);
        e.call(0x0078_eb50, &args![this]);
        assert_eq!(e.mem.f32(this + PICK_HEALTH), 6.0);
        assert_eq!(e.mem.u32(this + LOCK_STATE), 1);
        // State 7 fades the menu out unless it is fading in.
        e.mem.set_u32(this + LOCK_STATE, 7);
        e.register(D3D_FADE_STATE, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0078_eb50, &args![this]);
        assert_eq!(calls(&e, MENU_START_FADE_OUT), vec![vec![this]]);
        assert_eq!(calls(&e, INTERFACE_UPDATE_718930), vec![vec![0x4400, 1]]);
    }

    #[test]
    fn rotate_lockpick_follows_the_mouse_and_the_sweet_spot() {
        let mut e = world();
        let this = menu(&mut e);
        e.register(CONTROL_MOUSE_AXIS, |_, _| ret(10));
        e.register(INTERFACE_SCREEN_WIDTH, |_, _| float(2.0));
        e.register(INTERFACE_MOUSE_SCALE, |_, _| float(4.0));
        // The pick tile (+0x68) is at 0x90f0 (x 40), the spot tile (+0x64) at 0x90e0.
        e.register(TILE_GET_FLOAT, |_, a| {
            float(match (a[0], a[1]) {
                (0x90f0, _) => 40.0,
                (0x90e0, 0xfb1) => 100.0,
                (_, 0xfaa) => 2.0,
                _ => 8.0,
            })
        });
        e.register(FLOAT_MAX, |_, a| float(f64::from(f32::from_bits(a[0]))));
        e.register(FLOAT_MIN, |_, a| float(f64::from(f32::from_bits(a[0]))));
        e.register(FLOAT_ABS, |_, a| {
            float(f64::from(f32::from_bits(a[0]).abs()))
        });
        e.mem.set_f32(this + SWEET_SPOT, 43.0);
        e.mem.set_u32(this + SWEET_SPOT_RADIUS, 5);
        e.set_global(SPOT_SCALE, 2.0f64);
        e.set_global(FAA_OFFSET, 1.0f64);
        e.set_global(FAA_SCALE, 4.0f64);
        e.set_global(ANGLE_OFFSET_66808, 90.0f64);
        // The spot has one child ring, half width 4 (> 2 from the spot).
        let node = e.mem.alloc(8);
        let spot_tile = 0x90e0u32;
        e.map(spot_tile, 0x10);
        e.mem.set_u32(spot_tile + 4, node);
        e.register_double(TILE_LIST_NEXT, move |e, a| {
            e.mem.set_u32(a[1], 0);
            let item = e.mem.alloc(4);
            e.mem.set_u32(item, 0x9500);
            ret(item)
        });
        e.call_log = Some(vec![]);
        e.call(0x0079_08f0, &args![this]);
        // The turn is 2 / 4 * 10 = 5: the pick moves to 45 and the turn sound plays.
        assert!(calls(&e, TILE_SET_FLOAT).contains(&vec![0x90f0, 0, bits(45.0), 1]));
        assert_eq!(e.global::<f32>(LAST_TURN), 5.0);
        assert_eq!(sounds(&e), vec![TURN_SOUND]);
        // Inside the spot: the limit from the ring's 0xfaa (1 - (2 - 1) / 4) * 90.
        assert_eq!(e.mem.f32(this + MAX_LOCK_ANGLE), 67.5);
        assert!(calls(&e, TILE_SET_FLOAT).contains(&vec![0x90f0, 0xfb2, bits(8.0), 1]));
    }

    #[test]
    fn rotate_lockpick_outside_the_spot_uses_the_distance() {
        let mut e = world();
        let this = menu(&mut e);
        e.register(TILE_GET_FLOAT, |_, a| {
            float(if a[0] == 0x90e0 { 100.0 } else { 0.0 })
        });
        e.register(FLOAT_MAX, |_, a| float(f64::from(f32::from_bits(a[0]))));
        e.register(FLOAT_MIN, |_, a| float(f64::from(f32::from_bits(a[0]))));
        e.register(FLOAT_ABS, |_, a| {
            float(f64::from(f32::from_bits(a[0]).abs()))
        });
        e.mem.set_f32(this + SWEET_SPOT, 50.0);
        e.mem.set_u32(this + SWEET_SPOT_RADIUS, 10);
        e.set_global(SPOT_SCALE, 2.0f64);
        e.set_global(OUTSIDE_SCALE, 90.0f64);
        e.set_global(LAST_TURN, 1.0f32);
        e.call_log = Some(vec![]);
        e.call(0x0079_08f0, &args![this]);
        // (10 + 50 - 100) / (20 - 100) = 0.5, times 90.
        assert_eq!(e.mem.f32(this + MAX_LOCK_ANGLE), 45.0);
        assert!(sounds(&e).is_empty());
    }

    #[test]
    fn rotate_lockpick_model_turns_the_pick_around_its_point() {
        let mut e = world();
        let this = menu(&mut e);
        e.set_global(LOCK_MODEL, 0x5500u32);
        e.register(NODE_CHILD, |_, a| ret(0x5600 + a[1]));
        e.register(TILE_GET_FLOAT, |_, _| float(100.0));
        e.set_global(ANGLE_SCALE_6B160, 2.0f64);
        e.set_global(ANGLE_OFFSET_66808, 10.0f64);
        e.set_global(DEGREES_TO_RADIANS, 0.5f64);
        e.register(POINT_CONSTRUCT, |_, a| ret(a[0]));
        e.register(POINT_OP_4A0BD0, |_, a| ret(a[1]));
        e.register(POINT_OP_439E90, |_, a| ret(a[1]));
        e.register(POINT_OP_439EF0, |_, a| ret(a[1]));
        e.register(MATRIX_TIMES_POINT, |_, a| ret(a[1]));
        let product = e.mem.alloc(0x24);
        e.mem.set_u32(product, 0x1234);
        e.register_double(MATRIX_MULTIPLY, move |_, _| ret(product));
        e.call_log = Some(vec![]);
        e.call(0x0079_0df0, &args![this, bits(50.0)]);
        // (50 / 100 * 2 + 10) * 0.5 = 5.5.
        assert_eq!(calls(&e, MATRIX_MAKE_Y_ROTATION)[0][1], bits(5.5));
        let multiply = &calls(&e, MATRIX_MULTIPLY)[0];
        assert_eq!(multiply[2], this + PICK_ROTATE);
        let rotation = multiply[0];
        assert_eq!(calls(&e, NODE_SET_ROTATE), vec![vec![0x5601, rotation]]);
        assert_eq!(calls(&e, NODE_SET_TRANSLATE).len(), 1);
        assert_eq!(calls(&e, UPDATE_DOWNWARD)[0][0], 0x5601);
    }

    #[test]
    fn rotate_lock_sets_the_rotate_phase() {
        let mut e = world();
        let this = menu(&mut e);
        e.set_global(LOCK_MODEL, 0x5500u32);
        e.register(NODE_CHILD, |_, a| ret(0x5600 + a[1]));
        e.set_global(ANGLE_OFFSET_66808, 90.0f64);
        e.register(SEQUENCE_BEGIN_KEY, |_, _| float(1.0));
        e.register(SEQUENCE_END_KEY, |_, _| float(3.0));
        e.mem.set_f32(this + LOCK_ANGLE, 40.0);
        e.call_log = Some(vec![]);
        e.call(0x0079_0f40, &args![this, bits(5.0)]);
        assert!(calls(&e, SEQUENCE_SET_PHASE).is_empty());
        e.mem.set_u32(this + LOCK_ROTATE_ANIM, 0x32);
        e.call(0x0079_0f40, &args![this, bits(5.0)]);
        // (3 - 1) * 45 / 90 + 1 = 2.
        assert_eq!(
            calls(&e, SEQUENCE_SET_PHASE),
            vec![vec![0x32, bits(2.0), 0]]
        );
        assert_eq!(calls(&e, UPDATE_DOWNWARD)[0][0], 0x5600);
    }

    #[test]
    fn create_sweet_spot_sizes_and_places_the_spot() {
        let mut e = world();
        let this = menu(&mut e);
        e.mem.set_u32(this + LOCK_LEVEL, 1);
        e.mem.set_u32(this + LOCKPICK_SKILL, 30);
        e.set_global(SKILL_FOR_LEVEL + 4, 20u32);
        e.set_global(SKILL_FOR_LEVEL + 8, 40u32);
        e.register(FLOAT_MIN, |_, a| {
            float(f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))))
        });
        e.register(FLOAT_MAX, |_, a| {
            float(f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))))
        });
        // Settings: table A from 10 to 30, table B from 4 to 8.
        let values = e.mem.alloc(0x10);
        for (word, value) in [10u32, 30, 4, 8].into_iter().enumerate() {
            e.mem.set_u32(values + 4 * word as u32, value);
        }
        for (table, word) in [
            (SPOT_TABLE_A_LOW, 0u32),
            (SPOT_TABLE_A_HIGH, 1),
            (SPOT_TABLE_B_LOW, 2),
            (SPOT_TABLE_B_HIGH, 3),
        ] {
            e.set_global(table + 4, 0x100 + word);
        }
        e.register_double(SETTING_INT, move |_, a| ret(values + 4 * (a[0] - 0x100)));
        e.register(TILE_GET_FLOAT, |_, _| float(100.0));
        e.register(RANDOM_FLOAT, |_, _| float(0.5));
        e.register(MENU_RENDER_TEMPLATE, |_, _| ret(0x9900));
        e.register(COLOR_SUBTRACT, |_, a| ret(a[1]));
        e.register(COLOR_SCALE, |_, a| ret(a[1]));
        e.register(COLOR_ADD, |_, a| ret(a[1]));
        e.set_global(SPOT_COLOR_SCALE, 255.0f64);
        e.set_global(SPOT_STEPS, 6.0f64);
        e.register(RECT_CONSTRUCT, |e, a| {
            for word in 0..4u32 {
                e.mem.set_u32(a[0] + 4 * word, a[1 + word as usize]);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0079_1000, &args![this]);
        // The skill is a quarter into the range: widths 15 and 5.
        assert_eq!(e.global::<u32>(SWEET_SPOT_WIDTH_A), 15);
        assert_eq!(e.global::<u32>(SWEET_SPOT_WIDTH_B), 5);
        assert_eq!(e.mem.u32(this + SWEET_SPOT_RADIUS), 12);
        assert_eq!(e.mem.f32(this + SWEET_SPOT), 50.0);
        assert_eq!(calls(&e, TILE_DELETE_CHILDREN), vec![vec![0x90e0]]);
        let rings = calls(&e, MENU_RENDER_TEMPLATE);
        assert_eq!(rings.len(), 6);
        assert_eq!(rings[0][1..], [0x90e0, SWEET_SPOT_TEMPLATE, 0]);
        let values = calls(&e, TILE_SET_VALUE);
        assert_eq!(values[0], vec![0x9900, 0xfaa, 0x11]);
        // The first ring starts at 50 - 7 = 43, depth 6; the next one step (1) lower.
        assert_eq!(values[1], vec![0x9900, 0xfa1, 43]);
        assert_eq!(values[2], vec![0x9900, 0xfad, 6]);
        assert_eq!(values[4], vec![0x9900, 0xfa1, 42]);
        let floats = calls(&e, TILE_SET_FLOAT);
        assert_eq!(floats[0], vec![0x9900, 0xfb1, bits(14.0), 1]);
        assert_eq!(floats[1], vec![0x9900, 0xfb2, 0, 1]);
        assert_eq!(floats[2], vec![0x9900, 0xfb3, bits(255.0), 1]);
    }

    #[test]
    fn any_control_answers_true_on_the_first_pressed_one() {
        let mut e = world();
        e.register(CONTROL_PRESSED, |_, a| ret(u32::from(a[1] == 0)));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0079_1540, &args![0u32, 1u32]).bool());
        let asked: Vec<u32> = calls(&e, CONTROL_PRESSED).iter().map(|c| c[1]).collect();
        assert_eq!(asked, vec![2, 3, 0]);
        e.register(CONTROL_PRESSED, |_, _| ret(0));
        assert!(!e.call(0x0079_1540, &args![0u32, 1u32]).bool());
    }

    #[test]
    fn do_leave_restores_the_root_alpha_for_13() {
        let mut e = world();
        e.mem.set_f32(ENTER_ALPHA, 255.0);
        e.register(INTERFACE_ROOT_TILE, |_, _| ret(0x4500));
        e.call_log = Some(vec![]);
        e.call(0x0079_02f0, &args![0x10u32, 0xcu32, 0u32]);
        assert!(calls(&e, TILE_SET_FLOAT).is_empty());
        e.call(0x0079_02f0, &args![0x10u32, 0xdu32, 0u32]);
        assert_eq!(calls(&e, INTERFACE_ROOT_TILE), vec![vec![0x4400]]);
        assert_eq!(
            calls(&e, TILE_SET_FLOAT),
            vec![vec![0x4500, 0xfa9, bits(255.0), 1]]
        );
    }

    /// A menu at lock level 1 (skill 25 to 50) with skill 40, the tiles'
    /// children answering `0x7000 + id`, `0xfb1` 20.0 and `0xfa1` 30.0, and
    /// each level's width settings holding 100.
    fn click_world() -> (Engine, u32) {
        let mut e = world();
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + LOCK_LEVEL, 1);
        e.mem.set_u32(this + LOCKPICK_SKILL, 40);
        e.mem.set_u32(this + SWEET_SPOT_RADIUS, 4);
        e.mem.set_u32(this + TILES + 4 * 14, 0x7e00);
        e.mem.set_u32(SKILL_FOR_LEVEL + 4, 25);
        e.mem.set_u32(SKILL_FOR_LEVEL + 8, 50);
        e.mem.set_u32(SWEET_SPOT_WIDTH_A, 10);
        e.mem.set_u32(SWEET_SPOT_WIDTH_B, 20);
        e.set_global(SPOT_SCALE, 2.0f64);
        e.set_global(SPOT_STEPS, 5.0f64);
        for table in [
            SPOT_TABLE_A_LOW,
            SPOT_TABLE_A_HIGH,
            SPOT_TABLE_B_LOW,
            SPOT_TABLE_B_HIGH,
        ] {
            let setting = e.mem.alloc(4);
            e.mem.set_u32(setting, 100);
            e.mem.set_u32(table + 4, setting);
        }
        e.register(SETTING_INT, |_, a| ret(a[0]));
        e.register(SETTING_SET_INT, |_, _| Ret::default());
        e.register(TILE_GET_CHILD_BY_ID, |_, a| ret(0x7000 + a[1]));
        e.register(TILE_GET_FLOAT, |_, a| {
            float(if a[1] == 0xfb1 { 20.0 } else { 30.0 })
        });
        e.register(FLOAT_MIN, |_, a| {
            float(f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))))
        });
        e.register(FLOAT_MAX, |_, a| {
            float(f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))))
        });
        (e, this)
    }

    #[test]
    fn do_click_widens_the_first_sweet_spot() {
        let (mut e, this) = click_world();
        e.call_log = Some(vec![]);
        e.call(0x0079_0330, &args![this, 3u32, 0u32]);
        assert_eq!(e.mem.u32(SWEET_SPOT_WIDTH_A), 15);
        assert_eq!(e.mem.u32(this + SWEET_SPOT_RADIUS), 6);
        // (40 - 25) / (2 * 25) = 0.3.
        assert_eq!(calls(&e, FLOAT_MIN), vec![vec![bits(0.3), bits(1.0)]]);
        let text = calls(&e, STRING_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&e, STRING_FORMAT),
            vec![vec![text, INTEGER_FORMAT, 15]]
        );
        assert_eq!(calls(&e, TILE_SET_STRING)[0][1], 0xfc4);
        assert_eq!(calls(&e, STRING_DESTROY), vec![vec![text]]);
        // 5 * 0.7 truncated to 3 and 5 * 0.3 to 1, added to 100.
        let low = e.mem.u32(SPOT_TABLE_A_LOW + 4);
        let high = e.mem.u32(SPOT_TABLE_A_HIGH + 4);
        assert_eq!(
            calls(&e, SETTING_SET_INT),
            vec![vec![low, 103], vec![high, 101]]
        );
        let sets = calls(&e, TILE_SET_FLOAT);
        assert_eq!(sets.len(), 12);
        assert_eq!(sets[0], vec![0x7011, 0xfb1, bits(25.0), 1]);
        assert_eq!(sets[1], vec![0x7011, 0xfa1, bits(28.0), 1]);
        assert_eq!(sets[11][0], 0x7016);
        // Narrower (5): -5, half of it -2.
        e.call(0x0079_0330, &args![this, 5u32, 0u32]);
        assert_eq!(e.mem.u32(SWEET_SPOT_WIDTH_A), 10);
        assert_eq!(e.mem.u32(this + SWEET_SPOT_RADIUS), 4);
    }

    #[test]
    fn do_click_widens_the_second_sweet_spot() {
        let (mut e, this) = click_world();
        e.call_log = Some(vec![]);
        e.call(0x0079_0330, &args![this, 8u32, 0u32]);
        assert_eq!(e.mem.u32(SWEET_SPOT_WIDTH_B), 15);
        assert_eq!(e.mem.i32(this + SWEET_SPOT_RADIUS), -1);
        let low = e.mem.u32(SPOT_TABLE_B_LOW + 4);
        let high = e.mem.u32(SPOT_TABLE_B_HIGH + 4);
        // -5 * 0.7 truncated to -3 and -5 * 0.3 to -1.
        assert_eq!(
            calls(&e, SETTING_SET_INT),
            vec![vec![low, 97], vec![high, 99]]
        );
        let sets = calls(&e, TILE_SET_FLOAT);
        assert_eq!(sets.len(), 10);
        // Child 1: 20 + 1 * 2 * -5 / 5 = 18; 30 - (1 * -5) / 5 = 31.
        assert_eq!(sets[0], vec![0x7012, 0xfb1, bits(18.0), 1]);
        assert_eq!(sets[1], vec![0x7012, 0xfa1, bits(31.0), 1]);
        assert_eq!(sets[9], vec![0x7016, 0xfa1, bits(35.0), 1]);
        e.call(0x0079_0330, &args![this, 6u32, 0u32]);
        assert_eq!(e.mem.u32(SWEET_SPOT_WIDTH_B), 20);
    }

    #[test]
    fn do_click_forces_the_lock() {
        let (mut e, this) = click_world();
        e.mem.set_f32(FORCE_LOCK_ANGLE, 90.0);
        e.mem.set_f32(MESSAGE_SECONDS, 2.0);
        e.set_global(LOCK_PICK_MENU_ME, this);
        e.register(HANDLE_ENTRY_POINT, |_, _| Ret::default());
        e.register(RANDOM_GENERATOR, |_, _| ret(0x6600));
        e.register(RANDOM_UNSIGNED, |_, _| ret(10));
        e.register(GET_CLAMPED_ACTOR_VALUE, |_, _| ret(45));
        e.register(GET_FORCE_LOCK_CHANCE, |_, _| ret(20));
        e.call_log = Some(vec![]);
        e.call(0x0079_0330, &args![this, 9u32, 0u32]);
        let player: u32 = e.global(PLAYER);
        assert_eq!(
            calls(&e, HANDLE_ENTRY_POINT)[0][0..2].to_vec(),
            vec![0x20, player]
        );
        assert_eq!(calls(&e, RANDOM_UNSIGNED), vec![vec![0x6600, 100]]);
        assert_eq!(
            calls(&e, GET_CLAMPED_ACTOR_VALUE),
            vec![vec![player + 0xa4, LOCKPICK_ACTOR_VALUE]]
        );
        assert_eq!(calls(&e, GET_FORCE_LOCK_CHANCE), vec![vec![45, 25]]);
        assert_eq!(e.mem.u8(this + FORCE_LOCK_SUCCESS), 1);
        assert_eq!(e.mem.u32(this + MAX_LOCK_ANGLE), bits(90.0));
        assert_eq!(e.mem.u32(this + LOCK_STATE), 3);

        // The roll fails without a target: the lock is broken.
        e.register(RANDOM_UNSIGNED, |_, _| ret(30));
        e.mem.set_u32(this + TILES + 4 * 11, 0x7b00);
        e.call_log = Some(vec![]);
        e.call(0x0079_0330, &args![this, 9u32, 0u32]);
        assert_eq!(
            calls(&e, TILE_SET_STRING),
            vec![vec![0x7b00, 0xfc4, LOCK_BROKEN_TEXT, 1]]
        );
        assert_eq!(calls(&e, TILE_SET_VALUE), vec![vec![0x7b00, 0xfa3, 1]]);
        assert_eq!(e.mem.u32(this + LOCK_STATE), 6);

        // With a target: one more try on its lock, the message, closing.
        let lock = e.mem.alloc(0x10);
        e.mem.set_u32(lock + 0xc, 2);
        e.register_double(GET_LOCK, move |_, _| ret(lock));
        e.register(SETTING_STRING, |_, _| ret(0x5555));
        e.mem.set_u32(this + LOCK_TARGET, 0x7c00);
        e.call_log = Some(vec![]);
        e.call(0x0079_0330, &args![this, 9u32, 0u32]);
        assert_eq!(e.mem.u32(lock + 0xc), 3);
        assert_eq!(calls(&e, ADD_LOCK_CHANGE), vec![vec![0x7c00]]);
        assert_eq!(calls(&e, SETTING_STRING), vec![vec![FORCE_FAIL_SETTING]]);
        assert_eq!(
            calls(&e, SHOW_MESSAGE),
            vec![vec![
                0x5555,
                0,
                FORCE_FAIL_ICON,
                FORCE_FAIL_SOUND,
                bits(2.0),
                0
            ]]
        );
        assert_eq!(e.mem.u32(this + LOCK_STATE), 7);

        // 10 closes; other ids do nothing.
        e.mem.set_u32(this + LOCK_STATE, 0);
        e.call(0x0079_0330, &args![this, 10u32, 0u32]);
        assert_eq!(e.mem.u32(this + LOCK_STATE), 7);
        e.mem.set_u32(this + LOCK_STATE, 0);
        e.call_log = Some(vec![]);
        e.call(0x0079_0330, &args![this, 4u32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}
