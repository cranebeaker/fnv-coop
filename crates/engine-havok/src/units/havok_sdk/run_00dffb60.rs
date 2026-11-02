//! `Havok SDK/run_00dffb60` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dffb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x30` (`SUB ECX,0x30`) and jumps to `00dffb00`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00dffb60(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_fb00, &args![this.addr().wrapping_sub(0x30), flags])
        .u32()
}

// Translated from 00dffb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x14` (`SUB ECX,0x14`) and jumps to `00dffb00`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00dffb70(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_fb00, &args![this.addr().wrapping_sub(0x14), flags])
        .u32()
}

// Translated from 00dffbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x60-byte stack buffer by calling `00e46f20` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dffbb0(e: &mut Engine) -> u32 {
    e.with_stack(0x60, |e, buffer| {
        e.call(0x00e4_6f20, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dffc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x40-byte frame stores the vtable word 0x010c_c2fc at
/// its start, runs the constructor at `00cb0a70` on the member at frame+0x14 (argument 0) and
/// returns the constant 0x010d_09f4 (the stored word is not read back). Nothing is destroyed.
pub fn fn_00dffc20(e: &mut Engine) -> u32 {
    e.with_stack(0x40, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, 0x010c_c2fc);
        e.call(0x00cb_0a70, &args![base + 0x14, 0u32]);
        0x010d_09f4
    })
}

// Translated from 00dffc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_7c24.
pub fn return_constant_00dffc90(_e: &mut Engine) -> u32 {
    0x010e_7c24
}

// Translated from 00dffbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010cc2fc`, runs
/// `00cb0a70` (an `hkStringPtr` function, `this` = `object + 0x14`, one stack word `extra`) and
/// writes the vtable word `010d09f4`. The function takes two stack words and pops none (`RET`),
/// and returns nothing. The class is unconfirmed.
pub fn fn_00dffbe0(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010c_c2fc);
        e.call(0x00cb_0a70, &args![object.addr() + 0x14, extra]);
        e.mem.set_u32(object.addr(), 0x010d_09f4);
    }
}

// Translated from 00dffb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc4edf): cdecl with two stack words, `object` and `finishing`. When `object` is not null, runs `00e46f20` (the `CALL` at 00dffb8d) (thiscall, `RET 4`) on `object` with the one stack word `finishing`.
pub fn fn_00dffb80(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.call(0x00e4_6f20, &args![object.addr(), finishing]);
    }
}

// Translated from 00dffba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc4ee9): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00dffba0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dffc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc4f59): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00dffc10(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dffc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc4fc9): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00dffc60(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dffc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc4fbf): cdecl with two stack words, `object` and `finishing` (never read). When `object` is not null, stores the vtable word 0x010e_7c24 at its start.
pub fn fn_00dffc80(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010e_7c24);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_fbb0, fn_00dffbb0() -> u32),
        entry!(0x00df_fb60, fn_00dffb60(Ptr, u32) -> u32),
        entry!(0x00df_fb70, fn_00dffb70(Ptr, u32) -> u32),
        entry!(0x00dffc20, fn_00dffc20() -> u32),
        entry!(0x00dffc90, return_constant_00dffc90() -> u32),
        entry!(0x00df_fbe0, fn_00dffbe0(Ptr, u32)),
        entry!(0x00dffb80, fn_00dffb80(Ptr, u32)),
        entry!(0x00dffba0, fn_00dffba0(Ptr) -> u32),
        entry!(0x00dffc10, fn_00dffc10(Ptr) -> u32),
        entry!(0x00dffc60, fn_00dffc60(Ptr) -> u32),
        entry!(0x00dffc80, fn_00dffc80(Ptr, u32)),
    ]
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00dffb60() {
        assert_eq!(check(0x00df_fb60, 0x00df_fb00, 0x30), 0xdead_beef);
    }

    #[test]
    fn thunk_00dffb70() {
        assert_eq!(check(0x00df_fb70, 0x00df_fb00, 0x14), 0xdead_beef);
    }
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
        fn_00dffbb0_returns_first_word_of_built_object,
        0x00df_fbb0,
        0x00e4_6f20
    );
}

#[cfg(test)]
mod record_tests {
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
    fn fn_00dffbb0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e4_6f20, 0x0a0b_0c0d);
        let result = e.call(0x00df_fbb0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn fn_00dffc20_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x14], false);
        let result = e.call(0x00df_fc20, &args![]).u32();
        assert_eq!(result, 0x010d_09f4);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010c_c2fc);
        }
    }

    #[test]
    fn return_constant_00dffc90_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_fc90, &args![]).u32(), 0x010e_7c24);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// A null object does nothing; otherwise the vtable word ends at the second value and the
    /// string function runs in between on `object + 0x14`, seeing the first value.
    #[test]
    fn fn_00dffbe0_runs_the_string_function_between_the_vtable_writes() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x20);
        e.register_double(0x00cb_0a70, move |e, a| {
            assert_eq!(a, [object + 0x14, 7]);
            assert_eq!(e.mem.u32(object), 0x010c_c2fc);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00df_fbe0, &args![0u32, 7u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.call(0x00df_fbe0, &args![object, 7u32]);
        assert_eq!(e.mem.u32(object), 0x010d_09f4);
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
    fn fn_00dffb80_runs_the_callee_unless_the_object_is_null() {
        let mut e = Engine::new();
        e.register(0x00e4_6f20, |_, _| Ret::default());
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00df_fb80, &args![0u32, 7u32]);
        e.call(0x00df_fb80, &args![object, 7u32]);
        assert_eq!(calls_to(&mut e, 0x00e4_6f20), vec![vec![object, 7]]);
    }

    #[test]
    fn fn_00dffba0_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_fba0, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00dffc10_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_fc10, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00dffc60_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_fc60, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00dffc80_stores_the_vtable_word_unless_the_object_is_null() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.call(0x00df_fc80, &args![0u32, 1u32]);
        assert_eq!(e.mem.u32(object), 0);
        e.call(0x00df_fc80, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010e_7c24);
    }
}
