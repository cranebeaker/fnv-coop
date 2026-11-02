//! `NiParticle/run_00c21f00` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The `NiPSysEmitter` constructor and member copy (the engine map gives
//! them no names; the vtable the constructor stores, `010bde84`, has the
//! RTTI name `.?AVNiPSysEmitter@@`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::niparticle::nipsysemitter::NiPSysEmitter;

/// The `NiPSysModifier` constructor (`00c31720`) and member copy
/// (`00c31780`, `(dest, process)`), `thiscall`.
const MODIFIER_CONSTRUCTOR: u32 = 0x00c3_1720;
const MODIFIER_COPY_MEMBERS: u32 = 0x00c3_1780;
/// The vtable of `NiPSysEmitter`.
const VTABLE: u32 = 0x010b_de84;
/// The colour the constructor gives the initial colour (four `float`s at
/// `011a9be0`, copied by `MOV`).
const DEFAULT_COLOR: u32 = 0x011a_9be0;

// Translated from 00c21f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPSysEmitter` constructor. The
/// `NiPSysModifier` constructor, the vtable; speed, declination, planar
/// angle and their variations 0; the initial colour from `011a9be0`; the
/// initial radius and the scale 1; the radius variation, life span and its
/// variation 0. Returns `this`.
pub fn fn_00c21f00(e: &mut Engine, this: Ptr<NiPSysEmitter>) -> Ptr<NiPSysEmitter> {
    e.call(MODIFIER_CONSTRUCTOR, &args![this]);
    let zero = 0.0f32.to_bits();
    let one = 1.0f32.to_bits();
    e.set(this, NiPSysEmitter::m_fSpeed, zero);
    e.mem.set_u32(this.addr(), VTABLE);
    for field in [
        NiPSysEmitter::m_fSpeedVar,
        NiPSysEmitter::m_fDeclination,
        NiPSysEmitter::m_fDeclinationVar,
        NiPSysEmitter::m_fPlanarAngle,
        NiPSysEmitter::m_fPlanarAngleVar,
    ] {
        e.set(this, field, zero);
    }
    for word in 0..4 {
        let value = e.global::<u32>(DEFAULT_COLOR + 4 * word);
        e.mem.set_u32(
            this.addr() + NiPSysEmitter::m_kInitialColor.off + 4 * word,
            value,
        );
    }
    e.set(this, NiPSysEmitter::m_fInitialRadius, one);
    e.set(this, NiPSysEmitter::m_fScale, one);
    e.set(this, NiPSysEmitter::m_fRadiusVar, zero);
    e.set(this, NiPSysEmitter::m_fLifeSpan, zero);
    e.set(this, NiPSysEmitter::m_fLifeSpanVar, zero);
    this
}

// Translated from 00c21f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPSysEmitter` member copy into `dest`
/// (`thiscall`, `RET 8`: `dest`, cloning process). The `NiPSysModifier`
/// member copy, then the speed, declination, planar angle and their
/// variations, the initial colour (by `MOV`), initial radius, life span and
/// its variation, and the radius variation (last); the floats go through
/// the x87 stack (`FLD`/`FSTP`). The scale is not copied.
pub fn fn_00c21f60(
    e: &mut Engine,
    this: Ptr<NiPSysEmitter>,
    dest: Ptr<NiPSysEmitter>,
    process: Ptr,
) {
    e.call(MODIFIER_COPY_MEMBERS, &args![this, dest, process]);
    let copy_float = |e: &mut Engine, field| {
        let value = x87_quiet_bits(e.get(this, field));
        e.set(dest, field, value);
    };
    for field in [
        NiPSysEmitter::m_fSpeed,
        NiPSysEmitter::m_fSpeedVar,
        NiPSysEmitter::m_fDeclination,
        NiPSysEmitter::m_fDeclinationVar,
        NiPSysEmitter::m_fPlanarAngle,
        NiPSysEmitter::m_fPlanarAngleVar,
    ] {
        copy_float(e, field);
    }
    for word in 0..4 {
        let off = NiPSysEmitter::m_kInitialColor.off + 4 * word;
        let value = e.mem.u32(this.addr() + off);
        e.mem.set_u32(dest.addr() + off, value);
    }
    for field in [
        NiPSysEmitter::m_fInitialRadius,
        NiPSysEmitter::m_fLifeSpan,
        NiPSysEmitter::m_fLifeSpanVar,
        NiPSysEmitter::m_fRadiusVar,
    ] {
        copy_float(e, field);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c2_1f00,
            fn_00c21f00(Ptr<NiPSysEmitter>) -> Ptr<NiPSysEmitter>
        ),
        entry!(
            0x00c2_1f60,
            fn_00c21f60(Ptr<NiPSysEmitter>, Ptr<NiPSysEmitter>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_sets_the_emitter_defaults() {
        let mut e = Engine::new();
        e.register(MODIFIER_CONSTRUCTOR, |_, _| Ret::default());
        e.map(DEFAULT_COLOR & !0xfff, 0x1000);
        for (i, value) in [0.1f32, 0.2, 0.3, 0.4].iter().enumerate() {
            e.set_global(DEFAULT_COLOR + 4 * i as u32, *value);
        }
        let this = e.new_object::<NiPSysEmitter>();
        for off in (0x18..0x54).step_by(4) {
            e.mem.set_u32(this.addr() + off, 0xdead);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_1f00, &args![this]).u32(), this.addr());
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|c| c.0 == MODIFIER_CONSTRUCTOR && c.1 == vec![this.addr()]));
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        let f = |e: &Engine, off: u32| e.mem.f32(this.addr() + off);
        for off in [0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x44, 0x48, 0x4c] {
            assert_eq!(e.mem.u32(this.addr() + off), 0, "+{off:x}");
        }
        assert_eq!(f(&e, 0x30), 0.1);
        assert_eq!(f(&e, 0x3c), 0.4);
        assert_eq!(f(&e, 0x40), 1.0);
        assert_eq!(f(&e, 0x50), 1.0);
    }

    #[test]
    fn copy_members_copies_all_but_the_scale() {
        let mut e = Engine::new();
        e.register(MODIFIER_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.new_object::<NiPSysEmitter>();
        let dest = e.new_object::<NiPSysEmitter>();
        for off in (0x18..0x54).step_by(4) {
            e.mem.set_u32(this.addr() + off, 0x4000_0000 + off);
        }
        // A signalling NaN speed comes out quiet; the colour is copied as is.
        e.mem.set_u32(this.addr() + 0x18, 0x7f80_0001);
        e.mem.set_u32(this.addr() + 0x30, 0x7f80_0001);
        e.call_log = Some(vec![]);
        e.call(0x00c2_1f60, &args![this, dest, 0x4000u32]);
        let log = e.call_log.take().unwrap();
        assert!(
            log.iter()
                .any(|c| c.0 == MODIFIER_COPY_MEMBERS
                    && c.1 == vec![this.addr(), dest.addr(), 0x4000])
        );
        assert_eq!(e.mem.u32(dest.addr() + 0x18), 0x7fc0_0001);
        assert_eq!(e.mem.u32(dest.addr() + 0x30), 0x7f80_0001);
        for off in (0x1c..0x50).step_by(4).filter(|o| *o != 0x30) {
            assert_eq!(e.mem.u32(dest.addr() + off), 0x4000_0000 + off);
        }
        assert_eq!(e.mem.u32(dest.addr() + 0x50), 0);
    }
}
