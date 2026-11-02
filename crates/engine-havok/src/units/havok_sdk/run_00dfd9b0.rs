//! `Havok SDK/run_00dfd9b0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfd9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x80-byte frame stores the vtable word 0x010e_33dc at
/// its start, runs the constructor at `00cb0a70` on the member at frame+0x14 (argument 0) and
/// returns the word at the start of the frame. Nothing is destroyed.
pub fn fn_00dfd9b0(e: &mut Engine) -> u32 {
    e.with_stack(0x80, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, 0x010e_33dc);
        e.call(0x00cb_0a70, &args![base + 0x14, 0u32]);
        e.mem.u32(base)
    })
}

// Translated from 00dfd940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc34c9): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00dfd940(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfd960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011bbaa8): cdecl with two stack words (never read); only returns.
pub fn fn_00dfd960(_e: &mut Engine, _unused_0: u32, _unused_1: u32) {}

// Translated from 00dfd990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc34bf): cdecl with two stack words, `object` and `finishing`. When `object` is not null, stores the vtable word 0x010e_33dc at its start, then runs `00cb0a70` (the `CALL` at 00dfd9a6) (thiscall, `RET 4`) on `object + 0x14` with the one stack word `finishing`.
pub fn fn_00dfd990(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010e_33dc);
        e.call(0x00cb_0a70, &args![object.addr() + 20, finishing]);
    }
}

// Translated from 00dfd970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address held in the static data word at 011bbaac): cdecl with one stack word, `object`. When the word at its start is not 0, releases it (`00c905b0`, fastcall); then clears that word.
pub fn fn_00dfd970(e: &mut Engine, object: Ptr) {
    let referenced = e.mem.u32(object.addr());
    if referenced != 0 {
        e.call(0x00c9_05b0, &args![referenced]);
    }
    e.mem.set_u32(object.addr(), 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dfd9b0, fn_00dfd9b0() -> u32),
        entry!(0x00dfd940, fn_00dfd940(Ptr) -> u32),
        entry!(0x00dfd960, fn_00dfd960(u32, u32)),
        entry!(0x00dfd990, fn_00dfd990(Ptr, u32)),
        entry!(0x00dfd970, fn_00dfd970(Ptr)),
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
    fn fn_00dfd9b0_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x14], false);
        let result = e.call(0x00df_d9b0, &args![]).u32();
        assert_eq!(result, 0x010e_33dc);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010e_33dc);
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
    fn fn_00dfd940_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_d940, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00dfd960_only_returns() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_d960, &args![0u32, 0u32]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_00dfd990_stores_the_vtable_and_runs_the_callee_unless_the_object_is_null() {
        let mut e = Engine::new();
        e.register(0x00cb_0a70, |_, _| Ret::default());
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00df_d990, &args![0u32, 7u32]);
        assert_eq!(e.mem.u32(object), 0);
        e.call(0x00df_d990, &args![object, 7u32]);
        assert_eq!(e.mem.u32(object), 0x010e_33dc);
        assert_eq!(calls_to(&mut e, 0x00cb_0a70), vec![vec![object + 20, 7]]);
    }

    #[test]
    fn fn_00dfd970_releases_the_held_object_and_clears_the_word() {
        let mut e = Engine::new();
        e.register(0x00c9_05b0, |_, _| Ret::default());
        let object = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x00df_d970, &args![object]);
        assert!(calls_to(&mut e, 0x00c9_05b0).is_empty());
        e.mem.set_u32(object, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x00df_d970, &args![object]);
        assert_eq!(calls_to(&mut e, 0x00c9_05b0), vec![vec![0x1234]]);
        assert_eq!(e.mem.u32(object), 0);
    }
}
