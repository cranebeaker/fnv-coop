//! `Havok SDK/run_00dced70` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dced70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x30-byte stack buffer by calling `00d4dfc0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dced70(e: &mut Engine) -> u32 {
    e.with_stack(0x30, |e, buffer| {
        e.call(0x00d4_dfc0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

/// The `double` the first word of a record has added to it (`010c9300`, 2^-23).
const RECORD_TINY: u32 = 0x010c_9300;

// Translated from 00dced90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `first`, `second`, `cursor`; the vectors are 16 bytes).
/// Appends a 0x20-byte record at the address held in the word `cursor` points at: the vector
/// `first` at `+0`, the vector `second` at `+0x10`; the first `float` of the record then has the
/// `double` at `010c9300` added to it (x87, stored back as a `float`) and its low byte is
/// replaced by 0x19. Finally the word at `cursor`
/// advances by 0x20.
pub fn fn_00dced90(e: &mut Engine, first: u32, second: u32, cursor: u32) {
    let at = e.mem.u32(cursor);
    let head = [0, 1, 2, 3].map(|i| e.mem.u32(first + 4 * i));
    for (i, word) in head.into_iter().enumerate() {
        e.mem.set_u32(at + 4 * i as u32, word);
    }
    let value = e.mem.u32(at);
    let tail = [0, 1, 2, 3].map(|i| e.mem.u32(second + 4 * i));
    let tiny = e.mem.f64(RECORD_TINY);
    for (i, word) in tail.into_iter().enumerate() {
        e.mem.set_u32(at + 0x10 + 4 * i as u32, word);
    }
    e.mem
        .set_u32(at, x87_store32(x87_add(x87_load32(value), tiny)));
    e.mem.set_u8(at, 0x19);
    e.mem.set_u32(cursor, at.wrapping_add(0x20));
}

// Translated from 00dced30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word `object`): calls the object's virtual function at
/// vtable offset 0 (the destructor) with the one stack word 0 and answers what it answers. It is
/// the cleanup function of a type-info record (word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject`
/// calls it with `(object)`): the start-up initializer at `00fc00e9` stores its address there.
pub fn fn_00dced30(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dced50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words `(object, extra)`): with a non-null `object` runs
/// `00d4dfc0` on it with `extra` (`thiscall`, one stack word); otherwise does nothing. It is the
/// finish function of a type-info record (word `+4`): the start-up initializer at `00fc00df` stores
/// its address there.
pub fn fn_00dced50(e: &mut Engine, object: Ptr, extra: u32) {
    if !object.is_null() {
        e.call(0x00d4_dfc0, &args![object, extra]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dced30, fn_00dced30(Ptr) -> u32),
        entry!(0x00dced50, fn_00dced50(Ptr, u32)),
        entry!(0x00dc_ed70, fn_00dced70() -> u32),
        entry!(0x00dced90, fn_00dced90(u32, u32, u32)),
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
        fn_00dced70_returns_first_word_of_built_object,
        0x00dc_ed70,
        0x00d4_dfc0
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn fn_00dced30_calls_vtable_slot_zero_with_zero() {
        let mut e = Engine::new();
        let object = e.mem.alloc(8);
        e.put_vtable(0x0900_0000, &[0x0900_0100]);
        e.mem.set_u32(object, 0x0900_0000);
        e.register(0x0900_0100, |_, a| Ret {
            eax: a[0].wrapping_add(a[1]).wrapping_add(7),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let answer = e.call(0x00dc_ed30, &args![object]).u32();
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
    fn fn_00dced50_runs_the_callee_on_a_non_null_object() {
        let mut e = Engine::new();
        e.register(0x00d4_dfc0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00dc_ed50, &args![0x1234u32, 0x55u32]);
        let seen: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x00d4_dfc0)
            .cloned()
            .collect();
        assert_eq!(seen, vec![(0x00d4_dfc0, vec![0x1234, 0x55])]);
    }

    #[test]
    fn fn_00dced50_does_nothing_for_a_null_object() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00dc_ed50, &args![0u32, 0x55u32]);
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

    #[test]
    fn fn_00dced70_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00d4_dfc0, 0x0a0b_0c0d);
        let result = e.call(0x00dc_ed70, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }
}

#[cfg(test)]
mod a07_vector_tests {
    use super::*;

    #[test]
    fn a_record_of_two_vectors_is_appended_and_its_first_word_tagged() {
        let mut e = Engine::new();
        e.map(RECORD_TINY, 16);
        e.mem.set_f64(RECORD_TINY, f64::from(f32::EPSILON));
        let first = e.mem.alloc(16);
        let second = e.mem.alloc(16);
        for i in 0..4 {
            e.mem.set_f32(first + 4 * i, 1.0 + i as f32);
            e.mem.set_u32(second + 4 * i, 0x200 + i);
        }
        let area = e.mem.alloc(0x40);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, area);
        e.call(0x00dc_ed90, &args![first, second, cursor]);
        // 1.0 + 2^-23 is 0x3f800001; the low byte becomes 0x19.
        assert_eq!(e.mem.u32(area), 0x3f80_0019);
        assert_eq!(e.mem.f32(area + 4), 2.0);
        assert_eq!(e.mem.f32(area + 8), 3.0);
        assert_eq!(e.mem.f32(area + 0xc), 4.0);
        let tail: Vec<u32> = (0..4).map(|i| e.mem.u32(area + 0x10 + 4 * i)).collect();
        assert_eq!(tail, vec![0x200, 0x201, 0x202, 0x203]);
        assert_eq!(e.mem.u32(cursor), area + 0x20);
        // A second record goes after the first.
        e.call(0x00dc_ed90, &args![first, second, cursor]);
        assert_eq!(e.mem.u32(area + 0x20), 0x3f80_0019);
        assert_eq!(e.mem.u32(cursor), area + 0x40);
    }
}
