//! `SpeedTree/projectedshadow.cpp` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `CProjectedShadow`: the self-shadow projection of a tree (three axes and the name of the
//! shadow map): the reader of its tree-file chunk, the closest point on a line and the
//! computation of the shadow texture coordinates of a geometry.
//!
//! x87 note: the exe computes in extended precision and stores `float` results. The
//! translations compute in `f64` through the shared x87 helpers, in the exe's order of operations,
//! and round to `f32` at each `float` store (`FSTP dword`). A float copied through the x87 stack
//! (`FLD dword`, `FSTP dword`) goes through [`x87_quiet_bits`].
//!
//! Functions that carry an `FS:[0]` exception frame or the `/GS` cookie check are marked
//! `Translation incomplete` (the unwinding handler is not translated).

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::CVec3;
use super::indexedgeometry::CIndexedGeometry;

layout! {
    /// `CProjectedShadow` (Xbox PDB), 0x40 bytes on the PC as on the Xbox: the three axes as
    /// `CVec3`s; the shadow map's name, a `std::string`, sits at `+0x24` (read by offset).
    pub struct CProjectedShadow: 0x40 {
        /// `m_cRight` (Xbox PDB).
        0x00 m_cRight: Inline<CVec3>,
        /// `m_cUp` (Xbox PDB).
        0x0C m_cUp: Inline<CVec3>,
        /// `m_cOut` (Xbox PDB).
        0x18 m_cOut: Inline<CVec3>,
    }
}

/// The offset of `m_strSelfShadowMap` (Xbox PDB), the shadow map's name.
const SELF_SHADOW_MAP: u32 = 0x24;

/// `CTreeFileAccess::ParseInt` (`00b3ed80`, `fastcall`), `ParseVector3` (`00b3ef80`, `thiscall`:
/// the vector to fill; returns it) and `ParseString` (`00b3ee40`, `thiscall`: the string to
/// fill).
const PARSE_INT: u32 = 0x00b3_ed80;
const PARSE_VECTOR3: u32 = 0x00b3_ef80;
const PARSE_STRING: u32 = 0x00b3_ee40;
/// The `CIdvFilename` constructor from a `std::string` passed by value (`00b09ef0`, `thiscall`;
/// the 0x1C bytes of the string follow the object) and its `NoPath` (`00b09f90`, `thiscall`: the
/// string to fill; returns it).
const FILENAME_CONSTRUCT: u32 = 0x00b0_9ef0;
const FILENAME_NO_PATH: u32 = 0x00b0_9f90;
/// The string's `assign(string, pos, count)` (`0044a580`), its `_Tidy(built, new_size)`
/// (`0044a730`), `std::string::npos` (`010173f0`, read from memory) and `IdvFormatString`
/// (`00b18860`, `cdecl`, variadic: the string to fill, the format, the arguments).
const STRING_ASSIGN: u32 = 0x0044_a580;
const STRING_TIDY: u32 = 0x0044_a730;
const STRING_NPOS: u32 = 0x0101_73f0;
const FORMAT_STRING: u32 = 0x00b1_8860;
/// The format of the unknown-chunk error (`010a558c`), the parse-error exception constructor
/// (`00b09480`: the message string and the errno flag), `_CxxThrowException` and the parse
/// error's throw info (`01170384`).
const UNKNOWN_CHUNK_FORMAT: u32 = 0x010a_558c;
const PARSE_ERROR_CONSTRUCT: u32 = 0x00b0_9480;
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
const PARSE_THROW_INFO: u32 = 0x0117_0384;
/// The `CVec3` helpers outside this unit: `a - b` as `this = a` (`00b13040`: a result slot and
/// `b`), the sum (`00b0cf90`: a result slot and the other vector), the product with a float
/// (`00b0cff0`: a result slot and the float), `Normalize` (`00b12790`), the dot product
/// (`00b2a180`: the other vector, result in `ST0`) and `Distance` (`00b04d40`: the other point,
/// result in `ST0`); all `thiscall` on the first vector.
const VEC3_SUBTRACT: u32 = 0x00b1_3040;
const VEC3_ADD: u32 = 0x00b0_cf90;
const VEC3_SCALE: u32 = 0x00b0_cff0;
const VEC3_NORMALIZE: u32 = 0x00b1_2790;
const VEC3_DOT: u32 = 0x00b2_a180;
const VEC3_DISTANCE: u32 = 0x00b0_4d40;
/// The coordinate triple of a geometry's vertex (`00b30f80`, `thiscall`: the vertex index) and
/// `CIndexedGeometry::AddVertexTexCoord1` (`00b304f0`, `thiscall`: the two floats).
const COORD_POINTER: u32 = 0x00b3_0f80;
const ADD_TEX_COORD1: u32 = 0x00b3_04f0;

/// The chunk ids of the projected shadow's tree-file section: the end marker, the three axes and
/// the shadow map name.
const CHUNK_END: i32 = 0x4651;
const CHUNK_RIGHT: i32 = 0x4652;
const CHUNK_UP: i32 = 0x4653;
const CHUNK_OUT: i32 = 0x4654;
const CHUNK_SHADOW_MAP: i32 = 0x4655;

/// Reads a vector chunk (`ParseVector3` into a local) and copies its three floats through the
/// x87 stack to `target`.
fn parse_axis(e: &mut Engine, file: Ptr, target: u32) {
    e.with_stack(0x0C, |e, local| {
        let parsed = e.call(PARSE_VECTOR3, &args![file, local]).u32();
        for axis in [0u32, 4, 8] {
            let bits = x87_quiet_bits(e.mem.u32(parsed + axis));
            e.mem.set_u32(target + axis, bits);
        }
    });
}

/// Throws the error for an unknown chunk id: the message (`IdvFormatString` with the format and
/// the chunk id), the error object (`00b09480`, errno not used) built from it, and
/// `_CxxThrowException` with the parse error's throw info.
fn throw_unknown_chunk(e: &mut Engine, chunk: i32) -> ! {
    e.with_stack(0x1C, |e, text| {
        let formatted = e
            .call(FORMAT_STRING, &args![text, UNKNOWN_CHUNK_FORMAT, chunk])
            .u32();
        e.with_stack(0x28, |e, error| {
            e.call(PARSE_ERROR_CONSTRUCT, &args![error, formatted, 0u32]);
            e.call(CXX_THROW_EXCEPTION, &args![error, PARSE_THROW_INFO]);
        });
    });
    unreachable!("_CxxThrowException does not return (00ec7894)")
}

// Translated from 00b3e750 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3e750 - the FS:[0] exception frame (destroying the temporary strings
// if a step throws) and the /GS cookie check (`00ec408c`) are not translated.
/// `CProjectedShadow::Parse` (Xbox PDB; `thiscall`: the tree file access): reads chunk ids
/// (`ParseInt`) until the end marker `0x4651`: `0x4652`, `0x4653` and `0x4654` read a vector into
/// the right, up and out axes (each float copied through the x87 stack); `0x4655` reads a string
/// (`ParseString`), makes a `CIdvFilename` from it (`00b09ef0`, the string's 0x1C bytes passed by
/// value) and assigns `m_strSelfShadowMap` the filename without its path (`00b09f90`, assigned
/// with `0044a580`), then tidies the two temporaries (`0044a730` with 1 and 0). Any other id
/// (the first id is not checked against the end marker: it is read before the loop's test, so
/// an end marker first is an unknown chunk too) throws a parse error built from the formatted
/// message.
pub fn c_projected_shadow_parse(e: &mut Engine, this: Ptr<CProjectedShadow>, file: Ptr) {
    let base = this.addr();
    let mut chunk = e.call(PARSE_INT, &args![file]).i32();
    loop {
        match chunk {
            CHUNK_RIGHT => parse_axis(e, file, base),
            CHUNK_UP => parse_axis(e, file, base + 0x0C),
            CHUNK_OUT => parse_axis(e, file, base + 0x18),
            CHUNK_SHADOW_MAP => {
                // The string is built in the argument area of the filename constructor.
                e.with_stack(0x1C, |e, name| {
                    e.call(PARSE_STRING, &args![file, name]);
                    e.with_stack(0x1C, |e, filename| {
                        let mut words = vec![filename.addr()];
                        for word in 0..7u32 {
                            words.push(e.mem.u32(name.addr() + 4 * word));
                        }
                        e.call(FILENAME_CONSTRUCT, &words);
                        e.with_stack(0x1C, |e, no_path| {
                            let shortened =
                                e.call(FILENAME_NO_PATH, &args![filename, no_path]).u32();
                            let npos = e.global::<u32>(STRING_NPOS);
                            e.call(
                                STRING_ASSIGN,
                                &args![base + SELF_SHADOW_MAP, shortened, 0u32, npos],
                            );
                            e.call(STRING_TIDY, &args![no_path, 1u32, 0u32]);
                        });
                        e.call(STRING_TIDY, &args![filename, 1u32, 0u32]);
                    });
                });
            }
            _ => throw_unknown_chunk(e, chunk),
        }
        chunk = e.call(PARSE_INT, &args![file]).i32();
        if chunk == CHUNK_END {
            break;
        }
    }
}

// Translated from 00b3ec40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CProjectedShadow::ClosestPoint` (Xbox PDB; `thiscall`, `this` unused: a result vector, the
/// line's two points and the point): when the two points are equal (all three coordinates
/// ordered-equal: an `FUCOMPP` flag test, so a NaN is unequal) the result is the first point
/// (copied word by word); otherwise the direction `b - a` (`00b13040`) is normalized
/// (`00b12790`), the distance along it of `p - a` is its dot product with the direction
/// (`00b2a180`, stored as a float), and the result is `a + direction * distance` (`00b0cff0`,
/// `00b0cf90`): the closest point on the whole line through the two points. Returns the result
/// vector.
pub fn c_projected_shadow_closest_point(
    e: &mut Engine,
    _this: Ptr<CProjectedShadow>,
    out: Ptr,
    a: Ptr,
    b: Ptr,
    p: Ptr,
) -> Ptr {
    let mut equal = true;
    for axis in [0u32, 4, 8] {
        let first = x87_load32(e.mem.u32(a.addr() + axis));
        let second = x87_load32(e.mem.u32(b.addr() + axis));
        if first != second {
            equal = false;
            break;
        }
    }
    if equal {
        for axis in [0u32, 4, 8] {
            let word = e.mem.u32(a.addr() + axis);
            e.mem.set_u32(out.addr() + axis, word);
        }
        return out;
    }
    e.with_stack(0x24, |e, frame| {
        let (direction, offset, scaled) = (frame, frame.byte_add(0x0C), frame.byte_add(0x18));
        e.call(VEC3_SUBTRACT, &args![b, direction, a]);
        e.call(VEC3_NORMALIZE, &args![direction]);
        e.call(VEC3_SUBTRACT, &args![p, offset, a]);
        let dot = e.call(VEC3_DOT, &args![offset, direction]).f64();
        let along = x87_quiet_bits(x87_store32(dot));
        let product = e.call(VEC3_SCALE, &args![direction, scaled, along]).u32();
        e.call(VEC3_ADD, &args![a, out, product]);
    });
    out
}

/// `source * (high - low) + low` as the x87 sequence `FLD [source]; FLD [low]; FSUBR [high];
/// FMULP; FADD [low]` (the memory operands `high` and `low` as floats); `low` was first copied
/// through the x87 stack by the caller. Returns the float's bits.
fn blend(e: &Engine, source: u32, high: u32, low: u32) -> u32 {
    let low_bits = x87_quiet_bits(e.mem.u32(low));
    let scale = x87_load32(e.mem.u32(source));
    let span = x87_sub(x87_m32(e.mem.u32(high)), x87_load32(low_bits));
    x87_store32(x87_add(x87_mul(scale, span), x87_m32(low_bits)))
}

// Translated from 00b3ea10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CProjectedShadow::ComputeTexCoords` (Xbox PDB; `thiscall`: the geometry, the centre (a `CVec3`
/// passed by value, three words), the radius and an optional rectangle of six floats): builds
/// the end points of the right axis and of the up axis around the centre (`centre - axis *
/// radius` and `centre + axis * radius`, through `00b0cff0`, `00b13040` and `00b0cf90`), resets
/// the geometry's vertex counter (`+0x22`) and, for each vertex (`m_vCoords` floats / 3,
/// truncated to 16 bits), finds the closest point of each axis line (`ClosestPoint`), takes its
/// distance (`00b04d40`) from the line's first end point, divides by twice the radius, maps
/// the first through the rectangle (`u = x * (rect[0] - rect[2]) + rect[2]`) and the second
/// (`v = y * (rect[1] - rect[5]) + rect[5]`) when a rectangle is given, and adds the pair as
/// the vertex's second texture coordinates (`AddVertexTexCoord1`, `00b304f0`).
pub fn c_projected_shadow_compute_tex_coords(
    e: &mut Engine,
    this: Ptr<CProjectedShadow>,
    geometry: Ptr<CIndexedGeometry>,
    centre_x: u32,
    centre_y: u32,
    centre_z: u32,
    radius: f32,
    rectangle: Ptr,
) {
    let radius_bits = x87_quiet_bits(radius.to_bits());
    e.with_stack(0x6C, |e, frame| {
        let at = |offset: u32| frame.addr() + offset;
        // The centre as an argument vector at +0x00; the end points at +0x0C (right, minus),
        // +0x18 (right, plus), +0x24 (up, minus), +0x30 (up, plus); scratch at +0x3C...
        let centre_at = at(0);
        for (i, word) in [centre_x, centre_y, centre_z].iter().enumerate() {
            e.mem.set_u32(centre_at + 4 * i as u32, *word);
        }
        let (right_minus, right_plus) = (at(0x0C), at(0x18));
        let (up_minus, up_plus) = (at(0x24), at(0x30));
        let right = this.addr();
        let up = this.addr() + 0x0C;
        let scaled = e
            .call(VEC3_SCALE, &args![right, at(0x3C), radius_bits])
            .u32();
        e.call(VEC3_SUBTRACT, &args![centre_at, right_minus, scaled]);
        let scaled = e
            .call(VEC3_SCALE, &args![right, at(0x48), radius_bits])
            .u32();
        e.call(VEC3_ADD, &args![centre_at, right_plus, scaled]);
        let scaled = e.call(VEC3_SCALE, &args![up, at(0x54), radius_bits]).u32();
        e.call(VEC3_SUBTRACT, &args![centre_at, up_minus, scaled]);
        let scaled = e.call(VEC3_SCALE, &args![up, at(0x60), radius_bits]).u32();
        e.call(VEC3_ADD, &args![centre_at, up_plus, scaled]);
        e.mem.set_u16(geometry.addr() + 0x22, 0);
        let mut vertex: u16 = 0;
        loop {
            let coords = geometry.addr() + 0x88;
            let count = (((e
                .mem
                .u32(coords + 0x10)
                .wrapping_sub(e.mem.u32(coords + 0x0C))) as i32
                >> 2) as u32
                / 3) as u16;
            if i32::from(vertex) >= i32::from(count) {
                break;
            }
            let pointer = e
                .call(COORD_POINTER, &args![geometry, u32::from(vertex)])
                .u32();
            // The vertex's coordinates, copied through the x87 stack twice.
            e.with_stack(0x24, |e, work| {
                let (first, second, point) = (work.addr(), work.addr() + 0x0C, work.addr() + 0x18);
                for axis in [0u32, 4, 8] {
                    let bits = x87_quiet_bits(e.mem.u32(pointer + axis));
                    e.mem.set_u32(first + axis, bits);
                }
                for axis in [0u32, 4, 8] {
                    let bits = x87_quiet_bits(e.mem.u32(first + axis));
                    e.mem.set_u32(second + axis, bits);
                }
                for axis in [0u32, 4, 8] {
                    let word = e.mem.u32(second + axis);
                    e.mem.set_u32(point + axis, word);
                }
                e.with_stack(0x20, |e, result| {
                    let (closest_u, closest_v, pair) =
                        (result.addr(), result.addr() + 0x0C, result.addr() + 0x18);
                    c_projected_shadow_closest_point(
                        e,
                        this,
                        Ptr::new(closest_u),
                        Ptr::new(right_minus),
                        Ptr::new(right_plus),
                        Ptr::new(point),
                    );
                    let distance = e.call(VEC3_DISTANCE, &args![right_minus, closest_u]).f64();
                    let twice = x87_add(x87_load32(radius_bits), x87_load32(radius_bits));
                    let mut u_bits = x87_store32(x87_div(distance, twice));
                    e.mem.set_u32(pair, u_bits);
                    if !rectangle.is_null() {
                        u_bits = blend(e, pair, rectangle.addr(), rectangle.addr() + 8);
                        e.mem.set_u32(pair, u_bits);
                    }
                    c_projected_shadow_closest_point(
                        e,
                        this,
                        Ptr::new(closest_v),
                        Ptr::new(up_minus),
                        Ptr::new(up_plus),
                        Ptr::new(point),
                    );
                    let distance = e.call(VEC3_DISTANCE, &args![up_minus, closest_v]).f64();
                    let twice = x87_add(x87_load32(radius_bits), x87_load32(radius_bits));
                    let mut v_bits = x87_store32(x87_div(distance, twice));
                    e.mem.set_u32(pair + 4, v_bits);
                    if !rectangle.is_null() {
                        v_bits = blend(e, pair + 4, rectangle.addr() + 4, rectangle.addr() + 0x14);
                        e.mem.set_u32(pair + 4, v_bits);
                    }
                    e.call(ADD_TEX_COORD1, &args![geometry, pair]);
                });
            });
            vertex = vertex.wrapping_add(1);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b3e750,
            c_projected_shadow_parse(Ptr<CProjectedShadow>, Ptr)
        ),
        entry!(
            0x00b3ea10,
            c_projected_shadow_compute_tex_coords(
                Ptr<CProjectedShadow>,
                Ptr<CIndexedGeometry>,
                u32,
                u32,
                u32,
                f32,
                Ptr,
            )
        ),
        entry!(
            0x00b3ec40,
            c_projected_shadow_closest_point(Ptr<CProjectedShadow>, Ptr, Ptr, Ptr, Ptr) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0020_0000);
        e
    }

    fn panics(action: impl FnOnce()) -> bool {
        catch_unwind(AssertUnwindSafe(action)).is_err()
    }

    fn put(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    fn floats(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    /// Real-arithmetic doubles of the `CVec3` helpers (single precision per component).
    fn vec3_doubles(e: &mut Engine) -> Calls {
        let log: Calls = Rc::new(RefCell::new(Vec::new()));
        let binary = |e: &mut Engine, address: u32, op: fn(f32, f32) -> f32, log: &Calls| {
            let sink = log.clone();
            e.register_double(address, move |e, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                for k in 0..3 {
                    let v = op(e.mem.f32(a[0] + 4 * k), e.mem.f32(a[2] + 4 * k));
                    e.mem.set_f32(a[1] + 4 * k, v);
                }
                ret(a[1])
            });
        };
        binary(e, VEC3_SUBTRACT, |x, y| x - y, &log);
        binary(e, VEC3_ADD, |x, y| x + y, &log);
        let sink = log.clone();
        e.register_double(VEC3_SCALE, move |e, a| {
            sink.borrow_mut().push((VEC3_SCALE, a.to_vec()));
            let s = f32::from_bits(a[2]);
            for k in 0..3 {
                let v = e.mem.f32(a[0] + 4 * k) * s;
                e.mem.set_f32(a[1] + 4 * k, v);
            }
            ret(a[1])
        });
        e.register(VEC3_NORMALIZE, |e, a| {
            let v: Vec<f32> = (0..3).map(|k| e.mem.f32(a[0] + 4 * k)).collect();
            let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            for k in 0..3 {
                e.mem.set_f32(a[0] + 4 * k, v[k as usize] / length);
            }
            Ret::default()
        });
        e.register(VEC3_DOT, |e, a| Ret {
            st0: (0..3)
                .map(|k| f64::from(e.mem.f32(a[0] + 4 * k)) * f64::from(e.mem.f32(a[1] + 4 * k)))
                .sum(),
            ..Ret::default()
        });
        e.register(VEC3_DISTANCE, |e, a| Ret {
            st0: (0..3)
                .map(|k| {
                    let d = f64::from(e.mem.f32(a[1] + 4 * k)) - f64::from(e.mem.f32(a[0] + 4 * k));
                    d * d
                })
                .sum::<f64>()
                .sqrt(),
            ..Ret::default()
        });
        log
    }

    #[test]
    fn closest_point_projects_onto_the_whole_line() {
        let mut e = engine();
        let log = vec3_doubles(&mut e);
        let a = e.mem.alloc(12);
        let b = e.mem.alloc(12);
        let p = e.mem.alloc(12);
        let out = e.mem.alloc(12);
        put(&mut e, a, &[0.0, 0.0, 0.0]);
        put(&mut e, b, &[2.0, 0.0, 0.0]);
        put(&mut e, p, &[1.0, 5.0, 0.0]);
        let r = e.call(
            0x00b3_ec40,
            &args![
                Ptr::<()>::NULL,
                Ptr::<()>::new(out),
                Ptr::<()>::new(a),
                Ptr::<()>::new(b),
                Ptr::<()>::new(p)
            ],
        );
        assert_eq!(r.u32(), out);
        assert_eq!(floats(&e, out, 3), [1.0, 0.0, 0.0]);
        // Beyond the end point the line continues (no clamping).
        put(&mut e, p, &[5.0, 1.0, 0.0]);
        e.call(
            0x00b3_ec40,
            &args![
                Ptr::<()>::NULL,
                Ptr::<()>::new(out),
                Ptr::<()>::new(a),
                Ptr::<()>::new(b),
                Ptr::<()>::new(p)
            ],
        );
        assert_eq!(floats(&e, out, 3), [5.0, 0.0, 0.0]);
        // Equal end points: the first point, copied, with none of the vector helpers.
        log.borrow_mut().clear();
        put(&mut e, b, &[0.0, 0.0, 0.0]);
        put(&mut e, a, &[0.0, 0.0, 0.0]);
        e.mem.set_u32(a + 4, 0x8000_0000);
        e.mem.set_u32(b + 4, 0);
        put(&mut e, p, &[9.0, 9.0, 9.0]);
        e.call(
            0x00b3_ec40,
            &args![
                Ptr::<()>::NULL,
                Ptr::<()>::new(out),
                Ptr::<()>::new(a),
                Ptr::<()>::new(b),
                Ptr::<()>::new(p)
            ],
        );
        // -0.0 equals +0.0: the first point (with its sign bit) is copied.
        assert_eq!(e.mem.u32(out + 4), 0x8000_0000);
        assert!(log.borrow().is_empty());
        // A NaN end point is never equal to itself: the line path runs.
        put(&mut e, a, &[f32::NAN, 0.0, 0.0]);
        put(&mut e, b, &[f32::NAN, 0.0, 0.0]);
        e.call(
            0x00b3_ec40,
            &args![
                Ptr::<()>::NULL,
                Ptr::<()>::new(out),
                Ptr::<()>::new(a),
                Ptr::<()>::new(b),
                Ptr::<()>::new(p)
            ],
        );
        assert!(!log.borrow().is_empty());
    }

    /// A geometry with `points` vertices at the given coordinates.
    fn geometry_with(e: &mut Engine, points: &[[f32; 3]]) -> Ptr<CIndexedGeometry> {
        let geometry = e.new_object::<CIndexedGeometry>();
        let g = geometry.addr();
        let coords = e.mem.alloc(12 * points.len().max(1) as u32);
        for (i, point) in points.iter().enumerate() {
            put(e, coords + 12 * i as u32, point);
        }
        e.mem.set_u32(g + 0x88 + 0x0C, coords);
        e.mem
            .set_u32(g + 0x88 + 0x10, coords + 12 * points.len() as u32);
        e.mem.set_u16(g + 0x22, 7);
        e.register(COORD_POINTER, |e, a| {
            let first = e.mem.u32(a[0] + 0x88 + 0x0C);
            ret(first + 12 * a[1])
        });
        geometry
    }

    /// A shadow with the right axis (1, 0, 0) and the up axis (0, 1, 0).
    fn shadow(e: &mut Engine) -> Ptr<CProjectedShadow> {
        let shadow = e.new_object::<CProjectedShadow>();
        put(
            e,
            shadow.addr(),
            &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        );
        shadow
    }

    #[test]
    fn tex_coords_are_the_distances_along_the_two_axes() {
        let mut e = engine();
        let log = vec3_doubles(&mut e);
        let pairs: Rc<RefCell<Vec<(u32, Vec<f32>)>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = pairs.clone();
        e.register_double(ADD_TEX_COORD1, move |e, a| {
            sink.borrow_mut()
                .push((a[0], vec![e.mem.f32(a[1]), e.mem.f32(a[1] + 4)]));
            Ret::default()
        });
        let geometry = geometry_with(&mut e, &[[1.0, 0.0, 0.0], [-2.0, 2.0, 5.0]]);
        let shadow = shadow(&mut e);
        e.call(
            0x00b3_ea10,
            &args![
                shadow,
                geometry,
                0.0f32,
                0.0f32,
                0.0f32,
                2.0f32,
                Ptr::<()>::NULL
            ],
        );
        // Vertex 0: 3 along the right axis from its minus end, 2 along the up axis, each over
        // twice the radius (4); vertex 1: 0 along right, 4 along up.
        assert_eq!(
            *pairs.borrow(),
            vec![
                (geometry.addr(), vec![0.75, 0.5]),
                (geometry.addr(), vec![0.0, 1.0])
            ]
        );
        assert_eq!(e.mem.u16(geometry.addr() + 0x22), 0);
        // The four scalings come first, by the right axis then the up axis, with the radius.
        let calls = log.borrow().clone();
        let scales: Vec<&(u32, Vec<u32>)> = calls
            .iter()
            .filter(|(a, _)| *a == VEC3_SCALE)
            .take(4)
            .collect();
        assert_eq!(scales.len(), 4);
        assert_eq!(scales[0].1[0], shadow.addr());
        assert_eq!(scales[1].1[0], shadow.addr());
        assert_eq!(scales[2].1[0], shadow.addr() + 0x0C);
        assert_eq!(scales[3].1[0], shadow.addr() + 0x0C);
        assert!(scales.iter().all(|(_, w)| w[2] == 2.0f32.to_bits()));
        let order: Vec<u32> = calls.iter().take(8).map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![
                VEC3_SCALE,
                VEC3_SUBTRACT,
                VEC3_SCALE,
                VEC3_ADD,
                VEC3_SCALE,
                VEC3_SUBTRACT,
                VEC3_SCALE,
                VEC3_ADD
            ]
        );
    }

    #[test]
    fn tex_coords_are_mapped_through_the_rectangle() {
        let mut e = engine();
        vec3_doubles(&mut e);
        let pairs: Rc<RefCell<Vec<Vec<f32>>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = pairs.clone();
        e.register_double(ADD_TEX_COORD1, move |e, a| {
            sink.borrow_mut()
                .push(vec![e.mem.f32(a[1]), e.mem.f32(a[1] + 4)]);
            Ret::default()
        });
        let geometry = geometry_with(&mut e, &[[1.0, 0.0, 0.0]]);
        let shadow = shadow(&mut e);
        let rectangle = e.mem.alloc(0x18);
        put(&mut e, rectangle, &[10.0, 30.0, 4.0, 0.0, 0.0, 6.0]);
        e.call(
            0x00b3_ea10,
            &args![
                shadow,
                geometry,
                0.0f32,
                0.0f32,
                0.0f32,
                2.0f32,
                Ptr::<()>::new(rectangle)
            ],
        );
        // u = 0.75 * (10 - 4) + 4, v = 0.5 * (30 - 6) + 6.
        assert_eq!(*pairs.borrow(), vec![vec![8.5, 18.0]]);
    }

    #[test]
    fn a_geometry_without_vertices_gets_nothing() {
        let mut e = engine();
        vec3_doubles(&mut e);
        let added = Rc::new(RefCell::new(0));
        let sink = added.clone();
        e.register_double(ADD_TEX_COORD1, move |_, _| {
            *sink.borrow_mut() += 1;
            Ret::default()
        });
        let geometry = geometry_with(&mut e, &[]);
        let shadow = shadow(&mut e);
        e.call(
            0x00b3_ea10,
            &args![shadow, geometry, 0u32, 0u32, 0u32, 1.0f32, Ptr::<()>::NULL],
        );
        assert_eq!(*added.borrow(), 0);
        assert_eq!(e.mem.u16(geometry.addr() + 0x22), 0);
    }

    /// The doubles of the file readers and string helpers `Parse` uses; the chunk ids come from
    /// `chunks` in order, the vectors read are (1, 2, 3), (4, 5, 6), ...
    fn parse_doubles(e: &mut Engine, chunks: Vec<u32>) -> Calls {
        let log: Calls = Rc::new(RefCell::new(Vec::new()));
        let queue = Rc::new(RefCell::new(chunks.into_iter()));
        e.register_double(PARSE_INT, move |_, _| {
            ret(queue.borrow_mut().next().expect("no more chunks"))
        });
        let read = Rc::new(RefCell::new(0u32));
        let sink = log.clone();
        e.register_double(PARSE_VECTOR3, move |e, a| {
            sink.borrow_mut().push((PARSE_VECTOR3, a.to_vec()));
            let n = *read.borrow();
            *read.borrow_mut() += 1;
            for k in 0..3 {
                e.mem.set_f32(a[1] + 4 * k, (3 * n + k + 1) as f32);
            }
            ret(a[1])
        });
        for (address, result) in [
            (PARSE_STRING, 0u32),
            (FILENAME_CONSTRUCT, 0),
            (STRING_ASSIGN, 0),
            (STRING_TIDY, 0),
            (FORMAT_STRING, 0),
            (PARSE_ERROR_CONSTRUCT, 0),
        ] {
            let sink = log.clone();
            e.register_double(address, move |e, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                if address == PARSE_STRING {
                    for word in 0..7 {
                        e.mem.set_u32(a[1] + 4 * word, 0x600 + word);
                    }
                }
                if address == FORMAT_STRING {
                    return ret(a[0]);
                }
                ret(result)
            });
        }
        let sink = log.clone();
        e.register_double(FILENAME_NO_PATH, move |_, a| {
            sink.borrow_mut().push((FILENAME_NO_PATH, a.to_vec()));
            ret(a[1])
        });
        let sink = log.clone();
        e.register_double(CXX_THROW_EXCEPTION, move |_, a| {
            sink.borrow_mut().push((CXX_THROW_EXCEPTION, a.to_vec()));
            panic!("thrown");
        });
        log
    }

    #[test]
    fn parse_reads_the_axes_and_the_shadow_map_name() {
        let mut e = engine();
        e.set_global(STRING_NPOS, 0xFFFF_FFFFu32);
        let log = parse_doubles(&mut e, vec![0x4652, 0x4653, 0x4654, 0x4655, 0x4651]);
        let shadow = e.new_object::<CProjectedShadow>();
        let file = e.mem.alloc(0x20);
        e.call(0x00b3_e750, &args![shadow, Ptr::<()>::new(file)]);
        let s = shadow.addr();
        assert_eq!(floats(&e, s, 3), [1.0, 2.0, 3.0]);
        assert_eq!(floats(&e, s + 0x0C, 3), [4.0, 5.0, 6.0]);
        assert_eq!(floats(&e, s + 0x18, 3), [7.0, 8.0, 9.0]);
        let calls = log.borrow().clone();
        let at = |address: u32| calls.iter().position(|(a, _)| *a == address).unwrap();
        // The string: read into an argument area, handed by value to the filename constructor
        // with the filename object first; the shortened name assigned to the shadow's map name.
        let name = calls[at(PARSE_STRING)].1[1];
        let construct = &calls[at(FILENAME_CONSTRUCT)].1;
        assert_eq!(
            construct[1..],
            [0x600, 0x601, 0x602, 0x603, 0x604, 0x605, 0x606]
        );
        let filename = construct[0];
        assert_ne!(filename, name);
        let no_path = calls[at(FILENAME_NO_PATH)].1.clone();
        assert_eq!(no_path[0], filename);
        assert_eq!(
            calls[at(STRING_ASSIGN)].1,
            vec![s + 0x24, no_path[1], 0, 0xFFFF_FFFF]
        );
        let tidies: Vec<Vec<u32>> = calls
            .iter()
            .filter(|(a, _)| *a == STRING_TIDY)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(tidies, vec![vec![no_path[1], 1, 0], vec![filename, 1, 0]]);
    }

    #[test]
    fn parse_throws_for_an_unknown_chunk() {
        let mut e = engine();
        let log = parse_doubles(&mut e, vec![0x4652, 0x1234]);
        let shadow = e.new_object::<CProjectedShadow>();
        let file = e.mem.alloc(0x20);
        assert!(panics(|| {
            e.call(0x00b3_e750, &args![shadow, Ptr::<()>::new(file)]);
        }));
        let calls = log.borrow().clone();
        let format = calls.iter().find(|(a, _)| *a == FORMAT_STRING).unwrap();
        assert_eq!(format.1[1..], [UNKNOWN_CHUNK_FORMAT, 0x1234]);
        let error = calls
            .iter()
            .find(|(a, _)| *a == PARSE_ERROR_CONSTRUCT)
            .unwrap();
        // The error is built from the formatted text, errno not used.
        assert_eq!(error.1[1], format.1[0]);
        assert_eq!(error.1[2], 0);
        let thrown = calls
            .iter()
            .find(|(a, _)| *a == CXX_THROW_EXCEPTION)
            .unwrap();
        assert_eq!(thrown.1, vec![error.1[0], PARSE_THROW_INFO]);
        // The end marker as the first id is unknown too (the loop's test comes after a chunk).
        let mut e = engine();
        let log = parse_doubles(&mut e, vec![0x4651]);
        let shadow = e.new_object::<CProjectedShadow>();
        assert!(panics(|| {
            e.call(0x00b3_e750, &args![shadow, Ptr::<()>::new(file)]);
        }));
        let calls = log.borrow().clone();
        let format = calls.iter().find(|(a, _)| *a == FORMAT_STRING).unwrap();
        assert_eq!(format.1[2], 0x4651);
    }
}
