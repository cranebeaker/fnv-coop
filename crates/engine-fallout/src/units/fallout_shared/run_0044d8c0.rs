//! `fallout shared/run_0044d8c0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0044ddc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at `+8` of `this` (a plain field getter; `0040fae0` follows it from an element to the next one).
pub fn fn_0044ddc0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 8)
}

/// The function `fn_0044dd40` calls on the member (`0044cbf0`).
const CALLEE_0044CBF0: u32 = 0x0044_cbf0;

// Translated from 0044dd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0044cbf0` with `ECX` = `this + 0x4` and passes on nothing (the callee returns no value).
pub fn fn_0044dd40(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_0044CBF0, &args![this.byte_add(0x4)]);
}

// Translated from 0044d8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0044d8f0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0044d8c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_d8f0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0044d980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0044d9b0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0044d980(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_d9b0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0044da40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0044da70` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0044da40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_da70, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0044db00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0044db30` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0044db00(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_db30, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0044dbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0044c1e0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0044dbc0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_c1e0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0044dbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0044c3f0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0044dbf0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_c3f0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0044dc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0044dc50` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0044dc20(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_dc50, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0044dce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0044cae0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0044dce0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_cae0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The `LockFreeMap` destructors' callees: `00528e40` (`thiscall`, one
/// word, given 1) clears the map's entries; `00401030` (`cdecl`) frees a
/// block; `0044b4b0` (unit `modelloader.cpp`) is the base destructor body.
/// The blocks at +0x0c and +0x04 are freed in that order.
const LOCK_FREE_MAP_CLEAR: u32 = 0x0052_8e40;
const FREE: u32 = 0x0040_1030;
const LOCK_FREE_MAP_BASE_DESTRUCT: u32 = 0x0044_b4b0;
const MAP_BLOCK_A: u32 = 0x0c;
const MAP_BLOCK_B: u32 = 0x04;
/// `004019a0` (`cdecl`, one word; the engine map's name for it is
/// `WaitForSingleObjectEx`, but its body is `InterlockedDecrement` of the
/// word): the release of a counted object.
const DECREMENT: u32 = 0x0040_19a0;
/// A counted object's count (+0x08) and its scalar deleting destructor
/// (virtual slot 0).
const COUNT: u32 = 0x08;
const SLOT_DELETING_DESTRUCTOR: u32 = 0x00;
/// The virtual slot `0044dde0` reports a node to (`0x28`, the node's first
/// word).
const SLOT_NODE_REMOVED: u32 = 0x28;
/// The flags byte `0044de20` tests (+0x0c).
const FLAGS: u32 = 0x0c;

/// The body every `LockFreeMap` destructor here shares: the vtable, the
/// entries cleared (`00528e40` with 1), the two blocks freed, then the base
/// destructor body.
fn lock_free_map_destruct(e: &mut Engine, this: Ptr, vtable: u32) {
    let map = this.addr();
    e.mem.set_u32(map, vtable);
    e.call(LOCK_FREE_MAP_CLEAR, &args![this, 1u32]);
    let block = e.mem.u32(map.wrapping_add(MAP_BLOCK_A));
    e.call(FREE, &args![block]);
    let block = e.mem.u32(map.wrapping_add(MAP_BLOCK_B));
    e.call(FREE, &args![block]);
    e.call(LOCK_FREE_MAP_BASE_DESTRUCT, &args![this]);
}

// Translated from 0044d8f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0044d8f0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00eed038, state variable) is not translated
/// Unnamed in the engine map: the destructor of
/// `LockFreeMap<TESObjectREFR *, NiPointer<QueuedReference>>` (its vtable
/// `0101747c`, RTTI): see `lock_free_map_destruct`.
pub fn fn_0044d8f0(e: &mut Engine, this: Ptr) {
    lock_free_map_destruct(e, this, 0x0101_747c);
}

// Translated from 0044d9b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0044d9b0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00eed038, state variable) is not translated
/// Unnamed in the engine map: the destructor of
/// `LockFreeMap<AnimIdle *, NiPointer<QueuedAnimIdle>>` (vtable
/// `010174cc`, RTTI): see `lock_free_map_destruct`.
pub fn fn_0044d9b0(e: &mut Engine, this: Ptr) {
    lock_free_map_destruct(e, this, 0x0101_74cc);
}

// Translated from 0044da70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0044da70 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00eed038, state variable) is not translated
/// Unnamed in the engine map: the destructor of
/// `LockFreeMap<Animation *, NiPointer<QueuedReplacementKFList>>` (vtable
/// `0101751c`, RTTI): see `lock_free_map_destruct`.
pub fn fn_0044da70(e: &mut Engine, this: Ptr) {
    lock_free_map_destruct(e, this, 0x0101_751c);
}

// Translated from 0044db30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0044db30 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00eed038, state variable) is not translated
/// Unnamed in the engine map: the destructor of
/// `LockFreeMap<TESObjectREFR *, NiPointer<QueuedHelmet>>` (vtable
/// `0101756c`, RTTI): see `lock_free_map_destruct`.
pub fn fn_0044db30(e: &mut Engine, this: Ptr) {
    lock_free_map_destruct(e, this, 0x0101_756c);
}

// Translated from 0044dc50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0044dc50 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00eed038, state variable) is not translated
/// Unnamed in the engine map: the destructor of
/// `LockFreeMap<BSFileEntry *, NiPointer<QueuedTexture>>` (vtable
/// `0101765c`, RTTI): see `lock_free_map_destruct`.
pub fn fn_0044dc50(e: &mut Engine, this: Ptr) {
    lock_free_map_destruct(e, this, 0x0101_765c);
}

// Translated from 0044dd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: releases a counted object (count at +0x08,
/// through `004019a0`); at zero, a non-null object runs its scalar deleting
/// destructor (virtual slot 0) with 1.
pub fn fn_0044dd60(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    if e.call(DECREMENT, &args![object.wrapping_add(COUNT)]).u32() == 0 && object != 0 {
        e.vcall(object, SLOT_DELETING_DESTRUCTOR, &args![1u32]);
    }
}

// Translated from 0044dde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: reports `node`'s first word to the object's
/// slot `0x28`, then frees the node (`00401030`).
pub fn fn_0044dde0(e: &mut Engine, this: Ptr, node: Ptr) {
    let first = e.mem.u32(node.addr());
    e.vcall(this.addr(), SLOT_NODE_REMOVED, &args![first]);
    e.call(FREE, &args![node]);
}

// Translated from 0044de20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether bit 0 of the byte at +0x0c is set.
pub fn fn_0044de20(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr().wrapping_add(FLAGS)) & 1 != 0
}

/// `00ec7944` (`thiscall`, `RET 4`): the `std::exception` copy constructor.
const EXCEPTION_COPY: u32 = 0x00ec_7944;
/// `std::bad_alloc`'s vtable (`010176f8`; RTTI `std::bad_alloc`, read
/// through its locator).
const BAD_ALLOC_VTABLE: u32 = 0x0101_76f8;

// Translated from 0044dd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::bad_alloc`'s copy constructor (`thiscall`, `RET 4`; the class from
/// the vtable's RTTI): the `std::exception` copy (`00ec7944`), then the
/// vtable `010176f8`. Returns `this`.
pub fn fn_0044dd10(e: &mut Engine, this: Ptr, other: u32) -> Ptr {
    e.call(EXCEPTION_COPY, &args![this, other]);
    e.mem.set_u32(this.addr(), BAD_ALLOC_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0044_dd10, fn_0044dd10(Ptr, u32) -> Ptr),
        entry!(0x0044ddc0, fn_0044ddc0(Ptr) -> u32),
        entry!(0x0044_dd40, fn_0044dd40(Ptr)),
        entry!(0x0044_d8c0, fn_0044d8c0(Ptr, u32) -> Ptr),
        entry!(0x0044_d980, fn_0044d980(Ptr, u32) -> Ptr),
        entry!(0x0044_da40, fn_0044da40(Ptr, u32) -> Ptr),
        entry!(0x0044_db00, fn_0044db00(Ptr, u32) -> Ptr),
        entry!(0x0044_dbc0, fn_0044dbc0(Ptr, u32) -> Ptr),
        entry!(0x0044_dbf0, fn_0044dbf0(Ptr, u32) -> Ptr),
        entry!(0x0044_dc20, fn_0044dc20(Ptr, u32) -> Ptr),
        entry!(0x0044_dce0, fn_0044dce0(Ptr, u32) -> Ptr),
        entry!(0x0044_d8f0, fn_0044d8f0(Ptr)),
        entry!(0x0044_d9b0, fn_0044d9b0(Ptr)),
        entry!(0x0044_da70, fn_0044da70(Ptr)),
        entry!(0x0044_db30, fn_0044db30(Ptr)),
        entry!(0x0044_dc50, fn_0044dc50(Ptr)),
        entry!(0x0044_dd60, fn_0044dd60(Ptr)),
        entry!(0x0044_dde0, fn_0044dde0(Ptr, Ptr)),
        entry!(0x0044_de20, fn_0044de20(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_field_at_offset_8() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0x50));
        e.mem.set_u32(object.addr() + 8, 0x1234_5678);
        assert_eq!(e.call(0x0044_ddc0, &args![object]).u32(), 0x1234_5678);
    }

    #[test]
    fn l68d_0044dd40_passes_this_plus_4_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0044CBF0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0044_dd40, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0044CBF0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x4]);
    }
    #[test]
    fn at_0044d8c0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0044_d8f0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0044_d8c0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0044_d8c0)
                .collect();
            let mut expected = vec![(0x0044_d8f0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0044d980_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0044_d9b0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0044_d980, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0044_d980)
                .collect();
            let mut expected = vec![(0x0044_d9b0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0044da40_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0044_da70, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0044_da40, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0044_da40)
                .collect();
            let mut expected = vec![(0x0044_da70, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0044db00_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0044_db30, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0044_db00, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0044_db00)
                .collect();
            let mut expected = vec![(0x0044_db30, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0044dbc0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0044_c1e0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0044_dbc0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0044_dbc0)
                .collect();
            let mut expected = vec![(0x0044_c1e0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0044dbf0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0044_c3f0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0044_dbf0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0044_dbf0)
                .collect();
            let mut expected = vec![(0x0044_c3f0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0044dc20_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0044_dc50, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0044_dc20, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0044_dc20)
                .collect();
            let mut expected = vec![(0x0044_dc50, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0044dce0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0044_cae0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0044_dce0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0044_dce0)
                .collect();
            let mut expected = vec![(0x0044_cae0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const DELETING_DESTRUCTOR: u32 = 0x00ee_0100;
    const NODE_REMOVED: u32 = 0x00ee_0200;

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            eax.into_ret()
        });
    }

    #[test]
    fn lock_free_map_destructors_clear_free_and_run_the_base() {
        for (address, vtable) in [
            (0x0044_d8f0u32, 0x0101_747cu32),
            (0x0044_d9b0, 0x0101_74cc),
            (0x0044_da70, 0x0101_751c),
            (0x0044_db30, 0x0101_756c),
            (0x0044_dc50, 0x0101_765c),
        ] {
            let mut e = Engine::new();
            let log = Log::default();
            for callee in [LOCK_FREE_MAP_CLEAR, FREE, LOCK_FREE_MAP_BASE_DESTRUCT] {
                logged(&mut e, &log, callee, 0);
            }
            let this = e.mem.alloc(0x20);
            e.mem.set_u32(this + MAP_BLOCK_B, 0x4400);
            e.mem.set_u32(this + MAP_BLOCK_A, 0x5500);
            e.call(address, &args![this]);
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(
                *log.borrow(),
                [
                    (LOCK_FREE_MAP_CLEAR, vec![this, 1]),
                    (FREE, vec![0x5500]),
                    (FREE, vec![0x4400]),
                    (LOCK_FREE_MAP_BASE_DESTRUCT, vec![this]),
                ]
            );
        }
    }

    #[test]
    fn fn_0044dd60_deletes_at_a_zero_count() {
        let mut e = Engine::new();
        let log = Log::default();
        e.register(DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        logged(&mut e, &log, DELETING_DESTRUCTOR, 0);
        let vtable = e.mem.alloc(4);
        e.mem.set_u32(vtable, DELETING_DESTRUCTOR);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + COUNT, 2);
        e.call(0x0044_dd60, &args![object]);
        assert!(log.borrow().is_empty());
        e.call(0x0044_dd60, &args![object]);
        assert_eq!(*log.borrow(), [(DELETING_DESTRUCTOR, vec![object, 1])]);
    }

    #[test]
    fn fn_0044dde0_reports_and_frees_the_node_and_0044de20_tests_bit_0() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NODE_REMOVED, 0);
        logged(&mut e, &log, FREE, 0);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + SLOT_NODE_REMOVED, NODE_REMOVED);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, vtable);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, 0x77);
        e.call(0x0044_dde0, &args![this, node]);
        assert_eq!(
            *log.borrow(),
            [(NODE_REMOVED, vec![this, 0x77]), (FREE, vec![node])]
        );
        e.mem.set_u8(this + FLAGS, 0xfe);
        assert!(!e.call(0x0044_de20, &args![this]).bool());
        e.mem.set_u8(this + FLAGS, 3);
        assert!(e.call(0x0044_de20, &args![this]).bool());
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    #[test]
    fn at_0044dd10_copies_then_sets_the_vtable() {
        let mut e = Engine::new();
        e.register(EXCEPTION_COPY, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        let this = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_dd10, &args![this, 0x5000u32]).u32(), this);
        assert_eq!(e.mem.u32(this), BAD_ALLOC_VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (EXCEPTION_COPY, vec![this, 0x5000]));
    }
}
