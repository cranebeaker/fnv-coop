//! `SpeedTree/run_00b1e970` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The block free `00401030` (`cdecl`, one word).
const FREE_00401030: u32 = 0x0040_1030;

// Translated from 00b1e970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): a vector's tidy-up for 0x38-byte
/// elements (first +0xC, last +0x10, end of storage +0x14). With storage, the code walks
/// from the first element to the last in 0x38-byte steps doing nothing for each (the
/// elements need no destructor; like the exe, the walk only ends on reaching the last
/// exactly), works out the capacity ((end − first) / 0x38, signed, unused) and frees the
/// storage (`00401030`). Then the three pointers are cleared.
pub fn fn_00b1e970(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    let first = e.mem.u32(t.wrapping_add(0xc));
    if first != 0 {
        let last = e.mem.u32(t.wrapping_add(0x10));
        let mut element = first;
        while element != last {
            element = element.wrapping_add(0x38);
        }
        let end = e.mem.u32(t.wrapping_add(0x14));
        let _capacity = (end.wrapping_sub(first) as i32) / 0x38;
        let storage = e.mem.u32(t.wrapping_add(0xc));
        e.call(FREE_00401030, &args![storage]);
    }
    e.mem.set_u32(t.wrapping_add(0xc), 0);
    e.mem.set_u32(t.wrapping_add(0x10), 0);
    e.mem.set_u32(t.wrapping_add(0x14), 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b1_e970, fn_00b1e970(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00b1e970_frees_the_storage_and_clears_the_pointers() {
        let mut e = Engine::new();
        e.register(FREE_00401030, |_, _| Ret::default());
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0xc, 0x1000);
        e.mem.set_u32(this + 0x10, 0x1000 + 2 * 0x38);
        e.mem.set_u32(this + 0x14, 0x1000 + 4 * 0x38);
        e.call_log = Some(vec![]);
        e.call(0x00b1_e970, &args![this]);
        let frees: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == FREE_00401030)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(frees, vec![vec![0x1000]]);
        for offset in [0xcu32, 0x10, 0x14] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        // Without storage nothing is freed.
        e.call(0x00b1_e970, &args![this]);
        let count = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == FREE_00401030)
            .count();
        assert_eq!(count, 1);
    }
}
