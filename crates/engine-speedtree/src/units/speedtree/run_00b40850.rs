//! `SpeedTree/run_00b40850` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `float` the first two words start as (`01016248`, 0.5).
const HALF: u32 = 0x0101_6248;

// Translated from 00b40850 (decompiled, FalloutNV.exe 1.4.0.525)
/// A constructor (unnamed in the engine map): the first two `float`s from
/// the one at `01016248` (`FLD`/`FSTP` copies), the next five zero
/// (`FLDZ`). Returns `this`.
pub fn fn_00b40850(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    let half = x87_quiet_bits(e.global::<u32>(HALF));
    e.mem.set_u32(base, half);
    let half = x87_quiet_bits(e.global::<u32>(HALF));
    e.mem.set_u32(base + 4, half);
    for offset in [0x08u32, 0x0c, 0x10, 0x14, 0x18] {
        e.mem.set_u32(base + offset, x87_store32(0.0));
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b4_0850, fn_00b40850(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_with_two_halves_and_five_zeros() {
        let mut e = Engine::new();
        e.map(HALF, 4);
        e.mem.set_f32(HALF, 0.5);
        let this = e.mem.alloc(0x20);
        for offset in (0..0x20).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        assert_eq!(e.call(0x00b4_0850, &args![this]).u32(), this);
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(
            words,
            vec![0x3f00_0000, 0x3f00_0000, 0, 0, 0, 0, 0, 0xdead_beef]
        );
    }
}
