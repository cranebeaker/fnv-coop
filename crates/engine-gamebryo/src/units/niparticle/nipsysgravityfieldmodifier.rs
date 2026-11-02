//! `NiParticle/nipsysgravityfieldmodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c2a9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGravityFieldModifier::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00c31750`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x48 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_p_sys_gravity_field_modifier_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00c3_1750;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x48u32]);
    }
    this
}

// Translated from 00c2a9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGravityFieldModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2708.
pub fn ni_p_sys_gravity_field_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2708
}

use crate::units::niparticle::nipsysdata::NiPSysData;
use crate::units::niparticle::nipsysfieldmodifier::NiPSysFieldModifier;
use crate::units::niparticle::nipsysmodifier::{add_class_name, add_viewer_string, NiPSysModifier};

layout! {
    /// `NiPSysGravityFieldModifier` (Xbox PDB), 0x48 bytes; the PC code uses
    /// the same offsets. The floats are declared as their bit patterns.
    pub struct NiPSysGravityFieldModifier: 0x48 {
        /// `m_kDirection` (Xbox PDB): `NiPoint3`.
        0x30 m_kDirection: u32,
        /// `m_kUnitDirection` (Xbox PDB): `NiPoint3`.
        0x3C m_kUnitDirection: u32,
    }
}

/// The vtable of `NiPSysGravityFieldModifier` (RTTI
/// `.?AVNiPSysGravityFieldModifier@@`).
const VTABLE: u32 = 0x010b_f2ac;
/// The `NiRTTI` record of `NiPSysGravityFieldModifier` (`01202708`).
const RTTI: u32 = 0x0120_2708;
/// `00aa13e0` (`cdecl`, one word): the Gamebryo allocation.
const NI_NEW: u32 = 0x00aa_13e0;
/// `00c31ff0` (`thiscall`, no stack words): the `NiPSysFieldModifier`
/// constructor (it stores that class's vtable `010c0b5c`).
const FIELD_MODIFIER_CONSTRUCTOR: u32 = 0x00c3_1ff0;
/// `00c32020` (`thiscall`, `(clone, process)`): the `NiPSysFieldModifier`
/// member copy.
const FIELD_MODIFIER_COPY_MEMBERS: u32 = 0x00c3_2020;
/// `NiPSysFieldModifier::IsEqual` (`00c32060`), `LoadBinary` (`00c32120`),
/// `SaveBinary` (`00c321f0`) and `GetViewerStrings` (`00c322b0`):
/// `thiscall`, one word.
const FIELD_MODIFIER_IS_EQUAL: u32 = 0x00c3_2060;
const FIELD_MODIFIER_LOAD_BINARY: u32 = 0x00c3_2120;
const FIELD_MODIFIER_SAVE_BINARY: u32 = 0x00c3_21f0;
const FIELD_MODIFIER_GET_VIEWER_STRINGS: u32 = 0x00c3_22b0;
/// The point helpers the code calls on an `NiPoint3` (`thiscall`):
/// `00a59be0` and `00a69780` read and write its three floats from and to a
/// stream (one word; the engine map names them after `NiColor`, whose
/// bodies they share), `00439090` answers whether it differs from another
/// point (one word), `004a0c10` normalizes it in place, and `00a59af0`
/// (`NiPoint3::GetViewerString`) makes its viewer string with a label.
const POINT_LOAD_BINARY: u32 = 0x00a5_9be0;
const POINT_SAVE_BINARY: u32 = 0x00a6_9780;
const POINT_NOT_EQUAL: u32 = 0x0043_9090;
const POINT_UNITIZE: u32 = 0x004a_0c10;
const POINT_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
/// The unit point `(0, 1, 0)` (`011a9478`) whose negation is the default
/// direction.
const UNIT_Y: u32 = 0x011a_9478;
/// `"Direction"`.
const LABEL_DIRECTION: u32 = 0x0104_4b44;
/// `NiTransform::InvertNonUniform` (`004b4880`, `thiscall` on the
/// transform, the address of the result) and the transform product
/// `0062c250` (`thiscall` on the left transform, `(result, right)`).
const TRANSFORM_INVERT: u32 = 0x004b_4880;
const TRANSFORM_PRODUCT: u32 = 0x0062_c250;
/// `004b4500` (`thiscall` on an `NiMatrix3`, `(result, point)`): the matrix
/// times a point.
const MATRIX_TIMES_POINT: u32 = 0x004b_4500;
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

// Translated from 00c267f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGravityFieldModifier::SaveBinary` (Xbox PDB):
/// `NiPSysFieldModifier`'s `SaveBinary`, then the direction (`00a69780`).
pub fn ni_p_sys_gravity_field_modifier_save_binary(
    e: &mut Engine,
    this: Ptr<NiPSysGravityFieldModifier>,
    stream: Ptr,
) {
    e.call(FIELD_MODIFIER_SAVE_BINARY, &args![this, stream]);
    let direction = this.addr() + NiPSysGravityFieldModifier::m_kDirection.off;
    e.call(POINT_SAVE_BINARY, &args![direction, stream]);
}

// Translated from 00c26db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGravityFieldModifier::IsEqual` (Xbox PDB): `NiPSysFieldModifier`'s
/// `IsEqual`, then the directions must not differ (`00439090` on the other
/// modifier's direction with this one's).
pub fn ni_p_sys_gravity_field_modifier_is_equal(
    e: &mut Engine,
    this: Ptr<NiPSysGravityFieldModifier>,
    other: Ptr<NiPSysGravityFieldModifier>,
) -> bool {
    if !e.call(FIELD_MODIFIER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let off = NiPSysGravityFieldModifier::m_kDirection.off;
    !e.call(
        POINT_NOT_EQUAL,
        &args![other.addr() + off, this.addr() + off],
    )
    .bool()
}

// Translated from 00c2a9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPSysGravityFieldModifier` constructor
/// (it stores the vtable `010bf2ac`). The `NiPSysFieldModifier` constructor,
/// then the direction and unit direction both `-(0, 1, 0)` (each coordinate
/// of `011a9478` through `FLD`/`FCHS`/`FSTP`), the unit direction normalized
/// (`004a0c10`). Returns `this`.
pub fn fn_00c2a9e0(
    e: &mut Engine,
    this: Ptr<NiPSysGravityFieldModifier>,
) -> Ptr<NiPSysGravityFieldModifier> {
    e.call(FIELD_MODIFIER_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    let direction = this.addr() + NiPSysGravityFieldModifier::m_kDirection.off;
    let unit = this.addr() + NiPSysGravityFieldModifier::m_kUnitDirection.off;
    for i in 0..3 {
        let negated = x87_quiet_bits(e.global::<u32>(UNIT_Y + 4 * i)) ^ 0x8000_0000;
        e.mem.set_u32(direction + 4 * i, negated);
        e.mem.set_u32(unit + 4 * i, negated);
    }
    e.call(POINT_UNITIZE, &args![unit]);
    this
}

// Translated from 00c2aa50 (bytes disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it): the `NiPSysGravityFieldModifier`
/// factory. A 0x48-byte object from `00aa13e0` through the constructor
/// (`00c2a9e0`, a tail jump), or 0 when the allocation fails.
pub fn fn_00c2aa50(e: &mut Engine) -> u32 {
    let block = e.call(NI_NEW, &args![0x48u32]).u32();
    if block == 0 {
        return 0;
    }
    fn_00c2a9e0(e, Ptr::new(block)).addr()
}

// Translated from 00c2aa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGravityFieldModifier::LoadBinary` (Xbox PDB):
/// `NiPSysFieldModifier`'s `LoadBinary`, the direction (`00a59be0`), the
/// direction's words copied onto themselves (the inlined point assignment),
/// then the unit direction set to the direction and normalized
/// (`004a0c10`).
pub fn ni_p_sys_gravity_field_modifier_load_binary(
    e: &mut Engine,
    this: Ptr<NiPSysGravityFieldModifier>,
    stream: Ptr,
) {
    e.call(FIELD_MODIFIER_LOAD_BINARY, &args![this, stream]);
    let direction = this.addr() + NiPSysGravityFieldModifier::m_kDirection.off;
    e.call(POINT_LOAD_BINARY, &args![direction, stream]);
    for i in 0..3 {
        let word = e.mem.u32(direction + 4 * i);
        e.mem.set_u32(direction + 4 * i, word);
    }
    let unit = this.addr() + NiPSysGravityFieldModifier::m_kUnitDirection.off;
    for i in 0..3 {
        let word = e.mem.u32(direction + 4 * i);
        e.mem.set_u32(unit + 4 * i, word);
    }
    e.call(POINT_UNITIZE, &args![unit]);
}

// Translated from 00c2aac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGravityFieldModifier::UpdateNoAttenuationNoDist` (Xbox PDB): the
/// field object's world transform relative to the particle system's
/// (`InvertNonUniform` of the system's world transform, then the product
/// `0062c250` with the field object's, both copied to the stack first);
/// the direction rotated by that rotation, normalized (`004a0c10`) and
/// scaled by the magnitude; then each active particle whose time since its
/// last update (`time` minus +0x14, rounded to a float) is not 0 gets that
/// vector times the time added to its velocity. x87 operations one by one
/// in the exe's order.
pub fn ni_p_sys_gravity_field_modifier_update_no_attenuation_no_dist(
    e: &mut Engine,
    this: Ptr<NiPSysGravityFieldModifier>,
    time: f32,
    data: Ptr<NiPSysData>,
) {
    let modifier = this.cast::<NiPSysModifier>();
    let field = this.cast::<NiPSysFieldModifier>();
    e.with_stack(0xc0, |e, frame| {
        let at = |x: u32| frame.addr() + x;
        let object = e.get(field, NiPSysFieldModifier::m_pkFieldObj);
        let system = e.get(modifier, NiPSysModifier::m_pkTarget);
        for i in 0..13 {
            let word = e.mem.u32(object.wrapping_add(WORLD_TRANSFORM + 4 * i));
            e.mem.set_u32(at(0x50 + 4 * i), word);
        }
        for i in 0..13 {
            let word = e.mem.u32(system.wrapping_add(WORLD_TRANSFORM + 4 * i));
            e.mem.set_u32(at(0x1c + 4 * i), word);
        }
        e.call(TRANSFORM_INVERT, &args![at(0x1c), at(0x84)]);
        e.call(TRANSFORM_PRODUCT, &args![at(0x84), at(0x1c), at(0x50)]);
        let m = |e: &Engine, i: u32| x87_m32(e.mem.u32(at(0x1c + 4 * i)));
        let direction = this.addr() + NiPSysGravityFieldModifier::m_kDirection.off;
        let d = |e: &Engine, i: u32| x87_load32(e.mem.u32(direction + 4 * i));
        let r0 = x87_add(
            x87_add(x87_mul(d(e, 1), m(e, 1)), x87_mul(d(e, 0), m(e, 0))),
            x87_mul(d(e, 2), m(e, 2)),
        );
        e.mem.set_u32(at(0xc), x87_store32(r0));
        let r1 = x87_add(
            x87_add(x87_mul(d(e, 0), m(e, 3)), x87_mul(d(e, 1), m(e, 4))),
            x87_mul(d(e, 2), m(e, 5)),
        );
        e.mem.set_u32(at(0x10), x87_store32(r1));
        let r2 = x87_add(
            x87_add(x87_mul(d(e, 0), m(e, 6)), x87_mul(d(e, 1), m(e, 7))),
            x87_mul(d(e, 2), m(e, 8)),
        );
        e.mem.set_u32(at(0x14), x87_store32(r2));
        e.call(POINT_UNITIZE, &args![at(0xc)]);
        let magnitude = x87_quiet_bits(e.get(field, NiPSysFieldModifier::m_fMagnitude));
        e.mem.set_u32(at(0x18), magnitude);
        let local = |e: &Engine, x: u32| x87_load32(e.mem.u32(at(x)));
        let magnitude = local(e, 0x18);
        let a0 = x87_mul(local(e, 0xc), magnitude);
        e.mem.set_u32(at(0xc), x87_store32(a0));
        let a1 = x87_mul(local(e, 0x10), magnitude);
        e.mem.set_u32(at(0x10), x87_store32(a1));
        let a2 = x87_mul(magnitude, x87_m32(e.mem.u32(at(0x14))));
        e.mem.set_u32(at(0x14), x87_store32(a2));
        let mut i: u16 = 0;
        if i >= e.get(data, NiPSysData::m_usActive) {
            return;
        }
        let now = x87_load32(time.to_bits());
        let (g2, g1, g0) = (local(e, 0x14), local(e, 0x10), local(e, 0xc));
        loop {
            let info = e
                .get(data, NiPSysData::m_pkParticleInfo)
                .wrapping_add(u32::from(i) * INFO_SIZE);
            let last = e.mem.u32(info.wrapping_add(INFO_LAST_UPDATE));
            e.mem
                .set_u32(at(0x18), x87_store32(x87_sub(now, x87_m32(last))));
            let elapsed = local(e, 0x18);
            // `FUCOM` with 0; `TEST AH,0x44; JNP`: skipped only when equal.
            if elapsed != 0.0 {
                e.mem.set_u32(at(0xc), x87_store32(x87_mul(g0, elapsed)));
                e.mem.set_u32(at(0x10), x87_store32(x87_mul(g1, elapsed)));
                e.mem.set_u32(at(0x14), x87_store32(x87_mul(elapsed, g2)));
                for k in 0..3 {
                    let velocity = info.wrapping_add(4 * k);
                    let sum = x87_add(
                        x87_load32(e.mem.u32(velocity)),
                        x87_m32(e.mem.u32(at(0xc + 4 * k))),
                    );
                    e.mem.set_u32(velocity, x87_store32(sum));
                }
            }
            i = i.wrapping_add(1);
            if i >= e.get(data, NiPSysData::m_usActive) {
                break;
            }
        }
    });
}

// Translated from 00c2ac50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGravityFieldModifier::CreateClone` (Xbox PDB): a 0x48-byte object
/// through the constructor (`00c2a9e0`; 0 when the allocation fails), the
/// `NiPSysFieldModifier` member copy into it (`00c32020`, which still runs
/// for a null clone), the direction copied, the unit direction set to it
/// and normalized (`004a0c10`). Returns the clone.
pub fn ni_p_sys_gravity_field_modifier_create_clone(
    e: &mut Engine,
    this: Ptr<NiPSysGravityFieldModifier>,
    process: Ptr,
) -> Ptr<NiPSysGravityFieldModifier> {
    let block = e.call(NI_NEW, &args![0x48u32]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        fn_00c2a9e0(e, Ptr::new(block))
    };
    e.call(FIELD_MODIFIER_COPY_MEMBERS, &args![this, clone, process]);
    let off = NiPSysGravityFieldModifier::m_kDirection.off;
    for i in 0..3 {
        let word = e.mem.u32(this.addr() + off + 4 * i);
        e.mem.set_u32(clone.addr().wrapping_add(off + 4 * i), word);
    }
    let unit = clone
        .addr()
        .wrapping_add(NiPSysGravityFieldModifier::m_kUnitDirection.off);
    for i in 0..3 {
        let word = e.mem.u32(this.addr() + off + 4 * i);
        e.mem.set_u32(unit.wrapping_add(4 * i), word);
    }
    e.call(POINT_UNITIZE, &args![unit]);
    clone
}

// Translated from 00c2acb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGravityFieldModifier::Update` (Xbox PDB). Nothing happens with a
/// magnitude of exactly 0, no active particles or no field object. Without
/// the maximum distance and with an attenuation of exactly 0 it runs
/// `UpdateNoAttenuationNoDist` (`00c2aac0`). Otherwise the field object's
/// transform relative to the particle system (as there) and the unit
/// direction rotated by its rotation (`004b4500`); for each active particle
/// whose time since its last update (rounded to a float) is not 0: the
/// squared distance of its position from the field's translation; with the
/// maximum distance used, a particle beyond it (`m_fMaxDistanceSqr` below
/// the squared distance, or a NaN) is skipped; with a non-zero attenuation
/// and distance the step is `time * magnitude / max(1, pow(d2, attenuation
/// * 0.5))`, else `time * magnitude`; the rotated direction times the step
/// is added to the velocity. x87 operations one by one in the exe's order;
/// the stack slots are the exe's.
pub fn ni_p_sys_gravity_field_modifier_update(
    e: &mut Engine,
    this: Ptr<NiPSysGravityFieldModifier>,
    time: f32,
    data: Ptr<NiPSysData>,
) {
    let modifier = this.cast::<NiPSysModifier>();
    let field = this.cast::<NiPSysFieldModifier>();
    // `FLDZ; FCOM magnitude; TEST AH,0x44; JNP`: stops only when equal.
    if 0.0 == x87_m32(e.get(field, NiPSysFieldModifier::m_fMagnitude)) {
        return;
    }
    if e.get(data, NiPSysData::m_usActive) == 0 {
        return;
    }
    let object = e.get(field, NiPSysFieldModifier::m_pkFieldObj);
    if object == 0 {
        return;
    }
    let use_max_distance = e.get(field, NiPSysFieldModifier::m_bUseMaxDistance) != 0;
    if !use_max_distance && 0.0 == x87_m32(e.get(field, NiPSysFieldModifier::m_fAttenuation)) {
        ni_p_sys_gravity_field_modifier_update_no_attenuation_no_dist(e, this, time, data);
        return;
    }
    e.with_stack(0x108, |e, frame| {
        let at = |x: u32| frame.addr() + x;
        let system = e.get(modifier, NiPSysModifier::m_pkTarget);
        for i in 0..13 {
            let word = e.mem.u32(object.wrapping_add(WORLD_TRANSFORM + 4 * i));
            e.mem.set_u32(at(0xa0 + 4 * i), word);
        }
        for i in 0..13 {
            let word = e.mem.u32(system.wrapping_add(WORLD_TRANSFORM + 4 * i));
            e.mem.set_u32(at(0x6c + 4 * i), word);
        }
        e.call(TRANSFORM_INVERT, &args![at(0x6c), at(0xd4)]);
        e.call(TRANSFORM_PRODUCT, &args![at(0xd4), at(0x6c), at(0xa0)]);
        let half: f64 = e.global(HALF);
        let attenuation = x87_load32(e.get(field, NiPSysFieldModifier::m_fAttenuation));
        e.mem
            .set_u32(at(0x50), x87_store32(x87_mul(attenuation, half)));
        let unit = this.addr() + NiPSysGravityFieldModifier::m_kUnitDirection.off;
        e.call(MATRIX_TIMES_POINT, &args![at(0x6c), at(0x20), unit]);
        let mut i: u16 = 0;
        e.mem.set_u32(at(0x1c), 0);
        if i >= e.get(data, NiPSysData::m_usActive) {
            return;
        }
        let local = |e: &Engine, x: u32| x87_load32(e.mem.u32(at(x)));
        let (dz, dy, dx) = (local(e, 0x28), local(e, 0x24), local(e, 0x20));
        loop {
            let k = u32::from(i);
            let info = e
                .get(data, NiPSysData::m_pkParticleInfo)
                .wrapping_add(k * INFO_SIZE);
            let now = x87_load32(time.to_bits());
            let last = e.mem.u32(info.wrapping_add(INFO_LAST_UPDATE));
            e.mem
                .set_u32(at(0xc), x87_store32(x87_sub(now, x87_m32(last))));
            let elapsed = local(e, 0xc);
            // `FUCOM` with 0; `TEST AH,0x44; JNP`: skipped only when equal.
            if elapsed != 0.0 {
                let position = e.get(data, NiPSysData::m_pkVertex).wrapping_add(k * 12);
                for c in 0..3 {
                    let word = e.mem.u32(position.wrapping_add(4 * c));
                    e.mem.set_u32(at(0x2c + 4 * c), word);
                }
                for c in 0..3 {
                    let offset =
                        x87_sub(local(e, 0x2c + 4 * c), x87_m32(e.mem.u32(at(0x90 + 4 * c))));
                    e.mem.set_u32(at(0x54 + 4 * c), x87_store32(offset));
                }
                let (oy, ox, oz) = (local(e, 0x58), local(e, 0x54), local(e, 0x5c));
                let xx = x87_mul(ox, ox);
                let yy = x87_mul(oy, oy);
                let sum = x87_add(yy, xx);
                let zz = x87_mul(oz, oz);
                e.mem.set_u32(at(0xc), x87_store32(x87_add(sum, zz)));
                let squared = local(e, 0xc);
                // `FLD max²; FCOMP d²; TEST AH,1; JNZ`: skipped when below
                // (or unordered).
                let beyond = use_max_distance
                    && !(x87_load32(e.get(field, NiPSysFieldModifier::m_fMaxDistanceSqr))
                        >= squared);
                if !beyond {
                    let magnitude = x87_m32(e.get(field, NiPSysFieldModifier::m_fMagnitude));
                    let attenuated = 0.0
                        != x87_m32(e.get(field, NiPSysFieldModifier::m_fAttenuation))
                        && 0.0 != squared;
                    let (vx, vy, vz) = if attenuated {
                        let scaled = x87_mul(elapsed, magnitude);
                        let exponent = local(e, 0x50);
                        let power = e.call(POW, &args![squared, exponent]).f64();
                        e.mem.set_u32(at(0xc), x87_store32(power));
                        let power = local(e, 0xc);
                        let bigger = e
                            .call(FLOAT_MAX, &args![1.0f32.to_bits(), x87_store32(power)])
                            .f64();
                        e.mem.set_u32(at(0xc), x87_store32(x87_div(scaled, bigger)));
                        let step = local(e, 0xc);
                        let vx = x87_store32(x87_mul(step, local(e, 0x20)));
                        e.mem.set_u32(at(0x38), vx);
                        let vy = x87_store32(x87_mul(local(e, 0x24), step));
                        e.mem.set_u32(at(0x3c), vy);
                        let vz = x87_store32(x87_mul(step, local(e, 0x28)));
                        e.mem.set_u32(at(0x40), vz);
                        (vx, vy, vz)
                    } else {
                        e.mem
                            .set_u32(at(0xc), x87_store32(x87_mul(elapsed, magnitude)));
                        let step = local(e, 0xc);
                        let vx = x87_store32(x87_mul(dx, step));
                        e.mem.set_u32(at(0x44), vx);
                        let vy = x87_store32(x87_mul(dy, step));
                        e.mem.set_u32(at(0x48), vy);
                        let vz = x87_store32(x87_mul(step, dz));
                        e.mem.set_u32(at(0x4c), vz);
                        (vx, vy, vz)
                    };
                    e.mem.set_u32(at(0x10), vx);
                    e.mem.set_u32(at(0x14), vy);
                    e.mem.set_u32(at(0x18), vz);
                    let x = x87_add(x87_load32(e.mem.u32(info)), x87_m32(vx));
                    e.mem.set_u32(info, x87_store32(x));
                    let y = x87_add(x87_load32(e.mem.u32(info.wrapping_add(4))), x87_m32(vy));
                    e.mem.set_u32(info.wrapping_add(4), x87_store32(y));
                    let z = x87_add(x87_load32(vz), x87_m32(e.mem.u32(info.wrapping_add(8))));
                    e.mem.set_u32(info.wrapping_add(8), x87_store32(z));
                }
            }
            i = i.wrapping_add(1);
            e.mem.set_u32(at(0x1c), u32::from(i));
            if i >= e.get(data, NiPSysData::m_usActive) {
                break;
            }
        }
    });
}

// Translated from 00c2af80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysGravityFieldModifier::GetViewerStrings` (Xbox PDB):
/// `NiPSysFieldModifier`'s `GetViewerStrings`, then the class name
/// (`NiRTTI` `01202708`) and the direction's string with `"Direction"`
/// (`NiPoint3::GetViewerString`) appended.
pub fn ni_p_sys_gravity_field_modifier_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiPSysGravityFieldModifier>,
    strings: Ptr,
) {
    e.call(FIELD_MODIFIER_GET_VIEWER_STRINGS, &args![this, strings]);
    add_class_name(e, strings.addr(), RTTI);
    let direction = this.addr() + NiPSysGravityFieldModifier::m_kDirection.off;
    let text = e
        .call(POINT_GET_VIEWER_STRING, &args![direction, LABEL_DIRECTION])
        .u32();
    add_viewer_string(e, strings.addr(), text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c2_67f0,
            ni_p_sys_gravity_field_modifier_save_binary(Ptr<NiPSysGravityFieldModifier>, Ptr)
        ),
        entry!(
            0x00c2_6db0,
            ni_p_sys_gravity_field_modifier_is_equal(
                Ptr<NiPSysGravityFieldModifier>,
                Ptr<NiPSysGravityFieldModifier>,
            ) -> bool
        ),
        entry!(
            0x00c2_a9e0,
            fn_00c2a9e0(Ptr<NiPSysGravityFieldModifier>) -> Ptr<NiPSysGravityFieldModifier>
        ),
        entry!(0x00c2_aa50, fn_00c2aa50() -> u32),
        entry!(
            0x00c2_aa70,
            ni_p_sys_gravity_field_modifier_load_binary(Ptr<NiPSysGravityFieldModifier>, Ptr)
        ),
        entry!(
            0x00c2_aac0,
            ni_p_sys_gravity_field_modifier_update_no_attenuation_no_dist(
                Ptr<NiPSysGravityFieldModifier>,
                f32,
                Ptr<NiPSysData>,
            )
        ),
        entry!(
            0x00c2_ac50,
            ni_p_sys_gravity_field_modifier_create_clone(
                Ptr<NiPSysGravityFieldModifier>,
                Ptr,
            )
                -> Ptr<NiPSysGravityFieldModifier>
        ),
        entry!(
            0x00c2_acb0,
            ni_p_sys_gravity_field_modifier_update(
                Ptr<NiPSysGravityFieldModifier>,
                f32,
                Ptr<NiPSysData>,
            )
        ),
        entry!(
            0x00c2_af80,
            ni_p_sys_gravity_field_modifier_get_viewer_strings(
                Ptr<NiPSysGravityFieldModifier>,
                Ptr,
            )
        ),
        entry!(0x00c2a9b0, ni_p_sys_gravity_field_modifier_get_r_t_t_i() -> u32),
        entry!(0x00c2a9c0, ni_p_sys_gravity_field_modifier_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_gravity_field_modifier_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00c3_1750, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00c2a9c0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c2a9c0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x48]]);
        assert_eq!(calls_to(&e, 0x00c3_1750), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_p_sys_gravity_field_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_a9b0, &args![]).u32(), 0x0120_2708);
    }
}

#[cfg(test)]
mod gravity_tests {
    use super::*;
    use crate::units::niparticle::nipsysmodifier::test_stream;
    use crate::units::niparticle::nipsysmodifier::test_stream::calls_to;

    fn f(v: f32) -> u32 {
        v.to_bits()
    }

    #[test]
    fn save_binary_saves_the_direction_after_the_base() {
        let mut e = Engine::new();
        e.register(FIELD_MODIFIER_SAVE_BINARY, |_, _| Ret::default());
        e.register(POINT_SAVE_BINARY, |_, _| Ret::default());
        let this = e.new_object::<NiPSysGravityFieldModifier>();
        e.call_log = Some(vec![]);
        e.call(0x00c2_67f0, &args![this, 0x7000u32]);
        assert_eq!(
            calls_to(&e, FIELD_MODIFIER_SAVE_BINARY),
            vec![vec![this.addr(), 0x7000]]
        );
        assert_eq!(
            calls_to(&e, POINT_SAVE_BINARY),
            vec![vec![this.addr() + 0x30, 0x7000]]
        );
    }

    #[test]
    fn is_equal_needs_the_same_direction() {
        let mut e = Engine::new();
        e.register(FIELD_MODIFIER_IS_EQUAL, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(POINT_NOT_EQUAL, |_, _| Ret {
            eax: 0x100,
            ..Ret::default()
        });
        let a = e.new_object::<NiPSysGravityFieldModifier>();
        let b = e.new_object::<NiPSysGravityFieldModifier>();
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c2_6db0, &args![a, b]).bool());
        assert_eq!(
            calls_to(&e, POINT_NOT_EQUAL),
            vec![vec![b.addr() + 0x30, a.addr() + 0x30]]
        );
        e.register(POINT_NOT_EQUAL, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        assert!(!e.call(0x00c2_6db0, &args![a, b]).bool());
        e.register(FIELD_MODIFIER_IS_EQUAL, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c2_6db0, &args![a, b]).bool());
        assert!(calls_to(&e, POINT_NOT_EQUAL).is_empty());
    }

    fn construct_doubles(e: &mut Engine) {
        e.map(UNIT_Y & !0xfff, 0x1000);
        e.mem.set_u32(UNIT_Y, 0);
        e.mem.set_u32(UNIT_Y + 4, f(1.0));
        e.mem.set_u32(UNIT_Y + 8, 0);
        e.register(FIELD_MODIFIER_CONSTRUCTOR, |_, _| Ret::default());
        e.register(POINT_UNITIZE, |_, _| Ret::default());
    }

    #[test]
    fn constructor_points_the_field_down_y() {
        let mut e = Engine::new();
        construct_doubles(&mut e);
        let this = e.new_object::<NiPSysGravityFieldModifier>();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_a9e0, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        for base in [0x30, 0x3c] {
            assert_eq!(e.mem.u32(this.addr() + base), 0x8000_0000);
            assert_eq!(e.mem.u32(this.addr() + base + 4), f(-1.0));
            assert_eq!(e.mem.u32(this.addr() + base + 8), 0x8000_0000);
        }
        assert_eq!(
            calls_to(&e, FIELD_MODIFIER_CONSTRUCTOR),
            vec![vec![this.addr()]]
        );
        assert_eq!(calls_to(&e, POINT_UNITIZE), vec![vec![this.addr() + 0x3c]]);
    }

    #[test]
    fn factory_constructs_or_answers_null() {
        let mut e = Engine::new();
        construct_doubles(&mut e);
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let made = e.call(0x00c2_aa50, &args![]).u32();
        assert_ne!(made, 0);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x48]]);
        assert_eq!(e.mem.u32(made), VTABLE);
        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_aa50, &args![]).u32(), 0);
        assert!(calls_to(&e, FIELD_MODIFIER_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn load_binary_reads_the_direction_and_unitizes_a_copy() {
        let mut e = Engine::new();
        e.register(FIELD_MODIFIER_LOAD_BINARY, |_, _| Ret::default());
        e.register(POINT_LOAD_BINARY, |e, a| {
            for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
                e.mem.set_u32(a[0] + 4 * i as u32, v.to_bits());
            }
            Ret::default()
        });
        e.register(POINT_UNITIZE, |_, _| Ret::default());
        let this = e.new_object::<NiPSysGravityFieldModifier>();
        e.call_log = Some(vec![]);
        e.call(0x00c2_aa70, &args![this, 0x7000u32]);
        assert_eq!(
            calls_to(&e, FIELD_MODIFIER_LOAD_BINARY),
            vec![vec![this.addr(), 0x7000]]
        );
        assert_eq!(
            calls_to(&e, POINT_LOAD_BINARY),
            vec![vec![this.addr() + 0x30, 0x7000]]
        );
        assert_eq!(e.mem.u32(this.addr() + 0x40), f(2.0));
        assert_eq!(e.mem.u32(this.addr() + 0x44), f(3.0));
        assert_eq!(calls_to(&e, POINT_UNITIZE), vec![vec![this.addr() + 0x3c]]);
    }

    #[test]
    fn create_clone_copies_the_direction() {
        let mut e = Engine::new();
        construct_doubles(&mut e);
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(FIELD_MODIFIER_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.new_object::<NiPSysGravityFieldModifier>();
        e.mem.set_u32(this.addr() + 0x30, f(4.0));
        e.mem.set_u32(this.addr() + 0x34, f(5.0));
        e.mem.set_u32(this.addr() + 0x38, f(6.0));
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c2_ac50, &args![this, 0x6000u32]).u32();
        assert_eq!(e.mem.u32(clone), VTABLE);
        assert_eq!(
            calls_to(&e, FIELD_MODIFIER_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x6000]]
        );
        for base in [0x30, 0x3c] {
            assert_eq!(e.mem.u32(clone + base), f(4.0));
            assert_eq!(e.mem.u32(clone + base + 8), f(6.0));
        }
        assert_eq!(
            calls_to(&e, POINT_UNITIZE).last(),
            Some(&vec![clone + 0x3c])
        );
    }

    /// A gravity field with an identity relative transform translated by
    /// `(0, 0, 0)`, a system of two particles at `(2, 0, 0)` with zero
    /// velocity, the first last updated at 1.0, the second at 1.5.
    fn field(e: &mut Engine) -> (Ptr<NiPSysGravityFieldModifier>, Ptr<NiPSysData>) {
        e.register(TRANSFORM_INVERT, |_, _| Ret::default());
        e.register(TRANSFORM_PRODUCT, |e, a| {
            for i in 0..13 {
                e.mem.set_u32(a[1] + 4 * i, 0);
            }
            for i in [0, 4, 8, 12] {
                e.mem.set_u32(a[1] + 4 * i, 1.0f32.to_bits());
            }
            Ret::default()
        });
        e.register(POINT_UNITIZE, |_, _| Ret::default());
        e.register(MATRIX_TIMES_POINT, |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[2] + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, word);
            }
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
        let this = e.new_object::<NiPSysGravityFieldModifier>();
        e.mem.set_u32(this.addr() + 0x10, system);
        e.mem.set_u32(this.addr() + 0x18, object);
        e.mem.set_u32(this.addr() + 0x1c, f(3.0));
        e.mem.set_u32(this.addr() + 0x34, f(-2.0));
        e.mem.set_u32(this.addr() + 0x40, f(-1.0));
        let data = e.new_object::<NiPSysData>();
        let infos = e.mem.alloc(2 * INFO_SIZE);
        e.mem.set_u32(infos + INFO_LAST_UPDATE, f(1.0));
        e.mem.set_u32(infos + INFO_SIZE + INFO_LAST_UPDATE, f(1.5));
        let vertices = e.mem.alloc(24);
        e.mem.set_u32(vertices, f(2.0));
        e.mem.set_u32(vertices + 12, f(2.0));
        e.set(data, NiPSysData::m_pkParticleInfo, infos);
        e.set(data, NiPSysData::m_pkVertex, vertices);
        e.set(data, NiPSysData::m_usActive, 2);
        (this, data)
    }

    fn velocity(e: &Engine, data: Ptr<NiPSysData>, i: u32) -> [u32; 3] {
        let info = e.get(data, NiPSysData::m_pkParticleInfo) + i * INFO_SIZE;
        [e.mem.u32(info), e.mem.u32(info + 4), e.mem.u32(info + 8)]
    }

    #[test]
    fn update_no_attenuation_no_dist_pulls_along_the_rotated_direction() {
        let mut e = Engine::new();
        let (this, data) = field(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c2_aac0, &args![this, 1.5f32, data]);
        let invert = calls_to(&e, TRANSFORM_INVERT);
        let product = calls_to(&e, TRANSFORM_PRODUCT);
        assert_eq!(invert[0][1], product[0][0]);
        assert_eq!(invert[0][0], product[0][1]);
        assert_eq!(calls_to(&e, POINT_UNITIZE).len(), 1);
        // (0, -2, 0) * magnitude 3 * time 0.5.
        assert_eq!(velocity(&e, data, 0), [0, f(-3.0), 0]);
        // No time since the last update: unchanged.
        assert_eq!(velocity(&e, data, 1), [0, 0, 0]);
    }

    #[test]
    fn update_skips_without_magnitude_particles_or_field_object() {
        let mut e = Engine::new();
        let (this, data) = field(&mut e);
        e.mem.set_u32(this.addr() + 0x1c, 0x8000_0000);
        e.call_log = Some(vec![]);
        e.call(0x00c2_acb0, &args![this, 1.5f32, data]);
        assert!(calls_to(&e, TRANSFORM_INVERT).is_empty());
        e.mem.set_u32(this.addr() + 0x1c, f(3.0));
        e.mem.set_u32(this.addr() + 0x18, 0);
        e.call(0x00c2_acb0, &args![this, 1.5f32, data]);
        assert!(calls_to(&e, TRANSFORM_INVERT).is_empty());
        e.mem.set_u32(this.addr() + 0x18, 0x9000);
        e.set(data, NiPSysData::m_usActive, 0);
        e.call(0x00c2_acb0, &args![this, 1.5f32, data]);
        assert!(calls_to(&e, TRANSFORM_INVERT).is_empty());
    }

    #[test]
    fn update_without_attenuation_or_distance_runs_the_simple_update() {
        let mut e = Engine::new();
        let (this, data) = field(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c2_acb0, &args![this, 1.5f32, data]);
        // The direction (+0x30), not the unit direction, is used.
        assert!(calls_to(&e, MATRIX_TIMES_POINT).is_empty());
        assert_eq!(velocity(&e, data, 0), [0, f(-3.0), 0]);
    }

    #[test]
    fn update_with_attenuation_divides_by_the_distance_power() {
        let mut e = Engine::new();
        let (this, data) = field(&mut e);
        // Attenuation 2: the step is divided by max(1, d² ^ 1) = 4.
        e.mem.set_u32(this.addr() + 0x20, f(2.0));
        e.call_log = Some(vec![]);
        e.call(0x00c2_acb0, &args![this, 1.5f32, data]);
        assert_eq!(calls_to(&e, MATRIX_TIMES_POINT)[0][2], this.addr() + 0x3c);
        let pow = calls_to(&e, POW);
        assert_eq!(pow, vec![args![4.0f64, 1.0f64]]);
        assert_eq!(calls_to(&e, FLOAT_MAX), vec![vec![f(1.0), f(4.0)]]);
        // Unit direction (0, -1, 0) * 0.5 * 3 / 4.
        assert_eq!(velocity(&e, data, 0), [0, f(-0.375), 0]);
        assert_eq!(velocity(&e, data, 1), [0, 0, 0]);
    }

    #[test]
    fn update_with_max_distance_skips_far_particles() {
        let mut e = Engine::new();
        let (this, data) = field(&mut e);
        e.mem.set_u8(this.addr() + 0x24, 1);
        e.mem.set_u32(this.addr() + 0x2c, f(1.0));
        e.call_log = Some(vec![]);
        e.call(0x00c2_acb0, &args![this, 1.5f32, data]);
        assert_eq!(calls_to(&e, TRANSFORM_INVERT).len(), 1);
        assert_eq!(velocity(&e, data, 0), [0, 0, 0]);
        // Within the distance, no attenuation: the unit direction scaled by
        // time and magnitude.
        e.mem.set_u32(this.addr() + 0x2c, f(4.0));
        e.call(0x00c2_acb0, &args![this, 1.5f32, data]);
        assert_eq!(velocity(&e, data, 0), [0, f(-1.5), 0]);
        assert!(calls_to(&e, POW).is_empty());
    }

    #[test]
    fn get_viewer_strings_appends_class_and_direction() {
        let mut e = Engine::new();
        test_stream::global_word(&mut e, RTTI, 0x0abc_0008);
        e.register(FIELD_MODIFIER_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(POINT_GET_VIEWER_STRING, |_, _| Ret {
            eax: 0x4444,
            ..Ret::default()
        });
        let stored = test_stream::viewer_strings(&mut e);
        let strings = test_stream::strings(&mut e);
        let this = e.new_object::<NiPSysGravityFieldModifier>();
        e.call_log = Some(vec![]);
        e.call(0x00c2_af80, &args![this, strings]);
        assert_eq!(
            calls_to(&e, FIELD_MODIFIER_GET_VIEWER_STRINGS),
            vec![vec![this.addr(), strings]]
        );
        assert_eq!(
            calls_to(&e, POINT_GET_VIEWER_STRING),
            vec![vec![this.addr() + 0x30, LABEL_DIRECTION]]
        );
        assert_eq!(*stored.borrow(), vec![0x0abc_0008, 0x4444]);
    }
}
