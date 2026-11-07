//! Bound and plane tests: which side of a plane a bound is on, a point's distance from a plane, and
//! the merge of two bounds.

use super::layout::{NiBound, NiPlane, NiPoint3};
use super::{copy_float, fld, fld_local, fstp, m32, m32_local, FAMILY};
use engine_core::ptr::Ptr;
use engine_core::typed::leaf;
use engine_core::x87::{x87_add, x87_div, x87_mul, x87_sub};
use engine_core::{args, typed_call, Engine};
use std::cmp::Ordering;

/// `_CIsqrt` (`00ec6040`): the value in `ST0`, its root in `ST0`; called with the value as a leading
/// `f64` word pair.
const SQUARE_ROOT: u32 = 0x00ec_6040;
/// The float `Merge` compares the distance of the two centres with (`011ab3c0`, 1e-6).
const MERGE_THRESHOLD: u32 = 0x011a_b3c0;
/// The `double` 0.5 at `01011588`, a memory operand of `FMUL`.
const HALF: u32 = 0x0101_1588;

// Typed from 004b6160
/// `NiPlane::Distance` (`thiscall(plane, point)`): the dot product of the normal with the point
/// (`004b6190`, a float in `ST0`), minus the constant as a memory operand, stored to a float local
/// and loaded back.
pub fn plane_distance(e: &mut Engine, this: Ptr<NiPlane>, point: Ptr<NiPoint3>) -> f32 {
    let dot = typed_call!(leaf::FAMILY, 0x004b_6190, leaf::point3_dot(e, this, point));
    fstp(x87_sub(fld_local(dot), m32(e, this, NiPlane::m_fConstant)))
}

// Typed from 004b6100
/// `NiBound::WhichSide` (Xbox PDB, `thiscall(bound, plane)`): the centre's distance from the plane
/// (`004b6160`, stored to a float) against the radius: 2 when the distance is at most minus the
/// radius, else 1 when it is at least the radius, else 0, also when either is a NaN (each `FCOMPP`
/// with `TEST AH,0x41; JP` is taken on greater and on unordered).
pub fn which_side(e: &mut Engine, this: Ptr<NiBound>, plane: Ptr<NiPlane>) -> u32 {
    let distance = typed_call!(
        FAMILY,
        0x004b_6160,
        plane_distance(e, plane, this.at(NiBound::m_kCenter))
    );
    let distance = fld_local(distance);
    // FLD radius; FCHS; FLD distance; FCOMPP: distance against -radius.
    let negative_radius = -fld(e, this, NiBound::m_fRadius);
    if distance <= negative_radius {
        return 2;
    }
    // FLD distance; FLD radius; FCOMPP: radius against distance.
    if fld(e, this, NiBound::m_fRadius) <= distance {
        1
    } else {
        0
    }
}

/// `FSTP dword` then `FLD dword` of the same local.
#[inline(always)]
fn through_float(value: f64) -> f64 {
    fld_local(fstp(value))
}

// Typed from 00a7f3f0
/// `NiBound::Merge` (Xbox PDB, `thiscall(this, other)`): makes `this` the bound of both. With
/// `d = this.center - other.center` (each axis stored to a float), `dist2 = (dx*dx + dy*dy) + dz*dz`
/// and `dr = other.radius - this.radius` (both stored), `dr2 = dr*dr` (stored):
///
/// - `0 <= dr` and `dist2 <= dr2`: `other` holds `this`; its centre is copied as words and its
///   radius through the x87 stack;
/// - `dr < 0` or a NaN, and `dist2 <= dr2` or unordered: `this` holds `other`, nothing changes;
/// - otherwise `dist = sqrt(dist2)` (`00ec6040`, stored to a float twice); when 1e-6 is below it the
///   centre moves to `other.center + t * d`, `t = (dist - dr) / (dist + dist)`; the radius becomes
///   `(dist + other.radius + this.radius) * 0.5`.
pub fn merge(e: &mut Engine, this: Ptr<NiBound>, other: Ptr<NiBound>) {
    let (center, other_center) = (this.at(NiBound::m_kCenter), other.at(NiBound::m_kCenter));
    let dx = fstp(x87_sub(
        fld(e, center, NiPoint3::x),
        m32(e, other_center, NiPoint3::x),
    ));
    let dy = fstp(x87_sub(
        fld(e, center, NiPoint3::y),
        m32(e, other_center, NiPoint3::y),
    ));
    let dz = fstp(x87_sub(
        fld(e, center, NiPoint3::z),
        m32(e, other_center, NiPoint3::z),
    ));
    // FLD dx; FLD dy; FLD dz; FLD ST1; FMULP ST2; FLD ST2; FMULP ST3; FXCH; FADDP ST2,ST0;
    // FMUL ST0; FADDP.
    let (x, y, z) = (fld_local(dx), fld_local(dy), fld_local(dz));
    let yy = x87_mul(y, y);
    let xx = x87_mul(x, x);
    let dist2 = fstp(x87_add(x87_add(xx, yy), x87_mul(z, z)));
    let dr = fstp(x87_sub(
        fld(e, other, NiBound::m_fRadius),
        m32(e, this, NiBound::m_fRadius),
    ));
    let dr_value = fld_local(dr);
    let dr2 = fstp(x87_mul(dr_value, dr_value));
    let (dist2_value, dr2_value) = (fld_local(dist2), fld_local(dr2));
    // FLDZ; FCOMPP (0 against dr); TEST AH,0x41; JP: taken when dr < 0 or unordered.
    if 0.0 <= dr_value {
        // FCOM (dist2 against dr2); TEST AH,0x41; JP: taken when greater or unordered.
        if dist2_value <= dr2_value {
            copy_float(e, other_center, NiPoint3::x, center, NiPoint3::x);
            copy_float(e, other_center, NiPoint3::y, center, NiPoint3::y);
            copy_float(e, other_center, NiPoint3::z, center, NiPoint3::z);
            let radius = fstp(fld(e, other, NiBound::m_fRadius));
            e.set(this, NiBound::m_fRadius, radius);
            return;
        }
    } else if dist2_value.partial_cmp(&dr2_value) != Some(Ordering::Greater) {
        // TEST AH,0x41; JNZ: less, equal or unordered.
        return;
    }
    let root = e.call(SQUARE_ROOT, &args![dist2_value]).f64();
    let dist = through_float(through_float(root));
    // FLD threshold; FCOMP (against dist); TEST AH,5; JP: the centre moves when threshold < dist.
    let threshold = fld_local(e.global::<f32>(MERGE_THRESHOLD));
    if threshold < dist {
        let t = fstp(x87_div(x87_sub(dist, m32_local(dr)), x87_add(dist, dist)));
        let t = fld_local(t);
        let sx = fstp(x87_mul(fld_local(dx), t));
        let sy = fstp(x87_mul(fld_local(dy), t));
        let sz = fstp(x87_mul(t, m32_local(dz)));
        let nx = fstp(x87_add(fld(e, other_center, NiPoint3::x), m32_local(sx)));
        let ny = fstp(x87_add(fld_local(sy), m32(e, other_center, NiPoint3::y)));
        let nz = fstp(x87_add(fld(e, other_center, NiPoint3::z), m32_local(sz)));
        e.set(center, NiPoint3::x, nx);
        e.set(center, NiPoint3::y, ny);
        e.set(center, NiPoint3::z, nz);
    }
    let sum = x87_add(
        x87_add(dist, m32(e, other, NiBound::m_fRadius)),
        m32(e, this, NiBound::m_fRadius),
    );
    let half = f64::from_bits(e.global::<u64>(HALF));
    e.set(this, NiBound::m_fRadius, fstp(x87_mul(sum, half)));
}
