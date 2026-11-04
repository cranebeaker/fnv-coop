//! `(unplaced)/run_00844e80` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00844e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the hash map whose vtable `0107ed1c` has the RTTI name
/// `.?AV?$NiTMapBase@V?$DFALL@V?$NiTMapItem@PAVTESFile@@PAVBGSReconstructFormsInFile...`
/// (read through the word before the vtable): stores the vtable, empties the map (`00438af0`)
/// and frees its bucket table (`+8`) through `00aa10f0`.
pub fn fn_00844e80(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0107_ed1c);
    e.call(0x0043_8af0, &args![this]);
    let table = e.mem.u32(this.addr().wrapping_add(8));
    e.call(0x00aa_10f0, &args![table]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0084_4e80, fn_00844e80(Ptr))]
}

#[cfg(test)]
mod b035_tests {
    use super::*;

    #[test]
    fn fn_00844e80_empties_and_frees_the_map() {
        let mut e = Engine::new();
        e.register(0x0043_8af0, |_, _| Ret::default());
        e.register(0x00aa_10f0, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x0084_4e80, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), 0x0107_ed1c);
        assert_eq!(
            log[1..],
            [(0x0043_8af0, vec![this]), (0x00aa_10f0, vec![0x7000])]
        );
    }
}
