//! `fallout shared/run_0051a4c0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0051a6a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051a6a0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x1f (0xac bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0051a6a0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x3c]);
    e.call(0x0040_2c20, &args![base + 0x5c]);
    e.call(0x0048_cc50, &args![base + 0x68]);
    e.call(0x0048_e7a0, &args![base + 0x74]);
    e.call(0x0048_eac0, &args![base + 0x7c]);
    e.call(0x0047_5350, &args![base + 0x84]);
    e.call(0x0047_a130, &args![base + 0x8c]);
    e.call(0x0047_b4f0, &args![base + 0x9c]);
    e.mem.set_u32(base, 0x0102_b844);
    e.mem.set_u32(base + 0x30, 0x0102_b82c);
    e.mem.set_u32(base + 0x3c, 0x0102_b808);
    e.mem.set_u32(base + 0x5c, 0x0102_b7e8);
    e.mem.set_u32(base + 0x68, 0x0102_b7d4);
    e.mem.set_u32(base + 0x74, 0x0102_b7b4);
    e.mem.set_u32(base + 0x7c, 0x0102_b7a0);
    e.mem.set_u32(base + 0x84, 0x0102_b78c);
    e.mem.set_u32(base + 0x8c, 0x0102_b778);
    e.mem.set_u32(base + 0x9c, 0x0102_b764);
    e.call(0x004f_15a0, &args![this, 0x1f]);
    e.call(0x0051_a980, &args![this]);
    e.call(0x0048_4ab0, &args![this, 0x1]);
    this
}

// Translated from 0051a660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x7c (`SUB ECX,0x7c`) and jumps to `0051a550`. `0051a550` (`RET 0x4`) returns a 16-bit value in `AX` (it ends `MOV AX,word ptr [...]`), so the thunk returns `u16`; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_0051a660(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    e.call(
        0x0051_a550,
        &args![this.addr().wrapping_sub(0x7c), argument],
    )
    .u16()
}

// Translated from 0051a670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x7c (`SUB ECX,0x7c`) and jumps to `0051a5d0`. `0051a5d0` (`RET 0x8`) takes two stack words and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack words are passed on unchanged.
pub fn fn_0051a670(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(
        0x0051_a5d0,
        &args![this.addr().wrapping_sub(0x7c), first, second],
    );
}

// Translated from 0051a680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x3c (`SUB ECX,0x3c`) and jumps to `00519e40`. `00519e40` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0051a680(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0051_9e40, &args![this.addr().wrapping_sub(0x3c), flags])
        .u32()
}

// Translated from 0051a690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x7c (`SUB ECX,0x7c`) and jumps to `0051a5a0`. `0051a5a0` (`RET 0x4`) takes one stack word and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_0051a690(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(
        0x0051_a5a0,
        &args![this.addr().wrapping_sub(0x7c), argument],
    );
}

// Translated from 0051a980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Start value of the form `0051a6a0` builds (form type 0x1f; class unconfirmed): clears the word at `+0xa8`.
pub fn fn_0051a980(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0xa8, 0);
}

// Translated from 0051a810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0051a840` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0051a810(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0051_a840, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The vtables `0051a840` stores (offset, vtable), in its order.
const DESTRUCTOR_VTABLES_0051A840: [(u32, u32); 10] = [
    (0, 0x0102_b844),
    (0x30, 0x0102_b82c),
    (0x3c, 0x0102_b808),
    (0x5c, 0x0102_b7e8),
    (0x68, 0x0102_b7d4),
    (0x74, 0x0102_b7b4),
    (0x7c, 0x0102_b7a0),
    (0x84, 0x0102_b78c),
    (0x8c, 0x0102_b778),
    (0x9c, 0x0102_b764),
];
/// The member and base destructors `0051a840` runs (`thiscall`, no stack word), with the
/// offset each is called on, in its order.
const DESTRUCTOR_STEPS_0051A840: [(u32, u32); 10] = [
    (0x0048_3710, 0),
    (0x0047_b520, 0x9c),
    (0x0047_a1a0, 0x8c),
    (0x0047_5380, 0x84),
    (0x0048_eaf0, 0x7c),
    (0x0048_e7d0, 0x74),
    (0x0040_2c40, 0x5c),
    (0x0048_a490, 0x3c),
    (0x0059_1300, 0x30),
    (0x0050_e9f0, 0),
];

// Translated from 0051a840 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051a840 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef4a49, state variable) is not translated
/// Unnamed in the engine map (`thiscall`, no stack word): the destructor of the object
/// `fn_0051a6a0` builds. Sets the ten vtables of [`DESTRUCTOR_VTABLES_0051A840`], then runs
/// `00483710` on `this`, the member destructors on +0x9c, +0x8c, +0x84, +0x7c, +0x74, +0x5c,
/// +0x3c and +0x30, and the base destructor `0050e9f0` on `this`
/// ([`DESTRUCTOR_STEPS_0051A840`]).
pub fn fn_0051a840(e: &mut Engine, this: Ptr) {
    for (offset, vtable) in DESTRUCTOR_VTABLES_0051A840 {
        e.mem.set_u32(this.addr() + offset, vtable);
    }
    for (destructor, offset) in DESTRUCTOR_STEPS_0051A840 {
        e.call(destructor, &args![this.addr() + offset]);
    }
}

/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words) from `.?AVTESForm@@`
/// (`01183028`) to `.?AVTESObjectIMOD@@` (`0118a754`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_IMOD: u32 = 0x0118_a754;
/// `TESForm::CopyAllComponents` and `CompareAllComponents` (`004851b0`,
/// `00485270`, engine map; `thiscall`, one word).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;

/// `form` cast to `TESObjectIMOD`, or 0.
fn cast_to_imod(e: &mut Engine, form: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_IMOD, 0u32],
    )
    .u32()
}

// Translated from 0051a4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, one word, `RET 4`; a copy): when `source` casts
/// to `TESObjectIMOD`, `TESForm::CopyAllComponents(source)`.
pub fn fn_0051a4c0(e: &mut Engine, this: Ptr, source: u32) {
    if cast_to_imod(e, source) != 0 {
        e.call(COPY_ALL_COMPONENTS, &args![this, source]);
    }
}

// Translated from 0051a500 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, one word, `RET 4`): whether the form differs
/// from `form`: true when it does not cast to `TESObjectIMOD`, else what
/// `TESForm::CompareAllComponents` answers (as 1 or 0).
pub fn fn_0051a500(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let other = cast_to_imod(e, form);
    if other == 0 {
        return true;
    }
    e.call(COMPARE_ALL_COMPONENTS, &args![this, other]).bool()
}

// Translated from 0051a550 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (a save-size sum): the 16-bit sum of `00484bf0` on `this`
/// and `0048e9b0` on `this + 0x7C`, each given `argument`. `RET 4`.
pub fn fn_0051a550(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    let mut size: u16 = 0;
    size = size.wrapping_add(e.call(0x0048_4bf0, &args![this, argument]).u16());
    let component = this.addr().wrapping_add(0x7c);
    size.wrapping_add(e.call(0x0048_e9b0, &args![component, argument]).u16())
}

// Translated from 0051a5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `00484c20` on `this`, then `0048e9e0` on `this + 0x7C`,
/// each given `argument`. `RET 4`.
pub fn fn_0051a5a0(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(0x0048_4c20, &args![this, argument]);
    e.call(0x0048_e9e0, &args![this.addr() + 0x7c, argument]);
}

// Translated from 0051a5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `00484c50` on `this`, then `0048ea10` on `this + 0x7C`,
/// each given both stack words. `RET 8`.
pub fn fn_0051a5d0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(0x0048_4c50, &args![this, first, second]);
    e.call(0x0048_ea10, &args![this.addr() + 0x7c, first, second]);
}

// Translated from 0051a600 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `00484d60` on `this`, then `0048ea40` on `this + 0x7C`,
/// each given `argument`. `RET 4`.
pub fn fn_0051a600(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(0x0048_4d60, &args![this, argument]);
    e.call(0x0048_ea40, &args![this.addr() + 0x7c, argument]);
}

// Translated from 0051a630 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `00484da0` on `this`, then `0048ea80` on `this + 0x7C`,
/// each given `argument`. `RET 4`.
pub fn fn_0051a630(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(0x0048_4da0, &args![this, argument]);
    e.call(0x0048_ea80, &args![this.addr() + 0x7c, argument]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_a6a0, fn_0051a6a0(Ptr) -> Ptr),
        entry!(0x0051_a660, fn_0051a660(Ptr, u32) -> u16),
        entry!(0x0051_a670, fn_0051a670(Ptr, u32, u32)),
        entry!(0x0051_a680, fn_0051a680(Ptr, u32) -> u32),
        entry!(0x0051_a690, fn_0051a690(Ptr, u32)),
        entry!(0x0051_a980, fn_0051a980(Ptr)),
        entry!(0x0051_a810, fn_0051a810(Ptr, u32) -> Ptr),
        entry!(0x0051_a840, fn_0051a840(Ptr)),
        entry!(0x0051_a4c0, fn_0051a4c0(Ptr, u32)),
        entry!(0x0051_a500, fn_0051a500(Ptr, u32) -> bool),
        entry!(0x0051_a550, fn_0051a550(Ptr, u32) -> u16),
        entry!(0x0051_a5a0, fn_0051a5a0(Ptr, u32)),
        entry!(0x0051_a5d0, fn_0051a5d0(Ptr, u32, u32)),
        entry!(0x0051_a600, fn_0051a600(Ptr, u32)),
        entry!(0x0051_a630, fn_0051a630(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0051a6a0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_a6a0,
            size: 0xac,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x3c)]),
                (0x0040_2c20, &[A::T(0x5c)]),
                (0x0048_cc50, &[A::T(0x68)]),
                (0x0048_e7a0, &[A::T(0x74)]),
                (0x0048_eac0, &[A::T(0x7c)]),
                (0x0047_5350, &[A::T(0x84)]),
                (0x0047_a130, &[A::T(0x8c)]),
                (0x0047_b4f0, &[A::T(0x9c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x1f)]),
                (0x0051_a980, &[A::T(0x0)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_b844),
                (0x30, 4, 0x0102_b82c),
                (0x3c, 4, 0x0102_b808),
                (0x5c, 4, 0x0102_b7e8),
                (0x68, 4, 0x0102_b7d4),
                (0x74, 4, 0x0102_b7b4),
                (0x7c, 4, 0x0102_b7a0),
                (0x84, 4, 0x0102_b78c),
                (0x8c, 4, 0x0102_b778),
                (0x9c, 4, 0x0102_b764),
            ],
        });
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_0051a660() {
        assert_eq!(check(0x0051_a660, 0x0051_a550, 0x7c, &[0x77]).u16(), 0xbeef);
    }

    #[test]
    fn le81_thunk_0051a670() {
        check(0x0051_a670, 0x0051_a5d0, 0x7c, &[0x77, 0x88]);
    }

    #[test]
    fn le81_thunk_0051a680() {
        assert_eq!(
            check(0x0051_a680, 0x0051_9e40, 0x3c, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_0051a690() {
        check(0x0051_a690, 0x0051_a5a0, 0x7c, &[0x77]);
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;

    #[test]
    fn fn_0051a980_clears_the_word_at_0xa8() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb0);
        e.mem.write(this, &[0xcd; 0xb0]);
        e.call(0x0051_a980, &args![Ptr::<()>::new(this)]);
        let mut expected = vec![0xcd_u8; 0xb0];
        expected[0xa8..0xac].fill(0);
        assert_eq!(e.mem.bytes(this, 0xb0), expected);
    }
    #[test]
    fn at_0051a810_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0051_a840, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0051_a810, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0051_a810)
                .collect();
            let mut expected = vec![(0x0051_a840, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_0051a840_sets_the_vtables_and_destroys_the_members() {
        let mut e = Engine::new();
        for (destructor, _) in DESTRUCTOR_STEPS_0051A840 {
            e.register(destructor, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0xb0);
        e.call_log = Some(vec![]);
        e.call(0x0051_a840, &args![this]);
        let log = e.call_log.take().unwrap();
        let expected: Vec<_> = DESTRUCTOR_STEPS_0051A840
            .iter()
            .map(|&(destructor, offset)| (destructor, vec![this + offset]))
            .collect();
        assert_eq!(log[1..], expected[..]);
        assert_eq!(e.mem.u32(this + 0x9c), 0x0102_b764);
    }
}

#[cfg(test)]
mod n040_tests {
    use super::*;

    fn calls_to(e: &mut Engine, targets: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| targets.contains(a))
            .collect()
    }

    #[test]
    fn fn_0051a4c0_and_0051a500_copy_and_compare_item_mods() {
        let mut e = Engine::new();
        e.register(RT_DYNAMIC_CAST, |e, a| {
            assert_eq!(a[1..], [0, TYPE_TES_FORM, TYPE_TES_OBJECT_IMOD, 0]);
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
        e.mem.set_u32(other + 0xc, 1);
        e.call_log = Some(vec![]);
        e.call(0x0051_a4c0, &[this, other]);
        assert!(calls_to(&mut e, &[COPY_ALL_COMPONENTS]).is_empty());
        assert!(e.call(0x0051_a500, &[this, other]).bool());
        e.mem.set_u32(other + 0xc, 0);
        e.call_log = Some(vec![]);
        e.call(0x0051_a4c0, &[this, other]);
        assert_eq!(
            calls_to(&mut e, &[COPY_ALL_COMPONENTS]),
            vec![(COPY_ALL_COMPONENTS, vec![this, other])]
        );
        assert!(!e.call(0x0051_a500, &[this, other]).bool());
        e.mem.set_u32(other + 0x10, 1);
        assert!(e.call(0x0051_a500, &[this, other]).bool());
    }

    #[test]
    fn fn_0051a550_sums_in_16_bits() {
        let mut e = Engine::new();
        e.register(0x0048_4bf0, |_, _| Ret {
            eax: 0x8000,
            ..Ret::default()
        });
        e.register(0x0048_e9b0, |_, a| Ret {
            eax: 0x8000 + a[0] - 0x107c,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0051_a550, &[0x1000, 1]).u16(), 0);
    }

    #[test]
    fn fn_0051a5a0_and_friends_run_form_then_component() {
        let cases: [(u32, u32, u32, &[u32]); 4] = [
            (0x0051_a5a0, 0x0048_4c20, 0x0048_e9e0, &[7]),
            (0x0051_a5d0, 0x0048_4c50, 0x0048_ea10, &[7, 8]),
            (0x0051_a600, 0x0048_4d60, 0x0048_ea40, &[7]),
            (0x0051_a630, 0x0048_4da0, 0x0048_ea80, &[7]),
        ];
        for (addr, form, component, extra) in cases {
            let mut e = Engine::new();
            e.register(form, |_, _| Ret::default());
            e.register(component, |_, _| Ret::default());
            let mut words = vec![0x1000];
            words.extend_from_slice(extra);
            e.call_log = Some(vec![]);
            e.call(addr, &words);
            let mut moved = vec![0x107c];
            moved.extend_from_slice(extra);
            assert_eq!(
                calls_to(&mut e, &[form, component]),
                vec![(form, words.clone()), (component, moved)]
            );
        }
    }
}
