//! `fallout/interface/menus/racesexmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `RaceSexMenu` is the menu of the character creation, the barber and the
//! plastic surgeon: a column of option lists (`MenuOptionList`, 20 of them,
//! 0x24 bytes each) of options (`MenuOption`, whose first word is the tile)
//! and sliders, and a rendered 3D view of the player. `pMe`
//! (`011da634`) is the open menu.
//!
//! Every function of the unit is translated (83; the 84th, `00f74f70`, is
//! a library initializer). The earlier functions call the later ones (the
//! `MenuOptionList` methods, `SliderRelease`, ...) by address, which keeps
//! their tests' doubles; the later ones call each other directly. The unit holds
//! three classes: `RaceSexMenu` (+ the compiler's deleting destructors),
//! `MenuOption` and `MenuOptionList`; the layouts below declare the fields
//! used so far. The first part of the file has the layouts, the exe
//! addresses of the callees outside this unit and small helpers.
//!
//! Notes that apply to every function:
//! - The class sits at the PC offsets the code uses; the Xbox PDB's
//!   `RaceSexMenu` is 0x180 bytes, the PC one 0x1A4 (the allocation in
//!   `CreateMenuByClass` pushes `0x1a4`). From `+0xD4` on the PC fields sit 4
//!   bytes after the PDB's (one extra float at `+0xD0`).
//! - x87: the game computes in extended precision and stores `float`
//!   results. The translations compute in `f64` and round to `f32` at each
//!   `float` store; a `FLD`/`FSTP` copy of a float constant makes a
//!   signalling NaN quiet (`x87_copy_f32`). Floats pushed as arguments are
//!   passed as their bit pattern.
//! - The compiler's exception-unwinding frames (`FS:[0]`) and the security
//!   cookie are not translated; the functions that have them carry a
//!   `Translation incomplete` marker.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_interface::forenderedmenuracesex::FORenderedMenuRaceSex;

/// cdecl, one word: the absolute value of a stick axis.
const ABSOLUTE_VALUE: u32 = 0x0072_fa60;
/// `Actor::GetHeight` (engine map): the height as a float in ST0.
const ACTOR_GET_HEIGHT: u32 = 0x0088_53a0;
/// `Tile::AddUserTrait` (engine map), cdecl (name, id): the trait id.
const ADD_USER_TRAIT: u32 = 0x00a0_0940;
/// the allocator, cdecl (size).
const ALLOCATE: u32 = 0x0040_1000;
/// thiscall on a head object: (coordinates, object, 0).
const APPLY_COORDS: u32 = 0x0065_a970;
/// cdecl (coordinates, flag, 0, word, value): applies a face slider to the coordinate array.
const APPLY_FACE_SLIDER: u32 = 0x0065_2320;
/// cdecl (object, value).
const APPLY_HAIR_VALUE: u32 = 0x0065_3470;
/// `BSFaceGenModel::ApplyCoordinateTexturingToMesh` (engine map): (coordinates, pointer, `006151d0` value).
const APPLY_TEXTURING: u32 = 0x0065_b410;
/// the third argument of the texturing call.
const APPLY_TEXTURING_ARG: u32 = 0x0061_51d0;
/// `BSAudio::QInstance` (engine map): the audio instance.
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
/// `BSAudioManager::QInstance` (engine map).
const AUDIO_MANAGER_INSTANCE: u32 = 0x00ad_9060;
/// cdecl (axis size, limit).
const AXIS_REDUCE: u32 = 0x004a_8f20;
/// thiscall on the input device: (0, axis number): the axis value.
const AXIS_VALUE: u32 = 0x00a2_3390;
/// a getter on the base form.
const BASE_VALUE_4AC110: u32 = 0x004a_c110;
/// a getter on the base form.
const BASE_VALUE_602150: u32 = 0x0060_2150;
/// a getter on the base form.
const BASE_VALUE_603B50: u32 = 0x0060_3b50;
/// thiscall on the input device: (0, button number).
const BUTTON_STATE: u32 = 0x00a2_3a50;
/// cdecl (object): a cast whose result has a head object in slot `+0x94`.
const CAST_607310: u32 = 0x0060_7310;
/// thiscall on a node: (index): the child.
const CHILD_AT: u32 = 0x0043_b4a0;
/// thiscall on a node: the number of children.
const CHILD_COUNT: u32 = 0x0043_b480;
/// `BSFaceGenManager::ClearBodyTexturesFromPalette` (engine map), cdecl (form).
const CLEAR_BODY_TEXTURES: u32 = 0x0065_70e0;
/// `TESNPC::ClearHead` (engine map).
const CLEAR_HEAD: u32 = 0x005d_d560;
/// `NiColorA` constructor (this, r, g, b, a); returns it.
const COLOR_A_CONSTRUCT: u32 = 0x0041_4430;
/// `TES::CreateTextureImage` (engine map): (path, pointer out, 0, 0).
const CREATE_TEXTURE_IMAGE: u32 = 0x0045_68c0;
/// thiscall on the menu: the number of the current option list (trait `0x1004` of the menu tile, truncated).
const CURRENT_LIST_INDEX: u32 = 0x007d_cc00;
/// answers whether a dialogue service is running.
const DIALOGUE_ACTIVE: u32 = 0x0070_5100;
/// `DialogMenu::ServiceEnd` (engine map), cdecl (topic).
const DIALOG_SERVICE_END: u32 = 0x0076_40a0;
/// `__ehvec_ctor`: (array, size, count, constructor, destructor).
const EH_VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
/// `__ehvec_dtor`: (array, size, count, destructor).
const EH_VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
/// an empty member function.
const EMPTY_MEMBER: u32 = 0x0048_3710;
/// returns its `this`.
const EMPTY_MEMBER_RETURNS_THIS: u32 = 0x0068_15c0;
/// `InterfaceManager::EnterRenderedMenu` (engine map): (menu).
const ENTER_RENDERED_MENU: u32 = 0x0071_81d0;
/// `Actor::EquipObject` (engine map): (item, 1, 0, 1, 0, 1).
const EQUIP_OBJECT: u32 = 0x0088_c830;
/// a check on a list item.
const EYE_CHECK_A: u32 = 0x005f_c4d0;
/// a check on a list item against the base form: (item, base form).
const EYE_CHECK_B: u32 = 0x005f_c5f0;
/// thiscall on the list owner: the first node of a list.
const EYE_LIST_HEAD: u32 = 0x0043_c490;
/// cdecl (two node pointers, face data, 0).
const FACE_BUILD: u32 = 0x0065_5320;
/// constructor of one face coordinate matrix (0x20 bytes).
const FACE_COORD_CONSTRUCT: u32 = 0x0044_9610;
/// destructor of one face coordinate matrix.
const FACE_COORD_DESTRUCT: u32 = 0x0044_9680;
/// constructor of the face data object (0xF4 bytes).
const FACE_DATA_CONSTRUCT: u32 = 0x005d_d590;
/// destructor of the face data object.
const FACE_DATA_DESTRUCT: u32 = 0x005d_d6f0;
/// cdecl (race value, base value, coordinates, 0, 0.0).
const FACE_FILL_COORDS: u32 = 0x0065_2af0;
/// `BSFaceGenManager::GetInstance` (engine map).
const FACE_GEN_MANAGER_INSTANCE: u32 = 0x0065_1b30;
/// cdecl (buffer, index): the name of a face part.
const FACE_PART_NAME: u32 = 0x0065_7820;
/// cdecl (coordinates, group, 0, shape): the shape value as a float in ST0.
const FACE_SHAPE_VALUE: u32 = 0x0065_2230;
/// thiscall (sex): a value of the race word.
const FACE_VALUE_5D9FB0: u32 = 0x005d_9fb0;
/// a value of the player base form.
const FACE_VALUE_601800: u32 = 0x0060_1800;
/// cdecl (race value, sex, array).
const FILL_PRESET_HEADS: u32 = 0x0060_3280;
/// cdecl (name object, string, length): 0 for a match.
const FIND_BY_NAME: u32 = 0x0045_64f0;
/// `NiFixedString` destructor.
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
/// `NiFixedString` constructor from a C string (this, text); returns this.
const FIXED_STRING_FROM: u32 = 0x0043_8170;
/// assigns a C string to a string through `004037f0` (this, text); returns this.
const FIXED_STRING_SET: u32 = 0x0043_8390;
/// cdecl (a, b): `b` when `b >= a` or unordered, else `a`; result in ST0.
const FLOAT_MAX: u32 = 0x0040_4010;
/// cdecl (a, b): `b` when `b <= a` or unordered, else `a`; result in ST0.
const FLOAT_MIN: u32 = 0x0040_ebd0;
/// thiscall on a float setting: its value in ST0.
const FLOAT_SETTING_GETTER: u32 = 0x0045_0410;
/// thiscall on the interface: the tile that has the focus.
const FOCUS_TILE: u32 = 0x0097_ae90;
/// thiscall on the base form: (player, 0).
const FORM_UPDATE_6062E0: u32 = 0x0060_62e0;
/// the deallocator, cdecl (pointer).
const FREE: u32 = 0x0040_1030;
/// `_ftol2_sse`: truncates ST0 (passed as an `f64`).
const FTOL: u32 = 0x00ec_62c0;
/// whether the gamepad interface is in use.
const GAMEPAD_IN_USE: u32 = 0x004b_71d0;
/// `GetCurrentTexLanguage` (engine map).
const GET_CURRENT_TEX_LANGUAGE: u32 = 0x004b_7220;
/// `TESNPC::GetFaceCoord` (engine map): (array).
const GET_FACE_COORD: u32 = 0x0060_3ad0;
/// `TESRace::GetFaceGenData` (engine map): (form, face data, value, 0).
const GET_FACE_GEN_DATA: u32 = 0x0061_41f0;
/// the float of a tile trait, in ST0: (tile, trait id).
const GET_FLOAT_TRAIT: u32 = 0x00a0_11b0;
/// `InventoryChanges::GetInventoryChanges` (engine map), cdecl (player).
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
/// `Tile::GetMaximumDepth` (engine map).
const GET_MAXIMUM_DEPTH: u32 = 0x00a0_1530;
/// `Interface::GetMessageMenuResult` (engine map).
const GET_MESSAGE_MENU_RESULT: u32 = 0x0070_3fa0;
/// `Interface::GetPipboy` (engine map).
const GET_PIPBOY: u32 = 0x0070_5990;
/// `NiAVObject::GetProperty` (engine map): (object, kind).
const GET_PROPERTY: u32 = 0x00a5_9d30;
/// pointer to the rotation matrix of a node.
const GET_ROTATE: u32 = 0x006a_9540;
/// `TESActorBase::GetSex` (engine map).
const GET_SEX: u32 = 0x005f_0cc0;
/// `BSAudioManager::GetSoundHandleByName` (engine map): (audio manager, out, name, 0x31).
const GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ae_5680;
/// `BSAudio::GetSoundHandleByName` (engine map): (audio, out, name, 0x121).
const GET_SOUND_HANDLE_BY_NAME_ON_AUDIO: u32 = 0x00ad_7550;
/// the time in milliseconds.
const GET_TIME: u32 = 0x0045_7fe0;
/// `TESTopic::GetTopic` (engine map), cdecl (5, 4).
const GET_TOPIC: u32 = 0x0061_a2d0;
/// `InventoryChanges::GetWornItem` (engine map): (changes, slot, 0).
const GET_WORN_ITEM: u32 = 0x004c_8c10;
/// a check on a hair list item.
const HAIR_CHECK_A: u32 = 0x005f_df40;
/// a check on a hair list item against the base form: (item, base form).
const HAIR_CHECK_B: u32 = 0x005f_dfa0;
/// a check for female hair.
const HAIR_CHECK_FEMALE: u32 = 0x005f_df80;
/// a check for male hair.
const HAIR_CHECK_MALE: u32 = 0x005f_df60;
/// the base form's packed hair colour.
const HAIR_COLOR_GETTER: u32 = 0x0041_69d0;
/// thiscall on the list owner: the first node of the hair list.
const HAIR_LIST_HEAD: u32 = 0x0061_3790;
/// a float value of the base form in ST0.
const HAIR_VALUE_602130: u32 = 0x0060_2130;
/// thiscall on the base form: (player, biped, word, word).
const HEAD_BUILD: u32 = 0x0060_7420;
/// the head part list inside the base form.
const HEAD_PART_LIST: u32 = 0x0060_2170;
/// (address of a word).
const HEAD_PART_LIST_ADD: u32 = 0x005a_e3d0;
/// empties the head part list.
const HEAD_PART_LIST_CLEAR: u32 = 0x0047_0470;
/// thiscall on a head object.
const HEAD_REFRESH: u32 = 0x0065_a840;
/// `TESNPC::InitHead` (engine map): (pointer, pointer).
const INIT_HEAD: u32 = 0x0060_7370;
/// thiscall on the input owner: the input device.
const INPUT_DEVICE: u32 = 0x0087_7720;
/// the interface singleton (the word at `011d8a80`).
const INTERFACE_SINGLETON: u32 = 0x004b_7210;
/// `Interface::IsTopMenuID` (engine map), cdecl (class id).
const IS_TOP_MENU_ID: u32 = 0x0070_2450;
/// `TESRace::KillEGTData` (engine map).
const KILL_EGT_DATA: u32 = 0x0061_3fd0;
/// `FORenderedMenuRaceSex::LightButton` (engine map): (rendered menu, button).
const LIGHT_BUTTON: u32 = 0x007f_e950;
/// thiscall on a list: (address of a cursor): pointer to the item, advances the cursor.
const LIST_ADVANCE: u32 = 0x0057_cbe0;
/// returns the word at `+0xC` of an option list (the list `Back` goes to).
const LIST_BACK_TARGET: u32 = 0x0084_e3a0;
/// whether a list is empty.
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// returns the word at `+0x10` of an option list.
const LIST_PARENT_TARGET: u32 = 0x0044_edb0;
/// stores its argument at `+0xC` of an option list.
const LIST_SET_BACK_TARGET: u32 = 0x0041_fd00;
/// `IOManager::LoadQueuedPriority` (engine map).
const LOAD_QUEUED_PRIORITY: u32 = 0x0045_6520;
/// whether bit 0 of the word at `+0x70` of an item is set.
const MARKER_CHECK: u32 = 0x0059_f610;
/// whether bit 0 of the byte at `+0x44` of an item is set.
const MARKER_CHECK_2: u32 = 0x005e_7a60;
/// thiscall on the list owner: the address of a list head (`+0x60`).
const MARKER_LIST_HEAD: u32 = 0x004e_a950;
/// thiscall on the list owner: the address of a list head (`+0x48`).
const MARKER_LIST_HEAD_2: u32 = 0x0062_4700;
/// `MapMarkerData::GetLocationName` (engine map).
const MARKER_NAME: u32 = 0x0040_8da0;
/// `NiMatrix3::operator*` (engine map): (this, result, other); returns the result.
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
/// thiscall on a matrix: (row, column, value).
const MATRIX_SET_ELEMENT: u32 = 0x0064_2660;
/// thiscall on a matrix: (angle).
const MATRIX_SET_ROTATION: u32 = 0x004a_0c90;
/// cdecl (address, value, size).
const MEMSET: u32 = 0x0040_3d30;
/// the `Menu` base constructor.
const MENU_CONSTRUCT: u32 = 0x00a1_c4a0;
/// the `Menu` base destructor.
const MENU_DESTRUCT: u32 = 0x00a1_c520;
/// `Menu::InstantFadeOut` (engine map).
const MENU_INSTANT_FADE_OUT: u32 = 0x00a1_d9e0;
/// `Menu::PrepForVisibility` (engine map): (flag).
const MENU_PREP_FOR_VISIBILITY: u32 = 0x00a1_dc20;
/// `Menu::SetMenuTile` (engine map): (tile, flag).
const MENU_SET_MENU_TILE: u32 = 0x00a1_dc70;
/// `TESObjectREFR::MultipleMatrixByRace` (engine map): (result, matrix).
const MULTIPLE_MATRIX_BY_RACE: u32 = 0x0056_fac0;
/// cdecl (name object, string): 0 for a match.
const NAME_COMPARE: u32 = 0x0040_8b20;
/// thiscall on a name object.
const NAME_HASH: u32 = 0x0043_b1b0;
/// `NiPointer::operator=`.
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `NiPointer` constructor (this, value).
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `NiPointer` default constructor.
const NI_POINTER_DEFAULT_CONSTRUCT: u32 = 0x0066_94e0;
/// `NiPointer` destructor.
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// thiscall on a node: its owner object.
const OBJECT_9611E0: u32 = 0x0096_11e0;
/// thiscall on an object: the name object (the stack words are left to the caller's next call).
const OBJECT_NAME: u32 = 0x0041_3f40;
/// thiscall on an option list (tile): the option of the tile.
const OPTION_AT_TILE: u32 = 0x007b_41c0;
/// thiscall on an option list (text, id): the new option.
const OPTION_LIST_ADD_OPTION: u32 = 0x007b_3b80;
/// `MenuOptionList::AddSliderOption` (engine map): (text, id, min, max, value, step).
const OPTION_LIST_ADD_SLIDER: u32 = 0x007b_3ca0;
/// adds a sub list: (text, list).
const OPTION_LIST_ADD_SUB_LIST: u32 = 0x007b_3bb0;
/// the base constructor of `MenuOptionList`.
const OPTION_LIST_BASE_CONSTRUCT: u32 = 0x0048_f200;
/// the base destructor of `MenuOptionList`.
const OPTION_LIST_BASE_DESTRUCT: u32 = 0x004a_1a30;
/// empties an option list.
const OPTION_LIST_CLEAR: u32 = 0x007b_45b0;
/// returns the word at `+0x20` of its object (the current option of a list, the player's base form of the player).
const OPTION_LIST_CURRENT: u32 = 0x007a_f430;
/// `MenuOptionList::DoGamepad` (engine map): (button).
const OPTION_LIST_DO_GAMEPAD: u32 = 0x007b_4260;
/// thiscall on an option list (id): the option with the id.
const OPTION_LIST_FIND: u32 = 0x007b_4210;
/// `MenuOptionList::Focus` (engine map): (flag).
const OPTION_LIST_FOCUS: u32 = 0x007b_3fd0;
/// `MenuOptionList::InsertMenuOption` (engine map): (position, text, template, id).
const OPTION_LIST_INSERT_OPTION: u32 = 0x007b_3d50;
/// thiscall on an option list (other list).
const OPTION_LIST_LINK: u32 = 0x007b_4050;
/// refreshes an option list.
const OPTION_LIST_REFRESH: u32 = 0x007b_4410;
/// `MenuOptionList::RemoveMenuOption` (engine map): (id).
const OPTION_LIST_REMOVE_OPTION: u32 = 0x007b_3dd0;
/// `MenuOptionList::ScrollBy` (engine map): (steps).
const OPTION_LIST_SCROLL_BY: u32 = 0x007b_4530;
/// stores a callback at `+0x1C` of an option list.
const OPTION_LIST_SET_CALLBACK: u32 = 0x0050_f9a0;
/// `MenuOptionList::UpdateCurrentSelection` (engine map).
const OPTION_LIST_UPDATE_CURRENT_SELECTION: u32 = 0x007b_40d0;
/// stores its argument at `+8` of an option.
const OPTION_SET_DATA: u32 = 0x0040_3550;
/// thiscall on a palette entry.
const PALETTE_ENTRY_RELEASE: u32 = 0x00a6_e960;
/// cdecl (table, palette): an entry.
const PALETTE_FIND: u32 = 0x0065_3270;
/// the palette of an animation object.
const PALETTE_OF: u32 = 0x0053_7bd0;
/// `PlayerCharacter::GetAnimation` (engine map): (0).
const PLAYER_ANIMATION: u32 = 0x0095_0a60;
/// a height float of the base form in ST0.
const PLAYER_BASE_HEIGHT: u32 = 0x0094_4300;
/// `TESNPC::GetWeight` (engine map): in ST0.
const PLAYER_BASE_WEIGHT: u32 = 0x0060_4250;
/// `PlayerCharacter::GetBiped` (engine map): (0).
const PLAYER_BIPED: u32 = 0x0095_0b00;
/// the player flag byte.
const PLAYER_FLAG_GETTER: u32 = 0x008a_6970;
/// sets the player flag byte (flag).
const PLAYER_FLAG_SETTER: u32 = 0x008a_6840;
/// the player's 3D object: (flag).
const PLAYER_NODE_GETTER: u32 = 0x0095_0bb0;
/// called on the player after a race change.
const PLAYER_RACE_CHANGED: u32 = 0x0095_e5f0;
/// thiscall on `player + 0x94`: (0).
const PLAYER_RECALCULATE: u32 = 0x0082_49d0;
/// thiscall on the player: (flag).
const PLAYER_REFRESH_3D: u32 = 0x008b_78c0;
/// called on the player after a race change.
const PLAYER_UPDATE: u32 = 0x008c_2b60;
/// a value of the player.
const PLAYER_VALUE_87F4C0: u32 = 0x0087_f4c0;
/// thiscall on the interface: the pointer position as a float in ST0.
const POINTER_X: u32 = 0x0076_97f0;
/// `NiPoint3` constructor (this, x, y, z).
const POINT_CONSTRUCT: u32 = 0x0043_d410;
/// `BSShaderManager::PrepareObject` (engine map), cdecl (object, 0, 0).
const PREPARE_OBJECT: u32 = 0x00b5_7e30;
/// constructor of the preset heads array.
const PRESET_HEADS_CONSTRUCT: u32 = 0x005e_0560;
/// destructor of the preset heads array.
const PRESET_HEADS_DESTRUCT: u32 = 0x005e_0590;
/// `PathingLocation::GetWorldspace` (engine map; a getter that the property blocks compare with 8 to 12).
const PROPERTY_KIND: u32 = 0x0044_1110;
/// (player, biped).
const QUEUED_HEAD_APPLY: u32 = 0x0043_ed90;
/// (base form, 0).
const QUEUED_HEAD_CONSTRUCT: u32 = 0x0043_eaf0;
/// destructor of the temporary.
const QUEUED_HEAD_DESTRUCT: u32 = 0x0043_ebb0;
/// a step on the temporary.
const QUEUED_HEAD_FINISH: u32 = 0x0043_c610;
/// `QueuedHead::Run` (engine map).
const QUEUED_HEAD_RUN: u32 = 0x0043_ecb0;
/// thiscall on the race value: (sex).
const RACE_DEFAULT_HAIR: u32 = 0x0061_3870;
/// thiscall on the race value: the first node of its hair list.
const RACE_HAIR_LIST_HEAD: u32 = 0x0045_bb80;
/// cdecl (four words).
const REGENERATE_FACE: u32 = 0x0065_2e00;
/// `RecurseAndRemoveObjectsFromPalette` (engine map), cdecl (node, palette).
const REMOVE_FROM_PALETTE: u32 = 0x00a6_e8e0;
/// the base constructor of the rendered menu.
const RENDERED_MENU_BASE_CONSTRUCT: u32 = 0x007f_aa20;
/// thiscall on the interface: the rendered menu.
const RENDERED_MENU_GETTER: u32 = 0x0070_7af0;
/// thiscall on a setting: the address of its value.
const RENDERED_MENU_SETTING: u32 = 0x0040_8d60;
/// stores its argument at `+0x16C`.
const RENDERED_MENU_SET_MODE: u32 = 0x007a_c9d0;
/// assigns a `NiPointer` at `+0x14`.
const RENDERED_MENU_SET_POINTER: u32 = 0x0070_5fc0;
/// stores its argument at `+0x18`.
const RENDERED_MENU_SET_ROOT: u32 = 0x0098_4f60;
/// `FORenderedMenuRaceSex` (scale float).
const RENDERED_MENU_SET_SCALE: u32 = 0x007f_d660;
/// `Menu::RenderTemplate` (engine map): (tile, template name, 0).
const RENDER_TEMPLATE: u32 = 0x00a1_ddb0;
/// cdecl (value, step): the value rounded to a multiple of the step, in ST0.
const ROUND_TO_STEP: u32 = 0x004b_d510;
/// `__RTDynamicCast`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `MiddleHighProcess::GetSavedAcquireObject` (engine map).
const SAVED_OBJECT: u32 = 0x008d_8520;
/// cdecl (index): a scene node.
const SCENE_NODE_GETTER: u32 = 0x0045_0b80;
/// the game settings collection.
const SETTINGS_COLLECTION: u32 = 0x0040_4a70;
/// thiscall on the collection: (name, address of the result).
const SETTINGS_FIND: u32 = 0x004f_8a90;
/// thiscall on a float setting: the address of its value.
const SETTING_FLOAT_POINTER: u32 = 0x0040_3e20;
/// thiscall on an int setting: the address of its value.
const SETTING_INT_POINTER: u32 = 0x0043_d4d0;
/// thiscall on a setting: its text (like `00403df0`).
const SETTING_TEXT: u32 = 0x004c_69f0;
/// thiscall on a setting: the word at `+4`, 0 for a null setting.
const SETTING_VALUE: u32 = 0x0040_3df0;
/// `InterfaceManager::SetCurrentFocusTarget` (engine map): (tile, 0xfc3, 1).
const SET_CURRENT_FOCUS_TARGET: u32 = 0x0071_5860;
/// `TESActorBaseData::SetFlagBit` (engine map): (1, set, 1).
const SET_FLAG_BIT: u32 = 0x0047_dd50;
/// (tile, trait id, float, flag).
const SET_FLOAT_TRAIT: u32 = 0x00a0_12d0;
/// `Interface::SetCurrentFocusTarget` (engine map), cdecl (tile).
const SET_FOCUS_TARGET_CDECL: u32 = 0x0070_6ff0;
/// thiscall on the base form: (hair).
const SET_HAIR: u32 = 0x0060_31e0;
/// thiscall on the base form: (head).
const SET_HEAD: u32 = 0x0060_3200;
/// thiscall on the base form: (race, 0).
const SET_RACE: u32 = 0x0060_b240;
/// sets the rotation matrix of a node: (matrix).
const SET_ROTATE: u32 = 0x0043_fa80;
/// (tile, trait id, string, flag).
const SET_STRING_TRAIT: u32 = 0x00a0_1350;
/// (tile, trait id, integer).
const SET_TRAIT_FROM_INT: u32 = 0x0070_0320;
/// sets a node's translation: (x, y, z).
const SET_TRANSLATE: u32 = 0x004b_c1f0;
/// sets a node's translation: (point).
const SET_TRANSLATE_POINT: u32 = 0x00a5_9c60;
/// cdecl (0, 0): the number of face shapes.
const SHAPE_COUNT: u32 = 0x0065_21b0;
/// shows a message box, cdecl, 11 words.
const SHOW_MESSAGE_BOX: u32 = 0x0070_3e80;
/// empties a `BSSimpleArray`: (flag).
const SIMPLE_ARRAY_CLEAR: u32 = 0x0084_54f0;
/// whether the array is empty.
const SIMPLE_ARRAY_IS_EMPTY: u32 = 0x0076_b610;
/// called on the menu while a slider is held.
const SLIDER_PRESS: u32 = 0x007b_4930;
/// `RaceSexMenu::SliderRelease` (engine map).
const SLIDER_RELEASE: u32 = 0x007b_4b50;
/// `BSSoundHandle` constructor.
const SOUND_HANDLE_CONSTRUCT: u32 = 0x0041_a250;
/// copies a handle (this, other); returns this.
const SOUND_HANDLE_COPY: u32 = 0x0041_8900;
/// `BSSoundHandle::IsPlaying` (engine map).
const SOUND_HANDLE_IS_PLAYING: u32 = 0x00ad_8930;
/// `BSSoundHandle::Pause` (engine map).
const SOUND_HANDLE_PAUSE: u32 = 0x00ad_88b0;
/// `BSSoundHandle::Play` (engine map): (0).
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
/// `BSSoundHandle::Release` (engine map).
const SOUND_HANDLE_RELEASE: u32 = 0x00ad_8d10;
/// stores a byte at `+4` of a handle.
const SOUND_HANDLE_SET_BYTE: u32 = 0x004f_15a0;
/// `BSSoundHandle::Stop` (engine map).
const SOUND_HANDLE_STOP: u32 = 0x00ad_88f0;
/// constructs an 8-byte `BSStringT<char>`.
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// the word at the start of its object (the C string of a `BSStringT`).
const STRING_C_STR: u32 = 0x0055_9450;
/// destroys a `BSStringT<char>`.
const STRING_DESTROY: u32 = 0x0040_37d0;
/// `sprintf` into a `BSStringT`, cdecl (string, format, arguments...).
const STRING_FORMAT: u32 = 0x0040_6f60;
/// cdecl (string): its length.
const STRING_LENGTH: u32 = 0x0044_a670;
/// `BSStringT::Set` (this, text, 0).
const STRING_SET: u32 = 0x0040_37f0;
/// `_strlen`.
const STRLEN: u32 = 0x00ec_6130;
/// `Tile::GetChildByID` (engine map): (id).
const TILE_GET_CHILD_BY_ID: u32 = 0x00a0_3eb0;
/// `Tile::GetMenu` (engine map).
const TILE_GET_MENU: u32 = 0x00a0_3c90;
/// `Tile::GetMenuByClass` (engine map), cdecl (class id).
const TILE_GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// `Tile::GetValue` (engine map): (trait id).
const TILE_GET_VALUE: u32 = 0x00a0_0f30;
/// `Tile::IsTrue` (engine map): (trait id).
const TILE_IS_TRUE: u32 = 0x00a0_1230;
/// `Tile::IsVisible` (engine map).
const TILE_IS_VISIBLE: u32 = 0x00a0_40a0;
/// the word of the object at `+0x2C`.
const TILE_NAME_STRING: u32 = 0x0056_c7f0;
/// the tile reader of the interface.
const TILE_READER: u32 = 0x0058_6150;
/// `Tile::ReadFile` (engine map): (path).
const TILE_READ_FILE: u32 = 0x00a0_1b00;
/// `Tile::UpdateAll` (engine map), cdecl (0).
const TILE_UPDATE_ALL: u32 = 0x00a0_4200;
/// `Tile::Value::SetFloat` (engine map): (float, flag).
const TILE_VALUE_SET_FLOAT: u32 = 0x00a0_a270;
/// `NiMatrix3::ToEulerAnglesXYZ` (engine map): (x, y, z outputs).
const TO_EULER_ANGLES: u32 = 0x00a5_92c0;
/// `InterfaceManager::TutorialManager::ShowMessage` (engine map).
const TUTORIAL_SHOW_MESSAGE: u32 = 0x0071_8630;
/// `Actor::UnEquipObject` (engine map).
const UNEQUIP_OBJECT: u32 = 0x0088_d7d0;
/// `ShadowSceneNode::UpdateObjectLighting` (engine map).
const UPDATE_OBJECT_LIGHTING: u32 = 0x00b5_d9f0;
/// `StatsMenu::UpdatePlayerName` (engine map).
const UPDATE_PLAYER_NAME: u32 = 0x007d_f4e0;
/// `NiAVObject::UpdateProperties` (engine map).
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// `Tile::UpdateTile` (engine map): (0).
const UPDATE_TILE: u32 = 0x00a0_4640;
/// changes the case of a string in place.
const STRING_UPPER_CASE: u32 = 0x004a_fad0;
/// changes the case of a caption string in place.
const CAPTION_UPPER_CASE: u32 = 0x007b_52a0;
/// `__vec_ctor`: (array, size, count, constructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// thiscall on the interface: the truncated wheel movement.
const WHEEL_DELTA: u32 = 0x0070_ec70;
/// returns the word at `+0x28` of its object.
const WORD_AT_0X28: u32 = 0x0045_cd60;
/// returns the word at `+8` of its object.
const WORD_AT_EIGHT: u32 = 0x0044_ddc0;
/// a getter on the animation object.
const ANIMATION_OBJECT: u32 = 0x0049_6940;
/// scalar deleting destructor (flag 1) of a worn item record.
const WORN_ITEM_DELETE: u32 = 0x0044_59e0;
/// returns the word at `+4` of its object.
const WORD_AT_FOUR: u32 = 0x0072_6070;
/// `RaceSexMenu::ChooseDefaultPreset` (engine map).
const CHOOSE_DEFAULT_PRESET: u32 = 0x007b_5180;
/// `RaceSexMenu::Randomize` (engine map).
const RANDOMIZE: u32 = 0x007b_51f0;
/// Same address as `OPTION_LIST_CURRENT`: for the player it returns the base form (the word at `+0x20`).
const PLAYER_BASE_FORM: u32 = 0x007a_f430;
/// `RaceSexMenu::pMe` (Xbox PDB static): the menu singleton.
const RACE_SEX_MENU_ME: u32 = 0x011d_a634;
/// the vtable of `RaceSexMenu`.
const RACE_SEX_MENU_VTABLE: u32 = 0x0107_5974;
/// the vtable of the rendered menu `Create` allocates.
const RENDERED_MENU_VTABLE: u32 = 0x0107_59ec;
/// the class id of the menu (`GetClass`).
const RACE_SEX_MENU_CLASS: u32 = 0x0000_040c;
/// the player character pointer.
const PLAYER: u32 = 0x011d_ea3c;
/// a pointer to the object owning the form lists the option lists are filled from.
const MARKER_OWNER: u32 = 0x011c_3f2c;
/// a pointer to the input owner.
const INPUT_GLOBAL: u32 = 0x011d_ea0c;
/// the IO manager pointer.
const IO_MANAGER: u32 = 0x0120_2d98;
/// the object `TES::CreateTextureImage` is called on.
const TEXTURE_LOADER: u32 = 0x011d_ea10;
/// the setting that switches the rendered 3D menu on.
const RENDERED_MENU_SETTING_OBJECT: u32 = 0x011d_b620;
/// three words copied into the zoom bound by the constructor.
const ZOOM_BOUND_DEFAULT: u32 = 0x011f_426c;
/// a 3x3 matrix copied by `RotatePlayer` (9 words).
const IDENTITY_MATRIX: u32 = 0x011f_4248;
/// a 3x3 matrix copied by `007b25a0` (9 words).
const ROTATION_MATRIX: u32 = 0x011a_9448;
/// the table `00653270` searches.
const PALETTE_TABLE: u32 = 0x011f_4a04;
/// 20 words: the worn items `Create` took off, for `Close`.
const SAVED_WORN_ITEMS: u32 = 0x011d_a638;
/// a byte: the player flag `Create` saved.
const SAVED_PLAYER_FLAG: u32 = 0x011d_a688;
/// a byte `007b25a0` sets while it rebuilds the head.
const FACE_REBUILD_FLAG: u32 = 0x011c_5cb4;
/// `DoIdle`'s function-local static: the time of the last call.
const LAST_TIME: u32 = 0x011d_a740;
/// the initialization guard of `LAST_TIME` (bit 0).
const LAST_TIME_GUARD: u32 = 0x011d_a744;
/// `DoStillDown`'s function-local static float.
const ROTATE_WIDTH: u32 = 0x011d_a738;
/// the initialization guard of `ROTATE_WIDTH` (bit 0).
const ROTATE_WIDTH_GUARD: u32 = 0x011d_a73c;
/// a stick limit.
const AXIS_LIMIT_POSITIVE: u32 = 0x011a_07b0;
/// a stick limit.
const AXIS_LIMIT_NEGATIVE: u32 = 0x011a_07ac;
/// a static `BSStringT` (the Xbox PDB has strNextCaps, strDoneCaps and strBackCaps; which is which is unconfirmed).
const CAPTION_A: u32 = 0x011d_a70c;
/// a static `BSStringT`, see `CAPTION_A`.
const CAPTION_B: u32 = 0x011d_a700;
/// a static `BSStringT`, see `CAPTION_A`.
const CAPTION_C: u32 = 0x011d_a6f4;
/// the setting whose text `Create` copies into `CAPTION_A`.
const SETTING_FOR_CAPTION_A: u32 = 0x011d_35c4;
/// the setting for `CAPTION_B`.
const SETTING_FOR_CAPTION_B: u32 = 0x011d_34b0;
/// the setting for `CAPTION_C`.
const SETTING_FOR_CAPTION_C: u32 = 0x011d_1fec;
/// a caption setting of the confirmation popup.
const SETTING_POPUP_A: u32 = 0x011d_34f8;
/// a caption setting of the confirmation popup.
const SETTING_POPUP_B: u32 = 0x011d_3684;
/// the text setting of the randomize confirmation.
const SETTING_RANDOMIZE_TEXT: u32 = 0x011d_3288;
/// the text setting of the option with id 0x15.
const SETTING_ENTRY_15: u32 = 0x011d_37ec;
/// the text setting of the option with id 0x16.
const SETTING_ENTRY_16: u32 = 0x011d_2d3c;
/// the text setting of the option with id 0x19.
const SETTING_ENTRY_19: u32 = 0x011d_3d48;
/// the text setting of the slider with id 0x18.
const SETTING_SLIDER_18: u32 = 0x011d_3ff4;
/// the text setting of the slider with id 0x1a.
const SETTING_SLIDER_1A: u32 = 0x011d_4084;
/// the text setting of the slider with id 0x1d.
const SETTING_SLIDER_1D: u32 = 0x011d_3ff4;
/// the text setting of the slider with id 0x1e.
const SETTING_SLIDER_1E: u32 = 0x011d_2a84;
/// the text setting of the slider with id 0x1f.
const SETTING_SLIDER_1F: u32 = 0x011d_25a4;
/// the text setting of the slider with id 0x20.
const SETTING_SLIDER_20: u32 = 0x011d_2d48;
/// the float setting for the starting zoom (`fStartingZoom`, Xbox PDB static).
const SETTING_STARTING_ZOOM: u32 = 0x011d_a6cc;
/// a text setting.
const SETTING_2058: u32 = 0x011d_2058;
/// a text setting.
const SETTING_2250: u32 = 0x011d_2250;
/// a text setting.
const SETTING_22D4: u32 = 0x011d_22d4;
/// a text setting.
const SETTING_23A0: u32 = 0x011d_23a0;
/// a text setting.
const SETTING_26D0: u32 = 0x011d_26d0;
/// a text setting.
const SETTING_31C8: u32 = 0x011d_31c8;
/// a text setting.
const SETTING_3528: u32 = 0x011d_3528;
/// a text setting.
const SETTING_36E4: u32 = 0x011d_36e4;
/// a text setting.
const SETTING_4108: u32 = 0x011d_4108;
/// a text setting.
const SETTING_43D8: u32 = 0x011d_43d8;
/// a text setting.
const SETTING_454C: u32 = 0x011d_454c;
/// a text setting.
const SETTING_4A74: u32 = 0x011d_4a74;
/// a double the menu tile's stacking type is compared with.
const KIND_ONE_DOUBLE: u32 = 0x0107_04f0;
/// a double the menu tile's stacking type is compared with.
const KIND_TWO_DOUBLE: u32 = 0x0107_04e8;
/// a double compared with the pointer movement and angles.
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// a double `007b3900` compares with.
const ONE_DOUBLE: u32 = 0x0101_2070;
/// a double the `_selected` trait is compared with.
const SELECTED_VALUE: u32 = 0x0101_1590;
/// a double used as the scale of the face sliders.
const TENTH_DIVISOR: u32 = 0x0102_0758;
/// a double: the float 2 pi widened.
const TWO_PI_DOUBLE: u32 = 0x0101_ff48;
/// a double divisor of the stick rotation.
const MILLISECONDS_DOUBLE: u32 = 0x0101_7b70;
/// a double factor of the stick rotation.
const ROTATE_RATE_DOUBLE: u32 = 0x0107_5af0;
/// a double factor of the zoom acceleration.
const ZOOM_RATE_DOUBLE: u32 = 0x0107_5af8;
/// a double: the divisor of a colour channel.
const COLOR_DIVISOR: u32 = 0x0101_e568;
/// a float: the step of the sliders.
const SLIDER_STEP_QUARTER: u32 = 0x0101_622c;
/// a float: the step of the colour sliders.
const SLIDER_STEP_EIGHTH: u32 = 0x0105_559c;
/// a float: the zoom step of the wheel (forward).
const WHEEL_ZOOM_STEP: u32 = 0x0107_5aec;
/// a float: the zoom step of the wheel (backward).
const WHEEL_ZOOM_STEP_BACK: u32 = 0x0105_559c;
/// `Data\Menus\CharGen\race_sex_menu.xml`.
const MENU_FILE: u32 = 0x0107_59bc;
/// the string `<`.
const GLYPH_LESS: u32 = 0x0106_3ab4;
/// the string `>`.
const GLYPH_GREATER: u32 = 0x0106_92d0;
/// `OBJComputerTerminal01LP`.
const SOUND_COMPUTER_TERMINAL: u32 = 0x0107_5958;
/// `QSTBirthRoomLP`.
const SOUND_BIRTH_ROOM: u32 = 0x0107_5948;
/// `UIFaceGenZoomLP`.
const SOUND_FACE_GEN_ZOOM: u32 = 0x0107_5938;
/// `UIFaceGenRotateLP`.
const SOUND_FACE_GEN_ROTATE: u32 = 0x0107_5924;
/// `UIMenuOK`.
const SOUND_MENU_OK: u32 = 0x0106_f380;
/// `UIPipBoyMode`.
const SOUND_PIPBOY_MODE: u32 = 0x0107_5b00;
/// `%s %s?`.
const CONFIRMATION_FORMAT: u32 = 0x0107_4e34;
/// `Terminals\BarberInterface01.NIF`.
const PROJECTOR_BARBER: u32 = 0x0107_5aac;
/// `Terminals\PlasticSurgeryInterface01.NIF`.
const PROJECTOR_SURGERY: u32 = 0x0107_5a74;
/// `Terminals\NV_reflectron_UI.NIF`.
const PROJECTOR_REFLECTRON: u32 = 0x0107_5a4c;
/// a one-character string (a space).
const SPACE_STRING: u32 = 0x0102_0770;
/// `_selected`.
const SELECTED_TRAIT_NAME: u32 = 0x0107_1b64;
/// `_enabled`.
const ENABLED_TRAIT_NAME: u32 = 0x0107_0244;
/// `_position`.
const POSITION_TRAIT_NAME: u32 = 0x0107_5b50;
/// `_is_header`.
const HEADER_TRAIT_NAME: u32 = 0x0107_5b5c;
/// `RSM_list_item_template`.
const LIST_ITEM_TEMPLATE: u32 = 0x0107_5b10;
/// `%s %c`.
const CAPTION_FORMAT: u32 = 0x0107_5b28;
/// `iHairColor%02d`.
const FORMAT_HAIR_COLOR_INT: u32 = 0x0107_5b40;
/// `sHairColor%d`.
const FORMAT_HAIR_COLOR_NAME: u32 = 0x0107_5b30;
/// `FaceGenFace`.
const FACE_GEN_FACE: u32 = 0x0104_b3b4;
/// `FaceGenHair`.
const FACE_GEN_HAIR: u32 = 0x0104_aad0;
/// `FaceGenAccessory`.
const FACE_GEN_ACCESSORY: u32 = 0x0106_57e4;
/// `FaceGenEyeLeft`.
const FACE_GEN_EYE_LEFT: u32 = 0x0104_aac0;
/// `FaceGenEyeRight`.
const FACE_GEN_EYE_RIGHT: u32 = 0x0104_aab0;
/// `Textures\%s`.
const TEXTURE_PATH_FORMAT: u32 = 0x0106_57f8;
/// `Textures\Characters\Eyes\EyeDefault.dds`.
const DEFAULT_EYE_TEXTURE: u32 = 0x0106_5804;
/// `RaceSexMenu::Close`, run when the popup is confirmed.
const CLOSE_CALLBACK: u32 = 0x007a_da40;
/// the RTTI type descriptor of `TESBoundObject`.
const TYPE_BOUND_OBJECT: u32 = 0x0118_3108;
/// the RTTI type descriptor of `TESNPC`.
const TYPE_NPC: u32 = 0x0118_3a1c;
/// trait id.
const TRAIT_STACKING_TYPE: u32 = 0x0000_0fa7;
/// trait id.
const TRAIT_DEPTH: u32 = 0x0000_0fad;
/// trait id (string).
const TRAIT_STRING: u32 = 0x0000_0fc4;
/// trait id.
const TRAIT_VISIBLE: u32 = 0x0000_0fa3;
/// trait id.
const TRAIT_WIDTH: u32 = 0x0000_0fb1;
/// trait id (the list number on the menu tile).
const TRAIT_0X1004: u32 = 0x0000_1004;
/// trait id.
const TRAIT_0X1772: u32 = 0x0000_1772;

// Added with the `MenuOptionList` functions (007b3b80 on). The option lists
// are `NiTPointerList`s; their small helpers live in other units.
/// The node after `node` (`+0`), or 0 for no node (`007b52d0`).
const LIST_NODE_NEXT: u32 = 0x007b_52d0;
/// The node before `node` (`+4`), or 0 for no node (`007b5300`).
const LIST_NODE_PREVIOUS: u32 = 0x007b_5300;
/// The address of a node's item (`node + 8`, `006317a0`).
const LIST_NODE_DATA: u32 = 0x0063_17a0;
/// The address of the first node's item (`0073ac90`).
const LIST_HEAD_DATA: u32 = 0x0073_ac90;
/// The address of the last node's item (`007b5370`).
const LIST_TAIL_DATA: u32 = 0x007b_5370;
/// The node holding the item a pointer points at, searched from a node or
/// the head (`0049c680`).
const LIST_FIND_NODE: u32 = 0x0049_c680;
/// Puts an item in a new node (`0049fa80`) and links it through
/// `00559a70` (`004ed8c0`).
const LIST_INSERT_ITEM: u32 = 0x004e_d8c0;
/// `NiTPointerListBase<..>::InsertBefore` (map name, folded): a new node
/// for the item before a node (`007b5330`).
const LIST_INSERT_BEFORE: u32 = 0x007b_5330;
/// Removes the node holding the item a pointer points at (`0057c730`).
const LIST_REMOVE_ITEM: u32 = 0x0057_c730;
/// `NiTPointerListBase<..>::RemoveAll` (map name, folded) (`004ed900`).
const LIST_REMOVE_ALL: u32 = 0x004e_d900;
/// `Tile::GetString` (map name): a tile's string trait.
const TILE_GET_STRING: u32 = 0x00a0_11f0;
/// Constructs a `BSStringT` from a text (`0040c0e0`).
const STRING_FROM_TEXT: u32 = 0x0040_c0e0;
/// Compares two texts (`00469880`, which calls `_mbscmp`); 0 when equal.
const STRING_COMPARE: u32 = 0x0046_9880;
/// `Tile::PlayTileSound` (map name).
const PLAY_TILE_SOUND: u32 = 0x00a0_b110;
/// `BSFaceGenManager::InitFaceGenCoord` (map name).
const INIT_FACE_GEN_COORD: u32 = 0x0065_21f0;
/// Reads one float of a face coordinate array (`00652440`, cdecl: array,
/// two indices).
const FACE_COORD_GET: u32 = 0x0065_2440;
/// Writes one float of a face coordinate array (`00652470`, cdecl: array,
/// two indices, the float).
const FACE_COORD_SET: u32 = 0x0065_2470;
/// A `bsfacegenmanager.cpp` check of a face coordinate array against a
/// second value (`00652900`, cdecl, answers a byte).
const FACE_COORD_CHECK_652900: u32 = 0x0065_2900;
/// `BSFaceGenManager::CopyFaceGenCoord` (map name).
const COPY_FACE_GEN_COORD: u32 = 0x0065_29a0;
/// The address of entry `index` of an array whose data pointer is at
/// `+4` (`00877a30`).
const ARRAY_ENTRY: u32 = 0x0087_7a30;
/// A `TESNPC` method (`tesnpc.cpp`) taking a preset entry (`00603790`).
const NPC_METHOD_603790: u32 = 0x0060_3790;
/// A method of the player base form that `Randomize` calls with
/// `(1, 0, 1)` (`00601830`).
const NPC_METHOD_601830: u32 = 0x0060_1830;
/// `RSM_slider_option_template`.
const SLIDER_TEMPLATE: u32 = 0x0107_5b68;
/// `_top_bound`.
const TOP_BOUND_TRAIT_NAME: u32 = 0x0107_5b84;
/// a float (2.0 in the exe): the trait value of the highlighted option.
const HIGHLIGHT_VALUE: u32 = 0x0101_62c0;
/// a double (5.55 in the exe): the scale of the preset slider value.
const SHAPE_SCALE_DOUBLE: u32 = 0x0107_5ba0;
/// a double (5.6 in the exe): the offset of the preset slider value.
const SHAPE_OFFSET_DOUBLE: u32 = 0x0107_5b98;
/// a double (15.0 in the exe): the lower bound of the two face values.
const LOWER_BOUND_DOUBLE: u32 = 0x0101_5a38;
/// a double (65.0 in the exe): the upper bound of the two face values.
const UPPER_BOUND_DOUBLE: u32 = 0x0107_5b90;
/// a float (15.0 in the exe): the lower bound stored.
const LOWER_BOUND: u32 = 0x0101_e580;
/// a float (65.0 in the exe): the upper bound stored.
const UPPER_BOUND: u32 = 0x0102_243c;
/// The settings whose text captions the four numbered lists in
/// `_Create` (the first four).
const NUMBERED_SETTINGS: [u32; 4] = [0x011d_1f38, 0x011d_20dc, 0x011d_2058, 0x011d_43d8];
/// `"1. %s"`, `"2. %s"`, `"3. %s"` and `"4. %s"`.
const NUMBERED_FORMATS: [u32; 4] = [0x0107_5ae4, 0x0107_5adc, 0x0107_5ad4, 0x0107_5acc];
/// The settings whose text captions the lists 4 to 19.
const LIST_CAPTION_SETTINGS: [u32; 16] = [
    0x011d_36e4,
    0x011d_22d4,
    0x011d_4a74,
    0x011d_23a0,
    0x011d_31c8,
    0x011d_26d0,
    0x011d_3eb0,
    0x011d_4480,
    0x011d_4318,
    0x011d_3084,
    0x011d_4d20,
    0x011d_4180,
    0x011d_3078,
    0x011d_50e0,
    0x011d_3618,
    0x011d_2250,
];

layout! {
    /// `RaceSexMenu` (Xbox PDB) on PC: a `Menu` (0x28 bytes) followed by the
    /// fields this unit uses, at their PC offsets. The object is 0x1A4
    /// bytes. The PDB names are used where the PC code agrees; see the
    /// notes in the module documentation.
    pub struct RaceSexMenu: 0x1a4 {
        /// `Menu::pMenu` (Xbox PDB): the menu tile.
        0x04 pMenu: u32,
        /// `pOptionListsA` (Xbox PDB): 20 `MenuOptionList *`, the first one.
        0x28 pOptionListsA: u32,
        /// `pTiles` (Xbox PDB): six tile pointers, the first one.
        0x78 pTiles: u32,
        /// `strProjectorFile` (Xbox PDB): a `BSStringT`.
        0x90 strProjectorFile: u32,
        /// `strConfirmationMsg` (Xbox PDB): a `BSStringT`.
        0x98 strConfirmationMsg: u32,
        /// `ptZoomBound` (Xbox PDB): two `NiPoint3`, the first.
        0xA0 ptZoomBound: f32,
        /// `fDragStartX` (Xbox PDB): the pointer position a drag started at.
        0xB8 fDragStartX: f32,
        /// `iDragStartSliderVal` (Xbox PDB).
        0xBC iDragStartSliderVal: i32,
        /// `pSliderElem` (Xbox PDB): the dragged tile.
        0xC0 pSliderElem: u32,
        /// `pSliderOpt` (Xbox PDB): the dragged option.
        0xC4 pSliderOpt: u32,
        /// The zoom target the wheel sets (the PDB's `fZoomVelocity` slot).
        0xC8 fZoomTarget: f32,
        /// The zoom velocity (the PDB's `fZoomPct` slot).
        0xCC fZoomVelocity: f32,
        /// The current zoom, 0 to 1 (a PC-only float in front of
        /// `spPlayerNode`).
        0xD0 fZoomPct: f32,
        /// `spPlayerNode` (Xbox PDB): `NiPointer<NiAVObject>`.
        0xD4 spPlayerNode: u32,
        /// `pPlayerBase` (Xbox PDB): the player's `TESNPC`.
        0xD8 pPlayerBase: u32,
        /// `uiFlags` (Xbox PDB).
        0xEC uiFlags: u32,
        /// `eLastList` (Xbox PDB).
        0x120 eLastList: i32,
    }

    /// `RaceSexMenu::MenuOption` (Xbox PDB), 0xC bytes: `+0` the option's
    /// tile, `+4` its id, `+8` its data word.
    pub struct MenuOption: 0xc {
        /// The option's tile.
        0x00 pTile: u32,
        /// The option's id (`00726070` reads it).
        0x04 iId: u32,
        /// The option's data word (`0044ddc0` reads it; `00403550` sets it).
        0x08 pData: u32,
    }

    /// `RaceSexMenu::MenuOptionList` (Xbox PDB), 0x24 bytes on PC.
    pub struct MenuOptionList: 0x24 {
        /// `m_pkHead` (Xbox PDB, `NiTPointerList` base): the first node
        /// (`NiTListItem`: `+0` next, `+4` previous, `+8` the option).
        0x00 m_pkHead: u32,
        /// `m_pkTail` (Xbox PDB): the last node.
        0x04 m_pkTail: u32,
        /// The list that `Back` goes to (`0084e3a0`); 0x14 for none
        /// (`eBackLink`, Xbox PDB).
        0x0C iBackTarget: u32,
        /// The list after this one (`0044edb0`); 0x14 for none
        /// (`eForwardLink`, Xbox PDB).
        0x10 iParentTarget: u32,
        /// The rendered template tile (`pHeader`, Xbox PDB).
        0x14 pTemplateTile: u32,
        /// `fScrollDelta` (Xbox PDB): the largest option height, the step
        /// `ScrollBy` moves the options by.
        0x18 fScrollDelta: f32,
        /// The callback `0050f9a0` stores (`pfSelectionTest`, Xbox PDB).
        0x1C pCallback: u32,
        /// `pHighlighted` (Xbox PDB): the highlighted option.
        0x20 pHighlighted: u32,
    }
}

/// `FLD dword [address]` then `FSTP dword`: a float copied through the x87
/// stack, which turns a signalling NaN into the quiet one.
fn quiet_f32(value: f32) -> f32 {
    let bits = value.to_bits();
    if bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0 {
        f32::from_bits(bits | 0x0040_0000)
    } else {
        value
    }
}

/// The float at `address` as `FLD`/`FSTP` copies it.
fn x87_copy_f32(e: &Engine, address: u32) -> f32 {
    quiet_f32(f32::from_bits(e.global::<u32>(address)))
}

/// `FISTP qword` with the rounding mode set to truncation, low 32 bits
/// (NaN and out of range answer the integer indefinite, whose low word is 0).
fn fistp_truncate(value: f64) -> i32 {
    if value.is_nan()
        || !(-9_223_372_036_854_775_808.0..9_223_372_036_854_775_808.0).contains(&value)
    {
        0
    } else {
        (value.trunc() as i64) as i32
    }
}

/// The word at `+4` of `object` (`00726070`: the id of an option, the next
/// node of a list node).
fn word_at_four(e: &mut Engine, object: u32) -> u32 {
    e.call(WORD_AT_FOUR, &args![object]).u32()
}

/// `pOptionListsA[index]` of the menu at `base` (the index comes from the
/// game, so the address arithmetic wraps).
fn option_list(e: &Engine, base: u32, index: u32) -> u32 {
    e.mem
        .u32(base.wrapping_add(0x28).wrapping_add(index.wrapping_mul(4)))
}

/// Copies `words` words (`REP MOVSD`).
fn copy_words(e: &mut Engine, from: u32, to: u32, words: u32) {
    for word in 0..words {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// Adds the list at `sub_slot` as a sub list of the list at `list_slot`
/// (offsets in the menu), captioned by the text of `setting`.
fn add_sub_list(e: &mut Engine, base: u32, list_slot: u32, setting: u32, sub_slot: u32) {
    let sub_list = e.mem.u32(base + sub_slot);
    let text = e.call(SETTING_TEXT, &args![setting]).u32();
    let list = e.mem.u32(base + list_slot);
    e.call(OPTION_LIST_ADD_SUB_LIST, &args![list, text, sub_list]);
}

/// Sets the six floats of the zoom bounds (`+0xA0` to `+0xB4`) from the
/// float constants at `addresses`, copying each through the x87 stack.
fn set_zoom_bounds(e: &mut Engine, base: u32, addresses: [u32; 6]) {
    for (index, address) in addresses.iter().enumerate() {
        let value = x87_copy_f32(e, *address);
        e.mem.set_f32(base + 0xa0 + 4 * index as u32, value);
    }
}

// Translated from 007ac1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::RaceSexMenu` (Xbox PDB): constructs the menu in place and
/// returns it. Runs the `Menu` base constructor, sets the vtable, constructs
/// the two strings, the two zoom points, the player-node pointer, the preset
/// head array, the four sound handles and the four face coordinate matrices,
/// clears the option list and tile tables, sets the zoom bounds from the
/// three words at `011f426c` and fetches the four looping sounds
/// (`OBJComputerTerminal01LP`, `QSTBirthRoomLP`, `UIFaceGenZoomLP`,
/// `UIFaceGenRotateLP`) into the sound handle slots.
///
/// The compiler's exception-unwinding frame is not translated.
// Translation incomplete: 007ac1f0 - the C++ exception-unwinding frame (FS:[0]) that destroys the sound handle temporaries is not translated
pub fn race_sex_menu_race_sex_menu(e: &mut Engine, this: Ptr<RaceSexMenu>) -> Ptr<RaceSexMenu> {
    let base = this.addr();
    e.call(MENU_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, RACE_SEX_MENU_VTABLE);
    e.call(STRING_CONSTRUCT, &args![base + 0x90]);
    e.call(STRING_CONSTRUCT, &args![base + 0x98]);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![base + 0xa0, 0xcu32, 2u32, EMPTY_MEMBER_RETURNS_THIS],
    );
    e.call(NI_POINTER_CONSTRUCT, &args![base + 0xd4, 0u32]);
    e.call(PRESET_HEADS_CONSTRUCT, &args![base + 0xdc]);
    e.call(
        EH_VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            base + 0xf0,
            0xcu32,
            4u32,
            SOUND_HANDLE_CONSTRUCT,
            EMPTY_MEMBER
        ],
    );
    e.call(
        EH_VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            base + 0x124,
            0x20u32,
            4u32,
            FACE_COORD_CONSTRUCT,
            FACE_COORD_DESTRUCT
        ],
    );
    e.call(MEMSET, &args![base + 0x78, 0u32, 0x18u32]);
    e.call(MEMSET, &args![base + 0x28, 0u32, 0x50u32]);
    e.mem.set_f32(base + 0xb8, 0.0);
    e.mem.set_u32(base + 0xbc, 0);
    e.mem.set_u32(base + 0xec, 3);
    e.mem.set_f32(base + 0xc8, 0.0);
    e.mem.set_f32(base + 0xd0, 0.0);
    e.mem.set_f32(base + 0xcc, 0.0);
    // The zoom bound: the three words at `011f426c` go to +0xAC and are
    // copied to +0xA0.
    for word in 0..3u32 {
        let value: u32 = e.global(ZOOM_BOUND_DEFAULT + 4 * word);
        e.mem.set_u32(base + 0xac + 4 * word, value);
    }
    for word in 0..3u32 {
        let value = e.mem.u32(base + 0xac + 4 * word);
        e.mem.set_u32(base + 0xa0 + 4 * word, value);
    }
    let audio = e.call(AUDIO_MANAGER_INSTANCE, &args![]).u32();
    for (slot, name) in [
        (0xf0u32, SOUND_COMPUTER_TERMINAL),
        (0xfc, SOUND_BIRTH_ROOM),
        (0x108, SOUND_FACE_GEN_ZOOM),
        (0x114, SOUND_FACE_GEN_ROTATE),
    ] {
        e.with_stack(0xc, |e, handle| {
            let found = e
                .call(
                    GET_SOUND_HANDLE_BY_NAME,
                    &args![audio, handle, name, 0x31u32],
                )
                .u32();
            let slot_handle = e.call(SOUND_HANDLE_COPY, &args![base + slot, found]).u32();
            e.call(SOUND_HANDLE_SET_BYTE, &args![slot_handle, 1u32]);
            e.call(EMPTY_MEMBER, &args![handle]);
        });
    }
    e.mem.set_u32(base + 0xc4, 0);
    e.mem.set_u32(base + 0xc0, 0);
    e.mem.set_u32(base + 0x120, 0x14);
    this
}

// Translated from 007ac4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::GetClass` (Xbox PDB): the menu class id, `0x40c`.
pub fn race_sex_menu_get_class(_e: &mut Engine, _this: Ptr<RaceSexMenu>) -> u32 {
    RACE_SEX_MENU_CLASS
}

// Translated from 007ac500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::AttachTileByID` (Xbox PDB): stores the tile in `pTiles`
/// (+0x78) for ids 0 to 5 (signed compare); other ids do nothing.
pub fn race_sex_menu_attach_tile_by_id(e: &mut Engine, this: Ptr<RaceSexMenu>, id: i32, tile: u32) {
    if (0..6).contains(&id) {
        e.mem.set_u32(this.addr() + 0x78 + 4 * id as u32, tile);
    }
}

// Translated from 007ac530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor and, when bit 0 of the flags is set, frees the object.
pub fn race_sex_menu_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<RaceSexMenu>,
    flags: u32,
) -> Ptr<RaceSexMenu> {
    race_sex_menu_destructor(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 007ac560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::~RaceSexMenu` (Xbox PDB): leaves the rendered menu, makes
/// the player's race drop its EGT data, deletes the 20 option lists, empties
/// the preset head array, stops and releases the four sounds and destroys
/// the members and the `Menu` base in reverse order.
///
/// The compiler's exception-unwinding frame is not translated.
// Translation incomplete: 007ac560 - the C++ exception-unwinding frame (FS:[0]) that unwinds the partly destroyed members is not translated
pub fn race_sex_menu_destructor(e: &mut Engine, this: Ptr<RaceSexMenu>) {
    let base = this.addr();
    e.mem.set_u32(base, RACE_SEX_MENU_VTABLE);
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    e.call(ENTER_RENDERED_MENU, &args![interface, 0u32]);
    let player: u32 = e.global(PLAYER);
    let race = e.vcall(player, 0x37c, &args![]).u32();
    e.call(KILL_EGT_DATA, &args![race]);
    for index in 0..0x14u32 {
        let list = e.mem.u32(base + 0x28 + 4 * index);
        if list != 0 {
            fn_007ac700(e, Ptr::new(list), 1);
        }
    }
    e.call(SIMPLE_ARRAY_CLEAR, &args![base + 0xdc, 1u32]);
    for index in 0..4u32 {
        let handle = base + 0xf0 + index * 0xc;
        e.call(SOUND_HANDLE_STOP, &args![handle]);
        e.call(SOUND_HANDLE_RELEASE, &args![handle]);
    }
    e.call(
        EH_VECTOR_DESTRUCTOR_ITERATOR,
        &args![base + 0x124, 0x20u32, 4u32, FACE_COORD_DESTRUCT],
    );
    e.call(
        EH_VECTOR_DESTRUCTOR_ITERATOR,
        &args![base + 0xf0, 0xcu32, 4u32, EMPTY_MEMBER],
    );
    e.call(PRESET_HEADS_DESTRUCT, &args![base + 0xdc]);
    e.call(NI_POINTER_DESTRUCT, &args![base + 0xd4]);
    e.call(STRING_DESTROY, &args![base + 0x98]);
    e.call(STRING_DESTROY, &args![base + 0x90]);
    e.call(MENU_DESTRUCT, &args![this]);
}

// Translated from 007ac700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::MenuOptionList::_scalar_deleting_destructor_` (the
/// compiler-generated deleting destructor of the option list class, no
/// Xbox PDB name in the map): runs the list destructor (`007b3af0`) and,
/// when bit 0 of the flags is set, frees the object.
pub fn fn_007ac700(e: &mut Engine, this: Ptr<MenuOptionList>, flags: u32) -> Ptr<MenuOptionList> {
    fn_007b3af0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 007ac730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::Create` (Xbox PDB), `cdecl`: returns the existing menu tile
/// when the class is already open. Otherwise reads
/// `Data\Menus\CharGen\race_sex_menu.xml`; when the file loads, makes the
/// menu singleton (`pMe`, `011da634`), fills the three caption strings when
/// empty (upper-cased unless the texture language is 3), remembers the
/// player's `8a6970` flag, takes off the pieces of clothing in the slots 0,
/// 1, 5, 7 to 11, 14 and 15 (keeping them in `011da638` for `Close`), runs
/// `_Create`, and, when the setting at `011db620` is on, builds the rendered
/// menu (`0x170` bytes, the constructor at `007ac9f0`) and enters it.
/// Returns the menu tile (0 when the file did not load).
///
/// The compiler's exception-unwinding frame is not translated.
// Translation incomplete: 007ac730 - the C++ exception-unwinding frame (FS:[0]) that frees the allocation when the rendered menu constructor throws is not translated
pub fn race_sex_menu_create(e: &mut Engine, mode: u32) -> u32 {
    let class = fn_007ac9c0(e);
    let existing = e.call(TILE_GET_MENU_BY_CLASS, &args![class]).u32();
    if existing != 0 {
        return existing;
    }
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    let reader = e.call(TILE_READER, &args![interface]).u32();
    let root = e.call(TILE_READ_FILE, &args![reader, MENU_FILE]).u32();
    if root == 0 {
        return root;
    }
    let menu = e.call(TILE_GET_MENU, &args![root]).u32();
    e.set_global(RACE_SEX_MENU_ME, menu);
    e.call(MENU_SET_MENU_TILE, &args![menu, root, 0u32]);
    if e.call(STRING_C_STR, &args![CAPTION_A]).u32() == 0 {
        for (caption, setting) in [
            (CAPTION_A, SETTING_FOR_CAPTION_A),
            (CAPTION_B, SETTING_FOR_CAPTION_B),
            (CAPTION_C, SETTING_FOR_CAPTION_C),
        ] {
            let text = e.call(SETTING_VALUE, &args![setting]).u32();
            e.call(FIXED_STRING_SET, &args![caption, text]);
        }
        if e.call(GET_CURRENT_TEX_LANGUAGE, &args![]).i32() != 3 {
            for caption in [CAPTION_A, CAPTION_B, CAPTION_C] {
                e.call(CAPTION_UPPER_CASE, &args![caption]);
            }
        }
    }
    let player: u32 = e.global(PLAYER);
    let flag = e.call(PLAYER_FLAG_GETTER, &args![player]).u8();
    e.mem.set_u8(SAVED_PLAYER_FLAG, flag);
    e.call(PLAYER_FLAG_SETTER, &args![player, 0u32]);
    let changes = e.call(GET_INVENTORY_CHANGES, &args![player]).u32();
    for slot in 0..0x14u32 {
        if matches!(slot, 0 | 1 | 5 | 7 | 8 | 9 | 10 | 11 | 14 | 15) {
            let worn = e.call(GET_WORN_ITEM, &args![changes, slot, 0u32]).u32();
            e.mem.set_u32(SAVED_WORN_ITEMS + 4 * slot, worn);
            if worn != 0 {
                let item = e.call(WORD_AT_EIGHT, &args![worn]).u32();
                e.call(
                    UNEQUIP_OBJECT,
                    &args![player, item, 1u32, 0u32, 0u32, 0u32, 1u32],
                );
            }
        }
    }
    race_sex_menu_create_contents(e, Ptr::new(menu), mode as i32);
    let setting = e
        .call(RENDERED_MENU_SETTING, &args![RENDERED_MENU_SETTING_OBJECT])
        .u32();
    if e.mem.u8(setting) != 0 {
        e.call(TILE_UPDATE_ALL, &args![0u32]);
        let block = e.call(ALLOCATE, &args![0x170u32]).u32();
        let rendered = if block != 0 {
            fn_007ac9f0(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.call(RENDERED_MENU_SET_MODE, &args![rendered, mode]);
        e.call(RENDERED_MENU_SET_ROOT, &args![rendered, root]);
        let name = e.call(TILE_NAME_STRING, &args![root]).u32();
        e.call(RENDERED_MENU_SET_POINTER, &args![rendered, name]);
        let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
        e.call(ENTER_RENDERED_MENU, &args![interface, rendered]);
    }
    root
}

// Translated from 007ac9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `0x40c`, the class id of the menu (the `GetClass` slot's value),
/// which `Create` looks the menu up by.
pub fn fn_007ac9c0(_e: &mut Engine) -> u32 {
    RACE_SEX_MENU_CLASS
}

// Translated from 007ac9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the rendered menu object `Create` allocates (0x170 bytes,
/// vtable `010759ec`): runs the base constructor (`007faa20`), then
/// constructs two arrays of four words (`+0xd8`, `+0xe8`), six pointer
/// slots (`+0xf8` to `+0x10c`) and the three members at `+0x110`, `+0x11c`
/// and `+0x140`. The class is the `FORenderedMenuRaceSex` of
/// `forenderedmenuracesex.rs` (its fields sit at the same offsets);
/// unconfirmed in the Xbox PDB since the function has no name in the map.
///
/// The compiler's exception-unwinding frame is not translated.
// Translation incomplete: 007ac9f0 - the C++ exception-unwinding frame (FS:[0]) that destroys the already constructed members is not translated
pub fn fn_007ac9f0(e: &mut Engine, this: Ptr<FORenderedMenuRaceSex>) -> Ptr<FORenderedMenuRaceSex> {
    let base = this.addr();
    e.call(RENDERED_MENU_BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, RENDERED_MENU_VTABLE);
    e.call(
        EH_VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            base + 0xd8,
            4u32,
            4u32,
            NI_POINTER_DEFAULT_CONSTRUCT,
            NI_POINTER_DESTRUCT
        ],
    );
    e.call(
        EH_VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            base + 0xe8,
            4u32,
            4u32,
            NI_POINTER_DEFAULT_CONSTRUCT,
            NI_POINTER_DESTRUCT
        ],
    );
    for slot in [0xf8u32, 0xfc, 0x100, 0x104, 0x108, 0x10c] {
        e.call(NI_POINTER_CONSTRUCT, &args![base + slot, 0u32]);
    }
    for member in [0x110u32, 0x11c, 0x140] {
        e.call(EMPTY_MEMBER_RETURNS_THIS, &args![base + member]);
    }
    this
}
// Translated from 007acb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::_Create` (Xbox PDB): builds the menu's contents for `mode`
/// (2 = barber, 3 = plastic surgeon, anything else = character creation).
///
/// It gives the menu tile a depth when its stacking type is 6006 or 102,
/// shows the tutorial message, sets the two arrow glyph strings, reads the
/// player's base form (`+0xD8`), sets the title string of the tile at
/// `+0x88`, creates the 20 option lists (`MenuOptionList`, 0x24 bytes each,
/// with their captions), fills the lists with options and sliders from the
/// game settings (the marker lists and the hair colour sliders included),
/// calls `AddFaceSliders`, and then sets the projector file, the
/// confirmation message, the zoom bounds, the active list and the starting
/// zoom for the mode.
///
/// The compiler's exception-unwinding frame is not translated.
// Translation incomplete: 007acb60 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary strings when a callee throws is not translated
pub fn race_sex_menu_create_contents(e: &mut Engine, this: Ptr<RaceSexMenu>, mode: i32) {
    let base = this.addr();
    let tile = e.mem.u32(base + 4);
    let kind = e
        .call(GET_FLOAT_TRAIT, &args![tile, TRAIT_STACKING_TYPE])
        .f32();
    let kind_matches = kind as f64 == e.global::<f64>(KIND_ONE_DOUBLE) || {
        let tile = e.mem.u32(base + 4);
        let again = e
            .call(GET_FLOAT_TRAIT, &args![tile, TRAIT_STACKING_TYPE])
            .f32();
        again as f64 == e.global::<f64>(KIND_TWO_DOUBLE)
    };
    if kind_matches {
        let depth = e.call(GET_MAXIMUM_DEPTH, &args![]).f32();
        let tile = e.mem.u32(base + 4);
        e.call(SET_FLOAT_TRAIT, &args![tile, TRAIT_DEPTH, depth, 1u32]);
    }
    let class = fn_007ac9c0(e);
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    e.call(
        TUTORIAL_SHOW_MESSAGE,
        &args![interface + 0x4d4, 0xfu32, class, 0x200u32],
    );
    for (trait_id, glyph) in [(0x1005u32, GLYPH_LESS), (0x1006, GLYPH_GREATER)] {
        let tile = word_at_four(e, base);
        e.call(SET_STRING_TRAIT, &args![tile, trait_id, glyph, 1u32]);
    }
    let player: u32 = e.global(PLAYER);
    let form = e.call(PLAYER_BASE_FORM, &args![player]).u32();
    let player_base = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_BOUND_OBJECT, TYPE_NPC, 0u32],
        )
        .u32();
    e.mem.set_u32(base + 0xd8, player_base);
    let back = e.call(STRING_C_STR, &args![CAPTION_C]).u32();
    let title_tile = e.mem.u32(base + 0x88);
    e.call(
        SET_STRING_TRAIT,
        &args![title_tile, TRAIT_STRING, back, 1u32],
    );

    // Four temporary strings (`BSStringT`, 8 bytes each) with the captions
    // of the first four lists.
    // The exe's frame (`SUB ESP,0xac` under the exception record, then the record link): the
    // four numbered captions at `EBP-0x7c` (+0x40), the projector text at `EBP-0x90` (+0x2c).
    e.with_stack(0xbc, |e, frame| {
        let numbered = frame.addr() + 0x40;
        e.call(
            EH_VECTOR_CONSTRUCTOR_ITERATOR,
            &args![numbered, 8u32, 4u32, STRING_CONSTRUCT, STRING_DESTROY],
        );
        if (2..=3).contains(&mode) {
            for (index, setting) in NUMBERED_SETTINGS.iter().enumerate() {
                let text = e.call(SETTING_VALUE, &args![*setting]).u32();
                e.call(STRING_SET, &args![numbered + 8 * index as u32, text, 0u32]);
            }
        } else {
            for (index, setting) in NUMBERED_SETTINGS.iter().enumerate() {
                let text = e.call(SETTING_VALUE, &args![*setting]).u32();
                e.call(
                    STRING_FORMAT,
                    &args![numbered + 8 * index as u32, NUMBERED_FORMATS[index], text],
                );
            }
        }
        // The captions of the 20 lists: the four numbered strings, then the
        // values of 16 settings.
        let mut captions = [0u32; 0x14];
        for (index, caption) in captions.iter_mut().take(4).enumerate() {
            *caption = e
                .call(STRING_C_STR, &args![numbered + 8 * index as u32])
                .u32();
        }
        for (index, setting) in LIST_CAPTION_SETTINGS.iter().enumerate() {
            captions[4 + index] = e.call(SETTING_VALUE, &args![*setting]).u32();
        }
        for index in 0..0x14u32 {
            let block = e.call(ALLOCATE, &args![0x24u32]).u32();
            let list = if block != 0 {
                menu_option_list_menu_option_list(
                    e,
                    Ptr::new(block),
                    index,
                    captions[index as usize],
                )
                .addr()
            } else {
                0
            };
            e.mem.set_u32(base + 0x28 + 4 * index, list);
        }
        for (slot, callback) in [
            (0x28u32, 0x007a_d940u32),
            (0x2c, 0x007a_d7f0),
            (0x3c, 0x007a_d860),
            (0x48, 0x007a_d8d0),
            (0x44, 0x007a_d9c0),
        ] {
            let list = e.mem.u32(base + slot);
            e.call(OPTION_LIST_SET_CALLBACK, &args![list, callback]);
        }
        // Two entries of the first list (ids 0x15 and 0x16).
        for (setting, id) in [(SETTING_ENTRY_15, 0x15u32), (SETTING_ENTRY_16, 0x16)] {
            let text = e.call(SETTING_TEXT, &args![setting]).u32();
            let list = e.mem.u32(base + 0x28);
            e.call(OPTION_LIST_ADD_OPTION, &args![list, text, id]);
        }

        // Entries for the marker list (id 0x17, list at +0x2c): every node of
        // the first form list whose item passes the check at `0059f610`.
        let owner: u32 = e.global(MARKER_OWNER);
        let mut node = e.call(MARKER_LIST_HEAD, &args![owner]).u32();
        while node != 0 {
            let item_slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![node]).u32();
            if e.mem.u32(item_slot) == 0 {
                break;
            }
            let item_slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![node]).u32();
            let item = e.mem.u32(item_slot);
            if e.call(MARKER_CHECK, &args![item]).u8() != 0 {
                let item_slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![node]).u32();
                let item = e.mem.u32(item_slot);
                let item_slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![node]).u32();
                let named = e.mem.u32(item_slot);
                let name = e.call(MARKER_NAME, &args![named + 0x18]).u32();
                let list = e.mem.u32(base + 0x2c);
                let option = e
                    .call(OPTION_LIST_ADD_OPTION, &args![list, name, 0x17u32])
                    .u32();
                e.call(OPTION_SET_DATA, &args![option, item]);
            }
            node = word_at_four(e, node);
        }

        let step = x87_copy_f32(e, SLIDER_STEP_QUARTER);
        let value = e.call(SETTING_TEXT, &args![SETTING_SLIDER_18]).u32();
        let list = e.mem.u32(base + 0x30);
        e.call(
            OPTION_LIST_ADD_SLIDER,
            &args![list, value, 0x18u32, 1u32, 0x14u32, 0x8000_0001u32, step],
        );
        add_sub_list(e, base, 0x30, SETTING_36E4, 0x38);
        let text = e.call(SETTING_TEXT, &args![SETTING_ENTRY_19]).u32();
        let list = e.mem.u32(base + 0x30);
        e.call(OPTION_LIST_ADD_OPTION, &args![list, text, 0x19u32]);
        let step = x87_copy_f32(e, SLIDER_STEP_QUARTER);
        let value = e.call(SETTING_TEXT, &args![SETTING_SLIDER_1A]).u32();
        let list = e.mem.u32(base + 0x30);
        let option = e
            .call(
                OPTION_LIST_ADD_SLIDER,
                &args![list, value, 0x1au32, 1u32, 0xau32, 0x8000_0001u32, step],
            )
            .u32();
        fn_007b3810(e, Ptr::new(option), SPACE_STRING);
        add_sub_list(e, base, 0x34, SETTING_22D4, 0x3c);
        add_sub_list(e, base, 0x34, SETTING_4A74, 0x40);
        let player_base = e.mem.u32(base + 0xd8);
        if e.call(GET_SEX, &args![player_base]).u32() == 0 {
            add_sub_list(e, base, 0x34, SETTING_23A0, 0x44);
        } else {
            let list = e.mem.u32(base + 0x44);
            e.call(LIST_SET_BACK_TARGET, &args![list, 3u32]);
        }
        add_sub_list(e, base, 0x38, SETTING_26D0, 0x4c);
        add_sub_list(e, base, 0x38, SETTING_2250, 0x74);
        add_sub_list(e, base, 0x38, SETTING_31C8, 0x48);

        // The hair colour sliders (ids 0x1d to 0x20, list at +0x40).
        let player_base = e.mem.u32(base + 0xd8);
        let color = e.call(HAIR_COLOR_GETTER, &args![player_base]).u32();
        let text = e.call(SETTING_VALUE, &args![SETTING_3528]).u32();
        let step = x87_copy_f32(e, SLIDER_STEP_QUARTER);
        let value = e.call(SETTING_TEXT, &args![SETTING_SLIDER_1D]).u32();
        let list = e.mem.u32(base + 0x40);
        let option = e
            .call(
                OPTION_LIST_ADD_SLIDER,
                &args![list, value, 0x1du32, 0u32, 0xfu32, 0x8000_0001u32, step],
            )
            .u32();
        fn_007b3810(e, Ptr::new(option), text);
        for (setting, id, channel) in [
            (SETTING_SLIDER_1E, 0x1eu32, color & 0xff),
            (SETTING_SLIDER_1F, 0x1f, (color & 0xffff) >> 8),
            (SETTING_SLIDER_20, 0x20, (color >> 16) & 0xff),
        ] {
            let step = x87_copy_f32(e, SLIDER_STEP_EIGHTH);
            let value = e.call(SETTING_TEXT, &args![setting]).u32();
            let list = e.mem.u32(base + 0x40);
            e.call(
                OPTION_LIST_ADD_SLIDER,
                &args![list, value, id, 0u32, 0xffu32, channel, step],
            );
        }

        // Entries for the second marker list (id 0x1c, list at +0x44).
        let owner: u32 = e.global(MARKER_OWNER);
        let mut node = e.call(MARKER_LIST_HEAD_2, &args![owner]).u32();
        while node != 0 {
            let item_slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![node]).u32();
            if e.mem.u32(item_slot) == 0 {
                break;
            }
            let item_slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![node]).u32();
            let item = e.mem.u32(item_slot);
            if e.call(MARKER_CHECK_2, &args![item]).u8() != 0 {
                let item_slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![node]).u32();
                let item = e.mem.u32(item_slot);
                let item_slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![node]).u32();
                let named = e.mem.u32(item_slot);
                let name = e.call(MARKER_NAME, &args![named + 0x18]).u32();
                let list = e.mem.u32(base + 0x44);
                let option = e
                    .call(OPTION_LIST_ADD_OPTION, &args![list, name, 0x1cu32])
                    .u32();
                e.call(OPTION_SET_DATA, &args![option, item]);
            }
            node = word_at_four(e, node);
        }

        // The sub-lists 10 to 18 hang below the list at +0x4c, captioned like
        // the lists themselves.
        for index in 10..0x13u32 {
            let sub_list = e.mem.u32(base + 0x28 + 4 * index);
            let list = e.mem.u32(base + 0x4c);
            e.call(
                OPTION_LIST_ADD_SUB_LIST,
                &args![list, captions[index as usize], sub_list],
            );
        }
        race_sex_menu_add_face_sliders(e, this);

        let projector = frame.addr() + 0x2c;
        e.call(STRING_CONSTRUCT, &args![projector]);
        let active_list: i32 = match mode {
            2 => {
                e.call(STRING_SET, &args![base + 0x90, PROJECTOR_BARBER, 0u32]);
                let text = e.call(SETTING_VALUE, &args![SETTING_43D8]).u32();
                e.call(STRING_SET, &args![projector, text, 0u32]);
                e.mem.set_u32(base + 0x120, 3);
                set_zoom_bounds(
                    e,
                    base,
                    [
                        0x0107_5aa8,
                        0x0101_e33c,
                        0x0107_5aa4,
                        0x0107_5aa0,
                        0x0105_10fc,
                        0x0107_5a9c,
                    ],
                );
                3
            }
            3 => {
                e.call(STRING_SET, &args![base + 0x90, PROJECTOR_SURGERY, 0u32]);
                let text = e.call(SETTING_VALUE, &args![SETTING_2058]).u32();
                e.call(STRING_SET, &args![projector, text, 0u32]);
                e.mem.set_u32(base + 0x120, 2);
                set_zoom_bounds(
                    e,
                    base,
                    [
                        0x0107_5a70,
                        0x0102_edc0,
                        0x0107_5aa4,
                        0x0107_5aa0,
                        0x0105_10fc,
                        0x0107_5a6c,
                    ],
                );
                race_sex_menu_make_preset_head_list(e, this);
                2
            }
            _ => {
                e.call(STRING_SET, &args![base + 0x90, PROJECTOR_REFLECTRON, 0u32]);
                let text = e.call(SETTING_VALUE, &args![SETTING_454C]).u32();
                e.call(STRING_SET, &args![projector, text, 0u32]);
                for (target, source) in [(0x28u32, 0x2cu32), (0x2c, 0x30), (0x30, 0x34)] {
                    let target_list = e.mem.u32(base + target);
                    let source_list = e.mem.u32(base + source);
                    e.call(OPTION_LIST_LINK, &args![target_list, source_list]);
                }
                e.mem.set_u32(base + 0x120, 3);
                set_zoom_bounds(
                    e,
                    base,
                    [
                        0x0107_5a48,
                        0x0102_edc0,
                        0x0107_5aa4,
                        0x0107_5a44,
                        0x0105_10fc,
                        0x0107_5a9c,
                    ],
                );
                race_sex_menu_make_preset_head_list(e, this);
                0
            }
        };
        if e.call(GET_CURRENT_TEX_LANGUAGE, &args![]).i32() != 1 {
            e.call(STRING_UPPER_CASE, &args![projector]);
        }
        let name = e.call(STRING_C_STR, &args![projector]).u32();
        let label = e.call(SETTING_VALUE, &args![SETTING_4108]).u32();
        e.call(
            STRING_FORMAT,
            &args![base + 0x98, CONFIRMATION_FORMAT, label, name],
        );
        let pointer = e
            .call(SETTING_FLOAT_POINTER, &args![SETTING_STARTING_ZOOM])
            .u32();
        let starting_zoom = x87_copy_f32(e, pointer);
        let limited = e.call(FLOAT_MIN, &args![1.0f32, starting_zoom]).f32();
        let clamped = e.call(FLOAT_MAX, &args![0.0f32, limited]).f32();
        e.mem.set_f32(base + 0xd0, clamped);
        let copied = quiet_f32(e.mem.f32(base + 0xd0));
        e.mem.set_f32(base + 0xc8, copied);
        for slot in [0x28u32, 0x2c, 0x44] {
            let list = e.mem.u32(base + slot);
            e.call(OPTION_LIST_UPDATE_CURRENT_SELECTION, &args![list]);
        }
        e.call(SOUND_HANDLE_PLAY, &args![base + 0xf0, 0u32]);
        e.call(SOUND_HANDLE_PLAY, &args![base + 0xfc, 0u32]);
        race_sex_menu_set_active_list(e, this, active_list);
        e.call(MENU_PREP_FOR_VISIBILITY, &args![this, 0u32]);
        e.call(STRING_DESTROY, &args![projector]);
        e.call(
            EH_VECTOR_DESTRUCTOR_ITERATOR,
            &args![numbered, 8u32, 4u32, STRING_DESTROY],
        );
    });
}
/// The shared shape of `007ad7f0`, `007ad860` and `007ad8d0`: with a non-null
/// option whose object (`0044ddc0`) is non-null and the menu and its player
/// base form present, true when `getter` of the base form equals the
/// option's object.
fn option_matches_base_value(e: &mut Engine, option: u32, getter: u32) -> bool {
    if option != 0 && e.call(WORD_AT_EIGHT, &args![option]).u32() != 0 {
        let me: u32 = e.global(RACE_SEX_MENU_ME);
        if me != 0 && e.mem.u32(me + 0xd8) != 0 {
            let player_base = e.mem.u32(me + 0xd8);
            let current = e.call(getter, &args![player_base]).u32();
            let object = e.call(WORD_AT_EIGHT, &args![option]).u32();
            return current == object;
        }
    }
    false
}

// Translated from 007ad7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Option list check (`cdecl`, one word; `_Create` stores it in the list at
/// `+0x2c`): see `option_matches_base_value`, with `004ac110` of the base
/// form.
pub fn fn_007ad7f0(e: &mut Engine, option: u32) -> bool {
    option_matches_base_value(e, option, BASE_VALUE_4AC110)
}

// Translated from 007ad860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Option list check (`cdecl`, one word; stored in the list at `+0x3c`):
/// `option_matches_base_value` with `00603b50`.
pub fn fn_007ad860(e: &mut Engine, option: u32) -> bool {
    option_matches_base_value(e, option, BASE_VALUE_603B50)
}

// Translated from 007ad8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Option list check (`cdecl`, one word; stored in the list at `+0x48`):
/// `option_matches_base_value` with `00602150`.
pub fn fn_007ad8d0(e: &mut Engine, option: u32) -> bool {
    option_matches_base_value(e, option, BASE_VALUE_602150)
}

// Translated from 007ad940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Option list check (`cdecl`, one word, stored in the first list): true when
/// the menu and the player's base form exist and either the sex is 0 and
/// the option's id (`00726070`) is `0x15`, or the sex is 1 and the id is
/// `0x16`.
pub fn fn_007ad940(e: &mut Engine, option: u32) -> bool {
    if option == 0 {
        return false;
    }
    let me: u32 = e.global(RACE_SEX_MENU_ME);
    if me == 0 || e.mem.u32(me + 0xd8) == 0 {
        return false;
    }
    let player_base = e.mem.u32(me + 0xd8);
    if e.call(GET_SEX, &args![player_base]).u32() == 0 && word_at_four(e, option) == 0x15 {
        return true;
    }
    let me: u32 = e.global(RACE_SEX_MENU_ME);
    let player_base = e.mem.u32(me + 0xd8);
    e.call(GET_SEX, &args![player_base]).u32() == 1 && word_at_four(e, option) == 0x16
}

// Translated from 007ad9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Option list check (`cdecl`, one word, stored in the list at `+0x44`): true
/// when the menu and the player's base form exist, `008256d0` of the head
/// part list (`00602170`) is false and the first entry of that list is the
/// option's object.
pub fn fn_007ad9c0(e: &mut Engine, option: u32) -> bool {
    if option == 0 {
        return false;
    }
    let me: u32 = e.global(RACE_SEX_MENU_ME);
    if me == 0 || e.mem.u32(me + 0xd8) == 0 {
        return false;
    }
    let player_base = e.mem.u32(me + 0xd8);
    let parts = e.call(HEAD_PART_LIST, &args![player_base]).u32();
    if e.call(LIST_IS_EMPTY, &args![parts]).u8() != 0 {
        return false;
    }
    let me: u32 = e.global(RACE_SEX_MENU_ME);
    let player_base = e.mem.u32(me + 0xd8);
    let parts = e.call(HEAD_PART_LIST, &args![player_base]).u32();
    let first = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![parts]).u32();
    let object = e.call(WORD_AT_EIGHT, &args![option]).u32();
    e.mem.u32(first) == object
}

// Translated from 007ada40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::Close` (Xbox PDB): with the menu open, closes the open
/// flag-9 state (`007b25a0`, bit 9 cleared). If the message box result is 1
/// (the player confirmed a popup) and the gamepad is in use, focuses the
/// current list (or the tile at `+0x8c` when it is visible). Otherwise the
/// menu closes: it updates the stats menu's player name, fades out, clears
/// `pMe`, ends a dialogue service when `00705100` says so, re-equips the
/// worn items taken off by `Create` and restores the player's flag.
pub fn race_sex_menu_close(e: &mut Engine) {
    let me: u32 = e.global(RACE_SEX_MENU_ME);
    if me == 0 {
        return;
    }
    let this = Ptr::new(me);
    if race_sex_menu_get_flag(e, this, 9) {
        fn_007b25a0(e, this);
        race_sex_menu_set_flag(e, this, 9, false);
    }
    let result = e.call(GET_MESSAGE_MENU_RESULT, &args![]).u8() as i8;
    if result == 1 {
        if e.call(GAMEPAD_IN_USE, &args![]).u8() != 0 {
            let me: u32 = e.global(RACE_SEX_MENU_ME);
            let index = e.call(CURRENT_LIST_INDEX, &args![me]).u32();
            let list = e.mem.u32(me + 0x28 + 4 * index);
            if e.call(OPTION_LIST_FOCUS, &args![list, 1u32]).u8() == 0 {
                let me: u32 = e.global(RACE_SEX_MENU_ME);
                let tile = e.mem.u32(me + 0x8c);
                if e.call(TILE_IS_VISIBLE, &args![tile]).u8() != 0 {
                    let tile = e.mem.u32(e.global::<u32>(RACE_SEX_MENU_ME) + 0x8c);
                    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
                    e.call(
                        SET_CURRENT_FOCUS_TARGET,
                        &args![interface, tile, 0xfc3u32, 1u32],
                    );
                } else {
                    let me: u32 = e.global(RACE_SEX_MENU_ME);
                    let index = e.call(CURRENT_LIST_INDEX, &args![me]).u32();
                    let list = e.mem.u32(me + 0x28 + 4 * index);
                    e.call(OPTION_LIST_REFRESH, &args![list]);
                    race_sex_menu_update_scroll_arrows(e, Ptr::new(me));
                }
            }
        }
    } else {
        e.call(UPDATE_PLAYER_NAME, &args![]);
        let me: u32 = e.global(RACE_SEX_MENU_ME);
        let tile = word_at_four(e, me);
        e.call(SET_TRAIT_FROM_INT, &args![tile, TRAIT_0X1772, 1u32]);
        let me: u32 = e.global(RACE_SEX_MENU_ME);
        e.call(MENU_INSTANT_FADE_OUT, &args![me]);
        e.set_global(RACE_SEX_MENU_ME, 0u32);
        if e.call(DIALOGUE_ACTIVE, &args![]).u8() != 0 {
            let topic = e.call(GET_TOPIC, &args![5u32, 4u32]).u32();
            e.call(DIALOG_SERVICE_END, &args![topic]);
        }
        let player: u32 = e.global(PLAYER);
        for slot in 0..0x14u32 {
            let worn = e.mem.u32(SAVED_WORN_ITEMS + 4 * slot);
            if worn != 0 {
                let item = e.call(WORD_AT_EIGHT, &args![worn]).u32();
                e.call(
                    EQUIP_OBJECT,
                    &args![player, item, 1u32, 0u32, 1u32, 0u32, 1u32],
                );
                let again = e.mem.u32(SAVED_WORN_ITEMS + 4 * slot);
                if again != 0 {
                    e.call(WORN_ITEM_DELETE, &args![again, 1u32]);
                }
                e.mem.set_u32(SAVED_WORN_ITEMS + 4 * slot, 0);
            }
        }
        let flag = e.mem.u8(SAVED_PLAYER_FLAG);
        e.call(PLAYER_FLAG_SETTER, &args![player, flag as u32]);
        fn_007adcc0(e, Ptr::new(player), 1);
    }
}

// Translated from 007adc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `bit` (the count is taken modulo 32, as `SHL` does) of
/// `uiFlags` (+0xEC) is set.
pub fn race_sex_menu_get_flag(e: &mut Engine, this: Ptr<RaceSexMenu>, bit: u32) -> bool {
    let flags = e.mem.u32(this.addr() + 0xec);
    1u32.wrapping_shl(bit) & flags != 0
}

// Translated from 007adc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`on`) or clears bit `bit` (modulo 32) of `uiFlags` (+0xEC).
pub fn race_sex_menu_set_flag(e: &mut Engine, this: Ptr<RaceSexMenu>, bit: u32, on: bool) {
    let flags = e.mem.u32(this.addr() + 0xec);
    let mask = 1u32.wrapping_shl(bit);
    let flags = if on { flags | mask } else { flags & !mask };
    e.mem.set_u32(this.addr() + 0xec, flags);
}

// Translated from 007adcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte at `+0x75d` of its object (the player in `Close`).
pub fn fn_007adcc0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x75d, value);
}
/// The text of a confirmation popup: the value of a setting, or the C string
/// of a `BSStringT` (resolved after the two caption settings, as the code
/// does).
#[derive(Clone, Copy)]
enum PopupText {
    Setting(u32),
    String(u32),
}

/// Shows the confirmation popup (`00703e80`, cdecl, 11 words): the text,
/// `callback` (run when the popup is confirmed) and the two captions from
/// the settings at `011d34f8` and `011d3684`.
fn show_confirmation(e: &mut Engine, text: PopupText, callback: u32, flag: u32) {
    let first = e.call(SETTING_VALUE, &args![SETTING_POPUP_A]).u32();
    let second = e.call(SETTING_VALUE, &args![SETTING_POPUP_B]).u32();
    let text = match text {
        PopupText::Setting(setting) => e.call(SETTING_VALUE, &args![setting]).u32(),
        PopupText::String(string) => e.call(STRING_C_STR, &args![string]).u32(),
    };
    e.call(
        SHOW_MESSAGE_BOX,
        &args![text, 0u32, 0u32, callback, flag, 0x12u32, 0.0f32, 0.0f32, second, first, 0u32],
    );
}

// Translated from 007adce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::DoClick` (Xbox PDB): a click on `tile` (taken from
/// `pTiles` for the buttons 0 to 5 when null) with `button`.
///
/// When the tile belongs to an option of the current list (`007b41c0`),
/// the option's id (`00726070`) decides: `0x15`, `0x16` and `0x17` choose
/// the sex or the race (when the option is not already selected),
/// `0x19` randomizes (after a confirmation popup unless flag 7 is set),
/// `0x1b`, `0x1c` and `0x21` change hair, hair colour and a head part, ids
/// below 0x14 open that list. Otherwise button 4 goes back to the list
/// `84e3a0` names, button 5 closes (with a popup on the last list) or goes
/// to `0044edb0`'s list, the tile's parent option starts a slider drag
/// and the buttons 2 and 3 scroll the list by one.
pub fn race_sex_menu_do_click(e: &mut Engine, this: Ptr<RaceSexMenu>, button: i32, tile: u32) {
    let base = this.addr();
    let mut tile = tile;
    if tile == 0 && button < 6 {
        tile = e.mem.u32(
            base.wrapping_add(0x78)
                .wrapping_add((button as u32).wrapping_mul(4)),
        );
    }
    let list_index = e.call(CURRENT_LIST_INDEX, &args![this]).u32();
    let list = option_list(e, base, list_index);
    let mut option = e.call(OPTION_AT_TILE, &args![list, tile]).u32();
    if option != 0 {
        let mut changed = false;
        let mut sex_changed = false;
        let id = word_at_four(e, option);
        let case = id.wrapping_sub(0x15);
        // Cases 0x15, 0x16 and 0x17 share the selection body; 0x16 first
        // clears the head part list of the player's base form.
        let selection = case == 0 || case == 1 || case == 2;
        if case == 1 {
            let player_base = e.mem.u32(base + 0xd8);
            let parts = e.call(HEAD_PART_LIST, &args![player_base]).u32();
            e.call(HEAD_PART_LIST_CLEAR, &args![parts]);
        }
        if selection {
            if !menu_option_is_selected(e, Ptr::new(option)) {
                if word_at_four(e, option) == 0x17 {
                    let player_base = e.mem.u32(base + 0xd8);
                    let race = e.call(BASE_VALUE_4AC110, &args![player_base]).u32();
                    e.call(KILL_EGT_DATA, &args![race]);
                    let player: u32 = e.global(PLAYER);
                    e.call(PLAYER_RACE_CHANGED, &args![player]);
                    e.call(PLAYER_RECALCULATE, &args![player + 0x94, 0u32]);
                    e.call(PLAYER_UPDATE, &args![player]);
                    let object = e.call(WORD_AT_EIGHT, &args![option]).u32();
                    let player_base = e.mem.u32(base + 0xd8);
                    e.call(SET_RACE, &args![player_base, object, 0u32]);
                } else {
                    let player_base = e.mem.u32(base + 0xd8);
                    let race = e.call(BASE_VALUE_4AC110, &args![player_base]).u32();
                    e.call(KILL_EGT_DATA, &args![race]);
                    let is_female = word_at_four(e, option) == 0x16;
                    let player_base = e.mem.u32(base + 0xd8);
                    e.call(
                        SET_FLAG_BIT,
                        &args![player_base + 0x30, 1u32, is_female, 1u32],
                    );
                    sex_changed = true;
                }
                e.call(CHOOSE_DEFAULT_PRESET, &args![this]);
                fn_007b25a0(e, this);
                fn_007b1ca0(e, this);
                changed = true;
                if sex_changed {
                    let player: u32 = e.global(PLAYER);
                    e.call(PLAYER_REFRESH_3D, &args![player, 1u32]);
                }
            }
        } else {
            match id {
                0x19 => {
                    if race_sex_menu_get_flag(e, this, 7) {
                        e.call(RANDOMIZE, &args![this]);
                    } else {
                        show_confirmation(
                            e,
                            PopupText::Setting(SETTING_RANDOMIZE_TEXT),
                            RANDOMIZE,
                            0,
                        );
                    }
                }
                0x1b => {
                    if !menu_option_is_selected(e, Ptr::new(option)) {
                        let object = e.call(WORD_AT_EIGHT, &args![option]).u32();
                        let player_base = e.mem.u32(base + 0xd8);
                        e.call(SET_HAIR, &args![player_base, object]);
                        fn_007b2b50(e, this, true);
                        changed = true;
                    }
                }
                0x1c => {
                    let player_base = e.mem.u32(base + 0xd8);
                    let parts = e.call(HEAD_PART_LIST, &args![player_base]).u32();
                    e.call(HEAD_PART_LIST_CLEAR, &args![parts]);
                    if !menu_option_is_selected(e, Ptr::new(option)) {
                        let object = e.call(WORD_AT_EIGHT, &args![option]).u32();
                        e.with_stack(4, |e, slot| {
                            e.mem.set_u32(slot.addr(), object);
                            let player_base = e.mem.u32(base + 0xd8);
                            let parts = e.call(HEAD_PART_LIST, &args![player_base]).u32();
                            e.call(HEAD_PART_LIST_ADD, &args![parts, slot]);
                        });
                    }
                    fn_007b1e70(e, this, true, true, false);
                    changed = true;
                }
                0x21 => {
                    if !menu_option_is_selected(e, Ptr::new(option)) {
                        let object = e.call(WORD_AT_EIGHT, &args![option]).u32();
                        let player_base = e.mem.u32(base + 0xd8);
                        e.call(SET_HEAD, &args![player_base, object]);
                        fn_007b27c0(e, this);
                        changed = true;
                    }
                }
                _ => {
                    let id = word_at_four(e, option);
                    if id < 0x14 {
                        let id = word_at_four(e, option);
                        race_sex_menu_set_active_list(e, this, id as i32);
                    }
                }
            }
        }
        if changed {
            let list = option_list(e, base, list_index);
            e.call(OPTION_LIST_UPDATE_CURRENT_SELECTION, &args![list]);
            race_sex_menu_set_flag(e, this, 7, false);
        }
        return;
    }
    if button == 4 {
        let list = option_list(e, base, list_index);
        let target = e.call(LIST_BACK_TARGET, &args![list]).u32();
        race_sex_menu_set_active_list(e, this, target as i32);
    } else if button == 5 {
        if list_index == e.mem.u32(base + 0x120) {
            show_confirmation(e, PopupText::String(base + 0x98), CLOSE_CALLBACK, 1);
        } else {
            let list = option_list(e, base, list_index);
            let target = e.call(LIST_PARENT_TARGET, &args![list]).u32();
            race_sex_menu_set_active_list(e, this, target as i32);
        }
    } else {
        let tile_word = e.call(WORD_AT_0X28, &args![tile]).u32();
        let list = option_list(e, base, list_index);
        option = e.call(OPTION_AT_TILE, &args![list, tile_word]).u32();
        if option != 0 {
            e.mem.set_u32(base + 0xc0, tile);
            e.mem.set_u32(base + 0xc4, option);
            e.call(SLIDER_PRESS, &args![this]);
        } else if button == 2 && e.call(TILE_IS_TRUE, &args![tile, TRAIT_VISIBLE]).u8() != 0 {
            let list = option_list(e, base, list_index);
            e.call(OPTION_LIST_SCROLL_BY, &args![list, 1u32]);
            race_sex_menu_update_scroll_arrows(e, this);
        } else if button == 3 && e.call(TILE_IS_TRUE, &args![tile, TRAIT_VISIBLE]).u8() != 0 {
            let list = option_list(e, base, list_index);
            e.call(OPTION_LIST_SCROLL_BY, &args![list, u32::MAX]);
            race_sex_menu_update_scroll_arrows(e, this);
        }
    }
}

// Translated from 007ae160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::MenuOption::IsSelected` (Xbox PDB): with a tile at `+0`,
/// whether its user trait `_selected` equals the constant at `01011590`.
pub fn menu_option_is_selected(e: &mut Engine, this: Ptr<MenuOption>) -> bool {
    let tile = e.mem.u32(this.addr());
    if tile == 0 {
        return false;
    }
    let trait_id = e
        .call(ADD_USER_TRAIT, &args![SELECTED_TRAIT_NAME, u32::MAX])
        .u32();
    let value = e.call(GET_FLOAT_TRAIT, &args![tile, trait_id]).f32();
    value as f64 == e.global::<f64>(SELECTED_VALUE)
}

// Translated from 007ae1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::DoDownClick` (Xbox PDB): button `0x69` (a slider) records
/// the dragged tile and option and the option's value; button 1 and `0x69`
/// then store the pointer position (`007697f0`, a float) in `fDragStartX`.
pub fn race_sex_menu_do_down_click(e: &mut Engine, this: Ptr<RaceSexMenu>, button: i32, tile: u32) {
    let base = this.addr();
    if button != 1 {
        if button != 0x69 {
            return;
        }
        e.mem.set_u32(base + 0xc0, tile);
        let tile_word = e.call(WORD_AT_0X28, &args![tile]).u32();
        let list_index = e.call(CURRENT_LIST_INDEX, &args![this]).u32();
        let list = option_list(e, base, list_index);
        let option = e.call(OPTION_AT_TILE, &args![list, tile_word]).u32();
        e.mem.set_u32(base + 0xc4, option);
        let value = menu_option_get_value(e, Ptr::new(option));
        e.mem.set_i32(base + 0xbc, value);
    }
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    let x = e.call(POINTER_X, &args![interface]).f32();
    e.mem.set_f32(base + 0xb8, x);
}

// Translated from 007ae240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::DoStillDown` (Xbox PDB): while the pointer is held, the
/// distance moved since `fDragStartX` (when not 0): for a slider (button
/// `0x69`) sets the option's value from the drag and releases the slider
/// when the value changed; for button 1 rotates the player (the distance
/// over the width of the tile at `+0x7c`, times a full turn) and stores
/// the new pointer position.
///
/// The compiler's exception-unwinding frame (the static initializer guard)
/// is not translated.
// Translation incomplete: 007ae240 - the C++ exception-unwinding frame (FS:[0]) of the function-local static initializer is not translated
pub fn race_sex_menu_do_still_down(e: &mut Engine, this: Ptr<RaceSexMenu>, button: i32, tile: u32) {
    let base = this.addr();
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    let x = e.call(POINTER_X, &args![interface]).f32();
    let delta = (x as f64 - e.mem.f32(base + 0xb8) as f64) as f32;
    if delta as f64 == e.global::<f64>(ZERO_DOUBLE) {
        return;
    }
    if button == 0x69 && tile != 0 {
        let list_index = e.call(CURRENT_LIST_INDEX, &args![this]).u32();
        let tile_word = e.call(WORD_AT_0X28, &args![tile]).u32();
        let list = option_list(e, base, list_index);
        let option = e.call(OPTION_AT_TILE, &args![list, tile_word]).u32();
        if option != 0 {
            let (min, max) = e.with_stack(8, |e, range| {
                fn_007b3840(e, Ptr::new(option), range.addr(), range.addr() + 4);
                (e.mem.i32(range.addr()), e.mem.i32(range.addr() + 4))
            });
            let tile_word = e.call(WORD_AT_0X28, &args![tile]).u32();
            let child = e
                .call(TILE_GET_CHILD_BY_ID, &args![tile_word, 0x65u32])
                .u32();
            let width = e.call(GET_FLOAT_TRAIT, &args![child, TRAIT_WIDTH]).f32();
            let scale = (delta as f64 / width as f64) as f32;
            let span = max.wrapping_sub(min);
            let start = e.mem.i32(base + 0xbc);
            let target = (start as f64 + span as f64 * scale as f64) as f32;
            let rounded = e.call(ROUND_TO_STEP, &args![target, 1.0f32]).f32();
            let value = e.call(FTOL, &args![rounded as f64]).i32();
            if menu_option_set_value(e, Ptr::new(option), value) {
                e.mem.set_u32(base + 0xc4, option);
                e.mem.set_u32(base + 0xc0, tile);
                e.call(SLIDER_RELEASE, &args![this]);
            }
        }
    } else if button == 1 {
        if e.global::<u32>(ROTATE_WIDTH_GUARD) & 1 == 0 {
            let flags: u32 = e.global(ROTATE_WIDTH_GUARD);
            e.set_global(ROTATE_WIDTH_GUARD, flags | 1);
            let rotate_tile = e.mem.u32(base + 0x7c);
            let width = e
                .call(GET_FLOAT_TRAIT, &args![rotate_tile, TRAIT_WIDTH])
                .f32();
            e.set_global(ROTATE_WIDTH, width);
        }
        let width: f32 = e.global(ROTATE_WIDTH);
        let angle = ((delta as f64 / width as f64) * e.global::<f64>(TWO_PI_DOUBLE)) as f32;
        let moved = race_sex_menu_rotate_player(e, this, angle, 0);
        race_sex_menu_set_flag(e, this, 5, moved != 0);
        let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
        let x = e.call(POINTER_X, &args![interface]).f32();
        e.mem.set_f32(base + 0xb8, x);
    }
}
/// Starts the loop sound at `handle` when bit `bit` of the menu's flags is
/// set and it is not playing, stops it when the bit is clear and it is
/// playing (the pattern `DoIdle` repeats for the rotate and zoom sounds).
fn sync_loop_sound(e: &mut Engine, this: Ptr<RaceSexMenu>, bit: u32, handle: u32) {
    let wanted = race_sex_menu_get_flag(e, this, bit);
    if wanted {
        if e.call(SOUND_HANDLE_IS_PLAYING, &args![handle]).u8() == 0 {
            e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
        }
    } else if e.call(SOUND_HANDLE_IS_PLAYING, &args![handle]).u8() != 0 {
        e.call(SOUND_HANDLE_STOP, &args![handle]);
    }
}

// Translated from 007ae420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::DoIdle` (Xbox PDB), `__fastcall`/`this` in ECX: runs every
/// frame. When the menu is not the top menu the four loop sounds that are
/// playing (state 1) are paused. When it is, the paused ones (state 3)
/// resume, the elapsed time since the last call (`00457fe0`, kept in
/// `011da740`) is taken, and
///
/// - with the gamepad in use, the two sticks (axes 9 and 10, `00a23390`) turn
///   the player (axis 9) and zoom (axis 10) once past a dead zone of
///   `0x21f1`, the zoom velocity at `+0xCC` building up with time;
/// - otherwise the mouse wheel changes the zoom target `+0xC8` (when the
///   pointer is over the tile at `+0x7c`) or sends the click buttons 2 and
///   3, and the zoom eases towards the target, accelerating and braking.
///
/// Both modes keep the rotate (`+0x114`) and zoom (`+0x108`) sounds in step
/// with flags 5 and 6, and run `007b4b50` or `007b4930` for a pressed slider.
///
/// The compiler's exception-unwinding frame (the static initializer guard)
/// is not translated.
// Translation incomplete: 007ae420 - the C++ exception-unwinding frame (FS:[0]) of the function-local static initializer is not translated
pub fn race_sex_menu_do_idle(e: &mut Engine, this: Ptr<RaceSexMenu>) {
    let base = this.addr();
    let class = e.vcall(base, 0x34, &args![]).u32();
    if e.call(IS_TOP_MENU_ID, &args![class]).u8() == 0 {
        for index in 0..4u32 {
            let handle = base + 0xf0 + index * 0xc;
            if e.call(WORD_AT_EIGHT, &args![handle]).i32() == 1 {
                e.call(SOUND_HANDLE_PAUSE, &args![handle]);
            }
        }
        return;
    }
    for index in 0..4u32 {
        let handle = base + 0xf0 + index * 0xc;
        if e.call(WORD_AT_EIGHT, &args![handle]).i32() == 3 {
            e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
        }
    }
    if e.global::<u32>(LAST_TIME_GUARD) & 1 == 0 {
        let flags: u32 = e.global(LAST_TIME_GUARD);
        e.set_global(LAST_TIME_GUARD, flags | 1);
        let now = e.call(GET_TIME, &args![]).u32();
        e.set_global(LAST_TIME, now);
    }
    let now = e.call(GET_TIME, &args![]).u32();
    let last: u32 = e.global(LAST_TIME);
    let elapsed_ms = now.wrapping_sub(last) as f64;
    let zero: f64 = e.global(ZERO_DOUBLE);
    if e.call(GAMEPAD_IN_USE, &args![]).u8() != 0 {
        let input: u32 = e.global(INPUT_GLOBAL);
        let device = e.call(INPUT_DEVICE, &args![input]).u32();
        let turn = e.call(AXIS_VALUE, &args![device, 0u32, 9u32]).i32();
        let input: u32 = e.global(INPUT_GLOBAL);
        let device = e.call(INPUT_DEVICE, &args![input]).u32();
        let zoom = e.call(AXIS_VALUE, &args![device, 0u32, 10u32]).i32();
        let turn_size = e.call(ABSOLUTE_VALUE, &args![turn]).i32();
        let zoom_size = e.call(ABSOLUTE_VALUE, &args![zoom]).i32();
        race_sex_menu_set_flag(e, this, 5, false);
        race_sex_menu_set_flag(e, this, 6, false);
        let limit: i32 = e.global(AXIS_LIMIT_POSITIVE);
        if zoom_size > 0x21f1 && zoom_size > turn_size {
            let denominator = limit.wrapping_sub(0x21f1);
            let reduced = e.call(AXIS_REDUCE, &args![zoom_size, limit as u32]).i32();
            let numerator = denominator.wrapping_sub(limit.wrapping_sub(reduced));
            let rate = numerator as f64 * e.global::<f64>(ZOOM_RATE_DOUBLE) / denominator as f64;
            let velocity = e.mem.f32(base + 0xcc);
            e.mem
                .set_f32(base + 0xcc, (rate * elapsed_ms + velocity as f64) as f32);
            let direction: i32 = if zoom as f64 > zero { -1 } else { 1 };
            let velocity = e.mem.f32(base + 0xcc);
            let step = (direction as f64 * velocity as f64 * elapsed_ms) as f32;
            let moved = race_sex_menu_zoom_player(e, this, step, 0);
            race_sex_menu_set_flag(e, this, 6, moved != 0);
        } else {
            if turn_size > 0x21f1 && turn_size > zoom_size {
                let ratio = if turn > 0 {
                    let denominator = limit.wrapping_sub(0x21f1);
                    (turn.wrapping_sub(0x21f1) as f64 / denominator as f64) as f32
                } else {
                    let negative_limit: i32 = e.global(AXIS_LIMIT_NEGATIVE);
                    let denominator = negative_limit.wrapping_add(0x21f1);
                    (turn.wrapping_neg().wrapping_sub(0x21f1) as f64 / denominator as f64) as f32
                };
                let angle = (ratio as f64 * e.global::<f64>(ROTATE_RATE_DOUBLE) * elapsed_ms
                    / e.global::<f64>(MILLISECONDS_DOUBLE)) as f32;
                let moved = race_sex_menu_rotate_player(e, this, angle, 0);
                race_sex_menu_set_flag(e, this, 5, moved != 0);
            }
            e.mem.set_f32(base + 0xcc, 0.0);
        }
        sync_loop_sound(e, this, 5, base + 0x114);
        sync_loop_sound(e, this, 6, base + 0x108);
        if e.mem.u32(base + 0xc4) != 0 && race_sex_menu_get_flag(e, this, 4) {
            e.call(SLIDER_RELEASE, &args![this]);
            e.mem.set_u32(base + 0xc4, 0);
            e.mem.set_u32(base + 0xc0, 0);
        }
    } else {
        race_sex_menu_set_flag(e, this, 6, false);
        let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
        let wheel = e.call(WHEEL_DELTA, &args![interface]).i32();
        if wheel != 0 {
            let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
            if e.mem.u32(interface + 0xcc) != 0 {
                let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
                let wheel = e.call(WHEEL_DELTA, &args![interface]).i32();
                let forward = wheel as f64 > zero;
                let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
                if e.mem.u32(interface + 0xcc) == e.mem.u32(base + 0x7c) {
                    let step = if forward {
                        x87_copy_f32(e, WHEEL_ZOOM_STEP)
                    } else {
                        x87_copy_f32(e, WHEEL_ZOOM_STEP_BACK)
                    };
                    let target = (e.mem.f32(base + 0xc8) as f64 + step as f64) as f32;
                    e.mem.set_f32(base + 0xc8, target);
                    let limited = e.call(FLOAT_MIN, &args![target, 1.0f32]).f32();
                    let clamped = e.call(FLOAT_MAX, &args![limited, 0.0f32]).f32();
                    e.mem.set_f32(base + 0xc8, clamped);
                } else if forward {
                    e.vcall(base, 0xc, &args![2i32, 0u32]);
                } else {
                    e.vcall(base, 0xc, &args![3i32, 0u32]);
                }
            }
        }
        let target = e.mem.f32(base + 0xc8);
        let rest = e.mem.f32(base + 0xd0);
        if target > rest {
            let velocity = e.mem.f32(base + 0xcc);
            e.mem.set_f32(
                base + 0xcc,
                (elapsed_ms * e.global::<f64>(ZOOM_RATE_DOUBLE) + velocity as f64) as f32,
            );
            let mut step = (elapsed_ms * e.mem.f32(base + 0xcc) as f64) as f32;
            if (target as f64) < rest as f64 + step as f64 {
                step = (target as f64 - rest as f64) as f32;
            }
            let moved = race_sex_menu_zoom_player(e, this, step, 0);
            race_sex_menu_set_flag(e, this, 6, moved != 0);
        } else if (target as f64) < rest as f64 {
            let velocity = e.mem.f32(base + 0xcc);
            e.mem.set_f32(
                base + 0xcc,
                (velocity as f64 - elapsed_ms * e.global::<f64>(ZOOM_RATE_DOUBLE)) as f32,
            );
            let mut step = (elapsed_ms * e.mem.f32(base + 0xcc) as f64) as f32;
            if target as f64 > rest as f64 + step as f64 {
                step = (target as f64 - rest as f64) as f32;
            }
            let moved = race_sex_menu_zoom_player(e, this, step, 0);
            race_sex_menu_set_flag(e, this, 6, moved != 0);
        } else {
            e.mem.set_f32(base + 0xcc, 0.0);
        }
        sync_loop_sound(e, this, 6, base + 0x108);
        sync_loop_sound(e, this, 5, base + 0x114);
        race_sex_menu_set_flag(e, this, 5, false);
        if e.mem.u32(base + 0xc4) != 0 {
            let owner: u32 = e.global(INPUT_GLOBAL);
            if e.call(INPUT_DEVICE, &args![owner]).u32() != 0 {
                let owner: u32 = e.global(INPUT_GLOBAL);
                let device = e.call(INPUT_DEVICE, &args![owner]).u32();
                if e.call(BUTTON_STATE, &args![device, 0u32, 2u32]).u32() != 0 {
                    e.call(SLIDER_PRESS, &args![this]);
                }
            }
        }
    }
    e.set_global(LAST_TIME, now);
}
/// Plays the one-shot sound `name` (`BSAudio::GetSoundHandleByName` on the
/// audio instance, then `BSSoundHandle::Play(0)`), as `DoGamepad` does for
/// `UIMenuOK` and `UIPipBoyMode`; the temporary handle is destroyed after.
fn play_ui_sound(e: &mut Engine, name: u32) {
    e.with_stack(0xc, |e, handle| {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let found = e
            .call(
                GET_SOUND_HANDLE_BY_NAME_ON_AUDIO,
                &args![audio, handle, name, 0x121u32],
            )
            .u32();
        e.call(SOUND_HANDLE_PLAY, &args![found, 0u32]);
        e.call(EMPTY_MEMBER, &args![handle]);
    });
}

// Translated from 007aecb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::DoGamepad` (Xbox PDB): a gamepad button for the menu;
/// returns whether it was handled (false when the menu is not the top
/// menu or the button is not used). Before dispatching, a non-negative
/// button sets flag 4 to whether the stick (axis 7) is inside the dead zone
/// `0x1ea9` (never for buttons 3 and 4).
///
/// Button 1 goes to the current list's `DoGamepad`. Button 2 keeps the
/// focus on the two arrow tiles (`+0x88`, `+0x8c`) or focuses the visible one
/// when the list does not take it. Buttons 3, 4, 15 and 16 move between the
/// arrows or click the child tile `0x68`, `0x64`, `0x66`, `0x67` of the
/// focused tile through the `DoClick` slot, falling back to the list.
/// Button 10 enters the sub list (sound `UIMenuOK`). Buttons 13 and 14 go
/// back up the lists (sound `UIPipBoyMode`; button 14 asks to close the menu
/// when it cannot).
///
/// The compiler's exception-unwinding frame is not translated.
// Translation incomplete: 007aecb0 - the C++ exception-unwinding frame (FS:[0]) that destroys the sound handle temporaries is not translated
pub fn race_sex_menu_do_gamepad(
    e: &mut Engine,
    this: Ptr<RaceSexMenu>,
    button: i32,
    _unused_1: u32,
) -> bool {
    let base = this.addr();
    let class = e.vcall(base, 0x34, &args![]).u32();
    if e.call(IS_TOP_MENU_ID, &args![class]).u8() == 0 {
        return false;
    }
    if button >= 0 {
        let device = e
            .call(INPUT_DEVICE, &args![e.global::<u32>(INPUT_GLOBAL)])
            .u32();
        let stick = e.call(AXIS_VALUE, &args![device, 0u32, 7u32]).i32();
        let inside = if button == 4 || button == 3 {
            false
        } else {
            e.call(ABSOLUTE_VALUE, &args![stick]).i32() < 0x1ea9
        };
        race_sex_menu_set_flag(e, this, 4, inside);
    }
    let case = (button as u32).wrapping_sub(1);
    let entry = if case > 0xf {
        5
    } else {
        [0u8, 1, 2, 2, 5, 5, 5, 5, 5, 3, 5, 5, 4, 4, 2, 2][case as usize]
    };
    match entry {
        0 => gamepad_to_list(e, this, button),
        1 => {
            let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
            let focus = e.call(FOCUS_TILE, &args![interface]).u32();
            if focus == e.mem.u32(base + 0x88) {
                return true;
            }
            let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
            let focus = e.call(FOCUS_TILE, &args![interface]).u32();
            if focus == e.mem.u32(base + 0x8c) {
                return true;
            }
            if gamepad_to_list(e, this, button) {
                return true;
            }
            for arrow in [0x88u32, 0x8c] {
                let tile = e.mem.u32(base + arrow);
                if e.call(TILE_IS_VISIBLE, &args![tile]).u8() != 0 {
                    let index = e.call(CURRENT_LIST_INDEX, &args![this]).u32();
                    let list = option_list(e, base, index);
                    e.call(OPTION_LIST_FOCUS, &args![list, 0u32]);
                    let tile = e.mem.u32(base + arrow);
                    e.call(SET_FOCUS_TARGET_CDECL, &args![tile]);
                    return true;
                }
            }
            false
        }
        2 => {
            let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
            let mut focus = e.call(FOCUS_TILE, &args![interface]).u32();
            if focus == 0 && e.call(GAMEPAD_IN_USE, &args![]).u8() == 0 {
                let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
                focus = e.mem.u32(interface + 0xcc);
                while focus != 0 {
                    if e.call(WORD_AT_0X28, &args![focus]).u32() == e.mem.u32(base + 0x78) {
                        break;
                    }
                    focus = e.call(WORD_AT_0X28, &args![focus]).u32();
                }
            }
            if focus == e.mem.u32(base + 0x88) && button == 3 {
                let tile = e.mem.u32(base + 0x8c);
                if e.call(TILE_IS_VISIBLE, &args![tile]).u8() != 0 {
                    let tile = e.mem.u32(base + 0x8c);
                    e.call(SET_FOCUS_TARGET_CDECL, &args![tile]);
                    return true;
                }
            }
            if focus == e.mem.u32(base + 0x8c) && button == 4 {
                let tile = e.mem.u32(base + 0x88);
                if e.call(TILE_IS_VISIBLE, &args![tile]).u8() != 0 {
                    let tile = e.mem.u32(base + 0x88);
                    e.call(SET_FOCUS_TARGET_CDECL, &args![tile]);
                    return true;
                }
            }
            if focus != 0 {
                let child_id = match button {
                    0xf => 0x66u32,
                    0x10 => 0x67,
                    3 => 0x68,
                    4 => 0x64,
                    _ => return false,
                };
                let child = e.call(TILE_GET_CHILD_BY_ID, &args![focus, child_id]).u32();
                if child != 0 {
                    e.vcall(base, 0xc, &args![child_id, child]);
                    return true;
                }
            }
            gamepad_to_list(e, this, button)
        }
        3 => {
            let index = e.call(CURRENT_LIST_INDEX, &args![this]).u32();
            let list = option_list(e, base, index);
            let target = e.call(LIST_BACK_TARGET, &args![list]).u32();
            if race_sex_menu_set_active_list(e, this, target as i32) != 0 {
                play_ui_sound(e, SOUND_MENU_OK);
            }
            true
        }
        4 => {
            let mut index = e.call(CURRENT_LIST_INDEX, &args![this]).i32();
            while index >= 4 {
                let list = option_list(e, base, index as u32);
                index = e.call(LIST_BACK_TARGET, &args![list]).i32();
            }
            let list = option_list(e, base, index as u32);
            let handled = if button == 0xd {
                let target = e.call(LIST_BACK_TARGET, &args![list]).u32();
                race_sex_menu_set_active_list(e, this, target as i32)
            } else {
                let target = e.call(LIST_PARENT_TARGET, &args![list]).u32();
                race_sex_menu_set_active_list(e, this, target as i32)
            };
            if handled != 0 {
                play_ui_sound(e, SOUND_PIPBOY_MODE);
            } else if button == 0xe {
                show_confirmation(e, PopupText::String(base + 0x98), CLOSE_CALLBACK, 1);
            }
            true
        }
        _ => false,
    }
}

/// Sends `button` to the current option list's `DoGamepad` (`007b4260`).
fn gamepad_to_list(e: &mut Engine, this: Ptr<RaceSexMenu>, button: i32) -> bool {
    let index = e.call(CURRENT_LIST_INDEX, &args![this]).u32();
    let list = option_list(e, this.addr(), index);
    e.call(OPTION_LIST_DO_GAMEPAD, &args![list, button]).u8() != 0
}
// Translated from 007af180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::SetActiveList` (Xbox PDB): makes list `list` (below 0x14,
/// signed compare, else returns 0) the active one. It stores the number in
/// the menu tile's trait `0x1004`, lets the list refresh (`007b4410`),
/// fills the lists 5 (when flag 0 is set; `007af300`) and 8 (when flag 1 is
/// set; `007af450`), validates the hair option list for list 3, updates the
/// scroll arrows, lights the matching button of the rendered menu for lists
/// below 4, shows the two arrow tiles as the lists' targets say, and sets
/// the caption of the arrow at `+0x8c`. Returns 1.
pub fn race_sex_menu_set_active_list(e: &mut Engine, this: Ptr<RaceSexMenu>, list: i32) -> u8 {
    if list >= 0x14 {
        return 0;
    }
    let base = this.addr();
    let tile = e.mem.u32(base + 4);
    e.call(SET_TRAIT_FROM_INT, &args![tile, TRAIT_0X1004, list]);
    let current = option_list(e, base, list as u32);
    e.call(OPTION_LIST_REFRESH, &args![current]);
    if list == 5 && race_sex_menu_get_flag(e, this, 0) {
        fn_007af300(e, this);
    } else if list == 8 && race_sex_menu_get_flag(e, this, 1) {
        fn_007af450(e, this);
    } else if list == 3 {
        race_sex_menu_validate_hair_option_list(e, this);
    }
    race_sex_menu_update_scroll_arrows(e, this);
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    if e.call(RENDERED_MENU_GETTER, &args![interface]).u32() != 0 && list < 4 {
        let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
        let rendered = e.call(RENDERED_MENU_GETTER, &args![interface]).u32();
        e.call(LIGHT_BUTTON, &args![rendered, list]);
    }
    let current = option_list(e, base, list as u32);
    let back = e.call(LIST_BACK_TARGET, &args![current]).i32();
    let tile = e.mem.u32(base + 0x88);
    e.call(
        SET_TRAIT_FROM_INT,
        &args![tile, TRAIT_VISIBLE, (back < 0x14) as u32],
    );
    let current = option_list(e, base, list as u32);
    let parent = e.call(LIST_PARENT_TARGET, &args![current]).i32();
    let shown = parent < 0x14 || list == e.mem.i32(base + 0x120);
    let tile = e.mem.u32(base + 0x8c);
    e.call(
        SET_TRAIT_FROM_INT,
        &args![tile, TRAIT_VISIBLE, shown as u32],
    );
    let caption = if list == e.mem.i32(base + 0x120) {
        CAPTION_A
    } else {
        CAPTION_B
    };
    let text = e.call(STRING_C_STR, &args![caption]).u32();
    let tile = e.mem.u32(base + 0x8c);
    e.call(SET_STRING_TRAIT, &args![tile, TRAIT_STRING, text, 1u32]);
    1
}

// Translated from 007af300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Refills the option list at `+0x3c` (list 3 of the face lists): empties it
/// (`007b45b0`), adds an option (id `0x1b`, named by `00408da0` of the
/// item) for every node of the list at `011c3f2c` (head `00613790`) whose
/// item passes `005fdf40` and `005fdfa0` with the player's base form, makes
/// the first one `007ad860` accepts the selected one, updates the scroll
/// arrows, steps the list (`DoGamepad` with button 2) until the selected
/// option is the list's current one, and clears flag 0.
pub fn fn_007af300(e: &mut Engine, this: Ptr<RaceSexMenu>) {
    let base = this.addr();
    let list = e.mem.u32(base + 0x3c);
    e.call(OPTION_LIST_CLEAR, &args![list]);
    let mut selected = 0u32;
    let owner: u32 = e.global(MARKER_OWNER);
    let mut node = e.call(HAIR_LIST_HEAD, &args![owner]).u32();
    while node != 0 {
        let slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![node]).u32();
        let item = e.mem.u32(slot);
        if e.call(HAIR_CHECK_A, &args![item]).u8() != 0 {
            let player_base = e.mem.u32(base + 0xd8);
            let slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![node]).u32();
            let item = e.mem.u32(slot);
            if e.call(HAIR_CHECK_B, &args![item, player_base]).u8() != 0 {
                let slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![node]).u32();
                let item = e.mem.u32(slot);
                let name = e.call(MARKER_NAME, &args![item + 0x18]).u32();
                let list = e.mem.u32(base + 0x3c);
                let option = e
                    .call(OPTION_LIST_ADD_OPTION, &args![list, name, 0x1bu32])
                    .u32();
                let slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![node]).u32();
                let item = e.mem.u32(slot);
                e.call(OPTION_SET_DATA, &args![option, item]);
                if selected == 0 && fn_007ad860(e, option) {
                    selected = option;
                }
            }
        }
        node = word_at_four(e, node);
    }
    race_sex_menu_update_scroll_arrows(e, this);
    loop {
        if selected == 0 {
            break;
        }
        let list = e.mem.u32(base + 0x3c);
        if selected == e.call(OPTION_LIST_CURRENT, &args![list]).u32() {
            break;
        }
        let list = e.mem.u32(base + 0x3c);
        if e.call(OPTION_LIST_DO_GAMEPAD, &args![list, 2i32]).u8() == 0 {
            break;
        }
    }
    let list = e.mem.u32(base + 0x3c);
    e.call(OPTION_LIST_UPDATE_CURRENT_SELECTION, &args![list]);
    race_sex_menu_set_flag(e, this, 0, false);
}

// Translated from 007af450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Refills the option list at `+0x48`: empties it, adds an option (id `0x21`)
/// for every node of the list at `011c3f2c` (head `0043c490`) whose item
/// passes `005fc4d0` and `005fc5f0` with the player's base form, updates the
/// list's selection and clears flag 1.
pub fn fn_007af450(e: &mut Engine, this: Ptr<RaceSexMenu>) {
    let base = this.addr();
    let list = e.mem.u32(base + 0x48);
    e.call(OPTION_LIST_CLEAR, &args![list]);
    let owner: u32 = e.global(MARKER_OWNER);
    let mut node = e.call(EYE_LIST_HEAD, &args![owner]).u32();
    while node != 0 {
        let slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![node]).u32();
        let item = e.mem.u32(slot);
        if e.call(EYE_CHECK_A, &args![item]).u8() != 0 {
            let player_base = e.mem.u32(base + 0xd8);
            let slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![node]).u32();
            let item = e.mem.u32(slot);
            if e.call(EYE_CHECK_B, &args![item, player_base]).u8() != 0 {
                let slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![node]).u32();
                let item = e.mem.u32(slot);
                let slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![node]).u32();
                let named = e.mem.u32(slot);
                let name = e.call(MARKER_NAME, &args![named + 0x18]).u32();
                let list = e.mem.u32(base + 0x48);
                let option = e
                    .call(OPTION_LIST_ADD_OPTION, &args![list, name, 0x21u32])
                    .u32();
                e.call(OPTION_SET_DATA, &args![option, item]);
            }
        }
        node = word_at_four(e, node);
    }
    let list = e.mem.u32(base + 0x48);
    e.call(OPTION_LIST_UPDATE_CURRENT_SELECTION, &args![list]);
    race_sex_menu_set_flag(e, this, 1, false);
}

// Translated from 007af520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::ValidateHairOptionList` (Xbox PDB): keeps the entry with id
/// 7 of the list at `+0x34` in step with the sex of the player's base form:
/// a male without the entry gets one inserted at position 2 (caption from
/// the setting at `011d23a0` and `0x3e` through `"%s %c"`, template
/// `RSM_list_item_template`); a female with the entry has it removed.
///
/// The compiler's exception-unwinding frame is not translated.
// Translation incomplete: 007af520 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary string is not translated
pub fn race_sex_menu_validate_hair_option_list(e: &mut Engine, this: Ptr<RaceSexMenu>) {
    let base = this.addr();
    let list = e.mem.u32(base + 0x34);
    let has_entry = e.call(OPTION_LIST_FIND, &args![list, 7u32]).u32() != 0;
    let player_base = e.mem.u32(base + 0xd8);
    let male = e.call(GET_SEX, &args![player_base]).u32() == 0;
    if male && !has_entry {
        // The exe's frame (`SUB ESP,0x10` under the exception record, then the record link):
        // the caption at `EBP-0x18` (+8).
        e.with_stack(0x20, |e, frame| {
            let text = frame.addr() + 8;
            e.call(STRING_CONSTRUCT, &args![text]);
            let caption = e.call(SETTING_VALUE, &args![SETTING_23A0]).u32();
            e.call(
                STRING_FORMAT,
                &args![text, CAPTION_FORMAT, caption, 0x3eu32],
            );
            let name = e.call(STRING_C_STR, &args![text]).u32();
            let list = e.mem.u32(base + 0x34);
            e.call(
                OPTION_LIST_INSERT_OPTION,
                &args![list, 2u32, name, LIST_ITEM_TEMPLATE, 7u32],
            );
            e.call(STRING_DESTROY, &args![text]);
        });
    } else if has_entry && !male {
        let list = e.mem.u32(base + 0x34);
        e.call(OPTION_LIST_REMOVE_OPTION, &args![list, 7u32]);
    }
}

// Translated from 007af610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::MakePresetHeadList` (Xbox PDB): rebuilds `xPresetHeadsA`
/// (+0xDC) for the player's base form (`00603280` with the form's `004ac110`
/// value, its sex and the array) and enables the option with id `0x18` of
/// the list at `+0x30` with the range `1` to the array's count when the
/// array is not empty (`0076b610` true disables it).
pub fn race_sex_menu_make_preset_head_list(e: &mut Engine, this: Ptr<RaceSexMenu>) {
    let base = this.addr();
    e.call(SIMPLE_ARRAY_CLEAR, &args![base + 0xdc, 0u32]);
    let player_base = e.mem.u32(base + 0xd8);
    let sex = e.call(GET_SEX, &args![player_base]).u32();
    let race = e
        .call(BASE_VALUE_4AC110, &args![e.mem.u32(base + 0xd8)])
        .u32();
    e.call(FILL_PRESET_HEADS, &args![race, sex, base + 0xdc]);
    let list = e.mem.u32(base + 0x30);
    let option = e.call(OPTION_LIST_FIND, &args![list, 0x18u32]).u32();
    if e.call(SIMPLE_ARRAY_IS_EMPTY, &args![base + 0xdc]).u8() != 0 {
        menu_option_set_enabled(e, Ptr::new(option), false);
    } else {
        menu_option_set_enabled(e, Ptr::new(option), true);
        let count = e.call(WORD_AT_EIGHT, &args![base + 0xdc]).u32();
        menu_option_set_range(e, Ptr::new(option), 1, count as i32);
    }
}

// Translated from 007af6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::UpdateScrollArrows` (Xbox PDB): walks the current list's
/// options (`007b39b0` says -1 for one above and 1 for one below the
/// visible part) and shows the up arrow (tile `+0x80`) and the down arrow
/// (tile `+0x84`) accordingly.
pub fn race_sex_menu_update_scroll_arrows(e: &mut Engine, this: Ptr<RaceSexMenu>) {
    let base = this.addr();
    let index = e.call(CURRENT_LIST_INDEX, &args![this]).u32();
    let list = option_list(e, base, index);
    let head = e.call(STRING_C_STR, &args![list]).u32();
    let mut above = false;
    let mut below = false;
    e.call(TILE_UPDATE_ALL, &args![0u32]);
    e.with_stack(4, |e, cursor| {
        e.mem.set_u32(cursor.addr(), head);
        while e.mem.u32(cursor.addr()) != 0 && (!above || !below) {
            let slot = e.call(LIST_ADVANCE, &args![list, cursor]).u32();
            let option = e.mem.u32(slot);
            match fn_007b39b0(e, Ptr::new(option)) {
                -1 => above = true,
                1 => below = true,
                _ => {}
            }
        }
    });
    let tile = e.mem.u32(base + 0x80);
    e.call(
        SET_TRAIT_FROM_INT,
        &args![tile, TRAIT_VISIBLE, above as u32],
    );
    let tile = e.mem.u32(base + 0x84);
    e.call(
        SET_TRAIT_FROM_INT,
        &args![tile, TRAIT_VISIBLE, below as u32],
    );
}
// Translated from 007af770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::OnFaceSliderChange` (Xbox PDB): for a changed face slider
/// `option`, reads the player base form's four face coordinate matrices
/// into a temporary array (`TESNPC::GetFaceCoord`), applies the slider's
/// value (`GetValue / 10.0`, the option's `+8` word and whether the option's
/// id is not `0x22`) to them (`00652320`), reads the coordinates again into
/// `LastFaceCoord` (+0x124), regenerates the face from the temporary
/// (`00652e00`) and sets flag 8 (and flag 9 when the option's id is `0x23`).
///
/// The compiler's exception-unwinding frame is not translated.
// Translation incomplete: 007af770 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary matrices is not translated
pub fn race_sex_menu_on_face_slider_change(e: &mut Engine, this: Ptr<RaceSexMenu>, option: u32) {
    let base = this.addr();
    // The exe's frame (`SUB ESP,0x8c` under the exception record, then the record link): the
    // four face coordinate blocks at `EBP-0x8c` (+0x10).
    e.with_stack(0x9c, |e, frame| {
        let coords = frame.addr() + 0x10;
        e.call(
            EH_VECTOR_CONSTRUCTOR_ITERATOR,
            &args![
                coords,
                0x20u32,
                4u32,
                FACE_COORD_CONSTRUCT,
                FACE_COORD_DESTRUCT
            ],
        );
        let player_base = e.mem.u32(base + 0xd8);
        e.call(GET_FACE_COORD, &args![player_base, coords]);
        let value = menu_option_get_value(e, Ptr::new(option));
        let scaled = (value as f64 / e.global::<f64>(TENTH_DIVISOR)) as f32;
        let word = e.call(WORD_AT_EIGHT, &args![option]).u32();
        let id = word_at_four(e, option);
        e.call(
            APPLY_FACE_SLIDER,
            &args![coords, id != 0x22, 0u32, word, scaled],
        );
        let player_base = e.mem.u32(base + 0xd8);
        e.call(GET_FACE_COORD, &args![player_base, base + 0x124]);
        let player_base = e.mem.u32(base + 0xd8);
        let first = e.call(FACE_VALUE_601800, &args![player_base]).u32();
        let player_base = e.mem.u32(base + 0xd8);
        let sex = e.call(GET_SEX, &args![player_base]).u32();
        let player_base = e.mem.u32(base + 0xd8);
        let race_word = word_at_four(e, player_base + 0x10c);
        let race = e.call(FACE_VALUE_5D9FB0, &args![race_word, sex]).u32();
        e.call(REGENERATE_FACE, &args![race, coords, first, 0u32]);
        race_sex_menu_set_flag(e, this, 8, true);
        if word_at_four(e, option) == 0x23 {
            race_sex_menu_set_flag(e, this, 9, true);
        }
        e.call(
            EH_VECTOR_DESTRUCTOR_ITERATOR,
            &args![coords, 0x20u32, 4u32, FACE_COORD_DESTRUCT],
        );
    });
}

// Translated from 007af900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::OnHairColorSliderChange` (Xbox PDB): for a non-null
/// `option`: if its id is `0x1d` (the colour choice), looks up the game
/// settings `iHairColorNN` and `sHairColorN` (N = the option's value),
/// sets the option's text to the second one's value and moves the three
/// colour sliders to the first one's value (`SetHairColorSliders`);
/// otherwise sets the text of the list's `0x1d` option (list at `+0x40`)
/// to the value of the setting at `011d3528`. Ends with `007b3660`.
///
/// The compiler's exception-unwinding frame is not translated.
// Translation incomplete: 007af900 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary string is not translated
pub fn race_sex_menu_on_hair_color_slider_change(
    e: &mut Engine,
    this: Ptr<RaceSexMenu>,
    option: u32,
) {
    let base = this.addr();
    if option == 0 {
        return;
    }
    if word_at_four(e, option) == 0x1d {
        // The exe's frame (`SUB ESP,0x18` under the exception record, then the record link): the
        // text at `EBP-0x20` (+8), the two found settings at `EBP-0x18` and `EBP-0x14` (+0x10).
        e.with_stack(0x28, |e, frame| {
            let text = frame.addr() + 8;
            let found = Ptr::<()>::new(frame.addr() + 0x10);
            let (color_setting, name_setting) = {
                e.mem.set_u32(found.addr(), 0);
                e.mem.set_u32(found.addr() + 4, 0);
                e.call(STRING_CONSTRUCT, &args![text]);
                let value = menu_option_get_value(e, Ptr::new(option));
                e.call(STRING_FORMAT, &args![text, FORMAT_HAIR_COLOR_INT, value]);
                let name = e.call(STRING_C_STR, &args![text]).u32();
                let settings = e.call(SETTINGS_COLLECTION, &args![]).u32();
                e.call(SETTINGS_FIND, &args![settings, name, found.addr() + 4]);
                let value = menu_option_get_value(e, Ptr::new(option));
                e.call(STRING_FORMAT, &args![text, FORMAT_HAIR_COLOR_NAME, value]);
                let name = e.call(STRING_C_STR, &args![text]).u32();
                let settings = e.call(SETTINGS_COLLECTION, &args![]).u32();
                e.call(SETTINGS_FIND, &args![settings, name, found.addr()]);
                (e.mem.u32(found.addr() + 4), e.mem.u32(found.addr()))
            };
            let caption = e.call(SETTING_VALUE, &args![name_setting]).u32();
            fn_007b3810(e, Ptr::new(option), caption);
            let slot = e.call(SETTING_INT_POINTER, &args![color_setting]).u32();
            let color = e.mem.u32(slot);
            race_sex_menu_set_hair_color_sliders(e, this, color);
            e.call(STRING_DESTROY, &args![text]);
        });
    } else {
        let caption = e.call(SETTING_VALUE, &args![SETTING_3528]).u32();
        let list = e.mem.u32(base + 0x40);
        let colour_option = e.call(OPTION_LIST_FIND, &args![list, 0x1du32]).u32();
        fn_007b3810(e, Ptr::new(colour_option), caption);
    }
    fn_007b3660(e, this);
}

// Translated from 007afa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::SetHairColorSliders` (Xbox PDB): sets the options with id
/// `0x1e`, `0x1f` and `0x20` of the list at `+0x40` to the red, green and
/// blue byte of `color`.
pub fn race_sex_menu_set_hair_color_sliders(e: &mut Engine, this: Ptr<RaceSexMenu>, color: u32) {
    let list = e.mem.u32(this.addr() + 0x40);
    let head = e.call(STRING_C_STR, &args![list]).u32();
    e.with_stack(4, |e, cursor| {
        e.mem.set_u32(cursor.addr(), head);
        while e.mem.u32(cursor.addr()) != 0 {
            let slot = e.call(LIST_ADVANCE, &args![list, cursor]).u32();
            let option = e.mem.u32(slot);
            let value = match word_at_four(e, option) {
                0x1e => color & 0xff,
                0x1f => (color & 0xffff) >> 8 & 0xff,
                0x20 => color >> 16 & 0xff,
                _ => continue,
            };
            menu_option_set_value(e, Ptr::new(option), value as i32);
        }
    });
}
/// Where an `AddFaceSliders` table entry takes a bound from: 0.0, the float
/// of a game setting (`00450410`, setting in ECX) or a float constant in
/// the exe's data (copied through the x87 stack).
#[derive(Clone, Copy)]
enum Float {
    Zero,
    Setting(u32),
    Global(u32),
}

/// One `AddFaceSliders` table entry as the code builds it: the option list
/// number (0x14 = none), the caption setting (0 = no caption), the lower and
/// the upper bound.
type ShapeRow = (u32, u32, Float, Float);

/// The 87 table entries `AddFaceSliders` builds on its stack, in the order
/// it builds them. Entries 0 to 54 are the first 55 slots (16 bytes each);
/// the last 32 are the slots 63 to 94, the slots 55 to 62 being taken by the
/// face coordinate array (`frame` memory overlap, see the function).
const SHAPE_ROWS: [ShapeRow; 87] = [
    (
        0xc,
        0x011d_1ecc,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 0
    (
        0xc,
        0x011d_3504,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 1
    (
        0xc,
        0x011d_3840,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 2
    (
        0x10,
        0x011d_4b34,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 3
    (
        0x10,
        0x011d_228c,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 4
    (0x14, 0, Float::Zero, Float::Zero), // 5
    (
        0x10,
        0x011d_32dc,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 6
    (0x14, 0, Float::Zero, Float::Zero), // 7
    (
        0x12,
        0x011d_46f0,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 8
    (
        0x12,
        0x011d_4714,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 9
    (0x14, 0, Float::Zero, Float::Zero), // 10
    (
        0x12,
        0x011d_3150,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 11
    (0x14, 0, Float::Zero, Float::Zero), // 12
    (
        0x12,
        0x011d_3a8c,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 13
    (0x14, 0, Float::Zero, Float::Zero), // 14
    (
        0xd,
        0x011d_4384,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 15
    (
        0xd,
        0x011d_4e88,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 16
    (0x14, 0, Float::Zero, Float::Zero), // 17
    (
        0xd,
        0x011d_49f0,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 18
    (0x14, 0, Float::Zero, Float::Zero), // 19
    (0x14, 0, Float::Zero, Float::Zero), // 20
    (
        0xa,
        0x011d_2ca0,
        Float::Global(0x0104_a494),
        Float::Global(0x0101_62c0),
    ), // 21
    (0x14, 0, Float::Zero, Float::Zero), // 22
    (
        0xa,
        0x011d_35a0,
        Float::Global(0x0104_a494),
        Float::Global(0x0101_62c0),
    ), // 23
    (
        0xb,
        0x011d_4e88,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 24
    (
        0xb,
        0x011d_3a8c,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 25
    (
        0xb,
        0x011d_3720,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 26
    (
        0x11,
        0x011d_240c,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 27
    (
        0x11,
        0x011d_2514,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 28
    (
        0x11,
        0x011d_2778,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 29
    (
        0x11,
        0x011d_32dc,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 30
    (0x14, 0, Float::Zero, Float::Zero), // 31
    (0x14, 0, Float::Zero, Float::Zero), // 32
    (0x14, 0, Float::Zero, Float::Zero), // 33
    (0x14, 0, Float::Zero, Float::Zero), // 34
    (
        0xf,
        0x011d_34a4,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 35
    (0x14, 0, Float::Zero, Float::Zero), // 36
    (0x14, 0, Float::Zero, Float::Zero), // 37
    (0x14, 0, Float::Zero, Float::Zero), // 38
    (
        0xf,
        0x011d_22e0,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 39
    (0x14, 0, Float::Zero, Float::Zero), // 40
    (
        0xe,
        0x011d_3660,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 41
    (0x14, 0, Float::Zero, Float::Zero), // 42
    (
        0xe,
        0x011d_4384,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 43
    (
        0xe,
        0x011d_2e44,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 44
    (
        0xe,
        0x011d_2dfc,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 45
    (0x14, 0, Float::Zero, Float::Zero), // 46
    (
        0xe,
        0x011d_4900,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 47
    (0x14, 0, Float::Zero, Float::Zero), // 48
    (
        0xe,
        0x011d_23e8,
        Float::Setting(0x011d_a6a4),
        Float::Global(0x0101_7718),
    ), // 49
    (0x14, 0, Float::Zero, Float::Zero), // 50
    (0x14, 0, Float::Zero, Float::Zero), // 51
    (0x14, 0, Float::Zero, Float::Zero), // 52
    (
        0xe,
        0x011d_32a0,
        Float::Setting(0x011d_a6a4),
        Float::Global(0x0101_7718),
    ), // 53
    (0x14, 0, Float::Zero, Float::Zero), // 54
    (0x14, 0, Float::Zero, Float::Zero), // 63
    (0x14, 0, Float::Zero, Float::Zero), // 64
    (0x14, 0, Float::Zero, Float::Zero), // 65
    (
        0x13,
        0x011d_2b80,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 66
    (0x14, 0, Float::Zero, Float::Zero), // 67
    (
        0x13,
        0x011d_3b80,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 68
    (
        0x13,
        0x011d_3078,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 69
    (0x14, 0, Float::Zero, Float::Zero), // 70
    (
        0x13,
        0x011d_4174,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 71
    (
        0x13,
        0x011d_2304,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 72
    (0x14, 0, Float::Zero, Float::Zero), // 73
    (0x14, 0, Float::Zero, Float::Zero), // 74
    (0x14, 0, Float::Zero, Float::Zero), // 75
    (0x14, 0, Float::Zero, Float::Zero), // 76
    (0x14, 0, Float::Zero, Float::Zero), // 77
    (0x14, 0, Float::Zero, Float::Zero), // 78
    (0x14, 0, Float::Zero, Float::Zero), // 79
    (0x14, 0, Float::Zero, Float::Zero), // 80
    (
        0x13,
        0x011d_4300,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 81
    (0x14, 0, Float::Zero, Float::Zero), // 82
    (0x14, 0, Float::Zero, Float::Zero), // 83
    (0x14, 0, Float::Zero, Float::Zero), // 84
    (
        0x13,
        0x011d_4c30,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 85
    (0x14, 0, Float::Zero, Float::Zero), // 86
    (0x14, 0, Float::Zero, Float::Zero), // 87
    (0x14, 0, Float::Zero, Float::Zero), // 88
    (0x14, 0, Float::Zero, Float::Zero), // 89
    (
        0x13,
        0x011d_4d20,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 90
    (
        0x13,
        0x011d_5104,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 91
    (
        0x13,
        0x011d_33e4,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 92
    (
        0x13,
        0x011d_2928,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 93
    (
        0x13,
        0x011d_5098,
        Float::Setting(0x011d_a6a4),
        Float::Setting(0x011d_a6dc),
    ), // 94
];

/// The second group of `AddFaceSliders` sliders: the shape number passed to
/// `00652230` (and stored as the slider's data) and the index of the table
/// entry (slot number) holding the list, caption and bounds.
const EXTRA_SLIDERS: [(u32, usize); 12] = [
    (0x1d, 92),
    (0x1c, 91),
    (0x1e, 93),
    (0x1f, 94),
    (8, 71),
    (9, 72),
    (0x12, 81),
    (0x1b, 90),
    (0x16, 85),
    (5, 68),
    (6, 69),
    (3, 66),
];

/// Reads a bound of a table entry.
fn shape_bound(e: &mut Engine, source: Float) -> f32 {
    match source {
        Float::Zero => 0.0,
        Float::Setting(setting) => e.call(FLOAT_SETTING_GETTER, &args![setting]).f32(),
        Float::Global(address) => x87_copy_f32(e, address),
    }
}

/// Adds one face slider for the table entry at `entry` (address) whose
/// shape value is `value`: clamps the value to the bounds, scales all three
/// by 10 and adds a slider option with `slider_id` to the entry's list,
/// whose data is `data`.
fn add_face_slider(
    e: &mut Engine,
    this: Ptr<RaceSexMenu>,
    entry: u32,
    value: f32,
    slider_id: u32,
    data: u32,
) {
    let lower = e.mem.f32(entry + 8);
    let upper = e.mem.f32(entry + 0xc);
    let capped = e.call(FLOAT_MIN, &args![value, upper]).f32();
    let value = e.call(FLOAT_MAX, &args![capped, lower]).f32();
    let step = x87_copy_f32(e, SLIDER_STEP_QUARTER);
    let ten: f64 = e.global(TENTH_DIVISOR);
    let value_scaled = e.call(FTOL, &args![value as f64 * ten]).i32();
    let upper_scaled = e.call(FTOL, &args![upper as f64 * ten]).i32();
    let lower_scaled = e.call(FTOL, &args![lower as f64 * ten]).i32();
    let text = e.mem.u32(entry + 4);
    let list_index = e.mem.u32(entry);
    let list = e.mem.u32(this.addr() + 0x28 + 4 * list_index);
    let option = e
        .call(
            OPTION_LIST_ADD_SLIDER,
            &args![
                list,
                text,
                slider_id,
                lower_scaled,
                upper_scaled,
                value_scaled,
                step
            ],
        )
        .u32();
    fn_007b3810(e, Ptr::new(option), SPACE_STRING);
    e.call(OPTION_SET_DATA, &args![option, data]);
}

// Translated from 007afaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::AddFaceSliders` (Xbox PDB): adds the face shape sliders.
///
/// It builds on its stack a table of 87 entries (option list number, caption
/// setting text, lower and upper bound; the bounds from the settings at
/// `011da6a4` and `011da6dc`, or from float constants) and the player base
/// form's face coordinate array (four matrices, `601800`/`5d9fb0`/`652af0`).
/// For every shape number below the shape count (`006521b0`) whose entry has
/// a list below 0x14 and a caption, it reads the shape's value (`00652230`),
/// clamps it to the entry's bounds and adds a slider (id `0x22`) to the
/// entry's list; then twelve more sliders (id `0x23`, `EXTRA_SLIDERS`) from
/// further entries. It ends with `007b18f0(0x22)` and `007b18f0(0x23)`, runs
/// `007b1e70(1, 1, 0)` when flag 8 is set and `007b25a0` when flag 9 is set
/// (clearing the flags).
///
/// The game keeps the table and the coordinate array in one stack frame; the
/// array lies over the slots 55 to 62, so `frame` mirrors that layout. The
/// scaling by 10 multiplies in `f64` where the x87 uses extended precision.
/// The compiler's exception-unwinding frame is not translated.
// Translation incomplete: 007afaf0 - the C++ exception-unwinding frame (FS:[0]) that destroys the coordinate array is not translated
pub fn race_sex_menu_add_face_sliders(e: &mut Engine, this: Ptr<RaceSexMenu>) {
    // The exe's frame (`SUB ESP,0x660` under the exception record, then the record link):
    // `frame + 0x74 + 16 * slot` is slot `slot` of the table, `frame + 0x3e4` the coordinate
    // array (the game's `EBP - 0x5fc` and `EBP - 0x28c`).
    e.with_stack(0x670, |e, frame| {
        let frame = frame.addr();
        let slot_address = |slot: usize| frame + 0x74 + 16 * slot as u32;
        for (index, (list, text, lower, upper)) in SHAPE_ROWS.iter().enumerate() {
            let slot = if index < 55 { index } else { index + 8 };
            let entry = slot_address(slot);
            e.mem.set_u32(entry, *list);
            let caption = if *text != 0 {
                e.call(SETTING_VALUE, &args![*text]).u32()
            } else {
                0
            };
            e.mem.set_u32(entry + 4, caption);
            let value = shape_bound(e, *lower);
            e.mem.set_f32(entry + 8, value);
            let value = shape_bound(e, *upper);
            e.mem.set_f32(entry + 0xc, value);
        }
        let coords = frame + 0x3e4;
        e.call(
            EH_VECTOR_CONSTRUCTOR_ITERATOR,
            &args![
                coords,
                0x20u32,
                4u32,
                FACE_COORD_CONSTRUCT,
                FACE_COORD_DESTRUCT
            ],
        );
        let player_base = e.mem.u32(this.addr() + 0xd8);
        let first = e.call(FACE_VALUE_601800, &args![player_base]).u32();
        let player_base = e.mem.u32(this.addr() + 0xd8);
        let sex = e.call(GET_SEX, &args![player_base]).u32();
        let player_base = e.mem.u32(this.addr() + 0xd8);
        let race_word = word_at_four(e, player_base + 0x10c);
        let race = e.call(FACE_VALUE_5D9FB0, &args![race_word, sex]).u32();
        e.call(FACE_FILL_COORDS, &args![race, first, coords, 0u32, 0.0f32]);
        let mut shape = 0i32;
        loop {
            e.call(FACE_GEN_MANAGER_INSTANCE, &args![]);
            let count = e.call(SHAPE_COUNT, &args![0u32, 0u32]).i32();
            if shape >= count {
                break;
            }
            let entry = frame
                .wrapping_add(0x74)
                .wrapping_add((shape as u32).wrapping_mul(16));
            if (e.mem.i32(entry)) < 0x14 && e.mem.u32(entry + 4) != 0 {
                let value = e
                    .call(FACE_SHAPE_VALUE, &args![coords, 0u32, 0u32, shape])
                    .f32();
                add_face_slider(e, this, entry, value, 0x22, shape as u32);
            }
            shape += 1;
        }
        for (selector, slot) in EXTRA_SLIDERS {
            let value = e
                .call(FACE_SHAPE_VALUE, &args![coords, 1u32, 0u32, selector])
                .f32();
            add_face_slider(e, this, slot_address(slot), value, 0x23, selector);
        }
        fn_007b18f0(e, this, 0x22);
        fn_007b18f0(e, this, 0x23);
        if race_sex_menu_get_flag(e, this, 8) {
            fn_007b1e70(e, this, true, true, false);
            race_sex_menu_set_flag(e, this, 8, false);
        }
        if race_sex_menu_get_flag(e, this, 9) {
            fn_007b25a0(e, this);
            race_sex_menu_set_flag(e, this, 9, false);
        }
        e.call(
            EH_VECTOR_DESTRUCTOR_ITERATOR,
            &args![coords, 0x20u32, 4u32, FACE_COORD_DESTRUCT],
        );
    });
}
// Translated from 007b16d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Re-reads the face shape slider options (ids `0x22` and `0x23`) of the
/// option lists `from` up to (not including) `to`: for each, it reads the
/// shape value for `selector` (`00652230` on the player base form's face
/// coordinates, scaled by 10 and truncated) and pulls an option whose value
/// is out of its range back to the bound (returning true), or, when it is
/// not the dragged option (`+0xC4`), sets the option to the value.
///
/// The compiler's exception-unwinding frame is not translated.
// Translation incomplete: 007b16d0 - the C++ exception-unwinding frame (FS:[0]) that destroys the coordinate array is not translated
pub fn fn_007b16d0(
    e: &mut Engine,
    this: Ptr<RaceSexMenu>,
    from: i32,
    to: i32,
    selector: u32,
) -> bool {
    let base = this.addr();
    // The exe's frame (`SUB ESP,0xa0` under the exception record, then the record link): the
    // list cursor at `EBP-0x18` (+0x98), the four face coordinate blocks at `EBP-0x9c` (+0x14),
    // the slider range at `EBP-0xa4` and `EBP-0xa0` (+0xc).
    e.with_stack(0xb0, |e, frame| {
        let mut changed = false;
        let mut list_index = from;
        while list_index < to {
            let list = option_list(e, base, list_index as u32);
            let mut cursor = e.call(STRING_C_STR, &args![list]).u32();
            while cursor != 0 {
                let option = {
                    let slot = frame.addr() + 0x98;
                    e.mem.set_u32(slot, cursor);
                    let found = e.call(LIST_ADVANCE, &args![list, slot]).u32();
                    cursor = e.mem.u32(slot);
                    e.mem.u32(found)
                };
                if option == 0 {
                    continue;
                }
                if word_at_four(e, option) != 0x22 && word_at_four(e, option) != 0x23 {
                    continue;
                }
                let coords = frame.addr() + 0x14;
                e.call(
                    EH_VECTOR_CONSTRUCTOR_ITERATOR,
                    &args![
                        coords,
                        0x20u32,
                        4u32,
                        FACE_COORD_CONSTRUCT,
                        FACE_COORD_DESTRUCT
                    ],
                );
                let player_base = e.mem.u32(base + 0xd8);
                let first = e.call(FACE_VALUE_601800, &args![player_base]).u32();
                let player_base = e.mem.u32(base + 0xd8);
                let sex = e.call(GET_SEX, &args![player_base]).u32();
                let player_base = e.mem.u32(base + 0xd8);
                let race_word = word_at_four(e, player_base + 0x10c);
                let race = e.call(FACE_VALUE_5D9FB0, &args![race_word, sex]).u32();
                e.call(FACE_FILL_COORDS, &args![race, first, coords, 0u32, 0.0f32]);
                let shape = e.call(WORD_AT_EIGHT, &args![option]).u32();
                let value = e
                    .call(FACE_SHAPE_VALUE, &args![coords, selector, 0u32, shape])
                    .f32();
                let ten: f64 = e.global(TENTH_DIVISOR);
                let value = e.call(FTOL, &args![value as f64 * ten]).i32();
                let (min, max) = {
                    let range = frame.addr() + 0xc;
                    fn_007b3840(e, Ptr::new(option), range, range + 4);
                    (e.mem.i32(range), e.mem.i32(range + 4))
                };
                if value < min {
                    menu_option_set_value(e, Ptr::new(option), min);
                    changed = true;
                } else if value > max {
                    menu_option_set_value(e, Ptr::new(option), max);
                    changed = true;
                } else if option != e.mem.u32(base + 0xc4) {
                    menu_option_set_value(e, Ptr::new(option), value);
                }
                e.call(
                    EH_VECTOR_DESTRUCTOR_ITERATOR,
                    &args![coords, 0x20u32, 4u32, FACE_COORD_DESTRUCT],
                );
            }
            list_index += 1;
        }
        changed
    })
}

// Translated from 007b18f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// For `which` `0x22` (lists 10 to 18, selector 0) or `0x23` (list 19,
/// selector 1): runs `007b16d0` again while it changes something, at most
/// four times. Other values do nothing.
pub fn fn_007b18f0(e: &mut Engine, this: Ptr<RaceSexMenu>, which: u32) {
    let (from, to, selector) = match which {
        0x22 => (0xa, 0x13, 0),
        0x23 => (0x13, 0x14, 1),
        _ => return,
    };
    let mut counter = 0u32;
    loop {
        if !fn_007b16d0(e, this, from, to, selector) {
            break;
        }
        let before = counter;
        counter += 1;
        if before >= 3 {
            break;
        }
    }
}

// Translated from 007b1970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::RotatePlayer` (Xbox PDB): turns the player node (`+0xD4`)
/// about the vertical axis by `angle`. Reads the node's rotation matrix,
/// takes its Euler angles, subtracts `angle` from the third and wraps the
/// result into 0 to 2 pi; when the angle did not change and `force` is
/// false returns 0. Otherwise rebuilds the rotation (`004a0c90`), scales it
/// by the base form's weight and height values, sets it on the node and
/// runs `007b1ae0`; returns 1.
pub fn race_sex_menu_rotate_player(
    e: &mut Engine,
    this: Ptr<RaceSexMenu>,
    angle: f32,
    force: u32,
) -> u8 {
    let base = this.addr();
    let node = e.call(STRING_C_STR, &args![base + 0xd4]).u32();
    let rotation = e.call(GET_ROTATE, &args![node]).u32();
    // The exe's frame (`SUB ESP,0x80` under `EBP`, then `ESI` and `EDI`): the matrix at
    // `EBP-0x24` (+0x64), the scale at `EBP-0x58` (+0x30), the product at `EBP-0x7c` (+0xc) and
    // the three angles at `EBP-0x34`, `EBP-0x2c` and `EBP-0x30` (+0x54, +0x5c, +0x58).
    e.with_stack(0x88, |e, frame| {
        let matrix = frame.addr() + 0x64;
        let scale = frame.addr() + 0x30;
        let product = frame.addr() + 0xc;
        let angles = [0x54, 0x5c, 0x58].map(|offset| frame.addr() + offset);
        copy_words(e, rotation, matrix, 9);
        e.call(
            TO_EULER_ANGLES,
            &args![matrix, angles[0], angles[1], angles[2]],
        );
        let current = e.mem.f32(angles[2]);
        let two_pi: f64 = e.global(TWO_PI_DOUBLE);
        let zero: f64 = e.global(ZERO_DOUBLE);
        let mut wrapped = (current as f64 - angle as f64) as f32;
        if (wrapped as f64) < zero {
            wrapped = (wrapped as f64 + two_pi) as f32;
        } else if wrapped as f64 > two_pi {
            wrapped = (wrapped as f64 - two_pi) as f32;
        }
        if current == wrapped && force as u8 == 0 {
            0
        } else {
            e.call(MATRIX_SET_ROTATION, &args![matrix, wrapped]);
            copy_words(e, IDENTITY_MATRIX, scale, 9);
            for (row, getter) in [
                (0u32, PLAYER_BASE_WEIGHT),
                (1, PLAYER_BASE_WEIGHT),
                (2, PLAYER_BASE_HEIGHT),
            ] {
                let player_base = e.mem.u32(base + 0xd8);
                let value = e.call(getter, &args![player_base]).f32();
                e.call(MATRIX_SET_ELEMENT, &args![scale, row, row, value]);
            }
            let multiplied = e
                .call(MATRIX_MULTIPLY, &args![matrix, product, scale])
                .u32();
            copy_words(e, multiplied, matrix, 9);
            let node = e.call(STRING_C_STR, &args![base + 0xd4]).u32();
            e.call(SET_ROTATE, &args![node, matrix]);
            fn_007b1ae0(e, this);
            1
        }
    })
}

// Translated from 007b1ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Refreshes the player's lighting and the rendered menu after the player
/// node moved: `ShadowSceneNode::UpdateObjectLighting` on the scene node 3
/// with the player's `00950bb0(0)` object, sets the translation of scene
/// node 3 to (0, 0, 0), and, when the interface has a rendered menu,
/// calls `FORenderedMenuRaceSex` `fn_007fd660` with 1.0.
pub fn fn_007b1ae0(e: &mut Engine, _this: Ptr<RaceSexMenu>) {
    let player: u32 = e.global(PLAYER);
    let object = e.call(PLAYER_NODE_GETTER, &args![player, 0u32]).u32();
    let scene = e.call(SCENE_NODE_GETTER, &args![3u32]).u32();
    // 007b1b05: of the two 0s pushed first, `00950bb0` (RET 4) takes one; the other is
    // the last word of `00b5d9f0` (RET 8).
    e.call(UPDATE_OBJECT_LIGHTING, &args![scene, object, 0u32]);
    // The exe's frame (`SUB ESP,0x10` under `EBP`): the point at `EBP-0xc` (+4).
    e.with_stack(0x10, |e, frame| {
        let point = frame.addr() + 4;
        e.call(POINT_CONSTRUCT, &args![point, 0.0f32, 0u32, 0u32]);
        let scene = e.call(SCENE_NODE_GETTER, &args![3u32]).u32();
        e.call(SET_TRANSLATE_POINT, &args![scene, point]);
    });
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    if e.call(RENDERED_MENU_GETTER, &args![interface]).u32() != 0 {
        let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
        let rendered = e.call(RENDERED_MENU_GETTER, &args![interface]).u32();
        e.call(RENDERED_MENU_SET_SCALE, &args![rendered, 1.0f32]);
    }
}

// Translated from 007b1b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::ZoomPlayer` (Xbox PDB): moves the zoom position (`+0xD0`)
/// by `delta`, keeping it between 0 and 1. Returns 0 when it did not change
/// (and `force` is false). Otherwise places the player node at the point
/// between the two zoom bounds (`+0xA0` and `+0xAC`) that the zoom selects,
/// the height scaled by the player's `Actor::GetHeight`, runs `007b1ae0`
/// and returns 1.
pub fn race_sex_menu_zoom_player(
    e: &mut Engine,
    this: Ptr<RaceSexMenu>,
    delta: f32,
    force: u32,
) -> u8 {
    let base = this.addr();
    let old = e.mem.f32(base + 0xd0);
    let sum = (old as f64 + delta as f64) as f32;
    let limited = e.call(FLOAT_MIN, &args![1.0f32, sum]).f32();
    let zoom = e.call(FLOAT_MAX, &args![0.0f32, limited]).f32();
    e.mem.set_f32(base + 0xd0, zoom);
    if old == zoom && force as u8 == 0 {
        return 0;
    }
    let player: u32 = e.global(PLAYER);
    let height = e.call(ACTOR_GET_HEIGHT, &args![player]).f32();
    let bound = |e: &Engine, offset: u32| e.mem.f32(base + offset) as f64;
    let zoom = e.mem.f32(base + 0xd0) as f64;
    let z = (((bound(e, 0xb4) - bound(e, 0xa8)) * zoom + bound(e, 0xa8)) * height as f64) as f32;
    let y = ((bound(e, 0xb0) - bound(e, 0xa4)) * zoom + bound(e, 0xa4)) as f32;
    let x = ((bound(e, 0xac) - bound(e, 0xa0)) * zoom + bound(e, 0xa0)) as f32;
    let node = e.call(STRING_C_STR, &args![base + 0xd4]).u32();
    e.call(SET_TRANSLATE, &args![node, x, y, z]);
    fn_007b1ae0(e, this);
    1
}

// Translated from 007b1ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// After the sex or race changed: when the player base form's `00603b50`
/// value does not pass `005fdfa0`, picks a replacement from the race's list
/// (`00613870` first, then the first list entry that passes `005fdf40` and
/// the sex-specific `005fdf60` / `005fdf80`) and sets it (`006031e0`, then
/// `007b2b50(1)`); when `00602150` of the base form does not pass
/// `005fc5f0`, sets `007b1e50` of the race's value as `00603200` and runs
/// `007b27c0`. Sets flags 0 and 1.
pub fn fn_007b1ca0(e: &mut Engine, this: Ptr<RaceSexMenu>) {
    let base = this.addr();
    let player_base = e.mem.u32(base + 0xd8);
    let current = e.call(BASE_VALUE_603B50, &args![player_base]).u32();
    if e.call(HAIR_CHECK_B, &args![current, player_base]).u8() == 0 {
        let player_base = e.mem.u32(base + 0xd8);
        let sex = e.call(GET_SEX, &args![player_base]).u32();
        let race = e
            .call(BASE_VALUE_4AC110, &args![e.mem.u32(base + 0xd8)])
            .u32();
        let mut candidate = e.call(RACE_DEFAULT_HAIR, &args![race, sex]).u32();
        let race = e
            .call(BASE_VALUE_4AC110, &args![e.mem.u32(base + 0xd8)])
            .u32();
        let mut node = e.call(RACE_HAIR_LIST_HEAD, &args![race]).u32();
        while candidate == 0 && node != 0 {
            let slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![node]).u32();
            if e.mem.u32(slot) == 0 {
                break;
            }
            let slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![node]).u32();
            let item = e.mem.u32(slot);
            if e.call(HAIR_CHECK_A, &args![item]).u8() != 0 {
                let player_base = e.mem.u32(base + 0xd8);
                let mut take = false;
                if e.call(GET_SEX, &args![player_base]).u32() == 0 {
                    let slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![node]).u32();
                    let item = e.mem.u32(slot);
                    take = e.call(HAIR_CHECK_MALE, &args![item]).u8() != 0;
                }
                if !take {
                    let player_base = e.mem.u32(base + 0xd8);
                    if e.call(GET_SEX, &args![player_base]).u32() == 1 {
                        let slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![node]).u32();
                        let item = e.mem.u32(slot);
                        take = e.call(HAIR_CHECK_FEMALE, &args![item]).u8() != 0;
                    }
                }
                if take {
                    let slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![node]).u32();
                    candidate = e.mem.u32(slot);
                }
            }
            node = word_at_four(e, node);
        }
        let player_base = e.mem.u32(base + 0xd8);
        e.call(SET_HAIR, &args![player_base, candidate]);
        fn_007b2b50(e, this, true);
    }
    let player_base = e.mem.u32(base + 0xd8);
    let current = e.call(BASE_VALUE_602150, &args![player_base]).u32();
    if e.call(EYE_CHECK_B, &args![current, player_base]).u8() == 0 {
        let race = e
            .call(BASE_VALUE_4AC110, &args![e.mem.u32(base + 0xd8)])
            .u32();
        let value = fn_007b1e50(e, race);
        let player_base = e.mem.u32(base + 0xd8);
        e.call(SET_HEAD, &args![player_base, value]);
        fn_007b27c0(e, this);
    }
    race_sex_menu_set_flag(e, this, 0, true);
    race_sex_menu_set_flag(e, this, 1, true);
}

// Translated from 007b1e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the first word of the object at `+0xA8` of its `this` (through
/// `006815c0`, which returns its own `this`).
pub fn fn_007b1e50(e: &mut Engine, this: u32) -> u32 {
    let slot = e.call(EMPTY_MEMBER_RETURNS_THIS, &args![this + 0xa8]).u32();
    e.mem.u32(slot)
}

// Translated from 007b1e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates the player's 3D after a face change. The player's 3D accessors
/// are slots `+0x1ac` and `+0x1b0` of its vtable (null: nothing happens).
/// It first asks the player base form's slot `+0x48` with `0x800`.
///
/// With `reset` set: removes the facegen objects of the two 3D roots from
/// the object palette, tells the player's saved object (`008d8520`) slots
/// `+0x79c`, `+0x794` and `+0x7a4` and the roots' `009611e0` objects slot
/// `+0xe8`, clears the head (`TESNPC::ClearHead`, `InitHead`, `00607420`
/// with the biped).
///
/// Without it: reads the base form's face coordinates twice, and for the
/// two roots and each of their children that have a head object (`+0x94`
/// of the cast `00607310`): applies the coordinates (`apply`, `0065a970`)
/// and, when `retexture` is set and the child's name is not found by
/// `004564f0`, applies the coordinate texturing (`0065b410`) to the mesh
/// shader property when it is in world range 8 to 12.
///
/// Always ends with `007b1ae0` (except when the player has no 3D).
///
/// The compiler's exception-unwinding frame is not translated.
// Translation incomplete: 007b1e70 - the C++ exception-unwinding frame (FS:[0]) that destroys the coordinate arrays and the temporary pointer is not translated
pub fn fn_007b1e70(
    e: &mut Engine,
    this: Ptr<RaceSexMenu>,
    reset: bool,
    apply: bool,
    retexture: bool,
) {
    let base = this.addr();
    let player: u32 = e.global(PLAYER);
    if e.vcall(player, 0x1ac, &args![0u32]).u32() == 0
        || e.vcall(player, 0x1b0, &args![0u32]).u32() == 0
    {
        return;
    }
    let player_base = e.mem.u32(base + 0xd8);
    e.vcall(player_base, 0x48, &args![0x800u32]);
    // The exe's frame (`SUB ESP,0x17c` under the exception record, then the record link): the
    // head pair at `EBP-0x1c` (+0x170) and `EBP-0x10` (+0x17c), the face coordinates at
    // `EBP-0x12c` (+0x60) and the applied ones at `EBP-0xac` (+0xe0), the texture pointer at
    // `EBP-0x154` (+0x38).
    e.with_stack(0x18c, |e, frame| {
        if reset {
            let root = e.call(PLAYER_NODE_GETTER, &args![player, 0u32]).u32();
            let object = if root != 0 {
                e.vcall(root, 0xc, &args![]).u32()
            } else {
                0
            };
            if object != 0 {
                let mut palette = 0u32;
                let animation = e.call(PLAYER_ANIMATION, &args![player, 0u32]).u32();
                if animation != 0 {
                    let animation = e.call(PLAYER_ANIMATION, &args![player, 0u32]).u32();
                    let held = e.call(ANIMATION_OBJECT, &args![animation]).u32();
                    if held != 0 {
                        let animation = e.call(PLAYER_ANIMATION, &args![player, 0u32]).u32();
                        let held = e.call(ANIMATION_OBJECT, &args![animation]).u32();
                        palette = e.call(PALETTE_OF, &args![held]).u32();
                    }
                }
                let third = e.vcall(player, 0x1b0, &args![0u32]).u32();
                if third != 0 && e.call(OBJECT_9611E0, &args![third]).u32() != 0 {
                    e.call(REMOVE_FROM_PALETTE, &args![third, palette]);
                    let saved = e.call(SAVED_OBJECT, &args![player]).u32();
                    e.vcall(saved, 0x79c, &args![0u32]);
                    let owner = e.call(OBJECT_9611E0, &args![third]).u32();
                    e.vcall(owner, 0xe8, &args![third]);
                }
                let first = e.vcall(player, 0x1ac, &args![0u32]).u32();
                if first != 0 && e.call(OBJECT_9611E0, &args![first]).u32() != 0 {
                    e.call(REMOVE_FROM_PALETTE, &args![first, palette]);
                    let saved = e.call(SAVED_OBJECT, &args![player]).u32();
                    e.vcall(saved, 0x794, &args![0u32]);
                    let saved = e.call(SAVED_OBJECT, &args![player]).u32();
                    e.vcall(saved, 0x7a4, &args![0u32]);
                    let owner = e.call(OBJECT_9611E0, &args![first]).u32();
                    e.vcall(owner, 0xe8, &args![first]);
                }
            }
            if e.call(SAVED_OBJECT, &args![player]).u32() != 0 {
                let saved = e.call(SAVED_OBJECT, &args![player]).u32();
                e.vcall(saved, 0x58, &args![]);
            }
            let player_base = e.mem.u32(base + 0xd8);
            e.call(CLEAR_HEAD, &args![player_base]);
            let (first, second) = {
                let (first_at, second_at) = (frame.addr() + 0x170, frame.addr() + 0x17c);
                e.mem.set_u32(second_at, 0);
                e.mem.set_u32(first_at, 0);
                let player_base = e.mem.u32(base + 0xd8);
                e.call(INIT_HEAD, &args![player_base, first_at, second_at]);
                (e.mem.u32(first_at), e.mem.u32(second_at))
            };
            let biped = e.call(PLAYER_BIPED, &args![player, 0u32]).u32();
            let player_base = e.mem.u32(base + 0xd8);
            e.call(
                HEAD_BUILD,
                &args![player_base, player, biped, first, second],
            );
        } else {
            let coords = frame.addr() + 0x60;
            let applied = frame.addr() + 0xe0;
            e.call(
                EH_VECTOR_CONSTRUCTOR_ITERATOR,
                &args![
                    coords,
                    0x20u32,
                    4u32,
                    FACE_COORD_CONSTRUCT,
                    FACE_COORD_DESTRUCT
                ],
            );
            e.call(
                EH_VECTOR_CONSTRUCTOR_ITERATOR,
                &args![
                    applied,
                    0x20u32,
                    4u32,
                    FACE_COORD_CONSTRUCT,
                    FACE_COORD_DESTRUCT
                ],
            );
            let player_base = e.mem.u32(base + 0xd8);
            e.call(GET_FACE_COORD, &args![player_base, coords]);
            if apply {
                e.call(REGENERATE_FACE, &args![base + 0x124, coords, applied, 0u32]);
            }
            let mut root = e.vcall(player, 0x1ac, &args![0u32]).u32();
            for _ in 0..2 {
                if apply {
                    fn_007b2550(e, root, 1);
                }
                let count = e.call(CHILD_COUNT, &args![root]).u32();
                for index in 0..count {
                    let child = e.call(CHILD_AT, &args![root, index]).u32();
                    let object = if child != 0 {
                        e.vcall(child, 0x1c, &args![]).u32()
                    } else {
                        0
                    };
                    if object == 0 {
                        continue;
                    }
                    let cast = e.call(CAST_607310, &args![object]).u32();
                    let head = if cast != 0 {
                        e.vcall(cast, 0x94, &args![]).u32()
                    } else {
                        0
                    };
                    if head == 0 {
                        continue;
                    }
                    if apply {
                        e.call(APPLY_COORDS, &args![head, applied, object, 0u32]);
                    }
                    if retexture {
                        let length = e.call(STRLEN, &args![FACE_GEN_FACE]).u32();
                        let name = e.call(OBJECT_NAME, &args![object]).u32();
                        let hash = e.call(NAME_HASH, &args![name]).u32();
                        if e.call(FIND_BY_NAME, &args![hash, FACE_GEN_FACE, length])
                            .u32()
                            != 0
                        {
                            continue;
                        }
                        let property = property_in_world_range(e, child, 8, 12);
                        if property == 0 {
                            continue;
                        }
                        let texture = frame.addr() + 0x38;
                        e.call(NI_POINTER_CONSTRUCT, &args![texture, 0u32]);
                        if fn_007b2570(e, head) == 0 {
                            e.call(HEAD_REFRESH, &args![head]);
                        }
                        let language = e.call(APPLY_TEXTURING_ARG, &args![]).u32();
                        e.call(APPLY_TEXTURING, &args![head, coords, texture, language]);
                        let image = e.call(STRING_C_STR, &args![texture]).u32();
                        e.vcall(property, 0xfc, &args![1u32, image]);
                        e.call(NI_POINTER_DESTRUCT, &args![texture]);
                    }
                }
                root = e.vcall(player, 0x1b0, &args![0u32]).u32();
            }
            e.call(
                EH_VECTOR_DESTRUCTOR_ITERATOR,
                &args![applied, 0x20u32, 4u32, FACE_COORD_DESTRUCT],
            );
            e.call(
                EH_VECTOR_DESTRUCTOR_ITERATOR,
                &args![coords, 0x20u32, 4u32, FACE_COORD_DESTRUCT],
            );
        }
    });
    fn_007b1ae0(e, this);
}

// Translated from 007b2550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte at `+0xE4` of its object (the scene root in `007b1e70`).
pub fn fn_007b2550(e: &mut Engine, this: u32, value: u8) {
    e.mem.set_u8(this + 0xe4, value);
}

// Translated from 007b2570 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the word at `+0xC` of its object is non-null and the word at
/// `+8` of what it points to is non-null.
pub fn fn_007b2570(e: &mut Engine, this: u32) -> u8 {
    let inner = e.mem.u32(this + 0xc);
    (inner != 0 && e.mem.u32(inner + 8) != 0) as u8
}

// Translated from 007b25a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Rebuilds the player's head after the face data changed: clears the body
/// textures from the palette (`006570e0`), sets `011c5cb4` while it tells
/// the player's saved object slots `+0x58`, `+0x468(3)` and `+0x464(player)`,
/// and, when the player has no 3D, rebuilds it through a `QueuedHead`
/// temporary; then puts the player node's rotation back (the matrix at
/// `011a9448` through `MultipleMatrixByRace`), loads queued IO (`00456520`
/// on `01202d98`), runs `007b1ae0` and calls slots `+0x28` and `+0x24` of
/// the pipboy.
///
/// The compiler's exception-unwinding frame is not translated.
// Translation incomplete: 007b25a0 - the C++ exception-unwinding frame (FS:[0]) that destroys the QueuedHead temporary is not translated
pub fn fn_007b25a0(e: &mut Engine, this: Ptr<RaceSexMenu>) {
    let base = this.addr();
    let player: u32 = e.global(PLAYER);
    let form = e.call(PLAYER_BASE_FORM, &args![player]).u32();
    e.call(CLEAR_BODY_TEXTURES, &args![form]);
    e.mem.set_u8(FACE_REBUILD_FLAG, 1);
    let saved = e.call(SAVED_OBJECT, &args![player]).u32();
    e.vcall(saved, 0x58, &args![]);
    let saved = e.call(SAVED_OBJECT, &args![player]).u32();
    e.vcall(saved, 0x468, &args![3u32]);
    let saved = e.call(SAVED_OBJECT, &args![player]).u32();
    e.vcall(saved, 0x464, &args![player]);
    if e.vcall(player, 0x1b0, &args![0u32]).u32() == 0
        && e.vcall(player, 0x1ac, &args![0u32]).u32() == 0
    {
        // The exe's frame (`SUB ESP,0xa4` under the exception record, then `ESI`, `EDI` and the
        // record link): the queued head at `EBP-0x74` (+0x48).
        e.with_stack(0xbc, |e, frame| {
            let queued = frame.addr() + 0x48;
            let player_base = e.mem.u32(base + 0xd8);
            e.call(QUEUED_HEAD_CONSTRUCT, &args![queued, player_base, 0u32]);
            e.call(QUEUED_HEAD_RUN, &args![queued]);
            e.call(QUEUED_HEAD_FINISH, &args![queued]);
            let biped = e.call(PLAYER_BIPED, &args![player, 0u32]).u32();
            e.call(QUEUED_HEAD_APPLY, &args![queued, player, biped]);
            e.call(QUEUED_HEAD_DESTRUCT, &args![queued]);
        });
    }
    e.mem.set_u8(FACE_REBUILD_FLAG, 0);
    let root = e.call(PLAYER_NODE_GETTER, &args![player, 1u32]).u32();
    let child = e.call(CHILD_AT, &args![root, 0u32]).u32();
    // The same frame: the matrix at `EBP-0x30` (+0x8c), the rotated one at `EBP-0x98` (+0x24).
    e.with_stack(0xbc, |e, frame| {
        let identity = frame.addr() + 0x8c;
        let rotated = frame.addr() + 0x24;
        copy_words(e, ROTATION_MATRIX, identity, 9);
        let matrix = e
            .call(MULTIPLE_MATRIX_BY_RACE, &args![player, rotated, identity])
            .u32();
        e.call(SET_ROTATE, &args![child, matrix]);
    });
    let loader: u32 = e.global(IO_MANAGER);
    e.call(LOAD_QUEUED_PRIORITY, &args![loader]);
    fn_007b1ae0(e, this);
    let pipboy = e.call(GET_PIPBOY, &args![]).u32();
    e.vcall(pipboy, 0x28, &args![]);
    let pipboy = e.call(GET_PIPBOY, &args![]).u32();
    e.vcall(pipboy, 0x24, &args![]);
}
/// `GetProperty(3)` of `object` when its `00441110` value lies between `low`
/// and `high`, else 0 (the repeated block of `007b1e70`, `007b27c0` and
/// `007b2b50`; `441110` is only asked a second time when the first answer
/// is at least `low`).
fn property_in_world_range(e: &mut Engine, object: u32, low: i32, high: i32) -> u32 {
    let property = e.call(GET_PROPERTY, &args![object, 3u32]).u32();
    if property == 0 {
        return 0;
    }
    if e.call(PROPERTY_KIND, &args![property]).i32() < low {
        return 0;
    }
    if e.call(PROPERTY_KIND, &args![property]).i32() > high {
        return 0;
    }
    property
}

/// Sets the 4-float colour at `+0x84` of `property` from the packed colour
/// `color` (red in the low byte, green, blue; each over 255.0, alpha 1.0)
/// through the `NiColorA` constructor `00414430`.
fn set_property_color(e: &mut Engine, scratch: u32, property: u32, color: u32) {
    let divisor: f64 = e.global(COLOR_DIVISOR);
    let channel = |value: u32| (value as f64 / divisor) as f32;
    let red = channel(color & 0xff);
    let green = channel((color & 0xffff) >> 8 & 0xff);
    let blue = channel(color >> 16 & 0xff);
    let made = e
        .call(COLOR_A_CONSTRUCT, &args![scratch, red, green, blue, 1.0f32])
        .u32();
    copy_words(e, made, property + 0x84, 4);
}

// Translated from 007b27c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the eye textures of the player: finds the children named
/// `FaceGenEyeLeft` and `FaceGenEyeRight` of the player's 3D root (slot
/// `+0x1ac`), loads the texture `Textures\<eye texture>` (or
/// `Textures\Characters\Eyes\EyeDefault.dds` when the base form has none,
/// `TES::CreateTextureImage`) and applies it to the shader property of both
/// (kind 8 to 12), then runs `007b1ae0`. Returns false when the player has
/// no 3D root.
///
/// The compiler's exception-unwinding frame is not translated.
// Translation incomplete: 007b27c0 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary pointers and strings is not translated
pub fn fn_007b27c0(e: &mut Engine, this: Ptr<RaceSexMenu>) -> bool {
    let base = this.addr();
    let player: u32 = e.global(PLAYER);
    // The exe's frame (`SUB ESP,0x40` under the exception record, then the record link): the
    // eye pointers at `EBP-0x14` (+0x3c) and `EBP-0x10` (+0x40), the path at `EBP-0x2c`
    // (+0x24), the texture pointer at `EBP-0x24` (+0x2c).
    e.with_stack(0x50, |e, frame| {
        let left = frame.addr() + 0x3c;
        let right = frame.addr() + 0x40;
        e.call(NI_POINTER_CONSTRUCT, &args![left, 0u32]);
        e.call(NI_POINTER_CONSTRUCT, &args![right, 0u32]);
        let result = if e.vcall(player, 0x1ac, &args![0u32]).u32() == 0 {
            false
        } else {
            let player_base = e.mem.u32(base + 0xd8);
            e.vcall(player_base, 0x48, &args![0x800u32]);
            let root = e.vcall(player, 0x1ac, &args![0u32]).u32();
            let count = e.call(CHILD_COUNT, &args![root]).u32();
            for index in 0..count {
                let root = e.vcall(player, 0x1ac, &args![0u32]).u32();
                let child = e.call(CHILD_AT, &args![root, index]).u32();
                for (name, slot) in [(FACE_GEN_EYE_LEFT, left), (FACE_GEN_EYE_RIGHT, right)] {
                    let found = e.call(OBJECT_NAME, &args![child]).u32();
                    let hash = e.call(NAME_HASH, &args![found]).u32();
                    if e.call(NAME_COMPARE, &args![hash, name]).u32() == 0 {
                        let object = e.vcall(child, 0x1c, &args![]).u32();
                        e.call(NI_POINTER_ASSIGN, &args![slot, object]);
                    }
                }
            }
            if e.call(STRING_C_STR, &args![left]).u32() != 0
                && e.call(STRING_C_STR, &args![right]).u32() != 0
            {
                let path = frame.addr() + 0x24;
                e.call(STRING_CONSTRUCT, &args![path]);
                let player_base = e.mem.u32(base + 0xd8);
                let eyes = e.call(BASE_VALUE_602150, &args![player_base]).u32();
                if eyes != 0 {
                    let player_base = e.mem.u32(base + 0xd8);
                    let eyes = e.call(BASE_VALUE_602150, &args![player_base]).u32();
                    let name = e.call(MARKER_NAME, &args![eyes + 0x24]).u32();
                    e.call(STRING_FORMAT, &args![path, TEXTURE_PATH_FORMAT, name]);
                } else {
                    e.call(STRING_FORMAT, &args![path, DEFAULT_EYE_TEXTURE]);
                }
                let texture = frame.addr() + 0x2c;
                e.call(NI_POINTER_CONSTRUCT, &args![texture, 0u32]);
                let text = e.call(STRING_C_STR, &args![path]).u32();
                let renderer: u32 = e.global(TEXTURE_LOADER);
                e.call(
                    CREATE_TEXTURE_IMAGE,
                    &args![renderer, text, texture, 0u32, 0u32],
                );
                if e.call(STRING_C_STR, &args![texture]).u32() != 0 {
                    for slot in [left, right] {
                        let object = e.call(STRING_C_STR, &args![slot]).u32();
                        let property = property_in_world_range(e, object, 8, 12);
                        if property != 0 {
                            let image = e.call(STRING_C_STR, &args![texture]).u32();
                            e.vcall(property, 0xf0, &args![0u32, 0u32, image]);
                        }
                    }
                }
                e.call(NI_POINTER_DESTRUCT, &args![texture]);
                e.call(STRING_DESTROY, &args![path]);
            }
            fn_007b1ae0(e, this);
            true
        };
        e.call(NI_POINTER_DESTRUCT, &args![right]);
        e.call(NI_POINTER_DESTRUCT, &args![left]);
        result
    })
}

/// Finds the child of `node` named by `index` (`00657820` gives the name,
/// made a fixed string by `00438170`; the node's slot `+0x9c` finds it).
fn named_child(e: &mut Engine, scratch: u32, fixed: u32, node: u32, index: u32) -> u32 {
    let text = e.call(FACE_PART_NAME, &args![scratch, index]).u32();
    let name = e.call(FIXED_STRING_FROM, &args![fixed, text]).u32();
    let found = e.vcall(node, 0x9c, &args![name]).u32();
    e.call(FIXED_STRING_DESTRUCT, &args![fixed]);
    found
}

/// Whether the name of `child` is `name`: `004564f0` compares the name
/// object (`00413f40`, `0043b1b0`) with the string and its length
/// (`0044a670`) and answers 0 for a match.
fn child_named(e: &mut Engine, child: u32, name: u32) -> bool {
    let length = e.call(STRING_LENGTH, &args![name]).u32();
    let object = e.call(OBJECT_NAME, &args![child]).u32();
    let hash = e.call(NAME_HASH, &args![object]).u32();
    e.call(FIND_BY_NAME, &args![hash, name, length]).u32() == 0
}
// Translated from 007b2b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Recolours the player's hair (and accessory) after a hair or hair colour
/// change; returns false when the player has no 3D root (slot `+0x1ac`),
/// else true.
///
/// With `rebuild` clear: for every child of the 3D root named
/// `FaceGenHair` or `FaceGenAccessory` (`004564f0` against the names),
/// takes the child's object (slot `+0x1c`), for hair also applies the base
/// form's `00602130` value (`00653470`), and sets the colour of its shader
/// property (kind 8 or 12, `GetProperty(3)`) from the base form's hair
/// colour (`004169d0`).
///
/// With `rebuild` set: gets the race's face data (`TESRace::GetFaceGenData`),
/// detaches the two child objects of the 3D root named by `00657820`,
/// rebuilds the head (`00655320`, `006062e0`), removes the palette entry
/// (`00653270`/`00a6e960`), re-attaches and recolours the objects, and
/// updates the properties (`NiAVObject::UpdateProperties`).
///
/// Ends with the root translation set to (0, 0, 0) and `007b1ae0`. The five
/// temporary strings are constructed and destroyed without being used.
///
/// The compiler's exception-unwinding frame and security cookie are not
/// translated.
// Translation incomplete: 007b2b50 - the C++ exception-unwinding frame (FS:[0]) and the /GS cookie check (00ec408c) are not translated
pub fn fn_007b2b50(e: &mut Engine, this: Ptr<RaceSexMenu>, rebuild: bool) -> bool {
    let base = this.addr();
    let player: u32 = e.global(PLAYER);
    // The exe's frame (`SUB ESP,0x28c` under the exception record, then the record link): the
    // pointers at `EBP-0x50` (+0x24c, unused) and `EBP-0x18` (+0x284), the five unused strings
    // at `EBP-0x4c`, `-0x14`, `-0x20`, `-0x28` and `-0x34`, the face data at `EBP-0x174`
    // (+0x128), the two heads at `EBP-0x7c` (+0x220) and `EBP-0x178` (+0x124), the part name at
    // `EBP-0x78` (+0x224) with its fixed strings at `EBP-0x1c4` and `EBP-0x1c8`, the colours at
    // `EBP-0x1d8`, `-0x1e8`, `-0x1f8` and `-0x208`, the point at `EBP-0x44` (+0x258).
    e.with_stack(0x29c, |e, frame| {
        let local = |offset: u32| frame.addr() + offset;
        let unused_pointer = local(0x24c);
        let held_pointer = local(0x284);
        e.call(NI_POINTER_CONSTRUCT, &args![unused_pointer, 0u32]);
        e.call(NI_POINTER_CONSTRUCT, &args![held_pointer, 0u32]);
        // Constructed in this order and destroyed in reverse; none is used.
        let strings = [0x250, 0x288, 0x27c, 0x274, 0x268].map(local);
        for string in strings {
            e.call(STRING_CONSTRUCT, &args![string]);
        }
        let result = if e.vcall(player, 0x1ac, &args![0u32]).u32() == 0 {
            false
        } else {
            let player_base = e.mem.u32(base + 0xd8);
            e.vcall(player_base, 0x48, &args![0x800u32]);
            if rebuild {
                let face_data = local(0x128);
                e.call(FACE_DATA_CONSTRUCT, &args![face_data]);
                let value = e.call(PLAYER_VALUE_87F4C0, &args![player]).u32();
                let form = e.call(PLAYER_BASE_FORM, &args![player]).u32();
                let race = e.vcall(player, 0x37c, &args![]).u32();
                e.call(
                    GET_FACE_GEN_DATA,
                    &args![race, form, face_data, value, 0u32],
                );
                let (first_slot, second_slot) = (local(0x220), local(0x124));
                let first = e.vcall(player, 0x1ac, &args![0u32]).u32();
                e.mem.set_u32(first_slot, first);
                let second = e.vcall(player, 0x1b0, &args![0u32]).u32();
                e.mem.set_u32(second_slot, second);
                for index in 0..2u32 {
                    let found = named_child(e, local(0x224), local(0xd8), first, index);
                    if found != 0 {
                        e.vcall(first, 0xe8, &args![found]);
                    }
                }
                e.call(FACE_BUILD, &args![first_slot, second_slot, face_data, 0u32]);
                let first = e.mem.u32(first_slot);
                let animation = e.call(PLAYER_ANIMATION, &args![player, 0u32]).u32();
                let held = e.call(ANIMATION_OBJECT, &args![animation]).u32();
                let palette = e.call(PALETTE_OF, &args![held]).u32();
                if palette != 0 {
                    let entry = e.call(PALETTE_FIND, &args![PALETTE_TABLE, palette]).u32();
                    if entry != 0 {
                        e.call(PALETTE_ENTRY_RELEASE, &args![entry]);
                    }
                }
                let form = e.call(PLAYER_BASE_FORM, &args![player]).u32();
                e.call(FORM_UPDATE_6062E0, &args![form, player, 0u32]);
                for index in 0..2u32 {
                    let found = named_child(e, local(0x224), local(0xd4), first, index);
                    if found != 0 {
                        e.call(PREPARE_OBJECT, &args![found, 0u32, 0u32]);
                        let property = e.call(GET_PROPERTY, &args![found, 3u32]).u32();
                        let eight = if property != 0
                            && e.call(PROPERTY_KIND, &args![property]).i32() == 8
                        {
                            property
                        } else {
                            0
                        };
                        let player_base = e.mem.u32(base + 0xd8);
                        let color = e.call(HAIR_COLOR_GETTER, &args![player_base]).u32();
                        if eight != 0 {
                            set_property_color(e, local(0xc4), eight, color);
                        } else {
                            let twelve = if property != 0
                                && e.call(PROPERTY_KIND, &args![property]).i32() == 0xc
                            {
                                property
                            } else {
                                0
                            };
                            if twelve != 0 {
                                set_property_color(e, local(0xb4), twelve, color);
                            }
                        }
                    }
                }
                e.call(UPDATE_PROPERTIES, &args![first]);
                e.call(FACE_DATA_DESTRUCT, &args![face_data]);
            } else {
                let root = e.vcall(player, 0x1ac, &args![0u32]).u32();
                let count = e.call(CHILD_COUNT, &args![root]).u32();
                for index in 0..count {
                    let root = e.vcall(player, 0x1ac, &args![0u32]).u32();
                    let child = e.call(CHILD_AT, &args![root, index]).u32();
                    let hair = child != 0 && child_named(e, child, FACE_GEN_HAIR);
                    let accessory =
                        !hair && child != 0 && child_named(e, child, FACE_GEN_ACCESSORY);
                    if !(hair || accessory) {
                        continue;
                    }
                    let object = e.vcall(child, 0x1c, &args![]).u32();
                    e.call(NI_POINTER_ASSIGN, &args![held_pointer, object]);
                    if e.call(STRING_C_STR, &args![held_pointer]).u32() == 0 {
                        continue;
                    }
                    if hair {
                        let player_base = e.mem.u32(base + 0xd8);
                        let value = e.call(HAIR_VALUE_602130, &args![player_base]).f32();
                        let held = e.call(STRING_C_STR, &args![held_pointer]).u32();
                        e.call(APPLY_HAIR_VALUE, &args![held, value]);
                    }
                    let held = e.call(STRING_C_STR, &args![held_pointer]).u32();
                    let property = e.call(GET_PROPERTY, &args![held, 3u32]).u32();
                    let eight =
                        if property != 0 && e.call(PROPERTY_KIND, &args![property]).i32() == 8 {
                            property
                        } else {
                            0
                        };
                    if eight != 0 {
                        let player_base = e.mem.u32(base + 0xd8);
                        let color = e.call(HAIR_COLOR_GETTER, &args![player_base]).u32();
                        set_property_color(e, local(0xa4), eight, color);
                    } else {
                        let twelve = if property != 0
                            && e.call(PROPERTY_KIND, &args![property]).i32() == 0xc
                        {
                            property
                        } else {
                            0
                        };
                        if twelve != 0 {
                            let player_base = e.mem.u32(base + 0xd8);
                            let color = e.call(HAIR_COLOR_GETTER, &args![player_base]).u32();
                            set_property_color(e, local(0x94), twelve, color);
                        }
                    }
                }
            }
            let point = local(0x258);
            e.call(POINT_CONSTRUCT, &args![point, 0.0f32, 0u32, 0u32]);
            let root = e.vcall(player, 0x1ac, &args![0u32]).u32();
            e.call(SET_TRANSLATE_POINT, &args![root, point]);
            fn_007b1ae0(e, this);
            true
        };
        for string in strings.into_iter().rev() {
            e.call(STRING_DESTROY, &args![string]);
        }
        // 007b3627: the held pointer (`EBP-0x18`) is destroyed before the unused one.
        e.call(NI_POINTER_DESTRUCT, &args![held_pointer]);
        e.call(NI_POINTER_DESTRUCT, &args![unused_pointer]);
        result
    })
}
// Translated from 007b3660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the values of the options with id `0x1e`, `0x1f` and `0x20` of the
/// list at `+0x40` (a missing one counts as `0xffffffff`), packs the low
/// bytes as red, green and blue, stores the colour in the player base
/// form's hair colour word (`007b3720`) and runs `007b2b50(0)`.
pub fn fn_007b3660(e: &mut Engine, this: Ptr<RaceSexMenu>) {
    let base = this.addr();
    let mut red = u32::MAX;
    let mut green = u32::MAX;
    let mut blue = u32::MAX;
    let list = e.mem.u32(base + 0x40);
    let head = e.call(STRING_C_STR, &args![list]).u32();
    e.with_stack(4, |e, cursor| {
        e.mem.set_u32(cursor.addr(), head);
        while e.mem.u32(cursor.addr()) != 0 {
            let slot = e.call(LIST_ADVANCE, &args![list, cursor]).u32();
            let option = e.mem.u32(slot);
            match word_at_four(e, option) {
                0x1e => red = menu_option_get_value(e, Ptr::new(option)) as u32,
                0x1f => green = menu_option_get_value(e, Ptr::new(option)) as u32,
                0x20 => blue = menu_option_get_value(e, Ptr::new(option)) as u32,
                _ => {}
            }
        }
    });
    let color = (red & 0xff) | (green & 0xff) << 8 | (blue & 0xff) << 16;
    let player_base = e.mem.u32(base + 0xd8);
    fn_007b3720(e, player_base, color);
    fn_007b2b50(e, this, false);
}

// Translated from 007b3720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a word at `+0x1D8` of its object (the player base form's hair
/// colour, which `004169d0` reads).
pub fn fn_007b3720(e: &mut Engine, this: u32, value: u32) {
    e.mem.set_u32(this + 0x1d8, value);
}

// Translated from 007b3740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::MenuOption::GetValue` (Xbox PDB): the option tile's float
/// trait `0x1005`, truncated to an integer.
pub fn menu_option_get_value(e: &mut Engine, this: Ptr<MenuOption>) -> i32 {
    let tile = e.mem.u32(this.addr());
    let value = e.call(GET_FLOAT_TRAIT, &args![tile, 0x1005u32]).f32();
    e.call(FTOL, &args![value as f64]).i32()
}

// Translated from 007b3760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::MenuOption::SetValue` (Xbox PDB): with a tile, limits
/// `value` to the option's range (traits `0x1006` and `0x1007` through
/// `007b3840`) and, when it differs from the current value, stores it in
/// trait `0x1005`, sets the float of the tile's `0x1009` value to it
/// unless `0084e3a0` of that value answers non-zero, and returns true.
pub fn menu_option_set_value(e: &mut Engine, this: Ptr<MenuOption>, value: i32) -> bool {
    let tile = e.mem.u32(this.addr());
    if tile == 0 {
        return false;
    }
    let current = menu_option_get_value(e, this);
    let (min, max) = e.with_stack(8, |e, range| {
        fn_007b3840(e, this, range.addr(), range.addr() + 4);
        (e.mem.i32(range.addr()), e.mem.i32(range.addr() + 4))
    });
    let mut value = value;
    if value < min {
        value = min;
    }
    if value > max {
        value = max;
    }
    if current == value {
        return false;
    }
    let tile = e.mem.u32(this.addr());
    e.call(SET_TRAIT_FROM_INT, &args![tile, 0x1005u32, value]);
    let tile = e.mem.u32(this.addr());
    let tile_value = e.call(TILE_GET_VALUE, &args![tile, 0x1009u32]).u32();
    if e.call(LIST_BACK_TARGET, &args![tile_value]).u32() == 0 {
        e.call(TILE_VALUE_SET_FLOAT, &args![tile_value, value as f32, 1u32]);
    }
    true
}

// Translated from 007b3810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the string trait `0x1009` of the option's tile to `text` when both
/// exist.
pub fn fn_007b3810(e: &mut Engine, this: Ptr<MenuOption>, text: u32) {
    let tile = e.mem.u32(this.addr());
    if tile != 0 && text != 0 {
        e.call(SET_STRING_TRAIT, &args![tile, 0x1009u32, text, 1u32]);
    }
}

// Translated from 007b3840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the option's range (float traits `0x1006` and `0x1007`, truncated)
/// to the two output words; does nothing without a tile.
pub fn fn_007b3840(e: &mut Engine, this: Ptr<MenuOption>, min_out: u32, max_out: u32) {
    let tile = e.mem.u32(this.addr());
    if tile == 0 {
        return;
    }
    let min = e.call(GET_FLOAT_TRAIT, &args![tile, 0x1006u32]).f32();
    let min = e.call(FTOL, &args![min as f64]).i32();
    e.mem.set_i32(min_out, min);
    let tile = e.mem.u32(this.addr());
    let max = e.call(GET_FLOAT_TRAIT, &args![tile, 0x1007u32]).f32();
    let max = e.call(FTOL, &args![max as f64]).i32();
    e.mem.set_i32(max_out, max);
}

// Translated from 007b3890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::MenuOption::SetRange` (Xbox PDB): with a tile, stores the
/// range in the traits `0x1006` and `0x1007` and moves the value into it.
pub fn menu_option_set_range(e: &mut Engine, this: Ptr<MenuOption>, min: i32, max: i32) {
    let tile = e.mem.u32(this.addr());
    if tile == 0 {
        return;
    }
    e.call(SET_TRAIT_FROM_INT, &args![tile, 0x1006u32, min]);
    let tile = e.mem.u32(this.addr());
    e.call(SET_TRAIT_FROM_INT, &args![tile, 0x1007u32, max]);
    if menu_option_get_value(e, this) < min {
        menu_option_set_value(e, this, min);
    }
    if menu_option_get_value(e, this) > max {
        menu_option_set_value(e, this, max);
    }
}

// Translated from 007b3900 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a tile: its float trait `0x100a`, as `1` when it is not above the
/// constant at `01012070` and otherwise truncated (`FISTP`, round toward
/// zero, low 32 bits; an overflow or NaN gives 0). Without a tile: 0.
pub fn fn_007b3900(e: &mut Engine, this: Ptr<MenuOption>) -> i32 {
    let tile = e.mem.u32(this.addr());
    if tile == 0 {
        return 0;
    }
    let value = e.call(GET_FLOAT_TRAIT, &args![tile, 0x100au32]).f32();
    let limit: f64 = e.global(ONE_DOUBLE);
    if (value as f64) <= limit {
        1
    } else {
        fistp_truncate(value as f64)
    }
}

// Translated from 007b3970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::MenuOption::SetEnabled` (Xbox PDB): with a tile, sets its
/// user trait `_enabled` to the flag.
pub fn menu_option_set_enabled(e: &mut Engine, this: Ptr<MenuOption>, enabled: bool) {
    let tile = e.mem.u32(this.addr());
    if tile == 0 {
        return;
    }
    let trait_id = e
        .call(ADD_USER_TRAIT, &args![ENABLED_TRAIT_NAME, u32::MAX])
        .u32();
    let tile = e.mem.u32(this.addr());
    e.call(SET_TRAIT_FROM_INT, &args![tile, trait_id, enabled as u32]);
}

// Translated from 007b39b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The option tile's user trait `_position`, truncated to an integer.
pub fn fn_007b39b0(e: &mut Engine, this: Ptr<MenuOption>) -> i32 {
    let trait_id = e
        .call(ADD_USER_TRAIT, &args![POSITION_TRAIT_NAME, u32::MAX])
        .u32();
    let tile = e.mem.u32(this.addr());
    let value = e.call(GET_FLOAT_TRAIT, &args![tile, trait_id]).f32();
    e.call(FTOL, &args![value as f64]).i32()
}

// Translated from 007b39e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::MenuOptionList::MenuOptionList` (Xbox PDB): constructs the
/// list in place (base constructor `0048f200`), sets `+0x10` and `+0xC` to
/// 0x14, `+0x18` to 0.0 and `+0x1C` to 0, renders the template
/// `RSM_list_item_template` from the menu's first tile into `+0x14`, gives
/// that tile the number (trait `0x1004`) and the caption (string trait
/// `0xfc4`), clears trait `0xfaf`, sets its user trait `_is_header` to 1,
/// updates it, and clears `+0x20`. Returns the list.
///
/// The compiler's exception-unwinding frame is not translated.
// Translation incomplete: 007b39e0 - the C++ exception-unwinding frame (FS:[0]) of the base constructor is not translated
pub fn menu_option_list_menu_option_list(
    e: &mut Engine,
    this: Ptr<MenuOptionList>,
    number: u32,
    caption: u32,
) -> Ptr<MenuOptionList> {
    let base = this.addr();
    e.call(OPTION_LIST_BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(base + 0x10, 0x14);
    e.mem.set_u32(base + 0xc, 0x14);
    e.mem.set_f32(base + 0x18, 0.0);
    e.mem.set_u32(base + 0x1c, 0);
    let me: u32 = e.global(RACE_SEX_MENU_ME);
    let first_tile = e.mem.u32(me + 0x78);
    let me: u32 = e.global(RACE_SEX_MENU_ME);
    let rendered = e
        .call(
            RENDER_TEMPLATE,
            &args![me, first_tile, LIST_ITEM_TEMPLATE, 0u32],
        )
        .u32();
    e.mem.set_u32(base + 0x14, rendered);
    let tile = e.mem.u32(base + 0x14);
    e.call(SET_TRAIT_FROM_INT, &args![tile, 0x1004u32, number]);
    let tile = e.mem.u32(base + 0x14);
    e.call(SET_STRING_TRAIT, &args![tile, TRAIT_STRING, caption, 1u32]);
    let tile = e.mem.u32(base + 0x14);
    e.call(SET_TRAIT_FROM_INT, &args![tile, 0xfafu32, 0u32]);
    let trait_id = e
        .call(ADD_USER_TRAIT, &args![HEADER_TRAIT_NAME, u32::MAX])
        .u32();
    let tile = e.mem.u32(base + 0x14);
    e.call(SET_TRAIT_FROM_INT, &args![tile, trait_id, 1u32]);
    let tile = e.mem.u32(base + 0x14);
    e.call(UPDATE_TILE, &args![tile, 0u32]);
    e.mem.set_u32(base + 0x20, 0);
    this
}

// Translated from 007b3af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::MenuOptionList::~MenuOptionList` (Xbox PDB, no name in the
/// map): deletes the rendered template tile (`+0x14`, virtual slot 0 with
/// flag 1), clears it, empties the list (`007b45b0`) and runs the base
/// destructor (`004a1a30`).
///
/// The compiler's exception-unwinding frame is not translated.
// Translation incomplete: 007b3af0 - the C++ exception-unwinding frame (FS:[0]) is not translated
pub fn fn_007b3af0(e: &mut Engine, this: Ptr<MenuOptionList>) {
    let base = this.addr();
    let tile = e.mem.u32(base + 0x14);
    if tile != 0 {
        e.vcall(tile, 0, &args![1u32]);
    }
    e.mem.set_u32(base + 0x14, 0);
    e.call(OPTION_LIST_CLEAR, &args![this]);
    e.call(OPTION_LIST_BASE_DESTRUCT, &args![this]);
}

/// The loop `AddMenuOption` and `RemoveMenuOption` end with: from the node
/// in `cursor` (a word on the game's stack) on, places each option's tile
/// (float trait `0xfa2`) below the option before it (that option's `0xfa2`
/// plus its height `0xfb0`) or, for the first option, at the parent tile's
/// user trait `_top_bound`, and updates the tile.
fn place_options_from(e: &mut Engine, list: u32, cursor: u32) {
    while e.mem.u32(cursor) != 0 {
        let node = e.mem.u32(cursor);
        let previous_node = e.call(LIST_NODE_PREVIOUS, &args![list, node]).u32();
        let previous = if previous_node == 0 {
            0
        } else {
            let slot = e.call(LIST_NODE_DATA, &args![list, previous_node]).u32();
            e.mem.u32(slot)
        };
        let slot = e.call(LIST_ADVANCE, &args![list, cursor]).u32();
        let option = e.mem.u32(slot);
        // Kept as a double, then stored as a float.
        let top = if previous != 0 {
            let tile = e.mem.u32(previous);
            let y = e.call(GET_FLOAT_TRAIT, &args![tile, 0xfa2u32]).f32();
            let tile = e.mem.u32(previous);
            let height = e.call(GET_FLOAT_TRAIT, &args![tile, 0xfb0u32]).f32();
            height as f64 + y as f64
        } else {
            let trait_id = e
                .call(ADD_USER_TRAIT, &args![TOP_BOUND_TRAIT_NAME, u32::MAX])
                .u32();
            let tile = e.mem.u32(option);
            let parent = e.call(WORD_AT_0X28, &args![tile]).u32();
            e.call(GET_FLOAT_TRAIT, &args![parent, trait_id]).f32() as f64
        };
        let tile = e.mem.u32(option);
        e.call(SET_FLOAT_TRAIT, &args![tile, 0xfa2u32, top as f32, 1u32]);
        let tile = e.mem.u32(option);
        e.call(UPDATE_TILE, &args![tile, 0u32]);
    }
}

// Translated from 007b3b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds an option with the template `RSM_list_item_template`, the caption
/// and the id at the end of the list (`AddMenuOption` with no node to
/// insert before) and returns it.
pub fn fn_007b3b80(
    e: &mut Engine,
    this: Ptr<MenuOptionList>,
    caption: u32,
    id: u32,
) -> Ptr<MenuOption> {
    menu_option_list_add_menu_option(e, this, caption, LIST_ITEM_TEMPLATE, id, 0)
}

// Translated from 007b3bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds an option that opens `sub_list`: sets the sub list's back target
/// (`+0xC`) to this list's number (its template tile's trait `0x1004`,
/// truncated) and adds an `RSM_list_item_template` option captioned
/// `"<caption> >"` (`%s %c`) whose id is the sub list's number. Returns
/// the option.
///
/// The compiler's exception-unwinding frame is not translated.
// Translation incomplete: 007b3bb0 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary string is not translated
pub fn fn_007b3bb0(
    e: &mut Engine,
    this: Ptr<MenuOptionList>,
    caption: u32,
    sub_list: Ptr<MenuOptionList>,
) -> Ptr<MenuOption> {
    e.with_stack(8, |e, text| {
        let text = text.addr();
        e.call(STRING_CONSTRUCT, &args![text]);
        e.call(
            STRING_FORMAT,
            &args![text, CAPTION_FORMAT, caption, 0x3eu32],
        );
        let tile = e.get(this, MenuOptionList::pTemplateTile);
        let number = e.call(GET_FLOAT_TRAIT, &args![tile, 0x1004u32]).f32();
        e.set(
            sub_list,
            MenuOptionList::iBackTarget,
            fistp_truncate(number as f64) as u32,
        );
        let tile = e.get(sub_list, MenuOptionList::pTemplateTile);
        let number = e.call(GET_FLOAT_TRAIT, &args![tile, 0x1004u32]).f32();
        let id = fistp_truncate(number as f64) as u32;
        let name = e.call(STRING_C_STR, &args![text]).u32();
        let option = menu_option_list_add_menu_option(e, this, name, LIST_ITEM_TEMPLATE, id, 0);
        e.call(STRING_DESTROY, &args![text]);
        option
    })
}

// Translated from 007b3ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::MenuOptionList::AddSliderOption` (Xbox PDB): limits
/// `fraction` to 0..1 (below the double 0.0 becomes 0, above the double
/// 1.0 becomes 1; NaN stays), adds an `RSM_slider_option_template` option
/// at the end, sets its range and value, and sets its tile's float trait
/// `0x1008` to `(max - min + 1) * fraction` rounded to a step of 1
/// (`004bd510`). Returns the option.
#[allow(clippy::too_many_arguments)] // The exe ABI has six stack arguments.
pub fn menu_option_list_add_slider_option(
    e: &mut Engine,
    this: Ptr<MenuOptionList>,
    caption: u32,
    id: u32,
    min: i32,
    max: i32,
    value: i32,
    fraction: f32,
) -> Ptr<MenuOption> {
    let mut fraction = fraction;
    let zero: f64 = e.global(ZERO_DOUBLE);
    if (fraction as f64) < zero {
        fraction = 0.0;
    }
    let one: f64 = e.global(ONE_DOUBLE);
    if (fraction as f64) > one {
        fraction = 1.0;
    }
    let option = menu_option_list_add_menu_option(e, this, caption, SLIDER_TEMPLATE, id, 0);
    menu_option_set_range(e, option, min, max);
    menu_option_set_value(e, option, value);
    let count = max.wrapping_sub(min).wrapping_add(1);
    let scaled = (count as f64 * fraction as f64) as f32;
    let rounded = e.call(ROUND_TO_STEP, &args![scaled, 1.0f32]).f32();
    let tile = e.mem.u32(option.addr());
    e.call(SET_FLOAT_TRAIT, &args![tile, 0x1008u32, rounded, 1u32]);
    option
}

// Translated from 007b3d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::MenuOptionList::InsertMenuOption` (Xbox PDB): with a
/// caption, an open menu with a first tile and a template tile, adds the
/// option before the node `index` steps after the head (at the end when
/// the list is shorter); otherwise answers null.
pub fn menu_option_list_insert_menu_option(
    e: &mut Engine,
    this: Ptr<MenuOptionList>,
    index: i32,
    caption: u32,
    template: u32,
    id: u32,
) -> Ptr<MenuOption> {
    let me: u32 = e.global(RACE_SEX_MENU_ME);
    if caption == 0
        || me == 0
        || e.mem.u32(me + 0x78) == 0
        || e.get(this, MenuOptionList::pTemplateTile) == 0
    {
        return Ptr::new(0);
    }
    let mut node = e.call(STRING_C_STR, &args![this]).u32();
    let mut index = index;
    while node != 0 && index != 0 {
        node = e.call(LIST_NODE_NEXT, &args![this, node]).u32();
        index = index.wrapping_sub(1);
    }
    menu_option_list_add_menu_option(e, this, caption, template, id, node)
}

// Translated from 007b3dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::MenuOptionList::RemoveMenuOption` (Xbox PDB): finds the
/// first option with the id; if it is the highlighted one, highlights the
/// option after it (or the last one) and focuses its tile; deletes the
/// option's tile (virtual slot 0 with flag 1) and the option (`007b3fa0`)
/// and removes it from the list. Then places the options after it again
/// (`place_options_from`); nothing moves when no option has the id.
pub fn menu_option_list_remove_menu_option(e: &mut Engine, this: Ptr<MenuOptionList>, id: u32) {
    e.with_stack(8, |e, locals| {
        let cursor = locals.addr();
        let removed = cursor + 4;
        let head = e.call(STRING_C_STR, &args![this]).u32();
        e.mem.set_u32(cursor, head);
        let mut found = false;
        while e.mem.u32(cursor) != 0 {
            let slot = e.call(LIST_ADVANCE, &args![this, cursor]).u32();
            let option = e.mem.u32(slot);
            e.mem.set_u32(removed, option);
            if word_at_four(e, option) == id {
                found = true;
                break;
            }
        }
        if found {
            let option = e.mem.u32(removed);
            if e.get(this, MenuOptionList::pHighlighted) == option {
                let next = e.mem.u32(cursor);
                let slot = if next != 0 {
                    e.call(LIST_NODE_DATA, &args![this, next]).u32()
                } else {
                    e.call(LIST_TAIL_DATA, &args![this]).u32()
                };
                let replacement = e.mem.u32(slot);
                e.set(this, MenuOptionList::pHighlighted, replacement);
                let highlighted = e.get(this, MenuOptionList::pHighlighted);
                let tile = e.mem.u32(highlighted);
                e.call(SET_FOCUS_TARGET_CDECL, &args![tile]);
            }
            let tile = e.mem.u32(option);
            if tile != 0 {
                e.vcall(tile, 0, &args![1u32]);
            }
            if option != 0 {
                fn_007b3fa0(e, option, 1);
            }
            e.call(LIST_REMOVE_ITEM, &args![this, removed]);
        }
        place_options_from(e, this.addr(), cursor);
    });
}

// Translated from 007b3fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar deleting destructor whose destructor is the empty member
/// `00483710` (identical-code folding: many classes call this body; here
/// it deletes a `MenuOption`): frees the object when bit 0 of `flags` is
/// set and returns it.
pub fn fn_007b3fa0(e: &mut Engine, this: u32, flags: u32) -> u32 {
    e.call(EMPTY_MEMBER, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 007b3fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::MenuOptionList::Focus` (Xbox PDB): with `focus`, gives the
/// interface focus (`00715860` with `0xfc3`, 1) to the highlighted option's
/// tile when there is one and answers whether it did; without, takes the
/// focus away (tile 0), clears the highlighted option and answers 1.
pub fn menu_option_list_focus(e: &mut Engine, this: Ptr<MenuOptionList>, focus: bool) -> u8 {
    if focus {
        let highlighted = e.get(this, MenuOptionList::pHighlighted);
        if highlighted == 0 || e.mem.u32(highlighted) == 0 {
            return 0;
        }
        let tile = e.mem.u32(highlighted);
        let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
        e.call(
            SET_CURRENT_FOCUS_TARGET,
            &args![interface, tile, 0xfc3u32, 1u32],
        );
    } else {
        let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
        e.call(
            SET_CURRENT_FOCUS_TARGET,
            &args![interface, 0u32, 0xfc3u32, 1u32],
        );
        e.set(this, MenuOptionList::pHighlighted, 0);
    }
    1
}

// Translated from 007b4050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Links two lists: this list's forward target (`+0x10`) becomes `next`'s
/// number and `next`'s back target (`+0xC`) this list's number (the
/// template tiles' trait `0x1004`, truncated).
pub fn fn_007b4050(e: &mut Engine, this: Ptr<MenuOptionList>, next: Ptr<MenuOptionList>) {
    let tile = e.get(next, MenuOptionList::pTemplateTile);
    let number = e.call(GET_FLOAT_TRAIT, &args![tile, 0x1004u32]).f32();
    e.set(
        this,
        MenuOptionList::iParentTarget,
        fistp_truncate(number as f64) as u32,
    );
    let tile = e.get(this, MenuOptionList::pTemplateTile);
    let number = e.call(GET_FLOAT_TRAIT, &args![tile, 0x1004u32]).f32();
    e.set(
        next,
        MenuOptionList::iBackTarget,
        fistp_truncate(number as f64) as u32,
    );
}

// Translated from 007b40d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::MenuOptionList::UpdateCurrentSelection` (Xbox PDB): with a
/// selection test (`+0x1C`, cdecl, one option), marks every option
/// selected (`007b4140`) or not (`007b4180`) by its answer.
pub fn menu_option_list_update_current_selection(e: &mut Engine, this: Ptr<MenuOptionList>) {
    if e.get(this, MenuOptionList::pCallback) == 0 {
        return;
    }
    e.with_stack(4, |e, cursor| {
        let head = e.call(STRING_C_STR, &args![this]).u32();
        e.mem.set_u32(cursor.addr(), head);
        while e.mem.u32(cursor.addr()) != 0 {
            let slot = e.call(LIST_ADVANCE, &args![this, cursor]).u32();
            let option = e.mem.u32(slot);
            let test = e.get(this, MenuOptionList::pCallback);
            if e.call(test, &args![option]).u8() != 0 {
                fn_007b4140(e, Ptr::new(option));
            } else {
                fn_007b4180(e, Ptr::new(option));
            }
        }
    });
}

/// Sets the option tile's user trait `_selected` to `value` (with the
/// flag 1).
fn set_selected_trait(e: &mut Engine, this: Ptr<MenuOption>, value: f32) {
    let trait_id = e
        .call(ADD_USER_TRAIT, &args![SELECTED_TRAIT_NAME, u32::MAX])
        .u32();
    let tile = e.mem.u32(this.addr());
    e.call(SET_FLOAT_TRAIT, &args![tile, trait_id, value, 1u32]);
}

// Translated from 007b4140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks the option selected: with a tile, its user trait `_selected`
/// becomes the float at `010162c0` (copied through the x87 stack).
pub fn fn_007b4140(e: &mut Engine, this: Ptr<MenuOption>) {
    if e.mem.u32(this.addr()) == 0 {
        return;
    }
    let value = x87_copy_f32(e, HIGHLIGHT_VALUE);
    set_selected_trait(e, this, value);
}

// Translated from 007b4180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks the option not selected: with a tile, its user trait `_selected`
/// becomes 1.0.
pub fn fn_007b4180(e: &mut Engine, this: Ptr<MenuOption>) {
    if e.mem.u32(this.addr()) == 0 {
        return;
    }
    set_selected_trait(e, this, 1.0);
}

// Translated from 007b41c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first option of the list whose tile is `tile`, or 0.
pub fn fn_007b41c0(e: &mut Engine, this: Ptr<MenuOptionList>, tile: u32) -> u32 {
    e.with_stack(4, |e, cursor| {
        let head = e.call(STRING_C_STR, &args![this]).u32();
        e.mem.set_u32(cursor.addr(), head);
        while e.mem.u32(cursor.addr()) != 0 {
            let slot = e.call(LIST_ADVANCE, &args![this, cursor]).u32();
            let option = e.mem.u32(slot);
            if e.mem.u32(option) == tile {
                return option;
            }
        }
        0
    })
}

// Translated from 007b4210 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first option of the list whose id (`+4`) is `id`, or 0.
pub fn fn_007b4210(e: &mut Engine, this: Ptr<MenuOptionList>, id: u32) -> u32 {
    e.with_stack(4, |e, cursor| {
        let head = e.call(STRING_C_STR, &args![this]).u32();
        e.mem.set_u32(cursor.addr(), head);
        while e.mem.u32(cursor.addr()) != 0 {
            let slot = e.call(LIST_ADVANCE, &args![this, cursor]).u32();
            let option = e.mem.u32(slot);
            if e.mem.u32(option + 4) == id {
                return option;
            }
        }
        0
    })
}

// Translated from 007b4260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::MenuOptionList::DoGamepad` (Xbox PDB): button 1 (up) and
/// 2 (down) highlight the option before / after the highlighted one (the
/// last / first one when none is highlighted; answers false at an end),
/// 0xF the first and 0x10 the last option; other buttons answer false.
/// Then, while the highlighted tile is not visible (at most 100 times),
/// asks the open menu to scroll (virtual slot `0xC` with 2 for buttons 1
/// and 0xF, otherwise 3, and 0), and focuses the tile (`00706ff0`).
pub fn menu_option_list_do_gamepad(e: &mut Engine, this: Ptr<MenuOptionList>, button: i32) -> bool {
    let up = button == 1 || button == 0xf;
    let mut handled = true;
    match button {
        1 | 2 => {
            let highlighted = this.addr() + 0x20;
            let node = e
                .call(LIST_FIND_NODE, &args![this, highlighted, 0u32])
                .u32();
            let target = if node != 0 {
                if up {
                    e.call(LIST_NODE_PREVIOUS, &args![this, node]).u32()
                } else {
                    e.call(LIST_NODE_NEXT, &args![this, node]).u32()
                }
            } else if up {
                word_at_four(e, this.addr())
            } else {
                e.call(STRING_C_STR, &args![this]).u32()
            };
            if target != 0 {
                let slot = e.call(LIST_NODE_DATA, &args![this, target]).u32();
                let option = e.mem.u32(slot);
                e.set(this, MenuOptionList::pHighlighted, option);
            } else {
                handled = false;
            }
        }
        0xf => {
            let slot = e.call(LIST_HEAD_DATA, &args![this]).u32();
            let option = e.mem.u32(slot);
            e.set(this, MenuOptionList::pHighlighted, option);
        }
        0x10 => {
            let slot = e.call(LIST_TAIL_DATA, &args![this]).u32();
            let option = e.mem.u32(slot);
            e.set(this, MenuOptionList::pHighlighted, option);
        }
        _ => return false,
    }
    let highlighted = e.get(this, MenuOptionList::pHighlighted);
    if highlighted != 0 && e.mem.u32(highlighted) != 0 {
        let mut tries = 100u32;
        loop {
            let left = tries;
            tries = tries.wrapping_sub(1);
            if left == 0 {
                break;
            }
            let highlighted = e.get(this, MenuOptionList::pHighlighted);
            let tile = e.mem.u32(highlighted);
            if e.call(TILE_IS_VISIBLE, &args![tile]).u8() != 0 {
                break;
            }
            let me: u32 = e.global(RACE_SEX_MENU_ME);
            let direction = if up { 2u32 } else { 3u32 };
            e.vcall(me, 0xc, &args![direction, 0u32]);
        }
        let highlighted = e.get(this, MenuOptionList::pHighlighted);
        let tile = e.mem.u32(highlighted);
        e.call(SET_FOCUS_TARGET_CDECL, &args![tile]);
    }
    handled
}

// Translated from 007b4410 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a gamepad in use (`004b71d0`): highlights the first option when
/// none is and the list is not empty (`0076b610`); then, with a highlighted
/// option, sets its tile's trait `0xfe8` to -1, focuses the tile
/// (`00706ff0`), puts the trait's previous string back, and scrolls the
/// list by minus the option's `_position` (`007b39b0`) until that is 0.
///
/// The compiler's exception-unwinding frame is not translated.
// Translation incomplete: 007b4410 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary string is not translated
pub fn fn_007b4410(e: &mut Engine, this: Ptr<MenuOptionList>) {
    if e.call(GAMEPAD_IN_USE, &args![]).u8() == 0 {
        return;
    }
    if e.get(this, MenuOptionList::pHighlighted) == 0
        && e.call(SIMPLE_ARRAY_IS_EMPTY, &args![this]).u8() == 0
    {
        let slot = e.call(LIST_HEAD_DATA, &args![this]).u32();
        let option = e.mem.u32(slot);
        e.set(this, MenuOptionList::pHighlighted, option);
    }
    if e.get(this, MenuOptionList::pHighlighted) == 0 {
        return;
    }
    e.with_stack(8, |e, text| {
        let text = text.addr();
        let highlighted = e.get(this, MenuOptionList::pHighlighted);
        let tile = e.mem.u32(highlighted);
        let previous = e.call(TILE_GET_STRING, &args![tile, 0xfe8u32]).u32();
        e.call(STRING_FROM_TEXT, &args![text, previous]);
        let highlighted = e.get(this, MenuOptionList::pHighlighted);
        let tile = e.mem.u32(highlighted);
        e.call(SET_TRAIT_FROM_INT, &args![tile, 0xfe8u32, u32::MAX]);
        let highlighted = e.get(this, MenuOptionList::pHighlighted);
        let tile = e.mem.u32(highlighted);
        e.call(SET_FOCUS_TARGET_CDECL, &args![tile]);
        let name = e.call(STRING_C_STR, &args![text]).u32();
        let highlighted = e.get(this, MenuOptionList::pHighlighted);
        let tile = e.mem.u32(highlighted);
        e.call(SET_STRING_TRAIT, &args![tile, 0xfe8u32, name, 1u32]);
        loop {
            let highlighted = e.get(this, MenuOptionList::pHighlighted);
            let position = fn_007b39b0(e, Ptr::new(highlighted));
            if position == 0 {
                break;
            }
            menu_option_list_scroll_by(e, this, position.wrapping_neg());
        }
        e.call(STRING_DESTROY, &args![text]);
    });
}

// Translated from 007b4530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::MenuOptionList::ScrollBy` (Xbox PDB): when the scroll step
/// (`+0x18`) is above the double 0.0 (or NaN), moves every option's tile
/// (float trait `0xfa2`) by `delta` steps.
pub fn menu_option_list_scroll_by(e: &mut Engine, this: Ptr<MenuOptionList>, delta: i32) {
    let step = e.get(this, MenuOptionList::fScrollDelta) as f64;
    let zero: f64 = e.global(ZERO_DOUBLE);
    if step <= zero {
        return;
    }
    e.with_stack(4, |e, cursor| {
        let head = e.call(STRING_C_STR, &args![this]).u32();
        e.mem.set_u32(cursor.addr(), head);
        while e.mem.u32(cursor.addr()) != 0 {
            let slot = e.call(LIST_ADVANCE, &args![this, cursor]).u32();
            let option = e.mem.u32(slot);
            let tile = e.mem.u32(option);
            let y = e.call(GET_FLOAT_TRAIT, &args![tile, 0xfa2u32]).f32();
            let step = e.get(this, MenuOptionList::fScrollDelta);
            let moved = (delta as f64 * step as f64 + y as f64) as f32;
            e.call(SET_FLOAT_TRAIT, &args![tile, 0xfa2u32, moved, 1u32]);
        }
    });
}

// Translated from 007b45b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the list: deletes every option's tile (virtual slot 0 with flag
/// 1) and the option (`007b3fa0`), removes all nodes (`004ed900`) and
/// clears the highlighted option.
pub fn fn_007b45b0(e: &mut Engine, this: Ptr<MenuOptionList>) {
    e.with_stack(4, |e, cursor| {
        let head = e.call(STRING_C_STR, &args![this]).u32();
        e.mem.set_u32(cursor.addr(), head);
        while e.mem.u32(cursor.addr()) != 0 {
            let slot = e.call(LIST_ADVANCE, &args![this, cursor]).u32();
            let option = e.mem.u32(slot);
            let tile = e.mem.u32(option);
            if tile != 0 {
                e.vcall(tile, 0, &args![1u32]);
            }
            if option != 0 {
                fn_007b3fa0(e, option, 1);
            }
        }
    });
    e.call(LIST_REMOVE_ALL, &args![this]);
    e.set(this, MenuOptionList::pHighlighted, 0);
}

// Translated from 007b4650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::MenuOptionList::AddMenuOption` (Xbox PDB): with a caption,
/// an open menu with a first tile and a template tile: makes an option
/// (`007b4900`) with the id, renders `template` from the menu's first tile
/// as its tile, gives the tile the caption (string trait `0xfc4`), the
/// list's number (trait `0x1004`) and trait `0xfaf` = 1, inserts it before
/// the node `before` (or through `004ed8c0`), places it and the options
/// after it (`place_options_from`), raises the scroll step to the tile's
/// height (`00404010`), gives the options with an id below 4 of the lists
/// numbered below 4 the sound `UIPipBoyMode` (string trait `0xfcb`) and,
/// when nothing is highlighted, highlights it (float trait `0xfd6` = the
/// float at `010162c0`). Returns the option, or null.
///
/// The compiler's exception-unwinding frame is not translated.
// Translation incomplete: 007b4650 - the C++ exception-unwinding frame (FS:[0]) that frees the option when its constructor throws is not translated
pub fn menu_option_list_add_menu_option(
    e: &mut Engine,
    this: Ptr<MenuOptionList>,
    caption: u32,
    template: u32,
    id: u32,
    before: u32,
) -> Ptr<MenuOption> {
    let me: u32 = e.global(RACE_SEX_MENU_ME);
    if caption == 0
        || me == 0
        || e.mem.u32(me + 0x78) == 0
        || e.get(this, MenuOptionList::pTemplateTile) == 0
    {
        return Ptr::new(0);
    }
    let memory = e.call(ALLOCATE, &args![0xcu32]).u32();
    let option = if memory != 0 {
        fn_007b4900(e, memory)
    } else {
        0
    };
    let tile = e.get(this, MenuOptionList::pTemplateTile);
    let number = e.call(GET_FLOAT_TRAIT, &args![tile, 0x1004u32]).f32();
    let number = e.call(FTOL, &args![number as f64]).i32();
    e.mem.set_u32(option + 4, id);
    let me: u32 = e.global(RACE_SEX_MENU_ME);
    let first_tile = e.mem.u32(me + 0x78);
    let rendered = e
        .call(RENDER_TEMPLATE, &args![me, first_tile, template, 0u32])
        .u32();
    e.mem.set_u32(option, rendered);
    let tile = e.mem.u32(option);
    e.call(SET_STRING_TRAIT, &args![tile, TRAIT_STRING, caption, 1u32]);
    let tile = e.mem.u32(option);
    e.call(SET_TRAIT_FROM_INT, &args![tile, 0x1004u32, number]);
    let tile = e.mem.u32(option);
    e.call(SET_TRAIT_FROM_INT, &args![tile, 0xfafu32, 1u32]);
    let option = e.with_stack(8, |e, locals| {
        let item = locals.addr();
        let cursor = item + 4;
        e.mem.set_u32(item, option);
        if before != 0 {
            e.call(LIST_INSERT_BEFORE, &args![this, before, item]);
        } else {
            e.call(LIST_INSERT_ITEM, &args![this, item]);
        }
        let node = e.call(LIST_FIND_NODE, &args![this, item, 0u32]).u32();
        e.mem.set_u32(cursor, node);
        place_options_from(e, this.addr(), cursor);
        e.mem.u32(item)
    });
    let tile = e.mem.u32(option);
    let height = e.call(GET_FLOAT_TRAIT, &args![tile, 0xfb0u32]).f32();
    let step = quiet_f32(e.get(this, MenuOptionList::fScrollDelta));
    let larger = e.call(FLOAT_MAX, &args![step, height]).f32();
    e.set(this, MenuOptionList::fScrollDelta, larger);
    if number < 4 && e.mem.u32(option + 4) < 4 {
        let tile = e.mem.u32(option);
        e.call(
            SET_STRING_TRAIT,
            &args![tile, 0xfcbu32, SOUND_PIPBOY_MODE, 1u32],
        );
    }
    if e.get(this, MenuOptionList::pHighlighted) == 0 {
        e.set(this, MenuOptionList::pHighlighted, option);
        let value = x87_copy_f32(e, HIGHLIGHT_VALUE);
        let tile = e.mem.u32(option);
        e.call(SET_FLOAT_TRAIT, &args![tile, 0xfd6u32, value, 1u32]);
    }
    Ptr::new(option)
}

// Translated from 007b4900 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `MenuOption` constructor: clears the tile (`+0`) and the data word
/// (`+8`) and returns the object; the id (`+4`) is left as it is. (Ghidra
/// matches the body to a library constructor.)
pub fn fn_007b4900(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, 0);
    e.mem.set_u32(this + 8, 0);
    this
}

/// `SetValue(value)` on the pressed option (`+0xC4`); answers whether the
/// value changed.
fn set_pressed_option_value(e: &mut Engine, base: u32, value: i32) -> bool {
    let option = e.mem.u32(base + 0xc4);
    menu_option_set_value(e, Ptr::new(option), value)
}

// Translated from 007b4930 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a pressed tile (`+0xC0`) and option (`+0xC4`): reads the tile's
/// id (float trait `0xfaa`, `_ftol`). For the colour option (`0x1d`) the
/// arrows 100 and 0x68 count as a change when the parent tile's string
/// `0x1009` equals the text of the setting `011d3528`; otherwise 100 /
/// 0x68 step the value down / up by the option's step count (`007b3900`),
/// 0x66 / 0x67 by the parent tile's trait `0x1008`, and 0x69 counts as a
/// change when flag 2 is set. A change runs `SliderRelease` and plays the
/// tile's sound `0xfcb`. Then clears flag 2 and both fields.
pub fn fn_007b4930(e: &mut Engine, this: Ptr<RaceSexMenu>) {
    let base = this.addr();
    if e.mem.u32(base + 0xc0) == 0 || e.mem.u32(base + 0xc4) == 0 {
        return;
    }
    let tile = e.mem.u32(base + 0xc0);
    let raw = e.call(GET_FLOAT_TRAIT, &args![tile, 0xfaau32]).f32();
    let button = e.call(FTOL, &args![raw as f64]).i32();
    let mut changed = false;
    let mut decided = false;
    let option = e.mem.u32(base + 0xc4);
    if word_at_four(e, option) == 0x1d && (button == 100 || button == 0x68) {
        let text = e.call(SETTING_VALUE, &args![SETTING_3528]).u32();
        let tile = e.mem.u32(base + 0xc0);
        let parent = e.call(WORD_AT_0X28, &args![tile]).u32();
        let shown = e.call(TILE_GET_STRING, &args![parent, 0x1009u32]).u32();
        if e.call(STRING_COMPARE, &args![shown, text]).u32() == 0 {
            changed = true;
            decided = true;
        }
    }
    if !decided {
        match button {
            100 | 0x68 => {
                let option = e.mem.u32(base + 0xc4);
                let value = menu_option_get_value(e, Ptr::new(option));
                let option = e.mem.u32(base + 0xc4);
                let step = fn_007b3900(e, Ptr::new(option));
                let value = if button == 100 {
                    value.wrapping_sub(step)
                } else {
                    value.wrapping_add(step)
                };
                changed = set_pressed_option_value(e, base, value);
            }
            0x66 | 0x67 => {
                let option = e.mem.u32(base + 0xc4);
                let value = menu_option_get_value(e, Ptr::new(option));
                let tile = e.mem.u32(base + 0xc0);
                let parent = e.call(WORD_AT_0X28, &args![tile]).u32();
                let raw = e.call(GET_FLOAT_TRAIT, &args![parent, 0x1008u32]).f32();
                let step = e.call(FTOL, &args![raw as f64]).i32();
                let value = if button == 0x66 {
                    value.wrapping_sub(step)
                } else {
                    value.wrapping_add(step)
                };
                changed = set_pressed_option_value(e, base, value);
            }
            0x69 => changed = race_sex_menu_get_flag(e, this, 2),
            _ => {}
        }
    }
    if changed {
        race_sex_menu_slider_release(e, this);
        let tile = e.mem.u32(base + 0xc0);
        e.call(PLAY_TILE_SOUND, &args![tile, 0xfcbu32]);
    }
    race_sex_menu_set_flag(e, this, 2, false);
    e.mem.set_u32(base + 0xc0, 0);
    e.mem.set_u32(base + 0xc4, 0);
}

/// A face value `x` (compared as a double; `copy` is its float) limited
/// the way `SliderRelease` limits both: the float at `0101e580` unless `x`
/// is above the double at `01015a38` (NaN takes the float), then the float
/// at `0102243c` unless that result is below the double at `01075b90`
/// (when it is, the first step is repeated).
fn limit_face_value(e: &Engine, x: f64, copy: f32) -> f32 {
    let lower_or = |e: &Engine| {
        let low: f64 = e.global(LOWER_BOUND_DOUBLE);
        if x > low {
            quiet_f32(copy)
        } else {
            x87_copy_f32(e, LOWER_BOUND)
        }
    };
    let first = lower_or(e);
    let high: f64 = e.global(UPPER_BOUND_DOUBLE);
    if (first as f64) < high {
        quiet_f32(lower_or(e))
    } else {
        x87_copy_f32(e, UPPER_BOUND)
    }
}

/// The face refresh `Randomize` and `SliderRelease` run on the open menu:
/// `007b18f0` with 0x22 and 0x23, then `007b1e70(1, 1, 0)`.
fn refresh_face_of_open_menu(e: &mut Engine) {
    let me = Ptr::new(e.global::<u32>(RACE_SEX_MENU_ME));
    fn_007b18f0(e, me, 0x22);
    let me = Ptr::new(e.global::<u32>(RACE_SEX_MENU_ME));
    fn_007b18f0(e, me, 0x23);
    let me = Ptr::new(e.global::<u32>(RACE_SEX_MENU_ME));
    fn_007b1e70(e, me, true, true, false);
}

/// The `0x1a` case of `SliderRelease` (see there), with the two face
/// coordinate arrays on the game's stack (`current` at the lower address).
fn slider_release_face_value(e: &mut Engine, base: u32, shape: i32) {
    e.with_stack(0x100, |e, arrays| {
        let current = arrays.addr();
        let initial = current + 0x80;
        for array in [current, initial] {
            e.call(
                EH_VECTOR_CONSTRUCTOR_ITERATOR,
                &args![
                    array,
                    0x20u32,
                    4u32,
                    FACE_COORD_CONSTRUCT,
                    FACE_COORD_DESTRUCT
                ],
            );
        }
        e.call(INIT_FACE_GEN_COORD, &args![initial]);
        let player_base = e.mem.u32(base + 0xd8);
        e.call(GET_FACE_COORD, &args![player_base, current]);
        let player_base = e.mem.u32(base + 0xd8);
        e.call(GET_FACE_COORD, &args![player_base, base + 0x124]);
        let first = e.call(FACE_COORD_GET, &args![current, 0u32, 0u32]).f32();
        let second = e.call(FACE_COORD_GET, &args![current, 0u32, 1u32]).f32();
        let other = (second as f64 - (first as f64 - shape as f64)) as f32;
        let limited = limit_face_value(e, shape as f64, shape as f32);
        let shape = e.call(FTOL, &args![limited as f64]).i32();
        let other = limit_face_value(e, other as f64, other);
        e.call(FACE_COORD_SET, &args![current, 0u32, 0u32, shape as f32]);
        e.call(FACE_COORD_SET, &args![current, 0u32, 1u32, other]);
        let player_base = e.mem.u32(base + 0xd8);
        let sex = e.call(GET_SEX, &args![player_base]).u32();
        let player_base = e.mem.u32(base + 0xd8);
        let race_word = word_at_four(e, player_base + 0x10c);
        let race = e.call(FACE_VALUE_5D9FB0, &args![race_word, sex]).u32();
        e.call(REGENERATE_FACE, &args![race, current, initial, 0u32]);
        let player_base = e.mem.u32(base + 0xd8);
        let face = e.call(FACE_VALUE_601800, &args![player_base]).u32();
        if e.call(FACE_COORD_CHECK_652900, &args![initial, face]).u8() != 0 {
            let player_base = e.mem.u32(base + 0xd8);
            let face = e.call(FACE_VALUE_601800, &args![player_base]).u32();
            e.call(COPY_FACE_GEN_COORD, &args![initial, face, 0u32, 0u32]);
        }
        let me = Ptr::new(e.global::<u32>(RACE_SEX_MENU_ME));
        race_sex_menu_set_flag(e, me, 7, true);
        refresh_face_of_open_menu(e);
        let me = Ptr::new(e.global::<u32>(RACE_SEX_MENU_ME));
        race_sex_menu_set_flag(e, me, 8, false);
        let me = Ptr::new(e.global::<u32>(RACE_SEX_MENU_ME));
        race_sex_menu_set_flag(e, me, 9, true);
        for array in [initial, current] {
            e.call(
                EH_VECTOR_DESTRUCTOR_ITERATOR,
                &args![array, 0x20u32, 4u32, FACE_COORD_DESTRUCT],
            );
        }
    });
}

// Translated from 007b4b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::SliderRelease` (Xbox PDB): applies the dragged option
/// (`+0xC4`):
/// - in list 6: `OnHairColorSliderChange`;
/// - ids `0x22` / `0x23`: `OnFaceSliderChange`, `007b18f0(id)` and, with
///   flag 8, `007b1e70(0, 1, 0)` / `(0, 0, 1)` and clears flag 8;
/// - id `0x18` (the preset head): passes entry `value - 1` of the array at
///   `+0xDC` to the player base form (`00603790`), updates the selection
///   of the 20 lists, refreshes the face (`007b18f0` 0x22 and 0x23,
///   `007b1e70(1, 1, 0)`, `007b25a0`, `007b2b50(1)`, `007b27c0`), clears
///   flags 8 and 9, sets the hair colour sliders, resets the colour option
///   (`0x1d` of the list at `+0x40`: caption from the setting `011d3528`,
///   value its minimum) and sets flag 0;
/// - id `0x1a`: turns the value into a face value (`value * 5.55 + 5.6`,
///   the doubles at `01075ba0` / `01075b98`, `_ftol`), reads the face
///   coordinates, keeps the difference between coordinates `[0][0]` and
///   `[0][1]`, limits both to the bounds (15 to 65 in the exe), writes
///   them, regenerates the face (`00652e00`), copies the coordinates when
///   `00652900` answers true, and on the open menu sets flag 7, runs the
///   face refresh, clears flag 8 and sets flag 9;
/// - other ids: nothing (flag 7 stays as it is).
///
/// Every applied case ends by clearing flag 7.
///
/// The compiler's exception-unwinding frame is not translated.
// Translation incomplete: 007b4b50 - the C++ exception-unwinding frame (FS:[0]) that destroys the two face coordinate arrays is not translated
pub fn race_sex_menu_slider_release(e: &mut Engine, this: Ptr<RaceSexMenu>) {
    let base = this.addr();
    let option = e.mem.u32(base + 0xc4);
    if e.call(CURRENT_LIST_INDEX, &args![this]).u32() == 6 {
        race_sex_menu_on_hair_color_slider_change(e, this, option);
    } else if word_at_four(e, option) == 0x22 || word_at_four(e, option) == 0x23 {
        race_sex_menu_on_face_slider_change(e, this, option);
        let option = e.mem.u32(base + 0xc4);
        let id = word_at_four(e, option);
        fn_007b18f0(e, this, id);
        if race_sex_menu_get_flag(e, this, 8) {
            let option = e.mem.u32(base + 0xc4);
            if word_at_four(e, option) == 0x22 {
                fn_007b1e70(e, this, false, true, false);
            } else {
                fn_007b1e70(e, this, false, false, true);
            }
            race_sex_menu_set_flag(e, this, 8, false);
        }
    } else if word_at_four(e, option) == 0x18 {
        let value = menu_option_get_value(e, Ptr::new(option));
        let entry = e
            .call(ARRAY_ENTRY, &args![base + 0xdc, value.wrapping_sub(1)])
            .u32();
        let preset = e.mem.u32(entry);
        let player_base = e.mem.u32(base + 0xd8);
        e.call(NPC_METHOD_603790, &args![player_base, preset]);
        for index in 0..0x14u32 {
            let list = e.mem.u32(base + 0x28 + 4 * index);
            menu_option_list_update_current_selection(e, Ptr::new(list));
        }
        fn_007b18f0(e, this, 0x22);
        fn_007b18f0(e, this, 0x23);
        fn_007b1e70(e, this, true, true, false);
        fn_007b25a0(e, this);
        fn_007b2b50(e, this, true);
        fn_007b27c0(e, this);
        race_sex_menu_set_flag(e, this, 8, false);
        race_sex_menu_set_flag(e, this, 9, false);
        let player_base = e.mem.u32(base + 0xd8);
        let color = e.call(HAIR_COLOR_GETTER, &args![player_base]).u32();
        race_sex_menu_set_hair_color_sliders(e, this, color);
        let list = e.mem.u32(base + 0x40);
        let colour = fn_007b4210(e, Ptr::new(list), 0x1d);
        if colour != 0 {
            let text = e.call(SETTING_VALUE, &args![SETTING_3528]).u32();
            fn_007b3810(e, Ptr::new(colour), text);
            let min = e.with_stack(8, |e, range| {
                fn_007b3840(e, Ptr::new(colour), range.addr(), range.addr() + 4);
                e.mem.i32(range.addr())
            });
            menu_option_set_value(e, Ptr::new(colour), min);
        }
        race_sex_menu_set_flag(e, this, 0, true);
    } else if word_at_four(e, option) == 0x1a {
        let value = menu_option_get_value(e, Ptr::new(option));
        let scale: f64 = e.global(SHAPE_SCALE_DOUBLE);
        let offset: f64 = e.global(SHAPE_OFFSET_DOUBLE);
        let shape = e.call(FTOL, &args![value as f64 * scale + offset]).i32();
        slider_release_face_value(e, base, shape);
    } else {
        return;
    }
    race_sex_menu_set_flag(e, this, 7, false);
}

// Translated from 007b5180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::ChooseDefaultPreset` (Xbox PDB): rebuilds the preset head
/// list, makes its option `0x18` (of the list at `+0x30`) the dragged
/// option, sets it to its minimum, applies it (`SliderRelease`) and clears
/// the dragged option.
pub fn race_sex_menu_choose_default_preset(e: &mut Engine, this: Ptr<RaceSexMenu>) {
    let base = this.addr();
    race_sex_menu_make_preset_head_list(e, this);
    let list = e.mem.u32(base + 0x30);
    let option = fn_007b4210(e, Ptr::new(list), 0x18);
    e.mem.set_u32(base + 0xc4, option);
    let min = e.with_stack(8, |e, range| {
        let option = e.mem.u32(base + 0xc4);
        fn_007b3840(e, Ptr::new(option), range.addr(), range.addr() + 4);
        e.mem.i32(range.addr())
    });
    let option = e.mem.u32(base + 0xc4);
    menu_option_set_value(e, Ptr::new(option), min);
    race_sex_menu_slider_release(e, this);
    e.mem.set_u32(base + 0xc4, 0);
}

// Translated from 007b51f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RaceSexMenu::Randomize` (Xbox PDB): with an open menu, when the message
/// box answered 1 (`00703fa0`) or flag 7 is set: runs `00601830(1, 0, 1)`
/// on the player base form, sets flag 7, refreshes the face (`007b18f0`
/// 0x22 and 0x23, `007b1e70(1, 1, 0)`, `007b25a0`) and clears flags 8
/// and 9.
pub fn race_sex_menu_randomize(e: &mut Engine) {
    let me: u32 = e.global(RACE_SEX_MENU_ME);
    if me == 0 {
        return;
    }
    if e.call(GET_MESSAGE_MENU_RESULT, &args![]).u8() as i8 != 1
        && !race_sex_menu_get_flag(e, Ptr::new(me), 7)
    {
        return;
    }
    let me: u32 = e.global(RACE_SEX_MENU_ME);
    let player_base = e.mem.u32(me + 0xd8);
    e.call(NPC_METHOD_601830, &args![player_base, 1u32, 0u32, 1u32]);
    let me = Ptr::new(e.global::<u32>(RACE_SEX_MENU_ME));
    race_sex_menu_set_flag(e, me, 7, true);
    refresh_face_of_open_menu(e);
    let me = Ptr::new(e.global::<u32>(RACE_SEX_MENU_ME));
    fn_007b25a0(e, me);
    let me = Ptr::new(e.global::<u32>(RACE_SEX_MENU_ME));
    race_sex_menu_set_flag(e, me, 8, false);
    let me = Ptr::new(e.global::<u32>(RACE_SEX_MENU_ME));
    race_sex_menu_set_flag(e, me, 9, false);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x007ac1f0,
            race_sex_menu_race_sex_menu(Ptr<RaceSexMenu>) -> Ptr<RaceSexMenu>
        ),
        entry!(0x007ac4f0, race_sex_menu_get_class(Ptr<RaceSexMenu>) -> u32),
        entry!(
            0x007ac500,
            race_sex_menu_attach_tile_by_id(Ptr<RaceSexMenu>, i32, u32)
        ),
        entry!(
            0x007ac530,
            race_sex_menu_scalar_deleting_destructor(Ptr<RaceSexMenu>, u32) -> Ptr<RaceSexMenu>
        ),
        entry!(0x007ac560, race_sex_menu_destructor(Ptr<RaceSexMenu>)),
        entry!(
            0x007ac700,
            fn_007ac700(Ptr<MenuOptionList>, u32) -> Ptr<MenuOptionList>
        ),
        entry!(0x007ac730, race_sex_menu_create(u32) -> u32),
        entry!(0x007ac9c0, fn_007ac9c0() -> u32),
        entry!(
            0x007ac9f0,
            fn_007ac9f0(Ptr<FORenderedMenuRaceSex>) -> Ptr<FORenderedMenuRaceSex>
        ),
        entry!(
            0x007acb60,
            race_sex_menu_create_contents(Ptr<RaceSexMenu>, i32)
        ),
        entry!(0x007ad7f0, fn_007ad7f0(u32) -> bool),
        entry!(0x007ad860, fn_007ad860(u32) -> bool),
        entry!(0x007ad8d0, fn_007ad8d0(u32) -> bool),
        entry!(0x007ad940, fn_007ad940(u32) -> bool),
        entry!(0x007ad9c0, fn_007ad9c0(u32) -> bool),
        entry!(0x007ada40, race_sex_menu_close()),
        entry!(
            0x007adc30,
            race_sex_menu_get_flag(Ptr<RaceSexMenu>, u32) -> bool
        ),
        entry!(
            0x007adc60,
            race_sex_menu_set_flag(Ptr<RaceSexMenu>, u32, bool)
        ),
        entry!(0x007adcc0, fn_007adcc0(Ptr, u8)),
        entry!(
            0x007adce0,
            race_sex_menu_do_click(Ptr<RaceSexMenu>, i32, u32)
        ),
        entry!(0x007ae160, menu_option_is_selected(Ptr<MenuOption>) -> bool),
        entry!(
            0x007ae1b0,
            race_sex_menu_do_down_click(Ptr<RaceSexMenu>, i32, u32)
        ),
        entry!(
            0x007ae240,
            race_sex_menu_do_still_down(Ptr<RaceSexMenu>, i32, u32)
        ),
        entry!(0x007ae420, race_sex_menu_do_idle(Ptr<RaceSexMenu>)),
        entry!(
            0x007aecb0,
            race_sex_menu_do_gamepad(Ptr<RaceSexMenu>, i32, u32) -> bool
        ),
        entry!(
            0x007af180,
            race_sex_menu_set_active_list(Ptr<RaceSexMenu>, i32) -> u8
        ),
        entry!(0x007af300, fn_007af300(Ptr<RaceSexMenu>)),
        entry!(0x007af450, fn_007af450(Ptr<RaceSexMenu>)),
        entry!(
            0x007af520,
            race_sex_menu_validate_hair_option_list(Ptr<RaceSexMenu>)
        ),
        entry!(
            0x007af610,
            race_sex_menu_make_preset_head_list(Ptr<RaceSexMenu>)
        ),
        entry!(
            0x007af6b0,
            race_sex_menu_update_scroll_arrows(Ptr<RaceSexMenu>)
        ),
        entry!(
            0x007af770,
            race_sex_menu_on_face_slider_change(Ptr<RaceSexMenu>, u32)
        ),
        entry!(
            0x007af900,
            race_sex_menu_on_hair_color_slider_change(Ptr<RaceSexMenu>, u32)
        ),
        entry!(
            0x007afa40,
            race_sex_menu_set_hair_color_sliders(Ptr<RaceSexMenu>, u32)
        ),
        entry!(0x007afaf0, race_sex_menu_add_face_sliders(Ptr<RaceSexMenu>)),
        entry!(
            0x007b16d0,
            fn_007b16d0(Ptr<RaceSexMenu>, i32, i32, u32) -> bool
        ),
        entry!(0x007b18f0, fn_007b18f0(Ptr<RaceSexMenu>, u32)),
        entry!(
            0x007b1970,
            race_sex_menu_rotate_player(Ptr<RaceSexMenu>, f32, u32) -> u8
        ),
        entry!(0x007b1ae0, fn_007b1ae0(Ptr<RaceSexMenu>)),
        entry!(
            0x007b1b60,
            race_sex_menu_zoom_player(Ptr<RaceSexMenu>, f32, u32) -> u8
        ),
        entry!(0x007b1ca0, fn_007b1ca0(Ptr<RaceSexMenu>)),
        entry!(0x007b1e50, fn_007b1e50(u32) -> u32),
        entry!(0x007b1e70, fn_007b1e70(Ptr<RaceSexMenu>, bool, bool, bool)),
        entry!(0x007b2550, fn_007b2550(u32, u8)),
        entry!(0x007b2570, fn_007b2570(u32) -> u8),
        entry!(0x007b25a0, fn_007b25a0(Ptr<RaceSexMenu>)),
        entry!(0x007b27c0, fn_007b27c0(Ptr<RaceSexMenu>) -> bool),
        entry!(0x007b2b50, fn_007b2b50(Ptr<RaceSexMenu>, bool) -> bool),
        entry!(0x007b3660, fn_007b3660(Ptr<RaceSexMenu>)),
        entry!(0x007b3720, fn_007b3720(u32, u32)),
        entry!(0x007b3740, menu_option_get_value(Ptr<MenuOption>) -> i32),
        entry!(
            0x007b3760,
            menu_option_set_value(Ptr<MenuOption>, i32) -> bool
        ),
        entry!(0x007b3810, fn_007b3810(Ptr<MenuOption>, u32)),
        entry!(0x007b3840, fn_007b3840(Ptr<MenuOption>, u32, u32)),
        entry!(0x007b3890, menu_option_set_range(Ptr<MenuOption>, i32, i32)),
        entry!(0x007b3900, fn_007b3900(Ptr<MenuOption>) -> i32),
        entry!(0x007b3970, menu_option_set_enabled(Ptr<MenuOption>, bool)),
        entry!(0x007b39b0, fn_007b39b0(Ptr<MenuOption>) -> i32),
        entry!(
            0x007b39e0,
            menu_option_list_menu_option_list(Ptr<MenuOptionList>, u32, u32) -> Ptr<MenuOptionList>
        ),
        entry!(0x007b3af0, fn_007b3af0(Ptr<MenuOptionList>)),
        entry!(
            0x007b3b80,
            fn_007b3b80(Ptr<MenuOptionList>, u32, u32) -> Ptr<MenuOption>
        ),
        entry!(
            0x007b3bb0,
            fn_007b3bb0(Ptr<MenuOptionList>, u32, Ptr<MenuOptionList>) -> Ptr<MenuOption>
        ),
        entry!(
            0x007b3ca0,
            menu_option_list_add_slider_option(
                Ptr<MenuOptionList>,
                u32,
                u32,
                i32,
                i32,
                i32,
                f32,
            ) -> Ptr<MenuOption>
        ),
        entry!(
            0x007b3d50,
            menu_option_list_insert_menu_option(
                Ptr<MenuOptionList>,
                i32,
                u32,
                u32,
                u32,
            ) -> Ptr<MenuOption>
        ),
        entry!(
            0x007b3dd0,
            menu_option_list_remove_menu_option(Ptr<MenuOptionList>, u32)
        ),
        entry!(0x007b3fa0, fn_007b3fa0(u32, u32) -> u32),
        entry!(
            0x007b3fd0,
            menu_option_list_focus(Ptr<MenuOptionList>, bool) -> u8
        ),
        entry!(
            0x007b4050,
            fn_007b4050(Ptr<MenuOptionList>, Ptr<MenuOptionList>)
        ),
        entry!(
            0x007b40d0,
            menu_option_list_update_current_selection(Ptr<MenuOptionList>)
        ),
        entry!(0x007b4140, fn_007b4140(Ptr<MenuOption>)),
        entry!(0x007b4180, fn_007b4180(Ptr<MenuOption>)),
        entry!(0x007b41c0, fn_007b41c0(Ptr<MenuOptionList>, u32) -> u32),
        entry!(0x007b4210, fn_007b4210(Ptr<MenuOptionList>, u32) -> u32),
        entry!(
            0x007b4260,
            menu_option_list_do_gamepad(Ptr<MenuOptionList>, i32) -> bool
        ),
        entry!(0x007b4410, fn_007b4410(Ptr<MenuOptionList>)),
        entry!(
            0x007b4530,
            menu_option_list_scroll_by(Ptr<MenuOptionList>, i32)
        ),
        entry!(0x007b45b0, fn_007b45b0(Ptr<MenuOptionList>)),
        entry!(
            0x007b4650,
            menu_option_list_add_menu_option(
                Ptr<MenuOptionList>,
                u32,
                u32,
                u32,
                u32,
            ) -> Ptr<MenuOption>
        ),
        entry!(0x007b4900, fn_007b4900(u32) -> u32),
        entry!(0x007b4930, fn_007b4930(Ptr<RaceSexMenu>)),
        entry!(0x007b4b50, race_sex_menu_slider_release(Ptr<RaceSexMenu>)),
        entry!(
            0x007b5180,
            race_sex_menu_choose_default_preset(Ptr<RaceSexMenu>)
        ),
        entry!(0x007b51f0, race_sex_menu_randomize()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The local the next `with_stack(size)` on the running thread takes (the called function's
    /// own frame, taken at the stack's current top).
    fn next_frame(e: &mut Engine, size: u32) -> u32 {
        e.next_stack_local(size)
    }

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn float(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// The argument words of every logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// The `f64` an argument pair (low word first) holds.
    fn double_arg(words: &[u32]) -> f64 {
        f64::from_bits(words[0] as u64 | (words[1] as u64) << 32)
    }

    /// Pages of the exe's data that the translations read.
    const DATA_PAGES: [u32; 24] = [
        0x0101_5000,
        0x0102_2000,
        0x011d_a000,
        0x011d_b000,
        0x011d_e000,
        0x011c_3000,
        0x011c_5000,
        0x0120_2000,
        0x011f_4000,
        0x011a_9000,
        0x011a_0000,
        0x0107_0000,
        0x0101_2000,
        0x0101_1000,
        0x0102_0000,
        0x0101_f000,
        0x0101_7000,
        0x0107_5000,
        0x0101_e000,
        0x0101_6000,
        0x0105_5000,
        0x0105_1000,
        0x0102_e000,
        0x0104_a000,
    ];

    /// Every function outside the unit that the translations call; the
    /// engine registers a do-nothing double over each.
    const CALLEES: [u32; 275] = [
        ABSOLUTE_VALUE,
        ACTOR_GET_HEIGHT,
        ADD_USER_TRAIT,
        ALLOCATE,
        APPLY_COORDS,
        APPLY_FACE_SLIDER,
        APPLY_HAIR_VALUE,
        APPLY_TEXTURING,
        APPLY_TEXTURING_ARG,
        AUDIO_INSTANCE,
        AUDIO_MANAGER_INSTANCE,
        AXIS_REDUCE,
        AXIS_VALUE,
        BASE_VALUE_4AC110,
        BASE_VALUE_602150,
        BASE_VALUE_603B50,
        BUTTON_STATE,
        CAST_607310,
        CHILD_AT,
        CHILD_COUNT,
        CLEAR_BODY_TEXTURES,
        CLEAR_HEAD,
        COLOR_A_CONSTRUCT,
        CREATE_TEXTURE_IMAGE,
        CURRENT_LIST_INDEX,
        DIALOGUE_ACTIVE,
        DIALOG_SERVICE_END,
        EH_VECTOR_CONSTRUCTOR_ITERATOR,
        EH_VECTOR_DESTRUCTOR_ITERATOR,
        EMPTY_MEMBER,
        EMPTY_MEMBER_RETURNS_THIS,
        ENTER_RENDERED_MENU,
        EQUIP_OBJECT,
        EYE_CHECK_A,
        EYE_CHECK_B,
        EYE_LIST_HEAD,
        FACE_BUILD,
        FACE_COORD_CONSTRUCT,
        FACE_COORD_DESTRUCT,
        FACE_DATA_CONSTRUCT,
        FACE_DATA_DESTRUCT,
        FACE_FILL_COORDS,
        FACE_GEN_MANAGER_INSTANCE,
        FACE_PART_NAME,
        FACE_SHAPE_VALUE,
        FACE_VALUE_5D9FB0,
        FACE_VALUE_601800,
        FILL_PRESET_HEADS,
        FIND_BY_NAME,
        FIXED_STRING_DESTRUCT,
        FIXED_STRING_FROM,
        FIXED_STRING_SET,
        FLOAT_MAX,
        FLOAT_MIN,
        FLOAT_SETTING_GETTER,
        FOCUS_TILE,
        FORM_UPDATE_6062E0,
        FREE,
        FTOL,
        GAMEPAD_IN_USE,
        GET_CURRENT_TEX_LANGUAGE,
        GET_FACE_COORD,
        GET_FACE_GEN_DATA,
        GET_FLOAT_TRAIT,
        GET_INVENTORY_CHANGES,
        GET_MAXIMUM_DEPTH,
        GET_MESSAGE_MENU_RESULT,
        GET_PIPBOY,
        GET_PROPERTY,
        GET_ROTATE,
        GET_SEX,
        GET_SOUND_HANDLE_BY_NAME,
        GET_SOUND_HANDLE_BY_NAME_ON_AUDIO,
        GET_TIME,
        GET_TOPIC,
        GET_WORN_ITEM,
        HAIR_CHECK_A,
        HAIR_CHECK_B,
        HAIR_CHECK_FEMALE,
        HAIR_CHECK_MALE,
        HAIR_COLOR_GETTER,
        HAIR_LIST_HEAD,
        HAIR_VALUE_602130,
        HEAD_BUILD,
        HEAD_PART_LIST,
        HEAD_PART_LIST_ADD,
        HEAD_PART_LIST_CLEAR,
        HEAD_REFRESH,
        INIT_HEAD,
        INPUT_DEVICE,
        INTERFACE_SINGLETON,
        IS_TOP_MENU_ID,
        KILL_EGT_DATA,
        LIGHT_BUTTON,
        LIST_ADVANCE,
        LIST_BACK_TARGET,
        LIST_IS_EMPTY,
        LIST_PARENT_TARGET,
        LIST_SET_BACK_TARGET,
        LOAD_QUEUED_PRIORITY,
        MARKER_CHECK,
        MARKER_CHECK_2,
        MARKER_LIST_HEAD,
        MARKER_LIST_HEAD_2,
        MARKER_NAME,
        MATRIX_MULTIPLY,
        MATRIX_SET_ELEMENT,
        MATRIX_SET_ROTATION,
        MEMSET,
        MENU_CONSTRUCT,
        MENU_DESTRUCT,
        MENU_INSTANT_FADE_OUT,
        MENU_PREP_FOR_VISIBILITY,
        MENU_SET_MENU_TILE,
        MULTIPLE_MATRIX_BY_RACE,
        NAME_COMPARE,
        NAME_HASH,
        NI_POINTER_ASSIGN,
        NI_POINTER_CONSTRUCT,
        NI_POINTER_DEFAULT_CONSTRUCT,
        NI_POINTER_DESTRUCT,
        OBJECT_9611E0,
        OBJECT_NAME,
        OPTION_AT_TILE,
        OPTION_LIST_ADD_OPTION,
        OPTION_LIST_ADD_SLIDER,
        OPTION_LIST_ADD_SUB_LIST,
        OPTION_LIST_BASE_CONSTRUCT,
        OPTION_LIST_BASE_DESTRUCT,
        OPTION_LIST_CLEAR,
        OPTION_LIST_CURRENT,
        OPTION_LIST_DO_GAMEPAD,
        OPTION_LIST_FIND,
        OPTION_LIST_FOCUS,
        OPTION_LIST_INSERT_OPTION,
        OPTION_LIST_LINK,
        OPTION_LIST_REFRESH,
        OPTION_LIST_REMOVE_OPTION,
        OPTION_LIST_SCROLL_BY,
        OPTION_LIST_SET_CALLBACK,
        OPTION_LIST_UPDATE_CURRENT_SELECTION,
        OPTION_SET_DATA,
        PALETTE_ENTRY_RELEASE,
        PALETTE_FIND,
        PALETTE_OF,
        PLAYER_ANIMATION,
        PLAYER_BASE_HEIGHT,
        PLAYER_BASE_WEIGHT,
        PLAYER_BIPED,
        PLAYER_FLAG_GETTER,
        PLAYER_FLAG_SETTER,
        PLAYER_NODE_GETTER,
        PLAYER_RACE_CHANGED,
        PLAYER_RECALCULATE,
        PLAYER_REFRESH_3D,
        PLAYER_UPDATE,
        PLAYER_VALUE_87F4C0,
        POINTER_X,
        POINT_CONSTRUCT,
        PREPARE_OBJECT,
        PRESET_HEADS_CONSTRUCT,
        PRESET_HEADS_DESTRUCT,
        PROPERTY_KIND,
        QUEUED_HEAD_APPLY,
        QUEUED_HEAD_CONSTRUCT,
        QUEUED_HEAD_DESTRUCT,
        QUEUED_HEAD_FINISH,
        QUEUED_HEAD_RUN,
        RACE_DEFAULT_HAIR,
        RACE_HAIR_LIST_HEAD,
        REGENERATE_FACE,
        REMOVE_FROM_PALETTE,
        RENDERED_MENU_BASE_CONSTRUCT,
        RENDERED_MENU_GETTER,
        RENDERED_MENU_SETTING,
        RENDERED_MENU_SET_MODE,
        RENDERED_MENU_SET_POINTER,
        RENDERED_MENU_SET_ROOT,
        RENDERED_MENU_SET_SCALE,
        RENDER_TEMPLATE,
        ROUND_TO_STEP,
        RT_DYNAMIC_CAST,
        SAVED_OBJECT,
        SCENE_NODE_GETTER,
        SETTINGS_COLLECTION,
        SETTINGS_FIND,
        SETTING_FLOAT_POINTER,
        SETTING_INT_POINTER,
        SETTING_TEXT,
        SETTING_VALUE,
        SET_CURRENT_FOCUS_TARGET,
        SET_FLAG_BIT,
        SET_FLOAT_TRAIT,
        SET_FOCUS_TARGET_CDECL,
        SET_HAIR,
        SET_HEAD,
        SET_RACE,
        SET_ROTATE,
        SET_STRING_TRAIT,
        SET_TRAIT_FROM_INT,
        SET_TRANSLATE,
        SET_TRANSLATE_POINT,
        SHAPE_COUNT,
        SHOW_MESSAGE_BOX,
        SIMPLE_ARRAY_CLEAR,
        SIMPLE_ARRAY_IS_EMPTY,
        SLIDER_PRESS,
        SLIDER_RELEASE,
        SOUND_HANDLE_CONSTRUCT,
        SOUND_HANDLE_COPY,
        SOUND_HANDLE_IS_PLAYING,
        SOUND_HANDLE_PAUSE,
        SOUND_HANDLE_PLAY,
        SOUND_HANDLE_RELEASE,
        SOUND_HANDLE_SET_BYTE,
        SOUND_HANDLE_STOP,
        STRING_CONSTRUCT,
        STRING_C_STR,
        STRING_DESTROY,
        STRING_FORMAT,
        STRING_LENGTH,
        STRING_SET,
        STRLEN,
        TILE_GET_CHILD_BY_ID,
        TILE_GET_MENU,
        TILE_GET_MENU_BY_CLASS,
        TILE_GET_VALUE,
        TILE_IS_TRUE,
        TILE_IS_VISIBLE,
        TILE_NAME_STRING,
        TILE_READER,
        TILE_READ_FILE,
        TILE_UPDATE_ALL,
        TILE_VALUE_SET_FLOAT,
        TO_EULER_ANGLES,
        TUTORIAL_SHOW_MESSAGE,
        UNEQUIP_OBJECT,
        UPDATE_OBJECT_LIGHTING,
        UPDATE_PLAYER_NAME,
        UPDATE_PROPERTIES,
        UPDATE_TILE,
        STRING_UPPER_CASE,
        CAPTION_UPPER_CASE,
        VECTOR_CONSTRUCTOR_ITERATOR,
        WHEEL_DELTA,
        WORD_AT_0X28,
        WORD_AT_EIGHT,
        ANIMATION_OBJECT,
        WORN_ITEM_DELETE,
        WORD_AT_FOUR,
        CHOOSE_DEFAULT_PRESET,
        RANDOMIZE,
        PLAYER_BASE_FORM,
        LIST_NODE_NEXT,
        LIST_NODE_PREVIOUS,
        LIST_NODE_DATA,
        LIST_HEAD_DATA,
        LIST_TAIL_DATA,
        LIST_FIND_NODE,
        LIST_INSERT_ITEM,
        LIST_INSERT_BEFORE,
        LIST_REMOVE_ITEM,
        LIST_REMOVE_ALL,
        TILE_GET_STRING,
        STRING_FROM_TEXT,
        STRING_COMPARE,
        PLAY_TILE_SOUND,
        INIT_FACE_GEN_COORD,
        FACE_COORD_GET,
        FACE_COORD_SET,
        FACE_COORD_CHECK_652900,
        COPY_FACE_GEN_COORD,
        ARRAY_ENTRY,
        NPC_METHOD_603790,
        NPC_METHOD_601830,
    ];

    const PLAYER_VTABLE: u32 = 0x0f40_0000;
    const PLAYER_SLOTS: u32 = 0x0f41_0000;
    const MENU_VTABLE: u32 = 0x0f42_0000;
    const MENU_SLOTS: u32 = 0x0f43_0000;
    const BASE_VTABLE: u32 = 0x0f44_0000;
    const BASE_SLOTS: u32 = 0x0f45_0000;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in DATA_PAGES {
            e.map(page, 0x1000);
        }
        for addr in CALLEES {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(WORD_AT_FOUR, |e, a| word(e.mem.u32(a[0] + 4)));
        e.register(WORD_AT_EIGHT, |e, a| word(e.mem.u32(a[0] + 8)));
        e.register(WORD_AT_0X28, |e, a| word(e.mem.u32(a[0] + 0x28)));
        e.register(STRING_C_STR, |e, a| word(e.mem.u32(a[0])));
        e.register(EMPTY_MEMBER_RETURNS_THIS, |_, a| word(a[0]));
        e.register(SETTING_VALUE, |_, a| word(a[0]));
        e.register(SETTING_TEXT, |_, a| word(a[0]));
        e.register(ALLOCATE, |e, a| word(e.mem.alloc(a[0])));
        e.register(FREE, |e, a| {
            e.mem.free(a[0]);
            Ret::default()
        });
        e.register(MEMSET, |e, a| {
            for offset in 0..a[2] {
                e.mem.set_u8(a[0] + offset, a[1] as u8);
            }
            Ret::default()
        });
        e.register(FTOL, |_, a| word(double_arg(a) as i32 as u32));
        e.register(FLOAT_MIN, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            float(if y > x { x } else { y } as f64)
        });
        e.register(FLOAT_MAX, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            float(if y < x { x } else { y } as f64)
        });
        for addr in [
            OPTION_LIST_ADD_OPTION,
            OPTION_LIST_ADD_SLIDER,
            OPTION_LIST_FIND,
        ] {
            e.register(addr, |e, _| word(e.mem.alloc(0xc)));
        }
        e.register(MATRIX_MULTIPLY, |_, a| word(a[1]));
        let rotation = e.mem.alloc(0x24);
        e.register_double(GET_ROTATE, move |_, _| word(rotation));
        e.register(IS_TOP_MENU_ID, |_, _| word(1));
        e.call_log = Some(vec![]);
        e
    }

    /// An object with a vtable whose slot at byte offset `s` points at
    /// `slots + s` (a do-nothing double); returns the object.
    fn object_with_vtable(e: &mut Engine, size: u32, vtable: u32, slots: u32) -> u32 {
        let object = e.mem.alloc(size);
        let targets: Vec<u32> = (0..0x200).map(|i| slots + 4 * i).collect();
        e.put_vtable(vtable, &targets);
        for target in &targets {
            e.register(*target, |_, _| Ret::default());
        }
        e.mem.set_u32(object, vtable);
        object
    }

    /// The player: vtable at `PLAYER_VTABLE`, registered in the global.
    fn player(e: &mut Engine) -> u32 {
        let player = object_with_vtable(e, 0x800, PLAYER_VTABLE, PLAYER_SLOTS);
        e.set_global(PLAYER, player);
        player
    }

    /// A menu with the 20 option lists, the six tiles, the menu tile and a
    /// player base form (with a vtable), registered as `pMe`. The class slot
    /// (`+0x34`) answers 0x40c.
    fn menu(e: &mut Engine) -> Ptr<RaceSexMenu> {
        let menu = object_with_vtable(e, 0x1a4, MENU_VTABLE, MENU_SLOTS);
        e.register(MENU_SLOTS + 0x34, |_, _| word(0x40c));
        for index in 0..0x14u32 {
            let list = e.mem.alloc(0x24);
            e.mem.set_u32(menu + 0x28 + 4 * index, list);
        }
        for index in 0..6u32 {
            let tile = e.mem.alloc(0x40);
            e.mem.set_u32(menu + 0x78 + 4 * index, tile);
        }
        let menu_tile = e.mem.alloc(0x40);
        e.mem.set_u32(menu + 4, menu_tile);
        let base = object_with_vtable(e, 0x300, BASE_VTABLE, BASE_SLOTS);
        e.mem.set_u32(menu + 0xd8, base);
        e.set_global(RACE_SEX_MENU_ME, menu);
        Ptr::new(menu)
    }

    /// An option (0xC bytes) with a tile, an id and a data word.
    fn make_option(e: &mut Engine, tile: u32, id: u32, data: u32) -> u32 {
        let option = e.mem.alloc(0xc);
        e.mem.set_u32(option, tile);
        e.mem.set_u32(option + 4, id);
        e.mem.set_u32(option + 8, data);
        option
    }

    /// The `n`th option list of the menu.
    fn list_of(e: &Engine, menu: Ptr<RaceSexMenu>, n: u32) -> u32 {
        e.mem.u32(menu.addr() + 0x28 + 4 * n)
    }

    /// The `n`th tile of the menu.
    fn tile_of(e: &Engine, menu: Ptr<RaceSexMenu>, n: u32) -> u32 {
        e.mem.u32(menu.addr() + 0x78 + 4 * n)
    }

    /// A singly linked list with a head cell the owner's head getters point
    /// at: `cell + 0` the item, `cell + 4` the next cell. Returns the first.
    fn chain(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for item in items.iter().rev() {
            let cell = e.mem.alloc(8);
            e.mem.set_u32(cell, *item);
            e.mem.set_u32(cell + 4, next);
            next = cell;
        }
        next
    }

    /// Makes the engine's `LIST_ADVANCE` walk the cells `chain` builds.
    fn walk_chains(e: &mut Engine) {
        e.register(LIST_ADVANCE, |e, a| {
            let cell = e.mem.u32(a[1]);
            let next = e.mem.u32(cell + 4);
            e.mem.set_u32(a[1], next);
            word(cell)
        });
    }

    #[test]
    fn constructor_sets_up_the_menu() {
        let mut e = engine();
        e.register(GET_SOUND_HANDLE_BY_NAME, |_, a| word(a[2]));
        for (index, value) in [0x1111u32, 0x2222, 0x3333].iter().enumerate() {
            e.set_global(ZOOM_BOUND_DEFAULT + 4 * index as u32, *value);
        }
        let this = e.new_object::<RaceSexMenu>();
        e.mem.set_u32(this.addr() + 0x28, 0xdead);
        let result = e.call(0x007a_c1f0, &args![this]);
        assert_eq!(result.u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), RACE_SEX_MENU_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 0xec), 3);
        assert_eq!(e.mem.u32(this.addr() + 0x120), 0x14);
        assert_eq!(e.mem.u32(this.addr() + 0x28), 0);
        for word_index in 0..3u32 {
            let expected = [0x1111, 0x2222, 0x3333][word_index as usize];
            assert_eq!(e.mem.u32(this.addr() + 0xa0 + 4 * word_index), expected);
            assert_eq!(e.mem.u32(this.addr() + 0xac + 4 * word_index), expected);
        }
        assert_eq!(calls(&e, MENU_CONSTRUCT), vec![vec![this.addr()]]);
        let names: Vec<u32> = calls(&e, GET_SOUND_HANDLE_BY_NAME)
            .iter()
            .map(|c| c[2])
            .collect();
        assert_eq!(
            names,
            vec![
                SOUND_COMPUTER_TERMINAL,
                SOUND_BIRTH_ROOM,
                SOUND_FACE_GEN_ZOOM,
                SOUND_FACE_GEN_ROTATE
            ]
        );
        let slots: Vec<u32> = calls(&e, SOUND_HANDLE_COPY).iter().map(|c| c[0]).collect();
        assert_eq!(
            slots,
            vec![
                this.addr() + 0xf0,
                this.addr() + 0xfc,
                this.addr() + 0x108,
                this.addr() + 0x114
            ]
        );
    }

    #[test]
    fn get_class_answers_the_menu_class() {
        let mut e = engine();
        let this = e.new_object::<RaceSexMenu>();
        assert_eq!(e.call(0x007a_c4f0, &args![this]).u32(), 0x40c);
    }

    #[test]
    fn attach_tile_stores_ids_zero_to_five() {
        let mut e = engine();
        let this = e.new_object::<RaceSexMenu>();
        e.call(0x007a_c500, &args![this, 2i32, 0xaaaau32]);
        e.call(0x007a_c500, &args![this, -1i32, 0xbbbbu32]);
        e.call(0x007a_c500, &args![this, 6i32, 0xccccu32]);
        assert_eq!(e.mem.u32(this.addr() + 0x78 + 8), 0xaaaa);
        assert_eq!(e.mem.u32(this.addr() + 0x78 + 24), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x74), 0);
    }

    #[test]
    fn destructor_deletes_the_lists_and_the_sounds() {
        let mut e = engine();
        player(&mut e);
        let this = e.new_object::<RaceSexMenu>();
        let first = e.mem.alloc(0x24);
        let last = e.mem.alloc(0x24);
        e.mem.set_u32(this.addr() + 0x28, first);
        e.mem.set_u32(this.addr() + 0x28 + 4 * 19, last);
        e.call(0x007a_c560, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), RACE_SEX_MENU_VTABLE);
        // The two lists are destroyed (base destructor) and freed.
        assert_eq!(
            calls(&e, OPTION_LIST_BASE_DESTRUCT),
            vec![vec![first], vec![last]]
        );
        assert_eq!(calls(&e, FREE), vec![vec![first], vec![last]]);
        assert_eq!(calls(&e, ENTER_RENDERED_MENU).len(), 1);
        assert_eq!(calls(&e, KILL_EGT_DATA).len(), 1);
        assert_eq!(calls(&e, SOUND_HANDLE_STOP).len(), 4);
        assert_eq!(calls(&e, SOUND_HANDLE_RELEASE).len(), 4);
        assert_eq!(calls(&e, STRING_DESTROY).len(), 2);
        assert_eq!(calls(&e, MENU_DESTRUCT), vec![vec![this.addr()]]);
    }

    #[test]
    fn deleting_destructor_frees_only_with_the_flag() {
        let mut e = engine();
        player(&mut e);
        let this = e.new_object::<RaceSexMenu>();
        let result = e.call(0x007a_c530, &args![this, 0u32]);
        assert_eq!(result.u32(), this.addr());
        assert!(calls(&e, FREE).is_empty());
        assert_eq!(calls(&e, MENU_DESTRUCT).len(), 1);
        e.call(0x007a_c530, &args![this, 1u32]);
        assert_eq!(calls(&e, FREE), vec![vec![this.addr()]]);
    }

    #[test]
    fn list_deleting_destructor_frees_only_with_the_flag() {
        let mut e = engine();
        let list = e.new_object::<MenuOptionList>();
        let template = object_with_vtable(&mut e, 0x40, 0x0f46_0000, 0x0f47_0000);
        e.mem.set_u32(list.addr() + 0x14, template);
        let result = e.call(0x007a_c700, &args![list, 0u32]);
        assert_eq!(result.u32(), list.addr());
        assert!(calls(&e, FREE).is_empty());
        // The template tile was deleted through its slot 0 and cleared.
        assert_eq!(e.mem.u32(list.addr() + 0x14), 0);
        assert_eq!(calls(&e, OPTION_LIST_CLEAR), vec![vec![list.addr()]]);
        assert_eq!(calls(&e, OPTION_LIST_BASE_DESTRUCT).len(), 1);
        e.call(0x007a_c700, &args![list, 1u32]);
        assert_eq!(calls(&e, FREE), vec![vec![list.addr()]]);
    }

    #[test]
    fn create_returns_an_existing_menu_tile() {
        let mut e = engine();
        e.register(TILE_GET_MENU_BY_CLASS, |_, a| {
            assert_eq!(a, [0x40c]);
            word(0x1234)
        });
        assert_eq!(e.call(0x007a_c730, &args![2u32]).u32(), 0x1234);
        assert!(calls(&e, TILE_READ_FILE).is_empty());
    }

    #[test]
    fn create_stops_when_the_file_does_not_load() {
        let mut e = engine();
        e.register(TILE_READ_FILE, |_, a| {
            assert_eq!(a[1], MENU_FILE);
            word(0)
        });
        assert_eq!(e.call(0x007a_c730, &args![0u32]).u32(), 0);
        assert!(calls(&e, TILE_GET_MENU).is_empty());
        assert_eq!(e.global::<u32>(RACE_SEX_MENU_ME), 0);
    }

    /// A world where `Create` loads the file: the menu comes from
    /// `TILE_GET_MENU`, the worn-item slot answers `slot + 1`.
    fn create_world(rendered_setting: u8) -> (Engine, Ptr<RaceSexMenu>, u32) {
        let mut e = engine();
        let player = player(&mut e);
        let menu_object = menu(&mut e);
        e.set_global(RACE_SEX_MENU_ME, 0u32);
        let root = e.mem.alloc(0x40);
        let menu_addr = menu_object.addr();
        e.register_double(TILE_READ_FILE, move |_, _| word(root));
        e.register_double(TILE_GET_MENU, move |_, _| word(menu_addr));
        e.register(GET_WORN_ITEM, |e, _| word(e.mem.alloc(0x20)));
        e.register(GET_INVENTORY_CHANGES, |_, _| word(0x7700));
        let flag = e.mem.alloc(8);
        e.mem.set_u8(flag, rendered_setting);
        e.register_double(RENDERED_MENU_SETTING, move |_, _| word(flag));
        let base = e.mem.u32(menu_addr + 0xd8);
        e.register_double(PLAYER_BASE_FORM, move |_, _| word(base));
        e.register(RT_DYNAMIC_CAST, |_, a| word(a[0]));
        let float_slot = e.mem.alloc(8);
        e.register_double(SETTING_FLOAT_POINTER, move |_, _| word(float_slot));
        e.register(GET_FLOAT_TRAIT, |_, _| float(0.0));
        e.call_log = Some(vec![]);
        e.set_global(PLAYER, player);
        (e, menu_object, root)
    }

    #[test]
    fn create_builds_the_menu_and_takes_off_the_clothes() {
        let (mut e, menu_object, root) = create_world(0);
        let result = e.call(0x007a_c730, &args![0u32]);
        assert_eq!(result.u32(), root);
        assert_eq!(e.global::<u32>(RACE_SEX_MENU_ME), menu_object.addr());
        assert_eq!(
            calls(&e, MENU_SET_MENU_TILE),
            vec![vec![menu_object.addr(), root, 0]]
        );
        // The three empty captions are filled and (language != 3) converted.
        let filled: Vec<u32> = calls(&e, FIXED_STRING_SET).iter().map(|c| c[0]).collect();
        assert_eq!(filled, vec![CAPTION_A, CAPTION_B, CAPTION_C]);
        assert_eq!(calls(&e, CAPTION_UPPER_CASE).len(), 3);
        // Ten slots are taken off and remembered for `Close`.
        let slots: Vec<u32> = calls(&e, GET_WORN_ITEM).iter().map(|c| c[1]).collect();
        assert_eq!(slots, vec![0, 1, 5, 7, 8, 9, 10, 11, 14, 15]);
        assert_eq!(calls(&e, UNEQUIP_OBJECT).len(), 10);
        assert_ne!(e.mem.u32(SAVED_WORN_ITEMS + 4 * 5), 0);
        assert_eq!(e.mem.u32(SAVED_WORN_ITEMS + 4 * 2), 0);
        // The setting is off: no rendered menu.
        assert!(calls(&e, ALLOCATE).iter().all(|c| c[0] != 0x170));
        assert!(calls(&e, ENTER_RENDERED_MENU).is_empty());
    }

    #[test]
    fn create_enters_the_rendered_menu_when_the_setting_is_on() {
        let (mut e, _menu_object, root) = create_world(1);
        e.register(TILE_NAME_STRING, |_, _| word(0x5555));
        e.call(0x007a_c730, &args![3u32]);
        let rendered = calls(&e, ENTER_RENDERED_MENU)[0][1];
        assert_ne!(rendered, 0);
        assert!(calls(&e, TILE_UPDATE_ALL).contains(&vec![0]));
        assert_eq!(calls(&e, RENDERED_MENU_SET_MODE), vec![vec![rendered, 3]]);
        assert_eq!(
            calls(&e, RENDERED_MENU_SET_ROOT),
            vec![vec![rendered, root]]
        );
        assert_eq!(
            calls(&e, RENDERED_MENU_SET_POINTER),
            vec![vec![rendered, 0x5555]]
        );
        assert_eq!(e.mem.u32(rendered), RENDERED_MENU_VTABLE);
    }

    #[test]
    fn class_id_getter_answers_the_menu_class() {
        let mut e = engine();
        assert_eq!(e.call(0x007a_c9c0, &args![]).u32(), 0x40c);
    }

    #[test]
    fn rendered_menu_constructor_sets_the_vtable() {
        let mut e = engine();
        let this = e.new_object::<FORenderedMenuRaceSex>();
        let result = e.call(0x007a_c9f0, &args![this]);
        assert_eq!(result.u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), RENDERED_MENU_VTABLE);
        assert_eq!(
            calls(&e, RENDERED_MENU_BASE_CONSTRUCT),
            vec![vec![this.addr()]]
        );
        let pointers: Vec<u32> = calls(&e, NI_POINTER_CONSTRUCT)
            .iter()
            .map(|c| c[0] - this.addr())
            .collect();
        assert_eq!(pointers, vec![0xf8, 0xfc, 0x100, 0x104, 0x108, 0x10c]);
        let vectors: Vec<u32> = calls(&e, EH_VECTOR_CONSTRUCTOR_ITERATOR)
            .iter()
            .map(|c| c[0] - this.addr())
            .collect();
        assert_eq!(vectors, vec![0xd8, 0xe8]);
    }

    /// A menu world for the functions that build or rebuild the menu: the
    /// player, the pipboy and the saved object have vtables, the base form
    /// getters answer a valid block, and the starting zoom setting is 2.0.
    fn world() -> (Engine, Ptr<RaceSexMenu>) {
        let mut e = engine();
        player(&mut e);
        let menu = menu(&mut e);
        let slot = e.mem.alloc(8);
        e.mem.set_f32(slot, 2.0);
        e.register_double(SETTING_FLOAT_POINTER, move |_, _| word(slot));
        let pipboy = object_with_vtable(&mut e, 0x40, 0x0f48_0000, 0x0f49_0000);
        e.register_double(GET_PIPBOY, move |_, _| word(pipboy));
        let saved = object_with_vtable(&mut e, 0x40, 0x0f4a_0000, 0x0f4b_0000);
        e.register_double(SAVED_OBJECT, move |_, _| word(saved));
        let race = e.mem.alloc(0x200);
        e.register_double(BASE_VALUE_4AC110, move |_, _| word(race));
        let base = e.mem.u32(menu.addr() + 0xd8);
        e.register_double(PLAYER_BASE_FORM, move |_, _| word(base));
        e.register(RT_DYNAMIC_CAST, |_, a| word(a[0]));
        let interface = e.mem.alloc(0x800);
        e.register_double(INTERFACE_SINGLETON, move |_, _| word(interface));
        e.call_log = Some(vec![]);
        (e, menu)
    }

    #[test]
    fn create_body_sets_up_the_default_mode() {
        let (mut e, menu) = world();
        e.register(GET_FLOAT_TRAIT, |_, a| {
            float(if a[1] == TRAIT_STACKING_TYPE {
                6006.0
            } else {
                0.0
            })
        });
        e.register(GET_MAXIMUM_DEPTH, |_, _| float(7.5));
        e.set_global(KIND_ONE_DOUBLE, 6006.0f64);
        let tile = e.mem.u32(menu.addr() + 4);
        let frame = next_frame(&mut e, 0xbc);
        e.call(0x007a_cb60, &args![menu, 0i32]);
        // The 0xbc-byte frame: the numbered captions at +0x40, the projector text at +0x2c.
        assert_eq!(
            calls(&e, EH_VECTOR_CONSTRUCTOR_ITERATOR)[0][0],
            frame + 0x40
        );
        assert!(calls(&e, STRING_CONSTRUCT).contains(&vec![frame + 0x2c]));
        // The stacking type 6006 gives the tile the maximum depth.
        assert_eq!(
            calls(&e, SET_FLOAT_TRAIT)[0],
            vec![tile, TRAIT_DEPTH, 7.5f32.to_bits(), 1]
        );
        let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
        assert_eq!(
            calls(&e, TUTORIAL_SHOW_MESSAGE),
            vec![vec![interface + 0x4d4, 0xf, 0x40c, 0x200]]
        );
        let strings = calls(&e, SET_STRING_TRAIT);
        assert_eq!(strings[0], vec![tile, 0x1005, GLYPH_LESS, 1]);
        assert_eq!(strings[1], vec![tile, 0x1006, GLYPH_GREATER, 1]);
        // The numbered captions are formatted in the default mode.
        let formats = calls(&e, STRING_FORMAT);
        for (index, call) in formats.iter().take(4).enumerate() {
            assert_eq!(call[1], NUMBERED_FORMATS[index]);
            assert_eq!(call[2], NUMBERED_SETTINGS[index]);
        }
        // Twenty lists of 0x24 bytes, each with a caption number.
        let blocks = calls(&e, ALLOCATE).iter().filter(|c| c[0] == 0x24).count();
        assert_eq!(blocks, 20);
        let callbacks = calls(&e, OPTION_LIST_SET_CALLBACK);
        let expected = [
            (0x28, 0x007a_d940),
            (0x2c, 0x007a_d7f0),
            (0x3c, 0x007a_d860),
            (0x48, 0x007a_d8d0),
            (0x44, 0x007a_d9c0),
        ];
        for (call, (slot, callback)) in callbacks.iter().zip(expected) {
            assert_eq!(call, &vec![e.mem.u32(menu.addr() + slot), callback]);
        }
        // The default mode: list 3, the zoom limited to 0..1, the lists linked.
        assert_eq!(e.mem.u32(menu.addr() + 0x120), 3);
        assert_eq!(e.mem.f32(menu.addr() + 0xd0), 1.0);
        assert_eq!(e.mem.f32(menu.addr() + 0xc8), 1.0);
        assert_eq!(calls(&e, OPTION_LIST_LINK).len(), 3);
        assert!(calls(&e, STRING_SET).contains(&vec![menu.addr() + 0x90, PROJECTOR_REFLECTRON, 0]));
        assert!(calls(&e, SET_TRAIT_FROM_INT).contains(&vec![tile, 0x1004, 0]));
        assert_eq!(
            calls(&e, MENU_PREP_FOR_VISIBILITY),
            vec![vec![menu.addr(), 0]]
        );
        assert_eq!(calls(&e, SOUND_HANDLE_PLAY).len(), 2);
    }

    #[test]
    fn create_body_sets_up_the_barber() {
        let (mut e, menu) = world();
        let sources = [
            0x0107_5aa8u32,
            0x0101_e33c,
            0x0107_5aa4,
            0x0107_5aa0,
            0x0105_10fc,
            0x0107_5a9c,
        ];
        for (index, address) in sources.iter().enumerate() {
            e.set_global(*address, (index + 1) as f32);
        }
        e.call(0x007a_cb60, &args![menu, 2i32]);
        assert_eq!(e.mem.u32(menu.addr() + 0x120), 3);
        for index in 0..6u32 {
            assert_eq!(
                e.mem.f32(menu.addr() + 0xa0 + 4 * index),
                (index + 1) as f32
            );
        }
        // Mode 2 sets the numbered captions instead of formatting them.
        let sets = calls(&e, STRING_SET);
        assert_eq!(sets.len(), 6);
        assert_eq!(sets[0][1], NUMBERED_SETTINGS[0]);
        assert!(sets.contains(&vec![menu.addr() + 0x90, PROJECTOR_BARBER, 0]));
        assert!(calls(&e, SIMPLE_ARRAY_CLEAR).is_empty());
        assert!(calls(&e, OPTION_LIST_LINK).is_empty());
    }

    #[test]
    fn create_body_sets_up_the_plastic_surgeon() {
        let (mut e, menu) = world();
        e.set_global(0x0107_5a70u32, 4.5f32);
        e.call(0x007a_cb60, &args![menu, 3i32]);
        assert_eq!(e.mem.u32(menu.addr() + 0x120), 2);
        assert_eq!(e.mem.f32(menu.addr() + 0xa0), 4.5);
        assert!(calls(&e, STRING_SET).contains(&vec![menu.addr() + 0x90, PROJECTOR_SURGERY, 0]));
        // The preset head list is rebuilt, list 2 becomes the active one.
        assert!(calls(&e, SIMPLE_ARRAY_CLEAR).contains(&vec![menu.addr() + 0xdc, 0]));
        let tile = e.mem.u32(menu.addr() + 4);
        assert!(calls(&e, SET_TRAIT_FROM_INT).contains(&vec![tile, 0x1004, 2]));
    }

    /// A menu whose base form object (`+0xD8`) and the option `0x4242` data
    /// word the option checks compare.
    fn check_world() -> (Engine, u32) {
        let mut e = engine();
        player(&mut e);
        menu(&mut e);
        let option = make_option(&mut e, 0x77, 0x15, 0x4242);
        (e, option)
    }

    #[test]
    fn option_checks_compare_the_base_form_values() {
        for (check, getter) in [
            (0x007a_d7f0u32, BASE_VALUE_4AC110),
            (0x007a_d860, BASE_VALUE_603B50),
            (0x007a_d8d0, BASE_VALUE_602150),
        ] {
            let (mut e, option) = check_world();
            e.register(getter, |_, _| word(0x4242));
            assert!(e.call(check, &args![option]).bool());
            e.register(getter, |_, _| word(0x1));
            assert!(!e.call(check, &args![option]).bool());
            // No option, no data word or no menu: false.
            assert!(!e.call(check, &args![0u32]).bool());
            let bare = e.mem.alloc(0xc);
            assert!(!e.call(check, &args![bare]).bool());
            e.set_global(RACE_SEX_MENU_ME, 0u32);
            assert!(!e.call(check, &args![option]).bool());
        }
    }

    #[test]
    fn sex_option_check_follows_the_sex_and_the_id() {
        let (mut e, male) = check_world();
        let female = make_option(&mut e, 0x77, 0x16, 0x4242);
        e.register(GET_SEX, |_, _| word(0));
        assert!(e.call(0x007a_d940, &args![male]).bool());
        assert!(!e.call(0x007a_d940, &args![female]).bool());
        e.register(GET_SEX, |_, _| word(1));
        assert!(!e.call(0x007a_d940, &args![male]).bool());
        assert!(e.call(0x007a_d940, &args![female]).bool());
        assert!(!e.call(0x007a_d940, &args![0u32]).bool());
        e.set_global(RACE_SEX_MENU_ME, 0u32);
        assert!(!e.call(0x007a_d940, &args![female]).bool());
    }

    #[test]
    fn head_part_check_compares_the_first_entry() {
        let (mut e, option) = check_world();
        let parts = e.mem.alloc(8);
        e.mem.set_u32(parts, 0x4242);
        e.register_double(HEAD_PART_LIST, move |_, _| word(parts));
        assert!(e.call(0x007a_d9c0, &args![option]).bool());
        e.mem.set_u32(parts, 0x1);
        assert!(!e.call(0x007a_d9c0, &args![option]).bool());
        e.mem.set_u32(parts, 0x4242);
        e.register(LIST_IS_EMPTY, |_, _| word(1));
        assert!(!e.call(0x007a_d9c0, &args![option]).bool());
        assert!(!e.call(0x007a_d9c0, &args![0u32]).bool());
    }

    #[test]
    fn flags_are_set_and_cleared_by_bit() {
        let mut e = engine();
        let this = e.new_object::<RaceSexMenu>();
        e.call(0x007a_dc60, &args![this, 9u32, true]);
        assert_eq!(e.mem.u32(this.addr() + 0xec), 0x200);
        assert!(e.call(0x007a_dc30, &args![this, 9u32]).bool());
        assert!(!e.call(0x007a_dc30, &args![this, 8u32]).bool());
        // The shift count is taken modulo 32.
        assert!(e.call(0x007a_dc30, &args![this, 41u32]).bool());
        e.call(0x007a_dc60, &args![this, 3u32, true]);
        e.call(0x007a_dc60, &args![this, 9u32, false]);
        assert_eq!(e.mem.u32(this.addr() + 0xec), 8);
    }

    #[test]
    fn player_byte_setter_stores_the_byte() {
        let mut e = engine();
        let block = e.mem.alloc(0x800);
        e.call(0x007a_dcc0, &args![Ptr::<()>::new(block), 1u8]);
        assert_eq!(e.mem.u8(block + 0x75d), 1);
        e.call(0x007a_dcc0, &args![Ptr::<()>::new(block), 0x100u32]);
        assert_eq!(e.mem.u8(block + 0x75d), 0);
    }

    #[test]
    fn close_without_a_menu_does_nothing() {
        let mut e = engine();
        e.call(0x007a_da40, &args![]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn close_after_a_popup_focuses_the_current_list() {
        let (mut e, menu) = world();
        e.register(GET_MESSAGE_MENU_RESULT, |_, _| word(1));
        e.register(GAMEPAD_IN_USE, |_, _| word(1));
        e.register(CURRENT_LIST_INDEX, |_, _| word(3));
        let list = list_of(&e, menu, 3);
        e.call(0x007a_da40, &args![]);
        assert_eq!(calls(&e, OPTION_LIST_FOCUS), vec![vec![list, 1]]);
        // Not focused and the arrow is not visible: the list refreshes.
        assert_eq!(calls(&e, OPTION_LIST_REFRESH), vec![vec![list]]);
        assert_eq!(e.global::<u32>(RACE_SEX_MENU_ME), menu.addr());
        // With a visible arrow tile the focus goes to the tile instead.
        e.register(TILE_IS_VISIBLE, |_, _| word(1));
        e.call_log = Some(vec![]);
        e.call(0x007a_da40, &args![]);
        let tile = e.mem.u32(menu.addr() + 0x8c);
        let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
        assert_eq!(
            calls(&e, SET_CURRENT_FOCUS_TARGET),
            vec![vec![interface, tile, 0xfc3, 1]]
        );
        assert!(calls(&e, OPTION_LIST_REFRESH).is_empty());
    }

    #[test]
    fn close_leaves_and_puts_the_clothes_back() {
        let (mut e, menu) = world();
        let player: u32 = e.global(PLAYER);
        e.register(DIALOGUE_ACTIVE, |_, _| word(1));
        e.register(GET_TOPIC, |_, a| {
            assert_eq!(a, [5, 4]);
            word(0x7007)
        });
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(SAVED_WORN_ITEMS + 4 * 2, first);
        e.mem.set_u32(SAVED_WORN_ITEMS + 4 * 4, second);
        e.mem.set_u8(SAVED_PLAYER_FLAG, 7);
        e.mem.set_u32(first + 8, 0xa1);
        e.mem.set_u32(second + 8, 0xa2);
        e.call(0x007a_da40, &args![]);
        assert_eq!(e.global::<u32>(RACE_SEX_MENU_ME), 0);
        assert_eq!(calls(&e, MENU_INSTANT_FADE_OUT), vec![vec![menu.addr()]]);
        assert_eq!(calls(&e, UPDATE_PLAYER_NAME).len(), 1);
        assert_eq!(calls(&e, DIALOG_SERVICE_END), vec![vec![0x7007]]);
        assert_eq!(
            calls(&e, EQUIP_OBJECT),
            vec![
                vec![player, 0xa1, 1, 0, 1, 0, 1],
                vec![player, 0xa2, 1, 0, 1, 0, 1]
            ]
        );
        // Both records are deleted and forgotten.
        assert_eq!(
            calls(&e, WORN_ITEM_DELETE),
            vec![vec![first, 1], vec![second, 1]]
        );
        assert_eq!(e.mem.u32(SAVED_WORN_ITEMS + 8), 0);
        // The player's flag is restored and the byte set.
        assert_eq!(calls(&e, PLAYER_FLAG_SETTER), vec![vec![player, 7]]);
        assert_eq!(e.mem.u8(player + 0x75d), 1);
    }

    /// Makes `OPTION_AT_TILE` answer `option` for every tile.
    fn option_for_every_tile(e: &mut Engine, option: u32) {
        e.register_double(OPTION_AT_TILE, move |_, _| word(option));
    }

    #[test]
    fn click_back_goes_to_the_back_target() {
        let (mut e, menu) = world();
        e.register(CURRENT_LIST_INDEX, |_, _| word(6));
        e.register(LIST_BACK_TARGET, |_, _| word(5));
        e.call(0x007a_dce0, &args![menu, 4i32, 0u32]);
        let tile = e.mem.u32(menu.addr() + 4);
        assert!(calls(&e, SET_TRAIT_FROM_INT).contains(&vec![tile, 0x1004, 5]));
    }

    #[test]
    fn click_ok_on_the_last_list_asks_for_confirmation() {
        let (mut e, menu) = world();
        e.register(CURRENT_LIST_INDEX, |_, _| word(6));
        e.register(LIST_PARENT_TARGET, |_, _| word(2));
        e.mem.set_u32(menu.addr() + 0x98, 0x7777);
        e.mem.set_u32(menu.addr() + 0x120, 6);
        e.call(0x007a_dce0, &args![menu, 5i32, 0u32]);
        assert_eq!(
            calls(&e, SHOW_MESSAGE_BOX),
            vec![vec![
                0x7777,
                0,
                0,
                CLOSE_CALLBACK,
                1,
                0x12,
                0,
                0,
                SETTING_POPUP_B,
                SETTING_POPUP_A,
                0
            ]]
        );
        // On another list it goes to the list `00 44edb0` names.
        e.mem.set_u32(menu.addr() + 0x120, 7);
        e.call_log = Some(vec![]);
        e.call(0x007a_dce0, &args![menu, 5i32, 0u32]);
        let tile = e.mem.u32(menu.addr() + 4);
        assert!(calls(&e, SHOW_MESSAGE_BOX).is_empty());
        assert!(calls(&e, SET_TRAIT_FROM_INT).contains(&vec![tile, 0x1004, 2]));
    }

    #[test]
    fn click_on_randomize_asks_unless_flag_seven_is_set() {
        let (mut e, menu) = world();
        let option = make_option(&mut e, 0x55, 0x19, 0);
        option_for_every_tile(&mut e, option);
        e.call(0x007a_dc60, &args![menu, 7u32, true]);
        e.call(0x007a_dce0, &args![menu, 1i32, 0x55u32]);
        assert_eq!(calls(&e, RANDOMIZE), vec![vec![menu.addr()]]);
        assert!(calls(&e, SHOW_MESSAGE_BOX).is_empty());
        e.call(0x007a_dc60, &args![menu, 7u32, false]);
        e.call(0x007a_dce0, &args![menu, 1i32, 0x55u32]);
        assert_eq!(
            calls(&e, SHOW_MESSAGE_BOX),
            vec![vec![
                SETTING_RANDOMIZE_TEXT,
                0,
                0,
                RANDOMIZE,
                0,
                0x12,
                0,
                0,
                SETTING_POPUP_B,
                SETTING_POPUP_A,
                0
            ]]
        );
    }

    #[test]
    fn click_on_a_hair_option_sets_the_hair() {
        let (mut e, menu) = world();
        e.set_global(SELECTED_VALUE, 2.0f64);
        let option = make_option(&mut e, 0x55, 0x1b, 0x6600);
        option_for_every_tile(&mut e, option);
        let base = e.mem.u32(menu.addr() + 0xd8);
        e.call(0x007a_dc60, &args![menu, 7u32, true]);
        e.call(0x007a_dce0, &args![menu, 1i32, 0x55u32]);
        assert_eq!(calls(&e, SET_HAIR), vec![vec![base, 0x6600]]);
        let list = list_of(&e, menu, 0);
        assert_eq!(
            calls(&e, OPTION_LIST_UPDATE_CURRENT_SELECTION),
            vec![vec![list]]
        );
        assert!(!e.call(0x007a_dc30, &args![menu, 7u32]).bool());
        // An option that is already selected changes nothing.
        e.set_global(SELECTED_VALUE, 0.0f64);
        e.call_log = Some(vec![]);
        e.call(0x007a_dce0, &args![menu, 1i32, 0x55u32]);
        assert!(calls(&e, SET_HAIR).is_empty());
    }

    #[test]
    fn click_on_a_list_option_opens_that_list() {
        let (mut e, menu) = world();
        let option = make_option(&mut e, 0x55, 5, 0);
        option_for_every_tile(&mut e, option);
        e.call(0x007a_dce0, &args![menu, 1i32, 0x55u32]);
        let tile = e.mem.u32(menu.addr() + 4);
        assert!(calls(&e, SET_TRAIT_FROM_INT).contains(&vec![tile, 0x1004, 5]));
        // Ids of 0x14 and more do nothing.
        let other = option_big(&mut e);
        option_for_every_tile(&mut e, other);
        e.call_log = Some(vec![]);
        e.call(0x007a_dce0, &args![menu, 1i32, 0x55u32]);
        assert!(calls(&e, SET_TRAIT_FROM_INT).is_empty());
    }

    /// An option with id 0x20 (no case of the click switch).
    fn option_big(e: &mut Engine) -> u32 {
        make_option(e, 0x55, 0x20, 0)
    }

    #[test]
    fn click_on_the_sex_options_changes_the_sex() {
        let (mut e, menu) = world();
        e.set_global(SELECTED_VALUE, 2.0f64);
        let player: u32 = e.global(PLAYER);
        let base = e.mem.u32(menu.addr() + 0xd8);
        let option = make_option(&mut e, 0x55, 0x16, 0);
        option_for_every_tile(&mut e, option);
        e.call(0x007a_dce0, &args![menu, 1i32, 0x55u32]);
        assert_eq!(calls(&e, SET_FLAG_BIT), vec![vec![base + 0x30, 1, 1, 1]]);
        assert_eq!(calls(&e, CHOOSE_DEFAULT_PRESET), vec![vec![menu.addr()]]);
        assert_eq!(calls(&e, PLAYER_REFRESH_3D), vec![vec![player, 1]]);
        assert_eq!(calls(&e, KILL_EGT_DATA).len(), 1);
        assert!(e.call(0x007a_dc30, &args![menu, 0u32]).bool());
        assert!(e.call(0x007a_dc30, &args![menu, 1u32]).bool());
        // The male option clears the flag bit.
        let male = make_option(&mut e, 0x55, 0x15, 0);
        option_for_every_tile(&mut e, male);
        e.call_log = Some(vec![]);
        e.call(0x007a_dce0, &args![menu, 1i32, 0x55u32]);
        assert_eq!(calls(&e, SET_FLAG_BIT), vec![vec![base + 0x30, 1, 0, 1]]);
    }

    #[test]
    fn click_on_the_race_option_changes_the_race() {
        let (mut e, menu) = world();
        e.set_global(SELECTED_VALUE, 2.0f64);
        let player: u32 = e.global(PLAYER);
        let base = e.mem.u32(menu.addr() + 0xd8);
        let option = make_option(&mut e, 0x55, 0x17, 0x3300);
        option_for_every_tile(&mut e, option);
        e.call(0x007a_dce0, &args![menu, 1i32, 0x55u32]);
        assert_eq!(calls(&e, PLAYER_RACE_CHANGED), vec![vec![player]]);
        assert_eq!(calls(&e, PLAYER_RECALCULATE), vec![vec![player + 0x94, 0]]);
        assert_eq!(calls(&e, SET_RACE), vec![vec![base, 0x3300, 0]]);
        assert!(calls(&e, SET_FLAG_BIT).is_empty());
        assert_eq!(calls(&e, CHOOSE_DEFAULT_PRESET).len(), 1);
    }

    #[test]
    fn click_on_the_arrows_scrolls_the_list() {
        let (mut e, menu) = world();
        e.register(TILE_IS_TRUE, |_, _| word(1));
        let tile = tile_of(&e, menu, 2);
        let list = list_of(&e, menu, 0);
        e.call(0x007a_dce0, &args![menu, 2i32, tile]);
        e.call(0x007a_dce0, &args![menu, 3i32, tile]);
        assert_eq!(
            calls(&e, OPTION_LIST_SCROLL_BY),
            vec![vec![list, 1], vec![list, u32::MAX]]
        );
        e.register(TILE_IS_TRUE, |_, _| word(0));
        e.call_log = Some(vec![]);
        e.call(0x007a_dce0, &args![menu, 2i32, tile]);
        assert!(calls(&e, OPTION_LIST_SCROLL_BY).is_empty());
    }

    #[test]
    fn click_on_a_slider_starts_the_drag() {
        let (mut e, menu) = world();
        let slider = make_option(&mut e, 0x55, 0x22, 0);
        let tile = tile_of(&e, menu, 2);
        let mut answered = 0;
        e.register_double(OPTION_AT_TILE, move |_, _| {
            answered += 1;
            word(if answered == 1 { 0 } else { slider })
        });
        e.call(0x007a_dce0, &args![menu, 7i32, tile]);
        assert_eq!(e.mem.u32(menu.addr() + 0xc0), tile);
        assert_eq!(e.mem.u32(menu.addr() + 0xc4), slider);
        assert_eq!(calls(&e, SLIDER_PRESS), vec![vec![menu.addr()]]);
    }

    #[test]
    fn selected_options_compare_the_trait_with_the_constant() {
        let mut e = engine();
        e.set_global(SELECTED_VALUE, 2.0f64);
        let selected = make_option(&mut e, 0x55, 0, 0);
        e.register(GET_FLOAT_TRAIT, |_, _| float(2.0));
        e.register(ADD_USER_TRAIT, |_, _| word(0x777));
        assert!(e
            .call(0x007a_e160, &args![Ptr::<MenuOption>::new(selected)])
            .bool());
        assert_eq!(
            calls(&e, ADD_USER_TRAIT),
            vec![vec![SELECTED_TRAIT_NAME, u32::MAX]]
        );
        assert_eq!(calls(&e, GET_FLOAT_TRAIT), vec![vec![0x55, 0x777]]);
        e.register(GET_FLOAT_TRAIT, |_, _| float(1.0));
        assert!(!e
            .call(0x007a_e160, &args![Ptr::<MenuOption>::new(selected)])
            .bool());
        let tileless = make_option(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x007a_e160, &args![Ptr::<MenuOption>::new(tileless)])
            .bool());
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn down_click_records_the_slider_and_the_pointer() {
        let (mut e, menu) = world();
        let slider = make_option(&mut e, 0x55, 0x22, 0);
        option_for_every_tile(&mut e, slider);
        let tile = tile_of(&e, menu, 1);
        e.register(GET_FLOAT_TRAIT, |_, a| {
            float(if a[1] == 0x1005 { 7.0 } else { 0.0 })
        });
        e.register(POINTER_X, |_, _| float(12.5));
        e.call(0x007a_e1b0, &args![menu, 0x69i32, tile]);
        assert_eq!(e.mem.u32(menu.addr() + 0xc0), tile);
        assert_eq!(e.mem.u32(menu.addr() + 0xc4), slider);
        assert_eq!(e.mem.i32(menu.addr() + 0xbc), 7);
        assert_eq!(e.mem.f32(menu.addr() + 0xb8), 12.5);
        // Button 1 only stores the pointer; other buttons do nothing.
        e.mem.set_f32(menu.addr() + 0xb8, 0.0);
        e.call(0x007a_e1b0, &args![menu, 1i32, 0u32]);
        assert_eq!(e.mem.f32(menu.addr() + 0xb8), 12.5);
        e.mem.set_f32(menu.addr() + 0xb8, 0.0);
        e.call(0x007a_e1b0, &args![menu, 5i32, 0u32]);
        assert_eq!(e.mem.f32(menu.addr() + 0xb8), 0.0);
    }

    #[test]
    fn still_down_without_movement_does_nothing() {
        let (mut e, menu) = world();
        e.register(POINTER_X, |_, _| float(5.0));
        e.mem.set_f32(menu.addr() + 0xb8, 5.0);
        e.call(0x007a_e240, &args![menu, 0x69i32, 0x55u32]);
        assert!(calls(&e, OPTION_AT_TILE).is_empty());
        assert!(calls(&e, SLIDER_RELEASE).is_empty());
    }

    #[test]
    fn still_down_drags_the_slider() {
        let (mut e, menu) = world();
        let slider = make_option(&mut e, 0x66, 0x22, 0);
        option_for_every_tile(&mut e, slider);
        let tile = tile_of(&e, menu, 1);
        e.mem.set_u32(tile + 0x28, 0x1234);
        e.register(POINTER_X, |_, _| float(15.0));
        e.mem.set_f32(menu.addr() + 0xb8, 5.0);
        e.mem.set_i32(menu.addr() + 0xbc, 2);
        e.register(TILE_GET_CHILD_BY_ID, |_, a| {
            assert_eq!(a, [0x1234, 0x65]);
            word(0x4321)
        });
        e.register(GET_FLOAT_TRAIT, |_, a| {
            float(match (a[0], a[1]) {
                (0x4321, 0xfb1) => 50.0,
                (_, 0x1006) => 0.0,
                (_, 0x1007) => 10.0,
                (_, 0x1005) => 2.0,
                _ => 0.0,
            })
        });
        e.register(ROUND_TO_STEP, |_, a| float(f32::from_bits(a[0]) as f64));
        let value_block = e.mem.alloc(0x10);
        e.register_double(TILE_GET_VALUE, move |_, _| word(value_block));
        e.call(0x007a_e240, &args![menu, 0x69i32, tile]);
        // 2 + (10 - 0) * (10 / 50) = 4.
        assert_eq!(calls(&e, SET_TRAIT_FROM_INT), vec![vec![0x66, 0x1005, 4]]);
        assert_eq!(
            calls(&e, TILE_VALUE_SET_FLOAT),
            vec![vec![value_block, 4.0f32.to_bits(), 1]]
        );
        assert_eq!(e.mem.u32(menu.addr() + 0xc4), slider);
        assert_eq!(e.mem.u32(menu.addr() + 0xc0), tile);
        assert_eq!(calls(&e, SLIDER_RELEASE), vec![vec![menu.addr()]]);
    }

    #[test]
    fn still_down_rotates_the_player() {
        let (mut e, menu) = world();
        e.set_global(TWO_PI_DOUBLE, std::f64::consts::TAU);
        e.register(POINTER_X, |_, _| float(15.0));
        e.mem.set_f32(menu.addr() + 0xb8, 5.0);
        let node = e.mem.alloc(0x20);
        e.mem.set_u32(menu.addr() + 0xd4, node);
        e.register(GET_FLOAT_TRAIT, |_, _| float(100.0));
        e.call(0x007a_e240, &args![menu, 1i32, 0u32]);
        // The width is read once and kept in the function-local static.
        assert_eq!(e.global::<f32>(ROTATE_WIDTH), 100.0);
        assert_eq!(e.global::<u32>(ROTATE_WIDTH_GUARD) & 1, 1);
        let rotations = calls(&e, MATRIX_SET_ROTATION);
        assert_eq!(rotations.len(), 1);
        // The rotation is 0 - (10 / 100) * 2 pi, wrapped up by 2 pi.
        let wrapped = f32::from_bits(rotations[0][1]);
        let angle = (0.1f64 * std::f64::consts::TAU) as f32;
        let expected = ((0.0 - angle as f64) as f32 as f64 + std::f64::consts::TAU) as f32;
        assert!((wrapped - expected).abs() < 1e-4);
        assert!(e.call(0x007a_dc30, &args![menu, 5u32]).bool());
        assert_eq!(e.mem.f32(menu.addr() + 0xb8), 15.0);
    }

    /// The values `DoIdle` reads from the exe's data: the zoom rate (1e-5),
    /// the stick factors and the time of the last call.
    fn idle_world(now: u32, last: u32) -> (Engine, Ptr<RaceSexMenu>) {
        let (mut e, menu) = world();
        e.set_global(ZOOM_RATE_DOUBLE, 1.0e-5f64);
        e.set_global(ROTATE_RATE_DOUBLE, 2.5f64);
        e.set_global(MILLISECONDS_DOUBLE, 1000.0f64);
        e.set_global(AXIS_LIMIT_POSITIVE, 32767i32);
        e.set_global(AXIS_LIMIT_NEGATIVE, -32768i32);
        e.set_global(WHEEL_ZOOM_STEP, -0.125f32);
        e.set_global(WHEEL_ZOOM_STEP_BACK, 0.125f32);
        e.set_global(TWO_PI_DOUBLE, std::f64::consts::TAU);
        e.set_global(LAST_TIME_GUARD, 1u32);
        e.set_global(LAST_TIME, last);
        e.register_double(GET_TIME, move |_, _| word(now));
        e.register(GAMEPAD_IN_USE, |_, _| word(0));
        e.call_log = Some(vec![]);
        (e, menu)
    }

    #[test]
    fn idle_pauses_the_playing_sounds_when_not_on_top() {
        let (mut e, menu) = idle_world(1000, 900);
        e.register(IS_TOP_MENU_ID, |_, _| word(0));
        e.mem.set_u32(menu.addr() + 0xf0 + 8, 1);
        e.mem.set_u32(menu.addr() + 0x108 + 8, 1);
        e.mem.set_u32(menu.addr() + 0xfc + 8, 3);
        e.call(0x007a_e420, &args![menu]);
        assert_eq!(
            calls(&e, SOUND_HANDLE_PAUSE),
            vec![vec![menu.addr() + 0xf0], vec![menu.addr() + 0x108]]
        );
        assert!(calls(&e, SOUND_HANDLE_PLAY).is_empty());
        // The time of the last call is not updated.
        assert_eq!(e.global::<u32>(LAST_TIME), 900);
    }

    #[test]
    fn idle_resumes_paused_sounds_and_remembers_the_time() {
        let (mut e, menu) = idle_world(1000, 900);
        e.mem.set_u32(menu.addr() + 0xfc + 8, 3);
        e.call(0x007a_e420, &args![menu]);
        assert_eq!(calls(&e, SOUND_HANDLE_PLAY)[0], vec![menu.addr() + 0xfc, 0]);
        assert_eq!(e.global::<u32>(LAST_TIME), 1000);
        // With nothing to do the zoom velocity stays 0.
        assert_eq!(e.mem.f32(menu.addr() + 0xcc), 0.0);
    }

    #[test]
    fn idle_initializes_the_static_time_once() {
        let (mut e, menu) = idle_world(1000, 900);
        e.set_global(LAST_TIME_GUARD, 0u32);
        e.call(0x007a_e420, &args![menu]);
        assert_eq!(e.global::<u32>(LAST_TIME_GUARD) & 1, 1);
        assert_eq!(calls(&e, GET_TIME).len(), 2);
        assert_eq!(e.global::<u32>(LAST_TIME), 1000);
    }

    #[test]
    fn idle_eases_the_zoom_towards_the_target() {
        let (mut e, menu) = idle_world(1000, 900);
        e.mem.set_f32(menu.addr() + 0xc8, 1.0);
        e.mem.set_f32(menu.addr() + 0xd0, 0.0);
        e.call(0x007a_e420, &args![menu]);
        // 100 ms: velocity 100 * 1e-5, step 100 * velocity.
        let velocity = e.mem.f32(menu.addr() + 0xcc);
        assert!((velocity - 0.001).abs() < 1e-6);
        let zoom = e.mem.f32(menu.addr() + 0xd0);
        assert!((zoom - 0.1).abs() < 1e-5);
        assert!(e.call(0x007a_dc30, &args![menu, 6u32]).bool());
        assert!(calls(&e, SOUND_HANDLE_PLAY).contains(&vec![menu.addr() + 0x108, 0]));
    }

    #[test]
    fn idle_brakes_when_the_zoom_is_above_the_target() {
        let (mut e, menu) = idle_world(1000, 900);
        e.mem.set_f32(menu.addr() + 0xc8, 0.0);
        e.mem.set_f32(menu.addr() + 0xd0, 0.5);
        e.call(0x007a_e420, &args![menu]);
        // The velocity goes negative and the zoom moves down by 100 * velocity.
        let velocity = e.mem.f32(menu.addr() + 0xcc);
        assert!((velocity + 0.001).abs() < 1e-6);
        let zoom = e.mem.f32(menu.addr() + 0xd0);
        assert!((zoom - 0.4).abs() < 1e-5);
    }

    #[test]
    fn idle_moves_the_target_with_the_wheel() {
        let (mut e, menu) = idle_world(1000, 900);
        let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
        let tile = tile_of(&e, menu, 1);
        e.mem.set_u32(interface + 0xcc, tile);
        e.register(WHEEL_DELTA, |_, _| word(1));
        e.mem.set_f32(menu.addr() + 0xc8, 0.5);
        e.mem.set_f32(menu.addr() + 0xd0, 0.5);
        e.call(0x007a_e420, &args![menu]);
        assert_eq!(e.mem.f32(menu.addr() + 0xc8), 0.375);
        // A wheel over another tile sends the click buttons 2 and 3.
        e.mem.set_u32(interface + 0xcc, 0xabcd);
        e.call_log = Some(vec![]);
        e.call(0x007a_e420, &args![menu]);
        assert_eq!(calls(&e, MENU_SLOTS + 0xc), vec![vec![menu.addr(), 2, 0]]);
        e.register(WHEEL_DELTA, |_, _| word(-1i32 as u32));
        e.call_log = Some(vec![]);
        e.call(0x007a_e420, &args![menu]);
        assert_eq!(calls(&e, MENU_SLOTS + 0xc), vec![vec![menu.addr(), 3, 0]]);
    }

    #[test]
    fn idle_turns_the_player_with_the_stick() {
        let (mut e, menu) = idle_world(1000, 900);
        e.register(GAMEPAD_IN_USE, |_, _| word(1));
        e.register(AXIS_VALUE, |_, a| word(if a[2] == 9 { 20000 } else { 0 }));
        e.register(ABSOLUTE_VALUE, |_, a| word((a[0] as i32).unsigned_abs()));
        let node = e.mem.alloc(0x20);
        e.mem.set_u32(menu.addr() + 0xd4, node);
        e.mem.set_f32(menu.addr() + 0xcc, 0.25);
        e.call(0x007a_e420, &args![menu]);
        assert_eq!(calls(&e, MATRIX_SET_ROTATION).len(), 1);
        assert!(e.call(0x007a_dc30, &args![menu, 5u32]).bool());
        // Turning does not keep the zoom velocity.
        assert_eq!(e.mem.f32(menu.addr() + 0xcc), 0.0);
        assert!(calls(&e, SOUND_HANDLE_PLAY).contains(&vec![menu.addr() + 0x114, 0]));
    }

    #[test]
    fn idle_zooms_with_the_stick() {
        let (mut e, menu) = idle_world(1000, 900);
        e.register(GAMEPAD_IN_USE, |_, _| word(1));
        e.register(AXIS_VALUE, |_, a| word(if a[2] == 10 { 20000 } else { 0 }));
        e.register(ABSOLUTE_VALUE, |_, a| word((a[0] as i32).unsigned_abs()));
        e.register(AXIS_REDUCE, |_, a| word(a[0]));
        e.mem.set_f32(menu.addr() + 0xd0, 0.5);
        e.call(0x007a_e420, &args![menu]);
        // The velocity grew and the zoom moved (the stick is positive: down).
        assert!(e.mem.f32(menu.addr() + 0xcc) > 0.0);
        assert!(e.call(0x007a_dc30, &args![menu, 6u32]).bool());
    }

    #[test]
    fn idle_releases_a_slider_when_the_stick_button_is_up() {
        let (mut e, menu) = idle_world(1000, 900);
        e.register(GAMEPAD_IN_USE, |_, _| word(1));
        e.register(AXIS_VALUE, |_, _| word(0));
        e.mem.set_u32(menu.addr() + 0xc4, 0x1234);
        e.mem.set_u32(menu.addr() + 0xc0, 0x5678);
        e.call(0x007a_e420, &args![menu]);
        assert!(calls(&e, SLIDER_RELEASE).is_empty());
        e.call(0x007a_dc60, &args![menu, 4u32, true]);
        // `DoIdle` clears flags 5 and 6 first, so flag 4 stays what it was.
        e.call(0x007a_e420, &args![menu]);
        assert_eq!(calls(&e, SLIDER_RELEASE), vec![vec![menu.addr()]]);
        assert_eq!(e.mem.u32(menu.addr() + 0xc4), 0);
        assert_eq!(e.mem.u32(menu.addr() + 0xc0), 0);
    }

    #[test]
    fn gamepad_ignores_buttons_when_not_on_top() {
        let (mut e, menu) = world();
        e.register(IS_TOP_MENU_ID, |_, _| word(0));
        assert!(!e.call(0x007a_ecb0, &args![menu, 1i32, 0u32]).bool());
        assert!(calls(&e, OPTION_LIST_DO_GAMEPAD).is_empty());
    }

    #[test]
    fn gamepad_sets_flag_four_from_the_stick_and_asks_the_list() {
        let (mut e, menu) = world();
        e.register(OPTION_LIST_DO_GAMEPAD, |_, _| word(1));
        e.register(CURRENT_LIST_INDEX, |_, _| word(2));
        e.register(AXIS_VALUE, |_, a| {
            assert_eq!(a[1..], [0, 7]);
            word(100)
        });
        e.register(ABSOLUTE_VALUE, |_, a| word(a[0]));
        let list = list_of(&e, menu, 2);
        assert!(e.call(0x007a_ecb0, &args![menu, 1i32, 0u32]).bool());
        assert_eq!(calls(&e, OPTION_LIST_DO_GAMEPAD), vec![vec![list, 1]]);
        assert!(e.call(0x007a_dc30, &args![menu, 4u32]).bool());
        // Buttons 3 and 4 never set it.
        e.register(OPTION_LIST_DO_GAMEPAD, |_, _| word(0));
        e.call(0x007a_ecb0, &args![menu, 4i32, 0u32]);
        assert!(!e.call(0x007a_dc30, &args![menu, 4u32]).bool());
        // A big stick value clears it.
        e.register(ABSOLUTE_VALUE, |_, _| word(0x1ea9));
        e.call(0x007a_ecb0, &args![menu, 1i32, 0u32]);
        assert!(!e.call(0x007a_dc30, &args![menu, 4u32]).bool());
        // A negative button skips the stick altogether.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x007a_ecb0, &args![menu, -1i32, 0u32]).bool());
        assert!(calls(&e, AXIS_VALUE).is_empty());
    }

    #[test]
    fn gamepad_ok_enters_the_next_list_with_a_sound() {
        let (mut e, menu) = world();
        e.register(CURRENT_LIST_INDEX, |_, _| word(1));
        e.register(LIST_BACK_TARGET, |_, _| word(6));
        assert!(e.call(0x007a_ecb0, &args![menu, 10i32, 0u32]).bool());
        let tile = e.mem.u32(menu.addr() + 4);
        assert!(calls(&e, SET_TRAIT_FROM_INT).contains(&vec![tile, 0x1004, 6]));
        let sounds = calls(&e, GET_SOUND_HANDLE_BY_NAME_ON_AUDIO);
        assert_eq!(sounds.len(), 1);
        assert_eq!(sounds[0][2..], [SOUND_MENU_OK, 0x121]);
        assert_eq!(calls(&e, SOUND_HANDLE_PLAY).len(), 1);
        // No sound when the list is not accepted (0x14 and more).
        e.register(LIST_BACK_TARGET, |_, _| word(0x14));
        e.call_log = Some(vec![]);
        assert!(e.call(0x007a_ecb0, &args![menu, 10i32, 0u32]).bool());
        assert!(calls(&e, SOUND_HANDLE_PLAY).is_empty());
    }

    #[test]
    fn gamepad_back_walks_up_the_lists() {
        let (mut e, menu) = world();
        e.register(CURRENT_LIST_INDEX, |_, _| word(5));
        let list5 = list_of(&e, menu, 5);
        e.register_double(LIST_BACK_TARGET, move |_, a| {
            word(if a[0] == list5 { 2 } else { 3 })
        });
        assert!(e.call(0x007a_ecb0, &args![menu, 13i32, 0u32]).bool());
        let tile = e.mem.u32(menu.addr() + 4);
        assert!(calls(&e, SET_TRAIT_FROM_INT).contains(&vec![tile, 0x1004, 3]));
        let sounds = calls(&e, GET_SOUND_HANDLE_BY_NAME_ON_AUDIO);
        assert_eq!(sounds[0][2..], [SOUND_PIPBOY_MODE, 0x121]);
        // Button 14 asks to close the menu when the list is not accepted.
        e.register(LIST_PARENT_TARGET, |_, _| word(0x14));
        e.mem.set_u32(menu.addr() + 0x98, 0x7777);
        e.call_log = Some(vec![]);
        assert!(e.call(0x007a_ecb0, &args![menu, 14i32, 0u32]).bool());
        assert_eq!(calls(&e, SHOW_MESSAGE_BOX)[0][0], 0x7777);
        assert_eq!(calls(&e, SHOW_MESSAGE_BOX)[0][3], CLOSE_CALLBACK);
    }

    #[test]
    fn gamepad_moves_the_focus_between_the_arrows() {
        let (mut e, menu) = world();
        let left = e.mem.u32(menu.addr() + 0x88);
        let right = e.mem.u32(menu.addr() + 0x8c);
        e.register_double(FOCUS_TILE, move |_, _| word(left));
        e.register(TILE_IS_VISIBLE, |_, _| word(1));
        assert!(e.call(0x007a_ecb0, &args![menu, 3i32, 0u32]).bool());
        assert_eq!(calls(&e, SET_FOCUS_TARGET_CDECL), vec![vec![right]]);
        // Button 4 on the right arrow goes back to the left one.
        e.register_double(FOCUS_TILE, move |_, _| word(right));
        e.call_log = Some(vec![]);
        assert!(e.call(0x007a_ecb0, &args![menu, 4i32, 0u32]).bool());
        assert_eq!(calls(&e, SET_FOCUS_TARGET_CDECL), vec![vec![left]]);
    }

    #[test]
    fn gamepad_clicks_the_child_of_the_focused_tile() {
        let (mut e, menu) = world();
        let tile = tile_of(&e, menu, 1);
        e.register_double(FOCUS_TILE, move |_, _| word(tile));
        e.register(TILE_GET_CHILD_BY_ID, |_, a| word(a[1] + 0x1000));
        assert!(e.call(0x007a_ecb0, &args![menu, 0xfi32, 0u32]).bool());
        assert_eq!(
            calls(&e, MENU_SLOTS + 0xc),
            vec![vec![menu.addr(), 0x66, 0x1066]]
        );
        // Buttons 0x10, 3 and 4 use the ids 0x67, 0x68 and 0x64.
        e.call_log = Some(vec![]);
        e.call(0x007a_ecb0, &args![menu, 0x10i32, 0u32]);
        assert_eq!(
            calls(&e, MENU_SLOTS + 0xc),
            vec![vec![menu.addr(), 0x67, 0x1067]]
        );
    }

    #[test]
    fn gamepad_unused_buttons_are_not_handled() {
        let (mut e, menu) = world();
        assert!(!e.call(0x007a_ecb0, &args![menu, 6i32, 0u32]).bool());
        assert!(!e.call(0x007a_ecb0, &args![menu, 99i32, 0u32]).bool());
        assert!(calls(&e, OPTION_LIST_DO_GAMEPAD).is_empty());
    }

    #[test]
    fn gamepad_two_keeps_the_focus_on_an_arrow() {
        let (mut e, menu) = world();
        let left = e.mem.u32(menu.addr() + 0x88);
        e.register_double(FOCUS_TILE, move |_, _| word(left));
        assert!(e.call(0x007a_ecb0, &args![menu, 2i32, 0u32]).bool());
        assert!(calls(&e, OPTION_LIST_DO_GAMEPAD).is_empty());
        // Elsewhere the list decides, and then a visible arrow takes the focus.
        e.register(FOCUS_TILE, |_, _| word(0x9999));
        e.register(TILE_IS_VISIBLE, |_, _| word(1));
        e.call_log = Some(vec![]);
        assert!(e.call(0x007a_ecb0, &args![menu, 2i32, 0u32]).bool());
        assert_eq!(calls(&e, OPTION_LIST_DO_GAMEPAD).len(), 1);
        assert_eq!(calls(&e, SET_FOCUS_TARGET_CDECL), vec![vec![left]]);
        assert_eq!(calls(&e, OPTION_LIST_FOCUS).len(), 1);
    }

    #[test]
    fn active_list_shows_the_list_and_the_arrows() {
        let (mut e, menu) = world();
        e.mem.set_u32(CAPTION_B, 0xbbbb);
        e.mem.set_u32(CAPTION_A, 0xaaaa);
        e.register(LIST_BACK_TARGET, |_, _| word(3));
        e.register(LIST_PARENT_TARGET, |_, _| word(0x14));
        let tile = e.mem.u32(menu.addr() + 4);
        let left = e.mem.u32(menu.addr() + 0x88);
        let right = e.mem.u32(menu.addr() + 0x8c);
        let rendered = e.mem.alloc(0x20);
        e.register_double(RENDERED_MENU_GETTER, move |_, _| word(rendered));
        assert_eq!(e.call(0x007a_f180, &args![menu, 2i32]).u32(), 1);
        let list = list_of(&e, menu, 2);
        assert_eq!(calls(&e, OPTION_LIST_REFRESH), vec![vec![list]]);
        assert_eq!(calls(&e, LIGHT_BUTTON), vec![vec![rendered, 2]]);
        let traits = calls(&e, SET_TRAIT_FROM_INT);
        assert!(traits.contains(&vec![tile, 0x1004, 2]));
        assert!(traits.contains(&vec![left, TRAIT_VISIBLE, 1]));
        assert!(traits.contains(&vec![right, TRAIT_VISIBLE, 0]));
        assert_eq!(
            calls(&e, SET_STRING_TRAIT).last().unwrap(),
            &vec![right, TRAIT_STRING, 0xbbbb, 1]
        );
        // The last list shows the other caption and its right arrow.
        e.mem.set_u32(menu.addr() + 0x120, 2);
        e.call_log = Some(vec![]);
        e.call(0x007a_f180, &args![menu, 2i32]);
        let traits = calls(&e, SET_TRAIT_FROM_INT);
        assert!(traits.contains(&vec![right, TRAIT_VISIBLE, 1]));
        assert_eq!(
            calls(&e, SET_STRING_TRAIT).last().unwrap(),
            &vec![right, TRAIT_STRING, 0xaaaa, 1]
        );
        // Lists of 0x14 and more are refused.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007a_f180, &args![menu, 0x14i32]).u32(), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn active_list_refills_the_hair_and_eye_lists_when_flagged() {
        let (mut e, menu) = world();
        e.call(0x007a_dc60, &args![menu, 0u32, true]);
        e.call(0x007a_f180, &args![menu, 5i32]);
        assert_eq!(
            calls(&e, OPTION_LIST_CLEAR),
            vec![vec![list_of(&e, menu, 5)]]
        );
        assert!(!e.call(0x007a_dc30, &args![menu, 0u32]).bool());
        e.call(0x007a_dc60, &args![menu, 1u32, true]);
        e.call_log = Some(vec![]);
        e.call(0x007a_f180, &args![menu, 8i32]);
        assert_eq!(
            calls(&e, OPTION_LIST_CLEAR),
            vec![vec![list_of(&e, menu, 8)]]
        );
        assert!(!e.call(0x007a_dc30, &args![menu, 1u32]).bool());
        // The hair list is validated for list 3.
        e.call_log = Some(vec![]);
        e.call(0x007a_f180, &args![menu, 3i32]);
        assert_eq!(calls(&e, GET_SEX).len(), 1);
    }

    /// The hair list (list 5 at `+0x3c`) is filled with the nodes of the
    /// owner's list whose items pass the checks.
    #[test]
    fn hair_list_is_filled_from_the_owner_list() {
        let (mut e, menu) = world();
        walk_chains(&mut e);
        let owner = e.mem.alloc(0x20);
        e.set_global(MARKER_OWNER, owner);
        let (first, second, third) = (0x1001u32, 0x1002, 0x1003);
        let head = chain(&mut e, &[first, second, third]);
        e.register_double(HAIR_LIST_HEAD, move |_, a| {
            assert_eq!(a, [owner]);
            word(head)
        });
        e.register(HAIR_CHECK_A, |_, a| word((a[0] != 0x1002) as u32));
        e.register(HAIR_CHECK_B, |_, a| word((a[0] != 0x1003) as u32));
        e.register(MARKER_NAME, |_, a| word(a[0] + 0x100));
        let option_data = 0x4242;
        e.register_double(OPTION_LIST_ADD_OPTION, move |e, _| {
            let option = e.mem.alloc(0xc);
            e.mem.set_u32(option + 8, option_data);
            word(option)
        });
        e.register(BASE_VALUE_603B50, |_, _| word(0x4242));
        e.call(0x007a_dc60, &args![menu, 0u32, true]);
        e.call(0x007a_f300, &args![menu]);
        let list = list_of(&e, menu, 5);
        assert_eq!(calls(&e, OPTION_LIST_CLEAR), vec![vec![list]]);
        // Only the first node passes both checks.
        let added = calls(&e, OPTION_LIST_ADD_OPTION);
        assert_eq!(added, vec![vec![list, 0x1001 + 0x18 + 0x100, 0x1b]]);
        assert_eq!(calls(&e, OPTION_SET_DATA).len(), 1);
        assert_eq!(calls(&e, OPTION_SET_DATA)[0][1], 0x1001);
        // The accepted option is made the current one: the list is stepped.
        assert_eq!(calls(&e, OPTION_LIST_DO_GAMEPAD), vec![vec![list, 2]]);
        assert_eq!(
            calls(&e, OPTION_LIST_UPDATE_CURRENT_SELECTION),
            vec![vec![list]]
        );
        assert!(!e.call(0x007a_dc30, &args![menu, 0u32]).bool());
    }

    #[test]
    fn eye_list_is_filled_from_the_owner_list() {
        let (mut e, menu) = world();
        walk_chains(&mut e);
        let owner = e.mem.alloc(0x20);
        e.set_global(MARKER_OWNER, owner);
        let head = chain(&mut e, &[0x2001, 0x2002]);
        e.register_double(EYE_LIST_HEAD, move |_, a| {
            assert_eq!(a, [owner]);
            word(head)
        });
        e.register(EYE_CHECK_A, |_, _| word(1));
        e.register(EYE_CHECK_B, |_, a| word((a[0] == 0x2002) as u32));
        e.call(0x007a_dc60, &args![menu, 1u32, true]);
        e.call(0x007a_f450, &args![menu]);
        let list = list_of(&e, menu, 8);
        assert_eq!(calls(&e, OPTION_LIST_CLEAR), vec![vec![list]]);
        let added = calls(&e, OPTION_LIST_ADD_OPTION);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][2], 0x21);
        assert_eq!(calls(&e, OPTION_SET_DATA)[0][1], 0x2002);
        assert!(!e.call(0x007a_dc30, &args![menu, 1u32]).bool());
    }

    #[test]
    fn hair_option_follows_the_sex() {
        let (mut e, menu) = world();
        let target = e.mem.u32(menu.addr() + 0x34);
        e.register(GET_SEX, |_, _| word(0));
        e.register(OPTION_LIST_FIND, |_, _| word(0));
        let frame = next_frame(&mut e, 0x20);
        e.call(0x007a_f520, &args![menu]);
        // The 0x20-byte frame: the caption at +8.
        assert_eq!(calls(&e, STRING_CONSTRUCT)[0][0], frame + 8);
        assert_eq!(
            calls(&e, STRING_FORMAT),
            vec![vec![
                calls(&e, STRING_CONSTRUCT)[0][0],
                CAPTION_FORMAT,
                SETTING_23A0,
                0x3e
            ]]
        );
        assert_eq!(
            calls(&e, OPTION_LIST_INSERT_OPTION),
            vec![vec![target, 2, 0, LIST_ITEM_TEMPLATE, 7]]
        );
        assert!(calls(&e, OPTION_LIST_REMOVE_OPTION).is_empty());
        // A woman with the entry loses it.
        e.register(GET_SEX, |_, _| word(1));
        e.register(OPTION_LIST_FIND, |_, _| word(0x9999));
        e.call_log = Some(vec![]);
        e.call(0x007a_f520, &args![menu]);
        assert_eq!(calls(&e, OPTION_LIST_REMOVE_OPTION), vec![vec![target, 7]]);
        assert!(calls(&e, OPTION_LIST_INSERT_OPTION).is_empty());
        // A man with the entry (or a woman without) is left alone.
        e.register(GET_SEX, |_, _| word(0));
        e.call_log = Some(vec![]);
        e.call(0x007a_f520, &args![menu]);
        assert!(calls(&e, OPTION_LIST_REMOVE_OPTION).is_empty());
        assert!(calls(&e, OPTION_LIST_INSERT_OPTION).is_empty());
    }

    #[test]
    fn preset_head_list_enables_the_option_by_the_array() {
        let (mut e, menu) = world();
        let option = make_option(&mut e, 0x55, 0x18, 0);
        e.register_double(OPTION_LIST_FIND, move |_, a| {
            assert_eq!(a[1], 0x18);
            word(option)
        });
        e.register(GET_SEX, |_, _| word(1));
        e.register(ADD_USER_TRAIT, |_, _| word(0x777));
        e.register(SIMPLE_ARRAY_IS_EMPTY, |_, _| word(1));
        let race = e.call(BASE_VALUE_4AC110, &args![0u32]).u32();
        e.mem.set_u32(menu.addr() + 0xe4, 6);
        e.call(0x007a_f610, &args![menu]);
        assert_eq!(
            calls(&e, SIMPLE_ARRAY_CLEAR),
            vec![vec![menu.addr() + 0xdc, 0]]
        );
        assert_eq!(
            calls(&e, FILL_PRESET_HEADS),
            vec![vec![race, 1, menu.addr() + 0xdc]]
        );
        // Empty array: the option is disabled.
        assert_eq!(calls(&e, SET_TRAIT_FROM_INT), vec![vec![0x55, 0x777, 0]]);
        // Otherwise it is enabled with the range 1 to the count.
        e.register(SIMPLE_ARRAY_IS_EMPTY, |_, _| word(0));
        e.call_log = Some(vec![]);
        e.call(0x007a_f610, &args![menu]);
        let traits = calls(&e, SET_TRAIT_FROM_INT);
        assert_eq!(traits[0], vec![0x55, 0x777, 1]);
        assert_eq!(traits[1], vec![0x55, 0x1006, 1]);
        assert_eq!(traits[2], vec![0x55, 0x1007, 6]);
    }

    #[test]
    fn scroll_arrows_follow_the_hidden_options() {
        let (mut e, menu) = world();
        walk_chains(&mut e);
        e.register(ADD_USER_TRAIT, |_, _| word(0x777));
        e.register(GET_FLOAT_TRAIT, |_, a| {
            float(match a[0] {
                0x11 => -1.0,
                0x13 => 1.0,
                _ => 0.0,
            })
        });
        let options: Vec<u32> = [0x11u32, 0x12, 0x13]
            .iter()
            .map(|tile| make_option(&mut e, *tile, 0, 0))
            .collect();
        let head = chain(&mut e, &options);
        let list = list_of(&e, menu, 0);
        e.mem.set_u32(list, head);
        let up = e.mem.u32(menu.addr() + 0x80);
        let down = e.mem.u32(menu.addr() + 0x84);
        e.call(0x007a_f6b0, &args![menu]);
        let traits = calls(&e, SET_TRAIT_FROM_INT);
        assert_eq!(
            traits,
            vec![vec![up, TRAIT_VISIBLE, 1], vec![down, TRAIT_VISIBLE, 1]]
        );
        assert_eq!(calls(&e, TILE_UPDATE_ALL), vec![vec![0]]);
        // With only a middle option neither arrow shows.
        let head = chain(&mut e, &options[1..2]);
        e.mem.set_u32(list, head);
        e.call_log = Some(vec![]);
        e.call(0x007a_f6b0, &args![menu]);
        let traits = calls(&e, SET_TRAIT_FROM_INT);
        assert_eq!(
            traits,
            vec![vec![up, TRAIT_VISIBLE, 0], vec![down, TRAIT_VISIBLE, 0]]
        );
    }

    #[test]
    fn face_slider_change_applies_the_value_to_the_face() {
        let (mut e, menu) = world();
        e.set_global(TENTH_DIVISOR, 10.0f64);
        let slider = make_option(&mut e, 0x55, 0x22, 9);
        e.register(GET_FLOAT_TRAIT, |_, _| float(25.0));
        let frame = next_frame(&mut e, 0x9c);
        e.call(0x007a_f770, &args![menu, slider]);
        // The 0x9c-byte frame: the coordinates at +0x10.
        assert_eq!(calls(&e, GET_FACE_COORD)[0][1], frame + 0x10);
        let base = e.mem.u32(menu.addr() + 0xd8);
        let coords = calls(&e, GET_FACE_COORD)[0][1];
        assert_eq!(
            calls(&e, GET_FACE_COORD)[1],
            vec![base, menu.addr() + 0x124]
        );
        assert_eq!(
            calls(&e, APPLY_FACE_SLIDER),
            vec![vec![coords, 0, 0, 9, 2.5f32.to_bits()]]
        );
        assert_eq!(calls(&e, REGENERATE_FACE).len(), 1);
        assert_eq!(calls(&e, REGENERATE_FACE)[0][1], coords);
        assert!(e.call(0x007a_dc30, &args![menu, 8u32]).bool());
        assert!(!e.call(0x007a_dc30, &args![menu, 9u32]).bool());
        // Id 0x23 also sets flag 9 and applies with a non-zero flag.
        let other = make_option(&mut e, 0x55, 0x23, 9);
        e.call_log = Some(vec![]);
        e.call(0x007a_f770, &args![menu, other]);
        assert!(e.call(0x007a_dc30, &args![menu, 9u32]).bool());
        assert_eq!(calls(&e, APPLY_FACE_SLIDER)[0][1], 1);
    }

    #[test]
    fn hair_color_slider_change_looks_up_the_settings() {
        let (mut e, menu) = world();
        let colour_option = make_option(&mut e, 0x55, 0x1d, 0);
        e.register(GET_FLOAT_TRAIT, |_, _| float(3.0));
        e.register(SETTINGS_COLLECTION, |_, _| word(0x500));
        let mut lookups = 0;
        let color_word = e.mem.alloc(8);
        e.mem.set_u32(color_word, 0x0033_6699);
        e.register_double(SETTINGS_FIND, move |e, a| {
            lookups += 1;
            e.mem
                .set_u32(a[2], if lookups == 1 { 0x600 } else { 0x700 });
            word(1)
        });
        e.register_double(SETTING_INT_POINTER, move |_, a| {
            assert_eq!(a, [0x600]);
            word(color_word)
        });
        let frame = next_frame(&mut e, 0x28);
        e.call(0x007a_f900, &args![menu, colour_option]);
        // The 0x28-byte frame: the text at +8, the found settings at +0x14 and +0x10.
        assert_eq!(calls(&e, STRING_CONSTRUCT)[0][0], frame + 8);
        let found: Vec<u32> = calls(&e, SETTINGS_FIND).iter().map(|w| w[2]).collect();
        assert_eq!(found, vec![frame + 0x14, frame + 0x10]);
        let formats = calls(&e, STRING_FORMAT);
        assert_eq!(formats[0][1..], [FORMAT_HAIR_COLOR_INT, 3]);
        assert_eq!(formats[1][1..], [FORMAT_HAIR_COLOR_NAME, 3]);
        assert_eq!(calls(&e, SETTINGS_FIND).len(), 2);
        // The caption of the colour option is the second setting's text.
        assert_eq!(calls(&e, SET_STRING_TRAIT)[0], vec![0x55, 0x1009, 0x700, 1]);
        // `007b3660` ran at the end (it reads the three sliders).
        assert!(!calls(&e, STRING_C_STR).is_empty());
        // No option: nothing.
        e.call_log = Some(vec![]);
        e.call(0x007a_f900, &args![menu, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn hair_color_slider_change_for_another_option_sets_the_caption() {
        let (mut e, menu) = world();
        let other = make_option(&mut e, 0x55, 0x1f, 0);
        let colour_option = make_option(&mut e, 0x66, 0x1d, 0);
        e.register_double(OPTION_LIST_FIND, move |_, a| {
            assert_eq!(a[1], 0x1d);
            word(colour_option)
        });
        e.call(0x007a_f900, &args![menu, other]);
        assert_eq!(
            calls(&e, SET_STRING_TRAIT)[0],
            vec![0x66, 0x1009, SETTING_3528, 1]
        );
        assert!(calls(&e, SETTINGS_FIND).is_empty());
    }

    #[test]
    fn hair_color_sliders_take_the_color_channels() {
        let (mut e, menu) = world();
        walk_chains(&mut e);
        e.register(GET_FLOAT_TRAIT, |_, a| {
            float(if a[1] == 0x1007 { 255.0 } else { 0.0 })
        });
        let options: Vec<u32> = [(0x21u32, 0x1eu32), (0x22, 0x1f), (0x23, 0x20), (0x24, 0x30)]
            .iter()
            .map(|(tile, id)| make_option(&mut e, *tile, *id, 0))
            .collect();
        let head = chain(&mut e, &options);
        let list = list_of(&e, menu, 6);
        e.mem.set_u32(list, head);
        e.call(0x007a_fa40, &args![menu, 0x0033_6699u32]);
        let traits: Vec<Vec<u32>> = calls(&e, SET_TRAIT_FROM_INT)
            .into_iter()
            .filter(|c| c[1] == 0x1005)
            .collect();
        assert_eq!(
            traits,
            vec![
                vec![0x21, 0x1005, 0x99],
                vec![0x22, 0x1005, 0x66],
                vec![0x23, 0x1005, 0x33]
            ]
        );
    }

    #[test]
    fn face_sliders_are_added_from_the_table() {
        let (mut e, menu) = world();
        e.set_global(TENTH_DIVISOR, 10.0f64);
        e.set_global(SLIDER_STEP_QUARTER, 0.25f32);
        e.register(SHAPE_COUNT, |_, _| word(2));
        e.register(FACE_SHAPE_VALUE, |_, _| float(5.0));
        e.register(FLOAT_SETTING_GETTER, |_, a| {
            float(if a[0] == 0x011d_a6a4 { -2.0 } else { 2.0 })
        });
        let frame = next_frame(&mut e, 0x670);
        e.call(0x007a_faf0, &args![menu]);
        // The 0x670-byte frame: the coordinate array at +0x3e4.
        assert_eq!(
            calls(&e, EH_VECTOR_CONSTRUCTOR_ITERATOR)[0][0],
            frame + 0x3e4
        );
        let sliders = calls(&e, OPTION_LIST_ADD_SLIDER);
        // Two shapes from the table and twelve extras.
        assert_eq!(sliders.len(), 14);
        // Shape 0: list 0xc, caption setting 011d1ecc, value 5 limited to 2.
        assert_eq!(
            sliders[0],
            vec![
                list_of(&e, menu, 0xc),
                0x011d_1ecc,
                0x22,
                (-20i32) as u32,
                20,
                20,
                0.25f32.to_bits()
            ]
        );
        assert_eq!(sliders[1][0], list_of(&e, menu, 0xc));
        assert_eq!(sliders[1][1], 0x011d_3504);
        // The first extra slider is the shape group 1, number 0x1d (list 0x13).
        assert_eq!(sliders[2][0], list_of(&e, menu, 0x13));
        assert_eq!(sliders[2][1..3], [0x011d_33e4, 0x23]);
        let shapes = calls(&e, FACE_SHAPE_VALUE);
        assert_eq!(shapes.len(), 14);
        assert_eq!(shapes[0][1..], [0, 0, 0]);
        let selectors: Vec<u32> = shapes[2..].iter().map(|c| c[3]).collect();
        assert_eq!(
            selectors,
            vec![0x1d, 0x1c, 0x1e, 0x1f, 8, 9, 0x12, 0x1b, 0x16, 5, 6, 3]
        );
        assert!(shapes[2..].iter().all(|c| c[1] == 1));
        // The data of the options: the shape number.
        let data = calls(&e, OPTION_SET_DATA);
        assert_eq!(data[0][1], 0);
        assert_eq!(data[2][1], 0x1d);
        assert_eq!(calls(&e, SHAPE_COUNT).len(), 3);
        assert_eq!(calls(&e, FACE_FILL_COORDS).len(), 1);
    }

    #[test]
    fn face_sliders_skip_entries_without_a_list_or_caption() {
        let (mut e, menu) = world();
        e.set_global(TENTH_DIVISOR, 10.0f64);
        // Shape 5 of the table has no list (0x14).
        e.register(SHAPE_COUNT, |_, _| word(6));
        e.call(0x007a_faf0, &args![menu]);
        // Entries 0 to 4 have a list; entry 5 does not: 5 + 12 sliders.
        assert_eq!(calls(&e, OPTION_LIST_ADD_SLIDER).len(), 17);
    }

    #[test]
    fn face_sliders_finish_with_the_pending_flags() {
        let (mut e, menu) = world();
        e.call(0x007a_dc60, &args![menu, 8u32, true]);
        e.call(0x007a_dc60, &args![menu, 9u32, true]);
        e.call(0x007a_faf0, &args![menu]);
        assert!(!e.call(0x007a_dc30, &args![menu, 8u32]).bool());
        assert!(!e.call(0x007a_dc30, &args![menu, 9u32]).bool());
        // Flag 8 runs `007b1e70(1, 1, 0)`, flag 9 `007b25a0`.
        assert_eq!(calls(&e, CLEAR_BODY_TEXTURES).len(), 1);
        assert!(calls(&e, CLEAR_HEAD).is_empty());
    }

    /// An option of the lists 10 to 18 with the shape number `shape`.
    fn shape_option(e: &mut Engine, menu: Ptr<RaceSexMenu>, list: u32, id: u32) -> u32 {
        walk_chains(e);
        let option = make_option(e, 0x55, id, 4);
        let head = chain(e, &[option]);
        let list = list_of(e, menu, list);
        e.mem.set_u32(list, head);
        e.register(GET_FLOAT_TRAIT, |_, a| {
            float(match a[1] {
                0x1006 => 0.0,
                0x1007 => 20.0,
                _ => 0.0,
            })
        });
        e.set_global(TENTH_DIVISOR, 10.0f64);
        option
    }

    #[test]
    fn face_slider_refresh_pulls_options_back_into_range() {
        let (mut e, menu) = world();
        let option = shape_option(&mut e, menu, 10, 0x22);
        e.register(FACE_SHAPE_VALUE, |_, _| float(3.0));
        // 3.0 * 10 = 30 is above the range 0 to 20: the option gets 20.
        let frame = next_frame(&mut e, 0xb0);
        assert!(e.call(0x007b_16d0, &args![menu, 10i32, 11i32, 0u32]).bool());
        // The 0xb0-byte frame: the cursor at +0x98, the coordinates at +0x14.
        assert_eq!(calls(&e, LIST_ADVANCE)[0][1], frame + 0x98);
        assert_eq!(
            calls(&e, EH_VECTOR_CONSTRUCTOR_ITERATOR)[0][0],
            frame + 0x14
        );
        assert_eq!(calls(&e, SET_TRAIT_FROM_INT), vec![vec![0x55, 0x1005, 20]]);
        assert_eq!(calls(&e, WORD_AT_EIGHT)[0], vec![option]);
        // In range and not the dragged option: set to the value.
        e.register(FACE_SHAPE_VALUE, |_, _| float(1.5));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x007b_16d0, &args![menu, 10i32, 11i32, 0u32]).bool());
        assert_eq!(calls(&e, SET_TRAIT_FROM_INT), vec![vec![0x55, 0x1005, 15]]);
        // The dragged option is left alone.
        e.mem.set_u32(menu.addr() + 0xc4, option);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x007b_16d0, &args![menu, 10i32, 11i32, 0u32]).bool());
        assert!(calls(&e, SET_TRAIT_FROM_INT).is_empty());
        // Other option ids are ignored.
        let other = make_option(&mut e, 0x66, 0x30, 4);
        let head = chain(&mut e, &[other]);
        let list = list_of(&e, menu, 11);
        e.mem.set_u32(list, head);
        e.call_log = Some(vec![]);
        e.call(0x007b_16d0, &args![menu, 11i32, 12i32, 0u32]);
        assert!(calls(&e, FACE_SHAPE_VALUE).is_empty());
    }

    #[test]
    fn face_slider_refresh_repeats_up_to_four_times() {
        let (mut e, menu) = world();
        shape_option(&mut e, menu, 10, 0x22);
        e.register(FACE_SHAPE_VALUE, |_, _| float(3.0));
        e.call(0x007b_18f0, &args![menu, 0x22u32]);
        // The value stays out of range, so all four rounds run (selector 0).
        let rounds = calls(&e, FACE_SHAPE_VALUE);
        assert_eq!(rounds.len(), 4);
        assert!(rounds.iter().all(|c| c[1] == 0));
        // 0x23 works on list 19 with selector 1; others do nothing.
        e.call_log = Some(vec![]);
        e.call(0x007b_18f0, &args![menu, 0x24u32]);
        assert!(calls(&e, FACE_SHAPE_VALUE).is_empty());
        shape_option(&mut e, menu, 19, 0x23);
        e.call(0x007b_18f0, &args![menu, 0x23u32]);
        let rounds = calls(&e, FACE_SHAPE_VALUE);
        assert_eq!(rounds.len(), 4);
        assert!(rounds.iter().all(|c| c[1] == 1));
        // No change: a single round.
        e.register(FACE_SHAPE_VALUE, |_, _| float(1.0));
        e.register(GET_FLOAT_TRAIT, |_, a| {
            float(match a[1] {
                0x1007 => 20.0,
                0x1005 => 10.0,
                _ => 0.0,
            })
        });
        e.call_log = Some(vec![]);
        e.call(0x007b_18f0, &args![menu, 0x23u32]);
        assert_eq!(calls(&e, FACE_SHAPE_VALUE).len(), 1);
    }

    /// The Euler angles `TO_EULER_ANGLES` reports: z is `z`.
    fn euler_z(e: &mut Engine, z: f32) {
        e.register_double(TO_EULER_ANGLES, move |e, a| {
            e.mem.set_f32(a[3], z);
            Ret::default()
        });
    }

    #[test]
    fn rotate_player_turns_the_node_and_wraps_the_angle() {
        let (mut e, menu) = world();
        e.set_global(TWO_PI_DOUBLE, std::f64::consts::TAU);
        let node = e.mem.alloc(0x20);
        e.mem.set_u32(menu.addr() + 0xd4, node);
        e.register(PLAYER_BASE_WEIGHT, |_, _| float(1.5));
        e.register(PLAYER_BASE_HEIGHT, |_, _| float(2.5));
        euler_z(&mut e, 1.0);
        let frame = next_frame(&mut e, 0x88);
        assert_eq!(e.call(0x007b_1970, &args![menu, 0.25f32, 0u32]).u8(), 1);
        // The 0x88-byte frame: the matrix at +0x64 and the angles at +0x54, +0x5c, +0x58.
        assert_eq!(
            calls(&e, TO_EULER_ANGLES)[0],
            vec![frame + 0x64, frame + 0x54, frame + 0x5c, frame + 0x58]
        );
        let rotation = calls(&e, MATRIX_SET_ROTATION);
        assert_eq!(f32::from_bits(rotation[0][1]), 0.75);
        let elements = calls(&e, MATRIX_SET_ELEMENT);
        assert_eq!(elements.len(), 3);
        assert_eq!(elements[0][1..], [0, 0, 1.5f32.to_bits()]);
        assert_eq!(elements[1][1..], [1, 1, 1.5f32.to_bits()]);
        assert_eq!(elements[2][1..], [2, 2, 2.5f32.to_bits()]);
        assert_eq!(calls(&e, SET_ROTATE).len(), 1);
        assert_eq!(calls(&e, SET_ROTATE)[0][0], node);
        // Below zero the angle is brought up by a full turn.
        e.call_log = Some(vec![]);
        euler_z(&mut e, 0.5);
        e.call(0x007b_1970, &args![menu, 1.0f32, 0u32]);
        let wrapped = f32::from_bits(calls(&e, MATRIX_SET_ROTATION)[0][1]);
        let expected = ((0.5f32 - 1.0f32) as f64 + std::f64::consts::TAU) as f32;
        assert_eq!(wrapped, expected);
        // Above a full turn it is brought down.
        e.call_log = Some(vec![]);
        euler_z(&mut e, 7.0);
        e.call(0x007b_1970, &args![menu, 0.0f32, 1u32]);
        let wrapped = f32::from_bits(calls(&e, MATRIX_SET_ROTATION)[0][1]);
        assert_eq!(wrapped, (7.0f64 - std::f64::consts::TAU) as f32);
    }

    #[test]
    fn rotate_player_returns_zero_when_nothing_changes() {
        let (mut e, menu) = world();
        e.set_global(TWO_PI_DOUBLE, std::f64::consts::TAU);
        let node = e.mem.alloc(0x20);
        e.mem.set_u32(menu.addr() + 0xd4, node);
        euler_z(&mut e, 1.0);
        assert_eq!(e.call(0x007b_1970, &args![menu, 0.0f32, 0u32]).u8(), 0);
        assert!(calls(&e, MATRIX_SET_ROTATION).is_empty());
        // The force flag rotates anyway.
        assert_eq!(e.call(0x007b_1970, &args![menu, 0.0f32, 1u32]).u8(), 1);
        assert_eq!(calls(&e, MATRIX_SET_ROTATION).len(), 1);
    }

    #[test]
    fn lighting_update_moves_the_scene_and_scales_the_rendered_menu() {
        let (mut e, menu) = world();
        e.register(PLAYER_NODE_GETTER, |_, _| word(0xbb));
        e.register(SCENE_NODE_GETTER, |_, _| word(0xaa));
        let frame = next_frame(&mut e, 0x10);
        e.call(0x007b_1ae0, &args![menu]);
        // The 0x10-byte frame: the point at +4.
        assert_eq!(calls(&e, POINT_CONSTRUCT)[0][0], frame + 4);
        assert_eq!(calls(&e, UPDATE_OBJECT_LIGHTING), vec![vec![0xaa, 0xbb, 0]]);
        let point = calls(&e, POINT_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, SET_TRANSLATE_POINT), vec![vec![0xaa, point]]);
        assert!(calls(&e, RENDERED_MENU_SET_SCALE).is_empty());
        e.register(RENDERED_MENU_GETTER, |_, _| word(0xcc));
        e.call_log = Some(vec![]);
        e.call(0x007b_1ae0, &args![menu]);
        assert_eq!(
            calls(&e, RENDERED_MENU_SET_SCALE),
            vec![vec![0xcc, 1.0f32.to_bits()]]
        );
    }

    #[test]
    fn zoom_player_places_the_node_between_the_bounds() {
        let (mut e, menu) = world();
        let node = e.mem.alloc(0x20);
        e.mem.set_u32(menu.addr() + 0xd4, node);
        for (offset, value) in [
            (0xa0u32, 0.0f32),
            (0xa4, 0.0),
            (0xa8, 1.0),
            (0xac, 10.0),
            (0xb0, 20.0),
            (0xb4, 3.0),
        ] {
            e.mem.set_f32(menu.addr() + offset, value);
        }
        e.register(ACTOR_GET_HEIGHT, |_, _| float(2.0));
        assert_eq!(e.call(0x007b_1b60, &args![menu, 0.5f32, 0u32]).u8(), 1);
        assert_eq!(e.mem.f32(menu.addr() + 0xd0), 0.5);
        assert_eq!(
            calls(&e, SET_TRANSLATE),
            vec![vec![
                node,
                5.0f32.to_bits(),
                10.0f32.to_bits(),
                4.0f32.to_bits()
            ]]
        );
        // The zoom stops at 1.
        e.call(0x007b_1b60, &args![menu, 5.0f32, 0u32]);
        assert_eq!(e.mem.f32(menu.addr() + 0xd0), 1.0);
        // Unchanged: 0 unless forced.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007b_1b60, &args![menu, 1.0f32, 0u32]).u8(), 0);
        assert!(calls(&e, SET_TRANSLATE).is_empty());
        assert_eq!(e.call(0x007b_1b60, &args![menu, 1.0f32, 1u32]).u8(), 1);
        assert_eq!(calls(&e, SET_TRANSLATE).len(), 1);
    }

    #[test]
    fn default_hair_and_eyes_replace_unfit_ones() {
        let (mut e, menu) = world();
        let base = e.mem.u32(menu.addr() + 0xd8);
        let race = e.call(BASE_VALUE_4AC110, &args![0u32]).u32();
        e.mem.set_u32(race + 0xa8, 0x7123);
        // Everything fits: only the flags are set.
        e.register(HAIR_CHECK_B, |_, _| word(1));
        e.register(EYE_CHECK_B, |_, _| word(1));
        e.call_log = Some(vec![]);
        e.call(0x007b_1ca0, &args![menu]);
        assert!(calls(&e, SET_HAIR).is_empty());
        assert!(calls(&e, SET_HEAD).is_empty());
        assert!(e.call(0x007a_dc30, &args![menu, 0u32]).bool());
        assert!(e.call(0x007a_dc30, &args![menu, 1u32]).bool());
        // The hair does not fit: the race's default hair is used.
        e.register(HAIR_CHECK_B, |_, _| word(0));
        e.register(RACE_DEFAULT_HAIR, |_, _| word(0x9100));
        e.call_log = Some(vec![]);
        e.call(0x007b_1ca0, &args![menu]);
        assert_eq!(calls(&e, SET_HAIR), vec![vec![base, 0x9100]]);
        assert!(calls(&e, RACE_HAIR_LIST_HEAD).len() == 1);
        // The eyes do not fit: the value `007b1e50` finds is set.
        e.register(EYE_CHECK_B, |_, _| word(0));
        e.call_log = Some(vec![]);
        e.call(0x007b_1ca0, &args![menu]);
        assert_eq!(calls(&e, SET_HEAD), vec![vec![base, 0x7123]]);
    }

    #[test]
    fn default_hair_is_searched_in_the_race_list() {
        let (mut e, menu) = world();
        let base = e.mem.u32(menu.addr() + 0xd8);
        e.register(HAIR_CHECK_B, |_, _| word(0));
        e.register(EYE_CHECK_B, |_, _| word(1));
        e.register(RACE_DEFAULT_HAIR, |_, _| word(0));
        let head = chain(&mut e, &[0x3001, 0x3002, 0x3003]);
        e.register_double(RACE_HAIR_LIST_HEAD, move |_, _| word(head));
        e.register(HAIR_CHECK_A, |_, _| word(1));
        e.register(HAIR_CHECK_MALE, |_, a| word((a[0] == 0x3002) as u32));
        e.register(HAIR_CHECK_FEMALE, |_, a| word((a[0] == 0x3003) as u32));
        e.register(GET_SEX, |_, _| word(0));
        e.call(0x007b_1ca0, &args![menu]);
        assert_eq!(calls(&e, SET_HAIR), vec![vec![base, 0x3002]]);
        e.register(GET_SEX, |_, _| word(1));
        e.call_log = Some(vec![]);
        e.call(0x007b_1ca0, &args![menu]);
        assert_eq!(calls(&e, SET_HAIR), vec![vec![base, 0x3003]]);
    }

    #[test]
    fn word_at_a8_is_read_through_the_pointer_getter() {
        let mut e = engine();
        let block = e.mem.alloc(0x100);
        e.mem.set_u32(block + 0xa8, 0x5151);
        assert_eq!(e.call(0x007b_1e50, &args![block]).u32(), 0x5151);
        assert_eq!(
            calls(&e, EMPTY_MEMBER_RETURNS_THIS),
            vec![vec![block + 0xa8]]
        );
    }

    #[test]
    fn byte_setter_and_check_work_on_their_fields() {
        let mut e = engine();
        let block = e.mem.alloc(0x100);
        e.call(0x007b_2550, &args![block, 1u8]);
        assert_eq!(e.mem.u8(block + 0xe4), 1);
        let inner = e.mem.alloc(0x10);
        assert_eq!(e.call(0x007b_2570, &args![block]).u8(), 0);
        e.mem.set_u32(block + 0xc, inner);
        assert_eq!(e.call(0x007b_2570, &args![block]).u8(), 0);
        e.mem.set_u32(inner + 8, 5);
        assert_eq!(e.call(0x007b_2570, &args![block]).u8(), 1);
    }

    /// An object with the vtable slots the 3D walkers need: slot `+0x1c`
    /// answers `object`, slot `+0x94` answers `head`, slot `+0x9c` answers
    /// `found`.
    fn walker(e: &mut Engine, vtable: u32, slots: u32, object: u32, head: u32, found: u32) -> u32 {
        let walker = object_with_vtable(e, 0x100, vtable, slots);
        e.register_double(slots + 0x1c, move |_, _| word(object));
        e.register_double(slots + 0x94, move |_, _| word(head));
        e.register_double(slots + 0x9c, move |_, _| word(found));
        walker
    }

    /// Makes the player's 3D roots (slots `+0x1ac` and `+0x1b0`) answer
    /// `first` and `second`.
    fn player_roots(e: &mut Engine, first: u32, second: u32) {
        e.register_double(PLAYER_SLOTS + 0x1ac, move |_, _| word(first));
        e.register_double(PLAYER_SLOTS + 0x1b0, move |_, _| word(second));
    }

    #[test]
    fn head_update_does_nothing_without_the_3d() {
        let (mut e, menu) = world();
        e.call(0x007b_1e70, &args![menu, true, true, true]);
        // Only the call and the first 3D lookup; nothing else.
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        player_roots(&mut e, 0x10, 0);
        e.call_log = Some(vec![]);
        e.call(0x007b_1e70, &args![menu, true, true, true]);
        assert!(calls(&e, CLEAR_HEAD).is_empty());
    }

    #[test]
    fn head_update_applies_the_face_to_the_children() {
        let (mut e, menu) = world();
        let player: u32 = e.global(PLAYER);
        let root = e.mem.alloc(0x100);
        let object = e.mem.alloc(0x10);
        let head = e.mem.alloc(0x10);
        let child = walker(&mut e, 0x0f50_0000, 0x0f51_0000, object, 0, 0);
        let cast = walker(&mut e, 0x0f52_0000, 0x0f53_0000, 0, head, 0);
        player_roots(&mut e, root, root);
        e.register(CHILD_COUNT, |_, _| word(1));
        e.register_double(CHILD_AT, move |_, _| word(child));
        e.register_double(CAST_607310, move |_, a| {
            assert_eq!(a, [object]);
            word(cast)
        });
        let frame = next_frame(&mut e, 0x18c);
        e.call(0x007b_1e70, &args![menu, false, true, false]);
        // The 0x18c-byte frame: the coordinates at +0x60, the applied ones at +0xe0.
        assert_eq!(calls(&e, GET_FACE_COORD)[0][1], frame + 0x60);
        assert_eq!(
            calls(&e, EH_VECTOR_CONSTRUCTOR_ITERATOR)[1][0],
            frame + 0xe0
        );
        // The coordinates of the base form are read, regenerated and applied.
        let base = e.mem.u32(menu.addr() + 0xd8);
        assert_eq!(calls(&e, GET_FACE_COORD)[0][0], base);
        let regenerate = calls(&e, REGENERATE_FACE);
        assert_eq!(regenerate.len(), 1);
        assert_eq!(regenerate[0][0], menu.addr() + 0x124);
        let apply = calls(&e, APPLY_COORDS);
        assert_eq!(apply.len(), 2);
        assert_eq!(apply[0][0], head);
        assert_eq!(apply[0][1], regenerate[0][2]);
        assert_eq!(apply[0][2..], [object, 0]);
        // The scene root gets the byte of `007b2550` for both passes.
        assert_eq!(e.mem.u8(root + 0xe4), 1);
        assert!(calls(&e, APPLY_TEXTURING).is_empty());
        assert_eq!(calls(&e, PLAYER_NODE_GETTER).len(), 1);
        let _ = player;
    }

    #[test]
    fn head_update_applies_the_texturing_to_the_face_mesh() {
        let (mut e, menu) = world();
        let root = e.mem.alloc(0x100);
        let object = e.mem.alloc(0x10);
        let head = e.mem.alloc(0x10);
        let child = walker(&mut e, 0x0f50_0000, 0x0f51_0000, object, 0, 0);
        let cast = walker(&mut e, 0x0f52_0000, 0x0f53_0000, 0, head, 0);
        let property = object_with_vtable(&mut e, 0x100, 0x0f54_0000, 0x0f55_0000);
        player_roots(&mut e, root, root);
        e.register(CHILD_COUNT, |_, _| word(1));
        e.register_double(CHILD_AT, move |_, _| word(child));
        e.register_double(CAST_607310, move |_, _| word(cast));
        e.register_double(GET_PROPERTY, move |_, _| word(property));
        e.register(PROPERTY_KIND, |_, _| word(9));
        e.register(STRLEN, |_, _| word(11));
        e.register(STRING_C_STR, |e, a| word(e.mem.u32(a[0])));
        e.register_double(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.call(0x007b_1e70, &args![menu, false, false, true]);
        // The name `FaceGenFace` is looked up with its length.
        assert_eq!(calls(&e, FIND_BY_NAME)[0][1..], [FACE_GEN_FACE, 11]);
        // The head had no data (`007b2570` false): it is refreshed.
        assert_eq!(calls(&e, HEAD_REFRESH), vec![vec![head]; 2]);
        let texturing = calls(&e, APPLY_TEXTURING);
        assert_eq!(texturing.len(), 2);
        assert_eq!(texturing[0][0], head);
        // The shader property gets the image through its slot `+0xfc`.
        assert_eq!(calls(&e, 0x0f55_0000 + 0xfc), vec![vec![property, 1, 0]; 2]);
        assert!(calls(&e, APPLY_COORDS).is_empty());
    }

    #[test]
    fn head_update_resets_the_head() {
        let (mut e, menu) = world();
        let player: u32 = e.global(PLAYER);
        let base = e.mem.u32(menu.addr() + 0xd8);
        let node = object_with_vtable(&mut e, 0x40, 0x0f56_0000, 0x0f57_0000);
        let object = e.mem.alloc(0x10);
        e.register_double(0x0f57_0000 + 0xc, move |_, _| word(object));
        e.register_double(PLAYER_NODE_GETTER, move |_, _| word(node));
        player_roots(&mut e, 0x10, 0x20);
        e.register(OBJECT_9611E0, |_, _| word(0));
        e.register_double(INIT_HEAD, |e, a| {
            e.mem.set_u32(a[1], 0xaa);
            e.mem.set_u32(a[2], 0xbb);
            Ret::default()
        });
        e.register(PLAYER_BIPED, |_, _| word(0xcc));
        let frame = next_frame(&mut e, 0x18c);
        e.call(0x007b_1e70, &args![menu, true, false, false]);
        // The 0x18c-byte frame: the head pair at +0x170 and +0x17c.
        assert_eq!(calls(&e, INIT_HEAD)[0][1..], [frame + 0x170, frame + 0x17c]);
        assert_eq!(calls(&e, CLEAR_HEAD), vec![vec![base]]);
        assert_eq!(
            calls(&e, HEAD_BUILD),
            vec![vec![base, player, 0xcc, 0xaa, 0xbb]]
        );
        // The base form was asked with 0x800 through its slot `+0x48`.
        assert_eq!(calls(&e, BASE_SLOTS + 0x48), vec![vec![base, 0x800]]);
        // The saved object is told to reset (slot `+0x58`).
        assert_eq!(calls(&e, SAVED_OBJECT).len(), 2);
    }

    #[test]
    fn head_rebuild_runs_the_queued_head_and_the_pipboy() {
        let (mut e, menu) = world();
        let player: u32 = e.global(PLAYER);
        let base = e.mem.u32(menu.addr() + 0xd8);
        e.register(PLAYER_BASE_FORM, |_, _| word(0x6100));
        e.register(PLAYER_BIPED, |_, _| word(0x6200));
        let frame = next_frame(&mut e, 0xbc);
        e.call(0x007b_25a0, &args![menu]);
        // The 0xbc-byte frame: the queued head at +0x48, the matrices at +0x8c and +0x24.
        assert_eq!(calls(&e, QUEUED_HEAD_CONSTRUCT)[0][0], frame + 0x48);
        assert_eq!(
            calls(&e, MULTIPLE_MATRIX_BY_RACE)[0][1..],
            [frame + 0x24, frame + 0x8c]
        );
        assert_eq!(calls(&e, CLEAR_BODY_TEXTURES), vec![vec![0x6100]]);
        // The player has no 3D: the head is rebuilt through the temporary.
        let queued = calls(&e, QUEUED_HEAD_CONSTRUCT);
        assert_eq!(queued.len(), 1);
        assert_eq!(queued[0][1..], [base, 0]);
        assert_eq!(
            calls(&e, QUEUED_HEAD_APPLY),
            vec![vec![queued[0][0], player, 0x6200]]
        );
        assert_eq!(calls(&e, QUEUED_HEAD_DESTRUCT), vec![vec![queued[0][0]]]);
        // The flag is up during the rebuild and down after it.
        assert_eq!(e.mem.u8(FACE_REBUILD_FLAG), 0);
        // The saved object's slots `+0x58`, `+0x468(3)`, `+0x464(player)`.
        let saved = calls(&e, SAVED_OBJECT).len();
        assert_eq!(saved, 3);
        assert_eq!(calls(&e, 0x0f4b_0000 + 0x468)[0][1], 3);
        assert_eq!(calls(&e, 0x0f4b_0000 + 0x464)[0][1], player);
        // The pipboy's slots `+0x28` and `+0x24` run last.
        assert_eq!(calls(&e, 0x0f49_0000 + 0x28).len(), 1);
        assert_eq!(calls(&e, 0x0f49_0000 + 0x24).len(), 1);
        assert_eq!(calls(&e, LOAD_QUEUED_PRIORITY).len(), 1);
        // With a 3D root the queued head is skipped.
        player_roots(&mut e, 0x10, 0);
        e.call_log = Some(vec![]);
        e.call(0x007b_25a0, &args![menu]);
        assert!(calls(&e, QUEUED_HEAD_CONSTRUCT).is_empty());
    }

    #[test]
    fn eye_update_needs_the_3d_root() {
        let (mut e, menu) = world();
        assert!(!e.call(0x007b_27c0, &args![menu]).bool());
        assert!(calls(&e, SET_STRING_TRAIT).is_empty());
        assert_eq!(calls(&e, NI_POINTER_DESTRUCT).len(), 2);
    }

    #[test]
    fn eye_update_applies_the_default_texture_to_both_eyes() {
        let (mut e, menu) = world();
        let root = e.mem.alloc(0x100);
        let object = e.mem.alloc(0x10);
        let child = walker(&mut e, 0x0f50_0000, 0x0f51_0000, object, 0, 0);
        let property = object_with_vtable(&mut e, 0x100, 0x0f54_0000, 0x0f55_0000);
        player_roots(&mut e, root, 0);
        e.register(CHILD_COUNT, |_, _| word(1));
        e.register_double(CHILD_AT, move |_, _| word(child));
        e.register(NAME_COMPARE, |_, _| word(0));
        e.register(BASE_VALUE_602150, |_, _| word(0));
        e.register_double(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register_double(CREATE_TEXTURE_IMAGE, |e, a| {
            e.mem.set_u32(a[2], 0x7777);
            Ret::default()
        });
        e.register_double(GET_PROPERTY, move |_, _| word(property));
        e.register(PROPERTY_KIND, |_, _| word(10));
        let frame = next_frame(&mut e, 0x50);
        assert!(e.call(0x007b_27c0, &args![menu]).bool());
        // The 0x50-byte frame: the eye pointers at +0x3c and +0x40, the path at +0x24.
        assert_eq!(
            calls(&e, NI_POINTER_CONSTRUCT)[0..2],
            [vec![frame + 0x3c, 0], vec![frame + 0x40, 0]]
        );
        assert_eq!(calls(&e, STRING_CONSTRUCT)[0][0], frame + 0x24);
        // No eye texture on the base form: the default path.
        assert_eq!(calls(&e, STRING_FORMAT)[0][1], DEFAULT_EYE_TEXTURE);
        assert_eq!(calls(&e, CREATE_TEXTURE_IMAGE).len(), 1);
        // Both eyes got the image through slot `+0xf0`.
        assert_eq!(
            calls(&e, 0x0f55_0000 + 0xf0),
            vec![vec![property, 0, 0, 0x7777], vec![property, 0, 0, 0x7777]]
        );
        // The eye texture of the base form is used when it has one.
        let eyes = e.mem.alloc(0x40);
        e.register_double(BASE_VALUE_602150, move |_, _| word(eyes));
        e.register(MARKER_NAME, |_, a| word(a[0] + 1));
        e.call_log = Some(vec![]);
        e.call(0x007b_27c0, &args![menu]);
        assert_eq!(
            calls(&e, STRING_FORMAT)[0][1..],
            [TEXTURE_PATH_FORMAT, eyes + 0x24 + 1]
        );
    }

    #[test]
    fn hair_recolour_needs_the_3d_root() {
        let (mut e, menu) = world();
        assert!(!e.call(0x007b_2b50, &args![menu, false]).bool());
        assert!(calls(&e, GET_PROPERTY).is_empty());
        assert_eq!(calls(&e, STRING_CONSTRUCT).len(), 5);
        assert_eq!(calls(&e, STRING_DESTROY).len(), 5);
    }

    #[test]
    fn hair_recolour_sets_the_property_colour_of_the_hair() {
        let (mut e, menu) = world();
        let root = e.mem.alloc(0x100);
        let object = e.mem.alloc(0x10);
        let child = walker(&mut e, 0x0f50_0000, 0x0f51_0000, object, 0, 0);
        let property = e.mem.alloc(0xa0);
        player_roots(&mut e, root, 0);
        e.set_global(COLOR_DIVISOR, 255.0f64);
        e.register(CHILD_COUNT, |_, _| word(1));
        e.register_double(CHILD_AT, move |_, _| word(child));
        e.register(FIND_BY_NAME, |_, a| word((a[1] != FACE_GEN_HAIR) as u32));
        e.register(STRING_LENGTH, |_, _| word(11));
        e.register_double(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(HAIR_VALUE_602130, |_, _| float(1.5));
        e.register_double(GET_PROPERTY, move |_, _| word(property));
        e.register(PROPERTY_KIND, |_, _| word(8));
        e.register(HAIR_COLOR_GETTER, |_, _| word(0x0033_6699));
        let made = e.mem.alloc(0x10);
        for (index, value) in [1u32, 2, 3, 4].iter().enumerate() {
            e.mem.set_u32(made + 4 * index as u32, *value);
        }
        e.register_double(COLOR_A_CONSTRUCT, move |_, _| word(made));
        let frame = next_frame(&mut e, 0x29c);
        assert!(e.call(0x007b_2b50, &args![menu, false]).bool());
        // The 0x29c-byte frame: the pointers at +0x24c and +0x284, the colour at +0xa4.
        assert_eq!(
            calls(&e, NI_POINTER_CONSTRUCT)[0..2],
            [vec![frame + 0x24c, 0], vec![frame + 0x284, 0]]
        );
        assert_eq!(calls(&e, COLOR_A_CONSTRUCT)[0][0], frame + 0xa4);
        // The hair value is applied to the object.
        assert_eq!(
            calls(&e, APPLY_HAIR_VALUE),
            vec![vec![object, 1.5f32.to_bits()]]
        );
        // Red, green and blue over 255, alpha 1.
        let color = calls(&e, COLOR_A_CONSTRUCT);
        assert_eq!(color.len(), 1);
        assert_eq!(color[0][1], (0x99 as f32 / 255.0).to_bits());
        assert_eq!(color[0][2], (0x66 as f32 / 255.0).to_bits());
        assert_eq!(color[0][3], (0x33 as f32 / 255.0).to_bits());
        assert_eq!(color[0][4], 1.0f32.to_bits());
        for index in 0..4u32 {
            assert_eq!(e.mem.u32(property + 0x84 + 4 * index), index + 1);
        }
        // The translation of the root is reset.
        assert_eq!(calls(&e, SET_TRANSLATE_POINT)[0][0], root);
    }

    #[test]
    fn hair_recolour_rebuild_detaches_and_rebuilds_the_head() {
        let (mut e, menu) = world();
        let root = e.mem.alloc(0x100);
        let found = e.mem.alloc(0x10);
        let walker_root = walker(&mut e, 0x0f50_0000, 0x0f51_0000, 0, 0, found);
        player_roots(&mut e, walker_root, 0x30);
        e.register(PLAYER_BASE_FORM, |_, _| word(0x6100));
        e.register(FACE_PART_NAME, |_, a| word(a[1] + 0x40));
        e.register(FIXED_STRING_FROM, |_, a| word(a[0]));
        e.register(GET_PROPERTY, |_, _| word(0));
        let _ = root;
        let frame = next_frame(&mut e, 0x29c);
        assert!(e.call(0x007b_2b50, &args![menu, true]).bool());
        // The 0x29c-byte frame: the heads at +0x220 and +0x124, the face data at +0x128.
        assert_eq!(
            calls(&e, FACE_BUILD)[0][..3],
            [frame + 0x220, frame + 0x124, frame + 0x128]
        );
        assert_eq!(calls(&e, FACE_PART_NAME)[0][0], frame + 0x224);
        let race_call = calls(&e, GET_FACE_GEN_DATA);
        assert_eq!(race_call.len(), 1);
        assert_eq!(race_call[0][1], 0x6100);
        assert_eq!(race_call[0][4], 0);
        // Both named children are detached (slot `+0xe8`) and found again.
        assert_eq!(
            calls(&e, 0x0f51_0000 + 0xe8),
            vec![vec![walker_root, found]; 2]
        );
        assert_eq!(calls(&e, 0x0f51_0000 + 0x9c).len(), 4);
        assert_eq!(calls(&e, FACE_BUILD).len(), 1);
        let base_update = calls(&e, FORM_UPDATE_6062E0);
        assert_eq!(base_update[0][0], 0x6100);
        assert_eq!(calls(&e, UPDATE_PROPERTIES), vec![vec![walker_root]]);
        assert_eq!(calls(&e, FACE_DATA_DESTRUCT).len(), 1);
        assert_eq!(calls(&e, PREPARE_OBJECT).len(), 2);
    }

    #[test]
    fn hair_colour_is_packed_from_the_three_sliders() {
        let (mut e, menu) = world();
        walk_chains(&mut e);
        e.register(GET_FLOAT_TRAIT, |_, a| {
            float(match a[0] {
                0x21 => 10.0,
                0x22 => 20.0,
                0x23 => 30.0,
                _ => 0.0,
            })
        });
        let options: Vec<u32> = [(0x21u32, 0x1eu32), (0x22, 0x1f), (0x23, 0x20)]
            .iter()
            .map(|(tile, id)| make_option(&mut e, *tile, *id, 0))
            .collect();
        let head = chain(&mut e, &options);
        let list = list_of(&e, menu, 6);
        e.mem.set_u32(list, head);
        let base = e.mem.u32(menu.addr() + 0xd8);
        e.call(0x007b_3660, &args![menu]);
        assert_eq!(e.mem.u32(base + 0x1d8), 10 | 20 << 8 | 30 << 16);
        // A missing slider counts as 0xffffffff, whose low byte is 0xff.
        let head = chain(&mut e, &options[..1]);
        e.mem.set_u32(list, head);
        e.call(0x007b_3660, &args![menu]);
        assert_eq!(e.mem.u32(base + 0x1d8), 10 | 0xff << 8 | 0xff << 16);
    }

    #[test]
    fn word_setter_stores_the_hair_colour_word() {
        let mut e = engine();
        let block = e.mem.alloc(0x300);
        e.call(0x007b_3720, &args![block, 0x00ab_cdefu32]);
        assert_eq!(e.mem.u32(block + 0x1d8), 0x00ab_cdef);
    }

    #[test]
    fn option_value_is_the_truncated_trait() {
        let mut e = engine();
        let option = make_option(&mut e, 0x55, 0, 0);
        e.register(GET_FLOAT_TRAIT, |_, a| {
            assert_eq!(a, [0x55, 0x1005]);
            float(7.9)
        });
        assert_eq!(
            e.call(0x007b_3740, &args![Ptr::<MenuOption>::new(option)])
                .i32(),
            7
        );
    }

    /// An option with the tile 0x55 whose value is 5 in the range 0 to 10.
    fn ranged_option(e: &mut Engine) -> u32 {
        let option = make_option(e, 0x55, 0, 0);
        e.register(GET_FLOAT_TRAIT, |_, a| {
            float(match a[1] {
                0x1005 => 5.0,
                0x1007 => 10.0,
                _ => 0.0,
            })
        });
        option
    }

    #[test]
    fn option_value_is_limited_to_the_range() {
        let mut e = engine();
        let option = ranged_option(&mut e);
        let value_block = e.mem.alloc(0x10);
        e.register_double(TILE_GET_VALUE, move |_, a| {
            assert_eq!(a, [0x55, 0x1009]);
            word(value_block)
        });
        let this = Ptr::<MenuOption>::new(option);
        // The same value: nothing happens.
        assert!(!e.call(0x007b_3760, &args![this, 5i32]).bool());
        assert!(calls(&e, SET_TRAIT_FROM_INT).is_empty());
        // Above the range: the maximum.
        assert!(e.call(0x007b_3760, &args![this, 99i32]).bool());
        assert_eq!(calls(&e, SET_TRAIT_FROM_INT), vec![vec![0x55, 0x1005, 10]]);
        // Below the range: the minimum; the tile value takes it as a float.
        e.call_log = Some(vec![]);
        assert!(e.call(0x007b_3760, &args![this, -4i32]).bool());
        assert_eq!(calls(&e, SET_TRAIT_FROM_INT), vec![vec![0x55, 0x1005, 0]]);
        assert_eq!(
            calls(&e, TILE_VALUE_SET_FLOAT),
            vec![vec![value_block, 0.0f32.to_bits(), 1]]
        );
        // When the value answers a non-zero word the float is not set.
        e.register(LIST_BACK_TARGET, |_, _| word(1));
        e.call_log = Some(vec![]);
        assert!(e.call(0x007b_3760, &args![this, 7i32]).bool());
        assert!(calls(&e, TILE_VALUE_SET_FLOAT).is_empty());
        // Without a tile nothing changes.
        let tileless = make_option(&mut e, 0, 0, 0);
        assert!(!e
            .call(0x007b_3760, &args![Ptr::<MenuOption>::new(tileless), 7i32])
            .bool());
    }

    #[test]
    fn option_text_needs_a_tile_and_a_text() {
        let mut e = engine();
        let option = make_option(&mut e, 0x55, 0, 0);
        let this = Ptr::<MenuOption>::new(option);
        e.call(0x007b_3810, &args![this, 0x1234u32]);
        assert_eq!(
            calls(&e, SET_STRING_TRAIT),
            vec![vec![0x55, 0x1009, 0x1234, 1]]
        );
        e.call(0x007b_3810, &args![this, 0u32]);
        let tileless = make_option(&mut e, 0, 0, 0);
        e.call(
            0x007b_3810,
            &args![Ptr::<MenuOption>::new(tileless), 0x1234u32],
        );
        assert_eq!(calls(&e, SET_STRING_TRAIT).len(), 1);
    }

    #[test]
    fn option_range_is_written_to_both_outputs() {
        let mut e = engine();
        let option = ranged_option(&mut e);
        let out = e.mem.alloc(8);
        e.call(
            0x007b_3840,
            &args![Ptr::<MenuOption>::new(option), out, out + 4],
        );
        assert_eq!(e.mem.i32(out), 0);
        assert_eq!(e.mem.i32(out + 4), 10);
        // Without a tile the outputs are left alone.
        e.mem.set_i32(out, 77);
        let tileless = make_option(&mut e, 0, 0, 0);
        e.call(
            0x007b_3840,
            &args![Ptr::<MenuOption>::new(tileless), out, out + 4],
        );
        assert_eq!(e.mem.i32(out), 77);
    }

    #[test]
    fn option_range_moves_the_value_into_the_new_range() {
        let mut e = engine();
        let option = ranged_option(&mut e);
        let this = Ptr::<MenuOption>::new(option);
        e.call(0x007b_3890, &args![this, 2i32, 4i32]);
        let traits = calls(&e, SET_TRAIT_FROM_INT);
        assert_eq!(traits[0], vec![0x55, 0x1006, 2]);
        assert_eq!(traits[1], vec![0x55, 0x1007, 4]);
        // The value 5 is above 4: `SetValue(4)` runs (its range is still 0..10).
        assert_eq!(traits[2], vec![0x55, 0x1005, 4]);
        assert_eq!(traits.len(), 3);
        // A tile-less option is left alone.
        let tileless = make_option(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(
            0x007b_3890,
            &args![Ptr::<MenuOption>::new(tileless), 2i32, 4i32],
        );
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn option_step_count_is_one_up_to_the_constant() {
        let mut e = engine();
        e.set_global(ONE_DOUBLE, 1.0f64);
        let option = make_option(&mut e, 0x55, 0, 0);
        let this = Ptr::<MenuOption>::new(option);
        for (trait_value, expected) in [
            (0.5, 1),
            (1.0, 1),
            (-3.0, 1),
            (3.9, 3),
            (100.0, 100),
            (f64::NAN, 0),
        ] {
            e.register_double(GET_FLOAT_TRAIT, move |_, a| {
                assert_eq!(a, [0x55, 0x100a]);
                float(trait_value)
            });
            assert_eq!(
                e.call(0x007b_3900, &args![this]).i32(),
                expected,
                "{trait_value}"
            );
        }
        let tileless = make_option(&mut e, 0, 0, 0);
        assert_eq!(
            e.call(0x007b_3900, &args![Ptr::<MenuOption>::new(tileless)])
                .i32(),
            0
        );
    }

    #[test]
    fn option_enabled_sets_the_user_trait() {
        let mut e = engine();
        e.register(ADD_USER_TRAIT, |_, _| word(0x777));
        let option = make_option(&mut e, 0x55, 0, 0);
        let this = Ptr::<MenuOption>::new(option);
        e.call(0x007b_3970, &args![this, true]);
        e.call(0x007b_3970, &args![this, false]);
        assert_eq!(
            calls(&e, SET_TRAIT_FROM_INT),
            vec![vec![0x55, 0x777, 1], vec![0x55, 0x777, 0]]
        );
        assert_eq!(
            calls(&e, ADD_USER_TRAIT)[0],
            vec![ENABLED_TRAIT_NAME, u32::MAX]
        );
        let tileless = make_option(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x007b_3970, &args![Ptr::<MenuOption>::new(tileless), true]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn option_position_is_the_truncated_user_trait() {
        let mut e = engine();
        e.register(ADD_USER_TRAIT, |_, _| word(0x777));
        e.register(GET_FLOAT_TRAIT, |_, a| {
            assert_eq!(a, [0x55, 0x777]);
            float(-1.0)
        });
        let option = make_option(&mut e, 0x55, 0, 0);
        assert_eq!(
            e.call(0x007b_39b0, &args![Ptr::<MenuOption>::new(option)])
                .i32(),
            -1
        );
        assert_eq!(
            calls(&e, ADD_USER_TRAIT),
            vec![vec![POSITION_TRAIT_NAME, u32::MAX]]
        );
    }

    #[test]
    fn option_list_constructor_renders_the_template_tile() {
        let (mut e, menu) = world();
        let rendered = e.mem.alloc(0x40);
        e.register_double(RENDER_TEMPLATE, move |_, _| word(rendered));
        e.register(ADD_USER_TRAIT, |_, _| word(0x777));
        let list = e.mem.alloc(0x24);
        let first_tile = e.mem.u32(menu.addr() + 0x78);
        e.mem.set_u32(list + 0x20, 0x1234);
        let result = e.call(
            0x007b_39e0,
            &args![Ptr::<MenuOptionList>::new(list), 7u32, 0x4455u32],
        );
        assert_eq!(result.u32(), list);
        assert_eq!(calls(&e, OPTION_LIST_BASE_CONSTRUCT), vec![vec![list]]);
        assert_eq!(e.mem.u32(list + 0x10), 0x14);
        assert_eq!(e.mem.u32(list + 0xc), 0x14);
        assert_eq!(e.mem.f32(list + 0x18), 0.0);
        assert_eq!(e.mem.u32(list + 0x1c), 0);
        assert_eq!(e.mem.u32(list + 0x20), 0);
        assert_eq!(e.mem.u32(list + 0x14), rendered);
        assert_eq!(
            calls(&e, RENDER_TEMPLATE),
            vec![vec![menu.addr(), first_tile, LIST_ITEM_TEMPLATE, 0]]
        );
        assert_eq!(
            calls(&e, SET_TRAIT_FROM_INT),
            vec![
                vec![rendered, 0x1004, 7],
                vec![rendered, 0xfaf, 0],
                vec![rendered, 0x777, 1]
            ]
        );
        assert_eq!(
            calls(&e, SET_STRING_TRAIT),
            vec![vec![rendered, TRAIT_STRING, 0x4455, 1]]
        );
        assert_eq!(
            calls(&e, ADD_USER_TRAIT),
            vec![vec![HEADER_TRAIT_NAME, u32::MAX]]
        );
        assert_eq!(calls(&e, UPDATE_TILE), vec![vec![rendered, 0]]);
    }

    #[test]
    fn option_list_destructor_deletes_the_template() {
        let mut e = engine();
        let list = e.new_object::<MenuOptionList>();
        let template = object_with_vtable(&mut e, 0x40, 0x0f46_0000, 0x0f47_0000);
        e.mem.set_u32(list.addr() + 0x14, template);
        e.call(0x007b_3af0, &args![list]);
        assert_eq!(calls(&e, 0x0f47_0000), vec![vec![template, 1]]);
        assert_eq!(e.mem.u32(list.addr() + 0x14), 0);
        assert_eq!(calls(&e, OPTION_LIST_CLEAR), vec![vec![list.addr()]]);
        assert_eq!(
            calls(&e, OPTION_LIST_BASE_DESTRUCT),
            vec![vec![list.addr()]]
        );
        // Without a template only the list steps run.
        e.call_log = Some(vec![]);
        e.call(0x007b_3af0, &args![list]);
        assert_eq!(calls(&e, 0x0f47_0000).len(), 0);
        assert_eq!(calls(&e, OPTION_LIST_CLEAR).len(), 1);
    }

    const TILE_VTABLE: u32 = 0x0f54_0000;
    const TILE_SLOTS: u32 = 0x0f55_0000;

    /// Where the test model keeps a tile's float trait `id`: in the tile's
    /// own memory.
    fn trait_at(tile: u32, id: u32) -> u32 {
        tile + 0x40 + 4 * (id & 0xff)
    }

    /// A tile of the test model: a vtable (slot doubles at `TILE_SLOTS`)
    /// and `parent` at `+0x28`.
    fn model_tile(e: &mut Engine, parent: u32) -> u32 {
        let tile = e.mem.alloc(0x440);
        e.mem.set_u32(tile, TILE_VTABLE);
        e.mem.set_u32(tile + 0x28, parent);
        tile
    }

    /// Tiles keep their float traits in their memory (`trait_at`); the
    /// user traits `_top_bound`, `_selected` and `_position` are 0x7c0,
    /// 0x7c1 and 0x7c2.
    fn model_traits(e: &mut Engine) {
        object_with_vtable(e, 0x10, TILE_VTABLE, TILE_SLOTS);
        e.register(GET_FLOAT_TRAIT, |e, a| {
            float(e.mem.f32(trait_at(a[0], a[1])) as f64)
        });
        e.register(SET_FLOAT_TRAIT, |e, a| {
            e.mem.set_f32(trait_at(a[0], a[1]), f32::from_bits(a[2]));
            Ret::default()
        });
        e.register(SET_TRAIT_FROM_INT, |e, a| {
            e.mem.set_f32(trait_at(a[0], a[1]), a[2] as i32 as f32);
            Ret::default()
        });
        e.register(ADD_USER_TRAIT, |_, a| {
            word(match a[0] {
                TOP_BOUND_TRAIT_NAME => 0x7c0,
                SELECTED_TRAIT_NAME => 0x7c1,
                POSITION_TRAIT_NAME => 0x7c2,
                _ => 0x7c3,
            })
        });
    }

    /// Appends a node (`NiTListItem`: next, previous, item) for `item`.
    fn append_node(e: &mut Engine, list: u32, item: u32) -> u32 {
        let node = e.mem.alloc(0xc);
        let tail = e.mem.u32(list + 4);
        e.mem.set_u32(node, 0);
        e.mem.set_u32(node + 4, tail);
        e.mem.set_u32(node + 8, item);
        if tail == 0 {
            e.mem.set_u32(list, node);
        } else {
            e.mem.set_u32(tail, node);
        }
        e.mem.set_u32(list + 4, node);
        node
    }

    /// The items of the list, head first.
    fn items_of(e: &Engine, list: u32) -> Vec<u32> {
        let mut items = vec![];
        let mut node = e.mem.u32(list);
        while node != 0 {
            items.push(e.mem.u32(node + 8));
            node = e.mem.u32(node);
        }
        items
    }

    /// The list helpers of other units, working on real `NiTListItem`s.
    fn real_lists(e: &mut Engine) {
        e.register(LIST_ADVANCE, |e, a| {
            let node = e.mem.u32(a[1]);
            let next = e.mem.u32(node);
            e.mem.set_u32(a[1], next);
            word(node + 8)
        });
        e.register(LIST_NODE_NEXT, |e, a| word(e.mem.u32(a[1])));
        e.register(LIST_NODE_PREVIOUS, |e, a| word(e.mem.u32(a[1] + 4)));
        e.register(LIST_NODE_DATA, |_, a| word(a[1] + 8));
        e.register(LIST_HEAD_DATA, |e, a| word(e.mem.u32(a[0]) + 8));
        e.register(LIST_TAIL_DATA, |e, a| word(e.mem.u32(a[0] + 4) + 8));
        e.register(LIST_FIND_NODE, |e, a| {
            let item = e.mem.u32(a[1]);
            let mut node = if a[2] != 0 { a[2] } else { e.mem.u32(a[0]) };
            while node != 0 {
                if e.mem.u32(node + 8) == item {
                    return word(node);
                }
                node = e.mem.u32(node);
            }
            word(0)
        });
        e.register(LIST_INSERT_ITEM, |e, a| {
            let item = e.mem.u32(a[1]);
            append_node(e, a[0], item);
            Ret::default()
        });
        e.register(LIST_INSERT_BEFORE, |e, a| {
            let (list, before) = (a[0], a[1]);
            let node = e.mem.alloc(0xc);
            let previous = e.mem.u32(before + 4);
            let item = e.mem.u32(a[2]);
            e.mem.set_u32(node, before);
            e.mem.set_u32(node + 4, previous);
            e.mem.set_u32(node + 8, item);
            e.mem.set_u32(before + 4, node);
            if previous == 0 {
                e.mem.set_u32(list, node);
            } else {
                e.mem.set_u32(previous, node);
            }
            Ret::default()
        });
        e.register(LIST_REMOVE_ITEM, |e, a| {
            let (list, item) = (a[0], e.mem.u32(a[1]));
            let mut node = e.mem.u32(list);
            while node != 0 && e.mem.u32(node + 8) != item {
                node = e.mem.u32(node);
            }
            if node != 0 {
                let (next, previous) = (e.mem.u32(node), e.mem.u32(node + 4));
                if previous == 0 {
                    e.mem.set_u32(list, next);
                } else {
                    e.mem.set_u32(previous, next);
                }
                if next == 0 {
                    e.mem.set_u32(list + 4, previous);
                } else {
                    e.mem.set_u32(next + 4, previous);
                }
            }
            Ret::default()
        });
    }

    /// An open menu, real lists and the tile model. Rendered tiles are 5
    /// high under a parent whose `_top_bound` is 10. Returns a list whose
    /// template tile has the number `number`.
    fn list_world(number: f32) -> (Engine, Ptr<RaceSexMenu>, Ptr<MenuOptionList>) {
        let mut e = engine();
        let menu = menu(&mut e);
        model_traits(&mut e);
        real_lists(&mut e);
        e.set_global(HIGHLIGHT_VALUE, 2.0f32);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        let parent = model_tile(&mut e, 0);
        e.mem.set_f32(trait_at(parent, 0x7c0), 10.0);
        e.register_double(RENDER_TEMPLATE, move |e, _| {
            let tile = model_tile(e, parent);
            e.mem.set_f32(trait_at(tile, 0xfb0), 5.0);
            word(tile)
        });
        let list = e.new_object::<MenuOptionList>();
        let template = model_tile(&mut e, 0);
        e.mem.set_f32(trait_at(template, 0x1004), number);
        e.set(list, MenuOptionList::pTemplateTile, template);
        e.call_log = Some(vec![]);
        (e, menu, list)
    }

    /// Three options with the ids 1, 2 and 3 (tiles at y 10, 15 and 20).
    fn three_options(e: &mut Engine, list: Ptr<MenuOptionList>) -> [u32; 3] {
        [1, 2, 3].map(|id| fn_007b3b80(e, list, 0x111, id).addr())
    }

    fn tile_y(e: &Engine, option: u32) -> f32 {
        e.mem.f32(trait_at(e.mem.u32(option), 0xfa2))
    }

    #[test]
    fn add_menu_option_places_and_highlights_the_options() {
        let (mut e, menu, list) = list_world(2.0);
        let first = fn_007b3b80(&mut e, list, 0x111, 3).addr();
        let tile = e.mem.u32(first);
        assert_eq!(e.mem.u32(first + 4), 3);
        assert_eq!(e.mem.u32(first + 8), 0);
        assert_eq!(tile_y(&e, first), 10.0);
        assert_eq!(e.mem.f32(trait_at(tile, 0x1004)), 2.0);
        assert_eq!(e.mem.f32(trait_at(tile, 0xfaf)), 1.0);
        assert_eq!(e.mem.f32(trait_at(tile, 0xfd6)), 2.0);
        assert_eq!(e.get(list, MenuOptionList::pHighlighted), first);
        assert_eq!(e.get(list, MenuOptionList::fScrollDelta), 5.0);
        let first_tile = e.mem.u32(menu.addr() + 0x78);
        assert_eq!(
            calls(&e, RENDER_TEMPLATE),
            vec![vec![menu.addr(), first_tile, LIST_ITEM_TEMPLATE, 0]]
        );
        // The first lists' first options get the Pip-Boy sound.
        assert_eq!(
            calls(&e, SET_STRING_TRAIT),
            vec![
                vec![tile, TRAIT_STRING, 0x111, 1],
                vec![tile, 0xfcb, SOUND_PIPBOY_MODE, 1]
            ]
        );
        // The second goes below the first; no sound for id 7, and the
        // highlight stays.
        e.call_log = Some(vec![]);
        let second =
            menu_option_list_add_menu_option(&mut e, list, 0x222, SLIDER_TEMPLATE, 7, 0).addr();
        assert_eq!(tile_y(&e, second), 15.0);
        assert_eq!(calls(&e, SET_STRING_TRAIT).len(), 1);
        assert_eq!(calls(&e, RENDER_TEMPLATE)[0][2], SLIDER_TEMPLATE);
        assert_eq!(e.get(list, MenuOptionList::pHighlighted), first);
        // InsertMenuOption puts a third one before the second node.
        let second_node = e.mem.u32(list.addr() + 4);
        e.call_log = Some(vec![]);
        let third =
            menu_option_list_insert_menu_option(&mut e, list, 1, 0x333, LIST_ITEM_TEMPLATE, 9)
                .addr();
        assert_eq!(
            calls(&e, LIST_INSERT_BEFORE)[0][..2],
            [list.addr(), second_node]
        );
        assert_eq!(items_of(&e, list.addr()), vec![first, third, second]);
        assert_eq!(tile_y(&e, third), 15.0);
        assert_eq!(tile_y(&e, second), 20.0);
        // No caption, or no open menu: nothing is added.
        assert!(
            menu_option_list_add_menu_option(&mut e, list, 0, LIST_ITEM_TEMPLATE, 1, 0).is_null()
        );
        e.set_global(RACE_SEX_MENU_ME, 0u32);
        assert!(
            menu_option_list_insert_menu_option(&mut e, list, 0, 0x1, LIST_ITEM_TEMPLATE, 1)
                .is_null()
        );
        assert_eq!(items_of(&e, list.addr()).len(), 3);
    }

    #[test]
    fn add_menu_option_keeps_a_larger_scroll_step_and_list_four_is_silent() {
        let (mut e, _menu, list) = list_world(4.0);
        e.set(list, MenuOptionList::fScrollDelta, 8.0);
        e.set(list, MenuOptionList::pHighlighted, 0x1234);
        let option = fn_007b3b80(&mut e, list, 0x111, 0).addr();
        assert_eq!(e.get(list, MenuOptionList::fScrollDelta), 8.0);
        assert_eq!(calls(&e, SET_STRING_TRAIT).len(), 1);
        assert_eq!(e.get(list, MenuOptionList::pHighlighted), 0x1234);
        let tile = e.mem.u32(option);
        assert_eq!(e.mem.f32(trait_at(tile, 0xfd6)), 0.0);
        assert_eq!(fn_007b4900(&mut e, option), option);
        assert_eq!(e.mem.u32(option), 0);
        assert_eq!(e.mem.u32(option + 4), 0);
    }

    #[test]
    fn sub_list_option_links_back_and_takes_the_sub_list_number() {
        let (mut e, _menu, list) = list_world(3.0);
        let sub = e.new_object::<MenuOptionList>();
        let template = model_tile(&mut e, 0);
        e.mem.set_f32(trait_at(template, 0x1004), 7.0);
        e.set(sub, MenuOptionList::pTemplateTile, template);
        e.register(STRING_FORMAT, |e, a| {
            e.mem.set_u32(a[0], 0x5150);
            Ret::default()
        });
        let option = fn_007b3bb0(&mut e, list, 0x4400, sub).addr();
        assert_eq!(e.get(sub, MenuOptionList::iBackTarget), 3);
        assert_eq!(e.mem.u32(option + 4), 7);
        let text = calls(&e, STRING_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&e, STRING_FORMAT),
            vec![vec![text, CAPTION_FORMAT, 0x4400, 0x3e]]
        );
        let tile = e.mem.u32(option);
        assert_eq!(
            calls(&e, SET_STRING_TRAIT)[0],
            vec![tile, TRAIT_STRING, 0x5150, 1]
        );
        assert_eq!(calls(&e, STRING_DESTROY), vec![vec![text]]);
    }

    #[test]
    fn slider_option_sets_range_value_and_fraction() {
        let (mut e, _menu, list) = list_world(5.0);
        e.register(ROUND_TO_STEP, |_, a| float(f32::from_bits(a[0]) as f64));
        let option =
            menu_option_list_add_slider_option(&mut e, list, 0x111, 0x18, 1, 20, 5, 0.5).addr();
        let tile = e.mem.u32(option);
        assert_eq!(calls(&e, RENDER_TEMPLATE)[0][2], SLIDER_TEMPLATE);
        assert_eq!(e.mem.f32(trait_at(tile, 0x1006)), 1.0);
        assert_eq!(e.mem.f32(trait_at(tile, 0x1007)), 20.0);
        assert_eq!(e.mem.f32(trait_at(tile, 0x1005)), 5.0);
        assert_eq!(e.mem.f32(trait_at(tile, 0x1008)), 10.0);
        assert_eq!(
            calls(&e, ROUND_TO_STEP),
            vec![vec![10.0f32.to_bits(), 1.0f32.to_bits()]]
        );
        // The fraction is limited to 0..1; NaN stays.
        for (fraction, expected) in [(-3.0f32, 0.0f32), (7.0, 20.0)] {
            let option =
                menu_option_list_add_slider_option(&mut e, list, 0x1, 0x18, 1, 20, 5, fraction)
                    .addr();
            let tile = e.mem.u32(option);
            assert_eq!(e.mem.f32(trait_at(tile, 0x1008)), expected);
        }
        let option =
            menu_option_list_add_slider_option(&mut e, list, 0x1, 0x18, 1, 20, 5, f32::NAN).addr();
        let tile = e.mem.u32(option);
        assert!(e.mem.f32(trait_at(tile, 0x1008)).is_nan());
    }

    #[test]
    fn remove_menu_option_moves_the_highlight_and_the_rest_up() {
        let (mut e, _menu, list) = list_world(5.0);
        let [first, second, third] = three_options(&mut e, list);
        let (second_tile, third_tile) = (e.mem.u32(second), e.mem.u32(third));
        e.set(list, MenuOptionList::pHighlighted, second);
        e.call_log = Some(vec![]);
        menu_option_list_remove_menu_option(&mut e, list, 2);
        assert_eq!(items_of(&e, list.addr()), vec![first, third]);
        assert_eq!(e.get(list, MenuOptionList::pHighlighted), third);
        assert_eq!(calls(&e, SET_FOCUS_TARGET_CDECL), vec![vec![third_tile]]);
        assert_eq!(calls(&e, TILE_SLOTS), vec![vec![second_tile, 1]]);
        assert_eq!(calls(&e, EMPTY_MEMBER), vec![vec![second]]);
        assert_eq!(calls(&e, FREE), vec![vec![second]]);
        assert_eq!(tile_y(&e, third), 15.0);
        // The last option, highlighted: the highlight takes the tail's
        // item, the removed option itself.
        e.set(list, MenuOptionList::pHighlighted, third);
        menu_option_list_remove_menu_option(&mut e, list, 3);
        assert_eq!(items_of(&e, list.addr()), vec![first]);
        assert_eq!(e.get(list, MenuOptionList::pHighlighted), third);
        // An id nobody has: nothing happens.
        e.call_log = Some(vec![]);
        menu_option_list_remove_menu_option(&mut e, list, 9);
        assert!(calls(&e, TILE_SLOTS).is_empty());
        assert!(calls(&e, LIST_REMOVE_ITEM).is_empty());
    }

    #[test]
    fn deleting_destructor_frees_with_bit_zero() {
        let mut e = engine();
        let object = e.mem.alloc(0xc);
        assert_eq!(fn_007b3fa0(&mut e, object, 1), object);
        assert_eq!(calls(&e, EMPTY_MEMBER), vec![vec![object]]);
        assert_eq!(calls(&e, FREE), vec![vec![object]]);
        let other = e.mem.alloc(0xc);
        assert_eq!(fn_007b3fa0(&mut e, other, 2), other);
        assert_eq!(calls(&e, FREE).len(), 1);
    }

    #[test]
    fn focus_gives_and_takes_the_interface_focus() {
        let mut e = engine();
        e.register(INTERFACE_SINGLETON, |_, _| word(0x9000));
        let list = e.new_object::<MenuOptionList>();
        let option = make_option(&mut e, 0x55, 1, 0);
        e.set(list, MenuOptionList::pHighlighted, option);
        assert_eq!(menu_option_list_focus(&mut e, list, true), 1);
        assert_eq!(
            calls(&e, SET_CURRENT_FOCUS_TARGET),
            vec![vec![0x9000, 0x55, 0xfc3, 1]]
        );
        // An option without a tile, or none: no focus, answer 0.
        e.mem.set_u32(option, 0);
        assert_eq!(menu_option_list_focus(&mut e, list, true), 0);
        e.set(list, MenuOptionList::pHighlighted, 0);
        assert_eq!(menu_option_list_focus(&mut e, list, true), 0);
        assert_eq!(calls(&e, SET_CURRENT_FOCUS_TARGET).len(), 1);
        // Unfocusing clears the highlight.
        e.set(list, MenuOptionList::pHighlighted, option);
        assert_eq!(menu_option_list_focus(&mut e, list, false), 1);
        assert_eq!(
            calls(&e, SET_CURRENT_FOCUS_TARGET)[1],
            vec![0x9000, 0, 0xfc3, 1]
        );
        assert_eq!(e.get(list, MenuOptionList::pHighlighted), 0);
    }

    #[test]
    fn linking_lists_sets_forward_and_back_targets() {
        let (mut e, _menu, list) = list_world(3.0);
        let next = e.new_object::<MenuOptionList>();
        let template = model_tile(&mut e, 0);
        e.mem.set_f32(trait_at(template, 0x1004), 8.9);
        e.set(next, MenuOptionList::pTemplateTile, template);
        fn_007b4050(&mut e, list, next);
        assert_eq!(e.get(list, MenuOptionList::iParentTarget), 8);
        assert_eq!(e.get(next, MenuOptionList::iBackTarget), 3);
    }

    #[test]
    fn selection_update_marks_options_by_the_test() {
        let (mut e, _menu, list) = list_world(5.0);
        let [first, second, third] = three_options(&mut e, list);
        e.register_double(0x0f56_0000, move |_, a| word((a[0] == second) as u32));
        // Without a test nothing happens.
        e.call_log = Some(vec![]);
        menu_option_list_update_current_selection(&mut e, list);
        assert!(calls(&e, ADD_USER_TRAIT).is_empty());
        e.set(list, MenuOptionList::pCallback, 0x0f56_0000);
        menu_option_list_update_current_selection(&mut e, list);
        assert_eq!(
            calls(&e, 0x0f56_0000),
            vec![vec![first], vec![second], vec![third]]
        );
        let selected = |e: &Engine, option: u32| e.mem.f32(trait_at(e.mem.u32(option), 0x7c1));
        assert_eq!(selected(&e, first), 1.0);
        assert_eq!(selected(&e, second), 2.0);
        assert_eq!(selected(&e, third), 1.0);
        // Options without a tile are left alone.
        let bare = make_option(&mut e, 0, 1, 0);
        e.call_log = Some(vec![]);
        fn_007b4140(&mut e, Ptr::new(bare));
        fn_007b4180(&mut e, Ptr::new(bare));
        assert!(calls(&e, ADD_USER_TRAIT).is_empty());
    }

    #[test]
    fn options_are_found_by_tile_and_by_id() {
        let (mut e, _menu, list) = list_world(5.0);
        let [_, second, third] = three_options(&mut e, list);
        let tile = e.mem.u32(second);
        assert_eq!(fn_007b41c0(&mut e, list, tile), second);
        assert_eq!(fn_007b41c0(&mut e, list, 0x9999), 0);
        assert_eq!(fn_007b4210(&mut e, list, 3), third);
        assert_eq!(fn_007b4210(&mut e, list, 4), 0);
    }

    #[test]
    fn gamepad_moves_the_highlight_through_the_list() {
        let (mut e, menu, list) = list_world(5.0);
        let [first, second, third] = three_options(&mut e, list);
        e.register(TILE_IS_VISIBLE, |_, _| word(1));
        let highlight = |e: &Engine| e.get(list, MenuOptionList::pHighlighted);
        e.set(list, MenuOptionList::pHighlighted, second);
        assert!(menu_option_list_do_gamepad(&mut e, list, 1));
        assert_eq!(highlight(&e), first);
        // At the top: nothing above, answer false; the highlight stays.
        assert!(!menu_option_list_do_gamepad(&mut e, list, 1));
        assert_eq!(highlight(&e), first);
        assert!(menu_option_list_do_gamepad(&mut e, list, 2));
        assert_eq!(highlight(&e), second);
        assert!(menu_option_list_do_gamepad(&mut e, list, 0x10));
        assert_eq!(highlight(&e), third);
        assert!(menu_option_list_do_gamepad(&mut e, list, 0xf));
        assert_eq!(highlight(&e), first);
        // No highlight: up takes the last, down the first.
        e.set(list, MenuOptionList::pHighlighted, 0);
        assert!(menu_option_list_do_gamepad(&mut e, list, 1));
        assert_eq!(highlight(&e), third);
        e.set(list, MenuOptionList::pHighlighted, 0);
        assert!(menu_option_list_do_gamepad(&mut e, list, 2));
        assert_eq!(highlight(&e), first);
        // Other buttons are not handled.
        e.call_log = Some(vec![]);
        assert!(!menu_option_list_do_gamepad(&mut e, list, 3));
        assert!(calls(&e, SET_FOCUS_TARGET_CDECL).is_empty());
        // A hidden tile makes the menu scroll, at most 100 times.
        e.register(TILE_IS_VISIBLE, |_, _| word(0));
        assert!(menu_option_list_do_gamepad(&mut e, list, 2));
        let scrolls = calls(&e, MENU_SLOTS + 0xc);
        assert_eq!(scrolls.len(), 100);
        assert_eq!(scrolls[0], vec![menu.addr(), 3, 0]);
        let tile = e.mem.u32(second);
        assert_eq!(calls(&e, SET_FOCUS_TARGET_CDECL), vec![vec![tile]]);
        e.call_log = Some(vec![]);
        assert!(menu_option_list_do_gamepad(&mut e, list, 0xf));
        assert_eq!(calls(&e, MENU_SLOTS + 0xc)[0], vec![menu.addr(), 2, 0]);
    }

    #[test]
    fn gamepad_refresh_highlights_the_first_and_scrolls_it_home() {
        let (mut e, _menu, list) = list_world(5.0);
        let [first, second, _] = three_options(&mut e, list);
        let tile = e.mem.u32(first);
        // Without a gamepad nothing happens.
        e.set(list, MenuOptionList::pHighlighted, 0);
        e.call_log = Some(vec![]);
        fn_007b4410(&mut e, list);
        assert!(calls(&e, SIMPLE_ARRAY_IS_EMPTY).is_empty());
        e.register(GAMEPAD_IN_USE, |_, _| word(1));
        e.register(TILE_GET_STRING, |_, _| word(0x4444));
        e.register(STRING_FROM_TEXT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        // `_position` answers 2 once, then 0.
        let asked = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let counter = asked.clone();
        e.register_double(GET_FLOAT_TRAIT, move |e, a| {
            if a[1] == 0x7c2 {
                counter.set(counter.get() + 1);
                return float(if counter.get() == 1 { 2.0 } else { 0.0 });
            }
            float(e.mem.f32(trait_at(a[0], a[1])) as f64)
        });
        fn_007b4410(&mut e, list);
        assert_eq!(e.get(list, MenuOptionList::pHighlighted), first);
        assert_eq!(calls(&e, TILE_GET_STRING), vec![vec![tile, 0xfe8]]);
        assert_eq!(
            calls(&e, SET_TRAIT_FROM_INT),
            vec![vec![tile, 0xfe8, u32::MAX]]
        );
        assert_eq!(calls(&e, SET_FOCUS_TARGET_CDECL), vec![vec![tile]]);
        assert_eq!(
            calls(&e, SET_STRING_TRAIT),
            vec![vec![tile, 0xfe8, 0x4444, 1]]
        );
        assert_eq!(asked.get(), 2);
        // Scrolled by -2 steps of 5.
        assert_eq!(tile_y(&e, first), 0.0);
        assert_eq!(tile_y(&e, second), 5.0);
        assert_eq!(calls(&e, STRING_DESTROY).len(), 1);
        // An empty list without a highlight: nothing more.
        e.set(list, MenuOptionList::pHighlighted, 0);
        e.register(SIMPLE_ARRAY_IS_EMPTY, |_, _| word(1));
        e.call_log = Some(vec![]);
        fn_007b4410(&mut e, list);
        assert!(calls(&e, TILE_GET_STRING).is_empty());
    }

    #[test]
    fn scroll_by_moves_every_option_only_for_a_positive_step() {
        let (mut e, _menu, list) = list_world(5.0);
        let [first, _, third] = three_options(&mut e, list);
        menu_option_list_scroll_by(&mut e, list, -1);
        assert_eq!(tile_y(&e, first), 5.0);
        assert_eq!(tile_y(&e, third), 15.0);
        for step in [0.0f32, -0.0, -1.0] {
            e.set(list, MenuOptionList::fScrollDelta, step);
            e.call_log = Some(vec![]);
            menu_option_list_scroll_by(&mut e, list, 3);
            assert!(calls(&e, SET_FLOAT_TRAIT).is_empty());
        }
        e.set(list, MenuOptionList::fScrollDelta, f32::NAN);
        menu_option_list_scroll_by(&mut e, list, 3);
        assert!(tile_y(&e, first).is_nan());
    }

    #[test]
    fn clearing_deletes_tiles_and_options() {
        let (mut e, _menu, list) = list_world(5.0);
        let [first, second, third] = three_options(&mut e, list);
        let tiles = [first, second, third].map(|option| e.mem.u32(option));
        e.call_log = Some(vec![]);
        fn_007b45b0(&mut e, list);
        assert_eq!(
            calls(&e, TILE_SLOTS),
            tiles.iter().map(|tile| vec![*tile, 1]).collect::<Vec<_>>()
        );
        assert_eq!(
            calls(&e, FREE),
            vec![vec![first], vec![second], vec![third]]
        );
        assert_eq!(calls(&e, LIST_REMOVE_ALL), vec![vec![list.addr()]]);
        assert_eq!(e.get(list, MenuOptionList::pHighlighted), 0);
    }

    /// A menu whose pressed tile (`+0xC0`, button `button`, parent with
    /// trait `0x1008` = 3) and option (`+0xC4`, id `id`, range 0 to 10,
    /// value 5) use the tile model.
    fn pressed_world(button: f32, id: u32) -> (Engine, Ptr<RaceSexMenu>, u32) {
        let mut e = engine();
        let menu = menu(&mut e);
        model_traits(&mut e);
        e.set_global(ONE_DOUBLE, 1.0f64);
        let parent = model_tile(&mut e, 0);
        e.mem.set_f32(trait_at(parent, 0x1008), 3.0);
        let pressed = model_tile(&mut e, parent);
        e.mem.set_f32(trait_at(pressed, 0xfaa), button);
        let tile = model_tile(&mut e, 0);
        e.mem.set_f32(trait_at(tile, 0x1007), 10.0);
        e.mem.set_f32(trait_at(tile, 0x1005), 5.0);
        let option = make_option(&mut e, tile, id, 0);
        e.mem.set_u32(menu.addr() + 0xc0, pressed);
        e.mem.set_u32(menu.addr() + 0xc4, option);
        e.call_log = Some(vec![]);
        (e, menu, option)
    }

    fn option_value(e: &Engine, option: u32) -> f32 {
        e.mem.f32(trait_at(e.mem.u32(option), 0x1005))
    }

    #[test]
    fn pressed_arrows_step_the_option_and_play_the_sound() {
        for (button, expected) in [(100.0, 4.0), (104.0, 6.0), (102.0, 2.0), (103.0, 8.0)] {
            let (mut e, menu, option) = pressed_world(button, 0x55);
            let pressed = e.mem.u32(menu.addr() + 0xc0);
            e.call(0x007a_dc60, &args![menu, 2u32, true]);
            fn_007b4930(&mut e, menu);
            assert_eq!(option_value(&e, option), expected);
            assert_eq!(calls(&e, PLAY_TILE_SOUND), vec![vec![pressed, 0xfcb]]);
            assert!(!race_sex_menu_get_flag(&mut e, menu, 2));
            assert_eq!(e.mem.u32(menu.addr() + 0xc0), 0);
            assert_eq!(e.mem.u32(menu.addr() + 0xc4), 0);
        }
        // At the top of the range nothing changes: no sound.
        let (mut e, menu, option) = pressed_world(104.0, 0x55);
        e.mem.set_f32(trait_at(e.mem.u32(option), 0x1005), 10.0);
        fn_007b4930(&mut e, menu);
        assert!(calls(&e, PLAY_TILE_SOUND).is_empty());
        // 0x69 counts as a change with flag 2.
        let (mut e, menu, _) = pressed_world(105.0, 0x55);
        fn_007b4930(&mut e, menu);
        assert!(calls(&e, PLAY_TILE_SOUND).is_empty());
        let (mut e, menu, _) = pressed_world(105.0, 0x55);
        race_sex_menu_set_flag(&mut e, menu, 2, true);
        fn_007b4930(&mut e, menu);
        assert_eq!(calls(&e, PLAY_TILE_SOUND).len(), 1);
        // Nothing pressed: nothing at all.
        let (mut e, menu, _) = pressed_world(100.0, 0x55);
        e.mem.set_u32(menu.addr() + 0xc4, 0);
        e.call_log = Some(vec![]);
        fn_007b4930(&mut e, menu);
        assert!(calls(&e, GET_FLOAT_TRAIT).is_empty());
    }

    #[test]
    fn pressed_colour_arrows_compare_the_parent_caption() {
        // The caption equals the setting's text: a change without a step.
        let (mut e, menu, option) = pressed_world(100.0, 0x1d);
        fn_007b4930(&mut e, menu);
        assert_eq!(calls(&e, STRING_COMPARE), vec![vec![0, SETTING_3528]]);
        assert_eq!(option_value(&e, option), 5.0);
        assert_eq!(calls(&e, PLAY_TILE_SOUND).len(), 1);
        // Otherwise the arrow steps the value.
        let (mut e, menu, option) = pressed_world(100.0, 0x1d);
        e.register(STRING_COMPARE, |_, _| word(1));
        fn_007b4930(&mut e, menu);
        assert_eq!(option_value(&e, option), 4.0);
    }

    #[test]
    fn slider_release_in_the_colour_list_changes_the_hair_colour() {
        let (mut e, menu) = world();
        e.register(CURRENT_LIST_INDEX, |_, _| word(6));
        let other = make_option(&mut e, 0x55, 0x1f, 0);
        let colour_option = make_option(&mut e, 0x66, 0x1d, 0);
        e.register_double(OPTION_LIST_FIND, move |_, _| word(colour_option));
        e.mem.set_u32(menu.addr() + 0xc4, other);
        race_sex_menu_set_flag(&mut e, menu, 7, true);
        race_sex_menu_slider_release(&mut e, menu);
        assert_eq!(
            calls(&e, SET_STRING_TRAIT)[0],
            vec![0x66, 0x1009, SETTING_3528, 1]
        );
        assert!(!race_sex_menu_get_flag(&mut e, menu, 7));
    }

    #[test]
    fn slider_release_of_a_face_slider_updates_the_head() {
        for (id, regenerated) in [(0x22u32, 1usize), (0x23, 0)] {
            let (mut e, menu) = world();
            let root = e.mem.alloc(0x100);
            player_roots(&mut e, root, root);
            let option = make_option(&mut e, 0x55, id, 0);
            e.mem.set_u32(menu.addr() + 0xc4, option);
            race_sex_menu_set_flag(&mut e, menu, 7, true);
            race_sex_menu_slider_release(&mut e, menu);
            // `007b1e70(0, 1, 0)` regenerates into LastFaceCoord;
            // `(0, 0, 1)` does not.
            let into_last = calls(&e, REGENERATE_FACE)
                .iter()
                .filter(|words| words[0] == menu.addr() + 0x124)
                .count();
            assert_eq!(into_last, regenerated);
            assert!(!race_sex_menu_get_flag(&mut e, menu, 8));
            assert!(!race_sex_menu_get_flag(&mut e, menu, 7));
            assert_eq!(race_sex_menu_get_flag(&mut e, menu, 9), id == 0x23);
        }
    }

    #[test]
    fn slider_release_of_the_preset_applies_the_preset_head() {
        let (mut e, menu) = world();
        walk_chains(&mut e);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x7777);
        e.register_double(ARRAY_ENTRY, move |_, _| word(slot));
        e.register(GET_FLOAT_TRAIT, |_, a| {
            float(if a[1] == 0x1005 { 3.0 } else { 0.0 })
        });
        let colour = make_option(&mut e, 0x66, 0x1d, 0);
        let head = chain(&mut e, &[colour]);
        let colour_list = list_of(&e, menu, 6);
        e.mem.set_u32(colour_list, head);
        let option = make_option(&mut e, 0x55, 0x18, 0);
        e.mem.set_u32(menu.addr() + 0xc4, option);
        race_sex_menu_set_flag(&mut e, menu, 8, true);
        race_sex_menu_set_flag(&mut e, menu, 9, true);
        race_sex_menu_slider_release(&mut e, menu);
        let base = e.mem.u32(menu.addr() + 0xd8);
        assert_eq!(calls(&e, ARRAY_ENTRY), vec![vec![menu.addr() + 0xdc, 2]]);
        assert_eq!(calls(&e, NPC_METHOD_603790), vec![vec![base, 0x7777]]);
        assert!(calls(&e, SET_STRING_TRAIT).contains(&vec![0x66, 0x1009, SETTING_3528, 1]));
        assert!(race_sex_menu_get_flag(&mut e, menu, 0));
        for bit in [7, 8, 9] {
            assert!(!race_sex_menu_get_flag(&mut e, menu, bit));
        }
    }

    /// The face value case: the option's value is `value`; the face
    /// coordinates `[0][0]` and `[0][1]` read 30 and 40.
    fn face_value_world(value: f32) -> (Engine, Ptr<RaceSexMenu>) {
        let (mut e, menu) = world();
        e.set_global(SHAPE_SCALE_DOUBLE, 5.55f64);
        e.set_global(SHAPE_OFFSET_DOUBLE, 5.6f64);
        e.set_global(LOWER_BOUND_DOUBLE, 15.0f64);
        e.set_global(UPPER_BOUND_DOUBLE, 65.0f64);
        e.set_global(LOWER_BOUND, 15.0f32);
        e.set_global(UPPER_BOUND, 65.0f32);
        e.register_double(GET_FLOAT_TRAIT, move |_, a| {
            float(if a[1] == 0x1005 { value as f64 } else { 0.0 })
        });
        e.register(FACE_COORD_GET, |_, a| {
            float(if a[2] == 0 { 30.0 } else { 40.0 })
        });
        let option = make_option(&mut e, 0x55, 0x1a, 0);
        e.mem.set_u32(menu.addr() + 0xc4, option);
        e.call_log = Some(vec![]);
        (e, menu)
    }

    #[test]
    fn slider_release_of_the_face_value_keeps_the_difference_in_bounds() {
        let (mut e, menu) = face_value_world(2.0);
        let base = e.mem.u32(menu.addr() + 0xd8);
        race_sex_menu_slider_release(&mut e, menu);
        let built = calls(&e, EH_VECTOR_CONSTRUCTOR_ITERATOR);
        let (current, initial) = (built[0][0], built[1][0]);
        assert_eq!(initial, current + 0x80);
        assert_eq!(calls(&e, INIT_FACE_GEN_COORD), vec![vec![initial]]);
        assert_eq!(
            calls(&e, GET_FACE_COORD),
            vec![vec![base, current], vec![base, menu.addr() + 0x124]]
        );
        // 2 * 5.55 + 5.6 = 16.7 -> 16; 40 - (30 - 16) = 26.
        assert_eq!(
            calls(&e, FACE_COORD_SET),
            vec![
                vec![current, 0, 0, 16.0f32.to_bits()],
                vec![current, 0, 1, 26.0f32.to_bits()]
            ]
        );
        assert_eq!(calls(&e, REGENERATE_FACE)[0], vec![0, current, initial, 0]);
        assert!(calls(&e, COPY_FACE_GEN_COORD).is_empty());
        assert_eq!(
            calls(&e, EH_VECTOR_DESTRUCTOR_ITERATOR),
            vec![
                vec![initial, 0x20, 4, FACE_COORD_DESTRUCT],
                vec![current, 0x20, 4, FACE_COORD_DESTRUCT]
            ]
        );
        assert!(race_sex_menu_get_flag(&mut e, menu, 9));
        assert!(!race_sex_menu_get_flag(&mut e, menu, 7));
        // Out of the bounds both are limited to 65 and to 15; the check of
        // `00652900` copies the coordinates.
        for (value, bound) in [(20.0f32, 65.0f32), (-10.0, 15.0)] {
            let (mut e, menu) = face_value_world(value);
            e.register(FACE_COORD_CHECK_652900, |_, _| word(1));
            race_sex_menu_slider_release(&mut e, menu);
            let set = calls(&e, FACE_COORD_SET);
            assert_eq!(set[0][3], bound.to_bits());
            assert_eq!(set[1][3], bound.to_bits());
            let initial = calls(&e, INIT_FACE_GEN_COORD)[0][0];
            assert_eq!(calls(&e, COPY_FACE_GEN_COORD), vec![vec![initial, 0, 0, 0]]);
        }
    }

    #[test]
    fn slider_release_of_another_option_does_nothing() {
        let (mut e, menu) = world();
        let option = make_option(&mut e, 0x55, 0x30, 0);
        e.mem.set_u32(menu.addr() + 0xc4, option);
        race_sex_menu_set_flag(&mut e, menu, 7, true);
        race_sex_menu_slider_release(&mut e, menu);
        assert!(race_sex_menu_get_flag(&mut e, menu, 7));
    }

    #[test]
    fn default_preset_is_the_minimum_of_the_preset_option() {
        let (mut e, menu) = world();
        walk_chains(&mut e);
        let slot = e.mem.alloc(4);
        e.register_double(ARRAY_ENTRY, move |_, _| word(slot));
        let option = make_option(&mut e, 0x55, 0x18, 0);
        e.register_double(OPTION_LIST_FIND, move |_, _| word(option));
        let head = chain(&mut e, &[option]);
        let preset_list = list_of(&e, menu, 2);
        e.mem.set_u32(preset_list, head);
        race_sex_menu_choose_default_preset(&mut e, menu);
        assert_eq!(calls(&e, SIMPLE_ARRAY_CLEAR).len(), 1);
        assert_eq!(calls(&e, NPC_METHOD_603790).len(), 1);
        assert!(race_sex_menu_get_flag(&mut e, menu, 0));
        assert_eq!(e.mem.u32(menu.addr() + 0xc4), 0);
    }

    #[test]
    fn randomize_needs_the_answer_or_flag_seven() {
        let (mut e, menu) = world();
        let base = e.mem.u32(menu.addr() + 0xd8);
        race_sex_menu_randomize(&mut e);
        assert!(calls(&e, NPC_METHOD_601830).is_empty());
        e.register(GET_MESSAGE_MENU_RESULT, |_, _| word(1));
        race_sex_menu_set_flag(&mut e, menu, 8, true);
        race_sex_menu_randomize(&mut e);
        assert_eq!(calls(&e, NPC_METHOD_601830), vec![vec![base, 1, 0, 1]]);
        assert!(race_sex_menu_get_flag(&mut e, menu, 7));
        assert!(!race_sex_menu_get_flag(&mut e, menu, 8));
        // Flag 7 alone is enough.
        e.register(GET_MESSAGE_MENU_RESULT, |_, _| word(0));
        race_sex_menu_randomize(&mut e);
        assert_eq!(calls(&e, NPC_METHOD_601830).len(), 2);
        // No open menu: nothing.
        e.set_global(RACE_SEX_MENU_ME, 0u32);
        e.call_log = Some(vec![]);
        race_sex_menu_randomize(&mut e);
        assert!(calls(&e, GET_MESSAGE_MENU_RESULT).is_empty());
    }

    #[test]
    fn registry_has_all_eighty_three_functions() {
        let funcs = funcs();
        assert_eq!(funcs.len(), 83);
        let mut addresses: Vec<u32> = funcs.iter().map(|(address, _)| *address).collect();
        addresses.sort_unstable();
        addresses.dedup();
        assert_eq!(addresses.len(), 83);
        assert_eq!(addresses[0], 0x007a_c1f0);
        assert_eq!(addresses[82], 0x007b_51f0);
        // The entries call the translations: 007b4210 by address.
        let (mut e, _menu, list) = list_world(5.0);
        let [_, second, _] = three_options(&mut e, list);
        let (_, find) = funcs
            .iter()
            .find(|(address, _)| *address == 0x007b_4210)
            .unwrap();
        assert_eq!(find(&mut e, &[list.addr(), 2]).u32(), second);
    }
}
