//! The point helpers the pathing code calls on every node: differences, sums, scaling, lengths and
//! the square root and absolute value wrappers under them.

use super::layout::Point2;
use super::{fld, fld_local, fstp, m32, m32_local, through, FAMILY};
use crate::culling::layout::NiPoint3;
use engine_core::ptr::Ptr;
use engine_core::typed::leaf;
use engine_core::x87::{x87_add, x87_mul, x87_sub};
use engine_core::{args, typed_call, Engine};

/// The C runtime's `sqrt` (`00ec6054`): a `double` argument, the root in `ST0`.
const SQUARE_ROOT: u32 = 0x00ec_6054;
/// `fabs` of a float (`00408860`), the result in `ST0`.
const FLOAT_ABS: u32 = 0x0040_8860;

/// The three floats of a new point, `(x, y, z)`, as `NiPoint3::NiPoint3` (`00416870`) takes them.
#[inline(always)]
fn new_point(e: &mut Engine, out: Ptr<NiPoint3>, x: f32, y: f32, z: f32) {
    typed_call!(leaf::FAMILY, 0x0041_6870, leaf::point3_new(e, out, x, y, z));
}

// Typed from 00439ef0
/// `NiPoint3::operator-` (`thiscall(this, out, other)`): each axis `this - other`, z first, stored to a
/// float and passed on, into a new point at `out`.
pub fn subtract(
    e: &mut Engine,
    this: Ptr<NiPoint3>,
    out: Ptr<NiPoint3>,
    other: Ptr<NiPoint3>,
) -> Ptr<NiPoint3> {
    let z = fstp(x87_sub(
        fld(e, this, NiPoint3::z),
        m32(e, other, NiPoint3::z),
    ));
    let y = fstp(x87_sub(
        fld(e, this, NiPoint3::y),
        m32(e, other, NiPoint3::y),
    ));
    let x = fstp(x87_sub(
        fld(e, this, NiPoint3::x),
        m32(e, other, NiPoint3::x),
    ));
    new_point(e, out, through(x), through(y), through(z));
    out
}

// Typed from 00439e90
/// `NiPoint3::operator+` (`thiscall(this, out, other)`): as [`subtract`] with sums.
pub fn add(
    e: &mut Engine,
    this: Ptr<NiPoint3>,
    out: Ptr<NiPoint3>,
    other: Ptr<NiPoint3>,
) -> Ptr<NiPoint3> {
    let z = fstp(x87_add(
        fld(e, this, NiPoint3::z),
        m32(e, other, NiPoint3::z),
    ));
    let y = fstp(x87_add(
        fld(e, this, NiPoint3::y),
        m32(e, other, NiPoint3::y),
    ));
    let x = fstp(x87_add(
        fld(e, this, NiPoint3::x),
        m32(e, other, NiPoint3::x),
    ));
    new_point(e, out, through(x), through(y), through(z));
    out
}

// Typed from 004a3760
/// `scalar * point` (`cdecl(out, scalar, point)`): each axis `scalar * point`, z first.
pub fn scaled(
    e: &mut Engine,
    out: Ptr<NiPoint3>,
    scalar: f32,
    point: Ptr<NiPoint3>,
) -> Ptr<NiPoint3> {
    let s = fld_local(scalar);
    let z = fstp(x87_mul(s, m32(e, point, NiPoint3::z)));
    let y = fstp(x87_mul(s, m32(e, point, NiPoint3::y)));
    let x = fstp(x87_mul(s, m32(e, point, NiPoint3::x)));
    new_point(e, out, through(x), through(y), through(z));
    out
}

// Typed from 00439180
/// `NiPoint3::operator*=` (`thiscall(this, scalar)`): each axis in place, x first, the scalar a memory
/// operand.
pub fn scale_by(e: &mut Engine, this: Ptr<NiPoint3>, scalar: f32) -> Ptr<NiPoint3> {
    for f in [NiPoint3::x, NiPoint3::y, NiPoint3::z] {
        let v = fstp(x87_mul(fld(e, this, f), m32_local(scalar)));
        e.set(this, f, v);
    }
    this
}

// Typed from 0063c8a0
/// `NiPoint3::operator+=` (`thiscall(this, other)`): each axis in place, x first.
pub fn add_to(e: &mut Engine, this: Ptr<NiPoint3>, other: Ptr<NiPoint3>) -> Ptr<NiPoint3> {
    for f in [NiPoint3::x, NiPoint3::y, NiPoint3::z] {
        let v = fstp(x87_add(fld(e, this, f), m32(e, other, f)));
        e.set(this, f, v);
    }
    this
}

/// `x*x + y*y` with each square's second factor a memory operand.
#[inline(always)]
fn squares_xy<S>(
    e: &Engine,
    p: Ptr<S>,
    x: engine_core::ptr::Field<S, f32>,
    y: engine_core::ptr::Field<S, f32>,
) -> f64 {
    let xx = x87_mul(fld(e, p, x), m32(e, p, x));
    let yy = x87_mul(fld(e, p, y), m32(e, p, y));
    x87_add(xx, yy)
}

// Typed from 00595c80
/// The squared length of a point's x and y (`thiscall(this)`), stored to a float.
pub fn length_xy_squared(e: &mut Engine, this: Ptr<NiPoint3>) -> f32 {
    fstp(squares_xy(e, this, NiPoint3::x, NiPoint3::y))
}

/// `(x*x + y*y) + z*z`.
#[inline(always)]
fn squares_xyz(e: &Engine, p: Ptr<NiPoint3>) -> f64 {
    let zz = x87_mul(fld(e, p, NiPoint3::z), m32(e, p, NiPoint3::z));
    x87_add(squares_xy(e, p, NiPoint3::x, NiPoint3::y), zz)
}

// Typed from 004a7290
/// The squared length of a point (`thiscall(this)`), stored to a float.
pub fn length_squared(e: &mut Engine, this: Ptr<NiPoint3>) -> f32 {
    fstp(squares_xyz(e, this))
}

// Typed from 00457990
/// `NiPoint3::Length` (`thiscall(this)`): the squared length stored to a float, its root
/// (`004579e0`).
pub fn length(e: &mut Engine, this: Ptr<NiPoint3>) -> f32 {
    let squared = fstp(squares_xyz(e, this));
    typed_call!(FAMILY, 0x0045_79e0, root_of(e, through(squared)))
}

// Typed from 00589850
/// `NiPoint2::Length` (`thiscall(this)`): as [`length`] in two axes.
pub fn length_2d(e: &mut Engine, this: Ptr<Point2>) -> f32 {
    let squared = fstp(squares_xy(e, this, Point2::x, Point2::y));
    typed_call!(FAMILY, 0x0045_79e0, root_of(e, through(squared)))
}

// Typed from 00408840
/// `std::abs` of a float (`cdecl(value)`): `fabs` (`00408860`) of the value passed on.
pub fn abs(e: &mut Engine, value: f32) -> f32 {
    e.call(FLOAT_ABS, &args![through(value)]).f32()
}

// Typed from 004579e0
/// A float's square root (`cdecl(value)`), through `004019b0`.
pub fn root_of(e: &mut Engine, value: f32) -> f32 {
    typed_call!(FAMILY, 0x0040_19b0, root(e, through(value)))
}

// Typed from 004019b0
/// A float's square root (`cdecl(value)`), through `004019d0`.
pub fn root(e: &mut Engine, value: f32) -> f32 {
    typed_call!(FAMILY, 0x0040_19d0, root_rounded(e, through(value)))
}

// Typed from 004019d0
/// A float's square root (`cdecl(value)`): the C runtime's `sqrt` of the value as a `double`, stored
/// to a float.
pub fn root_rounded(e: &mut Engine, value: f32) -> f32 {
    let b = fld_local(value).to_le_bytes();
    let low = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
    let high = u32::from_le_bytes([b[4], b[5], b[6], b[7]]);
    fstp(e.call(SQUARE_ROOT, &args![low, high]).f64())
}

// Typed from 004a0bd0
/// The point negated (`thiscall(this, out)`): each axis loaded and its sign changed, z first.
pub fn negated(e: &mut Engine, this: Ptr<NiPoint3>, out: Ptr<NiPoint3>) -> Ptr<NiPoint3> {
    let z = fstp(-fld(e, this, NiPoint3::z));
    let y = fstp(-fld(e, this, NiPoint3::y));
    let x = fstp(-fld(e, this, NiPoint3::x));
    new_point(e, out, x, y, z);
    out
}

// Typed from 0069e0c0
/// A point on a circle (`thiscall(direction, out, radius)`): `(radius * x, radius * y, 0)` from a
/// direction's cosine and sine, z first.
pub fn on_circle(
    e: &mut Engine,
    this: Ptr<Point2>,
    out: Ptr<NiPoint3>,
    radius: f32,
) -> Ptr<NiPoint3> {
    let y = fstp(x87_mul(fld_local(radius), m32(e, this, Point2::y)));
    let x = fstp(x87_mul(fld_local(radius), m32(e, this, Point2::x)));
    new_point(e, out, through(x), through(y), 0.0);
    out
}
