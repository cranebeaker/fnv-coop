//! `fallout shared/tesweightform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00401170` (unnamed in the engine map): `thiscall` getter, the form type byte at `this + 4`
/// (zero-extended).
const FORM_TYPE_BYTE: u32 = 0x0040_1170;
/// `0048ee50` (unnamed in the engine map): `thiscall` getter, the float at `this + 0xb8` in
/// `ST0` (the base weight of the form, read for the type 0x29).
const FORM_BASE_WEIGHT: u32 = 0x0048_ee50;
/// `006b9130` (unnamed in the engine map): `thiscall` getter, the float at `this + 4` in `ST0`.
const WEIGHT_FIELD_VALUE: u32 = 0x006b_9130;
/// The `double` `0.0` the weight is compared with.
const DOUBLE_ZERO: u32 = 0x0101_2060;
/// The `float` `-1.0` answered when no weight is found.
const FLOAT_MINUS_ONE: u32 = 0x0101_2054;

// Translated from 0048ebc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWeightForm::GetFormWeight` (Xbox PDB): `cdecl`, two stack words (the form, a flag byte),
/// the result a `float` in `ST0`. The form's type byte (`00401170`) selects where its weight
/// field lives: types 0x18 and 0x1a at `+0x60`, 0x1b at `+0x74`, 0x1d and 0x1e at `+0x84`, 0x1f,
/// 0x2e, 0x32 and 0x67 at `+0x7c`, 0x19 and 0x28 at `+0x8c`, 0x2f at `+0x94` (the exe's jump
/// table at `0048edd0`); type 0x29 takes the base weight `0048ee50` when the flag byte is
/// non-zero. A weight that is not equal to `0.0` (a NaN included) is answered as it is; otherwise
/// the weight field's value (`006b9130`) is answered when there is a field, else `-1.0`. Each
/// field address is `form + offset`; the exe's null test on `form` inside the cases can never be
/// true (the form is known to be non-null there) and is kept for fidelity.
pub fn tes_weight_form_get_form_weight(e: &mut Engine, form: Ptr, flag: u8) -> f32 {
    let mut weight_field = 0u32;
    let mut weight = 0.0f32;
    if !form.is_null() {
        let kind = e.call(FORM_TYPE_BYTE, &args![form]).u32();
        let field_at = |offset: u32| {
            if form.is_null() {
                0
            } else {
                form.addr().wrapping_add(offset)
            }
        };
        match kind {
            0x18 | 0x1a => weight_field = field_at(0x60),
            0x19 | 0x28 => weight_field = field_at(0x8c),
            0x1b => weight_field = field_at(0x74),
            0x1d | 0x1e => weight_field = field_at(0x84),
            0x1f | 0x2e | 0x32 | 0x67 => weight_field = field_at(0x7c),
            0x2f => weight_field = field_at(0x94),
            0x29 => {
                if flag != 0 {
                    // `FSTP float`: the float result is rounded to a float again.
                    weight =
                        f32::from_bits(x87_store32(e.call(FORM_BASE_WEIGHT, &args![form]).f64()));
                }
            }
            _ => {}
        }
    }
    let zero: f64 = e.global(DOUBLE_ZERO);
    if x87_load32(weight.to_bits()) == zero {
        if weight_field != 0 {
            e.call(WEIGHT_FIELD_VALUE, &args![weight_field]).f32()
        } else {
            x87_quiet(e.global::<f32>(FLOAT_MINUS_ONE))
        }
    } else {
        x87_quiet(weight)
    }
}

// Translated from 00910b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWeightForm::InitializeDataComponent` (Xbox PDB, matched by the
/// engine map): the weight (`float` at +4) 0 (`FLDZ`/`FSTP`).
pub fn tes_weight_form_initialize_data_component(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 4, x87_store32(0.0));
}

/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words) and the RTTI type
/// descriptors of `BaseFormComponent` (`01183040`) and `TESWeightForm`
/// (`01183274`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
const TYPE_TES_WEIGHT_FORM: u32 = 0x0118_3274;
/// `004f5d90` (thiscall, `RET 4`: a float): sets a weight form's weight.
const SET_WEIGHT: u32 = 0x004f_5d90;

/// The other component cast to `TESWeightForm`, or 0.
fn as_weight_form(e: &mut Engine, other: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![
            other,
            0u32,
            TYPE_BASE_FORM_COMPONENT,
            TYPE_TES_WEIGHT_FORM,
            0u32
        ],
    )
    .u32()
}

// Translated from 0048eb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWeightForm::CopyComponent` (Xbox PDB, `RET 4`: the other component):
/// when it is a `TESWeightForm`, sets this weight (`004f5d90`) to the other's
/// (`006b9130`, stored to the pushed float with `FSTP`).
pub fn tes_weight_form_copy_component(e: &mut Engine, this: Ptr, other: u32) {
    let source = as_weight_form(e, other);
    if source == 0 {
        return;
    }
    let weight = e.call(WEIGHT_FIELD_VALUE, &args![source]).f64();
    e.call(SET_WEIGHT, &args![this, x87_store32(weight)]);
}

// Translated from 0048eb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWeightForm::CompareComponent` (Xbox PDB, `RET 4`: the other
/// component): true ("different") unless the other component is a
/// `TESWeightForm` whose weight (`006b9130`) equals this one's (`FUCOMPP`:
/// unordered counts as different).
pub fn tes_weight_form_compare_component(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let source = as_weight_form(e, other);
    if source == 0 {
        return true;
    }
    let mine = e.call(WEIGHT_FIELD_VALUE, &args![this]).f64();
    let theirs = e.call(WEIGHT_FIELD_VALUE, &args![source]).f64();
    mine != theirs
}
/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_ebc0, tes_weight_form_get_form_weight(Ptr, u8) -> f32),
        entry!(0x00910b10, tes_weight_form_initialize_data_component(Ptr)),
        entry!(0x0048_eb10, tes_weight_form_copy_component(Ptr, u32)),
        entry!(0x0048_eb60, tes_weight_form_compare_component(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod b019_tests {
    use super::*;

    #[test]
    fn weight_starts_at_zero() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0xffff_ffff);
        e.call(0x0091_0b10, &args![this]);
        assert_eq!(e.mem.u32(this + 4), 0);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(DOUBLE_ZERO, 8);
        e.map(FLOAT_MINUS_ONE, 4);
        e.set_global(DOUBLE_ZERO, 0.0f64);
        e.set_global(FLOAT_MINUS_ONE, -1.0f32);
        // The form type byte is kept in the form's first word, for the test.
        e.register(FORM_TYPE_BYTE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(FORM_BASE_WEIGHT, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0] + 0xb8)),
            ..Ret::default()
        });
        e.register(WEIGHT_FIELD_VALUE, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0] + 4)),
            ..Ret::default()
        });
        e
    }

    fn form_of_type(e: &mut Engine, kind: u32) -> u32 {
        let form = e.mem.alloc(0xc0);
        e.mem.set_u32(form + 4, kind);
        form
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn at_0048ebc0_answers_minus_one_for_a_null_form() {
        let mut e = rig();
        assert_eq!(e.call(0x0048_ebc0, &args![0u32, 1u8]).f32(), -1.0);
        assert!(calls(&e, FORM_TYPE_BYTE).is_empty());
    }

    #[test]
    fn at_0048ebc0_reads_the_weight_field_each_type_selects() {
        let table = [
            (0x18u32, 0x60u32),
            (0x19, 0x8c),
            (0x1a, 0x60),
            (0x1b, 0x74),
            (0x1d, 0x84),
            (0x1e, 0x84),
            (0x1f, 0x7c),
            (0x28, 0x8c),
            (0x2e, 0x7c),
            (0x2f, 0x94),
            (0x32, 0x7c),
            (0x67, 0x7c),
        ];
        for (kind, offset) in table {
            let mut e = rig();
            let form = form_of_type(&mut e, kind);
            e.mem.set_f32(form + offset + 4, 12.5);
            assert_eq!(
                e.call(0x0048_ebc0, &args![form, 0u8]).f32(),
                12.5,
                "{kind:#x}"
            );
            assert_eq!(calls(&e, WEIGHT_FIELD_VALUE), vec![vec![form + offset]]);
            assert!(calls(&e, FORM_BASE_WEIGHT).is_empty());
        }
    }

    #[test]
    fn at_0048ebc0_other_types_have_no_weight_field() {
        for kind in [0u32, 0x17, 0x1c, 0x20, 0x2a, 0x30, 0x66, 0x68, 0xff] {
            let mut e = rig();
            let form = form_of_type(&mut e, kind);
            assert_eq!(
                e.call(0x0048_ebc0, &args![form, 1u8]).f32(),
                -1.0,
                "{kind:#x}"
            );
            assert!(calls(&e, WEIGHT_FIELD_VALUE).is_empty());
        }
    }

    #[test]
    fn at_0048ebc0_type_0x29_takes_the_base_weight_when_the_flag_is_set() {
        let mut e = rig();
        let form = form_of_type(&mut e, 0x29);
        e.mem.set_f32(form + 0xb8, 3.25);
        assert_eq!(e.call(0x0048_ebc0, &args![form, 1u8]).f32(), 3.25);
        assert_eq!(calls(&e, FORM_BASE_WEIGHT), vec![vec![form]]);
        // Without the flag the base weight is not read: no field either, so -1.0.
        let mut e = rig();
        let form = form_of_type(&mut e, 0x29);
        e.mem.set_f32(form + 0xb8, 3.25);
        assert_eq!(e.call(0x0048_ebc0, &args![form, 0u8]).f32(), -1.0);
        assert!(calls(&e, FORM_BASE_WEIGHT).is_empty());
        // A zero base weight falls through to "no field": -1.0.
        let mut e = rig();
        let form = form_of_type(&mut e, 0x29);
        assert_eq!(e.call(0x0048_ebc0, &args![form, 7u8]).f32(), -1.0);
    }

    #[test]
    fn at_0048ebc0_a_nan_weight_is_answered_as_it_is() {
        let mut e = rig();
        let form = form_of_type(&mut e, 0x29);
        e.mem.set_u32(form + 0xb8, 0x7fc0_1234);
        let result = e.call(0x0048_ebc0, &args![form, 1u8]).f32();
        assert!(result.is_nan());
        assert_eq!(result.to_bits(), 0x7fc0_1234);
    }

    #[test]
    fn at_0048ebc0_a_field_weight_of_zero_is_answered_as_zero() {
        // The field's own value is returned unchanged even when it is 0.0 or negative.
        let mut e = rig();
        let form = form_of_type(&mut e, 0x18);
        assert_eq!(e.call(0x0048_ebc0, &args![form, 0u8]).f32(), 0.0);
        e.mem.set_f32(form + 0x64, -2.0);
        assert_eq!(e.call(0x0048_ebc0, &args![form, 0u8]).f32(), -2.0);
    }
}

#[cfg(test)]
mod n045_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(WEIGHT_FIELD_VALUE, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0] + 4)),
            ..Ret::default()
        });
        e.register(SET_WEIGHT, |_, _| Ret::default());
        e
    }

    #[test]
    fn copy_passes_the_others_weight_as_a_float() {
        let mut e = engine();
        let other = e.mem.alloc(8);
        e.mem.set_f32(other + 4, 2.5);
        e.call_log = Some(vec![]);
        e.call(0x0048_eb10, &[0x5000, 0]);
        e.call(0x0048_eb10, &[0x5000, other]);
        let log = e.call_log.take().unwrap();
        let sets: Vec<_> = log.into_iter().filter(|(a, _)| *a == SET_WEIGHT).collect();
        assert_eq!(sets, vec![(SET_WEIGHT, vec![0x5000, 2.5f32.to_bits()])]);
    }

    #[test]
    fn compare_treats_unordered_weights_as_different() {
        let mut e = engine();
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        e.mem.set_f32(a + 4, 1.0);
        e.mem.set_f32(b + 4, 1.0);
        assert!(e.call(0x0048_eb60, &[a, 0]).bool());
        assert!(!e.call(0x0048_eb60, &[a, b]).bool());
        e.mem.set_f32(b + 4, 1.5);
        assert!(e.call(0x0048_eb60, &[a, b]).bool());
        e.mem.set_f32(a + 4, f32::NAN);
        e.mem.set_f32(b + 4, f32::NAN);
        assert!(e.call(0x0048_eb60, &[a, b]).bool());
    }
}
