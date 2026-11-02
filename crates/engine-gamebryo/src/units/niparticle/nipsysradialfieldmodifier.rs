//! `NiParticle/nipsysradialfieldmodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c285c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysRadialFieldModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_26b0.
pub fn ni_p_sys_radial_field_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_26b0
}

use crate::units::niparticle::nipsysdata::NiPSysData;
use crate::units::niparticle::nipsysfieldmodifier::NiPSysFieldModifier;
use crate::units::niparticle::nipsysmodifier::{
    add_class_name, add_labelled, stream_read, stream_write, NiPSysModifier,
    NI_GET_VIEWER_STRING_FLOAT,
};

layout! {
    /// `NiPSysRadialFieldModifier` (Xbox PDB), 0x34 bytes; the PC code uses
    /// the same offsets.
    pub struct NiPSysRadialFieldModifier: 0x34 {
        /// `m_fRadialType` (Xbox PDB), as its bit pattern.
        0x30 m_fRadialType: u32,
    }
}

/// The vtable of `NiPSysRadialFieldModifier` (RTTI
/// `.?AVNiPSysRadialFieldModifier@@`).
const VTABLE: u32 = 0x010b_e99c;
/// The `NiRTTI` record of `NiPSysRadialFieldModifier` (`012026b0`).
const RTTI: u32 = 0x0120_26b0;
/// `00aa13e0` (`cdecl`, one word): the Gamebryo allocation.
const NI_NEW: u32 = 0x00aa_13e0;
/// `00c31ff0` (`thiscall`): the `NiPSysFieldModifier` constructor, and
/// `00c32020` (`thiscall`, `(clone, process)`): its member copy.
const FIELD_MODIFIER_CONSTRUCTOR: u32 = 0x00c3_1ff0;
const FIELD_MODIFIER_COPY_MEMBERS: u32 = 0x00c3_2020;
/// `NiPSysFieldModifier::IsEqual` (`00c32060`), `LoadBinary` (`00c32120`),
/// `SaveBinary` (`00c321f0`) and `GetViewerStrings` (`00c322b0`):
/// `thiscall`, one word.
const FIELD_MODIFIER_IS_EQUAL: u32 = 0x00c3_2060;
const FIELD_MODIFIER_LOAD_BINARY: u32 = 0x00c3_2120;
const FIELD_MODIFIER_SAVE_BINARY: u32 = 0x00c3_21f0;
const FIELD_MODIFIER_GET_VIEWER_STRINGS: u32 = 0x00c3_22b0;
/// `"RadialType"`.
const LABEL_RADIAL_TYPE: u32 = 0x010b_ea40;
/// `NiTransform::InvertNonUniform` (`004b4880`, `thiscall` on the
/// transform, the address of the result) and the transform product
/// `0062c250` (`thiscall` on the left transform, `(result, right)`).
const TRANSFORM_INVERT: u32 = 0x004b_4880;
const TRANSFORM_PRODUCT: u32 = 0x0062_c250;
/// `NiAVObject::m_kWorld` as the PC code reads it: the 13-word world
/// transform (rotation, translation, scale) at +0x68.
const WORLD_TRANSFORM: u32 = 0x68;
/// `_CIpow` (`00ec9b00`: base, exponent, as doubles) and the float maximum
/// `00404010` (`cdecl`, two floats, the larger in `ST0`).
const POW: u32 = 0x00ec_9b00;
const FLOAT_MAX: u32 = 0x0040_4010;
/// `0.5` (`double`): the attenuation is applied to the squared distance.
const HALF: u32 = 0x0101_1588;
/// `NiParticleInfo` (0x1c bytes): the velocity (+0, +4, +8) and the last
/// update time (+0x14).
const INFO_SIZE: u32 = 0x1c;
const INFO_LAST_UPDATE: u32 = 0x14;

// Translated from 00c27770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysRadialFieldModifier::SaveBinary` (Xbox PDB):
/// `NiPSysFieldModifier`'s `SaveBinary`, then the radial type (4 bytes; the
/// component size kept in the stack word that held the stream argument).
pub fn ni_p_sys_radial_field_modifier_save_binary(
    e: &mut Engine,
    this: Ptr<NiPSysRadialFieldModifier>,
    stream: Ptr,
) {
    let s = stream.addr();
    e.call(FIELD_MODIFIER_SAVE_BINARY, &args![this, s]);
    e.with_stack(4, |e, argument_slot| {
        e.mem.set_u32(argument_slot.addr(), s);
        let radial = this.addr() + NiPSysRadialFieldModifier::m_fRadialType.off;
        stream_write(e, s, radial, 4, 4, argument_slot.addr());
    });
}

// Translated from 00c285a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysRadialFieldModifier::IsEqual` (Xbox PDB): `NiPSysFieldModifier`'s
/// `IsEqual` (the radial type is not compared).
pub fn ni_p_sys_radial_field_modifier_is_equal(
    e: &mut Engine,
    this: Ptr<NiPSysRadialFieldModifier>,
    other: Ptr<NiPSysRadialFieldModifier>,
) -> bool {
    e.call(FIELD_MODIFIER_IS_EQUAL, &args![this, other]).bool()
}

// Translated from 00c285d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysRadialFieldModifier::CreateClone` (Xbox PDB): a 0x34-byte object
/// through the `NiPSysFieldModifier` constructor, its radial type 0 and
/// this class's vtable (0 when the allocation fails), then the
/// `NiPSysFieldModifier` member copy into it (which still runs for a null
/// clone; the radial type is not copied). Returns the clone.
pub fn ni_p_sys_radial_field_modifier_create_clone(
    e: &mut Engine,
    this: Ptr<NiPSysRadialFieldModifier>,
    process: Ptr,
) -> Ptr<NiPSysRadialFieldModifier> {
    let block = e.call(NI_NEW, &args![0x34u32]).u32();
    let clone = Ptr::<NiPSysRadialFieldModifier>::new(block);
    if block != 0 {
        e.call(FIELD_MODIFIER_CONSTRUCTOR, &args![block]);
        e.set(clone, NiPSysRadialFieldModifier::m_fRadialType, 0);
        e.mem.set_u32(block, VTABLE);
    }
    e.call(FIELD_MODIFIER_COPY_MEMBERS, &args![this, clone, process]);
    clone
}

/// The field object's world transform relative to the particle system's,
/// as both updates compute it: the two world transforms copied to the stack
/// (the field object's at `object_copy`, the system's at `result`),
/// `InvertNonUniform` of the system's into `inverse`, then the product of
/// that with the field object's into `result`.
fn relative_transform(
    e: &mut Engine,
    this: Ptr<NiPSysRadialFieldModifier>,
    result: u32,
    object_copy: u32,
    inverse: u32,
) {
    let object = e.get(
        this.cast::<NiPSysFieldModifier>(),
        NiPSysFieldModifier::m_pkFieldObj,
    );
    let system = e.get(this.cast::<NiPSysModifier>(), NiPSysModifier::m_pkTarget);
    for i in 0..13 {
        let word = e.mem.u32(object.wrapping_add(WORLD_TRANSFORM + 4 * i));
        e.mem.set_u32(object_copy + 4 * i, word);
    }
    for i in 0..13 {
        let word = e.mem.u32(system.wrapping_add(WORLD_TRANSFORM + 4 * i));
        e.mem.set_u32(result + 4 * i, word);
    }
    e.call(TRANSFORM_INVERT, &args![result, inverse]);
    e.call(TRANSFORM_PRODUCT, &args![inverse, result, object_copy]);
}

/// The velocity add both updates end with: `velocity += v`, x and y as
/// `FLD velocity; FADD v`, z as `FLD v; FADD velocity`.
fn add_velocity(e: &mut Engine, info: u32, v: [u32; 3]) {
    let x = x87_add(x87_load32(e.mem.u32(info)), x87_m32(v[0]));
    e.mem.set_u32(info, x87_store32(x));
    let y_at = info.wrapping_add(4);
    let y = x87_add(x87_load32(e.mem.u32(y_at)), x87_m32(v[1]));
    e.mem.set_u32(y_at, x87_store32(y));
    let z_at = info.wrapping_add(8);
    let z = x87_add(x87_load32(v[2]), x87_m32(e.mem.u32(z_at)));
    e.mem.set_u32(z_at, x87_store32(z));
}

// Translated from 00c28620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysRadialFieldModifier::UpdateNoAttenuationNoDirNoDist` (Xbox PDB):
/// the field object's transform relative to the particle system; then for
/// each active particle whose time since its last update (rounded to a
/// float) is not 0, its offset from the field's translation times
/// `time * magnitude` is added to its velocity. The position pointer only
/// moves on for the particles that are updated (the exe advances it there),
/// so after a skipped particle the next one uses the skipped one's
/// position. x87 operations one by one in the exe's order.
pub fn ni_p_sys_radial_field_modifier_update_no_attenuation_no_dir_no_dist(
    e: &mut Engine,
    this: Ptr<NiPSysRadialFieldModifier>,
    time: f32,
    data: Ptr<NiPSysData>,
) {
    let field = this.cast::<NiPSysFieldModifier>();
    e.with_stack(0xc8, |e, frame| {
        let at = |x: u32| frame.addr() + x;
        relative_transform(e, this, at(0x28), at(0x5c), at(0x90));
        let magnitude = x87_quiet_bits(e.get(field, NiPSysFieldModifier::m_fMagnitude));
        let mut position = e.get(data, NiPSysData::m_pkVertex);
        e.mem.set_u32(at(0xc), magnitude);
        let mut i: u16 = 0;
        if i >= e.get(data, NiPSysData::m_usActive) {
            return;
        }
        let local = |e: &Engine, x: u32| x87_load32(e.mem.u32(at(x)));
        let now = x87_load32(time.to_bits());
        let (tz, ty, tx, magnitude) = (
            local(e, 0x54),
            local(e, 0x50),
            local(e, 0x4c),
            local(e, 0xc),
        );
        loop {
            let info = e
                .get(data, NiPSysData::m_pkParticleInfo)
                .wrapping_add(u32::from(i) * INFO_SIZE);
            let last = e.mem.u32(info.wrapping_add(INFO_LAST_UPDATE));
            e.mem
                .set_u32(at(0xc), x87_store32(x87_sub(now, x87_m32(last))));
            let elapsed = local(e, 0xc);
            // `FUCOM` with 0; `TEST AH,0x44; JNP`: skipped only when equal.
            if elapsed != 0.0 {
                let p = position;
                position = position.wrapping_add(0xc);
                for (k, t) in [tx, ty, tz].into_iter().enumerate() {
                    let offset = x87_sub(x87_load32(e.mem.u32(p + 4 * k as u32)), t);
                    e.mem.set_u32(at(0x1c + 4 * k as u32), x87_store32(offset));
                }
                e.mem
                    .set_u32(at(0xc), x87_store32(x87_mul(elapsed, magnitude)));
                let step = local(e, 0xc);
                let vx = x87_store32(x87_mul(local(e, 0x1c), step));
                e.mem.set_u32(at(0x10), vx);
                let vy = x87_store32(x87_mul(local(e, 0x20), step));
                e.mem.set_u32(at(0x14), vy);
                let vz = x87_store32(x87_mul(step, x87_m32(e.mem.u32(at(0x24)))));
                e.mem.set_u32(at(0x18), vz);
                add_velocity(e, info, [vx, vy, vz]);
            }
            i = i.wrapping_add(1);
            if i >= e.get(data, NiPSysData::m_usActive) {
                break;
            }
        }
    });
}

// Translated from 00c28760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysRadialFieldModifier::LoadBinary` (Xbox PDB):
/// `NiPSysFieldModifier`'s `LoadBinary`, then the radial type (4 bytes; the
/// component size kept in the stack word that held the stream argument).
pub fn ni_p_sys_radial_field_modifier_load_binary(
    e: &mut Engine,
    this: Ptr<NiPSysRadialFieldModifier>,
    stream: Ptr,
) {
    let s = stream.addr();
    e.call(FIELD_MODIFIER_LOAD_BINARY, &args![this, s]);
    e.with_stack(4, |e, argument_slot| {
        e.mem.set_u32(argument_slot.addr(), s);
        let radial = this.addr() + NiPSysRadialFieldModifier::m_fRadialType.off;
        stream_read(e, s, radial, 4, 4, argument_slot.addr());
    });
}

// Translated from 00c287a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysRadialFieldModifier::Update` (Xbox PDB). Nothing happens with a
/// magnitude of exactly 0, no active particles or no field object. Without
/// the maximum distance, an attenuation or a radial type of exactly 0 runs
/// `UpdateNoAttenuationNoDirNoDist` (`00c28620`). Otherwise, with the field
/// object's transform relative to the particle system: for each active
/// particle whose time since its last update (rounded to a float) is not 0,
/// its offset from the field's translation and the squared length of that;
/// with the maximum distance used, a particle beyond it (`m_fMaxDistanceSqr`
/// below the squared length, or a NaN) is skipped; with a non-zero
/// attenuation and length the step is `time * magnitude / max(1,
/// pow(d2, attenuation * 0.5) * radial type)`, else `time * magnitude`;
/// the offset times the step is added to the velocity. x87 operations one
/// by one in the exe's order; the stack slots are the exe's.
pub fn ni_p_sys_radial_field_modifier_update(
    e: &mut Engine,
    this: Ptr<NiPSysRadialFieldModifier>,
    time: f32,
    data: Ptr<NiPSysData>,
) {
    let field = this.cast::<NiPSysFieldModifier>();
    // `FLDZ; FCOM magnitude; TEST AH,0x44; JNP`: stops only when equal.
    if 0.0 == x87_m32(e.get(field, NiPSysFieldModifier::m_fMagnitude)) {
        return;
    }
    if e.get(data, NiPSysData::m_usActive) == 0 {
        return;
    }
    if e.get(field, NiPSysFieldModifier::m_pkFieldObj) == 0 {
        return;
    }
    let use_max_distance = e.get(field, NiPSysFieldModifier::m_bUseMaxDistance) != 0;
    if !use_max_distance
        && (0.0 == x87_m32(e.get(field, NiPSysFieldModifier::m_fAttenuation))
            || 0.0 == x87_m32(e.get(this, NiPSysRadialFieldModifier::m_fRadialType)))
    {
        ni_p_sys_radial_field_modifier_update_no_attenuation_no_dir_no_dist(e, this, time, data);
        return;
    }
    e.with_stack(0x100, |e, frame| {
        let at = |x: u32| frame.addr() + x;
        relative_transform(e, this, at(0x64), at(0x98), at(0xcc));
        let half: f64 = e.global(HALF);
        let attenuation = x87_load32(e.get(field, NiPSysFieldModifier::m_fAttenuation));
        let mut i: u16 = 0;
        e.mem.set_u32(at(0x20), 0);
        e.mem
            .set_u32(at(0x48), x87_store32(x87_mul(attenuation, half)));
        if i >= e.get(data, NiPSysData::m_usActive) {
            return;
        }
        let local = |e: &Engine, x: u32| x87_load32(e.mem.u32(at(x)));
        loop {
            let k = u32::from(i);
            let info = e
                .get(data, NiPSysData::m_pkParticleInfo)
                .wrapping_add(k * INFO_SIZE);
            let now = x87_load32(time.to_bits());
            let last = e.mem.u32(info.wrapping_add(INFO_LAST_UPDATE));
            e.mem
                .set_u32(at(0x10), x87_store32(x87_sub(now, x87_m32(last))));
            let elapsed = local(e, 0x10);
            // `FUCOM` with 0; `TEST AH,0x44; JNP`: skipped only when equal.
            if elapsed != 0.0 {
                let position = e.get(data, NiPSysData::m_pkVertex).wrapping_add(k * 12);
                for c in 0..3 {
                    let word = e.mem.u32(position.wrapping_add(4 * c));
                    e.mem.set_u32(at(0x30 + 4 * c), word);
                }
                for c in 0..3 {
                    let offset =
                        x87_sub(local(e, 0x30 + 4 * c), x87_m32(e.mem.u32(at(0x88 + 4 * c))));
                    e.mem.set_u32(at(0x24 + 4 * c), x87_store32(offset));
                }
                let (oy, ox, oz) = (local(e, 0x28), local(e, 0x24), local(e, 0x2c));
                let xx = x87_mul(ox, ox);
                let yy = x87_mul(oy, oy);
                let sum = x87_add(yy, xx);
                let zz = x87_mul(oz, oz);
                e.mem.set_u32(at(0x10), x87_store32(x87_add(zz, sum)));
                let squared = local(e, 0x10);
                // `FLD max²; FCOMP d²; TEST AH,1; JNZ`: skipped when below
                // (or unordered).
                let beyond = use_max_distance
                    && !(x87_load32(e.get(field, NiPSysFieldModifier::m_fMaxDistanceSqr))
                        >= squared);
                if !beyond {
                    let magnitude = x87_load32(e.get(field, NiPSysFieldModifier::m_fMagnitude));
                    let attenuated = 0.0
                        != x87_m32(e.get(field, NiPSysFieldModifier::m_fAttenuation))
                        && 0.0 != squared;
                    let v = if attenuated {
                        let scaled = x87_mul(elapsed, magnitude);
                        let exponent = local(e, 0x48);
                        let power = e.call(POW, &args![squared, exponent]).f64();
                        e.mem.set_u32(at(0x10), x87_store32(power));
                        let radial = x87_m32(e.get(this, NiPSysRadialFieldModifier::m_fRadialType));
                        let weighted = x87_store32(x87_mul(local(e, 0x10), radial));
                        e.mem.set_u32(at(0x10), weighted);
                        let weighted = x87_store32(local(e, 0x10));
                        let bigger = e.call(FLOAT_MAX, &args![1.0f32.to_bits(), weighted]).f64();
                        e.mem
                            .set_u32(at(0x10), x87_store32(x87_div(scaled, bigger)));
                        let step = local(e, 0x10);
                        let vx = x87_store32(x87_mul(step, x87_m32(e.mem.u32(at(0x24)))));
                        e.mem.set_u32(at(0x4c), vx);
                        let vy = x87_store32(x87_mul(local(e, 0x28), step));
                        e.mem.set_u32(at(0x50), vy);
                        let vz = x87_store32(x87_mul(step, x87_m32(e.mem.u32(at(0x2c)))));
                        e.mem.set_u32(at(0x54), vz);
                        [vx, vy, vz]
                    } else {
                        e.mem
                            .set_u32(at(0x10), x87_store32(x87_mul(elapsed, magnitude)));
                        let step = local(e, 0x10);
                        let vx = x87_store32(x87_mul(ox, step));
                        e.mem.set_u32(at(0x3c), vx);
                        let vy = x87_store32(x87_mul(oy, step));
                        e.mem.set_u32(at(0x40), vy);
                        let vz = x87_store32(x87_mul(step, oz));
                        e.mem.set_u32(at(0x44), vz);
                        [vx, vy, vz]
                    };
                    e.mem.set_u32(at(0x14), v[0]);
                    e.mem.set_u32(at(0x18), v[1]);
                    e.mem.set_u32(at(0x1c), v[2]);
                    add_velocity(e, info, v);
                }
            }
            i = i.wrapping_add(1);
            e.mem.set_u32(at(0x20), u32::from(i));
            if i >= e.get(data, NiPSysData::m_usActive) {
                break;
            }
        }
    });
}

// Translated from 00c28a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysRadialFieldModifier::GetViewerStrings` (Xbox PDB):
/// `NiPSysFieldModifier`'s `GetViewerStrings`, then the class name
/// (`NiRTTI` `012026b0`) and `"RadialType"` (the float through
/// `FLD`/`FSTP`) appended.
pub fn ni_p_sys_radial_field_modifier_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiPSysRadialFieldModifier>,
    strings: Ptr,
) {
    e.call(FIELD_MODIFIER_GET_VIEWER_STRINGS, &args![this, strings]);
    add_class_name(e, strings.addr(), RTTI);
    let radial = x87_quiet_bits(e.get(this, NiPSysRadialFieldModifier::m_fRadialType));
    add_labelled(
        e,
        strings.addr(),
        NI_GET_VIEWER_STRING_FLOAT,
        LABEL_RADIAL_TYPE,
        radial,
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c285c0, ni_p_sys_radial_field_modifier_get_r_t_t_i() -> u32),
        entry!(
            0x00c2_7770,
            ni_p_sys_radial_field_modifier_save_binary(Ptr<NiPSysRadialFieldModifier>, Ptr)
        ),
        entry!(
            0x00c2_85a0,
            ni_p_sys_radial_field_modifier_is_equal(
                Ptr<NiPSysRadialFieldModifier>,
                Ptr<NiPSysRadialFieldModifier>,
            ) -> bool
        ),
        entry!(
            0x00c2_85d0,
            ni_p_sys_radial_field_modifier_create_clone(
                Ptr<NiPSysRadialFieldModifier>,
                Ptr,
            )
                -> Ptr<NiPSysRadialFieldModifier>
        ),
        entry!(
            0x00c2_8620,
            ni_p_sys_radial_field_modifier_update_no_attenuation_no_dir_no_dist(
                Ptr<NiPSysRadialFieldModifier>,
                f32,
                Ptr<NiPSysData>,
            )
        ),
        entry!(
            0x00c2_8760,
            ni_p_sys_radial_field_modifier_load_binary(Ptr<NiPSysRadialFieldModifier>, Ptr)
        ),
        entry!(
            0x00c2_87a0,
            ni_p_sys_radial_field_modifier_update(
                Ptr<NiPSysRadialFieldModifier>,
                f32,
                Ptr<NiPSysData>,
            )
        ),
        entry!(
            0x00c2_8a90,
            ni_p_sys_radial_field_modifier_get_viewer_strings(Ptr<NiPSysRadialFieldModifier>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_radial_field_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_85c0, &args![]).u32(), 0x0120_26b0);
    }
}

#[cfg(test)]
mod radial_tests {
    use super::*;
    use crate::units::niparticle::nipsysmodifier::test_stream;
    use crate::units::niparticle::nipsysmodifier::test_stream::calls_to;

    fn f(v: f32) -> u32 {
        v.to_bits()
    }

    #[test]
    fn save_and_load_binary_handle_the_radial_type() {
        let mut e = Engine::new();
        e.register(FIELD_MODIFIER_SAVE_BINARY, |_, _| Ret::default());
        e.register(FIELD_MODIFIER_LOAD_BINARY, |_, _| Ret::default());
        let writes = test_stream::writes(&mut e);
        let output = test_stream::new_output(&mut e);
        let this = e.new_object::<NiPSysRadialFieldModifier>();
        e.set(this, NiPSysRadialFieldModifier::m_fRadialType, f(0.25));
        e.call_log = Some(vec![]);
        e.call(0x00c2_7770, &args![this, output]);
        assert_eq!(
            calls_to(&e, FIELD_MODIFIER_SAVE_BINARY),
            vec![vec![this.addr(), output]]
        );
        assert_eq!(*writes.borrow(), vec![(0.25f32.to_le_bytes().to_vec(), 4)]);
        let input = test_stream::new(&mut e, 0x1402_0007, &0.75f32.to_le_bytes());
        e.call(0x00c2_8760, &args![this, input]);
        assert_eq!(
            calls_to(&e, FIELD_MODIFIER_LOAD_BINARY),
            vec![vec![this.addr(), input]]
        );
        assert_eq!(
            e.get(this, NiPSysRadialFieldModifier::m_fRadialType),
            f(0.75)
        );
    }

    #[test]
    fn is_equal_is_the_field_modifier_answer() {
        let mut e = Engine::new();
        e.register(FIELD_MODIFIER_IS_EQUAL, |_, _| Ret {
            eax: 0x1201,
            ..Ret::default()
        });
        assert!(e.call(0x00c2_85a0, &args![0x5000u32, 0x6000u32]).bool());
        e.register(FIELD_MODIFIER_IS_EQUAL, |_, _| Ret {
            eax: 0x1200,
            ..Ret::default()
        });
        assert!(!e.call(0x00c2_85a0, &args![0x5000u32, 0x6000u32]).bool());
    }

    #[test]
    fn create_clone_starts_with_radial_type_zero() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(FIELD_MODIFIER_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x30, 0x1234);
            Ret::default()
        });
        e.register(FIELD_MODIFIER_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c2_85d0, &args![0x5000u32, 0x6000u32]).u32();
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x34]]);
        assert_eq!(e.mem.u32(clone), VTABLE);
        assert_eq!(e.mem.u32(clone + 0x30), 0);
        assert_eq!(
            calls_to(&e, FIELD_MODIFIER_COPY_MEMBERS),
            vec![vec![0x5000, clone, 0x6000]]
        );
        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_85d0, &args![0x5000u32, 0x6000u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, FIELD_MODIFIER_COPY_MEMBERS),
            vec![vec![0x5000, 0, 0x6000]]
        );
    }

    /// A radial field with an identity relative transform translated by
    /// `(1, 0, 0)`, magnitude 2, and three particles at `(3, 0, 0)`,
    /// `(5, 0, 0)` and `(7, 0, 0)` last updated at 1.0, 1.5 and 1.0.
    fn field(e: &mut Engine) -> (Ptr<NiPSysRadialFieldModifier>, Ptr<NiPSysData>) {
        e.register(TRANSFORM_INVERT, |_, _| Ret::default());
        e.register(TRANSFORM_PRODUCT, |e, a| {
            for i in 0..13 {
                e.mem.set_u32(a[1] + 4 * i, 0);
            }
            for i in [0, 4, 8, 12] {
                e.mem.set_u32(a[1] + 4 * i, 1.0f32.to_bits());
            }
            e.mem.set_u32(a[1] + 0x24, 1.0f32.to_bits());
            Ret::default()
        });
        e.register(FLOAT_MAX, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            Ret {
                st0: f64::from(if x > y { x } else { y }),
                ..Ret::default()
            }
        });
        e.register(POW, |_, a| {
            let base = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            let exponent = f64::from_bits(u64::from(a[2]) | u64::from(a[3]) << 32);
            Ret {
                st0: base.powf(exponent),
                ..Ret::default()
            }
        });
        e.map(HALF & !0xfff, 0x1000);
        e.mem.set_f64(HALF, 0.5);
        let object = e.mem.alloc(0x100);
        let system = e.mem.alloc(0x100);
        let this = e.new_object::<NiPSysRadialFieldModifier>();
        e.mem.set_u32(this.addr() + 0x10, system);
        e.mem.set_u32(this.addr() + 0x18, object);
        e.mem.set_u32(this.addr() + 0x1c, f(2.0));
        let data = e.new_object::<NiPSysData>();
        let infos = e.mem.alloc(3 * INFO_SIZE);
        for (i, last) in [1.0f32, 1.5, 1.0].iter().enumerate() {
            e.mem.set_u32(
                infos + i as u32 * INFO_SIZE + INFO_LAST_UPDATE,
                last.to_bits(),
            );
        }
        let vertices = e.mem.alloc(36);
        for (i, x) in [3.0f32, 5.0, 7.0].iter().enumerate() {
            e.mem.set_u32(vertices + 12 * i as u32, x.to_bits());
        }
        e.set(data, NiPSysData::m_pkParticleInfo, infos);
        e.set(data, NiPSysData::m_pkVertex, vertices);
        e.set(data, NiPSysData::m_usActive, 3);
        (this, data)
    }

    fn velocity_x(e: &Engine, data: Ptr<NiPSysData>, i: u32) -> u32 {
        e.mem
            .u32(e.get(data, NiPSysData::m_pkParticleInfo) + i * INFO_SIZE)
    }

    #[test]
    fn simple_update_pushes_along_the_offset_and_lags_the_positions() {
        let mut e = Engine::new();
        let (this, data) = field(&mut e);
        e.call(0x00c2_8620, &args![this, 1.5f32, data]);
        // (3 - 1) * 0.5 * 2.
        assert_eq!(velocity_x(&e, data, 0), f(2.0));
        // No time passed: skipped, and the position pointer stays.
        assert_eq!(velocity_x(&e, data, 1), 0);
        // So the third particle uses the second one's position: (5 - 1).
        assert_eq!(velocity_x(&e, data, 2), f(4.0));
    }

    #[test]
    fn update_runs_the_simple_update_without_attenuation_or_radial_type() {
        for (attenuation, radial) in [(0.0f32, 1.0f32), (2.0, 0.0)] {
            let mut e = Engine::new();
            let (this, data) = field(&mut e);
            e.mem.set_u32(this.addr() + 0x20, f(attenuation));
            e.mem.set_u32(this.addr() + 0x30, f(radial));
            e.call_log = Some(vec![]);
            e.call(0x00c2_87a0, &args![this, 1.5f32, data]);
            assert!(calls_to(&e, POW).is_empty());
            assert_eq!(velocity_x(&e, data, 2), f(4.0));
        }
    }

    #[test]
    fn update_skips_without_magnitude_particles_or_field_object() {
        let mut e = Engine::new();
        let (this, data) = field(&mut e);
        e.mem.set_u32(this.addr() + 0x1c, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c2_87a0, &args![this, 1.5f32, data]);
        assert!(calls_to(&e, TRANSFORM_INVERT).is_empty());
        e.mem.set_u32(this.addr() + 0x1c, f(2.0));
        e.mem.set_u32(this.addr() + 0x18, 0);
        e.call(0x00c2_87a0, &args![this, 1.5f32, data]);
        assert!(calls_to(&e, TRANSFORM_INVERT).is_empty());
        e.mem.set_u32(this.addr() + 0x18, 0x9000);
        e.set(data, NiPSysData::m_usActive, 0);
        e.call(0x00c2_87a0, &args![this, 1.5f32, data]);
        assert!(calls_to(&e, TRANSFORM_INVERT).is_empty());
    }

    #[test]
    fn update_with_attenuation_weights_the_distance_power_by_the_radial_type() {
        let mut e = Engine::new();
        let (this, data) = field(&mut e);
        // Attenuation 2 (exponent 1), radial type 0.5.
        e.mem.set_u32(this.addr() + 0x20, f(2.0));
        e.mem.set_u32(this.addr() + 0x30, f(0.5));
        e.call_log = Some(vec![]);
        e.call(0x00c2_87a0, &args![this, 1.5f32, data]);
        let pow = calls_to(&e, POW);
        assert_eq!(pow[0], args![4.0f64, 1.0f64]);
        assert_eq!(pow[1], args![36.0f64, 1.0f64]);
        // max(1, 4 * 0.5) = 2: 2 * (0.5 * 2) / 2.
        assert_eq!(calls_to(&e, FLOAT_MAX)[0], vec![f(1.0), f(2.0)]);
        assert_eq!(velocity_x(&e, data, 0), f(1.0));
        assert_eq!(velocity_x(&e, data, 1), 0);
        // The general update reads every particle's own position: the step
        // 1 / 18 rounded to a float, times 6.
        let step = (1.0f64 / 18.0) as f32;
        assert_eq!(velocity_x(&e, data, 2), f((f64::from(step) * 6.0) as f32));
    }

    #[test]
    fn update_with_max_distance_skips_far_particles() {
        let mut e = Engine::new();
        let (this, data) = field(&mut e);
        e.mem.set_u8(this.addr() + 0x24, 1);
        e.mem.set_u32(this.addr() + 0x2c, f(16.0));
        e.call(0x00c2_87a0, &args![this, 1.5f32, data]);
        // d² 4 is within, d² 36 beyond.
        assert_eq!(velocity_x(&e, data, 0), f(2.0));
        assert_eq!(velocity_x(&e, data, 2), 0);
    }

    #[test]
    fn get_viewer_strings_appends_class_and_radial_type() {
        let mut e = Engine::new();
        test_stream::global_word(&mut e, RTTI, 0x0abc_0009);
        e.register(FIELD_MODIFIER_GET_VIEWER_STRINGS, |_, _| Ret::default());
        let stored = test_stream::viewer_strings(&mut e);
        let strings = test_stream::strings(&mut e);
        let this = e.new_object::<NiPSysRadialFieldModifier>();
        e.set(this, NiPSysRadialFieldModifier::m_fRadialType, f(0.5));
        e.call_log = Some(vec![]);
        e.call(0x00c2_8a90, &args![this, strings]);
        assert_eq!(
            calls_to(&e, FIELD_MODIFIER_GET_VIEWER_STRINGS),
            vec![vec![this.addr(), strings]]
        );
        assert_eq!(*stored.borrow(), vec![0x0abc_0009, LABEL_RADIAL_TYPE]);
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_FLOAT),
            vec![vec![LABEL_RADIAL_TYPE, f(0.5)]]
        );
    }
}
