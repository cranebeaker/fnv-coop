//! `fallout/magic/run_0081b650` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `BSTempEffect` base constructor `006827b0` (unnamed in the engine map; `thiscall`,
/// two words).
const TEMP_EFFECT_CONSTRUCTOR: u32 = 0x0068_27b0;
/// The `MagicHitEffect` vtable (`0107b0ec`, RTTI `.?AVMagicHitEffect@@`).
const MAGIC_HIT_EFFECT_VTABLE: u32 = 0x0107_b0ec;
/// The float `fLifetime` starts at (`01016970`).
const INITIAL_LIFETIME: u32 = 0x0101_6970;

// Translated from 0081b650 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `MagicHitEffect` constructor (by the vtable it stores; unnamed in the engine map;
/// `thiscall`, plain `RET`): the base constructor `006827b0` with 0 and 0.0, the vtable,
/// `pTarget` (+0x1C) and `pEffect` (+0x18) cleared, `fAge` (+0x20) 0.0, `fLifetime` (+8)
/// from the float at `01016970` (`FLD`/`FSTP`: a signalling NaN is stored quiet) and
/// `bFinished` (+0x24) cleared (Xbox PDB names). Returns `this`.
pub fn fn_0081b650(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(TEMP_EFFECT_CONSTRUCTOR, &args![this, 0u32, 0.0f32]);
    e.mem.set_u32(base, MAGIC_HIT_EFFECT_VTABLE);
    e.mem.set_u32(base + 0x1c, 0);
    e.mem.set_u32(base + 0x18, 0);
    e.mem.set_f32(base + 0x20, 0.0);
    let lifetime = x87_quiet_bits(e.global::<u32>(INITIAL_LIFETIME));
    e.mem.set_u32(base + 0x8, lifetime);
    e.mem.set_u8(base + 0x24, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0081_b650, fn_0081b650(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_the_hit_effect() {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x1000);
        e.set_global::<f32>(INITIAL_LIFETIME, 2.5);
        e.register(TEMP_EFFECT_CONSTRUCTOR, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0x28);
        for i in 0..10 {
            e.mem.set_u32(this + 4 * i, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0081_b650, &[this]).u32(), this);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == TEMP_EFFECT_CONSTRUCTOR)
            .collect();
        assert_eq!(base[0].1, vec![this, 0, 0]);
        assert_eq!(e.mem.u32(this), MAGIC_HIT_EFFECT_VTABLE);
        assert_eq!(e.mem.f32(this + 8), 2.5);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.u32(this + 0x24), 0xffff_ff00);
    }
}
