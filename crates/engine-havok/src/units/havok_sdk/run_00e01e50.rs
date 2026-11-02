//! `Havok SDK/run_00e01e50` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e01e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x20-byte stack buffer by calling `00e4ad00` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00e01e50(e: &mut Engine) -> u32 {
    e.with_stack(0x20, |e, buffer| {
        e.call(0x00e4_ad00, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00e01eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x2c-byte frame stores the vtable word 0x010e_9acc at
/// +0xC, runs the constructor at `00cb0a70` (argument 0) on the members at frame+0x20, 0x24 in that
/// order and returns the vtable word. Nothing is destroyed.
pub fn fn_00e01eb0(e: &mut Engine) -> u32 {
    e.with_stack(0x2c, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base + 0xc, 0x010e_9acc);
        e.call(0x00cb_0a70, &args![base + 0x20, 0u32]);
        e.call(0x00cb_0a70, &args![base + 0x24, 0u32]);
        e.mem.u32(base + 0xc)
    })
}

// Translated from 00e01e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010e9acc` and
/// runs `00cb0a70` (an `hkStringPtr` function, one stack word `extra`) on `object + 0x14` and
/// then on `object + 0x18`. The function takes two stack words and pops none (`RET`), and
/// returns nothing. The class is unconfirmed.
pub fn fn_00e01e80(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010e_9acc);
        e.call(0x00cb_0a70, &args![object.addr() + 0x14, extra]);
        e.call(0x00cb_0a70, &args![object.addr() + 0x18, extra]);
    }
}

// Translated from 00e01e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word `object`): calls the object's virtual function at
/// vtable offset 0 (the destructor) with the one stack word 0 and answers what it answers. It is
/// the cleanup function of a type-info record (word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject`
/// calls it with `(object)`): the start-up initializer at `00fc69b9` stores its address there.
pub fn fn_00e01e00(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00e01e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words `(object, extra)`): with a non-null `object` runs
/// `00e4ad00` on it with `extra` (`thiscall`, one stack word); otherwise does nothing. It is the
/// finish function of a type-info record (word `+4`): the start-up initializer at `00fc6a1f` stores
/// its address there.
pub fn fn_00e01e20(e: &mut Engine, object: Ptr, extra: u32) {
    if !object.is_null() {
        e.call(0x00e4_ad00, &args![object, extra]);
    }
}

// Translated from 00e01e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word `object`): calls the object's virtual function at
/// vtable offset 0 (the destructor) with the one stack word 0 and answers what it answers. It is
/// the cleanup function of a type-info record (word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject`
/// calls it with `(object)`): the start-up initializer at `00fc6a29` stores its address there.
pub fn fn_00e01e40(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e01e00, fn_00e01e00(Ptr) -> u32),
        entry!(0x00e01e20, fn_00e01e20(Ptr, u32)),
        entry!(0x00e01e40, fn_00e01e40(Ptr) -> u32),
        entry!(0x00e0_1e50, fn_00e01e50() -> u32),
        entry!(0x00e01eb0, fn_00e01eb0() -> u32),
        entry!(0x00e0_1e80, fn_00e01e80(Ptr, u32)),
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
        fn_00e01e50_returns_first_word_of_built_object,
        0x00e0_1e50,
        0x00e4_ad00
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn fn_00e01e00_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        let object = e.mem.alloc(8);
        e.put_vtable(0x0900_0000, &[0x0900_0100]);
        e.mem.set_u32(object, 0x0900_0000);
        e.register(0x0900_0100, |_, a| Ret {
            eax: a[0].wrapping_add(a[1]).wrapping_add(7),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let answer = e.call(0x00e0_1e00, &args![object]).u32();
        assert_eq!(answer, object + 7);
        let seen: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x0900_0100)
            .cloned()
            .collect();
        assert_eq!(seen, vec![(0x0900_0100, vec![object, 0])]);
    }

    #[test]
    fn fn_00e01e20_runs_the_callee_on_a_non_null_object() {
        let mut e = Engine::new();
        e.register(0x00e4_ad00, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00e0_1e20, &args![0x1234u32, 0x55u32]);
        let seen: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x00e4_ad00)
            .cloned()
            .collect();
        assert_eq!(seen, vec![(0x00e4_ad00, vec![0x1234, 0x55])]);
    }

    #[test]
    fn fn_00e01e20_does_nothing_for_a_null_object() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00e0_1e20, &args![0u32, 0x55u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn fn_00e01e40_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        let object = e.mem.alloc(8);
        e.put_vtable(0x0900_0000, &[0x0900_0100]);
        e.mem.set_u32(object, 0x0900_0000);
        e.register(0x0900_0100, |_, a| Ret {
            eax: a[0].wrapping_add(a[1]).wrapping_add(7),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let answer = e.call(0x00e0_1e40, &args![object]).u32();
        assert_eq!(answer, object + 7);
        let seen: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x0900_0100)
            .cloned()
            .collect();
        assert_eq!(seen, vec![(0x0900_0100, vec![object, 0])]);
    }

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
    fn fn_00e01e50_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e4_ad00, 0x0a0b_0c0d);
        let result = e.call(0x00e0_1e50, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00e01eb0_stores_the_vtable_and_constructs_its_members() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x14, 0x18], false);
        let result = e.call(0x00e0_1eb0, &args![]).u32();
        assert_eq!(result, 0x010e_9acc);
        let calls = log.borrow();
        assert_eq!(calls.len(), 2);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010e_9acc);
        }
        assert_eq!(calls[1].0 - calls[0].0, 0x4);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A null object does nothing; otherwise the string function runs on both members.
    #[test]
    fn fn_00e01e80_runs_the_string_function_twice() {
        let mut e = Engine::new();
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let string_log = Rc::clone(&log);
        e.register_double(0x00cb_0a70, move |_, a| {
            string_log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(0x00e0_1e80, &args![0u32, 4u32]);
        assert!(log.borrow().is_empty());
        let object = e.mem.alloc(0x30);
        e.call(0x00e0_1e80, &args![object, 4u32]);
        assert_eq!(e.mem.u32(object), 0x010e_9acc);
        assert_eq!(
            *log.borrow(),
            vec![vec![object + 0x14, 4], vec![object + 0x18, 4]]
        );
    }
}
