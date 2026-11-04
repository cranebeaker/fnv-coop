//! `(unplaced)/run_006db840` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The next free index of a `BSSimpleArray` (`00845460`, thiscall; it grows the buffer when
/// needed) and the 8-byte element-range constructor (`006dc3a0`, thiscall: first element,
/// count).
const NEXT_INDEX: u32 = 0x0084_5460;
const CONSTRUCT_ELEMENTS: u32 = 0x006d_c3a0;

// Translated from 006db840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `RET 4`; called from `AddRipple` and
/// `UpdateWaterDisplacementSimulation`): appends the 8-byte `value` to an array of 8-byte
/// elements (buffer at +4): takes the next index (`00845460`), constructs one element
/// there (`006dc3a0`) and copies the two words of `value` into it. Answers the index.
pub fn fn_006db840(e: &mut Engine, this: Ptr, value: Ptr) -> u32 {
    let buffer = this.addr() + 4;
    let index = e.call(NEXT_INDEX, &args![this]).u32();
    let at = e.mem.u32(buffer).wrapping_add(index.wrapping_mul(8));
    e.call(CONSTRUCT_ELEMENTS, &args![this, at, 1u32]);
    let first = e.mem.u32(value.addr());
    let second = e.mem.u32(value.addr() + 4);
    let at = e.mem.u32(buffer).wrapping_add(index.wrapping_mul(8));
    e.mem.set_u32(at, first);
    e.mem.set_u32(at.wrapping_add(4), second);
    index
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006d_b840, fn_006db840(Ptr, Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_copies_both_words_at_the_next_index() {
        let mut e = Engine::new();
        e.map(0x8000, 0x100);
        e.register(NEXT_INDEX, |e, a| {
            e.mem.set_u32(a[0] + 4, 0x8000);
            3u32.into_ret()
        });
        e.register(CONSTRUCT_ELEMENTS, |_, _| Ret::default());
        let array = e.mem.alloc(0x10);
        let value = e.mem.alloc(8);
        e.mem.set_u32(value, 0x11);
        e.mem.set_u32(value + 4, 0x22);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006d_b840, &args![array, value]).u32(), 3);
        assert_eq!(e.mem.u32(0x8018), 0x11);
        assert_eq!(e.mem.u32(0x801c), 0x22);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(CONSTRUCT_ELEMENTS, vec![array, 0x8018, 1])));
    }
}
