//! `Havok SDK/run_00c855f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The recursive sort `00c855f0` starts (`00c852c0`, cdecl: data, low index, high index, a
/// fourth word the sort only reads one byte of; its class is unconfirmed).
const SORT_RANGE: u32 = 0x00c8_52c0;

// Translated from 00c855f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sorts the elements of an array object (data pointer at `+0x10`, element count at `+0x14`)
/// when it has more than one: calls `00c852c0(data, 0, count - 1, word)`, where `word` is
/// `this` with its low byte cleared (the exe pushes `ECX`, zeroes the low byte of the pushed
/// word and passes the whole word on; the callee is given what the stack holds). The class is
/// unconfirmed.
pub fn fn_00c855f0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let count = e.mem.i32(base + 0x14);
    if count > 1 {
        let data = e.mem.u32(base + 0x10);
        e.call(
            SORT_RANGE,
            &args![data, 0u32, count.wrapping_sub(1), base & 0xffff_ff00],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c8_55f0, fn_00c855f0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Fewer than two elements sort nothing; otherwise the whole range is sorted with the
    /// object's address with its low byte cleared as the fourth word.
    #[test]
    fn sorts_the_whole_range_only_from_two_elements() {
        let mut e = Engine::new();
        // An address whose low byte is not zero, inside a block of the heap.
        let this = ((e.mem.alloc(0x200) + 0x100) & 0xffff_ff00) | 0x24;
        e.mem.set_u32(this + 0x10, 0x0222_0000);
        e.register_double(SORT_RANGE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for count in [0u32, 1, 0xffff_ffff] {
            e.mem.set_u32(this + 0x14, count);
            e.call(0x00c8_55f0, &args![this]);
        }
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(at, _)| *at == 0x00c8_55f0));
        e.call_log = Some(vec![]);
        e.mem.set_u32(this + 0x14, 5);
        e.call(0x00c8_55f0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1],
            (SORT_RANGE, vec![0x0222_0000, 0, 4, this & 0xffff_ff00])
        );
    }
}
