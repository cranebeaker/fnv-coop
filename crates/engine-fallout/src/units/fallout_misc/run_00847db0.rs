//! `fallout/misc/run_00847db0` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The two words the function clears (+0x0c and +0x10; in a
/// `BGSSaveGameBuffer`, `iBufferPosition` and `iDebugOverhead`).
const POSITION: u32 = 0x0c;
const OVERHEAD: u32 = 0x10;

// Translated from 00847db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the words at +0x0c and +0x10 of `this` (unnamed in the engine
/// map; Ghidra's library match `ContextBase::ClearCriticalRegion` is only
/// the shape). `0084bb50` runs it on a `BGSSaveGameBuffer`, where the two
/// words are the buffer's position and debug overhead.
pub fn fn_00847db0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + POSITION, 0);
    e.mem.set_u32(this.addr() + OVERHEAD, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0084_7db0, fn_00847db0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clears_the_words_at_0x0c_and_0x10_only() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x18);
        for offset in (0..0x18).step_by(4) {
            e.mem.set_u32(this + offset, 0x1111_1111 * (offset / 4 + 1));
        }
        e.call(0x0084_7db0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x08), 0x3333_3333);
        assert_eq!(e.mem.u32(this + POSITION), 0);
        assert_eq!(e.mem.u32(this + OVERHEAD), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0x6666_6666);
    }
}
