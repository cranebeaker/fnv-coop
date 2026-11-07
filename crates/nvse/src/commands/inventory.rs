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

pub static COMMANDS: [Command; 7] = [
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
];

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
