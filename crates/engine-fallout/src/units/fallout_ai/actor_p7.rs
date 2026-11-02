//! `fallout/ai/actor.cpp` (Xbox PDB source unit), part 7: its functions from `008bec80` up to
//! (not including) `ffffffff` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::actor`]; anything public there may be used here.
//!
//! This session covers `008bec80` to `008c1c80` (40 functions): the cached
//! value getter, the alarm package set-up (`008bed50`, `008bf500`, `008bf7b0`),
//! the crime reporting functions (`StealAlarm`, `AttackAlarm`, `008c00e0`,
//! `008c09e0`), the small `Actor`/`PlayerCharacter` field accessors and the
//! `NiPointer<HitData>` / `BSSimpleArray<Actor *, 1024>` helpers.
//!
//! Notes for the next session: this part continues at `008cfad0`
//! (`Actor::FakeWeaponHitSound`), then `008e2680` (`Actor::DoesFly`).
//!
//! Conventions: the `ActorValueOwner` sits at `Actor + 0xa4` and the
//! `CachedValuesOwner` at `Actor + 0xa8`. `008d8520` (named
//! `MiddleHighProcess::GetSavedAcquireObject` in the map: the linker folded it)
//! returns the actor's process, the word at `+0x68`. The decompiler hangs
//! pushed words on the wrong call in this unit, so every call was read from
//! the disassembly. C++ exception unwinding (the `__CxxFrameHandler` states
//! of the functions that allocate with `operator new`) is not translated.
//! Callee names that the exe map does not give are described by what the
//! body does; slot numbers of vtables are byte offsets.

#[allow(unused_imports)]
use super::actor::*;
#[allow(unused_imports)]
use crate::prelude::*;

/// `PlayerCharacter *`.
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// `ProcessLists *` (the singleton object itself).
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// The calendar singleton.
const CALENDAR: u32 = 0x011d_e7b8;
/// The lock `0040fbf0` / `0040fba0` take around the alarm broadcast.
const BROADCAST_LOCK: u32 = 0x011f_1180;

/// The actor's process (`008d8520`: the word at `+0x68`).
const ACTOR_PROCESS: u32 = 0x008d_8520;
/// `TESObjectREFR`'s base form getter (`007af430`, `ECX` = the reference).
const GET_FORM: u32 = 0x007a_f430;
/// The base form of an actor reference (`004181e0`).
const GET_BASE_FORM: u32 = 0x0041_81e0;
/// The form type byte at `+4` of a form (`00401170`).
const FORM_TYPE: u32 = 0x0040_1170;
/// `__RTDynamicCast(object, 0, source type, target type, 0)`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The type descriptors of the cast of a reference to `Actor`.
const CAST_FROM_REFERENCE: u32 = 0x0118_41cc;
const CAST_TO_ACTOR: u32 = 0x0118_46d4;
/// The other cast targets used in this part.
const CAST_TO_NPC: u32 = 0x0118_3a1c;
const CAST_FROM_FORM: u32 = 0x0118_3108;
const CAST_FROM_PACKAGE: u32 = 0x0118_46a0;
const CAST_TO_FLEE_PACKAGE: u32 = 0x0118_c6f4;
const CAST_TO_PROCESS_TYPE: u32 = 0x011a_3328;
/// `operator new(size)` (cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete(block)` (cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The reference's `ExtraDataList` (`005d43c0`).
const EXTRA_LIST_OF_REFERENCE: u32 = 0x005d_43c0;
/// `ExtraDataList::AddToPlayerCrimeList` (Xbox PDB): `ECX` = the list, the
/// crime and the hour (`float`).
const ADD_TO_PLAYER_CRIME_LIST: u32 = 0x0041_ce50;
/// `ExtraDataList::GetPlayerCrimeList` (Xbox PDB): `ECX` = the list.
const GET_PLAYER_CRIME_LIST: u32 = 0x0041_cf00;
/// The package type byte at `+0x20` of a package (`0041ca90`, sign-extended).
const PACKAGE_TYPE: u32 = 0x0041_ca90;
/// `ExtraDataList::GetPackageExtra` (Xbox PDB).
const GET_PACKAGE_EXTRA: u32 = 0x0041_cb10;
/// The extra data of a list that holds the faction changes (`0042e800`).
const GET_FACTION_CHANGES_EXTRA: u32 = 0x0042_e800;
/// The faction list of a base form, given `base + 0x30` (`005d8a70`).
const FACTION_LIST_OF: u32 = 0x005d_8a70;
/// A game setting's value address (`00403e20`, `ECX` = the setting): the
/// `float` at `+4`.
const SETTING_FLOAT_POINTER: u32 = 0x0040_3e20;
/// A game setting's value address (`0043d4d0`, `ECX` = the setting): the
/// integer at `+4`.
const SETTING_INT_POINTER: u32 = 0x0043_d4d0;
/// `_ftol2_sse` (`00ec62c0`): the value comes as a leading `f64`.
const FLOAT_TO_INT: u32 = 0x00ec_62c0;
/// `_sprintf` (`cdecl`).
const SPRINTF: u32 = 0x00ec_623a;
/// Takes the lock in `ECX` (second word unused).
const LOCK_TAKE: u32 = 0x0040_fbf0;
/// Releases the lock in `ECX`.
const LOCK_RELEASE: u32 = 0x0040_fba0;
/// The bool byte at `+0x104` of an actor, `bInCombat` (`00493bb0`).
const IN_COMBAT_FLAG: u32 = 0x0049_3bb0;
/// `Actor::IsFleeing` (Xbox PDB): one flag word.
const IS_FLEEING: u32 = 0x008a_6650;
/// `Actor::IsInCombatantFaction` (Xbox PDB).
const IS_IN_COMBATANT_FACTION: u32 = 0x008a_c6f0;
/// `Actor::IsInFaction` (Xbox PDB): `ECX` = the actor, the faction.
const IS_IN_FACTION: u32 = 0x008b_8e90;
/// `Actor::IntegrateFactionLists` (Xbox PDB): `ECX` = the actor, a buffer, its
/// capacity, the base faction list and the changes word.
const INTEGRATE_FACTION_LISTS: u32 = 0x008b_8ca0;
/// `Actor::AddFactionMinorCrime` (Xbox PDB): `ECX` = the actor, two words.
const ADD_FACTION_MINOR_CRIME: u32 = 0x008b_7c00;
/// `Actor::AddFactionMajorCrime` (Xbox PDB): `ECX` = the actor, two words.
const ADD_FACTION_MAJOR_CRIME: u32 = 0x008b_7d20;
/// `Actor::SetFactionsThatCareAboutCrime` (Xbox PDB): `ECX` = the witness, the
/// criminal and the crime.
const SET_FACTIONS_THAT_CARE_ABOUT_CRIME: u32 = 0x008b_8360;
/// `Actor::GetShouldAttackActor` (Xbox PDB): `ECX` = the witness, the target, a
/// word, the address of an out word and a word.
const GET_SHOULD_ATTACK_ACTOR: u32 = 0x008b_06d0;
/// `Actor::GetShouldHelp` (Xbox PDB): `ECX` = the witness, the criminal.
const GET_SHOULD_HELP: u32 = 0x008b_0970;
/// `Actor::GetDetectionLevelAgainstActor` (Xbox PDB): `ECX` = the observer,
/// seven words.
const GET_DETECTION_LEVEL: u32 = 0x008a_0d10;
/// `Actor::GetClass` (Xbox PDB).
const ACTOR_GET_CLASS: u32 = 0x0088_4350;
/// `TESClass::IsGuard` (Xbox PDB).
const CLASS_IS_GUARD: u32 = 0x005f_6e60;
/// `Actor::GetPackageSetAsPcurrent` (Xbox PDB).
const GET_PACKAGE_SET_AS_CURRENT: u32 = 0x0088_1510;
/// `Actor::EndInterruptPackage` (Xbox PDB): `ECX` = the actor, one word.
const END_INTERRUPT_PACKAGE: u32 = 0x0088_1680;
/// `Actor::IsRunningRunOnce` (Xbox PDB).
const IS_RUNNING_RUN_ONCE: u32 = 0x0088_1570;
/// `Actor::ClearDispositionModifiers` (Xbox PDB).
const CLEAR_DISPOSITION_MODIFIERS: u32 = 0x0087_fd20;
/// `MobileObject::GetCurrentProcessType` (Xbox PDB).
const GET_CURRENT_PROCESS_TYPE: u32 = 0x0093_1850;
/// `MobileObject::GetCurrentPackage` (Xbox PDB).
const GET_CURRENT_PACKAGE: u32 = 0x0093_44a0;
/// A test on the flag word `008846e0` of an actor (`004997b0`): bit `0x400`
/// set and bit `0x800` clear.
const ACTOR_FLAG_TEST: u32 = 0x0049_97b0;
/// `ActorValueOwner::GetClampedActorValue` (Xbox PDB): `ECX` = the owner, the
/// actor value index.
const GET_CLAMPED_ACTOR_VALUE: u32 = 0x0066_ef20;
/// `MiddleHighProcess::GetForceNextUpdate` (Xbox PDB): the byte at `+0x18d`.
const GET_FORCE_NEXT_UPDATE: u32 = 0x0056_6950;
/// `TESObjectREFR::GetOwner` (Xbox PDB).
const GET_OWNER: u32 = 0x0056_7790;
/// `TESObjectREFR::IsAnOwner` (Xbox PDB): `ECX` = the owner, a reference and a
/// flag word.
const IS_AN_OWNER: u32 = 0x0057_85e0;
/// `TESObjectREFR::GetRefPersists` (Xbox PDB).
const GET_REF_PERSISTS: u32 = 0x0056_53d0;
/// The name of a reference (`0055d520`), used for the log line.
const REFERENCE_NAME: u32 = 0x0055_d520;
/// Stores a word at `+0x70` of a reference (`0057bd60`).
const SET_WORD_AT_0X70: u32 = 0x0057_bd60;
/// `TESTopic::GetTopic` (Xbox PDB), `cdecl`: two words.
const GET_TOPIC: u32 = 0x0061_a2d0;
/// `TESWeightForm::GetFormWeight` (Xbox PDB), `cdecl`: the form and a flag.
const GET_FORM_WEIGHT: u32 = 0x0048_ebc0;
/// The flag byte `GetFormWeight` is given (`004d1360`, on the player).
const WEIGHT_FLAG_OF_PLAYER: u32 = 0x004d_1360;
/// Bit 1 of the flag word at `+0x34` of a base form (`0047d7c0`).
const BASE_FORM_FLAG_BIT_1: u32 = 0x0047_d7c0;
/// `TESActorBaseData::GetAlignmentForKarma` (Xbox PDB), `cdecl`: a `float`.
const GET_ALIGNMENT_FOR_KARMA: u32 = 0x0047_e040;
/// `PlayerCharacter::RewardKarma` (Xbox PDB): `ECX` = the player, an integer.
const REWARD_KARMA: u32 = 0x0094_fd30;
/// `PlayerCharacter::GetPlayerAction` (Xbox PDB): `ECX` = the player, a word.
const GET_PLAYER_ACTION: u32 = 0x0096_4100;
/// The byte at `+0x7c4` of the player (`008a08c0`).
const PLAYER_BYTE_AT_0X7C4: u32 = 0x008a_08c0;
/// The word at `+0x230` of the player (`005224e0`).
const PLAYER_WORD_AT_0X230: u32 = 0x0052_24e0;
/// `PlayerCharacter::GetAnimation` (Xbox PDB): `ECX` = the player, a word.
const PLAYER_GET_ANIMATION: u32 = 0x0095_0a60;
/// The player's counter bump for the alarm package of type `0x23`
/// (`00966e50`) and of type `0x22` (`00966e00`).
const PLAYER_ALARM_COUNTER_23: u32 = 0x0096_6e50;
const PLAYER_ALARM_COUNTER_22: u32 = 0x0096_6e00;
/// `BGSEntryPoint::HandleEntryPoint` (Xbox PDB), `cdecl`: the entry point, the
/// actor, a perk argument and the address of the value.
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// The word at `+8` of an object (`0044ddc0`).
const WORD_AT_8: u32 = 0x0044_ddc0;
/// The word at `+0xc` of an object (`0084e3a0`): the reference of an alarm
/// record or crime.
const RECORD_REFERENCE: u32 = 0x0084_e3a0;
/// The byte at `+0x10` of an alarm record (`00833c20`).
const RECORD_BYTE_AT_0X10: u32 = 0x0083_3c20;
/// A list node's own data slot (`006815c0`): the item is the word at it.
const LIST_NODE_ITEM_SLOT: u32 = 0x0068_15c0;
/// A list node's next node (`00726070`).
const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// Empties a list (`00470470`).
const LIST_CLEAR: u32 = 0x0047_0470;
/// Destroys a list (`004702f0`, flag word: free the block).
const LIST_DESTROY: u32 = 0x0047_02f0;
/// A list head test (`008256d0`): both words zero.
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `ProcessLists::GetActorRefInHigh` (Xbox PDB).
const PROCESS_LISTS_GET_ACTOR_REF_IN_HIGH: u32 = 0x0097_0a20;
/// Builds the list of actors that witness a crime (`00970f60`).
const PROCESS_LISTS_WITNESSES: u32 = 0x0097_0f60;
/// `ProcessLists` actor lookup (`00970b30`): a reference, a flag, a word.
const PROCESS_LISTS_FIND_ACTOR_REF: u32 = 0x0097_0b30;
/// `ProcessLists` lookup of the process entry for a reference and a package
/// type (`00971c30`).
const PROCESS_LISTS_FIND_ENTRY: u32 = 0x0097_1c30;
/// The number of items of such an entry (`005ae380`).
const ENTRY_COUNT: u32 = 0x005a_e380;
/// Registers a crime with the process lists (`009721f0`).
const PROCESS_LISTS_ADD_CRIME: u32 = 0x0097_21f0;
/// The array of actors embedded in the process lists (`00871a30`: `this +
/// 0x88`) and its count (`00472380`).
const PROCESS_LISTS_ACTOR_ARRAY: u32 = 0x0087_1a30;
const PROCESS_LISTS_ACTOR_COUNT: u32 = 0x0047_2380;
/// `ECX` = actor, the player: `-100` without a process, else the word at `+8`
/// of the entry slot `0x504` of the process gives (`008a8230`).
const LEVEL_TOWARD_ACTOR: u32 = 0x008a_8230;
/// The number of witnesses of a crime (`009ebab0`, 0 when its list is empty).
const CRIME_WITNESS_COUNT: u32 = 0x009e_bab0;
/// Adds an actor to a crime's witness list unless present (`009eb9c0`).
const CRIME_ADD_WITNESS: u32 = 0x009e_b9c0;
/// `Crime::Crime` (Xbox PDB): `ECX` = the block, six words.
const CRIME_CONSTRUCTOR: u32 = 0x009e_b500;
/// Whether an actor is in a crime's witness list (`009ec810`).
const CRIME_HAS_WITNESS: u32 = 0x009e_c810;
/// Adds an actor to a crime's witness list (`009ec830`).
const CRIME_APPEND_WITNESS: u32 = 0x009e_c830;
/// The alarm package constructor (`009ec670`): `ECX` = the block, the record.
const ALARM_PACKAGE_CONSTRUCTOR: u32 = 0x009e_c670;
/// `FleePackage::AddAvoidedRef` (Xbox PDB).
const FLEE_PACKAGE_ADD_AVOIDED_REF: u32 = 0x009f_1310;
/// The log helper `00703c00` (cdecl, one string).
const LOG_MESSAGE: u32 = 0x0070_3c00;
/// `TESPackage::CreatePackage` (Xbox PDB), `cdecl`.
const CREATE_PACKAGE: u32 = 0x0067_0b90;
/// `TESPackage::SetPackType` (Xbox PDB).
const SET_PACK_TYPE: u32 = 0x0067_0fc0;
/// `TESPackage::SetPackageLocation` (Xbox PDB).
const SET_PACKAGE_LOCATION: u32 = 0x0067_1d30;
/// `TESPackage::SetPackageTarget` (Xbox PDB).
const SET_PACKAGE_TARGET: u32 = 0x0067_2fc0;
/// The package's target object (`00671d10`: the word at `+0x30`).
const PACKAGE_TARGET_OBJECT: u32 = 0x0067_1d10;
/// `TESPackage::IsInterruptPackage` (Xbox PDB).
const IS_INTERRUPT_PACKAGE: u32 = 0x0067_8610;
/// Package flag accessors: bit `0x800000` (set `00671a20`, get `0067a460`),
/// bit `0x200000` (set `0067a640`, get `00441b00`), bit `0x400000` (get
/// `0067a3f0`) and the setters of bits 2 and 4 (`00826b40`, `00826b90`).
const PACKAGE_SET_FLAG_800000: u32 = 0x0067_1a20;
const PACKAGE_GET_FLAG_800000: u32 = 0x0067_a460;
const PACKAGE_SET_FLAG_200000: u32 = 0x0067_a640;
const PACKAGE_GET_FLAG_200000: u32 = 0x0044_1b00;
const PACKAGE_GET_FLAG_400000: u32 = 0x0067_a3f0;
const PACKAGE_SET_FLAG_2: u32 = 0x0082_6b40;
const PACKAGE_SET_FLAG_4: u32 = 0x0082_6b90;
/// `PackageLocation::PackageLocation` (Xbox PDB).
const PACKAGE_LOCATION_CONSTRUCTOR: u32 = 0x0067_f030;
/// `PackageLocation::SetLocReference` (Xbox PDB).
const SET_LOC_REFERENCE: u32 = 0x0067_f3c0;
/// `PackageLocation::~PackageLocation` (Xbox PDB): `ECX` = the block, delete flag.
const PACKAGE_LOCATION_DESTRUCTOR: u32 = 0x0067_0b30;
/// `PackageTarget::PackageTarget` (Xbox PDB).
const PACKAGE_TARGET_CONSTRUCTOR: u32 = 0x0067_ff70;
/// `PackageTarget::SetTargType` (Xbox PDB).
const SET_TARG_TYPE: u32 = 0x0068_00b0;
/// `PackageTarget::SetTargReference` (Xbox PDB).
const SET_TARG_REFERENCE: u32 = 0x0068_0110;
/// Stores the word at `+8` of the package target (`00403550`).
const SET_TARGET_WORD_AT_8: u32 = 0x0040_3550;
/// The package target's destructor (`007b3fa0`): `ECX` = the block, delete flag.
const PACKAGE_TARGET_DESTRUCTOR: u32 = 0x007b_3fa0;
/// Sets the word at `+0x18` of a package (`00984f60`).
const SET_PACKAGE_WORD_AT_0X18: u32 = 0x0098_4f60;
/// Script variable helpers: the script of a base form (`004826d0`, `cdecl`),
/// the reference's variables (`005673e0`), `Script::FindVariable`,
/// `ScriptLocals::SetVariable` and `ScriptLocals::GetVariable` (Xbox PDB).
const SCRIPT_OF_FORM: u32 = 0x0048_26d0;
const GET_SCRIPT_VARIABLES: u32 = 0x0056_73e0;
const SCRIPT_FIND_VARIABLE: u32 = 0x005a_c6a0;
const SCRIPT_LOCALS_SET_VARIABLE: u32 = 0x005a_9290;
const SCRIPT_LOCALS_GET_VARIABLE: u32 = 0x005a_9140;
/// `Calendar::GetMonth`, `GetDay`, `GetHour` (Xbox PDB) and the year getter.
const CALENDAR_GET_MONTH: u32 = 0x0086_7d20;
const CALENDAR_GET_DAY: u32 = 0x0086_7d60;
const CALENDAR_GET_HOUR: u32 = 0x0086_7da0;
const CALENDAR_GET_YEAR: u32 = 0x0086_7c60;
/// The count of major crimes: `iMajorCrimes & 0x3fffffff` (`005a4c00`).
const MAJOR_CRIME_COUNT: u32 = 0x005a_4c00;
/// Cached-values block tests: whether the process has one (`00884920`) and
/// whether a flag mask of the block is clear (`00884e90`).
const PROCESS_HAS_CACHED_VALUES: u32 = 0x0088_4920;
const CACHED_VALUES_FLAG_CLEAR: u32 = 0x0088_4e90;
/// Called on the `CachedValuesOwner` of an actor with a process (`008676d0`).
const CACHED_VALUES_OWNER_REFRESH: u32 = 0x0086_76d0;
/// Searches an array with a comparison callback (`00719b20`): `ECX` = the
/// array, the address of the key, a word and the callback; the index or -1.
const ARRAY_FIND_WITH_COMPARE: u32 = 0x0071_9b20;
/// The comparison `008c1680` passes.
const ARRAY_COMPARE_CALLBACK: u32 = 0x009a_3830;
/// `NiPointer` reference-count bump (`0087adf0`) and release (`0087cea0`).
const HIT_DATA_ADD_REFERENCE: u32 = 0x0087_adf0;
const HIT_DATA_RELEASE: u32 = 0x0087_cea0;
/// `BSSimpleArray<Actor *, 1024>` helpers: initialiser (`006b3eb0`) and the
/// destructor body (`008c1cb0`).
const SIMPLE_ARRAY_INITIALISE: u32 = 0x006b_3eb0;
const SIMPLE_ARRAY_DESTRUCTOR_BODY: u32 = 0x008c_1cb0;
/// Combat controller getters of `008c1470`: the word at `+0xc0`, the flag
/// (`00981420`), the group (`004fb070`: the word at `+0x80`) and the two
/// functions that fill an array from the group.
const COMBAT_CONTROLLER_GET_WORD: u32 = 0x0040_30b0;
const COMBAT_CONTROLLER_GET_FLAG: u32 = 0x0098_1420;
const COMBAT_CONTROLLER_GET_GROUP: u32 = 0x004f_b070;
const COMBAT_GROUP_FILL_TARGET_ARRAY: u32 = 0x0098_6760;
const COMBAT_GROUP_FILL_MEMBER_ARRAY: u32 = 0x0098_6b00;
const FIND_FIRST_COLLISION_OBJECT: u32 = 0x004b_5260;
const COLLISION_OBJECT_GET_TARGET: u32 = 0x006f_a820;
const HIT_SOUND_REQUEST_CONSTRUCTOR: u32 = 0x0062_40d0;
const TARGET_OBJECT_GET_OBJECT: u32 = 0x004a_e6a0;
const OBJECT_GET_KIND: u32 = 0x0062_05a0;
const IMPACT_MIXER_PLAY_COLLISION_SOUND: u32 = 0x0083_7550;
/// The constant `0.5f` stored in the sound request.
const HALF_FLOAT: u32 = 0x0101_6248;
/// The vtable of `BSSimpleArray<Actor *, 1024>` (RTTI `.?AV?$BSSimpleArray@PAVActor@@$0EAA@@@`).
const SIMPLE_ARRAY_VTABLE: u32 = 0x0108_4fec;
/// The format `"%s  attacking %s no one cared"`.
const NO_ONE_CARED_FORMAT: u32 = 0x0108_4fb0;
/// The name strings `008c1210` receives and ignores.
const NAME_STRING_NPC_ID: u32 = 0x0108_4fe0;
const NAME_STRING_REFERENCE_NPC_ID: u32 = 0x0108_4fd0;
/// The global setting objects the translations read.
const SETTING_STEAL_REPORT_LIMIT: u32 = 0x011c_d6fc;
const SETTING_ATTACK_ALARM_LIMIT: u32 = 0x011c_d9f4;
const SETTING_ALARM_ESCALATION_LIMIT: u32 = 0x011c_d808;
const SETTING_STEAL_WEIGHT_FACTOR: u32 = 0x011c_df84;
const SETTING_STEAL_KARMA_PENALTY: u32 = 0x011c_de20;
const SETTING_SPEED_BASE: u32 = 0x011d_0d68;
const SETTING_SPEED_FACTOR: u32 = 0x011c_ff58;
/// `0.0` (`double`).
const DOUBLE_ZERO: u32 = 0x0101_2060;
/// `-1.0` (`float`).
const FLOAT_MINUS_ONE: u32 = 0x0101_2054;
/// `-1.0` (`double`).
const DOUBLE_MINUS_ONE: u32 = 0x0101_a6b0;

/// The `float` setting stored in the setting object `setting` (`00403e20`).
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let at = e.call(SETTING_FLOAT_POINTER, &args![setting]).u32();
    e.mem.f32(at)
}

/// The integer setting stored in the setting object `setting` (`0043d4d0`).
fn setting_int(e: &mut Engine, setting: u32) -> i32 {
    let at = e.call(SETTING_INT_POINTER, &args![setting]).u32();
    e.mem.i32(at)
}

/// The actor's process (`008d8520`).
fn process_of(e: &mut Engine, actor: u32) -> u32 {
    e.call(ACTOR_PROCESS, &args![actor]).u32()
}

/// The calendar hour as the `float` the game pushes.
fn calendar_hour(e: &mut Engine) -> f32 {
    e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f32()
}

/// The theft date stamp: month `<< 9`, day, year `<< 13`.
fn theft_date_stamp(e: &mut Engine) -> u32 {
    let month = e.call(CALENDAR_GET_MONTH, &args![CALENDAR]).u32();
    let day = e.call(CALENDAR_GET_DAY, &args![CALENDAR]).u8() as i8 as i32 as u32;
    let year = e.call(CALENDAR_GET_YEAR, &args![CALENDAR]).u32();
    (month << 9) | day | (year << 13)
}

/// The "already stamped" test of the theft stamp: the stamp's day (low nine
/// bits) must still be today's; otherwise `bAttackOnNextTheft` is cleared.
fn theft_stamp_is_today(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let stamp = e.get(this, Actor::iThiefCrimeStamp);
    let mut today = true;
    if (stamp as i32) > 0 {
        let day = e.call(CALENDAR_GET_DAY, &args![CALENDAR]).u8() as i8 as i32;
        if (stamp & 0x1ff) as i32 != day {
            today = false;
            e.set(this, Actor::bAttackOnNextTheft, false);
        }
    }
    today
}

/// Empties and destroys a witness list the way the game does after a crime
/// broadcast (nothing for a null list).
fn destroy_list(e: &mut Engine, list: u32) {
    if list != 0 {
        e.call(LIST_CLEAR, &args![list]);
        e.call(LIST_DESTROY, &args![list, 1u32]);
    }
}

/// Allocates and constructs a `Crime` (`0x3c` bytes); 0 if the allocation fails.
fn new_crime(e: &mut Engine, kind: u32, words: [u32; 5]) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x3cu32]).u32();
    if block != 0 {
        e.call(
            CRIME_CONSTRUCTOR,
            &args![block, kind, words[0], words[1], words[2], words[3], words[4]],
        )
        .u32()
    } else {
        0
    }
}

/// Casts a reference to `Actor` (`__RTDynamicCast`).
fn cast_to_actor(e: &mut Engine, object: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![object, 0u32, CAST_FROM_REFERENCE, CAST_TO_ACTOR, 0u32],
    )
    .u32()
}

/// Makes `witness` say the topic `(2, topic_id)` about the crime: slot `0x2a4`
/// of its process with the speaker `witness`.
fn witness_says(e: &mut Engine, witness: u32, topic: u32) {
    let witness_process = process_of(e, witness);
    e.vcall(
        witness_process,
        0x2a4,
        &args![witness, topic, 0u32, 0u32, 1u32, 0u32],
    );
}

/// Slot `0x424` of `witness` with `target` and the flags every caller uses
/// (make the witness react against the crime's reference).
fn witness_reacts(e: &mut Engine, witness: u32, target: u32) {
    e.vcall(
        witness,
        0x424,
        &args![target, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
    );
}

/// Begins a witness line: records the form `speaker_form` in the thread block
/// (`008bff30`), tells the witness which crime reference is meant
/// (`0057bd60`) and returns the topic `(2, topic_id)`.
fn begin_witness_line(
    e: &mut Engine,
    form_owner: u32,
    crime: u32,
    witness: u32,
    topic_id: u32,
) -> u32 {
    let form = e.call(GET_FORM, &args![form_owner]).u32();
    fn_008bff30(e, form);
    let speaker = e.call(RECORD_REFERENCE, &args![crime]).u32();
    e.call(SET_WORD_AT_0X70, &args![witness, speaker]);
    e.call(GET_TOPIC, &args![2u32, topic_id]).u32()
}

// Translated from 008bec80 (decompiled, FalloutNV.exe 1.4.0.525)
/// A cached `float` value of the actor: when the actor's process has a
/// cached-values block, the value `008becf0` gives for the
/// `CachedValuesOwner` at `+0xa8`; otherwise slot `0x18` of that owner.
pub fn fn_008bec80(e: &mut Engine, this: Ptr<Actor>) -> f32 {
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null() && e.call(PROCESS_HAS_CACHED_VALUES, &args![process]).bool() {
        let owner = if this.is_null() {
            0
        } else {
            this.addr() + 0xa8
        };
        return fn_008becf0(e, process, Ptr::new(owner));
    }
    e.vcall(this.addr() + 0xa8, 0x18, &args![]).f32()
}

// Translated from 008becf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value of a process's cached-values block (the pointer at `+0x2c`):
/// 0 without a block; slot `0x18` of `owner` when bit `0x80000000` of the
/// block's flag word (`+0x44`) is clear; otherwise the `float` at `+0x18` of
/// the block.
pub fn fn_008becf0(e: &mut Engine, this: Ptr, owner: Ptr) -> f32 {
    let cached = e.mem.u32(this.addr() + 0x2c);
    if cached == 0 {
        return 0.0;
    }
    if e.call(CACHED_VALUES_FLAG_CLEAR, &args![cached, 0x8000_0000u32])
        .bool()
    {
        e.vcall(owner.addr(), 0x18, &args![]).f32()
    } else {
        e.mem.f32(cached + 0x18)
    }
}

// Translated from 008bed50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives the actor an alarm package (package type `0x15`) about the alarm
/// record `record` (its reference is the word at `+0xc`; `reference` stands
/// in when the record is null). Does nothing when the actor already runs a
/// package of that type; otherwise it may notify the reference's process,
/// ends an interrupt package, makes guards avoid the reference and builds
/// and starts the package, or adds the record to the package it has.
/// `process_argument` goes to slot `0x284` of the actor's process at the end;
/// `flag` selects the early "tell the process about the record" path.
pub fn fn_008bed50(
    e: &mut Engine,
    this: Ptr<Actor>,
    record: Ptr,
    process_argument: u32,
    flag: bool,
    mut reference: Ptr,
) {
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null() {
        let current = e.vcall(process.addr(), 0x22c, &args![]).u32();
        if current != 0 {
            let current = e.vcall(process.addr(), 0x22c, &args![]).u32();
            if e.call(PACKAGE_TYPE, &args![current]).i32() == 0x15 {
                return;
            }
        }
    }
    let player = e.global::<u32>(PLAYER_POINTER);
    if e.vcall(this.addr(), 0x304, &args![]).bool() {
        if !record.is_null() && reference.is_null() {
            reference = e.call(RECORD_REFERENCE, &args![record]).ptr();
        }
        let entry = e
            .call(
                PROCESS_LISTS_FIND_ENTRY,
                &args![PROCESS_LISTS, reference, 0x15u32, 0u32],
            )
            .u32();
        if entry == 0 {
            let reference_process = process_of(e, reference.addr());
            e.vcall(reference_process, 0x348, &args![0u32]);
        } else {
            let reference_process = process_of(e, reference.addr());
            let count = e.call(ENTRY_COUNT, &args![entry]).u32();
            e.vcall(reference_process, 0x348, &args![count]);
        }
        if !reference.is_null() {
            let reference_process = process_of(e, reference.addr());
            let level = e.vcall(reference_process, 0x5c0, &args![]).i32();
            if level >= setting_int(e, SETTING_ALARM_ESCALATION_LIMIT) {
                return;
            }
        }
        if !reference.is_null() {
            if reference.addr() == player && e.call(PLAYER_BYTE_AT_0X7C4, &args![player]).bool() {
                let own_process = e.get(this, Actor::pCurrentProcess).addr();
                e.vcall(
                    own_process,
                    0x33c,
                    &args![
                        this, reference, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                        1u32, 0u32
                    ],
                );
                return;
            }
            let reference_process = process_of(e, reference.addr());
            e.vcall(reference_process, 0x5c4, &args![1u32]);
        }
    }
    if e.vcall(this.addr(), 0x214, &args![]).u32() == 9 {
        e.vcall(this.addr(), 0x418, &args![]);
    }
    let mut existing = 0u32;
    let mut package = e.call(GET_PACKAGE_SET_AS_CURRENT, &args![this]).u32();
    if package != 0 && e.call(IS_INTERRUPT_PACKAGE, &args![package]).bool() {
        let list = e.call(EXTRA_LIST_OF_REFERENCE, &args![this]).u32();
        package = e.call(GET_PACKAGE_EXTRA, &args![list]).u32();
    }
    let mut busy = e.call(IN_COMBAT_FLAG, &args![this]).bool()
        || e.vcall(this.addr(), 0x22c, &args![0u32]).bool()
        || e.call(IS_FLEEING, &args![this, 0u32]).bool();
    if !busy
        && package != 0
        && e.call(PACKAGE_GET_FLAG_400000, &args![package]).bool()
        && !record.is_null()
    {
        let record_value = e.call(WORD_AT_8, &args![record]).u32();
        let own_process = e.get(this, Actor::pCurrentProcess).addr();
        let handled = e.vcall(own_process, 0x128, &args![]).u32();
        if record_value != handled {
            busy = true;
        }
    }
    if busy && flag {
        if !e.vcall(this.addr(), 0x22c, &args![0u32]).bool()
            && !e.call(RECORD_BYTE_AT_0X10, &args![record]).bool()
        {
            let target = e.call(RECORD_REFERENCE, &args![record]).u32();
            let own_process = e.get(this, Actor::pCurrentProcess).addr();
            e.vcall(own_process, 0x344, &args![this, target, record]);
        }
        return;
    }
    let own_process = e.get(this, Actor::pCurrentProcess).addr();
    e.vcall(own_process, 0x214, &args![]);
    let current = e.call(GET_CURRENT_PACKAGE, &args![this]).u32();
    if current != 0 {
        let current = e.call(GET_CURRENT_PACKAGE, &args![this]).u32();
        if e.call(PACKAGE_TYPE, &args![current]).i32() == 0x15 {
            existing = e.call(GET_CURRENT_PACKAGE, &args![this]).u32();
        } else {
            let current = e.call(GET_CURRENT_PACKAGE, &args![this]).u32();
            if e.call(IS_INTERRUPT_PACKAGE, &args![current]).bool() {
                e.call(END_INTERRUPT_PACKAGE, &args![this, 0u32]);
            }
        }
    }
    if !record.is_null() {
        let class = e.call(ACTOR_GET_CLASS, &args![this]).u32();
        if e.call(CLASS_IS_GUARD, &args![class]).bool() {
            fn_008bf4e0(e, record, 1);
            let target = e.call(RECORD_REFERENCE, &args![record]).u32();
            if e.vcall(target, 0x218, &args![]).bool() {
                let target = e.call(RECORD_REFERENCE, &args![record]).u32();
                if target != player {
                    let target = e.call(RECORD_REFERENCE, &args![record]).u32();
                    if !e.call(IN_COMBAT_FLAG, &args![target]).bool() {
                        let target = e.call(RECORD_REFERENCE, &args![record]).u32();
                        if e.call(GET_REF_PERSISTS, &args![target]).bool() {
                            let target = e.call(RECORD_REFERENCE, &args![record]).u32();
                            if !e.call(IS_FLEEING, &args![target, 0u32]).bool() {
                                let minus_one_a = e.global::<f32>(FLOAT_MINUS_ONE);
                                let minus_one_b = e.global::<f32>(FLOAT_MINUS_ONE);
                                e.vcall(
                                    target,
                                    0x410,
                                    &args![
                                        this,
                                        0u32,
                                        1u32,
                                        1u32,
                                        0u32,
                                        0u32,
                                        minus_one_b,
                                        minus_one_a
                                    ],
                                );
                            }
                            let package_of_target =
                                e.call(GET_CURRENT_PACKAGE, &args![target]).u32();
                            let flee_package = e
                                .call(
                                    RT_DYNAMIC_CAST,
                                    &args![
                                        package_of_target,
                                        0u32,
                                        CAST_FROM_PACKAGE,
                                        CAST_TO_FLEE_PACKAGE,
                                        0u32
                                    ],
                                )
                                .u32();
                            if flee_package != 0 {
                                e.call(FLEE_PACKAGE_ADD_AVOIDED_REF, &args![flee_package, this]);
                            }
                        }
                    }
                }
            }
        }
    }
    if existing == 0 {
        let own_process = process_of(e, this.addr());
        let process_package = e.vcall(own_process, 0x22c, &args![]).u32();
        let block = e.call(OPERATOR_NEW, &args![0x88u32]).u32();
        let alarm = if block != 0 {
            e.call(ALARM_PACKAGE_CONSTRUCTOR, &args![block, record])
                .u32()
        } else {
            0
        };
        e.call(SET_PACK_TYPE, &args![alarm, 0x15u32]);
        if process_package != 0 {
            let bit_800000 = e
                .call(PACKAGE_GET_FLAG_800000, &args![process_package])
                .u8();
            e.call(PACKAGE_SET_FLAG_800000, &args![alarm, bit_800000]);
            let bit_200000 = e
                .call(PACKAGE_GET_FLAG_200000, &args![process_package])
                .u8();
            e.call(PACKAGE_SET_FLAG_200000, &args![alarm, bit_200000]);
        }
        e.call(PACKAGE_SET_FLAG_2, &args![alarm, 1u32]);
        e.call(PACKAGE_SET_FLAG_4, &args![alarm, 1u32]);
        let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
        let location = if block != 0 {
            e.call(PACKAGE_LOCATION_CONSTRUCTOR, &args![block]).u32()
        } else {
            0
        };
        if !record.is_null() {
            let target = e.call(RECORD_REFERENCE, &args![record]).u32();
            e.call(SET_LOC_REFERENCE, &args![location, target]);
        } else {
            e.call(SET_LOC_REFERENCE, &args![location, reference]);
        }
        e.call(SET_PACKAGE_LOCATION, &args![alarm, location]);
        if location != 0 {
            e.call(PACKAGE_LOCATION_DESTRUCTOR, &args![location, 1u32]);
        }
        let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
        let target = if block != 0 {
            e.call(PACKAGE_TARGET_CONSTRUCTOR, &args![block]).u32()
        } else {
            0
        };
        e.call(SET_PACKAGE_TARGET, &args![alarm, target]);
        if target != 0 {
            e.call(PACKAGE_TARGET_DESTRUCTOR, &args![target, 1u32]);
        }
        e.call(SET_PACKAGE_WORD_AT_0X18, &args![alarm, 0xbu32]);
        let package_target = e.call(PACKAGE_TARGET_OBJECT, &args![alarm]).u32();
        e.call(SET_TARG_TYPE, &args![package_target, 0u32]);
        if !record.is_null() {
            let target = e.call(RECORD_REFERENCE, &args![record]).u32();
            let package_target = e.call(PACKAGE_TARGET_OBJECT, &args![alarm]).u32();
            e.call(SET_TARG_REFERENCE, &args![package_target, target]);
        } else {
            let package_target = e.call(PACKAGE_TARGET_OBJECT, &args![alarm]).u32();
            e.call(SET_TARG_REFERENCE, &args![package_target, reference]);
        }
        let own_process = process_of(e, this.addr());
        e.vcall(own_process, 0x28, &args![]);
        let own_process = process_of(e, this.addr());
        e.vcall(own_process, 0x710, &args![this]);
        e.vcall(this.addr(), 0x2f4, &args![alarm, 0u32, 1u32]);
        if !record.is_null() {
            e.call(CRIME_ADD_WITNESS, &args![record, this]);
        }
        let own_process = e.get(this, Actor::pCurrentProcess).addr();
        e.vcall(own_process, 0x284, &args![process_argument]);
    } else if !record.is_null() {
        let own_process = e.get(this, Actor::pCurrentProcess).addr();
        let handled = e.vcall(own_process, 0x128, &args![]).u32();
        let record_reference = e.call(RECORD_REFERENCE, &args![record]).u32();
        if handled == record_reference
            && !e.call(CRIME_HAS_WITNESS, &args![existing, record]).bool()
        {
            e.call(CRIME_APPEND_WITNESS, &args![existing, record]);
        }
    }
}

// Translated from 008bf4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte `value` at `+0x2c` of the object `this` (the alarm
/// record: `008bed50` sets it to 1 for guards).
pub fn fn_008bf4e0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x2c, value);
}

/// The package set-up `008bf500` and `008bf7b0` share: unless the actor is in
/// combat, builds a package of type `package_type` located at and aimed at
/// `target` (procedure word `kind`), starts it on `this` and bumps the
/// player's counter `counter`. `copy_flag` is the extra step of `008bf500`
/// that copies bit `0x200000` of the process's current package.
fn start_target_package(
    e: &mut Engine,
    this: Ptr<Actor>,
    target: Ptr,
    package_type: u32,
    kind: u32,
    copy_flag: bool,
    counter: u32,
) {
    if e.call(IN_COMBAT_FLAG, &args![this]).bool() {
        return;
    }
    let process = e.get(this, Actor::pCurrentProcess).addr();
    e.vcall(process, 0x214, &args![]);
    let own_process = process_of(e, this.addr());
    e.vcall(own_process, 0x644, &args![1u32]);
    let own_process = process_of(e, this.addr());
    let process_package = e.vcall(own_process, 0x22c, &args![]).u32();
    let current = e.call(GET_CURRENT_PACKAGE, &args![this]).u32();
    if current != 0 {
        let current = e.call(GET_CURRENT_PACKAGE, &args![this]).u32();
        if e.call(IS_INTERRUPT_PACKAGE, &args![current]).bool() {
            e.vcall(this.addr(), 0x288, &args![]);
        }
    }
    let package = e.call(CREATE_PACKAGE, &args![package_type]).u32();
    e.call(SET_PACK_TYPE, &args![package, package_type]);
    e.call(PACKAGE_SET_FLAG_2, &args![package, 1u32]);
    e.call(PACKAGE_SET_FLAG_4, &args![package, 1u32]);
    let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    let location = if block != 0 {
        e.call(PACKAGE_LOCATION_CONSTRUCTOR, &args![block]).u32()
    } else {
        0
    };
    e.call(SET_LOC_REFERENCE, &args![location, target]);
    e.call(SET_PACKAGE_LOCATION, &args![package, location]);
    if location != 0 {
        e.call(PACKAGE_LOCATION_DESTRUCTOR, &args![location, 1u32]);
    }
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let package_target = if block != 0 {
        e.call(PACKAGE_TARGET_CONSTRUCTOR, &args![block]).u32()
    } else {
        0
    };
    e.call(SET_PACKAGE_TARGET, &args![package, package_target]);
    if package_target != 0 {
        e.call(PACKAGE_TARGET_DESTRUCTOR, &args![package_target, 1u32]);
    }
    e.call(SET_PACKAGE_WORD_AT_0X18, &args![package, kind]);
    let target_object = e.call(PACKAGE_TARGET_OBJECT, &args![package]).u32();
    e.call(SET_TARG_TYPE, &args![target_object, 0u32]);
    let target_object = e.call(PACKAGE_TARGET_OBJECT, &args![package]).u32();
    e.call(SET_TARG_REFERENCE, &args![target_object, target]);
    let target_object = e.call(PACKAGE_TARGET_OBJECT, &args![package]).u32();
    e.call(SET_TARGET_WORD_AT_8, &args![target_object, 0u32]);
    if copy_flag && process_package != 0 {
        let bit_200000 = e
            .call(PACKAGE_GET_FLAG_200000, &args![process_package])
            .u8();
        e.call(PACKAGE_SET_FLAG_200000, &args![package, bit_200000]);
    }
    let own_process = process_of(e, this.addr());
    e.vcall(own_process, 0x28, &args![]);
    let own_process = process_of(e, this.addr());
    e.vcall(own_process, 0x710, &args![this]);
    e.vcall(this.addr(), 0x2f4, &args![package, 0u32, 1u32]);
    let player = e.global::<u32>(PLAYER_POINTER);
    e.call(counter, &args![player]);
}

// Translated from 008bf500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts a package of type `0x23` (procedure word `0x2b`) aimed at `target`,
/// unless the actor is in combat, and bumps the player's counter `00966e50`.
/// It also copies bit `0x200000` of the process's current package into the
/// new one.
pub fn fn_008bf500(e: &mut Engine, this: Ptr<Actor>, target: Ptr) {
    start_target_package(e, this, target, 0x23, 0x2b, true, PLAYER_ALARM_COUNTER_23);
}

// Translated from 008bf7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same package set-up as [`fn_008bf500`] with package type `0x22`
/// (procedure word `0x2a`), without the flag copy, bumping the player's
/// counter `00966e00`. The second stack word is never read.
pub fn fn_008bf7b0(e: &mut Engine, this: Ptr<Actor>, target: Ptr, _unused_1: u32) {
    start_target_package(e, this, target, 0x22, 0x2a, false, PLAYER_ALARM_COUNTER_22);
}

// Translated from 008bfa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::StealAlarm` (Xbox PDB): the player (`this`) took `item` (`count`
/// of them) from `taken_from`. Raises the player's theft counter, finds the
/// witness (the reference itself when it is an actor, else the actor of its
/// owner), penalises karma when the owner is not of the good alignments `2`
/// and `4`, and when a witness who is not dead detects the player builds a
/// `Crime` (kind 0), reports it and makes the witness react. `extra_value`
/// is only used when `item` is null.
pub fn actor_steal_alarm(
    e: &mut Engine,
    this: Ptr<Actor>,
    taken_from: Ptr,
    item: u32,
    count: u32,
    extra_value: u32,
    owner_argument: u32,
) {
    let player = e.global::<u32>(PLAYER_POINTER);
    if this.addr() != player {
        return;
    }
    let mut report_limit_reached = false;
    let mut witness = 0u32;
    let theft_counter = fn_008bffa0(e, Ptr::new(player)) as i32;
    if theft_counter < setting_int(e, SETTING_STEAL_REPORT_LIMIT) && this.addr() == player {
        report_limit_reached = true;
    }
    if this.addr() == player {
        fn_008bff70(e, Ptr::new(player), 0, count as i32);
    } else {
        // Unreachable (the actor is the player here); kept as the game has it.
        let owner = this.addr() + 0xa4;
        let value = e.vcall(owner, 8, &args![0x2au32]).i32();
        if value >= 100 {
            return;
        }
    }
    let mut stolen_from_actor = 0u32;
    let mut karma_penalty = false;
    if e.vcall(taken_from.addr(), 0x100, &args![]).bool() {
        stolen_from_actor = taken_from.addr();
    }
    let mut owner = owner_argument;
    if owner == 0 && stolen_from_actor != 0 {
        owner = e.call(GET_OWNER, &args![stolen_from_actor]).u32();
    }
    let flag = e.call(WEIGHT_FLAG_OF_PLAYER, &args![player]).u8();
    let mut weight = e.call(GET_FORM_WEIGHT, &args![item, flag]).f32();
    let zero = e.global::<f64>(DOUBLE_ZERO);
    // The game skips the weight when it is at most zero or not a number.
    if weight as f64 > zero {
        weight *= setting_float(e, SETTING_STEAL_WEIGHT_FACTOR);
        let own_process = process_of(e, this.addr());
        e.vcall(own_process, 0x4a4, &args![weight]);
    }
    if stolen_from_actor != 0 && e.vcall(stolen_from_actor, 0x218, &args![]).bool() {
        witness = stolen_from_actor;
    } else {
        let owner_type = e.call(FORM_TYPE, &args![owner]).u32();
        if owner_type == 8 {
            if !e.call(BASE_FORM_FLAG_BIT_1, &args![owner]).bool() {
                karma_penalty = true;
            }
            witness = e
                .call(
                    PROCESS_LISTS_FIND_ACTOR_REF,
                    &args![PROCESS_LISTS, owner, 1u32, 0u32],
                )
                .u32();
        } else if owner != 0 {
            let karma = e.vcall(owner + 0x100, 0xc, &args![0x17u32]).f32();
            let alignment = e.call(GET_ALIGNMENT_FOR_KARMA, &args![karma]).i32();
            if alignment != 2 && alignment != 4 {
                karma_penalty = true;
            }
            witness = e
                .call(
                    PROCESS_LISTS_GET_ACTOR_REF_IN_HIGH,
                    &args![PROCESS_LISTS, owner, 1u32],
                )
                .u32();
        }
    }
    if karma_penalty {
        let penalty = setting_float(e, SETTING_STEAL_KARMA_PENALTY);
        let amount = e.call(FLOAT_TO_INT, &args![penalty as f64]).i32();
        e.call(REWARD_KARMA, &args![player, amount]);
    }
    if witness == 0 || e.vcall(witness, 0x320, &args![]).bool() {
        return;
    }
    let detection = e.with_stack(4, |e, out| {
        e.call(
            GET_DETECTION_LEVEL,
            &args![witness, 0u32, this, out, 0u32, 0u32, 0u32, 0u32],
        )
        .i32()
    });
    if detection <= 0 {
        return;
    }
    let crime = if item != 0 {
        new_crime(
            e,
            0,
            [taken_from.addr(), this.addr(), item, count, owner_argument],
        )
    } else {
        new_crime(
            e,
            0,
            [
                taken_from.addr(),
                this.addr(),
                item,
                extra_value,
                owner_argument,
            ],
        )
    };
    let hour = calendar_hour(e);
    let extra_list = e.call(EXTRA_LIST_OF_REFERENCE, &args![witness]).u32();
    e.call(ADD_TO_PLAYER_CRIME_LIST, &args![extra_list, crime, hour]);
    fn_008bff50(e, Ptr::new(witness));
    e.call(ADD_FACTION_MINOR_CRIME, &args![witness, 1u32, 1u32]);
    e.call(PROCESS_LISTS_ADD_CRIME, &args![PROCESS_LISTS, crime]);
    e.call(CRIME_ADD_WITNESS, &args![crime, witness]);
    if e.get(this, Actor::bAttackOnNextTheft) {
        if theft_stamp_is_today(e, this) {
            e.vcall(
                witness,
                0x424,
                &args![player, 0u32, 1u32, 0u32, 0u32, 0u32, 1u32, 0u32],
            );
            return;
        }
    } else if e.call(GET_PLAYER_ACTION, &args![player, 5u32]).u32() == 0 {
        e.set(this, Actor::bAttackOnNextTheft, true);
        let stamp = theft_date_stamp(e);
        e.set(this, Actor::iThiefCrimeStamp, stamp);
    }
    if report_limit_reached {
        fn_008bf7b0(e, Ptr::new(witness), this.cast(), owner_argument);
        return;
    }
    fn_008bff30(e, owner);
    let speaker = e.call(RECORD_REFERENCE, &args![crime]).u32();
    e.call(SET_WORD_AT_0X70, &args![this, speaker]);
    let topic = e.call(GET_TOPIC, &args![2u32, 9u32]).u32();
    witness_says(e, witness, topic);
    fn_008bff30(e, 0);
}

// Translated from 008bff30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the thread-local block at `+0x26c`.
pub fn fn_008bff30(e: &mut Engine, value: u32) {
    let block = e.tls();
    e.mem.set_u32(block + 0x26c, value);
}

// Translated from 008bff50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds one to the actor's `iMinorCrimes` counter.
pub fn fn_008bff50(e: &mut Engine, this: Ptr<Actor>) {
    let count = e.get(this, Actor::iMinorCrimes);
    e.set(this, Actor::iMinorCrimes, count.wrapping_add(1));
}

// Translated from 008bff70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `delta` to entry `index` of the player's counter array at `+0x744`.
pub fn fn_008bff70(e: &mut Engine, this: Ptr, index: u32, delta: i32) {
    let at = this.addr().wrapping_add(0x744).wrapping_add(index * 4);
    let value = e.mem.i32(at);
    e.mem.set_i32(at, value.wrapping_add(delta));
}

// Translated from 008bffa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0x228` of the player (the theft counter `008bfa40`
/// compares with a setting).
pub fn fn_008bffa0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x228)
}

// Translated from 008bffc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsAngryWithPlayer` (Xbox PDB): true when the actor's extra data
/// carries a non-empty player crime list and either the actor has no
/// process, or its process has no package, or the package is not of type
/// `0x22` / `0x23`.
pub fn actor_is_angry_with_player(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let extra_list = e.call(EXTRA_LIST_OF_REFERENCE, &args![this]).u32();
    let crimes = e.call(GET_PLAYER_CRIME_LIST, &args![extra_list]).u32();
    if crimes == 0 || e.call(LIST_IS_EMPTY, &args![crimes]).bool() {
        return false;
    }
    if process_of(e, this.addr()) == 0 {
        return true;
    }
    let process = process_of(e, this.addr());
    let package = e.vcall(process, 0x22c, &args![]).u32();
    if package == 0 {
        return true;
    }
    let kind = e.call(PACKAGE_TYPE, &args![package]).i32();
    kind != 0x22 && kind != 0x23
}

// Translated from 008c0050 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the object that slot `0x27c` of the actor's process returns casts
/// (`0x011846a0` to `0x011a3328`), or when `Actor::IsRunningRunOnce` and that
/// object's type byte is 1.
pub fn fn_008c0050(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let process = process_of(e, this.addr());
    let object = e.vcall(process, 0x27c, &args![]).u32();
    let cast = e
        .call(
            RT_DYNAMIC_CAST,
            &args![object, 0u32, CAST_FROM_PACKAGE, CAST_TO_PROCESS_TYPE, 0u32],
        )
        .u32();
    if cast != 0 {
        return true;
    }
    if e.call(IS_RUNNING_RUN_ONCE, &args![this]).bool() {
        let process = process_of(e, this.addr());
        let object = e.vcall(process, 0x27c, &args![]).u32();
        return e.call(PACKAGE_TYPE, &args![object]).i32() == 1;
    }
    false
}

// Translated from 008c00e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The player (`this`) harmed `victim` (a reference cast to `Actor`): builds a
/// `Crime` of kind 1, reports it and starts the pursuit package `008bf500`.
/// Does nothing for another actor; a dead victim (slot `0x320`) or one that
/// slot `0x448` accepts is only flagged in its process. `argument_a` and
/// `argument_b` go into the crime.
pub fn fn_008c00e0(
    e: &mut Engine,
    this: Ptr<Actor>,
    victim: Ptr,
    argument_a: u32,
    argument_b: u32,
) {
    let player = e.global::<u32>(PLAYER_POINTER);
    if this.addr() != player {
        return;
    }
    // The game reads these two and never uses them.
    e.call(PLAYER_WORD_AT_0X230, &args![player]);
    setting_int(e, SETTING_ATTACK_ALARM_LIMIT);
    let actor = cast_to_actor(e, victim.addr());
    if e.vcall(actor, 0x320, &args![]).bool() || e.vcall(actor, 0x448, &args![]).bool() {
        let process = process_of(e, actor);
        e.vcall(process, 0x114, &args![1u32]);
        return;
    }
    if this.addr() != player {
        // Unreachable (the actor is the player here); kept as the game has it.
        let owner = this.addr() + 0xa4;
        if e.call(GET_CLAMPED_ACTOR_VALUE, &args![owner, 0x2au32])
            .i32()
            == 100
            && e.call(ACTOR_FLAG_TEST, &args![this]).bool()
        {
            return;
        }
    }
    let crime = new_crime(
        e,
        1,
        [victim.addr(), this.addr(), argument_a, argument_b, 0],
    );
    let crime_word_8 = if crime != 0 {
        e.call(WORD_AT_8, &args![crime]).u32()
    } else {
        0
    };
    if crime == 0 || victim.addr() != crime_word_8 {
        let crime_owner = e.call(WORD_AT_8, &args![crime]).u32();
        e.call(IS_AN_OWNER, &args![crime_owner, victim, 1u32]);
    }
    let victim_process = process_of(e, victim.addr());
    let reference = e.call(RECORD_REFERENCE, &args![crime]).u32();
    let entry = e
        .vcall(victim_process, 0x504, &args![reference, 0u32])
        .u32();
    if entry == 0 {
        let victim_process = process_of(e, victim.addr());
        e.vcall(victim_process, 0x508, &args![victim, 3u32]);
    } else {
        e.mem.set_u32(entry + 4, 3);
    }
    let form = e.call(GET_FORM, &args![victim]).u32();
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, CAST_FROM_FORM, CAST_TO_NPC, 0u32],
    );
    let actor_process = process_of(e, actor);
    e.vcall(actor_process, 0x114, &args![1u32]);
    let hour = calendar_hour(e);
    let extra_list = e.call(EXTRA_LIST_OF_REFERENCE, &args![victim]).u32();
    e.call(ADD_TO_PLAYER_CRIME_LIST, &args![extra_list, crime, hour]);
    e.call(CRIME_ADD_WITNESS, &args![crime, this]);
    e.call(PROCESS_LISTS_ADD_CRIME, &args![PROCESS_LISTS, crime]);
    fn_008bff50(e, victim.cast());
    e.call(ADD_FACTION_MINOR_CRIME, &args![victim, 1u32, 1u32]);
    if e.get(this, Actor::bAttackOnNextTheft) {
        if theft_stamp_is_today(e, this) {
            e.vcall(
                this.addr(),
                0x424,
                &args![player, 0u32, 1u32, 0u32, 0u32, 0u32, 1u32, 0u32],
            );
            return;
        }
    } else {
        e.set(this, Actor::bAttackOnNextTheft, true);
        let stamp = theft_date_stamp(e);
        e.set(this, Actor::iThiefCrimeStamp, stamp);
    }
    fn_008bf500(e, victim.cast(), this.cast());
}

// Translated from 008c0460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::AttackAlarm` (Xbox PDB): `attacker` (only the player counts)
/// attacked `this`. Builds a `Crime` of kind 3, bumps the player's counter 3,
/// reports it and lets every living witness react (an angry line and a
/// witness entry for a witness on the victim's side, otherwise an attack or a
/// plain line). `minor` selects the minor faction crime (and the victim itself
/// as the only angry witness); otherwise a major one is charged.
pub fn actor_attack_alarm(
    e: &mut Engine,
    this: Ptr<Actor>,
    attacker: Ptr,
    minor: bool,
    _unused_2: u32,
) {
    let player = e.global::<u32>(PLAYER_POINTER);
    if attacker.addr() != player || e.vcall(this.addr(), 0x320, &args![]).bool() {
        return;
    }
    let attacker_actor = cast_to_actor(e, attacker.addr());
    if e.call(IS_IN_COMBATANT_FACTION, &args![this]).bool()
        && attacker_actor != 0
        && e.call(IS_IN_COMBATANT_FACTION, &args![attacker_actor])
            .bool()
    {
        return;
    }
    if e.vcall(attacker_actor, 0x304, &args![]).bool() {
        return;
    }
    let crime = new_crime(e, 3, [this.addr(), attacker.addr(), 0, 0, 0]);
    fn_008bff70(e, Ptr::new(player), 3, 1);
    let witnesses = e
        .call(PROCESS_LISTS_WITNESSES, &args![PROCESS_LISTS, crime])
        .u32();
    let hour = calendar_hour(e);
    let extra_list = e.call(EXTRA_LIST_OF_REFERENCE, &args![this]).u32();
    e.call(ADD_TO_PLAYER_CRIME_LIST, &args![extra_list, crime, hour]);
    e.call(CRIME_ADD_WITNESS, &args![crime, this]);
    e.call(PROCESS_LISTS_ADD_CRIME, &args![PROCESS_LISTS, crime]);
    let mut node = witnesses;
    if node != 0 {
        while node != 0 {
            let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
            if e.mem.u32(slot) == 0 {
                break;
            }
            let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
            let witness = e.mem.u32(slot);
            if !e.vcall(witness, 0x22c, &args![0u32]).bool()
                && !e.vcall(witness, 0x320, &args![]).bool()
            {
                let angry = if minor {
                    witness == this.addr()
                } else {
                    e.call(
                        SET_FACTIONS_THAT_CARE_ABOUT_CRIME,
                        &args![witness, this, crime],
                    )
                    .bool()
                };
                if angry {
                    let topic = begin_witness_line(e, this.addr(), crime, witness, 0xd);
                    witness_says(e, witness, topic);
                    e.call(CRIME_ADD_WITNESS, &args![crime, witness]);
                    fn_008bff30(e, 0);
                } else {
                    let topic = begin_witness_line(e, this.addr(), crime, witness, 0xf);
                    let crime_word_8 = e.call(WORD_AT_8, &args![crime]).u32();
                    let is_victim = this.addr() == crime_word_8;
                    let crime_reference = e.call(RECORD_REFERENCE, &args![crime]).u32();
                    let attack = crime_reference != 0 && {
                        let target = e.call(RECORD_REFERENCE, &args![crime]).u32();
                        e.with_stack(4, |e, out| {
                            e.mem.set_u32(out.addr(), 0);
                            e.call(
                                GET_SHOULD_ATTACK_ACTOR,
                                &args![witness, target, 0u32, out, 0u32],
                            )
                            .bool()
                        })
                    };
                    if attack {
                        let witness_process = process_of(e, witness);
                        let target = e.call(RECORD_REFERENCE, &args![crime]).u32();
                        e.vcall(
                            witness_process,
                            0x33c,
                            &args![
                                witness, target, is_victim, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                                0u32, 0u32, 1u32, 0u32
                            ],
                        );
                    } else {
                        witness_says(e, witness, topic);
                    }
                    fn_008bff30(e, 0);
                }
            }
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }
        destroy_list(e, witnesses);
        if e.call(CRIME_WITNESS_COUNT, &args![crime]).i32() == 0 {
            let attacker_name = e.call(REFERENCE_NAME, &args![attacker]).u32();
            let victim_name = e.call(REFERENCE_NAME, &args![this]).u32();
            e.with_stack(0x134, |e, buffer| {
                e.call(
                    SPRINTF,
                    &args![buffer, NO_ONE_CARED_FORMAT, victim_name, attacker_name],
                );
                e.call(LOG_MESSAGE, &args![buffer]);
            });
        } else if !minor {
            fn_008c09b0(e, Ptr::new(player));
            e.call(ADD_FACTION_MAJOR_CRIME, &args![this, 1u32, 0u32]);
        } else {
            e.call(ADD_FACTION_MINOR_CRIME, &args![this, 1u32, 0u32]);
        }
    }
    destroy_list(e, node);
}

// Translated from 008c09b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Counts one more major crime: stores the major crime count plus one in the
/// low 30 bits of `iMajorCrimes`, keeping its two flag bits.
pub fn fn_008c09b0(e: &mut Engine, this: Ptr<Actor>) {
    let count = e.call(MAJOR_CRIME_COUNT, &args![this]).u32();
    let flags = e.get(this, Actor::iMajorCrimes) & 0xc000_0000;
    e.set(this, Actor::iMajorCrimes, count.wrapping_add(1) | flags);
}

// Translated from 008c09e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A crime of kind 4 by `other` against `this`: builds the `Crime`, bumps the
/// player's counter 4, reports it and lets each living witness react (an
/// angry line, help, attack or a plain line). Only the player, or an actor
/// whose process wants an update next, can commit it.
pub fn fn_008c09e0(e: &mut Engine, this: Ptr<Actor>, other: Ptr) {
    if other.is_null() {
        return;
    }
    let player = e.global::<u32>(PLAYER_POINTER);
    let actor = cast_to_actor(e, other.addr());
    let mut forced = false;
    if actor != 0 && e.call(GET_FORCE_NEXT_UPDATE, &args![actor]).bool() {
        forced = true;
    }
    if other.addr() != player && !forced {
        return;
    }
    if e.call(IS_IN_COMBATANT_FACTION, &args![this]).bool()
        && actor != 0
        && e.call(IS_IN_COMBATANT_FACTION, &args![actor]).bool()
    {
        return;
    }
    if e.vcall(actor, 0x304, &args![]).bool() {
        return;
    }
    if actor != 0
        && actor != player
        && e.vcall(actor + 0xa4, 8, &args![0x2au32]).i32() >= 100
        && e.call(ACTOR_FLAG_TEST, &args![actor]).bool()
    {
        return;
    }
    let crime = new_crime(e, 4, [this.addr(), other.addr(), 0, 0, 0]);
    fn_008bff70(e, Ptr::new(player), 4, 1);
    let hour = calendar_hour(e);
    let extra_list = e.call(EXTRA_LIST_OF_REFERENCE, &args![this]).u32();
    e.call(ADD_TO_PLAYER_CRIME_LIST, &args![extra_list, crime, hour]);
    e.call(PROCESS_LISTS_ADD_CRIME, &args![PROCESS_LISTS, crime]);
    e.call(CRIME_ADD_WITNESS, &args![crime, this]);
    e.call(
        SET_FACTIONS_THAT_CARE_ABOUT_CRIME,
        &args![this, this, crime],
    );
    let witnesses = e
        .call(PROCESS_LISTS_WITNESSES, &args![PROCESS_LISTS, crime])
        .u32();
    let mut node = witnesses;
    if node != 0 {
        while node != 0 {
            let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
            if e.mem.u32(slot) == 0 {
                break;
            }
            let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
            let witness = e.mem.u32(slot);
            if !e.vcall(witness, 0x22c, &args![0u32]).bool()
                && !e.vcall(witness, 0x320, &args![]).bool()
            {
                e.call(CRIME_ADD_WITNESS, &args![crime, this]);
                if e.call(
                    SET_FACTIONS_THAT_CARE_ABOUT_CRIME,
                    &args![witness, this, crime],
                )
                .bool()
                {
                    let topic = begin_witness_line(e, this.addr(), crime, witness, 0xe);
                    witness_says(e, witness, topic);
                    e.call(CRIME_ADD_WITNESS, &args![crime, witness]);
                    if e.call(GET_SHOULD_HELP, &args![witness, this]).bool()
                        && !e.call(IN_COMBAT_FLAG, &args![witness]).bool()
                    {
                        let target = e.call(RECORD_REFERENCE, &args![crime]).u32();
                        witness_reacts(e, witness, target);
                    }
                    fn_008bff30(e, 0);
                } else {
                    let topic = begin_witness_line(e, this.addr(), crime, witness, 0x10);
                    let crime_reference = e.call(RECORD_REFERENCE, &args![crime]).u32();
                    let attack = crime_reference != 0 && {
                        let target = e.call(RECORD_REFERENCE, &args![crime]).u32();
                        e.with_stack(4, |e, out| {
                            e.mem.set_u32(out.addr(), 0);
                            e.call(
                                GET_SHOULD_ATTACK_ACTOR,
                                &args![witness, target, 0u32, out, 0u32],
                            )
                            .bool()
                        })
                    };
                    if attack
                        || (e.call(GET_SHOULD_HELP, &args![witness, this]).bool()
                            && !e.call(IN_COMBAT_FLAG, &args![witness]).bool())
                    {
                        let target = e.call(RECORD_REFERENCE, &args![crime]).u32();
                        witness_reacts(e, witness, target);
                    } else {
                        witness_says(e, witness, topic);
                    }
                    fn_008bff30(e, 0);
                }
            }
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }
        destroy_list(e, witnesses);
        if e.call(CRIME_WITNESS_COUNT, &args![crime]).i32() > 0 {
            fn_008c09b0(e, Ptr::new(player));
            e.call(ADD_FACTION_MAJOR_CRIME, &args![this, 1u32, 1u32]);
        }
    }
    destroy_list(e, node);
}

// Translated from 008c0ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under the alarm lock, runs through the actors of the process lists: an
/// actor of the faction or form (`argument_a`, `argument_b` as for
/// [`is_actor_of_faction`]) that is not dead and whose level toward the player
/// is above 0 is told to attack the player (slot `0x33c` of its process), and
/// the first such actor is charged one minor faction crime; the player's
/// `iMinorCrimes` is bumped once. Returns `0xffffffff`.
pub fn fn_008c0ec0(
    e: &mut Engine,
    _this: u32,
    argument_a: Ptr,
    argument_b: Ptr,
    _unused_2: u32,
) -> u32 {
    e.call(LOCK_TAKE, &args![BROADCAST_LOCK, 0u32]);
    let mut charged = false;
    let list = e
        .call(PROCESS_LISTS_ACTOR_ARRAY, &args![PROCESS_LISTS])
        .u32();
    let count = e
        .call(PROCESS_LISTS_ACTOR_COUNT, &args![PROCESS_LISTS])
        .i32();
    let player = e.global::<u32>(PLAYER_POINTER);
    let mut index = 0;
    while index < count {
        let actor = e.mem.u32(list.wrapping_add(index as u32 * 4));
        e.call(LEVEL_TOWARD_ACTOR, &args![actor, player]);
        is_actor_of_faction(e, argument_a, argument_b, Ptr::new(actor));
        if !e.vcall(actor, 0x320, &args![]).bool() {
            let level = e.call(LEVEL_TOWARD_ACTOR, &args![actor, player]).i32();
            if level > 0 && is_actor_of_faction(e, argument_a, argument_b, Ptr::new(actor)) {
                let process = process_of(e, actor);
                e.vcall(
                    process,
                    0x33c,
                    &args![
                        actor, player, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32,
                        0u32
                    ],
                );
                if !charged {
                    e.call(ADD_FACTION_MINOR_CRIME, &args![actor, 1u32, 1u32]);
                    charged = true;
                }
            }
        }
        index += 1;
    }
    if charged {
        fn_008bff50(e, Ptr::new(player));
    }
    e.call(LOCK_RELEASE, &args![BROADCAST_LOCK]);
    0xffff_ffff
}

// Translated from 008c1010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IsActorOfFaction` (Xbox PDB), `cdecl`: whether `actor` is in the faction
/// `form` (a faction, or an `NPC_` whose faction list `005d8a70` yields) or,
/// failing that, in a faction of `reference` (an actor: its integrated faction
/// lists; any other `NPC_`-typed reference: its base's list).
pub fn is_actor_of_faction(e: &mut Engine, reference: Ptr, form: Ptr, actor: Ptr) -> bool {
    if !form.is_null() {
        let form_type = e.call(FORM_TYPE, &args![form]).u32();
        if form_type == 0x2a {
            let list = e.call(FACTION_LIST_OF, &args![form.addr() + 0x30]).u32();
            if list != 0 && fn_008c1210(e, Ptr::new(list), actor.cast(), NAME_STRING_NPC_ID) {
                return true;
            }
        } else if e.call(FORM_TYPE, &args![form]).u32() == 8
            && e.call(IS_IN_FACTION, &args![actor, form]).bool()
        {
            return true;
        }
    }
    if !reference.is_null() {
        let as_actor = cast_to_actor(e, reference.addr());
        if as_actor != 0 {
            let base = e.call(GET_BASE_FORM, &args![as_actor]).u32();
            let faction_list = e.call(FACTION_LIST_OF, &args![base + 0x30]).u32();
            let extra_list = e.call(EXTRA_LIST_OF_REFERENCE, &args![as_actor]).u32();
            let changes = e.call(GET_FACTION_CHANGES_EXTRA, &args![extra_list]).u32();
            let changes_word = if changes == 0 {
                0
            } else {
                e.mem.u32(changes + 0xc)
            };
            return e.with_stack(0x200, |e, buffer| {
                let count = e
                    .call(
                        INTEGRATE_FACTION_LISTS,
                        &args![as_actor, buffer, 0x80u32, faction_list, changes_word],
                    )
                    .u32();
                for i in 0..count {
                    let faction = e.mem.u32(buffer.addr() + i * 4);
                    if e.call(IS_IN_FACTION, &args![actor, faction]).bool() {
                        return true;
                    }
                }
                false
            });
        }
        if e.call(FORM_TYPE, &args![reference]).u32() == 0x2a {
            let npc = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![reference, 0u32, CAST_FROM_REFERENCE, CAST_TO_NPC, 0u32],
                )
                .u32();
            if npc != 0 {
                let list = e.call(FACTION_LIST_OF, &args![npc + 0x30]).u32();
                if list != 0
                    && fn_008c1210(
                        e,
                        Ptr::new(list),
                        actor.cast(),
                        NAME_STRING_REFERENCE_NPC_ID,
                    )
                {
                    return true;
                }
            }
        }
    }
    false
}

// Translated from 008c1210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl`, three words: walks the list from node `node` and returns true as
/// soon as `actor` is in the faction held by an entry (the faction is the word
/// the entry's item points to). `_unused_2` is a name string the callers
/// push. The game leaves `AL` undefined when the walk runs out; this returns
/// false.
pub fn fn_008c1210(e: &mut Engine, mut node: Ptr, actor: Ptr<Actor>, _unused_2: u32) -> bool {
    while !node.is_null() {
        let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            return false;
        }
        let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
        let item = e.mem.u32(slot);
        if item != 0 {
            let faction = e.mem.u32(item);
            if e.call(IS_IN_FACTION, &args![actor, faction]).bool() {
                return true;
            }
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).ptr();
    }
    false
}

// Translated from 008c1270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Refreshes the actor's process-dependent state: with a process, calls
/// `008676d0` on the `CachedValuesOwner`; calls slot `0x20c`; and for a process
/// of type 0 reads the 0x4d actor values (slot `0xc` of the `ActorValueOwner`,
/// results dropped) and calls slot `0x408` of the process for 3 up to 15.
pub fn fn_008c1270(e: &mut Engine, this: Ptr<Actor>) {
    if !e.get(this, Actor::pCurrentProcess).is_null() {
        e.call(CACHED_VALUES_OWNER_REFRESH, &args![this.addr() + 0xa8]);
    }
    e.vcall(this.addr(), 0x20c, &args![]);
    if e.call(GET_CURRENT_PROCESS_TYPE, &args![this]).u32() == 0 {
        for index in 0..0x4du32 {
            e.vcall(this.addr() + 0xa4, 0xc, &args![index]);
        }
        for index in 3..0x10u32 {
            let process = process_of(e, this.addr());
            e.vcall(process, 0x408, &args![index]);
        }
    }
}

/// Forgets the two combat arrays: destroys each through slot 0 of its vtable
/// (with the delete flag) when set, and clears the pointers.
fn drop_combat_arrays(e: &mut Engine, this: Ptr<Actor>) {
    let target_array = e.get(this, Actor::pCurrentCombatTargetArray);
    if !target_array.is_null() {
        e.vcall(target_array.addr(), 0, &args![1u32]);
    }
    e.set(this, Actor::pCurrentCombatTargetArray, Ptr::NULL);
    let member_array = e.get(this, Actor::pCurrentCombatMemberArray);
    if !member_array.is_null() {
        e.vcall(member_array.addr(), 0, &args![1u32]);
    }
    e.set(this, Actor::pCurrentCombatMemberArray, Ptr::NULL);
}

// Translated from 008c1320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forgets the actor's killer, disposition modifiers and combat target, and
/// destroys the two combat arrays (`pCurrentCombatTargetArray`,
/// `pCurrentCombatMemberArray`) through slot 0 of their vtables.
pub fn fn_008c1320(e: &mut Engine, this: Ptr<Actor>) {
    e.set(this, Actor::pMyKiller, Ptr::NULL);
    e.call(CLEAR_DISPOSITION_MODIFIERS, &args![this]);
    e.set(this, Actor::pCurrentCombatTarget, Ptr::NULL);
    drop_combat_arrays(e, this);
}

// Translated from 008c13d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsTryingToEnterFurniture` (Xbox PDB): true when slot `0x214` is none
/// of 0, 4 and 9 and the object that slot `0x4d4` of the process returns has a
/// byte at `+0xe` of at most `0x14`.
pub fn actor_is_trying_to_enter_furniture(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let mut result = false;
    if e.vcall(this.addr(), 0x214, &args![]).u32() != 0
        && e.vcall(this.addr(), 0x214, &args![]).u32() != 4
        && e.vcall(this.addr(), 0x214, &args![]).u32() != 9
    {
        let process = process_of(e, this.addr());
        if e.vcall(process, 0x4d4, &args![]).u32() != 0 {
            let process = process_of(e, this.addr());
            let object = e.vcall(process, 0x4d4, &args![]).u32();
            if e.mem.u8(object + 0xe) <= 0x14 {
                result = true;
            }
        }
    }
    result
}

/// A new `BSSimpleArray<Actor *, 1024>` (`operator new(0x10)` and `008c1c00`),
/// or null when the allocation fails.
fn new_actor_array(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    if block != 0 {
        fn_008c1c00(e, Ptr::new(block)).addr()
    } else {
        0
    }
}

// Translated from 008c1470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Refreshes the actor's combat links. In combat, takes the combat controller
/// from slot `0x428` and copies its word `+0xc0` to `pCurrentCombatTarget` and
/// its flag to `bSearchingInCombat`; when the controller has a group, creates
/// the two `BSSimpleArray<Actor *, 1024>` objects if missing and fills them
/// from the group (`00986760`, `00986b00`). Out of combat clears all of it.
pub fn fn_008c1470(e: &mut Engine, this: Ptr<Actor>) {
    if e.call(IN_COMBAT_FLAG, &args![this]).bool() {
        let controller = e.vcall(this.addr(), 0x428, &args![]).u32();
        if controller != 0 {
            let value = e.call(COMBAT_CONTROLLER_GET_WORD, &args![controller]).u32();
            e.set(this, Actor::pCurrentCombatTarget, Ptr::new(value));
            let flag = e
                .call(COMBAT_CONTROLLER_GET_FLAG, &args![controller])
                .bool();
            e.set(this, Actor::bSearchingInCombat, flag);
            if e.call(COMBAT_CONTROLLER_GET_GROUP, &args![controller])
                .u32()
                != 0
            {
                if e.get(this, Actor::pCurrentCombatTargetArray).is_null() {
                    let array = new_actor_array(e);
                    e.set(this, Actor::pCurrentCombatTargetArray, Ptr::new(array));
                }
                let array = e.get(this, Actor::pCurrentCombatTargetArray).addr();
                let group = e
                    .call(COMBAT_CONTROLLER_GET_GROUP, &args![controller])
                    .u32();
                e.call(COMBAT_GROUP_FILL_TARGET_ARRAY, &args![group, array]);
                if e.get(this, Actor::pCurrentCombatMemberArray).is_null() {
                    let array = new_actor_array(e);
                    e.set(this, Actor::pCurrentCombatMemberArray, Ptr::new(array));
                }
                let array = e.get(this, Actor::pCurrentCombatMemberArray).addr();
                let group = e
                    .call(COMBAT_CONTROLLER_GET_GROUP, &args![controller])
                    .u32();
                e.call(COMBAT_GROUP_FILL_MEMBER_ARRAY, &args![group, array]);
            }
        }
    } else {
        e.set(this, Actor::bSearchingInCombat, false);
        e.set(this, Actor::pCurrentCombatTarget, Ptr::NULL);
        drop_combat_arrays(e, this);
    }
}

// Translated from 008c1680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `value` is among the entries of the combat member array
/// (`pCurrentCombatMemberArray`): searches it with the comparison `009a3830`
/// (`00719b20`; the key is passed by address).
pub fn fn_008c1680(e: &mut Engine, this: Ptr<Actor>, value: u32) -> bool {
    let array = e.get(this, Actor::pCurrentCombatMemberArray);
    if array.is_null() {
        return false;
    }
    let index = e.with_stack(4, |e, key| {
        e.mem.set_u32(key.addr(), value);
        e.call(
            ARRAY_FIND_WITH_COMPARE,
            &args![array, key, 0u32, ARRAY_COMPARE_CALLBACK],
        )
        .i32()
    });
    index != -1
}

/// The script and variable list of a reference, and the script variable
/// numbered `variable` in it (0 when missing): shared by the two script
/// variable accessors.
fn find_script_variable(e: &mut Engine, this: Ptr, variable: u32) -> Option<(u32, u32)> {
    if this.is_null() {
        return None;
    }
    let form = e.call(GET_FORM, &args![this]).u32();
    let script = e.call(SCRIPT_OF_FORM, &args![form]).u32();
    let variables = e.call(GET_SCRIPT_VARIABLES, &args![this]).u32();
    if script == 0 || variables == 0 {
        return None;
    }
    let found = e.with_stack(4, |e, out| {
        e.mem.set_u32(out.addr(), 0);
        e.call(SCRIPT_FIND_VARIABLE, &args![script, variable, out]);
        e.mem.u32(out.addr())
    });
    if found == 0 {
        return None;
    }
    Some((variables, found))
}

// Translated from 008c16c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the script variable numbered `variable` of the reference to `value`.
/// False for a null reference, a form without script or variables, or an
/// unknown variable.
pub fn fn_008c16c0(e: &mut Engine, this: Ptr, variable: u32, value: f64) -> bool {
    match find_script_variable(e, this, variable) {
        Some((variables, found)) => {
            e.call(SCRIPT_LOCALS_SET_VARIABLE, &args![variables, found, value]);
            true
        }
        None => false,
    }
}

// Translated from 008c1740 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value of script variable `variable` of the reference, or `-1.0` when
/// the reference is null or has no such variable.
pub fn fn_008c1740(e: &mut Engine, this: Ptr, variable: u32) -> f64 {
    match find_script_variable(e, this, variable) {
        Some((variables, found)) => e
            .call(SCRIPT_LOCALS_GET_VARIABLE, &args![variables, found, 0u32])
            .f64(),
        None => e.global::<f64>(DOUBLE_MINUS_ONE),
    }
}

/// The shared body of `008c17c0` and `008c1940`: the animation speed is 1 plus
/// (actor value `0xa` minus a setting) times another setting, passed through
/// the perk entry point `entry_point`, and handed to `setter` for the animation
/// objects (the player's two, or the one slot `0x1e4` of the actor gives).
fn apply_animation_speed(
    e: &mut Engine,
    this: Ptr<Actor>,
    entry_point: u32,
    setter: fn(&mut Engine, Ptr, f32),
) {
    let skill = e.vcall(this.addr() + 0xa4, 0xc, &args![10u32]).f32();
    let base = setting_float(e, SETTING_SPEED_BASE);
    let difference = (skill as f64 - base as f64) as f32;
    let factor = setting_float(e, SETTING_SPEED_FACTOR);
    let value = (difference as f64 * factor as f64 + 1.0) as f32;
    let mut perk_argument = 0u32;
    if process_of(e, this.addr()) != 0 {
        let process = process_of(e, this.addr());
        if e.vcall(process, 0x148, &args![]).u32() != 0 {
            let process = process_of(e, this.addr());
            let entry = e.vcall(process, 0x148, &args![]).u32();
            perk_argument = e.call(WORD_AT_8, &args![entry]).u32();
        }
    }
    let value = e.with_stack(4, |e, slot| {
        e.mem.set_f32(slot.addr(), value);
        e.call(
            HANDLE_ENTRY_POINT,
            &args![entry_point, this, perk_argument, slot],
        );
        e.mem.f32(slot.addr())
    });
    let player = e.global::<u32>(PLAYER_POINTER);
    if this.addr() == player {
        for which in [1u32, 0u32] {
            if e.call(PLAYER_GET_ANIMATION, &args![player, which]).u32() != 0 {
                let animation = e.call(PLAYER_GET_ANIMATION, &args![player, which]).u32();
                setter(e, Ptr::new(animation), value);
            }
        }
    } else if e.vcall(this.addr(), 0x1e4, &args![]).u32() != 0 {
        let animation = e.vcall(this.addr(), 0x1e4, &args![]).u32();
        setter(e, Ptr::new(animation), value);
    }
}

// Translated from 008c17c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the animation speed of the actor's animation objects from the perk
/// entry point `0x25` (see [`apply_animation_speed`]); the value goes to
/// `+0x118` of each object.
pub fn fn_008c17c0(e: &mut Engine, this: Ptr<Actor>) {
    apply_animation_speed(e, this, 0x25, fn_008c1920);
}

// Translated from 008c1920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the `float` `value` at `+0x118` of an animation object.
pub fn fn_008c1920(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x118, x87_quiet(value));
}

// Translated from 008c1940 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same as [`fn_008c17c0`] for the perk entry point `0x26`; the value goes
/// to `+0x11c` of each object (through `008c1aa0`).
pub fn fn_008c1940(e: &mut Engine, this: Ptr<Actor>) {
    apply_animation_speed(e, this, 0x26, fn_008c1aa0);
}

// Translated from 008c1aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the `float` `value` at `+0x11c` of an animation object.
pub fn fn_008c1aa0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x11c, x87_quiet(value));
}

// Translated from 008c1ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears bit `0x80000000` of `iMajorCrimes`.
pub fn fn_008c1ac0(e: &mut Engine, this: Ptr<Actor>, set: bool) {
    let word = e.get(this, Actor::iMajorCrimes);
    let word = if set {
        word | 0x8000_0000
    } else {
        word & 0x7fff_ffff
    };
    e.set(this, Actor::iMajorCrimes, word);
}

// Translated from 008c1b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `0x80000000` of `iMajorCrimes` is set.
pub fn fn_008c1b30(e: &mut Engine, this: Ptr<Actor>) -> bool {
    e.get(this, Actor::iMajorCrimes) & 0x8000_0000 != 0
}

// Translated from 008c1b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears bit `0x40000000` of `iMajorCrimes`.
pub fn fn_008c1b50(e: &mut Engine, this: Ptr<Actor>, set: bool) {
    let word = e.get(this, Actor::iMajorCrimes);
    let word = if set {
        word | 0x4000_0000
    } else {
        word & 0xbfff_ffff
    };
    e.set(this, Actor::iMajorCrimes, word);
}

// Translated from 008c1bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::AlwaysShowSubtitles` (Xbox PDB): bit `0x40000000` of `iMajorCrimes`.
pub fn actor_always_show_subtitles(e: &mut Engine, this: Ptr<Actor>) -> bool {
    e.get(this, Actor::iMajorCrimes) & 0x4000_0000 != 0
}

// Translated from 008c1be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `this` ANDed with `mask`.
pub fn fn_008c1be0(e: &mut Engine, this: Ptr, mask: u32) -> u32 {
    e.mem.u32(this.addr()) & mask
}

// Translated from 008c1c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `BSSimpleArray<Actor *, 1024>` (the type of the vtable it
/// installs): sets the vtable and initialises the array with `006b3eb0(0, 0)`.
/// Returns `this`.
pub fn fn_008c1c00(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), SIMPLE_ARRAY_VTABLE);
    e.call(SIMPLE_ARRAY_INITIALISE, &args![this, 0u32, 0u32]);
    this
}

// Translated from 008c1c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointer<HitData>` constructor from a raw pointer: stores `pointer` and
/// adds a reference when it is not null. Returns `this`.
pub fn fn_008c1c30(e: &mut Engine, this: Ptr, pointer: u32) -> Ptr {
    e.mem.set_u32(this.addr(), pointer);
    if pointer != 0 {
        e.call(HIT_DATA_ADD_REFERENCE, &args![pointer]);
    }
    this
}

// Translated from 008c1c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPointer<HitData>::~NiPointer<HitData>` (Xbox PDB): releases the held
/// object when it is not null.
pub fn ni_pointer_hit_data_destructor(e: &mut Engine, this: Ptr) {
    let held = e.mem.u32(this.addr());
    if held != 0 {
        e.call(HIT_DATA_RELEASE, &args![held]);
    }
}

// Translated from 008c1c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<Actor_P_1024>::_scalar_deleting_destructor_` (Xbox PDB): runs
/// the destructor body `008c1cb0` and frees the block when bit 0 of `flags` is
/// set. Returns `this`.
pub fn bs_simple_array_actor_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(SIMPLE_ARRAY_DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008cfad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::FakeWeaponHitSound` (Xbox PDB): when the actor has a loaded 3D
/// object (virtual `+0x1d0`) and its word at `+0x88` points at a record whose
/// first word is 1, finds the first collision object under it and, if that has
/// a target, plays an impact sound described by a 0x24-byte request on the stack.
#[allow(clippy::too_many_arguments)]
pub fn actor_fake_weapon_hit_sound(
    e: &mut Engine,
    this: Ptr<Actor>,
    strength: f32,
    position_x: u32,
    position_y: u32,
    position_z: u32,
    offset: u32,
    flag: u8,
) {
    let node = e.vcall(this.addr(), 0x1d0, &args![]).u32();
    if node == 0 {
        return;
    }
    let record = e.mem.u32(this.addr() + 0x88);
    if record == 0 || e.mem.u32(record) != 1 {
        return;
    }
    let collision = e.call(FIND_FIRST_COLLISION_OBJECT, &args![node]).u32();
    let target = if collision != 0 {
        e.call(COLLISION_OBJECT_GET_TARGET, &args![collision]).u32()
    } else {
        0
    };
    if target == 0 {
        return;
    }
    e.with_stack(0x24, |e, request| {
        let base = request.addr();
        e.call(HIT_SOUND_REQUEST_CONSTRUCTOR, &args![base]);
        let half = e.global::<f32>(HALF_FLOAT);
        e.mem.set_f32(base + 0x10, half);
        e.mem.set_u32(base + 0x1c, target);
        e.mem.set_u32(base + 0x20, offset);
        let object = e.call(TARGET_OBJECT_GET_OBJECT, &args![target]).u32();
        let kind = e.call(OBJECT_GET_KIND, &args![object]).u32() as u8;
        e.mem.set_u8(base + 0x14, kind);
        e.mem.set_u8(base + 0x15, flag);
        e.mem.set_u32(base, position_x);
        e.mem.set_u32(base + 4, position_y);
        e.mem.set_u32(base + 8, position_z);
        e.mem.set_f32(base + 0xc, strength);
        e.mem.set_u32(base + 0x18, this.addr().wrapping_add(offset));
        e.call(IMPACT_MIXER_PLAY_COLLISION_SOUND, &args![base]);
    });
}

// Translated from 008e2680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::DoesFly` (Xbox PDB name on the map; the callers `Move`,
/// `LoadCharController` and `ProcessFollow` pass a process object, so the
/// name is a folded one): stores the word at `+0x3f0`.
pub fn fn_008e2680(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x3f0, value);
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008bec80, fn_008bec80(Ptr<Actor>) -> f32),
        entry!(0x008becf0, fn_008becf0(Ptr, Ptr) -> f32),
        entry!(0x008bed50, fn_008bed50(Ptr<Actor>, Ptr, u32, bool, Ptr)),
        entry!(0x008bf4e0, fn_008bf4e0(Ptr, u8)),
        entry!(0x008bf500, fn_008bf500(Ptr<Actor>, Ptr)),
        entry!(0x008bf7b0, fn_008bf7b0(Ptr<Actor>, Ptr, u32)),
        entry!(
            0x008bfa40,
            actor_steal_alarm(Ptr<Actor>, Ptr, u32, u32, u32, u32)
        ),
        entry!(0x008bff30, fn_008bff30(u32)),
        entry!(0x008bff50, fn_008bff50(Ptr<Actor>)),
        entry!(0x008bff70, fn_008bff70(Ptr, u32, i32)),
        entry!(0x008bffa0, fn_008bffa0(Ptr) -> u32),
        entry!(0x008bffc0, actor_is_angry_with_player(Ptr<Actor>) -> bool),
        entry!(0x008c0050, fn_008c0050(Ptr<Actor>) -> bool),
        entry!(0x008c00e0, fn_008c00e0(Ptr<Actor>, Ptr, u32, u32)),
        entry!(0x008c0460, actor_attack_alarm(Ptr<Actor>, Ptr, bool, u32)),
        entry!(0x008c09b0, fn_008c09b0(Ptr<Actor>)),
        entry!(0x008c09e0, fn_008c09e0(Ptr<Actor>, Ptr)),
        entry!(0x008c0ec0, fn_008c0ec0(u32, Ptr, Ptr, u32) -> u32),
        entry!(0x008c1010, is_actor_of_faction(Ptr, Ptr, Ptr) -> bool),
        entry!(0x008c1210, fn_008c1210(Ptr, Ptr<Actor>, u32) -> bool),
        entry!(0x008c1270, fn_008c1270(Ptr<Actor>)),
        entry!(0x008c1320, fn_008c1320(Ptr<Actor>)),
        entry!(
            0x008c13d0,
            actor_is_trying_to_enter_furniture(Ptr<Actor>) -> bool
        ),
        entry!(0x008c1470, fn_008c1470(Ptr<Actor>)),
        entry!(0x008c1680, fn_008c1680(Ptr<Actor>, u32) -> bool),
        entry!(0x008c16c0, fn_008c16c0(Ptr, u32, f64) -> bool),
        entry!(0x008c1740, fn_008c1740(Ptr, u32) -> f64),
        entry!(0x008c17c0, fn_008c17c0(Ptr<Actor>)),
        entry!(0x008c1920, fn_008c1920(Ptr, f32)),
        entry!(0x008c1940, fn_008c1940(Ptr<Actor>)),
        entry!(0x008c1aa0, fn_008c1aa0(Ptr, f32)),
        entry!(0x008c1ac0, fn_008c1ac0(Ptr<Actor>, bool)),
        entry!(0x008c1b30, fn_008c1b30(Ptr<Actor>) -> bool),
        entry!(0x008c1b50, fn_008c1b50(Ptr<Actor>, bool)),
        entry!(0x008c1bc0, actor_always_show_subtitles(Ptr<Actor>) -> bool),
        entry!(0x008c1be0, fn_008c1be0(Ptr, u32) -> u32),
        entry!(0x008c1c00, fn_008c1c00(Ptr) -> Ptr),
        entry!(0x008c1c30, fn_008c1c30(Ptr, u32) -> Ptr),
        entry!(0x008c1c60, ni_pointer_hit_data_destructor(Ptr)),
        entry!(
            0x008c1c80,
            bs_simple_array_actor_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x008cfad0,
            actor_fake_weapon_hit_sound(Ptr<Actor>, f32, u32, u32, u32, u32, u8)
        ),
        entry!(0x008e2680, fn_008e2680(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    static NEXT_TARGET: AtomicU32 = AtomicU32::new(0x0300_0000);

    /// Every external function the translations call; each answers zero until
    /// a test says otherwise.
    const CALLEES: &[u32] = &[
        GET_BASE_FORM,
        RT_DYNAMIC_CAST,
        OPERATOR_DELETE,
        ADD_TO_PLAYER_CRIME_LIST,
        GET_PLAYER_CRIME_LIST,
        GET_PACKAGE_EXTRA,
        GET_FACTION_CHANGES_EXTRA,
        FACTION_LIST_OF,
        FLOAT_TO_INT,
        SPRINTF,
        LOCK_TAKE,
        LOCK_RELEASE,
        IS_FLEEING,
        IS_IN_COMBATANT_FACTION,
        IS_IN_FACTION,
        INTEGRATE_FACTION_LISTS,
        ADD_FACTION_MINOR_CRIME,
        ADD_FACTION_MAJOR_CRIME,
        SET_FACTIONS_THAT_CARE_ABOUT_CRIME,
        GET_SHOULD_ATTACK_ACTOR,
        GET_SHOULD_HELP,
        GET_DETECTION_LEVEL,
        ACTOR_GET_CLASS,
        CLASS_IS_GUARD,
        GET_PACKAGE_SET_AS_CURRENT,
        END_INTERRUPT_PACKAGE,
        IS_RUNNING_RUN_ONCE,
        CLEAR_DISPOSITION_MODIFIERS,
        GET_CURRENT_PROCESS_TYPE,
        GET_CURRENT_PACKAGE,
        ACTOR_FLAG_TEST,
        GET_CLAMPED_ACTOR_VALUE,
        GET_FORCE_NEXT_UPDATE,
        GET_OWNER,
        IS_AN_OWNER,
        GET_REF_PERSISTS,
        REFERENCE_NAME,
        SET_WORD_AT_0X70,
        GET_TOPIC,
        GET_FORM_WEIGHT,
        WEIGHT_FLAG_OF_PLAYER,
        BASE_FORM_FLAG_BIT_1,
        GET_ALIGNMENT_FOR_KARMA,
        REWARD_KARMA,
        GET_PLAYER_ACTION,
        PLAYER_BYTE_AT_0X7C4,
        PLAYER_WORD_AT_0X230,
        PLAYER_GET_ANIMATION,
        PLAYER_ALARM_COUNTER_23,
        PLAYER_ALARM_COUNTER_22,
        HANDLE_ENTRY_POINT,
        RECORD_BYTE_AT_0X10,
        LIST_CLEAR,
        LIST_DESTROY,
        LIST_IS_EMPTY,
        PROCESS_LISTS_GET_ACTOR_REF_IN_HIGH,
        PROCESS_LISTS_WITNESSES,
        PROCESS_LISTS_FIND_ACTOR_REF,
        PROCESS_LISTS_FIND_ENTRY,
        ENTRY_COUNT,
        PROCESS_LISTS_ADD_CRIME,
        PROCESS_LISTS_ACTOR_ARRAY,
        PROCESS_LISTS_ACTOR_COUNT,
        LEVEL_TOWARD_ACTOR,
        CRIME_WITNESS_COUNT,
        CRIME_ADD_WITNESS,
        CRIME_CONSTRUCTOR,
        CRIME_HAS_WITNESS,
        CRIME_APPEND_WITNESS,
        ALARM_PACKAGE_CONSTRUCTOR,
        FLEE_PACKAGE_ADD_AVOIDED_REF,
        LOG_MESSAGE,
        CREATE_PACKAGE,
        SET_PACK_TYPE,
        SET_PACKAGE_LOCATION,
        SET_PACKAGE_TARGET,
        PACKAGE_TARGET_OBJECT,
        IS_INTERRUPT_PACKAGE,
        PACKAGE_SET_FLAG_800000,
        PACKAGE_GET_FLAG_800000,
        PACKAGE_SET_FLAG_200000,
        PACKAGE_GET_FLAG_200000,
        PACKAGE_GET_FLAG_400000,
        PACKAGE_SET_FLAG_2,
        PACKAGE_SET_FLAG_4,
        PACKAGE_LOCATION_CONSTRUCTOR,
        SET_LOC_REFERENCE,
        PACKAGE_LOCATION_DESTRUCTOR,
        PACKAGE_TARGET_CONSTRUCTOR,
        SET_TARG_TYPE,
        SET_TARG_REFERENCE,
        SET_TARGET_WORD_AT_8,
        PACKAGE_TARGET_DESTRUCTOR,
        SET_PACKAGE_WORD_AT_0X18,
        SCRIPT_OF_FORM,
        GET_SCRIPT_VARIABLES,
        SCRIPT_FIND_VARIABLE,
        SCRIPT_LOCALS_SET_VARIABLE,
        SCRIPT_LOCALS_GET_VARIABLE,
        CALENDAR_GET_MONTH,
        CALENDAR_GET_DAY,
        CALENDAR_GET_HOUR,
        CALENDAR_GET_YEAR,
        MAJOR_CRIME_COUNT,
        PROCESS_HAS_CACHED_VALUES,
        CACHED_VALUES_FLAG_CLEAR,
        CACHED_VALUES_OWNER_REFRESH,
        ARRAY_FIND_WITH_COMPARE,
        HIT_DATA_ADD_REFERENCE,
        HIT_DATA_RELEASE,
        SIMPLE_ARRAY_INITIALISE,
        SIMPLE_ARRAY_DESTRUCTOR_BODY,
        COMBAT_CONTROLLER_GET_WORD,
        COMBAT_CONTROLLER_GET_FLAG,
        COMBAT_CONTROLLER_GET_GROUP,
        COMBAT_GROUP_FILL_TARGET_ARRAY,
        COMBAT_GROUP_FILL_MEMBER_ARRAY,
        FIND_FIRST_COLLISION_OBJECT,
        COLLISION_OBJECT_GET_TARGET,
        HIT_SOUND_REQUEST_CONSTRUCTOR,
        TARGET_OBJECT_GET_OBJECT,
        OBJECT_GET_KIND,
        IMPACT_MIXER_PLAY_COLLISION_SOUND,
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

    /// Where the doubles of the setting objects keep a setting's value.
    fn setting_slot(setting: u32) -> u32 {
        0x0600_0000 + (setting & 0xfff) * 0x10 + 4
    }

    fn put_setting_float(e: &mut Engine, setting: u32, value: f32) {
        e.mem.set_f32(setting_slot(setting), value);
    }

    fn put_setting_int(e: &mut Engine, setting: u32, value: i32) {
        e.mem.set_i32(setting_slot(setting), value);
    }

    /// An engine with the pages the code reads and doubles over every callee;
    /// the small getters behave as the game's do.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x0101_2000, 0x0101_a000, 0x011d_e000] {
            e.map(page, 0x1000);
        }
        e.map(0x0600_0000, 0x1_0000);
        e.set_global(FLOAT_MINUS_ONE, -1.0f32);
        e.set_global(DOUBLE_MINUS_ONE, -1.0f64);
        for addr in CALLEES {
            double(&mut e, *addr, Ret::default());
        }
        e.register(ACTOR_PROCESS, |e, a| eax(e.mem.u32(a[0] + 0x68)));
        e.register(OPERATOR_NEW, |e, a| eax(e.mem.alloc(a[0])));
        e.register(SETTING_FLOAT_POINTER, |_, a| eax(setting_slot(a[0])));
        e.register(SETTING_INT_POINTER, |_, a| eax(setting_slot(a[0])));
        e.register(FORM_TYPE, |e, a| eax(e.mem.u8(a[0] + 4) as u32));
        e.register(
            PACKAGE_TYPE,
            |e, a| eax(e.mem.i8(a[0] + 0x20) as i32 as u32),
        );
        e.register(IN_COMBAT_FLAG, |e, a| eax(e.mem.u8(a[0] + 0x104) as u32));
        e.register(RECORD_REFERENCE, |e, a| eax(e.mem.u32(a[0] + 0xc)));
        e.register(WORD_AT_8, |e, a| eax(e.mem.u32(a[0] + 8)));
        e.register(LIST_NODE_ITEM_SLOT, |_, a| eax(a[0]));
        e.register(LIST_NODE_NEXT, |e, a| eax(e.mem.u32(a[0] + 4)));
        e.register(EXTRA_LIST_OF_REFERENCE, |_, a| eax(a[0] + 0x44));
        e.register(GET_FORM, |_, a| eax(a[0]));
        e
    }

    /// Puts a double answering `ret` into slot `offset` of the object's
    /// vtable (a fresh table when it has none) and returns the double's address.
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

    /// Slots that answer zero.
    fn quiet_slots(e: &mut Engine, object: u32, offsets: &[u32]) {
        for offset in offsets {
            slot(e, object, *offset, Ret::default());
        }
    }

    /// An actor-sized object (large enough for the player's fields).
    fn new_actor(e: &mut Engine) -> Ptr<Actor> {
        Ptr::new(e.mem.alloc(0x800))
    }

    /// Gives the actor a process whose alarm-related slots answer zero.
    fn give_process(e: &mut Engine, actor: Ptr<Actor>) -> u32 {
        let process = e.mem.alloc(0x800);
        e.set(actor, Actor::pCurrentProcess, Ptr::new(process));
        quiet_slots(
            e,
            process,
            &[0x28, 0x114, 0x128, 0x214, 0x22c, 0x284, 0x2a4, 0x33c, 0x710],
        );
        process
    }

    /// An actor with the slots the crime code asks of a witness.
    fn witness(e: &mut Engine) -> (u32, u32) {
        let actor = new_actor(e);
        quiet_slots(e, actor.addr(), &[0x22c, 0x2f4, 0x304, 0x320, 0x424]);
        let process = give_process(e, actor);
        (actor.addr(), process)
    }

    /// A two-word list node: the item, then the next node.
    fn node(e: &mut Engine, item: u32, next: u32) -> u32 {
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, item);
        e.mem.set_u32(node + 4, next);
        node
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap()
            .into_iter()
            .filter(|entry| entry.0 == addr)
            .map(|entry| entry.1)
            .collect()
    }

    /// The thread-local word `008bff30` stores.
    fn thread_word(e: &mut Engine) -> u32 {
        let at = e.tls() + 0x26c;
        e.mem.u32(at)
    }

    fn log_calls(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    /// The player: an actor registered as `PlayerCharacter *`.
    fn new_player(e: &mut Engine) -> Ptr<Actor> {
        let player = new_actor(e);
        e.set_global(PLAYER_POINTER, player.addr());
        player
    }

    // ---- 008bec80 / 008becf0 -------------------------------------------

    #[test]
    fn cached_value_without_a_process_asks_the_owner() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        slot(&mut e, actor.addr() + 0xa8, 0x18, st0(2.5));
        assert_eq!(fn_008bec80(&mut e, actor), 2.5);
    }

    #[test]
    fn cached_value_with_a_process_block() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        let process = e.mem.alloc(0x100);
        let block = e.mem.alloc(0x100);
        e.mem.set_u32(process + 0x2c, block);
        e.mem.set_f32(block + 0x18, 7.0);
        e.set(actor, Actor::pCurrentProcess, Ptr::new(process));
        slot(&mut e, actor.addr() + 0xa8, 0x18, st0(2.5));
        double(&mut e, PROCESS_HAS_CACHED_VALUES, eax(1));
        // Flag bit set: the block's own float.
        double(&mut e, CACHED_VALUES_FLAG_CLEAR, eax(0));
        assert_eq!(fn_008bec80(&mut e, actor), 7.0);
        // Flag bit clear: the owner.
        double(&mut e, CACHED_VALUES_FLAG_CLEAR, eax(1));
        assert_eq!(fn_008bec80(&mut e, actor), 2.5);
        // A process without a block goes straight to the owner.
        double(&mut e, PROCESS_HAS_CACHED_VALUES, eax(0));
        assert_eq!(fn_008bec80(&mut e, actor), 2.5);
    }

    #[test]
    fn cached_block_value_is_zero_without_a_block() {
        let mut e = engine();
        let process = e.mem.alloc(0x100);
        assert_eq!(fn_008becf0(&mut e, Ptr::new(process), Ptr::NULL), 0.0);
        let block = e.mem.alloc(0x100);
        e.mem.set_u32(process + 0x2c, block);
        e.mem.set_f32(block + 0x18, 1.25);
        log_calls(&mut e);
        assert_eq!(fn_008becf0(&mut e, Ptr::new(process), Ptr::NULL), 1.25);
        assert_eq!(
            calls_to(&e, CACHED_VALUES_FLAG_CLEAR),
            vec![vec![block, 0x8000_0000]]
        );
    }

    // ---- 008bed50 ---------------------------------------------------------

    /// An actor with a process, and an alarm record whose reference is `target`.
    struct Alarm {
        actor: Ptr<Actor>,
        process: u32,
        player: u32,
        record: u32,
        target: u32,
    }

    fn alarm_rig(e: &mut Engine) -> Alarm {
        let player = new_player(e).addr();
        let actor = new_actor(e);
        quiet_slots(e, actor.addr(), &[0x214, 0x22c, 0x2f4, 0x304, 0x418]);
        let process = give_process(e, actor);
        let target = e.mem.alloc(0x800);
        quiet_slots(e, target, &[0x218, 0x410]);
        let record = e.mem.alloc(0x100);
        e.mem.set_u32(record + 0xc, target);
        Alarm {
            actor,
            process,
            player,
            record,
            target,
        }
    }

    fn package(e: &mut Engine, kind: i8) -> u32 {
        let package = e.mem.alloc(0x40);
        e.mem.set_i8(package + 0x20, kind);
        package
    }

    #[test]
    fn alarm_package_is_not_repeated() {
        let mut e = engine();
        let rig = alarm_rig(&mut e);
        let running = package(&mut e, 0x15);
        slot(&mut e, rig.process, 0x22c, eax(running));
        log_calls(&mut e);
        fn_008bed50(&mut e, rig.actor, Ptr::new(rig.record), 0, false, Ptr::NULL);
        assert!(calls_to(&e, OPERATOR_NEW).is_empty());
        assert!(calls_to(&e, SET_PACK_TYPE).is_empty());
    }

    #[test]
    fn busy_actor_tells_its_process_about_the_record() {
        let mut e = engine();
        let rig = alarm_rig(&mut e);
        e.mem.set_u8(rig.actor.addr() + 0x104, 1);
        let tell = slot(&mut e, rig.process, 0x344, Ret::default());
        log_calls(&mut e);
        fn_008bed50(&mut e, rig.actor, Ptr::new(rig.record), 0, true, Ptr::NULL);
        assert_eq!(
            calls_to(&e, tell),
            vec![vec![rig.process, rig.actor.addr(), rig.target, rig.record]]
        );
        assert!(calls_to(&e, SET_PACK_TYPE).is_empty());
        // A record flagged at +0x10 is not told.
        double(&mut e, RECORD_BYTE_AT_0X10, eax(1));
        log_calls(&mut e);
        fn_008bed50(&mut e, rig.actor, Ptr::new(rig.record), 0, true, Ptr::NULL);
        assert!(calls_to(&e, tell).is_empty());
    }

    #[test]
    fn alarm_package_is_built_and_started() {
        let mut e = engine();
        let rig = alarm_rig(&mut e);
        let running = package(&mut e, 0x07);
        slot(&mut e, rig.process, 0x22c, eax(running));
        let alarm = 0xa1a0;
        double(&mut e, ALARM_PACKAGE_CONSTRUCTOR, eax(alarm));
        double(&mut e, PACKAGE_TARGET_OBJECT, eax(0xc0de));
        double(&mut e, PACKAGE_GET_FLAG_800000, eax(1));
        double(&mut e, PACKAGE_GET_FLAG_200000, eax(0));
        let location = 0x10c;
        double(&mut e, PACKAGE_LOCATION_CONSTRUCTOR, eax(location));
        let start = slot(&mut e, rig.actor.addr(), 0x2f4, Ret::default());
        let finish = slot(&mut e, rig.process, 0x284, Ret::default());
        log_calls(&mut e);
        fn_008bed50(
            &mut e,
            rig.actor,
            Ptr::new(rig.record),
            0x77,
            false,
            Ptr::NULL,
        );
        assert_eq!(calls_to(&e, SET_PACK_TYPE), vec![vec![alarm, 0x15]]);
        assert_eq!(calls_to(&e, PACKAGE_SET_FLAG_800000), vec![vec![alarm, 1]]);
        assert_eq!(calls_to(&e, PACKAGE_SET_FLAG_200000), vec![vec![alarm, 0]]);
        assert_eq!(
            calls_to(&e, SET_LOC_REFERENCE),
            vec![vec![location, rig.target]]
        );
        assert_eq!(
            calls_to(&e, SET_PACKAGE_LOCATION),
            vec![vec![alarm, location]]
        );
        assert_eq!(
            calls_to(&e, SET_PACKAGE_WORD_AT_0X18),
            vec![vec![alarm, 0xb]]
        );
        assert_eq!(calls_to(&e, SET_TARG_TYPE), vec![vec![0xc0de, 0]]);
        assert_eq!(
            calls_to(&e, SET_TARG_REFERENCE),
            vec![vec![0xc0de, rig.target]]
        );
        assert_eq!(
            calls_to(&e, start),
            vec![vec![rig.actor.addr(), alarm, 0, 1]]
        );
        assert_eq!(calls_to(&e, finish), vec![vec![rig.process, 0x77]]);
        assert_eq!(
            calls_to(&e, CRIME_ADD_WITNESS),
            vec![vec![rig.record, rig.actor.addr()]]
        );
    }

    #[test]
    fn alarm_package_without_a_record_uses_the_reference_argument() {
        let mut e = engine();
        let rig = alarm_rig(&mut e);
        double(&mut e, ALARM_PACKAGE_CONSTRUCTOR, eax(0xa1a0));
        double(&mut e, PACKAGE_TARGET_OBJECT, eax(0xc0de));
        log_calls(&mut e);
        fn_008bed50(&mut e, rig.actor, Ptr::NULL, 0, false, Ptr::new(rig.target));
        assert_eq!(
            calls_to(&e, SET_TARG_REFERENCE),
            vec![vec![0xc0de, rig.target]]
        );
        assert!(calls_to(&e, CRIME_ADD_WITNESS).is_empty());
    }

    #[test]
    fn existing_alarm_package_gets_the_record_once() {
        let mut e = engine();
        let rig = alarm_rig(&mut e);
        let existing = package(&mut e, 0x15);
        double(&mut e, GET_CURRENT_PACKAGE, eax(existing));
        slot(&mut e, rig.process, 0x128, eax(rig.target));
        log_calls(&mut e);
        fn_008bed50(&mut e, rig.actor, Ptr::new(rig.record), 0, false, Ptr::NULL);
        assert_eq!(
            calls_to(&e, CRIME_APPEND_WITNESS),
            vec![vec![existing, rig.record]]
        );
        assert!(calls_to(&e, SET_PACK_TYPE).is_empty());
        // Already listed: nothing is added.
        double(&mut e, CRIME_HAS_WITNESS, eax(1));
        log_calls(&mut e);
        fn_008bed50(&mut e, rig.actor, Ptr::new(rig.record), 0, false, Ptr::NULL);
        assert!(calls_to(&e, CRIME_APPEND_WITNESS).is_empty());
        // The process handles another reference: nothing is added either.
        double(&mut e, CRIME_HAS_WITNESS, eax(0));
        slot(&mut e, rig.process, 0x128, eax(0x1234));
        log_calls(&mut e);
        fn_008bed50(&mut e, rig.actor, Ptr::new(rig.record), 0, false, Ptr::NULL);
        assert!(calls_to(&e, CRIME_APPEND_WITNESS).is_empty());
    }

    #[test]
    fn guards_make_the_victim_avoid_the_reference() {
        let mut e = engine();
        let rig = alarm_rig(&mut e);
        double(&mut e, CLASS_IS_GUARD, eax(1));
        slot(&mut e, rig.target, 0x218, eax(1));
        double(&mut e, GET_REF_PERSISTS, eax(1));
        let order = slot(&mut e, rig.target, 0x410, Ret::default());
        let other = package(&mut e, 0x07);
        double(&mut e, GET_CURRENT_PACKAGE, eax(other));
        double(&mut e, RT_DYNAMIC_CAST, eax(0xf1ee));
        log_calls(&mut e);
        fn_008bed50(&mut e, rig.actor, Ptr::new(rig.record), 0, false, Ptr::NULL);
        assert_eq!(e.mem.u8(rig.record + 0x2c), 1);
        let minus_one = (-1.0f32).to_bits();
        assert_eq!(
            calls_to(&e, order),
            vec![vec![
                rig.target,
                rig.actor.addr(),
                0,
                1,
                1,
                0,
                0,
                minus_one,
                minus_one
            ]]
        );
        assert_eq!(
            calls_to(&e, FLEE_PACKAGE_ADD_AVOIDED_REF),
            vec![vec![0xf1ee, rig.actor.addr()]]
        );
        // The player as the victim is left alone.
        e.mem.set_u32(rig.record + 0xc, rig.player);
        quiet_slots(&mut e, rig.player, &[0x218]);
        slot(&mut e, rig.player, 0x218, eax(1));
        log_calls(&mut e);
        fn_008bed50(&mut e, rig.actor, Ptr::new(rig.record), 0, false, Ptr::NULL);
        assert!(calls_to(&e, FLEE_PACKAGE_ADD_AVOIDED_REF).is_empty());
    }

    #[test]
    fn combat_actor_with_a_target_notifies_the_reference() {
        let mut e = engine();
        let rig = alarm_rig(&mut e);
        slot(&mut e, rig.actor.addr(), 0x304, eax(1));
        let reference_process = e.mem.alloc(0x100);
        e.mem.set_u32(rig.target + 0x68, reference_process);
        let count = slot(&mut e, reference_process, 0x348, Ret::default());
        slot(&mut e, reference_process, 0x5c0, eax(3));
        put_setting_int(&mut e, SETTING_ALARM_ESCALATION_LIMIT, 2);
        double(&mut e, PROCESS_LISTS_FIND_ENTRY, eax(0x5e5));
        double(&mut e, ENTRY_COUNT, eax(4));
        log_calls(&mut e);
        fn_008bed50(&mut e, rig.actor, Ptr::new(rig.record), 0, false, Ptr::NULL);
        assert_eq!(
            calls_to(&e, PROCESS_LISTS_FIND_ENTRY),
            vec![vec![PROCESS_LISTS, rig.target, 0x15, 0]]
        );
        assert_eq!(calls_to(&e, count), vec![vec![reference_process, 4]]);
        // The level reaches the limit: nothing else happens.
        assert!(calls_to(&e, OPERATOR_NEW).is_empty());
        // No entry: the count is 0.
        double(&mut e, PROCESS_LISTS_FIND_ENTRY, eax(0));
        log_calls(&mut e);
        fn_008bed50(&mut e, rig.actor, Ptr::new(rig.record), 0, false, Ptr::NULL);
        assert_eq!(calls_to(&e, count), vec![vec![reference_process, 0]]);
    }

    #[test]
    fn combat_actor_sends_the_player_attack_order() {
        let mut e = engine();
        let rig = alarm_rig(&mut e);
        slot(&mut e, rig.actor.addr(), 0x304, eax(1));
        e.mem.set_u32(rig.record + 0xc, rig.player);
        let player_process = e.mem.alloc(0x100);
        e.mem.set_u32(rig.player + 0x68, player_process);
        quiet_slots(&mut e, player_process, &[0x348, 0x5c0, 0x5c4]);
        put_setting_int(&mut e, SETTING_ALARM_ESCALATION_LIMIT, 10);
        double(&mut e, PLAYER_BYTE_AT_0X7C4, eax(1));
        let order = slot(&mut e, rig.process, 0x33c, Ret::default());
        log_calls(&mut e);
        fn_008bed50(&mut e, rig.actor, Ptr::new(rig.record), 0, false, Ptr::NULL);
        assert_eq!(
            calls_to(&e, order),
            vec![vec![
                rig.process,
                rig.actor.addr(),
                rig.player,
                1,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                1,
                0
            ]]
        );
        assert!(calls_to(&e, OPERATOR_NEW).is_empty());
        // Without the player flag the reference's process gets slot 0x5c4.
        let mark = slot(&mut e, player_process, 0x5c4, Ret::default());
        double(&mut e, PLAYER_BYTE_AT_0X7C4, eax(0));
        log_calls(&mut e);
        fn_008bed50(&mut e, rig.actor, Ptr::new(rig.record), 0, false, Ptr::NULL);
        assert_eq!(calls_to(&e, mark), vec![vec![player_process, 1]]);
    }

    #[test]
    fn record_byte_is_stored() {
        let mut e = engine();
        let record = e.mem.alloc(0x40);
        fn_008bf4e0(&mut e, Ptr::new(record), 1);
        assert_eq!(e.mem.u8(record + 0x2c), 1);
        fn_008bf4e0(&mut e, Ptr::new(record), 0);
        assert_eq!(e.mem.u8(record + 0x2c), 0);
    }

    // ---- 008bf500 / 008bf7b0 ---------------------------------------------

    fn package_rig(e: &mut Engine) -> (Alarm, u32) {
        let rig = alarm_rig(e);
        double(e, CREATE_PACKAGE, eax(0xa1a0));
        double(e, PACKAGE_TARGET_OBJECT, eax(0xc0de));
        double(e, PACKAGE_LOCATION_CONSTRUCTOR, eax(0x10c));
        let start = slot(e, rig.actor.addr(), 0x2f4, Ret::default());
        (rig, start)
    }

    #[test]
    fn pursuit_package_type_0x23() {
        let mut e = engine();
        let (rig, start) = package_rig(&mut e);
        quiet_slots(&mut e, rig.process, &[0x644]);
        let other = package(&mut e, 0x05);
        slot(&mut e, rig.process, 0x22c, eax(other));
        double(&mut e, PACKAGE_GET_FLAG_200000, eax(1));
        // An interrupt package in progress is ended first.
        double(&mut e, GET_CURRENT_PACKAGE, eax(other));
        double(&mut e, IS_INTERRUPT_PACKAGE, eax(1));
        let end = slot(&mut e, rig.actor.addr(), 0x288, Ret::default());
        let target = Ptr::new(rig.target);
        log_calls(&mut e);
        fn_008bf500(&mut e, rig.actor, target);
        assert_eq!(calls_to(&e, end).len(), 1);
        assert_eq!(calls_to(&e, CREATE_PACKAGE), vec![vec![0x23]]);
        assert_eq!(calls_to(&e, SET_PACK_TYPE), vec![vec![0xa1a0, 0x23]]);
        assert_eq!(
            calls_to(&e, SET_PACKAGE_WORD_AT_0X18),
            vec![vec![0xa1a0, 0x2b]]
        );
        assert_eq!(
            calls_to(&e, SET_LOC_REFERENCE),
            vec![vec![0x10c, rig.target]]
        );
        assert_eq!(
            calls_to(&e, SET_TARG_REFERENCE),
            vec![vec![0xc0de, rig.target]]
        );
        assert_eq!(calls_to(&e, SET_TARGET_WORD_AT_8), vec![vec![0xc0de, 0]]);
        assert_eq!(calls_to(&e, PACKAGE_SET_FLAG_200000), vec![vec![0xa1a0, 1]]);
        assert_eq!(
            calls_to(&e, start),
            vec![vec![rig.actor.addr(), 0xa1a0, 0, 1]]
        );
        assert_eq!(
            calls_to(&e, PLAYER_ALARM_COUNTER_23),
            vec![vec![rig.player]]
        );
        assert!(calls_to(&e, PLAYER_ALARM_COUNTER_22).is_empty());
    }

    #[test]
    fn pursuit_package_type_0x22() {
        let mut e = engine();
        let (rig, _start) = package_rig(&mut e);
        quiet_slots(&mut e, rig.process, &[0x644]);
        let other = package(&mut e, 0x05);
        slot(&mut e, rig.process, 0x22c, eax(other));
        let target = Ptr::new(rig.target);
        log_calls(&mut e);
        fn_008bf7b0(&mut e, rig.actor, target, 0);
        assert_eq!(calls_to(&e, CREATE_PACKAGE), vec![vec![0x22]]);
        assert_eq!(
            calls_to(&e, SET_PACKAGE_WORD_AT_0X18),
            vec![vec![0xa1a0, 0x2a]]
        );
        assert!(calls_to(&e, PACKAGE_SET_FLAG_200000).is_empty());
        assert_eq!(
            calls_to(&e, PLAYER_ALARM_COUNTER_22),
            vec![vec![rig.player]]
        );
        assert!(calls_to(&e, PLAYER_ALARM_COUNTER_23).is_empty());
    }

    #[test]
    fn pursuit_packages_are_skipped_in_combat() {
        let mut e = engine();
        let (rig, _start) = package_rig(&mut e);
        e.mem.set_u8(rig.actor.addr() + 0x104, 1);
        log_calls(&mut e);
        fn_008bf500(&mut e, rig.actor, Ptr::new(rig.target));
        fn_008bf7b0(&mut e, rig.actor, Ptr::new(rig.target), 0);
        assert!(calls_to(&e, CREATE_PACKAGE).is_empty());
        assert!(calls_to(&e, PLAYER_ALARM_COUNTER_23).is_empty());
    }

    // ---- 008bfa40 StealAlarm ---------------------------------------------

    struct Theft {
        player: Ptr<Actor>,
        player_process: u32,
        victim: u32,
        victim_process: u32,
        crime: u32,
    }

    /// The player steals from an actor who is also the witness.
    fn theft_rig(e: &mut Engine) -> Theft {
        let player = new_player(e);
        let player_process = give_process(e, player);
        quiet_slots(e, player_process, &[0x4a4]);
        let (victim, victim_process) = witness(e);
        slot(e, victim, 0x100, eax(1));
        slot(e, victim, 0x218, eax(1));
        let crime = e.mem.alloc(0x40);
        e.mem.set_u32(crime + 0xc, 0xbeef);
        double(e, CRIME_CONSTRUCTOR, eax(crime));
        double(e, GET_DETECTION_LEVEL, eax(5));
        double(e, GET_FORM_WEIGHT, st0(2.0));
        double(e, GET_TOPIC, eax(0x70c));
        double(e, CALENDAR_GET_HOUR, st0(13.5));
        double(e, CALENDAR_GET_MONTH, eax(5));
        double(e, CALENDAR_GET_DAY, eax(15));
        double(e, CALENDAR_GET_YEAR, eax(2));
        put_setting_float(e, SETTING_STEAL_WEIGHT_FACTOR, 3.0);
        put_setting_int(e, SETTING_STEAL_REPORT_LIMIT, 0);
        Theft {
            player,
            player_process,
            victim,
            victim_process,
            crime,
        }
    }

    #[test]
    fn theft_is_reported_and_the_witness_speaks() {
        use std::cell::Cell;
        use std::rc::Rc;
        let mut e = engine();
        let rig = theft_rig(&mut e);
        let weight = slot(&mut e, rig.player_process, 0x4a4, Ret::default());
        // The thread-local word holds the owner while the witness speaks.
        let seen = Rc::new(Cell::new(0));
        let seen_in_double = seen.clone();
        let speak = NEXT_TARGET.fetch_add(4, Ordering::Relaxed);
        let table = e.mem.u32(rig.victim_process);
        e.mem.set_u32(table + 0x2a4, speak);
        e.register_double(speak, move |e, _| {
            seen_in_double.set(thread_word(e));
            Ret::default()
        });
        log_calls(&mut e);
        actor_steal_alarm(
            &mut e,
            rig.player,
            Ptr::new(rig.victim),
            0x1111,
            2,
            0x22,
            0x3333,
        );
        let player = rig.player.addr();
        assert_eq!(e.mem.i32(player + 0x744), 2);
        assert_eq!(
            calls_to(&e, weight),
            vec![vec![rig.player_process, 6.0f32.to_bits()]]
        );
        let constructed = calls_to(&e, CRIME_CONSTRUCTOR);
        assert_eq!(constructed.len(), 1);
        assert_eq!(
            constructed[0][1..],
            [0, rig.victim, player, 0x1111, 2, 0x3333]
        );
        assert_eq!(
            calls_to(&e, ADD_TO_PLAYER_CRIME_LIST),
            vec![vec![rig.victim + 0x44, rig.crime, 13.5f32.to_bits()]]
        );
        assert_eq!(e.get(Ptr::<Actor>::new(rig.victim), Actor::iMinorCrimes), 1);
        assert_eq!(
            calls_to(&e, ADD_FACTION_MINOR_CRIME),
            vec![vec![rig.victim, 1, 1]]
        );
        assert_eq!(
            calls_to(&e, PROCESS_LISTS_ADD_CRIME),
            vec![vec![PROCESS_LISTS, rig.crime]]
        );
        assert_eq!(
            calls_to(&e, CRIME_ADD_WITNESS),
            vec![vec![rig.crime, rig.victim]]
        );
        assert!(e.get(rig.player, Actor::bAttackOnNextTheft));
        assert_eq!(
            e.get(rig.player, Actor::iThiefCrimeStamp),
            (5 << 9) | 15 | (2 << 13)
        );
        assert_eq!(calls_to(&e, SET_WORD_AT_0X70), vec![vec![player, 0xbeef]]);
        assert_eq!(calls_to(&e, GET_TOPIC), vec![vec![2, 9]]);
        assert_eq!(
            calls_to(&e, speak),
            vec![vec![rig.victim_process, rig.victim, 0x70c, 0, 0, 1, 0]]
        );
        assert_eq!(seen.get(), 0x3333);
        assert_eq!(thread_word(&mut e), 0);
    }

    #[test]
    fn theft_below_the_report_limit_starts_the_pursuit() {
        let mut e = engine();
        let rig = theft_rig(&mut e);
        put_setting_int(&mut e, SETTING_STEAL_REPORT_LIMIT, 5);
        double(&mut e, CREATE_PACKAGE, eax(0xa1a0));
        quiet_slots(&mut e, rig.victim_process, &[0x644]);
        log_calls(&mut e);
        actor_steal_alarm(&mut e, rig.player, Ptr::new(rig.victim), 0x1111, 1, 0, 0);
        assert_eq!(calls_to(&e, CREATE_PACKAGE), vec![vec![0x22]]);
        assert_eq!(
            calls_to(&e, PLAYER_ALARM_COUNTER_22),
            vec![vec![rig.player.addr()]]
        );
        assert!(calls_to(&e, GET_TOPIC).is_empty());
    }

    #[test]
    fn repeated_theft_the_same_day_makes_the_witness_attack() {
        let mut e = engine();
        let rig = theft_rig(&mut e);
        e.set(rig.player, Actor::bAttackOnNextTheft, true);
        e.set(rig.player, Actor::iThiefCrimeStamp, 15);
        let react = slot(&mut e, rig.victim, 0x424, Ret::default());
        log_calls(&mut e);
        actor_steal_alarm(&mut e, rig.player, Ptr::new(rig.victim), 0x1111, 1, 0, 0);
        assert_eq!(
            calls_to(&e, react),
            vec![vec![rig.victim, rig.player.addr(), 0, 1, 0, 0, 0, 1, 0]]
        );
        assert!(calls_to(&e, GET_TOPIC).is_empty());
        // Another day: the flag is dropped and the witness only speaks.
        e.set(rig.player, Actor::iThiefCrimeStamp, 14);
        log_calls(&mut e);
        actor_steal_alarm(&mut e, rig.player, Ptr::new(rig.victim), 0x1111, 1, 0, 0);
        assert!(!e.get(rig.player, Actor::bAttackOnNextTheft));
        assert!(calls_to(&e, react).is_empty());
        assert_eq!(calls_to(&e, GET_TOPIC).len(), 1);
    }

    #[test]
    fn theft_penalises_karma_for_a_non_good_owner() {
        let mut e = engine();
        let player = new_player(&mut e);
        give_process(&mut e, player);
        let victim = e.mem.alloc(0x800);
        slot(&mut e, victim, 0x100, eax(0));
        let owner = e.mem.alloc(0x400);
        e.mem.set_u8(owner + 4, 8);
        put_setting_float(&mut e, SETTING_STEAL_KARMA_PENALTY, 5.0);
        double(&mut e, FLOAT_TO_INT, eax(5));
        log_calls(&mut e);
        actor_steal_alarm(&mut e, player, Ptr::new(victim), 0, 1, 0, owner);
        let bits = 5.0f64.to_bits();
        assert_eq!(
            calls_to(&e, FLOAT_TO_INT),
            vec![vec![bits as u32, (bits >> 32) as u32]]
        );
        assert_eq!(calls_to(&e, REWARD_KARMA), vec![vec![player.addr(), 5]]);
        assert_eq!(
            calls_to(&e, PROCESS_LISTS_FIND_ACTOR_REF),
            vec![vec![PROCESS_LISTS, owner, 1, 0]]
        );
        // A base form with bit 1 set is not penalised.
        double(&mut e, BASE_FORM_FLAG_BIT_1, eax(1));
        log_calls(&mut e);
        actor_steal_alarm(&mut e, player, Ptr::new(victim), 0, 1, 0, owner);
        assert!(calls_to(&e, REWARD_KARMA).is_empty());
    }

    #[test]
    fn theft_penalty_follows_the_owner_alignment() {
        let mut e = engine();
        let player = new_player(&mut e);
        give_process(&mut e, player);
        let victim = e.mem.alloc(0x800);
        slot(&mut e, victim, 0x100, eax(0));
        let owner = e.mem.alloc(0x400);
        e.mem.set_u8(owner + 4, 0x2a);
        slot(&mut e, owner + 0x100, 0xc, st0(10.0));
        put_setting_float(&mut e, SETTING_STEAL_KARMA_PENALTY, 5.0);
        double(&mut e, FLOAT_TO_INT, eax(5));
        double(&mut e, GET_ALIGNMENT_FOR_KARMA, eax(2));
        log_calls(&mut e);
        actor_steal_alarm(&mut e, player, Ptr::new(victim), 0, 1, 0, owner);
        assert!(calls_to(&e, REWARD_KARMA).is_empty());
        assert_eq!(
            calls_to(&e, GET_ALIGNMENT_FOR_KARMA),
            vec![vec![10.0f32.to_bits()]]
        );
        double(&mut e, GET_ALIGNMENT_FOR_KARMA, eax(3));
        log_calls(&mut e);
        actor_steal_alarm(&mut e, player, Ptr::new(victim), 0, 1, 0, owner);
        assert_eq!(calls_to(&e, REWARD_KARMA).len(), 1);
        assert_eq!(
            calls_to(&e, PROCESS_LISTS_GET_ACTOR_REF_IN_HIGH),
            vec![vec![PROCESS_LISTS, owner, 1]]
        );
    }

    #[test]
    fn theft_is_only_the_players_business() {
        let mut e = engine();
        new_player(&mut e);
        let other = new_actor(&mut e);
        log_calls(&mut e);
        actor_steal_alarm(&mut e, other, Ptr::NULL, 0, 1, 0, 0);
        assert!(e.call_log.clone().unwrap().is_empty());
    }

    #[test]
    fn heavy_loot_scales_the_process_weight_only_when_positive() {
        let mut e = engine();
        let rig = theft_rig(&mut e);
        let weight = slot(&mut e, rig.player_process, 0x4a4, Ret::default());
        double(&mut e, GET_FORM_WEIGHT, st0(0.0));
        log_calls(&mut e);
        actor_steal_alarm(&mut e, rig.player, Ptr::new(rig.victim), 0x1111, 1, 0, 0);
        assert!(calls_to(&e, weight).is_empty());
    }

    // ---- 008bff30 .. 008bffa0 --------------------------------------------

    #[test]
    fn thread_word_is_stored() {
        let mut e = engine();
        fn_008bff30(&mut e, 0x1234);
        assert_eq!(thread_word(&mut e), 0x1234);
    }

    #[test]
    fn minor_crimes_count_up() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        fn_008bff50(&mut e, actor);
        fn_008bff50(&mut e, actor);
        assert_eq!(e.get(actor, Actor::iMinorCrimes), 2);
    }

    #[test]
    fn player_counters_add() {
        let mut e = engine();
        let player = new_actor(&mut e);
        fn_008bff70(&mut e, player.cast(), 3, 4);
        fn_008bff70(&mut e, player.cast(), 3, -1);
        assert_eq!(e.mem.i32(player.addr() + 0x744 + 12), 3);
        assert_eq!(e.mem.i32(player.addr() + 0x744), 0);
    }

    #[test]
    fn theft_counter_is_read_from_the_player() {
        let mut e = engine();
        let player = new_actor(&mut e);
        e.mem.set_u32(player.addr() + 0x228, 77);
        assert_eq!(fn_008bffa0(&mut e, player.cast()), 77);
    }

    // ---- 008bffc0 / 008c0050 ---------------------------------------------

    fn angry_case(crimes: u32, empty: bool, process: bool, package_type: Option<i8>) -> bool {
        let mut e = engine();
        let actor = new_actor(&mut e);
        if process {
            let proc = give_process(&mut e, actor);
            let running = match package_type {
                Some(kind) => package(&mut e, kind),
                None => 0,
            };
            slot(&mut e, proc, 0x22c, eax(running));
        }
        double(&mut e, GET_PLAYER_CRIME_LIST, eax(crimes));
        double(&mut e, LIST_IS_EMPTY, eax(empty as u32));
        actor_is_angry_with_player(&mut e, actor)
    }

    #[test]
    fn anger_needs_a_crime_list_with_entries() {
        assert!(!angry_case(0, false, false, None));
        assert!(!angry_case(0x1000, true, false, None));
        assert!(angry_case(0x1000, false, false, None));
    }

    #[test]
    fn anger_depends_on_the_running_package() {
        assert!(angry_case(0x1000, false, true, None));
        assert!(angry_case(0x1000, false, true, Some(7)));
        assert!(!angry_case(0x1000, false, true, Some(0x22)));
        assert!(!angry_case(0x1000, false, true, Some(0x23)));
    }

    #[test]
    fn process_object_cast_or_run_once_type() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        let process = give_process(&mut e, actor);
        let object = package(&mut e, 1);
        slot(&mut e, process, 0x27c, eax(object));
        double(&mut e, RT_DYNAMIC_CAST, eax(0x99));
        assert!(fn_008c0050(&mut e, actor));
        double(&mut e, RT_DYNAMIC_CAST, eax(0));
        assert!(!fn_008c0050(&mut e, actor));
        double(&mut e, IS_RUNNING_RUN_ONCE, eax(1));
        assert!(fn_008c0050(&mut e, actor));
        e.mem.set_i8(object + 0x20, 2);
        assert!(!fn_008c0050(&mut e, actor));
    }

    // ---- 008c00e0 ---------------------------------------------------------

    struct Harm {
        player: Ptr<Actor>,
        victim: Ptr<Actor>,
        victim_process: u32,
        crime: u32,
    }

    fn harm_rig(e: &mut Engine) -> Harm {
        let player = new_player(e);
        quiet_slots(e, player.addr(), &[0x424]);
        let victim = new_actor(e);
        quiet_slots(e, victim.addr(), &[0x22c, 0x2f4, 0x304, 0x320, 0x448]);
        let victim_process = give_process(e, victim);
        quiet_slots(e, victim_process, &[0x504, 0x508, 0x644]);
        e.register(RT_DYNAMIC_CAST, |_, a| eax(a[0]));
        let crime = e.mem.alloc(0x40);
        e.mem.set_u32(crime + 8, victim.addr());
        e.mem.set_u32(crime + 0xc, 0xbeef);
        double(e, CRIME_CONSTRUCTOR, eax(crime));
        double(e, CALENDAR_GET_HOUR, st0(8.0));
        double(e, CALENDAR_GET_MONTH, eax(3));
        double(e, CALENDAR_GET_DAY, eax(9));
        double(e, CALENDAR_GET_YEAR, eax(1));
        double(e, CREATE_PACKAGE, eax(0xa1a0));
        Harm {
            player,
            victim,
            victim_process,
            crime,
        }
    }

    #[test]
    fn harm_to_a_dead_victim_only_flags_its_process() {
        let mut e = engine();
        let rig = harm_rig(&mut e);
        slot(&mut e, rig.victim.addr(), 0x320, eax(1));
        let flag = slot(&mut e, rig.victim_process, 0x114, Ret::default());
        log_calls(&mut e);
        fn_008c00e0(&mut e, rig.player, rig.victim.cast(), 1, 2);
        assert_eq!(calls_to(&e, flag), vec![vec![rig.victim_process, 1]]);
        assert!(calls_to(&e, CRIME_CONSTRUCTOR).is_empty());
        // The same for the second state test.
        slot(&mut e, rig.victim.addr(), 0x320, eax(0));
        slot(&mut e, rig.victim.addr(), 0x448, eax(1));
        log_calls(&mut e);
        fn_008c00e0(&mut e, rig.player, rig.victim.cast(), 1, 2);
        assert_eq!(calls_to(&e, flag), vec![vec![rig.victim_process, 1]]);
    }

    #[test]
    fn harm_builds_a_crime_and_starts_the_pursuit() {
        let mut e = engine();
        let rig = harm_rig(&mut e);
        let entry = e.mem.alloc(0x10);
        let lookup = slot(&mut e, rig.victim_process, 0x504, eax(entry));
        log_calls(&mut e);
        fn_008c00e0(&mut e, rig.player, rig.victim.cast(), 0x11, 0x22);
        let victim = rig.victim.addr();
        let player = rig.player.addr();
        let constructed = calls_to(&e, CRIME_CONSTRUCTOR);
        assert_eq!(constructed[0][1..], [1, victim, player, 0x11, 0x22, 0]);
        assert_eq!(
            calls_to(&e, lookup),
            vec![vec![rig.victim_process, 0xbeef, 0]]
        );
        assert_eq!(e.mem.u32(entry + 4), 3);
        assert!(calls_to(&e, IS_AN_OWNER).is_empty());
        assert_eq!(
            calls_to(&e, ADD_TO_PLAYER_CRIME_LIST),
            vec![vec![victim + 0x44, rig.crime, 8.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, CRIME_ADD_WITNESS),
            vec![vec![rig.crime, player]]
        );
        assert_eq!(e.get(rig.victim, Actor::iMinorCrimes), 1);
        assert_eq!(
            calls_to(&e, ADD_FACTION_MINOR_CRIME),
            vec![vec![victim, 1, 1]]
        );
        assert!(e.get(rig.player, Actor::bAttackOnNextTheft));
        assert_eq!(
            e.get(rig.player, Actor::iThiefCrimeStamp),
            (3 << 9) | 9 | (1 << 13)
        );
        assert_eq!(calls_to(&e, CREATE_PACKAGE), vec![vec![0x23]]);
        assert_eq!(calls_to(&e, PLAYER_ALARM_COUNTER_23), vec![vec![player]]);
    }

    #[test]
    fn harm_without_a_process_entry_adds_one() {
        let mut e = engine();
        let rig = harm_rig(&mut e);
        // A crime whose first word is another form asks for its owner.
        e.mem.set_u32(rig.crime + 8, 0x4444);
        let add = slot(&mut e, rig.victim_process, 0x508, Ret::default());
        log_calls(&mut e);
        fn_008c00e0(&mut e, rig.player, rig.victim.cast(), 0, 0);
        assert_eq!(
            calls_to(&e, IS_AN_OWNER),
            vec![vec![0x4444, rig.victim.addr(), 1]]
        );
        assert_eq!(
            calls_to(&e, add),
            vec![vec![rig.victim_process, rig.victim.addr(), 3]]
        );
    }

    #[test]
    fn repeated_harm_the_same_day_attacks_at_once() {
        let mut e = engine();
        let rig = harm_rig(&mut e);
        e.set(rig.player, Actor::bAttackOnNextTheft, true);
        e.set(rig.player, Actor::iThiefCrimeStamp, 9);
        let react = slot(&mut e, rig.player.addr(), 0x424, Ret::default());
        log_calls(&mut e);
        fn_008c00e0(&mut e, rig.player, rig.victim.cast(), 0, 0);
        assert_eq!(
            calls_to(&e, react),
            vec![vec![
                rig.player.addr(),
                rig.player.addr(),
                0,
                1,
                0,
                0,
                0,
                1,
                0
            ]]
        );
        assert!(calls_to(&e, CREATE_PACKAGE).is_empty());
    }

    #[test]
    fn harm_by_another_actor_is_ignored() {
        let mut e = engine();
        let rig = harm_rig(&mut e);
        log_calls(&mut e);
        fn_008c00e0(&mut e, rig.victim, rig.victim.cast(), 0, 0);
        assert!(e.call_log.clone().unwrap().is_empty());
    }

    // ---- 008c0460 AttackAlarm --------------------------------------------

    struct Attack {
        player: Ptr<Actor>,
        victim: u32,
        victim_process: u32,
        crime: u32,
    }

    fn attack_rig(e: &mut Engine) -> Attack {
        let player = new_player(e);
        quiet_slots(e, player.addr(), &[0x304]);
        let (victim, victim_process) = witness(e);
        e.register(RT_DYNAMIC_CAST, |_, a| eax(a[0]));
        e.register(GET_TOPIC, |_, a| eax(0x7000 + a[1]));
        let crime = e.mem.alloc(0x40);
        e.mem.set_u32(crime + 8, victim);
        e.mem.set_u32(crime + 0xc, 0xbeef);
        double(e, CRIME_CONSTRUCTOR, eax(crime));
        double(e, CALENDAR_GET_HOUR, st0(6.0));
        Attack {
            player,
            victim,
            victim_process,
            crime,
        }
    }

    #[test]
    fn attack_alarm_lets_each_witness_react() {
        let mut e = engine();
        let rig = attack_rig(&mut e);
        let (w1, p1) = witness(&mut e);
        let (w2, p2) = witness(&mut e);
        let tail = node(&mut e, w2, 0);
        let head = node(&mut e, w1, tail);
        double(&mut e, PROCESS_LISTS_WITNESSES, eax(head));
        e.register_double(SET_FACTIONS_THAT_CARE_ABOUT_CRIME, move |_, a| {
            eax((a[0] == w1) as u32)
        });
        e.register_double(
            GET_SHOULD_ATTACK_ACTOR,
            move |_, a| eax((a[0] == w2) as u32),
        );
        double(&mut e, CRIME_WITNESS_COUNT, eax(2));
        double(&mut e, MAJOR_CRIME_COUNT, eax(4));
        let say1 = slot(&mut e, p1, 0x2a4, Ret::default());
        let say2 = slot(&mut e, p2, 0x2a4, Ret::default());
        let order2 = slot(&mut e, p2, 0x33c, Ret::default());
        log_calls(&mut e);
        actor_attack_alarm(&mut e, Ptr::new(rig.victim), rig.player.cast(), false, 0);
        let player = rig.player.addr();
        assert_eq!(e.mem.i32(player + 0x744 + 12), 1);
        assert_eq!(
            calls_to(&e, CRIME_CONSTRUCTOR)[0][1..],
            [3, rig.victim, player, 0, 0, 0]
        );
        assert_eq!(
            calls_to(&e, ADD_TO_PLAYER_CRIME_LIST),
            vec![vec![rig.victim + 0x44, rig.crime, 6.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, GET_TOPIC), vec![vec![2, 0xd], vec![2, 0xf]]);
        assert_eq!(calls_to(&e, say1), vec![vec![p1, w1, 0x700d, 0, 0, 1, 0]]);
        assert!(calls_to(&e, say2).is_empty());
        assert_eq!(
            calls_to(&e, order2),
            vec![vec![p2, w2, 0xbeef, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0]]
        );
        assert_eq!(
            calls_to(&e, CRIME_ADD_WITNESS),
            vec![vec![rig.crime, rig.victim], vec![rig.crime, w1]]
        );
        assert_eq!(calls_to(&e, LIST_CLEAR), vec![vec![head]]);
        assert_eq!(calls_to(&e, LIST_DESTROY), vec![vec![head, 1]]);
        assert_eq!(
            calls_to(&e, ADD_FACTION_MAJOR_CRIME),
            vec![vec![rig.victim, 1, 0]]
        );
        assert!(calls_to(&e, ADD_FACTION_MINOR_CRIME).is_empty());
        assert_eq!(e.get(rig.player, Actor::iMajorCrimes), 5);
        assert_eq!(thread_word(&mut e), 0);
    }

    #[test]
    fn minor_attack_alarm_only_angers_the_victim() {
        let mut e = engine();
        let rig = attack_rig(&mut e);
        let (other, other_process) = witness(&mut e);
        let tail = node(&mut e, rig.victim, 0);
        let head = node(&mut e, other, tail);
        double(&mut e, PROCESS_LISTS_WITNESSES, eax(head));
        double(&mut e, CRIME_WITNESS_COUNT, eax(1));
        let say_other = slot(&mut e, other_process, 0x2a4, Ret::default());
        let say_victim = slot(&mut e, rig.victim_process, 0x2a4, Ret::default());
        log_calls(&mut e);
        actor_attack_alarm(&mut e, Ptr::new(rig.victim), rig.player.cast(), true, 0);
        // The bystander gets the plain line, the victim the angry one.
        assert_eq!(calls_to(&e, GET_TOPIC), vec![vec![2, 0xf], vec![2, 0xd]]);
        assert_eq!(calls_to(&e, say_other).len(), 1);
        assert_eq!(
            calls_to(&e, say_victim),
            vec![vec![rig.victim_process, rig.victim, 0x700d, 0, 0, 1, 0]]
        );
        assert_eq!(
            calls_to(&e, ADD_FACTION_MINOR_CRIME),
            vec![vec![rig.victim, 1, 0]]
        );
        assert!(calls_to(&e, ADD_FACTION_MAJOR_CRIME).is_empty());
        assert_eq!(e.get(rig.player, Actor::iMajorCrimes), 0);
    }

    #[test]
    fn unwitnessed_attack_is_only_logged() {
        let mut e = engine();
        let rig = attack_rig(&mut e);
        let (w1, _p1) = witness(&mut e);
        let head = node(&mut e, w1, 0);
        double(&mut e, PROCESS_LISTS_WITNESSES, eax(head));
        double(&mut e, CRIME_WITNESS_COUNT, eax(0));
        let victim = rig.victim;
        e.register_double(REFERENCE_NAME, move |_, a| {
            eax(if a[0] == victim { 0xb11 } else { 0xa77 })
        });
        log_calls(&mut e);
        actor_attack_alarm(&mut e, Ptr::new(rig.victim), rig.player.cast(), false, 0);
        let formatted = calls_to(&e, SPRINTF);
        assert_eq!(formatted.len(), 1);
        assert_eq!(formatted[0][1..], [NO_ONE_CARED_FORMAT, 0xb11, 0xa77]);
        assert_eq!(calls_to(&e, LOG_MESSAGE), vec![vec![formatted[0][0]]]);
        assert!(calls_to(&e, ADD_FACTION_MAJOR_CRIME).is_empty());
    }

    #[test]
    fn attack_without_witnesses_changes_nothing_more() {
        let mut e = engine();
        let rig = attack_rig(&mut e);
        double(&mut e, PROCESS_LISTS_WITNESSES, eax(0));
        log_calls(&mut e);
        actor_attack_alarm(&mut e, Ptr::new(rig.victim), rig.player.cast(), false, 0);
        assert_eq!(e.mem.i32(rig.player.addr() + 0x744 + 12), 1);
        assert!(calls_to(&e, LIST_CLEAR).is_empty());
        assert!(calls_to(&e, ADD_FACTION_MAJOR_CRIME).is_empty());
        assert_eq!(calls_to(&e, PROCESS_LISTS_ADD_CRIME).len(), 1);
    }

    #[test]
    fn attack_alarm_ignores_other_attackers_and_combatants() {
        let mut e = engine();
        let rig = attack_rig(&mut e);
        let other = new_actor(&mut e);
        log_calls(&mut e);
        actor_attack_alarm(&mut e, Ptr::new(rig.victim), other.cast(), false, 0);
        assert!(e.call_log.clone().unwrap().is_empty());
        // A dead victim.
        slot(&mut e, rig.victim, 0x320, eax(1));
        actor_attack_alarm(&mut e, Ptr::new(rig.victim), rig.player.cast(), false, 0);
        assert!(calls_to(&e, CRIME_CONSTRUCTOR).is_empty());
        slot(&mut e, rig.victim, 0x320, eax(0));
        // Both in the combatant factions.
        double(&mut e, IS_IN_COMBATANT_FACTION, eax(1));
        actor_attack_alarm(&mut e, Ptr::new(rig.victim), rig.player.cast(), false, 0);
        assert!(calls_to(&e, CRIME_CONSTRUCTOR).is_empty());
        // The attacker is busy with something else (slot 0x304).
        double(&mut e, IS_IN_COMBATANT_FACTION, eax(0));
        slot(&mut e, rig.player.addr(), 0x304, eax(1));
        actor_attack_alarm(&mut e, Ptr::new(rig.victim), rig.player.cast(), false, 0);
        assert!(calls_to(&e, CRIME_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn major_crime_count_keeps_the_flag_bits() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        e.set(actor, Actor::iMajorCrimes, 0xc000_0005);
        double(&mut e, MAJOR_CRIME_COUNT, eax(5));
        fn_008c09b0(&mut e, actor);
        assert_eq!(e.get(actor, Actor::iMajorCrimes), 0xc000_0006);
    }

    // ---- 008c09e0 ---------------------------------------------------------

    #[test]
    fn crime_of_kind_4_reaches_every_kind_of_witness() {
        let mut e = engine();
        let rig = attack_rig(&mut e);
        let (w1, p1) = witness(&mut e);
        let (w2, p2) = witness(&mut e);
        let (w3, p3) = witness(&mut e);
        let (w4, p4) = witness(&mut e);
        let n4 = node(&mut e, w4, 0);
        let n3 = node(&mut e, w3, n4);
        let n2 = node(&mut e, w2, n3);
        let head = node(&mut e, w1, n2);
        double(&mut e, PROCESS_LISTS_WITNESSES, eax(head));
        e.register_double(SET_FACTIONS_THAT_CARE_ABOUT_CRIME, move |_, a| {
            eax((a[0] == w1) as u32)
        });
        e.register_double(
            GET_SHOULD_ATTACK_ACTOR,
            move |_, a| eax((a[0] == w2) as u32),
        );
        e.register_double(GET_SHOULD_HELP, move |_, a| {
            eax((a[0] == w1 || a[0] == w3) as u32)
        });
        double(&mut e, CRIME_WITNESS_COUNT, eax(1));
        let say1 = slot(&mut e, p1, 0x2a4, Ret::default());
        let say2 = slot(&mut e, p2, 0x2a4, Ret::default());
        let say3 = slot(&mut e, p3, 0x2a4, Ret::default());
        let say4 = slot(&mut e, p4, 0x2a4, Ret::default());
        let react1 = slot(&mut e, w1, 0x424, Ret::default());
        let react2 = slot(&mut e, w2, 0x424, Ret::default());
        let react3 = slot(&mut e, w3, 0x424, Ret::default());
        let react4 = slot(&mut e, w4, 0x424, Ret::default());
        log_calls(&mut e);
        fn_008c09e0(&mut e, Ptr::new(rig.victim), rig.player.cast());
        let player = rig.player.addr();
        assert_eq!(e.mem.i32(player + 0x744 + 16), 1);
        assert_eq!(
            calls_to(&e, CRIME_CONSTRUCTOR)[0][1..],
            [4, rig.victim, player, 0, 0, 0]
        );
        assert_eq!(
            calls_to(&e, GET_TOPIC),
            vec![vec![2, 0xe], vec![2, 0x10], vec![2, 0x10], vec![2, 0x10]]
        );
        let react = |w: u32| vec![vec![w, 0xbeef, 0, 0, 0, 0, 0, 1, 0]];
        assert_eq!(calls_to(&e, react1), react(w1));
        assert_eq!(calls_to(&e, react2), react(w2));
        assert_eq!(calls_to(&e, react3), react(w3));
        assert!(calls_to(&e, react4).is_empty());
        assert_eq!(calls_to(&e, say1).len(), 1);
        assert!(calls_to(&e, say2).is_empty());
        assert!(calls_to(&e, say3).is_empty());
        assert_eq!(calls_to(&e, say4).len(), 1);
        assert_eq!(calls_to(&e, LIST_CLEAR), vec![vec![head]]);
        assert_eq!(calls_to(&e, LIST_DESTROY), vec![vec![head, 1]]);
        assert_eq!(
            calls_to(&e, ADD_FACTION_MAJOR_CRIME),
            vec![vec![rig.victim, 1, 1]]
        );
        assert_eq!(e.get(rig.player, Actor::iMajorCrimes), 1);
        assert_eq!(thread_word(&mut e), 0);
    }

    #[test]
    fn crime_of_kind_4_needs_the_player_or_a_forced_actor() {
        let mut e = engine();
        let rig = attack_rig(&mut e);
        double(&mut e, PROCESS_LISTS_WITNESSES, eax(0));
        let other = new_actor(&mut e);
        quiet_slots(&mut e, other.addr(), &[0x304]);
        slot(&mut e, other.addr() + 0xa4, 8, eax(0));
        log_calls(&mut e);
        fn_008c09e0(&mut e, Ptr::new(rig.victim), Ptr::NULL);
        fn_008c09e0(&mut e, Ptr::new(rig.victim), other.cast());
        assert!(calls_to(&e, CRIME_CONSTRUCTOR).is_empty());
        // An actor whose process wants an update next can commit it.
        double(&mut e, GET_FORCE_NEXT_UPDATE, eax(1));
        fn_008c09e0(&mut e, Ptr::new(rig.victim), other.cast());
        assert_eq!(
            calls_to(&e, CRIME_CONSTRUCTOR)[0][1..],
            [4, rig.victim, other.addr(), 0, 0, 0]
        );
        // A strong actor flagged by the 004997b0 test is spared.
        slot(&mut e, other.addr() + 0xa4, 8, eax(100));
        double(&mut e, ACTOR_FLAG_TEST, eax(1));
        log_calls(&mut e);
        fn_008c09e0(&mut e, Ptr::new(rig.victim), other.cast());
        assert!(calls_to(&e, CRIME_CONSTRUCTOR).is_empty());
    }

    // ---- 008c0ec0 / 008c1010 / 008c1210 ----------------------------------

    #[test]
    fn faction_members_are_told_to_attack_the_player() {
        let mut e = engine();
        let player = new_player(&mut e);
        let (a1, p1) = witness(&mut e);
        let (a2, _p2) = witness(&mut e);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, a1);
        e.mem.set_u32(list + 4, a2);
        double(&mut e, PROCESS_LISTS_ACTOR_ARRAY, eax(list));
        double(&mut e, PROCESS_LISTS_ACTOR_COUNT, eax(2));
        double(&mut e, LEVEL_TOWARD_ACTOR, eax(1));
        // Both are of the faction; the second is dead.
        slot(&mut e, a2, 0x320, eax(1));
        let faction = e.mem.alloc(0x40);
        e.mem.set_u8(faction + 4, 8);
        double(&mut e, IS_IN_FACTION, eax(1));
        let order = slot(&mut e, p1, 0x33c, Ret::default());
        log_calls(&mut e);
        let result = fn_008c0ec0(&mut e, 0, Ptr::NULL, Ptr::new(faction), 0);
        assert_eq!(result, 0xffff_ffff);
        assert_eq!(calls_to(&e, LOCK_TAKE), vec![vec![BROADCAST_LOCK, 0]]);
        assert_eq!(calls_to(&e, LOCK_RELEASE), vec![vec![BROADCAST_LOCK]]);
        assert_eq!(
            calls_to(&e, order),
            vec![vec![p1, a1, player.addr(), 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0]]
        );
        assert_eq!(calls_to(&e, ADD_FACTION_MINOR_CRIME), vec![vec![a1, 1, 1]]);
        assert_eq!(e.get(player, Actor::iMinorCrimes), 1);
    }

    #[test]
    fn nobody_charged_when_nobody_qualifies() {
        let mut e = engine();
        let player = new_player(&mut e);
        let (a1, _p1) = witness(&mut e);
        let list = e.mem.alloc(4);
        e.mem.set_u32(list, a1);
        double(&mut e, PROCESS_LISTS_ACTOR_ARRAY, eax(list));
        double(&mut e, PROCESS_LISTS_ACTOR_COUNT, eax(1));
        double(&mut e, LEVEL_TOWARD_ACTOR, eax(0));
        let faction = e.mem.alloc(0x40);
        e.mem.set_u8(faction + 4, 8);
        double(&mut e, IS_IN_FACTION, eax(1));
        log_calls(&mut e);
        fn_008c0ec0(&mut e, 0, Ptr::NULL, Ptr::new(faction), 0);
        assert!(calls_to(&e, ADD_FACTION_MINOR_CRIME).is_empty());
        assert_eq!(e.get(player, Actor::iMinorCrimes), 0);
    }

    #[test]
    fn actor_of_a_faction_form() {
        let mut e = engine();
        let actor = e.mem.alloc(0x40);
        let faction = e.mem.alloc(0x40);
        e.mem.set_u8(faction + 4, 8);
        double(&mut e, IS_IN_FACTION, eax(1));
        assert!(is_actor_of_faction(
            &mut e,
            Ptr::NULL,
            Ptr::new(faction),
            Ptr::new(actor)
        ));
        double(&mut e, IS_IN_FACTION, eax(0));
        assert!(!is_actor_of_faction(
            &mut e,
            Ptr::NULL,
            Ptr::new(faction),
            Ptr::new(actor)
        ));
        assert!(!is_actor_of_faction(
            &mut e,
            Ptr::NULL,
            Ptr::NULL,
            Ptr::new(actor)
        ));
    }

    #[test]
    fn actor_of_the_factions_of_an_npc_form() {
        let mut e = engine();
        let actor = e.mem.alloc(0x40);
        let npc = e.mem.alloc(0x80);
        e.mem.set_u8(npc + 4, 0x2a);
        let held = e.mem.alloc(8);
        e.mem.set_u32(held, 0xfac);
        let list = node(&mut e, held, 0);
        double(&mut e, FACTION_LIST_OF, eax(list));
        e.register_double(IS_IN_FACTION, move |_, a| eax((a[1] == 0xfac) as u32));
        log_calls(&mut e);
        assert!(is_actor_of_faction(
            &mut e,
            Ptr::NULL,
            Ptr::new(npc),
            Ptr::new(actor)
        ));
        assert_eq!(calls_to(&e, FACTION_LIST_OF), vec![vec![npc + 0x30]]);
        e.mem.set_u32(held, 0xbad);
        assert!(!is_actor_of_faction(
            &mut e,
            Ptr::NULL,
            Ptr::new(npc),
            Ptr::new(actor)
        ));
    }

    #[test]
    fn actor_of_the_integrated_factions_of_a_reference() {
        let mut e = engine();
        let actor = e.mem.alloc(0x40);
        let reference = e.mem.alloc(0x80);
        e.register(RT_DYNAMIC_CAST, |_, a| eax(a[0]));
        double(&mut e, GET_BASE_FORM, eax(0x8000));
        double(&mut e, FACTION_LIST_OF, eax(0x7000));
        let changes = e.mem.alloc(0x20);
        e.mem.set_u32(changes + 0xc, 0xc4);
        double(&mut e, GET_FACTION_CHANGES_EXTRA, eax(changes));
        e.register_double(INTEGRATE_FACTION_LISTS, |e, a| {
            e.mem.set_u32(a[1], 0x11);
            e.mem.set_u32(a[1] + 4, 0x22);
            eax(2)
        });
        e.register_double(IS_IN_FACTION, |_, a| eax((a[1] == 0x22) as u32));
        log_calls(&mut e);
        assert!(is_actor_of_faction(
            &mut e,
            Ptr::new(reference),
            Ptr::NULL,
            Ptr::new(actor)
        ));
        let integrated = calls_to(&e, INTEGRATE_FACTION_LISTS);
        assert_eq!(integrated[0][0], reference);
        assert_eq!(integrated[0][2..], [0x80, 0x7000, 0xc4]);
        assert_eq!(calls_to(&e, FACTION_LIST_OF), vec![vec![0x8030]]);
        // Without a match the answer is false.
        e.register_double(IS_IN_FACTION, |_, _| eax(0));
        assert!(!is_actor_of_faction(
            &mut e,
            Ptr::new(reference),
            Ptr::NULL,
            Ptr::new(actor)
        ));
    }

    #[test]
    fn actor_of_the_factions_of_an_npc_reference() {
        let mut e = engine();
        let actor = e.mem.alloc(0x40);
        let reference = e.mem.alloc(0x80);
        e.mem.set_u8(reference + 4, 0x2a);
        // The cast to an actor fails, the cast to the NPC type succeeds.
        e.register(RT_DYNAMIC_CAST, |_, a| {
            eax(if a[3] == CAST_TO_NPC { a[0] } else { 0 })
        });
        let held = e.mem.alloc(8);
        e.mem.set_u32(held, 0xfac);
        let list = node(&mut e, held, 0);
        double(&mut e, FACTION_LIST_OF, eax(list));
        double(&mut e, IS_IN_FACTION, eax(1));
        assert!(is_actor_of_faction(
            &mut e,
            Ptr::new(reference),
            Ptr::NULL,
            Ptr::new(actor)
        ));
        // A reference that is neither an actor nor an `NPC_`: false.
        e.mem.set_u8(reference + 4, 0x10);
        assert!(!is_actor_of_faction(
            &mut e,
            Ptr::new(reference),
            Ptr::NULL,
            Ptr::new(actor)
        ));
    }

    #[test]
    fn faction_list_walk_stops_at_the_first_member() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, 0x11);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0x22);
        let n2 = node(&mut e, second, 0);
        let n1 = node(&mut e, first, n2);
        e.register_double(IS_IN_FACTION, |_, a| eax((a[1] == 0x22) as u32));
        assert!(fn_008c1210(&mut e, Ptr::new(n1), actor, 0));
        e.register_double(IS_IN_FACTION, |_, _| eax(0));
        assert!(!fn_008c1210(&mut e, Ptr::new(n1), actor, 0));
        // An empty item slot ends the walk, and so does a null node.
        e.mem.set_u32(n1, 0);
        e.register_double(IS_IN_FACTION, |_, _| eax(1));
        assert!(!fn_008c1210(&mut e, Ptr::new(n1), actor, 0));
        assert!(!fn_008c1210(&mut e, Ptr::NULL, actor, 0));
    }

    // ---- 008c1270 .. 008c1680 --------------------------------------------

    #[test]
    fn process_reset_reads_values_for_a_type_zero_process() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        let process = give_process(&mut e, actor);
        let refresh = slot(&mut e, actor.addr(), 0x20c, Ret::default());
        let read = slot(&mut e, actor.addr() + 0xa4, 0xc, st0(1.0));
        let notify = slot(&mut e, process, 0x408, Ret::default());
        log_calls(&mut e);
        fn_008c1270(&mut e, actor);
        assert_eq!(
            calls_to(&e, CACHED_VALUES_OWNER_REFRESH),
            vec![vec![actor.addr() + 0xa8]]
        );
        assert_eq!(calls_to(&e, refresh).len(), 1);
        let reads = calls_to(&e, read);
        assert_eq!(reads.len(), 0x4d);
        assert_eq!(reads[0], vec![actor.addr() + 0xa4, 0]);
        assert_eq!(reads[0x4c], vec![actor.addr() + 0xa4, 0x4c]);
        let notified = calls_to(&e, notify);
        assert_eq!(notified.len(), 13);
        assert_eq!(notified[0], vec![process, 3]);
        assert_eq!(notified[12], vec![process, 15]);
        // Another process type: only the refresh.
        double(&mut e, GET_CURRENT_PROCESS_TYPE, eax(2));
        log_calls(&mut e);
        fn_008c1270(&mut e, actor);
        assert!(calls_to(&e, read).is_empty());
        assert!(calls_to(&e, notify).is_empty());
        assert_eq!(calls_to(&e, refresh).len(), 1);
    }

    #[test]
    fn process_reset_without_a_process_skips_the_owner_refresh() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        slot(&mut e, actor.addr(), 0x20c, Ret::default());
        double(&mut e, GET_CURRENT_PROCESS_TYPE, eax(1));
        log_calls(&mut e);
        fn_008c1270(&mut e, actor);
        assert!(calls_to(&e, CACHED_VALUES_OWNER_REFRESH).is_empty());
    }

    #[test]
    fn combat_state_is_forgotten() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        e.set(actor, Actor::pMyKiller, Ptr::new(5));
        e.set(actor, Actor::pCurrentCombatTarget, Ptr::new(6));
        let targets = e.mem.alloc(0x10);
        let members = e.mem.alloc(0x10);
        let free_targets = slot(&mut e, targets, 0, Ret::default());
        let free_members = slot(&mut e, members, 0, Ret::default());
        e.set(actor, Actor::pCurrentCombatTargetArray, Ptr::new(targets));
        e.set(actor, Actor::pCurrentCombatMemberArray, Ptr::new(members));
        log_calls(&mut e);
        fn_008c1320(&mut e, actor);
        assert_eq!(e.get(actor, Actor::pMyKiller), Ptr::NULL);
        assert_eq!(e.get(actor, Actor::pCurrentCombatTarget), Ptr::NULL);
        assert_eq!(e.get(actor, Actor::pCurrentCombatTargetArray), Ptr::NULL);
        assert_eq!(e.get(actor, Actor::pCurrentCombatMemberArray), Ptr::NULL);
        assert_eq!(
            calls_to(&e, CLEAR_DISPOSITION_MODIFIERS),
            vec![vec![actor.addr()]]
        );
        assert_eq!(calls_to(&e, free_targets), vec![vec![targets, 1]]);
        assert_eq!(calls_to(&e, free_members), vec![vec![members, 1]]);
        // Nothing to destroy the second time.
        log_calls(&mut e);
        fn_008c1320(&mut e, actor);
        assert!(calls_to(&e, free_targets).is_empty());
    }

    fn furniture_case(state: u32, object_byte: Option<u8>) -> bool {
        let mut e = engine();
        let actor = new_actor(&mut e);
        slot(&mut e, actor.addr(), 0x214, eax(state));
        let process = give_process(&mut e, actor);
        let object = match object_byte {
            Some(byte) => {
                let object = e.mem.alloc(0x20);
                e.mem.set_u8(object + 0xe, byte);
                object
            }
            None => 0,
        };
        slot(&mut e, process, 0x4d4, eax(object));
        actor_is_trying_to_enter_furniture(&mut e, actor)
    }

    #[test]
    fn furniture_entry_needs_a_state_and_a_low_byte() {
        assert!(!furniture_case(0, Some(1)));
        assert!(!furniture_case(4, Some(1)));
        assert!(!furniture_case(9, Some(1)));
        assert!(!furniture_case(5, None));
        assert!(furniture_case(5, Some(0x14)));
        assert!(!furniture_case(5, Some(0x15)));
    }

    #[test]
    fn combat_links_are_taken_from_the_controller() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        e.mem.set_u8(actor.addr() + 0x104, 1);
        slot(&mut e, actor.addr(), 0x428, eax(0x2000));
        double(&mut e, COMBAT_CONTROLLER_GET_WORD, eax(0x77));
        double(&mut e, COMBAT_CONTROLLER_GET_FLAG, eax(1));
        double(&mut e, COMBAT_CONTROLLER_GET_GROUP, eax(0x6000));
        log_calls(&mut e);
        fn_008c1470(&mut e, actor);
        assert_eq!(e.get(actor, Actor::pCurrentCombatTarget), Ptr::new(0x77));
        assert!(e.get(actor, Actor::bSearchingInCombat));
        let targets = e.get(actor, Actor::pCurrentCombatTargetArray).addr();
        let members = e.get(actor, Actor::pCurrentCombatMemberArray).addr();
        assert_ne!(targets, 0);
        assert_ne!(members, 0);
        assert_ne!(targets, members);
        assert_eq!(e.mem.u32(targets), SIMPLE_ARRAY_VTABLE);
        assert_eq!(
            calls_to(&e, COMBAT_GROUP_FILL_TARGET_ARRAY),
            vec![vec![0x6000, targets]]
        );
        assert_eq!(
            calls_to(&e, COMBAT_GROUP_FILL_MEMBER_ARRAY),
            vec![vec![0x6000, members]]
        );
        // The arrays already exist the second time: they are refilled.
        log_calls(&mut e);
        fn_008c1470(&mut e, actor);
        assert_eq!(
            e.get(actor, Actor::pCurrentCombatTargetArray).addr(),
            targets
        );
        assert_eq!(calls_to(&e, OPERATOR_NEW).len(), 0);
        assert_eq!(calls_to(&e, COMBAT_GROUP_FILL_TARGET_ARRAY).len(), 1);
    }

    #[test]
    fn combat_links_without_controller_or_group() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        e.mem.set_u8(actor.addr() + 0x104, 1);
        let controller = slot(&mut e, actor.addr(), 0x428, eax(0));
        log_calls(&mut e);
        fn_008c1470(&mut e, actor);
        assert_eq!(calls_to(&e, controller).len(), 1);
        assert!(calls_to(&e, COMBAT_CONTROLLER_GET_WORD).is_empty());
        slot(&mut e, actor.addr(), 0x428, eax(0x2000));
        double(&mut e, COMBAT_CONTROLLER_GET_GROUP, eax(0));
        log_calls(&mut e);
        fn_008c1470(&mut e, actor);
        assert!(calls_to(&e, OPERATOR_NEW).is_empty());
    }

    #[test]
    fn leaving_combat_clears_the_links() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        e.set(actor, Actor::bSearchingInCombat, true);
        e.set(actor, Actor::pCurrentCombatTarget, Ptr::new(6));
        let targets = e.mem.alloc(0x10);
        let free_targets = slot(&mut e, targets, 0, Ret::default());
        e.set(actor, Actor::pCurrentCombatTargetArray, Ptr::new(targets));
        log_calls(&mut e);
        fn_008c1470(&mut e, actor);
        assert!(!e.get(actor, Actor::bSearchingInCombat));
        assert_eq!(e.get(actor, Actor::pCurrentCombatTarget), Ptr::NULL);
        assert_eq!(e.get(actor, Actor::pCurrentCombatTargetArray), Ptr::NULL);
        assert_eq!(calls_to(&e, free_targets), vec![vec![targets, 1]]);
    }

    #[test]
    fn member_search_passes_the_key_by_address() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        assert!(!fn_008c1680(&mut e, actor, 0x42));
        let members = e.mem.alloc(0x10);
        e.set(actor, Actor::pCurrentCombatMemberArray, Ptr::new(members));
        e.register_double(ARRAY_FIND_WITH_COMPARE, |e, a| {
            assert_eq!(a[2..], [0, ARRAY_COMPARE_CALLBACK]);
            eax(if e.mem.u32(a[1]) == 0x42 { 3 } else { u32::MAX })
        });
        assert!(fn_008c1680(&mut e, actor, 0x42));
        assert!(!fn_008c1680(&mut e, actor, 0x43));
    }

    // ---- 008c16c0 / 008c1740 ---------------------------------------------

    fn script_rig(e: &mut Engine, variable: u32) {
        double(e, SCRIPT_OF_FORM, eax(0x500));
        double(e, GET_SCRIPT_VARIABLES, eax(0x600));
        e.register_double(SCRIPT_FIND_VARIABLE, move |e, a| {
            assert_eq!(a[0], 0x500);
            let found = if a[1] == variable { 0x900 } else { 0 };
            e.mem.set_u32(a[2], found);
            Ret::default()
        });
        double(e, SCRIPT_LOCALS_GET_VARIABLE, st0(3.5));
    }

    #[test]
    fn script_variable_is_set() {
        let mut e = engine();
        script_rig(&mut e, 7);
        let reference = e.mem.alloc(0x40);
        log_calls(&mut e);
        assert!(fn_008c16c0(&mut e, Ptr::new(reference), 7, 2.5));
        let bits = 2.5f64.to_bits();
        assert_eq!(
            calls_to(&e, SCRIPT_LOCALS_SET_VARIABLE),
            vec![vec![0x600, 0x900, bits as u32, (bits >> 32) as u32]]
        );
        assert!(!fn_008c16c0(&mut e, Ptr::new(reference), 8, 2.5));
        assert!(!fn_008c16c0(&mut e, Ptr::NULL, 7, 2.5));
        double(&mut e, SCRIPT_OF_FORM, eax(0));
        assert!(!fn_008c16c0(&mut e, Ptr::new(reference), 7, 2.5));
        double(&mut e, SCRIPT_OF_FORM, eax(0x500));
        double(&mut e, GET_SCRIPT_VARIABLES, eax(0));
        assert!(!fn_008c16c0(&mut e, Ptr::new(reference), 7, 2.5));
    }

    #[test]
    fn script_variable_is_read() {
        let mut e = engine();
        script_rig(&mut e, 7);
        let reference = e.mem.alloc(0x40);
        log_calls(&mut e);
        assert_eq!(fn_008c1740(&mut e, Ptr::new(reference), 7), 3.5);
        assert_eq!(
            calls_to(&e, SCRIPT_LOCALS_GET_VARIABLE),
            vec![vec![0x600, 0x900, 0]]
        );
        assert_eq!(fn_008c1740(&mut e, Ptr::new(reference), 8), -1.0);
        assert_eq!(fn_008c1740(&mut e, Ptr::NULL, 7), -1.0);
    }

    // ---- 008c17c0 .. 008c1aa0 --------------------------------------------

    fn speed_rig(e: &mut Engine, actor: Ptr<Actor>) {
        slot(e, actor.addr() + 0xa4, 0xc, st0(40.0));
        put_setting_float(e, SETTING_SPEED_BASE, 15.0);
        put_setting_float(e, SETTING_SPEED_FACTOR, 0.5);
    }

    #[test]
    fn player_animation_speed_goes_through_the_entry_point() {
        let mut e = engine();
        let player = new_player(&mut e);
        speed_rig(&mut e, player);
        let process = give_process(&mut e, player);
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry + 8, 0x5ad);
        slot(&mut e, process, 0x148, eax(entry));
        let near = e.mem.alloc(0x200);
        let far = e.mem.alloc(0x200);
        e.register_double(PLAYER_GET_ANIMATION, move |_, a| {
            eax(if a[1] == 1 { near } else { far })
        });
        // The perk doubles the value: 1 + (40 - 15) * 0.5 = 13.5, then 27.
        e.register_double(HANDLE_ENTRY_POINT, |e, a| {
            let value = e.mem.f32(a[3]);
            e.mem.set_f32(a[3], value * 2.0);
            Ret::default()
        });
        log_calls(&mut e);
        fn_008c17c0(&mut e, player);
        let handled = calls_to(&e, HANDLE_ENTRY_POINT);
        assert_eq!(handled[0][..3], [0x25, player.addr(), 0x5ad]);
        assert_eq!(e.mem.f32(near + 0x118), 27.0);
        assert_eq!(e.mem.f32(far + 0x118), 27.0);
        assert_eq!(e.mem.f32(near + 0x11c), 0.0);
        fn_008c1940(&mut e, player);
        assert_eq!(calls_to(&e, HANDLE_ENTRY_POINT)[1][0], 0x26);
        assert_eq!(e.mem.f32(near + 0x11c), 27.0);
        assert_eq!(e.mem.f32(far + 0x11c), 27.0);
    }

    #[test]
    fn actor_animation_speed_uses_the_actor_animation() {
        let mut e = engine();
        new_player(&mut e);
        let actor = new_actor(&mut e);
        speed_rig(&mut e, actor);
        let animation = e.mem.alloc(0x200);
        slot(&mut e, actor.addr(), 0x1e4, eax(animation));
        fn_008c17c0(&mut e, actor);
        assert_eq!(e.mem.f32(animation + 0x118), 13.5);
        // No animation object: nothing is stored.
        slot(&mut e, actor.addr(), 0x1e4, eax(0));
        e.mem.set_f32(animation + 0x118, 0.0);
        fn_008c17c0(&mut e, actor);
        assert_eq!(e.mem.f32(animation + 0x118), 0.0);
    }

    #[test]
    fn animation_speed_setters() {
        let mut e = engine();
        let animation = e.mem.alloc(0x200);
        fn_008c1920(&mut e, Ptr::new(animation), 1.5);
        fn_008c1aa0(&mut e, Ptr::new(animation), 2.5);
        assert_eq!(e.mem.f32(animation + 0x118), 1.5);
        assert_eq!(e.mem.f32(animation + 0x11c), 2.5);
    }

    // ---- 008c1ac0 .. 008c1c80 --------------------------------------------

    #[test]
    fn major_crime_flag_bits() {
        let mut e = engine();
        let actor = new_actor(&mut e);
        e.set(actor, Actor::iMajorCrimes, 7);
        fn_008c1ac0(&mut e, actor, true);
        assert_eq!(e.get(actor, Actor::iMajorCrimes), 0x8000_0007);
        assert!(fn_008c1b30(&mut e, actor));
        assert!(!actor_always_show_subtitles(&mut e, actor));
        fn_008c1b50(&mut e, actor, true);
        assert_eq!(e.get(actor, Actor::iMajorCrimes), 0xc000_0007);
        assert!(actor_always_show_subtitles(&mut e, actor));
        fn_008c1ac0(&mut e, actor, false);
        assert_eq!(e.get(actor, Actor::iMajorCrimes), 0x4000_0007);
        assert!(!fn_008c1b30(&mut e, actor));
        fn_008c1b50(&mut e, actor, false);
        assert_eq!(e.get(actor, Actor::iMajorCrimes), 7);
    }

    #[test]
    fn word_and_mask() {
        let mut e = engine();
        let word = e.mem.alloc(8);
        e.mem.set_u32(word, 0xf0f0);
        assert_eq!(fn_008c1be0(&mut e, Ptr::new(word), 0x0ff0), 0x00f0);
    }

    #[test]
    fn actor_array_constructor_sets_the_vtable() {
        let mut e = engine();
        let array = e.mem.alloc(0x10);
        log_calls(&mut e);
        assert_eq!(fn_008c1c00(&mut e, Ptr::new(array)), Ptr::new(array));
        assert_eq!(e.mem.u32(array), SIMPLE_ARRAY_VTABLE);
        assert_eq!(
            calls_to(&e, SIMPLE_ARRAY_INITIALISE),
            vec![vec![array, 0, 0]]
        );
    }

    #[test]
    fn hit_data_pointer_counts_references() {
        let mut e = engine();
        let slot_word = e.mem.alloc(8);
        log_calls(&mut e);
        assert_eq!(
            fn_008c1c30(&mut e, Ptr::new(slot_word), 0x1234),
            Ptr::new(slot_word)
        );
        assert_eq!(e.mem.u32(slot_word), 0x1234);
        assert_eq!(calls_to(&e, HIT_DATA_ADD_REFERENCE), vec![vec![0x1234]]);
        fn_008c1c30(&mut e, Ptr::new(slot_word), 0);
        assert_eq!(calls_to(&e, HIT_DATA_ADD_REFERENCE).len(), 1);
        assert_eq!(e.mem.u32(slot_word), 0);
        e.mem.set_u32(slot_word, 0x1234);
        ni_pointer_hit_data_destructor(&mut e, Ptr::new(slot_word));
        assert_eq!(calls_to(&e, HIT_DATA_RELEASE), vec![vec![0x1234]]);
        e.mem.set_u32(slot_word, 0);
        ni_pointer_hit_data_destructor(&mut e, Ptr::new(slot_word));
        assert_eq!(calls_to(&e, HIT_DATA_RELEASE).len(), 1);
    }

    #[test]
    fn actor_array_deleting_destructor_frees_on_request() {
        let mut e = engine();
        let array = e.mem.alloc(0x10);
        log_calls(&mut e);
        assert_eq!(
            bs_simple_array_actor_scalar_deleting_destructor(&mut e, Ptr::new(array), 0),
            Ptr::new(array)
        );
        assert_eq!(
            calls_to(&e, SIMPLE_ARRAY_DESTRUCTOR_BODY),
            vec![vec![array]]
        );
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        bs_simple_array_actor_scalar_deleting_destructor(&mut e, Ptr::new(array), 1);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![array]]);
    }

    // ---- 008cfad0 / 008e2680 -------------------------------------------

    /// An actor whose virtual `+0x1d0` answers `node` and whose `+0x88` word
    /// points at a record starting with `record_kind`.
    fn hit_sound_actor(e: &mut Engine, node: u32, record_kind: u32) -> Ptr<Actor> {
        let actor = new_actor(e);
        let table = e.mem.alloc(0x200);
        e.mem.set_u32(table + 0x1d0, 0x0900_0000);
        e.mem.set_u32(actor.addr(), table);
        e.register_double(0x0900_0000, move |_, _| eax(node));
        let record = e.mem.alloc(8);
        e.mem.set_u32(record, record_kind);
        e.mem.set_u32(actor.addr() + 0x88, record);
        actor
    }

    #[test]
    fn hit_sound_needs_a_node_and_a_record_of_kind_one() {
        let mut e = engine();
        let actor = hit_sound_actor(&mut e, 0, 1);
        log_calls(&mut e);
        actor_fake_weapon_hit_sound(&mut e, actor, 1.0, 1, 2, 3, 4, 5);
        assert!(calls_to(&e, FIND_FIRST_COLLISION_OBJECT).is_empty());
        let actor = hit_sound_actor(&mut e, 0x77, 2);
        actor_fake_weapon_hit_sound(&mut e, actor, 1.0, 1, 2, 3, 4, 5);
        assert!(calls_to(&e, FIND_FIRST_COLLISION_OBJECT).is_empty());
        e.mem.set_u32(actor.addr() + 0x88, 0);
        actor_fake_weapon_hit_sound(&mut e, actor, 1.0, 1, 2, 3, 4, 5);
        assert!(calls_to(&e, FIND_FIRST_COLLISION_OBJECT).is_empty());
    }

    #[test]
    fn hit_sound_without_collision_target_plays_nothing() {
        let mut e = engine();
        let actor = hit_sound_actor(&mut e, 0x77, 1);
        log_calls(&mut e);
        actor_fake_weapon_hit_sound(&mut e, actor, 1.0, 1, 2, 3, 4, 5);
        assert_eq!(calls_to(&e, FIND_FIRST_COLLISION_OBJECT), vec![vec![0x77]]);
        assert!(calls_to(&e, COLLISION_OBJECT_GET_TARGET).is_empty());
        e.register(FIND_FIRST_COLLISION_OBJECT, |_, _| eax(0x88));
        actor_fake_weapon_hit_sound(&mut e, actor, 1.0, 1, 2, 3, 4, 5);
        assert_eq!(calls_to(&e, COLLISION_OBJECT_GET_TARGET), vec![vec![0x88]]);
        assert!(calls_to(&e, IMPACT_MIXER_PLAY_COLLISION_SOUND).is_empty());
    }

    #[test]
    fn hit_sound_fills_the_request() {
        let mut e = engine();
        e.map(0x0101_6000, 0x1000);
        e.set_global(HALF_FLOAT, 0.5f32);
        let actor = hit_sound_actor(&mut e, 0x77, 1);
        e.register(FIND_FIRST_COLLISION_OBJECT, |_, _| eax(0x88));
        e.register(COLLISION_OBJECT_GET_TARGET, |_, _| eax(0x99));
        e.register(TARGET_OBJECT_GET_OBJECT, |_, _| eax(0xaa));
        e.register(OBJECT_GET_KIND, |_, a| {
            eax(if a[0] == 0xaa { 0x13 } else { 0 })
        });
        let seen = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
        let sink = seen.clone();
        e.register_double(IMPACT_MIXER_PLAY_COLLISION_SOUND, move |e, a| {
            let words: Vec<u32> = (0..9).map(|i| e.mem.u32(a[0] + 4 * i)).collect();
            sink.lock().unwrap().push(words);
            Ret::default()
        });
        actor_fake_weapon_hit_sound(&mut e, actor, 2.0, 0x11, 0x22, 0x33, 0x40, 1);
        let seen = seen.lock().unwrap();
        assert_eq!(seen.len(), 1);
        let w = &seen[0];
        assert_eq!(&w[0..3], &[0x11, 0x22, 0x33]);
        assert_eq!(f32::from_bits(w[3]), 2.0);
        assert_eq!(f32::from_bits(w[4]), 0.5);
        assert_eq!(w[5], 0x0113);
        assert_eq!(w[6], actor.addr() + 0x40);
        assert_eq!(w[7], 0x99);
        assert_eq!(w[8], 0x40);
    }

    #[test]
    fn does_fly_stores_the_word() {
        let mut e = engine();
        let process = e.mem.alloc(0x400);
        fn_008e2680(&mut e, Ptr::new(process), 7);
        assert_eq!(e.mem.u32(process + 0x3f0), 7);
    }
}
