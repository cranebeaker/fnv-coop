//! `fallout shared/run_004fe520` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004fe640 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004fe640 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x16 (0x98 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_004fe640(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0051_04f0, &args![this]);
    e.mem.set_u32(base, 0x0102_5594);
    e.mem.set_u32(base + 0x30, 0x0102_557c);
    e.mem.set_u32(base + 0x3c, 0x0102_5558);
    e.mem.set_u32(base + 0x5c, 0x0102_5544);
    e.mem.set_u32(base + 0x68, 0x0102_5530);
    e.mem.set_u32(base + 0x70, 0x0102_5520);
    e.call(0x004f_15a0, &args![this, 0x16]);
    e.call(0x004f_e7a0, &args![this]);
    this
}

// Translated from 004fe630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x30 (`SUB ECX,0x30`) and jumps to `004fe2e0`. `004fe2e0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_004fe630(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x004f_e2e0, &args![this.addr().wrapping_sub(0x30), flags])
        .u32()
}

// Translated from 004fe6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004fe710` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004fe6e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004f_e710, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004fe560 (decompiled, FalloutNV.exe 1.4.0.525)
/// An `InitItem` (the form behind the vtable `01025594`, RTTI
/// `.?AVBGSTalkingActivator@@`, stores the same component vtables): unless
/// form flag 0x8 is set (`004013e0`), `TESModelTextureSwap::InitItem` (Xbox
/// PDB, `0048aa80`) on `this + 0x30` with the form, then sets form flag 0x8
/// (`00484ab0`).
pub fn fn_004fe560(e: &mut Engine, this: Ptr) {
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    e.call(0x0048_aa80, &args![this.addr() + 0x30, this]);
    e.call(0x0048_4ab0, &args![this, true]);
}

// Translated from 004fe710 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004fe710 - the exception-unwinding frame (FS:[0]) is not translated.
/// The destructor body `004fe6e0` runs (vtable `01025594`, RTTI
/// `.?AVBGSTalkingActivator@@`): stores the object's and its components'
/// vtables as the constructor `004fe640` does, then runs `00483710` and
/// `00510640` on the object.
pub fn fn_004fe710(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0102_5594);
    e.mem.set_u32(base + 0x30, 0x0102_557c);
    e.mem.set_u32(base + 0x3c, 0x0102_5558);
    e.mem.set_u32(base + 0x5c, 0x0102_5544);
    e.mem.set_u32(base + 0x68, 0x0102_5530);
    e.mem.set_u32(base + 0x70, 0x0102_5520);
    e.call(0x0048_3710, &args![this]);
    e.call(0x0051_0640, &args![this]);
}

// Translated from 004fe7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start values of the constructor `004fe640`: clears the word at
/// `+0x94`, then runs `00510710` on the object.
pub fn fn_004fe7a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0x94, 0);
    e.call(0x0051_0710, &args![this]);
}

// Translated from 004fe520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the form (`ECX` only): `TESForm::StartForm`, the model at +0x30
/// (`TESModelTextureSwap::Save` with `MODL`, `MODT`, `MODS`), `TESForm::CloseForm`.
pub fn fn_004fe520(e: &mut Engine, this: Ptr) {
    e.call(0x0048_55a0, &args![this]);
    e.call(
        0x0048_a520,
        &args![
            this.addr() + 0x30,
            0x4c44_4f4du32,
            0x5444_4f4du32,
            0x5344_4f4du32
        ],
    );
    e.call(0x0048_5680, &args![this]);
}

// Translated from 004fe5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the components of `other` (`RET 4`, `TESForm::CopyAllComponents`) when it casts
/// (`__RTDynamicCast` from `01183028` to `01189ddc`).
pub fn fn_004fe5a0(e: &mut Engine, this: Ptr, other: u32) {
    let cast = e
        .call(
            0x00ec_43fb,
            &args![other, 0u32, 0x0118_3028u32, 0x0118_9ddcu32, 0u32],
        )
        .u32();
    if cast != 0 {
        e.call(0x0048_51b0, &args![this, cast]);
    }
}

// Translated from 004fe5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `other` differs (`RET 4`, `AL`): true unless it casts (`__RTDynamicCast` from
/// `01183028` to `01189ddc`) and `TESForm::CompareAllComponents` finds no difference.
pub fn fn_004fe5e0(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let cast = e
        .call(
            0x00ec_43fb,
            &args![other, 0u32, 0x0118_3028u32, 0x0118_9ddcu32, 0u32],
        )
        .u32();
    if cast == 0 {
        return true;
    }
    e.call(0x0048_5270, &args![this, cast]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_e640, fn_004fe640(Ptr) -> Ptr),
        entry!(0x004f_e630, fn_004fe630(Ptr, u32) -> u32),
        entry!(0x004f_e6e0, fn_004fe6e0(Ptr, u32) -> Ptr),
        entry!(0x004f_e560, fn_004fe560(Ptr)),
        entry!(0x004f_e710, fn_004fe710(Ptr)),
        entry!(0x004f_e7a0, fn_004fe7a0(Ptr)),
        entry!(0x004f_e520, fn_004fe520(Ptr)),
        entry!(0x004f_e5a0, fn_004fe5a0(Ptr, u32)),
        entry!(0x004f_e5e0, fn_004fe5e0(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod b023_tests {
    use super::*;

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    fn calls(e: &mut Engine, skip: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != skip)
            .collect()
    }

    #[test]
    fn init_item_once() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x0048_aa80, 0x0048_4ab0]);
        let this = e.mem.alloc(0x98);
        e.register(0x0040_13e0, |_, _| Ret {
            eax: 0x100,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x004f_e560, &[this]);
        // 0x100 is false in AL: the components run.
        assert_eq!(
            calls(&mut e, 0x004f_e560),
            vec![
                (0x0040_13e0, vec![this]),
                (0x0048_aa80, vec![this + 0x30, this]),
                (0x0048_4ab0, vec![this, 1]),
            ]
        );
        e.register(0x0040_13e0, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x004f_e560, &[this]);
        assert_eq!(calls(&mut e, 0x004f_e560), vec![(0x0040_13e0, vec![this])]);
    }

    #[test]
    fn destructor_body_and_start_values() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x0048_3710, 0x0051_0640, 0x0051_0710]);
        let this = e.mem.alloc(0x98);
        e.call_log = Some(vec![]);
        e.call(0x004f_e710, &[this]);
        assert_eq!(
            calls(&mut e, 0x004f_e710),
            vec![(0x0048_3710, vec![this]), (0x0051_0640, vec![this])]
        );
        for (offset, vtable) in [
            (0, 0x0102_5594),
            (0x30, 0x0102_557c),
            (0x3c, 0x0102_5558),
            (0x5c, 0x0102_5544),
            (0x68, 0x0102_5530),
            (0x70, 0x0102_5520),
        ] {
            assert_eq!(e.mem.u32(this + offset), vtable);
        }
        e.mem.set_u32(this + 0x94, 0xffff_ffff);
        e.call_log = Some(vec![]);
        e.call(0x004f_e7a0, &[this]);
        assert_eq!(calls(&mut e, 0x004f_e7a0), vec![(0x0051_0710, vec![this])]);
        assert_eq!(e.mem.u32(this + 0x94), 0);
    }
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004fe640_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004f_e640,
            size: 0x98,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0051_04f0, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x16)]),
                (0x004f_e7a0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_5594),
                (0x30, 4, 0x0102_557c),
                (0x3c, 4, 0x0102_5558),
                (0x5c, 4, 0x0102_5544),
                (0x68, 4, 0x0102_5530),
                (0x70, 4, 0x0102_5520),
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
    fn le81_thunk_004fe630() {
        assert_eq!(
            check(0x004f_e630, 0x004f_e2e0, 0x30, &[0x1]).u32(),
            0xdead_beef
        );
    }
    #[test]
    fn at_004fe6e0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_e710, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_e6e0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_e6e0)
                .collect();
            let mut expected = vec![(0x004f_e710, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod n039_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_004fe520_saves_the_model() {
        let mut e = Engine::new();
        for address in [0x0048_55a0, 0x0048_a520, 0x0048_5680] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x004f_e520, &args![0x1000u32]);
        let order: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (0x0048_55a0, vec![0x1000]),
                (
                    0x0048_a520,
                    vec![0x1030, 0x4c44_4f4d, 0x5444_4f4d, 0x5344_4f4d]
                ),
                (0x0048_5680, vec![0x1000])
            ]
        );
    }

    #[test]
    fn fn_004fe5a0_copies_only_a_cast_form() {
        let mut e = Engine::new();
        e.register(0x00ec_43fb, |_, a| a[0].into_ret());
        e.register(0x0048_51b0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004f_e5a0, &args![0x1000u32, 0x2000u32]);
        assert_eq!(calls_to(&e, 0x0048_51b0), vec![vec![0x1000, 0x2000]]);
        assert_eq!(
            calls_to(&e, 0x00ec_43fb),
            vec![vec![0x2000, 0, 0x0118_3028, 0x0118_9ddc, 0]]
        );
        e.register(0x00ec_43fb, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004f_e5a0, &args![0x1000u32, 0x2000u32]);
        assert!(calls_to(&e, 0x0048_51b0).is_empty());
    }

    #[test]
    fn fn_004fe5e0_compares_a_cast_form() {
        let mut e = Engine::new();
        e.register(0x00ec_43fb, |_, a| a[0].into_ret());
        e.register(0x0048_5270, |_, _| false.into_ret());
        assert!(!e.call(0x004f_e5e0, &args![0x1000u32, 0x2000u32]).bool());
        e.register(0x0048_5270, |_, _| 0x100u32.into_ret());
        assert!(!e.call(0x004f_e5e0, &args![0x1000u32, 0x2000u32]).bool());
        e.register(0x0048_5270, |_, _| true.into_ret());
        assert!(e.call(0x004f_e5e0, &args![0x1000u32, 0x2000u32]).bool());
        e.register(0x00ec_43fb, |_, _| Ret::default());
        assert!(e.call(0x004f_e5e0, &args![0x1000u32, 0x2000u32]).bool());
    }
}
