//! `fallout shared/teshealthform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESHealthForm::GetFormAsHealthForm` (Xbox PDB, `004872e0`): `cdecl`, one stack word; the
/// form's health part, or null.
const GET_FORM_AS_HEALTH_FORM: u32 = 0x0048_72e0;
/// The virtual slot (byte offset) of the health part that answers the health.
const SLOT_GET_HEALTH: u32 = 0x10;

// Translated from 004873d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESHealthForm::GetFormHealth` (Xbox PDB): `cdecl`, one stack word. Zero when the form has no
/// health part (`GetFormAsHealthForm`); otherwise what the part's virtual slot +0x10 answers.
pub fn tes_health_form_get_form_health(e: &mut Engine, form: Ptr) -> u32 {
    let part = e.call(GET_FORM_AS_HEALTH_FORM, &args![form]).u32();
    if part == 0 {
        0
    } else {
        e.vcall(part, SLOT_GET_HEALTH, &args![]).u32()
    }
}

/// `__RTDynamicCast` (engine map), `00ec43fb`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The RTTI type descriptors of `BaseFormComponent` and `TESHealthForm`.
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
const TYPE_HEALTH_FORM: u32 = 0x0118_6c3c;
/// `006ecd40` (`thiscall`, `RET 4`; the engine map's
/// `CombatProcedureAttackMelee::Initialize` is folded code): stores the
/// health.
const SET_HEALTH: u32 = 0x006e_cd40;
/// `00401170` (`thiscall`): the form's type byte.
const FORM_TYPE: u32 = 0x0040_1170;

// Translated from 00487220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESHealthForm::InitializeDataComponent` (Xbox PDB, matched by the
/// engine map): the health (+4) 100.
pub fn tes_health_form_initialize_data_component(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 4, 100);
}

// Translated from 00487240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESHealthForm::CopyComponent` (Xbox PDB, matched by the engine map):
/// when `other` is a `TESHealthForm` (`__RTDynamicCast` from
/// `BaseFormComponent`), its health (+4) is stored here (`006ecd40`).
pub fn tes_health_form_copy_component(e: &mut Engine, this: Ptr, other: Ptr) {
    let source = e
        .call(
            DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_HEALTH_FORM,
                0u32
            ],
        )
        .u32();
    if source != 0 {
        let health = e.mem.u32(source + 4);
        e.call(SET_HEALTH, &args![this, health]);
    }
}

// Translated from 004872e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESHealthForm::GetFormAsHealthForm` (Xbox PDB, matched by the engine
/// map; `cdecl`): the health part of a form by its type (`00401170`; a
/// jump table over `type - 0x18`): +0x68 for 0x18, +0x94 for 0x28, +0xB0
/// for 0x2A and 0x2B; null for a null form or any other type.
pub fn tes_health_form_get_form_as_health_form(e: &mut Engine, form: Ptr) -> u32 {
    if form.addr() == 0 {
        return 0;
    }
    let part = |offset: u32| {
        if form.addr() != 0 {
            form.addr() + offset
        } else {
            0
        }
    };
    match e.call(FORM_TYPE, &args![form]).u32() {
        0x18 => part(0x68),
        0x28 => part(0x94),
        0x2a | 0x2b => part(0xb0),
        _ => 0,
    }
}

// Translated from 00487290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESHealthForm::CompareComponent` (Xbox PDB): true (different) when
/// `other` is no `TESHealthForm` or its word at +4 (the health) differs.
pub fn tes_health_form_compare_component(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let source = e
        .call(
            DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_HEALTH_FORM,
                0u32
            ],
        )
        .u32();
    if source == 0 {
        return true;
    }
    e.mem.u32(this.addr() + 4) != e.mem.u32(source + 4)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_73d0, tes_health_form_get_form_health(Ptr) -> u32),
        entry!(0x00487220, tes_health_form_initialize_data_component(Ptr)),
        entry!(0x00487240, tes_health_form_copy_component(Ptr, Ptr)),
        entry!(
            0x004872e0,
            tes_health_form_get_form_as_health_form(Ptr) -> u32
        ),
        entry!(0x0048_7290, tes_health_form_compare_component(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod b019_tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn health_starts_at_100() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.call(0x0048_7220, &args![this]);
        assert_eq!(e.mem.u32(this + 4), 100);
    }

    #[test]
    fn copy_takes_the_health_of_a_health_form_only() {
        let mut e = Engine::new();
        let other = e.mem.alloc(8);
        e.mem.set_u32(other + 4, 75);
        e.register_double(DYNAMIC_CAST, move |_, a| Ret {
            eax: if a[0] == other { other } else { 0 },
            ..Ret::default()
        });
        e.register(SET_HEALTH, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_7240, &args![0x3000u32, other]);
        assert_eq!(calls(&e, SET_HEALTH), vec![vec![0x3000, 75]]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST)[0],
            vec![other, 0, TYPE_BASE_FORM_COMPONENT, TYPE_HEALTH_FORM, 0]
        );
        e.call(0x0048_7240, &args![0x3000u32, 0x4000u32]);
        assert_eq!(calls(&e, SET_HEALTH).len(), 1);
    }

    #[test]
    fn health_part_by_form_type() {
        let mut e = Engine::new();
        e.register(FORM_TYPE, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        let form = e.mem.alloc(4);
        for (kind, offset) in [(0x18u32, 0x68u32), (0x28, 0x94), (0x2a, 0xb0), (0x2b, 0xb0)] {
            e.mem.set_u32(form, kind);
            assert_eq!(e.call(0x0048_72e0, &args![form]).u32(), form + offset);
        }
        for kind in [0x17u32, 0x19, 0x29, 0x2c, 0] {
            e.mem.set_u32(form, kind);
            assert_eq!(e.call(0x0048_72e0, &args![form]).u32(), 0);
        }
        assert_eq!(e.call(0x0048_72e0, &args![0u32]).u32(), 0);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_004873d0_asks_the_health_part_for_the_health() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let mut slots = [0u32; 8];
        slots[SLOT_GET_HEALTH as usize / 4] = 0x0f00_1000;
        e.put_vtable(0x0f00_1100, &slots);
        e.register(0x0f00_1000, |e, a| e.mem.u32(a[0] + 8).into_ret());
        let part = e.mem.alloc(0x10);
        e.mem.set_u32(part, 0x0f00_1100);
        e.mem.set_u32(part + 8, 250);
        e.register_double(GET_FORM_AS_HEALTH_FORM, move |_, _| part.into_ret());
        assert_eq!(e.call(0x0048_73d0, &args![0x4000u32]).u32(), 250);
        let asked: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == GET_FORM_AS_HEALTH_FORM)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(asked, vec![vec![0x4000]]);
    }

    #[test]
    fn at_004873d0_answers_zero_without_a_health_part() {
        let mut e = Engine::new();
        e.register(GET_FORM_AS_HEALTH_FORM, |_, _| Ret::default());
        assert_eq!(e.call(0x0048_73d0, &args![0x4000u32]).u32(), 0);
    }
}

#[cfg(test)]
mod n043_tests {
    use super::*;

    #[test]
    fn compare_component_compares_the_health() {
        let mut e = Engine::new();
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        let this = e.mem.alloc(8);
        let other = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 30);
        e.mem.set_u32(other + 4, 30);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0048_7290, &args![this, other]).bool());
        assert_eq!(
            e.call_log.as_ref().unwrap()[1],
            (
                DYNAMIC_CAST,
                vec![other, 0, TYPE_BASE_FORM_COMPONENT, TYPE_HEALTH_FORM, 0]
            )
        );
        e.mem.set_u32(other + 4, 31);
        assert!(e.call(0x0048_7290, &args![this, other]).bool());
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.mem.set_u32(other + 4, 30);
        assert!(e.call(0x0048_7290, &args![this, other]).bool());
    }
}
