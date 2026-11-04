//! `fallout shared/tesattackdamageform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `__RTDynamicCast` (engine map), `00ec43fb`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The RTTI type descriptors of `TESForm`, `BaseFormComponent` and
/// `TESAttackDamageForm`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
const TYPE_ATTACK_DAMAGE_FORM: u32 = 0x0118_6c90;
/// The component's virtual slot 0x10: its attack damage (`GetAttackDamage`).
const SLOT_GET_ATTACK_DAMAGE: u32 = 0x10;
/// `0047ff50` (`thiscall`, `RET 4`): stores the attack damage.
const SET_ATTACK_DAMAGE: u32 = 0x0047_ff50;

// Translated from 0047fe10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAttackDamageForm::InitializeDataComponent` (Xbox PDB, matched by the
/// engine map): the 16-bit attack damage (+4) 0.
pub fn tes_attack_damage_form_initialize_data_component(e: &mut Engine, this: Ptr) {
    e.mem.set_u16(this.addr() + 4, 0);
}

// Translated from 0047fe30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAttackDamageForm::CopyComponent` (Xbox PDB, matched by the engine
/// map): when `other` is a `TESAttackDamageForm` (`__RTDynamicCast` from
/// `BaseFormComponent`), its attack damage (virtual slot 0x10, 16 bits) is
/// stored here (`0047ff50`).
pub fn tes_attack_damage_form_copy_component(e: &mut Engine, this: Ptr, other: Ptr) {
    let source = e
        .call(
            DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_ATTACK_DAMAGE_FORM,
                0u32
            ],
        )
        .u32();
    if source != 0 {
        let damage = e.vcall(source, SLOT_GET_ATTACK_DAMAGE, &args![]).u16();
        e.call(SET_ATTACK_DAMAGE, &args![this, u32::from(damage)]);
    }
}

// Translated from 0047fef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The attack damage of a form (`cdecl`; no Xbox PDB name): its
/// `TESAttackDamageForm` part (`__RTDynamicCast` from `TESForm`) asked
/// through virtual slot 0x10 (its `EAX` is the answer), 0 without one.
pub fn fn_0047fef0(e: &mut Engine, form: Ptr) -> u32 {
    let part = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_ATTACK_DAMAGE_FORM, 0u32],
        )
        .u32();
    if part == 0 {
        0
    } else {
        e.vcall(part, SLOT_GET_ATTACK_DAMAGE, &args![]).u32()
    }
}

// Translated from 0047ff30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAttackDamageForm::GetAttackDamage` (Xbox PDB, matched by the engine
/// map): the 16-bit attack damage (+4), in `AX`.
pub fn tes_attack_damage_form_get_attack_damage(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + 4)
}

// Translated from 0047fe80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAttackDamageForm::CompareComponent` (Xbox PDB; `RET 4`): true when
/// `other` is not a `TESAttackDamageForm` (`__RTDynamicCast` from
/// `BaseFormComponent`) or when the two attack damages (virtual slot 0x10,
/// 16 bits) differ.
pub fn tes_attack_damage_form_compare_component(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_ATTACK_DAMAGE_FORM,
                0u32
            ],
        )
        .u32();
    if cast == 0 {
        return true;
    }
    let mine = e.vcall(this.addr(), SLOT_GET_ATTACK_DAMAGE, &args![]).u16();
    let theirs = e.vcall(cast, SLOT_GET_ATTACK_DAMAGE, &args![]).u16();
    mine != theirs
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0047fe10,
            tes_attack_damage_form_initialize_data_component(Ptr)
        ),
        entry!(0x0047fe30, tes_attack_damage_form_copy_component(Ptr, Ptr)),
        entry!(0x0047fef0, fn_0047fef0(Ptr) -> u32),
        entry!(
            0x0047ff30,
            tes_attack_damage_form_get_attack_damage(Ptr) -> u16
        ),
        entry!(
            0x0047_fe80,
            tes_attack_damage_form_compare_component(Ptr, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A component with the damage at +4 whose slot 0x10 is the real
    /// getter (`0047ff30`).
    fn component(e: &mut Engine, damage: u16) -> u32 {
        let part = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x14);
        e.mem.set_u32(part, vtable);
        e.mem.set_u32(vtable + SLOT_GET_ATTACK_DAMAGE, 0x0047_ff30);
        e.mem.set_u16(part + 4, damage);
        part
    }

    #[test]
    fn damage_starts_at_zero_and_reads_back() {
        let mut e = Engine::new();
        let this = component(&mut e, 0x1234);
        e.mem.set_u16(this + 6, 0xbeef);
        assert_eq!(e.call(0x0047_ff30, &args![this]).u32(), 0x1234);
        e.call(0x0047_fe10, &args![this]);
        assert_eq!(e.mem.u16(this + 4), 0);
        assert_eq!(e.mem.u16(this + 6), 0xbeef);
    }

    #[test]
    fn copy_takes_the_damage_of_an_attack_damage_form() {
        let mut e = Engine::new();
        let other = component(&mut e, 33);
        e.register_double(DYNAMIC_CAST, move |_, a| Ret {
            eax: if a[0] == other { other } else { 0 },
            ..Ret::default()
        });
        e.register(SET_ATTACK_DAMAGE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0047_fe30, &args![0x3000u32, other]);
        assert_eq!(calls(&e, SET_ATTACK_DAMAGE), vec![vec![0x3000, 33]]);
        assert_eq!(calls(&e, DYNAMIC_CAST)[0][2], TYPE_BASE_FORM_COMPONENT);
        e.call(0x0047_fe30, &args![0x3000u32, 0x4000u32]);
        assert_eq!(calls(&e, SET_ATTACK_DAMAGE).len(), 1);
    }

    #[test]
    fn form_damage_through_its_component() {
        let mut e = Engine::new();
        let part = component(&mut e, 12);
        e.register_double(DYNAMIC_CAST, move |_, a| Ret {
            eax: if a[0] == 0x5000 { part } else { 0 },
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_fef0, &args![0x5000u32]).u32(), 12);
        assert_eq!(calls(&e, DYNAMIC_CAST)[0][2], TYPE_TES_FORM);
        assert_eq!(e.call(0x0047_fef0, &args![0x6000u32]).u32(), 0);
    }
}

#[cfg(test)]
mod tests_n042 {
    use super::*;

    #[test]
    fn compare_component_compares_the_damages() {
        let mut e = Engine::new();
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(0x0f00_1000, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.put_vtable(0x0f00_0000, &[0, 0, 0, 0, 0x0f00_1000]);
        let this = e.mem.alloc(8);
        let other = e.mem.alloc(8);
        for p in [this, other] {
            e.mem.set_u32(p, 0x0f00_0000);
        }
        e.mem.set_u32(this + 4, 0x1_0007);
        e.mem.set_u32(other + 4, 0x2_0007);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0047_fe80, &args![this, other]).bool());
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(
            DYNAMIC_CAST,
            vec![
                other,
                0,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_ATTACK_DAMAGE_FORM,
                0
            ]
        )));
        e.mem.set_u32(other + 4, 8);
        assert!(e.call(0x0047_fe80, &args![this, other]).bool());
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        assert!(e.call(0x0047_fe80, &args![this, other]).bool());
    }
}
