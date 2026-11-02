//! `SpeedTree/run_00b1e750` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The CRT's invalid-parameter report (`00ec7c56`, no arguments).
const INVALID_PARAMETER: u32 = 0x00ec_7c56;
/// The range erase (`00b1ffd0`, thiscall, `RET 0x14`: the result
/// iterator's address, then the first and last iterators, each the
/// container word and the element pointer).
const ERASE_RANGE_00B1FFD0: u32 = 0x00b1_ffd0;
/// The vector's container word (+0) and first and last pointers (+0xc,
/// +0x10).
const CONTAINER: u32 = 0x00;
const FIRST: u32 = 0x0c;
const LAST: u32 = 0x10;

/// The checked iterator's test: `pointer` must lie in `[first, last]` of a
/// non-null vector, else the invalid-parameter report.
fn check_iterator(e: &mut Engine, this: u32, pointer: u32) {
    if this == 0 || e.mem.u32(this + FIRST) > pointer || pointer > e.mem.u32(this + LAST) {
        e.call(INVALID_PARAMETER, &args![]);
    }
}

// Translated from 00b1e750 (decompiled, FalloutNV.exe 1.4.0.525)
/// A vector's `clear` (unnamed in the engine map): the end and begin
/// iterators (each checked, the invalid-parameter report when out of
/// range) handed to the range erase `00b1ffd0(this, &result, begin, end)`;
/// the result is not used.
pub fn fn_00b1e750(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let last = e.mem.u32(base + LAST);
    check_iterator(e, base, last);
    let end_container = e.mem.u32(base + CONTAINER);
    let first = e.mem.u32(base + FIRST);
    check_iterator(e, base, first);
    let begin_container = e.mem.u32(base + CONTAINER);
    e.with_stack(8, |e, result| {
        e.call(
            ERASE_RANGE_00B1FFD0,
            &args![this, result, begin_container, first, end_container, last],
        );
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b1_e750, fn_00b1e750(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn erases_from_begin_to_end() {
        let mut e = Engine::new();
        e.register(INVALID_PARAMETER, |_, _| Ret::default());
        e.register(ERASE_RANGE_00B1FFD0, |_, _| Ret::default());
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + CONTAINER, 0x0777_0000);
        e.mem.set_u32(this + FIRST, 0x0600_0000);
        e.mem.set_u32(this + LAST, 0x0600_0030);
        e.call_log = Some(vec![]);
        e.call(0x00b1_e750, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == INVALID_PARAMETER));
        let erase: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == ERASE_RANGE_00B1FFD0)
            .collect();
        let words = &erase[0].1;
        assert_eq!(words[0], this);
        assert_eq!(
            &words[2..],
            &[0x0777_0000, 0x0600_0000, 0x0777_0000, 0x0600_0030]
        );
        // A broken vector (first after last) reports both iterators.
        e.mem.set_u32(this + FIRST, 0x0600_0040);
        e.call_log = Some(vec![]);
        e.call(0x00b1_e750, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter().filter(|(a, _)| *a == INVALID_PARAMETER).count(),
            2
        );
    }
}
