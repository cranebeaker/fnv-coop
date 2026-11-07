//! The six culling planes of a camera's frustum (`NiFrustumPlanes::Set`): the planes the walk tests
//! every bound against, made each time a culling process or compound frustum takes a camera.

use super::layout::{NiFrustum, NiFrustumPlanes, NiMatrix3, NiPlane, NiPoint3, NiTransform};
use super::{fld, fld_local, fstp, m32, m32_local};
use engine_core::ptr::{Field, Ptr};
use engine_core::x87::{x87_add, x87_div, x87_mul};
use engine_core::{args, Engine};

/// `_CIsqrt` (`00ec6040`): the value in `ST0`, its root in `ST0`.
const SQUARE_ROOT: u32 = 0x00ec_6040;
/// The `double` 1.0 at `01012070`, a memory operand of `FADD`.
const ONE_DOUBLE: u32 = 0x0101_2070;

/// Three floats the exe keeps in stack slots.
type Floats = [f32; 3];

/// A column of the rotation, each entry through the x87 stack into a float slot.
#[inline(always)]
fn column(e: &Engine, matrix: Ptr<NiMatrix3>, index: u32) -> Floats {
    [0, 1, 2].map(|row| fstp(fld(e, matrix, NiMatrix3::entry(row, index))))
}

/// `column * factor`, each product stored. Every operand is a value already through a float slot
/// (quiet), so the exe's mix of stack and memory operands gives the same products.
#[inline(always)]
fn scaled(column: Floats, factor: f32) -> Floats {
    column.map(|c| fstp(x87_mul(fld_local(c), fld_local(factor))))
}

/// `first + second` per component, `first` loaded and `second` a memory operand, stored.
#[inline(always)]
fn sum(first: Floats, second: Floats) -> Floats {
    [0, 1, 2].map(|i| fstp(x87_add(fld_local(first[i]), m32_local(second[i]))))
}

/// The column negated (`FCHS` of the loaded value, stored).
#[inline(always)]
fn negated(column: Floats) -> Floats {
    column.map(|c| fstp(-fld_local(c)))
}

/// `1 / sqrt(x * x + 1)`, each step stored to a float: `x * x`, plus the `double` 1.0 (a memory
/// operand), the root (`00ec6040`), then `FLD1; FDIVRP`.
#[inline(always)]
fn inverse_length(e: &mut Engine, x: f64) -> f32 {
    let one = f64::from_bits(e.global::<u64>(ONE_DOUBLE));
    let square = fstp(x87_mul(x, x));
    let square = fstp(x87_add(fld_local(square), one));
    let root = e.call(SQUARE_ROOT, &args![fld_local(square)]).f64();
    let root = fstp(root);
    fstp(x87_div(1.0, fld_local(root)))
}

/// What `NiPlane::NiPlane(normal, point)` (`00a69990`) makes, done here: the normal's words, and the
/// constant `(ny*py + nx*px) + nz*pz` (the normal loaded, the point a memory operand), stored into
/// plane `index`.
#[inline(always)]
fn set_plane(
    e: &mut Engine,
    this: Ptr<NiFrustumPlanes>,
    index: u32,
    normal: Floats,
    point: Floats,
) {
    let plane = this.cast::<NiPlane>().index(index, 0x10);
    let at = plane.at(NiPlane::m_kNormal);
    let components: [Field<NiPoint3, f32>; 3] = [NiPoint3::x, NiPoint3::y, NiPoint3::z];
    for (component, value) in components.iter().zip(normal) {
        e.set(at, *component, value);
    }
    let product = |i: usize| x87_mul(fld_local(normal[i]), m32_local(point[i]));
    let constant = fstp(x87_add(x87_add(product(1), product(0)), product(2)));
    e.set(plane, NiPlane::m_fConstant, constant);
}

// Typed from 00a74e10
/// `NiFrustumPlanes::Set` (Xbox PDB, `thiscall(planes, frustum, world transform)`): the six planes of
/// the frustum seen through the transform, with `c0`, `c1`, `c2` the rotation's columns (each entry
/// through a float slot) and `t` the translation (its words, unquieted). Near `(c0, t + near c0)`;
/// far `(-c0, far c0 + t)`. Orthographic: left `(c2, left c2 + t)`, right `(-c2, right c2 + t)`, top
/// `(-c1, top c1 + t)`, bottom `(c1, bottom c1 + t)`. Perspective, with `i = 1 / sqrt(x*x + 1)` for
/// each edge `x`, every plane through `t`: left `(-left i) c0 + i c2`, right `(right i) c0 + (-i)
/// c2`, top `(i top) c0 + (-i) c1`, bottom `(-bottom i) c0 + i c1`. Every product and sum is stored to
/// a float where the exe stores it; the near point sums `t` first, the others the product first (it
/// decides which NaN comes out when both are one).
pub fn frustum_planes_set(
    e: &mut Engine,
    this: Ptr<NiFrustumPlanes>,
    frustum: Ptr<NiFrustum>,
    transform: Ptr<NiTransform>,
) {
    let matrix = transform.at(NiTransform::m_Rotate);
    let (c0, c1, c2) = (
        column(e, matrix, 0),
        column(e, matrix, 1),
        column(e, matrix, 2),
    );
    let at = transform.at(NiTransform::m_Translate);
    let t: Floats = [
        e.get(at, NiPoint3::x),
        e.get(at, NiPoint3::y),
        e.get(at, NiPoint3::z),
    ];
    let stored = |e: &Engine, f: Field<NiFrustum, f32>| fstp(fld(e, frustum, f));

    let near = scaled(c0, stored(e, NiFrustum::m_fNear));
    set_plane(e, this, 0, c0, sum(t, near));
    let far = scaled(c0, stored(e, NiFrustum::m_fFar));
    set_plane(e, this, 1, negated(c0), sum(far, t));

    if e.get(frustum, NiFrustum::m_bOrtho) != 0 {
        let left = scaled(c2, stored(e, NiFrustum::m_fLeft));
        set_plane(e, this, 2, c2, sum(left, t));
        let right = scaled(c2, stored(e, NiFrustum::m_fRight));
        set_plane(e, this, 3, negated(c2), sum(right, t));
        let top = scaled(c1, stored(e, NiFrustum::m_fTop));
        set_plane(e, this, 4, negated(c1), sum(top, t));
        let bottom = scaled(c1, stored(e, NiFrustum::m_fBottom));
        set_plane(e, this, 5, c1, sum(bottom, t));
        return;
    }

    // Left: u = -left * i.
    let left = fld(e, frustum, NiFrustum::m_fLeft);
    let i = inverse_length(e, left);
    let u = fstp(x87_mul(-fld(e, frustum, NiFrustum::m_fLeft), fld_local(i)));
    let normal = sum(scaled(c0, u), scaled(c2, i));
    set_plane(e, this, 2, normal, t);

    // Right: k = right * i, m = -i.
    let i = inverse_length(e, fld(e, frustum, NiFrustum::m_fRight));
    let k = fstp(x87_mul(fld(e, frustum, NiFrustum::m_fRight), fld_local(i)));
    let m = fstp(-fld_local(i));
    let normal = sum(scaled(c0, k), scaled(c2, m));
    set_plane(e, this, 3, normal, t);

    // Top: k = i * top (the float a memory operand), m = -i.
    let i = inverse_length(e, fld(e, frustum, NiFrustum::m_fTop));
    let k = fstp(x87_mul(fld_local(i), m32(e, frustum, NiFrustum::m_fTop)));
    let m = fstp(-fld_local(i));
    let normal = sum(scaled(c0, k), scaled(c1, m));
    set_plane(e, this, 4, normal, t);

    // Bottom: k = -bottom * i.
    let i = inverse_length(e, fld(e, frustum, NiFrustum::m_fBottom));
    let k = fstp(x87_mul(
        -fld(e, frustum, NiFrustum::m_fBottom),
        fld_local(i),
    ));
    let normal = sum(scaled(c0, k), scaled(c1, i));
    set_plane(e, this, 5, normal, t);
}
