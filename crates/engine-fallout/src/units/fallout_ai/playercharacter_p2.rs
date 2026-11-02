//! `fallout/ai/playercharacter.cpp` (Xbox PDB source unit), part 2: its functions from `0094c490` up to
//! (not including) `0095d090` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::playercharacter`]; anything public there may be used here.
//!
//! This session covers `0094c490` to `009500a0` (40 functions): the
//! actor-value modifier accessors, the spell and scroll selection, the
//! parent-cell change handler (`0094cae0`), `ReturnToLastKnownGoodPosition`,
//! the one-hour wait step (`0094df80`), `Load3D`, `ExportProgressData`,
//! `RewardKarma` and the vanity-mode reset.
//!
//! The second session covers `00950110` to `00952a20` (40 functions): the
//! first-/third-person camera switch and its temporary variants,
//! `Resurrect`, the animation and biped accessors, `CloneInventory3D`, the
//! animation group player (`009520f0`), the first-person zoom (`00952290`),
//! the god-mode flags, and the topic and quest lists.
//!
//! The third session covers `00952b30` to `0095c9c0` (40 functions): the
//! quest target list and path, the map marker, `FocusOnActor`, the sit and
//! get-up packages, the pick-up and drop of objects (`00953ff0`, `00954610`),
//! the package predicates, the save size (`00954d40`), the old-format save
//! and load (`00955620`, `00956f70`, `00958990`, `00958ec0`, `00958fc0`)
//! and the buffer-format save, load, finish and revert (`009590f0`,
//! `0095a3b0`, `0095c0a0`, `0095c730`, `0095c9c0`).
//!
//! Notes for the next session: this part continues after `0095c9c0` (the
//! functions from there to `0095d090`).
//!
//! Conventions: the `ActorValueOwner` sits at `PlayerCharacter + 0xa4` and the
//! `MagicCaster` at `+0x88`, so a few functions receive those interior
//! pointers as `this` and subtract the offset. Many callees are `__thiscall`
//! functions that return with a plain `RET` while the caller has pushed words
//! for the next call; every call was read from the disassembly with that in
//! mind. The compiler's exception-unwinding frames, the stack-cookie checks
//! and the debug scope object of `Load3D` are not translated, except that the
//! scope object is constructed and destroyed as the game does.

#[allow(unused_imports)]
use super::playercharacter::*;
#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleList, NiPoint3};

// ---------------------------------------------------------------------
// Globals
// ---------------------------------------------------------------------

/// `PlayerCharacter *`.
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// The global object whose word at `+0x68` (read through `008d8520`) is handed
/// to `Sky::Update` and whose `00451530` test guards the border-region check.
const SKY_HOLDER_POINTER: u32 = 0x011d_ea10;
/// A global object pointer tested by `005dc960` and `00678ce0` before the
/// "called before the game was over" log line of `0094eb40`.
const GAME_STATE_POINTER: u32 = 0x011d_ea0c;
/// The `ModelLoader` singleton pointer (`ECX` of `ModelLoader::LoadFile`).
const MODEL_LOADER_POINTER: u32 = 0x011c_3b3c;
/// The object `0042ce10` is called on at the start of `Load3D`.
const LOAD_STATE_POINTER: u32 = 0x011d_df38;
/// The `TESIdleManager` singleton pointer (`ECX` of `GetRootFilenameList`).
const IDLE_MANAGER_POINTER: u32 = 0x011c_b6a0;
/// The calendar singleton.
const CALENDAR: u32 = 0x011d_e7b8;
/// The `ProcessLists` singleton.
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// The object whose float getter (`0084d030`) feeds `Sky::Update`.
const TIMER_SOURCE_011F6394: u32 = 0x011f_6394;
/// The lock object taken by `0040fbf0` and released by `0040fba0`, and the
/// byte that says the processing under it is enabled.
const PROCESS_LOCK: u32 = 0x011f_11a0;
const PROCESS_LOCK_ENABLED: u32 = 0x011f_1220;
/// The byte tested at the start of each wait step (`0094df80`): the step then
/// also updates the magic and the health; the step that ends the countdown sets
/// it to 1 (its value in the exe's data is 1).
const WAIT_STEP_ACTIVE: u32 = 0x011a_3b30;
/// The byte `0094dba0` returns.
const SKIP_WATER_HEIGHT_FLAG: u32 = 0x011c_7a5a;
/// The globals `0094e1d0`, `0094eaf0` and `0094eb40` fill with nodes found by
/// name (slot `0x9c`) in the player's models: the node named by `0094ead0`, the
/// "Camera1st" node, the "Camera3rd" node and the "Bip" node (the names are
/// the ones of the error messages).
const NODE_BY_EAD0_SLOT: u32 = 0x011e_07cc;
const NODE_CAMERA_1ST_SLOT: u32 = 0x011e_07d0;
const NODE_CAMERA_3RD_SLOT: u32 = 0x011e_07d4;
const NODE_BIP_SLOT: u32 = 0x011e_07d8;
/// The words `0094eac0`, `0094ead0`, `0094eae0` and `0094eb30` return (node
/// names the 3D lookups use).
const NODE_NAME_EAC0: u32 = 0x011c_61a8;
const NODE_NAME_EAD0: u32 = 0x011c_6264;
const NODE_NAME_EAE0: u32 = 0x011c_626c;
const NODE_NAME_EB30: u32 = 0x011c_6270;
/// The byte `00950090` returns, and the vanity-mode globals `009500a0` resets.
const VANITY_ACTIVE: u32 = 0x011e_07b8;
const VANITY_SAVED_FLAG: u32 = 0x011e_07b9;
const VANITY_SAVED_VALUE: u32 = 0x011e_07bc;
const VANITY_RESTORED_VALUE: u32 = 0x011e_0b5c;
const VANITY_FLAG_07C1: u32 = 0x011e_07c1;
const VANITY_FLAG_07C3: u32 = 0x011e_07c3;
const VANITY_VALUE_07C4: u32 = 0x011e_07c4;
const VANITY_VALUE_07C8: u32 = 0x011e_07c8;
const VANITY_VALUE_07DC: u32 = 0x011e_07dc;
const VANITY_VALUE_0B58: u32 = 0x011e_0b58;
const VANITY_VALUE_0B60: u32 = 0x011e_0b60;
/// The region that `0094cae0` found to hold sounds, kept for the sound code.
const CURRENT_REGION_WITH_SOUNDS: u32 = 0x011d_d380;
/// The `char *` table indexed by the reason argument of `ExportProgressData`.
const REASON_NAME_TABLE: u32 = 0x011a_3b44;
/// The `char *` table indexed by the value of `0087f4c0` (the sex of the player).
const SEX_NAME_TABLE: u32 = 0x0119_9e8c;
/// The `char *` table indexed by the cause word (`+0x10`) of the death extra data.
const CAUSE_NAME_TABLE: u32 = 0x0118_4a7c;
/// The game version string and the build-name buffer written to the export.
const GAME_VERSION_TEXT: u32 = 0x0107_6e68;
const BUILD_NAME_BUFFER: u32 = 0x011c_5fa0;
const UNKNOWN_TEXT: u32 = 0x0101_5890;
/// The nine words (a 3x3 matrix) `Load3D` copies and hands to `MultipleMatrixByRace`.
const LOAD_3D_MATRIX: u32 = 0x011a_9448;
/// The word `ExportProgressData` passes as the second argument of the file's
/// slot `0x14` (seek).
const EXPORT_SEEK_MODE: u32 = 0x010a_2488;

// Settings: objects whose text (`00403df0`), float (`00403e20`) or integer
// (`0043d4d0`) value the code reads. They are constructed at start-up (zero in
// the exe's data), so their names are the roles the code gives them.
/// Its text is the path of the player's 1st-person model.
const MODEL_PATH_SETTING: u32 = 0x011c_dd78;
/// Its text is the directory prefix of the progress-data file.
const EXPORT_DIRECTORY_SETTING: u32 = 0x011e_09d4;
/// The `float` that limits the change of actor value `0x4b` per wait step.
const ACTOR_VALUE_4B_LIMIT_SETTING: u32 = 0x011d_0c24;
// The karma is kept within the lower and upper limit; a change of the size of
// the threshold or more shows the big message. Each message is an icon and a
// text setting.
const KARMA_LOWER_LIMIT_SETTING: u32 = 0x011c_dd6c;
const KARMA_UPPER_LIMIT_SETTING: u32 = 0x011c_d644;
const KARMA_MESSAGE_THRESHOLD_SETTING: u32 = 0x011d_26ac;
const KARMA_LOSS_BIG_ICON_SETTING: u32 = 0x011d_42c4;
const KARMA_LOSS_BIG_TEXT_SETTING: u32 = 0x011d_4a14;
const KARMA_LOSS_ICON_SETTING: u32 = 0x011d_2cd0;
const KARMA_LOSS_TEXT_SETTING: u32 = 0x011d_3420;
const KARMA_GAIN_ICON_SETTING: u32 = 0x011d_3e38;
const KARMA_GAIN_TEXT_SETTING: u32 = 0x011d_1fb0;
const KARMA_GAIN_BIG_ICON_SETTING: u32 = 0x011d_3b14;
const KARMA_GAIN_BIG_TEXT_SETTING: u32 = 0x011d_3c70;
/// The text of the message shown by `ReturnToLastKnownGoodPosition`.
const RETURN_MESSAGE_SETTING: u32 = 0x011d_50bc;
/// A boolean setting read by `0094cae0` through `00408d60`.
const BORDER_REGION_SETTING: u32 = 0x011e_08e0;

// Float and double constants in the exe's data.
/// `2.0f`: the display time of the on-screen messages.
const MESSAGE_SECONDS: u32 = 0x0101_62c0;
/// `-FLT_MAX`: the water height when the cell has none.
const NO_WATER_HEIGHT: u32 = 0x0101_5f5c;
/// `4096.0` (`double`): the size of an exterior cell in world units.
const CELL_SIZE: u32 = 0x0101_7a10;
/// `2048.0` (`double`): the border distance limit.
const BORDER_DISTANCE_LIMIT: u32 = 0x0101_6968;
/// The sphere radius of the camera body (`float`, `0.001`).
const CAMERA_SPHERE_RADIUS: u32 = 0x0101_7d00;
/// `3600.0` (`double`): seconds per hour.
const SECONDS_PER_HOUR: u32 = 0x0101_2640;
/// The `float` (`3600.0`) the wait step passes to the process-list updates.
const WAIT_UPDATE_TIME: u32 = 0x0108_4838;

// Strings in the exe's data.
const SOURCE_FILE_NAME: u32 = 0x0108_a8e0;
const SAD_ICON_PATH: u32 = 0x0102_08a0;
const EMPTY_QUOTE_TEXT: u32 = 0x0102_1184;
const SOUND_KARMA_UP: u32 = 0x0108_b2d0;
const SOUND_KARMA_DOWN: u32 = 0x0108_b2dc;
const FORMAT_TEXT: u32 = 0x0108_b2b4;
const FORMAT_INT: u32 = 0x0108_b2b0;
const FORMAT_EXPORT_PATH: u32 = 0x0108_b2b8;
const FORMAT_PERK: u32 = 0x0108_b2a0;
const TEXT_CLOSE_QUOTE: u32 = 0x0108_b29c;
const TEXT_EMPTY_QUOTES: u32 = 0x0108_b298;
const FORMAT_TWO_FLOATS: u32 = 0x0108_b28c;
const FORMAT_ONE_FLOAT: u32 = 0x0108_b284;
const FORMAT_QUEST: u32 = 0x0108_b278;
const FORMAT_CELL_WORLD: u32 = 0x0108_b264;
const FORMAT_CELL_UNKNOWN_WORLD: u32 = 0x0108_b248;
const FORMAT_NAME_ID: u32 = 0x0108_b23c;
const TEXT_NONE: u32 = 0x0108_b234;
const FORMAT_CAUSE_OBJECT: u32 = 0x0108_b224;
const TEXT_UNKNOWN_CAUSE: u32 = 0x0108_b214;
const TEXT_UNKNOWN_OBJECT: u32 = 0x0108_b204;
const FORMAT_LEVELLED_BOTH: u32 = 0x0108_b1ec;
const FORMAT_LEVELLED_ORIGINAL_ONLY: u32 = 0x0108_b1d4;
const FORMAT_LEVELLED_TEMPLATE_ONLY: u32 = 0x0108_b1bc;
const TEXT_UNKNOWN_LEVELLED: u32 = 0x0108_b1a4;
const TEXT_NO_WEAPON: u32 = 0x0108_b198;
const TEXT_UNKNOWN_KILLER: u32 = 0x0108_b188;
const TEXT_LINE_END: u32 = 0x0101_f520;
const FORMAT_SET3D_MESSAGE: u32 = 0x0108_b130;
const FORMAT_MISSING_CAMERA_1ST: u32 = 0x0108_b110;
const FORMAT_MISSING_BIP: u32 = 0x0108_b0f8;
const FORMAT_MISSING_CAMERA_3RD: u32 = 0x0108_b0b8;
const TODDLER_SUFFIX: u32 = 0x0108_b0d8;
const FEMALE_SUFFIX: u32 = 0x0101_6f38;
const MALE_SUFFIX: u32 = 0x0101_6f1c;
const INVENTORY_PREFIX_WORN: u32 = 0x0108_b184;
const INVENTORY_PREFIX_CARRIED: u32 = 0x0101_1584;
const FORMAT_INVENTORY_ITEM: u32 = 0x0108_b174;

// ---------------------------------------------------------------------
// Functions of other units
// ---------------------------------------------------------------------

/// Reads the modifier of kind `mode` (0 temporary, 1 script, 2 damage) of an
/// actor value; `ECX` = the player (`0094c3d0`, result in ST0).
const GET_MODIFIER: u32 = 0x0094_c3d0;
/// `ClampAdd(current, delta, mode)` (`00937810`, `cdecl`): the sum, clamped to
/// at most 0 when `mode` is 0 and to at least 0 when `mode` is 1.
const CLAMP_ADD: u32 = 0x0093_7810;
/// `ActorValue::CheckClampDamageModifier` (Xbox PDB), `cdecl`: owner, actor
/// value index, `float`.
const CHECK_CLAMP_DAMAGE_MODIFIER: u32 = 0x0066_eea0;
/// Forwarded to by `0094c640` (`ECX` = the actor, two words, `float` result).
const MODIFIER_ITEM_GETTER: u32 = 0x0088_0660;
/// Forwarded to by `0094c660` (`ECX` = the actor, a word and a `float`).
const MODIFIER_ITEM_SETTER: u32 = 0x0088_0690;
/// `Actor::AddSpell` (Xbox PDB name of `008c32f0`) and `Actor::RemoveSpell`
/// (`008c3400`).
const ACTOR_ADD_SPELL: u32 = 0x008c_32f0;
const ACTOR_REMOVE_SPELL: u32 = 0x008c_3400;
/// `008c45e0`: `ECX` = the actor, two words, the address of an out word and a
/// flag byte; returns a bool.
const SPELL_CHECK: u32 = 0x008c_45e0;
/// `PlayerCharacter::IsGodMode` (Xbox PDB).
const IS_GOD_MODE: u32 = 0x0095_26b0;
/// `MagicCaster::CastAbility` (Xbox PDB name of `00815600`) and
/// `MagicCaster::TransferDisease` (`008157a0`).
const MAGIC_CASTER_CAST_ABILITY: u32 = 0x0081_5600;
const MAGIC_CASTER_TRANSFER_DISEASE: u32 = 0x0081_57a0;
/// `MagicTarget::AddTarget` (Xbox PDB name of `008230f0`).
const MAGIC_TARGET_ADD_TARGET: u32 = 0x0082_30f0;
/// `MagicItem::Unload` and `MagicItem::Preload` (Xbox PDB), one word each.
const MAGIC_ITEM_UNLOAD: u32 = 0x0040_b800;
const MAGIC_ITEM_PRELOAD: u32 = 0x0040_a420;
/// `PlayerCharacter::ResetMagicCastSound` (Xbox PDB).
const RESET_MAGIC_CAST_SOUND: u32 = 0x0096_1f90;
/// The scale of a reference (`00567400`, `TESObjectREFR::GetScale` in the
/// Xbox PDB; ST0).
const REFERENCE_GET_SCALE: u32 = 0x0056_7400;
/// `009a60e0` (`cdecl`): the player and a `float`; returns a pointer or 0.
const PLAYER_LOOKUP_BY_SIZE: u32 = 0x009a_60e0;
/// `00703350`: the fallback used when `009a60e0` finds nothing.
const FALLBACK_LOOKUP: u32 = 0x0070_3350;
/// Base constructor `0056e690` of the object `0094dbb0` builds.
const SPHERE_BASE_CONSTRUCTOR: u32 = 0x0056_e690;
/// `005dc960` and `00678ce0`: boolean tests on the global game-state object.
const GAME_STATE_TEST_1: u32 = 0x005d_c960;
const GAME_STATE_TEST_2: u32 = 0x0067_8ce0;
/// `TESObjectREFR::Set3D` (Xbox PDB name of `005702e0`): the node and a flag.
const REFERENCE_SET_3D: u32 = 0x0057_02e0;
/// `005b5e40` (`cdecl`): logs one message.
const LOG_MESSAGE: u32 = 0x005b_5e40;
/// Destructors with a delete flag: `00418e00` (biped animation) and
/// `00418d20` (animation).
const DELETE_BIPED_ANIM: u32 = 0x0041_8e00;
const DELETE_ANIMATION: u32 = 0x0041_8d20;
/// `NiPointer` assignment (`0066b0d0`): `ECX` = the pointer slot, the new
/// object.
const SMART_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `memset` wrapper `00403d30` (`cdecl`): block, value, size.
const MEMSET: u32 = 0x0040_3d30;
/// Reads the word at `+4` of the object in `ECX` (0 for a null `ECX`):
/// `00403df0`, used on settings to get their text.
const SETTING_TEXT: u32 = 0x0040_3df0;
/// Returns its first stack word (`00464f30`).
const IDENTITY: u32 = 0x0046_4f30;
/// `0045a5e0`: `ECX` = the model loader, one word.
const MODEL_LOADER_NOTIFY: u32 = 0x0045_a5e0;
/// `HUDMainMenu::SetMenuMode` (Xbox PDB name of `00771700`), `cdecl`.
const SET_MENU_MODE: u32 = 0x0077_1700;
/// `Interface::GetInventoryMenuVisible` (Xbox PDB name of `00704ad0`).
const INVENTORY_MENU_VISIBLE: u32 = 0x0070_4ad0;
/// `00483710` (11 bytes): called on the player by `00950030` and on sound
/// handles by `RewardKarma`.
const RELEASE_HANDLE: u32 = 0x0048_3710;
/// `ProcessLists::PrintLists` (Xbox PDB name of `008d0600`) and
/// `CharacterProgression::RewardExperience` (`008d5100`).
const PROCESS_LISTS_PRINT_LISTS: u32 = 0x008d_0600;
const CHARACTER_PROGRESSION_REWARD_EXPERIENCE: u32 = 0x008d_5100;
/// `ItemChange::GetWorn` (Xbox PDB name of `004bddd0`), one word.
const ITEM_CHANGE_GET_WORN: u32 = 0x004b_ddd0;
/// The form id of an object (`0084e3a0`: the word at `+0xc`).
const FORM_ID_OF: u32 = 0x0084_e3a0;
/// The word at `+8` of an object (`0044ddc0`): the item's form.
const WORD_AT_8: u32 = 0x0044_ddc0;
/// The full name of an item (`004be2d0`, `ItemChange::GetFullName` in the Xbox
/// PDB).
const ITEM_CHANGE_GET_FULL_NAME: u32 = 0x004b_e2d0;
/// `sprintf` (`00406d00`, `cdecl`) and the `BSString` append `BSStringT<char>::operator+=`
/// (Xbox PDB name of `00404820`).
const SPRINTF: u32 = 0x0040_6d00;
const STRING_APPEND: u32 = 0x0040_4820;
/// List helpers: item slot of a node (`006815c0`), next node (`00726070`),
/// emptiness test (`008256d0`), clear (`00470470`).
const LIST_NODE_ITEM_SLOT: u32 = 0x0068_15c0;
const LIST_NODE_NEXT: u32 = 0x0072_6070;
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_CLEAR: u32 = 0x0047_0470;
/// `MobileObject::GetCurrentParentCell`-style getter `008d6f30`: the player's
/// parent cell.
const PARENT_CELL_OF: u32 = 0x008d_6f30;
/// The extra data list of a reference (`005d43c0`).
const EXTRA_LIST_OF: u32 = 0x005d_43c0;
/// `BaseExtraList::GetExtraData` (Xbox PDB name of `00410220`).
const GET_EXTRA_DATA: u32 = 0x0041_0220;
/// The base form of a reference (`004181e0`).
const BASE_FORM_OF: u32 = 0x0041_81e0;
/// The form type byte of a form (`00401170`).
const FORM_TYPE_OF: u32 = 0x0040_1170;
/// `operator new(size)` (`cdecl`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `Error(format, ...)` (`cdecl`).
const ERROR: u32 = 0x0040_fbe0;
/// The setting getters: `00403e20` (float pointer) and `0043d4d0` (int
/// pointer); `ECX` = the setting.
const SETTING_FLOAT_POINTER: u32 = 0x0040_3e20;
const SETTING_INT_POINTER: u32 = 0x0043_d4d0;

// Constants of the functions from `00952b30` on.
/// `Bip01 Speaker`, the node `FocusOnActor` looks for when there is no head.
const SPEAKER_NODE_NAME: u32 = 0x0108_b3a4;
/// The log text of `FocusOnActor` for an actor without a head node.
const FORMAT_FOCUS_WITHOUT_HEAD: u32 = 0x0108_b338;
/// The last actor `FocusOnActor` warned about.
const FOCUS_LAST_WARNED_ACTOR: u32 = 0x011e_0d3c;
/// The blend of the previous `FocusOnActor` call (`float`).
const FOCUS_BLEND: u32 = 0x011e_0d48;
/// The ratio (`float`) and the angle limit (`float`) derived from the head's
/// bound, kept between calls.
const FOCUS_RATIO: u32 = 0x011e_0d44;
const FOCUS_ANGLE_LIMIT: u32 = 0x011e_0d40;
/// The bytes that say the pitch (`0d39`) and the yaw (`0d38`) are turning.
const FOCUS_PITCH_TURNING: u32 = 0x011e_0d39;
const FOCUS_YAW_TURNING: u32 = 0x011e_0d38;
/// The heading and the looking angle saved before the camera refresh.
const FOCUS_SAVED_HEADING: u32 = 0x011e_076c;
const FOCUS_SAVED_LOOKING: u32 = 0x011e_0764;
/// The `float` (32.0 in the exe's data) the bound of a head-less node gets.
const FOCUS_BOUND_RADIUS: u32 = 0x0101_e340;
/// `100.0` (`double`).
const HUNDRED: u32 = 0x0101_7a40;
/// `44.0` and `25.0` (`double`s): the distance base and scale of the field of
/// view `FocusOnActor` sets.
const FOCUS_DISTANCE_BASE: u32 = 0x0103_57e8;
const FOCUS_DISTANCE_SCALE: u32 = 0x0104_f2f0;
/// `pi`, `2 * pi` and `-pi` as `double`s (the exe's truncated values).
const PI_DOUBLE: u32 = 0x0101_ff40;
const TWO_PI_DOUBLE: u32 = 0x0101_ff48;
const MINUS_PI_DOUBLE: u32 = 0x0101_ff58;
/// `pi / 180` (`double`).
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
// Settings `FocusOnActor` reads (floats).
const HEAD_HEIGHT_SETTING: u32 = 0x011e_08d0;
const HEAD_DISTANCE_FACTOR_SETTING: u32 = 0x011e_0b84;
const FOCUS_MAXIMUM_SETTING: u32 = 0x0120_3150;
const FOCUS_SMOOTHING_SETTING: u32 = 0x011d_3ee0;
const FOCUS_RATE_SETTING: u32 = 0x011e_0958;
const PITCH_START_SETTING: u32 = 0x011e_094c;
const PITCH_STOP_SETTING: u32 = 0x011e_0900;
const YAW_START_SETTING: u32 = 0x011e_0b28;
const YAW_STOP_SETTING: u32 = 0x011e_08c4;
/// The double (5.0 in the exe's data) after which the greet flag resets.
const GREET_TIMEOUT: u32 = 0x0102_0998;
/// The actor `fn_00953ff0` found to blame for a theft.
const STEAL_ACTOR: u32 = 0x011e_07a4;
/// `##NifRound`, the node hidden when the current ammo is dropped.
const ROUND_NODE_NAME: u32 = 0x0108_9a7c;
/// The save-load object (`TESSaveLoadGame`) pointer and its debug setting.
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
const SAVE_SIZE_DEBUG_SETTING: u32 = 0x011d_e4e8;
/// The texts of the `GetSaveSize()` and `SaveGame()` debug reports: with a
/// world space record, and without.
const FORMAT_SAVE_SIZE_FORM: u32 = 0x0101_2cb0;
const FORMAT_SAVE_SIZE: u32 = 0x0101_2c78;
const SAVE_SIZE_FORMATS: (u32, u32) = (FORMAT_SAVE_SIZE_FORM, FORMAT_SAVE_SIZE);
const FORMAT_SAVE_GAME_FORM: u32 = 0x0101_53a0;
const FORMAT_SAVE_GAME: u32 = 0x0101_536c;
/// The log text for a save block longer than 16 bits.
const FORMAT_BLOCK_TOO_BIG: u32 = 0x0101_5318;
/// Form pointers the save writes as ids (globals).
const SAVED_FORM_011E0784: u32 = 0x011e_0784;
const SAVED_FORM_011E078C: u32 = 0x011e_078c;
/// `__RTDynamicCast` and the two type descriptors the selected spell is cast
/// between.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const RTTI_CAST_SOURCE: u32 = 0x0118_3140;
const RTTI_CAST_TARGET: u32 = 0x0118_3060;
/// The element count of a `BSSimpleList` (`005ae380`, `ECX` = the list).
const LIST_COUNT: u32 = 0x005a_e380;
/// Type descriptors the load casts forms to (source `01183028`, a `TESForm`).
const RTTI_LOAD_SOURCE: u32 = 0x0118_3028;
const RTTI_LOAD_REGION: u32 = 0x0118_99b4;
const RTTI_LOAD_FORM_0604: u32 = 0x0118_41cc;
const RTTI_LOAD_REGION_DATA: u32 = 0x0118_629c;
const RTTI_LOAD_QUEST: u32 = 0x0118_6500;
const RTTI_LOAD_TOPIC: u32 = 0x0118_4720;
const RTTI_LOAD_FIRE_NODE: u32 = 0x0118_6424;
/// The global tested after the perk list is loaded.
const LOADED_PERK_FLAG: u32 = 0x011e_07b4;
/// The log texts of the load: wrong block header (with and without the form
/// being loaded), buffer overrun and underrun (the same).
const FORMAT_BLOCK_HEADER_FORM: u32 = 0x0101_5718;
const FORMAT_BLOCK_HEADER: u32 = 0x0101_56a8;
const FORMAT_OVERRUN_FORM: u32 = 0x0101_5588;
const FORMAT_UNDERRUN_FORM: u32 = 0x0101_5500;
const FORMAT_OVERRUN: u32 = 0x0101_54a0;
const FORMAT_UNDERRUN: u32 = 0x0101_5440;
/// Appends the item whose address is the argument to a `BSSimpleList`
/// (`005ae3d0`, `ECX` = the list).
const LIST_APPEND: u32 = 0x005a_e3d0;
/// Type descriptors the second load pass casts to.
const RTTI_RESOLVE_0208: u32 = 0x0119_9c3c;
const RTTI_SPELL_INTERFACE: u32 = 0x0118_395c;
const RTTI_MAGIC_ITEM_FORM: u32 = 0x0118_30e8;
const RTTI_RESOLVE_SCROLL: u32 = 0x0118_a650;
const RTTI_RESOLVE_0758: u32 = 0x0118_30cc;
/// More type descriptors of the buffer load: the list at `+0x5e4`, the perk
/// entries and the effect items.
const RTTI_LOAD_LIST_05E4: u32 = 0x0118_640c;
const RTTI_LOAD_PERK: u32 = 0x0118_61dc;
const RTTI_LOAD_EFFECT_ITEM: u32 = 0x0118_c5c4;
/// The object `0084a810` is called on for the flagged first-person data, and
/// the holder that finds a combat group by id (`00991d60`).
const STATISTICS_OBJECT: u32 = 0x011d_df38;
const COMBAT_HOLDER: u32 = 0x011f_1958;
/// The type descriptor of the forms at `+0xd2c`, `+0xd44` and the `+0xd48`
/// list; the two heartbeat settings (floats) and sound names.
const RTTI_RESOLVE_0D2C: u32 = 0x0118_46d4;
const HEARTBEAT_UPPER_SETTING: u32 = 0x011d_00b4;
const HEARTBEAT_LOWER_SETTING: u32 = 0x011d_04a8;
const HEARTBEAT_SOUND_ALP: u32 = 0x0108_af40;
const HEARTBEAT_SOUND_BLP: u32 = 0x0108_af28;
/// Camera values `fn_0095c9c0` resets: a float (`0768`), the saved alpha
/// (`011a3b34`, set to -1.0) and a word (`0788`); the `float` (5.0) the
/// player's timer at `+0x684` restarts with; the two view offset settings.
const CAMERA_VALUE_0768: u32 = 0x011e_0768;
const INVENTORY_ALPHA_SAVE: u32 = 0x011a_3b34;
const CAMERA_WORD_0788: u32 = 0x011e_0788;
const DEFAULT_TIMER_VALUE: u32 = 0x0101_712c;
const VIEW_OFFSET_SETTING_A: u32 = 0x0120_315c;
const VIEW_OFFSET_SETTING_B: u32 = 0x0120_3168;

// ---------------------------------------------------------------------
// Layout
// ---------------------------------------------------------------------

layout! {
    /// `PlayerCharacter` (Xbox PDB), `0xe50` bytes on PC (the PDB's `0xe5c` is
    /// larger: the `TESForm` of the PDB has `0x10` more bytes, and a few
    /// accounting words differ; the constructor `00938180` is allocated with
    /// `0xe50`). Offsets are the PC ones, checked against the code of this
    /// part: PDB offset minus `0x10` for everything after `TESForm`. Only the
    /// fields the translations of this part use are listed.
    pub struct PlayerCharacter: 0xe50 {
        /// `pCurrentProcess` (Xbox PDB): `BaseProcess *`.
        0x68 pCurrentProcess: Ptr,
        /// `pMyKiller` (Xbox PDB): `Actor *`.
        0xc0 pMyKiller: Ptr,
        /// `fHealthModifier` (Xbox PDB): the damage modifier of health.
        0x4ac fHealthModifier: f32,
        /// `iSleepTime` (Xbox PDB): hours left to wait or sleep.
        0x654 iSleepTime: i32,
        /// `bIsSleeping` (Xbox PDB).
        0x658 bIsSleeping: bool,
        /// `b3rdPerson` (Xbox PDB).
        0x64a b3rdPerson: bool,
        /// `bActually3rdPerson` (Xbox PDB).
        0x64b bActually3rdPerson: u8,
        /// `bWant3rdPerson` (Xbox PDB).
        0x64c bWant3rdPerson: u8,
        /// `bTemp3rdPerson` (Xbox PDB).
        0x64d bTemp3rdPerson: u8,
        /// `bTemp3rdPersonSwitchBack` (Xbox PDB).
        0x64e bTemp3rdPersonSwitchBack: u8,
        /// `bTemp1stPerson` (Xbox PDB).
        0x64f bTemp1stPerson: u8,
        /// `bTemp1stPersonSwitchBack` (Xbox PDB).
        0x650 bTemp1stPersonSwitchBack: u8,
        /// `fFOV` (Xbox PDB).
        0x65c fFOV: f32,
        /// `spInventoryPC` (Xbox PDB): `NiPointer<NiNode>`.
        0x69c spInventoryPC: Ptr,
        /// `pInventoryAnimation` (Xbox PDB): `Animation *`.
        0x6a0 pInventoryAnimation: Ptr,
        /// `pActiveQuest` (Xbox PDB): `TESQuest *`.
        0x6b8 pActiveQuest: Ptr,
        /// `p1stPersonBipedAnim` (Xbox PDB): `BipedAnim *`.
        0x68c p1stPersonBipedAnim: Ptr,
        /// `p1stPersonAnimation` (Xbox PDB): `Animation *`.
        0x690 p1stPersonAnimation: Ptr,
        /// `sp1stPerson3D` (Xbox PDB): `NiPointer<NiAVObject>`.
        0x694 sp1stPerson3D: Ptr,
        /// `fEyeHeight` (Xbox PDB).
        0x698 fEyeHeight: f32,
        /// `listQuestLog` (Xbox PDB): `BSSimpleList<TESQuestStageItem *>`.
        0x6b0 listQuestLog: Inline<BSSimpleList>,
        /// `pSelectedSpell` (Xbox PDB): the `MagicItem` interface pointer.
        0x6ec pSelectedSpell: Ptr,
        /// `pSelectedScroll` (Xbox PDB): `TESObjectBOOK *`.
        0x6f0 pSelectedScroll: Ptr,
        /// `bChargen` (Xbox PDB).
        0x75c bChargen: u8,
        /// `pOccupiedRegion` (Xbox PDB): `TESRegion *`.
        0x760 pOccupiedRegion: Ptr,
        /// `m_AllOccupiedRegions` (Xbox PDB): `TESRegionList`, whose
        /// `BSSimpleList` starts 4 bytes in (after its vtable pointer).
        0x764 m_AllOccupiedRegions: Ptr,
        /// The `BSSimpleList` inside `m_AllOccupiedRegions` (`+0x768`).
        0x768 AllOccupiedRegionsList: Inline<BSSimpleList>,
        /// `CurrentRegionSoundList` (Xbox PDB): `BSSimpleList<TESRegionSound *>`.
        0x774 CurrentRegionSoundList: Inline<BSSimpleList>,
        /// `bInBorderContainedCell` (Xbox PDB).
        0x79c bInBorderContainedCell: bool,
        /// `bReturnToLastKnownGoodPosition` (Xbox PDB).
        0x79d bReturnToLastKnownGoodPosition: bool,
        /// `LastKnownGoodPosition` (Xbox PDB).
        0x7a0 LastKnownGoodPosition: Inline<NiPoint3>,
        /// `pLastKnownGoodLocation` (Xbox PDB): `TESForm *`.
        0x7ac pLastKnownGoodLocation: Ptr,
        /// `pBorderRegions` (Xbox PDB): `NiTPrimitiveArray<TESRegion *> *`.
        0x7b0 pBorderRegions: Ptr,
        /// `pLastKnownMusicType` (Xbox PDB): `BGSMusicType *`.
        0x7b4 pLastKnownMusicType: Ptr,
        /// `bIsToddler` (Xbox PDB).
        0x7c6 bIsToddler: bool,
        /// `Perks` (Xbox PDB): `BSSimpleList<PerkRankData *>`.
        0x87c Perks: Inline<BSSimpleList>,
        /// `pWobbleNodes` (Xbox PDB): the first of the cleared pointers.
        0xd74 pWobbleNodes: Ptr,
        /// `spCameraRigidBody` (Xbox PDB): `NiPointer<bhkRigidBody>`.
        0xdec spCameraRigidBody: Ptr,
    }
}

/// Offset of the `ActorValueOwner` sub-object in `PlayerCharacter`.
const ACTOR_VALUE_OWNER: u32 = 0xa4;
/// Offset of the `MagicCaster` sub-object.
const MAGIC_CASTER: u32 = 0x88;
/// Offset of the `MagicTarget` sub-object.
const MAGIC_TARGET: u32 = 0x94;
/// Offset of the `CharacterProgression` sub-object.
const CHARACTER_PROGRESSION: u32 = 0x878;
/// Bases of the actor-value modifier arrays (`float` per actor value).
const TEMPORARY_MODIFIERS: u32 = 0x244;
const SCRIPT_MODIFIERS: u32 = 0x378;
const DAMAGE_MODIFIERS: u32 = 0x4b0;
/// The actor value index that has its own modifier word (`fHealthModifier`).
const HEALTH_ACTOR_VALUE: i32 = 0x10;

/// The word at `+4` of a setting object (its text pointer), `00403df0`.
fn setting_text(e: &mut Engine, setting: u32) -> u32 {
    e.call(SETTING_TEXT, &args![setting]).u32()
}

/// An integer setting (`0043d4d0` gives the address of its value).
fn setting_int(e: &mut Engine, setting: u32) -> i32 {
    let at = e.call(SETTING_INT_POINTER, &args![setting]).u32();
    e.mem.i32(at)
}

/// The value slot of the list node (`006815c0`).
fn list_item_slot(e: &mut Engine, node: u32) -> u32 {
    e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32()
}

/// The next node (`00726070`).
fn list_next(e: &mut Engine, node: u32) -> u32 {
    e.call(LIST_NODE_NEXT, &args![node]).u32()
}

/// Whether the list head is empty (`008256d0`).
fn list_is_empty(e: &mut Engine, node: u32) -> bool {
    e.call(LIST_IS_EMPTY, &args![node]).bool()
}

/// The form id (`0084e3a0`).
fn form_id_of(e: &mut Engine, object: u32) -> u32 {
    e.call(FORM_ID_OF, &args![object]).u32()
}

/// The pointer of the actor's process (`this + 0x68`).
fn process_of(e: &Engine, this: Ptr<PlayerCharacter>) -> u32 {
    e.get(this, PlayerCharacter::pCurrentProcess).addr()
}

// Translated from 0094c490 (decompiled, FalloutNV.exe 1.4.0.525)
/// The damage modifier (kind 2, the array at `+0x4b0`) of an actor value.
/// `this` is the `ActorValueOwner` sub-object at `+0xa4` of the player; the
/// result is `0094c3d0(player, 2, actor_value)`.
pub fn fn_0094c490(e: &mut Engine, this: Ptr, actor_value: u32) -> f32 {
    let player = this.addr().wrapping_sub(ACTOR_VALUE_OWNER);
    e.call(GET_MODIFIER, &args![player, 2u32, actor_value])
        .f32()
}

// Translated from 0094c4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The script modifier (kind 1, the array at `+0x378`) of an actor value.
/// `this` is the `ActorValueOwner` sub-object at `+0xa4` of the player; the
/// result is `0094c3d0(player, 1, actor_value)`.
pub fn fn_0094c4c0(e: &mut Engine, this: Ptr, actor_value: u32) -> f32 {
    let player = this.addr().wrapping_sub(ACTOR_VALUE_OWNER);
    e.call(GET_MODIFIER, &args![player, 1u32, actor_value])
        .f32()
}

// Translated from 0094c4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `delta` to the modifier of an actor value. `mode` 0 is the
/// temporary modifier array (`+0x244`), 1 the script modifier array
/// (`+0x378`), 2 the damage modifier array (`+0x4b0`, with the actor value
/// `0x10` kept in `fHealthModifier` at `+0x4ac`); the delta of mode 2 first
/// goes through `ActorValue::CheckClampDamageModifier`. The new value is
/// `ClampAdd(old, delta, clamp)`. An actor value of `-1` changes nothing
/// (mode 2 still calls the damage check). Any other mode does nothing.
pub fn fn_0094c4f0(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    mode: i32,
    actor_value: i32,
    delta: f32,
    clamp: u32,
) {
    let base = this.addr();
    let index_bytes = (actor_value as u32).wrapping_mul(4);
    match mode {
        0 | 1 => {
            if actor_value != -1 {
                let array = if mode == 0 {
                    TEMPORARY_MODIFIERS
                } else {
                    SCRIPT_MODIFIERS
                };
                let at = base.wrapping_add(array).wrapping_add(index_bytes);
                let current = e.mem.f32(at);
                let value = e.call(CLAMP_ADD, &args![current, delta, clamp]).f32();
                e.mem.set_f32(at, value);
            }
        }
        2 => {
            let owner = if this.is_null() {
                0
            } else {
                base.wrapping_add(ACTOR_VALUE_OWNER)
            };
            let checked = e
                .call(
                    CHECK_CLAMP_DAMAGE_MODIFIER,
                    &args![owner, actor_value, delta],
                )
                .f32();
            if actor_value == HEALTH_ACTOR_VALUE {
                let current = e.get(this, PlayerCharacter::fHealthModifier);
                let value = e.call(CLAMP_ADD, &args![current, checked, clamp]).f32();
                e.set(this, PlayerCharacter::fHealthModifier, value);
            } else if actor_value != -1 {
                let at = base
                    .wrapping_add(DAMAGE_MODIFIERS)
                    .wrapping_add(index_bytes);
                let current = e.mem.f32(at);
                let value = e.call(CLAMP_ADD, &args![current, checked, clamp]).f32();
                e.mem.set_f32(at, value);
            }
        }
        _ => {}
    }
}

// Translated from 0094c640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `00880660` (`ECX` = the actor, two words) and returns its
/// `float`.
pub fn fn_0094c640(e: &mut Engine, this: Ptr, first: u32, second: u32) -> f32 {
    e.call(MODIFIER_ITEM_GETTER, &args![this, first, second])
        .f32()
}

// Translated from 0094c660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `00880690` (`ECX` = the actor, a word and a `float`).
pub fn fn_0094c660(e: &mut Engine, this: Ptr, first: u32, second: f32) {
    e.call(MODIFIER_ITEM_SETTER, &args![this, first, second]);
}

// Translated from 0094c680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::AddSpell` (Xbox PDB): adds the spell through
/// `Actor::AddSpell` and, when that succeeded, runs `SpellLearned`. Returns
/// whether the spell was added.
pub fn player_character_add_spell(e: &mut Engine, this: Ptr<PlayerCharacter>, spell: Ptr) -> bool {
    let added = e.call(ACTOR_ADD_SPELL, &args![this, spell]).bool();
    if added {
        player_character_spell_learned(e, this, spell);
    }
    added
}

// Translated from 0094c6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::SpellLearned` (Xbox PDB): for a spell whose type (slot
/// `0x18` of its interface at `+0x18`) is 0, 4, 2 or 3, walks the effect list
/// that starts at `+0x28` (a node whose list head is tested with `008256d0`)
/// and, for every entry that has an item, calls `00407e00` on the object
/// `00825c00` gives for that item with `0x200000` and 1. `this` is not used.
pub fn player_character_spell_learned(e: &mut Engine, _this: Ptr<PlayerCharacter>, spell: Ptr) {
    let interface = spell.addr().wrapping_add(0x18);
    let kinds = [0i32, 4, 2, 3];
    let mut matches = false;
    for expected in kinds {
        if e.vcall(interface, 0x18, &args![]).i32() == expected {
            matches = true;
            break;
        }
    }
    if !matches {
        return;
    }
    let mut node = spell.addr();
    while node != 0 {
        let head = node.wrapping_add(0x28);
        if list_is_empty(e, head) {
            break;
        }
        let slot = list_item_slot(e, head);
        let item = e.mem.u32(slot);
        if item != 0 {
            let target = e.call(0x0082_5c00, &args![item]).u32();
            e.call(0x0040_7e00, &args![target, 0x0020_0000u32, 1u32]);
        }
        let next = list_next(e, head);
        node = if next == 0 {
            0
        } else {
            next.wrapping_sub(0x28)
        };
    }
}

// Translated from 0094c7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the spell through `Actor::RemoveSpell` (`008c3400`); when that
/// succeeded and the spell's interface (`spell + 0x18`, or 0 for no spell) is
/// the selected spell, clears the selection. Returns whether the spell was
/// removed.
pub fn fn_0094c7a0(e: &mut Engine, this: Ptr<PlayerCharacter>, spell: Ptr) -> bool {
    let removed = e.call(ACTOR_REMOVE_SPELL, &args![this, spell]).bool();
    if removed {
        let interface = if spell.is_null() {
            0
        } else {
            spell.addr().wrapping_add(0x18)
        };
        if interface == fn_0094c8c0(e, this) {
            player_character_set_selected_spell(e, this, 0);
        }
    }
    removed
}

// Translated from 0094c800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `008c45e0(this, first, second, &code, flag)`; stores the code it
/// reports at `out` when `out` is not null. The result is false when that call
/// failed with code 5; otherwise it is true when
/// `PlayerCharacter::IsGodMode` is, and else the call's own result.
pub fn fn_0094c800(e: &mut Engine, this: Ptr, first: u32, second: u32, out: Ptr, flag: u8) -> bool {
    // The exe's frame (`SUB ESP,0x10` under `EBP`): the code at +8 (`EBP-0x8`).
    let (ok, reported) = e.with_stack(0x10, |e, frame| {
        let code = frame.addr() + 8;
        e.mem.set_u32(code, 0);
        let ok = e
            .call(SPELL_CHECK, &args![this, first, second, code, flag])
            .bool();
        (ok, e.mem.u32(code))
    });
    if !out.is_null() {
        e.mem.set_u32(out.addr(), reported);
    }
    if !ok && reported == 5 {
        return false;
    }
    if e.call(IS_GOD_MODE, &args![]).bool() {
        true
    } else {
        ok
    }
}

// Translated from 0094c870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `MagicCaster::CastAbility` (Xbox PDB name of `00815600`).
pub fn fn_0094c870(e: &mut Engine, this: Ptr, ability: u32, flag: u8) {
    e.call(MAGIC_CASTER_CAST_ABILITY, &args![this, ability, flag]);
}

// Translated from 0094c890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `MagicCaster::TransferDisease` (Xbox PDB name of `008157a0`).
pub fn fn_0094c890(e: &mut Engine, this: Ptr, first: u32, second: u32, flag: u8) {
    e.call(
        MAGIC_CASTER_TRANSFER_DISEASE,
        &args![this, first, second, flag],
    );
}

// Translated from 0094c8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The selected spell (`pSelectedSpell`, `+0x6ec`).
pub fn fn_0094c8c0(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u32 {
    e.get(this, PlayerCharacter::pSelectedSpell).addr()
}

// Translated from 0094c8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::SetSelectedSpell` (Xbox PDB): when the spell changes,
/// unloads the old one (`MagicItem::Unload(1)`), stores the new one, preloads
/// it (`MagicItem::Preload(0)`) and resets the cast sound.
pub fn player_character_set_selected_spell(e: &mut Engine, this: Ptr<PlayerCharacter>, spell: u32) {
    let current = e.get(this, PlayerCharacter::pSelectedSpell);
    if current.addr() == spell {
        return;
    }
    if !current.is_null() {
        e.call(MAGIC_ITEM_UNLOAD, &args![current, 1u32]);
    }
    e.set(this, PlayerCharacter::pSelectedSpell, Ptr::new(spell));
    let selected = e.get(this, PlayerCharacter::pSelectedSpell);
    if !selected.is_null() {
        e.call(MAGIC_ITEM_PRELOAD, &args![selected, 0u32]);
    }
    e.call(RESET_MAGIC_CAST_SOUND, &args![this]);
}

// Translated from 0094c950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Selects a scroll (`pSelectedScroll`, `+0x6f0`). A scroll for which
/// `00726070(scroll + 0x74)` gives nothing clears the selection without
/// touching the selected spell. Otherwise a changed selection stores the
/// scroll and selects the spell `00726070(scroll + 0x74) + 0x18` (or none for
/// no scroll); removing the scroll also clears the selected spell first.
pub fn fn_0094c950(e: &mut Engine, this: Ptr<PlayerCharacter>, scroll: Ptr) {
    if !scroll.is_null() && list_next(e, scroll.addr().wrapping_add(0x74)) == 0 {
        e.set(this, PlayerCharacter::pSelectedScroll, Ptr::NULL);
        return;
    }
    let current = e.get(this, PlayerCharacter::pSelectedScroll);
    if !current.is_null() && current == scroll {
        return;
    }
    if !current.is_null() && scroll.is_null() {
        player_character_set_selected_spell(e, this, 0);
    }
    e.set(this, PlayerCharacter::pSelectedScroll, scroll);
    let stored = e.get(this, PlayerCharacter::pSelectedScroll);
    if stored.is_null() {
        player_character_set_selected_spell(e, this, 0);
    } else {
        let spell = list_next(e, stored.addr().wrapping_add(0x74));
        let interface = if spell == 0 {
            0
        } else {
            spell.wrapping_add(0x18)
        };
        player_character_set_selected_spell(e, this, interface);
    }
}

// Translated from 0094ca20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `MagicTarget::AddTarget` (Xbox PDB name of `008230f0`) and answers its `AL`:
/// the exe ends on that call. The function is slot 0 of `PlayerCharacter`'s `MagicTarget`
/// vtable, `MagicTarget::AddTarget` (`bool` in the Xbox PDB).
pub fn fn_0094ca20(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    flag: u8,
) -> bool {
    e.call(
        MAGIC_TARGET_ADD_TARGET,
        &args![this, first, second, third, flag],
    )
    .bool()
}

// Translated from 0094ca50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `this` is the sub-object at `player + 0x88`. Multiplies the value of the
/// player's slot `0x380` (stored as a `double`) by the scale `00567400` gives,
/// rounds the product to a `float`, calls `009a60e0(player_global, product)`
/// (falling back to `00703350()` when that gives 0) and returns slot `0x19c`
/// of the result, or 0 when there is none.
pub fn fn_0094ca50(e: &mut Engine, this: Ptr) -> u32 {
    let player = this.addr().wrapping_sub(0x88);
    let slot_value = e.vcall(player, 0x380, &args![]).f64();
    let scale = e.call(REFERENCE_GET_SCALE, &args![player]).f64();
    let size = (scale * slot_value) as f32;
    let global_player = e.global::<u32>(PLAYER_POINTER);
    let mut found = e
        .call(PLAYER_LOOKUP_BY_SIZE, &args![global_player, size])
        .u32();
    if found == 0 {
        found = e.call(FALLBACK_LOOKUP, &args![]).u32();
    }
    if found != 0 {
        e.vcall(found, 0x19c, &args![]).u32()
    } else {
        0
    }
}

// ---------------------------------------------------------------------
// The parent-cell change handler
// ---------------------------------------------------------------------

/// Stack frame of `0094cae0`: the distances below its 16-byte aligned `EBP` (`SUB ESP,0x25c`
/// under the exception record and `EBX`, then `ESI` and the record link; the music pair is
/// the by-value argument copy built under that, after the pushed 1).
const FRAME_CORNER_A: u32 = 0x1ac;
const FRAME_CORNER_B: u32 = 0x1b4;
const FRAME_CORNER_C: u32 = 0x1bc;
const FRAME_CORNER_D: u32 = 0x1c4;
const FRAME_REGION_LIST: u32 = 0x1cc;
const FRAME_PLAYER_POINT: u32 = 0x1f4;
const FRAME_REGION_VARIABLE: u32 = 0x1d8;
const FRAME_ITEM_VARIABLE: u32 = 0x1ec;
const FRAME_POSITION_COPY: u32 = 0x15c;
const FRAME_DISTANCE_COPY: u32 = 0x234;
const FRAME_OUT_BUFFER: u32 = 0x150;
const FRAME_CINFO: u32 = 0x120;
const FRAME_MUSIC_PAIR: u32 = 0x280;
const FRAME_SIZE: u32 = 0x280;

/// The region data of a region (`009611e0` gives the data list, `004f35b0`
/// finds the entry of the given kind).
fn region_data(e: &mut Engine, region: u32, kind: u32) -> u32 {
    let list = e.call(0x0096_11e0, &args![region]).u32();
    e.call(0x004f_35b0, &args![list, kind]).u32()
}

/// Whether the region data is the kind that `004f1540` answers true for.
fn region_data_flag(e: &mut Engine, data: u32) -> bool {
    e.call(0x004f_1540, &args![data]).bool()
}

/// The priority byte of region data (`005bb4d0`).
fn region_data_priority(e: &mut Engine, data: u32) -> u8 {
    e.call(0x005b_b4d0, &args![data]).u8()
}

// Translated from 0094cae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Handles a change of the player's parent cell: hands the new cell to
/// `00930700`, then rebuilds the player's region bookkeeping from the cell
/// (`008d6f30` gives the parent cell the rest of the function works on). Does
/// nothing when `cell` is already the parent cell.
///
/// - Unless `0094dba0` says otherwise, hands the water height of the cell (or
///   the "no water" constant `-FLT_MAX`) to `0062f8f0`.
/// - Calls `00421e40(extra list of the player, 0)` and `00930700(player,
///   cell)`. For no cell it then clears the list at `+0x774`
///   (`CurrentRegionSoundList`), calls `FalloutAudio::UpdateRegionSounds` and
///   stops.
/// - For a cell it stores `005474b0(cell, 0)` in `pLastKnownMusicType`
///   (`+0x7b4`), passes the cell's `004543c0` object on (`0045cd60`,
///   `004893c0`), calls virtual slot `0x1fc` of the player, adds decals
///   (`DecalCaster::Add`) for the values of `00622bb0` and `00622ba0`, and, when
///   `00822510(+0xdec, 0)` says so, builds the camera body: a `bhkSphereShape`
///   (`fn_0094dbb0`), a `bhkRigidBodyCinfo` and a `bhkRigidBody` stored in
///   `spCameraRigidBody`.
/// - It then walks the cell's region list. For every region that applies and
///   has an entry containing the player's position it records the region in
///   the occupied region list (`+0x764`), collects the sounds of the best
///   region data of kind 7 into `+0x774` and `0x011dd380`, sets
///   `bInBorderContainedCell` (`+0x79c`) when all four corners of the cell are
///   inside, updates the occupied region (`+0x760`, through `0093a7a0`) by
///   the flag and priority of the region data of kind 3, and adds border
///   regions to `pBorderRegions` (`+0x7b0`).
/// - It sets `bReturnToLastKnownGoodPosition` (`+0x79d`) when the border check
///   setting is on, the world space has a border region and no border region
///   was found, and the last known good location is another cell, another
///   world space, or too far from the player's position (2048 units).
/// - Finally it hands the music of the occupied region (kind 4 data, slot
///   `0x2c`) to `00705420`, calls `00947d80` and `ResetMapMarkerStructList`,
///   ends the regions that were left (kind 8 data, `004f2480`) and adds the
///   references of the regions entered to the cell (`AddReference`,
///   `004f24e0`).
///
/// C++ exception unwinding is not translated.
pub fn fn_0094cae0(e: &mut Engine, this: Ptr<PlayerCharacter>, cell: Ptr) {
    let player = this.addr();
    let cell = cell.addr();
    if cell == e.call(PARENT_CELL_OF, &args![player]).u32() {
        return;
    }
    if fn_0094dba0(e) == 0 {
        let water = if cell != 0 && e.call(0x0045_18e0, &args![cell]).bool() {
            e.call(0x0054_71e0, &args![cell]).f32()
        } else {
            e.global::<f32>(NO_WATER_HEIGHT)
        };
        e.call(0x0062_f8f0, &args![water]);
    }
    let extra = e.call(EXTRA_LIST_OF, &args![player]).u32();
    e.call(0x0042_1e40, &args![extra, 0u32]);
    e.call(0x0093_0700, &args![player, cell]);
    if cell == 0 {
        e.call(LIST_CLEAR, &args![player + 0x774]);
        e.call(0x0082_d7c0, &args![]);
        return;
    }

    e.with_stack(FRAME_SIZE + 0x10, |e, raw| {
        let ebp = (raw.addr() + FRAME_SIZE + 0xf) & !0xf;
        let list_local = ebp - FRAME_REGION_LIST;
        let player_point = ebp - FRAME_PLAYER_POINT;
        let region_variable = ebp - FRAME_REGION_VARIABLE;
        let item_variable = ebp - FRAME_ITEM_VARIABLE;

        if !e.call(0x0042_5fd0, &args![cell]).bool() {
            e.call(0x0093_a7a0, &args![player, 0u32]);
        }
        e.call(0x0057_43f0, &args![]);
        let music = e.call(0x0054_74b0, &args![cell, 0u32]).u32();
        e.mem.set_u32(player + 0x7b4, music);
        let interior = e.call(0x0045_43c0, &args![cell]).u32();
        if interior != 0 {
            let lighting = e.call(0x0045_cd60, &args![interior]).u32();
            e.call(0x0048_93c0, &args![lighting, 1u32]);
        }
        e.vcall(player, 0x1fc, &args![0u32]);

        // Decals for the two values `00622bb0` and `00622ba0` give.
        for getter in [0x0062_2bb0u32, 0x0062_2ba0] {
            let current = e.call(PARENT_CELL_OF, &args![player]).u32();
            if current != 0 && {
                let current = e.call(PARENT_CELL_OF, &args![player]).u32();
                e.call(0x0045_43c0, &args![current]).u32() != 0
            } {
                let value = e.call(getter, &args![]).f32();
                if e.call(0x0062_2a70, &args![value, 0u32]).u32() != 0 {
                    let current = e.call(PARENT_CELL_OF, &args![player]).u32();
                    let lighting = e.call(0x0045_43c0, &args![current]).u32();
                    let value = e.call(getter, &args![]).f32();
                    let decal = e.call(0x0062_2a70, &args![value, 1u32]).u32();
                    e.call(0x0062_31a0, &args![decal, lighting]);
                }
            }
        }

        // The camera body.
        if e.call(0x0082_2510, &args![player + 0xdec, 0u32]).bool() {
            let block = e.call(0x00aa_13e0, &args![0x14u32]).u32();
            let sphere = if block != 0 {
                fn_0094dbb0(e, Ptr::new(block)).addr()
            } else {
                0
            };
            let radius = e.global::<f32>(CAMERA_SPHERE_RADIUS);
            e.call(0x0056_ea00, &args![sphere, radius]);
            let cinfo = ebp - FRAME_CINFO;
            e.call(0x00c8_f510, &args![cinfo]);
            e.call(0x0056_f110, &args![cinfo, sphere]);
            let flags: u32 = 0x0003_0000 | 0x21;
            e.call(0x0056_f0f0, &args![cinfo, flags]);
            let body_block = e.call(0x00aa_13e0, &args![0x1cu32]).u32();
            let body = if body_block != 0 {
                e.call(0x0056_d380, &args![body_block, cinfo]).u32()
            } else {
                0
            };
            e.call(SMART_POINTER_ASSIGN, &args![player + 0xdec, body]);
            let body_ptr = e.call(0x0055_9450, &args![player + 0xdec]).u32();
            e.call(0x0056_1580, &args![body_ptr, 1u32]);
            let body_ptr = e.call(0x0055_9450, &args![player + 0xdec]).u32();
            e.vcall(body_ptr, 0xe4, &args![4u32]);
            let body_ptr = e.call(0x0055_9450, &args![player + 0xdec]).u32();
            let motion = e.call(0x004a_e700, &args![body_ptr]).u32();
            fn_0094db80(e, Ptr::new(motion), 9);
            let out_buffer = ebp - FRAME_OUT_BUFFER;
            e.call(LIST_NODE_ITEM_SLOT, &args![out_buffer]);
            let position = e.vcall(player, 0x1f4, &args![]).u32();
            let copy = ebp - FRAME_POSITION_COPY;
            for word in 0..3 {
                let value = e.mem.u32(position + 4 * word);
                e.mem.set_u32(copy + 4 * word, value);
            }
            e.call(0x004a_3e00, &args![out_buffer, copy]);
            let body_ptr = e.call(0x0055_9450, &args![player + 0xdec]).u32();
            e.call(0x0056_10f0, &args![body_ptr, copy]);
            e.call(0x0056_d730, &args![cinfo]);
        }
        let body_ptr = e.call(0x0055_9450, &args![player + 0xdec]).u32();
        let current = e.call(PARENT_CELL_OF, &args![player]).u32();
        let current_interior = e.call(0x0045_43c0, &args![current]).u32();
        e.vcall(body_ptr, 0x9c, &args![current_interior]);

        // Whether the player is in a state that keeps the sky as it is.
        let mut flag = e.call(0x0043_7bd0, &args![player]).bool();
        if !flag {
            flag = e.vcall(player, 0x22c, &args![0u32]).bool();
        }
        if !flag {
            flag = e.vcall(player, 0x2e8, &args![]).bool();
        }
        if !flag {
            flag = e.vcall(player, 0x230, &args![]).bool();
        }
        let reset = !flag && !e.call(0x005a_1e50, &args![player]).bool();
        if reset {
            e.vcall(player, 0x494, &args![]);
        }
        let loader = e.global::<u32>(MODEL_LOADER_POINTER);
        e.call(0x0044_6b50, &args![loader, 0u32]);
        let climate = e.call(0x0054_75b0, &args![cell]).u32();
        let sky = e.call(0x0046_dd00, &args![]).u32();
        e.call(0x0063_c8f0, &args![sky, climate, 0u32]);
        let weather = e.call(0x0054_7680, &args![cell]).u32();
        let weather = e.call(0x0050_0940, &args![weather]).u32();
        e.call(0x00b4_f430, &args![weather]);
        let world_space = e.call(0x0054_ddd0, &args![cell]).u32();
        let mut found_border = false;
        e.mem.set_u8(player + 0x79c, 0);

        let border_regions = e.mem.u32(player + 0x7b0);
        if border_regions != 0 {
            e.call(0x005e_03d0, &args![border_regions]);
            let border_regions = e.mem.u32(player + 0x7b0);
            if border_regions != 0 {
                e.vcall(border_regions, 0, &args![1u32]);
            }
            e.mem.set_u32(player + 0x7b0, 0);
        }

        // The four corners of the cell.
        let cell_x = e.call(0x0054_4c30, &args![cell]).i32();
        let cell_y = e.call(0x0054_4c60, &args![cell]).i32();
        let cell_size: f64 = e.global(CELL_SIZE);
        let west = (cell_x << 12) as f32;
        let east = (west as f64 + cell_size) as f32;
        let south = (cell_y << 12) as f32;
        let north = (south as f64 + cell_size) as f32;
        for (corner, x, y) in [
            (FRAME_CORNER_A, west, south),
            (FRAME_CORNER_B, west, north),
            (FRAME_CORNER_C, east, south),
            (FRAME_CORNER_D, east, north),
        ] {
            e.call(0x004f_7070, &args![ebp - corner, x, y]);
        }

        e.call(LIST_CLEAR, &args![player + 0x774]);
        e.set_global(CURRENT_REGION_WITH_SOUNDS, 0u32);
        e.call(0x0096_a2d0, &args![list_local]);

        // The regions the player was in.
        let mut node = if player.wrapping_add(0x764) == 0 {
            0
        } else {
            player + 0x768
        };
        while node != 0 {
            let slot = list_item_slot(e, node);
            if e.mem.u32(slot) == 0 {
                break;
            }
            let slot = list_item_slot(e, node);
            let region = e.mem.u32(slot);
            e.mem.set_u32(region_variable, region);
            if region != 0 {
                e.call(0x005a_e3d0, &args![list_local, region_variable]);
            }
            node = list_next(e, node);
        }
        e.call(LIST_CLEAR, &args![player + 0x768]);

        // The regions of the new cell.
        let mut best = 0u32;
        let region_list = e.call(0x0054_7110, &args![cell, 1u32]).u32();
        let mut region_node = if region_list != 0 { region_list + 4 } else { 0 };
        while region_node != 0 {
            let slot = list_item_slot(e, region_node);
            let item = e.mem.u32(slot);
            if item == 0 {
                break;
            }
            e.mem.set_u32(item_variable, item);
            let position = e.call(0x0089_1170, &args![player]).u32();
            e.call(0x004f_7030, &args![player_point, position + 0x10]);

            let skip = e.call(0x0044_0d80, &args![item]).bool() || {
                let form = e.call(0x007a_f430, &args![item]).u32();
                let wrong_world = form != 0 && {
                    let again = e.call(0x007a_f430, &args![item]).u32();
                    let world = e.call(0x0057_5d70, &args![player]).u32();
                    again != world
                };
                wrong_world || {
                    let regions = e.call(0x0044_1110, &args![item]).u32();
                    regions == 0 || {
                        let regions = e.call(0x0044_1110, &args![item]).u32();
                        list_is_empty(e, regions)
                    }
                }
            };
            if skip {
                region_node = list_next(e, region_node);
                continue;
            }

            let mut done = false;
            let mut entry_node = e.call(0x0044_1110, &args![item]).u32();
            while !done && entry_node != 0 {
                let slot = list_item_slot(e, entry_node);
                let entry = e.mem.u32(slot);
                if entry == 0 {
                    break;
                }
                if e.call(0x004f_8360, &args![entry, player_point]).bool() {
                    e.call(0x004f_6600, &args![player + 0x764, item]);
                    if e.call(0x005f_65d0, &args![list_local, item_variable])
                        .bool()
                    {
                        e.call(0x0090_5330, &args![list_local, item_variable]);
                    }
                    let data = region_data(e, item, 7);
                    if data != 0 {
                        if region_data_flag(e, data) {
                            let replace = if best == 0 {
                                true
                            } else if best == data {
                                false
                            } else {
                                let old = region_data_priority(e, best);
                                let new = region_data_priority(e, data);
                                old < new
                            };
                            if replace {
                                e.call(LIST_CLEAR, &args![player + 0x774]);
                                best = data;
                            }
                        }
                        if best == 0 || best == data {
                            let mut sound_node = e.call(0x0048_d150, &args![data]).u32();
                            while sound_node != 0 {
                                let slot = list_item_slot(e, sound_node);
                                if e.mem.u32(slot) == 0 {
                                    break;
                                }
                                let slot = list_item_slot(e, sound_node);
                                e.call(0x005a_e3d0, &args![player + 0x774, slot]);
                                sound_node = list_next(e, sound_node);
                            }
                            if e.vcall(data, 0x30, &args![]).u32() != 0 {
                                e.set_global(CURRENT_REGION_WITH_SOUNDS, data);
                            }
                        }
                    }
                    if e.call(0x0054_9580, &args![item]).bool() {
                        found_border = true;
                        let corners = [
                            FRAME_CORNER_A,
                            FRAME_CORNER_B,
                            FRAME_CORNER_C,
                            FRAME_CORNER_D,
                        ];
                        let mut all = true;
                        for corner in corners {
                            if !e.call(0x004f_8360, &args![entry, ebp - corner]).bool() {
                                all = false;
                                break;
                            }
                        }
                        if all {
                            e.mem.set_u8(player + 0x79c, 1);
                        }
                    }
                    let occupied = e.mem.u32(player + 0x760);
                    if occupied != 0 {
                        let occupied_data = region_data(e, occupied, 3);
                        let new_data = region_data(e, item, 3);
                        if new_data != 0 {
                            let mut change = false;
                            if occupied_data == 0 {
                                change = true;
                            } else {
                                let new_flag = region_data_flag(e, new_data);
                                if new_flag && !region_data_flag(e, occupied_data) {
                                    change = true;
                                } else {
                                    let new_flag = region_data_flag(e, new_data);
                                    let old_flag = region_data_flag(e, occupied_data);
                                    if new_flag == old_flag {
                                        let new_priority = region_data_priority(e, new_data);
                                        let old_priority = region_data_priority(e, occupied_data);
                                        if new_priority > old_priority {
                                            change = true;
                                        }
                                    }
                                }
                            }
                            if change {
                                e.call(0x0093_a7a0, &args![player, item]);
                            }
                        }
                    } else {
                        e.call(0x0093_a7a0, &args![player, item]);
                    }
                    done = true;
                }
                entry_node = list_next(e, entry_node);
            }
            if e.call(0x0054_9580, &args![item]).bool() {
                if e.mem.u32(player + 0x7b0) == 0 {
                    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
                    let array = if block != 0 {
                        e.call(0x0096_a640, &args![block, 1u32, 1u32]).u32()
                    } else {
                        0
                    };
                    e.mem.set_u32(player + 0x7b0, array);
                }
                let array = e.mem.u32(player + 0x7b0);
                e.call(0x0096_a610, &args![array, item_variable]);
            }
            region_node = list_next(e, region_node);
        }

        // The border check.
        let border_enabled = {
            let at = e.call(0x0040_8d60, &args![BORDER_REGION_SETTING]).u32();
            e.mem.u8(at) != 0
        };
        if border_enabled
            && world_space != 0
            && e.call(0x0058_6260, &args![world_space]).bool()
            && !{
                let holder = e.global::<u32>(SKY_HOLDER_POINTER);
                e.call(0x0045_1530, &args![holder]).bool()
            }
            && !found_border
        {
            let mut exterior_cell = 0u32;
            let mut world = 0u32;
            let location = e.mem.u32(player + 0x7ac);
            if location != 0 {
                let kind = e.call(FORM_TYPE_OF, &args![location]).i32();
                if kind == 0x39 {
                    exterior_cell = location;
                } else if kind == 0x41 {
                    world = location;
                }
            }
            let left_exterior = exterior_cell != 0 && exterior_cell != cell;
            let other_world =
                !left_exterior && world != 0 && world != e.call(0x0054_ddd0, &args![cell]).u32();
            if left_exterior || other_world {
                e.mem.set_u8(player + 0x79d, 1);
            } else {
                let position = e.vcall(player, 0x1f4, &args![]).u32();
                let copy = ebp - FRAME_DISTANCE_COPY;
                for word in 0..3 {
                    let value = e.mem.u32(position + 4 * word);
                    e.mem.set_u32(copy + 4 * word, value);
                }
                e.call(0x0045_78c0, &args![copy, player + 0x7a0]);
                let distance = e.call(0x0045_7990, &args![copy]).f64();
                let limit: f64 = e.global(BORDER_DISTANCE_LIMIT);
                if distance > limit {
                    e.mem.set_u8(player + 0x79d, 1);
                }
            }
        }

        // The music type of the occupied region.
        let occupied = e.mem.u32(player + 0x760);
        let music_data = if occupied != 0 {
            region_data(e, occupied, 4)
        } else {
            0
        };
        let music_value = if music_data != 0 {
            e.vcall(music_data, 0x2c, &args![]).u32()
        } else {
            0
        };
        let pair = ebp - FRAME_MUSIC_PAIR;
        e.call(0x0040_c0e0, &args![pair, music_value]);
        let first = e.mem.u32(pair);
        let second = e.mem.u32(pair + 4);
        let occupied = e.mem.u32(player + 0x760);
        e.call(0x0070_5420, &args![occupied, first, second, 1u32]);
        e.call(0x0094_7d80, &args![player]);
        e.call(0x0094_80c0, &args![player]);

        // The regions that were left.
        let mut node = list_local;
        while node != 0 {
            let slot = list_item_slot(e, node);
            if e.mem.u32(slot) == 0 {
                break;
            }
            let slot = list_item_slot(e, node);
            let region = e.mem.u32(slot);
            if region != 0 {
                let data = region_data(e, region, 8);
                if data != 0 {
                    e.call(0x004f_2480, &args![data, 0u32, 0u32]);
                }
            }
            node = list_next(e, node);
        }

        // The regions entered: add their references to the cell.
        let mut node = if player.wrapping_add(0x764) == 0 {
            0
        } else {
            player + 0x768
        };
        while node != 0 {
            let slot = list_item_slot(e, node);
            if e.mem.u32(slot) == 0 {
                break;
            }
            let slot = list_item_slot(e, node);
            let region = e.mem.u32(slot);
            if region != 0 {
                let data = region_data(e, region, 8);
                if data != 0 {
                    let mut reference_node = e.call(0x0041_3f40, &args![data]).u32();
                    while reference_node != 0 {
                        let slot = list_item_slot(e, reference_node);
                        if e.mem.u32(slot) != 0 {
                            let slot = list_item_slot(e, reference_node);
                            let reference = e.mem.u32(slot);
                            let parent = e.call(PARENT_CELL_OF, &args![player]).u32();
                            e.call(0x0054_8230, &args![parent, reference, 0u32]);
                            let slot = list_item_slot(e, reference_node);
                            let reference = e.mem.u32(slot);
                            e.call(0x004f_24e0, &args![reference, 1u32, 0u32]);
                        }
                        reference_node = list_next(e, reference_node);
                    }
                }
            }
            node = list_next(e, node);
        }
        e.call(LIST_CLEAR, &args![list_local]);
        e.call(0x0046_ffb0, &args![list_local]);
    });
}

// Translated from 0094db80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte at `+0x1a` of the object (`0094cae0` calls it with 9 on the
/// object `004ae700` gives for the camera body).
pub fn fn_0094db80(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x1a), value);
}

// Translated from 0094dba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `0x011c7a5a`.
pub fn fn_0094dba0(e: &mut Engine) -> u8 {
    e.global::<u8>(SKIP_WATER_HEIGHT_FLAG)
}

// Translated from 0094dbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a `bhkSphereShape` (its RTTI is the one of the vtable it
/// installs, `0x01030cdc`): the base constructor `0056e690`, the vtable, and
/// the instance counter at `0x01268280` incremented. Returns `this`.
pub fn fn_0094dbb0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(SPHERE_BASE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), 0x0103_0cdc);
    let count: u32 = e.global(0x0126_8280);
    e.set_global(0x0126_8280, count.wrapping_add(1));
    this
}

// Translated from 0094dbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::ReturnToLastKnownGoodPosition` (Xbox PDB): does nothing
/// without a last known good location (`+0x7ac`). It shows the "sad vault
/// boy" message (`007052f0`), then
/// - with `use_position_functions`: moves the player to that location with
///   `PositionPlayer` (`0093c200`, for a cell) or `PositionPlayerExterior`
///   (`0093cce0`, for a world space), using the position `00430830` gives for
///   it and the stored position (`fn_0094df30`);
/// - otherwise: sets the position directly (the water-height check on the
///   character controller, `SetLocationOnReference`, the controller's
///   `SetPosition`), re-parents the player into the cell found from the
///   location (virtual slot `0x228` of the player, twice), resets the
///   collision objects of both biped bodies and zeroes their velocities.
pub fn player_character_return_to_last_known_good_position(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    use_position_functions: bool,
) {
    let player = this.addr();
    if e.get(this, PlayerCharacter::pLastKnownGoodLocation)
        .is_null()
    {
        return;
    }
    let message = setting_text(e, RETURN_MESSAGE_SETTING);
    let seconds = e.global::<f32>(MESSAGE_SECONDS);
    e.call(
        0x0070_52f0,
        &args![message, 0u32, SAD_ICON_PATH, 0u32, seconds, 0u32],
    );

    let mut cell = 0u32;
    let mut world = 0u32;
    let location = e.get(this, PlayerCharacter::pLastKnownGoodLocation).addr();
    if location != 0 {
        let kind = e.call(FORM_TYPE_OF, &args![location]).i32();
        if kind == 0x39 {
            cell = location;
        } else if kind == 0x41 {
            world = location;
        }
    }

    if use_position_functions {
        // The exe's frame (`SUB ESP,0x64` under `EBP`) keeps the stored position at
        // `EBP-0x4c` (+0x18) on the world path and at `EBP-0x58` (+0xc) on the cell path.
        let (target, function, stored_at) = if world != 0 {
            (world, 0x0093_cce0u32, 0x18)
        } else if cell != 0 {
            (cell, 0x0093_c200u32, 0xc)
        } else {
            return;
        };
        let position = e.call(0x0043_0830, &args![player]).u32();
        let wanted = [
            e.mem.u32(position),
            e.mem.u32(position + 4),
            e.mem.u32(position + 8),
        ];
        let known = e.with_stack(0x64, |e, frame| {
            let stored = frame.addr() + stored_at;
            let known = fn_0094df30(e, this, Ptr::new(stored));
            [
                e.mem.u32(known.addr()),
                e.mem.u32(known.addr() + 4),
                e.mem.u32(known.addr() + 8),
            ]
        });
        e.call(
            function,
            &args![
                player, known[0], known[1], known[2], wanted[0], wanted[1], wanted[2], target, 0u32
            ],
        );
        return;
    }

    let controller = e.call(0x0093_06d0, &args![player]).u32();
    if controller != 0 && !e.call(0x0088_b0c0, &args![controller]).bool() {
        let reference = e.call(0x0043_6aa0, &args![player]).u32();
        let height = e.mem.u32(reference + 8);
        e.mem.set_u32(player + 0x7a8, height);
    }
    e.call(0x0057_5830, &args![player, player + 0x7a0]);
    if controller != 0 {
        e.call(0x0056_20e0, &args![controller, player + 0x7a0]);
    }
    let mut target_cell = 0u32;
    let mut target_world = 0u32;
    let location = e.get(this, PlayerCharacter::pLastKnownGoodLocation).addr();
    if location != 0 {
        let kind = e.call(FORM_TYPE_OF, &args![location]).i32();
        if kind == 0x39 {
            target_cell = location;
        } else if kind == 0x41 {
            target_world = location;
        }
    }
    if target_world != 0 {
        let x = e.mem.f32(player + 0x7a0);
        let x = e.call(0x0040_6d90, &args![x]).i32() >> 12;
        let y = e.mem.f32(player + 0x7a4);
        let y = e.call(0x0040_6d90, &args![y]).i32() >> 12;
        let source = e.global::<u32>(0x011c_3f2c);
        target_cell = e
            .call(0x0046_1c20, &args![source, x, y, target_world, 0u32])
            .u32();
    }
    if target_cell != 0 {
        e.vcall(player, 0x228, &args![target_cell]);
        e.vcall(player, 0x228, &args![target_cell]);
    }
    let bodies = [
        e.call(0x0095_0bb0, &args![player, 0u32]).u32(),
        e.call(0x0095_0bb0, &args![player, 1u32]).u32(),
    ];
    for body in bodies {
        e.call(0x0044_0460, &args![body, player + 0x7a0]);
    }
    for body in bodies {
        e.call(0x00c6_bd00, &args![body, 1u32]);
    }
    // The same frame: the zero vectors at `EBP-0x20` (+0x44) and `EBP-0x14` (+0x50).
    e.with_stack(0x64, |e, frame| {
        for (body, zero_at) in bodies.into_iter().zip([0x44, 0x50]) {
            let zero = frame.addr() + zero_at;
            e.call(0x0043_d410, &args![zero, 0.0f32, 0u32, 0u32]);
            e.call(0x00a5_9c60, &args![body, zero]);
        }
    });
}

// Translated from 0094df30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the three words at `+0x7a0` (`LastKnownGoodPosition`) into `out` and
/// returns `out`.
pub fn fn_0094df30(e: &mut Engine, this: Ptr<PlayerCharacter>, out: Ptr) -> Ptr {
    let source = this.addr().wrapping_add(0x7a0);
    for word in 0..3 {
        let value = e.mem.u32(source + 4 * word);
        e.mem.set_u32(out.addr() + 4 * word, value);
    }
    out
}

// Translated from 0094df60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::IsSleepingorResting` (Xbox PDB): true while `iSleepTime`
/// (`+0x654`) is positive.
pub fn player_character_is_sleepingor_resting(e: &mut Engine, this: Ptr<PlayerCharacter>) -> bool {
    e.get(this, PlayerCharacter::iSleepTime) > 0
}

// Translated from 0094df80 (decompiled, FalloutNV.exe 1.4.0.525)
/// One hour of waiting or sleeping: ends all sounds of type 4, interrupts the
/// magic cast, advances the system clock and the calendar by
/// `3600 / time scale` seconds, and (while `0x011a3b30` is set) updates the
/// magic target or the process lists, restores health and, for a sleeping
/// player, applies the actor value `0x4b` change; then updates the sky and the
/// process lists and the magic, and counts `iSleepTime` down. When it reaches
/// 0 the wait is over: the flag at `0x011a3b30` is set and a sleeping player
/// is fully restored.
pub fn fn_0094df80(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let player = this.addr();
    let audio = e.call(0x0045_3a70, &args![]).u32();
    e.call(0x00ad_8780, &args![audio, 4u32]);
    e.call(0x0081_5b00, &args![player + MAGIC_CASTER]);
    let time_scale = e.call(0x0086_7950, &args![CALENDAR]).f64();
    let seconds_per_hour: f64 = e.global(SECONDS_PER_HOUR);
    let step = (seconds_per_hour / time_scale) as f32;
    let clock = e.call(0x0096_d490, &args![PROCESS_LISTS]).f64();
    let new_clock = (clock + step as f64) as f32;
    e.call(0x0096_d4b0, &args![PROCESS_LISTS, new_clock]);
    e.call(0x0086_7a40, &args![CALENDAR, step]);

    if e.global::<u8>(WAIT_STEP_ACTIVE) != 0 {
        if !e.call(0x005c_7870, &args![player]).bool() {
            if e.global::<u8>(PROCESS_LOCK_ENABLED) != 0 {
                e.call(0x0040_fbf0, &args![PROCESS_LOCK, 0u32]);
                let time = e.global::<f32>(WAIT_UPDATE_TIME);
                for function in [0x0096_bcd0u32, 0x0096_b810, 0x0096_b470, 0x0096_b050] {
                    e.call(function, &args![PROCESS_LISTS, time, 1u32]);
                }
                e.call(0x0040_fba0, &args![PROCESS_LOCK]);
            }
        } else {
            e.call(0x0082_3c40, &args![player + MAGIC_TARGET, step]);
        }
        let global_player = e.global::<u32>(PLAYER_POINTER);
        e.call(0x0088_b510, &args![global_player, step]);
        if e.call(0x004d_1360, &args![player]).bool() {
            let global_player = e.global::<u32>(PLAYER_POINTER);
            if e.call(0x005a_1e50, &args![global_player]).bool() {
                let value = e
                    .vcall(player + ACTOR_VALUE_OWNER, 0xc, &args![0x4bu32])
                    .f32();
                let at = e
                    .call(SETTING_FLOAT_POINTER, &args![ACTOR_VALUE_4B_LIMIT_SETTING])
                    .u32();
                let limit = e.mem.f32(at);
                let change = if value < limit { value } else { limit };
                e.vcall(player, 0x3a4, &args![0x4bu32, -change, 0u32]);
            }
        }
    }

    let source = e.call(0x0084_d030, &args![TIMER_SOURCE_011F6394]).f32();
    let holder = e.global::<u32>(SKY_HOLDER_POINTER);
    let sky = e.call(0x008d_8520, &args![holder]).u32();
    e.call(0x0063_ac70, &args![sky, source]);
    e.call(0x0097_23f0, &args![PROCESS_LISTS]);
    e.call(0x008c_3c40, &args![player, 1u32, 0u32]);
    let remaining = e.get(this, PlayerCharacter::iSleepTime).wrapping_sub(1);
    e.set(this, PlayerCharacter::iSleepTime, remaining);
    if remaining <= 0 {
        e.set_global(WAIT_STEP_ACTIVE, 1u8);
        if e.get(this, PlayerCharacter::bIsSleeping) && !e.call(0x004d_1360, &args![player]).bool()
        {
            e.call(0x008a_0960, &args![player]);
        }
        e.set(this, PlayerCharacter::bIsSleeping, false);
    }
}

// Translated from 0094e1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::Load3D` (Xbox PDB): loads the player's 3D.
///
/// - Opens the debug scope (`00404eb0`, size `0x34`, line `0x2458` of
///   `PlayerCharacter.cpp`) and closes it at the end.
/// - Unless `0042ce10(load state)` or the byte at `0x011d8907` say otherwise,
///   looks for a worn item in the 20 slots of the inventory changes; with
///   none it calls `006047c0` on the base form, and with items but none worn
///   in slot 6 it creates the default item for slot 6 (`004c8220`) and
///   equips it (virtual slot `0x184`).
/// - Loads the model named by the model-path setting (`ModelLoader::LoadFile`),
///   stores it in `sp1stPerson3D`, finds the bone root and the two camera
///   nodes by name (logging an error for a missing one), positions the model,
///   builds the `BipedAnim` and `Animation` objects (stored at `+0x68c` and
///   `+0x690`), runs the base `0087e060` with `b3rdPerson` set, clears the
///   animation group, builds the animation file list (the model's `.kf` list
///   plus the locomotion idle animations for toddlers and for the sex of the
///   player), starts the animation, then updates the third person camera
///   node, process, lighting and character-controller proxy and finally
///   updates the biped bodies. Returns the node `0087e060` returned.
///
/// C++ exception unwinding and the stack cookie are not translated.
pub fn player_character_load_3d(e: &mut Engine, this: Ptr<PlayerCharacter>, flag: u8) -> u32 {
    let player = this.addr();
    // The exe's frame (`SUB ESP,0x204` under the exception record, then `ESI`, `EDI` and the
    // record link): the scope at `EBP-0x40` (+0x1dc), the zero vector at `EBP-0x58` (+0x1c4),
    // the matrix copy at `EBP-0x3c` (+0x1e0), the rotated matrix at `EBP-0x1c8` (+0x54), the
    // path buffer at `EBP-0x16c` (+0xb0) and the built word at `EBP-0x1dc` (+0x40).
    e.with_stack(0x21c, |e, frame| {
        let scope = frame.addr() + 0x1dc;
        e.call(
            0x0040_4eb0,
            &args![scope, 0x34u32, 1u32, SOURCE_FILE_NAME, 0x2458u32],
        );
        let form = e.call(0x007a_f430, &args![player]).u32();
        let load_state = e.global::<u32>(LOAD_STATE_POINTER);
        let skip =
            e.call(0x0042_ce10, &args![load_state]).bool() || e.global::<u8>(0x011d_8907) != 0;
        if !skip {
            let mut nothing_worn = true;
            let inventory = e.call(0x004b_f220, &args![player]).u32();
            if inventory != 0 {
                for slot in 0..0x14u32 {
                    let worn = e.call(0x004c_8c10, &args![inventory, slot, 0u32]).u32();
                    if worn != 0 {
                        nothing_worn = false;
                        e.call(0x0044_59e0, &args![worn, 1u32]);
                        break;
                    }
                }
            }
            if nothing_worn {
                e.call(0x0060_47c0, &args![form, player, 1u32, 1u32, 0u32, 0u32]);
            } else if inventory != 0 {
                let worn = e.call(0x004c_8c10, &args![inventory, 6u32, 0u32]).u32();
                if worn == 0 {
                    let base = e.call(BASE_FORM_OF, &args![player]).u32();
                    let item = e
                        .call(0x004c_8220, &args![inventory, base, 6u32, 1u32])
                        .u32();
                    if item != 0 {
                        let name = e.call(WORD_AT_8, &args![item]).u32();
                        let mut extra = 0;
                        if e.call(0x0055_9450, &args![item]).u32() != 0 {
                            let pointer = e.call(0x0055_9450, &args![item]).u32();
                            let slot = list_item_slot(e, pointer);
                            extra = e.mem.u32(slot);
                        }
                        e.vcall(player, 0x184, &args![name, 1u32, extra, 0u32]);
                        e.call(0x0044_59e0, &args![item, 1u32]);
                    }
                } else {
                    e.call(0x0044_59e0, &args![worn, 1u32]);
                }
            }
        }

        let path = setting_text(e, MODEL_PATH_SETTING);
        let path = e.call(IDENTITY, &args![path, 0u32]).u32();
        let loader = e.global::<u32>(MODEL_LOADER_POINTER);
        let model = e
            .call(
                0x0044_7080,
                &args![loader, path, 3u32, 1u32, 0u32, 0u32, 0u32],
            )
            .u32();
        e.call(0x0054_68b0, &args![model, 1u32]);
        e.call(SMART_POINTER_ASSIGN, &args![player + 0x694, model]);
        for (name, slot) in [
            (fn_0094ead0(e), NODE_BY_EAD0_SLOT),
            (fn_0094eae0(e), NODE_CAMERA_1ST_SLOT),
            (fn_0094eac0(e), NODE_BIP_SLOT),
        ] {
            let node = e.call(0x0055_9450, &args![player + 0x694]).u32();
            let found = e.vcall(node, 0x9c, &args![name]).u32();
            e.set_global(slot, found);
        }
        if e.global::<u32>(NODE_CAMERA_1ST_SLOT) == 0 {
            let text = setting_text(e, MODEL_PATH_SETTING);
            e.call(ERROR, &args![FORMAT_MISSING_CAMERA_1ST, text]);
        }
        if e.global::<u32>(NODE_BIP_SLOT) == 0 {
            let text = setting_text(e, MODEL_PATH_SETTING);
            e.call(ERROR, &args![FORMAT_MISSING_BIP, text]);
        }
        let zero = frame.addr() + 0x1c4;
        e.call(0x0043_d410, &args![zero, 0.0f32, 0u32, 0u32]);
        e.call(0x00a5_9c60, &args![model, zero]);
        let camera = e.global::<u32>(NODE_CAMERA_1ST_SLOT);
        let at = e.call(0x0045_bb80, &args![camera]).u32();
        let height = e.mem.f32(at + 8);
        e.set(this, PlayerCharacter::fEyeHeight, height);

        let matrix_copy = frame.addr() + 0x1e0;
        let matrix_result = frame.addr() + 0x54;
        for word in 0..9u32 {
            let value = e.mem.u32(LOAD_3D_MATRIX + 4 * word);
            e.mem.set_u32(matrix_copy + 4 * word, value);
        }
        let rotated = e
            .call(0x0056_fac0, &args![player, matrix_result, matrix_copy])
            .u32();
        e.call(0x0043_fa80, &args![model, rotated]);

        let block = e.call(OPERATOR_NEW, &args![0x2b4u32]).u32();
        let biped = if block != 0 {
            e.call(0x004a_aca0, &args![block, player, model]).u32()
        } else {
            0
        };
        e.set(this, PlayerCharacter::p1stPersonBipedAnim, Ptr::new(biped));
        let block = e.call(OPERATOR_NEW, &args![0x13cu32]).u32();
        let animation = if block != 0 {
            e.call(0x0048_f810, &args![block]).u32()
        } else {
            0
        };
        e.set(
            this,
            PlayerCharacter::p1stPersonAnimation,
            Ptr::new(animation),
        );

        let third_person = e.get(this, PlayerCharacter::b3rdPerson);
        e.set(this, PlayerCharacter::b3rdPerson, true);
        let node = e.call(0x0087_e060, &args![player, flag]).u32();
        e.set(this, PlayerCharacter::b3rdPerson, third_person);

        let current = e.call(0x0095_0a60, &args![player, 0u32]).u32();
        e.call(0x0049_6080, &args![current, 0u32, 0.0f32]);

        let path = setting_text(e, MODEL_PATH_SETTING);
        let path = e.call(IDENTITY, &args![path, 0u32]).u32();
        let loader = e.global::<u32>(MODEL_LOADER_POINTER);
        let list = e
            .call(0x0044_7330, &args![loader, path, 1u32, 0u32, 0xcu32])
            .u32();
        let path = setting_text(e, MODEL_PATH_SETTING);
        let path = e.call(IDENTITY, &args![path, 0u32]).u32();
        let buffer = frame.addr() + 0xb0;
        e.call(0x0040_6d30, &args![buffer, 0x104u32, path]);
        let last_separator = e.call(0x0040_ab30, &args![buffer, 0x5cu32]).u32();
        let room = 0x103u32.wrapping_sub(last_separator.wrapping_sub(buffer));
        let idle_manager = e.global::<u32>(IDLE_MANAGER_POINTER);
        if e.call(0x0083_97d0, &args![player]).bool() {
            e.call(0x0040_6d30, &args![last_separator, room, TODDLER_SUFFIX]);
            let root = e
                .call(0x0060_0700, &args![idle_manager, buffer, 0u32])
                .u32();
            let loader = e.global::<u32>(MODEL_LOADER_POINTER);
            e.call(0x0044_7850, &args![loader, root, list]);
        }
        let suffix = if e.call(0x0087_f4c0, &args![player]).i32() == 1 {
            FEMALE_SUFFIX
        } else {
            MALE_SUFFIX
        };
        e.call(0x0040_6d30, &args![last_separator, room, suffix]);
        let root = e
            .call(0x0060_0700, &args![idle_manager, buffer, 0u32])
            .u32();
        let loader = e.global::<u32>(MODEL_LOADER_POINTER);
        e.call(0x0044_7850, &args![loader, root, list]);

        let animation = e.get(this, PlayerCharacter::p1stPersonAnimation);
        e.call(0x0048_ffd0, &args![animation, list, model, player, 1u32]);
        let first = e.call(0x008d_8520, &args![player]).u32();
        let second = e.call(0x008d_8520, &args![player]).u32();
        // 0094e8ee..0094e91d: the two 0s are pushed before GetCurrentWeapon (slot 0x148, no
        // stack words); they are SetCurrentWeapon's (slot 0x160) group and force.
        let result = e.vcall(first, 0x148, &args![]).u32();
        e.vcall(second, 0x160, &args![result, 0u32, 0u32]);
        fn_0094eaf0(e, this.cast(), Ptr::new(node), true);
        if e.global::<u32>(NODE_CAMERA_3RD_SLOT) == 0 {
            let name = e.call(0x0057_15d0, &args![player]).u32();
            e.call(ERROR, &args![FORMAT_MISSING_CAMERA_3RD, name]);
        }
        e.call(0x008c_2e50, &args![player]);
        let process = process_of(e, this);
        e.vcall(process, 0x470, &args![]);
        e.call(0x008b_0bd0, &args![player, model]);
        e.vcall(player, 0x1c4, &args![]);
        let controller = e.call(0x0093_06d0, &args![player]).u32();
        let proxy = if controller != 0 {
            e.call(0x0062_1ad0, &args![controller]).u32()
        } else {
            0
        };
        if proxy != 0 {
            let out = frame.addr() + 0x40;
            let built = e.call(0x0070_c440, &args![controller, out]).u32();
            let value = e.call(0x004a_3a20, &args![built]).u32();
            e.vcall(proxy, 0xd0, &args![node, 1u32, 0u32, value, 0u32]);
        }
        e.call(0x0070_59d0, &args![]);
        if e.get(this, PlayerCharacter::bIsToddler) {
            e.set(this, PlayerCharacter::bIsToddler, false);
            e.call(0x0096_97c0, &args![player, 1u32]);
        }
        for which in 0..2u32 {
            let biped = e.call(0x0095_0b00, &args![player, which]).u32();
            let pointer = e.call(0x0055_9450, &args![biped]).u32();
            e.call(0x0093_80e0, &args![pointer]);
        }
        e.call(0x0040_4ee0, &args![scope]);
        node
    })
}

// Translated from 0094eac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `0x011c61a8`.
pub fn fn_0094eac0(e: &mut Engine) -> u32 {
    e.global::<u32>(NODE_NAME_EAC0)
}

// Translated from 0094ead0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `0x011c6264`.
pub fn fn_0094ead0(e: &mut Engine) -> u32 {
    e.global::<u32>(NODE_NAME_EAD0)
}

// Translated from 0094eae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `0x011c626c`.
pub fn fn_0094eae0(e: &mut Engine) -> u32 {
    e.global::<u32>(NODE_NAME_EAE0)
}

// Translated from 0094eaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks up the third person camera node: with `flag`, first calls `00450f90`
/// on `object` (with 1); then stores `object`'s slot `0x9c` with the word of
/// `0094eb30` in the global `0x011e07d4`. `this` is not used.
pub fn fn_0094eaf0(e: &mut Engine, _this: Ptr, object: Ptr, flag: bool) {
    if flag {
        e.call(0x0045_0f90, &args![object, 1u32]);
    }
    let name = fn_0094eb30(e);
    let found = e.vcall(object.addr(), 0x9c, &args![name]).u32();
    e.set_global(NODE_CAMERA_3RD_SLOT, found);
}

// Translated from 0094eb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `0x011c6270`.
pub fn fn_0094eb30(e: &mut Engine) -> u32 {
    e.global::<u32>(NODE_NAME_EB30)
}

// Translated from 0094eb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `TESObjectREFR::Set3D` (Xbox PDB name of `005702e0`) with the node and
/// the flag. Setting no node while the game state object (`0x011dea0c`) exists
/// and both tests `005dc960` and `00678ce0` fail is refused with a log line.
/// Setting no node otherwise releases the 1st-person `BipedAnim` and
/// `Animation` (`+0x68c`, `+0x690`), clears `sp1stPerson3D`, the three camera
/// node globals and the `0x60` bytes at `+0xd74`, and tells the model loader.
pub fn fn_0094eb40(e: &mut Engine, this: Ptr<PlayerCharacter>, node: Ptr, flag: u8) {
    let player = this.addr();
    let state = e.global::<u32>(GAME_STATE_POINTER);
    if state != 0
        && !e.call(GAME_STATE_TEST_1, &args![state]).bool()
        && !e.call(GAME_STATE_TEST_2, &args![state]).bool()
        && node.is_null()
    {
        e.call(LOG_MESSAGE, &args![FORMAT_SET3D_MESSAGE]);
        return;
    }
    e.call(REFERENCE_SET_3D, &args![player, node, flag]);
    if !node.is_null() {
        return;
    }
    let biped = e.get(this, PlayerCharacter::p1stPersonBipedAnim);
    if !biped.is_null() {
        e.call(DELETE_BIPED_ANIM, &args![biped, 1u32]);
    }
    e.set(this, PlayerCharacter::p1stPersonBipedAnim, Ptr::NULL);
    let animation = e.get(this, PlayerCharacter::p1stPersonAnimation);
    if !animation.is_null() {
        e.call(DELETE_ANIMATION, &args![animation, 1u32]);
    }
    e.set(this, PlayerCharacter::p1stPersonAnimation, Ptr::NULL);
    e.call(SMART_POINTER_ASSIGN, &args![player + 0x694, 0u32]);
    e.set_global(NODE_CAMERA_1ST_SLOT, 0u32);
    e.set_global(NODE_CAMERA_3RD_SLOT, 0u32);
    e.set_global(NODE_BIP_SLOT, 0u32);
    e.call(MEMSET, &args![player + 0xd74, 0u32, 0x60u32]);
    let path = setting_text(e, MODEL_PATH_SETTING);
    let path = e.call(IDENTITY, &args![path, 0u32]).u32();
    let loader = e.global::<u32>(MODEL_LOADER_POINTER);
    e.call(MODEL_LOADER_NOTIFY, &args![loader, path]);
}

// Translated from 0094ec90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BuildInvStringCallback` (Xbox PDB): appends `"<prefix><count> <name>
/// (<form id>),"` to the string `output` for an inventory entry; the prefix
/// is `"[E]"` for a worn entry. Does nothing for a null entry or string.
/// Always returns false. The stack-cookie check is not translated.
pub fn build_inv_string_callback(e: &mut Engine, item: Ptr, output: Ptr) -> bool {
    if !output.is_null() && !item.is_null() {
        let worn = e.call(ITEM_CHANGE_GET_WORN, &args![item, 0u32]).bool();
        let prefix = if worn {
            INVENTORY_PREFIX_WORN
        } else {
            INVENTORY_PREFIX_CARRIED
        };
        let form = e.call(WORD_AT_8, &args![item]).u32();
        let id = form_id_of(e, form);
        let name = e.call(ITEM_CHANGE_GET_FULL_NAME, &args![item]).u32();
        let count = list_next(e, item.addr());
        // The exe's frame (`SUB ESP,0x114` under `EBP`): the line at `EBP-0x110` (+4).
        e.with_stack(0x114, |e, frame| {
            let buffer = frame.addr() + 4;
            e.call(
                SPRINTF,
                &args![
                    buffer,
                    0x104u32,
                    FORMAT_INVENTORY_ITEM,
                    prefix,
                    count,
                    name,
                    id
                ],
            );
            e.call(STRING_APPEND, &args![output, buffer]);
        });
    }
    false
}

// ---------------------------------------------------------------------
// ExportProgressData
// ---------------------------------------------------------------------

/// `BSStringT::operator=`-style formatter `00406f60` (`cdecl`, variadic):
/// formats into the string `text`.
fn format_into(e: &mut Engine, text: u32, rest: &[u32]) {
    let mut words = vec![text];
    words.extend_from_slice(rest);
    e.call(0x0040_6f60, &words);
}

/// Writes the string to the file (virtual slot `0x38` of the file with the
/// string and the flag 1).
fn write_text(e: &mut Engine, file: u32, text: u32) {
    e.vcall(file, 0x38, &args![text, 1u32]);
}

/// Formats `rest` into `text` and writes it to the file.
fn format_and_write(e: &mut Engine, file: u32, text: u32, rest: &[u32]) {
    format_into(e, text, rest);
    write_text(e, file, text);
}

/// The text of a name (`00408da0`, `ECX` = form + `0x18`).
fn name_of_form(e: &mut Engine, form: u32) -> u32 {
    e.call(0x0040_8da0, &args![form.wrapping_add(0x18)]).u32()
}

/// Resets the string to `"` (a quote), `004037f0`.
fn reset_to_quote(e: &mut Engine, text: u32) {
    e.call(0x0040_37f0, &args![text, EMPTY_QUOTE_TEXT, 0u32]);
}

// Translated from 0094ed40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::ExportProgressData` (Xbox PDB): writes a tab-separated
/// progress line to `"<export directory>ProgressData_1.txt"`: build name,
/// version, calendar value, the reason (indexed by `reason`), the player's
/// name and location, sex, level, karma, perks, skills and attributes
/// (base and current), inventory, weight, reputation values, quest log,
/// the 43 counters of `005a3370`, the cell, and, for reason 1 (death), the
/// cause, killer and weapon data. Returns nothing; does nothing when the file
/// cannot be opened. C++ exception unwinding and the stack cookie are not
/// translated.
pub fn player_character_export_progress_data(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    reason: i32,
) {
    let player = this.addr();
    // The exe's frame (`SUB ESP,0x39c` under the exception record, then the record link): the
    // path at `EBP-0x12c` (+0x280), the text at `EBP-0x20` (+0x38c), the perk line at `EBP-0x254`
    // (+0x158), the quest line at `EBP-0x36c` (+0x40), the two floats at `EBP-0x140` (+0x26c)
    // and `EBP-0x138` (+0x274).
    e.with_stack(0x3ac, |e, frame| {
        let directory = setting_text(e, EXPORT_DIRECTORY_SETTING);
        let path = frame.addr() + 0x280;
        e.call(
            SPRINTF,
            &args![path, 0x104u32, FORMAT_EXPORT_PATH, directory],
        );
        let block = e.call(OPERATOR_NEW, &args![0x158u32]).u32();
        let file = if block != 0 {
            e.call(0x00b0_0260, &args![block, path, 1u32, 0x4000u32, 0u32])
                .u32()
        } else {
            0
        };
        if file == 0 || !e.call(0x0089_05f0, &args![file]).bool() {
            return;
        }
        let seek = e.global::<u32>(EXPORT_SEEK_MODE);
        e.vcall(file, 0x14, &args![0u32, seek]);
        let text = frame.addr() + 0x38c;
        e.call(0x0040_37b0, &args![text]);
        let _form = e.call(0x007a_f430, &args![player]).u32();

        let owner = player + ACTOR_VALUE_OWNER;
        if e.mem.i8(BUILD_NAME_BUFFER) != 0 {
            format_and_write(e, file, text, &args![FORMAT_TEXT, BUILD_NAME_BUFFER]);
        } else {
            format_and_write(e, file, text, &args![FORMAT_TEXT, UNKNOWN_TEXT]);
        }
        format_and_write(e, file, text, &args![FORMAT_TEXT, GAME_VERSION_TEXT]);
        let calendar_value = e.call(0x0086_7e30, &args![CALENDAR]).u32();
        format_and_write(e, file, text, &args![FORMAT_INT, calendar_value]);
        let reason_name = e
            .mem
            .u32(REASON_NAME_TABLE.wrapping_add((reason as u32).wrapping_mul(4)));
        format_and_write(e, file, text, &args![FORMAT_TEXT, reason_name]);
        let name = e.call(0x0055_d520, &args![player]).u32();
        format_and_write(e, file, text, &args![FORMAT_TEXT, name]);
        let location = e.vcall(player, 0x37c, &args![]).u32();
        let location_name = name_of_form(e, location);
        format_and_write(e, file, text, &args![FORMAT_TEXT, location_name]);
        let sex = e.call(0x0087_f4c0, &args![player]).u32();
        let sex_name = e.mem.u32(SEX_NAME_TABLE.wrapping_add(sex.wrapping_mul(4)));
        format_and_write(e, file, text, &args![FORMAT_TEXT, sex_name]);
        let level = e.call(0x0087_f9f0, &args![player]).u16() as u32;
        format_and_write(e, file, text, &args![FORMAT_INT, level]);
        let experience = e.vcall(owner, 0x8, &args![0x18u32]).u32();
        format_and_write(e, file, text, &args![FORMAT_INT, experience]);

        // The perks, as `"name (id)-rank,name (id)-rank,"`.
        reset_to_quote(e, text);
        let mut node = player + 0x87c;
        while node != 0 && !list_is_empty(e, node) {
            let slot = list_item_slot(e, node);
            let entry = e.mem.u32(slot);
            node = list_next(e, node);
            if e.mem.u32(entry) != 0 {
                let rank = e.mem.u8(entry + 4) as u32;
                let perk = e.mem.u32(entry);
                let id = form_id_of(e, perk);
                let perk_name = name_of_form(e, perk);
                let buffer = frame.addr() + 0x158;
                e.call(
                    SPRINTF,
                    &args![buffer, 0x104u32, FORMAT_PERK, perk_name, id, rank],
                );
                e.call(STRING_APPEND, &args![text, buffer]);
            }
        }
        e.call(STRING_APPEND, &args![text, TEXT_CLOSE_QUOTE]);
        write_text(e, file, text);

        for index in 0..0xeu32 {
            let value = e.vcall(owner, 0x8, &args![index + 0x20]).u32();
            format_and_write(e, file, text, &args![FORMAT_INT, value]);
        }
        for index in 0..7u32 {
            let value = e.vcall(owner, 0x0, &args![index + 5]).u32();
            format_and_write(e, file, text, &args![FORMAT_INT, value]);
            let value = e.vcall(owner, 0x8, &args![index + 5]).u32();
            format_and_write(e, file, text, &args![FORMAT_INT, value]);
        }
        for index in 0..0x14u32 {
            let value = e.vcall(owner, 0x0, &args![index + 0xc]).u32();
            format_and_write(e, file, text, &args![FORMAT_INT, value]);
            let value = e.vcall(owner, 0x8, &args![index + 0xc]).u32();
            format_and_write(e, file, text, &args![FORMAT_INT, value]);
        }

        // The inventory.
        let inventory = e.call(0x004b_f220, &args![player]).u32();
        if inventory != 0 {
            reset_to_quote(e, text);
            e.call(0x004d_4530, &args![inventory, 0x0094_ec90u32, text, 0u32]);
            e.call(STRING_APPEND, &args![text, TEXT_CLOSE_QUOTE]);
        } else {
            format_into(e, text, &args![TEXT_EMPTY_QUOTES]);
        }
        write_text(e, file, text);

        let (first_at, second_at) = (frame.addr() + 0x26c, frame.addr() + 0x274);
        e.call(0x0093_abe0, &args![player, first_at, second_at]);
        let first = e.mem.f32(first_at) as f64;
        let second = e.mem.f32(second_at) as f64;
        format_and_write(e, file, text, &args![FORMAT_TWO_FLOATS, first, second]);
        let weight = if inventory != 0 {
            e.call(0x004d_0f40, &args![inventory, 0u32, 0u32]).u32()
        } else {
            0
        };
        format_and_write(e, file, text, &args![FORMAT_INT, weight]);
        let value = e.call(0x0057_7d50, &args![player]).u32();
        format_and_write(e, file, text, &args![FORMAT_INT, value]);
        let speed = e.vcall(player, 0x43c, &args![]).f64();
        format_and_write(e, file, text, &args![FORMAT_ONE_FLOAT, speed]);

        // The quest log.
        reset_to_quote(e, text);
        let mut node = player + 0x6b0;
        while node != 0 && !list_is_empty(e, node) {
            let slot = list_item_slot(e, node);
            let item = e.mem.u32(slot);
            node = list_next(e, node);
            if e.call(0x0060_fd70, &args![item]).bool()
                && e.call(0x005e_3fa0, &args![item]).u32() != 0
            {
                let quest = e.call(0x005e_3fa0, &args![item]).u32();
                let quest_again = e.call(0x005e_3fa0, &args![item]).u32();
                let id = form_id_of(e, quest_again);
                let quest_name = e.vcall(quest, 0x138, &args![]).u32();
                let buffer = frame.addr() + 0x40;
                e.call(
                    SPRINTF,
                    &args![buffer, 0x104u32, FORMAT_QUEST, quest_name, id],
                );
                e.call(STRING_APPEND, &args![text, buffer]);
            }
        }
        e.call(STRING_APPEND, &args![text, TEXT_CLOSE_QUOTE]);
        write_text(e, file, text);

        for index in 0..0x2bu32 {
            let value = e.call(0x005a_3370, &args![index]).u32();
            format_and_write(e, file, text, &args![FORMAT_INT, value]);
        }

        // The cell.
        let cell = e.call(PARENT_CELL_OF, &args![player]).u32();
        if cell != 0 {
            if e.call(0x0042_5fd0, &args![cell]).bool() {
                let id = form_id_of(e, cell);
                let cell_name = name_of_form(e, cell);
                format_into(e, text, &args![FORMAT_NAME_ID, cell_name, id]);
            } else {
                let world = e.call(0x0054_ddd0, &args![cell]).u32();
                if world != 0 {
                    let id = form_id_of(e, cell);
                    let y = e.call(0x0054_4c60, &args![cell]).u32();
                    let x = e.call(0x0054_4c30, &args![cell]).u32();
                    let world_name = name_of_form(e, world);
                    format_into(e, text, &args![FORMAT_CELL_WORLD, world_name, x, y, id]);
                } else {
                    let id = form_id_of(e, cell);
                    let y = e.call(0x0054_4c60, &args![cell]).u32();
                    let x = e.call(0x0054_4c30, &args![cell]).u32();
                    format_into(e, text, &args![FORMAT_CELL_UNKNOWN_WORLD, x, y, id]);
                }
            }
        } else {
            format_into(e, text, &args![TEXT_NONE]);
        }
        write_text(e, file, text);

        if reason == 1 {
            let extra_list = e.call(EXTRA_LIST_OF, &args![player]).u32();
            let extra = e.call(GET_EXTRA_DATA, &args![extra_list, 0x5fu32]).u32();
            if extra != 0 {
                let cause = e.mem.i32(extra + 0x10);
                if cause != -1 {
                    let cause_name = e
                        .mem
                        .u32(CAUSE_NAME_TABLE.wrapping_add((cause as u32).wrapping_mul(4)));
                    format_into(e, text, &args![FORMAT_TEXT, cause_name]);
                } else {
                    format_into(e, text, &args![TEXT_NONE]);
                }
                write_text(e, file, text);
                let object = e.mem.u32(extra + 0x14);
                if object != 0 {
                    let id = form_id_of(e, object);
                    let object_name = e.vcall(object, 0x130, &args![]).u32();
                    let object_text = e.call(0x0044_0e30, &args![object]).u32();
                    format_into(
                        e,
                        text,
                        &args![FORMAT_CAUSE_OBJECT, object_text, object_name, id],
                    );
                } else {
                    format_into(e, text, &args![TEXT_NONE]);
                }
            } else {
                format_and_write(e, file, text, &args![TEXT_UNKNOWN_CAUSE]);
                format_and_write(e, file, text, &args![TEXT_UNKNOWN_OBJECT]);
            }

            let killer = e.get(this, PlayerCharacter::pMyKiller).addr();
            if killer != 0 {
                let id = form_id_of(e, killer);
                let killer_name = e.call(0x0055_d520, &args![killer]).u32();
                format_and_write(e, file, text, &args![FORMAT_NAME_ID, killer_name, id]);
                if e.call(0x0056_af40, &args![killer]).bool() {
                    let extra_list = e.call(EXTRA_LIST_OF, &args![killer]).u32();
                    let original = e.call(0x0042_16f0, &args![extra_list]).u32();
                    let extra_list = e.call(EXTRA_LIST_OF, &args![killer]).u32();
                    let template = e.call(0x0042_1720, &args![extra_list]).u32();
                    if original != 0 && template != 0 {
                        let template_id = form_id_of(e, template);
                        let template_name = e.vcall(template, 0x130, &args![]).u32();
                        let original_id = form_id_of(e, original);
                        let original_name = e.vcall(original, 0x130, &args![]).u32();
                        format_into(
                            e,
                            text,
                            &args![
                                FORMAT_LEVELLED_BOTH,
                                original_name,
                                original_id,
                                template_name,
                                template_id
                            ],
                        );
                    } else if original != 0 {
                        let original_id = form_id_of(e, original);
                        let original_name = e.vcall(original, 0x130, &args![]).u32();
                        format_into(
                            e,
                            text,
                            &args![FORMAT_LEVELLED_ORIGINAL_ONLY, original_name, original_id],
                        );
                    } else if template != 0 {
                        let template_id = form_id_of(e, template);
                        let template_name = e.vcall(template, 0x130, &args![]).u32();
                        format_into(
                            e,
                            text,
                            &args![FORMAT_LEVELLED_TEMPLATE_ONLY, template_name, template_id],
                        );
                    } else {
                        format_into(e, text, &args![TEXT_UNKNOWN_LEVELLED]);
                    }
                } else {
                    let base = e.call(BASE_FORM_OF, &args![killer]).u32();
                    let base_again = e.call(BASE_FORM_OF, &args![killer]).u32();
                    let id = form_id_of(e, base_again);
                    let base_name = e.vcall(base, 0x130, &args![]).u32();
                    format_into(e, text, &args![FORMAT_NAME_ID, base_name, id]);
                }
                write_text(e, file, text);
                let damage = e.call(0x0056_8ad0, &args![killer]).f64();
                format_and_write(e, file, text, &args![FORMAT_ONE_FLOAT, damage]);
                let weapon = e.call(0x008a_1710, &args![killer]).u32();
                if weapon != 0 {
                    let id = form_id_of(e, weapon);
                    let weapon_name = e.call(0x0040_8da0, &args![weapon + 0x30]).u32();
                    format_into(e, text, &args![FORMAT_NAME_ID, weapon_name, id]);
                } else {
                    format_into(e, text, &args![TEXT_NO_WEAPON]);
                }
                write_text(e, file, text);
            } else {
                format_and_write(e, file, text, &args![TEXT_UNKNOWN_KILLER]);
            }
        }

        format_and_write(e, file, text, &args![TEXT_LINE_END]);
        e.call(0x00af_fd10, &args![file]);
        e.vcall(file, 0, &args![1u32]);
        e.call(0x0040_37d0, &args![text]);
    });
}

// Translated from 0094fce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `ProcessLists::PrintLists` (Xbox PDB name of `008d0600`) on the
/// sub-object at `+0x878` (`ECX` = the player's `CharacterProgression`... the
/// object at `+0x878`).
pub fn fn_0094fce0(e: &mut Engine, this: Ptr<PlayerCharacter>, first: u32, second: u32) {
    e.call(
        PROCESS_LISTS_PRINT_LISTS,
        &args![
            this.addr().wrapping_add(CHARACTER_PROGRESSION),
            first,
            second
        ],
    );
}

// Translated from 0094fd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `CharacterProgression::RewardExperience` (Xbox PDB name of
/// `008d5100`) on the `CharacterProgression` at `+0x878`.
pub fn fn_0094fd10(e: &mut Engine, this: Ptr<PlayerCharacter>, amount: u32) {
    e.call(
        CHARACTER_PROGRESSION_REWARD_EXPERIENCE,
        &args![this.addr().wrapping_add(CHARACTER_PROGRESSION), amount],
    );
}

// Translated from 0094fd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::RewardKarma` (Xbox PDB): changes the karma (actor value
/// `0x17`) by `delta`. The change is first limited so that the new karma stays
/// within the lower and upper limit settings, a message (a big one when the
/// change reaches the message threshold setting) is shown and the sound
/// `UIKarmaUp` or `UIKarmaDown` is played; the actor value is changed (virtual
/// slot `0x3a8` of the player) unless the karma already sits at the limit in
/// the direction of the change.
pub fn player_character_reward_karma(e: &mut Engine, this: Ptr<PlayerCharacter>, delta: i32) {
    let player = this.addr();
    let owner = player + ACTOR_VALUE_OWNER;
    let karma = e.vcall(owner, 0x8, &args![0x17u32]).i32();
    e.call(BASE_FORM_OF, &args![player]);
    let karma_value = e.vcall(owner, 0xc, &args![0x17u32]).f32();
    e.call(0x0047_e040, &args![karma_value]);
    let mut delta = delta;

    let show = |e: &mut Engine, icon: u32, text: u32| {
        let icon = setting_text(e, icon);
        let text = setting_text(e, text);
        let seconds = e.global::<f32>(MESSAGE_SECONDS);
        e.call(0x0070_52f0, &args![text, 0u32, icon, 0u32, seconds, 0u32]);
    };

    let sound = if delta < 0 {
        let lower = setting_int(e, KARMA_LOWER_LIMIT_SETTING);
        if karma.wrapping_add(delta) < lower {
            delta = lower.wrapping_sub(karma);
        }
        let threshold = setting_int(e, KARMA_MESSAGE_THRESHOLD_SETTING);
        if delta < threshold.wrapping_neg() {
            show(e, KARMA_LOSS_BIG_ICON_SETTING, KARMA_LOSS_BIG_TEXT_SETTING);
        } else {
            show(e, KARMA_LOSS_ICON_SETTING, KARMA_LOSS_TEXT_SETTING);
        }
        SOUND_KARMA_DOWN
    } else {
        let upper = setting_int(e, KARMA_UPPER_LIMIT_SETTING);
        if karma.wrapping_add(delta) > upper {
            delta = upper.wrapping_sub(karma);
        }
        let threshold = setting_int(e, KARMA_MESSAGE_THRESHOLD_SETTING);
        if delta < threshold {
            show(e, KARMA_GAIN_ICON_SETTING, KARMA_GAIN_TEXT_SETTING);
        } else {
            show(e, KARMA_GAIN_BIG_ICON_SETTING, KARMA_GAIN_BIG_TEXT_SETTING);
        }
        SOUND_KARMA_UP
    };

    // `BSSoundHandle` locals: the handle and the temporary the lookup fills. The exe's frame
    // (`SUB ESP,0x4c` under the exception record, then `ESI` and the record link) has a pair
    // for each branch: `EBP-0x20` (+0x40) and `EBP-0x38` (+0x28) for the loss, `EBP-0x2c`
    // (+0x34) and `EBP-0x44` (+0x1c) for the gain.
    e.with_stack(0x60, |e, frame| {
        let (handle, temporary) = if sound == SOUND_KARMA_DOWN {
            (frame.addr() + 0x40, frame.addr() + 0x28)
        } else {
            (frame.addr() + 0x34, frame.addr() + 0x1c)
        };
        e.call(0x0041_a250, &args![handle]);
        let audio = e.call(0x0045_3a70, &args![]).u32();
        let found = e
            .call(0x00ad_7550, &args![audio, temporary, sound, 0x121u32])
            .u32();
        e.call(0x0041_8900, &args![handle, found]);
        e.call(RELEASE_HANDLE, &args![temporary]);
        e.call(0x00ad_8830, &args![handle, 0u32]);
        e.call(RELEASE_HANDLE, &args![handle]);
    });

    let upper = setting_int(e, KARMA_UPPER_LIMIT_SETTING);
    let upper_ok = delta <= 0 || karma < upper;
    let lower = setting_int(e, KARMA_LOWER_LIMIT_SETTING);
    let lower_ok = delta >= 0 || karma > lower;
    if upper_ok && lower_ok {
        e.vcall(player, 0x3a8, &args![0x17u32, delta, 0u32]);
    }
}

// Translated from 00950010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `bChargen` (`+0x75c`).
pub fn fn_00950010(e: &mut Engine, this: Ptr<PlayerCharacter>, value: u8) {
    e.set(this, PlayerCharacter::bChargen, value);
}

// Translated from 00950030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::UpdatePlayer3d` (Xbox PDB): when the process's slot
/// `0x474` (with 1) says false and the inventory menu is visible, calls
/// `00483710` on the player; then calls the process's slot `0x464` with the
/// player.
pub fn player_character_update_player_3d(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let process = process_of(e, this);
    if !e.vcall(process, 0x474, &args![1u32]).bool()
        && e.call(INVENTORY_MENU_VISIBLE, &args![]).bool()
    {
        e.call(RELEASE_HANDLE, &args![this]);
    }
    let process = process_of(e, this);
    e.vcall(process, 0x464, &args![this]);
}

// Translated from 00950090 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `0x011e07b8` (vanity mode active). `this` is not used.
pub fn fn_00950090(e: &mut Engine, _this: Ptr) -> u8 {
    e.global::<u8>(VANITY_ACTIVE)
}

// Translated from 009500a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::StopVanityMode` (Xbox PDB): ends vanity mode. When the
/// saved flag (`0x011e07b9`) is set, restores the saved value; clears both
/// flags, sets the HUD menu mode to 1 and resets the vanity state.
pub fn player_character_stop_vanity_mode(e: &mut Engine, _this: Ptr) {
    if e.global::<u8>(VANITY_SAVED_FLAG) != 0 {
        let saved: f32 = e.global(VANITY_SAVED_VALUE);
        e.set_global(VANITY_RESTORED_VALUE, saved);
    }
    e.set_global(VANITY_ACTIVE, 0u8);
    e.set_global(VANITY_SAVED_FLAG, 0u8);
    e.call(SET_MENU_MODE, &args![1u32]);
    e.set_global(VANITY_FLAG_07C3, 1u8);
    e.set_global(VANITY_FLAG_07C1, 0u8);
    for address in [
        VANITY_VALUE_07C8,
        VANITY_VALUE_07C4,
        VANITY_VALUE_0B58,
        VANITY_VALUE_0B60,
        VANITY_VALUE_07DC,
    ] {
        e.set_global(address, 0.0f32);
    }
}

// ---------------------------------------------------------------------
// Second session: the camera switch, the animations and the quest lists
// (`00950110` to `00952a20`)
// ---------------------------------------------------------------------

/// The float setting copied into [`VANITY_RESTORED_VALUE`] by `SetFirstPerson`.
const RESTORED_VALUE_SETTING_011CD614: u32 = 0x011c_d614;
/// The float setting `ForceTemp3rdPerson` raises [`VANITY_RESTORED_VALUE`] to.
const RESTORED_VALUE_SETTING_011CD498: u32 = 0x011c_d498;
/// Three floats (a position) copied from the `Camera1st` node by `SetFirstPerson`.
const CAMERA_POSITION_COPY: u32 = 0x011e_0808;
/// Two flags that make `SetFirstPerson` apply the switch at once.
const CAMERA_SWITCH_FLAG_011F21D0: u32 = 0x011f_21d0;
const CAMERA_SWITCH_FLAG_011F21D1: u32 = 0x011f_21d1;
/// `30.0` (`double`): the zoom value up to which `UpdateFirstPersonZoom` acts.
const FIRST_PERSON_ZOOM_LIMIT: u32 = 0x0101_db88;
/// The byte that keeps `UpdateTemp1stPerson` from testing the iron sights.
const IRON_SIGHTS_SKIP_FLAG: u32 = 0x011e_0780;
/// The byte `UpdateTemp1stPerson` sets when the temporary first person ends.
const TEMP_FIRST_PERSON_ENDED: u32 = 0x011a_3b32;
/// The global object that `0044ddc0` and `009c8cc0` are called on.
const OBJECT_011F2250: u32 = 0x011f_2250;
/// The depth counter that `00950930` raises while it updates the animations.
const ANIMATION_UPDATE_DEPTH: u32 = 0x011e_07a8;
/// `-1.0` (`float`).
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
/// `0.0`, `1.0` and `-1.0` as `double` constants of the exe.
const ZERO_DOUBLE: u32 = 0x0101_2060;
const ONE_DOUBLE: u32 = 0x0101_2070;
const MINUS_ONE_DOUBLE: u32 = 0x0101_a6b0;
/// The byte `00951900` returns and `00951910` sets, and the two words
/// `00951920` and `00951930` return (names the clone looks nodes up by).
const INVENTORY_CLONE_FLAG: u32 = 0x011f_a00c;
const NODE_NAME_011C623C: u32 = 0x011c_623c;
const NODE_NAME_011C6274: u32 = 0x011c_6274;
/// The god-mode and demigod-mode bytes and the setting that forces both.
const GOD_MODE_BYTE: u32 = 0x011e_07ba;
const DEMIGOD_MODE_BYTE: u32 = 0x011e_07bb;
const GOD_MODE_SETTING: u32 = 0x011e_0894;
/// The float `00952290` moves towards its target, the angle it computes, and
/// a float set to 1.0 before `Actor::UpdateAlpha` (`008c4640`).
const ZOOM_FACTOR: u32 = 0x011a_3b68;
const LOOK_ANGLE: u32 = 0x011e_0770;
const INVENTORY_ALPHA: u32 = 0x011a_3b38;
/// Settings read by `00952290`: the target when not zoomed (float), the
/// duration of the zoom in and of the zoom out.
const ZOOM_TARGET_SETTING: u32 = 0x011c_dcb0;
const ZOOM_IN_SECONDS_SETTING: u32 = 0x011c_d620;
const ZOOM_OUT_SECONDS_SETTING: u32 = 0x011c_de38;
/// The table of animation group types (`0x24` bytes per entry): a byte at
/// `+0` and a word at `+4`.
const GROUP_TYPE_TABLE_FLAG: u32 = 0x0119_77dc;
const GROUP_TYPE_TABLE_WORD: u32 = 0x0119_77e0;
/// The table of 11 idle-folder names `CloneInventory3D` lists animations in
/// (index 1 to 11).
const IDLE_NAME_TABLE: u32 = 0x0119_77a4;
/// Type information used by the dynamic cast `00653270`.
const CAST_TYPE_011D5BF8: u32 = 0x011d_5bf8;
/// The word handed to `00440460` on the inventory model.
const INVENTORY_MODEL_ARGUMENT: u32 = 0x011f_426c;
/// Strings of `CloneInventory3D`: the format of the model path, the idle
/// file name, the folder names and the file-list formats.
const FORMAT_MODEL_PATH: u32 = 0x0101_9f08;
const TEXT_MT_IDLE: u32 = 0x0101_6fa0;
const TEXT_MESHES: u32 = 0x0101_dccc;
const FORMAT_DATA_PATH: u32 = 0x0101_fb74;
const FORMAT_IDLE_PATH: u32 = 0x0108_b320;
const FORMAT_IDLE_NAME: u32 = 0x0108_b314;
const FORMAT_TORCH_IDLE_PATH: u32 = 0x0108_b2f8;
const FORMAT_TORCH_IDLE_NAME: u32 = 0x0108_b2e8;

/// The offsets of the lists of the player used by the quest code.
const TOPIC_LIST: u32 = 0x6a8;
const QUEST_LOG_LIST: u32 = 0x6b0;
const QUEST_TARGET_LIST: u32 = 0x6c4;

/// `NiPointer` dereference (`00559450`): the object the smart pointer at
/// `slot` holds.
fn pointer_target(e: &mut Engine, slot: u32) -> u32 {
    e.call(0x0055_9450, &args![slot]).u32()
}

// Translated from 00950110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::SetFirstPerson` (Xbox PDB): `first_person` non-zero
/// asks for the first person (`bWant3rdPerson` becomes 0, otherwise 1).
/// Returns true when the wanted camera changed (the code compares the old
/// `bWant3rdPerson` with the argument, which is equal exactly then). When the
/// third person is wanted and slot `0x22c` of the player says yes, copies the
/// restored-value setting `0x011cd614` to `0x011e0b5c`. Outside vanity mode,
/// with a first-person model and a changed request for the third person
/// (`bWant3rdPerson` set), copies the position of the `Camera1st` node and
/// applies the model switch (`00951a10`); otherwise marks `b3rdPerson` when
/// the model is in the state `00456610` tests and the first person is wanted.
/// When the request changed and both camera-switch flags are set, copies the
/// wanted camera to `b3rdPerson` and applies the switch at once.
pub fn player_character_set_first_person(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    first_person: u8,
) -> bool {
    let player = this.addr();
    let changed = e.get(this, PlayerCharacter::bWant3rdPerson) == first_person;
    let want_third = u8::from(first_person == 0);
    e.set(this, PlayerCharacter::bWant3rdPerson, want_third);
    if want_third != 0 && e.vcall(player, 0x22c, &args![0u32]).bool() {
        let at = e
            .call(
                SETTING_FLOAT_POINTER,
                &args![RESTORED_VALUE_SETTING_011CD614],
            )
            .u32();
        let value = e.mem.f32(at);
        e.set_global(VANITY_RESTORED_VALUE, value);
    }
    let mut applied = false;
    if e.global::<u8>(VANITY_ACTIVE) == 0
        && pointer_target(e, player + 0x694) != 0
        && e.get(this, PlayerCharacter::bWant3rdPerson) != 0
        && changed
    {
        let camera = e.global::<u32>(NODE_CAMERA_1ST_SLOT);
        let at = e.call(0x0045_bb80, &args![camera]).u32();
        for word in 0..3u32 {
            let value = e.mem.u32(at + 4 * word);
            e.mem.set_u32(CAMERA_POSITION_COPY + 4 * word, value);
        }
        let hide = u8::from(e.get(this, PlayerCharacter::bWant3rdPerson) == 0);
        fn_00951a10(e, this, hide);
        applied = true;
    }
    if !applied && e.global::<u8>(VANITY_ACTIVE) == 0 && pointer_target(e, player + 0x694) != 0 {
        let model = pointer_target(e, player + 0x694);
        if e.call(0x0045_6610, &args![model]).bool()
            && e.get(this, PlayerCharacter::bWant3rdPerson) == 0
        {
            e.set(this, PlayerCharacter::b3rdPerson, true);
        }
    }
    if changed
        && e.global::<u8>(CAMERA_SWITCH_FLAG_011F21D0) != 0
        && e.global::<u8>(CAMERA_SWITCH_FLAG_011F21D1) != 0
    {
        // Copy of the byte at +0x64c to the byte at +0x64a (b3rdPerson).
        let wanted = e.mem.u8(player + 0x64c);
        e.mem.set_u8(player + 0x64a, wanted);
        fn_00951a10(e, this, 1);
    }
    changed
}

// Translated from 00950290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::UpdateFirstPersonZoom` (Xbox PDB): when the wanted
/// camera (`+0x64c`) differs from the current one (`+0x64a`) and the zoom
/// value `0x011e07dc` is not above `30.0`, adopts it and, outside vanity
/// mode and with a first-person model, applies the switch (`00951a10`).
pub fn player_character_update_first_person_zoom(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let player = this.addr();
    let wanted = e.get(this, PlayerCharacter::bWant3rdPerson);
    if wanted == e.mem.u8(player + 0x64a) {
        return;
    }
    let zoom = f64::from(e.global::<f32>(VANITY_VALUE_07DC));
    let limit = e.global::<f64>(FIRST_PERSON_ZOOM_LIMIT);
    if zoom > limit {
        return;
    }
    e.mem.set_u8(player + 0x64a, wanted);
    if e.global::<u8>(VANITY_ACTIVE) == 0
        && pointer_target(e, player + 0x694) != 0
        && e.mem.u8(player + 0x64a) == 0
    {
        fn_00951a10(e, this, 1);
    }
}

// Translated from 00950340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::ForceTemp3rdPerson` (Xbox PDB): does nothing (returns
/// false) while a temporary camera is already forced (`005721e0`). Else
/// sets `bTemp3rdPerson`; with `raise_restored_value` raises the restored
/// value `0x011e0b5c` to the setting `0x011cd498` when that is larger; and,
/// if the camera is in the first person (`b3rdPerson` 0), remembers to switch
/// back and requests the third person (`SetFirstPerson(0)`), returning its
/// result. Returns false when the player already is in the third person.
pub fn player_character_force_temp_3rd_person(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    raise_restored_value: u8,
) -> bool {
    let player = this.addr();
    if e.call(0x0057_21e0, &args![player]).bool() {
        return false;
    }
    e.set(this, PlayerCharacter::bTemp3rdPerson, 1);
    if raise_restored_value != 0 {
        let at = e
            .call(
                SETTING_FLOAT_POINTER,
                &args![RESTORED_VALUE_SETTING_011CD498],
            )
            .u32();
        let setting = e.mem.f32(at);
        let current = e.global::<f32>(VANITY_RESTORED_VALUE);
        if setting > current {
            let at = e
                .call(
                    SETTING_FLOAT_POINTER,
                    &args![RESTORED_VALUE_SETTING_011CD498],
                )
                .u32();
            let value = e.mem.f32(at);
            e.set_global(VANITY_RESTORED_VALUE, value);
        }
    }
    if e.mem.u8(player + 0x64a) != 0 {
        return false;
    }
    e.set(this, PlayerCharacter::bTemp3rdPersonSwitchBack, 1);
    player_character_set_first_person(e, this, 0)
}

// Translated from 009503d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::UpdateTemp3rdPerson` (Xbox PDB): ends the temporary
/// third person when the process's slot `0x3e4` answers `-1`, slot `0x40c`
/// answers 0 and neither vanity flag is set: clears `bTemp3rdPerson`,
/// switches back to the first person if it was remembered, and forgets it.
pub fn player_character_update_temp_3rd_person(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    if e.get(this, PlayerCharacter::bTemp3rdPerson) == 0 {
        return;
    }
    let process = process_of(e, this);
    if e.vcall(process, 0x3e4, &args![]).i32() != -1 {
        return;
    }
    let process = process_of(e, this);
    if e.vcall(process, 0x40c, &args![]).u32() == 0
        && e.global::<u8>(VANITY_ACTIVE) == 0
        && e.global::<u8>(VANITY_SAVED_FLAG) == 0
    {
        e.set(this, PlayerCharacter::bTemp3rdPerson, 0);
        if e.get(this, PlayerCharacter::bTemp3rdPersonSwitchBack) != 0 {
            player_character_set_first_person(e, this, 1);
        }
        e.set(this, PlayerCharacter::bTemp3rdPersonSwitchBack, 0);
    }
}

// Translated from 00950460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::ForceTemp1stPerson` (Xbox PDB): stops vanity mode, sets
/// `bTemp1stPerson`, lowers the iron sights (when `keep_sights` is 0 and they
/// are up) and the block, then either does nothing (returns false) or asks
/// for the first person (`SetFirstPerson(1)`, remembered in
/// `bTemp1stPersonSwitchBack`) and returns its result.
pub fn player_character_force_temp_1st_person(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    keep_sights: u8,
) -> bool {
    let player = this.addr();
    player_character_stop_vanity_mode(e, this.cast());
    e.set(this, PlayerCharacter::bTemp1stPerson, 1);
    if keep_sights == 0 && e.call(0x008b_bc10, &args![player]).bool() {
        e.call(0x008b_b650, &args![player, 0u32, 0u32, 0u32]);
    }
    e.call(0x0089_4cc0, &args![player, 0u32]);
    if keep_sights != 0 {
        let model = fn_00950bb0(e, this, 0);
        if e.call(0x0045_6610, &args![model]).bool() {
            return false;
        }
    } else if e.mem.u8(player + 0x64a) == 0 {
        let model = fn_00950bb0(e, this, 0);
        if !e.call(0x0045_6610, &args![model]).bool() {
            let model = fn_00950bb0(e, this, 0);
            e.call(0x0045_0f90, &args![model, 1u32]);
        }
        return false;
    }
    e.set(this, PlayerCharacter::bTemp1stPersonSwitchBack, 1);
    player_character_set_first_person(e, this, 1)
}

// Translated from 00950530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::UpdateTemp1stPerson` (Xbox PDB): ends the temporary
/// first person (`bTemp1stPerson`) unless the Pipboy is active or
/// `0044ddc0` on the global object answers 4, or the iron sights are up with
/// a weapon that `0048cee0` accepts. Sets the "ended" byte and switches back
/// (`SetFirstPerson(0)`) if it was remembered.
pub fn player_character_update_temp_1st_person(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let player = this.addr();
    if e.get(this, PlayerCharacter::bTemp1stPerson) == 0 {
        return;
    }
    if e.call(0x0096_7ae0, &args![player]).bool() {
        return;
    }
    if e.call(0x0044_ddc0, &args![OBJECT_011F2250]).u32() == 4 {
        return;
    }
    if e.global::<u8>(IRON_SIGHTS_SKIP_FLAG) == 0 && e.call(0x008b_bc10, &args![player]).bool() {
        let process = process_of(e, this);
        if e.vcall(process, 0x148, &args![]).u32() != 0 {
            let process = process_of(e, this);
            let item = e.vcall(process, 0x148, &args![]).u32();
            let form = e.call(0x0044_ddc0, &args![item]).u32();
            let form = e.call(0x0050_4e60, &args![form]).u32();
            if e.call(0x0048_cee0, &args![form]).u32() != 0 {
                return;
            }
        }
    }
    e.set(this, PlayerCharacter::bTemp1stPerson, 0);
    e.set_global(TEMP_FIRST_PERSON_ENDED, 1u8);
    if e.get(this, PlayerCharacter::bTemp1stPersonSwitchBack) != 0 {
        player_character_set_first_person(e, this, 0);
    }
    e.set(this, PlayerCharacter::bTemp1stPersonSwitchBack, 0);
}

// Translated from 00950610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the field of view `fFOV` (`+0x65c`), hands it to the scene
/// graph's camera (`BSSceneGraph::SetCameraFOV`, Xbox PDB name of
/// `00c52020`, on the object `0045c670` returns, with three zero flags) and
/// to `00b54000`, and returns the stored value. `0045c670` is a plain
/// `RET` in the exe: the four words the caller pushes stay on the stack for
/// `00c52020`.
pub fn fn_00950610(e: &mut Engine, this: Ptr<PlayerCharacter>, fov: f32) -> f32 {
    e.set(this, PlayerCharacter::fFOV, fov);
    let stored = e.get(this, PlayerCharacter::fFOV);
    let scene = e.call(0x0045_c670, &args![]).u32();
    e.call(0x00c5_2020, &args![scene, stored, 0u32, 0u32, 0u32]);
    e.call(0x00b5_4000, &args![fov]);
    e.get(this, PlayerCharacter::fFOV)
}

// Translated from 00950660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::Resurrect` (Xbox PDB): resets the health modifier,
/// replaces the player's process by a new `HighProcess` (the old one is
/// destroyed through its virtual destructor; the process lists are told
/// twice), rebuilds the character controller's camera caster, and runs
/// `Actor::Resurrect` (`0089f780`) and `InitAnimation` (`005659f0`). The
/// three words the caller pushes are not read. C++ exception unwinding is not
/// translated.
pub fn player_character_resurrect(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) {
    let player = this.addr();
    e.set(this, PlayerCharacter::fHealthModifier, 0.0);
    e.call(0x0070_4e10, &args![0x10u32]);
    e.call(0x0040_fbf0, &args![PROCESS_LOCK, 0u32]);
    let old = process_of(e, this);
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    let block = e.call(OPERATOR_NEW, &args![0xb4u32]).u32();
    let base = if block != 0 {
        e.call(0x0090_6dc0, &args![block]).u32()
    } else {
        0
    };
    e.set(this, PlayerCharacter::pCurrentProcess, Ptr::new(base));
    let block = e.call(OPERATOR_NEW, &args![0x46cu32]).u32();
    let high = if block != 0 {
        e.call(0x008d_7510, &args![block]).u32()
    } else {
        0
    };
    let current = process_of(e, this);
    e.vcall(high, 4, &args![current]);
    let process_type = e.call(0x0093_1850, &args![player]).u32();
    e.call(0x0096_d470, &args![PROCESS_LISTS, player, process_type]);
    let old = process_of(e, this);
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    e.set(this, PlayerCharacter::pCurrentProcess, Ptr::new(high));
    let process_type = e.call(0x0093_1850, &args![player]).u32();
    e.call(0x0096_d470, &args![PROCESS_LISTS, player, process_type]);
    e.call(0x0040_fba0, &args![PROCESS_LOCK]);
    e.vcall(player, 0x1c4, &args![]);
    let controller = e.call(0x0093_06d0, &args![player]).u32();
    if controller != 0 {
        // The exe's frame (`SUB ESP,0x50` under the exception record, then the record link):
        // the built word at `EBP-0x1c` (+0x44).
        let caster = e.with_stack(0x60, |e, frame| {
            let out = frame.addr() + 0x44;
            e.call(0x0070_c440, &args![controller, out]);
            let value = e.call(0x004a_3a20, &args![out]).u32();
            let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
            if block != 0 {
                let at = e.call(SETTING_FLOAT_POINTER, &args![0x011e_0934u32]).u32();
                let seconds = e.mem.f32(at);
                e.call(0x0062_0850, &args![block, seconds, value]).u32()
            } else {
                0
            }
        });
        e.mem.set_u32(player + 0x21c, caster);
    } else {
        e.mem.set_u32(player + 0x21c, 0);
    }
    e.call(0x0089_f780, &args![player, 0u32, 0u32, 0u32]);
    e.call(0x0056_59f0, &args![player]);
}

// Translated from 009508b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Damages the equipment (`Actor::DamageEquipment` in the decompiler, the
/// callee `00891360`) unless the player is in god mode (`IsGodMode`, which
/// reads no `this`). Returns the callee's result, or false in god mode.
pub fn fn_009508b0(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    first: u32,
    amount: f32,
    flag: u8,
) -> bool {
    if player_character_is_god_mode(e) != 0 {
        return false;
    }
    e.call(0x0089_1360, &args![this, first, amount, flag])
        .bool()
}

// Translated from 009508f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00898650(this, first, amount)` and then virtual slot `0x22c` with 0.
pub fn fn_009508f0(e: &mut Engine, this: Ptr<PlayerCharacter>, first: u32, amount: f32) {
    e.call(0x0089_8650, &args![this, first, amount]);
    e.vcall(this.addr(), 0x22c, &args![0u32]);
}

// Translated from 00950930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates the animations of the player for one frame: the delta is the
/// frame time (`0084d030` on the global timer object) times the player
/// update multiplier (`VATS::GetPlayerUpdateMult`, Xbox PDB name of
/// `009c8cc0`). Both animations (`GetAnimation(0)` and the first-person one
/// at `+0x690`) get `Animation::Update(this, delta, -1.0)` (`00491180`), the
/// process gets its slot `0x740` with the delta, and slot `0x178` of the
/// player runs. A depth counter (`0x011e07a8`) is raised meanwhile.
pub fn fn_00950930(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let player = this.addr();
    let depth = e.global::<u8>(ANIMATION_UPDATE_DEPTH);
    e.set_global(ANIMATION_UPDATE_DEPTH, depth.wrapping_add(1));
    let frame = e.call(0x0084_d030, &args![TIMER_SOURCE_011F6394]).f64();
    let multiplier = e.call(0x009c_8cc0, &args![OBJECT_011F2250]).f64();
    let delta = (multiplier * frame) as f32;
    let minus_one = e.global::<f32>(MINUS_ONE_FLOAT);
    let animation = player_character_get_animation(e, this, 0);
    if animation != 0 {
        e.call(0x0049_1180, &args![animation, player, delta, minus_one]);
    }
    let first_person_animation = e.mem.u32(player + 0x690);
    if first_person_animation != 0 {
        e.call(
            0x0049_1180,
            &args![first_person_animation, player, delta, minus_one],
        );
    }
    let process = process_of(e, this);
    if process != 0 {
        e.vcall(process, 0x740, &args![player, delta]);
    }
    e.vcall(player, 0x178, &args![]);
    let depth = e.global::<u8>(ANIMATION_UPDATE_DEPTH);
    e.set_global(ANIMATION_UPDATE_DEPTH, depth.wrapping_sub(1));
}

// Translated from 00950a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The animation to use: the inventory animation (`+0x6a0`) when there is
/// one, else the first-person animation (`+0x690`) when the camera is in the
/// first person and has one, else the player's own (`008b70d0`).
pub fn fn_00950a10(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u32 {
    let player = this.addr();
    let inventory = e.get(this, PlayerCharacter::pInventoryAnimation).addr();
    if inventory != 0 {
        return inventory;
    }
    let first_person = e.get(this, PlayerCharacter::p1stPersonAnimation).addr();
    if e.mem.u8(player + 0x64a) == 0 && first_person != 0 {
        return first_person;
    }
    e.call(0x008b_70d0, &args![player]).u32()
}

// Translated from 00950a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::GetAnimation` (Xbox PDB): the first-person animation
/// (`+0x690`) when `first_person` is non-zero, else the player's own
/// (`008b70d0`).
pub fn player_character_get_animation(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    first_person: u8,
) -> u32 {
    if first_person != 0 {
        e.get(this, PlayerCharacter::p1stPersonAnimation).addr()
    } else {
        e.call(0x008b_70d0, &args![this]).u32()
    }
}

// Translated from 00950a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The biped to use: the first-person one (`+0x68c`) when the camera is in
/// the first person and has one, else the player's own (`005d9f90`).
pub fn fn_00950a90(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u32 {
    let player = this.addr();
    let biped = e.get(this, PlayerCharacter::p1stPersonBipedAnim).addr();
    if e.mem.u8(player + 0x64a) == 0 && biped != 0 {
        biped
    } else {
        e.call(0x005d_9f90, &args![player]).u32()
    }
}

// Translated from 00950ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetBiped(!result of 00524d10)`: the first-person biped when `00524d10`
/// says no, the player's own otherwise.
pub fn fn_00950ad0(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u32 {
    let own = e.call(0x0052_4d10, &args![this]).bool();
    player_character_get_biped(e, this, u8::from(!own))
}

// Translated from 00950b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::GetBiped` (Xbox PDB): the first-person biped (`+0x68c`)
/// when `first_person` is non-zero, else the player's own (`005d9f90`).
pub fn player_character_get_biped(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    first_person: u8,
) -> u32 {
    if first_person != 0 {
        e.get(this, PlayerCharacter::p1stPersonBipedAnim).addr()
    } else {
        e.call(0x005d_9f90, &args![this]).u32()
    }
}

// Translated from 00950b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::Is1stPersonBiped` (Xbox PDB): whether `biped` is the
/// (non-null) first-person biped.
pub fn player_character_is_1st_person_biped(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    biped: u32,
) -> bool {
    let own = e.get(this, PlayerCharacter::p1stPersonBipedAnim).addr();
    own != 0 && own == biped
}

// Translated from 00950b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 3D of the camera: the first-person model (the `NiPointer` at `+0x694`)
/// when the camera is in the first person and has one, else `0043fcd0`.
pub fn fn_00950b60(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u32 {
    let player = this.addr();
    if e.mem.u8(player + 0x64a) == 0 && pointer_target(e, player + 0x694) != 0 {
        pointer_target(e, player + 0x694)
    } else {
        e.call(0x0043_fcd0, &args![player]).u32()
    }
}

// Translated from 00950bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first-person model (`first_person` non-zero) or the player's own 3D
/// (`0043fcd0`).
pub fn fn_00950bb0(e: &mut Engine, this: Ptr<PlayerCharacter>, first_person: u8) -> u32 {
    let player = this.addr();
    if first_person != 0 {
        pointer_target(e, player + 0x694)
    } else {
        e.call(0x0043_fcd0, &args![player]).u32()
    }
}

// Translated from 00950be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::GetCurrent3D` (Xbox PDB): the player's own 3D
/// (`0043fcd0`) when `00524d10` says yes, else the first-person model.
pub fn player_character_get_current_3d(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u32 {
    let player = this.addr();
    if e.call(0x0052_4d10, &args![player]).bool() {
        e.call(0x0043_fcd0, &args![player]).u32()
    } else {
        pointer_target(e, player + 0x694)
    }
}

// Translated from 00950c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::CloneInventory3D` (Xbox PDB). With `create` non-zero
/// and no inventory model yet (the `NiPointer` at `+0x69c`), clones the
/// player's own 3D into it: removes the forces and controllers, clones with
/// a `NiCloningProcess`, culls it, gives it the properties and the matrix of
/// the race, finds the nodes whose names the exe keeps and hides or shows
/// them, builds an `Animation` (`+0x6a0`) from the idle files found next to
/// the model (`Meshes` folder, the 11 idle names, plain and torch idles) and
/// plays the first-person idle. With `create` 0 and a model present, destroys
/// the animation, hides the two named nodes of the model, clears the pointer
/// and destroys the object at `+0x6a4` (`00820bf0`). C++ exception unwinding
/// and the stack cookie are not translated.
pub fn player_character_clone_inventory_3d(e: &mut Engine, this: Ptr<PlayerCharacter>, create: u8) {
    let player = this.addr();
    let slot = player + 0x69c;
    let present = pointer_target(e, slot) != 0;
    if create != 0 {
        if !present && clone_inventory_3d_create(e, this) {
            return;
        }
    } else if present {
        clone_inventory_3d_destroy(e, this);
    }
    pointer_target(e, slot);
}

/// The creating branch of `CloneInventory3D`. Returns true when it ended
/// early (the model path has no folder), which skips the final
/// `NiPointer` dereference.
fn clone_inventory_3d_create(e: &mut Engine, this: Ptr<PlayerCharacter>) -> bool {
    let player = this.addr();
    let slot = player + 0x69c;
    let saved_flag = fn_00951900(e);
    fn_00951910(e, 1);
    e.call(0x0097_4c30, &args![PROCESS_LISTS]);
    e.call(0x0097_4420, &args![PROCESS_LISTS, 0.0f32]);
    let controller = e.vcall(player, 0x1d0, &args![]).u32();
    e.call(0x00cb_9650, &args![controller]);
    let own = fn_00950bb0(e, this, 0);
    e.call(0x00cb_9650, &args![own]);
    let own = fn_00950bb0(e, this, 0);
    let controllers = e.call(0x0043_b230, &args![own]).u32();
    // The exe's frame (`SUB ESP,0x354` under the exception record, then `ESI`, `EDI` and the
    // record link): the kept controllers at `EBP-0x24` (+0x348), the cloning process at
    // `EBP-0x2a4` (+0xc8), the matrix at `EBP-0x168` (+0x204), the rotated matrix at `EBP-0x2ec`
    // (+0x80), the holder at `EBP-0x18` (+0x354), the folder at `EBP-0x274` (+0xf8), the path at
    // `EBP-0x134` (+0x238) and the list words at `EBP-0x2b8` (+0xb4) and `EBP-0x2bc` (+0xb0).
    e.with_stack(0x36c, |e, frame| {
        // `NiPointer` local: the controllers of the player's own 3D.
        let kept_controllers = frame.addr() + 0x348;
        e.call(0x0063_3c90, &args![kept_controllers, controllers]);
        e.call(0x00a5_c050, &args![own]);
        // `NiCloningProcess` local, built with scale 1.0.
        let cloning = frame.addr() + 0xc8;
        e.call(0x004a_d050, &args![cloning, 1.0f32]);
        let clone = e.call(0x00a5_d2c0, &args![own, cloning]).u32();
        let clone_node = if clone != 0 {
            e.vcall(clone, 0xc, &args![]).u32()
        } else {
            0
        };
        if clone_node != 0 {
            let scene = e.call(0x0045_0b80, &args![1u32]).u32();
            e.call(0x00b5_f150, &args![scene, clone_node, 0u32]);
        }
        let held = pointer_target(e, kept_controllers);
        e.call(0x00a5_c000, &args![own, held]);
        let clone_node = if clone != 0 {
            e.vcall(clone, 0xc, &args![]).u32()
        } else {
            0
        };
        e.call(SMART_POINTER_ASSIGN, &args![slot, clone_node]);
        let model = pointer_target(e, slot);
        e.call(0x00a5_c050, &args![model]);
        let model = pointer_target(e, slot);
        e.call(0x0049_9160, &args![model]);
        let property = e.call(0x0040_df90, &args![]).u32();
        let model = pointer_target(e, slot);
        e.call(0x00a5_b230, &args![model, property]);
        let model = pointer_target(e, slot);
        e.call(0x00a5_a040, &args![model]);
        let model = pointer_target(e, slot);
        e.call(0x00b6_bb30, &args![model, 1.0f32]);
        let model = pointer_target(e, slot);
        fn_00951940(e, this.cast(), model);
        let model = pointer_target(e, slot);
        e.call(0x0044_0460, &args![model, INVENTORY_MODEL_ARGUMENT]);
        let model = pointer_target(e, slot);
        let form = e.call(0x007a_f430, &args![player]).u32();
        e.call(0x0060_62e0, &args![form, player, model]);
        let matrix = frame.addr() + 0x204;
        let rotated_out = frame.addr() + 0x80;
        for word in 0..9u32 {
            let value = e.mem.u32(LOAD_3D_MATRIX + 4 * word);
            e.mem.set_u32(matrix + 4 * word, value);
        }
        let rotated = e
            .call(0x0056_fac0, &args![player, rotated_out, matrix])
            .u32();
        let model = pointer_target(e, slot);
        e.call(0x0043_fa80, &args![model, rotated]);
        let block = e.call(0x00aa_13e0, &args![0x1c0u32]).u32();
        let made = if block != 0 {
            e.call(0x0064_9680, &args![block]).u32()
        } else {
            0
        };
        // `NiPointer` local holding the new object.
        let holder = frame.addr() + 0x354;
        e.call(0x0063_3c90, &args![holder, made]);
        let model = pointer_target(e, slot);
        let name = e.call(0x008d_3f70, &args![]).u32();
        let found = e.vcall(model, 0x9c, &args![name]).u32();
        let cast = e.call(0x0065_3270, &args![CAST_TYPE_011D5BF8, found]).u32();
        if cast != 0 {
            let held = pointer_target(e, holder);
            e.vcall(cast, 0x104, &args![held]);
        }
        let model = pointer_target(e, slot);
        let name = e.call(0x008d_3f80, &args![]).u32();
        let found = e.vcall(model, 0x9c, &args![name]).u32();
        let cast = e.call(0x0065_3270, &args![CAST_TYPE_011D5BF8, found]).u32();
        if cast != 0 {
            let held = pointer_target(e, holder);
            e.vcall(cast, 0x104, &args![held]);
            e.call(0x0066_3050, &args![cast, 1u32]);
        }
        e.call(SMART_POINTER_ASSIGN, &args![holder, 0u32]);
        // The children of the nodes named by `00951920` and `00499b70`.
        for name_getter in [0u32, 1u32] {
            let model = pointer_target(e, slot);
            let name = if name_getter == 0 {
                fn_00951920(e)
            } else {
                e.call(0x0049_9b70, &args![]).u32()
            };
            let found = e.vcall(model, 0x9c, &args![name]).u32();
            let node = if found != 0 {
                e.vcall(found, 0xc, &args![]).u32()
            } else {
                0
            };
            if node != 0 {
                let mut index = 0u32;
                while index < e.call(0x0043_b480, &args![node]).u32() {
                    e.vcall(node, 0xf0, &args![index]);
                    index += 1;
                }
            }
        }
        let model = pointer_target(e, slot);
        e.call(0x00b6_8770, &args![model, 0u32, 0.0f32, 0u32, 0.0f32, 0u32]);
        let block = e.call(OPERATOR_NEW, &args![0x13cu32]).u32();
        let animation = if block != 0 {
            e.call(0x0048_f810, &args![block]).u32()
        } else {
            0
        };
        e.mem.set_u32(player + 0x6a0, animation);
        e.call(0x007a_f430, &args![player]);
        let model_name = e.call(0x0057_15d0, &args![player]).u32();
        let folder = frame.addr() + 0xf8;
        e.call(
            0x0040_6d00,
            &args![folder, 0x104u32, FORMAT_MODEL_PATH, model_name],
        );
        let separator = e.call(0x0040_ab30, &args![folder, 0x5cu32]).u32();
        if separator == 0 {
            e.call(0x0045_cec0, &args![holder]);
            e.call(0x004a_d270, &args![cloning]);
            e.call(0x0045_cec0, &args![kept_controllers]);
            return true;
        }
        let remaining = 0x104u32.wrapping_sub(separator.wrapping_sub(folder));
        e.call(0x0040_6d30, &args![separator, remaining, TEXT_MT_IDLE]);
        let path = frame.addr() + 0x238;
        e.call(
            0x0040_6d00,
            &args![path, 0x104u32, FORMAT_DATA_PATH, TEXT_MESHES, folder],
        );
        let model_name = e.call(0x0057_15d0, &args![player]).u32();
        let loader = e.global::<u32>(MODEL_LOADER_POINTER);
        let files = e
            .call(0x0044_7300, &args![loader, path, model_name, 0u32])
            .u32();
        e.mem.set_u8(separator, 0);
        for index in 1..12u32 {
            let idle_name = e.mem.u32(IDLE_NAME_TABLE + 4 * index);
            for (path_format, name_format, word_at) in [
                (FORMAT_IDLE_PATH, FORMAT_IDLE_NAME, 0xb4),
                (FORMAT_TORCH_IDLE_PATH, FORMAT_TORCH_IDLE_NAME, 0xb0),
            ] {
                e.call(
                    0x0040_6d00,
                    &args![path, 0x104u32, path_format, TEXT_MESHES, folder, idle_name],
                );
                let files_object = e.call(0x0050_f6e0, &args![]).u32();
                let exists = e
                    .call(
                        0x00af_e0d0,
                        &args![files_object, path, 0u32, 0u32, 0xffff_ffffu32],
                    )
                    .u32();
                if exists != 0 {
                    let copy = e.call(OPERATOR_NEW, &args![0x104u32]).u32();
                    let model_name = e.call(0x0057_15d0, &args![player]).u32();
                    e.call(0x0040_6d30, &args![copy, 0x104u32, model_name]);
                    let backslash = e.call(0x0040_ab30, &args![copy, 0x5cu32]).u32();
                    if backslash != 0 {
                        let name_at = backslash + 1;
                        let remaining =
                            0x104u32.wrapping_sub(name_at.wrapping_sub(copy).wrapping_add(1));
                        e.call(
                            0x0040_6d00,
                            &args![name_at, remaining, name_format, idle_name],
                        );
                        let word = frame.addr() + word_at;
                        e.mem.set_u32(word, copy);
                        e.call(0x0090_5820, &args![files, word]);
                    }
                }
            }
        }
        let animation = e.mem.u32(player + 0x6a0);
        let model = pointer_target(e, slot);
        e.call(0x0048_ffd0, &args![animation, files, model, player, 1u32]);
        let extra = e.call(0x005d_43c0, &args![player]).u32();
        e.call(0x0041_8250, &args![extra]);
        let model = pointer_target(e, slot);
        e.vcall(model, 0xbc, &args![]);
        if e.call(0x008d_8520, &args![player]).u32() != 0 {
            let process = e.call(0x008d_8520, &args![player]).u32();
            e.vcall(process, 0x580, &args![1u32, 0u32, 0u32]);
        }
        fn_00951910(e, saved_flag);
        let model = pointer_target(e, slot);
        let name = fn_00951930(e);
        let found = e.vcall(model, 0x9c, &args![name]).u32();
        if found != 0 {
            e.call(0x0045_0f90, &args![found, 1u32]);
        }
        e.call(0x0045_cec0, &args![holder]);
        e.call(0x004a_d270, &args![cloning]);
        e.call(0x0045_cec0, &args![kept_controllers]);
        false
    })
}

/// The destroying branch of `CloneInventory3D`.
fn clone_inventory_3d_destroy(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let player = this.addr();
    let slot = player + 0x69c;
    let animation = e.get(this, PlayerCharacter::pInventoryAnimation).addr();
    if animation != 0 {
        e.call(DELETE_ANIMATION, &args![animation, 1u32]);
    }
    e.set(this, PlayerCharacter::pInventoryAnimation, Ptr::NULL);
    for name_getter in [0x008d_3f70u32, 0x008d_3f80u32] {
        let model = pointer_target(e, slot);
        let name = e.call(name_getter, &args![]).u32();
        let found = e.vcall(model, 0x9c, &args![name]).u32();
        let cast = e.call(0x0065_3270, &args![CAST_TYPE_011D5BF8, found]).u32();
        if cast != 0 {
            e.call(0x0066_3050, &args![cast, 1u32]);
        }
    }
    e.call(SMART_POINTER_ASSIGN, &args![slot, 0u32]);
    let object = e.call(0x0082_0bf0, &args![player]).u32();
    if object != 0 {
        e.vcall(object, 0, &args![1u32]);
        e.call(0x0092_9220, &args![player, 0u32]);
    }
}

// Translated from 00951900 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `0x011fa00c` (set around `CloneInventory3D`).
pub fn fn_00951900(e: &mut Engine) -> u8 {
    e.global::<u8>(INVENTORY_CLONE_FLAG)
}

// Translated from 00951910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte at `0x011fa00c`.
pub fn fn_00951910(e: &mut Engine, value: u8) {
    e.set_global(INVENTORY_CLONE_FLAG, value);
}

// Translated from 00951920 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `0x011c623c` (a node name).
pub fn fn_00951920(e: &mut Engine) -> u32 {
    e.global::<u32>(NODE_NAME_011C623C)
}

// Translated from 00951930 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `0x011c6274` (a node name).
pub fn fn_00951930(e: &mut Engine) -> u32 {
    e.global::<u32>(NODE_NAME_011C6274)
}

// Translated from 00951940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Strips a node tree for the inventory view: for `node` (a `NiNode`) and,
/// recursively, every child: removes the controllers that `0043b300` does
/// not accept (the table at `0x011f36ec`), clears the collision object
/// (`NiAVObject::SetCollisionObject`, `0062bc90` with 0), and un-culls
/// the child (`00450f90(child, 0)`); recursion goes through the child's
/// virtual slot `0xc`. `ECX` (`this`) is not used.
pub fn fn_00951940(e: &mut Engine, _this: Ptr, node: u32) {
    if node == 0 {
        return;
    }
    let mut controller = e.call(0x0043_b230, &args![node]).u32();
    while controller != 0 {
        let next = e.call(0x004a_8a90, &args![controller]).u32();
        if !e
            .call(0x0043_b300, &args![0x011f_36ecu32, controller])
            .bool()
        {
            e.call(0x00a5_c480, &args![node, controller]);
        }
        controller = next;
    }
    e.call(0x0062_bc90, &args![node, 0u32]);
    let mut index = 0u32;
    while index < e.call(0x0043_b480, &args![node]).u32() {
        let child = e.call(0x0043_b4a0, &args![node, index]).u32();
        if child != 0 {
            e.call(0x0045_0f90, &args![child, 0u32]);
            let child_node = e.vcall(child, 0xc, &args![]).u32();
            if child_node != 0 {
                fn_00951940(e, _this, child_node);
            }
        }
        index += 1;
    }
}

// Translated from 00951a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Switches the player between the first-person model and the player's own
/// 3D: `hide_first_person` non-zero shows the own 3D. Does nothing unless
/// both models exist, and (when `hide_first_person` is non-zero) the player
/// cannot use the temporary camera (slot `0x22c`, `bTemp3rdPerson`).
/// Culls and un-culls the two models, picks the scope visibility from the
/// weapon (iron sights), places the first-person model's matrix, moves the
/// weapon sounds (idle and attack handles) between the two, swaps the
/// animation controllers' sequences when `008c4560` says so, lets the
/// lighting follow (`008b0bd0`) and refreshes the process (slot `0x580`).
/// C++ exception unwinding is not translated.
pub fn fn_00951a10(e: &mut Engine, this: Ptr<PlayerCharacter>, hide_first_person: u8) {
    let player = this.addr();
    let flag = hide_first_person;
    if fn_00950bb0(e, this, 1) == 0 || fn_00950bb0(e, this, 0) == 0 {
        return;
    }
    if flag != 0
        && (e.vcall(player, 0x22c, &args![0u32]).bool()
            || e.get(this, PlayerCharacter::bTemp3rdPerson) != 0)
    {
        return;
    }
    let own = fn_00950bb0(e, this, 0);
    let own_flag = e.call(0x0045_6610, &args![own]).u8();
    let first = fn_00950bb0(e, this, 1);
    let first_flag = e.call(0x0045_6610, &args![first]).u8();
    if own_flag != first_flag {
        if flag == 0 {
            let own = fn_00950bb0(e, this, 0);
            if e.call(0x0045_6610, &args![own]).u8() == 0 {
                return;
            }
        }
        if flag != 0 {
            let first = fn_00950bb0(e, this, 1);
            if e.call(0x0045_6610, &args![first]).u8() == 0 {
                e.mem.set_u8(player + 0x64b, 0);
                return;
            }
        }
    }
    // The exe's frame (`SUB ESP,0xb8` under the exception record, then `ESI` and the record
    // link): the scope float at `EBP-0x1c` (+0xb0), the rotation at `EBP-0x48` (+0x84), the
    // rotated matrix at `EBP-0xac` (+0x20), the zero vector at `EBP-0x60` (+0x6c), the sound
    // handle at `EBP-0x54` (+0x78) when hiding and at `EBP-0x6c` (+0x60) otherwise, and the
    // controller pointer at `EBP-0x88` (+0x44).
    e.with_stack(0xcc, |e, frame| {
        let own = fn_00950bb0(e, this, 0);
        e.call(0x0045_0f90, &args![own, u32::from(flag)]);
        let mut show_scope = false;
        let mut handled = false;
        if flag != 0 && e.mem.u8(player + 0x64f) != 0 && e.call(0x008b_bc10, &args![player]).bool()
        {
            let process = process_of(e, this);
            if e.vcall(process, 0x148, &args![]).u32() != 0 {
                let process = process_of(e, this);
                let item = e.vcall(process, 0x148, &args![]).u32();
                let form = e.call(0x0044_ddc0, &args![item]).u32();
                let form = e.call(0x0050_4e60, &args![form]).u32();
                if e.call(0x0048_cee0, &args![form]).u32() != 0 {
                    handled = true;
                    let process = process_of(e, this);
                    let item = e.vcall(process, 0x148, &args![]).u32();
                    let form = e.call(0x0044_ddc0, &args![item]).u32();
                    let local = frame.addr() + 0xb0;
                    e.mem.set_f32(local, 0.0);
                    if !e.call(0x004a_d030, &args![form]).bool() {
                        show_scope = true;
                    } else {
                        let process = process_of(e, this);
                        let item = e.vcall(process, 0x148, &args![]).u32();
                        if e.call(0x004b_d8d0, &args![item, 0xeu32, local]).bool() {
                            show_scope = true;
                        }
                    }
                }
            }
        }
        if show_scope {
            e.call(0x0070_9c40, &args![1u32]);
        }
        if !handled {
            let first = fn_00950bb0(e, this, 1);
            e.call(0x0045_0f90, &args![first, u32::from(flag == 0)]);
        }
        e.mem.set_u8(player + 0x64b, u8::from(flag == 0));
        if flag != 0 {
            let own = fn_00950bb0(e, this, 0);
            let argument = e.vcall(player, 0x1f4, &args![]).u32();
            e.call(0x0044_0460, &args![own, argument]);
            let rotation = frame.addr() + 0x84;
            let rotated_out = frame.addr() + 0x20;
            e.call(0x0068_15c0, &args![rotation]);
            let angle = e.vcall(player, 0x2bc, &args![0u32]).f64() as f32;
            e.call(0x004a_0c90, &args![rotation, angle]);
            let rotated = e
                .call(0x0056_fac0, &args![player, rotated_out, rotation])
                .u32();
            e.call(0x0043_fa80, &args![own, rotated]);
            e.set_global(INVENTORY_ALPHA, 1.0f32);
            e.call(0x008c_4640, &args![player]);
            let animation = player_character_get_animation(e, this, 0);
            if animation != 0 {
                e.call(0x0049_3900, &args![animation, player]);
            } else {
                let vector = frame.addr() + 0x6c;
                e.call(0x0043_d410, &args![vector, 0.0f32, 0u32, 0u32]);
                e.call(0x00a5_9c60, &args![own, vector]);
            }
        }
        // The weapon sounds: the idle and the attack handle of the extra data.
        let handle = frame.addr() + if flag != 0 { 0x78 } else { 0x60 };
        e.call(0x0041_a250, &args![handle]);
        for getter in [0x0041_89c0u32, 0x0041_8a00u32] {
            let extra = e.call(0x005d_43c0, &args![player]).u32();
            e.call(getter, &args![extra, handle]);
            if e.call(0x00ad_8ce0, &args![handle]).bool() {
                e.call(0x00ad_8fa0, &args![handle, u32::from(flag != 0)]);
            }
        }
        e.call(0x0048_3710, &args![handle]);
        e.vcall(player, 0x210, &args![0u32]);
        if e.call(0x008c_4560, &args![player]).bool() {
            let own_animation = player_character_get_animation(e, this, u8::from(flag == 0));
            let other_animation = player_character_get_animation(e, this, flag);
            let mut sequences = [0u32; 2];
            for (index, animation) in [own_animation, other_animation].into_iter().enumerate() {
                let manager = e.call(0x0049_6940, &args![animation]).u32();
                let manager = e.call(0x0053_7bd0, &args![manager]).u32();
                let name = e.call(0x0049_9b70, &args![]).u32();
                let found = e.vcall(manager, 0x8c, &args![name]).u32();
                sequences[index] = if found != 0 {
                    e.vcall(found, 0xc, &args![]).u32()
                } else {
                    0
                };
            }
            let (from, to) = (sequences[0], sequences[1]);
            if from != 0 && to != 0 {
                let mut index = 0u32;
                while index < e.call(0x0043_b480, &args![from]).u32() {
                    let pointer = frame.addr() + 0x44;
                    e.call(0x0063_3c90, &args![pointer, 0u32]);
                    e.vcall(from, 0xec, &args![index, pointer]);
                    if e.call(0x0052_aa80, &args![pointer, 0u32]).bool() {
                        let target = pointer_target(e, pointer);
                        e.vcall(to, 0xdc, &args![target, 1u32]);
                    }
                    e.call(0x0045_cec0, &args![pointer]);
                    index += 1;
                }
            }
        }
        let mut lighting = e.vcall(player, 0x1d0, &args![]).u32();
        if flag != 0 {
            lighting = e.call(0x0043_b4a0, &args![lighting, 0u32]).u32();
        }
        e.call(0x0048_3710, &args![player]);
        e.call(0x008b_0bd0, &args![player, lighting]);
        let process = e.call(0x008d_8520, &args![player]).u32();
        e.vcall(process, 0x580, &args![1u32, 1u32, 0u32]);
    });
}

// Translated from 009520f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Plays an animation group of the first-person animation: `group` (the
/// high bit is dropped) must have a type (`005f2440` not `0xff`). If the
/// type's table entry says so and the sequences of the first-person and the
/// player's animation can be synchronised (`00495da0`), that is all.
/// Otherwise plays the group itself when loaded, else its iron-sights
/// variant `group - 3` when loaded, else the group the player's own
/// selection returns (`00897910`) when its type is the same, or the
/// iron-sights variant of it.
pub fn fn_009520f0(e: &mut Engine, this: Ptr<PlayerCharacter>, group: u16, argument: u32) {
    let player = this.addr();
    let group = if group & 0x8000 != 0 {
        group & 0x7fff
    } else {
        group
    };
    let group_type = e.call(0x005f_2440, &args![u32::from(group)]).u32();
    if group_type == 0xff {
        return;
    }
    let animation = player_character_get_animation(e, this, 1);
    let entry = group_type.wrapping_mul(0x24);
    let mut synchronised = false;
    if e.mem.u8(GROUP_TYPE_TABLE_FLAG.wrapping_add(entry)) != 0 {
        let own = player_character_get_animation(e, this, 0);
        let word = e.mem.u32(GROUP_TYPE_TABLE_WORD.wrapping_add(entry));
        let sequence = e.call(0x0049_1040, &args![own, word]).u32();
        synchronised = e
            .call(0x0049_5da0, &args![animation, sequence, u32::from(group)])
            .u32()
            != 0;
    }
    if synchronised {
        return;
    }
    if e.call(0x0049_4710, &args![animation, u32::from(group)])
        .bool()
    {
        e.call(
            0x0049_4740,
            &args![
                animation,
                u32::from(group),
                argument,
                0xffff_ffffu32,
                0xffff_ffffu32
            ],
        );
        return;
    }
    let lower = u32::from(group).wrapping_sub(3);
    if e.call(0x005f_2720, &args![u32::from(group)]).bool()
        && e.call(0x0049_4710, &args![animation, lower]).bool()
    {
        e.call(
            0x0049_4740,
            &args![animation, lower, argument, 0xffff_ffffu32, 0xffff_ffffu32],
        );
        return;
    }
    // 00952216: `008a1710` takes no stack word (plain RET); the pushed animation
    // is the last word of `00897910` (RET 0x10).
    let weapon = e.call(0x008a_1710, &args![player]).u32();
    let chosen = e
        .call(
            0x0089_7910,
            &args![player, group_type, 0u32, u32::from(weapon == 0), animation],
        )
        .u16();
    let chosen_type = e.call(0x005f_2440, &args![u32::from(chosen)]).u32();
    let plays = chosen_type == group_type
        || (e.call(0x005f_2750, &args![group_type]).bool()
            && e.call(0x005f_2440, &args![u32::from(chosen)]).u32() == group_type.wrapping_sub(3));
    if plays {
        e.call(
            0x0049_4740,
            &args![
                animation,
                u32::from(chosen),
                argument,
                0xffff_ffffu32,
                0xffff_ffffu32
            ],
        );
    }
}

// Translated from 00952290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the first-person zoom factor (`0x011a3b68`) towards its target and
/// orients the first-person model. The target is the setting `0x011cdcb0`,
/// or `1.0` when the current animation group's type is in `0x18..=0xa8` (and
/// the animation action is not 4), the Pipboy is active, or the iron sights
/// are up. The factor moves by `frame time / duration * (target - factor)`
/// (duration: setting `0x011cd620`, or `0x011cde38` when the target is
/// `1.0`) and is clamped to the target. The look angle `0x011e0770` is the
/// angle between the player's look direction and the horizontal
/// (`004f6e40` of the clamped dot product), scaled by `1 - target`; the
/// matrix of the model (or of the node `0x011e07cc`) is rebuilt from it.
pub fn fn_00952290(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let player = this.addr();
    let at = e
        .call(SETTING_FLOAT_POINTER, &args![ZOOM_TARGET_SETTING])
        .u32();
    let mut target = e.mem.f32(at);
    let animation = player_character_get_animation(e, this, 0);
    if animation == 0 {
        return;
    }
    let group = e.call(0x0043_01b0, &args![animation, 4u32]).u16();
    let group_type = e.call(0x005f_2440, &args![u32::from(group)]).i32();
    let action = e.call(0x008a_7570, &args![player]).i32();
    if (action != 4 && (0x18..=0xa8).contains(&group_type))
        || (action != 4 && e.call(0x0096_7ae0, &args![player]).bool())
        || e.call(0x008b_bc10, &args![player]).bool()
    {
        target = 1.0;
    }
    let mut zoom = e.global::<f32>(ZOOM_FACTOR);
    if target != zoom {
        let at = e
            .call(SETTING_FLOAT_POINTER, &args![ZOOM_IN_SECONDS_SETTING])
            .u32();
        let mut duration = e.mem.f32(at);
        if f64::from(target) == e.global::<f64>(ONE_DOUBLE) {
            let at = e
                .call(SETTING_FLOAT_POINTER, &args![ZOOM_OUT_SECONDS_SETTING])
                .u32();
            duration = e.mem.f32(at);
        }
        let distance = f64::from(target) - f64::from(zoom);
        let frame = e.call(0x0084_d030, &args![TIMER_SOURCE_011F6394]).f64();
        let step = (frame / f64::from(duration) * distance) as f32;
        zoom = (f64::from(zoom) + f64::from(step)) as f32;
        e.set_global(ZOOM_FACTOR, zoom);
        let zero = e.global::<f64>(ZERO_DOUBLE);
        if (f64::from(step) < zero && target > zoom) || (f64::from(step) > zero && target < zoom) {
            e.set_global(ZOOM_FACTOR, target);
        }
        target = e.global::<f32>(ZOOM_FACTOR);
    }
    // Matrices (9 floats), vectors (3 floats) and the temporaries the game
    // keeps on its stack: the exe's frame (`SUB ESP,0x12c` under `EBP`, then `ESI` and `EDI`)
    // has them at `EBP-0x24`, `-0xc0`, `-0x8c`, `-0x68`, `-0xf0`, `-0x120`, `-0x98`, `-0x3c`
    // and `-0xfc`.
    e.with_stack(0x134, |e, frame| {
        let matrix_a = frame.addr() + 0x110;
        let matrix_b = frame.addr() + 0x74;
        let matrix_c = frame.addr() + 0xa8;
        let matrix_d = frame.addr() + 0xcc;
        let scratch_1 = frame.addr() + 0x44;
        let scratch_2 = frame.addr() + 0x14;
        let up = frame.addr() + 0x9c;
        let flat = frame.addr() + 0xf8;
        let rotated_up = frame.addr() + 0x38;
        e.call(0x0068_15c0, &args![matrix_a]);
        e.call(0x0068_15c0, &args![matrix_b]);
        let pitch = e.vcall(player, 0x2bc, &args![0u32]).f64() as f32;
        let node = e.global::<u32>(NODE_BY_EAD0_SLOT);
        if node != 0 {
            let look = e.call(0x0093_1d70, &args![player]).f32();
            e.call(0x0052_4ac0, &args![matrix_a, look]);
        } else {
            e.call(0x004a_0c90, &args![matrix_a, pitch]);
            let look = e.call(0x0093_1d70, &args![player]).f32();
            e.call(0x0052_4ac0, &args![matrix_b, look]);
            let product = e
                .call(0x0043_f8d0, &args![matrix_a, scratch_1, matrix_b])
                .u32();
            copy_words(e, product, matrix_a, 9);
        }
        e.call(0x0041_6870, &args![up, 0.0f32, 1.0f32, 0.0f32]);
        e.call(0x0068_15c0, &args![flat]);
        let rotated = e.call(0x004b_4500, &args![matrix_a, rotated_up, up]).u32();
        copy_words(e, rotated, up, 3);
        copy_words(e, up, flat, 3);
        e.mem.set_f32(flat + 8, 0.0);
        let mut dot = e.call(0x004b_6190, &args![up, flat]).f32();
        if f64::from(dot) < e.global::<f64>(MINUS_ONE_DOUBLE) {
            dot = e.global::<f32>(MINUS_ONE_FLOAT);
        }
        if f64::from(dot) > e.global::<f64>(ONE_DOUBLE) {
            dot = 1.0;
        }
        let angle = e.call(0x004f_6e40, &args![dot]).f32();
        e.set_global(LOOK_ANGLE, angle);
        let scaled = ((1.0 - f64::from(target)) * f64::from(angle)) as f32;
        e.set_global(LOOK_ANGLE, scaled);
        if e.mem.f32(up + 8) < e.mem.f32(flat + 8) {
            let flipped = (f64::from(scaled) * e.global::<f64>(MINUS_ONE_DOUBLE)) as f32;
            e.set_global(LOOK_ANGLE, flipped);
        }
        e.call(0x0068_15c0, &args![matrix_c]);
        e.call(0x0068_15c0, &args![matrix_d]);
        let angle = e.global::<f32>(LOOK_ANGLE);
        e.call(0x0052_4ac0, &args![matrix_c, angle]);
        copy_words(e, matrix_a, matrix_d, 9);
        let product = e
            .call(0x0043_f8d0, &args![matrix_d, scratch_2, matrix_c])
            .u32();
        copy_words(e, product, matrix_d, 9);
        if node != 0 {
            e.call(0x0043_fa80, &args![node, matrix_d]);
            e.call(0x004a_0c90, &args![matrix_a, pitch]);
            let model = pointer_target(e, player + 0x694);
            e.call(0x0043_fa80, &args![model, matrix_a]);
        } else {
            let model = pointer_target(e, player + 0x694);
            e.call(0x0043_fa80, &args![model, matrix_a]);
        }
    });
}

/// Copies `count` words from `from` to `to` (`REP MOVSD`).
fn copy_words(e: &mut Engine, from: u32, to: u32, count: u32) {
    for word in 0..count {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

// Translated from 009526a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::SetGodMode` (Xbox PDB): stores the byte `0x011e07ba`.
pub fn player_character_set_god_mode(e: &mut Engine, value: u8) {
    e.set_global(GOD_MODE_BYTE, value);
}

// Translated from 009526b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::IsGodMode` (Xbox PDB): 1 when the boolean setting
/// `0x011e0894` is on, else the byte `0x011e07ba`.
pub fn player_character_is_god_mode(e: &mut Engine) -> u8 {
    let at = e.call(0x0040_8d60, &args![GOD_MODE_SETTING]).u32();
    if e.mem.u8(at) != 0 {
        1
    } else {
        e.global::<u8>(GOD_MODE_BYTE)
    }
}

// Translated from 009526e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::SetDemigodMode` (Xbox PDB): stores the byte `0x011e07bb`.
pub fn player_character_set_demigod_mode(e: &mut Engine, value: u8) {
    e.set_global(DEMIGOD_MODE_BYTE, value);
}

// Translated from 009526f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::IsDemigodMode` (Xbox PDB): 1 when the boolean setting
/// `0x011e0894` is on, else the byte `0x011e07bb`.
pub fn player_character_is_demigod_mode(e: &mut Engine) -> u8 {
    let at = e.call(0x0040_8d60, &args![GOD_MODE_SETTING]).u32();
    if e.mem.u8(at) != 0 {
        1
    } else {
        e.global::<u8>(DEMIGOD_MODE_BYTE)
    }
}

// Translated from 00952720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `IsGodMode()` (`this` is not used).
pub fn fn_00952720(e: &mut Engine, _this: Ptr) -> u8 {
    player_character_is_god_mode(e)
}

// Translated from 00952730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the actor value `actor_value` may change by `delta`. Always true
/// unless slot `0x10` of the `MagicTarget` sub-object (`+0x94`) says yes;
/// then a loss (`delta < 0`) is refused for actor value `0x10` and `0x19` to
/// `0x1f`, and a gain (`delta > 0`) for `0x36`.
pub fn fn_00952730(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    actor_value: i32,
    delta: f32,
) -> bool {
    let target = this.addr() + MAGIC_TARGET;
    if !e.vcall(target, 0x10, &args![]).bool() {
        return true;
    }
    let zero = e.global::<f64>(ZERO_DOUBLE);
    let delta = f64::from(delta);
    if delta < zero {
        !(actor_value == 0x10 || (actor_value > 0x18 && actor_value <= 0x1f))
    } else {
        !(delta > zero && actor_value == 0x36)
    }
}

// Translated from 009527b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds every item of the list starting at `list` (a `BSSimpleList` node)
/// to the player's topic list with `00952830(item, 0, 1)` (no refresh each
/// time), and refreshes once (`0061a5a0(1, list head)`) if any was new.
pub fn fn_009527b0(e: &mut Engine, this: Ptr<PlayerCharacter>, list: u32) {
    if list == 0 {
        return;
    }
    let mut added = false;
    let mut node = list;
    while node != 0 {
        let item_slot = list_item_slot(e, node);
        if e.mem.u32(item_slot) == 0 {
            break;
        }
        let item_slot = list_item_slot(e, node);
        let item = e.mem.u32(item_slot);
        if fn_00952830(e, this, item, 0, 1) {
            added = true;
        }
        node = list_next(e, node);
    }
    if added {
        let head = e.call(0x0046_4e30, &args![this]).u32();
        e.call(0x0061_a5a0, &args![1u32, head]);
    }
}

// Translated from 00952830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `item` to the player's topic list (`+0x6a8`, whose head `00464e30`
/// returns) unless it is null or already in it (`005ae3d0` appends it);
/// when `refresh` is non-zero then refreshes the list (`0061a5a0(1, head)`).
/// Returns whether it was added. The fourth word is not read.
pub fn fn_00952830(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    item: u32,
    refresh: u8,
    _unused_3: u32,
) -> bool {
    if item == 0 {
        return false;
    }
    let mut node = e.call(0x0046_4e30, &args![this]).u32();
    while node != 0 {
        let item_slot = list_item_slot(e, node);
        if e.mem.u32(item_slot) == 0 {
            break;
        }
        let item_slot = list_item_slot(e, node);
        if e.mem.u32(item_slot) == item {
            return false;
        }
        node = list_next(e, node);
    }
    // The exe passes its own argument slot (`EBP+8`): `SUB ESP,0x8` under `EBP`, so the slot
    // is +0x10 of a frame that reaches it.
    e.with_stack(0x14, |e, frame| {
        let word = frame.addr() + 0x10;
        e.mem.set_u32(word, item);
        e.call(0x005a_e3d0, &args![this.addr() + TOPIC_LIST, word]);
    });
    if refresh != 0 {
        let head = e.call(0x0046_4e30, &args![this]).u32();
        e.call(0x0061_a5a0, &args![1u32, head]);
    }
    true
}

// Translated from 009528c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::AddQuestStageItem` (Xbox PDB): appends `item` to the
/// quest log (`+0x6b0`). Returns false for a null item, or when the log
/// already holds it and `0060d5e0` refuses the item's quest; otherwise
/// true. When scripts are processed (`005ac740`) and `0060fd70(item)` says
/// so, plays menu sound 9 and, if the item's quest is the player's
/// (`005cbb50`) and has a parent world, passes that to `0060c9c0`.
pub fn player_character_add_quest_stage_item(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    item: u32,
) -> bool {
    if item == 0 {
        return false;
    }
    let player = this.addr();
    let mut node = e.call(0x0060_da90, &args![player]).u32();
    let mut _same_quest = false;
    while node != 0 {
        let item_slot = list_item_slot(e, node);
        if e.mem.u32(item_slot) == 0 {
            break;
        }
        let item_slot = list_item_slot(e, node);
        let entry = e.mem.u32(item_slot);
        node = list_next(e, node);
        if entry == item {
            let quest = e.call(0x005e_3fa0, &args![entry]).u32();
            if !e.call(0x0060_d5e0, &args![quest]).bool() {
                return false;
            }
        }
        let entry_quest = e.call(0x005e_3fa0, &args![entry]).u32();
        let item_quest = e.call(0x005e_3fa0, &args![item]).u32();
        if entry_quest == item_quest {
            _same_quest = true;
        }
    }
    // The exe passes its own argument slot (`EBP+8`): `SUB ESP,0x10` and `ESI` under `EBP`, so
    // the slot is +0x1c of a frame that reaches it.
    e.with_stack(0x20, |e, frame| {
        let word = frame.addr() + 0x1c;
        e.mem.set_u32(word, item);
        e.call(0x005a_e3d0, &args![player + QUEST_LOG_LIST, word]);
    });
    if !e.call(0x005a_c740, &args![]).bool() {
        return true;
    }
    if e.call(0x0060_fd70, &args![item]).bool() {
        e.call(0x0070_6f30, &args![9u32]);
        let quest = e.call(0x005e_3fa0, &args![item]).u32();
        let own = e.global::<u32>(PLAYER_POINTER);
        let own_quest = e.call(0x005c_bb50, &args![own]).u32();
        if quest == own_quest && e.call(0x004f_d380, &args![item]).u32() != 0 {
            // 009529b7: `004fd380` takes no stack word (plain RET); the pushed 1 is
            // the word of `0060c9c0` (RET 4).
            let parent = e.call(0x004f_d380, &args![item]).u32();
            e.call(0x0060_c9c0, &args![parent, 1u32]);
        }
    }
    true
}

// Translated from 009529d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::SetActiveQuest` (Xbox PDB): stores `pActiveQuest`
/// (`+0x6b8`) when it differs; clearing it (a null quest) also clears the
/// quest targets (`00470470` on `+0x6c4`).
pub fn player_character_set_active_quest(e: &mut Engine, this: Ptr<PlayerCharacter>, quest: u32) {
    let player = this.addr();
    if quest != e.get(this, PlayerCharacter::pActiveQuest).addr() {
        e.set(this, PlayerCharacter::pActiveQuest, Ptr::new(quest));
        if quest == 0 {
            e.call(LIST_CLEAR, &args![player + QUEST_TARGET_LIST]);
        }
    }
}

// Translated from 00952a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `target` to the list `0079bc30` returns when it is not in it yet and
/// `005d43e0` of its `0044edb0` has a non-empty list (`008256d0`); if
/// the `0044edb0` of `target` is the active quest's (`+0x6b8`), calls
/// `005ec500(target, player + 0x6c4)`.
pub fn fn_00952a20(e: &mut Engine, this: Ptr<PlayerCharacter>, target: u32) {
    let player = this.addr();
    let mut node = e.call(0x0079_bc30, &args![player]).u32();
    let index = e.call(0x0044_edb0, &args![target]).u32();
    let mut _seen_index = false;
    let mut present = false;
    while node != 0 {
        let item_slot = list_item_slot(e, node);
        if e.mem.u32(item_slot) == 0 {
            break;
        }
        let item_slot = list_item_slot(e, node);
        let entry = e.mem.u32(item_slot);
        if target == entry {
            present = true;
            break;
        }
        if !_seen_index && e.call(0x0044_edb0, &args![entry]).u32() == index {
            _seen_index = true;
        }
        node = list_next(e, node);
    }
    let owner = e.call(0x0044_edb0, &args![target]).u32();
    let mut worth_adding = false;
    if e.call(0x005d_43e0, &args![owner]).u32() != 0 {
        let owner = e.call(0x0044_edb0, &args![target]).u32();
        let list = e.call(0x005d_43e0, &args![owner]).u32();
        worth_adding = !e.call(LIST_IS_EMPTY, &args![list]).bool();
    }
    if worth_adding && !present {
        // The exe passes its own argument slot (`EBP+8`): `SUB ESP,0x18` under `EBP`, so the
        // slot is +0x20 of a frame that reaches it.
        e.with_stack(0x24, |e, frame| {
            let word = frame.addr() + 0x20;
            e.mem.set_u32(word, target);
            let list = e.call(0x0079_bc30, &args![player]).u32();
            e.call(0x005a_e3d0, &args![list, word]);
        });
    }
    let owner = e.call(0x0044_edb0, &args![target]).u32();
    if owner == e.get(this, PlayerCharacter::pActiveQuest).addr() {
        e.call(0x005e_c500, &args![target, player + QUEST_TARGET_LIST]);
    }
}

// Translated from 00952b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the list at `+0x6bc` and returns the first entry whose `0044edb0`
/// equals the active quest (`+0x6b8`) and whose `007af430` is 1, or 0.
pub fn fn_00952b30(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u32 {
    let active = e.get(this, PlayerCharacter::pActiveQuest).addr();
    let mut node = this.addr() + 0x6bc;
    while node != 0 {
        let item_slot = list_item_slot(e, node);
        if e.mem.u32(item_slot) == 0 {
            break;
        }
        let item_slot = list_item_slot(e, node);
        let item = e.mem.u32(item_slot);
        if e.call(0x0044_edb0, &args![item]).u32() == active
            && e.call(0x007a_f430, &args![item]).u32() == 1
        {
            return item;
        }
        node = list_next(e, node);
    }
    0
}

// Translated from 00952ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::GetCurrentTargetList` (Xbox PDB): the quest target list
/// at `+0x6c4`, or 0 without an active quest. The list is rebuilt from the
/// one at `+0x6bc` (`0060f110(quest, +0x6c4, +0x6bc)`) when the dirty byte
/// at `+0x206` is set or the active quest's check `0060efd0` says it is
/// stale; the byte is then cleared.
pub fn player_character_get_current_target_list(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u32 {
    let player = this.addr();
    let quest = e.get(this, PlayerCharacter::pActiveQuest).addr();
    if quest == 0 {
        return 0;
    }
    let dirty = e.mem.u8(player + 0x206) != 0;
    if dirty
        || !e
            .call(0x0060_efd0, &args![quest, player + 0x6c4, player + 0x6bc])
            .bool()
    {
        e.call(0x0060_f110, &args![quest, player + 0x6c4, player + 0x6bc]);
        e.mem.set_u8(player + 0x206, 0);
    }
    player + 0x6c4
}

// Translated from 00952c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::CheckForQuestTargetUpdate` (Xbox PDB): marks the target
/// list dirty (`+0x206`) when `reference` is the player (after rebuilding the
/// path to the target at `+0x6f4` into `+0x6f8`), or when it is the
/// reference of one of the current quest targets (`006101b0(target, 1)`), or
/// when it holds an inventory (`0055d310`, `004bf220`) for which `004cfe20`
/// accepts its form id.
pub fn player_character_check_for_quest_target_update(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    reference: u32,
) {
    let player = this.addr();
    if reference == e.global::<u32>(PLAYER_POINTER) {
        let target = e.mem.u32(player + 0x6f4);
        if target != 0 {
            player_character_build_path_to_target(e, this, target, player + 0x6f8, 0);
        }
        e.mem.set_u8(player + 0x206, 1);
    }
    let mut list = player_character_get_current_target_list(e, this);
    while list != 0 {
        let item_slot = list_item_slot(e, list);
        if e.mem.u32(item_slot) == 0 || e.mem.u8(player + 0x206) != 0 {
            break;
        }
        let item_slot = list_item_slot(e, list);
        let target = e.mem.u32(item_slot);
        list = list_next(e, list);
        let found = e.call(0x0061_01b0, &args![target, 1u32]).u32();
        if found == reference {
            e.mem.set_u8(player + 0x206, 1);
        }
        if found != 0 && e.call(0x0055_d310, &args![found]).u32() != 0 {
            let container = e.call(0x0055_d310, &args![found]).u32();
            if container != 0 {
                let changes = e.call(0x004b_f220, &args![found]).u32();
                if changes != 0 {
                    let form_id = form_id_of(e, found);
                    if e.call(0x004c_fe20, &args![changes, form_id]).bool() {
                        e.mem.set_u8(player + 0x206, 1);
                    }
                }
            }
        }
    }
}

// Translated from 00952d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::BuildPathToTarget` (Xbox PDB): with a target location
/// and a `TeleportPath` (`path`), builds the low path from the player's
/// position to the target (`006d4f70`, mode 2) and clears the path again when
/// that fails. `this` and the third stack word are not read. The stack
/// objects are two `PathingLocation`s (`006dcd70`, destroyed by `004ff7e0`),
/// a `TeleportPath` (`006f48b0`, `006f4930`) and the lock data `00502670`
/// (no destructor runs); the exception frame is not translated.
pub fn player_character_build_path_to_target(
    e: &mut Engine,
    _this: Ptr<PlayerCharacter>,
    target: u32,
    path: u32,
    _unused_3: u32,
) {
    if target == 0 || path == 0 {
        return;
    }
    let player = e.global::<u32>(PLAYER_POINTER);
    e.with_stack(0xa0, |e, frame| {
        let target_location = frame.addr() + 0x28;
        let from_location = frame.addr();
        let lock_data = frame.addr() + 0x50;
        let scratch_path = frame.addr() + 0x5c;
        e.call(0x006d_cd70, &args![target_location, player]);
        e.call(0x006d_cd70, &args![from_location, target]);
        e.call(0x006f_48b0, &args![scratch_path]);
        e.call(0x0050_2670, &args![lock_data, player]);
        e.call(0x006f_4990, &args![path]);
        let built = e
            .call(
                0x006d_4f70,
                &args![target_location, from_location, path, lock_data, 2u32],
            )
            .bool();
        if !built {
            e.call(0x006f_4990, &args![path]);
        }
        e.call(0x006f_4930, &args![scratch_path]);
        e.call(0x004f_f7e0, &args![from_location]);
        e.call(0x004f_f7e0, &args![target_location]);
    });
}

// Translated from 00952e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::SetPlayerMapMarker` (Xbox PDB): creates the map marker
/// reference (`+0x6f4`, a `TESObjectREFR` of `0x68` bytes built by `0055a2f0`
/// and given the base object stored at `011ca248` by `00575690`) when there
/// is none, moves it to the position (`0049eea0`), puts it into the cell or
/// worldspace `place` (`0087ce80`; for a form of type `0x39` directly, for
/// type `0x41` the object `005f36f0` returns, when not null), and rebuilds
/// the path to it into `+0x6f8`. The exception frame is not translated.
pub fn player_character_set_player_map_marker(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    x: f32,
    y: f32,
    z: f32,
    place: u32,
) {
    let player = this.addr();
    if e.mem.u32(player + 0x6f4) == 0 {
        let block = e.call(OPERATOR_NEW, &args![0x68u32]).u32();
        let marker = if block == 0 {
            0
        } else {
            e.call(0x0055_a2f0, &args![block]).u32()
        };
        e.mem.set_u32(player + 0x6f4, marker);
        let base = e.global::<u32>(0x011c_a248);
        e.call(0x0057_5690, &args![marker, base]);
    }
    let marker = e.mem.u32(player + 0x6f4);
    e.with_stack(12, |e, position| {
        e.mem.set_f32(position.addr(), x);
        e.mem.set_f32(position.addr() + 4, y);
        e.mem.set_f32(position.addr() + 8, z);
        e.call(0x0049_eea0, &args![marker, position.addr()]);
    });
    if e.call(FORM_TYPE_OF, &args![place]).u32() == 0x39 {
        e.call(0x0087_ce80, &args![marker, place]);
    } else if e.call(FORM_TYPE_OF, &args![place]).u32() == 0x41 {
        let cell = e.call(0x005f_36f0, &args![place]).u32();
        if cell != 0 {
            e.call(0x0087_ce80, &args![marker, cell]);
        }
    }
    let marker = e.mem.u32(player + 0x6f4);
    player_character_build_path_to_target(e, this, marker, player + 0x6f8, 0);
}

// Translated from 00952f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::RemovePlayerMapMarker` (Xbox PDB): destroys the map
/// marker (`+0x6f4`) through its slot `0x10` with the delete flag, forgets
/// it and clears the path at `+0x6f8` (`006f4990`).
pub fn player_character_remove_player_map_marker(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let player = this.addr();
    let marker = e.mem.u32(player + 0x6f4);
    if marker != 0 {
        e.vcall(marker, 0x10, &args![1u32]);
    }
    e.mem.set_u32(player + 0x6f4, 0);
    e.call(0x006f_4990, &args![player + 0x6f8]);
}

// Translated from 00952ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes a three-float position into `out`: the three words `0045bb80`
/// returns for the node in `011e07d0` when the player is in first person
/// (`+0x64a` is 0) and that node exists; otherwise what `008a2fa0(player, out)`
/// writes. Returns `out`.
pub fn fn_00952ff0(e: &mut Engine, this: Ptr<PlayerCharacter>, out: u32) -> u32 {
    let node = e.global::<u32>(NODE_CAMERA_1ST_SLOT);
    if !e.get(this, PlayerCharacter::b3rdPerson) && node != 0 {
        let source = e.call(0x0045_bb80, &args![node]).u32();
        for i in 0..3 {
            let word = e.mem.u32(source + i * 4);
            e.mem.set_u32(out + i * 4, word);
        }
    } else {
        e.call(0x008a_2fa0, &args![this, out]);
    }
    out
}

// Translated from 00953060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::FocusOnActor` (Xbox PDB): turns the camera and the
/// player towards the head of `actor` (dialogue). `strength` is clamped to
/// `0..=1` and stored in `011e0d48` at the end (the previous call's blend
/// decides how fast the turn is: at `0` the angle limit `011e0d40` is
/// derived from the head's bound; below `1` the step is the frame time over
/// `(1 - blend) * setting`, capped at 1). Nothing happens without an actor,
/// or when the actor is in another cell (another world space outside).
/// The head is the node `008a30f0` names or the node named `Bip01 Speaker`;
/// an actor with neither logs once. With `skip_turn == 0` the pitch
/// (`00931e50`) and the yaw (`00931d30`, or `+0x6e4` when slot `0x214` says
/// so) are turned, then the first-person camera is refreshed. Without
/// face-gen animation data the field of view is set by `00950610` from the
/// distance. The exception frame is not translated.
pub fn player_character_focus_on_actor(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    actor: u32,
    strength: f32,
    skip_turn: u8,
) {
    let player = this.addr();
    if actor == 0 {
        return;
    }
    let mut blend = if strength < 0.0 { 0.0 } else { strength };
    if blend > 1.0 {
        blend = 1.0;
    }
    if e.global::<u8>(VANITY_ACTIVE) != 0 {
        player_character_stop_vanity_mode(e, this.cast());
        let hide = u8::from(!e.get(this, PlayerCharacter::b3rdPerson));
        fn_00951a10(e, this, hide);
    }
    let cell = e.call(PARENT_CELL_OF, &args![player]).u32();
    let mut interior = false;
    if cell != 0 {
        let cell = e.call(PARENT_CELL_OF, &args![player]).u32();
        interior = e.call(0x0042_5fd0, &args![cell]).bool();
    }
    if interior {
        let actor_cell = e.call(PARENT_CELL_OF, &args![actor]).u32();
        let player_cell = e.call(PARENT_CELL_OF, &args![player]).u32();
        if actor_cell != player_cell {
            return;
        }
    } else {
        let actor_world = e.call(0x0057_5d70, &args![actor]).u32();
        let player_world = e.call(0x0057_5d70, &args![player]).u32();
        if actor_world != player_world {
            return;
        }
    }
    e.with_stack(0x50, |e, frame| {
        // The frame: the vector from the head to the camera, the head
        // position, the head's bound, a zero point and two results.
        let delta = frame.addr();
        let head_position = frame.addr() + 0x0c;
        let bound = frame.addr() + 0x18;
        let zero_point = frame.addr() + 0x28;
        let angle_vector_a = frame.addr() + 0x34;
        let angle_vector_b = frame.addr() + 0x40;
        let camera_node = e.global::<u32>(NODE_CAMERA_1ST_SLOT);
        let camera_position = e.call(0x0045_bb80, &args![camera_node]).u32();
        for i in 0..3 {
            let word = e.mem.u32(camera_position + i * 4);
            e.mem.set_u32(delta + i * 4, word);
        }
        e.call(LIST_NODE_ITEM_SLOT, &args![head_position]);
        let mut has_face_animation = false;
        let mut head_found = true;
        let target = if e.call(0x0057_4900, &args![actor]).bool() {
            let base = e.call(0x005e_3fa0, &args![actor]).u32();
            e.vcall(base, 0x1d0, &args![]).u32()
        } else {
            e.vcall(actor, 0x1d0, &args![]).u32()
        };
        if target != 0 {
            let head = e.vcall(actor, 0x1b0, &args![0u32]).u32();
            let mut found = 0;
            if head == 0 {
                let head_name = e.call(0x008a_30f0, &args![]).u32();
                let looked_up = e.call(0x004a_de00, &args![target, head_name]).u32();
                found = e.call(0x0065_3270, &args![0x011f_4428u32, looked_up]).u32();
            }
            let block = e.call(OPERATOR_NEW, &args![4u32]).u32();
            let speaker_name = if block == 0 {
                0
            } else {
                e.call(0x0043_8170, &args![block, SPEAKER_NODE_NAME]).u32()
            };
            if found == 0 {
                let looked_up = e.call(0x004a_de00, &args![target, speaker_name]).u32();
                found = e.call(0x0065_3270, &args![0x011f_4428u32, looked_up]).u32();
            }
            if head == 0 && found == 0 {
                if e.global::<u32>(FOCUS_LAST_WARNED_ACTOR) != actor {
                    let model = e.call(0x0057_15d0, &args![actor]).u32();
                    e.call(LOG_MESSAGE, &args![FORMAT_FOCUS_WITHOUT_HEAD, model]);
                }
                e.set_global(FOCUS_LAST_WARNED_ACTOR, actor);
                head_found = false;
            } else {
                if head != 0 && e.call(0x0045_6610, &args![head]).bool() {
                    e.call(0x0045_0f90, &args![head, 0u32]);
                    e.call(0x0043_d410, &args![zero_point, 0u32, 0u32, 0u32]);
                    e.call(0x00a5_9c60, &args![head, zero_point]);
                    e.vcall(head, 0xbc, &args![]);
                    e.call(0x0045_0f90, &args![head, 1u32]);
                }
                e.call(0x0062_40d0, &args![bound]);
                if head != 0 {
                    let world_bound = e.call(0x0043_d450, &args![head]).u32();
                    e.call(0x004a_5250, &args![bound, world_bound]);
                } else {
                    let position = e.call(0x0045_bb80, &args![found]).u32();
                    e.call(0x0098_ddd0, &args![bound, position]);
                    let radius = e.global::<u32>(FOCUS_BOUND_RADIUS);
                    e.call(0x0063_f790, &args![bound, radius]);
                }
                let center = e.call(LIST_NODE_ITEM_SLOT, &args![bound]).u32();
                for i in 0..3 {
                    let word = e.mem.u32(center + i * 4);
                    e.mem.set_u32(head_position + i * 4, word);
                }
                let height = setting_float(e, HEAD_HEIGHT_SETTING);
                let z = e.mem.f32(head_position + 8);
                e.mem
                    .set_f32(head_position + 8, (f64::from(z) + f64::from(height)) as f32);
                e.call(0x0045_78c0, &args![delta, head_position]);
                let owner = e.call(0x0045_c670, &args![]).u32();
                let animation_data = e.call(0x0066_29f0, &args![owner]).u32();
                if animation_data != 0 {
                    has_face_animation = true;
                    focus_ease_view(e, this, bound, delta, blend);
                }
            }
            if skip_turn == 0 && head_found {
                focus_turn(e, this, delta, angle_vector_a, angle_vector_b, blend);
            }
            if !has_face_animation {
                let distance = e.call(0x0045_7990, &args![delta]).f64();
                let scaled = (distance - e.global::<f64>(FOCUS_DISTANCE_BASE))
                    / e.global::<f64>(HUNDRED)
                    * e.global::<f64>(FOCUS_DISTANCE_SCALE);
                let whole = e.call(FTOL, &args![scaled]).i32();
                fn_00950610(e, this, (whole + 0xf) as f32);
            }
        }
        e.set_global(FOCUS_BLEND, blend);
    });
}

/// The frame time: `0084d030` reads the float at `+0xc` of the timer object
/// `011f6394` (the game also pushes a `1.0`, which it ignores).
fn focus_frame_time(e: &mut Engine) -> f64 {
    e.call(0x0084_d030, &args![TIMER_SOURCE_011F6394]).f64()
}

/// A float setting (`00403e20` gives the address of its value).
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let at = e.call(SETTING_FLOAT_POINTER, &args![setting]).u32();
    e.mem.f32(at)
}

/// The step `min(frame_time / ((1 - blend) * setting), 1)` of `FocusOnActor`
/// (`0040ebd0` is `min`; its second word is the `1.0` the game pushed for
/// the frame-time getter, which ignores it).
fn focus_step(e: &mut Engine, blend: f32) -> f64 {
    let frame_time = focus_frame_time(e);
    let smoothing = setting_float(e, FOCUS_SMOOTHING_SETTING);
    let ratio = (frame_time / ((1.0 - f64::from(blend)) * f64::from(smoothing))) as f32;
    e.call(0x0040_ebd0, &args![ratio, 1.0f32]).f64()
}

/// The part of `FocusOnActor` that eases the view offsets `+0x670` and
/// `+0x674` towards the angle limit derived from the head's bound.
fn focus_ease_view(e: &mut Engine, this: Ptr<PlayerCharacter>, bound: u32, delta: u32, blend: f32) {
    let player = this.addr();
    if blend == 0.0 {
        let radius = e.call(0x0084_d030, &args![bound]).f64();
        let factor = setting_float(e, HEAD_DISTANCE_FACTOR_SETTING);
        let product = f64::from(factor) * radius;
        let distance = e.call(0x0045_7990, &args![delta]).f64();
        let ratio = (product / distance) as f32;
        e.set_global(FOCUS_RATIO, ratio);
        let angle = e.call(0x005d_c330, &args![ratio]).f64();
        let limit = (angle * e.global::<f64>(HUNDRED)) as f32;
        e.set_global(FOCUS_ANGLE_LIMIT, limit);
        let maximum = setting_float(e, FOCUS_MAXIMUM_SETTING);
        if limit > maximum {
            e.set_global(FOCUS_ANGLE_LIMIT, maximum);
        }
    }
    let limit = e.global::<f32>(FOCUS_ANGLE_LIMIT);
    let first = e.mem.f32(player + 0x670);
    let second = e.mem.f32(player + 0x674);
    let first_diff = (f64::from(limit) - f64::from(first)) as f32;
    let second_diff = (f64::from(limit) - f64::from(second)) as f32;
    let step = focus_step(e, blend);
    let first_new = step * f64::from(first_diff) + f64::from(e.mem.f32(player + 0x670));
    e.mem.set_f32(player + 0x670, first_new as f32);
    let step = focus_step(e, blend);
    let second_new = step * f64::from(second_diff) + f64::from(e.mem.f32(player + 0x674));
    e.mem.set_f32(player + 0x674, second_new as f32);
}

/// Wraps an angle into `-pi..=pi` with the exe's constants (`0101ff40`,
/// `0101ff48`, `0101ff58`), storing as `float` after every step.
fn focus_wrap_angle(e: &mut Engine, mut angle: f32) -> f32 {
    let pi = e.global::<f64>(PI_DOUBLE);
    let two_pi = e.global::<f64>(TWO_PI_DOUBLE);
    let minus_pi = e.global::<f64>(MINUS_PI_DOUBLE);
    while f64::from(angle) > pi {
        angle = (f64::from(angle) - two_pi) as f32;
    }
    while f64::from(angle) < minus_pi {
        angle = (f64::from(angle) + two_pi) as f32;
    }
    angle
}

/// The turn by `turn` radians: `step * turn` below full blend, otherwise
/// `frame_time * (turn * rate)`.
fn focus_turn_amount(e: &mut Engine, blend: f32, turn: f32) -> f64 {
    if blend < 1.0 {
        focus_step(e, blend) * f64::from(turn)
    } else {
        let rate = setting_float(e, FOCUS_RATE_SETTING);
        let scaled = f64::from(turn) * f64::from(rate);
        focus_frame_time(e) * scaled
    }
}

/// The turning half of `FocusOnActor`: the pitch step, the yaw step, then the
/// first-person camera refresh.
fn focus_turn(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    delta: u32,
    angle_vector_a: u32,
    angle_vector_b: u32,
    blend: f32,
) {
    let player = this.addr();
    let degrees = e.global::<f64>(DEGREES_TO_RADIANS);
    let look_rate = (f64::from(e.mem.f32(player + 0x674))
        / f64::from(setting_float(e, FOCUS_MAXIMUM_SETTING))) as f32;
    // Pitch.
    let distance = e.call(0x0045_7990, &args![delta]).f64();
    let sine = (f64::from(e.mem.f32(delta + 8)) / distance) as f32;
    let wanted_pitch = e.call(0x004b_5510, &args![sine]).f64() as f32;
    let looking = e.call(0x0093_1d70, &args![player]).f64();
    let turn = (f64::from(wanted_pitch) - looking) as f32;
    let size = e.call(0x0040_8840, &args![turn]).f64();
    let threshold =
        f64::from(setting_float(e, PITCH_START_SETTING)) * degrees * f64::from(look_rate);
    if threshold < size {
        e.set_global(FOCUS_PITCH_TURNING, 1u8);
    }
    if blend < 1.0 {
        e.set_global(FOCUS_PITCH_TURNING, 1u8);
    }
    if e.global::<u8>(FOCUS_PITCH_TURNING) != 0 {
        let amount = focus_turn_amount(e, blend, turn) as f32;
        e.call(0x0093_1e50, &args![player, amount]);
        let size = e.call(0x0040_8840, &args![turn]).f64();
        let threshold =
            f64::from(setting_float(e, PITCH_STOP_SETTING)) * degrees * f64::from(look_rate);
        if threshold > size {
            e.set_global(FOCUS_PITCH_TURNING, 0u8);
        }
    }
    // Yaw.
    let vector = e.call(0x004a_0bd0, &args![delta, angle_vector_a]).u32();
    let z_angle = e.call(0x004b_13c0, &args![vector]).f64();
    let heading = e.vcall(player, 0x2bc, &args![0u32]).f64();
    let mut turn = focus_wrap_angle(e, (z_angle - heading) as f32);
    let size = e.call(0x0040_8840, &args![turn]).f64();
    let threshold = f64::from(setting_float(e, YAW_START_SETTING)) * degrees * f64::from(look_rate);
    if threshold < size {
        e.set_global(FOCUS_YAW_TURNING, 1u8);
    }
    if blend < 1.0 {
        e.set_global(FOCUS_YAW_TURNING, 1u8);
    }
    if e.global::<u8>(FOCUS_YAW_TURNING) != 0 {
        if e.vcall(player, 0x214, &args![]).u32() == 0 {
            let amount = focus_turn_amount(e, blend, turn) as f32;
            e.call(0x0093_1d30, &args![player, amount]);
        } else {
            let vector = e.call(0x004a_0bd0, &args![delta, angle_vector_b]).u32();
            let z_angle = e.call(0x004b_13c0, &args![vector]).f64();
            let heading = e.call(0x008b_d7b0, &args![player, 0u32]).f64();
            let current = e.mem.f32(player + 0x6e4);
            turn = focus_wrap_angle(e, ((z_angle - heading) - f64::from(current)) as f32);
            let amount = focus_turn_amount(e, blend, turn);
            let updated = amount + f64::from(e.mem.f32(player + 0x6e4));
            e.mem.set_f32(player + 0x6e4, updated as f32);
        }
        let size = e.call(0x0040_8840, &args![turn]).f64();
        let threshold =
            f64::from(setting_float(e, YAW_STOP_SETTING)) * degrees * f64::from(look_rate);
        if threshold > size {
            e.set_global(FOCUS_YAW_TURNING, 0u8);
        }
    }
    // The camera refresh.
    let heading = e.vcall(player, 0x2bc, &args![0u32]).f64();
    e.set_global(FOCUS_SAVED_HEADING, heading as f32);
    let looking = e.call(0x0093_1d70, &args![player]).f64();
    e.set_global(FOCUS_SAVED_LOOKING, looking as f32);
    player_character_set_first_person(e, this, 1);
    player_character_update_first_person_zoom(e, this);
    player_character_update_temp_3rd_person(e, this);
    let camera = e.mem.u32(player + 0x190);
    e.call(0x009e_a3b0, &args![camera, 0x3fu32]);
    e.call(0x00c6_6210, &args![]);
    e.mem.set_u8(player + 0x64a, 1);
    e.vcall(player, 0x1e0, &args![]);
    e.call(0x0089_5110, &args![player, 1.0f32, 1.0f32]);
    let animation = player_character_get_animation(e, this, 0);
    e.call(0x0088_85e0, &args![player, animation, 0.0f32]);
    e.mem.set_u8(player + 0x64a, 0);
    let frame_time = focus_frame_time(e) as f32;
    e.call(0x008d_3550, &args![player, frame_time]);
    let frame_time = focus_frame_time(e) as f32;
    let first_person = u8::from(e.mem.u8(player + 0x64a) == 0);
    let animation = player_character_get_animation(e, this, first_person);
    e.call(0x0088_85e0, &args![player, animation, frame_time]);
}

// Translated from 00953c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::GetNumberActorsInCombat` (Xbox PDB): `005a4320` of the
/// combat group at `+0xd64`, or 0 without one.
pub fn player_character_get_number_actors_in_combat(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
) -> u32 {
    let group = e.mem.u32(this.addr() + 0xd64);
    if group == 0 {
        0
    } else {
        e.call(0x005a_4320, &args![group]).u32()
    }
}

// Translated from 00953c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::IsPlayerCharacterInCombat` (Xbox PDB): returns the byte
/// at `+0xdf0` and, when `out` is not null, stores the byte at `+0xdf1` there.
pub fn player_character_is_player_character_in_combat(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    out: u32,
) -> u8 {
    let player = this.addr();
    if out != 0 {
        let flag = e.mem.u8(player + 0xdf1);
        e.mem.set_u8(out, flag);
    }
    e.mem.u8(player + 0xdf0)
}

// Translated from 00953c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Asks the `ProcessLists` singleton (`00971c30(this, 0x15, 0)`) for an
/// object; when it finds one, calls `004702f0(object, 1)` on it. Returns
/// whether an object was found.
pub fn fn_00953c80(e: &mut Engine, this: Ptr<PlayerCharacter>) -> bool {
    let found = e
        .call(0x0097_1c30, &args![PROCESS_LISTS, this, 0x15u32, 0u32])
        .u32();
    if found != 0 {
        e.call(0x0047_02f0, &args![found, 1u32]);
    }
    found != 0
}

// Translated from 00953ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::ResetPlayerGreetFlag` (Xbox PDB): clears the greet flag
/// (`+0x6cc`) and its timer (`+0x6d0`).
pub fn player_character_reset_player_greet_flag(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    e.mem.set_u8(this.addr() + 0x6cc, 0);
    e.mem.set_f32(this.addr() + 0x6d0, 0.0);
}

// Translated from 00953d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the greet flag once its timer (`+0x6d0`) is above the double at
/// `01020998` (5.0 in the exe's data) while the flag (`+0x6cc`) is set.
pub fn fn_00953d00(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let player = this.addr();
    if e.mem.u8(player + 0x6cc) != 0
        && f64::from(e.mem.f32(player + 0x6d0)) > e.global::<f64>(GREET_TIMEOUT)
    {
        player_character_reset_player_greet_flag(e, this);
    }
}

// Translated from 00953d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// While the greet flag (`+0x6cc`) is set, adds the frame time (`0084d030` of
/// the timer object) to the greet timer (`+0x6d0`).
pub fn fn_00953d40(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let player = this.addr();
    if e.mem.u8(player + 0x6cc) != 0 {
        let frame_time = focus_frame_time(e);
        let timer = e.mem.f32(player + 0x6d0);
        e.mem
            .set_f32(player + 0x6d0, (frame_time + f64::from(timer)) as f32);
    }
}

/// The start of the sit, sleep and get-up packages: when the player's path is
/// not complete (`008b3bb0`), clears the mover data (`009daf80` on the
/// mover at `+0x190`).
fn clear_mover_unless_pathing_complete(e: &mut Engine) {
    let player = e.global::<u32>(PLAYER_POINTER);
    if !e.call(0x008b_3bb0, &args![player]).bool() {
        let mover = e.mem.u32(player + 0x190);
        e.call(0x009d_af80, &args![mover]);
    }
}

// Translated from 00953d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::InitiateSitSleepPackage` (Xbox PDB): gives the player a
/// package of type 6 (`00670b90`, `00670fc0`) located at `reference` (a
/// `PackageLocation` of `0xc` bytes, built by `0067f030`, `0067f3c0`, handed
/// to `00671d30`, destroyed by `00670b30`) and runs it through slot `0x2f4`.
/// Before, sets `0093a5f0(true)` and, when the process's slot `0x3e4` says 7,
/// calls `00894cc0(player, 0)`. The exception frame is not translated.
pub fn player_character_initiate_sit_sleep_package(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    reference: u32,
) {
    let player = this.addr();
    clear_mover_unless_pathing_complete(e);
    e.call(0x0093_a5f0, &args![this, 1u32]);
    let process = process_of(e, this);
    if e.vcall(process, 0x3e4, &args![]).i32() == 7 {
        e.call(0x0089_4cc0, &args![this, 0u32]);
    }
    let package = e.call(0x0067_0b90, &args![6u32]).u32();
    e.call(0x0067_0fc0, &args![package, 6u32]);
    e.call(0x0082_6b40, &args![package, 0u32]);
    e.call(0x0082_6b90, &args![package, 1u32]);
    let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    let location = if block == 0 {
        0
    } else {
        e.call(0x0067_f030, &args![block]).u32()
    };
    e.call(0x0067_f3c0, &args![location, reference]);
    e.call(0x0067_1d30, &args![package, location]);
    if location != 0 {
        e.call(0x0067_0b30, &args![location, 1u32]);
    }
    e.call(0x0098_4f60, &args![package, 0u32]);
    e.vcall(player, 0x2f4, &args![package, 0u32, 1u32]);
}

// Translated from 00953ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::InitiateGetUpPackage` (Xbox PDB): clears the mover as
/// the sit package does, then `0093a6f0(this, true)` and `008a75a0(this)`.
pub fn player_character_initiate_get_up_package(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    clear_mover_unless_pathing_complete(e);
    e.call(0x0093_a6f0, &args![this, 1u32]);
    e.call(0x008a_75a0, &args![this]);
}

// Translated from 00953f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::GetHeading` (Xbox PDB): the actor's heading
/// (`008bd7b0(this, 0)`) plus the offset `+0x6e4`, clamped with `004b1480`.
/// The stack word is not read.
pub fn player_character_get_heading(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    _unused_1: u32,
) -> f32 {
    let heading = e.call(0x008b_d7b0, &args![this, 0u32]).f32();
    let sum = (f64::from(heading) + f64::from(e.mem.f32(this.addr() + 0x6e4))) as f32;
    e.call(0x004b_1480, &args![sum]).f32()
}

// Translated from 00953f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the murderer byte (`+0x6d8`) is set.
pub fn fn_00953f60(e: &mut Engine, this: Ptr<PlayerCharacter>) -> bool {
    e.mem.u8(this.addr() + 0x6d8) != 0
}

// Translated from 00953f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::SetIsAMurderer` (Xbox PDB): sets the murderer byte
/// (`+0x6d8`) and calls `008bff70(this, 4, 1)`.
pub fn player_character_set_is_a_murderer(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    e.mem.set_u8(this.addr() + 0x6d8, 1);
    e.call(0x008b_ff70, &args![this, 4u32, 1u32]);
}

// Translated from 00953fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x6e8`.
pub fn fn_00953fb0(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u8 {
    e.mem.u8(this.addr() + 0x6e8)
}

// Translated from 00953fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `+0x6e8`.
pub fn fn_00953fd0(e: &mut Engine, this: Ptr<PlayerCharacter>, value: u8) {
    e.mem.set_u8(this.addr() + 0x6e8, value);
}

// Translated from 00953ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The player takes `count` of the reference `item` (pick-up). The base
/// object (`007af430`) of form type `0x28` must pass `0047bcf0`, one of type
/// `0x29` must pass `004c94d0`, other types go on. Then: `008aded0(base, 1,
/// 0)`, `00c6a270(item's slot 0x1d0, 1, 1, 0)`; every actor of the
/// `ProcessLists` list `0096f450(form id, player)` is told with `00881620`
/// (the player when its `00881650` is the item, else null) and the list is
/// freed; unless a menu is visible (`00702680(0x3f1, 0)`, `00705020`,
/// `00705000`) the ownership is handled (stealing raises the alarm through
/// `008bfa40`, then the owner is set from or removed from the extra data);
/// a form of type `0x28` that is the process's current item (slot `0x14c`)
/// adds `count` to the count of the item in slot `0x148` and `0x14c`;
/// `00574b30` does the transfer; the weapon's ammo regeneration may start the
/// reload (slot `0x3ec`); a reference that was fully taken is destroyed
/// through its slot `0x10`.
pub fn fn_00953ff0(e: &mut Engine, this: Ptr<PlayerCharacter>, item: u32, count: u32, flag: u8) {
    let player = this.addr();
    let mut flag = flag;
    let mut touched_item = false;
    let base = e.call(0x007a_f430, &args![item]).u32();
    let kind = e.call(FORM_TYPE_OF, &args![base]).u32();
    if kind == 0x28 {
        let base = e.call(0x007a_f430, &args![item]).u32();
        if !e.call(0x0047_bcf0, &args![base]).bool() {
            return;
        }
    } else if kind == 0x29 {
        let base = e.call(0x007a_f430, &args![item]).u32();
        if !e.call(0x004c_94d0, &args![base]).bool() {
            return;
        }
    }
    let base = e.call(0x007a_f430, &args![item]).u32();
    e.call(0x008a_ded0, &args![this, base, 1u32, 0u32]);
    let slot = e.vcall(item, 0x1d0, &args![]).u32();
    e.call(0x00c6_a270, &args![slot, 1u32, 1u32, 0u32]);
    let form_id = form_id_of(e, item);
    let list = e
        .call(0x0096_f450, &args![PROCESS_LISTS, form_id, player])
        .u32();
    let mut node = list;
    while node != 0 {
        let item_slot = list_item_slot(e, node);
        if e.mem.u32(item_slot) == 0 {
            break;
        }
        let item_slot = list_item_slot(e, node);
        let actor = e.mem.u32(item_slot);
        if actor != 0 && e.call(0x0088_1650, &args![actor]).u32() == item {
            e.call(0x0088_1620, &args![actor, player]);
        } else {
            e.call(0x0088_1620, &args![actor, 0u32]);
        }
        node = list_next(e, node);
    }
    if list != 0 {
        e.call(LIST_CLEAR, &args![list]);
        e.call(0x0047_02f0, &args![list, 1u32]);
    }
    let file = e.call(0x0048_4e60, &args![item, 0xffff_ffffu32]).u32();
    if !e.call(0x0070_2680, &args![0x3f1u32, 0u32]).bool()
        && !e.call(0x0070_5020, &args![]).bool()
        && !e.call(0x0070_5000, &args![]).bool()
    {
        let owner = e.call(0x0056_7790, &args![item]).u32();
        let mut keep_ownership = false;
        if owner != 0 && !e.call(0x0057_85e0, &args![item, player, 1u32]).bool() {
            let base = e.call(0x007a_f430, &args![item]).u32();
            if e.call(FORM_TYPE_OF, &args![base]).u32() != 0x33 {
                let thief = e.call(0x0097_3ab0, &args![PROCESS_LISTS, item]).u32();
                e.set_global(STEAL_ACTOR, thief);
                if e.call(0x0046_0250, &args![item]).bool()
                    || file != 0
                    || e.call(0x0057_2d30, &args![item, 2u32]).bool()
                {
                    let base = e.call(0x007a_f430, &args![item]).u32();
                    e.call(0x008b_fa40, &args![this, item, base, count, 0u32, owner]);
                } else if thief != 0 {
                    let base = e.call(0x007a_f430, &args![item]).u32();
                    e.call(0x008b_fa40, &args![this, thief, base, count, 0u32, owner]);
                }
                let evil = e.call(0x0057_8790, &args![item, 0u32]).bool();
                let extra = e.call(EXTRA_LIST_OF, &args![item]).u32();
                if !evil {
                    e.call(0x0041_9700, &args![extra, owner]);
                } else {
                    e.call(0x0041_aed0, &args![extra]);
                }
                keep_ownership = true;
            }
        }
        if !keep_ownership {
            let extra = e.call(EXTRA_LIST_OF, &args![item]).u32();
            e.call(0x0041_aed0, &args![extra]);
        }
    } else {
        // A menu is visible: nothing about ownership is touched.
    }
    let process = process_of(e, this);
    let base = e.call(0x007a_f430, &args![item]).u32();
    let kind = e.call(FORM_TYPE_OF, &args![base]).u32();
    if kind == 0x28 {
        let base = e.call(0x007a_f430, &args![item]).u32();
        if e.call(0x004c_0bf0, &args![base]).bool() && e.vcall(process, 0x14c, &args![]).u32() != 0
        {
            let current = e.vcall(process, 0x14c, &args![]).u32();
            if base == e.call(WORD_AT_8, &args![current]).u32() {
                if e.vcall(process, 0x148, &args![]).u32() != 0 {
                    let entry = e.vcall(process, 0x148, &args![]).u32();
                    let total = e
                        .call(LIST_NODE_NEXT, &args![entry])
                        .u32()
                        .wrapping_add(count);
                    let entry = e.vcall(process, 0x148, &args![]).u32();
                    e.call(0x006e_cd40, &args![entry, total]);
                }
                if e.vcall(process, 0x14c, &args![]).u32() != 0 {
                    let entry = e.vcall(process, 0x14c, &args![]).u32();
                    let total = e
                        .call(LIST_NODE_NEXT, &args![entry])
                        .u32()
                        .wrapping_add(count);
                    let entry = e.vcall(process, 0x14c, &args![]).u32();
                    e.call(0x006e_cd40, &args![entry, total]);
                }
                touched_item = true;
            }
        }
    } else if kind == 0x29 {
        flag = 0;
    }
    let mut taken_whole = false;
    if e.call(0x0046_0250, &args![item]).bool() || file != 0 {
        e.call(0x0057_2230, &args![item]);
        e.call(
            0x0057_4b30,
            &args![this, item, count, u32::from(flag), u32::from(touched_item)],
        );
    } else if e.call(0x0057_2d30, &args![item, 2u32]).bool() {
        e.call(0x0057_2230, &args![item]);
        e.call(
            0x0057_4b30,
            &args![this, item, count, 0u32, u32::from(touched_item)],
        );
    } else {
        e.call(
            0x0057_4b30,
            &args![this, item, count, 0u32, u32::from(touched_item)],
        );
        taken_whole = true;
    }
    let mut skip = false;
    if e.vcall(process, 0x14c, &args![]).u32() != 0 {
        let entry = e.vcall(process, 0x14c, &args![]).u32();
        if e.call(LIST_NODE_NEXT, &args![entry]).u32() != 0 {
            skip = true;
        }
    }
    if !skip && e.vcall(process, 0x148, &args![]).u32() != 0 && item != 0 {
        let base = e.call(0x007a_f430, &args![item]).u32();
        let player_pointer = e.global::<u32>(PLAYER_POINTER);
        let entry = e.vcall(process, 0x148, &args![]).u32();
        let form = e.call(WORD_AT_8, &args![entry]).u32();
        let ammo = e.call(0x0052_5980, &args![form, player_pointer]).u32();
        if base == ammo {
            let entry = e.vcall(process, 0x148, &args![]).u32();
            let modded = e.call(0x004b_da70, &args![entry, 6u32]).u8();
            let entry = e.vcall(process, 0x148, &args![]).u32();
            let form = e.call(WORD_AT_8, &args![entry]).u32();
            let regeneration = e.call(0x0070_9430, &args![form, u32::from(modded)]).f64();
            if regeneration <= 0.0 {
                let entry = e.vcall(process, 0x148, &args![]).u32();
                let modded = e.call(0x004b_da70, &args![entry, 2u32]).u8();
                let drawn = e.call(0x008a_16d0, &args![this]).bool();
                let mode = if drawn { 2u32 } else { 0 };
                let entry = e.vcall(process, 0x148, &args![]).u32();
                let form = e.call(WORD_AT_8, &args![entry]).u32();
                e.vcall(player, 0x3ec, &args![form, mode, u32::from(modded), 0u32]);
            }
        }
    }
    if taken_whole && item != 0 {
        e.vcall(item, 0x10, &args![1u32]);
    }
}

// Translated from 00954610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Drops `count` of `item` (a base form) from the player: quits VATS
/// playback (`009c8950(011f2250, 0, 0)`), then, when `007043c0` says so and
/// the player has a parent cell, lets slot `0x17c` do the whole drop at the
/// position `00551110(cell)` or `00551180(cell, player)` gives (returning 0);
/// otherwise, for ammunition (type `0x29`) that is the current ammo of the
/// weapon `008a1710` returns, empties the ammo count and hides the two
/// `##NifRound` nodes of the first-person biped. The remembered item
/// (`+0x1f0`) is forgotten when it is the dropped one and worn (`00418ab0`).
/// The reference slot `0x17c` creates (the base form only when slot `0xe4`
/// of `item` says so) is placed at the physics body (`1d0`) or put on the
/// player's pending list (`+0x84c`) and flagged (`00954910`, `00564d20`);
/// `008aded0(base, 0, 0)` closes it. Returns the reference.
pub fn fn_00954610(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    item: u32,
    extra: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
) -> u32 {
    let player = this.addr();
    e.call(0x009c_8950, &args![OBJECT_011F2250, 0u32, 0u32]);
    let cell = e.call(PARENT_CELL_OF, &args![player]).u32();
    let mut skip_ammo = false;
    if e.call(0x0070_43c0, &args![]).bool() && cell != 0 {
        let mut position = e.call(0x0055_1110, &args![cell]).u32();
        if position == 0 {
            position = e.call(0x0055_1180, &args![cell, player]).u32();
        }
        if position != 0 {
            e.vcall(
                player,
                0x17c,
                &args![item, extra, third, 0u32, 0u32, position, 0u32, 0u32, 1u32, 0u32],
            );
            return 0;
        }
        skip_ammo = true;
    }
    if !skip_ammo
        && e.call(FORM_TYPE_OF, &args![item]).u32() == 0x29
        && e.call(0x008a_1710, &args![this]).u32() != 0
    {
        let weapon = e.call(0x008a_1710, &args![this]).u32();
        let player_pointer = e.global::<u32>(PLAYER_POINTER);
        if e.call(0x0052_5980, &args![weapon, player_pointer]).u32() == item {
            let process = process_of(e, this);
            let entry = e.vcall(process, 0x14c, &args![]).u32();
            e.call(0x006e_cd40, &args![entry, 0u32]);
            for _ in 0..2 {
                let biped = e.call(0x0095_0bb0, &args![this, 0u32]).u32();
                let node = e.call(0x004a_ae30, &args![biped, ROUND_NODE_NAME]).u32();
                if node != 0 {
                    e.call(0x0045_0f90, &args![node, 1u32]);
                }
            }
        }
    }
    let remembered = e.mem.u32(player + 0x1f0);
    if remembered != 0
        && item == remembered
        && extra != 0
        && e.call(0x0041_8ab0, &args![extra, 0u32]).bool()
    {
        e.mem.set_u32(player + 0x1f0, 0);
    }
    let selected = if e.vcall(item, 0xe4, &args![]).bool() {
        item
    } else {
        0
    };
    let reference = e
        .vcall(
            player,
            0x17c,
            &args![selected, extra, third, 0u32, 1u32, 0u32, fourth, fifth, 1u32, 0u32],
        )
        .u32();
    if reference != 0 {
        let body = e.vcall(reference, 0x1d0, &args![]).u32();
        if body != 0 {
            e.call(0x00c6_a040, &args![body, 0u32, 1u32, 1u32]);
            // The exe's frame (`SUB ESP,0x40` under `EBP`): the difference at `EBP-0x3c` (+4), the
            // zero point at `EBP-0x30` (+0x10), the position at `EBP-0x24` (+0x1c).
            e.with_stack(0x40, |e, frame| {
                let scratch = frame.addr() + 4;
                let zero_point = frame.addr() + 0x10;
                let position = frame.addr() + 0x1c;
                e.call(0x0043_d410, &args![zero_point, 0u32, 0u32, 0u32]);
                e.call(0x00a5_9c60, &args![body, zero_point]);
                e.call(0x00c6_a040, &args![body, 1u32, 1u32, 1u32]);
                let source = e.vcall(reference, 0x1f4, &args![]).u32();
                for i in 0..3 {
                    let word = e.mem.u32(source + i * 4);
                    e.mem.set_u32(position + i * 4, word);
                }
                let world_bound = e.call(0x0043_d450, &args![body]).u32();
                let center = e.call(LIST_NODE_ITEM_SLOT, &args![world_bound]).u32();
                e.call(0x0043_9ef0, &args![position, scratch, center]);
                e.call(0x0063_c8a0, &args![position, scratch]);
                e.call(0x0044_0460, &args![body, position]);
                e.call(0x0057_5830, &args![reference, position]);
                e.call(0x00c6_bd00, &args![body, 1u32]);
                e.call(0x00c6_a270, &args![body, 1u32, 1u32, 0u32]);
            });
        } else {
            // The same frame: the reference word at `EBP-0x8` (+0x38).
            e.with_stack(0x40, |e, frame| {
                let word = frame.addr() + 0x38;
                e.mem.set_u32(word, reference);
                e.call(0x005a_e3d0, &args![player + 0x84c, word]);
            });
            fn_00954910(e, Ptr::new(reference), 1);
            e.call(0x0056_4d20, &args![reference, 1u32]);
        }
        e.call(0x008a_ded0, &args![this, selected, 0u32, 0u32]);
    }
    reference
}

// Translated from 00954910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` non-zero) or clears bit `0x400000` of the form flags at `+8`.
pub fn fn_00954910(e: &mut Engine, this: Ptr, set: u8) {
    let flags = e.mem.u32(this.addr() + 8);
    let flags = if set != 0 {
        flags | 0x0040_0000
    } else {
        flags & 0xffbf_ffff
    };
    e.mem.set_u32(this.addr() + 8, flags);
}

// Translated from 00954960 (decompiled, FalloutNV.exe 1.4.0.525)
/// 1.0 unless god mode is on (`IsGodMode`), then `008c4610(this, flag, 0.0)`.
/// The second stack word is not read.
pub fn fn_00954960(e: &mut Engine, this: Ptr<PlayerCharacter>, flag: u8, _unused_2: u32) -> f32 {
    if e.call(IS_GOD_MODE, &args![]).bool() {
        e.call(0x008c_4610, &args![this, u32::from(flag), 0.0f32])
            .f32()
    } else {
        1.0
    }
}

// Translated from 009549a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `actor` is currently using the player (a sit or sleep package):
/// false when slot `0x22c(0)`, `00440da0`, slot `0x234` or `00437bf0` say so;
/// otherwise true when its package (`00881510`) is of kind 1 or 7 (`0041ca90`),
/// the acquire object of its process (`008d8520`) targets the player (slot
/// `0x128`) and `008a6210` is false. `this` is not used.
pub fn fn_009549a0(e: &mut Engine, _this: Ptr, actor: u32) -> u8 {
    if e.vcall(actor, 0x22c, &args![0u32]).bool()
        || e.call(0x0044_0da0, &args![actor]).bool()
        || e.vcall(actor, 0x234, &args![]).bool()
        || e.call(0x0043_7bf0, &args![actor]).bool()
    {
        return 0;
    }
    let package = e.call(0x0088_1510, &args![actor]).u32();
    if package == 0 {
        return 0;
    }
    if e.call(0x0041_ca90, &args![package]).u32() != 1
        && e.call(0x0041_ca90, &args![package]).u32() != 7
    {
        return 0;
    }
    let holder = e.call(0x008d_8520, &args![actor]).u32();
    let target = e.vcall(holder, 0x128, &args![]).u32();
    if target == e.global::<u32>(PLAYER_POINTER) && !e.call(0x008a_6210, &args![actor]).bool() {
        1
    } else {
        0
    }
}

// Translated from 00954a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the player is the reason `actor` stands in its current package
/// (`mode` selects the check; see the two branches). False first when slot
/// `0x22c(0)`, `00440da0`, slot `0x234` or `00437bf0` say so. Tells the
/// process's holder (`008d8520`) about the actor (slot `0x24(actor, 1)`).
/// `this` is not used.
pub fn fn_00954a70(e: &mut Engine, _this: Ptr, actor: u32, mode: u8) -> u8 {
    let player_pointer = e.global::<u32>(PLAYER_POINTER);
    if e.vcall(actor, 0x22c, &args![0u32]).bool()
        || e.call(0x0044_0da0, &args![actor]).bool()
        || e.vcall(actor, 0x234, &args![]).bool()
        || e.call(0x0043_7bf0, &args![actor]).bool()
    {
        return 0;
    }
    let holder = e.call(0x008d_8520, &args![actor]).u32();
    e.vcall(holder, 0x24, &args![actor, 1u32]);
    let package = e.call(0x0088_1510, &args![actor]).u32();
    let holder = e.call(0x008d_8520, &args![actor]).u32();
    let acquire = holder + 4;
    if mode != 0 {
        let mut target = 0;
        if acquire != 0 && e.call(WORD_AT_8, &args![acquire]).u32() != 0 {
            target = e.call(WORD_AT_8, &args![acquire]).u32();
            if target == 0 {
                let package_target = e.call(0x0067_1d10, &args![package]).u32();
                let object = e.call(0x0068_0050, &args![package_target]).u32();
                if object != 0 {
                    let player_base = e.call(0x007a_f430, &args![player_pointer]).u32();
                    if object == player_base {
                        target = player_pointer;
                    }
                }
            }
        }
        let location_is_player =
            package != 0 && e.call(0x0067_6140, &args![package, actor]).u32() == player_pointer;
        if !location_is_player {
            if target != player_pointer {
                let holder = e.call(0x008d_8520, &args![actor]).u32();
                if e.vcall(holder, 0x128, &args![]).u32() != player_pointer {
                    return 0;
                }
            }
            if e.call(0x0041_ca90, &args![package]).u32() == 9 {
                return 0;
            }
        }
        if target == player_pointer {
            u8::from(!e.call(0x008a_6210, &args![actor]).bool())
        } else {
            1
        }
    } else {
        if package == 0 {
            return 0;
        }
        if e.call(0x0041_ca90, &args![package]).u32() != 1
            && e.call(0x0041_ca90, &args![package]).u32() != 7
        {
            return 0;
        }
        if e.call(0x0067_1d10, &args![package]).u32() == 0 {
            return 0;
        }
        let holder = e.call(0x008d_8520, &args![actor]).u32();
        let acquire = holder + 4;
        let mut target = e.call(WORD_AT_8, &args![acquire]).u32();
        if target == 0 {
            let package_target = e.call(0x0067_1d10, &args![package]).u32();
            let object = e.call(0x0068_0050, &args![package_target]).u32();
            if object != 0 {
                let player_base = e.call(0x007a_f430, &args![player_pointer]).u32();
                if object == player_base {
                    target = player_pointer;
                }
            }
        }
        if target == player_pointer {
            u8::from(!e.call(0x008a_6290, &args![package]).bool())
        } else {
            0
        }
    }
}

// Translated from 00954cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `008a0c20(this)` is below the value of actor value `0x2e` (slot 8
/// of the `ActorValueOwner` at `+0xa4`); false in god mode.
pub fn fn_00954cc0(e: &mut Engine, this: Ptr<PlayerCharacter>) -> bool {
    if e.call(IS_GOD_MODE, &args![]).bool() {
        return false;
    }
    let owner = this.addr() + ACTOR_VALUE_OWNER;
    let limit = e.vcall(owner, 8, &args![0x2eu32]).i32();
    let value = e.call(0x008a_0c20, &args![this]).f64();
    value < f64::from(limit)
}

/// `GetSaveSize()` debug report of `fn_00954d40`: when the setting at
/// `011de4e8` is on, logs the size computed so far with the world space
/// record of the save-load object (form id, name through slot `0x130`, flags
/// at `+5`) or without it, and the source line.
fn save_size_report(e: &mut Engine, formats: (u32, u32), size: i32, line: u32) {
    let at = e.call(0x0040_8d60, &args![SAVE_SIZE_DEBUG_SETTING]).u32();
    if e.mem.u8(at) == 0 {
        return;
    }
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    let record = e.call(0x004f_d3e0, &args![save_load]).u32();
    if record != 0 {
        let form_id = e.mem.u32(record);
        let form_type = e.call(0x0048_39c0, &args![form_id]).u32();
        let name = e.vcall(form_type, 0x130, &args![]).u32();
        let flags = e.mem.u32(record + 5);
        e.call(
            ERROR,
            &args![
                formats.0,
                size as u32,
                form_id,
                name,
                flags,
                line,
                SOURCE_FILE_NAME
            ],
        );
    } else {
        e.call(
            ERROR,
            &args![formats.1, size as u32, line, SOURCE_FILE_NAME],
        );
    }
}

/// The save game version (`008df040` of the save-load object), a byte.
fn save_version(e: &mut Engine) -> u32 {
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    u32::from(e.call(0x008d_f040, &args![save_load]).u8())
}

/// `total + amount`, wrapped to 16 bits as the game's `word` accumulator.
fn add16(total: u16, amount: u32) -> u16 {
    u32::from(total).wrapping_add(amount) as u16
}

// Translated from 00954d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of bytes the player's save data takes for the save format
/// version, `flags` being the save flags (its bit `0x10000000` adds the
/// first-person animation's data, `0049aa20`). Every field of the format is
/// added with the version that introduced it (`008df040` of the save-load
/// object `011de45c`), the variable parts by their counts (`005ae380` of the
/// lists, `0084e3a0`, `008d32b0`, `00805a00`, `008d54e0`, `00609c70`,
/// `005f6ed0` and the length of the name `0055d520`). With the debug setting
/// `011de4e8` on, the sizes before and after the base part are logged
/// ("GetSaveSize(): ..."). The result is a 16-bit sum.
pub fn fn_00954d40(e: &mut Engine, this: Ptr<PlayerCharacter>, flags: u32) -> u16 {
    let player = this.addr();
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    let mut total: u16 = 0;
    if e.call(0x0086_2110, &args![save_load]).bool() {
        total = add16(total, 4);
        total = add16(total, 2);
    }
    total = add16(total, 0x134);
    total = add16(total, 0x134);
    if save_version(e) >= 0x31 {
        total = add16(total, 0x134);
    }
    total = add16(total, 4);
    save_size_report(e, SAVE_SIZE_FORMATS, i32::from(total), 0x3248);
    let base = e.call(0x008d_32b0, &args![this, flags]).u16();
    total = add16(total, u32::from(base));
    let after_base = total;
    if e.call(0x0086_2110, &args![save_load]).bool() {
        total = add16(total, 4);
        total = add16(total, 2);
    }
    if flags & 0x1000_0000 != 0 {
        let animation = e.mem.u32(player + 0x690);
        let size = e.call(0x0049_aa20, &args![this, animation]).u16();
        total = add16(total, u32::from(size));
    }
    // Fields present in every version, in the order of the format.
    for amount in [
        1u32, 1, 1, 1, 4, 4, 4, 4, 1, 1, 4, 4, 1, 4, 1, 4, 0xc, 4, 4, 4, 4, 1, 4, 1, 1, 4, 4, 1,
    ] {
        total = add16(total, amount);
    }
    if save_version(e) >= 0x28 && save_version(e) < 0x2d {
        total = add16(total, 0xc);
        total = add16(total, 0xc);
    }
    if save_version(e) >= 0x39 {
        total = add16(total, 0xac);
        total = add16(total, 0x14);
    }
    if save_version(e) >= 0x3f {
        total = add16(total, 1);
    }
    if save_version(e) >= 0x40 {
        total = add16(total, 4);
        total = add16(total, 4);
        total = add16(total, 4);
    }
    if save_version(e) >= 0x49 {
        total = add16(total, 4);
        total = add16(total, 1);
    }
    save_version(e);
    if save_version(e) >= 0x71 {
        total = add16(total, 1);
        total = add16(total, 1);
        total = add16(total, 4);
        total = add16(total, 4);
    }
    if save_version(e) >= 0x78 {
        total = add16(total, 1);
    }
    if save_version(e) >= 0x7a {
        total = add16(total, 4);
    }
    for _ in 0..10 {
        total = add16(total, 4);
    }
    if save_version(e) >= 0x28 && save_version(e) < 0x2d {
        total = add16(total, 4);
    }
    for (version, amount) in [
        (0x40u32, 4u32),
        (0x42, 4),
        (0x57, 4),
        (0x60, 4),
        (0x63, 2),
        (0x6c, 4),
    ] {
        if save_version(e) >= version {
            total = add16(total, amount);
        }
    }
    if save_version(e) >= 0x6f {
        total = add16(total, 2);
        let count = e.call(0x0084_e3a0, &args![player + 0x854]).u32();
        total = add16(total, count.wrapping_mul(5));
    }
    if save_version(e) >= 0x73 {
        total = add16(total, 2);
        let count = e.call(LIST_COUNT, &args![0x011e_0ae8u32]).u32();
        total = add16(total, count.wrapping_mul(4));
    }
    if save_version(e) >= 0x7a {
        total = add16(total, 2);
        let list = e.mem.u32(player + 0x610);
        let count = e.call(LIST_COUNT, &args![list]).u32();
        total = add16(total, count.wrapping_mul(8));
    }
    if save_version(e) >= 0x7a {
        total = add16(total, 2);
        let list = e.mem.u32(player + 0x614);
        let count = e.call(LIST_COUNT, &args![list]).u32();
        total = add16(total, count.wrapping_mul(4));
        total = add16(total, 2);
        let list = e.mem.u32(player + 0x618);
        let count = e.call(LIST_COUNT, &args![list]).u32();
        total = add16(total, count.wrapping_mul(4));
        total = add16(total, 0x14);
    }
    let size = e
        .call(0x0080_5a00, &args![e.mem.u32(player + 0x210), this])
        .u16();
    total = add16(total, u32::from(size));
    let size = e
        .call(0x008d_54e0, &args![player + CHARACTER_PROGRESSION, flags])
        .u16();
    total = add16(total, u32::from(size));
    total = add16(total, 4);
    total = add16(total, 2);
    let count = e.call(LIST_COUNT, &args![player + 0x6a8]).u32();
    total = add16(total, count.wrapping_mul(4));
    total = add16(total, 2);
    let count = e.call(LIST_COUNT, &args![player + 0x6b0]).u32();
    total = add16(total, count.wrapping_mul(6));
    let count = e.call(LIST_COUNT, &args![player + 0x6bc]).u32();
    total = add16(total, count.wrapping_mul(5));
    let extra = e.call(0x0040_8c30, &args![]).u32();
    total = add16(total, extra);
    let base_form = e.call(0x007a_f430, &args![this]).u32();
    let size = e.call(0x0060_9c70, &args![base_form, this]).u16();
    total = add16(total, u32::from(size));
    let name = e.call(0x0055_d520, &args![this]).u32();
    let length = e.call(0x00ec_6130, &args![name]).u32();
    let length_byte = length.wrapping_add(1) as u8;
    total = add16(total, u32::from(length_byte) + 1);
    if save_version(e) >= 0x2c {
        let base_object = e.call(BASE_FORM_OF, &args![this]).u32();
        let fire_node = e.call(0x0050_2430, &args![base_object]).u32();
        let at = e.call(SETTING_INT_POINTER, &args![0x011d_0a9cu32]).u32();
        let wanted = e.mem.u32(at);
        let holder = e.global::<u32>(0x011c_3f2c);
        let matching = e.call(0x0046_15a0, &args![holder, wanted]).u32();
        total = add16(total, 4);
        if fire_node != 0 && fire_node == matching {
            let size = e.call(0x005f_6ed0, &args![fire_node]).u16();
            total = add16(total, u32::from(size));
        }
    }
    if save_version(e) >= 0x45 {
        total = add16(total, 4);
    }
    save_size_report(
        e,
        SAVE_SIZE_FORMATS,
        i32::from(total) - i32::from(after_base),
        0x3365,
    );
    total
}

/// `TESForm::SaveGameDataOLD` (`00484ce0`): writes `size` bytes at `ptr`.
fn save_data(e: &mut Engine, this: u32, ptr: u32, size: u32) {
    e.call(0x0048_4ce0, &args![this, ptr, size]);
}

/// `TESForm::SaveNumericID` (`00484d20`): writes the form id at `ptr`.
fn save_numeric_id(e: &mut Engine, this: u32, ptr: u32, size: u32) {
    e.call(0x0048_4d20, &args![this, ptr, size]);
}

/// Saves the low `size` bytes of `value` through a temporary word in game
/// memory (`SaveGameDataOLD`).
fn save_value(e: &mut Engine, this: u32, value: u32, size: u32) {
    let word = e.mem.alloc(4);
    e.mem.set_u32(word, value);
    save_data(e, this, word, size);
    e.mem.free(word);
}

/// Saves a form id held in a temporary word (`SaveNumericID`).
fn save_id_value(e: &mut Engine, this: u32, value: u32) {
    let word = e.mem.alloc(4);
    e.mem.set_u32(word, value);
    save_numeric_id(e, this, word, 4);
    e.mem.free(word);
}

/// Saves the form id of `form`, or 0 for a null form.
fn save_form_id(e: &mut Engine, this: u32, form: u32) {
    let id = if form != 0 { form_id_of(e, form) } else { 0 };
    save_id_value(e, this, id);
}

/// Raw write of the save-load object (`008579b0`): `size` bytes at `ptr`.
fn save_raw(e: &mut Engine, ptr: u32, size: u32) {
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(0x0085_79b0, &args![save_load, ptr, size]);
}

/// The current write position of the save-load object (`00825c00`).
fn save_position(e: &mut Engine) -> u32 {
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(0x0082_5c00, &args![save_load]).u32()
}

/// Starts a save block when the save game uses blocks (`00862110`): writes the
/// marker `BLOK` and a zero length word, and returns the position of that
/// word (0 when no block is used).
fn save_block_begin(e: &mut Engine) -> u32 {
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    if !e.call(0x0086_2110, &args![save_load]).bool() {
        return 0;
    }
    let marker = e.mem.alloc(4);
    e.mem.set_u32(marker, 0x424c_4f4b);
    save_raw(e, marker, 4);
    e.mem.free(marker);
    let block = save_position(e);
    let length = e.mem.alloc(4);
    e.mem.set_u32(length, 0);
    save_raw(e, length, 2);
    e.mem.free(length);
    block
}

/// Ends the block that `save_block_begin` started at `block`: stores the
/// number of bytes written since (and logs when it does not fit 16 bits).
fn save_block_end(e: &mut Engine, block: u32, line: u32) {
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    if !e.call(0x0086_2110, &args![save_load]).bool() {
        return;
    }
    let position = save_position(e);
    if position > block.wrapping_add(0xffff) {
        e.call(
            LOG_MESSAGE,
            &args![FORMAT_BLOCK_TOO_BIG, SOURCE_FILE_NAME, line],
        );
    }
    e.mem.set_u16(block, position.wrapping_sub(block) as u16);
}

/// The list entries `fn_00955620` writes as `count` then one id per entry:
/// the entries of the `BSSimpleList` at `head`, each as `SaveGameDataOLD`
/// of the form id (`0084e3a0`) of the entry.
fn save_form_list(e: &mut Engine, this: u32, head: u32) {
    let count = e.call(LIST_COUNT, &args![head]).u32();
    save_value(e, this, count, 4);
    let mut node = head;
    while node != 0 {
        let item_slot = list_item_slot(e, node);
        if e.mem.u32(item_slot) == 0 {
            break;
        }
        let item_slot = list_item_slot(e, node);
        let item = e.mem.u32(item_slot);
        let id = form_id_of(e, item);
        save_value(e, this, id, 4);
        node = list_next(e, node);
    }
}

// Translated from 00955620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the player's save data (`SaveGame`, the error text calls it
/// `SaveGame()`): first the actor value modifiers (`+0x244`, `+0x378`,
/// `+0x4b0` from version 0x31, `+0x4ac`), then the base class
/// (`008d32f0(flags)`) and the player's own fields in the order of the
/// format, each added with the version that introduced it (the sizes are the
/// ones `fn_00954d40` adds). Bytes go through `TESForm::SaveGameDataOLD`
/// (`00484ce0`), form ids through `SaveNumericID` (`00484d20`), raw words
/// through `008579b0`; two blocks (`save_block_begin`) bracket the base part
/// and the player part when the save game uses blocks. With the debug
/// setting `011de4e8` on, the sizes are logged. The quest target loop
/// (`>= 0x63`) is compiled into a loop over a null list, so it writes only
/// its zero count (the compiler folded the head to zero). The exception
/// frame is not translated.
pub fn fn_00955620(e: &mut Engine, this: Ptr<PlayerCharacter>, flags: u32) {
    let player = this.addr();
    let formats = (FORMAT_SAVE_GAME_FORM, FORMAT_SAVE_GAME);
    let mut debug_start = save_position(e);
    let at = e.call(0x0040_8d60, &args![SAVE_SIZE_DEBUG_SETTING]).u32();
    if e.mem.u8(at) != 0 {
        debug_start = save_position(e);
    }
    let first_block = save_block_begin(e);
    save_data(e, player, player + 0x244, 0x134);
    save_data(e, player, player + 0x378, 0x134);
    if save_version(e) >= 0x31 {
        save_data(e, player, player + 0x4b0, 0x134);
    }
    save_data(e, player, player + 0x4ac, 4);
    let at = e.call(0x0040_8d60, &args![SAVE_SIZE_DEBUG_SETTING]).u32();
    if e.mem.u8(at) != 0 {
        let position = save_position(e);
        save_size_report(
            e,
            formats,
            position.wrapping_sub(debug_start) as i32,
            0x3378,
        );
    }
    save_block_end(e, first_block, 0x3378);
    e.call(0x008d_32f0, &args![this, flags]);

    let mut second_start = save_position(e);
    let at = e.call(0x0040_8d60, &args![SAVE_SIZE_DEBUG_SETTING]).u32();
    if e.mem.u8(at) != 0 {
        second_start = save_position(e);
    }
    let second_block = save_block_begin(e);
    if flags & 0x1000_0000 != 0 {
        let animation = e.mem.u32(player + 0x690);
        e.call(0x0049_aa80, &args![this, animation]);
    }
    let marker = e.mem.u32(player + 0x6f4);
    let position_source = if marker == 0 {
        INVENTORY_MODEL_ARGUMENT
    } else {
        e.vcall(marker, 0x1f4, &args![]).u32()
    };
    // The exe's frame (`SUB ESP,0x228` under `EBP`): the position copy at `EBP-0x40` (+0x1e8).
    e.with_stack(0x228, |e, frame| {
        let position = frame.addr() + 0x1e8;
        for i in 0..3 {
            let word = e.mem.u32(position_source + i * 4);
            e.mem.set_u32(position + i * 4, word);
        }
        for (offset, size) in [
            (0x64au32, 1u32),
            (0x64d, 1),
            (0x651, 1),
            (0x652, 1),
            (0x654, 4),
            (0x660, 4),
            (0x664, 4),
            (0x668, 4),
            (0x66c, 1),
            (0x6cc, 1),
            (0x6d0, 4),
            (0x6d4, 4),
            (0x6d8, 1),
            (0x6dc, 4),
            (0x6e8, 1),
            (0x6e4, 4),
        ] {
            save_data(e, player, player + offset, size);
        }
        save_data(e, player, position, 0xc);
    });
    for (offset, size) in [(0x698u32, 4u32), (0x67c, 4), (0x734, 4)] {
        save_data(e, player, player + offset, size);
    }
    save_raw(e, player + 0x738, 4);
    for (offset, size) in [
        (0x658u32, 1u32),
        (0x65c, 4),
        (0x75c, 1),
        (0x75e, 1),
        (0x730, 4),
        (0x790, 4),
        (0x680, 1),
    ] {
        save_data(e, player, player + offset, size);
    }
    if save_version(e) >= 0x28 && save_version(e) < 0x2d {
        save_data(e, player, player + 0x7a0, 0xc);
        save_data(e, player, player + 0x7a0, 0xc);
    }
    if save_version(e) >= 0x39 {
        // The same frame: the block at `EBP-0xf0` (+0x138).
        e.with_stack(0x228, |e, frame| {
            let buffer = frame.addr() + 0x138;
            e.call(0x004d_5f60, &args![buffer]);
            save_data(e, player, buffer, 0xac);
        });
        save_data(e, player, player + 0x744, 0x14);
    }
    if save_version(e) >= 0x3f {
        save_data(e, player, player + 0x7c4, 1);
    }
    if save_version(e) >= 0x40 {
        save_raw(e, player + 0x63c, 4);
        save_data(e, player, player + 0x640, 4);
        save_data(e, player, player + 0x644, 4);
    }
    if save_version(e) >= 0x49 {
        save_data(e, player, player + 0x200, 4);
        save_data(e, player, player + 0x240, 1);
    }
    save_version(e);
    if save_version(e) >= 0x71 {
        save_data(e, player, player + 0x64e, 1);
        save_data(e, player, player + 0x66d, 1);
        save_data(e, player, player + 0x794, 4);
        save_numeric_id(e, player, player + 0x7f4, 4);
    }
    if save_version(e) >= 0x78 {
        let iron_sights = e.call(0x008b_bc10, &args![this]).u8();
        save_value(e, player, u32::from(iron_sights), 1);
    }
    if save_version(e) >= 0x7a {
        save_data(e, player, VANITY_RESTORED_VALUE, 4);
    }
    if save_version(e) >= 0x7a {
        // The per-perk style list: count, then three fields per entry.
        let head = e.call(0x005a_6260, &args![this]).u32();
        let count = e.call(LIST_COUNT, &args![head]).u32();
        save_value(e, player, count, 4);
        let mut node = head;
        while node != 0 {
            let item_slot = list_item_slot(e, node);
            if e.mem.u32(item_slot) == 0 {
                break;
            }
            let item_slot = list_item_slot(e, node);
            let entry = e.mem.u32(item_slot);
            let first = e.mem.u32(entry);
            save_value(e, player, first, 4);
            let item_slot = list_item_slot(e, node);
            let entry = e.mem.u32(item_slot);
            save_data(e, player, entry + 4, 4);
            let item_slot = list_item_slot(e, node);
            let entry = e.mem.u32(item_slot);
            save_data(e, player, entry + 8, 2);
            node = list_next(e, node);
        }
    }
    if save_version(e) >= 0x7a {
        let first_list = e.call(0x0073_cba0, &args![this]).u32();
        save_form_list(e, player, first_list);
        let second_list = e.call(0x0073_cbc0, &args![this]).u32();
        save_form_list(e, player, second_list);
        for offset in [0x61cu32, 0x620, 0x624, 0x628, 0x62c] {
            save_data(e, player, player + offset, 4);
        }
    }
    // The form ids: the form at +0x208 (`011e0784` next), the selected spell
    // (checked with a dynamic cast), the magic item and target, the occupied
    // region, the selected scroll and two more.
    let form = e.mem.u32(player + 0x208);
    save_form_id(e, player, form);
    let form = e.global::<u32>(SAVED_FORM_011E0784);
    save_form_id(e, player, form);
    let mut spell_id = 0;
    let selected_spell = e.mem.u32(player + 0x6ec);
    if selected_spell != 0 {
        let cast = e
            .call(
                DYNAMIC_CAST,
                &args![
                    selected_spell,
                    0u32,
                    RTTI_CAST_SOURCE,
                    RTTI_CAST_TARGET,
                    0u32
                ],
            )
            .u32();
        if cast != 0 {
            spell_id = form_id_of(e, cast);
        }
    }
    save_id_value(e, player, spell_id);
    let magic_item = e.mem.u32(player + 0x214);
    let id = if magic_item != 0 {
        e.call(0x0040_a1e0, &args![magic_item]).u32()
    } else {
        0
    };
    save_id_value(e, player, id);
    let magic_target = e.mem.u32(player + 0x218);
    let id = if magic_target != 0 {
        e.call(0x0082_54c0, &args![magic_target]).u32()
    } else {
        0
    };
    save_id_value(e, player, id);
    for offset in [0x760u32, 0x6f0, 0x73c, 0x758] {
        let form = e.mem.u32(player + offset);
        save_form_id(e, player, form);
    }
    if save_version(e) >= 0x28 && save_version(e) < 0x2d {
        let form = e.mem.u32(player + 0x7ac);
        save_form_id(e, player, form);
    }
    if save_version(e) >= 0x40 {
        let form = e.mem.u32(player + 0x638);
        save_form_id(e, player, form);
    }
    if save_version(e) >= 0x42 {
        let form = e.global::<u32>(SAVED_FORM_011E078C);
        save_form_id(e, player, form);
    }
    if save_version(e) >= 0x57 {
        let form = e.mem.u32(player + 0x604);
        save_form_id(e, player, form);
    }
    if save_version(e) >= 0x60 {
        let mut id = 0;
        let marker = e.mem.u32(player + 0x6f4);
        if marker != 0 {
            let mut place = e.call(0x0057_5d70, &args![marker]).u32();
            if place == 0 {
                place = e.call(PARENT_CELL_OF, &args![marker]).u32();
            }
            if place != 0 {
                id = form_id_of(e, place);
            }
        }
        save_id_value(e, player, id);
    }
    if save_version(e) >= 0x63 {
        // The loop over the (folded to null) list writes nothing; the count
        // word, zero, is stored over the placeholder.
        let count_position = save_position(e);
        save_value(e, player, 0, 2);
        e.mem.set_u16(count_position, 0);
    }
    if save_version(e) >= 0x6c {
        let mut id = 0;
        let region = e.mem.u32(player + 0x760);
        if region != 0 && e.call(0x0059_bb30, &args![region]).u32() != 0 {
            let region_data = e.call(0x0059_bb30, &args![region]).u32();
            id = form_id_of(e, region_data);
        }
        save_id_value(e, player, id);
    }
    if save_version(e) >= 0x6f {
        let table = player + 0x854;
        let count = form_id_of(e, table);
        save_value(e, player, count & 0xffff, 2);
        // The same frame: the cursor at `EBP-0x168` (+0xc0), the value at `EBP-0x174` (+0xb4), the
        // flag byte at `EBP-0x175` (+0xb3).
        e.with_stack(0x228, |e, frame| {
            let iterator = frame.addr() + 0xc0;
            let value = frame.addr() + 0xb4;
            let flag = frame.addr() + 0xb3;
            let cursor = e.call(0x004b_9ba0, &args![table]).u32();
            e.mem.set_u32(iterator, cursor);
            while e.mem.u32(iterator) != 0 {
                e.mem.set_u32(value, 0);
                e.mem.set_u8(flag, 0);
                e.call(0x0096_a400, &args![table, iterator, value, flag]);
                save_numeric_id(e, player, value, 4);
                save_data(e, player, flag, 1);
            }
        });
    }
    if save_version(e) >= 0x73 {
        let count_position = save_position(e);
        save_value(e, player, 0, 2);
        let mut count: u16 = 0;
        let mut node = 0x011e_0ae8;
        while node != 0 && !list_is_empty(e, node) {
            let item_slot = list_item_slot(e, node);
            let item = e.mem.u32(item_slot);
            let id = if item != 0 { form_id_of(e, item) } else { 0 };
            save_id_value(e, player, id);
            count = count.wrapping_add(1);
            node = list_next(e, node);
        }
        e.mem.set_u16(count_position, count);
    }
    e.call(0x0080_5b40, &args![e.mem.u32(player + 0x210), this]);
    e.call(0x008d_5510, &args![player + CHARACTER_PROGRESSION, flags]);
    let quest = e.mem.u32(player + 0x6b8);
    save_form_id(e, player, quest);

    // The topic list.
    let count_position = save_position(e);
    save_value(e, player, 0, 2);
    let mut count: u16 = 0;
    let mut node = e.call(0x0046_4e30, &args![this]).u32();
    while node != 0 && !list_is_empty(e, node) {
        let item_slot = list_item_slot(e, node);
        let item = e.mem.u32(item_slot);
        let id = form_id_of(e, item);
        save_id_value(e, player, id);
        count = count.wrapping_add(1);
        node = list_next(e, node);
    }
    e.mem.set_u16(count_position, count);

    // The quest log.
    let count_position = save_position(e);
    save_value(e, player, 0, 2);
    let mut count: u16 = 0;
    let mut node = player + 0x6b0;
    while node != 0 && !list_is_empty(e, node) {
        let item_slot = list_item_slot(e, node);
        let item = e.mem.u32(item_slot);
        let quest = e.call(0x005e_3fa0, &args![item]).u32();
        let id = form_id_of(e, quest);
        let mut stage_done = 0u8;
        let stage = e.call(0x005d_c980, &args![item]).u8();
        let mut stage_list = e.call(EXTRA_LIST_OF, &args![quest]).u32();
        while stage_list != 0 && !list_is_empty(e, stage_list) {
            let item_slot = list_item_slot(e, stage_list);
            let candidate = e.mem.u32(item_slot);
            let found = e
                .call(0x0060_f2c0, &args![candidate, u32::from(stage)])
                .u32();
            if item == found {
                stage_done = e.call(0x0093_73f0, &args![candidate]).u8();
                break;
            }
            stage_list = list_next(e, stage_list);
        }
        save_id_value(e, player, id);
        save_value(e, player, u32::from(stage_done), 1);
        save_value(e, player, u32::from(stage), 1);
        count = count.wrapping_add(1);
        node = list_next(e, node);
    }
    e.mem.set_u16(count_position, count);

    // The quest targets.
    let count_position = save_position(e);
    save_value(e, player, 0, 2);
    let mut count: u16 = 0;
    let mut node = player + 0x6bc;
    while node != 0 && !list_is_empty(e, node) {
        let item_slot = list_item_slot(e, node);
        let item = e.mem.u32(item_slot);
        let quest = e.call(0x0044_edb0, &args![item]).u32();
        let id = form_id_of(e, quest);
        let delta = list_next(e, item);
        save_id_value(e, player, id);
        save_value(e, player, delta, 4);
        count = count.wrapping_add(1);
        node = list_next(e, node);
    }
    e.mem.set_u16(count_position, count);

    // The base object's data and the name.
    let base = e.call(0x007a_f430, &args![this]).u32();
    e.call(0x0060_9d60, &args![base, this]);
    let name = e.call(0x0055_d520, &args![this]).u32();
    let length = e.call(0x00ec_6130, &args![name]).u32();
    let length_byte = length.wrapping_add(1) as u8;
    save_value(e, player, u32::from(length_byte), 1);
    save_data(e, player, name, u32::from(length_byte));
    if save_version(e) >= 0x2c {
        let base_object = e.call(BASE_FORM_OF, &args![this]).u32();
        let fire_node = e.call(0x0050_2430, &args![base_object]).u32();
        let at = e.call(SETTING_INT_POINTER, &args![0x011d_0a9cu32]).u32();
        let wanted = e.mem.u32(at);
        let holder = e.global::<u32>(0x011c_3f2c);
        let matching = e.call(0x0046_15a0, &args![holder, wanted]).u32();
        save_form_id(e, player, fire_node);
        if fire_node != 0 && fire_node == matching {
            e.call(0x005f_6f70, &args![fire_node]);
        }
    }
    if save_version(e) >= 0x45 {
        let form = e.mem.u32(player + 0x740);
        save_form_id(e, player, form);
    }
    let at = e.call(0x0040_8d60, &args![SAVE_SIZE_DEBUG_SETTING]).u32();
    if e.mem.u8(at) != 0 {
        let position = save_position(e);
        save_size_report(
            e,
            formats,
            position.wrapping_sub(second_start) as i32,
            0x3592,
        );
    }
    save_block_end(e, second_block, 0x3592);
}

/// `TESForm::LoadGameDataOLD` (`00484d00`): reads `size` bytes to `ptr`.
fn load_data(e: &mut Engine, this: u32, ptr: u32, size: u32) {
    e.call(0x0048_4d00, &args![this, ptr, size]);
}

/// `TESForm::LoadNumericID` (`00484d40`): reads a saved form id to `ptr`.
fn load_numeric_id(e: &mut Engine, this: u32, ptr: u32, size: u32) {
    e.call(0x0048_4d40, &args![this, ptr, size]);
}

/// Reads `size` bytes (1, 2 or 4) through a temporary word.
fn load_value(e: &mut Engine, this: u32, size: u32) -> u32 {
    let word = e.mem.alloc(4);
    e.mem.set_u32(word, 0);
    load_data(e, this, word, size);
    let value = match size {
        1 => u32::from(e.mem.u8(word)),
        2 => u32::from(e.mem.u16(word)),
        _ => e.mem.u32(word),
    };
    e.mem.free(word);
    value
}

/// Reads a saved form id through a temporary word (`LoadNumericID`).
fn load_id_value(e: &mut Engine, this: u32) -> u32 {
    let word = e.mem.alloc(4);
    e.mem.set_u32(word, 0);
    load_numeric_id(e, this, word, 4);
    let value = e.mem.u32(word);
    e.mem.free(word);
    value
}

/// The form with the loaded id, cast with `__RTDynamicCast` from the `TESForm`
/// type descriptor `01183028` to `target` (`004839c0` finds the form).
fn load_cast_form(e: &mut Engine, id: u32, target: u32) -> u32 {
    let form = e.call(0x0048_39c0, &args![id]).u32();
    e.call(
        DYNAMIC_CAST,
        &args![form, 0u32, RTTI_LOAD_SOURCE, target, 0u32],
    )
    .u32()
}

/// Raw read of the save-load object (`008579e0`).
fn load_raw(e: &mut Engine, ptr: u32, size: u32) {
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(0x0085_79e0, &args![save_load, ptr, size]);
}

/// Skips `size` bytes of the save game (`00857bd0`).
fn load_skip(e: &mut Engine, size: u32) {
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(0x0085_7bd0, &args![save_load, size]);
}

/// Starts a block when the save game uses blocks: reads and checks the
/// marker (`BLOK`; a wrong one is logged with the form being loaded,
/// `004fd3c0`), then the length word. Returns the position after the marker
/// and the length (zeroes without blocks).
fn load_block_begin(e: &mut Engine, line: u32) -> (u32, u32) {
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    if !e.call(0x0086_2110, &args![save_load]).bool() {
        return (0, 0);
    }
    let word = e.mem.alloc(4);
    load_raw(e, word, 4);
    if e.mem.u32(word) != 0x424c_4f4b {
        let record = e.call(0x004f_d3c0, &args![save_load]).u32();
        if record != 0 {
            let form_id = e.mem.u32(record);
            let form_type = e.call(0x0048_39c0, &args![form_id]).u32();
            let version = u32::from(e.mem.u8(record + 9));
            let flags = e.mem.u32(record + 5);
            let name = e.vcall(form_type, 0x130, &args![]).u32();
            e.call(
                LOG_MESSAGE,
                &args![
                    FORMAT_BLOCK_HEADER_FORM,
                    SOURCE_FILE_NAME,
                    line,
                    form_id,
                    name,
                    version,
                    flags
                ],
            );
        } else {
            let version = u32::from(e.call(0x008d_f040, &args![save_load]).u8());
            e.call(
                LOG_MESSAGE,
                &args![FORMAT_BLOCK_HEADER, SOURCE_FILE_NAME, line, version],
            );
        }
    }
    let position = save_position(e);
    e.mem.set_u32(word, 0);
    load_raw(e, word, 2);
    let length = u32::from(e.mem.u16(word));
    e.mem.free(word);
    (position, length)
}

/// Ends a block started by `load_block_begin`: when the save game uses
/// blocks and the read position is not at `start + length`, logs the
/// overrun or underrun in bytes (with the form being loaded when known).
fn load_block_end(e: &mut Engine, start: u32, length: u32, line: u32) {
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    if !e.call(0x0086_2110, &args![save_load]).bool() {
        return;
    }
    let position = save_position(e);
    let record = e.call(0x004f_d3c0, &args![save_load]).u32();
    let expected = length.wrapping_add(start);
    if record != 0 {
        let form_id = e.mem.u32(record);
        let form_type = e.call(0x0048_39c0, &args![form_id]).u32();
        if position != expected {
            let version = u32::from(e.mem.u8(record + 9));
            let flags = e.mem.u32(record + 5);
            let name = e.vcall(form_type, 0x130, &args![]).u32();
            let (format, difference) = if position > expected {
                (FORMAT_OVERRUN_FORM, position - expected)
            } else {
                (FORMAT_UNDERRUN_FORM, expected - position)
            };
            e.call(
                LOG_MESSAGE,
                &args![
                    format,
                    difference,
                    SOURCE_FILE_NAME,
                    line,
                    form_id,
                    name,
                    version,
                    flags
                ],
            );
        }
    } else if position != expected {
        let version = u32::from(e.call(0x008d_f040, &args![save_load]).u8());
        let (format, difference) = if position > expected {
            (FORMAT_OVERRUN, position - expected)
        } else {
            (FORMAT_UNDERRUN, expected - position)
        };
        e.call(
            LOG_MESSAGE,
            &args![format, difference, SOURCE_FILE_NAME, line, version],
        );
    }
}

// Translated from 00956f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the player's save data (`LoadGame`): the mirror of `fn_00955620`.
/// Actor value modifiers (`+0x244`, `+0x378`, `+0x4b0`; `0x130` bytes each
/// before version 0x3b, `0x134` after), the base class (`008d3310(flags,
/// second)`), then the player's fields in the order the save wrote them,
/// each read when the version is high enough; the fields of versions that
/// dropped them are skipped (`00857bd0`). Form ids (`LoadNumericID`) are
/// resolved with `004839c0` and cast with `__RTDynamicCast` to the type the
/// field holds. Lists are rebuilt with `00905820` / `005ae3d0` after being
/// cleared (`00470470`). The two blocks are checked for over- and
/// underruns (line numbers 0x35b6, 0x35ba, 0x37c8). The exception frame is
/// not translated.
pub fn fn_00956f70(e: &mut Engine, this: Ptr<PlayerCharacter>, flags: u32, second: u32) {
    let player = this.addr();
    let (first_start, first_length) = load_block_begin(e, 0x359e);
    if save_version(e) < 0x3b {
        load_data(e, player, player + 0x244, 0x130);
        load_data(e, player, player + 0x378, 0x130);
        if save_version(e) >= 0x31 {
            load_data(e, player, player + 0x4b0, 0x130);
        }
    } else {
        load_data(e, player, player + 0x244, 0x134);
        load_data(e, player, player + 0x378, 0x134);
        load_data(e, player, player + 0x4b0, 0x134);
    }
    load_data(e, player, player + 0x4ac, 4);
    load_block_end(e, first_start, first_length, 0x35b6);
    e.call(0x008d_3310, &args![this, flags, second]);

    let (second_start, second_length) = load_block_begin(e, 0x35ba);
    if flags & 0x1000_0000 != 0 {
        let animation = e.mem.u32(player + 0x690);
        e.call(0x0049_ab00, &args![this, animation]);
    }
    // The exe's frame (`SUB ESP,0x31c` under the exception record, then the record link): the
    // position at `EBP-0x38` (+0x2f4), the block at `EBP-0xfc` (+0x230), the list words at
    // `EBP-0x104` (+0x228), `EBP-0x114` (+0x218), `EBP-0x320` (+0xc), `EBP-0x1a0` (+0x18c),
    // `EBP-0x1c4` (+0x168) and `EBP-0x1dc` (+0x150), the name at `EBP-0x2ec` (+0x40).
    e.with_stack(0x32c, |e, frame| {
        let position = frame.addr() + 0x2f4;
        e.call(LIST_NODE_ITEM_SLOT, &args![position]);
        for (offset, size) in [
            (0x64au32, 1u32),
            (0x64d, 1),
            (0x651, 1),
            (0x652, 1),
            (0x654, 4),
            (0x660, 4),
            (0x664, 4),
            (0x668, 4),
            (0x66c, 1),
            (0x6cc, 1),
            (0x6d0, 4),
            (0x6d4, 4),
            (0x6d8, 1),
            (0x6dc, 4),
            (0x6e8, 1),
            (0x6e4, 4),
        ] {
            load_data(e, player, player + offset, size);
        }
        load_data(e, player, position, 0xc);
        for (offset, size) in [(0x698u32, 4u32), (0x67c, 4), (0x734, 4)] {
            load_data(e, player, player + offset, size);
        }
        load_raw(e, player + 0x738, 4);
        for (offset, size) in [
            (0x658u32, 1u32),
            (0x65c, 4),
            (0x75c, 1),
            (0x75e, 1),
            (0x730, 4),
        ] {
            load_data(e, player, player + offset, size);
        }
        if save_version(e) >= 0x1d {
            load_data(e, player, player + 0x790, 4);
        }
        if save_version(e) >= 0x22 {
            load_data(e, player, player + 0x680, 1);
        }
        if save_version(e) >= 0x28 && save_version(e) < 0x2d {
            load_skip(e, 1);
            load_data(e, player, player + 0x7a0, 0xc);
            load_data(e, player, player + 0x7a0, 0xc);
        }
        if save_version(e) >= 0x35 && save_version(e) < 0x71 {
            load_skip(e, 4);
        }
        if save_version(e) >= 0x39 {
            let buffer = frame.addr() + 0x230;
            load_data(e, player, buffer, 0xac);
            e.call(0x004d_5f20, &args![buffer]);
            load_data(e, player, player + 0x744, 0x14);
        }
        if save_version(e) >= 0x3f {
            load_data(e, player, player + 0x7c4, 1);
        }
        if save_version(e) >= 0x40 {
            load_raw(e, player + 0x63c, 4);
            load_data(e, player, player + 0x640, 4);
            load_data(e, player, player + 0x644, 4);
        }
        if save_version(e) >= 0x49 {
            load_data(e, player, player + 0x200, 4);
            load_data(e, player, player + 0x240, 1);
        }
        if save_version(e) >= 0x4a && save_version(e) < 0x59 {
            load_skip(e, 8);
        }
        if save_version(e) >= 0x59 && save_version(e) < 0x5a {
            load_skip(e, 4);
        }
        save_version(e);
        if save_version(e) >= 0x71 {
            load_data(e, player, player + 0x64e, 1);
            load_data(e, player, player + 0x66d, 1);
            load_data(e, player, player + 0x794, 4);
            load_numeric_id(e, player, player + 0x7f4, 4);
        }
        if save_version(e) >= 0x78 {
            let iron_sights = load_value(e, player, 1);
            e.call(0x008b_b650, &args![this, iron_sights, 0u32, 0u32]);
        }
        if save_version(e) >= 0x7a {
            load_data(e, player, VANITY_RESTORED_VALUE, 4);
        }
        if save_version(e) >= 0x7a {
            let count = load_value(e, player, 4) as i32;
            let mut index = 0;
            while index < count {
                let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
                let entry = if block == 0 {
                    0
                } else {
                    e.call(0x0073_3f50, &args![block]).u32()
                };
                load_data(e, player, entry, 4);
                load_data(e, player, entry + 4, 4);
                load_data(e, player, entry + 8, 2);
                let list = e.mem.u32(player + 0x610);
                let slot = frame.addr() + 0x228;
                e.mem.set_u32(slot, entry);
                e.call(0x0090_5820, &args![list, slot]);
                index += 1;
            }
            if e.global::<u32>(LOADED_PERK_FLAG) != 0 {
                e.call(0x0096_9ac0, &args![this]);
            }
        }
        if save_version(e) >= 0x7a {
            let first_list = e.mem.u32(player + 0x614);
            e.call(LIST_CLEAR, &args![first_list]);
            let second_list = e.mem.u32(player + 0x618);
            e.call(LIST_CLEAR, &args![second_list]);
            for list_offset in [0x614u32, 0x618] {
                let count = load_value(e, player, 4) as i32;
                let mut index = 0;
                while index < count {
                    let id = load_value(e, player, 4);
                    let form = e.call(0x0048_39c0, &args![id]).u32();
                    let list = e.mem.u32(player + list_offset);
                    let slot = frame.addr() + 0x218;
                    e.mem.set_u32(slot, form);
                    e.call(0x0090_5820, &args![list, slot]);
                    index += 1;
                }
            }
            for offset in [0x61cu32, 0x620, 0x624, 0x628, 0x62c] {
                load_data(e, player, player + offset, 4);
            }
        }
        let id = load_id_value(e, player);
        e.mem.set_u32(player + 0x208, id);
        let id = load_id_value(e, player);
        e.set_global(SAVED_FORM_011E0784, id);
        player_character_set_selected_spell(e, this, 0);
        let id = load_id_value(e, player);
        e.mem.set_u32(player + 0x6ec, id);
        let id = load_id_value(e, player);
        e.mem.set_u32(player + 0x214, id);
        let id = load_id_value(e, player);
        e.mem.set_u32(player + 0x218, id);
        let id = load_id_value(e, player);
        let region = load_cast_form(e, id, RTTI_LOAD_REGION);
        e.call(0x0093_a7a0, &args![this, region]);
        let id = load_id_value(e, player);
        e.mem.set_u32(player + 0x6f0, id);
        let id = load_id_value(e, player);
        e.mem.set_u32(player + 0x73c, id);
        let id = load_id_value(e, player);
        e.mem.set_u32(player + 0x758, id);
        if save_version(e) >= 0x28 && save_version(e) < 0x2d {
            let id = load_id_value(e, player);
            let form = e.call(0x0048_39c0, &args![id]).u32();
            e.mem.set_u32(player + 0x7ac, form);
        }
        if save_version(e) >= 0x40 {
            let id = load_id_value(e, player);
            e.mem.set_u32(player + 0x638, id);
        }
        if save_version(e) >= 0x42 {
            let id = load_id_value(e, player);
            e.set_global(SAVED_FORM_011E078C, id);
        }
        if save_version(e) >= 0x57 {
            let id = load_id_value(e, player);
            let form = load_cast_form(e, id, RTTI_LOAD_FORM_0604);
            e.mem.set_u32(player + 0x604, form);
        }
        if save_version(e) >= 0x60 {
            let id = load_id_value(e, player);
            let place = e.call(0x0048_39c0, &args![id]).u32();
            e.mem.set_u32(player + 0x6f4, 0);
            if place != 0 {
                let x = e.mem.f32(position);
                let y = e.mem.f32(position + 4);
                let z = e.mem.f32(position + 8);
                player_character_set_player_map_marker(e, this, x, y, z, place);
            }
        }
        if save_version(e) >= 0x63 {
            let count = load_value(e, player, 2);
            let mut index = 0;
            while index < count {
                load_id_value(e, player);
                index += 1;
            }
        }
        if save_version(e) >= 0x6c {
            let id = load_id_value(e, player);
            let object = load_cast_form(e, id, RTTI_LOAD_REGION_DATA);
            let region = e.mem.u32(player + 0x760);
            if region != 0 && object != 0 {
                e.call(0x0070_37c0, &args![region, object]);
            }
        }
        if save_version(e) >= 0x6f {
            e.call(0x0043_8af0, &args![player + 0x854]);
            let count = load_value(e, player, 2);
            let mut index = 0;
            while index < count {
                let id = load_id_value(e, player);
                let flag = load_value(e, player, 1);
                if id != 0 {
                    e.call(0x0084_d310, &args![player + 0x854, id, flag]);
                }
                index += 1;
            }
        }
        if save_version(e) >= 0x73 {
            let count = load_value(e, player, 2);
            let mut index = 0;
            while index < count {
                let id = load_id_value(e, player);
                let slot = frame.addr() + 0xc;
                e.mem.set_u32(slot, id);
                e.call(LIST_APPEND, &args![0x011e_0ae8u32, slot]);
                index += 1;
            }
        }
        let progress = e.mem.u32(player + 0x210);
        e.call(0x0080_5d40, &args![progress, this]);
        e.call(
            0x008d_5550,
            &args![player + CHARACTER_PROGRESSION, flags, second],
        );
        let id = load_id_value(e, player);
        if id != 0 {
            let quest = load_cast_form(e, id, RTTI_LOAD_QUEST);
            e.mem.set_u32(player + 0x6b8, quest);
            if quest != 0 {
                e.call(0x0060_f110, &args![quest, player + 0x6c4, player + 0x6bc]);
            }
        } else {
            e.mem.set_u32(player + 0x6b8, 0);
        }
        e.call(LIST_CLEAR, &args![player + 0x6a8]);
        let count = load_value(e, player, 2);
        let mut index = 0;
        while index < count {
            let id = load_id_value(e, player);
            let topic = load_cast_form(e, id, RTTI_LOAD_TOPIC);
            if topic != 0 {
                let slot = frame.addr() + 0x18c;
                e.mem.set_u32(slot, topic);
                e.call(LIST_APPEND, &args![player + 0x6a8, slot]);
            }
            index += 1;
        }
        let head = e.call(0x0046_4e30, &args![this]).u32();
        e.call(0x0061_a5a0, &args![1u32, head]);
        e.call(LIST_CLEAR, &args![player + 0x6b0]);
        let count = load_value(e, player, 2);
        let mut index = 0;
        while index < count {
            let id = load_id_value(e, player);
            let stage_done = load_value(e, player, 1);
            let stage = load_value(e, player, 1);
            let quest = load_cast_form(e, id, RTTI_LOAD_QUEST);
            if quest != 0 {
                let entry = e.call(0x0060_db40, &args![quest, stage_done]).u32();
                if entry != 0 {
                    let item = e.call(0x0060_f2c0, &args![entry, stage]).u32();
                    if item != 0 {
                        let slot = frame.addr() + 0x168;
                        e.mem.set_u32(slot, item);
                        e.call(0x0090_5820, &args![player + 0x6b0, slot]);
                    }
                }
            }
            index += 1;
        }
        e.call(LIST_CLEAR, &args![player + 0x6bc]);
        let count = load_value(e, player, 2);
        let mut index = 0;
        while index < count {
            let id = load_id_value(e, player);
            let delta = load_value(e, player, 4);
            let quest = load_cast_form(e, id, RTTI_LOAD_QUEST);
            if quest != 0 {
                let target = e.call(0x0060_c8e0, &args![quest, delta]).u32();
                if target != 0 {
                    let slot = frame.addr() + 0x150;
                    e.mem.set_u32(slot, target);
                    e.call(0x0090_5820, &args![player + 0x6bc, slot]);
                }
            }
            index += 1;
        }
        let base = e.call(0x007a_f430, &args![this]).u32();
        e.call(0x0060_9f60, &args![base, this]);
        let length = load_value(e, player, 1);
        let name = frame.addr() + 0x40;
        e.mem.set_u32(name, 0);
        load_data(e, player, name, length);
        let base = e.call(0x007a_f430, &args![this]).u32();
        e.call(0x0048_9100, &args![base + 0xd0, name]);
        if save_version(e) >= 0x2c {
            let base_object = e.call(BASE_FORM_OF, &args![this]).u32();
            let at = e.call(SETTING_INT_POINTER, &args![0x011d_0a9cu32]).u32();
            let wanted = e.mem.u32(at);
            let holder = e.global::<u32>(0x011c_3f2c);
            let matching = e.call(0x0046_15a0, &args![holder, wanted]).u32();
            let id = load_id_value(e, player);
            let fire_node = load_cast_form(e, id, RTTI_LOAD_FIRE_NODE);
            if fire_node != 0 {
                e.call(0x0060_1c70, &args![base_object, fire_node]);
            }
            if fire_node != 0 && fire_node == matching {
                e.call(0x005f_7050, &args![fire_node]);
            }
        }
        if save_version(e) >= 0x45 {
            let id = load_id_value(e, player);
            if id != 0 {
                let form = load_cast_form(e, id, RTTI_LOAD_FIRE_NODE);
                e.mem.set_u32(player + 0x740, form);
            }
        }
        load_block_end(e, second_start, second_length, 0x37c8);
    });
}

// Translated from 00958990 (decompiled, FalloutNV.exe 1.4.0.525)
/// The second pass of loading the player: turns the form ids that
/// `fn_00956f70` stored into pointers. After `008d3380(first, second)` the
/// form at `+0x208`, the global `011e0784`, the selected spell (`+0x6ec`,
/// cleared first, then set through `SetSelectedSpell` from the spell or
/// scroll interface of the form), the magic item (`0040a250`) and target
/// (`00825550`) by numeric id, the selected scroll, the forms at `+0x73c`,
/// `+0x758`, `+0x638` (from version 0x40) and the global `011e078c` (from
/// version 0x42) are resolved with `004839c0` and `__RTDynamicCast`. From
/// version 0x73 the global list `011e0ae8` is resolved the same way and the
/// entries that do not resolve are removed (`0063f7b0` for the first node,
/// `00905330` after the previous one). The list loop of version 0x63 and up
/// walks a list the compiler folded to null and does nothing. Finally
/// `008060e0`, slot `0x58` of the process, `0060a890`,
/// `0093a5f0(false)`, `0093a6f0(false)`, `0095f590` (controls disabled
/// flag, `+0x680`) and `008d0600(first, second)` of the progression.
pub fn fn_00958990(e: &mut Engine, this: Ptr<PlayerCharacter>, first: u32, second: u32) {
    let player = this.addr();
    e.call(0x008d_3380, &args![this, first, second]);
    let id = e.mem.u32(player + 0x208);
    if id != 0 {
        let form = load_cast_form(e, id, RTTI_RESOLVE_0208);
        e.mem.set_u32(player + 0x208, form);
    }
    let id = e.global::<u32>(SAVED_FORM_011E0784);
    if id != 0 {
        let form = load_cast_form(e, id, RTTI_LOAD_FORM_0604);
        e.set_global(SAVED_FORM_011E0784, form);
    }
    let spell = e.mem.u32(player + 0x6ec);
    e.mem.set_u32(player + 0x6ec, 0);
    if spell != 0 {
        let form = e.call(0x0048_39c0, &args![spell]).u32();
        let as_spell = e
            .call(
                DYNAMIC_CAST,
                &args![form, 0u32, RTTI_LOAD_SOURCE, RTTI_SPELL_INTERFACE, 0u32],
            )
            .u32();
        let as_scroll_item = e
            .call(
                DYNAMIC_CAST,
                &args![form, 0u32, RTTI_LOAD_SOURCE, RTTI_MAGIC_ITEM_FORM, 0u32],
            )
            .u32();
        if as_spell != 0 {
            player_character_set_selected_spell(e, this, as_spell + 0x18);
        } else if as_scroll_item != 0 {
            player_character_set_selected_spell(e, this, as_scroll_item + 0x30);
        }
    }
    let magic_item = e.mem.u32(player + 0x214);
    if magic_item != 0 {
        let resolved = e.call(0x0040_a250, &args![magic_item]).u32();
        e.mem.set_u32(player + 0x214, resolved);
    }
    let magic_target = e.mem.u32(player + 0x218);
    if magic_target != 0 {
        let resolved = e.call(0x0082_5550, &args![magic_target]).u32();
        e.mem.set_u32(player + 0x218, resolved);
    }
    let scroll = e.mem.u32(player + 0x6f0);
    if scroll != 0 {
        let form = load_cast_form(e, scroll, RTTI_RESOLVE_SCROLL);
        fn_0094c950(e, this, Ptr::new(form));
    }
    for (offset, target) in [(0x73cu32, RTTI_LOAD_FIRE_NODE), (0x758, RTTI_RESOLVE_0758)] {
        let id = e.mem.u32(player + offset);
        if id != 0 {
            let form = load_cast_form(e, id, target);
            e.mem.set_u32(player + offset, form);
        }
    }
    if save_version(e) >= 0x40 {
        let id = e.mem.u32(player + 0x638);
        if id != 0 {
            let form = load_cast_form(e, id, RTTI_LOAD_FORM_0604);
            e.mem.set_u32(player + 0x638, form);
        }
    }
    if save_version(e) >= 0x42 {
        let id = e.global::<u32>(SAVED_FORM_011E078C);
        if id != 0 {
            let form = load_cast_form(e, id, RTTI_LOAD_FORM_0604);
            e.set_global(SAVED_FORM_011E078C, form);
        }
    }
    save_version(e);
    if save_version(e) >= 0x73 {
        // The exe's frame (`SUB ESP,0x6c` under `EBP`): the resolved form at `EBP-0x58` (+0x14),
        // the id to remove at `EBP-0x60` (+0xc).
        e.with_stack(0x6c, |e, frame| {
            let mut node = 0x011e_0ae8;
            let mut previous = 0;
            while node != 0 && !list_is_empty(e, node) {
                let item_slot = list_item_slot(e, node);
                let id = e.mem.u32(item_slot);
                let mut resolved = 0;
                if id != 0 {
                    resolved = load_cast_form(e, id, RTTI_LOAD_FORM_0604);
                    let word = frame.addr() + 0x14;
                    e.mem.set_u32(word, resolved);
                    e.call(0x0072_6c60, &args![node, word]);
                }
                if resolved == 0 {
                    if previous == 0 {
                        e.call(0x0063_f7b0, &args![node]);
                    } else {
                        let word = frame.addr() + 0xc;
                        e.mem.set_u32(word, id);
                        e.call(0x0090_5330, &args![previous, word]);
                        node = list_next(e, previous);
                    }
                } else {
                    previous = node;
                    node = list_next(e, node);
                }
            }
        });
    }
    let progress = e.mem.u32(player + 0x210);
    e.call(0x0080_60e0, &args![progress, this]);
    let process = process_of(e, this);
    e.vcall(process, 0x58, &args![]);
    let base = e.call(0x007a_f430, &args![this]).u32();
    e.call(0x0060_a890, &args![base, this]);
    e.call(0x0093_a5f0, &args![this, 0u32]);
    e.call(0x0093_a6f0, &args![this, 0u32]);
    let disabled = e.mem.u8(player + 0x680);
    e.call(0x0095_f590, &args![this, u32::from(disabled)]);
    e.call(
        0x008d_0600,
        &args![player + CHARACTER_PROGRESSION, first, second],
    );
}

// Translated from 00958ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The last pass of loading the player: `008aa9a0(first, second)`; when the
/// first-person biped exists (`00950bb0(1)`), `SetFirstPerson` (the opposite
/// of `+0x64a`) and, with `+0x64d` set, `ForceTemp3rdPerson(true)`;
/// `00806130`; the camera spring (`0095f930`) of the body at `+0x638` unless
/// its mode (`+0x63c`) is 3; `0080cec0`, `00825990`, `SetLastKnownGoodPosition`
/// (`00947c90`), the field of view (`00950610(+0x65c)`) and
/// `008d0600(first, second)` of the progression.
pub fn fn_00958ec0(e: &mut Engine, this: Ptr<PlayerCharacter>, first: u32, second: u32) {
    let player = this.addr();
    e.call(0x008a_a9a0, &args![this, first, second]);
    if e.call(0x0095_0bb0, &args![this, 1u32]).u32() != 0 {
        let third_person = e.get(this, PlayerCharacter::b3rdPerson);
        player_character_set_first_person(e, this, u8::from(!third_person));
        if e.mem.u8(player + 0x64d) != 0 {
            player_character_force_temp_3rd_person(e, this, 1);
        }
    }
    let progress = e.mem.u32(player + 0x210);
    e.call(0x0080_6130, &args![progress, this]);
    if e.mem.u32(player + 0x638) != 0 && e.mem.u32(player + 0x63c) != 3 {
        let body = e.mem.u32(player + 0x638);
        let mode = e.mem.u32(player + 0x63c);
        let strength = e.mem.f32(player + 0x644);
        e.call(0x0095_f930, &args![this, body, mode, strength]);
    }
    e.call(0x0080_cec0, &args![]);
    e.call(0x0082_5990, &args![]);
    e.call(0x0094_7c90, &args![this]);
    let fov = e.mem.f32(player + 0x65c);
    fn_00950610(e, this, fov);
    e.call(
        0x008d_0600,
        &args![player + CHARACTER_PROGRESSION, first, second],
    );
}

// Translated from 00958fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the player before a load: `008d3330(flags)`, the byte at `+0xe39`
/// set (`009590d0`), the float `+0xc8` and the byte `+0x66d` cleared,
/// `005d14d0(true)`; when the save-load object says so (`0055f5b0`) also the
/// globals `011e0784` and `011e078c`, vanity mode, `00950010(false)`, the
/// list `011e0ae8`, `008061b0`, the first-person animation
/// (`0049a920`, with bit `0x10000000` of `flags`), `00851d10`, `00961280` and
/// the list at `+0x84c`; then `004534f0(flags)` of the progression and the
/// byte `+0x75d` set.
pub fn fn_00958fc0(e: &mut Engine, this: Ptr<PlayerCharacter>, flags: u32) {
    let player = this.addr();
    e.call(0x008d_3330, &args![this, flags]);
    fn_009590d0(e, this, 1);
    e.mem.set_f32(player + 0xc8, 0.0);
    e.mem.set_u8(player + 0x66d, 0);
    e.call(0x005d_14d0, &args![this, 1u32]);
    e.mem.set_f32(player + 0xc8, 0.0);
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    if e.call(0x0055_f5b0, &args![save_load]).bool() {
        e.set_global(SAVED_FORM_011E0784, 0u32);
        e.set_global(SAVED_FORM_011E078C, 0u32);
        player_character_stop_vanity_mode(e, this.cast());
        fn_00950010(e, this, 0);
        e.call(LIST_CLEAR, &args![0x011e_0ae8u32]);
        let progress = e.mem.u32(player + 0x210);
        e.call(0x0080_61b0, &args![progress, this]);
        if flags & 0x1000_0000 != 0 && e.mem.u32(player + 0x690) != 0 {
            let animation = e.mem.u32(player + 0x690);
            e.call(0x0049_a920, &args![animation, this]);
        }
        e.call(0x0085_1d10, &args![this]);
        e.call(0x0096_1280, &args![this]);
        e.call(LIST_CLEAR, &args![player + 0x84c]);
    }
    e.call(0x0045_34f0, &args![player + CHARACTER_PROGRESSION, flags]);
    e.mem.set_u8(player + 0x75d, 1);
}

// Translated from 009590d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `+0xe39`.
pub fn fn_009590d0(e: &mut Engine, this: Ptr<PlayerCharacter>, value: u8) {
    e.mem.set_u8(this.addr() + 0xe39, value);
}

/// `BGSSaveGameBuffer::SaveGameData`-style write (`00865e50`): `size` bytes
/// at `ptr`, into the buffer `buffer`.
fn buffer_write(e: &mut Engine, buffer: u32, ptr: u32, size: u32) {
    e.call(0x0086_5e50, &args![buffer, ptr, size, 0u32]);
}

/// Writes the low `size` bytes of `value` through a temporary word.
fn buffer_write_value(e: &mut Engine, buffer: u32, value: u32, size: u32) {
    let word = e.mem.alloc(4);
    e.mem.set_u32(word, value);
    buffer_write(e, buffer, word, size);
    e.mem.free(word);
}

/// `BGSSaveGameBuffer::SaveFormID` (`00865df0`).
fn buffer_form_id(e: &mut Engine, buffer: u32, form: u32) {
    e.call(0x0086_5df0, &args![buffer, form, 0u32]);
}

/// `BGSSaveGameBuffer::StartVariableSizedValue` (`00865f20`).
fn buffer_start_sized(e: &mut Engine, buffer: u32) -> u32 {
    e.call(0x0086_5f20, &args![buffer]).u32()
}

/// `BGSSaveGameBuffer::SaveVariableSizedValue` (`00865ff0`): stores `count`
/// over the placeholder `start` returned.
fn buffer_finish_sized(e: &mut Engine, buffer: u32, count: u32, start: u32) {
    e.call(0x0086_5ff0, &args![buffer, count, start]);
}

/// A counted list of the save buffer: for every node of the list starting at
/// `head` (walked until the node is null) whose item is not null,
/// `write_item` stores the item and returns whether it counts; the count is
/// stored over the placeholder at the end.
fn buffer_write_list(
    e: &mut Engine,
    buffer: u32,
    head: u32,
    mut write_item: impl FnMut(&mut Engine, u32) -> bool,
) {
    let mut count = 0u32;
    let start = buffer_start_sized(e, buffer);
    let mut node = head;
    while node != 0 {
        let item_slot = list_item_slot(e, node);
        let item = e.mem.u32(item_slot);
        if item != 0 && write_item(e, item) {
            count += 1;
        }
        node = list_next(e, node);
    }
    buffer_finish_sized(e, buffer, count, start);
}

// Translated from 009590f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the player's save data into the save game buffer `buffer`
/// (`BGSSaveGameBuffer`, the PC format): the three actor value modifier
/// arrays (`0x4d` floats each, `+0x244`, `+0x378`, `+0x4b0`) and `+0x4ac`,
/// the base class (`008d33a0`), the first-person animation (`0049ab40`) when
/// the buffer's flags (`00428110`, `004280f0`) ask for it, the player's
/// fields, `008d5660` of the progression, the form ids the player refers to,
/// the topic list (skipping the entries `00619410` rejects), the list at
/// `+0x5e4`, the item array at `+0xdf4` (`004bed60` for each), the lists at
/// `+0xd48`, `+0x87c`, `+0x60c`, `+0x610`, `+0x614`, `+0x618`, the quest log
/// (`+0x6b0`), the quest targets (`+0x6bc`), the active effects (`00806a10`),
/// the combat group count, two floats (`008d1cf0`, `008d1d00`), the list at
/// `+0xad4`, two bytes (`004d1360`, `005a6050`), `+0x66e`, eight form ids
/// (`004bfb30`) and the pending script references (`005aa930`). Each counted
/// list stores its count over a placeholder (`StartVariableSizedValue`).
pub fn fn_009590f0(e: &mut Engine, this: Ptr<PlayerCharacter>, buffer: u32) {
    let player = this.addr();
    for base in [0x244u32, 0x378, 0x4b0] {
        for index in 0..0x4du32 {
            buffer_write(e, buffer, player + base + index * 4, 4);
        }
    }
    buffer_write(e, buffer, player + 0x4ac, 4);
    e.call(0x008d_33a0, &args![this, buffer]);
    // The exe's frame (`SUB ESP,0x13c` under `EBP`): the flag holder at `EBP-0x124` (+0x18).
    let wanted = e.with_stack(0x13c, |e, frame| {
        let flag_holder = frame.addr() + 0x18;
        let flags = e.call(0x0042_8110, &args![buffer, flag_holder]).u32();
        e.call(0x0042_80f0, &args![flags, 0x1000_0000u32]).bool()
    });
    if wanted {
        let start = buffer_start_sized(e, buffer);
        let before = form_id_of(e, buffer);
        let animation = e.mem.u32(player + 0x690);
        e.call(0x0049_ab40, &args![animation, buffer]);
        let after = form_id_of(e, buffer);
        buffer_finish_sized(e, buffer, after.wrapping_sub(before), start);
    }
    for (offset, size) in [
        (0x64au32, 1u32),
        (0x64d, 1),
        (0x651, 1),
        (0x652, 1),
        (0x654, 4),
        (0x660, 4),
        (0x664, 4),
        (0x668, 4),
        (0x66c, 1),
        (0x6cc, 1),
        (0x6d0, 4),
        (0x6d4, 4),
        (0x6d8, 1),
        (0x6dc, 4),
        (0x6e8, 1),
        (0x681, 1),
        (0x7c5, 1),
        (0x7c6, 1),
    ] {
        buffer_write(e, buffer, player + offset, size);
    }
    let marker = e.mem.u32(player + 0x6f4);
    let position_source = if marker == 0 {
        INVENTORY_MODEL_ARGUMENT
    } else {
        e.vcall(marker, 0x1f4, &args![]).u32()
    };
    // The same frame: the position copy at `EBP-0xc` (+0x130).
    e.with_stack(0x13c, |e, frame| {
        let position = frame.addr() + 0x130;
        for i in 0..3 {
            let word = e.mem.u32(position_source + i * 4);
            e.mem.set_u32(position + i * 4, word);
        }
        buffer_write(e, buffer, player + 0x6e4, 4);
        buffer_write(e, buffer, position, 0xc);
    });
    for (offset, size) in [
        (0x698u32, 4u32),
        (0x67c, 4),
        (0x738, 4),
        (0x658, 1),
        (0x65c, 4),
        (0x674, 4),
        (0x670, 4),
        (0x75c, 1),
        (0x730, 4),
        (0x790, 4),
        (0x680, 1),
        (0x7c4, 1),
        (0x63c, 4),
        (0x640, 4),
        (0x644, 4),
        (0x200, 4),
        (0x240, 1),
        (0x64e, 1),
        (0x66d, 1),
        (0x794, 4),
    ] {
        buffer_write(e, buffer, player + offset, size);
    }
    buffer_write(e, buffer, VANITY_RESTORED_VALUE, 4);
    for (offset, size) in [
        (0xd6cu32, 4u32),
        (0xd70, 4),
        (0x228, 4),
        (0x22c, 4),
        (0x230, 4),
        (0x234, 4),
        (0x608, 1),
        (0xdf2, 1),
        (0x64f, 1),
        (0x650, 1),
        (0x7c7, 1),
        (0x5f8, 1),
        (0x1fc, 4),
        (0x684, 4),
    ] {
        buffer_write(e, buffer, player + offset, size);
    }
    for index in 0..5u32 {
        buffer_write(e, buffer, player + 0x744 + index * 4, 4);
    }
    e.call(0x008d_5660, &args![player + CHARACTER_PROGRESSION, buffer]);

    // The form ids.
    let mut marker_place = 0;
    let marker = e.mem.u32(player + 0x6f4);
    if marker != 0 {
        marker_place = e.call(0x0057_5d70, &args![marker]).u32();
        if marker_place == 0 {
            marker_place = e.call(PARENT_CELL_OF, &args![marker]).u32();
        }
    }
    let quest = e.mem.u32(player + 0x6b8);
    buffer_form_id(e, buffer, quest);
    let form = e.mem.u32(player + 0x73c);
    buffer_form_id(e, buffer, form);
    buffer_form_id(e, buffer, marker_place);
    let region = e.mem.u32(player + 0x760);
    buffer_form_id(e, buffer, region);
    let region_data = if e.mem.u32(player + 0x760) != 0 {
        e.call(0x0059_bb30, &args![region]).u32()
    } else {
        0
    };
    buffer_form_id(e, buffer, region_data);
    for offset in [0x208u32, 0x224, 0x638, 0x604, 0xd2c, 0xd44] {
        let form = e.mem.u32(player + offset);
        buffer_form_id(e, buffer, form);
    }

    // The counted lists.
    buffer_write_list(e, buffer, player + 0x6a8, |e, item| {
        if e.call(0x0061_9410, &args![item]).bool() {
            false
        } else {
            buffer_form_id(e, buffer, item);
            true
        }
    });
    buffer_write_list(e, buffer, player + 0x5e4, |e, item| {
        buffer_form_id(e, buffer, item);
        true
    });
    let items = player + 0xdf4;
    let item_count = if items == 0 {
        0
    } else {
        e.call(WORD_AT_8, &args![items]).u32()
    };
    e.call(0x0086_5f60, &args![buffer, item_count]);
    let mut index = 0;
    while index < item_count {
        let slot = e.call(0x006a_7ad0, &args![items, index]).u32();
        let entry = e.mem.u32(slot);
        e.call(0x004b_ed60, &args![entry, buffer]);
        index += 1;
    }
    let list = e.mem.u32(player + 0xd48);
    buffer_write_list(e, buffer, list, |e, item| {
        let form = e.mem.u32(item);
        buffer_form_id(e, buffer, form);
        buffer_write(e, buffer, item + 4, 1);
        buffer_write(e, buffer, item + 5, 1);
        true
    });
    buffer_write_list(e, buffer, player + 0x87c, |e, item| {
        let form = e.mem.u32(item);
        buffer_form_id(e, buffer, form);
        buffer_write(e, buffer, item + 4, 1);
        true
    });
    let list = e.mem.u32(player + 0x60c);
    buffer_write_list(e, buffer, list, |e, item| {
        buffer_write(e, buffer, item, 4);
        buffer_write(e, buffer, item + 4, 4);
        let form = e.mem.u32(item + 8);
        buffer_form_id(e, buffer, form);
        true
    });
    let list = e.mem.u32(player + 0x610);
    buffer_write_list(e, buffer, list, |e, item| {
        buffer_write(e, buffer, item, 4);
        buffer_write(e, buffer, item + 4, 4);
        buffer_write(e, buffer, item + 8, 2);
        true
    });
    for list_offset in [0x614u32, 0x618] {
        let list = e.mem.u32(player + list_offset);
        buffer_write_list(e, buffer, list, |e, item| {
            buffer_form_id(e, buffer, item);
            true
        });
    }
    for offset in [0x61cu32, 0x620, 0x624, 0x628, 0x62c] {
        buffer_write(e, buffer, player + offset, 4);
    }
    buffer_write_list(e, buffer, player + 0x6b0, |e, item| {
        let quest = e.call(0x005e_3fa0, &args![item]).u32();
        let stage_done = e.call(0x0060_f1a0, &args![quest, item]).u8();
        let stage = e.call(0x005d_c980, &args![item]).u8();
        buffer_form_id(e, buffer, quest);
        buffer_write_value(e, buffer, u32::from(stage_done), 1);
        buffer_write_value(e, buffer, u32::from(stage), 1);
        true
    });
    buffer_write_list(e, buffer, player + 0x6bc, |e, item| {
        let quest = e.call(0x0044_edb0, &args![item]).u32();
        let delta = list_next(e, item);
        buffer_form_id(e, buffer, quest);
        buffer_write_value(e, buffer, delta, 4);
        true
    });
    let progress = e.mem.u32(player + 0x210);
    e.call(0x0080_6a10, &args![buffer, progress]);
    let group = e.mem.u32(player + 0xd64);
    let group_count = if group == 0 { 0 } else { list_next(e, group) };
    buffer_write_value(e, buffer, group_count, 4);
    let first = e.call(0x008d_1cf0, &args![]).f32();
    let second = e.call(0x008d_1d00, &args![]).f32();
    buffer_write_value(e, buffer, first.to_bits(), 4);
    buffer_write_value(e, buffer, second.to_bits(), 4);
    buffer_write_list(e, buffer, player + 0xad4, |e, item| {
        let form = e.mem.u32(item);
        buffer_form_id(e, buffer, form);
        buffer_write(e, buffer, item + 4, 1);
        true
    });
    let first_byte = e.call(0x005a_6050, &args![this]).u8();
    let second_byte = e.call(0x004d_1360, &args![this]).u8();
    buffer_write_value(e, buffer, u32::from(second_byte), 1);
    buffer_write_value(e, buffer, u32::from(first_byte), 1);
    buffer_write(e, buffer, player + 0x66e, 1);
    for index in 0..8u32 {
        let form = e.call(0x004b_fb30, &args![this, index]).u32();
        let id = if form != 0 { form_id_of(e, form) } else { 0 };
        buffer_write_value(e, buffer, id, 4);
    }
    e.call(0x005a_a930, &args![buffer]);
}

/// `BGSLoadGameBuffer` raw read (`00864980`): `size` bytes to `ptr`.
fn buffer_read(e: &mut Engine, buffer: u32, ptr: u32, size: u32) {
    e.call(0x0086_4980, &args![buffer, ptr, size]);
}

/// Reads `size` bytes (1, 2 or 4) through a temporary word.
fn buffer_read_value(e: &mut Engine, buffer: u32, size: u32) -> u32 {
    let word = e.mem.alloc(4);
    e.mem.set_u32(word, 0);
    buffer_read(e, buffer, word, size);
    let value = match size {
        1 => u32::from(e.mem.u8(word)),
        2 => u32::from(e.mem.u16(word)),
        _ => e.mem.u32(word),
    };
    e.mem.free(word);
    value
}

/// `BGSLoadGameBuffer::LoadFormID` (`008648a0`), cast to `target` as the
/// callers do (`004839c0`, then `__RTDynamicCast` from the `TESForm` type).
fn buffer_read_form(e: &mut Engine, buffer: u32, target: u32) -> u32 {
    let id = e.call(0x0086_48a0, &args![buffer]).u32();
    load_cast_form(e, id, target)
}

/// `BGSLoadGameBuffer::LoadVariableSizedValue` (`00864a60`).
fn buffer_read_count(e: &mut Engine, buffer: u32) -> u32 {
    e.call(0x0086_4a60, &args![buffer]).u32()
}

/// The version the load buffer reads (slot 0 of its vtable).
fn buffer_version(e: &mut Engine, buffer: u32) -> u32 {
    u32::from(e.vcall(buffer, 0, &args![]).u8())
}

/// Appends `item` to the list object `list` (`005ae3d0` / `00905820`).
fn append_item(e: &mut Engine, append: u32, list: u32, item: u32) {
    let slot = e.mem.alloc(4);
    e.mem.set_u32(slot, item);
    e.call(append, &args![list, slot]);
    e.mem.free(slot);
}

/// Creates the heap list a player field holds when it has none (`0096a2d0`
/// on a new 8-byte block) and stores it at `+offset`.
fn create_list_field(e: &mut Engine, player: u32, offset: u32) {
    let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
    let list = if block == 0 {
        0
    } else {
        e.call(0x0096_a2d0, &args![block]).u32()
    };
    e.mem.set_u32(player + offset, list);
}

/// Removes the entries of the embedded list at `head` whose first word is
/// null (they are freed) or that are null, the way the load does after
/// reading it: `0063f7b0` for the first node, `00905330` after the previous
/// one otherwise.
fn purge_dead_entries(e: &mut Engine, head: u32) {
    let mut node = head;
    let mut previous = 0;
    while node != 0 && !list_is_empty(e, node) {
        let item_slot = list_item_slot(e, node);
        let item = e.mem.u32(item_slot);
        let mut dead = false;
        if item != 0 {
            if e.mem.u32(item) == 0 {
                e.call(0x0040_1030, &args![item]);
                dead = true;
            }
        } else {
            dead = true;
        }
        if dead {
            if previous != 0 {
                let item_slot = list_item_slot(e, node);
                e.call(0x0090_5330, &args![previous, item_slot]);
                node = list_next(e, previous);
            } else {
                e.call(0x0063_f7b0, &args![node]);
            }
        } else {
            previous = node;
            node = list_next(e, node);
        }
    }
}

// Translated from 0095a3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the player's save data from the load buffer `buffer`
/// (`BGSLoadGameBuffer`, the PC format): the mirror of `fn_009590f0`, with
/// the buffer's version (slot 0 of its vtable) gating the later additions.
/// It stops the sound handle at `+0x77c` first, reads the three actor value
/// arrays and `+0x4ac`, the base class (`008d33f0`), the flagged first-person
/// data (`0084a810`), the player's fields (a changed byte `+0x7c5` clears the
/// NPC's head), then form ids resolved with `004839c0` and
/// `__RTDynamicCast`, and rebuilds the topic list, the lists at `+0x5e4`,
/// `+0xd48`, `+0x87c`, `+0x60c`, `+0x610`, `+0x614`, `+0x618`, `+0xad4`,
/// the quest log, the quest targets and the item array (`+0xdf4`).
/// The exception frame is not translated.
pub fn fn_0095a3b0(e: &mut Engine, this: Ptr<PlayerCharacter>, buffer: u32) {
    let player = this.addr();
    fn_009590d0(e, this, 1);
    e.call(0x00ad_88f0, &args![player + 0x77c]);
    e.call(0x00ad_8d10, &args![player + 0x77c]);
    for base in [0x244u32, 0x378, 0x4b0] {
        for index in 0..0x4du32 {
            buffer_read(e, buffer, player + base + index * 4, 4);
        }
    }
    buffer_read(e, buffer, player + 0x4ac, 4);
    e.call(0x008d_33f0, &args![this, buffer]);
    // The exe's frame (`SUB ESP,0x1f0` under the exception record, then the record link): the
    // flag holder at `EBP-0x134` (+0xcc).
    let wanted = e.with_stack(0x200, |e, frame| {
        let flag_holder = frame.addr() + 0xcc;
        let flags = e.call(0x0042_8110, &args![buffer, flag_holder]).u32();
        e.call(0x0042_80f0, &args![flags, 0x1000_0000u32]).bool()
    });
    if wanted {
        let base = e.call(0x007a_f430, &args![this]).u32();
        let statistics = e.global::<u32>(STATISTICS_OBJECT);
        e.call(0x0084_a810, &args![statistics, 0u32, buffer, base]);
    }
    for (offset, size) in [
        (0x64au32, 1u32),
        (0x64d, 1),
        (0x651, 1),
        (0x652, 1),
        (0x654, 4),
        (0x660, 4),
        (0x664, 4),
        (0x668, 4),
        (0x66c, 1),
        (0x6cc, 1),
        (0x6d0, 4),
        (0x6d4, 4),
        (0x6d8, 1),
        (0x6dc, 4),
        (0x6e8, 1),
        (0x681, 1),
    ] {
        buffer_read(e, buffer, player + offset, size);
    }
    let old_byte = e.mem.u8(player + 0x7c5);
    buffer_read(e, buffer, player + 0x7c5, 1);
    if e.mem.u8(player + 0x7c5) != old_byte && e.vcall(player, 0x1d0, &args![]).u32() != 0 {
        let base = e.call(BASE_FORM_OF, &args![this]).u32();
        e.call(0x005d_d560, &args![base]);
        let holder = e.call(0x008d_8520, &args![this]).u32();
        e.vcall(holder, 0x468, &args![8u32]);
    }
    buffer_read(e, buffer, player + 0x7c6, 1);
    // The same frame: the position at `EBP-0x1c` (+0x1e4); its words are kept for the map
    // marker below.
    let position = e.with_stack(0x200, |e, frame| {
        let position = frame.addr() + 0x1e4;
        e.call(LIST_NODE_ITEM_SLOT, &args![position]);
        buffer_read(e, buffer, player + 0x6e4, 4);
        buffer_read(e, buffer, position, 0xc);
        [0u32, 4, 8].map(|at| e.mem.u32(position + at))
    });
    for (offset, size) in [
        (0x698u32, 4u32),
        (0x67c, 4),
        (0x738, 4),
        (0x658, 1),
        (0x65c, 4),
        (0x674, 4),
        (0x670, 4),
        (0x75c, 1),
        (0x730, 4),
        (0x790, 4),
        (0x680, 1),
        (0x7c4, 1),
        (0x63c, 4),
        (0x640, 4),
        (0x644, 4),
        (0x200, 4),
        (0x240, 1),
        (0x64e, 1),
        (0x66d, 1),
        (0x794, 4),
    ] {
        buffer_read(e, buffer, player + offset, size);
    }
    buffer_read(e, buffer, VANITY_RESTORED_VALUE, 4);
    for (offset, size) in [
        (0xd6cu32, 4u32),
        (0xd70, 4),
        (0x228, 4),
        (0x22c, 4),
        (0x230, 4),
        (0x234, 4),
        (0x608, 1),
        (0xdf2, 1),
    ] {
        buffer_read(e, buffer, player + offset, size);
    }
    if buffer_version(e, buffer) >= 0xd {
        buffer_read(e, buffer, player + 0x64f, 1);
        buffer_read(e, buffer, player + 0x650, 1);
    }
    if buffer_version(e, buffer) >= 0x10 {
        buffer_read(e, buffer, player + 0x7c7, 1);
    }
    if buffer_version(e, buffer) >= 0x12 {
        buffer_read(e, buffer, player + 0x5f8, 1);
    }
    if buffer_version(e, buffer) >= 0x15 {
        buffer_read(e, buffer, player + 0x1fc, 4);
        buffer_read(e, buffer, player + 0x684, 4);
        for index in 0..5u32 {
            buffer_read(e, buffer, player + 0x744 + index * 4, 4);
        }
    }
    e.call(0x008d_5680, &args![player + CHARACTER_PROGRESSION, buffer]);

    // The form ids.
    let quest = buffer_read_form(e, buffer, RTTI_LOAD_QUEST);
    e.mem.set_u32(player + 0x6b8, quest);
    let form = buffer_read_form(e, buffer, RTTI_LOAD_FIRE_NODE);
    e.mem.set_u32(player + 0x73c, form);
    let id = e.call(0x0086_48a0, &args![buffer]).u32();
    let place = e.call(0x0048_39c0, &args![id]).u32();
    e.mem.set_u32(player + 0x6f4, 0);
    if place != 0 {
        let [x, y, z] = position.map(f32::from_bits);
        player_character_set_player_map_marker(e, this, x, y, z, place);
    }
    let region = buffer_read_form(e, buffer, RTTI_LOAD_REGION);
    e.mem.set_u32(player + 0x760, region);
    let region_data = buffer_read_form(e, buffer, RTTI_LOAD_REGION_DATA);
    if e.mem.u32(player + 0x760) != 0 && region_data != 0 {
        let region = e.mem.u32(player + 0x760);
        e.call(0x0070_37c0, &args![region, region_data]);
    }
    e.call(0x0086_48e0, &args![buffer, player + 0x208]);
    if buffer_version(e, buffer) >= 0x12 {
        e.call(0x0086_48e0, &args![buffer, player + 0x224]);
    }
    for offset in [0x638u32, 0x604, 0xd2c, 0xd44] {
        e.call(0x0086_48e0, &args![buffer, player + offset]);
    }
    let count = buffer_read_count(e, buffer);
    let mut index = 0;
    while index < count {
        let topic = buffer_read_form(e, buffer, RTTI_LOAD_TOPIC);
        if topic != 0 {
            append_item(e, LIST_APPEND, player + 0x6a8, topic);
        }
        index += 1;
    }
    e.call(0x0061_a5a0, &args![1u32, player + 0x6a8]);
    let count = buffer_read_count(e, buffer);
    let mut index = 0;
    while index < count {
        let form = buffer_read_form(e, buffer, RTTI_LOAD_LIST_05E4);
        if form != 0 {
            append_item(e, 0x0090_5820, player + 0x5e4, form);
        }
        index += 1;
    }
    let count = buffer_read_count(e, buffer);
    e.call(0x006f_2290, &args![player + 0xdf4, count, 1u32]);
    let mut index = 0;
    while index < count {
        let slot = e.call(0x006a_7ad0, &args![player + 0xdf4, index]).u32();
        let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
        let entry = if block == 0 {
            0
        } else {
            e.call(0x0076_b630, &args![block]).u32()
        };
        e.mem.set_u32(slot, entry);
        e.call(0x004b_ee00, &args![entry, buffer]);
        index += 1;
    }
    e.call(0x0096_9110, &args![this, 1u32]);
    e.call(0x0096_7290, &args![this]);

    // The list at +0xd48 (items of a form pointer and two bytes).
    let count = buffer_read_count(e, buffer);
    if count != 0 {
        create_list_field(e, player, 0xd48);
        let mut index = 0;
        while index < count {
            let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
            let entry = if block == 0 {
                0
            } else {
                e.call(0x0047_81b0, &args![block]).u32()
            };
            e.call(0x0086_48e0, &args![buffer, entry]);
            buffer_read(e, buffer, entry + 4, 1);
            buffer_read(e, buffer, entry + 5, 1);
            let list = e.mem.u32(player + 0xd48);
            append_item(e, LIST_APPEND, list, entry);
            index += 1;
        }
    }
    // The perk list (+0x87c).
    let count = buffer_read_count(e, buffer);
    let mut index = 0;
    while index < count {
        let entry = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let form = buffer_read_form(e, buffer, RTTI_LOAD_PERK);
        e.mem.set_u32(entry, form);
        buffer_read(e, buffer, entry + 4, 1);
        append_item(e, LIST_APPEND, player + 0x87c, entry);
        index += 1;
    }
    purge_dead_entries(e, player + 0x87c);
    // The list at +0x60c.
    let count = buffer_read_count(e, buffer);
    if count != 0 {
        if e.mem.u32(player + 0x60c) != 0 {
            let list = e.mem.u32(player + 0x60c);
            e.call(LIST_CLEAR, &args![list]);
        } else {
            create_list_field(e, player, 0x60c);
        }
        let mut index = 0;
        while index < count {
            let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
            let entry = if block == 0 {
                0
            } else {
                e.call(0x0078_d900, &args![block]).u32()
            };
            buffer_read(e, buffer, entry, 4);
            buffer_read(e, buffer, entry + 4, 4);
            e.call(0x0086_48e0, &args![buffer, entry + 8]);
            let list = e.mem.u32(player + 0x60c);
            append_item(e, LIST_APPEND, list, entry);
            index += 1;
        }
    }
    // The list at +0x610: the old entries are freed first.
    if e.mem.u32(player + 0x610) != 0 {
        let list = e.mem.u32(player + 0x610);
        let mut node = list;
        while node != 0 {
            let item_slot = list_item_slot(e, node);
            let item = e.mem.u32(item_slot);
            e.call(0x0040_1030, &args![item]);
            node = list_next(e, node);
        }
        e.call(LIST_CLEAR, &args![list]);
    }
    if e.mem.u32(player + 0x610) != 0 {
        let list = e.mem.u32(player + 0x610);
        e.call(LIST_CLEAR, &args![list]);
    }
    let count = buffer_read_count(e, buffer);
    if count != 0 {
        if e.mem.u32(player + 0x610) == 0 {
            create_list_field(e, player, 0x610);
        }
        let mut index = 0;
        while index < count {
            let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
            let entry = if block == 0 {
                0
            } else {
                e.call(0x0073_3f50, &args![block]).u32()
            };
            buffer_read(e, buffer, entry, 4);
            buffer_read(e, buffer, entry + 4, 4);
            buffer_read(e, buffer, entry + 8, 2);
            let list = e.mem.u32(player + 0x610);
            append_item(e, LIST_APPEND, list, entry);
            index += 1;
        }
    }
    if e.global::<u32>(LOADED_PERK_FLAG) != 0 {
        e.call(0x0096_9ac0, &args![this]);
    }
    // The lists at +0x614 and +0x618 hold objects that are destroyed
    // through their slot 0x10 before the list is rebuilt.
    for list_offset in [0x614u32, 0x618] {
        if e.mem.u32(player + list_offset) != 0 {
            let mut node = e.mem.u32(player + list_offset);
            while node != 0 {
                let item_slot = list_item_slot(e, node);
                let item = e.mem.u32(item_slot);
                if item != 0 {
                    e.vcall(item, 0x10, &args![1u32]);
                }
                node = list_next(e, node);
            }
            let list = e.mem.u32(player + list_offset);
            e.call(LIST_CLEAR, &args![list]);
        }
        if e.mem.u32(player + list_offset) != 0 {
            let list = e.mem.u32(player + list_offset);
            e.call(LIST_CLEAR, &args![list]);
        }
        let count = buffer_read_count(e, buffer);
        if count != 0 {
            if e.mem.u32(player + list_offset) == 0 {
                create_list_field(e, player, list_offset);
            }
            let mut index = 0;
            while index < count {
                let block = e.call(OPERATOR_NEW, &args![0xbcu32]).u32();
                if block != 0 {
                    e.call(0x0059_a370, &args![block]);
                }
                let object = buffer_read_form(e, buffer, RTTI_LOAD_EFFECT_ITEM);
                let list = e.mem.u32(player + list_offset);
                append_item(e, LIST_APPEND, list, object);
                index += 1;
            }
        }
    }
    for offset in [0x61cu32, 0x620, 0x624, 0x628, 0x62c] {
        buffer_read(e, buffer, player + offset, 4);
    }
    // The quest log and the quest targets.
    let count = buffer_read_count(e, buffer);
    let mut index = 0;
    while index < count {
        let quest = buffer_read_form(e, buffer, RTTI_LOAD_QUEST);
        let stage_done = buffer_read_value(e, buffer, 1);
        let stage = buffer_read_value(e, buffer, 1);
        if quest != 0 {
            let entry = e.call(0x0060_db40, &args![quest, stage_done]).u32();
            if entry != 0 {
                let item = e.call(0x0060_f2c0, &args![entry, stage]).u32();
                if item != 0 {
                    append_item(e, 0x0090_5820, player + 0x6b0, item);
                }
            }
        }
        index += 1;
    }
    let count = buffer_read_count(e, buffer);
    let mut index = 0;
    while index < count {
        let quest = buffer_read_form(e, buffer, RTTI_LOAD_QUEST);
        let delta = buffer_read_value(e, buffer, 4);
        if quest != 0 {
            let target = e.call(0x0060_c8e0, &args![quest, delta]).u32();
            if target != 0 {
                append_item(e, 0x0090_5820, player + 0x6bc, target);
            }
        }
        index += 1;
    }
    let progress = e.mem.u32(player + 0x210);
    e.call(0x0080_6a90, &args![buffer, progress]);
    let group_id = buffer_read_value(e, buffer, 4);
    if group_id != 0 {
        let holder = e.global::<u32>(COMBAT_HOLDER);
        let group = e.call(0x0099_1d60, &args![holder, group_id]).u32();
        e.mem.set_u32(player + 0xd64, group);
    }
    if buffer_version(e, buffer) >= 8 {
        let first = buffer_read_value(e, buffer, 4);
        let second = buffer_read_value(e, buffer, 4);
        e.call(0x008d_1d10, &args![first, second]);
    }
    if buffer_version(e, buffer) >= 0x16 {
        let count = buffer_read_count(e, buffer);
        let mut index = 0;
        while index < count {
            let entry = e.call(OPERATOR_NEW, &args![8u32]).u32();
            let form = buffer_read_form(e, buffer, RTTI_LOAD_PERK);
            e.mem.set_u32(entry, form);
            buffer_read(e, buffer, entry + 4, 1);
            append_item(e, LIST_APPEND, player + 0xad4, entry);
            index += 1;
        }
        purge_dead_entries(e, player + 0xad4);
    }
    if buffer_version(e, buffer) >= 0x17 {
        let hardcore_flag = 1u32;
        // The same frame: the hardcore byte at `EBP-0x11d` (+0xe3), the enabled byte at
        // `EBP-0x11e` (+0xe2).
        let (hardcore, enabled) = e.with_stack(0x200, |e, frame| {
            let (hardcore_at, enabled_at) = (frame.addr() + 0xe3, frame.addr() + 0xe2);
            e.mem.set_u8(hardcore_at, 0);
            e.mem.set_u8(enabled_at, hardcore_flag as u8);
            buffer_read(e, buffer, hardcore_at, 1);
            buffer_read(e, buffer, enabled_at, 1);
            (u32::from(e.mem.u8(hardcore_at)), e.mem.u8(enabled_at))
        });
        e.call(0x0096_9e90, &args![this, hardcore, 1u32]);
        if enabled == 0 {
            e.call(0x005d_e9b0, &args![this]);
        }
    }
    if buffer_version(e, buffer) >= 0x19 {
        buffer_read(e, buffer, player + 0x66e, 1);
    }
    if buffer_version(e, buffer) >= 0x1a {
        for index in 0..8u32 {
            let id = buffer_read_value(e, buffer, 4);
            let form = if id != 0 {
                e.call(0x0048_39c0, &args![id]).u32()
            } else {
                0
            };
            e.call(0x004b_fb70, &args![this, index, form]);
        }
        let holder = e.call(0x008d_8520, &args![this]).u32();
        if holder != 0 {
            let holder = e.call(0x008d_8520, &args![this]).u32();
            if e.vcall(holder, 0x148, &args![]).u32() != 0 {
                let holder = e.call(0x008d_8520, &args![this]).u32();
                e.vcall(holder, 0x168, &args![0u32]);
                let first = e.call(0x008d_8520, &args![this]).u32();
                let second = e.call(0x008d_8520, &args![this]).u32();
                let entry = e.vcall(first, 0x148, &args![]).u32();
                let modded = e.call(0x004b_da70, &args![entry, 2u32]).u8();
                let entry = e.vcall(second, 0x148, &args![]).u32();
                let form = e.call(WORD_AT_8, &args![entry]).u32();
                e.vcall(player, 0x3ec, &args![form, 0u32, u32::from(modded), 1u32]);
            }
        }
    }
    e.call(0x0094_62c0, &args![this, 0u32, 1u32]);
    if buffer_version(e, buffer) >= 0x1b {
        e.call(0x005a_aaf0, &args![buffer]);
    }
}

/// Plays the heartbeat sound `name` on the player's sound handle (`+0x77c`):
/// asks the audio singleton (`00453a70`) for a handle by name
/// (`00ad7550`, 0x31), assigns it (`00418900`), releases the temporary
/// (`00483710`) and plays it (`00ad8830(true)`).
fn play_heartbeat(e: &mut Engine, player: u32, name: u32) {
    e.with_stack(12, |e, handle| {
        let audio = e.call(0x0045_3a70, &args![]).u32();
        let found = e
            .call(0x00ad_7550, &args![audio, handle, name, 0x31u32])
            .u32();
        e.call(0x0041_8900, &args![player + 0x77c, found]);
        e.call(0x0048_3710, &args![handle]);
        e.call(0x00ad_8830, &args![player + 0x77c, 1u32]);
    });
}

// Translated from 0095c0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The last pass of loading the player in the buffer format (the mirror of
/// the id resolution of `fn_00958990`): `008d34b0(buffer)`, then the form
/// ids at `+0x208` and `+0x224` (from version 0x12) go through `0084aa90` of
/// the object `011ddf38` and a cast, the forms at `+0x638`, `+0x604`,
/// `+0xd2c`, `+0xd44` are resolved with `004839c0` and a cast (null stays
/// null), the list at `+0xd48` is resolved and purged of entries that do not
/// resolve to a base object (`007af430`), the list at `+0x60c` is resolved
/// and each target flagged (`00564db0(true)`), then `0095f590` (controls
/// byte `+0x680`), `00806b00(buffer, +0x210)`, the perks (`005eb980`) and the
/// matrix `011a9448` for the camera node. The ratio of actor value `0x10`
/// (slots `0xc` and `0x20` of the `ActorValueOwner`) starts the
/// `UIHealthHeartbeatALP` or `BLP` sound when it is below the two settings.
/// Finally, when the buffer's flags (`00428110`, `0042ce30`) say so,
/// `006047c0` on the base object and slot `0x464` of the process.
/// The exception frame is not translated.
pub fn fn_0095c0a0(e: &mut Engine, this: Ptr<PlayerCharacter>, buffer: u32) {
    let player = this.addr();
    e.call(0x008d_34b0, &args![this, buffer]);
    let statistics = e.global::<u32>(STATISTICS_OBJECT);
    let old_id = e.mem.u32(player + 0x208);
    let resolved = e.call(0x0084_aa90, &args![statistics, old_id]).u32();
    let form = if resolved == 0 {
        0
    } else {
        load_cast_form(e, resolved, RTTI_RESOLVE_0208)
    };
    e.mem.set_u32(player + 0x208, form);
    if buffer_version(e, buffer) >= 0x12 {
        let old_id = e.mem.u32(player + 0x224);
        let resolved = e.call(0x0084_aa90, &args![statistics, old_id]).u32();
        let form = if resolved == 0 {
            0
        } else {
            load_cast_form(e, resolved, RTTI_RESOLVE_0208)
        };
        e.mem.set_u32(player + 0x224, form);
    }
    for (offset, target) in [
        (0x638u32, RTTI_LOAD_FORM_0604),
        (0x604, RTTI_LOAD_FORM_0604),
        (0xd2c, RTTI_RESOLVE_0D2C),
        (0xd44, RTTI_RESOLVE_0D2C),
    ] {
        let id = e.mem.u32(player + offset);
        let form = if id == 0 {
            0
        } else {
            load_cast_form(e, id, target)
        };
        e.mem.set_u32(player + offset, form);
    }
    // The list at +0xd48: resolve every entry, drop the ones without a base.
    let mut node = e.mem.u32(player + 0xd48);
    let mut previous = 0;
    while node != 0 && !list_is_empty(e, node) {
        let item_slot = list_item_slot(e, node);
        let item = e.mem.u32(item_slot);
        let mut dead = false;
        if item == 0 {
            dead = true;
        } else {
            let id = e.mem.u32(item);
            let form = if id == 0 {
                0
            } else {
                load_cast_form(e, id, RTTI_RESOLVE_0D2C)
            };
            e.mem.set_u32(item, form);
            if form == 0 || e.call(0x007a_f430, &args![form]).u32() == 0 {
                e.call(0x0040_1030, &args![item]);
                dead = true;
            }
        }
        if dead {
            if previous != 0 {
                let item_slot = list_item_slot(e, node);
                e.call(0x0090_5330, &args![previous, item_slot]);
                node = list_next(e, previous);
            } else {
                e.call(0x0063_f7b0, &args![node]);
            }
        } else {
            previous = node;
            node = list_next(e, node);
        }
    }
    // The list at +0x60c: resolve the target of every entry.
    let mut node = e.mem.u32(player + 0x60c);
    while node != 0 {
        let item_slot = list_item_slot(e, node);
        let item = e.mem.u32(item_slot);
        if item != 0 {
            let id = e.mem.u32(item + 8);
            let form = if id == 0 {
                0
            } else {
                load_cast_form(e, id, RTTI_LOAD_FORM_0604)
            };
            e.mem.set_u32(item + 8, form);
            if form != 0 {
                e.call(0x0056_4db0, &args![form, 1u32]);
            }
        }
        node = list_next(e, node);
    }
    let disabled = e.mem.u8(player + 0x680);
    e.call(0x0095_f590, &args![this, u32::from(disabled)]);
    let progress = e.mem.u32(player + 0x210);
    e.call(0x0080_6b00, &args![buffer, progress]);
    let mut node = player + 0x87c;
    while node != 0 {
        let item_slot = list_item_slot(e, node);
        let item = e.mem.u32(item_slot);
        if item != 0 && e.mem.u32(item) != 0 {
            let perk = e.mem.u32(item);
            let rank = u32::from(e.mem.u8(item + 4));
            e.call(0x005e_b980, &args![perk, this, rank, 0u32]);
        }
        node = list_next(e, node);
    }
    let camera_node = e.global::<u32>(NODE_CAMERA_1ST_SLOT);
    if camera_node != 0 {
        e.call(0x0043_fa80, &args![camera_node, LOAD_3D_MATRIX]);
    }
    let owner = player + ACTOR_VALUE_OWNER;
    let current = e.vcall(owner, 0xc, &args![0x10u32]).f64();
    let maximum = e.vcall(owner, 0x20, &args![0x10u32]).f64();
    let ratio = (current / maximum) as f32;
    let upper = setting_float(e, HEARTBEAT_UPPER_SETTING);
    let lower = setting_float(e, HEARTBEAT_LOWER_SETTING);
    let above_upper = upper < ratio;
    let above_lower = lower < ratio;
    if !above_upper && above_lower {
        play_heartbeat(e, player, HEARTBEAT_SOUND_ALP);
    } else if !above_lower {
        play_heartbeat(e, player, HEARTBEAT_SOUND_BLP);
    }
    if e.vcall(player, 0x1d0, &args![]).u32() != 0 {
        // The exe's frame (`SUB ESP,0x90` under the exception record, then the record link): the
        // flag holders at `EBP-0x5c` (+0x44) and `EBP-0x60` (+0x40).
        e.with_stack(0xa0, |e, frame| {
            let flags = e
                .call(0x0042_8110, &args![buffer, frame.addr() + 0x44])
                .u32();
            let first = e.call(0x0042_80f0, &args![flags, 0x0800_0020u32]).bool();
            if !first {
                let flags = e
                    .call(0x0042_ce30, &args![buffer, frame.addr() + 0x40])
                    .u32();
                if e.call(0x0042_80f0, &args![flags, 0x0800_0020u32]).bool() {
                    let base = e.call(BASE_FORM_OF, &args![this]).u32();
                    e.call(0x0060_47c0, &args![base, this, 1u32, 1u32, 0u32, 1u32]);
                    let process = process_of(e, this);
                    if process != 0 && e.vcall(process, 0x478, &args![]).bool() {
                        e.vcall(process, 0x464, &args![this]);
                    }
                }
            }
        });
    }
}

// Translated from 0095c730 (decompiled, FalloutNV.exe 1.4.0.525)
/// The final pass of loading the player in the buffer format: `008d34d0`,
/// the field of view (`00567490(00598040)`) when the buffer's flags have
/// bit `0x10`, the wanted camera (`+0x64c`) taken from `+0x64a`, the
/// first-person biped refresh (`ForceTemp3rdPerson`, `00951a10`), the alpha
/// `011a3b38` raised to 1 (`008c4640`), `00952290`, the first-person
/// animation's scene graph (`00493bd0`), `0094ae40(true, false)`, `009466d0`,
/// the camera spring, `SetLastKnownGoodPosition`, the field of view,
/// `00806b50`, `00947d80`, `009480c0`, the quest target list, `007059d0`,
/// `009444d0(0.0)`, iron sights, the pipboy light effect when the player is
/// a spell target of the `0093ccd0` spell, and slot `0x3f4`.
pub fn fn_0095c730(e: &mut Engine, this: Ptr<PlayerCharacter>, buffer: u32) {
    let player = this.addr();
    e.call(0x008d_34d0, &args![this, buffer]);
    // The exe's frame (`SUB ESP,0x14` under `EBP`): the flag holder at `EBP-0x4` (+0x10).
    let wanted = e.with_stack(0x14, |e, frame| {
        let flags = e
            .call(0x0042_8110, &args![buffer, frame.addr() + 0x10])
            .u32();
        e.call(0x0042_80f0, &args![flags, 0x10u32]).bool()
    });
    if wanted {
        let field_of_view = e.call(0x0059_8040, &args![this]).f32();
        e.call(0x0056_7490, &args![this, field_of_view]);
    }
    let third_person = e.mem.u8(player + 0x64a);
    e.mem.set_u8(player + 0x64c, third_person);
    if e.call(0x0095_0bb0, &args![this, 1u32]).u32() != 0 {
        if e.mem.u8(player + 0x64d) != 0 {
            player_character_force_temp_3rd_person(e, this, 1);
        }
        let hide = u8::from(e.mem.u8(player + 0x64a) == 0);
        fn_00951a10(e, this, hide);
    }
    if e.global::<f32>(INVENTORY_ALPHA) < 1.0 {
        e.set_global(INVENTORY_ALPHA, 1.0f32);
        e.call(0x008c_4640, &args![this]);
    }
    // 0095c7ee: the first-person zoom factor and model orientation (00952290).
    e.call(0x0095_2290, &args![this]);
    let animation = player_character_get_animation(e, this, 1);
    e.call(0x0049_3bd0, &args![animation]);
    e.call(0x0094_ae40, &args![this, 1u32, 0u32]);
    let game_state = e.global::<u32>(GAME_STATE_POINTER);
    if e.call(0x005b_b4d0, &args![game_state]).bool() {
        e.call(0x0094_66d0, &args![this, 0.0f32, 1u32]);
    }
    if e.mem.u32(player + 0x638) != 0 && e.mem.u32(player + 0x63c) != 3 {
        let body = e.mem.u32(player + 0x638);
        let mode = e.mem.u32(player + 0x63c);
        let strength = e.mem.f32(player + 0x644);
        e.call(0x0095_f930, &args![this, body, mode, strength]);
        e.call(0x0070_5ad0, &args![2u32]);
    }
    e.call(0x0094_7c90, &args![this]);
    let fov = e.mem.f32(player + 0x65c);
    fn_00950610(e, this, fov);
    let progress = e.mem.u32(player + 0x210);
    e.call(0x0080_6b50, &args![buffer, progress]);
    e.call(0x0094_7d80, &args![this]);
    e.call(0x0094_80c0, &args![this]);
    let quest = e.mem.u32(player + 0x6b8);
    if quest != 0 {
        e.call(0x0060_f110, &args![quest, player + 0x6c4, player + 0x6bc]);
    }
    e.call(0x0070_59d0, &args![]);
    e.call(0x0094_44d0, &args![this, 0.0f32]);
    if e.call(0x008b_bc10, &args![this]).bool() {
        let holder = e.call(0x008d_8520, &args![this]).u32();
        e.vcall(holder, 0x400, &args![0u32]);
        e.call(0x008b_b650, &args![this, 1u32, 0u32, 0u32]);
    }
    let spell = e.call(0x0093_ccd0, &args![]).u32();
    let spell_target = if spell == 0 { 0 } else { spell + 0x18 };
    if e.call(
        0x0082_2b90,
        &args![player + MAGIC_TARGET, spell_target, 1u32],
    )
    .bool()
    {
        // 0095c97e / 0095c990: `00705990` takes no word (plain RET, no stack read); the
        // three pushed words are those of `007fa310` (RET 0xc).
        let pipboy = e.call(0x0070_5990, &args![]).u32();
        e.call(0x007f_a310, &args![pipboy, 1u32, 1u32, 1u32]);
        let pipboy = e.call(0x0070_5990, &args![]).u32();
        e.call(0x007f_a310, &args![pipboy, 0u32, 1u32, 1u32]);
    }
    e.vcall(player, 0x3f4, &args![]);
}

// Translated from 0095c9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the player to its start state before a revert (`Revert` of the
/// save game): `008d34f0`, the first-person animation (`0049bb70`) when the
/// buffer's flags (`0042ce30`) have bit `0x10000000`, the biped
/// (`00c81db0`), the field of view (`00567490(1.0)`) with bit `0x10`, the
/// globals (`011e0784`, `011e078c`, vanity, camera values), the player's
/// bytes and words, the lists (`00470470` on `+0x610`, `+0x614`, `+0x618`,
/// the global `011e0ae8`, `+0x84c`, `+0x6a8`, `+0x6bc`, `+0x6b0`, `+0x5e4`,
/// `+0x6c4`), the modifier arrays, the objects of the lists at `+0x60c`,
/// `+0x87c`, `+0xad4`, the map marker, and finally the quest topics are
/// rebuilt from the list `004612e0` (`011c3f2c`) with `00619410`.
/// The exception frame is not translated.
pub fn fn_0095c9c0(e: &mut Engine, this: Ptr<PlayerCharacter>, buffer: u32) {
    let player = this.addr();
    e.call(0x008d_34f0, &args![this, buffer]);
    // The exe's frame (`SUB ESP,0x44` under `EBP`): the flag holders at `EBP-0x20` (+0x24) and
    // `EBP-0x24` (+0x20).
    let wanted = e.with_stack(0x44, |e, frame| {
        let flags = e
            .call(0x0042_ce30, &args![buffer, frame.addr() + 0x24])
            .u32();
        let wanted = e.call(0x0042_80f0, &args![flags, 0x1000_0000u32]).bool();
        if wanted && e.mem.u32(player + 0x690) != 0 {
            let animation = e.mem.u32(player + 0x690);
            e.call(0x0049_bb70, &args![animation, buffer]);
        }
        let biped = e.call(0x0095_0bb0, &args![this, 0u32]).u32();
        e.call(0x00c8_1db0, &args![biped, 0u32]);
        let flags = e
            .call(0x0042_ce30, &args![buffer, frame.addr() + 0x20])
            .u32();
        e.call(0x0042_80f0, &args![flags, 0x10u32]).bool()
    });
    if wanted {
        e.call(0x0056_7490, &args![this, 1.0f32]);
    }
    e.set_global(SAVED_FORM_011E0784, 0u32);
    e.set_global(SAVED_FORM_011E078C, 0u32);
    player_character_stop_vanity_mode(e, this.cast());
    e.mem.set_u8(player + 0x608, 0);
    e.set_global(VANITY_VALUE_07DC, 0.0f32);
    e.set_global(CAMERA_VALUE_0768, 0.0f32);
    let minus_one = e.global::<f32>(MINUS_ONE_FLOAT);
    e.set_global(INVENTORY_ALPHA_SAVE, minus_one);
    e.set_global(IRON_SIGHTS_SKIP_FLAG, 0u8);
    e.set_global(CAMERA_WORD_0788, 0u32);
    e.mem.set_u8(player + 0x66e, 1);
    e.mem.set_u8(player + 0x64a, 0);
    e.mem.set_u8(player + 0x64c, 0);
    e.mem.set_u32(player + 0xd64, 0);
    e.mem.set_u32(player + 0xd68, 0);
    e.mem.set_f32(player + 0xd6c, 0.0);
    e.mem.set_f32(player + 0xd70, 0.0);
    for offset in [0x798u32, 0x799, 0x79a, 0x79b, 0x681] {
        e.mem.set_u8(player + offset, 0);
    }
    e.mem.set_u8(player + 0x75d, 1);
    let default_value = e.global::<f32>(DEFAULT_TIMER_VALUE);
    e.mem.set_f32(player + 0x684, default_value);
    if e.call(0x0056_2d00, &args![buffer]).bool() {
        e.mem.set_u8(player + 0x7c5, 0);
    }
    for offset in [0x7c6u32, 0x7c7, 0x5f8, 0x680, 0x64d, 0x64e, 0x64f, 0x650] {
        e.mem.set_u8(player + offset, 0);
    }
    e.set_global(CAMERA_SWITCH_FLAG_011F21D0, 0u8);
    e.mem.set_u8(player + 0x7c7, 0);
    let first = setting_float(e, VIEW_OFFSET_SETTING_A);
    e.mem.set_f32(player + 0x670, first);
    let second = setting_float(e, VIEW_OFFSET_SETTING_B);
    e.mem.set_f32(player + 0x674, second);
    e.mem.set_u32(player + 0x228, 0);
    e.mem.set_f32(player + 0x22c, 0.0);
    e.mem.set_u32(player + 0x230, 0);
    e.mem.set_f32(player + 0x234, 0.0);
    e.mem.set_u8(player + 0x608, 0);
    e.mem.set_u8(player + 0xdf2, 0);
    e.mem.set_u32(player + 0x654, 0);
    e.mem.set_f32(player + 0x6e4, 0.0);
    for index in 0..5u32 {
        e.mem.set_u32(player + 0x744 + index * 4, 0);
    }
    for offset in [0x610u32, 0x614, 0x618] {
        let list = e.mem.u32(player + offset);
        e.call(LIST_CLEAR, &args![list]);
    }
    for offset in [0x61cu32, 0x620, 0x624, 0x628, 0x62c, 0x1fc] {
        e.mem.set_u32(player + offset, 0);
    }
    e.mem.set_f32(player + 0x684, default_value);
    e.mem.set_u32(player + 0x790, 0);
    e.call(0x0085_1d10, &args![this]);
    e.mem.set_f32(player + 0x4ac, 0.0);
    for index in 0..0x4du32 {
        e.mem.set_f32(player + 0x244 + index * 4, 0.0);
        e.mem.set_f32(player + 0x378 + index * 4, 0.0);
        e.mem.set_f32(player + 0x4b0 + index * 4, 0.0);
    }
    e.call(LIST_CLEAR, &args![0x011e_0ae8u32]);
    e.call(0x0085_1d10, &args![this]);
    e.call(0x0096_1280, &args![this]);
    for offset in [0x84cu32, 0x6a8, 0x6bc, 0x6b0, 0x5e4, 0x6c4] {
        e.call(LIST_CLEAR, &args![player + offset]);
    }
    e.call(0x0096_90a0, &args![this]);
    e.call(0x0096_7290, &args![this]);
    e.call(0x008d_56a0, &args![player + CHARACTER_PROGRESSION, buffer]);
    e.mem.set_u32(player + 0x208, 0);
    e.mem.set_u32(player + 0x224, 0);
    e.mem.set_u32(player + 0x1f0, 0);
    player_character_remove_player_map_marker(e, this);
    e.mem.set_u32(player + 0x604, 0);
    fn_00950010(e, this, 0);
    // The list at +0x60c: free the objects, then the list itself.
    while e.mem.u32(player + 0x60c) != 0 {
        let list = e.mem.u32(player + 0x60c);
        let item_slot = list_item_slot(e, list);
        if e.mem.u32(item_slot) == 0 {
            break;
        }
        let list = e.mem.u32(player + 0x60c);
        let item_slot = list_item_slot(e, list);
        let item = e.mem.u32(item_slot);
        e.call(0x0040_1030, &args![item]);
        let list = e.mem.u32(player + 0x60c);
        e.call(0x0063_f7b0, &args![list]);
    }
    let list = e.mem.u32(player + 0x60c);
    if list != 0 {
        e.call(0x0047_02f0, &args![list, 1u32]);
    }
    e.mem.set_u32(player + 0x60c, 0);
    for (list_head, first_extra, extra_count) in
        [(0x87cu32, 0x884u32, 0x4au32), (0xad4, 0xadc, 0x4a)]
    {
        let mut node = player + list_head;
        while node != 0 {
            let item_slot = list_item_slot(e, node);
            let item = e.mem.u32(item_slot);
            e.call(0x0040_1030, &args![item]);
            node = list_next(e, node);
        }
        e.call(LIST_CLEAR, &args![player + list_head]);
        for index in 0..extra_count {
            e.call(LIST_CLEAR, &args![player + first_extra + index * 8]);
        }
    }
    let name_buffer = e.mem.u32(player + 0x1ec);
    e.call(0x0040_1030, &args![name_buffer]);
    e.mem.set_u32(player + 0x1ec, 0);
    e.mem.set_u8(player + 0xe38, 1);
    e.mem.set_u32(player + 0xe0c, 0);
    e.call(0x008b_bbf0, &args![this, 0u32]);
    let progress = e.mem.u32(player + 0x210);
    e.call(0x0080_6ba0, &args![buffer, progress]);
    let holder = e.global::<u32>(0x011c_3f2c);
    let mut node = e.call(0x0046_12e0, &args![holder]).u32();
    while node != 0 {
        let item_slot = list_item_slot(e, node);
        let item = e.mem.u32(item_slot);
        if item != 0 && e.call(0x0061_9410, &args![item]).bool() {
            append_item(e, LIST_APPEND, player + 0x6a8, item);
        }
        node = list_next(e, node);
    }
    let head = e.call(0x0046_4e30, &args![this]).u32();
    e.call(0x0061_a5a0, &args![1u32, head]);
    e.call(0x0094_7d10, &args![this]);
    e.call(0x0094_8050, &args![this]);
    e.call(0x0052_4d70, &args![this]);
    e.call(0x005a_ae20, &args![buffer]);
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0094c490, fn_0094c490(Ptr, u32) -> f32),
        entry!(0x0094c4c0, fn_0094c4c0(Ptr, u32) -> f32),
        entry!(
            0x0094c4f0,
            fn_0094c4f0(Ptr<PlayerCharacter>, i32, i32, f32, u32)
        ),
        entry!(0x0094c640, fn_0094c640(Ptr, u32, u32) -> f32),
        entry!(0x0094c660, fn_0094c660(Ptr, u32, f32)),
        entry!(
            0x0094c680,
            player_character_add_spell(Ptr<PlayerCharacter>, Ptr) -> bool
        ),
        entry!(
            0x0094c6c0,
            player_character_spell_learned(Ptr<PlayerCharacter>, Ptr)
        ),
        entry!(0x0094c7a0, fn_0094c7a0(Ptr<PlayerCharacter>, Ptr) -> bool),
        entry!(0x0094c800, fn_0094c800(Ptr, u32, u32, Ptr, u8) -> bool),
        entry!(0x0094c870, fn_0094c870(Ptr, u32, u8)),
        entry!(0x0094c890, fn_0094c890(Ptr, u32, u32, u8)),
        entry!(0x0094c8c0, fn_0094c8c0(Ptr<PlayerCharacter>) -> u32),
        entry!(
            0x0094c8e0,
            player_character_set_selected_spell(Ptr<PlayerCharacter>, u32)
        ),
        entry!(0x0094c950, fn_0094c950(Ptr<PlayerCharacter>, Ptr)),
        entry!(0x0094ca20, fn_0094ca20(Ptr, u32, u32, u32, u8) -> bool),
        entry!(0x0094ca50, fn_0094ca50(Ptr) -> u32),
        entry!(0x0094cae0, fn_0094cae0(Ptr<PlayerCharacter>, Ptr)),
        entry!(0x0094db80, fn_0094db80(Ptr, u8)),
        entry!(0x0094dba0, fn_0094dba0() -> u8),
        entry!(0x0094dbb0, fn_0094dbb0(Ptr) -> Ptr),
        entry!(
            0x0094dbe0,
            player_character_return_to_last_known_good_position(Ptr<PlayerCharacter>, bool)
        ),
        entry!(0x0094df30, fn_0094df30(Ptr<PlayerCharacter>, Ptr) -> Ptr),
        entry!(
            0x0094df60,
            player_character_is_sleepingor_resting(Ptr<PlayerCharacter>) -> bool
        ),
        entry!(0x0094df80, fn_0094df80(Ptr<PlayerCharacter>)),
        entry!(
            0x0094e1d0,
            player_character_load_3d(Ptr<PlayerCharacter>, u8) -> u32
        ),
        entry!(0x0094eac0, fn_0094eac0() -> u32),
        entry!(0x0094ead0, fn_0094ead0() -> u32),
        entry!(0x0094eae0, fn_0094eae0() -> u32),
        entry!(0x0094eaf0, fn_0094eaf0(Ptr, Ptr, bool)),
        entry!(0x0094eb30, fn_0094eb30() -> u32),
        entry!(0x0094eb40, fn_0094eb40(Ptr<PlayerCharacter>, Ptr, u8)),
        entry!(0x0094ec90, build_inv_string_callback(Ptr, Ptr) -> bool),
        entry!(
            0x0094ed40,
            player_character_export_progress_data(Ptr<PlayerCharacter>, i32)
        ),
        entry!(0x0094fce0, fn_0094fce0(Ptr<PlayerCharacter>, u32, u32)),
        entry!(0x0094fd10, fn_0094fd10(Ptr<PlayerCharacter>, u32)),
        entry!(
            0x0094fd30,
            player_character_reward_karma(Ptr<PlayerCharacter>, i32)
        ),
        entry!(0x00950010, fn_00950010(Ptr<PlayerCharacter>, u8)),
        entry!(
            0x00950030,
            player_character_update_player_3d(Ptr<PlayerCharacter>)
        ),
        entry!(0x00950090, fn_00950090(Ptr) -> u8),
        entry!(0x009500a0, player_character_stop_vanity_mode(Ptr)),
        entry!(
            0x00950110,
            player_character_set_first_person(Ptr<PlayerCharacter>, u8) -> bool
        ),
        entry!(
            0x00950290,
            player_character_update_first_person_zoom(Ptr<PlayerCharacter>)
        ),
        entry!(
            0x00950340,
            player_character_force_temp_3rd_person(Ptr<PlayerCharacter>, u8) -> bool
        ),
        entry!(
            0x009503d0,
            player_character_update_temp_3rd_person(Ptr<PlayerCharacter>)
        ),
        entry!(
            0x00950460,
            player_character_force_temp_1st_person(Ptr<PlayerCharacter>, u8) -> bool
        ),
        entry!(
            0x00950530,
            player_character_update_temp_1st_person(Ptr<PlayerCharacter>)
        ),
        entry!(0x00950610, fn_00950610(Ptr<PlayerCharacter>, f32) -> f32),
        entry!(
            0x00950660,
            player_character_resurrect(Ptr<PlayerCharacter>, u32, u32, u32)
        ),
        entry!(
            0x009508b0,
            fn_009508b0(Ptr<PlayerCharacter>, u32, f32, u8) -> bool
        ),
        entry!(0x009508f0, fn_009508f0(Ptr<PlayerCharacter>, u32, f32)),
        entry!(0x00950930, fn_00950930(Ptr<PlayerCharacter>)),
        entry!(0x00950a10, fn_00950a10(Ptr<PlayerCharacter>) -> u32),
        entry!(
            0x00950a60,
            player_character_get_animation(Ptr<PlayerCharacter>, u8) -> u32
        ),
        entry!(0x00950a90, fn_00950a90(Ptr<PlayerCharacter>) -> u32),
        entry!(0x00950ad0, fn_00950ad0(Ptr<PlayerCharacter>) -> u32),
        entry!(
            0x00950b00,
            player_character_get_biped(Ptr<PlayerCharacter>, u8) -> u32
        ),
        entry!(
            0x00950b30,
            player_character_is_1st_person_biped(Ptr<PlayerCharacter>, u32) -> bool
        ),
        entry!(0x00950b60, fn_00950b60(Ptr<PlayerCharacter>) -> u32),
        entry!(0x00950bb0, fn_00950bb0(Ptr<PlayerCharacter>, u8) -> u32),
        entry!(
            0x00950be0,
            player_character_get_current_3d(Ptr<PlayerCharacter>) -> u32
        ),
        entry!(
            0x00950c20,
            player_character_clone_inventory_3d(Ptr<PlayerCharacter>, u8)
        ),
        entry!(0x00951900, fn_00951900() -> u8),
        entry!(0x00951910, fn_00951910(u8)),
        entry!(0x00951920, fn_00951920() -> u32),
        entry!(0x00951930, fn_00951930() -> u32),
        entry!(0x00951940, fn_00951940(Ptr, u32)),
        entry!(0x00951a10, fn_00951a10(Ptr<PlayerCharacter>, u8)),
        entry!(0x009520f0, fn_009520f0(Ptr<PlayerCharacter>, u16, u32)),
        entry!(0x00952290, fn_00952290(Ptr<PlayerCharacter>)),
        entry!(0x009526a0, player_character_set_god_mode(u8)),
        entry!(0x009526b0, player_character_is_god_mode() -> u8),
        entry!(0x009526e0, player_character_set_demigod_mode(u8)),
        entry!(0x009526f0, player_character_is_demigod_mode() -> u8),
        entry!(0x00952720, fn_00952720(Ptr) -> u8),
        entry!(
            0x00952730,
            fn_00952730(Ptr<PlayerCharacter>, i32, f32) -> bool
        ),
        entry!(0x009527b0, fn_009527b0(Ptr<PlayerCharacter>, u32)),
        entry!(
            0x00952830,
            fn_00952830(Ptr<PlayerCharacter>, u32, u8, u32) -> bool
        ),
        entry!(
            0x009528c0,
            player_character_add_quest_stage_item(Ptr<PlayerCharacter>, u32) -> bool
        ),
        entry!(
            0x009529d0,
            player_character_set_active_quest(Ptr<PlayerCharacter>, u32)
        ),
        entry!(0x00952a20, fn_00952a20(Ptr<PlayerCharacter>, u32)),
        entry!(0x00952b30, fn_00952b30(Ptr<PlayerCharacter>) -> u32),
        entry!(
            0x00952ba0,
            player_character_get_current_target_list(Ptr<PlayerCharacter>) -> u32
        ),
        entry!(
            0x00952c30,
            player_character_check_for_quest_target_update(Ptr<PlayerCharacter>, u32)
        ),
        entry!(
            0x00952d60,
            player_character_build_path_to_target(Ptr<PlayerCharacter>, u32, u32, u32)
        ),
        entry!(
            0x00952e60,
            player_character_set_player_map_marker(Ptr<PlayerCharacter>, f32, f32, f32, u32)
        ),
        entry!(
            0x00952f90,
            player_character_remove_player_map_marker(Ptr<PlayerCharacter>)
        ),
        entry!(0x00952ff0, fn_00952ff0(Ptr<PlayerCharacter>, u32) -> u32),
        entry!(
            0x00953060,
            player_character_focus_on_actor(Ptr<PlayerCharacter>, u32, f32, u8)
        ),
        entry!(
            0x00953c20,
            player_character_get_number_actors_in_combat(Ptr<PlayerCharacter>) -> u32
        ),
        entry!(
            0x00953c50,
            player_character_is_player_character_in_combat(Ptr<PlayerCharacter>, u32) -> u8
        ),
        entry!(0x00953c80, fn_00953c80(Ptr<PlayerCharacter>) -> bool),
        entry!(
            0x00953ce0,
            player_character_reset_player_greet_flag(Ptr<PlayerCharacter>)
        ),
        entry!(0x00953d00, fn_00953d00(Ptr<PlayerCharacter>)),
        entry!(0x00953d40, fn_00953d40(Ptr<PlayerCharacter>)),
        entry!(
            0x00953d80,
            player_character_initiate_sit_sleep_package(Ptr<PlayerCharacter>, u32)
        ),
        entry!(
            0x00953ee0,
            player_character_initiate_get_up_package(Ptr<PlayerCharacter>)
        ),
        entry!(
            0x00953f20,
            player_character_get_heading(Ptr<PlayerCharacter>, u32) -> f32
        ),
        entry!(0x00953f60, fn_00953f60(Ptr<PlayerCharacter>) -> bool),
        entry!(
            0x00953f80,
            player_character_set_is_a_murderer(Ptr<PlayerCharacter>)
        ),
        entry!(0x00953fb0, fn_00953fb0(Ptr<PlayerCharacter>) -> u8),
        entry!(0x00953fd0, fn_00953fd0(Ptr<PlayerCharacter>, u8)),
        entry!(0x00953ff0, fn_00953ff0(Ptr<PlayerCharacter>, u32, u32, u8)),
        entry!(
            0x00954610,
            fn_00954610(Ptr<PlayerCharacter>, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00954910, fn_00954910(Ptr, u8)),
        entry!(
            0x00954960,
            fn_00954960(Ptr<PlayerCharacter>, u8, u32) -> f32
        ),
        entry!(0x009549a0, fn_009549a0(Ptr, u32) -> u8),
        entry!(0x00954a70, fn_00954a70(Ptr, u32, u8) -> u8),
        entry!(0x00954cc0, fn_00954cc0(Ptr<PlayerCharacter>) -> bool),
        entry!(0x00954d40, fn_00954d40(Ptr<PlayerCharacter>, u32) -> u16),
        entry!(0x00955620, fn_00955620(Ptr<PlayerCharacter>, u32)),
        entry!(0x00956f70, fn_00956f70(Ptr<PlayerCharacter>, u32, u32)),
        entry!(0x00958990, fn_00958990(Ptr<PlayerCharacter>, u32, u32)),
        entry!(0x00958ec0, fn_00958ec0(Ptr<PlayerCharacter>, u32, u32)),
        entry!(0x00958fc0, fn_00958fc0(Ptr<PlayerCharacter>, u32)),
        entry!(0x009590d0, fn_009590d0(Ptr<PlayerCharacter>, u8)),
        entry!(0x009590f0, fn_009590f0(Ptr<PlayerCharacter>, u32)),
        entry!(0x0095a3b0, fn_0095a3b0(Ptr<PlayerCharacter>, u32)),
        entry!(0x0095c0a0, fn_0095c0a0(Ptr<PlayerCharacter>, u32)),
        entry!(0x0095c730, fn_0095c730(Ptr<PlayerCharacter>, u32)),
        entry!(0x0095c9c0, fn_0095c9c0(Ptr<PlayerCharacter>, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::atomic::{AtomicU32, Ordering};

    /// The source of this file: every code address it mentions is a callee
    /// (or one of its own functions), and gets a test double below.
    const SOURCE: &str = include_str!("playercharacter_p2.rs");

    static NEXT_TARGET: AtomicU32 = AtomicU32::new(0x0300_0000);

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// Every address in the code range that the source mentions.
    fn mentioned_code_addresses() -> Vec<u32> {
        let bytes = SOURCE.as_bytes();
        let mut found = Vec::new();
        let mut i = 0;
        while i + 2 < bytes.len() {
            if bytes[i] == b'0' && bytes[i + 1] == b'x' {
                let mut j = i + 2;
                let mut digits = String::new();
                while j < bytes.len() && (bytes[j].is_ascii_hexdigit() || bytes[j] == b'_') {
                    if bytes[j] != b'_' {
                        digits.push(bytes[j] as char);
                    }
                    j += 1;
                }
                if let Ok(value) = u32::from_str_radix(&digits, 16) {
                    if (0x0040_1000..0x00fd_f000).contains(&value) && !found.contains(&value) {
                        found.push(value);
                    }
                }
                i = j;
            } else {
                i += 1;
            }
        }
        found
    }

    /// An engine with the data pages the code reads, and a double that answers
    /// zero over every callee outside this file.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0010_0000);
        e.map(0x0118_0000, 0x0010_0000);
        e.map(0x0600_0000, 0x0001_0000);
        let own: Vec<u32> = funcs().iter().map(|(address, _)| *address).collect();
        for address in mentioned_code_addresses() {
            if !own.contains(&address) {
                e.register_double(address, |_, _| Ret::default());
            }
        }
        e
    }

    fn double(e: &mut Engine, address: u32, ret: Ret) {
        e.register_double(address, move |_, _| ret);
    }

    /// The arguments of every logged call to `address`.
    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn start_log(e: &mut Engine) {
        e.call_log = Some(Vec::new());
    }

    fn new_player(e: &mut Engine) -> Ptr<PlayerCharacter> {
        e.new_object::<PlayerCharacter>()
    }

    /// Puts a double answering `ret` into slot `offset` of the object's vtable
    /// (a fresh table when it has none).
    fn slot(e: &mut Engine, object: u32, offset: u32, ret: Ret) -> u32 {
        let mut table = e.mem.u32(object);
        if table == 0 {
            table = e.mem.alloc(0x800);
            e.mem.set_u32(object, table);
        }
        let target = NEXT_TARGET.fetch_add(4, Ordering::Relaxed);
        e.mem.set_u32(table + offset, target);
        double(e, target, ret);
        target
    }

    /// Like [`slot`] with a double that records its argument words.
    fn recording_slot(e: &mut Engine, object: u32, offset: u32, ret: Ret) -> CallRecord {
        let mut table = e.mem.u32(object);
        if table == 0 {
            table = e.mem.alloc(0x800);
            e.mem.set_u32(object, table);
        }
        let target = NEXT_TARGET.fetch_add(4, Ordering::Relaxed);
        e.mem.set_u32(table + offset, target);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let record = seen.clone();
        e.register_double(target, move |_, words| {
            record.borrow_mut().push(words.to_vec());
            ret
        });
        seen
    }

    /// Makes the list helpers behave as the game's: a node is `{item, next}`,
    /// the slot of a node is the node, the head test is "both words zero".
    fn list_helpers(e: &mut Engine) {
        e.register(LIST_NODE_ITEM_SLOT, |_, a| eax(a[0]));
        e.register(LIST_NODE_NEXT, |e, a| eax(e.mem.u32(a[0] + 4)));
        e.register(LIST_IS_EMPTY, |e, a| {
            eax((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
    }

    /// A list node `{item, next}`.
    fn node(e: &mut Engine, item: u32, next: u32) -> u32 {
        let block = e.mem.alloc(8);
        e.mem.set_u32(block, item);
        e.mem.set_u32(block + 4, next);
        block
    }

    type CallRecord = Rc<RefCell<Vec<Vec<u32>>>>;

    // ----- actor value modifiers -----

    #[test]
    fn modifier_getters_ask_for_the_player_and_their_kind() {
        let mut e = engine();
        let player = new_player(&mut e);
        double(&mut e, GET_MODIFIER, st0(1.5));
        start_log(&mut e);
        let owner = Ptr::<()>::new(player.addr() + 0xa4);
        assert_eq!(fn_0094c490(&mut e, owner, 7), 1.5);
        assert_eq!(fn_0094c4c0(&mut e, owner, 9), 1.5);
        assert_eq!(
            calls_to(&e, GET_MODIFIER),
            vec![vec![player.addr(), 2, 7], vec![player.addr(), 1, 9]]
        );
    }

    fn clamp_adds(e: &mut Engine) {
        e.register(CLAMP_ADD, |_, a| {
            st0((f32::from_bits(a[0]) + f32::from_bits(a[1])) as f64)
        });
        e.register(CHECK_CLAMP_DAMAGE_MODIFIER, |_, a| {
            st0((f32::from_bits(a[2]) * 2.0) as f64)
        });
    }

    #[test]
    fn modifier_change_updates_the_temporary_and_script_arrays() {
        let mut e = engine();
        clamp_adds(&mut e);
        let player = new_player(&mut e);
        e.mem.set_f32(player.addr() + 0x244 + 3 * 4, 2.0);
        e.mem.set_f32(player.addr() + 0x378 + 5 * 4, 10.0);
        start_log(&mut e);
        fn_0094c4f0(&mut e, player, 0, 3, 1.5, 0);
        fn_0094c4f0(&mut e, player, 1, 5, -4.0, 1);
        assert_eq!(e.mem.f32(player.addr() + 0x244 + 12), 3.5);
        assert_eq!(e.mem.f32(player.addr() + 0x378 + 20), 6.0);
        let adds = calls_to(&e, CLAMP_ADD);
        assert_eq!(adds[0][2], 0);
        assert_eq!(adds[1][2], 1);
    }

    #[test]
    fn modifier_change_for_no_actor_value_or_unknown_mode_does_nothing() {
        let mut e = engine();
        clamp_adds(&mut e);
        let player = new_player(&mut e);
        start_log(&mut e);
        fn_0094c4f0(&mut e, player, 0, -1, 1.0, 0);
        fn_0094c4f0(&mut e, player, 1, -1, 1.0, 0);
        fn_0094c4f0(&mut e, player, 7, 3, 1.0, 0);
        assert!(calls_to(&e, CLAMP_ADD).is_empty());
        assert!(calls_to(&e, CHECK_CLAMP_DAMAGE_MODIFIER).is_empty());
    }

    #[test]
    fn damage_modifier_goes_through_the_clamp_check_first() {
        let mut e = engine();
        clamp_adds(&mut e);
        let player = new_player(&mut e);
        e.mem.set_f32(player.addr() + 0x4b0 + 4 * 4, 1.0);
        e.mem.set_f32(player.addr() + 0x4ac, 10.0);
        start_log(&mut e);
        // Actor value 4: the damage array; the check doubles the delta.
        fn_0094c4f0(&mut e, player, 2, 4, 3.0, 0);
        assert_eq!(e.mem.f32(player.addr() + 0x4b0 + 16), 7.0);
        let checks = calls_to(&e, CHECK_CLAMP_DAMAGE_MODIFIER);
        assert_eq!(checks[0][0], player.addr() + 0xa4);
        assert_eq!(checks[0][1], 4);
        // Actor value 0x10 has its own word.
        fn_0094c4f0(&mut e, player, 2, 0x10, 1.0, 0);
        assert_eq!(e.get(player, PlayerCharacter::fHealthModifier), 12.0);
        // No actor value: the check still runs, nothing is stored.
        fn_0094c4f0(&mut e, player, 2, -1, 1.0, 0);
        assert_eq!(calls_to(&e, CHECK_CLAMP_DAMAGE_MODIFIER).len(), 3);
        assert_eq!(calls_to(&e, CLAMP_ADD).len(), 2);
    }

    #[test]
    fn modifier_item_forwarders_pass_everything_on() {
        let mut e = engine();
        let player = new_player(&mut e);
        double(&mut e, MODIFIER_ITEM_GETTER, st0(2.5));
        start_log(&mut e);
        assert_eq!(fn_0094c640(&mut e, player.cast(), 4, 5), 2.5);
        fn_0094c660(&mut e, player.cast(), 6, 0.25);
        assert_eq!(
            calls_to(&e, MODIFIER_ITEM_GETTER),
            vec![vec![player.addr(), 4, 5]]
        );
        assert_eq!(
            calls_to(&e, MODIFIER_ITEM_SETTER),
            vec![vec![player.addr(), 6, 0.25f32.to_bits()]]
        );
    }

    // ----- spells -----

    /// A spell object whose interface (at `+0x18`) answers `kind` in slot 0x18.
    fn spell_with_kind(e: &mut Engine, kind: u32) -> u32 {
        let spell = e.mem.alloc(0x100);
        slot(e, spell + 0x18, 0x18, eax(kind));
        spell
    }

    #[test]
    fn learning_a_spell_of_a_plain_kind_does_nothing() {
        let mut e = engine();
        list_helpers(&mut e);
        let player = new_player(&mut e);
        let spell = spell_with_kind(&mut e, 1);
        e.mem.set_u32(spell + 0x28, 0x1234);
        start_log(&mut e);
        player_character_spell_learned(&mut e, player, Ptr::new(spell));
        assert!(calls_to(&e, 0x0040_7e00).is_empty());
    }

    #[test]
    fn learning_a_spell_walks_its_effect_list() {
        let mut e = engine();
        list_helpers(&mut e);
        e.register(0x0082_5c00, |_, a| eax(a[0] + 0x1000));
        let player = new_player(&mut e);
        for kind in [0, 4, 2, 3] {
            let spell = spell_with_kind(&mut e, kind);
            // Two entries, the second without an item, then the end.
            let third = node(&mut e, 0x30, 0);
            let second = node(&mut e, 0, third);
            e.mem.set_u32(spell + 0x28, 0x10);
            e.mem.set_u32(spell + 0x2c, second);
            start_log(&mut e);
            player_character_spell_learned(&mut e, player, Ptr::new(spell));
            assert_eq!(
                calls_to(&e, 0x0040_7e00),
                vec![
                    vec![0x10 + 0x1000, 0x0020_0000, 1],
                    vec![0x30 + 0x1000, 0x0020_0000, 1]
                ],
                "kind {kind}"
            );
        }
    }

    #[test]
    fn adding_a_spell_learns_it_only_when_it_was_added() {
        let mut e = engine();
        list_helpers(&mut e);
        let player = new_player(&mut e);
        let spell = spell_with_kind(&mut e, 1);
        double(&mut e, ACTOR_ADD_SPELL, eax(0));
        start_log(&mut e);
        assert!(!player_character_add_spell(&mut e, player, Ptr::new(spell)));
        double(&mut e, ACTOR_ADD_SPELL, eax(1));
        assert!(player_character_add_spell(&mut e, player, Ptr::new(spell)));
        let table = e.mem.u32(spell + 0x18);
        assert!(table != 0);
        assert_eq!(
            calls_to(&e, ACTOR_ADD_SPELL),
            vec![vec![player.addr(), spell], vec![player.addr(), spell]]
        );
    }

    #[test]
    fn selecting_a_spell_unloads_the_old_one_and_preloads_the_new_one() {
        let mut e = engine();
        let player = new_player(&mut e);
        start_log(&mut e);
        player_character_set_selected_spell(&mut e, player, 0x5000);
        assert_eq!(
            e.get(player, PlayerCharacter::pSelectedSpell).addr(),
            0x5000
        );
        assert!(calls_to(&e, MAGIC_ITEM_UNLOAD).is_empty());
        assert_eq!(calls_to(&e, MAGIC_ITEM_PRELOAD), vec![vec![0x5000, 0]]);
        assert_eq!(calls_to(&e, RESET_MAGIC_CAST_SOUND).len(), 1);
        // The same spell again changes nothing.
        player_character_set_selected_spell(&mut e, player, 0x5000);
        assert_eq!(calls_to(&e, RESET_MAGIC_CAST_SOUND).len(), 1);
        // Another spell unloads the first.
        player_character_set_selected_spell(&mut e, player, 0x6000);
        assert_eq!(calls_to(&e, MAGIC_ITEM_UNLOAD), vec![vec![0x5000, 1]]);
        // No spell: unload only.
        player_character_set_selected_spell(&mut e, player, 0);
        assert_eq!(calls_to(&e, MAGIC_ITEM_UNLOAD).len(), 2);
        assert_eq!(calls_to(&e, MAGIC_ITEM_PRELOAD).len(), 2);
        assert_eq!(calls_to(&e, RESET_MAGIC_CAST_SOUND).len(), 3);
        assert_eq!(fn_0094c8c0(&mut e, player), 0);
    }

    #[test]
    fn removing_the_selected_spell_clears_the_selection() {
        let mut e = engine();
        let player = new_player(&mut e);
        let spell = 0x7000;
        e.set(
            player,
            PlayerCharacter::pSelectedSpell,
            Ptr::new(spell + 0x18),
        );
        double(&mut e, ACTOR_REMOVE_SPELL, eax(0));
        assert!(!fn_0094c7a0(&mut e, player, Ptr::new(spell)));
        assert_eq!(fn_0094c8c0(&mut e, player), spell + 0x18);
        // Removed, but another spell is selected.
        double(&mut e, ACTOR_REMOVE_SPELL, eax(1));
        assert!(fn_0094c7a0(&mut e, player, Ptr::new(0x8000)));
        assert_eq!(fn_0094c8c0(&mut e, player), spell + 0x18);
        // Removed and selected.
        assert!(fn_0094c7a0(&mut e, player, Ptr::new(spell)));
        assert_eq!(fn_0094c8c0(&mut e, player), 0);
        // No spell and nothing selected: the null interface matches.
        assert!(fn_0094c7a0(&mut e, player, Ptr::NULL));
    }

    #[test]
    fn selecting_a_scroll_selects_its_spell() {
        let mut e = engine();
        list_helpers(&mut e);
        let player = new_player(&mut e);
        // A scroll whose list entry at +0x74 has a next word of 0x9000.
        let scroll = e.mem.alloc(0x100);
        e.mem.set_u32(scroll + 0x78, 0x9000);
        start_log(&mut e);
        fn_0094c950(&mut e, player, Ptr::new(scroll));
        assert_eq!(
            e.get(player, PlayerCharacter::pSelectedScroll).addr(),
            scroll
        );
        assert_eq!(fn_0094c8c0(&mut e, player), 0x9000 + 0x18);
        // Selecting it again does nothing.
        let before = calls_to(&e, RESET_MAGIC_CAST_SOUND).len();
        fn_0094c950(&mut e, player, Ptr::new(scroll));
        assert_eq!(calls_to(&e, RESET_MAGIC_CAST_SOUND).len(), before);
        // Clearing it clears the spell too.
        fn_0094c950(&mut e, player, Ptr::NULL);
        assert_eq!(e.get(player, PlayerCharacter::pSelectedScroll).addr(), 0);
        assert_eq!(fn_0094c8c0(&mut e, player), 0);
    }

    #[test]
    fn a_scroll_without_a_spell_clears_the_scroll_only() {
        let mut e = engine();
        list_helpers(&mut e);
        let player = new_player(&mut e);
        let scroll = e.mem.alloc(0x100);
        e.set(player, PlayerCharacter::pSelectedScroll, Ptr::new(0x4444));
        e.set(player, PlayerCharacter::pSelectedSpell, Ptr::new(0x5555));
        fn_0094c950(&mut e, player, Ptr::new(scroll));
        assert_eq!(e.get(player, PlayerCharacter::pSelectedScroll).addr(), 0);
        assert_eq!(fn_0094c8c0(&mut e, player), 0x5555);
    }

    #[test]
    fn the_spell_check_reports_its_code_and_honours_god_mode() {
        let mut e = engine();
        let player = new_player(&mut e);
        let out = e.mem.alloc(4);
        e.register_double(SPELL_CHECK, |e, a| {
            e.mem.set_u32(a[3], 5);
            eax(0)
        });
        // Code 5 and failure: false even in god mode.
        double(&mut e, IS_GOD_MODE, eax(1));
        assert!(!fn_0094c800(&mut e, player.cast(), 1, 2, Ptr::new(out), 3));
        assert_eq!(e.mem.u32(out), 5);
        // Another failure code: god mode wins.
        e.register_double(SPELL_CHECK, |e, a| {
            e.mem.set_u32(a[3], 2);
            eax(0)
        });
        assert!(fn_0094c800(&mut e, player.cast(), 1, 2, Ptr::NULL, 3));
        // Without god mode the call's own answer is returned.
        double(&mut e, IS_GOD_MODE, eax(0));
        assert!(!fn_0094c800(&mut e, player.cast(), 1, 2, Ptr::NULL, 3));
        e.register_double(SPELL_CHECK, |_, _| eax(1));
        assert!(fn_0094c800(&mut e, player.cast(), 1, 2, Ptr::NULL, 3));
    }

    #[test]
    fn magic_forwarders_pass_everything_on() {
        let mut e = engine();
        let player = new_player(&mut e);
        start_log(&mut e);
        fn_0094c870(&mut e, player.cast(), 11, 1);
        fn_0094c890(&mut e, player.cast(), 12, 13, 1);
        fn_0094ca20(&mut e, player.cast(), 14, 15, 16, 1);
        let me = player.addr();
        assert_eq!(
            calls_to(&e, MAGIC_CASTER_CAST_ABILITY),
            vec![vec![me, 11, 1]]
        );
        assert_eq!(
            calls_to(&e, MAGIC_CASTER_TRANSFER_DISEASE),
            vec![vec![me, 12, 13, 1]]
        );
        assert_eq!(
            calls_to(&e, MAGIC_TARGET_ADD_TARGET),
            vec![vec![me, 14, 15, 16, 1]]
        );
    }

    #[test]
    fn fn_0094ca20_answers_add_target_al() {
        let mut e = engine();
        let player = new_player(&mut e);
        double(&mut e, MAGIC_TARGET_ADD_TARGET, eax(0x0000_0301));
        assert!(fn_0094ca20(&mut e, player.cast(), 14, 15, 16, 1));
        double(&mut e, MAGIC_TARGET_ADD_TARGET, eax(0x0000_0300));
        assert!(!fn_0094ca20(&mut e, player.cast(), 14, 15, 16, 1));
    }

    #[test]
    fn the_size_lookup_falls_back_when_nothing_is_found() {
        let mut e = engine();
        let player = new_player(&mut e);
        e.set_global(PLAYER_POINTER, 0x1111u32);
        slot(&mut e, player.addr(), 0x380, st0(2.0));
        double(&mut e, REFERENCE_GET_SCALE, st0(1.5));
        let found = e.mem.alloc(0x10);
        let found_slot = slot(&mut e, found, 0x19c, eax(77));
        let _ = found_slot;
        double(&mut e, PLAYER_LOOKUP_BY_SIZE, eax(0));
        double(&mut e, FALLBACK_LOOKUP, eax(0));
        let this = Ptr::<()>::new(player.addr() + 0x88);
        start_log(&mut e);
        assert_eq!(fn_0094ca50(&mut e, this), 0);
        assert_eq!(
            calls_to(&e, PLAYER_LOOKUP_BY_SIZE),
            vec![vec![0x1111, 3.0f32.to_bits()]]
        );
        // The fallback finds the object.
        double(&mut e, FALLBACK_LOOKUP, eax(found));
        assert_eq!(fn_0094ca50(&mut e, this), 77);
        // The first lookup finds it.
        double(&mut e, PLAYER_LOOKUP_BY_SIZE, eax(found));
        double(&mut e, FALLBACK_LOOKUP, eax(0));
        assert_eq!(fn_0094ca50(&mut e, this), 77);
    }

    // ----- the parent-cell change handler -----

    const PARENT_SLOT: u32 = 0x0600_0000;

    fn quiet_slots(e: &mut Engine, object: u32, offsets: &[u32]) {
        for offset in offsets {
            slot(e, object, *offset, Ret::default());
        }
    }

    /// A player in a world where the parent cell is `old`, with the pointer
    /// getter, the camera body and the virtual slots the handler uses.
    fn cell_environment(e: &mut Engine, old: u32) -> (Ptr<PlayerCharacter>, u32) {
        list_helpers(e);
        let player = new_player(e);
        quiet_slots(e, player.addr(), &[0x1fc, 0x22c, 0x2e8, 0x230, 0x494]);
        let body = e.mem.alloc(0x40);
        quiet_slots(e, body, &[0x9c, 0xe4]);
        e.mem.set_u32(player.addr() + 0xdec, body);
        e.mem.set_u32(PARENT_SLOT, old);
        e.register(PARENT_CELL_OF, |e, _| eax(e.mem.u32(PARENT_SLOT)));
        e.register(0x0093_0700, |e, a| {
            e.mem.set_u32(PARENT_SLOT, a[1]);
            Ret::default()
        });
        e.register(0x0055_9450, |e, a| eax(e.mem.u32(a[0])));
        e.set_global(CELL_SIZE, 4096.0f64);
        e.set_global(BORDER_DISTANCE_LIMIT, 2048.0f64);
        e.set_global(NO_WATER_HEIGHT, -3.4e38f32);
        // The border setting reads a zero byte.
        e.register(0x0040_8d60, |_, _| eax(0x0600_0100));
        (player, body)
    }

    #[test]
    fn entering_the_current_cell_does_nothing() {
        let mut e = engine();
        let (player, _) = cell_environment(&mut e, 0x5000);
        start_log(&mut e);
        fn_0094cae0(&mut e, player, Ptr::new(0x5000));
        let log = e.call_log.clone().unwrap();
        let addresses: Vec<u32> = log.iter().map(|(address, _)| *address).collect();
        assert_eq!(addresses, vec![PARENT_CELL_OF]);
    }

    #[test]
    fn leaving_every_cell_clears_the_region_sounds() {
        let mut e = engine();
        let (player, _) = cell_environment(&mut e, 0x5000);
        start_log(&mut e);
        fn_0094cae0(&mut e, player, Ptr::NULL);
        // The water height falls back to the "no water" constant.
        assert_eq!(
            calls_to(&e, 0x0062_f8f0),
            vec![vec![(-3.4e38f32).to_bits()]]
        );
        assert_eq!(calls_to(&e, 0x0042_1e40).len(), 1);
        assert_eq!(calls_to(&e, 0x0093_0700), vec![vec![player.addr(), 0]]);
        assert_eq!(calls_to(&e, LIST_CLEAR), vec![vec![player.addr() + 0x774]]);
        assert_eq!(calls_to(&e, 0x0082_d7c0).len(), 1);
        assert_eq!(e.mem.u32(PARENT_SLOT), 0);
        // Nothing else of the cell handling ran.
        assert!(calls_to(&e, 0x0054_74b0).is_empty());
    }

    #[test]
    fn the_water_height_of_the_new_cell_is_used_when_it_has_water() {
        let mut e = engine();
        let (player, _) = cell_environment(&mut e, 0x1111);
        double(&mut e, 0x0045_18e0, eax(1));
        double(&mut e, 0x0054_71e0, st0(12.5));
        start_log(&mut e);
        fn_0094cae0(&mut e, player, Ptr::new(0x5000));
        assert_eq!(calls_to(&e, 0x0062_f8f0), vec![vec![12.5f32.to_bits()]]);
        // The flag byte of 0094dba0 suppresses it.
        let mut e = engine();
        let (player, _) = cell_environment(&mut e, 0x1111);
        e.set_global(SKIP_WATER_HEIGHT_FLAG, 1u8);
        start_log(&mut e);
        fn_0094cae0(&mut e, player, Ptr::new(0x5000));
        assert!(calls_to(&e, 0x0062_f8f0).is_empty());
    }

    #[test]
    fn a_new_cell_updates_the_music_the_interior_and_the_corners() {
        let mut e = engine();
        let (player, _) = cell_environment(&mut e, 0x1111);
        double(&mut e, 0x0054_74b0, eax(0xaaaa));
        double(&mut e, 0x0045_43c0, eax(0x6000));
        double(&mut e, 0x0045_cd60, eax(0x6100));
        double(&mut e, 0x0054_4c30, eax(2));
        double(&mut e, 0x0054_4c60, eax(3));
        double(&mut e, 0x0054_ddd0, eax(0x7777));
        double(&mut e, 0x0054_75b0, eax(0x6200));
        double(&mut e, 0x0046_dd00, eax(0x6300));
        start_log(&mut e);
        fn_0094cae0(&mut e, player, Ptr::new(0x5000));
        assert_eq!(e.mem.u32(player.addr() + 0x7b4), 0xaaaa);
        assert_eq!(calls_to(&e, 0x0093_a7a0), vec![vec![player.addr(), 0]]);
        assert_eq!(calls_to(&e, 0x0048_93c0), vec![vec![0x6100, 1]]);
        assert_eq!(calls_to(&e, 0x0063_c8f0), vec![vec![0x6300, 0x6200, 0]]);
        let corners: Vec<(u32, u32)> = calls_to(&e, 0x004f_7070)
            .iter()
            .map(|words| (words[1], words[2]))
            .collect();
        let bits = |x: f32, y: f32| (x.to_bits(), y.to_bits());
        assert_eq!(
            corners,
            vec![
                bits(8192.0, 12288.0),
                bits(8192.0, 16384.0),
                bits(12288.0, 12288.0),
                bits(12288.0, 16384.0)
            ]
        );
        assert_eq!(
            calls_to(&e, 0x0070_5420),
            vec![vec![0, 0, 0, 1]],
            "no occupied region: the music pair is built from nothing"
        );
        assert_eq!(calls_to(&e, 0x0094_7d80), vec![vec![player.addr()]]);
        assert_eq!(calls_to(&e, 0x0094_80c0), vec![vec![player.addr()]]);
    }

    #[test]
    fn a_cell_region_that_holds_the_player_becomes_the_current_region() {
        let mut e = engine();
        let (player, _) = cell_environment(&mut e, 0x1111);
        // The cell has one region entry: region REGION with one shape ENTRY.
        let region = e.mem.alloc(0x40);
        let entry = 0x4242;
        let entries = node(&mut e, entry, 0);
        let region_list = e.mem.alloc(0x10);
        e.mem.set_u32(region_list + 4, region);
        e.register_double(0x0054_7110, move |_, _| eax(region_list));
        double(&mut e, 0x0044_1110, eax(entries));
        double(&mut e, 0x004f_8360, eax(1));
        // Region data: kind 7 is the sound data.
        let sound_data = e.mem.alloc(0x40);
        let sound = node(&mut e, 0x9999, 0);
        slot(&mut e, sound_data, 0x30, eax(1));
        e.register(0x0096_11e0, |_, a| eax(a[0]));
        e.register_double(0x004f_35b0, move |_, a| {
            eax(if a[1] == 7 { sound_data } else { 0 })
        });
        double(&mut e, 0x004f_1540, eax(1));
        double(&mut e, 0x0048_d150, eax(sound));
        double(&mut e, 0x0054_9580, eax(1));
        e.register(OPERATOR_NEW, |e, a| eax(e.mem.alloc(a[0])));
        e.register(0x0096_a640, |_, a| eax(a[0]));
        start_log(&mut e);
        fn_0094cae0(&mut e, player, Ptr::new(0x5000));

        // The region sounds were collected and the region remembered.
        assert_eq!(e.global::<u32>(CURRENT_REGION_WITH_SOUNDS), sound_data);
        assert_eq!(
            calls_to(&e, 0x005a_e3d0),
            vec![vec![player.addr() + 0x774, sound]]
        );
        // The best region replaced the (empty) list once, at the start and
        // for the replacement.
        let clears: Vec<u32> = calls_to(&e, LIST_CLEAR).iter().map(|w| w[0]).collect();
        assert_eq!(
            clears
                .iter()
                .filter(|a| **a == player.addr() + 0x774)
                .count(),
            2
        );
        assert!(clears.contains(&(player.addr() + 0x768)));
        // The player is inside all four corners: the cell is a border cell.
        assert!(e.get(player, PlayerCharacter::bInBorderContainedCell));
        // No occupied region yet: the region becomes the occupied one.
        assert!(calls_to(&e, 0x0093_a7a0).contains(&vec![player.addr(), region]));
        // The border region array was created and the region added to it.
        let array = e.mem.u32(player.addr() + 0x7b0);
        assert!(array != 0);
        let added = calls_to(&e, 0x0096_a610);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], array);
        assert_eq!(e.mem.u32(added[0][1]), region);
        // The entries of the occupied region list were gathered with the
        // player point.
        assert_eq!(
            calls_to(&e, 0x004f_6600),
            vec![vec![player.addr() + 0x764, region]]
        );
    }

    #[test]
    fn regions_that_do_not_apply_are_skipped() {
        let mut e = engine();
        let (player, _) = cell_environment(&mut e, 0x1111);
        let region = e.mem.alloc(0x40);
        let region_list = e.mem.alloc(0x10);
        e.mem.set_u32(region_list + 4, region);
        e.register_double(0x0054_7110, move |_, _| eax(region_list));
        // Hidden regions are skipped before anything else.
        double(&mut e, 0x0044_0d80, eax(1));
        start_log(&mut e);
        fn_0094cae0(&mut e, player, Ptr::new(0x5000));
        assert!(calls_to(&e, 0x0044_1110).is_empty());
        // A region of another world space is skipped.
        double(&mut e, 0x0044_0d80, eax(0));
        double(&mut e, 0x007a_f430, eax(0x1000));
        double(&mut e, 0x0057_5d70, eax(0x2000));
        start_log(&mut e);
        fn_0094cae0(&mut e, player, Ptr::new(0x5001));
        assert!(calls_to(&e, 0x0044_1110).is_empty());
        // A region without entries is skipped.
        double(&mut e, 0x0057_5d70, eax(0x1000));
        double(&mut e, 0x0044_1110, eax(0));
        start_log(&mut e);
        fn_0094cae0(&mut e, player, Ptr::new(0x5002));
        assert!(calls_to(&e, 0x004f_8360).is_empty());
        // So is a region whose entry list is empty.
        let empty = e.mem.alloc(8);
        double(&mut e, 0x0044_1110, eax(empty));
        start_log(&mut e);
        fn_0094cae0(&mut e, player, Ptr::new(0x5003));
        assert!(calls_to(&e, 0x004f_8360).is_empty());
        assert_eq!(e.mem.u32(player.addr() + 0x7b0), 0);
    }

    #[test]
    fn an_occupied_region_is_replaced_by_a_better_one() {
        let mut e = engine();
        let (player, _) = cell_environment(&mut e, 0x1111);
        let region = e.mem.alloc(0x40);
        let entries = node(&mut e, 0x4242, 0);
        let region_list = e.mem.alloc(0x10);
        e.mem.set_u32(region_list + 4, region);
        e.register_double(0x0054_7110, move |_, _| eax(region_list));
        double(&mut e, 0x0044_1110, eax(entries));
        double(&mut e, 0x004f_8360, eax(1));
        e.register(0x0096_11e0, |_, a| eax(a[0]));
        // The occupied region and the new one have data of kind 3.
        let occupied = e.mem.alloc(0x40);
        e.mem.set_u32(player.addr() + 0x760, occupied);
        let old_data = 0x7100;
        let new_data = 0x7200;
        e.register_double(0x004f_35b0, move |_, a| {
            eax(match (a[0], a[1]) {
                (list, 3) if list == occupied => old_data,
                (_, 3) => new_data,
                _ => 0,
            })
        });
        // Same flag, higher priority for the new data.
        double(&mut e, 0x004f_1540, eax(0));
        e.register(0x005b_b4d0, |_, a| eax(if a[0] == 0x7200 { 9 } else { 4 }));
        start_log(&mut e);
        fn_0094cae0(&mut e, player, Ptr::new(0x5000));
        assert!(calls_to(&e, 0x0093_a7a0).contains(&vec![player.addr(), region]));
        // A lower priority keeps the occupied region.
        e.register(0x005b_b4d0, |_, a| eax(if a[0] == 0x7200 { 2 } else { 4 }));
        e.mem.set_u32(PARENT_SLOT, 0x1111);
        start_log(&mut e);
        fn_0094cae0(&mut e, player, Ptr::new(0x5000));
        assert!(!calls_to(&e, 0x0093_a7a0).contains(&vec![player.addr(), region]));
    }

    fn border_environment(e: &mut Engine) -> (Ptr<PlayerCharacter>, u32) {
        let (player, _) = cell_environment(e, 0x1111);
        let setting = e.mem.alloc(4);
        e.mem.set_u8(setting, 1);
        e.register_double(0x0040_8d60, move |_, _| eax(setting));
        let world = 0x7777;
        double(e, 0x0054_ddd0, eax(world));
        double(e, 0x0058_6260, eax(1));
        quiet_slots(e, player.addr(), &[0x1f4]);
        let position = e.mem.alloc(12);
        slot(e, player.addr(), 0x1f4, eax(position));
        (player, world)
    }

    #[test]
    fn leaving_the_remembered_cell_marks_a_return_to_the_last_good_position() {
        let mut e = engine();
        let (player, _) = border_environment(&mut e);
        // The last known good location is an exterior cell (type 0x39) other
        // than the new one.
        e.mem.set_u32(player.addr() + 0x7ac, 0x3333);
        double(&mut e, FORM_TYPE_OF, eax(0x39));
        fn_0094cae0(&mut e, player, Ptr::new(0x5000));
        assert!(e.get(player, PlayerCharacter::bReturnToLastKnownGoodPosition));
    }

    #[test]
    fn another_world_space_marks_a_return_to_the_last_good_position() {
        let mut e = engine();
        let (player, _) = border_environment(&mut e);
        e.mem.set_u32(player.addr() + 0x7ac, 0x3333);
        double(&mut e, FORM_TYPE_OF, eax(0x41));
        fn_0094cae0(&mut e, player, Ptr::new(0x5000));
        assert!(e.get(player, PlayerCharacter::bReturnToLastKnownGoodPosition));
    }

    #[test]
    fn far_from_the_border_in_the_same_world_space_marks_it_too() {
        let mut e = engine();
        let (player, world) = border_environment(&mut e);
        e.mem.set_u32(player.addr() + 0x7ac, world);
        double(&mut e, FORM_TYPE_OF, eax(0x41));
        double(&mut e, 0x0045_7990, st0(3000.0));
        fn_0094cae0(&mut e, player, Ptr::new(0x5000));
        assert!(e.get(player, PlayerCharacter::bReturnToLastKnownGoodPosition));
        // Within the limit it stays clear.
        let mut e = engine();
        let (player, world) = border_environment(&mut e);
        e.mem.set_u32(player.addr() + 0x7ac, world);
        double(&mut e, FORM_TYPE_OF, eax(0x41));
        double(&mut e, 0x0045_7990, st0(2048.0));
        fn_0094cae0(&mut e, player, Ptr::new(0x5000));
        assert!(!e.get(player, PlayerCharacter::bReturnToLastKnownGoodPosition));
    }

    #[test]
    fn the_border_check_is_skipped_without_the_setting() {
        let mut e = engine();
        let (player, _) = border_environment(&mut e);
        let off = e.mem.alloc(4);
        e.register_double(0x0040_8d60, move |_, _| eax(off));
        e.mem.set_u32(player.addr() + 0x7ac, 0x3333);
        double(&mut e, FORM_TYPE_OF, eax(0x39));
        fn_0094cae0(&mut e, player, Ptr::new(0x5000));
        assert!(!e.get(player, PlayerCharacter::bReturnToLastKnownGoodPosition));
    }

    #[test]
    fn the_camera_body_is_built_when_it_is_missing() {
        let mut e = engine();
        let (player, body) = cell_environment(&mut e, 0x1111);
        e.register(OPERATOR_NEW, |e, a| eax(e.mem.alloc(a[0])));
        e.register(0x00aa_13e0, |e, a| eax(e.mem.alloc(a[0])));
        e.register(SMART_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        double(&mut e, 0x0082_2510, eax(1));
        let prototype = e.mem.alloc(0x10);
        quiet_slots(&mut e, prototype, &[0x9c, 0xe4]);
        e.register_double(0x0056_d380, move |e, a| {
            let table = e.mem.u32(prototype);
            e.mem.set_u32(a[0], table);
            eax(a[0])
        });
        let motion = e.mem.alloc(0x40);
        double(&mut e, 0x004a_e700, eax(motion));
        let position = e.mem.alloc(12);
        e.mem.set_u32(position, 1);
        e.mem.set_u32(position + 4, 2);
        e.mem.set_u32(position + 8, 3);
        slot(&mut e, player.addr(), 0x1f4, eax(position));
        e.set_global(CAMERA_SPHERE_RADIUS, 1.25f32);
        let counter = e.global::<u32>(0x0126_8280);
        start_log(&mut e);
        fn_0094cae0(&mut e, player, Ptr::new(0x5000));
        // The sphere shape: allocated with 0x14, built by the constructor.
        let sizes: Vec<u32> = calls_to(&e, 0x00aa_13e0).iter().map(|w| w[0]).collect();
        assert_eq!(sizes, vec![0x14, 0x1c]);
        let sphere = calls_to(&e, SPHERE_BASE_CONSTRUCTOR)[0][0];
        assert_eq!(e.mem.u32(sphere), 0x0103_0cdc);
        assert_eq!(e.global::<u32>(0x0126_8280), counter + 1);
        assert_eq!(
            calls_to(&e, 0x0056_ea00),
            vec![vec![sphere, 1.25f32.to_bits()]]
        );
        // The rigid body info got the shape and the flags 0x30021.
        let cinfo = calls_to(&e, 0x00c8_f510)[0][0];
        assert_eq!(calls_to(&e, 0x0056_f110), vec![vec![cinfo, sphere]]);
        assert_eq!(calls_to(&e, 0x0056_f0f0), vec![vec![cinfo, 0x30021]]);
        // The body was built from it and stored; its motion byte is 9.
        let built = calls_to(&e, 0x0056_d380)[0][0];
        assert_eq!(e.mem.u32(player.addr() + 0xdec), built);
        assert_ne!(built, body);
        assert_eq!(e.mem.u8(motion + 0x1a), 9);
        // The body is placed at the player's position.
        let placed = calls_to(&e, 0x0056_10f0);
        assert_eq!(placed[0][0], built);
        assert_eq!(
            [
                e.mem.u32(placed[0][1]),
                e.mem.u32(placed[0][1] + 4),
                e.mem.u32(placed[0][1] + 8)
            ],
            [1, 2, 3]
        );
        assert_eq!(calls_to(&e, 0x0056_d730), vec![vec![cinfo]]);
    }

    // ----- small accessors and setters -----

    #[test]
    fn small_setters_and_getters() {
        let mut e = engine();
        let object = e.mem.alloc(0x40);
        fn_0094db80(&mut e, Ptr::new(object), 7);
        assert_eq!(e.mem.u8(object + 0x1a), 7);

        e.set_global(SKIP_WATER_HEIGHT_FLAG, 3u8);
        assert_eq!(fn_0094dba0(&mut e), 3);

        let player = new_player(&mut e);
        fn_00950010(&mut e, player, 1);
        assert_eq!(e.get(player, PlayerCharacter::bChargen), 1);

        e.set_global(VANITY_ACTIVE, 1u8);
        assert_eq!(fn_00950090(&mut e, player.cast()), 1);

        e.set_global(NODE_NAME_EAC0, 11u32);
        e.set_global(NODE_NAME_EAD0, 12u32);
        e.set_global(NODE_NAME_EAE0, 13u32);
        e.set_global(NODE_NAME_EB30, 14u32);
        assert_eq!(fn_0094eac0(&mut e), 11);
        assert_eq!(fn_0094ead0(&mut e), 12);
        assert_eq!(fn_0094eae0(&mut e), 13);
        assert_eq!(fn_0094eb30(&mut e), 14);
    }

    #[test]
    fn the_last_known_good_position_is_copied_out() {
        let mut e = engine();
        let player = new_player(&mut e);
        e.mem.set_f32(player.addr() + 0x7a0, 1.0);
        e.mem.set_f32(player.addr() + 0x7a4, 2.0);
        e.mem.set_f32(player.addr() + 0x7a8, 3.0);
        let out = e.mem.alloc(12);
        let result = fn_0094df30(&mut e, player, Ptr::new(out));
        assert_eq!(result.addr(), out);
        assert_eq!(
            [e.mem.f32(out), e.mem.f32(out + 4), e.mem.f32(out + 8)],
            [1.0, 2.0, 3.0]
        );
    }

    #[test]
    fn sleeping_or_resting_means_a_positive_sleep_time() {
        let mut e = engine();
        let player = new_player(&mut e);
        assert!(!player_character_is_sleepingor_resting(&mut e, player));
        e.set(player, PlayerCharacter::iSleepTime, 3);
        assert!(player_character_is_sleepingor_resting(&mut e, player));
        e.set(player, PlayerCharacter::iSleepTime, -2);
        assert!(!player_character_is_sleepingor_resting(&mut e, player));
    }

    #[test]
    fn the_camera_node_lookup_stores_the_found_node() {
        let mut e = engine();
        let object = e.mem.alloc(0x40);
        let seen = recording_slot(&mut e, object, 0x9c, eax(0x5151));
        e.set_global(NODE_NAME_EB30, 0x66u32);
        start_log(&mut e);
        fn_0094eaf0(&mut e, Ptr::NULL, Ptr::new(object), false);
        assert_eq!(e.global::<u32>(NODE_CAMERA_3RD_SLOT), 0x5151);
        assert_eq!(seen.borrow().clone(), vec![vec![object, 0x66]]);
        assert!(calls_to(&e, 0x0045_0f90).is_empty());
        // With the flag, the object is first told (with 1).
        fn_0094eaf0(&mut e, Ptr::NULL, Ptr::new(object), true);
        assert_eq!(calls_to(&e, 0x0045_0f90), vec![vec![object, 1]]);
    }

    #[test]
    fn forwarders_to_the_progression_object() {
        let mut e = engine();
        let player = new_player(&mut e);
        start_log(&mut e);
        fn_0094fce0(&mut e, player, 5, 6);
        fn_0094fd10(&mut e, player, 9);
        assert_eq!(
            calls_to(&e, PROCESS_LISTS_PRINT_LISTS),
            vec![vec![player.addr() + 0x878, 5, 6]]
        );
        assert_eq!(
            calls_to(&e, CHARACTER_PROGRESSION_REWARD_EXPERIENCE),
            vec![vec![player.addr() + 0x878, 9]]
        );
    }

    #[test]
    fn updating_the_player_3d_notifies_the_menu_only_when_asked() {
        let mut e = engine();
        let player = new_player(&mut e);
        let process = e.mem.alloc(0x40);
        e.set(player, PlayerCharacter::pCurrentProcess, Ptr::new(process));
        let busy = recording_slot(&mut e, process, 0x474, eax(1));
        let update = recording_slot(&mut e, process, 0x464, Ret::default());
        double(&mut e, INVENTORY_MENU_VISIBLE, eax(1));
        start_log(&mut e);
        // The process says yes: nothing to release.
        player_character_update_player_3d(&mut e, player);
        assert!(calls_to(&e, RELEASE_HANDLE).is_empty());
        assert_eq!(busy.borrow().clone(), vec![vec![process, 1]]);
        assert_eq!(update.borrow().clone(), vec![vec![process, player.addr()]]);
        // The process says no and the inventory is visible.
        let table = e.mem.u32(process);
        let no = NEXT_TARGET.fetch_add(4, Ordering::Relaxed);
        e.mem.set_u32(table + 0x474, no);
        double(&mut e, no, eax(0));
        player_character_update_player_3d(&mut e, player);
        assert_eq!(calls_to(&e, RELEASE_HANDLE), vec![vec![player.addr()]]);
        // ... and the inventory is not.
        double(&mut e, INVENTORY_MENU_VISIBLE, eax(0));
        player_character_update_player_3d(&mut e, player);
        assert_eq!(calls_to(&e, RELEASE_HANDLE).len(), 1);
        assert_eq!(update.borrow().len(), 3);
    }

    #[test]
    fn stopping_vanity_mode_restores_and_clears() {
        let mut e = engine();
        e.set_global(VANITY_ACTIVE, 1u8);
        e.set_global(VANITY_SAVED_FLAG, 1u8);
        e.set_global(VANITY_SAVED_VALUE, 42.5f32);
        e.set_global(VANITY_RESTORED_VALUE, 1.0f32);
        for address in [
            VANITY_VALUE_07C4,
            VANITY_VALUE_07C8,
            VANITY_VALUE_07DC,
            VANITY_VALUE_0B58,
            VANITY_VALUE_0B60,
        ] {
            e.set_global(address, 9.0f32);
        }
        e.set_global(VANITY_FLAG_07C1, 1u8);
        start_log(&mut e);
        player_character_stop_vanity_mode(&mut e, Ptr::NULL);
        assert_eq!(e.global::<f32>(VANITY_RESTORED_VALUE), 42.5);
        assert_eq!(e.global::<u8>(VANITY_ACTIVE), 0);
        assert_eq!(e.global::<u8>(VANITY_SAVED_FLAG), 0);
        assert_eq!(e.global::<u8>(VANITY_FLAG_07C3), 1);
        assert_eq!(e.global::<u8>(VANITY_FLAG_07C1), 0);
        for address in [
            VANITY_VALUE_07C4,
            VANITY_VALUE_07C8,
            VANITY_VALUE_07DC,
            VANITY_VALUE_0B58,
            VANITY_VALUE_0B60,
        ] {
            assert_eq!(e.global::<f32>(address), 0.0);
        }
        assert_eq!(calls_to(&e, SET_MENU_MODE), vec![vec![1]]);
        // Without the saved flag the restored value is left alone.
        e.set_global(VANITY_RESTORED_VALUE, 5.0f32);
        e.set_global(VANITY_SAVED_VALUE, 7.0f32);
        player_character_stop_vanity_mode(&mut e, Ptr::NULL);
        assert_eq!(e.global::<f32>(VANITY_RESTORED_VALUE), 5.0);
    }

    #[test]
    fn the_sphere_shape_constructor_installs_its_vtable_and_counts() {
        let mut e = engine();
        let object = e.mem.alloc(0x14);
        e.set_global(0x0126_8280u32, 4u32);
        start_log(&mut e);
        let result = fn_0094dbb0(&mut e, Ptr::new(object));
        assert_eq!(result.addr(), object);
        assert_eq!(e.mem.u32(object), 0x0103_0cdc);
        assert_eq!(e.global::<u32>(0x0126_8280), 5);
        assert_eq!(calls_to(&e, SPHERE_BASE_CONSTRUCTOR), vec![vec![object]]);
    }

    // ----- ReturnToLastKnownGoodPosition -----

    #[test]
    fn nothing_happens_without_a_last_known_good_location() {
        let mut e = engine();
        let player = new_player(&mut e);
        start_log(&mut e);
        player_character_return_to_last_known_good_position(&mut e, player, true);
        player_character_return_to_last_known_good_position(&mut e, player, false);
        assert!(calls_to(&e, 0x0070_52f0).is_empty());
    }

    #[test]
    fn returning_with_the_position_functions_picks_the_world_space_function() {
        let mut e = engine();
        let player = new_player(&mut e);
        e.set_global(MESSAGE_SECONDS, 2.0f32);
        e.mem.set_f32(player.addr() + 0x7a0, 10.0);
        e.mem.set_f32(player.addr() + 0x7a4, 20.0);
        e.mem.set_f32(player.addr() + 0x7a8, 30.0);
        e.mem.set_u32(player.addr() + 0x7ac, 0x6000);
        e.register(0x0040_3df0, |_, a| eax(a[0] + 4));
        let wanted = e.mem.alloc(12);
        e.mem.set_f32(wanted, 1.0);
        e.mem.set_f32(wanted + 4, 2.0);
        e.mem.set_f32(wanted + 8, 3.0);
        double(&mut e, 0x0043_0830, eax(wanted));
        // A cell (type 0x39) uses PositionPlayer.
        double(&mut e, FORM_TYPE_OF, eax(0x39));
        start_log(&mut e);
        player_character_return_to_last_known_good_position(&mut e, player, true);
        let known = [10.0f32.to_bits(), 20.0f32.to_bits(), 30.0f32.to_bits()];
        let wanted_bits = [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()];
        let mut expected = vec![player.addr()];
        expected.extend(known);
        expected.extend(wanted_bits);
        expected.extend([0x6000, 0]);
        assert_eq!(calls_to(&e, 0x0093_c200), vec![expected.clone()]);
        assert!(calls_to(&e, 0x0093_cce0).is_empty());
        // The message: text, 0, icon, 0, 2.0 seconds, 0.
        assert_eq!(
            calls_to(&e, 0x0070_52f0),
            vec![vec![
                RETURN_MESSAGE_SETTING + 4,
                0,
                SAD_ICON_PATH,
                0,
                2.0f32.to_bits(),
                0
            ]]
        );
        // A world space (type 0x41) uses PositionPlayerExterior.
        double(&mut e, FORM_TYPE_OF, eax(0x41));
        player_character_return_to_last_known_good_position(&mut e, player, true);
        assert_eq!(calls_to(&e, 0x0093_cce0), vec![expected]);
        // Any other form does not move the player.
        double(&mut e, FORM_TYPE_OF, eax(0x10));
        player_character_return_to_last_known_good_position(&mut e, player, true);
        assert_eq!(calls_to(&e, 0x0093_c200).len(), 1);
        assert_eq!(calls_to(&e, 0x0093_cce0).len(), 1);
    }

    #[test]
    fn returning_directly_sets_the_position_and_resets_the_bodies() {
        let mut e = engine();
        let player = new_player(&mut e);
        let cell = 0x6000;
        e.mem.set_u32(player.addr() + 0x7ac, cell);
        e.mem.set_f32(player.addr() + 0x7a0, 8192.0);
        e.mem.set_f32(player.addr() + 0x7a4, 12288.5);
        e.register(0x0040_3df0, |_, a| eax(a[0] + 4));
        double(&mut e, FORM_TYPE_OF, eax(0x39));
        let controller = 0x9100;
        double(&mut e, 0x0093_06d0, eax(controller));
        let ground = e.mem.alloc(16);
        e.mem.set_f32(ground + 8, 55.0);
        double(&mut e, 0x0043_6aa0, eax(ground));
        let move_calls = recording_slot(&mut e, player.addr(), 0x228, Ret::default());
        let bodies = [0xb100, 0xb200];
        e.register_double(0x0095_0bb0, move |_, a| eax(bodies[a[1] as usize]));
        start_log(&mut e);
        player_character_return_to_last_known_good_position(&mut e, player, false);
        // No water: the height comes from the reference position.
        assert_eq!(e.mem.f32(player.addr() + 0x7a8), 55.0);
        let position = player.addr() + 0x7a0;
        assert_eq!(
            calls_to(&e, 0x0057_5830),
            vec![vec![player.addr(), position]]
        );
        assert_eq!(calls_to(&e, 0x0056_20e0), vec![vec![controller, position]]);
        // The location is a cell: the player is moved into it, twice.
        assert_eq!(
            move_calls.borrow().clone(),
            vec![vec![player.addr(), cell]; 2]
        );
        for body in bodies {
            assert_eq!(
                calls_to(&e, 0x0044_0460)
                    .iter()
                    .filter(|w| w[0] == body)
                    .count(),
                1
            );
            assert!(calls_to(&e, 0x00c6_bd00).contains(&vec![body, 1]));
            assert_eq!(
                calls_to(&e, 0x00a5_9c60)
                    .iter()
                    .filter(|w| w[0] == body)
                    .count(),
                1
            );
        }
    }

    #[test]
    fn returning_directly_to_a_world_space_finds_the_cell_from_the_position() {
        let mut e = engine();
        let player = new_player(&mut e);
        let world = 0x6000;
        e.mem.set_u32(player.addr() + 0x7ac, world);
        e.mem.set_f32(player.addr() + 0x7a0, 8192.0);
        e.mem.set_f32(player.addr() + 0x7a4, 12288.5);
        e.register(0x0040_3df0, |_, a| eax(a[0] + 4));
        double(&mut e, FORM_TYPE_OF, eax(0x41));
        // Water: the height is left alone.
        double(&mut e, 0x0093_06d0, eax(0x9100));
        double(&mut e, 0x0088_b0c0, eax(1));
        e.register(0x0040_6d90, |_, a| eax(f32::from_bits(a[0]) as i32 as u32));
        e.set_global(0x011c_3f2cu32, 0x3000u32);
        double(&mut e, 0x0046_1c20, eax(0x7000));
        let move_calls = recording_slot(&mut e, player.addr(), 0x228, Ret::default());
        e.mem.set_f32(player.addr() + 0x7a8, 5.0);
        start_log(&mut e);
        player_character_return_to_last_known_good_position(&mut e, player, false);
        assert_eq!(e.mem.f32(player.addr() + 0x7a8), 5.0);
        assert_eq!(
            calls_to(&e, 0x0046_1c20),
            vec![vec![0x3000, 2, 3, world, 0]]
        );
        assert_eq!(
            move_calls.borrow().clone(),
            vec![vec![player.addr(), 0x7000]; 2]
        );
    }

    // ----- the one-hour wait step -----

    /// A player whose actor value owner answers `0xc` with `value` and whose
    /// slot `0x3a4` records its calls.
    fn waiting_player(e: &mut Engine, value: f64) -> (Ptr<PlayerCharacter>, CallRecord) {
        let player = new_player(e);
        slot(e, player.addr() + 0xa4, 0xc, st0(value));
        let changes = recording_slot(e, player.addr(), 0x3a4, Ret::default());
        e.set_global(SECONDS_PER_HOUR, 3600.0f64);
        e.set_global(PLAYER_POINTER, 0x1111u32);
        e.set_global(SKY_HOLDER_POINTER, 0x2222u32);
        e.set_global(WAIT_UPDATE_TIME, 1.5f32);
        double(e, 0x0045_3a70, eax(0x1234));
        double(e, 0x0086_7950, st0(2.0));
        double(e, 0x0096_d490, st0(100.0));
        double(e, 0x0084_d030, st0(0.25));
        double(e, 0x008d_8520, eax(0x4000));
        let limit = e.mem.alloc(4);
        e.mem.set_f32(limit, 50.0);
        e.register_double(SETTING_FLOAT_POINTER, move |_, _| eax(limit));
        (player, changes)
    }

    #[test]
    fn an_hour_passes_on_the_clock_the_calendar_and_the_sky() {
        let mut e = engine();
        let (player, _) = waiting_player(&mut e, 80.0);
        e.set(player, PlayerCharacter::iSleepTime, 3);
        start_log(&mut e);
        fn_0094df80(&mut e, player);
        // One hour at time scale 2 is 1800 seconds.
        assert_eq!(calls_to(&e, 0x00ad_8780), vec![vec![0x1234, 4]]);
        assert_eq!(calls_to(&e, 0x0081_5b00), vec![vec![player.addr() + 0x88]]);
        assert_eq!(
            calls_to(&e, 0x0096_d4b0),
            vec![vec![PROCESS_LISTS, 1900.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, 0x0086_7a40),
            vec![vec![CALENDAR, 1800.0f32.to_bits()]]
        );
        // The wait is not active: no updates of the magic or the lists.
        assert!(calls_to(&e, 0x0088_b510).is_empty());
        assert!(calls_to(&e, 0x0040_fbf0).is_empty());
        // The sky and the lists are updated, the countdown goes on.
        assert_eq!(
            calls_to(&e, 0x0063_ac70),
            vec![vec![0x4000, 0.25f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, 0x0097_23f0), vec![vec![PROCESS_LISTS]]);
        assert_eq!(calls_to(&e, 0x008c_3c40), vec![vec![player.addr(), 1, 0]]);
        assert_eq!(e.get(player, PlayerCharacter::iSleepTime), 2);
        assert_eq!(e.global::<u8>(WAIT_STEP_ACTIVE), 0);
    }

    #[test]
    fn an_active_wait_updates_the_process_lists_under_the_lock() {
        let mut e = engine();
        let (player, changes) = waiting_player(&mut e, 80.0);
        e.set_global(WAIT_STEP_ACTIVE, 1u8);
        e.set_global(PROCESS_LOCK_ENABLED, 1u8);
        e.set(player, PlayerCharacter::iSleepTime, 3);
        start_log(&mut e);
        fn_0094df80(&mut e, player);
        let log: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(address, _)| *address)
            .collect();
        let at = |address: u32| log.iter().position(|a| *a == address).unwrap();
        assert!(at(0x0040_fbf0) < at(0x0096_bcd0));
        assert!(at(0x0096_bcd0) < at(0x0096_b810));
        assert!(at(0x0096_b810) < at(0x0096_b470));
        assert!(at(0x0096_b470) < at(0x0096_b050));
        assert!(at(0x0096_b050) < at(0x0040_fba0));
        assert_eq!(calls_to(&e, 0x0040_fbf0), vec![vec![PROCESS_LOCK, 0]]);
        for address in [0x0096_bcd0, 0x0096_b810, 0x0096_b470, 0x0096_b050] {
            assert_eq!(
                calls_to(&e, address),
                vec![vec![PROCESS_LISTS, 1.5f32.to_bits(), 1]]
            );
        }
        // Health is restored on the player global.
        assert_eq!(
            calls_to(&e, 0x0088_b510),
            vec![vec![0x1111, 1800.0f32.to_bits()]]
        );
        assert!(calls_to(&e, 0x0082_3c40).is_empty());
        assert!(changes.borrow().is_empty());
    }

    #[test]
    fn an_active_wait_without_the_lock_flag_skips_the_list_updates() {
        let mut e = engine();
        let (player, _) = waiting_player(&mut e, 80.0);
        e.set_global(WAIT_STEP_ACTIVE, 1u8);
        e.set(player, PlayerCharacter::iSleepTime, 3);
        start_log(&mut e);
        fn_0094df80(&mut e, player);
        assert!(calls_to(&e, 0x0040_fbf0).is_empty());
        assert!(calls_to(&e, 0x0096_bcd0).is_empty());
        assert_eq!(calls_to(&e, 0x0088_b510).len(), 1);
    }

    #[test]
    fn an_active_wait_with_a_magic_target_updates_it() {
        let mut e = engine();
        let (player, _) = waiting_player(&mut e, 80.0);
        e.set_global(WAIT_STEP_ACTIVE, 1u8);
        double(&mut e, 0x005c_7870, eax(1));
        e.set(player, PlayerCharacter::iSleepTime, 3);
        start_log(&mut e);
        fn_0094df80(&mut e, player);
        assert_eq!(
            calls_to(&e, 0x0082_3c40),
            vec![vec![player.addr() + 0x94, 1800.0f32.to_bits()]]
        );
        assert!(calls_to(&e, 0x0096_bcd0).is_empty());
    }

    #[test]
    fn a_waiting_player_loses_the_smaller_of_the_value_and_the_limit() {
        let mut e = engine();
        let (player, changes) = waiting_player(&mut e, 80.0);
        e.set_global(WAIT_STEP_ACTIVE, 1u8);
        double(&mut e, 0x004d_1360, eax(1));
        double(&mut e, 0x005a_1e50, eax(1));
        e.set(player, PlayerCharacter::iSleepTime, 3);
        fn_0094df80(&mut e, player);
        assert_eq!(
            changes.borrow().clone(),
            vec![vec![player.addr(), 0x4b, (-50.0f32).to_bits(), 0]]
        );
        // A value below the limit is used as it is.
        let (player, changes) = waiting_player(&mut e, 20.0);
        e.set_global(WAIT_STEP_ACTIVE, 1u8);
        e.set(player, PlayerCharacter::iSleepTime, 3);
        fn_0094df80(&mut e, player);
        assert_eq!(
            changes.borrow().clone(),
            vec![vec![player.addr(), 0x4b, (-20.0f32).to_bits(), 0]]
        );
    }

    #[test]
    fn the_last_hour_ends_the_wait_and_restores_a_sleeper() {
        let mut e = engine();
        let (player, _) = waiting_player(&mut e, 80.0);
        e.set(player, PlayerCharacter::iSleepTime, 1);
        e.set(player, PlayerCharacter::bIsSleeping, true);
        start_log(&mut e);
        fn_0094df80(&mut e, player);
        assert_eq!(e.get(player, PlayerCharacter::iSleepTime), 0);
        assert_eq!(e.global::<u8>(WAIT_STEP_ACTIVE), 1);
        assert!(!e.get(player, PlayerCharacter::bIsSleeping));
        assert_eq!(calls_to(&e, 0x008a_0960), vec![vec![player.addr()]]);
        // A player in the state 004d1360 describes is not restored.
        let (player, _) = waiting_player(&mut e, 80.0);
        e.set(player, PlayerCharacter::iSleepTime, 1);
        e.set(player, PlayerCharacter::bIsSleeping, true);
        double(&mut e, 0x004d_1360, eax(1));
        fn_0094df80(&mut e, player);
        assert_eq!(calls_to(&e, 0x008a_0960).len(), 1);
        assert!(!e.get(player, PlayerCharacter::bIsSleeping));
    }

    // ----- karma -----

    fn int_setting_slot(setting: u32) -> u32 {
        0x0600_1000 + (setting & 0xfff) * 4
    }

    fn karma_world(e: &mut Engine, karma: i32) -> (Ptr<PlayerCharacter>, CallRecord) {
        let player = new_player(e);
        slot(e, player.addr() + 0xa4, 0x8, eax(karma as u32));
        slot(e, player.addr() + 0xa4, 0xc, st0(karma as f64));
        let changes = recording_slot(e, player.addr(), 0x3a8, Ret::default());
        e.register(SETTING_INT_POINTER, |_, a| eax(int_setting_slot(a[0])));
        e.register(SETTING_TEXT, |_, a| eax(a[0] + 4));
        e.set_global(MESSAGE_SECONDS, 2.0f32);
        for (setting, value) in [
            (KARMA_LOWER_LIMIT_SETTING, -1000i32),
            (KARMA_UPPER_LIMIT_SETTING, 1000),
            (KARMA_MESSAGE_THRESHOLD_SETTING, 100),
        ] {
            e.mem.set_i32(int_setting_slot(setting), value);
        }
        double(e, 0x0045_3a70, eax(0x1234));
        e.register(0x00ad_7550, |_, a| eax(a[1]));
        (player, changes)
    }

    fn message_of(e: &Engine) -> Vec<u32> {
        let messages = calls_to(e, 0x0070_52f0);
        assert_eq!(messages.len(), 1);
        messages[0].clone()
    }

    #[test]
    fn a_small_karma_loss_shows_the_small_message_and_plays_the_down_sound() {
        let mut e = engine();
        let (player, changes) = karma_world(&mut e, 10);
        start_log(&mut e);
        player_character_reward_karma(&mut e, player, -50);
        assert_eq!(
            message_of(&e),
            vec![
                KARMA_LOSS_TEXT_SETTING + 4,
                0,
                KARMA_LOSS_ICON_SETTING + 4,
                0,
                2.0f32.to_bits(),
                0
            ]
        );
        assert_eq!(
            changes.borrow().clone(),
            vec![vec![player.addr(), 0x17, (-50i32) as u32, 0]]
        );
        let lookups = calls_to(&e, 0x00ad_7550);
        assert_eq!(lookups.len(), 1);
        assert_eq!(lookups[0][0], 0x1234);
        assert_eq!(lookups[0][2], SOUND_KARMA_DOWN);
        assert_eq!(lookups[0][3], 0x121);
        // The sound is played from the handle and both handles are released.
        let handle = calls_to(&e, 0x0041_a250)[0][0];
        assert_eq!(calls_to(&e, 0x00ad_8830), vec![vec![handle, 0]]);
        assert_eq!(calls_to(&e, RELEASE_HANDLE).len(), 2);
        assert_eq!(calls_to(&e, 0x0041_8900)[0][0], handle);
        assert_eq!(calls_to(&e, 0x0047_e040).len(), 1);
    }

    #[test]
    fn a_big_karma_loss_shows_the_big_message() {
        let mut e = engine();
        let (player, changes) = karma_world(&mut e, 10);
        start_log(&mut e);
        player_character_reward_karma(&mut e, player, -200);
        assert_eq!(
            message_of(&e)[..3],
            [
                KARMA_LOSS_BIG_TEXT_SETTING + 4,
                0,
                KARMA_LOSS_BIG_ICON_SETTING + 4
            ]
        );
        assert_eq!(changes.borrow()[0][2], (-200i32) as u32);
    }

    #[test]
    fn a_karma_loss_stops_at_the_lower_limit() {
        let mut e = engine();
        let (player, changes) = karma_world(&mut e, 10);
        start_log(&mut e);
        player_character_reward_karma(&mut e, player, -5000);
        // The change is cut to reach exactly the limit: -1000 - 10.
        assert_eq!(changes.borrow()[0][2], (-1010i32) as u32);
        assert_eq!(
            message_of(&e)[..3],
            [
                KARMA_LOSS_BIG_TEXT_SETTING + 4,
                0,
                KARMA_LOSS_BIG_ICON_SETTING + 4
            ]
        );
    }

    #[test]
    fn karma_gains_use_the_up_sound_and_the_upper_limit() {
        let mut e = engine();
        let (player, changes) = karma_world(&mut e, 10);
        start_log(&mut e);
        player_character_reward_karma(&mut e, player, 30);
        assert_eq!(
            message_of(&e)[..3],
            [KARMA_GAIN_TEXT_SETTING + 4, 0, KARMA_GAIN_ICON_SETTING + 4]
        );
        assert_eq!(calls_to(&e, 0x00ad_7550)[0][2], SOUND_KARMA_UP);
        assert_eq!(changes.borrow()[0][2], 30);

        let (player, changes) = karma_world(&mut e, 10);
        start_log(&mut e);
        player_character_reward_karma(&mut e, player, 100);
        assert_eq!(
            message_of(&e)[..3],
            [
                KARMA_GAIN_BIG_TEXT_SETTING + 4,
                0,
                KARMA_GAIN_BIG_ICON_SETTING + 4
            ]
        );
        assert_eq!(changes.borrow()[0][2], 100);

        let (player, changes) = karma_world(&mut e, 990);
        start_log(&mut e);
        player_character_reward_karma(&mut e, player, 500);
        assert_eq!(changes.borrow()[0][2], 10);
        let _ = player;
    }

    #[test]
    fn a_zero_karma_reward_still_reports_a_gain() {
        let mut e = engine();
        let (player, changes) = karma_world(&mut e, 10);
        start_log(&mut e);
        player_character_reward_karma(&mut e, player, 0);
        assert_eq!(calls_to(&e, 0x00ad_7550)[0][2], SOUND_KARMA_UP);
        assert_eq!(changes.borrow()[0][2], 0);
    }

    // ----- the inventory callback -----

    #[test]
    fn the_inventory_callback_appends_one_entry() {
        let mut e = engine();
        list_helpers(&mut e);
        let item = e.mem.alloc(0x20);
        e.mem.set_u32(item + 4, 3);
        let output = e.mem.alloc(8);
        double(&mut e, ITEM_CHANGE_GET_WORN, eax(1));
        e.register(WORD_AT_8, |_, a| eax(a[0] + 0x100));
        e.register(FORM_ID_OF, |_, a| eax(a[0] + 1));
        double(&mut e, ITEM_CHANGE_GET_FULL_NAME, eax(0x5050));
        start_log(&mut e);
        assert!(!build_inv_string_callback(
            &mut e,
            Ptr::new(item),
            Ptr::new(output)
        ));
        let format = calls_to(&e, SPRINTF);
        assert_eq!(format.len(), 1);
        assert_eq!(
            format[0][1..],
            [
                0x104,
                FORMAT_INVENTORY_ITEM,
                INVENTORY_PREFIX_WORN,
                3,
                0x5050,
                item + 0x100 + 1
            ]
        );
        let buffer = format[0][0];
        assert_eq!(calls_to(&e, STRING_APPEND), vec![vec![output, buffer]]);
        // A carried item gets the other prefix.
        double(&mut e, ITEM_CHANGE_GET_WORN, eax(0));
        assert!(!build_inv_string_callback(
            &mut e,
            Ptr::new(item),
            Ptr::new(output)
        ));
        assert_eq!(calls_to(&e, SPRINTF)[1][3], INVENTORY_PREFIX_CARRIED);
    }

    #[test]
    fn the_inventory_callback_ignores_missing_arguments() {
        let mut e = engine();
        start_log(&mut e);
        assert!(!build_inv_string_callback(
            &mut e,
            Ptr::NULL,
            Ptr::new(0x1000)
        ));
        assert!(!build_inv_string_callback(
            &mut e,
            Ptr::new(0x1000),
            Ptr::NULL
        ));
        assert!(calls_to(&e, SPRINTF).is_empty());
    }

    // ----- Set3D -----

    #[test]
    fn setting_no_3d_before_the_game_is_over_is_refused() {
        let mut e = engine();
        let player = new_player(&mut e);
        e.set_global(GAME_STATE_POINTER, 0x4444u32);
        start_log(&mut e);
        fn_0094eb40(&mut e, player, Ptr::NULL, 1);
        assert_eq!(calls_to(&e, LOG_MESSAGE), vec![vec![FORMAT_SET3D_MESSAGE]]);
        assert!(calls_to(&e, REFERENCE_SET_3D).is_empty());
        // The game being over lets it through.
        double(&mut e, GAME_STATE_TEST_2, eax(1));
        fn_0094eb40(&mut e, player, Ptr::NULL, 1);
        assert_eq!(calls_to(&e, LOG_MESSAGE).len(), 1);
        assert_eq!(calls_to(&e, REFERENCE_SET_3D).len(), 1);
        // So does a missing state object or a real node.
        double(&mut e, GAME_STATE_TEST_2, eax(0));
        double(&mut e, GAME_STATE_TEST_1, eax(0));
        fn_0094eb40(&mut e, player, Ptr::new(0x1234), 0);
        assert_eq!(calls_to(&e, LOG_MESSAGE).len(), 1);
        assert_eq!(
            calls_to(&e, REFERENCE_SET_3D).last().unwrap().clone(),
            vec![player.addr(), 0x1234, 0]
        );
    }

    #[test]
    fn setting_a_real_3d_changes_nothing_else() {
        let mut e = engine();
        let player = new_player(&mut e);
        e.set(
            player,
            PlayerCharacter::p1stPersonBipedAnim,
            Ptr::new(0x1000),
        );
        start_log(&mut e);
        fn_0094eb40(&mut e, player, Ptr::new(0x2000), 1);
        assert_eq!(
            calls_to(&e, REFERENCE_SET_3D),
            vec![vec![player.addr(), 0x2000, 1]]
        );
        assert!(calls_to(&e, DELETE_BIPED_ANIM).is_empty());
        assert_eq!(
            e.get(player, PlayerCharacter::p1stPersonBipedAnim).addr(),
            0x1000
        );
    }

    #[test]
    fn clearing_the_3d_releases_the_first_person_objects() {
        let mut e = engine();
        let player = new_player(&mut e);
        e.set(
            player,
            PlayerCharacter::p1stPersonBipedAnim,
            Ptr::new(0x1000),
        );
        e.set(
            player,
            PlayerCharacter::p1stPersonAnimation,
            Ptr::new(0x2000),
        );
        e.set_global(NODE_CAMERA_1ST_SLOT, 1u32);
        e.set_global(NODE_CAMERA_3RD_SLOT, 2u32);
        e.set_global(NODE_BIP_SLOT, 3u32);
        e.set_global(MODEL_LOADER_POINTER, 0x3000u32);
        e.register(SETTING_TEXT, |_, a| eax(a[0] + 4));
        e.register(IDENTITY, |_, a| eax(a[0]));
        start_log(&mut e);
        fn_0094eb40(&mut e, player, Ptr::NULL, 0);
        assert_eq!(calls_to(&e, DELETE_BIPED_ANIM), vec![vec![0x1000, 1]]);
        assert_eq!(calls_to(&e, DELETE_ANIMATION), vec![vec![0x2000, 1]]);
        assert_eq!(
            e.get(player, PlayerCharacter::p1stPersonBipedAnim).addr(),
            0
        );
        assert_eq!(
            e.get(player, PlayerCharacter::p1stPersonAnimation).addr(),
            0
        );
        assert_eq!(
            calls_to(&e, SMART_POINTER_ASSIGN),
            vec![vec![player.addr() + 0x694, 0]]
        );
        assert_eq!(e.global::<u32>(NODE_CAMERA_1ST_SLOT), 0);
        assert_eq!(e.global::<u32>(NODE_CAMERA_3RD_SLOT), 0);
        assert_eq!(e.global::<u32>(NODE_BIP_SLOT), 0);
        assert_eq!(
            calls_to(&e, MEMSET),
            vec![vec![player.addr() + 0xd74, 0, 0x60]]
        );
        assert_eq!(
            calls_to(&e, MODEL_LOADER_NOTIFY),
            vec![vec![0x3000, MODEL_PATH_SETTING + 4]]
        );
    }

    // ----- Load3D -----

    /// An object whose slot `0x9c` finds a node by name: it answers
    /// `0x1000 + name`, or 0 for the names in `missing`.
    fn finder(e: &mut Engine, missing: Vec<u32>) -> u32 {
        let object = e.mem.alloc(0x40);
        let table = e.mem.alloc(0x800);
        e.mem.set_u32(object, table);
        let target = NEXT_TARGET.fetch_add(4, Ordering::Relaxed);
        e.mem.set_u32(table + 0x9c, target);
        e.register_double(target, move |_, a| {
            if missing.contains(&a[1]) {
                Ret::default()
            } else {
                eax(0x1000 + a[1])
            }
        });
        object
    }

    struct LoadWorld {
        player: Ptr<PlayerCharacter>,
        model: u32,
        node: u32,
        inventory: u32,
        entry_calls: Rc<RefCell<Vec<Vec<u32>>>>,
        proxy_calls: Rc<RefCell<Vec<Vec<u32>>>>,
        process_calls: Rc<RefCell<Vec<Vec<u32>>>>,
        third_person_during_base: Rc<RefCell<Vec<u8>>>,
        file_lists: Rc<RefCell<Vec<String>>>,
    }

    fn load_world(e: &mut Engine, missing_first_person: Vec<u32>) -> LoadWorld {
        let player = new_player(e);
        e.set_global(NODE_NAME_EAD0, 0x11u32);
        e.set_global(NODE_NAME_EAE0, 0x12u32);
        e.set_global(NODE_NAME_EAC0, 0x13u32);
        e.set_global(NODE_NAME_EB30, 0x14u32);
        e.set_global(MODEL_LOADER_POINTER, 0x3000u32);
        e.set_global(IDLE_MANAGER_POINTER, 0x3100u32);
        e.mem.set_cstr(
            MODEL_PATH_SETTING + 4,
            b"Meshes\\Characters\\_1stPerson\\1stPersonMale.nif",
        );
        e.mem
            .set_cstr(TODDLER_SUFFIX, b"\\Locomotion\\Toddler\\IdleAnims");
        e.mem
            .set_cstr(FEMALE_SUFFIX, b"\\Locomotion\\Female\\IdleAnims");
        e.mem
            .set_cstr(MALE_SUFFIX, b"\\Locomotion\\Male\\IdleAnims");
        for word in 0..9u32 {
            e.mem.set_u32(LOAD_3D_MATRIX + 4 * word, 0x100 + word);
        }
        e.register(SETTING_TEXT, |_, a| eax(a[0] + 4));
        e.register(IDENTITY, |_, a| eax(a[0]));
        e.register(OPERATOR_NEW, |e, a| eax(e.mem.alloc(a[0])));
        e.register(SMART_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(0x0055_9450, |e, a| eax(e.mem.u32(a[0])));
        list_helpers(e);
        double(e, 0x007a_f430, eax(0xf0f0));
        let inventory = e.mem.alloc(0x40);
        double(e, 0x004b_f220, eax(inventory));
        let model = finder(e, missing_first_person);
        double(e, 0x0044_7080, eax(model));
        let eye = e.mem.alloc(16);
        e.mem.set_f32(eye + 8, 1.75);
        double(e, 0x0045_bb80, eax(eye));
        e.register(0x0056_fac0, |_, a| eax(a[1]));
        e.register(0x004a_aca0, |_, a| eax(a[0]));
        e.register(0x0048_f810, |_, a| eax(a[0]));
        let node = finder(e, vec![]);
        let third_person_during_base = Rc::new(RefCell::new(Vec::new()));
        let seen = third_person_during_base.clone();
        let player_address = player.addr();
        e.register_double(0x0087_e060, move |e, _| {
            seen.borrow_mut().push(e.mem.u8(player_address + 0x64a));
            eax(node)
        });
        double(e, 0x0095_0a60, eax(0x8800));
        double(e, 0x0044_7330, eax(0x9900));
        e.register(0x0040_6d30, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            eax(a[0])
        });
        e.register(0x0040_ab30, |e, a| {
            let text = e.mem.cstr(a[0]);
            match text.iter().rposition(|byte| *byte as u32 == a[1]) {
                Some(at) => eax(a[0] + at as u32),
                None => eax(0),
            }
        });
        let file_lists = Rc::new(RefCell::new(Vec::new()));
        let lists = file_lists.clone();
        e.register_double(0x0060_0700, move |e, a| {
            lists
                .borrow_mut()
                .push(String::from_utf8(e.mem.cstr(a[1])).unwrap());
            eax(0x5500)
        });
        let process = e.mem.alloc(0x40);
        e.set(player, PlayerCharacter::pCurrentProcess, Ptr::new(process));
        let process_calls = recording_slot(e, process, 0x470, Ret::default());
        slot(e, process, 0x148, eax(0x77));
        let entry_calls = recording_slot(e, process, 0x160, Ret::default());
        double(e, 0x008d_8520, eax(process));
        quiet_slots(e, player.addr(), &[0x1c4]);
        let proxy = e.mem.alloc(0x40);
        let proxy_calls = recording_slot(e, proxy, 0xd0, Ret::default());
        double(e, 0x0093_06d0, eax(0x6600));
        double(e, 0x0062_1ad0, eax(proxy));
        e.register(0x0070_c440, |_, a| eax(a[1]));
        double(e, 0x004a_3a20, eax(0x55));
        let bipeds: Vec<u32> = (0..2u32)
            .map(|which| {
                let holder = e.mem.alloc(8);
                e.mem.set_u32(holder, 0xc0 + which);
                holder
            })
            .collect();
        e.register_double(0x0095_0b00, move |_, a| eax(bipeds[a[1] as usize]));
        LoadWorld {
            player,
            model,
            node,
            inventory,
            entry_calls,
            proxy_calls,
            process_calls,
            third_person_during_base,
            file_lists,
        }
    }

    #[test]
    fn loading_the_3d_builds_the_first_person_model() {
        let mut e = engine();
        let world = load_world(&mut e, vec![]);
        let player = world.player;
        e.set(player, PlayerCharacter::b3rdPerson, false);
        start_log(&mut e);
        let result = player_character_load_3d(&mut e, player, 5);
        assert_eq!(result, world.node);

        // The scope object is opened with the source line and closed again.
        let scope = calls_to(&e, 0x0040_4eb0);
        assert_eq!(scope[0][1..], [0x34, 1, SOURCE_FILE_NAME, 0x2458]);
        assert_eq!(calls_to(&e, 0x0040_4ee0), vec![vec![scope[0][0]]]);

        // The model comes from the model-path setting.
        assert_eq!(
            calls_to(&e, 0x0044_7080),
            vec![vec![0x3000, MODEL_PATH_SETTING + 4, 3, 1, 0, 0, 0]]
        );
        assert_eq!(
            e.get(player, PlayerCharacter::sp1stPerson3D).addr(),
            world.model
        );
        assert_eq!(calls_to(&e, 0x0054_68b0), vec![vec![world.model, 1]]);

        // The named nodes were found in the model.
        assert_eq!(e.global::<u32>(NODE_BY_EAD0_SLOT), 0x1011);
        assert_eq!(e.global::<u32>(NODE_CAMERA_1ST_SLOT), 0x1012);
        assert_eq!(e.global::<u32>(NODE_BIP_SLOT), 0x1013);
        // ... and the third person camera in the 3D the base returned.
        assert_eq!(e.global::<u32>(NODE_CAMERA_3RD_SLOT), 0x1014);
        assert!(calls_to(&e, ERROR).is_empty());

        // The eye height comes from the first person camera node.
        assert_eq!(e.get(player, PlayerCharacter::fEyeHeight), 1.75);

        // The model is rotated by the matrix copied from the exe.
        let rotate = calls_to(&e, 0x0056_fac0);
        assert_eq!(rotate.len(), 1);
        let matrix: Vec<u32> = (0..9)
            .map(|word| e.mem.u32(rotate[0][2] + 4 * word))
            .collect();
        assert_eq!(
            matrix,
            (0..9).map(|word| 0x100 + word).collect::<Vec<u32>>()
        );
        assert_eq!(
            calls_to(&e, 0x0043_fa80),
            vec![vec![world.model, rotate[0][1]]]
        );

        // The objects were built with the right sizes and stored.
        let biped = e.get(player, PlayerCharacter::p1stPersonBipedAnim).addr();
        let animation = e.get(player, PlayerCharacter::p1stPersonAnimation).addr();
        assert_eq!(
            calls_to(&e, 0x004a_aca0),
            vec![vec![biped, player.addr(), world.model]]
        );
        assert_eq!(calls_to(&e, 0x0048_f810), vec![vec![animation]]);
        assert_eq!(e.mem.block_size(biped), Some(0x2b8));
        assert_eq!(e.mem.block_size(animation), Some(0x140));

        // The base load runs with the third person flag on, then restores it.
        assert_eq!(world.third_person_during_base.borrow().clone(), vec![1]);
        assert!(!e.get(player, PlayerCharacter::b3rdPerson));
        assert_eq!(calls_to(&e, 0x0087_e060), vec![vec![player.addr(), 5]]);

        // The animation group is cleared and the file list built.
        assert_eq!(calls_to(&e, 0x0095_0a60), vec![vec![player.addr(), 0]]);
        assert_eq!(calls_to(&e, 0x0049_6080), vec![vec![0x8800, 0, 0]]);
        assert_eq!(
            calls_to(&e, 0x0044_7330),
            vec![vec![0x3000, MODEL_PATH_SETTING + 4, 1, 0, 0xc]]
        );
        // Not a toddler, sex 0: only the male idle animations.
        assert_eq!(
            world.file_lists.borrow().clone(),
            vec!["Meshes\\Characters\\_1stPerson\\Locomotion\\Male\\IdleAnims".to_string()]
        );
        assert_eq!(
            calls_to(&e, 0x0044_7850),
            vec![vec![0x3000, 0x5500, 0x9900]]
        );
        assert_eq!(
            calls_to(&e, 0x0048_ffd0),
            vec![vec![animation, 0x9900, world.model, player.addr(), 1]]
        );

        // The process: slot 0x148 (0, 0) feeds slot 0x160.
        assert_eq!(world.entry_calls.borrow().len(), 1);
        assert_eq!(world.entry_calls.borrow()[0][1], 0x77);
        assert_eq!(
            world.entry_calls.borrow()[0][2..],
            [0, 0],
            "group and force"
        );
        assert_eq!(world.process_calls.borrow().len(), 1);

        // The lighting and the character controller proxy.
        assert_eq!(
            calls_to(&e, 0x008b_0bd0),
            vec![vec![player.addr(), world.model]]
        );
        assert_eq!(
            world.proxy_calls.borrow()[0][1..],
            [world.node, 1, 0, 0x55, 0]
        );
        // The biped bodies are updated.
        assert_eq!(calls_to(&e, 0x0093_80e0), vec![vec![0xc0], vec![0xc1]]);
    }

    #[test]
    fn toddlers_and_women_get_their_own_idle_animations() {
        let mut e = engine();
        let world = load_world(&mut e, vec![]);
        let player = world.player;
        double(&mut e, 0x0083_97d0, eax(1));
        double(&mut e, 0x0087_f4c0, eax(1));
        e.set(player, PlayerCharacter::bIsToddler, true);
        start_log(&mut e);
        player_character_load_3d(&mut e, player, 0);
        assert_eq!(
            world.file_lists.borrow().clone(),
            vec![
                "Meshes\\Characters\\_1stPerson\\Locomotion\\Toddler\\IdleAnims".to_string(),
                "Meshes\\Characters\\_1stPerson\\Locomotion\\Female\\IdleAnims".to_string()
            ]
        );
        // The toddler flag is cleared and announced.
        assert!(!e.get(player, PlayerCharacter::bIsToddler));
        assert_eq!(calls_to(&e, 0x0096_97c0), vec![vec![player.addr(), 1]]);
    }

    #[test]
    fn a_missing_node_is_reported() {
        let mut e = engine();
        let world = load_world(&mut e, vec![0x12, 0x13]);
        // The third person node is missing too.
        let node = finder(&mut e, vec![0x14]);
        e.register_double(0x0087_e060, move |_, _| eax(node));
        double(&mut e, 0x0057_15d0, eax(0x7a7a));
        start_log(&mut e);
        player_character_load_3d(&mut e, world.player, 0);
        let errors = calls_to(&e, ERROR);
        assert_eq!(
            errors,
            vec![
                vec![FORMAT_MISSING_CAMERA_1ST, MODEL_PATH_SETTING + 4],
                vec![FORMAT_MISSING_BIP, MODEL_PATH_SETTING + 4],
                vec![FORMAT_MISSING_CAMERA_3RD, 0x7a7a],
            ]
        );
        assert_eq!(e.global::<u32>(NODE_CAMERA_3RD_SLOT), 0);
    }

    #[test]
    fn a_player_without_worn_items_gets_the_base_form_default_outfit() {
        let mut e = engine();
        let world = load_world(&mut e, vec![]);
        start_log(&mut e);
        player_character_load_3d(&mut e, world.player, 0);
        assert_eq!(
            calls_to(&e, 0x0060_47c0),
            vec![vec![0xf0f0, world.player.addr(), 1, 1, 0, 0]]
        );
        // All twenty slots were examined.
        assert_eq!(calls_to(&e, 0x004c_8c10).len(), 20);
        assert!(calls_to(&e, 0x004c_8220).is_empty());
    }

    #[test]
    fn a_player_with_worn_items_but_nothing_in_slot_six_gets_a_default_item() {
        let mut e = engine();
        let world = load_world(&mut e, vec![]);
        let inventory = world.inventory;
        e.register_double(0x004c_8c10, move |_, a| {
            assert_eq!(a[0], inventory);
            eax(if a[1] == 2 { 0xabc0 } else { 0 })
        });
        let item = e.mem.alloc(8);
        let holder = e.mem.alloc(8);
        e.mem.set_u32(holder, 0xe5);
        e.mem.set_u32(item, holder);
        double(&mut e, 0x004c_8220, eax(item));
        e.register(WORD_AT_8, |_, a| eax(a[0] + 0x10));
        double(&mut e, BASE_FORM_OF, eax(0xba5e));
        let equip = recording_slot(&mut e, world.player.addr(), 0x184, Ret::default());
        start_log(&mut e);
        player_character_load_3d(&mut e, world.player, 0);
        // The first found worn item is released, the search stops there.
        assert_eq!(calls_to(&e, 0x0044_59e0)[0], vec![0xabc0, 1]);
        assert!(calls_to(&e, 0x0060_47c0).is_empty());
        assert_eq!(
            calls_to(&e, 0x004c_8220),
            vec![vec![inventory, 0xba5e, 6, 1]]
        );
        assert_eq!(
            equip.borrow().clone(),
            vec![vec![world.player.addr(), item + 0x10, 1, 0xe5, 0]]
        );
        // The created item is released.
        assert_eq!(
            calls_to(&e, 0x0044_59e0).last().unwrap().clone(),
            vec![item, 1]
        );
    }

    #[test]
    fn a_player_with_an_item_in_slot_six_keeps_it() {
        let mut e = engine();
        let world = load_world(&mut e, vec![]);
        e.register(0x004c_8c10, |_, a| eax(if a[1] == 6 { 0xabc6 } else { 0 }));
        start_log(&mut e);
        player_character_load_3d(&mut e, world.player, 0);
        assert!(calls_to(&e, 0x004c_8220).is_empty());
        assert_eq!(calls_to(&e, 0x0044_59e0)[0], vec![0xabc6, 1]);
    }

    #[test]
    fn the_outfit_check_is_skipped_while_loading_a_save() {
        let mut e = engine();
        let world = load_world(&mut e, vec![]);
        double(&mut e, 0x0042_ce10, eax(1));
        start_log(&mut e);
        player_character_load_3d(&mut e, world.player, 0);
        assert!(calls_to(&e, 0x004b_f220).is_empty());
        assert!(calls_to(&e, 0x0060_47c0).is_empty());
        // The other byte does the same.
        double(&mut e, 0x0042_ce10, eax(0));
        e.set_global(0x011d_8907u32, 1u8);
        start_log(&mut e);
        player_character_load_3d(&mut e, world.player, 0);
        assert!(calls_to(&e, 0x004b_f220).is_empty());
    }

    // ----- ExportProgressData -----

    type Snapshot = Vec<Vec<u32>>;
    /// Marks "string appended" in the recorded text operations.
    const APPENDED: u32 = 0xffff_0001;

    struct ExportWorld {
        player: Ptr<PlayerCharacter>,
        file: u32,
        lines: Rc<RefCell<Vec<Snapshot>>>,
        seeks: Rc<RefCell<Vec<Vec<u32>>>>,
        deletes: Rc<RefCell<Vec<Vec<u32>>>>,
    }

    /// An object whose slots `0x130` and `0x138` answer `this + 0x30`.
    fn named_object(e: &mut Engine) -> u32 {
        let object = e.mem.alloc(0x40);
        let table = e.mem.alloc(0x800);
        e.mem.set_u32(object, table);
        for offset in [0x130u32, 0x138] {
            let target = NEXT_TARGET.fetch_add(4, Ordering::Relaxed);
            e.mem.set_u32(table + offset, target);
            e.register_double(target, |_, a| eax(a[0] + 0x30));
        }
        object
    }

    fn f64_words(value: f64) -> [u32; 2] {
        let bits = value.to_bits();
        [bits as u32, (bits >> 32) as u32]
    }

    fn export_world(e: &mut Engine) -> ExportWorld {
        let player = new_player(e);
        list_helpers(e);
        e.register(SETTING_TEXT, |_, a| eax(a[0] + 4));
        e.register(OPERATOR_NEW, |e, a| eax(e.mem.alloc(a[0])));

        // The file and the text recorder.
        let file = e.mem.alloc(0x20);
        double(e, 0x00b0_0260, eax(file));
        double(e, 0x0089_05f0, eax(1));
        let seeks = recording_slot(e, file, 0x14, Ret::default());
        let deletes = recording_slot(e, file, 0x0, Ret::default());
        let table = e.mem.u32(file);
        let current: Rc<RefCell<Snapshot>> = Rc::default();
        let lines: Rc<RefCell<Vec<Snapshot>>> = Rc::default();
        let recorded = current.clone();
        e.register_double(0x0040_6f60, move |_, a| {
            *recorded.borrow_mut() = vec![a[1..].to_vec()];
            Ret::default()
        });
        let recorded = current.clone();
        e.register_double(0x0040_37f0, move |_, _| {
            recorded.borrow_mut().clear();
            Ret::default()
        });
        let recorded = current.clone();
        e.register_double(STRING_APPEND, move |_, a| {
            recorded.borrow_mut().push(vec![APPENDED, a[1]]);
            Ret::default()
        });
        let write = NEXT_TARGET.fetch_add(4, Ordering::Relaxed);
        e.mem.set_u32(table + 0x38, write);
        let written = lines.clone();
        let recorded = current.clone();
        e.register_double(write, move |_, _| {
            written.borrow_mut().push(recorded.borrow().clone());
            Ret::default()
        });

        // The player's values.
        let owner = player.addr() + 0xa4;
        let owner_table = e.mem.alloc(0x100);
        e.mem.set_u32(owner, owner_table);
        for (offset, base) in [(0u32, 1000u32), (8, 2000)] {
            let target = NEXT_TARGET.fetch_add(4, Ordering::Relaxed);
            e.mem.set_u32(owner_table + offset, target);
            e.register_double(target, move |_, a| eax(base + a[1]));
        }
        slot(e, player.addr(), 0x37c, eax(0x1000));
        slot(e, player.addr(), 0x43c, st0(1.25));
        e.register(0x0040_8da0, |_, a| eax(a[0] + 1));
        e.register(FORM_ID_OF, |_, a| eax(a[0] + 1));
        e.register(0x0055_d520, |_, a| eax(a[0] + 7));
        double(e, 0x0086_7e30, eax(777));
        e.mem.set_u32(REASON_NAME_TABLE, 0xaaa0);
        e.mem.set_u32(REASON_NAME_TABLE + 4, 0xaaa1);
        e.mem.set_u32(SEX_NAME_TABLE + 4, 0xbbb1);
        e.mem.set_u32(CAUSE_NAME_TABLE + 12, 0xccc3);
        double(e, 0x0087_f4c0, eax(1));
        double(e, 0x0087_f9f0, eax(0x0007_1234));
        e.register(0x005a_3370, |_, a| eax(5000 + a[0]));
        double(e, PARENT_CELL_OF, eax(0x4000));
        double(e, 0x0042_5fd0, eax(1));
        ExportWorld {
            player,
            file,
            lines,
            seeks,
            deletes,
        }
    }

    #[test]
    fn nothing_is_written_when_the_file_cannot_be_opened() {
        let mut e = engine();
        let world = export_world(&mut e);
        double(&mut e, 0x00b0_0260, eax(0));
        start_log(&mut e);
        player_character_export_progress_data(&mut e, world.player, 0);
        assert!(world.lines.borrow().is_empty());
        assert!(calls_to(&e, 0x0040_37b0).is_empty());
        // The path is built from the directory setting.
        let path = calls_to(&e, SPRINTF);
        assert_eq!(
            path[0][1..],
            [0x104, FORMAT_EXPORT_PATH, EXPORT_DIRECTORY_SETTING + 4]
        );
        // The block of the file is 0x158 bytes.
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x158]]);
        // A file that does not open is not written either.
        double(&mut e, 0x00b0_0260, eax(world.file));
        double(&mut e, 0x0089_05f0, eax(0));
        player_character_export_progress_data(&mut e, world.player, 0);
        assert!(world.lines.borrow().is_empty());
        assert!(world.seeks.borrow().is_empty());
    }

    #[test]
    fn the_progress_record_has_all_its_lines() {
        let mut e = engine();
        let world = export_world(&mut e);
        let player = world.player;
        e.set_global(EXPORT_SEEK_MODE, 2u32);

        // Two perks, the second one empty.
        let perk = e.mem.alloc(0x20);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, perk);
        e.mem.set_u8(entry + 4, 2);
        let empty_entry = e.mem.alloc(8);
        let second = node(&mut e, empty_entry, 0);
        e.mem.set_u32(player.addr() + 0x87c, entry);
        e.mem.set_u32(player.addr() + 0x880, second);

        // The inventory and the values taken from it.
        let inventory = e.mem.alloc(0x40);
        double(&mut e, 0x004b_f220, eax(inventory));
        e.register(0x0093_abe0, |e, a| {
            e.mem.set_f32(a[1], 1.5);
            e.mem.set_f32(a[2], 2.5);
            Ret::default()
        });
        double(&mut e, 0x004d_0f40, eax(321));
        double(&mut e, 0x0057_7d50, eax(654));

        // The quest log: one real quest entry, one without a quest.
        let quest_item = e.mem.alloc(8);
        let other_item = e.mem.alloc(8);
        let quest = named_object(&mut e);
        e.register_double(0x0060_fd70, move |_, a| eax((a[0] == quest_item) as u32));
        e.register_double(0x005e_3fa0, move |_, a| {
            eax(if a[0] == quest_item { quest } else { 0 })
        });
        let quest_second = node(&mut e, other_item, 0);
        e.mem.set_u32(player.addr() + 0x6b0, quest_item);
        e.mem.set_u32(player.addr() + 0x6b4, quest_second);

        start_log(&mut e);
        player_character_export_progress_data(&mut e, player, 0);

        let lines = world.lines.borrow().clone();
        assert_eq!(lines.len(), 129);
        let one = |words: &[u32]| vec![words.to_vec()];
        assert_eq!(lines[0], one(&[FORMAT_TEXT, UNKNOWN_TEXT]));
        assert_eq!(lines[1], one(&[FORMAT_TEXT, GAME_VERSION_TEXT]));
        assert_eq!(lines[2], one(&[FORMAT_INT, 777]));
        assert_eq!(lines[3], one(&[FORMAT_TEXT, 0xaaa0]));
        assert_eq!(lines[4], one(&[FORMAT_TEXT, player.addr() + 7]));
        assert_eq!(lines[5], one(&[FORMAT_TEXT, 0x1000 + 0x18 + 1]));
        assert_eq!(lines[6], one(&[FORMAT_TEXT, 0xbbb1]));
        assert_eq!(lines[7], one(&[FORMAT_INT, 0x1234]));
        assert_eq!(lines[8], one(&[FORMAT_INT, 2000 + 0x18]));

        // The perk line: quote, one entry, closing quote.
        let perk_format = calls_to(&e, SPRINTF)
            .into_iter()
            .find(|words| words[2] == FORMAT_PERK)
            .unwrap();
        assert_eq!(perk_format[3..], [perk + 0x18 + 1, perk + 1, 2]);
        assert_eq!(
            lines[9],
            vec![
                vec![APPENDED, perk_format[0]],
                vec![APPENDED, TEXT_CLOSE_QUOTE]
            ]
        );

        // Fourteen skills (actual value slot 8), then 7 and 20 pairs.
        for index in 0..14u32 {
            assert_eq!(
                lines[10 + index as usize],
                one(&[FORMAT_INT, 2000 + 0x20 + index])
            );
        }
        for index in 0..7u32 {
            assert_eq!(
                lines[24 + 2 * index as usize],
                one(&[FORMAT_INT, 1000 + 5 + index])
            );
            assert_eq!(
                lines[25 + 2 * index as usize],
                one(&[FORMAT_INT, 2000 + 5 + index])
            );
        }
        for index in 0..20u32 {
            assert_eq!(
                lines[38 + 2 * index as usize],
                one(&[FORMAT_INT, 1000 + 12 + index])
            );
            assert_eq!(
                lines[39 + 2 * index as usize],
                one(&[FORMAT_INT, 2000 + 12 + index])
            );
        }

        // The inventory is built by the callback of 0094ec90.
        assert_eq!(lines[78], vec![vec![APPENDED, TEXT_CLOSE_QUOTE]]);
        let walk = calls_to(&e, 0x004d_4530);
        assert_eq!(walk.len(), 1);
        assert_eq!(walk[0][0], inventory);
        assert_eq!(walk[0][1], 0x0094_ec90);
        assert_eq!(walk[0][3], 0);
        let [low1, high1] = f64_words(1.5);
        let [low2, high2] = f64_words(2.5);
        assert_eq!(
            lines[79],
            one(&[FORMAT_TWO_FLOATS, low1, high1, low2, high2])
        );
        assert_eq!(lines[80], one(&[FORMAT_INT, 321]));
        assert_eq!(lines[81], one(&[FORMAT_INT, 654]));
        let [low, high] = f64_words(1.25);
        assert_eq!(lines[82], one(&[FORMAT_ONE_FLOAT, low, high]));

        // The quest line.
        let quest_format = calls_to(&e, SPRINTF)
            .into_iter()
            .find(|words| words[2] == FORMAT_QUEST)
            .unwrap();
        assert_eq!(quest_format[3..], [quest + 0x30, quest + 1]);
        assert_eq!(
            lines[83],
            vec![
                vec![APPENDED, quest_format[0]],
                vec![APPENDED, TEXT_CLOSE_QUOTE]
            ]
        );

        // The 43 counters, the cell (an interior), the line end.
        for index in 0..43u32 {
            assert_eq!(lines[84 + index as usize], one(&[FORMAT_INT, 5000 + index]));
        }
        assert_eq!(
            lines[127],
            one(&[FORMAT_NAME_ID, 0x4000 + 0x18 + 1, 0x4001])
        );
        assert_eq!(lines[128], one(&[TEXT_LINE_END]));

        // The file was seeked, closed and deleted; the string was built and
        // destroyed.
        assert_eq!(world.seeks.borrow().clone(), vec![vec![world.file, 0, 2]]);
        assert_eq!(calls_to(&e, 0x00af_fd10), vec![vec![world.file]]);
        assert_eq!(world.deletes.borrow().clone(), vec![vec![world.file, 1]]);
        let text = calls_to(&e, 0x0040_37b0)[0][0];
        assert_eq!(calls_to(&e, 0x0040_37d0), vec![vec![text]]);
        // The file is opened for writing with the buffer size 0x4000.
        let open = calls_to(&e, 0x00b0_0260);
        assert_eq!(open[0][2..], [1, 0x4000, 0]);
    }

    #[test]
    fn the_record_without_an_inventory_has_empty_quotes_and_zero_weight() {
        let mut e = engine();
        let world = export_world(&mut e);
        e.mem.set_u8(BUILD_NAME_BUFFER, b'N');
        e.mem.set_u32(world.player.addr() + 0x87c, 0);
        // The first perk entry list head is empty: no perks at all.
        start_log(&mut e);
        player_character_export_progress_data(&mut e, world.player, 1);
        let lines = world.lines.borrow().clone();
        assert_eq!(lines[0], vec![vec![FORMAT_TEXT, BUILD_NAME_BUFFER]]);
        assert_eq!(lines[9], vec![vec![APPENDED, TEXT_CLOSE_QUOTE]]);
        assert_eq!(lines[78], vec![vec![TEXT_EMPTY_QUOTES]]);
        assert_eq!(lines[79][0][1..], [0, 0, 0, 0]);
        assert_eq!(lines[80], vec![vec![FORMAT_INT, 0]]);
        assert!(calls_to(&e, 0x004d_4530).is_empty());
        assert_eq!(calls_to(&e, 0x004d_0f40).len(), 0);
    }

    #[test]
    fn the_cell_line_covers_exteriors_and_no_cell() {
        // A cell in a world space.
        let mut e = engine();
        let world = export_world(&mut e);
        double(&mut e, 0x0042_5fd0, eax(0));
        double(&mut e, 0x0054_ddd0, eax(0x7000));
        double(&mut e, 0x0054_4c30, eax(11));
        double(&mut e, 0x0054_4c60, eax(22));
        player_character_export_progress_data(&mut e, world.player, 0);
        assert_eq!(
            world.lines.borrow()[127],
            vec![vec![FORMAT_CELL_WORLD, 0x7000 + 0x19, 11, 22, 0x4001]]
        );
        // A cell without a world space.
        let mut e = engine();
        let world = export_world(&mut e);
        double(&mut e, 0x0042_5fd0, eax(0));
        double(&mut e, 0x0054_ddd0, eax(0));
        double(&mut e, 0x0054_4c30, eax(11));
        double(&mut e, 0x0054_4c60, eax(22));
        player_character_export_progress_data(&mut e, world.player, 0);
        assert_eq!(
            world.lines.borrow()[127],
            vec![vec![FORMAT_CELL_UNKNOWN_WORLD, 11, 22, 0x4001]]
        );
        // No cell at all.
        let mut e = engine();
        let world = export_world(&mut e);
        double(&mut e, PARENT_CELL_OF, eax(0));
        player_character_export_progress_data(&mut e, world.player, 0);
        assert_eq!(world.lines.borrow()[127], vec![vec![TEXT_NONE]]);
    }

    /// The lines after the cell line for a death (reason 1).
    fn death_tail(e: &mut Engine, world: &ExportWorld) -> Vec<Snapshot> {
        player_character_export_progress_data(e, world.player, 1);
        world.lines.borrow()[128..].to_vec()
    }

    #[test]
    fn a_death_names_the_cause_the_killer_and_the_weapon() {
        let mut e = engine();
        let world = export_world(&mut e);
        let killer = e.mem.alloc(0x40);
        e.set(world.player, PlayerCharacter::pMyKiller, Ptr::new(killer));
        // The cause: damage type 3 from an object.
        let extra = e.mem.alloc(0x20);
        e.mem.set_i32(extra + 0x10, 3);
        let source = named_object(&mut e);
        e.mem.set_u32(extra + 0x14, source);
        double(&mut e, EXTRA_LIST_OF, eax(0xe0));
        double(&mut e, GET_EXTRA_DATA, eax(extra));
        double(&mut e, 0x0044_0e30, eax(0x6161));
        // A plain killer with a base form, damage and a weapon.
        let base = named_object(&mut e);
        double(&mut e, BASE_FORM_OF, eax(base));
        double(&mut e, 0x0056_8ad0, st0(12.5));
        let weapon = e.mem.alloc(0x40);
        double(&mut e, 0x008a_1710, eax(weapon));
        start_log(&mut e);
        let tail = death_tail(&mut e, &world);
        let [low, high] = f64_words(12.5);
        assert_eq!(
            tail,
            vec![
                vec![vec![FORMAT_TEXT, 0xccc3]],
                vec![vec![FORMAT_NAME_ID, killer + 7, killer + 1]],
                vec![vec![FORMAT_NAME_ID, base + 0x30, base + 1]],
                vec![vec![FORMAT_ONE_FLOAT, low, high]],
                vec![vec![FORMAT_NAME_ID, weapon + 0x31, weapon + 1]],
                vec![vec![TEXT_LINE_END]],
            ]
        );
        // The object of the cause is formatted but not written: its format
        // call is there.
        let object_format: Vec<Vec<u32>> = calls_to(&e, 0x0040_6f60)
            .into_iter()
            .filter(|words| words[1] == FORMAT_CAUSE_OBJECT)
            .collect();
        assert_eq!(object_format.len(), 1);
        assert_eq!(object_format[0][2..], [0x6161, source + 0x30, source + 1]);
    }

    #[test]
    fn a_death_without_extra_data_or_killer_is_unknown() {
        let mut e = engine();
        let world = export_world(&mut e);
        double(&mut e, GET_EXTRA_DATA, eax(0));
        let tail = death_tail(&mut e, &world);
        assert_eq!(
            tail,
            vec![
                vec![vec![TEXT_UNKNOWN_CAUSE]],
                vec![vec![TEXT_UNKNOWN_OBJECT]],
                vec![vec![TEXT_UNKNOWN_KILLER]],
                vec![vec![TEXT_LINE_END]],
            ]
        );
    }

    #[test]
    fn a_death_without_a_cause_type_says_none() {
        let mut e = engine();
        let world = export_world(&mut e);
        let extra = e.mem.alloc(0x20);
        e.mem.set_i32(extra + 0x10, -1);
        double(&mut e, GET_EXTRA_DATA, eax(extra));
        let tail = death_tail(&mut e, &world);
        assert_eq!(tail[0], vec![vec![TEXT_NONE]]);
        // Without a source object, "none" is formatted and not written.
        assert_eq!(tail[1], vec![vec![TEXT_UNKNOWN_KILLER]]);
    }

    fn levelled_killer(
        e: &mut Engine,
        world: &ExportWorld,
        original: u32,
        template: u32,
    ) -> Vec<Snapshot> {
        let killer = e.mem.alloc(0x40);
        e.set(world.player, PlayerCharacter::pMyKiller, Ptr::new(killer));
        double(e, GET_EXTRA_DATA, eax(0));
        double(e, 0x0056_af40, eax(1));
        double(e, 0x0042_16f0, eax(original));
        double(e, 0x0042_1720, eax(template));
        death_tail(e, world)
    }

    #[test]
    fn a_levelled_killer_shows_the_original_and_the_template() {
        let mut e = engine();
        let world = export_world(&mut e);
        let original = named_object(&mut e);
        let template = named_object(&mut e);
        let tail = levelled_killer(&mut e, &world, original, template);
        // The extras line, the killer line, the levelled line.
        assert_eq!(
            tail[3],
            vec![vec![
                FORMAT_LEVELLED_BOTH,
                original + 0x30,
                original + 1,
                template + 0x30,
                template + 1
            ]]
        );
    }

    #[test]
    fn a_levelled_killer_may_lack_one_of_them() {
        let mut e = engine();
        let world = export_world(&mut e);
        let original = named_object(&mut e);
        let tail = levelled_killer(&mut e, &world, original, 0);
        assert_eq!(
            tail[3],
            vec![vec![
                FORMAT_LEVELLED_ORIGINAL_ONLY,
                original + 0x30,
                original + 1
            ]]
        );
        let mut e = engine();
        let world = export_world(&mut e);
        let template = named_object(&mut e);
        let tail = levelled_killer(&mut e, &world, 0, template);
        assert_eq!(
            tail[3],
            vec![vec![
                FORMAT_LEVELLED_TEMPLATE_ONLY,
                template + 0x30,
                template + 1
            ]]
        );
        let mut e = engine();
        let world = export_world(&mut e);
        let tail = levelled_killer(&mut e, &world, 0, 0);
        assert_eq!(tail[3], vec![vec![TEXT_UNKNOWN_LEVELLED]]);
        // The weapon line reports no weapon.
        assert_eq!(tail[5], vec![vec![TEXT_NO_WEAPON]]);
    }

    // ----- second session: camera switch, animations, quests -----

    /// Makes `00559450` read the pointer at the address it is given, as the
    /// dereference of a `NiPointer` does.
    fn pointer_reads(e: &mut Engine) {
        e.register(0x0055_9450, |e, a| eax(e.mem.u32(a[0])));
    }

    /// A block of words in the heap (at least four).
    fn word_block(e: &mut Engine, words: &[u32]) -> u32 {
        let block = e.mem.alloc(4 * words.len().max(4) as u32);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(block + 4 * i as u32, *word);
        }
        block
    }

    /// An object whose vtable slot `offset` is a double that answers `ret`.
    fn object_with_slot(e: &mut Engine, offset: u32, ret: Ret) -> u32 {
        let object = e.mem.alloc(16);
        slot(e, object, offset, ret);
        object
    }

    #[test]
    fn asking_for_the_third_person_copies_the_restored_value() {
        let mut e = engine();
        let player = new_player(&mut e);
        slot(&mut e, player.addr(), 0x22c, eax(1));
        let setting = word_block(&mut e, &[0]);
        e.mem.set_f32(setting, 7.5);
        double(&mut e, SETTING_FLOAT_POINTER, eax(setting));
        assert!(player_character_set_first_person(&mut e, player, 0));
        assert_eq!(e.mem.u8(player.addr() + 0x64c), 1);
        assert_eq!(e.global::<f32>(VANITY_RESTORED_VALUE), 7.5);
        // Asking for the first person changes it back; asking again changes
        // nothing.
        assert!(player_character_set_first_person(&mut e, player, 1));
        assert_eq!(e.mem.u8(player.addr() + 0x64c), 0);
        assert!(!player_character_set_first_person(&mut e, player, 1));
    }

    #[test]
    fn the_first_person_model_receives_the_camera_position_and_the_switch() {
        let mut e = engine();
        pointer_reads(&mut e);
        let player = new_player(&mut e);
        slot(&mut e, player.addr(), 0x22c, eax(0));
        e.mem.set_u32(player.addr() + 0x694, 0x0600_0100);
        e.set_global(NODE_CAMERA_1ST_SLOT, 0x0600_0200u32);
        let position = word_block(&mut e, &[0, 0, 0]);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_f32(position + 8, 3.0);
        double(&mut e, 0x0045_bb80, eax(position));
        start_log(&mut e);
        assert!(player_character_set_first_person(&mut e, player, 0));
        assert_eq!(calls_to(&e, 0x0045_bb80), vec![vec![0x0600_0200]]);
        assert_eq!(e.global::<f32>(CAMERA_POSITION_COPY), 1.0);
        assert_eq!(e.global::<f32>(CAMERA_POSITION_COPY + 4), 2.0);
        assert_eq!(e.global::<f32>(CAMERA_POSITION_COPY + 8), 3.0);
        // The switch (`00951a10`) looked for the player's own 3D.
        assert!(!calls_to(&e, 0x0043_fcd0).is_empty());
    }

    #[test]
    fn a_hidden_model_marks_the_player_as_third_person_when_the_first_is_asked_for() {
        let mut e = engine();
        pointer_reads(&mut e);
        let player = new_player(&mut e);
        e.mem.set_u32(player.addr() + 0x694, 0x0600_0100);
        double(&mut e, 0x0045_6610, eax(1));
        assert!(!player_character_set_first_person(&mut e, player, 1));
        assert!(e.get(player, PlayerCharacter::b3rdPerson));
        // With the model not hidden the flag stays clear.
        let other = new_player(&mut e);
        e.mem.set_u32(other.addr() + 0x694, 0x0600_0100);
        double(&mut e, 0x0045_6610, eax(0));
        player_character_set_first_person(&mut e, other, 1);
        assert!(!e.get(other, PlayerCharacter::b3rdPerson));
    }

    #[test]
    fn both_switch_flags_copy_the_wanted_camera_at_once() {
        let mut e = engine();
        let player = new_player(&mut e);
        e.mem.set_u8(player.addr() + 0x64a, 1);
        e.set_global(CAMERA_SWITCH_FLAG_011F21D0, 1u8);
        e.set_global(CAMERA_SWITCH_FLAG_011F21D1, 1u8);
        // The wanted camera changes from the third to the first person: the
        // camera byte follows at once.
        e.mem.set_u8(player.addr() + 0x64c, 1);
        assert!(player_character_set_first_person(&mut e, player, 1));
        assert_eq!(e.mem.u8(player.addr() + 0x64a), 0);
        // With one flag clear the camera byte is left alone.
        e.mem.set_u8(player.addr() + 0x64a, 1);
        e.mem.set_u8(player.addr() + 0x64c, 1);
        e.set_global(CAMERA_SWITCH_FLAG_011F21D1, 0u8);
        player_character_set_first_person(&mut e, player, 1);
        assert_eq!(e.mem.u8(player.addr() + 0x64a), 1);
    }

    #[test]
    fn the_zoom_update_adopts_the_wanted_camera_unless_the_zoom_is_large() {
        let mut e = engine();
        pointer_reads(&mut e);
        e.set_global(FIRST_PERSON_ZOOM_LIMIT, 30.0f64);
        let player = new_player(&mut e);
        // Same camera: nothing.
        start_log(&mut e);
        player_character_update_first_person_zoom(&mut e, player);
        assert!(calls_to(&e, 0x0055_9450).is_empty());
        // Different camera, zoom above the limit: nothing.
        e.mem.set_u8(player.addr() + 0x64c, 1);
        e.set_global(VANITY_VALUE_07DC, 31.0f32);
        player_character_update_first_person_zoom(&mut e, player);
        assert_eq!(e.mem.u8(player.addr() + 0x64a), 0);
        // Zoom at the limit: adopted.
        e.set_global(VANITY_VALUE_07DC, 30.0f32);
        player_character_update_first_person_zoom(&mut e, player);
        assert_eq!(e.mem.u8(player.addr() + 0x64a), 1);
        // Going back to the first person with a model applies the switch.
        e.mem.set_u8(player.addr() + 0x64c, 0);
        e.mem.set_u32(player.addr() + 0x694, 0x0600_0100);
        player_character_update_first_person_zoom(&mut e, player);
        assert_eq!(e.mem.u8(player.addr() + 0x64a), 0);
        assert!(!calls_to(&e, 0x0043_fcd0).is_empty());
    }

    #[test]
    fn forcing_the_temporary_third_person_switches_from_the_first_person() {
        let mut e = engine();
        let player = new_player(&mut e);
        slot(&mut e, player.addr(), 0x22c, eax(0));
        let setting = word_block(&mut e, &[0]);
        e.mem.set_f32(setting, 9.0);
        double(&mut e, SETTING_FLOAT_POINTER, eax(setting));
        e.set_global(VANITY_RESTORED_VALUE, 3.0f32);
        // Refused while a temporary camera is already forced.
        double(&mut e, 0x0057_21e0, eax(1));
        assert!(!player_character_force_temp_3rd_person(&mut e, player, 1));
        assert_eq!(e.mem.u8(player.addr() + 0x64d), 0);
        // In the first person: remembers to switch back and asks for the
        // third person (SetFirstPerson(0) makes `bWant3rdPerson` 1).
        double(&mut e, 0x0057_21e0, eax(0));
        assert!(player_character_force_temp_3rd_person(&mut e, player, 1));
        assert_eq!(e.mem.u8(player.addr() + 0x64d), 1);
        assert_eq!(e.mem.u8(player.addr() + 0x64e), 1);
        assert_eq!(e.mem.u8(player.addr() + 0x64c), 1);
        assert_eq!(e.global::<f32>(VANITY_RESTORED_VALUE), 9.0);
        // Already in the third person: only the flag; the restored value is
        // not lowered.
        let other = new_player(&mut e);
        e.mem.set_u8(other.addr() + 0x64a, 1);
        e.set_global(VANITY_RESTORED_VALUE, 12.0f32);
        assert!(!player_character_force_temp_3rd_person(&mut e, other, 1));
        assert_eq!(e.global::<f32>(VANITY_RESTORED_VALUE), 12.0);
        assert_eq!(e.mem.u8(other.addr() + 0x64e), 0);
    }

    #[test]
    fn the_temporary_third_person_ends_when_the_process_is_idle() {
        let mut e = engine();
        let player = new_player(&mut e);
        let process = e.mem.alloc(16);
        slot(&mut e, process, 0x3e4, eax(0xffff_ffff));
        slot(&mut e, process, 0x40c, eax(0));
        e.set(player, PlayerCharacter::pCurrentProcess, Ptr::new(process));
        slot(&mut e, player.addr(), 0x22c, eax(0));
        // Not forced: nothing happens.
        player_character_update_temp_3rd_person(&mut e, player);
        // Forced, with the switch-back remembered; the vanity flag blocks it.
        e.mem.set_u8(player.addr() + 0x64d, 1);
        e.mem.set_u8(player.addr() + 0x64e, 1);
        e.mem.set_u8(player.addr() + 0x64c, 1);
        e.set_global(VANITY_SAVED_FLAG, 1u8);
        player_character_update_temp_3rd_person(&mut e, player);
        assert_eq!(e.mem.u8(player.addr() + 0x64d), 1);
        // Without it the temporary third person ends and the first person is
        // asked for (`bWant3rdPerson` 0).
        e.set_global(VANITY_SAVED_FLAG, 0u8);
        player_character_update_temp_3rd_person(&mut e, player);
        assert_eq!(e.mem.u8(player.addr() + 0x64d), 0);
        assert_eq!(e.mem.u8(player.addr() + 0x64e), 0);
        assert_eq!(e.mem.u8(player.addr() + 0x64c), 0);
    }

    #[test]
    fn forcing_the_temporary_first_person_lowers_the_sights_and_asks_for_the_first_person() {
        let mut e = engine();
        pointer_reads(&mut e);
        let player = new_player(&mut e);
        slot(&mut e, player.addr(), 0x22c, eax(0));
        double(&mut e, 0x008b_bc10, eax(1));
        double(&mut e, 0x0043_fcd0, eax(0x0600_0300));
        double(&mut e, 0x0045_6610, eax(0));
        start_log(&mut e);
        // In the first person already (`b3rdPerson` 0): the player's own 3D is
        // un-culled and nothing else.
        assert!(!player_character_force_temp_1st_person(&mut e, player, 0));
        assert_eq!(e.mem.u8(player.addr() + 0x64f), 1);
        assert_eq!(
            calls_to(&e, 0x008b_b650),
            vec![vec![player.addr(), 0, 0, 0]]
        );
        assert_eq!(calls_to(&e, 0x0089_4cc0), vec![vec![player.addr(), 0]]);
        assert_eq!(calls_to(&e, 0x0045_0f90), vec![vec![0x0600_0300, 1]]);
        assert_eq!(e.mem.u8(player.addr() + 0x650), 0);
        // In the third person: switches.
        e.mem.set_u8(player.addr() + 0x64a, 1);
        player_character_force_temp_1st_person(&mut e, player, 0);
        assert_eq!(e.mem.u8(player.addr() + 0x650), 1);
        assert_eq!(e.mem.u8(player.addr() + 0x64c), 0);
    }

    #[test]
    fn forcing_the_temporary_first_person_with_the_sights_kept_checks_the_model_state() {
        let mut e = engine();
        pointer_reads(&mut e);
        let player = new_player(&mut e);
        slot(&mut e, player.addr(), 0x22c, eax(0));
        e.mem.set_u32(player.addr() + 0x694, 0x0600_0100);
        double(&mut e, 0x0045_6610, eax(1));
        start_log(&mut e);
        assert!(!player_character_force_temp_1st_person(&mut e, player, 1));
        assert!(calls_to(&e, 0x008b_bc10).is_empty());
        assert_eq!(e.mem.u8(player.addr() + 0x650), 0);
        double(&mut e, 0x0045_6610, eax(0));
        player_character_force_temp_1st_person(&mut e, player, 1);
        assert_eq!(e.mem.u8(player.addr() + 0x650), 1);
    }

    #[test]
    fn the_temporary_first_person_ends_unless_the_pipboy_or_the_sights_hold_it() {
        let mut e = engine();
        let player = new_player(&mut e);
        slot(&mut e, player.addr(), 0x22c, eax(0));
        let process = e.mem.alloc(16);
        slot(&mut e, process, 0x148, eax(0x0600_0400));
        e.set(player, PlayerCharacter::pCurrentProcess, Ptr::new(process));
        e.mem.set_u8(player.addr() + 0x64f, 1);
        e.mem.set_u8(player.addr() + 0x650, 1);
        // The Pipboy is active.
        double(&mut e, 0x0096_7ae0, eax(1));
        player_character_update_temp_1st_person(&mut e, player);
        assert_eq!(e.mem.u8(player.addr() + 0x64f), 1);
        // The sights are up and the weapon is accepted.
        double(&mut e, 0x0096_7ae0, eax(0));
        double(&mut e, 0x008b_bc10, eax(1));
        double(&mut e, 0x0048_cee0, eax(1));
        player_character_update_temp_1st_person(&mut e, player);
        assert_eq!(e.mem.u8(player.addr() + 0x64f), 1);
        // Not accepted: ends, and switches back (SetFirstPerson(0)).
        double(&mut e, 0x0048_cee0, eax(0));
        player_character_update_temp_1st_person(&mut e, player);
        assert_eq!(e.mem.u8(player.addr() + 0x64f), 0);
        assert_eq!(e.mem.u8(player.addr() + 0x650), 0);
        assert_eq!(e.mem.u8(player.addr() + 0x64c), 1);
        assert_eq!(e.global::<u8>(TEMP_FIRST_PERSON_ENDED), 1);
        // The object test of 0044ddc0 answering 4 also holds it.
        e.mem.set_u8(player.addr() + 0x64f, 1);
        double(&mut e, 0x0044_ddc0, eax(4));
        player_character_update_temp_1st_person(&mut e, player);
        assert_eq!(e.mem.u8(player.addr() + 0x64f), 1);
    }

    #[test]
    fn the_field_of_view_goes_to_the_camera_and_the_shader_manager() {
        let mut e = engine();
        let player = new_player(&mut e);
        double(&mut e, 0x0045_c670, eax(0x0600_0500));
        start_log(&mut e);
        assert_eq!(fn_00950610(&mut e, player, 75.0), 75.0);
        assert_eq!(e.get(player, PlayerCharacter::fFOV), 75.0);
        assert_eq!(
            calls_to(&e, 0x00c5_2020),
            vec![vec![0x0600_0500, 75.0f32.to_bits(), 0, 0, 0]]
        );
        assert_eq!(calls_to(&e, 0x00b5_4000), vec![vec![75.0f32.to_bits()]]);
    }

    #[test]
    fn resurrecting_replaces_the_process_and_rebuilds_the_camera_caster() {
        let mut e = engine();
        let player = new_player(&mut e);
        e.set(player, PlayerCharacter::fHealthModifier, 5.0);
        slot(&mut e, player.addr(), 0x1c4, eax(0));
        let old = e.mem.alloc(16);
        let destroyed = recording_slot(&mut e, old, 0, eax(0));
        e.set(player, PlayerCharacter::pCurrentProcess, Ptr::new(old));
        e.register(OPERATOR_NEW, |e, a| eax(e.mem.alloc(a[0])));
        let base = e.mem.alloc(16);
        let base_destroyed = recording_slot(&mut e, base, 0, eax(0));
        double(&mut e, 0x0090_6dc0, eax(base));
        let high = e.mem.alloc(16);
        let adopted = recording_slot(&mut e, high, 4, eax(0));
        double(&mut e, 0x008d_7510, eax(high));
        double(&mut e, 0x0093_1850, eax(7));
        double(&mut e, 0x0093_06d0, eax(0x0600_0700));
        double(&mut e, 0x004a_3a20, eax(0x55));
        double(&mut e, 0x0062_0850, eax(0x0600_0800));
        let setting = word_block(&mut e, &[0]);
        e.mem.set_f32(setting, 2.5);
        double(&mut e, SETTING_FLOAT_POINTER, eax(setting));
        start_log(&mut e);
        player_character_resurrect(&mut e, player, 1, 2, 3);
        assert_eq!(e.get(player, PlayerCharacter::fHealthModifier), 0.0);
        assert_eq!(destroyed.borrow().len(), 1);
        assert_eq!(destroyed.borrow()[0], vec![old, 1]);
        // The new base process was handed to the new high process.
        assert_eq!(adopted.borrow()[0], vec![high, base]);
        // The temporary base process is destroyed once the new one is in.
        assert_eq!(base_destroyed.borrow()[0], vec![base, 1]);
        assert_eq!(e.get(player, PlayerCharacter::pCurrentProcess).addr(), high);
        assert_eq!(
            calls_to(&e, 0x0096_d470),
            vec![vec![PROCESS_LISTS, player.addr(), 7]; 2]
        );
        assert_eq!(calls_to(&e, 0x0070_4e10), vec![vec![0x10]]);
        let caster = calls_to(&e, 0x0062_0850);
        assert_eq!(caster[0][1..], [2.5f32.to_bits(), 0x55]);
        assert_eq!(e.mem.u32(player.addr() + 0x21c), 0x0600_0800);
        assert_eq!(
            calls_to(&e, 0x0089_f780),
            vec![vec![player.addr(), 0, 0, 0]]
        );
        assert_eq!(calls_to(&e, 0x0056_59f0), vec![vec![player.addr()]]);
        assert_eq!(calls_to(&e, 0x0040_fbf0), vec![vec![PROCESS_LOCK, 0]]);
        assert_eq!(calls_to(&e, 0x0040_fba0), vec![vec![PROCESS_LOCK]]);
    }

    #[test]
    fn resurrecting_without_a_controller_clears_the_camera_caster() {
        let mut e = engine();
        let player = new_player(&mut e);
        slot(&mut e, player.addr(), 0x1c4, eax(0));
        e.mem.set_u32(player.addr() + 0x21c, 0x1234);
        e.register(OPERATOR_NEW, |e, a| eax(e.mem.alloc(a[0])));
        let high = e.mem.alloc(16);
        slot(&mut e, high, 4, eax(0));
        double(&mut e, 0x008d_7510, eax(high));
        start_log(&mut e);
        player_character_resurrect(&mut e, player, 0, 0, 0);
        assert_eq!(e.mem.u32(player.addr() + 0x21c), 0);
        assert!(calls_to(&e, 0x0062_0850).is_empty());
    }

    #[test]
    fn equipment_damage_is_skipped_in_god_mode() {
        let mut e = engine();
        let player = new_player(&mut e);
        let flag = word_block(&mut e, &[0]);
        double(&mut e, 0x0040_8d60, eax(flag));
        double(&mut e, 0x0089_1360, eax(1));
        start_log(&mut e);
        assert!(fn_009508b0(&mut e, player, 4, 2.5, 1));
        assert_eq!(
            calls_to(&e, 0x0089_1360),
            vec![vec![player.addr(), 4, 2.5f32.to_bits(), 1]]
        );
        e.set_global(GOD_MODE_BYTE, 1u8);
        assert!(!fn_009508b0(&mut e, player, 4, 2.5, 1));
        assert_eq!(calls_to(&e, 0x0089_1360).len(), 1);
    }

    #[test]
    fn the_forwarder_calls_the_callee_and_then_slot_22c() {
        let mut e = engine();
        let player = new_player(&mut e);
        let record = recording_slot(&mut e, player.addr(), 0x22c, eax(0));
        start_log(&mut e);
        fn_009508f0(&mut e, player, 9, 1.5);
        assert_eq!(
            calls_to(&e, 0x0089_8650),
            vec![vec![player.addr(), 9, 1.5f32.to_bits()]]
        );
        assert_eq!(record.borrow()[0], vec![player.addr(), 0]);
    }

    #[test]
    fn the_animation_update_scales_the_frame_time_and_runs_every_animation() {
        let mut e = engine();
        let player = new_player(&mut e);
        e.set_global(MINUS_ONE_FLOAT, -1.0f32);
        double(&mut e, 0x0084_d030, st0(0.5));
        double(&mut e, 0x009c_8cc0, st0(3.0));
        double(&mut e, 0x008b_70d0, eax(0x0600_0900));
        e.mem.set_u32(player.addr() + 0x690, 0x0600_0a00);
        let process = e.mem.alloc(16);
        let process_update = recording_slot(&mut e, process, 0x740, eax(0));
        e.set(player, PlayerCharacter::pCurrentProcess, Ptr::new(process));
        let own_update = recording_slot(&mut e, player.addr(), 0x178, eax(0));
        e.set_global(ANIMATION_UPDATE_DEPTH, 4u8);
        start_log(&mut e);
        fn_00950930(&mut e, player);
        let delta = 1.5f32.to_bits();
        let minus_one = (-1.0f32).to_bits();
        assert_eq!(
            calls_to(&e, 0x0049_1180),
            vec![
                vec![0x0600_0900, player.addr(), delta, minus_one],
                vec![0x0600_0a00, player.addr(), delta, minus_one]
            ]
        );
        assert_eq!(
            process_update.borrow()[0],
            vec![process, player.addr(), delta]
        );
        assert_eq!(own_update.borrow().len(), 1);
        assert_eq!(e.global::<u8>(ANIMATION_UPDATE_DEPTH), 4);
    }

    #[test]
    fn the_animation_update_skips_missing_animations_and_process() {
        let mut e = engine();
        let player = new_player(&mut e);
        slot(&mut e, player.addr(), 0x178, eax(0));
        start_log(&mut e);
        fn_00950930(&mut e, player);
        assert!(calls_to(&e, 0x0049_1180).is_empty());
    }

    #[test]
    fn the_animation_choice_prefers_the_inventory_then_the_first_person_one() {
        let mut e = engine();
        let player = new_player(&mut e);
        double(&mut e, 0x008b_70d0, eax(0x0600_0900));
        assert_eq!(fn_00950a10(&mut e, player), 0x0600_0900);
        e.mem.set_u32(player.addr() + 0x690, 0x0600_0a00);
        assert_eq!(fn_00950a10(&mut e, player), 0x0600_0a00);
        e.mem.set_u8(player.addr() + 0x64a, 1);
        assert_eq!(fn_00950a10(&mut e, player), 0x0600_0900);
        e.mem.set_u32(player.addr() + 0x6a0, 0x0600_0b00);
        assert_eq!(fn_00950a10(&mut e, player), 0x0600_0b00);
    }

    #[test]
    fn get_animation_picks_by_the_argument() {
        let mut e = engine();
        let player = new_player(&mut e);
        double(&mut e, 0x008b_70d0, eax(0x0600_0900));
        e.mem.set_u32(player.addr() + 0x690, 0x0600_0a00);
        assert_eq!(
            player_character_get_animation(&mut e, player, 0),
            0x0600_0900
        );
        assert_eq!(
            player_character_get_animation(&mut e, player, 1),
            0x0600_0a00
        );
    }

    #[test]
    fn the_biped_helpers_pick_the_first_person_biped() {
        let mut e = engine();
        let player = new_player(&mut e);
        double(&mut e, 0x005d_9f90, eax(0x0600_0c00));
        assert_eq!(fn_00950a90(&mut e, player), 0x0600_0c00);
        e.mem.set_u32(player.addr() + 0x68c, 0x0600_0d00);
        assert_eq!(fn_00950a90(&mut e, player), 0x0600_0d00);
        e.mem.set_u8(player.addr() + 0x64a, 1);
        assert_eq!(fn_00950a90(&mut e, player), 0x0600_0c00);
        assert_eq!(player_character_get_biped(&mut e, player, 1), 0x0600_0d00);
        assert_eq!(player_character_get_biped(&mut e, player, 0), 0x0600_0c00);
        // `00950ad0` asks 00524d10 and passes the opposite.
        double(&mut e, 0x0052_4d10, eax(1));
        assert_eq!(fn_00950ad0(&mut e, player), 0x0600_0c00);
        double(&mut e, 0x0052_4d10, eax(0));
        assert_eq!(fn_00950ad0(&mut e, player), 0x0600_0d00);
        assert!(player_character_is_1st_person_biped(
            &mut e,
            player,
            0x0600_0d00
        ));
        assert!(!player_character_is_1st_person_biped(
            &mut e,
            player,
            0x0600_0e00
        ));
        let other = new_player(&mut e);
        assert!(!player_character_is_1st_person_biped(&mut e, other, 0));
    }

    #[test]
    fn the_3d_helpers_pick_the_first_person_model_or_the_players_own() {
        let mut e = engine();
        pointer_reads(&mut e);
        let player = new_player(&mut e);
        double(&mut e, 0x0043_fcd0, eax(0x0600_0f00));
        assert_eq!(fn_00950b60(&mut e, player), 0x0600_0f00);
        assert_eq!(fn_00950bb0(&mut e, player, 0), 0x0600_0f00);
        assert_eq!(fn_00950bb0(&mut e, player, 1), 0);
        e.mem.set_u32(player.addr() + 0x694, 0x0600_1000);
        assert_eq!(fn_00950b60(&mut e, player), 0x0600_1000);
        assert_eq!(fn_00950bb0(&mut e, player, 1), 0x0600_1000);
        e.mem.set_u8(player.addr() + 0x64a, 1);
        assert_eq!(fn_00950b60(&mut e, player), 0x0600_0f00);
        double(&mut e, 0x0052_4d10, eax(1));
        assert_eq!(player_character_get_current_3d(&mut e, player), 0x0600_0f00);
        double(&mut e, 0x0052_4d10, eax(0));
        assert_eq!(player_character_get_current_3d(&mut e, player), 0x0600_1000);
    }

    #[test]
    fn the_inventory_clone_does_nothing_when_the_state_already_matches() {
        let mut e = engine();
        pointer_reads(&mut e);
        let player = new_player(&mut e);
        start_log(&mut e);
        // Asking to destroy while there is no model, or to create while there
        // is one: only the dereference of the pointer is made.
        player_character_clone_inventory_3d(&mut e, player, 0);
        e.mem.set_u32(player.addr() + 0x69c, 0x0600_1100);
        player_character_clone_inventory_3d(&mut e, player, 1);
        assert!(calls_to(&e, 0x0097_4c30).is_empty());
        assert!(calls_to(&e, 0x0066_b0d0).is_empty());
        assert_eq!(
            calls_to(&e, 0x0055_9450),
            vec![vec![player.addr() + 0x69c]; 4]
        );
    }

    #[test]
    fn the_inventory_model_is_destroyed_with_its_animation() {
        let mut e = engine();
        pointer_reads(&mut e);
        e.register(SMART_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            eax(0)
        });
        let player = new_player(&mut e);
        let model = object_with_slot(&mut e, 0x9c, eax(0x0600_1200));
        e.mem.set_u32(player.addr() + 0x69c, model);
        e.mem.set_u32(player.addr() + 0x6a0, 0x0600_1300);
        double(&mut e, 0x008d_3f70, eax(1));
        double(&mut e, 0x008d_3f80, eax(2));
        double(&mut e, 0x0065_3270, eax(0x0600_1400));
        let extra = e.mem.alloc(16);
        let destroyed = recording_slot(&mut e, extra, 0, eax(0));
        double(&mut e, 0x0082_0bf0, eax(extra));
        start_log(&mut e);
        player_character_clone_inventory_3d(&mut e, player, 0);
        assert_eq!(calls_to(&e, DELETE_ANIMATION), vec![vec![0x0600_1300, 1]]);
        assert_eq!(e.mem.u32(player.addr() + 0x6a0), 0);
        assert_eq!(
            calls_to(&e, 0x0066_3050),
            vec![vec![0x0600_1400, 1], vec![0x0600_1400, 1]]
        );
        assert_eq!(
            calls_to(&e, 0x0065_3270),
            vec![
                vec![CAST_TYPE_011D5BF8, 0x0600_1200],
                vec![CAST_TYPE_011D5BF8, 0x0600_1200]
            ]
        );
        assert_eq!(e.mem.u32(player.addr() + 0x69c), 0);
        assert_eq!(destroyed.borrow()[0], vec![extra, 1]);
        assert_eq!(calls_to(&e, 0x0092_9220), vec![vec![player.addr(), 0]]);
    }

    /// Everything the creating branch of `CloneInventory3D` needs: returns
    /// the player, the model (the clone) and the list of files.
    fn clone_scenario(e: &mut Engine, separator_found: bool) -> (Ptr<PlayerCharacter>, u32, u32) {
        pointer_reads(e);
        e.register(SMART_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            eax(0)
        });
        e.register(OPERATOR_NEW, |e, a| eax(e.mem.alloc(a[0].max(16))));
        let player = new_player(e);
        let model = e.mem.alloc(16);
        slot(e, model, 0xbc, eax(0));
        slot(e, model, 0x9c, eax(0));
        let clone = e.mem.alloc(16);
        slot(e, clone, 0xc, eax(model));
        double(e, 0x0043_fcd0, eax(0x0600_1500));
        double(e, 0x00a5_d2c0, eax(clone));
        double(e, 0x0048_f810, eax(0x0600_1600));
        double(e, 0x0044_7300, eax(0x0600_1700));
        double(e, 0x00af_e0d0, eax(1));
        if separator_found {
            e.register(0x0040_ab30, |_, a| eax(a[0] + 8));
        }
        // The process lists object answers the player's slot `0x1d0` too.
        slot(e, player.addr(), 0x1d0, eax(0x0600_1800));
        slot(e, player.addr(), 0x1f4, eax(0));
        e.set_global(INVENTORY_CLONE_FLAG, 5u8);
        for i in 0..12u32 {
            e.mem.set_u32(IDLE_NAME_TABLE + 4 * i, 0x0101_0000 + i);
        }
        (player, model, clone)
    }

    #[test]
    fn the_inventory_clone_builds_the_model_and_lists_the_idle_files() {
        let mut e = engine();
        let (player, model, _) = clone_scenario(&mut e, true);
        start_log(&mut e);
        player_character_clone_inventory_3d(&mut e, player, 1);
        assert_eq!(e.mem.u32(player.addr() + 0x69c), model);
        assert_eq!(e.mem.u32(player.addr() + 0x6a0), 0x0600_1600);
        // The flag byte is raised during the clone and restored at the end.
        assert_eq!(calls_to(&e, 0x0097_4420), vec![vec![PROCESS_LISTS, 0]]);
        assert_eq!(e.global::<u8>(INVENTORY_CLONE_FLAG), 5);
        // The clone is made of the player's own 3D with a cloning process.
        assert_eq!(calls_to(&e, 0x00a5_d2c0)[0][0], 0x0600_1500);
        // Every idle name gets a plain and a torch entry.
        assert_eq!(calls_to(&e, 0x0090_5820).len(), 22);
        assert!(calls_to(&e, 0x0090_5820)
            .iter()
            .all(|a| a[0] == 0x0600_1700));
        assert_eq!(
            calls_to(&e, 0x0048_ffd0),
            vec![vec![0x0600_1600, 0x0600_1700, model, player.addr(), 1]]
        );
        assert_eq!(calls_to(&e, 0x00af_e0d0).len(), 22);
        assert_eq!(calls_to(&e, 0x00af_e0d0)[0][2..], [0, 0, 0xffff_ffff]);
    }

    #[test]
    fn the_inventory_clone_stops_when_the_model_path_has_no_folder() {
        let mut e = engine();
        let (player, model, _) = clone_scenario(&mut e, false);
        start_log(&mut e);
        player_character_clone_inventory_3d(&mut e, player, 1);
        assert_eq!(e.mem.u32(player.addr() + 0x69c), model);
        assert!(calls_to(&e, 0x0044_7300).is_empty());
        assert!(calls_to(&e, 0x0048_ffd0).is_empty());
        // The flag byte was raised and stays so.
        assert_eq!(e.global::<u8>(INVENTORY_CLONE_FLAG), 1);
    }

    #[test]
    fn the_small_inventory_clone_accessors() {
        let mut e = engine();
        e.set_global(INVENTORY_CLONE_FLAG, 3u8);
        assert_eq!(fn_00951900(&mut e), 3);
        fn_00951910(&mut e, 7);
        assert_eq!(e.global::<u8>(INVENTORY_CLONE_FLAG), 7);
        e.set_global(NODE_NAME_011C623C, 0x1111u32);
        e.set_global(NODE_NAME_011C6274, 0x2222u32);
        assert_eq!(fn_00951920(&mut e), 0x1111);
        assert_eq!(fn_00951930(&mut e), 0x2222);
    }

    #[test]
    fn stripping_a_node_tree_removes_unwanted_controllers_and_recurses() {
        let mut e = engine();
        let node = 0x0600_2000;
        // Every node has two controllers: the first is dropped, the second kept.
        double(&mut e, 0x0043_b230, eax(0x0600_2300));
        e.register(0x004a_8a90, |_, a| {
            eax(if a[0] == 0x0600_2300 { 0x0600_2400 } else { 0 })
        });
        e.register(0x0043_b300, |_, a| eax(u32::from(a[1] == 0x0600_2400)));
        // Only the top node has a child; the child answers `AsNode` with a
        // grandchild node without children.
        e.register(0x0043_b480, |_, a| eax(u32::from(a[0] == 0x0600_2000)));
        let child = e.mem.alloc(16);
        slot(&mut e, child, 0xc, eax(0x0600_2200));
        e.register_double(0x0043_b4a0, move |_, _| eax(child));
        start_log(&mut e);
        fn_00951940(&mut e, Ptr::NULL, node);
        assert_eq!(
            calls_to(&e, 0x00a5_c480),
            vec![vec![node, 0x0600_2300], vec![0x0600_2200, 0x0600_2300]]
        );
        assert_eq!(
            calls_to(&e, 0x0062_bc90),
            vec![vec![node, 0], vec![0x0600_2200, 0]]
        );
        assert_eq!(calls_to(&e, 0x0045_0f90), vec![vec![child, 0]]);
        start_log(&mut e);
        fn_00951940(&mut e, Ptr::NULL, 0);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    const OWN_3D: u32 = 0x0600_3100;
    const FIRST_PERSON_3D: u32 = 0x0600_3200;

    /// A player with both models and the virtual slots `00951a10` uses.
    fn switch_scenario(e: &mut Engine) -> Ptr<PlayerCharacter> {
        pointer_reads(e);
        let player = new_player(e);
        slot(e, player.addr(), 0x22c, eax(0));
        slot(e, player.addr(), 0x1f4, eax(0x77));
        slot(e, player.addr(), 0x2bc, st0(0.25));
        slot(e, player.addr(), 0x210, eax(0));
        slot(e, player.addr(), 0x1d0, eax(0x0600_3300));
        e.mem.set_u32(player.addr() + 0x694, FIRST_PERSON_3D);
        double(e, 0x0043_fcd0, eax(OWN_3D));
        double(e, 0x0056_fac0, eax(0));
        e.set_global(INVENTORY_ALPHA, 0.0f32);
        let process = e.mem.alloc(16);
        slot(e, process, 0x580, eax(0));
        double(e, 0x008d_8520, eax(process));
        player
    }

    #[test]
    fn showing_the_players_own_3d_culls_the_first_person_model() {
        let mut e = engine();
        let player = switch_scenario(&mut e);
        double(&mut e, 0x00ad_8ce0, eax(1));
        let process = e.mem.alloc(16);
        let refresh = recording_slot(&mut e, process, 0x580, eax(0));
        double(&mut e, 0x008d_8520, eax(process));
        start_log(&mut e);
        fn_00951a10(&mut e, player, 0);
        assert_eq!(
            calls_to(&e, 0x0045_0f90),
            vec![vec![OWN_3D, 0], vec![FIRST_PERSON_3D, 1]]
        );
        assert_eq!(e.mem.u8(player.addr() + 0x64b), 1);
        // Both weapon sounds are switched off.
        let sound = calls_to(&e, 0x00ad_8fa0);
        assert_eq!(sound.len(), 2);
        assert!(sound.iter().all(|a| a[1] == 0));
        assert_eq!(
            calls_to(&e, 0x008b_0bd0),
            vec![vec![player.addr(), 0x0600_3300]]
        );
        assert_eq!(refresh.borrow()[0], vec![process, 1, 1, 0]);
        assert!(calls_to(&e, 0x0044_0460).is_empty());
    }

    #[test]
    fn showing_the_first_person_model_places_it_and_plays_the_animation() {
        let mut e = engine();
        let player = switch_scenario(&mut e);
        double(&mut e, 0x00ad_8ce0, eax(1));
        double(&mut e, 0x008b_70d0, eax(0x0600_3400));
        double(&mut e, 0x0043_b4a0, eax(0x0600_3500));
        let process = e.mem.alloc(16);
        slot(&mut e, process, 0x580, eax(0));
        double(&mut e, 0x008d_8520, eax(process));
        start_log(&mut e);
        fn_00951a10(&mut e, player, 1);
        assert_eq!(
            calls_to(&e, 0x0045_0f90),
            vec![vec![OWN_3D, 1], vec![FIRST_PERSON_3D, 0]]
        );
        assert_eq!(e.mem.u8(player.addr() + 0x64b), 0);
        assert_eq!(calls_to(&e, 0x0044_0460), vec![vec![OWN_3D, 0x77]]);
        assert_eq!(calls_to(&e, 0x004a_0c90)[0][1], 0.25f32.to_bits());
        assert_eq!(e.global::<f32>(INVENTORY_ALPHA), 1.0);
        assert_eq!(
            calls_to(&e, 0x0049_3900),
            vec![vec![0x0600_3400, player.addr()]]
        );
        assert!(calls_to(&e, 0x00a5_9c60).is_empty());
        assert!(calls_to(&e, 0x0043_d410).is_empty());
        // The sounds are switched on, and the lighting follows the first child.
        assert!(calls_to(&e, 0x00ad_8fa0).iter().all(|a| a[1] == 1));
        assert_eq!(
            calls_to(&e, 0x008b_0bd0),
            vec![vec![player.addr(), 0x0600_3500]]
        );
        // Without an animation the model gets a zero vector instead.
        double(&mut e, 0x008b_70d0, eax(0));
        start_log(&mut e);
        fn_00951a10(&mut e, player, 1);
        assert!(calls_to(&e, 0x0049_3900).is_empty());
        assert_eq!(calls_to(&e, 0x00a5_9c60).len(), 1);
    }

    #[test]
    fn the_switch_does_nothing_without_both_models_or_when_the_temporary_camera_holds_it() {
        let mut e = engine();
        let player = switch_scenario(&mut e);
        e.mem.set_u32(player.addr() + 0x694, 0);
        start_log(&mut e);
        fn_00951a10(&mut e, player, 0);
        assert!(calls_to(&e, 0x0045_0f90).is_empty());
        e.mem.set_u32(player.addr() + 0x694, FIRST_PERSON_3D);
        e.mem.set_u8(player.addr() + 0x64d, 1);
        fn_00951a10(&mut e, player, 1);
        assert!(calls_to(&e, 0x0045_0f90).is_empty());
        // The same when the player cannot use the camera (slot 0x22c).
        e.mem.set_u8(player.addr() + 0x64d, 0);
        slot(&mut e, player.addr(), 0x22c, eax(1));
        fn_00951a10(&mut e, player, 1);
        assert!(calls_to(&e, 0x0045_0f90).is_empty());
    }

    #[test]
    fn a_hidden_model_state_mismatch_stops_the_switch() {
        let mut e = engine();
        let player = switch_scenario(&mut e);
        // The player's own 3D is flagged, the first-person one is not.
        e.register(0x0045_6610, |_, a| eax(u32::from(a[0] == OWN_3D)));
        e.mem.set_u8(player.addr() + 0x64b, 1);
        start_log(&mut e);
        fn_00951a10(&mut e, player, 1);
        assert_eq!(e.mem.u8(player.addr() + 0x64b), 0);
        assert!(calls_to(&e, 0x0045_0f90).is_empty());
        // Switching the other way is allowed with the own 3D flagged.
        fn_00951a10(&mut e, player, 0);
        assert_eq!(calls_to(&e, 0x0045_0f90).len(), 2);
        // With only the first-person model flagged it stops.
        e.register(0x0045_6610, |_, a| eax(u32::from(a[0] == FIRST_PERSON_3D)));
        start_log(&mut e);
        fn_00951a10(&mut e, player, 0);
        assert!(calls_to(&e, 0x0045_0f90).is_empty());
    }

    #[test]
    fn the_scope_is_shown_for_a_sighted_weapon_and_the_first_person_model_stays_culled() {
        let mut e = engine();
        let player = switch_scenario(&mut e);
        e.mem.set_u8(player.addr() + 0x64f, 1);
        double(&mut e, 0x008b_bc10, eax(1));
        let process = e.mem.alloc(16);
        slot(&mut e, process, 0x148, eax(0x0600_3600));
        slot(&mut e, process, 0x580, eax(0));
        e.set(player, PlayerCharacter::pCurrentProcess, Ptr::new(process));
        double(&mut e, 0x0048_cee0, eax(1));
        double(&mut e, 0x004a_d030, eax(0));
        double(&mut e, 0x008d_8520, eax(process));
        start_log(&mut e);
        fn_00951a10(&mut e, player, 1);
        assert_eq!(calls_to(&e, 0x0070_9c40), vec![vec![1]]);
        // The first-person model is not touched by the else branch.
        assert_eq!(calls_to(&e, 0x0045_0f90), vec![vec![OWN_3D, 1]]);
        // A scoped weapon with the mod effect missing shows no scope.
        double(&mut e, 0x004a_d030, eax(1));
        double(&mut e, 0x004b_d8d0, eax(0));
        start_log(&mut e);
        fn_00951a10(&mut e, player, 1);
        assert!(calls_to(&e, 0x0070_9c40).is_empty());
        assert_eq!(calls_to(&e, 0x004b_d8d0)[0][1], 0xe);
        double(&mut e, 0x004b_d8d0, eax(1));
        start_log(&mut e);
        fn_00951a10(&mut e, player, 1);
        assert_eq!(calls_to(&e, 0x0070_9c40), vec![vec![1]]);
    }

    #[test]
    fn the_animation_sequences_are_copied_to_the_other_animation() {
        let mut e = engine();
        let player = switch_scenario(&mut e);
        double(&mut e, 0x008c_4560, eax(1));
        double(&mut e, 0x008b_70d0, eax(0x0600_3700));
        e.mem.set_u32(player.addr() + 0x690, 0x0600_3800);
        double(&mut e, 0x0053_7bd0, eax(0));
        // Both animations lead to the same sequence list of one entry; the
        // list's slot 0xec stores a `NiPointer` value, slot 0xdc receives it.
        let sequences = e.mem.alloc(16);
        let table = e.mem.alloc(0x800);
        e.mem.set_u32(sequences, table);
        e.register(0x0300_0100, |e, a| {
            e.mem.set_u32(a[2], 0x0600_3900);
            eax(0)
        });
        e.mem.set_u32(table + 0xec, 0x0300_0100);
        let received = recording_slot(&mut e, sequences, 0xdc, eax(0));
        let found = e.mem.alloc(16);
        slot(&mut e, found, 0xc, eax(sequences));
        let manager = e.mem.alloc(16);
        slot(&mut e, manager, 0x8c, eax(found));
        double(&mut e, 0x0053_7bd0, eax(manager));
        e.register(0x0043_b480, |_, a| eax(u32::from(a[0] != 0)));
        e.register(0x0052_aa80, |_, a| eax(u32::from(a[0] != 0)));
        start_log(&mut e);
        fn_00951a10(&mut e, player, 0);
        assert_eq!(received.borrow().len(), 1);
        assert_eq!(received.borrow()[0][1..], [0x0600_3900, 1]);
        // The first-person animation is the source when the flag is 0.
        assert_eq!(calls_to(&e, 0x0049_6940).len(), 2);
    }

    /// Sets the entry of the group type table.
    fn group_entry(e: &mut Engine, group_type: u32, flag: u8, word: u32) {
        e.mem
            .set_u8(GROUP_TYPE_TABLE_FLAG + group_type * 0x24, flag);
        e.mem
            .set_u32(GROUP_TYPE_TABLE_WORD + group_type * 0x24, word);
    }

    #[test]
    fn playing_a_group_ignores_groups_without_a_type() {
        let mut e = engine();
        let player = new_player(&mut e);
        double(&mut e, 0x005f_2440, eax(0xff));
        start_log(&mut e);
        fn_009520f0(&mut e, player, 0x8005, 1);
        // The high bit is dropped before the lookup.
        assert_eq!(calls_to(&e, 0x005f_2440), vec![vec![5]]);
        assert!(calls_to(&e, 0x0049_4740).is_empty());
    }

    #[test]
    fn playing_a_group_prefers_a_synchronised_sequence_then_the_loaded_group() {
        let mut e = engine();
        let player = new_player(&mut e);
        double(&mut e, 0x005f_2440, eax(3));
        double(&mut e, 0x008b_70d0, eax(0x0600_4000));
        e.mem.set_u32(player.addr() + 0x690, 0x0600_4100);
        group_entry(&mut e, 3, 1, 0x77);
        double(&mut e, 0x0049_1040, eax(0x0600_4200));
        double(&mut e, 0x0049_5da0, eax(1));
        start_log(&mut e);
        fn_009520f0(&mut e, player, 9, 0x55);
        assert_eq!(calls_to(&e, 0x0049_1040), vec![vec![0x0600_4000, 0x77]]);
        assert_eq!(
            calls_to(&e, 0x0049_5da0),
            vec![vec![0x0600_4100, 0x0600_4200, 9]]
        );
        assert!(calls_to(&e, 0x0049_4740).is_empty());
        // Not synchronised, the group is loaded: it is played.
        double(&mut e, 0x0049_5da0, eax(0));
        double(&mut e, 0x0049_4710, eax(1));
        fn_009520f0(&mut e, player, 9, 0x55);
        assert_eq!(
            calls_to(&e, 0x0049_4740),
            vec![vec![0x0600_4100, 9, 0x55, 0xffff_ffff, 0xffff_ffff]]
        );
    }

    #[test]
    fn playing_a_group_falls_back_to_the_iron_sights_variant_or_the_selected_group() {
        let mut e = engine();
        let player = new_player(&mut e);
        double(&mut e, 0x005f_2440, eax(3));
        e.mem.set_u32(player.addr() + 0x690, 0x0600_4100);
        // Iron-sights action whose lower group is loaded.
        double(&mut e, 0x005f_2720, eax(1));
        e.register(0x0049_4710, |_, a| eax(u32::from(a[1] == 6)));
        start_log(&mut e);
        fn_009520f0(&mut e, player, 9, 0x55);
        assert_eq!(
            calls_to(&e, 0x0049_4740),
            vec![vec![0x0600_4100, 6, 0x55, 0xffff_ffff, 0xffff_ffff]]
        );
        // Nothing is loaded: the player's own selection (slot 0x00897910) is
        // played when its type is the same.
        double(&mut e, 0x005f_2720, eax(0));
        double(&mut e, 0x0049_4710, eax(0));
        double(&mut e, 0x008a_1710, eax(0));
        double(&mut e, 0x0089_7910, eax(0x1234));
        start_log(&mut e);
        fn_009520f0(&mut e, player, 9, 0x55);
        assert_eq!(
            calls_to(&e, 0x0089_7910),
            vec![vec![player.addr(), 3, 0, 1, 0x0600_4100]]
        );
        assert_eq!(
            calls_to(&e, 0x0049_4740),
            vec![vec![0x0600_4100, 0x1234, 0x55, 0xffff_ffff, 0xffff_ffff]]
        );
        // A different type is refused unless it is the iron-sights variant.
        e.register(0x005f_2440, |_, a| eax(if a[0] == 0x1234 { 8 } else { 3 }));
        start_log(&mut e);
        fn_009520f0(&mut e, player, 9, 0x55);
        assert!(calls_to(&e, 0x0049_4740).is_empty());
        double(&mut e, 0x005f_2750, eax(1));
        e.register(0x005f_2440, |_, a| eax(if a[0] == 0x1234 { 0 } else { 3 }));
        fn_009520f0(&mut e, player, 9, 0x55);
        assert_eq!(calls_to(&e, 0x0049_4740).len(), 1);
    }

    /// The doubles `00952290` needs, with the zoom settings in the exe's data.
    fn zoom_scenario(e: &mut Engine) -> Ptr<PlayerCharacter> {
        pointer_reads(e);
        e.register(SETTING_FLOAT_POINTER, |_, a| eax(a[0] + 4));
        for (setting, value) in [
            (ZOOM_TARGET_SETTING, 0.5f32),
            (ZOOM_IN_SECONDS_SETTING, 2.0),
            (ZOOM_OUT_SECONDS_SETTING, 1.0),
        ] {
            e.mem.set_f32(setting + 4, value);
        }
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        e.set_global(MINUS_ONE_DOUBLE, -1.0f64);
        e.set_global(MINUS_ONE_FLOAT, -1.0f32);
        let player = new_player(e);
        slot(e, player.addr(), 0x2bc, st0(0.5));
        e.mem.set_u32(player.addr() + 0x694, 0x0600_4300);
        double(e, 0x008b_70d0, eax(0x0600_4400));
        double(e, 0x0043_01b0, eax(0));
        double(e, 0x005f_2440, eax(0));
        double(e, 0x008a_7570, eax(0));
        double(e, 0x0084_d030, st0(0.5));
        double(e, 0x0093_1d70, st0(0.3));
        double(e, 0x004f_6e40, st0(0.8));
        let product = word_block(e, &[0; 9]);
        double(e, 0x0043_f8d0, eax(product));
        let rotated = word_block(e, &[0, 0, 0]);
        double(e, 0x004b_4500, eax(rotated));
        double(e, 0x004b_6190, st0(0.5));
        player
    }

    #[test]
    fn the_zoom_moves_towards_its_target_and_the_model_matrix_is_set() {
        let mut e = engine();
        let player = zoom_scenario(&mut e);
        // A group type in 0x18..=0xa8 makes the target 1.0 (zoom out
        // duration 1.0): 0.5 / 1.0 * (1.0 - 0.25).
        double(&mut e, 0x005f_2440, eax(0x20));
        e.set_global(ZOOM_FACTOR, 0.25f32);
        start_log(&mut e);
        fn_00952290(&mut e, player);
        assert_eq!(e.global::<f32>(ZOOM_FACTOR), 0.625);
        // The look angle is (1 - target) * acos, the target being the new
        // zoom factor.
        assert_eq!(e.global::<f32>(LOOK_ANGLE), (1.0 - 0.625f32) * 0.8);
        assert_eq!(calls_to(&e, 0x0043_fa80).len(), 1);
        // The matrix goes to the first-person model.
        assert_eq!(calls_to(&e, 0x0043_fa80)[0][0], 0x0600_4300);
    }

    #[test]
    fn the_zoom_is_clamped_to_the_target() {
        let mut e = engine();
        let player = zoom_scenario(&mut e);
        // Nothing in the animation: the target is the setting 0.5 and the
        // factor is far below it with a long frame time.
        double(&mut e, 0x0084_d030, st0(8.0));
        e.set_global(ZOOM_FACTOR, 0.25f32);
        fn_00952290(&mut e, player);
        assert_eq!(e.global::<f32>(ZOOM_FACTOR), 0.5);
        // Moving down from above the target clamps from below.
        e.set_global(ZOOM_FACTOR, 1.0f32);
        fn_00952290(&mut e, player);
        assert_eq!(e.global::<f32>(ZOOM_FACTOR), 0.5);
        // Equal to the target: no step is taken (the frame time is not asked).
        start_log(&mut e);
        fn_00952290(&mut e, player);
        assert!(calls_to(&e, 0x0084_d030).is_empty());
    }

    #[test]
    fn the_zoom_target_is_one_for_the_pipboy_and_the_iron_sights() {
        let mut e = engine();
        let player = zoom_scenario(&mut e);
        double(&mut e, 0x0084_d030, st0(0.0));
        e.set_global(ZOOM_FACTOR, 0.5f32);
        // The Pipboy makes the target 1.0, so the zoom-out duration is read
        // (the factor stays: the frame time is 0).
        double(&mut e, 0x0096_7ae0, eax(1));
        start_log(&mut e);
        fn_00952290(&mut e, player);
        assert_eq!(e.global::<f32>(ZOOM_FACTOR), 0.5);
        assert!(calls_to(&e, SETTING_FLOAT_POINTER).contains(&vec![ZOOM_OUT_SECONDS_SETTING]));
        // The Pipboy is not asked while the animation action is 4.
        double(&mut e, 0x008a_7570, eax(4));
        double(&mut e, 0x008b_bc10, eax(1));
        start_log(&mut e);
        fn_00952290(&mut e, player);
        assert!(calls_to(&e, 0x0096_7ae0).is_empty());
        assert_eq!(calls_to(&e, 0x008b_bc10).len(), 1);
    }

    #[test]
    fn the_zoom_does_nothing_without_an_animation() {
        let mut e = engine();
        let player = zoom_scenario(&mut e);
        double(&mut e, 0x008b_70d0, eax(0));
        start_log(&mut e);
        fn_00952290(&mut e, player);
        assert!(calls_to(&e, 0x0043_fa80).is_empty());
    }

    #[test]
    fn the_look_angle_flips_when_the_looking_vector_points_down() {
        let mut e = engine();
        let player = zoom_scenario(&mut e);
        e.set_global(ZOOM_FACTOR, 0.5f32);
        let down = word_block(&mut e, &[0, 0, 0]);
        e.mem.set_f32(down + 8, -1.0);
        double(&mut e, 0x004b_4500, eax(down));
        fn_00952290(&mut e, player);
        assert_eq!(e.global::<f32>(LOOK_ANGLE), -(0.5f32 * 0.8));
        // A dot product outside [-1, 1] is clamped before the call.
        double(&mut e, 0x004b_6190, st0(3.0));
        start_log(&mut e);
        fn_00952290(&mut e, player);
        assert_eq!(calls_to(&e, 0x004f_6e40), vec![vec![1.0f32.to_bits()]]);
        double(&mut e, 0x004b_6190, st0(-3.0));
        start_log(&mut e);
        fn_00952290(&mut e, player);
        assert_eq!(calls_to(&e, 0x004f_6e40), vec![vec![(-1.0f32).to_bits()]]);
    }

    #[test]
    fn the_god_mode_bytes_and_the_setting() {
        let mut e = engine();
        let setting_value = word_block(&mut e, &[0]);
        double(&mut e, 0x0040_8d60, eax(setting_value));
        start_log(&mut e);
        assert_eq!(player_character_is_god_mode(&mut e), 0);
        assert_eq!(player_character_is_demigod_mode(&mut e), 0);
        player_character_set_god_mode(&mut e, 1);
        player_character_set_demigod_mode(&mut e, 1);
        assert_eq!(e.global::<u8>(GOD_MODE_BYTE), 1);
        assert_eq!(e.global::<u8>(DEMIGOD_MODE_BYTE), 1);
        assert_eq!(player_character_is_god_mode(&mut e), 1);
        assert_eq!(fn_00952720(&mut e, Ptr::NULL), 1);
        player_character_set_god_mode(&mut e, 0);
        player_character_set_demigod_mode(&mut e, 0);
        assert_eq!(fn_00952720(&mut e, Ptr::NULL), 0);
        // The setting turns both on.
        e.mem.set_u8(setting_value, 1);
        assert_eq!(player_character_is_god_mode(&mut e), 1);
        assert_eq!(player_character_is_demigod_mode(&mut e), 1);
        assert!(calls_to(&e, 0x0040_8d60)
            .iter()
            .all(|a| a == &vec![GOD_MODE_SETTING]));
    }

    #[test]
    fn actor_value_changes_are_refused_for_a_protected_player() {
        let mut e = engine();
        let player = new_player(&mut e);
        let target = player.addr() + MAGIC_TARGET;
        // The sub-object's slot 0x10 says no: everything is allowed.
        slot(&mut e, target, 0x10, eax(0));
        assert!(fn_00952730(&mut e, player, 0x10, -5.0));
        // It says yes: health and the limb values cannot be lost, and the
        // value 0x36 cannot be gained.
        slot(&mut e, target, 0x10, eax(1));
        assert!(!fn_00952730(&mut e, player, 0x10, -5.0));
        assert!(fn_00952730(&mut e, player, 0x10, 5.0));
        assert!(fn_00952730(&mut e, player, 0x18, -5.0));
        assert!(!fn_00952730(&mut e, player, 0x19, -5.0));
        assert!(!fn_00952730(&mut e, player, 0x1f, -5.0));
        assert!(fn_00952730(&mut e, player, 0x20, -5.0));
        assert!(!fn_00952730(&mut e, player, 0x36, 1.0));
        assert!(fn_00952730(&mut e, player, 0x36, -1.0));
        assert!(fn_00952730(&mut e, player, 0x36, 0.0));
    }

    /// Records the item stored in the word the list append `005ae3d0` is given.
    fn record_appends(e: &mut Engine) -> Rc<RefCell<Vec<(u32, u32)>>> {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let record = seen.clone();
        e.register_double(0x005a_e3d0, move |e, words| {
            record.borrow_mut().push((words[0], e.mem.u32(words[1])));
            eax(0)
        });
        seen
    }

    #[test]
    fn a_topic_is_added_once_and_the_list_refreshed_on_request() {
        let mut e = engine();
        list_helpers(&mut e);
        let player = new_player(&mut e);
        let second = node(&mut e, 0xb0b, 0);
        let head = node(&mut e, 0xa0a, second);
        double(&mut e, 0x0046_4e30, eax(head));
        let appended = record_appends(&mut e);
        start_log(&mut e);
        assert!(!fn_00952830(&mut e, player, 0, 1, 1));
        assert!(!fn_00952830(&mut e, player, 0xb0b, 1, 1));
        assert!(appended.borrow().is_empty());
        assert!(calls_to(&e, 0x0061_a5a0).is_empty());
        assert!(fn_00952830(&mut e, player, 0xc0c, 0, 1));
        assert_eq!(*appended.borrow(), vec![(player.addr() + 0x6a8, 0xc0c)]);
        assert!(calls_to(&e, 0x0061_a5a0).is_empty());
        assert!(fn_00952830(&mut e, player, 0xd0d, 1, 0));
        assert_eq!(calls_to(&e, 0x0061_a5a0), vec![vec![1, head]]);
    }

    #[test]
    fn a_list_of_topics_is_added_with_a_single_refresh() {
        let mut e = engine();
        list_helpers(&mut e);
        let player = new_player(&mut e);
        double(&mut e, 0x0046_4e30, eax(0));
        let appended = record_appends(&mut e);
        let second = node(&mut e, 0xb0b, 0);
        let first = node(&mut e, 0xa0a, second);
        start_log(&mut e);
        fn_009527b0(&mut e, player, first);
        assert_eq!(
            *appended.borrow(),
            vec![
                (player.addr() + 0x6a8, 0xa0a),
                (player.addr() + 0x6a8, 0xb0b)
            ]
        );
        assert_eq!(calls_to(&e, 0x0061_a5a0), vec![vec![1, 0]]);
        // A list whose topics are all known refreshes nothing.
        let known = node(&mut e, 0xa0a, 0);
        double(&mut e, 0x0046_4e30, eax(known));
        start_log(&mut e);
        fn_009527b0(&mut e, player, known);
        assert!(calls_to(&e, 0x0061_a5a0).is_empty());
        fn_009527b0(&mut e, player, 0);
    }

    #[test]
    fn a_quest_stage_item_is_logged_and_may_play_the_menu_sound() {
        let mut e = engine();
        list_helpers(&mut e);
        let player = new_player(&mut e);
        let appended = record_appends(&mut e);
        assert!(!player_character_add_quest_stage_item(&mut e, player, 0));
        // Not in the log; scripts are not processed: only appended.
        let log = node(&mut e, 0xa0a, 0);
        double(&mut e, 0x0060_da90, eax(log));
        double(&mut e, 0x005a_c740, eax(0));
        start_log(&mut e);
        assert!(player_character_add_quest_stage_item(&mut e, player, 0xb0b));
        assert_eq!(*appended.borrow(), vec![(player.addr() + 0x6b0, 0xb0b)]);
        assert!(calls_to(&e, 0x0070_6f30).is_empty());
        // Already in the log: refused when the quest says so.
        double(&mut e, 0x0060_d5e0, eax(0));
        assert!(!player_character_add_quest_stage_item(
            &mut e, player, 0xa0a
        ));
        assert_eq!(appended.borrow().len(), 1);
        double(&mut e, 0x0060_d5e0, eax(1));
        assert!(player_character_add_quest_stage_item(&mut e, player, 0xa0a));
        assert_eq!(appended.borrow().len(), 2);
    }

    #[test]
    fn a_new_stage_of_the_players_quest_plays_the_sound_and_selects_the_parent_world() {
        let mut e = engine();
        list_helpers(&mut e);
        let player = new_player(&mut e);
        e.set_global(PLAYER_POINTER, 0x0600_5000u32);
        double(&mut e, 0x0060_da90, eax(0));
        record_appends(&mut e);
        double(&mut e, 0x005a_c740, eax(1));
        double(&mut e, 0x0060_fd70, eax(1));
        double(&mut e, 0x005e_3fa0, eax(0x77));
        double(&mut e, 0x005c_bb50, eax(0x77));
        double(&mut e, 0x004f_d380, eax(0x0600_5100));
        start_log(&mut e);
        assert!(player_character_add_quest_stage_item(&mut e, player, 0xb0b));
        assert_eq!(calls_to(&e, 0x0070_6f30), vec![vec![9]]);
        assert_eq!(calls_to(&e, 0x005c_bb50), vec![vec![0x0600_5000]]);
        assert_eq!(calls_to(&e, 0x0060_c9c0), vec![vec![0x0600_5100, 1]]);
        // A different quest, or no parent world, stops after the sound.
        double(&mut e, 0x005c_bb50, eax(0x78));
        start_log(&mut e);
        player_character_add_quest_stage_item(&mut e, player, 0xb0b);
        assert!(calls_to(&e, 0x0060_c9c0).is_empty());
        double(&mut e, 0x005c_bb50, eax(0x77));
        double(&mut e, 0x004f_d380, eax(0));
        player_character_add_quest_stage_item(&mut e, player, 0xb0b);
        assert!(calls_to(&e, 0x0060_c9c0).is_empty());
        // Items the script system does not want show nothing.
        double(&mut e, 0x0060_fd70, eax(0));
        start_log(&mut e);
        assert!(player_character_add_quest_stage_item(&mut e, player, 0xb0b));
        assert!(calls_to(&e, 0x0070_6f30).is_empty());
    }

    #[test]
    fn the_active_quest_is_stored_and_clearing_it_clears_the_targets() {
        let mut e = engine();
        let player = new_player(&mut e);
        start_log(&mut e);
        player_character_set_active_quest(&mut e, player, 0x55);
        assert_eq!(e.get(player, PlayerCharacter::pActiveQuest).addr(), 0x55);
        assert!(calls_to(&e, LIST_CLEAR).is_empty());
        player_character_set_active_quest(&mut e, player, 0x55);
        player_character_set_active_quest(&mut e, player, 0);
        assert_eq!(e.get(player, PlayerCharacter::pActiveQuest).addr(), 0);
        assert_eq!(calls_to(&e, LIST_CLEAR), vec![vec![player.addr() + 0x6c4]]);
        // Clearing again changes nothing.
        player_character_set_active_quest(&mut e, player, 0);
        assert_eq!(calls_to(&e, LIST_CLEAR).len(), 1);
    }

    #[test]
    fn a_quest_target_is_added_when_its_quest_has_objectives_and_it_is_new() {
        let mut e = engine();
        list_helpers(&mut e);
        let player = new_player(&mut e);
        let head = node(&mut e, 0xa0a, 0);
        double(&mut e, 0x0079_bc30, eax(head));
        double(&mut e, 0x0044_edb0, eax(0x77));
        double(&mut e, 0x005d_43e0, eax(0x0600_5200));
        double(&mut e, LIST_IS_EMPTY, eax(0));
        let appended = record_appends(&mut e);
        start_log(&mut e);
        fn_00952a20(&mut e, player, 0xb0b);
        assert_eq!(*appended.borrow(), vec![(head, 0xb0b)]);
        assert!(calls_to(&e, 0x005e_c500).is_empty());
        // Already present: not added again.
        fn_00952a20(&mut e, player, 0xa0a);
        assert_eq!(appended.borrow().len(), 1);
        // An empty objective list, or none at all, adds nothing.
        double(&mut e, LIST_IS_EMPTY, eax(1));
        fn_00952a20(&mut e, player, 0xc0c);
        double(&mut e, 0x005d_43e0, eax(0));
        fn_00952a20(&mut e, player, 0xc0c);
        assert_eq!(appended.borrow().len(), 1);
        // The active quest's target is handed on.
        e.set(player, PlayerCharacter::pActiveQuest, Ptr::new(0x77));
        fn_00952a20(&mut e, player, 0xc0c);
        assert_eq!(
            calls_to(&e, 0x005e_c500),
            vec![vec![0xc0c, player.addr() + 0x6c4]]
        );
    }

    // ----- third session: quest targets, map marker, focus, pick-up, save and load -----

    /// Float settings: the getter answers the address of the setting itself,
    /// which then holds the float.
    fn float_settings(e: &mut Engine) {
        e.register(SETTING_FLOAT_POINTER, |_, a| eax(a[0]));
    }

    /// The save version `008df040` answers.
    fn version(e: &mut Engine, value: u32) {
        double(e, 0x008d_f040, eax(value));
    }

    #[test]
    fn the_first_quest_entry_of_the_active_quest_is_found() {
        let mut e = engine();
        list_helpers(&mut e);
        let player = new_player(&mut e);
        e.set(player, PlayerCharacter::pActiveQuest, Ptr::new(7));
        let second = node(&mut e, 0xb0b, 0);
        e.mem.set_u32(player.addr() + 0x6bc, 0xa0a);
        e.mem.set_u32(player.addr() + 0x6c0, second);
        e.register(0x0044_edb0, |_, a| eax(if a[0] == 0xb0b { 7 } else { 8 }));
        double(&mut e, 0x007a_f430, eax(1));
        assert_eq!(fn_00952b30(&mut e, player), 0xb0b);
        double(&mut e, 0x007a_f430, eax(2));
        assert_eq!(fn_00952b30(&mut e, player), 0);
    }

    #[test]
    fn the_target_list_is_rebuilt_only_when_stale() {
        let mut e = engine();
        let player = new_player(&mut e);
        let p = player.addr();
        assert_eq!(player_character_get_current_target_list(&mut e, player), 0);
        e.set(player, PlayerCharacter::pActiveQuest, Ptr::new(7));
        double(&mut e, 0x0060_efd0, eax(1));
        start_log(&mut e);
        assert_eq!(
            player_character_get_current_target_list(&mut e, player),
            p + 0x6c4
        );
        assert!(calls_to(&e, 0x0060_f110).is_empty());
        e.mem.set_u8(p + 0x206, 1);
        player_character_get_current_target_list(&mut e, player);
        assert_eq!(
            calls_to(&e, 0x0060_f110),
            vec![vec![7, p + 0x6c4, p + 0x6bc]]
        );
        assert_eq!(e.mem.u8(p + 0x206), 0);
        double(&mut e, 0x0060_efd0, eax(0));
        player_character_get_current_target_list(&mut e, player);
        assert_eq!(calls_to(&e, 0x0060_f110).len(), 2);
    }

    #[test]
    fn a_quest_target_update_marks_the_list_for_a_target_reference_or_the_player() {
        let mut e = engine();
        list_helpers(&mut e);
        let player = new_player(&mut e);
        let p = player.addr();
        e.set_global(PLAYER_POINTER, 0x0600_5000u32);
        player_character_check_for_quest_target_update(&mut e, player, 0x77);
        assert_eq!(e.mem.u8(p + 0x206), 0);
        // A quest with one target whose reference is the argument.
        e.set(player, PlayerCharacter::pActiveQuest, Ptr::new(7));
        double(&mut e, 0x0060_efd0, eax(1));
        e.mem.set_u32(p + 0x6c4, 0x55);
        double(&mut e, 0x0061_01b0, eax(0x77));
        player_character_check_for_quest_target_update(&mut e, player, 0x77);
        assert_eq!(e.mem.u8(p + 0x206), 1);
        // A target holding an inventory the check accepts.
        e.mem.set_u8(p + 0x206, 0);
        double(&mut e, 0x0061_01b0, eax(0x78));
        double(&mut e, 0x0055_d310, eax(0x99));
        double(&mut e, 0x004b_f220, eax(0xaa));
        double(&mut e, 0x004c_fe20, eax(1));
        player_character_check_for_quest_target_update(&mut e, player, 0x77);
        assert_eq!(e.mem.u8(p + 0x206), 1);
        // The player: the path to the marker target is rebuilt.
        e.mem.set_u8(p + 0x206, 0);
        double(&mut e, 0x004c_fe20, eax(0));
        e.mem.set_u32(p + 0x6f4, 0x31);
        start_log(&mut e);
        player_character_check_for_quest_target_update(&mut e, player, 0x0600_5000);
        // The list is marked stale and rebuilt at once, which clears the mark.
        assert_eq!(e.mem.u8(p + 0x206), 0);
        assert_eq!(calls_to(&e, 0x0060_f110).len(), 1);
        assert_eq!(calls_to(&e, 0x006d_4f70).len(), 1);
    }

    #[test]
    fn the_path_to_a_target_is_built_or_cleared_again() {
        let mut e = engine();
        let player = new_player(&mut e);
        e.set_global(PLAYER_POINTER, 0x0600_5000u32);
        start_log(&mut e);
        player_character_build_path_to_target(&mut e, player, 0, 0x44, 0);
        player_character_build_path_to_target(&mut e, player, 0x33, 0, 0);
        assert!(calls_to(&e, 0x006d_4f70).is_empty());
        double(&mut e, 0x006d_4f70, eax(1));
        player_character_build_path_to_target(&mut e, player, 0x33, 0x44, 0);
        let built = calls_to(&e, 0x006d_4f70);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][2], 0x44);
        assert_eq!(built[0][4], 2);
        // One clear before the search, none after a success.
        assert_eq!(calls_to(&e, 0x006f_4990), vec![vec![0x44]]);
        // Both locations are destroyed.
        assert_eq!(calls_to(&e, 0x004f_f7e0).len(), 2);
        double(&mut e, 0x006d_4f70, eax(0));
        player_character_build_path_to_target(&mut e, player, 0x33, 0x44, 0);
        assert_eq!(calls_to(&e, 0x006f_4990).len(), 3);
    }

    #[test]
    fn the_map_marker_is_created_placed_and_removed() {
        let mut e = engine();
        let player = new_player(&mut e);
        let p = player.addr();
        e.set_global(PLAYER_POINTER, 0x0600_5000u32);
        e.set_global(0x011c_a248u32, 0x4141u32);
        e.register(OPERATOR_NEW, |e, a| eax(e.mem.alloc(a[0])));
        e.register(0x0055_a2f0, |_, a| eax(a[0]));
        double(&mut e, FORM_TYPE_OF, eax(0x39));
        start_log(&mut e);
        player_character_set_player_map_marker(&mut e, player, 1.0, 2.0, 3.0, 0x66);
        let marker = e.mem.u32(p + 0x6f4);
        assert_ne!(marker, 0);
        assert_eq!(calls_to(&e, 0x0057_5690), vec![vec![marker, 0x4141]]);
        assert_eq!(calls_to(&e, 0x0087_ce80), vec![vec![marker, 0x66]]);
        assert_eq!(calls_to(&e, 0x0049_eea0).len(), 1);
        // A second call reuses the marker; a type 0x41 place goes through 005f36f0.
        double(&mut e, FORM_TYPE_OF, eax(0x41));
        double(&mut e, 0x005f_36f0, eax(0x88));
        player_character_set_player_map_marker(&mut e, player, 1.0, 2.0, 3.0, 0x66);
        assert_eq!(e.mem.u32(p + 0x6f4), marker);
        assert_eq!(calls_to(&e, 0x0087_ce80).last(), Some(&vec![marker, 0x88]));
        // Removal destroys it through slot 0x10.
        let seen = recording_slot(&mut e, marker, 0x10, eax(0));
        player_character_remove_player_map_marker(&mut e, player);
        assert_eq!(*seen.borrow(), vec![vec![marker, 1]]);
        assert_eq!(e.mem.u32(p + 0x6f4), 0);
        assert_eq!(calls_to(&e, 0x006f_4990).last(), Some(&vec![p + 0x6f8]));
        player_character_remove_player_map_marker(&mut e, player);
    }

    #[test]
    fn the_camera_position_is_copied_in_first_person() {
        let mut e = engine();
        let player = new_player(&mut e);
        let source = e.mem.alloc(12);
        e.mem.set_f32(source, 1.0);
        e.mem.set_f32(source + 4, 2.0);
        e.mem.set_f32(source + 8, 3.0);
        double(&mut e, 0x0045_bb80, eax(source));
        double(&mut e, 0x008a_2fa0, eax(0));
        let out = e.mem.alloc(12);
        start_log(&mut e);
        // No camera node yet: the player's own position is asked for.
        assert_eq!(fn_00952ff0(&mut e, player, out), out);
        assert_eq!(calls_to(&e, 0x008a_2fa0), vec![vec![player.addr(), out]]);
        e.set_global(NODE_CAMERA_1ST_SLOT, 0x100u32);
        assert_eq!(fn_00952ff0(&mut e, player, out), out);
        assert_eq!(e.mem.f32(out + 8), 3.0);
        assert_eq!(e.mem.f32(out), 1.0);
        // Third person asks again.
        e.set(player, PlayerCharacter::b3rdPerson, true);
        fn_00952ff0(&mut e, player, out);
        assert_eq!(calls_to(&e, 0x008a_2fa0).len(), 2);
    }

    // ----- FocusOnActor -----

    const TIMER: u32 = 0x011f_6394;

    fn focus_world(e: &mut Engine) -> (Ptr<PlayerCharacter>, u32) {
        float_settings(e);
        list_helpers(e);
        let player = new_player(e);
        let actor = e.mem.alloc(0x10);
        e.set_global(NODE_CAMERA_1ST_SLOT, 0x100u32);
        let position = e.mem.alloc(12);
        e.mem.set_f32(position + 8, 1.0);
        double(e, 0x0045_bb80, eax(position));
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            eax(value as i32 as u32)
        });
        (player, actor)
    }

    #[test]
    fn focusing_without_an_actor_or_in_another_world_does_nothing() {
        let mut e = engine();
        let (player, actor) = focus_world(&mut e);
        e.set_global(FOCUS_BLEND, 0.25f32);
        player_character_focus_on_actor(&mut e, player, 0, 1.0, 0);
        assert_eq!(e.global::<f32>(FOCUS_BLEND), 0.25);
        e.register(0x0057_5d70, |_, a| eax(a[0] & 1));
        player_character_focus_on_actor(&mut e, player, actor + 1, 1.0, 0);
        assert_eq!(e.global::<f32>(FOCUS_BLEND), 0.25);
        // In an interior the cells decide.
        double(&mut e, PARENT_CELL_OF, eax(0x44));
        double(&mut e, 0x0042_5fd0, eax(1));
        e.register(PARENT_CELL_OF, |_, a| {
            eax(if a[0] & 1 == 1 { 0x45 } else { 0x44 })
        });
        player_character_focus_on_actor(&mut e, player, actor + 1, 1.0, 0);
        assert_eq!(e.global::<f32>(FOCUS_BLEND), 0.25);
    }

    #[test]
    fn an_actor_without_a_head_is_logged_once() {
        let mut e = engine();
        let (player, actor) = focus_world(&mut e);
        slot(&mut e, actor, 0x1d0, eax(0x77));
        slot(&mut e, actor, 0x1b0, eax(0));
        double(&mut e, 0x0057_15d0, eax(0x1234));
        start_log(&mut e);
        // Strength is clamped, and stored even without a head.
        player_character_focus_on_actor(&mut e, player, actor, 7.0, 0);
        assert_eq!(e.global::<f32>(FOCUS_BLEND), 1.0);
        assert_eq!(e.global::<u32>(FOCUS_LAST_WARNED_ACTOR), actor);
        assert_eq!(
            calls_to(&e, LOG_MESSAGE),
            vec![vec![FORMAT_FOCUS_WITHOUT_HEAD, 0x1234]]
        );
        player_character_focus_on_actor(&mut e, player, actor, -3.0, 0);
        assert_eq!(e.global::<f32>(FOCUS_BLEND), 0.0);
        assert_eq!(calls_to(&e, LOG_MESSAGE).len(), 1);
        // The field of view is set from the distance (no face data).
        assert_eq!(calls_to(&e, 0x0045_7990).len(), 2);
    }

    #[test]
    fn the_head_bound_sets_the_angle_limit_and_eases_the_view() {
        let mut e = engine();
        let (player, actor) = focus_world(&mut e);
        let p = player.addr();
        slot(&mut e, actor, 0x1d0, eax(0x77));
        slot(&mut e, actor, 0x1b0, eax(0x88));
        double(&mut e, 0x0043_d450, eax(0x99));
        double(&mut e, 0x0066_29f0, eax(1));
        e.register(0x0084_d030, |_, a| {
            st0(if a[0] == TIMER { 0.5 } else { 2.0 })
        });
        e.register(0x0045_7990, |_, _| st0(4.0));
        double(&mut e, 0x005d_c330, st0(0.25));
        e.register(0x0040_ebd0, |_, a| {
            st0(f32::from_bits(a[0]).min(f32::from_bits(a[1])) as f64)
        });
        e.set_global(HUNDRED, 100.0f64);
        e.set_global(HEAD_DISTANCE_FACTOR_SETTING, 1.0f32);
        e.set_global(FOCUS_MAXIMUM_SETTING, 30.0f32);
        e.set_global(FOCUS_SMOOTHING_SETTING, 1.0f32);
        e.mem.set_f32(p + 0x670, 5.0);
        e.mem.set_f32(p + 0x674, 10.0);
        start_log(&mut e);
        player_character_focus_on_actor(&mut e, player, actor, 0.0, 1);
        // limit = 0.25 * 100, below the maximum of 30; the offsets move by
        // min(0.5 / 1, 1) of the way.
        assert_eq!(e.global::<f32>(FOCUS_ANGLE_LIMIT), 25.0);
        assert_eq!(e.global::<f32>(FOCUS_RATIO), 0.5);
        assert_eq!(e.mem.f32(p + 0x670), 15.0);
        assert_eq!(e.mem.f32(p + 0x674), 17.5);
        assert_eq!(e.global::<f32>(FOCUS_BLEND), 0.0);
        // A smaller maximum caps the limit.
        e.set_global(FOCUS_MAXIMUM_SETTING, 20.0f32);
        e.mem.set_f32(p + 0x670, 0.0);
        player_character_focus_on_actor(&mut e, player, actor, 0.0, 1);
        assert_eq!(e.global::<f32>(FOCUS_ANGLE_LIMIT), 20.0);
        assert_eq!(e.mem.f32(p + 0x670), 10.0);
        // The bound of a head-less actor comes from the node found by name.
        assert!(calls_to(&e, 0x0098_ddd0).is_empty());
    }

    #[test]
    fn the_player_turns_towards_the_actor_and_the_camera_is_refreshed() {
        let mut e = engine();
        let (player, actor) = focus_world(&mut e);
        let p = player.addr();
        slot(&mut e, actor, 0x1d0, eax(0x77));
        slot(&mut e, actor, 0x1b0, eax(0x88));
        // The player's slots: heading (0x2bc), the mover flag (0x214), 0x1e0.
        slot(&mut e, p, 0x2bc, st0(0.5));
        slot(&mut e, p, 0x214, eax(0));
        slot(&mut e, p, 0x1e0, eax(0));
        double(&mut e, 0x0043_d450, eax(0x99));
        double(&mut e, 0x0066_29f0, eax(0));
        e.register(0x0084_d030, |_, a| {
            st0(if a[0] == TIMER { 0.5 } else { 2.0 })
        });
        e.register(0x0045_7990, |_, _| st0(4.0));
        e.register(0x0040_ebd0, |_, a| {
            st0(f32::from_bits(a[0]).min(f32::from_bits(a[1])) as f64)
        });
        double(&mut e, 0x004b_5510, st0(0.5));
        double(&mut e, 0x0093_1d70, st0(0.1));
        e.register(0x0040_8840, |_, a| st0(f32::from_bits(a[0]).abs() as f64));
        double(&mut e, 0x004b_13c0, st0(0.8));
        e.set_global(DEGREES_TO_RADIANS, 0.017453292519943295f64);
        e.set_global(PI_DOUBLE, std::f64::consts::PI);
        e.set_global(TWO_PI_DOUBLE, std::f64::consts::TAU);
        e.set_global(MINUS_PI_DOUBLE, -std::f64::consts::PI);
        e.set_global(FOCUS_MAXIMUM_SETTING, 30.0f32);
        e.set_global(FOCUS_RATE_SETTING, 1.0f32);
        e.set_global(PITCH_START_SETTING, 0.0f32);
        e.set_global(YAW_START_SETTING, 0.0f32);
        e.mem.set_f32(p + 0x674, 15.0);
        start_log(&mut e);
        // Full blend: the pitch turn is frame time * (pitch error * rate).
        player_character_focus_on_actor(&mut e, player, actor, 1.0, 0);
        let pitch = calls_to(&e, 0x0093_1e50);
        assert_eq!(pitch.len(), 1);
        assert_eq!(f32::from_bits(pitch[0][1]), 0.5 * (0.5 - 0.1));
        let yaw = calls_to(&e, 0x0093_1d30);
        assert_eq!(yaw.len(), 1);
        assert_eq!(e.global::<u8>(FOCUS_PITCH_TURNING), 1);
        assert_eq!(e.global::<u8>(FOCUS_YAW_TURNING), 1);
        // The camera was switched to first person and refreshed.
        assert_eq!(calls_to(&e, 0x0089_5110).len(), 1);
        assert_eq!(calls_to(&e, 0x008d_3550).len(), 1);
        assert_eq!(calls_to(&e, 0x0088_85e0).len(), 2);
        assert_eq!(e.global::<f32>(FOCUS_SAVED_HEADING), 0.5);
        // With the skip flag set nothing is turned.
        start_log(&mut e);
        player_character_focus_on_actor(&mut e, player, actor, 1.0, 1);
        assert!(calls_to(&e, 0x0093_1e50).is_empty());
    }

    #[test]
    fn small_combat_and_greet_accessors_read_and_write_the_player() {
        let mut e = engine();
        let player = new_player(&mut e);
        let p = player.addr();
        assert_eq!(
            player_character_get_number_actors_in_combat(&mut e, player),
            0
        );
        e.mem.set_u32(p + 0xd64, 0x55);
        double(&mut e, 0x005a_4320, eax(3));
        assert_eq!(
            player_character_get_number_actors_in_combat(&mut e, player),
            3
        );
        e.mem.set_u8(p + 0xdf0, 1);
        e.mem.set_u8(p + 0xdf1, 9);
        let out = e.mem.alloc(4);
        assert_eq!(
            player_character_is_player_character_in_combat(&mut e, player, out),
            1
        );
        assert_eq!(e.mem.u8(out), 9);
        assert_eq!(
            player_character_is_player_character_in_combat(&mut e, player, 0),
            1
        );
        // Greet flag.
        e.mem.set_u8(p + 0x6cc, 1);
        e.mem.set_f32(p + 0x6d0, 2.0);
        e.set_global(GREET_TIMEOUT, 5.0f64);
        fn_00953d00(&mut e, player);
        assert_eq!(e.mem.u8(p + 0x6cc), 1);
        double(&mut e, 0x0084_d030, st0(4.5));
        fn_00953d40(&mut e, player);
        assert_eq!(e.mem.f32(p + 0x6d0), 6.5);
        fn_00953d00(&mut e, player);
        assert_eq!(e.mem.u8(p + 0x6cc), 0);
        assert_eq!(e.mem.f32(p + 0x6d0), 0.0);
        // The timer does not run without the flag.
        fn_00953d40(&mut e, player);
        assert_eq!(e.mem.f32(p + 0x6d0), 0.0);
        player_character_reset_player_greet_flag(&mut e, player);
        // Murderer, and the other byte accessors.
        assert!(!fn_00953f60(&mut e, player));
        start_log(&mut e);
        player_character_set_is_a_murderer(&mut e, player);
        assert!(fn_00953f60(&mut e, player));
        assert_eq!(calls_to(&e, 0x008b_ff70), vec![vec![p, 4, 1]]);
        fn_00953fd0(&mut e, player, 5);
        assert_eq!(fn_00953fb0(&mut e, player), 5);
    }

    #[test]
    fn the_process_lists_object_is_asked_for_and_released() {
        let mut e = engine();
        let player = new_player(&mut e);
        start_log(&mut e);
        assert!(!fn_00953c80(&mut e, player));
        assert!(calls_to(&e, 0x0047_02f0).is_empty());
        assert_eq!(
            calls_to(&e, 0x0097_1c30),
            vec![vec![PROCESS_LISTS, player.addr(), 0x15, 0]]
        );
        double(&mut e, 0x0097_1c30, eax(0x44));
        assert!(fn_00953c80(&mut e, player));
        assert_eq!(calls_to(&e, 0x0047_02f0), vec![vec![0x44, 1]]);
    }

    #[test]
    fn sitting_and_getting_up_start_packages() {
        let mut e = engine();
        let player = new_player(&mut e);
        let p = player.addr();
        e.set_global(PLAYER_POINTER, 0x0600_5000u32);
        let process = e.mem.alloc(8);
        e.set(player, PlayerCharacter::pCurrentProcess, Ptr::new(process));
        slot(&mut e, process, 0x3e4, eax(7));
        let launched = recording_slot(&mut e, p, 0x2f4, eax(0));
        e.register(OPERATOR_NEW, |e, a| eax(e.mem.alloc(a[0])));
        double(&mut e, 0x0067_0b90, eax(0x500));
        double(&mut e, 0x0067_f030, eax(0x600));
        start_log(&mut e);
        player_character_initiate_sit_sleep_package(&mut e, player, 0x77);
        // The mover is cleared unless the path is complete.
        assert_eq!(calls_to(&e, 0x009d_af80).len(), 1);
        assert_eq!(calls_to(&e, 0x0093_a5f0), vec![vec![p, 1]]);
        // Process type 7 resets something on the player.
        assert_eq!(calls_to(&e, 0x0089_4cc0), vec![vec![p, 0]]);
        assert_eq!(calls_to(&e, 0x0067_0fc0), vec![vec![0x500, 6]]);
        assert_eq!(calls_to(&e, 0x0067_f3c0), vec![vec![0x600, 0x77]]);
        assert_eq!(calls_to(&e, 0x0067_1d30), vec![vec![0x500, 0x600]]);
        assert_eq!(calls_to(&e, 0x0067_0b30), vec![vec![0x600, 1]]);
        assert_eq!(*launched.borrow(), vec![vec![p, 0x500, 0, 1]]);
        // Getting up.
        double(&mut e, 0x008b_3bb0, eax(1));
        start_log(&mut e);
        player_character_initiate_get_up_package(&mut e, player);
        assert!(calls_to(&e, 0x009d_af80).is_empty());
        assert_eq!(calls_to(&e, 0x0093_a6f0), vec![vec![p, 1]]);
        assert_eq!(calls_to(&e, 0x008a_75a0), vec![vec![p]]);
    }

    #[test]
    fn the_heading_adds_the_offset_and_is_wrapped() {
        let mut e = engine();
        let player = new_player(&mut e);
        e.mem.set_f32(player.addr() + 0x6e4, 0.5);
        double(&mut e, 0x008b_d7b0, st0(1.0));
        e.register(0x004b_1480, |_, a| st0(f32::from_bits(a[0]) as f64 - 0.25));
        assert_eq!(player_character_get_heading(&mut e, player, 0), 1.25);
    }

    /// The settings the save code reads: the pointer getters answer the
    /// address of the setting, whose value then sits in game memory.
    fn save_settings(e: &mut Engine) {
        e.register(0x0040_8d60, |_, a| eax(a[0]));
        e.register(SETTING_INT_POINTER, |_, a| eax(a[0]));
    }

    // ----- pick-up, drop and the small predicates -----

    /// A process with the item-change slots `0x148` and `0x14c` answering
    /// `current`, hung on the player.
    fn process_with_items(e: &mut Engine, player: Ptr<PlayerCharacter>, current: u32) -> u32 {
        let process = e.mem.alloc(8);
        e.set(player, PlayerCharacter::pCurrentProcess, Ptr::new(process));
        slot(e, process, 0x148, eax(current));
        slot(e, process, 0x14c, eax(current));
        process
    }

    #[test]
    fn picking_up_an_unwanted_ammunition_form_stops_at_once() {
        let mut e = engine();
        let player = new_player(&mut e);
        let item = e.mem.alloc(8);
        double(&mut e, 0x007a_f430, eax(0x4000));
        double(&mut e, FORM_TYPE_OF, eax(0x28));
        double(&mut e, 0x0047_bcf0, eax(0));
        start_log(&mut e);
        fn_00953ff0(&mut e, player, item, 3, 0);
        assert!(calls_to(&e, 0x008a_ded0).is_empty());
        double(&mut e, FORM_TYPE_OF, eax(0x29));
        double(&mut e, 0x004c_94d0, eax(0));
        fn_00953ff0(&mut e, player, item, 3, 0);
        assert!(calls_to(&e, 0x008a_ded0).is_empty());
    }

    #[test]
    fn a_stolen_object_raises_the_alarm_and_the_reference_is_destroyed() {
        let mut e = engine();
        list_helpers(&mut e);
        let player = new_player(&mut e);
        let p = player.addr();
        e.set_global(PLAYER_POINTER, 0x0600_5000u32);
        process_with_items(&mut e, player, 0);
        let item = e.mem.alloc(8);
        let destroyed = recording_slot(&mut e, item, 0x10, eax(0));
        slot(&mut e, item, 0x1d0, eax(0x5151));
        double(&mut e, 0x007a_f430, eax(0x4000));
        double(&mut e, FORM_TYPE_OF, eax(0x11));
        let actors = node(&mut e, 0x55, 0);
        double(&mut e, 0x0096_f450, eax(actors));
        double(&mut e, 0x0088_1650, eax(item));
        double(&mut e, 0x0056_7790, eax(0x77));
        double(&mut e, 0x0097_3ab0, eax(0x88));
        double(&mut e, EXTRA_LIST_OF, eax(0xe0));
        start_log(&mut e);
        fn_00953ff0(&mut e, player, item, 3, 0);
        assert_eq!(calls_to(&e, 0x008a_ded0), vec![vec![p, 0x4000, 1, 0]]);
        assert_eq!(calls_to(&e, 0x00c6_a270), vec![vec![0x5151, 1, 1, 0]]);
        // The actor that was at the item is told about the player.
        assert_eq!(calls_to(&e, 0x0088_1620), vec![vec![0x55, p]]);
        assert_eq!(calls_to(&e, 0x0047_02f0), vec![vec![actors, 1]]);
        // Not an owner: the thief is told, the owner is set on the extra data.
        assert_eq!(
            calls_to(&e, 0x008b_fa40),
            vec![vec![p, 0x88, 0x4000, 3, 0, 0x77]]
        );
        assert_eq!(calls_to(&e, 0x0041_9700), vec![vec![0xe0, 0x77]]);
        assert_eq!(e.global::<u32>(STEAL_ACTOR), 0x88);
        // Nothing was a reason to keep it: transferred whole and destroyed.
        assert_eq!(calls_to(&e, 0x0057_4b30), vec![vec![p, item, 3, 0, 0]]);
        assert_eq!(*destroyed.borrow(), vec![vec![item, 1]]);
        // An evil owner takes the ownership away instead.
        double(&mut e, 0x0057_8790, eax(1));
        fn_00953ff0(&mut e, player, item, 3, 0);
        assert_eq!(calls_to(&e, 0x0041_aed0), vec![vec![0xe0]]);
    }

    #[test]
    fn picking_up_the_current_ammunition_adds_to_the_item_counts() {
        let mut e = engine();
        list_helpers(&mut e);
        let player = new_player(&mut e);
        let p = player.addr();
        process_with_items(&mut e, player, 0x300);
        let item = e.mem.alloc(8);
        slot(&mut e, item, 0x1d0, eax(0x5151));
        double(&mut e, 0x007a_f430, eax(0x4000));
        double(&mut e, FORM_TYPE_OF, eax(0x28));
        double(&mut e, 0x0047_bcf0, eax(1));
        double(&mut e, 0x004c_0bf0, eax(1));
        double(&mut e, WORD_AT_8, eax(0x4000));
        double(&mut e, LIST_NODE_NEXT, eax(5));
        double(&mut e, 0x0057_2d30, eax(1));
        start_log(&mut e);
        fn_00953ff0(&mut e, player, item, 3, 0);
        assert_eq!(
            calls_to(&e, 0x006e_cd40),
            vec![vec![0x300, 8], vec![0x300, 8]]
        );
        // Marked as touched; picked up with the flag byte given and kept.
        assert_eq!(calls_to(&e, 0x0057_4b30), vec![vec![p, item, 3, 0, 1]]);
        assert_eq!(calls_to(&e, 0x0057_2230), vec![vec![item]]);
    }

    #[test]
    fn dropping_through_a_position_lets_the_virtual_slot_do_everything() {
        let mut e = engine();
        let player = new_player(&mut e);
        let p = player.addr();
        let dropped = recording_slot(&mut e, p, 0x17c, eax(0xd0d));
        double(&mut e, 0x0070_43c0, eax(1));
        double(&mut e, PARENT_CELL_OF, eax(0x500));
        double(&mut e, 0x0055_1110, eax(0x600));
        start_log(&mut e);
        let result = fn_00954610(&mut e, player, 0x11, 0x22, 0x33, 0x44, 0x55);
        assert_eq!(result, 0);
        assert_eq!(
            *dropped.borrow(),
            vec![vec![p, 0x11, 0x22, 0x33, 0, 0, 0x600, 0, 0, 1, 0]]
        );
        assert_eq!(calls_to(&e, 0x009c_8950), vec![vec![OBJECT_011F2250, 0, 0]]);
    }

    #[test]
    fn dropping_the_current_ammunition_empties_it_and_places_the_reference() {
        let mut e = engine();
        list_helpers(&mut e);
        let player = new_player(&mut e);
        let p = player.addr();
        e.set_global(PLAYER_POINTER, 0x0600_5000u32);
        process_with_items(&mut e, player, 0x300);
        let item = e.mem.alloc(8);
        slot(&mut e, item, 0xe4, eax(1));
        double(&mut e, FORM_TYPE_OF, eax(0x29));
        double(&mut e, 0x008a_1710, eax(0x31));
        double(&mut e, 0x0052_5980, eax(item));
        double(&mut e, 0x0095_0bb0, eax(0x4242));
        double(&mut e, 0x004a_ae30, eax(0x4343));
        // The remembered item is forgotten when it is dropped and worn.
        e.mem.set_u32(p + 0x1f0, item);
        double(&mut e, 0x0041_8ab0, eax(1));
        // The reference slot 0x17c creates: without a body it is listed.
        let reference = e.mem.alloc(0x20);
        slot(&mut e, reference, 0x1d0, eax(0));
        let created = recording_slot(&mut e, p, 0x17c, eax(reference));
        let appended = record_appends(&mut e);
        start_log(&mut e);
        let result = fn_00954610(&mut e, player, item, 0x22, 0x33, 0x44, 0x55);
        assert_eq!(result, reference);
        assert_eq!(calls_to(&e, 0x006e_cd40), vec![vec![0x300, 0]]);
        assert_eq!(
            calls_to(&e, 0x0045_0f90),
            vec![vec![0x4343, 1], vec![0x4343, 1]]
        );
        assert_eq!(e.mem.u32(p + 0x1f0), 0);
        assert_eq!(
            *created.borrow(),
            vec![vec![p, item, 0x22, 0x33, 0, 1, 0, 0x44, 0x55, 1, 0]]
        );
        assert_eq!(*appended.borrow(), vec![(p + 0x84c, reference)]);
        assert_eq!(e.mem.u32(reference + 8) & 0x0040_0000, 0x0040_0000);
        assert_eq!(calls_to(&e, 0x0056_4d20), vec![vec![reference, 1]]);
        assert_eq!(calls_to(&e, 0x008a_ded0), vec![vec![p, item, 0, 0]]);
    }

    #[test]
    fn a_dropped_reference_with_a_body_is_activated_in_place() {
        let mut e = engine();
        let player = new_player(&mut e);
        let p = player.addr();
        e.set_global(PLAYER_POINTER, 0x0600_5000u32);
        let item = e.mem.alloc(8);
        slot(&mut e, item, 0xe4, eax(0));
        double(&mut e, FORM_TYPE_OF, eax(0x11));
        let reference = e.mem.alloc(0x20);
        slot(&mut e, reference, 0x1d0, eax(0x900));
        let position = e.mem.alloc(12);
        e.mem.set_u32(position, 7);
        slot(&mut e, reference, 0x1f4, eax(position));
        recording_slot(&mut e, p, 0x17c, eax(reference));
        start_log(&mut e);
        assert_eq!(fn_00954610(&mut e, player, item, 0, 0, 0, 0), reference);
        assert_eq!(calls_to(&e, 0x00c6_a040).len(), 2);
        assert_eq!(calls_to(&e, 0x00c6_bd00), vec![vec![0x900, 1]]);
        assert_eq!(calls_to(&e, 0x00c6_a270), vec![vec![0x900, 1, 1, 0]]);
        assert_eq!(calls_to(&e, 0x0057_5830).len(), 1);
        // Not worth selecting: the closing call gets a null base.
        assert_eq!(calls_to(&e, 0x008a_ded0), vec![vec![p, 0, 0, 0]]);
    }

    #[test]
    fn the_form_flag_and_god_mode_scale_helpers() {
        let mut e = engine();
        let object = e.mem.alloc(16);
        fn_00954910(&mut e, Ptr::new(object), 1);
        assert_eq!(e.mem.u32(object + 8), 0x0040_0000);
        e.mem.set_u32(object + 8, 0xffff_ffff);
        fn_00954910(&mut e, Ptr::new(object), 0);
        assert_eq!(e.mem.u32(object + 8), 0xffbf_ffff);
        let player = new_player(&mut e);
        double(&mut e, IS_GOD_MODE, eax(0));
        assert_eq!(fn_00954960(&mut e, player, 3, 0), 1.0);
        double(&mut e, IS_GOD_MODE, eax(1));
        double(&mut e, 0x008c_4610, st0(0.75));
        start_log(&mut e);
        assert_eq!(fn_00954960(&mut e, player, 3, 0), 0.75);
        assert_eq!(
            calls_to(&e, 0x008c_4610),
            vec![vec![player.addr(), 3, 0.0f32.to_bits()]]
        );
    }

    #[test]
    fn the_health_level_is_compared_with_the_actor_value() {
        let mut e = engine();
        let player = new_player(&mut e);
        let owner = player.addr() + 0xa4;
        let seen = recording_slot(&mut e, owner, 8, eax(10));
        double(&mut e, IS_GOD_MODE, eax(1));
        assert!(!fn_00954cc0(&mut e, player));
        assert!(seen.borrow().is_empty());
        double(&mut e, IS_GOD_MODE, eax(0));
        double(&mut e, 0x008a_0c20, st0(5.0));
        assert!(fn_00954cc0(&mut e, player));
        assert_eq!(*seen.borrow(), vec![vec![owner, 0x2e]]);
        double(&mut e, 0x008a_0c20, st0(15.0));
        assert!(!fn_00954cc0(&mut e, player));
    }

    /// An actor that passes the four checks of the package predicates, with
    /// a package, a process holder and an acquire object.
    fn package_actor(e: &mut Engine) -> (u32, u32, u32) {
        let actor = e.mem.alloc(8);
        slot(e, actor, 0x22c, eax(0));
        slot(e, actor, 0x234, eax(0));
        let holder = e.mem.alloc(16);
        slot(e, holder, 0x128, eax(0x0600_5000));
        slot(e, holder, 0x24, eax(0));
        double(e, 0x008d_8520, eax(holder));
        double(e, 0x0088_1510, eax(0x700));
        e.set_global(PLAYER_POINTER, 0x0600_5000u32);
        (actor, holder, 0x700)
    }

    #[test]
    fn an_actor_using_the_player_is_recognised_by_its_package() {
        let mut e = engine();
        let (actor, _, _package) = package_actor(&mut e);
        let this = Ptr::<()>::new(0);
        e.register(0x0041_ca90, |_, a| eax(if a[0] == 0x700 { 1 } else { 0 }));
        assert_eq!(fn_009549a0(&mut e, this, actor), 1);
        // The player must be the target; a detail the last check can veto.
        double(&mut e, 0x008a_6210, eax(1));
        assert_eq!(fn_009549a0(&mut e, this, actor), 0);
        double(&mut e, 0x008a_6210, eax(0));
        e.register(0x0041_ca90, |_, _| eax(5));
        assert_eq!(fn_009549a0(&mut e, this, actor), 0);
        e.register(0x0041_ca90, |_, _| eax(7));
        assert_eq!(fn_009549a0(&mut e, this, actor), 1);
        // One of the first four checks failing ends it.
        double(&mut e, 0x0044_0da0, eax(1));
        assert_eq!(fn_009549a0(&mut e, this, actor), 0);
        double(&mut e, 0x0044_0da0, eax(0));
        double(&mut e, 0x0088_1510, eax(0));
        assert_eq!(fn_009549a0(&mut e, this, actor), 0);
    }

    #[test]
    fn the_package_check_with_a_mode_looks_at_the_target_object() {
        let mut e = engine();
        let (actor, holder, _) = package_actor(&mut e);
        let this = Ptr::<()>::new(0);
        let told = recording_slot(&mut e, holder, 0x24, eax(0));
        let player_pointer = 0x0600_5000;
        // Mode 0: the acquire object (holder + 4) names the player.
        e.register(0x0041_ca90, |_, _| eax(1));
        double(&mut e, 0x0067_1d10, eax(0x710));
        double(&mut e, WORD_AT_8, eax(player_pointer));
        double(&mut e, 0x008a_6290, eax(0));
        assert_eq!(fn_00954a70(&mut e, this, actor, 0), 1);
        assert_eq!(*told.borrow(), vec![vec![holder, actor, 1]]);
        double(&mut e, 0x008a_6290, eax(1));
        assert_eq!(fn_00954a70(&mut e, this, actor, 0), 0);
        double(&mut e, WORD_AT_8, eax(0x1234));
        double(&mut e, 0x0068_0050, eax(0));
        assert_eq!(fn_00954a70(&mut e, this, actor, 0), 0);
        e.register(0x0041_ca90, |_, _| eax(3));
        assert_eq!(fn_00954a70(&mut e, this, actor, 0), 0);
        // Mode 1: the package location is the player.
        e.register(0x0041_ca90, |_, _| eax(1));
        double(&mut e, WORD_AT_8, eax(player_pointer));
        double(&mut e, 0x0067_6140, eax(player_pointer));
        double(&mut e, 0x008a_6210, eax(0));
        assert_eq!(fn_00954a70(&mut e, this, actor, 1), 1);
        double(&mut e, 0x008a_6210, eax(1));
        assert_eq!(fn_00954a70(&mut e, this, actor, 1), 0);
        // Not the location: the kind 9 package or a foreign holder target ends it.
        double(&mut e, 0x0067_6140, eax(0));
        double(&mut e, 0x008a_6210, eax(0));
        e.register(0x0041_ca90, |_, _| eax(9));
        assert_eq!(fn_00954a70(&mut e, this, actor, 1), 0);
        e.register(0x0041_ca90, |_, _| eax(1));
        assert_eq!(fn_00954a70(&mut e, this, actor, 1), 1);
        slot(&mut e, holder, 0x128, eax(0x1111));
        double(&mut e, WORD_AT_8, eax(0x2222));
        assert_eq!(fn_00954a70(&mut e, this, actor, 1), 0);
        // A failing first check.
        double(&mut e, 0x0043_7bf0, eax(1));
        assert_eq!(fn_00954a70(&mut e, this, actor, 1), 0);
    }

    // ----- save sizes, save and load -----

    #[test]
    fn the_save_size_adds_every_field_of_the_version() {
        let mut e = engine();
        save_settings(&mut e);
        let player = new_player(&mut e);
        e.set_global(SAVE_LOAD_GAME, 0x1111u32);
        let name = e.mem.alloc(8);
        double(&mut e, 0x0055_d520, eax(name));
        double(&mut e, 0x00ec_6130, eax(5));
        version(&mut e, 0x7a);
        assert_eq!(fn_00954d40(&mut e, player, 0), 1352);
        version(&mut e, 0x20);
        assert_eq!(fn_00954d40(&mut e, player, 0), 759);
        // Blocks add a header before and after the base part; the flagged
        // first-person animation adds its own size.
        double(&mut e, 0x0086_2110, eax(1));
        double(&mut e, 0x0049_aa20, eax(100));
        assert_eq!(fn_00954d40(&mut e, player, 0), 771);
        assert_eq!(fn_00954d40(&mut e, player, 0x1000_0000), 871);
    }

    #[test]
    fn the_save_size_counts_the_lists_and_logs_when_asked() {
        let mut e = engine();
        save_settings(&mut e);
        let player = new_player(&mut e);
        e.set_global(SAVE_LOAD_GAME, 0x1111u32);
        let name = e.mem.alloc(8);
        double(&mut e, 0x0055_d520, eax(name));
        double(&mut e, 0x00ec_6130, eax(0));
        version(&mut e, 0x7a);
        let without = fn_00954d40(&mut e, player, 0);
        // Three entries in each counted list.
        double(&mut e, LIST_COUNT, eax(3));
        double(&mut e, FORM_ID_OF, eax(3));
        let with = fn_00954d40(&mut e, player, 0);
        // 0x854 table (x5), the global list (x4), +0x610 (x8), +0x614 (x4),
        // +0x618 (x4), the topics (x4), the quest log (x6), the targets (x5).
        assert_eq!(with - without, 3 * (5 + 4 + 8 + 4 + 4 + 4 + 6 + 5));
        // The debug report: with and without a world space record.
        e.mem.set_u8(SAVE_SIZE_DEBUG_SETTING, 1);
        start_log(&mut e);
        fn_00954d40(&mut e, player, 0);
        let reports = calls_to(&e, ERROR);
        assert_eq!(reports.len(), 2);
        assert_eq!(reports[0][0], FORMAT_SAVE_SIZE);
        assert_eq!(reports[0][2], 0x3248);
        assert_eq!(reports[1][2], 0x3365);
        let record = e.mem.alloc(16);
        e.mem.set_u32(record, 0x77);
        double(&mut e, 0x004f_d3e0, eax(record));
        let form_type = e.mem.alloc(4);
        slot(&mut e, form_type, 0x130, eax(0x4040));
        double(&mut e, 0x0048_39c0, eax(form_type));
        start_log(&mut e);
        fn_00954d40(&mut e, player, 0);
        let reports = calls_to(&e, ERROR);
        assert_eq!(reports[0][0], FORMAT_SAVE_SIZE_FORM);
        assert_eq!(&reports[0][2..4], &[0x77, 0x4040]);
    }

    type Writes = Rc<RefCell<Vec<(u32, Vec<u8>)>>>;

    /// Records the writes of `SaveGameDataOLD`, `SaveNumericID` and the raw
    /// write: the address called and the bytes at the moment of the call.
    fn record_saves(e: &mut Engine) -> Writes {
        let seen: Writes = Rc::new(RefCell::new(Vec::new()));
        for address in [0x0048_4ce0u32, 0x0048_4d20, 0x0085_79b0] {
            let record = seen.clone();
            e.register_double(address, move |e, w| {
                let bytes = e.mem.bytes(w[1], w[2]);
                record.borrow_mut().push((address, bytes));
                eax(0)
            });
        }
        seen
    }

    #[test]
    fn the_old_save_writes_the_modifiers_the_fields_and_the_form_ids() {
        let mut e = engine();
        save_settings(&mut e);
        list_helpers(&mut e);
        double(&mut e, 0x0082_5c00, eax(0x0600_2000));
        let player = new_player(&mut e);
        let p = player.addr();
        e.set_global(SAVE_LOAD_GAME, 0x1111u32);
        version(&mut e, 0x20);
        let writes = record_saves(&mut e);
        e.mem.set_u32(p + 0x244, 0xdead_beef);
        e.mem.set_u32(p + 0x208, 0x5000);
        e.mem.set_u32(p + 0x6ec, 0x6000);
        e.register(FORM_ID_OF, |_, a| eax(a[0] + 1));
        double(&mut e, DYNAMIC_CAST, eax(0x6100));
        let name = e.mem.alloc(8);
        e.mem.set_cstr(name, b"Abc");
        double(&mut e, 0x0055_d520, eax(name));
        double(&mut e, 0x00ec_6130, eax(3));
        fn_00955620(&mut e, player, 0);
        let writes = writes.borrow();
        assert_eq!(writes[0].0, 0x0048_4ce0);
        assert_eq!(writes[0].1.len(), 0x134);
        assert_eq!(&writes[0].1[..4], &[0xef, 0xbe, 0xad, 0xde]);
        assert_eq!(writes[1].1.len(), 0x134);
        // Version 0x20 has no damage array: the next write is the health word.
        assert_eq!(writes[2].1.len(), 4);
        let ids: Vec<&(u32, Vec<u8>)> = writes.iter().filter(|w| w.0 == 0x0048_4d20).collect();
        assert_eq!(ids[0].1, 0x5001u32.to_le_bytes());
        // The global form is empty; the spell cast gives 0x6100.
        assert_eq!(ids[1].1, 0u32.to_le_bytes());
        assert_eq!(ids[2].1, 0x6101u32.to_le_bytes());
        // The name goes last among the data writes: its length byte, then
        // the characters.
        let tail = &writes[writes.len() - 2..];
        assert_eq!(tail[0].1, vec![4u8]);
        assert_eq!(tail[1].1.len(), 4);
        assert_eq!(&tail[1].1[..3], b"Abc");
    }

    #[test]
    fn the_old_save_brackets_its_parts_in_blocks_when_the_save_uses_them() {
        let mut e = engine();
        save_settings(&mut e);
        list_helpers(&mut e);
        let player = new_player(&mut e);
        e.set_global(SAVE_LOAD_GAME, 0x1111u32);
        version(&mut e, 0x20);
        double(&mut e, 0x0086_2110, eax(1));
        let writes = record_saves(&mut e);
        let counter = Rc::new(RefCell::new(0x0600_1000u32));
        let ticks = counter.clone();
        e.register_double(0x0082_5c00, move |_, _| {
            *ticks.borrow_mut() += 0x10;
            eax(*ticks.borrow())
        });
        double(&mut e, 0x0055_d520, eax(0x0600_8000));
        e.mem.set_u8(0x0600_8000, 0);
        fn_00955620(&mut e, player, 0);
        let writes = writes.borrow();
        // The block marker comes first, in raw writes.
        assert_eq!(writes[0].0, 0x0085_79b0);
        assert_eq!(writes[0].1, b"KOLB".to_vec());
        assert_eq!(writes[1].0, 0x0085_79b0);
        assert_eq!(writes[1].1.len(), 2);
        // The first block's length word (the second position asked for,
        // 0x20 further than the first) holds the bytes written since.
        let first_block = 0x0600_1020;
        assert_ne!(e.mem.u16(first_block), 0);
    }

    #[test]
    fn the_old_load_reads_in_the_save_order_and_resolves_the_ids() {
        let mut e = engine();
        save_settings(&mut e);
        let player = new_player(&mut e);
        let p = player.addr();
        e.set_global(SAVE_LOAD_GAME, 0x1111u32);
        version(&mut e, 0x20);
        let reads: Rc<RefCell<Vec<(u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let record = reads.clone();
        e.register_double(0x0048_4d00, move |_, w| {
            record.borrow_mut().push((w[1], w[2]));
            eax(0)
        });
        let next_id = Rc::new(RefCell::new(0x100u32));
        let ids = next_id.clone();
        e.register_double(0x0048_4d40, move |e, w| {
            e.mem.set_u32(w[1], *ids.borrow());
            *ids.borrow_mut() += 1;
            eax(0)
        });
        e.register(0x0048_39c0, |_, a| eax(a[0] + 0x2000));
        e.register(DYNAMIC_CAST, |_, a| eax(a[0]));
        fn_00956f70(&mut e, player, 0, 0x55);
        let reads = reads.borrow();
        assert_eq!(reads[0], (p + 0x244, 0x130));
        assert_eq!(reads[1], (p + 0x378, 0x130));
        assert_eq!(reads[2], (p + 0x4ac, 4));
        // The fields follow, then the ids.
        assert_eq!(e.mem.u32(p + 0x208), 0x100);
        assert_eq!(e.global::<u32>(SAVED_FORM_011E0784), 0x101);
        assert_eq!(e.mem.u32(p + 0x214), 0x103);
        assert_eq!(e.mem.u32(p + 0x218), 0x104);
        assert_eq!(e.mem.u32(p + 0x6f0), 0x106);
        assert_eq!(e.mem.u32(p + 0x73c), 0x107);
        assert_eq!(e.mem.u32(p + 0x758), 0x108);
        // The base class is loaded with both words.
        assert!(reads.iter().any(|r| *r == (p + 0x64a, 1)));
        assert!(reads.iter().any(|r| *r == (p + 0x730, 4)));
    }

    #[test]
    fn the_old_load_of_a_recent_version_reads_the_lists_and_rebuilds_the_quest_log() {
        let mut e = engine();
        save_settings(&mut e);
        list_helpers(&mut e);
        let player = new_player(&mut e);
        let p = player.addr();
        e.set_global(SAVE_LOAD_GAME, 0x1111u32);
        version(&mut e, 0x7a);
        let reads: Rc<RefCell<Vec<(u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let record = reads.clone();
        // Counts read through the plain loader are 1 for the first word.
        e.register_double(0x0048_4d00, move |e, w| {
            record.borrow_mut().push((w[1], w[2]));
            if w[2] == 2 {
                e.mem.set_u16(w[1], 1);
            }
            eax(0)
        });
        e.register(0x0048_4d40, |e, w| {
            e.mem.set_u32(w[1], 0x300);
            eax(0)
        });
        e.register(0x0048_39c0, |_, a| eax(a[0] + 0x2000));
        e.register(DYNAMIC_CAST, |_, a| eax(a[0]));
        double(&mut e, 0x0060_db40, eax(0x9100));
        double(&mut e, 0x0060_f2c0, eax(0x9200));
        double(&mut e, 0x0060_c8e0, eax(0x9300));
        let appended = Rc::new(RefCell::new(Vec::<(u32, u32)>::new()));
        for address in [0x005a_e3d0u32, 0x0090_5820] {
            let seen = appended.clone();
            e.register_double(address, move |e, w| {
                seen.borrow_mut().push((w[0], e.mem.u32(w[1])));
                eax(0)
            });
        }
        fn_00956f70(&mut e, player, 0x10, 0x55);
        let appended = appended.borrow();
        // The quest log, the targets and the topics were appended.
        assert!(appended.contains(&(p + 0x6b0, 0x9200)));
        assert!(appended.contains(&(p + 0x6bc, 0x9300)));
        assert!(appended.contains(&(p + 0x6a8, 0x2300)));
        assert_eq!(e.mem.u32(p + 0x6b8), 0x2300);
    }

    #[test]
    fn the_second_load_pass_turns_ids_into_forms_and_selects_the_spell_and_scroll() {
        let mut e = engine();
        let player = new_player(&mut e);
        let p = player.addr();
        e.set_global(SAVE_LOAD_GAME, 0x1111u32);
        version(&mut e, 0x20);
        e.register(0x0048_39c0, |_, a| eax(a[0] + 0x1000));
        e.register(DYNAMIC_CAST, |_, a| {
            eax(match a[3] {
                RTTI_SPELL_INTERFACE => 0x40,
                RTTI_MAGIC_ITEM_FORM => 0,
                _ => a[0],
            })
        });
        double(&mut e, LIST_NODE_NEXT, eax(0x70));
        e.register(0x0040_a250, |_, a| eax(a[0] + 1));
        e.register(0x0082_5550, |_, a| eax(a[0] + 2));
        e.mem.set_u32(p + 0x208, 5);
        e.mem.set_u32(p + 0x6ec, 9);
        e.mem.set_u32(p + 0x214, 0x10);
        e.mem.set_u32(p + 0x218, 0x20);
        e.mem.set_u32(p + 0x6f0, 0x30);
        e.mem.set_u32(p + 0x73c, 0x40);
        e.set_global(SAVED_FORM_011E0784, 8u32);
        let process = e.mem.alloc(8);
        e.set(player, PlayerCharacter::pCurrentProcess, Ptr::new(process));
        let seen = recording_slot(&mut e, process, 0x58, eax(0));
        start_log(&mut e);
        fn_00958990(&mut e, player, 1, 2);
        assert_eq!(e.mem.u32(p + 0x208), 0x1005);
        assert_eq!(e.global::<u32>(SAVED_FORM_011E0784), 0x1008);
        assert_eq!(e.mem.u32(p + 0x214), 0x11);
        assert_eq!(e.mem.u32(p + 0x218), 0x22);
        assert_eq!(e.mem.u32(p + 0x73c), 0x1040);
        // The scroll (cast to 0x1030) selects the spell after 0x70.
        assert_eq!(
            e.get(player, PlayerCharacter::pSelectedScroll).addr(),
            0x1030
        );
        assert_eq!(e.get(player, PlayerCharacter::pSelectedSpell).addr(), 0x88);
        assert_eq!(seen.borrow().len(), 1);
        assert_eq!(calls_to(&e, 0x008d_3380), vec![vec![p, 1, 2]]);
        assert_eq!(calls_to(&e, 0x008d_0600), vec![vec![p + 0x878, 1, 2]]);
        assert_eq!(calls_to(&e, 0x0093_a5f0), vec![vec![p, 0]]);
    }

    #[test]
    fn the_second_load_pass_purges_the_global_list_of_unresolved_entries() {
        let mut e = engine();
        list_helpers(&mut e);
        let player = new_player(&mut e);
        let process = e.mem.alloc(8);
        e.set(player, PlayerCharacter::pCurrentProcess, Ptr::new(process));
        slot(&mut e, process, 0x58, eax(0));
        e.set_global(SAVE_LOAD_GAME, 0x1111u32);
        version(&mut e, 0x73);
        // The global list: two entries; the first does not resolve.
        e.mem.set_u32(0x011e_0ae8, 0xa0a);
        let second = node(&mut e, 0xb0b, 0);
        e.mem.set_u32(0x011e_0aec, second);
        e.register(0x0048_39c0, |_, a| eax(a[0]));
        e.register(DYNAMIC_CAST, |_, a| {
            eax(if a[0] == 0xa0a { 0 } else { a[0] })
        });
        e.register(0x0063_f7b0, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next != 0 {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            } else {
                e.mem.set_u32(a[0], 0);
            }
            eax(0)
        });
        start_log(&mut e);
        fn_00958990(&mut e, player, 0, 0);
        assert_eq!(calls_to(&e, 0x0063_f7b0), vec![vec![0x011e_0ae8]]);
        // The resolved entry was stored back.
        assert_eq!(calls_to(&e, 0x0072_6c60).len(), 2);
    }

    #[test]
    fn the_last_load_pass_sets_the_first_person_camera_and_spring() {
        let mut e = engine();
        let player = new_player(&mut e);
        let p = player.addr();
        double(&mut e, 0x0095_0bb0, eax(0));
        e.mem.set_u32(p + 0x638, 0x77);
        e.mem.set_u32(p + 0x63c, 2);
        e.mem.set_f32(p + 0x644, 1.5);
        start_log(&mut e);
        fn_00958ec0(&mut e, player, 3, 4);
        assert_eq!(calls_to(&e, 0x008a_a9a0), vec![vec![p, 3, 4]]);
        assert_eq!(
            calls_to(&e, 0x0095_f930),
            vec![vec![p, 0x77, 2, 1.5f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, 0x008d_0600), vec![vec![p + 0x878, 3, 4]]);
        // A spring of mode 3 is not created.
        e.mem.set_u32(p + 0x63c, 3);
        fn_00958ec0(&mut e, player, 3, 4);
        assert_eq!(calls_to(&e, 0x0095_f930).len(), 1);
    }

    #[test]
    fn the_reset_before_a_load_clears_the_player_and_marks_it() {
        let mut e = engine();
        let player = new_player(&mut e);
        let p = player.addr();
        e.set_global(SAVE_LOAD_GAME, 0x1111u32);
        e.set_global(SAVED_FORM_011E0784, 5u32);
        e.mem.set_u32(p + 0xc8, 0x3f80_0000);
        e.mem.set_u8(p + 0x66d, 1);
        start_log(&mut e);
        fn_00958fc0(&mut e, player, 0x1000_0000);
        assert_eq!(e.mem.u8(p + 0xe39), 1);
        assert_eq!(e.mem.f32(p + 0xc8), 0.0);
        assert_eq!(e.mem.u8(p + 0x66d), 0);
        assert_eq!(e.mem.u8(p + 0x75d), 1);
        assert_eq!(
            calls_to(&e, 0x0045_34f0),
            vec![vec![p + 0x878, 0x1000_0000]]
        );
        // The extra clean-up only runs when the save-load object asks for it.
        assert_eq!(e.global::<u32>(SAVED_FORM_011E0784), 5);
        double(&mut e, 0x0055_f5b0, eax(1));
        e.mem.set_u32(p + 0x690, 0x4141);
        fn_00958fc0(&mut e, player, 0x1000_0000);
        assert_eq!(e.global::<u32>(SAVED_FORM_011E0784), 0);
        assert_eq!(calls_to(&e, 0x0049_a920), vec![vec![0x4141, p]]);
        fn_009590d0(&mut e, player, 7);
        assert_eq!(e.mem.u8(p + 0xe39), 7);
    }

    // ----- the save and load buffers -----

    /// A save buffer object: records every write (`00865e50`) and form id
    /// (`00865df0`), hands out increasing placeholders (`00865f20`) and
    /// records the counts stored (`00865ff0`).
    struct BufferLog {
        writes: Writes,
        forms: Rc<RefCell<Vec<u32>>>,
        counts: Rc<RefCell<Vec<(u32, u32)>>>,
    }

    fn record_buffer_saves(e: &mut Engine) -> BufferLog {
        let log = BufferLog {
            writes: Rc::new(RefCell::new(Vec::new())),
            forms: Rc::new(RefCell::new(Vec::new())),
            counts: Rc::new(RefCell::new(Vec::new())),
        };
        let writes = log.writes.clone();
        e.register_double(0x0086_5e50, move |e, w| {
            let bytes = e.mem.bytes(w[1], w[2]);
            writes.borrow_mut().push((w[1], bytes));
            eax(0)
        });
        let forms = log.forms.clone();
        e.register_double(0x0086_5df0, move |_, w| {
            forms.borrow_mut().push(w[1]);
            eax(0)
        });
        let next = Rc::new(RefCell::new(0x100u32));
        e.register_double(0x0086_5f20, move |_, _| {
            *next.borrow_mut() += 1;
            eax(*next.borrow())
        });
        let counts = log.counts.clone();
        e.register_double(0x0086_5ff0, move |_, w| {
            counts.borrow_mut().push((w[1], w[2]));
            eax(0)
        });
        log
    }

    #[test]
    fn the_buffer_save_writes_the_arrays_the_forms_and_the_counted_lists() {
        let mut e = engine();
        list_helpers(&mut e);
        let player = new_player(&mut e);
        let p = player.addr();
        let log = record_buffer_saves(&mut e);
        e.mem.set_f32(p + 0x244 + 4, 2.5);
        e.mem.set_u32(p + 0x6b8, 0xa1);
        e.mem.set_u32(p + 0x208, 0xa2);
        // One topic the check accepts and one it rejects.
        let second = node(&mut e, 0xc2, 0);
        e.mem.set_u32(p + 0x6a8, 0xc1);
        e.mem.set_u32(p + 0x6ac, second);
        e.register(0x0061_9410, |_, a| eax(u32::from(a[0] == 0xc2)));
        // One quest-log entry.
        e.mem.set_u32(p + 0x6b0, 0xd1);
        double(&mut e, 0x005e_3fa0, eax(0xd2));
        double(&mut e, 0x0060_f1a0, eax(1));
        double(&mut e, 0x005d_c980, eax(2));
        // One target, whose word after the item is 0x44.
        let target = e.mem.alloc(8);
        e.mem.set_u32(target + 4, 0x44);
        e.mem.set_u32(p + 0x6bc, target);
        double(&mut e, 0x0044_edb0, eax(0xe2));
        // The id words of the 8 forms.
        e.register(0x004b_fb30, |_, a| eax(if a[1] == 3 { 0xf3 } else { 0 }));
        e.register(FORM_ID_OF, |_, a| eax(a[0] + 1));
        let buffer = 0x0600_4000;
        start_log(&mut e);
        fn_009590f0(&mut e, player, buffer);
        let writes = log.writes.borrow();
        // 3 * 0x4d modifier words come first, then +0x4ac.
        assert_eq!(writes[1].1, 2.5f32.to_le_bytes());
        assert_eq!(writes[0x4d * 3].0, p + 0x4ac);
        // The first form ids: the active quest then the form at +0x73c.
        let forms = log.forms.borrow();
        assert_eq!(forms[0], 0xa1);
        assert!(forms.contains(&0xa2));
        // The accepted topic is counted and written.
        assert!(forms.contains(&0xc1));
        assert!(!forms.contains(&0xc2));
        let counts = log.counts.borrow();
        assert_eq!(counts[0].0, 1);
        // The quest log entry has the quest's id and the two bytes.
        assert!(forms.contains(&0xd2));
        assert!(writes.iter().any(|w| w.1 == vec![1u8]));
        // The target is its id and the word after the item.
        assert!(forms.contains(&0xe2));
        assert!(writes.iter().any(|w| w.1 == 0x44u32.to_le_bytes()));
        // The eight form ids are written; the one found is its id.
        assert!(writes.iter().any(|w| w.1 == 0xf4u32.to_le_bytes()));
        // The pending references close the save.
        assert_eq!(calls_to(&e, 0x005a_a930), vec![vec![buffer]]);
    }

    /// A load buffer object whose version (slot 0) is `version`.
    fn load_buffer(e: &mut Engine, version: u32) -> u32 {
        let buffer = e.mem.alloc(8);
        slot(e, buffer, 0, eax(version));
        buffer
    }

    #[test]
    fn the_buffer_load_reads_the_arrays_and_rebuilds_the_topics() {
        let mut e = engine();
        list_helpers(&mut e);
        let player = new_player(&mut e);
        let p = player.addr();
        let buffer = load_buffer(&mut e, 0x1b);
        let reads: Rc<RefCell<Vec<(u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let record = reads.clone();
        e.register_double(0x0086_4980, move |_, w| {
            record.borrow_mut().push((w[1], w[2]));
            eax(0)
        });
        // The counts: the first list counted is the topics (2 entries).
        let counts = Rc::new(RefCell::new(vec![2u32]));
        let queue = counts.clone();
        e.register_double(0x0086_4a60, move |_, _| {
            let mut queue = queue.borrow_mut();
            eax(if queue.is_empty() { 0 } else { queue.remove(0) })
        });
        let next_id = Rc::new(RefCell::new(1u32));
        let ids = next_id.clone();
        e.register_double(0x0086_48a0, move |_, _| {
            *ids.borrow_mut() += 1;
            eax(*ids.borrow())
        });
        e.register(0x0048_39c0, |_, a| eax(a[0] + 0x1000));
        e.register(DYNAMIC_CAST, |_, a| eax(a[0]));
        let appended = Rc::new(RefCell::new(Vec::<(u32, u32)>::new()));
        let seen = appended.clone();
        e.register_double(0x005a_e3d0, move |e, w| {
            seen.borrow_mut().push((w[0], e.mem.u32(w[1])));
            eax(0)
        });
        start_log(&mut e);
        fn_0095a3b0(&mut e, player, buffer);
        let reads = reads.borrow();
        assert_eq!(reads[0], (p + 0x244, 4));
        assert_eq!(reads[0x4d], (p + 0x378, 4));
        assert_eq!(reads[0x4d * 3], (p + 0x4ac, 4));
        // The quest, the weapon, the marker, the region and its data take
        // ids 2 to 6; the topics follow.
        assert_eq!(e.mem.u32(p + 0x6b8), 0x1002);
        assert_eq!(e.mem.u32(p + 0x73c), 0x1003);
        assert_eq!(e.mem.u32(p + 0x760), 0x1005);
        assert_eq!(
            *appended.borrow(),
            vec![(p + 0x6a8, 0x1007), (p + 0x6a8, 0x1008)]
        );
        assert_eq!(calls_to(&e, 0x0061_a5a0).len(), 1);
        // The versions after 0x1a close with the pending references.
        assert_eq!(calls_to(&e, 0x005a_aaf0), vec![vec![buffer]]);
        assert_eq!(calls_to(&e, 0x0094_62c0), vec![vec![p, 0, 1]]);
    }

    #[test]
    fn the_buffer_load_of_an_old_version_stops_early() {
        let mut e = engine();
        list_helpers(&mut e);
        let player = new_player(&mut e);
        let buffer = load_buffer(&mut e, 5);
        e.register(0x0048_39c0, |_, a| eax(a[0]));
        e.register(DYNAMIC_CAST, |_, a| eax(a[0]));
        start_log(&mut e);
        fn_0095a3b0(&mut e, player, buffer);
        assert!(calls_to(&e, 0x005a_aaf0).is_empty());
        assert!(calls_to(&e, 0x008d_1d10).is_empty());
        assert!(calls_to(&e, 0x0096_9e90).is_empty());
        assert_eq!(calls_to(&e, 0x0094_62c0).len(), 1);
    }

    #[test]
    fn the_buffer_load_rebuilds_the_quest_log_the_effect_lists_and_the_hardcore_flag() {
        let mut e = engine();
        list_helpers(&mut e);
        let player = new_player(&mut e);
        let p = player.addr();
        slot(&mut e, p, 0x1d0, eax(0));
        let buffer = load_buffer(&mut e, 0x1b);
        // Every list holds one entry.
        double(&mut e, 0x0086_4a60, eax(1));
        e.register(0x0086_48a0, |_, _| eax(0x40));
        e.register(0x0048_39c0, |_, a| eax(a[0] + 0x1000));
        e.register(DYNAMIC_CAST, |_, a| eax(a[0]));
        double(&mut e, 0x0060_db40, eax(0x9100));
        double(&mut e, 0x0060_f2c0, eax(0x9200));
        double(&mut e, 0x0060_c8e0, eax(0x9300));
        e.register(OPERATOR_NEW, |e, a| eax(e.mem.alloc(a[0].max(0x20))));
        e.register(0x006a_7ad0, |e, _| eax(e.mem.alloc(4)));
        for ctor in [
            0x0096_a2d0u32,
            0x0047_81b0,
            0x0078_d900,
            0x0073_3f50,
            0x0059_a370,
            0x0076_b630,
        ] {
            e.register(ctor, |_, a| eax(a[0]));
        }
        let appended = Rc::new(RefCell::new(Vec::<(u32, u32)>::new()));
        for address in [0x005a_e3d0u32, 0x0090_5820] {
            let seen = appended.clone();
            e.register_double(address, move |e, w| {
                seen.borrow_mut().push((w[0], e.mem.u32(w[1])));
                eax(0)
            });
        }
        // Reading the hardcore bytes: both come back as 1.
        e.register(0x0086_4980, |e, w| {
            if w[2] == 1 {
                e.mem.set_u8(w[1], 1);
            }
            eax(0)
        });
        start_log(&mut e);
        fn_0095a3b0(&mut e, player, buffer);
        let appended = appended.borrow();
        assert!(appended.contains(&(p + 0x6b0, 0x9200)));
        assert!(appended.contains(&(p + 0x6bc, 0x9300)));
        // The heap lists were created for the objects that were read.
        assert_ne!(e.mem.u32(p + 0xd48), 0);
        assert_ne!(e.mem.u32(p + 0x610), 0);
        assert_ne!(e.mem.u32(p + 0x614), 0);
        assert_ne!(e.mem.u32(p + 0x618), 0);
        // Hardcore mode: first byte 1, enabled byte 1: no further call.
        assert_eq!(calls_to(&e, 0x0096_9e90), vec![vec![p, 1, 1]]);
        assert!(calls_to(&e, 0x005d_e9b0).is_empty());
        // The effect items were built with the 0xbc-byte constructor.
        assert_eq!(calls_to(&e, 0x0059_a370).len(), 2);
    }

    #[test]
    fn the_buffer_load_finish_resolves_forms_and_starts_the_heartbeat() {
        let mut e = engine();
        list_helpers(&mut e);
        float_settings(&mut e);
        let player = new_player(&mut e);
        let p = player.addr();
        let buffer = load_buffer(&mut e, 0x12);
        e.set_global(STATISTICS_OBJECT, 0x77u32);
        e.register(0x0084_aa90, |_, a| eax(a[1] + 1));
        e.register(0x0048_39c0, |_, a| eax(a[0] + 0x1000));
        e.register(DYNAMIC_CAST, |_, a| eax(a[0]));
        e.mem.set_u32(p + 0x208, 5);
        e.mem.set_u32(p + 0x224, 7);
        e.mem.set_u32(p + 0x638, 9);
        e.mem.set_u32(p + 0xd2c, 0x20);
        // The health ratio is 2 / 10 = 0.2: between the two settings.
        let owner = p + 0xa4;
        slot(&mut e, owner, 0xc, st0(2.0));
        slot(&mut e, owner, 0x20, st0(10.0));
        slot(&mut e, p, 0x1d0, eax(0));
        e.set_global(HEARTBEAT_UPPER_SETTING, 0.5f32);
        e.set_global(HEARTBEAT_LOWER_SETTING, 0.1f32);
        // The list at +0x60c has one target to flag.
        let target = e.mem.alloc(16);
        e.mem.set_u32(target + 8, 0x30);
        let list = node(&mut e, target, 0);
        e.mem.set_u32(p + 0x60c, list);
        start_log(&mut e);
        fn_0095c0a0(&mut e, player, buffer);
        assert_eq!(e.mem.u32(p + 0x208), 0x1006);
        assert_eq!(e.mem.u32(p + 0x224), 0x1008);
        assert_eq!(e.mem.u32(p + 0x638), 0x1009);
        assert_eq!(e.mem.u32(p + 0xd2c), 0x1020);
        assert_eq!(e.mem.u32(target + 8), 0x1030);
        assert_eq!(calls_to(&e, 0x0056_4db0), vec![vec![0x1030, 1]]);
        let sounds = calls_to(&e, 0x00ad_7550);
        assert_eq!(sounds.len(), 1);
        assert_eq!(sounds[0][2], HEARTBEAT_SOUND_ALP);
        assert_eq!(calls_to(&e, 0x00ad_8830), vec![vec![p + 0x77c, 1]]);
        // A ratio below the lower setting plays the other sound; above the
        // upper one, none.
        slot(&mut e, owner, 0xc, st0(0.5));
        start_log(&mut e);
        fn_0095c0a0(&mut e, player, buffer);
        assert_eq!(calls_to(&e, 0x00ad_7550)[0][2], HEARTBEAT_SOUND_BLP);
        slot(&mut e, owner, 0xc, st0(9.0));
        start_log(&mut e);
        fn_0095c0a0(&mut e, player, buffer);
        assert!(calls_to(&e, 0x00ad_7550).is_empty());
    }

    #[test]
    fn the_buffer_load_finish_purges_the_item_list_and_flags_the_perks() {
        let mut e = engine();
        list_helpers(&mut e);
        float_settings(&mut e);
        let player = new_player(&mut e);
        let p = player.addr();
        let buffer = load_buffer(&mut e, 5);
        e.register(0x0084_aa90, |_, _| eax(0));
        e.register(0x0048_39c0, |_, a| eax(a[0]));
        e.register(DYNAMIC_CAST, |_, a| {
            eax(if a[0] == 0x31 { 0 } else { a[0] })
        });
        // The list at +0xd48: two entries, the first resolves to nothing.
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, 0x31);
        let second_item = e.mem.alloc(8);
        e.mem.set_u32(second_item, 0x32);
        let second = node(&mut e, second_item, 0);
        let head = node(&mut e, first, second);
        e.mem.set_u32(p + 0xd48, head);
        double(&mut e, 0x007a_f430, eax(1));
        e.register(0x0063_f7b0, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let item = e.mem.u32(next);
            let after = e.mem.u32(next + 4);
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, after);
            eax(0)
        });
        // A perk entry (form, rank).
        slot(&mut e, p + 0xa4, 0xc, st0(1.0));
        slot(&mut e, p + 0xa4, 0x20, st0(1.0));
        slot(&mut e, p, 0x1d0, eax(0));
        let perk = e.mem.alloc(8);
        e.mem.set_u32(perk, 0x66);
        e.mem.set_u8(perk + 4, 3);
        e.mem.set_u32(p + 0x87c, perk);
        start_log(&mut e);
        fn_0095c0a0(&mut e, player, buffer);
        assert_eq!(calls_to(&e, 0x0040_1030), vec![vec![first]]);
        assert_eq!(calls_to(&e, 0x0063_f7b0), vec![vec![head]]);
        assert_eq!(e.mem.u32(second_item), 0x32);
        assert_eq!(calls_to(&e, 0x005e_b980), vec![vec![0x66, p, 3, 0]]);
        assert_eq!(calls_to(&e, 0x0080_6b00), vec![vec![buffer, 0]]);
    }

    #[test]
    fn the_buffer_load_end_restores_the_camera_the_spring_and_the_combat_state() {
        let mut e = engine();
        let player = new_player(&mut e);
        let p = player.addr();
        let buffer = e.mem.alloc(8);
        double(&mut e, 0x0042_8110, eax(buffer));
        e.register(0x0042_80f0, |_, a| eax(u32::from(a[1] == 0x10)));
        double(&mut e, 0x0059_8040, st0(75.0));
        double(&mut e, 0x0095_0bb0, eax(0));
        double(&mut e, 0x005b_b4d0, eax(1));
        e.mem.set_u8(p + 0x64a, 1);
        e.mem.set_u32(p + 0x638, 0x77);
        e.mem.set_u32(p + 0x63c, 2);
        e.mem.set_u32(p + 0x6b8, 0x31);
        e.set_global(INVENTORY_ALPHA, 0.5f32);
        double(&mut e, 0x008b_bc10, eax(1));
        let holder = e.mem.alloc(8);
        double(&mut e, 0x008d_8520, eax(holder));
        let reset = recording_slot(&mut e, holder, 0x400, eax(0));
        let end = recording_slot(&mut e, p, 0x3f4, eax(0));
        double(&mut e, 0x0093_ccd0, eax(0x500));
        double(&mut e, 0x0082_2b90, eax(1));
        double(&mut e, 0x0070_5990, eax(0x600));
        start_log(&mut e);
        double(&mut e, 0x0095_2290, eax(0));
        fn_0095c730(&mut e, player, buffer);
        assert_eq!(calls_to(&e, 0x0056_7490), vec![vec![p, 75.0f32.to_bits()]]);
        assert_eq!(e.mem.u8(p + 0x64c), 1);
        assert_eq!(e.global::<f32>(INVENTORY_ALPHA), 1.0);
        assert_eq!(calls_to(&e, 0x008c_4640), vec![vec![p]]);
        // 0095c7ee: the zoom factor and model orientation (00952290), not UpdateFirstPersonZoom.
        assert_eq!(calls_to(&e, 0x0095_2290), vec![vec![p]]);
        assert!(calls_to(&e, 0x0095_0290).is_empty());
        assert_eq!(calls_to(&e, 0x0094_66d0).len(), 1);
        assert_eq!(calls_to(&e, 0x0095_f930).len(), 1);
        assert_eq!(
            calls_to(&e, 0x0060_f110),
            vec![vec![0x31, p + 0x6c4, p + 0x6bc]]
        );
        assert_eq!(*reset.borrow(), vec![vec![holder, 0]]);
        assert_eq!(calls_to(&e, 0x008b_b650), vec![vec![p, 1, 0, 0]]);
        // The spell target check: the pipboy light flashes twice.
        assert_eq!(calls_to(&e, 0x0082_2b90), vec![vec![p + 0x94, 0x518, 1]]);
        assert_eq!(calls_to(&e, 0x0070_5990), vec![Vec::<u32>::new(); 2]);
        assert_eq!(
            calls_to(&e, 0x007f_a310),
            vec![vec![0x600, 1, 1, 1], vec![0x600, 0, 1, 1]]
        );
        assert_eq!(end.borrow().len(), 1);
    }

    #[test]
    fn the_revert_resets_the_player_and_rebuilds_the_topic_list() {
        let mut e = engine();
        list_helpers(&mut e);
        float_settings(&mut e);
        let player = new_player(&mut e);
        let p = player.addr();
        let buffer = e.mem.alloc(8);
        e.mem.set_u32(buffer, 0);
        e.set_global(MINUS_ONE_FLOAT, -1.0f32);
        e.set_global(DEFAULT_TIMER_VALUE, 5.0f32);
        e.set_global(VIEW_OFFSET_SETTING_A, 1.5f32);
        e.set_global(VIEW_OFFSET_SETTING_B, 2.5f32);
        e.set_global(SAVED_FORM_011E0784, 9u32);
        e.mem.set_u8(p + 0x64a, 1);
        e.mem.set_u32(p + 0x208, 4);
        e.mem.set_f32(p + 0x244 + 8, 3.0);
        e.mem.set_f32(p + 0x4ac, 3.0);
        // The list at +0x60c has one object; the lists +0x87c and +0xad4 too.
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x69);
        e.mem.set_u32(p + 0x60c, object);
        e.mem.set_u32(p + 0x87c, 0x71);
        e.mem.set_u32(p + 0xad4, 0x72);
        e.mem.set_u32(p + 0x1ec, 0x73);
        // The generic list objects.
        e.register(0x0063_f7b0, |e, a| {
            e.mem.set_u32(a[0], 0);
            eax(0)
        });
        // The topics come from a list the quest code gives; one is accepted.
        e.set_global(0x011c_3f2cu32, 0x0600_9000u32);
        let list = node(&mut e, 0x81, 0);
        double(&mut e, 0x0046_12e0, eax(list));
        double(&mut e, 0x0061_9410, eax(1));
        let appended = record_appends(&mut e);
        start_log(&mut e);
        fn_0095c9c0(&mut e, player, buffer);
        assert_eq!(e.mem.u8(p + 0x64a), 0);
        assert_eq!(e.mem.u32(p + 0x208), 0);
        assert_eq!(e.global::<u32>(SAVED_FORM_011E0784), 0);
        assert_eq!(e.mem.f32(p + 0x244 + 8), 0.0);
        assert_eq!(e.mem.f32(p + 0x4ac), 0.0);
        assert_eq!(e.mem.u8(p + 0x66e), 1);
        assert_eq!(e.mem.f32(p + 0x670), 1.5);
        assert_eq!(e.mem.f32(p + 0x674), 2.5);
        assert_eq!(e.mem.f32(p + 0x684), 5.0);
        assert_eq!(e.global::<f32>(INVENTORY_ALPHA_SAVE), -1.0);
        // The list object at +0x60c was freed with its list.
        assert_eq!(e.mem.u32(p + 0x60c), 0);
        assert!(calls_to(&e, 0x0040_1030).contains(&vec![0x69]));
        assert!(calls_to(&e, 0x0040_1030).contains(&vec![0x71]));
        assert!(calls_to(&e, 0x0040_1030).contains(&vec![0x72]));
        assert!(calls_to(&e, 0x0040_1030).contains(&vec![0x73]));
        assert_eq!(e.mem.u32(p + 0x1ec), 0);
        assert_eq!(e.mem.u8(p + 0xe38), 1);
        // The cleared lists: 0x4a extra heads each for the perk lists.
        assert!(calls_to(&e, LIST_CLEAR).contains(&vec![p + 0x884]));
        assert!(calls_to(&e, LIST_CLEAR).contains(&vec![p + 0xadc + 0x49 * 8]));
        // The topics are rebuilt and the marker removed.
        assert_eq!(*appended.borrow(), vec![(p + 0x6a8, 0x81)]);
        assert_eq!(calls_to(&e, 0x005a_ae20), vec![vec![buffer]]);
    }

    // ---- the exe's stack frames ----

    /// The local the next `with_stack(size)` on this thread takes (the frame of a function the
    /// test calls directly, outside any other local), with the call log switched on. With the
    /// block check on (the tests that read a frame after its call ended) locals are heap blocks
    /// again, and the next one of that size is the block just freed.
    fn next_frame(e: &mut Engine, size: u32) -> u32 {
        let frame = if e.mem.block_check_on() {
            let block = e.mem.alloc(size);
            e.mem.free(block);
            block
        } else {
            e.next_stack_local(size)
        };
        start_log(e);
        frame
    }

    /// `EBP` of an exe frame that aligns it to 16 bytes, `below` bytes of locals under it.
    fn aligned_ebp(raw: u32, below: u32) -> u32 {
        (raw + below + 0xf) & !0xf
    }

    #[test]
    fn the_spell_check_code_is_at_frame_offset_8() {
        let mut e = engine();
        let player = new_player(&mut e);
        double(&mut e, IS_GOD_MODE, eax(0));
        let frame = next_frame(&mut e, 0x10);
        fn_0094c800(&mut e, player.cast(), 1, 2, Ptr::NULL, 3);
        assert_eq!(
            calls_to(&e, SPELL_CHECK),
            vec![vec![player.addr(), 1, 2, frame + 8, 3]]
        );
    }

    #[test]
    fn the_parent_cell_handler_keeps_its_locals_under_an_aligned_ebp() {
        let mut e = engine();
        let (player, _) = cell_environment(&mut e, 0x1111);
        double(&mut e, 0x0054_4c30, eax(2));
        double(&mut e, 0x0054_4c60, eax(3));
        let raw = next_frame(&mut e, 0x280 + 0x10);
        fn_0094cae0(&mut e, player, Ptr::new(0x5000));
        let ebp = aligned_ebp(raw, 0x280);
        let corners: Vec<u32> = calls_to(&e, 0x004f_7070).iter().map(|w| w[0]).collect();
        assert_eq!(
            corners,
            vec![ebp - 0x1ac, ebp - 0x1b4, ebp - 0x1bc, ebp - 0x1c4]
        );
        assert_eq!(calls_to(&e, 0x0096_a2d0), vec![vec![ebp - 0x1cc]]);
        // The music pair is the by-value argument copy under the pushed 1.
        assert_eq!(calls_to(&e, 0x0040_c0e0)[0][0], ebp - 0x280);
        // The distance check copies the position to EBP-0x234.
        let mut e = engine();
        let (player, world) = border_environment(&mut e);
        e.mem.set_u32(player.addr() + 0x7ac, world);
        double(&mut e, FORM_TYPE_OF, eax(0x41));
        let raw = next_frame(&mut e, 0x280 + 0x10);
        fn_0094cae0(&mut e, player, Ptr::new(0x5000));
        let ebp = aligned_ebp(raw, 0x280);
        assert_eq!(calls_to(&e, 0x0045_78c0)[0][0], ebp - 0x234);
        assert_eq!(calls_to(&e, 0x0045_7990), vec![vec![ebp - 0x234]]);
    }

    #[test]
    fn returning_keeps_the_stored_position_and_the_zero_vectors_in_the_frame() {
        let mut e = engine();
        // The stored position is read after the call ended, and the position functions it calls
        // next take their own locals where the frame was on the thread's stack: locals are heap
        // blocks with the block check on.
        e.mem.enable_block_check(false, false);
        let player = new_player(&mut e);
        e.mem.set_f32(player.addr() + 0x7a0, 10.0);
        e.mem.set_u32(player.addr() + 0x7ac, 0x6000);
        e.register(0x0040_3df0, |_, a| eax(a[0] + 4));
        let wanted = e.mem.alloc(12);
        double(&mut e, 0x0043_0830, eax(wanted));
        // A cell: the stored position at EBP-0x58 (+0xc of the 0x64-byte frame).
        double(&mut e, FORM_TYPE_OF, eax(0x39));
        let frame = next_frame(&mut e, 0x64);
        e.mem.set_f32(frame + 0xc, 0.0);
        player_character_return_to_last_known_good_position(&mut e, player, true);
        assert_eq!(e.mem.f32(frame + 0xc), 10.0);
        // A world space: at EBP-0x4c (+0x18).
        double(&mut e, FORM_TYPE_OF, eax(0x41));
        let frame = next_frame(&mut e, 0x64);
        e.mem.set_f32(frame + 0x18, 0.0);
        player_character_return_to_last_known_good_position(&mut e, player, true);
        assert_eq!(e.mem.f32(frame + 0x18), 10.0);
        // Directly: the zero vectors of the two bodies at EBP-0x20 and EBP-0x14.
        let bodies = [0xb100, 0xb200];
        e.register_double(0x0095_0bb0, move |_, a| eax(bodies[a[1] as usize]));
        let frame = next_frame(&mut e, 0x64);
        player_character_return_to_last_known_good_position(&mut e, player, false);
        let zeros: Vec<u32> = calls_to(&e, 0x0043_d410).iter().map(|w| w[0]).collect();
        assert_eq!(zeros, vec![frame + 0x44, frame + 0x50]);
        assert_eq!(
            calls_to(&e, 0x00a5_9c60),
            vec![vec![0xb100, frame + 0x44], vec![0xb200, frame + 0x50]]
        );
    }

    #[test]
    fn loading_the_3d_keeps_its_locals_at_the_exe_offsets() {
        let mut e = engine();
        let world = load_world(&mut e, vec![]);
        let player = world.player;
        e.set(player, PlayerCharacter::b3rdPerson, false);
        let frame = next_frame(&mut e, 0x21c);
        player_character_load_3d(&mut e, player, 5);
        assert_eq!(calls_to(&e, 0x0040_4eb0)[0][0], frame + 0x1dc);
        assert_eq!(calls_to(&e, 0x0040_4ee0), vec![vec![frame + 0x1dc]]);
        assert_eq!(calls_to(&e, 0x0043_d410)[0][0], frame + 0x1c4);
        assert_eq!(
            calls_to(&e, 0x0056_fac0),
            vec![vec![player.addr(), frame + 0x54, frame + 0x1e0]]
        );
        assert_eq!(calls_to(&e, 0x0040_6d30)[0][0], frame + 0xb0);
        assert_eq!(calls_to(&e, 0x0070_c440), vec![vec![0x6600, frame + 0x40]]);
    }

    #[test]
    fn the_inventory_callback_line_is_at_frame_offset_4() {
        let mut e = engine();
        list_helpers(&mut e);
        let item = e.mem.alloc(0x20);
        let output = e.mem.alloc(8);
        let frame = next_frame(&mut e, 0x114);
        build_inv_string_callback(&mut e, Ptr::new(item), Ptr::new(output));
        assert_eq!(calls_to(&e, SPRINTF)[0][0], frame + 4);
        assert_eq!(calls_to(&e, STRING_APPEND), vec![vec![output, frame + 4]]);
    }

    #[test]
    fn the_progress_export_keeps_its_locals_at_the_exe_offsets() {
        let mut e = engine();
        let world = export_world(&mut e);
        let player = world.player;
        // One perk and one quest, for one line of each.
        let perk = e.mem.alloc(0x20);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, perk);
        e.mem.set_u32(player.addr() + 0x87c, entry);
        let inventory = e.mem.alloc(0x40);
        double(&mut e, 0x004b_f220, eax(inventory));
        let quest_item = e.mem.alloc(8);
        let quest = named_object(&mut e);
        double(&mut e, 0x0060_fd70, eax(1));
        double(&mut e, 0x005e_3fa0, eax(quest));
        e.mem.set_u32(player.addr() + 0x6b0, quest_item);
        let frame = next_frame(&mut e, 0x3ac);
        player_character_export_progress_data(&mut e, player, 0);
        let lines: Vec<u32> = calls_to(&e, SPRINTF).iter().map(|w| w[0]).collect();
        assert_eq!(lines, vec![frame + 0x280, frame + 0x158, frame + 0x40]);
        assert_eq!(calls_to(&e, 0x0040_37b0), vec![vec![frame + 0x38c]]);
        assert_eq!(calls_to(&e, 0x0040_37d0), vec![vec![frame + 0x38c]]);
        assert_eq!(
            calls_to(&e, 0x0093_abe0),
            vec![vec![player.addr(), frame + 0x26c, frame + 0x274]]
        );
    }

    #[test]
    fn the_karma_sound_handles_use_the_frame_pair_of_their_branch() {
        // A loss: the handle at EBP-0x20 (+0x40), the temporary at EBP-0x38 (+0x28).
        let mut e = engine();
        let (player, _) = karma_world(&mut e, 10);
        let frame = next_frame(&mut e, 0x60);
        player_character_reward_karma(&mut e, player, -5);
        assert_eq!(calls_to(&e, 0x0041_a250), vec![vec![frame + 0x40]]);
        assert_eq!(calls_to(&e, 0x00ad_7550)[0][1], frame + 0x28);
        assert_eq!(calls_to(&e, 0x00ad_8830), vec![vec![frame + 0x40, 0]]);
        // A gain: EBP-0x2c (+0x34) and EBP-0x44 (+0x1c).
        let mut e = engine();
        let (player, _) = karma_world(&mut e, 10);
        let frame = next_frame(&mut e, 0x60);
        player_character_reward_karma(&mut e, player, 5);
        assert_eq!(calls_to(&e, 0x0041_a250), vec![vec![frame + 0x34]]);
        assert_eq!(calls_to(&e, 0x00ad_7550)[0][1], frame + 0x1c);
    }

    #[test]
    fn resurrecting_builds_the_controller_word_at_frame_offset_0x44() {
        let mut e = engine();
        let player = new_player(&mut e);
        slot(&mut e, player.addr(), 0x1c4, eax(0));
        let high = e.mem.alloc(16);
        slot(&mut e, high, 4, eax(0));
        double(&mut e, 0x008d_7510, eax(high));
        double(&mut e, 0x0093_06d0, eax(0x0600_0700));
        let setting = word_block(&mut e, &[0]);
        double(&mut e, SETTING_FLOAT_POINTER, eax(setting));
        double(&mut e, OPERATOR_NEW, eax(0x0600_0900));
        let frame = next_frame(&mut e, 0x60);
        player_character_resurrect(&mut e, player, 0, 0, 0);
        assert_eq!(
            calls_to(&e, 0x0070_c440),
            vec![vec![0x0600_0700, frame + 0x44]]
        );
        assert_eq!(calls_to(&e, 0x004a_3a20), vec![vec![frame + 0x44]]);
    }

    #[test]
    fn the_inventory_clone_keeps_its_locals_at_the_exe_offsets() {
        let mut e = engine();
        let (player, _, _) = clone_scenario(&mut e, true);
        let frame = next_frame(&mut e, 0x36c);
        player_character_clone_inventory_3d(&mut e, player, 1);
        let pointers: Vec<u32> = calls_to(&e, 0x0063_3c90).iter().map(|w| w[0]).collect();
        assert_eq!(pointers, vec![frame + 0x348, frame + 0x354]);
        assert_eq!(calls_to(&e, 0x004a_d050)[0][0], frame + 0xc8);
        assert_eq!(calls_to(&e, 0x004a_d270), vec![vec![frame + 0xc8]]);
        assert_eq!(
            calls_to(&e, 0x0056_fac0),
            vec![vec![player.addr(), frame + 0x80, frame + 0x204]]
        );
        let paths = calls_to(&e, 0x0040_6d00);
        assert_eq!((paths[0][0], paths[1][0]), (frame + 0xf8, frame + 0x238));
        let words = calls_to(&e, 0x0090_5820);
        assert_eq!((words[0][1], words[1][1]), (frame + 0xb4, frame + 0xb0));
        assert_eq!(
            calls_to(&e, 0x0045_cec0),
            vec![vec![frame + 0x354], vec![frame + 0x348]]
        );
    }

    #[test]
    fn the_3d_switch_keeps_its_locals_at_the_exe_offsets() {
        // Showing the own 3D: the sound handle at EBP-0x6c (+0x60 of the 0xcc-byte frame).
        let mut e = engine();
        let player = switch_scenario(&mut e);
        let frame = next_frame(&mut e, 0xcc);
        fn_00951a10(&mut e, player, 0);
        assert_eq!(calls_to(&e, 0x0041_a250), vec![vec![frame + 0x60]]);
        assert_eq!(calls_to(&e, 0x0048_3710)[0], vec![frame + 0x60]);
        // Showing the first person model: the rotation at +0x84, the rotated matrix at +0x20,
        // the zero vector at +0x6c and the sound handle at EBP-0x54 (+0x78).
        let mut e = engine();
        let player = switch_scenario(&mut e);
        let frame = next_frame(&mut e, 0xcc);
        fn_00951a10(&mut e, player, 1);
        assert_eq!(calls_to(&e, 0x0068_15c0), vec![vec![frame + 0x84]]);
        assert_eq!(
            calls_to(&e, 0x0056_fac0),
            vec![vec![player.addr(), frame + 0x20, frame + 0x84]]
        );
        assert_eq!(calls_to(&e, 0x0043_d410)[0][0], frame + 0x6c);
        assert_eq!(calls_to(&e, 0x0041_a250), vec![vec![frame + 0x78]]);
    }

    #[test]
    fn the_zoom_matrices_and_vectors_are_at_the_exe_offsets() {
        let mut e = engine();
        let player = zoom_scenario(&mut e);
        e.set_global(ZOOM_FACTOR, 0.5f32);
        let frame = next_frame(&mut e, 0x134);
        fn_00952290(&mut e, player);
        let (a, b, c, d) = (frame + 0x110, frame + 0x74, frame + 0xa8, frame + 0xcc);
        let (up, flat) = (frame + 0x9c, frame + 0xf8);
        assert_eq!(
            calls_to(&e, 0x0068_15c0),
            vec![vec![a], vec![b], vec![flat], vec![c], vec![d]]
        );
        assert_eq!(
            calls_to(&e, 0x0043_f8d0),
            vec![vec![a, frame + 0x44, b], vec![d, frame + 0x14, c]]
        );
        assert_eq!(calls_to(&e, 0x0041_6870)[0][0], up);
        assert_eq!(calls_to(&e, 0x004b_4500), vec![vec![a, frame + 0x38, up]]);
        assert_eq!(calls_to(&e, 0x004b_6190), vec![vec![up, flat]]);
    }

    #[test]
    fn the_list_adders_pass_their_own_argument_slot() {
        let mut e = engine();
        let player = new_player(&mut e);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(0x005a_e3d0, move |e, a| {
            log.borrow_mut().push((a[1], e.mem.u32(a[1])));
            Ret::default()
        });
        // 00952830: `SUB ESP,0x8`, the slot at +0x10.
        let frame = next_frame(&mut e, 0x14);
        assert!(fn_00952830(&mut e, player, 0xc0c, 0, 1));
        // 009528c0: `SUB ESP,0x10` and `ESI`, the slot at +0x1c.
        let quest_frame = next_frame(&mut e, 0x20);
        player_character_add_quest_stage_item(&mut e, player, 0xb0b);
        // 00952a20: `SUB ESP,0x18`, the slot at +0x20.
        double(&mut e, 0x005d_43e0, eax(0x0600_0100));
        let target_frame = next_frame(&mut e, 0x24);
        fn_00952a20(&mut e, player, 0xa0a);
        assert_eq!(
            *seen.borrow(),
            vec![
                (frame + 0x10, 0xc0c),
                (quest_frame + 0x1c, 0xb0b),
                (target_frame + 0x20, 0xa0a)
            ]
        );
    }

    #[test]
    fn dropping_keeps_its_locals_in_the_0x40_byte_frame() {
        // With a body: the zero point at +0x10, the position at +0x1c, the difference at +4.
        let mut e = engine();
        // The reference word is read after the call ended (on the thread's stack the calls after
        // the frame take its place): locals are heap blocks with the block check on.
        e.mem.enable_block_check(false, false);
        let player = new_player(&mut e);
        e.set_global(PLAYER_POINTER, 0x0600_5000u32);
        let item = e.mem.alloc(8);
        slot(&mut e, item, 0xe4, eax(0));
        double(&mut e, FORM_TYPE_OF, eax(0x11));
        let reference = e.mem.alloc(0x20);
        slot(&mut e, reference, 0x1d0, eax(0x900));
        let position = e.mem.alloc(12);
        slot(&mut e, reference, 0x1f4, eax(position));
        recording_slot(&mut e, player.addr(), 0x17c, eax(reference));
        let frame = next_frame(&mut e, 0x40);
        fn_00954610(&mut e, player, item, 0, 0, 0, 0);
        assert_eq!(calls_to(&e, 0x0043_d410)[0][0], frame + 0x10);
        assert_eq!(calls_to(&e, 0x00a5_9c60), vec![vec![0x900, frame + 0x10]]);
        assert_eq!(calls_to(&e, 0x0043_9ef0)[0][..2], [frame + 0x1c, frame + 4]);
        assert_eq!(
            calls_to(&e, 0x0063_c8a0),
            vec![vec![frame + 0x1c, frame + 4]]
        );
        // Without one: the reference word at EBP-0x8 (+0x38).
        slot(&mut e, reference, 0x1d0, eax(0));
        let frame = next_frame(&mut e, 0x40);
        fn_00954610(&mut e, player, item, 0, 0, 0, 0);
        assert_eq!(calls_to(&e, 0x005a_e3d0)[0][1], frame + 0x38);
        assert_eq!(e.mem.u32(frame + 0x38), reference);
    }

    #[test]
    fn the_old_save_keeps_its_locals_in_the_0x228_byte_frame() {
        let mut e = engine();
        save_settings(&mut e);
        list_helpers(&mut e);
        double(&mut e, 0x0082_5c00, eax(0x0600_2000));
        let player = new_player(&mut e);
        let p = player.addr();
        e.set_global(SAVE_LOAD_GAME, 0x1111u32);
        version(&mut e, 0x6f);
        double(&mut e, 0x004b_9ba0, eax(0x0600_2100));
        e.register(0x0096_a400, |e, a| {
            e.mem.set_u32(a[1], 0);
            Ret::default()
        });
        let frame = next_frame(&mut e, 0x228);
        fn_00955620(&mut e, player, 0);
        // The position copy at EBP-0x40, the block at EBP-0xf0.
        assert!(calls_to(&e, 0x0048_4ce0).contains(&vec![p, frame + 0x1e8, 0xc]));
        assert_eq!(calls_to(&e, 0x004d_5f60), vec![vec![frame + 0x138]]);
        assert!(calls_to(&e, 0x0048_4ce0).contains(&vec![p, frame + 0x138, 0xac]));
        // The cursor at EBP-0x168, the value at EBP-0x174, the flag at EBP-0x175.
        assert_eq!(
            calls_to(&e, 0x0096_a400),
            vec![vec![p + 0x854, frame + 0xc0, frame + 0xb4, frame + 0xb3]]
        );
    }

    #[test]
    fn the_old_load_keeps_its_locals_in_the_0x32c_byte_frame() {
        let mut e = engine();
        save_settings(&mut e);
        let player = new_player(&mut e);
        e.set_global(SAVE_LOAD_GAME, 0x1111u32);
        version(&mut e, 0x39);
        e.register(0x0048_39c0, |_, a| eax(a[0] + 0x2000));
        e.register(DYNAMIC_CAST, |_, a| eax(a[0]));
        let frame = next_frame(&mut e, 0x32c);
        fn_00956f70(&mut e, player, 0, 0x55);
        // The position at EBP-0x38, the block at EBP-0xfc, the name at EBP-0x2ec.
        assert_eq!(calls_to(&e, LIST_NODE_ITEM_SLOT)[0], vec![frame + 0x2f4]);
        assert_eq!(calls_to(&e, 0x004d_5f20), vec![vec![frame + 0x230]]);
        assert_eq!(calls_to(&e, 0x0048_9100)[0][1], frame + 0x40);
    }

    #[test]
    fn the_second_load_pass_keeps_its_list_words_in_the_frame() {
        let mut e = engine();
        // The removed id is read after the call ended (on the thread's stack the calls after the
        // frame take its place): locals are heap blocks with the block check on.
        e.mem.enable_block_check(false, false);
        list_helpers(&mut e);
        let player = new_player(&mut e);
        let process = e.mem.alloc(8);
        e.set(player, PlayerCharacter::pCurrentProcess, Ptr::new(process));
        slot(&mut e, process, 0x58, eax(0));
        e.set_global(SAVE_LOAD_GAME, 0x1111u32);
        version(&mut e, 0x73);
        // The first entry resolves, the second does not and is removed after it.
        e.mem.set_u32(0x011e_0ae8, 0xb0b);
        let second = node(&mut e, 0xa0a, 0);
        e.mem.set_u32(0x011e_0aec, second);
        e.register(0x0048_39c0, |_, a| eax(a[0]));
        e.register(DYNAMIC_CAST, |_, a| {
            eax(if a[0] == 0xa0a { 0 } else { a[0] })
        });
        e.register(0x0090_5330, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let after = e.mem.u32(next + 4);
            e.mem.set_u32(a[0] + 4, after);
            eax(0)
        });
        let frame = next_frame(&mut e, 0x6c);
        fn_00958990(&mut e, player, 0, 0);
        // The looked-up form at EBP-0x58 (+0x14) for both entries, the removed id at EBP-0x60
        // (+0xc).
        let words: Vec<u32> = calls_to(&e, 0x0072_6c60).iter().map(|w| w[1]).collect();
        assert_eq!(words, vec![frame + 0x14; 2]);
        assert_eq!(
            calls_to(&e, 0x0090_5330),
            vec![vec![0x011e_0ae8, frame + 0xc]]
        );
        assert_eq!(e.mem.u32(frame + 0xc), 0xa0a);
    }

    #[test]
    fn the_buffer_save_keeps_its_locals_in_the_0x13c_byte_frame() {
        let mut e = engine();
        list_helpers(&mut e);
        let player = new_player(&mut e);
        record_buffer_saves(&mut e);
        e.register(FORM_ID_OF, |_, a| eax(a[0] + 1));
        let buffer = 0x0600_4000;
        let frame = next_frame(&mut e, 0x13c);
        fn_009590f0(&mut e, player, buffer);
        // The flag holder at EBP-0x124, the position copy at EBP-0xc.
        assert_eq!(calls_to(&e, 0x0042_8110), vec![vec![buffer, frame + 0x18]]);
        assert!(calls_to(&e, 0x0086_5e50).contains(&vec![buffer, frame + 0x130, 0xc, 0]));
    }

    #[test]
    fn the_buffer_load_keeps_its_locals_in_the_0x200_byte_frame() {
        let mut e = engine();
        list_helpers(&mut e);
        let player = new_player(&mut e);
        let buffer = load_buffer(&mut e, 0x1b);
        e.register(0x0048_39c0, |_, a| eax(a[0] + 0x1000));
        e.register(DYNAMIC_CAST, |_, a| eax(a[0]));
        let frame = next_frame(&mut e, 0x200);
        fn_0095a3b0(&mut e, player, buffer);
        // The flag holder at EBP-0x134, the position at EBP-0x1c, the hardcore bytes at
        // EBP-0x11d and EBP-0x11e.
        assert_eq!(calls_to(&e, 0x0042_8110), vec![vec![buffer, frame + 0xcc]]);
        let reads = calls_to(&e, 0x0086_4980);
        assert!(reads.contains(&vec![buffer, frame + 0x1e4, 0xc]));
        let bytes: Vec<u32> = reads.iter().filter(|w| w[2] == 1).map(|w| w[1]).collect();
        let at = bytes.iter().position(|b| *b == frame + 0xe3).unwrap();
        assert_eq!(bytes[at + 1], frame + 0xe2);
    }

    #[test]
    fn the_buffer_load_passes_keep_their_flag_holders_in_their_frames() {
        // 0095c0a0 (0xa0 bytes): the holders at EBP-0x5c (+0x44) and EBP-0x60 (+0x40).
        let mut e = engine();
        list_helpers(&mut e);
        float_settings(&mut e);
        let player = new_player(&mut e);
        let p = player.addr();
        let buffer = load_buffer(&mut e, 5);
        e.register(0x0084_aa90, |_, _| eax(0));
        e.register(0x0048_39c0, |_, a| eax(a[0]));
        e.register(DYNAMIC_CAST, |_, a| eax(a[0]));
        slot(&mut e, p + 0xa4, 0xc, st0(1.0));
        slot(&mut e, p + 0xa4, 0x20, st0(1.0));
        slot(&mut e, p, 0x1d0, eax(0x77));
        let frame = next_frame(&mut e, 0xa0);
        fn_0095c0a0(&mut e, player, buffer);
        assert_eq!(calls_to(&e, 0x0042_8110), vec![vec![buffer, frame + 0x44]]);
        assert_eq!(calls_to(&e, 0x0042_ce30), vec![vec![buffer, frame + 0x40]]);

        // 0095c730 (0x14 bytes): the holder at EBP-0x4 (+0x10).
        let mut e = engine();
        let player = new_player(&mut e);
        let buffer = e.mem.alloc(8);
        double(&mut e, 0x0095_0bb0, eax(0));
        let holder = e.mem.alloc(8);
        double(&mut e, 0x008d_8520, eax(holder));
        slot(&mut e, holder, 0x400, eax(0));
        slot(&mut e, player.addr(), 0x3f4, eax(0));
        let frame = next_frame(&mut e, 0x14);
        double(&mut e, 0x0095_2290, eax(0));
        fn_0095c730(&mut e, player, buffer);
        assert_eq!(calls_to(&e, 0x0042_8110), vec![vec![buffer, frame + 0x10]]);
    }

    #[test]
    fn the_revert_keeps_its_flag_holders_in_the_0x44_byte_frame() {
        let mut e = engine();
        list_helpers(&mut e);
        float_settings(&mut e);
        let player = new_player(&mut e);
        let buffer = e.mem.alloc(8);
        e.set_global(0x011c_3f2cu32, 0x0600_9000u32);
        let list = node(&mut e, 0x81, 0);
        double(&mut e, 0x0046_12e0, eax(list));
        e.register(0x0063_f7b0, |e, a| {
            e.mem.set_u32(a[0], 0);
            eax(0)
        });
        let frame = next_frame(&mut e, 0x44);
        fn_0095c9c0(&mut e, player, buffer);
        // The holders at EBP-0x20 (+0x24) and EBP-0x24 (+0x20).
        assert_eq!(
            calls_to(&e, 0x0042_ce30),
            vec![vec![buffer, frame + 0x24], vec![buffer, frame + 0x20]]
        );
    }
}
