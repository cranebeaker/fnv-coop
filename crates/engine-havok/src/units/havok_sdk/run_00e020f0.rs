//! `Havok SDK/run_00e020f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e020f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x20-byte stack buffer by calling `00e4ae80` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00e020f0(e: &mut Engine) -> u32 {
    e.with_stack(0x20, |e, buffer| {
        e.call(0x00e4_ae80, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00e02180 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x3c-byte frame stores the vtable word 0x010e_9ccc at
/// +0xC, runs the constructor at `00cb0a70` (argument 0) on the members at frame+0x14, 0x1c, 0x20, 0x24 in that
/// order and returns the vtable word. Nothing is destroyed.
pub fn fn_00e02180(e: &mut Engine) -> u32 {
    e.with_stack(0x3c, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base + 0xc, 0x010e_9ccc);
        e.call(0x00cb_0a70, &args![base + 0x14, 0u32]);
        e.call(0x00cb_0a70, &args![base + 0x1c, 0u32]);
        e.call(0x00cb_0a70, &args![base + 0x20, 0u32]);
        e.call(0x00cb_0a70, &args![base + 0x24, 0u32]);
        e.mem.u32(base + 0xc)
    })
}

// Translated from 00e02140 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010e9ccc` and
/// runs `00cb0a70` (an `hkStringPtr` function, one stack word `extra`) on `object + 8`, then on
/// `object + 0x10`, `object + 0x14` and `object + 0x18`. The function takes two stack words and
/// pops none (`RET`), and returns nothing. The class is unconfirmed.
pub fn fn_00e02140(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010e_9ccc);
        e.call(0x00cb_0a70, &args![object.addr() + 0x8, extra]);
        e.call(0x00cb_0a70, &args![object.addr() + 0x10, extra]);
        e.call(0x00cb_0a70, &args![object.addr() + 0x14, extra]);
        e.call(0x00cb_0a70, &args![object.addr() + 0x18, extra]);
    }
}

// Translated from 00e020c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc6aff): cdecl with two stack words, `object` and `finishing`. When `object` is not null, runs `00e4ae80` (the `CALL` at 00e020cd) (thiscall, `RET 4`) on `object` with the one stack word `finishing`.
pub fn fn_00e020c0(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.call(0x00e4_ae80, &args![object.addr(), finishing]);
    }
}

// Translated from 00e020e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc6b09): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00e020e0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00e02120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc6b99): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00e02120(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0_20f0, fn_00e020f0() -> u32),
        entry!(0x00e02180, fn_00e02180() -> u32),
        entry!(0x00e0_2140, fn_00e02140(Ptr, u32)),
        entry!(0x00e020c0, fn_00e020c0(Ptr, u32)),
        entry!(0x00e020e0, fn_00e020e0(Ptr) -> u32),
        entry!(0x00e02120, fn_00e02120(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod a07_first_word_tests {
    use crate::prelude::*;

    const BUILT_WORD: u32 = 0x0abc_def0;

    /// A double over `callee` writes `BUILT_WORD` at the buffer it is given; `function`
    /// must call it once with that buffer and a zero flag word, and return the word.
    fn check(function: u32, callee: u32) {
        let mut e = Engine::new();
        e.register(callee, |e, a| {
            e.mem.set_u32(a[0], BUILT_WORD);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(function, &args![]).u32();
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == callee).collect();
        assert_eq!(calls.len(), 1);
        assert_ne!(calls[0].1[0], 0);
        assert_eq!(calls[0].1[1], 0);
        assert_eq!(result, BUILT_WORD);
    }

    macro_rules! first_word_test {
        ($name:ident, $function:expr, $callee:expr) => {
            #[test]
            fn $name() {
                check($function, $callee);
            }
        };
    }

    first_word_test!(
        fn_00e020f0_returns_first_word_of_built_object,
        0x00e0_20f0,
        0x00e4_ae80
    );
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
    fn fn_00e020f0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e4_ae80, 0x0a0b_0c0d);
        let result = e.call(0x00e0_20f0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00e02180_stores_the_vtable_and_constructs_its_members() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x8, 0x10, 0x14, 0x18], false);
        let result = e.call(0x00e0_2180, &args![]).u32();
        assert_eq!(result, 0x010e_9ccc);
        let calls = log.borrow();
        assert_eq!(calls.len(), 4);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010e_9ccc);
        }
        assert_eq!(calls[1].0 - calls[0].0, 0x8);
        assert_eq!(calls[2].0 - calls[0].0, 0xc);
        assert_eq!(calls[3].0 - calls[0].0, 0x10);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A null object does nothing; otherwise the string function runs on the four members.
    #[test]
    fn fn_00e02140_runs_the_string_function_four_times() {
        let mut e = Engine::new();
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let string_log = Rc::clone(&log);
        e.register_double(0x00cb_0a70, move |_, a| {
            string_log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(0x00e0_2140, &args![0u32, 4u32]);
        assert!(log.borrow().is_empty());
        let object = e.mem.alloc(0x30);
        e.call(0x00e0_2140, &args![object, 4u32]);
        assert_eq!(e.mem.u32(object), 0x010e_9ccc);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![object + 8, 4],
                vec![object + 0x10, 4],
                vec![object + 0x14, 4],
                vec![object + 0x18, 4]
            ]
        );
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
    fn fn_00e020c0_runs_the_callee_unless_the_object_is_null() {
        let mut e = Engine::new();
        e.register(0x00e4_ae80, |_, _| Ret::default());
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00e0_20c0, &args![0u32, 7u32]);
        e.call(0x00e0_20c0, &args![object, 7u32]);
        assert_eq!(calls_to(&mut e, 0x00e4_ae80), vec![vec![object, 7]]);
    }

    #[test]
    fn fn_00e020e0_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e0_20e0, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00e02120_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e0_2120, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }
}
