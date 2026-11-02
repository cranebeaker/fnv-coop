//! `Havok SDK/run_00dfbd30` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dfbd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e3_60f0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dfbd40(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e3_60f0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dfbd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_ec3c.
pub fn return_constant_00dfbd30(_e: &mut Engine) -> u32 {
    0x010d_ec3c
}

// Translated from 00dfbd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `8` (`SUB ECX,0x8`) and jumps to the deleting destructor
/// `00dfbd40` (`RET 4`); the caller gets its `EAX`, `this - 8`. The class is unconfirmed.
pub fn fn_00dfbd70(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    deleting_destructor_00dfbd40(e, Ptr::new(this.addr().wrapping_sub(0x8)), flags)
}

// Translated from 00dfbbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011ba358): cdecl with two stack words, `object` and `finishing`. When `object` is not null, runs `00cb0a70` (the `CALL` at 00dfbbfd) (thiscall, `RET 4`) on `object` with the one stack word `finishing`.
pub fn fn_00dfbbf0(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.call(0x00cb_0a70, &args![object.addr(), finishing]);
    }
}

// Translated from 00dfbc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011ba35c): cdecl with one stack word, `object`. Jumps to `00cb0c00` (the `JMP` at 00dfbc24) (`hkStringPtr::~hkStringPtr` in Ghidra, fastcall, no stack word) with `object` as `this`, so it answers what that answers.
pub fn fn_00dfbc20(e: &mut Engine, object: Ptr) -> u32 {
    e.call(0x00cb_0c00, &args![object.addr()]).u32()
}

// Translated from 00dfbc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011ba388): cdecl with two stack words, `object` and `finishing`. When `object` is not null, runs `00cb0a70` (the `CALL` at 00dfbc40) (thiscall, `RET 4`) on `object + 0x4` with the one stack word `finishing`.
pub fn fn_00dfbc30(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.call(0x00cb_0a70, &args![object.addr() + 4, finishing]);
    }
}

// Translated from 00dfbc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011ba38c): cdecl with one stack word, `object`. Jumps to `00cb0c00` (the `JMP` at 00dfbc77) (`hkStringPtr::~hkStringPtr` in Ghidra, fastcall, no stack word) with `object + 0x4` as `this`, so it answers what that answers.
pub fn fn_00dfbc70(e: &mut Engine, object: Ptr) -> u32 {
    e.call(0x00cb_0c00, &args![object.addr() + 4]).u32()
}

// Translated from 00dfbc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011ba3a0): cdecl with two stack words, `object` and `finishing`. When `object` is not null, runs `00cb0a70` (the `CALL` at 00dfbc8d) (thiscall, `RET 4`) on `object` with the one stack word `finishing`.
pub fn fn_00dfbc80(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.call(0x00cb_0a70, &args![object.addr(), finishing]);
    }
}

// Translated from 00dfbca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011ba3a4): cdecl with one stack word, `object`. Jumps to `00dfb5a0` (the `JMP` at 00dfbca4) (`hkaAnnotationTrack::~hkaAnnotationTrack` in Ghidra, fastcall, no stack word) with `object` as `this`, so it answers what that answers.
pub fn fn_00dfbca0(e: &mut Engine, object: Ptr) -> u32 {
    e.call(0x00df_b5a0, &args![object.addr()]).u32()
}

// Translated from 00dfbcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011ba48c): cdecl with one stack word (never read); only returns.
pub fn fn_00dfbcb0(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfbcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc19b9): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00dfbcd0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfbcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc19af): cdecl with two stack words, `object` and `finishing`. When `object` is not null: writes the vtable word 0x010d_ec04 at its start and, when `finishing` is not 0, the byte 0 at `+9`; then the vtable word 0x010d_ec3c, the words 0x010c_925c at `+0x24`, 0x010d_ebf4 at `+0x1c` and 0x010d_ebe4 at `+0x24` (the last replaces the first), and, when `finishing` is not 0, the byte 2 at `+9`.
pub fn fn_00dfbcf0(e: &mut Engine, object: Ptr, finishing: u32) {
    let base = object.addr();
    if base == 0 {
        return;
    }
    e.mem.set_u32(base, 0x010d_ec04);
    if finishing != 0 {
        e.mem.set_u8(base + 9, 0);
    }
    e.mem.set_u32(base, 0x010d_ec3c);
    e.mem.set_u32(base + 0x24, 0x010c_925c);
    e.mem.set_u32(base + 0x1c, 0x010d_ebf4);
    e.mem.set_u32(base + 0x24, 0x010d_ebe4);
    if finishing != 0 {
        e.mem.set_u8(base + 9, 2);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dfbd30, return_constant_00dfbd30() -> u32),
        entry!(0x00df_bd40, deleting_destructor_00dfbd40(Ptr, u8) -> Ptr),
        entry!(0x00df_bd70, fn_00dfbd70(Ptr, u8) -> Ptr),
        entry!(0x00dfbbf0, fn_00dfbbf0(Ptr, u32)),
        entry!(0x00dfbc20, fn_00dfbc20(Ptr) -> u32),
        entry!(0x00dfbc30, fn_00dfbc30(Ptr, u32)),
        entry!(0x00dfbc70, fn_00dfbc70(Ptr) -> u32),
        entry!(0x00dfbc80, fn_00dfbc80(Ptr, u32)),
        entry!(0x00dfbca0, fn_00dfbca0(Ptr) -> u32),
        entry!(0x00dfbcb0, fn_00dfbcb0(u32)),
        entry!(0x00dfbcd0, fn_00dfbcd0(Ptr) -> u32),
        entry!(0x00dfbcf0, fn_00dfbcf0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00dfbd40_frees_only_when_bit_zero_is_set() {
        check(0x00df_bd40, 0x00e3_60f0);
    }

    #[test]
    fn return_constant_00dfbd30_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_bd30, &args![]).u32(), 0x010d_ec3c);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// The thunk runs the deleting destructor on `this - 8` (the base destructor `00e360f0`
    /// is a double) and returns that address; without bit 0 nothing is freed.
    #[test]
    fn fn_00dfbd70_runs_the_destructor_on_this_minus_eight() {
        let mut e = Engine::new();
        e.register(0x00e3_60f0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20) + 8;
        let got = e.call(0x00df_bd70, &args![this, 0u32]).u32();
        assert_eq!(got, this - 8);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == 0x00e3_60f0).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![this - 8]);
        assert!(log.iter().all(|(a, _)| *a != 0x00c8_5750));
    }
}

#[cfg(test)]
mod tests_a07_gap {
    use super::*;

    /// The argument words of every call to `address` in the engine's call log (which is taken).
    fn calls_to(e: &mut Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words)
            .collect()
    }

    /// The address the doubled virtual slot 0 of [`object_with_slot_0`] answers at.
    const SLOT_0_TARGET: u32 = 0x0900_0001;

    /// An object whose vtable's slot 0 is a double answering 0x1234.
    fn object_with_slot_0(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable, SLOT_0_TARGET);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        e.register(SLOT_0_TARGET, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        object
    }

    #[test]
    fn fn_00dfbbf0_runs_the_callee_unless_the_object_is_null() {
        let mut e = Engine::new();
        e.register(0x00cb_0a70, |_, _| Ret::default());
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00df_bbf0, &args![0u32, 7u32]);
        e.call(0x00df_bbf0, &args![object, 7u32]);
        assert_eq!(calls_to(&mut e, 0x00cb_0a70), vec![vec![object, 7]]);
    }

    #[test]
    fn fn_00dfbc20_hands_the_object_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x00cb_0c00, |_, _| Ret {
            eax: 0x55,
            ..Ret::default()
        });
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_bc20, &args![object]).u32(), 0x55);
        assert_eq!(calls_to(&mut e, 0x00cb_0c00), vec![vec![object]]);
    }

    #[test]
    fn fn_00dfbc30_runs_the_callee_unless_the_object_is_null() {
        let mut e = Engine::new();
        e.register(0x00cb_0a70, |_, _| Ret::default());
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00df_bc30, &args![0u32, 7u32]);
        e.call(0x00df_bc30, &args![object, 7u32]);
        assert_eq!(calls_to(&mut e, 0x00cb_0a70), vec![vec![object + 4, 7]]);
    }

    #[test]
    fn fn_00dfbc70_hands_the_object_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x00cb_0c00, |_, _| Ret {
            eax: 0x55,
            ..Ret::default()
        });
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_bc70, &args![object]).u32(), 0x55);
        assert_eq!(calls_to(&mut e, 0x00cb_0c00), vec![vec![object + 4]]);
    }

    #[test]
    fn fn_00dfbc80_runs_the_callee_unless_the_object_is_null() {
        let mut e = Engine::new();
        e.register(0x00cb_0a70, |_, _| Ret::default());
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00df_bc80, &args![0u32, 7u32]);
        e.call(0x00df_bc80, &args![object, 7u32]);
        assert_eq!(calls_to(&mut e, 0x00cb_0a70), vec![vec![object, 7]]);
    }

    #[test]
    fn fn_00dfbca0_hands_the_object_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x00df_b5a0, |_, _| Ret {
            eax: 0x55,
            ..Ret::default()
        });
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_bca0, &args![object]).u32(), 0x55);
        assert_eq!(calls_to(&mut e, 0x00df_b5a0), vec![vec![object]]);
    }

    #[test]
    fn fn_00dfbcb0_only_returns() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_bcb0, &args![0u32]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_00dfbcd0_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_bcd0, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00dfbcf0_writes_the_vtable_words_and_marks_the_byte_when_finishing() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.call(0x00df_bcf0, &args![0u32, 1u32]);
        for finishing in [0u32, 1] {
            e.mem.set_u8(object + 9, 0x77);
            e.call(0x00df_bcf0, &args![object, finishing]);
            assert_eq!(e.mem.u32(object), 0x010d_ec3c);
            assert_eq!(e.mem.u32(object + 0x1c), 0x010d_ebf4);
            assert_eq!(e.mem.u32(object + 0x24), 0x010d_ebe4);
            assert_eq!(e.mem.u8(object + 9), if finishing == 0 { 0x77 } else { 2 });
        }
    }
}
