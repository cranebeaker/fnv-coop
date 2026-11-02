//! `fallout shared/tesconditionfunctions.cpp` (Xbox PDB source unit), part 2: its functions from `005a2a50` up to
//! (not including) `ffffffff` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::tesconditionfunctions`]; anything public there may be used here.
//!
//! Progress: this part holds `005a2a50` to `005a60f0` (the first session
//! `005a2a50` to `005a3c90`, the second `005a3d00` to `005a4f30`, 40 functions
//! each; the third `005a5020` to `005a60f0`, 37 functions). Every function
//! of the range is translated; `00f46450` is a `library` entry.
//!
//! The main file keeps its constants and helpers private, so this part
//! declares again the ones it needs. Every condition function has the shape
//! `bool f(TESObjectREFR* ref, void* param1, void* param2, double* result)`
//! (cdecl, four words), which are the parameters below; the value goes to
//! `*result` and the function returns `true`. The game computes `*result` in
//! extended precision; every value here is a whole number or 1.0 / 0.0 / 2.0,
//! so `f64` is exact.

#[allow(unused_imports)]
use super::tesconditionfunctions::*;
#[allow(unused_imports)]
use crate::prelude::*;

/// Offset in the TLS block of the byte that turns on the condition
/// functions' debug lines.
const TLS_TRACE_FLAG: u32 = 0x268;
/// Offset in the TLS block of the form `IsActorVictim` compares with.
const TLS_CRIME_VICTIM: u32 = 0x26c;
/// The debug print (`printf`-like, varargs, `interface.cpp`).
const DEBUG_PRINT: u32 = 0x0070_3c00;
/// `PlayerCharacter*` global.
const PLAYER: u32 = 0x011d_ea3c;
/// The `Calendar` singleton.
const CALENDAR: u32 = 0x011d_e7b8;

/// Virtual `IsActor` (vtable `+0x100`, Xbox PDB name).
const VSLOT_IS_ACTOR: u32 = 0x100;
/// Virtual (vtable `+0x218`) on an actor: a byte test (`IsActorVictim`
/// requires it).
const VSLOT_ACTOR_TEST_218: u32 = 0x218;
/// Virtual (vtable `+0x22c`) on an actor, called with 0: a byte test.
const VSLOT_ACTOR_TEST_22C: u32 = 0x22c;
/// Virtual (vtable `+0x60`) on the actor's middle-high process: a byte test.
const VSLOT_PROCESS_TEST_60: u32 = 0x60;
/// Virtual (vtable `+0x69c`) on the actor's middle-high process: a `float`
/// in ST0.
const VSLOT_PROCESS_FLOAT_69C: u32 = 0x69c;

/// `TESForm::cFormType` (byte at `+0x04`) of the form in ECX.
const FORM_GET_TYPE: u32 = 0x0040_1170;
/// The form type `fn_005a3b90` requires of a reference's base form.
const FORM_TYPE_SPEAKER_SOURCE: u32 = 0x16;
/// The form type `GetArmorRatingUpper` requires of the worn item's form.
const FORM_TYPE_ARMOR: u32 = 0x18;
/// Reads `[this + 0x20]` (a reference's base form).
const REF_GET_BASE_FORM: u32 = 0x007a_f430;
/// Reads `[this + 0x08]` of the object in ECX (the worn `ItemChange`'s item).
const OBJECT_GET_FIELD_08: u32 = 0x0044_ddc0;
/// The name of a form (cdecl, one form argument; a fixed string for null).
const FORM_GET_FULL_NAME: u32 = 0x0048_2720;
/// The name of a reference (`tesobjectrefr.cpp`, `this` the reference).
const REF_GET_NAME: u32 = 0x0055_d520;
/// `this + 0x44`: the reference's `ExtraDataList`.
const REF_GET_EXTRA_DATA_LIST: u32 = 0x005d_43c0;
/// `ExtraDataList` owner getter (`extradatalist.cpp`): the owner form or null.
const EXTRA_LIST_GET_OWNER: u32 = 0x0041_8660;
/// `ExtraDataList::GetFriendHitCount` (Xbox PDB).
const EXTRA_LIST_GET_FRIEND_HIT_COUNT: u32 = 0x0042_2640;
/// `[this + 0x40]` of a reference: its parent cell.
const REF_GET_PARENT_CELL: u32 = 0x008d_6f30;
/// `TESObjectCELL::GetOwner` (Xbox PDB).
const CELL_GET_OWNER: u32 = 0x0054_6a40;
/// True when the cell in ECX is an interior (`extradatalist.cpp`).
const CELL_IS_INTERIOR: u32 = 0x0042_5fd0;
/// `TESObjectCELL::GetWaterType` (Xbox PDB).
const CELL_GET_WATER_TYPE: u32 = 0x0054_7770;
/// `TESWaterForm::GetDangerous` (Xbox PDB).
const WATER_FORM_GET_DANGEROUS: u32 = 0x0058_0060;

/// The actor's middle-high process, or null (the engine map names this body
/// `MiddleHighProcess::GetSavedAcquireObject` because of folded code).
const ACTOR_GET_PROCESS: u32 = 0x008d_8520;
/// `actor.cpp`: the result of virtual `+0x770` of the actor's process, or
/// null without a process.
const ACTOR_GET_PROCESS_PART: u32 = 0x008a_5270;
/// Byte tests of an `Actor` (`actor.cpp`): ghost, offers services, and the
/// one behind `fn_005a35f0`.
const ACTOR_IS_GHOST: u32 = 0x008a_ce90;
const ACTOR_OFFERS_SERVICES: u32 = 0x008a_c680;
const ACTOR_TEST_87F510: u32 = 0x0087_f510;
/// `Actor::GetBarterGoldBase` (Xbox PDB).
const ACTOR_GET_BARTER_GOLD_BASE: u32 = 0x0088_4320;
/// `Actor::IsRunning` (Xbox PDB).
const ACTOR_IS_RUNNING: u32 = 0x0088_4730;
/// Byte test of the actor in ECX (`animation.cpp`): movement flag `0x400`
/// set and `0x800` clear (sneaking).
const ACTOR_IS_SNEAKING: u32 = 0x0049_97b0;
/// Byte at `+0x104` of the actor (`animation.cpp`): behind `IsInCombat`.
const ACTOR_IN_COMBAT_FLAG: u32 = 0x0049_3bb0;
/// `TESObjectREFR::IsPartofEvilFaction` (Xbox PDB).
const REF_IS_PART_OF_EVIL_FACTION: u32 = 0x0056_78a0;
/// `Actor::GetEssential` (Xbox PDB).
const ACTOR_GET_ESSENTIAL: u32 = 0x0087_f3d0;
/// A whole-number value of the calendar (`this` the [`CALENDAR`]; its global
/// at `+0x10` times 24, truncated), the minuend of `fn_005a3a30`.
const CALENDAR_GET_WHOLE_VALUE: u32 = 0x0086_7e30;
/// The table of pointers `fn_005a3370` indexes; the value is at `+4` of the
/// entry.
const MISC_STAT_TABLE: u32 = 0x011c_6d50;
/// The `int` global `fn_005a3650` returns.
const INT_GLOBAL_5A3650: u32 = 0x0119_f344;

/// `InventoryChanges::GetInventoryChanges` (Xbox PDB), cdecl, one reference.
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
/// `InventoryChanges::GetWornItem` (Xbox PDB): `this` the changes and (slot,
/// 0); the worn `ItemChange` or null.
const INVENTORY_GET_WORN_ITEM: u32 = 0x004c_8c10;
/// Destructor with a delete flag: `this` the `ItemChange`, flag 1.
const ITEM_CHANGE_DESTROY: u32 = 0x0044_59e0;
/// Armor test (`0x00514410`): `2` for an armor with a flag set, else 0.
const ARMOR_GET_CLASS_FLAG: u32 = 0x0051_4410;
/// `[this + 0x90]` of the base form `fn_005a3b90` tests
/// (`tesobjectcont.cpp`).
const FORM_GET_FIELD_90: u32 = 0x0051_6bf0;

/// `PlayerCharacter` functions (`this` the player): sleeping or resting,
/// `IsPlayerCharacterInCombat` (Xbox PDB, one out byte), `fn_005a3af0`'s
/// value, the test behind `IsActionInList`, and the one behind
/// `fn_005a3bf0`.
const PLAYER_IS_SLEEPING_OR_RESTING: u32 = 0x0094_df60;
const PLAYER_IS_IN_COMBAT: u32 = 0x0095_3c50;
const PLAYER_GET_VALUE_964060: u32 = 0x0096_4060;
const PLAYER_IS_ACTION_ACTIVE: u32 = 0x0096_3ce0;
const PLAYER_TEST_966C10: u32 = 0x0096_6c10;
/// The platform test behind `IsXbox` and `IsPC` (`interface.cpp`).
const PLATFORM_IS_CONSOLE: u32 = 0x0070_9d40;
/// `[this + 0x58] & mask` of the process part (`fn_005a3c90`'s test).
const PROCESS_PART_TEST_FLAGS: u32 = 0x0058_cba0;

/// `-1.0`, `0.0` and `2.0` (`double`s in the exe's data).
const DOUBLE_MINUS_ONE: u32 = 0x0101_a6b0;
const DOUBLE_ZERO: u32 = 0x0101_2060;
const DOUBLE_TWO: u32 = 0x0101_1590;

/// The globals `fn_005a38e0` clears: the caches of other condition
/// functions of the unit (`GetDisposition`, `GetInCell`, `GetInFaction`,
/// `GetIsID`, `GetIsVoiceType`, `GetIsPlayableRace`).
const CACHE_WORDS: [u32; 15] = [
    0x011c_aae8,
    0x011c_aaec,
    0x011c_aaf0,
    0x011c_aaf4,
    0x011c_aaf8,
    0x011c_aafc,
    0x011c_ab00,
    0x011c_ab04,
    0x011c_ab08,
    0x011c_ab0c,
    0x011c_ab10,
    0x011c_ab14,
    0x011c_ab18,
    0x011c_ab1c,
    0x011c_ab20,
];

/// The byte at `TLS + 0x268`: the condition functions print a debug line
/// for their result when it is set.
fn trace_enabled(e: &mut Engine) -> bool {
    let tls = e.tls();
    e.mem.u8(tls + TLS_TRACE_FLAG) != 0
}

fn set_result(e: &mut Engine, result: Ptr, value: f64) {
    e.mem.set_f64(result.addr(), value);
}

/// With the debug lines on, the format the result selects (`zero` for a
/// result equal to 0.0, else `nonzero`); `None` with them off.
fn trace_choice(e: &mut Engine, result: Ptr, zero: u32, nonzero: u32) -> Option<u32> {
    if !trace_enabled(e) {
        return None;
    }
    Some(if e.mem.f64(result.addr()) == 0.0 {
        zero
    } else {
        nonzero
    })
}

/// The debug line `format(*result)`.
fn trace_result(e: &mut Engine, format: u32, result: Ptr) {
    let value = e.mem.f64(result.addr());
    e.call(DEBUG_PRINT, &args![format, value]);
}

/// The name of a form through the cdecl name function.
fn form_name(e: &mut Engine, form: Ptr) -> u32 {
    e.call(FORM_GET_FULL_NAME, &args![form]).u32()
}

/// `ref` when it is non-null and virtual `IsActor` says so, else null.
fn actor_of(e: &mut Engine, reference: Ptr) -> Ptr {
    if !reference.is_null() && e.vcall(reference.addr(), VSLOT_IS_ACTOR, &args![]).bool() {
        reference
    } else {
        Ptr::NULL
    }
}

// Translated from 005a2a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: always 0.0, no debug line.
pub fn fn_005a2a50(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    true
}

// Translated from 005a2a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetGhostConditionFunction` (Xbox PDB): 1.0 when the reference is
/// an actor and `008ace90` says it is a ghost.
pub fn script_get_ghost_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() && e.call(ACTOR_IS_GHOST, &args![actor]).bool() {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "GetGhost >> %0.2f"
        trace_result(e, 0x0103_5bfc, result);
    }
    true
}

// Translated from 005a2af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetOffersServiceConditionFunction` (Xbox PDB): 1.0 when the
/// reference is an actor and `008ac680` says it offers services.
pub fn script_get_offers_service_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() && e.call(ACTOR_OFFERS_SERVICES, &args![actor]).bool() {
        set_result(e, result, 1.0);
    }
    // "%s is not offering services" / "%s  is offering services"
    if let Some(format) = trace_choice(e, result, 0x0103_5c10, 0x0103_5c2c) {
        let name = e.call(REF_GET_NAME, &args![actor]).u32();
        e.call(DEBUG_PRINT, &args![format, name]);
    }
    true
}

// Translated from 005a2ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetBarterGoldConditionFunction` (Xbox PDB): the actor's base
/// barter gold. Leaves the result alone for a reference that is not an
/// actor.
pub fn script_get_barter_gold_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let gold = e.call(ACTOR_GET_BARTER_GOLD_BASE, &args![actor]).u32();
        set_result(e, result, gold as f64);
        if trace_enabled(e) {
            let gold = e.call(ACTOR_GET_BARTER_GOLD_BASE, &args![actor]).u32();
            // "%s  has %d barter gold currently"
            e.call(DEBUG_PRINT, &args![0x0103_5c48u32, actor, gold]);
        }
    }
    true
}

// Translated from 005a2c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: always 0.0 (it asks the
/// reference whether it is an actor and does nothing with the answer).
pub fn fn_005a2c30(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    actor_of(e, reference);
    set_result(e, result, 0.0);
    true
}

// Translated from 005a2c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsTimePassingConditionFunction` (Xbox PDB): 1.0 when the player
/// is sleeping or resting. Does not clear the result first.
pub fn script_is_time_passing_condition_function(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let player = e.global::<u32>(PLAYER);
    if e.call(PLAYER_IS_SLEEPING_OR_RESTING, &args![player]).bool() {
        set_result(e, result, 1.0);
    }
    // "Time is not passing" / "Time is passing"
    if let Some(format) = trace_choice(e, result, 0x0103_5c6c, 0x0103_5c80) {
        e.call(DEBUG_PRINT, &args![format]);
    }
    true
}

// Translated from 005a2ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetArmorRatingUpperConditionFunction` (Xbox PDB): for the item
/// worn in slot 2 (upper body) whose form is of the armor type: 1.0, or 2.0
/// when the armor test `00514410` answers non-zero; 0.0 otherwise. The
/// `ItemChange` obtained from the worn-item lookup is destroyed (flag 1).
pub fn script_get_armor_rating_upper_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let actor = actor_of(e, reference);
    set_result(e, result, 0.0);
    if !actor.is_null() {
        let changes = e.call(GET_INVENTORY_CHANGES, &args![reference]).ptr::<()>();
        if !changes.is_null() {
            let worn = e
                .call(INVENTORY_GET_WORN_ITEM, &args![changes, 2u32, 0u32])
                .ptr::<()>();
            if !worn.is_null() {
                let mut armor = Ptr::<()>::NULL;
                if !e
                    .call(OBJECT_GET_FIELD_08, &args![worn])
                    .ptr::<()>()
                    .is_null()
                {
                    let form = e.call(OBJECT_GET_FIELD_08, &args![worn]).ptr::<()>();
                    if e.call(FORM_GET_TYPE, &args![form]).u32() == FORM_TYPE_ARMOR {
                        armor = e.call(OBJECT_GET_FIELD_08, &args![worn]).ptr::<()>();
                    }
                }
                if !armor.is_null() {
                    if e.call(ARMOR_GET_CLASS_FLAG, &args![armor]).bool() {
                        let two: f64 = e.global(DOUBLE_TWO);
                        set_result(e, result, two);
                    } else {
                        set_result(e, result, 1.0);
                    }
                }
                e.call(ITEM_CHANGE_DESTROY, &args![worn, 1u32]);
            }
        }
    }
    if trace_enabled(e) {
        // "Armor Rating upper body is %0.2f"
        trace_result(e, 0x0103_5c90, result);
    }
    true
}

// Translated from 005a2e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name, debug lines "is the owner":
/// 1.0 when the owner of the reference's extra data list is the form
/// parameter (the base form of the player when the parameter is null). Does
/// not clear the result first.
pub fn fn_005a2e20(e: &mut Engine, reference: Ptr, form: Ptr, _param2: u32, result: Ptr) -> bool {
    let mut owner = form;
    if owner.is_null() {
        let player = e.global::<u32>(PLAYER);
        owner = e.call(REF_GET_BASE_FORM, &args![player]).ptr();
    }
    if !reference.is_null() && !owner.is_null() {
        let list = e.call(REF_GET_EXTRA_DATA_LIST, &args![reference]).u32();
        if e.call(EXTRA_LIST_GET_OWNER, &args![list]).ptr::<()>() == owner {
            set_result(e, result, 1.0);
        }
    }
    // "%s is not the owner" / "%s is the owner"
    if let Some(format) = trace_choice(e, result, 0x0103_5cb4, 0x0103_5cc8) {
        let name = form_name(e, owner);
        e.call(DEBUG_PRINT, &args![format, name]);
    }
    true
}

// Translated from 005a2ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetCellOwnershipConditionFunction` (Xbox PDB): 1.0 when the
/// owner of the cell (the first parameter) is the form (the second
/// parameter). Does not clear the result first.
pub fn script_get_cell_ownership_condition_function(
    e: &mut Engine,
    _reference: u32,
    cell: Ptr,
    owner: Ptr,
    result: Ptr,
) -> bool {
    if e.call(CELL_GET_OWNER, &args![cell]).ptr::<()>() == owner {
        set_result(e, result, 1.0);
    }
    // "%s is not the owner" / "%s is the owner"
    if let Some(format) = trace_choice(e, result, 0x0103_5cb4, 0x0103_5cc8) {
        let name = form_name(e, owner);
        e.call(DEBUG_PRINT, &args![format, name]);
    }
    true
}

// Translated from 005a2f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name, debug lines "is sneaking": 1.0
/// when `004997b0` says the actor is sneaking. Asks it even for a null actor.
pub fn fn_005a2f60(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if e.call(ACTOR_IS_SNEAKING, &args![actor]).bool() {
        set_result(e, result, 1.0);
    }
    // "%s is not sneaking" / "%s is sneaking"
    if let Some(format) = trace_choice(e, result, 0x0103_5cd8, 0x0103_5cec) {
        let name = form_name(e, actor);
        e.call(DEBUG_PRINT, &args![format, name]);
    }
    true
}

// Translated from 005a3010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsRunningConditionFunction` (Xbox PDB): 1.0 when
/// `Actor::IsRunning` (Xbox PDB) says so. Asks it even for a null actor.
pub fn script_is_running_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if e.call(ACTOR_IS_RUNNING, &args![actor]).bool() {
        set_result(e, result, 1.0);
    }
    // "%s is not running" / "%s is running"
    if let Some(format) = trace_choice(e, result, 0x0103_5cfc, 0x0103_5d10) {
        let name = form_name(e, actor);
        e.call(DEBUG_PRINT, &args![format, name]);
    }
    true
}

// Translated from 005a30c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetFriendHitConditionFunction` (Xbox PDB): the friend-hit count
/// of the actor's extra data list; 0.0 and no debug line for a reference
/// that is not an actor. The debug line names the player and the actor.
pub fn script_get_friend_hit_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if actor.is_null() {
        return true;
    }
    let list = e.call(REF_GET_EXTRA_DATA_LIST, &args![actor]).u32();
    let count = e.call(EXTRA_LIST_GET_FRIEND_HIT_COUNT, &args![list]).i32();
    set_result(e, result, count as f64);
    if trace_enabled(e) {
        let value = e.mem.f64(result.addr());
        let actor_name = form_name(e, actor);
        let player = Ptr::<()>::new(e.global::<u32>(PLAYER));
        let player_name = form_name(e, player);
        // "%s has hit %s %0.2f times"
        e.call(
            DEBUG_PRINT,
            &args![0x0103_5d20u32, player_name, actor_name, value],
        );
    }
    true
}

// Translated from 005a3180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsInCombatConditionFunction` (Xbox PDB): 1.0 when the byte test
/// `00493bb0` says the actor is in combat; for the player the value is
/// instead `PlayerCharacter::IsPlayerCharacterInCombat` (Xbox PDB), called
/// with a local byte that starts as 0.
pub fn script_is_in_combat_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() && e.call(ACTOR_IN_COMBAT_FLAG, &args![actor]).bool() {
        set_result(e, result, 1.0);
    }
    let player = e.global::<u32>(PLAYER);
    if actor.addr() == player {
        let in_combat = e.with_stack(1, |e, out| {
            e.call(PLAYER_IS_IN_COMBAT, &args![player, out]).u32() & 0xff
        });
        set_result(e, result, in_combat as f64);
    }
    // "%s is not in combat" / "%s is in combat"
    if let Some(format) = trace_choice(e, result, 0x0103_5d3c, 0x0103_5d50) {
        let name = form_name(e, actor);
        e.call(DEBUG_PRINT, &args![format, name]);
    }
    true
}

// Translated from 005a3270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsInInteriorConditionFunction` (Xbox PDB): 1.0 when the
/// reference has a parent cell and it is an interior.
pub fn script_is_in_interior_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !reference.is_null()
        && !e
            .call(REF_GET_PARENT_CELL, &args![reference])
            .ptr::<()>()
            .is_null()
    {
        let cell = e.call(REF_GET_PARENT_CELL, &args![reference]).u32();
        if e.call(CELL_IS_INTERIOR, &args![cell]).bool() {
            set_result(e, result, 1.0);
        }
    }
    // "%s is not an Interior" / "%s is in an Interior"
    if let Some(format) = trace_choice(e, result, 0x0103_5d60, 0x0103_5d78) {
        let name = form_name(e, reference);
        e.call(DEBUG_PRINT, &args![format, name]);
    }
    true
}

// Translated from 005a3310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetPCMiscStatConditionFunction` (Xbox PDB): the player's misc
/// stat whose index is the first parameter ([`fn_005a3370`]).
pub fn script_get_pc_misc_stat_condition_function(
    e: &mut Engine,
    _reference: u32,
    index: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let value = fn_005a3370(e, index) as i32;
    set_result(e, result, value as f64);
    if trace_enabled(e) {
        // "Player misc stat value %.02f"
        trace_result(e, 0x0103_5d90, result);
    }
    true
}

// Translated from 005a3370 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value (`+4`) of entry `index` of the table of misc stats at
/// `0x011c6d50` (a table of pointers); cdecl, one word.
pub fn fn_005a3370(e: &mut Engine, index: u32) -> u32 {
    let entry = e.mem.u32(MISC_STAT_TABLE + index.wrapping_mul(4));
    e.mem.u32(entry + 4)
}

// Translated from 005a3390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsActorEvilConditionFunction` (Xbox PDB): 1.0 when
/// `TESObjectREFR::IsPartofEvilFaction` (Xbox PDB) says the actor is. A
/// reference that is not an actor gets 0.0 and no debug line.
pub fn script_is_actor_evil_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let actor = actor_of(e, reference);
    set_result(e, result, 0.0);
    if !actor.is_null() {
        if e.call(REF_IS_PART_OF_EVIL_FACTION, &args![actor]).bool() {
            set_result(e, result, 1.0);
        }
        // "%s  is not evil " / "%s  is evil "
        if let Some(format) = trace_choice(e, result, 0x0103_5db0, 0x0103_5dc4) {
            let name = e.call(REF_GET_NAME, &args![actor]).u32();
            e.call(DEBUG_PRINT, &args![format, name]);
        }
    }
    true
}

// Translated from 005a3460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsActorVictimConditionFunction` (Xbox PDB): for an actor whose
/// virtual `+0x218` answers yes, 1.0 when its base form is non-null and is
/// the form kept in the TLS block at `+0x26c` (the current crime victim).
pub fn script_is_actor_victim_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let actor = actor_of(e, reference);
    set_result(e, result, 0.0);
    if !actor.is_null() && e.vcall(actor.addr(), VSLOT_ACTOR_TEST_218, &args![]).bool() {
        let base = e.call(REF_GET_BASE_FORM, &args![actor]).u32();
        let tls = e.tls();
        if base != 0 && base == e.mem.u32(tls + TLS_CRIME_VICTIM) {
            set_result(e, result, 1.0);
        }
        // "%s  is not a crime victim " / "%s  crime victim "
        if let Some(format) = trace_choice(e, result, 0x0103_5dd4, 0x0103_5df0) {
            let name = e.call(REF_GET_NAME, &args![actor]).u32();
            e.call(DEBUG_PRINT, &args![format, name]);
        }
    }
    true
}

// Translated from 005a3570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: for an actor with a
/// middle-high process, 1.0 when the process's virtual `+0x60` answers yes.
pub fn fn_005a3570(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null()
        && !e
            .call(ACTOR_GET_PROCESS, &args![actor])
            .ptr::<()>()
            .is_null()
    {
        let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
        if e.vcall(process, VSLOT_PROCESS_TEST_60, &args![]).bool() {
            set_result(e, result, 1.0);
        }
    }
    true
}

// Translated from 005a35f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: for an actor, 1.0 when the byte
/// test `0087f510` answers yes.
pub fn fn_005a35f0(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() && e.call(ACTOR_TEST_87F510, &args![actor]).bool() {
        set_result(e, result, 1.0);
    }
    true
}

// Translated from 005a3650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: the `int` global at
/// `0x0119f344`. (The code first stores -1.0 and overwrites it at once.)
pub fn fn_005a3650(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let value = e.global::<u32>(INT_GLOBAL_5A3650) as i32;
    set_result(e, result, value as f64);
    true
}

// Translated from 005a3670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: always 0.0 (it asks a
/// non-null reference whether it is an actor and does nothing with the
/// answer).
pub fn fn_005a3670(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !reference.is_null() {
        actor_of(e, reference);
    }
    true
}

// Translated from 005a36b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsInDangerousWaterConditionFunction` (Xbox PDB): for an actor
/// that has the flag byte `fn_005a3740` reads, standing in a cell with a
/// water type whose form says it is dangerous: 1.0.
pub fn script_is_in_dangerous_water_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null()
        && fn_005a3740(e, actor) != 0
        && !e
            .call(REF_GET_PARENT_CELL, &args![actor])
            .ptr::<()>()
            .is_null()
    {
        let cell = e.call(REF_GET_PARENT_CELL, &args![actor]).u32();
        if !e
            .call(CELL_GET_WATER_TYPE, &args![cell])
            .ptr::<()>()
            .is_null()
        {
            let cell = e.call(REF_GET_PARENT_CELL, &args![actor]).u32();
            let water = e.call(CELL_GET_WATER_TYPE, &args![cell]).u32();
            if e.call(WATER_FORM_GET_DANGEROUS, &args![water]).bool() {
                set_result(e, result, 1.0);
            }
        }
    }
    true
}

// Translated from 005a3740 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x14c` of the actor.
pub fn fn_005a3740(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x14c)
}

// Translated from 005a3760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: 1.0 when [`fn_005a3790`] says
/// the reference has the flag. Does not check for a null reference.
pub fn fn_005a3760(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if fn_005a3790(e, reference) {
        set_result(e, result, 1.0);
    }
    true
}

// Translated from 005a3790 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when bit `0x00100000` of the form flags (`this + 0x08`) is set.
pub fn fn_005a3790(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x08) & 0x0010_0000 != 0
}

// Translated from 005a37b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsXboxConditionFunction` (Xbox PDB): 1.0 when `00709d40` (a
/// platform test with no arguments) says yes, else 0.0.
pub fn script_is_xbox_condition_function(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let console = e.call(PLATFORM_IS_CONSOLE, &args![]).bool();
    set_result(e, result, if console { 1.0 } else { 0.0 });
    if trace_enabled(e) {
        // "IsXBox >> %1.0f"
        trace_result(e, 0x0103_5e04, result);
    }
    true
}

// Translated from 005a3820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsPS3ConditionFunction` (Xbox PDB): always 0.0.
pub fn script_is_ps3_condition_function(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if trace_enabled(e) {
        // "IsPS3 >> %1.0f"
        trace_result(e, 0x0103_5e14, result);
    }
    true
}

// Translated from 005a3870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsPCConditionFunction` (Xbox PDB): 0.0 when `00709d40` says
/// yes, else 1.0 (the opposite of `IsXbox`).
pub fn script_is_pc_condition_function(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 1.0);
    let console = e.call(PLATFORM_IS_CONSOLE, &args![]).bool();
    set_result(e, result, if console { 0.0 } else { 1.0 });
    if trace_enabled(e) {
        // "IsPC >> %1.0f"
        trace_result(e, 0x0103_5e24, result);
    }
    true
}

// Translated from 005a38e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the fifteen words of the caches the condition functions keep
/// (the reference / form pairs and their `float` results).
pub fn fn_005a38e0(e: &mut Engine) {
    for address in CACHE_WORDS {
        e.set_global(address, 0u32);
    }
}

// Translated from 005a3980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsEssentialConditionFunction` (Xbox PDB): 1.0 when the
/// reference is an actor and `Actor::GetEssential` (Xbox PDB) says so. Does
/// not check for a null reference.
pub fn script_is_essential_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if e.vcall(reference.addr(), VSLOT_IS_ACTOR, &args![]).bool()
        && e.call(ACTOR_GET_ESSENTIAL, &args![reference]).bool()
    {
        set_result(e, result, 1.0);
    }
    true
}

// Translated from 005a39d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: 1.0 when the reference is an
/// actor (virtual `IsActor`). Does not check for a null reference.
pub fn fn_005a39d0(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if e.vcall(reference.addr(), VSLOT_IS_ACTOR, &args![]).bool() {
        set_result(e, result, 1.0);
    }
    true
}

// Translated from 005a3a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: 1.0 when the byte at `+0x20c`
/// of the player is set.
pub fn fn_005a3a00(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let player = e.global::<u32>(PLAYER);
    if e.mem.u8(player + 0x20c) != 0 {
        set_result(e, result, 1.0);
    }
    true
}

// Translated from 005a3a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: for an actor whose virtual
/// `+0x22c` (called with 0) answers yes and that has a middle-high process
/// whose float (virtual `+0x69c`) is positive, the calendar's whole value
/// (`00867e30`, as a `float`) minus that float; else 0.0.
pub fn fn_005a3a30(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !reference.is_null()
        && e.vcall(reference.addr(), VSLOT_ACTOR_TEST_22C, &args![0u32])
            .bool()
        && e.vcall(reference.addr(), VSLOT_IS_ACTOR, &args![]).bool()
        && !e
            .call(ACTOR_GET_PROCESS, &args![reference])
            .ptr::<()>()
            .is_null()
    {
        let process = e.call(ACTOR_GET_PROCESS, &args![reference]).u32();
        let value = e.vcall(process, VSLOT_PROCESS_FLOAT_69C, &args![]).f32();
        let zero: f64 = e.global(DOUBLE_ZERO);
        if value as f64 > zero {
            let whole = e.call(CALENDAR_GET_WHOLE_VALUE, &args![CALENDAR]).u32();
            set_result(e, result, (whole as f32) as f64 - value as f64);
        }
    }
    true
}

// Translated from 005a3af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: the `int` that `00964060`
/// returns for the player.
pub fn fn_005a3af0(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let player = e.global::<u32>(PLAYER);
    let value = e.call(PLAYER_GET_VALUE_964060, &args![player]).i32();
    set_result(e, result, value as f64);
    true
}

// Translated from 005a3b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsPlayerActionActiveConditionFunction` (Xbox PDB): the byte
/// answer of the player's `00963ce0` for the action in the first parameter
/// (0 or 1).
pub fn script_is_player_action_active_condition_function(
    e: &mut Engine,
    _reference: u32,
    action: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let player = e.global::<u32>(PLAYER);
    let active = e
        .call(PLAYER_IS_ACTION_ACTIVE, &args![player, action])
        .u32()
        & 0xff;
    set_result(e, result, active as f64);
    if trace_enabled(e) {
        // "IsActionInList >> %0.2f"
        trace_result(e, 0x0103_5e34, result);
    }
    true
}

// Translated from 005a3b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: 1.0 when the reference's base
/// form has type `0x16` and its `+0x90` value (`00516bf0`) equals the first
/// parameter.
pub fn fn_005a3b90(e: &mut Engine, reference: Ptr, value: u32, _param2: u32, result: Ptr) -> bool {
    set_result(e, result, 0.0);
    if !reference.is_null() {
        let base = e.call(REF_GET_BASE_FORM, &args![reference]).u32();
        if e.call(FORM_GET_TYPE, &args![base]).u32() == FORM_TYPE_SPEAKER_SOURCE {
            let base = e.call(REF_GET_BASE_FORM, &args![reference]).u32();
            if e.call(FORM_GET_FIELD_90, &args![base]).u32() == value {
                set_result(e, result, 1.0);
            }
        }
    }
    true
}

// Translated from 005a3bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: 1.0 when the player's
/// `00966c10` answers yes for the first parameter.
pub fn fn_005a3bf0(e: &mut Engine, _reference: u32, value: u32, _param2: u32, result: Ptr) -> bool {
    set_result(e, result, 0.0);
    let player = e.global::<u32>(PLAYER);
    if e.call(PLAYER_TEST_966C10, &args![player, value]).bool() {
        set_result(e, result, 1.0);
    }
    true
}

// Translated from 005a3c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: for an actor with a part
/// object (`008a5270`), the `int` at its `+0x10`; -1.0 otherwise.
pub fn fn_005a3c30(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let minus_one: f64 = e.global(DOUBLE_MINUS_ONE);
    set_result(e, result, minus_one);
    let actor = actor_of(e, reference);
    if !actor.is_null()
        && !e
            .call(ACTOR_GET_PROCESS_PART, &args![actor])
            .ptr::<()>()
            .is_null()
    {
        let part = e.call(ACTOR_GET_PROCESS_PART, &args![actor]).u32();
        let value = e.mem.u32(part + 0x10) as i32;
        set_result(e, result, value as f64);
    }
    true
}

// Translated from 005a3c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: for an actor with a part object
/// (`008a5270`), 1.0 when its flag test `0058cba0` with mask 4 answers yes.
pub fn fn_005a3c90(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null()
        && !e
            .call(ACTOR_GET_PROCESS_PART, &args![actor])
            .ptr::<()>()
            .is_null()
    {
        let part = e.call(ACTOR_GET_PROCESS_PART, &args![actor]).u32();
        let flagged = e.call(PROCESS_PART_TEST_FLAGS, &args![part, 4u32]).bool();
        set_result(e, result, if flagged { 1.0 } else { 0.0 });
    }
    true
}

// ---------------------------------------------------------------------------
// Second session: `005a3d00` to `005a4f30`.
// ---------------------------------------------------------------------------

/// A byte test of the player (`teswater.cpp`, `this` the player): when it
/// answers no, `fn_005a3d00` reports 1.0.
const PLAYER_TEST_4EAF60: u32 = 0x004e_af60;
/// `TESObjectREFR` getters in `tesobjectrefr.cpp` (`this` the reference):
/// the value behind `GetCauseofDeath`, `TESObjectREFR::GetDismembered`
/// (Xbox PDB, one limb argument) and `TESObjectREFR::GetLastHitLimb`
/// (Xbox PDB).
const REF_GET_CAUSE_OF_DEATH: u32 = 0x0057_30d0;
const REF_GET_DISMEMBERED: u32 = 0x0057_3090;
const REF_GET_LAST_HIT_LIMB: u32 = 0x0057_3110;
/// Virtual (vtable `+0x344`) on an actor, called with (the player, 0): an
/// `int` that `fn_005a3e90` compares with a setting.
const VSLOT_ACTOR_INT_344: u32 = 0x344;
/// The static object (`0x011d0844`) whose method `0043d4d0` (`modelloader.cpp`)
/// returns the address of an `int` setting for `fn_005a3e90`.
const SETTING_OBJECT: u32 = 0x011d_0844;
const SETTING_GET_INT_ADDRESS: u32 = 0x0043_d4d0;
/// `[this + 0xc0]` of an actor is the value `GetKiller` compares with.
const ACTOR_KILLER_OFFSET: u32 = 0xc0;
/// `ExtraDataList::GetDismembermentExtra` (Xbox PDB), then two unnamed
/// getters that `GetKillerObject` chains on its result.
const EXTRA_LIST_GET_DISMEMBERMENT_EXTRA: u32 = 0x0042_e8c0;
const DISMEMBERMENT_GET_LIST_NODE: u32 = 0x0082_5c00;
/// `FormList`-like `GetList` (`00500940`, main file's `FORM_LIST_GET_LIST`):
/// `this` the form, no stack word; the result is the list whose test
/// `0x005f65d0` (RET 4) is called with the pointer to the node pointer.
const NODE_TEST_5F65D0: u32 = 0x005f_65d0;
/// The flags word of an actor (`actor.cpp`, `this` the actor).
const ACTOR_GET_FLAGS: u32 = 0x0088_46e0;
/// `Actor::GetAnimAction` (Xbox PDB).
const ACTOR_GET_ANIM_ACTION: u32 = 0x008a_7570;
/// `3.0` and `4.0` (`double`s in the exe's data); `2.0` is [`DOUBLE_TWO`].
const DOUBLE_THREE: u32 = 0x0102_1928;
const DOUBLE_FOUR: u32 = 0x0101_db80;
/// `tesreactionform.cpp`: `this` the object at `param1 + 0x24`, one argument.
const REACTION_FORM_GET_VALUE: u32 = 0x0048_c1b0;
/// Virtual (vtable `+0x3f8`) on a reference: an object (or null) that
/// `fn_005a4240` and `fn_005a42b0` count.
const VSLOT_REF_COUNTED_OBJECT: u32 = 0x3f8;
/// `combatgroupstrategy.cpp`: the count of the object in ECX.
const OBJECT_GET_COUNT: u32 = 0x0099_0890;
/// Virtual (vtable `+0x388`) on the actor's middle-high process: a byte.
const VSLOT_PROCESS_BYTE_388: u32 = 0x388;
/// The process-list singleton (`0x011e0e80`) and its byte test of a
/// reference (`processlists.cpp`).
const PROCESS_LISTS: u32 = 0x011e_0e80;
const PROCESS_LISTS_TEST_REFERENCE: u32 = 0x0097_3d50;
/// Virtuals (vtable `+0xfc`, `+0x2d0`, `+0x2d8`) on a reference: the test
/// behind the dialogue emotion functions (the unit's main file names slot
/// `0xfc` `IsMobileObject`), the emotion, and a call that returns nothing
/// used.
const VSLOT_EMOTION: u32 = 0x2d0;
const VSLOT_EMOTION_VALUE: u32 = 0x2d8;
/// `TESIdleManager::GetUsedItem` (Xbox PDB): the item or null.
const IDLE_MANAGER_GET_USED_ITEM: u32 = 0x0060_08f0;
/// Virtual (vtable `+0x140`) on the used item: non-zero for a water object.
const VSLOT_ITEM_WATER_TEST: u32 = 0x140;
/// Virtual (vtable `+0x4a0`) on an actor: (perk, flag) -> rank byte.
const VSLOT_ACTOR_PERK_RANK: u32 = 0x4a0;
/// `Actor::GetFactionFightReaction` (Xbox PDB): `this` the actor, (other
/// actor, pointer to an out byte).
const ACTOR_GET_FACTION_FIGHT_REACTION: u32 = 0x008b_87a0;
/// Virtual (vtable `+0x38c`) on the actor's process: the last idle played.
const VSLOT_PROCESS_LAST_IDLE: u32 = 0x38c;
/// `MiddleHighProcess::GetForceNextUpdate` (Xbox PDB).
const MIDDLE_HIGH_GET_FORCE_NEXT_UPDATE: u32 = 0x0056_6950;
/// `ExtraDataList::GetPlayerCrimeList` (Xbox PDB).
const EXTRA_LIST_GET_PLAYER_CRIME_LIST: u32 = 0x0041_cf00;
/// The crime counts of an actor other than the player (`actor.cpp`).
const ACTOR_GET_MINOR_CRIME_COUNT: u32 = 0x008b_7e20;
const ACTOR_GET_MAJOR_CRIME_COUNT: u32 = 0x008b_7f00;
/// A getter of the player (`actor.cpp`, `this` the player).
const PLAYER_GET_VALUE_89F4E0: u32 = 0x0089_f4e0;
/// `bgsdestructibleobjectform.cpp`, cdecl, one reference: the stage.
const GET_DESTRUCTION_STAGE: u32 = 0x0047_7430;
/// `Actor::CalculateCombatStrength` (Xbox PDB): `this` the actor, one
/// `float`; the strength in ST0.
const ACTOR_CALCULATE_COMBAT_STRENGTH: u32 = 0x008a_cbe0;
/// The `float` (`-1.0`) passed to it.
const COMBAT_STRENGTH_ARGUMENT: u32 = 0x0101_2054;
/// `TESActorBaseData::GetAlignmentForKarma` (Xbox PDB): cdecl, a `float`.
const GET_ALIGNMENT_FOR_KARMA: u32 = 0x0047_e040;
/// The actor-value owner embedded at `+0xa4` of an actor, and its virtual
/// `+0xc` (actor value in, `float` in ST0 out).
const ACTOR_VALUE_OWNER_OFFSET: u32 = 0xa4;
const VSLOT_OWNER_GET_VALUE: u32 = 0xc;
/// The actor value `GetIsAlignment` reads (Karma).
const ACTOR_VALUE_KARMA: u32 = 0x17;
/// Tables of string pointers: alignment names and equip type names.
const ALIGNMENT_NAMES: u32 = 0x0118_6cf4;
const EQUIP_TYPE_NAMES: u32 = 0x0118_69bc;
/// `BGSEquipType::GetEquipType` (Xbox PDB): cdecl, the item.
const GET_EQUIP_TYPE: u32 = 0x0047_9430;
/// Virtual (vtable `+0x20c`) on the actor's process: an object (or null)
/// that the same virtual is then called on again with the process.
const VSLOT_PROCESS_AGGRO_OBJECT: u32 = 0x20c;
/// `extradatalist.cpp`: a getter on the object in ECX, which
/// `GetAggroRadiusViolated` compares with 14.
const AGGRO_GET_STATE: u32 = 0x0041_ca90;
const AGGRO_STATE_VIOLATED: u32 = 0x0e;
/// `"false"` and `"true"` in the exe's strings.
const TEXT_FALSE: u32 = 0x0102_fd24;
const TEXT_TRUE: u32 = 0x0102_fd2c;

/// `"true"` when the result is above 0.0, else `"false"` (the debug lines).
fn trace_text(e: &mut Engine, result: Ptr) -> u32 {
    if e.mem.f64(result.addr()) > 0.0 {
        TEXT_TRUE
    } else {
        TEXT_FALSE
    }
}

/// The debug line `format(ftol(*result))`.
fn trace_result_as_int(e: &mut Engine, format: u32, result: Ptr) {
    let value = e.mem.f64(result.addr());
    let rounded = e.call(FTOL, &args![value]).u32();
    e.call(DEBUG_PRINT, &args![format, rounded]);
}

fn set_minus_one(e: &mut Engine, result: Ptr) {
    let minus_one: f64 = e.global(DOUBLE_MINUS_ONE);
    set_result(e, result, minus_one);
}

// Translated from 005a3d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: 1.0 when the byte test
/// `004eaf60` of the player answers no, else 0.0.
pub fn fn_005a3d00(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let player = e.global::<u32>(PLAYER);
    if !e.call(PLAYER_TEST_4EAF60, &args![player]).bool() {
        set_result(e, result, 1.0);
    }
    true
}

// Translated from 005a3d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetCauseofDeathConditionFunction` (Xbox PDB): -1.0, or for an
/// actor the cause of death (`005730d0`). The reference is not checked for
/// null.
pub fn script_get_causeof_death_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_minus_one(e, result);
    if e.vcall(reference.addr(), VSLOT_IS_ACTOR, &args![]).bool() {
        let cause = e.call(REF_GET_CAUSE_OF_DEATH, &args![reference]).i32();
        set_result(e, result, cause as f64);
    }
    if trace_enabled(e) {
        // "Cause of death is >> %0.2f"
        trace_result(e, 0x0103_5e4c, result);
    }
    true
}

// Translated from 005a3db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsLimbGoneConditionFunction` (Xbox PDB): 1.0 when the reference
/// is an actor and `TESObjectREFR::GetDismembered` (Xbox PDB) says the limb
/// `param1` is gone.
pub fn script_is_limb_gone_condition_function(
    e: &mut Engine,
    reference: Ptr,
    param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() && e.call(REF_GET_DISMEMBERED, &args![actor, param1]).bool() {
        set_result(e, result, 1.0);
    }
    true
}

// Translated from 005a3e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetKillingBlowLimbConditionFunction` (Xbox PDB): -1.0, or for an
/// actor `TESObjectREFR::GetLastHitLimb` (Xbox PDB). The reference is not
/// checked for null.
pub fn script_get_killing_blow_limb_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_minus_one(e, result);
    if e.vcall(reference.addr(), VSLOT_IS_ACTOR, &args![]).bool() {
        let limb = e.call(REF_GET_LAST_HIT_LIMB, &args![reference]).i32();
        set_result(e, result, limb as f64);
    }
    if trace_enabled(e) {
        // "Last hit limb is >> %0.0f"
        trace_result(e, 0x0103_5e68, result);
    }
    true
}

// Translated from 005a3e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: 1.0 when the actor's virtual
/// `+0x344`, called with (the player, 0), is at least the setting whose
/// address `0043d4d0` returns (signed comparison).
pub fn fn_005a3e90(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let player = e.global::<u32>(PLAYER);
        let value = e
            .vcall(actor.addr(), VSLOT_ACTOR_INT_344, &args![player, 0u32])
            .i32();
        let setting = e
            .call(SETTING_GET_INT_ADDRESS, &args![SETTING_OBJECT])
            .u32();
        let limit = e.mem.u32(setting) as i32;
        if value >= limit {
            set_result(e, result, 1.0);
        }
    }
    true
}

// Translated from 005a3f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetKillerConditionFunction` (Xbox PDB): for an actor whose
/// virtual `+0x22c` (called with 0) answers yes, 1.0 when `param1` is the
/// word at `+0xc0` of the actor. The debug line is only printed for actors.
pub fn script_get_killer_condition_function(
    e: &mut Engine,
    reference: Ptr,
    param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        if e.vcall(actor.addr(), VSLOT_ACTOR_TEST_22C, &args![0u32])
            .bool()
            && param1 == e.mem.u32(actor.addr() + ACTOR_KILLER_OFFSET)
        {
            set_result(e, result, 1.0);
        }
        if trace_enabled(e) {
            // "IsKiller >> %0.2f"
            trace_result(e, 0x0103_5e84, result);
        }
    }
    true
}

// Translated from 005a3fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetKillerObjectConditionFunction` (Xbox PDB): for an actor with
/// a process whose virtual `+0x22c` answers yes and a non-null `param1`:
/// takes the actor's dismemberment extra data (through `ExtraDataList::
/// GetDismembermentExtra`, Xbox PDB, and `00825c00`) and sets 1.0 when
/// `00500940(param1, &node)` then `005f65d0` says yes. Unlike its
/// neighbours it does not clear the result first, so a caller's value is
/// left (and printed) when nothing sets it.
pub fn script_get_killer_object_condition_function(
    e: &mut Engine,
    reference: Ptr,
    param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
        if process != 0
            && e.vcall(actor.addr(), VSLOT_ACTOR_TEST_22C, &args![0u32])
                .bool()
            && param1 != 0
        {
            let list = e.call(REF_GET_EXTRA_DATA_LIST, &args![actor]).u32();
            let extra = e
                .call(EXTRA_LIST_GET_DISMEMBERMENT_EXTRA, &args![list])
                .u32();
            let node = e.call(DISMEMBERMENT_GET_LIST_NODE, &args![extra]).u32();
            if node != 0 {
                let found = e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), node);
                    // 005a4031: `00500940` takes no stack word (plain RET); the pushed
                    // node address is the word of `005f65d0` (RET 4).
                    let list_node = e.call(FORM_LIST_GET_LIST, &args![param1]).u32();
                    e.call(NODE_TEST_5F65D0, &args![list_node, slot]).bool()
                });
                if found {
                    set_result(e, result, 1.0);
                }
            }
        }
    }
    if trace_enabled(e) {
        // "IsKillerObject >> %0.2f"
        trace_result(e, 0x0103_5e98, result);
    }
    true
}

// Translated from 005a4090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: when `param1` and `param2` are
/// both non-null, the value `0048c1b0(param1 + 0x24, param2)`
/// (`tesreactionform.cpp`); otherwise the result is left alone.
pub fn fn_005a4090(e: &mut Engine, _reference: u32, param1: u32, param2: u32, result: Ptr) -> bool {
    if param2 != 0 && param1 != 0 {
        let value = e
            .call(REACTION_FORM_GET_VALUE, &args![param1 + 0x24, param2])
            .i32();
        set_result(e, result, value as f64);
    }
    true
}

// Translated from 005a40d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: for an actor, the lowest of
/// the low four bits of the actor flags word `008846e0` that is set, as
/// 1.0 / 2.0 / 3.0 / 4.0 (bit 0 / 1 / 2 / 3); 0.0 for none.
pub fn fn_005a40d0(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let flags = e.call(ACTOR_GET_FLAGS, &args![actor]).u32() & 0xf;
        if flags & 1 != 0 {
            set_result(e, result, 1.0);
        } else if flags & 2 != 0 {
            let two: f64 = e.global(DOUBLE_TWO);
            set_result(e, result, two);
        } else if flags & 4 != 0 {
            let three: f64 = e.global(DOUBLE_THREE);
            set_result(e, result, three);
        } else if flags & 8 != 0 {
            let four: f64 = e.global(DOUBLE_FOUR);
            set_result(e, result, four);
        }
    }
    true
}

// Translated from 005a4160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: for an actor, 1.0 when bit
/// `0x10` of the flags word `008846e0` is set, else 2.0 when bit `0x20` is.
pub fn fn_005a4160(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let flags = e.call(ACTOR_GET_FLAGS, &args![actor]).u32() & 0x30;
        if flags & 0x10 != 0 {
            set_result(e, result, 1.0);
        } else if flags & 0x20 != 0 {
            let two: f64 = e.global(DOUBLE_TWO);
            set_result(e, result, two);
        }
    }
    true
}

// Translated from 005a41c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetAnimActionConditionFunction` (Xbox PDB): -1.0, or for an
/// actor `Actor::GetAnimAction` (Xbox PDB).
pub fn script_get_anim_action_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_minus_one(e, result);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let action = e.call(ACTOR_GET_ANIM_ACTION, &args![actor]).i32();
        set_result(e, result, action as f64);
    }
    true
}

// Translated from 005a4210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: 0.0 when the reference is null
/// or is `param1`, else 1.0.
pub fn fn_005a4210(e: &mut Engine, reference: u32, param1: u32, _param2: u32, result: Ptr) -> bool {
    if reference == 0 || reference == param1 {
        set_result(e, result, 0.0);
    } else {
        set_result(e, result, 1.0);
    }
    true
}

// Translated from 005a4240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: for an actor whose virtual
/// `+0x3f8` returns an object, the count `00990890` of that object.
pub fn fn_005a4240(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let object = e
            .vcall(actor.addr(), VSLOT_REF_COUNTED_OBJECT, &args![])
            .u32();
        if object != 0 {
            let count = e.call(OBJECT_GET_COUNT, &args![object]).u32();
            set_result(e, result, count as f64);
        }
    }
    true
}

// Translated from 005a42b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: for an actor whose virtual
/// `+0x3f8` returns an object, `fn_005a4320` of that object (as an
/// unsigned number).
pub fn fn_005a42b0(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let object = e
            .vcall(actor.addr(), VSLOT_REF_COUNTED_OBJECT, &args![])
            .ptr::<()>();
        if !object.is_null() {
            let value = fn_005a4320(e, object);
            set_result(e, result, value as f64);
        }
    }
    true
}

// Translated from 005a4320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `0044ddc0` on `this + 8` and returns its result.
pub fn fn_005a4320(e: &mut Engine, this: Ptr) -> u32 {
    e.call(OBJECT_GET_FIELD_08, &args![this.addr() + 8]).u32()
}

// Translated from 005a4340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: for an actor with a process,
/// the byte returned by the process's virtual `+0x388`.
pub fn fn_005a4340(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() && e.call(ACTOR_GET_PROCESS, &args![actor]).u32() != 0 {
        let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
        let value = e.vcall(process, VSLOT_PROCESS_BYTE_388, &args![]).u32() & 0xff;
        set_result(e, result, value as f64);
    }
    true
}

// Translated from 005a43b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: for an actor, the byte test
/// `00973d50` of the process lists with the actor.
pub fn fn_005a43b0(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let value = e
            .call(PROCESS_LISTS_TEST_REFERENCE, &args![PROCESS_LISTS, actor])
            .u32()
            & 0xff;
        set_result(e, result, value as f64);
    }
    true
}

// Translated from 005a4410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsObjectTypeConditionFunction` (Xbox PDB): 1.0 when the form
/// type of the reference's base form is `param1`. Neither the reference nor
/// the base form is checked for null.
pub fn script_get_is_object_type_condition_function(
    e: &mut Engine,
    reference: Ptr,
    param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let base = e.call(REF_GET_BASE_FORM, &args![reference]).u32();
    let form_type = e.call(FORM_GET_TYPE, &args![base]).u32();
    if form_type == param1 {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "GetIsObjectType is  >> %0.2f"
        trace_result(e, 0x0103_5eb0, result);
    }
    true
}

// Translated from 005a4480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetDialogueEmotionConditionFunction` (Xbox PDB): 0.0, or when
/// the reference's virtual `+0xfc` answers yes: the emotion (virtual
/// `+0x2d0`), changed to -1.0 when the byte at `+0x86` of the reference
/// (`fn_005a4520`) is zero. The reference is not checked for null.
pub fn script_get_dialogue_emotion_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if e.vcall(reference.addr(), VSLOT_IS_MOBILE_OBJECT, &args![])
        .bool()
    {
        let emotion = e.vcall(reference.addr(), VSLOT_EMOTION, &args![]).i32();
        set_result(e, result, emotion as f64);
        if fn_005a4520(e, reference) == 0 {
            set_minus_one(e, result);
        }
    }
    if trace_enabled(e) {
        // "GetEmotion >> %0.2f"
        trace_result(e, 0x0103_5ed0, result);
    }
    true
}

// Translated from 005a4520 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x86` of the object.
pub fn fn_005a4520(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x86)
}

// Translated from 005a4540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetDialogueEmotionValueConditionFunction` (Xbox PDB): always
/// 0.0; when the reference's virtual `+0xfc` answers yes it also calls
/// virtual `+0x2d8` and ignores what it returns. The reference is not
/// checked for null.
pub fn script_get_dialogue_emotion_value_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if e.vcall(reference.addr(), VSLOT_IS_MOBILE_OBJECT, &args![])
        .bool()
    {
        e.vcall(reference.addr(), VSLOT_EMOTION_VALUE, &args![]);
    }
    if trace_enabled(e) {
        // "GetEmotionValue >> %0.2f"
        trace_result(e, 0x0103_5ee4, result);
    }
    true
}

// Translated from 005a45c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsWaterObjectConditionFunction` (Xbox PDB): 1.0 when
/// `TESIdleManager::GetUsedItem` (Xbox PDB) returns an item whose virtual
/// `+0x140` is non-zero.
pub fn script_is_water_object_condition_function(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let item = e.call(IDLE_MANAGER_GET_USED_ITEM, &args![]).u32();
    if item != 0 && e.vcall(item, VSLOT_ITEM_WATER_TEST, &args![]).u32() != 0 {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "IsWaterActivator >> %0.2f"
        trace_result(e, 0x0103_5f00, result);
    }
    true
}

// Translated from 005a4630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::HasPerkConditionFunction` (Xbox PDB): for an actor, 1.0 when its
/// virtual `+0x4a0` called with (`param1`, `param2 != 0`) returns a non-zero
/// rank byte.
pub fn script_has_perk_condition_function(
    e: &mut Engine,
    reference: Ptr,
    param1: u32,
    param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let rank = e
            .vcall(
                actor.addr(),
                VSLOT_ACTOR_PERK_RANK,
                &args![param1, (param2 != 0) as u32],
            )
            .u32()
            & 0xff;
        if trace_enabled(e) {
            // "Perk Rank >> %d"
            e.call(DEBUG_PRINT, &args![0x0103_5f1cu32, rank]);
        }
        if rank != 0 {
            set_result(e, result, 1.0);
        }
    }
    true
}

// Translated from 005a46d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetFactionRelationConditionFunction` (Xbox PDB): for an actor,
/// `Actor::GetFactionFightReaction` (Xbox PDB) of the actor towards the
/// actor `param1`.
pub fn script_get_faction_relation_condition_function(
    e: &mut Engine,
    reference: Ptr,
    param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let reaction = e.with_stack(1, |e, out| {
            e.mem.set_u8(out.addr(), 0);
            e.call(ACTOR_GET_FACTION_FIGHT_REACTION, &args![actor, param1, out])
                .i32()
        });
        set_result(e, result, reaction as f64);
        if trace_enabled(e) {
            // "Faction Relation  >> %d" (the result truncated toward zero)
            let truncated = e.mem.f64(result.addr()) as i64 as u32;
            e.call(DEBUG_PRINT, &args![0x0103_5f2cu32, truncated]);
        }
    }
    true
}

// Translated from 005a4780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsLastPlayedIdleConditionFunction` (Xbox PDB): for an actor,
/// 1.0 when the process's virtual `+0x38c` returns `param1`. The process is
/// not checked for null. The debug line passes the `double` result to a
/// `%d`.
pub fn script_is_last_played_idle_condition_function(
    e: &mut Engine,
    reference: Ptr,
    param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
        let idle = e.vcall(process, VSLOT_PROCESS_LAST_IDLE, &args![]).u32();
        if idle == param1 {
            set_result(e, result, 1.0);
        }
        if trace_enabled(e) {
            // "The last idle played was  >> %d"
            trace_result(e, 0x0103_5f44, result);
        }
    }
    true
}

// Translated from 005a4820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetPlayerTeammateConditionFunction` (Xbox PDB): for an actor,
/// 1.0 when `MiddleHighProcess::GetForceNextUpdate` (Xbox PDB; the map's
/// name for this body) answers yes.
pub fn script_get_player_teammate_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        if e.call(MIDDLE_HIGH_GET_FORCE_NEXT_UPDATE, &args![actor])
            .bool()
        {
            set_result(e, result, 1.0);
        }
        if trace_enabled(e) {
            // "Player teammate  >> %d"
            trace_result_as_int(e, 0x0103_5f64, result);
        }
    }
    true
}

// Translated from 005a48a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetPlayerTeammateCountConditionFunction` (Xbox PDB): the
/// player's teammate count (`fn_005a4910`, unsigned).
pub fn script_get_player_teammate_count_condition_function(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let player = e.global::<u32>(PLAYER);
    let count = fn_005a4910(e, Ptr::new(player));
    set_result(e, result, count as f64);
    if trace_enabled(e) {
        // "Player teammate count  >> %d"
        trace_result_as_int(e, 0x0103_5f7c, result);
    }
    true
}

// Translated from 005a4910 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0xd68` of the player (the teammate count).
pub fn fn_005a4910(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0xd68)
}

// Translated from 005a4930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetActorCrimePlayerEnemyConditionFunction` (Xbox PDB): for an
/// actor whose extra data has a player crime list (`ExtraDataList::
/// GetPlayerCrimeList`, Xbox PDB): 1.0 when that list's node test
/// `008256d0` answers no.
pub fn script_get_actor_crime_player_enemy_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let list = e.call(REF_GET_EXTRA_DATA_LIST, &args![actor]).u32();
        if e.call(EXTRA_LIST_GET_PLAYER_CRIME_LIST, &args![list]).u32() != 0 {
            let list = e.call(REF_GET_EXTRA_DATA_LIST, &args![actor]).u32();
            let crimes = e.call(EXTRA_LIST_GET_PLAYER_CRIME_LIST, &args![list]).u32();
            if !e.call(LIST_NODE_IS_EMPTY, &args![crimes]).bool() {
                set_result(e, result, 1.0);
            }
        }
        // "This actor is not an enemy of player because of crime" /
        // "This actor is an enemy of player because of crime"
        if let Some(format) = trace_choice(e, result, 0x0103_5f9c, 0x0103_5fd4) {
            e.call(DEBUG_PRINT, &args![format]);
        }
    }
    true
}

// Translated from 005a49f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetActorFactionPlayerEnemyConditionFunction` (Xbox PDB): for an
/// actor, 1.0 when `Actor::GetFactionFightReaction` (Xbox PDB) of the actor
/// towards the player sets its out byte.
pub fn script_get_actor_faction_player_enemy_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let player = e.global::<u32>(PLAYER);
        let enemy = e.with_stack(1, |e, out| {
            e.mem.set_u8(out.addr(), 0);
            e.call(ACTOR_GET_FACTION_FIGHT_REACTION, &args![actor, player, out]);
            e.mem.u8(out.addr())
        });
        if enemy != 0 {
            set_result(e, result, 1.0);
        }
        // "This actor is not an enemy of player because of  a faction crime" /
        // "This actor's faction is an enemy of player because of crime"
        if let Some(format) = trace_choice(e, result, 0x0103_6008, 0x0103_604c) {
            e.call(DEBUG_PRINT, &args![format]);
        }
    }
    true
}

// Translated from 005a4aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetMinorCrimeCountConditionFunction` (Xbox PDB): for an actor,
/// the minor crime count: `fn_005a4b40` for the player, `008b7e20` for any
/// other actor.
pub fn script_get_minor_crime_count_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let count = if actor.addr() == e.global::<u32>(PLAYER) {
            fn_005a4b40(e, actor)
        } else {
            e.call(ACTOR_GET_MINOR_CRIME_COUNT, &args![actor]).i32()
        };
        set_result(e, result, count as f64);
        if trace_enabled(e) {
            // "The number of minor crimes is %.2f"
            trace_result(e, 0x0103_6088, result);
        }
    }
    true
}

// Translated from 005a4b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0x13c` of the player (its minor crime count).
pub fn fn_005a4b40(e: &mut Engine, this: Ptr) -> i32 {
    e.mem.u32(this.addr() + 0x13c) as i32
}

// Translated from 005a4b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetMajorCrimeCountConditionFunction` (Xbox PDB): for an actor,
/// the major crime count: `fn_005a4c00` for the player, `008b7f00` for any
/// other actor.
pub fn script_get_major_crime_count_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let count = if actor.addr() == e.global::<u32>(PLAYER) {
            fn_005a4c00(e, actor) as i32
        } else {
            e.call(ACTOR_GET_MAJOR_CRIME_COUNT, &args![actor]).i32()
        };
        set_result(e, result, count as f64);
        if trace_enabled(e) {
            // "The number of major crimes is %.2f"
            trace_result(e, 0x0103_60ac, result);
        }
    }
    true
}

// Translated from 005a4c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0x140` of the player without its top two bits (its major
/// crime count).
pub fn fn_005a4c00(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x140) & 0x3fff_ffff
}

// Translated from 005a4c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name: 1.0 when `param1` is non-null
/// and is what the player getter `0089f4e0` returns.
pub fn fn_005a4c20(
    e: &mut Engine,
    _reference: u32,
    param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if param1 != 0 {
        let player = e.global::<u32>(PLAYER);
        if e.call(PLAYER_GET_VALUE_89F4E0, &args![player]).u32() == param1 {
            set_result(e, result, 1.0);
        }
    }
    true
}

// Translated from 005a4c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetDestructionStageConditionFunction` (Xbox PDB): -1.0, or for a
/// non-null reference the destruction stage `00477430` (no actor test).
pub fn script_get_destruction_stage_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_minus_one(e, result);
    if !reference.is_null() {
        let stage = e.call(GET_DESTRUCTION_STAGE, &args![reference]).i32();
        set_result(e, result, stage as f64);
    }
    if trace_enabled(e) {
        // "GetDestructionStage >> %.2f"
        trace_result(e, 0x0103_60d0, result);
    }
    true
}

// Translated from 005a4cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetThreatRatioConditionFunction` (Xbox PDB): for an actor and a
/// non-null `param1` (another reference): the ratio of
/// `Actor::CalculateCombatStrength` (Xbox PDB) of the reference to that of
/// `param1`, each called with the `float` -1.0. Otherwise 0.0, with a
/// different debug line.
pub fn script_get_threat_ratio_condition_function(
    e: &mut Engine,
    reference: Ptr,
    param1: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if reference.is_null()
        || !e.vcall(reference.addr(), VSLOT_IS_ACTOR, &args![]).bool()
        || param1.is_null()
    {
        if trace_enabled(e) {
            // The line printed when the ratio cannot be computed.
            e.call(DEBUG_PRINT, &args![0x0103_60ecu32]);
        }
        return true;
    }
    let argument: f32 = e.global(COMBAT_STRENGTH_ARGUMENT);
    let own = e
        .call(ACTOR_CALCULATE_COMBAT_STRENGTH, &args![reference, argument])
        .f64();
    let other = e
        .call(ACTOR_CALCULATE_COMBAT_STRENGTH, &args![param1, argument])
        .f64();
    set_result(e, result, own / other);
    if trace_enabled(e) {
        let value = e.mem.f64(result.addr());
        let other_name = e.vcall(param1.addr(), VSLOT_FORM_GET_NAME, &args![]).u32();
        let own_name = e
            .vcall(reference.addr(), VSLOT_FORM_GET_NAME, &args![])
            .u32();
        // "GetThreatRatio: '%s' to '%s' >> %.2f"
        e.call(
            DEBUG_PRINT,
            &args![0x0103_6108u32, own_name, other_name, value],
        );
    }
    true
}

// Translated from 005a4dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsAlignmentConditionFunction` (Xbox PDB): for an actor, 1.0
/// when `TESActorBaseData::GetAlignmentForKarma` (Xbox PDB) of the Karma
/// value (the owner at `+0xa4`, virtual `+0xc` with actor value `0x17`) is
/// `param1`. The debug line is printed for any reference, with the name
/// from the table at `0x01186cf4`.
pub fn script_get_is_alignment_condition_function(
    e: &mut Engine,
    reference: Ptr,
    param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let karma = e
            .vcall(
                actor.addr() + ACTOR_VALUE_OWNER_OFFSET,
                VSLOT_OWNER_GET_VALUE,
                &args![ACTOR_VALUE_KARMA],
            )
            .f32();
        let alignment = e.call(GET_ALIGNMENT_FOR_KARMA, &args![karma]).u32();
        if alignment == param1 {
            set_result(e, result, 1.0);
        }
    }
    if trace_enabled(e) {
        let name = e.mem.u32(ALIGNMENT_NAMES + param1.wrapping_mul(4));
        let text = trace_text(e, result);
        // "GetIsAlignment '%s' >> %s"
        e.call(DEBUG_PRINT, &args![0x0103_6130u32, name, text]);
    }
    true
}

// Translated from 005a4ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIsUsedItemEquipTypeConditionFunction` (Xbox PDB): 1.0 when
/// `TESIdleManager::GetUsedItem` (Xbox PDB) returns an item whose
/// `BGSEquipType::GetEquipType` (Xbox PDB) is `param1`. The debug line uses
/// the name table at `0x011869bc`.
pub fn script_get_is_used_item_equip_type_condition_function(
    e: &mut Engine,
    _reference: u32,
    param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if e.call(IDLE_MANAGER_GET_USED_ITEM, &args![]).u32() != 0 {
        let item = e.call(IDLE_MANAGER_GET_USED_ITEM, &args![]).u32();
        if e.call(GET_EQUIP_TYPE, &args![item]).u32() == param1 {
            set_result(e, result, 1.0);
        }
    }
    if trace_enabled(e) {
        let name = e.mem.u32(EQUIP_TYPE_NAMES + param1.wrapping_mul(4));
        let text = trace_text(e, result);
        // "GetIsUsedItemEquipType '%s' >> %s"
        e.call(DEBUG_PRINT, &args![0x0103_614cu32, name, text]);
    }
    true
}

// Translated from 005a4f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetAggroRadiusViolatedConditionFunction` (Xbox PDB): for an
/// actor with a process whose virtual `+0x20c` returns an object, 1.0 when
/// the same virtual of the process, called again, gives an object whose
/// `0041ca90` is 14.
pub fn script_get_aggro_radius_violated_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        if e.call(ACTOR_GET_PROCESS, &args![actor]).u32() != 0 {
            let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
            if e.vcall(process, VSLOT_PROCESS_AGGRO_OBJECT, &args![]).u32() != 0 {
                let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
                let object = e.vcall(process, VSLOT_PROCESS_AGGRO_OBJECT, &args![]).u32();
                if e.call(AGGRO_GET_STATE, &args![object]).u32() == AGGRO_STATE_VIOLATED {
                    set_result(e, result, 1.0);
                }
            }
        }
        if trace_enabled(e) {
            let name = e.call(REF_GET_NAME, &args![reference]).u32();
            // "%s does not have his aggro radius violated " /
            // "%s has his aggro radius violated "
            let format = if e.mem.f64(result.addr()) > 0.0 {
                0x0103_619cu32
            } else {
                0x0103_6170
            };
            e.call(DEBUG_PRINT, &args![format, name]);
        }
    }
    true
}

// ---- session 3: 005a5020 to 005a60f0 -------------------------------------

/// The `float` global VATS target checks pass as their angle: pi / 2.
const VATS_ANGLE_RIGHT: u32 = 0x0101_ff38;
/// 3 pi / 2.
const VATS_ANGLE_LEFT: u32 = 0x0101_ff34;
/// pi.
const VATS_ANGLE_BACK: u32 = 0x0102_b3c8;
/// `GetVATSAreaFree`-style test of an actor (`this` the actor; the second
/// word and an angle `float`), result in ST0.
const ACTOR_GET_VATS_AREA_FREE: u32 = 0x008b_d830;
/// Same shape, the target-visible test.
const ACTOR_GET_VATS_TARGET_VISIBLE: u32 = 0x008b_dbd0;
/// The `double` 100.0 in the exe's data.
const DOUBLE_ONE_HUNDRED: u32 = 0x0101_7a40;
/// `bool` of the actor in ECX with a flag byte at `+0x94`'s object: the
/// `0x21` / 0 test behind `GetConcussed` (`this` is the actor + `0x94`).
const ACTOR_PART_TEST_822CF0: u32 = 0x0082_2cf0;
/// The byte global `fn_005a50e0` returns.
const BYTE_GLOBAL_5A50E0: u32 = 0x011d_be50;
/// Virtual (vtable `+0x764`, `+0x75c`, `+0x76c`) on the actor's process:
/// three `float`s in ST0 that `GetRadiationLevel` adds.
const VSLOT_PROCESS_RADIATION_A: u32 = 0x764;
const VSLOT_PROCESS_RADIATION_B: u32 = 0x75c;
const VSLOT_PROCESS_RADIATION_C: u32 = 0x76c;
/// `TESObjectREFR::GetMapMarkerData` (Xbox PDB): the reference's marker data.
const REF_GET_MAP_MARKER_DATA: u32 = 0x0056_9060;
/// `MapMarkerData::GetVisible` / `GetTravelLoc` (Xbox PDB).
const MAP_MARKER_GET_VISIBLE: u32 = 0x0043_8ed0;
const MAP_MARKER_GET_TRAVEL_LOC: u32 = 0x0043_8ef0;
/// Virtual (vtable `+0x148`) on the actor's process: the weapon `ItemChange`
/// or null.
const VSLOT_PROCESS_WEAPON_ITEM: u32 = 0x148;
/// The `int` global `GetWeaponHealthPerc` compares the weapon's `+0x08`
/// with.
const WEAPON_COMPARE_GLOBAL: u32 = 0x011c_a278;
/// `ItemChange::GetItemHealth` (Xbox PDB): `this` the change and (1); ST0.
const ITEM_CHANGE_GET_ITEM_HEALTH: u32 = 0x004b_cdb0;
/// Virtual (vtable `+0x30c`) on the actor's process: a byte test.
const VSLOT_PROCESS_TEST_30C: u32 = 0x30c;
/// `TESWorldSpace::GetParentWorldSimple` (Xbox PDB) of the reference in ECX.
const REF_GET_PARENT_WORLD_SIMPLE: u32 = 0x004f_d380;
/// `Actor::IsInCombatWithActor` (Xbox PDB): `this` the actor and (other).
const ACTOR_IS_IN_COMBAT_WITH_ACTOR: u32 = 0x008b_c700;
/// The player's level (`this` the player; the low 16 bits).
const PLAYER_GET_LEVEL: u32 = 0x0087_f9f0;
/// Returns a pointer to the `int` maximum level (`this` is [`GAME_SETTING_MAX_LEVEL`]).
const SETTING_GET_INT_PTR: u32 = 0x0043_d4d0;
const GAME_SETTING_MAX_LEVEL: u32 = 0x011d_0c60;
/// Virtual (vtable `+0x08`) of the object at `player + 0xa4`, called with 0x18.
const PLAYER_PART_OFFSET: u32 = 0xa4;
const VSLOT_PLAYER_PART_08: u32 = 0x08;
/// `GamePlayFormulas::GetRequiredExperiencePoints` (Xbox PDB), cdecl, a
/// level.
const GET_REQUIRED_EXPERIENCE_POINTS: u32 = 0x0064_8b50;
/// Virtual (vtable `+0x320`) on an actor: a byte test (ignoring crime).
const VSLOT_ACTOR_TEST_320: u32 = 0x320;
/// The object whose byte `IsGoreDisabled` reads, and its getter.
const GORE_OBJECT: u32 = 0x011d_f7f8;
const GORE_GET_BYTE_PTR: u32 = 0x0040_8d60;
/// The object `GetActorsInHigh` reads from.
const ACTORS_IN_HIGH_OBJECT: u32 = 0x011e_0e80;
/// `[this + 0x18]` getter used by the `GetReputation` debug lines: the name
/// string (`this` a form plus `0x18`).
const FORM_PART_GET_NAME: u32 = 0x0040_8da0;
/// Three reputation-like accessors of the form in ECX, one word pushed:
/// ST0, ST0, and a signed byte in AL.
const FORM_QUERY_6155C0: u32 = 0x0061_55c0;
const FORM_QUERY_616A60: u32 = 0x0061_6a60;
const FORM_QUERY_616A90: u32 = 0x0061_6a90;
/// The objective of the quest in ECX with the objective number pushed.
const QUEST_GET_OBJECTIVE: u32 = 0x0060_c8e0;
/// `cdecl` test of one word (`IsPlayerTagSkill`'s first test).
const TAG_SKILL_TEST: u32 = 0x0047_f060;
/// `this` the player, one word pushed: the class record, tested by
/// [`fn_005a5f40`].
const PLAYER_GET_CLASS: u32 = 0x0088_4350;
/// `this` the player: the hardcore-mode byte.
const PLAYER_IS_HARDCORE: u32 = 0x004d_1360;
/// Counterpart of [`fn_005a60d0`] (`this` the challenge), outside this unit
/// range.
const CHALLENGE_GET_COMPLETED: u32 = 0x005a_6150;

/// The VATS checks of this part share one shape: the value is the callee's
/// ST0 for the actor with `param1` and an angle, 0.0 for a non-actor.
/// `call` is (callee, angle, debug format).
fn vats_check(
    e: &mut Engine,
    reference: Ptr,
    param1: u32,
    result: Ptr,
    call: (u32, f32, u32),
) -> bool {
    let (callee, angle, format) = call;
    set_result(e, result, 0.0);
    if !reference.is_null() && e.vcall(reference.addr(), VSLOT_IS_ACTOR, &args![]).bool() {
        let value = e.call(callee, &args![reference, param1, angle]).f32();
        set_result(e, result, value as f64);
    }
    if trace_enabled(e) {
        trace_result(e, format, result);
    }
    true
}

// Translated from 005a5020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetConcussedConditionFunction` (Xbox PDB): for an actor, when
/// [`fn_005a50e0`] is set, 1.0 if the `0x21` test on the actor part says yes.
pub fn script_get_concussed_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() && fn_005a50e0(e) != 0 {
        if e.call(
            ACTOR_PART_TEST_822CF0,
            &args![actor.addr() + 0x94, 0x21u32, 0u32],
        )
        .bool()
        {
            set_result(e, result, 1.0);
        }
        // "%s is not concussed " / "%s is concussed"
        if let Some(format) = trace_choice(e, result, 0x0103_61d8, 0x0103_61c0) {
            let name = e.call(REF_GET_NAME, &args![actor]).u32();
            e.call(DEBUG_PRINT, &args![format, name]);
        }
    }
    true
}

// Translated from 005a50e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition helper without Xbox PDB name: the byte global at `011dbe50`.
pub fn fn_005a50e0(e: &mut Engine) -> u8 {
    e.global::<u8>(BYTE_GLOBAL_5A50E0)
}

// Translated from 005a50f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetRadiationLevelConditionFunction` (Xbox PDB): for an actor
/// with a process, the sum of three `float`s the process returns from its
/// virtuals `+0x76c`, `+0x75c` and `+0x764`.
pub fn script_get_radiation_level_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if actor.is_null() {
        return true;
    }
    set_result(e, result, 0.0);
    if !e
        .call(ACTOR_GET_PROCESS, &args![actor])
        .ptr::<()>()
        .is_null()
    {
        let first = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
        let second = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
        let third = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
        let a = e.vcall(third, VSLOT_PROCESS_RADIATION_A, &args![]).f32() as f64;
        let b = e.vcall(second, VSLOT_PROCESS_RADIATION_B, &args![]).f32() as f64;
        let c = e.vcall(first, VSLOT_PROCESS_RADIATION_C, &args![]).f32() as f64;
        set_result(e, result, c + b + a);
    }
    if trace_enabled(e) {
        let value = e.mem.f64(result.addr());
        let name = e.call(REF_GET_NAME, &args![actor]).u32();
        // "%s radiation level is %.2f"
        e.call(DEBUG_PRINT, &args![0x0103_61e8u32, name, value]);
    }
    true
}

// Translated from 005a51e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetMapMarkerVisibleConditionFunction` (Xbox PDB): 0.0 without
/// marker data, 1.0 when the marker is visible, 2.0 when it also has a
/// travel location. The debug line is printed for every call.
pub fn script_get_map_marker_visible_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    if !reference.is_null() {
        let data = e.call(REF_GET_MAP_MARKER_DATA, &args![reference]).u32();
        if data != 0 && e.call(MAP_MARKER_GET_VISIBLE, &args![data]).bool() {
            set_result(e, result, 1.0);
            if e.call(MAP_MARKER_GET_TRAVEL_LOC, &args![data]).bool() {
                let two: f64 = e.global(DOUBLE_TWO);
                set_result(e, result, two);
            }
        }
    }
    if trace_enabled(e) {
        // "marker state = %.0f"
        trace_result(e, 0x0103_6204, result);
    }
    true
}

// Translated from 005a5270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetWeaponHealthPercConditionFunction` (Xbox PDB): for an actor
/// with a process, the health of its weapon item, or 100.0 without one.
/// Does not clear the result first.
pub fn script_get_weapon_health_perc_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let actor = actor_of(e, reference);
    if !actor.is_null()
        && !e
            .call(ACTOR_GET_PROCESS, &args![actor])
            .ptr::<()>()
            .is_null()
    {
        let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
        let item = e.vcall(process, VSLOT_PROCESS_WEAPON_ITEM, &args![]).u32();
        let mut found = false;
        if item != 0 {
            let field = e.call(OBJECT_GET_FIELD_08, &args![item]).u32();
            if field != e.global::<u32>(WEAPON_COMPARE_GLOBAL) {
                let health = e
                    .call(ITEM_CHANGE_GET_ITEM_HEALTH, &args![item, 1u32])
                    .f32();
                set_result(e, result, health as f64);
                found = true;
            }
        }
        if !found {
            let hundred: f64 = e.global(DOUBLE_ONE_HUNDRED);
            set_result(e, result, hundred);
        }
    }
    if trace_enabled(e) {
        // "GetWeaponHealthPerc >> %0.2f"
        trace_result(e, 0x0103_6218, result);
    }
    true
}

// Translated from 005a5330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsGreetingPlayerConditionFunction` (Xbox PDB): 1.0 when the
/// actor's process says (virtual `+0x30c`) yes and the reference's parent
/// world is the one in the player global (`011dea3c`).
pub fn script_is_greeting_player_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null()
        && !e
            .call(ACTOR_GET_PROCESS, &args![actor])
            .ptr::<()>()
            .is_null()
    {
        let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
        if e.vcall(process, VSLOT_PROCESS_TEST_30C, &args![]).bool() {
            let world = e.call(REF_GET_PARENT_WORLD_SIMPLE, &args![actor]).u32();
            if world == e.global::<u32>(PLAYER) {
                set_result(e, result, 1.0);
            }
        }
    }
    if trace_enabled(e) {
        // "IsGreeting the player >> %.2f"
        trace_result(e, 0x0103_6238, result);
    }
    true
}

// Translated from 005a53e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsCombatTargetConditionFunction` (Xbox PDB): 1.0 when the actor
/// is in combat with the actor `param1`.
pub fn script_is_combat_target_condition_function(
    e: &mut Engine,
    reference: Ptr,
    param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null()
        && e.call(ACTOR_IS_IN_COMBAT_WITH_ACTOR, &args![actor, param1])
            .bool()
    {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "IsCombatTarget >> %.2f"
        trace_result(e, 0x0103_6258, result);
    }
    true
}

// Translated from 005a5460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetVATSRightAreaFreeConditionFunction` (Xbox PDB): the area
/// check to the right (angle pi / 2).
pub fn script_get_vats_right_area_free_condition_function(
    e: &mut Engine,
    reference: Ptr,
    param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let angle = e.global::<f32>(VATS_ANGLE_RIGHT);
    let call = (ACTOR_GET_VATS_AREA_FREE, angle, 0x0103_6270);
    vats_check(e, reference, param1, result, call)
}

// Translated from 005a54f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetVATSLeftAreaFreeConditionFunction` (Xbox PDB): the area
/// check to the left (angle 3 pi / 2).
pub fn script_get_vats_left_area_free_condition_function(
    e: &mut Engine,
    reference: Ptr,
    param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let angle = e.global::<f32>(VATS_ANGLE_LEFT);
    let call = (ACTOR_GET_VATS_AREA_FREE, angle, 0x0103_628c);
    vats_check(e, reference, param1, result, call)
}

// Translated from 005a5580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetVATSBackAreaFreeConditionFunction` (Xbox PDB): the area
/// check behind (angle pi).
pub fn script_get_vats_back_area_free_condition_function(
    e: &mut Engine,
    reference: Ptr,
    param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let angle = e.global::<f32>(VATS_ANGLE_BACK);
    let call = (ACTOR_GET_VATS_AREA_FREE, angle, 0x0103_62a8);
    vats_check(e, reference, param1, result, call)
}

// Translated from 005a5610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetVATSFrontAreaFreeConditionFunction` (Xbox PDB): the area
/// check in front (angle 0.0).
pub fn script_get_vats_front_area_free_condition_function(
    e: &mut Engine,
    reference: Ptr,
    param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let call = (ACTOR_GET_VATS_AREA_FREE, 0.0, 0x0103_62c4);
    vats_check(e, reference, param1, result, call)
}

// Translated from 005a5690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetVATSRightTargetVisibleConditionFunction` (Xbox PDB): the
/// target-visible check to the right (angle pi / 2).
pub fn script_get_vats_right_target_visible_condition_function(
    e: &mut Engine,
    reference: Ptr,
    param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let angle = e.global::<f32>(VATS_ANGLE_RIGHT);
    let call = (ACTOR_GET_VATS_TARGET_VISIBLE, angle, 0x0103_62e0);
    vats_check(e, reference, param1, result, call)
}

// Translated from 005a5720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetVATSLeftTargetVisibleConditionFunction` (Xbox PDB): the
/// target-visible check to the left (angle 3 pi / 2).
pub fn script_get_vats_left_target_visible_condition_function(
    e: &mut Engine,
    reference: Ptr,
    param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let angle = e.global::<f32>(VATS_ANGLE_LEFT);
    let call = (ACTOR_GET_VATS_TARGET_VISIBLE, angle, 0x0103_6300);
    vats_check(e, reference, param1, result, call)
}

// Translated from 005a57b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetVATSBackTargetVisibleConditionFunction` (Xbox PDB): the
/// target-visible check behind (angle pi).
pub fn script_get_vats_back_target_visible_condition_function(
    e: &mut Engine,
    reference: Ptr,
    param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let angle = e.global::<f32>(VATS_ANGLE_BACK);
    let call = (ACTOR_GET_VATS_TARGET_VISIBLE, angle, 0x0103_6320);
    vats_check(e, reference, param1, result, call)
}

// Translated from 005a5840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetVATSFrontTargetVisibleConditionFunction` (Xbox PDB): the
/// target-visible check in front (angle 0.0).
pub fn script_get_vats_front_target_visible_condition_function(
    e: &mut Engine,
    reference: Ptr,
    param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let call = (ACTOR_GET_VATS_TARGET_VISIBLE, 0.0, 0x0103_6340);
    vats_check(e, reference, param1, result, call)
}

// Translated from 005a58c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetXPForNextLevelConditionFunction` (Xbox PDB): the experience
/// points still needed for the player's next level; 0.0 (and a message)
/// at the maximum level.
pub fn script_get_xp_for_next_level_condition_function(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let player = e.global::<u32>(PLAYER);
    let level = e.call(PLAYER_GET_LEVEL, &args![player]).u16() as i32;
    let max_ptr = e
        .call(SETTING_GET_INT_PTR, &args![GAME_SETTING_MAX_LEVEL])
        .u32();
    let max_level = e.mem.u32(max_ptr) as i32;
    if level >= max_level {
        if trace_enabled(e) {
            // "Player already at maximum level."
            e.call(DEBUG_PRINT, &args![0x0103_637cu32]);
        }
        return true;
    }
    let player = e.global::<u32>(PLAYER);
    let current = e
        .vcall(
            player + PLAYER_PART_OFFSET,
            VSLOT_PLAYER_PART_08,
            &args![0x18u32],
        )
        .i32();
    let required = e
        .call(GET_REQUIRED_EXPERIENCE_POINTS, &args![(level + 1) as u32])
        .i32();
    set_result(e, result, required.wrapping_sub(current) as f64);
    if trace_enabled(e) {
        // "GetXPForNextLevel >> %.2f"
        trace_result(e, 0x0103_6360, result);
    }
    true
}

// Translated from 005a59a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetIgnoreCrimeConditionFunction` (Xbox PDB): 1.0 when the actor
/// answers its virtual `+0x320` test with yes.
pub fn script_get_ignore_crime_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() && e.vcall(actor.addr(), VSLOT_ACTOR_TEST_320, &args![]).bool() {
        set_result(e, result, 1.0);
    }
    // "%s is not ignoring crime" / "%s is ignoring crime"
    if let Some(format) = trace_choice(e, result, 0x0103_63bc, 0x0103_63a0) {
        let name = e.call(REF_GET_NAME, &args![reference]).u32();
        e.call(DEBUG_PRINT, &args![format, name]);
    }
    true
}

// Translated from 005a5a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsGoreDisabledConditionFunction` (Xbox PDB): 1.0 when the byte
/// the object at `011df7f8` hands out is exactly 1.
pub fn script_is_gore_disabled_condition_function(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let byte_ptr = e.call(GORE_GET_BYTE_PTR, &args![GORE_OBJECT]).u32();
    if e.mem.u8(byte_ptr) == 1 {
        set_result(e, result, 1.0);
    }
    if trace_enabled(e) {
        // "IsGoreDisabled >> %0.2f"
        trace_result(e, 0x0103_63d4, result);
    }
    true
}

// Translated from 005a5ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetActorsInHighConditionFunction` (Xbox PDB): the `int` that
/// [`fn_005a5b10`] reads from the object at `011e0e80`.
pub fn script_get_actors_in_high_condition_function(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    set_result(e, result, 0.0);
    let count = fn_005a5b10(e, Ptr::new(ACTORS_IN_HIGH_OBJECT)) as i32;
    set_result(e, result, count as f64);
    if trace_enabled(e) {
        // "GetActorsInHigh >> %0.2f"
        trace_result(e, 0x0103_63ec, result);
    }
    true
}

// Translated from 005a5b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Getter without Xbox PDB name: the word at `this + 0x103b0`.
pub fn fn_005a5b10(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x103b0)
}

// Translated from 005a5b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetForceHitReactionConditionFunction` (Xbox PDB): the byte at
/// actor `+0x1b1`. Does not clear the result first.
pub fn script_get_force_hit_reaction_condition_function(
    e: &mut Engine,
    reference: Ptr,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let actor = actor_of(e, reference);
    if !actor.is_null() {
        let value = fn_005a5bb0(e, actor);
        set_result(e, result, value as f64);
        if trace_enabled(e) {
            let value = e.mem.f64(result.addr());
            let name = e.call(REF_GET_NAME, &args![actor]).u32();
            // "GetForceHitReactionConditionFunction: %s >> %0.2f"
            e.call(DEBUG_PRINT, &args![0x0103_6408u32, name, value]);
        }
    }
    true
}

// Translated from 005a5bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Getter without Xbox PDB name: the byte at `this + 0x1b1`.
pub fn fn_005a5bb0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x1b1)
}

/// The three `GetReputation:` condition functions below print the same line
/// (a copy in the exe): the name through `FORM_PART_GET_NAME` of
/// `form + 0x18`, and the result.
fn trace_form_value(e: &mut Engine, form: Ptr, result: Ptr) {
    if trace_enabled(e) {
        let value = e.mem.f64(result.addr());
        let name = e.call(FORM_PART_GET_NAME, &args![form.addr() + 0x18]).u32();
        // "GetReputation: %s >> %0.2f"
        e.call(DEBUG_PRINT, &args![0x0103_643cu32, name, value]);
    }
}

// Translated from 005a5bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name (debug line "GetReputation"):
/// for a non-null form `param1`, the ST0 value of `006155c0(param1,
/// param2)`. Leaves the result alone for a null `param1`.
pub fn fn_005a5bd0(e: &mut Engine, _reference: u32, form: Ptr, param2: u32, result: Ptr) -> bool {
    if !form.is_null() {
        let value = e.call(FORM_QUERY_6155C0, &args![form, param2]).f64();
        set_result(e, result, value);
        trace_form_value(e, form, result);
    }
    true
}

// Translated from 005a5c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name (debug line "GetReputation"):
/// like [`fn_005a5bd0`] with `00616a60`.
pub fn fn_005a5c40(e: &mut Engine, _reference: u32, form: Ptr, param2: u32, result: Ptr) -> bool {
    if !form.is_null() {
        let value = e.call(FORM_QUERY_616A60, &args![form, param2]).f64();
        set_result(e, result, value);
        trace_form_value(e, form, result);
    }
    true
}

// Translated from 005a5cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name (debug line "GetReputation"):
/// for a non-null form `param1`, the signed byte `00616a90(param1, low
/// byte of param2)` returns.
pub fn fn_005a5cb0(e: &mut Engine, _reference: u32, form: Ptr, param2: u32, result: Ptr) -> bool {
    if !form.is_null() {
        let value = e.call(FORM_QUERY_616A90, &args![form, param2 & 0xff]).u8() as i8;
        set_result(e, result, value as f64);
        trace_form_value(e, form, result);
    }
    true
}

// Translated from 005a5d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name (debug line
/// "GetObjectiveCompleted"): 1.0 when the objective `param2` of the quest
/// `param1` exists and [`fn_005a5dc0`] says so, else 0.0. Leaves the result
/// alone when there is no quest or objective.
pub fn fn_005a5d30(e: &mut Engine, _reference: u32, quest: Ptr, param2: u32, result: Ptr) -> bool {
    if !quest.is_null() {
        let objective = e.call(QUEST_GET_OBJECTIVE, &args![quest, param2]).u32();
        if objective != 0 {
            let done = fn_005a5dc0(e, Ptr::new(objective));
            set_result(e, result, if done { 1.0 } else { 0.0 });
            if trace_enabled(e) {
                // "GetObjectiveCompleted >> %0.2f"
                trace_result(e, 0x0103_6458, result);
            }
        }
    }
    true
}

// Translated from 005a5dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Objective test without Xbox PDB name: the `int` at `this + 0x20` is
/// greater than 1.
pub fn fn_005a5dc0(e: &mut Engine, this: Ptr) -> bool {
    (e.mem.u32(this.addr() + 0x20) as i32) > 1
}

// Translated from 005a5de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name (debug line
/// "GetObjectiveDisplayed"): like [`fn_005a5d30`] with [`fn_005a5e70`].
pub fn fn_005a5de0(e: &mut Engine, _reference: u32, quest: Ptr, param2: u32, result: Ptr) -> bool {
    if !quest.is_null() {
        let objective = e.call(QUEST_GET_OBJECTIVE, &args![quest, param2]).u32();
        if objective != 0 {
            let shown = fn_005a5e70(e, Ptr::new(objective)) as u8 != 0;
            set_result(e, result, if shown { 1.0 } else { 0.0 });
            if trace_enabled(e) {
                // "GetObjectiveDisplayed >> %0.2f"
                trace_result(e, 0x0103_6478, result);
            }
        }
    }
    true
}

// Translated from 005a5e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Objective bit test without Xbox PDB name: bit 0 of the word at
/// `this + 0x20`.
pub fn fn_005a5e70(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x20) & 1
}

// Translated from 005a5e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Condition function without Xbox PDB name (debug line
/// "IsPlayerTagSkill"): for an actor, 1.0 when `param1` passes the test at
/// `0047f060` and equals one of the four words at `+0x44..+0x50` of the
/// record `00884350` returns for the player ([`fn_005a5f40`]).
pub fn fn_005a5e90(e: &mut Engine, reference: Ptr, param1: u32, _param2: u32, result: Ptr) -> bool {
    set_result(e, result, 0.0);
    let actor = actor_of(e, reference);
    if !actor.is_null() && e.call(TAG_SKILL_TEST, &args![param1]).bool() {
        let player = e.global::<u32>(PLAYER);
        let class = e.call(PLAYER_GET_CLASS, &args![player]).u32();
        if fn_005a5f40(e, Ptr::new(class), param1) {
            set_result(e, result, 1.0);
        }
    }
    if trace_enabled(e) {
        // "IsPlayerTagSkill >> %.0f"
        trace_result(e, 0x0103_6498, result);
    }
    true
}

// Translated from 005a5f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Test without Xbox PDB name (`this` a record, one word): whether the word
/// equals one of the four at `this + 0x44`, `0x48`, `0x4c` and `0x50`.
pub fn fn_005a5f40(e: &mut Engine, this: Ptr, value: u32) -> bool {
    [0x44u32, 0x48, 0x4c, 0x50]
        .iter()
        .any(|offset| e.mem.u32(this.addr() + offset) == value)
}

// Translated from 005a5f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsHardcoreConditionalFunction` (Xbox PDB): the byte `004d1360`
/// returns for the player, when the player exists. Leaves the result alone
/// otherwise.
pub fn script_is_hardcore_conditional_function(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let player = e.global::<u32>(PLAYER);
    if player != 0 {
        let value = e.call(PLAYER_IS_HARDCORE, &args![player]).u8();
        set_result(e, result, value as f64);
    }
    if trace_enabled(e) {
        // "IsHardcore? >> %f"
        trace_result(e, 0x0103_64b4, result);
    }
    true
}

// Translated from 005a5ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsAlwaysHardcoreConditionalFunction` (Xbox PDB): the byte at
/// `+0xe38` of the player ([`fn_005a6050`]), when the player exists.
pub fn script_is_always_hardcore_conditional_function(
    e: &mut Engine,
    _reference: u32,
    _param1: u32,
    _param2: u32,
    result: Ptr,
) -> bool {
    let player = e.global::<u32>(PLAYER);
    if player != 0 {
        let value = fn_005a6050(e, Ptr::new(player));
        set_result(e, result, value as f64);
    }
    if trace_enabled(e) {
        // "GetAlwaysHardcore? >> %f"
        trace_result(e, 0x0103_64c8, result);
    }
    true
}

// Translated from 005a6050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Getter without Xbox PDB name: the byte at `this + 0xe38`.
pub fn fn_005a6050(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0xe38)
}

// Translated from 005a6070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ChallengeLockedConditionFunction` (Xbox PDB): bit 0 of the word
/// at `param1 + 0x70` ([`fn_005a60d0`]), for a non-null `param1`.
pub fn script_challenge_locked_condition_function(
    e: &mut Engine,
    _reference: u32,
    param1: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    if !param1.is_null() {
        let value = fn_005a60d0(e, param1);
        set_result(e, result, value as i32 as f64);
    }
    if trace_enabled(e) {
        // "ChallengeLocked? >> %f"
        trace_result(e, 0x0103_64e4, result);
    }
    true
}

// Translated from 005a60d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Getter without Xbox PDB name: bit 0 of the word at `this + 0x70`.
pub fn fn_005a60d0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x70) & 1
}

// Translated from 005a60f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetChallengeCompletedConditionFunction` (Xbox PDB): the `int`
/// `005a6150` returns for a non-null `param1`.
pub fn script_get_challenge_completed_condition_function(
    e: &mut Engine,
    _reference: u32,
    param1: Ptr,
    _param2: u32,
    result: Ptr,
) -> bool {
    if !param1.is_null() {
        let value = e.call(CHALLENGE_GET_COMPLETED, &args![param1]).i32();
        set_result(e, result, value as f64);
    }
    if trace_enabled(e) {
        // "ChallengeComplete? >> %f"
        trace_result(e, 0x0103_64fc, result);
    }
    true
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005a2a50, fn_005a2a50(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a2a60, script_get_ghost_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a2af0, script_get_offers_service_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a2ba0, script_get_barter_gold_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a2c30, fn_005a2c30(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a2c70, script_is_time_passing_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a2ce0, script_get_armor_rating_upper_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a2e20, fn_005a2e20(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x005a2ed0, script_get_cell_ownership_condition_function(u32, Ptr, Ptr, Ptr) -> bool),
        entry!(0x005a2f60, fn_005a2f60(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a3010, script_is_running_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a30c0, script_get_friend_hit_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a3180, script_is_in_combat_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a3270, script_is_in_interior_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a3310, script_get_pc_misc_stat_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a3370, fn_005a3370(u32) -> u32),
        entry!(0x005a3390, script_is_actor_evil_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a3460, script_is_actor_victim_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a3570, fn_005a3570(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a35f0, fn_005a35f0(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a3650, fn_005a3650(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a3670, fn_005a3670(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a36b0, script_is_in_dangerous_water_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a3740, fn_005a3740(Ptr) -> u8),
        entry!(0x005a3760, fn_005a3760(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a3790, fn_005a3790(Ptr) -> bool),
        entry!(0x005a37b0, script_is_xbox_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a3820, script_is_ps3_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a3870, script_is_pc_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a38e0, fn_005a38e0()),
        entry!(0x005a3980, script_is_essential_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a39d0, fn_005a39d0(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a3a00, fn_005a3a00(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a3a30, fn_005a3a30(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a3af0, fn_005a3af0(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a3b20, script_is_player_action_active_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a3b90, fn_005a3b90(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a3bf0, fn_005a3bf0(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a3c30, fn_005a3c30(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a3c90, fn_005a3c90(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a3d00, fn_005a3d00(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a3d30, script_get_causeof_death_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a3db0, script_is_limb_gone_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a3e10, script_get_killing_blow_limb_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a3e90, fn_005a3e90(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a3f00, script_get_killer_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a3fa0, script_get_killer_object_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a4090, fn_005a4090(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a40d0, fn_005a40d0(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a4160, fn_005a4160(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a41c0, script_get_anim_action_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a4210, fn_005a4210(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a4240, fn_005a4240(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a42b0, fn_005a42b0(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a4320, fn_005a4320(Ptr) -> u32),
        entry!(0x005a4340, fn_005a4340(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a43b0, fn_005a43b0(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a4410, script_get_is_object_type_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a4480, script_get_dialogue_emotion_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a4520, fn_005a4520(Ptr) -> u8),
        entry!(0x005a4540, script_get_dialogue_emotion_value_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a45c0, script_is_water_object_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a4630, script_has_perk_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a46d0, script_get_faction_relation_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a4780, script_is_last_played_idle_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a4820, script_get_player_teammate_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a48a0, script_get_player_teammate_count_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a4910, fn_005a4910(Ptr) -> u32),
        entry!(0x005a4930, script_get_actor_crime_player_enemy_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a49f0, script_get_actor_faction_player_enemy_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a4aa0, script_get_minor_crime_count_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a4b40, fn_005a4b40(Ptr) -> i32),
        entry!(0x005a4b60, script_get_major_crime_count_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a4c00, fn_005a4c00(Ptr) -> u32),
        entry!(0x005a4c20, fn_005a4c20(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a4c60, script_get_destruction_stage_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a4cd0, script_get_threat_ratio_condition_function(Ptr, Ptr, u32, Ptr) -> bool),
        entry!(0x005a4dd0, script_get_is_alignment_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a4ea0, script_get_is_used_item_equip_type_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a4f30, script_get_aggro_radius_violated_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a5020, script_get_concussed_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a50e0, fn_005a50e0() -> u8),
        entry!(0x005a50f0, script_get_radiation_level_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a51e0, script_get_map_marker_visible_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a5270, script_get_weapon_health_perc_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a5330, script_is_greeting_player_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a53e0, script_is_combat_target_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a5460, script_get_vats_right_area_free_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a54f0, script_get_vats_left_area_free_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a5580, script_get_vats_back_area_free_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a5610, script_get_vats_front_area_free_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a5690, script_get_vats_right_target_visible_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a5720, script_get_vats_left_target_visible_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a57b0, script_get_vats_back_target_visible_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a5840, script_get_vats_front_target_visible_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a58c0, script_get_xp_for_next_level_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a59a0, script_get_ignore_crime_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a5a50, script_is_gore_disabled_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a5ab0, script_get_actors_in_high_condition_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a5b10, fn_005a5b10(Ptr) -> u32),
        entry!(0x005a5b30, script_get_force_hit_reaction_condition_function(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a5bb0, fn_005a5bb0(Ptr) -> u8),
        entry!(0x005a5bd0, fn_005a5bd0(u32, Ptr, u32, Ptr) -> bool),
        entry!(0x005a5c40, fn_005a5c40(u32, Ptr, u32, Ptr) -> bool),
        entry!(0x005a5cb0, fn_005a5cb0(u32, Ptr, u32, Ptr) -> bool),
        entry!(0x005a5d30, fn_005a5d30(u32, Ptr, u32, Ptr) -> bool),
        entry!(0x005a5dc0, fn_005a5dc0(Ptr) -> bool),
        entry!(0x005a5de0, fn_005a5de0(u32, Ptr, u32, Ptr) -> bool),
        entry!(0x005a5e70, fn_005a5e70(Ptr) -> u32),
        entry!(0x005a5e90, fn_005a5e90(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x005a5f40, fn_005a5f40(Ptr, u32) -> bool),
        entry!(0x005a5f90, script_is_hardcore_conditional_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a5ff0, script_is_always_hardcore_conditional_function(u32, u32, u32, Ptr) -> bool),
        entry!(0x005a6050, fn_005a6050(Ptr) -> u8),
        entry!(0x005a6070, script_challenge_locked_condition_function(u32, Ptr, u32, Ptr) -> bool),
        entry!(0x005a60d0, fn_005a60d0(Ptr) -> u32),
        entry!(0x005a60f0, script_get_challenge_completed_condition_function(u32, Ptr, u32, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What `*result` holds before a call; a function that leaves the result
    /// alone leaves this.
    const SENTINEL: f64 = -7.5;
    /// Virtual `IsActor` answering yes / no.
    const IS_ACTOR_YES: u32 = 0x0f00_0001;
    const IS_ACTOR_NO: u32 = 0x0f00_0002;
    /// A virtual that answers yes / no for the other byte-test slots.
    const SLOT_YES: u32 = 0x0f00_0003;
    const SLOT_NO: u32 = 0x0f00_0004;

    /// An engine with the constants the functions read from the exe's data,
    /// the debug print and the two `IsActor` answers.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000u32,
            0x0101_2000,
            0x0101_a000,
            0x0119_f000,
            0x011c_6000,
            0x011c_a000,
            0x011d_e000,
            0x0101_d000,
            0x0102_1000,
            0x0118_6000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(DOUBLE_TWO, 2.0f64);
        e.set_global(DOUBLE_ZERO, 0.0f64);
        e.set_global(DOUBLE_MINUS_ONE, -1.0f64);
        e.register(DEBUG_PRINT, |_, _| Ret::default());
        // `_ftol2`: the `double` argument truncated.
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            (value as i32 as u32).into_ret()
        });
        e.register(IS_ACTOR_YES, |_, _| true.into_ret());
        e.register(IS_ACTOR_NO, |_, _| false.into_ret());
        e.register(SLOT_YES, |_, _| true.into_ret());
        e.register(SLOT_NO, |_, _| false.into_ret());
        // The name functions return the argument plus one (or 0x100 for the
        // reference version), so the debug lines show which object was named.
        e.register(FORM_GET_FULL_NAME, |_, a| (a[0] + 1).into_ret());
        e.register(REF_GET_NAME, |_, a| (a[0] + 0x100).into_ret());
        e
    }

    /// Calls the condition function at `addr` with `*result` starting at
    /// `initial`; returns what it returned and what it left in `*result`.
    fn run_from(
        e: &mut Engine,
        initial: f64,
        addr: u32,
        reference: u32,
        param1: u32,
        param2: u32,
    ) -> (bool, f64) {
        let result = e.mem.alloc(8);
        e.mem.set_f64(result, initial);
        let returned = e
            .call(addr, &args![reference, param1, param2, result])
            .bool();
        (returned, e.mem.f64(result))
    }

    fn run(e: &mut Engine, addr: u32, reference: u32, param1: u32, param2: u32) -> (bool, f64) {
        run_from(e, SENTINEL, addr, reference, param1, param2)
    }

    /// A function that returns `eax`.
    fn stub(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    /// A function that returns `value` in ST0.
    fn stub_st0(e: &mut Engine, addr: u32, value: f64) {
        e.register_double(addr, move |_, _| Ret {
            st0: value,
            ..Ret::default()
        });
    }

    /// An object whose vtable has the given (offset, function) slots.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x800);
        for &(offset, function) in slots {
            e.mem.set_u32(vtable + offset, function);
        }
        let object = e.mem.alloc(0x300);
        e.mem.set_u32(object, vtable);
        object
    }

    fn actor(e: &mut Engine) -> u32 {
        object(e, &[(VSLOT_IS_ACTOR, IS_ACTOR_YES)])
    }

    fn non_actor(e: &mut Engine) -> u32 {
        object(e, &[(VSLOT_IS_ACTOR, IS_ACTOR_NO)])
    }

    /// Turns the debug lines on and starts recording calls.
    fn trace_on(e: &mut Engine) {
        let tls = e.tls();
        e.mem.set_u8(tls + TLS_TRACE_FLAG, 1);
        e.call_log = Some(vec![]);
    }

    type Log = Vec<(u32, Vec<u32>)>;

    fn take_log(e: &mut Engine) -> Log {
        e.call_log.take().unwrap_or_default()
    }

    /// The argument words of every call to `addr`.
    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn words(value: f64) -> [u32; 2] {
        let bits = value.to_bits();
        [bits as u32, (bits >> 32) as u32]
    }

    /// The debug lines printed by one call with the lines on.
    fn printed(e: &mut Engine, initial: f64, addr: u32, reference: u32, p1: u32, p2: u32) -> Log {
        trace_on(e);
        run_from(e, initial, addr, reference, p1, p2);
        let log = take_log(e);
        log.into_iter().filter(|(a, _)| *a == DEBUG_PRINT).collect()
    }

    #[test]
    fn a2a50_is_always_zero() {
        let mut e = engine();
        assert_eq!(run(&mut e, 0x005a_2a50, 0x1000, 1, 2), (true, 0.0));
    }

    #[test]
    fn ghost_asks_the_actor() {
        let mut e = engine();
        let a = actor(&mut e);
        stub(&mut e, ACTOR_IS_GHOST, 1);
        assert_eq!(run(&mut e, 0x005a_2a60, a, 0, 0), (true, 1.0));
        stub(&mut e, ACTOR_IS_GHOST, 0);
        assert_eq!(run(&mut e, 0x005a_2a60, a, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_2a60, plain, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x005a_2a60, 0, 0, 0), (true, 0.0));
        stub(&mut e, ACTOR_IS_GHOST, 1);
        let lines = printed(&mut e, 0.0, 0x005a_2a60, a, 0, 0);
        let w = words(1.0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5bfc, w[0], w[1]])]);
    }

    #[test]
    fn offers_service_names_the_actor_in_the_debug_line() {
        let mut e = engine();
        let a = actor(&mut e);
        stub(&mut e, ACTOR_OFFERS_SERVICES, 1);
        assert_eq!(run(&mut e, 0x005a_2af0, a, 0, 0), (true, 1.0));
        stub(&mut e, ACTOR_OFFERS_SERVICES, 0);
        assert_eq!(run(&mut e, 0x005a_2af0, a, 0, 0), (true, 0.0));
        stub(&mut e, ACTOR_OFFERS_SERVICES, 1);
        let lines = printed(&mut e, 0.0, 0x005a_2af0, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5c2c, a + 0x100])]);
        // Not an actor: result 0, and the "not offering" line names null.
        let plain = non_actor(&mut e);
        let lines = printed(&mut e, 0.0, 0x005a_2af0, plain, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5c10, 0x100])]);
    }

    #[test]
    fn barter_gold_is_the_base_gold_and_leaves_non_actors_alone() {
        let mut e = engine();
        let a = actor(&mut e);
        stub(&mut e, ACTOR_GET_BARTER_GOLD_BASE, 250);
        assert_eq!(run(&mut e, 0x005a_2ba0, a, 0, 0), (true, 250.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_2ba0, plain, 0, 0), (true, SENTINEL));
        let lines = printed(&mut e, 0.0, 0x005a_2ba0, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5c48, a, 250])]);
        assert!(printed(&mut e, 0.0, 0x005a_2ba0, plain, 0, 0).is_empty());
    }

    #[test]
    fn c30_is_always_zero() {
        let mut e = engine();
        let a = actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_2c30, a, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x005a_2c30, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn time_passing_follows_the_player_and_keeps_an_unset_result() {
        let mut e = engine();
        let player = actor(&mut e);
        e.set_global(PLAYER, player);
        e.register_double(PLAYER_IS_SLEEPING_OR_RESTING, move |_, a| {
            assert_eq!(a, [player]);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_2c70, 0, 0, 0), (true, 1.0));
        stub(&mut e, PLAYER_IS_SLEEPING_OR_RESTING, 0);
        assert_eq!(run(&mut e, 0x005a_2c70, 0, 0, 0), (true, SENTINEL));
        let lines = printed(&mut e, 0.0, 0x005a_2c70, 0, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5c6c])]);
        stub(&mut e, PLAYER_IS_SLEEPING_OR_RESTING, 1);
        let lines = printed(&mut e, 0.0, 0x005a_2c70, 0, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5c80])]);
    }

    #[test]
    fn armor_rating_upper_reads_the_worn_armor_and_destroys_the_item() {
        let mut e = engine();
        let a = actor(&mut e);
        stub(&mut e, GET_INVENTORY_CHANGES, 0x7000);
        e.register_double(INVENTORY_GET_WORN_ITEM, |_, args| {
            assert_eq!(args, [0x7000, 2, 0]);
            0x7100u32.into_ret()
        });
        stub(&mut e, OBJECT_GET_FIELD_08, 0x7200);
        stub(&mut e, FORM_GET_TYPE, 0x18);
        stub(&mut e, ARMOR_GET_CLASS_FLAG, 0);
        stub(&mut e, ITEM_CHANGE_DESTROY, 0);
        e.call_log = Some(vec![]);
        assert_eq!(run(&mut e, 0x005a_2ce0, a, 0, 0), (true, 1.0));
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, ITEM_CHANGE_DESTROY), vec![vec![0x7100, 1]]);
        assert_eq!(calls_to(&log, GET_INVENTORY_CHANGES), vec![vec![a]]);
        // The armor test answers: the exe's 2.0.
        stub(&mut e, ARMOR_GET_CLASS_FLAG, 2);
        assert_eq!(run(&mut e, 0x005a_2ce0, a, 0, 0), (true, 2.0));
        // A form of another type: 0.0, but the item is still destroyed.
        stub(&mut e, FORM_GET_TYPE, 0x28);
        e.call_log = Some(vec![]);
        assert_eq!(run(&mut e, 0x005a_2ce0, a, 0, 0), (true, 0.0));
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, ITEM_CHANGE_DESTROY).len(), 1);
        // Nothing worn / no changes / not an actor.
        stub(&mut e, INVENTORY_GET_WORN_ITEM, 0);
        e.call_log = Some(vec![]);
        assert_eq!(run(&mut e, 0x005a_2ce0, a, 0, 0), (true, 0.0));
        assert!(calls_to(&take_log(&mut e), ITEM_CHANGE_DESTROY).is_empty());
        stub(&mut e, GET_INVENTORY_CHANGES, 0);
        assert_eq!(run(&mut e, 0x005a_2ce0, a, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_2ce0, plain, 0, 0), (true, 0.0));
        let lines = printed(&mut e, 0.0, 0x005a_2ce0, plain, 0, 0);
        let w = words(0.0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5c90, w[0], w[1]])]);
    }

    #[test]
    fn e20_compares_the_extra_data_owner_with_the_form() {
        let mut e = engine();
        let player = actor(&mut e);
        e.set_global(PLAYER, player);
        stub(&mut e, REF_GET_EXTRA_DATA_LIST, 0x8000);
        e.register_double(EXTRA_LIST_GET_OWNER, |_, a| {
            assert_eq!(a, [0x8000]);
            0x9000u32.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_2e20, 0x2000, 0x9000, 0), (true, 1.0));
        // A different form leaves the result as it was.
        assert_eq!(
            run(&mut e, 0x005a_2e20, 0x2000, 0x9100, 0),
            (true, SENTINEL)
        );
        // A null reference too.
        assert_eq!(run(&mut e, 0x005a_2e20, 0, 0x9000, 0), (true, SENTINEL));
        // A null form: the player's base form is used.
        e.register_double(REF_GET_BASE_FORM, move |_, a| {
            assert_eq!(a, [player]);
            0x9000u32.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_2e20, 0x2000, 0, 0), (true, 1.0));
        let lines = printed(&mut e, 0.0, 0x005a_2e20, 0x2000, 0x9000, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5cc8, 0x9001])]);
        let lines = printed(&mut e, 0.0, 0x005a_2e20, 0x2000, 0x9100, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5cb4, 0x9101])]);
    }

    #[test]
    fn cell_ownership_compares_the_cell_owner_with_the_form() {
        let mut e = engine();
        e.register_double(CELL_GET_OWNER, |_, a| {
            assert_eq!(a, [0x3000]);
            0x9000u32.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_2ed0, 0, 0x3000, 0x9000), (true, 1.0));
        assert_eq!(
            run(&mut e, 0x005a_2ed0, 0, 0x3000, 0x9100),
            (true, SENTINEL)
        );
        let lines = printed(&mut e, 0.0, 0x005a_2ed0, 0, 0x3000, 0x9100);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5cb4, 0x9101])]);
        let lines = printed(&mut e, 0.0, 0x005a_2ed0, 0, 0x3000, 0x9000);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5cc8, 0x9001])]);
    }

    #[test]
    fn f60_asks_the_sneaking_test_even_without_an_actor() {
        let mut e = engine();
        let a = actor(&mut e);
        stub(&mut e, ACTOR_IS_SNEAKING, 1);
        assert_eq!(run(&mut e, 0x005a_2f60, a, 0, 0), (true, 1.0));
        stub(&mut e, ACTOR_IS_SNEAKING, 0);
        assert_eq!(run(&mut e, 0x005a_2f60, a, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        e.register_double(ACTOR_IS_SNEAKING, |_, a| {
            assert_eq!(a, [0]);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_2f60, plain, 0, 0), (true, 1.0));
        stub(&mut e, ACTOR_IS_SNEAKING, 1);
        let lines = printed(&mut e, 0.0, 0x005a_2f60, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5cec, a + 1])]);
        stub(&mut e, ACTOR_IS_SNEAKING, 0);
        let lines = printed(&mut e, 0.0, 0x005a_2f60, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5cd8, a + 1])]);
    }

    #[test]
    fn is_running_asks_the_actor_even_without_one() {
        let mut e = engine();
        let a = actor(&mut e);
        stub(&mut e, ACTOR_IS_RUNNING, 1);
        assert_eq!(run(&mut e, 0x005a_3010, a, 0, 0), (true, 1.0));
        stub(&mut e, ACTOR_IS_RUNNING, 0);
        assert_eq!(run(&mut e, 0x005a_3010, a, 0, 0), (true, 0.0));
        e.register_double(ACTOR_IS_RUNNING, |_, a| {
            assert_eq!(a, [0]);
            false.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_3010, 0, 0, 0), (true, 0.0));
        stub(&mut e, ACTOR_IS_RUNNING, 1);
        let lines = printed(&mut e, 0.0, 0x005a_3010, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5d10, a + 1])]);
        stub(&mut e, ACTOR_IS_RUNNING, 0);
        let lines = printed(&mut e, 0.0, 0x005a_3010, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5cfc, a + 1])]);
    }

    #[test]
    fn friend_hit_is_the_extra_data_count_and_names_the_player_and_actor() {
        let mut e = engine();
        let a = actor(&mut e);
        let player = actor(&mut e);
        e.set_global(PLAYER, player);
        stub(&mut e, REF_GET_EXTRA_DATA_LIST, 0x8000);
        e.register_double(EXTRA_LIST_GET_FRIEND_HIT_COUNT, |_, args| {
            assert_eq!(args, [0x8000]);
            3u32.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_30c0, a, 0, 0), (true, 3.0));
        let lines = printed(&mut e, 0.0, 0x005a_30c0, a, 0, 0);
        let w = words(3.0);
        assert_eq!(
            lines,
            vec![(
                DEBUG_PRINT,
                vec![0x0103_5d20, player + 1, a + 1, w[0], w[1]]
            )]
        );
        // Not an actor: 0.0 and no debug line.
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_30c0, plain, 0, 0), (true, 0.0));
        assert!(printed(&mut e, 0.0, 0x005a_30c0, plain, 0, 0).is_empty());
    }

    #[test]
    fn in_combat_uses_the_player_function_for_the_player() {
        let mut e = engine();
        let a = actor(&mut e);
        let player = actor(&mut e);
        e.set_global(PLAYER, player);
        stub(&mut e, ACTOR_IN_COMBAT_FLAG, 1);
        assert_eq!(run(&mut e, 0x005a_3180, a, 0, 0), (true, 1.0));
        stub(&mut e, ACTOR_IN_COMBAT_FLAG, 0);
        assert_eq!(run(&mut e, 0x005a_3180, a, 0, 0), (true, 0.0));
        // The player: the flag says yes, the player's own test says no.
        stub(&mut e, ACTOR_IN_COMBAT_FLAG, 1);
        e.register_double(PLAYER_IS_IN_COMBAT, move |e, args| {
            assert_eq!(args[0], player);
            // The out byte starts as 0.
            assert_eq!(e.mem.u8(args[1]), 0);
            0u32.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_3180, player, 0, 0), (true, 0.0));
        stub(&mut e, PLAYER_IS_IN_COMBAT, 0x101);
        assert_eq!(run(&mut e, 0x005a_3180, player, 0, 0), (true, 1.0));
        // A non-actor: nothing is asked.
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_3180, plain, 0, 0), (true, 0.0));
        let lines = printed(&mut e, 0.0, 0x005a_3180, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5d50, a + 1])]);
        stub(&mut e, ACTOR_IN_COMBAT_FLAG, 0);
        let lines = printed(&mut e, 0.0, 0x005a_3180, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5d3c, a + 1])]);
    }

    #[test]
    fn in_interior_asks_the_parent_cell() {
        let mut e = engine();
        e.register_double(REF_GET_PARENT_CELL, |_, a| {
            assert_eq!(a, [0x2000]);
            0x6000u32.into_ret()
        });
        e.register_double(CELL_IS_INTERIOR, |_, a| {
            assert_eq!(a, [0x6000]);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_3270, 0x2000, 0, 0), (true, 1.0));
        stub(&mut e, CELL_IS_INTERIOR, 0);
        assert_eq!(run(&mut e, 0x005a_3270, 0x2000, 0, 0), (true, 0.0));
        stub(&mut e, REF_GET_PARENT_CELL, 0);
        assert_eq!(run(&mut e, 0x005a_3270, 0x2000, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x005a_3270, 0, 0, 0), (true, 0.0));
        let lines = printed(&mut e, 0.0, 0x005a_3270, 0x2000, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5d60, 0x2001])]);
        stub(&mut e, REF_GET_PARENT_CELL, 0x6000);
        stub(&mut e, CELL_IS_INTERIOR, 1);
        let lines = printed(&mut e, 0.0, 0x005a_3270, 0x2000, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5d78, 0x2001])]);
    }

    /// The table of misc stats: entry `index` points at a block whose second
    /// word is the value.
    fn misc_stat(e: &mut Engine, index: u32, value: u32) {
        let block = e.mem.alloc(8);
        e.mem.set_u32(block + 4, value);
        e.mem.set_u32(MISC_STAT_TABLE + 4 * index, block);
    }

    #[test]
    fn pc_misc_stat_reads_the_indexed_entry() {
        let mut e = engine();
        misc_stat(&mut e, 2, 42);
        misc_stat(&mut e, 3, (-1i32) as u32);
        assert_eq!(run(&mut e, 0x005a_3310, 0, 2, 0), (true, 42.0));
        assert_eq!(run(&mut e, 0x005a_3310, 0, 3, 0), (true, -1.0));
        let lines = printed(&mut e, 0.0, 0x005a_3310, 0, 2, 0);
        let w = words(42.0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5d90, w[0], w[1]])]);
    }

    #[test]
    fn a3370_reads_the_value_of_the_table_entry() {
        let mut e = engine();
        misc_stat(&mut e, 5, 1234);
        assert_eq!(e.call(0x005a_3370, &args![5u32]).u32(), 1234);
    }

    #[test]
    fn actor_evil_asks_the_faction_test_and_prints_only_for_actors() {
        let mut e = engine();
        let a = actor(&mut e);
        stub(&mut e, REF_IS_PART_OF_EVIL_FACTION, 1);
        assert_eq!(run(&mut e, 0x005a_3390, a, 0, 0), (true, 1.0));
        stub(&mut e, REF_IS_PART_OF_EVIL_FACTION, 0);
        assert_eq!(run(&mut e, 0x005a_3390, a, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_3390, plain, 0, 0), (true, 0.0));
        let lines = printed(&mut e, 0.0, 0x005a_3390, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5db0, a + 0x100])]);
        stub(&mut e, REF_IS_PART_OF_EVIL_FACTION, 1);
        let lines = printed(&mut e, 0.0, 0x005a_3390, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5dc4, a + 0x100])]);
        assert!(printed(&mut e, 0.0, 0x005a_3390, plain, 0, 0).is_empty());
    }

    #[test]
    fn actor_victim_compares_the_base_form_with_the_tls_victim() {
        let mut e = engine();
        let a = object(
            &mut e,
            &[
                (VSLOT_IS_ACTOR, IS_ACTOR_YES),
                (VSLOT_ACTOR_TEST_218, SLOT_YES),
            ],
        );
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CRIME_VICTIM, 0x5000);
        stub(&mut e, REF_GET_BASE_FORM, 0x5000);
        assert_eq!(run(&mut e, 0x005a_3460, a, 0, 0), (true, 1.0));
        stub(&mut e, REF_GET_BASE_FORM, 0x5100);
        assert_eq!(run(&mut e, 0x005a_3460, a, 0, 0), (true, 0.0));
        // A null base form never matches (even a null victim).
        e.mem.set_u32(tls + TLS_CRIME_VICTIM, 0);
        stub(&mut e, REF_GET_BASE_FORM, 0);
        assert_eq!(run(&mut e, 0x005a_3460, a, 0, 0), (true, 0.0));
        // The virtual `+0x218` says no: nothing else runs, no debug line.
        let b = object(
            &mut e,
            &[
                (VSLOT_IS_ACTOR, IS_ACTOR_YES),
                (VSLOT_ACTOR_TEST_218, SLOT_NO),
            ],
        );
        assert_eq!(run(&mut e, 0x005a_3460, b, 0, 0), (true, 0.0));
        assert!(printed(&mut e, 0.0, 0x005a_3460, b, 0, 0).is_empty());
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_3460, plain, 0, 0), (true, 0.0));
        e.mem.set_u32(tls + TLS_CRIME_VICTIM, 0x5000);
        stub(&mut e, REF_GET_BASE_FORM, 0x5000);
        let lines = printed(&mut e, 0.0, 0x005a_3460, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5df0, a + 0x100])]);
        stub(&mut e, REF_GET_BASE_FORM, 0x5100);
        let lines = printed(&mut e, 0.0, 0x005a_3460, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5dd4, a + 0x100])]);
    }

    #[test]
    fn a3570_asks_the_process() {
        let mut e = engine();
        let a = actor(&mut e);
        let process = object(&mut e, &[(VSLOT_PROCESS_TEST_60, SLOT_YES)]);
        stub(&mut e, ACTOR_GET_PROCESS, process);
        assert_eq!(run(&mut e, 0x005a_3570, a, 0, 0), (true, 1.0));
        let process_no = object(&mut e, &[(VSLOT_PROCESS_TEST_60, SLOT_NO)]);
        stub(&mut e, ACTOR_GET_PROCESS, process_no);
        assert_eq!(run(&mut e, 0x005a_3570, a, 0, 0), (true, 0.0));
        stub(&mut e, ACTOR_GET_PROCESS, 0);
        assert_eq!(run(&mut e, 0x005a_3570, a, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_3570, plain, 0, 0), (true, 0.0));
    }

    #[test]
    fn a35f0_asks_the_actor_test() {
        let mut e = engine();
        let a = actor(&mut e);
        stub(&mut e, ACTOR_TEST_87F510, 1);
        assert_eq!(run(&mut e, 0x005a_35f0, a, 0, 0), (true, 1.0));
        stub(&mut e, ACTOR_TEST_87F510, 0);
        assert_eq!(run(&mut e, 0x005a_35f0, a, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        stub(&mut e, ACTOR_TEST_87F510, 1);
        assert_eq!(run(&mut e, 0x005a_35f0, plain, 0, 0), (true, 0.0));
    }

    #[test]
    fn a3650_returns_the_int_global() {
        let mut e = engine();
        e.set_global(INT_GLOBAL_5A3650, (-5i32) as u32);
        assert_eq!(run(&mut e, 0x005a_3650, 0, 0, 0), (true, -5.0));
        e.set_global(INT_GLOBAL_5A3650, 12u32);
        assert_eq!(run(&mut e, 0x005a_3650, 0, 0, 0), (true, 12.0));
    }

    #[test]
    fn a3670_is_always_zero() {
        let mut e = engine();
        let a = actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_3670, a, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x005a_3670, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn dangerous_water_needs_the_flag_the_cell_and_a_dangerous_water_type() {
        let mut e = engine();
        let a = actor(&mut e);
        e.mem.set_u8(a + 0x14c, 1);
        stub(&mut e, REF_GET_PARENT_CELL, 0x6000);
        e.register_double(CELL_GET_WATER_TYPE, |_, args| {
            assert_eq!(args, [0x6000]);
            0x6100u32.into_ret()
        });
        e.register_double(WATER_FORM_GET_DANGEROUS, |_, args| {
            assert_eq!(args, [0x6100]);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_36b0, a, 0, 0), (true, 1.0));
        stub(&mut e, WATER_FORM_GET_DANGEROUS, 0);
        assert_eq!(run(&mut e, 0x005a_36b0, a, 0, 0), (true, 0.0));
        stub(&mut e, WATER_FORM_GET_DANGEROUS, 1);
        stub(&mut e, CELL_GET_WATER_TYPE, 0);
        assert_eq!(run(&mut e, 0x005a_36b0, a, 0, 0), (true, 0.0));
        stub(&mut e, CELL_GET_WATER_TYPE, 0x6100);
        stub(&mut e, REF_GET_PARENT_CELL, 0);
        assert_eq!(run(&mut e, 0x005a_36b0, a, 0, 0), (true, 0.0));
        stub(&mut e, REF_GET_PARENT_CELL, 0x6000);
        e.mem.set_u8(a + 0x14c, 0);
        assert_eq!(run(&mut e, 0x005a_36b0, a, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        e.mem.set_u8(plain + 0x14c, 1);
        assert_eq!(run(&mut e, 0x005a_36b0, plain, 0, 0), (true, 0.0));
    }

    #[test]
    fn a3740_reads_the_byte_at_14c() {
        let mut e = engine();
        let block = e.mem.alloc(0x200);
        e.mem.set_u8(block + 0x14c, 0x7f);
        assert_eq!(e.call(0x005a_3740, &args![block]).u32() & 0xff, 0x7f);
    }

    #[test]
    fn a3760_tests_the_reference_flag() {
        let mut e = engine();
        let block = e.mem.alloc(0x20);
        assert_eq!(run(&mut e, 0x005a_3760, block, 0, 0), (true, 0.0));
        e.mem.set_u32(block + 8, 0x0010_0000);
        assert_eq!(run(&mut e, 0x005a_3760, block, 0, 0), (true, 1.0));
    }

    #[test]
    fn a3790_tests_bit_100000_of_the_flags() {
        let mut e = engine();
        let block = e.mem.alloc(0x20);
        e.mem.set_u32(block + 8, 0xffef_ffff);
        assert!(!e.call(0x005a_3790, &args![block]).bool());
        e.mem.set_u32(block + 8, 0x0010_0000);
        assert!(e.call(0x005a_3790, &args![block]).bool());
    }

    #[test]
    fn xbox_follows_the_platform_test() {
        let mut e = engine();
        stub(&mut e, PLATFORM_IS_CONSOLE, 1);
        assert_eq!(run(&mut e, 0x005a_37b0, 0, 0, 0), (true, 1.0));
        stub(&mut e, PLATFORM_IS_CONSOLE, 0);
        assert_eq!(run(&mut e, 0x005a_37b0, 0, 0, 0), (true, 0.0));
        let lines = printed(&mut e, 0.0, 0x005a_37b0, 0, 0, 0);
        let w = words(0.0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5e04, w[0], w[1]])]);
    }

    #[test]
    fn ps3_is_always_zero() {
        let mut e = engine();
        assert_eq!(run(&mut e, 0x005a_3820, 0, 0, 0), (true, 0.0));
        let lines = printed(&mut e, 1.0, 0x005a_3820, 0, 0, 0);
        let w = words(0.0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5e14, w[0], w[1]])]);
    }

    #[test]
    fn pc_is_the_opposite_of_the_platform_test() {
        let mut e = engine();
        stub(&mut e, PLATFORM_IS_CONSOLE, 1);
        assert_eq!(run(&mut e, 0x005a_3870, 0, 0, 0), (true, 0.0));
        stub(&mut e, PLATFORM_IS_CONSOLE, 0);
        assert_eq!(run(&mut e, 0x005a_3870, 0, 0, 0), (true, 1.0));
        let lines = printed(&mut e, 0.0, 0x005a_3870, 0, 0, 0);
        let w = words(1.0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5e24, w[0], w[1]])]);
    }

    #[test]
    fn a38e0_clears_the_caches() {
        let mut e = engine();
        for address in CACHE_WORDS {
            e.set_global(address, 0xdead_beefu32);
        }
        e.call(0x005a_38e0, &args![]);
        for address in CACHE_WORDS {
            assert_eq!(e.global::<u32>(address), 0);
        }
    }

    #[test]
    fn essential_asks_the_actor_without_a_null_check() {
        let mut e = engine();
        let a = actor(&mut e);
        stub(&mut e, ACTOR_GET_ESSENTIAL, 1);
        assert_eq!(run(&mut e, 0x005a_3980, a, 0, 0), (true, 1.0));
        stub(&mut e, ACTOR_GET_ESSENTIAL, 0);
        assert_eq!(run(&mut e, 0x005a_3980, a, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        stub(&mut e, ACTOR_GET_ESSENTIAL, 1);
        assert_eq!(run(&mut e, 0x005a_3980, plain, 0, 0), (true, 0.0));
    }

    #[test]
    fn a39d0_is_the_actor_test() {
        let mut e = engine();
        let a = actor(&mut e);
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_39d0, a, 0, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x005a_39d0, plain, 0, 0), (true, 0.0));
    }

    #[test]
    fn a3a00_reads_the_player_byte_at_20c() {
        let mut e = engine();
        let player = e.mem.alloc(0x300);
        e.set_global(PLAYER, player);
        assert_eq!(run(&mut e, 0x005a_3a00, 0, 0, 0), (true, 0.0));
        e.mem.set_u8(player + 0x20c, 1);
        assert_eq!(run(&mut e, 0x005a_3a00, 0, 0, 0), (true, 1.0));
    }

    #[test]
    fn a3a30_subtracts_the_process_float_from_the_calendar_value() {
        let mut e = engine();
        let a = object(
            &mut e,
            &[
                (VSLOT_IS_ACTOR, IS_ACTOR_YES),
                (VSLOT_ACTOR_TEST_22C, SLOT_YES),
            ],
        );
        let process = object(&mut e, &[(VSLOT_PROCESS_FLOAT_69C, 0x0f00_0010)]);
        stub_st0(&mut e, 0x0f00_0010, 30.5);
        stub(&mut e, ACTOR_GET_PROCESS, process);
        e.register_double(CALENDAR_GET_WHOLE_VALUE, |_, args| {
            assert_eq!(args, [CALENDAR]);
            100u32.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_3a30, a, 0, 0), (true, 69.5));
        // A process float that is not positive: 0.0.
        stub_st0(&mut e, 0x0f00_0010, 0.0);
        assert_eq!(run(&mut e, 0x005a_3a30, a, 0, 0), (true, 0.0));
        stub_st0(&mut e, 0x0f00_0010, 30.5);
        // No process.
        stub(&mut e, ACTOR_GET_PROCESS, 0);
        assert_eq!(run(&mut e, 0x005a_3a30, a, 0, 0), (true, 0.0));
        stub(&mut e, ACTOR_GET_PROCESS, process);
        // The `+0x22c` test says no.
        let b = object(
            &mut e,
            &[
                (VSLOT_IS_ACTOR, IS_ACTOR_YES),
                (VSLOT_ACTOR_TEST_22C, SLOT_NO),
            ],
        );
        assert_eq!(run(&mut e, 0x005a_3a30, b, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x005a_3a30, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn a3af0_returns_the_players_value() {
        let mut e = engine();
        e.set_global(PLAYER, 0x4400u32);
        e.register_double(PLAYER_GET_VALUE_964060, |_, args| {
            assert_eq!(args, [0x4400]);
            (-3i32 as u32).into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_3af0, 0, 0, 0), (true, -3.0));
    }

    #[test]
    fn player_action_active_passes_the_action_and_masks_the_byte() {
        let mut e = engine();
        e.set_global(PLAYER, 0x4400u32);
        e.register_double(PLAYER_IS_ACTION_ACTIVE, |_, args| {
            assert_eq!(args, [0x4400, 9]);
            0x101u32.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_3b20, 0, 9, 0), (true, 1.0));
        stub(&mut e, PLAYER_IS_ACTION_ACTIVE, 0x100);
        assert_eq!(run(&mut e, 0x005a_3b20, 0, 9, 0), (true, 0.0));
        let lines = printed(&mut e, 0.0, 0x005a_3b20, 0, 9, 0);
        let w = words(0.0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5e34, w[0], w[1]])]);
    }

    #[test]
    fn b90_compares_the_form_field_when_the_base_form_has_type_16() {
        let mut e = engine();
        stub(&mut e, REF_GET_BASE_FORM, 0x5000);
        stub(&mut e, FORM_GET_TYPE, 0x16);
        e.register_double(FORM_GET_FIELD_90, |_, args| {
            assert_eq!(args, [0x5000]);
            77u32.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_3b90, 0x2000, 77, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x005a_3b90, 0x2000, 78, 0), (true, 0.0));
        stub(&mut e, FORM_GET_TYPE, 0x17);
        assert_eq!(run(&mut e, 0x005a_3b90, 0x2000, 77, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x005a_3b90, 0, 77, 0), (true, 0.0));
    }

    #[test]
    fn bf0_asks_the_player_with_the_parameter() {
        let mut e = engine();
        e.set_global(PLAYER, 0x4400u32);
        e.register_double(PLAYER_TEST_966C10, |_, args| {
            assert_eq!(args, [0x4400, 0x55]);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_3bf0, 0, 0x55, 0), (true, 1.0));
        stub(&mut e, PLAYER_TEST_966C10, 0);
        assert_eq!(run(&mut e, 0x005a_3bf0, 0, 0x55, 0), (true, 0.0));
    }

    #[test]
    fn c30_reads_the_part_value_and_defaults_to_minus_one() {
        let mut e = engine();
        let a = actor(&mut e);
        let part = e.mem.alloc(0x20);
        e.mem.set_u32(part + 0x10, (-4i32) as u32);
        stub(&mut e, ACTOR_GET_PROCESS_PART, part);
        assert_eq!(run(&mut e, 0x005a_3c30, a, 0, 0), (true, -4.0));
        e.mem.set_u32(part + 0x10, 9);
        assert_eq!(run(&mut e, 0x005a_3c30, a, 0, 0), (true, 9.0));
        stub(&mut e, ACTOR_GET_PROCESS_PART, 0);
        assert_eq!(run(&mut e, 0x005a_3c30, a, 0, 0), (true, -1.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_3c30, plain, 0, 0), (true, -1.0));
        assert_eq!(run(&mut e, 0x005a_3c30, 0, 0, 0), (true, -1.0));
    }

    #[test]
    fn c90_tests_mask_4_on_the_part() {
        let mut e = engine();
        let a = actor(&mut e);
        stub(&mut e, ACTOR_GET_PROCESS_PART, 0x6600);
        e.register_double(PROCESS_PART_TEST_FLAGS, |_, args| {
            assert_eq!(args, [0x6600, 4]);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_3c90, a, 0, 0), (true, 1.0));
        stub(&mut e, PROCESS_PART_TEST_FLAGS, 0);
        assert_eq!(run(&mut e, 0x005a_3c90, a, 0, 0), (true, 0.0));
        stub(&mut e, ACTOR_GET_PROCESS_PART, 0);
        assert_eq!(run(&mut e, 0x005a_3c90, a, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_3c90, plain, 0, 0), (true, 0.0));
    }

    // -----------------------------------------------------------------
    // Second session: `005a3d00` to `005a4f30`.
    // -----------------------------------------------------------------

    /// Virtual functions of the second session's tests: a name each, and the
    /// words the other slots return.
    const NAME_OF_REF: u32 = 0x0f00_0020;
    const NAME_OF_OTHER: u32 = 0x0f00_0021;
    const SLOT_VALUE: u32 = 0x0f00_0022;

    fn set_player(e: &mut Engine, player: u32) {
        e.set_global(PLAYER, player);
    }

    /// The `(format, words of the result)` debug line.
    fn line_with_result(format: u32, value: f64) -> (u32, Vec<u32>) {
        let w = words(value);
        (DEBUG_PRINT, vec![format, w[0], w[1]])
    }

    /// An actor with a name function, for the lines that print names.
    fn named(e: &mut Engine, slots: &[(u32, u32)], name: u32) -> u32 {
        stub(e, NAME_OF_REF, name);
        let mut all = vec![
            (VSLOT_IS_ACTOR, IS_ACTOR_YES),
            (VSLOT_FORM_GET_NAME, NAME_OF_REF),
        ];
        all.extend_from_slice(slots);
        object(e, &all)
    }

    #[test]
    fn a3d00_is_one_when_the_player_test_says_no() {
        let mut e = engine();
        set_player(&mut e, 0x7000);
        e.register_double(PLAYER_TEST_4EAF60, |_, a| {
            assert_eq!(a, [0x7000]);
            false.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_3d00, 0, 0, 0), (true, 1.0));
        stub(&mut e, PLAYER_TEST_4EAF60, 1);
        assert_eq!(run(&mut e, 0x005a_3d00, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn a3d30_cause_of_death_defaults_to_minus_one_and_does_not_check_null() {
        let mut e = engine();
        let a = actor(&mut e);
        stub(&mut e, REF_GET_CAUSE_OF_DEATH, 5);
        assert_eq!(run(&mut e, 0x005a_3d30, a, 0, 0), (true, 5.0));
        stub(&mut e, REF_GET_CAUSE_OF_DEATH, (-3i32) as u32);
        assert_eq!(run(&mut e, 0x005a_3d30, a, 0, 0), (true, -3.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_3d30, plain, 0, 0), (true, -1.0));
        // The line is printed for non-actors too.
        let lines = printed(&mut e, 0.0, 0x005a_3d30, plain, 0, 0);
        assert_eq!(lines, vec![line_with_result(0x0103_5e4c, -1.0)]);
    }

    #[test]
    fn a3db0_limb_gone_passes_the_limb_to_get_dismembered() {
        let mut e = engine();
        let a = actor(&mut e);
        e.register_double(REF_GET_DISMEMBERED, move |_, args| {
            assert_eq!(args, [a, 6]);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_3db0, a, 6, 0), (true, 1.0));
        stub(&mut e, REF_GET_DISMEMBERED, 0);
        assert_eq!(run(&mut e, 0x005a_3db0, a, 6, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_3db0, plain, 6, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x005a_3db0, 0, 6, 0), (true, 0.0));
    }

    #[test]
    fn a3e10_killing_blow_limb_is_minus_one_for_non_actors() {
        let mut e = engine();
        let a = actor(&mut e);
        stub(&mut e, REF_GET_LAST_HIT_LIMB, 4);
        assert_eq!(run(&mut e, 0x005a_3e10, a, 0, 0), (true, 4.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_3e10, plain, 0, 0), (true, -1.0));
        let lines = printed(&mut e, 0.0, 0x005a_3e10, a, 0, 0);
        assert_eq!(lines, vec![line_with_result(0x0103_5e68, 4.0)]);
    }

    #[test]
    fn a3e90_compares_the_virtual_value_with_the_setting_signed() {
        let mut e = engine();
        let player = e.mem.alloc(0x10);
        set_player(&mut e, player);
        let setting = e.mem.alloc(4);
        e.register_double(SETTING_GET_INT_ADDRESS, move |_, a| {
            assert_eq!(a, [SETTING_OBJECT]);
            setting.into_ret()
        });
        e.mem.set_u32(setting, 10);
        e.register_double(SLOT_VALUE, move |_, a| {
            assert_eq!(&a[1..], [player, 0]);
            10u32.into_ret()
        });
        let a = object(
            &mut e,
            &[
                (VSLOT_IS_ACTOR, IS_ACTOR_YES),
                (VSLOT_ACTOR_INT_344, SLOT_VALUE),
            ],
        );
        assert_eq!(run(&mut e, 0x005a_3e90, a, 0, 0), (true, 1.0));
        e.mem.set_u32(setting, 11);
        assert_eq!(run(&mut e, 0x005a_3e90, a, 0, 0), (true, 0.0));
        // Signed: -5 is below 3.
        e.mem.set_u32(setting, 3);
        stub(&mut e, SLOT_VALUE, (-5i32) as u32);
        assert_eq!(run(&mut e, 0x005a_3e90, a, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_3e90, plain, 0, 0), (true, 0.0));
    }

    #[test]
    fn a3f00_killer_compares_param1_with_the_word_at_c0() {
        let mut e = engine();
        let a = object(
            &mut e,
            &[
                (VSLOT_IS_ACTOR, IS_ACTOR_YES),
                (VSLOT_ACTOR_TEST_22C, SLOT_YES),
            ],
        );
        e.mem.set_u32(a + 0xc0, 0x4242);
        assert_eq!(run(&mut e, 0x005a_3f00, a, 0x4242, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x005a_3f00, a, 0x4243, 0), (true, 0.0));
        let lines = printed(&mut e, 0.0, 0x005a_3f00, a, 0x4242, 0);
        assert_eq!(lines, vec![line_with_result(0x0103_5e84, 1.0)]);
        let no = object(
            &mut e,
            &[
                (VSLOT_IS_ACTOR, IS_ACTOR_YES),
                (VSLOT_ACTOR_TEST_22C, SLOT_NO),
            ],
        );
        assert_eq!(run(&mut e, 0x005a_3f00, no, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_3f00, plain, 0, 0), (true, 0.0));
        assert!(printed(&mut e, 0.0, 0x005a_3f00, plain, 0, 0).is_empty());
    }

    #[test]
    fn a3fa0_killer_object_walks_the_dismemberment_extra() {
        let mut e = engine();
        let a = object(
            &mut e,
            &[
                (VSLOT_IS_ACTOR, IS_ACTOR_YES),
                (VSLOT_ACTOR_TEST_22C, SLOT_YES),
            ],
        );
        stub(&mut e, ACTOR_GET_PROCESS, 0x6600);
        stub(&mut e, REF_GET_EXTRA_DATA_LIST, 0x6700);
        e.register_double(EXTRA_LIST_GET_DISMEMBERMENT_EXTRA, |_, a| {
            assert_eq!(a, [0x6700]);
            0x6800u32.into_ret()
        });
        e.register_double(DISMEMBERMENT_GET_LIST_NODE, |_, a| {
            assert_eq!(a, [0x6800]);
            0x6900u32.into_ret()
        });
        e.register_double(FORM_LIST_GET_LIST, |_, a| {
            assert_eq!(a, [0x5000]);
            0x6a00u32.into_ret()
        });
        e.register_double(NODE_TEST_5F65D0, |e, a| {
            assert_eq!(a[0], 0x6a00);
            assert_eq!(e.mem.u32(a[1]), 0x6900);
            true.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_3fa0, a, 0x5000, 0), (true, 1.0));
        // Nothing clears the result: a caller's value stays.
        stub(&mut e, NODE_TEST_5F65D0, 0);
        assert_eq!(run(&mut e, 0x005a_3fa0, a, 0x5000, 0), (true, SENTINEL));
        assert_eq!(run(&mut e, 0x005a_3fa0, a, 0, 0), (true, SENTINEL));
        stub(&mut e, DISMEMBERMENT_GET_LIST_NODE, 0);
        assert_eq!(run(&mut e, 0x005a_3fa0, a, 0x5000, 0), (true, SENTINEL));
        stub(&mut e, ACTOR_GET_PROCESS, 0);
        assert_eq!(run(&mut e, 0x005a_3fa0, a, 0x5000, 0), (true, SENTINEL));
        // The line is printed for every reference.
        let lines = printed(&mut e, 0.0, 0x005a_3fa0, 0, 0x5000, 0);
        assert_eq!(lines, vec![line_with_result(0x0103_5e98, 0.0)]);
    }

    #[test]
    fn a4090_needs_both_params_and_leaves_the_result_otherwise() {
        let mut e = engine();
        e.register_double(0x0048_c1b0, |_, a| {
            assert_eq!(a, [0x5024, 0x77]);
            9u32.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_4090, 1, 0x5000, 0x77), (true, 9.0));
        assert_eq!(run(&mut e, 0x005a_4090, 1, 0, 0x77), (true, SENTINEL));
        assert_eq!(run(&mut e, 0x005a_4090, 1, 0x5000, 0), (true, SENTINEL));
    }

    #[test]
    fn a40d0_reports_the_lowest_set_flag() {
        let mut e = engine();
        e.set_global(DOUBLE_THREE, 3.0f64);
        e.set_global(DOUBLE_FOUR, 4.0f64);
        let a = actor(&mut e);
        for (flags, expected) in [
            (0u32, 0.0),
            (1, 1.0),
            (0xf, 1.0),
            (2, 2.0),
            (6, 2.0),
            (4, 3.0),
            (0xc, 3.0),
            (8, 4.0),
            (0x10, 0.0),
        ] {
            stub(&mut e, ACTOR_GET_FLAGS, flags);
            assert_eq!(
                run(&mut e, 0x005a_40d0, a, 0, 0),
                (true, expected),
                "{flags:#x}"
            );
        }
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_40d0, plain, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x005a_40d0, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn a4160_reports_bit_10_then_bit_20() {
        let mut e = engine();
        let a = actor(&mut e);
        for (flags, expected) in [
            (0u32, 0.0),
            (0x10, 1.0),
            (0x30, 1.0),
            (0x20, 2.0),
            (0x40, 0.0),
        ] {
            stub(&mut e, ACTOR_GET_FLAGS, flags);
            assert_eq!(
                run(&mut e, 0x005a_4160, a, 0, 0),
                (true, expected),
                "{flags:#x}"
            );
        }
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_4160, plain, 0, 0), (true, 0.0));
    }

    #[test]
    fn a41c0_anim_action_defaults_to_minus_one() {
        let mut e = engine();
        let a = actor(&mut e);
        stub(&mut e, ACTOR_GET_ANIM_ACTION, 12);
        assert_eq!(run(&mut e, 0x005a_41c0, a, 0, 0), (true, 12.0));
        stub(&mut e, ACTOR_GET_ANIM_ACTION, (-1i32) as u32);
        assert_eq!(run(&mut e, 0x005a_41c0, a, 0, 0), (true, -1.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_41c0, plain, 0, 0), (true, -1.0));
        assert_eq!(run(&mut e, 0x005a_41c0, 0, 0, 0), (true, -1.0));
    }

    #[test]
    fn a4210_is_zero_for_null_or_the_same_reference() {
        let mut e = engine();
        assert_eq!(run(&mut e, 0x005a_4210, 0, 5, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x005a_4210, 5, 5, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x005a_4210, 5, 6, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x005a_4210, 5, 0, 0), (true, 1.0));
    }

    #[test]
    fn a4240_counts_the_object_of_virtual_3f8() {
        let mut e = engine();
        let a = object(
            &mut e,
            &[
                (VSLOT_IS_ACTOR, IS_ACTOR_YES),
                (VSLOT_REF_COUNTED_OBJECT, SLOT_VALUE),
            ],
        );
        stub(&mut e, SLOT_VALUE, 0x8000);
        e.register_double(OBJECT_GET_COUNT, |_, a| {
            assert_eq!(a, [0x8000]);
            0xffff_fff0u32.into_ret()
        });
        // Unsigned count.
        assert_eq!(run(&mut e, 0x005a_4240, a, 0, 0), (true, 4294967280.0));
        stub(&mut e, SLOT_VALUE, 0);
        assert_eq!(run(&mut e, 0x005a_4240, a, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_4240, plain, 0, 0), (true, 0.0));
    }

    #[test]
    fn a42b0_and_a4320_read_the_word_at_plus_0x10() {
        let mut e = engine();
        let a = object(
            &mut e,
            &[
                (VSLOT_IS_ACTOR, IS_ACTOR_YES),
                (VSLOT_REF_COUNTED_OBJECT, SLOT_VALUE),
            ],
        );
        let object_address = 0x8000;
        stub(&mut e, SLOT_VALUE, object_address);
        e.register_double(OBJECT_GET_FIELD_08, |_, a| {
            assert_eq!(a, [0x8008]);
            0x8000_0001u32.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_42b0, a, 0, 0), (true, 2147483649.0));
        assert_eq!(
            e.call(0x005a_4320, &args![object_address]).u32(),
            0x8000_0001
        );
        stub(&mut e, SLOT_VALUE, 0);
        assert_eq!(run(&mut e, 0x005a_42b0, a, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_42b0, plain, 0, 0), (true, 0.0));
    }

    #[test]
    fn a4340_returns_the_process_byte() {
        let mut e = engine();
        let a = actor(&mut e);
        let process = object(&mut e, &[(VSLOT_PROCESS_BYTE_388, SLOT_VALUE)]);
        stub(&mut e, ACTOR_GET_PROCESS, process);
        // Only the low byte counts.
        stub(&mut e, SLOT_VALUE, 0x1_0207);
        assert_eq!(run(&mut e, 0x005a_4340, a, 0, 0), (true, 7.0));
        stub(&mut e, ACTOR_GET_PROCESS, 0);
        assert_eq!(run(&mut e, 0x005a_4340, a, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_4340, plain, 0, 0), (true, 0.0));
    }

    #[test]
    fn a43b0_asks_the_process_lists() {
        let mut e = engine();
        let a = actor(&mut e);
        e.register_double(PROCESS_LISTS_TEST_REFERENCE, move |_, args| {
            assert_eq!(args, [PROCESS_LISTS, a]);
            0x101u32.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_43b0, a, 0, 0), (true, 1.0));
        stub(&mut e, PROCESS_LISTS_TEST_REFERENCE, 0);
        assert_eq!(run(&mut e, 0x005a_43b0, a, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_43b0, plain, 0, 0), (true, 0.0));
    }

    #[test]
    fn a4410_compares_the_base_form_type() {
        let mut e = engine();
        stub(&mut e, REF_GET_BASE_FORM, 0x5000);
        e.register_double(FORM_GET_TYPE, |_, a| {
            assert_eq!(a, [0x5000]);
            0x2au32.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_4410, 0x1000, 0x2a, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x005a_4410, 0x1000, 0x2b, 0), (true, 0.0));
        let lines = printed(&mut e, 0.0, 0x005a_4410, 0x1000, 0x2a, 0);
        assert_eq!(lines, vec![line_with_result(0x0103_5eb0, 1.0)]);
    }

    #[test]
    fn a4480_and_a4520_emotion_is_minus_one_without_the_flag_byte() {
        let mut e = engine();
        let r = object(
            &mut e,
            &[
                (VSLOT_IS_MOBILE_OBJECT, SLOT_YES),
                (VSLOT_EMOTION, SLOT_VALUE),
            ],
        );
        stub(&mut e, SLOT_VALUE, 3);
        e.mem.set_u8(r + 0x86, 1);
        assert_eq!(e.call(0x005a_4520, &args![r]).u32() & 0xff, 1);
        assert_eq!(run(&mut e, 0x005a_4480, r, 0, 0), (true, 3.0));
        e.mem.set_u8(r + 0x86, 0);
        assert_eq!(run(&mut e, 0x005a_4480, r, 0, 0), (true, -1.0));
        let lines = printed(&mut e, 0.0, 0x005a_4480, r, 0, 0);
        assert_eq!(lines, vec![line_with_result(0x0103_5ed0, -1.0)]);
        let off = object(&mut e, &[(VSLOT_IS_MOBILE_OBJECT, SLOT_NO)]);
        assert_eq!(run(&mut e, 0x005a_4480, off, 0, 0), (true, 0.0));
    }

    #[test]
    fn a4540_calls_virtual_2d8_and_ignores_it() {
        let mut e = engine();
        let r = object(
            &mut e,
            &[
                (VSLOT_IS_MOBILE_OBJECT, SLOT_YES),
                (VSLOT_EMOTION_VALUE, SLOT_VALUE),
            ],
        );
        stub(&mut e, SLOT_VALUE, 99);
        e.call_log = Some(vec![]);
        assert_eq!(run(&mut e, 0x005a_4540, r, 0, 0), (true, 0.0));
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, SLOT_VALUE).len(), 1);
        let off = object(&mut e, &[(VSLOT_IS_MOBILE_OBJECT, SLOT_NO)]);
        e.call_log = Some(vec![]);
        assert_eq!(run(&mut e, 0x005a_4540, off, 0, 0), (true, 0.0));
        let log = take_log(&mut e);
        assert!(calls_to(&log, SLOT_VALUE).is_empty());
        let lines = printed(&mut e, 0.0, 0x005a_4540, off, 0, 0);
        assert_eq!(lines, vec![line_with_result(0x0103_5ee4, 0.0)]);
    }

    #[test]
    fn a45c0_water_object_tests_the_used_item() {
        let mut e = engine();
        let item = object(&mut e, &[(VSLOT_ITEM_WATER_TEST, SLOT_VALUE)]);
        stub(&mut e, IDLE_MANAGER_GET_USED_ITEM, item);
        stub(&mut e, SLOT_VALUE, 5);
        assert_eq!(run(&mut e, 0x005a_45c0, 0, 0, 0), (true, 1.0));
        stub(&mut e, SLOT_VALUE, 0);
        assert_eq!(run(&mut e, 0x005a_45c0, 0, 0, 0), (true, 0.0));
        stub(&mut e, IDLE_MANAGER_GET_USED_ITEM, 0);
        let lines = printed(&mut e, 0.0, 0x005a_45c0, 0, 0, 0);
        assert_eq!(lines, vec![line_with_result(0x0103_5f00, 0.0)]);
    }

    #[test]
    fn a4630_has_perk_passes_the_perk_and_the_flag() {
        let mut e = engine();
        e.register_double(SLOT_VALUE, |_, a| {
            assert_eq!(&a[1..], [0x4444, 1]);
            0x103u32.into_ret()
        });
        let a = object(
            &mut e,
            &[
                (VSLOT_IS_ACTOR, IS_ACTOR_YES),
                (VSLOT_ACTOR_PERK_RANK, SLOT_VALUE),
            ],
        );
        assert_eq!(run(&mut e, 0x005a_4630, a, 0x4444, 0x99), (true, 1.0));
        // The rank byte is the low byte: 0x100 is rank 0.
        stub(&mut e, SLOT_VALUE, 0x100);
        assert_eq!(run(&mut e, 0x005a_4630, a, 0x4444, 0), (true, 0.0));
        stub(&mut e, SLOT_VALUE, 2);
        let lines = printed(&mut e, 0.0, 0x005a_4630, a, 0x4444, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5f1c, 2])]);
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_4630, plain, 0x4444, 0), (true, 0.0));
    }

    #[test]
    fn a46d0_faction_relation_passes_the_other_actor_and_an_out_byte() {
        let mut e = engine();
        let a = actor(&mut e);
        e.register_double(ACTOR_GET_FACTION_FIGHT_REACTION, move |e, args| {
            assert_eq!((args[0], args[1]), (a, 0x5000));
            assert_eq!(e.mem.u8(args[2]), 0);
            (-2i32 as u32).into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_46d0, a, 0x5000, 0), (true, -2.0));
        let lines = printed(&mut e, 0.0, 0x005a_46d0, a, 0x5000, 0);
        assert_eq!(
            lines,
            vec![(DEBUG_PRINT, vec![0x0103_5f2c, (-2i32) as u32])]
        );
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_46d0, plain, 0x5000, 0), (true, 0.0));
    }

    #[test]
    fn a4780_last_played_idle_compares_with_param1() {
        let mut e = engine();
        let a = actor(&mut e);
        let process = object(&mut e, &[(VSLOT_PROCESS_LAST_IDLE, SLOT_VALUE)]);
        stub(&mut e, ACTOR_GET_PROCESS, process);
        stub(&mut e, SLOT_VALUE, 0x77);
        assert_eq!(run(&mut e, 0x005a_4780, a, 0x77, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x005a_4780, a, 0x78, 0), (true, 0.0));
        let lines = printed(&mut e, 0.0, 0x005a_4780, a, 0x77, 0);
        assert_eq!(lines, vec![line_with_result(0x0103_5f44, 1.0)]);
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_4780, plain, 0x77, 0), (true, 0.0));
        assert!(printed(&mut e, 0.0, 0x005a_4780, plain, 0x77, 0).is_empty());
    }

    #[test]
    fn a4820_player_teammate_prints_the_truncated_result() {
        let mut e = engine();
        let a = actor(&mut e);
        stub(&mut e, MIDDLE_HIGH_GET_FORCE_NEXT_UPDATE, 1);
        assert_eq!(run(&mut e, 0x005a_4820, a, 0, 0), (true, 1.0));
        let lines = printed(&mut e, 0.0, 0x005a_4820, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5f64, 1])]);
        stub(&mut e, MIDDLE_HIGH_GET_FORCE_NEXT_UPDATE, 0);
        assert_eq!(run(&mut e, 0x005a_4820, a, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_4820, plain, 0, 0), (true, 0.0));
    }

    #[test]
    fn a48a0_and_a4910_teammate_count_is_the_word_at_d68() {
        let mut e = engine();
        let player = e.mem.alloc(0xe00);
        set_player(&mut e, player);
        e.mem.set_u32(player + 0xd68, 3);
        assert_eq!(e.call(0x005a_4910, &args![player]).u32(), 3);
        assert_eq!(run(&mut e, 0x005a_48a0, 0, 0, 0), (true, 3.0));
        e.mem.set_u32(player + 0xd68, 0xffff_ffff);
        assert_eq!(run(&mut e, 0x005a_48a0, 0, 0, 0), (true, 4294967295.0));
        e.mem.set_u32(player + 0xd68, 3);
        trace_on(&mut e);
        run(&mut e, 0x005a_48a0, 0, 0, 0);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, DEBUG_PRINT), vec![vec![0x0103_5f7c, 3]]);
    }

    #[test]
    fn a4930_crime_enemy_needs_a_non_empty_crime_list() {
        let mut e = engine();
        let a = actor(&mut e);
        stub(&mut e, REF_GET_EXTRA_DATA_LIST, 0x6700);
        stub(&mut e, EXTRA_LIST_GET_PLAYER_CRIME_LIST, 0x6800);
        e.register_double(LIST_NODE_IS_EMPTY, |_, a| {
            assert_eq!(a, [0x6800]);
            false.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_4930, a, 0, 0), (true, 1.0));
        let lines = printed(&mut e, 0.0, 0x005a_4930, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5fd4])]);
        stub(&mut e, LIST_NODE_IS_EMPTY, 1);
        assert_eq!(run(&mut e, 0x005a_4930, a, 0, 0), (true, 0.0));
        let lines = printed(&mut e, 0.0, 0x005a_4930, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_5f9c])]);
        stub(&mut e, EXTRA_LIST_GET_PLAYER_CRIME_LIST, 0);
        assert_eq!(run(&mut e, 0x005a_4930, a, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_4930, plain, 0, 0), (true, 0.0));
        assert!(printed(&mut e, 0.0, 0x005a_4930, plain, 0, 0).is_empty());
    }

    #[test]
    fn a49f0_faction_enemy_asks_about_the_player() {
        let mut e = engine();
        let player = e.mem.alloc(0x10);
        set_player(&mut e, player);
        let a = actor(&mut e);
        e.register_double(ACTOR_GET_FACTION_FIGHT_REACTION, move |e, args| {
            assert_eq!((args[0], args[1]), (a, player));
            e.mem.set_u8(args[2], 1);
            Ret::default()
        });
        assert_eq!(run(&mut e, 0x005a_49f0, a, 0, 0), (true, 1.0));
        let lines = printed(&mut e, 0.0, 0x005a_49f0, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_604c])]);
        stub(&mut e, ACTOR_GET_FACTION_FIGHT_REACTION, 0);
        assert_eq!(run(&mut e, 0x005a_49f0, a, 0, 0), (true, 0.0));
        let lines = printed(&mut e, 0.0, 0x005a_49f0, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_6008])]);
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_49f0, plain, 0, 0), (true, 0.0));
    }

    #[test]
    fn a4aa0_and_a4b40_minor_crimes_split_player_and_others() {
        let mut e = engine();
        let player = object(&mut e, &[(VSLOT_IS_ACTOR, IS_ACTOR_YES)]);
        set_player(&mut e, player);
        e.mem.set_u32(player + 0x13c, 7);
        assert_eq!(e.call(0x005a_4b40, &args![player]).u32(), 7);
        assert_eq!(run(&mut e, 0x005a_4aa0, player, 0, 0), (true, 7.0));
        let other = actor(&mut e);
        stub(&mut e, ACTOR_GET_MINOR_CRIME_COUNT, 2);
        assert_eq!(run(&mut e, 0x005a_4aa0, other, 0, 0), (true, 2.0));
        let lines = printed(&mut e, 0.0, 0x005a_4aa0, other, 0, 0);
        assert_eq!(lines, vec![line_with_result(0x0103_6088, 2.0)]);
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_4aa0, plain, 0, 0), (true, 0.0));
    }

    #[test]
    fn a4b60_and_a4c00_major_crimes_mask_the_top_bits_for_the_player() {
        let mut e = engine();
        let player = object(&mut e, &[(VSLOT_IS_ACTOR, IS_ACTOR_YES)]);
        set_player(&mut e, player);
        e.mem.set_u32(player + 0x140, 0xc000_0005);
        assert_eq!(e.call(0x005a_4c00, &args![player]).u32(), 5);
        assert_eq!(run(&mut e, 0x005a_4b60, player, 0, 0), (true, 5.0));
        let other = actor(&mut e);
        stub(&mut e, ACTOR_GET_MAJOR_CRIME_COUNT, 9);
        assert_eq!(run(&mut e, 0x005a_4b60, other, 0, 0), (true, 9.0));
        let lines = printed(&mut e, 0.0, 0x005a_4b60, other, 0, 0);
        assert_eq!(lines, vec![line_with_result(0x0103_60ac, 9.0)]);
        assert_eq!(run(&mut e, 0x005a_4b60, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn a4c20_compares_the_player_value_with_param1() {
        let mut e = engine();
        set_player(&mut e, 0x7000);
        e.register_double(PLAYER_GET_VALUE_89F4E0, |_, a| {
            assert_eq!(a, [0x7000]);
            0x33u32.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_4c20, 0, 0x33, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x005a_4c20, 0, 0x34, 0), (true, 0.0));
        // A null param1 never matches, even a null value.
        stub(&mut e, PLAYER_GET_VALUE_89F4E0, 0);
        assert_eq!(run(&mut e, 0x005a_4c20, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn a4c60_destruction_stage_needs_no_actor() {
        let mut e = engine();
        e.register_double(GET_DESTRUCTION_STAGE, |_, a| {
            assert_eq!(a, [0x1000]);
            2u32.into_ret()
        });
        assert_eq!(run(&mut e, 0x005a_4c60, 0x1000, 0, 0), (true, 2.0));
        assert_eq!(run(&mut e, 0x005a_4c60, 0, 0, 0), (true, -1.0));
        let lines = printed(&mut e, 0.0, 0x005a_4c60, 0x1000, 0, 0);
        assert_eq!(lines, vec![line_with_result(0x0103_60d0, 2.0)]);
    }

    #[test]
    fn a4cd0_threat_ratio_divides_the_two_strengths() {
        let mut e = engine();
        e.set_global(COMBAT_STRENGTH_ARGUMENT, -1.0f32);
        let a = named(&mut e, &[], 0xaaa);
        let b = object(&mut e, &[(VSLOT_FORM_GET_NAME, NAME_OF_OTHER)]);
        stub(&mut e, NAME_OF_OTHER, 0xbbb);
        e.register_double(ACTOR_CALCULATE_COMBAT_STRENGTH, move |_, args| {
            assert_eq!(f32::from_bits(args[1]), -1.0);
            Ret {
                st0: if args[0] == a { 6.0 } else { 4.0 },
                ..Ret::default()
            }
        });
        assert_eq!(run(&mut e, 0x005a_4cd0, a, b, 0), (true, 1.5));
        let lines = printed(&mut e, 0.0, 0x005a_4cd0, a, b, 0);
        let w = words(1.5);
        assert_eq!(
            lines,
            vec![(DEBUG_PRINT, vec![0x0103_6108, 0xaaa, 0xbbb, w[0], w[1]])]
        );
        // No other reference, a non-actor or a null reference: 0.0 and the
        // other line.
        assert_eq!(run(&mut e, 0x005a_4cd0, a, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_4cd0, plain, b, 0), (true, 0.0));
        let lines = printed(&mut e, 0.0, 0x005a_4cd0, 0, b, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_60ec])]);
    }

    #[test]
    fn a4dd0_alignment_compares_the_karma_class_and_always_prints() {
        let mut e = engine();
        let a = object(&mut e, &[(VSLOT_IS_ACTOR, IS_ACTOR_YES)]);
        // The owner embedded at +0xa4 has its own vtable.
        let owner_vtable = e.mem.alloc(0x20);
        e.mem
            .set_u32(owner_vtable + VSLOT_OWNER_GET_VALUE, SLOT_VALUE);
        e.mem.set_u32(a + ACTOR_VALUE_OWNER_OFFSET, owner_vtable);
        e.register_double(SLOT_VALUE, move |_, args| {
            assert_eq!(args, [a + ACTOR_VALUE_OWNER_OFFSET, ACTOR_VALUE_KARMA]);
            Ret {
                st0: 250.5,
                ..Ret::default()
            }
        });
        e.register_double(GET_ALIGNMENT_FOR_KARMA, |_, args| {
            assert_eq!(f32::from_bits(args[0]), 250.5);
            3u32.into_ret()
        });
        e.mem.set_u32(ALIGNMENT_NAMES + 12, 0x9000);
        assert_eq!(run(&mut e, 0x005a_4dd0, a, 3, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x005a_4dd0, a, 2, 0), (true, 0.0));
        let lines = printed(&mut e, 0.0, 0x005a_4dd0, a, 3, 0);
        assert_eq!(
            lines,
            vec![(DEBUG_PRINT, vec![0x0103_6130, 0x9000, TEXT_TRUE])]
        );
        // A non-actor or null reference still prints (false).
        e.mem.set_u32(ALIGNMENT_NAMES + 4, 0x9100);
        let plain = non_actor(&mut e);
        let lines = printed(&mut e, 0.0, 0x005a_4dd0, plain, 1, 0);
        assert_eq!(
            lines,
            vec![(DEBUG_PRINT, vec![0x0103_6130, 0x9100, TEXT_FALSE])]
        );
        assert_eq!(run(&mut e, 0x005a_4dd0, 0, 1, 0), (true, 0.0));
    }

    #[test]
    fn a4ea0_used_item_equip_type() {
        let mut e = engine();
        stub(&mut e, IDLE_MANAGER_GET_USED_ITEM, 0x5000);
        e.register_double(GET_EQUIP_TYPE, |_, a| {
            assert_eq!(a, [0x5000]);
            4u32.into_ret()
        });
        e.mem.set_u32(EQUIP_TYPE_NAMES + 16, 0x9200);
        assert_eq!(run(&mut e, 0x005a_4ea0, 0, 4, 0), (true, 1.0));
        assert_eq!(run(&mut e, 0x005a_4ea0, 0, 5, 0), (true, 0.0));
        let lines = printed(&mut e, 0.0, 0x005a_4ea0, 0, 4, 0);
        assert_eq!(
            lines,
            vec![(DEBUG_PRINT, vec![0x0103_614c, 0x9200, TEXT_TRUE])]
        );
        stub(&mut e, IDLE_MANAGER_GET_USED_ITEM, 0);
        let lines = printed(&mut e, 0.0, 0x005a_4ea0, 0, 4, 0);
        assert_eq!(
            lines,
            vec![(DEBUG_PRINT, vec![0x0103_614c, 0x9200, TEXT_FALSE])]
        );
    }

    #[test]
    fn a4f30_aggro_radius_violated_needs_the_state_14() {
        let mut e = engine();
        let a = named(&mut e, &[], 0xaaa);
        let process = object(&mut e, &[(VSLOT_PROCESS_AGGRO_OBJECT, SLOT_VALUE)]);
        stub(&mut e, ACTOR_GET_PROCESS, process);
        stub(&mut e, SLOT_VALUE, 0x7000);
        stub(&mut e, AGGRO_GET_STATE, AGGRO_STATE_VIOLATED);
        stub(&mut e, REF_GET_NAME, 0x9300);
        assert_eq!(run(&mut e, 0x005a_4f30, a, 0, 0), (true, 1.0));
        let lines = printed(&mut e, 0.0, 0x005a_4f30, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_619c, 0x9300])]);
        stub(&mut e, AGGRO_GET_STATE, 13);
        assert_eq!(run(&mut e, 0x005a_4f30, a, 0, 0), (true, 0.0));
        let lines = printed(&mut e, 0.0, 0x005a_4f30, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_6170, 0x9300])]);
        stub(&mut e, SLOT_VALUE, 0);
        assert_eq!(run(&mut e, 0x005a_4f30, a, 0, 0), (true, 0.0));
        stub(&mut e, ACTOR_GET_PROCESS, 0);
        assert_eq!(run(&mut e, 0x005a_4f30, a, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_4f30, plain, 0, 0), (true, 0.0));
        assert!(printed(&mut e, 0.0, 0x005a_4f30, plain, 0, 0).is_empty());
    }

    /// `engine()` plus the pages the third session's functions read.
    fn engine_s3() -> Engine {
        let mut e = engine();
        for page in [
            0x0101_f000u32,
            0x0102_b000,
            0x0101_7000,
            0x011d_b000,
            0x011f_1000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(DOUBLE_ONE_HUNDRED, 100.0f64);
        e
    }

    /// A stub returning `value` in ST0 that checks its argument words.
    fn stub_st0_args(e: &mut Engine, addr: u32, expected: Vec<u32>, value: f64) {
        e.register_double(addr, move |_, a| {
            assert_eq!(a, expected.as_slice());
            Ret {
                st0: value,
                ..Ret::default()
            }
        });
    }

    /// A stub returning `eax` that checks its argument words.
    fn stub_args(e: &mut Engine, addr: u32, expected: Vec<u32>, eax: u32) {
        e.register_double(addr, move |_, a| {
            assert_eq!(a, expected.as_slice());
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn a5020_concussed_needs_the_flag_and_the_part_test() {
        let mut e = engine_s3();
        let a = actor(&mut e);
        e.set_global(BYTE_GLOBAL_5A50E0, 1u8);
        stub_args(&mut e, ACTOR_PART_TEST_822CF0, vec![a + 0x94, 0x21, 0], 1);
        assert_eq!(run(&mut e, 0x005a_5020, a, 0, 0), (true, 1.0));
        let lines = printed(&mut e, 0.0, 0x005a_5020, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_61c0, a + 0x100])]);
        stub_args(&mut e, ACTOR_PART_TEST_822CF0, vec![a + 0x94, 0x21, 0], 0);
        assert_eq!(run(&mut e, 0x005a_5020, a, 0, 0), (true, 0.0));
        let lines = printed(&mut e, 0.0, 0x005a_5020, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_61d8, a + 0x100])]);
        // The flag clear: 0.0, and no debug line.
        e.set_global(BYTE_GLOBAL_5A50E0, 0u8);
        assert_eq!(run(&mut e, 0x005a_5020, a, 0, 0), (true, 0.0));
        assert!(printed(&mut e, 0.0, 0x005a_5020, a, 0, 0).is_empty());
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_5020, plain, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x005a_5020, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn a50e0_returns_the_byte_global() {
        let mut e = engine_s3();
        e.set_global(BYTE_GLOBAL_5A50E0, 5u8);
        assert_eq!(e.call(0x005a_50e0, &args![]).u8(), 5);
    }

    #[test]
    fn a50f0_radiation_level_sums_three_process_floats() {
        let mut e = engine_s3();
        let a = actor(&mut e);
        let process = object(
            &mut e,
            &[
                (VSLOT_PROCESS_RADIATION_A, 0x0f00_0011),
                (VSLOT_PROCESS_RADIATION_B, 0x0f00_0012),
                (VSLOT_PROCESS_RADIATION_C, 0x0f00_0013),
            ],
        );
        stub_st0(&mut e, 0x0f00_0011, 1.5);
        stub_st0(&mut e, 0x0f00_0012, 2.25);
        stub_st0(&mut e, 0x0f00_0013, 0.5);
        stub(&mut e, ACTOR_GET_PROCESS, process);
        assert_eq!(run(&mut e, 0x005a_50f0, a, 0, 0), (true, 4.25));
        let w = words(4.25);
        let lines = printed(&mut e, 0.0, 0x005a_50f0, a, 0, 0);
        assert_eq!(
            lines,
            vec![(DEBUG_PRINT, vec![0x0103_61e8, a + 0x100, w[0], w[1]])]
        );
        // Without a process: 0.0 (and the line still prints).
        stub(&mut e, ACTOR_GET_PROCESS, 0);
        assert_eq!(run(&mut e, 0x005a_50f0, a, 0, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_50f0, plain, 0, 0), (true, 0.0));
        assert!(printed(&mut e, 0.0, 0x005a_50f0, plain, 0, 0).is_empty());
    }

    #[test]
    fn a51e0_map_marker_visible_is_zero_one_or_two() {
        let mut e = engine_s3();
        stub_args(&mut e, REF_GET_MAP_MARKER_DATA, vec![0x1000], 0x5000);
        stub(&mut e, MAP_MARKER_GET_VISIBLE, 1);
        stub(&mut e, MAP_MARKER_GET_TRAVEL_LOC, 0);
        assert_eq!(run(&mut e, 0x005a_51e0, 0x1000, 0, 0), (true, 1.0));
        stub(&mut e, MAP_MARKER_GET_TRAVEL_LOC, 1);
        assert_eq!(run(&mut e, 0x005a_51e0, 0x1000, 0, 0), (true, 2.0));
        stub(&mut e, MAP_MARKER_GET_VISIBLE, 0);
        assert_eq!(run(&mut e, 0x005a_51e0, 0x1000, 0, 0), (true, 0.0));
        stub(&mut e, REF_GET_MAP_MARKER_DATA, 0);
        assert_eq!(run(&mut e, 0x005a_51e0, 0x1000, 0, 0), (true, 0.0));
        // No reference: 0.0, and the debug line prints anyway.
        let w = words(0.0);
        let lines = printed(&mut e, 5.0, 0x005a_51e0, 0, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_6204, w[0], w[1]])]);
    }

    #[test]
    fn a5270_weapon_health_perc() {
        let mut e = engine_s3();
        let a = actor(&mut e);
        let process = object(&mut e, &[(VSLOT_PROCESS_WEAPON_ITEM, 0x0f00_0021)]);
        stub(&mut e, 0x0f00_0021, 0x6000);
        stub(&mut e, ACTOR_GET_PROCESS, process);
        e.set_global(WEAPON_COMPARE_GLOBAL, 0x6666u32);
        stub_args(&mut e, OBJECT_GET_FIELD_08, vec![0x6000], 0x7000);
        stub_st0_args(&mut e, ITEM_CHANGE_GET_ITEM_HEALTH, vec![0x6000, 1], 42.5);
        assert_eq!(run(&mut e, 0x005a_5270, a, 0, 0), (true, 42.5));
        let w = words(42.5);
        let lines = printed(&mut e, 0.0, 0x005a_5270, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_6218, w[0], w[1]])]);
        // The item's field equals the global: 100.0.
        stub(&mut e, OBJECT_GET_FIELD_08, 0x6666);
        assert_eq!(run(&mut e, 0x005a_5270, a, 0, 0), (true, 100.0));
        // No weapon item: 100.0.
        stub(&mut e, 0x0f00_0021, 0);
        assert_eq!(run(&mut e, 0x005a_5270, a, 0, 0), (true, 100.0));
        // No process or no actor: the result is left alone.
        stub(&mut e, ACTOR_GET_PROCESS, 0);
        assert_eq!(run(&mut e, 0x005a_5270, a, 0, 0), (true, SENTINEL));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_5270, plain, 0, 0), (true, SENTINEL));
    }

    #[test]
    fn a5330_is_greeting_player_compares_the_world() {
        let mut e = engine_s3();
        let a = actor(&mut e);
        let process = object(&mut e, &[(VSLOT_PROCESS_TEST_30C, SLOT_YES)]);
        stub(&mut e, ACTOR_GET_PROCESS, process);
        stub_args(&mut e, REF_GET_PARENT_WORLD_SIMPLE, vec![a], 0x1234);
        e.set_global(PLAYER, 0x1234u32);
        assert_eq!(run(&mut e, 0x005a_5330, a, 0, 0), (true, 1.0));
        let w = words(1.0);
        let lines = printed(&mut e, 0.0, 0x005a_5330, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_6238, w[0], w[1]])]);
        e.set_global(PLAYER, 0x4321u32);
        assert_eq!(run(&mut e, 0x005a_5330, a, 0, 0), (true, 0.0));
        let other = object(&mut e, &[(VSLOT_PROCESS_TEST_30C, SLOT_NO)]);
        stub(&mut e, ACTOR_GET_PROCESS, other);
        e.set_global(PLAYER, 0x1234u32);
        assert_eq!(run(&mut e, 0x005a_5330, a, 0, 0), (true, 0.0));
        stub(&mut e, ACTOR_GET_PROCESS, 0);
        assert_eq!(run(&mut e, 0x005a_5330, a, 0, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x005a_5330, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn a53e0_is_combat_target() {
        let mut e = engine_s3();
        let a = actor(&mut e);
        stub_args(&mut e, ACTOR_IS_IN_COMBAT_WITH_ACTOR, vec![a, 0x777], 1);
        assert_eq!(run(&mut e, 0x005a_53e0, a, 0x777, 0), (true, 1.0));
        let w = words(1.0);
        let lines = printed(&mut e, 0.0, 0x005a_53e0, a, 0x777, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_6258, w[0], w[1]])]);
        stub_args(&mut e, ACTOR_IS_IN_COMBAT_WITH_ACTOR, vec![a, 0x777], 0);
        assert_eq!(run(&mut e, 0x005a_53e0, a, 0x777, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_53e0, plain, 0x777, 0), (true, 0.0));
    }

    /// One VATS check: the actor, the angle word and the debug format.
    fn check_vats(addr: u32, callee: u32, angle: f32, format: u32) {
        let mut e = engine_s3();
        e.set_global(VATS_ANGLE_RIGHT, std::f32::consts::FRAC_PI_2);
        e.set_global(VATS_ANGLE_LEFT, 4.712_389f32);
        e.set_global(VATS_ANGLE_BACK, std::f32::consts::PI);
        let a = actor(&mut e);
        stub_st0_args(&mut e, callee, vec![a, 0x55, angle.to_bits()], 0.75);
        assert_eq!(run(&mut e, addr, a, 0x55, 0), (true, 0.75));
        let w = words(0.75);
        let lines = printed(&mut e, 0.0, addr, a, 0x55, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![format, w[0], w[1]])]);
        // A reference that is not an actor, or none: 0.0 and the line.
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, addr, plain, 0x55, 0), (true, 0.0));
        assert_eq!(run(&mut e, addr, 0, 0x55, 0), (true, 0.0));
        let w = words(0.0);
        let lines = printed(&mut e, 3.0, addr, 0, 0x55, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![format, w[0], w[1]])]);
    }

    #[test]
    fn a5460_vats_right_area_free() {
        let angle = std::f32::consts::FRAC_PI_2;
        check_vats(0x005a_5460, ACTOR_GET_VATS_AREA_FREE, angle, 0x0103_6270);
    }

    #[test]
    fn a54f0_vats_left_area_free() {
        check_vats(
            0x005a_54f0,
            ACTOR_GET_VATS_AREA_FREE,
            4.712_389,
            0x0103_628c,
        );
    }

    #[test]
    fn a5580_vats_back_area_free() {
        let angle = std::f32::consts::PI;
        check_vats(0x005a_5580, ACTOR_GET_VATS_AREA_FREE, angle, 0x0103_62a8);
    }

    #[test]
    fn a5610_vats_front_area_free() {
        check_vats(0x005a_5610, ACTOR_GET_VATS_AREA_FREE, 0.0, 0x0103_62c4);
    }

    #[test]
    fn a5690_vats_right_target_visible() {
        let angle = std::f32::consts::FRAC_PI_2;
        check_vats(
            0x005a_5690,
            ACTOR_GET_VATS_TARGET_VISIBLE,
            angle,
            0x0103_62e0,
        );
    }

    #[test]
    fn a5720_vats_left_target_visible() {
        check_vats(
            0x005a_5720,
            ACTOR_GET_VATS_TARGET_VISIBLE,
            4.712_389,
            0x0103_6300,
        );
    }

    #[test]
    fn a57b0_vats_back_target_visible() {
        let angle = std::f32::consts::PI;
        check_vats(
            0x005a_57b0,
            ACTOR_GET_VATS_TARGET_VISIBLE,
            angle,
            0x0103_6320,
        );
    }

    #[test]
    fn a5840_vats_front_target_visible() {
        check_vats(0x005a_5840, ACTOR_GET_VATS_TARGET_VISIBLE, 0.0, 0x0103_6340);
    }

    #[test]
    fn a58c0_xp_for_next_level() {
        let mut e = engine_s3();
        let player = e.mem.alloc(0x300);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(player + PLAYER_PART_OFFSET, vtable);
        e.mem.set_u32(vtable + VSLOT_PLAYER_PART_08, 0x0f00_0031);
        stub_args(
            &mut e,
            0x0f00_0031,
            vec![player + PLAYER_PART_OFFSET, 0x18],
            1000,
        );
        e.set_global(PLAYER, player);
        // The high bits of the level word are dropped.
        stub(&mut e, PLAYER_GET_LEVEL, 0x7_0005);
        let max_ptr = e.mem.alloc(8);
        e.mem.set_u32(max_ptr, 30);
        stub(&mut e, SETTING_GET_INT_PTR, max_ptr);
        stub_args(&mut e, GET_REQUIRED_EXPERIENCE_POINTS, vec![6], 1500);
        assert_eq!(run(&mut e, 0x005a_58c0, 0, 0, 0), (true, 500.0));
        let w = words(500.0);
        let lines = printed(&mut e, 0.0, 0x005a_58c0, 0, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_6360, w[0], w[1]])]);
        // At the maximum level: 0.0 and the message.
        e.mem.set_u32(max_ptr, 5);
        assert_eq!(run(&mut e, 0x005a_58c0, 0, 0, 0), (true, 0.0));
        let lines = printed(&mut e, 0.0, 0x005a_58c0, 0, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_637c])]);
    }

    #[test]
    fn a59a0_ignore_crime() {
        let mut e = engine_s3();
        let a = object(
            &mut e,
            &[
                (VSLOT_IS_ACTOR, IS_ACTOR_YES),
                (VSLOT_ACTOR_TEST_320, SLOT_YES),
            ],
        );
        assert_eq!(run(&mut e, 0x005a_59a0, a, 0, 0), (true, 1.0));
        let lines = printed(&mut e, 0.0, 0x005a_59a0, a, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_63a0, a + 0x100])]);
        let b = object(
            &mut e,
            &[
                (VSLOT_IS_ACTOR, IS_ACTOR_YES),
                (VSLOT_ACTOR_TEST_320, SLOT_NO),
            ],
        );
        assert_eq!(run(&mut e, 0x005a_59a0, b, 0, 0), (true, 0.0));
        let lines = printed(&mut e, 0.0, 0x005a_59a0, b, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_63bc, b + 0x100])]);
        assert_eq!(run(&mut e, 0x005a_59a0, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn a5a50_gore_disabled_is_exactly_one() {
        let mut e = engine_s3();
        let byte = e.mem.alloc(4);
        stub_args(&mut e, GORE_GET_BYTE_PTR, vec![GORE_OBJECT], byte);
        e.mem.set_u8(byte, 1);
        assert_eq!(run(&mut e, 0x005a_5a50, 0, 0, 0), (true, 1.0));
        let w = words(1.0);
        let lines = printed(&mut e, 0.0, 0x005a_5a50, 0, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_63d4, w[0], w[1]])]);
        e.mem.set_u8(byte, 2);
        assert_eq!(run(&mut e, 0x005a_5a50, 0, 0, 0), (true, 0.0));
    }

    #[test]
    fn a5ab0_actors_in_high_reads_the_count() {
        let mut e = engine_s3();
        e.mem.set_u32(ACTORS_IN_HIGH_OBJECT + 0x103b0, 7);
        assert_eq!(run(&mut e, 0x005a_5ab0, 0, 0, 0), (true, 7.0));
        e.mem.set_u32(ACTORS_IN_HIGH_OBJECT + 0x103b0, -3i32 as u32);
        assert_eq!(run(&mut e, 0x005a_5ab0, 0, 0, 0), (true, -3.0));
        let w = words(-3.0);
        let lines = printed(&mut e, 0.0, 0x005a_5ab0, 0, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_63ec, w[0], w[1]])]);
    }

    #[test]
    fn a5b10_reads_the_word_at_103b0() {
        let mut e = engine_s3();
        let base = 0x011e_0000u32;
        e.map(0x011f_0000, 0x1000);
        e.mem.set_u32(base + 0x103b0, 0xabcd);
        assert_eq!(e.call(0x005a_5b10, &args![base]).u32(), 0xabcd);
    }

    #[test]
    fn a5b30_force_hit_reaction_reads_the_actor_byte() {
        let mut e = engine_s3();
        let a = actor(&mut e);
        e.mem.set_u8(a + 0x1b1, 3);
        assert_eq!(run(&mut e, 0x005a_5b30, a, 0, 0), (true, 3.0));
        let w = words(3.0);
        let lines = printed(&mut e, 0.0, 0x005a_5b30, a, 0, 0);
        assert_eq!(
            lines,
            vec![(DEBUG_PRINT, vec![0x0103_6408, a + 0x100, w[0], w[1]])]
        );
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_5b30, plain, 0, 0), (true, SENTINEL));
        assert_eq!(run(&mut e, 0x005a_5b30, 0, 0, 0), (true, SENTINEL));
    }

    #[test]
    fn a5bb0_reads_the_byte_at_1b1() {
        let mut e = engine_s3();
        let o = e.mem.alloc(0x300);
        e.mem.set_u8(o + 0x1b1, 9);
        assert_eq!(e.call(0x005a_5bb0, &args![o]).u8(), 9);
    }

    /// The three reputation-like functions: the form, the value, the line.
    fn check_form_value(addr: u32, callee: u32, p2: u32, passed: u32, ret: Ret, expected: f64) {
        let mut e = engine_s3();
        let form = e.mem.alloc(0x40);
        e.register_double(callee, move |_, a| {
            assert_eq!(a, [form, passed]);
            ret
        });
        stub_args(&mut e, FORM_PART_GET_NAME, vec![form + 0x18], 0x9400);
        let result = e.mem.alloc(8);
        e.mem.set_f64(result, SENTINEL);
        assert!(e.call(addr, &args![0x1000u32, form, p2, result]).bool());
        assert_eq!(e.mem.f64(result), expected);
        e.call_log = Some(vec![]);
        e.call(addr, &args![0x1000u32, form, p2, result]);
        let w = words(expected);
        let log = take_log(&mut e);
        assert!(calls_to(&log, DEBUG_PRINT).is_empty());
        trace_on(&mut e);
        e.call(addr, &args![0x1000u32, form, p2, result]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, DEBUG_PRINT),
            vec![vec![0x0103_643c, 0x9400, w[0], w[1]]]
        );
        // A null form leaves the result alone and prints nothing.
        e.mem.set_f64(result, SENTINEL);
        trace_on(&mut e);
        e.call(addr, &args![0x1000u32, 0u32, p2, result]);
        assert_eq!(e.mem.f64(result), SENTINEL);
        assert!(calls_to(&take_log(&mut e), DEBUG_PRINT).is_empty());
    }

    #[test]
    fn a5bd0_form_value_from_6155c0() {
        let ret = Ret {
            st0: 12.5,
            ..Ret::default()
        };
        check_form_value(0x005a_5bd0, FORM_QUERY_6155C0, 0x33, 0x33, ret, 12.5);
    }

    #[test]
    fn a5c40_form_value_from_616a60() {
        let ret = Ret {
            st0: -4.0,
            ..Ret::default()
        };
        check_form_value(0x005a_5c40, FORM_QUERY_616A60, 0x33, 0x33, ret, -4.0);
    }

    #[test]
    fn a5cb0_form_value_is_a_signed_byte() {
        let ret = Ret {
            eax: 0xfe,
            ..Ret::default()
        };
        // Only the low byte of the second word is passed on.
        check_form_value(0x005a_5cb0, FORM_QUERY_616A90, 0x1ff, 0xff, ret, -2.0);
    }

    /// The two objective functions: the quest, the objective and the line.
    fn check_objective(addr: u32, field: u32, expected: f64, format: u32) {
        let mut e = engine_s3();
        let objective = e.mem.alloc(0x40);
        e.mem.set_u32(objective + 0x20, field);
        stub_args(&mut e, QUEST_GET_OBJECTIVE, vec![0x4000, 3], objective);
        let result = e.mem.alloc(8);
        e.mem.set_f64(result, SENTINEL);
        trace_on(&mut e);
        e.call(addr, &args![0x1000u32, 0x4000u32, 3u32, result]);
        assert_eq!(e.mem.f64(result), expected);
        let w = words(expected);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, DEBUG_PRINT), vec![vec![format, w[0], w[1]]]);
        // No objective, or no quest: the result is left alone, no line.
        stub(&mut e, QUEST_GET_OBJECTIVE, 0);
        e.mem.set_f64(result, SENTINEL);
        trace_on(&mut e);
        e.call(addr, &args![0x1000u32, 0x4000u32, 3u32, result]);
        e.call(addr, &args![0x1000u32, 0u32, 3u32, result]);
        assert_eq!(e.mem.f64(result), SENTINEL);
        assert!(calls_to(&take_log(&mut e), DEBUG_PRINT).is_empty());
    }

    #[test]
    fn a5d30_objective_completed_needs_state_above_one() {
        check_objective(0x005a_5d30, 2, 1.0, 0x0103_6458);
        check_objective(0x005a_5d30, 1, 0.0, 0x0103_6458);
    }

    #[test]
    fn a5dc0_greater_than_one() {
        let mut e = engine_s3();
        let o = e.mem.alloc(0x40);
        for (value, expected) in [(0u32, false), (1, false), (2, true), (-5i32 as u32, false)] {
            e.mem.set_u32(o + 0x20, value);
            assert_eq!(e.call(0x005a_5dc0, &args![o]).bool(), expected);
        }
    }

    #[test]
    fn a5de0_objective_displayed_is_bit_zero() {
        check_objective(0x005a_5de0, 3, 1.0, 0x0103_6478);
        check_objective(0x005a_5de0, 2, 0.0, 0x0103_6478);
    }

    #[test]
    fn a5e70_bit_zero_of_the_word_at_20() {
        let mut e = engine_s3();
        let o = e.mem.alloc(0x40);
        e.mem.set_u32(o + 0x20, 0xff);
        assert_eq!(e.call(0x005a_5e70, &args![o]).u32(), 1);
        e.mem.set_u32(o + 0x20, 0xfe);
        assert_eq!(e.call(0x005a_5e70, &args![o]).u32(), 0);
    }

    #[test]
    fn a5e90_player_tag_skill() {
        let mut e = engine_s3();
        let a = actor(&mut e);
        let class = e.mem.alloc(0x80);
        for (i, value) in [11u32, 12, 5, 14].into_iter().enumerate() {
            e.mem.set_u32(class + 0x44 + 4 * i as u32, value);
        }
        e.set_global(PLAYER, 0x8000u32);
        stub_args(&mut e, TAG_SKILL_TEST, vec![5], 1);
        stub_args(&mut e, PLAYER_GET_CLASS, vec![0x8000], class);
        assert_eq!(run(&mut e, 0x005a_5e90, a, 5, 0), (true, 1.0));
        let w = words(1.0);
        let lines = printed(&mut e, 0.0, 0x005a_5e90, a, 5, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_6498, w[0], w[1]])]);
        // The skill is not among the four.
        e.mem.set_u32(class + 0x4c, 6);
        assert_eq!(run(&mut e, 0x005a_5e90, a, 5, 0), (true, 0.0));
        // The first test fails.
        e.mem.set_u32(class + 0x4c, 5);
        stub_args(&mut e, TAG_SKILL_TEST, vec![5], 0);
        assert_eq!(run(&mut e, 0x005a_5e90, a, 5, 0), (true, 0.0));
        let plain = non_actor(&mut e);
        assert_eq!(run(&mut e, 0x005a_5e90, plain, 5, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x005a_5e90, 0, 5, 0), (true, 0.0));
    }

    #[test]
    fn a5f40_matches_any_of_four_words() {
        let mut e = engine_s3();
        let record = e.mem.alloc(0x80);
        for (i, value) in [1u32, 2, 3, 4].into_iter().enumerate() {
            e.mem.set_u32(record + 0x44 + 4 * i as u32, value);
        }
        for value in [1u32, 2, 3, 4] {
            assert!(e.call(0x005a_5f40, &args![record, value]).bool());
        }
        assert!(!e.call(0x005a_5f40, &args![record, 5u32]).bool());
        assert!(!e.call(0x005a_5f40, &args![record, 0u32]).bool());
    }

    #[test]
    fn a5f90_hardcore_reads_the_player_byte() {
        let mut e = engine_s3();
        e.set_global(PLAYER, 0x5000u32);
        stub_args(&mut e, PLAYER_IS_HARDCORE, vec![0x5000], 1);
        assert_eq!(run(&mut e, 0x005a_5f90, 0, 0, 0), (true, 1.0));
        let w = words(1.0);
        let lines = printed(&mut e, 0.0, 0x005a_5f90, 0, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_64b4, w[0], w[1]])]);
        // No player: the result is left alone.
        e.set_global(PLAYER, 0u32);
        assert_eq!(run(&mut e, 0x005a_5f90, 0, 0, 0), (true, SENTINEL));
    }

    #[test]
    fn a5ff0_always_hardcore_reads_the_byte_at_e38() {
        let mut e = engine_s3();
        let player = e.mem.alloc(0x1000);
        e.mem.set_u8(player + 0xe38, 1);
        e.set_global(PLAYER, player);
        assert_eq!(run(&mut e, 0x005a_5ff0, 0, 0, 0), (true, 1.0));
        let w = words(1.0);
        let lines = printed(&mut e, 0.0, 0x005a_5ff0, 0, 0, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_64c8, w[0], w[1]])]);
        e.mem.set_u8(player + 0xe38, 0);
        assert_eq!(run(&mut e, 0x005a_5ff0, 0, 0, 0), (true, 0.0));
        e.set_global(PLAYER, 0u32);
        assert_eq!(run(&mut e, 0x005a_5ff0, 0, 0, 0), (true, SENTINEL));
    }

    #[test]
    fn a6050_reads_the_byte_at_e38() {
        let mut e = engine_s3();
        let o = e.mem.alloc(0x1000);
        e.mem.set_u8(o + 0xe38, 7);
        assert_eq!(e.call(0x005a_6050, &args![o]).u8(), 7);
    }

    #[test]
    fn a6070_challenge_locked_is_bit_zero_of_the_word_at_70() {
        let mut e = engine_s3();
        let o = e.mem.alloc(0x100);
        e.mem.set_u32(o + 0x70, 3);
        assert_eq!(run(&mut e, 0x005a_6070, 0, o, 0), (true, 1.0));
        let w = words(1.0);
        let lines = printed(&mut e, 0.0, 0x005a_6070, 0, o, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_64e4, w[0], w[1]])]);
        e.mem.set_u32(o + 0x70, 2);
        assert_eq!(run(&mut e, 0x005a_6070, 0, o, 0), (true, 0.0));
        assert_eq!(run(&mut e, 0x005a_6070, 0, 0, 0), (true, SENTINEL));
    }

    #[test]
    fn a60d0_bit_zero_of_the_word_at_70() {
        let mut e = engine_s3();
        let o = e.mem.alloc(0x100);
        e.mem.set_u32(o + 0x70, 0xff);
        assert_eq!(e.call(0x005a_60d0, &args![o]).u32(), 1);
        e.mem.set_u32(o + 0x70, 0xfe);
        assert_eq!(e.call(0x005a_60d0, &args![o]).u32(), 0);
    }

    #[test]
    fn a60f0_challenge_completed_passes_the_value_through() {
        let mut e = engine_s3();
        stub_args(&mut e, CHALLENGE_GET_COMPLETED, vec![0x6100], 7);
        assert_eq!(run(&mut e, 0x005a_60f0, 0, 0x6100, 0), (true, 7.0));
        let w = words(7.0);
        let lines = printed(&mut e, 0.0, 0x005a_60f0, 0, 0x6100, 0);
        assert_eq!(lines, vec![(DEBUG_PRINT, vec![0x0103_64fc, w[0], w[1]])]);
        stub(&mut e, CHALLENGE_GET_COMPLETED, -1i32 as u32);
        assert_eq!(run(&mut e, 0x005a_60f0, 0, 0x6100, 0), (true, -1.0));
        assert_eq!(run(&mut e, 0x005a_60f0, 0, 0, 0), (true, SENTINEL));
    }
}
