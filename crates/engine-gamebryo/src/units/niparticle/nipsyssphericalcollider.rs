//! `NiParticle/nipsyssphericalcollider.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c281f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphericalCollider::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_26a0.
pub fn ni_p_sys_spherical_collider_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_26a0
}

use crate::units::niparticle::nipsysmodifier::{stream_read, READ_LINK_ID, STREAM_VERSION};

/// `NiPSysCollider`'s fields (Xbox PDB names, PC offsets): `m_fBounce`
/// +0x08, `m_kCollisionPoint` +0x14, `m_fCollisionTime` +0x20,
/// `m_pkManager` +0x24, `m_spNext` +0x28; this class's `m_pkColliderObj`
/// +0x2C, `m_fRadius` +0x30, `m_fScaledRadius` +0x34, `m_fRadiusSq` +0x38,
/// `m_kPosition` +0x3C, `m_kLastColliderTransform` +0x48 and
/// `m_kLastPSysTransform` +0x7C (`NiTransform`s, 13 words each).
const BOUNCE: u32 = 0x08;
const COLLISION_POINT: u32 = 0x14;
const COLLISION_TIME: u32 = 0x20;
const MANAGER: u32 = 0x24;
const NEXT: u32 = 0x28;
const COLLIDER_OBJECT: u32 = 0x2c;
const RADIUS: u32 = 0x30;
const SCALED_RADIUS: u32 = 0x34;
const RADIUS_SQUARED: u32 = 0x38;
const POSITION: u32 = 0x3c;
const LAST_COLLIDER_TRANSFORM: u32 = 0x48;
const LAST_SYSTEM_TRANSFORM: u32 = 0x7c;
/// An `NiAVObject`'s world transform (+0x68) and `m_kName` (+8, the fixed
/// string's handle); the collider manager's particle system (+0x10).
const WORLD: u32 = 0x68;
const OBJECT_NAME: u32 = 0x08;
const MANAGER_SYSTEM: u32 = 0x10;
/// `NiPSysData`: vertices (+0x20) and particle infos (+0x70, 0x1C each,
/// the velocity first).
const DATA_VERTICES: u32 = 0x20;
const DATA_INFO: u32 = 0x70;
/// The class's vtable (RTTI `.?AVNiPSysSphericalCollider@@`), its
/// `NiRTTI` record, `NiPoint3::ZERO` (`011f426c`), the identity matrix
/// (`011f4248`), -FLT_MAX (`01015f5c`) and the resolve tolerance 0.001
/// (`010be824`).
const SPHERICAL_COLLIDER_VTABLE: u32 = 0x010b_e834;
const SPHERICAL_COLLIDER_RTTI: u32 = 0x0120_26a0;
const POINT_ZERO: u32 = 0x011f_426c;
const IDENTITY_MATRIX: u32 = 0x011f_4248;
const NO_VALUE_FLOAT: u32 = 0x0101_5f5c;
const TOLERANCE: u32 = 0x010b_e824;
/// `NiPSysCollider`'s constructor (`00c32780`, `RET 0x10`), member copy
/// (`00c32510`, `(clone, process)`), `Update` (`00c32490`, `RET 0xC`),
/// `Resolve` (`00c32450`, `RET 0x10`), `IsEqual` (`00c326b0`),
/// `LoadBinary` (`00c32840`), `SaveBinary` (`00c32900`) and
/// `GetViewerStrings` (`00c329c0`), all `thiscall`.
const COLLIDER_CONSTRUCTOR: u32 = 0x00c3_2780;
const COLLIDER_COPY_MEMBERS: u32 = 0x00c3_2510;
const COLLIDER_UPDATE: u32 = 0x00c3_2490;
const COLLIDER_RESOLVE: u32 = 0x00c3_2450;
const COLLIDER_IS_EQUAL: u32 = 0x00c3_26b0;
const COLLIDER_LOAD_BINARY: u32 = 0x00c3_2840;
const COLLIDER_SAVE_BINARY: u32 = 0x00c3_2900;
const COLLIDER_GET_VIEWER_STRINGS: u32 = 0x00c3_29c0;
/// `00aa13e0` (`cdecl`, one word: the size) and the sized delete
/// `00aa1460` (`cdecl`, `(block, size)`); `00a5d3d0` (unnamed,
/// `NiMain/niobject.cpp`, `thiscall`): the base destructor body.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
const OBJECT_DESTRUCTOR: u32 = 0x00a5_d3d0;
/// Imported `InterlockedDecrement` (import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The square-root helper (`00ec6040`, `ST0` in and out).
const SQRT: u32 = 0x00ec_6040;
/// `NiTransform::operator!=` (`00a86c30`, `thiscall`, one word: answers
/// nonzero when the transforms differ), `NiTransform::MakeIdentity`
/// (`00a86bf0`, `thiscall`), `NiTransform::Invert` (`004b4880`, `thiscall`,
/// the result) and the transform product (`0062c250`, `thiscall`,
/// `(result, other)`).
const TRANSFORM_DIFFERS: u32 = 0x00a8_6c30;
const TRANSFORM_MAKE_IDENTITY: u32 = 0x00a8_6bf0;
const TRANSFORM_INVERT: u32 = 0x004b_4880;
const TRANSFORM_TIMES_TRANSFORM: u32 = 0x0062_c250;
/// The collider object's `IsEqual` slot (+0x5C) and the next collider's
/// `SetResolveConstants` slot (+0x94); the stream's `SaveLinkID` slot
/// (+0x30).
const OBJECT_IS_EQUAL_SLOT: u32 = 0x5c;
const SET_RESOLVE_CONSTANTS_SLOT: u32 = 0x94;
const SAVE_LINK_ID_SLOT: u32 = 0x30;
/// The first file version with the radius stored (`CMP [stream+0xd8],
/// 0x0a000113`) and `NiStream`'s output stream (+0x250).
const VERSION_RADIUS: u32 = 0x0a00_0113;
const STREAM_OUTPUT: u32 = 0x250;
/// `NiGetViewerString` (`00a7d330`), its `const char*` (`00a7d5e0`) and
/// `float` (`00a7d550`) overloads, the viewer-string array's grow
/// (`0096ad30`) and store (`0096ae90`), and the names "Collider Object"
/// (`010be8cc`), "None" (`0102909c`) and "Radius" (`01015a9c`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
const NI_GET_VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
const COLLIDER_OBJECT_NAME: u32 = 0x010b_e8cc;
const NONE_NAME: u32 = 0x0102_909c;
const RADIUS_NAME: u32 = 0x0101_5a9c;

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

// Translated from 00c27ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphericalCollider::IsEqual` (Xbox PDB): the collider comparison
/// (`00c326b0`); then both collider objects null, or both set and equal
/// (the object's `IsEqual`, slot +0x5C); then the radii (`FUCOMPP`: NaN
/// unequal).
pub fn ni_p_sys_spherical_collider_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(COLLIDER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let mine = e.mem.u32(this.addr() + COLLIDER_OBJECT);
    let theirs = e.mem.u32(other.addr() + COLLIDER_OBJECT);
    if (mine != 0) != (theirs != 0) {
        return false;
    }
    if mine != 0 && theirs != 0 && e.vcall(mine, OBJECT_IS_EQUAL_SLOT, &args![theirs]).u8() == 0 {
        return false;
    }
    e.mem.f32(this.addr() + RADIUS) == e.mem.f32(other.addr() + RADIUS)
}

// Translated from 00c27b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphericalCollider::Resolve` (Xbox PDB): `thiscall`, `RET 0x10`
/// (the time, the address of the earliest collision time, the particle
/// data, the particle's index in the low 16 bits). With `d` the particle's
/// offset from the sphere's centre and `e = |d|^2 - radius^2`: a particle
/// inside (`-e` above 0.001) or outside (`e` above 0.001) and moving
/// towards the sphere (`-(v . d)` above 0) is traced along its velocity to
/// the sphere's surface (`b = -(v . d) / |v|^2`, the closest approach `c`;
/// outside it must pass within the radius); the hit time `b + sqrt((r^2 -
/// |c - centre|^2) / |v|^2)` inside, or `b - sqrt(..)` outside (the latter
/// through a `double`), when before the earliest collision so far (`*time -
/// time`), becomes the collision (`*earliest`, `m_fCollisionTime` and
/// `m_kCollisionPoint`). Then the base `Resolve` (`00c32450`); its nonzero
/// answer is passed on, else this collider when it hit, else 0. x87 in the
/// exe's order; the locals the exe keeps in its argument slots are not
/// observable.
pub fn ni_p_sys_spherical_collider_resolve(
    e: &mut Engine,
    this: Ptr,
    time: u32,
    earliest: Ptr,
    data: Ptr,
    index: u32,
) -> u32 {
    let t = this.addr();
    let i = index & 0xffff;
    let vertex = e.mem.u32(data.addr() + DATA_VERTICES).wrapping_add(i * 12);
    let info = e.mem.u32(data.addr() + DATA_INFO).wrapping_add(i * 0x1c);
    let ld = |e: &Engine, a: u32| x87_load32(e.mem.u32(a));
    let m = |e: &Engine, a: u32| x87_m32(e.mem.u32(a));
    let mut hit = false;
    let now = x87_load32(time);
    let d0 = x87_load32(x87_store32(x87_sub(ld(e, vertex), m(e, t + POSITION))));
    let d1 = x87_load32(x87_store32(x87_sub(
        ld(e, vertex + 4),
        m(e, t + POSITION + 4),
    )));
    let d2 = x87_load32(x87_store32(x87_sub(
        ld(e, vertex + 8),
        m(e, t + POSITION + 8),
    )));
    let squared = x87_add(x87_mul(d2, d2), x87_add(x87_mul(d0, d0), x87_mul(d1, d1)));
    let squared = x87_load32(x87_store32(squared));
    let excess = x87_store32(x87_sub(squared, m(e, t + RADIUS_SQUARED)));
    let excess = x87_load32(excess);
    let tolerance = ld(e, TOLERANCE);
    let inside = tolerance < -excess;
    'resolve: {
        if !inside && !(excess > tolerance) {
            break 'resolve;
        }
        let (v0, v1, v2) = (ld(e, info), ld(e, info + 4), ld(e, info + 8));
        let along = x87_add(x87_mul(d2, v2), x87_add(x87_mul(d0, v0), x87_mul(d1, v1)));
        let approach = x87_load32(x87_store32(-x87_load32(x87_store32(along))));
        if !inside && !(0.0 < approach) {
            break 'resolve;
        }
        let speed = x87_add(x87_mul(v2, v2), x87_add(x87_mul(v0, v0), x87_mul(v1, v1)));
        let speed = x87_load32(x87_store32(speed));
        let k = x87_store32(x87_div(approach, speed));
        let kf = x87_load32(k);
        let s0 = x87_store32(x87_mul(kf, x87_m32(e.mem.u32(info))));
        let s1 = x87_store32(x87_mul(v1, kf));
        let s2 = x87_store32(x87_mul(v2, kf));
        let p0 = x87_store32(x87_add(ld(e, vertex), x87_m32(s0)));
        let p1 = x87_store32(x87_add(x87_load32(s1), m(e, vertex + 4)));
        let p2 = x87_store32(x87_add(ld(e, vertex + 8), x87_m32(s2)));
        let c0 = x87_load32(x87_store32(x87_sub(x87_load32(p0), m(e, t + POSITION))));
        let c1 = x87_load32(x87_store32(x87_sub(x87_load32(p1), m(e, t + POSITION + 4))));
        let c2 = x87_load32(x87_store32(x87_sub(x87_load32(p2), m(e, t + POSITION + 8))));
        let closest = x87_add(x87_add(x87_mul(c1, c1), x87_mul(c0, c0)), x87_mul(c2, c2));
        let closest = x87_load32(x87_store32(closest));
        if !inside && !(ld(e, t + RADIUS_SQUARED) > closest) {
            break 'resolve;
        }
        let h = x87_store32(x87_sub(ld(e, t + RADIUS_SQUARED), closest));
        let hit_time = if inside {
            let q = x87_store32(x87_div(x87_m32(h), speed));
            let root = e.call(SQRT, &args![x87_load32(q)]).f64();
            x87_add(x87_load32(x87_store32(root)), x87_m32(k))
        } else {
            let k_double = kf;
            let q = x87_store32(x87_div(x87_m32(h), speed));
            let root = e.call(SQRT, &args![x87_load32(q)]).f64();
            x87_sub(k_double, x87_load32(x87_store32(root)))
        };
        let r = x87_load32(x87_store32(hit_time));
        let current = ld(e, earliest.addr());
        let window = x87_sub(current, now);
        if !(window > r) {
            break 'resolve;
        }
        hit = true;
        let at = x87_store32(x87_add(r, now));
        e.mem.set_u32(earliest.addr(), at);
        e.mem.set_u32(t + COLLISION_TIME, at);
        let q0 = x87_store32(x87_mul(r, m(e, info)));
        let q1 = x87_store32(x87_mul(ld(e, info + 4), r));
        let q2 = x87_store32(x87_mul(r, ld(e, info + 8)));
        let x = x87_store32(x87_add(ld(e, vertex), x87_m32(q0)));
        let y = x87_store32(x87_add(x87_load32(q1), m(e, vertex + 4)));
        let z = x87_store32(x87_add(ld(e, vertex + 8), x87_m32(q2)));
        e.mem.set_u32(t + COLLISION_POINT, x);
        e.mem.set_u32(t + COLLISION_POINT + 4, y);
        e.mem.set_u32(t + COLLISION_POINT + 8, z);
    }
    let base = e
        .call(
            COLLIDER_RESOLVE,
            &args![this, x87_store32(now), earliest, data, index],
        )
        .u32();
    if base != 0 {
        base
    } else if hit {
        t
    } else {
        0
    }
}

// Translated from 00c27e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphericalCollider::Update` (Xbox PDB): `thiscall`, `RET 0xC` (the
/// time, the particle data, the particle's index in the low 16 bits). The
/// normal at the collision point (`(point - centre) / scaledRadius`), the
/// particle's velocity reflected about it (`v - 2 (v . n) n`, each
/// component stored), then scaled by the bounce; then the base `Update`
/// (`00c32490`) with the same words.
pub fn ni_p_sys_spherical_collider_update(
    e: &mut Engine,
    this: Ptr,
    time: u32,
    data: Ptr,
    index: u32,
) {
    let t = this.addr();
    let ld = |e: &Engine, a: u32| x87_load32(e.mem.u32(a));
    let m = |e: &Engine, a: u32| x87_m32(e.mem.u32(a));
    let d0 = x87_store32(x87_sub(ld(e, t + COLLISION_POINT), m(e, t + POSITION)));
    let d1 = x87_store32(x87_sub(
        ld(e, t + COLLISION_POINT + 4),
        m(e, t + POSITION + 4),
    ));
    let d2 = x87_store32(x87_sub(
        ld(e, t + COLLISION_POINT + 8),
        m(e, t + POSITION + 8),
    ));
    let info = e
        .mem
        .u32(data.addr() + DATA_INFO)
        .wrapping_add((index & 0xffff) * 0x1c);
    let inverse = x87_load32(x87_store32(x87_div(1.0, ld(e, t + SCALED_RADIUS))));
    let n0 = x87_load32(x87_store32(x87_mul(inverse, x87_m32(d0))));
    let n1 = x87_load32(x87_store32(x87_mul(x87_load32(d1), inverse)));
    let n2 = x87_load32(x87_store32(x87_mul(inverse, x87_m32(d2))));
    let (v0, v1, v2) = (ld(e, info), ld(e, info + 4), ld(e, info + 8));
    let dot = x87_add(x87_mul(v2, n2), x87_add(x87_mul(v1, n1), x87_mul(v0, n0)));
    let dot = x87_load32(x87_store32(dot));
    let p = [
        x87_load32(x87_store32(x87_mul(n0, dot))),
        x87_load32(x87_store32(x87_mul(n1, dot))),
        x87_load32(x87_store32(x87_mul(n2, dot))),
    ];
    let mut reflected = [0.0f64; 3];
    for k in 0..3 {
        let twice = x87_store32(x87_add(p[k], p[k]));
        let address = info + 4 * k as u32;
        let value = x87_load32(x87_store32(x87_sub(ld(e, address), x87_m32(twice))));
        e.mem.set_u32(address, x87_store32(value));
        reflected[k] = value;
    }
    let bounce = x87_load32(x87_quiet_bits(e.mem.u32(t + BOUNCE)));
    e.mem
        .set_u32(info, x87_store32(x87_mul(reflected[0], bounce)));
    e.mem
        .set_u32(info + 4, x87_store32(x87_mul(reflected[1], bounce)));
    e.mem
        .set_u32(info + 8, x87_store32(x87_mul(bounce, reflected[2])));
    let time = x87_quiet_bits(time);
    e.call(COLLIDER_UPDATE, &args![this, time, data, index]);
}

// Translated from 00c27f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphericalCollider::SetResolveConstants` (Xbox PDB): when the
/// collider object's world transform (`00a86c30` with the object's +0x68,
/// even for a null object) or the particle system's (through the manager)
/// differs from the one kept, the collider's transform in the system's space
/// is rebuilt (the object's world, or the identity without an object; the
/// inverse of the system's world times it): its translation is the centre,
/// `radius * scale` the scaled radius and its square `m_fRadiusSq`, and
/// both world transforms are kept. Ends in a tail jump to the next
/// collider's `SetResolveConstants` (slot +0x94) when there is one. Locals
/// sit at the exe's frame offsets.
pub fn ni_p_sys_spherical_collider_set_resolve_constants(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    let object = e.mem.u32(t + COLLIDER_OBJECT);
    let mut changed = e
        .call(
            TRANSFORM_DIFFERS,
            &args![t + LAST_COLLIDER_TRANSFORM, object.wrapping_add(WORLD)],
        )
        .u8()
        != 0;
    if !changed {
        let system = e.mem.u32(e.mem.u32(t + MANAGER) + MANAGER_SYSTEM);
        changed = e
            .call(
                TRANSFORM_DIFFERS,
                &args![t + LAST_SYSTEM_TRANSFORM, system.wrapping_add(WORLD)],
            )
            .u8()
            != 0;
    }
    if changed {
        e.with_stack(0xe4, |e, frame| {
            let f = frame.addr();
            let object = e.mem.u32(t + COLLIDER_OBJECT);
            if object != 0 {
                copy_words(e, f + 0x48, object + WORLD, 13);
            } else {
                e.call(TRANSFORM_MAKE_IDENTITY, &args![f + 0x48]);
            }
            let system = e.mem.u32(e.mem.u32(t + MANAGER) + MANAGER_SYSTEM);
            copy_words(e, f + 0x7c, system + WORLD, 13);
            e.call(TRANSFORM_INVERT, &args![f + 0x7c, f + 0xb0]);
            e.call(
                TRANSFORM_TIMES_TRANSFORM,
                &args![f + 0xb0, f + 0x14, f + 0x48],
            );
            let scaled = x87_mul(
                x87_load32(e.mem.u32(t + RADIUS)),
                x87_m32(e.mem.u32(f + 0x44)),
            );
            copy_words(e, t + POSITION, f + 0x38, 3);
            let scaled = x87_load32(x87_store32(scaled));
            e.mem.set_u32(t + SCALED_RADIUS, x87_store32(scaled));
            copy_words(e, t + LAST_SYSTEM_TRANSFORM, f + 0x7c, 13);
            e.mem
                .set_u32(t + RADIUS_SQUARED, x87_store32(x87_mul(scaled, scaled)));
            copy_words(e, t + LAST_COLLIDER_TRANSFORM, f + 0x48, 13);
        });
    }
    let next = e.mem.u32(t + NEXT);
    if next != 0 {
        e.vcall(next, SET_RESOLVE_CONSTANTS_SLOT, &args![]);
    }
}

// Translated from 00c28070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphericalCollider::LoadBinary` (Xbox PDB): the collider part
/// (`00c32840`), the collider object's link id (`00a645b0`), and from file
/// version 10.0.1.19 the radius (four bytes; the count word is the
/// argument slot).
pub fn ni_p_sys_spherical_collider_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(COLLIDER_LOAD_BINARY, &args![this, stream]);
    e.call(READ_LINK_ID, &args![stream]);
    if e.mem.u32(stream.addr() + STREAM_VERSION) >= VERSION_RADIUS {
        e.with_stack(4, |e, slot| {
            stream_read(e, stream.addr(), this.addr() + RADIUS, 4, 4, slot.addr());
        });
    }
}

// Translated from 00c280c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphericalCollider::SaveBinary` (Xbox PDB): the collider part
/// (`00c32900`), the collider object's link id (the stream's slot +0x30),
/// then the radius through the stream's output (its write function at
/// +0xC; the count word is the argument slot).
pub fn ni_p_sys_spherical_collider_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(COLLIDER_SAVE_BINARY, &args![this, stream]);
    let object = e.mem.u32(this.addr() + COLLIDER_OBJECT);
    e.vcall(stream.addr(), SAVE_LINK_ID_SLOT, &args![object]);
    let output = e.mem.u32(stream.addr() + STREAM_OUTPUT);
    let write = e.mem.u32(output + 0xc);
    e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 4);
        let put = e
            .call(
                write,
                &args![output, this.addr() + RADIUS, 4u32, count, 1u32],
            )
            .u32();
        let position = e.mem.u32(output + 4);
        e.mem.set_u32(output + 4, position.wrapping_add(put));
    });
}

// Translated from 00c28110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphericalCollider::NiPSysSphericalCollider` (Xbox PDB):
/// `thiscall`, `RET 0x18` (bounce, spawn-on-collide, die-on-collide, spawn
/// modifier, collider object, radius). The collider constructor
/// (`00c32780` with the first four words, the bounce through
/// `FLD`/`FSTP`), the radius and its square 1.0, the collider object, the
/// vtable, the centre `NiPoint3::ZERO`; the radius argument replaces the
/// radius unless it is below 0 or a NaN (the square stays 1.0); both kept
/// transforms are the identity with every translation component and the
/// scale -FLT_MAX. Answers `this`.
pub fn ni_p_sys_spherical_collider_ni_p_sys_spherical_collider(
    e: &mut Engine,
    this: Ptr,
    bounce: u32,
    spawn_on_collide: u32,
    die_on_collide: u32,
    spawn_modifier: Ptr,
    collider_object: Ptr,
    radius: u32,
) -> Ptr {
    let t = this.addr();
    let bounce = x87_quiet_bits(bounce);
    e.call(
        COLLIDER_CONSTRUCTOR,
        &args![
            this,
            bounce,
            spawn_on_collide,
            die_on_collide,
            spawn_modifier
        ],
    );
    let one = 1.0f32.to_bits();
    e.mem.set_u32(t + RADIUS, one);
    e.mem.set_u32(t + RADIUS_SQUARED, one);
    e.mem.set_u32(t + COLLIDER_OBJECT, collider_object.addr());
    e.mem.set_u32(t, SPHERICAL_COLLIDER_VTABLE);
    copy_words(e, t + POSITION, POINT_ZERO, 3);
    let wanted = x87_load32(radius);
    if !(wanted < 0.0 || wanted.is_nan()) {
        e.mem.set_u32(t + RADIUS, x87_store32(wanted));
    }
    let none = x87_store32(x87_load32(e.global::<u32>(NO_VALUE_FLOAT)));
    for off in [0xa0, 0x6c, 0xa4, 0x70, 0xa8, 0x74] {
        e.mem.set_u32(t + off, none);
    }
    copy_words(e, t + LAST_SYSTEM_TRANSFORM, IDENTITY_MATRIX, 9);
    e.mem.set_u32(t + 0xac, none);
    e.mem.set_u32(t + 0x78, none);
    copy_words(e, t + LAST_COLLIDER_TRANSFORM, t + LAST_SYSTEM_TRANSFORM, 9);
    this
}

// Translated from 00c28200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphericalCollider::_scalar_deleting_destructor_` (Xbox PDB): the
/// next collider released (not cleared), the base destructor body
/// (`00a5d3d0`), then the sized delete of 0xB0 bytes with bit 0 of `flags`.
/// Answers `this`.
pub fn ni_p_sys_spherical_collider_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let next = e.mem.u32(this.addr() + NEXT);
    if next != 0 && e.call(INTERLOCKED_DECREMENT, &args![next + 4]).u32() == 0 {
        e.vcall(next, 4, &args![]);
    }
    e.call(OBJECT_DESTRUCTOR, &args![this]);
    if flags as u8 & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xb0u32]);
    }
    this
}

/// The inlined default construction: 0xB0 bytes constructed with bounce
/// and radius 1.0 and every other word 0 (null when the allocation fails).
fn new_default(e: &mut Engine) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![0xb0u32]).u32();
    if block == 0 {
        return 0;
    }
    let one = 1.0f32.to_bits();
    ni_p_sys_spherical_collider_ni_p_sys_spherical_collider(
        e,
        Ptr::new(block),
        one,
        0,
        0,
        Ptr::NULL,
        Ptr::NULL,
        one,
    )
    .addr()
}

// Translated from 00c28250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphericalCollider::CreateClone` (Xbox PDB): a default-constructed
/// collider (null when the allocation fails, then used anyway as the exe
/// does), the collider members copied (`00c32510`) and the radius copied
/// (`FLD`/`FSTP`). Answers the clone.
pub fn ni_p_sys_spherical_collider_create_clone(e: &mut Engine, this: Ptr, process: Ptr) -> Ptr {
    let clone = new_default(e);
    e.call(COLLIDER_COPY_MEMBERS, &args![this, clone, process]);
    let radius = x87_quiet_bits(e.mem.u32(this.addr() + RADIUS));
    e.mem.set_u32(clone.wrapping_add(RADIUS), radius);
    Ptr::new(clone)
}

// Translated from 00c282a0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The class's creation function (Ghidra made no function of it; `cdecl`,
/// no words): a default-constructed collider (bounce and radius 1.0), or
/// null when the allocation fails.
pub fn fn_00c282a0(e: &mut Engine) -> Ptr {
    Ptr::new(new_default(e))
}

// Translated from 00c282d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphericalCollider::GetViewerStrings` (Xbox PDB): the collider
/// strings (`00c329c0`), the class name, "Collider Object" with the
/// object's name (its fixed string at +8, or "None"), and "Radius" with the
/// radius (`FLD`/`FSTP`).
pub fn ni_p_sys_spherical_collider_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(COLLIDER_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(SPHERICAL_COLLIDER_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    let object = e.mem.u32(this.addr() + COLLIDER_OBJECT);
    let name = if object != 0 {
        e.mem.u32(object + OBJECT_NAME)
    } else {
        NONE_NAME
    };
    let text = e
        .call(
            NI_GET_VIEWER_STRING_TEXT,
            &args![COLLIDER_OBJECT_NAME, name],
        )
        .u32();
    push_viewer_string(e, strings, text);
    let radius = x87_quiet_bits(e.mem.u32(this.addr() + RADIUS));
    let text = e
        .call(NI_GET_VIEWER_STRING_FLOAT, &args![RADIUS_NAME, radius])
        .u32();
    push_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c281f0, ni_p_sys_spherical_collider_get_r_t_t_i() -> u32),
        entry!(0x00c2_7ae0, ni_p_sys_spherical_collider_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00c2_7b40,
            ni_p_sys_spherical_collider_resolve(Ptr, u32, Ptr, Ptr, u32) -> u32
        ),
        entry!(
            0x00c2_7e20,
            ni_p_sys_spherical_collider_update(Ptr, u32, Ptr, u32)
        ),
        entry!(
            0x00c2_7f60,
            ni_p_sys_spherical_collider_set_resolve_constants(Ptr)
        ),
        entry!(
            0x00c2_8070,
            ni_p_sys_spherical_collider_load_binary(Ptr, Ptr)
        ),
        entry!(
            0x00c2_80c0,
            ni_p_sys_spherical_collider_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00c2_8110,
            ni_p_sys_spherical_collider_ni_p_sys_spherical_collider(
                Ptr,
                u32,
                u32,
                u32,
                Ptr,
                Ptr,
                u32,
            ) -> Ptr
        ),
        entry!(
            0x00c2_8200,
            ni_p_sys_spherical_collider_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00c2_8250, ni_p_sys_spherical_collider_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00c2_82a0, fn_00c282a0() -> Ptr),
        entry!(
            0x00c2_82d0,
            ni_p_sys_spherical_collider_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_spherical_collider_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_81f0, &args![]).u32(), 0x0120_26a0);
    }
}

#[cfg(test)]
mod n066_tests {
    use super::*;
    use crate::units::niparticle::nipsysmodifier::test_stream;
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

    fn set3(e: &mut Engine, p: u32, v: [f32; 3]) {
        for (i, x) in v.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, *x);
        }
    }

    fn read3(e: &Engine, p: u32) -> [f32; 3] {
        [e.mem.f32(p), e.mem.f32(p + 4), e.mem.f32(p + 8)]
    }

    /// A collider centred at the origin with radius 1 and data with one
    /// particle at `vertex` moving at `velocity`; answers (collider, data,
    /// info).
    fn setup(e: &mut Engine, vertex: [f32; 3], velocity: [f32; 3]) -> (u32, u32, u32) {
        let this = e.mem.alloc(0xb0);
        e.mem.set_f32(this + RADIUS_SQUARED, 1.0);
        e.mem.set_f32(this + SCALED_RADIUS, 1.0);
        let data = e.mem.alloc(0x7c);
        let vertices = e.mem.alloc(12);
        set3(e, vertices, vertex);
        let info = e.mem.alloc(0x1c);
        set3(e, info, velocity);
        e.mem.set_u32(data + DATA_VERTICES, vertices);
        e.mem.set_u32(data + DATA_INFO, info);
        (this, data, info)
    }

    fn resolve_engine() -> Engine {
        let mut e = Engine::new();
        e.map(TOLERANCE, 4);
        e.set_global(TOLERANCE, 0.001f32);
        e.register(SQRT, |_, a| Ret {
            st0: f64_arg(a, 0).sqrt(),
            ..Ret::default()
        });
        e.register(COLLIDER_RESOLVE, |_, _| word(0));
        e
    }

    #[test]
    fn is_equal_compares_the_objects_and_the_radii() {
        let mut e = Engine::new();
        e.register(COLLIDER_IS_EQUAL, |_, _| word(1));
        let this = e.mem.alloc(0xb0);
        let other = e.mem.alloc(0xb0);
        assert!(e.call(0x00c2_7ae0, &args![this, other]).bool());
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + OBJECT_IS_EQUAL_SLOT, 0x0100_0090);
        e.register(0x0100_0090, |_, a| word(u32::from(a[1] == 0x7700)));
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(this + COLLIDER_OBJECT, object);
        assert!(!e.call(0x00c2_7ae0, &args![this, other]).bool());
        e.mem.set_u32(other + COLLIDER_OBJECT, 0x7700);
        assert!(e.call(0x00c2_7ae0, &args![this, other]).bool());
        e.mem.set_u32(other + COLLIDER_OBJECT, 0x7704);
        assert!(!e.call(0x00c2_7ae0, &args![this, other]).bool());
        e.mem.set_u32(other + COLLIDER_OBJECT, 0x7700);
        e.mem.set_f32(other + RADIUS, 2.0);
        assert!(!e.call(0x00c2_7ae0, &args![this, other]).bool());
        e.register(COLLIDER_IS_EQUAL, |_, _| word(0));
        assert!(!e.call(0x00c2_7ae0, &args![this, this]).bool());
    }

    #[test]
    fn resolve_finds_the_hit_from_outside_and_from_inside() {
        for (vertex, velocity, hit_time) in [
            ([3.0f32, 0.0, 0.0], [-1.0f32, 0.0, 0.0], 2.0f32),
            ([0.5, 0.0, 0.0], [1.0, 0.0, 0.0], 0.5),
        ] {
            let mut e = resolve_engine();
            let (this, data, _) = setup(&mut e, vertex, velocity);
            let earliest = e.mem.alloc(4);
            e.mem.set_f32(earliest, 10.0);
            e.call_log = Some(vec![]);
            let answer = e
                .call(
                    0x00c2_7b40,
                    &args![this, 0.0f32, earliest, data, 0xabcd_0000u32],
                )
                .u32();
            assert_eq!(answer, this);
            assert_eq!(e.mem.f32(earliest), hit_time);
            assert_eq!(e.mem.f32(this + COLLISION_TIME), hit_time);
            assert_eq!(read3(&e, this + COLLISION_POINT), [1.0, 0.0, 0.0]);
            assert_eq!(
                calls_to(&e, COLLIDER_RESOLVE),
                vec![vec![this, 0, earliest, data, 0xabcd_0000]]
            );
        }
    }

    #[test]
    fn resolve_misses_and_passes_on_the_base_answer() {
        let mut e = resolve_engine();
        let (this, data, _) = setup(&mut e, [3.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let earliest = e.mem.alloc(4);
        e.mem.set_f32(earliest, 10.0);
        let answer = e
            .call(0x00c2_7b40, &args![this, 0.0f32, earliest, data, 0u32])
            .u32();
        assert_eq!(answer, 0);
        assert_eq!(e.mem.f32(earliest), 10.0);
        // A later hit than the earliest one is not taken either.
        let (this, data, _) = setup(&mut e, [3.0, 0.0, 0.0], [-1.0, 0.0, 0.0]);
        e.mem.set_f32(earliest, 1.5);
        assert_eq!(
            e.call(0x00c2_7b40, &args![this, 0.0f32, earliest, data, 0u32])
                .u32(),
            0
        );
        e.register(COLLIDER_RESOLVE, |_, _| word(0x5555));
        assert_eq!(
            e.call(0x00c2_7b40, &args![this, 0.0f32, earliest, data, 0u32])
                .u32(),
            0x5555
        );
    }

    #[test]
    fn update_reflects_and_bounces_the_velocity() {
        let mut e = Engine::new();
        e.register(COLLIDER_UPDATE, |_, _| Ret::default());
        let (this, data, info) = setup(&mut e, [0.0; 3], [-2.0, 1.0, 0.0]);
        set3(&mut e, this + COLLISION_POINT, [1.0, 0.0, 0.0]);
        e.mem.set_f32(this + BOUNCE, 0.5);
        e.call_log = Some(vec![]);
        e.call(
            0x00c2_7e20,
            &args![this, 0x7f80_0001u32, data, 0x0001_0000u32],
        );
        assert_eq!(read3(&e, info), [1.0, 0.5, 0.0]);
        assert_eq!(
            calls_to(&e, COLLIDER_UPDATE),
            vec![vec![this, 0x7fc0_0001, data, 0x0001_0000]]
        );
    }

    fn transform_engine(differs: u32) -> Engine {
        let mut e = Engine::new();
        e.register_double(TRANSFORM_DIFFERS, move |_, _| word(differs));
        e.register(TRANSFORM_MAKE_IDENTITY, |e, a| {
            for i in 0..13 {
                e.mem.set_u32(a[0] + 4 * i, 0);
            }
            Ret::default()
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
        e
    }

    #[test]
    fn set_resolve_constants_rebuilds_a_changed_transform() {
        let mut e = transform_engine(1);
        let this = e.mem.alloc(0xb0);
        e.mem.set_f32(this + RADIUS, 1.5);
        let object = e.mem.alloc(0x100);
        set3(&mut e, object + WORLD + 0x24, [1.0, 2.0, 3.0]);
        e.mem.set_f32(object + WORLD + 0x30, 2.0);
        e.mem.set_u32(this + COLLIDER_OBJECT, object);
        let system = e.mem.alloc(0x100);
        e.mem.set_u32(system + WORLD, 0x1234);
        let manager = e.mem.alloc(0x20);
        e.mem.set_u32(manager + MANAGER_SYSTEM, system);
        e.mem.set_u32(this + MANAGER, manager);
        e.call(0x00c2_7f60, &args![this]);
        assert_eq!(read3(&e, this + POSITION), [1.0, 2.0, 3.0]);
        assert_eq!(e.mem.f32(this + SCALED_RADIUS), 3.0);
        assert_eq!(e.mem.f32(this + RADIUS_SQUARED), 9.0);
        assert_eq!(e.mem.f32(this + LAST_COLLIDER_TRANSFORM + 0x30), 2.0);
        assert_eq!(e.mem.u32(this + LAST_SYSTEM_TRANSFORM), 0x1234);
        // No object: the identity (here all zeros) stands in for its world.
        e.mem.set_u32(this + COLLIDER_OBJECT, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c2_7f60, &args![this]);
        assert_eq!(calls_to(&e, TRANSFORM_MAKE_IDENTITY).len(), 1);
        assert_eq!(e.mem.f32(this + SCALED_RADIUS), 0.0);
    }

    #[test]
    fn set_resolve_constants_without_changes_goes_to_the_next_collider() {
        let mut e = transform_engine(0);
        e.register(0x0100_00a0, |_, _| Ret::default());
        let this = e.mem.alloc(0xb0);
        let manager = e.mem.alloc(0x20);
        let system = e.mem.alloc(0x100);
        e.mem.set_u32(manager + MANAGER_SYSTEM, system);
        e.mem.set_u32(this + MANAGER, manager);
        let vtable = e.mem.alloc(0x100);
        e.mem
            .set_u32(vtable + SET_RESOLVE_CONSTANTS_SLOT, 0x0100_00a0);
        let next = e.mem.alloc(8);
        e.mem.set_u32(next, vtable);
        e.mem.set_u32(this + NEXT, next);
        e.call_log = Some(vec![]);
        e.call(0x00c2_7f60, &args![this]);
        assert_eq!(calls_to(&e, TRANSFORM_DIFFERS).len(), 2);
        assert!(calls_to(&e, TRANSFORM_INVERT).is_empty());
        assert_eq!(calls_to(&e, 0x0100_00a0), vec![vec![next]]);
    }

    #[test]
    fn load_binary_reads_the_radius_from_version_10_0_1_19() {
        for (version, radius) in [(0x0a00_0113u32, 2.5f32), (0x0a00_0112, 0.0)] {
            let mut e = Engine::new();
            e.register(COLLIDER_LOAD_BINARY, |_, _| Ret::default());
            e.register(READ_LINK_ID, |_, _| Ret::default());
            let stream = test_stream::new(&mut e, version, &2.5f32.to_le_bytes());
            let this = e.mem.alloc(0xb0);
            e.call_log = Some(vec![]);
            e.call(0x00c2_8070, &args![this, stream]);
            assert_eq!(e.mem.f32(this + RADIUS), radius);
            assert_eq!(calls_to(&e, READ_LINK_ID), vec![vec![stream]]);
        }
    }

    #[test]
    fn save_binary_writes_the_link_and_the_radius() {
        const WRITE: u32 = 0x0100_00b0;
        const SAVE_LINK: u32 = 0x0100_00c0;
        let mut e = Engine::new();
        e.register(COLLIDER_SAVE_BINARY, |_, _| Ret::default());
        e.register(SAVE_LINK, |_, _| Ret::default());
        e.register(WRITE, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            word(a[2])
        });
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + SAVE_LINK_ID_SLOT, SAVE_LINK);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 0xc, WRITE);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream, vtable);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        let this = e.mem.alloc(0xb0);
        e.mem.set_u32(this + COLLIDER_OBJECT, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x00c2_80c0, &args![this, stream]);
        assert_eq!(calls_to(&e, SAVE_LINK), vec![vec![stream, 0x4444]]);
        assert_eq!(calls_to(&e, WRITE)[0][..3], [output, this + RADIUS, 4]);
        assert_eq!(e.mem.u32(output + 4), 4);
    }

    fn constructor_engine() -> Engine {
        let mut e = Engine::new();
        e.map(POINT_ZERO, 12);
        e.map(IDENTITY_MATRIX, 0x24);
        e.set_global(IDENTITY_MATRIX, 1.0f32);
        e.map(NO_VALUE_FLOAT, 4);
        e.set_global(NO_VALUE_FLOAT, 0xff7f_ffffu32);
        e.register(COLLIDER_CONSTRUCTOR, |_, _| Ret::default());
        e
    }

    #[test]
    fn constructor_takes_a_valid_radius_and_sets_the_kept_transforms() {
        for (radius, kept) in [(2.0f32, 2.0f32), (-1.0, 1.0)] {
            let mut e = constructor_engine();
            let this = e.mem.alloc(0xb0);
            e.call_log = Some(vec![]);
            let answer = e
                .call(
                    0x00c2_8110,
                    &args![
                        this,
                        0x7f80_0001u32,
                        1u32,
                        0u32,
                        0x5000u32,
                        0x6000u32,
                        radius
                    ],
                )
                .u32();
            assert_eq!(answer, this);
            assert_eq!(
                calls_to(&e, COLLIDER_CONSTRUCTOR),
                vec![vec![this, 0x7fc0_0001, 1, 0, 0x5000]]
            );
            assert_eq!(e.mem.f32(this + RADIUS), kept);
            assert_eq!(e.mem.f32(this + RADIUS_SQUARED), 1.0);
            assert_eq!(e.mem.u32(this + COLLIDER_OBJECT), 0x6000);
            assert_eq!(e.mem.u32(this), SPHERICAL_COLLIDER_VTABLE);
            assert_eq!(e.mem.f32(this + LAST_SYSTEM_TRANSFORM), 1.0);
            assert_eq!(e.mem.f32(this + LAST_COLLIDER_TRANSFORM), 1.0);
            for off in [0x6c, 0x70, 0x74, 0x78, 0xa0, 0xa4, 0xa8, 0xac] {
                assert_eq!(e.mem.u32(this + off), 0xff7f_ffff);
            }
        }
    }

    #[test]
    fn scalar_deleting_destructor_releases_the_next_collider() {
        let mut e = Engine::new();
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], v);
            word(v)
        });
        e.register(OBJECT_DESTRUCTOR, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        e.register(0x0100_00d0, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 4, 0x0100_00d0);
        let next = e.mem.alloc(8);
        e.mem.set_u32(next, vtable);
        e.mem.set_u32(next + 4, 1);
        let this = e.mem.alloc(0xb0);
        e.mem.set_u32(this + NEXT, next);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_8200, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x0100_00d0), vec![vec![next]]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this, 0xb0]]);
        assert_eq!(e.mem.u32(this + NEXT), next);
        e.mem.set_u32(this + NEXT, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c2_8200, &args![this, 2u32]);
        assert!(calls_to(&e, SIZED_DELETE).is_empty());
        assert_eq!(calls_to(&e, OBJECT_DESTRUCTOR), vec![vec![this]]);
    }

    #[test]
    fn create_clone_and_the_creation_function_construct_defaults() {
        let mut e = constructor_engine();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(COLLIDER_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0xb0);
        e.mem.set_f32(this + RADIUS, 4.0);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c2_8250, &args![this, 0x7000u32]).u32();
        assert_eq!(e.mem.f32(clone + RADIUS), 4.0);
        assert_eq!(e.mem.u32(clone), SPHERICAL_COLLIDER_VTABLE);
        let one = 1.0f32.to_bits();
        assert_eq!(
            calls_to(&e, COLLIDER_CONSTRUCTOR),
            vec![vec![clone, one, 0, 0, 0]]
        );
        assert_eq!(
            calls_to(&e, COLLIDER_COPY_MEMBERS),
            vec![vec![this, clone, 0x7000]]
        );
        let made = e.call(0x00c2_82a0, &args![]).u32();
        assert_eq!(e.mem.f32(made + RADIUS), 1.0);
        e.register(NI_ALLOCATE, |_, _| word(0));
        assert_eq!(e.call(0x00c2_82a0, &args![]).u32(), 0);
    }

    #[test]
    fn get_viewer_strings_names_the_object_or_none() {
        let mut e = Engine::new();
        e.map(SPHERICAL_COLLIDER_RTTI, 4);
        e.set_global(SPHERICAL_COLLIDER_RTTI, 0x0555_0000u32);
        e.register(COLLIDER_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0x7000_0001));
        e.register(NI_GET_VIEWER_STRING_TEXT, |_, a| word(a[1]));
        e.register(NI_GET_VIEWER_STRING_FLOAT, |_, a| word(a[1]));
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
        let this = e.mem.alloc(0xb0);
        e.mem.set_f32(this + RADIUS, 2.0);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xe, 1);
        e.call(0x00c2_82d0, &args![this, strings]);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + OBJECT_NAME, 0x3333);
        e.mem.set_u32(this + COLLIDER_OBJECT, object);
        e.call(0x00c2_82d0, &args![this, strings]);
        let two = 2.0f32.to_bits();
        assert_eq!(
            *stored.borrow(),
            vec![0x7000_0001, NONE_NAME, two, 0x7000_0001, 0x3333, two]
        );
    }
}
