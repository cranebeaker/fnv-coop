//! `Havok SDK/run_00dff8b0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dff8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x40-byte stack buffer by calling `00dff8d0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dff8b0(e: &mut Engine) -> u32 {
    e.with_stack(0x40, |e, buffer| {
        e.call(0x00df_f8d0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dff8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a constructor chain that the compiler collapsed (`this` in `ECX`;
/// the one stack word is never read, `RET 4`). It writes the words at `+8`, `+0xc`, `+0x10`,
/// `+0x14` and `+0x30` with the base-class values (`010c3b80`, `010c3b98`, `010c3bac`,
/// `010c3b8c`, `0102f13c`), then the vtable word at `+0` (`010e7700`) and the five words again
/// with the final values (`010e76f4`, `010e76e0`, `010e76d4`, `010e76c8`, `010e76ac`). Returns
/// `this` (`EAX`). The class is unconfirmed.
pub fn fn_00dff8d0(e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base + 0x8, 0x010c_3b80);
    e.mem.set_u32(base + 0xc, 0x010c_3b98);
    e.mem.set_u32(base + 0x10, 0x010c_3bac);
    e.mem.set_u32(base + 0x14, 0x010c_3b8c);
    e.mem.set_u32(base + 0x30, 0x0102_f13c);
    e.mem.set_u32(base, 0x010e_7700);
    e.mem.set_u32(base + 0x8, 0x010e_76f4);
    e.mem.set_u32(base + 0xc, 0x010e_76e0);
    e.mem.set_u32(base + 0x10, 0x010e_76d4);
    e.mem.set_u32(base + 0x14, 0x010e_76c8);
    e.mem.set_u32(base + 0x30, 0x010e_76ac);
    this
}

// Translated from 00dff870 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns what it returns (`EAX`). Ghidra made no function here; the address is an immediate operand at 00fc4d89.
pub fn fn_00dff870(e: &mut Engine, object: u32) -> u32 {
    e.vcall(object, 0, &args![0u32]).u32()
}
// Translated from 00dff890 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: two stack words (cdecl, nothing popped): `object` and `extra`. When
/// `object` is not null, runs the function `00dff8d0` on it (`this` in `ECX`) with `extra` (which
/// the callee pops). Returns nothing. Ghidra made no function here; the address is an immediate operand at 00fc4d7f.
pub fn fn_00dff890(e: &mut Engine, object: u32, extra: u32) {
    if object != 0 {
        e.call(0x00df_f8d0, &args![object, extra]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_f8b0, fn_00dff8b0() -> u32),
        entry!(0x00df_f8d0, fn_00dff8d0(Ptr, u32) -> Ptr),
        entry!(0x00df_f870, fn_00dff870(u32) -> u32),
        entry!(0x00df_f890, fn_00dff890(u32, u32)),
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
        fn_00dff8b0_returns_first_word_of_built_object,
        0x00df_f8b0,
        0x00df_f8d0
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
    fn fn_00dff8b0_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00df_f8d0, 0x0a0b_0c0d);
        let result = e.call(0x00df_f8b0, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// The final vtable words are in place and `this` comes back; neighbours are untouched.
    #[test]
    fn fn_00dff8d0_writes_the_final_vtable_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        e.mem.write(this, &[0x33; 0x40]);
        assert_eq!(e.call(0x00df_f8d0, &args![this, 0u32]).u32(), this);
        let words: Vec<u32> = [0x0, 0x8, 0xc, 0x10, 0x14, 0x30]
            .iter()
            .map(|offset| e.mem.u32(this + offset))
            .collect();
        assert_eq!(
            words,
            [
                0x010e_7700,
                0x010e_76f4,
                0x010e_76e0,
                0x010e_76d4,
                0x010e_76c8,
                0x010e_76ac
            ]
        );
        assert_eq!(e.mem.u32(this + 0x4), 0x3333_3333);
        assert_eq!(e.mem.u32(this + 0x18), 0x3333_3333);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The virtual function in slot 0 gets the object and the word 0; its answer comes back.
    #[test]
    fn fn_00dff870_calls_slot_zero_with_zero() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(vtable, 0x0900_0002);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x0900_0002, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret {
                eax: 0x77,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x00df_f870, &args![object]).u32(), 0x77);
        assert_eq!(*seen.borrow(), vec![vec![object, 0]]);
    }
    /// A null object calls nothing; any other is passed on with the extra word.
    #[test]
    fn fn_00dff890_runs_the_member_function_for_a_non_null_object() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00df_f8d0, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(0x00df_f890, &args![0u32, 5u32]);
        assert!(seen.borrow().is_empty());
        e.call(0x00df_f890, &args![0x1000u32, 5u32]);
        assert_eq!(*seen.borrow(), vec![vec![0x1000, 5]]);
    }
}
