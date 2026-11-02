//! `Havok SDK/run_00dfe440` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfe440 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0xa0-byte stack buffer by calling `00e40ca0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfe440(e: &mut Engine) -> u32 {
    e.with_stack(0xa0, |e, buffer| {
        e.call(0x00e4_0ca0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfe4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_56d4.
pub fn return_constant_00dfe4a0(_e: &mut Engine) -> u32 {
    0x010e_56d4
}

// Translated from 00dfe410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc3e0f): cdecl with two stack words, `object` and `finishing`. When `object` is not null, runs `00e40ca0` (the `CALL` at 00dfe41d) (thiscall, `RET 4`) on `object` with the one stack word `finishing`.
pub fn fn_00dfe410(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.call(0x00e4_0ca0, &args![object.addr(), finishing]);
    }
}

// Translated from 00dfe430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc3e19): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00dfe430(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfe470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_cleanupLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc3e89): cdecl with one stack word, `object`. Calls the object's virtual slot 0 with the one stack word 0 (no null check) and answers what it answers.
pub fn fn_00dfe470(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfe490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfo::m_finishLoadedObjectFunction` (Xbox PDB field name) of a class record whose class the engine map does not name (address stored by the code at 00fc3e7f): cdecl with two stack words, `object` and `finishing` (never read). When `object` is not null, stores the vtable word 0x010e_56d4 at its start.
pub fn fn_00dfe490(e: &mut Engine, object: Ptr, _unused_1: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010e_56d4);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_e440, fn_00dfe440() -> u32),
        entry!(0x00dfe4a0, return_constant_00dfe4a0() -> u32),
        entry!(0x00dfe410, fn_00dfe410(Ptr, u32)),
        entry!(0x00dfe430, fn_00dfe430(Ptr) -> u32),
        entry!(0x00dfe470, fn_00dfe470(Ptr) -> u32),
        entry!(0x00dfe490, fn_00dfe490(Ptr, u32)),
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
        fn_00dfe440_returns_first_word_of_built_object,
        0x00df_e440,
        0x00e4_0ca0
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

    #[test]
    fn fn_00dfe440_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e4_0ca0, 0x0a0b_0c0d);
        let result = e.call(0x00df_e440, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00dfe4a0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_e4a0, &args![]).u32(), 0x010e_56d4);
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
    fn fn_00dfe410_runs_the_callee_unless_the_object_is_null() {
        let mut e = Engine::new();
        e.register(0x00e4_0ca0, |_, _| Ret::default());
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00df_e410, &args![0u32, 7u32]);
        e.call(0x00df_e410, &args![object, 7u32]);
        assert_eq!(calls_to(&mut e, 0x00e4_0ca0), vec![vec![object, 7]]);
    }

    #[test]
    fn fn_00dfe430_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_e430, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00dfe470_calls_virtual_slot_0_with_zero() {
        let mut e = Engine::new();
        let object = object_with_slot_0(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00df_e470, &args![object]).u32(), 0x1234);
        assert_eq!(calls_to(&mut e, SLOT_0_TARGET), vec![vec![object, 0]]);
    }

    #[test]
    fn fn_00dfe490_stores_the_vtable_word_unless_the_object_is_null() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        e.call(0x00df_e490, &args![0u32, 1u32]);
        assert_eq!(e.mem.u32(object), 0);
        e.call(0x00df_e490, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010e_56d4);
    }
}
