//! `Havok SDK/run_00dfc400` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfc480 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_04ac.
pub fn return_constant_00dfc480(_e: &mut Engine) -> u32 {
    0x010e_04ac
}

// Translated from 00dfc400 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x140-byte frame stores the vtable word 0x010d_77d8 at
/// its start, runs the constructor at `00d21de0` on the member at frame+0x20 (argument 0) and
/// returns the word at the start of the frame. Nothing is destroyed.
pub fn fn_00dfc400(e: &mut Engine) -> u32 {
    e.with_stack(0x140, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, 0x010d_77d8);
        e.call(0x00d2_1de0, &args![base + 0x20, 0u32]);
        e.mem.u32(base)
    })
}

// Translated from 00dfc380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc1f19): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00dfc380(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfc3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011ba8b0): cdecl with one stack word, `object`. Jumps to `00e05c20` (the `JMP` at 00dfc3a4) (fastcall, no stack word) with `object` as `this`, so it answers what that answers.
pub fn fn_00dfc3a0(e: &mut Engine, object: Ptr) -> u32 {
    e.call(0x00e0_5c20, &args![object.addr()]).u32()
}

// Translated from 00dfc3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011ba8c8): cdecl with one stack word, `object`. Jumps to `00e05c20` (the `JMP` at 00dfc3b4) (fastcall, no stack word) with `object` as `this`, so it answers what that answers.
pub fn fn_00dfc3b0(e: &mut Engine, object: Ptr) -> u32 {
    e.call(0x00e0_5c20, &args![object.addr()]).u32()
}

// Translated from 00dfc3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011ba8ac): cdecl with two stack words (never read); only returns.
pub fn fn_00dfc3c0(_e: &mut Engine, _unused_0: u32, _unused_1: u32) {}

// Translated from 00dfc3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011ba8c4): cdecl with two stack words (never read); only returns.
pub fn fn_00dfc3d0(_e: &mut Engine, _unused_0: u32, _unused_1: u32) {}

// Translated from 00dfc3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc1f0f): cdecl with two stack words, `object` and `finishing`. When `object` is not null, stores the vtable word 0x010d_77d8 at its start, then runs `00d21de0` (the `CALL` at 00dfc3f6) (thiscall, `RET 4`) on `object + 0x20` with the one stack word `finishing`.
pub fn fn_00dfc3e0(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010d_77d8);
        e.call(0x00d2_1de0, &args![object.addr() + 32, finishing]);
    }
}

// Translated from 00dfc430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011baa88): cdecl with one stack word (never read); only returns.
pub fn fn_00dfc430(_e: &mut Engine, _unused_0: u32) {}

// Translated from 00dfc450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc1fb9): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00dfc450(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfc470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc1faf): cdecl with two stack words, `object` and `finishing` (never read). When `object` is not null, stores the vtable word 0x010e_04ac at its start.
pub fn fn_00dfc470(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010e_04ac);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dfc480, return_constant_00dfc480() -> u32),
        entry!(0x00dfc400, fn_00dfc400() -> u32),
        entry!(0x00dfc380, fn_00dfc380(Ptr) -> u32),
        entry!(0x00dfc3a0, fn_00dfc3a0(Ptr) -> u32),
        entry!(0x00dfc3b0, fn_00dfc3b0(Ptr) -> u32),
        entry!(0x00dfc3c0, fn_00dfc3c0(u32, u32)),
        entry!(0x00dfc3d0, fn_00dfc3d0(u32, u32)),
        entry!(0x00dfc3e0, fn_00dfc3e0(Ptr, u32)),
        entry!(0x00dfc430, fn_00dfc430(u32)),
        entry!(0x00dfc450, fn_00dfc450(Ptr) -> u32),
        entry!(0x00dfc470, fn_00dfc470(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00dfc480_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_c480, &args![]).u32(), 0x010e_04ac);
    }
}

#[cfg(test)]
mod record_tests {
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
    fn fn_00dfc400_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00d2_1de0, &[0x20], false);
        let result = e.call(0x00df_c400, &args![]).u32();
        assert_eq!(result, 0x010d_77d8);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010d_77d8);
        }
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
    fn fn_00dfc380_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_c380, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00dfc3a0_hands_the_object_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x00e0_5c20, |_, _| Ret {
            eax: 0x55,
            ..Ret::default()
        });
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_c3a0, &args![object]).u32(), 0x55);
        assert_eq!(calls_to(&mut e, 0x00e0_5c20), vec![vec![object]]);
    }

    #[test]
    fn fn_00dfc3b0_hands_the_object_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x00e0_5c20, |_, _| Ret {
            eax: 0x55,
            ..Ret::default()
        });
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_c3b0, &args![object]).u32(), 0x55);
        assert_eq!(calls_to(&mut e, 0x00e0_5c20), vec![vec![object]]);
    }

    #[test]
    fn fn_00dfc3c0_only_returns() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_c3c0, &args![0u32, 0u32]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_00dfc3d0_only_returns() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_c3d0, &args![0u32, 0u32]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_00dfc3e0_stores_the_vtable_and_runs_the_callee_unless_the_object_is_null() {
        let mut e = Engine::new();
        e.register(0x00d2_1de0, |_, _| Ret::default());
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00df_c3e0, &args![0u32, 7u32]);
        assert_eq!(e.mem.u32(object), 0);
        e.call(0x00df_c3e0, &args![object, 7u32]);
        assert_eq!(e.mem.u32(object), 0x010d_77d8);
        assert_eq!(calls_to(&mut e, 0x00d2_1de0), vec![vec![object + 32, 7]]);
    }

    #[test]
    fn fn_00dfc430_only_returns() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_c430, &args![0u32]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_00dfc450_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_c450, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00dfc470_stores_the_vtable_word_unless_the_object_is_null() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.call(0x00df_c470, &args![0u32, 1u32]);
        assert_eq!(e.mem.u32(object), 0);
        e.call(0x00df_c470, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010e_04ac);
    }
}
