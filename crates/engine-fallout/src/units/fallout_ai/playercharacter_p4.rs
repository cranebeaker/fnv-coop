//! `fallout/ai/playercharacter.cpp` (Xbox PDB source unit), part 4: its functions from `00967aa0` up to
//! (not including) `ffffffff` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::playercharacter`]; anything public there may be used here.
//!
//! This session covers `00967aa0` to `0096a2b0` (40 functions): the
//! `PlayerCharacter` accessors, the combat group clean-up `00967da0`, the
//! RockIt launcher ammunition list (`AddRockItAmmo`, `RemoveRockItAmmo`,
//! `ClearRockItAmmo`, ...), the two package starters `00969260` and
//! `009694d0`, the audio marker lookup, the hardcore mode update and the
//! sleep/wait test `00969fa0`.
//!
//! Second session: `00967b70` (marker added) and `0096a2d0` to `0096a7b0` (the
//! hash table, array and list constructors and destructors). The range is
//! finished; nothing is left after `0096a7b0`.
//!
//! Conventions:
//! * In the range this part touches, `PlayerCharacter` fields sit 0x10 lower on
//!   PC than in the Xbox PDB (`TESForm` is 0x10 smaller); every offset below was
//!   confirmed by the PC code, and the PDB name is used where the use fits it.
//! * The small accessors the game's code calls everywhere are called by their
//!   address and named for what they do here: `00559450` (word at +0, the
//!   extra data list head of an `ItemChange`), `00726070` (word at +4, the next
//!   list node or the number of an `ItemChange`), `0044ddc0` (word at +8, the
//!   form of an `ItemChange` or the item count of a `BSSimpleArray`),
//!   `006815c0` (a list node's own address: the item is the word there),
//!   `00877a30` (element address of a `BSSimpleArray` of pointers).
//! * The decompiler hangs pushed words on the wrong call in this unit, so every
//!   call was read from the disassembly. C++ exception unwinding (the
//!   `FS:[0]` frames of the functions that allocate) is not translated.
//! * x87: values stored as `float` are rounded to `f32`; the arithmetic is
//!   done in `f64`.

#[allow(unused_imports)]
use super::playercharacter::*;
use crate::prelude::*;
use crate::types::{BSSimpleArray, BSSimpleList};
use crate::units::fallout_shared::inventorychanges::ItemChange;

// ---------------------------------------------------------------------------
// Globals

/// `PlayerCharacter *` (the player).
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// `ProcessLists` (the object itself).
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// The calendar singleton.
const CALENDAR: u32 = 0x011d_e7b8;
/// The pointer to the combat manager's array of combat groups: the object
/// `00968650` (count) and `00968670` (element) work on, and `CombatManager`
/// for `CreateCombatGroup`.
const COMBAT_MANAGER: u32 = 0x011f_1958;
/// `CombatDialogueManager *`.
const COMBAT_DIALOGUE_MANAGER: u32 = 0x011f_1708;
/// The byte `00969ac0` sets and `00969ae0` clears (the flag `00969b80` tests).
const TIMER_ACTIVE_FLAG: u32 = 0x011e_07ab;
/// The time in seconds at which the timer was started (`00969b00` clears it,
/// `00969b20` sets it).
const TIMER_START_SECONDS: u32 = 0x011e_07b4;
/// `UpdateHardcoreMode`'s function-local `static float`s: the calendar value
/// seen at the last update for each of the three needs (actor values 0x49,
/// 0x4b and 0x4a), and the current ones.
const LAST_VALUE_49: u32 = 0x011e_0d7c;
const LAST_VALUE_4A: u32 = 0x011e_0d78;
const LAST_VALUE_4B: u32 = 0x011e_0d74;
const NOW_VALUE_49: u32 = 0x011e_0d80;
const NOW_VALUE_4A: u32 = 0x011e_0d84;
const NOW_VALUE_4B: u32 = 0x011e_0d88;

// Setting objects (a `Setting` keeps its value at +4; `00403e20` gives the
// address of a float value, `0043d4d0` of an integer value, `00403df0` the
// string pointer).
const SETTING_COMBAT_RADIUS: u32 = 0x011c_e3a8;
const SETTING_LIMIT_49: u32 = 0x011d_0c48;
const SETTING_LIMIT_4B: u32 = 0x011d_1440;
const SETTING_LIMIT_4A: u32 = 0x011d_1014;
const SETTING_TIMER_SECONDS: u32 = 0x011d_3570;
const SETTING_TODDLER_TEXT: u32 = 0x011c_dd78;
/// The message settings of `00969fa0`, in the order the function tests the
/// reasons.
const MESSAGE_FIRST: u32 = 0x011d_4474;
const MESSAGE_SECOND: u32 = 0x011d_28b0;
const MESSAGE_THIRD: u32 = 0x011d_4948;
const MESSAGE_FOURTH: u32 = 0x011d_2e50;
const MESSAGE_FIFTH: u32 = 0x011d_3e20;
const MESSAGE_SIXTH: u32 = 0x011d_20ac;
const MESSAGE_SEVENTH: u32 = 0x011d_4b10;
const MESSAGE_EIGHTH: u32 = 0x011d_47d4;

/// `0.0` (`double`).
const DOUBLE_ZERO: u32 = 0x0101_2060;
/// Divisor (`double`) of the hardcore need limits (`00969c30`).
const HARDCORE_DIVISOR: u32 = 0x0101_2638;
/// The icon path `Interface\Icons\Message Icons\glow_message_vault`.
const MESSAGE_ICON: u32 = 0x0102_08a0;
/// The `float` shown with each message (`00969fa0`).
const MESSAGE_TIME: u32 = 0x0101_62c0;
/// The `float` `00969fa0` gives `885520`.
const SLEEP_TEST_DISTANCE: u32 = 0x0108_4d20;
/// Start value of the nearest-marker search of `00969930` (a `float`).
const NEAREST_MARKER_START: u32 = 0x0101_6970;
/// Start value of the nearest-actor search of `00967da0` (a `float`).
const NEAREST_ACTOR_START: u32 = 0x0108_a8a0;
/// The vtable `0096a280` installs after `0096a390` built the base.
const VTABLE_0096A280: u32 = 0x0108_b474;

// ---------------------------------------------------------------------------
// Functions of other units

/// Imported Windows time function (`[00fdf060]`; milliseconds).
const TIME_IMPORT: u32 = 0x00fd_f060;
/// `operator new(size)` (cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `_ftol2_sse` (`00ec62c0`): the value comes as a leading `f64`.
const FLOAT_TO_INT: u32 = 0x00ec_62c0;
/// Word at +0 / +4 / +8 of an object (see the module notes).
const WORD_AT_0: u32 = 0x0055_9450;
const WORD_AT_4: u32 = 0x0072_6070;
const WORD_AT_8: u32 = 0x0044_ddc0;
/// A list node's own address (`006815c0`).
const NODE_ITEM_SLOT: u32 = 0x0068_15c0;
/// Element address of a `BSSimpleArray` of pointers (`00877a30`: `this[+4] +
/// index * 4`).
const ARRAY_ELEMENT_SLOT: u32 = 0x0087_7a30;
/// The form type byte at +4 of a form (`00401170`).
const FORM_TYPE: u32 = 0x0040_1170;
/// `Setting` accessors: float value address, integer value address, string.
const SETTING_FLOAT_POINTER: u32 = 0x0040_3e20;
const SETTING_INT_POINTER: u32 = 0x0043_d4d0;
const SETTING_STRING: u32 = 0x0040_3df0;
/// `00464f30(text, unused)`: returns its first argument (cdecl).
const PASS_THROUGH: u32 = 0x0046_4f30;
/// The 16-bit word at +0xa of an object (`00658930`).
const WORD_AT_0XA: u32 = 0x0065_8930;

// The animation chain of `IsPipboyActive`.
/// The word at `+0xe0 + 4 * index` of an animation, with index 0x14 mapped to
/// 1 and 0x15 to 4 (`00491040`, one word).
const ANIMATION_SLOT: u32 = 0x0049_1040;
/// The word at +0x74 of the object (`0048f7f0`; the engine map's name,
/// `Animation::ZeroGlobalTransform`, is a folded one).
const ANIMATION_WORD_AT_0X74: u32 = 0x0048_f7f0;
/// The animation group id of the object (`005f2420`).
const ANIMATION_GROUP_ID: u32 = 0x005f_2420;
/// `Interface::IsInPipboyMenu` (Xbox PDB).
const IS_IN_PIPBOY_MENU: u32 = 0x0070_5a00;

// The inventory entry test `00967b70`.
const EXTRA_GET_CAN_NOT_WEAR: u32 = 0x0041_8b10;
/// `BaseExtraList::GetExtraData(type)` (Xbox PDB).
const EXTRA_GET_BY_TYPE: u32 = 0x0041_0220;
/// `TESHealthForm::GetFormHealth` (Xbox PDB), cdecl.
const GET_FORM_HEALTH: u32 = 0x0048_73d0;
/// Test on a form of type 0x18 or 0x28 (`0047bcf0`).
const FORM_TEST_47BCF0: u32 = 0x0047_bcf0;
/// Test of a type 0x1a form against an extra data list (`0080f7d0`).
const FORM_TEST_80F7D0: u32 = 0x0080_f7d0;
/// `Actor::GetCurrentWeapon` (Xbox PDB).
const GET_CURRENT_WEAPON: u32 = 0x008a_1710;
/// First and second form of the block at `weapon + 0xa4` (`00474a00`,
/// `00474a40`).
const WEAPON_FIRST_FORM: u32 = 0x0047_4a00;
const WEAPON_SECOND_FORM: u32 = 0x0047_4a40;
/// `BGSListForm::GetItemIndex` (Xbox PDB).
const LIST_FORM_GET_ITEM_INDEX: u32 = 0x0058_ff60;

// Combat group and process lists.
/// `ProcessLists` lookup that returns a list of entries (`00971fd0`, one
/// `float`).
const PROCESS_LISTS_ENTRIES: u32 = 0x0097_1fd0;
/// The member count of a combat group (`00990890`: the count of the
/// `MemberArray` at +0x18; the map's name `CPropertySection::GetCount` is a
/// library guess).
const GROUP_MEMBER_COUNT: u32 = 0x0099_0890;
/// The actor of member `index` of a combat group (`00903600`).
const GROUP_MEMBER_AT: u32 = 0x0090_3600;
/// The target count of a combat group (`005a4320`: the count of the
/// `TargetArray` at +8).
const GROUP_TARGET_COUNT: u32 = 0x005a_4320;
/// `MiddleHighProcess::GetForceNextUpdate` (Xbox PDB), asked of an actor.
const GET_FORCE_NEXT_UPDATE: u32 = 0x0056_6950;
/// `CombatManager::CreateCombatGroup` (Xbox PDB): `ECX` = the manager.
const CREATE_COMBAT_GROUP: u32 = 0x0099_1e80;
/// Copies the targets of a combat group into another (`00986740`).
const GROUP_COPY_TARGETS: u32 = 0x0098_6740;
/// `CombatGroup::IsTarget` (Xbox PDB).
const GROUP_IS_TARGET: u32 = 0x0098_65b0;
/// `Actor::GetShouldAttackActor` (Xbox PDB): `ECX` = the actor, four words.
const GET_SHOULD_ATTACK_ACTOR: u32 = 0x008b_06d0;
/// Subtracts a position from another (`00439ef0`): `ECX` = the minuend, the
/// address of the result and the subtrahend.
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// Length of the vector in `ECX` (`004a7290`, returned in ST0).
const POINT_LENGTH: u32 = 0x004a_7290;
/// `CombatGroup::RemoveTarget` (Xbox PDB).
const GROUP_REMOVE_TARGET: u32 = 0x0098_6500;
/// `CombatGroup::AddMember` (Xbox PDB).
const GROUP_ADD_MEMBER: u32 = 0x0098_67d0;
/// `Actor::IsAngryWithPlayer` (Xbox PDB).
const IS_ANGRY_WITH_PLAYER: u32 = 0x008b_ffc0;
/// `CombatDialogueManager::StartDialogue_ov2` (Xbox PDB).
const START_DIALOGUE: u32 = 0x0098_39b0;
/// Sets the byte at +0x125 of the object (`005c1ae0`).
const SET_BYTE_AT_0X125: u32 = 0x005c_1ae0;
/// The embedded node of `ProcessLists` that heads the list of actors
/// (`0096e290`: `this + 0x80`).
const PROCESS_LISTS_ACTOR_NODE: u32 = 0x0096_e290;
/// `MobileObject::GetCurrentPackage` (Xbox PDB).
const GET_CURRENT_PACKAGE: u32 = 0x0093_44a0;
/// The package type byte at +0x20 of a package (`0041ca90`, sign-extended).
const PACKAGE_TYPE: u32 = 0x0041_ca90;
/// Updates of packages of type 0x18 (`009f84d0`) and 0x16 (`009f1350`): `ECX`
/// = the package, the player.
const PACKAGE_0X18_UPDATE: u32 = 0x009f_84d0;
const PACKAGE_0X16_UPDATE: u32 = 0x009f_1350;
/// `Actor::EvaluatePackage` (Xbox PDB): two words.
const EVALUATE_PACKAGE: u32 = 0x008a_6ce0;
/// A `CombatGroup` method given an actor (`009869a0`, `ECX` = the player's
/// group).
const GROUP_METHOD_9869A0: u32 = 0x0098_69a0;
/// Handles one entry of the list `971fd0` gave (`008bcbd0`): `ECX` = the
/// actor (the word at +8 of the entry), the entry.
const HANDLE_ENTRY: u32 = 0x008b_cbd0;
/// The scalar deleting destructor of such an entry (`008f25e0`).
const ENTRY_DESTROY: u32 = 0x008f_25e0;
/// Unlinks the node holding `*item`, starting from the node it is called on
/// (`00905330`).
const LIST_REMOVE: u32 = 0x0090_5330;
/// Removes the first node of a list by moving the next node's content into
/// it (`0063f7b0`).
const LIST_POP_FRONT: u32 = 0x0063_f7b0;
/// `BSSimpleList` head test (`008256d0`): both words zero.
const LIST_IS_EMPTY: u32 = 0x0082_56d0;

// The ammunition list.
/// `BSSimpleArray::IsEmpty` (`0076b610`: the count is zero).
const ARRAY_IS_EMPTY: u32 = 0x0076_b610;
/// A random number (`00487f50`).
const RANDOM: u32 = 0x0048_7f50;
/// `ItemChange::ItemChange` (Xbox PDB): `ECX` = the block, the form and the
/// number.
const ITEM_CHANGE_CONSTRUCTOR: u32 = 0x004b_c550;
/// `ItemChange::DeleteAllExtra` (Xbox PDB).
const ITEM_CHANGE_DELETE_ALL_EXTRA: u32 = 0x004b_c780;
/// Stores the word at +4 of an `ItemChange`, its number (`006ecd40`).
const SET_NUMBER: u32 = 0x006e_cd40;
/// `BSSimpleArray<ItemChange *>::Remove(index, count)` (Xbox PDB).
const ARRAY_REMOVE: u32 = 0x006b_f8f0;
/// Appends the pointer at the given address to a `BSSimpleArray` (`007cb2e0`).
const ARRAY_APPEND: u32 = 0x007c_b2e0;
/// Empties a `BSSimpleArray` (`008454f0`, one flag word).
const ARRAY_CLEAR: u32 = 0x0084_54f0;
/// Adds the item at the given address to a `BSSimpleList` (`005ae3d0`).
const LIST_ADD: u32 = 0x005a_e3d0;
/// `BSSimpleList` node constructor (`0096a2d0`).
const LIST_NODE_CONSTRUCT: u32 = 0x0096_a2d0;
/// `ExtraDataList::ExtraDataList` (Xbox PDB).
const EXTRA_DATA_LIST_CONSTRUCT: u32 = 0x0041_0360;
/// `ExtraDataList::DuplicateExtraListForContainer` (Xbox PDB).
const EXTRA_DATA_LIST_DUPLICATE: u32 = 0x0041_2380;
/// `ExtraDataList::IsExtraDefaultforContainer` (Xbox PDB).
const EXTRA_IS_DEFAULT_FOR_CONTAINER: u32 = 0x0041_d120;
/// `ExtraDataList::CompareListForContainer` (Xbox PDB).
const EXTRA_COMPARE_LIST_FOR_CONTAINER: u32 = 0x0041_26c0;
/// `ExtraDataList::GetCount` / `SetCount` (Xbox PDB).
const EXTRA_GET_COUNT: u32 = 0x0041_8770;
const EXTRA_SET_COUNT: u32 = 0x0041_9ad0;
/// `TESWeightForm::GetFormWeight` (Xbox PDB), cdecl: the form and a flag.
const GET_FORM_WEIGHT: u32 = 0x0048_ebc0;
/// The flag byte `GetFormWeight` is given (`004d1360`, on the player).
const WEIGHT_FLAG_OF_PLAYER: u32 = 0x004d_1360;
/// `InventoryChanges::GetInventoryChanges` (Xbox PDB), cdecl: the actor.
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
/// Moves an `ItemChange` into an `InventoryChanges` (`004c3380`): `ECX` = the
/// changes, the item and the delete flag.
const INVENTORY_ADD_ITEM_CHANGE: u32 = 0x004c_3380;

// The package starters.
/// `TESPackage::CreatePackage` (Xbox PDB), cdecl.
const CREATE_PACKAGE: u32 = 0x0067_0b90;
/// Package flag setters (`00826b40`, `00826b90`).
const PACKAGE_SET_FLAG_2: u32 = 0x0082_6b40;
const PACKAGE_SET_FLAG_4: u32 = 0x0082_6b90;
/// `TESPackage::SetPackageLocation` (Xbox PDB) and `SetPackageTarget`.
const SET_PACKAGE_LOCATION: u32 = 0x0067_1d30;
const SET_PACKAGE_TARGET: u32 = 0x0067_2fc0;
/// The package's target object (`00671d10`: the word at `+0x30`).
const PACKAGE_TARGET_OBJECT: u32 = 0x0067_1d10;
/// `PackageLocation::PackageLocation` (Xbox PDB), a `PackageLocation` method
/// called with 0 (`0067f140`), `SetLocReference` and the destructor
/// (`00670b30`, delete flag).
const PACKAGE_LOCATION_CONSTRUCTOR: u32 = 0x0067_f030;
const PACKAGE_LOCATION_METHOD_67F140: u32 = 0x0067_f140;
const SET_LOC_REFERENCE: u32 = 0x0067_f3c0;
const PACKAGE_LOCATION_DESTRUCTOR: u32 = 0x0067_0b30;
/// `PackageTarget::PackageTarget`, `SetTargType`, `SetTargReference` (Xbox
/// PDB), the word store at +8 (`00403550`) and the destructor (`007b3fa0`).
const PACKAGE_TARGET_CONSTRUCTOR: u32 = 0x0067_ff70;
const SET_TARG_TYPE: u32 = 0x0068_00b0;
const SET_TARG_REFERENCE: u32 = 0x0068_0110;
const SET_TARGET_WORD_AT_8: u32 = 0x0040_3550;
const PACKAGE_TARGET_DESTRUCTOR: u32 = 0x007b_3fa0;
/// Sets the word at +0x18 of a package (`00984f60`).
const SET_PACKAGE_WORD_AT_0X18: u32 = 0x0098_4f60;
/// The actor's process (`008d8520`: the word at `+0x68`).
const ACTOR_PROCESS: u32 = 0x008d_8520;
/// `ProcessLists` call that `00969260` stores at +0x1fc (`00973710`: the
/// player and a flag).
const PROCESS_LISTS_QUEUE_TARGET: u32 = 0x0097_3710;
/// A call on the player with one word (`005cc7a0`) and
/// `PlayerCharacter::ForceTemp3rdPerson` (Xbox PDB, `00950340`).
const PLAYER_SET_MODE_5CC7A0: u32 = 0x005c_c7a0;
const FORCE_TEMP_3RD_PERSON: u32 = 0x0095_0340;
/// Test that ends `009694d0` early (`008a8170`) and the closing call of
/// `00969260` (`008a8060`).
const REFERENCE_EARLY_OUT_TEST: u32 = 0x008a_8170;
const REFERENCE_AFTER_PACKAGE: u32 = 0x008a_8060;
/// `PlayerCharacter` helper `0093a740` (one of four flag bytes at
/// +0x798 / +0x799 / +0x79a / +0x79b is set).
const PLAYER_ANY_FLAG: u32 = 0x0093_a740;

// Small tests and actions.
/// `PlayerCharacter::GetAnimation` (Xbox PDB), one flag word.
const PLAYER_GET_ANIMATION: u32 = 0x0095_0a60;
/// A call on the player taking an animation and a name (`008b73f0`).
const PLAYER_PLAY_NAMED: u32 = 0x008b_73f0;
/// Called by the "is young" setter (`008b78c0`, one word) and `008d3fa0`.
const ACTOR_REFRESH_8B78C0: u32 = 0x008b_78c0;
const ACTOR_REFRESH_8D3FA0: u32 = 0x008d_3fa0;
/// Distance helpers of the audio marker search: squared length (`00595c80`)
/// and the radius of a reference (`00568cb0`).
const POINT_LENGTH_SQUARED: u32 = 0x0059_5c80;
const REFERENCE_RADIUS: u32 = 0x0056_8cb0;
/// Looks an item up through the word at +0x24 of a marker (`004839c0`,
/// cdecl).
const AUDIO_MARKER_LOOKUP: u32 = 0x0048_39c0;
/// Whether the list at the first argument holds the item at the second
/// (`005f65d0`).
const LIST_CONTAINS: u32 = 0x005f_65d0;
/// A flag setter on a caravan card (`006e2b50`, one byte).
const SET_CARD_FLAG: u32 = 0x006e_2b50;
/// Hardcore mode: the test `5a03f0(4)` on the player, `5a1e50` and the
/// reset `009590d0`.
const PLAYER_IS_IN_STATE: u32 = 0x005a_03f0;
const PLAYER_NEEDS_TEST: u32 = 0x005a_1e50;
const PLAYER_RESET_NEEDS: u32 = 0x0095_90d0;
/// `Calendar::GetTimeScale` and the calendar's day-time getter (`00867ea0`).
const CALENDAR_GET_TIME_SCALE: u32 = 0x0086_7950;
const CALENDAR_GET_VALUE: u32 = 0x0086_7ea0;
/// The extra data type 0x15 getter (`00418520`) and the reset `00963b00`.
const EXTRA_GET_TYPE_0X15: u32 = 0x0041_8520;
const RESET_TYPE_0X15: u32 = 0x0096_3b00;
/// The reference's `ExtraDataList` (`005d43c0`).
const EXTRA_LIST_OF_REFERENCE: u32 = 0x005d_43c0;
/// Clears the byte at +0xe38 of the player (`005de9b0`) and the byte global
/// `007d6e60` returns.
const CLEAR_BYTE_AT_0XE38: u32 = 0x005d_e9b0;
const GLOBAL_FLAG_7D6E60: u32 = 0x007d_6e60;
/// Message output (`007052f0`, cdecl, six words) and `Interface::CreateSleepMenu`
/// (Xbox PDB, cdecl).
const SHOW_MESSAGE: u32 = 0x0070_52f0;
const CREATE_SLEEP_MENU: u32 = 0x0070_54f0;
/// The tests of the sleep/wait function `00969fa0` (all unnamed in the map).
const SLEEP_TEST_953C80: u32 = 0x0095_3c80;
const ACTOR_PROCESS_OF_THIS: u32 = 0x008d_6f30;
const SLEEP_TEST_885520: u32 = 0x0088_5520;
const REFERENCE_IS_INTERIOR: u32 = 0x0057_5d10;
const PROCESS_LISTS_TEST_9764A0: u32 = 0x0097_64a0;
const GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
const CONTROLLER_STATE: u32 = 0x005c_0880;
const CELL_TEST_5444C0: u32 = 0x0054_44c0;
const SLEEP_TEST_944360: u32 = 0x0094_4360;
const FURNITURE_TEST_5098E0: u32 = 0x0050_98e0;
const MAGIC_TEST_822E00: u32 = 0x0082_2e00;
/// The predicted position (`00968690`): the position (`00436aa0`), its
/// character controller's linear velocity (`0066ca00`), the scaling of a
/// vector (`0045bb20`) and the vector sum (`0063c8a0`).
const POSITION_OF: u32 = 0x0043_6aa0;
const CONTROLLER_LINEAR_VELOCITY: u32 = 0x0066_ca00;
const VECTOR_SCALE: u32 = 0x0045_bb20;
const VECTOR_ADD: u32 = 0x0063_c8a0;
/// `HUDMainMenu::SetRadiationLevel` (Xbox PDB), cdecl, two floats.
const SET_RADIATION_LEVEL: u32 = 0x0077_38c0;
/// `operator delete` of an object (`00401030`, cdecl, one word).
const FREE_OBJECT: u32 = 0x0040_1030;
/// Allocates `bytes` bytes (`00aa1070`, cdecl).
const ALLOCATE_BYTES: u32 = 0x00aa_1070;
/// Allocates `count` words (`0096afc0`, cdecl).
const ALLOCATE_WORDS: u32 = 0x0096_afc0;
/// Frees a block from `00aa1070` (`00aa10f0`, cdecl; ignores 0).
const FREE_BYTES: u32 = 0x00aa_10f0;
/// `memset(block, value, size)` (`00403d30`, cdecl).
const FILL_MEMORY: u32 = 0x0040_3d30;
/// Frees all items of a hash table (`00438af0`).
const HASH_TABLE_EMPTY: u32 = 0x0043_8af0;
/// `006b8310(item)` on the sub-object at +0xc of `0096a540`'s object.
const SUB_OBJECT_RELEASE: u32 = 0x006b_8310;
/// Array initialiser `006b3eb0(size, capacity)`.
const ARRAY_INITIALISE_SIZED: u32 = 0x006b_3eb0;
/// `SetAtGrow` (`00470000`): index, value.
const ARRAY_SET_AT_GROW: u32 = 0x0047_0000;
/// Vtables installed by the constructors and destructors of this range.
const MAP_VTABLE: u32 = 0x0108_b474;
const HASH_TABLE_VTABLE: u32 = 0x0108_b494;
const ITEM_CHANGE_ARRAY_VTABLE: u32 = 0x0108_b4b4;
const AMMO_ARRAY_VTABLE: u32 = 0x0108_b4c8;
const TABLE_ARRAY_DERIVED_VTABLE: u32 = 0x0108_b4dc;
const CAPACITY_ARRAY_VTABLE: u32 = 0x0108_b4e4;

layout! {
    /// `PlayerCharacter` (Xbox PDB source `fallout/ai/playercharacter.cpp`),
    /// 0xE5C bytes on the Xbox; the PC size is not established, the code of
    /// this part reaches +0xE39. The fields of this part, at their PC
    /// offsets (0x10 lower than the PDB's, which are named in the comments).
    pub struct PlayerCharacter: 0xe5c {
        /// The actor's process, the word at +0x68 (`008d8520` returns it).
        0x068 pProcess: Ptr,
        /// The word `00969260` stores the result of `00973710` in (the PDB
        /// has `pQueuedTargetLoc` at +0x1fc; the shift does not apply to
        /// this one).
        0x1fc field_01fc: u32,
        /// A pointer whose object has a word at +0x94 (the PDB has
        /// `pCurrentSpell` at +0x224).
        0x224 field_0224: Ptr,
        /// `bSpeaking` (Xbox PDB +0x618): the setter `00967ac0` stores the
        /// byte as given.
        0x608 bSpeaking: u8,
        /// `pInactiveListofCaravanCards` (Xbox PDB +0x624):
        /// `BSSimpleList<TESCaravanCard *>*`.
        0x614 pInactiveListofCaravanCards: Ptr,
        /// `pActiveListofCaravanCards` (Xbox PDB +0x628).
        0x618 pActiveListofCaravanCards: Ptr,
        /// `p1stPersonAnimation` (Xbox PDB +0x6a0): `Animation*`.
        0x690 p1stPersonAnimation: Ptr,
        /// `eHardcoreSetting` (Xbox PDB +0x7cc).
        0x7bc eHardcoreSetting: u32,
        /// `bIsYoung` (Xbox PDB +0x7d5).
        0x7c5 bIsYoung: u8,
        /// `bIsToddler` (Xbox PDB +0x7d6).
        0x7c6 bIsToddler: u8,
        /// `AudioMarkerList` (Xbox PDB +0x7e4): `BSSimpleList<AudioMarkerInfo *>`,
        /// the head node embedded.
        0x7d4 AudioMarkerList: Inline<BSSimpleList>,
        /// `pClosestAudioMarkerInfo` (Xbox PDB +0x7ec).
        0x7dc pClosestAudioMarkerInfo: Ptr,
        /// `pCombatGroup` (Xbox PDB +0xd74).
        0xd64 pCombatGroup: Ptr,
        /// `bPlayerInCombat` (Xbox PDB +0xe00).
        0xdf0 bPlayerInCombat: u8,
        /// `RockItLauncherAmmoList` (Xbox PDB +0xe04):
        /// `BSSimpleArray<ItemChange *, 1024>`.
        0xdf4 RockItLauncherAmmoList: Inline<BSSimpleArray>,
        /// `fRockItLauncherAmmoWeight` (Xbox PDB +0xe14).
        0xe04 fRockItLauncherAmmoWeight: f32,
        /// `bAlwaysHardcore` (Xbox PDB +0xe48), cleared by `005de9b0`.
        0xe38 bAlwaysHardcore: u8,
        /// `bResetHardcoreTimers` (Xbox PDB +0xe49).
        0xe39 bResetHardcoreTimers: u8,
    }
}

// ---------------------------------------------------------------------------
// Helpers

/// The item of a `BSSimpleList` node (`006815c0` gives the node's own
/// address; the item is the word there).
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(NODE_ITEM_SLOT, &args![node]).u32();
    e.mem.u32(slot)
}

/// The next node of a `BSSimpleList` node (`00726070`).
fn node_next(e: &mut Engine, node: u32) -> u32 {
    e.call(WORD_AT_4, &args![node]).u32()
}

/// The count of a `BSSimpleArray` (`0044ddc0`: the word at +8).
fn array_count(e: &mut Engine, array: u32) -> u32 {
    e.call(WORD_AT_8, &args![array]).u32()
}

/// Element `index` of a `BSSimpleArray` of pointers (`00877a30`).
fn array_item(e: &mut Engine, array: u32, index: u32) -> u32 {
    let slot = e.call(ARRAY_ELEMENT_SLOT, &args![array, index]).u32();
    e.mem.u32(slot)
}

/// The number of an `ItemChange` (`00726070`: the word at +4).
fn item_number(e: &mut Engine, item: u32) -> u32 {
    e.call(WORD_AT_4, &args![item]).u32()
}

/// The `float` setting stored in the setting object (`00403e20`).
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let at = e.call(SETTING_FLOAT_POINTER, &args![setting]).u32();
    e.mem.f32(at)
}

/// The player pointer.
fn player(e: &Engine) -> Ptr<PlayerCharacter> {
    e.global(PLAYER_POINTER)
}

/// Allocates a block with `operator new` and runs `construct` on it unless
/// the allocation failed; 0 when it failed.
fn new_object(e: &mut Engine, size: u32, construct: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    if block != 0 {
        e.call(construct, &args![block]).u32()
    } else {
        0
    }
}

/// Allocates and constructs an `ItemChange` (12 bytes) for `form` and
/// `number`; 0 when the allocation fails.
fn new_item_change(e: &mut Engine, form: u32, number: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    if block != 0 {
        e.call(ITEM_CHANGE_CONSTRUCTOR, &args![block, form, number])
            .u32()
    } else {
        0
    }
}

/// Allocates an `ExtraDataList` (0x20 bytes) and copies `source` into it for
/// a container, giving it `number` as its count.
fn new_extra_copy(e: &mut Engine, source: u32, number: u32) -> u32 {
    let list = new_object(e, 0x20, EXTRA_DATA_LIST_CONSTRUCT);
    e.call(EXTRA_DATA_LIST_DUPLICATE, &args![list, source]);
    e.call(EXTRA_SET_COUNT, &args![list, number]);
    list
}

/// Appends `extra` to the extra data list of `item` (the head node of the
/// `ItemChange` at +0), through the address of a temporary holding it.
fn add_extra_to_item(e: &mut Engine, item: u32, extra: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), extra);
        let list = e.call(WORD_AT_0, &args![item]).u32();
        e.call(LIST_ADD, &args![list, slot]);
    });
}

/// The weight of one unit of `form` the way the game asks (`GetFormWeight`
/// with the flag the player gives).
fn form_weight(e: &mut Engine, this: Ptr<PlayerCharacter>, form: u32) -> f64 {
    let flag = e.call(WEIGHT_FLAG_OF_PLAYER, &args![this]).u8();
    e.call(GET_FORM_WEIGHT, &args![form, flag as u32]).f64()
}

/// `MiddleHighProcess::GetForceNextUpdate` of an actor (`00566950`).
fn force_next_update(e: &mut Engine, actor: u32) -> bool {
    e.call(GET_FORCE_NEXT_UPDATE, &args![actor]).bool()
}

// ---------------------------------------------------------------------------
// Translations

// Translated from 00967aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the player's byte at +0x608 (calls `00967ac0` on the player pointer
/// with 0).
pub fn fn_00967aa0(e: &mut Engine) {
    let this = player(e);
    fn_00967ac0(e, this, 0);
}

// Translated from 00967ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a byte at +0x608 of the player.
pub fn fn_00967ac0(e: &mut Engine, this: Ptr<PlayerCharacter>, value: u8) {
    e.set(this, PlayerCharacter::bSpeaking, value);
}

// Translated from 00967ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::IsPipboyActive` (Xbox PDB): true while the first person
/// animation plays animation group `0xe2` or `0xf0` (through its sequence at
/// index 2), else whether the Pipboy menu is open.
pub fn player_character_is_pipboy_active(e: &mut Engine, this: Ptr<PlayerCharacter>) -> bool {
    let animation = e.get(this, PlayerCharacter::p1stPersonAnimation);
    if !animation.is_null() {
        let sequence = e.call(ANIMATION_SLOT, &args![animation, 2u32]).u32();
        if sequence != 0 {
            for group in [0xe2u32, 0xf0] {
                let sequence = e.call(ANIMATION_SLOT, &args![animation, 2u32]).u32();
                let inner = e.call(ANIMATION_WORD_AT_0X74, &args![sequence]).u32();
                if e.call(ANIMATION_GROUP_ID, &args![inner]).u32() == group {
                    return true;
                }
            }
        }
    }
    e.call(IS_IN_PIPBOY_MENU, &args![]).bool()
}

// Translated from 00967b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the inventory entry (`ItemChange`) passes the player's usability
/// test (the name is not in the map). A form that answers true to its slot
/// 0x94 (`GetQuestObject` in the Xbox PDB) is ruled out unless its type is
/// 0x18 or 0x28 and `0047bcf0` holds; an extra data list that cannot be worn
/// rules the entry out; then by the entry's form type: 0x18 and 0x28 need
/// health above zero, 0x19, 0x1d, 0x1e and 0x2f always pass, 0x1a needs
/// `0080f7d0`, 0x29 must be the current weapon's first form or in its second
/// form list, every other type fails. The health is the health extra data's
/// float or the form's own health.
pub fn fn_00967b70(e: &mut Engine, this: Ptr<PlayerCharacter>, entry: Ptr<ItemChange>) -> bool {
    let mut usable = true;
    let form = e.call(WORD_AT_8, &args![entry]).u32();
    let list = e.call(WORD_AT_0, &args![entry]).u32();
    let extra = if list != 0 { node_item(e, list) } else { 0 };
    if e.vcall(form, 0x94, &args![]).bool() {
        let kind = e.call(FORM_TYPE, &args![form]).u32();
        if (kind != 0x28 && e.call(FORM_TYPE, &args![form]).u32() != 0x18)
            || !e.call(FORM_TEST_47BCF0, &args![form]).bool()
        {
            usable = false;
        }
    }
    if extra != 0 && e.call(EXTRA_GET_CAN_NOT_WEAR, &args![extra]).bool() {
        usable = false;
    }
    let health_extra = if extra != 0 {
        e.call(EXTRA_GET_BY_TYPE, &args![extra, 0x25u32]).u32()
    } else {
        0
    };
    let health = if health_extra != 0 {
        // The health extra data: the float at +0xc.
        e.mem.f32(health_extra + 0xc)
    } else {
        e.call(GET_FORM_HEALTH, &args![form]).u32() as f32
    };
    let entry_form = e.call(WORD_AT_8, &args![entry]).u32();
    let kind = e.call(FORM_TYPE, &args![entry_form]).u32();
    match kind {
        0x18 | 0x28 => {
            if health <= 0.0 {
                usable = false;
            }
        }
        0x19 | 0x1d | 0x1e | 0x2f => {}
        0x1a => {
            if !e.call(FORM_TEST_80F7D0, &args![this, form, extra]).bool() {
                usable = false;
            }
        }
        0x29 => {
            usable = true;
            let form = e.call(WORD_AT_8, &args![entry]).u32();
            let weapon = e.call(GET_CURRENT_WEAPON, &args![this]).u32();
            if weapon == 0 || form == 0 {
                usable = false;
            } else {
                let first = e.call(WEAPON_FIRST_FORM, &args![weapon + 0xa4]).u32();
                let list = e.call(WEAPON_SECOND_FORM, &args![weapon + 0xa4]).u32();
                if first != form
                    && (list == 0
                        || e.call(LIST_FORM_GET_ITEM_INDEX, &args![list, form]).i32() == -1)
                {
                    usable = false;
                }
            }
        }
        _ => usable = false,
    }
    usable
}

// Translated from 00967da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives up the player's fight with the other combat groups (the name is
/// not in the map) and returns how many of their members it dealt with.
/// First `971fd0` asks `ProcessLists` for a list of entries (given the radius
/// setting `0x11ce3a8`); with none, nothing is done. Then, when the player's
/// group has members that `GetForceNextUpdate` does not force, a new group is
/// created with the player group's targets. For every other combat group that
/// targets the player and none of whose members would attack the player: its
/// targets are removed from the player's group (or handed to the new group for
/// members that may attack one of its members), the targets forced to update
/// that are not angry with the player are dropped (slot 0x434), each member is
/// removed from the groups and its entry from the list, and the nearest
/// member gets a combat dialogue (`StartDialogue`: 2, 0xb, 1, 0). Afterwards
/// the player group's targets are emptied, the current packages of the actors
/// of `ProcessLists` (types 0x18, 0x16 and 0xa) are re-evaluated, the in-combat
/// byte at +0xdf0 is cleared, the entries still in the list are handed to
/// `008bcbd0`, and the members of the new group get slot 0x3fc (with the new
/// group) and `009869a0` on the player's group.
pub fn fn_00967da0(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u32 {
    let mut handled = 0u32;
    let radius = setting_float(e, SETTING_COMBAT_RADIUS);
    let entries = e
        .call(PROCESS_LISTS_ENTRIES, &args![PROCESS_LISTS, radius])
        .u32();
    if entries == 0 {
        return 0;
    }
    let group = e.get(this, PlayerCharacter::pCombatGroup).addr();
    let mut created = 0u32;
    let member_total = if group != 0 {
        e.call(GROUP_MEMBER_COUNT, &args![group]).u32()
    } else {
        0
    };
    // Members the game does not force to update (`GetForceNextUpdate` false);
    // the ones it forces are counted by the game too but the count is unused.
    let mut free_members = 0u32;
    for member_index in 0..member_total {
        let member = e.call(GROUP_MEMBER_AT, &args![group, member_index]).u32();
        if member == e.global::<u32>(PLAYER_POINTER) {
            continue;
        }
        if !force_next_update(e, member) {
            free_members += 1;
        }
    }
    let manager = e.global::<u32>(COMBAT_MANAGER);
    if free_members != 0 {
        created = e.call(CREATE_COMBAT_GROUP, &args![manager]).u32();
        e.call(GROUP_COPY_TARGETS, &args![created, group]);
    }
    let group_total = fn_00968650(e, Ptr::new(manager));
    for group_index in 0..group_total {
        let other = fn_00968670(e, Ptr::new(manager), group_index);
        if other == 0 || other == group || other == created {
            continue;
        }
        if !e.call(GROUP_IS_TARGET, &args![other, this]).bool() {
            continue;
        }
        let mut hostile_free = true;
        let mut all_clear = true;
        let other_total = e.call(GROUP_MEMBER_COUNT, &args![other]).u32();
        // The exe's 0xe8-byte frame: the verdict word at EBP-0x60 (+0x88).
        e.with_stack(0xe8, |e, frame| {
            let out = frame.addr() + 0x88;
            for other_index in 0..other_total {
                let candidate = e.call(GROUP_MEMBER_AT, &args![other, other_index]).u32();
                e.mem.set_u32(out, 0);
                if e.call(
                    GET_SHOULD_ATTACK_ACTOR,
                    &args![candidate, this, 0u32, out, 1u32],
                )
                .bool()
                {
                    hostile_free = false;
                    break;
                }
                if all_clear && free_members != 0 {
                    for member_index in 0..member_total {
                        let member = e.call(GROUP_MEMBER_AT, &args![group, member_index]).u32();
                        if member == e.global::<u32>(PLAYER_POINTER) || force_next_update(e, member)
                        {
                            continue;
                        }
                        if e.call(
                            GET_SHOULD_ATTACK_ACTOR,
                            &args![candidate, member, 0u32, out, 1u32],
                        )
                        .bool()
                            || e.call(
                                GET_SHOULD_ATTACK_ACTOR,
                                &args![member, candidate, 0u32, out, 1u32],
                            )
                            .bool()
                        {
                            all_clear = false;
                            break;
                        }
                    }
                }
            }
        });
        if !hostile_free {
            continue;
        }
        let mut nearest = 0u32;
        let mut nearest_distance: f32 = e.global(NEAREST_ACTOR_START);
        // The same frame: the position copy at EBP-0x54 (+0x94), the difference
        // at EBP-0x8c (+0x5c).
        e.with_stack(0xe8, |e, frame| {
            let work = frame.addr() + 0x94;
            let difference = frame.addr() + 0x5c;
            let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
            for word in 0..3 {
                let value = e.mem.u32(position + 4 * word);
                e.mem.set_u32(work + 4 * word, value);
            }
            if group != 0 {
                for member_index in 0..member_total {
                    let member = e.call(GROUP_MEMBER_AT, &args![group, member_index]).u32();
                    if all_clear
                        || force_next_update(e, member)
                        || member == e.global::<u32>(PLAYER_POINTER)
                    {
                        e.call(GROUP_REMOVE_TARGET, &args![other, member]);
                    } else if created != 0 {
                        e.call(GROUP_ADD_MEMBER, &args![created, member]);
                    }
                }
            } else {
                e.call(GROUP_REMOVE_TARGET, &args![other, this]);
            }
            let mut target_index = 0u32;
            while target_index < e.call(GROUP_TARGET_COUNT, &args![other]).u32() {
                let target = fn_00968630(e, Ptr::new(other), target_index);
                if target != 0
                    && force_next_update(e, target)
                    && !e.call(IS_ANGRY_WITH_PLAYER, &args![target]).bool()
                {
                    e.call(GROUP_REMOVE_TARGET, &args![other, target]);
                    target_index = target_index.wrapping_sub(1);
                    e.vcall(target, 0x434, &args![0u32]);
                }
                target_index = target_index.wrapping_add(1);
            }
            for other_index in 0..other_total {
                let candidate = e.call(GROUP_MEMBER_AT, &args![other, other_index]).u32();
                handled += 1;
                if group != 0 {
                    e.call(GROUP_REMOVE_TARGET, &args![group, candidate]);
                }
                if all_clear && created != 0 {
                    e.call(GROUP_REMOVE_TARGET, &args![created, candidate]);
                }
                let candidate_position = e.vcall(candidate, 0x1f4, &args![]).u32();
                e.call(POINT_SUBTRACT, &args![work, difference, candidate_position]);
                let distance = e.call(POINT_LENGTH, &args![difference]).f32();
                if distance < nearest_distance {
                    nearest = candidate;
                    nearest_distance = distance;
                }
                // Take the candidate's entry out of the list `971fd0` gave.
                let mut current = entries;
                let mut previous = 0u32;
                while current != 0 && !e.call(LIST_IS_EMPTY, &args![current]).bool() {
                    let item = node_item(e, current);
                    let mut remove = false;
                    if item == 0 {
                        remove = true;
                    } else if e.call(WORD_AT_8, &args![item]).u32() == candidate {
                        e.call(ENTRY_DESTROY, &args![item, 1u32]);
                        remove = true;
                    }
                    if remove {
                        if previous != 0 {
                            let slot = e.call(NODE_ITEM_SLOT, &args![current]).u32();
                            e.call(LIST_REMOVE, &args![previous, slot]);
                            current = node_next(e, previous);
                        } else {
                            e.call(LIST_POP_FRONT, &args![current]);
                        }
                    } else {
                        previous = current;
                        current = node_next(e, current);
                    }
                }
            }
        });
        if nearest != 0 {
            let dialogue: u32 = e.global(COMBAT_DIALOGUE_MANAGER);
            e.call(
                START_DIALOGUE,
                &args![dialogue, nearest, this, 2u32, 0xbu32, 1u32, 0u32],
            );
        }
    }
    if group != 0 && handled != 0 {
        let mut member_index = 0u32;
        while member_index < e.call(GROUP_MEMBER_COUNT, &args![group]).u32() {
            let member = e.call(GROUP_MEMBER_AT, &args![group, member_index]).u32();
            if member != 0 && e.vcall(member, 0x428, &args![]).u32() != 0 {
                let target = e.vcall(member, 0x428, &args![]).u32();
                e.call(SET_BYTE_AT_0X125, &args![target]);
            }
            member_index += 1;
        }
        while e.call(GROUP_TARGET_COUNT, &args![group]).u32() != 0 {
            let target = fn_00968630(e, Ptr::new(group), 0);
            e.call(GROUP_REMOVE_TARGET, &args![group, target]);
        }
    }
    let mut node = e
        .call(PROCESS_LISTS_ACTOR_NODE, &args![PROCESS_LISTS])
        .u32();
    while node != 0 {
        let actor = node_item(e, node);
        if actor != 0 {
            let package = e.call(GET_CURRENT_PACKAGE, &args![actor]).u32();
            if package != 0 {
                let the_player = e.global::<u32>(PLAYER_POINTER);
                match e.call(PACKAGE_TYPE, &args![package]).i32() {
                    0x18 => {
                        e.call(PACKAGE_0X18_UPDATE, &args![package, the_player]);
                        e.call(EVALUATE_PACKAGE, &args![actor, 0u32, 0u32]);
                    }
                    0x16 => {
                        e.call(PACKAGE_0X16_UPDATE, &args![package, the_player]);
                        e.call(EVALUATE_PACKAGE, &args![actor, 0u32, 0u32]);
                    }
                    0x0a => {
                        e.call(EVALUATE_PACKAGE, &args![actor, 0u32, 0u32]);
                    }
                    _ => {}
                }
            }
        }
        node = node_next(e, node);
    }
    e.set(this, PlayerCharacter::bPlayerInCombat, 0);
    let mut node = entries;
    while node != 0 {
        let item = node_item(e, node);
        if item != 0 {
            let actor = e.call(WORD_AT_8, &args![item]).u32();
            if actor != 0 {
                e.call(HANDLE_ENTRY, &args![actor, item]);
            }
        }
        node = node_next(e, node);
    }
    if created != 0 {
        let created_total = e.call(GROUP_MEMBER_COUNT, &args![created]).u32();
        for created_index in 0..created_total {
            let member = e
                .call(GROUP_MEMBER_AT, &args![created, created_index])
                .u32();
            e.vcall(member, 0x3fc, &args![created]);
            e.call(GROUP_METHOD_9869A0, &args![group, member]);
        }
    }
    handled
}

// Translated from 00968630 (decompiled, FalloutNV.exe 1.4.0.525)
/// The actor of target `index` of a combat group: the first word of the
/// 0x68-byte `CombatTarget` at `index` of the group's `TargetArray` (+8).
pub fn fn_00968630(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let target = fn_0096a2b0(e, this.byte_add(8), index);
    e.mem.u32(target.addr())
}

// Translated from 00968650 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 16-bit word at +0xa of the object (`00658930`), the combat manager's
/// number of groups.
pub fn fn_00968650(e: &mut Engine, this: Ptr) -> u32 {
    e.call(WORD_AT_0XA, &args![this]).u32()
}

// Translated from 00968670 (decompiled, FalloutNV.exe 1.4.0.525)
/// The combat group at `index` of the manager's array (`00877a30`: element
/// address of the pointer array at +4).
pub fn fn_00968670(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    array_item(e, this.addr(), index)
}

// Translated from 00968690 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reference's position moved ahead by its character controller's linear
/// velocity times `time` (only when `time` is above zero and the reference
/// has a character controller), written to `result`, which is returned. The
/// call of `006815c0` on an unused temporary is left out.
pub fn fn_00968690(e: &mut Engine, this: Ptr, result: Ptr, time: f32) -> Ptr {
    let position = e.call(POSITION_OF, &args![this]).u32();
    // The exe's frame (`SUB ESP,0x28` under `EBP`): the velocity at +0x10 (`EBP-0x18`), the
    // scaled velocity at +4 (`EBP-0x24`), the sum at +0x1c (`EBP-0xc`).
    e.with_stack(0x28, |e, frame| {
        let velocity = frame.addr() + 0x10;
        let scaled = frame.addr() + 4;
        let sum = frame.addr() + 0x1c;
        for word in 0..3 {
            let value = e.mem.u32(position + 4 * word);
            e.mem.set_u32(sum + 4 * word, value);
        }
        if time as f64 > e.global::<f64>(DOUBLE_ZERO) {
            let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
            if controller != 0 {
                let controller = e.call(GET_CHAR_CONTROLLER, &args![this]).u32();
                e.call(CONTROLLER_LINEAR_VELOCITY, &args![controller, velocity]);
                let scaled_vector = e.call(VECTOR_SCALE, &args![velocity, scaled, time]).u32();
                e.call(VECTOR_ADD, &args![sum, scaled_vector]);
            }
        }
        for word in 0..3 {
            let value = e.mem.u32(sum + 4 * word);
            e.mem.set_u32(result.addr() + 4 * word, value);
        }
    });
    result
}

// Translated from 00968730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shows the radiation level on the HUD: `HUDMainMenu::SetRadiationLevel` gets the process's
/// `GetRadiationMagicDelta` (slot 0x75c) as the first float and its `GetRadiationDelta` (slot
/// 0x76c) plus `GetRadiationWaterDelta` (slot 0x764) as the second: the sum is pushed first
/// (00968786), the magic delta last (0096879a).
pub fn fn_00968730(e: &mut Engine, this: Ptr) {
    let first = e.call(ACTOR_PROCESS, &args![this]).u32();
    let second = e.call(ACTOR_PROCESS, &args![this]).u32();
    let third = e.call(ACTOR_PROCESS, &args![this]).u32();
    let water = e.vcall(second, 0x764, &args![]).f64();
    let delta = e.vcall(first, 0x76c, &args![]).f64();
    let total = (delta + water) as f32;
    let magic = e.vcall(third, 0x75c, &args![]).f32();
    e.call(SET_RADIATION_LEVEL, &args![magic, total]);
}

// Translated from 009687b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::AddRockItAmmo` (Xbox PDB): adds `number` of `form` (with
/// the extra data list `extra`, or none) to the RockIt launcher ammunition
/// list at +0xdf4. An entry of the same form takes the number when its extra
/// data list is the default (no `extra` given) or compares equal to `extra`
/// (the extra data list's own count grows too); otherwise a new `ItemChange`
/// is appended, with a copy of `extra` for a container when there is one. The
/// list weight grows by the form's weight times `number`.
pub fn player_character_add_rock_it_ammo(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    form: u32,
    extra: u32,
    number: i32,
) {
    let array = this.at(PlayerCharacter::RockItLauncherAmmoList).addr();
    let mut added = false;
    let mut index = 0u32;
    while !added && index < array_count(e, array) {
        let item = array_item(e, array, index);
        if e.call(WORD_AT_8, &args![item]).u32() == form {
            // The entry has no extra data list, or only a default one.
            let list = e.call(WORD_AT_0, &args![item]).u32();
            let default_extras = !(list != 0 && {
                let first = node_item(e, list);
                first != 0
                    && !e
                        .call(EXTRA_IS_DEFAULT_FOR_CONTAINER, &args![first, 0u32])
                        .bool()
            });
            if extra == 0 {
                if default_extras {
                    let total = item_number(e, item).wrapping_add(number as u32);
                    e.call(SET_NUMBER, &args![item, total]);
                    added = true;
                }
            } else {
                let mut node = e.call(WORD_AT_0, &args![item]).u32();
                while !added && node != 0 && node_item(e, node) != 0 {
                    let held = node_item(e, node);
                    let differs = e
                        .call(
                            EXTRA_COMPARE_LIST_FOR_CONTAINER,
                            &args![held, extra, 1u32, 0u32],
                        )
                        .bool()
                        || e.call(
                            EXTRA_COMPARE_LIST_FOR_CONTAINER,
                            &args![extra, held, 1u32, 0u32],
                        )
                        .bool();
                    if !differs {
                        let total = item_number(e, item).wrapping_add(number as u32);
                        e.call(SET_NUMBER, &args![item, total]);
                        let held_number = e.call(EXTRA_GET_COUNT, &args![held]).u16() as i16 as i32;
                        e.call(
                            EXTRA_SET_COUNT,
                            &args![held, held_number.wrapping_add(number)],
                        );
                        added = true;
                    }
                    node = node_next(e, node);
                }
                if !added
                    && e.call(EXTRA_IS_DEFAULT_FOR_CONTAINER, &args![extra, 0u32])
                        .bool()
                    && default_extras
                {
                    let copy = new_extra_copy(e, extra, number as u16 as u32);
                    if e.call(WORD_AT_0, &args![item]).u32() == 0 {
                        let node = new_object(e, 8, LIST_NODE_CONSTRUCT);
                        e.mem.set_u32(item, node);
                    }
                    add_extra_to_item(e, item, copy);
                    let total = item_number(e, item).wrapping_add(number as u32);
                    e.call(SET_NUMBER, &args![item, total]);
                    added = true;
                }
            }
        }
        index += 1;
    }
    if !added {
        let item = new_item_change(e, form, number as u32);
        if extra != 0 {
            let copy = new_extra_copy(e, extra, number as u16 as u32);
            add_extra_to_item(e, item, copy);
        }
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), item);
            e.call(ARRAY_APPEND, &args![array, slot]);
        });
    }
    let weight = form_weight(e, this, form);
    let old = e.get(this, PlayerCharacter::fRockItLauncherAmmoWeight);
    e.set(
        this,
        PlayerCharacter::fRockItLauncherAmmoWeight,
        (weight * number as f64 + old as f64) as f32,
    );
}

// Translated from 00968be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::RemoveRockItAmmo` (Xbox PDB): takes `number` of `form`
/// (the stack with the extra data list `extra`, or the one without) out of
/// the RockIt launcher ammunition list. With `keep` set it returns a new
/// `ItemChange` holding what was taken (with `extra` or a copy of it in its
/// list); without it the removed extra data list is deleted. An entry whose
/// number is used up leaves the array. The list weight shrinks by the form's
/// weight times the number removed (the extra data list's own count when the
/// whole list goes).
pub fn player_character_remove_rock_it_ammo(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    form: u32,
    extra: u32,
    number: i32,
    keep: u8,
) -> u32 {
    let array = this.at(PlayerCharacter::RockItLauncherAmmoList).addr();
    let mut done = false;
    let mut taken = 0u32;
    let mut remaining = number;
    let mut index = 0u32;
    while !done && index < array_count(e, array) {
        let item = array_item(e, array, index);
        if e.call(WORD_AT_8, &args![item]).u32() == form {
            if extra == 0 {
                let list = e.call(WORD_AT_0, &args![item]).u32();
                if list == 0 || node_item(e, list) == 0 {
                    if keep != 0 {
                        taken = new_item_change(e, form, remaining as u32);
                    }
                    let held = item_number(e, item) as i32;
                    if remaining >= held {
                        e.call(ARRAY_REMOVE, &args![array, index, 1u32]);
                    } else {
                        e.call(SET_NUMBER, &args![item, (held - remaining) as u32]);
                    }
                    done = true;
                }
            } else {
                let mut node = e.call(WORD_AT_0, &args![item]).u32();
                while !done && node != 0 && node_item(e, node) != 0 {
                    if node_item(e, node) == extra {
                        let held = e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16 as i32;
                        let whole = held <= number;
                        if whole {
                            remaining = held;
                        }
                        if keep != 0 {
                            let mut moved = extra;
                            taken = new_item_change(e, form, remaining as u32);
                            if !whole {
                                moved = new_extra_copy(e, extra, remaining as u16 as u32);
                            }
                            add_extra_to_item(e, taken, moved);
                        }
                        if !whole {
                            e.call(EXTRA_SET_COUNT, &args![extra, (held - number) as u32]);
                        } else {
                            let list = e.call(WORD_AT_0, &args![item]).u32();
                            if list != 0 {
                                e.with_stack(4, |e, slot| {
                                    e.mem.set_u32(slot.addr(), extra);
                                    e.call(LIST_REMOVE, &args![list, slot]);
                                });
                            }
                            if keep == 0 {
                                // The extra data list's scalar deleting destructor.
                                e.vcall(extra, 0, &args![1u32]);
                            }
                        }
                        let held = item_number(e, item) as i32;
                        if remaining >= held {
                            e.call(ARRAY_REMOVE, &args![array, index, 1u32]);
                        } else {
                            e.call(SET_NUMBER, &args![item, (held - remaining) as u32]);
                        }
                        done = true;
                    }
                    node = node_next(e, node);
                }
            }
        }
        index += 1;
    }
    let weight = form_weight(e, this, form);
    let old = e.get(this, PlayerCharacter::fRockItLauncherAmmoWeight);
    e.set(
        this,
        PlayerCharacter::fRockItLauncherAmmoWeight,
        (old as f64 - weight * remaining as f64) as f32,
    );
    taken
}

// Translated from 00968f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes one unit of a randomly chosen entry out of the RockIt launcher
/// ammunition list (through `RemoveRockItAmmo` with the entry's first extra
/// data list, keeping what was taken) and returns the `ItemChange` it made;
/// 0 when the list is empty.
pub fn fn_00968f90(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u32 {
    let array = this.at(PlayerCharacter::RockItLauncherAmmoList).addr();
    if e.call(ARRAY_IS_EMPTY, &args![array]).bool() {
        return 0;
    }
    let random = e.call(RANDOM, &args![]).u32();
    let count = array_count(e, array);
    let item = array_item(e, array, random % count);
    let list = e.call(WORD_AT_0, &args![item]).u32();
    let extra = if list != 0 { node_item(e, list) } else { 0 };
    let form = e.call(WORD_AT_8, &args![item]).u32();
    player_character_remove_rock_it_ammo(e, this, form, extra, 1, 1)
}

// Translated from 00969040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::GetNumRockItAmmo` (Xbox PDB): the sum of the numbers of
/// the entries of the RockIt launcher ammunition list.
pub fn player_character_get_num_rock_it_ammo(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u32 {
    let array = this.at(PlayerCharacter::RockItLauncherAmmoList).addr();
    let mut total = 0u32;
    let mut index = 0;
    while index < array_count(e, array) {
        let item = array_item(e, array, index);
        total = total.wrapping_add(item_number(e, item));
        index += 1;
    }
    total
}

// Translated from 009690a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::ClearRockItAmmo` (Xbox PDB): `DeleteAllExtra` on every
/// entry, empties the array and resets the weight to zero.
pub fn player_character_clear_rock_it_ammo(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let array = this.at(PlayerCharacter::RockItLauncherAmmoList).addr();
    let mut index = 0;
    while index < array_count(e, array) {
        let item = array_item(e, array, index);
        e.call(ITEM_CHANGE_DELETE_ALL_EXTRA, &args![item]);
        index += 1;
    }
    e.call(ARRAY_CLEAR, &args![array, 1u32]);
    player_character_reset_rock_it_ammo_weight(e, this, 0);
}

// Translated from 00969110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::ResetRockItAmmoWeight` (Xbox PDB): sets the list weight
/// to zero and, when `recompute` is set, adds the weight of every entry (the
/// form's weight times its number).
pub fn player_character_reset_rock_it_ammo_weight(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    recompute: u8,
) {
    e.set(this, PlayerCharacter::fRockItLauncherAmmoWeight, 0.0);
    if recompute != 0 {
        let array = this.at(PlayerCharacter::RockItLauncherAmmoList).addr();
        let mut index = 0;
        while index < array_count(e, array) {
            let item = array_item(e, array, index);
            let form = e.call(WORD_AT_8, &args![item]).u32();
            let weight = form_weight(e, this, form);
            let number = item_number(e, item) as i32;
            let old = e.get(this, PlayerCharacter::fRockItLauncherAmmoWeight);
            e.set(
                this,
                PlayerCharacter::fRockItLauncherAmmoWeight,
                (number as f64 * weight + old as f64) as f32,
            );
            index += 1;
        }
    }
}

// Translated from 009691d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::ReturnRockItAmmo` (Xbox PDB): moves every entry of the
/// RockIt launcher ammunition list into the player's inventory changes, then
/// empties the array and resets the weight to zero.
pub fn player_character_return_rock_it_ammo(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let array = this.at(PlayerCharacter::RockItLauncherAmmoList).addr();
    let inventory = e.call(GET_INVENTORY_CHANGES, &args![player(e)]).u32();
    let mut index = 0;
    while inventory != 0 && index < array_count(e, array) {
        let item = array_item(e, array, index);
        e.call(INVENTORY_ADD_ITEM_CHANGE, &args![inventory, item, 1u32]);
        index += 1;
    }
    e.call(ARRAY_CLEAR, &args![array, 1u32]);
    player_character_reset_rock_it_ammo_weight(e, this, 0);
}

/// The package set-up the two package starters share: a package of type
/// `kind` with two flags set, located at `this` and targeting `reference`
/// (target type 0, word at +8 of the target object 0), and the word `word`
/// stored at +0x18 of the package. The temporary location and target are
/// destroyed after they are copied into the package. Returns the package.
fn build_package(e: &mut Engine, kind: u32, word: u32, this: u32, reference: u32) -> u32 {
    let package = e.call(CREATE_PACKAGE, &args![kind]).u32();
    e.call(PACKAGE_SET_FLAG_2, &args![package, 1u32]);
    e.call(PACKAGE_SET_FLAG_4, &args![package, 1u32]);
    let location = new_object(e, 0xc, PACKAGE_LOCATION_CONSTRUCTOR);
    e.call(PACKAGE_LOCATION_METHOD_67F140, &args![location, 0u32]);
    e.call(SET_LOC_REFERENCE, &args![location, this]);
    e.call(SET_PACKAGE_LOCATION, &args![package, location]);
    if location != 0 {
        e.call(PACKAGE_LOCATION_DESTRUCTOR, &args![location, 1u32]);
    }
    let target = new_object(e, 0x10, PACKAGE_TARGET_CONSTRUCTOR);
    e.call(SET_PACKAGE_TARGET, &args![package, target]);
    if target != 0 {
        e.call(PACKAGE_TARGET_DESTRUCTOR, &args![target, 1u32]);
    }
    e.call(SET_PACKAGE_WORD_AT_0X18, &args![package, word]);
    let object = e.call(PACKAGE_TARGET_OBJECT, &args![package]).u32();
    e.call(SET_TARG_TYPE, &args![object, 0u32]);
    let object = e.call(PACKAGE_TARGET_OBJECT, &args![package]).u32();
    e.call(SET_TARG_REFERENCE, &args![object, reference]);
    let object = e.call(PACKAGE_TARGET_OBJECT, &args![package]).u32();
    e.call(SET_TARGET_WORD_AT_8, &args![object, 0u32]);
    package
}

// Translated from 00969260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts a package of type 0x25 (the one `00969700` tests for) on the
/// player: stores the `ProcessLists` call `00973710(player, 0)` at +0x1fc,
/// puts the player in the modes `005cc7a0(1)` and `ForceTemp3rdPerson(1)`,
/// clears the process's run-once package (slot 0x210) and action head track
/// target (slot 0x644), builds the package for `reference` (`build_package`
/// with word 0x1d), tells the process slot 0x28 and the player slot 0x370
/// (1), hands the process `slot 0x540 (a, b, c)` and the package to the
/// player's slot 0x2f4 (`package, 0, 1`), and ends with `008a8060` on
/// `reference`.
pub fn fn_00969260(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    reference: u32,
    first: u32,
    third: u32,
    second: u8,
) {
    let queued = e
        .call(
            PROCESS_LISTS_QUEUE_TARGET,
            &args![PROCESS_LISTS, player(e), 0u32],
        )
        .u32();
    e.set(this, PlayerCharacter::field_01fc, queued);
    e.call(PLAYER_SET_MODE_5CC7A0, &args![this, 1u32]);
    e.call(FORCE_TEMP_3RD_PERSON, &args![this, 1u32]);
    let process = e.get(this, PlayerCharacter::pProcess).addr();
    e.vcall(process, 0x210, &args![0u32, this]);
    let process = e.get(this, PlayerCharacter::pProcess).addr();
    e.vcall(process, 0x644, &args![0u32]);
    let package = build_package(e, 0x25, 0x1d, this.addr(), reference);
    let process = e.call(ACTOR_PROCESS, &args![this]).u32();
    e.vcall(process, 0x28, &args![]);
    e.vcall(this.addr(), 0x370, &args![1u32]);
    let process = e.get(this, PlayerCharacter::pProcess).addr();
    e.vcall(process, 0x540, &args![first, second as u32, third]);
    e.vcall(this.addr(), 0x2f4, &args![package, 0u32, 1u32]);
    e.call(REFERENCE_AFTER_PACKAGE, &args![reference]);
}

// Translated from 009694d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts a package of type 0x26 (the one `00969760` tests for) on the
/// player for `reference` unless `008a8170` says the reference is ruled out:
/// the same set-up as `00969260` (word 0x2f), without the queued target, and
/// the process slot 0x28 and the player slot 0x2f4 (`package, 0, 1`).
pub fn fn_009694d0(e: &mut Engine, this: Ptr<PlayerCharacter>, reference: u32) {
    if e.call(REFERENCE_EARLY_OUT_TEST, &args![reference]).bool() {
        return;
    }
    e.call(PLAYER_SET_MODE_5CC7A0, &args![this, 1u32]);
    e.call(FORCE_TEMP_3RD_PERSON, &args![this, 1u32]);
    let process = e.get(this, PlayerCharacter::pProcess).addr();
    e.vcall(process, 0x210, &args![0u32, this]);
    let process = e.get(this, PlayerCharacter::pProcess).addr();
    e.vcall(process, 0x644, &args![0u32]);
    let package = build_package(e, 0x26, 0x2f, this.addr(), reference);
    let process = e.call(ACTOR_PROCESS, &args![this]).u32();
    e.vcall(process, 0x28, &args![]);
    e.vcall(this.addr(), 0x2f4, &args![package, 0u32, 1u32]);
}

/// Whether the player's current package has the type `kind` (the shared body
/// of `00969700` and `00969760`): needs `0093a740` to say a flag is set,
/// then asks the process (slot 0x22c) for its current package.
fn current_package_is(e: &mut Engine, this: Ptr<PlayerCharacter>, kind: i32) -> bool {
    if e.call(PLAYER_ANY_FLAG, &args![this]).bool() {
        let process = e.call(ACTOR_PROCESS, &args![this]).u32();
        let package = e.vcall(process, 0x22c, &args![]).u32();
        if package != 0 && e.call(PACKAGE_TYPE, &args![package]).i32() == kind {
            return true;
        }
    }
    false
}

// Translated from 00969700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the player's current package is of type 0x25.
pub fn fn_00969700(e: &mut Engine, this: Ptr<PlayerCharacter>) -> bool {
    current_package_is(e, this, 0x25)
}

// Translated from 00969760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the player's current package is of type 0x26.
pub fn fn_00969760(e: &mut Engine, this: Ptr<PlayerCharacter>) -> bool {
    current_package_is(e, this, 0x26)
}

// Translated from 009697c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the toddler byte at +0x7c6; when it changes, calls `008b73f0(0,
/// first person animation, text of the setting 0x11cdd78, 0)` on the player.
pub fn fn_009697c0(e: &mut Engine, this: Ptr<PlayerCharacter>, value: u8) {
    if e.get(this, PlayerCharacter::bIsToddler) == value {
        return;
    }
    e.set(this, PlayerCharacter::bIsToddler, value);
    let text = e.call(SETTING_STRING, &args![SETTING_TODDLER_TEXT]).u32();
    let text = e.call(PASS_THROUGH, &args![text, 0u32]).u32();
    let animation = e.call(PLAYER_GET_ANIMATION, &args![this, 1u32]).u32();
    e.call(PLAYER_PLAY_NAMED, &args![this, 0u32, animation, text, 0u32]);
}

// Translated from 00969820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the young byte at +0x7c5; when it changes, calls `008b78c0(0)` and
/// `008d3fa0` on the player.
pub fn fn_00969820(e: &mut Engine, this: Ptr<PlayerCharacter>, value: u8) {
    if e.get(this, PlayerCharacter::bIsYoung) == value {
        return;
    }
    e.set(this, PlayerCharacter::bIsYoung, value);
    e.call(ACTOR_REFRESH_8B78C0, &args![this, 0u32]);
    e.call(ACTOR_REFRESH_8D3FA0, &args![this]);
}

// Translated from 00969860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the object at +0x224 exists and has a non-zero word at +0x94.
pub fn fn_00969860(e: &mut Engine, this: Ptr<PlayerCharacter>) -> bool {
    let object = e.get(this, PlayerCharacter::field_0224);
    !object.is_null() && e.mem.u32(object.addr() + 0x94) != 0
}

// Translated from 009698a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The lookup (`004839c0`) of the closest audio marker's word at +0x24: of
/// the cached marker if there is one, else of the one `00969930` finds; 0 when
/// there is none or it has no word at +4.
pub fn fn_009698a0(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u32 {
    let cached = e.get(this, PlayerCharacter::pClosestAudioMarkerInfo);
    if !cached.is_null() && e.mem.u32(cached.addr() + 4) != 0 {
        let info = e.mem.u32(cached.addr() + 4);
        return fn_00969910(e, Ptr::new(info));
    }
    let closest = Ptr::<()>::new(fn_00969930(e, this));
    if !closest.is_null() && e.mem.u32(closest.addr() + 4) != 0 {
        let info = e.mem.u32(closest.addr() + 4);
        return fn_00969910(e, Ptr::new(info));
    }
    0
}

// Translated from 00969910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `004839c0` of the word at +0x24 of the object.
pub fn fn_00969910(e: &mut Engine, this: Ptr) -> u32 {
    let word = e.mem.u32(this.addr() + 0x24);
    e.call(AUDIO_MARKER_LOOKUP, &args![word]).u32()
}

// Translated from 00969930 (decompiled, FalloutNV.exe 1.4.0.525)
/// The closest audio marker (cached at +0x7dc): the cache if set, 0 for an
/// empty list, else the marker of the list `AudioMarkerList` whose reference
/// is nearest (by squared distance to the player), where a marker inside its
/// reference's radius beats every marker outside it.
pub fn fn_00969930(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u32 {
    let cached = e.get(this, PlayerCharacter::pClosestAudioMarkerInfo);
    if !cached.is_null() {
        return cached.addr();
    }
    let head = this.at(PlayerCharacter::AudioMarkerList).addr();
    if e.call(LIST_IS_EMPTY, &args![head]).bool() {
        return 0;
    }
    let mut best = 0u32;
    let mut best_distance = e.global::<f32>(NEAREST_MARKER_START);
    let mut best_inside = false;
    let mut node = head;
    while node != 0 && node_item(e, node) != 0 {
        let marker = node_item(e, node);
        node = node_next(e, node);
        let reference = e.mem.u32(marker);
        if reference == 0 {
            continue;
        }
        // The exe's frame (`SUB ESP,0x40` under `EBP`): the position copy at +0x1c (`EBP-0x24`),
        // the difference at +0x10 (`EBP-0x30`).
        let (distance, radius) = e.with_stack(0x40, |e, frame| {
            let work = frame.addr() + 0x1c;
            let difference = frame.addr() + 0x10;
            let position = e.vcall(reference, 0x1f4, &args![]).u32();
            for word in 0..3 {
                let value = e.mem.u32(position + 4 * word);
                e.mem.set_u32(work + 4 * word, value);
            }
            let mine = e.vcall(this.addr(), 0x1f4, &args![]).u32();
            e.call(POINT_SUBTRACT, &args![work, difference, mine]);
            let distance = e.call(POINT_LENGTH_SQUARED, &args![difference]).f32();
            let radius = e.call(REFERENCE_RADIUS, &args![reference]).f32();
            (distance, radius)
        });
        let inside = (distance as f64) < radius as f64 * radius as f64;
        if best_distance > distance {
            if !best_inside {
                best_distance = distance;
                best = marker;
                best_inside = inside;
            }
        } else if inside && !best_inside {
            best_distance = distance;
            best = marker;
            best_inside = inside;
        }
    }
    e.set(
        this,
        PlayerCharacter::pClosestAudioMarkerInfo,
        Ptr::new(best),
    );
    best
}

// Translated from 00969ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the timer flag byte (global `011e07ab`).
pub fn fn_00969ac0(e: &mut Engine, _this: Ptr) {
    e.set_global(TIMER_ACTIVE_FLAG, 1u8);
}

// Translated from 00969ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the timer flag byte (global `011e07ab`).
pub fn fn_00969ae0(e: &mut Engine, _this: Ptr) {
    e.set_global(TIMER_ACTIVE_FLAG, 0u8);
}

// Translated from 00969b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the timer's start second (global `011e07b4`).
pub fn fn_00969b00(e: &mut Engine, _this: Ptr) {
    e.set_global(TIMER_START_SECONDS, 0u32);
}

// Translated from 00969b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the current time in whole seconds (the time import / 1000) as the
/// timer's start second.
pub fn fn_00969b20(e: &mut Engine, _this: Ptr) {
    let milliseconds = e.call(TIME_IMPORT, &args![]).u32();
    e.set_global(TIMER_START_SECONDS, milliseconds / 1000);
}

// Translated from 00969b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The seconds left on the timer: start second plus the setting `0x11d3570`
/// (an integer) minus the current second.
pub fn fn_00969b40(e: &mut Engine, _this: Ptr) -> i32 {
    let at = e
        .call(SETTING_INT_POINTER, &args![SETTING_TIMER_SECONDS])
        .u32();
    let length = e.mem.u32(at);
    let started = e.global::<u32>(TIMER_START_SECONDS);
    let milliseconds = e.call(TIME_IMPORT, &args![]).u32();
    started
        .wrapping_add(length)
        .wrapping_sub(milliseconds / 1000) as i32
}

// Translated from 00969b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the timer is running with time left: true while the flag is set
/// and `00969b40` is above zero, else it clears the start second and the
/// flag and answers false.
pub fn fn_00969b80(e: &mut Engine, this: Ptr) -> bool {
    if e.global::<u8>(TIMER_ACTIVE_FLAG) != 0 && fn_00969b40(e, this) > 0 {
        return true;
    }
    fn_00969b00(e, this);
    fn_00969ae0(e, this);
    false
}

// Translated from 00969bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the caravan card `card` to the player's inactive caravan card list
/// unless either list holds it already (a new card first gets the flag
/// `006e2b50(0)`). Always true.
pub fn fn_00969bc0(e: &mut Engine, this: Ptr<PlayerCharacter>, card: u32) -> bool {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), card);
        let inactive = e
            .get(this, PlayerCharacter::pInactiveListofCaravanCards)
            .addr();
        let active = e
            .get(this, PlayerCharacter::pActiveListofCaravanCards)
            .addr();
        if !e.call(LIST_CONTAINS, &args![inactive, slot]).bool()
            && !e.call(LIST_CONTAINS, &args![active, slot]).bool()
        {
            e.call(SET_CARD_FLAG, &args![card, 0u32]);
            let inactive = e
                .get(this, PlayerCharacter::pInactiveListofCaravanCards)
                .addr();
            e.call(LIST_ADD, &args![inactive, slot]);
        }
    });
    true
}

// Translated from 00969c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::UpdateHardcoreMode` (Xbox PDB): unless the player is in
/// the state `005a03f0(4)`, compares the calendar value now with the one at
/// the last update for each of three actor values (indices 0x49, 0x4b and
/// 0x4a, each with its own limit setting scaled by the calendar time scale)
/// and, for each whole limit passed, modifies that actor value by the number
/// of limits passed (slot 0x3b0). The 0x4b one is skipped while `005a1e50`
/// holds. The saved values are reset first when the first is zero or
/// `00969e70` (`bResetHardcoreTimers`) says so.
pub fn player_character_update_hardcore_mode(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    let the_player = player(e);
    if e.call(PLAYER_IS_IN_STATE, &args![the_player, 4u32]).bool() {
        return;
    }
    let time_scale = e.call(CALENDAR_GET_TIME_SCALE, &args![CALENDAR]).f32();
    let divisor: f64 = e.global(HARDCORE_DIVISOR);
    let limit = |e: &mut Engine, setting: u32| -> f32 {
        let value = setting_float(e, setting);
        ((value as f64 * time_scale as f64) / divisor) as f32
    };
    let limit_49 = limit(e, SETTING_LIMIT_49);
    let limit_4b = limit(e, SETTING_LIMIT_4B);
    let limit_4a = limit(e, SETTING_LIMIT_4A);
    let now = e.call(CALENDAR_GET_VALUE, &args![CALENDAR]).f32();
    e.set_global(NOW_VALUE_4B, now);
    let now: f32 = e.global(NOW_VALUE_4B);
    e.set_global(NOW_VALUE_4A, now);
    let now: f32 = e.global(NOW_VALUE_4A);
    e.set_global(NOW_VALUE_49, now);
    if e.global::<f32>(LAST_VALUE_49) as f64 == e.global::<f64>(DOUBLE_ZERO)
        || fn_00969e70(e, this) != 0
    {
        let value: f32 = e.global(NOW_VALUE_49);
        e.set_global(LAST_VALUE_49, value);
        let value: f32 = e.global(NOW_VALUE_4A);
        e.set_global(LAST_VALUE_4A, value);
        let value: f32 = e.global(NOW_VALUE_4B);
        e.set_global(LAST_VALUE_4B, value);
        e.call(PLAYER_RESET_NEEDS, &args![this, 0u32]);
    }
    let passed_49 =
        (e.global::<f32>(NOW_VALUE_49) as f64 - e.global::<f32>(LAST_VALUE_49) as f64) as f32;
    let passed_4a =
        (e.global::<f32>(NOW_VALUE_4A) as f64 - e.global::<f32>(LAST_VALUE_4A) as f64) as f32;
    let passed_4b =
        (e.global::<f32>(NOW_VALUE_4B) as f64 - e.global::<f32>(LAST_VALUE_4B) as f64) as f32;
    if limit_49 <= passed_49 {
        let times = e
            .call(FLOAT_TO_INT, &args![passed_49 as f64 / limit_49 as f64])
            .i32();
        e.vcall(this.addr(), 0x3b0, &args![0x49u32, times, 0u32]);
        let value = e.call(CALENDAR_GET_VALUE, &args![CALENDAR]).f32();
        e.set_global(NOW_VALUE_49, value);
        let value: f32 = e.global(NOW_VALUE_49);
        e.set_global(LAST_VALUE_49, value);
    }
    if limit_4b <= passed_4b {
        let times = e
            .call(FLOAT_TO_INT, &args![passed_4b as f64 / limit_4b as f64])
            .i32();
        if !e.call(PLAYER_NEEDS_TEST, &args![the_player]).bool() {
            e.vcall(this.addr(), 0x3b0, &args![0x4bu32, times, 0u32]);
        }
        let value = e.call(CALENDAR_GET_VALUE, &args![CALENDAR]).f32();
        e.set_global(NOW_VALUE_4B, value);
        let value: f32 = e.global(NOW_VALUE_4B);
        e.set_global(LAST_VALUE_4B, value);
    }
    if limit_4a <= passed_4a {
        let times = e
            .call(FLOAT_TO_INT, &args![passed_4a as f64 / limit_4a as f64])
            .i32();
        e.vcall(this.addr(), 0x3b0, &args![0x4au32, times, 0u32]);
        let value = e.call(CALENDAR_GET_VALUE, &args![CALENDAR]).f32();
        e.set_global(NOW_VALUE_4A, value);
        let value: f32 = e.global(NOW_VALUE_4A);
        e.set_global(LAST_VALUE_4A, value);
    }
}

// Translated from 00969e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte `bResetHardcoreTimers` at +0xe39.
pub fn fn_00969e70(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u8 {
    e.get(this, PlayerCharacter::bResetHardcoreTimers)
}

// Translated from 00969e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PlayerCharacter::SetHardcore` (Xbox PDB): resets the reference's
/// extra-data type 0x15 value (`00963b00`) and, when switching to no hardcore
/// mode (`mode` and `keep` both zero) while the global flag `007d6e60`
/// is clear, gives back the three needs by modifying actors values 0x49,
/// 0x4b and 0x4a (slot 0x3a4) by minus their current value (slot 0xc of the
/// `ActorValueOwner` at +0xa4); then stores `mode` at +0x7bc.
pub fn player_character_set_hardcore(
    e: &mut Engine,
    this: Ptr<PlayerCharacter>,
    mode: u32,
    keep: u8,
) {
    let list = e.call(EXTRA_LIST_OF_REFERENCE, &args![this]).u32();
    let extra = e.call(EXTRA_GET_TYPE_0X15, &args![list]).u32();
    if extra != 0 {
        e.call(RESET_TYPE_0X15, &args![extra]);
    }
    if mode == 0 && keep == 0 {
        e.call(CLEAR_BYTE_AT_0XE38, &args![this]);
        if !e.call(GLOBAL_FLAG_7D6E60, &args![]).bool() {
            for value in [0x49u32, 0x4b, 0x4a] {
                // 00969f00 and the next two: the actor-value owner's +0xc takes one word
                // (RET 4); the 0 pushed first is the last word of the player's +0x3a4
                // (RET 0xc).
                let current = e.vcall(this.addr() + 0xa4, 0xc, &args![value]).f32();
                e.vcall(this.addr(), 0x3a4, &args![value, -current, 0u32]);
            }
        }
    }
    e.set(this, PlayerCharacter::eHardcoreSetting, mode);
}

/// Shows the message of the setting object `setting`: its text with no
/// sound, the glow icon and the shared display time.
fn show_message(e: &mut Engine, setting: u32) {
    let text = e.call(SETTING_STRING, &args![setting]).u32();
    let time: f32 = e.global(MESSAGE_TIME);
    e.call(
        SHOW_MESSAGE,
        &args![text, 0u32, MESSAGE_ICON, 0u32, time, 0u32],
    );
}

// Translated from 00969fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The player's sleep/wait request: shows the message of the first reason
/// that rules it out (slot 0x448 of the player, `00953c80`, `00885520`
/// against the player's position, `009764a0` on `ProcessLists`, the
/// character controller state 1 or 2, `005444c0` or `00944360`, `005098e0`
/// on `ProcessLists`, a non-zero radiation water delta of the process,
/// `00822e00` on the player's `+0x94`), or opens the sleep menu
/// (`Interface::CreateSleepMenu(1)`) when none holds.
pub fn fn_00969fa0(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    if e.vcall(this.addr(), 0x448, &args![]).bool() {
        show_message(e, MESSAGE_FIRST);
        return;
    }
    if e.call(SLEEP_TEST_953C80, &args![this]).bool() {
        show_message(e, MESSAGE_SECOND);
        return;
    }
    let distance: f32 = e.global(SLEEP_TEST_DISTANCE);
    let place = e.call(ACTOR_PROCESS_OF_THIS, &args![this]).u32();
    let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
    if e.call(SLEEP_TEST_885520, &args![this, position, place, distance])
        .bool()
    {
        show_message(e, MESSAGE_THIRD);
        return;
    }
    let the_player = player(e);
    let interior = e.call(REFERENCE_IS_INTERIOR, &args![the_player]).u8();
    if e.call(
        PROCESS_LISTS_TEST_9764A0,
        &args![PROCESS_LISTS, interior as u32],
    )
    .bool()
    {
        show_message(e, MESSAGE_FOURTH);
        return;
    }
    let controller = e.call(GET_CHAR_CONTROLLER, &args![the_player]).u32();
    if e.call(CONTROLLER_STATE, &args![controller]).u32() == 1 {
        show_message(e, MESSAGE_FIFTH);
        return;
    }
    let controller = e.call(GET_CHAR_CONTROLLER, &args![the_player]).u32();
    if e.call(CONTROLLER_STATE, &args![controller]).u32() == 2 {
        show_message(e, MESSAGE_FIFTH);
        return;
    }
    let cell = e.call(ACTOR_PROCESS_OF_THIS, &args![the_player]).u32();
    if e.call(CELL_TEST_5444C0, &args![cell]).bool()
        || e.call(SLEEP_TEST_944360, &args![the_player]).bool()
    {
        show_message(e, MESSAGE_SIXTH);
        return;
    }
    if e.call(FURNITURE_TEST_5098E0, &args![PROCESS_LISTS]).bool() {
        show_message(e, MESSAGE_SEVENTH);
        return;
    }
    if e.call(ACTOR_PROCESS, &args![this]).u32() != 0 {
        let process = e.call(ACTOR_PROCESS, &args![this]).u32();
        if e.vcall(process, 0x764, &args![]).f64() != 0.0 {
            show_message(e, MESSAGE_SEVENTH);
            return;
        }
    }
    if e.call(MAGIC_TEST_822E00, &args![the_player.addr() + 0x94])
        .bool()
    {
        show_message(e, MESSAGE_EIGHTH);
        return;
    }
    e.call(CREATE_SLEEP_MENU, &args![1u32]);
}

// Translated from 0096a280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor: runs the base initialiser `0096a390(size)` and installs the
/// vtable `0108b474`. Returns `this`.
pub fn fn_0096a280(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    fn_0096a390(e, this, size);
    e.mem.set_u32(this.addr(), VTABLE_0096A280);
    this
}

// Translated from 0096a2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of element `index` of a `BSSimpleArray` of 0x68-byte
/// elements (`0096a330`).
pub fn fn_0096a2b0(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    Ptr::new(fn_0096a330(e, this, index))
}

// Translated from 0096a2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a two-word object (the `BSSimpleList` node: item and next
/// node): clears both words. Returns `this`. (The map names it
/// `Concurrency::details::QuickBitSet::QuickBitSet`, a folded library name.)
pub fn fn_0096a2d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

// Translated from 0096a300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned_int_unsigned_char>::_scalar_deleting_destructor_` (Xbox
/// PDB): runs the destructor `0096a4b0`; when bit 0 of `flags` is set, frees
/// the object (`00401030`). Returns `this`.
pub fn fn_0096a300(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0096a4b0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_OBJECT, &args![this]);
    }
    this
}

// Translated from 0096a330 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of element `index` of an array of 0x68-byte elements whose
/// buffer is the word at +4.
pub fn fn_0096a330(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    index
        .wrapping_mul(0x68)
        .wrapping_add(e.mem.u32(this.addr() + 4))
}

// Translated from 0096a350 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 16-bit word at +4 of the object.
pub fn fn_0096a350(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u16(this.addr() + 4) as u32
}

// Translated from 0096a370 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of element `index` of an array of 0x30-byte elements whose
/// buffer is the word at +0.
pub fn fn_0096a370(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    index
        .wrapping_mul(0x30)
        .wrapping_add(e.mem.u32(this.addr()))
}

// Translated from 0096a390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialiser of the hash table base: vtable `0108b494`, bucket count
/// `size` at +4, item count 0 at +0xc, and a zero-filled bucket array of
/// `size` words (`00aa1070`, `00403d30`) at +8. Returns `this`.
pub fn fn_0096a390(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    e.mem.set_u32(this.addr(), HASH_TABLE_VTABLE);
    e.mem.set_u32(this.addr() + 4, size);
    e.mem.set_u32(this.addr() + 0xc, 0);
    let bytes = size << 2;
    let buckets = e.call(ALLOCATE_BYTES, &args![bytes]).u32();
    e.mem.set_u32(this.addr() + 8, buckets);
    e.call(FILL_MEMORY, &args![buckets, 0u32, bytes]);
    this
}

// Translated from 0096a400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Advances a hash table iterator. `position` points at the current item
/// pointer (an item is: next in chain at +0, key at +4, value byte at +8).
/// Stores the current item's key in `key` and value byte in `value`, then
/// moves `position` to the next item in the chain, or else to the first item
/// of the next non-empty bucket (the bucket of the key comes from slot 4 of
/// the table's vtable), or to 0 at the end.
pub fn fn_0096a400(e: &mut Engine, this: Ptr, position: Ptr, key: Ptr, value: Ptr) {
    let item = e.mem.u32(position.addr());
    let item_key = e.mem.u32(item + 4);
    e.mem.set_u32(key.addr(), item_key);
    let item_value = e.mem.u8(item + 8);
    e.mem.set_u8(value.addr(), item_value);
    let next = e.mem.u32(item);
    if next != 0 {
        e.mem.set_u32(position.addr(), next);
        return;
    }
    let mut bucket = e.vcall(this.addr(), 4, &args![item_key]).u32();
    let count = e.mem.u32(this.addr() + 4);
    loop {
        bucket = bucket.wrapping_add(1);
        if bucket >= count {
            e.mem.set_u32(position.addr(), 0);
            return;
        }
        let buckets = e.mem.u32(this.addr() + 8);
        let first = e.mem.u32(buckets + bucket * 4);
        if first != 0 {
            e.mem.set_u32(position.addr(), first);
            return;
        }
    }
}

// Translated from 0096a4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the map class whose vtable is `0108b474`: installs that
/// vtable, empties the table (`00438af0`), then runs the base destructor
/// `0096a510`. The exception unwinding frame is not translated.
pub fn fn_0096a4b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MAP_VTABLE);
    e.call(HASH_TABLE_EMPTY, &args![this]);
    fn_0096a510(e, this);
}

// Translated from 0096a510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the hash table base: installs vtable `0108b494`, empties the
/// table (`00438af0`) and frees the bucket array at +8 (`00aa10f0`).
pub fn fn_0096a510(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), HASH_TABLE_VTABLE);
    e.call(HASH_TABLE_EMPTY, &args![this]);
    let buckets = e.mem.u32(this.addr() + 8);
    e.call(FREE_BYTES, &args![buckets]);
}

// Translated from 0096a540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the item's byte at +8 (`0096a547`: `MOV EAX,[EBP+8]`; `MOV byte ptr [EAX+8],0`) and
/// passes `item` to `006b8310` on the sub-object at +0xc (it frees `item` through `00401030`).
pub fn fn_0096a540(e: &mut Engine, this: Ptr, item: u32) {
    e.mem.set_u8(item.wrapping_add(8), 0);
    e.call(SUB_OBJECT_RELEASE, &args![this.addr() + 0xc, item]);
}

// Translated from 0096a570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `BSSimpleArray<ItemChange *>` (vtable `0108b4b4`): the
/// array initialiser `006b3eb0(0, 0)`. Returns `this`.
pub fn fn_0096a570(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ITEM_CHANGE_ARRAY_VTABLE);
    e.call(ARRAY_INITIALISE_SIZED, &args![this, 0u32, 0u32]);
    this
}

// Translated from 0096a5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray<ItemChange *>`: installs vtable
/// `0108b4b4` and empties the array (`008454f0(1)`).
pub fn fn_0096a5a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ITEM_CHANGE_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 0096a5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `BSSimpleArray<TESAmmo *>` (vtable `0108b4c8`): the
/// array initialiser `006b3eb0(0, 0)`. Returns `this`.
pub fn fn_0096a5c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), AMMO_ARRAY_VTABLE);
    e.call(ARRAY_INITIALISE_SIZED, &args![this, 0u32, 0u32]);
    this
}

// Translated from 0096a5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray<TESAmmo *>`: installs vtable `0108b4c8`
/// and empties the array (`008454f0(1)`).
pub fn fn_0096a5f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), AMMO_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 0096a610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SetAtGrow` (`00470000`; the map names it
/// `NiTArray<char *,NiTMallocInterface<char *>>::SetAtGrow`) on the array
/// object, at the 16-bit index stored at +0xa, with `value`.
pub fn fn_0096a610(e: &mut Engine, this: Ptr, value: u32) {
    let index = e.mem.u16(this.addr() + 0xa) as u32;
    e.call(ARRAY_SET_AT_GROW, &args![this, index, value]);
}

// Translated from 0096a640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor: runs `0096a7b0(first, second)` and installs vtable
/// `0108b4dc`. Returns `this`.
pub fn fn_0096a640(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    fn_0096a7b0(e, this, first, second);
    e.mem.set_u32(this.addr(), TABLE_ARRAY_DERIVED_VTABLE);
    this
}

// Translated from 0096a670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<unsigned_int_unsigned_char>>,unsigned_int,unsigned_char>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the hash table base destructor `0096a510`; when bit 0 of
/// `flags` is set, frees the object (`00401030`). Returns `this`.
pub fn fn_0096a670(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0096a510(e, this);
    if flags & 1 != 0 {
        e.call(FREE_OBJECT, &args![this]);
    }
    this
}

// Translated from 0096a6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<ItemChange *>::_scalar_deleting_destructor_` (Xbox PDB):
/// runs the destructor `0096a5a0`; when bit 0 of `flags` is set, frees the
/// object (`00401030`). Returns `this`.
pub fn fn_0096a6a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0096a5a0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_OBJECT, &args![this]);
    }
    this
}

// Translated from 0096a6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<TESAmmo *>::_scalar_deleting_destructor_` (Xbox PDB): runs
/// the destructor `0096a5f0`; when bit 0 of `flags` is set, frees the object
/// (`00401030`). Returns `this`.
pub fn fn_0096a6d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0096a5f0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_OBJECT, &args![this]);
    }
    this
}

// Translated from 0096a7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a fixed-capacity array: vtable `0108b4e4`; the 16-bit
/// capacity at +8 and the 16-bit grow size at +0xe come from the two
/// arguments, the 16-bit words at +0xa and +0xc are cleared; the buffer at +4
/// is `0096afc0(capacity)` when the capacity is above zero, otherwise 0.
/// Returns `this`.
pub fn fn_0096a7b0(e: &mut Engine, this: Ptr, capacity: u32, grow: u32) -> Ptr {
    let capacity = capacity as u16;
    e.mem.set_u32(this.addr(), CAPACITY_ARRAY_VTABLE);
    e.mem.set_u16(this.addr() + 8, capacity);
    e.mem.set_u16(this.addr() + 0xe, grow as u16);
    e.mem.set_u16(this.addr() + 0xa, 0);
    e.mem.set_u16(this.addr() + 0xc, 0);
    if capacity > 0 {
        let buffer = e.call(ALLOCATE_WORDS, &args![capacity as u32]).u32();
        e.mem.set_u32(this.addr() + 4, buffer);
    } else {
        e.mem.set_u32(this.addr() + 4, 0);
    }
    this
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00967aa0, fn_00967aa0()),
        entry!(0x00967ac0, fn_00967ac0(Ptr<PlayerCharacter>, u8)),
        entry!(
            0x00967ae0,
            player_character_is_pipboy_active(Ptr<PlayerCharacter>) -> bool
        ),
        entry!(
            0x00967b70,
            fn_00967b70(Ptr<PlayerCharacter>, Ptr<ItemChange>) -> bool
        ),
        entry!(0x00967da0, fn_00967da0(Ptr<PlayerCharacter>) -> u32),
        entry!(0x00968630, fn_00968630(Ptr, u32) -> u32),
        entry!(0x00968650, fn_00968650(Ptr) -> u32),
        entry!(0x00968670, fn_00968670(Ptr, u32) -> u32),
        entry!(0x00968690, fn_00968690(Ptr, Ptr, f32) -> Ptr),
        entry!(0x00968730, fn_00968730(Ptr)),
        entry!(
            0x009687b0,
            player_character_add_rock_it_ammo(Ptr<PlayerCharacter>, u32, u32, i32)
        ),
        entry!(
            0x00968be0,
            player_character_remove_rock_it_ammo(Ptr<PlayerCharacter>, u32, u32, i32, u8) -> u32
        ),
        entry!(0x00968f90, fn_00968f90(Ptr<PlayerCharacter>) -> u32),
        entry!(
            0x00969040,
            player_character_get_num_rock_it_ammo(Ptr<PlayerCharacter>) -> u32
        ),
        entry!(
            0x009690a0,
            player_character_clear_rock_it_ammo(Ptr<PlayerCharacter>)
        ),
        entry!(
            0x00969110,
            player_character_reset_rock_it_ammo_weight(Ptr<PlayerCharacter>, u8)
        ),
        entry!(
            0x009691d0,
            player_character_return_rock_it_ammo(Ptr<PlayerCharacter>)
        ),
        entry!(
            0x00969260,
            fn_00969260(Ptr<PlayerCharacter>, u32, u32, u32, u8)
        ),
        entry!(0x009694d0, fn_009694d0(Ptr<PlayerCharacter>, u32)),
        entry!(0x00969700, fn_00969700(Ptr<PlayerCharacter>) -> bool),
        entry!(0x00969760, fn_00969760(Ptr<PlayerCharacter>) -> bool),
        entry!(0x009697c0, fn_009697c0(Ptr<PlayerCharacter>, u8)),
        entry!(0x00969820, fn_00969820(Ptr<PlayerCharacter>, u8)),
        entry!(0x00969860, fn_00969860(Ptr<PlayerCharacter>) -> bool),
        entry!(0x009698a0, fn_009698a0(Ptr<PlayerCharacter>) -> u32),
        entry!(0x00969910, fn_00969910(Ptr) -> u32),
        entry!(0x00969930, fn_00969930(Ptr<PlayerCharacter>) -> u32),
        entry!(0x00969ac0, fn_00969ac0(Ptr)),
        entry!(0x00969ae0, fn_00969ae0(Ptr)),
        entry!(0x00969b00, fn_00969b00(Ptr)),
        entry!(0x00969b20, fn_00969b20(Ptr)),
        entry!(0x00969b40, fn_00969b40(Ptr) -> i32),
        entry!(0x00969b80, fn_00969b80(Ptr) -> bool),
        entry!(0x00969bc0, fn_00969bc0(Ptr<PlayerCharacter>, u32) -> bool),
        entry!(
            0x00969c30,
            player_character_update_hardcore_mode(Ptr<PlayerCharacter>)
        ),
        entry!(0x00969e70, fn_00969e70(Ptr<PlayerCharacter>) -> u8),
        entry!(
            0x00969e90,
            player_character_set_hardcore(Ptr<PlayerCharacter>, u32, u8)
        ),
        entry!(0x00969fa0, fn_00969fa0(Ptr<PlayerCharacter>)),
        entry!(0x0096a280, fn_0096a280(Ptr, u32) -> Ptr),
        entry!(0x0096a2b0, fn_0096a2b0(Ptr, u32) -> Ptr),
        entry!(0x0096a2d0, fn_0096a2d0(Ptr) -> Ptr),
        entry!(0x0096a300, fn_0096a300(Ptr, u32) -> Ptr),
        entry!(0x0096a330, fn_0096a330(Ptr, u32) -> u32),
        entry!(0x0096a350, fn_0096a350(Ptr) -> u32),
        entry!(0x0096a370, fn_0096a370(Ptr, u32) -> u32),
        entry!(0x0096a390, fn_0096a390(Ptr, u32) -> Ptr),
        entry!(0x0096a400, fn_0096a400(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x0096a4b0, fn_0096a4b0(Ptr)),
        entry!(0x0096a510, fn_0096a510(Ptr)),
        entry!(0x0096a540, fn_0096a540(Ptr, u32)),
        entry!(0x0096a570, fn_0096a570(Ptr) -> Ptr),
        entry!(0x0096a5a0, fn_0096a5a0(Ptr)),
        entry!(0x0096a5c0, fn_0096a5c0(Ptr) -> Ptr),
        entry!(0x0096a5f0, fn_0096a5f0(Ptr)),
        entry!(0x0096a610, fn_0096a610(Ptr, u32)),
        entry!(0x0096a640, fn_0096a640(Ptr, u32, u32) -> Ptr),
        entry!(0x0096a670, fn_0096a670(Ptr, u32) -> Ptr),
        entry!(0x0096a6a0, fn_0096a6a0(Ptr, u32) -> Ptr),
        entry!(0x0096a6d0, fn_0096a6d0(Ptr, u32) -> Ptr),
        entry!(0x0096a7b0, fn_0096a7b0(Ptr, u32, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;
    use std::sync::atomic::{AtomicU32, Ordering};

    static NEXT_TARGET: AtomicU32 = AtomicU32::new(0x0300_0000);

    /// Every external function the translations call; each answers zero until
    /// a test says otherwise.
    const CALLEES: &[u32] = &[
        TIME_IMPORT,
        OPERATOR_NEW,
        FLOAT_TO_INT,
        WORD_AT_0,
        WORD_AT_4,
        WORD_AT_8,
        NODE_ITEM_SLOT,
        ARRAY_ELEMENT_SLOT,
        FORM_TYPE,
        SETTING_FLOAT_POINTER,
        SETTING_INT_POINTER,
        SETTING_STRING,
        PASS_THROUGH,
        WORD_AT_0XA,
        ANIMATION_SLOT,
        ANIMATION_WORD_AT_0X74,
        ANIMATION_GROUP_ID,
        IS_IN_PIPBOY_MENU,
        EXTRA_GET_CAN_NOT_WEAR,
        EXTRA_GET_BY_TYPE,
        GET_FORM_HEALTH,
        FORM_TEST_47BCF0,
        FORM_TEST_80F7D0,
        GET_CURRENT_WEAPON,
        WEAPON_FIRST_FORM,
        WEAPON_SECOND_FORM,
        LIST_FORM_GET_ITEM_INDEX,
        PROCESS_LISTS_ENTRIES,
        GROUP_MEMBER_COUNT,
        GROUP_MEMBER_AT,
        GROUP_TARGET_COUNT,
        GET_FORCE_NEXT_UPDATE,
        CREATE_COMBAT_GROUP,
        GROUP_COPY_TARGETS,
        GROUP_IS_TARGET,
        GET_SHOULD_ATTACK_ACTOR,
        POINT_SUBTRACT,
        POINT_LENGTH,
        GROUP_REMOVE_TARGET,
        GROUP_ADD_MEMBER,
        IS_ANGRY_WITH_PLAYER,
        START_DIALOGUE,
        SET_BYTE_AT_0X125,
        PROCESS_LISTS_ACTOR_NODE,
        GET_CURRENT_PACKAGE,
        PACKAGE_TYPE,
        PACKAGE_0X18_UPDATE,
        PACKAGE_0X16_UPDATE,
        EVALUATE_PACKAGE,
        GROUP_METHOD_9869A0,
        HANDLE_ENTRY,
        ENTRY_DESTROY,
        LIST_REMOVE,
        LIST_POP_FRONT,
        LIST_IS_EMPTY,
        ARRAY_IS_EMPTY,
        RANDOM,
        ITEM_CHANGE_CONSTRUCTOR,
        ITEM_CHANGE_DELETE_ALL_EXTRA,
        SET_NUMBER,
        ARRAY_REMOVE,
        ARRAY_APPEND,
        ARRAY_CLEAR,
        LIST_ADD,
        EXTRA_DATA_LIST_CONSTRUCT,
        EXTRA_DATA_LIST_DUPLICATE,
        EXTRA_IS_DEFAULT_FOR_CONTAINER,
        EXTRA_COMPARE_LIST_FOR_CONTAINER,
        EXTRA_GET_COUNT,
        EXTRA_SET_COUNT,
        GET_FORM_WEIGHT,
        WEIGHT_FLAG_OF_PLAYER,
        GET_INVENTORY_CHANGES,
        INVENTORY_ADD_ITEM_CHANGE,
        CREATE_PACKAGE,
        PACKAGE_SET_FLAG_2,
        PACKAGE_SET_FLAG_4,
        SET_PACKAGE_LOCATION,
        SET_PACKAGE_TARGET,
        PACKAGE_TARGET_OBJECT,
        PACKAGE_LOCATION_CONSTRUCTOR,
        PACKAGE_LOCATION_METHOD_67F140,
        SET_LOC_REFERENCE,
        PACKAGE_LOCATION_DESTRUCTOR,
        PACKAGE_TARGET_CONSTRUCTOR,
        SET_TARG_TYPE,
        SET_TARG_REFERENCE,
        SET_TARGET_WORD_AT_8,
        PACKAGE_TARGET_DESTRUCTOR,
        SET_PACKAGE_WORD_AT_0X18,
        ACTOR_PROCESS,
        PROCESS_LISTS_QUEUE_TARGET,
        PLAYER_SET_MODE_5CC7A0,
        FORCE_TEMP_3RD_PERSON,
        REFERENCE_EARLY_OUT_TEST,
        REFERENCE_AFTER_PACKAGE,
        PLAYER_ANY_FLAG,
        PLAYER_GET_ANIMATION,
        PLAYER_PLAY_NAMED,
        ACTOR_REFRESH_8B78C0,
        ACTOR_REFRESH_8D3FA0,
        POINT_LENGTH_SQUARED,
        REFERENCE_RADIUS,
        AUDIO_MARKER_LOOKUP,
        LIST_CONTAINS,
        SET_CARD_FLAG,
        PLAYER_IS_IN_STATE,
        PLAYER_NEEDS_TEST,
        PLAYER_RESET_NEEDS,
        CALENDAR_GET_TIME_SCALE,
        CALENDAR_GET_VALUE,
        EXTRA_GET_TYPE_0X15,
        RESET_TYPE_0X15,
        EXTRA_LIST_OF_REFERENCE,
        CLEAR_BYTE_AT_0XE38,
        GLOBAL_FLAG_7D6E60,
        SHOW_MESSAGE,
        CREATE_SLEEP_MENU,
        SLEEP_TEST_953C80,
        ACTOR_PROCESS_OF_THIS,
        SLEEP_TEST_885520,
        REFERENCE_IS_INTERIOR,
        PROCESS_LISTS_TEST_9764A0,
        GET_CHAR_CONTROLLER,
        CONTROLLER_STATE,
        CELL_TEST_5444C0,
        SLEEP_TEST_944360,
        FURNITURE_TEST_5098E0,
        MAGIC_TEST_822E00,
        POSITION_OF,
        CONTROLLER_LINEAR_VELOCITY,
        VECTOR_SCALE,
        VECTOR_ADD,
        SET_RADIATION_LEVEL,
        FREE_OBJECT,
        ALLOCATE_BYTES,
        ALLOCATE_WORDS,
        FREE_BYTES,
        FILL_MEMORY,
        HASH_TABLE_EMPTY,
        SUB_OBJECT_RELEASE,
        ARRAY_INITIALISE_SIZED,
        ARRAY_SET_AT_GROW,
    ];

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

    fn double(e: &mut Engine, addr: u32, ret: Ret) {
        e.register_double(addr, move |_, _| ret);
    }

    /// A double that answers the given values in turn.
    fn sequence(e: &mut Engine, addr: u32, values: &[Ret]) {
        let values: Vec<Ret> = values.into();
        let mut next = 0;
        e.register_double(addr, move |_, _| {
            next += 1;
            *values.get(next - 1).expect("call count")
        });
    }

    /// Where the doubles of the setting objects keep a setting's value.
    fn setting_slot(setting: u32) -> u32 {
        0x0600_0000 + (setting & 0xfff) * 0x10 + 4
    }

    /// An engine with the pages the code reads and doubles over every callee;
    /// the small accessors behave as the game's do.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_2000,
            0x0101_6000,
            0x0108_4000,
            0x0108_a000,
            0x011d_e000,
            0x011e_0000,
            0x011f_1000,
        ] {
            e.map(page, 0x1000);
        }
        e.map(0x0600_0000, 0x1_0000);
        for addr in CALLEES {
            double(&mut e, *addr, Ret::default());
        }
        e.register(WORD_AT_0, |e, a| eax(e.mem.u32(a[0])));
        e.register(WORD_AT_4, |e, a| eax(e.mem.u32(a[0] + 4)));
        e.register(WORD_AT_8, |e, a| eax(e.mem.u32(a[0] + 8)));
        e.register(WORD_AT_0XA, |e, a| eax(e.mem.u16(a[0] + 0xa) as u32));
        e.register(NODE_ITEM_SLOT, |_, a| eax(a[0]));
        e.register(ARRAY_ELEMENT_SLOT, |e, a| {
            eax(e.mem.u32(a[0] + 4) + a[1] * 4)
        });
        e.register(FORM_TYPE, |e, a| eax(e.mem.u8(a[0] + 4) as u32));
        e.register(ACTOR_PROCESS, |e, a| eax(e.mem.u32(a[0] + 0x68)));
        e.register(OPERATOR_NEW, |e, a| eax(e.mem.alloc(a[0])));
        e.register(SETTING_FLOAT_POINTER, |_, a| eax(setting_slot(a[0])));
        e.register(SETTING_INT_POINTER, |_, a| eax(setting_slot(a[0])));
        e.register(SETTING_STRING, |_, a| eax(setting_slot(a[0])));
        e.register(PASS_THROUGH, |_, a| eax(a[0]));
        e.register(
            PACKAGE_TYPE,
            |e, a| eax(e.mem.i8(a[0] + 0x20) as i32 as u32),
        );
        e.register(PACKAGE_TARGET_OBJECT, |e, a| eax(e.mem.u32(a[0] + 0x30)));
        e.register(GROUP_MEMBER_COUNT, |e, a| eax(e.mem.u32(a[0] + 0x20)));
        e.register(GROUP_MEMBER_AT, |e, a| {
            eax(e.mem.u32(e.mem.u32(a[0] + 0x1c) + a[1] * 0x14))
        });
        e.register(GROUP_TARGET_COUNT, |e, a| eax(e.mem.u32(a[0] + 0x10)));
        e.register(LIST_IS_EMPTY, |e, a| {
            eax((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        e.register(
            ARRAY_IS_EMPTY,
            |e, a| eax((e.mem.u32(a[0] + 8) == 0) as u32),
        );
        e.register(SET_NUMBER, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            eax(0)
        });
        e.register(ITEM_CHANGE_CONSTRUCTOR, |e, a| {
            let list = e.mem.alloc(8);
            e.mem.set_u32(a[0], list);
            e.mem.set_u32(a[0] + 4, a[2]);
            e.mem.set_u32(a[0] + 8, a[1]);
            eax(a[0])
        });
        e.register(ARRAY_APPEND, |e, a| {
            let count = e.mem.u32(a[0] + 8);
            let buffer = e.mem.u32(a[0] + 4);
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(buffer + 4 * count, item);
            e.mem.set_u32(a[0] + 8, count + 1);
            eax(0)
        });
        e.register(ARRAY_REMOVE, |e, a| {
            let count = e.mem.u32(a[0] + 8);
            let buffer = e.mem.u32(a[0] + 4);
            for i in a[1]..count - 1 {
                let next = e.mem.u32(buffer + 4 * (i + 1));
                e.mem.set_u32(buffer + 4 * i, next);
            }
            e.mem.set_u32(a[0] + 8, count - 1);
            eax(0)
        });
        e.register(ARRAY_CLEAR, |e, a| {
            e.mem.set_u32(a[0] + 8, 0);
            eax(0)
        });
        e.register(LIST_ADD, |e, a| {
            let item = e.mem.u32(a[1]);
            let mut node = a[0];
            if e.mem.u32(node) != 0 {
                while e.mem.u32(node + 4) != 0 {
                    node = e.mem.u32(node + 4);
                }
                let created = e.mem.alloc(8);
                e.mem.set_u32(node + 4, created);
                node = created;
            }
            e.mem.set_u32(node, item);
            eax(0)
        });
        e.register(EXTRA_DATA_LIST_CONSTRUCT, |_, a| eax(a[0]));
        e.register(PACKAGE_LOCATION_CONSTRUCTOR, |_, a| eax(a[0]));
        e.register(PACKAGE_TARGET_CONSTRUCTOR, |_, a| eax(a[0]));
        e.register(FLOAT_TO_INT, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            eax(value as i32 as u32)
        });
        e
    }

    /// The player, registered as the player global.
    fn new_player(e: &mut Engine) -> Ptr<PlayerCharacter> {
        let player = e.new_object::<PlayerCharacter>();
        e.set_global(PLAYER_POINTER, player.addr());
        player
    }

    /// Puts a double answering `ret` into slot `offset` of the object's
    /// vtable (a fresh table when it has none) and returns the double's
    /// address.
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

    /// The argument words of every logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .expect("call log")
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A `BSSimpleList` of the items: nodes of (item, next); an empty list is
    /// a zeroed head node.
    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        if next == 0 {
            e.mem.alloc(8)
        } else {
            next
        }
    }

    /// An `ItemChange`: extra data list head (none when `extras` is empty),
    /// number, form.
    fn item(e: &mut Engine, form: u32, number: u32, extras: &[u32]) -> u32 {
        let entry = e.mem.alloc(0xc);
        let head = if extras.is_empty() {
            0
        } else {
            list(e, extras)
        };
        e.mem.set_u32(entry, head);
        e.mem.set_u32(entry + 4, number);
        e.mem.set_u32(entry + 8, form);
        entry
    }

    /// Fills the player's RockIt ammunition array with the items.
    fn put_ammo(e: &mut Engine, player: Ptr<PlayerCharacter>, items: &[u32]) {
        let array = player.at(PlayerCharacter::RockItLauncherAmmoList).addr();
        let buffer = e.mem.alloc(0x40);
        e.mem.set_u32(array + 4, buffer);
        e.mem.set_u32(array + 8, items.len() as u32);
        for (i, entry) in items.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *entry);
        }
    }

    /// The items of the player's RockIt ammunition array.
    fn ammo(e: &Engine, player: Ptr<PlayerCharacter>) -> Vec<u32> {
        let array = player.at(PlayerCharacter::RockItLauncherAmmoList).addr();
        let buffer = e.mem.u32(array + 4);
        (0..e.mem.u32(array + 8))
            .map(|i| e.mem.u32(buffer + 4 * i))
            .collect()
    }

    /// An engine where every form weighs `weight` per unit.
    fn weighing(weight: f64) -> Engine {
        let mut e = engine();
        double(&mut e, GET_FORM_WEIGHT, st0(weight));
        e
    }

    /// A form object of the given type byte.
    fn form(e: &mut Engine, kind: u8) -> u32 {
        let form = e.mem.alloc(0x20);
        e.mem.set_u8(form + 4, kind);
        form
    }

    #[test]
    fn clearing_the_byte_goes_through_the_player_global() {
        let mut e = engine();
        let player = new_player(&mut e);
        e.set(player, PlayerCharacter::bSpeaking, 1);
        e.call(0x00967aa0, &args![]);
        assert_eq!(e.get(player, PlayerCharacter::bSpeaking), 0);
    }

    #[test]
    fn the_byte_is_stored_as_given() {
        let mut e = engine();
        let player = new_player(&mut e);
        e.call(0x00967ac0, &args![player, 7u32]);
        assert_eq!(e.get(player, PlayerCharacter::bSpeaking), 7);
    }

    #[test]
    fn pipboy_is_active_by_animation_group_or_menu() {
        // No first person animation: the menu decides.
        let mut e = engine();
        let player = new_player(&mut e);
        assert!(!e.call(0x00967ae0, &args![player]).bool());
        double(&mut e, IS_IN_PIPBOY_MENU, eax(1));
        assert!(e.call(0x00967ae0, &args![player]).bool());

        // An animation whose sequence is in group 0xe2 or 0xf0.
        for (group, expected) in [(0xe2, true), (0xf0, true), (7, false)] {
            let mut e = engine();
            let player = new_player(&mut e);
            let animation = e.mem.alloc(0x100);
            e.set(
                player,
                PlayerCharacter::p1stPersonAnimation,
                Ptr::new(animation),
            );
            double(&mut e, ANIMATION_SLOT, eax(0x55));
            double(&mut e, ANIMATION_GROUP_ID, eax(group));
            assert_eq!(e.call(0x00967ae0, &args![player]).bool(), expected);
        }

        // No sequence: the group is never asked, the menu decides.
        let mut e = engine();
        let player = new_player(&mut e);
        let animation = e.mem.alloc(0x100);
        e.set(
            player,
            PlayerCharacter::p1stPersonAnimation,
            Ptr::new(animation),
        );
        double(&mut e, ANIMATION_GROUP_ID, eax(0xe2));
        assert!(!e.call(0x00967ae0, &args![player]).bool());
    }

    /// An inventory entry whose form has the type and is not a quest object
    /// (slot 0x94 answers false).
    fn usable_setup(kind: u8) -> (Engine, Ptr<PlayerCharacter>, u32, u32) {
        let mut e = engine();
        let player = new_player(&mut e);
        let form = form(&mut e, kind);
        slot(&mut e, form, 0x94, eax(0));
        let entry = item(&mut e, form, 1, &[]);
        (e, player, form, entry)
    }

    #[test]
    fn the_usability_test_goes_by_form_type() {
        // Weapon or armour: needs health above zero.
        let (mut e, player, _, entry) = usable_setup(0x28);
        double(&mut e, GET_FORM_HEALTH, eax(10));
        assert!(e.call(0x00967b70, &args![player, entry]).bool());
        double(&mut e, GET_FORM_HEALTH, eax(0));
        assert!(!e.call(0x00967b70, &args![player, entry]).bool());

        // The health extra data's float wins over the form's health.
        let (mut e, player, form, _) = usable_setup(0x18);
        double(&mut e, GET_FORM_HEALTH, eax(10));
        let extra = e.mem.alloc(0x20);
        let health = e.mem.alloc(0x20);
        e.mem.set_f32(health + 0xc, -1.0);
        double(&mut e, EXTRA_GET_BY_TYPE, eax(health));
        let entry = item(&mut e, form, 1, &[extra]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00967b70, &args![player, entry]).bool());
        assert_eq!(calls(&e, EXTRA_GET_BY_TYPE), vec![vec![extra, 0x25]]);

        // Types that always pass, and the ones that never do.
        for (kind, expected) in [
            (0x19, true),
            (0x1d, true),
            (0x1e, true),
            (0x2f, true),
            (0x30, false),
        ] {
            let (mut e, player, _, entry) = usable_setup(kind);
            assert_eq!(e.call(0x00967b70, &args![player, entry]).bool(), expected);
        }

        // An extra data list that cannot be worn rules the entry out.
        let (mut e, player, form, _) = usable_setup(0x19);
        let extra = e.mem.alloc(0x20);
        let entry = item(&mut e, form, 1, &[extra]);
        double(&mut e, EXTRA_GET_CAN_NOT_WEAR, eax(1));
        assert!(!e.call(0x00967b70, &args![player, entry]).bool());
    }

    #[test]
    fn a_quest_object_is_only_usable_as_weapon_or_armour_passing_its_test() {
        // Weapon (0x28) or armour (0x18): `0047bcf0` decides.
        for kind in [0x28u8, 0x18] {
            let (mut e, player, form, entry) = usable_setup(kind);
            slot(&mut e, form, 0x94, eax(1));
            double(&mut e, GET_FORM_HEALTH, eax(10));
            double(&mut e, FORM_TEST_47BCF0, eax(1));
            assert!(e.call(0x00967b70, &args![player, entry]).bool());
            double(&mut e, FORM_TEST_47BCF0, eax(0));
            assert!(!e.call(0x00967b70, &args![player, entry]).bool());
        }
        // Any other type is ruled out.
        let (mut e, player, form, entry) = usable_setup(0x19);
        slot(&mut e, form, 0x94, eax(1));
        double(&mut e, FORM_TEST_47BCF0, eax(1));
        assert!(!e.call(0x00967b70, &args![player, entry]).bool());
    }

    #[test]
    fn the_usability_test_of_type_0x1a_asks_the_form_test() {
        let (mut e, player, form, _) = usable_setup(0x1a);
        let extra = e.mem.alloc(0x20);
        let entry = item(&mut e, form, 1, &[extra]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00967b70, &args![player, entry]).bool());
        assert_eq!(
            calls(&e, FORM_TEST_80F7D0),
            vec![vec![player.addr(), form, extra]]
        );
        double(&mut e, FORM_TEST_80F7D0, eax(1));
        assert!(e.call(0x00967b70, &args![player, entry]).bool());
    }

    #[test]
    fn the_usability_test_of_type_0x29_follows_the_current_weapon() {
        let (mut e, player, form, entry) = usable_setup(0x29);
        // No current weapon.
        assert!(!e.call(0x00967b70, &args![player, entry]).bool());
        // The weapon's first form is the entry's form.
        let weapon = e.mem.alloc(0x100);
        double(&mut e, GET_CURRENT_WEAPON, eax(weapon));
        double(&mut e, WEAPON_FIRST_FORM, eax(form));
        assert!(e.call(0x00967b70, &args![player, entry]).bool());
        // Another first form and no form list.
        double(&mut e, WEAPON_FIRST_FORM, eax(0x1234));
        assert!(!e.call(0x00967b70, &args![player, entry]).bool());
        // A form list that does not hold the form, then one that does.
        let forms = e.mem.alloc(0x20);
        double(&mut e, WEAPON_SECOND_FORM, eax(forms));
        double(&mut e, LIST_FORM_GET_ITEM_INDEX, eax(u32::MAX));
        assert!(!e.call(0x00967b70, &args![player, entry]).bool());
        double(&mut e, LIST_FORM_GET_ITEM_INDEX, eax(2));
        assert!(e.call(0x00967b70, &args![player, entry]).bool());
    }

    /// A combat group with the actors as members (0x14-byte entries at +0x1c,
    /// count at +0x20) and targets (0x68-byte entries at +0xc, count at +0x10).
    fn group(e: &mut Engine, members: &[u32], targets: &[u32]) -> u32 {
        let group = e.mem.alloc(0x40);
        let member_buffer = e.mem.alloc(0x14 * members.len().max(1) as u32);
        for (i, actor) in members.iter().enumerate() {
            e.mem.set_u32(member_buffer + 0x14 * i as u32, *actor);
        }
        e.mem.set_u32(group + 0x1c, member_buffer);
        e.mem.set_u32(group + 0x20, members.len() as u32);
        let target_buffer = e.mem.alloc(0x68 * targets.len().max(1) as u32);
        for (i, actor) in targets.iter().enumerate() {
            e.mem.set_u32(target_buffer + 0x68 * i as u32, *actor);
        }
        e.mem.set_u32(group + 0xc, target_buffer);
        e.mem.set_u32(group + 0x10, targets.len() as u32);
        group
    }

    /// The combat manager with the groups: array pointer at +4, number of
    /// groups as the 16-bit word at +0xa.
    fn manager(e: &mut Engine, groups: &[u32]) -> u32 {
        let manager = e.mem.alloc(0x20);
        let buffer = e.mem.alloc(4 * groups.len().max(1) as u32);
        for (i, group) in groups.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *group);
        }
        e.mem.set_u32(manager + 4, buffer);
        e.mem.set_u16(manager + 0xa, groups.len() as u16);
        e.set_global(COMBAT_MANAGER, manager);
        manager
    }

    #[test]
    fn the_combat_clean_up_without_entries_does_nothing() {
        let mut e = engine();
        let player = new_player(&mut e);
        manager(&mut e, &[]);
        e.mem.set_u8(player.addr() + 0xdf0, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00967da0, &args![player]).u32(), 0);
        assert_eq!(e.get(player, PlayerCharacter::bPlayerInCombat), 1);
        assert!(calls(&e, GROUP_IS_TARGET).is_empty());
        assert_eq!(calls(&e, PROCESS_LISTS_ENTRIES).len(), 1);
    }

    #[test]
    fn the_combat_clean_up_drops_a_group_that_would_not_attack() {
        let mut e = engine();
        let player = new_player(&mut e);
        e.mem.set_f32(setting_slot(SETTING_COMBAT_RADIUS), 7.0);
        // One other group with one member, which has an entry in the list.
        let actor = e.mem.alloc(0x40);
        let position = e.mem.alloc(0x10);
        slot(&mut e, actor, 0x1f4, eax(position));
        let other = group(&mut e, &[actor], &[]);
        manager(&mut e, &[other]);
        let entry = e.mem.alloc(0x20);
        e.mem.set_u32(entry + 8, actor);
        let entries = list(&mut e, &[entry]);
        double(&mut e, PROCESS_LISTS_ENTRIES, eax(entries));
        double(&mut e, GROUP_IS_TARGET, eax(1));
        double(&mut e, POINT_LENGTH, st0(5.0));
        e.register(LIST_POP_FRONT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            eax(0)
        });
        // The player's own position, for the distance.
        slot(&mut e, player.addr(), 0x1f4, eax(position));
        let dialogue = e.mem.alloc(0x10);
        e.set_global(COMBAT_DIALOGUE_MANAGER, dialogue);
        e.set_global(NEAREST_ACTOR_START, 1.0e30f32);
        e.set(player, PlayerCharacter::bPlayerInCombat, 1);
        e.call_log = Some(vec![]);

        assert_eq!(e.call(0x00967da0, &args![player]).u32(), 1);
        assert_eq!(
            calls(&e, PROCESS_LISTS_ENTRIES),
            vec![vec![PROCESS_LISTS, 7.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, GROUP_REMOVE_TARGET),
            vec![vec![other, player.addr()]]
        );
        assert_eq!(calls(&e, ENTRY_DESTROY), vec![vec![entry, 1]]);
        assert_eq!(
            calls(&e, START_DIALOGUE),
            vec![vec![dialogue, actor, player.addr(), 2, 0xb, 1, 0]]
        );
        assert!(calls(&e, HANDLE_ENTRY).is_empty());
        assert_eq!(e.get(player, PlayerCharacter::bPlayerInCombat), 0);
    }

    #[test]
    fn the_combat_clean_up_keeps_a_group_that_would_attack() {
        let mut e = engine();
        let player = new_player(&mut e);
        let actor = e.mem.alloc(0x40);
        let other = group(&mut e, &[actor], &[]);
        manager(&mut e, &[other]);
        let entry = e.mem.alloc(0x20);
        e.mem.set_u32(entry + 8, actor);
        let entries = list(&mut e, &[entry]);
        double(&mut e, PROCESS_LISTS_ENTRIES, eax(entries));
        double(&mut e, GROUP_IS_TARGET, eax(1));
        double(&mut e, GET_SHOULD_ATTACK_ACTOR, eax(1));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00967da0, &args![player]).u32(), 0);
        assert!(calls(&e, GROUP_REMOVE_TARGET).is_empty());
        // The entry that stayed in the list is handed on.
        assert_eq!(calls(&e, HANDLE_ENTRY), vec![vec![actor, entry]]);
    }

    #[test]
    fn the_combat_clean_up_moves_free_members_to_a_new_group() {
        let mut e = engine();
        let player = new_player(&mut e);
        let member = e.mem.alloc(0x40);
        let second = e.mem.alloc(0x40);
        let player_group = group(&mut e, &[member], &[]);
        e.set(
            player,
            PlayerCharacter::pCombatGroup,
            Ptr::new(player_group),
        );
        let manager = manager(&mut e, &[]);
        let created = group(&mut e, &[second], &[]);
        let entries = list(&mut e, &[]);
        double(&mut e, PROCESS_LISTS_ENTRIES, eax(entries));
        double(&mut e, CREATE_COMBAT_GROUP, eax(created));
        let target = slot(&mut e, second, 0x3fc, eax(0));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00967da0, &args![player]).u32(), 0);
        assert_eq!(calls(&e, CREATE_COMBAT_GROUP), vec![vec![manager]]);
        assert_eq!(
            calls(&e, GROUP_COPY_TARGETS),
            vec![vec![created, player_group]]
        );
        assert_eq!(calls(&e, target), vec![vec![second, created]]);
        assert_eq!(
            calls(&e, GROUP_METHOD_9869A0),
            vec![vec![player_group, second]]
        );
    }

    #[test]
    fn the_combat_clean_up_evaluates_the_packages_of_the_actors() {
        let mut e = engine();
        let player = new_player(&mut e);
        manager(&mut e, &[]);
        let entries = list(&mut e, &[]);
        double(&mut e, PROCESS_LISTS_ENTRIES, eax(entries));
        // Three actors whose current packages have types 0x18, 0x16 and 0x0a,
        // and one with another type.
        let actors: Vec<u32> = (0..4).map(|_| e.mem.alloc(0x40)).collect();
        let packages: Vec<u32> = [0x18u8, 0x16, 0x0a, 0x05]
            .iter()
            .map(|kind| {
                let package = e.mem.alloc(0x40);
                e.mem.set_u8(package + 0x20, *kind);
                package
            })
            .collect();
        let head = list(&mut e, &actors);
        double(&mut e, PROCESS_LISTS_ACTOR_NODE, eax(head));
        let current = packages.clone();
        let table = actors.clone();
        e.register_double(GET_CURRENT_PACKAGE, move |_, a| {
            let at = table.iter().position(|actor| *actor == a[0]).unwrap();
            eax(current[at])
        });
        e.call_log = Some(vec![]);
        e.call(0x00967da0, &args![player]);
        assert_eq!(
            calls(&e, PACKAGE_0X18_UPDATE),
            vec![vec![packages[0], player.addr()]]
        );
        assert_eq!(
            calls(&e, PACKAGE_0X16_UPDATE),
            vec![vec![packages[1], player.addr()]]
        );
        assert_eq!(
            calls(&e, EVALUATE_PACKAGE),
            vec![
                vec![actors[0], 0, 0],
                vec![actors[1], 0, 0],
                vec![actors[2], 0, 0]
            ]
        );
    }

    #[test]
    fn the_target_array_element_is_the_first_word_of_the_entry() {
        let mut e = engine();
        let group = group(&mut e, &[], &[0x111, 0x222, 0x333]);
        assert_eq!(e.call(0x00968630, &args![group, 1u32]).u32(), 0x222);
        assert_eq!(e.call(0x00968630, &args![group, 2u32]).u32(), 0x333);
    }

    #[test]
    fn the_group_count_is_the_word_at_0xa() {
        let mut e = engine();
        let manager = manager(&mut e, &[0x10, 0x20, 0x30]);
        assert_eq!(e.call(0x00968650, &args![manager]).u32(), 3);
        // Only the low 16 bits count.
        e.mem.set_u16(manager + 8, 0xffff);
        assert_eq!(e.call(0x00968650, &args![manager]).u32(), 3);
    }

    #[test]
    fn the_group_at_an_index_is_read_from_the_pointer_array() {
        let mut e = engine();
        let manager = manager(&mut e, &[0x10, 0x20, 0x30]);
        assert_eq!(e.call(0x00968670, &args![manager, 2u32]).u32(), 0x30);
    }

    #[test]
    fn the_predicted_position_adds_the_velocity_times_the_time() {
        let mut e = engine();
        let reference = e.mem.alloc(0x40);
        let position = e.mem.alloc(0x10);
        for (i, v) in [10.0f32, 20.0, 30.0].iter().enumerate() {
            e.mem.set_f32(position + 4 * i as u32, *v);
        }
        double(&mut e, POSITION_OF, eax(position));
        double(&mut e, GET_CHAR_CONTROLLER, eax(0x77));
        e.register(CONTROLLER_LINEAR_VELOCITY, |e, a| {
            for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, *v);
            }
            eax(a[1])
        });
        // out = vector * scale, like the game's vector scaling.
        e.register(VECTOR_SCALE, |e, a| {
            let scale = f32::from_bits(a[2]);
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) * scale;
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            eax(a[1])
        });
        e.register(VECTOR_ADD, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[1] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            eax(a[0])
        });
        let result = e.mem.alloc(0x10);
        assert_eq!(
            e.call(0x00968690, &args![reference, result, 2.0f32]).u32(),
            result
        );
        let moved: Vec<f32> = (0..3).map(|i| e.mem.f32(result + 4 * i)).collect();
        assert_eq!(moved, vec![12.0, 24.0, 36.0]);
        // No time, or no controller: the position as it is.
        e.call(0x00968690, &args![reference, result, 0.0f32]);
        let same: Vec<f32> = (0..3).map(|i| e.mem.f32(result + 4 * i)).collect();
        assert_eq!(same, vec![10.0, 20.0, 30.0]);
        double(&mut e, GET_CHAR_CONTROLLER, eax(0));
        e.call(0x00968690, &args![reference, result, 2.0f32]);
        let same: Vec<f32> = (0..3).map(|i| e.mem.f32(result + 4 * i)).collect();
        assert_eq!(same, vec![10.0, 20.0, 30.0]);
    }

    #[test]
    fn the_radiation_level_is_the_sum_and_the_magic_delta() {
        let mut e = engine();
        let actor = e.mem.alloc(0x100);
        let process = e.mem.alloc(0x10);
        e.mem.set_u32(actor + 0x68, process);
        slot(&mut e, process, 0x764, st0(1.5));
        slot(&mut e, process, 0x76c, st0(2.0));
        slot(&mut e, process, 0x75c, st0(0.5));
        e.call_log = Some(vec![]);
        e.call(0x00968730, &args![actor]);
        assert_eq!(
            calls(&e, SET_RADIATION_LEVEL),
            vec![vec![0.5f32.to_bits(), 3.5f32.to_bits()]]
        );
    }

    #[test]
    fn adding_ammo_appends_a_new_entry_and_adds_the_weight() {
        let mut e = weighing(0.5);
        let player = new_player(&mut e);
        put_ammo(&mut e, player, &[]);
        let call = args![player, 0x1000u32, 0u32, 3u32];
        e.call(0x009687b0, &call);
        let entries = ammo(&e, player);
        assert_eq!(entries.len(), 1);
        assert_eq!(e.mem.u32(entries[0] + 8), 0x1000);
        assert_eq!(e.mem.u32(entries[0] + 4), 3);
        assert_eq!(
            e.get(player, PlayerCharacter::fRockItLauncherAmmoWeight),
            1.5
        );
    }

    #[test]
    fn adding_ammo_with_extra_data_appends_a_copy_of_it() {
        let mut e = weighing(1.0);
        let player = new_player(&mut e);
        put_ammo(&mut e, player, &[]);
        let extra = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x009687b0, &args![player, 0x1000u32, extra, 2u32]);
        let entries = ammo(&e, player);
        assert_eq!(entries.len(), 1);
        let copy = e.mem.u32(e.mem.u32(entries[0]));
        assert_ne!(copy, 0);
        assert_eq!(
            calls(&e, EXTRA_DATA_LIST_DUPLICATE),
            vec![vec![copy, extra]]
        );
        assert_eq!(calls(&e, EXTRA_SET_COUNT), vec![vec![copy, 2]]);
    }

    #[test]
    fn adding_ammo_merges_into_an_entry_with_default_extra_data() {
        let mut e = weighing(0.5);
        let player = new_player(&mut e);
        let entry = item(&mut e, 0x1000, 2, &[]);
        put_ammo(&mut e, player, &[entry]);
        e.set(player, PlayerCharacter::fRockItLauncherAmmoWeight, 1.0);
        e.call(0x009687b0, &args![player, 0x1000u32, 0u32, 3u32]);
        assert_eq!(ammo(&e, player), vec![entry]);
        assert_eq!(e.mem.u32(entry + 4), 5);
        assert_eq!(
            e.get(player, PlayerCharacter::fRockItLauncherAmmoWeight),
            2.5
        );

        // Another form: a second entry.
        e.call(0x009687b0, &args![player, 0x2000u32, 0u32, 1u32]);
        assert_eq!(ammo(&e, player).len(), 2);

        // An entry with non-default extra data does not take a plain addition.
        let mut e = weighing(0.5);
        let player = new_player(&mut e);
        let extra = e.mem.alloc(0x20);
        let entry = item(&mut e, 0x1000, 2, &[extra]);
        put_ammo(&mut e, player, &[entry]);
        double(&mut e, EXTRA_IS_DEFAULT_FOR_CONTAINER, eax(0));
        e.call(0x009687b0, &args![player, 0x1000u32, 0u32, 3u32]);
        assert_eq!(ammo(&e, player).len(), 2);
        assert_eq!(e.mem.u32(entry + 4), 2);
    }

    #[test]
    fn adding_ammo_merges_equal_extra_data_lists() {
        let mut e = weighing(0.5);
        let player = new_player(&mut e);
        let held = e.mem.alloc(0x20);
        let entry = item(&mut e, 0x1000, 4, &[held]);
        put_ammo(&mut e, player, &[entry]);
        let extra = e.mem.alloc(0x20);
        double(&mut e, EXTRA_GET_COUNT, eax(4));
        e.call_log = Some(vec![]);
        e.call(0x009687b0, &args![player, 0x1000u32, extra, 3u32]);
        assert_eq!(ammo(&e, player), vec![entry]);
        assert_eq!(e.mem.u32(entry + 4), 7);
        assert_eq!(calls(&e, EXTRA_SET_COUNT), vec![vec![held, 7]]);
        assert_eq!(
            calls(&e, EXTRA_COMPARE_LIST_FOR_CONTAINER),
            vec![vec![held, extra, 1, 0], vec![extra, held, 1, 0]]
        );

        // Lists that differ: a default extra data list is added to the entry's
        // own list; one that is not default makes a new entry.
        let mut e = weighing(0.5);
        let player = new_player(&mut e);
        let held = e.mem.alloc(0x20);
        let entry = item(&mut e, 0x1000, 4, &[held]);
        put_ammo(&mut e, player, &[entry]);
        let extra = e.mem.alloc(0x20);
        double(&mut e, EXTRA_COMPARE_LIST_FOR_CONTAINER, eax(1));
        // The entry's own list is default, the new one is default.
        double(&mut e, EXTRA_IS_DEFAULT_FOR_CONTAINER, eax(1));
        e.call(0x009687b0, &args![player, 0x1000u32, extra, 2u32]);
        assert_eq!(ammo(&e, player), vec![entry]);
        assert_eq!(e.mem.u32(entry + 4), 6);
        let second = e.mem.u32(e.mem.u32(entry) + 4);
        assert_ne!(second, 0);
        assert_ne!(e.mem.u32(second), 0);
    }

    #[test]
    fn removing_ammo_takes_part_of_an_entry_or_all_of_it() {
        let mut e = weighing(0.5);
        let player = new_player(&mut e);
        let entry = item(&mut e, 0x1000, 5, &[]);
        let other = item(&mut e, 0x2000, 1, &[]);
        put_ammo(&mut e, player, &[other, entry]);
        e.set(player, PlayerCharacter::fRockItLauncherAmmoWeight, 3.0);

        // Part of it, nothing kept.
        let kept = e
            .call(0x00968be0, &args![player, 0x1000u32, 0u32, 2u32, 0u32])
            .u32();
        assert_eq!(kept, 0);
        assert_eq!(e.mem.u32(entry + 4), 3);
        assert_eq!(
            e.get(player, PlayerCharacter::fRockItLauncherAmmoWeight),
            2.0
        );

        // All of it, kept: the entry leaves the array and a new one is returned.
        let kept = e
            .call(0x00968be0, &args![player, 0x1000u32, 0u32, 3u32, 1u32])
            .u32();
        assert_eq!(ammo(&e, player), vec![other]);
        assert_ne!(kept, 0);
        assert_eq!(e.mem.u32(kept + 8), 0x1000);
        assert_eq!(e.mem.u32(kept + 4), 3);
        assert_eq!(
            e.get(player, PlayerCharacter::fRockItLauncherAmmoWeight),
            0.5
        );

        // An entry that is not there: nothing changes but the weight.
        let kept = e
            .call(0x00968be0, &args![player, 0x3000u32, 0u32, 1u32, 1u32])
            .u32();
        assert_eq!(kept, 0);
        assert_eq!(ammo(&e, player), vec![other]);
    }

    #[test]
    fn removing_ammo_with_extra_data_splits_or_removes_the_list() {
        // Fewer taken than the list's count: the list keeps the rest and a
        // copy goes into the returned entry.
        let mut e = weighing(1.0);
        let player = new_player(&mut e);
        let extra = e.mem.alloc(0x20);
        let entry = item(&mut e, 0x1000, 5, &[extra]);
        put_ammo(&mut e, player, &[entry]);
        double(&mut e, EXTRA_GET_COUNT, eax(4));
        e.call_log = Some(vec![]);
        let kept = e
            .call(0x00968be0, &args![player, 0x1000u32, extra, 3u32, 1u32])
            .u32();
        assert_eq!(e.mem.u32(entry + 4), 2);
        assert_eq!(calls(&e, EXTRA_SET_COUNT).last(), Some(&vec![extra, 1]));
        let copy = e.mem.u32(e.mem.u32(kept));
        assert_ne!(copy, 0);
        assert_ne!(copy, extra);
        assert_eq!(
            calls(&e, EXTRA_DATA_LIST_DUPLICATE),
            vec![vec![copy, extra]]
        );
        assert_eq!(e.mem.u32(kept + 4), 3);

        // The whole list goes: it is unlinked and, when nothing is kept,
        // deleted through its destructor.
        let mut e = weighing(1.0);
        let player = new_player(&mut e);
        let extra = e.mem.alloc(0x20);
        let entry = item(&mut e, 0x1000, 3, &[extra]);
        put_ammo(&mut e, player, &[entry]);
        double(&mut e, EXTRA_GET_COUNT, eax(3));
        let destructor = slot(&mut e, extra, 0, eax(0));
        e.call_log = Some(vec![]);
        e.call(0x00968be0, &args![player, 0x1000u32, extra, 3u32, 0u32]);
        assert_eq!(calls(&e, LIST_REMOVE).len(), 1);
        assert_eq!(calls(&e, destructor), vec![vec![extra, 1]]);
        assert!(ammo(&e, player).is_empty());
    }

    #[test]
    fn removing_a_random_ammo_entry_takes_one_unit() {
        let mut e = weighing(0.5);
        let player = new_player(&mut e);
        // Nothing to take from an empty list.
        put_ammo(&mut e, player, &[]);
        assert_eq!(e.call(0x00968f90, &args![player]).u32(), 0);
        let first = item(&mut e, 0x1000, 1, &[]);
        let second = item(&mut e, 0x2000, 4, &[]);
        put_ammo(&mut e, player, &[first, second]);
        // 7 % 2 = 1: the second entry.
        double(&mut e, RANDOM, eax(7));
        let taken = e.call(0x00968f90, &args![player]).u32();
        assert_eq!(e.mem.u32(second + 4), 3);
        assert_eq!(e.mem.u32(taken + 8), 0x2000);
        assert_eq!(e.mem.u32(taken + 4), 1);
        assert_eq!(ammo(&e, player), vec![first, second]);
    }

    #[test]
    fn the_ammo_total_is_the_sum_of_the_numbers() {
        let mut e = engine();
        let player = new_player(&mut e);
        let items: Vec<u32> = [2u32, 3, 4]
            .iter()
            .map(|n| item(&mut e, 0x1000, *n, &[]))
            .collect();
        put_ammo(&mut e, player, &items);
        assert_eq!(e.call(0x00969040, &args![player]).u32(), 9);
        put_ammo(&mut e, player, &[]);
        assert_eq!(e.call(0x00969040, &args![player]).u32(), 0);
    }

    #[test]
    fn clearing_the_ammo_empties_the_array_and_the_weight() {
        let mut e = weighing(0.5);
        let player = new_player(&mut e);
        let first = item(&mut e, 0x1000, 1, &[]);
        let second = item(&mut e, 0x2000, 2, &[]);
        put_ammo(&mut e, player, &[first, second]);
        e.set(player, PlayerCharacter::fRockItLauncherAmmoWeight, 4.0);
        e.call_log = Some(vec![]);
        e.call(0x009690a0, &args![player]);
        assert_eq!(
            calls(&e, ITEM_CHANGE_DELETE_ALL_EXTRA),
            vec![vec![first], vec![second]]
        );
        assert!(ammo(&e, player).is_empty());
        assert_eq!(
            e.get(player, PlayerCharacter::fRockItLauncherAmmoWeight),
            0.0
        );
    }

    #[test]
    fn the_ammo_weight_is_zero_or_recomputed() {
        let mut e = weighing(0.5);
        let player = new_player(&mut e);
        let first = item(&mut e, 0x1000, 2, &[]);
        let second = item(&mut e, 0x2000, 3, &[]);
        put_ammo(&mut e, player, &[first, second]);
        e.set(player, PlayerCharacter::fRockItLauncherAmmoWeight, 9.0);
        e.call(0x00969110, &args![player, 0u32]);
        assert_eq!(
            e.get(player, PlayerCharacter::fRockItLauncherAmmoWeight),
            0.0
        );
        e.call(0x00969110, &args![player, 1u32]);
        assert_eq!(
            e.get(player, PlayerCharacter::fRockItLauncherAmmoWeight),
            2.5
        );
    }

    #[test]
    fn returning_the_ammo_hands_every_entry_to_the_inventory() {
        let mut e = weighing(0.5);
        let player = new_player(&mut e);
        let first = item(&mut e, 0x1000, 2, &[]);
        let second = item(&mut e, 0x2000, 3, &[]);
        put_ammo(&mut e, player, &[first, second]);
        e.set(player, PlayerCharacter::fRockItLauncherAmmoWeight, 2.5);
        double(&mut e, GET_INVENTORY_CHANGES, eax(0xabc));
        e.call_log = Some(vec![]);
        e.call(0x009691d0, &args![player]);
        assert_eq!(
            calls(&e, INVENTORY_ADD_ITEM_CHANGE),
            vec![vec![0xabc, first, 1], vec![0xabc, second, 1]]
        );
        assert_eq!(calls(&e, GET_INVENTORY_CHANGES), vec![vec![player.addr()]]);
        assert!(ammo(&e, player).is_empty());
        assert_eq!(
            e.get(player, PlayerCharacter::fRockItLauncherAmmoWeight),
            0.0
        );

        // Without inventory changes nothing is handed on but the list is still
        // emptied.
        let mut e = weighing(0.5);
        let player = new_player(&mut e);
        let first = item(&mut e, 0x1000, 2, &[]);
        put_ammo(&mut e, player, &[first]);
        e.call_log = Some(vec![]);
        e.call(0x009691d0, &args![player]);
        assert!(calls(&e, INVENTORY_ADD_ITEM_CHANGE).is_empty());
        assert!(ammo(&e, player).is_empty());
    }

    /// A player whose process has the slots the package starters use, with a
    /// package whose target object is at +0x30.
    fn package_setup() -> (Engine, Ptr<PlayerCharacter>, u32, u32, u32) {
        let mut e = engine();
        let player = new_player(&mut e);
        let process = e.mem.alloc(0x10);
        e.set(player, PlayerCharacter::pProcess, Ptr::new(process));
        let package = e.mem.alloc(0x40);
        let target = e.mem.alloc(0x40);
        e.mem.set_u32(package + 0x30, target);
        double(&mut e, CREATE_PACKAGE, eax(package));
        (e, player, process, package, target)
    }

    #[test]
    fn the_first_package_starter_sets_up_a_type_0x25_package() {
        let (mut e, player, process, package, target) = package_setup();
        let clear_run_once = slot(&mut e, process, 0x210, eax(0));
        let clear_head_track = slot(&mut e, process, 0x644, eax(0));
        let process_28 = slot(&mut e, process, 0x28, eax(0));
        let process_540 = slot(&mut e, process, 0x540, eax(0));
        let player_370 = slot(&mut e, player.addr(), 0x370, eax(0));
        let player_2f4 = slot(&mut e, player.addr(), 0x2f4, eax(0));
        double(&mut e, PROCESS_LISTS_QUEUE_TARGET, eax(0x1234));
        e.call_log = Some(vec![]);
        e.call(
            0x00969260,
            &args![player, 0x5555u32, 0x11u32, 0x33u32, 0x22u32],
        );

        assert_eq!(e.get(player, PlayerCharacter::field_01fc), 0x1234);
        assert_eq!(
            calls(&e, PROCESS_LISTS_QUEUE_TARGET),
            vec![vec![PROCESS_LISTS, player.addr(), 0]]
        );
        assert_eq!(
            calls(&e, PLAYER_SET_MODE_5CC7A0),
            vec![vec![player.addr(), 1]]
        );
        assert_eq!(
            calls(&e, FORCE_TEMP_3RD_PERSON),
            vec![vec![player.addr(), 1]]
        );
        assert_eq!(
            calls(&e, clear_run_once),
            vec![vec![process, 0, player.addr()]]
        );
        assert_eq!(calls(&e, clear_head_track), vec![vec![process, 0]]);
        assert_eq!(calls(&e, CREATE_PACKAGE), vec![vec![0x25]]);
        assert_eq!(calls(&e, PACKAGE_SET_FLAG_2), vec![vec![package, 1]]);
        assert_eq!(calls(&e, PACKAGE_SET_FLAG_4), vec![vec![package, 1]]);
        assert_eq!(
            calls(&e, SET_PACKAGE_WORD_AT_0X18),
            vec![vec![package, 0x1d]]
        );
        assert_eq!(calls(&e, SET_TARG_TYPE), vec![vec![target, 0]]);
        assert_eq!(calls(&e, SET_TARG_REFERENCE), vec![vec![target, 0x5555]]);
        assert_eq!(calls(&e, SET_TARGET_WORD_AT_8), vec![vec![target, 0]]);
        assert_eq!(calls(&e, process_28), vec![vec![process]]);
        assert_eq!(calls(&e, player_370), vec![vec![player.addr(), 1]]);
        assert_eq!(
            calls(&e, process_540),
            vec![vec![process, 0x11, 0x22, 0x33]]
        );
        assert_eq!(
            calls(&e, player_2f4),
            vec![vec![player.addr(), package, 0, 1]]
        );
        assert_eq!(calls(&e, REFERENCE_AFTER_PACKAGE), vec![vec![0x5555]]);
        // The temporary location and target are destroyed after use.
        assert_eq!(calls(&e, PACKAGE_LOCATION_DESTRUCTOR).len(), 1);
        assert_eq!(calls(&e, PACKAGE_TARGET_DESTRUCTOR).len(), 1);
    }

    #[test]
    fn the_second_package_starter_skips_a_reference_that_is_ruled_out() {
        let (mut e, player, process, package, target) = package_setup();
        let player_2f4 = slot(&mut e, player.addr(), 0x2f4, eax(0));
        double(&mut e, REFERENCE_EARLY_OUT_TEST, eax(1));
        e.call_log = Some(vec![]);
        e.call(0x009694d0, &args![player, 0x5555u32]);
        assert!(calls(&e, CREATE_PACKAGE).is_empty());

        double(&mut e, REFERENCE_EARLY_OUT_TEST, eax(0));
        slot(&mut e, process, 0x210, eax(0));
        slot(&mut e, process, 0x644, eax(0));
        let process_28 = slot(&mut e, process, 0x28, eax(0));
        e.call(0x009694d0, &args![player, 0x5555u32]);
        assert_eq!(calls(&e, CREATE_PACKAGE), vec![vec![0x26]]);
        assert_eq!(
            calls(&e, SET_PACKAGE_WORD_AT_0X18),
            vec![vec![package, 0x2f]]
        );
        assert_eq!(calls(&e, SET_TARG_REFERENCE), vec![vec![target, 0x5555]]);
        assert_eq!(calls(&e, process_28), vec![vec![process]]);
        assert_eq!(
            calls(&e, player_2f4),
            vec![vec![player.addr(), package, 0, 1]]
        );
        // No queued target and no closing call, unlike the first starter.
        assert!(calls(&e, PROCESS_LISTS_QUEUE_TARGET).is_empty());
        assert!(calls(&e, REFERENCE_AFTER_PACKAGE).is_empty());
    }

    #[test]
    fn the_current_package_tests_check_the_type() {
        let mut e = engine();
        let player = new_player(&mut e);
        let process = e.mem.alloc(0x10);
        e.set(player, PlayerCharacter::pProcess, Ptr::new(process));
        let package = e.mem.alloc(0x40);
        slot(&mut e, process, 0x22c, eax(package));
        double(&mut e, PLAYER_ANY_FLAG, eax(1));
        e.mem.set_u8(package + 0x20, 0x25);
        assert!(e.call(0x00969700, &args![player]).bool());
        assert!(!e.call(0x00969760, &args![player]).bool());
        e.mem.set_u8(package + 0x20, 0x26);
        assert!(!e.call(0x00969700, &args![player]).bool());
        assert!(e.call(0x00969760, &args![player]).bool());
        // No flag set: false whatever the package.
        double(&mut e, PLAYER_ANY_FLAG, eax(0));
        assert!(!e.call(0x00969760, &args![player]).bool());
        // No package.
        double(&mut e, PLAYER_ANY_FLAG, eax(1));
        slot(&mut e, process, 0x22c, eax(0));
        assert!(!e.call(0x00969760, &args![player]).bool());
    }

    #[test]
    fn the_toddler_setter_acts_only_on_a_change() {
        let mut e = engine();
        let player = new_player(&mut e);
        double(&mut e, PLAYER_GET_ANIMATION, eax(0x77));
        e.call_log = Some(vec![]);
        e.call(0x009697c0, &args![player, 0u32]);
        assert!(calls(&e, PLAYER_PLAY_NAMED).is_empty());
        e.call(0x009697c0, &args![player, 1u32]);
        assert_eq!(e.get(player, PlayerCharacter::bIsToddler), 1);
        assert_eq!(calls(&e, SETTING_STRING), vec![vec![SETTING_TODDLER_TEXT]]);
        assert_eq!(
            calls(&e, PLAYER_GET_ANIMATION),
            vec![vec![player.addr(), 1]]
        );
        let text = setting_slot(SETTING_TODDLER_TEXT);
        assert_eq!(
            calls(&e, PLAYER_PLAY_NAMED),
            vec![vec![player.addr(), 0, 0x77, text, 0]]
        );
        // Unchanged: nothing more.
        e.call(0x009697c0, &args![player, 1u32]);
        assert_eq!(calls(&e, PLAYER_PLAY_NAMED).len(), 1);
    }

    #[test]
    fn the_young_setter_acts_only_on_a_change() {
        let mut e = engine();
        let player = new_player(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00969820, &args![player, 0u32]);
        assert!(calls(&e, ACTOR_REFRESH_8B78C0).is_empty());
        e.call(0x00969820, &args![player, 1u32]);
        assert_eq!(e.get(player, PlayerCharacter::bIsYoung), 1);
        assert_eq!(
            calls(&e, ACTOR_REFRESH_8B78C0),
            vec![vec![player.addr(), 0]]
        );
        assert_eq!(calls(&e, ACTOR_REFRESH_8D3FA0), vec![vec![player.addr()]]);
    }

    #[test]
    fn the_object_at_0x224_needs_a_word_at_0x94() {
        let mut e = engine();
        let player = new_player(&mut e);
        assert!(!e.call(0x00969860, &args![player]).bool());
        let object = e.mem.alloc(0x100);
        e.set(player, PlayerCharacter::field_0224, Ptr::new(object));
        assert!(!e.call(0x00969860, &args![player]).bool());
        e.mem.set_u32(object + 0x94, 5);
        assert!(e.call(0x00969860, &args![player]).bool());
    }

    #[test]
    fn the_marker_lookup_uses_the_cache_or_the_search() {
        let mut e = engine();
        let player = new_player(&mut e);
        // Nothing cached and no markers: 0.
        assert_eq!(e.call(0x009698a0, &args![player]).u32(), 0);

        // A cached marker whose word at +4 is an info object with a word at +0x24.
        let info = e.mem.alloc(0x40);
        e.mem.set_u32(info + 0x24, 0xcafe);
        let marker = e.mem.alloc(0x10);
        e.mem.set_u32(marker + 4, info);
        e.set(
            player,
            PlayerCharacter::pClosestAudioMarkerInfo,
            Ptr::new(marker),
        );
        e.register(AUDIO_MARKER_LOOKUP, |_, a| eax(a[0] + 1));
        assert_eq!(e.call(0x009698a0, &args![player]).u32(), 0xcaff);

        // A cached marker without the info: 0 (the search is not asked).
        e.mem.set_u32(marker + 4, 0);
        assert_eq!(e.call(0x009698a0, &args![player]).u32(), 0);
    }

    #[test]
    fn the_marker_info_lookup_uses_the_word_at_0x24() {
        let mut e = engine();
        let info = e.mem.alloc(0x40);
        e.mem.set_u32(info + 0x24, 0x4321);
        e.register(AUDIO_MARKER_LOOKUP, |_, a| eax(a[0] * 2));
        assert_eq!(e.call(0x00969910, &args![info]).u32(), 0x8642);
    }

    /// The player with an audio marker list of markers whose references have a
    /// position getter in slot 0x1f4.
    fn marker_setup(e: &mut Engine, count: usize) -> (Ptr<PlayerCharacter>, Vec<u32>) {
        let player = new_player(e);
        let position = e.mem.alloc(0x10);
        slot(e, player.addr(), 0x1f4, eax(position));
        e.set_global(NEAREST_MARKER_START, 1.0e30f32);
        let markers: Vec<u32> = (0..count)
            .map(|_| {
                let reference = e.mem.alloc(0x40);
                slot(e, reference, 0x1f4, eax(position));
                let marker = e.mem.alloc(0x10);
                e.mem.set_u32(marker, reference);
                marker
            })
            .collect();
        // The list head is embedded in the player; the others are allocated.
        let head = player.at(PlayerCharacter::AudioMarkerList).addr();
        e.mem.set_u32(head, markers[0]);
        let mut previous = head;
        for marker in &markers[1..] {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *marker);
            e.mem.set_u32(previous + 4, node);
            previous = node;
        }
        (player, markers)
    }

    #[test]
    fn the_closest_marker_is_the_nearest_unless_one_is_inside_its_radius() {
        // Both outside their radius: the nearer one.
        let mut e = engine();
        let (player, markers) = marker_setup(&mut e, 2);
        sequence(&mut e, POINT_LENGTH_SQUARED, &[st0(100.0), st0(25.0)]);
        sequence(&mut e, REFERENCE_RADIUS, &[st0(3.0), st0(3.0)]);
        assert_eq!(e.call(0x00969930, &args![player]).u32(), markers[1]);
        // It is cached.
        assert_eq!(
            e.get(player, PlayerCharacter::pClosestAudioMarkerInfo)
                .addr(),
            markers[1]
        );
        assert_eq!(e.call(0x00969930, &args![player]).u32(), markers[1]);

        // One inside its radius beats a nearer one outside.
        let mut e = engine();
        let (player, markers) = marker_setup(&mut e, 2);
        sequence(&mut e, POINT_LENGTH_SQUARED, &[st0(25.0), st0(100.0)]);
        sequence(&mut e, REFERENCE_RADIUS, &[st0(3.0), st0(20.0)]);
        assert_eq!(e.call(0x00969930, &args![player]).u32(), markers[1]);

        // An empty list has no marker.
        let mut e = engine();
        let player = new_player(&mut e);
        assert_eq!(e.call(0x00969930, &args![player]).u32(), 0);
    }

    #[test]
    fn the_timer_helpers_use_their_globals() {
        let mut e = engine();
        e.call(0x00969ac0, &args![0u32]);
        assert_eq!(e.global::<u8>(TIMER_ACTIVE_FLAG), 1);
        e.call(0x00969ae0, &args![0u32]);
        assert_eq!(e.global::<u8>(TIMER_ACTIVE_FLAG), 0);
        e.set_global(TIMER_START_SECONDS, 99u32);
        e.call(0x00969b00, &args![0u32]);
        assert_eq!(e.global::<u32>(TIMER_START_SECONDS), 0);
        double(&mut e, TIME_IMPORT, eax(123_456));
        e.call(0x00969b20, &args![0u32]);
        assert_eq!(e.global::<u32>(TIMER_START_SECONDS), 123);
    }

    #[test]
    fn the_timer_counts_down_from_its_start() {
        let mut e = engine();
        e.mem.set_i32(setting_slot(SETTING_TIMER_SECONDS), 30);
        e.set_global(TIMER_START_SECONDS, 100u32);
        double(&mut e, TIME_IMPORT, eax(110_000));
        assert_eq!(e.call(0x00969b40, &args![0u32]).i32(), 20);
        double(&mut e, TIME_IMPORT, eax(140_000));
        assert_eq!(e.call(0x00969b40, &args![0u32]).i32(), -10);
    }

    #[test]
    fn the_timer_test_stops_the_timer_when_it_runs_out() {
        let mut e = engine();
        e.mem.set_i32(setting_slot(SETTING_TIMER_SECONDS), 30);
        e.set_global(TIMER_START_SECONDS, 100u32);
        e.set_global(TIMER_ACTIVE_FLAG, 1u8);
        double(&mut e, TIME_IMPORT, eax(110_000));
        assert!(e.call(0x00969b80, &args![0u32]).bool());
        assert_eq!(e.global::<u8>(TIMER_ACTIVE_FLAG), 1);
        // Out of time: false, and the start and the flag are cleared.
        double(&mut e, TIME_IMPORT, eax(130_000));
        assert!(!e.call(0x00969b80, &args![0u32]).bool());
        assert_eq!(e.global::<u8>(TIMER_ACTIVE_FLAG), 0);
        assert_eq!(e.global::<u32>(TIMER_START_SECONDS), 0);
        // Not running: false.
        double(&mut e, TIME_IMPORT, eax(0));
        assert!(!e.call(0x00969b80, &args![0u32]).bool());
    }

    #[test]
    fn a_caravan_card_is_added_once() {
        let mut e = engine();
        let player = new_player(&mut e);
        let inactive = e.mem.alloc(8);
        let active = e.mem.alloc(8);
        e.set(
            player,
            PlayerCharacter::pInactiveListofCaravanCards,
            Ptr::new(inactive),
        );
        e.set(
            player,
            PlayerCharacter::pActiveListofCaravanCards,
            Ptr::new(active),
        );
        let added = Rc::new(Cell::new(0u32));
        let seen = added.clone();
        e.register_double(LIST_ADD, move |e, a| {
            seen.set(e.mem.u32(a[1]));
            eax(0)
        });
        e.call_log = Some(vec![]);
        // In neither list: flagged and added to the inactive one.
        assert!(e.call(0x00969bc0, &args![player, 0xc0deu32]).bool());
        assert_eq!(calls(&e, SET_CARD_FLAG), vec![vec![0xc0de, 0]]);
        assert_eq!(added.get(), 0xc0de);
        assert_eq!(calls(&e, LIST_ADD)[0][0], inactive);
        // Already in the active list: nothing happens.
        double(&mut e, LIST_CONTAINS, eax(1));
        e.call_log = Some(vec![]);
        assert!(e.call(0x00969bc0, &args![player, 0xbeefu32]).bool());
        assert!(calls(&e, LIST_ADD).is_empty());
        assert!(calls(&e, SET_CARD_FLAG).is_empty());
    }

    /// The state the hardcore update reads: time scale 1, divisor 1, limits
    /// 3, 6 and 12 for the actor values 0x49, 0x4b and 0x4a, the calendar at
    /// 100 and the values at the last update 90, 95 and 70.
    fn hardcore_setup() -> (Engine, Ptr<PlayerCharacter>, u32) {
        let mut e = engine();
        let player = new_player(&mut e);
        e.set_global(HARDCORE_DIVISOR, 1.0f64);
        double(&mut e, CALENDAR_GET_TIME_SCALE, st0(1.0));
        double(&mut e, CALENDAR_GET_VALUE, st0(100.0));
        e.mem.set_f32(setting_slot(SETTING_LIMIT_49), 3.0);
        e.mem.set_f32(setting_slot(SETTING_LIMIT_4B), 6.0);
        e.mem.set_f32(setting_slot(SETTING_LIMIT_4A), 12.0);
        e.set_global(LAST_VALUE_49, 90.0f32);
        e.set_global(LAST_VALUE_4B, 95.0f32);
        e.set_global(LAST_VALUE_4A, 70.0f32);
        let modify = slot(&mut e, player.addr(), 0x3b0, eax(0));
        (e, player, modify)
    }

    #[test]
    fn the_hardcore_update_modifies_each_value_whose_limit_has_passed() {
        let (mut e, player, modify) = hardcore_setup();
        e.call_log = Some(vec![]);
        e.call(0x00969c30, &args![player]);
        // 10 / 3 = 3 for 0x49 and 30 / 12 = 2 for 0x4a; 0x4b has 5 < 6.
        assert_eq!(
            calls(&e, modify),
            vec![
                vec![player.addr(), 0x49, 3, 0],
                vec![player.addr(), 0x4a, 2, 0]
            ]
        );
        // The values the limits were met for move to the calendar value.
        assert_eq!(e.global::<f32>(LAST_VALUE_49), 100.0);
        assert_eq!(e.global::<f32>(LAST_VALUE_4A), 100.0);
        assert_eq!(e.global::<f32>(LAST_VALUE_4B), 95.0);
        assert_eq!(e.global::<f32>(NOW_VALUE_49), 100.0);
        assert!(calls(&e, PLAYER_RESET_NEEDS).is_empty());
    }

    #[test]
    fn the_hardcore_update_does_nothing_in_the_excluded_state() {
        let (mut e, player, modify) = hardcore_setup();
        double(&mut e, PLAYER_IS_IN_STATE, eax(1));
        e.call_log = Some(vec![]);
        e.call(0x00969c30, &args![player]);
        assert!(calls(&e, modify).is_empty());
        assert_eq!(calls(&e, PLAYER_IS_IN_STATE), vec![vec![player.addr(), 4]]);
        assert_eq!(e.global::<f32>(LAST_VALUE_49), 90.0);
    }

    #[test]
    fn the_hardcore_update_skips_the_0x4b_value_while_its_test_holds() {
        let (mut e, player, modify) = hardcore_setup();
        // 0x4b: 100 - 80 = 20 >= 6, so it would be modified.
        e.set_global(LAST_VALUE_4B, 80.0f32);
        e.call_log = Some(vec![]);
        e.call(0x00969c30, &args![player]);
        assert_eq!(calls(&e, modify).len(), 3);
        assert!(calls(&e, modify).contains(&vec![player.addr(), 0x4b, 3, 0]));

        let (mut e, player, modify) = hardcore_setup();
        e.set_global(LAST_VALUE_4B, 80.0f32);
        double(&mut e, PLAYER_NEEDS_TEST, eax(1));
        e.call_log = Some(vec![]);
        e.call(0x00969c30, &args![player]);
        assert_eq!(calls(&e, modify).len(), 2);
        // The saved value still moves on.
        assert_eq!(e.global::<f32>(LAST_VALUE_4B), 100.0);
    }

    #[test]
    fn the_hardcore_update_resets_the_saved_values_when_asked() {
        let (mut e, player, modify) = hardcore_setup();
        e.set(player, PlayerCharacter::bResetHardcoreTimers, 1);
        e.call_log = Some(vec![]);
        e.call(0x00969c30, &args![player]);
        assert_eq!(calls(&e, PLAYER_RESET_NEEDS), vec![vec![player.addr(), 0]]);
        // Everything now equals the calendar value: no value is modified.
        assert!(calls(&e, modify).is_empty());
        assert_eq!(e.global::<f32>(LAST_VALUE_4B), 100.0);

        // A zero saved first value resets as well.
        let (mut e, player, modify) = hardcore_setup();
        e.set_global(LAST_VALUE_49, 0.0f32);
        e.call_log = Some(vec![]);
        e.call(0x00969c30, &args![player]);
        assert_eq!(calls(&e, PLAYER_RESET_NEEDS).len(), 1);
        assert!(calls(&e, modify).is_empty());
    }

    #[test]
    fn the_reset_flag_is_the_byte_at_0xe39() {
        let mut e = engine();
        let player = new_player(&mut e);
        assert_eq!(e.call(0x00969e70, &args![player]).u8(), 0);
        e.mem.set_u8(player.addr() + 0xe39, 1);
        assert_eq!(e.call(0x00969e70, &args![player]).u8(), 1);
    }

    #[test]
    fn setting_hardcore_off_gives_back_the_three_values() {
        let mut e = engine();
        let player = new_player(&mut e);
        e.set(player, PlayerCharacter::eHardcoreSetting, 3);
        let list = e.mem.alloc(0x20);
        let extra = e.mem.alloc(0x20);
        double(&mut e, EXTRA_LIST_OF_REFERENCE, eax(list));
        double(&mut e, EXTRA_GET_TYPE_0X15, eax(extra));
        let current = slot(&mut e, player.addr() + 0xa4, 0xc, st0(5.0));
        let modify = slot(&mut e, player.addr(), 0x3a4, eax(0));
        e.call_log = Some(vec![]);
        e.call(0x00969e90, &args![player, 0u32, 0u32]);
        assert_eq!(calls(&e, EXTRA_GET_TYPE_0X15), vec![vec![list]]);
        assert_eq!(calls(&e, RESET_TYPE_0X15), vec![vec![extra]]);
        assert_eq!(calls(&e, CLEAR_BYTE_AT_0XE38), vec![vec![player.addr()]]);
        let sub_object = player.addr() + 0xa4;
        assert_eq!(
            calls(&e, current),
            vec![
                vec![sub_object, 0x49],
                vec![sub_object, 0x4b],
                vec![sub_object, 0x4a]
            ]
        );
        let minus_five = (-5.0f32).to_bits();
        assert_eq!(
            calls(&e, modify),
            vec![
                vec![player.addr(), 0x49, minus_five, 0],
                vec![player.addr(), 0x4b, minus_five, 0],
                vec![player.addr(), 0x4a, minus_five, 0]
            ]
        );
        assert_eq!(e.get(player, PlayerCharacter::eHardcoreSetting), 0);
    }

    #[test]
    fn setting_a_hardcore_mode_only_stores_it() {
        let mut e = engine();
        let player = new_player(&mut e);
        let modify = slot(&mut e, player.addr(), 0x3a4, eax(0));
        e.call_log = Some(vec![]);
        e.call(0x00969e90, &args![player, 2u32, 0u32]);
        assert_eq!(e.get(player, PlayerCharacter::eHardcoreSetting), 2);
        assert!(calls(&e, modify).is_empty());
        assert!(calls(&e, RESET_TYPE_0X15).is_empty());
        // Mode 0 with the keep flag, or with the global flag set: no change of values.
        e.call(0x00969e90, &args![player, 0u32, 1u32]);
        double(&mut e, GLOBAL_FLAG_7D6E60, eax(1));
        e.call(0x00969e90, &args![player, 0u32, 0u32]);
        assert!(calls(&e, modify).is_empty());
        assert_eq!(calls(&e, CLEAR_BYTE_AT_0XE38).len(), 1);
    }

    /// The reasons `00969fa0` tests, each with the setup that makes it hold
    /// and the message it shows.
    type Reason = (&'static str, fn(&mut Engine, Ptr<PlayerCharacter>), u32);

    fn sleep_reasons() -> [Reason; 11] {
        [
            (
                "slot 0x448",
                |e, p| {
                    slot(e, p.addr(), 0x448, eax(1));
                },
                MESSAGE_FIRST,
            ),
            (
                "953c80",
                |e, _| double(e, SLEEP_TEST_953C80, eax(1)),
                MESSAGE_SECOND,
            ),
            (
                "885520",
                |e, _| double(e, SLEEP_TEST_885520, eax(1)),
                MESSAGE_THIRD,
            ),
            (
                "9764a0",
                |e, _| double(e, PROCESS_LISTS_TEST_9764A0, eax(1)),
                MESSAGE_FOURTH,
            ),
            (
                "controller state 1",
                |e, _| {
                    double(e, GET_CHAR_CONTROLLER, eax(0x77));
                    double(e, CONTROLLER_STATE, eax(1));
                },
                MESSAGE_FIFTH,
            ),
            (
                "controller state 2",
                |e, _| {
                    double(e, GET_CHAR_CONTROLLER, eax(0x77));
                    double(e, CONTROLLER_STATE, eax(2));
                },
                MESSAGE_FIFTH,
            ),
            (
                "5444c0",
                |e, _| double(e, CELL_TEST_5444C0, eax(1)),
                MESSAGE_SIXTH,
            ),
            (
                "944360",
                |e, _| double(e, SLEEP_TEST_944360, eax(1)),
                MESSAGE_SIXTH,
            ),
            (
                "5098e0",
                |e, _| double(e, FURNITURE_TEST_5098E0, eax(1)),
                MESSAGE_SEVENTH,
            ),
            (
                "water delta",
                |e, p| {
                    let process = e.mem.alloc(0x10);
                    e.mem.set_u32(p.addr() + 0x68, process);
                    slot(e, process, 0x764, st0(0.25));
                },
                MESSAGE_SEVENTH,
            ),
            (
                "822e00",
                |e, _| double(e, MAGIC_TEST_822E00, eax(1)),
                MESSAGE_EIGHTH,
            ),
        ]
    }

    #[test]
    fn the_sleep_wait_request_shows_the_first_reason_it_cannot_happen() {
        for (name, set_up, message) in sleep_reasons() {
            let mut e = engine();
            let player = new_player(&mut e);
            e.set_global(MESSAGE_TIME, 2.5f32);
            let position = e.mem.alloc(0x10);
            slot(&mut e, player.addr(), 0x448, eax(0));
            slot(&mut e, player.addr(), 0x1f4, eax(position));
            set_up(&mut e, player);
            e.call_log = Some(vec![]);
            e.call(0x00969fa0, &args![player]);
            assert_eq!(
                calls(&e, SHOW_MESSAGE),
                vec![vec![
                    setting_slot(message),
                    0,
                    MESSAGE_ICON,
                    0,
                    2.5f32.to_bits(),
                    0
                ]],
                "{name}"
            );
            assert!(calls(&e, CREATE_SLEEP_MENU).is_empty(), "{name}");
        }
    }

    #[test]
    fn the_sleep_wait_request_opens_the_menu_when_nothing_rules_it_out() {
        let mut e = engine();
        let player = new_player(&mut e);
        // A process whose water delta is zero does not rule it out.
        let process = e.mem.alloc(0x10);
        e.mem.set_u32(player.addr() + 0x68, process);
        slot(&mut e, process, 0x764, st0(0.0));
        e.set_global(SLEEP_TEST_DISTANCE, 3.0f32);
        let place = 0x4444;
        slot(&mut e, player.addr(), 0x448, eax(0));
        double(&mut e, ACTOR_PROCESS_OF_THIS, eax(place));
        let position = e.mem.alloc(0x10);
        slot(&mut e, player.addr(), 0x1f4, eax(position));
        e.call_log = Some(vec![]);
        e.call(0x00969fa0, &args![player]);
        assert!(calls(&e, SHOW_MESSAGE).is_empty());
        assert_eq!(calls(&e, CREATE_SLEEP_MENU), vec![vec![1]]);
        assert_eq!(
            calls(&e, SLEEP_TEST_885520),
            vec![vec![player.addr(), position, place, 3.0f32.to_bits()]]
        );
    }

    #[test]
    fn the_array_constructor_runs_the_initialiser_and_installs_its_vtable() {
        let mut e = engine();
        let block = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0096a280, &args![block, 5u32]).u32(), block);
        assert_eq!(calls(&e, ALLOCATE_BYTES), vec![vec![20]]);
        assert_eq!(e.mem.u32(block), VTABLE_0096A280);
    }

    #[test]
    fn the_element_of_the_0x68_byte_array_comes_from_its_helper() {
        let mut e = engine();
        let array = e.mem.alloc(0x20);
        let buffer = e.mem.alloc(0x68 * 3);
        e.mem.set_u32(array + 4, buffer);
        assert_eq!(e.call(0x0096a2b0, &args![array, 2u32]).u32(), buffer + 0xd0);
    }

    #[test]
    fn the_two_word_constructor_clears_both_words() {
        let mut e = engine();
        let block = e.mem.alloc(8);
        e.mem.set_u32(block, 7);
        e.mem.set_u32(block + 4, 9);
        assert_eq!(e.call(0x0096a2d0, &args![block]).u32(), block);
        assert_eq!((e.mem.u32(block), e.mem.u32(block + 4)), (0, 0));
    }

    #[test]
    fn the_deleting_destructors_free_only_when_bit_zero_is_set() {
        for (addr, empty) in [
            (0x0096a300, HASH_TABLE_EMPTY),
            (0x0096a670, HASH_TABLE_EMPTY),
            (0x0096a6a0, ARRAY_CLEAR),
            (0x0096a6d0, ARRAY_CLEAR),
        ] {
            let mut e = engine();
            let block = e.mem.alloc(0x20);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(addr, &args![block, 0u32]).u32(), block);
            assert!(calls(&e, FREE_OBJECT).is_empty());
            assert!(!calls(&e, empty).is_empty());
            assert_eq!(e.call(addr, &args![block, 1u32]).u32(), block);
            assert_eq!(calls(&e, FREE_OBJECT), vec![vec![block]]);
        }
    }

    #[test]
    fn the_small_array_accessors_scale_by_the_element_size() {
        let mut e = engine();
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, 0x5000);
        e.mem.set_u32(array + 4, 0x6000);
        assert_eq!(e.call(0x0096a330, &args![array, 2u32]).u32(), 0x6000 + 0xd0);
        assert_eq!(e.call(0x0096a370, &args![array, 2u32]).u32(), 0x5000 + 0x60);
        e.mem.set_u32(array + 4, 0xffff_8001);
        assert_eq!(e.call(0x0096a350, &args![array]).u32(), 0x8001);
    }

    #[test]
    fn the_hash_table_initialiser_allocates_and_clears_the_buckets() {
        let mut e = engine();
        let block = e.mem.alloc(0x10);
        double(&mut e, ALLOCATE_BYTES, eax(0x7000));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0096a390, &args![block, 8u32]).u32(), block);
        assert_eq!(e.mem.u32(block), HASH_TABLE_VTABLE);
        assert_eq!(e.mem.u32(block + 4), 8);
        assert_eq!(e.mem.u32(block + 8), 0x7000);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(calls(&e, ALLOCATE_BYTES), vec![vec![32]]);
        assert_eq!(calls(&e, FILL_MEMORY), vec![vec![0x7000, 0, 32]]);
    }

    /// A table of four buckets with a chain of two items in bucket 1 and one
    /// item in bucket 3; the bucket function answers 3 for key 33, otherwise 1.
    fn iterator_table(e: &mut Engine) -> (u32, [u32; 3]) {
        let table = e.mem.alloc(0x10);
        let buckets = e.mem.alloc(16);
        let second = e.mem.alloc(12);
        let first = e.mem.alloc(12);
        let third = e.mem.alloc(12);
        e.mem.set_u32(table + 4, 4);
        e.mem.set_u32(table + 8, buckets);
        e.mem.set_u32(buckets + 4, first);
        e.mem.set_u32(buckets + 12, third);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 11);
        e.mem.set_u8(first + 8, 1);
        e.mem.set_u32(second + 4, 22);
        e.mem.set_u8(second + 8, 2);
        e.mem.set_u32(third + 4, 33);
        e.mem.set_u8(third + 8, 3);
        let bucket_of = slot(e, table, 4, eax(1));
        e.register(bucket_of, |_, a| eax(if a[1] == 33 { 3 } else { 1 }));
        (table, [first, second, third])
    }

    #[test]
    fn the_hash_iterator_follows_the_chain_then_the_next_bucket_then_ends() {
        let mut e = engine();
        let (table, [first, second, third]) = iterator_table(&mut e);
        let out = e.mem.alloc(12);
        e.mem.set_u32(out, first);
        e.call(0x0096a400, &args![table, out, out + 4, out + 8]);
        assert_eq!(e.mem.u32(out), second);
        assert_eq!((e.mem.u32(out + 4), e.mem.u8(out + 8)), (11, 1));
        e.call(0x0096a400, &args![table, out, out + 4, out + 8]);
        assert_eq!(e.mem.u32(out), third);
        assert_eq!((e.mem.u32(out + 4), e.mem.u8(out + 8)), (22, 2));
        e.call(0x0096a400, &args![table, out, out + 4, out + 8]);
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!((e.mem.u32(out + 4), e.mem.u8(out + 8)), (33, 3));
    }

    #[test]
    fn the_map_destructors_empty_the_table_and_free_the_buckets() {
        let mut e = engine();
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block + 8, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x0096a4b0, &args![block]);
        assert_eq!(e.mem.u32(block), HASH_TABLE_VTABLE);
        assert_eq!(calls(&e, HASH_TABLE_EMPTY), vec![vec![block], vec![block]]);
        assert_eq!(calls(&e, FREE_BYTES), vec![vec![0x7000]]);
        e.mem.set_u32(block, 0);
        e.call_log = Some(vec![]);
        e.call(0x0096a510, &args![block]);
        assert_eq!(e.mem.u32(block), HASH_TABLE_VTABLE);
        assert_eq!(calls(&e, HASH_TABLE_EMPTY), vec![vec![block]]);
        assert_eq!(calls(&e, FREE_BYTES), vec![vec![0x7000]]);
    }

    #[test]
    fn the_sub_object_release_clears_the_item_byte_and_passes_the_item() {
        let mut e = engine();
        let block = e.mem.alloc(0x20);
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(block + 8, 0x30b9_dad0);
        e.mem.set_u8(item + 8, 1);
        e.call_log = Some(vec![]);
        e.call(0x0096a540, &args![block, item]);
        assert_eq!(e.mem.u8(item + 8), 0);
        assert_eq!(
            e.mem.u32(block + 8),
            0x30b9_dad0,
            "the map's own word is left"
        );
        assert_eq!(calls(&e, SUB_OBJECT_RELEASE), vec![vec![block + 0xc, item]]);
    }

    #[test]
    fn the_item_and_ammo_arrays_install_their_vtables() {
        let mut e = engine();
        let block = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0096a570, &args![block]).u32(), block);
        assert_eq!(e.mem.u32(block), ITEM_CHANGE_ARRAY_VTABLE);
        assert_eq!(calls(&e, ARRAY_INITIALISE_SIZED), vec![vec![block, 0, 0]]);
        e.call(0x0096a5c0, &args![block]);
        assert_eq!(e.mem.u32(block), AMMO_ARRAY_VTABLE);
        e.call_log = Some(vec![]);
        e.call(0x0096a5a0, &args![block]);
        assert_eq!(e.mem.u32(block), ITEM_CHANGE_ARRAY_VTABLE);
        e.call(0x0096a5f0, &args![block]);
        assert_eq!(e.mem.u32(block), AMMO_ARRAY_VTABLE);
        assert_eq!(calls(&e, ARRAY_CLEAR), vec![vec![block, 1], vec![block, 1]]);
    }

    #[test]
    fn the_set_at_grow_wrapper_uses_the_stored_index() {
        let mut e = engine();
        let block = e.mem.alloc(0x20);
        e.mem.set_u16(block + 0xa, 5);
        e.call_log = Some(vec![]);
        e.call(0x0096a610, &args![block, 0x99u32]);
        assert_eq!(calls(&e, ARRAY_SET_AT_GROW), vec![vec![block, 5, 0x99]]);
    }

    #[test]
    fn the_capacity_array_allocates_only_for_a_positive_capacity() {
        let mut e = engine();
        let block = e.mem.alloc(0x20);
        double(&mut e, ALLOCATE_WORDS, eax(0x7100));
        e.mem.set_u16(block + 0xa, 3);
        e.mem.set_u16(block + 0xc, 3);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0096a7b0, &args![block, 6u32, 2u32]).u32(), block);
        assert_eq!(e.mem.u32(block), CAPACITY_ARRAY_VTABLE);
        assert_eq!(e.mem.u16(block + 8), 6);
        assert_eq!(e.mem.u16(block + 0xe), 2);
        assert_eq!((e.mem.u16(block + 0xa), e.mem.u16(block + 0xc)), (0, 0));
        assert_eq!(e.mem.u32(block + 4), 0x7100);
        assert_eq!(calls(&e, ALLOCATE_WORDS), vec![vec![6]]);
        e.call_log = Some(vec![]);
        e.call(0x0096a7b0, &args![block, 0u32, 2u32]);
        assert_eq!(e.mem.u32(block + 4), 0);
        assert!(calls(&e, ALLOCATE_WORDS).is_empty());
    }

    #[test]
    fn the_derived_constructor_runs_the_base_then_installs_its_vtable() {
        let mut e = engine();
        let block = e.mem.alloc(0x20);
        double(&mut e, ALLOCATE_WORDS, eax(0x7100));
        assert_eq!(e.call(0x0096a640, &args![block, 4u32, 1u32]).u32(), block);
        assert_eq!(e.mem.u32(block), TABLE_ARRAY_DERIVED_VTABLE);
        assert_eq!(e.mem.u16(block + 8), 4);
        assert_eq!(e.mem.u16(block + 0xe), 1);
        assert_eq!(e.mem.u32(block + 4), 0x7100);
    }

    // ---- the exe's stack frames ----

    /// The local the next `with_stack(size)` on this thread takes (asked of the engine), with
    /// the call log switched on.
    fn next_frame(e: &mut Engine, size: u32) -> u32 {
        e.call_log = Some(vec![]);
        e.next_stack_local(size)
    }

    #[test]
    fn the_movement_frames_keep_their_locals_at_the_exe_offsets() {
        // 00967da0 (0xe8 bytes): the verdict word at +0x88, the position copy at +0x94, the
        // difference at +0x5c.
        let mut e = engine();
        let player = new_player(&mut e);
        let actor = e.mem.alloc(0x40);
        let position = e.mem.alloc(0x10);
        slot(&mut e, actor, 0x1f4, eax(position));
        let other = group(&mut e, &[actor], &[]);
        manager(&mut e, &[other]);
        let entry = e.mem.alloc(0x20);
        e.mem.set_u32(entry + 8, actor);
        let entries = list(&mut e, &[entry]);
        double(&mut e, PROCESS_LISTS_ENTRIES, eax(entries));
        double(&mut e, GROUP_IS_TARGET, eax(1));
        e.register(LIST_POP_FRONT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            eax(0)
        });
        slot(&mut e, player.addr(), 0x1f4, eax(position));
        let frame = next_frame(&mut e, 0xe8);
        e.call(0x00967da0, &args![player]);
        assert_eq!(
            calls(&e, GET_SHOULD_ATTACK_ACTOR),
            vec![vec![actor, player.addr(), 0, frame + 0x88, 1]]
        );
        assert_eq!(
            calls(&e, POINT_SUBTRACT),
            vec![vec![frame + 0x94, frame + 0x5c, position]]
        );
        assert_eq!(calls(&e, POINT_LENGTH), vec![vec![frame + 0x5c]]);

        // 00968690 (0x28 bytes): the velocity at +0x10, the scaled velocity at +4, the sum at
        // +0x1c.
        let mut e = engine();
        let reference = e.mem.alloc(0x40);
        let position = e.mem.alloc(0x10);
        double(&mut e, POSITION_OF, eax(position));
        double(&mut e, GET_CHAR_CONTROLLER, eax(0x77));
        e.register(VECTOR_SCALE, |_, a| eax(a[1]));
        let result = e.mem.alloc(0x10);
        let frame = next_frame(&mut e, 0x28);
        e.call(0x00968690, &args![reference, result, 2.0f32]);
        assert_eq!(
            calls(&e, CONTROLLER_LINEAR_VELOCITY),
            vec![vec![0x77, frame + 0x10]]
        );
        assert_eq!(
            calls(&e, VECTOR_SCALE),
            vec![vec![frame + 0x10, frame + 4, 2.0f32.to_bits()]]
        );
        assert_eq!(calls(&e, VECTOR_ADD), vec![vec![frame + 0x1c, frame + 4]]);

        // 00969930 (0x40 bytes a marker): the position copy at +0x1c, the difference at +0x10.
        let mut e = engine();
        let (player, _) = marker_setup(&mut e, 1);
        let mine = e.vcall(player.addr(), 0x1f4, &args![]).u32();
        let frame = next_frame(&mut e, 0x40);
        e.call(0x00969930, &args![player]);
        assert_eq!(
            calls(&e, POINT_SUBTRACT),
            vec![vec![frame + 0x1c, frame + 0x10, mine]]
        );
        assert_eq!(calls(&e, POINT_LENGTH_SQUARED), vec![vec![frame + 0x10]]);
    }
}
