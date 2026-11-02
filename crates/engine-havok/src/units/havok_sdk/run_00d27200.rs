//! `Havok SDK/run_00d27200` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d27200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `0x10` of the vtable `010ce404` (class `hkpShapeContainer`-style wrapper whose
/// constructor `00d26b50` stores the inner object at `+0x18`): `ECX = [this + 0x18]` and a `JMP`
/// to slot `0x10` of that object's vtable, leaving the caller's stack words untouched. The words
/// are not visible here (the exe does not pop them): every word after `this` is passed on, and the
/// callee's result (`EAX`, `ST0`) is returned as it is. The sibling class at `010ce370` has
/// `00d26440` (`RET 4`, one word) in that slot.
pub fn fn_00d27200(e: &mut Engine, this: Ptr, rest: &[u32]) -> Ret {
    let inner = e.mem.u32(this.addr() + 0x18);
    e.vcall(inner, 0x10, rest)
}

/// The uniform form of `fn_00d27200`: `this`, then the caller's stack words.
fn fn_00d27200_entry(e: &mut Engine, words: &[u32]) -> Ret {
    fn_00d27200(e, Ptr::new(words[0]), &words[1..])
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![(0x00d2_7200, fn_00d27200_entry as AbiFn)]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d27200_jumps_to_slot_0x10_of_the_inner_object_with_the_stack_words() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x10, 0x0900_0061);
        let inner = e.mem.alloc(0x10);
        e.mem.set_u32(inner, vtable);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x18, inner);
        e.register(0x0900_0061, |_, a| Ret {
            eax: a.len() as u32 * 100 + a[1],
            edx: 7,
            st0: 1.5,
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x00d2_7200, &args![this, 0x42u32]);
        assert_eq!(result.eax, 266);
        assert_eq!(result.edx, 7);
        assert_eq!(result.st0, 1.5);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x0900_0061, vec![inner, 0x42]));
        // Whatever the caller pushed is passed on.
        e.call_log = Some(vec![]);
        e.call(0x00d2_7200, &args![this, 1u32, 2u32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1].1, vec![inner, 1, 2, 3]);
    }
}
