//! `Havok SDK/run_00e026f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e026f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_a1ac.
pub fn return_constant_00e026f0(_e: &mut Engine) -> u32 {
    0x010e_a1ac
}

// Translated from 00e02730 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): builds a temporary object in a 0x20-byte frame with
/// the constructor at `00e4af80` (`this` = the frame, one argument 0) and returns the word at the
/// start of the object, the vtable pointer the constructor stored there. The temporary is not
/// destroyed.
pub fn fn_00e02730(e: &mut Engine) -> u32 {
    e.with_stack(0x20, |e, record| {
        e.call(0x00e4_af80, &args![record, 0u32]);
        e.mem.u32(record.addr())
    })
}

// Translated from 00e027a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010c_9314.
pub fn return_constant_00e027a0(_e: &mut Engine) -> u32 {
    0x010c_9314
}

// Translated from 00e026c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc7089): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00e026c0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00e026e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc707f): cdecl with two stack words, `object` and `finishing` (never read). When `object` is not null, stores the vtable word 0x010e_a1ac at its start.
pub fn fn_00e026e0(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010e_a1ac);
    }
}

// Translated from 00e02700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc70ef): cdecl with two stack words, `object` and `finishing`. When `object` is not null, runs `00e4af80` (the `CALL` at 00e0270d) (thiscall, `RET 4`) on `object` with the one stack word `finishing`.
pub fn fn_00e02700(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.call(0x00e4_af80, &args![object.addr(), finishing]);
    }
}

// Translated from 00e02720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc70f9): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00e02720(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00e02760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc7169): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00e02760(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00e02780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc715f): cdecl with two stack words, `object` and `finishing` (never read). When `object` is not null, writes the 16-bit value 1 at `+6` and the vtable word 0x010c_9314 at its start.
pub fn fn_00e02780(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u16(object.addr() + 6, 1);
        e.mem.set_u32(object.addr(), 0x010c_9314);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e026f0, return_constant_00e026f0() -> u32),
        entry!(0x00e02730, fn_00e02730() -> u32),
        entry!(0x00e027a0, return_constant_00e027a0() -> u32),
        entry!(0x00e026c0, fn_00e026c0(Ptr) -> u32),
        entry!(0x00e026e0, fn_00e026e0(Ptr, u32)),
        entry!(0x00e02700, fn_00e02700(Ptr, u32)),
        entry!(0x00e02720, fn_00e02720(Ptr) -> u32),
        entry!(0x00e02760, fn_00e02760(Ptr) -> u32),
        entry!(0x00e02780, fn_00e02780(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type ConstructorLog = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Double for a constructor: logs its argument words and writes `vtable` at the start of `this`.
    fn constructor_double(e: &mut Engine, address: u32, vtable: u32) -> ConstructorLog {
        let log: ConstructorLog = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        e.register_double(address, move |e, a| {
            seen.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[0], vtable);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        log
    }

    #[test]
    fn return_constant_00e026f0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_26f0, &args![]).u32(), 0x010e_a1ac);
    }

    #[test]
    fn fn_00e02730_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e4_af80, 0x0a0b_0c0d);
        let result = e.call(0x00e0_2730, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00e027a0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_27a0, &args![]).u32(), 0x010c_9314);
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
    fn fn_00e026c0_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e0_26c0, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00e026e0_stores_the_vtable_word_unless_the_object_is_null() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.call(0x00e0_26e0, &args![0u32, 1u32]);
        assert_eq!(e.mem.u32(object), 0);
        e.call(0x00e0_26e0, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010e_a1ac);
    }

    #[test]
    fn fn_00e02700_runs_the_callee_unless_the_object_is_null() {
        let mut e = Engine::new();
        e.register(0x00e4_af80, |_, _| Ret::default());
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00e0_2700, &args![0u32, 7u32]);
        e.call(0x00e0_2700, &args![object, 7u32]);
        assert_eq!(calls_to(&mut e, 0x00e4_af80), vec![vec![object, 7]]);
    }

    #[test]
    fn fn_00e02720_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e0_2720, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00e02760_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e0_2760, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00e02780_stores_the_count_and_the_vtable_unless_the_object_is_null() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.call(0x00e0_2780, &args![0u32, 1u32]);
        assert_eq!(e.mem.u16(object + 6), 0);
        e.call(0x00e0_2780, &args![object, 1u32]);
        assert_eq!(e.mem.u16(object + 6), 1);
        assert_eq!(e.mem.u32(object), 0x010c_9314);
    }
}
