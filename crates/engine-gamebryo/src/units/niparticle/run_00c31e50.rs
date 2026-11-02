//! `NiParticle/run_00c31e50` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPoint3::ZERO` (three `float`s at `011f426c`, copied by `MOV`).
const POINT_ZERO: u32 = 0x011f_426c;

// Translated from 00c31e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiParticleInfo` constructor (its callers
/// construct the 0x1c-byte particle info arrays with it). The velocity
/// (`m_kVelocity`, +0) from `NiPoint3::ZERO`, the age (+0xc), life span
/// (+0x10) and last update (+0x14) 0.0, the generation (+0x18) and code
/// (+0x1a) 0 (`NiParticleInfo`, Xbox PDB offsets). Returns `this` (the code
/// leaves it in `EAX`).
pub fn fn_00c31e50(e: &mut Engine, this: Ptr) -> Ptr {
    let p = this.addr();
    for word in 0..3 {
        let value = e.global::<u32>(POINT_ZERO + 4 * word);
        e.mem.set_u32(p + 4 * word, value);
    }
    let zero = 0.0f32.to_bits();
    e.mem.set_u32(p + 0xc, zero);
    e.mem.set_u32(p + 0x10, zero);
    e.mem.set_u32(p + 0x14, zero);
    e.mem.set_u16(p + 0x18, 0);
    e.mem.set_u16(p + 0x1a, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c3_1e50, fn_00c31e50(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_clears_the_particle_info() {
        let mut e = Engine::new();
        e.map(POINT_ZERO & !0xfff, 0x1000);
        e.set_global(POINT_ZERO, 1u32);
        e.set_global(POINT_ZERO + 4, 2u32);
        e.set_global(POINT_ZERO + 8, 3u32);
        let info = e.mem.alloc(0x1c);
        e.mem.write(info, &[0xff; 0x1c]);
        assert_eq!(e.call(0x00c3_1e50, &args![info]).u32(), info);
        assert_eq!(e.mem.u32(info), 1);
        assert_eq!(e.mem.u32(info + 4), 2);
        assert_eq!(e.mem.u32(info + 8), 3);
        assert_eq!(e.mem.bytes(info + 0xc, 0x10), vec![0; 0x10]);
    }
}
