//! `Havok SDK/run_00e01540` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkaabbtree::{allocator, release_array};

// Translated from 00e01540 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_9300.
pub fn return_constant_00e01540(_e: &mut Engine) -> u32 {
    0x010e_9300
}

// Translated from 00e01580 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_9358.
pub fn return_constant_00e01580(_e: &mut Engine) -> u32 {
    0x010e_9358
}

// Translated from 00e015c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_93ac.
pub fn return_constant_00e015c0(_e: &mut Engine) -> u32 {
    0x010e_93ac
}

// Translated from 00e01670 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_943c.
pub fn return_constant_00e01670(_e: &mut Engine) -> u32 {
    0x010e_943c
}

// Translated from 00e016b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_9494.
pub fn return_constant_00e016b0(_e: &mut Engine) -> u32 {
    0x010e_9494
}

// Translated from 00e01710 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x2c-byte frame stores the vtable word 0x010e_9544 at
/// +0xC, runs the constructor at `00cb0a70` (argument 0) on the members at frame+0x24, 0x28 in that
/// order and returns the vtable word. Nothing is destroyed.
pub fn fn_00e01710(e: &mut Engine) -> u32 {
    e.with_stack(0x2c, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base + 0xc, 0x010e_9544);
        e.call(0x00cb_0a70, &args![base + 0x24, 0u32]);
        e.call(0x00cb_0a70, &args![base + 0x28, 0u32]);
        e.mem.u32(base + 0xc)
    })
}

// Translated from 00e015d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a deleting destructor (`this` in `ECX`, one stack word `flags`)
/// of an object with an `hkArray`-shaped triple of 4-byte elements at `+8` (data, size,
/// capacity and flags): it releases the array's storage (size cleared, data returned through
/// the allocator's slot `0x10` unless the capacity word is negative, data cleared, capacity
/// word `0x80000000`), writes the vtable word `0102e388` and, when bit 0 of `flags` is set,
/// returns the block to the memory router's allocator (slot `8`) with the 16-bit size kept at
/// `+4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e015d0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let base = this.addr();
    release_array(e, base + 8, 4);
    e.mem.set_u32(base, 0x0102_e388);
    if flags & 1 != 0 {
        let allocator = allocator(e);
        let size = u32::from(e.mem.u16(base + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00e016e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010e9544` and
/// runs `00cb0a70` (an `hkStringPtr` function, one stack word `extra`) on `object + 0x18` and
/// then on `object + 0x1c`. The function takes two stack words and pops none (`RET`), and
/// returns nothing. The class is unconfirmed.
pub fn fn_00e016e0(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010e_9544);
        e.call(0x00cb_0a70, &args![object.addr() + 0x18, extra]);
        e.call(0x00cb_0a70, &args![object.addr() + 0x1c, extra]);
    }
}

// Translated from 00e01510 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc63d9). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00e01510(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00e01530 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc63cf). Cdecl `(object)`: does nothing for a
/// null `object`; otherwise stores the vtable word `0x010e_9300` (RTTI `hkxVertexVectorDataChannel`) at `+0`.
pub fn fn_00e01530(e: &mut Engine, object: Ptr) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010e_9300);
    }
}

// Translated from 00e01550 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc6449). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00e01550(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00e01570 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc643f). Cdecl `(object)`: does nothing for a
/// null `object`; otherwise stores the vtable word `0x010e_9358` (RTTI `hkxVertexSelectionChannel`) at `+0`.
pub fn fn_00e01570(e: &mut Engine, object: Ptr) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010e_9358);
    }
}

// Translated from 00e01590 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc64b9). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00e01590(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00e015b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc64af). Cdecl `(object)`: does nothing for a
/// null `object`; otherwise stores the vtable word `0x010e_93ac` (RTTI `hkxVertexIntDataChannel`) at `+0`.
pub fn fn_00e015b0(e: &mut Engine, object: Ptr) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010e_93ac);
    }
}

// Translated from 00e01640 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc6549). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00e01640(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00e01660 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc653f). Cdecl `(object)`: does nothing for a
/// null `object`; otherwise stores the vtable word `0x010e_943c` (RTTI `hkxVertexFloatDataChannel`) at `+0`.
pub fn fn_00e01660(e: &mut Engine, object: Ptr) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010e_943c);
    }
}

// Translated from 00e01680 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc65b9). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00e01680(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00e016a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc65af). Cdecl `(object)`: does nothing for a
/// null `object`; otherwise stores the vtable word `0x010e_9494` (RTTI `hkxTriangleSelectionChannel`) at `+0`.
pub fn fn_00e016a0(e: &mut Engine, object: Ptr) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010e_9494);
    }
}

// Translated from 00e016c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc6629). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00e016c0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e01540, return_constant_00e01540() -> u32),
        entry!(0x00e01580, return_constant_00e01580() -> u32),
        entry!(0x00e015c0, return_constant_00e015c0() -> u32),
        entry!(0x00e01670, return_constant_00e01670() -> u32),
        entry!(0x00e016b0, return_constant_00e016b0() -> u32),
        entry!(0x00e01710, fn_00e01710() -> u32),
        entry!(0x00e0_15d0, deleting_destructor_00e015d0(Ptr, u8) -> Ptr),
        entry!(0x00e0_16e0, fn_00e016e0(Ptr, u32)),
        entry!(0x00e0_1510, fn_00e01510(Ptr) -> u32),
        entry!(0x00e0_1530, fn_00e01530(Ptr)),
        entry!(0x00e0_1550, fn_00e01550(Ptr) -> u32),
        entry!(0x00e0_1570, fn_00e01570(Ptr)),
        entry!(0x00e0_1590, fn_00e01590(Ptr) -> u32),
        entry!(0x00e0_15b0, fn_00e015b0(Ptr)),
        entry!(0x00e0_1640, fn_00e01640(Ptr) -> u32),
        entry!(0x00e0_1660, fn_00e01660(Ptr)),
        entry!(0x00e0_1680, fn_00e01680(Ptr) -> u32),
        entry!(0x00e0_16a0, fn_00e016a0(Ptr)),
        entry!(0x00e0_16c0, fn_00e016c0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// What the probe saw on one call: `this`, the argument, the word `behind` bytes before `this`
    /// (the frame's vtable slot) and, when asked for, the `u32` at +0x20, `u16` at +0x24 and `u16`
    /// at +0x26 of the frame.
    type ProbeCall = (u32, u32, u32, [u32; 3]);
    type ProbeLog = Rc<RefCell<Vec<ProbeCall>>>;

    /// Double for the member constructor: logs, per call, what `ProbeCall` describes, with the
    /// vtable slot `behind[call]` bytes before `this`.
    fn frame_probe(e: &mut Engine, address: u32, behind: &[u32], with_tail: bool) -> ProbeLog {
        let log: ProbeLog = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        let behind = behind.to_vec();
        e.register_double(address, move |e, a| {
            let call = seen.borrow().len();
            let slot = a[0] - behind[call];
            let mut tail = [0; 3];
            if with_tail {
                tail = [
                    e.mem.u32(slot + 0x20),
                    u32::from(e.mem.u16(slot + 0x24)),
                    u32::from(e.mem.u16(slot + 0x26)),
                ];
            }
            seen.borrow_mut().push((a[0], a[1], e.mem.u32(slot), tail));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        log
    }

    #[test]
    fn return_constant_00e01540_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_1540, &args![]).u32(), 0x010e_9300);
    }

    #[test]
    fn return_constant_00e01580_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_1580, &args![]).u32(), 0x010e_9358);
    }

    #[test]
    fn return_constant_00e015c0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_15c0, &args![]).u32(), 0x010e_93ac);
    }

    #[test]
    fn return_constant_00e01670_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_1670, &args![]).u32(), 0x010e_943c);
    }

    #[test]
    fn return_constant_00e016b0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_16b0, &args![]).u32(), 0x010e_9494);
    }

    #[test]
    fn fn_00e01710_stores_the_vtable_and_constructs_its_members() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x18, 0x1c], false);
        let result = e.call(0x00e0_1710, &args![]).u32();
        assert_eq!(result, 0x010e_9544);
        let calls = log.borrow();
        assert_eq!(calls.len(), 2);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010e_9544);
        }
        assert_eq!(calls[1].0 - calls[0].0, 0x4);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;
    const BLOCK_SLOT_DOUBLE: u32 = 0x0900_0021;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    /// An owned array: the storage is freed (capacity 5 times 4 bytes) before the vtable word is
    /// written; the block itself only with bit 0 of the flags (size kept at +4).
    #[test]
    fn deleting_destructor_00e015d0_releases_the_array_then_frees_the_block() {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, FREE_SLOT_DOUBLE);
        e.mem.set_u32(vtable + 8, BLOCK_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for (tag, slot) in [(1u32, FREE_SLOT_DOUBLE), (2, BLOCK_SLOT_DOUBLE)] {
            let slot_log = Rc::clone(&log);
            e.register_double(slot, move |_, a| {
                let mut words = vec![tag];
                words.extend_from_slice(a);
                slot_log.borrow_mut().push(words);
                Ret::default()
            });
        }
        let this = e.mem.alloc(0x20);
        e.mem.set_u16(this + 4, 0x2222);
        e.mem.set_u32(this + 8, 0xd00d);
        e.mem.set_u32(this + 0xc, 4);
        e.mem.set_u32(this + 0x10, 5);
        let got = e.call(0x00e0_15d0, &args![this, 0u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![vec![1, allocator, 0xd00d, 20]]);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x8000_0000);
        // The array is now empty and not owned: only the block is freed.
        log.borrow_mut().clear();
        e.call(0x00e0_15d0, &args![this, 1u32]);
        assert_eq!(*log.borrow(), vec![vec![2, allocator, this, 0x2222]]);
    }

    /// A null object does nothing; otherwise the string function runs on both members.
    #[test]
    fn fn_00e016e0_runs_the_string_function_twice() {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let string_log = Rc::clone(&log);
        e.register_double(0x00cb_0a70, move |_, a| {
            string_log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(0x00e0_16e0, &args![0u32, 4u32]);
        assert!(log.borrow().is_empty());
        let object = e.mem.alloc(0x30);
        e.call(0x00e0_16e0, &args![object, 4u32]);
        assert_eq!(e.mem.u32(object), 0x010e_9544);
        assert_eq!(
            *log.borrow(),
            vec![vec![object + 0x18, 4], vec![object + 0x1c, 4]]
        );
    }
}

#[cfg(test)]
mod a05_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type CallLog = Rc<RefCell<Vec<Vec<u32>>>>;

    /// A double at `address` that records the argument words of each call and answers `answer`.
    fn recording_double(e: &mut Engine, address: u32, answer: u32) -> CallLog {
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        e.register_double(address, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
        log
    }

    /// `function(object)` calls slot 0 of the object's vtable with `(object, 0)` and answers
    /// what the slot answers.
    fn check_cleanup(function: u32) {
        let mut e = Engine::new();
        let log = recording_double(&mut e, 0x7000_0000, 0x1234_5678);
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table, 0x7000_0000);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, table);
        assert_eq!(e.call(function, &args![object]).u32(), 0x1234_5678);
        assert_eq!(*log.borrow(), vec![vec![object, 0]]);
    }

    macro_rules! cleanup_test {
        ($name:ident, $function:expr) => {
            #[test]
            fn $name() {
                check_cleanup($function);
            }
        };
    }

    /// `function(object)` stores `vtable` at the start of a non-null object and leaves a null one.
    fn check_vtable(function: u32, vtable: u32) {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.call(function, &args![object]);
        assert_eq!(e.mem.u32(object), vtable);
        e.call(function, &args![0u32]);
    }

    macro_rules! vtable_test {
        ($name:ident, $function:expr, $vtable:expr) => {
            #[test]
            fn $name() {
                check_vtable($function, $vtable);
            }
        };
    }

    cleanup_test!(fn_00e01510_calls_virtual_slot_0_with_zero, 0x00e0_1510);
    cleanup_test!(fn_00e01550_calls_virtual_slot_0_with_zero, 0x00e0_1550);
    cleanup_test!(fn_00e01590_calls_virtual_slot_0_with_zero, 0x00e0_1590);
    cleanup_test!(fn_00e01640_calls_virtual_slot_0_with_zero, 0x00e0_1640);
    cleanup_test!(fn_00e01680_calls_virtual_slot_0_with_zero, 0x00e0_1680);
    cleanup_test!(fn_00e016c0_calls_virtual_slot_0_with_zero, 0x00e0_16c0);
    vtable_test!(fn_00e01530_stores_the_vtable_word, 0x00e0_1530, 0x010e_9300);
    vtable_test!(fn_00e01570_stores_the_vtable_word, 0x00e0_1570, 0x010e_9358);
    vtable_test!(fn_00e015b0_stores_the_vtable_word, 0x00e0_15b0, 0x010e_93ac);
    vtable_test!(fn_00e01660_stores_the_vtable_word, 0x00e0_1660, 0x010e_943c);
    vtable_test!(fn_00e016a0_stores_the_vtable_word, 0x00e0_16a0, 0x010e_9494);
}
