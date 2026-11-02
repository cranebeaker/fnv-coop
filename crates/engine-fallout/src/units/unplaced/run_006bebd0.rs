//! `(unplaced)/run_006bebd0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006967f0` (`thiscall`, `RET 8`; unit `fallout shared/pathfinding/navmesh_util.cpp`).
const RANGE_CALL: u32 = 0x0069_67f0;

// Translated from 006bebd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): when the record that the word at
/// +8 points to has a nonzero count at +0x1c, calls `006967f0` on the object at +0 with the
/// low 16 bits of the record's word at +0x18 and the last index `start + count - 1` (the
/// full word at +0x18 plus the count, minus 1).
pub fn fn_006bebd0(e: &mut Engine, this: Ptr) {
    let record = e.mem.u32(this.addr() + 8);
    let count = e.mem.u32(record + 0x1c);
    if count != 0 {
        let start = e.mem.u32(record + 0x18);
        let last = start.wrapping_add(count).wrapping_sub(1);
        let first = e.mem.u16(record + 0x18) as u32;
        let target = e.mem.u32(this.addr());
        e.call(RANGE_CALL, &args![target, first, last]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006b_ebd0, fn_006bebd0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_the_range_when_the_count_is_set() {
        let mut e = Engine::new();
        e.register(RANGE_CALL, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        let record = e.mem.alloc(0x20);
        e.mem.set_u32(this, 0x7000);
        e.mem.set_u32(this + 8, record);
        e.mem.set_u32(record + 0x18, 0x0001_0005);
        e.call_log = Some(vec![]);
        e.call(0x006b_ebd0, &args![this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.mem.set_u32(record + 0x1c, 3);
        e.call_log = Some(vec![]);
        e.call(0x006b_ebd0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(RANGE_CALL, vec![0x7000, 5, 0x0001_0007])]);
    }
}
