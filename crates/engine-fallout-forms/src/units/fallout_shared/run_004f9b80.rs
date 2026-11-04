//! `fallout shared/run_004f9b80` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004f9cd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f9cd0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x52 (0x24 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_004f9cd0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x004f_9d90, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0102_4834);
    e.mem.set_u32(base + 0x18, 0x0102_4818);
    e.call(0x0096_a2d0, &args![base + 0x1c]);
    e.call(0x004f_15a0, &args![this, 0x52]);
    e.call(0x0048_3710, &args![this]);
    this
}

// Translated from 004f9c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x74 (`SUB ECX,0x74`) and jumps to `0051b070`. `0051b070` (`RET 0x4`) takes one stack word and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_004f9c90(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(
        0x0051_b070,
        &args![this.addr().wrapping_sub(0x74), argument],
    );
}

// Translated from 004f9ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x74 (`SUB ECX,0x74`) and jumps to `0051b020`. `0051b020` (`RET 0x4`) returns a 16-bit value in `AX` (it ends `MOV AX,word ptr [...]`), so the thunk returns `u16`; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_004f9ca0(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    e.call(
        0x0051_b020,
        &args![this.addr().wrapping_sub(0x74), argument],
    )
    .u16()
}

// Translated from 004f9cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x3c (`SUB ECX,0x3c`) and jumps to `004f9480`. `004f9480` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_004f9cb0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x004f_9480, &args![this.addr().wrapping_sub(0x3c), flags])
        .u32()
}

// Translated from 004f9cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x74 (`SUB ECX,0x74`) and jumps to `0051b0a0`. `0051b0a0` (`RET 0x8`) takes two stack words and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack words are passed on unchanged.
pub fn fn_004f9cc0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(
        0x0051_b0a0,
        &args![this.addr().wrapping_sub(0x74), first, second],
    );
}

// Translated from 004f9d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the form component `BGSPreloadable` (the engine map
/// gives it no name; the RTTI name of the vtable it stores, `01024970`, is
/// `.?AVBGSPreloadable@@`): runs the base constructor (`00402d30`) and stores
/// the vtable at `+0`. Returns `this`.
pub fn fn_004f9d90(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0040_2d30, &args![this]);
    e.mem.set_u32(this.addr(), 0x0102_4970);
    this
}

// Translated from 004f9d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004f9db0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004f9d60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004f_9db0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004f9ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004f9ef0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004f9ec0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004f_9ef0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004f9bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `this + 0xAC` (`ECX` only).
pub fn fn_004f9bf0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0xac)
}

// Translated from 004f9db0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f9db0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// `BGSDebris` destructor body (the class of the vtables `01024834` and
/// `01024818` it stores back, by their RTTI): frees the debris entries
/// ([`fn_004f9e30`]), then the member destructor `0046ffb0` (+0x1C) and
/// `TESForm::~TESForm` (`00483630`, engine map).
pub fn fn_004f9db0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0102_4834);
    e.mem.set_u32(this.addr() + 0x18, 0x0102_4818);
    fn_004f9e30(e, this);
    e.call(0x0046_ffb0, &args![this.addr() + 0x1c]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 004f9e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the debris entry list at `this + 0x1C`: while the head is not
/// empty (`008256d0`), the head's entry (`006815c0`) has its model name
/// (+0x4) freed (`00401030`) and is deleted ([`fn_004f9ec0`] with 1), and
/// the head is removed (`0063f7b0`).
pub fn fn_004f9e30(e: &mut Engine, this: Ptr) {
    let head = this.addr() + 0x1c;
    while !e.call(0x0082_56d0, &args![head]).bool() {
        let slot = e.call(0x0068_15c0, &args![head]).u32();
        let entry = e.mem.u32(slot);
        if entry != 0 {
            let name = e.mem.u32(entry + 4);
            e.call(0x0040_1030, &args![name]);
            fn_004f9ec0(e, Ptr::new(entry), 1);
        }
        e.call(0x0063_f7b0, &args![head]);
    }
}

// Translated from 004f9ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of a debris entry: the member destructor `00488ff0` on
/// `this + 8` (its texture list; Ghidra's library match name for this
/// function is unrelated).
pub fn fn_004f9ef0(e: &mut Engine, this: Ptr) {
    e.call(0x0048_8ff0, &args![this.addr() + 8]);
}

// Translated from 004f9b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies from `other` (`RET 4`) when it casts (`__RTDynamicCast` from `01183028` to
/// `01189c78`): the base copy `0051af50`, then the cast object's word at +0xAC
/// (`004f9bf0`) to +0xAC and what `009ee040` gives for it to +0xB0.
pub fn fn_004f9b80(e: &mut Engine, this: Ptr, other: u32) {
    let base = this.addr();
    let cast = e
        .call(
            0x00ec_43fb,
            &args![other, 0u32, 0x0118_3028u32, 0x0118_9c78u32, 0u32],
        )
        .u32();
    if cast == 0 {
        return;
    }
    e.call(0x0051_af50, &args![this, cast]);
    let word = fn_004f9bf0(e, Ptr::new(cast));
    e.mem.set_u32(base + 0xac, word);
    let started = e.call(0x009e_e040, &args![cast]).u32();
    e.mem.set_u32(base + 0xb0, started);
}

// Translated from 004f9c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `other` differs (`RET 4`, `AL`): true unless it casts (`__RTDynamicCast` from
/// `01183028` to `01189c78`), the base comparison `0051afb0` finds no difference, and its
/// word at +0xAC (`004f9bf0`) and `009ee040`'s value equal +0xAC and +0xB0.
pub fn fn_004f9c10(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let base = this.addr();
    let cast = e
        .call(
            0x00ec_43fb,
            &args![other, 0u32, 0x0118_3028u32, 0x0118_9c78u32, 0u32],
        )
        .u32();
    if cast == 0 {
        return true;
    }
    if e.call(0x0051_afb0, &args![this, cast]).bool() {
        return true;
    }
    if e.mem.u32(base + 0xac) != fn_004f9bf0(e, Ptr::new(cast)) {
        return true;
    }
    let started = e.call(0x009e_e040, &args![cast]).u32();
    e.mem.u32(base + 0xb0) != started
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_9bf0, fn_004f9bf0(Ptr) -> u32),
        entry!(0x004f_9db0, fn_004f9db0(Ptr)),
        entry!(0x004f_9e30, fn_004f9e30(Ptr)),
        entry!(0x004f_9ef0, fn_004f9ef0(Ptr)),
        entry!(0x004f_9cd0, fn_004f9cd0(Ptr) -> Ptr),
        entry!(0x004f_9c90, fn_004f9c90(Ptr, u32)),
        entry!(0x004f_9ca0, fn_004f9ca0(Ptr, u32) -> u16),
        entry!(0x004f_9cb0, fn_004f9cb0(Ptr, u32) -> u32),
        entry!(0x004f_9cc0, fn_004f9cc0(Ptr, u32, u32)),
        entry!(0x004f_9d90, fn_004f9d90(Ptr) -> Ptr),
        entry!(0x004f_9d60, fn_004f9d60(Ptr, u32) -> Ptr),
        entry!(0x004f_9ec0, fn_004f9ec0(Ptr, u32) -> Ptr),
        entry!(0x004f_9b80, fn_004f9b80(Ptr, u32)),
        entry!(0x004f_9c10, fn_004f9c10(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004f9cd0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004f_9cd0,
            size: 0x24,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x004f_9d90, &[A::T(0x18)]),
                (0x0096_a2d0, &[A::T(0x1c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x52)]),
                (0x0048_3710, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_4834), (0x18, 4, 0x0102_4818)],
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
    fn le81_thunk_004f9c90() {
        check(0x004f_9c90, 0x0051_b070, 0x74, &[0x77]);
    }

    #[test]
    fn le81_thunk_004f9ca0() {
        assert_eq!(check(0x004f_9ca0, 0x0051_b020, 0x74, &[0x77]).u16(), 0xbeef);
    }

    #[test]
    fn le81_thunk_004f9cb0() {
        assert_eq!(
            check(0x004f_9cb0, 0x004f_9480, 0x3c, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_004f9cc0() {
        check(0x004f_9cc0, 0x0051_b0a0, 0x74, &[0x77, 0x88]);
    }
}

#[cfg(test)]
mod component_tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004f9d90_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004f_9d90,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[(0x0040_2d30, &[A::T(0x0)])],
            stores: &[(0x0, 4, 0x0102_4970)],
        });
    }
    #[test]
    fn at_004f9d60_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_9db0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_9d60, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_9d60)
                .collect();
            let mut expected = vec![(0x004f_9db0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_004f9ec0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_9ef0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_9ec0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_9ec0)
                .collect();
            let mut expected = vec![(0x004f_9ef0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    #[test]
    fn fn_004f9bf0_reads_0xac() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb0);
        e.mem.set_u32(this + 0xac, 0x1234);
        assert_eq!(e.call(0x004f_9bf0, &args![this]).u32(), 0x1234);
    }

    #[test]
    fn fn_004f9ef0_destroys_the_texture_list() {
        let mut e = Engine::new();
        e.register(0x0048_8ff0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004f_9ef0, &args![0x4000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x0048_8ff0, vec![0x4008])]);
    }

    /// The list: the head (+0x1C) holds an entry and a next pointer; the
    /// pop double moves the next node's words into the head.
    fn list_engine() -> Engine {
        let mut e = Engine::new();
        e.register(0x0082_56d0, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0).into_ret()
        });
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(0x0063_f7b0, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let (item, after) = if next == 0 {
                (0, 0)
            } else {
                (e.mem.u32(next), e.mem.u32(next + 4))
            };
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        for callee in [0x0040_1030, 0x0048_8ff0, 0x0046_ffb0, 0x0048_3630] {
            e.register(callee, |_, _| Ret::default());
        }
        e
    }

    fn entry(e: &mut Engine, name: u32) -> u32 {
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry + 4, name);
        entry
    }

    #[test]
    fn fn_004f9e30_frees_every_entry() {
        let mut e = list_engine();
        let this = e.mem.alloc(0x30);
        let first = entry(&mut e, 0x5001);
        let second = entry(&mut e, 0x5002);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, second);
        e.mem.set_u32(this + 0x1c, first);
        e.mem.set_u32(this + 0x20, node);
        e.call_log = Some(vec![]);
        e.call(0x004f_9e30, &args![this]);
        let log = e.call_log.take().unwrap();
        let freed: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == 0x0040_1030)
            .map(|c| c.1[0])
            .collect();
        // Each entry: its name, then the entry itself (deleting destructor).
        assert_eq!(freed, vec![0x5001, first, 0x5002, second]);
        let destroyed: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == 0x0048_8ff0)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(destroyed, vec![first + 8, second + 8]);
        assert_eq!(log.iter().filter(|c| c.0 == 0x0063_f7b0).count(), 2);
    }

    #[test]
    fn fn_004f9db0_restores_the_vtables_and_destroys_the_members() {
        let mut e = list_engine();
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        e.call(0x004f_9db0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0102_4834);
        assert_eq!(e.mem.u32(this + 0x18), 0x0102_4818);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[2..],
            [(0x0046_ffb0, vec![this + 0x1c]), (0x0048_3630, vec![this])]
        );
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
    fn fn_004f9b80_copies_from_the_cast_object() {
        let mut e = Engine::new();
        let other = e.mem.alloc(0xc0);
        e.mem.set_u32(other + 0xac, 0x55);
        e.register(0x00ec_43fb, |_, a| a[0].into_ret());
        e.register(0x0051_af50, |_, _| Ret::default());
        e.register(0x009e_e040, |_, _| 0x66u32.into_ret());
        let this = e.mem.alloc(0xc0);
        e.call_log = Some(vec![]);
        e.call(0x004f_9b80, &args![this, other]);
        assert_eq!(calls_to(&e, 0x0051_af50), vec![vec![this, other]]);
        assert_eq!(
            (e.mem.u32(this + 0xac), e.mem.u32(this + 0xb0)),
            (0x55, 0x66)
        );
        assert_eq!(
            calls_to(&e, 0x00ec_43fb)[0],
            vec![other, 0, 0x0118_3028, 0x0118_9c78, 0]
        );
        e.register(0x00ec_43fb, |_, _| Ret::default());
        let fresh = e.mem.alloc(0xc0);
        e.call(0x004f_9b80, &args![fresh, other]);
        assert_eq!(e.mem.u32(fresh + 0xac), 0);
    }

    #[test]
    fn fn_004f9c10_reports_each_difference() {
        let mut e = Engine::new();
        let other = e.mem.alloc(0xc0);
        e.mem.set_u32(other + 0xac, 0x55);
        e.register(0x00ec_43fb, |_, a| a[0].into_ret());
        e.register(0x0051_afb0, |_, _| false.into_ret());
        e.register(0x009e_e040, |_, _| 0x66u32.into_ret());
        let this = e.mem.alloc(0xc0);
        e.mem.set_u32(this + 0xac, 0x55);
        e.mem.set_u32(this + 0xb0, 0x66);
        assert!(!e.call(0x004f_9c10, &args![this, other]).bool());
        e.mem.set_u32(this + 0xb0, 0x67);
        assert!(e.call(0x004f_9c10, &args![this, other]).bool());
        e.mem.set_u32(this + 0xb0, 0x66);
        e.mem.set_u32(this + 0xac, 0x56);
        assert!(e.call(0x004f_9c10, &args![this, other]).bool());
        e.mem.set_u32(this + 0xac, 0x55);
        e.register(0x0051_afb0, |_, _| true.into_ret());
        assert!(e.call(0x004f_9c10, &args![this, other]).bool());
        e.register(0x00ec_43fb, |_, _| Ret::default());
        assert!(e.call(0x004f_9c10, &args![this, other]).bool());
    }
}
