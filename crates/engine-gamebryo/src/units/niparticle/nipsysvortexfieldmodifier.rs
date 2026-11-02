//! `NiParticle/nipsysvortexfieldmodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c26810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysVortexFieldModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2688.
pub fn ni_p_sys_vortex_field_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2688
}

/// `NiPSysFieldModifier`'s fields as the PC code reads them (Xbox PDB
/// names): `m_pkTarget` +0x10 (the particle system), `m_pkFieldObj` +0x18,
/// `m_fMagnitude` +0x1C, `m_fAttenuation` +0x20, `m_bUseMaxDistance` +0x24,
/// `m_fMaxDistance` +0x28; this class's `m_kDirection` (`NiPoint3`) +0x30.
const TARGET: u32 = 0x10;
const FIELD_OBJECT: u32 = 0x18;
const MAGNITUDE: u32 = 0x1c;
const ATTENUATION: u32 = 0x20;
const USE_MAX_DISTANCE: u32 = 0x24;
const MAX_DISTANCE: u32 = 0x28;
const DIRECTION: u32 = 0x30;
/// An `NiAVObject`'s world transform (+0x68, 13 words).
const WORLD: u32 = 0x68;
/// `NiPSysData` (as the PC code reads it): vertices (+0x20), the active
/// count (`u16` +0x50) and the `NiParticleInfo` array (+0x70, 0x1C each:
/// velocity +0, last update +0x14).
const DATA_VERTICES: u32 = 0x20;
const DATA_ACTIVE: u32 = 0x50;
const DATA_INFO: u32 = 0x70;
/// The vortex modifier's vtable (RTTI `.?AVNiPSysVortexFieldModifier@@`)
/// and `NiPoint3::ZERO` (`011f426c`, three floats).
const VORTEX_VTABLE: u32 = 0x010b_e604;
const POINT_ZERO: u32 = 0x011f_426c;
/// `00aa13e0` (`cdecl`, one word: the size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The `NiPSysFieldModifier` constructor (`00c31ff0`, `thiscall`, no
/// words), member copy (`00c32020`, `thiscall`, `(clone, process)`),
/// `LoadBinary` (`00c32120`, `thiscall`, one word) and `GetViewerStrings`
/// (`00c322b0`, `thiscall`, one word).
const FIELD_CONSTRUCTOR: u32 = 0x00c3_1ff0;
const FIELD_COPY_MEMBERS: u32 = 0x00c3_2020;
const FIELD_LOAD_BINARY: u32 = 0x00c3_2120;
const FIELD_GET_VIEWER_STRINGS: u32 = 0x00c3_22b0;
/// `NiColor::LoadBinary` (`00a59be0`, `thiscall` on three floats, the
/// stream).
const COLOR_LOAD_BINARY: u32 = 0x00a5_9be0;
/// `NiPoint3::UnitCross` (`0053d1a0`, `thiscall`, `(result, other)`).
const POINT_UNIT_CROSS: u32 = 0x0053_d1a0;
/// `004390c0` (unnamed, `thiscall` on a point, one word: the other point):
/// answers nonzero when the points are equal.
const POINT_EQUALS: u32 = 0x0043_90c0;
/// `NiTransform::Invert` (`004b4880`, `thiscall`, one word: the result),
/// the transform product (`0062c250`, `thiscall`, `(result, other)`), the
/// matrix-times-point (`004b4500`, `thiscall`, `(result, point)`),
/// `NiPoint3::Unitize` (`004a0c10`, `thiscall`, no words) and
/// `NiPoint3::Length` (`00457990`, `thiscall`, no words, answers `ST0`).
const TRANSFORM_INVERT: u32 = 0x004b_4880;
const TRANSFORM_TIMES_TRANSFORM: u32 = 0x0062_c250;
const MATRIX_TIMES_POINT: u32 = 0x004b_4500;
const POINT_UNITIZE: u32 = 0x004a_0c10;
const POINT_LENGTH: u32 = 0x0045_7990;
/// `NiPoint3::operator*=(float)` (`00439180`, `thiscall`, one word) and
/// `NiPoint3::operator+=` (`0063c8a0`, `thiscall`, one word: the point).
const POINT_SCALE: u32 = 0x0043_9180;
const POINT_ADD: u32 = 0x0063_c8a0;
/// The square-root helper (`00ec6040`) and `_CIpow` (`00ec9b00`, base then
/// exponent): `ST0` values passed as leading `f64`s, answers in `ST0`.
const SQRT: u32 = 0x00ec_6040;
const CI_POW: u32 = 0x00ec_9b00;
/// `NiGetViewerString` (`00a7d330`, `cdecl`, the class name),
/// `NiPoint3::GetViewerString` (`00a59af0`, `thiscall`, the name), the
/// viewer-string array's grow (`0096ad30`) and store (`0096ae90`), the
/// class's `NiRTTI` record and `"Direction"` (`01044b44`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const POINT_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
const VORTEX_RTTI: u32 = 0x0120_2688;
const DIRECTION_NAME: u32 = 0x0104_4b44;

/// Appends `text` to the viewer-string array (the inlined `Add`).
fn push_viewer_string(e: &mut Engine, strings: Ptr, text: u32) {
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// Copies `words` words from `from` to `to`.
fn copy_words(e: &mut Engine, to: u32, from: u32, words: u32) {
    for i in 0..words {
        let value = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, value);
    }
}

/// The field object's transform in the particle system's space, as the
/// three update functions build it: the field object's world transform
/// copied to `field`, the system's to `system`, the system's inverted into
/// `inverse` (`004b4880`) and the product `inverse * field` written over
/// `system` (`0062c250`).
fn field_in_system_space(e: &mut Engine, this: Ptr, field: u32, system: u32, inverse: u32) {
    let field_object = e.mem.u32(this.addr() + FIELD_OBJECT);
    copy_words(e, field, field_object + WORLD, 13);
    let target = e.mem.u32(this.addr() + TARGET);
    copy_words(e, system, target + WORLD, 13);
    e.call(TRANSFORM_INVERT, &args![system, inverse]);
    e.call(TRANSFORM_TIMES_TRANSFORM, &args![inverse, system, field]);
}

// Translated from 00c267d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysVortexFieldModifier::LoadBinary` (Xbox PDB): the field-modifier
/// part (`00c32120`), then the direction (`NiColor::LoadBinary`, three
/// floats).
pub fn ni_p_sys_vortex_field_modifier_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(FIELD_LOAD_BINARY, &args![this, stream]);
    e.call(COLOR_LOAD_BINARY, &args![this.addr() + DIRECTION, stream]);
}

// Translated from 00c26820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysVortexFieldModifier::VortexDirection` (Xbox PDB; `RET 0x10`, its
/// callers pass the modifier in ECX, which it never reads): the offset of
/// `point` from `center` with its component along `axis` removed (`d - (d .
/// axis) axis`, the dot product kept in `point`'s argument slot); when that
/// equals `NiPoint3::ZERO` it is the result, otherwise the result is
/// `axis.UnitCross(offset)` (`0053d1a0`). Answers `result`.
pub fn ni_p_sys_vortex_field_modifier_vortex_direction(
    e: &mut Engine,
    _this: Ptr,
    result: Ptr,
    center: Ptr,
    axis: Ptr,
    point: Ptr,
) -> Ptr {
    let (p, c, n) = (point.addr(), center.addr(), axis.addr());
    let m = |e: &Engine, a: u32| x87_m32(e.mem.u32(a));
    let ld = |e: &Engine, a: u32| x87_load32(e.mem.u32(a));
    let d0 = x87_load32(x87_store32(x87_sub(ld(e, p), m(e, c))));
    let d1 = x87_load32(x87_store32(x87_sub(ld(e, p + 4), m(e, c + 4))));
    let d2 = x87_load32(x87_store32(x87_sub(ld(e, p + 8), m(e, c + 8))));
    let dot = x87_add(
        x87_add(x87_mul(ld(e, n), d0), x87_mul(d1, m(e, n + 4))),
        x87_mul(d2, m(e, n + 8)),
    );
    let dot = x87_load32(x87_store32(dot));
    let along0 = x87_store32(x87_mul(ld(e, n), dot));
    let along1 = x87_store32(x87_mul(dot, m(e, n + 4)));
    let along2 = x87_store32(x87_mul(dot, m(e, n + 8)));
    e.with_stack(12, |e, offset| {
        let o = offset.addr();
        e.mem
            .set_u32(o, x87_store32(x87_sub(d0, x87_load32(along0))));
        e.mem
            .set_u32(o + 4, x87_store32(x87_sub(d1, x87_m32(along1))));
        e.mem
            .set_u32(o + 8, x87_store32(x87_sub(d2, x87_m32(along2))));
        let zero = |e: &Engine, i: u32| x87_load32(e.global::<u32>(POINT_ZERO + 4 * i));
        let is_zero = (0..3).all(|i| ld(e, o + 4 * i) == zero(e, i));
        if is_zero {
            copy_words(e, result.addr(), o, 3);
        } else {
            e.call(POINT_UNIT_CROSS, &args![axis, result, offset]);
        }
    });
    result
}

// Translated from 00c26940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysVortexFieldModifier::CreateClone` (Xbox PDB): 0x3C bytes from the
/// Ni allocator, constructed (the field-modifier constructor `00c31ff0`, the
/// vtable, the direction `NiPoint3::ZERO`; null when the allocation fails,
/// then used anyway as the exe does), the field members copied (`00c32020`)
/// and the direction copied (`MOV`). Answers the clone.
pub fn ni_p_sys_vortex_field_modifier_create_clone(e: &mut Engine, this: Ptr, process: Ptr) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x3cu32]).u32();
    if block != 0 {
        e.call(FIELD_CONSTRUCTOR, &args![block]);
        e.mem.set_u32(block, VORTEX_VTABLE);
        copy_words(e, block + DIRECTION, POINT_ZERO, 3);
    }
    e.call(FIELD_COPY_MEMBERS, &args![this, block, process]);
    copy_words(e, block.wrapping_add(DIRECTION), this.addr() + DIRECTION, 3);
    Ptr::new(block)
}

/// The vortex push on one particle: `VortexDirection` from the field's
/// centre about its axis, times `magnitude * dt` (`FLD`/`FMUL`, stored),
/// the three products stored, then added to the velocity. `swap_y` is the
/// operand order of the y sum (`00c26c00` loads the product first).
fn push_particle(
    e: &mut Engine,
    this: Ptr,
    frame: u32,
    [result, center, axis, point, scale]: [u32; 5],
    dt: u32,
    info: u32,
    swap_y: bool,
) {
    ni_p_sys_vortex_field_modifier_vortex_direction(
        e,
        this,
        Ptr::new(frame + result),
        Ptr::new(frame + center),
        Ptr::new(frame + axis),
        Ptr::new(frame + point),
    );
    let ld = |e: &Engine, a: u32| x87_load32(e.mem.u32(a));
    let m = |e: &Engine, a: u32| x87_m32(e.mem.u32(a));
    let k = x87_store32(x87_mul(ld(e, this.addr() + MAGNITUDE), x87_m32(dt)));
    e.mem.set_u32(frame + scale, k);
    let r = frame + result;
    let k = x87_load32(k);
    e.mem.set_u32(r, x87_store32(x87_mul(ld(e, r), k)));
    e.mem.set_u32(r + 4, x87_store32(x87_mul(ld(e, r + 4), k)));
    e.mem.set_u32(r + 8, x87_store32(x87_mul(k, m(e, r + 8))));
    let x = x87_add(ld(e, info), m(e, r));
    e.mem.set_u32(info, x87_store32(x));
    let y = if swap_y {
        x87_add(ld(e, r + 4), m(e, info + 4))
    } else {
        x87_add(ld(e, info + 4), m(e, r + 4))
    };
    e.mem.set_u32(info + 4, x87_store32(y));
    let z = x87_add(ld(e, info + 8), m(e, r + 8));
    e.mem.set_u32(info + 8, x87_store32(z));
}

// Translated from 00c269b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the vortex update without attenuation, with
/// the maximum distance (`thiscall`, `RET 8`: the time, the particle data;
/// `Update` calls it). The field's transform in the system's space
/// (inverse of the system's world times the field object's), the direction
/// turned by its rotation and unitized (`004a0c10`). Each particle whose
/// `time - lastUpdate` is not 0 and whose distance from the field's centre
/// (`ST0` through `00ec6040`) is not 0 and not above `maxDistance *
/// maxDistance` (the exe compares the distance with the squared maximum; a
/// NaN fails) gets the vortex push (`VortexDirection` times `magnitude *
/// dt`) added to its velocity. Locals sit at the exe's frame offsets.
pub fn fn_00c269b0(e: &mut Engine, this: Ptr, time: u32, data: Ptr) {
    e.with_stack(0xf4, |e, frame| {
        let f = frame.addr();
        let t = this.addr();
        field_in_system_space(e, this, f + 0x8c, f + 0x4c, f + 0xc0);
        let ld = |e: &Engine, a: u32| x87_load32(e.mem.u32(a));
        let m = |e: &Engine, a: u32| x87_m32(e.mem.u32(a));
        let (x, y, z) = (t + DIRECTION, t + DIRECTION + 4, t + DIRECTION + 8);
        let first = x87_add(
            x87_mul(ld(e, y), m(e, f + 0x50)),
            x87_mul(ld(e, x), m(e, f + 0x4c)),
        );
        copy_words(e, f + 0x80, f + 0x70, 3);
        let first = x87_add(first, x87_mul(ld(e, z), m(e, f + 0x54)));
        e.mem.set_u32(f + 0x40, x87_store32(first));
        let second = x87_add(
            x87_mul(ld(e, x), m(e, f + 0x58)),
            x87_mul(ld(e, y), m(e, f + 0x5c)),
        );
        let second = x87_add(second, x87_mul(ld(e, z), m(e, f + 0x60)));
        e.mem.set_u32(f + 0x44, x87_store32(second));
        let third = x87_add(
            x87_mul(ld(e, x), m(e, f + 0x64)),
            x87_mul(ld(e, y), m(e, f + 0x68)),
        );
        let third = x87_add(third, x87_mul(ld(e, z), m(e, f + 0x6c)));
        e.mem.set_u32(f + 0x48, x87_store32(third));
        e.call(POINT_UNITIZE, &args![f + 0x40]);
        let max = ld(e, t + MAX_DISTANCE);
        e.mem.set_u32(f + 0x3c, x87_store32(x87_mul(max, max)));
        let d = data.addr();
        let mut index = 0u16;
        while index < e.mem.u16(d + DATA_ACTIVE) {
            let i = u32::from(index);
            let info = e.mem.u32(d + DATA_INFO).wrapping_add(i * 0x1c);
            let dt = x87_store32(x87_sub(x87_load32(time), m(e, info + 0x14)));
            e.mem.set_u32(f + 0x20, dt);
            if 0.0 != x87_m32(dt) {
                let vertex = e.mem.u32(d + DATA_VERTICES).wrapping_add(i * 12);
                copy_words(e, f + 0x24, vertex, 3);
                for k in 0..3 {
                    let v = x87_sub(ld(e, f + 0x24 + 4 * k), m(e, f + 0x70 + 4 * k));
                    e.mem.set_u32(f + 0x30 + 4 * k, x87_store32(v));
                }
                let (dy, dx, dz) = (ld(e, f + 0x34), ld(e, f + 0x30), ld(e, f + 0x38));
                let planar = x87_add(x87_mul(dy, dy), x87_mul(dx, dx));
                let squared = x87_add(planar, x87_mul(dz, dz));
                e.mem.set_u32(f + 0x10, x87_store32(squared));
                let root = e.call(SQRT, &args![ld(e, f + 0x10)]).f64();
                e.mem.set_u32(f + 0x10, x87_quiet_bits(x87_store32(root)));
                let distance = ld(e, f + 0x10);
                if distance != 0.0 && ld(e, f + 0x3c) >= distance {
                    push_particle(e, this, f, [0x14, 0x80, 0x40, 0x24, 0x10], dt, info, false);
                }
            }
            index = index.wrapping_add(1);
        }
    });
}

// Translated from 00c26c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the vortex update without attenuation or a
/// maximum distance (`thiscall`, `RET 8`: the time, the particle data;
/// `Update` calls it): as `00c269b0` without the distance, every particle
/// whose `time - lastUpdate` is not 0 gets the vortex push. The direction's
/// second row and the y velocity sum take their operands in the exe's
/// (different) order. Locals sit at the exe's frame offsets.
pub fn fn_00c26c00(e: &mut Engine, this: Ptr, time: u32, data: Ptr) {
    e.with_stack(0xe0, |e, frame| {
        let f = frame.addr();
        let t = this.addr();
        field_in_system_space(e, this, f + 0x78, f + 0x44, f + 0xac);
        let ld = |e: &Engine, a: u32| x87_load32(e.mem.u32(a));
        let m = |e: &Engine, a: u32| x87_m32(e.mem.u32(a));
        let (x, y, z) = (t + DIRECTION, t + DIRECTION + 4, t + DIRECTION + 8);
        let first = x87_add(
            x87_mul(ld(e, y), m(e, f + 0x48)),
            x87_mul(ld(e, x), m(e, f + 0x44)),
        );
        copy_words(e, f + 0x38, f + 0x68, 3);
        let first = x87_add(first, x87_mul(ld(e, z), m(e, f + 0x4c)));
        e.mem.set_u32(f + 0x20, x87_store32(first));
        let second = x87_add(
            x87_mul(ld(e, y), m(e, f + 0x54)),
            x87_mul(ld(e, x), m(e, f + 0x50)),
        );
        let second = x87_add(second, x87_mul(ld(e, z), m(e, f + 0x58)));
        e.mem.set_u32(f + 0x24, x87_store32(second));
        let third = x87_add(
            x87_mul(ld(e, x), m(e, f + 0x5c)),
            x87_mul(ld(e, y), m(e, f + 0x60)),
        );
        let third = x87_add(third, x87_mul(ld(e, z), m(e, f + 0x64)));
        e.mem.set_u32(f + 0x28, x87_store32(third));
        e.call(POINT_UNITIZE, &args![f + 0x20]);
        let d = data.addr();
        let mut index = 0u16;
        while index < e.mem.u16(d + DATA_ACTIVE) {
            let i = u32::from(index);
            let info = e.mem.u32(d + DATA_INFO).wrapping_add(i * 0x1c);
            let dt = x87_store32(x87_sub(x87_load32(time), m(e, info + 0x14)));
            e.mem.set_u32(f + 0x10, dt);
            if 0.0 != x87_m32(dt) {
                let vertex = e.mem.u32(d + DATA_VERTICES).wrapping_add(i * 12);
                copy_words(e, f + 0x2c, vertex, 3);
                push_particle(e, this, f, [0x14, 0x38, 0x20, 0x2c, 0x10], dt, info, true);
            }
            index = index.wrapping_add(1);
        }
    });
}

// Translated from 00c26de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysVortexFieldModifier::Update` (Xbox PDB): nothing when the
/// magnitude is 0, the data has no active particles, the direction is
/// `NiPoint3::ZERO` (`004390c0`) or there is no field object. Without
/// attenuation (0) it hands over to `00c269b0` (with a maximum distance) or
/// `00c26c00` (the time pushed by `FLD`/`FSTP`). Otherwise: the field's
/// transform in the system's space, the direction turned by its rotation
/// (`004b4500`) and unitized; each particle whose `time - lastUpdate` is not
/// 0 and whose distance (`NiPoint3::Length`) is not 0 and, with a maximum
/// distance, not above it (a NaN fails), gets `VortexDirection` scaled by
/// `dt * magnitude / distance^attenuation` (`_CIpow`; `dt * magnitude` kept
/// as a double) through `NiPoint3::operator*=` and added to its velocity
/// (`0063c8a0`). Locals sit at the exe's frame offsets.
pub fn ni_p_sys_vortex_field_modifier_update(e: &mut Engine, this: Ptr, time: u32, data: Ptr) {
    let t = this.addr();
    let d = data.addr();
    if 0.0 == x87_m32(e.mem.u32(t + MAGNITUDE)) || e.mem.u16(d + DATA_ACTIVE) == 0 {
        return;
    }
    if e.call(POINT_EQUALS, &args![t + DIRECTION, POINT_ZERO]).u8() != 0 {
        return;
    }
    if e.mem.u32(t + FIELD_OBJECT) == 0 {
        return;
    }
    if 0.0 == x87_m32(e.mem.u32(t + ATTENUATION)) {
        let time = x87_quiet_bits(time);
        if e.mem.u8(t + USE_MAX_DISTANCE) != 0 {
            fn_00c269b0(e, this, time, data);
        } else {
            fn_00c26c00(e, this, time, data);
        }
        return;
    }
    e.with_stack(0x100, |e, frame| {
        let f = frame.addr();
        field_in_system_space(e, this, f + 0x98, f + 0x64, f + 0xcc);
        copy_words(e, f + 0x40, f + 0x88, 3);
        e.call(
            MATRIX_TIMES_POINT,
            &args![f + 0x64, f + 0x58, t + DIRECTION],
        );
        e.call(POINT_UNITIZE, &args![f + 0x58]);
        let ld = |e: &Engine, a: u32| x87_load32(e.mem.u32(a));
        let m = |e: &Engine, a: u32| x87_m32(e.mem.u32(a));
        let mut index = 0u16;
        while index < e.mem.u16(d + DATA_ACTIVE) {
            let i = u32::from(index);
            let info = e.mem.u32(d + DATA_INFO).wrapping_add(i * 0x1c);
            e.mem.set_u32(f + 0x30, info);
            let dt = x87_store32(x87_sub(x87_load32(time), m(e, info + 0x14)));
            e.mem.set_u32(f + 0x18, dt);
            if 0.0 != x87_m32(dt) {
                let vertex = e.mem.u32(d + DATA_VERTICES).wrapping_add(i * 12);
                copy_words(e, f + 0x24, vertex, 3);
                for k in 0..3 {
                    let v = x87_sub(ld(e, f + 0x24 + 4 * k), m(e, f + 0x88 + 4 * k));
                    e.mem.set_u32(f + 0x34 + 4 * k, x87_store32(v));
                }
                let length = e.call(POINT_LENGTH, &args![f + 0x34]).f64();
                e.mem.set_u32(f + 0x10, x87_store32(length));
                let distance = ld(e, f + 0x10);
                let within = distance != 0.0
                    && (e.mem.u8(t + USE_MAX_DISTANCE) == 0 || ld(e, t + MAX_DISTANCE) >= distance);
                if within {
                    ni_p_sys_vortex_field_modifier_vortex_direction(
                        e,
                        this,
                        Ptr::new(f + 0x4c),
                        Ptr::new(f + 0x40),
                        Ptr::new(f + 0x58),
                        Ptr::new(f + 0x24),
                    );
                    let attenuation = e.mem.u32(t + ATTENUATION);
                    let push = x87_mul(ld(e, f + 0x18), m(e, t + MAGNITUDE));
                    let scale = if 0.0 == x87_m32(attenuation) {
                        x87_store32(push)
                    } else {
                        e.mem.set_u64(f + 0x18, push.to_bits());
                        let power = e
                            .call(CI_POW, &args![ld(e, f + 0x10), x87_load32(attenuation)])
                            .f64();
                        e.mem.set_u32(f + 0x10, x87_store32(power));
                        let scaled = x87_div(f64::from_bits(e.mem.u64(f + 0x18)), ld(e, f + 0x10));
                        e.mem.set_u32(f + 0x10, x87_store32(scaled));
                        e.mem.u32(f + 0x10)
                    };
                    e.call(POINT_SCALE, &args![f + 0x4c, scale]);
                    e.call(POINT_ADD, &args![info, f + 0x4c]);
                }
            }
            index = index.wrapping_add(1);
        }
    });
}

// Translated from 00c27060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysVortexFieldModifier::GetViewerStrings` (Xbox PDB): the
/// field-modifier strings (`00c322b0`), then the class name and the
/// direction (`NiPoint3::GetViewerString` with `"Direction"`).
pub fn ni_p_sys_vortex_field_modifier_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(FIELD_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(VORTEX_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    let text = e
        .call(
            POINT_GET_VIEWER_STRING,
            &args![this.addr() + DIRECTION, DIRECTION_NAME],
        )
        .u32();
    push_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c26810, ni_p_sys_vortex_field_modifier_get_r_t_t_i() -> u32),
        entry!(
            0x00c2_67d0,
            ni_p_sys_vortex_field_modifier_load_binary(Ptr, Ptr)
        ),
        entry!(
            0x00c2_6820,
            ni_p_sys_vortex_field_modifier_vortex_direction(Ptr, Ptr, Ptr, Ptr, Ptr) -> Ptr
        ),
        entry!(0x00c2_6940, ni_p_sys_vortex_field_modifier_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00c2_69b0, fn_00c269b0(Ptr, u32, Ptr)),
        entry!(0x00c2_6c00, fn_00c26c00(Ptr, u32, Ptr)),
        entry!(
            0x00c2_6de0,
            ni_p_sys_vortex_field_modifier_update(Ptr, u32, Ptr)
        ),
        entry!(
            0x00c2_7060,
            ni_p_sys_vortex_field_modifier_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_vortex_field_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_6810, &args![]).u32(), 0x0120_2688);
    }
}

#[cfg(test)]
mod n066_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn f64_arg(a: &[u32], i: usize) -> f64 {
        f64::from_bits(u64::from(a[i]) | u64::from(a[i + 1]) << 32)
    }

    fn point(e: &mut Engine, x: f32, y: f32, z: f32) -> u32 {
        let p = e.mem.alloc(12);
        e.mem.set_f32(p, x);
        e.mem.set_f32(p + 4, y);
        e.mem.set_f32(p + 8, z);
        p
    }

    fn read(e: &Engine, p: u32) -> [f32; 3] {
        [e.mem.f32(p), e.mem.f32(p + 4), e.mem.f32(p + 8)]
    }

    /// Doubles for the point and transform helpers: the cross product
    /// answers (0, 1, 0), inversion and products copy, unitizing and the
    /// point equality (against zero) are real enough for the tests.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(POINT_ZERO, 12);
        e.register(POINT_UNIT_CROSS, |e, a| {
            e.mem.set_f32(a[1], 0.0);
            e.mem.set_f32(a[1] + 4, 1.0);
            e.mem.set_f32(a[1] + 8, 0.0);
            word(a[1])
        });
        e.register(TRANSFORM_INVERT, |e, a| {
            let bytes = e.mem.bytes(a[0], 52);
            e.mem.write(a[1], &bytes);
            Ret::default()
        });
        e.register(TRANSFORM_TIMES_TRANSFORM, |e, a| {
            let bytes = e.mem.bytes(a[2], 52);
            e.mem.write(a[1], &bytes);
            word(a[1])
        });
        e.register(MATRIX_TIMES_POINT, |e, a| {
            let bytes = e.mem.bytes(a[2], 12);
            e.mem.write(a[1], &bytes);
            word(a[1])
        });
        e.register(POINT_UNITIZE, |_, _| Ret::default());
        e.register(POINT_EQUALS, |e, a| {
            let zero = (0..3).all(|i| e.mem.f32(a[0] + 4 * i) == 0.0);
            word(u32::from(zero))
        });
        e.register(SQRT, |_, a| Ret {
            st0: f64_arg(a, 0).sqrt(),
            ..Ret::default()
        });
        e.register(POINT_LENGTH, |e, a| {
            let [x, y, z] = read(e, a[0]);
            Ret {
                st0: f64::from(x * x + y * y + z * z).sqrt(),
                ..Ret::default()
            }
        });
        e.register(CI_POW, |_, a| Ret {
            st0: f64_arg(a, 0).powf(f64_arg(a, 2)),
            ..Ret::default()
        });
        e.register(POINT_SCALE, |e, a| {
            let k = f32::from_bits(a[1]);
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, v * k);
            }
            Ret::default()
        });
        e.register(POINT_ADD, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[1] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            word(a[0])
        });
        e
    }

    /// A modifier (magnitude 3, direction (0, 0, 1), identity transforms
    /// for the field object and the system) and data with one particle at
    /// `vertex`, last updated at 0; answers (modifier, data, info).
    fn setup(e: &mut Engine, vertex: [f32; 3]) -> (Ptr, Ptr, u32) {
        let this = e.mem.alloc(0x3c);
        e.mem.set_f32(this + MAGNITUDE, 3.0);
        e.mem.set_f32(this + DIRECTION + 8, 1.0);
        for owner in [FIELD_OBJECT, TARGET] {
            let object = e.mem.alloc(0x100);
            for i in [0u32, 4, 8] {
                e.mem.set_f32(object + WORLD + i * 4, 1.0);
            }
            e.mem.set_f32(object + WORLD + 0x30, 1.0);
            e.mem.set_u32(this + owner, object);
        }
        let data = e.mem.alloc(0x7c);
        e.mem.set_u16(data + DATA_ACTIVE, 1);
        let info = e.mem.alloc(0x1c);
        e.mem.set_u32(data + DATA_INFO, info);
        let vertices = point(e, vertex[0], vertex[1], vertex[2]);
        e.mem.set_u32(data + DATA_VERTICES, vertices);
        (Ptr::new(this), Ptr::new(data), info)
    }

    #[test]
    fn load_binary_reads_the_field_then_the_direction() {
        let mut e = Engine::new();
        e.register(FIELD_LOAD_BINARY, |_, _| Ret::default());
        e.register(COLOR_LOAD_BINARY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c2_67d0, &args![0x4000u32, 0x5000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (FIELD_LOAD_BINARY, vec![0x4000, 0x5000]));
        assert_eq!(log[2], (COLOR_LOAD_BINARY, vec![0x4030, 0x5000]));
    }

    #[test]
    fn vortex_direction_crosses_unless_the_point_is_on_the_axis() {
        let mut e = engine();
        let center = point(&mut e, 0.0, 0.0, 0.0);
        let axis = point(&mut e, 0.0, 0.0, 1.0);
        let result = point(&mut e, 9.0, 9.0, 9.0);
        let on_axis = point(&mut e, 0.0, 0.0, 5.0);
        e.call_log = Some(vec![]);
        let answer = e
            .call(
                0x00c2_6820,
                &args![0x1234u32, result, center, axis, on_axis],
            )
            .u32();
        assert_eq!(answer, result);
        assert_eq!(read(&e, result), [0.0, 0.0, 0.0]);
        assert!(calls_to(&e, POINT_UNIT_CROSS).is_empty());
        let off_axis = point(&mut e, 2.0, 0.0, 5.0);
        e.call(
            0x00c2_6820,
            &args![0x1234u32, result, center, axis, off_axis],
        );
        assert_eq!(read(&e, result), [0.0, 1.0, 0.0]);
        let cross = calls_to(&e, POINT_UNIT_CROSS);
        assert_eq!(cross.len(), 1);
        assert_eq!(cross[0][..2], [axis, result]);
        assert_eq!(read(&e, cross[0][2]), [2.0, 0.0, 0.0]);
    }

    #[test]
    fn create_clone_constructs_and_copies_the_direction() {
        let mut e = Engine::new();
        e.map(POINT_ZERO, 12);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FIELD_CONSTRUCTOR, |_, _| Ret::default());
        e.register(FIELD_COPY_MEMBERS, |_, _| Ret::default());
        let this = point(&mut e, 0.0, 0.0, 0.0);
        let this = this - DIRECTION;
        e.mem.set_u32(this + DIRECTION, 0x7f80_0001);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c2_6940, &args![this, 0x7000u32]).u32();
        assert_eq!(e.mem.u32(clone), VORTEX_VTABLE);
        assert_eq!(e.mem.u32(clone + DIRECTION), 0x7f80_0001);
        assert_eq!(
            calls_to(&e, FIELD_COPY_MEMBERS),
            vec![vec![this, clone, 0x7000]]
        );
    }

    #[test]
    fn fn_00c269b0_pushes_particles_within_the_squared_maximum() {
        for (max, pushed) in [(2.0f32, true), (0.5, false)] {
            let mut e = engine();
            let (this, data, info) = setup(&mut e, [1.0, 0.0, 0.0]);
            e.mem.set_f32(this.addr() + MAX_DISTANCE, max);
            e.call(0x00c2_69b0, &args![this, 2.0f32, data]);
            let expected = if pushed { [0.0, 6.0, 0.0] } else { [0.0; 3] };
            assert_eq!(read(&e, info), expected);
        }
    }

    #[test]
    fn fn_00c26c00_pushes_every_particle_with_time_passed() {
        let mut e = engine();
        let (this, data, info) = setup(&mut e, [1.0, 0.0, 0.0]);
        e.mem.set_f32(info, 1.0);
        e.call(0x00c2_6c00, &args![this, 2.0f32, data]);
        assert_eq!(read(&e, info), [1.0, 6.0, 0.0]);
        // No time passed since the last update: untouched.
        e.mem.set_f32(info + 0x14, 2.0);
        e.call(0x00c2_6c00, &args![this, 2.0f32, data]);
        assert_eq!(read(&e, info), [1.0, 6.0, 0.0]);
    }

    #[test]
    fn update_skips_hands_over_or_attenuates() {
        let mut e = engine();
        let (this, data, info) = setup(&mut e, [2.0, 0.0, 0.0]);
        // Magnitude 0: nothing at all.
        e.mem.set_f32(this.addr() + MAGNITUDE, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x00c2_6de0, &args![this, 2.0f32, data]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.mem.set_f32(this.addr() + MAGNITUDE, 3.0);
        // No attenuation, no maximum: `00c26c00` (direct, not logged).
        e.call(0x00c2_6de0, &args![this, 2.0f32, data]);
        assert_eq!(read(&e, info), [0.0, 6.0, 0.0]);
        assert!(calls_to(&e, POINT_LENGTH).is_empty());
        // Attenuation 1: the push is divided by the distance (2).
        e.mem.set_f32(info + 4, 0.0);
        e.mem.set_f32(this.addr() + ATTENUATION, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x00c2_6de0, &args![this, 2.0f32, data]);
        assert_eq!(read(&e, info), [0.0, 3.0, 0.0]);
        assert_eq!(calls_to(&e, CI_POW).len(), 1);
        assert_eq!(calls_to(&e, POINT_ADD)[0][0], info);
        // A maximum distance below the particle's: no push.
        e.mem.set_u8(this.addr() + USE_MAX_DISTANCE, 1);
        e.mem.set_f32(this.addr() + MAX_DISTANCE, 1.5);
        e.call(0x00c2_6de0, &args![this, 2.0f32, data]);
        assert_eq!(read(&e, info), [0.0, 3.0, 0.0]);
        // A zero direction: nothing.
        e.mem.set_f32(this.addr() + DIRECTION + 8, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x00c2_6de0, &args![this, 2.0f32, data]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn get_viewer_strings_appends_the_name_and_the_direction() {
        let mut e = Engine::new();
        e.map(VORTEX_RTTI, 4);
        e.set_global(VORTEX_RTTI, 0x0555_0000u32);
        e.register(FIELD_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0x7000_0001));
        e.register(POINT_GET_VIEWER_STRING, |_, _| word(0x7000_0002));
        e.register(STRINGS_GROW, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        let stored = Rc::new(RefCell::new(vec![]));
        let seen = stored.clone();
        e.register_double(STRINGS_SET_AT, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[2]));
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xe, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c2_7060, &args![0x4000u32, strings]);
        assert_eq!(*stored.borrow(), vec![0x7000_0001, 0x7000_0002]);
        assert_eq!(
            calls_to(&e, POINT_GET_VIEWER_STRING),
            vec![vec![0x4030, DIRECTION_NAME]]
        );
    }
}
