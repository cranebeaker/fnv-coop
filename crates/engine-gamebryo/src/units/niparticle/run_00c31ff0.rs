//! `NiParticle/run_00c31ff0` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiPSysModifier` constructor (`00c31720`, `thiscall`, no words) and
/// member copy (`00c31780`, `thiscall`, `(clone, process)`).
const MODIFIER_CONSTRUCTOR: u32 = 0x00c3_1720;
const MODIFIER_COPY_MEMBERS: u32 = 0x00c3_1780;
/// The `NiPSysFieldModifier` vtable (RTTI `.?AVNiPSysFieldModifier@@`).
const FIELD_MODIFIER_VTABLE: u32 = 0x010c_0b5c;

// Translated from 00c31ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPSysFieldModifier` constructor
/// (`thiscall`, no words): the `NiPSysModifier` part (`00c31720`), then
/// `m_fMagnitude` (+0x1C) and `m_fAttenuation` (+0x20) 0.0, `m_pkFieldObj`
/// (+0x18) null, `m_fMaxDistance` (+0x28) 0.0, `m_bUseMaxDistance` (+0x24)
/// false, `m_fMaxDistanceSqr` (+0x2C) 0.0 (Xbox PDB names), in the exe's
/// order, and the vtable. Answers `this`.
pub fn fn_00c31ff0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MODIFIER_CONSTRUCTOR, &args![this]);
    let t = this.addr();
    let zero = 0.0f32.to_bits();
    e.mem.set_u32(t + 0x1c, zero);
    e.mem.set_u32(t + 0x20, zero);
    e.mem.set_u32(t + 0x18, 0);
    e.mem.set_u32(t + 0x28, zero);
    e.mem.set_u8(t + 0x24, 0);
    e.mem.set_u32(t + 0x2c, zero);
    e.mem.set_u32(t, FIELD_MODIFIER_VTABLE);
    this
}

// Translated from 00c32020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPSysFieldModifier` member copy into
/// `clone` (`thiscall`, `RET 8`): the `NiPSysModifier` copy (`00c31780`),
/// then the field object (`MOV`), magnitude and attenuation (`FLD`/`FSTP`,
/// quieting a signalling NaN), the use-maximum flag, the maximum distance
/// and its square.
pub fn fn_00c32020(e: &mut Engine, this: Ptr, clone: Ptr, process: Ptr) {
    e.call(MODIFIER_COPY_MEMBERS, &args![this, clone, process]);
    let (from, to) = (this.addr(), clone.addr());
    let field_object = e.mem.u32(from + 0x18);
    e.mem.set_u32(to + 0x18, field_object);
    for off in [0x1c, 0x20] {
        let value = x87_quiet_bits(e.mem.u32(from + off));
        e.mem.set_u32(to + off, value);
    }
    let flag = e.mem.u8(from + 0x24);
    e.mem.set_u8(to + 0x24, flag);
    for off in [0x28, 0x2c] {
        let value = x87_quiet_bits(e.mem.u32(from + off));
        e.mem.set_u32(to + off, value);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_1ff0, fn_00c31ff0(Ptr) -> Ptr),
        entry!(0x00c3_2020, fn_00c32020(Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c31ff0_constructs_an_empty_field_modifier() {
        let mut e = Engine::new();
        e.register(MODIFIER_CONSTRUCTOR, |e, a| {
            for off in (0..0x30).step_by(4) {
                e.mem.set_u32(a[0] + off, 0xffff_ffff);
            }
            Ret::default()
        });
        let this = e.mem.alloc(0x30);
        assert_eq!(e.call(0x00c3_1ff0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), FIELD_MODIFIER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x14), 0xffff_ffff);
        for off in [0x18, 0x1c, 0x20, 0x28, 0x2c] {
            assert_eq!(e.mem.u32(this + off), 0);
        }
        assert_eq!(e.mem.u32(this + 0x24), 0xffff_ff00);
    }

    #[test]
    fn fn_00c32020_copies_the_field_members() {
        let mut e = Engine::new();
        e.register(MODIFIER_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        let clone = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x18, 0x1234_5678);
        e.mem.set_u32(this + 0x1c, 0x7f80_0001);
        e.mem.set_f32(this + 0x20, 2.0);
        e.mem.set_u8(this + 0x24, 1);
        e.mem.set_f32(this + 0x28, 3.0);
        e.mem.set_f32(this + 0x2c, 9.0);
        e.call_log = Some(vec![]);
        e.call(0x00c3_2020, &args![this, clone, 0x9000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (MODIFIER_COPY_MEMBERS, vec![this, clone, 0x9000]));
        assert_eq!(e.mem.u32(clone + 0x18), 0x1234_5678);
        assert_eq!(e.mem.u32(clone + 0x1c), 0x7fc0_0001);
        assert_eq!(e.mem.f32(clone + 0x20), 2.0);
        assert_eq!(e.mem.u8(clone + 0x24), 1);
        assert_eq!(e.mem.f32(clone + 0x28), 3.0);
        assert_eq!(e.mem.f32(clone + 0x2c), 9.0);
    }
}
