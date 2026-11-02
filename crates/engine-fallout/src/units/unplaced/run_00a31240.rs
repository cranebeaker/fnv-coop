//! `(unplaced)/run_00a31240` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The byte (`.rdata`) copied into +0xC.
const DEFAULT_BYTE: u32 = 0x0109_6968;

// Translated from 00a31240 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function, no engine-map name; `thiscall`, no stack words; its address
/// is taken as data by `LoadBinary` code at `00a33edf` and others): zeroes the words at
/// +0x0, +0x4 and +0x8, copies the byte at `01096968` into +0xC, sets +0xD to 0xFF and
/// answers `this`.
pub fn fn_00a31240(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, 0);
    e.mem.set_u32(this + 0x4, 0);
    e.mem.set_u32(this + 0x8, 0);
    let byte = e.mem.u8(DEFAULT_BYTE);
    e.mem.set_u8(this + 0xc, byte);
    e.mem.set_u8(this + 0xd, 0xff);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a3_1240, fn_00a31240(u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a31240_clears_and_sets_the_bytes() {
        let mut e = Engine::new();
        e.map(DEFAULT_BYTE & !0xfff, 0x1000);
        e.mem.set_u8(DEFAULT_BYTE, 0x5a);
        let this = e.mem.alloc(0x10);
        for offset in (0..0x10).step_by(4) {
            e.mem.set_u32(this + offset, 0x1111_1111);
        }
        assert_eq!(e.call(0x00a3_1240, &[this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0x1111_ff5a);
    }
}
