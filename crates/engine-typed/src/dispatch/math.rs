//! The arithmetic of the transforms, matrices and points the scene graph and the AI compose on every
//! update, and the small vector helpers around it.

use super::{copy_words, fld, fld_local, fstp, FAMILY};
use crate::culling::layout::{BSMultiBoundAABB, NiMatrix3, NiPoint3, NiTransform};
use crate::pathing::{self, fcom, m64_global};
use engine_core::ptr::Ptr;
use engine_core::typed::leaf;
use engine_core::x87::{
    x87_div, x87_dot3_store32, x87_load32, x87_m32, x87_mul_store32, x87_quiet, x87_quiet_bits,
    x87_store32, x87_sub,
};
use engine_core::{args, typed_call, Engine};

/// `NiMatrix3::NiMatrix3` (`006815c0`), which returns `this` and does nothing else.
const MATRIX_NEW: u32 = 0x0068_15c0;
/// `D3DXMatrixInverse` (`00ee6dda`).
const D3DX_MATRIX_INVERSE: u32 = 0x00ee_6dda;
/// The translation the fixed 4x4 inverse uses (`011f474c`).
const FIXED_TRANSLATION: u32 = 0x011f_474c;
/// The unit axes the box's half extents scale (`011a946c`, `011a9478`, `011a9484`).
const UNIT_AXES: [u32; 3] = [0x011a_946c, 0x011a_9478, 0x011a_9484];
/// The `double` a point's length must be above to be normalised (`01017cf8`).
const MINIMUM_LENGTH: u32 = 0x0101_7cf8;
/// The float the game's units are scaled by into Havok's (`011c582c`).
const HAVOK_SCALE: u32 = 0x011c_582c;

/// The float at `index` of the nine of a matrix.
#[inline(always)]
fn entry(e: &Engine, m: Ptr<NiMatrix3>, index: u32) -> u32 {
    e.mem.u32(m.addr().wrapping_add(4 * index))
}

/// A point's float at `index` (x, y, z).
#[inline(always)]
fn axis(e: &Engine, p: Ptr<NiPoint3>, index: u32) -> u32 {
    e.mem.u32(p.addr().wrapping_add(4 * index))
}

/// `a0*b0 + a1*b1 + a2*b2` as the exe sums three products and stores the sum: each `FLD a; FMUL b`
/// (the second factor a memory operand), added with `FADDP`, then `FSTP` ([`x87_dot3_store32`],
/// exact, with a fast path).
#[inline(always)]
fn dot3(a: [u32; 3], b: [u32; 3]) -> f32 {
    f32::from_bits(x87_dot3_store32(a, b))
}

/// `FLD a; FMUL b` (a memory operand) then `FSTP` ([`x87_mul_store32`], exact, with a fast path).
#[inline(always)]
fn mul_store(a: f32, b: f32) -> f32 {
    f32::from_bits(x87_mul_store32(a.to_bits(), b.to_bits()))
}

// Typed from 0062c250
/// `NiTransform::operator*` (`thiscall(this, out, other)`): the product of two transforms, built in a
/// local and copied to `out`: scale `this * other` (x87), rotation `this.rotate * other.rotate`
/// (`0043f8d0`), translation `this.translate + this.scale * (this.rotate * other.translate)`.
pub fn compose(
    e: &mut Engine,
    this: Ptr<NiTransform>,
    out: Ptr<NiTransform>,
    other: Ptr<NiTransform>,
) -> Ptr<NiTransform> {
    e.with_stack(0x80, |e, frame| {
        let at = |offset: u32| frame.addr() + 0x80 - offset;
        let product = Ptr::<NiTransform>::new(at(0x34));
        typed_call!(FAMILY, 0x0047_6a80, transform_new(e, product));
        let scale = mul_store(
            e.get(this, NiTransform::m_fScale),
            e.get(other, NiTransform::m_fScale),
        );
        e.set(product, NiTransform::m_fScale, scale);
        let rotation = typed_call!(
            FAMILY,
            0x0043_f8d0,
            multiply(e, this.cast(), Ptr::new(at(0x58)), other.cast())
        );
        copy_words(e, rotation.addr(), product.addr(), 9);
        let translate = other.byte_add(0x24).cast::<NiPoint3>();
        let rotated = typed_call!(
            FAMILY,
            0x004b_4500,
            rotate(e, this.cast(), Ptr::new(at(0x64)), translate)
        );
        let factor = f32::from_bits(x87_quiet_bits(e.mem.u32(this.addr() + 0x30)));
        let scaled = typed_call!(
            pathing::FAMILY,
            0x004a_3760,
            pathing::point::scaled(e, Ptr::new(at(0x70)), factor, rotated)
        );
        let own = this.byte_add(0x24).cast::<NiPoint3>();
        let moved = typed_call!(
            pathing::FAMILY,
            0x0043_9e90,
            pathing::point::add(e, own, Ptr::new(at(0x7c)), scaled)
        );
        copy_words(e, moved.addr(), product.addr() + 0x24, 3);
        copy_words(e, product.addr(), out.addr(), 13);
    });
    out
}

// Typed from 004b4880
/// `NiTransform::InvertNonUniform` (`thiscall(this, result)`): the transposed rotation (`004768c0`),
/// `1 / scale` (x87 `FDIVR` of 1.0), and the translation `scale' * (R' * -t)` (`004a0bd0`, `004b4500`,
/// `004a3760`), with the translation's frame of locals.
pub fn invert_non_uniform(e: &mut Engine, this: Ptr<NiTransform>, result: Ptr<NiTransform>) {
    e.with_stack(0x80, |e, frame| {
        let transposed = typed_call!(FAMILY, 0x0047_68c0, transpose(e, this.cast(), frame.cast()));
        copy_words(e, transposed.addr(), result.addr(), 9);
        let inverse = fstp(x87_div(1.0, fld(e, this, NiTransform::m_fScale)));
        e.set(result, NiTransform::m_fScale, inverse);
        let translate = this.byte_add(0x24).cast::<NiPoint3>();
        let negated = typed_call!(
            pathing::FAMILY,
            0x004a_0bd0,
            pathing::point::negated(e, translate, frame.byte_add(0x30).cast())
        );
        let rotated = typed_call!(
            FAMILY,
            0x004b_4500,
            rotate(e, result.cast(), frame.byte_add(0x40).cast(), negated)
        );
        let scale_now = x87_quiet(e.get(result, NiTransform::m_fScale));
        let scaled = typed_call!(
            pathing::FAMILY,
            0x004a_3760,
            pathing::point::scaled(e, frame.byte_add(0x50).cast(), scale_now, rotated)
        );
        copy_words(e, scaled.addr(), result.addr() + 0x24, 3);
    });
}

// Typed from 004b4500
/// `NiMatrix3::operator*(NiPoint3)` (`thiscall(this, out, vector)`): each row times the vector, z
/// first, into a new point at `out` (`00416870`).
pub fn rotate(
    e: &mut Engine,
    this: Ptr<NiMatrix3>,
    out: Ptr<NiPoint3>,
    vector: Ptr<NiPoint3>,
) -> Ptr<NiPoint3> {
    let v = [axis(e, vector, 0), axis(e, vector, 1), axis(e, vector, 2)];
    let row = |e: &Engine, r: u32| {
        [
            entry(e, this, 3 * r),
            entry(e, this, 3 * r + 1),
            entry(e, this, 3 * r + 2),
        ]
    };
    let z = dot3(row(e, 2), v);
    let y = dot3(row(e, 1), v);
    let x = dot3(row(e, 0), v);
    typed_call!(leaf::FAMILY, 0x0041_6870, leaf::point3_new(e, out, x, y, z));
    out
}

// Typed from 0043f8d0
/// `NiMatrix3::operator*` (`thiscall(this, out, other)`): each entry the row of `this` times the
/// column of `other`, column by column, into a local matrix (constructed with `006815c0`) copied to
/// `out`.
pub fn multiply(
    e: &mut Engine,
    this: Ptr<NiMatrix3>,
    out: Ptr<NiMatrix3>,
    other: Ptr<NiMatrix3>,
) -> Ptr<NiMatrix3> {
    e.with_stack(36, |e, result| {
        typed_call!(leaf::FAMILY, 0x0068_15c0, leaf::same_word(e, result));
        for column in 0..3 {
            for row in 0..3 {
                let a = [
                    entry(e, this, 3 * row),
                    entry(e, this, 3 * row + 1),
                    entry(e, this, 3 * row + 2),
                ];
                let b = [
                    entry(e, other, column),
                    entry(e, other, 3 + column),
                    entry(e, other, 6 + column),
                ];
                let value = dot3(a, b).to_bits();
                e.mem.set_u32(result.addr() + 4 * (3 * row + column), value);
            }
        }
        copy_words(e, result.addr(), out.addr(), 9);
    });
    out
}

// Typed from 00401050
/// The compiler's `eh vector constructor iterator` (`stdcall(array, size, count, constructor)`):
/// the `thiscall` constructor on each of `count` elements of `size` bytes from `array`, its answer
/// unread; a zero or negative count constructs nothing. `NiMatrix3::NiMatrix3` and `NiPoint3`'s
/// (`006815c0`, the leaf that answers `this`), what the matrices' arrays are made with, are called
/// directly; any other constructor by address.
pub fn construct_array(e: &mut Engine, array: u32, size: u32, count: i32, constructor: u32) {
    let mut element = array;
    let mut remaining = count;
    loop {
        remaining = remaining.wrapping_sub(1);
        if remaining < 0 {
            break;
        }
        if constructor == MATRIX_NEW {
            typed_call!(leaf::FAMILY, 0x0068_15c0, leaf::same_word(e, element));
        } else {
            e.call(constructor, &args![element]);
        }
        element = element.wrapping_add(size);
    }
}

// Typed from 004768c0
/// `NiMatrix3::Transpose` (`thiscall(this, result)`): the three rows of `this` (`00476930`) into a
/// local array (constructed with `00401050` and `006815c0`), set as the columns of `result`
/// (`00476980`).
pub fn transpose(e: &mut Engine, this: Ptr<NiMatrix3>, result: Ptr<NiMatrix3>) -> Ptr<NiMatrix3> {
    e.with_stack(0x24, |e, rows| {
        let at = |i: u32| Ptr::<NiPoint3>::new(rows.addr() + 12 * i);
        typed_call!(
            FAMILY,
            0x0040_1050,
            construct_array(e, rows.addr(), 12u32, 3i32, MATRIX_NEW)
        );
        for i in 0..3 {
            typed_call!(FAMILY, 0x0047_6930, row(e, this, i, at(i)));
        }
        typed_call!(
            FAMILY,
            0x0047_6980,
            set_columns(e, result, at(0), at(1), at(2))
        );
    });
    result
}

// Typed from 00476930
/// `NiMatrix3::GetRow` (`thiscall(this, row, out)`): the row's three floats through the x87 stack.
pub fn row(e: &mut Engine, this: Ptr<NiMatrix3>, row: u32, out: Ptr<NiPoint3>) {
    let from = this.addr().wrapping_add(row.wrapping_mul(12));
    for i in 0..3 {
        let value = x87_quiet_bits(e.mem.u32(from.wrapping_add(4 * i)));
        e.mem.set_u32(out.addr().wrapping_add(4 * i), value);
    }
}

// Typed from 00476980
/// `NiMatrix3::SetCol` of all three (`thiscall(this, c0, c1, c2)`), each with `004769c0`.
pub fn set_columns(
    e: &mut Engine,
    this: Ptr<NiMatrix3>,
    column0: Ptr<NiPoint3>,
    column1: Ptr<NiPoint3>,
    column2: Ptr<NiPoint3>,
) -> Ptr<NiMatrix3> {
    for (i, column) in (0u32..).zip([column0, column1, column2]) {
        typed_call!(FAMILY, 0x0047_69c0, set_column(e, this, i, column));
    }
    this
}

// Typed from 004769c0
/// `NiMatrix3::SetCol` (`thiscall(this, column, source)`): the point's three floats down the column,
/// through the x87 stack.
pub fn set_column(e: &mut Engine, this: Ptr<NiMatrix3>, column: u32, source: Ptr<NiPoint3>) {
    for i in 0..3 {
        let value = x87_quiet_bits(axis(e, source, i));
        let to = this
            .addr()
            .wrapping_add(column.wrapping_mul(4))
            .wrapping_add(12 * i);
        e.mem.set_u32(to, value);
    }
}

// Typed from 00476a80
/// `NiTransform::NiTransform` (`thiscall(this)`): the rotation's and the translation's constructors
/// (`006815c0`, which return `this`); nothing is stored.
pub fn transform_new(e: &mut Engine, this: Ptr<NiTransform>) -> Ptr<NiTransform> {
    typed_call!(leaf::FAMILY, 0x0068_15c0, leaf::same_word(e, this));
    let translation = this.addr() + 0x24;
    typed_call!(leaf::FAMILY, 0x0068_15c0, leaf::same_word(e, translation));
    this
}

// Typed from 0045bb20
/// `NiPoint3::operator*(float)` (`thiscall(this, out, scale)`): each axis `scale * this`, z first, into
/// a new point at `out`.
pub fn times(e: &mut Engine, this: Ptr<NiPoint3>, out: Ptr<NiPoint3>, scale: f32) -> Ptr<NiPoint3> {
    let z = mul_store(scale, e.get(this, NiPoint3::z));
    let y = mul_store(scale, e.get(this, NiPoint3::y));
    let x = mul_store(scale, e.get(this, NiPoint3::x));
    typed_call!(leaf::FAMILY, 0x0041_6870, leaf::point3_new(e, out, x, y, z));
    out
}

// Typed from 004a0c10
/// `NiPoint3::Unitize` (`thiscall(this)`): a length (`00457990`) above the minimum (`01017cf8`) divides
/// each axis (by `1 / length` stored as a float, a memory operand); any other length, NaN included,
/// zeroes the point.
pub fn normalize(e: &mut Engine, this: Ptr<NiPoint3>) {
    let length = typed_call!(
        pathing::FAMILY,
        0x0045_7990,
        pathing::point::length(e, this)
    );
    let length = fstp(f64::from(length));
    if fcom(fld_local(length), m64_global(e, MINIMUM_LENGTH)) & 0x41 != 0 {
        for f in [NiPoint3::x, NiPoint3::y, NiPoint3::z] {
            e.set(this, f, 0.0);
        }
        return;
    }
    let inverse = fstp(x87_div(1.0, fld_local(length)));
    for f in [NiPoint3::x, NiPoint3::y, NiPoint3::z] {
        let v = mul_store(e.get(this, f), inverse);
        e.set(this, f, v);
    }
}

// Typed from 00c39270
/// `BSMultiBoundAABB::GetVertices` (`thiscall(this, out)`): the unit axes scaled by the half extents
/// (`0045bb20`), then each corner the centre plus or minus each of them in turn (`00439e90`,
/// `00439ef0`; bit 2 of the corner's number makes the first a subtraction), the eight corners at
/// `out`. The translation's frame: the axis points, then three scratch points a corner.
pub fn box_corners(e: &mut Engine, this: Ptr, out: Ptr) {
    let this = this.cast::<BSMultiBoundAABB>();
    let center = this.at(BSMultiBoundAABB::Center);
    let half = this.at(BSMultiBoundAABB::HalfExtents);
    e.with_stack(12 * (3 + 24), |e, scratch| {
        let axis_points = [0, 12, 24].map(|o| Ptr::<NiPoint3>::new(scratch.addr() + o));
        for a in 0..3u32 {
            let extent = f32::from_bits(x87_quiet_bits(axis(e, half, a)));
            let unit = Ptr::<NiPoint3>::new(UNIT_AXES[a as usize]);
            typed_call!(
                FAMILY,
                0x0045_bb20,
                times(e, unit, axis_points[a as usize], extent)
            );
        }
        for corner in 0..8u32 {
            let mut current = center;
            for a in 0..3u32 {
                let temporary = Ptr::<NiPoint3>::new(scratch.addr() + 36 + 12 * (corner * 3 + a));
                let point = axis_points[a as usize];
                current = if corner >> (2 - a) & 1 == 0 {
                    typed_call!(
                        pathing::FAMILY,
                        0x0043_9e90,
                        pathing::point::add(e, current, temporary, point)
                    )
                } else {
                    typed_call!(
                        pathing::FAMILY,
                        0x0043_9ef0,
                        pathing::point::subtract(e, current, temporary, point)
                    )
                };
            }
            copy_words(e, current.addr(), out.addr() + 12 * corner, 3);
        }
    });
}

/// `value * scale` as the inverse's rows have it: the element through a float temporary, times the
/// scale as a memory operand.
#[inline(always)]
fn scaled(value: u32, scale: u32) -> u32 {
    x87_mul_store32(value, scale)
}

// Typed from 00c4c2d0
/// The inverse of a transform as a row-major 4x4 matrix at `out` (`cdecl(transform, fixed, out)`).
/// With `fixed` zero: `004b4880`, then the inverse rotation's columns times its scale as the first
/// three rows, 0.0 in the fourth column, the translation as the last row and 1.0. Otherwise
/// `D3DXMatrixInverse` of the identity with the translation at `011f474c`.
pub fn inverse_4x4(e: &mut Engine, transform: Ptr<NiTransform>, fixed: u32, out: Ptr) {
    let out = out.addr();
    if fixed == 0 {
        e.with_stack(0x34, |e, inverse| {
            typed_call!(
                FAMILY,
                0x004b_4880,
                invert_non_uniform(e, transform, inverse.cast())
            );
            let inverse = inverse.addr();
            let scale = e.mem.u32(inverse + 0x30);
            for column in 0..3u32 {
                for row in 0..3u32 {
                    let element = x87_quiet_bits(e.mem.u32(inverse + row * 12 + column * 4));
                    e.mem.set_u32(
                        out.wrapping_add(column * 16 + row * 4),
                        scaled(element, scale),
                    );
                }
                e.mem.set_f32(out.wrapping_add(column * 16 + 12), 0.0);
            }
            for k in 0..3u32 {
                let value = x87_quiet_bits(e.mem.u32(inverse + 0x24 + 4 * k));
                e.mem.set_u32(out.wrapping_add(0x30 + 4 * k), value);
            }
            e.mem.set_f32(out.wrapping_add(0x3c), 1.0);
        });
        return;
    }
    e.with_stack(0x40, |e, matrix| {
        let matrix = matrix.addr();
        for k in 0..16u32 {
            e.mem
                .set_f32(matrix + 4 * k, if k % 5 == 0 { 1.0 } else { 0.0 });
        }
        for k in 0..3u32 {
            let value = x87_quiet_bits(e.mem.u32(FIXED_TRANSLATION + 4 * k));
            e.mem.set_u32(matrix + 0x30 + 4 * k, value);
        }
        e.call(D3DX_MATRIX_INVERSE, &args![out, 0u32, matrix]);
    });
}

// Typed from 004a3e00
/// A point in Havok's units as a vector of four (`cdecl(out, point)`): each axis through the x87
/// stack into `004a3e90`, stored in its lane (`00560d30`); the fourth lane 0.0.
pub fn havok_vector(e: &mut Engine, out: Ptr, point: Ptr<NiPoint3>) -> Ptr {
    for index in 0..3u32 {
        let value = f32::from_bits(x87_quiet_bits(axis(e, point, index)));
        let scaled = typed_call!(FAMILY, 0x004a_3e90, havok_scale(e, value));
        let slot = typed_call!(FAMILY, 0x0056_0d30, lane(e, out, index));
        e.mem.set_f32(slot.addr(), scaled);
    }
    let w = typed_call!(FAMILY, 0x0056_0d30, lane(e, out, 3));
    e.mem.set_f32(w.addr(), 0.0);
    out
}

// Typed from 004a3e90
/// `value` in Havok's units (`cdecl(value)`): the scale at `011c582c` times the argument (a memory
/// operand), stored as a float and loaded back.
pub fn havok_scale(e: &mut Engine, value: f32) -> f32 {
    mul_store(e.global::<f32>(HAVOK_SCALE), value)
}

// Typed from 00560d30
/// `hkVector4`'s lane `index` (`thiscall(this, index)`): its address.
pub fn lane(_e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    Ptr::new(this.addr().wrapping_add(index.wrapping_mul(4)))
}

// Typed from 004a3c90
/// `hkVector4::operator=` (`thiscall(this, source)`): the 16 bytes copied.
pub fn copy_16(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    copy_words(e, source.addr(), this.addr(), 4);
    this
}

// Typed from 00452dc0
/// `NiPoint2::NiPoint2` (`thiscall(this, x, y)`): both through the x87 stack.
pub fn point2_new(e: &mut Engine, this: Ptr, x: f32, y: f32) -> Ptr {
    e.mem.set_f32(this.addr(), x87_quiet(x));
    e.mem.set_f32(this.addr() + 4, x87_quiet(y));
    this
}

// Typed from 0049e2f0
/// Whether two points are within `tolerance` on every axis (`cdecl(a, b, tolerance)`): each pair with
/// `0049e390`, the floats through the x87 stack; false at the first that isn't.
pub fn points_close(e: &mut Engine, a: Ptr<NiPoint3>, b: Ptr<NiPoint3>, tolerance: f32) -> bool {
    let tolerance = x87_quiet(tolerance);
    for i in 0..3 {
        let x = f32::from_bits(x87_quiet_bits(axis(e, a, i)));
        let y = f32::from_bits(x87_quiet_bits(axis(e, b, i)));
        if !typed_call!(FAMILY, 0x0049_e390, floats_close(e, x, y, tolerance)) {
            return false;
        }
    }
    true
}

// Typed from 0049e390
/// Whether `|a - b|` (`00408840` of the difference stored as a float) is at most `tolerance`
/// (`cdecl(a, b, tolerance)`).
pub fn floats_close(e: &mut Engine, a: f32, b: f32, tolerance: f32) -> bool {
    let difference = x87_store32(x87_sub(x87_load32(a.to_bits()), x87_m32(b.to_bits())));
    let difference = f32::from_bits(x87_quiet_bits(difference));
    let magnitude = typed_call!(
        pathing::FAMILY,
        0x0040_8840,
        pathing::point::abs(e, difference)
    );
    x87_load32(tolerance.to_bits()) >= f64::from(magnitude)
}
