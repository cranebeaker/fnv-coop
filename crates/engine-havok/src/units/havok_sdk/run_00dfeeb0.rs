//! `Havok SDK/run_00dfeeb0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfeeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_62a4.
pub fn return_constant_00dfeeb0(_e: &mut Engine) -> u32 {
    0x010e_62a4
}

// Translated from 00dfeef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_6394.
pub fn return_constant_00dfeef0(_e: &mut Engine) -> u32 {
    0x010e_6394
}

// Translated from 00dfef40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_ebf4.
pub fn return_constant_00dfef40(_e: &mut Engine) -> u32 {
    0x010d_ebf4
}

// Translated from 00dfef90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x60-byte frame stores the vtable word 0x010c_c2fc at
/// its start, runs the constructor at `00cb0a70` on the member at frame+0x14 (argument 0) and
/// returns the constant 0x010d_08f4 (the stored word is not read back). Nothing is destroyed.
pub fn fn_00dfef90(e: &mut Engine) -> u32 {
    e.with_stack(0x60, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, 0x010c_c2fc);
        e.call(0x00cb_0a70, &args![base + 0x14, 0u32]);
        0x010d_08f4
    })
}

// Translated from 00dff000 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_66fc.
pub fn return_constant_00dff000(_e: &mut Engine) -> u32 {
    0x010e_66fc
}

// Translated from 00dfef50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010cc2fc`, runs
/// `00cb0a70` (an `hkStringPtr` function, `this` = `object + 0x14`, one stack word `extra`) and
/// writes the vtable word `010d08f4`. The function takes two stack words and pops none (`RET`),
/// and returns nothing. The class is unconfirmed.
pub fn fn_00dfef50(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010c_c2fc);
        e.call(0x00cb_0a70, &args![object.addr() + 0x14, extra]);
        e.mem.set_u32(object.addr(), 0x010d_08f4);
    }
}

// Translated from 00dfee80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc4539): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00dfee80(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfeea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc452f): cdecl with two stack words, `object` and `finishing` (never read). When `object` is not null, stores the vtable word 0x010e_62a4 at its start.
pub fn fn_00dfeea0(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010e_62a4);
    }
}

// Translated from 00dfeec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc45a9): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00dfeec0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfeee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc459f): cdecl with two stack words, `object` and `finishing` (never read). When `object` is not null, stores the vtable word 0x010e_6394 at its start.
pub fn fn_00dfeee0(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010e_6394);
    }
}

// Translated from 00dfef00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc4619): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00dfef00(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfef80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc4699): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00dfef80(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfefd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc4709): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00dfefd0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfeff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc46ff): cdecl with two stack words, `object` and `finishing` (never read). When `object` is not null, stores the vtable word 0x010e_66fc at its start.
pub fn fn_00dfeff0(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010e_66fc);
    }
}

// Translated from 00dfef20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc460f): cdecl with two stack words, `object` and `finishing` (never read). When `object` is not null, writes the word 0x010c_925c at `object + 8`, then the vtable word 0x010d_ebf4 at its start and then 0x010d_ebe4 at `object + 8` (the last write replaces the first).
pub fn fn_00dfef20(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr() + 8, 0x010c_925c);
        e.mem.set_u32(object.addr(), 0x010d_ebf4);
        e.mem.set_u32(object.addr() + 8, 0x010d_ebe4);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dfeeb0, return_constant_00dfeeb0() -> u32),
        entry!(0x00dfeef0, return_constant_00dfeef0() -> u32),
        entry!(0x00dfef40, return_constant_00dfef40() -> u32),
        entry!(0x00dfef90, fn_00dfef90() -> u32),
        entry!(0x00dff000, return_constant_00dff000() -> u32),
        entry!(0x00df_ef50, fn_00dfef50(Ptr, u32)),
        entry!(0x00dfee80, fn_00dfee80(Ptr) -> u32),
        entry!(0x00dfeea0, fn_00dfeea0(Ptr, u32)),
        entry!(0x00dfeec0, fn_00dfeec0(Ptr) -> u32),
        entry!(0x00dfeee0, fn_00dfeee0(Ptr, u32)),
        entry!(0x00dfef00, fn_00dfef00(Ptr) -> u32),
        entry!(0x00dfef80, fn_00dfef80(Ptr) -> u32),
        entry!(0x00dfefd0, fn_00dfefd0(Ptr) -> u32),
        entry!(0x00dfeff0, fn_00dfeff0(Ptr, u32)),
        entry!(0x00dfef20, fn_00dfef20(Ptr, u32)),
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
    fn return_constant_00dfeeb0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_eeb0, &args![]).u32(), 0x010e_62a4);
    }

    #[test]
    fn return_constant_00dfeef0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_eef0, &args![]).u32(), 0x010e_6394);
    }

    #[test]
    fn return_constant_00dfef40_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_ef40, &args![]).u32(), 0x010d_ebf4);
    }

    #[test]
    fn fn_00dfef90_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x14], false);
        let result = e.call(0x00df_ef90, &args![]).u32();
        assert_eq!(result, 0x010d_08f4);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010c_c2fc);
        }
    }

    #[test]
    fn return_constant_00dff000_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_f000, &args![]).u32(), 0x010e_66fc);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// A null object does nothing; otherwise the vtable word ends at the second value and the
    /// string function runs in between on `object + 0x14`, seeing the first value.
    #[test]
    fn fn_00dfef50_runs_the_string_function_between_the_vtable_writes() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x20);
        e.register_double(0x00cb_0a70, move |e, a| {
            assert_eq!(a, [object + 0x14, 7]);
            assert_eq!(e.mem.u32(object), 0x010c_c2fc);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00df_ef50, &args![0u32, 7u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.call(0x00df_ef50, &args![object, 7u32]);
        assert_eq!(e.mem.u32(object), 0x010d_08f4);
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
    fn fn_00dfee80_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_ee80, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00dfeea0_stores_the_vtable_word_unless_the_object_is_null() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.call(0x00df_eea0, &args![0u32, 1u32]);
        assert_eq!(e.mem.u32(object), 0);
        e.call(0x00df_eea0, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010e_62a4);
    }

    #[test]
    fn fn_00dfeec0_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_eec0, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00dfeee0_stores_the_vtable_word_unless_the_object_is_null() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.call(0x00df_eee0, &args![0u32, 1u32]);
        assert_eq!(e.mem.u32(object), 0);
        e.call(0x00df_eee0, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010e_6394);
    }

    #[test]
    fn fn_00dfef00_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_ef00, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00dfef80_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_ef80, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00dfefd0_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_efd0, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00dfeff0_stores_the_vtable_word_unless_the_object_is_null() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.call(0x00df_eff0, &args![0u32, 1u32]);
        assert_eq!(e.mem.u32(object), 0);
        e.call(0x00df_eff0, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010e_66fc);
    }

    #[test]
    fn fn_00dfef20_writes_the_vtable_words_unless_the_object_is_null() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.call(0x00df_ef20, &args![0u32, 1u32]);
        assert_eq!(e.mem.u32(object), 0);
        e.call(0x00df_ef20, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010d_ebf4);
        assert_eq!(e.mem.u32(object + 8), 0x010d_ebe4);
    }
}
