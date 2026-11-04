//! `fallout shared/run_00519cb0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00519cc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00519cc0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x67 (0xb0 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0102_b5ac`) is `TESObjectIMOD`, so this is the constructor of `TESObjectIMOD`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00519cc0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x3c]);
    e.call(0x0040_2c20, &args![base + 0x5c]);
    e.call(0x0048_cc50, &args![base + 0x68]);
    e.call(0x0048_2c50, &args![base + 0x74]);
    e.call(0x0048_e7a0, &args![base + 0x7c]);
    e.call(0x0048_eac0, &args![base + 0x84]);
    e.call(0x0047_5350, &args![base + 0x8c]);
    e.call(0x0047_a130, &args![base + 0x94]);
    e.call(0x0047_b4f0, &args![base + 0xa4]);
    e.mem.set_u32(base, 0x0102_b5ac);
    e.mem.set_u32(base + 0x30, 0x0102_b594);
    e.mem.set_u32(base + 0x3c, 0x0102_b570);
    e.mem.set_u32(base + 0x5c, 0x0102_b550);
    e.mem.set_u32(base + 0x68, 0x0102_b53c);
    e.mem.set_u32(base + 0x74, 0x0102_b524);
    e.mem.set_u32(base + 0x7c, 0x0102_b504);
    e.mem.set_u32(base + 0x84, 0x0102_b4f0);
    e.mem.set_u32(base + 0x8c, 0x0102_b4dc);
    e.mem.set_u32(base + 0x94, 0x0102_b4c8);
    e.mem.set_u32(base + 0xa4, 0x0102_b4b4);
    e.call(0x004f_15a0, &args![this, 0x67u32]);
    e.call(0x0048_4ab0, &args![this, 0x1u32]);
    this
}

// Translated from 00519cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x3c (`SUB ECX,0x3c`) and jumps to `005174d0`. `005174d0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00519cb0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0051_74d0, &args![this.addr().wrapping_sub(0x3c), flags])
        .u32()
}

// Translated from 00519e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00519e70` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00519e40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0051_9e70, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The vtables `00519e70` stores (offset, vtable), in its order.
const DESTRUCTOR_VTABLES_00519E70: [(u32, u32); 11] = [
    (0, 0x0102_b5ac),
    (0x30, 0x0102_b594),
    (0x3c, 0x0102_b570),
    (0x5c, 0x0102_b550),
    (0x68, 0x0102_b53c),
    (0x74, 0x0102_b524),
    (0x7c, 0x0102_b504),
    (0x84, 0x0102_b4f0),
    (0x8c, 0x0102_b4dc),
    (0x94, 0x0102_b4c8),
    (0xa4, 0x0102_b4b4),
];
/// The member and base destructors `00519e70` runs (`thiscall`, no stack word), with the
/// offset each is called on, in its order.
const DESTRUCTOR_STEPS_00519E70: [(u32, u32); 10] = [
    (0x0048_3710, 0),
    (0x0047_b520, 0xa4),
    (0x0047_a1a0, 0x94),
    (0x0047_5380, 0x8c),
    (0x0048_eaf0, 0x84),
    (0x0048_e7d0, 0x7c),
    (0x0040_2c40, 0x5c),
    (0x0048_a490, 0x3c),
    (0x0059_1300, 0x30),
    (0x0050_e9f0, 0),
];

// Translated from 00519e70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00519e70 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef49bc, state variable) is not translated
/// Unnamed in the engine map (`thiscall`, no stack word): the destructor of the object
/// `fn_00519cc0` builds. Sets the eleven vtables of [`DESTRUCTOR_VTABLES_00519E70`], then runs
/// `00483710` on `this`, the member destructors on +0xa4, +0x94, +0x8c, +0x84, +0x7c, +0x5c,
/// +0x3c and +0x30, and the base destructor `0050e9f0` on `this`
/// ([`DESTRUCTOR_STEPS_00519E70`]).
pub fn fn_00519e70(e: &mut Engine, this: Ptr) {
    for (offset, vtable) in DESTRUCTOR_VTABLES_00519E70 {
        e.mem.set_u32(this.addr() + offset, vtable);
    }
    for (destructor, offset) in DESTRUCTOR_STEPS_00519E70 {
        e.call(destructor, &args![this.addr() + offset]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_9cc0, fn_00519cc0(Ptr) -> Ptr),
        entry!(0x0051_9cb0, fn_00519cb0(Ptr, u32) -> u32),
        entry!(0x0051_9e40, fn_00519e40(Ptr, u32) -> Ptr),
        entry!(0x0051_9e70, fn_00519e70(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00519cc0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_9cc0,
            size: 0xb0,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x3c)]),
                (0x0040_2c20, &[A::T(0x5c)]),
                (0x0048_cc50, &[A::T(0x68)]),
                (0x0048_2c50, &[A::T(0x74)]),
                (0x0048_e7a0, &[A::T(0x7c)]),
                (0x0048_eac0, &[A::T(0x84)]),
                (0x0047_5350, &[A::T(0x8c)]),
                (0x0047_a130, &[A::T(0x94)]),
                (0x0047_b4f0, &[A::T(0xa4)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x67)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_b5ac),
                (0x30, 4, 0x0102_b594),
                (0x3c, 4, 0x0102_b570),
                (0x5c, 4, 0x0102_b550),
                (0x68, 4, 0x0102_b53c),
                (0x74, 4, 0x0102_b524),
                (0x7c, 4, 0x0102_b504),
                (0x84, 4, 0x0102_b4f0),
                (0x8c, 4, 0x0102_b4dc),
                (0x94, 4, 0x0102_b4c8),
                (0xa4, 4, 0x0102_b4b4),
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
    fn le81_thunk_00519cb0() {
        assert_eq!(
            check(0x0051_9cb0, 0x0051_74d0, 0x3c, &[0x1]).u32(),
            0xdead_beef
        );
    }
    #[test]
    fn at_00519e40_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0051_9e70, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0051_9e40, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0051_9e40)
                .collect();
            let mut expected = vec![(0x0051_9e70, vec![this.addr()])];
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
    fn fn_00519e70_sets_the_vtables_and_destroys_the_members() {
        let mut e = Engine::new();
        for (destructor, _) in DESTRUCTOR_STEPS_00519E70 {
            e.register(destructor, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0xb0);
        e.call_log = Some(vec![]);
        e.call(0x0051_9e70, &args![this]);
        let log = e.call_log.take().unwrap();
        let expected: Vec<_> = DESTRUCTOR_STEPS_00519E70
            .iter()
            .map(|&(destructor, offset)| (destructor, vec![this + offset]))
            .collect();
        assert_eq!(log[1..], expected[..]);
        assert_eq!(e.mem.u32(this + 0x74), 0x0102_b524);
        assert_eq!(e.mem.u32(this + 0xa4), 0x0102_b4b4);
    }
}
