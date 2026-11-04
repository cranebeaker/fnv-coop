//! `fallout/audio/run_00831eb0` (Xbox PDB source unit), subsystem `fallout/audio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Methods of `CBSAReader` (RTTI `.?AVCBSAReader@@`; vtables `0107cdac`
//! at +0, `0107cd6c` at +0xC and `0107cd54` at +0x10). The three
//! forwarders are `__stdcall` methods of the interface at +0xC: their
//! `this` is the interface pointer on the stack (`ECX` is not read); they
//! take the object (`this - 0xC`), read the inner object it holds at +4
//! (`00726070`) and call one of that object's first three virtual slots,
//! also `__stdcall` with the inner object as the first stack word.

#[allow(unused_imports)]
use crate::prelude::*;

/// The getter `00726070` (`ECX` = object): the word at +4.
const INNER_OBJECT: u32 = 0x0072_6070;
/// The method run on the member at +0x104 (`009fc990`, `this` and two
/// words).
const MEMBER_FORWARD: u32 = 0x009f_c990;
/// The base destructor body `009fdc10` (`this` only).
const BASE_DESTRUCTOR: u32 = 0x009f_dc10;
/// `operator delete` (`00401030`, cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `CBSAReader`'s vtables: the object's own, and those of the interfaces
/// at +0xC and +0x10.
const VTABLE: u32 = 0x0107_cdac;
const VTABLE_AT_C: u32 = 0x0107_cd6c;
const VTABLE_AT_10: u32 = 0x0107_cd54;

/// The inner object of the interface at `interface` (`this - 0xC` of it).
fn inner(e: &mut Engine, interface: Ptr) -> u32 {
    e.call(INNER_OBJECT, &args![interface.addr().wrapping_sub(0xc)])
        .u32()
}

// Translated from 00831eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The interface's forwarder with two arguments: calls the inner object's
/// virtual slot +0 with `first` and `second`; `EAX` is the inner call's.
pub fn fn_00831eb0(e: &mut Engine, interface: Ptr, first: u32, second: u32) -> u32 {
    let object = inner(e, interface);
    e.vcall(object, 0x0, &args![first, second]).u32()
}

// Translated from 00831ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The interface's forwarder to the inner object's virtual slot +4 (no
/// further arguments); `EAX` is the inner call's.
pub fn fn_00831ee0(e: &mut Engine, interface: Ptr) -> u32 {
    let object = inner(e, interface);
    e.vcall(object, 0x4, &[]).u32()
}

// Translated from 00831f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The interface's forwarder to the inner object's virtual slot +8 (no
/// further arguments); `EAX` is the inner call's.
pub fn fn_00831f10(e: &mut Engine, interface: Ptr) -> u32 {
    let object = inner(e, interface);
    e.vcall(object, 0x8, &[]).u32()
}

// Translated from 00831f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands `first` and `second` to the member at +0x104 (`009fc990`);
/// `EAX` is that call's.
pub fn fn_00831f40(e: &mut Engine, this: Ptr, first: u32, second: u32) -> u32 {
    let member = this.addr().wrapping_add(0x104);
    e.call(MEMBER_FORWARD, &args![member, first, second]).u32()
}

// Translated from 00831f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CBSAReader`'s scalar deleting destructor: the destructor body
/// (`00831fb0`), then `operator delete` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_00831f80(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00831fb0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00831fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CBSAReader`'s destructor body: puts back its three vtables (+0, +0xC,
/// +0x10) and runs the base destructor body (`009fdc10`). Ghidra matches
/// the body to a library destructor of the same shape.
pub fn fn_00831fb0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.mem.set_u32(this.addr().wrapping_add(0xc), VTABLE_AT_C);
    e.mem.set_u32(this.addr().wrapping_add(0x10), VTABLE_AT_10);
    e.call(BASE_DESTRUCTOR, &args![this]);
}

// Translated from 00831f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The interface methods at +0xC and +0x10 of the interface vtable `0107cd54` (`__stdcall`, the
/// interface pointer as the only word, `RET 4`): answers 0 (`XOR EAX,EAX`).
pub fn fn_00831f70(_e: &mut Engine, _interface: Ptr) -> u32 {
    0
}

// Translated from 00831fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of the interface at +0x10 (`SUB [ESP+4],4; JMP`): [`fn_00831eb0`] on the interface at
/// +0xC (`interface - 4`) with the same two words; its answer is passed on.
pub fn fn_00831fe0(e: &mut Engine, interface: Ptr, first: u32, second: u32) -> u32 {
    fn_00831eb0(e, Ptr::new(interface.addr().wrapping_sub(4)), first, second)
}

// Translated from 00831ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of the interface at +0x10: [`fn_00831f10`] on `interface - 4`; its answer is passed
/// on.
pub fn fn_00831ff0(e: &mut Engine, interface: Ptr) -> u32 {
    fn_00831f10(e, Ptr::new(interface.addr().wrapping_sub(4)))
}

// Translated from 00832000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of the interface at +0x10: [`fn_00831ee0`] on `interface - 4`; its answer is passed
/// on.
pub fn fn_00832000(e: &mut Engine, interface: Ptr) -> u32 {
    fn_00831ee0(e, Ptr::new(interface.addr().wrapping_sub(4)))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0083_1eb0, fn_00831eb0(Ptr, u32, u32) -> u32),
        entry!(0x0083_1ee0, fn_00831ee0(Ptr) -> u32),
        entry!(0x0083_1f10, fn_00831f10(Ptr) -> u32),
        entry!(0x0083_1f40, fn_00831f40(Ptr, u32, u32) -> u32),
        entry!(0x0083_1f80, fn_00831f80(Ptr, u32) -> Ptr),
        entry!(0x0083_1fb0, fn_00831fb0(Ptr)),
        entry!(0x0083_1f70, fn_00831f70(Ptr) -> u32),
        entry!(0x0083_1fe0, fn_00831fe0(Ptr, u32, u32) -> u32),
        entry!(0x0083_1ff0, fn_00831ff0(Ptr) -> u32),
        entry!(0x0083_2000, fn_00832000(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
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

    /// An object whose +4 holds an inner object with three recording
    /// slots; returns (engine, object, inner).
    fn reader() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register(INNER_OBJECT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        for slot in 0..3u32 {
            e.register(0x0093_0000 + slot * 4, |_, a| Ret {
                eax: a.len() as u32,
                ..Ret::default()
            });
        }
        e.put_vtable(0x0093_1000, &[0x0093_0000, 0x0093_0004, 0x0093_0008]);
        let object = e.mem.alloc(0x120);
        let inner = e.mem.alloc(8);
        e.mem.set_u32(inner, 0x0093_1000);
        e.mem.set_u32(object + 4, inner);
        (e, object, inner)
    }

    #[test]
    fn two_argument_forwarder() {
        let (mut e, object, inner) = reader();
        e.call_log = Some(vec![]);
        let r = e.call(0x0083_1eb0, &args![object + 0xc, 5u32, 6u32]).u32();
        assert_eq!(r, 3);
        assert_eq!(calls_to(&e, INNER_OBJECT), vec![vec![object]]);
        assert_eq!(calls_to(&e, 0x0093_0000), vec![vec![inner, 5, 6]]);
    }

    #[test]
    fn slot_four_and_eight_forwarders() {
        let (mut e, object, inner) = reader();
        e.call_log = Some(vec![]);
        e.call(0x0083_1ee0, &args![object + 0xc]);
        e.call(0x0083_1f10, &args![object + 0xc]);
        assert_eq!(calls_to(&e, 0x0093_0004), vec![vec![inner]]);
        assert_eq!(calls_to(&e, 0x0093_0008), vec![vec![inner]]);
        assert!(calls_to(&e, 0x0093_0000).is_empty());
    }

    #[test]
    fn member_forwarder() {
        let mut e = Engine::new();
        e.register(MEMBER_FORWARD, |_, _| Ret {
            eax: 9,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let r = e.call(0x0083_1f40, &args![0x0050_0000u32, 1u32, 2u32]);
        assert_eq!(r.u32(), 9);
        assert_eq!(calls_to(&e, MEMBER_FORWARD), vec![vec![0x0050_0104, 1, 2]]);
    }

    #[test]
    fn destructors() {
        let mut e = Engine::new();
        e.register(BASE_DESTRUCTOR, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let object = e.mem.alloc(0x120);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_1f80, &args![object, 0u32]).u32(), object);
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(e.mem.u32(object + 0xc), VTABLE_AT_C);
        assert_eq!(e.mem.u32(object + 0x10), VTABLE_AT_10);
        assert_eq!(calls_to(&e, BASE_DESTRUCTOR), vec![vec![object]]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        e.call(0x0083_1f80, &args![object, 1u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![object]]);
        e.call(0x0083_1fb0, &args![object]);
        assert_eq!(calls_to(&e, BASE_DESTRUCTOR).len(), 3);
    }
}

#[cfg(test)]
mod n089_tests {
    use super::*;

    #[test]
    fn n089_00831f70_answers_zero() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0083_1f70, &args![0x1234u32]).u32(), 0);
    }

    #[test]
    fn n089_00831fe0_00831ff0_00832000_forward_from_the_second_interface() {
        let (mut e, object, inner) = tests_reader();
        let interface = object + 0x10;
        e.call_log = Some(vec![]);
        // Each inner slot answers its word count.
        assert_eq!(e.call(0x0083_1fe0, &args![interface, 7u32, 8u32]).u32(), 3);
        assert_eq!(e.call(0x0083_1ff0, &args![interface]).u32(), 1);
        assert_eq!(e.call(0x0083_2000, &args![interface]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let slots: Vec<_> = log
            .iter()
            .filter(|c| (0x0093_0000..0x0093_000c).contains(&c.0))
            .cloned()
            .collect();
        assert_eq!(
            slots,
            vec![
                (0x0093_0000, vec![inner, 7, 8]),
                (0x0093_0008, vec![inner]),
                (0x0093_0004, vec![inner])
            ]
        );
        assert!(log
            .iter()
            .filter(|c| c.0 == INNER_OBJECT)
            .all(|c| c.1 == vec![object]));
    }

    /// The reader of the `tests` module.
    fn tests_reader() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register(INNER_OBJECT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        for slot in 0..3u32 {
            e.register(0x0093_0000 + slot * 4, |_, a| (a.len() as u32).into_ret());
        }
        e.put_vtable(0x0093_1000, &[0x0093_0000, 0x0093_0004, 0x0093_0008]);
        let object = e.mem.alloc(0x120);
        let inner = e.mem.alloc(8);
        e.mem.set_u32(inner, 0x0093_1000);
        e.mem.set_u32(object + 4, inner);
        (e, object, inner)
    }
}
