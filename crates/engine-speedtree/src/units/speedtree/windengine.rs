//! `SpeedTree/windengine.cpp` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `CWindEngine`'s two local matrix words (+0x28 and +0x2c, as the code
/// writes them).
const LOCAL_MATRICES: u32 = 0x28;
const LOCAL_MATRIX_COUNT: u32 = 0x2c;

// Translated from 00b37570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CWindEngine::SetLocalMatrices` (engine map; thiscall, `RET 8`): stores
/// its two words at +0x28 and +0x2c.
pub fn c_wind_engine_set_local_matrices(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.mem.set_u32(this.addr() + LOCAL_MATRICES, first);
    e.mem.set_u32(this.addr() + LOCAL_MATRIX_COUNT, second);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00b3_7570,
        c_wind_engine_set_local_matrices(Ptr, u32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_both_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        e.call(0x00b3_7570, &args![this, 0x0abc_0000u32, 4u32]);
        assert_eq!(e.mem.u32(this + LOCAL_MATRICES), 0x0abc_0000);
        assert_eq!(e.mem.u32(this + LOCAL_MATRIX_COUNT), 4);
    }
}
