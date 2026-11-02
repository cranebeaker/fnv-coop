//! `Havok SDK/run_00e520b0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::run_00d6bf60::{andps, fcom_status, load_vec};
use crate::units::havok_sdk::run_00e499d0::fistp_truncate_i32;

/// The `double` the four components are multiplied by (`010a5798`).
const NIBBLE_SCALE: u32 = 0x010a_5798;
/// The vector that clears the sign bit of every lane (`01017d10`).
const ABSOLUTE_MASK: u32 = 0x0101_7d10;
/// `sqrt` of the x87 stack top (`00ec6040`, the argument as a leading `double`).
const X87_SQRT: u32 = 0x00ec_6040;
/// The `double` the square root is taken of, and the one it is multiplied by (`01011590`,
/// `010ec930`), to make the static scale.
const ROOT_ARGUMENT: u32 = 0x0101_1590;
const ROOT_FACTOR: u32 = 0x010e_c930;
/// The function-static scale (`0126ef88`) and the byte whose bit 0 says it is set (`0126ef8c`).
const STATIC_SCALE: u32 = 0x0126_ef88;
const STATIC_FLAG: u32 = 0x0126_ef8c;

/// One component of `00e520b0`: `7 - (low byte of trunc(value * scale))`.
fn nibble(product: f64) -> u8 {
    7u8.wrapping_sub(fistp_truncate_i32(product) as u8)
}

// Translated from 00e520b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, `(values, out)`: packs the four `float`s at `values` into two bytes. Each is multiplied
/// by the `double` at `010a5798` and converted with the control word forced to truncate
/// (`FISTP`; a NaN or a value out of range gives `0x80000000`); the nibble is `7` minus the low
/// byte of the integer. `out[0]` is `(n1 << 4) | n0` and `out[1]` is `(n3 << 4) | n2` (byte
/// arithmetic). The first three values are loaded through the x87 stack, the last is the memory
/// operand of the multiplication.
pub fn fn_00e520b0(e: &mut Engine, values: u32, out: u32) {
    let scale = e.mem.f64(NIBBLE_SCALE);
    let mut nibbles = [0u8; 4];
    for (index, slot) in nibbles.iter_mut().enumerate() {
        let bits = e.mem.u32(values.wrapping_add(4 * index as u32));
        let product = if index < 3 {
            x87_mul(x87_load32(bits), scale)
        } else {
            x87_mul(scale, x87_m32(bits))
        };
        *slot = nibble(product);
    }
    e.mem.set_u8(out, nibbles[1].wrapping_shl(4) | nibbles[0]);
    e.mem
        .set_u8(out.wrapping_add(1), nibbles[3].wrapping_shl(4) | nibbles[2]);
}

// Translated from 00e52190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, `(quaternion, out)`: packs a quaternion into three bytes ("smallest three"). The first
/// time, the function-static scale at `0126ef88` is made (flag bit 0 of `0126ef8c` set first):
/// `float(sqrt(double at 01011590)) * double at 010ec930`, stored as a float. The lane with the
/// largest absolute value is found (`index`; a later lane replaces the current best only when it
/// is strictly larger); the other three lanes, in order, are multiplied by the scale and
/// truncated (`FISTP` with truncation, the low byte plus `0x3f`). `out[0]` is `(index << 7) |
/// first`, `out[1]` is `((index & 0xfe) << 6) | second` and `out[2]` is `third`, with bit 7 set
/// when the largest lane is negative (`0.0 > lane`).
pub fn fn_00e52190(e: &mut Engine, quaternion: u32, out: u32) {
    if e.mem.u32(STATIC_FLAG) & 1 == 0 {
        let argument = e.mem.f64(ROOT_ARGUMENT);
        let flag = e.mem.u32(STATIC_FLAG);
        e.mem.set_u32(STATIC_FLAG, flag | 1);
        let root = e.call(X87_SQRT, &args![argument]).f64();
        let stored = x87_load32(x87_store32(root));
        let scale = x87_mul(stored, e.mem.f64(ROOT_FACTOR));
        e.mem.set_u32(STATIC_SCALE, x87_store32(scale));
    }
    let absolute = andps(load_vec(e, quaternion), load_vec(e, ABSOLUTE_MASK));
    let lane = |index: usize| x87_load32(absolute[index].to_bits());
    // FCOMPP and `TEST AH, 5` / `JP`: the later lane replaces the best only when it is larger.
    let larger = |best: f64, candidate: f64| (fcom_status(best, candidate) & 0x05) == 0x01;
    let mut best = 0usize;
    if larger(lane(0), lane(1)) {
        best = 1;
    }
    if larger(lane(best), lane(2)) {
        best = 2;
    }
    if larger(lane(best), lane(3)) {
        best = 3;
    }
    let scale = x87_load32(e.mem.u32(STATIC_SCALE));
    let component = |e: &Engine, index: usize| e.mem.u32(quaternion.wrapping_add(4 * index as u32));
    let mut other = usize::from(best == 0);
    let first = x87_mul(x87_load32(component(e, other)), scale);
    let first_byte = (fistp_truncate_i32(first) as u8).wrapping_add(0x3f);
    other += 1;
    other += usize::from(other == best);
    let second = x87_mul(x87_load32(component(e, other)), scale);
    let second_byte = (fistp_truncate_i32(second) as u8).wrapping_add(0x3f);
    other += 1;
    other += usize::from(other == best);
    let third = x87_mul(scale, x87_m32(component(e, other)));
    let third_byte = (fistp_truncate_i32(third) as u8).wrapping_add(0x3f);
    let index_byte = best as u8;
    e.mem.set_u8(out, index_byte.wrapping_shl(7) | first_byte);
    e.mem.set_u8(
        out.wrapping_add(1),
        (index_byte & 0xfe).wrapping_shl(6) | second_byte,
    );
    e.mem.set_u8(out.wrapping_add(2), third_byte);
    // FLDZ and FCOMP with the largest lane: set bit 7 only for a strictly negative lane.
    let negative = (fcom_status(0.0, x87_m32(component(e, best))) & 0x41) == 0;
    let sign = if negative { 0x80 } else { 0 };
    e.mem.set_u8(out.wrapping_add(2), sign | third_byte);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e5_20b0, fn_00e520b0(u32, u32)),
        entry!(0x00e5_2190, fn_00e52190(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::run_00d6bf60::store_vec;
    use std::cell::Cell;
    use std::rc::Rc;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    fn vector(e: &mut Engine, v: [f32; 4]) -> u32 {
        let at = e.mem.alloc(16);
        store_vec(e, at, v);
        at
    }

    fn pack_nibbles(e: &mut Engine, v: [f32; 4]) -> (u8, u8) {
        let values = vector(e, v);
        let out = e.mem.alloc(8);
        e.mem.set_u8(out + 2, 0x77);
        e.call(0x00e5_20b0, &args![values, out]);
        assert_eq!(e.mem.u8(out + 2), 0x77);
        (e.mem.u8(out), e.mem.u8(out + 1))
    }

    #[test]
    fn components_are_scaled_truncated_and_subtracted_from_seven() {
        let mut e = engine();
        e.mem.set_f64(NIBBLE_SCALE, -7.0);
        // Products 7, 3.5, -0, -7 truncate to 7, 3, 0, -7 (low bytes 7, 3, 0, 0xf9).
        let (low, high) = pack_nibbles(&mut e, [-1.0, -0.5, 0.0, 1.0]);
        assert_eq!((low, high), (0x40, 0xe7));
    }

    #[test]
    fn nan_gives_the_indefinite_integer_whose_low_byte_is_zero() {
        let mut e = engine();
        e.mem.set_f64(NIBBLE_SCALE, -7.0);
        assert_eq!(pack_nibbles(&mut e, [f32::NAN; 4]), (0x77, 0x77));
    }

    fn quaternion_engine() -> (Engine, Rc<Cell<u32>>) {
        let mut e = engine();
        for word in 0..4 {
            e.mem.set_u32(ABSOLUTE_MASK + 4 * word, 0x7fff_ffff);
        }
        e.mem.set_f64(ROOT_ARGUMENT, 4.0);
        e.mem.set_f64(ROOT_FACTOR, 31.5);
        let calls = Rc::new(Cell::new(0));
        let counter = Rc::clone(&calls);
        e.register_double(X87_SQRT, move |_, a| {
            counter.set(counter.get() + 1);
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            Ret {
                st0: value.sqrt(),
                ..Ret::default()
            }
        });
        (e, calls)
    }

    fn pack_quaternion(e: &mut Engine, q: [f32; 4]) -> [u8; 3] {
        let at = vector(e, q);
        let out = e.mem.alloc(8);
        e.call(0x00e5_2190, &args![at, out]);
        [e.mem.u8(out), e.mem.u8(out + 1), e.mem.u8(out + 2)]
    }

    #[test]
    fn largest_lane_is_dropped_and_its_sign_stored() {
        let (mut e, calls) = quaternion_engine();
        // Lane 1 is the largest (negative): the others are 0.1, 0.3 and 0.2 times 63.
        assert_eq!(
            pack_quaternion(&mut e, [0.1, -0.9, 0.3, 0.2]),
            [0xc5, 0x51, 0xcb]
        );
        assert_eq!(e.mem.f32(STATIC_SCALE), 63.0);
        assert_eq!(e.mem.u32(STATIC_FLAG) & 1, 1);
        // The static is made once.
        assert_eq!(
            pack_quaternion(&mut e, [0.9, 0.1, -0.3, 0.2]),
            [0x45, 0x2d, 0x4b]
        );
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn ties_and_the_last_lane_are_handled() {
        let (mut e, _) = quaternion_engine();
        // Equal absolute values: the first lane stays the largest.
        assert_eq!(pack_quaternion(&mut e, [0.5, -0.5, 0.0, 0.0])[0] >> 7, 0);
        // Lane 3 largest: index 3 sets bit 7 of byte 0 and (as `(3 & 0xfe) << 6`) bit 7 of byte 1.
        let packed = pack_quaternion(&mut e, [0.1, 0.2, 0.3, 0.9]);
        assert_eq!(packed[0] & 0x80, 0x80);
        assert_eq!(packed[1] & 0x80, 0x80);
        assert_eq!(packed[2] & 0x80, 0);
    }
}
