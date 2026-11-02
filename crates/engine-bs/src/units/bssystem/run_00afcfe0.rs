//! `BSSystem/run_00afcfe0` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00afcfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A three-bit number from the top bits of three bytes (`thiscall`; no Xbox
/// PDB name): bit 7 of the byte at +3 as 4, of the byte at +0 as 2 and of
/// the byte at +1 as 1.
pub fn fn_00afcfe0(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let high = (u32::from(e.mem.u8(base + 3)) & 0x80) >> 5;
    let middle = (u32::from(e.mem.u8(base)) & 0x80) >> 6;
    let low = (u32::from(e.mem.u8(base + 1)) & 0x80) >> 7;
    high + middle + low
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00af_cfe0, fn_00afcfe0(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combines_the_three_top_bits() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        for (bytes, expected) in [
            ([0u8, 0, 0, 0], 0u32),
            ([0x80, 0, 0, 0], 2),
            ([0, 0x80, 0, 0], 1),
            ([0, 0, 0xff, 0x80], 4),
            ([0xff, 0xff, 0x7f, 0xff], 7),
            ([0x7f, 0x7f, 0x80, 0x7f], 0),
        ] {
            e.mem.write(this, &bytes);
            assert_eq!(
                e.call(0x00af_cfe0, &args![this]).u32(),
                expected,
                "{bytes:x?}"
            );
        }
    }
}
