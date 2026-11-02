//! `NiParticle/nipsysdragfieldmodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c2c810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysDragFieldModifier::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00c31750`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x40 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_p_sys_drag_field_modifier_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00c3_1750;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x40u32]);
    }
    this
}

// Translated from 00c2c800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysDragFieldModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2770.
pub fn ni_p_sys_drag_field_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2770
}

use crate::units::niparticle::nipsysdata::{add_viewer_string, stream_write};
use crate::units::niparticle::nipsysmodifier::stream_read;

layout! {
    /// `NiPSysDragFieldModifier` (Xbox PDB), 0x40 bytes; the PC code uses
    /// the same offsets. Its own fields; those of `NiPSysFieldModifier`
    /// (Xbox PDB) are read at their offsets below.
    pub struct NiPSysDragFieldModifier: 0x40 {
        /// `m_bUseDirection` (Xbox PDB).
        0x30 m_bUseDirection: u8,
        /// `m_kDirection` (Xbox PDB): an `NiPoint3`.
        0x34 m_kDirection: u32,
    }
}

/// `NiPSysModifier::m_pkTarget` (+0x10) and the `NiPSysFieldModifier`
/// fields (Xbox PDB names, PC offsets): `m_pkFieldObj` (+0x18),
/// `m_fMagnitude` (+0x1C), `m_fAttenuation` (+0x20), `m_bUseMaxDistance`
/// (+0x24), `m_fMaxDistance` (+0x28) and `m_fMaxDistanceSqr` (+0x2C).
const TARGET: u32 = 0x10;
const FIELD_OBJECT: u32 = 0x18;
const MAGNITUDE: u32 = 0x1c;
const ATTENUATION: u32 = 0x20;
const USE_MAX_DISTANCE: u32 = 0x24;
const MAX_DISTANCE: u32 = 0x28;
const MAX_DISTANCE_SQUARED: u32 = 0x2c;
/// `NiPoint3::ZERO`'s three words (`011f426c`), which a stopped particle's
/// velocity becomes.
const ZERO_POINT: u32 = 0x011f_426c;
/// The `double` 1.0 (`01012070`) the attenuation divisor adds.
const ONE: u32 = 0x0101_2070;
/// The particle data (`NiPSysData`) fields the updates read: the active
/// count (+0x50), the positions (+0x20, 12 bytes each) and the particle
/// infos (+0x70, 0x1c bytes each: velocity at +0, last update at +0x14).
const DATA_ACTIVE: u32 = 0x50;
const DATA_VERTICES: u32 = 0x20;
const DATA_INFOS: u32 = 0x70;
const INFO_SIZE: u32 = 0x1c;
const INFO_LAST_UPDATE: u32 = 0x14;
/// `NiAVObject`'s world transform (`m_kWorld`, +0x68 as the PC code reads
/// it, 0x34 bytes; its translation at +0x24).
const WORLD_TRANSFORM: u32 = 0x68;
const TRANSFORM_SIZE: u32 = 0x34;
const TRANSLATION: u32 = 0x24;
/// `NiTransform::InvertNonUniform` (`004b4880`, Xbox PDB name; `thiscall`
/// on a transform, the result), `0062c250` (unnamed; `thiscall` on a
/// transform: result, right-hand transform; the product), `004b4500`
/// (unnamed; `thiscall` on a transform: result, point; returns the result:
/// the point turned by the transform), `004a0c10` (unnamed; `thiscall` on a
/// point, no words; it normalizes the point, by its use here) and
/// `00439090` (unnamed; `thiscall` on a point, the other point; nonzero
/// when they differ, by its use here).
const TRANSFORM_INVERT: u32 = 0x004b_4880;
const TRANSFORM_MULTIPLY: u32 = 0x0062_c250;
const TRANSFORM_POINT: u32 = 0x004b_4500;
const POINT_UNITIZE: u32 = 0x004a_0c10;
const POINT_UNEQUAL: u32 = 0x0043_9090;
/// The CRT square root (`00ec6040`: the `ST0` value as a leading `f64`,
/// the root back in `ST0`).
const SQRT: u32 = 0x00ec_6040;
/// `00aa13e0` (`cdecl`, one word): the object allocation; the
/// `NiPSysFieldModifier` constructor (`00c31ff0`, `thiscall`) and member
/// copy (`00c32020`, `thiscall`: clone, process); the vtable the clone gets
/// (`010bff2c`).
const NI_NEW: u32 = 0x00aa_13e0;
const FIELD_MODIFIER_CONSTRUCTOR: u32 = 0x00c3_1ff0;
const FIELD_MODIFIER_COPY_MEMBERS: u32 = 0x00c3_2020;
const CLONE_VTABLE: u32 = 0x010b_ff2c;
/// `NiPSysFieldModifier`'s `IsEqual` (`00c32060`), `LoadBinary`
/// (`00c32120`), `SaveBinary` (`00c321f0`) and `GetViewerStrings`
/// (`00c322b0`) (Xbox PDB names; `thiscall`, one word).
const FIELD_MODIFIER_IS_EQUAL: u32 = 0x00c3_2060;
const FIELD_MODIFIER_LOAD_BINARY: u32 = 0x00c3_2120;
const FIELD_MODIFIER_SAVE_BINARY: u32 = 0x00c3_21f0;
const FIELD_MODIFIER_GET_VIEWER_STRINGS: u32 = 0x00c3_22b0;
/// `NiColor::LoadBinary` (`00a59be0`) and `SaveBinary` (`00a69780`), and
/// `NiPoint3::GetViewerString` (`00a59af0`, a label) (Xbox PDB names;
/// `thiscall` on the direction).
const POINT_LOAD_BINARY: u32 = 0x00a5_9be0;
const POINT_SAVE_BINARY: u32 = 0x00a6_9780;
const POINT_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
/// `NiGetViewerString` (`00a7d330`, the text) and `_ov2` (`00a7d370`: a
/// label and a `bool`) (Xbox PDB names; `cdecl`).
const VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
/// `NiPSysDragFieldModifier::ms_RTTI` (its name is the first word).
const RTTI: u32 = 0x0120_2770;
/// The labels "Use Direction" (`010bffd0`) and "Direction" (`01044b44`).
const LABEL_USE_DIRECTION: u32 = 0x010b_ffd0;
const LABEL_DIRECTION: u32 = 0x0104_4b44;

/// The three words of the point at `addr`.
fn point(e: &Engine, addr: u32) -> [u32; 3] {
    [e.mem.u32(addr), e.mem.u32(addr + 4), e.mem.u32(addr + 8)]
}

/// The velocity at `velocity` set to `NiPoint3::ZERO` (three `MOV`s).
fn stop(e: &mut Engine, velocity: u32) {
    for word in 0..3 {
        let value = e.global::<u32>(ZERO_POINT + 4 * word);
        e.mem.set_u32(velocity + 4 * word, value);
    }
}

/// The velocity at `velocity` times `scale` (a value on the x87 stack):
/// `FLD x; FMUL ST1`, `FLD y; FMUL ST1`, `FMUL z`, each stored.
fn scale_velocity(e: &mut Engine, velocity: u32, scale: f64) {
    let x = x87_mul(x87_load32(e.mem.u32(velocity)), scale);
    e.mem.set_u32(velocity, x87_store32(x));
    let y = x87_mul(x87_load32(e.mem.u32(velocity + 4)), scale);
    e.mem.set_u32(velocity + 4, x87_store32(y));
    let z = x87_mul(scale, x87_m32(e.mem.u32(velocity + 8)));
    e.mem.set_u32(velocity + 8, x87_store32(z));
}

/// The field object's position in the particle system's space: the system's
/// world transform inverted (`004b4880`) times the field object's
/// (`0062c250`), written to `product` (0x34 bytes) through locals in
/// `frame` (0x68 bytes).
fn field_transform(e: &mut Engine, this: u32, frame: u32, product: u32) {
    let field_world = frame;
    let inverse = frame + TRANSFORM_SIZE;
    let field_object = e.mem.u32(this + FIELD_OBJECT);
    let words = e.mem.bytes(field_object + WORLD_TRANSFORM, TRANSFORM_SIZE);
    e.mem.write(field_world, &words);
    let target = e.mem.u32(this + TARGET);
    let words = e.mem.bytes(target + WORLD_TRANSFORM, TRANSFORM_SIZE);
    e.mem.write(product, &words);
    e.call(TRANSFORM_INVERT, &args![product, inverse]);
    e.call(TRANSFORM_MULTIPLY, &args![inverse, product, field_world]);
}

// Translated from 00c2c830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the drag field's update without direction,
/// attenuation or distance limit. For each active particle the drag is
/// `(time - last update) * magnitude`; at 1.0 or more (or a NaN) the
/// velocity becomes `NiPoint3::ZERO`, otherwise it is scaled by
/// `1 - drag`.
pub fn fn_00c2c830(e: &mut Engine, this: Ptr<NiPSysDragFieldModifier>, time: f32, data: Ptr) {
    let data = data.addr();
    let time = x87_load32(time.to_bits());
    let mut index: u16 = 0;
    while index < e.mem.u16(data + DATA_ACTIVE) {
        let info = e
            .mem
            .u32(data + DATA_INFOS)
            .wrapping_add(u32::from(index) * INFO_SIZE);
        let elapsed = x87_sub(time, x87_m32(e.mem.u32(info + INFO_LAST_UPDATE)));
        let magnitude = x87_m32(e.mem.u32(this.addr() + MAGNITUDE));
        let drag = x87_load32(x87_store32(x87_mul(elapsed, magnitude)));
        if drag < 1.0 {
            let scale = x87_load32(x87_store32(x87_sub(1.0, drag)));
            scale_velocity(e, info, scale);
        } else {
            stop(e, info);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 00c2c8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysDragFieldModifier::CreateClone` (Xbox PDB): a 0x40-byte object
/// from `00aa13e0` (null when the allocation fails) through the
/// `NiPSysFieldModifier` constructor, this class's vtable, no direction use
/// and a direction of `NiPoint3::ZERO`; the field modifier's member copy
/// into it, then the direction flag and the direction (by `MOV`). Returns
/// the clone.
pub fn ni_p_sys_drag_field_modifier_create_clone(
    e: &mut Engine,
    this: Ptr<NiPSysDragFieldModifier>,
    process: Ptr,
) -> Ptr<NiPSysDragFieldModifier> {
    type M = NiPSysDragFieldModifier;
    let block = e.call(NI_NEW, &args![0x40u32]).u32();
    let clone = Ptr::<M>::new(block);
    if block != 0 {
        e.call(FIELD_MODIFIER_CONSTRUCTOR, &args![block]);
        e.mem.set_u32(block, CLONE_VTABLE);
        e.set(clone, M::m_bUseDirection, 0);
        let zero = [
            e.global::<u32>(ZERO_POINT),
            e.global::<u32>(ZERO_POINT + 4),
            e.global::<u32>(ZERO_POINT + 8),
        ];
        for (i, word) in zero.into_iter().enumerate() {
            e.mem
                .set_u32(block + M::m_kDirection.off + 4 * i as u32, word);
        }
    }
    e.call(FIELD_MODIFIER_COPY_MEMBERS, &args![this, clone, process]);
    let use_direction = e.get(this, M::m_bUseDirection);
    e.set(clone, M::m_bUseDirection, use_direction);
    let direction = point(e, this.addr() + M::m_kDirection.off);
    for (i, word) in direction.into_iter().enumerate() {
        e.mem
            .set_u32(clone.addr() + M::m_kDirection.off + 4 * i as u32, word);
    }
    clone
}

// Translated from 00c2c950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysDragFieldModifier::UpdateNoAttenuationNoDir` (engine map name):
/// the update with a distance limit but no direction or attenuation. The
/// field object's position in the system's space (the translation of the
/// inverse of the system's world transform times the field object's); for
/// each active particle the drag is `(time - last update) * magnitude`;
/// a drag of 0 leaves the particle alone, as does a squared distance from
/// the field object above `m_fMaxDistanceSqr` (or a NaN); otherwise, at 1.0
/// or more (or a NaN) the velocity becomes `NiPoint3::ZERO`, else it is
/// scaled by `1 - drag`.
pub fn ni_p_sys_drag_field_modifier_update_no_attenuation_no_dir(
    e: &mut Engine,
    this: Ptr<NiPSysDragFieldModifier>,
    time: f32,
    data: Ptr,
) {
    let data = data.addr();
    e.with_stack(0xa0, |e, frame| {
        let product = frame.addr() + 0x68;
        field_transform(e, this.addr(), frame.addr(), product);
        let origin = point(e, product + TRANSLATION).map(x87_load32);
        let mut index: u16 = 0;
        while index < e.mem.u16(data + DATA_ACTIVE) {
            let info = e
                .mem
                .u32(data + DATA_INFOS)
                .wrapping_add(u32::from(index) * INFO_SIZE);
            let elapsed = x87_sub(
                x87_load32(time.to_bits()),
                x87_m32(e.mem.u32(info + INFO_LAST_UPDATE)),
            );
            let magnitude = x87_m32(e.mem.u32(this.addr() + MAGNITUDE));
            let drag = x87_load32(x87_store32(x87_mul(elapsed, magnitude)));
            index = index.wrapping_add(1);
            if drag == 0.0 {
                continue;
            }
            let vertex = e
                .mem
                .u32(data + DATA_VERTICES)
                .wrapping_add(u32::from(index - 1) * 12);
            let position = point(e, vertex);
            let offset: [f64; 3] = std::array::from_fn(|i| {
                x87_load32(x87_store32(x87_sub(x87_load32(position[i]), origin[i])))
            });
            let yy = x87_mul(offset[1], offset[1]);
            let xx = x87_mul(offset[0], offset[0]);
            let zz = x87_mul(offset[2], offset[2]);
            let squared = x87_store32(x87_add(zz, x87_add(xx, yy)));
            let limit = x87_load32(e.mem.u32(this.addr() + MAX_DISTANCE_SQUARED));
            if !(limit >= x87_load32(squared)) {
                continue;
            }
            if 1.0 > drag {
                let scale = x87_load32(x87_store32(x87_sub(1.0, drag)));
                scale_velocity(e, info, scale);
            } else {
                stop(e, info);
            }
        }
    });
}

// Translated from 00c2cae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysDragFieldModifier::IsEqual` (Xbox PDB): `NiPSysFieldModifier`'s,
/// the same direction flag, and equal directions (`00439090` on the
/// other's answering 0).
pub fn ni_p_sys_drag_field_modifier_is_equal(
    e: &mut Engine,
    this: Ptr<NiPSysDragFieldModifier>,
    other: Ptr<NiPSysDragFieldModifier>,
) -> bool {
    type M = NiPSysDragFieldModifier;
    if !e.call(FIELD_MODIFIER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    if e.get(other, M::m_bUseDirection) != e.get(this, M::m_bUseDirection) {
        return false;
    }
    let offset = M::m_kDirection.off;
    !e.call(
        POINT_UNEQUAL,
        &args![other.addr() + offset, this.addr() + offset],
    )
    .bool()
}

// Translated from 00c2cb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysDragFieldModifier::Update` (Xbox PDB): nothing for a magnitude of
/// 0, no particles or no field object. Without direction and with an
/// attenuation of 0 the simpler updates run (`00c2c950` with a distance
/// limit, `00c2c830` without). Otherwise the field object's position (and,
/// with a direction other than `NiPoint3::ZERO`, the direction turned into
/// the system's space and normalized) in the system's space; for each
/// active particle with an elapsed time other than 0 and, with a distance
/// limit, within `m_fMaxDistance`: the drag is `elapsed * magnitude *
/// strength / (attenuation * distance + 1.0)`, the strength 1.0 or, with a
/// direction, the dot product of the direction and the normalized
/// velocity; at 1.0 or more (or a NaN) the velocity becomes
/// `NiPoint3::ZERO`, otherwise it loses `velocity * drag`. With a direction
/// equal to `NiPoint3::ZERO` the exe reads a direction it never wrote (an
/// uninitialised local; here the frame's zeroed memory).
pub fn ni_p_sys_drag_field_modifier_update(
    e: &mut Engine,
    this: Ptr<NiPSysDragFieldModifier>,
    time: f32,
    data: Ptr,
) {
    type M = NiPSysDragFieldModifier;
    let magnitude = x87_m32(e.mem.u32(this.addr() + MAGNITUDE));
    if 0.0 == magnitude {
        return;
    }
    if e.mem.u16(data.addr() + DATA_ACTIVE) == 0 || e.mem.u32(this.addr() + FIELD_OBJECT) == 0 {
        return;
    }
    if e.get(this, M::m_bUseDirection) == 0 && 0.0 == x87_m32(e.mem.u32(this.addr() + ATTENUATION))
    {
        let time = f32::from_bits(x87_quiet_bits(time.to_bits()));
        if e.mem.u8(this.addr() + USE_MAX_DISTANCE) != 0 {
            ni_p_sys_drag_field_modifier_update_no_attenuation_no_dir(e, this, time, data);
        } else {
            fn_00c2c830(e, this, time, data);
        }
        return;
    }
    let data = data.addr();
    e.with_stack(0x100, |e, frame| {
        let product = frame.addr() + 0x68;
        let turned = frame.addr() + 0xa0;
        let direction = frame.addr() + 0xb0;
        let velocity = frame.addr() + 0xc0;
        field_transform(e, this.addr(), frame.addr(), product);
        if e.get(this, M::m_bUseDirection) != 0 {
            let own = this.addr() + M::m_kDirection.off;
            if e.call(POINT_UNEQUAL, &args![own, ZERO_POINT]).bool() {
                let result = e.call(TRANSFORM_POINT, &args![product, turned, own]).u32();
                let words = e.mem.bytes(result, 12);
                e.mem.write(direction, &words);
                e.call(POINT_UNITIZE, &args![direction]);
            }
        }
        let origin = point(e, product + TRANSLATION);
        let mut index: u16 = 0;
        while index < e.mem.u16(data + DATA_ACTIVE) {
            let info = e
                .mem
                .u32(data + DATA_INFOS)
                .wrapping_add(u32::from(index) * INFO_SIZE);
            let current = index;
            index = index.wrapping_add(1);
            let elapsed = x87_store32(x87_sub(
                x87_load32(time.to_bits()),
                x87_m32(e.mem.u32(info + INFO_LAST_UPDATE)),
            ));
            if 0.0 == x87_m32(elapsed) {
                continue;
            }
            let vertex = e
                .mem
                .u32(data + DATA_VERTICES)
                .wrapping_add(u32::from(current) * 12);
            let position = point(e, vertex);
            let offset: [f64; 3] = std::array::from_fn(|i| {
                x87_load32(x87_store32(x87_sub(
                    x87_load32(position[i]),
                    x87_m32(origin[i]),
                )))
            });
            let xx = x87_mul(offset[0], offset[0]);
            let yy = x87_mul(offset[1], offset[1]);
            let zz = x87_mul(offset[2], offset[2]);
            let squared = x87_store32(x87_add(x87_add(yy, xx), zz));
            let root = e.call(SQRT, &args![x87_load32(squared)]).st0;
            let distance = x87_store32(x87_load32(x87_store32(root)));
            if e.mem.u8(this.addr() + USE_MAX_DISTANCE) != 0 {
                let limit = x87_load32(e.mem.u32(this.addr() + MAX_DISTANCE));
                if !(limit >= x87_load32(distance)) {
                    continue;
                }
            }
            let mut strength = 1.0f32.to_bits();
            if e.get(this, M::m_bUseDirection) != 0 {
                let words = e.mem.bytes(info, 12);
                e.mem.write(velocity, &words);
                e.call(POINT_UNITIZE, &args![velocity]);
                let v = point(e, velocity);
                let d = point(e, direction);
                let yy = x87_mul(x87_load32(v[1]), x87_m32(d[1]));
                let xx = x87_mul(x87_load32(d[0]), x87_m32(v[0]));
                let zz = x87_mul(x87_load32(v[2]), x87_m32(d[2]));
                strength = x87_store32(x87_add(x87_add(yy, xx), zz));
            }
            let drag = x87_mul(
                x87_load32(elapsed),
                x87_m32(e.mem.u32(this.addr() + MAGNITUDE)),
            );
            let drag = x87_mul(drag, x87_m32(strength));
            let attenuation = x87_load32(e.mem.u32(this.addr() + ATTENUATION));
            let divisor = x87_mul(attenuation, x87_m32(distance));
            let divisor = x87_add(divisor, e.global::<f64>(ONE));
            let drag = x87_load32(x87_store32(x87_div(drag, divisor)));
            if drag < 1.0 {
                let change = [
                    x87_store32(x87_mul(x87_load32(e.mem.u32(info)), drag)),
                    x87_store32(x87_mul(x87_load32(e.mem.u32(info + 4)), drag)),
                    x87_store32(x87_mul(drag, x87_m32(e.mem.u32(info + 8)))),
                ];
                for (i, change) in change.into_iter().enumerate() {
                    let slot = info + 4 * i as u32;
                    let value = x87_sub(x87_load32(e.mem.u32(slot)), x87_m32(change));
                    e.mem.set_u32(slot, x87_store32(value));
                }
            } else {
                stop(e, info);
            }
        }
    });
}

// Translated from 00c2ce10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysDragFieldModifier::LoadBinary` (Xbox PDB): `NiPSysFieldModifier`'s,
/// then one byte (the direction flag, stored as 1 or 0) and the direction
/// (`NiColor::LoadBinary`).
pub fn ni_p_sys_drag_field_modifier_load_binary(
    e: &mut Engine,
    this: Ptr<NiPSysDragFieldModifier>,
    stream: Ptr,
) {
    type M = NiPSysDragFieldModifier;
    e.call(FIELD_MODIFIER_LOAD_BINARY, &args![this, stream]);
    e.with_stack(8, |e, locals| {
        let flag = locals.addr();
        stream_read(e, stream.addr(), flag, 1, 1, flag + 4);
        let use_direction = u8::from(e.mem.u8(flag) != 0);
        e.set(this, M::m_bUseDirection, use_direction);
    });
    let direction = this.addr() + M::m_kDirection.off;
    e.call(POINT_LOAD_BINARY, &args![direction, stream]);
}

// Translated from 00c2ce70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysDragFieldModifier::SaveBinary` (Xbox PDB): `NiPSysFieldModifier`'s,
/// then the direction flag (one byte) and the direction
/// (`NiColor::SaveBinary`).
pub fn ni_p_sys_drag_field_modifier_save_binary(
    e: &mut Engine,
    this: Ptr<NiPSysDragFieldModifier>,
    stream: Ptr,
) {
    type M = NiPSysDragFieldModifier;
    e.call(FIELD_MODIFIER_SAVE_BINARY, &args![this, stream]);
    let use_direction = e.get(this, M::m_bUseDirection);
    e.with_stack(8, |e, locals| {
        let flag = locals.addr();
        e.mem.set_u8(flag, use_direction);
        stream_write(e, stream.addr(), flag, 1, 1, flag + 4);
    });
    let direction = this.addr() + M::m_kDirection.off;
    e.call(POINT_SAVE_BINARY, &args![direction, stream]);
}

// Translated from 00c2cec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysDragFieldModifier::GetViewerStrings` (Xbox PDB):
/// `NiPSysFieldModifier`'s strings, the class name, the direction flag and
/// the direction (`NiPoint3::GetViewerString`).
pub fn ni_p_sys_drag_field_modifier_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiPSysDragFieldModifier>,
    strings: Ptr,
) {
    type M = NiPSysDragFieldModifier;
    let strings = strings.addr();
    e.call(FIELD_MODIFIER_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.global::<u32>(RTTI);
    let string = e.call(VIEWER_STRING, &args![name]).u32();
    add_viewer_string(e, strings, string);
    let use_direction = u32::from(e.get(this, M::m_bUseDirection));
    let string = e
        .call(
            VIEWER_STRING_BOOL,
            &args![LABEL_USE_DIRECTION, use_direction],
        )
        .u32();
    add_viewer_string(e, strings, string);
    let direction = this.addr() + M::m_kDirection.off;
    let string = e
        .call(POINT_GET_VIEWER_STRING, &args![direction, LABEL_DIRECTION])
        .u32();
    add_viewer_string(e, strings, string);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c2c800, ni_p_sys_drag_field_modifier_get_r_t_t_i() -> u32),
        entry!(0x00c2c810, ni_p_sys_drag_field_modifier_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00c2_c830,
            fn_00c2c830(Ptr<NiPSysDragFieldModifier>, f32, Ptr)
        ),
        entry!(
            0x00c2_c8d0,
            ni_p_sys_drag_field_modifier_create_clone(
                Ptr<NiPSysDragFieldModifier>,
                Ptr,
            ) -> Ptr<NiPSysDragFieldModifier>
        ),
        entry!(
            0x00c2_c950,
            ni_p_sys_drag_field_modifier_update_no_attenuation_no_dir(
                Ptr<NiPSysDragFieldModifier>,
                f32,
                Ptr,
            )
        ),
        entry!(
            0x00c2_cae0,
            ni_p_sys_drag_field_modifier_is_equal(
                Ptr<NiPSysDragFieldModifier>,
                Ptr<NiPSysDragFieldModifier>,
            ) -> bool
        ),
        entry!(
            0x00c2_cb20,
            ni_p_sys_drag_field_modifier_update(Ptr<NiPSysDragFieldModifier>, f32, Ptr)
        ),
        entry!(
            0x00c2_ce10,
            ni_p_sys_drag_field_modifier_load_binary(Ptr<NiPSysDragFieldModifier>, Ptr)
        ),
        entry!(
            0x00c2_ce70,
            ni_p_sys_drag_field_modifier_save_binary(Ptr<NiPSysDragFieldModifier>, Ptr)
        ),
        entry!(
            0x00c2_cec0,
            ni_p_sys_drag_field_modifier_get_viewer_strings(Ptr<NiPSysDragFieldModifier>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_drag_field_modifier_scalar_deleting_destructor_deletes_only_on_bit_0() {
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
        assert_eq!(e.call(0x00c2c810, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c2c810, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x40]]);
        assert_eq!(calls_to(&e, 0x00c3_1750), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_p_sys_drag_field_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_c800, &args![]).u32(), 0x0120_2770);
    }
}

#[cfg(test)]
mod n064_tests {
    use super::*;
    use crate::units::niparticle::nipsysdata::viewer_test::{self, answer, calls_to, quiet};
    use crate::units::niparticle::nipsysmodifier::test_stream;

    type M = NiPSysDragFieldModifier;

    fn set_floats(e: &mut Engine, addr: u32, values: &[f32]) {
        for (i, value) in values.iter().enumerate() {
            e.mem.set_u32(addr + 4 * i as u32, value.to_bits());
        }
    }

    fn floats(e: &Engine, addr: u32) -> Vec<f32> {
        (0..3).map(|i| e.mem.f32(addr + 4 * i)).collect()
    }

    /// Doubles: the product is the field object's world transform, a point
    /// turns into itself, normalizing really normalizes, points compare by
    /// their bytes, the square root is exact; `NiPoint3::ZERO` and 1.0 are
    /// mapped.
    fn helpers(e: &mut Engine) {
        quiet(e, &[TRANSFORM_INVERT]);
        e.register(TRANSFORM_MULTIPLY, |e, a| {
            let words = e.mem.bytes(a[2], TRANSFORM_SIZE);
            e.mem.write(a[1], &words);
            Ret::default()
        });
        e.register(TRANSFORM_POINT, |e, a| {
            let words = e.mem.bytes(a[2], 12);
            e.mem.write(a[1], &words);
            answer(a[1])
        });
        e.register(POINT_UNITIZE, |e, a| {
            let v: Vec<f32> = (0..3).map(|i| e.mem.f32(a[0] + 4 * i)).collect();
            let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            for (i, x) in v.iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * i as u32, x / length);
            }
            Ret::default()
        });
        e.register(POINT_UNEQUAL, |e, a| {
            answer(u32::from(e.mem.bytes(a[0], 12) != e.mem.bytes(a[1], 12)))
        });
        e.register(SQRT, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)).sqrt(),
            ..Ret::default()
        });
        e.map(ZERO_POINT, 12);
        e.map(ONE, 8);
        e.set_global::<f64>(ONE, 1.0);
    }

    /// A modifier with magnitude 0.5 and one particle at `position` with
    /// velocity (2, 4, 6), last updated at 1.0; the field object is at the
    /// origin.
    fn scene(e: &mut Engine, position: [f32; 3]) -> (Ptr<M>, u32, u32) {
        let this = e.new_object::<M>();
        set_floats(e, this.addr() + MAGNITUDE, &[0.5]);
        let field = e.mem.alloc(0x100);
        let system = e.mem.alloc(0x100);
        e.mem.set_u32(this.addr() + FIELD_OBJECT, field);
        e.mem.set_u32(this.addr() + TARGET, system);
        let data = e.mem.alloc(0x80);
        e.mem.set_u16(data + DATA_ACTIVE, 1);
        let vertices = e.mem.alloc(12);
        set_floats(e, vertices, &position);
        e.mem.set_u32(data + DATA_VERTICES, vertices);
        let infos = e.mem.alloc(INFO_SIZE);
        set_floats(e, infos, &[2.0, 4.0, 6.0]);
        e.mem.set_u32(infos + INFO_LAST_UPDATE, 1.0f32.to_bits());
        e.mem.set_u32(data + DATA_INFOS, infos);
        (this, data, infos)
    }

    #[test]
    fn plain_update_scales_or_stops_the_velocity() {
        let mut e = Engine::new();
        helpers(&mut e);
        let (this, data, infos) = scene(&mut e, [0.0; 3]);
        e.call(0x00c2_c830, &args![this, 2.0f32, data]);
        assert_eq!(floats(&e, infos), vec![1.0, 2.0, 3.0]);
        // A drag of 1.0 stops the particle.
        e.call(0x00c2_c830, &args![this, 3.0f32, data]);
        assert_eq!(floats(&e, infos), vec![0.0, 0.0, 0.0]);
    }

    #[test]
    fn update_without_attenuation_or_direction_keeps_to_the_squared_distance() {
        let mut e = Engine::new();
        helpers(&mut e);
        let (this, data, infos) = scene(&mut e, [3.0, 4.0, 0.0]);
        set_floats(&mut e, this.addr() + MAX_DISTANCE_SQUARED, &[25.0]);
        e.call(0x00c2_c950, &args![this, 2.0f32, data]);
        assert_eq!(floats(&e, infos), vec![1.0, 2.0, 3.0]);
        set_floats(&mut e, this.addr() + MAX_DISTANCE_SQUARED, &[24.0]);
        e.call(0x00c2_c950, &args![this, 2.0f32, data]);
        assert_eq!(floats(&e, infos), vec![1.0, 2.0, 3.0]);
        // A drag of 0 changes nothing; one of 1.0 stops the particle.
        set_floats(&mut e, this.addr() + MAX_DISTANCE_SQUARED, &[26.0]);
        e.call(0x00c2_c950, &args![this, 1.0f32, data]);
        assert_eq!(floats(&e, infos), vec![1.0, 2.0, 3.0]);
        e.call(0x00c2_c950, &args![this, 3.0f32, data]);
        assert_eq!(floats(&e, infos), vec![0.0, 0.0, 0.0]);
    }

    #[test]
    fn update_picks_the_simple_paths() {
        let mut e = Engine::new();
        helpers(&mut e);
        let (this, data, infos) = scene(&mut e, [3.0, 4.0, 0.0]);
        e.call_log = Some(vec![]);
        // Magnitude 0: nothing.
        set_floats(&mut e, this.addr() + MAGNITUDE, &[0.0]);
        e.call(0x00c2_cb20, &args![this, 2.0f32, data]);
        assert_eq!(floats(&e, infos), vec![2.0, 4.0, 6.0]);
        set_floats(&mut e, this.addr() + MAGNITUDE, &[0.5]);
        // A distance limit (squared 24, below 25): the particle is kept.
        e.mem.set_u8(this.addr() + USE_MAX_DISTANCE, 1);
        set_floats(&mut e, this.addr() + MAX_DISTANCE_SQUARED, &[24.0]);
        e.call(0x00c2_cb20, &args![this, 2.0f32, data]);
        assert_eq!(floats(&e, infos), vec![2.0, 4.0, 6.0]);
        assert_eq!(calls_to(&e, TRANSFORM_INVERT).len(), 1);
        // No limit: the plain update.
        e.mem.set_u8(this.addr() + USE_MAX_DISTANCE, 0);
        e.call(0x00c2_cb20, &args![this, 2.0f32, data]);
        assert_eq!(floats(&e, infos), vec![1.0, 2.0, 3.0]);
        assert_eq!(calls_to(&e, TRANSFORM_INVERT).len(), 1);
        // No field object or no particles: nothing.
        e.mem.set_u16(data + DATA_ACTIVE, 0);
        e.call(0x00c2_cb20, &args![this, 2.0f32, data]);
        assert_eq!(floats(&e, infos), vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn update_attenuates_with_the_distance() {
        let mut e = Engine::new();
        helpers(&mut e);
        let (this, data, infos) = scene(&mut e, [3.0, 4.0, 0.0]);
        // Drag 1.0 * 3.0 / (1.0 * 5 + 1) = 0.5.
        set_floats(&mut e, this.addr() + MAGNITUDE, &[3.0, 1.0]);
        e.call(0x00c2_cb20, &args![this, 2.0f32, data]);
        assert_eq!(floats(&e, infos), vec![1.0, 2.0, 3.0]);
        // Beyond the distance limit: kept.
        e.mem.set_u8(this.addr() + USE_MAX_DISTANCE, 1);
        set_floats(&mut e, this.addr() + MAX_DISTANCE, &[4.9]);
        e.call(0x00c2_cb20, &args![this, 2.0f32, data]);
        assert_eq!(floats(&e, infos), vec![1.0, 2.0, 3.0]);
        // Within it, at 1.0 or more: stopped.
        set_floats(&mut e, this.addr() + MAX_DISTANCE, &[5.0]);
        e.call(0x00c2_cb20, &args![this, 3.0f32, data]);
        assert_eq!(floats(&e, infos), vec![0.0, 0.0, 0.0]);
        // No elapsed time: kept.
        set_floats(&mut e, infos, &[2.0, 4.0, 6.0]);
        e.call(0x00c2_cb20, &args![this, 1.0f32, data]);
        assert_eq!(floats(&e, infos), vec![2.0, 4.0, 6.0]);
    }

    #[test]
    fn update_with_a_direction_drags_along_it() {
        let mut e = Engine::new();
        helpers(&mut e);
        let (this, data, infos) = scene(&mut e, [3.0, 4.0, 0.0]);
        set_floats(&mut e, this.addr() + MAGNITUDE, &[3.0, 1.0]);
        e.set(this, M::m_bUseDirection, 1);
        set_floats(&mut e, this.addr() + 0x34, &[0.0, 0.0, 1.0]);
        // Velocity along the direction: strength 1.
        set_floats(&mut e, infos, &[0.0, 0.0, 6.0]);
        e.call(0x00c2_cb20, &args![this, 2.0f32, data]);
        assert_eq!(floats(&e, infos), vec![0.0, 0.0, 3.0]);
        // Across it: strength 0.
        set_floats(&mut e, infos, &[6.0, 0.0, 0.0]);
        e.call(0x00c2_cb20, &args![this, 2.0f32, data]);
        assert_eq!(floats(&e, infos), vec![6.0, 0.0, 0.0]);
        // A zero direction is not turned; the unwritten local is 0 here.
        set_floats(&mut e, this.addr() + 0x34, &[0.0, 0.0, 0.0]);
        set_floats(&mut e, infos, &[0.0, 0.0, 6.0]);
        e.call_log = Some(vec![]);
        e.call(0x00c2_cb20, &args![this, 2.0f32, data]);
        assert!(calls_to(&e, TRANSFORM_POINT).is_empty());
        assert_eq!(floats(&e, infos), vec![0.0, 0.0, 6.0]);
    }

    #[test]
    fn create_clone_copies_the_direction() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| answer(e.mem.alloc(a[0])));
        quiet(
            &mut e,
            &[FIELD_MODIFIER_CONSTRUCTOR, FIELD_MODIFIER_COPY_MEMBERS],
        );
        e.map(ZERO_POINT, 12);
        let this = e.new_object::<M>();
        e.set(this, M::m_bUseDirection, 1);
        set_floats(&mut e, this.addr() + 0x34, &[1.0, 2.0, 3.0]);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c2_c8d0, &args![this, 0x900u32]).u32();
        assert_eq!(e.mem.u32(clone), CLONE_VTABLE);
        assert_eq!(calls_to(&e, FIELD_MODIFIER_CONSTRUCTOR), vec![vec![clone]]);
        assert_eq!(
            calls_to(&e, FIELD_MODIFIER_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x900]]
        );
        assert_eq!(e.mem.u8(clone + 0x30), 1);
        assert_eq!(floats(&e, clone + 0x34), vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn is_equal_compares_flag_and_direction() {
        let mut e = Engine::new();
        helpers(&mut e);
        e.register(FIELD_MODIFIER_IS_EQUAL, |_, _| answer(1));
        let this = e.new_object::<M>();
        let other = e.new_object::<M>();
        let check = |e: &mut Engine| e.call(0x00c2_cae0, &args![this, other]).bool();
        e.call_log = Some(vec![]);
        assert!(check(&mut e));
        assert_eq!(
            calls_to(&e, POINT_UNEQUAL),
            vec![vec![other.addr() + 0x34, this.addr() + 0x34]]
        );
        e.set(other, M::m_bUseDirection, 1);
        assert!(!check(&mut e));
        e.set(this, M::m_bUseDirection, 1);
        set_floats(&mut e, other.addr() + 0x38, &[1.0]);
        assert!(!check(&mut e));
        set_floats(&mut e, this.addr() + 0x38, &[1.0]);
        assert!(check(&mut e));
        e.register(FIELD_MODIFIER_IS_EQUAL, |_, _| answer(0));
        assert!(!check(&mut e));
    }

    #[test]
    fn load_and_save_binary_carry_flag_and_direction() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                FIELD_MODIFIER_LOAD_BINARY,
                FIELD_MODIFIER_SAVE_BINARY,
                POINT_LOAD_BINARY,
                POINT_SAVE_BINARY,
            ],
        );
        let this = e.new_object::<M>();
        let stream = test_stream::new(&mut e, 0x1402_0007, &[7]);
        e.call_log = Some(vec![]);
        e.call(0x00c2_ce10, &args![this, stream]);
        assert_eq!(e.get(this, M::m_bUseDirection), 1);
        assert_eq!(test_stream::position(&e, stream), 1);
        assert_eq!(
            calls_to(&e, POINT_LOAD_BINARY),
            vec![vec![this.addr() + 0x34, stream]]
        );
        let out = viewer_test::out_stream(&mut e);
        e.call(0x00c2_ce70, &args![this, out]);
        assert_eq!(viewer_test::written(&e, out), (vec![1], 1));
        assert_eq!(
            calls_to(&e, POINT_SAVE_BINARY),
            vec![vec![this.addr() + 0x34, out]]
        );
        assert_eq!(
            calls_to(&e, FIELD_MODIFIER_SAVE_BINARY),
            vec![vec![this.addr(), out]]
        );
    }

    #[test]
    fn viewer_strings_name_flag_and_direction() {
        let mut e = Engine::new();
        quiet(&mut e, &[FIELD_MODIFIER_GET_VIEWER_STRINGS]);
        viewer_test::counting(
            &mut e,
            &[VIEWER_STRING, VIEWER_STRING_BOOL, POINT_GET_VIEWER_STRING],
        );
        e.map(RTTI, 4);
        e.set_global::<u32>(RTTI, 0x0100_00c0);
        let this = e.new_object::<M>();
        e.set(this, M::m_bUseDirection, 1);
        let strings = viewer_test::strings(&mut e, 8);
        e.call_log = Some(vec![]);
        e.call(0x00c2_cec0, &args![this, strings]);
        assert_eq!(viewer_test::collected(&e, strings), vec![1, 2, 3]);
        assert_eq!(
            calls_to(&e, VIEWER_STRING_BOOL),
            vec![vec![LABEL_USE_DIRECTION, 1]]
        );
        assert_eq!(
            calls_to(&e, POINT_GET_VIEWER_STRING),
            vec![vec![this.addr() + 0x34, LABEL_DIRECTION]]
        );
    }
}
