//! xNVSE's inventory-object commands (`Commands_Inventory.cpp` (xNVSE)), batch X09 of ADR-0016's
//! build order: the base-form getters that read one component of the form.
//!
//! Each command reads an optional form (an inventory object or any form); a reference stands for
//! its base form (`TryGetREFRParent`), and no form means the calling reference's base form. Its
//! condition eval reads the condition's first parameter, or the subject's base form.

use super::{
    base_form, console_print, double_words, dynamic_cast, is_console_mode, rtti,
    try_get_refr_parent,
};
use crate::args::extract_args_ex;
use crate::table::{Command, Param, Returns};
use crate::{handled, CommandArgs, EvalArgs};
use engine_core::prelude::*;

/// `kParams_OneOptionalObjectID` (xNVSE).
const ONE_OPTIONAL_OBJECT_ID: &[Param] = &[Param {
    name: "item",
    kind: 0x03,
    optional: true,
}];
/// `kParams_OneOptionalForm` (xNVSE).
const ONE_OPTIONAL_FORM: &[Param] = &[Param {
    name: "form",
    kind: 0x3d,
    optional: true,
}];

/// `TESWeightForm::weight` (`+4`, a float), `TESAmmo::weight` (`+0xB8`), `TESHealthForm::health`
/// (`+4`, a `UInt32`), `TESValueForm::value` (`+4`, a `UInt32`), `TESForm::typeID` (`+4`, a
/// byte) (xNVSE `GameForms.h`).
const WEIGHT: u32 = 4;
const AMMO_WEIGHT: u32 = 0xb8;
const HEALTH: u32 = 4;
const VALUE: u32 = 4;
const TYPE_ID: u32 = 4;
/// `BGSEquipType::equipType` (`+4`, a `UInt32`; the exe's getter `00726070` reads it for the
/// weapon's component in `00479430`) and `BGSClipRoundsForm::clipRounds` (`+4`, a byte;
/// `BGSClipRoundsForm::CopyComponent`).
const EQUIP_TYPE: u32 = 4;
const CLIP_ROUNDS: u32 = 4;
/// `TESAttackDamageForm::damage` (`+4`, a `UInt16`; its `InitializeDataComponent` `0047fe10` clears it).
const DAMAGE: u32 = 4;

pub static COMMANDS: &[Command] = &[
    Command {
        opcode: 0x1404,
        name: "GetWeight",
        short: "",
        help: "returns the weight of the specified base form",
        needs_ref: false,
        params: ONE_OPTIONAL_OBJECT_ID,
        execute: get_weight_execute,
        eval: Some(get_weight_eval),
        returns: Returns::Default,
    },
    Command {
        opcode: 0x1405,
        name: "GetHealth",
        short: "GetBaseHealth",
        help: "returns the base health of the object or calling reference",
        needs_ref: false,
        params: ONE_OPTIONAL_FORM,
        execute: get_health_execute,
        eval: Some(get_health_eval),
        returns: Returns::Default,
    },
    Command {
        opcode: 0x1406,
        name: "GetValue",
        short: "GetItemValue",
        help: "returns the base value of the object or calling reference",
        needs_ref: false,
        params: ONE_OPTIONAL_OBJECT_ID,
        execute: get_value_execute,
        eval: Some(get_value_eval),
        returns: Returns::Default,
    },
    Command {
        opcode: 0x140a,
        name: "GetType",
        short: "GetObjectType",
        help: "returns the type of the specified base form",
        needs_ref: false,
        params: ONE_OPTIONAL_FORM,
        execute: get_type_execute,
        eval: Some(get_type_eval),
        returns: Returns::Default,
    },
    Command {
        opcode: 0x140c,
        name: "GetEquipType",
        short: "",
        help: "returns the equipment type of the inventory object",
        needs_ref: false,
        params: ONE_OPTIONAL_OBJECT_ID,
        execute: get_equip_type_execute,
        eval: Some(get_equip_type_eval),
        returns: Returns::Default,
    },
    Command {
        opcode: 0x140e,
        name: "GetWeaponClipRounds",
        short: "GetClipSize",
        help: "returns the clip size for the weapon",
        needs_ref: false,
        params: ONE_OPTIONAL_OBJECT_ID,
        execute: get_weapon_clip_rounds_execute,
        eval: Some(get_weapon_clip_rounds_eval),
        returns: Returns::Default,
    },
    Command {
        opcode: 0x140f,
        name: "GetAttackDamage",
        short: "GetDamage",
        help: "returns the attack damage for the weapon",
        needs_ref: false,
        params: ONE_OPTIONAL_OBJECT_ID,
        execute: get_attack_damage_execute,
        eval: Some(get_attack_damage_eval),
        returns: Returns::Default,
    },
    plain(
        0x1407,
        "SetWeight",
        "",
        "sets the weight of the object",
        FLOAT_OPT_ITEM,
        set_weight,
        false,
        Returns::Default,
    ),
    plain(
        0x1408,
        "SetHealth",
        "SetObjectHealth",
        "sets the base health of the specified base form",
        INT_OPT_FORM,
        set_health,
        false,
        Returns::Default,
    ),
    plain(
        0x1409,
        "SetBaseItemValue",
        "SetValue",
        "sets the monetary value of a base object",
        ITEM_INT,
        set_base_item_value,
        false,
        Returns::Default,
    ),
    plain(
        0x140b,
        "GetRepairList",
        "grl",
        "returns the repair list for the inventory object",
        ONE_OPTIONAL_OBJECT_ID,
        get_repair_list,
        false,
        Returns::Form,
    ),
    cond(
        0x1478,
        "GetArmorAR",
        "GetArmorArmorRating",
        "returns the armor rating of the specified armor.",
        ONE_OPTIONAL_OBJECT_ID,
        get_armor_ar,
        get_armor_ar_eval,
        false,
    ),
    cond(
        0x1479,
        "IsPowerArmor",
        "IsPA",
        "returns whether the biped form is considered power armor.",
        ONE_OPTIONAL_OBJECT_ID,
        is_power_armor,
        is_power_armor_eval,
        false,
    ),
    plain(
        0x147a,
        "SetIsPowerArmor",
        "SetIsPA",
        "sets whether the armor is power armor or not.",
        INT_OPT_ITEM,
        set_is_power_armor,
        false,
        Returns::Default,
    ),
    plain(
        0x147b,
        "SetRepairList",
        "",
        "sets the repair list for the specified item.",
        FORM_LIST_OPT_TARGET,
        set_repair_list,
        false,
        Returns::Default,
    ),
    cond(
        0x147c,
        "IsQuestItem",
        "",
        "returns 1 if the object or calling reference is a quest item",
        ONE_OPTIONAL_OBJECT_ID,
        is_quest_item,
        is_quest_item_eval,
        false,
    ),
    plain(
        0x147d,
        "SetQuestItem",
        "",
        "Sets whether the specified object or object reference is a quest item",
        INT_OPT_ITEM,
        set_quest_item,
        false,
        Returns::Default,
    ),
    plain(
        0x147e,
        "GetObjectEffect",
        "GetEnchantment",
        "returns the object effect of the inventory object.",
        ONE_OPTIONAL_OBJECT_ID,
        get_object_effect,
        false,
        Returns::Form,
    ),
    cond(
        0x14b3,
        "GetArmorDT",
        "GetArmorDamageThreshold",
        "returns the damage threshold of the armor.",
        ONE_OPTIONAL_OBJECT_ID,
        get_armor_dt,
        get_armor_dt_eval,
        false,
    ),
    plain(
        0x14b4,
        "SetArmorAR",
        "SetArmorArmorRating",
        "sets the armor rating of the armor.",
        INT_OPT_ITEM,
        set_armor_ar,
        false,
        Returns::Default,
    ),
    plain(
        0x14b5,
        "SetArmorDT",
        "SetArmorDamageThreshold",
        "sets the damage threshold of the armor",
        FLOAT_OPT_ITEM,
        set_armor_dt,
        false,
        Returns::Default,
    ),
    cond(
        0x14c2,
        "GetAmmoSpeed",
        "",
        "returns the speed of the specified ammo.",
        ONE_OPTIONAL_OBJECT_ID,
        get_ammo_speed,
        get_ammo_speed_eval,
        false,
    ),
    cond(
        0x14c3,
        "GetAmmoConsumedPercent",
        "",
        "returns the percentage of ammo consumed for the specified ammo.",
        ONE_OPTIONAL_OBJECT_ID,
        get_ammo_consumed_percent,
        get_ammo_consumed_percent_eval,
        false,
    ),
    plain(
        0x14c4,
        "GetAmmoCasing",
        "",
        "returns the casing of the specified ammo",
        ONE_OPTIONAL_OBJECT_ID,
        get_ammo_casing,
        false,
        Returns::Form,
    ),
    cond(
        0x1657,
        "HasAmmoEquipped",
        "IsAmmoEquipped",
        "\"returns if a specific ammo is equipped by the calling ref\"",
        ONE_OBJECT_ID,
        has_ammo_equipped,
        has_ammo_equipped_eval,
        true,
    ),
    cond(
        0x1658,
        "GetEquippedWeaponCanUseAmmo",
        "",
        "\"returns if the equipped weapon can use the specified ammo\"",
        ONE_OBJECT_ID,
        equipped_weapon_can_use_ammo,
        equipped_weapon_can_use_ammo_eval,
        true,
    ),
    cond(
        0x1659,
        "IsEquippedAmmoInList",
        "",
        "\"returns if any ammo in a list is currently equipped by the calling ref\"",
        ONE_FORM_LIST,
        has_ammo_equipped,
        has_ammo_equipped_eval,
        true,
    ),
    cond(
        0x165a,
        "GetEquippedWeaponUsesAmmoList",
        "",
        "\"returns if the equipped weapon uses a specific list of ammo\"",
        ONE_FORM_LIST,
        equipped_weapon_can_use_ammo,
        equipped_weapon_can_use_ammo_eval,
        true,
    ),
    cond(
        0x165f,
        "GetWeaponCanUseAmmo",
        "",
        "\"returns if the weapon can use the specified ammo\"",
        ONE_OBJECT_ID_OPT_ITEM,
        weapon_can_use_ammo,
        weapon_can_use_ammo_eval,
        false,
    ),
    plain(
        0x1660,
        "SetAmmoConsumedPercent",
        "",
        "\"\"",
        FLOAT_OPT_ITEM,
        set_ammo_consumed_percent,
        false,
        Returns::Default,
    ),
];

const fn p(name: &'static str, kind: u32, optional: bool) -> Param {
    Param {
        name,
        kind,
        optional,
    }
}
/// `kParams_OneFloat_OneOptionalObjectID`, `kParams_OneInt_OneOptionalObjectID`,
/// `kParams_OneInt_OneOptionalObject`, `kParams_OneObjectID_OneInt`,
/// `kParams_OneFormList_OneOptionalObjectID`, `kParams_OneObjectID`, `kParams_OneFormList`,
/// `kParams_OneObjectID_OneOptionalObjectID` (xNVSE).
const FLOAT_OPT_ITEM: &[Param] = &[p("float", 0x02, false), p("item", 0x03, true)];
const INT_OPT_ITEM: &[Param] = &[p("path type", 0x01, false), p("item", 0x03, true)];
const INT_OPT_FORM: &[Param] = &[p("int", 0x01, false), p("item", 0x3d, true)];
const ITEM_INT: &[Param] = &[p("item", 0x03, false), p("integer", 0x01, false)];
const FORM_LIST_OPT_TARGET: &[Param] = &[p("form list", 0x25, false), p("target item", 0x03, true)];
const ONE_OBJECT_ID: &[Param] = &[p("item", 0x03, false)];
const ONE_FORM_LIST: &[Param] = &[p("form list", 0x25, false)];
const ONE_OBJECT_ID_OPT_ITEM: &[Param] = &[p("item", 0x03, false), p("item", 0x03, true)];

#[allow(clippy::too_many_arguments)]
const fn plain(
    opcode: u32,
    name: &'static str,
    short: &'static str,
    help: &'static str,
    params: &'static [Param],
    execute: AbiFn,
    needs_ref: bool,
    returns: Returns,
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
        returns,
    }
}

#[allow(clippy::too_many_arguments)]
const fn cond(
    opcode: u32,
    name: &'static str,
    short: &'static str,
    help: &'static str,
    params: &'static [Param],
    execute: AbiFn,
    eval: AbiFn,
    needs_ref: bool,
) -> Command {
    Command {
        opcode,
        name,
        short,
        help,
        needs_ref,
        params,
        execute,
        eval: Some(eval),
        returns: Returns::Default,
    }
}

/// The optional form argument (`ExtractArgs(EXTRACT_ARGS, &form)`): `None` when the read
/// failed, `Some(0)` when the argument was left out.
fn optional_form(e: &mut Engine, a: &CommandArgs) -> Option<u32> {
    let read = extract_args_ex(e, a);
    read.ok
        .then(|| read.args.first().and_then(|x| x.form()).unwrap_or(0))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp: GetWeight_Call.
/// The weight of a form with a weight component, or of ammo; nothing written otherwise.
fn get_weight_call(e: &mut Engine, form: u32, result: u32) {
    let weight = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_WEIGHT_FORM);
    if weight != 0 {
        let v = e.mem.f32(weight + WEIGHT);
        e.mem.set_f64(result, f64::from(v));
    } else {
        let ammo = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_AMMO);
        if ammo != 0 {
            let v = e.mem.f32(ammo + AMMO_WEIGHT);
            e.mem.set_f64(result, f64::from(v));
        }
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp: Cmd_GetWeight_Eval.
fn get_weight_eval(e: &mut Engine, w: &[u32]) -> Ret {
    let a = EvalArgs::new(w);
    e.mem.set_f64(a.result, 0.0);
    if a.arg1 != 0 {
        get_weight_call(e, a.arg1, a.result);
    } else if a.this_obj != 0 {
        let form = base_form(e, a.this_obj);
        get_weight_call(e, form, a.result);
    }
    if is_console_mode(e) {
        let v = e.mem.f64(a.result);
        console_print(e, "GetWeight >> %.2f", &double_words(v));
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetWeight_Execute.
fn get_weight_execute(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let Some(form) = optional_form(e, &a) {
        let mut form = try_get_refr_parent(e, form);
        if form == 0 && a.this_obj != 0 {
            form = base_form(e, a.this_obj);
        }
        if form != 0 {
            return get_weight_eval(e, &[a.this_obj, form, 0, a.result]);
        }
    }
    handled(true)
}

/// The form a getter reads: the argument's base form, else the calling reference's; `None` to
/// stop (a failed read, or no form and no reference).
fn getter_form(e: &mut Engine, a: &CommandArgs) -> Option<u32> {
    let form = optional_form(e, a)?;
    let form = try_get_refr_parent(e, form);
    if form != 0 {
        return Some(form);
    }
    if a.this_obj == 0 {
        return None;
    }
    Some(base_form(e, a.this_obj))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetHealth_Execute.
fn get_health_execute(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let Some(form) = getter_form(e, &a) {
        let health = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_HEALTH_FORM);
        if health != 0 {
            let v = e.mem.u32(health + HEALTH);
            a.set_result(e, f64::from(v));
            if is_console_mode(e) {
                console_print(e, "GetHealth >> %.2f", &double_words(f64::from(v)));
            }
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp: Cmd_GetHealth_Eval.
/// The subject's base form only (its parameter is any form, which a condition cannot name).
fn get_health_eval(e: &mut Engine, w: &[u32]) -> Ret {
    let a = EvalArgs::new(w);
    e.mem.set_f64(a.result, 0.0);
    if a.this_obj != 0 {
        let form = base_form(e, a.this_obj);
        let health = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_HEALTH_FORM);
        if health != 0 {
            let v = e.mem.u32(health + HEALTH);
            e.mem.set_f64(a.result, f64::from(v));
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetValue_Execute.
fn get_value_execute(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let Some(form) = getter_form(e, &a) {
        let value = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_VALUE_FORM);
        if value != 0 {
            let v = e.mem.u32(value + VALUE);
            a.set_result(e, f64::from(v));
            if is_console_mode(e) {
                console_print(e, "GetValue >> %.2f", &double_words(f64::from(v)));
            }
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp: Cmd_GetValue_Eval.
fn get_value_eval(e: &mut Engine, w: &[u32]) -> Ret {
    let a = EvalArgs::new(w);
    e.mem.set_f64(a.result, 0.0);
    let form = if a.arg1 != 0 {
        a.arg1
    } else if a.this_obj != 0 {
        base_form(e, a.this_obj)
    } else {
        return handled(true);
    };
    let value = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_VALUE_FORM);
    if value != 0 {
        let v = e.mem.u32(value + VALUE);
        e.mem.set_f64(a.result, f64::from(v));
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp: Cmd_GetType_Execute.
fn get_type_execute(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let Some(form) = optional_form(e, &a) else {
        return handled(true);
    };
    let mut form = try_get_refr_parent(e, form);
    if form == 0 {
        if a.this_obj == 0 || base_form(e, a.this_obj) == 0 {
            return handled(true);
        }
        form = base_form(e, a.this_obj);
    }
    let kind = e.mem.u8(form + TYPE_ID);
    a.set_result(e, f64::from(kind));
    if is_console_mode(e) {
        console_print(e, "Type: %d", &[u32::from(kind)]);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp: Cmd_GetType_Eval.
fn get_type_eval(e: &mut Engine, w: &[u32]) -> Ret {
    let a = EvalArgs::new(w);
    e.mem.set_f64(a.result, 0.0);
    if a.this_obj != 0 {
        let form = base_form(e, a.this_obj);
        let kind = e.mem.u8(form + TYPE_ID);
        e.mem.set_f64(a.result, f64::from(kind));
    }
    handled(true)
}

/// The eval of a component getter (`Cmd_GetEquipType_Eval`, `Cmd_GetWeaponClipRounds_Eval`):
/// the first parameter, else the subject's base form; `read` takes the cast component.
fn component_eval(e: &mut Engine, w: &[u32], class: u32, read: fn(&Engine, u32) -> f64) -> Ret {
    let a = EvalArgs::new(w);
    e.mem.set_f64(a.result, 0.0);
    let form = if a.arg1 != 0 {
        a.arg1
    } else if a.this_obj != 0 {
        base_form(e, a.this_obj)
    } else {
        return handled(true);
    };
    let component = dynamic_cast(e, form, rtti::TES_FORM, class);
    if component != 0 {
        let v = read(e, component);
        e.mem.set_f64(a.result, v);
    }
    handled(true)
}

/// The execute of a component getter (`Cmd_GetEquipType_Execute`,
/// `Cmd_GetWeaponClipRounds_Execute`): the getter form, then the cast component.
fn component_execute(e: &mut Engine, w: &[u32], class: u32, read: fn(&Engine, u32) -> f64) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let Some(form) = getter_form(e, &a) {
        let component = dynamic_cast(e, form, rtti::TES_FORM, class);
        if component != 0 {
            let v = read(e, component);
            a.set_result(e, v);
        }
    }
    handled(true)
}

fn read_equip_type(e: &Engine, component: u32) -> f64 {
    f64::from(e.mem.u32(component + EQUIP_TYPE))
}

fn read_clip_rounds(e: &Engine, component: u32) -> f64 {
    f64::from(e.mem.u8(component + CLIP_ROUNDS))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetEquipType_Execute.
fn get_equip_type_execute(e: &mut Engine, w: &[u32]) -> Ret {
    component_execute(e, w, rtti::BGS_EQUIP_TYPE, read_equip_type)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetEquipType_Eval.
fn get_equip_type_eval(e: &mut Engine, w: &[u32]) -> Ret {
    component_eval(e, w, rtti::BGS_EQUIP_TYPE, read_equip_type)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetWeaponClipRounds_Execute.
fn get_weapon_clip_rounds_execute(e: &mut Engine, w: &[u32]) -> Ret {
    component_execute(e, w, rtti::BGS_CLIP_ROUNDS_FORM, read_clip_rounds)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetWeaponClipRounds_Eval.
fn get_weapon_clip_rounds_eval(e: &mut Engine, w: &[u32]) -> Ret {
    component_eval(e, w, rtti::BGS_CLIP_ROUNDS_FORM, read_clip_rounds)
}

fn read_damage(e: &Engine, component: u32) -> f64 {
    f64::from(e.mem.u16(component + DAMAGE))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetAttackDamage_Execute.
fn get_attack_damage_execute(e: &mut Engine, w: &[u32]) -> Ret {
    component_execute(e, w, rtti::TES_ATTACK_DAMAGE_FORM, read_damage)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetAttackDamage_Eval.
fn get_attack_damage_eval(e: &mut Engine, w: &[u32]) -> Ret {
    component_eval(e, w, rtti::TES_ATTACK_DAMAGE_FORM, read_damage)
}

/// `TESObjectARMO::armorRating` (`+0x178`, 16 bits), `damageThreshold` (`+0x17C`, a float);
/// `TESBipedModelForm::bipedFlags` (`+8`, a byte; `eBipedFlag_PowerArmor` 0x20);
/// `TESAmmo::speed` (`+0xA8`), `casing` (`+0xBC`), `ammoPercentConsumed` (`+0xC0`);
/// `TESEnchantableForm::enchantItem` (`+4`); `BGSRepairItemList::listForm` (`+4`) (xNVSE
/// `GameForms.h`, `STATIC_ASSERT(offsetof(TESObjectARMO, damageThreshold) == 0x17C)`).
const ARMOR_RATING: u32 = 0x178;
const DAMAGE_THRESHOLD: u32 = 0x17c;
const BIPED_FLAGS: u32 = 8;
const BIPED_POWER_ARMOR: u8 = 0x20;
const AMMO_SPEED: u32 = 0xa8;
const AMMO_CASING: u32 = 0xbc;
const AMMO_PERCENT_CONSUMED: u32 = 0xc0;
/// `TESForm::IsQuestItem` (vtable `+0x94`: the flag 0x400 of `+8`, `00401190`) and
/// `SetQuestItem` (`+0xCC`: sets or clears it, then `MarkAsModified(1)`, `00484860`).
const IS_QUEST_ITEM: u32 = 0x94;
const SET_QUEST_ITEM: u32 = 0xcc;
/// `kFormType_TESAmmo`.
const TYPE_AMMO: u8 = 0x29;

/// A form's id (`refID`, `+0xC`).
fn ref_id(e: &Engine, form: u32) -> u32 {
    e.mem.u32(form + 0xc)
}

/// The form a getter's eval reads: its first parameter, else the subject's base form.
fn eval_form(e: &Engine, a: &EvalArgs) -> Option<u32> {
    if a.arg1 != 0 {
        Some(a.arg1)
    } else if a.this_obj != 0 {
        Some(base_form(e, a.this_obj))
    } else {
        None
    }
}

/// `GetFullName(TESForm *)` (xNVSE `GameForms.cpp`): the form's full name, `"<no name>"` when it
/// has none or it is empty; a guest string for the console's `%s`.
fn full_name(e: &mut Engine, form: u32) -> u32 {
    const TES_FULL_NAME: u32 = 0x0118_3158;
    let name = dynamic_cast(e, form, rtti::TES_FORM, TES_FULL_NAME);
    if name != 0 {
        let (data, len) = (e.mem.u32(name + 4), e.mem.u16(name + 8));
        if data != 0 && len != 0 {
            return data;
        }
    }
    crate::table::guest_str(e, b"<no name>")
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_SetWeight_Execute.
fn set_weight(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let (v, Some(form)) = super::extract_float_and_form(e, &a) {
        let weight = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_WEIGHT_FORM);
        if weight != 0 {
            e.mem.set_f32(weight + WEIGHT, v);
        } else {
            let ammo = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_AMMO);
            if ammo != 0 {
                e.mem.set_f32(ammo + AMMO_WEIGHT, v);
            }
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_SetHealth_Execute.
fn set_health(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let (v, Some(form)) = super::extract_int_and_form(e, &a) {
        let health = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_HEALTH_FORM);
        if health != 0 {
            e.mem.set_u32(health + HEALTH, v);
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_SetBaseItemValue_Execute.
/// The form is used as given (no reference's base form) and the result is not touched.
fn set_base_item_value(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let read = super::Read::of(e, &a);
    if read.ok {
        let value = dynamic_cast(e, read.form(0), rtti::TES_FORM, rtti::TES_VALUE_FORM);
        if value != 0 {
            e.mem.set_u32(value + VALUE, read.int(1, 0));
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetRepairList_Execute.
fn get_repair_list(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let Some(form) = getter_form(e, &a) {
        let repair = dynamic_cast(e, form, rtti::TES_FORM, rtti::BGS_REPAIR_ITEM_LIST);
        if repair != 0 && e.mem.u32(repair + 4) != 0 {
            let id = ref_id(e, e.mem.u32(repair + 4));
            a.set_form_result(e, id);
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_SetRepairList_Execute.
fn set_repair_list(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = super::Read::of(e, &a);
    if read.ok {
        if let Some(form) = super::form_or_caller(e, read.form(1), a.this_obj) {
            let repair = dynamic_cast(e, form, rtti::TES_FORM, rtti::BGS_REPAIR_ITEM_LIST);
            if repair != 0 && read.form(0) != 0 {
                e.mem.set_u32(repair + 4, read.form(0));
            }
        }
    }
    handled(true)
}

/// An armour getter's execute: the getter form cast to `TESObjectARMO`, then `read`.
fn armor_execute(e: &mut Engine, w: &[u32], read: fn(&mut Engine, u32, u32)) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let Some(form) = getter_form(e, &a) {
        let armor = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_OBJECT_ARMO);
        if armor != 0 {
            read(e, armor, a.result);
        }
    }
    handled(true)
}

/// A getter's eval: [`eval_form`] cast to `class`, then `read` of the component.
fn cast_eval(e: &mut Engine, w: &[u32], class: u32, read: fn(&Engine, u32) -> f64) -> Ret {
    let a = EvalArgs::new(w);
    e.mem.set_f64(a.result, 0.0);
    if let Some(form) = eval_form(e, &a) {
        let component = dynamic_cast(e, form, rtti::TES_FORM, class);
        if component != 0 {
            let v = read(e, component);
            e.mem.set_f64(a.result, v);
        }
    }
    handled(true)
}

fn armor_rating(e: &Engine, armor: u32) -> f64 {
    f64::from(e.mem.u16(armor + ARMOR_RATING))
}

fn damage_threshold(e: &Engine, armor: u32) -> f64 {
    f64::from(e.mem.f32(armor + DAMAGE_THRESHOLD))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetArmorAR_Execute, Cmd_GetArmorAR_Eval.
fn get_armor_ar(e: &mut Engine, w: &[u32]) -> Ret {
    armor_execute(e, w, |e, armor, result| {
        let v = e.mem.u16(armor + ARMOR_RATING);
        e.mem.set_f64(result, f64::from(v));
        if is_console_mode(e) {
            let name = full_name(e, armor);
            console_print(e, "%s armor rating: %d", &[name, u32::from(v)]);
        }
    })
}

fn get_armor_ar_eval(e: &mut Engine, w: &[u32]) -> Ret {
    cast_eval(e, w, rtti::TES_OBJECT_ARMO, armor_rating)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetArmorDT_Execute, Cmd_GetArmorDT_Eval.
fn get_armor_dt(e: &mut Engine, w: &[u32]) -> Ret {
    armor_execute(e, w, |e, armor, result| {
        let v = damage_threshold(e, armor);
        e.mem.set_f64(result, v);
        if is_console_mode(e) {
            let name = full_name(e, armor);
            let [lo, hi] = double_words(v);
            console_print(e, "%s damage threshold: %f", &[name, lo, hi]);
        }
    })
}

fn get_armor_dt_eval(e: &mut Engine, w: &[u32]) -> Ret {
    cast_eval(e, w, rtti::TES_OBJECT_ARMO, damage_threshold)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_SetArmorAR_Execute.
fn set_armor_ar(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let (v, Some(form)) = super::extract_int_and_form(e, &a) {
        let armor = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_OBJECT_ARMO);
        if armor != 0 {
            e.mem.set_u16(armor + ARMOR_RATING, v as u16);
            if is_console_mode(e) {
                let name = full_name(e, armor);
                console_print(e, "Setting %s armor rating to %d", &[name, v]);
            }
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_SetArmorDT_Execute.
fn set_armor_dt(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let (v, Some(form)) = super::extract_float_and_form(e, &a) {
        let armor = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_OBJECT_ARMO);
        if armor != 0 {
            e.mem.set_f32(armor + DAMAGE_THRESHOLD, v);
            if is_console_mode(e) {
                let name = full_name(e, armor);
                let [lo, hi] = double_words(f64::from(v));
                console_print(e, "Setting %s damage threshold to %f", &[name, lo, hi]);
            }
        }
    }
    handled(true)
}

fn power_armor(e: &Engine, biped: u32) -> f64 {
    f64::from(u8::from(
        e.mem.u8(biped + BIPED_FLAGS) & BIPED_POWER_ARMOR == BIPED_POWER_ARMOR,
    ))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_IsPowerArmor_Execute, Cmd_IsPowerArmor_Eval.
fn is_power_armor(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let Some(form) = getter_form(e, &a) {
        let biped = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_BIPED_MODEL_FORM);
        if biped != 0 {
            let v = power_armor(e, biped);
            a.set_result(e, v);
        }
    }
    handled(true)
}

fn is_power_armor_eval(e: &mut Engine, w: &[u32]) -> Ret {
    cast_eval(e, w, rtti::TES_BIPED_MODEL_FORM, power_armor)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_SetIsPowerArmor_Execute (the result is not touched).
fn set_is_power_armor(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    if let (v, Some(form)) = super::extract_int_and_form(e, &a) {
        let biped = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_BIPED_MODEL_FORM);
        if biped != 0 {
            let flags = e.mem.u8(biped + BIPED_FLAGS);
            let flags = if v != 0 {
                flags | BIPED_POWER_ARMOR
            } else {
                flags & !BIPED_POWER_ARMOR
            };
            e.mem.set_u8(biped + BIPED_FLAGS, flags);
        }
    }
    handled(true)
}

fn quest_item(e: &mut Engine, form: u32) -> f64 {
    let v = e.vcall(form, IS_QUEST_ITEM, &args![]).u32() & 0xff;
    f64::from(u8::from(v != 0))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_IsQuestItem_Execute, Cmd_IsQuestItem_Eval.
fn is_quest_item(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let Some(form) = getter_form(e, &a) {
        let v = quest_item(e, form);
        a.set_result(e, v);
    }
    handled(true)
}

fn is_quest_item_eval(e: &mut Engine, w: &[u32]) -> Ret {
    let a = EvalArgs::new(w);
    e.mem.set_f64(a.result, 0.0);
    if let Some(form) = eval_form(e, &a).filter(|&f| f != 0) {
        let v = quest_item(e, form);
        e.mem.set_f64(a.result, v);
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_SetQuestItem_Execute.
fn set_quest_item(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let (v, Some(form)) = super::extract_int_and_form(e, &a) {
        e.vcall(form, SET_QUEST_ITEM, &args![u32::from(v == 1)]);
    }
    handled(true)
}

/// The form of a getter that reads with `ExtractArgsEx(&form)` and does not look at the read:
/// what it stored, else the calling reference's base form (no reference's base form taken).
fn stored_form_or_caller(e: &mut Engine, a: &CommandArgs) -> Option<u32> {
    let read = super::Read::of(e, a);
    let form = read.form(0);
    if form != 0 {
        return Some(form);
    }
    (a.this_obj != 0).then(|| base_form(e, a.this_obj))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetObjectEffect_Execute.
fn get_object_effect(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let Some(form) = stored_form_or_caller(e, &a) {
        let enchantable = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_ENCHANTABLE_FORM);
        if enchantable != 0 && e.mem.u32(enchantable + 4) != 0 {
            let id = ref_id(e, e.mem.u32(enchantable + 4));
            a.set_form_result(e, id);
        }
    }
    handled(true)
}

/// An ammo getter's execute (`ExtractArgsEx(&form)`, the form as stored).
fn ammo_execute(e: &mut Engine, w: &[u32], read: fn(&mut Engine, u32, &CommandArgs)) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    if let Some(form) = stored_form_or_caller(e, &a) {
        let ammo = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_AMMO);
        if ammo != 0 {
            read(e, ammo, &a);
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetAmmoSpeed_Execute, Cmd_GetAmmoSpeed_Eval.
fn get_ammo_speed(e: &mut Engine, w: &[u32]) -> Ret {
    ammo_execute(e, w, |e, ammo, a| {
        let v = f64::from(e.mem.f32(ammo + AMMO_SPEED));
        a.set_result(e, v);
        if is_console_mode(e) {
            let name = full_name(e, ammo);
            let [lo, hi] = double_words(v);
            console_print(e, "%s ammo speed: %f", &[name, lo, hi]);
        }
    })
}

fn get_ammo_speed_eval(e: &mut Engine, w: &[u32]) -> Ret {
    cast_eval(e, w, rtti::TES_AMMO, |e, ammo| {
        f64::from(e.mem.f32(ammo + AMMO_SPEED))
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetAmmoConsumedPercent_Execute, Cmd_GetAmmoConsumedPercent_Eval.
fn get_ammo_consumed_percent(e: &mut Engine, w: &[u32]) -> Ret {
    ammo_execute(e, w, |e, ammo, a| {
        let v = f64::from(e.mem.f32(ammo + AMMO_PERCENT_CONSUMED));
        a.set_result(e, v);
    })
}

fn get_ammo_consumed_percent_eval(e: &mut Engine, w: &[u32]) -> Ret {
    cast_eval(e, w, rtti::TES_AMMO, |e, ammo| {
        f64::from(e.mem.f32(ammo + AMMO_PERCENT_CONSUMED))
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetAmmoCasing_Execute.
fn get_ammo_casing(e: &mut Engine, w: &[u32]) -> Ret {
    ammo_execute(e, w, |e, ammo, a| {
        let casing = e.mem.u32(ammo + AMMO_CASING);
        if casing != 0 {
            let id = ref_id(e, casing);
            a.set_form_result(e, id);
        }
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_SetAmmoConsumedPercent_Execute.
/// The percentage is an uninitialised `float` in the C++ when the read stops before it; here it
/// is 0 then. The result is not touched.
fn set_ammo_consumed_percent(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let read = super::Read::of(e, &a);
    let v = read.float(0, 0.0);
    let form = match read.form(1) {
        0 if a.this_obj != 0 => base_form(e, a.this_obj),
        0 => return handled(true),
        form => form,
    };
    let ammo = dynamic_cast(e, form, rtti::TES_FORM, rtti::TES_AMMO);
    if ammo != 0 {
        e.mem.set_f32(ammo + AMMO_PERCENT_CONSUMED, v);
    }
    handled(true)
}

/// `BGSListForm::GetIndexOf` (xNVSE): whether a form with `form`'s id is in the list (`+0x18`,
/// nodes of data and next).
fn list_contains(e: &Engine, list: u32, form: u32) -> bool {
    let id = ref_id(e, form);
    let mut node = list + 0x18;
    while node != 0 {
        let item = e.mem.u32(node);
        if item != 0 && ref_id(e, item) == id {
            return true;
        }
        node = e.mem.u32(node + 4);
    }
    false
}

/// `TESForm::IsActor_Runtime` (vtable `+0x10C`), `Actor::baseProcess` (`+0x68`),
/// `BaseProcess::GetWeaponInfo` (`+0x148`) and `GetAmmoInfo` (`+0x14C`) (xNVSE `GameForms.h`,
/// `GameProcess.h`); the weapon info's form at `+8`, the ammo info's ammo at `+8`.
const IS_ACTOR: u32 = 0x10c;
const BASE_PROCESS: u32 = 0x68;
const GET_WEAPON_INFO: u32 = 0x148;
const GET_AMMO_INFO: u32 = 0x14c;

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_HasAmmoEquipped_Eval (and Cmd_IsEquippedAmmoInList_Eval, the same).
fn has_ammo_equipped_eval(e: &mut Engine, w: &[u32]) -> Ret {
    let a = EvalArgs::new(w);
    e.mem.set_f64(a.result, 0.0);
    if a.this_obj == 0 || e.vcall(a.this_obj, IS_ACTOR, &args![]).u32() & 0xff == 0 {
        return handled(true);
    }
    let process = e.mem.u32(a.this_obj + BASE_PROCESS);
    if process == 0 {
        return handled(true);
    }
    let info = e.vcall(process, GET_AMMO_INFO, &args![]).u32();
    if info == 0 {
        return handled(true);
    }
    let equipped = e.mem.u32(info + 8);
    let ammo = dynamic_cast(e, a.arg1, rtti::TES_FORM, rtti::TES_AMMO);
    if ammo != 0 {
        e.mem
            .set_f64(a.result, f64::from(u8::from(equipped == ammo)));
    } else {
        let list = dynamic_cast(e, a.arg1, rtti::TES_FORM, rtti::BGS_LIST_FORM);
        if list != 0 {
            let v = list_contains(e, list, equipped);
            e.mem.set_f64(a.result, f64::from(u8::from(v)));
        }
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_HasAmmoEquipped_Execute (and Cmd_IsEquippedAmmoInList_Execute).
fn has_ammo_equipped(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = super::Read::of(e, &a);
    if !read.ok {
        return handled(true);
    }
    has_ammo_equipped_eval(e, &[a.this_obj, read.form(0), 0, a.result])
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetWeaponCanUseAmmo_Eval.
/// The weapon is the second parameter, else the subject's base form, taken as a weapon without
/// a check (`static_cast`): its ammo word (`+0xA8`) is compared.
fn weapon_can_use_ammo_eval(e: &mut Engine, w: &[u32]) -> Ret {
    let a = EvalArgs::new(w);
    e.mem.set_f64(a.result, 0.0);
    let weapon = if a.arg2 != 0 {
        a.arg2
    } else if a.this_obj != 0 {
        base_form(e, a.this_obj)
    } else {
        0
    };
    if weapon == 0 {
        return handled(true);
    }
    let ammo_or_list = a.arg1;
    let uses = e.mem.u32(weapon + super::weapons::field::AMMO);
    let mut can = uses == ammo_or_list;
    if !can {
        let list = dynamic_cast(e, uses, rtti::TES_FORM, rtti::BGS_LIST_FORM);
        if list != 0 && e.mem.u8(ammo_or_list + 4) == TYPE_AMMO {
            can = list_contains(e, list, ammo_or_list);
        }
    }
    e.mem.set_f64(a.result, f64::from(u8::from(can)));
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetWeaponCanUseAmmo_Execute: the weapon form as read (no reference's base form).
fn weapon_can_use_ammo(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = super::Read::of(e, &a);
    if !read.ok {
        return handled(true);
    }
    weapon_can_use_ammo_eval(e, &[a.this_obj, read.form(0), read.form(1), a.result])
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetEquippedWeaponCanUseAmmo_Eval (and Cmd_GetEquippedWeaponUsesAmmoList_Eval), with
// `Actor::GetEquippedWeapon` (`GameObjects.cpp`): the subject is taken as an actor.
fn equipped_weapon_can_use_ammo_eval(e: &mut Engine, w: &[u32]) -> Ret {
    let a = EvalArgs::new(w);
    e.mem.set_f64(a.result, 0.0);
    if a.this_obj == 0 {
        return handled(true);
    }
    let process = e.mem.u32(a.this_obj + BASE_PROCESS);
    let mut weapon = 0;
    if process != 0 {
        let info = e.vcall(process, GET_WEAPON_INFO, &args![]).u32();
        if info != 0 {
            weapon = e.mem.u32(info + 8);
        }
    }
    weapon_can_use_ammo_eval(e, &[0, a.arg1, weapon, a.result])
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Inventory.cpp:
// Cmd_GetEquippedWeaponCanUseAmmo_Execute (and Cmd_GetEquippedWeaponUsesAmmoList_Execute).
fn equipped_weapon_can_use_ammo(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = super::Read::of(e, &a);
    if !read.ok {
        return handled(true);
    }
    equipped_weapon_can_use_ammo_eval(e, &[a.this_obj, read.form(0), 0, a.result])
}

#[cfg(test)]
mod tests {
    use super::super::test_support::engine;
    use super::*;

    const DATA: u32 = 0x0200_0000;
    const OFFSET: u32 = 0x0200_0100;
    const RESULT: u32 = 0x0200_0108;
    const SCRIPT: u32 = 0x0200_1000;
    const LIST: u32 = 0x0200_2000;
    /// A weapon base form (weight, health, value; type 0x28) and a reference to it.
    const WEAPON: u32 = 0x0200_3000;
    const REFERENCE: u32 = 0x0200_4000;
    const AMMO: u32 = 0x0200_5000;

    /// An engine with the table installed (the command's record holds its parameters), a weapon,
    /// ammo, a reference to the weapon and the script's reference list naming the weapon (1),
    /// the reference (2) and the ammo (3).
    fn setup() -> Engine {
        let mut e = engine();
        e.map(0x0118_e000, 0x9000);
        crate::table::install(&mut e);
        e.mem.set_u32(WEAPON + 0x7f0, 2 | 4 | 8);
        e.mem.set_u8(WEAPON + 4, 0x28);
        // Its weight, health and value components where the fake cast places them.
        e.mem.set_f32(WEAPON + 0x100 + WEIGHT, 7.5);
        e.mem.set_u32(WEAPON + 0x200 + HEALTH, 250);
        e.mem.set_u32(WEAPON + 0x300 + VALUE, 120);
        e.mem.set_u32(REFERENCE + 0x7f0, 1);
        e.mem.set_u8(REFERENCE + 4, 0x3a);
        e.mem.set_u32(REFERENCE + 0x20, WEAPON);
        e.mem.set_u32(AMMO + 0x7f0, 16);
        e.mem.set_u8(AMMO + 4, 0x29);
        e.mem.set_f32(AMMO + AMMO_WEIGHT, 0.25);
        // Reference list nodes: {RefVariable, next}, each RefVariable's form at +8.
        for (i, form) in [WEAPON, REFERENCE, AMMO].iter().enumerate() {
            let ref_var = SCRIPT + 0x100 + 0x10 * i as u32;
            e.mem.set_u32(ref_var + 8, *form);
            let node = if i == 0 {
                SCRIPT + 0x44
            } else {
                SCRIPT + 0x200 + 8 * i as u32
            };
            e.mem.set_u32(node, ref_var);
            if i < 2 {
                e.mem.set_u32(node + 4, SCRIPT + 0x200 + 8 * (i as u32 + 1));
            }
        }
        e
    }

    /// Calls `opcode` through its record as `EvaluateLine` does, with no argument (`None`) or
    /// the script's reference `n`.
    fn run(e: &mut Engine, opcode: u32, this_obj: u32, reference: Option<u16>) -> f64 {
        e.mem.set_u16(DATA, opcode as u16);
        match reference {
            None => e.mem.set_u16(DATA + 4, 0),
            Some(n) => {
                e.mem.set_u16(DATA + 4, 1);
                e.mem.set_u8(DATA + 6, b'r');
                e.mem.set_u16(DATA + 7, n);
            }
        }
        e.mem.set_u32(OFFSET, 4);
        e.mem.set_f64(RESULT, 99.0);
        let record = crate::table::record_of(e, opcode);
        let params = e.mem.u32(record + 0x14);
        let execute = e.mem.u32(record + 0x18);
        let ok = e
            .call(
                execute,
                &[params, DATA, this_obj, 0, SCRIPT, LIST, RESULT, OFFSET],
            )
            .eax;
        assert_eq!(ok, 1);
        e.mem.f64(RESULT)
    }

    fn eval(e: &mut Engine, opcode: u32, this_obj: u32, arg1: u32) -> f64 {
        let record = crate::table::record_of(e, opcode);
        let eval = e.mem.u32(record + 0x20);
        assert_eq!(e.mem.u32(record + 0x24), 1, "a condition-capable record");
        e.mem.set_f64(RESULT, 99.0);
        e.call(eval, &[this_obj, arg1, 0, RESULT]);
        e.mem.f64(RESULT)
    }

    #[test]
    fn get_weight_reads_the_form_the_reference_or_ammo() {
        let mut e = setup();
        assert_eq!(run(&mut e, 0x1404, 0, Some(1)), 7.5);
        // A reference is not an inventory object: the read fails (`kInventoryType`).
        assert_eq!(run(&mut e, 0x1404, 0, Some(2)), 0.0);
        // No argument: the calling reference's base form.
        assert_eq!(run(&mut e, 0x1404, REFERENCE, None), 7.5);
        assert_eq!(run(&mut e, 0x1404, 0, Some(3)), 0.25);
        // Nothing: 0.
        assert_eq!(run(&mut e, 0x1404, 0, None), 0.0);
        assert_eq!(eval(&mut e, 0x1404, 0, WEAPON), 7.5);
        assert_eq!(eval(&mut e, 0x1404, REFERENCE, 0), 7.5);
        assert_eq!(eval(&mut e, 0x1404, 0, 0), 0.0);
    }

    #[test]
    fn get_health_value_and_type() {
        let mut e = setup();
        assert_eq!(run(&mut e, 0x1405, 0, Some(1)), 250.0);
        assert_eq!(run(&mut e, 0x1406, REFERENCE, None), 120.0);
        assert_eq!(run(&mut e, 0x140a, 0, Some(1)), 40.0);
        assert_eq!(run(&mut e, 0x140a, 0, Some(2)), 40.0);
        // Ammo has no health or value component: 0.
        assert_eq!(run(&mut e, 0x1405, 0, Some(3)), 0.0);
        assert_eq!(run(&mut e, 0x1406, 0, Some(3)), 0.0);
        // The evals: health and type read only the subject.
        assert_eq!(eval(&mut e, 0x1405, REFERENCE, WEAPON), 250.0);
        assert_eq!(eval(&mut e, 0x1405, 0, WEAPON), 0.0);
        assert_eq!(eval(&mut e, 0x1406, 0, WEAPON), 120.0);
        assert_eq!(eval(&mut e, 0x140a, REFERENCE, 0), 40.0);
    }

    #[test]
    fn get_equip_type_and_clip_rounds() {
        let mut e = setup();
        e.mem
            .set_u32(WEAPON + 0x7f0, e.mem.u32(WEAPON + 0x7f0) | 32 | 64);
        e.mem.set_u32(WEAPON + 0x400 + EQUIP_TYPE, 3);
        e.mem.set_u32(WEAPON + 0x500 + CLIP_ROUNDS, 0xffff_ff1e);
        assert_eq!(run(&mut e, 0x140c, 0, Some(1)), 3.0);
        assert_eq!(run(&mut e, 0x140c, REFERENCE, None), 3.0);
        // The clip size is one byte.
        assert_eq!(run(&mut e, 0x140e, 0, Some(1)), 30.0);
        assert_eq!(run(&mut e, 0x140e, 0, Some(3)), 0.0);
        assert_eq!(eval(&mut e, 0x140c, 0, WEAPON), 3.0);
        assert_eq!(eval(&mut e, 0x140e, REFERENCE, 0), 30.0);
        assert_eq!(eval(&mut e, 0x140e, 0, 0), 0.0);
        // The attack damage is 16 bits.
        e.mem
            .set_u32(WEAPON + 0x7f0, e.mem.u32(WEAPON + 0x7f0) | 128);
        e.mem.set_u32(WEAPON + 0x600 + DAMAGE, 0x0007_002a);
        assert_eq!(run(&mut e, 0x140f, 0, Some(1)), 42.0);
        assert_eq!(eval(&mut e, 0x140f, REFERENCE, 0), 42.0);
    }

    /// A read that fails (a Cell for an inventory object) leaves the result 0 and the line goes
    /// on.
    #[test]
    fn a_failed_read_answers_zero() {
        let mut e = setup();
        e.mem.set_u8(WEAPON + 4, 0x39);
        assert_eq!(run(&mut e, 0x1404, REFERENCE, Some(1)), 0.0);
    }
}

#[cfg(test)]
mod armor_ammo_tests {
    use super::super::test_support::*;
    use super::*;

    const ARMOR: u32 = 0x0200_8000;
    const AMMO: u32 = 0x0200_9000;
    const CASING: u32 = 0x0200_9800;
    const LIST: u32 = 0x0200_a000;
    const WEAPON: u32 = 0x0200_b000;
    const ACTOR: u32 = 0x0200_c000;
    const PROCESS: u32 = 0x0200_c800;
    const INFO: u32 = 0x0200_cc00;
    const VTABLE: u32 = 0x0200_d000;
    /// Fake virtual functions: `IsActor_Runtime` true, `GetAmmoInfo`/`GetWeaponInfo` the info
    /// block, `IsQuestItem` the flag 0x400 and `SetQuestItem` as the exe's (`00401190`,
    /// `00484860`).
    const FN_TRUE: u32 = 0x0060_0000;
    const FN_INFO: u32 = 0x0060_0004;
    const FN_IS_QUEST: u32 = 0x0060_0008;
    const FN_SET_QUEST: u32 = 0x0060_000c;

    fn setup() -> Engine {
        let mut e = engine_with_table();
        e.register(FN_TRUE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(FN_INFO, |_, _| Ret {
            eax: INFO,
            ..Ret::default()
        });
        e.register(FN_IS_QUEST, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[0] + 8) & 0x400 != 0),
            ..Ret::default()
        });
        e.register(FN_SET_QUEST, |e, a| {
            let flags = e.mem.u32(a[0] + 8);
            let flags = if a[1] & 0xff != 0 {
                flags | 0x400
            } else {
                flags & !0x400
            };
            e.mem.set_u32(a[0] + 8, flags);
            Ret::default()
        });
        e.mem.set_u32(VTABLE + IS_ACTOR, FN_TRUE);
        e.mem.set_u32(VTABLE + GET_AMMO_INFO, FN_INFO);
        e.mem.set_u32(VTABLE + GET_WEAPON_INFO, FN_INFO);
        e.mem.set_u32(VTABLE + IS_QUEST_ITEM, FN_IS_QUEST);
        e.mem.set_u32(VTABLE + SET_QUEST_ITEM, FN_SET_QUEST);
        // T-45d power armor as the real game read it: DT 22, AR 0, power armor.
        e.mem.set_u32(ARMOR, VTABLE);
        e.mem.set_u32(ARMOR + 0x7f0, 512 | 1024);
        e.mem.set_u8(ARMOR + 4, 0x18);
        e.mem.set_f32(ARMOR + DAMAGE_THRESHOLD, 22.0);
        e.mem.set_u8(ARMOR + 0x700 + BIPED_FLAGS, 0x20 | 0x80);
        // 9mm rounds: speed 1, 25 percent, a casing (00140ADD).
        e.mem.set_u32(AMMO + 0x7f0, 16);
        e.mem.set_u8(AMMO + 4, TYPE_AMMO);
        e.mem.set_u32(AMMO + 0xc, 0x0015_37e7);
        e.mem.set_f32(AMMO + AMMO_SPEED, 1.0);
        e.mem.set_f32(AMMO + AMMO_PERCENT_CONSUMED, 25.0);
        e.mem.set_u32(AMMO + AMMO_CASING, CASING);
        e.mem.set_u32(CASING + 0xc, 0x0014_0add);
        // A form list holding the ammo; a weapon using the list; an actor whose process has the
        // weapon and the ammo in its info blocks.
        e.mem.set_u32(LIST + 0x7f0, 16384);
        e.mem.set_u8(LIST + 4, 0x55);
        e.mem.set_u32(LIST + 0x18, AMMO);
        e.mem
            .set_u32(WEAPON + super::super::weapons::field::AMMO, LIST);
        e.mem.set_u32(ACTOR, VTABLE);
        e.mem.set_u32(ACTOR + BASE_PROCESS, PROCESS);
        e.mem.set_u32(PROCESS, VTABLE);
        e.mem.set_u32(INFO + 8, AMMO);
        set_script_ref(&mut e, 1, ARMOR);
        set_script_ref(&mut e, 2, AMMO);
        set_script_ref(&mut e, 3, LIST);
        e
    }

    #[test]
    fn armor_getters_setters_and_power_armor() {
        let mut e = setup();
        assert_eq!(call(&mut e, 0x14b3, 0, &[r(1)]).1, 22.0);
        assert_eq!(call(&mut e, 0x1478, 0, &[r(1)]).1, 0.0);
        assert_eq!(call(&mut e, 0x1479, 0, &[r(1)]).1, 1.0);
        assert_eq!(eval(&mut e, 0x14b3, 0, ARMOR, 0), 22.0);
        call(&mut e, 0x14b4, 0, &[n(0x1_0007), r(1)]);
        assert_eq!(e.mem.u16(ARMOR + ARMOR_RATING), 7, "16 bits");
        call(&mut e, 0x14b5, 0, &[z(12.5), r(1)]);
        assert_eq!(e.mem.f32(ARMOR + DAMAGE_THRESHOLD), 12.5);
        call(&mut e, 0x147a, 0, &[n(0), r(1)]);
        assert_eq!(e.mem.u8(ARMOR + 0x700 + BIPED_FLAGS), 0x80);
        assert_eq!(eval(&mut e, 0x1479, 0, ARMOR, 0), 0.0);
    }

    #[test]
    fn quest_items_go_through_the_forms_virtual_functions() {
        let mut e = setup();
        assert_eq!(call(&mut e, 0x147c, 0, &[r(1)]).1, 0.0);
        call(&mut e, 0x147d, 0, &[n(1), r(1)]);
        assert_eq!(e.mem.u32(ARMOR + 8) & 0x400, 0x400);
        assert_eq!(eval(&mut e, 0x147c, 0, ARMOR, 0), 1.0);
        // Only exactly 1 sets it.
        call(&mut e, 0x147d, 0, &[n(2), r(1)]);
        assert_eq!(e.mem.u32(ARMOR + 8) & 0x400, 0);
    }

    #[test]
    fn ammo_getters_and_the_casing() {
        let mut e = setup();
        assert_eq!(call(&mut e, 0x14c2, 0, &[r(2)]).1, 1.0);
        assert_eq!(call(&mut e, 0x14c3, 0, &[r(2)]).1, 25.0);
        assert_eq!(
            call(&mut e, 0x14c4, 0, &[r(2)]).1.to_bits() as u32,
            0x0014_0add
        );
        call(&mut e, 0x1660, 0, &[z(35.0), r(2)]);
        assert_eq!(eval(&mut e, 0x14c3, 0, AMMO, 0), 35.0);
    }

    /// The equipped ammo and weapon, through the actor's process.
    #[test]
    fn equipped_ammo_and_weapons_that_can_use_it() {
        let mut e = setup();
        assert_eq!(eval(&mut e, 0x1657, ACTOR, AMMO, 0), 1.0);
        assert_eq!(eval(&mut e, 0x1659, ACTOR, LIST, 0), 1.0);
        assert_eq!(eval(&mut e, 0x1657, ACTOR, ARMOR, 0), 0.0);
        // The weapon uses a list holding the ammo.
        assert_eq!(eval(&mut e, 0x165f, 0, AMMO, WEAPON), 1.0);
        assert_eq!(eval(&mut e, 0x165f, 0, LIST, WEAPON), 1.0);
        // The equipped weapon is the weapon info block's form.
        e.mem.set_u32(INFO + 8, WEAPON);
        assert_eq!(eval(&mut e, 0x1658, ACTOR, AMMO, 0), 1.0);
    }
}
