//! `Havok SDK/hkpwheelconstraintdata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpconvexradiusbuilder::{
    mulps, normalising_scale, shufps, splat, subps, sum_of_three,
};
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, load_vec, store_bits, store_vec, Vec4, HALF_F32, THREE_F32, ZERO_F32,
};

// hkpWheelConstraintData has no type in the Xbox PDB, so its layout is described by the offsets
// the PC code uses (roles unconfirmed): `+0x10` the constraint atoms (`fn_00d4f000` fills their
// types), `+0x20` .. `+0x50` a basis (three vectors), `+0x50` and `+0x90` the pivots in body A
// and body B, `+0x60` an axle, `+0x70` a unit vector perpendicular to it and `+0x80` their cross
// product, `+0xa4` and `+0xa8` two floats (the constructor sets `-0.5` and `0.5`), `+0xd0`,
// `+0xe0` and `+0xf0` a second basis (the third vector the cross product of the first two),
// `+0x100`, `+0x110` and `+0x120` a third basis built the same way, `+0x140` and `+0x150` copies
// of its first two vectors.

/// `hkpConstraintData::getConstraintInfoUtil` (cdecl): the atoms, their size and the info.
const GET_CONSTRAINT_INFO_UTIL: u32 = 0x00d2_1a60;
/// `00d4f000`'s unreachable `memset` (cdecl): the pointer, the value and the size.
const MEMSET: u32 = 0x00ec_61c0;
/// A vector helper (`ECX` the output, then a transform and a vector): rotates the vector by the
/// transform's inverse rotation (role unconfirmed).
const ROTATE_INVERSE: u32 = 0x004b_4ab0;
/// `hkRotation::isOrthonormal` (`ECX` the rotation, then a pointer to the result byte and the
/// epsilon; returns that pointer).
const IS_ORTHONORMAL: u32 = 0x00cb_2e70;
/// The `float` `-0.5` (`.rdata`) the constructor stores at `+0xa4`.
const NEGATIVE_HALF: u32 = 0x0102_295c;
/// The `float` epsilon (`.rdata`) `isValid` passes to `isOrthonormal`.
const ORTHONORMAL_EPSILON: u32 = 0x0101_7d00;
/// The four floats (`.data`) `isValid` uses as the translation of its two local transforms.
const TRANSLATION_CONSTANT: u32 = 0x0126_7e30;
/// The constructor's vtable (`.rdata`).
const VTABLE: u32 = 0x010d_0a64;

/// The constants the normalisation reads from `.rdata`: zero, three and a half.
fn normalise_constants(e: &Engine) -> [f32; 3] {
    [
        e.mem.f32(ZERO_F32),
        e.mem.f32(THREE_F32),
        e.mem.f32(HALF_F32),
    ]
}

/// Writes `source` (a vector in memory) scaled by the refined `RSQRTSS` inverse length of its
/// first three lanes (zero for a zero vector) to `target`, all four lanes scaled.
fn normalise_into(e: &mut Engine, constants: [f32; 3], source: u32, target: u32) {
    let [zero, three, half] = constants;
    let v = load_vec(e, source);
    let scale = normalising_scale(sum_of_three(mulps(v, v)), zero, half, three);
    store_vec(e, target, mulps(splat(scale), v));
}

/// `a x b` the way the exe's `SHUFPS`/`MULPS`/`SUBPS` sequence builds it:
/// `a.yzx * b.zxy - a.zxy * b.yzx`.
fn cross(a: Vec4, b: Vec4) -> Vec4 {
    subps(
        mulps(shufps(a, a, 0xc9), shufps(b, b, 0xd2)),
        mulps(shufps(a, a, 0xd2), shufps(b, b, 0xc9)),
    )
}

/// `|source[lane]|` through the x87 stack (`FLD`, `FABS`, `FSTP`).
fn magnitude(e: &Engine, source: u32, lane: u32) -> f32 {
    f32::from_bits(x87_quiet_bits(e.mem.u32(source + 4 * lane)) & 0x7fff_ffff)
}

/// Fills `target` with a vector perpendicular to the first three lanes of `source`: zero, then
/// `target[low] = source[high]` and `target[high] = -source[low]`, where `low` and `high` pick
/// the two components that are not the smallest in magnitude (`low` is `0` when `|x| > |y|`
/// else `1`; `high` is `2` unless the smaller of `|x|` and `|y|` is above `|z|`, when it is the
/// other of `0` and `1`).
fn write_perpendicular(e: &mut Engine, source: u32, target: u32) {
    let x = magnitude(e, source, 0);
    let y = magnitude(e, source, 1);
    let z = magnitude(e, source, 2);
    let (smallest, low, other) = if x > y { (y, 0, 1) } else { (x, 1, 0) };
    let high = if smallest > z { other } else { 2 };
    store_bits(e, target, [0; 4]);
    let moved = x87_quiet_bits(e.mem.u32(source + 4 * high));
    e.mem.set_u32(target + 4 * low, moved);
    let negated = x87_quiet_bits(e.mem.u32(source + 4 * low)) ^ 0x8000_0000;
    e.mem.set_u32(target + 4 * high, negated);
}

/// The basis at `+0x20` (the three unit vectors; the 12 words cleared first).
fn set_identity_basis(e: &mut Engine, base: u32) {
    store_bits(e, base, [0; 4]);
    store_bits(e, base + 0x10, [0; 4]);
    store_bits(e, base + 0x20, [0; 4]);
    e.mem.set_u32(base, 0x3f80_0000);
    e.mem.set_u32(base + 0x14, 0x3f80_0000);
    e.mem.set_u32(base + 0x28, 0x3f80_0000);
}

// Translated from 00d4e560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWheelConstraintData::getConstraintInfo` (Xbox PDB name, engine map). Thiscall, one stack
/// word (`RET 4`): `info`. Calls `getConstraintInfoUtil` (`00d21a60`, cdecl) with the atoms
/// (`this + 0x10`), their size `0x124` and `info`.
pub fn hkp_wheel_constraint_data_get_constraint_info(e: &mut Engine, this: Ptr, info: u32) {
    e.call(
        GET_CONSTRAINT_INFO_UTIL,
        &args![this.addr().wrapping_add(0x10), 0x124u32, info],
    );
}

// Translated from 00d4e580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWheelConstraintData::getRuntimeInfo` (Xbox PDB name, engine map). Two stack words
/// (`RET 8`; `ECX` is not read): `want` (its low byte) and `info`. Writes `info[1] = 6` and
/// `info[0] = 0x30` when the byte is non-zero, else both zero.
pub fn hkp_wheel_constraint_data_get_runtime_info(e: &mut Engine, _this: Ptr, want: u8, info: u32) {
    if want != 0 {
        e.mem.set_u32(info + 4, 6);
        e.mem.set_u32(info, 0x30);
    } else {
        e.mem.set_u32(info + 4, 0);
        e.mem.set_u32(info, 0);
    }
}

// Translated from 00d4e5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWheelConstraintData::getType` (Xbox PDB name, engine map): `9` (`ECX` is not read).
pub fn hkp_wheel_constraint_data_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    9
}

// Translated from 00d4e5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, six stack words (`RET 0x18`; the engine map has no name for it, and its body is
/// the world-space setter of the class): `transform_a` and `transform_b` (transforms, the
/// translation at `+0x30`), `pivot`, `first` and `second` (vectors, each normalised into a local
/// copy) and `axle` (a vector). Rotates (`004b4ab0`, `ECX` the output) the pivot minus each
/// translation into `+0x50` and `+0x90`, sets the basis at `+0x20`, rotates `axle` by the second
/// transform into `+0x60`, builds the perpendicular (`write_perpendicular`) at `+0x70`
/// normalised and `+0x80` = `axle x perpendicular`, rotates the normalised `first` by the first
/// and second transform into `+0xd0` and `+0x100` (a copy at `+0x140`), the normalised `second`
/// into `+0xe0` and `+0x110` (a copy at `+0x150`) and stores `+0xf0 = +0xd0 x +0xe0` and
/// `+0x120 = +0x100 x +0x110`. The local vector the third rotation clears is not read again.
#[allow(clippy::too_many_arguments)]
pub fn fn_00d4e5c0(
    e: &mut Engine,
    this: Ptr,
    transform_a: u32,
    transform_b: u32,
    pivot: u32,
    first: u32,
    axle: u32,
    second: u32,
) {
    let t = this.addr();
    let constants = normalise_constants(e);
    e.with_stack(0x40, |e, frame| {
        let f = frame.addr();
        let (first_local, second_local, scratch) = (f, f + 0x10, f + 0x20);
        normalise_into(e, constants, first, first_local);
        normalise_into(e, constants, second, second_local);
        for (output, transform) in [(t + 0x50, transform_a), (t + 0x90, transform_b)] {
            let moved = subps(load_vec(e, pivot), load_vec(e, transform + 0x30));
            store_vec(e, scratch, moved);
            e.call(ROTATE_INVERSE, &args![output, transform, scratch]);
        }
        set_identity_basis(e, t + 0x20);
        store_bits(e, scratch, [0; 4]);
        e.call(ROTATE_INVERSE, &args![t + 0x60, transform_b, axle]);
        write_perpendicular(e, t + 0x60, t + 0x70);
        normalise_into(e, constants, t + 0x70, t + 0x70);
        let perpendicular_cross = cross(load_vec(e, t + 0x60), load_vec(e, t + 0x70));
        store_vec(e, t + 0x80, perpendicular_cross);
        e.call(ROTATE_INVERSE, &args![t + 0xd0, transform_a, first_local]);
        e.call(ROTATE_INVERSE, &args![t + 0x100, transform_b, first_local]);
        copy_vec(e, t + 0x100, t + 0x140);
        e.call(ROTATE_INVERSE, &args![t + 0xe0, transform_a, second_local]);
        e.call(ROTATE_INVERSE, &args![t + 0x110, transform_b, second_local]);
        copy_vec(e, t + 0x110, t + 0x150);
        let upper = cross(load_vec(e, t + 0xd0), load_vec(e, t + 0xe0));
        store_vec(e, t + 0xf0, upper);
        let lower = cross(load_vec(e, t + 0x100), load_vec(e, t + 0x110));
        store_vec(e, t + 0x120, lower);
    });
}

// Translated from 00d4e960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWheelConstraintData::setInBodySpace` (Xbox PDB name, engine map). Thiscall, six stack
/// words (`RET 0x18`): `pivot_a`, `pivot_b`, `first`, `second`, `axle` and `third` (pointers to
/// vectors). Copies the pivots to `+0x50` and `+0x90`, sets the basis at `+0x20`, copies `axle`
/// to `+0x60`, builds the perpendicular (`write_perpendicular`) at `+0x70` normalised and `+0x80`
/// = `axle x perpendicular`, normalises `first` into `+0xd0`, `second` into `+0x100` and
/// `+0x140`, builds the perpendicular of `+0xd0` at `+0xe0` normalised, normalises `third` into
/// `+0x110` and `+0x150` and stores `+0xf0 = +0xd0 x +0xe0` and `+0x120 = +0x100 x +0x110`.
#[allow(clippy::too_many_arguments)]
pub fn hkp_wheel_constraint_data_set_in_body_space(
    e: &mut Engine,
    this: Ptr,
    pivot_a: u32,
    pivot_b: u32,
    first: u32,
    second: u32,
    axle: u32,
    third: u32,
) {
    let t = this.addr();
    copy_vec(e, pivot_a, t + 0x50);
    copy_vec(e, pivot_b, t + 0x90);
    set_identity_basis(e, t + 0x20);
    copy_vec(e, axle, t + 0x60);
    write_perpendicular(e, t + 0x60, t + 0x70);
    let constants = normalise_constants(e);
    normalise_into(e, constants, t + 0x70, t + 0x70);
    let perpendicular_cross = cross(load_vec(e, t + 0x60), load_vec(e, t + 0x70));
    store_vec(e, t + 0x80, perpendicular_cross);
    normalise_into(e, constants, first, t + 0xd0);
    normalise_into(e, constants, second, t + 0x100);
    copy_vec(e, t + 0x100, t + 0x140);
    write_perpendicular(e, t + 0xd0, t + 0xe0);
    normalise_into(e, constants, t + 0xe0, t + 0xe0);
    copy_vec(e, third, t + 0x110);
    normalise_into(e, constants, t + 0x110, t + 0x110);
    copy_vec(e, t + 0x110, t + 0x150);
    let upper = cross(load_vec(e, t + 0xd0), load_vec(e, t + 0xe0));
    store_vec(e, t + 0xf0, upper);
    let lower = cross(load_vec(e, t + 0x100), load_vec(e, t + 0x110));
    store_vec(e, t + 0x120, lower);
}

// Translated from 00d4edc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWheelConstraintData::hkpWheelConstraintData` (Xbox PDB name, engine map). Fastcall, no
/// stack word. Stores `1` as a 16-bit word at `+6`, `0` at `+8`, the vtable `010d0a64`, fills the
/// atoms (`fn_00d4f000` on `this + 0x10`), the floats `-0.5` (`+0xa4`, from `.rdata`) and the
/// half constant (`+0xa8`) through the x87 stack, the bytes `+0xa2` and `+0xae` zero, `+0xba`
/// `1`, `+0xbe` `2`, zero at `+0xb0` and `+0xb4`, the unit bases at `+0xd0`, `+0x100`, `+0x20`
/// (each three vectors), `+0x60` (the unit matrix over `+0x60`, `+0x70` and `+0x80`) and zero at
/// `+0x50`, `+0x90` and the byte `+0x132`. Returns `this`.
pub fn hkp_wheel_constraint_data_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    e.mem.set_u16(t + 6, 1);
    e.mem.set_u32(t + 8, 0);
    e.mem.set_u32(t, VTABLE);
    fn_00d4f000(e, Ptr::new(t + 0x10));
    let negative_half = x87_quiet_bits(e.mem.u32(NEGATIVE_HALF));
    e.mem.set_u32(t + 0xa4, negative_half);
    e.mem.set_u8(t + 0xa2, 0);
    let half = x87_quiet_bits(e.mem.u32(HALF_F32));
    e.mem.set_u8(t + 0xae, 0);
    e.mem.set_u32(t + 0xa8, half);
    e.mem.set_u8(t + 0xba, 1);
    e.mem.set_u8(t + 0xbe, 2);
    e.mem.set_u32(t + 0xb0, 0);
    e.mem.set_u32(t + 0xb4, 0);
    set_identity_basis(e, t + 0xd0);
    set_identity_basis(e, t + 0x100);
    set_identity_basis(e, t + 0x20);
    store_bits(e, t + 0x50, [0; 4]);
    store_bits(e, t + 0x60, [0; 4]);
    store_bits(e, t + 0x70, [0; 4]);
    store_bits(e, t + 0x80, [0; 4]);
    e.mem.set_u32(t + 0x60, 0x3f80_0000);
    e.mem.set_u32(t + 0x74, 0x3f80_0000);
    e.mem.set_u32(t + 0x88, 0x3f80_0000);
    store_bits(e, t + 0x90, [0; 4]);
    e.mem.set_u8(t + 0x132, 0);
    this
}

// Translated from 00d4eec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWheelConstraintData::isValid` (Xbox PDB name, engine map). Thiscall, one stack word
/// (`RET 4`): `out`, a byte pointer that receives `1` or `0`. Builds two local transforms from
/// the bases at `+0xd0` and `+0x100` (translation the four floats at `01267e30`) and succeeds
/// when `hkRotation::isOrthonormal` (`00cb2e70`, with the epsilon at `01017d00`) accepts, in
/// order, the rotations at `this + 0x20`, `this + 0x60` and the two local ones, and the float at
/// `+0xa4` is not above the float at `+0xa8` (false for a NaN).
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn hkp_wheel_constraint_data_is_valid(e: &mut Engine, this: Ptr, out: u32) -> u32 {
    let t = this.addr();
    e.with_stack(0x90, |e, frame| {
        let f = frame.addr();
        for row in 0..3u32 {
            copy_vec(e, t + 0xd0 + 0x10 * row, f + 0x08 + 0x10 * row);
            copy_vec(e, t + 0x100 + 0x10 * row, f + 0x48 + 0x10 * row);
        }
        for lane in 0..4u32 {
            let word = e.mem.u32(TRANSLATION_CONSTANT + 4 * lane);
            e.mem.set_u32(f + 0x38 + 4 * lane, word);
            e.mem.set_u32(f + 0x78 + 4 * lane, word);
        }
        let epsilon = x87_quiet_bits(e.mem.u32(ORTHONORMAL_EPSILON));
        let checks = [
            (t + 0x20, f + 7),
            (t + 0x60, f + 6),
            (f + 0x08, f + 4),
            (f + 0x48, f + 5),
        ];
        let mut valid = true;
        for (rotation, result_slot) in checks {
            let result = e
                .call(IS_ORTHONORMAL, &args![rotation, result_slot, epsilon])
                .u32();
            if e.mem.u8(result) == 0 {
                valid = false;
                break;
            }
        }
        if valid {
            valid = e.mem.f32(t + 0xa4) <= e.mem.f32(t + 0xa8);
        }
        e.mem.set_u8(out, u8::from(valid));
    });
    out
}

// Translated from 00d4f000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall, no stack word (the engine map has no name for it; the constructor calls it with
/// `this + 0x10`): stores the 16-bit words `2` at `+0`, `9` at `+0x90`, `8` at `+0x9c`, `7` at
/// `+0xa8` and `+0xac`, `4` at `+0xb0` and `0xc` at `+0x120`. The exe then compares the
/// address `+0xb0` with itself and, if it were below, clears the range between the two with
/// `memset` (`00ec61c0`); it never is, the call is unreachable. Returns `this`.
pub fn fn_00d4f000(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    e.mem.set_u16(t, 2);
    e.mem.set_u16(t + 0x90, 9);
    e.mem.set_u16(t + 0x9c, 8);
    e.mem.set_u16(t + 0xa8, 7);
    e.mem.set_u16(t + 0xac, 7);
    let start = t.wrapping_add(0xb0);
    let end = t.wrapping_add(0xb0);
    e.mem.set_u16(start, 4);
    e.mem.set_u16(t + 0x120, 0xc);
    if start < end {
        e.call(MEMSET, &args![start, 0u32, end.wrapping_sub(start)]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d4_e560,
            hkp_wheel_constraint_data_get_constraint_info(Ptr, u32)
        ),
        entry!(
            0x00d4_e580,
            hkp_wheel_constraint_data_get_runtime_info(Ptr, u8, u32)
        ),
        entry!(0x00d4_e5b0, hkp_wheel_constraint_data_get_type(Ptr) -> u32),
        entry!(0x00d4_e5c0, fn_00d4e5c0(Ptr, u32, u32, u32, u32, u32, u32)),
        entry!(
            0x00d4_e960,
            hkp_wheel_constraint_data_set_in_body_space(Ptr, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x00d4_edc0, hkp_wheel_constraint_data_constructor(Ptr) -> Ptr),
        entry!(0x00d4_eec0, hkp_wheel_constraint_data_is_valid(Ptr, u32) -> u32),
        entry!(0x00d4_f000, fn_00d4f000(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::load_bits;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// An engine with the exe's data range mapped and an empty call log.
    fn setup() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f32(ZERO_F32, 0.0);
        e.mem.set_f32(THREE_F32, 3.0);
        e.mem.set_f32(HALF_F32, 0.5);
        (e, Rc::new(RefCell::new(Vec::new())))
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// Whether the vector at `address` is within `1e-4` of `expected` in every lane.
    fn near(e: &Engine, address: u32, expected: [f32; 4]) -> bool {
        load_vec(e, address)
            .iter()
            .zip(expected)
            .all(|(got, want)| (got - want).abs() < 1e-4)
    }

    /// Doubles `004b4ab0`: copies the vector given as the third word to the output (`ECX`) and
    /// logs the words and the vector's bits.
    fn rotate_double(e: &mut Engine, log: &Log) {
        let sink = Rc::clone(log);
        e.register_double(ROTATE_INVERSE, move |e, a| {
            let mut logged = a.to_vec();
            logged.extend(load_bits(e, a[2]));
            sink.borrow_mut().push((ROTATE_INVERSE, logged));
            copy_vec(e, a[2], a[0]);
            Ret::default()
        });
    }

    fn put(e: &mut Engine, vector: [f32; 4]) -> u32 {
        let address = e.mem.alloc(0x10);
        store_vec(e, address, vector);
        address
    }

    fn bits(vector: [f32; 4]) -> [u32; 4] {
        vector.map(f32::to_bits)
    }

    #[test]
    fn get_constraint_info_passes_the_atoms() {
        let (mut e, log) = setup();
        let sink = Rc::clone(&log);
        e.register_double(GET_CONSTRAINT_INFO_UTIL, move |_, a| {
            sink.borrow_mut()
                .push((GET_CONSTRAINT_INFO_UTIL, a.to_vec()));
            Ret::default()
        });
        e.call(0x00d4_e560, &args![0x1000u32, 0x77u32]);
        assert_eq!(
            calls_to(&log, GET_CONSTRAINT_INFO_UTIL),
            vec![vec![0x1010, 0x124, 0x77]]
        );
    }

    #[test]
    fn get_runtime_info_reports_the_solver_size_when_asked() {
        let (mut e, _) = setup();
        let info = e.mem.alloc(8);
        e.mem.set_u32(info, 0x11);
        e.mem.set_u32(info + 4, 0x22);
        e.call(0x00d4_e580, &args![0u32, 1u32, info]);
        assert_eq!((e.mem.u32(info), e.mem.u32(info + 4)), (0x30, 6));
        e.call(0x00d4_e580, &args![0u32, 0u32, info]);
        assert_eq!((e.mem.u32(info), e.mem.u32(info + 4)), (0, 0));
    }

    #[test]
    fn get_type_is_nine() {
        let (mut e, _) = setup();
        assert_eq!(e.call(0x00d4_e5b0, &args![0u32]).u32(), 9);
    }

    #[test]
    fn fn_00d4e5c0_sets_up_the_world_space_data() {
        let (mut e, log) = setup();
        rotate_double(&mut e, &log);
        let this = e.mem.alloc(0x180);
        let transform_a = e.mem.alloc(0x40);
        let transform_b = e.mem.alloc(0x40);
        store_vec(&mut e, transform_a + 0x30, [1.0, 2.0, 3.0, 0.0]);
        store_vec(&mut e, transform_b + 0x30, [2.0, 4.0, 6.0, 0.0]);
        let pivot = put(&mut e, [10.0, 20.0, 30.0, 0.0]);
        let first = put(&mut e, [0.0, 0.0, 2.0, 0.0]);
        let axle = put(&mut e, [1.0, 2.0, -3.0, 0.0]);
        let second = put(&mut e, [3.0, 0.0, 0.0, 0.0]);
        e.call(
            0x00d4_e5c0,
            &args![this, transform_a, transform_b, pivot, first, axle, second],
        );
        let calls = calls_to(&log, ROTATE_INVERSE);
        assert_eq!(calls.len(), 7);
        assert_eq!(calls[0][..2], [this + 0x50, transform_a]);
        assert_eq!(calls[0][3..], bits([9.0, 18.0, 27.0, 0.0]));
        assert_eq!(calls[1][..2], [this + 0x90, transform_b]);
        assert_eq!(calls[1][3..], bits([8.0, 16.0, 24.0, 0.0]));
        assert_eq!(
            calls[2],
            [
                vec![this + 0x60, transform_b, axle],
                bits([1.0, 2.0, -3.0, 0.0]).to_vec()
            ]
            .concat()
        );
        assert_eq!(calls[3][..2], [this + 0xd0, transform_a]);
        assert_eq!(calls[4][..2], [this + 0x100, transform_b]);
        assert_eq!(calls[5][..2], [this + 0xe0, transform_a]);
        assert_eq!(calls[6][..2], [this + 0x110, transform_b]);
        assert_eq!(calls[3][2], calls[4][2]);
        assert_eq!(calls[5][2], calls[6][2]);
        assert!(near(&e, this + 0xd0, [0.0, 0.0, 1.0, 0.0]));
        assert!(near(&e, this + 0x100, [0.0, 0.0, 1.0, 0.0]));
        assert!(near(&e, this + 0x140, [0.0, 0.0, 1.0, 0.0]));
        assert!(near(&e, this + 0xe0, [1.0, 0.0, 0.0, 0.0]));
        assert!(near(&e, this + 0x110, [1.0, 0.0, 0.0, 0.0]));
        assert!(near(&e, this + 0x150, [1.0, 0.0, 0.0, 0.0]));
        assert!(near(&e, this + 0xf0, [0.0, 1.0, 0.0, 0.0]));
        assert!(near(&e, this + 0x120, [0.0, 1.0, 0.0, 0.0]));
        assert_eq!(load_vec(&e, this + 0x20), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, this + 0x30), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, this + 0x40), [0.0, 0.0, 1.0, 0.0]);
        let s = 1.0 / 13.0f32.sqrt();
        assert!(near(&e, this + 0x70, [0.0, -3.0 * s, -2.0 * s, 0.0]));
        assert!(near(
            &e,
            this + 0x80,
            [-13.0f32.sqrt(), 0.5547, -0.83205, 0.0]
        ));
    }

    #[test]
    fn set_in_body_space_builds_the_bases() {
        let (mut e, _) = setup();
        let this = e.mem.alloc(0x180);
        let pivot_a = put(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let pivot_b = put(&mut e, [5.0, 6.0, 7.0, 8.0]);
        let first = put(&mut e, [0.0, 0.0, 2.0, 0.0]);
        let second = put(&mut e, [3.0, 0.0, 0.0, 0.0]);
        let axle = put(&mut e, [1.0, 2.0, -3.0, 0.0]);
        let third = put(&mut e, [0.0, 0.0, 5.0, 0.0]);
        e.call(
            0x00d4_e960,
            &args![this, pivot_a, pivot_b, first, second, axle, third],
        );
        assert_eq!(load_vec(&e, this + 0x50), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, this + 0x90), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(load_vec(&e, this + 0x20), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, this + 0x60), [1.0, 2.0, -3.0, 0.0]);
        let s = 1.0 / 13.0f32.sqrt();
        assert!(near(&e, this + 0x70, [0.0, -3.0 * s, -2.0 * s, 0.0]));
        assert!(near(
            &e,
            this + 0x80,
            [-13.0f32.sqrt(), 0.5547, -0.83205, 0.0]
        ));
        assert!(near(&e, this + 0xd0, [0.0, 0.0, 1.0, 0.0]));
        assert!(near(&e, this + 0x100, [1.0, 0.0, 0.0, 0.0]));
        assert!(near(&e, this + 0x140, [1.0, 0.0, 0.0, 0.0]));
        assert!(near(&e, this + 0xe0, [0.0, 1.0, 0.0, 0.0]));
        assert!(near(&e, this + 0x110, [0.0, 0.0, 1.0, 0.0]));
        assert!(near(&e, this + 0x150, [0.0, 0.0, 1.0, 0.0]));
        assert!(near(&e, this + 0xf0, [-1.0, 0.0, 0.0, 0.0]));
        assert!(near(&e, this + 0x120, [0.0, -1.0, 0.0, 0.0]));
    }

    #[test]
    fn constructor_sets_the_defaults() {
        let (mut e, log) = setup();
        e.mem.set_f32(NEGATIVE_HALF, -0.5);
        log_memset(&mut e, &log);
        let this = e.mem.alloc(0x140);
        for word in 0..0x50u32 {
            e.mem.set_u32(this + 4 * word, 0xaaaa_aaaa);
        }
        let got = e.call(0x00d4_edc0, &args![this]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this), 0x010d_0a64);
        assert_eq!(e.mem.u16(this + 0x10), 2);
        assert_eq!(e.mem.u16(this + 0xc0), 4);
        assert_eq!(e.mem.u16(this + 0x130), 0xc);
        assert_eq!(e.mem.f32(this + 0xa4), -0.5);
        assert_eq!(e.mem.f32(this + 0xa8), 0.5);
        assert_eq!(e.mem.u8(this + 0xa2), 0);
        assert_eq!(e.mem.u8(this + 0xae), 0);
        assert_eq!(e.mem.u8(this + 0xba), 1);
        assert_eq!(e.mem.u8(this + 0xbe), 2);
        assert_eq!(e.mem.u32(this + 0xb0), 0);
        assert_eq!(e.mem.u32(this + 0xb4), 0);
        assert_eq!(e.mem.u8(this + 0x132), 0);
        for base in [0x20u32, 0xd0, 0x100] {
            assert_eq!(load_vec(&e, this + base), [1.0, 0.0, 0.0, 0.0]);
            assert_eq!(load_vec(&e, this + base + 0x10), [0.0, 1.0, 0.0, 0.0]);
            assert_eq!(load_vec(&e, this + base + 0x20), [0.0, 0.0, 1.0, 0.0]);
        }
        assert_eq!(load_vec(&e, this + 0x50), [0.0; 4]);
        assert_eq!(load_vec(&e, this + 0x60), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, this + 0x70), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, this + 0x80), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(load_vec(&e, this + 0x90), [0.0; 4]);
        assert!(calls_to(&log, MEMSET).is_empty());
    }

    /// A logging double for the unreachable `memset`.
    fn log_memset(e: &mut Engine, log: &Log) {
        let sink = Rc::clone(log);
        e.register_double(MEMSET, move |_, a| {
            sink.borrow_mut().push((MEMSET, a.to_vec()));
            Ret::default()
        });
    }

    #[test]
    fn is_valid_needs_every_rotation_and_ordered_limits() {
        let run = |verdicts: [u8; 4], low: f32, high: f32| -> (u8, Vec<Vec<u32>>) {
            let (mut e, log) = setup();
            e.mem.set_u32(ORTHONORMAL_EPSILON, 0x3a83_126f);
            for (lane, word) in [0x41u32, 0x42, 0x43, 0x44].into_iter().enumerate() {
                e.mem.set_u32(TRANSLATION_CONSTANT + 4 * lane as u32, word);
            }
            let this = e.mem.alloc(0x140);
            for (row, base) in [0xd0u32, 0xe0, 0xf0, 0x100, 0x110, 0x120]
                .into_iter()
                .enumerate()
            {
                e.mem.set_u32(this + base, 0x30 + row as u32);
            }
            e.mem.set_f32(this + 0xa4, low);
            e.mem.set_f32(this + 0xa8, high);
            let sink = Rc::clone(&log);
            let mut count = 0usize;
            e.register_double(IS_ORTHONORMAL, move |e, a| {
                let mut logged = a.to_vec();
                logged.extend([0u32, 0x10, 0x20, 0x30].map(|offset| e.mem.u32(a[0] + offset)));
                sink.borrow_mut().push((IS_ORTHONORMAL, logged));
                e.mem.set_u8(a[1], verdicts[count]);
                count += 1;
                Ret {
                    eax: a[1],
                    ..Ret::default()
                }
            });
            let out = e.mem.alloc(4);
            e.mem.set_u8(out, 0x77);
            e.call(0x00d4_eec0, &args![this, out]);
            let seen = calls_to(&log, IS_ORTHONORMAL);
            (e.mem.u8(out), seen)
        };
        let (valid, seen) = run([1, 1, 1, 1], -0.5, 0.5);
        assert_eq!(valid, 1);
        assert_eq!(seen.len(), 4);
        assert_eq!(seen[0][2], 0x3a83_126f);
        assert_eq!(seen[2][3..], [0x30, 0x31, 0x32, 0x41]);
        assert_eq!(seen[3][3..], [0x33, 0x34, 0x35, 0x41]);
        assert_eq!(seen[2][1], seen[2][0] - 0x08 + 4);
        let (valid, seen) = run([1, 1, 0, 1], -0.5, 0.5);
        assert_eq!(valid, 0);
        assert_eq!(seen.len(), 3);
        assert_eq!(run([1, 1, 1, 1], 0.5, -0.5).0, 0);
        assert_eq!(run([1, 1, 1, 1], f32::NAN, 0.5).0, 0);
        assert_eq!(run([1, 1, 1, 1], 0.25, 0.25).0, 1);
    }

    #[test]
    fn fn_00d4eec0_answers_its_result_pointer() {
        let (mut e, _) = setup();
        e.register_double(IS_ORTHONORMAL, |e, a| {
            e.mem.set_u8(a[1], 0);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x140);
        let out = e.mem.alloc(4);
        // EAX is `out` at both RETs (the hkBool result pointer the callers read through).
        assert_eq!(e.call(0x00d4_eec0, &args![this, out]).u32(), out);
    }

    #[test]
    fn fn_00d4f000_stores_the_atom_types() {
        let (mut e, log) = setup();
        log_memset(&mut e, &log);
        let this = e.mem.alloc(0x140);
        let got = e.call(0x00d4_f000, &args![this]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u16(this), 2);
        assert_eq!(e.mem.u16(this + 0x90), 9);
        assert_eq!(e.mem.u16(this + 0x9c), 8);
        assert_eq!(e.mem.u16(this + 0xa8), 7);
        assert_eq!(e.mem.u16(this + 0xac), 7);
        assert_eq!(e.mem.u16(this + 0xb0), 4);
        assert_eq!(e.mem.u16(this + 0x120), 0xc);
        assert!(calls_to(&log, MEMSET).is_empty());
    }
}
