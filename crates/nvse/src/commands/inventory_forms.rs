//! More of xNVSE's `Commands_Inventory.cpp` (xNVSE), batch I-15a: whether a form is playable and
//! the biped slot and flag masks, the equipped object of a slot and its health, a reference's
//! current health and whether it is worn; with `GetRefCount` of `Commands_MiscRef.cpp`. Each reads
//! or flips data the game already has; the setters that make extra data (`SetCurrentHealth`,
//! `SetEquippedCurrentHealth`, `SetRefCount`) come with the next batch.

use super::{
    console_print, double_words, dynamic_cast, form_or_caller, is_console_mode, rtti, Read,
};
use crate::invref::{self, has_type, list_items};
use crate::table::{Command, Param, Returns};
use crate::{handled, CommandArgs, EvalArgs};
use engine_core::prelude::*;

const fn p(name: &'static str, kind: u32, optional: bool) -> Param {
    Param {
        name,
        kind,
        optional,
    }
}
/// `kParams_OneOptionalObjectID`, `kParams_OneInt_OneOptionalObjectID`, `kParams_OneInt` (xNVSE).
const ONE_OPTIONAL_OBJECT_ID: &[Param] = &[p("item", 0x03, true)];
const INT_OPTIONAL_OBJECT_ID: &[Param] = &[p("path type", 0x01, false), p("item", 0x03, true)];
const ONE_INT: &[Param] = &[p("int", 0x01, false)];

/// `TESBipedModelForm` (xNVSE): `partMask` (`+4`, the slots; `eSlot_Full` the 20 used bits) and
/// `bipedFlags` (`+8`, a byte; `eBipedFlag_NonPlayable` 0x40).
const PART_MASK: u32 = 4;
const SLOT_FULL: u32 = 0xf_ffff;
const BIPED_FLAGS: u32 = 8;
const BIPED_NON_PLAYABLE: u8 = 0x40;
/// `TESObjectWEAP::weaponFlags1` (`+0x100`, bit 0x80 non-playable), `TESAmmo::flags` (`+0xAC`,
/// `kFlags_NonPlayable` 2), `TESRace::raceFlags` (`+0x70`, `kFlag_Playable` 1).
const WEAPON_FLAGS1: u32 = 0x100;
const AMMO_FLAGS: u32 = 0xac;
const RACE_FLAGS: u32 = 0x70;
/// `TESForm::typeID` (`+4`) and `kFormType_TESObjectWEAP`.
const FORM_TYPE: u32 = 4;
const TYPE_WEAPON: u8 = 0x28;
/// `ePart_Weapon`'s slot bit.
const SLOT_WEAPON: u32 = 1 << 5;
/// `TESHealthForm::health` (`+4`), `BGSDestructibleObjectForm::data` (`+4`) and its `health`
/// (`+0`); `ExtraHealth::health` and `ExtraObjectHealth::health` (`+0xC`, floats).
const HEALTH_FORM_HEALTH: u32 = 4;
const DESTRUCTIBLE_DATA: u32 = 4;
const EXTRA_HEALTH_VALUE: u32 = 0xc;
/// `kExtraData_Health`, `kExtraData_ObjectHealth`, `kExtraData_WornLeft`.
const EXTRA_HEALTH: u32 = 0x25;
const EXTRA_OBJECT_HEALTH: u32 = 0x56;
const EXTRA_WORN_LEFT: u32 = 0x17;
/// `TESForm::refID`.
const FORM_ID: u32 = 0xc;

const fn cmd(
    opcode: u32,
    name: &'static str,
    short: &'static str,
    help: &'static str,
    needs_ref: bool,
    params: &'static [Param],
    execute: AbiFn,
) -> Command {
    Command {
        opcode,
        name,
        short,
        help,
        needs_ref,
        params,
        execute,
        eval: None,
        returns: Returns::Default,
    }
}

pub static COMMANDS: &[Command] = &[
    Command {
        returns: Returns::Form,
        ..cmd(
            0x1449,
            "GetEquippedObject",
            "GetEqObj",
            "returns the base object of the item in the specified slot",
            true,
            ONE_INT,
            get_equipped_object,
        )
    },
    Command {
        eval: Some(get_equipped_current_health_eval),
        ..cmd(
            0x144a,
            "GetEquippedCurrentHealth",
            "GetEqCurHealth",
            "returns the current health of the object equipped in the specified slot",
            true,
            ONE_INT,
            get_equipped_current_health,
        )
    },
    Command {
        eval: Some(get_current_health_eval),
        ..cmd(
            0x1451,
            "GetCurrentHealth",
            "",
            "returns the current health of the calling ref",
            true,
            &[],
            get_current_health,
        )
    },
    cmd(
        0x1476,
        "GetRefCount",
        "",
        "returns the count in a stacked reference",
        true,
        &[],
        get_ref_count,
    ),
    cmd(
        0x15ac,
        "IsPlayable",
        "",
        "returns whether the biped form is usable by the player.",
        false,
        ONE_OPTIONAL_OBJECT_ID,
        is_playable,
    ),
    cmd(
        0x15ad,
        "SetIsPlayable",
        "",
        "sets whether the armor is usable by the player or not.",
        false,
        INT_OPTIONAL_OBJECT_ID,
        set_is_playable,
    ),
    cmd(
        0x15ae,
        "GetEquipmentSlotsMask",
        "GetESM",
        "returns the slots used by the biped form as a bitmask.",
        false,
        ONE_OPTIONAL_OBJECT_ID,
        get_equipment_slots_mask,
    ),
    cmd(
        0x15af,
        "SetEquipmentSlotsMask",
        "SetESM",
        "sets the slots used by the biped form from a bitmask.",
        false,
        INT_OPTIONAL_OBJECT_ID,
        set_equipment_slots_mask,
    ),
    cmd(
        0x15b9,
        "IsEquipped",
        "",
        "returns 1 if the calling object is currently being worn",
        true,
        &[],
        is_equipped,
    ),
    cmd(
        0x15c5,
        "GetEquipmentBipedMask",
        "GetEBM",
        "gets the flags used by the biped form as a bitmask.",
        false,
        ONE_OPTIONAL_OBJECT_ID,
        get_equipment_biped_mask,
    ),
    cmd(
        0x15c6,
        "SetEquipmentBipedMask",
        "SetEBM",
        "sets the flags used by the biped form from a bitmask.",
        false,
        INT_OPTIONAL_OBJECT_ID,
        set_equipment_biped_mask,
    ),
];

/// The form of the inventory getters and setters: when the read succeeds, the given form (a
/// reference's base) or else the caller's base form.
fn read_form(e: &mut Engine, a: &CommandArgs, read: &Read, at: usize) -> Option<u32> {
    if read.ok {
        form_or_caller(e, read.form(at), a.this_obj)
    } else {
        None
    }
}

fn biped(e: &mut Engine, form: u32) -> u32 {
    dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_BIPED_MODEL_FORM)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_IsPlayable_Execute.
/// 1 unless the form is an armor, weapon or ammo marked non-playable or a race not marked
/// playable (the first of those it is); 1 with no form.
fn is_playable(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 1.0);
    let read = Read::of(e, &a);
    let Some(form) = read_form(e, &a, &read, 0) else {
        return handled(true);
    };
    let playable = {
        let b = biped(e, form);
        if b != 0 {
            Some(e.mem.u8(b + BIPED_FLAGS) & BIPED_NON_PLAYABLE == 0)
        } else {
            let weapon = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_OBJECT_WEAP);
            if weapon != 0 {
                Some(e.mem.u8(weapon + WEAPON_FLAGS1) & 0x80 == 0)
            } else {
                let ammo = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_AMMO);
                if ammo != 0 {
                    Some(e.mem.u32(ammo + AMMO_FLAGS) & 2 == 0)
                } else {
                    let race = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_RACE);
                    (race != 0).then(|| e.mem.u32(race + RACE_FLAGS) & 1 == 1)
                }
            }
        }
    };
    if let Some(playable) = playable {
        a.set_result(e, f64::from(u8::from(playable)));
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_SetIsPlayable_Execute.
/// An armor non-playable for 0, playable otherwise. xNVSE reads the integer into a `float`, so
/// the integer's bits are the float compared with 0 (0 and 0x80000000 are both 0).
fn set_is_playable(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    let do_set = f32::from_bits(read.int(0, 0));
    if let Some(form) = read_form(e, &a, &read, 1) {
        let b = biped(e, form);
        if b != 0 {
            let flags = e.mem.u8(b + BIPED_FLAGS);
            let flags = if do_set == 0.0 {
                flags | BIPED_NON_PLAYABLE
            } else {
                flags & !BIPED_NON_PLAYABLE
            };
            e.mem.set_u8(b + BIPED_FLAGS, flags);
        }
    }
    handled(true)
}

/// The biped getters: `value` of the form's biped component, 0 without one.
fn get_biped(e: &mut Engine, w: &[u32], value: fn(&Engine, u32) -> u32) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    if let Some(form) = read_form(e, &a, &read, 0) {
        let b = biped(e, form);
        if b != 0 {
            let v = value(e, b);
            a.set_result(e, f64::from(v));
        }
    }
    handled(true)
}

/// The biped setters: `set` on the form's biped component with the integer.
fn set_biped(e: &mut Engine, w: &[u32], set: fn(&mut Engine, u32, u32)) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    let mask = read.int(0, 0);
    if let Some(form) = read_form(e, &a, &read, 1) {
        let b = biped(e, form);
        if b != 0 {
            set(e, b, mask);
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetEquipmentSlotsMask_Execute (TESBipedModelForm::GetSlotsMask).
fn get_equipment_slots_mask(e: &mut Engine, w: &[u32]) -> Ret {
    get_biped(e, w, |e, b| e.mem.u32(b + PART_MASK))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_SetEquipmentSlotsMask_Execute (TESBipedModelForm::SetSlotsMask: the 20 slot bits).
fn set_equipment_slots_mask(e: &mut Engine, w: &[u32]) -> Ret {
    set_biped(e, w, |e, b, mask| {
        e.mem.set_u32(b + PART_MASK, mask & SLOT_FULL)
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetEquipmentBipedMask_Execute (TESBipedModelForm::GetBipedMask).
fn get_equipment_biped_mask(e: &mut Engine, w: &[u32]) -> Ret {
    get_biped(e, w, |e, b| u32::from(e.mem.u8(b + BIPED_FLAGS)))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_SetEquipmentBipedMask_Execute (TESBipedModelForm::SetBipedMask: the low byte).
fn set_equipment_biped_mask(e: &mut Engine, w: &[u32]) -> Ret {
    set_biped(e, w, |e, b, mask| e.mem.set_u8(b + BIPED_FLAGS, mask as u8))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameForms.cpp:
// TESBipedModelForm::MaskForSlot.
/// The slot bit of slot `slot` (0-19), all bits for any other.
fn mask_for_slot(slot: u32) -> u32 {
    if slot < 20 {
        1 << slot
    } else {
        u32::MAX
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp: MatchBySlot.
/// Whether the item uses a slot of `mask`: a weapon the weapon slot, else its biped slots.
fn matches_slot(e: &mut Engine, item: u32, mask: u32) -> bool {
    let mut form_mask = 0;
    if item != 0 {
        if e.mem.u8(item + FORM_TYPE) == TYPE_WEAPON {
            form_mask = SLOT_WEAPON;
        } else {
            let b = biped(e, item);
            if b != 0 {
                form_mask = e.mem.u32(b + PART_MASK);
            }
        }
    }
    form_mask & mask != 0
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameExtraData.cpp:
// ExtraContainerChanges::FindEquipped, GetMatchingEquipped.
/// The first stack in the reference's container changes, in list order, whose item uses a slot
/// of `mask` and one of whose extra data lists (up to the first empty node) is worn in either
/// hand: the item and that list; `(0, 0)` for none.
fn find_equipped(e: &mut Engine, reference: u32, mask: u32) -> (u32, u32) {
    let list = invref::entry_list(e, reference);
    if list == 0 {
        return (0, 0);
    }
    for entry in list_items(e, list) {
        let extend = e.mem.u32(entry + invref::ENTRY_EXTEND_DATA);
        let item = e.mem.u32(entry + invref::ENTRY_TYPE);
        if extend == 0 || !matches_slot(e, item, mask) {
            continue;
        }
        let mut node = extend;
        while node != 0 {
            let xdata = e.mem.u32(node);
            if xdata == 0 {
                break;
            }
            if has_type(e, xdata, invref::EXTRA_WORN) || has_type(e, xdata, EXTRA_WORN_LEFT) {
                return (item, xdata);
            }
            node = e.mem.u32(node + 4);
        }
    }
    (0, 0)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetEquippedObject_Execute.
/// The base form equipped in a slot; in the console `At Slot %d: (%X)%s` with its name (a form
/// without a name component stops the game there, as xNVSE reads through the null name).
fn get_equipped_object(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if a.this_obj == 0 {
        return handled(true);
    }
    let read = Read::of(e, &a);
    if !read.ok {
        return handled(true);
    }
    let slot = read.int(0, 0);
    let (item, _) = find_equipped(e, a.this_obj, mask_for_slot(slot));
    if item != 0 {
        let id = e.mem.u32(item + FORM_ID);
        a.set_form_result(e, id);
        if is_console_mode(e) {
            let name = super::string::tes_full_name(e, item);
            let text = e.mem.u32(name + 4);
            console_print(e, "At Slot %d: (%X)%s", &[slot, id, text]);
        }
    }
    handled(true)
}

/// The equipped item's health in a slot: its stack's `ExtraHealth`, else the item's base health;
/// with the console line for each.
fn equipped_health(
    e: &mut Engine,
    reference: u32,
    slot: u32,
) -> Option<(f64, &'static str, Vec<u32>)> {
    let (item, xdata) = find_equipped(e, reference, mask_for_slot(slot));
    if item == 0 {
        return None;
    }
    let extra = if xdata != 0 {
        invref::get_by_type(e, xdata, EXTRA_HEALTH)
    } else {
        0
    };
    if extra != 0 {
        let health = e.mem.f32(extra + EXTRA_HEALTH_VALUE);
        let words = double_words(f64::from(health)).to_vec();
        return Some((f64::from(health), "GetEquippedCurrentHealth: %.2f", words));
    }
    let form = dynamic_cast(e, item, rtti::TES_FORM, rtti::TES_HEALTH_FORM);
    if form == 0 {
        return None;
    }
    let health = e.mem.u32(form + HEALTH_FORM_HEALTH);
    Some((
        f64::from(health),
        "GetEquippedCurrentHealth: baseHealth: %d",
        vec![health],
    ))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetEquippedCurrentHealth_Execute.
fn get_equipped_current_health(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if a.this_obj == 0 {
        return handled(true);
    }
    let read = Read::of(e, &a);
    if !read.ok {
        return handled(true);
    }
    if let Some((health, format, words)) = equipped_health(e, a.this_obj, read.int(0, 0)) {
        a.set_result(e, health);
        if is_console_mode(e) {
            console_print(e, format, &words);
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetEquippedCurrentHealth_Eval.
fn get_equipped_current_health_eval(e: &mut Engine, w: &[u32]) -> Ret {
    let a = EvalArgs::new(w);
    e.mem.set_f64(a.result, 0.0);
    if a.this_obj != 0 {
        if let Some((health, _, _)) = equipped_health(e, a.this_obj, a.arg1) {
            e.mem.set_f64(a.result, health);
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetCurrentHealth_Execute.
/// A reference's health: its `ExtraHealth`, else its base form's health (an integer made a
/// `float` by the conditional, as xNVSE's `?:` does); for a destructible object without a health
/// form, its `ExtraObjectHealth`, else the destructible data's.
fn current_health(e: &mut Engine, reference: u32) -> f64 {
    let base = e.mem.u32(reference + 0x20);
    let list = reference + invref::REFR_EXTRA;
    let health_form = dynamic_cast(e, base, rtti::TES_FORM, rtti::TES_HEALTH_FORM);
    if health_form != 0 {
        let extra = invref::get_by_type(e, list, EXTRA_HEALTH);
        let v = if extra != 0 {
            e.mem.f32(extra + EXTRA_HEALTH_VALUE)
        } else {
            e.mem.u32(health_form + HEALTH_FORM_HEALTH) as i32 as f32
        };
        return f64::from(v);
    }
    let destructible = dynamic_cast(e, base, rtti::TES_FORM, rtti::BGS_DESTRUCTIBLE_OBJECT_FORM);
    if destructible != 0 {
        let data = e.mem.u32(destructible + DESTRUCTIBLE_DATA);
        if data != 0 {
            let extra = invref::get_by_type(e, list, EXTRA_OBJECT_HEALTH);
            let v = if extra != 0 {
                e.mem.f32(extra + EXTRA_HEALTH_VALUE)
            } else {
                e.mem.u32(data) as i32 as f32
            };
            return f64::from(v);
        }
    }
    0.0
}

fn get_current_health(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let health = current_health(e, a.this_obj);
    a.set_result(e, health);
    if is_console_mode(e) {
        console_print(e, "GetCurrentHealth >> %.4f", &double_words(health));
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetCurrentHealth_Eval.
fn get_current_health_eval(e: &mut Engine, w: &[u32]) -> Ret {
    let a = EvalArgs::new(w);
    let health = if a.this_obj != 0 {
        current_health(e, a.this_obj)
    } else {
        0.0
    };
    e.mem.set_f64(a.result, health);
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_IsEquipped_Execute.
fn is_equipped(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let list = a.this_obj + invref::REFR_EXTRA;
    let worn = a.this_obj != 0
        && (has_type(e, list, invref::EXTRA_WORN) || has_type(e, list, EXTRA_WORN_LEFT));
    a.set_result(e, f64::from(u8::from(worn)));
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_MiscRef.cpp:
// Cmd_GetRefCount_Execute.
/// An inventory reference's stack count (its entry's count delta), else the reference's
/// `ExtraCount`, else 1.
fn get_ref_count(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let id = e.mem.u32(a.this_obj + FORM_ID);
    let entry = invref::get(e, id).map(|r| r.data.entry);
    let count = match entry {
        Some(entry) => f64::from(e.mem.u32(entry + invref::ENTRY_COUNT_DELTA) as i32),
        None => f64::from(invref::count_for_extra_list(
            e,
            a.this_obj + invref::REFR_EXTRA,
        )),
    };
    a.set_result(e, count);
    handled(true)
}

#[cfg(test)]
#[path = "inventory_forms_tests.rs"]
mod tests;
