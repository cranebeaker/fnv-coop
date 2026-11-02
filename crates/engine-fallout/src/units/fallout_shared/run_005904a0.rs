//! `fallout shared/run_005904a0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00590530 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00590530 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x62 (0x40 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_37c4`) is `BGSMessage`, so this is the constructor of `BGSMessage`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00590530(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0048_2c50, &args![base + 0x24]);
    e.mem.set_u32(base, 0x0103_37c4);
    e.mem.set_u32(base + 0x18, 0x0103_37ac);
    e.mem.set_u32(base + 0x24, 0x0103_3794);
    e.call(0x0096_a2d0, &args![base + 0x30]);
    e.call(0x004f_15a0, &args![this, 0x62u32]);
    e.call(0x0059_06a0, &args![this]);
    this
}

// Translated from 005905e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00590610` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005905e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0059_0610, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005906a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMessage`'s vtable (`010337c4`, RTTI `.?AVBGSMessage@@`) slot +0x14, the
/// `InitializeData` slot of the Xbox PDB's `TESForm` (`ECX`): the word at +0x2c cleared, +0x38
/// set to 1 and +0x3c to 2.
pub fn fn_005906a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0x2c, 0);
    e.mem.set_u32(this.addr() + 0x38, 1);
    e.mem.set_u32(this.addr() + 0x3c, 2);
}

// Translated from 005906d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMessage`'s vtable slot +0x18, the `ClearData` slot of the Xbox PDB's `TESForm`
/// (`ECX`): `0086cf00` with 0 (the word at +0x2c), then `005910d0` on `this`.
pub fn fn_005906d0(e: &mut Engine, this: Ptr) {
    e.call(0x0086_cf00, &args![this, 0u32]);
    e.call(0x0059_10d0, &args![this]);
}

// Translated from 00590610 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00590610 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef833e, state variable) is not translated
/// The `BGSMessage` destructor body (unnamed in the map; `ECX`, no stack words; the scalar
/// deleting destructor `005905e0` runs it): stores the three vtables (`010337c4` at +0,
/// `010337ac` at +0x18, `01033794` at +0x24), clears the data ([`fn_005906d0`]), then destroys
/// the list at +0x30 (`0046ffb0`), the component at +0x18 (`00591300`) and the `TESForm` base
/// (`TESForm::~TESForm`, `00483630`).
pub fn fn_00590610(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_37c4);
    e.mem.set_u32(base + 0x18, 0x0103_37ac);
    e.mem.set_u32(base + 0x24, 0x0103_3794);
    fn_005906d0(e, this);
    e.call(0x0046_ffb0, &args![base + 0x30]);
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

/// `__RTDynamicCast` (`00ec43fb`) from `.?AVTESForm@@` (`01183028`) to
/// `.?AVBGSMenuIcon@@` (`011863f0`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_MENU_ICON: u32 = 0x0118_63f0;
/// `TESForm::CopyAllComponents` and `CompareAllComponents` (`004851b0`,
/// `00485270`, engine map; `thiscall`, one word).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;

/// `form` cast to `BGSMenuIcon`, or 0.
fn cast_to_menu_icon(e: &mut Engine, form: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, TYPE_MENU_ICON, 0u32],
    )
    .u32()
}

// Translated from 005904a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMenuIcon`'s copy (by the RTTI it casts to; `thiscall`, one word,
/// `RET 4`): when `source` casts, `TESForm::CopyAllComponents` with the
/// cast form.
pub fn fn_005904a0(e: &mut Engine, this: Ptr, source: u32) {
    let other = cast_to_menu_icon(e, source);
    if other != 0 {
        e.call(COPY_ALL_COMPONENTS, &args![this, other]);
    }
}

// Translated from 005904e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMenuIcon`'s comparison (`thiscall`, one word, `RET 4`): true when
/// `form` does not cast, else what `TESForm::CompareAllComponents` answers
/// (as 1 or 0).
pub fn fn_005904e0(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let other = cast_to_menu_icon(e, form);
    if other == 0 {
        return true;
    }
    e.call(COMPARE_ALL_COMPONENTS, &args![this, other]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0059_0530, fn_00590530(Ptr) -> Ptr),
        entry!(0x0059_05e0, fn_005905e0(Ptr, u32) -> Ptr),
        entry!(0x0059_06a0, fn_005906a0(Ptr)),
        entry!(0x0059_06d0, fn_005906d0(Ptr)),
        entry!(0x0059_0610, fn_00590610(Ptr)),
        entry!(0x0059_04a0, fn_005904a0(Ptr, u32)),
        entry!(0x0059_04e0, fn_005904e0(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod message_tests {
    use super::*;

    fn order(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .skip(1)
            .cloned()
            .collect()
    }

    #[test]
    fn fn_005906a0_initializes_the_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        e.mem.write(this, &[0xaa; 0x40]);
        e.call(0x0059_06a0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x2c), 0);
        assert_eq!(e.mem.u32(this + 0x30), 0xaaaa_aaaa);
        assert_eq!(e.mem.u32(this + 0x38), 1);
        assert_eq!(e.mem.u32(this + 0x3c), 2);
    }

    #[test]
    fn fn_005906d0_clears_then_runs_005910d0() {
        let mut e = Engine::new();
        e.register(0x0086_cf00, |_, _| Ret::default());
        e.register(0x0059_10d0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0059_06d0, &args![0x5000u32]);
        assert_eq!(
            order(&e),
            vec![(0x0086_cf00, vec![0x5000, 0]), (0x0059_10d0, vec![0x5000])]
        );
    }

    #[test]
    fn fn_00590610_destroys_last_first() {
        let mut e = Engine::new();
        for addr in [
            0x0086_cf00,
            0x0059_10d0,
            0x0046_ffb0,
            0x0059_1300,
            0x0048_3630,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x0059_0610, &args![this]);
        assert_eq!(
            order(&e),
            vec![
                (0x0086_cf00, vec![this, 0]),
                (0x0059_10d0, vec![this]),
                (0x0046_ffb0, vec![this + 0x30]),
                (0x0059_1300, vec![this + 0x18]),
                (0x0048_3630, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0103_37c4);
        assert_eq!(e.mem.u32(this + 0x18), 0x0103_37ac);
        assert_eq!(e.mem.u32(this + 0x24), 0x0103_3794);
    }
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00590530_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_0530,
            size: 0x40,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0048_2c50, &[A::T(0x24)]),
                (0x0096_a2d0, &[A::T(0x30)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x62)]),
                (0x0059_06a0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0103_37c4),
                (0x18, 4, 0x0103_37ac),
                (0x24, 4, 0x0103_3794),
            ],
        });
    }
    #[test]
    fn at_005905e0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0059_0610, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0059_05e0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0059_05e0)
                .collect();
            let mut expected = vec![(0x0059_0610, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod n040_tests {
    use super::*;

    #[test]
    fn fn_005904a0_and_005904e0_copy_and_compare_menu_icons() {
        let mut e = Engine::new();
        e.register(RT_DYNAMIC_CAST, |e, a| {
            assert_eq!(a[1..], [0, TYPE_TES_FORM, TYPE_MENU_ICON, 0]);
            let form = if e.mem.u32(a[0] + 0xc) == 0 { a[0] } else { 0 };
            Ret {
                eax: form,
                ..Ret::default()
            }
        });
        e.register(COPY_ALL_COMPONENTS, |_, _| Ret::default());
        e.register(COMPARE_ALL_COMPONENTS, |e, a| Ret {
            eax: e.mem.u32(a[1] + 0x10),
            ..Ret::default()
        });
        let this = e.mem.alloc(0x20);
        let other = e.mem.alloc(0x20);
        let copies = |e: &mut Engine| -> usize {
            e.call_log
                .take()
                .unwrap()
                .iter()
                .filter(|(a, _)| *a == COPY_ALL_COMPONENTS)
                .count()
        };
        e.mem.set_u32(other + 0xc, 1);
        e.call_log = Some(vec![]);
        e.call(0x0059_04a0, &[this, other]);
        assert_eq!(copies(&mut e), 0);
        assert!(e.call(0x0059_04e0, &[this, other]).bool());
        e.mem.set_u32(other + 0xc, 0);
        e.call_log = Some(vec![]);
        e.call(0x0059_04a0, &[this, other]);
        assert_eq!(copies(&mut e), 1);
        assert!(!e.call(0x0059_04e0, &[this, other]).bool());
        e.mem.set_u32(other + 0x10, 1);
        assert!(e.call(0x0059_04e0, &[this, other]).bool());
    }
}
