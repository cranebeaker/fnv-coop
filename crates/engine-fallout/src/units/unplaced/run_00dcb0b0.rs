//! `(unplaced)/run_00dcb0b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dcb0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a vtable word of `ArrayOfTuplesImplementation::View`, slot
/// +0x18, and of `StructArrayImplementation::Object`, slot +8, by their RTTI): a jump
/// through vtable slot +0x18 of the object at `this + 0xc`, with that object as `this` and
/// the caller's stack words passed on as they are (a `JMP`, so however many there are), and
/// its result (`EAX`, `EDX`, `ST0`) passed back.
pub fn fn_00dcb0b0(e: &mut Engine, this: Ptr, rest: &[u32]) -> Ret {
    let inner = e.mem.u32(this.addr() + 0xc);
    e.vcall(inner, 0x18, rest)
}

/// The registered form of `00dcb0b0`: `this`, then every stack word the caller pushed.
fn fn_00dcb0b0_entry(e: &mut Engine, words: &[u32]) -> Ret {
    fn_00dcb0b0(e, Ptr::new(words[0]), &words[1..])
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![(0x00dc_b0b0, fn_00dcb0b0_entry as AbiFn)]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    #[test]
    fn forwards_every_word_to_the_inner_objects_slot() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x1c);
        e.mem.set_u32(vtable + 0x18, 0x0079_0018);
        e.register(0x0079_0018, |_, a| Ret {
            eax: a.iter().sum(),
            edx: 7,
            st0: 1.5,
        });
        let inner = e.mem.alloc(4);
        e.mem.set_u32(inner, vtable);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, inner);
        e.call_log = Some(vec![]);
        let r = e.call(0x00dc_b0b0, &args![this, 2u32, 3u32]);
        assert_eq!(
            r,
            Ret {
                eax: inner + 5,
                edx: 7,
                st0: 1.5
            }
        );
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (0x0079_0018, vec![inner, 2, 3])
        );
        let r = e.call(0x00dc_b0b0, &args![this]);
        assert_eq!(r.u32(), inner);
    }
}
