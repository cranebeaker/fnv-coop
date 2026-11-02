//! `NiAnimation/run_00a55a10` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBlendPoint3Interpolator`'s vtable (its class confirmed by the RTTI before it),
/// `NiBlendInterpolator::NiBlendInterpolator` (`thiscall`, three words, the second a float)
/// and `NiInterpolator::INVALID_POINT3` (three floats).
const NI_BLEND_POINT3_INTERPOLATOR_VTABLE: u32 = 0x0109_a49c;
const BLEND_INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a3_70b0;
const INVALID_POINT3: u32 = 0x011a_9424;

// Translated from 00a55a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiBlendPoint3Interpolator` constructor (unnamed in the map; `thiscall`, three words,
/// `RET 0xC`): `NiBlendInterpolator`'s constructor with the same arguments (the second a
/// float), the class's vtable, the value (`m_kPoint3Value`, +0x30) the invalid point and
/// `m_bValidValue` (+0x3C) false. Answers `this`.
pub fn fn_00a55a10(e: &mut Engine, this: Ptr, first: u32, float_argument: f32, third: u32) -> Ptr {
    e.call(
        BLEND_INTERPOLATOR_CONSTRUCTOR,
        &args![this, first, x87_quiet_bits(float_argument.to_bits()), third],
    );
    e.mem
        .set_u32(this.addr(), NI_BLEND_POINT3_INTERPOLATOR_VTABLE);
    for word in 0..3u32 {
        let value = e.global::<u32>(INVALID_POINT3 + 4 * word);
        e.mem.set_u32(this.addr() + 0x30 + 4 * word, value);
    }
    e.mem.set_u8(this.addr() + 0x3c, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a55a10, fn_00a55a10(Ptr, u32, f32, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a55a10_sets_the_vtable_and_an_invalid_value() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(BLEND_INTERPOLATOR_CONSTRUCTOR, |_, _| Ret::default());
        e.map(INVALID_POINT3, 12);
        for word in 0..3u32 {
            e.set_global::<u32>(INVALID_POINT3 + 4 * word, 0x7f7f_ffff);
        }
        let this = e.mem.alloc(0x40);
        e.mem.set_u8(this + 0x3c, 1);
        assert_eq!(
            e.call(0x00a55a10, &args![this, 4u32, 0.25f32, 2u32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), NI_BLEND_POINT3_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(this + 0x38), 0x7f7f_ffff);
        assert_eq!(e.mem.u8(this + 0x3c), 0);
        let base: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == BLEND_INTERPOLATOR_CONSTRUCTOR)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(base, vec![vec![this, 4, 0.25f32.to_bits(), 2]]);
    }
}
