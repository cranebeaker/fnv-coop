//! `Havok SDK/run_00dff990` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dff990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x30` (`SUB ECX,0x30`) and jumps to `00dff930`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00dff990(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_f930, &args![this.addr().wrapping_sub(0x30), flags])
        .u32()
}

// Translated from 00dff9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x14` (`SUB ECX,0x14`) and jumps to `00dff930`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00dff9a0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_f930, &args![this.addr().wrapping_sub(0x14), flags])
        .u32()
}

// Translated from 00dffa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x40-byte stack buffer by calling `00dffa90` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dffa70(e: &mut Engine) -> u32 {
    e.with_stack(0x40, |e, buffer| {
        e.call(0x00df_fa90, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dff9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): in a 0x60-byte frame stores the vtable word 0x010c_c2fc at
/// its start, runs the constructor at `00cb0a70` on the member at frame+0x14 (argument 0) and
/// returns the constant 0x010d_0994 (the stored word is not read back). Nothing is destroyed.
pub fn fn_00dff9f0(e: &mut Engine) -> u32 {
    e.with_stack(0x60, |e, frame| {
        let base = frame.addr();
        e.mem.set_u32(base, 0x010c_c2fc);
        e.call(0x00cb_0a70, &args![base + 0x14, 0u32]);
        0x010d_0994
    })
}

// Translated from 00dff9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010cc2fc`, runs
/// `00cb0a70` (an `hkStringPtr` function, `this` = `object + 0x14`, one stack word `extra`) and
/// writes the vtable word `010d0994`. The function takes two stack words and pops none (`RET`),
/// and returns nothing. The class is unconfirmed.
pub fn fn_00dff9b0(e: &mut Engine, object: Ptr, extra: u32) {
    if object.addr() != 0 {
        e.mem.set_u32(object.addr(), 0x010c_c2fc);
        e.call(0x00cb_0a70, &args![object.addr() + 0x14, extra]);
        e.mem.set_u32(object.addr(), 0x010d_0994);
    }
}

// Translated from 00dffa90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a constructor chain that the compiler collapsed (`this` in `ECX`;
/// the one stack word is never read, `RET 4`). It writes the words at `+8`, `+0xc`, `+0x10`,
/// `+0x14` and `+0x30` with the base-class values (`010c3b80`, `010c3b98`, `010c3bac`,
/// `010c3b8c`, `010cb1e4`), then the vtable word at `+0` (`010e7874`) and the five words again
/// with the final values (`010e7868`, `010e7854`, `010e7848`, `010e783c`, `010e781c`). Returns
/// `this` (`EAX`). The class is unconfirmed.
pub fn fn_00dffa90(e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base + 0x8, 0x010c_3b80);
    e.mem.set_u32(base + 0xc, 0x010c_3b98);
    e.mem.set_u32(base + 0x10, 0x010c_3bac);
    e.mem.set_u32(base + 0x14, 0x010c_3b8c);
    e.mem.set_u32(base + 0x30, 0x010c_b1e4);
    e.mem.set_u32(base, 0x010e_7874);
    e.mem.set_u32(base + 0x8, 0x010e_7868);
    e.mem.set_u32(base + 0xc, 0x010e_7854);
    e.mem.set_u32(base + 0x10, 0x010e_7848);
    e.mem.set_u32(base + 0x14, 0x010e_783c);
    e.mem.set_u32(base + 0x30, 0x010e_781c);
    this
}

// Translated from 00dffaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant `0x0126_d658`, the
/// address of a data object.
pub fn return_constant_00dffaf0(_e: &mut Engine) -> u32 {
    0x0126_d658
}

// Translated from 00dff9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word `object`): calls the object's virtual function at
/// vtable offset 0 (the destructor) with the one stack word 0 and answers what it answers. It is
/// the cleanup function of a type-info record (word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject`
/// calls it with `(object)`): the start-up initializer at `00fc4df9` stores its address there.
pub fn fn_00dff9e0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dffa30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word `object`): calls the object's virtual function at
/// vtable offset 0 (the destructor) with the one stack word 0 and answers what it answers. It is
/// the cleanup function of a type-info record (word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject`
/// calls it with `(object)`): the start-up initializer at `00fc4e69` stores its address there.
pub fn fn_00dffa30(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dffa50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words `(object, extra)`): with a non-null `object` runs
/// `00dffa90` on it with `extra` (`thiscall`, one stack word); otherwise does nothing. It is the
/// finish function of a type-info record (word `+4`): the start-up initializer at `00fc4e5f` stores
/// its address there.
pub fn fn_00dffa50(e: &mut Engine, object: Ptr, extra: u32) {
    if !object.is_null() {
        e.call(0x00df_fa90, &args![object, extra]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dff9e0, fn_00dff9e0(Ptr) -> u32),
        entry!(0x00dffa30, fn_00dffa30(Ptr) -> u32),
        entry!(0x00dffa50, fn_00dffa50(Ptr, u32)),
        entry!(0x00df_fa70, fn_00dffa70() -> u32),
        entry!(0x00df_f990, fn_00dff990(Ptr, u32) -> u32),
        entry!(0x00df_f9a0, fn_00dff9a0(Ptr, u32) -> u32),
        entry!(0x00dff9f0, fn_00dff9f0() -> u32),
        entry!(0x00df_f9b0, fn_00dff9b0(Ptr, u32)),
        entry!(0x00df_fa90, fn_00dffa90(Ptr, u32) -> Ptr),
        entry!(0x00df_faf0, return_constant_00dffaf0() -> u32),
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
    fn thunk_00dff990() {
        assert_eq!(check(0x00df_f990, 0x00df_f930, 0x30), 0xdead_beef);
    }

    #[test]
    fn thunk_00dff9a0() {
        assert_eq!(check(0x00df_f9a0, 0x00df_f930, 0x14), 0xdead_beef);
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
        fn_00dffa70_returns_first_word_of_built_object,
        0x00df_fa70,
        0x00df_fa90
    );
}

#[cfg(test)]
mod record_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn fn_00dff9e0_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        let object = e.mem.alloc(8);
        e.put_vtable(0x0900_0000, &[0x0900_0100]);
        e.mem.set_u32(object, 0x0900_0000);
        e.register(0x0900_0100, |_, a| Ret {
            eax: a[0].wrapping_add(a[1]).wrapping_add(7),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let answer = e.call(0x00df_f9e0, &args![object]).u32();
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
    fn fn_00dffa30_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        let object = e.mem.alloc(8);
        e.put_vtable(0x0900_0000, &[0x0900_0100]);
        e.mem.set_u32(object, 0x0900_0000);
        e.register(0x0900_0100, |_, a| Ret {
            eax: a[0].wrapping_add(a[1]).wrapping_add(7),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let answer = e.call(0x00df_fa30, &args![object]).u32();
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
    fn fn_00dffa50_runs_the_callee_on_a_non_null_object() {
        let mut e = Engine::new();
        e.register(0x00df_fa90, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00df_fa50, &args![0x1234u32, 0x55u32]);
        let seen: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x00df_fa90)
            .cloned()
            .collect();
        assert_eq!(seen, vec![(0x00df_fa90, vec![0x1234, 0x55])]);
    }

    #[test]
    fn fn_00dffa50_does_nothing_for_a_null_object() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00df_fa50, &args![0u32, 0x55u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
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
    fn fn_00dff9f0_stores_the_vtable_and_constructs_its_member() {
        let mut e = Engine::new();
        let log = frame_probe(&mut e, 0x00cb_0a70, &[0x14], false);
        let result = e.call(0x00df_f9f0, &args![]).u32();
        assert_eq!(result, 0x010d_0994);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        for call in calls.iter() {
            assert_eq!(call.1, 0);
            assert_eq!(call.2, 0x010c_c2fc);
        }
    }

    #[test]
    fn fn_00dffa70_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00df_fa90, 0x0a0b_0c0d);
        let result = e.call(0x00df_fa70, &args![]).u32();
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

    /// A null object does nothing; otherwise the vtable word ends at the second value and the
    /// string function runs in between on `object + 0x14`, seeing the first value.
    #[test]
    fn fn_00dff9b0_runs_the_string_function_between_the_vtable_writes() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x20);
        e.register_double(0x00cb_0a70, move |e, a| {
            assert_eq!(a, [object + 0x14, 7]);
            assert_eq!(e.mem.u32(object), 0x010c_c2fc);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00df_f9b0, &args![0u32, 7u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.call(0x00df_f9b0, &args![object, 7u32]);
        assert_eq!(e.mem.u32(object), 0x010d_0994);
    }

    /// The final vtable words are in place and `this` comes back; neighbours are untouched.
    #[test]
    fn fn_00dffa90_writes_the_final_vtable_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        e.mem.write(this, &[0x33; 0x40]);
        assert_eq!(e.call(0x00df_fa90, &args![this, 0u32]).u32(), this);
        let words: Vec<u32> = [0x0, 0x8, 0xc, 0x10, 0x14, 0x30]
            .iter()
            .map(|offset| e.mem.u32(this + offset))
            .collect();
        assert_eq!(
            words,
            [
                0x010e_7874,
                0x010e_7868,
                0x010e_7854,
                0x010e_7848,
                0x010e_783c,
                0x010e_781c
            ]
        );
        assert_eq!(e.mem.u32(this + 0x4), 0x3333_3333);
        assert_eq!(e.mem.u32(this + 0x18), 0x3333_3333);
    }

    #[test]
    fn return_constant_00dffaf0_returns_the_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_faf0, &args![]).u32(), 0x0126_d658);
    }
}
