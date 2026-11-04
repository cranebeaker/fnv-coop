//! `fallout shared/run_004fe1d0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004fe260 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004fe260 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x21 (0x50 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_004fe260(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0048_a3d0, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0102_535c);
    e.mem.set_u32(base + 0x30, 0x0102_5334);
    e.call(0x004f_15a0, &args![this, 0x21]);
    this
}

// Translated from 004fe250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x3c (`SUB ECX,0x3c`) and jumps to `004fcf50`. `004fcf50` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_004fe250(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x004f_cf50, &args![this.addr().wrapping_sub(0x3c), flags])
        .u32()
}

// Translated from 004fe2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004fe310` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004fe2e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004f_e310, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The member and base destructors `004fe310` runs (`thiscall`, no stack word), with the
/// offset each is called on, in its order: `0052d240` (unit `tesimagespacemodifier.cpp`) on
/// `this`, `0048a490` (unit `tesmodeltextureswap.cpp`) on +0x30, and the base `0050e9f0`
/// (unit `tesobject.cpp`) on `this`.
const DESTRUCTOR_STEPS_004FE310: [(u32, u32); 3] =
    [(0x0052_d240, 0), (0x0048_a490, 0x30), (0x0050_e9f0, 0)];

// Translated from 004fe310 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004fe310 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef35c3, state variable) is not translated
/// Unnamed in the engine map (`thiscall`, no stack word): a destructor. Sets the vtables
/// `0102535c` (+0) and `01025334` (+0x30), then runs `0052d240` on `this`, `0048a490` on
/// +0x30 and the base destructor `0050e9f0` on `this`.
pub fn fn_004fe310(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0102_535c);
    e.mem.set_u32(this.addr() + 0x30, 0x0102_5334);
    for (destructor, offset) in DESTRUCTOR_STEPS_004FE310 {
        e.call(destructor, &args![this.addr() + offset]);
    }
}

// Translated from 004fe1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number the kind bits (`004fd4f0`, signed compares) map to: 0x10000 gives 0x3D,
/// 0x20000 0x3E, 0x40000 0x3F, 0x80000 0x40, 0x100000 0x69, anything else 0.
pub fn fn_004fe1d0(e: &mut Engine, this: Ptr) -> u32 {
    let kind = e.call(0x004f_d4f0, &args![this]).u32() as i32;
    if kind <= 0x40000 {
        match kind {
            0x40000 => 0x3f,
            0x10000 => 0x3d,
            0x20000 => 0x3e,
            _ => 0,
        }
    } else {
        match kind {
            0x80000 => 0x40,
            0x0010_0000 => 0x69,
            _ => 0,
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_e260, fn_004fe260(Ptr) -> Ptr),
        entry!(0x004f_e250, fn_004fe250(Ptr, u32) -> u32),
        entry!(0x004f_e2e0, fn_004fe2e0(Ptr, u32) -> Ptr),
        entry!(0x004f_e310, fn_004fe310(Ptr)),
        entry!(0x004f_e1d0, fn_004fe1d0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004fe260_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004f_e260,
            size: 0x50,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0048_a3d0, &[A::T(0x30)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x21)]),
            ],
            stores: &[(0x0, 4, 0x0102_535c), (0x30, 4, 0x0102_5334)],
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
    fn le81_thunk_004fe250() {
        assert_eq!(
            check(0x004f_e250, 0x004f_cf50, 0x3c, &[0x1]).u32(),
            0xdead_beef
        );
    }
    #[test]
    fn at_004fe2e0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_e310, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_e2e0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_e2e0)
                .collect();
            let mut expected = vec![(0x004f_e310, vec![this.addr()])];
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
    fn fn_004fe310_sets_the_vtables_and_destroys_the_members() {
        let mut e = Engine::new();
        for (destructor, _) in DESTRUCTOR_STEPS_004FE310 {
            e.register(destructor, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x004f_e310, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0052_d240, vec![this]),
                (0x0048_a490, vec![this + 0x30]),
                (0x0050_e9f0, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0102_535c);
        assert_eq!(e.mem.u32(this + 0x30), 0x0102_5334);
    }
}

#[cfg(test)]
mod n039_tests {
    use super::*;

    #[test]
    fn fn_004fe1d0_maps_the_kind() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        for (kind, expected) in [
            (0x10000u32, 0x3du32),
            (0x20000, 0x3e),
            (0x40000, 0x3f),
            (0x80000, 0x40),
            (0x0010_0000, 0x69),
            (0x30000, 0),
            (0, 0),
            (0x8000_0000, 0),
            (0x0018_0000, 0),
        ] {
            e.register_double(0x004f_d4f0, move |_, _| kind.into_ret());
            assert_eq!(
                e.call(0x004f_e1d0, &args![this]).u32(),
                expected,
                "{kind:x}"
            );
        }
    }
}
